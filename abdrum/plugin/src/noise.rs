/* ------------------------------------------------------------
name: "ABDrumNoise"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn NoiseDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn NoiseDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct NoiseDsp {
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fTempPerm0: F32,
	fTempPerm1: F32,
	fTempPerm2: F32,
	fTempPerm3: F32,
	fTempPerm4: F32,
	iVec0: [i32;2],
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
	fTempPerm21: F32,
	fTempPerm22: F32,
	fTempPerm23: F32,
	fTempPerm24: F32,
	fTempPerm25: F32,
	fConst3: F32,
	fConst4: F32,
	iTempPerm26: i32,
	iRec14: [i32;2],
	fRec13: [F32;2],
	fTempPerm27: F32,
	fTempPerm28: F32,
	fTempPerm29: F32,
	fTempPerm30: F32,
	iRec16: [i32;2],
	fTempPerm31: F32,
	fRec17: [F32;4],
	fConst5: F32,
	iTempPerm32: i32,
	iTempPerm33: i32,
	iRec18: [i32;2],
	fConst6: F32,
	fTempPerm34: F32,
	iTempPerm35: i32,
	fConst7: F32,
	fTempPerm36: F32,
	fRec22: [F32;2],
	fRec21: [F32;2],
	fTempPerm37: F32,
	fTempPerm38: F32,
	fTempPerm39: F32,
	fTempPerm40: F32,
	fConst8: F32,
	fTempPerm41: F32,
	fRec23: [F32;2],
	fTempPerm42: F32,
	fConst9: F32,
	fTempPerm43: F32,
	fRec24: [F32;2],
	fTempPerm44: F32,
	fConst10: F32,
	fTempPerm45: F32,
	fRec25: [F32;2],
	fTempPerm46: F32,
	fConst11: F32,
	fTempPerm47: F32,
	fRec26: [F32;2],
	fTempPerm48: F32,
	fConst12: F32,
	fTempPerm49: F32,
	fRec27: [F32;2],
	fTempPerm50: F32,
	fTempPerm51: F32,
	fRec28: [F32;2],
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
	fTempPerm82: F32,
	fConst13: F32,
	fTempPerm83: F32,
	fTempPerm84: F32,
	fRec31: [F32;2],
	fTempPerm85: F32,
	iTempPerm86: i32,
	iTempPerm87: i32,
	iTempPerm88: i32,
	iRec32: [i32;2],
	fRec30: [F32;2],
	iTempPerm89: i32,
	fConst14: F32,
	fTempPerm90: F32,
	fRec38: [F32;2],
	fRec37: [F32;2],
	fTempPerm91: F32,
	fRec36: [F32;2],
	fTempPerm92: F32,
	fTempPerm93: F32,
	fVec1: [F32;2],
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fRec29: [F32;2],
	fTempPerm94: F32,
	fConst18: F32,
	fConst20: F32,
	fTempPerm95: F32,
	fVec2: [F32;2],
	fConst21: F32,
	fRec41: [F32;2],
	fRec40: [F32;2],
	fRec39: [F32;2],
	fTempPerm96: F32,
	fTempPerm97: F32,
	fTempPerm98: F32,
	fTempPerm99: F32,
	fTempPerm100: F32,
	fTempPerm101: F32,
	fRec43: [F32;2],
	fRec42: [F32;2],
	fTempPerm102: F32,
	fTempPerm103: F32,
	fTempPerm104: F32,
	fTempPerm105: F32,
	fTempPerm106: F32,
	fTempPerm107: F32,
	fRec10: [F32;2],
	fTempPerm108: F32,
	fTempPerm109: F32,
	fRec11: [F32;2],
	fTempPerm110: F32,
	fTempPerm111: F32,
	fTempPerm112: F32,
	fTempPerm113: F32,
	fTempPerm114: F32,
	fRec7: [F32;2],
	fTempPerm115: F32,
	fTempPerm116: F32,
	fRec8: [F32;2],
	fTempPerm117: F32,
	fTempPerm118: F32,
	fTempPerm119: F32,
	fTempPerm120: F32,
	fTempPerm121: F32,
	fRec4: [F32;2],
	fTempPerm122: F32,
	fTempPerm123: F32,
	fRec5: [F32;2],
	fTempPerm124: F32,
	fTempPerm125: F32,
	fTempPerm126: F32,
	fTempPerm127: F32,
	fTempPerm128: F32,
	fRec1: [F32;2],
	fTempPerm129: F32,
	fTempPerm130: F32,
	fRec2: [F32;2],
	fTempPerm131: F32,
	fTempPerm132: F32,
	fRec0: [F32;3],
	fTempPerm133: F32,
	fTempPerm134: F32,
	fTempPerm135: F32,
	fTempPerm136: F32,
	fTempPerm137: F32,
	fTempPerm138: F32,
	fTempPerm139: F32,
	fTempPerm140: F32,
	fTempPerm141: F32,
	fRec54: [F32;2],
	fTempPerm142: F32,
	fTempPerm143: F32,
	fRec55: [F32;2],
	fTempPerm144: F32,
	fTempPerm145: F32,
	fTempPerm146: F32,
	fTempPerm147: F32,
	fRec51: [F32;2],
	fTempPerm148: F32,
	fTempPerm149: F32,
	fRec52: [F32;2],
	fTempPerm150: F32,
	fTempPerm151: F32,
	fTempPerm152: F32,
	fTempPerm153: F32,
	fRec48: [F32;2],
	fTempPerm154: F32,
	fTempPerm155: F32,
	fRec49: [F32;2],
	fTempPerm156: F32,
	fTempPerm157: F32,
	fTempPerm158: F32,
	fTempPerm159: F32,
	fRec45: [F32;2],
	fTempPerm160: F32,
	fTempPerm161: F32,
	fRec46: [F32;2],
	fRec44: [F32;3],
	fTempPerm162: F32,
}

impl FaustDsp for NoiseDsp {
	type T = F32;
		
	fn new() -> NoiseDsp { 
		NoiseDsp {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fTempPerm0: 0.0,
			fTempPerm1: 0.0,
			fTempPerm2: 0.0,
			fTempPerm3: 0.0,
			fTempPerm4: 0.0,
			iVec0: [0;2],
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
			fTempPerm21: 0.0,
			fTempPerm22: 0.0,
			fTempPerm23: 0.0,
			fTempPerm24: 0.0,
			fTempPerm25: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			iTempPerm26: 0,
			iRec14: [0;2],
			fRec13: [0.0;2],
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fTempPerm29: 0.0,
			fTempPerm30: 0.0,
			iRec16: [0;2],
			fTempPerm31: 0.0,
			fRec17: [0.0;4],
			fConst5: 0.0,
			iTempPerm32: 0,
			iTempPerm33: 0,
			iRec18: [0;2],
			fConst6: 0.0,
			fTempPerm34: 0.0,
			iTempPerm35: 0,
			fConst7: 0.0,
			fTempPerm36: 0.0,
			fRec22: [0.0;2],
			fRec21: [0.0;2],
			fTempPerm37: 0.0,
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fTempPerm40: 0.0,
			fConst8: 0.0,
			fTempPerm41: 0.0,
			fRec23: [0.0;2],
			fTempPerm42: 0.0,
			fConst9: 0.0,
			fTempPerm43: 0.0,
			fRec24: [0.0;2],
			fTempPerm44: 0.0,
			fConst10: 0.0,
			fTempPerm45: 0.0,
			fRec25: [0.0;2],
			fTempPerm46: 0.0,
			fConst11: 0.0,
			fTempPerm47: 0.0,
			fRec26: [0.0;2],
			fTempPerm48: 0.0,
			fConst12: 0.0,
			fTempPerm49: 0.0,
			fRec27: [0.0;2],
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			fRec28: [0.0;2],
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
			fTempPerm82: 0.0,
			fConst13: 0.0,
			fTempPerm83: 0.0,
			fTempPerm84: 0.0,
			fRec31: [0.0;2],
			fTempPerm85: 0.0,
			iTempPerm86: 0,
			iTempPerm87: 0,
			iTempPerm88: 0,
			iRec32: [0;2],
			fRec30: [0.0;2],
			iTempPerm89: 0,
			fConst14: 0.0,
			fTempPerm90: 0.0,
			fRec38: [0.0;2],
			fRec37: [0.0;2],
			fTempPerm91: 0.0,
			fRec36: [0.0;2],
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fVec1: [0.0;2],
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fRec29: [0.0;2],
			fTempPerm94: 0.0,
			fConst18: 0.0,
			fConst20: 0.0,
			fTempPerm95: 0.0,
			fVec2: [0.0;2],
			fConst21: 0.0,
			fRec41: [0.0;2],
			fRec40: [0.0;2],
			fRec39: [0.0;2],
			fTempPerm96: 0.0,
			fTempPerm97: 0.0,
			fTempPerm98: 0.0,
			fTempPerm99: 0.0,
			fTempPerm100: 0.0,
			fTempPerm101: 0.0,
			fRec43: [0.0;2],
			fRec42: [0.0;2],
			fTempPerm102: 0.0,
			fTempPerm103: 0.0,
			fTempPerm104: 0.0,
			fTempPerm105: 0.0,
			fTempPerm106: 0.0,
			fTempPerm107: 0.0,
			fRec10: [0.0;2],
			fTempPerm108: 0.0,
			fTempPerm109: 0.0,
			fRec11: [0.0;2],
			fTempPerm110: 0.0,
			fTempPerm111: 0.0,
			fTempPerm112: 0.0,
			fTempPerm113: 0.0,
			fTempPerm114: 0.0,
			fRec7: [0.0;2],
			fTempPerm115: 0.0,
			fTempPerm116: 0.0,
			fRec8: [0.0;2],
			fTempPerm117: 0.0,
			fTempPerm118: 0.0,
			fTempPerm119: 0.0,
			fTempPerm120: 0.0,
			fTempPerm121: 0.0,
			fRec4: [0.0;2],
			fTempPerm122: 0.0,
			fTempPerm123: 0.0,
			fRec5: [0.0;2],
			fTempPerm124: 0.0,
			fTempPerm125: 0.0,
			fTempPerm126: 0.0,
			fTempPerm127: 0.0,
			fTempPerm128: 0.0,
			fRec1: [0.0;2],
			fTempPerm129: 0.0,
			fTempPerm130: 0.0,
			fRec2: [0.0;2],
			fTempPerm131: 0.0,
			fTempPerm132: 0.0,
			fRec0: [0.0;3],
			fTempPerm133: 0.0,
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			fTempPerm136: 0.0,
			fTempPerm137: 0.0,
			fTempPerm138: 0.0,
			fTempPerm139: 0.0,
			fTempPerm140: 0.0,
			fTempPerm141: 0.0,
			fRec54: [0.0;2],
			fTempPerm142: 0.0,
			fTempPerm143: 0.0,
			fRec55: [0.0;2],
			fTempPerm144: 0.0,
			fTempPerm145: 0.0,
			fTempPerm146: 0.0,
			fTempPerm147: 0.0,
			fRec51: [0.0;2],
			fTempPerm148: 0.0,
			fTempPerm149: 0.0,
			fRec52: [0.0;2],
			fTempPerm150: 0.0,
			fTempPerm151: 0.0,
			fTempPerm152: 0.0,
			fTempPerm153: 0.0,
			fRec48: [0.0;2],
			fTempPerm154: 0.0,
			fTempPerm155: 0.0,
			fRec49: [0.0;2],
			fTempPerm156: 0.0,
			fTempPerm157: 0.0,
			fTempPerm158: 0.0,
			fTempPerm159: 0.0,
			fRec45: [0.0;2],
			fTempPerm160: 0.0,
			fTempPerm161: 0.0,
			fRec46: [0.0;2],
			fRec44: [0.0;3],
			fTempPerm162: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn NoiseDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("drum.lib/name", r"abdrum-layers");
		m.declare("drum.lib/version", r"0.1");
		m.declare("filename", r"plugin-noise.dsp");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
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
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"ABDrumNoise");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.0");
		m.declare("oscillators.lib/lf_sawpos:author", r"Bart Brouns, revised by Stéphane Letz");
		m.declare("oscillators.lib/lf_sawpos:licence", r"STK-4.3");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/version", r"1.5.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 14;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
	}
	fn instance_clear(&mut self) {
		self.fTempPerm0 = 0.0;
		self.fTempPerm1 = 0.0;
		self.fTempPerm2 = 0.0;
		self.fTempPerm3 = 0.0;
		self.fTempPerm4 = 0.0;
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
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
		self.fTempPerm21 = 0.0;
		self.fTempPerm22 = 0.0;
		self.fTempPerm23 = 0.0;
		self.fTempPerm24 = 0.0;
		self.fTempPerm25 = 0.0;
		self.iTempPerm26 = 0;
		for l1 in 0..2 {
			self.iRec14[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec13[l2 as usize] = 0.0;
		}
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		self.fTempPerm29 = 0.0;
		self.fTempPerm30 = 0.0;
		for l3 in 0..2 {
			self.iRec16[l3 as usize] = 0;
		}
		self.fTempPerm31 = 0.0;
		for l4 in 0..4 {
			self.fRec17[l4 as usize] = 0.0;
		}
		self.iTempPerm32 = 0;
		self.iTempPerm33 = 0;
		for l5 in 0..2 {
			self.iRec18[l5 as usize] = 0;
		}
		self.fTempPerm34 = 0.0;
		self.iTempPerm35 = 0;
		self.fTempPerm36 = 0.0;
		for l6 in 0..2 {
			self.fRec22[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec21[l7 as usize] = 0.0;
		}
		self.fTempPerm37 = 0.0;
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		for l8 in 0..2 {
			self.fRec23[l8 as usize] = 0.0;
		}
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		for l9 in 0..2 {
			self.fRec24[l9 as usize] = 0.0;
		}
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		for l10 in 0..2 {
			self.fRec25[l10 as usize] = 0.0;
		}
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		for l11 in 0..2 {
			self.fRec26[l11 as usize] = 0.0;
		}
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		for l12 in 0..2 {
			self.fRec27[l12 as usize] = 0.0;
		}
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		for l13 in 0..2 {
			self.fRec28[l13 as usize] = 0.0;
		}
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
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.fTempPerm84 = 0.0;
		for l14 in 0..2 {
			self.fRec31[l14 as usize] = 0.0;
		}
		self.fTempPerm85 = 0.0;
		self.iTempPerm86 = 0;
		self.iTempPerm87 = 0;
		self.iTempPerm88 = 0;
		for l15 in 0..2 {
			self.iRec32[l15 as usize] = 0;
		}
		for l16 in 0..2 {
			self.fRec30[l16 as usize] = 0.0;
		}
		self.iTempPerm89 = 0;
		self.fTempPerm90 = 0.0;
		for l17 in 0..2 {
			self.fRec38[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec37[l18 as usize] = 0.0;
		}
		self.fTempPerm91 = 0.0;
		for l19 in 0..2 {
			self.fRec36[l19 as usize] = 0.0;
		}
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		for l20 in 0..2 {
			self.fVec1[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec29[l21 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		for l22 in 0..2 {
			self.fVec2[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec41[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec40[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec39[l25 as usize] = 0.0;
		}
		self.fTempPerm96 = 0.0;
		self.fTempPerm97 = 0.0;
		self.fTempPerm98 = 0.0;
		self.fTempPerm99 = 0.0;
		self.fTempPerm100 = 0.0;
		self.fTempPerm101 = 0.0;
		for l26 in 0..2 {
			self.fRec43[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec42[l27 as usize] = 0.0;
		}
		self.fTempPerm102 = 0.0;
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		self.fTempPerm105 = 0.0;
		self.fTempPerm106 = 0.0;
		self.fTempPerm107 = 0.0;
		for l28 in 0..2 {
			self.fRec10[l28 as usize] = 0.0;
		}
		self.fTempPerm108 = 0.0;
		self.fTempPerm109 = 0.0;
		for l29 in 0..2 {
			self.fRec11[l29 as usize] = 0.0;
		}
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		self.fTempPerm112 = 0.0;
		self.fTempPerm113 = 0.0;
		self.fTempPerm114 = 0.0;
		for l30 in 0..2 {
			self.fRec7[l30 as usize] = 0.0;
		}
		self.fTempPerm115 = 0.0;
		self.fTempPerm116 = 0.0;
		for l31 in 0..2 {
			self.fRec8[l31 as usize] = 0.0;
		}
		self.fTempPerm117 = 0.0;
		self.fTempPerm118 = 0.0;
		self.fTempPerm119 = 0.0;
		self.fTempPerm120 = 0.0;
		self.fTempPerm121 = 0.0;
		for l32 in 0..2 {
			self.fRec4[l32 as usize] = 0.0;
		}
		self.fTempPerm122 = 0.0;
		self.fTempPerm123 = 0.0;
		for l33 in 0..2 {
			self.fRec5[l33 as usize] = 0.0;
		}
		self.fTempPerm124 = 0.0;
		self.fTempPerm125 = 0.0;
		self.fTempPerm126 = 0.0;
		self.fTempPerm127 = 0.0;
		self.fTempPerm128 = 0.0;
		for l34 in 0..2 {
			self.fRec1[l34 as usize] = 0.0;
		}
		self.fTempPerm129 = 0.0;
		self.fTempPerm130 = 0.0;
		for l35 in 0..2 {
			self.fRec2[l35 as usize] = 0.0;
		}
		self.fTempPerm131 = 0.0;
		self.fTempPerm132 = 0.0;
		for l36 in 0..3 {
			self.fRec0[l36 as usize] = 0.0;
		}
		self.fTempPerm133 = 0.0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		self.fTempPerm136 = 0.0;
		self.fTempPerm137 = 0.0;
		self.fTempPerm138 = 0.0;
		self.fTempPerm139 = 0.0;
		self.fTempPerm140 = 0.0;
		self.fTempPerm141 = 0.0;
		for l37 in 0..2 {
			self.fRec54[l37 as usize] = 0.0;
		}
		self.fTempPerm142 = 0.0;
		self.fTempPerm143 = 0.0;
		for l38 in 0..2 {
			self.fRec55[l38 as usize] = 0.0;
		}
		self.fTempPerm144 = 0.0;
		self.fTempPerm145 = 0.0;
		self.fTempPerm146 = 0.0;
		self.fTempPerm147 = 0.0;
		for l39 in 0..2 {
			self.fRec51[l39 as usize] = 0.0;
		}
		self.fTempPerm148 = 0.0;
		self.fTempPerm149 = 0.0;
		for l40 in 0..2 {
			self.fRec52[l40 as usize] = 0.0;
		}
		self.fTempPerm150 = 0.0;
		self.fTempPerm151 = 0.0;
		self.fTempPerm152 = 0.0;
		self.fTempPerm153 = 0.0;
		for l41 in 0..2 {
			self.fRec48[l41 as usize] = 0.0;
		}
		self.fTempPerm154 = 0.0;
		self.fTempPerm155 = 0.0;
		for l42 in 0..2 {
			self.fRec49[l42 as usize] = 0.0;
		}
		self.fTempPerm156 = 0.0;
		self.fTempPerm157 = 0.0;
		self.fTempPerm158 = 0.0;
		self.fTempPerm159 = 0.0;
		for l43 in 0..2 {
			self.fRec45[l43 as usize] = 0.0;
		}
		self.fTempPerm160 = 0.0;
		self.fTempPerm161 = 0.0;
		for l44 in 0..2 {
			self.fRec46[l44 as usize] = 0.0;
		}
		for l45 in 0..3 {
			self.fRec44[l45 as usize] = 0.0;
		}
		self.fTempPerm162 = 0.0;
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 0.45 * self.fConst0;
		self.fConst2 = 3.1415927 / self.fConst0;
		self.fConst3 = F32::exp(-(1.25e+03 / self.fConst0));
		self.fConst4 = 3e+01 / self.fConst0;
		self.fConst5 = 3e+03 / self.fConst0;
		self.fConst6 = 0.9999 * self.fConst0;
		self.fConst7 = 1.0 / self.fConst0;
		self.fConst8 = 3.897 / self.fConst0;
		self.fConst9 = 2.63 / self.fConst0;
		self.fConst10 = 2.546 / self.fConst0;
		self.fConst11 = 1.8 / self.fConst0;
		self.fConst12 = 1.483 / self.fConst0;
		self.fConst13 = 0.1 * self.fConst0;
		self.fConst14 = 12.0 / self.fConst0;
		self.fConst15 = 1.0 / F32::tan(62.831852 / self.fConst0);
		self.fConst16 = 1.0 - self.fConst15;
		self.fConst17 = 1.0 / (self.fConst15 + 1.0);
		self.fConst18 = 1.0 - F32::exp(-(9090.909 / self.fConst0));
		let mut fConst19: F32 = 1.0 / F32::tan(6283.1855 / self.fConst0);
		self.fConst20 = 1.0 - fConst19;
		self.fConst21 = 1.0 / (fConst19 + 1.0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		NoiseDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("ABDrumNoise");
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10, inputs11, inputs12, inputs13) = if let [inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10, inputs11, inputs12, inputs13, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			let inputs3 = inputs3[..count as usize].iter();
			let inputs4 = inputs4[..count as usize].iter();
			let inputs5 = inputs5[..count as usize].iter();
			let inputs6 = inputs6[..count as usize].iter();
			let inputs7 = inputs7[..count as usize].iter();
			let inputs8 = inputs8[..count as usize].iter();
			let inputs9 = inputs9[..count as usize].iter();
			let inputs10 = inputs10[..count as usize].iter();
			let inputs11 = inputs11[..count as usize].iter();
			let inputs12 = inputs12[..count as usize].iter();
			let inputs13 = inputs13[..count as usize].iter();
			(inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10, inputs11, inputs12, inputs13)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fTemp2: F32 = self.fTempPerm0;
		let mut fTemp4: F32 = self.fTempPerm1;
		let mut fTemp5: F32 = self.fTempPerm2;
		let mut fTemp6: F32 = self.fTempPerm3;
		let mut fTemp7: F32 = self.fTempPerm4;
		let mut fTemp8: F32 = self.fTempPerm5;
		let mut fTemp9: F32 = self.fTempPerm6;
		let mut fTemp10: F32 = self.fTempPerm7;
		let mut fTemp11: F32 = self.fTempPerm8;
		let mut fTemp12: F32 = self.fTempPerm9;
		let mut fTemp13: F32 = self.fTempPerm10;
		let mut fTemp14: F32 = self.fTempPerm11;
		let mut fTemp15: F32 = self.fTempPerm12;
		let mut fTemp16: F32 = self.fTempPerm13;
		let mut fTemp17: F32 = self.fTempPerm14;
		let mut fTemp18: F32 = self.fTempPerm15;
		let mut fTemp19: F32 = self.fTempPerm16;
		let mut fTemp20: F32 = self.fTempPerm17;
		let mut fTemp21: F32 = self.fTempPerm18;
		let mut fTemp22: F32 = self.fTempPerm19;
		let mut fTemp23: F32 = self.fTempPerm20;
		let mut fTemp24: F32 = self.fTempPerm21;
		let mut fTemp25: F32 = self.fTempPerm22;
		let mut fTemp26: F32 = self.fTempPerm23;
		let mut fTemp27: F32 = self.fTempPerm24;
		let mut fTemp28: F32 = self.fTempPerm25;
		let mut iTemp29: i32 = self.iTempPerm26;
		let mut fTemp30: F32 = self.fTempPerm27;
		let mut fTemp31: F32 = self.fTempPerm28;
		let mut fTemp32: F32 = self.fTempPerm29;
		let mut fTemp33: F32 = self.fTempPerm30;
		let mut fTemp34: F32 = self.fTempPerm31;
		let mut iTemp35: i32 = self.iTempPerm32;
		let mut iTemp36: i32 = self.iTempPerm33;
		let mut fTemp37: F32 = self.fTempPerm34;
		let mut iTemp38: i32 = self.iTempPerm35;
		let mut fTemp39: F32 = self.fTempPerm36;
		let mut fTemp40: F32 = self.fTempPerm37;
		let mut fTemp42: F32 = self.fTempPerm38;
		let mut fTemp41: F32 = self.fTempPerm39;
		let mut fTemp43: F32 = self.fTempPerm40;
		let mut fTemp44: F32 = self.fTempPerm41;
		let mut fTemp45: F32 = self.fTempPerm42;
		let mut fTemp46: F32 = self.fTempPerm43;
		let mut fTemp47: F32 = self.fTempPerm44;
		let mut fTemp48: F32 = self.fTempPerm45;
		let mut fTemp49: F32 = self.fTempPerm46;
		let mut fTemp50: F32 = self.fTempPerm47;
		let mut fTemp51: F32 = self.fTempPerm48;
		let mut fTemp52: F32 = self.fTempPerm49;
		let mut fTemp53: F32 = self.fTempPerm50;
		let mut fTemp54: F32 = self.fTempPerm51;
		let mut fTemp55: F32 = self.fTempPerm52;
		let mut fTemp56: F32 = self.fTempPerm53;
		let mut fTemp57: F32 = self.fTempPerm54;
		let mut fTemp58: F32 = self.fTempPerm55;
		let mut fTemp59: F32 = self.fTempPerm56;
		let mut fTemp60: F32 = self.fTempPerm57;
		let mut fTemp61: F32 = self.fTempPerm58;
		let mut fTemp62: F32 = self.fTempPerm59;
		let mut fTemp63: F32 = self.fTempPerm60;
		let mut fTemp64: F32 = self.fTempPerm61;
		let mut fTemp65: F32 = self.fTempPerm62;
		let mut fTemp66: F32 = self.fTempPerm63;
		let mut fTemp67: F32 = self.fTempPerm64;
		let mut fTemp68: F32 = self.fTempPerm65;
		let mut fTemp69: F32 = self.fTempPerm66;
		let mut fTemp70: F32 = self.fTempPerm67;
		let mut fTemp71: F32 = self.fTempPerm68;
		let mut fTemp72: F32 = self.fTempPerm69;
		let mut fTemp73: F32 = self.fTempPerm70;
		let mut fTemp74: F32 = self.fTempPerm71;
		let mut fTemp75: F32 = self.fTempPerm72;
		let mut fTemp76: F32 = self.fTempPerm73;
		let mut fTemp77: F32 = self.fTempPerm74;
		let mut fTemp78: F32 = self.fTempPerm75;
		let mut fTemp79: F32 = self.fTempPerm76;
		let mut fTemp80: F32 = self.fTempPerm77;
		let mut fTemp81: F32 = self.fTempPerm78;
		let mut fTemp82: F32 = self.fTempPerm79;
		let mut fTemp83: F32 = self.fTempPerm80;
		let mut fTemp84: F32 = self.fTempPerm81;
		let mut fTemp85: F32 = self.fTempPerm82;
		let mut fTemp86: F32 = self.fTempPerm83;
		let mut fTemp87: F32 = self.fTempPerm84;
		let mut fTemp88: F32 = self.fTempPerm85;
		let mut iTemp89: i32 = self.iTempPerm86;
		let mut iTemp90: i32 = self.iTempPerm87;
		let mut iTemp91: i32 = self.iTempPerm88;
		let mut iTemp92: i32 = self.iTempPerm89;
		let mut fTemp93: F32 = self.fTempPerm90;
		let mut fTemp94: F32 = self.fTempPerm91;
		let mut fTemp95: F32 = self.fTempPerm92;
		let mut fTemp96: F32 = self.fTempPerm93;
		let mut fTemp97: F32 = self.fTempPerm94;
		let mut fTemp98: F32 = self.fTempPerm95;
		let mut fTemp99: F32 = self.fTempPerm96;
		let mut fTemp100: F32 = self.fTempPerm97;
		let mut fTemp101: F32 = self.fTempPerm98;
		let mut fTemp102: F32 = self.fTempPerm99;
		let mut fTemp103: F32 = self.fTempPerm100;
		let mut fTemp104: F32 = self.fTempPerm101;
		let mut fTemp105: F32 = self.fTempPerm102;
		let mut fTemp106: F32 = self.fTempPerm103;
		let mut fTemp107: F32 = self.fTempPerm104;
		let mut fTemp108: F32 = self.fTempPerm105;
		let mut fTemp109: F32 = self.fTempPerm106;
		let mut fTemp110: F32 = self.fTempPerm107;
		let mut fTemp111: F32 = self.fTempPerm108;
		let mut fTemp112: F32 = self.fTempPerm109;
		let mut fTemp113: F32 = self.fTempPerm110;
		let mut fTemp114: F32 = self.fTempPerm111;
		let mut fTemp115: F32 = self.fTempPerm112;
		let mut fTemp116: F32 = self.fTempPerm113;
		let mut fTemp117: F32 = self.fTempPerm114;
		let mut fTemp118: F32 = self.fTempPerm115;
		let mut fTemp119: F32 = self.fTempPerm116;
		let mut fTemp120: F32 = self.fTempPerm117;
		let mut fTemp121: F32 = self.fTempPerm118;
		let mut fTemp122: F32 = self.fTempPerm119;
		let mut fTemp123: F32 = self.fTempPerm120;
		let mut fTemp124: F32 = self.fTempPerm121;
		let mut fTemp125: F32 = self.fTempPerm122;
		let mut fTemp126: F32 = self.fTempPerm123;
		let mut fTemp127: F32 = self.fTempPerm124;
		let mut fTemp128: F32 = self.fTempPerm125;
		let mut fTemp129: F32 = self.fTempPerm126;
		let mut fTemp130: F32 = self.fTempPerm127;
		let mut fTemp131: F32 = self.fTempPerm128;
		let mut fTemp132: F32 = self.fTempPerm129;
		let mut fTemp133: F32 = self.fTempPerm130;
		let mut fTemp134: F32 = self.fTempPerm131;
		let mut fTemp135: F32 = self.fTempPerm132;
		let mut fTemp136: F32 = self.fTempPerm133;
		let mut fTemp137: F32 = self.fTempPerm134;
		let mut fTemp138: F32 = self.fTempPerm135;
		let mut fTemp139: F32 = self.fTempPerm136;
		let mut fTemp140: F32 = self.fTempPerm137;
		let mut fTemp141: F32 = self.fTempPerm138;
		let mut fTemp142: F32 = self.fTempPerm139;
		let mut fTemp143: F32 = self.fTempPerm140;
		let mut fTemp144: F32 = self.fTempPerm141;
		let mut fTemp145: F32 = self.fTempPerm142;
		let mut fTemp146: F32 = self.fTempPerm143;
		let mut fTemp147: F32 = self.fTempPerm144;
		let mut fTemp148: F32 = self.fTempPerm145;
		let mut fTemp149: F32 = self.fTempPerm146;
		let mut fTemp150: F32 = self.fTempPerm147;
		let mut fTemp151: F32 = self.fTempPerm148;
		let mut fTemp152: F32 = self.fTempPerm149;
		let mut fTemp153: F32 = self.fTempPerm150;
		let mut fTemp154: F32 = self.fTempPerm151;
		let mut fTemp155: F32 = self.fTempPerm152;
		let mut fTemp156: F32 = self.fTempPerm153;
		let mut fTemp157: F32 = self.fTempPerm154;
		let mut fTemp158: F32 = self.fTempPerm155;
		let mut fTemp159: F32 = self.fTempPerm156;
		let mut fTemp160: F32 = self.fTempPerm157;
		let mut fTemp161: F32 = self.fTempPerm158;
		let mut fTemp162: F32 = self.fTempPerm159;
		let mut fTemp163: F32 = self.fTempPerm160;
		let mut fTemp164: F32 = self.fTempPerm161;
		let mut fTemp165: F32 = self.fTempPerm162;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(inputs4).zip(inputs5).zip(inputs6).zip(inputs7).zip(inputs8).zip(inputs9).zip(inputs10).zip(inputs11).zip(inputs12).zip(inputs13).zip(outputs0);
		for ((((((((((((((input0, input1), input2), input3), input4), input5), input6), input7), input8), input9), input10), input11), input12), input13), output0) in zipped_iterators {
			let mut iRec15: i32 = 0;
			let mut iRec19: i32 = 0;
			let mut iRec20: i32 = 0;
			let mut iRec33: i32 = 0;
			let mut iRec34: i32 = 0;
			let mut iRec35: i32 = 0;
			let mut fRec12: F32 = 0.0;
			let mut fRec9: F32 = 0.0;
			let mut fRec6: F32 = 0.0;
			let mut fRec3: F32 = 0.0;
			let mut fRec56: F32 = 0.0;
			let mut fRec53: F32 = 0.0;
			let mut fRec50: F32 = 0.0;
			let mut fRec47: F32 = 0.0;
			let mut fTemp0: F32 = *input10;
			let mut fTemp1: F32 = ((fTemp0 > 0.0001) as i32) as u32 as F32;
			let mut fTemp3: F32 = ((fTemp0 <= 0.0001) as i32) as u32 as F32;
			if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
				fTemp2 = F32::tan(self.fConst2 * F32::max(2e+01, F32::min(self.fConst1, *input9)));
				fTemp4 = 1.0 / fTemp2;
				fTemp5 = (fTemp4 + 1.4142135) / fTemp2 + 1.0;
				fTemp6 = NoiseDsp_faustpower2_f(fTemp2);
				fTemp7 = fTemp6 * fTemp5;
				self.iVec0[0] = 1;
				fTemp8 = 1.0 - 1.0 / fTemp6;
				fTemp9 = (fTemp4 + -1.4142135) / fTemp2 + 1.0;
				fTemp10 = *input8;
				fTemp11 = 4.0 * fTemp10;
				fTemp12 = F32::max(0.0, F32::min(1.0, fTemp11 + -3.0));
				fTemp13 = *input2;
				fTemp14 = F32::tan(self.fConst2 * F32::max(2e+01, F32::min(self.fConst1, F32::max(2e+01, F32::min(self.fConst1, fTemp13)))));
				fTemp15 = fTemp14 + 1.1111112;
				fTemp16 = fTemp14 * fTemp15 + 1.0;
			}
				if (fTemp1 != 0.0) as i32 != 0 {
					fTemp17 = F32::tanh(0.2 * self.fRec1[1]);
				}
					if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
						fTemp18 = F32::max(0.0, F32::min(1.0, fTemp11 + -2.0));
					}
						if (fTemp1 != 0.0) as i32 != 0 {
							fTemp19 = F32::tanh(0.2 * self.fRec4[1]);
						}
							if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
								fTemp20 = F32::max(0.0, F32::min(1.0, fTemp11 + -1.0));
							}
								if (fTemp1 != 0.0) as i32 != 0 {
									fTemp21 = F32::tanh(0.2 * self.fRec7[1]);
								}
									if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
										fTemp22 = F32::max(0.0, F32::min(1.0, fTemp11));
									}
										if (fTemp1 != 0.0) as i32 != 0 {
											fTemp23 = F32::tanh(0.2 * self.fRec10[1]);
											fTemp24 = F32::max(0.0, F32::min(1.0, fTemp0));
											fTemp25 = F32::max(0.0, *input11);
											fTemp26 = 39.0 * NoiseDsp_faustpower2_f(fTemp24) + 1.0;
										}
											if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
												fTemp27 = F32::max(0.0, F32::min(1.0, *input7));
												fTemp28 = F32::max(0.0, F32::min(1.0, 6.6666665 * (fTemp27 + -0.85)));
												iTemp29 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec14[1], 12345));
												self.iRec14[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp29, 12345));
												iRec15 = iTemp29;
												self.fRec13[0] = F32::max(self.fConst3 * self.fRec13[1], (((0.5 * (4.656613e-10 * (iRec15) as F32 + 1.0)) < (self.fConst4 * F32::powf(666.0, fTemp27))) as i32) as u32 as F32);
												fTemp30 = fTemp28 + self.fRec13[0] * (1.0 - fTemp28);
												fTemp31 = *input6;
												fTemp32 = 1.5707964 * *input5;
												fTemp33 = *input0;
												self.iRec16[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec16[1]), 12345);
												fTemp34 = 4.656613e-10 * (self.iRec16[0]) as F32;
												self.fRec17[0] = 0.5221894 * self.fRec17[3] + fTemp34 + 2.494956 * self.fRec17[1] - 2.0172658 * self.fRec17[2];
												iTemp35 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec18[1], 12345));
												iTemp36 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp35, 12345));
												self.iRec18[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp36, 12345));
												iRec19 = iTemp36;
												iRec20 = iTemp35;
												fTemp37 = 2.0 * fTemp13;
												iTemp38 = i32::wrapping_sub(1, self.iVec0[1]);
												fTemp39 = if iTemp38 != 0 {0.0} else {self.fRec22[1] + self.fConst7 * F32::max(fTemp37, 0.0)};
												self.fRec22[0] = fTemp39 - F32::floor(fTemp39);
												self.fRec21[0] = if ((self.fRec22[0] < self.fRec22[1]) as i32) | ((fTemp37 >= self.fConst6) as i32) != 0 {fTemp34} else {self.fRec21[1]};
												fTemp40 = ((fTemp33 == 4.0) as i32) as u32 as F32;
												fTemp42 = ((fTemp33 == 5.0) as i32) as u32 as F32;
											}
												if (((fTemp1 != 0.0) as i32) & ((fTemp42 != 0.0) as i32)) | ((((fTemp1 != 0.0) as i32) & ((fTemp40 != 0.0) as i32)) | ((((fTemp40 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)))) != 0 {
													fTemp41 = *input1;
												}
													if (((fTemp1 != 0.0) as i32) & ((fTemp40 != 0.0) as i32)) | (((fTemp40 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) != 0 {
														fTemp43 = F32::max(self.fConst7 * F32::min(3.897 * fTemp41, self.fConst1), 1e-06);
														fTemp44 = if iTemp38 != 0 {0.0} else {self.fRec23[1] + self.fConst8 * fTemp41};
														self.fRec23[0] = fTemp44 - F32::floor(fTemp44);
														fTemp45 = F32::max(self.fConst7 * F32::min(2.63 * fTemp41, self.fConst1), 1e-06);
														fTemp46 = if iTemp38 != 0 {0.0} else {self.fRec24[1] + self.fConst9 * fTemp41};
														self.fRec24[0] = fTemp46 - F32::floor(fTemp46);
														fTemp47 = F32::max(self.fConst7 * F32::min(2.546 * fTemp41, self.fConst1), 1e-06);
														fTemp48 = if iTemp38 != 0 {0.0} else {self.fRec25[1] + self.fConst10 * fTemp41};
														self.fRec25[0] = fTemp48 - F32::floor(fTemp48);
														fTemp49 = F32::max(self.fConst7 * F32::min(1.8 * fTemp41, self.fConst1), 1e-06);
														fTemp50 = if iTemp38 != 0 {0.0} else {self.fRec26[1] + self.fConst11 * fTemp41};
														self.fRec26[0] = fTemp50 - F32::floor(fTemp50);
														fTemp51 = F32::max(self.fConst7 * F32::min(1.483 * fTemp41, self.fConst1), 1e-06);
														fTemp52 = if iTemp38 != 0 {0.0} else {self.fRec27[1] + self.fConst12 * fTemp41};
														self.fRec27[0] = fTemp52 - F32::floor(fTemp52);
														fTemp53 = F32::max(self.fConst7 * F32::min(fTemp41, self.fConst1), 1e-06);
														fTemp54 = if iTemp38 != 0 {0.0} else {self.fRec28[1] + self.fConst7 * fTemp41};
														self.fRec28[0] = fTemp54 - F32::floor(fTemp54);
														fTemp55 = self.fRec28[0] + -1.0;
														fTemp56 = 1.0 - fTemp53;
														fTemp57 = self.fRec27[0] + -1.0;
														fTemp58 = 1.0 - fTemp51;
														fTemp59 = self.fRec26[0] + -1.0;
														fTemp60 = 1.0 - fTemp49;
														fTemp61 = self.fRec25[0] + -1.0;
														fTemp62 = 1.0 - fTemp47;
														fTemp63 = self.fRec24[0] + -1.0;
														fTemp64 = 1.0 - fTemp45;
														fTemp65 = self.fRec23[0] + -1.0;
														fTemp66 = 1.0 - fTemp43;
														fTemp67 = F32::floor(self.fRec28[0] + 0.5);
														fTemp68 = self.fRec28[0] + (0.5 - fTemp67);
														fTemp69 = self.fRec28[0] + (-0.5 - fTemp67);
														fTemp70 = F32::floor(self.fRec27[0] + 0.5);
														fTemp71 = self.fRec27[0] + (0.5 - fTemp70);
														fTemp72 = self.fRec27[0] + (-0.5 - fTemp70);
														fTemp73 = F32::floor(self.fRec26[0] + 0.5);
														fTemp74 = self.fRec26[0] + (0.5 - fTemp73);
														fTemp75 = self.fRec26[0] + (-0.5 - fTemp73);
														fTemp76 = F32::floor(self.fRec25[0] + 0.5);
														fTemp77 = self.fRec25[0] + (0.5 - fTemp76);
														fTemp78 = self.fRec25[0] + (-0.5 - fTemp76);
														fTemp79 = F32::floor(self.fRec24[0] + 0.5);
														fTemp80 = self.fRec24[0] + (0.5 - fTemp79);
														fTemp81 = self.fRec24[0] + (-0.5 - fTemp79);
														fTemp82 = F32::floor(self.fRec23[0] + 0.5);
														fTemp83 = self.fRec23[0] + (0.5 - fTemp82);
														fTemp84 = self.fRec23[0] + (-0.5 - fTemp82);
														fTemp85 = 0.23333333 * fTemp40 * (((fTemp83 > fTemp66) as i32) as u32 as F32 * (fTemp84 * (fTemp84 / fTemp43 + 2.0) / fTemp43 + 1.0) + ((fTemp83 < fTemp43) as i32) as u32 as F32 * (fTemp83 * (2.0 - fTemp83 / fTemp43) / fTemp43 + -1.0) + ((fTemp80 > fTemp64) as i32) as u32 as F32 * (fTemp81 * (fTemp81 / fTemp45 + 2.0) / fTemp45 + 1.0) + ((fTemp80 < fTemp45) as i32) as u32 as F32 * (fTemp80 * (2.0 - fTemp80 / fTemp45) / fTemp45 + -1.0) + ((fTemp77 > fTemp62) as i32) as u32 as F32 * (fTemp78 * (fTemp78 / fTemp47 + 2.0) / fTemp47 + 1.0) + ((fTemp77 < fTemp47) as i32) as u32 as F32 * (fTemp77 * (2.0 - fTemp77 / fTemp47) / fTemp47 + -1.0) + ((fTemp74 > fTemp60) as i32) as u32 as F32 * (fTemp75 * (fTemp75 / fTemp49 + 2.0) / fTemp49 + 1.0) + ((fTemp74 < fTemp49) as i32) as u32 as F32 * (fTemp74 * (2.0 - fTemp74 / fTemp49) / fTemp49 + -1.0) + ((fTemp71 > fTemp58) as i32) as u32 as F32 * (fTemp72 * (fTemp72 / fTemp51 + 2.0) / fTemp51 + 1.0) + ((fTemp71 < fTemp51) as i32) as u32 as F32 * (fTemp71 * (2.0 - fTemp71 / fTemp51) / fTemp51 + -1.0) + ((fTemp68 > fTemp56) as i32) as u32 as F32 * (fTemp69 * (fTemp69 / fTemp53 + 2.0) / fTemp53 + 1.0) + ((fTemp68 < fTemp53) as i32) as u32 as F32 * (fTemp68 * (2.0 - fTemp68 / fTemp53) / fTemp53 + -1.0) + (if (self.fRec28[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec27[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec26[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec25[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec23[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec24[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 - (((self.fRec23[0] > fTemp66) as i32) as u32 as F32 * (fTemp65 * (fTemp65 / fTemp43 + 2.0) / fTemp43 + 1.0) + ((self.fRec24[0] > fTemp64) as i32) as u32 as F32 * (fTemp63 * (fTemp63 / fTemp45 + 2.0) / fTemp45 + 1.0) + ((self.fRec25[0] > fTemp62) as i32) as u32 as F32 * (fTemp61 * (fTemp61 / fTemp47 + 2.0) / fTemp47 + 1.0) + ((self.fRec26[0] > fTemp60) as i32) as u32 as F32 * (fTemp59 * (fTemp59 / fTemp49 + 2.0) / fTemp49 + 1.0) + ((self.fRec27[0] > fTemp58) as i32) as u32 as F32 * (fTemp57 * (fTemp57 / fTemp51 + 2.0) / fTemp51 + 1.0) + ((self.fRec28[0] < fTemp53) as i32) as u32 as F32 * (self.fRec28[0] * (2.0 - self.fRec28[0] / fTemp53) / fTemp53 + -1.0) + ((self.fRec28[0] > fTemp56) as i32) as u32 as F32 * (fTemp55 * (fTemp55 / fTemp53 + 2.0) / fTemp53 + 1.0) + ((self.fRec27[0] < fTemp51) as i32) as u32 as F32 * (self.fRec27[0] * (2.0 - self.fRec27[0] / fTemp51) / fTemp51 + -1.0) + ((self.fRec26[0] < fTemp49) as i32) as u32 as F32 * (self.fRec26[0] * (2.0 - self.fRec26[0] / fTemp49) / fTemp49 + -1.0) + ((self.fRec25[0] < fTemp47) as i32) as u32 as F32 * (self.fRec25[0] * (2.0 - self.fRec25[0] / fTemp47) / fTemp47 + -1.0) + ((self.fRec24[0] < fTemp45) as i32) as u32 as F32 * (self.fRec24[0] * (2.0 - self.fRec24[0] / fTemp45) / fTemp45 + -1.0) + ((self.fRec23[0] < fTemp43) as i32) as u32 as F32 * (self.fRec23[0] * (2.0 - self.fRec23[0] / fTemp43) / fTemp43 + -1.0)));
													}
														if (((fTemp1 != 0.0) as i32) & ((fTemp42 != 0.0) as i32)) | (((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) != 0 {
															fTemp86 = F32::max(1.0, F32::min(self.fConst13, fTemp41));
															fTemp87 = if iTemp38 != 0 {0.0} else {self.fRec31[1] + self.fConst7 * fTemp86};
															self.fRec31[0] = fTemp87 - F32::floor(fTemp87);
														}
															if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
																fTemp88 = ((fTemp33 == 6.0) as i32) as u32 as F32;
															}
																if (((fTemp1 != 0.0) as i32) & ((fTemp42 != 0.0) as i32)) | ((((fTemp88 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | ((((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp88 != 0.0) as i32)))) != 0 {
																	iTemp89 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec32[1], 12345));
																	iTemp90 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp89, 12345));
																	iTemp91 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp90, 12345));
																	self.iRec32[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp91, 12345));
																	iRec33 = iTemp91;
																	iRec34 = iTemp90;
																	iRec35 = iTemp89;
																}
																	if (((fTemp1 != 0.0) as i32) & ((fTemp42 != 0.0) as i32)) | (((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) != 0 {
																		self.fRec30[0] = if (self.fRec31[0] < self.fRec31[1]) as i32 != 0 {4.656613e-10 * (self.iRec32[0]) as F32} else {self.fRec30[1]};
																		iTemp92 = ((self.fRec31[0] < 0.3) as i32) * ((self.fRec30[0] > -0.6) as i32);
																		fTemp93 = if iTemp38 != 0 {0.0} else {self.fRec38[1] + self.fConst14 * fTemp86};
																		self.fRec38[0] = fTemp93 - F32::floor(fTemp93);
																		self.fRec37[0] = if (self.fRec38[0] < self.fRec38[1]) as i32 != 0 {4.656613e-10 * (iRec33) as F32} else {self.fRec37[1]};
																		fTemp94 = if iTemp38 != 0 {0.0} else {self.fRec36[1] + self.fConst7 * F32::max(2e+01, F32::min(self.fConst1, fTemp13 * F32::exp(0.1732868 * (F32::floor(F32::max(0.0, F32::min(3.999, 2.0 * (self.fRec37[0] + 1.0)))) + -1.5)) * (0.3 * NoiseDsp_faustpower2_f(F32::max(0.0, F32::min(1.0, 1.0 - 3.3333333 * self.fRec31[0]))) + 1.0)))};
																		self.fRec36[0] = fTemp94 - F32::floor(fTemp94);
																		fTemp95 = F32::sin(6.2831855 * self.fRec36[0]);
																		fTemp96 = F32::tanh(3.0 * (fTemp95 + 0.8 * F32::abs(fTemp95))) * (iTemp92) as F32 + 5.5879353e-11 * (iRec34) as F32 * (i32::wrapping_sub(1, iTemp92)) as F32;
																		self.fVec1[0] = fTemp96;
																		self.fRec29[0] = -(self.fConst17 * (self.fConst16 * self.fRec29[1] - self.fConst15 * (fTemp96 - self.fVec1[1])));
																		fTemp97 = 1.7 * self.fRec29[0] * fTemp42;
																	}
																		if (((fTemp88 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp88 != 0.0) as i32)) != 0 {
																			fTemp98 = (iRec35) as F32;
																			self.fVec2[0] = fTemp98;
																			self.fRec41[0] = self.fConst21 * (4.656613e-10 * (fTemp98 + self.fVec2[1]) - self.fConst20 * self.fRec41[1]);
																			self.fRec40[0] = ((self.fRec41[0] >= 0.0) as i32) as u32 as F32 * (self.fRec40[1] + self.fConst18 * (1.0 - self.fRec40[1]));
																			self.fRec39[0] = -(self.fConst17 * (self.fConst16 * self.fRec39[1] - self.fConst15 * (self.fRec40[0] - self.fRec40[1])));
																			fTemp99 = 1.67 * self.fRec39[0] * fTemp88;
																		}
																			if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
																				fTemp100 = F32::cos(fTemp32) * if (fTemp33 >= 4.0) as i32 != 0 {if (fTemp33 >= 6.0) as i32 != 0 {fTemp99} else {if (fTemp33 >= 5.0) as i32 != 0 {fTemp97} else {fTemp85}}} else {if (fTemp33 >= 2.0) as i32 != 0 {if (fTemp33 >= 3.0) as i32 != 0 {0.00048828125 * F32::floor(2048.0 * self.fRec21[0] + 0.5)} else {F32::tanh(1.7229468e-09 * (iRec20) as F32 * (((0.5 * (4.656613e-10 * (iRec19) as F32 + 1.0)) < self.fConst5) as i32) as u32 as F32)}} else {if (fTemp33 >= 1.0) as i32 != 0 {11.6 * (0.049922034 * self.fRec17[0] + 0.0506127 * self.fRec17[2] - (0.095993534 * self.fRec17[1] + 0.004408786 * self.fRec17[3]))} else {fTemp34}}} + 4.0 * sl_loop((*input3) as i32, *input4) * F32::sin(fTemp32);
																				fTemp101 = F32::max(0.0, F32::min(1.0, fTemp31));
																				fTemp102 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp101)) + -1.0));
																				fTemp103 = self.fConst0 / (23.0 * NoiseDsp_faustpower2_f(fTemp101) + 1.0);
																				fTemp104 = if iTemp38 != 0 {0.0} else {self.fRec43[1] + self.fConst7 * F32::max(fTemp103, 0.0)};
																				self.fRec43[0] = fTemp104 - F32::floor(fTemp104);
																				self.fRec42[0] = if ((self.fRec43[0] < self.fRec43[1]) as i32) | ((fTemp103 >= self.fConst6) as i32) != 0 {fTemp100} else {self.fRec42[1]};
																				fTemp105 = if (fTemp31 > 0.0001) as i32 != 0 {F32::floor(self.fRec42[0] * fTemp102 + 0.5) / fTemp102} else {fTemp100};
																			}
																				if (fTemp1 != 0.0) as i32 != 0 {
																					fTemp106 = fTemp25 * fTemp105 * fTemp30 * fTemp26;
																					fTemp107 = (F32::min(fTemp106, 0.0) + (1.0 - F32::exp(-(F32::max(fTemp106, 0.0))))) * (0.75 * fTemp24 * F32::min(1.0, fTemp25) + 1.0) / fTemp26 + 0.2 * fTemp25 * fTemp24;
																					fTemp108 = fTemp107 - (5.0 * fTemp15 * fTemp23 + self.fRec11[1]);
																					fTemp109 = fTemp14 * fTemp108 / fTemp16;
																					fTemp110 = 5.0 * fTemp23;
																					self.fRec10[0] = fTemp110 + 2.0 * fTemp109;
																					fTemp111 = fTemp110 + fTemp109;
																					fTemp112 = fTemp14 * fTemp111;
																					self.fRec11[0] = self.fRec11[1] + 2.0 * fTemp112;
																					fRec12 = fTemp111;
																				}
																					if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
																						fTemp113 = 1.0 - fTemp22;
																					}
																						if (fTemp1 != 0.0) as i32 != 0 {
																							fTemp114 = fTemp113 * fTemp107 + 1.1111112 * fRec12 * fTemp22;
																							fTemp115 = fTemp114 - (5.0 * fTemp15 * fTemp21 + self.fRec8[1]);
																							fTemp116 = fTemp14 * fTemp115 / fTemp16;
																							fTemp117 = 5.0 * fTemp21;
																							self.fRec7[0] = fTemp117 + 2.0 * fTemp116;
																							fTemp118 = fTemp117 + fTemp116;
																							fTemp119 = fTemp14 * fTemp118;
																							self.fRec8[0] = self.fRec8[1] + 2.0 * fTemp119;
																							fRec9 = fTemp118;
																						}
																							if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
																								fTemp120 = 1.0 - fTemp20;
																							}
																								if (fTemp1 != 0.0) as i32 != 0 {
																									fTemp121 = fTemp120 * fTemp114 + 1.1111112 * fRec9 * fTemp20;
																									fTemp122 = fTemp121 - (5.0 * fTemp15 * fTemp19 + self.fRec5[1]);
																									fTemp123 = fTemp14 * fTemp122 / fTemp16;
																									fTemp124 = 5.0 * fTemp19;
																									self.fRec4[0] = fTemp124 + 2.0 * fTemp123;
																									fTemp125 = fTemp124 + fTemp123;
																									fTemp126 = fTemp14 * fTemp125;
																									self.fRec5[0] = self.fRec5[1] + 2.0 * fTemp126;
																									fRec6 = fTemp125;
																								}
																									if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
																										fTemp127 = 1.0 - fTemp18;
																									}
																										if (fTemp1 != 0.0) as i32 != 0 {
																											fTemp128 = fTemp127 * fTemp121 + 1.1111112 * fRec6 * fTemp18;
																											fTemp129 = fTemp128 - (5.0 * fTemp15 * fTemp17 + self.fRec2[1]);
																											fTemp130 = fTemp14 * fTemp129 / fTemp16;
																											fTemp131 = 5.0 * fTemp17;
																											self.fRec1[0] = fTemp131 + 2.0 * fTemp130;
																											fTemp132 = fTemp131 + fTemp130;
																											fTemp133 = fTemp14 * fTemp132;
																											self.fRec2[0] = self.fRec2[1] + 2.0 * fTemp133;
																											fRec3 = fTemp132;
																										}
																											if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
																												fTemp134 = 1.0 - fTemp12;
																												fTemp135 = F32::powf(1e+01, 0.05 * (12.0 * F32::powf(F32::max(0.0, F32::min(1.0, fTemp10)), 0.35) + -4.4));
																											}
																												if (fTemp1 != 0.0) as i32 != 0 {
																													self.fRec0[0] = fTemp135 * (fTemp134 * fTemp128 + 1.1111112 * fRec3 * fTemp12) - (fTemp9 * self.fRec0[2] + 2.0 * fTemp8 * self.fRec0[1]) / fTemp5;
																													fTemp136 = *input12 * fTemp1 * (self.fRec0[0] + self.fRec0[2] - 2.0 * self.fRec0[1]) / fTemp7;
																												}
																													if (fTemp3 != 0.0) as i32 != 0 {
																														fTemp137 = F32::tanh(0.2 * self.fRec45[1]);
																														fTemp138 = F32::tanh(0.2 * self.fRec48[1]);
																														fTemp139 = F32::tanh(0.2 * self.fRec51[1]);
																														fTemp140 = F32::tanh(0.2 * self.fRec54[1]);
																														fTemp141 = fTemp105 * fTemp30;
																														fTemp142 = fTemp141 - (5.0 * fTemp15 * fTemp140 + self.fRec55[1]);
																														fTemp143 = fTemp14 * fTemp142 / fTemp16;
																														fTemp144 = 5.0 * fTemp140;
																														self.fRec54[0] = fTemp144 + 2.0 * fTemp143;
																														fTemp145 = fTemp144 + fTemp143;
																														fTemp146 = fTemp14 * fTemp145;
																														self.fRec55[0] = self.fRec55[1] + 2.0 * fTemp146;
																														fRec56 = fTemp145;
																														fTemp147 = fTemp141 * fTemp113 + 1.1111112 * fRec56 * fTemp22;
																														fTemp148 = fTemp147 - (5.0 * fTemp15 * fTemp139 + self.fRec52[1]);
																														fTemp149 = fTemp14 * fTemp148 / fTemp16;
																														fTemp150 = 5.0 * fTemp139;
																														self.fRec51[0] = fTemp150 + 2.0 * fTemp149;
																														fTemp151 = fTemp150 + fTemp149;
																														fTemp152 = fTemp14 * fTemp151;
																														self.fRec52[0] = self.fRec52[1] + 2.0 * fTemp152;
																														fRec53 = fTemp151;
																														fTemp153 = fTemp147 * fTemp120 + 1.1111112 * fRec53 * fTemp20;
																														fTemp154 = fTemp153 - (5.0 * fTemp15 * fTemp138 + self.fRec49[1]);
																														fTemp155 = fTemp14 * fTemp154 / fTemp16;
																														fTemp156 = 5.0 * fTemp138;
																														self.fRec48[0] = fTemp156 + 2.0 * fTemp155;
																														fTemp157 = fTemp156 + fTemp155;
																														fTemp158 = fTemp14 * fTemp157;
																														self.fRec49[0] = self.fRec49[1] + 2.0 * fTemp158;
																														fRec50 = fTemp157;
																														fTemp159 = fTemp153 * fTemp127 + 1.1111112 * fRec50 * fTemp18;
																														fTemp160 = fTemp159 - (5.0 * fTemp15 * fTemp137 + self.fRec46[1]);
																														fTemp161 = fTemp14 * fTemp160 / fTemp16;
																														fTemp162 = 5.0 * fTemp137;
																														self.fRec45[0] = fTemp162 + 2.0 * fTemp161;
																														fTemp163 = fTemp162 + fTemp161;
																														fTemp164 = fTemp14 * fTemp163;
																														self.fRec46[0] = self.fRec46[1] + 2.0 * fTemp164;
																														fRec47 = fTemp163;
																														self.fRec44[0] = (fTemp159 * fTemp134 + 1.1111112 * fRec47 * fTemp12) * fTemp135 - (self.fRec44[2] * fTemp9 + 2.0 * self.fRec44[1] * fTemp8) / fTemp5;
																														fTemp165 = *input13 * fTemp3 * (self.fRec44[0] + self.fRec44[2] - 2.0 * self.fRec44[1]) / fTemp7;
																													}
																														*output0 = fTemp165 + fTemp136;
																														if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
																															self.iVec0[1] = self.iVec0[0];
																															self.iRec14[1] = self.iRec14[0];
																															self.fRec13[1] = self.fRec13[0];
																															self.iRec16[1] = self.iRec16[0];
																															for j0 in (1..=3).rev() {
																																self.fRec17[j0 as usize] = self.fRec17[(i32::wrapping_sub(j0, 1)) as usize];
																															}
																															self.iRec18[1] = self.iRec18[0];
																															self.fRec22[1] = self.fRec22[0];
																															self.fRec21[1] = self.fRec21[0];
																														}
																															if (((fTemp1 != 0.0) as i32) & ((fTemp40 != 0.0) as i32)) | (((fTemp40 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) != 0 {
																																self.fRec23[1] = self.fRec23[0];
																																self.fRec24[1] = self.fRec24[0];
																																self.fRec25[1] = self.fRec25[0];
																																self.fRec26[1] = self.fRec26[0];
																																self.fRec27[1] = self.fRec27[0];
																																self.fRec28[1] = self.fRec28[0];
																															}
																																if (((fTemp1 != 0.0) as i32) & ((fTemp42 != 0.0) as i32)) | (((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) != 0 {
																																	self.fRec31[1] = self.fRec31[0];
																																}
																																	if (((fTemp1 != 0.0) as i32) & ((fTemp42 != 0.0) as i32)) | ((((fTemp88 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | ((((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp88 != 0.0) as i32)))) != 0 {
																																		self.iRec32[1] = self.iRec32[0];
																																	}
																																		if (((fTemp1 != 0.0) as i32) & ((fTemp42 != 0.0) as i32)) | (((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) != 0 {
																																			self.fRec30[1] = self.fRec30[0];
																																			self.fRec38[1] = self.fRec38[0];
																																			self.fRec37[1] = self.fRec37[0];
																																			self.fRec36[1] = self.fRec36[0];
																																			self.fVec1[1] = self.fVec1[0];
																																			self.fRec29[1] = self.fRec29[0];
																																		}
																																			if (((fTemp88 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp88 != 0.0) as i32)) != 0 {
																																				self.fVec2[1] = self.fVec2[0];
																																				self.fRec41[1] = self.fRec41[0];
																																				self.fRec40[1] = self.fRec40[0];
																																				self.fRec39[1] = self.fRec39[0];
																																			}
																																				if ((fTemp1 != 0.0) as i32) | ((fTemp3 != 0.0) as i32) != 0 {
																																					self.fRec43[1] = self.fRec43[0];
																																					self.fRec42[1] = self.fRec42[0];
																																				}
																																					if (fTemp1 != 0.0) as i32 != 0 {
																																						self.fRec10[1] = self.fRec10[0];
																																						self.fRec11[1] = self.fRec11[0];
																																						self.fRec7[1] = self.fRec7[0];
																																						self.fRec8[1] = self.fRec8[0];
																																						self.fRec4[1] = self.fRec4[0];
																																						self.fRec5[1] = self.fRec5[0];
																																						self.fRec1[1] = self.fRec1[0];
																																						self.fRec2[1] = self.fRec2[0];
																																						self.fRec0[2] = self.fRec0[1];
																																						self.fRec0[1] = self.fRec0[0];
																																					}
																																						if (fTemp3 != 0.0) as i32 != 0 {
																																							self.fRec54[1] = self.fRec54[0];
																																							self.fRec55[1] = self.fRec55[0];
																																							self.fRec51[1] = self.fRec51[0];
																																							self.fRec52[1] = self.fRec52[0];
																																							self.fRec48[1] = self.fRec48[0];
																																							self.fRec49[1] = self.fRec49[0];
																																							self.fRec45[1] = self.fRec45[0];
																																							self.fRec46[1] = self.fRec46[0];
																																							self.fRec44[2] = self.fRec44[1];
																																							self.fRec44[1] = self.fRec44[0];
																																						}
																																						}
																																						self.fTempPerm0 = fTemp2;
																																						self.fTempPerm1 = fTemp4;
																																						self.fTempPerm2 = fTemp5;
																																						self.fTempPerm3 = fTemp6;
																																						self.fTempPerm4 = fTemp7;
																																						self.fTempPerm5 = fTemp8;
																																						self.fTempPerm6 = fTemp9;
																																						self.fTempPerm7 = fTemp10;
																																						self.fTempPerm8 = fTemp11;
																																						self.fTempPerm9 = fTemp12;
																																						self.fTempPerm10 = fTemp13;
																																						self.fTempPerm11 = fTemp14;
																																						self.fTempPerm12 = fTemp15;
																																						self.fTempPerm13 = fTemp16;
																																						self.fTempPerm14 = fTemp17;
																																						self.fTempPerm15 = fTemp18;
																																						self.fTempPerm16 = fTemp19;
																																						self.fTempPerm17 = fTemp20;
																																						self.fTempPerm18 = fTemp21;
																																						self.fTempPerm19 = fTemp22;
																																						self.fTempPerm20 = fTemp23;
																																						self.fTempPerm21 = fTemp24;
																																						self.fTempPerm22 = fTemp25;
																																						self.fTempPerm23 = fTemp26;
																																						self.fTempPerm24 = fTemp27;
																																						self.fTempPerm25 = fTemp28;
																																						self.iTempPerm26 = iTemp29;
																																						self.fTempPerm27 = fTemp30;
																																						self.fTempPerm28 = fTemp31;
																																						self.fTempPerm29 = fTemp32;
																																						self.fTempPerm30 = fTemp33;
																																						self.fTempPerm31 = fTemp34;
																																						self.iTempPerm32 = iTemp35;
																																						self.iTempPerm33 = iTemp36;
																																						self.fTempPerm34 = fTemp37;
																																						self.iTempPerm35 = iTemp38;
																																						self.fTempPerm36 = fTemp39;
																																						self.fTempPerm37 = fTemp40;
																																						self.fTempPerm38 = fTemp42;
																																						self.fTempPerm39 = fTemp41;
																																						self.fTempPerm40 = fTemp43;
																																						self.fTempPerm41 = fTemp44;
																																						self.fTempPerm42 = fTemp45;
																																						self.fTempPerm43 = fTemp46;
																																						self.fTempPerm44 = fTemp47;
																																						self.fTempPerm45 = fTemp48;
																																						self.fTempPerm46 = fTemp49;
																																						self.fTempPerm47 = fTemp50;
																																						self.fTempPerm48 = fTemp51;
																																						self.fTempPerm49 = fTemp52;
																																						self.fTempPerm50 = fTemp53;
																																						self.fTempPerm51 = fTemp54;
																																						self.fTempPerm52 = fTemp55;
																																						self.fTempPerm53 = fTemp56;
																																						self.fTempPerm54 = fTemp57;
																																						self.fTempPerm55 = fTemp58;
																																						self.fTempPerm56 = fTemp59;
																																						self.fTempPerm57 = fTemp60;
																																						self.fTempPerm58 = fTemp61;
																																						self.fTempPerm59 = fTemp62;
																																						self.fTempPerm60 = fTemp63;
																																						self.fTempPerm61 = fTemp64;
																																						self.fTempPerm62 = fTemp65;
																																						self.fTempPerm63 = fTemp66;
																																						self.fTempPerm64 = fTemp67;
																																						self.fTempPerm65 = fTemp68;
																																						self.fTempPerm66 = fTemp69;
																																						self.fTempPerm67 = fTemp70;
																																						self.fTempPerm68 = fTemp71;
																																						self.fTempPerm69 = fTemp72;
																																						self.fTempPerm70 = fTemp73;
																																						self.fTempPerm71 = fTemp74;
																																						self.fTempPerm72 = fTemp75;
																																						self.fTempPerm73 = fTemp76;
																																						self.fTempPerm74 = fTemp77;
																																						self.fTempPerm75 = fTemp78;
																																						self.fTempPerm76 = fTemp79;
																																						self.fTempPerm77 = fTemp80;
																																						self.fTempPerm78 = fTemp81;
																																						self.fTempPerm79 = fTemp82;
																																						self.fTempPerm80 = fTemp83;
																																						self.fTempPerm81 = fTemp84;
																																						self.fTempPerm82 = fTemp85;
																																						self.fTempPerm83 = fTemp86;
																																						self.fTempPerm84 = fTemp87;
																																						self.fTempPerm85 = fTemp88;
																																						self.iTempPerm86 = iTemp89;
																																						self.iTempPerm87 = iTemp90;
																																						self.iTempPerm88 = iTemp91;
																																						self.iTempPerm89 = iTemp92;
																																						self.fTempPerm90 = fTemp93;
																																						self.fTempPerm91 = fTemp94;
																																						self.fTempPerm92 = fTemp95;
																																						self.fTempPerm93 = fTemp96;
																																						self.fTempPerm94 = fTemp97;
																																						self.fTempPerm95 = fTemp98;
																																						self.fTempPerm96 = fTemp99;
																																						self.fTempPerm97 = fTemp100;
																																						self.fTempPerm98 = fTemp101;
																																						self.fTempPerm99 = fTemp102;
																																						self.fTempPerm100 = fTemp103;
																																						self.fTempPerm101 = fTemp104;
																																						self.fTempPerm102 = fTemp105;
																																						self.fTempPerm103 = fTemp106;
																																						self.fTempPerm104 = fTemp107;
																																						self.fTempPerm105 = fTemp108;
																																						self.fTempPerm106 = fTemp109;
																																						self.fTempPerm107 = fTemp110;
																																						self.fTempPerm108 = fTemp111;
																																						self.fTempPerm109 = fTemp112;
																																						self.fTempPerm110 = fTemp113;
																																						self.fTempPerm111 = fTemp114;
																																						self.fTempPerm112 = fTemp115;
																																						self.fTempPerm113 = fTemp116;
																																						self.fTempPerm114 = fTemp117;
																																						self.fTempPerm115 = fTemp118;
																																						self.fTempPerm116 = fTemp119;
																																						self.fTempPerm117 = fTemp120;
																																						self.fTempPerm118 = fTemp121;
																																						self.fTempPerm119 = fTemp122;
																																						self.fTempPerm120 = fTemp123;
																																						self.fTempPerm121 = fTemp124;
																																						self.fTempPerm122 = fTemp125;
																																						self.fTempPerm123 = fTemp126;
																																						self.fTempPerm124 = fTemp127;
																																						self.fTempPerm125 = fTemp128;
																																						self.fTempPerm126 = fTemp129;
																																						self.fTempPerm127 = fTemp130;
																																						self.fTempPerm128 = fTemp131;
																																						self.fTempPerm129 = fTemp132;
																																						self.fTempPerm130 = fTemp133;
																																						self.fTempPerm131 = fTemp134;
																																						self.fTempPerm132 = fTemp135;
																																						self.fTempPerm133 = fTemp136;
																																						self.fTempPerm134 = fTemp137;
																																						self.fTempPerm135 = fTemp138;
																																						self.fTempPerm136 = fTemp139;
																																						self.fTempPerm137 = fTemp140;
																																						self.fTempPerm138 = fTemp141;
																																						self.fTempPerm139 = fTemp142;
																																						self.fTempPerm140 = fTemp143;
																																						self.fTempPerm141 = fTemp144;
																																						self.fTempPerm142 = fTemp145;
																																						self.fTempPerm143 = fTemp146;
																																						self.fTempPerm144 = fTemp147;
																																						self.fTempPerm145 = fTemp148;
																																						self.fTempPerm146 = fTemp149;
																																						self.fTempPerm147 = fTemp150;
																																						self.fTempPerm148 = fTemp151;
																																						self.fTempPerm149 = fTemp152;
																																						self.fTempPerm150 = fTemp153;
																																						self.fTempPerm151 = fTemp154;
																																						self.fTempPerm152 = fTemp155;
																																						self.fTempPerm153 = fTemp156;
																																						self.fTempPerm154 = fTemp157;
																																						self.fTempPerm155 = fTemp158;
																																						self.fTempPerm156 = fTemp159;
																																						self.fTempPerm157 = fTemp160;
																																						self.fTempPerm158 = fTemp161;
																																						self.fTempPerm159 = fTemp162;
																																						self.fTempPerm160 = fTemp163;
																																						self.fTempPerm161 = fTemp164;
																																						self.fTempPerm162 = fTemp165;
																																					}

}

