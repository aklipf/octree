use glam::Vec3;

use crate::octree::{AsPoint, Octree};

use super::subdivide::Subdivide;

pub struct TreeIterator<'a, T: AsPoint + Clone> {
    pub(crate) tree: &'a Octree<T>,
    pub(crate) stack: Vec<(u32, Vec3, f32)>,
    pub(crate) point: u32,
}

pub enum TreeElements<'a, T: AsPoint + Clone> {
    Node { center: Vec3, size: f32 },
    Point { point: &'a T },
}

impl<'a, T: AsPoint + Clone> IntoIterator for &'a Octree<T> {
    type Item = TreeElements<'a, T>;
    type IntoIter = TreeIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        TreeIterator::<T> {
            tree: self,
            stack: vec![(self.root, self.center, self.size)],
            point: 0,
        }
    }
}

impl<'a, T: AsPoint + Clone> Iterator for TreeIterator<'a, T> {
    type Item = TreeElements<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            return None;
        }

        let (idx, center, size) = *self.stack.last().unwrap();

        if (idx & 0x80000000) == 0 {
            self.stack.pop();
            for (i, child) in self.tree.stems[idx as usize].childs_idx.iter().enumerate() {
                self.stack.push((
                    *child,
                    center + (0.25 * size * Subdivide::offset(i)),
                    0.5 * size,
                ));
            }
            return Some(TreeElements::Node {
                center: center,
                size: size,
            });
        }

        let idx = idx & 0x7fffffff;
        let node = &self.tree.leaf[idx as usize];
        let n_points = node.end - node.begin;

        if self.point >= n_points {
            self.stack.pop();
            self.point = 0;
            return Some(TreeElements::Node {
                center: center,
                size: size,
            });
        }

        let point_idx = node.begin + self.point;

        self.point += 1;
        Some(TreeElements::Point {
            point: &self.tree.get_data(point_idx),
        })
    }
}
