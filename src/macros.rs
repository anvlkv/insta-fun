/// Macro for audio node snapshot testing
///
/// This macro processes an audio node with the given configuration,
/// generates an SVG visualization, and asserts it against a stored snapshot.
///
/// ## Examples
///
/// ```
/// use fundsp::prelude::*;
/// use insta_fun::{assert_audio_node_snapshot, SnapshotConfig, InputSource};
///
/// // Simple usage with just a node
/// let node = sine_hz::<f32>(440.0);
/// assert_audio_node_snapshot!(node);
///
/// // With a custom name
/// let node = saw_hz(220.0);
/// assert_audio_node_snapshot!("sawtooth", node);
///
/// // With input source
/// let node = lowpass_hz(1000.0, 1.0);
/// assert_audio_node_snapshot!("lowpass", node, InputSource::impulse());
///
/// // With input source and custom config
/// let config = SnapshotConfig::with_samples(512);
/// let node = highpass_hz(2000.0, 0.7);
/// assert_audio_node_snapshot!("highpass", node, InputSource::sine(100.0, 44100.0), config);
/// ```
#[macro_export]
macro_rules! assert_audio_node_snapshot {
    // With just the node
    ($node:expr) => {{
        let svg = $crate::snapshot_audio_node($node);

        ::insta::assert_binary_snapshot!(".svg", svg.as_bytes().to_vec());
    }};

    // With name and node
    ($name:expr, $node:expr) => {{
        let svg = $crate::snapshot_audio_node($node);

        ::insta::assert_binary_snapshot!(&format!("{}.svg", $name), svg.as_bytes().to_vec());
    }};

    // With input source
    ($name:expr, $node:expr, $input:expr) => {{
        let svg = $crate::snapshot_audio_node_with_input($node, $input);

        ::insta::assert_binary_snapshot!(&format!("{}.svg", $name), svg.as_bytes().to_vec());
    }};

    // With input source and config
    ($name:expr, $node:expr, $input:expr, $config:expr) => {{
        let svg = $crate::snapshot_audio_node_with_input_and_options($node, $input, $config);

        ::insta::assert_binary_snapshot!(&format!("{}.svg", $name), svg.as_bytes().to_vec());
    }};
}
