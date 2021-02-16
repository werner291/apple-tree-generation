use std::boxed::Box;
use std::default::Default;
use std::prelude::v1::Vec;

use na::UnitQuaternion;
use rand::Rng;
use rand::prelude::ThreadRng;
use std::f32::consts::PI;

// pub mod mesh_utilities;
extern crate nalgebra as na;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct TreeBranchSegment {
    pub length: f32, // TODO code defensively
    pub taper: f32,
    // apples: Vec<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub children: Vec<(f32, TreeBranch)>
}

impl Default for TreeBranchSegment {
    fn default() -> Self {
        TreeBranchSegment {
            length: 1.0,
            taper: 1.0,
            rotation: UnitQuaternion::identity(),
            children: vec![]
        }
    }
}

impl TreeBranchSegment {
    fn random_zero_length(rng: &mut ThreadRng) -> TreeBranchSegment {
        const TURN: f32 = PI / 4.0;
        TreeBranchSegment {
            length: 0.0,
            taper: 0.0,
            rotation: UnitQuaternion::from_euler_angles(rng.gen_range(-TURN..TURN),
                                                        rng.gen_range(-TURN..TURN),
                                                        rng.gen_range(-TURN..TURN)),
            children: vec![]
        }
    }
}

pub struct TreeBranch {
    initial_radius: f32,
    pub nodes: Vec<TreeBranchSegment>
}

impl Default for TreeBranch {
    fn default() -> Self {
        TreeBranch {
            initial_radius: 0.0,
            nodes: vec![TreeBranchSegment::default()]
        }
    }
}

impl TreeBranch {
    pub fn grow(&mut self, t : f32) {

        let mut rng = rand::thread_rng();

        for node in self.nodes.iter_mut() {

            for (_, child) in node.children.iter_mut() {
                child.grow(t / 3.0);
            }

            node.length = node.length + t / (node.length + 1.0);

            if rng.gen_bool(t as f64 / (node.children.len() + 1) as f64) {
                node.children.push((rng.gen_range(0.0 .. 1.0), TreeBranch {
                    initial_radius: 0.0,
                    nodes: vec![TreeBranchSegment::random_zero_length(&mut rng)]
                }))
            }
        }

        if rng.gen_bool(t as f64) {
            self.nodes.push(TreeBranchSegment::random_zero_length(&mut rng))
        }
    }


}
