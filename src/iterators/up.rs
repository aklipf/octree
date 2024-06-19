use glam::Vec3;

use crate::octree::{AsPoint, Octree};

use super::{
    iter::{IterLeafNode, IterNode, IterStemNode},
    subdivide::Subdivide,
};

pub struct UpIterator<'a, T: AsPoint + Clone, N: Default> {
    pub(crate) tree: &'a Octree<T, N>,
    pub(crate) stack: Vec<(u32, bool, Vec3, f32)>,
}

pub trait IntoUpIterator {
    type Item;
    type Iter;
    fn iter_up(self) -> Self::Iter;
}

impl<'a, P: AsPoint + Clone, N: Default> IntoUpIterator for &'a Octree<P, N> {
    type Item = IterNode;
    type Iter = UpIterator<'a, P, N>;

    fn iter_up(self) -> Self::Iter {
        UpIterator {
            tree: self,
            stack: vec![(self.root, false, self.center, self.size)],
        }
    }
}

impl<'a, T: AsPoint + Clone, N: Default> Iterator for UpIterator<'a, T, N> {
    type Item = IterNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            return None;
        }

        let (idx, explored, center, size) = self.stack.pop().unwrap();

        if !explored {
            if (idx & 0x80000000) == 0 {
                self.stack.push((idx, true, center, size));

                let stem = &self.tree.stems[idx as usize];
                for i in 0..8 {
                    let child_idx: u32 = stem.childs_idx[i];
                    self.stack.push((
                        child_idx,
                        false,
                        center + (0.25 * size * Subdivide::offset(i)),
                        0.5 * size,
                    ));
                }
                return self.next();
            }
        }

        if (idx & 0x80000000) == 0 {
            return Some(IterNode::Stem(IterStemNode {
                node_idx: idx,
                center: center,
                size: size,
            }));
        }

        Some(IterNode::Leaf(IterLeafNode {
            node_idx: idx,
            center: center,
            size: size,
        }))
    }
}
