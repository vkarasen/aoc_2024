use bimap::BiMap;

use crate::table::TableIdx;

use petgraph::prelude::*;

pub type NodeMap = BiMap<NodeIndex, TableIdx>;

pub fn get_node_or_insert<Ty: petgraph::EdgeType>(pos: &TableIdx, graph: &mut Graph<(),(), Ty>, nodemap: &mut NodeMap) -> NodeIndex {
    if let Some(n) = nodemap.get_by_right(pos) {
        *n
    } else {
        let n = graph.add_node(());
        nodemap.insert(n, *pos);
        n
    }
}

