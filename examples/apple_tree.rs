

extern crate kiss3d;
extern crate nalgebra as na;

use std::boxed::Box;
use std::convert::{From, Into};
use std::default::Default;
use std::prelude::v1::Vec;

use kiss3d::light::Light;
use kiss3d::window::Window;
use ncollide3d::procedural::TriMesh;
use rand::{Rng, thread_rng};

use na::{Isometry3, Point3, Rotation3, Translation3, UnitQuaternion, Vector3};
use treegen::TreeBranch;

extern crate treegen;

fn main() {
    let mut window = Window::new("Kiss3d: cube");

    window.set_background_color(0.5,0.5,1.0);

    window.set_light(Light::StickToCamera);

    let mut tree = TreeBranch::default();

    let mut growth_rate = 0.01;

    while window.render() {

        growth_rate *= 0.999;

        tree.grow(growth_rate);

        draw_branch(&mut window, &mut tree, Isometry3::<f32>::identity());
    }
}

fn draw_branch(window: &mut Window, tree: &TreeBranch, mut base_pos : Isometry3<f32>) {

    for node in tree.nodes.iter() {

        let seg_base_pos = base_pos * node.rotation;

        let seg_start: Point3<f32> = seg_base_pos.translation.vector.into();


        for (t, branch) in node.children.iter() {
            draw_branch(window,branch,seg_base_pos.clone() * (&Translation3::new(0.0, node.length * t, 0.0)));
        }

        base_pos = seg_base_pos * &Translation3::new(0.0, node.length, 0.0);
        let seg_end: Point3<f32> = base_pos.translation.vector.into();

        window.draw_line(&seg_start, &seg_end, &Point3::new(0.6, 0.4, 0.2));

    }
}