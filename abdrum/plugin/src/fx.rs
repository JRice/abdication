/* ------------------------------------------------------------
name: "ABDrumFX"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn FxDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn FxDsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}
fn FxDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct FxDsp {
	iVec0: [i32;2],
	fButton0: F32,
	fVec1: [F32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fHslider0: F32,
	fRec0: [F32;2],
	fHslider1: F32,
	fRec1: [F32;2],
	fHslider2: F32,
	fRec2: [F32;2],
	fHslider3: F32,
	fRec3: [F32;2],
	iRec4: [i32;2],
	fConst2: F32,
	fHslider4: F32,
	fRec5: [F32;2],
	fHslider5: F32,
	fRec6: [F32;2],
	fHslider6: F32,
	fRec7: [F32;2],
	iRec8: [i32;2],
	fHslider7: F32,
	fRec9: [F32;2],
	fHslider8: F32,
	fRec10: [F32;2],
	fHslider9: F32,
	fRec11: [F32;2],
	fHslider10: F32,
	fRec12: [F32;2],
	fEntry0: F32,
	fHslider11: F32,
	fRec14: [F32;2],
	fHslider12: F32,
	fRec15: [F32;2],
	fHslider13: F32,
	fRec16: [F32;2],
	fHslider14: F32,
	fRec17: [F32;2],
	fHslider15: F32,
	fRec18: [F32;2],
	fHslider16: F32,
	fRec19: [F32;2],
	fHslider17: F32,
	fRec20: [F32;2],
	fHslider18: F32,
	fRec21: [F32;2],
	fHslider19: F32,
	fRec22: [F32;2],
	fHslider20: F32,
	fRec23: [F32;2],
	fConst3: F32,
	fConst4: F32,
	fRec25: [F32;2],
	fRec24: [F32;2],
	fTempPerm0: F32,
	fTempPerm1: F32,
	fConst5: F32,
	fTempPerm2: F32,
	fConst6: F32,
	fTempPerm3: F32,
	fTempPerm4: F32,
	fTempPerm5: F32,
	fTempPerm6: F32,
	fTempPerm7: F32,
	fTempPerm8: F32,
	fTempPerm9: F32,
	fTempPerm10: F32,
	fTempPerm11: F32,
	fTempPerm12: F32,
	fTempPerm13: F32,
	fTempPerm14: F32,
	fTempPerm15: F32,
	fTempPerm16: F32,
	fTempPerm17: F32,
	fTempPerm18: F32,
	fTempPerm19: F32,
	fTempPerm20: F32,
	fHslider21: F32,
	fRec36: [F32;2],
	fHslider22: F32,
	fRec37: [F32;2],
	fTempPerm21: F32,
	fTempPerm22: F32,
	fTempPerm23: F32,
	fTempPerm24: F32,
	fTempPerm25: F32,
	fTempPerm26: F32,
	fTempPerm27: F32,
	fTempPerm28: F32,
	fTempPerm29: F32,
	fTempPerm30: F32,
	fTempPerm31: F32,
	fRec26: [F32;2],
	fTempPerm32: F32,
	fTempPerm33: F32,
	fTempPerm34: F32,
	fRec27: [F32;2],
	fTempPerm35: F32,
	fTempPerm36: F32,
	fTempPerm37: F32,
	fRec28: [F32;2],
	fTempPerm38: F32,
	fTempPerm39: F32,
	fRec29: [F32;2],
	fRec30: [F32;2],
	fRec31: [F32;2],
	fRec32: [F32;2],
	fRec33: [F32;2],
	fRec34: [F32;2],
	fEntry1: F32,
	fTempPerm40: F32,
	fVec2: [F32;2],
	fEntry2: F32,
	fConst7: F32,
	fRec42: [F32;2],
	fVec3: [F32;2],
	fRec41: [F32;2],
	fVec4: [F32;2],
	fTempPerm41: F32,
	fConst8: F32,
	fRec43: [F32;2],
	fTempPerm42: F32,
	fTempPerm43: F32,
	fTempPerm44: F32,
	fTempPerm45: F32,
	fTempPerm46: F32,
	fTempPerm47: F32,
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	fVec5: [F32;2],
	fRec40: [F32;2],
	fVec6: [F32;2],
	fRec39: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fRec38: [F32;2],
	fVec7: [F32;2],
	fConst12: F32,
	fConst13: F32,
	fConst14: F32,
	fRec44: [F32;2],
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fRec45: [F32;2],
	fHslider23: F32,
	fRec46: [F32;2],
	IOTA0: i32,
	fConst18: F32,
	fConst19: F32,
	fConst20: F32,
	fConst21: F32,
	fVec8: [F32;2],
	fRec52: [F32;2],
	fRec51: [F32;2],
	fRec50: [F32;2],
	fRec49: [F32;2],
	fConst23: F32,
	fConst24: F32,
	fRec48: [F32;2],
	fRec47: [F32;8192],
	fTempPerm51: F32,
	fTempPerm52: F32,
	fTempPerm53: F32,
	fTempPerm54: F32,
	fTempPerm55: F32,
	fTempPerm56: F32,
	fTempPerm57: F32,
	fTempPerm58: F32,
	fTempPerm59: F32,
	fTempPerm60: F32,
	fTempPerm61: F32,
	fTempPerm62: F32,
	fTempPerm63: F32,
	fTempPerm64: F32,
	fTempPerm65: F32,
	fTempPerm66: F32,
	fTempPerm67: F32,
	fTempPerm68: F32,
	fTempPerm69: F32,
	fTempPerm70: F32,
	fTempPerm71: F32,
	fTempPerm72: F32,
	fTempPerm73: F32,
	fTempPerm74: F32,
	fTempPerm75: F32,
	fTempPerm76: F32,
	fTempPerm77: F32,
	fTempPerm78: F32,
	fTempPerm79: F32,
	fTempPerm80: F32,
	fTempPerm81: F32,
	fRec53: [F32;2],
	fTempPerm82: F32,
	fTempPerm83: F32,
	fTempPerm84: F32,
	fRec54: [F32;2],
	fTempPerm85: F32,
	fTempPerm86: F32,
	fTempPerm87: F32,
	fRec55: [F32;2],
	fTempPerm88: F32,
	fTempPerm89: F32,
	fRec56: [F32;2],
	fRec57: [F32;2],
	fRec58: [F32;2],
	fRec59: [F32;2],
	fRec60: [F32;2],
	fRec61: [F32;2],
	fTempPerm90: F32,
	fEntry3: F32,
	fConst25: F32,
	fRec13: [F32;2],
	fTempPerm91: F32,
	fTempPerm92: F32,
	fConst26: F32,
	fTempPerm93: F32,
	fRec63: [F32;2],
	fTempPerm94: F32,
}

impl FaustDsp for FxDsp {
	type T = F32;
		
	fn new() -> FxDsp { 
		FxDsp {
			iVec0: [0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fHslider0: 0.0,
			fRec0: [0.0;2],
			fHslider1: 0.0,
			fRec1: [0.0;2],
			fHslider2: 0.0,
			fRec2: [0.0;2],
			fHslider3: 0.0,
			fRec3: [0.0;2],
			iRec4: [0;2],
			fConst2: 0.0,
			fHslider4: 0.0,
			fRec5: [0.0;2],
			fHslider5: 0.0,
			fRec6: [0.0;2],
			fHslider6: 0.0,
			fRec7: [0.0;2],
			iRec8: [0;2],
			fHslider7: 0.0,
			fRec9: [0.0;2],
			fHslider8: 0.0,
			fRec10: [0.0;2],
			fHslider9: 0.0,
			fRec11: [0.0;2],
			fHslider10: 0.0,
			fRec12: [0.0;2],
			fEntry0: 0.0,
			fHslider11: 0.0,
			fRec14: [0.0;2],
			fHslider12: 0.0,
			fRec15: [0.0;2],
			fHslider13: 0.0,
			fRec16: [0.0;2],
			fHslider14: 0.0,
			fRec17: [0.0;2],
			fHslider15: 0.0,
			fRec18: [0.0;2],
			fHslider16: 0.0,
			fRec19: [0.0;2],
			fHslider17: 0.0,
			fRec20: [0.0;2],
			fHslider18: 0.0,
			fRec21: [0.0;2],
			fHslider19: 0.0,
			fRec22: [0.0;2],
			fHslider20: 0.0,
			fRec23: [0.0;2],
			fConst3: 0.0,
			fConst4: 0.0,
			fRec25: [0.0;2],
			fRec24: [0.0;2],
			fTempPerm0: 0.0,
			fTempPerm1: 0.0,
			fConst5: 0.0,
			fTempPerm2: 0.0,
			fConst6: 0.0,
			fTempPerm3: 0.0,
			fTempPerm4: 0.0,
			fTempPerm5: 0.0,
			fTempPerm6: 0.0,
			fTempPerm7: 0.0,
			fTempPerm8: 0.0,
			fTempPerm9: 0.0,
			fTempPerm10: 0.0,
			fTempPerm11: 0.0,
			fTempPerm12: 0.0,
			fTempPerm13: 0.0,
			fTempPerm14: 0.0,
			fTempPerm15: 0.0,
			fTempPerm16: 0.0,
			fTempPerm17: 0.0,
			fTempPerm18: 0.0,
			fTempPerm19: 0.0,
			fTempPerm20: 0.0,
			fHslider21: 0.0,
			fRec36: [0.0;2],
			fHslider22: 0.0,
			fRec37: [0.0;2],
			fTempPerm21: 0.0,
			fTempPerm22: 0.0,
			fTempPerm23: 0.0,
			fTempPerm24: 0.0,
			fTempPerm25: 0.0,
			fTempPerm26: 0.0,
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fTempPerm29: 0.0,
			fTempPerm30: 0.0,
			fTempPerm31: 0.0,
			fRec26: [0.0;2],
			fTempPerm32: 0.0,
			fTempPerm33: 0.0,
			fTempPerm34: 0.0,
			fRec27: [0.0;2],
			fTempPerm35: 0.0,
			fTempPerm36: 0.0,
			fTempPerm37: 0.0,
			fRec28: [0.0;2],
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fRec29: [0.0;2],
			fRec30: [0.0;2],
			fRec31: [0.0;2],
			fRec32: [0.0;2],
			fRec33: [0.0;2],
			fRec34: [0.0;2],
			fEntry1: 0.0,
			fTempPerm40: 0.0,
			fVec2: [0.0;2],
			fEntry2: 0.0,
			fConst7: 0.0,
			fRec42: [0.0;2],
			fVec3: [0.0;2],
			fRec41: [0.0;2],
			fVec4: [0.0;2],
			fTempPerm41: 0.0,
			fConst8: 0.0,
			fRec43: [0.0;2],
			fTempPerm42: 0.0,
			fTempPerm43: 0.0,
			fTempPerm44: 0.0,
			fTempPerm45: 0.0,
			fTempPerm46: 0.0,
			fTempPerm47: 0.0,
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			fVec5: [0.0;2],
			fRec40: [0.0;2],
			fVec6: [0.0;2],
			fRec39: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fRec38: [0.0;2],
			fVec7: [0.0;2],
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fRec44: [0.0;2],
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fRec45: [0.0;2],
			fHslider23: 0.0,
			fRec46: [0.0;2],
			IOTA0: 0,
			fConst18: 0.0,
			fConst19: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fVec8: [0.0;2],
			fRec52: [0.0;2],
			fRec51: [0.0;2],
			fRec50: [0.0;2],
			fRec49: [0.0;2],
			fConst23: 0.0,
			fConst24: 0.0,
			fRec48: [0.0;2],
			fRec47: [0.0;8192],
			fTempPerm51: 0.0,
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fTempPerm54: 0.0,
			fTempPerm55: 0.0,
			fTempPerm56: 0.0,
			fTempPerm57: 0.0,
			fTempPerm58: 0.0,
			fTempPerm59: 0.0,
			fTempPerm60: 0.0,
			fTempPerm61: 0.0,
			fTempPerm62: 0.0,
			fTempPerm63: 0.0,
			fTempPerm64: 0.0,
			fTempPerm65: 0.0,
			fTempPerm66: 0.0,
			fTempPerm67: 0.0,
			fTempPerm68: 0.0,
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			fTempPerm71: 0.0,
			fTempPerm72: 0.0,
			fTempPerm73: 0.0,
			fTempPerm74: 0.0,
			fTempPerm75: 0.0,
			fTempPerm76: 0.0,
			fTempPerm77: 0.0,
			fTempPerm78: 0.0,
			fTempPerm79: 0.0,
			fTempPerm80: 0.0,
			fTempPerm81: 0.0,
			fRec53: [0.0;2],
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			fTempPerm84: 0.0,
			fRec54: [0.0;2],
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			fTempPerm87: 0.0,
			fRec55: [0.0;2],
			fTempPerm88: 0.0,
			fTempPerm89: 0.0,
			fRec56: [0.0;2],
			fRec57: [0.0;2],
			fRec58: [0.0;2],
			fRec59: [0.0;2],
			fRec60: [0.0;2],
			fRec61: [0.0;2],
			fTempPerm90: 0.0,
			fEntry3: 0.0,
			fConst25: 0.0,
			fRec13: [0.0;2],
			fTempPerm91: 0.0,
			fTempPerm92: 0.0,
			fConst26: 0.0,
			fTempPerm93: 0.0,
			fRec63: [0.0;2],
			fTempPerm94: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn FxDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("drum.lib/name", r"abdrum-layers");
		m.declare("drum.lib/version", r"0.1");
		m.declare("filename", r"plugin-fx.dsp");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("fx.lib/name", r"abdrum-fx");
		m.declare("fx.lib/version", r"0.1");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"ABDrumFX");
		m.declare("oscillators.lib/lf_sawpos:author", r"Bart Brouns, revised by Stéphane Letz");
		m.declare("oscillators.lib/lf_sawpos:licence", r"STK-4.3");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/version", r"1.5.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.5.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 1;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.0;
		self.fHslider2 = 3e+02;
		self.fHslider3 = 0.0;
		self.fHslider4 = 0.5;
		self.fHslider5 = 1.0;
		self.fHslider6 = 0.5;
		self.fHslider7 = 0.0;
		self.fHslider8 = 2e+03;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.0;
		self.fEntry0 = 0.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 2e+04;
		self.fHslider13 = 0.0;
		self.fHslider14 = 0.0;
		self.fHslider15 = 0.0;
		self.fHslider16 = 0.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 0.0;
		self.fHslider19 = 0.0;
		self.fHslider20 = 0.0;
		self.fHslider21 = 0.0;
		self.fHslider22 = 0.0;
		self.fEntry1 = 0.0;
		self.fEntry2 = 0.0;
		self.fHslider23 = 5e+01;
		self.fEntry3 = 2.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fVec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec0[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec1[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec2[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec3[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.iRec4[l6 as usize] = 0;
		}
		for l7 in 0..2 {
			self.fRec5[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec6[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec7[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.iRec8[l10 as usize] = 0;
		}
		for l11 in 0..2 {
			self.fRec9[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec10[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec11[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec12[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec14[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec15[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec16[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec17[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec18[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec19[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec20[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec21[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec22[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec23[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec25[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec24[l26 as usize] = 0.0;
		}
		self.fTempPerm0 = 0.0;
		self.fTempPerm1 = 0.0;
		self.fTempPerm2 = 0.0;
		self.fTempPerm3 = 0.0;
		self.fTempPerm4 = 0.0;
		self.fTempPerm5 = 0.0;
		self.fTempPerm6 = 0.0;
		self.fTempPerm7 = 0.0;
		self.fTempPerm8 = 0.0;
		self.fTempPerm9 = 0.0;
		self.fTempPerm10 = 0.0;
		self.fTempPerm11 = 0.0;
		self.fTempPerm12 = 0.0;
		self.fTempPerm13 = 0.0;
		self.fTempPerm14 = 0.0;
		self.fTempPerm15 = 0.0;
		self.fTempPerm16 = 0.0;
		self.fTempPerm17 = 0.0;
		self.fTempPerm18 = 0.0;
		self.fTempPerm19 = 0.0;
		self.fTempPerm20 = 0.0;
		for l27 in 0..2 {
			self.fRec36[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec37[l28 as usize] = 0.0;
		}
		self.fTempPerm21 = 0.0;
		self.fTempPerm22 = 0.0;
		self.fTempPerm23 = 0.0;
		self.fTempPerm24 = 0.0;
		self.fTempPerm25 = 0.0;
		self.fTempPerm26 = 0.0;
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		self.fTempPerm29 = 0.0;
		self.fTempPerm30 = 0.0;
		self.fTempPerm31 = 0.0;
		for l29 in 0..2 {
			self.fRec26[l29 as usize] = 0.0;
		}
		self.fTempPerm32 = 0.0;
		self.fTempPerm33 = 0.0;
		self.fTempPerm34 = 0.0;
		for l30 in 0..2 {
			self.fRec27[l30 as usize] = 0.0;
		}
		self.fTempPerm35 = 0.0;
		self.fTempPerm36 = 0.0;
		self.fTempPerm37 = 0.0;
		for l31 in 0..2 {
			self.fRec28[l31 as usize] = 0.0;
		}
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		for l32 in 0..2 {
			self.fRec29[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec30[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec31[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec32[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec33[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec34[l37 as usize] = 0.0;
		}
		self.fTempPerm40 = 0.0;
		for l38 in 0..2 {
			self.fVec2[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec42[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec3[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec41[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec4[l42 as usize] = 0.0;
		}
		self.fTempPerm41 = 0.0;
		for l43 in 0..2 {
			self.fRec43[l43 as usize] = 0.0;
		}
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		for l44 in 0..2 {
			self.fVec5[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec40[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec6[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec39[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec38[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fVec7[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec44[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec45[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec46[l52 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l53 in 0..2 {
			self.fVec8[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec52[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec51[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec50[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec49[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec48[l58 as usize] = 0.0;
		}
		for l59 in 0..8192 {
			self.fRec47[l59 as usize] = 0.0;
		}
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		self.fTempPerm54 = 0.0;
		self.fTempPerm55 = 0.0;
		self.fTempPerm56 = 0.0;
		self.fTempPerm57 = 0.0;
		self.fTempPerm58 = 0.0;
		self.fTempPerm59 = 0.0;
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		self.fTempPerm62 = 0.0;
		self.fTempPerm63 = 0.0;
		self.fTempPerm64 = 0.0;
		self.fTempPerm65 = 0.0;
		self.fTempPerm66 = 0.0;
		self.fTempPerm67 = 0.0;
		self.fTempPerm68 = 0.0;
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		self.fTempPerm71 = 0.0;
		self.fTempPerm72 = 0.0;
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		self.fTempPerm75 = 0.0;
		self.fTempPerm76 = 0.0;
		self.fTempPerm77 = 0.0;
		self.fTempPerm78 = 0.0;
		self.fTempPerm79 = 0.0;
		self.fTempPerm80 = 0.0;
		self.fTempPerm81 = 0.0;
		for l60 in 0..2 {
			self.fRec53[l60 as usize] = 0.0;
		}
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.fTempPerm84 = 0.0;
		for l61 in 0..2 {
			self.fRec54[l61 as usize] = 0.0;
		}
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		self.fTempPerm87 = 0.0;
		for l62 in 0..2 {
			self.fRec55[l62 as usize] = 0.0;
		}
		self.fTempPerm88 = 0.0;
		self.fTempPerm89 = 0.0;
		for l63 in 0..2 {
			self.fRec56[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec57[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec58[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec59[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec60[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec61[l68 as usize] = 0.0;
		}
		self.fTempPerm90 = 0.0;
		for l69 in 0..2 {
			self.fRec13[l69 as usize] = 0.0;
		}
		self.fTempPerm91 = 0.0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		for l70 in 0..2 {
			self.fRec63[l70 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 - 44.1 / self.fConst0;
		self.fConst2 = 1.0 / self.fConst0;
		self.fConst3 = 0.9999 * self.fConst0;
		self.fConst4 = 2e+03 / self.fConst0;
		self.fConst5 = 0.45 * self.fConst0;
		self.fConst6 = 3.1415927 / self.fConst0;
		self.fConst7 = 2.0 * self.fConst0;
		self.fConst8 = F32::exp(-(25.0 / self.fConst0));
		let mut fConst9: F32 = 1.0 / F32::tan(21991.148 / self.fConst0);
		self.fConst10 = 1.0 - fConst9;
		self.fConst11 = 1.0 / (fConst9 + 1.0);
		self.fConst12 = 1.0 / F32::tan(31.415926 / self.fConst0);
		self.fConst13 = 1.0 - self.fConst12;
		self.fConst14 = 1.0 / (self.fConst12 + 1.0);
		self.fConst15 = 1.0 / F32::tan(15.707963 / self.fConst0);
		self.fConst16 = 1.0 - self.fConst15;
		self.fConst17 = 1.0 / (self.fConst15 + 1.0);
		self.fConst18 = 0.2 * self.fConst0;
		self.fConst19 = 6.2831855 / self.fConst0;
		self.fConst20 = 9.424778 / self.fConst0;
		self.fConst21 = 0.15915494 * self.fConst0;
		let mut fConst22: F32 = 1.0 / F32::tan(28274.334 / self.fConst0);
		self.fConst23 = 1.0 - fConst22;
		self.fConst24 = 1.0 / (fConst22 + 1.0);
		self.fConst25 = 1e+03 / self.fConst0;
		self.fConst26 = 1e+01 / self.fConst0;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		FxDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("ABDrumFX");
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Compressor");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.add_horizontal_slider("Threshold Mod", ParamIndex(0), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(1)), "0", "");
		ui_interface.declare(Some(ParamIndex(1)), "tooltip", "0 dB is off. 4:1, hard knee, auto makeup");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "dB");
		ui_interface.add_horizontal_slider("Threshold", ParamIndex(1), 0.0, -4e+01, 0.0, 0.1);
		ui_interface.declare(Some(ParamIndex(2)), "1", "");
		ui_interface.declare(Some(ParamIndex(2)), "style", "menu{'Squash 0.3/50 ms':0;'Tight 1/80 ms':1;'Punch 3/120 ms':2;'Snap 10/200 ms':3;'Open 30/400 ms':4}");
		ui_interface.add_num_entry("Timing", ParamIndex(2), 2.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "11", "");
		ui_interface.open_horizontal_box("Filter");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("Cutoff Mod", ParamIndex(3), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "0", "");
		ui_interface.declare(Some(ParamIndex(4)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(4)), "tooltip", "The SH-101's filter (lowpass, or bandpass/highpass by Type). At the top (20 kHz) it's off");
		ui_interface.declare(Some(ParamIndex(4)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Cutoff", ParamIndex(4), 2e+04, 2e+01, 2e+04, 0.1);
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.add_horizontal_slider("Resonance Mod", ParamIndex(5), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.declare(Some(ParamIndex(6)), "tooltip", "Up to the edge of oscillation: the top rings for about 2 s, then dies away");
		ui_interface.add_horizontal_slider("Resonance", ParamIndex(6), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "2", "");
		ui_interface.declare(Some(ParamIndex(7)), "style", "menu{'Post (after Comb)':0;'Pre (into Drive)':1}");
		ui_interface.add_num_entry("Position", ParamIndex(7), 0.0, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "3", "");
		ui_interface.declare(Some(ParamIndex(8)), "style", "menu{'LP (24 dB)':0;'BP (12 dB each side)':1;'HP (24 dB)':2}");
		ui_interface.add_num_entry("Type", ParamIndex(8), 0.0, 0.0, 2.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "12", "");
		ui_interface.open_horizontal_box("Chop");
		ui_interface.declare(Some(ParamIndex(9)), "0", "");
		ui_interface.add_horizontal_slider("Hold Mod", ParamIndex(9), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "0", "");
		ui_interface.declare(Some(ParamIndex(10)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(10)), "tooltip", "Each hit, then the gate slams shut. At the top (2 s) it's off");
		ui_interface.declare(Some(ParamIndex(10)), "unit", "ms");
		ui_interface.add_horizontal_slider("Hold", ParamIndex(10), 2e+03, 5.0, 2e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(11)), "1", "");
		ui_interface.add_horizontal_slider("Rate Mod", ParamIndex(11), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "1", "");
		ui_interface.declare(Some(ParamIndex(12)), "tooltip", "Square-wave AM from each hit: its first fifth deepens it at 10 Hz, then 10 to 400 Hz");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(12), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Crush");
		ui_interface.declare(Some(ParamIndex(13)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(13), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(14)), "0", "");
		ui_interface.declare(Some(ParamIndex(14)), "tooltip", "Lowers the sample rate (to 2 kHz), then the bit depth (to 4)");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(14), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "7", "");
		ui_interface.open_horizontal_box("Drive");
		ui_interface.declare(Some(ParamIndex(15)), "0", "");
		ui_interface.declare(Some(ParamIndex(15)), "style", "menu{'Tape':0;'Tube':1;'Clip':2;'Fuzz':3}");
		ui_interface.add_num_entry("Type", ParamIndex(15), 0.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(16)), "1", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(16), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.declare(Some(ParamIndex(17)), "tooltip", "First third: blends the drive under the clean hit. Above: all drive, harder");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(17), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("Ring Mod");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(18), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(19)), "0", "");
		ui_interface.declare(Some(ParamIndex(19)), "tooltip", "Deepens the ring mod as its carrier climbs from 1.07x to 3.7x Tone's pitch");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(19), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("Comb");
		ui_interface.declare(Some(ParamIndex(20)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(20), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "0", "");
		ui_interface.declare(Some(ParamIndex(21)), "tooltip", "Feedback (ring length) and mix together. Tuned to Tone's pitch, inharmonic");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(21), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.add_horizontal_slider("env_attack", ParamIndex(22), 0.0, 0.0, 1e+03, 0.1);
		ui_interface.add_horizontal_slider("env_curve", ParamIndex(23), 0.5, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("env_decay", ParamIndex(24), 3e+02, 5.0, 4e+03, 0.1);
		ui_interface.add_horizontal_slider("env_velocity", ParamIndex(25), 0.5, 0.0, 1.0, 0.001);
		ui_interface.add_button("hit", ParamIndex(26));
		ui_interface.add_horizontal_slider("tune", ParamIndex(27), 5e+01, 1.0, 2e+04, 0.01);
		ui_interface.add_horizontal_slider("vel", ParamIndex(28), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			26 => Some(self.fButton0),
			7 => Some(self.fEntry0),
			8 => Some(self.fEntry1),
			15 => Some(self.fEntry2),
			2 => Some(self.fEntry3),
			11 => Some(self.fHslider0),
			12 => Some(self.fHslider1),
			1 => Some(self.fHslider10),
			3 => Some(self.fHslider11),
			4 => Some(self.fHslider12),
			20 => Some(self.fHslider13),
			21 => Some(self.fHslider14),
			18 => Some(self.fHslider15),
			19 => Some(self.fHslider16),
			16 => Some(self.fHslider17),
			17 => Some(self.fHslider18),
			13 => Some(self.fHslider19),
			24 => Some(self.fHslider2),
			14 => Some(self.fHslider20),
			5 => Some(self.fHslider21),
			6 => Some(self.fHslider22),
			27 => Some(self.fHslider23),
			22 => Some(self.fHslider3),
			23 => Some(self.fHslider4),
			28 => Some(self.fHslider5),
			25 => Some(self.fHslider6),
			9 => Some(self.fHslider7),
			10 => Some(self.fHslider8),
			0 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			26 => { self.fButton0 = value }
			7 => { self.fEntry0 = value }
			8 => { self.fEntry1 = value }
			15 => { self.fEntry2 = value }
			2 => { self.fEntry3 = value }
			11 => { self.fHslider0 = value }
			12 => { self.fHslider1 = value }
			1 => { self.fHslider10 = value }
			3 => { self.fHslider11 = value }
			4 => { self.fHslider12 = value }
			20 => { self.fHslider13 = value }
			21 => { self.fHslider14 = value }
			18 => { self.fHslider15 = value }
			19 => { self.fHslider16 = value }
			16 => { self.fHslider17 = value }
			17 => { self.fHslider18 = value }
			13 => { self.fHslider19 = value }
			24 => { self.fHslider2 = value }
			14 => { self.fHslider20 = value }
			5 => { self.fHslider21 = value }
			6 => { self.fHslider22 = value }
			27 => { self.fHslider23 = value }
			22 => { self.fHslider3 = value }
			23 => { self.fHslider4 = value }
			28 => { self.fHslider5 = value }
			25 => { self.fHslider6 = value }
			9 => { self.fHslider7 = value }
			10 => { self.fHslider8 = value }
			0 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0) = if let [inputs0, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			(inputs0)
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
		let mut fSlow1: F32 = self.fHslider0;
		let mut fSlow2: F32 = self.fHslider1;
		let mut fSlow3: F32 = 0.001 * self.fHslider2;
		let mut fSlow4: F32 = 0.001 * self.fHslider3;
		let mut fSlow5: F32 = self.fHslider4;
		let mut fSlow6: F32 = self.fHslider5;
		let mut fSlow7: F32 = self.fHslider6;
		let mut fSlow8: F32 = self.fHslider7;
		let mut fSlow9: F32 = self.fHslider8;
		let mut fSlow10: F32 = self.fHslider9;
		let mut fSlow11: F32 = self.fHslider10;
		let mut iSlow12: i32 = (self.fEntry0 > 0.5) as i32;
		let mut iSlow13: i32 = i32::wrapping_sub(1, iSlow12);
		let mut fSlow14: F32 = self.fHslider11;
		let mut fSlow15: F32 = self.fHslider12;
		let mut fSlow16: F32 = self.fHslider13;
		let mut fSlow17: F32 = self.fHslider14;
		let mut fSlow18: F32 = self.fHslider15;
		let mut fSlow19: F32 = self.fHslider16;
		let mut fSlow20: F32 = self.fHslider17;
		let mut fSlow21: F32 = self.fHslider18;
		let mut fSlow22: F32 = self.fHslider19;
		let mut fSlow23: F32 = self.fHslider20;
		let mut fTemp35: F32 = self.fTempPerm0;
		let mut fTemp36: F32 = self.fTempPerm1;
		let mut fTemp37: F32 = self.fTempPerm2;
		let mut fTemp38: F32 = self.fTempPerm3;
		let mut fTemp39: F32 = self.fTempPerm4;
		let mut fTemp40: F32 = self.fTempPerm5;
		let mut fTemp41: F32 = self.fTempPerm6;
		let mut fTemp42: F32 = self.fTempPerm7;
		let mut fTemp43: F32 = self.fTempPerm8;
		let mut fTemp44: F32 = self.fTempPerm9;
		let mut fTemp45: F32 = self.fTempPerm10;
		let mut fTemp46: F32 = self.fTempPerm11;
		let mut fTemp47: F32 = self.fTempPerm12;
		let mut fTemp48: F32 = self.fTempPerm13;
		let mut fTemp49: F32 = self.fTempPerm14;
		let mut fTemp50: F32 = self.fTempPerm15;
		let mut fTemp51: F32 = self.fTempPerm16;
		let mut fTemp52: F32 = self.fTempPerm17;
		let mut fTemp53: F32 = self.fTempPerm18;
		let mut fTemp54: F32 = self.fTempPerm19;
		let mut fTemp55: F32 = self.fTempPerm20;
		let mut fSlow24: F32 = self.fHslider21;
		let mut fSlow25: F32 = self.fHslider22;
		let mut fTemp57: F32 = self.fTempPerm21;
		let mut fTemp58: F32 = self.fTempPerm22;
		let mut fTemp59: F32 = self.fTempPerm23;
		let mut fTemp60: F32 = self.fTempPerm24;
		let mut fTemp61: F32 = self.fTempPerm25;
		let mut fTemp62: F32 = self.fTempPerm26;
		let mut fTemp63: F32 = self.fTempPerm27;
		let mut fTemp64: F32 = self.fTempPerm28;
		let mut fTemp65: F32 = self.fTempPerm29;
		let mut fTemp66: F32 = self.fTempPerm30;
		let mut fTemp67: F32 = self.fTempPerm31;
		let mut fTemp68: F32 = self.fTempPerm32;
		let mut fTemp69: F32 = self.fTempPerm33;
		let mut fTemp70: F32 = self.fTempPerm34;
		let mut fTemp71: F32 = self.fTempPerm35;
		let mut fTemp72: F32 = self.fTempPerm36;
		let mut fTemp73: F32 = self.fTempPerm37;
		let mut fTemp74: F32 = self.fTempPerm38;
		let mut fTemp75: F32 = self.fTempPerm39;
		let mut fSlow26: F32 = self.fEntry1;
		let mut fSlow27: F32 = ((fSlow26 == 2.0) as i32) as u32 as F32;
		let mut fSlow28: F32 = 4.0 * ((fSlow26 == 1.0) as i32) as u32 as F32;
		let mut fSlow29: F32 = ((fSlow26 == 0.0) as i32) as u32 as F32;
		let mut fTemp76: F32 = self.fTempPerm40;
		let mut fSlow30: F32 = self.fEntry2;
		let mut iSlow31: i32 = (fSlow30 == 1.0) as i32;
		let mut iSlow32: i32 = (fSlow30 == 3.0) as i32;
		let mut iSlow33: i32 = (fSlow30 >= 2.0) as i32;
		let mut iSlow34: i32 = (fSlow30 >= 1.0) as i32;
		let mut iSlow35: i32 = if iSlow33 != 0 {0} else {if iSlow34 != 0 {0} else {6}};
		let mut fSlow36: F32 = F32::sqrt(F32::powf(1e+01, 0.05 * (-iSlow35) as F32));
		let mut fSlow37: F32 = 942.4778 / fSlow36;
		let mut fSlow38: F32 = self.fConst7 + fSlow37;
		let mut iSlow39: i32 = (fSlow30 >= 3.0) as i32;
		let mut fSlow40: F32 = F32::powf(1e+01, 0.05 * (if iSlow33 != 0 {if iSlow39 != 0 {0} else {6}} else {0}) as F32);
		let mut fSlow41: F32 = 942.4778 * fSlow36;
		let mut fSlow42: F32 = self.fConst7 + fSlow41;
		let mut fSlow43: F32 = F32::sqrt(fSlow40);
		let mut fSlow44: F32 = 18849.557 / fSlow43;
		let mut fSlow45: F32 = self.fConst7 + fSlow44;
		let mut fSlow46: F32 = fSlow44 - self.fConst7;
		let mut fSlow47: F32 = F32::min(fSlow30, 2.0);
		let mut iSlow48: i32 = (fSlow47 >= 2.0) as i32;
		let mut iSlow49: i32 = (fSlow47 >= 1.0) as i32;
		let mut iSlow50: i32 = if iSlow33 != 0 {15} else {if iSlow34 != 0 {12} else {9}};
		let mut fSlow51: F32 = (i32::wrapping_sub(if iSlow33 != 0 {if iSlow39 != 0 {36} else {40}} else {if iSlow34 != 0 {30} else {24}}, iSlow50)) as F32;
		let mut fSlow52: F32 = (iSlow50) as F32;
		let mut fSlow53: F32 = ((fSlow30 == 0.0) as i32) as u32 as F32;
		let mut fSlow54: F32 = 0.12 * fSlow53;
		let mut fSlow55: F32 = 18849.557 * fSlow43;
		let mut fSlow56: F32 = self.fConst7 + fSlow55;
		let mut fSlow57: F32 = F32::sqrt(F32::powf(1e+01, 0.05 * (iSlow35) as F32));
		let mut fSlow58: F32 = 942.4778 / fSlow57;
		let mut fSlow59: F32 = self.fConst7 + fSlow58;
		let mut fSlow60: F32 = 942.4778 * fSlow57;
		let mut fSlow61: F32 = fSlow60 - self.fConst7;
		let mut fSlow62: F32 = self.fConst7 + fSlow60;
		let mut fSlow63: F32 = fSlow58 - self.fConst7;
		let mut fSlow64: F32 = fSlow55 - self.fConst7;
		let mut fTemp134: F32 = self.fTempPerm41;
		let mut fTemp135: F32 = self.fTempPerm42;
		let mut fTemp136: F32 = self.fTempPerm43;
		let mut fTemp137: F32 = self.fTempPerm44;
		let mut fTemp138: F32 = self.fTempPerm45;
		let mut fTemp139: F32 = self.fTempPerm46;
		let mut fTemp140: F32 = self.fTempPerm47;
		let mut fTemp141: F32 = self.fTempPerm48;
		let mut fTemp142: F32 = self.fTempPerm49;
		let mut fTemp143: F32 = self.fTempPerm50;
		let mut fSlow65: F32 = fSlow41 - self.fConst7;
		let mut fSlow66: F32 = fSlow37 - self.fConst7;
		let mut fSlow67: F32 = self.fHslider23;
		let mut fSlow68: F32 = 1.07 * fSlow67;
		let mut fSlow69: F32 = F32::max(2e+01, F32::min(self.fConst18, fSlow67));
		let mut fSlow70: F32 = F32::atan(0.00011111111 * fSlow69);
		let mut fSlow71: F32 = self.fConst19 * fSlow69;
		let mut fSlow72: F32 = self.fConst20 * fSlow69;
		let mut fSlow73: F32 = F32::max(-0.9999, F32::min(-0.0001, -((1.0 - fSlow72) / (fSlow72 + 1.0))));
		let mut fSlow74: F32 = 4.0 * (fSlow71 - 2.0 * F32::atan2(fSlow73 * F32::sin(fSlow71), fSlow73 * F32::cos(fSlow71) + 1.0));
		let mut iSlow75: i32 = (F32::max(1.0, F32::min(4094.0, self.fConst21 * ((6.2831855 - (fSlow74 + fSlow70)) / fSlow69) + -1.0))) as i32;
		let mut iSlow76: i32 = i32::wrapping_add(std::cmp::min(4097, std::cmp::max(0, i32::wrapping_add(iSlow75, 1))), 1);
		let mut fSlow77: F32 = F32::max(1.0, F32::min(4094.0, self.fConst21 * ((6.2831855 - fSlow74 - fSlow70) / fSlow69) + -1.0));
		let mut fSlow78: F32 = F32::floor(fSlow77);
		let mut fSlow79: F32 = fSlow77 - fSlow78;
		let mut iSlow80: i32 = i32::wrapping_add(std::cmp::min(4097, std::cmp::max(0, iSlow75)), 1);
		let mut fSlow81: F32 = fSlow78 + (1.0 - fSlow77);
		let mut fTemp162: F32 = self.fTempPerm51;
		let mut fTemp163: F32 = self.fTempPerm52;
		let mut fTemp164: F32 = self.fTempPerm53;
		let mut fTemp165: F32 = self.fTempPerm54;
		let mut fTemp166: F32 = self.fTempPerm55;
		let mut fTemp167: F32 = self.fTempPerm56;
		let mut fTemp168: F32 = self.fTempPerm57;
		let mut fTemp169: F32 = self.fTempPerm58;
		let mut fTemp170: F32 = self.fTempPerm59;
		let mut fTemp171: F32 = self.fTempPerm60;
		let mut fTemp172: F32 = self.fTempPerm61;
		let mut fTemp173: F32 = self.fTempPerm62;
		let mut fTemp174: F32 = self.fTempPerm63;
		let mut fTemp175: F32 = self.fTempPerm64;
		let mut fTemp176: F32 = self.fTempPerm65;
		let mut fTemp177: F32 = self.fTempPerm66;
		let mut fTemp178: F32 = self.fTempPerm67;
		let mut fTemp179: F32 = self.fTempPerm68;
		let mut fTemp180: F32 = self.fTempPerm69;
		let mut fTemp181: F32 = self.fTempPerm70;
		let mut fTemp182: F32 = self.fTempPerm71;
		let mut fTemp183: F32 = self.fTempPerm72;
		let mut fTemp184: F32 = self.fTempPerm73;
		let mut fTemp185: F32 = self.fTempPerm74;
		let mut fTemp186: F32 = self.fTempPerm75;
		let mut fTemp187: F32 = self.fTempPerm76;
		let mut fTemp188: F32 = self.fTempPerm77;
		let mut fTemp189: F32 = self.fTempPerm78;
		let mut fTemp190: F32 = self.fTempPerm79;
		let mut fTemp191: F32 = self.fTempPerm80;
		let mut fTemp192: F32 = self.fTempPerm81;
		let mut fTemp193: F32 = self.fTempPerm82;
		let mut fTemp194: F32 = self.fTempPerm83;
		let mut fTemp195: F32 = self.fTempPerm84;
		let mut fTemp196: F32 = self.fTempPerm85;
		let mut fTemp197: F32 = self.fTempPerm86;
		let mut fTemp198: F32 = self.fTempPerm87;
		let mut fTemp199: F32 = self.fTempPerm88;
		let mut fTemp200: F32 = self.fTempPerm89;
		let mut fTemp201: F32 = self.fTempPerm90;
		let mut iSlow82: i32 = (self.fEntry3) as i32;
		let mut iSlow83: i32 = (iSlow82 >= 3) as i32;
		let mut iSlow84: i32 = (iSlow82 >= 2) as i32;
		let mut iSlow85: i32 = (iSlow82 >= 1) as i32;
		let mut iSlow86: i32 = (iSlow82 >= 4) as i32;
		let mut fSlow87: F32 = F32::exp(-(self.fConst25 / (if iSlow83 != 0 {if iSlow86 != 0 {400} else {200}} else {if iSlow84 != 0 {120} else {if iSlow85 != 0 {80} else {50}}}) as F32));
		let mut fSlow88: F32 = F32::exp(-(self.fConst25 / if iSlow83 != 0 {(if iSlow86 != 0 {30} else {10}) as F32} else {if iSlow84 != 0 {3.0} else {if iSlow85 != 0 {1.0} else {0.3}}}));
		let mut fTemp210: F32 = self.fTempPerm91;
		let mut fTemp211: F32 = self.fTempPerm92;
		let mut fTemp212: F32 = self.fTempPerm93;
		let mut fTemp213: F32 = self.fTempPerm94;
		let zipped_iterators = inputs0.zip(outputs0);
		for (input0, output0) in zipped_iterators {
			let mut fRec35: F32 = 0.0;
			let mut fRec62: F32 = 0.0;
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow0;
			let mut iTemp0: i32 = (fSlow0 > self.fVec1[1]) as i32;
			let mut iTemp1: i32 = i32::wrapping_sub(1, iTemp0);
			let mut fTemp2: F32 = (iTemp1) as F32;
			let mut fTemp3: F32 = 1.0 - self.fConst1 * fTemp2;
			self.fRec0[0] = fSlow1 * fTemp3 + self.fConst1 * fTemp2 * self.fRec0[1];
			self.fRec1[0] = fSlow2 * fTemp3 + self.fConst1 * fTemp2 * self.fRec1[1];
			self.fRec2[0] = fSlow3 * fTemp3 + self.fConst1 * fTemp2 * self.fRec2[1];
			self.fRec3[0] = fSlow4 * fTemp3 + self.fConst1 * fTemp2 * self.fRec3[1];
			self.iRec4[0] = i32::wrapping_mul(iTemp1, i32::wrapping_add(self.iRec4[1], 1));
			let mut fTemp4: F32 = (self.iRec4[0]) as F32;
			let mut fTemp5: F32 = self.fConst2 * fTemp4;
			self.fRec5[0] = fSlow5 * fTemp3 + self.fConst1 * fTemp2 * self.fRec5[1];
			let mut fTemp6: F32 = F32::powf(1.0 - F32::max(0.0, F32::min(1.0, (fTemp5 - self.fRec3[0]) / F32::max(0.0001, self.fRec2[0]))), 6.0 * self.fRec5[0] + 1.0);
			let mut fTemp7: F32 = F32::min(1.0, self.fConst2 * (fTemp4 / F32::max(1e-05, self.fRec3[0])));
			self.fRec6[0] = if iTemp0 != 0 {fSlow6} else {self.fRec6[1]};
			self.fRec7[0] = fSlow7 * fTemp3 + self.fConst1 * fTemp2 * self.fRec7[1];
			let mut fTemp8: F32 = 1.0 - self.fRec7[0] * (1.0 - self.fRec6[0]);
			self.iRec8[0] = std::cmp::max(self.iRec8[1], iTemp0);
			let mut fTemp9: F32 = (self.iRec8[0]) as F32;
			let mut fTemp10: F32 = if (self.fRec0[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec1[0] + self.fRec0[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec1[0]};
			let mut iTemp11: i32 = (fTemp10 > 0.0001) as i32;
			self.fRec9[0] = fSlow8 * fTemp3 + self.fConst1 * fTemp2 * self.fRec9[1];
			self.fRec10[0] = fSlow9 * fTemp3 + self.fConst1 * fTemp2 * self.fRec10[1];
			let mut fTemp12: F32 = if (self.fRec9[0] != 0.0) as i32 != 0 {1995.0 * FxDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0005012531 * (self.fRec10[0] + -5.0)))) + self.fRec9[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))) + 5.0} else {self.fRec10[0]};
			let mut iTemp13: i32 = (fTemp12 < 1999.0) as i32;
			let mut iTemp14: i32 = iTemp13 | iTemp11;
			self.fRec11[0] = fSlow10 * fTemp3 + self.fConst1 * fTemp2 * self.fRec11[1];
			let mut iTemp15: i32 = (self.fRec11[0] != 0.0) as i32;
			self.fRec12[0] = fSlow11 * fTemp3 + self.fConst1 * fTemp2 * self.fRec12[1];
			let mut fTemp16: F32 = 1.0 - F32::max(0.0, F32::min(1.0, 0.025 * (self.fRec12[0] + 4e+01) + self.fRec11[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6));
			self.fRec14[0] = fSlow14 * fTemp3 + self.fConst1 * fTemp2 * self.fRec14[1];
			self.fRec15[0] = fSlow15 * fTemp3 + self.fConst1 * fTemp2 * self.fRec15[1];
			let mut fTemp17: F32 = if (self.fRec14[0] != 0.0) as i32 != 0 {1.998e+04 * FxDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 5.005005e-05 * (self.fRec15[0] + -2e+01)))) + self.fRec14[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))) + 2e+01} else {self.fRec15[0]};
			let mut fTemp18: F32 = if iSlow13 != 0 {fTemp17} else {2e+04};
			let mut iTemp19: i32 = (fTemp18 >= 19999.0) as i32;
			self.fRec16[0] = fSlow16 * fTemp3 + self.fConst1 * fTemp2 * self.fRec16[1];
			self.fRec17[0] = fSlow17 * fTemp3 + self.fConst1 * fTemp2 * self.fRec17[1];
			let mut fTemp20: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec16[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec17[0] + self.fRec16[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec17[0]}));
			let mut iTemp21: i32 = (fTemp20 > 0.0) as i32;
			self.fRec18[0] = fSlow18 * fTemp3 + self.fConst1 * fTemp2 * self.fRec18[1];
			self.fRec19[0] = fSlow19 * fTemp3 + self.fConst1 * fTemp2 * self.fRec19[1];
			let mut fTemp22: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec18[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec19[0] + self.fRec18[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec19[0]}));
			let mut iTemp23: i32 = (fTemp22 > 0.0) as i32;
			self.fRec20[0] = fSlow20 * fTemp3 + self.fConst1 * fTemp2 * self.fRec20[1];
			self.fRec21[0] = fSlow21 * fTemp3 + self.fConst1 * fTemp2 * self.fRec21[1];
			let mut fTemp24: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec20[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec21[0] + self.fRec20[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec21[0]}));
			let mut iTemp25: i32 = (fTemp24 > 0.0) as i32;
			let mut fTemp26: F32 = if iSlow12 != 0 {fTemp17} else {2e+04};
			self.fRec22[0] = fSlow22 * fTemp3 + self.fConst1 * fTemp2 * self.fRec22[1];
			self.fRec23[0] = fSlow23 * fTemp3 + self.fConst1 * fTemp2 * self.fRec23[1];
			let mut fTemp27: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec22[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec23[0] + self.fRec22[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec23[0]}));
			let mut fTemp28: F32 = *input0;
			let mut fTemp29: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * F32::max(0.0, F32::min(1.0, 1.6666666 * (fTemp27 + -0.4))))) + -1.0));
			let mut fTemp30: F32 = self.fConst0 * F32::powf(self.fConst4, F32::max(0.0, F32::min(1.0, 1.6666666 * fTemp27)));
			let mut fTemp31: F32 = if i32::wrapping_sub(1, self.iVec0[1]) != 0 {0.0} else {self.fRec25[1] + self.fConst2 * F32::max(fTemp30, 0.0)};
			self.fRec25[0] = fTemp31 - F32::floor(fTemp31);
			self.fRec24[0] = if ((self.fRec25[0] < self.fRec25[1]) as i32) | ((fTemp30 >= self.fConst3) as i32) != 0 {fTemp28} else {self.fRec24[1]};
			let mut fTemp32: F32 = if (fTemp27 > 0.0001) as i32 != 0 {F32::floor(self.fRec24[0] * fTemp29 + 0.5) / fTemp29} else {fTemp28};
			let mut fTemp33: F32 = F32::max(0.0, F32::min(1.0, 0.00025006253 * (fTemp26 + -1.6e+04)));
			let mut fTemp34: F32 = ((fTemp26 < 19999.0) as i32) as u32 as F32;
			if (fTemp34 != 0.0) as i32 != 0 {
				fTemp35 = F32::abs(0.5 * self.fRec30[1]);
				fTemp36 = ((fTemp35 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp35)) + ((fTemp35 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp35) / F32::max(fTemp35, 0.001);
				fTemp37 = F32::max(1e+01, F32::min(self.fConst5, fTemp26));
				fTemp38 = F32::tan(self.fConst6 * fTemp37);
				fTemp39 = fTemp38 * fTemp36;
				fTemp40 = fTemp39 + 1.0;
				fTemp41 = F32::abs(0.5 * self.fRec33[1]);
				fTemp42 = ((fTemp41 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp41)) + ((fTemp41 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp41) / F32::max(fTemp41, 0.001);
				fTemp43 = fTemp38 * fTemp42;
				fTemp44 = fTemp43 + 1.0;
				fTemp45 = F32::abs(0.5 * self.fRec32[1]);
				fTemp46 = ((fTemp45 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp45)) + ((fTemp45 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp45) / F32::max(fTemp45, 0.001);
				fTemp47 = fTemp38 * fTemp46;
				fTemp48 = fTemp47 + 1.0;
				fTemp49 = F32::abs(0.5 * self.fRec31[1]);
				fTemp50 = ((fTemp49 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp49)) + ((fTemp49 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp49) / F32::max(fTemp49, 0.001);
				fTemp51 = fTemp38 * fTemp50;
				fTemp52 = fTemp51 + 1.0;
				fTemp53 = fTemp40 * fTemp52 * fTemp48;
				fTemp54 = F32::abs(self.fRec34[1]);
				fTemp55 = ((fTemp54 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp54)) + ((fTemp54 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp54) / F32::max(fTemp54, 0.001);
			}
				let mut fTemp56: F32 = ((fTemp18 < 19999.0) as i32) as u32 as F32;
				if ((fTemp56 != 0.0) as i32) | ((fTemp34 != 0.0) as i32) != 0 {
					self.fRec36[0] = fSlow24 * fTemp3 + self.fConst1 * fTemp2 * self.fRec36[1];
					self.fRec37[0] = fSlow25 * fTemp3 + self.fConst1 * fTemp2 * self.fRec37[1];
					fTemp57 = F32::powf(F32::max(0.0, F32::min(1.0, if (self.fRec36[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec37[0] + self.fRec36[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec37[0]})), 1.5);
				}
					if (fTemp34 != 0.0) as i32 != 0 {
						fTemp58 = FxDsp_faustpower4_f(1.0 - F32::powf(F32::min(0.5, 0.54970527 / fTemp37), fTemp57));
						fTemp59 = FxDsp_faustpower4_f(fTemp38) * fTemp36;
						fTemp60 = fTemp44 * (4.0 * (fTemp59 * fTemp58 * fTemp55 * fTemp50 * fTemp46 * fTemp42 / (fTemp53 * fTemp44)) + 1.0);
						fTemp61 = self.fRec29[1] + fTemp43 * (self.fRec28[1] + fTemp47 * (self.fRec27[1] + fTemp51 * self.fRec26[1] / fTemp40) / fTemp52) / fTemp48 + fTemp59 * fTemp32 * fTemp50 * fTemp46 * fTemp42 / fTemp53;
						fTemp62 = fTemp58 * fTemp55;
						fTemp63 = fTemp62 * fTemp61 / fTemp60;
						fTemp64 = 4.0 * fTemp63;
						fTemp65 = self.fRec26[1] + fTemp39 * (fTemp32 - fTemp64);
						fTemp66 = fTemp65 / fTemp40;
						fTemp67 = 2.0 * fTemp66 - self.fRec26[1];
						self.fRec26[0] = if (F32::abs(fTemp67) > 1e+06) as i32 != 0 {0.0} else {fTemp67};
						fTemp68 = self.fRec27[1] + fTemp51 * fTemp65 / fTemp40;
						fTemp69 = fTemp68 / fTemp52;
						fTemp70 = 2.0 * fTemp69 - self.fRec27[1];
						self.fRec27[0] = if (F32::abs(fTemp70) > 1e+06) as i32 != 0 {0.0} else {fTemp70};
						fTemp71 = (self.fRec28[1] + fTemp47 * fTemp68 / fTemp52) / fTemp48;
						fTemp72 = 2.0 * fTemp71;
						fTemp73 = fTemp72 - self.fRec28[1];
						self.fRec28[0] = if (F32::abs(fTemp73) > 1e+06) as i32 != 0 {0.0} else {fTemp73};
						fTemp74 = fTemp61 / fTemp60;
						fTemp75 = 2.0 * fTemp74 - self.fRec29[1];
						self.fRec29[0] = if (F32::abs(fTemp75) > 1e+06) as i32 != 0 {0.0} else {fTemp75};
						self.fRec30[0] = fTemp32 - (fTemp64 + fTemp66);
						self.fRec31[0] = fTemp66 - fTemp69;
						self.fRec32[0] = fTemp69 - fTemp71;
						self.fRec33[0] = fTemp71 - fTemp74;
						self.fRec34[0] = fTemp74;
						fRec35 = fSlow29 * (fTemp61 * (2.0 * fTemp62 + 1.0) / fTemp60) + fSlow28 * (fTemp74 + fTemp69 - fTemp72) + fSlow27 * (6.0 * fTemp69 + fTemp32 + fTemp74 - 4.0 * (fTemp71 + fTemp63 + fTemp66));
						fTemp76 = if (F32::abs(fRec35) > 1e+06) as i32 != 0 {0.0} else {fRec35} * fTemp34;
					}
						let mut fTemp77: F32 = if (fTemp26 >= 19999.0) as i32 != 0 {fTemp32} else {(1.0 - fTemp33) * fTemp34 * fTemp76 + fTemp33 * fTemp32};
						self.fVec2[0] = fTemp77;
						let mut fTemp78: F32 = fTemp24 + -0.33333334;
						let mut fTemp79: F32 = F32::max(0.0, F32::min(1.0, 1.5 * fTemp78));
						let mut fTemp80: F32 = F32::powf(1e+01, 0.05 * (fSlow52 + fSlow51 * F32::powf(fTemp79, 1.5)));
						let mut fTemp81: F32 = 0.8 * fTemp80;
						let mut fTemp82: F32 = F32::tanh(fTemp81);
						let mut fTemp83: F32 = 0.012 * fTemp80;
						let mut fTemp84: F32 = 0.5 * fTemp79;
						let mut fTemp85: F32 = 0.7 - fTemp84;
						let mut fTemp86: F32 = fTemp84 + 1.3;
						let mut fTemp87: F32 = if iSlow32 != 0 {F32::max(-1.0, F32::min(1.0, fTemp80 * (0.788 * fTemp86 * ((fTemp81 > fTemp83) as i32) as u32 as F32 + 0.812 * fTemp85 * ((fTemp81 < -fTemp83) as i32) as u32 as F32)))} else {if iSlow48 != 0 {F32::max(-1.0, F32::min(1.0, fTemp81))} else {if iSlow49 != 0 {fTemp82 * ((fTemp81 < 0.0) as i32) as u32 as F32 + (i32::wrapping_mul(2, (fTemp81 >= 0.0) as i32)) as F32 * F32::tanh(0.4 * fTemp80)} else {fTemp82}}};
						let mut fTemp88: F32 = F32::powf(1e+01, -(fSlow54 * fTemp79));
						self.fRec42[0] = -((self.fRec42[1] * fSlow63 - (fTemp77 * fSlow62 + self.fVec2[1] * fSlow61)) / fSlow59);
						let mut fTemp89: F32 = self.fRec42[0] * fSlow40;
						self.fVec3[0] = fTemp89;
						self.fRec41[0] = -((self.fRec41[1] * fSlow64 - (fTemp89 * fSlow45 + fSlow46 * self.fVec3[1])) / fSlow56);
						let mut fTemp90: F32 = self.fRec41[0] * fTemp80;
						self.fVec4[0] = fTemp90;
						let mut fTemp91: F32 = fTemp90 + self.fVec4[1];
						let mut fTemp92: F32 = 0.5 * fTemp91;
						let mut fTemp93: F32 = F32::tanh(fTemp92);
						let mut fTemp94: F32 = fTemp90 - self.fVec4[1];
						let mut fTemp95: F32 = F32::abs(fTemp94);
						let mut iTemp96: i32 = (fTemp95 > 0.0001) as i32;
						let mut fTemp97: F32 = (i32::wrapping_sub(1, iTemp96)) as F32;
						let mut fTemp98: F32 = F32::abs(self.fVec4[1]);
						let mut fTemp99: F32 = fTemp98 + F32::log(F32::exp(-(2.0 * fTemp98)) + 1.0, std::f32::consts::E);
						let mut fTemp100: F32 = F32::abs(fTemp90);
						let mut fTemp101: F32 = fTemp100 + F32::log(F32::exp(-(2.0 * fTemp100)) + 1.0, std::f32::consts::E);
						let mut fTemp102: F32 = fTemp90 * self.fVec4[1];
						let mut fTemp103: F32 = F32::max(fTemp100, fTemp98);
						let mut fTemp104: F32 = F32::exp(-(2.0 * fTemp103)) + 1.0;
						let mut fTemp105: F32 = F32::min(fTemp100, fTemp98);
						let mut fTemp106: F32 = fTemp105 - fTemp103;
						let mut fTemp107: F32 = 2.0 * fTemp106;
						let mut fTemp108: F32 = F32::abs(fTemp107);
						let mut fTemp109: F32 = F32::exp(-(2.0 * fTemp105)) * (2.0 * fTemp106 * ((fTemp108 < 0.001) as i32) as u32 as F32 * (fTemp106 * (0.6666667 * fTemp106 + 1.0) + 1.0) + ((fTemp108 >= 0.001) as i32) as u32 as F32 * (F32::exp(F32::max(fTemp107, -8e+01)) + -1.0));
						let mut fTemp110: F32 = fTemp109 / fTemp104;
						let mut fTemp111: F32 = F32::abs(-fTemp110);
						let mut fTemp112: F32 = fTemp100 + (i32::wrapping_add(i32::wrapping_mul(2, (fTemp100 <= fTemp98) as i32), -1)) as F32 * (((fTemp111 >= 0.001) as i32) as u32 as F32 * F32::log(1.0 - fTemp110, std::f32::consts::E) - fTemp109 * ((fTemp111 < 0.001) as i32) as u32 as F32 * (fTemp109 * (0.33333334 * fTemp110 + 0.5) / fTemp104 + 1.0) / fTemp104) - fTemp98;
						let mut fTemp113: F32 = F32::abs(0.5 * self.fVec4[1]);
						let mut iTemp114: i32 = (self.fVec4[1] >= 0.0) as i32;
						let mut iTemp115: i32 = (self.fVec4[1] < 0.0) as i32;
						let mut fTemp116: F32 = F32::abs(0.5 * fTemp90);
						let mut iTemp117: i32 = (fTemp90 >= 0.0) as i32;
						let mut iTemp118: i32 = (fTemp90 < 0.0) as i32;
						let mut iTemp119: i32 = i32::wrapping_mul(iTemp117, iTemp114);
						let mut iTemp120: i32 = i32::wrapping_mul(iTemp118, iTemp115);
						let mut fTemp121: F32 = F32::max(fTemp116, fTemp113);
						let mut fTemp122: F32 = F32::exp(-(2.0 * fTemp121)) + 1.0;
						let mut fTemp123: F32 = F32::min(fTemp116, fTemp113);
						let mut fTemp124: F32 = fTemp123 - fTemp121;
						let mut fTemp125: F32 = 2.0 * fTemp124;
						let mut fTemp126: F32 = F32::abs(fTemp125);
						let mut fTemp127: F32 = F32::exp(-(2.0 * fTemp123)) * (2.0 * fTemp124 * ((fTemp126 < 0.001) as i32) as u32 as F32 * (fTemp124 * (0.6666667 * fTemp124 + 1.0) + 1.0) + ((fTemp126 >= 0.001) as i32) as u32 as F32 * (F32::exp(F32::max(fTemp125, -8e+01)) + -1.0));
						let mut fTemp128: F32 = fTemp127 / fTemp122;
						let mut fTemp129: F32 = F32::abs(-fTemp128);
						let mut iTemp130: i32 = (fTemp98 <= 1.0) as i32;
						let mut iTemp131: i32 = (fTemp100 <= 1.0) as i32;
						let mut iTemp132: i32 = i32::wrapping_mul(iTemp131, iTemp130);
						let mut fTemp133: F32 = (i32::wrapping_mul(iSlow32, iTemp25)) as F32;
						if (fTemp133 != 0.0) as i32 != 0 {
							fTemp134 = 1.0 / fTemp85;
							self.fRec43[0] = F32::max(self.fConst8 * self.fRec43[1], F32::abs(fTemp77));
							fTemp135 = 0.05 * self.fRec43[0] + 0.012;
							fTemp136 = fTemp80 * fTemp135;
							fTemp137 = fTemp136 + fTemp134;
							fTemp138 = 1.0 / fTemp86;
							fTemp139 = fTemp136 + fTemp138;
							fTemp140 = self.fVec4[1] + fTemp136;
							fTemp141 = fTemp80 * (self.fRec41[0] + fTemp135);
							fTemp142 = fTemp80 * (self.fRec41[0] - fTemp135);
							fTemp143 = if (((i32::wrapping_sub(((fTemp90 > fTemp136) as i32) + ((fTemp90 > fTemp139) as i32), ((fTemp90 < -fTemp136) as i32) + ((fTemp90 < -fTemp137) as i32))) == (i32::wrapping_sub(((self.fVec4[1] > fTemp136) as i32) + ((self.fVec4[1] > fTemp139) as i32), ((self.fVec4[1] < -fTemp136) as i32) + ((self.fVec4[1] < -fTemp137) as i32)))) as i32) | ((fTemp95 < (0.001 * (fTemp100 + fTemp98 + 1.0))) as i32) != 0 {F32::max(-1.0, F32::min(1.0, fTemp86 * ((fTemp92 > fTemp136) as i32) as u32 as F32 * (fTemp92 - fTemp136) + fTemp85 * ((fTemp92 < -fTemp136) as i32) as u32 as F32 * (fTemp92 + fTemp136)))} else {(0.5 * (fTemp86 * FxDsp_faustpower2_f(F32::max(0.0, F32::min(fTemp138, fTemp142))) + fTemp85 * FxDsp_faustpower2_f(F32::max(0.0, F32::min(fTemp134, -fTemp141))) - (fTemp86 * FxDsp_faustpower2_f(F32::max(0.0, F32::min(fTemp138, self.fVec4[1] - fTemp136))) + fTemp85 * FxDsp_faustpower2_f(F32::max(0.0, F32::min(fTemp134, -fTemp140))))) + F32::max(0.0, fTemp142 - fTemp138) + F32::max(0.0, -(fTemp134 + fTemp141)) - (F32::max(0.0, self.fVec4[1] - fTemp139) + F32::max(0.0, -(fTemp134 + fTemp140)))) / (fTemp90 + ((fTemp94 == 0.0) as i32) as u32 as F32 - self.fVec4[1])} * fTemp133;
						}
							let mut fTemp144: F32 = if iSlow32 != 0 {fTemp143} else {(iTemp96) as F32 * if iSlow48 != 0 {0.5 * fTemp94 * (iTemp132) as F32 * fTemp91 + (i32::wrapping_sub(1, iTemp132)) as F32 * (((fTemp100 > 1.0) as i32) as u32 as F32 * (fTemp100 + -0.5) + 0.5 * (FxDsp_faustpower2_f(self.fRec41[0]) * FxDsp_faustpower2_f(fTemp80) * (iTemp131) as F32 - FxDsp_faustpower2_f(self.fVec4[1]) * (iTemp130) as F32) - ((fTemp98 > 1.0) as i32) as u32 as F32 * (fTemp98 + -0.5))} else {if iSlow49 != 0 {fTemp112 * (iTemp120) as F32 + (i32::wrapping_mul(4, iTemp119)) as F32 * (fTemp116 + (i32::wrapping_add(i32::wrapping_mul(2, (fTemp116 <= fTemp113) as i32), -1)) as F32 * (((fTemp129 >= 0.001) as i32) as u32 as F32 * F32::log(1.0 - fTemp128, std::f32::consts::E) - fTemp127 * ((fTemp129 < 0.001) as i32) as u32 as F32 * (fTemp127 * (0.33333334 * fTemp128 + 0.5) / fTemp122 + 1.0) / fTemp122) - fTemp113) + (i32::wrapping_sub(1, i32::wrapping_add(iTemp120, iTemp119))) as F32 * ((fTemp101 + -0.6931472) * (iTemp118) as F32 + (i32::wrapping_mul(4, iTemp117)) as F32 * (fTemp116 + F32::log(F32::exp(-(2.0 * fTemp116)) + 1.0, std::f32::consts::E) + -0.6931472) - ((fTemp99 + -0.6931472) * (iTemp115) as F32 + (i32::wrapping_mul(4, iTemp114)) as F32 * (fTemp113 + F32::log(F32::exp(-(2.0 * fTemp113)) + 1.0, std::f32::consts::E) + -0.6931472)))} else {((fTemp102 >= 0.0) as i32) as u32 as F32 * fTemp112 + ((fTemp102 < 0.0) as i32) as u32 as F32 * (fTemp101 - fTemp99)}} / (fTemp90 + fTemp97 - self.fVec4[1]) + fTemp97 * if iSlow48 != 0 {F32::max(-1.0, F32::min(1.0, fTemp92))} else {if iSlow49 != 0 {fTemp93 * ((fTemp92 < 0.0) as i32) as u32 as F32 + (i32::wrapping_mul(2, (fTemp92 >= 0.0) as i32)) as F32 * F32::tanh(0.25 * fTemp91)} else {fTemp93}}};
							let mut fTemp145: F32 = if (F32::abs(fTemp144) > 1e+06) as i32 != 0 {0.0} else {fTemp144};
							self.fVec5[0] = fTemp145 * fTemp88 / fTemp87;
							self.fRec40[0] = (0.8 * (fSlow64 * self.fVec5[1] + fSlow56 * fTemp145 * fTemp88 / fTemp87) - self.fRec40[1] * fSlow46) / fSlow45;
							self.fVec6[0] = self.fRec40[0] / fSlow40;
							self.fRec39[0] = -((self.fRec39[1] * fSlow66 - (self.fVec6[1] * fSlow65 + self.fRec40[0] * fSlow42 / fSlow40)) / fSlow38);
							self.fRec38[0] = -(self.fConst11 * (self.fConst10 * self.fRec38[1] - (self.fRec39[0] + self.fRec39[1])));
							let mut fTemp146: F32 = self.fRec39[0] * (1.0 - fSlow53 * fTemp79) + fSlow53 * self.fRec38[0] * fTemp79;
							self.fVec7[0] = fTemp146;
							let mut fTemp147: F32 = fTemp146 - self.fVec7[1];
							self.fRec44[0] = -(self.fConst14 * (self.fConst13 * self.fRec44[1] - self.fConst12 * fTemp147));
							self.fRec45[0] = -(self.fConst17 * (self.fConst16 * self.fRec45[1] - self.fConst15 * fTemp147));
							let mut fTemp148: F32 = if iSlow31 != 0 {self.fRec45[0]} else {if iSlow32 != 0 {self.fRec44[0]} else {fTemp146}};
							let mut fTemp149: F32 = F32::max(0.0, F32::min(1.0, 3.0 * fTemp24)) + F32::max(0.0, F32::min(1.0, 6.0 * fTemp78));
							let mut fTemp150: F32 = if iTemp25 != 0 {fTemp77 * (1.0 - 0.5 * fTemp149) + 0.5 * fTemp149 * if (F32::abs(fTemp148) > 1e+06) as i32 != 0 {0.0} else {fTemp148}} else {fTemp77};
							let mut fTemp151: F32 = F32::powf(fTemp22, 0.7);
							let mut fTemp152: F32 = self.fRec46[1] + self.fConst2 * F32::max(1.0, F32::min(self.fConst5, fSlow68 * F32::powf(3.457944, fTemp22)));
							self.fRec46[0] = if iTemp0 != 0 {0.0} else {fTemp152 - F32::floor(fTemp152)};
							let mut fTemp153: F32 = F32::sin(6.2831855 * self.fRec46[0]);
							let mut fTemp154: F32 = if iTemp23 != 0 {fTemp150 * (1.0 - fTemp151 + fTemp153 * fTemp151)} else {fTemp150};
							let mut fTemp155: F32 = F32::sqrt(fTemp20);
							let mut fTemp156: F32 = fSlow81 * self.fRec47[((i32::wrapping_sub(self.IOTA0, iSlow80)) & 8191) as usize] + fSlow79 * self.fRec47[((i32::wrapping_sub(self.IOTA0, iSlow76)) & 8191) as usize];
							self.fVec8[0] = fTemp156;
							self.fRec52[0] = self.fVec8[1] - fSlow73 * (self.fRec52[1] - fTemp156);
							self.fRec51[0] = self.fRec52[1] - fSlow73 * (self.fRec51[1] - self.fRec52[0]);
							self.fRec50[0] = self.fRec51[1] - fSlow73 * (self.fRec50[1] - self.fRec51[0]);
							self.fRec49[0] = self.fRec50[1] - fSlow73 * (self.fRec49[1] - self.fRec50[0]);
							self.fRec48[0] = -(self.fConst24 * (self.fConst23 * self.fRec48[1] - (self.fRec49[0] + self.fRec49[1])));
							self.fRec47[(self.IOTA0 & 8191) as usize] = 0.96 * self.fRec48[0] * fTemp155 + 2.0 * fTemp154 * (1.0 - 0.96 * fTemp155);
							let mut fTemp157: F32 = self.fRec47[(self.IOTA0 & 8191) as usize];
							let mut fTemp158: F32 = fTemp20 * if (F32::abs(fTemp157) > 1e+06) as i32 != 0 {0.0} else {fTemp157};
							let mut fTemp159: F32 = 1.0 - fTemp20;
							let mut fTemp160: F32 = if iTemp21 != 0 {fTemp154 * fTemp159 + fTemp158} else {fTemp154};
							let mut fTemp161: F32 = F32::max(0.0, F32::min(1.0, 0.00025006253 * (fTemp18 + -1.6e+04)));
							if (fTemp56 != 0.0) as i32 != 0 {
								fTemp162 = F32::abs(0.5 * self.fRec57[1]);
								fTemp163 = ((fTemp162 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp162)) + ((fTemp162 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp162) / F32::max(fTemp162, 0.001);
								fTemp164 = F32::max(1e+01, F32::min(self.fConst5, fTemp18));
								fTemp165 = F32::tan(self.fConst6 * fTemp164);
								fTemp166 = fTemp165 * fTemp163;
								fTemp167 = fTemp166 + 1.0;
								fTemp168 = F32::abs(0.5 * self.fRec60[1]);
								fTemp169 = ((fTemp168 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp168)) + ((fTemp168 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp168) / F32::max(fTemp168, 0.001);
								fTemp170 = fTemp165 * fTemp169;
								fTemp171 = fTemp170 + 1.0;
								fTemp172 = F32::abs(0.5 * self.fRec59[1]);
								fTemp173 = ((fTemp172 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp172)) + ((fTemp172 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp172) / F32::max(fTemp172, 0.001);
								fTemp174 = fTemp165 * fTemp173;
								fTemp175 = fTemp174 + 1.0;
								fTemp176 = F32::abs(0.5 * self.fRec58[1]);
								fTemp177 = ((fTemp176 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp176)) + ((fTemp176 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp176) / F32::max(fTemp176, 0.001);
								fTemp178 = fTemp165 * fTemp177;
								fTemp179 = fTemp178 + 1.0;
								fTemp180 = fTemp167 * fTemp179 * fTemp175;
								fTemp181 = F32::abs(self.fRec61[1]);
								fTemp182 = ((fTemp181 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp181)) + ((fTemp181 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp181) / F32::max(fTemp181, 0.001);
								fTemp183 = FxDsp_faustpower4_f(1.0 - F32::powf(F32::min(0.5, 0.54970527 / fTemp164), fTemp57));
								fTemp184 = FxDsp_faustpower4_f(fTemp165) * fTemp163;
								fTemp185 = fTemp171 * (4.0 * (fTemp184 * fTemp183 * fTemp182 * fTemp177 * fTemp173 * fTemp169 / (fTemp180 * fTemp171)) + 1.0);
								fTemp186 = self.fRec56[1] + fTemp170 * (self.fRec55[1] + fTemp174 * (self.fRec54[1] + fTemp178 * self.fRec53[1] / fTemp167) / fTemp179) / fTemp175 + fTemp184 * fTemp160 * fTemp177 * fTemp173 * fTemp169 / fTemp180;
								fTemp187 = fTemp183 * fTemp182;
								fTemp188 = fTemp187 * fTemp186 / fTemp185;
								fTemp189 = 4.0 * fTemp188;
								fTemp190 = self.fRec53[1] + fTemp166 * (fTemp160 - fTemp189);
								fTemp191 = fTemp190 / fTemp167;
								fTemp192 = 2.0 * fTemp191 - self.fRec53[1];
								self.fRec53[0] = if (F32::abs(fTemp192) > 1e+06) as i32 != 0 {0.0} else {fTemp192};
								fTemp193 = self.fRec54[1] + fTemp178 * fTemp190 / fTemp167;
								fTemp194 = fTemp193 / fTemp179;
								fTemp195 = 2.0 * fTemp194 - self.fRec54[1];
								self.fRec54[0] = if (F32::abs(fTemp195) > 1e+06) as i32 != 0 {0.0} else {fTemp195};
								fTemp196 = (self.fRec55[1] + fTemp174 * fTemp193 / fTemp179) / fTemp175;
								fTemp197 = 2.0 * fTemp196;
								fTemp198 = fTemp197 - self.fRec55[1];
								self.fRec55[0] = if (F32::abs(fTemp198) > 1e+06) as i32 != 0 {0.0} else {fTemp198};
								fTemp199 = fTemp186 / fTemp185;
								fTemp200 = 2.0 * fTemp199 - self.fRec56[1];
								self.fRec56[0] = if (F32::abs(fTemp200) > 1e+06) as i32 != 0 {0.0} else {fTemp200};
								self.fRec57[0] = fTemp160 - (fTemp189 + fTemp191);
								self.fRec58[0] = fTemp191 - fTemp194;
								self.fRec59[0] = fTemp194 - fTemp196;
								self.fRec60[0] = fTemp196 - fTemp199;
								self.fRec61[0] = fTemp199;
								fRec62 = fSlow29 * (fTemp186 * (2.0 * fTemp187 + 1.0) / fTemp185) + fSlow28 * (fTemp199 + fTemp194 - fTemp197) + fSlow27 * (6.0 * fTemp194 + fTemp160 + fTemp199 - 4.0 * (fTemp196 + fTemp188 + fTemp191));
								fTemp201 = if (F32::abs(fRec62) > 1e+06) as i32 != 0 {0.0} else {fRec62} * fTemp56;
							}
								let mut fTemp202: F32 = (1.0 - fTemp161) * fTemp56 * fTemp201;
								let mut fTemp203: F32 = F32::max(0.0, 2e+01 * F32::log10(F32::max(1.1754944e-38, F32::abs(if iTemp19 != 0 {fTemp160} else {fTemp202 + fTemp161 * fTemp160}) + 1e-09)) - if iTemp15 != 0 {-(4e+01 * fTemp16)} else {self.fRec12[0]});
								let mut fTemp204: F32 = if ((0.75 * fTemp203) > self.fRec13[1]) as i32 != 0 {fSlow88} else {fSlow87};
								self.fRec13[0] = self.fRec13[1] * fTemp204 + 0.75 * fTemp203 * (1.0 - fTemp204);
								let mut fTemp205: F32 = F32::powf(1e+01, -0.05 * (self.fRec13[0] + 0.375 * if iTemp15 != 0 {-4e+01 * fTemp16} else {self.fRec12[0]}));
								let mut fTemp206: F32 = if iTemp23 != 0 {fTemp150 * (1.0 - fTemp151 * (1.0 - fTemp153))} else {fTemp150};
								let mut fTemp207: F32 = if iTemp21 != 0 {fTemp158 + fTemp159 * fTemp206} else {fTemp206};
								let mut fTemp208: F32 = if iTemp19 != 0 {fTemp207} else {fTemp202 + fTemp161 * fTemp207};
								let mut fTemp209: F32 = (iTemp14) as F32;
								if (fTemp209 != 0.0) as i32 != 0 {
									fTemp210 = F32::max(0.0, F32::min(1.0, fTemp10));
									fTemp211 = F32::powf(4e+01, F32::max(0.0, F32::min(1.0, 1.25 * (fTemp210 + -0.2))));
									fTemp212 = self.fRec63[1] + self.fConst26 * fTemp211;
									self.fRec63[0] = if iTemp0 != 0 {0.0} else {fTemp212 - F32::floor(fTemp212)};
									fTemp213 = (1.0 - (iTemp13) as F32 * (1.0 - F32::max(0.0, F32::min(1.0, 1e+03 * (0.001 * (fTemp12 + 1.0) - fTemp5))))) * (1.0 - 0.5 * (iTemp11) as F32 * F32::max(0.0, F32::min(1.0, 5.0 * fTemp210)) * (1.0 - F32::max(-1.0, F32::min(1.0, F32::cos(6.2831855 * self.fRec63[0]) * F32::max(1.0, 106.103294 / fTemp211))))) * fTemp209;
								}
									*output0 = if iTemp14 != 0 {fTemp213 * fTemp208 * fTemp205} else {fTemp208 * fTemp205};
									self.iVec0[1] = self.iVec0[0];
									self.fVec1[1] = self.fVec1[0];
									self.fRec0[1] = self.fRec0[0];
									self.fRec1[1] = self.fRec1[0];
									self.fRec2[1] = self.fRec2[0];
									self.fRec3[1] = self.fRec3[0];
									self.iRec4[1] = self.iRec4[0];
									self.fRec5[1] = self.fRec5[0];
									self.fRec6[1] = self.fRec6[0];
									self.fRec7[1] = self.fRec7[0];
									self.iRec8[1] = self.iRec8[0];
									self.fRec9[1] = self.fRec9[0];
									self.fRec10[1] = self.fRec10[0];
									self.fRec11[1] = self.fRec11[0];
									self.fRec12[1] = self.fRec12[0];
									self.fRec14[1] = self.fRec14[0];
									self.fRec15[1] = self.fRec15[0];
									self.fRec16[1] = self.fRec16[0];
									self.fRec17[1] = self.fRec17[0];
									self.fRec18[1] = self.fRec18[0];
									self.fRec19[1] = self.fRec19[0];
									self.fRec20[1] = self.fRec20[0];
									self.fRec21[1] = self.fRec21[0];
									self.fRec22[1] = self.fRec22[0];
									self.fRec23[1] = self.fRec23[0];
									self.fRec25[1] = self.fRec25[0];
									self.fRec24[1] = self.fRec24[0];
									if ((fTemp56 != 0.0) as i32) | ((fTemp34 != 0.0) as i32) != 0 {
										self.fRec36[1] = self.fRec36[0];
										self.fRec37[1] = self.fRec37[0];
									}
										if (fTemp34 != 0.0) as i32 != 0 {
											self.fRec26[1] = self.fRec26[0];
											self.fRec27[1] = self.fRec27[0];
											self.fRec28[1] = self.fRec28[0];
											self.fRec29[1] = self.fRec29[0];
											self.fRec30[1] = self.fRec30[0];
											self.fRec31[1] = self.fRec31[0];
											self.fRec32[1] = self.fRec32[0];
											self.fRec33[1] = self.fRec33[0];
											self.fRec34[1] = self.fRec34[0];
										}
											self.fVec2[1] = self.fVec2[0];
											self.fRec42[1] = self.fRec42[0];
											self.fVec3[1] = self.fVec3[0];
											self.fRec41[1] = self.fRec41[0];
											self.fVec4[1] = self.fVec4[0];
											if (fTemp133 != 0.0) as i32 != 0 {
												self.fRec43[1] = self.fRec43[0];
											}
												self.fVec5[1] = self.fVec5[0];
												self.fRec40[1] = self.fRec40[0];
												self.fVec6[1] = self.fVec6[0];
												self.fRec39[1] = self.fRec39[0];
												self.fRec38[1] = self.fRec38[0];
												self.fVec7[1] = self.fVec7[0];
												self.fRec44[1] = self.fRec44[0];
												self.fRec45[1] = self.fRec45[0];
												self.fRec46[1] = self.fRec46[0];
												self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
												self.fVec8[1] = self.fVec8[0];
												self.fRec52[1] = self.fRec52[0];
												self.fRec51[1] = self.fRec51[0];
												self.fRec50[1] = self.fRec50[0];
												self.fRec49[1] = self.fRec49[0];
												self.fRec48[1] = self.fRec48[0];
												if (fTemp56 != 0.0) as i32 != 0 {
													self.fRec53[1] = self.fRec53[0];
													self.fRec54[1] = self.fRec54[0];
													self.fRec55[1] = self.fRec55[0];
													self.fRec56[1] = self.fRec56[0];
													self.fRec57[1] = self.fRec57[0];
													self.fRec58[1] = self.fRec58[0];
													self.fRec59[1] = self.fRec59[0];
													self.fRec60[1] = self.fRec60[0];
													self.fRec61[1] = self.fRec61[0];
												}
													self.fRec13[1] = self.fRec13[0];
													if (fTemp209 != 0.0) as i32 != 0 {
														self.fRec63[1] = self.fRec63[0];
													}
													}
													self.fTempPerm0 = fTemp35;
													self.fTempPerm1 = fTemp36;
													self.fTempPerm2 = fTemp37;
													self.fTempPerm3 = fTemp38;
													self.fTempPerm4 = fTemp39;
													self.fTempPerm5 = fTemp40;
													self.fTempPerm6 = fTemp41;
													self.fTempPerm7 = fTemp42;
													self.fTempPerm8 = fTemp43;
													self.fTempPerm9 = fTemp44;
													self.fTempPerm10 = fTemp45;
													self.fTempPerm11 = fTemp46;
													self.fTempPerm12 = fTemp47;
													self.fTempPerm13 = fTemp48;
													self.fTempPerm14 = fTemp49;
													self.fTempPerm15 = fTemp50;
													self.fTempPerm16 = fTemp51;
													self.fTempPerm17 = fTemp52;
													self.fTempPerm18 = fTemp53;
													self.fTempPerm19 = fTemp54;
													self.fTempPerm20 = fTemp55;
													self.fTempPerm21 = fTemp57;
													self.fTempPerm22 = fTemp58;
													self.fTempPerm23 = fTemp59;
													self.fTempPerm24 = fTemp60;
													self.fTempPerm25 = fTemp61;
													self.fTempPerm26 = fTemp62;
													self.fTempPerm27 = fTemp63;
													self.fTempPerm28 = fTemp64;
													self.fTempPerm29 = fTemp65;
													self.fTempPerm30 = fTemp66;
													self.fTempPerm31 = fTemp67;
													self.fTempPerm32 = fTemp68;
													self.fTempPerm33 = fTemp69;
													self.fTempPerm34 = fTemp70;
													self.fTempPerm35 = fTemp71;
													self.fTempPerm36 = fTemp72;
													self.fTempPerm37 = fTemp73;
													self.fTempPerm38 = fTemp74;
													self.fTempPerm39 = fTemp75;
													self.fTempPerm40 = fTemp76;
													self.fTempPerm41 = fTemp134;
													self.fTempPerm42 = fTemp135;
													self.fTempPerm43 = fTemp136;
													self.fTempPerm44 = fTemp137;
													self.fTempPerm45 = fTemp138;
													self.fTempPerm46 = fTemp139;
													self.fTempPerm47 = fTemp140;
													self.fTempPerm48 = fTemp141;
													self.fTempPerm49 = fTemp142;
													self.fTempPerm50 = fTemp143;
													self.fTempPerm51 = fTemp162;
													self.fTempPerm52 = fTemp163;
													self.fTempPerm53 = fTemp164;
													self.fTempPerm54 = fTemp165;
													self.fTempPerm55 = fTemp166;
													self.fTempPerm56 = fTemp167;
													self.fTempPerm57 = fTemp168;
													self.fTempPerm58 = fTemp169;
													self.fTempPerm59 = fTemp170;
													self.fTempPerm60 = fTemp171;
													self.fTempPerm61 = fTemp172;
													self.fTempPerm62 = fTemp173;
													self.fTempPerm63 = fTemp174;
													self.fTempPerm64 = fTemp175;
													self.fTempPerm65 = fTemp176;
													self.fTempPerm66 = fTemp177;
													self.fTempPerm67 = fTemp178;
													self.fTempPerm68 = fTemp179;
													self.fTempPerm69 = fTemp180;
													self.fTempPerm70 = fTemp181;
													self.fTempPerm71 = fTemp182;
													self.fTempPerm72 = fTemp183;
													self.fTempPerm73 = fTemp184;
													self.fTempPerm74 = fTemp185;
													self.fTempPerm75 = fTemp186;
													self.fTempPerm76 = fTemp187;
													self.fTempPerm77 = fTemp188;
													self.fTempPerm78 = fTemp189;
													self.fTempPerm79 = fTemp190;
													self.fTempPerm80 = fTemp191;
													self.fTempPerm81 = fTemp192;
													self.fTempPerm82 = fTemp193;
													self.fTempPerm83 = fTemp194;
													self.fTempPerm84 = fTemp195;
													self.fTempPerm85 = fTemp196;
													self.fTempPerm86 = fTemp197;
													self.fTempPerm87 = fTemp198;
													self.fTempPerm88 = fTemp199;
													self.fTempPerm89 = fTemp200;
													self.fTempPerm90 = fTemp201;
													self.fTempPerm91 = fTemp210;
													self.fTempPerm92 = fTemp211;
													self.fTempPerm93 = fTemp212;
													self.fTempPerm94 = fTemp213;
												}

}

