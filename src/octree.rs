use glam::Vec3;

use crate::iterators::subdivide::Subdivide;

pub trait AsPoint {
    fn get_point(&self) -> &Vec3;
}

impl AsPoint for Vec3 {
    #[inline]
    fn get_point(&self) -> &Vec3 {
        return self;
    }
}

#[derive(Debug, Default)]
pub struct StemNode {
    pub childs_idx: [u32; 8],
}

#[derive(Debug)]
pub struct LeafNode {
    pub(crate) begin: u32,
    pub(crate) end: u32,
}

#[derive(Debug, Default)]
pub struct EmptyNode {}

#[derive(Debug)]
pub struct Octree<P: AsPoint + Clone = Vec3, N: Default = EmptyNode> {
    pub(crate) root: u32,
    pub stems: Vec<StemNode>,
    pub leafs: Vec<LeafNode>,
    pub stems_data: Vec<N>,
    pub leafs_data: Vec<N>,
    pub indices: Vec<u32>,
    pub points: Vec<P>,
    pub(crate) center: Vec3,
    pub(crate) size: f32,
}

impl<P: AsPoint + Clone, N: Default> Octree<P, N> {
    pub fn fixed_depth(points: &[P], depth: u32) -> Octree<P, N> {
        let (center, size) = Self::get_dimentions(points);

        let mut tree = Octree {
            root: 0,
            stems: Default::default(),
            leafs: Default::default(),
            stems_data: Default::default(),
            leafs_data: Default::default(),
            indices: (0..(points.len() as u32)).collect::<Vec<u32>>(),
            points: points.to_vec(),
            center: center,
            size: size,
        };
        tree.root = tree.fixed_depth_recursive(0, points.len() as u32, center, size, depth);

        tree
    }

    pub fn variable_depth(points: &[P], bucket_size: u32) -> Octree<P, N> {
        let (center, size) = Self::get_dimentions(points);

        let mut tree = Octree {
            root: 0,
            stems: Default::default(),
            leafs: Default::default(),
            stems_data: Default::default(),
            leafs_data: Default::default(),
            indices: (0..(points.len() as u32)).collect::<Vec<u32>>(),
            points: points.to_vec(),
            center: center,
            size: size,
        };
        tree.root =
            tree.variable_depth_recursive(0, points.len() as u32, center, size, bucket_size);

        tree
    }

    fn get_dimentions(points: &[P]) -> (Vec3, f32) {
        let mut min: Vec3 = Vec3::INFINITY;
        let mut max: Vec3 = Vec3::NEG_INFINITY;

        for x in points {
            min = min.min(*x.get_point());
            max = max.max(*x.get_point());
        }

        let center = 0.5 * (max + min);
        let size = (max - min).max_element();

        (center, size)
    }

    #[inline]
    pub(crate) fn get_point(&self, idx: u32) -> &Vec3 {
        &self.points[self.indices[idx as usize] as usize].get_point()
    }

    #[inline]
    pub(crate) fn get_data(&self, idx: u32) -> &P {
        &self.points[self.indices[idx as usize] as usize]
    }

    #[inline]
    fn swap_points(&mut self, idx_a: u32, idx_b: u32) {
        self.indices.swap(idx_a as usize, idx_b as usize);
    }

    fn split<const DIM: usize>(&mut self, begin: u32, end: u32, pivot: f32) -> u32 {
        if begin >= end {
            return begin;
        }
        let mut idx_down: u32 = begin;
        let mut idx_up: u32 = end - 1;

        while idx_down <= idx_up {
            let error_down = self.get_point(idx_down)[DIM] >= pivot;
            let error_up = self.get_point(idx_up)[DIM] < pivot;

            if error_down && error_up {
                self.swap_points(idx_down, idx_up);
                idx_down += 1;
                idx_up -= 1;
                continue;
            }

            if !error_down {
                idx_down += 1;
                if idx_down >= end {
                    return end;
                }
            }
            if !error_up {
                if idx_up <= begin {
                    return begin;
                }
                idx_up -= 1;
            }
        }

        idx_down
    }

    // split points into an octree branch and return the delimiter id
    fn subdivide(&mut self, begin: u32, end: u32, center: Vec3, size: f32) -> Subdivide {
        let pivot_x = self.split::<0>(begin, end, center.x);

        let pivot_y_low = self.split::<1>(begin, pivot_x, center.y);
        let pivot_y_high = self.split::<1>(pivot_x, end, center.y);

        let pivot_z_low_low = self.split::<2>(begin, pivot_y_low, center.y);
        let pivot_z_low_high = self.split::<2>(pivot_y_low, pivot_x, center.y);
        let pivot_z_high_low = self.split::<2>(pivot_x, pivot_y_high, center.y);
        let pivot_z_high_high = self.split::<2>(pivot_y_high, end, center.y);

        Subdivide {
            current: 0,
            delimiter: [
                begin as u32,
                pivot_z_low_low as u32,
                pivot_y_low as u32,
                pivot_z_low_high as u32,
                pivot_x as u32,
                pivot_z_high_low as u32,
                pivot_y_high as u32,
                pivot_z_high_high as u32,
                end as u32,
            ],
            center: center,
            size: size,
        }
    }

    fn fixed_depth_recursive(
        &mut self,
        begin: u32,
        end: u32,
        center: Vec3,
        size: f32,
        depth: u32,
    ) -> u32 {
        if depth == 0 || (end - begin) <= 1 {
            let idx = 0x80000000 | (self.leafs.len() as u32);
            self.leafs.push(LeafNode {
                begin: begin as u32,
                end: end as u32,
            });
            self.leafs_data.push(Default::default());
            return idx;
        }

        let mut node: StemNode = Default::default();

        for (idx, subspace) in self.subdivide(begin, end, center, size).enumerate() {
            let child_idx = self.fixed_depth_recursive(
                subspace.begin,
                subspace.end,
                subspace.center,
                subspace.size,
                depth - 1,
            );
            node.childs_idx[idx] = child_idx as u32;
        }

        let node_idx = self.stems.len() as u32;
        self.stems.push(node);
        self.stems_data.push(Default::default());

        node_idx
    }

    fn variable_depth_recursive(
        &mut self,
        begin: u32,
        end: u32,
        center: Vec3,
        size: f32,
        bucket_size: u32,
    ) -> u32 {
        if (end - begin) <= bucket_size {
            let idx = 0x80000000 | (self.leafs.len() as u32);
            self.leafs.push(LeafNode { begin, end });

            if std::mem::size_of::<N>() != 0 {
                self.leafs_data.push(Default::default());
            }

            return idx;
        }

        let mut node: StemNode = Default::default();

        for (idx, subspace) in self.subdivide(begin, end, center, size).enumerate() {
            let child_idx = self.variable_depth_recursive(
                subspace.begin,
                subspace.end,
                subspace.center,
                subspace.size,
                bucket_size,
            );
            node.childs_idx[idx] = child_idx as u32;
        }

        let node_idx = self.stems.len() as u32;
        self.stems.push(node);
        if std::mem::size_of::<N>() != 0 {
            self.stems_data.push(Default::default());
        }

        node_idx
    }
}

#[cfg(test)]
mod tests {
    use glam::{vec3, Vec3};
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};

    use super::{EmptyNode, Octree};

    fn random_points(n: usize) -> Vec<Vec3> {
        let mut rng = thread_rng();
        let side = Uniform::new(-1.0f32, 1.0f32);

        (0..n)
            .map(|_| vec3(rng.sample(side), rng.sample(side), rng.sample(side)))
            .collect()
    }

    fn test_split<const DIM: usize>(n: usize) {
        let mut tree = Octree::<Vec3, EmptyNode> {
            root: 0,
            stems: Default::default(),
            leafs: Default::default(),
            stems_data: Default::default(),
            leafs_data: Default::default(),
            indices: (0..(n as u32)).collect::<Vec<u32>>(),
            points: random_points(n),
            center: Vec3::ZERO,
            size: 0.0,
        };

        let pivot = tree.split::<DIM>(0, n as u32, 0.0) as usize;

        for i in 0..(pivot as u32) {
            assert!(tree.get_point(i)[DIM] < 0.0);
        }
        for i in (pivot as u32)..(n as u32) {
            assert!(tree.get_point(i)[DIM] >= 0.0);
        }
    }

    #[test]
    fn octree_split() {
        for _ in 0..16 {
            for n in [0, 1, 7, 8, 15, 16, 128, 135] {
                test_split::<0>(n);
                test_split::<1>(n);
                test_split::<2>(n);
            }
        }
    }
}
