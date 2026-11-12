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
	fRec10: [F32;2],
	fHslider8: F32,
	fRec11: [F32;2],
	fHslider9: F32,
	fRec12: [F32;2],
	fHslider10: F32,
	fRec13: [F32;2],
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
	fConst3: F32,
	fConst4: F32,
	fRec21: [F32;2],
	fRec20: [F32;2],
	fVec2: [F32;2],
	fEntry0: F32,
	fConst5: F32,
	fRec26: [F32;2],
	fVec3: [F32;2],
	fRec25: [F32;2],
	fVec4: [F32;2],
	fVec5: [F32;2],
	fRec24: [F32;2],
	fVec6: [F32;2],
	fRec23: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fRec22: [F32;2],
	fVec7: [F32;2],
	fConst9: F32,
	fConst10: F32,
	fConst11: F32,
	fRec27: [F32;2],
	fConst12: F32,
	fHslider17: F32,
	fRec28: [F32;2],
	IOTA0: i32,
	fConst13: F32,
	fConst14: F32,
	fConst15: F32,
	fConst16: F32,
	fVec8: [F32;2],
	fRec34: [F32;2],
	fRec33: [F32;2],
	fRec32: [F32;2],
	fRec31: [F32;2],
	fConst18: F32,
	fConst19: F32,
	fRec30: [F32;2],
	fRec29: [F32;8192],
	fTempPerm0: F32,
	fTempPerm1: F32,
	fTempPerm2: F32,
	fConst20: F32,
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
	fHslider18: F32,
	fRec45: [F32;2],
	fHslider19: F32,
	fRec46: [F32;2],
	fTempPerm21: F32,
	fTempPerm22: F32,
	fTempPerm23: F32,
	fTempPerm24: F32,
	fTempPerm25: F32,
	fTempPerm26: F32,
	fTempPerm27: F32,
	fTempPerm28: F32,
	fTempPerm29: F32,
	fRec35: [F32;2],
	fTempPerm30: F32,
	fTempPerm31: F32,
	fTempPerm32: F32,
	fRec36: [F32;2],
	fTempPerm33: F32,
	fTempPerm34: F32,
	fRec37: [F32;2],
	fTempPerm35: F32,
	fTempPerm36: F32,
	fRec38: [F32;2],
	fRec39: [F32;2],
	fRec40: [F32;2],
	fRec41: [F32;2],
	fRec42: [F32;2],
	fRec43: [F32;2],
	fTempPerm37: F32,
	fEntry1: F32,
	fConst21: F32,
	fRec9: [F32;2],
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
			fRec10: [0.0;2],
			fHslider8: 0.0,
			fRec11: [0.0;2],
			fHslider9: 0.0,
			fRec12: [0.0;2],
			fHslider10: 0.0,
			fRec13: [0.0;2],
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
			fConst3: 0.0,
			fConst4: 0.0,
			fRec21: [0.0;2],
			fRec20: [0.0;2],
			fVec2: [0.0;2],
			fEntry0: 0.0,
			fConst5: 0.0,
			fRec26: [0.0;2],
			fVec3: [0.0;2],
			fRec25: [0.0;2],
			fVec4: [0.0;2],
			fVec5: [0.0;2],
			fRec24: [0.0;2],
			fVec6: [0.0;2],
			fRec23: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fRec22: [0.0;2],
			fVec7: [0.0;2],
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fRec27: [0.0;2],
			fConst12: 0.0,
			fHslider17: 0.0,
			fRec28: [0.0;2],
			IOTA0: 0,
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fVec8: [0.0;2],
			fRec34: [0.0;2],
			fRec33: [0.0;2],
			fRec32: [0.0;2],
			fRec31: [0.0;2],
			fConst18: 0.0,
			fConst19: 0.0,
			fRec30: [0.0;2],
			fRec29: [0.0;8192],
			fTempPerm0: 0.0,
			fTempPerm1: 0.0,
			fTempPerm2: 0.0,
			fConst20: 0.0,
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
			fHslider18: 0.0,
			fRec45: [0.0;2],
			fHslider19: 0.0,
			fRec46: [0.0;2],
			fTempPerm21: 0.0,
			fTempPerm22: 0.0,
			fTempPerm23: 0.0,
			fTempPerm24: 0.0,
			fTempPerm25: 0.0,
			fTempPerm26: 0.0,
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fTempPerm29: 0.0,
			fRec35: [0.0;2],
			fTempPerm30: 0.0,
			fTempPerm31: 0.0,
			fTempPerm32: 0.0,
			fRec36: [0.0;2],
			fTempPerm33: 0.0,
			fTempPerm34: 0.0,
			fRec37: [0.0;2],
			fTempPerm35: 0.0,
			fTempPerm36: 0.0,
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fRec40: [0.0;2],
			fRec41: [0.0;2],
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fTempPerm37: 0.0,
			fEntry1: 0.0,
			fConst21: 0.0,
			fRec9: [0.0;2],
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
		self.fHslider8 = 2e+04;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 0.0;
		self.fHslider15 = 0.0;
		self.fHslider16 = 0.0;
		self.fEntry0 = 0.0;
		self.fHslider17 = 5e+01;
		self.fHslider18 = 0.0;
		self.fHslider19 = 0.0;
		self.fEntry1 = 2.0;
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
			self.fRec10[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec11[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec12[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec13[l14 as usize] = 0.0;
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
			self.fRec21[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec20[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fVec2[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec26[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec3[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec25[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fVec4[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fVec5[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec24[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fVec6[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec23[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec22[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec7[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec27[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec28[l35 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l36 in 0..2 {
			self.fVec8[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec34[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec33[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec32[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec31[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec30[l41 as usize] = 0.0;
		}
		for l42 in 0..8192 {
			self.fRec29[l42 as usize] = 0.0;
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
		for l43 in 0..2 {
			self.fRec45[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec46[l44 as usize] = 0.0;
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
		for l45 in 0..2 {
			self.fRec35[l45 as usize] = 0.0;
		}
		self.fTempPerm30 = 0.0;
		self.fTempPerm31 = 0.0;
		self.fTempPerm32 = 0.0;
		for l46 in 0..2 {
			self.fRec36[l46 as usize] = 0.0;
		}
		self.fTempPerm33 = 0.0;
		self.fTempPerm34 = 0.0;
		for l47 in 0..2 {
			self.fRec37[l47 as usize] = 0.0;
		}
		self.fTempPerm35 = 0.0;
		self.fTempPerm36 = 0.0;
		for l48 in 0..2 {
			self.fRec38[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec39[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec40[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec41[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec42[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec43[l53 as usize] = 0.0;
		}
		self.fTempPerm37 = 0.0;
		for l54 in 0..2 {
			self.fRec9[l54 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 - 44.1 / self.fConst0;
		self.fConst2 = 1.0 / self.fConst0;
		self.fConst3 = 0.9999 * self.fConst0;
		self.fConst4 = 2e+03 / self.fConst0;
		self.fConst5 = 2.0 * self.fConst0;
		let mut fConst6: F32 = 1.0 / F32::tan(21991.148 / self.fConst0);
		self.fConst7 = 1.0 - fConst6;
		self.fConst8 = 1.0 / (fConst6 + 1.0);
		self.fConst9 = 1.0 / F32::tan(15.707963 / self.fConst0);
		self.fConst10 = 1.0 - self.fConst9;
		self.fConst11 = 1.0 / (self.fConst9 + 1.0);
		self.fConst12 = 0.45 * self.fConst0;
		self.fConst13 = 0.2 * self.fConst0;
		self.fConst14 = 6.2831855 / self.fConst0;
		self.fConst15 = 9.424778 / self.fConst0;
		self.fConst16 = 0.15915494 * self.fConst0;
		let mut fConst17: F32 = 1.0 / F32::tan(28274.334 / self.fConst0);
		self.fConst18 = 1.0 - fConst17;
		self.fConst19 = 1.0 / (fConst17 + 1.0);
		self.fConst20 = 3.1415927 / self.fConst0;
		self.fConst21 = 1e+03 / self.fConst0;
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
		ui_interface.declare(Some(ParamIndex(4)), "tooltip", "The SH-101's 24 dB/oct lowpass. At the top (20 kHz) it's off");
		ui_interface.declare(Some(ParamIndex(4)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Cutoff", ParamIndex(4), 2e+04, 2e+01, 2e+04, 0.1);
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.add_horizontal_slider("Resonance Mod", ParamIndex(5), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.declare(Some(ParamIndex(6)), "tooltip", "Up to the edge of oscillation: the top rings for about 2 s, then dies away");
		ui_interface.add_horizontal_slider("Resonance", ParamIndex(6), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Crush");
		ui_interface.declare(Some(ParamIndex(7)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(7), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "0", "");
		ui_interface.declare(Some(ParamIndex(8)), "tooltip", "Lowers the sample rate (to 2 kHz), then the bit depth (to 4)");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(8), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "7", "");
		ui_interface.open_horizontal_box("Drive");
		ui_interface.declare(Some(ParamIndex(9)), "0", "");
		ui_interface.declare(Some(ParamIndex(9)), "style", "menu{'Tape':0;'Tube':1;'Clip':2}");
		ui_interface.add_num_entry("Type", ParamIndex(9), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(10)), "1", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(10), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(11)), "1", "");
		ui_interface.declare(Some(ParamIndex(11)), "tooltip", "First third: blends the drive under the clean hit. Above: all drive, harder");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(11), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("Ring Mod");
		ui_interface.declare(Some(ParamIndex(12)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(12), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(13)), "0", "");
		ui_interface.declare(Some(ParamIndex(13)), "tooltip", "Deepens the ring mod as its carrier climbs from 1.07x to 3.7x Tone's pitch");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(13), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("Comb");
		ui_interface.declare(Some(ParamIndex(14)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(14), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(15)), "0", "");
		ui_interface.declare(Some(ParamIndex(15)), "tooltip", "Feedback (ring length) and mix together. Tuned to Tone's pitch, inharmonic");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(15), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.add_horizontal_slider("env_attack", ParamIndex(16), 0.0, 0.0, 1e+03, 0.1);
		ui_interface.add_horizontal_slider("env_curve", ParamIndex(17), 0.5, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("env_decay", ParamIndex(18), 3e+02, 5.0, 4e+03, 0.1);
		ui_interface.add_horizontal_slider("env_velocity", ParamIndex(19), 0.5, 0.0, 1.0, 0.001);
		ui_interface.add_button("hit", ParamIndex(20));
		ui_interface.add_horizontal_slider("tune", ParamIndex(21), 5e+01, 1.0, 2e+04, 0.01);
		ui_interface.add_horizontal_slider("vel", ParamIndex(22), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			20 => Some(self.fButton0),
			9 => Some(self.fEntry0),
			2 => Some(self.fEntry1),
			0 => Some(self.fHslider0),
			1 => Some(self.fHslider1),
			15 => Some(self.fHslider10),
			12 => Some(self.fHslider11),
			13 => Some(self.fHslider12),
			10 => Some(self.fHslider13),
			11 => Some(self.fHslider14),
			7 => Some(self.fHslider15),
			8 => Some(self.fHslider16),
			21 => Some(self.fHslider17),
			5 => Some(self.fHslider18),
			6 => Some(self.fHslider19),
			18 => Some(self.fHslider2),
			16 => Some(self.fHslider3),
			17 => Some(self.fHslider4),
			22 => Some(self.fHslider5),
			19 => Some(self.fHslider6),
			3 => Some(self.fHslider7),
			4 => Some(self.fHslider8),
			14 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			20 => { self.fButton0 = value }
			9 => { self.fEntry0 = value }
			2 => { self.fEntry1 = value }
			0 => { self.fHslider0 = value }
			1 => { self.fHslider1 = value }
			15 => { self.fHslider10 = value }
			12 => { self.fHslider11 = value }
			13 => { self.fHslider12 = value }
			10 => { self.fHslider13 = value }
			11 => { self.fHslider14 = value }
			7 => { self.fHslider15 = value }
			8 => { self.fHslider16 = value }
			21 => { self.fHslider17 = value }
			5 => { self.fHslider18 = value }
			6 => { self.fHslider19 = value }
			18 => { self.fHslider2 = value }
			16 => { self.fHslider3 = value }
			17 => { self.fHslider4 = value }
			22 => { self.fHslider5 = value }
			19 => { self.fHslider6 = value }
			3 => { self.fHslider7 = value }
			4 => { self.fHslider8 = value }
			14 => { self.fHslider9 = value }
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
		let mut fSlow12: F32 = self.fHslider11;
		let mut fSlow13: F32 = self.fHslider12;
		let mut fSlow14: F32 = self.fHslider13;
		let mut fSlow15: F32 = self.fHslider14;
		let mut fSlow16: F32 = self.fHslider15;
		let mut fSlow17: F32 = self.fHslider16;
		let mut fSlow18: F32 = self.fEntry0;
		let mut iSlow19: i32 = (fSlow18 == 1.0) as i32;
		let mut iSlow20: i32 = (fSlow18 >= 2.0) as i32;
		let mut iSlow21: i32 = (fSlow18 >= 1.0) as i32;
		let mut iSlow22: i32 = if iSlow20 != 0 {0} else {if iSlow21 != 0 {0} else {6}};
		let mut fSlow23: F32 = F32::sqrt(F32::powf(1e+01, 0.05 * (-iSlow22) as F32));
		let mut fSlow24: F32 = 942.4778 / fSlow23;
		let mut fSlow25: F32 = self.fConst5 + fSlow24;
		let mut fSlow26: F32 = F32::powf(1e+01, 0.05 * (if iSlow20 != 0 {6} else {0}) as F32);
		let mut fSlow27: F32 = 942.4778 * fSlow23;
		let mut fSlow28: F32 = self.fConst5 + fSlow27;
		let mut fSlow29: F32 = F32::sqrt(fSlow26);
		let mut fSlow30: F32 = 18849.557 / fSlow29;
		let mut fSlow31: F32 = self.fConst5 + fSlow30;
		let mut fSlow32: F32 = fSlow30 - self.fConst5;
		let mut iSlow33: i32 = if iSlow20 != 0 {15} else {if iSlow21 != 0 {12} else {9}};
		let mut fSlow34: F32 = (i32::wrapping_sub(if iSlow20 != 0 {40} else {if iSlow21 != 0 {30} else {24}}, iSlow33)) as F32;
		let mut fSlow35: F32 = (iSlow33) as F32;
		let mut fSlow36: F32 = ((fSlow18 == 0.0) as i32) as u32 as F32;
		let mut fSlow37: F32 = 0.12 * fSlow36;
		let mut fSlow38: F32 = 18849.557 * fSlow29;
		let mut fSlow39: F32 = self.fConst5 + fSlow38;
		let mut fSlow40: F32 = F32::sqrt(F32::powf(1e+01, 0.05 * (iSlow22) as F32));
		let mut fSlow41: F32 = 942.4778 / fSlow40;
		let mut fSlow42: F32 = self.fConst5 + fSlow41;
		let mut fSlow43: F32 = 942.4778 * fSlow40;
		let mut fSlow44: F32 = fSlow43 - self.fConst5;
		let mut fSlow45: F32 = self.fConst5 + fSlow43;
		let mut fSlow46: F32 = fSlow41 - self.fConst5;
		let mut fSlow47: F32 = fSlow38 - self.fConst5;
		let mut fSlow48: F32 = fSlow27 - self.fConst5;
		let mut fSlow49: F32 = fSlow24 - self.fConst5;
		let mut fSlow50: F32 = self.fHslider17;
		let mut fSlow51: F32 = 1.07 * fSlow50;
		let mut fSlow52: F32 = F32::max(2e+01, F32::min(self.fConst13, fSlow50));
		let mut fSlow53: F32 = F32::atan(0.00011111111 * fSlow52);
		let mut fSlow54: F32 = self.fConst14 * fSlow52;
		let mut fSlow55: F32 = self.fConst15 * fSlow52;
		let mut fSlow56: F32 = F32::max(-0.9999, F32::min(-0.0001, -((1.0 - fSlow55) / (fSlow55 + 1.0))));
		let mut fSlow57: F32 = 4.0 * (fSlow54 - 2.0 * F32::atan2(fSlow56 * F32::sin(fSlow54), fSlow56 * F32::cos(fSlow54) + 1.0));
		let mut iSlow58: i32 = (F32::max(1.0, F32::min(4094.0, self.fConst16 * ((6.2831855 - (fSlow57 + fSlow53)) / fSlow52) + -1.0))) as i32;
		let mut iSlow59: i32 = i32::wrapping_add(std::cmp::min(4097, std::cmp::max(0, i32::wrapping_add(iSlow58, 1))), 1);
		let mut fSlow60: F32 = F32::max(1.0, F32::min(4094.0, self.fConst16 * ((6.2831855 - fSlow57 - fSlow53) / fSlow52) + -1.0));
		let mut fSlow61: F32 = F32::floor(fSlow60);
		let mut fSlow62: F32 = fSlow60 - fSlow61;
		let mut iSlow63: i32 = i32::wrapping_add(std::cmp::min(4097, std::cmp::max(0, iSlow58)), 1);
		let mut fSlow64: F32 = fSlow61 + (1.0 - fSlow60);
		let mut fTemp92: F32 = self.fTempPerm0;
		let mut fTemp93: F32 = self.fTempPerm1;
		let mut fTemp94: F32 = self.fTempPerm2;
		let mut fTemp95: F32 = self.fTempPerm3;
		let mut fTemp96: F32 = self.fTempPerm4;
		let mut fTemp97: F32 = self.fTempPerm5;
		let mut fTemp98: F32 = self.fTempPerm6;
		let mut fTemp99: F32 = self.fTempPerm7;
		let mut fTemp100: F32 = self.fTempPerm8;
		let mut fTemp101: F32 = self.fTempPerm9;
		let mut fTemp102: F32 = self.fTempPerm10;
		let mut fTemp103: F32 = self.fTempPerm11;
		let mut fTemp104: F32 = self.fTempPerm12;
		let mut fTemp105: F32 = self.fTempPerm13;
		let mut fTemp106: F32 = self.fTempPerm14;
		let mut fTemp107: F32 = self.fTempPerm15;
		let mut fTemp108: F32 = self.fTempPerm16;
		let mut fTemp109: F32 = self.fTempPerm17;
		let mut fTemp110: F32 = self.fTempPerm18;
		let mut fTemp111: F32 = self.fTempPerm19;
		let mut fTemp112: F32 = self.fTempPerm20;
		let mut fSlow65: F32 = self.fHslider18;
		let mut fSlow66: F32 = self.fHslider19;
		let mut fTemp113: F32 = self.fTempPerm21;
		let mut fTemp114: F32 = self.fTempPerm22;
		let mut fTemp115: F32 = self.fTempPerm23;
		let mut fTemp116: F32 = self.fTempPerm24;
		let mut fTemp117: F32 = self.fTempPerm25;
		let mut fTemp118: F32 = self.fTempPerm26;
		let mut fTemp119: F32 = self.fTempPerm27;
		let mut fTemp120: F32 = self.fTempPerm28;
		let mut fTemp121: F32 = self.fTempPerm29;
		let mut fTemp122: F32 = self.fTempPerm30;
		let mut fTemp123: F32 = self.fTempPerm31;
		let mut fTemp124: F32 = self.fTempPerm32;
		let mut fTemp125: F32 = self.fTempPerm33;
		let mut fTemp126: F32 = self.fTempPerm34;
		let mut fTemp127: F32 = self.fTempPerm35;
		let mut fTemp128: F32 = self.fTempPerm36;
		let mut fTemp129: F32 = self.fTempPerm37;
		let mut iSlow67: i32 = (self.fEntry1) as i32;
		let mut iSlow68: i32 = (iSlow67 >= 3) as i32;
		let mut iSlow69: i32 = (iSlow67 >= 2) as i32;
		let mut iSlow70: i32 = (iSlow67 >= 1) as i32;
		let mut iSlow71: i32 = (iSlow67 >= 4) as i32;
		let mut fSlow72: F32 = F32::exp(-(self.fConst21 / (if iSlow68 != 0 {if iSlow71 != 0 {400} else {200}} else {if iSlow69 != 0 {120} else {if iSlow70 != 0 {80} else {50}}}) as F32));
		let mut fSlow73: F32 = F32::exp(-(self.fConst21 / if iSlow68 != 0 {(if iSlow71 != 0 {30} else {10}) as F32} else {if iSlow69 != 0 {3.0} else {if iSlow70 != 0 {1.0} else {0.3}}}));
		let zipped_iterators = inputs0.zip(outputs0);
		for (input0, output0) in zipped_iterators {
			let mut fRec44: F32 = 0.0;
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow0;
			let mut iTemp0: i32 = (fSlow0 > self.fVec1[1]) as i32;
			let mut iTemp1: i32 = i32::wrapping_sub(1, iTemp0);
			let mut fTemp2: F32 = (iTemp1) as F32;
			let mut fTemp3: F32 = 1.0 - self.fConst1 * fTemp2;
			self.fRec0[0] = fSlow1 * fTemp3 + self.fConst1 * fTemp2 * self.fRec0[1];
			let mut iTemp4: i32 = (self.fRec0[0] != 0.0) as i32;
			self.fRec1[0] = fSlow2 * fTemp3 + self.fConst1 * fTemp2 * self.fRec1[1];
			self.fRec2[0] = fSlow3 * fTemp3 + self.fConst1 * fTemp2 * self.fRec2[1];
			self.fRec3[0] = fSlow4 * fTemp3 + self.fConst1 * fTemp2 * self.fRec3[1];
			self.iRec4[0] = i32::wrapping_mul(iTemp1, i32::wrapping_add(self.iRec4[1], 1));
			let mut fTemp5: F32 = (self.iRec4[0]) as F32;
			self.fRec5[0] = fSlow5 * fTemp3 + self.fConst1 * fTemp2 * self.fRec5[1];
			let mut fTemp6: F32 = F32::powf(1.0 - F32::max(0.0, F32::min(1.0, (self.fConst2 * fTemp5 - self.fRec3[0]) / F32::max(0.0001, self.fRec2[0]))), 6.0 * self.fRec5[0] + 1.0);
			let mut fTemp7: F32 = F32::min(1.0, self.fConst2 * (fTemp5 / F32::max(1e-05, self.fRec3[0])));
			self.fRec6[0] = if iTemp0 != 0 {fSlow6} else {self.fRec6[1]};
			self.fRec7[0] = fSlow7 * fTemp3 + self.fConst1 * fTemp2 * self.fRec7[1];
			let mut fTemp8: F32 = 1.0 - self.fRec7[0] * (1.0 - self.fRec6[0]);
			self.iRec8[0] = std::cmp::max(self.iRec8[1], iTemp0);
			let mut fTemp9: F32 = (self.iRec8[0]) as F32;
			let mut fTemp10: F32 = 1.0 - F32::max(0.0, F32::min(1.0, 0.025 * (self.fRec1[0] + 4e+01) + self.fRec0[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6));
			self.fRec10[0] = fSlow8 * fTemp3 + self.fConst1 * fTemp2 * self.fRec10[1];
			self.fRec11[0] = fSlow9 * fTemp3 + self.fConst1 * fTemp2 * self.fRec11[1];
			let mut fTemp11: F32 = if (self.fRec10[0] != 0.0) as i32 != 0 {1.998e+04 * FxDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 5.005005e-05 * (self.fRec11[0] + -2e+01)))) + self.fRec10[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))) + 2e+01} else {self.fRec11[0]};
			let mut iTemp12: i32 = (fTemp11 >= 19999.0) as i32;
			self.fRec12[0] = fSlow10 * fTemp3 + self.fConst1 * fTemp2 * self.fRec12[1];
			self.fRec13[0] = fSlow11 * fTemp3 + self.fConst1 * fTemp2 * self.fRec13[1];
			let mut fTemp13: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec12[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec13[0] + self.fRec12[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec13[0]}));
			let mut iTemp14: i32 = (fTemp13 > 0.0) as i32;
			self.fRec14[0] = fSlow12 * fTemp3 + self.fConst1 * fTemp2 * self.fRec14[1];
			self.fRec15[0] = fSlow13 * fTemp3 + self.fConst1 * fTemp2 * self.fRec15[1];
			let mut fTemp15: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec14[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec15[0] + self.fRec14[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec15[0]}));
			let mut iTemp16: i32 = (fTemp15 > 0.0) as i32;
			self.fRec16[0] = fSlow14 * fTemp3 + self.fConst1 * fTemp2 * self.fRec16[1];
			self.fRec17[0] = fSlow15 * fTemp3 + self.fConst1 * fTemp2 * self.fRec17[1];
			let mut fTemp17: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec16[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec17[0] + self.fRec16[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec17[0]}));
			self.fRec18[0] = fSlow16 * fTemp3 + self.fConst1 * fTemp2 * self.fRec18[1];
			self.fRec19[0] = fSlow17 * fTemp3 + self.fConst1 * fTemp2 * self.fRec19[1];
			let mut fTemp18: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec18[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec19[0] + self.fRec18[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec19[0]}));
			let mut fTemp19: F32 = *input0;
			let mut fTemp20: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * F32::max(0.0, F32::min(1.0, 1.6666666 * (fTemp18 + -0.4))))) + -1.0));
			let mut fTemp21: F32 = self.fConst0 * F32::powf(self.fConst4, F32::max(0.0, F32::min(1.0, 1.6666666 * fTemp18)));
			let mut fTemp22: F32 = if i32::wrapping_sub(1, self.iVec0[1]) != 0 {0.0} else {self.fRec21[1] + self.fConst2 * F32::max(fTemp21, 0.0)};
			self.fRec21[0] = fTemp22 - F32::floor(fTemp22);
			self.fRec20[0] = if ((self.fRec21[0] < self.fRec21[1]) as i32) | ((fTemp21 >= self.fConst3) as i32) != 0 {fTemp19} else {self.fRec20[1]};
			let mut fTemp23: F32 = if (fTemp18 > 0.0001) as i32 != 0 {F32::floor(self.fRec20[0] * fTemp20 + 0.5) / fTemp20} else {fTemp19};
			self.fVec2[0] = fTemp23;
			let mut fTemp24: F32 = fTemp17 + -0.33333334;
			let mut fTemp25: F32 = F32::max(0.0, F32::min(1.0, 1.5 * fTemp24));
			let mut fTemp26: F32 = F32::powf(1e+01, 0.05 * (fSlow35 + fSlow34 * F32::powf(fTemp25, 1.5)));
			let mut fTemp27: F32 = 0.8 * fTemp26;
			let mut fTemp28: F32 = F32::tanh(fTemp27);
			let mut fTemp29: F32 = if iSlow20 != 0 {F32::max(-1.0, F32::min(1.0, fTemp27))} else {if iSlow21 != 0 {fTemp28 * ((fTemp27 < 0.0) as i32) as u32 as F32 + (i32::wrapping_mul(2, (fTemp27 >= 0.0) as i32)) as F32 * F32::tanh(0.4 * fTemp26)} else {fTemp28}};
			let mut fTemp30: F32 = F32::powf(1e+01, -(fSlow37 * fTemp25));
			self.fRec26[0] = -((self.fRec26[1] * fSlow46 - (fTemp23 * fSlow45 + self.fVec2[1] * fSlow44)) / fSlow42);
			let mut fTemp31: F32 = self.fRec26[0] * fSlow26;
			self.fVec3[0] = fTemp31;
			self.fRec25[0] = -((self.fRec25[1] * fSlow47 - (fTemp31 * fSlow31 + fSlow32 * self.fVec3[1])) / fSlow39);
			let mut fTemp32: F32 = self.fRec25[0] * fTemp26;
			self.fVec4[0] = fTemp32;
			let mut fTemp33: F32 = fTemp32 + self.fVec4[1];
			let mut fTemp34: F32 = 0.5 * fTemp33;
			let mut fTemp35: F32 = F32::tanh(fTemp34);
			let mut fTemp36: F32 = fTemp32 - self.fVec4[1];
			let mut iTemp37: i32 = (F32::abs(fTemp36) > 0.0001) as i32;
			let mut fTemp38: F32 = (i32::wrapping_sub(1, iTemp37)) as F32;
			let mut fTemp39: F32 = F32::abs(self.fVec4[1]);
			let mut fTemp40: F32 = fTemp39 + F32::log(F32::exp(-(2.0 * fTemp39)) + 1.0, std::f32::consts::E);
			let mut fTemp41: F32 = F32::abs(fTemp32);
			let mut fTemp42: F32 = fTemp41 + F32::log(F32::exp(-(2.0 * fTemp41)) + 1.0, std::f32::consts::E);
			let mut fTemp43: F32 = fTemp32 * self.fVec4[1];
			let mut fTemp44: F32 = F32::max(fTemp41, fTemp39);
			let mut fTemp45: F32 = F32::exp(-(2.0 * fTemp44)) + 1.0;
			let mut fTemp46: F32 = F32::min(fTemp41, fTemp39);
			let mut fTemp47: F32 = fTemp46 - fTemp44;
			let mut fTemp48: F32 = 2.0 * fTemp47;
			let mut fTemp49: F32 = F32::abs(fTemp48);
			let mut fTemp50: F32 = F32::exp(-(2.0 * fTemp46)) * (2.0 * fTemp47 * ((fTemp49 < 0.001) as i32) as u32 as F32 * (fTemp47 * (0.6666667 * fTemp47 + 1.0) + 1.0) + ((fTemp49 >= 0.001) as i32) as u32 as F32 * (F32::exp(F32::max(fTemp48, -8e+01)) + -1.0));
			let mut fTemp51: F32 = fTemp50 / fTemp45;
			let mut fTemp52: F32 = F32::abs(-fTemp51);
			let mut fTemp53: F32 = fTemp41 + (i32::wrapping_add(i32::wrapping_mul(2, (fTemp41 <= fTemp39) as i32), -1)) as F32 * (((fTemp52 >= 0.001) as i32) as u32 as F32 * F32::log(1.0 - fTemp51, std::f32::consts::E) - fTemp50 * ((fTemp52 < 0.001) as i32) as u32 as F32 * (fTemp50 * (0.33333334 * fTemp51 + 0.5) / fTemp45 + 1.0) / fTemp45) - fTemp39;
			let mut fTemp54: F32 = F32::abs(0.5 * self.fVec4[1]);
			let mut iTemp55: i32 = (self.fVec4[1] >= 0.0) as i32;
			let mut iTemp56: i32 = (self.fVec4[1] < 0.0) as i32;
			let mut fTemp57: F32 = F32::abs(0.5 * fTemp32);
			let mut iTemp58: i32 = (fTemp32 >= 0.0) as i32;
			let mut iTemp59: i32 = (fTemp32 < 0.0) as i32;
			let mut iTemp60: i32 = i32::wrapping_mul(iTemp58, iTemp55);
			let mut iTemp61: i32 = i32::wrapping_mul(iTemp59, iTemp56);
			let mut fTemp62: F32 = F32::max(fTemp57, fTemp54);
			let mut fTemp63: F32 = F32::exp(-(2.0 * fTemp62)) + 1.0;
			let mut fTemp64: F32 = F32::min(fTemp57, fTemp54);
			let mut fTemp65: F32 = fTemp64 - fTemp62;
			let mut fTemp66: F32 = 2.0 * fTemp65;
			let mut fTemp67: F32 = F32::abs(fTemp66);
			let mut fTemp68: F32 = F32::exp(-(2.0 * fTemp64)) * (2.0 * fTemp65 * ((fTemp67 < 0.001) as i32) as u32 as F32 * (fTemp65 * (0.6666667 * fTemp65 + 1.0) + 1.0) + ((fTemp67 >= 0.001) as i32) as u32 as F32 * (F32::exp(F32::max(fTemp66, -8e+01)) + -1.0));
			let mut fTemp69: F32 = fTemp68 / fTemp63;
			let mut fTemp70: F32 = F32::abs(-fTemp69);
			let mut iTemp71: i32 = (fTemp39 <= 1.0) as i32;
			let mut iTemp72: i32 = (fTemp41 <= 1.0) as i32;
			let mut iTemp73: i32 = i32::wrapping_mul(iTemp72, iTemp71);
			let mut fTemp74: F32 = (iTemp37) as F32 * if iSlow20 != 0 {0.5 * fTemp36 * (iTemp73) as F32 * fTemp33 + (i32::wrapping_sub(1, iTemp73)) as F32 * (((fTemp41 > 1.0) as i32) as u32 as F32 * (fTemp41 + -0.5) + 0.5 * (FxDsp_faustpower2_f(self.fRec25[0]) * FxDsp_faustpower2_f(fTemp26) * (iTemp72) as F32 - FxDsp_faustpower2_f(self.fVec4[1]) * (iTemp71) as F32) - ((fTemp39 > 1.0) as i32) as u32 as F32 * (fTemp39 + -0.5))} else {if iSlow21 != 0 {fTemp53 * (iTemp61) as F32 + (i32::wrapping_mul(4, iTemp60)) as F32 * (fTemp57 + (i32::wrapping_add(i32::wrapping_mul(2, (fTemp57 <= fTemp54) as i32), -1)) as F32 * (((fTemp70 >= 0.001) as i32) as u32 as F32 * F32::log(1.0 - fTemp69, std::f32::consts::E) - fTemp68 * ((fTemp70 < 0.001) as i32) as u32 as F32 * (fTemp68 * (0.33333334 * fTemp69 + 0.5) / fTemp63 + 1.0) / fTemp63) - fTemp54) + (i32::wrapping_sub(1, i32::wrapping_add(iTemp61, iTemp60))) as F32 * ((fTemp42 + -0.6931472) * (iTemp59) as F32 + (i32::wrapping_mul(4, iTemp58)) as F32 * (fTemp57 + F32::log(F32::exp(-(2.0 * fTemp57)) + 1.0, std::f32::consts::E) + -0.6931472) - ((fTemp40 + -0.6931472) * (iTemp56) as F32 + (i32::wrapping_mul(4, iTemp55)) as F32 * (fTemp54 + F32::log(F32::exp(-(2.0 * fTemp54)) + 1.0, std::f32::consts::E) + -0.6931472)))} else {((fTemp43 >= 0.0) as i32) as u32 as F32 * fTemp53 + ((fTemp43 < 0.0) as i32) as u32 as F32 * (fTemp42 - fTemp40)}} / (fTemp32 + fTemp38 - self.fVec4[1]) + fTemp38 * if iSlow20 != 0 {F32::max(-1.0, F32::min(1.0, fTemp34))} else {if iSlow21 != 0 {fTemp35 * ((fTemp34 < 0.0) as i32) as u32 as F32 + (i32::wrapping_mul(2, (fTemp34 >= 0.0) as i32)) as F32 * F32::tanh(0.25 * fTemp33)} else {fTemp35}};
			let mut fTemp75: F32 = if (F32::abs(fTemp74) > 1e+06) as i32 != 0 {0.0} else {fTemp74};
			self.fVec5[0] = fTemp75 * fTemp30 / fTemp29;
			self.fRec24[0] = (0.8 * (fSlow47 * self.fVec5[1] + fSlow39 * fTemp75 * fTemp30 / fTemp29) - self.fRec24[1] * fSlow32) / fSlow31;
			self.fVec6[0] = self.fRec24[0] / fSlow26;
			self.fRec23[0] = -((self.fRec23[1] * fSlow49 - (self.fVec6[1] * fSlow48 + self.fRec24[0] * fSlow28 / fSlow26)) / fSlow25);
			self.fRec22[0] = -(self.fConst8 * (self.fConst7 * self.fRec22[1] - (self.fRec23[0] + self.fRec23[1])));
			let mut fTemp76: F32 = self.fRec23[0] * (1.0 - fSlow36 * fTemp25) + fSlow36 * self.fRec22[0] * fTemp25;
			self.fVec7[0] = fTemp76;
			self.fRec27[0] = -(self.fConst11 * (self.fConst10 * self.fRec27[1] - self.fConst9 * (fTemp76 - self.fVec7[1])));
			let mut fTemp77: F32 = if iSlow19 != 0 {self.fRec27[0]} else {fTemp76};
			let mut fTemp78: F32 = F32::max(0.0, F32::min(1.0, 3.0 * fTemp17)) + F32::max(0.0, F32::min(1.0, 6.0 * fTemp24));
			let mut fTemp79: F32 = if (fTemp17 > 0.0) as i32 != 0 {fTemp23 * (1.0 - 0.5 * fTemp78) + 0.5 * fTemp78 * if (F32::abs(fTemp77) > 1e+06) as i32 != 0 {0.0} else {fTemp77}} else {fTemp23};
			let mut fTemp80: F32 = F32::powf(fTemp15, 0.7);
			let mut fTemp81: F32 = self.fRec28[1] + self.fConst2 * F32::max(1.0, F32::min(self.fConst12, fSlow51 * F32::powf(3.457944, fTemp15)));
			self.fRec28[0] = if iTemp0 != 0 {0.0} else {fTemp81 - F32::floor(fTemp81)};
			let mut fTemp82: F32 = F32::sin(6.2831855 * self.fRec28[0]);
			let mut fTemp83: F32 = if iTemp16 != 0 {fTemp79 * (1.0 - fTemp80 + fTemp82 * fTemp80)} else {fTemp79};
			let mut fTemp84: F32 = F32::sqrt(fTemp13);
			let mut fTemp85: F32 = fSlow64 * self.fRec29[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 8191) as usize] + fSlow62 * self.fRec29[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 8191) as usize];
			self.fVec8[0] = fTemp85;
			self.fRec34[0] = self.fVec8[1] - fSlow56 * (self.fRec34[1] - fTemp85);
			self.fRec33[0] = self.fRec34[1] - fSlow56 * (self.fRec33[1] - self.fRec34[0]);
			self.fRec32[0] = self.fRec33[1] - fSlow56 * (self.fRec32[1] - self.fRec33[0]);
			self.fRec31[0] = self.fRec32[1] - fSlow56 * (self.fRec31[1] - self.fRec32[0]);
			self.fRec30[0] = -(self.fConst19 * (self.fConst18 * self.fRec30[1] - (self.fRec31[0] + self.fRec31[1])));
			self.fRec29[(self.IOTA0 & 8191) as usize] = 0.96 * self.fRec30[0] * fTemp84 + 2.0 * fTemp83 * (1.0 - 0.96 * fTemp84);
			let mut fTemp86: F32 = self.fRec29[(self.IOTA0 & 8191) as usize];
			let mut fTemp87: F32 = fTemp13 * if (F32::abs(fTemp86) > 1e+06) as i32 != 0 {0.0} else {fTemp86};
			let mut fTemp88: F32 = 1.0 - fTemp13;
			let mut fTemp89: F32 = if iTemp14 != 0 {fTemp83 * fTemp88 + fTemp87} else {fTemp83};
			let mut fTemp90: F32 = F32::max(0.0, F32::min(1.0, 0.00025006253 * (fTemp11 + -1.6e+04)));
			let mut fTemp91: F32 = ((fTemp11 < 19999.0) as i32) as u32 as F32;
			if (fTemp91 != 0.0) as i32 != 0 {
				fTemp92 = F32::abs(0.5 * self.fRec39[1]);
				fTemp93 = ((fTemp92 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp92)) + ((fTemp92 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp92) / F32::max(fTemp92, 0.001);
				fTemp94 = F32::max(1e+01, F32::min(self.fConst12, fTemp11));
				fTemp95 = F32::tan(self.fConst20 * fTemp94);
				fTemp96 = fTemp95 * fTemp93;
				fTemp97 = fTemp96 + 1.0;
				fTemp98 = F32::abs(0.5 * self.fRec42[1]);
				fTemp99 = ((fTemp98 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp98)) + ((fTemp98 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp98) / F32::max(fTemp98, 0.001);
				fTemp100 = fTemp95 * fTemp99;
				fTemp101 = fTemp100 + 1.0;
				fTemp102 = F32::abs(0.5 * self.fRec41[1]);
				fTemp103 = ((fTemp102 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp102)) + ((fTemp102 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp102) / F32::max(fTemp102, 0.001);
				fTemp104 = fTemp95 * fTemp103;
				fTemp105 = fTemp104 + 1.0;
				fTemp106 = F32::abs(0.5 * self.fRec40[1]);
				fTemp107 = ((fTemp106 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp106)) + ((fTemp106 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp106) / F32::max(fTemp106, 0.001);
				fTemp108 = fTemp95 * fTemp107;
				fTemp109 = fTemp108 + 1.0;
				fTemp110 = fTemp97 * fTemp109 * fTemp105;
				fTemp111 = F32::abs(self.fRec43[1]);
				fTemp112 = ((fTemp111 < 0.001) as i32) as u32 as F32 * (1.0 - 0.33333334 * FxDsp_faustpower2_f(fTemp111)) + ((fTemp111 >= 0.001) as i32) as u32 as F32 * F32::tanh(fTemp111) / F32::max(fTemp111, 0.001);
				self.fRec45[0] = fSlow65 * fTemp3 + self.fConst1 * fTemp2 * self.fRec45[1];
				self.fRec46[0] = fSlow66 * fTemp3 + self.fConst1 * fTemp2 * self.fRec46[1];
				fTemp113 = FxDsp_faustpower4_f(1.0 - F32::powf(F32::min(0.5, 0.54970527 / fTemp94), F32::powf(F32::max(0.0, F32::min(1.0, if (self.fRec45[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec46[0] + self.fRec45[0] * fTemp9 * fTemp8 * fTemp7 * fTemp6))} else {self.fRec46[0]})), 1.5)));
				fTemp114 = FxDsp_faustpower4_f(fTemp95) * fTemp93;
				fTemp115 = fTemp101 * (4.0 * (fTemp114 * fTemp113 * fTemp112 * fTemp107 * fTemp103 * fTemp99 / (fTemp110 * fTemp101)) + 1.0);
				fTemp116 = self.fRec38[1] + fTemp100 * (self.fRec37[1] + fTemp104 * (self.fRec36[1] + fTemp108 * self.fRec35[1] / fTemp97) / fTemp109) / fTemp105 + fTemp114 * fTemp89 * fTemp107 * fTemp103 * fTemp99 / fTemp110;
				fTemp117 = fTemp113 * fTemp112;
				fTemp118 = 4.0 * (fTemp117 * fTemp116 / fTemp115);
				fTemp119 = self.fRec35[1] + fTemp96 * (fTemp89 - fTemp118);
				fTemp120 = fTemp119 / fTemp97;
				fTemp121 = 2.0 * fTemp120 - self.fRec35[1];
				self.fRec35[0] = if (F32::abs(fTemp121) > 1e+06) as i32 != 0 {0.0} else {fTemp121};
				fTemp122 = self.fRec36[1] + fTemp108 * fTemp119 / fTemp97;
				fTemp123 = fTemp122 / fTemp109;
				fTemp124 = 2.0 * fTemp123 - self.fRec36[1];
				self.fRec36[0] = if (F32::abs(fTemp124) > 1e+06) as i32 != 0 {0.0} else {fTemp124};
				fTemp125 = (self.fRec37[1] + fTemp104 * fTemp122 / fTemp109) / fTemp105;
				fTemp126 = 2.0 * fTemp125 - self.fRec37[1];
				self.fRec37[0] = if (F32::abs(fTemp126) > 1e+06) as i32 != 0 {0.0} else {fTemp126};
				fTemp127 = fTemp116 / fTemp115;
				fTemp128 = 2.0 * fTemp127 - self.fRec38[1];
				self.fRec38[0] = if (F32::abs(fTemp128) > 1e+06) as i32 != 0 {0.0} else {fTemp128};
				self.fRec39[0] = fTemp89 - (fTemp118 + fTemp120);
				self.fRec40[0] = fTemp120 - fTemp123;
				self.fRec41[0] = fTemp123 - fTemp125;
				self.fRec42[0] = fTemp125 - fTemp127;
				self.fRec43[0] = fTemp127;
				fRec44 = fTemp116 * (2.0 * fTemp117 + 1.0) / fTemp115;
				fTemp129 = if (F32::abs(fRec44) > 1e+06) as i32 != 0 {0.0} else {fRec44} * fTemp91;
			}
				let mut fTemp130: F32 = (1.0 - fTemp90) * fTemp91 * fTemp129;
				let mut fTemp131: F32 = F32::max(0.0, 2e+01 * F32::log10(F32::max(1.1754944e-38, F32::abs(if iTemp12 != 0 {fTemp89} else {fTemp130 + fTemp90 * fTemp89}) + 1e-09)) - if iTemp4 != 0 {-(4e+01 * fTemp10)} else {self.fRec1[0]});
				let mut fTemp132: F32 = if ((0.75 * fTemp131) > self.fRec9[1]) as i32 != 0 {fSlow73} else {fSlow72};
				self.fRec9[0] = self.fRec9[1] * fTemp132 + 0.75 * fTemp131 * (1.0 - fTemp132);
				let mut fTemp133: F32 = if iTemp16 != 0 {fTemp79 * (1.0 - fTemp80 * (1.0 - fTemp82))} else {fTemp79};
				let mut fTemp134: F32 = if iTemp14 != 0 {fTemp87 + fTemp88 * fTemp133} else {fTemp133};
				*output0 = if iTemp12 != 0 {fTemp134} else {fTemp130 + fTemp90 * fTemp134} * F32::powf(1e+01, -0.05 * (self.fRec9[0] + 0.375 * if iTemp4 != 0 {-4e+01 * fTemp10} else {self.fRec1[0]}));
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
				self.fRec10[1] = self.fRec10[0];
				self.fRec11[1] = self.fRec11[0];
				self.fRec12[1] = self.fRec12[0];
				self.fRec13[1] = self.fRec13[0];
				self.fRec14[1] = self.fRec14[0];
				self.fRec15[1] = self.fRec15[0];
				self.fRec16[1] = self.fRec16[0];
				self.fRec17[1] = self.fRec17[0];
				self.fRec18[1] = self.fRec18[0];
				self.fRec19[1] = self.fRec19[0];
				self.fRec21[1] = self.fRec21[0];
				self.fRec20[1] = self.fRec20[0];
				self.fVec2[1] = self.fVec2[0];
				self.fRec26[1] = self.fRec26[0];
				self.fVec3[1] = self.fVec3[0];
				self.fRec25[1] = self.fRec25[0];
				self.fVec4[1] = self.fVec4[0];
				self.fVec5[1] = self.fVec5[0];
				self.fRec24[1] = self.fRec24[0];
				self.fVec6[1] = self.fVec6[0];
				self.fRec23[1] = self.fRec23[0];
				self.fRec22[1] = self.fRec22[0];
				self.fVec7[1] = self.fVec7[0];
				self.fRec27[1] = self.fRec27[0];
				self.fRec28[1] = self.fRec28[0];
				self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
				self.fVec8[1] = self.fVec8[0];
				self.fRec34[1] = self.fRec34[0];
				self.fRec33[1] = self.fRec33[0];
				self.fRec32[1] = self.fRec32[0];
				self.fRec31[1] = self.fRec31[0];
				self.fRec30[1] = self.fRec30[0];
				if (fTemp91 != 0.0) as i32 != 0 {
					self.fRec45[1] = self.fRec45[0];
					self.fRec46[1] = self.fRec46[0];
					self.fRec35[1] = self.fRec35[0];
					self.fRec36[1] = self.fRec36[0];
					self.fRec37[1] = self.fRec37[0];
					self.fRec38[1] = self.fRec38[0];
					self.fRec39[1] = self.fRec39[0];
					self.fRec40[1] = self.fRec40[0];
					self.fRec41[1] = self.fRec41[0];
					self.fRec42[1] = self.fRec42[0];
					self.fRec43[1] = self.fRec43[0];
				}
					self.fRec9[1] = self.fRec9[0];
				}
				self.fTempPerm0 = fTemp92;
				self.fTempPerm1 = fTemp93;
				self.fTempPerm2 = fTemp94;
				self.fTempPerm3 = fTemp95;
				self.fTempPerm4 = fTemp96;
				self.fTempPerm5 = fTemp97;
				self.fTempPerm6 = fTemp98;
				self.fTempPerm7 = fTemp99;
				self.fTempPerm8 = fTemp100;
				self.fTempPerm9 = fTemp101;
				self.fTempPerm10 = fTemp102;
				self.fTempPerm11 = fTemp103;
				self.fTempPerm12 = fTemp104;
				self.fTempPerm13 = fTemp105;
				self.fTempPerm14 = fTemp106;
				self.fTempPerm15 = fTemp107;
				self.fTempPerm16 = fTemp108;
				self.fTempPerm17 = fTemp109;
				self.fTempPerm18 = fTemp110;
				self.fTempPerm19 = fTemp111;
				self.fTempPerm20 = fTemp112;
				self.fTempPerm21 = fTemp113;
				self.fTempPerm22 = fTemp114;
				self.fTempPerm23 = fTemp115;
				self.fTempPerm24 = fTemp116;
				self.fTempPerm25 = fTemp117;
				self.fTempPerm26 = fTemp118;
				self.fTempPerm27 = fTemp119;
				self.fTempPerm28 = fTemp120;
				self.fTempPerm29 = fTemp121;
				self.fTempPerm30 = fTemp122;
				self.fTempPerm31 = fTemp123;
				self.fTempPerm32 = fTemp124;
				self.fTempPerm33 = fTemp125;
				self.fTempPerm34 = fTemp126;
				self.fTempPerm35 = fTemp127;
				self.fTempPerm36 = fTemp128;
				self.fTempPerm37 = fTemp129;
			}

}

