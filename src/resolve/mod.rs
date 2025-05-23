mod iter;
mod ops;
#[cfg(test)]
mod tests;
use crate::parser::{Node as QueryNode, NodeKind, Path, RangedIterator};
use iter::AnywhereIterator;
use kdl::{KdlDocument, KdlEntry, KdlNode};

pub(crate) struct Resolver<'k> {
    current_nodes: Vec<&'k KdlNode>,
    found_nodes: Vec<&'k KdlNode>,
}

impl<'k> Resolver<'k> {
    pub(crate) fn resolve<'q>(kdl_doc: &'k KdlDocument, query: Path<'q>) -> Vec<&'k KdlNode> {
        let mut r = Resolver {
            current_nodes: Vec::with_capacity(query.nodes().len()),
            found_nodes: Vec::new(),
        };
        r.resolve_query_node(query.nodes(), kdl_doc.nodes().iter());
        r.found_nodes
    }
    fn resolve_query_node<'q>(
        &mut self,
        query: &'q [QueryNode],
        it_nodes: impl Iterator<Item = &'k KdlNode>,
    ) {
        let query_node = unsafe { query.first().unwrap_unchecked() };
        let query_next = &query[1..];
        let compare_entries = |entries: &[KdlEntry]| {
            query_node
                .entries
                .as_ref()
                .map(|query_entries| entries == query_entries)
                .unwrap_or(true)
        };
        let node_compare_entries = |node: &&KdlNode| compare_entries(node.entries());

        match &query_node.node {
            NodeKind::Named(query_name) => {
                let it = it_nodes
                    .filter(|kdl_node| {
                        kdl_node
                            .name()
                            .repr()
                            .map(|node_name| node_name == query_name)
                            .unwrap_or(false)
                            && compare_entries(kdl_node.entries())
                    })
                    .ranged(query_node.range.as_ref());
                self.dispatch(query_next, it);
            }
            NodeKind::Any => self.dispatch(
                query_next,
                it_nodes
                    .filter(node_compare_entries)
                    .ranged(query_node.range.as_ref()),
            ),
            NodeKind::Anywhere => self.dispatch_itself(query_next, it_nodes.anywhere_nodes()),
            NodeKind::Parent => {
                let Some(parent) = self.current_nodes.pop() else {
                    return;
                };
                self.dispatch(
                    query_next,
                    std::iter::once(parent)
                        .filter(node_compare_entries)
                        .ranged(query_node.range.as_ref()),
                );
                self.current_nodes.push(parent);
            }
        }
    }
    fn dispatch<'q>(
        &mut self,
        query: &'q [QueryNode],
        it_nodes: impl Iterator<Item = &'k KdlNode>,
    ) {
        if query.is_empty() {
            it_nodes.for_each(|kdl_node| self.found_nodes.push(&kdl_node));
            return;
        }

        for kdl_node in it_nodes {
            let Some(kdl_doc) = kdl_node.children() else {
                self.resolve_query_node(query, std::iter::empty());
                continue;
            };
            self.current_nodes.push(kdl_node);
            self.resolve_query_node(query, kdl_doc.nodes().iter());
            let _ = self.current_nodes.pop();
        }
    }
    fn dispatch_itself<'q>(
        &mut self,
        query: &'q [QueryNode],
        it_nodes: impl Iterator<Item = &'k KdlNode>,
    ) {
        if query.is_empty() {
            it_nodes.for_each(|kdl_node| self.found_nodes.push(&kdl_node));
            return;
        }
        let boxed_iter: Box<dyn Iterator<Item = &KdlNode>> = Box::new(it_nodes);
        self.resolve_query_node(query, boxed_iter);
    }
}
