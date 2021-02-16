use nalgebra::{Vector3, Point3, UnitQuaternion, Isometry3, Translation3};
use std::vec::Vec;
use ncollide3d::procedural::{TriMesh, IndexBuffer};
use std::convert::{Into, From};
use std::clone::Clone;
use std::borrow::ToOwned;
use std::iter::{IntoIterator, once};
use std::option::Option::{None, Some};
use generational_arena::Index;
use crate::Tree;

const VERTICES_PER_RING: usize = 6;

pub fn make_tree_mesh(tree: &Tree) -> TriMesh<f32> {
    let mut vb = Vec::new();
    let mut ib = Vec::new();

    gen_vertices_for_node(tree, tree.root_node, &Isometry3::identity(), &mut vb, &mut ib);

    TriMesh::new(vb, None, None, Some(IndexBuffer::Unified(ib)))
}

pub fn gen_vertices_for_node(tree: &Tree, node: Index,
                             pos: &Isometry3<f32>,
                             vertex_buffer: &mut Vec<Point3<f32>>,
                             indices: &mut Vec<Point3<u32>>) -> Vec<u32> {

    let node = &tree.arena[node];

    let node_base_pos = pos * node.rotation;

    let ring = make_ring(&(node_base_pos * Point3::new(0.0, 0.0, 0.0)), 0.1, &node_base_pos.rotation);
    let ring_indices : Vec<u32> = (vertex_buffer.len() .. vertex_buffer.len() + VERTICES_PER_RING).into_iter().map(|x| x as u32 ).collect();

    vertex_buffer.extend_from_slice(&ring);

    let node_end_pos = node_base_pos * Translation3::new(0.0, node.length, 0.0);

    if node.children.is_empty() {
        let end_index = vertex_buffer.len() as u32;
        vertex_buffer.push(node_end_pos.translation.vector.to_owned().into());

        for i in 0 .. VERTICES_PER_RING {
            indices.push(Point3::new(ring_indices[i], end_index, ring_indices[(i+1) % VERTICES_PER_RING]));
        }
    } else {
        for (t, child) in node.children.iter() {
            let child_ring_indices = gen_vertices_for_node(tree, *child, &(node_base_pos * Translation3::new(0.0, node.length * t, 0.0)), vertex_buffer, indices);

            for i in 0 .. VERTICES_PER_RING {
                indices.push(Point3::new(ring_indices[i], child_ring_indices[i], ring_indices[(i+1) % VERTICES_PER_RING]));

                indices.push(Point3::new(ring_indices[(i+1) % VERTICES_PER_RING], child_ring_indices[i], child_ring_indices[(i+1) % VERTICES_PER_RING]));
            }
        }
    }

    ring_indices
}

// pub fn tree_branch_to_vertices(branch: &TreeBranch, parent_radius: f32, parent_xform: &Isometry3<f32>) -> TriMesh<f32> {
//
//     let mut current_xform = parent_xform.to_owned();
//
//     let mut current_radius = parent_radius;
//
//     let mut rings = Vec::new();
//
//     for (idx, link) in branch.nodes.iter().enumerate() {
//         current_radius *= link.radius_factor;
//         current_xform = current_xform * link.rotation;
//
//         let ring = make_ring(&current_xform.translation.vector.to_owned().into(), current_radius, &current_xform.rotation);
//
//         rings.push(ring);
//
//         current_xform = current_xform * Translation3::new(0.0, link.length,0.0);
//     }
//
//     let tip = current_xform.translation.vector.to_owned().into();
//
//     let vertices: Vec<Point3<f32>> = rings.into_iter().flatten().chain(once(tip)).collect();
//
//     let index_triangles: Vec<Point3<usize>> = (0..branch.nodes.len()).flat_map(|i| {
//         if i < branch.nodes.len() - 1 {
//             (0..VERTICES_PER_RING).flat_map(|j| {
//                 [
//                     Point3::new(
//                         i * VERTICES_PER_RING + j,
//                         (i+1) * VERTICES_PER_RING + j,
//                             (i+1) * VERTICES_PER_RING + j
//                     ),
//                     Point3::new(
//                         i * VERTICES_PER_RING + j,
//                         (i+1) * VERTICES_PER_RING + j,
//                         (i+1) * VERTICES_PER_RING + j
//                     )
//                 ]
//             })
//         } else {
//             (0..VERTICES_PER_RING).map(|j| {
//                 Point3::new(
//                     i * VERTICES_PER_RING + j,
//                     i * VERTICES_PER_RING + (j+1),
//                     (i+1) * VERTICES_PER_RING,
//                 )
//             })
//         }
//     }).collect();
//
//     todo!()
// }

pub fn make_ring(center: &Point3<f32>, radius: f32, orientation: &UnitQuaternion<f32>) -> Vec<Point3<f32>> {



    (0..VERTICES_PER_RING).map(|i| {
        let theta = (2.0 * std::f32::consts::PI) * (i as f32) / (VERTICES_PER_RING as f32);

        let from_center = orientation * Vector3::new(theta.cos() * radius, 0.0, theta.sin() * radius);

        let pt = center + from_center;

        Point3::new(pt.x, pt.y, pt.z)

    }).collect()
}