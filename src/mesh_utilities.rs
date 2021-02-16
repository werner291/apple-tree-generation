use nalgebra::{Vector3, Point3, UnitQuaternion};
use std::vec::Vec;

pub fn make_ring(center: &Point3<f32>, radius: f32, orientation: &UnitQuaternion<f32>) -> Vec<Point3<f32>> {

    const N_PTS : usize = 8;

    (0..N_PTS).map(|i| {
        let theta = (2.0 * std::f32::consts::PI) * (i as f32) / (N_PTS as f32);

        let from_center = orientation * Vector3::new(theta.cos() * radius, 0.0, theta.sin() * radius);

        let pt = center + from_center;

        Point3::new(pt.x, pt.y, pt.z)

    }).collect()
}