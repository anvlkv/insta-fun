use fundsp::net::{Net, Source};
use fundsp::prelude::*;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};

use std::collections::HashMap;

/// Kind of node in the rendered graph.
#[derive(Clone, Debug)]
enum DspNodeKind {
    Unit,
    GlobalIn,
    GlobalOut,
}

/// Node weight with kind and human-readable label.
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct DspNode {
    kind: DspNodeKind,
    label: String,
    // Optional unit id for unit nodes
    unit_id: Option<NodeId>,
    // Optional port index for global input/output nodes
    port: Option<usize>,
}

/// Edge weight capturing port indices for labeling.
#[derive(Clone, Debug)]
struct DspEdge {
    /// Source port index (unit output channel) if applicable.
    src_port: Option<usize>,
    /// Destination port index (unit input channel) if applicable.
    dst_port: Option<usize>,
}

/// Build a petgraph DiGraph from a fundsp Net, including global inputs/outputs and edges.
/// Returns:
/// - The constructed graph.
/// - Mapping from fundsp NodeId to petgraph NodeIndex.
/// - The list of global input node indices.
/// - The list of global output node indices.
#[allow(clippy::type_complexity)]
fn build_petgraph(
    net: &mut Net,
) -> (
    DiGraph<DspNode, DspEdge>,
    HashMap<NodeId, NodeIndex>,
    Vec<NodeIndex>,
    Vec<NodeIndex>,
) {
    let mut g: DiGraph<DspNode, DspEdge> = DiGraph::new();

    // Create global input nodes
    let mut global_in_nodes = Vec::new();
    for in_port in 0..net.inputs() {
        let idx = g.add_node(DspNode {
            kind: DspNodeKind::GlobalIn,
            label: format!("IN[{}]", in_port),
            unit_id: None,
            port: Some(in_port),
        });
        global_in_nodes.push(idx);
    }

    // Create global output nodes
    let mut global_out_nodes = Vec::new();
    for out_port in 0..net.outputs() {
        let idx = g.add_node(DspNode {
            kind: DspNodeKind::GlobalOut,
            label: format!("OUT[{}]", out_port),
            unit_id: None,
            port: Some(out_port),
        });
        global_out_nodes.push(idx);
    }

    // Collect ids to avoid mixed borrows, then sort deterministically by numeric NodeId value
    let mut ids: Vec<NodeId> = net.ids().copied().collect();
    // NodeId Debug format is `NodeId(n)`. Parse n to a number for stable sorting across runs.
    ids.sort_by_key(|id| {
        // Fallback to Debug string if parsing fails (shouldn't happen), to keep deterministic order.
        let s = format!("{:?}", id);
        if let Some(num) = s
            .strip_prefix("NodeId(")
            .and_then(|r| r.strip_suffix(')'))
            .and_then(|r| r.parse::<u64>().ok())
        {
            num
        } else {
            // Hash the string into a u64 for a stable fallback order
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            s.hash(&mut hasher);
            hasher.finish()
        }
    });

    // Precompute unit display labels to avoid borrow conflicts later
    let mut labels: HashMap<NodeId, String> = HashMap::new();
    for &id in &ids {
        let label = net.node_mut(id).display();
        labels.insert(id, label);
    }

    // Create unit nodes
    let mut id_to_idx: HashMap<NodeId, NodeIndex> = HashMap::new();
    for &id in &ids {
        let label = labels
            .get(&id)
            .cloned()
            .unwrap_or_else(|| format!("Unit {:?}", id));
        let idx = g.add_node(DspNode {
            kind: DspNodeKind::Unit,
            label,
            unit_id: Some(id),
            port: None,
        });
        id_to_idx.insert(id, idx);
    }

    // Prepare to collect and sort edges before insertion
    let mut edges: Vec<(NodeIndex, NodeIndex, DspEdge)> = Vec::new();

    // Edges for unit inputs: connect sources to each unit input channel
    for &id in &ids {
        let inputs = net.inputs_in(id);
        for i in 0..inputs {
            match net.source(id, i) {
                Source::Local(src_id, src_port) => {
                    if let (Some(&src_idx), Some(&dst_idx)) =
                        (id_to_idx.get(&src_id), id_to_idx.get(&id))
                    {
                        edges.push((
                            src_idx,
                            dst_idx,
                            DspEdge {
                                src_port: Some(src_port),
                                dst_port: Some(i),
                            },
                        ));
                    }
                }
                Source::Global(global_port) => {
                    if let (Some(&src_idx), Some(&dst_idx)) =
                        (global_in_nodes.get(global_port), id_to_idx.get(&id))
                    {
                        edges.push((
                            src_idx,
                            dst_idx,
                            DspEdge {
                                src_port: None,
                                dst_port: Some(i),
                            },
                        ));
                    }
                }
                Source::Zero => {
                    // No edge for zero source
                }
            }
        }
    }

    // Edges for global outputs: connect sources to each global output channel
    for out_port in 0..net.outputs() {
        match net.output_source(out_port) {
            Source::Local(src_id, src_port) => {
                if let (Some(&src_idx), Some(&dst_idx)) =
                    (id_to_idx.get(&src_id), global_out_nodes.get(out_port))
                {
                    edges.push((
                        src_idx,
                        dst_idx,
                        DspEdge {
                            src_port: Some(src_port),
                            dst_port: None,
                        },
                    ));
                }
            }
            Source::Global(global_port) => {
                if let (Some(&src_idx), Some(&dst_idx)) = (
                    global_in_nodes.get(global_port),
                    global_out_nodes.get(out_port),
                ) {
                    edges.push((
                        src_idx,
                        dst_idx,
                        DspEdge {
                            src_port: None,
                            dst_port: None,
                        },
                    ));
                }
            }
            Source::Zero => {
                // No edge for zero source
            }
        }
    }

    // Sort edges by (source index, dest index, src_port, dst_port) for deterministic DOT.
    edges.sort_by_key(|(s, d, w)| {
        (
            s.index(),
            d.index(),
            w.src_port.unwrap_or(usize::MAX),
            w.dst_port.unwrap_or(usize::MAX),
        )
    });
    // Insert edges in sorted order.
    for (s, d, w) in edges {
        g.add_edge(s, d, w);
    }

    (g, id_to_idx, global_in_nodes, global_out_nodes)
}

/// Render the given graph to Graphviz DOT format using custom node and edge labels.
/// Returns the DOT as a String.
fn dot_string(graph: &DiGraph<DspNode, DspEdge>) -> String {
    let dot = Dot::with_attr_getters(
        graph,
        &[Config::NodeNoLabel, Config::EdgeNoLabel],
        // Edge attributes: include port indices where available.
        &|_g, e| {
            let w = e.weight();
            match (w.src_port, w.dst_port) {
                (Some(sp), Some(dp)) => format!(r#"label = "out {} -> in {}""#, sp, dp),
                (Some(sp), None) => format!(r#"label = "out {}""#, sp),
                (None, Some(dp)) => format!(r#"label = "in {}""#, dp),
                (None, None) => String::new(),
            }
        },
        // Node attributes: label and shape based on kind.
        &|_g, (_idx, weight)| {
            let shape = match weight.kind {
                DspNodeKind::GlobalIn => "shape = invhouse",
                DspNodeKind::GlobalOut => "shape = house",
                DspNodeKind::Unit => "shape = plaintext, margin = 0",
            };
            // Build Graphviz HTML-like TABLE label to preserve whitespace.
            // - Escape &, <, > for HTML safety.
            // - Replace spaces with &nbsp; to keep alignment.
            // - Replace newlines with <BR/> to preserve line breaks.
            let html_safe = weight
                .label
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            let html_nbsp = html_safe.replace(' ', "&nbsp;");
            let html_lines = html_nbsp.replace('\n', "<BR/>");
            // Read unit_id/port to avoid dead code warnings and enrich labels
            let extra = match weight.kind {
                DspNodeKind::Unit => weight
                    .unit_id
                    .map(|id| format!(" [id: {:?}]", id))
                    .unwrap_or_default(),
                DspNodeKind::GlobalIn | DspNodeKind::GlobalOut => weight
                    .port
                    .map(|p| format!(" [ch: {}]", p))
                    .unwrap_or_default(),
            };
            let extra_row = if extra.is_empty() {
                String::new()
            } else {
                let extra_html = extra
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace(' ', "&nbsp;");
                format!(
                    r#"<TR><TD><FONT FACE="monospace">{}</FONT></TD></TR>"#,
                    extra_html
                )
            };
            // Use Graphviz HTML-like labels with a TABLE to ensure whitespace is preserved.
            format!(
                r#"label = <<TABLE BORDER="1" CELLBORDER="1" CELLSPACING="0" CELLPADDING="2"><TR><TD><FONT FACE="monospace" POINT-SIZE="10">{}</FONT></TD></TR>{}</TABLE>>, {}"#,
                html_lines, extra_row, shape
            )
        },
    );
    format!("{:?}", dot)
}

/// Build the graph from the given Net and return the DOT output as bytes (Vec<u8>).
pub fn snapshot_dsp_net_wiring(mut net: Net) -> Vec<u8> {
    let (graph, _map, _ins, _outs) = build_petgraph(&mut net);
    let dot = dot_string(&graph);
    dot.into_bytes()
}
