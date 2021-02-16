pub mod mesh_utilities;

use std::boxed::Box;
use std::default::Default;
use std::prelude::v1::Vec;

use na::{UnitQuaternion, Isometry3, Translation3};
use rand::Rng;
use rand::prelude::ThreadRng;
use std::f32::consts::PI;

use std::borrow::ToOwned;
use std::convert::From;
use generational_arena::{Arena, Index};

extern crate nalgebra as na;

pub struct TreeNodeData {
    pub length: f32,
    pub radius_factor: f32,
    pub rotation: UnitQuaternion<f32>,
    pub children: Vec<(f32, Index)>
}

impl TreeNodeData {
    fn random_zero_length(rng: &mut ThreadRng) -> Self {
        const TURN: f32 = PI / 6.0;
        TreeNodeData {
            length: 0.0,
            radius_factor: 1.0,
            rotation: UnitQuaternion::from_euler_angles(rng.gen_range(-TURN..TURN),
                                                        rng.gen_range(-TURN..TURN),
                                                        rng.gen_range(-TURN..TURN)),
            children: vec![]
        }
    }
}

pub struct Tree {
    pub arena: Arena<TreeNodeData>,
    pub root_node: Index
}

impl Default for Tree {
    fn default() -> Self {
        let mut arena = Arena::new();
        let root_node = arena.insert(TreeNodeData {
            length: 0.0,
            radius_factor: 0.0,
            rotation: UnitQuaternion::identity(),
            children: vec![]
        });
        Self {
            arena, root_node
        }
    }
}

impl Tree {
    pub fn grow(&mut self, t : f32) {

        let mut rng = rand::thread_rng();

        let indices : Vec<Index> = self.arena.iter().map(|(idx,_)| idx).collect();

        for idx in indices {

            let node = &mut self.arena[idx];

            node.length = node.length + t / (node.length + 1.0);

            if rng.gen_bool(t as f64 / (node.children.len() + 1) as f64) {
                let new_node = self.arena.insert(TreeNodeData::random_zero_length(&mut rng));
                self.arena[idx].children.push(( rng.gen_range(0.0 .. 1.0), new_node));
            }
        }




    }
}
