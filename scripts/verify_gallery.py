#!/usr/bin/env python3
"""
verify_gallery.py

Purpose:
  Verify consistency between snapshot source files in examples/snapshots/*.snap
  and Markdown gallery references in .gh-pages/*.md.

Definitions:
  Snapshot file:
    - Chart snapshot: <stem>.snap               (no "@audio" in stem)
    - Audio snapshot: <stem>@audio.snap         (contains "@audio" in stem)
  Referenced resources in markdown:
    - Chart image:    <stem>.snap.svg
    - Audio file:     <stem>@audio.snap.wav

Rules:
  1. Every non-audio snapshot (<stem>.snap) MUST have its .snap.svg referenced
     in at least one .gh-pages/*.md file.
  2. Every audio snapshot (<stem>@audio.snap) MUST have its .snap.wav referenced.
     (Its .snap.svg may exist/be referenced but is NOT required.)
  3. Every .svg/.wav reference in markdown MUST correspond to an existing
     underlying .snap file.
  4. Ignore any ".new.svg" or ".new.wav" references (transient update artifacts).
  5. Report:
     - missing_chart_refs: non-audio snapshots lacking .svg reference
     - missing_audio_refs: audio snapshots lacking .wav reference
     - orphan_references: markdown references whose underlying .snap file does not exist
     - unreferenced_snapshots: snapshots not referenced at all (optional informational)
  6. Exit code:
     - 0 if no missing_chart_refs, missing_audio_refs, orphan_references
     - 1 otherwise

CLI:
  --json          Output JSON report instead of human-readable text.
  --strict        Treat unreferenced_snapshots also as an error (adds to exit code).
  --root PATH     Override project root (default: parent of this script).
  --require-audio-svg  Enforce that audio snapshots also have .snap.svg referenced.
  --verbose       Show additional diagnostic detail.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Dict, Set, List


SNAP_DIR_REL = Path("examples/snapshots")
MD_DIR_REL = Path(".gh-pages")
REFERENCE_REGEX = re.compile(
    r"examples/snapshots/([A-Za-z0-9_.@-]+\.snap\.(?:svg|wav))(?!\.new)",
    re.IGNORECASE,
)


class SnapshotInfo:
    def __init__(self, snap_path: Path):
        self.snap_path = snap_path
        self.name = snap_path.name  # e.g. foo.snap or foo@audio.snap
        self.is_audio = "@audio" in self.name
        self.stem = self.name  # keep full file name for mapping
        self.chart_referenced = False
        self.audio_referenced = False

    def expected_chart(self) -> str:
        return f"{self.stem}.svg"

    def expected_audio(self) -> str:
        return f"{self.stem}.wav"

    def to_dict(self) -> Dict[str, object]:
        return {
            "snap": self.name,
            "is_audio": self.is_audio,
            "chart_referenced": self.chart_referenced,
            "audio_referenced": self.audio_referenced,
        }


def gather_snapshots(snapshots_dir: Path) -> Dict[str, SnapshotInfo]:
    mapping: Dict[str, SnapshotInfo] = {}
    if not snapshots_dir.is_dir():
        raise SystemExit(f"Snapshot directory not found: {snapshots_dir}")
    for snap in sorted(snapshots_dir.glob("*.snap")):
        info = SnapshotInfo(snap)
        mapping[info.name] = info
    return mapping


def parse_markdown(md_dir: Path) -> List[Path]:
    if not md_dir.is_dir():
        raise SystemExit(f"Markdown directory not found: {md_dir}")
    return sorted(md_dir.glob("*.md"))


def analyze(
    snapshots: Dict[str, SnapshotInfo],
    md_files: List[Path],
    require_audio_svg: bool,
) -> Dict[str, object]:
    # Track referenced resources.
    referenced_resources: Set[str] = set()  # e.g. foo.snap.svg, bar@audio.snap.wav
    orphan_references: Set[str] = set()

    for md in md_files:
        text = md.read_text(encoding="utf-8", errors="replace")
        for match in REFERENCE_REGEX.finditer(text):
            resource = match.group(1)  # e.g. foo.snap.svg
            if ".new." in resource:
                # Ignore transient new artifacts explicitly.
                continue
            referenced_resources.add(resource)
            underlying_snap = resource.rsplit(".", 1)[0]  # strip .svg or .wav
            if underlying_snap not in snapshots:
                orphan_references.add(resource)
                continue
            snap_info = snapshots[underlying_snap]
            if resource.endswith(".svg"):
                snap_info.chart_referenced = True
            elif resource.endswith(".wav"):
                snap_info.audio_referenced = True

    missing_chart_refs = [
        s.name for s in snapshots.values() if not s.is_audio and not s.chart_referenced
    ]
    missing_audio_refs = [
        s.name for s in snapshots.values() if s.is_audio and not s.audio_referenced
    ]

    # Optional enforcement: audio snapshots also require chart .svg.
    missing_audio_svg_refs: List[str] = []
    if require_audio_svg:
        missing_audio_svg_refs = [
            s.name for s in snapshots.values() if s.is_audio and not s.chart_referenced
        ]

    # Unreferenced snapshots (neither chart nor audio referenced)
    unreferenced_snapshots = [
        s.name
        for s in snapshots.values()
        if (not s.chart_referenced and (not s.is_audio or not s.audio_referenced))
        and (not s.is_audio or not s.audio_referenced)
    ]

    return {
        "missing_chart_refs": sorted(missing_chart_refs),
        "missing_audio_refs": sorted(missing_audio_refs),
        "missing_audio_svg_refs": sorted(missing_audio_svg_refs),
        "orphan_references": sorted(orphan_references),
        "unreferenced_snapshots": sorted(unreferenced_snapshots),
        "total_snapshots": len(snapshots),
        "snapshots": [s.to_dict() for s in snapshots.values()],
    }


def print_human(report: Dict[str, object], strict: bool, verbose: bool) -> int:
    def section(title: str, items: List[str]):
        print(f"\n{title}:")
        if items:
            for it in items:
                print(f"  - {it}")
        else:
            print("  (none)")

    print("Gallery Verification Report")
    print("===========================")
    print(f"Total snapshots: {report['total_snapshots']}")

    section("Missing chart references (non-audio)", report["missing_chart_refs"])
    section("Missing audio references (@audio)", report["missing_audio_refs"])
    if report["missing_audio_svg_refs"]:
        section(
            "Missing audio chart references (@audio, enforced)",
            report["missing_audio_svg_refs"],
        )
    section("Orphan markdown references", report["orphan_references"])

    if verbose:
        section("Unreferenced snapshots (informational)", report["unreferenced_snapshots"])

    error_conditions = (
        report["missing_chart_refs"]
        or report["missing_audio_refs"]
        or report["orphan_references"]
        or (strict and report["unreferenced_snapshots"])
        or report["missing_audio_svg_refs"]
    )
    if error_conditions:
        print("\nResult: FAIL")
        return 1
    print("\nResult: OK")
    return 0


def main(argv: List[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Verify that markdown pages reference existing snapshot artifacts."
    )
    parser.add_argument("--json", action="store_true", help="Output JSON report.")
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Treat unreferenced snapshots as errors (affects exit code).",
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parent.parent,
        help="Project root (default: parent of script directory).",
    )
    parser.add_argument(
        "--require-audio-svg",
        action="store_true",
        help="Require .snap.svg reference for audio snapshots in addition to .wav.",
    )
    parser.add_argument(
        "--verbose", action="store_true", help="Show additional informational sections."
    )
    args = parser.parse_args(argv)

    snapshots_dir = args.root / SNAP_DIR_REL
    md_dir = args.root / MD_DIR_REL

    snapshots = gather_snapshots(snapshots_dir)
    md_files = parse_markdown(md_dir)

    report = analyze(
        snapshots=snapshots,
        md_files=md_files,
        require_audio_svg=args.require_audio_svg,
    )

    if args.json:
        print(json.dumps(report, indent=2))
        # Determine exit status similar to human mode.
        fail = (
            report["missing_chart_refs"]
            or report["missing_audio_refs"]
            or report["orphan_references"]
            or (args.strict and report["unreferenced_snapshots"])
            or report["missing_audio_svg_refs"]
        )
        return 1 if fail else 0

    return print_human(report, strict=args.strict, verbose=args.verbose)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
