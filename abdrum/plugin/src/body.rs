/* ------------------------------------------------------------
name: "ABDrumBody"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn BodyDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn BodyDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct BodyDsp {
	fButton0: F32,
	fVec0: [F32;2],
	fSampleRate: i32,
	fConst1: F32,
	fHslider0: F32,
	fHslider1: F32,
	fHslider2: F32,
	fConst2: F32,
	fRec1: [F32;2],
	fHslider3: F32,
	fRec2: [F32;2],
	fConst3: F32,
	fHslider4: F32,
	fHslider5: F32,
	fConst4: F32,
	fRec3: [F32;2],
	fRec4: [F32;2],
	fRec0: [F32;3],
	fRec6: [F32;2],
	fRec7: [F32;2],
	fRec8: [F32;2],
	fRec5: [F32;3],
	fRec10: [F32;2],
	fRec11: [F32;2],
	fRec12: [F32;2],
	fRec9: [F32;3],
	fRec14: [F32;2],
	fRec15: [F32;2],
	fRec16: [F32;2],
	fRec13: [F32;3],
	fRec18: [F32;2],
	fRec19: [F32;2],
	fRec20: [F32;2],
	fRec17: [F32;3],
	fRec22: [F32;2],
	fRec23: [F32;2],
	fRec24: [F32;2],
	fRec21: [F32;3],
	fRec26: [F32;2],
	fRec27: [F32;2],
	fRec28: [F32;2],
	fRec25: [F32;3],
	fRec30: [F32;2],
	fRec31: [F32;2],
	fRec32: [F32;2],
	fRec29: [F32;3],
	fRec34: [F32;2],
	fRec35: [F32;2],
	fRec36: [F32;2],
	fRec33: [F32;3],
	fRec38: [F32;2],
	fRec39: [F32;2],
	fRec40: [F32;2],
	fRec37: [F32;3],
	fRec42: [F32;2],
	fRec43: [F32;2],
	fRec44: [F32;2],
	fRec41: [F32;3],
	fRec46: [F32;2],
	fRec47: [F32;2],
	fRec48: [F32;2],
	fRec45: [F32;3],
	fRec50: [F32;2],
	fRec51: [F32;2],
	fRec52: [F32;2],
	fRec49: [F32;3],
	fRec54: [F32;2],
	fRec55: [F32;2],
	fRec56: [F32;2],
	fRec53: [F32;3],
	fRec58: [F32;2],
	fRec59: [F32;2],
	fRec60: [F32;2],
	fRec57: [F32;3],
	fRec62: [F32;2],
	fRec63: [F32;2],
	fRec64: [F32;2],
	fRec61: [F32;3],
	fRec66: [F32;2],
	fRec67: [F32;2],
	fRec68: [F32;2],
	fRec65: [F32;3],
	fRec70: [F32;2],
	fRec71: [F32;2],
	fRec72: [F32;2],
	fRec69: [F32;3],
	fRec74: [F32;2],
	fRec75: [F32;2],
	fRec76: [F32;2],
	fRec73: [F32;3],
	fRec78: [F32;2],
	fRec79: [F32;2],
	fRec80: [F32;2],
	fRec77: [F32;3],
	fRec82: [F32;2],
	fRec83: [F32;2],
	fRec84: [F32;2],
	fRec81: [F32;3],
	fRec86: [F32;2],
	fRec87: [F32;2],
	fRec88: [F32;2],
	fRec85: [F32;3],
	fRec90: [F32;2],
	fRec91: [F32;2],
	fRec92: [F32;2],
	fRec89: [F32;3],
	fRec94: [F32;2],
	fRec95: [F32;2],
	fRec96: [F32;2],
	fRec93: [F32;3],
	fVec1: [F32;2],
	fConst5: F32,
	fConst6: F32,
	fConst7: F32,
	fRec97: [F32;2],
}

impl FaustDsp for BodyDsp {
	type T = F32;
		
	fn new() -> BodyDsp { 
		BodyDsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			fSampleRate: 0,
			fConst1: 0.0,
			fHslider0: 0.0,
			fHslider1: 0.0,
			fHslider2: 0.0,
			fConst2: 0.0,
			fRec1: [0.0;2],
			fHslider3: 0.0,
			fRec2: [0.0;2],
			fConst3: 0.0,
			fHslider4: 0.0,
			fHslider5: 0.0,
			fConst4: 0.0,
			fRec3: [0.0;2],
			fRec4: [0.0;2],
			fRec0: [0.0;3],
			fRec6: [0.0;2],
			fRec7: [0.0;2],
			fRec8: [0.0;2],
			fRec5: [0.0;3],
			fRec10: [0.0;2],
			fRec11: [0.0;2],
			fRec12: [0.0;2],
			fRec9: [0.0;3],
			fRec14: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;2],
			fRec13: [0.0;3],
			fRec18: [0.0;2],
			fRec19: [0.0;2],
			fRec20: [0.0;2],
			fRec17: [0.0;3],
			fRec22: [0.0;2],
			fRec23: [0.0;2],
			fRec24: [0.0;2],
			fRec21: [0.0;3],
			fRec26: [0.0;2],
			fRec27: [0.0;2],
			fRec28: [0.0;2],
			fRec25: [0.0;3],
			fRec30: [0.0;2],
			fRec31: [0.0;2],
			fRec32: [0.0;2],
			fRec29: [0.0;3],
			fRec34: [0.0;2],
			fRec35: [0.0;2],
			fRec36: [0.0;2],
			fRec33: [0.0;3],
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fRec40: [0.0;2],
			fRec37: [0.0;3],
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fRec44: [0.0;2],
			fRec41: [0.0;3],
			fRec46: [0.0;2],
			fRec47: [0.0;2],
			fRec48: [0.0;2],
			fRec45: [0.0;3],
			fRec50: [0.0;2],
			fRec51: [0.0;2],
			fRec52: [0.0;2],
			fRec49: [0.0;3],
			fRec54: [0.0;2],
			fRec55: [0.0;2],
			fRec56: [0.0;2],
			fRec53: [0.0;3],
			fRec58: [0.0;2],
			fRec59: [0.0;2],
			fRec60: [0.0;2],
			fRec57: [0.0;3],
			fRec62: [0.0;2],
			fRec63: [0.0;2],
			fRec64: [0.0;2],
			fRec61: [0.0;3],
			fRec66: [0.0;2],
			fRec67: [0.0;2],
			fRec68: [0.0;2],
			fRec65: [0.0;3],
			fRec70: [0.0;2],
			fRec71: [0.0;2],
			fRec72: [0.0;2],
			fRec69: [0.0;3],
			fRec74: [0.0;2],
			fRec75: [0.0;2],
			fRec76: [0.0;2],
			fRec73: [0.0;3],
			fRec78: [0.0;2],
			fRec79: [0.0;2],
			fRec80: [0.0;2],
			fRec77: [0.0;3],
			fRec82: [0.0;2],
			fRec83: [0.0;2],
			fRec84: [0.0;2],
			fRec81: [0.0;3],
			fRec86: [0.0;2],
			fRec87: [0.0;2],
			fRec88: [0.0;2],
			fRec85: [0.0;3],
			fRec90: [0.0;2],
			fRec91: [0.0;2],
			fRec92: [0.0;2],
			fRec89: [0.0;3],
			fRec94: [0.0;2],
			fRec95: [0.0;2],
			fRec96: [0.0;2],
			fRec93: [0.0;3],
			fVec1: [0.0;2],
			fConst5: 0.0,
			fConst6: 0.0,
			fConst7: 0.0,
			fRec97: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn BodyDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("drum.lib/name", r"abdrum-layers");
		m.declare("drum.lib/version", r"0.1");
		m.declare("filename", r"plugin-body.dsp");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass0_highpass1", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"ABDrumBody");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.5.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 2;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.4;
		self.fHslider2 = 0.4;
		self.fHslider3 = 0.0;
		self.fHslider4 = 1e+02;
		self.fHslider5 = 0.3;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec2[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec3[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec4[l4 as usize] = 0.0;
		}
		for l5 in 0..3 {
			self.fRec0[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec6[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec7[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec8[l8 as usize] = 0.0;
		}
		for l9 in 0..3 {
			self.fRec5[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec10[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec11[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec12[l12 as usize] = 0.0;
		}
		for l13 in 0..3 {
			self.fRec9[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec14[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec15[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec16[l16 as usize] = 0.0;
		}
		for l17 in 0..3 {
			self.fRec13[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec18[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec19[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec20[l20 as usize] = 0.0;
		}
		for l21 in 0..3 {
			self.fRec17[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec22[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec23[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec24[l24 as usize] = 0.0;
		}
		for l25 in 0..3 {
			self.fRec21[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec26[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec27[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec28[l28 as usize] = 0.0;
		}
		for l29 in 0..3 {
			self.fRec25[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec30[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec31[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec32[l32 as usize] = 0.0;
		}
		for l33 in 0..3 {
			self.fRec29[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec34[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec35[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec36[l36 as usize] = 0.0;
		}
		for l37 in 0..3 {
			self.fRec33[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec38[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec39[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec40[l40 as usize] = 0.0;
		}
		for l41 in 0..3 {
			self.fRec37[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec42[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec43[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec44[l44 as usize] = 0.0;
		}
		for l45 in 0..3 {
			self.fRec41[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec46[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec47[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec48[l48 as usize] = 0.0;
		}
		for l49 in 0..3 {
			self.fRec45[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec50[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec51[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec52[l52 as usize] = 0.0;
		}
		for l53 in 0..3 {
			self.fRec49[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec54[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec55[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec56[l56 as usize] = 0.0;
		}
		for l57 in 0..3 {
			self.fRec53[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec58[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec59[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec60[l60 as usize] = 0.0;
		}
		for l61 in 0..3 {
			self.fRec57[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec62[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec63[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec64[l64 as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec61[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec66[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec67[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec68[l68 as usize] = 0.0;
		}
		for l69 in 0..3 {
			self.fRec65[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec70[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec71[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec72[l72 as usize] = 0.0;
		}
		for l73 in 0..3 {
			self.fRec69[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec74[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec75[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec76[l76 as usize] = 0.0;
		}
		for l77 in 0..3 {
			self.fRec73[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec78[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec79[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec80[l80 as usize] = 0.0;
		}
		for l81 in 0..3 {
			self.fRec77[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec82[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec83[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec84[l84 as usize] = 0.0;
		}
		for l85 in 0..3 {
			self.fRec81[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec86[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec87[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec88[l88 as usize] = 0.0;
		}
		for l89 in 0..3 {
			self.fRec85[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec90[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec91[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec92[l92 as usize] = 0.0;
		}
		for l93 in 0..3 {
			self.fRec89[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec94[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec95[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec96[l96 as usize] = 0.0;
		}
		for l97 in 0..3 {
			self.fRec93[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fVec1[l98 as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec97[l99 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::exp(-(3333.3333 / fConst0));
		self.fConst2 = 6.907755 / fConst0;
		self.fConst3 = 0.45 * fConst0;
		self.fConst4 = 6.2831855 / fConst0;
		self.fConst5 = 1.0 / F32::tan(31.415926 / fConst0);
		self.fConst6 = 1.0 - self.fConst5;
		self.fConst7 = 1.0 / (self.fConst5 + 1.0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		BodyDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("ABDrumBody");
		ui_interface.add_horizontal_slider("body_bright", ParamIndex(0), 0.4, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("body_clip", ParamIndex(1), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_button("body_gate", ParamIndex(2));
		ui_interface.add_horizontal_slider("body_hz", ParamIndex(3), 1e+02, 0.0, 1e+05, 0.001);
		ui_interface.add_horizontal_slider("body_material", ParamIndex(4), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("body_position", ParamIndex(5), 0.3, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("body_t60", ParamIndex(6), 0.4, 0.0, 1e+02, 0.0001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			2 => Some(self.fButton0),
			4 => Some(self.fHslider0),
			0 => Some(self.fHslider1),
			6 => Some(self.fHslider2),
			1 => Some(self.fHslider3),
			3 => Some(self.fHslider4),
			5 => Some(self.fHslider5),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			2 => { self.fButton0 = value }
			4 => { self.fHslider0 = value }
			0 => { self.fHslider1 = value }
			6 => { self.fHslider2 = value }
			1 => { self.fHslider3 = value }
			3 => { self.fHslider4 = value }
			5 => { self.fHslider5 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			(inputs0, inputs1)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = F32::max(0.0, F32::min(1.0, self.fHslider0));
		let mut fSlow2: F32 = 1.0 - fSlow1;
		let mut fSlow3: F32 = F32::max(0.0, 1.0 - F32::abs(-(2.0 * fSlow2)));
		let mut fSlow4: F32 = 2.0 * fSlow1;
		let mut fSlow5: F32 = F32::max(0.0, 1.0 - F32::abs(fSlow4 + -1.0));
		let mut fSlow6: F32 = F32::max(0.0, 1.0 - F32::abs(fSlow4));
		let mut fSlow7: F32 = 1.7879188 * fSlow6 + 2.8011155 * fSlow5;
		let mut fSlow8: F32 = F32::exp(fSlow7 + 5.578937 * fSlow3);
		let mut fSlow9: F32 = F32::max(0.0, F32::min(1.0, self.fHslider1));
		let mut fSlow10: F32 = fSlow9 - 1.0;
		let mut fSlow11: F32 = 1.2 * fSlow10;
		let mut fSlow12: F32 = self.fHslider2;
		let mut fSlow13: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow8, fSlow11))));
		let mut fSlow14: F32 = BodyDsp_faustpower2_f(fSlow13);
		let mut fSlow15: F32 = self.fHslider3;
		let mut iSlow16: i32 = (fSlow15 > 0.0001) as i32;
		let mut fSlow17: F32 = (iSlow16) as F32;
		let mut fSlow18: F32 = F32::powf(16.0, fSlow17 * F32::max(0.0, F32::min(1.0, fSlow15))) + -1.0;
		let mut fSlow19: F32 = self.fHslider4;
		let mut fSlow20: F32 = fSlow19 * fSlow8;
		let mut fSlow21: F32 = ((fSlow20 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow22: F32 = 1.5 * fSlow10;
		let mut fSlow23: F32 = F32::powf(fSlow8, fSlow22);
		let mut fSlow24: F32 = self.fHslider5;
		let mut fSlow25: F32 = 0.65 * F32::abs(F32::sin(75.398224 * fSlow24)) + 0.35;
		let mut fSlow26: F32 = BodyDsp_faustpower2_f(fSlow25);
		let mut fSlow27: F32 = 1.7318325 * fSlow6 + 2.7995958 * fSlow5;
		let mut fSlow28: F32 = F32::exp(fSlow27 + 5.495593 * fSlow3);
		let mut fSlow29: F32 = fSlow19 * fSlow28;
		let mut fSlow30: F32 = ((fSlow29 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow31: F32 = F32::powf(fSlow28, fSlow22);
		let mut fSlow32: F32 = 0.65 * F32::abs(F32::sin(72.25663 * fSlow24)) + 0.35;
		let mut fSlow33: F32 = BodyDsp_faustpower2_f(fSlow32);
		let mut fSlow34: F32 = 1.7143383 * fSlow6 + 2.7725887 * fSlow5;
		let mut fSlow35: F32 = F32::exp(fSlow34 + 5.4086237 * fSlow3);
		let mut fSlow36: F32 = fSlow19 * fSlow35;
		let mut fSlow37: F32 = ((fSlow36 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow38: F32 = F32::powf(fSlow35, fSlow22);
		let mut fSlow39: F32 = 0.65 * F32::abs(F32::sin(69.115036 * fSlow24)) + 0.35;
		let mut fSlow40: F32 = BodyDsp_faustpower2_f(fSlow39);
		let mut fSlow41: F32 = 1.7119945 * fSlow6 + 2.6579492 * fSlow5;
		let mut fSlow42: F32 = F32::exp(fSlow41 + 5.3176985 * fSlow3);
		let mut fSlow43: F32 = fSlow19 * fSlow42;
		let mut fSlow44: F32 = ((fSlow43 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow45: F32 = F32::powf(fSlow42, fSlow22);
		let mut fSlow46: F32 = 0.65 * F32::abs(F32::sin(65.97344 * fSlow24)) + 0.35;
		let mut fSlow47: F32 = BodyDsp_faustpower2_f(fSlow46);
		let mut fSlow48: F32 = 1.6886187 * fSlow6 + 2.6436894 * fSlow5;
		let mut fSlow49: F32 = F32::exp(fSlow48 + 5.2224402 * fSlow3);
		let mut fSlow50: F32 = fSlow19 * fSlow49;
		let mut fSlow51: F32 = ((fSlow50 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow52: F32 = F32::powf(fSlow49, fSlow22);
		let mut fSlow53: F32 = 0.65 * F32::abs(F32::sin(62.831852 * fSlow24)) + 0.35;
		let mut fSlow54: F32 = BodyDsp_faustpower2_f(fSlow53);
		let mut fSlow55: F32 = 1.6353005 * fSlow6 + 2.5968952 * fSlow5;
		let mut fSlow56: F32 = F32::exp(fSlow55 + 5.1224213 * fSlow3);
		let mut fSlow57: F32 = fSlow19 * fSlow56;
		let mut fSlow58: F32 = ((fSlow57 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow59: F32 = F32::powf(fSlow56, fSlow22);
		let mut fSlow60: F32 = 0.65 * F32::abs(F32::sin(59.69026 * fSlow24)) + 0.35;
		let mut fSlow61: F32 = BodyDsp_faustpower2_f(fSlow60);
		let mut fSlow62: F32 = 1.6260984 * fSlow6 + 2.5529547 * fSlow5;
		let mut fSlow63: F32 = F32::exp(fSlow62 + 5.017134 * fSlow3);
		let mut fSlow64: F32 = fSlow19 * fSlow63;
		let mut fSlow65: F32 = ((fSlow64 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow66: F32 = F32::powf(fSlow63, fSlow22);
		let mut fSlow67: F32 = 0.65 * F32::abs(F32::sin(56.548668 * fSlow24)) + 0.35;
		let mut fSlow68: F32 = BodyDsp_faustpower2_f(fSlow67);
		let mut fSlow69: F32 = 1.5898472 * fSlow6 + 2.5402625 * fSlow5;
		let mut fSlow70: F32 = F32::exp(fSlow69 + 4.905993 * fSlow3);
		let mut fSlow71: F32 = fSlow19 * fSlow70;
		let mut fSlow72: F32 = ((fSlow71 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow73: F32 = F32::powf(fSlow70, fSlow22);
		let mut fSlow74: F32 = 0.65 * F32::abs(F32::sin(53.407074 * fSlow24)) + 0.35;
		let mut fSlow75: F32 = BodyDsp_faustpower2_f(fSlow74);
		let mut fSlow76: F32 = 1.5752605 * fSlow6 + 2.4726653 * fSlow5;
		let mut fSlow77: F32 = F32::exp(fSlow76 + 4.7883162 * fSlow3);
		let mut fSlow78: F32 = fSlow19 * fSlow77;
		let mut fSlow79: F32 = ((fSlow78 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow80: F32 = F32::powf(fSlow77, fSlow22);
		let mut fSlow81: F32 = 0.65 * F32::abs(F32::sin(50.265484 * fSlow24)) + 0.35;
		let mut fSlow82: F32 = BodyDsp_faustpower2_f(fSlow81);
		let mut fSlow83: F32 = 1.5282278 * fSlow6 + 2.3919687 * fSlow5;
		let mut fSlow84: F32 = F32::exp(fSlow83 + 4.663269 * fSlow3);
		let mut fSlow85: F32 = fSlow19 * fSlow84;
		let mut fSlow86: F32 = ((fSlow85 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow87: F32 = F32::powf(fSlow84, fSlow22);
		let mut fSlow88: F32 = 0.65 * F32::abs(F32::sin(47.12389 * fSlow24)) + 0.35;
		let mut fSlow89: F32 = BodyDsp_faustpower2_f(fSlow88);
		let mut fSlow90: F32 = 1.5262737 * fSlow6 + 2.367811 * fSlow5;
		let mut fSlow91: F32 = F32::exp(fSlow90 + 4.5298862 * fSlow3);
		let mut fSlow92: F32 = fSlow19 * fSlow91;
		let mut fSlow93: F32 = ((fSlow92 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow94: F32 = F32::powf(fSlow91, fSlow22);
		let mut fSlow95: F32 = 0.65 * F32::abs(F32::sin(43.982296 * fSlow24)) + 0.35;
		let mut fSlow96: F32 = BodyDsp_faustpower2_f(fSlow95);
		let mut fSlow97: F32 = 1.442202 * fSlow6 + 2.2827904 * fSlow5;
		let mut fSlow98: F32 = F32::exp(fSlow97 + 4.3869767 * fSlow3);
		let mut fSlow99: F32 = fSlow19 * fSlow98;
		let mut fSlow100: F32 = ((fSlow99 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow101: F32 = F32::powf(fSlow98, fSlow22);
		let mut fSlow102: F32 = 0.65 * F32::abs(F32::sin(40.840706 * fSlow24)) + 0.35;
		let mut fSlow103: F32 = BodyDsp_faustpower2_f(fSlow102);
		let mut fSlow104: F32 = 1.4187615 * fSlow6 + 2.1972246 * fSlow5;
		let mut fSlow105: F32 = F32::exp(fSlow104 + 4.233048 * fSlow3);
		let mut fSlow106: F32 = fSlow19 * fSlow105;
		let mut fSlow107: F32 = ((fSlow106 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow108: F32 = F32::powf(fSlow105, fSlow22);
		let mut fSlow109: F32 = 0.65 * F32::abs(F32::sin(37.699112 * fSlow24)) + 0.35;
		let mut fSlow110: F32 = BodyDsp_faustpower2_f(fSlow109);
		let mut fSlow111: F32 = 1.4009366 * fSlow6 + 2.032481 * fSlow5;
		let mut fSlow112: F32 = F32::exp(fSlow111 + 4.066288 * fSlow3);
		let mut fSlow113: F32 = fSlow19 * fSlow112;
		let mut fSlow114: F32 = ((fSlow113 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow115: F32 = F32::powf(fSlow112, fSlow22);
		let mut fSlow116: F32 = 0.65 * F32::abs(F32::sin(34.557518 * fSlow24)) + 0.35;
		let mut fSlow117: F32 = BodyDsp_faustpower2_f(fSlow116);
		let mut fSlow118: F32 = 1.2939049 * fSlow6 + 2.0305138 * fSlow5;
		let mut fSlow119: F32 = F32::exp(fSlow118 + 3.8843434 * fSlow3);
		let mut fSlow120: F32 = fSlow19 * fSlow119;
		let mut fSlow121: F32 = ((fSlow120 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow122: F32 = F32::powf(fSlow119, fSlow22);
		let mut fSlow123: F32 = 0.65 * F32::abs(F32::sin(31.415926 * fSlow24)) + 0.35;
		let mut fSlow124: F32 = BodyDsp_faustpower2_f(fSlow123);
		let mut fSlow125: F32 = 1.2803781 * fSlow6 + 1.9902004 * fSlow5;
		let mut fSlow126: F32 = F32::exp(fSlow125 + 3.6841683 * fSlow3);
		let mut fSlow127: F32 = fSlow19 * fSlow126;
		let mut fSlow128: F32 = ((fSlow127 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow129: F32 = F32::powf(fSlow126, fSlow22);
		let mut fSlow130: F32 = 0.65 * F32::abs(F32::sin(28.274334 * fSlow24)) + 0.35;
		let mut fSlow131: F32 = BodyDsp_faustpower2_f(fSlow130);
		let mut fSlow132: F32 = 1.2527629 * fSlow6 + 1.9153037 * fSlow5;
		let mut fSlow133: F32 = F32::exp(fSlow132 + 3.4617279 * fSlow3);
		let mut fSlow134: F32 = fSlow19 * fSlow133;
		let mut fSlow135: F32 = ((fSlow134 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow136: F32 = F32::powf(fSlow133, fSlow22);
		let mut fSlow137: F32 = 0.65 * F32::abs(F32::sin(25.132742 * fSlow24)) + 0.35;
		let mut fSlow138: F32 = BodyDsp_faustpower2_f(fSlow137);
		let mut fSlow139: F32 = 1.1489885 * fSlow6 + 1.68306 * fSlow5;
		let mut fSlow140: F32 = F32::exp(fSlow139 + 3.211408 * fSlow3);
		let mut fSlow141: F32 = fSlow19 * fSlow140;
		let mut fSlow142: F32 = ((fSlow141 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow143: F32 = F32::powf(fSlow140, fSlow22);
		let mut fSlow144: F32 = 0.65 * F32::abs(F32::sin(21.991148 * fSlow24)) + 0.35;
		let mut fSlow145: F32 = BodyDsp_faustpower2_f(fSlow144);
		let mut fSlow146: F32 = 1.0705557 * fSlow6 + 1.6382197 * fSlow5;
		let mut fSlow147: F32 = F32::exp(fSlow146 + 2.9252026 * fSlow3);
		let mut fSlow148: F32 = fSlow19 * fSlow147;
		let mut fSlow149: F32 = ((fSlow148 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow150: F32 = F32::powf(fSlow147, fSlow22);
		let mut fSlow151: F32 = 0.65 * F32::abs(F32::sin(18.849556 * fSlow24)) + 0.35;
		let mut fSlow152: F32 = BodyDsp_faustpower2_f(fSlow151);
		let mut fSlow153: F32 = 0.9756911 * fSlow6 + 1.3862944 * fSlow5;
		let mut fSlow154: F32 = F32::exp(fSlow153 + 2.5910668 * fSlow3);
		let mut fSlow155: F32 = fSlow19 * fSlow154;
		let mut fSlow156: F32 = ((fSlow155 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow157: F32 = F32::powf(fSlow154, fSlow22);
		let mut fSlow158: F32 = 0.65 * F32::abs(F32::sin(15.707963 * fSlow24)) + 0.35;
		let mut fSlow159: F32 = BodyDsp_faustpower2_f(fSlow158);
		let mut fSlow160: F32 = 0.8307328 * fSlow6 + 1.1665825 * fSlow5;
		let mut fSlow161: F32 = F32::exp(fSlow160 + 2.1897523 * fSlow3);
		let mut fSlow162: F32 = fSlow19 * fSlow161;
		let mut fSlow163: F32 = ((fSlow162 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow164: F32 = F32::powf(fSlow161, fSlow22);
		let mut fSlow165: F32 = 0.65 * F32::abs(F32::sin(12.566371 * fSlow24)) + 0.35;
		let mut fSlow166: F32 = BodyDsp_faustpower2_f(fSlow165);
		let mut fSlow167: F32 = 0.7589349 * fSlow6 + 1.154047 * fSlow5;
		let mut fSlow168: F32 = F32::exp(fSlow167 + 1.6871394 * fSlow3);
		let mut fSlow169: F32 = fSlow19 * fSlow168;
		let mut fSlow170: F32 = ((fSlow169 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow171: F32 = F32::powf(fSlow168, fSlow22);
		let mut fSlow172: F32 = 0.65 * F32::abs(F32::sin(9.424778 * fSlow24)) + 0.35;
		let mut fSlow173: F32 = BodyDsp_faustpower2_f(fSlow172);
		let mut fSlow174: F32 = 0.46561903 * fSlow6 + 0.60376936 * fSlow5;
		let mut fSlow175: F32 = F32::exp(fSlow174 + 1.0141431 * fSlow3);
		let mut fSlow176: F32 = fSlow19 * fSlow175;
		let mut fSlow177: F32 = ((fSlow176 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow178: F32 = F32::powf(fSlow175, fSlow22);
		let mut fSlow179: F32 = 0.65 * F32::abs(F32::sin(6.2831855 * fSlow24)) + 0.35;
		let mut fSlow180: F32 = BodyDsp_faustpower2_f(fSlow179);
		let mut fSlow181: F32 = ((fSlow19 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow182: F32 = BodyDsp_faustpower2_f(fSlow181);
		let mut fSlow183: F32 = F32::powf(1.0, fSlow22);
		let mut fSlow184: F32 = 0.65 * F32::abs(F32::sin(3.1415927 * fSlow24)) + 0.35;
		let mut fSlow185: F32 = BodyDsp_faustpower2_f(fSlow184);
		let mut fSlow186: F32 = F32::max(0.1, F32::sqrt(fSlow185 * BodyDsp_faustpower2_f(fSlow183) * fSlow182 + fSlow180 * BodyDsp_faustpower2_f(fSlow178) * BodyDsp_faustpower2_f(fSlow177) + fSlow173 * BodyDsp_faustpower2_f(fSlow171) * BodyDsp_faustpower2_f(fSlow170) + fSlow166 * BodyDsp_faustpower2_f(fSlow164) * BodyDsp_faustpower2_f(fSlow163) + fSlow159 * BodyDsp_faustpower2_f(fSlow157) * BodyDsp_faustpower2_f(fSlow156) + fSlow152 * BodyDsp_faustpower2_f(fSlow150) * BodyDsp_faustpower2_f(fSlow149) + fSlow145 * BodyDsp_faustpower2_f(fSlow143) * BodyDsp_faustpower2_f(fSlow142) + fSlow138 * BodyDsp_faustpower2_f(fSlow136) * BodyDsp_faustpower2_f(fSlow135) + fSlow131 * BodyDsp_faustpower2_f(fSlow129) * BodyDsp_faustpower2_f(fSlow128) + fSlow124 * BodyDsp_faustpower2_f(fSlow122) * BodyDsp_faustpower2_f(fSlow121) + fSlow117 * BodyDsp_faustpower2_f(fSlow115) * BodyDsp_faustpower2_f(fSlow114) + fSlow110 * BodyDsp_faustpower2_f(fSlow108) * BodyDsp_faustpower2_f(fSlow107) + fSlow103 * BodyDsp_faustpower2_f(fSlow101) * BodyDsp_faustpower2_f(fSlow100) + fSlow96 * BodyDsp_faustpower2_f(fSlow94) * BodyDsp_faustpower2_f(fSlow93) + fSlow89 * BodyDsp_faustpower2_f(fSlow87) * BodyDsp_faustpower2_f(fSlow86) + fSlow82 * BodyDsp_faustpower2_f(fSlow80) * BodyDsp_faustpower2_f(fSlow79) + fSlow75 * BodyDsp_faustpower2_f(fSlow73) * BodyDsp_faustpower2_f(fSlow72) + fSlow68 * BodyDsp_faustpower2_f(fSlow66) * BodyDsp_faustpower2_f(fSlow65) + fSlow61 * BodyDsp_faustpower2_f(fSlow59) * BodyDsp_faustpower2_f(fSlow58) + fSlow54 * BodyDsp_faustpower2_f(fSlow52) * BodyDsp_faustpower2_f(fSlow51) + fSlow47 * BodyDsp_faustpower2_f(fSlow45) * BodyDsp_faustpower2_f(fSlow44) + fSlow40 * BodyDsp_faustpower2_f(fSlow38) * BodyDsp_faustpower2_f(fSlow37) + fSlow33 * BodyDsp_faustpower2_f(fSlow31) * BodyDsp_faustpower2_f(fSlow30) + fSlow26 * BodyDsp_faustpower2_f(fSlow23) * BodyDsp_faustpower2_f(fSlow21)));
		let mut fSlow187: F32 = fSlow25 * fSlow23 * fSlow21;
		let mut fSlow188: F32 = F32::max(1e-09, 0.3 * (fSlow187 / fSlow186));
		let mut fSlow189: F32 = self.fConst4 * F32::min(fSlow20, self.fConst3);
		let mut fSlow190: F32 = 2.0 * fSlow13 * F32::cos(fSlow189);
		let mut fSlow191: F32 = 0.3 * (fSlow187 * F32::sin(fSlow189) / fSlow186);
		let mut fSlow192: F32 = F32::max(0.0, 1.0 - F32::abs(-2.0 * fSlow2));
		let mut fSlow193: F32 = F32::exp(fSlow7 + 5.578937 * fSlow192);
		let mut fSlow194: F32 = (((fSlow19 * fSlow193) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow195: F32 = 1.5 * (fSlow9 + -1.0);
		let mut fSlow196: F32 = F32::powf(fSlow193, fSlow195);
		let mut fSlow197: F32 = F32::exp(fSlow27 + 5.495593 * fSlow192);
		let mut fSlow198: F32 = (((fSlow19 * fSlow197) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow199: F32 = F32::powf(fSlow197, fSlow195);
		let mut fSlow200: F32 = F32::exp(fSlow34 + 5.4086237 * fSlow192);
		let mut fSlow201: F32 = (((fSlow19 * fSlow200) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow202: F32 = F32::powf(fSlow200, fSlow195);
		let mut fSlow203: F32 = F32::exp(fSlow48 + 5.2224402 * fSlow192);
		let mut fSlow204: F32 = (((fSlow19 * fSlow203) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow205: F32 = F32::powf(fSlow203, fSlow195);
		let mut fSlow206: F32 = F32::exp(fSlow55 + 5.1224213 * fSlow192);
		let mut fSlow207: F32 = (((fSlow19 * fSlow206) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow208: F32 = F32::powf(fSlow206, fSlow195);
		let mut fSlow209: F32 = F32::exp(fSlow69 + 4.905993 * fSlow192);
		let mut fSlow210: F32 = (((fSlow19 * fSlow209) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow211: F32 = F32::powf(fSlow209, fSlow195);
		let mut fSlow212: F32 = F32::exp(fSlow76 + 4.7883162 * fSlow192);
		let mut fSlow213: F32 = (((fSlow19 * fSlow212) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow214: F32 = F32::powf(fSlow212, fSlow195);
		let mut fSlow215: F32 = F32::exp(fSlow83 + 4.663269 * fSlow192);
		let mut fSlow216: F32 = (((fSlow19 * fSlow215) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow217: F32 = F32::powf(fSlow215, fSlow195);
		let mut fSlow218: F32 = F32::exp(fSlow90 + 4.5298862 * fSlow192);
		let mut fSlow219: F32 = (((fSlow19 * fSlow218) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow220: F32 = F32::powf(fSlow218, fSlow195);
		let mut fSlow221: F32 = F32::exp(fSlow97 + 4.3869767 * fSlow192);
		let mut fSlow222: F32 = (((fSlow19 * fSlow221) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow223: F32 = F32::powf(fSlow221, fSlow195);
		let mut fSlow224: F32 = F32::exp(fSlow104 + 4.233048 * fSlow192);
		let mut fSlow225: F32 = (((fSlow19 * fSlow224) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow226: F32 = F32::powf(fSlow224, fSlow195);
		let mut fSlow227: F32 = F32::exp(fSlow111 + 4.066288 * fSlow192);
		let mut fSlow228: F32 = (((fSlow19 * fSlow227) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow229: F32 = F32::powf(fSlow227, fSlow195);
		let mut fSlow230: F32 = F32::exp(fSlow118 + 3.8843434 * fSlow192);
		let mut fSlow231: F32 = (((fSlow19 * fSlow230) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow232: F32 = F32::powf(fSlow230, fSlow195);
		let mut fSlow233: F32 = F32::exp(fSlow125 + 3.6841683 * fSlow192);
		let mut fSlow234: F32 = (((fSlow19 * fSlow233) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow235: F32 = F32::powf(fSlow233, fSlow195);
		let mut fSlow236: F32 = F32::exp(fSlow132 + 3.4617279 * fSlow192);
		let mut fSlow237: F32 = (((fSlow19 * fSlow236) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow238: F32 = F32::powf(fSlow236, fSlow195);
		let mut fSlow239: F32 = F32::exp(fSlow139 + 3.211408 * fSlow192);
		let mut fSlow240: F32 = (((fSlow19 * fSlow239) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow241: F32 = F32::powf(fSlow239, fSlow195);
		let mut fSlow242: F32 = F32::exp(fSlow146 + 2.9252026 * fSlow192);
		let mut fSlow243: F32 = (((fSlow19 * fSlow242) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow244: F32 = F32::powf(fSlow242, fSlow195);
		let mut fSlow245: F32 = F32::exp(fSlow153 + 2.5910668 * fSlow192);
		let mut fSlow246: F32 = (((fSlow19 * fSlow245) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow247: F32 = F32::powf(fSlow245, fSlow195);
		let mut fSlow248: F32 = F32::exp(fSlow160 + 2.1897523 * fSlow192);
		let mut fSlow249: F32 = (((fSlow19 * fSlow248) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow250: F32 = F32::powf(fSlow248, fSlow195);
		let mut fSlow251: F32 = F32::exp(fSlow167 + 1.6871394 * fSlow192);
		let mut fSlow252: F32 = (((fSlow19 * fSlow251) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow253: F32 = F32::powf(fSlow251, fSlow195);
		let mut fSlow254: F32 = F32::exp(fSlow174 + 1.0141431 * fSlow192);
		let mut fSlow255: F32 = (((fSlow19 * fSlow254) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow256: F32 = F32::powf(fSlow254, fSlow195);
		let mut fSlow257: F32 = F32::powf(1.0, fSlow195);
		let mut fSlow258: F32 = F32::exp(fSlow62 + 5.017134 * fSlow192);
		let mut fSlow259: F32 = (((fSlow19 * fSlow258) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow260: F32 = F32::powf(fSlow258, fSlow195);
		let mut fSlow261: F32 = F32::exp(fSlow41 + 5.3176985 * fSlow192);
		let mut fSlow262: F32 = (((fSlow19 * fSlow261) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow263: F32 = F32::powf(fSlow261, fSlow195);
		let mut fSlow264: F32 = F32::max(0.1, F32::sqrt(fSlow47 * BodyDsp_faustpower2_f(fSlow263) * BodyDsp_faustpower2_f(fSlow262) + fSlow68 * BodyDsp_faustpower2_f(fSlow260) * BodyDsp_faustpower2_f(fSlow259) + fSlow185 * fSlow182 * BodyDsp_faustpower2_f(fSlow257) + fSlow180 * BodyDsp_faustpower2_f(fSlow256) * BodyDsp_faustpower2_f(fSlow255) + fSlow173 * BodyDsp_faustpower2_f(fSlow253) * BodyDsp_faustpower2_f(fSlow252) + fSlow166 * BodyDsp_faustpower2_f(fSlow250) * BodyDsp_faustpower2_f(fSlow249) + fSlow159 * BodyDsp_faustpower2_f(fSlow247) * BodyDsp_faustpower2_f(fSlow246) + fSlow152 * BodyDsp_faustpower2_f(fSlow244) * BodyDsp_faustpower2_f(fSlow243) + fSlow145 * BodyDsp_faustpower2_f(fSlow241) * BodyDsp_faustpower2_f(fSlow240) + fSlow138 * BodyDsp_faustpower2_f(fSlow238) * BodyDsp_faustpower2_f(fSlow237) + fSlow131 * BodyDsp_faustpower2_f(fSlow235) * BodyDsp_faustpower2_f(fSlow234) + fSlow124 * BodyDsp_faustpower2_f(fSlow232) * BodyDsp_faustpower2_f(fSlow231) + fSlow117 * BodyDsp_faustpower2_f(fSlow229) * BodyDsp_faustpower2_f(fSlow228) + fSlow110 * BodyDsp_faustpower2_f(fSlow226) * BodyDsp_faustpower2_f(fSlow225) + fSlow103 * BodyDsp_faustpower2_f(fSlow223) * BodyDsp_faustpower2_f(fSlow222) + fSlow96 * BodyDsp_faustpower2_f(fSlow220) * BodyDsp_faustpower2_f(fSlow219) + fSlow89 * BodyDsp_faustpower2_f(fSlow217) * BodyDsp_faustpower2_f(fSlow216) + fSlow82 * BodyDsp_faustpower2_f(fSlow214) * BodyDsp_faustpower2_f(fSlow213) + fSlow75 * BodyDsp_faustpower2_f(fSlow211) * BodyDsp_faustpower2_f(fSlow210) + fSlow61 * BodyDsp_faustpower2_f(fSlow208) * BodyDsp_faustpower2_f(fSlow207) + fSlow54 * BodyDsp_faustpower2_f(fSlow205) * BodyDsp_faustpower2_f(fSlow204) + fSlow40 * BodyDsp_faustpower2_f(fSlow202) * BodyDsp_faustpower2_f(fSlow201) + fSlow33 * BodyDsp_faustpower2_f(fSlow199) * BodyDsp_faustpower2_f(fSlow198) + fSlow26 * BodyDsp_faustpower2_f(fSlow196) * BodyDsp_faustpower2_f(fSlow194)));
		let mut fSlow265: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow25 * fSlow196 * fSlow194 / fSlow264));
		let mut fSlow266: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow28, fSlow11))));
		let mut fSlow267: F32 = BodyDsp_faustpower2_f(fSlow266);
		let mut fSlow268: F32 = fSlow32 * fSlow31 * fSlow30;
		let mut fSlow269: F32 = F32::max(1e-09, 0.3 * (fSlow268 / fSlow186));
		let mut fSlow270: F32 = self.fConst4 * F32::min(fSlow29, self.fConst3);
		let mut fSlow271: F32 = 2.0 * fSlow266 * F32::cos(fSlow270);
		let mut fSlow272: F32 = 0.3 * (fSlow268 * F32::sin(fSlow270) / fSlow186);
		let mut fSlow273: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow32 * fSlow199 * fSlow198 / fSlow264));
		let mut fSlow274: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow35, fSlow11))));
		let mut fSlow275: F32 = BodyDsp_faustpower2_f(fSlow274);
		let mut fSlow276: F32 = fSlow39 * fSlow38 * fSlow37;
		let mut fSlow277: F32 = F32::max(1e-09, 0.3 * (fSlow276 / fSlow186));
		let mut fSlow278: F32 = self.fConst4 * F32::min(fSlow36, self.fConst3);
		let mut fSlow279: F32 = 2.0 * fSlow274 * F32::cos(fSlow278);
		let mut fSlow280: F32 = 0.3 * (fSlow276 * F32::sin(fSlow278) / fSlow186);
		let mut fSlow281: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow39 * fSlow202 * fSlow201 / fSlow264));
		let mut fSlow282: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow42, fSlow11))));
		let mut fSlow283: F32 = BodyDsp_faustpower2_f(fSlow282);
		let mut fSlow284: F32 = fSlow46 * fSlow45 * fSlow44;
		let mut fSlow285: F32 = F32::max(1e-09, 0.3 * (fSlow284 / fSlow186));
		let mut fSlow286: F32 = self.fConst4 * F32::min(fSlow43, self.fConst3);
		let mut fSlow287: F32 = 2.0 * fSlow282 * F32::cos(fSlow286);
		let mut fSlow288: F32 = 0.3 * (fSlow284 * F32::sin(fSlow286) / fSlow186);
		let mut fSlow289: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow46 * fSlow263 * fSlow262 / fSlow264));
		let mut fSlow290: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow49, fSlow11))));
		let mut fSlow291: F32 = BodyDsp_faustpower2_f(fSlow290);
		let mut fSlow292: F32 = fSlow53 * fSlow52 * fSlow51;
		let mut fSlow293: F32 = F32::max(1e-09, 0.3 * (fSlow292 / fSlow186));
		let mut fSlow294: F32 = self.fConst4 * F32::min(fSlow50, self.fConst3);
		let mut fSlow295: F32 = 2.0 * fSlow290 * F32::cos(fSlow294);
		let mut fSlow296: F32 = 0.3 * (fSlow292 * F32::sin(fSlow294) / fSlow186);
		let mut fSlow297: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow53 * fSlow205 * fSlow204 / fSlow264));
		let mut fSlow298: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow56, fSlow11))));
		let mut fSlow299: F32 = BodyDsp_faustpower2_f(fSlow298);
		let mut fSlow300: F32 = fSlow60 * fSlow59 * fSlow58;
		let mut fSlow301: F32 = F32::max(1e-09, 0.3 * (fSlow300 / fSlow186));
		let mut fSlow302: F32 = self.fConst4 * F32::min(fSlow57, self.fConst3);
		let mut fSlow303: F32 = 2.0 * fSlow298 * F32::cos(fSlow302);
		let mut fSlow304: F32 = 0.3 * (fSlow300 * F32::sin(fSlow302) / fSlow186);
		let mut fSlow305: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow60 * fSlow208 * fSlow207 / fSlow264));
		let mut fSlow306: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow63, fSlow11))));
		let mut fSlow307: F32 = BodyDsp_faustpower2_f(fSlow306);
		let mut fSlow308: F32 = fSlow67 * fSlow66 * fSlow65;
		let mut fSlow309: F32 = F32::max(1e-09, 0.3 * (fSlow308 / fSlow186));
		let mut fSlow310: F32 = self.fConst4 * F32::min(fSlow64, self.fConst3);
		let mut fSlow311: F32 = 2.0 * fSlow306 * F32::cos(fSlow310);
		let mut fSlow312: F32 = 0.3 * (fSlow308 * F32::sin(fSlow310) / fSlow186);
		let mut fSlow313: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow67 * fSlow260 * fSlow259 / fSlow264));
		let mut fSlow314: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow70, fSlow11))));
		let mut fSlow315: F32 = BodyDsp_faustpower2_f(fSlow314);
		let mut fSlow316: F32 = fSlow74 * fSlow73 * fSlow72;
		let mut fSlow317: F32 = F32::max(1e-09, 0.3 * (fSlow316 / fSlow186));
		let mut fSlow318: F32 = self.fConst4 * F32::min(fSlow71, self.fConst3);
		let mut fSlow319: F32 = 2.0 * fSlow314 * F32::cos(fSlow318);
		let mut fSlow320: F32 = 0.3 * (fSlow316 * F32::sin(fSlow318) / fSlow186);
		let mut fSlow321: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow74 * fSlow211 * fSlow210 / fSlow264));
		let mut fSlow322: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow77, fSlow11))));
		let mut fSlow323: F32 = BodyDsp_faustpower2_f(fSlow322);
		let mut fSlow324: F32 = fSlow81 * fSlow80 * fSlow79;
		let mut fSlow325: F32 = F32::max(1e-09, 0.3 * (fSlow324 / fSlow186));
		let mut fSlow326: F32 = self.fConst4 * F32::min(fSlow78, self.fConst3);
		let mut fSlow327: F32 = 2.0 * fSlow322 * F32::cos(fSlow326);
		let mut fSlow328: F32 = 0.3 * (fSlow324 * F32::sin(fSlow326) / fSlow186);
		let mut fSlow329: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow81 * fSlow214 * fSlow213 / fSlow264));
		let mut fSlow330: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow84, fSlow11))));
		let mut fSlow331: F32 = BodyDsp_faustpower2_f(fSlow330);
		let mut fSlow332: F32 = fSlow88 * fSlow87 * fSlow86;
		let mut fSlow333: F32 = F32::max(1e-09, 0.3 * (fSlow332 / fSlow186));
		let mut fSlow334: F32 = self.fConst4 * F32::min(fSlow85, self.fConst3);
		let mut fSlow335: F32 = 2.0 * fSlow330 * F32::cos(fSlow334);
		let mut fSlow336: F32 = 0.3 * (fSlow332 * F32::sin(fSlow334) / fSlow186);
		let mut fSlow337: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow88 * fSlow217 * fSlow216 / fSlow264));
		let mut fSlow338: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow91, fSlow11))));
		let mut fSlow339: F32 = BodyDsp_faustpower2_f(fSlow338);
		let mut fSlow340: F32 = fSlow95 * fSlow94 * fSlow93;
		let mut fSlow341: F32 = F32::max(1e-09, 0.3 * (fSlow340 / fSlow186));
		let mut fSlow342: F32 = self.fConst4 * F32::min(fSlow92, self.fConst3);
		let mut fSlow343: F32 = 2.0 * fSlow338 * F32::cos(fSlow342);
		let mut fSlow344: F32 = 0.3 * (fSlow340 * F32::sin(fSlow342) / fSlow186);
		let mut fSlow345: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow95 * fSlow220 * fSlow219 / fSlow264));
		let mut fSlow346: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow98, fSlow11))));
		let mut fSlow347: F32 = BodyDsp_faustpower2_f(fSlow346);
		let mut fSlow348: F32 = fSlow102 * fSlow101 * fSlow100;
		let mut fSlow349: F32 = F32::max(1e-09, 0.3 * (fSlow348 / fSlow186));
		let mut fSlow350: F32 = self.fConst4 * F32::min(fSlow99, self.fConst3);
		let mut fSlow351: F32 = 2.0 * fSlow346 * F32::cos(fSlow350);
		let mut fSlow352: F32 = 0.3 * (fSlow348 * F32::sin(fSlow350) / fSlow186);
		let mut fSlow353: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow102 * fSlow223 * fSlow222 / fSlow264));
		let mut fSlow354: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow105, fSlow11))));
		let mut fSlow355: F32 = BodyDsp_faustpower2_f(fSlow354);
		let mut fSlow356: F32 = fSlow109 * fSlow108 * fSlow107;
		let mut fSlow357: F32 = F32::max(1e-09, 0.3 * (fSlow356 / fSlow186));
		let mut fSlow358: F32 = self.fConst4 * F32::min(fSlow106, self.fConst3);
		let mut fSlow359: F32 = 2.0 * fSlow354 * F32::cos(fSlow358);
		let mut fSlow360: F32 = 0.3 * (fSlow356 * F32::sin(fSlow358) / fSlow186);
		let mut fSlow361: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow109 * fSlow226 * fSlow225 / fSlow264));
		let mut fSlow362: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow112, fSlow11))));
		let mut fSlow363: F32 = BodyDsp_faustpower2_f(fSlow362);
		let mut fSlow364: F32 = fSlow116 * fSlow115 * fSlow114;
		let mut fSlow365: F32 = F32::max(1e-09, 0.3 * (fSlow364 / fSlow186));
		let mut fSlow366: F32 = self.fConst4 * F32::min(fSlow113, self.fConst3);
		let mut fSlow367: F32 = 2.0 * fSlow362 * F32::cos(fSlow366);
		let mut fSlow368: F32 = 0.3 * (fSlow364 * F32::sin(fSlow366) / fSlow186);
		let mut fSlow369: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow116 * fSlow229 * fSlow228 / fSlow264));
		let mut fSlow370: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow119, fSlow11))));
		let mut fSlow371: F32 = BodyDsp_faustpower2_f(fSlow370);
		let mut fSlow372: F32 = fSlow123 * fSlow122 * fSlow121;
		let mut fSlow373: F32 = F32::max(1e-09, 0.3 * (fSlow372 / fSlow186));
		let mut fSlow374: F32 = self.fConst4 * F32::min(fSlow120, self.fConst3);
		let mut fSlow375: F32 = 2.0 * fSlow370 * F32::cos(fSlow374);
		let mut fSlow376: F32 = 0.3 * (fSlow372 * F32::sin(fSlow374) / fSlow186);
		let mut fSlow377: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow123 * fSlow232 * fSlow231 / fSlow264));
		let mut fSlow378: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow126, fSlow11))));
		let mut fSlow379: F32 = BodyDsp_faustpower2_f(fSlow378);
		let mut fSlow380: F32 = fSlow130 * fSlow129 * fSlow128;
		let mut fSlow381: F32 = F32::max(1e-09, 0.3 * (fSlow380 / fSlow186));
		let mut fSlow382: F32 = self.fConst4 * F32::min(fSlow127, self.fConst3);
		let mut fSlow383: F32 = 2.0 * fSlow378 * F32::cos(fSlow382);
		let mut fSlow384: F32 = 0.3 * (fSlow380 * F32::sin(fSlow382) / fSlow186);
		let mut fSlow385: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow130 * fSlow235 * fSlow234 / fSlow264));
		let mut fSlow386: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow133, fSlow11))));
		let mut fSlow387: F32 = BodyDsp_faustpower2_f(fSlow386);
		let mut fSlow388: F32 = fSlow137 * fSlow136 * fSlow135;
		let mut fSlow389: F32 = F32::max(1e-09, 0.3 * (fSlow388 / fSlow186));
		let mut fSlow390: F32 = self.fConst4 * F32::min(fSlow134, self.fConst3);
		let mut fSlow391: F32 = 2.0 * fSlow386 * F32::cos(fSlow390);
		let mut fSlow392: F32 = 0.3 * (fSlow388 * F32::sin(fSlow390) / fSlow186);
		let mut fSlow393: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow137 * fSlow238 * fSlow237 / fSlow264));
		let mut fSlow394: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow140, fSlow11))));
		let mut fSlow395: F32 = BodyDsp_faustpower2_f(fSlow394);
		let mut fSlow396: F32 = fSlow144 * fSlow143 * fSlow142;
		let mut fSlow397: F32 = F32::max(1e-09, 0.3 * (fSlow396 / fSlow186));
		let mut fSlow398: F32 = self.fConst4 * F32::min(fSlow141, self.fConst3);
		let mut fSlow399: F32 = 2.0 * fSlow394 * F32::cos(fSlow398);
		let mut fSlow400: F32 = 0.3 * (fSlow396 * F32::sin(fSlow398) / fSlow186);
		let mut fSlow401: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow144 * fSlow241 * fSlow240 / fSlow264));
		let mut fSlow402: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow147, fSlow11))));
		let mut fSlow403: F32 = BodyDsp_faustpower2_f(fSlow402);
		let mut fSlow404: F32 = fSlow151 * fSlow150 * fSlow149;
		let mut fSlow405: F32 = F32::max(1e-09, 0.3 * (fSlow404 / fSlow186));
		let mut fSlow406: F32 = self.fConst4 * F32::min(fSlow148, self.fConst3);
		let mut fSlow407: F32 = 2.0 * fSlow402 * F32::cos(fSlow406);
		let mut fSlow408: F32 = 0.3 * (fSlow404 * F32::sin(fSlow406) / fSlow186);
		let mut fSlow409: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow151 * fSlow244 * fSlow243 / fSlow264));
		let mut fSlow410: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow154, fSlow11))));
		let mut fSlow411: F32 = BodyDsp_faustpower2_f(fSlow410);
		let mut fSlow412: F32 = fSlow158 * fSlow157 * fSlow156;
		let mut fSlow413: F32 = F32::max(1e-09, 0.3 * (fSlow412 / fSlow186));
		let mut fSlow414: F32 = self.fConst4 * F32::min(fSlow155, self.fConst3);
		let mut fSlow415: F32 = 2.0 * fSlow410 * F32::cos(fSlow414);
		let mut fSlow416: F32 = 0.3 * (fSlow412 * F32::sin(fSlow414) / fSlow186);
		let mut fSlow417: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow158 * fSlow247 * fSlow246 / fSlow264));
		let mut fSlow418: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow161, fSlow11))));
		let mut fSlow419: F32 = BodyDsp_faustpower2_f(fSlow418);
		let mut fSlow420: F32 = fSlow165 * fSlow164 * fSlow163;
		let mut fSlow421: F32 = F32::max(1e-09, 0.3 * (fSlow420 / fSlow186));
		let mut fSlow422: F32 = self.fConst4 * F32::min(fSlow162, self.fConst3);
		let mut fSlow423: F32 = 2.0 * fSlow418 * F32::cos(fSlow422);
		let mut fSlow424: F32 = 0.3 * (fSlow420 * F32::sin(fSlow422) / fSlow186);
		let mut fSlow425: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow165 * fSlow250 * fSlow249 / fSlow264));
		let mut fSlow426: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow168, fSlow11))));
		let mut fSlow427: F32 = BodyDsp_faustpower2_f(fSlow426);
		let mut fSlow428: F32 = fSlow172 * fSlow171 * fSlow170;
		let mut fSlow429: F32 = F32::max(1e-09, 0.3 * (fSlow428 / fSlow186));
		let mut fSlow430: F32 = self.fConst4 * F32::min(fSlow169, self.fConst3);
		let mut fSlow431: F32 = 2.0 * fSlow426 * F32::cos(fSlow430);
		let mut fSlow432: F32 = 0.3 * (fSlow428 * F32::sin(fSlow430) / fSlow186);
		let mut fSlow433: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow172 * fSlow253 * fSlow252 / fSlow264));
		let mut fSlow434: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow175, fSlow11))));
		let mut fSlow435: F32 = BodyDsp_faustpower2_f(fSlow434);
		let mut fSlow436: F32 = fSlow179 * fSlow178 * fSlow177;
		let mut fSlow437: F32 = F32::max(1e-09, 0.3 * (fSlow436 / fSlow186));
		let mut fSlow438: F32 = self.fConst4 * F32::min(fSlow176, self.fConst3);
		let mut fSlow439: F32 = 2.0 * fSlow434 * F32::cos(fSlow438);
		let mut fSlow440: F32 = 0.3 * (fSlow436 * F32::sin(fSlow438) / fSlow186);
		let mut fSlow441: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow179 * fSlow256 * fSlow255 / fSlow264));
		let mut fSlow442: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(1.0, fSlow11))));
		let mut fSlow443: F32 = BodyDsp_faustpower2_f(fSlow442);
		let mut fSlow444: F32 = F32::max(1e-09, 0.3 * (fSlow184 * fSlow183 * fSlow181 / fSlow186));
		let mut fSlow445: F32 = self.fConst4 * F32::min(fSlow19, self.fConst3);
		let mut fSlow446: F32 = 2.0 * fSlow442 * F32::cos(fSlow445);
		let mut fSlow447: F32 = 0.3 * (fSlow184 * F32::sin(fSlow445) * fSlow183 * fSlow181 / fSlow186);
		let mut fSlow448: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow184 * fSlow181 * fSlow257 / fSlow264));
		let mut fSlow449: F32 = 1.0 / fSlow188;
		let mut fSlow450: F32 = 1.0 / fSlow269;
		let mut fSlow451: F32 = 1.0 / fSlow277;
		let mut fSlow452: F32 = 1.0 / fSlow285;
		let mut fSlow453: F32 = 1.0 / fSlow293;
		let mut fSlow454: F32 = 1.0 / fSlow301;
		let mut fSlow455: F32 = 1.0 / fSlow309;
		let mut fSlow456: F32 = 1.0 / fSlow317;
		let mut fSlow457: F32 = 1.0 / fSlow325;
		let mut fSlow458: F32 = 1.0 / fSlow333;
		let mut fSlow459: F32 = 1.0 / fSlow341;
		let mut fSlow460: F32 = 1.0 / fSlow349;
		let mut fSlow461: F32 = 1.0 / fSlow357;
		let mut fSlow462: F32 = 1.0 / fSlow365;
		let mut fSlow463: F32 = 1.0 / fSlow373;
		let mut fSlow464: F32 = 1.0 / fSlow381;
		let mut fSlow465: F32 = 1.0 / fSlow389;
		let mut fSlow466: F32 = 1.0 / fSlow397;
		let mut fSlow467: F32 = 1.0 / fSlow405;
		let mut fSlow468: F32 = 1.0 / fSlow413;
		let mut fSlow469: F32 = 1.0 / fSlow421;
		let mut fSlow470: F32 = 1.0 / fSlow429;
		let mut fSlow471: F32 = 1.0 / fSlow437;
		let mut fSlow472: F32 = 1.0 / fSlow444;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0);
		for ((input0, input1), output0) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			let mut fTemp0: F32 = (i32::wrapping_sub(1, (fSlow0 > self.fVec0[1]) as i32)) as F32;
			let mut fTemp1: F32 = 1.0 - self.fConst1 * fTemp0;
			self.fRec1[0] = fSlow14 * fTemp1 + self.fConst1 * fTemp0 * self.fRec1[1];
			let mut fTemp2: F32 = self.fRec1[0] * self.fRec0[2];
			self.fRec2[0] = fSlow18 * fTemp1 + self.fConst1 * fTemp0 * self.fRec2[1];
			let mut fTemp3: F32 = F32::max(1e-06, self.fRec2[0]);
			let mut fTemp4: F32 = if iSlow16 != 0 {fSlow188 / fTemp3} else {1e+30};
			self.fRec3[0] = fSlow190 * fTemp1 + self.fConst1 * fTemp0 * self.fRec3[1];
			self.fRec4[0] = fSlow191 * fTemp1 + self.fConst1 * fTemp0 * self.fRec4[1];
			let mut fTemp5: F32 = *input0;
			let mut fTemp6: F32 = fTemp5 * self.fRec4[0] + self.fRec3[0] * self.fRec0[1];
			self.fRec0[0] = fTemp6 - (fTemp2 + 0.002 * (F32::max(0.0, fTemp6 - (fTemp2 + 2.5 * fTemp4)) + F32::min(0.0, fTemp6 + fTemp4 - fTemp2)));
			let mut fTemp7: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec0[0]) + 0.4 * F32::max(0.0, self.fRec0[0]));
			self.fRec6[0] = fSlow267 * fTemp1 + self.fConst1 * fTemp0 * self.fRec6[1];
			let mut fTemp8: F32 = self.fRec6[0] * self.fRec5[2];
			let mut fTemp9: F32 = if iSlow16 != 0 {fSlow269 / fTemp3} else {1e+30};
			self.fRec7[0] = fSlow271 * fTemp1 + self.fConst1 * fTemp0 * self.fRec7[1];
			self.fRec8[0] = fSlow272 * fTemp1 + self.fConst1 * fTemp0 * self.fRec8[1];
			let mut fTemp10: F32 = fTemp5 * self.fRec8[0] + self.fRec7[0] * self.fRec5[1];
			self.fRec5[0] = fTemp10 - (fTemp8 + 0.002 * (F32::max(0.0, fTemp10 - (fTemp8 + 2.5 * fTemp9)) + F32::min(0.0, fTemp10 + fTemp9 - fTemp8)));
			let mut fTemp11: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec5[0]) + 0.4 * F32::max(0.0, self.fRec5[0]));
			self.fRec10[0] = fSlow275 * fTemp1 + self.fConst1 * fTemp0 * self.fRec10[1];
			let mut fTemp12: F32 = self.fRec10[0] * self.fRec9[2];
			let mut fTemp13: F32 = if iSlow16 != 0 {fSlow277 / fTemp3} else {1e+30};
			self.fRec11[0] = fSlow279 * fTemp1 + self.fConst1 * fTemp0 * self.fRec11[1];
			self.fRec12[0] = fSlow280 * fTemp1 + self.fConst1 * fTemp0 * self.fRec12[1];
			let mut fTemp14: F32 = fTemp5 * self.fRec12[0] + self.fRec11[0] * self.fRec9[1];
			self.fRec9[0] = fTemp14 - (fTemp12 + 0.002 * (F32::max(0.0, fTemp14 - (fTemp12 + 2.5 * fTemp13)) + F32::min(0.0, fTemp14 + fTemp13 - fTemp12)));
			let mut fTemp15: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec9[0]) + 0.4 * F32::max(0.0, self.fRec9[0]));
			self.fRec14[0] = fSlow283 * fTemp1 + self.fConst1 * fTemp0 * self.fRec14[1];
			let mut fTemp16: F32 = self.fRec14[0] * self.fRec13[2];
			let mut fTemp17: F32 = if iSlow16 != 0 {fSlow285 / fTemp3} else {1e+30};
			self.fRec15[0] = fSlow287 * fTemp1 + self.fConst1 * fTemp0 * self.fRec15[1];
			self.fRec16[0] = fSlow288 * fTemp1 + self.fConst1 * fTemp0 * self.fRec16[1];
			let mut fTemp18: F32 = fTemp5 * self.fRec16[0] + self.fRec15[0] * self.fRec13[1];
			self.fRec13[0] = fTemp18 - (fTemp16 + 0.002 * (F32::max(0.0, fTemp18 - (fTemp16 + 2.5 * fTemp17)) + F32::min(0.0, fTemp18 + fTemp17 - fTemp16)));
			let mut fTemp19: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec13[0]) + 0.4 * F32::max(0.0, self.fRec13[0]));
			self.fRec18[0] = fSlow291 * fTemp1 + self.fConst1 * fTemp0 * self.fRec18[1];
			let mut fTemp20: F32 = self.fRec18[0] * self.fRec17[2];
			let mut fTemp21: F32 = if iSlow16 != 0 {fSlow293 / fTemp3} else {1e+30};
			self.fRec19[0] = fSlow295 * fTemp1 + self.fConst1 * fTemp0 * self.fRec19[1];
			self.fRec20[0] = fSlow296 * fTemp1 + self.fConst1 * fTemp0 * self.fRec20[1];
			let mut fTemp22: F32 = fTemp5 * self.fRec20[0] + self.fRec19[0] * self.fRec17[1];
			self.fRec17[0] = fTemp22 - (fTemp20 + 0.002 * (F32::max(0.0, fTemp22 - (fTemp20 + 2.5 * fTemp21)) + F32::min(0.0, fTemp22 + fTemp21 - fTemp20)));
			let mut fTemp23: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec17[0]) + 0.4 * F32::max(0.0, self.fRec17[0]));
			self.fRec22[0] = fSlow299 * fTemp1 + self.fConst1 * fTemp0 * self.fRec22[1];
			let mut fTemp24: F32 = self.fRec22[0] * self.fRec21[2];
			let mut fTemp25: F32 = if iSlow16 != 0 {fSlow301 / fTemp3} else {1e+30};
			self.fRec23[0] = fSlow303 * fTemp1 + self.fConst1 * fTemp0 * self.fRec23[1];
			self.fRec24[0] = fSlow304 * fTemp1 + self.fConst1 * fTemp0 * self.fRec24[1];
			let mut fTemp26: F32 = fTemp5 * self.fRec24[0] + self.fRec23[0] * self.fRec21[1];
			self.fRec21[0] = fTemp26 - (fTemp24 + 0.002 * (F32::max(0.0, fTemp26 - (fTemp24 + 2.5 * fTemp25)) + F32::min(0.0, fTemp26 + fTemp25 - fTemp24)));
			let mut fTemp27: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec21[0]) + 0.4 * F32::max(0.0, self.fRec21[0]));
			self.fRec26[0] = fSlow307 * fTemp1 + self.fConst1 * fTemp0 * self.fRec26[1];
			let mut fTemp28: F32 = self.fRec26[0] * self.fRec25[2];
			let mut fTemp29: F32 = if iSlow16 != 0 {fSlow309 / fTemp3} else {1e+30};
			self.fRec27[0] = fSlow311 * fTemp1 + self.fConst1 * fTemp0 * self.fRec27[1];
			self.fRec28[0] = fSlow312 * fTemp1 + self.fConst1 * fTemp0 * self.fRec28[1];
			let mut fTemp30: F32 = fTemp5 * self.fRec28[0] + self.fRec27[0] * self.fRec25[1];
			self.fRec25[0] = fTemp30 - (fTemp28 + 0.002 * (F32::max(0.0, fTemp30 - (fTemp28 + 2.5 * fTemp29)) + F32::min(0.0, fTemp30 + fTemp29 - fTemp28)));
			let mut fTemp31: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec25[0]) + 0.4 * F32::max(0.0, self.fRec25[0]));
			self.fRec30[0] = fSlow315 * fTemp1 + self.fConst1 * fTemp0 * self.fRec30[1];
			let mut fTemp32: F32 = self.fRec30[0] * self.fRec29[2];
			let mut fTemp33: F32 = if iSlow16 != 0 {fSlow317 / fTemp3} else {1e+30};
			self.fRec31[0] = fSlow319 * fTemp1 + self.fConst1 * fTemp0 * self.fRec31[1];
			self.fRec32[0] = fSlow320 * fTemp1 + self.fConst1 * fTemp0 * self.fRec32[1];
			let mut fTemp34: F32 = fTemp5 * self.fRec32[0] + self.fRec31[0] * self.fRec29[1];
			self.fRec29[0] = fTemp34 - (fTemp32 + 0.002 * (F32::max(0.0, fTemp34 - (fTemp32 + 2.5 * fTemp33)) + F32::min(0.0, fTemp34 + fTemp33 - fTemp32)));
			let mut fTemp35: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec29[0]) + 0.4 * F32::max(0.0, self.fRec29[0]));
			self.fRec34[0] = fSlow323 * fTemp1 + self.fConst1 * fTemp0 * self.fRec34[1];
			let mut fTemp36: F32 = self.fRec34[0] * self.fRec33[2];
			let mut fTemp37: F32 = if iSlow16 != 0 {fSlow325 / fTemp3} else {1e+30};
			self.fRec35[0] = fSlow327 * fTemp1 + self.fConst1 * fTemp0 * self.fRec35[1];
			self.fRec36[0] = fSlow328 * fTemp1 + self.fConst1 * fTemp0 * self.fRec36[1];
			let mut fTemp38: F32 = fTemp5 * self.fRec36[0] + self.fRec35[0] * self.fRec33[1];
			self.fRec33[0] = fTemp38 - (fTemp36 + 0.002 * (F32::max(0.0, fTemp38 - (fTemp36 + 2.5 * fTemp37)) + F32::min(0.0, fTemp38 + fTemp37 - fTemp36)));
			let mut fTemp39: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec33[0]) + 0.4 * F32::max(0.0, self.fRec33[0]));
			self.fRec38[0] = fSlow331 * fTemp1 + self.fConst1 * fTemp0 * self.fRec38[1];
			let mut fTemp40: F32 = self.fRec38[0] * self.fRec37[2];
			let mut fTemp41: F32 = if iSlow16 != 0 {fSlow333 / fTemp3} else {1e+30};
			self.fRec39[0] = fSlow335 * fTemp1 + self.fConst1 * fTemp0 * self.fRec39[1];
			self.fRec40[0] = fSlow336 * fTemp1 + self.fConst1 * fTemp0 * self.fRec40[1];
			let mut fTemp42: F32 = fTemp5 * self.fRec40[0] + self.fRec39[0] * self.fRec37[1];
			self.fRec37[0] = fTemp42 - (fTemp40 + 0.002 * (F32::max(0.0, fTemp42 - (fTemp40 + 2.5 * fTemp41)) + F32::min(0.0, fTemp42 + fTemp41 - fTemp40)));
			let mut fTemp43: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec37[0]) + 0.4 * F32::max(0.0, self.fRec37[0]));
			self.fRec42[0] = fSlow339 * fTemp1 + self.fConst1 * fTemp0 * self.fRec42[1];
			let mut fTemp44: F32 = self.fRec42[0] * self.fRec41[2];
			let mut fTemp45: F32 = if iSlow16 != 0 {fSlow341 / fTemp3} else {1e+30};
			self.fRec43[0] = fSlow343 * fTemp1 + self.fConst1 * fTemp0 * self.fRec43[1];
			self.fRec44[0] = fSlow344 * fTemp1 + self.fConst1 * fTemp0 * self.fRec44[1];
			let mut fTemp46: F32 = fTemp5 * self.fRec44[0] + self.fRec43[0] * self.fRec41[1];
			self.fRec41[0] = fTemp46 - (fTemp44 + 0.002 * (F32::max(0.0, fTemp46 - (fTemp44 + 2.5 * fTemp45)) + F32::min(0.0, fTemp46 + fTemp45 - fTemp44)));
			let mut fTemp47: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec41[0]) + 0.4 * F32::max(0.0, self.fRec41[0]));
			self.fRec46[0] = fSlow347 * fTemp1 + self.fConst1 * fTemp0 * self.fRec46[1];
			let mut fTemp48: F32 = self.fRec46[0] * self.fRec45[2];
			let mut fTemp49: F32 = if iSlow16 != 0 {fSlow349 / fTemp3} else {1e+30};
			self.fRec47[0] = fSlow351 * fTemp1 + self.fConst1 * fTemp0 * self.fRec47[1];
			self.fRec48[0] = fSlow352 * fTemp1 + self.fConst1 * fTemp0 * self.fRec48[1];
			let mut fTemp50: F32 = fTemp5 * self.fRec48[0] + self.fRec47[0] * self.fRec45[1];
			self.fRec45[0] = fTemp50 - (fTemp48 + 0.002 * (F32::max(0.0, fTemp50 - (fTemp48 + 2.5 * fTemp49)) + F32::min(0.0, fTemp50 + fTemp49 - fTemp48)));
			let mut fTemp51: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec45[0]) + 0.4 * F32::max(0.0, self.fRec45[0]));
			self.fRec50[0] = fSlow355 * fTemp1 + self.fConst1 * fTemp0 * self.fRec50[1];
			let mut fTemp52: F32 = self.fRec50[0] * self.fRec49[2];
			let mut fTemp53: F32 = if iSlow16 != 0 {fSlow357 / fTemp3} else {1e+30};
			self.fRec51[0] = fSlow359 * fTemp1 + self.fConst1 * fTemp0 * self.fRec51[1];
			self.fRec52[0] = fSlow360 * fTemp1 + self.fConst1 * fTemp0 * self.fRec52[1];
			let mut fTemp54: F32 = fTemp5 * self.fRec52[0] + self.fRec51[0] * self.fRec49[1];
			self.fRec49[0] = fTemp54 - (fTemp52 + 0.002 * (F32::max(0.0, fTemp54 - (fTemp52 + 2.5 * fTemp53)) + F32::min(0.0, fTemp54 + fTemp53 - fTemp52)));
			let mut fTemp55: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec49[0]) + 0.4 * F32::max(0.0, self.fRec49[0]));
			self.fRec54[0] = fSlow363 * fTemp1 + self.fConst1 * fTemp0 * self.fRec54[1];
			let mut fTemp56: F32 = self.fRec54[0] * self.fRec53[2];
			let mut fTemp57: F32 = if iSlow16 != 0 {fSlow365 / fTemp3} else {1e+30};
			self.fRec55[0] = fSlow367 * fTemp1 + self.fConst1 * fTemp0 * self.fRec55[1];
			self.fRec56[0] = fSlow368 * fTemp1 + self.fConst1 * fTemp0 * self.fRec56[1];
			let mut fTemp58: F32 = fTemp5 * self.fRec56[0] + self.fRec55[0] * self.fRec53[1];
			self.fRec53[0] = fTemp58 - (fTemp56 + 0.002 * (F32::max(0.0, fTemp58 - (fTemp56 + 2.5 * fTemp57)) + F32::min(0.0, fTemp58 + fTemp57 - fTemp56)));
			let mut fTemp59: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec53[0]) + 0.4 * F32::max(0.0, self.fRec53[0]));
			self.fRec58[0] = fSlow371 * fTemp1 + self.fConst1 * fTemp0 * self.fRec58[1];
			let mut fTemp60: F32 = self.fRec58[0] * self.fRec57[2];
			let mut fTemp61: F32 = if iSlow16 != 0 {fSlow373 / fTemp3} else {1e+30};
			self.fRec59[0] = fSlow375 * fTemp1 + self.fConst1 * fTemp0 * self.fRec59[1];
			self.fRec60[0] = fSlow376 * fTemp1 + self.fConst1 * fTemp0 * self.fRec60[1];
			let mut fTemp62: F32 = fTemp5 * self.fRec60[0] + self.fRec59[0] * self.fRec57[1];
			self.fRec57[0] = fTemp62 - (fTemp60 + 0.002 * (F32::max(0.0, fTemp62 - (fTemp60 + 2.5 * fTemp61)) + F32::min(0.0, fTemp62 + fTemp61 - fTemp60)));
			let mut fTemp63: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec57[0]) + 0.4 * F32::max(0.0, self.fRec57[0]));
			self.fRec62[0] = fSlow379 * fTemp1 + self.fConst1 * fTemp0 * self.fRec62[1];
			let mut fTemp64: F32 = self.fRec62[0] * self.fRec61[2];
			let mut fTemp65: F32 = if iSlow16 != 0 {fSlow381 / fTemp3} else {1e+30};
			self.fRec63[0] = fSlow383 * fTemp1 + self.fConst1 * fTemp0 * self.fRec63[1];
			self.fRec64[0] = fSlow384 * fTemp1 + self.fConst1 * fTemp0 * self.fRec64[1];
			let mut fTemp66: F32 = fTemp5 * self.fRec64[0] + self.fRec63[0] * self.fRec61[1];
			self.fRec61[0] = fTemp66 - (fTemp64 + 0.002 * (F32::max(0.0, fTemp66 - (fTemp64 + 2.5 * fTemp65)) + F32::min(0.0, fTemp66 + fTemp65 - fTemp64)));
			let mut fTemp67: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec61[0]) + 0.4 * F32::max(0.0, self.fRec61[0]));
			self.fRec66[0] = fSlow387 * fTemp1 + self.fConst1 * fTemp0 * self.fRec66[1];
			let mut fTemp68: F32 = self.fRec66[0] * self.fRec65[2];
			let mut fTemp69: F32 = if iSlow16 != 0 {fSlow389 / fTemp3} else {1e+30};
			self.fRec67[0] = fSlow391 * fTemp1 + self.fConst1 * fTemp0 * self.fRec67[1];
			self.fRec68[0] = fSlow392 * fTemp1 + self.fConst1 * fTemp0 * self.fRec68[1];
			let mut fTemp70: F32 = fTemp5 * self.fRec68[0] + self.fRec67[0] * self.fRec65[1];
			self.fRec65[0] = fTemp70 - (fTemp68 + 0.002 * (F32::max(0.0, fTemp70 - (fTemp68 + 2.5 * fTemp69)) + F32::min(0.0, fTemp70 + fTemp69 - fTemp68)));
			let mut fTemp71: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec65[0]) + 0.4 * F32::max(0.0, self.fRec65[0]));
			self.fRec70[0] = fSlow395 * fTemp1 + self.fConst1 * fTemp0 * self.fRec70[1];
			let mut fTemp72: F32 = self.fRec70[0] * self.fRec69[2];
			let mut fTemp73: F32 = if iSlow16 != 0 {fSlow397 / fTemp3} else {1e+30};
			self.fRec71[0] = fSlow399 * fTemp1 + self.fConst1 * fTemp0 * self.fRec71[1];
			self.fRec72[0] = fSlow400 * fTemp1 + self.fConst1 * fTemp0 * self.fRec72[1];
			let mut fTemp74: F32 = fTemp5 * self.fRec72[0] + self.fRec71[0] * self.fRec69[1];
			self.fRec69[0] = fTemp74 - (fTemp72 + 0.002 * (F32::max(0.0, fTemp74 - (fTemp72 + 2.5 * fTemp73)) + F32::min(0.0, fTemp74 + fTemp73 - fTemp72)));
			let mut fTemp75: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec69[0]) + 0.4 * F32::max(0.0, self.fRec69[0]));
			self.fRec74[0] = fSlow403 * fTemp1 + self.fConst1 * fTemp0 * self.fRec74[1];
			let mut fTemp76: F32 = self.fRec74[0] * self.fRec73[2];
			let mut fTemp77: F32 = if iSlow16 != 0 {fSlow405 / fTemp3} else {1e+30};
			self.fRec75[0] = fSlow407 * fTemp1 + self.fConst1 * fTemp0 * self.fRec75[1];
			self.fRec76[0] = fSlow408 * fTemp1 + self.fConst1 * fTemp0 * self.fRec76[1];
			let mut fTemp78: F32 = fTemp5 * self.fRec76[0] + self.fRec75[0] * self.fRec73[1];
			self.fRec73[0] = fTemp78 - (fTemp76 + 0.002 * (F32::max(0.0, fTemp78 - (fTemp76 + 2.5 * fTemp77)) + F32::min(0.0, fTemp78 + fTemp77 - fTemp76)));
			let mut fTemp79: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec73[0]) + 0.4 * F32::max(0.0, self.fRec73[0]));
			self.fRec78[0] = fSlow411 * fTemp1 + self.fConst1 * fTemp0 * self.fRec78[1];
			let mut fTemp80: F32 = self.fRec78[0] * self.fRec77[2];
			let mut fTemp81: F32 = if iSlow16 != 0 {fSlow413 / fTemp3} else {1e+30};
			self.fRec79[0] = fSlow415 * fTemp1 + self.fConst1 * fTemp0 * self.fRec79[1];
			self.fRec80[0] = fSlow416 * fTemp1 + self.fConst1 * fTemp0 * self.fRec80[1];
			let mut fTemp82: F32 = fTemp5 * self.fRec80[0] + self.fRec79[0] * self.fRec77[1];
			self.fRec77[0] = fTemp82 - (fTemp80 + 0.002 * (F32::max(0.0, fTemp82 - (fTemp80 + 2.5 * fTemp81)) + F32::min(0.0, fTemp82 + fTemp81 - fTemp80)));
			let mut fTemp83: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec77[0]) + 0.4 * F32::max(0.0, self.fRec77[0]));
			self.fRec82[0] = fSlow419 * fTemp1 + self.fConst1 * fTemp0 * self.fRec82[1];
			let mut fTemp84: F32 = self.fRec82[0] * self.fRec81[2];
			let mut fTemp85: F32 = if iSlow16 != 0 {fSlow421 / fTemp3} else {1e+30};
			self.fRec83[0] = fSlow423 * fTemp1 + self.fConst1 * fTemp0 * self.fRec83[1];
			self.fRec84[0] = fSlow424 * fTemp1 + self.fConst1 * fTemp0 * self.fRec84[1];
			let mut fTemp86: F32 = fTemp5 * self.fRec84[0] + self.fRec83[0] * self.fRec81[1];
			self.fRec81[0] = fTemp86 - (fTemp84 + 0.002 * (F32::max(0.0, fTemp86 - (fTemp84 + 2.5 * fTemp85)) + F32::min(0.0, fTemp86 + fTemp85 - fTemp84)));
			let mut fTemp87: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec81[0]) + 0.4 * F32::max(0.0, self.fRec81[0]));
			self.fRec86[0] = fSlow427 * fTemp1 + self.fConst1 * fTemp0 * self.fRec86[1];
			let mut fTemp88: F32 = self.fRec86[0] * self.fRec85[2];
			let mut fTemp89: F32 = if iSlow16 != 0 {fSlow429 / fTemp3} else {1e+30};
			self.fRec87[0] = fSlow431 * fTemp1 + self.fConst1 * fTemp0 * self.fRec87[1];
			self.fRec88[0] = fSlow432 * fTemp1 + self.fConst1 * fTemp0 * self.fRec88[1];
			let mut fTemp90: F32 = fTemp5 * self.fRec88[0] + self.fRec87[0] * self.fRec85[1];
			self.fRec85[0] = fTemp90 - (fTemp88 + 0.002 * (F32::max(0.0, fTemp90 - (fTemp88 + 2.5 * fTemp89)) + F32::min(0.0, fTemp90 + fTemp89 - fTemp88)));
			let mut fTemp91: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec85[0]) + 0.4 * F32::max(0.0, self.fRec85[0]));
			self.fRec90[0] = fSlow435 * fTemp1 + self.fConst1 * fTemp0 * self.fRec90[1];
			let mut fTemp92: F32 = self.fRec90[0] * self.fRec89[2];
			let mut fTemp93: F32 = if iSlow16 != 0 {fSlow437 / fTemp3} else {1e+30};
			self.fRec91[0] = fSlow439 * fTemp1 + self.fConst1 * fTemp0 * self.fRec91[1];
			self.fRec92[0] = fSlow440 * fTemp1 + self.fConst1 * fTemp0 * self.fRec92[1];
			let mut fTemp94: F32 = fTemp5 * self.fRec92[0] + self.fRec91[0] * self.fRec89[1];
			self.fRec89[0] = fTemp94 - (fTemp92 + 0.002 * (F32::max(0.0, fTemp94 - (fTemp92 + 2.5 * fTemp93)) + F32::min(0.0, fTemp94 + fTemp93 - fTemp92)));
			let mut fTemp95: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec89[0]) + 0.4 * F32::max(0.0, self.fRec89[0]));
			self.fRec94[0] = fSlow443 * fTemp1 + self.fConst1 * fTemp0 * self.fRec94[1];
			let mut fTemp96: F32 = self.fRec94[0] * self.fRec93[2];
			let mut fTemp97: F32 = if iSlow16 != 0 {fSlow444 / fTemp3} else {1e+30};
			self.fRec95[0] = fSlow446 * fTemp1 + self.fConst1 * fTemp0 * self.fRec95[1];
			self.fRec96[0] = fSlow447 * fTemp1 + self.fConst1 * fTemp0 * self.fRec96[1];
			let mut fTemp98: F32 = fTemp5 * self.fRec96[0] + self.fRec95[0] * self.fRec93[1];
			self.fRec93[0] = fTemp98 - (fTemp96 + 0.002 * (F32::max(0.0, fTemp98 - (fTemp96 + 2.5 * fTemp97)) + F32::min(0.0, fTemp98 + fTemp97 - fTemp96)));
			let mut fTemp99: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec93[0]) + 0.4 * F32::max(0.0, self.fRec93[0]));
			let mut fTemp100: F32 = self.fRec93[0] / (fSlow448 * fTemp99 + 1.0) + self.fRec89[0] / (fSlow441 * fTemp95 + 1.0) + self.fRec85[0] / (fSlow433 * fTemp91 + 1.0) + self.fRec81[0] / (fSlow425 * fTemp87 + 1.0) + self.fRec77[0] / (fSlow417 * fTemp83 + 1.0) + self.fRec73[0] / (fSlow409 * fTemp79 + 1.0) + self.fRec69[0] / (fSlow401 * fTemp75 + 1.0) + self.fRec65[0] / (fSlow393 * fTemp71 + 1.0) + self.fRec61[0] / (fSlow385 * fTemp67 + 1.0) + self.fRec57[0] / (fSlow377 * fTemp63 + 1.0) + self.fRec53[0] / (fSlow369 * fTemp59 + 1.0) + self.fRec49[0] / (fSlow361 * fTemp55 + 1.0) + self.fRec45[0] / (fSlow353 * fTemp51 + 1.0) + self.fRec41[0] / (fSlow345 * fTemp47 + 1.0) + self.fRec37[0] / (fSlow337 * fTemp43 + 1.0) + self.fRec33[0] / (fSlow329 * fTemp39 + 1.0) + self.fRec29[0] / (fSlow321 * fTemp35 + 1.0) + self.fRec25[0] / (fSlow313 * fTemp31 + 1.0) + self.fRec21[0] / (fSlow305 * fTemp27 + 1.0) + self.fRec17[0] / (fSlow297 * fTemp23 + 1.0) + self.fRec13[0] / (fSlow289 * fTemp19 + 1.0) + self.fRec9[0] / (fSlow281 * fTemp15 + 1.0) + self.fRec5[0] / (fSlow273 * fTemp11 + 1.0) + self.fRec0[0] / (fSlow265 * fTemp7 + 1.0);
			let mut fTemp101: F32 = self.fRec93[0] / (fSlow472 * fTemp99 + 1.0) + self.fRec89[0] / (fSlow471 * fTemp95 + 1.0) + self.fRec85[0] / (fSlow470 * fTemp91 + 1.0) + self.fRec81[0] / (fSlow469 * fTemp87 + 1.0) + self.fRec77[0] / (fSlow468 * fTemp83 + 1.0) + self.fRec73[0] / (fSlow467 * fTemp79 + 1.0) + self.fRec69[0] / (fSlow466 * fTemp75 + 1.0) + self.fRec65[0] / (fSlow465 * fTemp71 + 1.0) + self.fRec61[0] / (fSlow464 * fTemp67 + 1.0) + self.fRec57[0] / (fSlow463 * fTemp63 + 1.0) + self.fRec53[0] / (fSlow462 * fTemp59 + 1.0) + self.fRec49[0] / (fSlow461 * fTemp55 + 1.0) + self.fRec45[0] / (fSlow460 * fTemp51 + 1.0) + self.fRec41[0] / (fSlow459 * fTemp47 + 1.0) + self.fRec37[0] / (fSlow458 * fTemp43 + 1.0) + self.fRec33[0] / (fSlow457 * fTemp39 + 1.0) + self.fRec29[0] / (fSlow456 * fTemp35 + 1.0) + self.fRec25[0] / (fSlow455 * fTemp31 + 1.0) + self.fRec21[0] / (fSlow454 * fTemp27 + 1.0) + self.fRec17[0] / (fSlow453 * fTemp23 + 1.0) + self.fRec13[0] / (fSlow452 * fTemp19 + 1.0) + self.fRec9[0] / (fSlow451 * fTemp15 + 1.0) + self.fRec5[0] / (fSlow450 * fTemp11 + 1.0) + self.fRec0[0] / (fSlow449 * fTemp7 + 1.0);
			self.fVec1[0] = fTemp101;
			self.fRec97[0] = -(self.fConst7 * (self.fConst6 * self.fRec97[1] - self.fConst5 * (fTemp101 - self.fVec1[1])));
			*output0 = *input1 * F32::powf(self.fRec2[0] + 1.0, 0.6) * (fTemp100 + fSlow17 * (self.fRec97[0] - fTemp100));
			self.fVec0[1] = self.fVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec0[2] = self.fRec0[1];
			self.fRec0[1] = self.fRec0[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec5[2] = self.fRec5[1];
			self.fRec5[1] = self.fRec5[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec9[2] = self.fRec9[1];
			self.fRec9[1] = self.fRec9[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec17[2] = self.fRec17[1];
			self.fRec17[1] = self.fRec17[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec21[2] = self.fRec21[1];
			self.fRec21[1] = self.fRec21[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec25[2] = self.fRec25[1];
			self.fRec25[1] = self.fRec25[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec29[2] = self.fRec29[1];
			self.fRec29[1] = self.fRec29[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec33[2] = self.fRec33[1];
			self.fRec33[1] = self.fRec33[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec37[2] = self.fRec37[1];
			self.fRec37[1] = self.fRec37[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec41[2] = self.fRec41[1];
			self.fRec41[1] = self.fRec41[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec45[2] = self.fRec45[1];
			self.fRec45[1] = self.fRec45[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec49[2] = self.fRec49[1];
			self.fRec49[1] = self.fRec49[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec53[2] = self.fRec53[1];
			self.fRec53[1] = self.fRec53[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec57[2] = self.fRec57[1];
			self.fRec57[1] = self.fRec57[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec61[2] = self.fRec61[1];
			self.fRec61[1] = self.fRec61[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec65[2] = self.fRec65[1];
			self.fRec65[1] = self.fRec65[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec69[2] = self.fRec69[1];
			self.fRec69[1] = self.fRec69[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec73[2] = self.fRec73[1];
			self.fRec73[1] = self.fRec73[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec79[1] = self.fRec79[0];
			self.fRec80[1] = self.fRec80[0];
			self.fRec77[2] = self.fRec77[1];
			self.fRec77[1] = self.fRec77[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec81[2] = self.fRec81[1];
			self.fRec81[1] = self.fRec81[0];
			self.fRec86[1] = self.fRec86[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec85[2] = self.fRec85[1];
			self.fRec85[1] = self.fRec85[0];
			self.fRec90[1] = self.fRec90[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec89[2] = self.fRec89[1];
			self.fRec89[1] = self.fRec89[0];
			self.fRec94[1] = self.fRec94[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec93[2] = self.fRec93[1];
			self.fRec93[1] = self.fRec93[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec97[1] = self.fRec97[0];
		}
	}

}

