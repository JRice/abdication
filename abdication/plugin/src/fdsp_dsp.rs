/* ------------------------------------------------------------
name: "plugin-fdsp"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn FdspDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn FdspDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct FdspDsp {
	fHslider0: F32,
	fVec0: [F32;2],
	fEntry0: F32,
	fTempPerm0: F32,
	IOTA0: i32,
	fVec1: [F32;8192],
	fTempPerm1: F32,
	fTempPerm2: F32,
	fSampleRate: i32,
	fConst0: F32,
	fTempPerm3: F32,
	fButton0: F32,
	fVec2: [F32;2],
	iVec3: [i32;2],
	iTempPerm4: i32,
	iRec0: [i32;2],
	fTempPerm5: F32,
	fConst1: F32,
	fTempPerm6: F32,
	fHslider1: F32,
	fVec4: [F32;2],
	fHslider2: F32,
	fTempPerm7: F32,
	fRec1: [F32;2],
	fRec2: [F32;2],
	fTempPerm8: F32,
	fTempPerm9: F32,
	fConst2: F32,
	fTempPerm10: F32,
	fTempPerm11: F32,
	fEntry1: F32,
	fHslider3: F32,
	fVec5: [F32;2],
	fTempPerm12: F32,
	fRec6: [F32;2],
	fRec7: [F32;2],
	fTempPerm13: F32,
	fTempPerm14: F32,
	iTempPerm15: i32,
	fTempPerm16: F32,
	fTempPerm17: F32,
	fConst3: F32,
	fConst4: F32,
	fTempPerm18: F32,
	fTempPerm19: F32,
	fTempPerm20: F32,
	fVec6: [F32;2],
	fTempPerm21: F32,
	fRec8: [F32;2],
	fRec5: [F32;2],
	fRec10: [F32;2],
	fRec9: [F32;2],
	fRec12: [F32;2],
	fRec11: [F32;2],
	fRec14: [F32;2],
	fRec13: [F32;2],
	fRec16: [F32;2],
	fRec15: [F32;2],
	fRec18: [F32;2],
	fRec17: [F32;2],
	fRec4: [F32;2],
	fRec3: [F32;2],
	fTempPerm22: F32,
	fRec19: [F32;2],
	fTempPerm23: F32,
	fRec20: [F32;2],
	fRec21: [F32;2],
	fTempPerm24: F32,
	fTempPerm25: F32,
	fTempPerm26: F32,
	fTempPerm27: F32,
	fTempPerm28: F32,
	iTempPerm29: i32,
	fTempPerm30: F32,
	fTempPerm31: F32,
	fTempPerm32: F32,
	fTempPerm33: F32,
	fTempPerm34: F32,
	fRec22: [F32;8192],
	fTempPerm35: F32,
	iTempPerm36: i32,
	fTempPerm37: F32,
	fTempPerm38: F32,
	fTempPerm39: F32,
	fTempPerm40: F32,
	fTempPerm41: F32,
	fTempPerm42: F32,
	fTempPerm43: F32,
	fTempPerm44: F32,
	fRec23: [F32;2],
	fRec24: [F32;2],
	fTempPerm45: F32,
	fHslider4: F32,
	fVec7: [F32;2],
	fTempPerm46: F32,
	fRec25: [F32;2],
	fRec26: [F32;2],
	fTempPerm47: F32,
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	iTempPerm51: i32,
	fTempPerm52: F32,
	fTempPerm53: F32,
	fConst5: F32,
	fHslider5: F32,
	fVec8: [F32;2],
	fTempPerm54: F32,
	fRec28: [F32;2],
	fRec29: [F32;2],
	fTempPerm55: F32,
	fTempPerm56: F32,
	iTempPerm57: i32,
	fTempPerm58: F32,
	fTempPerm59: F32,
	fTempPerm60: F32,
	fTempPerm61: F32,
	fTempPerm62: F32,
	fHslider6: F32,
	fVec9: [F32;2],
	fTempPerm63: F32,
	fRec31: [F32;2],
	fRec32: [F32;2],
	fRec30: [F32;3],
	fHslider7: F32,
	fVec10: [F32;2],
	fTempPerm64: F32,
	fRec33: [F32;2],
	fRec34: [F32;2],
	fConst6: F32,
	fTempPerm65: F32,
	fTempPerm66: F32,
	iTempPerm67: i32,
	fTempPerm68: F32,
	fTempPerm69: F32,
	fTempPerm70: F32,
	fTempPerm71: F32,
	fTempPerm72: F32,
	fRec27: [F32;2],
	fTempPerm73: F32,
	fTempPerm74: F32,
	fTempPerm75: F32,
	fRec35: [F32;2],
	fRec36: [F32;2],
	fTempPerm76: F32,
	fConst7: F32,
	iTempPerm77: i32,
	iTempPerm78: i32,
	fConst8: F32,
	fRec39: [F32;2],
	fHslider8: F32,
	iRec40: [i32;2],
	iTempPerm79: i32,
	fRec38: [F32;2],
	fConst9: F32,
	fRec37: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fRec43: [F32;2],
	iRec44: [i32;2],
	iTempPerm80: i32,
	fRec42: [F32;2],
	fConst12: F32,
	fRec41: [F32;2],
	fHslider9: F32,
	fVec11: [F32;2],
	fTempPerm81: F32,
	fRec45: [F32;2],
	fRec46: [F32;2],
	fTempPerm82: F32,
	fTempPerm83: F32,
	iTempPerm84: i32,
	fTempPerm85: F32,
	fTempPerm86: F32,
	fTempPerm87: F32,
	fTempPerm88: F32,
	fConst13: F32,
	fTempPerm89: F32,
	fTempPerm90: F32,
	fConst14: F32,
	fTempPerm91: F32,
	fTempPerm92: F32,
	fHslider10: F32,
	fVec12: [F32;2],
	fTempPerm93: F32,
	fRec49: [F32;2],
	fRec50: [F32;2],
	fTempPerm94: F32,
	fTempPerm95: F32,
	iTempPerm96: i32,
	fTempPerm97: F32,
	fTempPerm98: F32,
	fHslider11: F32,
	fVec13: [F32;2],
	fTempPerm99: F32,
	fRec51: [F32;2],
	fRec52: [F32;2],
	fTempPerm100: F32,
	fTempPerm101: F32,
	iTempPerm102: i32,
	fTempPerm103: F32,
	fTempPerm104: F32,
	fTempPerm105: F32,
	fTempPerm106: F32,
	fTempPerm107: F32,
	iTempPerm108: i32,
	fTempPerm109: F32,
	fTempPerm110: F32,
	fTempPerm111: F32,
	fTempPerm112: F32,
	fTempPerm113: F32,
	fRec54: [F32;2],
	fRec53: [F32;8192],
	fTempPerm114: F32,
	fVec14: [F32;2],
	fRec48: [F32;2],
	fRec47: [F32;2],
	fRec55: [F32;2],
	fTempPerm115: F32,
	fTempPerm116: F32,
}

impl FaustDsp for FdspDsp {
	type T = F32;
		
	fn new() -> FdspDsp { 
		FdspDsp {
			fHslider0: 0.0,
			fVec0: [0.0;2],
			fEntry0: 0.0,
			fTempPerm0: 0.0,
			IOTA0: 0,
			fVec1: [0.0;8192],
			fTempPerm1: 0.0,
			fTempPerm2: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fTempPerm3: 0.0,
			fButton0: 0.0,
			fVec2: [0.0;2],
			iVec3: [0;2],
			iTempPerm4: 0,
			iRec0: [0;2],
			fTempPerm5: 0.0,
			fConst1: 0.0,
			fTempPerm6: 0.0,
			fHslider1: 0.0,
			fVec4: [0.0;2],
			fHslider2: 0.0,
			fTempPerm7: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fTempPerm8: 0.0,
			fTempPerm9: 0.0,
			fConst2: 0.0,
			fTempPerm10: 0.0,
			fTempPerm11: 0.0,
			fEntry1: 0.0,
			fHslider3: 0.0,
			fVec5: [0.0;2],
			fTempPerm12: 0.0,
			fRec6: [0.0;2],
			fRec7: [0.0;2],
			fTempPerm13: 0.0,
			fTempPerm14: 0.0,
			iTempPerm15: 0,
			fTempPerm16: 0.0,
			fTempPerm17: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			fTempPerm18: 0.0,
			fTempPerm19: 0.0,
			fTempPerm20: 0.0,
			fVec6: [0.0;2],
			fTempPerm21: 0.0,
			fRec8: [0.0;2],
			fRec5: [0.0;2],
			fRec10: [0.0;2],
			fRec9: [0.0;2],
			fRec12: [0.0;2],
			fRec11: [0.0;2],
			fRec14: [0.0;2],
			fRec13: [0.0;2],
			fRec16: [0.0;2],
			fRec15: [0.0;2],
			fRec18: [0.0;2],
			fRec17: [0.0;2],
			fRec4: [0.0;2],
			fRec3: [0.0;2],
			fTempPerm22: 0.0,
			fRec19: [0.0;2],
			fTempPerm23: 0.0,
			fRec20: [0.0;2],
			fRec21: [0.0;2],
			fTempPerm24: 0.0,
			fTempPerm25: 0.0,
			fTempPerm26: 0.0,
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			iTempPerm29: 0,
			fTempPerm30: 0.0,
			fTempPerm31: 0.0,
			fTempPerm32: 0.0,
			fTempPerm33: 0.0,
			fTempPerm34: 0.0,
			fRec22: [0.0;8192],
			fTempPerm35: 0.0,
			iTempPerm36: 0,
			fTempPerm37: 0.0,
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fTempPerm40: 0.0,
			fTempPerm41: 0.0,
			fTempPerm42: 0.0,
			fTempPerm43: 0.0,
			fTempPerm44: 0.0,
			fRec23: [0.0;2],
			fRec24: [0.0;2],
			fTempPerm45: 0.0,
			fHslider4: 0.0,
			fVec7: [0.0;2],
			fTempPerm46: 0.0,
			fRec25: [0.0;2],
			fRec26: [0.0;2],
			fTempPerm47: 0.0,
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			iTempPerm51: 0,
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fConst5: 0.0,
			fHslider5: 0.0,
			fVec8: [0.0;2],
			fTempPerm54: 0.0,
			fRec28: [0.0;2],
			fRec29: [0.0;2],
			fTempPerm55: 0.0,
			fTempPerm56: 0.0,
			iTempPerm57: 0,
			fTempPerm58: 0.0,
			fTempPerm59: 0.0,
			fTempPerm60: 0.0,
			fTempPerm61: 0.0,
			fTempPerm62: 0.0,
			fHslider6: 0.0,
			fVec9: [0.0;2],
			fTempPerm63: 0.0,
			fRec31: [0.0;2],
			fRec32: [0.0;2],
			fRec30: [0.0;3],
			fHslider7: 0.0,
			fVec10: [0.0;2],
			fTempPerm64: 0.0,
			fRec33: [0.0;2],
			fRec34: [0.0;2],
			fConst6: 0.0,
			fTempPerm65: 0.0,
			fTempPerm66: 0.0,
			iTempPerm67: 0,
			fTempPerm68: 0.0,
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			fTempPerm71: 0.0,
			fTempPerm72: 0.0,
			fRec27: [0.0;2],
			fTempPerm73: 0.0,
			fTempPerm74: 0.0,
			fTempPerm75: 0.0,
			fRec35: [0.0;2],
			fRec36: [0.0;2],
			fTempPerm76: 0.0,
			fConst7: 0.0,
			iTempPerm77: 0,
			iTempPerm78: 0,
			fConst8: 0.0,
			fRec39: [0.0;2],
			fHslider8: 0.0,
			iRec40: [0;2],
			iTempPerm79: 0,
			fRec38: [0.0;2],
			fConst9: 0.0,
			fRec37: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fRec43: [0.0;2],
			iRec44: [0;2],
			iTempPerm80: 0,
			fRec42: [0.0;2],
			fConst12: 0.0,
			fRec41: [0.0;2],
			fHslider9: 0.0,
			fVec11: [0.0;2],
			fTempPerm81: 0.0,
			fRec45: [0.0;2],
			fRec46: [0.0;2],
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			iTempPerm84: 0,
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			fTempPerm87: 0.0,
			fTempPerm88: 0.0,
			fConst13: 0.0,
			fTempPerm89: 0.0,
			fTempPerm90: 0.0,
			fConst14: 0.0,
			fTempPerm91: 0.0,
			fTempPerm92: 0.0,
			fHslider10: 0.0,
			fVec12: [0.0;2],
			fTempPerm93: 0.0,
			fRec49: [0.0;2],
			fRec50: [0.0;2],
			fTempPerm94: 0.0,
			fTempPerm95: 0.0,
			iTempPerm96: 0,
			fTempPerm97: 0.0,
			fTempPerm98: 0.0,
			fHslider11: 0.0,
			fVec13: [0.0;2],
			fTempPerm99: 0.0,
			fRec51: [0.0;2],
			fRec52: [0.0;2],
			fTempPerm100: 0.0,
			fTempPerm101: 0.0,
			iTempPerm102: 0,
			fTempPerm103: 0.0,
			fTempPerm104: 0.0,
			fTempPerm105: 0.0,
			fTempPerm106: 0.0,
			fTempPerm107: 0.0,
			iTempPerm108: 0,
			fTempPerm109: 0.0,
			fTempPerm110: 0.0,
			fTempPerm111: 0.0,
			fTempPerm112: 0.0,
			fTempPerm113: 0.0,
			fRec54: [0.0;2],
			fRec53: [0.0;8192],
			fTempPerm114: 0.0,
			fVec14: [0.0;2],
			fRec48: [0.0;2],
			fRec47: [0.0;2],
			fRec55: [0.0;2],
			fTempPerm115: 0.0,
			fTempPerm116: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn FdspDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("delays.lib/fdelay3:author", r"Julius O. Smith III");
		m.declare("delays.lib/fdelayltv:author", r"Julius O. Smith III");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("filename", r"plugin-fdsp.dsp");
		m.declare("filters.lib/dcblocker:author", r"Julius O. Smith III");
		m.declare("filters.lib/dcblocker:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/dcblocker:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
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
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("filters.lib/zero:author", r"Julius O. Smith III");
		m.declare("filters.lib/zero:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/zero:license", r"MIT-style STK-4.3 license");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"plugin-fdsp");
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
		self.fHslider0 = 0.5;
		self.fEntry0 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider1 = 0.5;
		self.fHslider2 = 64.0;
		self.fEntry1 = 2.0;
		self.fHslider3 = 7.0;
		self.fHslider4 = 0.3;
		self.fHslider5 = 0.5;
		self.fHslider6 = 0.2;
		self.fHslider7 = 0.4;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.3;
		self.fHslider10 = 0.4;
		self.fHslider11 = 0.6;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		self.fTempPerm0 = 0.0;
		self.IOTA0 = 0;
		for l1 in 0..8192 {
			self.fVec1[l1 as usize] = 0.0;
		}
		self.fTempPerm1 = 0.0;
		self.fTempPerm2 = 0.0;
		self.fTempPerm3 = 0.0;
		for l2 in 0..2 {
			self.fVec2[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.iVec3[l3 as usize] = 0;
		}
		self.iTempPerm4 = 0;
		for l4 in 0..2 {
			self.iRec0[l4 as usize] = 0;
		}
		self.fTempPerm5 = 0.0;
		self.fTempPerm6 = 0.0;
		for l5 in 0..2 {
			self.fVec4[l5 as usize] = 0.0;
		}
		self.fTempPerm7 = 0.0;
		for l6 in 0..2 {
			self.fRec1[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec2[l7 as usize] = 0.0;
		}
		self.fTempPerm8 = 0.0;
		self.fTempPerm9 = 0.0;
		self.fTempPerm10 = 0.0;
		self.fTempPerm11 = 0.0;
		for l8 in 0..2 {
			self.fVec5[l8 as usize] = 0.0;
		}
		self.fTempPerm12 = 0.0;
		for l9 in 0..2 {
			self.fRec6[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec7[l10 as usize] = 0.0;
		}
		self.fTempPerm13 = 0.0;
		self.fTempPerm14 = 0.0;
		self.iTempPerm15 = 0;
		self.fTempPerm16 = 0.0;
		self.fTempPerm17 = 0.0;
		self.fTempPerm18 = 0.0;
		self.fTempPerm19 = 0.0;
		self.fTempPerm20 = 0.0;
		for l11 in 0..2 {
			self.fVec6[l11 as usize] = 0.0;
		}
		self.fTempPerm21 = 0.0;
		for l12 in 0..2 {
			self.fRec8[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec5[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec10[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec9[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec12[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec11[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec14[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec13[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec16[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec15[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec18[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec17[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec4[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec3[l25 as usize] = 0.0;
		}
		self.fTempPerm22 = 0.0;
		for l26 in 0..2 {
			self.fRec19[l26 as usize] = 0.0;
		}
		self.fTempPerm23 = 0.0;
		for l27 in 0..2 {
			self.fRec20[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec21[l28 as usize] = 0.0;
		}
		self.fTempPerm24 = 0.0;
		self.fTempPerm25 = 0.0;
		self.fTempPerm26 = 0.0;
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		self.iTempPerm29 = 0;
		self.fTempPerm30 = 0.0;
		self.fTempPerm31 = 0.0;
		self.fTempPerm32 = 0.0;
		self.fTempPerm33 = 0.0;
		self.fTempPerm34 = 0.0;
		for l29 in 0..8192 {
			self.fRec22[l29 as usize] = 0.0;
		}
		self.fTempPerm35 = 0.0;
		self.iTempPerm36 = 0;
		self.fTempPerm37 = 0.0;
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		self.fTempPerm44 = 0.0;
		for l30 in 0..2 {
			self.fRec23[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec24[l31 as usize] = 0.0;
		}
		self.fTempPerm45 = 0.0;
		for l32 in 0..2 {
			self.fVec7[l32 as usize] = 0.0;
		}
		self.fTempPerm46 = 0.0;
		for l33 in 0..2 {
			self.fRec25[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec26[l34 as usize] = 0.0;
		}
		self.fTempPerm47 = 0.0;
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.iTempPerm51 = 0;
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		for l35 in 0..2 {
			self.fVec8[l35 as usize] = 0.0;
		}
		self.fTempPerm54 = 0.0;
		for l36 in 0..2 {
			self.fRec28[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec29[l37 as usize] = 0.0;
		}
		self.fTempPerm55 = 0.0;
		self.fTempPerm56 = 0.0;
		self.iTempPerm57 = 0;
		self.fTempPerm58 = 0.0;
		self.fTempPerm59 = 0.0;
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		self.fTempPerm62 = 0.0;
		for l38 in 0..2 {
			self.fVec9[l38 as usize] = 0.0;
		}
		self.fTempPerm63 = 0.0;
		for l39 in 0..2 {
			self.fRec31[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec32[l40 as usize] = 0.0;
		}
		for l41 in 0..3 {
			self.fRec30[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec10[l42 as usize] = 0.0;
		}
		self.fTempPerm64 = 0.0;
		for l43 in 0..2 {
			self.fRec33[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec34[l44 as usize] = 0.0;
		}
		self.fTempPerm65 = 0.0;
		self.fTempPerm66 = 0.0;
		self.iTempPerm67 = 0;
		self.fTempPerm68 = 0.0;
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		self.fTempPerm71 = 0.0;
		self.fTempPerm72 = 0.0;
		for l45 in 0..2 {
			self.fRec27[l45 as usize] = 0.0;
		}
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		self.fTempPerm75 = 0.0;
		for l46 in 0..2 {
			self.fRec35[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec36[l47 as usize] = 0.0;
		}
		self.fTempPerm76 = 0.0;
		self.iTempPerm77 = 0;
		self.iTempPerm78 = 0;
		for l48 in 0..2 {
			self.fRec39[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.iRec40[l49 as usize] = 0;
		}
		self.iTempPerm79 = 0;
		for l50 in 0..2 {
			self.fRec38[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec37[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec43[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.iRec44[l53 as usize] = 0;
		}
		self.iTempPerm80 = 0;
		for l54 in 0..2 {
			self.fRec42[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec41[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fVec11[l56 as usize] = 0.0;
		}
		self.fTempPerm81 = 0.0;
		for l57 in 0..2 {
			self.fRec45[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec46[l58 as usize] = 0.0;
		}
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.iTempPerm84 = 0;
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		self.fTempPerm87 = 0.0;
		self.fTempPerm88 = 0.0;
		self.fTempPerm89 = 0.0;
		self.fTempPerm90 = 0.0;
		self.fTempPerm91 = 0.0;
		self.fTempPerm92 = 0.0;
		for l59 in 0..2 {
			self.fVec12[l59 as usize] = 0.0;
		}
		self.fTempPerm93 = 0.0;
		for l60 in 0..2 {
			self.fRec49[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec50[l61 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		self.iTempPerm96 = 0;
		self.fTempPerm97 = 0.0;
		self.fTempPerm98 = 0.0;
		for l62 in 0..2 {
			self.fVec13[l62 as usize] = 0.0;
		}
		self.fTempPerm99 = 0.0;
		for l63 in 0..2 {
			self.fRec51[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec52[l64 as usize] = 0.0;
		}
		self.fTempPerm100 = 0.0;
		self.fTempPerm101 = 0.0;
		self.iTempPerm102 = 0;
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		self.fTempPerm105 = 0.0;
		self.fTempPerm106 = 0.0;
		self.fTempPerm107 = 0.0;
		self.iTempPerm108 = 0;
		self.fTempPerm109 = 0.0;
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		self.fTempPerm112 = 0.0;
		self.fTempPerm113 = 0.0;
		for l65 in 0..2 {
			self.fRec54[l65 as usize] = 0.0;
		}
		for l66 in 0..8192 {
			self.fRec53[l66 as usize] = 0.0;
		}
		self.fTempPerm114 = 0.0;
		for l67 in 0..2 {
			self.fVec14[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec48[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec47[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec55[l70 as usize] = 0.0;
		}
		self.fTempPerm115 = 0.0;
		self.fTempPerm116 = 0.0;
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 5e+02 / self.fConst0;
		self.fConst2 = F32::exp(-(4.0 / self.fConst0));
		self.fConst3 = 0.45 * self.fConst0;
		self.fConst4 = 3.1415927 / self.fConst0;
		self.fConst5 = 0.4 * self.fConst0;
		self.fConst6 = 0.5 * self.fConst0;
		self.fConst7 = F32::exp(-(19.4 / self.fConst0));
		self.fConst8 = 9.7 / self.fConst0;
		self.fConst9 = 1.0 - self.fConst7;
		self.fConst10 = F32::exp(-(6.6 / self.fConst0));
		self.fConst11 = 3.3 / self.fConst0;
		self.fConst12 = 1.0 - self.fConst10;
		self.fConst13 = F32::exp(-(2.0 / self.fConst0));
		self.fConst14 = 6.2831855 / self.fConst0;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		FdspDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin-fdsp");
		ui_interface.declare(None, "14", "");
		ui_interface.open_horizontal_box("FDSP");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.declare(Some(ParamIndex(0)), "style", "menu{'Off':0;'Water':1;'Tornado':2;'Flange':3;'Phase':4}");
		ui_interface.add_num_entry("Type", ParamIndex(0), 0.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "st");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(1), 7.0, -24.0, 48.0, 0.01);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.add_horizontal_slider("Feedback", ParamIndex(2), 0.5, -0.95, 0.95, 0.001);
		ui_interface.declare(Some(ParamIndex(3)), "9", "");
		ui_interface.add_horizontal_slider("Mix", ParamIndex(3), 0.5, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "15", "");
		ui_interface.open_horizontal_box("Water");
		ui_interface.declare(Some(ParamIndex(4)), "2", "");
		ui_interface.add_horizontal_slider("Ring", ParamIndex(4), 0.6, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(5)), "3", "");
		ui_interface.add_horizontal_slider("Damp", ParamIndex(5), 0.4, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "4", "");
		ui_interface.add_horizontal_slider("Wobble", ParamIndex(6), 0.3, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "16", "");
		ui_interface.open_horizontal_box("Tornado");
		ui_interface.declare(Some(ParamIndex(7)), "1", "");
		ui_interface.add_horizontal_slider("Drive", ParamIndex(7), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "2", "");
		ui_interface.add_horizontal_slider("Overtone", ParamIndex(8), 0.4, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "3", "");
		ui_interface.add_horizontal_slider("Ceiling", ParamIndex(9), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "4", "");
		ui_interface.add_horizontal_slider("Edge", ParamIndex(10), 0.2, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "18", "");
		ui_interface.open_horizontal_box("Phase");
		ui_interface.declare(Some(ParamIndex(11)), "3", "");
		ui_interface.declare(Some(ParamIndex(11)), "style", "menu{'2':0;'4':1;'6':2;'8':3;'10':4;'12':5}");
		ui_interface.add_num_entry("Stages", ParamIndex(11), 2.0, 0.0, 5.0, 1.0);
		ui_interface.close_box();
		ui_interface.add_button("gate", ParamIndex(12));
		ui_interface.add_horizontal_slider("ramp", ParamIndex(13), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("slot", ParamIndex(14), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			12 => Some(self.fButton0),
			0 => Some(self.fEntry0),
			11 => Some(self.fEntry1),
			3 => Some(self.fHslider0),
			2 => Some(self.fHslider1),
			5 => Some(self.fHslider10),
			4 => Some(self.fHslider11),
			13 => Some(self.fHslider2),
			1 => Some(self.fHslider3),
			7 => Some(self.fHslider4),
			9 => Some(self.fHslider5),
			10 => Some(self.fHslider6),
			8 => Some(self.fHslider7),
			14 => Some(self.fHslider8),
			6 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			12 => { self.fButton0 = value }
			0 => { self.fEntry0 = value }
			11 => { self.fEntry1 = value }
			3 => { self.fHslider0 = value }
			2 => { self.fHslider1 = value }
			5 => { self.fHslider10 = value }
			4 => { self.fHslider11 = value }
			13 => { self.fHslider2 = value }
			1 => { self.fHslider3 = value }
			7 => { self.fHslider4 = value }
			9 => { self.fHslider5 = value }
			10 => { self.fHslider6 = value }
			8 => { self.fHslider7 = value }
			14 => { self.fHslider8 = value }
			6 => { self.fHslider9 = value }
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
		let mut fSlow0: F32 = self.fHslider0;
		let mut iSlow1: i32 = (fSlow0 != 0.0) as i32;
		let mut fSlow2: F32 = self.fEntry0;
		let mut fSlow3: F32 = fSlow2 * (((fSlow2 > 0.0) as i32) & iSlow1) as F32;
		let mut iSlow4: i32 = (fSlow3 == 4.0) as i32;
		let mut fSlow5: F32 = (iSlow4) as F32;
		let mut iSlow6: i32 = (fSlow5 != 0.0) as i32;
		let mut iSlow7: i32 = (fSlow3 == 3.0) as i32;
		let mut fSlow8: F32 = (iSlow7) as F32;
		let mut iSlow9: i32 = (fSlow8 != 0.0) as i32;
		let mut iSlow10: i32 = (fSlow3 == 2.0) as i32;
		let mut fSlow11: F32 = (iSlow10) as F32;
		let mut iSlow12: i32 = (fSlow11 != 0.0) as i32;
		let mut iSlow13: i32 = (fSlow3 == 1.0) as i32;
		let mut fSlow14: F32 = (iSlow13) as F32;
		let mut iSlow15: i32 = (fSlow14 != 0.0) as i32;
		let mut iSlow16: i32 = (fSlow3 == 0.0) as i32;
		let mut fSlow17: F32 = (iSlow16) as F32;
		let mut iSlow18: i32 = (fSlow17 != 0.0) as i32;
		let mut fTemp0: F32 = self.fTempPerm0;
		let mut fTemp1: F32 = self.fTempPerm1;
		let mut fTemp2: F32 = self.fTempPerm2;
		let mut fTemp3: F32 = self.fTempPerm3;
		let mut fSlow19: F32 = self.fButton0;
		let mut iTemp4: i32 = self.iTempPerm4;
		let mut fTemp5: F32 = self.fTempPerm5;
		let mut fTemp6: F32 = self.fTempPerm6;
		let mut fSlow20: F32 = self.fHslider1;
		let mut fSlow21: F32 = self.fHslider2;
		let mut fTemp7: F32 = self.fTempPerm7;
		let mut fTemp8: F32 = self.fTempPerm8;
		let mut fTemp9: F32 = self.fTempPerm9;
		let mut fTemp10: F32 = self.fTempPerm10;
		let mut fTemp11: F32 = self.fTempPerm11;
		let mut iSlow22: i32 = (self.fEntry1) as i32;
		let mut iSlow23: i32 = (iSlow22 >= 3) as i32;
		let mut iSlow24: i32 = (iSlow22 >= 2) as i32;
		let mut iSlow25: i32 = (iSlow22 >= 1) as i32;
		let mut fSlow26: F32 = self.fHslider3;
		let mut fTemp12: F32 = self.fTempPerm12;
		let mut fTemp13: F32 = self.fTempPerm13;
		let mut fTemp14: F32 = self.fTempPerm14;
		let mut iTemp15: i32 = self.iTempPerm15;
		let mut fTemp16: F32 = self.fTempPerm16;
		let mut fTemp17: F32 = self.fTempPerm17;
		let mut fTemp18: F32 = self.fTempPerm18;
		let mut fTemp19: F32 = self.fTempPerm19;
		let mut fTemp20: F32 = self.fTempPerm20;
		let mut fTemp21: F32 = self.fTempPerm21;
		let mut iSlow27: i32 = (iSlow22 >= 5) as i32;
		let mut iSlow28: i32 = (iSlow22 >= 4) as i32;
		let mut fTemp22: F32 = self.fTempPerm22;
		let mut fTemp23: F32 = self.fTempPerm23;
		let mut fSlow29: F32 = (iSlow1) as F32;
		let mut fTemp24: F32 = self.fTempPerm24;
		let mut fSlow30: F32 = ((iSlow4 != 0) as i32) as u32 as F32;
		let mut fTemp25: F32 = self.fTempPerm25;
		let mut fTemp26: F32 = self.fTempPerm26;
		let mut fTemp27: F32 = self.fTempPerm27;
		let mut fTemp28: F32 = self.fTempPerm28;
		let mut iTemp29: i32 = self.iTempPerm29;
		let mut fTemp30: F32 = self.fTempPerm30;
		let mut fTemp31: F32 = self.fTempPerm31;
		let mut fTemp32: F32 = self.fTempPerm32;
		let mut fTemp33: F32 = self.fTempPerm33;
		let mut fTemp34: F32 = self.fTempPerm34;
		let mut fTemp35: F32 = self.fTempPerm35;
		let mut iTemp36: i32 = self.iTempPerm36;
		let mut fTemp37: F32 = self.fTempPerm37;
		let mut fTemp38: F32 = self.fTempPerm38;
		let mut fTemp39: F32 = self.fTempPerm39;
		let mut fTemp40: F32 = self.fTempPerm40;
		let mut fTemp41: F32 = self.fTempPerm41;
		let mut fTemp42: F32 = self.fTempPerm42;
		let mut fTemp43: F32 = self.fTempPerm43;
		let mut fTemp44: F32 = self.fTempPerm44;
		let mut fTemp45: F32 = self.fTempPerm45;
		let mut fSlow31: F32 = ((iSlow7 != 0) as i32) as u32 as F32;
		let mut fSlow32: F32 = self.fHslider4;
		let mut fTemp46: F32 = self.fTempPerm46;
		let mut fTemp47: F32 = self.fTempPerm47;
		let mut fTemp48: F32 = self.fTempPerm48;
		let mut fTemp49: F32 = self.fTempPerm49;
		let mut fTemp50: F32 = self.fTempPerm50;
		let mut iTemp51: i32 = self.iTempPerm51;
		let mut fTemp52: F32 = self.fTempPerm52;
		let mut fTemp53: F32 = self.fTempPerm53;
		let mut fSlow33: F32 = self.fHslider5;
		let mut fTemp54: F32 = self.fTempPerm54;
		let mut fTemp55: F32 = self.fTempPerm55;
		let mut fTemp56: F32 = self.fTempPerm56;
		let mut iTemp57: i32 = self.iTempPerm57;
		let mut fTemp58: F32 = self.fTempPerm58;
		let mut fTemp59: F32 = self.fTempPerm59;
		let mut fTemp60: F32 = self.fTempPerm60;
		let mut fTemp61: F32 = self.fTempPerm61;
		let mut fTemp62: F32 = self.fTempPerm62;
		let mut fSlow34: F32 = self.fHslider6;
		let mut fTemp63: F32 = self.fTempPerm63;
		let mut fSlow35: F32 = self.fHslider7;
		let mut fTemp64: F32 = self.fTempPerm64;
		let mut fTemp65: F32 = self.fTempPerm65;
		let mut fTemp66: F32 = self.fTempPerm66;
		let mut iTemp67: i32 = self.iTempPerm67;
		let mut fTemp68: F32 = self.fTempPerm68;
		let mut fTemp69: F32 = self.fTempPerm69;
		let mut fTemp70: F32 = self.fTempPerm70;
		let mut fTemp71: F32 = self.fTempPerm71;
		let mut fTemp72: F32 = self.fTempPerm72;
		let mut fTemp73: F32 = self.fTempPerm73;
		let mut fTemp74: F32 = self.fTempPerm74;
		let mut fTemp75: F32 = self.fTempPerm75;
		let mut fTemp76: F32 = self.fTempPerm76;
		let mut fSlow36: F32 = ((iSlow10 != 0) as i32) as u32 as F32;
		let mut iTemp77: i32 = self.iTempPerm77;
		let mut iTemp78: i32 = self.iTempPerm78;
		let mut fSlow37: F32 = 97.0 * self.fHslider8;
		let mut iSlow38: i32 = (fSlow37 + 6001.0) as i32;
		let mut iTemp79: i32 = self.iTempPerm79;
		let mut iSlow39: i32 = (fSlow37 + 6e+03) as i32;
		let mut iTemp80: i32 = self.iTempPerm80;
		let mut fSlow40: F32 = self.fHslider9;
		let mut fTemp81: F32 = self.fTempPerm81;
		let mut fTemp82: F32 = self.fTempPerm82;
		let mut fTemp83: F32 = self.fTempPerm83;
		let mut iTemp84: i32 = self.iTempPerm84;
		let mut fTemp85: F32 = self.fTempPerm85;
		let mut fTemp86: F32 = self.fTempPerm86;
		let mut fTemp87: F32 = self.fTempPerm87;
		let mut fTemp88: F32 = self.fTempPerm88;
		let mut fTemp89: F32 = self.fTempPerm89;
		let mut fTemp90: F32 = self.fTempPerm90;
		let mut fTemp91: F32 = self.fTempPerm91;
		let mut fTemp92: F32 = self.fTempPerm92;
		let mut fSlow41: F32 = self.fHslider10;
		let mut fTemp93: F32 = self.fTempPerm93;
		let mut fTemp94: F32 = self.fTempPerm94;
		let mut fTemp95: F32 = self.fTempPerm95;
		let mut iTemp96: i32 = self.iTempPerm96;
		let mut fTemp97: F32 = self.fTempPerm97;
		let mut fTemp98: F32 = self.fTempPerm98;
		let mut fSlow42: F32 = self.fHslider11;
		let mut fTemp99: F32 = self.fTempPerm99;
		let mut fTemp100: F32 = self.fTempPerm100;
		let mut fTemp101: F32 = self.fTempPerm101;
		let mut iTemp102: i32 = self.iTempPerm102;
		let mut fTemp103: F32 = self.fTempPerm103;
		let mut fTemp104: F32 = self.fTempPerm104;
		let mut fTemp105: F32 = self.fTempPerm105;
		let mut fTemp106: F32 = self.fTempPerm106;
		let mut fTemp107: F32 = self.fTempPerm107;
		let mut iTemp108: i32 = self.iTempPerm108;
		let mut fTemp109: F32 = self.fTempPerm109;
		let mut fTemp110: F32 = self.fTempPerm110;
		let mut fTemp111: F32 = self.fTempPerm111;
		let mut fTemp112: F32 = self.fTempPerm112;
		let mut fTemp113: F32 = self.fTempPerm113;
		let mut fTemp114: F32 = self.fTempPerm114;
		let mut fTemp115: F32 = self.fTempPerm115;
		let mut fSlow43: F32 = ((iSlow13 != 0) as i32) as u32 as F32;
		let mut fTemp116: F32 = self.fTempPerm116;
		let mut fSlow44: F32 = ((iSlow16 != 0) as i32) as u32 as F32;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0);
		for ((input0, input1), output0) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			if iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow6))) != 0 {
				fTemp0 = *input0;
				self.fVec1[(self.IOTA0 & 8191) as usize] = fTemp0;
			}
				if iSlow15 | (iSlow12 | (iSlow9 | iSlow6)) != 0 {
					fTemp1 = *input1;
				}
					if iSlow12 | iSlow6 != 0 {
						fTemp2 = F32::max(fTemp1, 1.0);
						fTemp3 = self.fConst0 / fTemp2;
					}
						if iSlow15 | (iSlow12 | (iSlow9 | iSlow6)) != 0 {
							self.fVec2[0] = fSlow19;
							self.iVec3[0] = 1;
							iTemp4 = (fSlow19 > self.fVec2[1]) as i32;
							self.iRec0[0] = i32::wrapping_mul(i32::wrapping_add(self.iRec0[1], 1), i32::wrapping_sub(1, iTemp4));
							fTemp5 = (self.iRec0[0]) as F32;
						}
							if iSlow6 != 0 {
								fTemp6 = F32::min(1.0, F32::max(0.0, self.fConst1 * (fTemp5 - fTemp3)));
							}
								if iSlow9 | iSlow6 != 0 {
									self.fVec4[0] = fSlow20;
									fTemp7 = if ((fSlow20 != self.fVec4[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec1[1] + -1.0};
									self.fRec1[0] = fTemp7;
									self.fRec2[0] = if iTemp4 != 0 {fSlow20} else {if (fTemp7 > 0.0) as i32 != 0 {self.fRec2[1] + (fSlow20 - self.fRec2[1]) / F32::max(1.0, fTemp7)} else {fSlow20}};
								}
									if iSlow6 != 0 {
										fTemp8 = F32::max(-0.9, F32::min(0.9, self.fRec2[0]));
										fTemp9 = F32::sqrt(1.0 - FdspDsp_faustpower2_f(fTemp8) * FdspDsp_faustpower2_f(fTemp6));
										fTemp10 = if (fTemp6 < 1.0) as i32 != 0 {1.0} else {self.fConst2};
										fTemp11 = 1.0 - fTemp10;
									}
										if iSlow15 | (iSlow9 | iSlow6) != 0 {
											self.fVec5[0] = fSlow26;
											fTemp12 = if ((fSlow26 != self.fVec5[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec6[1] + -1.0};
											self.fRec6[0] = fTemp12;
											self.fRec7[0] = if iTemp4 != 0 {fSlow26} else {if (fTemp12 > 0.0) as i32 != 0 {self.fRec7[1] + (fSlow26 - self.fRec7[1]) / F32::max(1.0, fTemp12)} else {fSlow26}};
											fTemp13 = 0.083333336 * self.fRec7[0];
										}
											if iSlow9 | iSlow6 != 0 {
												fTemp14 = F32::floor(fTemp13);
												iTemp15 = (fTemp14) as i32;
												fTemp16 = fTemp13 - fTemp14;
												fTemp17 = fTemp1 * (fTemp16 * (fTemp16 * (fTemp16 * (fTemp16 * (0.0013333 * fTemp16 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp15))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp15))) as F32;
											}
												if iSlow6 != 0 {
													fTemp18 = F32::tan(self.fConst4 * F32::min(fTemp17, self.fConst3));
													fTemp19 = fTemp18 + 1.0;
													fTemp20 = fTemp0 + fTemp8 * fTemp6 * self.fRec4[1];
													self.fVec6[0] = fTemp20;
													fTemp21 = fTemp18 + -1.0;
													self.fRec8[0] = self.fVec6[1] - fTemp21 * (self.fRec8[1] - fTemp20) / fTemp19;
													self.fRec5[0] = self.fRec8[1] - fTemp21 * (self.fRec5[1] - self.fRec8[0]) / fTemp19;
													self.fRec10[0] = self.fRec5[1] - fTemp21 * (self.fRec10[1] - self.fRec5[0]) / fTemp19;
													self.fRec9[0] = self.fRec10[1] - fTemp21 * (self.fRec9[1] - self.fRec10[0]) / fTemp19;
													self.fRec12[0] = self.fRec9[1] - fTemp21 * (self.fRec12[1] - self.fRec9[0]) / fTemp19;
													self.fRec11[0] = self.fRec12[1] - fTemp21 * (self.fRec11[1] - self.fRec12[0]) / fTemp19;
													self.fRec14[0] = self.fRec11[1] - fTemp21 * (self.fRec14[1] - self.fRec11[0]) / fTemp19;
													self.fRec13[0] = self.fRec14[1] - fTemp21 * (self.fRec13[1] - self.fRec14[0]) / fTemp19;
													self.fRec16[0] = self.fRec13[1] - fTemp21 * (self.fRec16[1] - self.fRec13[0]) / fTemp19;
													self.fRec15[0] = self.fRec16[1] - fTemp21 * (self.fRec15[1] - self.fRec16[0]) / fTemp19;
													self.fRec18[0] = self.fRec15[1] - fTemp21 * (self.fRec18[1] - self.fRec15[0]) / fTemp19;
													self.fRec17[0] = self.fRec18[1] - fTemp21 * (self.fRec17[1] - self.fRec18[0]) / fTemp19;
													self.fRec4[0] = if iSlow23 != 0 {if iSlow27 != 0 {self.fRec17[0]} else {if iSlow28 != 0 {self.fRec15[0]} else {self.fRec13[0]}}} else {if iSlow24 != 0 {self.fRec11[0]} else {if iSlow25 != 0 {self.fRec9[0]} else {self.fRec5[0]}}};
													self.fRec3[0] = FdspDsp_faustpower2_f(self.fRec4[0]) * FdspDsp_faustpower2_f(fTemp9) * fTemp11 + fTemp10 * self.fRec3[1];
												}
													if iSlow15 | (iSlow12 | (iSlow9 | iSlow6)) != 0 {
														fTemp22 = FdspDsp_faustpower2_f(fTemp0);
													}
														if iSlow6 != 0 {
															self.fRec19[0] = fTemp22 * fTemp11 + fTemp10 * self.fRec19[1];
														}
															if iSlow15 | (iSlow12 | (iSlow9 | iSlow6)) != 0 {
																fTemp23 = if ((fSlow0 != self.fVec0[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec20[1] + -1.0};
																self.fRec20[0] = fTemp23;
																self.fRec21[0] = if iTemp4 != 0 {fSlow0} else {if (fTemp23 > 0.0) as i32 != 0 {self.fRec21[1] + (fSlow0 - self.fRec21[1]) / F32::max(1.0, fTemp23)} else {fSlow0}};
															}
																if iSlow6 != 0 {
																	fTemp24 = fSlow5 * (fTemp0 + fSlow29 * self.fRec21[0] * fTemp6 * (self.fRec4[0] * F32::min(3.9810717, F32::max(0.25118864, F32::sqrt((self.fRec19[0] + 1e-09) / (self.fRec3[0] + 1e-09)))) * fTemp9 - fTemp0));
																}
																	if iSlow9 != 0 {
																		fTemp25 = F32::min(8188.0, F32::max(3.0, self.fConst0 / F32::max(fTemp17, 1.0)));
																		fTemp26 = F32::min(1.0, F32::max(0.0, self.fConst1 * (fTemp5 - fTemp25)));
																		fTemp27 = F32::sqrt(1.0 - FdspDsp_faustpower2_f(self.fRec2[0]) * FdspDsp_faustpower2_f(fTemp26));
																		fTemp28 = fTemp25 + -1.999995;
																		iTemp29 = (fTemp28) as i32;
																		fTemp30 = F32::floor(fTemp28);
																		fTemp31 = fTemp25 + (-3.0 - fTemp30);
																		fTemp32 = fTemp25 + (-2.0 - fTemp30);
																		fTemp33 = fTemp25 + (-1.0 - fTemp30);
																		fTemp34 = fTemp33 * fTemp32;
																		self.fRec22[(self.IOTA0 & 8191) as usize] = fTemp0 + self.fRec2[0] * fTemp26 * ((fTemp25 + (-4.0 - fTemp30)) * (fTemp31 * (0.5 * fTemp33 * self.fRec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp29, 1))), 1))) & 8191) as usize] - 0.16666667 * self.fRec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, iTemp29)), 1))) & 8191) as usize] * fTemp32) - 0.5 * fTemp34 * self.fRec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp29, 2))), 1))) & 8191) as usize]) + 0.16666667 * fTemp34 * fTemp31 * self.fRec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp29, 3))), 1))) & 8191) as usize]);
																		fTemp35 = fTemp25 + -0.999995;
																		iTemp36 = (fTemp35) as i32;
																		fTemp37 = F32::floor(fTemp35);
																		fTemp38 = fTemp25 + (-2.0 - fTemp37);
																		fTemp39 = fTemp25 + (-1.0 - fTemp37);
																		fTemp40 = fTemp25 - fTemp37;
																		fTemp41 = fTemp40 * fTemp39;
																		fTemp42 = (fTemp25 + (-3.0 - fTemp37)) * (fTemp38 * (0.5 * fTemp40 * self.fRec22[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp36, 1))))) & 8191) as usize] - 0.16666667 * self.fRec22[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp36)))) & 8191) as usize] * fTemp39) - 0.5 * fTemp41 * self.fRec22[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp36, 2))))) & 8191) as usize]) + 0.16666667 * fTemp41 * fTemp38 * self.fRec22[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp36, 3))))) & 8191) as usize];
																		fTemp43 = if (fTemp26 < 1.0) as i32 != 0 {1.0} else {self.fConst2};
																		fTemp44 = 1.0 - fTemp43;
																		self.fRec23[0] = FdspDsp_faustpower2_f(fTemp42) * FdspDsp_faustpower2_f(fTemp27) * fTemp44 + fTemp43 * self.fRec23[1];
																		self.fRec24[0] = fTemp22 * fTemp44 + fTemp43 * self.fRec24[1];
																		fTemp45 = fSlow8 * (fTemp0 + fSlow29 * self.fRec21[0] * fTemp26 * (F32::min(3.9810717, F32::max(0.25118864, F32::sqrt((self.fRec24[0] + 1e-09) / (self.fRec23[0] + 1e-09)))) * fTemp42 * fTemp27 - fTemp0));
																	}
																		if iSlow12 != 0 {
																			self.fVec7[0] = fSlow32;
																			fTemp46 = if ((fSlow32 != self.fVec7[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec25[1] + -1.0};
																			self.fRec25[0] = fTemp46;
																			self.fRec26[0] = if iTemp4 != 0 {fSlow32} else {if (fTemp46 > 0.0) as i32 != 0 {self.fRec26[1] + (fSlow32 - self.fRec26[1]) / F32::max(1.0, fTemp46)} else {fSlow32}};
																			fTemp47 = 4.0 * self.fRec26[0];
																			fTemp48 = F32::floor(fTemp47);
																			fTemp49 = fTemp47 - fTemp48;
																			fTemp50 = fTemp49 * (fTemp49 * (fTemp49 * (fTemp49 * (0.0013333 * fTemp49 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0;
																			iTemp51 = (fTemp48) as i32;
																			fTemp52 = (1 << std::cmp::min(30, std::cmp::max(0, iTemp51))) as F32;
																			fTemp53 = (1 << std::cmp::min(30, std::cmp::max(0, -iTemp51))) as F32;
																			self.fVec8[0] = fSlow33;
																			fTemp54 = if ((fSlow33 != self.fVec8[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec28[1] + -1.0};
																			self.fRec28[0] = fTemp54;
																			self.fRec29[0] = if iTemp4 != 0 {fSlow33} else {if (fTemp54 > 0.0) as i32 != 0 {self.fRec29[1] + (fSlow33 - self.fRec29[1]) / F32::max(1.0, fTemp54)} else {fSlow33}};
																			fTemp55 = 5.0 * self.fRec29[0];
																			fTemp56 = F32::floor(fTemp55);
																			iTemp57 = (fTemp56) as i32;
																			fTemp58 = fTemp55 - fTemp56;
																			fTemp59 = F32::tan(self.fConst4 * F32::min(self.fConst5, fTemp1 * (1 << std::cmp::min(30, std::cmp::max(0, iTemp57))) as F32 * (fTemp58 * (fTemp58 * (fTemp58 * (fTemp58 * (0.0013333 * fTemp58 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp57))) as F32));
																			fTemp60 = 1.0 / fTemp59;
																			fTemp61 = (fTemp60 + 1.4142135) / fTemp59 + 1.0;
																			fTemp62 = F32::min(1.0, F32::max(0.0, self.fConst1 * (fTemp5 + (-3.0 - fTemp3))));
																			self.fVec9[0] = fSlow34;
																			fTemp63 = if ((fSlow34 != self.fVec9[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec31[1] + -1.0};
																			self.fRec31[0] = fTemp63;
																			self.fRec32[0] = if iTemp4 != 0 {fSlow34} else {if (fTemp63 > 0.0) as i32 != 0 {self.fRec32[1] + (fSlow34 - self.fRec32[1]) / F32::max(1.0, fTemp63)} else {fSlow34}};
																			self.fRec30[0] = F32::tanh(2.5 * (fTemp0 + 2.5 * self.fRec32[0] * fTemp62 * self.fRec27[1])) - (self.fRec30[2] * ((fTemp60 + -1.4142135) / fTemp59 + 1.0) + 2.0 * self.fRec30[1] * (1.0 - 1.0 / FdspDsp_faustpower2_f(fTemp59))) / fTemp61;
																			self.fVec10[0] = fSlow35;
																			fTemp64 = if ((fSlow35 != self.fVec10[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec33[1] + -1.0};
																			self.fRec33[0] = fTemp64;
																			self.fRec34[0] = if iTemp4 != 0 {fSlow35} else {if (fTemp64 > 0.0) as i32 != 0 {self.fRec34[1] + (fSlow35 - self.fRec34[1]) / F32::max(1.0, fTemp64)} else {fSlow35}};
																			fTemp65 = F32::min(8188.0, F32::max(3.0, self.fConst6 * (FdspDsp_faustpower2_f(self.fRec34[0]) * ((self.fRec30[2] + self.fRec30[0] + 2.0 * self.fRec30[1]) / fTemp61 + 1.0) / fTemp2) + 3.0));
																			fTemp66 = fTemp65 + -0.999995;
																			iTemp67 = (fTemp66) as i32;
																			fTemp68 = F32::floor(fTemp66);
																			fTemp69 = fTemp65 + (-2.0 - fTemp68);
																			fTemp70 = fTemp65 + (-1.0 - fTemp68);
																			fTemp71 = fTemp65 - fTemp68;
																			fTemp72 = fTemp71 * fTemp70;
																			self.fRec27[0] = (fTemp65 + (-3.0 - fTemp68)) * (fTemp69 * (0.5 * fTemp71 * self.fVec1[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp67, 1))))) & 8191) as usize] - 0.16666667 * self.fVec1[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp67)))) & 8191) as usize] * fTemp70) - 0.5 * fTemp72 * self.fVec1[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp67, 2))))) & 8191) as usize]) + 0.16666667 * fTemp72 * fTemp69 * self.fVec1[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp67, 3))))) & 8191) as usize];
																			fTemp73 = F32::tanh(self.fRec27[0] * fTemp52 * fTemp50 / fTemp53);
																			fTemp74 = if (fTemp62 < 1.0) as i32 != 0 {1.0} else {self.fConst2};
																			fTemp75 = 1.0 - fTemp74;
																			self.fRec35[0] = FdspDsp_faustpower2_f(fTemp53) * FdspDsp_faustpower2_f(fTemp73) * fTemp75 / (FdspDsp_faustpower2_f(fTemp52) * FdspDsp_faustpower2_f(fTemp50)) + fTemp74 * self.fRec35[1];
																			self.fRec36[0] = fTemp22 * fTemp75 + fTemp74 * self.fRec36[1];
																			fTemp76 = fSlow11 * (fTemp0 + fSlow29 * self.fRec21[0] * fTemp62 * (F32::min(3.9810717, F32::max(0.25118864, F32::sqrt((self.fRec36[0] + 1e-09) / (self.fRec35[0] + 1e-09)))) * fTemp53 * fTemp73 / (fTemp52 * fTemp50) - fTemp0));
																		}
																			if iSlow15 != 0 {
																				iTemp77 = i32::wrapping_sub(1, self.iVec3[1]);
																				iTemp78 = (iTemp77 > 0) as i32;
																				self.fRec39[0] = self.fConst8 + (self.fRec39[1] - F32::floor(self.fConst8 + self.fRec39[1]));
																				self.iRec40[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec40[1], i32::wrapping_mul(iSlow38, iTemp77))), 12345);
																				iTemp79 = i32::wrapping_mul(1664525, self.iRec40[0] ^ (self.iRec40[0] >> 15));
																				self.fRec38[0] = if ((self.fRec39[0] < self.fRec39[1]) as i32) | iTemp78 != 0 {4.656613e-10 * (i32::wrapping_mul(1013904223, iTemp79 ^ (iTemp79 >> 13))) as F32} else {self.fRec38[1]};
																				self.fRec37[0] = self.fConst9 * self.fRec38[0] + self.fConst7 * self.fRec37[1];
																				self.fRec43[0] = self.fConst11 + (self.fRec43[1] - F32::floor(self.fConst11 + self.fRec43[1]));
																				self.iRec44[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec44[1], i32::wrapping_mul(iSlow39, iTemp77))), 12345);
																				iTemp80 = i32::wrapping_mul(1664525, self.iRec44[0] ^ (self.iRec44[0] >> 15));
																				self.fRec42[0] = if ((self.fRec43[0] < self.fRec43[1]) as i32) | iTemp78 != 0 {4.656613e-10 * (i32::wrapping_mul(1013904223, iTemp80 ^ (iTemp80 >> 13))) as F32} else {self.fRec42[1]};
																				self.fRec41[0] = self.fConst12 * self.fRec42[0] + self.fConst10 * self.fRec41[1];
																				self.fVec11[0] = fSlow40;
																				fTemp81 = if ((fSlow40 != self.fVec11[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec45[1] + -1.0};
																				self.fRec45[0] = fTemp81;
																				self.fRec46[0] = if iTemp4 != 0 {fSlow40} else {if (fTemp81 > 0.0) as i32 != 0 {self.fRec46[1] + (fSlow40 - self.fRec46[1]) / F32::max(1.0, fTemp81)} else {fSlow40}};
																				fTemp82 = fTemp13 + 0.033333335 * self.fRec46[0] * (0.7 * self.fRec41[0] + 0.3 * self.fRec37[0]);
																				fTemp83 = F32::floor(fTemp82);
																				iTemp84 = (fTemp83) as i32;
																				fTemp85 = fTemp82 - fTemp83;
																				fTemp86 = F32::min(self.fConst5, fTemp1 * (fTemp85 * (fTemp85 * (fTemp85 * (fTemp85 * (0.0013333 * fTemp85 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp84))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp84))) as F32);
																				fTemp87 = F32::max(fTemp86, 1.0);
																				fTemp88 = F32::min(1.0, F32::max(0.0, self.fConst1 * (fTemp5 - self.fConst0 / fTemp87)));
																				fTemp89 = if (fTemp88 < 1.0) as i32 != 0 {1.0} else {self.fConst13};
																				fTemp90 = 1.0 - fTemp89;
																				fTemp91 = self.fConst14 * fTemp86;
																				fTemp92 = F32::cos(fTemp91);
																				self.fVec12[0] = fSlow41;
																				fTemp93 = if ((fSlow41 != self.fVec12[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec49[1] + -1.0};
																				self.fRec49[0] = fTemp93;
																				self.fRec50[0] = if iTemp4 != 0 {fSlow41} else {if (fTemp93 > 0.0) as i32 != 0 {self.fRec50[1] + (fSlow41 - self.fRec50[1]) / F32::max(1.0, fTemp93)} else {fSlow41}};
																				fTemp94 = 5.9 * self.fRec50[0];
																				fTemp95 = F32::floor(6.5 - fTemp94);
																				iTemp96 = (fTemp95) as i32;
																				fTemp97 = 6.5 - (fTemp94 + fTemp95);
																				fTemp98 = F32::exp(-(self.fConst14 * F32::min(self.fConst3, (1 << std::cmp::min(30, std::cmp::max(0, iTemp96))) as F32 * fTemp86 * (fTemp97 * (fTemp97 * (fTemp97 * (fTemp97 * (0.0013333 * fTemp97 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp96))) as F32)));
																				self.fVec13[0] = fSlow42;
																				fTemp99 = if ((fSlow42 != self.fVec13[1]) as i32) | iTemp4 != 0 {fSlow21} else {self.fRec51[1] + -1.0};
																				self.fRec51[0] = fTemp99;
																				self.fRec52[0] = if iTemp4 != 0 {fSlow42} else {if (fTemp99 > 0.0) as i32 != 0 {self.fRec52[1] + (fSlow42 - self.fRec52[1]) / F32::max(1.0, fTemp99)} else {fSlow42}};
																				fTemp100 = 9.228819 * self.fRec52[0];
																				fTemp101 = F32::floor(fTemp100);
																				iTemp102 = (fTemp101) as i32;
																				fTemp103 = fTemp100 - fTemp101;
																				fTemp104 = F32::powf(1e+01, -(1.5e+02 * ((1 << std::cmp::min(30, std::cmp::max(0, -iTemp102))) as F32 / (fTemp87 * (fTemp103 * (fTemp103 * (fTemp103 * (fTemp103 * (0.0013333 * fTemp103 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp102))) as F32))));
																				fTemp105 = 1.0 - fTemp98;
																				fTemp106 = F32::min(8188.0, F32::max(3.0, self.fConst0 * (1.0 / fTemp87 - 0.15915494 * (F32::atan(fTemp98 * F32::sin(fTemp91) / (1.0 - fTemp98 * fTemp92)) / fTemp86)) + -1.0));
																				fTemp107 = fTemp106 + -0.999995;
																				iTemp108 = (fTemp107) as i32;
																				fTemp109 = F32::floor(fTemp107);
																				fTemp110 = fTemp106 + (-1.0 - fTemp109);
																				fTemp111 = fTemp106 - fTemp109;
																				fTemp112 = fTemp111 * fTemp110;
																				fTemp113 = fTemp106 + (-2.0 - fTemp109);
																				self.fRec54[0] = 0.16666667 * fTemp112 * fTemp113 * self.fRec53[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp108, 3))), 1))) & 8191) as usize] + self.fRec54[1] * fTemp98 + (fTemp106 + (-3.0 - fTemp109)) * (fTemp113 * (0.5 * fTemp111 * self.fRec53[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp108, 1))), 1))) & 8191) as usize] - 0.16666667 * self.fRec53[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, iTemp108)), 1))) & 8191) as usize] * fTemp110) - 0.5 * fTemp112 * self.fRec53[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp108, 2))), 1))) & 8191) as usize]);
																				self.fRec53[(self.IOTA0 & 8191) as usize] = fTemp0 + self.fRec54[0] * fTemp88 * fTemp105 * fTemp104;
																				fTemp114 = self.fRec53[(self.IOTA0 & 8191) as usize] * (1.0 - fTemp88 * fTemp105 * fTemp104 / F32::sqrt(fTemp98 * (fTemp98 - 2.0 * fTemp92) + 1.0));
																				self.fVec14[0] = fTemp114;
																				self.fRec48[0] = 0.995 * self.fRec48[1] + fTemp114 - self.fVec14[1];
																				self.fRec47[0] = FdspDsp_faustpower2_f(self.fRec48[0]) * fTemp90 + fTemp89 * self.fRec47[1];
																				self.fRec55[0] = fTemp22 * fTemp90 + fTemp89 * self.fRec55[1];
																				fTemp115 = fSlow14 * (fTemp0 + fSlow29 * self.fRec21[0] * (self.fRec48[0] * F32::min(15.848932, F32::max(1.0, F32::sqrt((self.fRec55[0] + 1e-09) / (self.fRec47[0] + 1e-09)))) - fTemp0) * fTemp88);
																			}
																				if iSlow18 != 0 {
																					fTemp116 = fSlow17 * fTemp0;
																				}
																					*output0 = fSlow44 * fTemp116 + fSlow43 * fTemp115 + fSlow36 * fTemp76 + fSlow31 * fTemp45 + fSlow30 * fTemp24;
																					self.fVec0[1] = self.fVec0[0];
																					self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
																					if iSlow15 | (iSlow12 | (iSlow9 | iSlow6)) != 0 {
																						self.fVec2[1] = self.fVec2[0];
																						self.iVec3[1] = self.iVec3[0];
																						self.iRec0[1] = self.iRec0[0];
																					}
																						if iSlow9 | iSlow6 != 0 {
																							self.fVec4[1] = self.fVec4[0];
																							self.fRec1[1] = self.fRec1[0];
																							self.fRec2[1] = self.fRec2[0];
																						}
																							if iSlow15 | (iSlow9 | iSlow6) != 0 {
																								self.fVec5[1] = self.fVec5[0];
																								self.fRec6[1] = self.fRec6[0];
																								self.fRec7[1] = self.fRec7[0];
																							}
																								if iSlow6 != 0 {
																									self.fVec6[1] = self.fVec6[0];
																									self.fRec8[1] = self.fRec8[0];
																									self.fRec5[1] = self.fRec5[0];
																									self.fRec10[1] = self.fRec10[0];
																									self.fRec9[1] = self.fRec9[0];
																									self.fRec12[1] = self.fRec12[0];
																									self.fRec11[1] = self.fRec11[0];
																									self.fRec14[1] = self.fRec14[0];
																									self.fRec13[1] = self.fRec13[0];
																									self.fRec16[1] = self.fRec16[0];
																									self.fRec15[1] = self.fRec15[0];
																									self.fRec18[1] = self.fRec18[0];
																									self.fRec17[1] = self.fRec17[0];
																									self.fRec4[1] = self.fRec4[0];
																									self.fRec3[1] = self.fRec3[0];
																									self.fRec19[1] = self.fRec19[0];
																								}
																									if iSlow15 | (iSlow12 | (iSlow9 | iSlow6)) != 0 {
																										self.fRec20[1] = self.fRec20[0];
																										self.fRec21[1] = self.fRec21[0];
																									}
																										if iSlow9 != 0 {
																											self.fRec23[1] = self.fRec23[0];
																											self.fRec24[1] = self.fRec24[0];
																										}
																											if iSlow12 != 0 {
																												self.fVec7[1] = self.fVec7[0];
																												self.fRec25[1] = self.fRec25[0];
																												self.fRec26[1] = self.fRec26[0];
																												self.fVec8[1] = self.fVec8[0];
																												self.fRec28[1] = self.fRec28[0];
																												self.fRec29[1] = self.fRec29[0];
																												self.fVec9[1] = self.fVec9[0];
																												self.fRec31[1] = self.fRec31[0];
																												self.fRec32[1] = self.fRec32[0];
																												self.fRec30[2] = self.fRec30[1];
																												self.fRec30[1] = self.fRec30[0];
																												self.fVec10[1] = self.fVec10[0];
																												self.fRec33[1] = self.fRec33[0];
																												self.fRec34[1] = self.fRec34[0];
																												self.fRec27[1] = self.fRec27[0];
																												self.fRec35[1] = self.fRec35[0];
																												self.fRec36[1] = self.fRec36[0];
																											}
																												if iSlow15 != 0 {
																													self.fRec39[1] = self.fRec39[0];
																													self.iRec40[1] = self.iRec40[0];
																													self.fRec38[1] = self.fRec38[0];
																													self.fRec37[1] = self.fRec37[0];
																													self.fRec43[1] = self.fRec43[0];
																													self.iRec44[1] = self.iRec44[0];
																													self.fRec42[1] = self.fRec42[0];
																													self.fRec41[1] = self.fRec41[0];
																													self.fVec11[1] = self.fVec11[0];
																													self.fRec45[1] = self.fRec45[0];
																													self.fRec46[1] = self.fRec46[0];
																													self.fVec12[1] = self.fVec12[0];
																													self.fRec49[1] = self.fRec49[0];
																													self.fRec50[1] = self.fRec50[0];
																													self.fVec13[1] = self.fVec13[0];
																													self.fRec51[1] = self.fRec51[0];
																													self.fRec52[1] = self.fRec52[0];
																													self.fRec54[1] = self.fRec54[0];
																													self.fVec14[1] = self.fVec14[0];
																													self.fRec48[1] = self.fRec48[0];
																													self.fRec47[1] = self.fRec47[0];
																													self.fRec55[1] = self.fRec55[0];
																												}
																												}
																												self.fTempPerm0 = fTemp0;
																												self.fTempPerm1 = fTemp1;
																												self.fTempPerm2 = fTemp2;
																												self.fTempPerm3 = fTemp3;
																												self.iTempPerm4 = iTemp4;
																												self.fTempPerm5 = fTemp5;
																												self.fTempPerm6 = fTemp6;
																												self.fTempPerm7 = fTemp7;
																												self.fTempPerm8 = fTemp8;
																												self.fTempPerm9 = fTemp9;
																												self.fTempPerm10 = fTemp10;
																												self.fTempPerm11 = fTemp11;
																												self.fTempPerm12 = fTemp12;
																												self.fTempPerm13 = fTemp13;
																												self.fTempPerm14 = fTemp14;
																												self.iTempPerm15 = iTemp15;
																												self.fTempPerm16 = fTemp16;
																												self.fTempPerm17 = fTemp17;
																												self.fTempPerm18 = fTemp18;
																												self.fTempPerm19 = fTemp19;
																												self.fTempPerm20 = fTemp20;
																												self.fTempPerm21 = fTemp21;
																												self.fTempPerm22 = fTemp22;
																												self.fTempPerm23 = fTemp23;
																												self.fTempPerm24 = fTemp24;
																												self.fTempPerm25 = fTemp25;
																												self.fTempPerm26 = fTemp26;
																												self.fTempPerm27 = fTemp27;
																												self.fTempPerm28 = fTemp28;
																												self.iTempPerm29 = iTemp29;
																												self.fTempPerm30 = fTemp30;
																												self.fTempPerm31 = fTemp31;
																												self.fTempPerm32 = fTemp32;
																												self.fTempPerm33 = fTemp33;
																												self.fTempPerm34 = fTemp34;
																												self.fTempPerm35 = fTemp35;
																												self.iTempPerm36 = iTemp36;
																												self.fTempPerm37 = fTemp37;
																												self.fTempPerm38 = fTemp38;
																												self.fTempPerm39 = fTemp39;
																												self.fTempPerm40 = fTemp40;
																												self.fTempPerm41 = fTemp41;
																												self.fTempPerm42 = fTemp42;
																												self.fTempPerm43 = fTemp43;
																												self.fTempPerm44 = fTemp44;
																												self.fTempPerm45 = fTemp45;
																												self.fTempPerm46 = fTemp46;
																												self.fTempPerm47 = fTemp47;
																												self.fTempPerm48 = fTemp48;
																												self.fTempPerm49 = fTemp49;
																												self.fTempPerm50 = fTemp50;
																												self.iTempPerm51 = iTemp51;
																												self.fTempPerm52 = fTemp52;
																												self.fTempPerm53 = fTemp53;
																												self.fTempPerm54 = fTemp54;
																												self.fTempPerm55 = fTemp55;
																												self.fTempPerm56 = fTemp56;
																												self.iTempPerm57 = iTemp57;
																												self.fTempPerm58 = fTemp58;
																												self.fTempPerm59 = fTemp59;
																												self.fTempPerm60 = fTemp60;
																												self.fTempPerm61 = fTemp61;
																												self.fTempPerm62 = fTemp62;
																												self.fTempPerm63 = fTemp63;
																												self.fTempPerm64 = fTemp64;
																												self.fTempPerm65 = fTemp65;
																												self.fTempPerm66 = fTemp66;
																												self.iTempPerm67 = iTemp67;
																												self.fTempPerm68 = fTemp68;
																												self.fTempPerm69 = fTemp69;
																												self.fTempPerm70 = fTemp70;
																												self.fTempPerm71 = fTemp71;
																												self.fTempPerm72 = fTemp72;
																												self.fTempPerm73 = fTemp73;
																												self.fTempPerm74 = fTemp74;
																												self.fTempPerm75 = fTemp75;
																												self.fTempPerm76 = fTemp76;
																												self.iTempPerm77 = iTemp77;
																												self.iTempPerm78 = iTemp78;
																												self.iTempPerm79 = iTemp79;
																												self.iTempPerm80 = iTemp80;
																												self.fTempPerm81 = fTemp81;
																												self.fTempPerm82 = fTemp82;
																												self.fTempPerm83 = fTemp83;
																												self.iTempPerm84 = iTemp84;
																												self.fTempPerm85 = fTemp85;
																												self.fTempPerm86 = fTemp86;
																												self.fTempPerm87 = fTemp87;
																												self.fTempPerm88 = fTemp88;
																												self.fTempPerm89 = fTemp89;
																												self.fTempPerm90 = fTemp90;
																												self.fTempPerm91 = fTemp91;
																												self.fTempPerm92 = fTemp92;
																												self.fTempPerm93 = fTemp93;
																												self.fTempPerm94 = fTemp94;
																												self.fTempPerm95 = fTemp95;
																												self.iTempPerm96 = iTemp96;
																												self.fTempPerm97 = fTemp97;
																												self.fTempPerm98 = fTemp98;
																												self.fTempPerm99 = fTemp99;
																												self.fTempPerm100 = fTemp100;
																												self.fTempPerm101 = fTemp101;
																												self.iTempPerm102 = iTemp102;
																												self.fTempPerm103 = fTemp103;
																												self.fTempPerm104 = fTemp104;
																												self.fTempPerm105 = fTemp105;
																												self.fTempPerm106 = fTemp106;
																												self.fTempPerm107 = fTemp107;
																												self.iTempPerm108 = iTemp108;
																												self.fTempPerm109 = fTemp109;
																												self.fTempPerm110 = fTemp110;
																												self.fTempPerm111 = fTemp111;
																												self.fTempPerm112 = fTemp112;
																												self.fTempPerm113 = fTemp113;
																												self.fTempPerm114 = fTemp114;
																												self.fTempPerm115 = fTemp115;
																												self.fTempPerm116 = fTemp116;
																											}

}

