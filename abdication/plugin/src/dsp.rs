/* ------------------------------------------------------------
name: "plugin-voice"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn VoiceDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn VoiceDsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}
fn VoiceDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct VoiceDsp {
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	fHslider0: F32,
	fVec2: [F32;2],
	fHslider1: F32,
	fRec0: [F32;2],
	fRec1: [F32;2],
	iTempPerm0: i32,
	fHslider2: F32,
	iRec3: [i32;2],
	iTempPerm1: i32,
	fSampleRate: i32,
	fConst1: F32,
	fConst3: F32,
	fRec2: [F32;2],
	fConst4: F32,
	fHslider3: F32,
	fRec4: [F32;2],
	fConst5: F32,
	fTempPerm2: F32,
	fRec6: [F32;2],
	fVslider0: F32,
	fVslider1: F32,
	fRec7: [F32;2],
	fConst6: F32,
	fVslider2: F32,
	fRec8: [F32;2],
	fVslider3: F32,
	fVslider4: F32,
	fVslider5: F32,
	fRec9: [F32;2],
	fVslider6: F32,
	fRec5: [F32;2],
	fHslider4: F32,
	fVec3: [F32;2],
	fRec11: [F32;2],
	fRec12: [F32;2],
	fConst7: F32,
	fHslider5: F32,
	fRec13: [F32;2],
	fEntry0: F32,
	fTempPerm3: F32,
	fConst8: F32,
	fHslider6: F32,
	fRec28: [F32;2],
	fEntry1: F32,
	fTempPerm4: F32,
	fRec31: [F32;2],
	iRec32: [i32;2],
	fRec30: [F32;2],
	fTempPerm5: F32,
	fVec4: [F32;2],
	fRec29: [F32;2],
	fTempPerm6: F32,
	fHslider7: F32,
	fVec5: [F32;2],
	fRec33: [F32;2],
	fRec34: [F32;2],
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
	fTempPerm21: F32,
	fHslider8: F32,
	fVec6: [F32;2],
	fRec35: [F32;2],
	fRec36: [F32;2],
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
	fTempPerm32: F32,
	fTempPerm33: F32,
	fTempPerm34: F32,
	fTempPerm35: F32,
	fTempPerm36: F32,
	fTempPerm37: F32,
	fRec14: [F32;2],
	fTempPerm38: F32,
	fTempPerm39: F32,
	fRec15: [F32;2],
	fTempPerm40: F32,
	fTempPerm41: F32,
	fRec16: [F32;2],
	fTempPerm42: F32,
	fTempPerm43: F32,
	fRec17: [F32;2],
	fTempPerm44: F32,
	fTempPerm45: F32,
	fTempPerm46: F32,
	fTempPerm47: F32,
	fRec18: [F32;2],
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	fTempPerm51: F32,
	fRec19: [F32;2],
	fTempPerm52: F32,
	fTempPerm53: F32,
	fTempPerm54: F32,
	fTempPerm55: F32,
	fRec20: [F32;2],
	fTempPerm56: F32,
	fTempPerm57: F32,
	fTempPerm58: F32,
	fTempPerm59: F32,
	fRec21: [F32;2],
	fRec22: [F32;2],
	fRec23: [F32;2],
	fRec24: [F32;2],
	fRec25: [F32;2],
	fTempPerm60: F32,
	fTempPerm61: F32,
	fRec26: [F32;2],
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
	fTempPerm82: F32,
	fTempPerm83: F32,
	fTempPerm84: F32,
	fTempPerm85: F32,
	fRec37: [F32;2],
	fTempPerm86: F32,
	fTempPerm87: F32,
	fRec38: [F32;2],
	fTempPerm88: F32,
	fTempPerm89: F32,
	fRec39: [F32;2],
	fTempPerm90: F32,
	fTempPerm91: F32,
	fRec40: [F32;2],
	fTempPerm92: F32,
	fTempPerm93: F32,
	fRec41: [F32;2],
	fTempPerm94: F32,
	fTempPerm95: F32,
	fTempPerm96: F32,
	fRec42: [F32;2],
	fTempPerm97: F32,
	fTempPerm98: F32,
	fTempPerm99: F32,
	fRec43: [F32;2],
	fTempPerm100: F32,
	fTempPerm101: F32,
	fTempPerm102: F32,
	fRec44: [F32;2],
	fRec45: [F32;2],
	fRec46: [F32;2],
	fRec47: [F32;2],
	fTempPerm103: F32,
	fTempPerm104: F32,
	fRec48: [F32;2],
	fRec49: [F32;2],
	fTempPerm105: F32,
	fCheckbox0: F32,
	fHslider9: F32,
	fVec7: [F32;2],
	fRec51: [F32;2],
	fRec52: [F32;2],
	fTempPerm106: F32,
	fTempPerm107: F32,
	fTempPerm108: F32,
	fTempPerm109: F32,
	fConst9: F32,
	fTempPerm110: F32,
	fConst10: F32,
	fTempPerm111: F32,
	fTempPerm112: F32,
	fTempPerm113: F32,
	fRec53: [F32;2],
	fTempPerm114: F32,
	fTempPerm115: F32,
	fRec54: [F32;2],
	fHslider10: F32,
	fVec8: [F32;2],
	fRec58: [F32;2],
	fRec59: [F32;2],
	fTempPerm116: F32,
	iTempPerm117: i32,
	fTempPerm118: F32,
	fTempPerm119: F32,
	fTempPerm120: F32,
	fTempPerm121: F32,
	fTempPerm122: F32,
	fTempPerm123: F32,
	fTempPerm124: F32,
	fTempPerm125: F32,
	fTempPerm126: F32,
	fTempPerm127: F32,
	fTempPerm128: F32,
	fRec60: [F32;2],
	fTempPerm129: F32,
	fTempPerm130: F32,
	fRec61: [F32;2],
	fTempPerm131: F32,
	fTempPerm132: F32,
	fTempPerm133: F32,
	fTempPerm134: F32,
	fTempPerm135: F32,
	fVec9: [F32;2],
	fRec10: [F32;2],
	fHslider11: F32,
}

impl FaustDsp for VoiceDsp {
	type T = F32;
		
	fn new() -> VoiceDsp { 
		VoiceDsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			fHslider0: 0.0,
			fVec2: [0.0;2],
			fHslider1: 0.0,
			fRec0: [0.0;2],
			fRec1: [0.0;2],
			iTempPerm0: 0,
			fHslider2: 0.0,
			iRec3: [0;2],
			iTempPerm1: 0,
			fSampleRate: 0,
			fConst1: 0.0,
			fConst3: 0.0,
			fRec2: [0.0;2],
			fConst4: 0.0,
			fHslider3: 0.0,
			fRec4: [0.0;2],
			fConst5: 0.0,
			fTempPerm2: 0.0,
			fRec6: [0.0;2],
			fVslider0: 0.0,
			fVslider1: 0.0,
			fRec7: [0.0;2],
			fConst6: 0.0,
			fVslider2: 0.0,
			fRec8: [0.0;2],
			fVslider3: 0.0,
			fVslider4: 0.0,
			fVslider5: 0.0,
			fRec9: [0.0;2],
			fVslider6: 0.0,
			fRec5: [0.0;2],
			fHslider4: 0.0,
			fVec3: [0.0;2],
			fRec11: [0.0;2],
			fRec12: [0.0;2],
			fConst7: 0.0,
			fHslider5: 0.0,
			fRec13: [0.0;2],
			fEntry0: 0.0,
			fTempPerm3: 0.0,
			fConst8: 0.0,
			fHslider6: 0.0,
			fRec28: [0.0;2],
			fEntry1: 0.0,
			fTempPerm4: 0.0,
			fRec31: [0.0;2],
			iRec32: [0;2],
			fRec30: [0.0;2],
			fTempPerm5: 0.0,
			fVec4: [0.0;2],
			fRec29: [0.0;2],
			fTempPerm6: 0.0,
			fHslider7: 0.0,
			fVec5: [0.0;2],
			fRec33: [0.0;2],
			fRec34: [0.0;2],
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
			fTempPerm21: 0.0,
			fHslider8: 0.0,
			fVec6: [0.0;2],
			fRec35: [0.0;2],
			fRec36: [0.0;2],
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
			fTempPerm32: 0.0,
			fTempPerm33: 0.0,
			fTempPerm34: 0.0,
			fTempPerm35: 0.0,
			fTempPerm36: 0.0,
			fTempPerm37: 0.0,
			fRec14: [0.0;2],
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fRec15: [0.0;2],
			fTempPerm40: 0.0,
			fTempPerm41: 0.0,
			fRec16: [0.0;2],
			fTempPerm42: 0.0,
			fTempPerm43: 0.0,
			fRec17: [0.0;2],
			fTempPerm44: 0.0,
			fTempPerm45: 0.0,
			fTempPerm46: 0.0,
			fTempPerm47: 0.0,
			fRec18: [0.0;2],
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			fRec19: [0.0;2],
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fTempPerm54: 0.0,
			fTempPerm55: 0.0,
			fRec20: [0.0;2],
			fTempPerm56: 0.0,
			fTempPerm57: 0.0,
			fTempPerm58: 0.0,
			fTempPerm59: 0.0,
			fRec21: [0.0;2],
			fRec22: [0.0;2],
			fRec23: [0.0;2],
			fRec24: [0.0;2],
			fRec25: [0.0;2],
			fTempPerm60: 0.0,
			fTempPerm61: 0.0,
			fRec26: [0.0;2],
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
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			fTempPerm84: 0.0,
			fTempPerm85: 0.0,
			fRec37: [0.0;2],
			fTempPerm86: 0.0,
			fTempPerm87: 0.0,
			fRec38: [0.0;2],
			fTempPerm88: 0.0,
			fTempPerm89: 0.0,
			fRec39: [0.0;2],
			fTempPerm90: 0.0,
			fTempPerm91: 0.0,
			fRec40: [0.0;2],
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fRec41: [0.0;2],
			fTempPerm94: 0.0,
			fTempPerm95: 0.0,
			fTempPerm96: 0.0,
			fRec42: [0.0;2],
			fTempPerm97: 0.0,
			fTempPerm98: 0.0,
			fTempPerm99: 0.0,
			fRec43: [0.0;2],
			fTempPerm100: 0.0,
			fTempPerm101: 0.0,
			fTempPerm102: 0.0,
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			fRec46: [0.0;2],
			fRec47: [0.0;2],
			fTempPerm103: 0.0,
			fTempPerm104: 0.0,
			fRec48: [0.0;2],
			fRec49: [0.0;2],
			fTempPerm105: 0.0,
			fCheckbox0: 0.0,
			fHslider9: 0.0,
			fVec7: [0.0;2],
			fRec51: [0.0;2],
			fRec52: [0.0;2],
			fTempPerm106: 0.0,
			fTempPerm107: 0.0,
			fTempPerm108: 0.0,
			fTempPerm109: 0.0,
			fConst9: 0.0,
			fTempPerm110: 0.0,
			fConst10: 0.0,
			fTempPerm111: 0.0,
			fTempPerm112: 0.0,
			fTempPerm113: 0.0,
			fRec53: [0.0;2],
			fTempPerm114: 0.0,
			fTempPerm115: 0.0,
			fRec54: [0.0;2],
			fHslider10: 0.0,
			fVec8: [0.0;2],
			fRec58: [0.0;2],
			fRec59: [0.0;2],
			fTempPerm116: 0.0,
			iTempPerm117: 0,
			fTempPerm118: 0.0,
			fTempPerm119: 0.0,
			fTempPerm120: 0.0,
			fTempPerm121: 0.0,
			fTempPerm122: 0.0,
			fTempPerm123: 0.0,
			fTempPerm124: 0.0,
			fTempPerm125: 0.0,
			fTempPerm126: 0.0,
			fTempPerm127: 0.0,
			fTempPerm128: 0.0,
			fRec60: [0.0;2],
			fTempPerm129: 0.0,
			fTempPerm130: 0.0,
			fRec61: [0.0;2],
			fTempPerm131: 0.0,
			fTempPerm132: 0.0,
			fTempPerm133: 0.0,
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			fVec9: [0.0;2],
			fRec10: [0.0;2],
			fHslider11: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn VoiceDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("filename", r"plugin-voice.dsp");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/pole:author", r"Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", r"MIT-style STK-4.3 license");
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
		m.declare("name", r"plugin-voice");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.0");
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
		return 3;
	}
	fn get_num_outputs(&self) -> i32 {
		return 6;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 64.0;
		self.fHslider2 = 0.0;
		self.fHslider3 = 1.0;
		self.fVslider0 = -0.6;
		self.fVslider1 = 0.5;
		self.fVslider2 = 0.005;
		self.fVslider3 = 0.6;
		self.fVslider4 = -0.6;
		self.fVslider5 = 0.4;
		self.fVslider6 = 0.6;
		self.fHslider4 = 2e+01;
		self.fHslider5 = 1.0;
		self.fEntry0 = 0.0;
		self.fHslider6 = 0.0;
		self.fEntry1 = 0.0;
		self.fHslider7 = 1.2e+03;
		self.fHslider8 = 0.8;
		self.fCheckbox0 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 0.8;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.iVec1[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.fVec2[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec0[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec1[l4 as usize] = 0.0;
		}
		self.iTempPerm0 = 0;
		for l5 in 0..2 {
			self.iRec3[l5 as usize] = 0;
		}
		self.iTempPerm1 = 0;
		for l6 in 0..2 {
			self.fRec2[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec4[l7 as usize] = 0.0;
		}
		self.fTempPerm2 = 0.0;
		for l8 in 0..2 {
			self.fRec6[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec7[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec8[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec9[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec5[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fVec3[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec11[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec12[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec13[l16 as usize] = 0.0;
		}
		self.fTempPerm3 = 0.0;
		for l17 in 0..2 {
			self.fRec28[l17 as usize] = 0.0;
		}
		self.fTempPerm4 = 0.0;
		for l18 in 0..2 {
			self.fRec31[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.iRec32[l19 as usize] = 0;
		}
		for l20 in 0..2 {
			self.fRec30[l20 as usize] = 0.0;
		}
		self.fTempPerm5 = 0.0;
		for l21 in 0..2 {
			self.fVec4[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec29[l22 as usize] = 0.0;
		}
		self.fTempPerm6 = 0.0;
		for l23 in 0..2 {
			self.fVec5[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec33[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec34[l25 as usize] = 0.0;
		}
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
		self.fTempPerm21 = 0.0;
		for l26 in 0..2 {
			self.fVec6[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec35[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec36[l28 as usize] = 0.0;
		}
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
		self.fTempPerm32 = 0.0;
		self.fTempPerm33 = 0.0;
		self.fTempPerm34 = 0.0;
		self.fTempPerm35 = 0.0;
		self.fTempPerm36 = 0.0;
		self.fTempPerm37 = 0.0;
		for l29 in 0..2 {
			self.fRec14[l29 as usize] = 0.0;
		}
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		for l30 in 0..2 {
			self.fRec15[l30 as usize] = 0.0;
		}
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		for l31 in 0..2 {
			self.fRec16[l31 as usize] = 0.0;
		}
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		for l32 in 0..2 {
			self.fRec17[l32 as usize] = 0.0;
		}
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		for l33 in 0..2 {
			self.fRec18[l33 as usize] = 0.0;
		}
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		for l34 in 0..2 {
			self.fRec19[l34 as usize] = 0.0;
		}
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		self.fTempPerm54 = 0.0;
		self.fTempPerm55 = 0.0;
		for l35 in 0..2 {
			self.fRec20[l35 as usize] = 0.0;
		}
		self.fTempPerm56 = 0.0;
		self.fTempPerm57 = 0.0;
		self.fTempPerm58 = 0.0;
		self.fTempPerm59 = 0.0;
		for l36 in 0..2 {
			self.fRec21[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec22[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec23[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec24[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec25[l40 as usize] = 0.0;
		}
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		for l41 in 0..2 {
			self.fRec26[l41 as usize] = 0.0;
		}
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
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.fTempPerm84 = 0.0;
		self.fTempPerm85 = 0.0;
		for l42 in 0..2 {
			self.fRec37[l42 as usize] = 0.0;
		}
		self.fTempPerm86 = 0.0;
		self.fTempPerm87 = 0.0;
		for l43 in 0..2 {
			self.fRec38[l43 as usize] = 0.0;
		}
		self.fTempPerm88 = 0.0;
		self.fTempPerm89 = 0.0;
		for l44 in 0..2 {
			self.fRec39[l44 as usize] = 0.0;
		}
		self.fTempPerm90 = 0.0;
		self.fTempPerm91 = 0.0;
		for l45 in 0..2 {
			self.fRec40[l45 as usize] = 0.0;
		}
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		for l46 in 0..2 {
			self.fRec41[l46 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		self.fTempPerm96 = 0.0;
		for l47 in 0..2 {
			self.fRec42[l47 as usize] = 0.0;
		}
		self.fTempPerm97 = 0.0;
		self.fTempPerm98 = 0.0;
		self.fTempPerm99 = 0.0;
		for l48 in 0..2 {
			self.fRec43[l48 as usize] = 0.0;
		}
		self.fTempPerm100 = 0.0;
		self.fTempPerm101 = 0.0;
		self.fTempPerm102 = 0.0;
		for l49 in 0..2 {
			self.fRec44[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec45[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec46[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec47[l52 as usize] = 0.0;
		}
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		for l53 in 0..2 {
			self.fRec48[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec49[l54 as usize] = 0.0;
		}
		self.fTempPerm105 = 0.0;
		for l55 in 0..2 {
			self.fVec7[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec51[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec52[l57 as usize] = 0.0;
		}
		self.fTempPerm106 = 0.0;
		self.fTempPerm107 = 0.0;
		self.fTempPerm108 = 0.0;
		self.fTempPerm109 = 0.0;
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		self.fTempPerm112 = 0.0;
		self.fTempPerm113 = 0.0;
		for l58 in 0..2 {
			self.fRec53[l58 as usize] = 0.0;
		}
		self.fTempPerm114 = 0.0;
		self.fTempPerm115 = 0.0;
		for l59 in 0..2 {
			self.fRec54[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fVec8[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec58[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec59[l62 as usize] = 0.0;
		}
		self.fTempPerm116 = 0.0;
		self.iTempPerm117 = 0;
		self.fTempPerm118 = 0.0;
		self.fTempPerm119 = 0.0;
		self.fTempPerm120 = 0.0;
		self.fTempPerm121 = 0.0;
		self.fTempPerm122 = 0.0;
		self.fTempPerm123 = 0.0;
		self.fTempPerm124 = 0.0;
		self.fTempPerm125 = 0.0;
		self.fTempPerm126 = 0.0;
		self.fTempPerm127 = 0.0;
		self.fTempPerm128 = 0.0;
		for l63 in 0..2 {
			self.fRec60[l63 as usize] = 0.0;
		}
		self.fTempPerm129 = 0.0;
		self.fTempPerm130 = 0.0;
		for l64 in 0..2 {
			self.fRec61[l64 as usize] = 0.0;
		}
		self.fTempPerm131 = 0.0;
		self.fTempPerm132 = 0.0;
		self.fTempPerm133 = 0.0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		for l65 in 0..2 {
			self.fVec9[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec10[l66 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::exp(-(157.07964 / fConst0));
		let mut fConst2: F32 = 1.0 - self.fConst1;
		self.fConst3 = 4.656613e-10 * F32::sqrt(3.0 * ((self.fConst1 + 1.0) / fConst2));
		self.fConst4 = 1.0 - 44.1 / fConst0;
		self.fConst5 = 0.11626748 * fConst2;
		self.fConst6 = 1.0 / fConst0;
		self.fConst7 = 3.1415927 / fConst0;
		self.fConst8 = 0.45 * fConst0;
		self.fConst9 = F32::tanh(0.15);
		self.fConst10 = 0.5882353 / (1.0 - VoiceDsp_faustpower2_f(self.fConst9));
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		VoiceDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin-voice");
		ui_interface.declare(None, "0.5", "");
		ui_interface.open_horizontal_box("Vintage");
		ui_interface.declare(Some(ParamIndex(0)), "1", "");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(0), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Perform");
		ui_interface.declare(Some(ParamIndex(1)), "5", "");
		ui_interface.declare(Some(ParamIndex(1)), "style", "menu{'2 Semitones':0;'Octave':1;'Cutoff 50%':2;'Resonance 50%':3;'Bandpass 50%':4}");
		ui_interface.add_num_entry("Bend Mode", ParamIndex(1), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "13", "");
		ui_interface.open_horizontal_box("Settings");
		ui_interface.declare(Some(ParamIndex(2)), "1", "");
		ui_interface.add_horizontal_slider("Vintage Noise", ParamIndex(2), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(3)), "2", "");
		ui_interface.add_horizontal_slider("Heat", ParamIndex(3), 1.0, 0.0, 2.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("Filter");
		ui_interface.declare(Some(ParamIndex(4)), "0", "");
		ui_interface.declare(Some(ParamIndex(4)), "style", "menu{'SVF':0;'Ladder':1;'SH-101':2}");
		ui_interface.add_num_entry("Type", ParamIndex(4), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.declare(Some(ParamIndex(5)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(5)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Cutoff", ParamIndex(5), 1.2e+03, 2e+01, 1.8e+04, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "2", "");
		ui_interface.add_horizontal_slider("Resonance", ParamIndex(6), 0.8, 0.5, 6.0, 0.01);
		ui_interface.declare(Some(ParamIndex(7)), "3", "");
		ui_interface.add_horizontal_slider("Morph LP-Notch-HP", ParamIndex(7), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(8)), "4", "");
		ui_interface.add_horizontal_slider("Bandpass", ParamIndex(8), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(9)), "5", "");
		ui_interface.add_check_button("4-Pole", ParamIndex(9));
		ui_interface.declare(Some(ParamIndex(10)), "8", "");
		ui_interface.declare(Some(ParamIndex(10)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(10)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Utility HPF", ParamIndex(10), 2e+01, 1e+01, 2e+03, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("Amp");
		ui_interface.declare(Some(ParamIndex(11)), "1", "");
		ui_interface.declare(Some(ParamIndex(11)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(11)), "unit", "s");
		ui_interface.add_vertical_slider("Attack", ParamIndex(11), 0.005, 0.005, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "2", "");
		ui_interface.declare(Some(ParamIndex(12)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(12)), "unit", "s");
		ui_interface.add_vertical_slider("Decay", ParamIndex(12), 0.4, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(13)), "3", "");
		ui_interface.add_vertical_slider("Sustain", ParamIndex(13), 0.6, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(14)), "4", "");
		ui_interface.declare(Some(ParamIndex(14)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(14)), "unit", "s");
		ui_interface.add_vertical_slider("Release", ParamIndex(14), 0.5, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(15)), "5", "");
		ui_interface.add_vertical_slider("Attack Curve", ParamIndex(15), 0.6, -1.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(16)), "6", "");
		ui_interface.add_vertical_slider("Decay Curve", ParamIndex(16), -0.6, -1.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(17)), "7", "");
		ui_interface.add_vertical_slider("Release Curve", ParamIndex(17), -0.6, -1.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(18)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(18)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(18), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("gain", ParamIndex(19), 0.8, 0.0, 1.0, 0.01);
		ui_interface.add_button("gate", ParamIndex(20));
		ui_interface.add_horizontal_slider("ramp", ParamIndex(21), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("slot", ParamIndex(22), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			20 => Some(self.fButton0),
			9 => Some(self.fCheckbox0),
			4 => Some(self.fEntry0),
			1 => Some(self.fEntry1),
			0 => Some(self.fHslider0),
			21 => Some(self.fHslider1),
			7 => Some(self.fHslider10),
			19 => Some(self.fHslider11),
			22 => Some(self.fHslider2),
			2 => Some(self.fHslider3),
			10 => Some(self.fHslider4),
			3 => Some(self.fHslider5),
			18 => Some(self.fHslider6),
			5 => Some(self.fHslider7),
			6 => Some(self.fHslider8),
			8 => Some(self.fHslider9),
			17 => Some(self.fVslider0),
			14 => Some(self.fVslider1),
			11 => Some(self.fVslider2),
			13 => Some(self.fVslider3),
			16 => Some(self.fVslider4),
			12 => Some(self.fVslider5),
			15 => Some(self.fVslider6),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			20 => { self.fButton0 = value }
			9 => { self.fCheckbox0 = value }
			4 => { self.fEntry0 = value }
			1 => { self.fEntry1 = value }
			0 => { self.fHslider0 = value }
			21 => { self.fHslider1 = value }
			7 => { self.fHslider10 = value }
			19 => { self.fHslider11 = value }
			22 => { self.fHslider2 = value }
			2 => { self.fHslider3 = value }
			10 => { self.fHslider4 = value }
			3 => { self.fHslider5 = value }
			18 => { self.fHslider6 = value }
			5 => { self.fHslider7 = value }
			6 => { self.fHslider8 = value }
			8 => { self.fHslider9 = value }
			17 => { self.fVslider0 = value }
			14 => { self.fVslider1 = value }
			11 => { self.fVslider2 = value }
			13 => { self.fVslider3 = value }
			16 => { self.fVslider4 = value }
			12 => { self.fVslider5 = value }
			15 => { self.fVslider6 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2) = if let [inputs0, inputs1, inputs2, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			(inputs0, inputs1, inputs2)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1, outputs2, outputs3, outputs4, outputs5) = if let [outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			let outputs3 = outputs3[..count as usize].iter_mut();
			let outputs4 = outputs4[..count as usize].iter_mut();
			let outputs5 = outputs5[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2, outputs3, outputs4, outputs5)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = self.fHslider0;
		let mut fSlow2: F32 = self.fHslider1;
		let mut iTemp5: i32 = self.iTempPerm0;
		let mut fSlow3: F32 = self.fHslider2;
		let mut iSlow4: i32 = (521.0 * fSlow3 + 5e+03) as i32;
		let mut iTemp6: i32 = self.iTempPerm1;
		let mut fSlow5: F32 = self.fHslider3;
		let mut fTemp9: F32 = self.fTempPerm2;
		let mut iSlow6: i32 = (fSlow0) as i32;
		let mut fSlow7: F32 = self.fVslider0;
		let mut fSlow8: F32 = 12.9898 * fSlow3;
		let mut fSlow9: F32 = 43758.547 * F32::sin(fSlow8 + 7901.533);
		let mut fSlow10: F32 = 2.0 * (fSlow9 - F32::floor(fSlow9)) + -1.0;
		let mut fSlow11: F32 = 0.4 * fSlow10;
		let mut fSlow12: F32 = 43758.547 * F32::sin(fSlow8 + 625.864);
		let mut fSlow13: F32 = 43758.547 * F32::sin(fSlow8 + 7979.766);
		let mut fSlow14: F32 = 2.0 * (fSlow13 - F32::floor(fSlow13)) + -1.0;
		let mut fSlow15: F32 = 0.25 * F32::max(-1.0, F32::min(1.0, 1.230915 * (0.5 * (fSlow14 + 2.0 * (fSlow12 - F32::floor(fSlow12)) + -1.0) - fSlow11)));
		let mut fSlow16: F32 = self.fVslider1;
		let mut fSlow17: F32 = 1.0 - fSlow0;
		let mut fSlow18: F32 = 43758.547 * F32::sin(fSlow8 + 469.398);
		let mut fSlow19: F32 = 0.25 * F32::max(-1.0, F32::min(1.0, 1.1952286 * (0.6 * fSlow14 + 0.5 * (2.0 * (fSlow18 - F32::floor(fSlow18)) + -1.0) - 0.3 * fSlow10)));
		let mut fSlow20: F32 = self.fVslider2;
		let mut fSlow21: F32 = self.fVslider3;
		let mut fSlow22: F32 = self.fVslider4;
		let mut fSlow23: F32 = if (fSlow21 >= 1.0) as i32 != 0 {fSlow22} else {-fSlow22};
		let mut fSlow24: F32 = 5.0 * fSlow23;
		let mut iSlow25: i32 = (F32::abs(fSlow24) < 0.001) as i32;
		let mut fSlow26: F32 = 1.0 - F32::exp(-fSlow24);
		let mut fSlow27: F32 = 43758.547 * F32::sin(fSlow8 + 547.631);
		let mut fSlow28: F32 = 0.25 * F32::max(-1.0, F32::min(1.0, 1.230915 * (0.5 * (fSlow14 + 2.0 * (fSlow27 - F32::floor(fSlow27)) + -1.0) - fSlow11)));
		let mut fSlow29: F32 = self.fVslider5;
		let mut fSlow30: F32 = fSlow21 + -1.0;
		let mut fSlow31: F32 = self.fVslider6;
		let mut fSlow32: F32 = self.fHslider4;
		let mut fSlow33: F32 = self.fHslider5;
		let mut fSlow34: F32 = self.fEntry0;
		let mut iSlow35: i32 = (fSlow34 == 2.0) as i32;
		let mut fSlow36: F32 = (iSlow35) as F32;
		let mut iSlow37: i32 = (fSlow36 != 0.0) as i32;
		let mut fTemp28: F32 = self.fTempPerm3;
		let mut fSlow38: F32 = self.fHslider6;
		let mut fSlow39: F32 = self.fEntry1;
		let mut fSlow40: F32 = 0.5 * ((fSlow39 == 2.0) as i32) as u32 as F32;
		let mut fSlow41: F32 = 43758.547 * F32::sin(fSlow8 + 391.165);
		let mut fSlow42: F32 = 0.1 * F32::max(-1.0, F32::min(1.0, 1.7149858 * (0.3 * fSlow14 + 0.5 * (2.0 * (fSlow41 - F32::floor(fSlow41)) + -1.0)))) + 0.2;
		let mut fSlow43: F32 = 1.0 / F32::tan(self.fConst7 * F32::max(1.0, fSlow42));
		let mut fSlow44: F32 = 1.0 - fSlow43;
		let mut fSlow45: F32 = self.fConst6 * F32::max(fSlow42, 0.0);
		let mut fTemp29: F32 = self.fTempPerm4;
		let mut fTemp30: F32 = self.fTempPerm5;
		let mut fSlow46: F32 = 1.0 / (fSlow43 + 1.0);
		let mut fSlow47: F32 = 43758.547 * F32::sin(fSlow8 + 312.932);
		let mut fSlow48: F32 = 0.25 * F32::max(-1.0, F32::min(1.0, 1.1396058 * (0.6 * fSlow10 + 0.5 * (2.0 * (fSlow47 - F32::floor(fSlow47)) + -1.0) - 0.4 * fSlow14)));
		let mut fTemp31: F32 = self.fTempPerm6;
		let mut fSlow49: F32 = self.fHslider7;
		let mut iSlow50: i32 = (fSlow34 == 1.0) as i32;
		let mut fSlow51: F32 = (iSlow50) as F32;
		let mut iSlow52: i32 = (fSlow51 != 0.0) as i32;
		let mut iSlow53: i32 = (fSlow34 == 0.0) as i32;
		let mut fSlow54: F32 = (iSlow53) as F32;
		let mut iSlow55: i32 = (fSlow54 != 0.0) as i32;
		let mut fTemp34: F32 = self.fTempPerm7;
		let mut fTemp35: F32 = self.fTempPerm8;
		let mut fTemp36: F32 = self.fTempPerm9;
		let mut fTemp37: F32 = self.fTempPerm10;
		let mut fTemp38: F32 = self.fTempPerm11;
		let mut fTemp39: F32 = self.fTempPerm12;
		let mut fTemp40: F32 = self.fTempPerm13;
		let mut fTemp41: F32 = self.fTempPerm14;
		let mut fTemp42: F32 = self.fTempPerm15;
		let mut fTemp43: F32 = self.fTempPerm16;
		let mut fTemp44: F32 = self.fTempPerm17;
		let mut fTemp45: F32 = self.fTempPerm18;
		let mut fTemp46: F32 = self.fTempPerm19;
		let mut fTemp47: F32 = self.fTempPerm20;
		let mut fTemp48: F32 = self.fTempPerm21;
		let mut fSlow56: F32 = 0.5 * ((fSlow39 == 3.0) as i32) as u32 as F32;
		let mut fSlow57: F32 = self.fHslider8;
		let mut fTemp51: F32 = self.fTempPerm22;
		let mut fTemp52: F32 = self.fTempPerm23;
		let mut fTemp53: F32 = self.fTempPerm24;
		let mut fTemp54: F32 = self.fTempPerm25;
		let mut fTemp55: F32 = self.fTempPerm26;
		let mut fTemp56: F32 = self.fTempPerm27;
		let mut fTemp57: F32 = self.fTempPerm28;
		let mut fTemp58: F32 = self.fTempPerm29;
		let mut fTemp59: F32 = self.fTempPerm30;
		let mut fTemp60: F32 = self.fTempPerm31;
		let mut fTemp61: F32 = self.fTempPerm32;
		let mut fTemp62: F32 = self.fTempPerm33;
		let mut fTemp63: F32 = self.fTempPerm34;
		let mut fTemp64: F32 = self.fTempPerm35;
		let mut fTemp65: F32 = self.fTempPerm36;
		let mut fTemp66: F32 = self.fTempPerm37;
		let mut fTemp67: F32 = self.fTempPerm38;
		let mut fTemp68: F32 = self.fTempPerm39;
		let mut fTemp69: F32 = self.fTempPerm40;
		let mut fTemp70: F32 = self.fTempPerm41;
		let mut fTemp71: F32 = self.fTempPerm42;
		let mut fTemp72: F32 = self.fTempPerm43;
		let mut fTemp73: F32 = self.fTempPerm44;
		let mut fTemp74: F32 = self.fTempPerm45;
		let mut fTemp75: F32 = self.fTempPerm46;
		let mut fTemp76: F32 = self.fTempPerm47;
		let mut fTemp77: F32 = self.fTempPerm48;
		let mut fTemp78: F32 = self.fTempPerm49;
		let mut fTemp79: F32 = self.fTempPerm50;
		let mut fTemp80: F32 = self.fTempPerm51;
		let mut fTemp81: F32 = self.fTempPerm52;
		let mut fTemp82: F32 = self.fTempPerm53;
		let mut fTemp83: F32 = self.fTempPerm54;
		let mut fTemp84: F32 = self.fTempPerm55;
		let mut fTemp85: F32 = self.fTempPerm56;
		let mut fTemp86: F32 = self.fTempPerm57;
		let mut fTemp87: F32 = self.fTempPerm58;
		let mut fTemp88: F32 = self.fTempPerm59;
		let mut fTemp89: F32 = self.fTempPerm60;
		let mut fTemp90: F32 = self.fTempPerm61;
		let mut fSlow58: F32 = 2.0 * fSlow36;
		let mut fTemp91: F32 = self.fTempPerm62;
		let mut fSlow59: F32 = ((iSlow35 != 0) as i32) as u32 as F32;
		let mut fTemp92: F32 = self.fTempPerm63;
		let mut fTemp93: F32 = self.fTempPerm64;
		let mut fTemp94: F32 = self.fTempPerm65;
		let mut fTemp95: F32 = self.fTempPerm66;
		let mut fTemp96: F32 = self.fTempPerm67;
		let mut fTemp97: F32 = self.fTempPerm68;
		let mut fTemp98: F32 = self.fTempPerm69;
		let mut fTemp99: F32 = self.fTempPerm70;
		let mut fTemp100: F32 = self.fTempPerm71;
		let mut fTemp101: F32 = self.fTempPerm72;
		let mut fTemp102: F32 = self.fTempPerm73;
		let mut fTemp103: F32 = self.fTempPerm74;
		let mut fTemp104: F32 = self.fTempPerm75;
		let mut fTemp105: F32 = self.fTempPerm76;
		let mut fTemp106: F32 = self.fTempPerm77;
		let mut fTemp107: F32 = self.fTempPerm78;
		let mut fTemp108: F32 = self.fTempPerm79;
		let mut fTemp109: F32 = self.fTempPerm80;
		let mut fTemp110: F32 = self.fTempPerm81;
		let mut fTemp111: F32 = self.fTempPerm82;
		let mut fTemp112: F32 = self.fTempPerm83;
		let mut fTemp113: F32 = self.fTempPerm84;
		let mut fTemp114: F32 = self.fTempPerm85;
		let mut fTemp115: F32 = self.fTempPerm86;
		let mut fTemp116: F32 = self.fTempPerm87;
		let mut fTemp117: F32 = self.fTempPerm88;
		let mut fTemp118: F32 = self.fTempPerm89;
		let mut fTemp119: F32 = self.fTempPerm90;
		let mut fTemp120: F32 = self.fTempPerm91;
		let mut fTemp121: F32 = self.fTempPerm92;
		let mut fTemp122: F32 = self.fTempPerm93;
		let mut fTemp123: F32 = self.fTempPerm94;
		let mut fTemp124: F32 = self.fTempPerm95;
		let mut fTemp125: F32 = self.fTempPerm96;
		let mut fTemp126: F32 = self.fTempPerm97;
		let mut fTemp127: F32 = self.fTempPerm98;
		let mut fTemp128: F32 = self.fTempPerm99;
		let mut fTemp129: F32 = self.fTempPerm100;
		let mut fTemp130: F32 = self.fTempPerm101;
		let mut fTemp131: F32 = self.fTempPerm102;
		let mut fTemp132: F32 = self.fTempPerm103;
		let mut fTemp133: F32 = self.fTempPerm104;
		let mut fSlow60: F32 = 2.0 * fSlow51;
		let mut fTemp134: F32 = self.fTempPerm105;
		let mut fSlow61: F32 = ((iSlow50 != 0) as i32) as u32 as F32;
		let mut fSlow62: F32 = self.fCheckbox0;
		let mut iSlow63: i32 = (fSlow62) as i32;
		let mut fSlow64: F32 = 0.5 * ((fSlow39 == 4.0) as i32) as u32 as F32;
		let mut fSlow65: F32 = self.fHslider9;
		let mut fTemp137: F32 = self.fTempPerm106;
		let mut fTemp138: F32 = self.fTempPerm107;
		let mut fTemp139: F32 = self.fTempPerm108;
		let mut fTemp140: F32 = self.fTempPerm109;
		let mut fTemp141: F32 = self.fTempPerm110;
		let mut fTemp142: F32 = self.fTempPerm111;
		let mut fTemp143: F32 = self.fTempPerm112;
		let mut fTemp144: F32 = self.fTempPerm113;
		let mut fTemp145: F32 = self.fTempPerm114;
		let mut fTemp146: F32 = self.fTempPerm115;
		let mut fSlow66: F32 = self.fHslider10;
		let mut fTemp148: F32 = self.fTempPerm116;
		let mut iTemp149: i32 = self.iTempPerm117;
		let mut fTemp150: F32 = self.fTempPerm118;
		let mut fTemp151: F32 = self.fTempPerm119;
		let mut fTemp152: F32 = self.fTempPerm120;
		let mut fTemp153: F32 = self.fTempPerm121;
		let mut fTemp154: F32 = self.fTempPerm122;
		let mut fTemp155: F32 = self.fTempPerm123;
		let mut fTemp156: F32 = self.fTempPerm124;
		let mut iSlow67: i32 = (fSlow62 != 0.0) as i32;
		let mut fTemp157: F32 = self.fTempPerm125;
		let mut fTemp158: F32 = self.fTempPerm126;
		let mut fTemp159: F32 = self.fTempPerm127;
		let mut fTemp160: F32 = self.fTempPerm128;
		let mut fTemp161: F32 = self.fTempPerm129;
		let mut fTemp162: F32 = self.fTempPerm130;
		let mut fTemp163: F32 = self.fTempPerm131;
		let mut fTemp164: F32 = self.fTempPerm132;
		let mut fSlow68: F32 = (iSlow67) as F32;
		let mut fTemp165: F32 = self.fTempPerm133;
		let mut fSlow69: F32 = ((iSlow53 != 0) as i32) as u32 as F32;
		let mut fTemp168: F32 = self.fTempPerm134;
		let mut fTemp169: F32 = self.fTempPerm135;
		let mut fSlow70: F32 = 0.9 * self.fHslider11;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5);
		for ((((((((input0, input1), input2), output0), output1), output2), output3), output4), output5) in zipped_iterators {
			let mut fRec27: F32 = 0.0;
			let mut fRec50: F32 = 0.0;
			let mut fRec55: F32 = 0.0;
			let mut fRec56: F32 = 0.0;
			let mut fRec57: F32 = 0.0;
			let mut fRec62: F32 = 0.0;
			let mut fRec63: F32 = 0.0;
			let mut fRec64: F32 = 0.0;
			self.fVec0[0] = fSlow0;
			self.iVec1[0] = 1;
			let mut iTemp0: i32 = (fSlow0 > self.fVec0[1]) as i32;
			self.fVec2[0] = fSlow1;
			let mut fTemp1: F32 = if ((fSlow1 != self.fVec2[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec0[1] + -1.0};
			self.fRec0[0] = fTemp1;
			self.fRec1[0] = if iTemp0 != 0 {fSlow1} else {if (fTemp1 > 0.0) as i32 != 0 {self.fRec1[1] + (fSlow1 - self.fRec1[1]) / F32::max(1.0, fTemp1)} else {fSlow1}};
			let mut iTemp2: i32 = (F32::abs(self.fRec1[0]) > 0.0001) as i32;
			let mut fTemp3: F32 = (iTemp2) as F32;
			let mut iTemp4: i32 = (fTemp3 != 0.0) as i32;
			if iTemp4 != 0 {
				iTemp5 = i32::wrapping_sub(1, self.iVec1[1]);
				self.iRec3[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec3[1], i32::wrapping_mul(iSlow4, iTemp5))), 12345);
				iTemp6 = i32::wrapping_mul(1664525, self.iRec3[0] ^ (self.iRec3[0] >> 15));
				self.fRec2[0] = self.fConst1 * self.fRec2[1] + self.fConst3 * (i32::wrapping_mul(1013904223, iTemp6 ^ (iTemp6 >> 13))) as F32;
			}
				let mut fTemp7: F32 = (i32::wrapping_sub(1, iTemp0)) as F32;
				let mut fTemp8: F32 = 1.0 - self.fConst4 * fTemp7;
				if iTemp4 != 0 {
					self.fRec4[0] = fSlow5 * fTemp8 + self.fConst4 * fTemp7 * self.fRec4[1];
					fTemp9 = self.fConst5 * self.fRec1[0] * self.fRec4[0] * self.fRec2[0] * fTemp3;
				}
					let mut fTemp10: F32 = ((iTemp2 != 0) as i32) as u32 as F32;
					let mut fTemp11: F32 = fTemp10 * fTemp9;
					let mut fTemp12: F32 = F32::floor(fTemp11);
					let mut iTemp13: i32 = (fTemp12) as i32;
					let mut fTemp14: F32 = fTemp11 - fTemp12;
					self.fRec6[0] = if (fSlow0 < self.fVec0[1]) as i32 != 0 {self.fRec5[1]} else {self.fRec6[1]};
					let mut fTemp15: F32 = if (0.0 >= self.fRec6[0]) as i32 != 0 {fSlow7} else {-fSlow7};
					let mut fTemp16: F32 = 5.0 * fTemp15;
					self.fRec7[0] = fSlow17 * (self.fRec7[1] + 1.0);
					let mut fTemp17: F32 = F32::max(0.0, F32::min(1.0, self.fConst6 * (self.fRec7[0] / F32::max(fSlow16 * (fSlow15 * self.fRec1[0] + 1.0), 0.0001))));
					let mut fTemp18: F32 = F32::max(F32::max(0.005, fSlow20 * (fSlow19 * self.fRec1[0] + 1.0)), 0.0001);
					self.fRec8[0] = fSlow0 * (self.fRec8[1] + 1.0);
					let mut fTemp19: F32 = self.fConst6 * self.fRec8[0];
					let mut fTemp20: F32 = F32::max(0.0, F32::min(1.0, (fTemp19 - fTemp18) / F32::max(fSlow29 * (fSlow28 * self.fRec1[0] + 1.0), 0.0001)));
					self.fRec9[0] = if iTemp0 != 0 {self.fRec5[1]} else {self.fRec9[1]};
					let mut fTemp21: F32 = if (1.0 >= self.fRec9[0]) as i32 != 0 {fSlow31} else {-fSlow31};
					let mut fTemp22: F32 = 5.0 * fTemp21;
					let mut fTemp23: F32 = F32::max(0.0, F32::min(1.0, self.fConst6 * (self.fRec8[0] / fTemp18)));
					self.fRec5[0] = if iSlow6 != 0 {if (fTemp19 < fTemp18) as i32 != 0 {self.fRec9[0] + (1.0 - self.fRec9[0]) * if (F32::abs(fTemp22) < 0.001) as i32 != 0 {fTemp23} else {(1.0 - F32::exp(-(5.0 * fTemp21 * fTemp23))) / (1.0 - F32::exp(-fTemp22))}} else {fSlow30 * if iSlow25 != 0 {fTemp20} else {(1.0 - F32::exp(-(5.0 * fSlow23 * fTemp20))) / fSlow26} + 1.0}} else {self.fRec6[0] * (1.0 - if (F32::abs(fTemp16) < 0.001) as i32 != 0 {fTemp17} else {(1.0 - F32::exp(-(5.0 * fTemp15 * fTemp17))) / (1.0 - F32::exp(-fTemp16))})};
					self.fVec3[0] = fSlow32;
					let mut fTemp24: F32 = if ((fSlow32 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec11[1] + -1.0};
					self.fRec11[0] = fTemp24;
					self.fRec12[0] = if iTemp0 != 0 {fSlow32} else {if (fTemp24 > 0.0) as i32 != 0 {self.fRec12[1] + (fSlow32 - self.fRec12[1]) / F32::max(1.0, fTemp24)} else {fSlow32}};
					let mut fTemp25: F32 = F32::tan(self.fConst7 * F32::max(1e+01, F32::min(2e+03, self.fRec12[0])));
					let mut fTemp26: F32 = 1.0 / fTemp25;
					self.fRec13[0] = fSlow33 * fTemp8 + self.fConst4 * fTemp7 * self.fRec13[1];
					let mut iTemp27: i32 = (self.fRec13[0] > 0.0) as i32;
					if iSlow37 != 0 {
						fTemp28 = 1.0 - self.fRec22[1];
					}
						self.fRec28[0] = fSlow38 * fTemp8 + self.fConst4 * fTemp7 * self.fRec28[1];
						if iTemp4 != 0 {
							fTemp29 = if iTemp5 != 0 {0.0} else {fSlow45 + self.fRec31[1]};
							self.fRec31[0] = fTemp29 - F32::floor(fTemp29);
							self.iRec32[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec32[1]), 12345);
							self.fRec30[0] = if (self.fRec31[0] < self.fRec31[1]) as i32 != 0 {4.656613e-10 * (self.iRec32[0]) as F32} else {self.fRec30[1]};
							fTemp30 = F32::floor(32768.0 * self.fRec30[0] + 0.5);
							self.fVec4[0] = fTemp30;
							self.fRec29[0] = fSlow46 * (3.0517578e-05 * (fTemp30 + self.fVec4[1]) - fSlow44 * self.fRec29[1]);
							fTemp31 = self.fRec1[0] * fTemp3 * (fSlow48 + 0.1 * self.fRec29[0]);
						}
							self.fVec5[0] = fSlow49;
							let mut fTemp32: F32 = if ((fSlow49 != self.fVec5[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec33[1] + -1.0};
							self.fRec33[0] = fTemp32;
							self.fRec34[0] = if iTemp0 != 0 {fSlow49} else {if (fTemp32 > 0.0) as i32 != 0 {self.fRec34[1] + (fSlow49 - self.fRec34[1]) / F32::max(1.0, fTemp32)} else {fSlow49}};
							let mut fTemp33: F32 = F32::max(2e+01, F32::min(1.8e+04, self.fRec34[0] * F32::exp(0.6931472 * fTemp10 * fTemp31) * (fSlow40 * self.fRec28[0] + 1.0)));
							if iSlow37 | (iSlow55 | iSlow52) != 0 {
								fTemp34 = F32::min(self.fConst8, fTemp33);
							}
								if iSlow37 | iSlow52 != 0 {
									fTemp35 = F32::tan(self.fConst7 * F32::max(1e+01, fTemp34));
								}
									if iSlow37 != 0 {
										fTemp36 = fTemp35 * fTemp28 + 1.0;
										fTemp37 = 1.0 - self.fRec25[1];
										fTemp38 = fTemp35 * fTemp37 + 1.0;
										fTemp39 = 1.0 - self.fRec24[1];
										fTemp40 = fTemp35 * fTemp39 + 1.0;
										fTemp41 = 1.0 - self.fRec23[1];
										fTemp42 = fTemp35 * fTemp41 + 1.0;
										fTemp43 = fTemp36 * fTemp42 * fTemp40;
										fTemp44 = 1.0 - self.fRec21[1];
										fTemp45 = 1.0 - self.fRec20[1];
										fTemp46 = 1.0 - self.fRec19[1];
										fTemp47 = 1.0 - self.fRec26[1];
										fTemp48 = 1.0 - self.fRec18[1];
									}
										self.fVec6[0] = fSlow57;
										let mut fTemp49: F32 = if ((fSlow57 != self.fVec6[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec35[1] + -1.0};
										self.fRec35[0] = fTemp49;
										self.fRec36[0] = if iTemp0 != 0 {fSlow57} else {if (fTemp49 > 0.0) as i32 != 0 {self.fRec36[1] + (fSlow57 - self.fRec36[1]) / F32::max(1.0, fTemp49)} else {fSlow57}};
										let mut fTemp50: F32 = F32::max(0.5, F32::min(8.0, self.fRec36[0] * (fSlow56 * self.fRec28[0] + 1.0)));
										if iSlow37 | iSlow52 != 0 {
											fTemp51 = F32::max(0.0, F32::min(1.0, 0.18181819 * (fTemp50 + -0.5)));
											fTemp52 = VoiceDsp_faustpower4_f(fTemp35);
										}
											if iSlow37 != 0 {
												fTemp53 = 4.4 * (fTemp52 * fTemp51 * fTemp48 * fTemp47 * fTemp46 * fTemp45 * fTemp44 / (fTemp43 * fTemp38)) + 1.0;
												fTemp54 = fTemp38 * fTemp53;
											}
												if iSlow37 | (iSlow55 | iSlow52) != 0 {
													fTemp55 = *input0;
												}
													if iSlow37 | iSlow52 != 0 {
														fTemp56 = fTemp55 * fTemp52;
													}
														if iSlow37 != 0 {
															fTemp57 = fTemp35 * fTemp46;
															fTemp58 = fTemp35 * fTemp45;
															fTemp59 = fTemp35 * fTemp44;
															fTemp60 = self.fRec17[1] + fTemp59 * (self.fRec16[1] + fTemp58 * (self.fRec15[1] + fTemp57 * self.fRec14[1] / fTemp36) / fTemp42) / fTemp40 + 0.5 * (fTemp56 * fTemp48 * fTemp46 * fTemp45 * fTemp44 / fTemp43);
															fTemp61 = fTemp51 * fTemp47;
															fTemp62 = 4.4 * (fTemp61 * fTemp60 / fTemp54);
															fTemp63 = 0.5 * fTemp55;
															fTemp64 = fTemp63 - fTemp62;
															fTemp65 = self.fRec14[1] + fTemp35 * fTemp48 * fTemp64;
															fTemp66 = fTemp65 / fTemp36;
															self.fRec14[0] = fTemp66 + fTemp35 * (fTemp48 * fTemp64 - fTemp28 * fTemp65 / fTemp36);
															fTemp67 = self.fRec15[1] + fTemp57 * fTemp65 / fTemp36;
															fTemp68 = fTemp67 / fTemp42;
															self.fRec15[0] = fTemp68 + fTemp35 * (fTemp46 * fTemp65 / fTemp36 - fTemp41 * fTemp67 / fTemp42);
															fTemp69 = self.fRec16[1] + fTemp58 * fTemp67 / fTemp42;
															fTemp70 = fTemp69 / fTemp40;
															self.fRec16[0] = fTemp70 + fTemp35 * (fTemp45 * fTemp67 / fTemp42 - fTemp39 * fTemp69 / fTemp40);
															fTemp71 = self.fRec17[1] + fTemp59 * fTemp69 / fTemp40;
															fTemp72 = fTemp71 / fTemp38;
															self.fRec17[0] = fTemp72 + fTemp35 * (fTemp44 * fTemp69 / fTemp40 - fTemp37 * fTemp71 / fTemp38);
															fTemp73 = fTemp63 - (fTemp62 + fTemp66);
															fTemp74 = VoiceDsp_faustpower2_f(fTemp73);
															fTemp75 = 0.25 * fTemp74;
															fTemp76 = 1.0 - if (fTemp75 < 9.0) as i32 != 0 {(fTemp75 + 27.0) / (2.25 * fTemp74 + 27.0)} else {1.0 / F32::max(F32::abs(0.5 * fTemp73), 1e-09)};
															self.fRec18[0] = fTemp76;
															fTemp77 = fTemp66 - fTemp68;
															fTemp78 = VoiceDsp_faustpower2_f(fTemp77);
															fTemp79 = 0.25 * fTemp78;
															fTemp80 = 1.0 - if (fTemp79 < 9.0) as i32 != 0 {(fTemp79 + 27.0) / (2.25 * fTemp78 + 27.0)} else {1.0 / F32::max(F32::abs(0.5 * fTemp77), 1e-09)};
															self.fRec19[0] = fTemp80;
															fTemp81 = fTemp68 - fTemp70;
															fTemp82 = VoiceDsp_faustpower2_f(fTemp81);
															fTemp83 = 0.25 * fTemp82;
															fTemp84 = 1.0 - if (fTemp83 < 9.0) as i32 != 0 {(fTemp83 + 27.0) / (2.25 * fTemp82 + 27.0)} else {1.0 / F32::max(F32::abs(0.5 * fTemp81), 1e-09)};
															self.fRec20[0] = fTemp84;
															fTemp85 = fTemp70 - fTemp72;
															fTemp86 = VoiceDsp_faustpower2_f(fTemp85);
															fTemp87 = 0.25 * fTemp86;
															fTemp88 = 1.0 - if (fTemp87 < 9.0) as i32 != 0 {(fTemp87 + 27.0) / (2.25 * fTemp86 + 27.0)} else {1.0 / F32::max(F32::abs(0.5 * fTemp85), 1e-09)};
															self.fRec21[0] = fTemp88;
															self.fRec22[0] = fTemp76;
															self.fRec23[0] = fTemp80;
															self.fRec24[0] = fTemp84;
															self.fRec25[0] = fTemp88;
															fTemp89 = VoiceDsp_faustpower2_f(fTemp60) / (VoiceDsp_faustpower2_f(fTemp38) * VoiceDsp_faustpower2_f(fTemp53));
															fTemp90 = 25.0 * fTemp89;
															self.fRec26[0] = 1.0 - if (fTemp90 < 9.0) as i32 != 0 {(fTemp90 + 27.0) / (225.0 * fTemp89 + 27.0)} else {1.0 / F32::max(F32::abs(5.0 * (fTemp60 / fTemp54)), 1e-09)};
															fRec27 = fTemp60 * (2.2 * fTemp61 + 1.0) / fTemp54;
															fTemp91 = fSlow58 * fRec27;
														}
															if iSlow52 != 0 {
																fTemp92 = 1.0 - self.fRec45[1];
																fTemp93 = fTemp35 * fTemp92 + 1.0;
																fTemp94 = 1.0 - self.fRec48[1];
																fTemp95 = fTemp35 * fTemp94 + 1.0;
																fTemp96 = 1.0 - self.fRec47[1];
																fTemp97 = fTemp35 * fTemp96 + 1.0;
																fTemp98 = 1.0 - self.fRec46[1];
																fTemp99 = fTemp35 * fTemp98 + 1.0;
																fTemp100 = fTemp93 * fTemp99 * fTemp97;
																fTemp101 = 1.0 - self.fRec44[1];
																fTemp102 = 1.0 - self.fRec43[1];
																fTemp103 = 1.0 - self.fRec42[1];
																fTemp104 = 1.0 - self.fRec49[1];
																fTemp105 = 1.0 - self.fRec41[1];
																fTemp106 = fTemp95 * (4.2 * (fTemp52 * fTemp105 * fTemp51 * fTemp104 * fTemp103 * fTemp102 * fTemp101 / (fTemp100 * fTemp95)) + 1.0);
																fTemp107 = 1.47 * fTemp51 + 1.0;
																fTemp108 = fTemp35 * fTemp103;
																fTemp109 = fTemp35 * fTemp102;
																fTemp110 = fTemp35 * fTemp101;
																fTemp111 = self.fRec40[1] + fTemp110 * (self.fRec39[1] + fTemp109 * (self.fRec38[1] + fTemp108 * self.fRec37[1] / fTemp93) / fTemp99) / fTemp97 + 0.5 * (fTemp56 * fTemp105 * fTemp107 * fTemp103 * fTemp102 * fTemp101 / fTemp100);
																fTemp112 = 0.5 * fTemp55 * fTemp107 - 4.2 * (fTemp51 * fTemp104 * fTemp111 / fTemp106);
																fTemp113 = self.fRec37[1] + fTemp35 * fTemp105 * fTemp112;
																fTemp114 = fTemp113 / fTemp93;
																self.fRec37[0] = fTemp114 + fTemp35 * (fTemp105 * fTemp112 - fTemp92 * fTemp113 / fTemp93);
																fTemp115 = self.fRec38[1] + fTemp108 * fTemp113 / fTemp93;
																fTemp116 = fTemp115 / fTemp99;
																self.fRec38[0] = fTemp116 + fTemp35 * (fTemp103 * fTemp113 / fTemp93 - fTemp98 * fTemp115 / fTemp99);
																fTemp117 = self.fRec39[1] + fTemp109 * fTemp115 / fTemp99;
																fTemp118 = fTemp117 / fTemp97;
																self.fRec39[0] = fTemp118 + fTemp35 * (fTemp102 * fTemp115 / fTemp99 - fTemp96 * fTemp117 / fTemp97);
																fTemp119 = self.fRec40[1] + fTemp110 * fTemp117 / fTemp97;
																fTemp120 = fTemp119 / fTemp95;
																self.fRec40[0] = fTemp120 + fTemp35 * (fTemp101 * fTemp117 / fTemp97 - fTemp94 * fTemp119 / fTemp95);
																fTemp121 = VoiceDsp_faustpower2_f(fTemp112);
																fTemp122 = 2.56 * fTemp121;
																self.fRec41[0] = 1.0 - if (fTemp122 < 9.0) as i32 != 0 {(fTemp122 + 27.0) / (23.04 * fTemp121 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp112), 1e-09)};
																fTemp123 = VoiceDsp_faustpower2_f(fTemp113) / VoiceDsp_faustpower2_f(fTemp93);
																fTemp124 = 2.56 * fTemp123;
																fTemp125 = 1.0 - if (fTemp124 < 9.0) as i32 != 0 {(fTemp124 + 27.0) / (23.04 * fTemp123 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp114), 1e-09)};
																self.fRec42[0] = fTemp125;
																fTemp126 = VoiceDsp_faustpower2_f(fTemp115) / VoiceDsp_faustpower2_f(fTemp99);
																fTemp127 = 2.56 * fTemp126;
																fTemp128 = 1.0 - if (fTemp127 < 9.0) as i32 != 0 {(fTemp127 + 27.0) / (23.04 * fTemp126 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp116), 1e-09)};
																self.fRec43[0] = fTemp128;
																fTemp129 = VoiceDsp_faustpower2_f(fTemp117) / VoiceDsp_faustpower2_f(fTemp97);
																fTemp130 = 2.56 * fTemp129;
																fTemp131 = 1.0 - if (fTemp130 < 9.0) as i32 != 0 {(fTemp130 + 27.0) / (23.04 * fTemp129 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp118), 1e-09)};
																self.fRec44[0] = fTemp131;
																self.fRec45[0] = fTemp125;
																self.fRec46[0] = fTemp128;
																self.fRec47[0] = fTemp131;
																fTemp132 = VoiceDsp_faustpower2_f(fTemp119) / VoiceDsp_faustpower2_f(fTemp95);
																fTemp133 = 2.56 * fTemp132;
																self.fRec48[0] = 1.0 - if (fTemp133 < 9.0) as i32 != 0 {(fTemp133 + 27.0) / (23.04 * fTemp132 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp120), 1e-09)};
															}
																self.fRec49[0] = 0.0;
																if iSlow52 != 0 {
																	fRec50 = fTemp111 / fTemp106;
																	fTemp134 = fSlow60 * fRec50;
																}
																	self.fVec7[0] = fSlow65;
																	let mut fTemp135: F32 = if ((fSlow65 != self.fVec7[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec51[1] + -1.0};
																	self.fRec51[0] = fTemp135;
																	self.fRec52[0] = if iTemp0 != 0 {fSlow65} else {if (fTemp135 > 0.0) as i32 != 0 {self.fRec52[1] + (fSlow65 - self.fRec52[1]) / F32::max(1.0, fTemp135)} else {fSlow65}};
																	let mut fTemp136: F32 = F32::max(0.0, F32::min(1.0, self.fRec52[0] + fSlow64 * self.fRec28[0]));
																	if iSlow55 != 0 {
																		fTemp137 = F32::max(2e+01, fTemp34);
																		fTemp138 = F32::tan(self.fConst7 * fTemp137);
																		fTemp139 = fTemp138 + 1.0 / F32::max(0.5, fTemp50 / (1.2 * VoiceDsp_faustpower2_f(F32::max(0.0, 1.0 - 0.0013333333 * fTemp137)) + 1.0));
																		fTemp140 = fTemp138 * fTemp139 + 1.0;
																		fTemp141 = F32::tanh(1.7 * self.fRec53[1] + 0.15) - self.fConst9;
																		fTemp142 = fTemp55 - (self.fConst10 * fTemp139 * fTemp141 + self.fRec54[1]);
																		fTemp143 = fTemp138 * fTemp142 / fTemp140;
																		fTemp144 = self.fConst10 * fTemp141;
																		self.fRec53[0] = fTemp144 + 2.0 * fTemp143;
																		fTemp145 = fTemp144 + fTemp143;
																		fTemp146 = fTemp138 * fTemp145;
																		self.fRec54[0] = self.fRec54[1] + 2.0 * fTemp146;
																		fRec55 = self.fRec54[1] + fTemp146;
																		fRec56 = fTemp145;
																		fRec57 = fTemp142 / fTemp140;
																	}
																		self.fVec8[0] = fSlow66;
																		let mut fTemp147: F32 = if ((fSlow66 != self.fVec8[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec58[1] + -1.0};
																		self.fRec58[0] = fTemp147;
																		self.fRec59[0] = if iTemp0 != 0 {fSlow66} else {if (fTemp147 > 0.0) as i32 != 0 {self.fRec59[1] + (fSlow66 - self.fRec59[1]) / F32::max(1.0, fTemp147)} else {fSlow66}};
																		if iSlow55 != 0 {
																			fTemp148 = F32::max(0.0, F32::min(1.0, self.fRec59[0]));
																			iTemp149 = (fTemp148 > 0.5) as i32;
																			fTemp150 = fRec55 + fRec57;
																			fTemp151 = 2.0 * fTemp148;
																			fTemp152 = 1.0 - fTemp151;
																			fTemp153 = fTemp151 + -1.0;
																			fTemp154 = 1.0 - fTemp148;
																			fTemp155 = 1.0 - fTemp136;
																			fTemp156 = fTemp155 * if iTemp149 != 0 {2.0 * fTemp150 * fTemp154 + fRec57 * fTemp153} else {fRec55 * fTemp152 + 2.0 * fTemp150 * fTemp148} + fRec56 * fTemp136;
																		}
																			if iSlow67 & iSlow55 != 0 {
																				fTemp157 = F32::tanh(1.7 * self.fRec60[1] + 0.15) - self.fConst9;
																				fTemp158 = fTemp156 - (self.fConst10 * fTemp139 * fTemp157 + self.fRec61[1]);
																				fTemp159 = fTemp138 * fTemp158 / fTemp140;
																				fTemp160 = self.fConst10 * fTemp157;
																				self.fRec60[0] = fTemp160 + 2.0 * fTemp159;
																				fTemp161 = fTemp160 + fTemp159;
																				fTemp162 = fTemp138 * fTemp161;
																				self.fRec61[0] = self.fRec61[1] + 2.0 * fTemp162;
																				fRec62 = self.fRec61[1] + fTemp162;
																				fRec63 = fTemp161;
																				fRec64 = fTemp158 / fTemp140;
																				fTemp163 = fRec62 + fRec64;
																				fTemp164 = fSlow62 * (fTemp155 * if iTemp149 != 0 {2.0 * fTemp163 * fTemp154 + fRec64 * fTemp153} else {fRec62 * fTemp152 + 2.0 * fTemp163 * fTemp148} + fRec63 * fTemp136);
																			}
																				if iSlow55 != 0 {
																					fTemp165 = fSlow54 * if iSlow63 != 0 {fSlow68 * fTemp164} else {fTemp156};
																				}
																					let mut fTemp166: F32 = fSlow69 * fTemp165 + fSlow61 * fTemp134 + fSlow59 * fTemp91;
																					let mut fTemp167: F32 = (iTemp27) as F32;
																					if (fTemp167 != 0.0) as i32 != 0 {
																						fTemp168 = F32::max(self.fRec13[0], 0.001);
																						fTemp169 = 0.8 * (fTemp167 * F32::tanh(1.25 * fTemp168 * fTemp166) * F32::powf(1e+01, 0.1 * self.fRec13[0]) / fTemp168);
																					}
																						let mut fTemp170: F32 = if iTemp27 != 0 {((iTemp27 != 0) as i32) as u32 as F32 * fTemp169} else {fTemp166};
																						self.fVec9[0] = fTemp170;
																						self.fRec10[0] = -((self.fRec10[1] * (1.0 - fTemp26) - (fTemp170 - self.fVec9[1]) / fTemp25) / (fTemp26 + 1.0));
																						*output0 = fSlow70 * ((*input1 + self.fRec10[0] * self.fRec5[0]) * (fTemp14 * (fTemp14 * (fTemp14 * (fTemp14 * (0.0013333 * fTemp14 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp13))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp13))) as F32);
																						*output1 = fTemp33;
																						*output2 = fTemp50;
																						*output3 = self.fRec59[0];
																						*output4 = fTemp136;
																						*output5 = self.fRec12[0];
																						self.fVec0[1] = self.fVec0[0];
																						self.iVec1[1] = self.iVec1[0];
																						self.fVec2[1] = self.fVec2[0];
																						self.fRec0[1] = self.fRec0[0];
																						self.fRec1[1] = self.fRec1[0];
																						if iTemp4 != 0 {
																							self.iRec3[1] = self.iRec3[0];
																							self.fRec2[1] = self.fRec2[0];
																							self.fRec4[1] = self.fRec4[0];
																						}
																							self.fRec6[1] = self.fRec6[0];
																							self.fRec7[1] = self.fRec7[0];
																							self.fRec8[1] = self.fRec8[0];
																							self.fRec9[1] = self.fRec9[0];
																							self.fRec5[1] = self.fRec5[0];
																							self.fVec3[1] = self.fVec3[0];
																							self.fRec11[1] = self.fRec11[0];
																							self.fRec12[1] = self.fRec12[0];
																							self.fRec13[1] = self.fRec13[0];
																							self.fRec28[1] = self.fRec28[0];
																							if iTemp4 != 0 {
																								self.fRec31[1] = self.fRec31[0];
																								self.iRec32[1] = self.iRec32[0];
																								self.fRec30[1] = self.fRec30[0];
																								self.fVec4[1] = self.fVec4[0];
																								self.fRec29[1] = self.fRec29[0];
																							}
																								self.fVec5[1] = self.fVec5[0];
																								self.fRec33[1] = self.fRec33[0];
																								self.fRec34[1] = self.fRec34[0];
																								self.fVec6[1] = self.fVec6[0];
																								self.fRec35[1] = self.fRec35[0];
																								self.fRec36[1] = self.fRec36[0];
																								if iSlow37 != 0 {
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
																									self.fRec24[1] = self.fRec24[0];
																									self.fRec25[1] = self.fRec25[0];
																									self.fRec26[1] = self.fRec26[0];
																								}
																									if iSlow52 != 0 {
																										self.fRec37[1] = self.fRec37[0];
																										self.fRec38[1] = self.fRec38[0];
																										self.fRec39[1] = self.fRec39[0];
																										self.fRec40[1] = self.fRec40[0];
																										self.fRec41[1] = self.fRec41[0];
																										self.fRec42[1] = self.fRec42[0];
																										self.fRec43[1] = self.fRec43[0];
																										self.fRec44[1] = self.fRec44[0];
																										self.fRec45[1] = self.fRec45[0];
																										self.fRec46[1] = self.fRec46[0];
																										self.fRec47[1] = self.fRec47[0];
																										self.fRec48[1] = self.fRec48[0];
																									}
																										self.fRec49[1] = self.fRec49[0];
																										self.fVec7[1] = self.fVec7[0];
																										self.fRec51[1] = self.fRec51[0];
																										self.fRec52[1] = self.fRec52[0];
																										if iSlow55 != 0 {
																											self.fRec53[1] = self.fRec53[0];
																											self.fRec54[1] = self.fRec54[0];
																										}
																											self.fVec8[1] = self.fVec8[0];
																											self.fRec58[1] = self.fRec58[0];
																											self.fRec59[1] = self.fRec59[0];
																											if iSlow67 & iSlow55 != 0 {
																												self.fRec60[1] = self.fRec60[0];
																												self.fRec61[1] = self.fRec61[0];
																											}
																												self.fVec9[1] = self.fVec9[0];
																												self.fRec10[1] = self.fRec10[0];
																											}
																											self.iTempPerm0 = iTemp5;
																											self.iTempPerm1 = iTemp6;
																											self.fTempPerm2 = fTemp9;
																											self.fTempPerm3 = fTemp28;
																											self.fTempPerm4 = fTemp29;
																											self.fTempPerm5 = fTemp30;
																											self.fTempPerm6 = fTemp31;
																											self.fTempPerm7 = fTemp34;
																											self.fTempPerm8 = fTemp35;
																											self.fTempPerm9 = fTemp36;
																											self.fTempPerm10 = fTemp37;
																											self.fTempPerm11 = fTemp38;
																											self.fTempPerm12 = fTemp39;
																											self.fTempPerm13 = fTemp40;
																											self.fTempPerm14 = fTemp41;
																											self.fTempPerm15 = fTemp42;
																											self.fTempPerm16 = fTemp43;
																											self.fTempPerm17 = fTemp44;
																											self.fTempPerm18 = fTemp45;
																											self.fTempPerm19 = fTemp46;
																											self.fTempPerm20 = fTemp47;
																											self.fTempPerm21 = fTemp48;
																											self.fTempPerm22 = fTemp51;
																											self.fTempPerm23 = fTemp52;
																											self.fTempPerm24 = fTemp53;
																											self.fTempPerm25 = fTemp54;
																											self.fTempPerm26 = fTemp55;
																											self.fTempPerm27 = fTemp56;
																											self.fTempPerm28 = fTemp57;
																											self.fTempPerm29 = fTemp58;
																											self.fTempPerm30 = fTemp59;
																											self.fTempPerm31 = fTemp60;
																											self.fTempPerm32 = fTemp61;
																											self.fTempPerm33 = fTemp62;
																											self.fTempPerm34 = fTemp63;
																											self.fTempPerm35 = fTemp64;
																											self.fTempPerm36 = fTemp65;
																											self.fTempPerm37 = fTemp66;
																											self.fTempPerm38 = fTemp67;
																											self.fTempPerm39 = fTemp68;
																											self.fTempPerm40 = fTemp69;
																											self.fTempPerm41 = fTemp70;
																											self.fTempPerm42 = fTemp71;
																											self.fTempPerm43 = fTemp72;
																											self.fTempPerm44 = fTemp73;
																											self.fTempPerm45 = fTemp74;
																											self.fTempPerm46 = fTemp75;
																											self.fTempPerm47 = fTemp76;
																											self.fTempPerm48 = fTemp77;
																											self.fTempPerm49 = fTemp78;
																											self.fTempPerm50 = fTemp79;
																											self.fTempPerm51 = fTemp80;
																											self.fTempPerm52 = fTemp81;
																											self.fTempPerm53 = fTemp82;
																											self.fTempPerm54 = fTemp83;
																											self.fTempPerm55 = fTemp84;
																											self.fTempPerm56 = fTemp85;
																											self.fTempPerm57 = fTemp86;
																											self.fTempPerm58 = fTemp87;
																											self.fTempPerm59 = fTemp88;
																											self.fTempPerm60 = fTemp89;
																											self.fTempPerm61 = fTemp90;
																											self.fTempPerm62 = fTemp91;
																											self.fTempPerm63 = fTemp92;
																											self.fTempPerm64 = fTemp93;
																											self.fTempPerm65 = fTemp94;
																											self.fTempPerm66 = fTemp95;
																											self.fTempPerm67 = fTemp96;
																											self.fTempPerm68 = fTemp97;
																											self.fTempPerm69 = fTemp98;
																											self.fTempPerm70 = fTemp99;
																											self.fTempPerm71 = fTemp100;
																											self.fTempPerm72 = fTemp101;
																											self.fTempPerm73 = fTemp102;
																											self.fTempPerm74 = fTemp103;
																											self.fTempPerm75 = fTemp104;
																											self.fTempPerm76 = fTemp105;
																											self.fTempPerm77 = fTemp106;
																											self.fTempPerm78 = fTemp107;
																											self.fTempPerm79 = fTemp108;
																											self.fTempPerm80 = fTemp109;
																											self.fTempPerm81 = fTemp110;
																											self.fTempPerm82 = fTemp111;
																											self.fTempPerm83 = fTemp112;
																											self.fTempPerm84 = fTemp113;
																											self.fTempPerm85 = fTemp114;
																											self.fTempPerm86 = fTemp115;
																											self.fTempPerm87 = fTemp116;
																											self.fTempPerm88 = fTemp117;
																											self.fTempPerm89 = fTemp118;
																											self.fTempPerm90 = fTemp119;
																											self.fTempPerm91 = fTemp120;
																											self.fTempPerm92 = fTemp121;
																											self.fTempPerm93 = fTemp122;
																											self.fTempPerm94 = fTemp123;
																											self.fTempPerm95 = fTemp124;
																											self.fTempPerm96 = fTemp125;
																											self.fTempPerm97 = fTemp126;
																											self.fTempPerm98 = fTemp127;
																											self.fTempPerm99 = fTemp128;
																											self.fTempPerm100 = fTemp129;
																											self.fTempPerm101 = fTemp130;
																											self.fTempPerm102 = fTemp131;
																											self.fTempPerm103 = fTemp132;
																											self.fTempPerm104 = fTemp133;
																											self.fTempPerm105 = fTemp134;
																											self.fTempPerm106 = fTemp137;
																											self.fTempPerm107 = fTemp138;
																											self.fTempPerm108 = fTemp139;
																											self.fTempPerm109 = fTemp140;
																											self.fTempPerm110 = fTemp141;
																											self.fTempPerm111 = fTemp142;
																											self.fTempPerm112 = fTemp143;
																											self.fTempPerm113 = fTemp144;
																											self.fTempPerm114 = fTemp145;
																											self.fTempPerm115 = fTemp146;
																											self.fTempPerm116 = fTemp148;
																											self.iTempPerm117 = iTemp149;
																											self.fTempPerm118 = fTemp150;
																											self.fTempPerm119 = fTemp151;
																											self.fTempPerm120 = fTemp152;
																											self.fTempPerm121 = fTemp153;
																											self.fTempPerm122 = fTemp154;
																											self.fTempPerm123 = fTemp155;
																											self.fTempPerm124 = fTemp156;
																											self.fTempPerm125 = fTemp157;
																											self.fTempPerm126 = fTemp158;
																											self.fTempPerm127 = fTemp159;
																											self.fTempPerm128 = fTemp160;
																											self.fTempPerm129 = fTemp161;
																											self.fTempPerm130 = fTemp162;
																											self.fTempPerm131 = fTemp163;
																											self.fTempPerm132 = fTemp164;
																											self.fTempPerm133 = fTemp165;
																											self.fTempPerm134 = fTemp168;
																											self.fTempPerm135 = fTemp169;
																										}

}

