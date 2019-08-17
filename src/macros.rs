macro_rules! cross {
    ($dest:ident, $v1:ident, $v2:ident) => {
        $dest[0] = $v1[1] * $v2[2] - $v1[2] * $v2[1];
        $dest[1] = $v1[2] * $v2[0] - $v1[0] * $v2[2];
        $dest[2] = $v1[0] * $v2[1] - $v1[1] * $v2[0];
    };
}

macro_rules! dot {
    ($v1:ident, $v2:ident) => {
        ($v1[0] * $v2[0] + $v1[1] * $v2[1] + $v1[2] * $v2[2])
    };
}

macro_rules! sub {
    ($dest:ident, $v1:ident, $v2:ident) => {
        $dest[0] = $v1[0] - $v2[0];
        $dest[1] = $v1[1] - $v2[1];
        $dest[2] = $v1[2] - $v2[2];
    };
}

macro_rules! edge_against_tri_edges {
    ($v0:ident, $v1:ident, $u0:ident, $u1:ident, $u2:ident, $i0:ident, $i1:ident) => {
        let ax = $v1[$i0] - $v0[$i0];
        let ay = $v1[$i1] - $v0[$i1];

        let mut bx;
        let mut by;

        let mut cx;
        let mut cy;

        let mut e;
        let mut d;
        let mut f;

        // this edge to edge test is based on Franlin Antonio's gem:
        // "Faster Line Segment Intersection", in Graphics Gems III,
        // pp. 199-202
        macro_rules! edge_edge_test {
            ($u_0:ident, $u_1:ident) => {
                bx = $u_0[$i0] - $u_1[$i0];
                by = $u_0[$i1] - $u_1[$i1];
                cx = $v0[$i0] - $u_0[$i0];
                cy = $v0[$i1] - $u_0[$i1];
                f = ay * bx - ax * by;
                d = by * cx - bx * cy;

                if (f > 0. && d >= 0. && d <= f) || (f < 0. && d <= 0. && d >= f) {
                    e = ax * cy - ay * cx;
                    if f > 0. {
                        if e >= 0. && e <= f {
                            return true;
                        }
                    } else if e <= 0. && e >= f {
                        return true;
                    }
                }
            };
        }

        // test edge u0,u1 against v0,v1
        edge_edge_test!($u0, $u1);
        // test edge u1,u2 against v0,v1
        edge_edge_test!($u1, $u2);
        // test edge u2,u1 against v0,v1
        edge_edge_test!($u2, $u0);
    };
}

macro_rules! point_in_tri {
    ($v0:ident, $u0:ident, $u1:ident, $u2:ident, $i0:ident, $i1:ident) => {
        let mut a;
        let mut b;
        let mut c;

        // is triangle 1 completely inside triangle 2?
        // check if v0 is inside tri(u0, u1, u2)
        a = $u1[$i1] - $u0[$i1];
        b = -($u1[$i0] - $u0[$i0]);
        c = -a * $u0[$i0] - b * $u0[$i1];
        let d0 = a * $v0[$i0] + b * $v0[$i1] + c;

        a = $u2[$i1] - $u1[$i1];
        b = -($u2[$i0] - $u1[$i0]);
        c = -a * $u1[$i0] - b * $u1[$i1];
        let d1 = a * $v0[$i0] + b * $v0[$i1] + c;

        a = $u0[$i1] - $u2[$i1];
        b = -($u0[$i0] - $u2[$i0]);
        c = -a * $u2[$i0] - b * $u2[$i1];
        let d2 = a * $v0[$i0] + b * $v0[$i1] + c;

        if d0 * d1 > 0. && d0 * d2 > 0. {
            return true;
        }
    };
}

macro_rules! compute_intervals {
    ($vv0:ident, $vv1:ident, $vv2:ident, $d0:ident, $d1:ident, $d2:ident, $d0d1:ident, $d0d2:ident, $a:ident, $b:ident, $c:ident, $x0:ident, $x1:ident, $n1:ident, $v0:ident, $v1:ident, $v2:ident, $u0:ident, $u1:ident, $u2:ident) => {
        if $d0d1 > 0. {
            // here we know that d0d2 <= 0.0
            // that is d0, d1 are on the same side, d2 on the other or on the plane
            $a = $vv2;
            $b = ($vv0 - $vv2) * $d2;
            $c = ($vv1 - $vv2) * $d2;
            $x0 = $d2 - $d0;
            $x1 = $d2 - $d1;
        } else if $d0d2 > 0. {
            // here we know that d0d1 <= 0.0
            $a = $vv1;
            $b = ($vv0 - $vv1) * $d1;
            $c = ($vv2 - $vv1) * $d1;
            $x0 = $d1 - $d0;
            $x1 = $d1 - $d2;
        } else if $d1 * $d2 > 0. || $d0 != 0. {
            // here we know that d0d1 <= 0.0 or that d0 != 0.0
            $a = $vv0;
            $b = ($vv1 - $vv0) * $d0;
            $c = ($vv2 - $vv0) * $d0;
            $x0 = $d0 - $d1;
            $x1 = $d0 - $d2;
        } else if $d1 != 0. {
            $a = $vv1;
            $b = ($vv0 - $vv1) * $d1;
            $c = ($vv2 - $vv1) * $d1;
            $x0 = $d1 - $d0;
            $x1 = $d1 - $d2;
        } else if $d2 != 0. {
            $a = $vv2;
            $b = ($vv0 - $vv2) * $d2;
            $c = ($vv1 - $vv2) * $d2;
            $x0 = $d2 - $d0;
            $x1 = $d2 - $d1;
        } else {
            // triangles are coplanar
            return coplanar_tri_tri($n1, $v0, $v1, $v2, $u0, $u1, $u2);
        }
    };
}

macro_rules! check_coplanarity_robustness {
    ($epsilon:ident, $v0:ident, $v1:ident, $v2:ident) => {
        if let Some(epsilon) = $epsilon {
            if $v0.abs() < epsilon {
                $v0 = 0.;
            }
            if $v1.abs() < epsilon {
                $v1 = 0.;
            }
            if $v2.abs() < epsilon {
                $v2 = 0.;
            }
        }
    };
}
