# insta-fun

SVG snapshot testing for FunDSP audio nodes.

Generate visual snapshots of audio processing nodes to catch regressions and verify signal behavior.

> **Note:** Snapshot assertion uses `insta::assert_binary_snapshot` which is currently experimental.

## Usage

```rust
use insta_fun::*;
use fundsp::prelude::*;

// Simple snapshot
let node = sine_hz::<f32>(440.0);
let svg = snapshot_audio_node(node);

// With input signal
let filter = lowpass_hz(1000.0, 1.0);
let svg = snapshot_audio_node_with_input(filter, InputSource::impulse());

// Custom configuration
let config = SnapshotConfig::with_samples(100);
let svg = snapshot_audio_node_with_options( sine_hz::<f32>(440.0), config);

// With a macro
let node = sine_hz::<f32>(440.0);
assert_audio_node_snapshot!("docs", node);
```

## Features

- Visualizes audio node inputs and outputs as SVG waveforms
- Supports multi-channel audio with color-coded traces
- Configurable sample count, SVG dimensions, and processing modes
- Built-in input generators (impulse, sine, custom)
- Batch or tick-by-tick processing
- Assertion macro

## Processing Modes

- **Tick**: Process one sample at a time (default)
- **Batch**: Process up to 64 samples at once for efficiency

## The Unlicense

See [LICENSE](./LICENSE) file for details.
