//! SVG path data (the mockup's glyphs, emblems and beasts are written that way) into
//! femtovg paths. Handles M L H V C S Q T A Z, absolute and relative.

use std::f32::consts::PI;

use nih_plug_vizia::vizia::vg::Path;

fn numbers(s: &str) -> Vec<(char, Vec<f32>)> {
    let mut out: Vec<(char, Vec<f32>)> = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_alphabetic() && c != 'e' && c != 'E' {
            out.push((c, Vec::new()));
            i += 1;
        } else if c == '-' || c == '.' || c.is_ascii_digit() {
            let start = i;
            let mut seen_dot = c == '.';
            i += 1;
            while i < bytes.len() {
                let d = bytes[i] as char;
                if d.is_ascii_digit() {
                    i += 1;
                } else if d == '.' && !seen_dot {
                    seen_dot = true;
                    i += 1;
                } else if (d == 'e' || d == 'E') && i + 1 < bytes.len() {
                    i += 1;
                    if bytes[i] == b'-' || bytes[i] == b'+' {
                        i += 1;
                    }
                } else {
                    break;
                }
            }
            if let (Some(last), Ok(v)) = (out.last_mut(), s[start..i].parse::<f32>()) {
                last.1.push(v);
            }
        } else {
            i += 1;
        }
    }
    out
}

/// An elliptical arc (SVG's endpoint form) as cubic Béziers.
#[allow(clippy::too_many_arguments)]
fn arc(p: &mut Path, x0: f32, y0: f32, mut rx: f32, mut ry: f32, rot: f32, large: bool, sweep: bool, x: f32, y: f32) {
    if rx == 0.0 || ry == 0.0 || (x0 == x && y0 == y) {
        p.line_to(x, y);
        return;
    }
    rx = rx.abs();
    ry = ry.abs();
    let phi = rot * PI / 180.0;
    let (sp, cp) = phi.sin_cos();
    let (dx, dy) = ((x0 - x) / 2.0, (y0 - y) / 2.0);
    let x1 = cp * dx + sp * dy;
    let y1 = -sp * dx + cp * dy;
    let lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }
    let num = (rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1).max(0.0);
    let den = rx * rx * y1 * y1 + ry * ry * x1 * x1;
    let mut co = (num / den).sqrt();
    if large == sweep {
        co = -co;
    }
    let cx1 = co * rx * y1 / ry;
    let cy1 = -co * ry * x1 / rx;
    let cx = cp * cx1 - sp * cy1 + (x0 + x) / 2.0;
    let cy = sp * cx1 + cp * cy1 + (y0 + y) / 2.0;
    let angle = |ux: f32, uy: f32, vx: f32, vy: f32| (ux * vy - uy * vx).atan2(ux * vx + uy * vy);
    let t1 = angle(1.0, 0.0, (x1 - cx1) / rx, (y1 - cy1) / ry);
    let mut dt = angle((x1 - cx1) / rx, (y1 - cy1) / ry, (-x1 - cx1) / rx, (-y1 - cy1) / ry);
    if !sweep && dt > 0.0 {
        dt -= 2.0 * PI;
    } else if sweep && dt < 0.0 {
        dt += 2.0 * PI;
    }
    let segments = (dt.abs() / (PI / 2.0)).ceil().max(1.0) as usize;
    let delta = dt / segments as f32;
    let k = 4.0 / 3.0 * (delta / 4.0).tan();
    let point = |t: f32| {
        let (s, c) = t.sin_cos();
        (cx + rx * c * cp - ry * s * sp, cy + rx * c * sp + ry * s * cp)
    };
    let deriv = |t: f32| {
        let (s, c) = t.sin_cos();
        (-rx * s * cp - ry * c * sp, -rx * s * sp + ry * c * cp)
    };
    let mut t = t1;
    for _ in 0..segments {
        let (ax, ay) = point(t);
        let (dax, day) = deriv(t);
        let t2 = t + delta;
        let (bx, by) = point(t2);
        let (dbx, dby) = deriv(t2);
        p.bezier_to(ax + k * dax, ay + k * day, bx - k * dbx, by - k * dby, bx, by);
        t = t2;
    }
}

/// Parse path data `d`, scaled by `s` and moved by (`ox`, `oy`).
pub fn path(d: &str, ox: f32, oy: f32, s: f32) -> Path {
    let mut p = Path::new();
    let (mut x, mut y) = (0.0f32, 0.0f32);
    let (mut sx, mut sy) = (0.0f32, 0.0f32);
    // The last control point, for S and T.
    let (mut cx, mut cy) = (0.0f32, 0.0f32);
    let mut last = ' ';
    let t = |v: f32, o: f32| o + s * v;
    for (cmd, args) in numbers(d) {
        let rel = cmd.is_ascii_lowercase();
        let up = cmd.to_ascii_uppercase();
        let mut a = args.as_slice();
        let take = |a: &mut &[f32], n: usize| -> Option<Vec<f32>> {
            if a.len() >= n {
                let v = a[..n].to_vec();
                *a = &a[n..];
                Some(v)
            } else {
                None
            }
        };
        match up {
            'Z' => {
                p.close();
                (x, y) = (sx, sy);
            }
            'M' => {
                let mut first = true;
                while let Some(v) = take(&mut a, 2) {
                    let (nx, ny) = if rel { (x + v[0], y + v[1]) } else { (v[0], v[1]) };
                    if first {
                        p.move_to(t(nx, ox), t(ny, oy));
                        (sx, sy) = (nx, ny);
                        first = false;
                    } else {
                        p.line_to(t(nx, ox), t(ny, oy));
                    }
                    (x, y) = (nx, ny);
                }
            }
            'L' => {
                while let Some(v) = take(&mut a, 2) {
                    let (nx, ny) = if rel { (x + v[0], y + v[1]) } else { (v[0], v[1]) };
                    p.line_to(t(nx, ox), t(ny, oy));
                    (x, y) = (nx, ny);
                }
            }
            'H' => {
                while let Some(v) = take(&mut a, 1) {
                    x = if rel { x + v[0] } else { v[0] };
                    p.line_to(t(x, ox), t(y, oy));
                }
            }
            'V' => {
                while let Some(v) = take(&mut a, 1) {
                    y = if rel { y + v[0] } else { v[0] };
                    p.line_to(t(x, ox), t(y, oy));
                }
            }
            'C' => {
                while let Some(v) = take(&mut a, 6) {
                    let (bx, by) = if rel { (x, y) } else { (0.0, 0.0) };
                    let (c1x, c1y, c2x, c2y, nx, ny) = (bx + v[0], by + v[1], bx + v[2], by + v[3], bx + v[4], by + v[5]);
                    p.bezier_to(t(c1x, ox), t(c1y, oy), t(c2x, ox), t(c2y, oy), t(nx, ox), t(ny, oy));
                    (cx, cy, x, y) = (c2x, c2y, nx, ny);
                }
            }
            'S' => {
                while let Some(v) = take(&mut a, 4) {
                    let (bx, by) = if rel { (x, y) } else { (0.0, 0.0) };
                    let (c1x, c1y) = if matches!(last, 'C' | 'S') { (2.0 * x - cx, 2.0 * y - cy) } else { (x, y) };
                    let (c2x, c2y, nx, ny) = (bx + v[0], by + v[1], bx + v[2], by + v[3]);
                    p.bezier_to(t(c1x, ox), t(c1y, oy), t(c2x, ox), t(c2y, oy), t(nx, ox), t(ny, oy));
                    (cx, cy, x, y) = (c2x, c2y, nx, ny);
                    last = 'S';
                }
            }
            'Q' => {
                while let Some(v) = take(&mut a, 4) {
                    let (bx, by) = if rel { (x, y) } else { (0.0, 0.0) };
                    let (qx, qy, nx, ny) = (bx + v[0], by + v[1], bx + v[2], by + v[3]);
                    p.quad_to(t(qx, ox), t(qy, oy), t(nx, ox), t(ny, oy));
                    (cx, cy, x, y) = (qx, qy, nx, ny);
                    last = 'Q';
                }
            }
            'T' => {
                while let Some(v) = take(&mut a, 2) {
                    let (bx, by) = if rel { (x, y) } else { (0.0, 0.0) };
                    let (qx, qy) = if matches!(last, 'Q' | 'T') { (2.0 * x - cx, 2.0 * y - cy) } else { (x, y) };
                    let (nx, ny) = (bx + v[0], by + v[1]);
                    p.quad_to(t(qx, ox), t(qy, oy), t(nx, ox), t(ny, oy));
                    (cx, cy, x, y) = (qx, qy, nx, ny);
                    last = 'T';
                }
            }
            'A' => {
                while let Some(v) = take(&mut a, 7) {
                    let (bx, by) = if rel { (x, y) } else { (0.0, 0.0) };
                    let (nx, ny) = (bx + v[5], by + v[6]);
                    arc_scaled(&mut p, (x, y), (v[0], v[1], v[2], v[3] != 0.0, v[4] != 0.0), (nx, ny), ox, oy, s);
                    (x, y) = (nx, ny);
                }
            }
            _ => {}
        }
        if !matches!(up, 'S' | 'Q' | 'T') {
            last = up;
        }
    }
    p
}

fn arc_scaled(p: &mut Path, from: (f32, f32), (rx, ry, rot, large, sweep): (f32, f32, f32, bool, bool), to: (f32, f32),
              ox: f32, oy: f32, s: f32) {
    arc(p, ox + s * from.0, oy + s * from.1, rx * s, ry * s, rot, large, sweep, ox + s * to.0, oy + s * to.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens() {
        let n = numbers("M-6 -5 Q1 -11 8 -5 L1.6-6 a1 1 0 0 1 2 0 h.1 Z");
        assert_eq!(n[0], ('M', vec![-6.0, -5.0]));
        assert_eq!(n[2], ('L', vec![1.6, -6.0]));
        assert_eq!(n[3], ('a', vec![1.0, 1.0, 0.0, 0.0, 1.0, 2.0, 0.0]));
        assert_eq!(n[4], ('h', vec![0.1]));
        assert_eq!(n[5].0, 'Z');
        let _ = path("M0 0 a1 1 0 0 1 2 0 a2 2 0 0 1 -4 0 C1 2 3 4 5 6 S7 8 9 10 Q1 1 2 2 T3 3 Z", 0.0, 0.0, 1.0);
    }
}
