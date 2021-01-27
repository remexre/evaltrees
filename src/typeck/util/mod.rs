mod annot_env;
mod unreify_env;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use petgraph::{algo::kosaraju_scc, Graph};
use symbol::Symbol;

use crate::ast::Decl;
pub use crate::typeck::util::annot_env::AnnotEnv;
pub use crate::typeck::util::unreify_env::UnreifyEnv;

/// Collects the values into collections by a key.
pub fn group<F, K, II, T>(vals: II, get_key: F) -> HashMap<K, Vec<T>>
where
    F: Fn(&T) -> K,
    K: Eq + Hash,
    II: IntoIterator<Item = T>,
{
    let mut out = HashMap::new();
    for val in vals {
        let key = get_key(&val);
        out.entry(key).or_insert_with(Vec::new).push(val);
    }
    out
}

/// Collects declarations into strongly connected components.
pub fn scc_decls(
    mut decls: HashMap<Symbol, Vec<Decl<()>>>,
    is_known: &HashSet<Symbol>,
) -> Vec<Vec<Decl<()>>> {
    let mut gr = Graph::new();
    let mut names = HashMap::with_capacity(decls.len());
    for &name in decls.keys() {
        names.insert(name, gr.add_node(name));
    }
    for (from, decls) in &decls {
        decls
            .iter()
            .flat_map(|decl| decl.freevars())
            .filter(|to| !is_known.contains(to))
            .for_each(|to| {
                gr.update_edge(names[from], names[&to], ());
            });
    }

    kosaraju_scc(&gr)
        .into_iter()
        .map(|scc| {
            scc.into_iter()
                .flat_map(|id| decls.remove(&gr[id]).unwrap())
                .collect()
        })
        .collect()
}
