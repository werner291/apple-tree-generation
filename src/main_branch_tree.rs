use std::prelude::v1::Vec;

/// A general-purpose datastructure for a tree with a distinct "main" branch,
/// where every node on the main branch can have child branches.
struct MainBranchTree<N> {
    // TODO Maybe use some kind of arena allocator?
    nodes: Vec<(N,Vec<MainBranchTree<N>>)>
}