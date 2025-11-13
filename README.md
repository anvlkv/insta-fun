# insta-fun

SVG snapshot testing for FunDSP audio units.

Generate visual snapshots of audio processing units to catch regressions and verify signal behavior.

![Example](https://raw.githubusercontent.com/anvlkv/insta-fun/refs/heads/main/src/snapshots/insta_fun__tests__macro_with_input.snap.svg)

> **Note:** Snapshot assertion uses `insta::assert_binary_snapshot` which is currently experimental.

## Usage

```rust
use insta_fun::prelude::*;
use fundsp::prelude::*;

#[test]
fn example_test() {
    // Simple snapshot
    let unit = sine_hz::<f32>(440.0);
    let svg = snapshot_audio_unit(unit);

    // With input signal
    let filter = lowpass_hz(1000.0, 1.0);
    let svg = snapshot_audio_unit_with_input(filter, InputSource::impulse());

    // Custom configuration
    let config = SnapshotConfigBuilder::default().num_samples(100).build().unwrap();
    let svg = snapshot_audio_unit_with_options(sine_hz::<f32>(440.0), config);

    // With a macro
    let unit = sine_hz::<f32>(440.0);
    assert_audio_unit_snapshot!(unit);
}
```

## Features

- Visualizes audio unit inputs and outputs as SVG waveforms
- Supports multi-channel audio with color-coded traces
- Configurable sample count, SVG dimensions, and processing modes
- Built-in input generators (impulse, sine, custom)
- Batch or tick-by-tick processing
- Warmup
- Chart layouts with legend
- Label formatting
- Assertion macro

## Processing Modes

- **Tick**: Process one sample at a time (default) for testing `fundsp::AudioUnit::tick`
- **Batch**: Process up to 64 samples at once for testing `fundsp::AudioUnit::process`

## Examples

See [snapshots](https://github.com/anvlkv/insta-fun/tree/main/src/snapshots) for full list of example charts.

See [tests](https://github.com/anvlkv/insta-fun/tree/main/src/tests.rs) for all usage examples.

## The Unlicense

See [LICENSE](https://raw.githubusercontent.com/anvlkv/insta-fun/refs/heads/main/LICENSE) file for details.
