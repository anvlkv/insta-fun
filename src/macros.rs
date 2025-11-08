/// Macro for audio unit snapshot testing
///
/// This macro processes an audio unit with the given configuration,
/// generates an SVG visualization, and asserts it against a stored snapshot.
///
/// ## Examples
///
/// ```
/// use fundsp::prelude::*;
/// use insta_fun::prelude::*;
///
/// // With a custom name
/// let unit = saw_hz(220.0);
/// assert_audio_unit_snapshot!("doc_sawtooth", unit);
///
/// ```
///
/// ```
/// use fundsp::prelude::*;
/// use insta_fun::prelude::*;
///
/// // With input source
/// let unit = lowpass_hz(1000.0, 1.0);
/// assert_audio_unit_snapshot!("doc_lowpass", unit, InputSource::impulse());
/// ```
///
/// ```
/// use fundsp::prelude::*;
/// use insta_fun::prelude::*;
///
/// // With input source and custom config
/// let config = SnapshotConfigBuilder::default().num_samples(512).build().unwrap();
/// let unit = highpass_hz(2000.0, 0.7);
/// assert_audio_unit_snapshot!("doc_highpass", unit, InputSource::sine(100.0, 44100.0), config);
/// ```
///
/// ```
/// use fundsp::prelude::*;
/// use insta_fun::prelude::*;
///
/// // With unit and config
/// let unit = lowpass_hz(1000.0, 1.0);
/// let config = SnapshotConfigBuilder::default().num_samples(512).build().unwrap();
/// assert_audio_unit_snapshot!(unit, config);
///
/// ```
#[macro_export]
macro_rules! assert_audio_unit_snapshot {
    // With just the unit
    ($unit:expr) => {{
        let svg = $crate::snapshot::snapshot_audio_unit($unit);

        ::insta::assert_binary_snapshot!(".svg", svg.as_bytes().to_vec());
    }};

    // With name and unit
    ($name:literal, $unit:expr) => {{
        let config = $crate::config::SnapshotConfigBuilder::default()
            .chart_title($name)
            .build()
            .unwrap();
        let svg = $crate::snapshot::snapshot_audio_unit_with_options($unit, config);

        ::insta::assert_binary_snapshot!(&format!("{}.svg", $name), svg.as_bytes().to_vec());
    }};

    // With input source
    ($name:literal, $unit:expr, $input:expr) => {{
        let config = $crate::config::SnapshotConfigBuilder::default()
            .chart_title($name)
            .build()
            .unwrap();
        let svg =
            $crate::snapshot::snapshot_audio_unit_with_input_and_options($unit, $input, config);

        ::insta::assert_binary_snapshot!(&format!("{}.svg", $name), svg.as_bytes().to_vec());
    }};

    // With input source and config
    ($name:literal, $unit:expr, $input:expr, $config:expr) => {{
        let config = if $config.chart_title.is_some() {
            $config
        } else {
            $crate::config::SnapshotConfig {
                chart_title: Some($name.to_string()),
                ..$config
            }
        };
        let svg =
            $crate::snapshot::snapshot_audio_unit_with_input_and_options($unit, $input, config);

        ::insta::assert_binary_snapshot!(&format!("{}.svg", $name), svg.as_bytes().to_vec());
    }};

    // With unit and config
    ($unit:expr, $config:expr) => {{
        let svg = $crate::snapshot::snapshot_audio_unit_with_options($unit, $config);

        ::insta::assert_binary_snapshot!(".svg", svg.as_bytes().to_vec());
    }};
}
