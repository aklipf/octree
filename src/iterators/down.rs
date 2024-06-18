use crate::octree::{AsPoint, Octree};

use super::{
    iter::{LeafNode, Node, StemNode, TreeIterator},
    subdivide::Subdivide,
};

type DownIterator<'a, P, N> = TreeIterator<'a, P, N>;

pub trait IntoDownIterator {
    type Item;
    type Iter;
    fn iter_down(self) -> Self::Iter;
}

impl<'a, P: AsPoint + Clone, N: Default> IntoDownIterator for &'a Octree<P, N> {
    type Item = Node<'a, P, N>;
    type Iter = DownIterator<'a, P, N>;

    fn iter_down(self) -> Self::Iter {
        DownIterator {
            tree: self,
            stack: vec![(self.root, self.center, self.size)],
        }
    }
}

impl<'a, T: AsPoint + Clone, N: Default> Iterator for DownIterator<'a, T, N> {
    type Item = Node<'a, T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            return None;
        }

        let (idx, center, size) = self.stack.pop().unwrap();

        if (idx & 0x80000000) == 0 {
            for (i, child) in self.tree.stems[idx as usize].childs_idx.iter().enumerate() {
                self.stack.push((
                    *child,
                    center + (0.25 * size * Subdivide::offset(i)),
                    0.5 * size,
                ));
            }
            return Some(Node::Stem(StemNode {
                tree: &self.tree,
                node_idx: idx,
                center: center,
                size: size,
            }));
        }

        Some(Node::Leaf(LeafNode {
            tree: &self.tree,
            node_idx: idx & 0x7fffffff,
            center: center,
            size: size,
        }))
    }
}
