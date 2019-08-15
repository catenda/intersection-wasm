#![allow(clippy::many_single_char_names)]

#[macro_use]
mod macros;
mod utils;

use wasm_bindgen::prelude::*;

#[allow(clippy::cognitive_complexity)]
fn coplanar_tri_tri(
    n: [f32; 3],
    v0: &[f32],
    v1: &[f32],
    v2: &[f32],
    u0: &[f32],
    u1: &[f32],
    u2: &[f32],
) -> bool {
    let i0;
    let i1;

    // first project onto an axis-aligned plane, that maximizes the area
    // of the triangles, compute indices: i0, i1
    let a: [f32; 3] = [n[0].abs(), n[1].abs(), n[2].abs()];

    if a[0] > a[1] {
        if a[0] > a[2] {
            i0 = 1; // a[0] is greatest
            i1 = 2;
        } else {
            i0 = 0; // a[2] is greatest
            i1 = 1;
        }
    } else {
        // a[0] <= a[1]
        if a[2] > a[1] {
            i0 = 0; // a[2] is greatest
            i1 = 1;
        } else {
            i0 = 0; // a[1] is greatest
            i1 = 2;
        }
    }

    // test all edges of triangle 1 against the edges of triangle 2
    edge_against_tri_edges!(v0, v1, u0, u1, u2, i0, i1);
    edge_against_tri_edges!(v1, v2, u0, u1, u2, i0, i1);
    edge_against_tri_edges!(v2, v0, u0, u1, u2, i0, i1);

    // finally, test if triangle 1 is totally contained in triangle 2 or vice versa
    point_in_tri!(v0, u0, u1, u2, i0, i1);
    point_in_tri!(u0, v0, v1, v2, i0, i1);

    false
}

#[wasm_bindgen(js_name = "noDivTriTriIsect")]
pub fn no_div_tri_tri_isect(
    v0: &[f32],
    v1: &[f32],
    v2: &[f32],
    u0: &[f32],
    u1: &[f32],
    u2: &[f32],
    epsilon: Option<f32>,
) -> bool {
    #[cfg(feature = "console_error_panic_hook")]
    {
        utils::set_panic_hook();
    }

    let mut e1 = [0.; 3];
    let mut e2 = [0.; 3];

    let mut n1 = [0.; 3];
    let mut n2 = [0.; 3];

    let d1;
    let d2;

    let mut du0;
    let mut du1;
    let mut du2;

    let mut dv0;
    let mut dv1;
    let mut dv2;

    let mut d = [0.; 3];

    let mut isect1 = [0.; 2];
    let mut isect2 = [0.; 2];

    let du0du1;
    let du0du2;
    let dv0dv1;
    let dv0dv2;

    let mut index;

    let vp0;
    let vp1;
    let vp2;

    let up0;
    let up1;
    let up2;

    let bb;
    let cc;
    let mut max;

    // compute plane equation of triangle(v0,v1,v2)
    sub!(e1, v1, v0);
    sub!(e2, v2, v0);
    cross!(n1, e1, e2);
    d1 = -dot!(n1, v0);
    // plane equation 1: n1.x + d1 = 0

    // put u0,u1,u2 into plane equation 1 to compute signed distances to the plane
    du0 = dot!(n1, u0) + d1;
    du1 = dot!(n1, u1) + d1;
    du2 = dot!(n1, u2) + d1;

    // coplanarity robustness check
    if let Some(epsilon) = epsilon {
        if du0.abs() < epsilon {
            du0 = 0.;
        }
        if du1.abs() < epsilon {
            du1 = 0.;
        }
        if du2.abs() < epsilon {
            du2 = 0.;
        }
    }

    du0du1 = du0 * du1;
    du0du2 = du0 * du2;

    if du0du1 > 0. && du0du2 > 0. {
        // same sign on all of them + not equal 0?
        return false; // no intersection occurs
    }

    // compute plane of triangle (u0,u1,u2)
    sub!(e1, u1, u0);
    sub!(e2, u2, u0);
    cross!(n2, e1, e2);
    d2 = -dot!(n2, u0);
    // plane equation 2: n2.x + d2 = 0

    // put v0,v1,v2 into plane equation 2
    dv0 = dot!(n2, v0) + d2;
    dv1 = dot!(n2, v1) + d2;
    dv2 = dot!(n2, v2) + d2;

    if let Some(epsilon) = epsilon {
        if dv0.abs() < epsilon {
            dv0 = 0.;
        }
        if dv1.abs() < epsilon {
            dv1 = 0.;
        }
        if dv2.abs() < epsilon {
            dv2 = 0.;
        }
    }

    dv0dv1 = dv0 * dv1;
    dv0dv2 = dv0 * dv2;

    if dv0dv1 > 0. && dv0dv2 > 0. {
        // same sign on all of them + not equal 0?
        return false; // no intersection occurs
    }

    // compute direction of intersection line
    cross!(d, n1, n2);

    // compute and index to the largest component of d
    max = d[0].abs();
    index = 0;
    bb = d[1].abs();
    cc = d[2].abs();

    if bb > max {
        max = bb;
        index = 1;
    }

    if cc > max {
        index = 2;
    }

    // this is the simplified projection onto l
    vp0 = v0[index];
    vp1 = v1[index];
    vp2 = v2[index];

    up0 = u0[index];
    up1 = u1[index];
    up2 = u2[index];

    // compute interval for triangle 1
    let a;
    let b;
    let c;
    let x0;
    let x1;

    compute_intervals!(
        vp0, vp1, vp2, dv0, dv1, dv2, dv0dv1, dv0dv2, a, b, c, x0, x1, n1, v0, v1, v2, u0, u1, u2
    );

    // compute interval for triangle 2
    let d;
    let e;
    let f;
    let y0;
    let y1;

    compute_intervals!(
        up0, up1, up2, du0, du1, du2, du0du1, du0du2, d, e, f, y0, y1, n1, v0, v1, v2, u0, u1, u2
    );

    let xx;
    let yy;
    let xxyy;
    let mut tmp;

    xx = x0 * x1;
    yy = y0 * y1;
    xxyy = xx * yy;

    tmp = a * xxyy;
    isect1[0] = tmp + b * x1 * yy;
    isect1[1] = tmp + c * x0 * yy;

    tmp = d * xxyy;
    isect2[0] = tmp + e * xx * y1;
    isect2[1] = tmp + f * xx * y0;

    isect1.swap(0, 1);
    isect2.swap(0, 1);

    if isect1[1] < isect2[0] || isect2[1] < isect1[0] {
        return false;
    }

    true
}

#[wasm_bindgen(js_name = "meshMeshIsect")]
pub fn mesh_mesh_isect(m1: &[f32], m2: &[f32], epsilon: Option<f32>) -> bool {
    #[cfg(feature = "console_error_panic_hook")]
    {
        utils::set_panic_hook();
    }

    let epsilon = epsilon.unwrap_or(0.000_001);

    for tri1 in m1.chunks(9) {
        for tri2 in m2.chunks(9) {
            if no_div_tri_tri_isect(
                &[tri1[0], tri1[1], tri1[2]],
                &[tri1[3], tri1[4], tri1[5]],
                &[tri1[6], tri1[7], tri1[8]],
                &[tri2[0], tri2[1], tri2[2]],
                &[tri2[3], tri2[4], tri2[5]],
                &[tri2[6], tri2[7], tri2[8]],
                Some(epsilon),
            ) {
                return true;
            }
        }
    }

    false
}
