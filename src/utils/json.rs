use serde_json::{Value, json};
use superslice::*;

pub fn find_with_keys<T: serde::Serialize, U: Ord>(
    items: &Vec<T>,
    keys: &Vec<U>,
    key: U
) -> Value {
    let lo = keys.lower_bound(&key);
    let up = keys.upper_bound(&key);
    json!((lo..up).into_iter().map(|i| {
        json!(items[i])
    }).collect::<Value>())
}

pub fn join_association<'r, A: serde::Serialize, B: Ord, C: serde::Serialize>(
    parents: &Vec<A>,
    id_extractor: fn(x: &A) -> B,
    children: &mut Vec<C>,
    pid_extractor: fn(x: &C) -> B,
    name: &'r str
) -> Value {
    children.sort_unstable_by_key(pid_extractor);
    let pids: Vec<B> = children.into_iter().map(|x| { pid_extractor(x) }).collect();
    json!(parents.into_iter().map(|x| -> Value {
        let mut j_parent = json!(x);
        j_parent[name] = find_with_keys(children, &pids, id_extractor(x));
        j_parent
    }).collect::<Value>())
}
