use intersection_wasm::no_div_tri_tri_isect;

const EPSILON: f32 = 0.000_001;

#[test]
fn no_div_tri_tri_isect_separated() {
    let v0 = [0., 0., -1.];
    let v1 = [0., 0., 0.];
    let v2 = [0., 1.751, 0.];

    let u0 = [-0.5, 0.8755, 0.5];
    let u1 = [0.5, 0.8755, 1.5];
    let u2 = [0.5, 0.8755, 0.5];

    assert_eq!(
        no_div_tri_tri_isect(&v0, &v1, &v2, &u0, &u1, &u2, Some(EPSILON)),
        false
    );
}

#[test]
fn no_div_tri_tri_isect_intersected() {
    let v0 = [0., 0., -1.];
    let v1 = [0., 0., 0.];
    let v2 = [0., 1.751, 0.];

    let u0 = [-0.5, 0.8755, -0.5];
    let u1 = [0.5, 0.8755, 0.5];
    let u2 = [0.5, 0.8755, -0.5];

    assert_eq!(
        no_div_tri_tri_isect(&v0, &v1, &v2, &u0, &u1, &u2, Some(EPSILON)),
        true
    );
}
