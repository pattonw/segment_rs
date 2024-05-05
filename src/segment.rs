use disjoint_sets::UnionFind;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::wrap_pymodule;
use pyo3::PyResult;

use std::collections::HashMap;

use std;

#[pyfunction]
fn connected_components(
    nodes: Vec<usize>,
    edges: Vec<(usize, usize)>,
    scores: Option<Vec<f32>>,
    threshold: Option<f32>,
) -> Vec<usize> {
    // set defaults for threshold (0.0) and scores (-1.0 for all edges)
    let threshold = threshold.unwrap_or(0.0);
    let scores = scores.unwrap_or(vec![-1.0; edges.len()]);

    // remap nodes to 0..n
    let mut next_id = (0..).into_iter();
    let mut node_mapping: HashMap<usize, usize> = HashMap::new();
    let mut rev_node_mapping: HashMap<usize, usize> = HashMap::new();
    nodes.iter().for_each(|&x| {
        if !node_mapping.contains_key(&x) {
            node_mapping.insert(x, next_id.next().unwrap());
        };
        if !rev_node_mapping.contains_key(&node_mapping[&x]) {
            rev_node_mapping.insert(node_mapping[&x], x);
        };
    });

    // remap and filter edges
    let mapped_edges: Vec<(usize, usize)> = edges
        .iter()
        .zip(scores)
        .filter(|((u, v), s)| {
            node_mapping.contains_key(u) && node_mapping.contains_key(v) && s <= &threshold
        })
        .map(|((u, v), _s)| (node_mapping[u], node_mapping[v]))
        .collect();

    // print skipped edges
    let skipped_edges = edges.len() - mapped_edges.len();
    println!(
        "Skipped {} edges due the end points not being in nodes",
        skipped_edges
    );

    // find connected components
    let mut components = UnionFind::<usize>::new(next_id.next().unwrap());
    for (u, v) in mapped_edges.into_iter() {
        components.union(u, v);
    }

    // collect connected components
    nodes
        .iter()
        .map(|node_id| rev_node_mapping[&components.find(node_mapping[&node_id])])
        .collect()
}

#[pymodule]
pub fn segment(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(connected_components))?;

    Ok(())
}
