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
	fConst0: F32,
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
	fHslider6: F32,
	fConst8: F32,
	iRec101: [i32;2],
	iTempPerm0: i32,
	iTempPerm1: i32,
	iTempPerm2: i32,
	iRec102: [i32;2],
	IOTA0: i32,
	fRec100: [F32;8192],
	fTempPerm3: F32,
	fConst9: F32,
	fConst10: F32,
	fTempPerm4: F32,
	fVec2: [F32;2],
	fTempPerm5: F32,
	fVec3: [F32;2],
	fTempPerm6: F32,
	fVec4: [F32;2],
	fRec106: [F32;2],
	fRec105: [F32;2],
	fRec104: [F32;2],
	fTempPerm7: F32,
	fRec99: [F32;16384],
	fTempPerm8: F32,
	fConst11: F32,
	fConst12: F32,
	fTempPerm9: F32,
	fVec5: [F32;2],
	fConst13: F32,
	fConst14: F32,
	fConst15: F32,
	fRec98: [F32;2],
	fTempPerm10: F32,
}

impl FaustDsp for BodyDsp {
	type T = F32;
		
	fn new() -> BodyDsp { 
		BodyDsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			fSampleRate: 0,
			fConst0: 0.0,
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
			fHslider6: 0.0,
			fConst8: 0.0,
			iRec101: [0;2],
			iTempPerm0: 0,
			iTempPerm1: 0,
			iTempPerm2: 0,
			iRec102: [0;2],
			IOTA0: 0,
			fRec100: [0.0;8192],
			fTempPerm3: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fTempPerm4: 0.0,
			fVec2: [0.0;2],
			fTempPerm5: 0.0,
			fVec3: [0.0;2],
			fTempPerm6: 0.0,
			fVec4: [0.0;2],
			fRec106: [0.0;2],
			fRec105: [0.0;2],
			fRec104: [0.0;2],
			fTempPerm7: 0.0,
			fRec99: [0.0;16384],
			fTempPerm8: 0.0,
			fConst11: 0.0,
			fConst12: 0.0,
			fTempPerm9: 0.0,
			fVec5: [0.0;2],
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fRec98: [0.0;2],
			fTempPerm10: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn BodyDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("delays.lib/fdelaylti:author", r"Julius O. Smith III");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
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
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.0");
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
		self.fHslider6 = 0.0;
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
		for l100 in 0..2 {
			self.iRec101[l100 as usize] = 0;
		}
		self.iTempPerm0 = 0;
		self.iTempPerm1 = 0;
		self.iTempPerm2 = 0;
		for l101 in 0..2 {
			self.iRec102[l101 as usize] = 0;
		}
		self.IOTA0 = 0;
		for l102 in 0..8192 {
			self.fRec100[l102 as usize] = 0.0;
		}
		self.fTempPerm3 = 0.0;
		self.fTempPerm4 = 0.0;
		for l103 in 0..2 {
			self.fVec2[l103 as usize] = 0.0;
		}
		self.fTempPerm5 = 0.0;
		for l104 in 0..2 {
			self.fVec3[l104 as usize] = 0.0;
		}
		self.fTempPerm6 = 0.0;
		for l105 in 0..2 {
			self.fVec4[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec106[l106 as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec105[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec104[l108 as usize] = 0.0;
		}
		self.fTempPerm7 = 0.0;
		for l109 in 0..16384 {
			self.fRec99[l109 as usize] = 0.0;
		}
		self.fTempPerm8 = 0.0;
		self.fTempPerm9 = 0.0;
		for l110 in 0..2 {
			self.fVec5[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec98[l111 as usize] = 0.0;
		}
		self.fTempPerm10 = 0.0;
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::exp(-(3333.3333 / self.fConst0));
		self.fConst2 = 6.907755 / self.fConst0;
		self.fConst3 = 0.45 * self.fConst0;
		self.fConst4 = 6.2831855 / self.fConst0;
		self.fConst5 = 1.0 / F32::tan(31.415926 / self.fConst0);
		self.fConst6 = 1.0 - self.fConst5;
		self.fConst7 = 1.0 / (self.fConst5 + 1.0);
		self.fConst8 = F32::max(2e+01, 0.00012218964 * self.fConst0);
		self.fConst9 = 0.25 * self.fConst0;
		self.fConst10 = 50.265484 / self.fConst0;
		self.fConst11 = F32::tanh(0.5);
		self.fConst12 = 0.33333334 / (1.0 - BodyDsp_faustpower2_f(self.fConst11));
		self.fConst13 = 1.0 / F32::tan(62.831852 / self.fConst0);
		self.fConst14 = 1.0 - self.fConst13;
		self.fConst15 = 1.0 / (self.fConst13 + 1.0);
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
		ui_interface.add_horizontal_slider("body_model", ParamIndex(5), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("body_position", ParamIndex(6), 0.3, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("body_t60", ParamIndex(7), 0.4, 0.0, 1e+02, 0.0001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			2 => Some(self.fButton0),
			4 => Some(self.fHslider0),
			0 => Some(self.fHslider1),
			7 => Some(self.fHslider2),
			1 => Some(self.fHslider3),
			3 => Some(self.fHslider4),
			6 => Some(self.fHslider5),
			5 => Some(self.fHslider6),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			2 => { self.fButton0 = value }
			4 => { self.fHslider0 = value }
			0 => { self.fHslider1 = value }
			7 => { self.fHslider2 = value }
			1 => { self.fHslider3 = value }
			3 => { self.fHslider4 = value }
			6 => { self.fHslider5 = value }
			5 => { self.fHslider6 = value }
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
		let mut fSlow17: F32 = F32::min(1.0, fSlow15);
		let mut fSlow18: F32 = (iSlow16) as F32;
		let mut fSlow19: F32 = F32::powf(16.0, fSlow18 * F32::max(0.0, fSlow17)) + -1.0;
		let mut fSlow20: F32 = self.fHslider4;
		let mut fSlow21: F32 = fSlow20 * fSlow8;
		let mut fSlow22: F32 = ((fSlow21 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow23: F32 = 1.5 * fSlow10;
		let mut fSlow24: F32 = F32::powf(fSlow8, fSlow23);
		let mut fSlow25: F32 = self.fHslider5;
		let mut fSlow26: F32 = 0.65 * F32::abs(F32::sin(75.398224 * fSlow25)) + 0.35;
		let mut fSlow27: F32 = BodyDsp_faustpower2_f(fSlow26);
		let mut fSlow28: F32 = 1.7318325 * fSlow6 + 2.7995958 * fSlow5;
		let mut fSlow29: F32 = F32::exp(fSlow28 + 5.495593 * fSlow3);
		let mut fSlow30: F32 = fSlow20 * fSlow29;
		let mut fSlow31: F32 = ((fSlow30 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow32: F32 = F32::powf(fSlow29, fSlow23);
		let mut fSlow33: F32 = 0.65 * F32::abs(F32::sin(72.25663 * fSlow25)) + 0.35;
		let mut fSlow34: F32 = BodyDsp_faustpower2_f(fSlow33);
		let mut fSlow35: F32 = 1.7143383 * fSlow6 + 2.7725887 * fSlow5;
		let mut fSlow36: F32 = F32::exp(fSlow35 + 5.4086237 * fSlow3);
		let mut fSlow37: F32 = fSlow20 * fSlow36;
		let mut fSlow38: F32 = ((fSlow37 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow39: F32 = F32::powf(fSlow36, fSlow23);
		let mut fSlow40: F32 = 0.65 * F32::abs(F32::sin(69.115036 * fSlow25)) + 0.35;
		let mut fSlow41: F32 = BodyDsp_faustpower2_f(fSlow40);
		let mut fSlow42: F32 = 1.7119945 * fSlow6 + 2.6579492 * fSlow5;
		let mut fSlow43: F32 = F32::exp(fSlow42 + 5.3176985 * fSlow3);
		let mut fSlow44: F32 = fSlow20 * fSlow43;
		let mut fSlow45: F32 = ((fSlow44 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow46: F32 = F32::powf(fSlow43, fSlow23);
		let mut fSlow47: F32 = 0.65 * F32::abs(F32::sin(65.97344 * fSlow25)) + 0.35;
		let mut fSlow48: F32 = BodyDsp_faustpower2_f(fSlow47);
		let mut fSlow49: F32 = 1.6886187 * fSlow6 + 2.6436894 * fSlow5;
		let mut fSlow50: F32 = F32::exp(fSlow49 + 5.2224402 * fSlow3);
		let mut fSlow51: F32 = fSlow20 * fSlow50;
		let mut fSlow52: F32 = ((fSlow51 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow53: F32 = F32::powf(fSlow50, fSlow23);
		let mut fSlow54: F32 = 0.65 * F32::abs(F32::sin(62.831852 * fSlow25)) + 0.35;
		let mut fSlow55: F32 = BodyDsp_faustpower2_f(fSlow54);
		let mut fSlow56: F32 = 1.6353005 * fSlow6 + 2.5968952 * fSlow5;
		let mut fSlow57: F32 = F32::exp(fSlow56 + 5.1224213 * fSlow3);
		let mut fSlow58: F32 = fSlow20 * fSlow57;
		let mut fSlow59: F32 = ((fSlow58 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow60: F32 = F32::powf(fSlow57, fSlow23);
		let mut fSlow61: F32 = 0.65 * F32::abs(F32::sin(59.69026 * fSlow25)) + 0.35;
		let mut fSlow62: F32 = BodyDsp_faustpower2_f(fSlow61);
		let mut fSlow63: F32 = 1.6260984 * fSlow6 + 2.5529547 * fSlow5;
		let mut fSlow64: F32 = F32::exp(fSlow63 + 5.017134 * fSlow3);
		let mut fSlow65: F32 = fSlow20 * fSlow64;
		let mut fSlow66: F32 = ((fSlow65 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow67: F32 = F32::powf(fSlow64, fSlow23);
		let mut fSlow68: F32 = 0.65 * F32::abs(F32::sin(56.548668 * fSlow25)) + 0.35;
		let mut fSlow69: F32 = BodyDsp_faustpower2_f(fSlow68);
		let mut fSlow70: F32 = 1.5898472 * fSlow6 + 2.5402625 * fSlow5;
		let mut fSlow71: F32 = F32::exp(fSlow70 + 4.905993 * fSlow3);
		let mut fSlow72: F32 = fSlow20 * fSlow71;
		let mut fSlow73: F32 = ((fSlow72 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow74: F32 = F32::powf(fSlow71, fSlow23);
		let mut fSlow75: F32 = 0.65 * F32::abs(F32::sin(53.407074 * fSlow25)) + 0.35;
		let mut fSlow76: F32 = BodyDsp_faustpower2_f(fSlow75);
		let mut fSlow77: F32 = 1.5752605 * fSlow6 + 2.4726653 * fSlow5;
		let mut fSlow78: F32 = F32::exp(fSlow77 + 4.7883162 * fSlow3);
		let mut fSlow79: F32 = fSlow20 * fSlow78;
		let mut fSlow80: F32 = ((fSlow79 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow81: F32 = F32::powf(fSlow78, fSlow23);
		let mut fSlow82: F32 = 0.65 * F32::abs(F32::sin(50.265484 * fSlow25)) + 0.35;
		let mut fSlow83: F32 = BodyDsp_faustpower2_f(fSlow82);
		let mut fSlow84: F32 = 1.5282278 * fSlow6 + 2.3919687 * fSlow5;
		let mut fSlow85: F32 = F32::exp(fSlow84 + 4.663269 * fSlow3);
		let mut fSlow86: F32 = fSlow20 * fSlow85;
		let mut fSlow87: F32 = ((fSlow86 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow88: F32 = F32::powf(fSlow85, fSlow23);
		let mut fSlow89: F32 = 0.65 * F32::abs(F32::sin(47.12389 * fSlow25)) + 0.35;
		let mut fSlow90: F32 = BodyDsp_faustpower2_f(fSlow89);
		let mut fSlow91: F32 = 1.5262737 * fSlow6 + 2.367811 * fSlow5;
		let mut fSlow92: F32 = F32::exp(fSlow91 + 4.5298862 * fSlow3);
		let mut fSlow93: F32 = fSlow20 * fSlow92;
		let mut fSlow94: F32 = ((fSlow93 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow95: F32 = F32::powf(fSlow92, fSlow23);
		let mut fSlow96: F32 = 0.65 * F32::abs(F32::sin(43.982296 * fSlow25)) + 0.35;
		let mut fSlow97: F32 = BodyDsp_faustpower2_f(fSlow96);
		let mut fSlow98: F32 = 1.442202 * fSlow6 + 2.2827904 * fSlow5;
		let mut fSlow99: F32 = F32::exp(fSlow98 + 4.3869767 * fSlow3);
		let mut fSlow100: F32 = fSlow20 * fSlow99;
		let mut fSlow101: F32 = ((fSlow100 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow102: F32 = F32::powf(fSlow99, fSlow23);
		let mut fSlow103: F32 = 0.65 * F32::abs(F32::sin(40.840706 * fSlow25)) + 0.35;
		let mut fSlow104: F32 = BodyDsp_faustpower2_f(fSlow103);
		let mut fSlow105: F32 = 1.4187615 * fSlow6 + 2.1972246 * fSlow5;
		let mut fSlow106: F32 = F32::exp(fSlow105 + 4.233048 * fSlow3);
		let mut fSlow107: F32 = fSlow20 * fSlow106;
		let mut fSlow108: F32 = ((fSlow107 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow109: F32 = F32::powf(fSlow106, fSlow23);
		let mut fSlow110: F32 = 0.65 * F32::abs(F32::sin(37.699112 * fSlow25)) + 0.35;
		let mut fSlow111: F32 = BodyDsp_faustpower2_f(fSlow110);
		let mut fSlow112: F32 = 1.4009366 * fSlow6 + 2.032481 * fSlow5;
		let mut fSlow113: F32 = F32::exp(fSlow112 + 4.066288 * fSlow3);
		let mut fSlow114: F32 = fSlow20 * fSlow113;
		let mut fSlow115: F32 = ((fSlow114 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow116: F32 = F32::powf(fSlow113, fSlow23);
		let mut fSlow117: F32 = 0.65 * F32::abs(F32::sin(34.557518 * fSlow25)) + 0.35;
		let mut fSlow118: F32 = BodyDsp_faustpower2_f(fSlow117);
		let mut fSlow119: F32 = 1.2939049 * fSlow6 + 2.0305138 * fSlow5;
		let mut fSlow120: F32 = F32::exp(fSlow119 + 3.8843434 * fSlow3);
		let mut fSlow121: F32 = fSlow20 * fSlow120;
		let mut fSlow122: F32 = ((fSlow121 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow123: F32 = F32::powf(fSlow120, fSlow23);
		let mut fSlow124: F32 = 0.65 * F32::abs(F32::sin(31.415926 * fSlow25)) + 0.35;
		let mut fSlow125: F32 = BodyDsp_faustpower2_f(fSlow124);
		let mut fSlow126: F32 = 1.2803781 * fSlow6 + 1.9902004 * fSlow5;
		let mut fSlow127: F32 = F32::exp(fSlow126 + 3.6841683 * fSlow3);
		let mut fSlow128: F32 = fSlow20 * fSlow127;
		let mut fSlow129: F32 = ((fSlow128 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow130: F32 = F32::powf(fSlow127, fSlow23);
		let mut fSlow131: F32 = 0.65 * F32::abs(F32::sin(28.274334 * fSlow25)) + 0.35;
		let mut fSlow132: F32 = BodyDsp_faustpower2_f(fSlow131);
		let mut fSlow133: F32 = 1.2527629 * fSlow6 + 1.9153037 * fSlow5;
		let mut fSlow134: F32 = F32::exp(fSlow133 + 3.4617279 * fSlow3);
		let mut fSlow135: F32 = fSlow20 * fSlow134;
		let mut fSlow136: F32 = ((fSlow135 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow137: F32 = F32::powf(fSlow134, fSlow23);
		let mut fSlow138: F32 = 0.65 * F32::abs(F32::sin(25.132742 * fSlow25)) + 0.35;
		let mut fSlow139: F32 = BodyDsp_faustpower2_f(fSlow138);
		let mut fSlow140: F32 = 1.1489885 * fSlow6 + 1.68306 * fSlow5;
		let mut fSlow141: F32 = F32::exp(fSlow140 + 3.211408 * fSlow3);
		let mut fSlow142: F32 = fSlow20 * fSlow141;
		let mut fSlow143: F32 = ((fSlow142 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow144: F32 = F32::powf(fSlow141, fSlow23);
		let mut fSlow145: F32 = 0.65 * F32::abs(F32::sin(21.991148 * fSlow25)) + 0.35;
		let mut fSlow146: F32 = BodyDsp_faustpower2_f(fSlow145);
		let mut fSlow147: F32 = 1.0705557 * fSlow6 + 1.6382197 * fSlow5;
		let mut fSlow148: F32 = F32::exp(fSlow147 + 2.9252026 * fSlow3);
		let mut fSlow149: F32 = fSlow20 * fSlow148;
		let mut fSlow150: F32 = ((fSlow149 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow151: F32 = F32::powf(fSlow148, fSlow23);
		let mut fSlow152: F32 = 0.65 * F32::abs(F32::sin(18.849556 * fSlow25)) + 0.35;
		let mut fSlow153: F32 = BodyDsp_faustpower2_f(fSlow152);
		let mut fSlow154: F32 = 0.9756911 * fSlow6 + 1.3862944 * fSlow5;
		let mut fSlow155: F32 = F32::exp(fSlow154 + 2.5910668 * fSlow3);
		let mut fSlow156: F32 = fSlow20 * fSlow155;
		let mut fSlow157: F32 = ((fSlow156 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow158: F32 = F32::powf(fSlow155, fSlow23);
		let mut fSlow159: F32 = 0.65 * F32::abs(F32::sin(15.707963 * fSlow25)) + 0.35;
		let mut fSlow160: F32 = BodyDsp_faustpower2_f(fSlow159);
		let mut fSlow161: F32 = 0.8307328 * fSlow6 + 1.1665825 * fSlow5;
		let mut fSlow162: F32 = F32::exp(fSlow161 + 2.1897523 * fSlow3);
		let mut fSlow163: F32 = fSlow20 * fSlow162;
		let mut fSlow164: F32 = ((fSlow163 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow165: F32 = F32::powf(fSlow162, fSlow23);
		let mut fSlow166: F32 = 0.65 * F32::abs(F32::sin(12.566371 * fSlow25)) + 0.35;
		let mut fSlow167: F32 = BodyDsp_faustpower2_f(fSlow166);
		let mut fSlow168: F32 = 0.7589349 * fSlow6 + 1.154047 * fSlow5;
		let mut fSlow169: F32 = F32::exp(fSlow168 + 1.6871394 * fSlow3);
		let mut fSlow170: F32 = fSlow20 * fSlow169;
		let mut fSlow171: F32 = ((fSlow170 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow172: F32 = F32::powf(fSlow169, fSlow23);
		let mut fSlow173: F32 = 0.65 * F32::abs(F32::sin(9.424778 * fSlow25)) + 0.35;
		let mut fSlow174: F32 = BodyDsp_faustpower2_f(fSlow173);
		let mut fSlow175: F32 = 0.46561903 * fSlow6 + 0.60376936 * fSlow5;
		let mut fSlow176: F32 = F32::exp(fSlow175 + 1.0141431 * fSlow3);
		let mut fSlow177: F32 = fSlow20 * fSlow176;
		let mut fSlow178: F32 = ((fSlow177 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow179: F32 = F32::powf(fSlow176, fSlow23);
		let mut fSlow180: F32 = 0.65 * F32::abs(F32::sin(6.2831855 * fSlow25)) + 0.35;
		let mut fSlow181: F32 = BodyDsp_faustpower2_f(fSlow180);
		let mut fSlow182: F32 = ((fSlow20 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow183: F32 = BodyDsp_faustpower2_f(fSlow182);
		let mut fSlow184: F32 = F32::powf(1.0, fSlow23);
		let mut fSlow185: F32 = 0.65 * F32::abs(F32::sin(3.1415927 * fSlow25)) + 0.35;
		let mut fSlow186: F32 = BodyDsp_faustpower2_f(fSlow185);
		let mut fSlow187: F32 = F32::max(0.1, F32::sqrt(fSlow186 * BodyDsp_faustpower2_f(fSlow184) * fSlow183 + fSlow181 * BodyDsp_faustpower2_f(fSlow179) * BodyDsp_faustpower2_f(fSlow178) + fSlow174 * BodyDsp_faustpower2_f(fSlow172) * BodyDsp_faustpower2_f(fSlow171) + fSlow167 * BodyDsp_faustpower2_f(fSlow165) * BodyDsp_faustpower2_f(fSlow164) + fSlow160 * BodyDsp_faustpower2_f(fSlow158) * BodyDsp_faustpower2_f(fSlow157) + fSlow153 * BodyDsp_faustpower2_f(fSlow151) * BodyDsp_faustpower2_f(fSlow150) + fSlow146 * BodyDsp_faustpower2_f(fSlow144) * BodyDsp_faustpower2_f(fSlow143) + fSlow139 * BodyDsp_faustpower2_f(fSlow137) * BodyDsp_faustpower2_f(fSlow136) + fSlow132 * BodyDsp_faustpower2_f(fSlow130) * BodyDsp_faustpower2_f(fSlow129) + fSlow125 * BodyDsp_faustpower2_f(fSlow123) * BodyDsp_faustpower2_f(fSlow122) + fSlow118 * BodyDsp_faustpower2_f(fSlow116) * BodyDsp_faustpower2_f(fSlow115) + fSlow111 * BodyDsp_faustpower2_f(fSlow109) * BodyDsp_faustpower2_f(fSlow108) + fSlow104 * BodyDsp_faustpower2_f(fSlow102) * BodyDsp_faustpower2_f(fSlow101) + fSlow97 * BodyDsp_faustpower2_f(fSlow95) * BodyDsp_faustpower2_f(fSlow94) + fSlow90 * BodyDsp_faustpower2_f(fSlow88) * BodyDsp_faustpower2_f(fSlow87) + fSlow83 * BodyDsp_faustpower2_f(fSlow81) * BodyDsp_faustpower2_f(fSlow80) + fSlow76 * BodyDsp_faustpower2_f(fSlow74) * BodyDsp_faustpower2_f(fSlow73) + fSlow69 * BodyDsp_faustpower2_f(fSlow67) * BodyDsp_faustpower2_f(fSlow66) + fSlow62 * BodyDsp_faustpower2_f(fSlow60) * BodyDsp_faustpower2_f(fSlow59) + fSlow55 * BodyDsp_faustpower2_f(fSlow53) * BodyDsp_faustpower2_f(fSlow52) + fSlow48 * BodyDsp_faustpower2_f(fSlow46) * BodyDsp_faustpower2_f(fSlow45) + fSlow41 * BodyDsp_faustpower2_f(fSlow39) * BodyDsp_faustpower2_f(fSlow38) + fSlow34 * BodyDsp_faustpower2_f(fSlow32) * BodyDsp_faustpower2_f(fSlow31) + fSlow27 * BodyDsp_faustpower2_f(fSlow24) * BodyDsp_faustpower2_f(fSlow22)));
		let mut fSlow188: F32 = fSlow26 * fSlow24 * fSlow22;
		let mut fSlow189: F32 = F32::max(1e-09, 0.3 * (fSlow188 / fSlow187));
		let mut fSlow190: F32 = self.fConst4 * F32::min(fSlow21, self.fConst3);
		let mut fSlow191: F32 = 2.0 * fSlow13 * F32::cos(fSlow190);
		let mut fSlow192: F32 = 0.3 * (fSlow188 * F32::sin(fSlow190) / fSlow187);
		let mut fSlow193: F32 = F32::max(0.0, 1.0 - F32::abs(-2.0 * fSlow2));
		let mut fSlow194: F32 = F32::exp(fSlow7 + 5.578937 * fSlow193);
		let mut fSlow195: F32 = (((fSlow20 * fSlow194) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow196: F32 = 1.5 * (fSlow9 + -1.0);
		let mut fSlow197: F32 = F32::powf(fSlow194, fSlow196);
		let mut fSlow198: F32 = F32::exp(fSlow35 + 5.4086237 * fSlow193);
		let mut fSlow199: F32 = (((fSlow20 * fSlow198) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow200: F32 = F32::powf(fSlow198, fSlow196);
		let mut fSlow201: F32 = F32::exp(fSlow42 + 5.3176985 * fSlow193);
		let mut fSlow202: F32 = (((fSlow20 * fSlow201) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow203: F32 = F32::powf(fSlow201, fSlow196);
		let mut fSlow204: F32 = F32::exp(fSlow49 + 5.2224402 * fSlow193);
		let mut fSlow205: F32 = (((fSlow20 * fSlow204) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow206: F32 = F32::powf(fSlow204, fSlow196);
		let mut fSlow207: F32 = F32::exp(fSlow70 + 4.905993 * fSlow193);
		let mut fSlow208: F32 = (((fSlow20 * fSlow207) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow209: F32 = F32::powf(fSlow207, fSlow196);
		let mut fSlow210: F32 = F32::exp(fSlow77 + 4.7883162 * fSlow193);
		let mut fSlow211: F32 = (((fSlow20 * fSlow210) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow212: F32 = F32::powf(fSlow210, fSlow196);
		let mut fSlow213: F32 = F32::exp(fSlow84 + 4.663269 * fSlow193);
		let mut fSlow214: F32 = (((fSlow20 * fSlow213) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow215: F32 = F32::powf(fSlow213, fSlow196);
		let mut fSlow216: F32 = F32::exp(fSlow91 + 4.5298862 * fSlow193);
		let mut fSlow217: F32 = (((fSlow20 * fSlow216) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow218: F32 = F32::powf(fSlow216, fSlow196);
		let mut fSlow219: F32 = F32::exp(fSlow98 + 4.3869767 * fSlow193);
		let mut fSlow220: F32 = (((fSlow20 * fSlow219) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow221: F32 = F32::powf(fSlow219, fSlow196);
		let mut fSlow222: F32 = F32::exp(fSlow105 + 4.233048 * fSlow193);
		let mut fSlow223: F32 = (((fSlow20 * fSlow222) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow224: F32 = F32::powf(fSlow222, fSlow196);
		let mut fSlow225: F32 = F32::exp(fSlow112 + 4.066288 * fSlow193);
		let mut fSlow226: F32 = (((fSlow20 * fSlow225) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow227: F32 = F32::powf(fSlow225, fSlow196);
		let mut fSlow228: F32 = F32::exp(fSlow119 + 3.8843434 * fSlow193);
		let mut fSlow229: F32 = (((fSlow20 * fSlow228) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow230: F32 = F32::powf(fSlow228, fSlow196);
		let mut fSlow231: F32 = F32::exp(fSlow126 + 3.6841683 * fSlow193);
		let mut fSlow232: F32 = (((fSlow20 * fSlow231) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow233: F32 = F32::powf(fSlow231, fSlow196);
		let mut fSlow234: F32 = F32::exp(fSlow133 + 3.4617279 * fSlow193);
		let mut fSlow235: F32 = (((fSlow20 * fSlow234) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow236: F32 = F32::powf(fSlow234, fSlow196);
		let mut fSlow237: F32 = F32::exp(fSlow140 + 3.211408 * fSlow193);
		let mut fSlow238: F32 = (((fSlow20 * fSlow237) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow239: F32 = F32::powf(fSlow237, fSlow196);
		let mut fSlow240: F32 = F32::exp(fSlow147 + 2.9252026 * fSlow193);
		let mut fSlow241: F32 = (((fSlow20 * fSlow240) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow242: F32 = F32::powf(fSlow240, fSlow196);
		let mut fSlow243: F32 = F32::exp(fSlow154 + 2.5910668 * fSlow193);
		let mut fSlow244: F32 = (((fSlow20 * fSlow243) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow245: F32 = F32::powf(fSlow243, fSlow196);
		let mut fSlow246: F32 = F32::exp(fSlow161 + 2.1897523 * fSlow193);
		let mut fSlow247: F32 = (((fSlow20 * fSlow246) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow248: F32 = F32::powf(fSlow246, fSlow196);
		let mut fSlow249: F32 = F32::exp(fSlow168 + 1.6871394 * fSlow193);
		let mut fSlow250: F32 = (((fSlow20 * fSlow249) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow251: F32 = F32::powf(fSlow249, fSlow196);
		let mut fSlow252: F32 = F32::exp(fSlow175 + 1.0141431 * fSlow193);
		let mut fSlow253: F32 = (((fSlow20 * fSlow252) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow254: F32 = F32::powf(fSlow252, fSlow196);
		let mut fSlow255: F32 = F32::powf(1.0, fSlow196);
		let mut fSlow256: F32 = F32::exp(fSlow63 + 5.017134 * fSlow193);
		let mut fSlow257: F32 = (((fSlow20 * fSlow256) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow258: F32 = F32::powf(fSlow256, fSlow196);
		let mut fSlow259: F32 = F32::exp(fSlow56 + 5.1224213 * fSlow193);
		let mut fSlow260: F32 = (((fSlow20 * fSlow259) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow261: F32 = F32::powf(fSlow259, fSlow196);
		let mut fSlow262: F32 = F32::exp(fSlow28 + 5.495593 * fSlow193);
		let mut fSlow263: F32 = (((fSlow20 * fSlow262) < self.fConst3) as i32) as u32 as F32;
		let mut fSlow264: F32 = F32::powf(fSlow262, fSlow196);
		let mut fSlow265: F32 = F32::max(0.1, F32::sqrt(fSlow34 * BodyDsp_faustpower2_f(fSlow264) * BodyDsp_faustpower2_f(fSlow263) + fSlow62 * BodyDsp_faustpower2_f(fSlow261) * BodyDsp_faustpower2_f(fSlow260) + fSlow69 * BodyDsp_faustpower2_f(fSlow258) * BodyDsp_faustpower2_f(fSlow257) + fSlow186 * fSlow183 * BodyDsp_faustpower2_f(fSlow255) + fSlow181 * BodyDsp_faustpower2_f(fSlow254) * BodyDsp_faustpower2_f(fSlow253) + fSlow174 * BodyDsp_faustpower2_f(fSlow251) * BodyDsp_faustpower2_f(fSlow250) + fSlow167 * BodyDsp_faustpower2_f(fSlow248) * BodyDsp_faustpower2_f(fSlow247) + fSlow160 * BodyDsp_faustpower2_f(fSlow245) * BodyDsp_faustpower2_f(fSlow244) + fSlow153 * BodyDsp_faustpower2_f(fSlow242) * BodyDsp_faustpower2_f(fSlow241) + fSlow146 * BodyDsp_faustpower2_f(fSlow239) * BodyDsp_faustpower2_f(fSlow238) + fSlow139 * BodyDsp_faustpower2_f(fSlow236) * BodyDsp_faustpower2_f(fSlow235) + fSlow132 * BodyDsp_faustpower2_f(fSlow233) * BodyDsp_faustpower2_f(fSlow232) + fSlow125 * BodyDsp_faustpower2_f(fSlow230) * BodyDsp_faustpower2_f(fSlow229) + fSlow118 * BodyDsp_faustpower2_f(fSlow227) * BodyDsp_faustpower2_f(fSlow226) + fSlow111 * BodyDsp_faustpower2_f(fSlow224) * BodyDsp_faustpower2_f(fSlow223) + fSlow104 * BodyDsp_faustpower2_f(fSlow221) * BodyDsp_faustpower2_f(fSlow220) + fSlow97 * BodyDsp_faustpower2_f(fSlow218) * BodyDsp_faustpower2_f(fSlow217) + fSlow90 * BodyDsp_faustpower2_f(fSlow215) * BodyDsp_faustpower2_f(fSlow214) + fSlow83 * BodyDsp_faustpower2_f(fSlow212) * BodyDsp_faustpower2_f(fSlow211) + fSlow76 * BodyDsp_faustpower2_f(fSlow209) * BodyDsp_faustpower2_f(fSlow208) + fSlow55 * BodyDsp_faustpower2_f(fSlow206) * BodyDsp_faustpower2_f(fSlow205) + fSlow48 * BodyDsp_faustpower2_f(fSlow203) * BodyDsp_faustpower2_f(fSlow202) + fSlow41 * BodyDsp_faustpower2_f(fSlow200) * BodyDsp_faustpower2_f(fSlow199) + fSlow27 * BodyDsp_faustpower2_f(fSlow197) * BodyDsp_faustpower2_f(fSlow195)));
		let mut fSlow266: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow26 * fSlow197 * fSlow195 / fSlow265));
		let mut fSlow267: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow29, fSlow11))));
		let mut fSlow268: F32 = BodyDsp_faustpower2_f(fSlow267);
		let mut fSlow269: F32 = fSlow33 * fSlow32 * fSlow31;
		let mut fSlow270: F32 = F32::max(1e-09, 0.3 * (fSlow269 / fSlow187));
		let mut fSlow271: F32 = self.fConst4 * F32::min(fSlow30, self.fConst3);
		let mut fSlow272: F32 = 2.0 * fSlow267 * F32::cos(fSlow271);
		let mut fSlow273: F32 = 0.3 * (fSlow269 * F32::sin(fSlow271) / fSlow187);
		let mut fSlow274: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow33 * fSlow264 * fSlow263 / fSlow265));
		let mut fSlow275: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow36, fSlow11))));
		let mut fSlow276: F32 = BodyDsp_faustpower2_f(fSlow275);
		let mut fSlow277: F32 = fSlow40 * fSlow39 * fSlow38;
		let mut fSlow278: F32 = F32::max(1e-09, 0.3 * (fSlow277 / fSlow187));
		let mut fSlow279: F32 = self.fConst4 * F32::min(fSlow37, self.fConst3);
		let mut fSlow280: F32 = 2.0 * fSlow275 * F32::cos(fSlow279);
		let mut fSlow281: F32 = 0.3 * (fSlow277 * F32::sin(fSlow279) / fSlow187);
		let mut fSlow282: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow40 * fSlow200 * fSlow199 / fSlow265));
		let mut fSlow283: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow43, fSlow11))));
		let mut fSlow284: F32 = BodyDsp_faustpower2_f(fSlow283);
		let mut fSlow285: F32 = fSlow47 * fSlow46 * fSlow45;
		let mut fSlow286: F32 = F32::max(1e-09, 0.3 * (fSlow285 / fSlow187));
		let mut fSlow287: F32 = self.fConst4 * F32::min(fSlow44, self.fConst3);
		let mut fSlow288: F32 = 2.0 * fSlow283 * F32::cos(fSlow287);
		let mut fSlow289: F32 = 0.3 * (fSlow285 * F32::sin(fSlow287) / fSlow187);
		let mut fSlow290: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow47 * fSlow203 * fSlow202 / fSlow265));
		let mut fSlow291: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow50, fSlow11))));
		let mut fSlow292: F32 = BodyDsp_faustpower2_f(fSlow291);
		let mut fSlow293: F32 = fSlow54 * fSlow53 * fSlow52;
		let mut fSlow294: F32 = F32::max(1e-09, 0.3 * (fSlow293 / fSlow187));
		let mut fSlow295: F32 = self.fConst4 * F32::min(fSlow51, self.fConst3);
		let mut fSlow296: F32 = 2.0 * fSlow291 * F32::cos(fSlow295);
		let mut fSlow297: F32 = 0.3 * (fSlow293 * F32::sin(fSlow295) / fSlow187);
		let mut fSlow298: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow54 * fSlow206 * fSlow205 / fSlow265));
		let mut fSlow299: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow57, fSlow11))));
		let mut fSlow300: F32 = BodyDsp_faustpower2_f(fSlow299);
		let mut fSlow301: F32 = fSlow61 * fSlow60 * fSlow59;
		let mut fSlow302: F32 = F32::max(1e-09, 0.3 * (fSlow301 / fSlow187));
		let mut fSlow303: F32 = self.fConst4 * F32::min(fSlow58, self.fConst3);
		let mut fSlow304: F32 = 2.0 * fSlow299 * F32::cos(fSlow303);
		let mut fSlow305: F32 = 0.3 * (fSlow301 * F32::sin(fSlow303) / fSlow187);
		let mut fSlow306: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow61 * fSlow261 * fSlow260 / fSlow265));
		let mut fSlow307: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow64, fSlow11))));
		let mut fSlow308: F32 = BodyDsp_faustpower2_f(fSlow307);
		let mut fSlow309: F32 = fSlow68 * fSlow67 * fSlow66;
		let mut fSlow310: F32 = F32::max(1e-09, 0.3 * (fSlow309 / fSlow187));
		let mut fSlow311: F32 = self.fConst4 * F32::min(fSlow65, self.fConst3);
		let mut fSlow312: F32 = 2.0 * fSlow307 * F32::cos(fSlow311);
		let mut fSlow313: F32 = 0.3 * (fSlow309 * F32::sin(fSlow311) / fSlow187);
		let mut fSlow314: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow68 * fSlow258 * fSlow257 / fSlow265));
		let mut fSlow315: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow71, fSlow11))));
		let mut fSlow316: F32 = BodyDsp_faustpower2_f(fSlow315);
		let mut fSlow317: F32 = fSlow75 * fSlow74 * fSlow73;
		let mut fSlow318: F32 = F32::max(1e-09, 0.3 * (fSlow317 / fSlow187));
		let mut fSlow319: F32 = self.fConst4 * F32::min(fSlow72, self.fConst3);
		let mut fSlow320: F32 = 2.0 * fSlow315 * F32::cos(fSlow319);
		let mut fSlow321: F32 = 0.3 * (fSlow317 * F32::sin(fSlow319) / fSlow187);
		let mut fSlow322: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow75 * fSlow209 * fSlow208 / fSlow265));
		let mut fSlow323: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow78, fSlow11))));
		let mut fSlow324: F32 = BodyDsp_faustpower2_f(fSlow323);
		let mut fSlow325: F32 = fSlow82 * fSlow81 * fSlow80;
		let mut fSlow326: F32 = F32::max(1e-09, 0.3 * (fSlow325 / fSlow187));
		let mut fSlow327: F32 = self.fConst4 * F32::min(fSlow79, self.fConst3);
		let mut fSlow328: F32 = 2.0 * fSlow323 * F32::cos(fSlow327);
		let mut fSlow329: F32 = 0.3 * (fSlow325 * F32::sin(fSlow327) / fSlow187);
		let mut fSlow330: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow82 * fSlow212 * fSlow211 / fSlow265));
		let mut fSlow331: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow85, fSlow11))));
		let mut fSlow332: F32 = BodyDsp_faustpower2_f(fSlow331);
		let mut fSlow333: F32 = fSlow89 * fSlow88 * fSlow87;
		let mut fSlow334: F32 = F32::max(1e-09, 0.3 * (fSlow333 / fSlow187));
		let mut fSlow335: F32 = self.fConst4 * F32::min(fSlow86, self.fConst3);
		let mut fSlow336: F32 = 2.0 * fSlow331 * F32::cos(fSlow335);
		let mut fSlow337: F32 = 0.3 * (fSlow333 * F32::sin(fSlow335) / fSlow187);
		let mut fSlow338: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow89 * fSlow215 * fSlow214 / fSlow265));
		let mut fSlow339: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow92, fSlow11))));
		let mut fSlow340: F32 = BodyDsp_faustpower2_f(fSlow339);
		let mut fSlow341: F32 = fSlow96 * fSlow95 * fSlow94;
		let mut fSlow342: F32 = F32::max(1e-09, 0.3 * (fSlow341 / fSlow187));
		let mut fSlow343: F32 = self.fConst4 * F32::min(fSlow93, self.fConst3);
		let mut fSlow344: F32 = 2.0 * fSlow339 * F32::cos(fSlow343);
		let mut fSlow345: F32 = 0.3 * (fSlow341 * F32::sin(fSlow343) / fSlow187);
		let mut fSlow346: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow96 * fSlow218 * fSlow217 / fSlow265));
		let mut fSlow347: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow99, fSlow11))));
		let mut fSlow348: F32 = BodyDsp_faustpower2_f(fSlow347);
		let mut fSlow349: F32 = fSlow103 * fSlow102 * fSlow101;
		let mut fSlow350: F32 = F32::max(1e-09, 0.3 * (fSlow349 / fSlow187));
		let mut fSlow351: F32 = self.fConst4 * F32::min(fSlow100, self.fConst3);
		let mut fSlow352: F32 = 2.0 * fSlow347 * F32::cos(fSlow351);
		let mut fSlow353: F32 = 0.3 * (fSlow349 * F32::sin(fSlow351) / fSlow187);
		let mut fSlow354: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow103 * fSlow221 * fSlow220 / fSlow265));
		let mut fSlow355: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow106, fSlow11))));
		let mut fSlow356: F32 = BodyDsp_faustpower2_f(fSlow355);
		let mut fSlow357: F32 = fSlow110 * fSlow109 * fSlow108;
		let mut fSlow358: F32 = F32::max(1e-09, 0.3 * (fSlow357 / fSlow187));
		let mut fSlow359: F32 = self.fConst4 * F32::min(fSlow107, self.fConst3);
		let mut fSlow360: F32 = 2.0 * fSlow355 * F32::cos(fSlow359);
		let mut fSlow361: F32 = 0.3 * (fSlow357 * F32::sin(fSlow359) / fSlow187);
		let mut fSlow362: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow110 * fSlow224 * fSlow223 / fSlow265));
		let mut fSlow363: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow113, fSlow11))));
		let mut fSlow364: F32 = BodyDsp_faustpower2_f(fSlow363);
		let mut fSlow365: F32 = fSlow117 * fSlow116 * fSlow115;
		let mut fSlow366: F32 = F32::max(1e-09, 0.3 * (fSlow365 / fSlow187));
		let mut fSlow367: F32 = self.fConst4 * F32::min(fSlow114, self.fConst3);
		let mut fSlow368: F32 = 2.0 * fSlow363 * F32::cos(fSlow367);
		let mut fSlow369: F32 = 0.3 * (fSlow365 * F32::sin(fSlow367) / fSlow187);
		let mut fSlow370: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow117 * fSlow227 * fSlow226 / fSlow265));
		let mut fSlow371: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow120, fSlow11))));
		let mut fSlow372: F32 = BodyDsp_faustpower2_f(fSlow371);
		let mut fSlow373: F32 = fSlow124 * fSlow123 * fSlow122;
		let mut fSlow374: F32 = F32::max(1e-09, 0.3 * (fSlow373 / fSlow187));
		let mut fSlow375: F32 = self.fConst4 * F32::min(fSlow121, self.fConst3);
		let mut fSlow376: F32 = 2.0 * fSlow371 * F32::cos(fSlow375);
		let mut fSlow377: F32 = 0.3 * (fSlow373 * F32::sin(fSlow375) / fSlow187);
		let mut fSlow378: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow124 * fSlow230 * fSlow229 / fSlow265));
		let mut fSlow379: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow127, fSlow11))));
		let mut fSlow380: F32 = BodyDsp_faustpower2_f(fSlow379);
		let mut fSlow381: F32 = fSlow131 * fSlow130 * fSlow129;
		let mut fSlow382: F32 = F32::max(1e-09, 0.3 * (fSlow381 / fSlow187));
		let mut fSlow383: F32 = self.fConst4 * F32::min(fSlow128, self.fConst3);
		let mut fSlow384: F32 = 2.0 * fSlow379 * F32::cos(fSlow383);
		let mut fSlow385: F32 = 0.3 * (fSlow381 * F32::sin(fSlow383) / fSlow187);
		let mut fSlow386: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow131 * fSlow233 * fSlow232 / fSlow265));
		let mut fSlow387: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow134, fSlow11))));
		let mut fSlow388: F32 = BodyDsp_faustpower2_f(fSlow387);
		let mut fSlow389: F32 = fSlow138 * fSlow137 * fSlow136;
		let mut fSlow390: F32 = F32::max(1e-09, 0.3 * (fSlow389 / fSlow187));
		let mut fSlow391: F32 = self.fConst4 * F32::min(fSlow135, self.fConst3);
		let mut fSlow392: F32 = 2.0 * fSlow387 * F32::cos(fSlow391);
		let mut fSlow393: F32 = 0.3 * (fSlow389 * F32::sin(fSlow391) / fSlow187);
		let mut fSlow394: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow138 * fSlow236 * fSlow235 / fSlow265));
		let mut fSlow395: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow141, fSlow11))));
		let mut fSlow396: F32 = BodyDsp_faustpower2_f(fSlow395);
		let mut fSlow397: F32 = fSlow145 * fSlow144 * fSlow143;
		let mut fSlow398: F32 = F32::max(1e-09, 0.3 * (fSlow397 / fSlow187));
		let mut fSlow399: F32 = self.fConst4 * F32::min(fSlow142, self.fConst3);
		let mut fSlow400: F32 = 2.0 * fSlow395 * F32::cos(fSlow399);
		let mut fSlow401: F32 = 0.3 * (fSlow397 * F32::sin(fSlow399) / fSlow187);
		let mut fSlow402: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow145 * fSlow239 * fSlow238 / fSlow265));
		let mut fSlow403: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow148, fSlow11))));
		let mut fSlow404: F32 = BodyDsp_faustpower2_f(fSlow403);
		let mut fSlow405: F32 = fSlow152 * fSlow151 * fSlow150;
		let mut fSlow406: F32 = F32::max(1e-09, 0.3 * (fSlow405 / fSlow187));
		let mut fSlow407: F32 = self.fConst4 * F32::min(fSlow149, self.fConst3);
		let mut fSlow408: F32 = 2.0 * fSlow403 * F32::cos(fSlow407);
		let mut fSlow409: F32 = 0.3 * (fSlow405 * F32::sin(fSlow407) / fSlow187);
		let mut fSlow410: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow152 * fSlow242 * fSlow241 / fSlow265));
		let mut fSlow411: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow155, fSlow11))));
		let mut fSlow412: F32 = BodyDsp_faustpower2_f(fSlow411);
		let mut fSlow413: F32 = fSlow159 * fSlow158 * fSlow157;
		let mut fSlow414: F32 = F32::max(1e-09, 0.3 * (fSlow413 / fSlow187));
		let mut fSlow415: F32 = self.fConst4 * F32::min(fSlow156, self.fConst3);
		let mut fSlow416: F32 = 2.0 * fSlow411 * F32::cos(fSlow415);
		let mut fSlow417: F32 = 0.3 * (fSlow413 * F32::sin(fSlow415) / fSlow187);
		let mut fSlow418: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow159 * fSlow245 * fSlow244 / fSlow265));
		let mut fSlow419: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow162, fSlow11))));
		let mut fSlow420: F32 = BodyDsp_faustpower2_f(fSlow419);
		let mut fSlow421: F32 = fSlow166 * fSlow165 * fSlow164;
		let mut fSlow422: F32 = F32::max(1e-09, 0.3 * (fSlow421 / fSlow187));
		let mut fSlow423: F32 = self.fConst4 * F32::min(fSlow163, self.fConst3);
		let mut fSlow424: F32 = 2.0 * fSlow419 * F32::cos(fSlow423);
		let mut fSlow425: F32 = 0.3 * (fSlow421 * F32::sin(fSlow423) / fSlow187);
		let mut fSlow426: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow166 * fSlow248 * fSlow247 / fSlow265));
		let mut fSlow427: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow169, fSlow11))));
		let mut fSlow428: F32 = BodyDsp_faustpower2_f(fSlow427);
		let mut fSlow429: F32 = fSlow173 * fSlow172 * fSlow171;
		let mut fSlow430: F32 = F32::max(1e-09, 0.3 * (fSlow429 / fSlow187));
		let mut fSlow431: F32 = self.fConst4 * F32::min(fSlow170, self.fConst3);
		let mut fSlow432: F32 = 2.0 * fSlow427 * F32::cos(fSlow431);
		let mut fSlow433: F32 = 0.3 * (fSlow429 * F32::sin(fSlow431) / fSlow187);
		let mut fSlow434: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow173 * fSlow251 * fSlow250 / fSlow265));
		let mut fSlow435: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(fSlow176, fSlow11))));
		let mut fSlow436: F32 = BodyDsp_faustpower2_f(fSlow435);
		let mut fSlow437: F32 = fSlow180 * fSlow179 * fSlow178;
		let mut fSlow438: F32 = F32::max(1e-09, 0.3 * (fSlow437 / fSlow187));
		let mut fSlow439: F32 = self.fConst4 * F32::min(fSlow177, self.fConst3);
		let mut fSlow440: F32 = 2.0 * fSlow435 * F32::cos(fSlow439);
		let mut fSlow441: F32 = 0.3 * (fSlow437 * F32::sin(fSlow439) / fSlow187);
		let mut fSlow442: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow180 * fSlow254 * fSlow253 / fSlow265));
		let mut fSlow443: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow12 * F32::powf(1.0, fSlow11))));
		let mut fSlow444: F32 = BodyDsp_faustpower2_f(fSlow443);
		let mut fSlow445: F32 = F32::max(1e-09, 0.3 * (fSlow185 * fSlow184 * fSlow182 / fSlow187));
		let mut fSlow446: F32 = self.fConst4 * F32::min(fSlow20, self.fConst3);
		let mut fSlow447: F32 = 2.0 * fSlow443 * F32::cos(fSlow446);
		let mut fSlow448: F32 = 0.3 * (fSlow185 * F32::sin(fSlow446) * fSlow184 * fSlow182 / fSlow187);
		let mut fSlow449: F32 = 1.0 / F32::max(1e-09, 0.3 * (fSlow185 * fSlow182 * fSlow255 / fSlow265));
		let mut fSlow450: F32 = 1.0 / fSlow189;
		let mut fSlow451: F32 = 1.0 / fSlow270;
		let mut fSlow452: F32 = 1.0 / fSlow278;
		let mut fSlow453: F32 = 1.0 / fSlow286;
		let mut fSlow454: F32 = 1.0 / fSlow294;
		let mut fSlow455: F32 = 1.0 / fSlow302;
		let mut fSlow456: F32 = 1.0 / fSlow310;
		let mut fSlow457: F32 = 1.0 / fSlow318;
		let mut fSlow458: F32 = 1.0 / fSlow326;
		let mut fSlow459: F32 = 1.0 / fSlow334;
		let mut fSlow460: F32 = 1.0 / fSlow342;
		let mut fSlow461: F32 = 1.0 / fSlow350;
		let mut fSlow462: F32 = 1.0 / fSlow358;
		let mut fSlow463: F32 = 1.0 / fSlow366;
		let mut fSlow464: F32 = 1.0 / fSlow374;
		let mut fSlow465: F32 = 1.0 / fSlow382;
		let mut fSlow466: F32 = 1.0 / fSlow390;
		let mut fSlow467: F32 = 1.0 / fSlow398;
		let mut fSlow468: F32 = 1.0 / fSlow406;
		let mut fSlow469: F32 = 1.0 / fSlow414;
		let mut fSlow470: F32 = 1.0 / fSlow422;
		let mut fSlow471: F32 = 1.0 / fSlow430;
		let mut fSlow472: F32 = 1.0 / fSlow438;
		let mut fSlow473: F32 = 1.0 / fSlow445;
		let mut fSlow474: F32 = ((self.fHslider6 > 0.5) as i32) as u32 as F32;
		let mut iSlow475: i32 = (fSlow474 != 0.0) as i32;
		let mut fSlow476: F32 = F32::max(self.fConst8, F32::min(self.fConst3, fSlow20));
		let mut fSlow477: F32 = self.fConst0 / fSlow476;
		let mut iTemp105: i32 = self.iTempPerm0;
		let mut iTemp106: i32 = self.iTempPerm1;
		let mut iTemp107: i32 = self.iTempPerm2;
		let mut fSlow478: F32 = 2.7939678e-10 * (0.75 * fSlow9 + 0.25);
		let mut fSlow479: F32 = BodyDsp_faustpower2_f(fSlow9);
		let mut fSlow480: F32 = F32::exp(-(self.fConst4 * F32::min(self.fConst3, fSlow476 * (24.0 * fSlow479 + 1.5))));
		let mut fSlow481: F32 = 1.0 - fSlow480;
		let mut fSlow482: F32 = fSlow481 * F32::sqrt((fSlow480 + 1.0) / fSlow481);
		let mut fSlow483: F32 = self.fConst0 * ((0.42 * F32::max(0.0, F32::min(1.0, fSlow25)) + 0.08) / fSlow476);
		let mut iSlow484: i32 = (fSlow483) as i32;
		let mut iSlow485: i32 = std::cmp::min(8193, std::cmp::max(0, i32::wrapping_add(iSlow484, 1)));
		let mut fSlow486: F32 = F32::floor(fSlow483);
		let mut fSlow487: F32 = fSlow483 - fSlow486;
		let mut iSlow488: i32 = std::cmp::min(8193, std::cmp::max(0, iSlow484));
		let mut fSlow489: F32 = fSlow486 + (1.0 - fSlow483);
		let mut fTemp108: F32 = self.fTempPerm3;
		let mut fSlow490: F32 = self.fConst4 * fSlow476;
		let mut fSlow491: F32 = F32::sin(fSlow490);
		let mut fSlow492: F32 = self.fConst9 / fSlow476;
		let mut fSlow493: F32 = F32::min(0.9 * (1.0 - BodyDsp_faustpower2_f(fSlow2)), (fSlow492 + -1.0) / (fSlow492 + 1.0));
		let mut fSlow494: F32 = F32::cos(fSlow490);
		let mut fSlow495: F32 = 0.31830987 * (F32::atan2(fSlow491, fSlow494 - fSlow493) - F32::atan2(-(fSlow493 * fSlow491), 1.0 - fSlow493 * fSlow494));
		let mut fSlow496: F32 = F32::max(0.001, fSlow12) * fSlow476;
		let mut fSlow497: F32 = F32::max(0.0001, F32::exp(-(13.81551 * ((1.0 / (0.57 * fSlow479 + 0.03) + -1.0) / fSlow496))));
		let mut fSlow498: F32 = F32::min(-1e-06, fSlow497 + -1.0);
		let mut fSlow499: F32 = fSlow494 - fSlow497 * F32::cos(F32::min(self.fConst10 * fSlow476, 2.5132742));
		let mut fSlow500: F32 = F32::exp(-(6.907755 / fSlow496));
		let mut fSlow501: F32 = BodyDsp_faustpower2_f(F32::min(0.999999, 1.0005002 * fSlow500));
		let mut fSlow502: F32 = 1.0 - fSlow501;
		let mut fSlow503: F32 = 1.0 - fSlow494 * fSlow501;
		let mut fSlow504: F32 = F32::max(0.0, F32::min(0.995, F32::min(-(fSlow499 / fSlow498 + F32::sqrt(F32::max(0.0, BodyDsp_faustpower2_f(fSlow499) / BodyDsp_faustpower2_f(fSlow498) + -1.0))), fSlow503 / fSlow502 - F32::sqrt(F32::max(0.0, BodyDsp_faustpower2_f(fSlow503) / BodyDsp_faustpower2_f(fSlow502) + -1.0)))));
		let mut fSlow505: F32 = 0.15915494 * F32::atan2(fSlow504 * fSlow491, 1.0 - fSlow494 * fSlow504);
		let mut iSlow506: i32 = i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, (F32::max(2.0, self.fConst0 * ((1.0 - (fSlow505 + fSlow495)) / fSlow476) + -1.0) + -0.999995) as i32)), 1);
		let mut fTemp109: F32 = self.fTempPerm4;
		let mut fSlow507: F32 = F32::max(2.0, self.fConst0 * ((1.0 - fSlow505 - fSlow495) / fSlow476) + -1.0);
		let mut fSlow508: F32 = F32::floor(fSlow507 + -0.999995);
		let mut fSlow509: F32 = fSlow507 - fSlow508;
		let mut fTemp110: F32 = self.fTempPerm5;
		let mut fSlow510: F32 = 0.5 * (fSlow508 + (1.0 - fSlow507));
		let mut fTemp111: F32 = self.fTempPerm6;
		let mut fSlow511: F32 = 0.33333334 * (fSlow508 + (2.0 - fSlow507));
		let mut fSlow512: F32 = 1.0 - fSlow504;
		let mut fSlow513: F32 = F32::min(0.9995, fSlow500 * F32::sqrt(fSlow504 * (fSlow504 - 2.0 * fSlow494) + 1.0) / fSlow512);
		let mut fTemp112: F32 = self.fTempPerm7;
		let mut fSlow514: F32 = F32::exp(2.0794415 * fSlow18 * fSlow17);
		let mut fSlow515: F32 = 1.0 / fSlow514;
		let mut fTemp113: F32 = self.fTempPerm8;
		let mut fSlow516: F32 = 3.0 * fSlow514;
		let mut fSlow517: F32 = self.fConst12 / fSlow514;
		let mut fTemp114: F32 = self.fTempPerm9;
		let mut fSlow518: F32 = 0.74 * fSlow474 * fSlow514;
		let mut fTemp115: F32 = self.fTempPerm10;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0);
		for ((input0, input1), output0) in zipped_iterators {
			let mut iRec103: i32 = 0;
			self.fVec0[0] = fSlow0;
			let mut iTemp0: i32 = (fSlow0 > self.fVec0[1]) as i32;
			let mut fTemp1: F32 = (i32::wrapping_sub(1, iTemp0)) as F32;
			let mut fTemp2: F32 = 1.0 - self.fConst1 * fTemp1;
			self.fRec1[0] = fSlow14 * fTemp2 + self.fConst1 * fTemp1 * self.fRec1[1];
			let mut fTemp3: F32 = self.fRec1[0] * self.fRec0[2];
			self.fRec2[0] = fSlow19 * fTemp2 + self.fConst1 * fTemp1 * self.fRec2[1];
			let mut fTemp4: F32 = F32::max(1e-06, self.fRec2[0]);
			let mut fTemp5: F32 = if iSlow16 != 0 {fSlow189 / fTemp4} else {1e+30};
			self.fRec3[0] = fSlow191 * fTemp2 + self.fConst1 * fTemp1 * self.fRec3[1];
			self.fRec4[0] = fSlow192 * fTemp2 + self.fConst1 * fTemp1 * self.fRec4[1];
			let mut fTemp6: F32 = *input0;
			let mut fTemp7: F32 = fTemp6 * self.fRec4[0] + self.fRec3[0] * self.fRec0[1];
			self.fRec0[0] = fTemp7 - (fTemp3 + 0.002 * (F32::max(0.0, fTemp7 - (fTemp3 + 2.5 * fTemp5)) + F32::min(0.0, fTemp7 + fTemp5 - fTemp3)));
			let mut fTemp8: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec0[0]) + 0.4 * F32::max(0.0, self.fRec0[0]));
			self.fRec6[0] = fSlow268 * fTemp2 + self.fConst1 * fTemp1 * self.fRec6[1];
			let mut fTemp9: F32 = self.fRec6[0] * self.fRec5[2];
			let mut fTemp10: F32 = if iSlow16 != 0 {fSlow270 / fTemp4} else {1e+30};
			self.fRec7[0] = fSlow272 * fTemp2 + self.fConst1 * fTemp1 * self.fRec7[1];
			self.fRec8[0] = fSlow273 * fTemp2 + self.fConst1 * fTemp1 * self.fRec8[1];
			let mut fTemp11: F32 = fTemp6 * self.fRec8[0] + self.fRec7[0] * self.fRec5[1];
			self.fRec5[0] = fTemp11 - (fTemp9 + 0.002 * (F32::max(0.0, fTemp11 - (fTemp9 + 2.5 * fTemp10)) + F32::min(0.0, fTemp11 + fTemp10 - fTemp9)));
			let mut fTemp12: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec5[0]) + 0.4 * F32::max(0.0, self.fRec5[0]));
			self.fRec10[0] = fSlow276 * fTemp2 + self.fConst1 * fTemp1 * self.fRec10[1];
			let mut fTemp13: F32 = self.fRec10[0] * self.fRec9[2];
			let mut fTemp14: F32 = if iSlow16 != 0 {fSlow278 / fTemp4} else {1e+30};
			self.fRec11[0] = fSlow280 * fTemp2 + self.fConst1 * fTemp1 * self.fRec11[1];
			self.fRec12[0] = fSlow281 * fTemp2 + self.fConst1 * fTemp1 * self.fRec12[1];
			let mut fTemp15: F32 = fTemp6 * self.fRec12[0] + self.fRec11[0] * self.fRec9[1];
			self.fRec9[0] = fTemp15 - (fTemp13 + 0.002 * (F32::max(0.0, fTemp15 - (fTemp13 + 2.5 * fTemp14)) + F32::min(0.0, fTemp15 + fTemp14 - fTemp13)));
			let mut fTemp16: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec9[0]) + 0.4 * F32::max(0.0, self.fRec9[0]));
			self.fRec14[0] = fSlow284 * fTemp2 + self.fConst1 * fTemp1 * self.fRec14[1];
			let mut fTemp17: F32 = self.fRec14[0] * self.fRec13[2];
			let mut fTemp18: F32 = if iSlow16 != 0 {fSlow286 / fTemp4} else {1e+30};
			self.fRec15[0] = fSlow288 * fTemp2 + self.fConst1 * fTemp1 * self.fRec15[1];
			self.fRec16[0] = fSlow289 * fTemp2 + self.fConst1 * fTemp1 * self.fRec16[1];
			let mut fTemp19: F32 = fTemp6 * self.fRec16[0] + self.fRec15[0] * self.fRec13[1];
			self.fRec13[0] = fTemp19 - (fTemp17 + 0.002 * (F32::max(0.0, fTemp19 - (fTemp17 + 2.5 * fTemp18)) + F32::min(0.0, fTemp19 + fTemp18 - fTemp17)));
			let mut fTemp20: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec13[0]) + 0.4 * F32::max(0.0, self.fRec13[0]));
			self.fRec18[0] = fSlow292 * fTemp2 + self.fConst1 * fTemp1 * self.fRec18[1];
			let mut fTemp21: F32 = self.fRec18[0] * self.fRec17[2];
			let mut fTemp22: F32 = if iSlow16 != 0 {fSlow294 / fTemp4} else {1e+30};
			self.fRec19[0] = fSlow296 * fTemp2 + self.fConst1 * fTemp1 * self.fRec19[1];
			self.fRec20[0] = fSlow297 * fTemp2 + self.fConst1 * fTemp1 * self.fRec20[1];
			let mut fTemp23: F32 = fTemp6 * self.fRec20[0] + self.fRec19[0] * self.fRec17[1];
			self.fRec17[0] = fTemp23 - (fTemp21 + 0.002 * (F32::max(0.0, fTemp23 - (fTemp21 + 2.5 * fTemp22)) + F32::min(0.0, fTemp23 + fTemp22 - fTemp21)));
			let mut fTemp24: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec17[0]) + 0.4 * F32::max(0.0, self.fRec17[0]));
			self.fRec22[0] = fSlow300 * fTemp2 + self.fConst1 * fTemp1 * self.fRec22[1];
			let mut fTemp25: F32 = self.fRec22[0] * self.fRec21[2];
			let mut fTemp26: F32 = if iSlow16 != 0 {fSlow302 / fTemp4} else {1e+30};
			self.fRec23[0] = fSlow304 * fTemp2 + self.fConst1 * fTemp1 * self.fRec23[1];
			self.fRec24[0] = fSlow305 * fTemp2 + self.fConst1 * fTemp1 * self.fRec24[1];
			let mut fTemp27: F32 = fTemp6 * self.fRec24[0] + self.fRec23[0] * self.fRec21[1];
			self.fRec21[0] = fTemp27 - (fTemp25 + 0.002 * (F32::max(0.0, fTemp27 - (fTemp25 + 2.5 * fTemp26)) + F32::min(0.0, fTemp27 + fTemp26 - fTemp25)));
			let mut fTemp28: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec21[0]) + 0.4 * F32::max(0.0, self.fRec21[0]));
			self.fRec26[0] = fSlow308 * fTemp2 + self.fConst1 * fTemp1 * self.fRec26[1];
			let mut fTemp29: F32 = self.fRec26[0] * self.fRec25[2];
			let mut fTemp30: F32 = if iSlow16 != 0 {fSlow310 / fTemp4} else {1e+30};
			self.fRec27[0] = fSlow312 * fTemp2 + self.fConst1 * fTemp1 * self.fRec27[1];
			self.fRec28[0] = fSlow313 * fTemp2 + self.fConst1 * fTemp1 * self.fRec28[1];
			let mut fTemp31: F32 = fTemp6 * self.fRec28[0] + self.fRec27[0] * self.fRec25[1];
			self.fRec25[0] = fTemp31 - (fTemp29 + 0.002 * (F32::max(0.0, fTemp31 - (fTemp29 + 2.5 * fTemp30)) + F32::min(0.0, fTemp31 + fTemp30 - fTemp29)));
			let mut fTemp32: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec25[0]) + 0.4 * F32::max(0.0, self.fRec25[0]));
			self.fRec30[0] = fSlow316 * fTemp2 + self.fConst1 * fTemp1 * self.fRec30[1];
			let mut fTemp33: F32 = self.fRec30[0] * self.fRec29[2];
			let mut fTemp34: F32 = if iSlow16 != 0 {fSlow318 / fTemp4} else {1e+30};
			self.fRec31[0] = fSlow320 * fTemp2 + self.fConst1 * fTemp1 * self.fRec31[1];
			self.fRec32[0] = fSlow321 * fTemp2 + self.fConst1 * fTemp1 * self.fRec32[1];
			let mut fTemp35: F32 = fTemp6 * self.fRec32[0] + self.fRec31[0] * self.fRec29[1];
			self.fRec29[0] = fTemp35 - (fTemp33 + 0.002 * (F32::max(0.0, fTemp35 - (fTemp33 + 2.5 * fTemp34)) + F32::min(0.0, fTemp35 + fTemp34 - fTemp33)));
			let mut fTemp36: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec29[0]) + 0.4 * F32::max(0.0, self.fRec29[0]));
			self.fRec34[0] = fSlow324 * fTemp2 + self.fConst1 * fTemp1 * self.fRec34[1];
			let mut fTemp37: F32 = self.fRec34[0] * self.fRec33[2];
			let mut fTemp38: F32 = if iSlow16 != 0 {fSlow326 / fTemp4} else {1e+30};
			self.fRec35[0] = fSlow328 * fTemp2 + self.fConst1 * fTemp1 * self.fRec35[1];
			self.fRec36[0] = fSlow329 * fTemp2 + self.fConst1 * fTemp1 * self.fRec36[1];
			let mut fTemp39: F32 = fTemp6 * self.fRec36[0] + self.fRec35[0] * self.fRec33[1];
			self.fRec33[0] = fTemp39 - (fTemp37 + 0.002 * (F32::max(0.0, fTemp39 - (fTemp37 + 2.5 * fTemp38)) + F32::min(0.0, fTemp39 + fTemp38 - fTemp37)));
			let mut fTemp40: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec33[0]) + 0.4 * F32::max(0.0, self.fRec33[0]));
			self.fRec38[0] = fSlow332 * fTemp2 + self.fConst1 * fTemp1 * self.fRec38[1];
			let mut fTemp41: F32 = self.fRec38[0] * self.fRec37[2];
			let mut fTemp42: F32 = if iSlow16 != 0 {fSlow334 / fTemp4} else {1e+30};
			self.fRec39[0] = fSlow336 * fTemp2 + self.fConst1 * fTemp1 * self.fRec39[1];
			self.fRec40[0] = fSlow337 * fTemp2 + self.fConst1 * fTemp1 * self.fRec40[1];
			let mut fTemp43: F32 = fTemp6 * self.fRec40[0] + self.fRec39[0] * self.fRec37[1];
			self.fRec37[0] = fTemp43 - (fTemp41 + 0.002 * (F32::max(0.0, fTemp43 - (fTemp41 + 2.5 * fTemp42)) + F32::min(0.0, fTemp43 + fTemp42 - fTemp41)));
			let mut fTemp44: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec37[0]) + 0.4 * F32::max(0.0, self.fRec37[0]));
			self.fRec42[0] = fSlow340 * fTemp2 + self.fConst1 * fTemp1 * self.fRec42[1];
			let mut fTemp45: F32 = self.fRec42[0] * self.fRec41[2];
			let mut fTemp46: F32 = if iSlow16 != 0 {fSlow342 / fTemp4} else {1e+30};
			self.fRec43[0] = fSlow344 * fTemp2 + self.fConst1 * fTemp1 * self.fRec43[1];
			self.fRec44[0] = fSlow345 * fTemp2 + self.fConst1 * fTemp1 * self.fRec44[1];
			let mut fTemp47: F32 = fTemp6 * self.fRec44[0] + self.fRec43[0] * self.fRec41[1];
			self.fRec41[0] = fTemp47 - (fTemp45 + 0.002 * (F32::max(0.0, fTemp47 - (fTemp45 + 2.5 * fTemp46)) + F32::min(0.0, fTemp47 + fTemp46 - fTemp45)));
			let mut fTemp48: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec41[0]) + 0.4 * F32::max(0.0, self.fRec41[0]));
			self.fRec46[0] = fSlow348 * fTemp2 + self.fConst1 * fTemp1 * self.fRec46[1];
			let mut fTemp49: F32 = self.fRec46[0] * self.fRec45[2];
			let mut fTemp50: F32 = if iSlow16 != 0 {fSlow350 / fTemp4} else {1e+30};
			self.fRec47[0] = fSlow352 * fTemp2 + self.fConst1 * fTemp1 * self.fRec47[1];
			self.fRec48[0] = fSlow353 * fTemp2 + self.fConst1 * fTemp1 * self.fRec48[1];
			let mut fTemp51: F32 = fTemp6 * self.fRec48[0] + self.fRec47[0] * self.fRec45[1];
			self.fRec45[0] = fTemp51 - (fTemp49 + 0.002 * (F32::max(0.0, fTemp51 - (fTemp49 + 2.5 * fTemp50)) + F32::min(0.0, fTemp51 + fTemp50 - fTemp49)));
			let mut fTemp52: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec45[0]) + 0.4 * F32::max(0.0, self.fRec45[0]));
			self.fRec50[0] = fSlow356 * fTemp2 + self.fConst1 * fTemp1 * self.fRec50[1];
			let mut fTemp53: F32 = self.fRec50[0] * self.fRec49[2];
			let mut fTemp54: F32 = if iSlow16 != 0 {fSlow358 / fTemp4} else {1e+30};
			self.fRec51[0] = fSlow360 * fTemp2 + self.fConst1 * fTemp1 * self.fRec51[1];
			self.fRec52[0] = fSlow361 * fTemp2 + self.fConst1 * fTemp1 * self.fRec52[1];
			let mut fTemp55: F32 = fTemp6 * self.fRec52[0] + self.fRec51[0] * self.fRec49[1];
			self.fRec49[0] = fTemp55 - (fTemp53 + 0.002 * (F32::max(0.0, fTemp55 - (fTemp53 + 2.5 * fTemp54)) + F32::min(0.0, fTemp55 + fTemp54 - fTemp53)));
			let mut fTemp56: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec49[0]) + 0.4 * F32::max(0.0, self.fRec49[0]));
			self.fRec54[0] = fSlow364 * fTemp2 + self.fConst1 * fTemp1 * self.fRec54[1];
			let mut fTemp57: F32 = self.fRec54[0] * self.fRec53[2];
			let mut fTemp58: F32 = if iSlow16 != 0 {fSlow366 / fTemp4} else {1e+30};
			self.fRec55[0] = fSlow368 * fTemp2 + self.fConst1 * fTemp1 * self.fRec55[1];
			self.fRec56[0] = fSlow369 * fTemp2 + self.fConst1 * fTemp1 * self.fRec56[1];
			let mut fTemp59: F32 = fTemp6 * self.fRec56[0] + self.fRec55[0] * self.fRec53[1];
			self.fRec53[0] = fTemp59 - (fTemp57 + 0.002 * (F32::max(0.0, fTemp59 - (fTemp57 + 2.5 * fTemp58)) + F32::min(0.0, fTemp59 + fTemp58 - fTemp57)));
			let mut fTemp60: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec53[0]) + 0.4 * F32::max(0.0, self.fRec53[0]));
			self.fRec58[0] = fSlow372 * fTemp2 + self.fConst1 * fTemp1 * self.fRec58[1];
			let mut fTemp61: F32 = self.fRec58[0] * self.fRec57[2];
			let mut fTemp62: F32 = if iSlow16 != 0 {fSlow374 / fTemp4} else {1e+30};
			self.fRec59[0] = fSlow376 * fTemp2 + self.fConst1 * fTemp1 * self.fRec59[1];
			self.fRec60[0] = fSlow377 * fTemp2 + self.fConst1 * fTemp1 * self.fRec60[1];
			let mut fTemp63: F32 = fTemp6 * self.fRec60[0] + self.fRec59[0] * self.fRec57[1];
			self.fRec57[0] = fTemp63 - (fTemp61 + 0.002 * (F32::max(0.0, fTemp63 - (fTemp61 + 2.5 * fTemp62)) + F32::min(0.0, fTemp63 + fTemp62 - fTemp61)));
			let mut fTemp64: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec57[0]) + 0.4 * F32::max(0.0, self.fRec57[0]));
			self.fRec62[0] = fSlow380 * fTemp2 + self.fConst1 * fTemp1 * self.fRec62[1];
			let mut fTemp65: F32 = self.fRec62[0] * self.fRec61[2];
			let mut fTemp66: F32 = if iSlow16 != 0 {fSlow382 / fTemp4} else {1e+30};
			self.fRec63[0] = fSlow384 * fTemp2 + self.fConst1 * fTemp1 * self.fRec63[1];
			self.fRec64[0] = fSlow385 * fTemp2 + self.fConst1 * fTemp1 * self.fRec64[1];
			let mut fTemp67: F32 = fTemp6 * self.fRec64[0] + self.fRec63[0] * self.fRec61[1];
			self.fRec61[0] = fTemp67 - (fTemp65 + 0.002 * (F32::max(0.0, fTemp67 - (fTemp65 + 2.5 * fTemp66)) + F32::min(0.0, fTemp67 + fTemp66 - fTemp65)));
			let mut fTemp68: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec61[0]) + 0.4 * F32::max(0.0, self.fRec61[0]));
			self.fRec66[0] = fSlow388 * fTemp2 + self.fConst1 * fTemp1 * self.fRec66[1];
			let mut fTemp69: F32 = self.fRec66[0] * self.fRec65[2];
			let mut fTemp70: F32 = if iSlow16 != 0 {fSlow390 / fTemp4} else {1e+30};
			self.fRec67[0] = fSlow392 * fTemp2 + self.fConst1 * fTemp1 * self.fRec67[1];
			self.fRec68[0] = fSlow393 * fTemp2 + self.fConst1 * fTemp1 * self.fRec68[1];
			let mut fTemp71: F32 = fTemp6 * self.fRec68[0] + self.fRec67[0] * self.fRec65[1];
			self.fRec65[0] = fTemp71 - (fTemp69 + 0.002 * (F32::max(0.0, fTemp71 - (fTemp69 + 2.5 * fTemp70)) + F32::min(0.0, fTemp71 + fTemp70 - fTemp69)));
			let mut fTemp72: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec65[0]) + 0.4 * F32::max(0.0, self.fRec65[0]));
			self.fRec70[0] = fSlow396 * fTemp2 + self.fConst1 * fTemp1 * self.fRec70[1];
			let mut fTemp73: F32 = self.fRec70[0] * self.fRec69[2];
			let mut fTemp74: F32 = if iSlow16 != 0 {fSlow398 / fTemp4} else {1e+30};
			self.fRec71[0] = fSlow400 * fTemp2 + self.fConst1 * fTemp1 * self.fRec71[1];
			self.fRec72[0] = fSlow401 * fTemp2 + self.fConst1 * fTemp1 * self.fRec72[1];
			let mut fTemp75: F32 = fTemp6 * self.fRec72[0] + self.fRec71[0] * self.fRec69[1];
			self.fRec69[0] = fTemp75 - (fTemp73 + 0.002 * (F32::max(0.0, fTemp75 - (fTemp73 + 2.5 * fTemp74)) + F32::min(0.0, fTemp75 + fTemp74 - fTemp73)));
			let mut fTemp76: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec69[0]) + 0.4 * F32::max(0.0, self.fRec69[0]));
			self.fRec74[0] = fSlow404 * fTemp2 + self.fConst1 * fTemp1 * self.fRec74[1];
			let mut fTemp77: F32 = self.fRec74[0] * self.fRec73[2];
			let mut fTemp78: F32 = if iSlow16 != 0 {fSlow406 / fTemp4} else {1e+30};
			self.fRec75[0] = fSlow408 * fTemp2 + self.fConst1 * fTemp1 * self.fRec75[1];
			self.fRec76[0] = fSlow409 * fTemp2 + self.fConst1 * fTemp1 * self.fRec76[1];
			let mut fTemp79: F32 = fTemp6 * self.fRec76[0] + self.fRec75[0] * self.fRec73[1];
			self.fRec73[0] = fTemp79 - (fTemp77 + 0.002 * (F32::max(0.0, fTemp79 - (fTemp77 + 2.5 * fTemp78)) + F32::min(0.0, fTemp79 + fTemp78 - fTemp77)));
			let mut fTemp80: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec73[0]) + 0.4 * F32::max(0.0, self.fRec73[0]));
			self.fRec78[0] = fSlow412 * fTemp2 + self.fConst1 * fTemp1 * self.fRec78[1];
			let mut fTemp81: F32 = self.fRec78[0] * self.fRec77[2];
			let mut fTemp82: F32 = if iSlow16 != 0 {fSlow414 / fTemp4} else {1e+30};
			self.fRec79[0] = fSlow416 * fTemp2 + self.fConst1 * fTemp1 * self.fRec79[1];
			self.fRec80[0] = fSlow417 * fTemp2 + self.fConst1 * fTemp1 * self.fRec80[1];
			let mut fTemp83: F32 = fTemp6 * self.fRec80[0] + self.fRec79[0] * self.fRec77[1];
			self.fRec77[0] = fTemp83 - (fTemp81 + 0.002 * (F32::max(0.0, fTemp83 - (fTemp81 + 2.5 * fTemp82)) + F32::min(0.0, fTemp83 + fTemp82 - fTemp81)));
			let mut fTemp84: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec77[0]) + 0.4 * F32::max(0.0, self.fRec77[0]));
			self.fRec82[0] = fSlow420 * fTemp2 + self.fConst1 * fTemp1 * self.fRec82[1];
			let mut fTemp85: F32 = self.fRec82[0] * self.fRec81[2];
			let mut fTemp86: F32 = if iSlow16 != 0 {fSlow422 / fTemp4} else {1e+30};
			self.fRec83[0] = fSlow424 * fTemp2 + self.fConst1 * fTemp1 * self.fRec83[1];
			self.fRec84[0] = fSlow425 * fTemp2 + self.fConst1 * fTemp1 * self.fRec84[1];
			let mut fTemp87: F32 = fTemp6 * self.fRec84[0] + self.fRec83[0] * self.fRec81[1];
			self.fRec81[0] = fTemp87 - (fTemp85 + 0.002 * (F32::max(0.0, fTemp87 - (fTemp85 + 2.5 * fTemp86)) + F32::min(0.0, fTemp87 + fTemp86 - fTemp85)));
			let mut fTemp88: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec81[0]) + 0.4 * F32::max(0.0, self.fRec81[0]));
			self.fRec86[0] = fSlow428 * fTemp2 + self.fConst1 * fTemp1 * self.fRec86[1];
			let mut fTemp89: F32 = self.fRec86[0] * self.fRec85[2];
			let mut fTemp90: F32 = if iSlow16 != 0 {fSlow430 / fTemp4} else {1e+30};
			self.fRec87[0] = fSlow432 * fTemp2 + self.fConst1 * fTemp1 * self.fRec87[1];
			self.fRec88[0] = fSlow433 * fTemp2 + self.fConst1 * fTemp1 * self.fRec88[1];
			let mut fTemp91: F32 = fTemp6 * self.fRec88[0] + self.fRec87[0] * self.fRec85[1];
			self.fRec85[0] = fTemp91 - (fTemp89 + 0.002 * (F32::max(0.0, fTemp91 - (fTemp89 + 2.5 * fTemp90)) + F32::min(0.0, fTemp91 + fTemp90 - fTemp89)));
			let mut fTemp92: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec85[0]) + 0.4 * F32::max(0.0, self.fRec85[0]));
			self.fRec90[0] = fSlow436 * fTemp2 + self.fConst1 * fTemp1 * self.fRec90[1];
			let mut fTemp93: F32 = self.fRec90[0] * self.fRec89[2];
			let mut fTemp94: F32 = if iSlow16 != 0 {fSlow438 / fTemp4} else {1e+30};
			self.fRec91[0] = fSlow440 * fTemp2 + self.fConst1 * fTemp1 * self.fRec91[1];
			self.fRec92[0] = fSlow441 * fTemp2 + self.fConst1 * fTemp1 * self.fRec92[1];
			let mut fTemp95: F32 = fTemp6 * self.fRec92[0] + self.fRec91[0] * self.fRec89[1];
			self.fRec89[0] = fTemp95 - (fTemp93 + 0.002 * (F32::max(0.0, fTemp95 - (fTemp93 + 2.5 * fTemp94)) + F32::min(0.0, fTemp95 + fTemp94 - fTemp93)));
			let mut fTemp96: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec89[0]) + 0.4 * F32::max(0.0, self.fRec89[0]));
			self.fRec94[0] = fSlow444 * fTemp2 + self.fConst1 * fTemp1 * self.fRec94[1];
			let mut fTemp97: F32 = self.fRec94[0] * self.fRec93[2];
			let mut fTemp98: F32 = if iSlow16 != 0 {fSlow445 / fTemp4} else {1e+30};
			self.fRec95[0] = fSlow447 * fTemp2 + self.fConst1 * fTemp1 * self.fRec95[1];
			self.fRec96[0] = fSlow448 * fTemp2 + self.fConst1 * fTemp1 * self.fRec96[1];
			let mut fTemp99: F32 = fTemp6 * self.fRec96[0] + self.fRec95[0] * self.fRec93[1];
			self.fRec93[0] = fTemp99 - (fTemp97 + 0.002 * (F32::max(0.0, fTemp99 - (fTemp97 + 2.5 * fTemp98)) + F32::min(0.0, fTemp99 + fTemp98 - fTemp97)));
			let mut fTemp100: F32 = self.fRec2[0] * (F32::max(0.0, -self.fRec93[0]) + 0.4 * F32::max(0.0, self.fRec93[0]));
			let mut fTemp101: F32 = self.fRec93[0] / (fSlow449 * fTemp100 + 1.0) + self.fRec89[0] / (fSlow442 * fTemp96 + 1.0) + self.fRec85[0] / (fSlow434 * fTemp92 + 1.0) + self.fRec81[0] / (fSlow426 * fTemp88 + 1.0) + self.fRec77[0] / (fSlow418 * fTemp84 + 1.0) + self.fRec73[0] / (fSlow410 * fTemp80 + 1.0) + self.fRec69[0] / (fSlow402 * fTemp76 + 1.0) + self.fRec65[0] / (fSlow394 * fTemp72 + 1.0) + self.fRec61[0] / (fSlow386 * fTemp68 + 1.0) + self.fRec57[0] / (fSlow378 * fTemp64 + 1.0) + self.fRec53[0] / (fSlow370 * fTemp60 + 1.0) + self.fRec49[0] / (fSlow362 * fTemp56 + 1.0) + self.fRec45[0] / (fSlow354 * fTemp52 + 1.0) + self.fRec41[0] / (fSlow346 * fTemp48 + 1.0) + self.fRec37[0] / (fSlow338 * fTemp44 + 1.0) + self.fRec33[0] / (fSlow330 * fTemp40 + 1.0) + self.fRec29[0] / (fSlow322 * fTemp36 + 1.0) + self.fRec25[0] / (fSlow314 * fTemp32 + 1.0) + self.fRec21[0] / (fSlow306 * fTemp28 + 1.0) + self.fRec17[0] / (fSlow298 * fTemp24 + 1.0) + self.fRec13[0] / (fSlow290 * fTemp20 + 1.0) + self.fRec9[0] / (fSlow282 * fTemp16 + 1.0) + self.fRec5[0] / (fSlow274 * fTemp12 + 1.0) + self.fRec0[0] / (fSlow266 * fTemp8 + 1.0);
			let mut fTemp102: F32 = self.fRec93[0] / (fSlow473 * fTemp100 + 1.0) + self.fRec89[0] / (fSlow472 * fTemp96 + 1.0) + self.fRec85[0] / (fSlow471 * fTemp92 + 1.0) + self.fRec81[0] / (fSlow470 * fTemp88 + 1.0) + self.fRec77[0] / (fSlow469 * fTemp84 + 1.0) + self.fRec73[0] / (fSlow468 * fTemp80 + 1.0) + self.fRec69[0] / (fSlow467 * fTemp76 + 1.0) + self.fRec65[0] / (fSlow466 * fTemp72 + 1.0) + self.fRec61[0] / (fSlow465 * fTemp68 + 1.0) + self.fRec57[0] / (fSlow464 * fTemp64 + 1.0) + self.fRec53[0] / (fSlow463 * fTemp60 + 1.0) + self.fRec49[0] / (fSlow462 * fTemp56 + 1.0) + self.fRec45[0] / (fSlow461 * fTemp52 + 1.0) + self.fRec41[0] / (fSlow460 * fTemp48 + 1.0) + self.fRec37[0] / (fSlow459 * fTemp44 + 1.0) + self.fRec33[0] / (fSlow458 * fTemp40 + 1.0) + self.fRec29[0] / (fSlow457 * fTemp36 + 1.0) + self.fRec25[0] / (fSlow456 * fTemp32 + 1.0) + self.fRec21[0] / (fSlow455 * fTemp28 + 1.0) + self.fRec17[0] / (fSlow454 * fTemp24 + 1.0) + self.fRec13[0] / (fSlow453 * fTemp20 + 1.0) + self.fRec9[0] / (fSlow452 * fTemp16 + 1.0) + self.fRec5[0] / (fSlow451 * fTemp12 + 1.0) + self.fRec0[0] / (fSlow450 * fTemp8 + 1.0);
			self.fVec1[0] = fTemp102;
			self.fRec97[0] = -(self.fConst7 * (self.fConst6 * self.fRec97[1] - self.fConst5 * (fTemp102 - self.fVec1[1])));
			let mut fTemp103: F32 = *input1;
			let mut fTemp104: F32 = fTemp103 * F32::powf(self.fRec2[0] + 1.0, 0.6) * (fTemp101 + fSlow18 * (self.fRec97[0] - fTemp101));
			if iSlow475 != 0 {
				self.iRec101[0] = if (iTemp0 > 0) as i32 != 0 {0} else {std::cmp::min(8192, i32::wrapping_add(self.iRec101[1], 1))};
				iTemp105 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec102[1], 12345));
				iTemp106 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp105, 12345));
				iTemp107 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp106, 12345));
				self.iRec102[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp107, 12345));
				iRec103 = iTemp107;
				self.fRec100[(self.IOTA0 & 8191) as usize] = fSlow480 * self.fRec100[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize] + fSlow482 * (2.0 * fTemp6 + fSlow478 * (iRec103) as F32 * ((((self.iRec101[0]) as F32) < fSlow477) as i32) as u32 as F32);
				fTemp108 = 0.95 * (fSlow489 * self.fRec100[((i32::wrapping_sub(self.IOTA0, iSlow488)) & 8191) as usize] + fSlow487 * self.fRec100[((i32::wrapping_sub(self.IOTA0, iSlow485)) & 8191) as usize]);
				fTemp109 = self.fRec99[((i32::wrapping_sub(self.IOTA0, iSlow506)) & 16383) as usize];
				self.fVec2[0] = fTemp109;
				fTemp110 = fSlow509 * (fTemp109 - self.fVec2[1]);
				self.fVec3[0] = fTemp110;
				fTemp111 = fSlow510 * (self.fVec3[1] - fTemp110);
				self.fVec4[0] = fTemp111;
				self.fRec106[0] = fSlow504 * self.fRec106[1] + fSlow512 * (fSlow511 * (fTemp111 - self.fVec4[1]) + fTemp109 + fTemp111 - fTemp110);
				self.fRec105[0] = self.fRec106[1] + fSlow493 * (self.fRec105[1] - self.fRec106[0]);
				self.fRec104[0] = self.fRec105[1] + fSlow493 * (self.fRec104[1] - self.fRec105[0]);
				fTemp112 = self.fRec100[(self.IOTA0 & 8191) as usize] + fSlow513 * self.fRec104[0];
				self.fRec99[(self.IOTA0 & 16383) as usize] = fTemp112 + fSlow18 * (fTemp108 + fSlow515 * F32::tanh(fSlow514 * (fTemp112 - fTemp108)) - fTemp112) - fTemp108;
				fTemp113 = self.fRec99[(self.IOTA0 & 16383) as usize];
				fTemp114 = fTemp113 + fSlow18 * (fSlow517 * (F32::tanh(fSlow516 * fTemp113 + 0.5) - self.fConst11) - fTemp113);
				self.fVec5[0] = fTemp114;
				self.fRec98[0] = -(self.fConst15 * (self.fConst14 * self.fRec98[1] - self.fConst13 * (fTemp114 - self.fVec5[1])));
				fTemp115 = fSlow518 * fTemp103 * self.fRec98[0];
			}
				*output0 = fTemp104 + fSlow474 * (fTemp115 - fTemp104);
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
				if iSlow475 != 0 {
					self.iRec101[1] = self.iRec101[0];
					self.iRec102[1] = self.iRec102[0];
				}
					self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
					if iSlow475 != 0 {
						self.fVec2[1] = self.fVec2[0];
						self.fVec3[1] = self.fVec3[0];
						self.fVec4[1] = self.fVec4[0];
						self.fRec106[1] = self.fRec106[0];
						self.fRec105[1] = self.fRec105[0];
						self.fRec104[1] = self.fRec104[0];
						self.fVec5[1] = self.fVec5[0];
						self.fRec98[1] = self.fRec98[0];
					}
					}
					self.iTempPerm0 = iTemp105;
					self.iTempPerm1 = iTemp106;
					self.iTempPerm2 = iTemp107;
					self.fTempPerm3 = fTemp108;
					self.fTempPerm4 = fTemp109;
					self.fTempPerm5 = fTemp110;
					self.fTempPerm6 = fTemp111;
					self.fTempPerm7 = fTemp112;
					self.fTempPerm8 = fTemp113;
					self.fTempPerm9 = fTemp114;
					self.fTempPerm10 = fTemp115;
				}

}

