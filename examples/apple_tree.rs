

extern crate kiss3d;
extern crate nalgebra as na;

use std::convert::Into;
use std::default::Default;

use kiss3d::light::Light;
use kiss3d::window::Window;

use na::{Isometry3, Point3};
use treegen::{TreeBranch, BranchPositions};
use std::borrow::ToOwned;

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

        let positions = tree.node_positions(Isometry3::<f32>::identity());

        draw_branch(&mut window, &positions);
    }
}

fn draw_branch(window: &mut Window, tree: &BranchPositions) {

    for (idx, (base_pos,children)) in tree.node_positions.iter().enumerate() {

        let seg_start: Point3<f32> = base_pos.translation.vector.into();

        let seg_end: Point3<f32> = if idx < tree.node_positions.len() - 1 {
            tree.node_positions[idx+1].0.translation.vector.into()
        } else {
            tree.tip.to_owned()
        };

        window.draw_line(&seg_start, &seg_end, &Point3::new(0.6, 0.4, 0.2));

        for child in children {
            draw_branch(window, child);
        }
    }
}