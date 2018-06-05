mod annot_env;
mod toposort;
mod unreify_env;

use std::collections::HashMap;
use std::hash::Hash;

pub use typeck::util::annot_env::AnnotEnv;
pub use typeck::util::toposort::toposort;
pub use typeck::util::unreify_env::UnreifyEnv;

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
