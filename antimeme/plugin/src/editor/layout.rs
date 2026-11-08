//! Where everything goes, what it controls, and how its value reads in Tibetan.

use nih_plug::prelude::*;

use crate::dsp::agitation::speed_hz;
use crate::params::AntimemeParams;

pub const WIDTH: f32 = 960.0;
pub const HEIGHT: f32 = 640.0;

pub const MARGIN: f32 = 24.0;
pub const WORDMARK: &str = "ཨེན་ཊི་མིམ";

/// The Time label row, under the numerals.
pub const TIME_ROW_Y: f32 = 327.0;
pub const TIME_LABEL: &str = "དུས་ཚོད";
pub const TIME_UNIT: &str = "སྐར་ཆའི་སྟོང་ཆ";

/// The numerals: size and baseline fill this band with the digits' real height.
pub const NUMERALS: Rect = Rect { x: 0.0, y: 40.0, w: WIDTH, h: 272.0 };
pub const NUMERAL_SIZE: f32 = 389.0;
pub const NUMERAL_BASELINE: f32 = 317.0;
pub const NUMERAL_WIDTH: f32 = 912.0;

pub const AGITATION_LABEL: &str = "དཀྲུག་པ";
pub const AGITATION_LABEL_Y: f32 = 364.0;
pub const AGITATION_TRACE: Rect = Rect { x: MARGIN, y: 380.0, w: 306.0, h: 70.0 };
pub const INTERFERENCE_LABEL: &str = "བར་ཆད";
pub const INTERFERENCE_LABEL_Y: f32 = 472.0;
pub const INTERFERENCE_TRACE: Rect = Rect { x: MARGIN, y: 488.0, w: 306.0, h: 76.0 };

const GRID_X: f32 = 352.0;
const GRID_Y: f32 = 350.0;
const COLUMN: f32 = 284.0;
const COLUMN_GAP: f32 = 16.0;
pub const ROW_H: f32 = 52.0;
const ROWS_PER_COLUMN: usize = 5;

/// Index of the Time row among the panel's sliders; the grid comes before it.
pub const TIME: usize = 10;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }

    pub fn centre_y(&self) -> f32 {
        self.y + self.h / 2.0
    }
}

/// Which part of a slider: its value, or one of its two modulation sends.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Zone {
    Value,
    Agitation,
    Interference,
}

#[derive(Clone, Copy)]
enum Format {
    Percent,
    Level,
    Speed,
}

pub struct Row {
    pub name: &'static str,
    pub seed: u32,
    pub top: f32,
    pub left: f32,
    pub slider: Rect,
    pub value: ParamPtr,
    pub agitation: Option<ParamPtr>,
    pub interference: Option<ParamPtr>,
    format: Format,
}

impl Row {
    pub fn glyph_x(&self) -> f32 {
        self.left
    }

    pub fn label_x(&self) -> f32 {
        self.left + 26.0
    }

    pub fn value_x(&self) -> f32 {
        self.left + COLUMN
    }

    pub fn centre_y(&self) -> f32 {
        self.top + ROW_H / 2.0
    }

    /// What a press at this height grabs. Sends take the top and bottom of the slider.
    pub fn zone_at(&self, y: f32) -> Zone {
        let offset = y - self.slider.centre_y();
        if offset < -5.0 && self.agitation.is_some() {
            Zone::Agitation
        } else if offset > 5.0 && self.interference.is_some() {
            Zone::Interference
        } else {
            Zone::Value
        }
    }

    pub fn param(&self, zone: Zone) -> ParamPtr {
        match zone {
            Zone::Value => self.value,
            Zone::Agitation => self.agitation.unwrap_or(self.value),
            Zone::Interference => self.interference.unwrap_or(self.value),
        }
    }

    pub fn normalized(&self, zone: Zone) -> f32 {
        unsafe { self.param(zone).unmodulated_normalized_value() }
    }

    /// A send's amount, -1..1.
    pub fn send(&self, zone: Zone) -> f32 {
        unsafe { self.param(zone).unmodulated_plain_value() }
    }

    /// The value's readout, one or two lines of Tibetan.
    pub fn readout(&self) -> Vec<String> {
        let plain = unsafe { self.value.unmodulated_plain_value() };
        match self.format {
            Format::Percent => vec![percent(plain)],
            Format::Level => vec![format!("{} ཌི་བི", tibetan_digits(&format!("{:.1}", util::gain_to_db(plain))))],
            Format::Speed => vec!["སྐར་ཆར་ཐེངས་".to_string(), tibetan_digits(&format!("{:.2}", speed_hz(plain)))],
        }
    }

    /// A send's readout, signed.
    pub fn send_readout(&self, zone: Zone) -> String {
        tibetan_digits(&format!("{:+.0}", self.send(zone) * 100.0))
    }
}

pub fn percent(value: f32) -> String {
    format!("བརྒྱ་ཆ་{}", tibetan_digits(&format!("{:.0}", value * 100.0)))
}

/// Western digits to Tibetan. Signs and the decimal point pass through.
pub fn tibetan_digits(text: &str) -> String {
    text.chars()
        .map(|c| c.to_digit(10).and_then(|d| char::from_u32(0x0F20 + d)).unwrap_or(c))
        .collect()
}

/// The panel's sliders: the grid in reading order (down each column), then Time.
pub fn rows(p: &AntimemeParams) -> Vec<Row> {
    let cell = |index: usize| {
        let column = (index / ROWS_PER_COLUMN) as f32;
        let row = (index % ROWS_PER_COLUMN) as f32;
        let left = GRID_X + column * (COLUMN + COLUMN_GAP);
        let top = GRID_Y + row * ROW_H;
        (left, top, Rect { x: left + 96.0, y: top + 4.0, w: 102.0, h: 44.0 })
    };
    let row = |index: usize, name, seed, value, agitation, interference, format| {
        let (left, top, slider) = cell(index);
        Row { name, seed, top, left, slider, value, agitation, interference, format }
    };
    use Format::*;
    vec![
        row(0, "ཉམས་འགྲིབ", 21, p.decay.as_ptr(), Some(p.agitation_to_decay.as_ptr()), Some(p.interference_to_decay.as_ptr()), Percent),
        row(1, "ཚགས་མ", 22, p.filter.as_ptr(), Some(p.agitation_to_filter.as_ptr()), Some(p.interference_to_filter.as_ptr()), Percent),
        row(2, "འཇིབ་འཐེན", 23, p.absorb.as_ptr(), Some(p.agitation_to_absorb.as_ptr()), Some(p.interference_to_absorb.as_ptr()), Percent),
        row(3, "སྒྲ་རྣོ", 24, p.whine.as_ptr(), Some(p.agitation_to_whine.as_ptr()), Some(p.interference_to_whine.as_ptr()), Percent),
        row(4, "ཤུགས་ཚད", 25, p.strength.as_ptr(), None, None, Percent),
        row(5, "བསྲེ་བ", 26, p.blend.as_ptr(), None, None, Percent),
        row(6, "རྒྱ་བསྐྱེད", 27, p.spread.as_ptr(), None, None, Percent),
        row(7, "སྒྲ་ཚད", 28, p.level.as_ptr(), None, None, Level),
        row(8, "མྱུར་ཚད", 29, p.agitation_speed.as_ptr(), None, None, Speed),
        row(9, "ཟུར་ཚད", 30, p.agitation_angle.as_ptr(), None, None, Percent),
        Row {
            name: TIME_LABEL,
            seed: 0,
            top: TIME_ROW_Y - ROW_H / 2.0,
            left: MARGIN,
            slider: Rect { x: 120.0, y: TIME_ROW_Y - 18.0, w: 690.0, h: 36.0 },
            value: p.time.as_ptr(),
            agitation: Some(p.agitation_to_time.as_ptr()),
            interference: Some(p.interference_to_time.as_ptr()),
            format: Percent,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_become_tibetan() {
        assert_eq!(tibetan_digits("1950"), "༡༩༥༠");
        assert_eq!(tibetan_digits("-0.26"), "-༠.༢༦");
    }

    #[test]
    fn every_grid_row_fits_the_window() {
        let p = AntimemeParams::default();
        for row in rows(&p) {
            assert!(row.top >= 0.0 && row.top + ROW_H <= HEIGHT, "{}", row.name);
            assert!(row.slider.x + row.slider.w <= WIDTH, "{}", row.name);
        }
    }
}
