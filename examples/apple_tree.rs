

extern crate kiss3d;
extern crate nalgebra as na;

use std::boxed::Box;
use std::convert::{From, Into};
use std::default::Default;
use std::vec::Vec;

use kiss3d::light::Light;
use kiss3d::window::Window;

use na::{Isometry3, Point3, Translation3, Vector3};
use treegen::Tree;
use treegen::mesh_utilities;
use generational_arena::Index;
use kiss3d::resource::Mesh;
use ncollide3d::procedural::TriMesh;
use std::option::Option::None;
use std::rc::Rc;
use std::cell::RefCell;


extern crate treegen;

fn main() {
    let mut window = Window::new("Kiss3d: cube");

    window.set_background_color(0.5,0.5,1.0);

    window.set_light(Light::StickToCamera);

    let mut tree = Tree::default();

    let nc_mesh : TriMesh<f32> = mesh_utilities::make_tree_mesh(&tree);
    //
    // let mut mesh = Rc::new(RefCell::new(Mesh::new(
    //     nc_mesh.coords,
    //     nc_mesh.indices.unwrap_unified().into_iter().map(|tri| Point3::new(tri.x as u16, tri.y as u16, tri.z as u16)).collect(),
    //     None,
    //     None,
    //     true
    // )));

    let mut tree_sn = window.add_trimesh(nc_mesh, Vector3::new(1.0, 1.0, 1.0));
    tree_sn.set_color(0.6, 0.4, 0.2);

    let mut growth_rate = 0.01;

    while window.render() {

        growth_rate *= 0.999;

        tree.grow(growth_rate);

        let nc_mesh : TriMesh<f32> = mesh_utilities::make_tree_mesh(&tree);
        window.remove_node(&mut tree_sn);
        tree_sn = window.add_trimesh(nc_mesh, Vector3::new(1.0, 1.0, 1.0));
        tree_sn.set_color(0.6, 0.4, 0.2);

        draw_node(&mut window, &tree, tree.root_node, Isometry3::<f32>::identity());
    }
}


fn draw_node(window: &mut Window, tree: &Tree, node: Index, mut base_pos : Isometry3<f32>) {

    let node = &tree.arena[node];

        let seg_base_pos = base_pos * node.rotation;

        let seg_start: Point3<f32> = seg_base_pos.translation.vector.into();

        for (t, branch) in node.children.iter() {
            draw_node(window,tree,*branch, &seg_base_pos * &Translation3::new(0.0, node.length* t, 0.0));
        }

        base_pos = seg_base_pos * &Translation3::new(0.0, node.length, 0.0);
        let seg_end: Point3<f32> = base_pos.translation.vector.into();

        window.draw_line(&seg_start, &seg_end, &Point3::new(0.6, 0.4, 0.2));

}