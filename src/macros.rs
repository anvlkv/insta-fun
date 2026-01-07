/// Macro for audio unit snapshot testing
///
/// By default (arms without an explicit `SnapshotConfig`) this macro produces
/// TWO snapshots for the given audio unit:
/// 1. An SVG chart (default `SvgChartConfig`)
/// 2. A 16-bit WAV file (`WavOutput::Wav16`)
///
/// Arms that accept a custom `SnapshotConfig` produce exactly ONE snapshot
/// using the `output_mode` specified in that config.
///
/// The unit is internally cloned so it can be processed twice (SVG + WAV)
/// without triggering a move error. If your audio unit type does not implement
/// `Clone`, use the config form and select a single output mode.
///
/// ## Examples
///
/// ```rust,no_run
/// use fundsp::prelude::*;
/// use insta_fun::prelude::*;
///
/// // With a custom name (generates name.svg & name.wav)
/// let unit = saw_hz(220.0);
/// assert_audio_unit_snapshot!("doc_sawtooth", unit);
/// ```
///
/// ```rust,no_run
/// use fundsp::prelude::*;
/// use insta_fun::prelude::*;
///
/// // With input source (generates doc_lowpass.svg & doc_lowpass.wav)
/// assert_audio_unit_snapshot!("doc_lowpass", lowpass_hz(1000.0, 1.0), InputSource::impulse());
/// ```
///
/// ```rust,no_run
/// use fundsp::prelude::*;
/// use insta_fun::prelude::*;
///
/// // With input source and custom config (generates only one file per config.output_mode)
/// let chart = SvgChartConfigBuilder::default().chart_title("Highpass").build().unwrap();
/// let config = SnapshotConfigBuilder::default()
///     .num_samples(512)
///     .output_mode(chart)
///     .build()
///     .unwrap();
/// assert_audio_unit_snapshot!(
///     "doc_highpass",
///     highpass_hz(2000.0, 0.7),
///     InputSource::sine(100.0, 44100.0),
///     config
/// );
/// ```
///
/// ```rust,no_run
/// use fundsp::prelude::*;
/// use insta_fun::prelude::*;
///
/// // With unit and config (single snapshot)
/// let config = SnapshotConfigBuilder::default()
///     .output_mode(WavOutput::Wav32)
///     .num_samples(512)
///     .build()
///     .unwrap();
/// assert_audio_unit_snapshot!(
///     "doc_wav32",
///     sine_hz::<f32>(440.0),
///     InputSource::None,
///     config
/// );
/// ```
///
/// Invariants / Notes:
/// - Unit expression is evaluated once per macro invocation; cloned only for dual-snapshot arms.
/// - Name expression (in widened arms) is evaluated exactly once and converted with Into<String>.
/// - Input expression in the (name, unit, input) arm is evaluated twice (once per SVG, once per WAV).
/// - Two-arg (name, unit) arm keeps `$name:literal` to avoid ambiguity with the (unit, config) arm.
///   To use a dynamic (non-literal) name, supply an input or a config (3 or 4 argument forms).
#[macro_export]
macro_rules! assert_audio_unit_snapshot {
    // With just the unit (SVG + WAV16)
    ($unit:expr) => {{
        // Capture unit once; clone for second snapshot to avoid move error.
        let mut __unit = $unit;
        let __unit_clone = __unit.clone();

        // SVG
        let config = $crate::config::SnapshotConfigBuilder::default()
            .build()
            .unwrap();
        let name = config.file_name(None);
        let data_svg = $crate::snapshot::snapshot_audio_unit_with_options(__unit, config);

        ::insta::with_settings!({ omit_expression => true}, {
            ::insta::assert_binary_snapshot!(&name, data_svg.as_slice().to_vec());
        });

        // WAV16
        let config = $crate::config::SnapshotConfigBuilder::default()
            .output_mode($crate::config::WavOutput::Wav16)
            .build()
            .unwrap();
        let name = config.file_name(None);
        let data_wav = $crate::snapshot::snapshot_audio_unit_with_options(__unit_clone, config);

        ::insta::with_settings!({ omit_expression => true, snapshot_suffix => "audio" }, {
            ::insta::assert_binary_snapshot!(&name, data_wav.as_slice().to_vec());
        });
    }};

    // With name and unit (name.svg + name.wav) - kept literal for disambiguation.
    ($name:literal, $unit:expr) => {{
        let mut __unit = $unit;
        let __unit_clone = __unit.clone();

        // SVG
        let config = $crate::config::SnapshotConfigBuilder::default()
            .chart_title($name)
            .build()
            .unwrap();
        let name = config.file_name(Some($name));
        let data_svg = $crate::snapshot::snapshot_audio_unit_with_options(__unit, config);

        ::insta::with_settings!({ omit_expression => true}, {
            ::insta::assert_binary_snapshot!(&name, data_svg.as_slice().to_vec());
        });

        // WAV16
        let config = $crate::config::SnapshotConfigBuilder::default()
            .output_mode($crate::config::WavOutput::Wav16)
            .build()
            .unwrap();
        let name = config.file_name(Some($name));
        let data_wav = $crate::snapshot::snapshot_audio_unit_with_options(__unit_clone, config);

        ::insta::with_settings!({ omit_expression => true, snapshot_suffix => "audio" }, {
            ::insta::assert_binary_snapshot!(&name, data_wav.as_slice().to_vec());
        });
    }};

    // With input source (name.svg + name.wav) - widened name
    ($name:expr, $unit:expr, $input:expr) => {{
        let __name: String = ::std::convert::Into::into($name);
        let mut __unit = $unit;
        let __unit_clone = __unit.clone();
        // Input expression is evaluated twice; prefer passing a cheap expression (e.g. InputSource::impulse()).
        // If you have a bound variable you need two independent values.

        // SVG
        let config = $crate::config::SnapshotConfigBuilder::default()
            .chart_title(__name.as_str())
            .build()
            .unwrap();
        let name = config.file_name(Some(__name.as_str()));
        let data_svg =
            $crate::snapshot::snapshot_audio_unit_with_input_and_options(__unit, $input, config);

        ::insta::with_settings!({ omit_expression => true}, {
            ::insta::assert_binary_snapshot!(&name, data_svg.as_slice().to_vec());
        });

        // WAV16
        let config = $crate::config::SnapshotConfigBuilder::default()
            .output_mode($crate::config::WavOutput::Wav16)
            .build()
            .unwrap();
        let name = config.file_name(Some(__name.as_str()));
        let data_wav =
            $crate::snapshot::snapshot_audio_unit_with_input_and_options(__unit_clone, $input, config);

        ::insta::with_settings!({ omit_expression => true, snapshot_suffix => "audio" }, {
            ::insta::assert_binary_snapshot!(&name, data_wav.as_slice().to_vec());
        });
    }};

    // With input source and config (single snapshot; uses config.output_mode) - widened name
    ($name:expr, $unit:expr, $input:expr, $config:expr) => {{
        let __name: String = ::std::convert::Into::into($name);
        let mut config = $config;
        config.maybe_title(__name.as_str());

        let is_audio = matches!(
            config.output_mode,
            $crate::config::SnapshotOutputMode::Wav(_)
        );

        let name = config.file_name(Some(__name.as_str()));
        let data = $crate::snapshot::snapshot_audio_unit_with_input_and_options($unit, $input, config);

        if is_audio {
            ::insta::with_settings!({ omit_expression => true, snapshot_suffix => "audio" }, {
                ::insta::assert_binary_snapshot!(&name, data.as_slice().to_vec());
            });
        }
        else {
            ::insta::with_settings!({ omit_expression => true}, {
                ::insta::assert_binary_snapshot!(&name, data.as_slice().to_vec());
            });
        }
    }};

    // With unit and config (single snapshot; uses config.output_mode)
    // NOTE: The (name, unit) two-arg arm keeps name as a literal to avoid ambiguity with this (unit, config) arm.
    // If you need a dynamic (non-literal) name, use a 3-arg or 4-arg form (name, unit, input[, config]) which are widened to $name:expr.
        ($unit:expr, $config:expr) => {{

        let is_audio = matches!(
            $config.output_mode,
            $crate::config::SnapshotOutputMode::Wav(_)
        );
        // Capture name before moving config into processing.
        let name = $config.file_name(None);
        let data = $crate::snapshot::snapshot_audio_unit_with_options($unit, $config);

        if is_audio {
            ::insta::with_settings!({ omit_expression => true, snapshot_suffix => "audio" }, {
                ::insta::assert_binary_snapshot!(&name, data.as_slice().to_vec());
            });
        }
        else {
            ::insta::with_settings!({ omit_expression => true }, {
                ::insta::assert_binary_snapshot!(&name, data.as_slice().to_vec());
            });
        }
    }};
}

/// Macro to snapshot a fundsp `Net` as a Graphviz DOT binary using `snapshot_dsp_net`.
///
/// ### Usage:
///
/// `assert_dsp_net_snapshot!(name_expr, net_expr);`
///
/// - name_expr: used to name the snapshot file (e.g., "my_net")
/// - net_expr: expression that evaluates to a `fundsp::net::Net`
///
/// Produces a single binary snapshot (.dot content) under the given name.
#[macro_export]
macro_rules! assert_dsp_net_snapshot {
    ($name:expr, $net:expr) => {{
        let __name: String = ::std::convert::Into::into($name);
        // Build DOT bytes from the Net
        let __snap_name = format!("{}.dot", __name);
        let __bytes: ::std::vec::Vec<u8> = $crate::graph::snapshot_dsp_net_wiring($net);
        // Assert as binary snapshot
        ::insta::with_settings!({ omit_expression => true }, {
            ::insta::assert_binary_snapshot!(&__snap_name, __bytes);
        });
    }};
}
