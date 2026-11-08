//! Linear-phase 2x resampling, so the loop can run at 88.2 kHz or more.


const TAPS: usize = 31;

/// Latency of a full up-and-down trip, in host samples.
pub const ROUND_TRIP_LATENCY: usize = (TAPS - 1) / 2;

fn bessel_i0(x: f64) -> f64 {
    let (mut sum, mut term) = (1.0, 1.0);
    for k in 1..32 {
        term *= (x / (2.0 * k as f64)).powi(2);
        sum += term;
    }
    sum
}

/// A Kaiser-windowed halfband lowpass.
fn halfband() -> [f32; TAPS] {
    const BETA: f64 = 8.0;
    let centre = (TAPS - 1) as f64 / 2.0;
    std::array::from_fn(|n| {
        let t = n as f64 - centre;
        let sinc = if t == 0.0 { 1.0 } else { (std::f64::consts::PI * t / 2.0).sin() / (std::f64::consts::PI * t / 2.0) };
        let window = bessel_i0(BETA * (1.0 - (t / centre).powi(2)).sqrt()) / bessel_i0(BETA);
        (0.5 * sinc * window) as f32
    })
}

struct Fir {
    taps: [f32; TAPS],
    history: [f32; TAPS],
    head: usize,
}

impl Fir {
    fn new() -> Self {
        Self { taps: halfband(), history: [0.0; TAPS], head: 0 }
    }

    #[inline(always)]
    fn process(&mut self, x: f32) -> f32 {
        self.head = (self.head + TAPS - 1) % TAPS;
        self.history[self.head] = x;
        let (newer, older) = self.history.split_at(self.head);
        older.iter().chain(newer).zip(&self.taps).map(|(h, t)| h * t).sum()
    }
}

/// Doubles and halves the rate of one channel.
pub struct Oversampler {
    up: Fir,
    down: Fir,
}

impl Oversampler {
    pub fn new() -> Self {
        Self { up: Fir::new(), down: Fir::new() }
    }

    #[inline(always)]
    pub fn up(&mut self, x: f32) -> [f32; 2] {
        [self.up.process(2.0 * x), self.up.process(0.0)]
    }

    #[inline(always)]
    pub fn down(&mut self, x: [f32; 2]) -> f32 {
        self.down.process(x[0]);
        self.down.process(x[1])
    }
}

