//! Implementations of different minimum spanning tree algorithms for the sake
//! of learning.
//! DEFINITION: Minimum Spanning Tree (MST): Also known as a minimum *weight*
//! spanning tree, is a subset of the edges of a connected, edge-weighted,
//! undirected graph that connects all the vertices together without any cycles,
//! and with the minimum possible sum of edge weights.
//! IOW: A spanning tree whose sum of edge weights is as small as possible.
//!
//! GENERAL CASE: Minumum Spanning Forest (MSF): A collection of MSTs that
//! results from a disconnected graph.
//!
//! USEFULNESS: Not only avoid redundant paths in a network, but also minimize
//! the cost of the network, whether the cost is due to traversal time/distace,
//! physical resources, etc.
//!
//! Their usefulness in hierarchical clustering stems from multiple qualities:
//! - Hierarchies should not have cycles
//! - Minimizing distances means points near each other are more likely to be
//!   connected, and thus more likely to be contained within the same cluster.
//! - Edge thresholding, or removing edges that exceed some distance, can help
//!   to break apart clusters.
//! - Noise reduction -- due to emphasis on shortest paths.
//! - Adaptability -- works with different distance or weight metrics.
//! - Visualization and analysis.
mod binary_search;
mod min_spanning_tree;
mod sorting;


fn main() {
    println!("Hello, world!");
}
