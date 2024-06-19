use glam::Vec3;

use crate::octree::{AsPoint, Octree};

use super::{
    iter::{IterLeafNode, IterNode, IterStemNode},
    subdivide::Subdivide,
};

pub struct DownIterator<'a, T: AsPoint + Clone, N: Default> {
    pub(crate) tree: &'a Octree<T, N>,
    pub(crate) stack: Vec<(u32, Vec3, f32)>,
}

pub trait IntoDownIterator {
    type Item;
    type Iter;
    fn iter_down(self) -> Self::Iter;
}

impl<'a, P: AsPoint + Clone, N: Default> IntoDownIterator for &'a Octree<P, N> {
    type Item = IterNode;
    type Iter = DownIterator<'a, P, N>;

    fn iter_down(self) -> Self::Iter {
        DownIterator {
            tree: self,
            stack: vec![(self.root, self.center, self.size)],
        }
    }
}

impl<'a, T: AsPoint + Clone, N: Default> Iterator for DownIterator<'a, T, N> {
    type Item = IterNode;

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
            return Some(IterNode::Stem(IterStemNode {
                node_idx: idx,
                center: center,
                size: size,
            }));
        }

        Some(IterNode::Leaf(IterLeafNode {
            node_idx: idx & 0x7fffffff,
            center: center,
            size: size,
        }))
    }
}
