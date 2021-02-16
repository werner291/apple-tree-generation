use nalgebra::{Vector3, Point3, UnitQuaternion, Isometry3, Translation3};
use std::vec::Vec;
use crate::{TreeBranch, TreeBranchSegment};
use ncollide3d::procedural::TriMesh;
use std::convert::{Into, From};
use std::clone::Clone;
use std::borrow::ToOwned;
use std::iter::{IntoIterator, once};

const VERTICES_PER_RING: usize = 8;

pub fn tree_branch_to_vertices(branch: &TreeBranch, parent_radius: f32, parent_xform: &Isometry3<f32>) -> TriMesh<f32> {

    let mut current_xform = parent_xform.to_owned();

    let mut current_radius = parent_radius;

    let mut rings = Vec::new();

    for (idx, link) in branch.nodes.iter().enumerate() {
        current_radius *= link.radius_factor;
        current_xform = current_xform * link.rotation;

        let ring = make_ring(&current_xform.translation.vector.to_owned().into(), current_radius, &current_xform.rotation);

        rings.push(ring);

        current_xform = current_xform * Translation3::new(0.0, link.length,0.0);
    }

    let tip = current_xform.translation.vector.to_owned().into();

    let vertices: Vec<Point3<f32>> = rings.into_iter().flatten().chain(once(tip)).collect();

    let index_triangles: Vec<Point3<usize>> = (0..branch.nodes.len()).flat_map(|i| {
        if i < branch.nodes.len() - 1 {
            (0..VERTICES_PER_RING).flat_map(|j| {
                [
                    Point3::new(
                        i * VERTICES_PER_RING + j,
                        (i+1) * VERTICES_PER_RING + j,
                            (i+1) * VERTICES_PER_RING + j
                    ),
                    Point3::new(
                        i * VERTICES_PER_RING + j,
                        (i+1) * VERTICES_PER_RING + j,
                        (i+1) * VERTICES_PER_RING + j
                    )
                ]
            })
        } else {
            (0..VERTICES_PER_RING).map(|j| {
                Point3::new(
                    i * VERTICES_PER_RING + j,
                    i * VERTICES_PER_RING + (j+1),
                    (i+1) * VERTICES_PER_RING,
                )
            })
        }
    }).collect();

    todo!()
}

pub fn make_ring(center: &Point3<f32>, radius: f32, orientation: &UnitQuaternion<f32>) -> Vec<Point3<f32>> {



    (0..VERTICES_PER_RING).map(|i| {
        let theta = (2.0 * std::f32::consts::PI) * (i as f32) / (VERTICES_PER_RING as f32);

        let from_center = orientation * Vector3::new(theta.cos() * radius, 0.0, theta.sin() * radius);

        let pt = center + from_center;

        Point3::new(pt.x, pt.y, pt.z)

    }).collect()
}