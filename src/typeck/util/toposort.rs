use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A topological sort. Returns `Err` with the relevant label if a cycle is found.
pub fn toposort<GetEdges, Label, Node>(
    mut nodes: HashMap<Label, Node>,
    mut known: HashSet<Label>,
    get_edges: GetEdges,
) -> Result<Vec<Node>, Label>
where
    GetEdges: Fn(&Node) -> Vec<Label>,
    Label: Clone + Eq + Hash,
{
    let mut out = Vec::with_capacity(nodes.len());
    loop {
        let label = nodes.keys().next().cloned();
        if let Some(label) = label {
            visit(&mut nodes, &mut known, &mut out, label, &get_edges)?;
        } else {
            return Ok(out);
        }
    }
}

fn visit<GetEdges, Label, Node>(
    nodes: &mut HashMap<Label, Node>,
    known: &mut HashSet<Label>,
    out: &mut Vec<Node>,
    label: Label,
    get_edges: &GetEdges,
) -> Result<(), Label>
where
    GetEdges: Fn(&Node) -> Vec<Label>,
    Label: Eq + Hash,
{
    if known.contains(&label) {
        return Ok(());
    }

    let node = if let Some(node) = nodes.remove(&label) {
        node
    } else {
        return Err(label);
    };

    for edge in get_edges(&node) {
        visit(nodes, known, out, edge, get_edges)?;
    }

    out.push(node);
    known.insert(label);
    Ok(())
}
