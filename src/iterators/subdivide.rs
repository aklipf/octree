use glam::{vec3, Vec3};

pub(crate) struct Subdivide {
    pub(crate) current: u32,
    pub(crate) delimiter: [u32; 9],
    pub(crate) center: Vec3,
    pub(crate) size: f32,
}

pub(crate) struct Subspace {
    pub(crate) begin: u32,
    pub(crate) end: u32,
    pub(crate) center: Vec3,
    pub(crate) size: f32,
}

impl Subdivide {
    pub(super) fn offset(idx: usize) -> Vec3 {
        match idx {
            0 => vec3(-1.0, -1.0, -1.0),
            1 => vec3(-1.0, -1.0, 1.0),
            2 => vec3(-1.0, 1.0, -1.0),
            3 => vec3(-1.0, 1.0, 1.0),
            4 => vec3(1.0, -1.0, -1.0),
            5 => vec3(1.0, -1.0, 1.0),
            6 => vec3(1.0, 1.0, -1.0),
            7 => vec3(1.0, 1.0, 1.0),
            _ => panic!("unkown offset"),
        }
    }
}

impl Iterator for Subdivide {
    type Item = Subspace;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= 8 {
            return None;
        }

        let idx: u32 = self.current as u32;
        self.current += 1;

        Some(Subspace {
            begin: self.delimiter[idx as usize] as u32,
            end: self.delimiter[(idx + 1) as usize] as u32,
            center: self.center + (0.25 * self.size * Subdivide::offset(idx as usize)),
            size: 0.5 * self.size,
        })
    }
}
