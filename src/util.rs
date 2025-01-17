use bevy::prelude::*;
use bevy_easings::Lerp;

const EPSILON: f32 = 0.001;

pub fn calculate_from_translation_and_focus(translation: Vec3, focus: Vec3) -> (f32, f32, f32) {
    let comp_vec = translation - focus;
    let mut radius = comp_vec.length();
    if radius == 0.0 {
        radius = 0.05; // Radius 0 causes problems
    }
    let alpha = if comp_vec.x == 0.0 && comp_vec.z >= 0.0 {
        0.0
    } else {
        (comp_vec.z / (comp_vec.x.powi(2) + comp_vec.z.powi(2)).sqrt()).acos()
    };
    let beta = (comp_vec.y / radius).asin();
    (alpha, beta, radius)
}

/// Update `transform` based on alpha, beta, and the camera's focus and radius
pub fn update_orbit_transform(
    alpha: f32,
    beta: f32,
    radius: f32,
    focus: Vec3,
    transform: &mut Transform,
    basis: &Transform,
) {
    let mut new_transform = *basis;
    new_transform.rotation *= Quat::from_rotation_y(alpha) * Quat::from_rotation_x(-beta);
    new_transform.translation += focus + new_transform.rotation * Vec3::new(0.0, 0.0, radius);
    *transform = new_transform;
}

pub fn approx_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub fn lerp_and_snap_f32(from: f32, to: f32, smoothness: f32) -> f32 {
    let t = 1.0 - smoothness;
    let mut new_value = from.lerp(&to, &t);
    if smoothness < 1.0 && approx_equal(new_value, to) {
        new_value = to;
    }
    new_value
}

pub fn lerp_and_snap_vec3(from: Vec3, to: Vec3, smoothness: f32) -> Vec3 {
    let t = 1.0 - smoothness;
    let mut new_value = from.lerp(to, t);
    if smoothness < 1.0 && approx_equal((new_value - to).length(), 0.0) {
        new_value.x = to.x;
    }
    new_value
}

#[cfg(test)]
mod calculate_from_translation_and_focus_tests {
    use super::*;
    use float_cmp::approx_eq;
    use std::f32::consts::PI;

    #[test]
    fn zero() {
        let translation = Vec3::new(0.0, 0.0, 0.0);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert_eq!(alpha, 0.0);
        assert_eq!(beta, 0.0);
        assert_eq!(radius, 0.05);
    }

    #[test]
    fn in_front() {
        let translation = Vec3::new(0.0, 0.0, 5.0);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert_eq!(alpha, 0.0);
        assert_eq!(beta, 0.0);
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn to_the_side() {
        let translation = Vec3::new(5.0, 0.0, 0.0);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert!(approx_eq!(f32, alpha, PI / 2.0));
        assert_eq!(beta, 0.0);
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn above() {
        let translation = Vec3::new(0.0, 5.0, 0.0);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert_eq!(alpha, 0.0);
        assert!(approx_eq!(f32, beta, PI / 2.0));
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn arbitrary() {
        let translation = Vec3::new(0.92563736, 3.864204, -1.0105048);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert!(approx_eq!(f32, alpha, 2.4));
        assert!(approx_eq!(f32, beta, 1.23));
        assert_eq!(radius, 4.1);
    }
}

#[cfg(test)]
mod approx_equal_tests {
    use super::*;

    #[test]
    fn same_value_is_approx_equal() {
        assert!(approx_equal(1.0, 1.0));
    }

    #[test]
    fn value_within_threshold_is_approx_equal() {
        assert!(approx_equal(1.0, 1.0000001));
    }

    #[test]
    fn value_outside_threshold_is_not_approx_equal() {
        assert!(!approx_equal(1.0, 1.01));
    }
}

#[cfg(test)]
mod lerp_and_snap_f32_tests {
    use super::*;

    #[test]
    fn lerps_when_output_outside_snap_threshold() {
        let out = lerp_and_snap_f32(1.0, 2.0, 0.5);
        assert_eq!(out, 1.5);
    }

    #[test]
    fn snaps_to_target_when_inside_threshold() {
        let out = lerp_and_snap_f32(1.9991, 2.0, 0.5);
        assert_eq!(out, 2.0);
        let out = lerp_and_snap_f32(1.9991, 2.0, 0.1);
        assert_eq!(out, 2.0);
        let out = lerp_and_snap_f32(1.9991, 2.0, 0.9);
        assert_eq!(out, 2.0);
    }

    #[test]
    fn does_not_snap_if_smoothness_is_one() {
        // Smoothness of one results in the value not changing, so it doesn't make sense to snap
        let out = lerp_and_snap_f32(1.9991, 2.0, 1.0);
        assert_eq!(out, 1.9991);
    }
}

#[cfg(test)]
mod lerp_and_snap_vec3_tests {
    use super::*;

    #[test]
    fn lerps_when_output_outside_snap_threshold() {
        let out = lerp_and_snap_vec3(Vec3::ZERO, Vec3::X, 0.5);
        assert_eq!(out, Vec3::X * 0.5);
    }

    #[test]
    fn snaps_to_target_when_inside_threshold() {
        let out = lerp_and_snap_vec3(Vec3::X * 0.9991, Vec3::X, 0.5);
        assert_eq!(out, Vec3::X);
        let out = lerp_and_snap_vec3(Vec3::X * 0.9991, Vec3::X, 0.1);
        assert_eq!(out, Vec3::X);
        let out = lerp_and_snap_vec3(Vec3::X * 0.9991, Vec3::X, 0.9);
        assert_eq!(out, Vec3::X);
    }

    #[test]
    fn does_not_snap_if_smoothness_is_one() {
        // Smoothness of one results in the value not changing, so it doesn't make sense to snap
        let out = lerp_and_snap_vec3(Vec3::X * 0.9991, Vec3::X, 1.0);
        assert_eq!(out, Vec3::X * 0.9991);
    }
}
