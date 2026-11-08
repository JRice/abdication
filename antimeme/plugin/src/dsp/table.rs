//! Curves measured at a handful of knob positions.

/// (x, y) points, x rising. Between them, interpolate; beyond them, hold.
pub type Curve = [(f32, f32)];

#[inline]
pub fn lookup(curve: &Curve, x: f32) -> f32 {
    interpolate(curve, x, |y| y, |y| y)
}

/// As `lookup`, but straight in log(y): for delays and frequencies.
#[inline]
pub fn lookup_log(curve: &Curve, x: f32) -> f32 {
    interpolate(curve, x, f32::ln, f32::exp)
}

/// The x where a falling curve reaches `y`, in log(y): the inverse of `lookup_log`.
pub fn inverse_log(curve: &Curve, y: f32) -> f32 {
    let target = y.ln();
    for pair in curve.windows(2) {
        let (x0, y0) = (pair[0].0, pair[0].1.ln());
        let (x1, y1) = (pair[1].0, pair[1].1.ln());
        if target >= y1 {
            return x0 + (x1 - x0) * ((target - y0) / (y1 - y0)).clamp(0.0, 1.0);
        }
    }
    curve[curve.len() - 1].0
}

#[inline(always)]
fn interpolate(curve: &Curve, x: f32, into: impl Fn(f32) -> f32, back: impl Fn(f32) -> f32) -> f32 {
    if x <= curve[0].0 {
        return curve[0].1;
    }
    for pair in curve.windows(2) {
        if x <= pair[1].0 {
            let t = (x - pair[0].0) / (pair[1].0 - pair[0].0);
            return back(into(pair[0].1) + (into(pair[1].1) - into(pair[0].1)) * t);
        }
    }
    curve[curve.len() - 1].1
}
