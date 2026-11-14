/* ------------------------------------------------------------
name: "plugin-mix"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn MixDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */


pub struct MixDspSIG0 {
	iVec12: [i32;2],
	iRec26: [i32;2],
}

impl MixDspSIG0 {
	
	fn get_num_inputsMixDspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsMixDspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initMixDspSIG0(&mut self, sample_rate: i32) {
		for l34 in 0..2 {
			self.iVec12[l34 as usize] = 0;
		}
		for l35 in 0..2 {
			self.iRec26[l35 as usize] = 0;
		}
	}
	
	fn fillMixDspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec12[0] = 1;
			self.iRec26[0] = (i32::wrapping_add(self.iVec12[1], self.iRec26[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec26[0]) as F32);
			self.iVec12[1] = self.iVec12[0];
			self.iRec26[1] = self.iRec26[0];
		}
	}

}


pub fn newMixDspSIG0() -> MixDspSIG0 { 
	MixDspSIG0 {
		iVec12: [0;2],
		iRec26: [0;2],
	}
}
fn MixDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut ftbl0MixDspSIG0: [F32;65536] = [0.0;65536];
fn MixDsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct MixDsp {
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	fVslider0: F32,
	fVec2: [F32;2],
	fHslider0: F32,
	fRec0: [F32;2],
	fRec1: [F32;2],
	fVslider1: F32,
	fVec3: [F32;2],
	fRec2: [F32;2],
	fRec3: [F32;2],
	fVslider2: F32,
	fVec4: [F32;2],
	fRec4: [F32;2],
	fRec5: [F32;2],
	fHslider1: F32,
	fVec5: [F32;2],
	fTempPerm0: F32,
	fRec6: [F32;2],
	fRec7: [F32;2],
	fTempPerm1: F32,
	fEntry0: F32,
	fSampleRate: i32,
	fConst2: F32,
	fHslider2: F32,
	fVec6: [F32;2],
	fTempPerm2: F32,
	fRec12: [F32;2],
	fRec13: [F32;2],
	fHslider3: F32,
	fVec7: [F32;2],
	fRec15: [F32;2],
	fRec16: [F32;2],
	fConst3: F32,
	fHslider4: F32,
	fRec17: [F32;2],
	iTempPerm3: i32,
	fTempPerm4: F32,
	fHslider5: F32,
	fVec8: [F32;2],
	fHslider6: F32,
	fVec9: [F32;2],
	fTempPerm5: F32,
	fRec18: [F32;2],
	fRec19: [F32;2],
	fTempPerm6: F32,
	iRec20: [i32;2],
	fTempPerm7: F32,
	fConst4: F32,
	fTempPerm8: F32,
	iRec21: [i32;2],
	iTempPerm9: i32,
	fTempPerm10: F32,
	fEntry1: F32,
	fHslider7: F32,
	fVec10: [F32;2],
	fRec22: [F32;2],
	fRec23: [F32;2],
	iRec14: [i32;2],
	fTempPerm11: F32,
	fTempPerm12: F32,
	fTempPerm13: F32,
	fVec11: [F32;2],
	fConst5: F32,
	fRec11: [F32;2],
	fConst6: F32,
	fConst7: F32,
	fConst8: F32,
	fRec10: [F32;2],
	fTempPerm14: F32,
	iTempPerm15: i32,
	fConst9: F32,
	fTempPerm16: F32,
	fRec27: [F32;2],
	fHslider8: F32,
	fTempPerm17: F32,
	fRec28: [F32;2],
	iRec30: [i32;2],
	iTempPerm18: i32,
	fConst11: F32,
	fConst13: F32,
	fRec29: [F32;2],
	fHslider9: F32,
	fRec31: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fTempPerm19: F32,
	fRec34: [F32;2],
	fRec33: [F32;2],
	fTempPerm20: F32,
	fVec13: [F32;2],
	fRec32: [F32;2],
	fTempPerm21: F32,
	fHslider10: F32,
	fRec35: [F32;2],
	fEntry2: F32,
	fHslider11: F32,
	fVec14: [F32;2],
	fRec36: [F32;2],
	fRec37: [F32;2],
	fHslider12: F32,
	fHslider13: F32,
	fRec38: [F32;2],
	fHslider14: F32,
	fTempPerm22: F32,
	fRec25: [F32;2],
	iTempPerm23: i32,
	iTempPerm24: i32,
	iRec24: [i32;2],
	fTempPerm25: F32,
	iTempPerm26: i32,
	iTempPerm27: i32,
	iRec39: [i32;2],
	fTempPerm28: F32,
	fRec40: [F32;2],
	fHslider15: F32,
	fTempPerm29: F32,
	fTempPerm30: F32,
	fTempPerm31: F32,
	fVec15: [F32;2],
	fConst16: F32,
	fConst17: F32,
	fConst18: F32,
	fRec9: [F32;2],
	fTempPerm32: F32,
	fVec16: [F32;2],
	fRec8: [F32;2],
	fTempPerm33: F32,
	fConst20: F32,
	fConst21: F32,
	fTempPerm34: F32,
	fRec44: [F32;2],
	fRec43: [F32;2],
	fTempPerm35: F32,
	fVec17: [F32;2],
	fConst22: F32,
	fRec42: [F32;2],
	fTempPerm36: F32,
	fTempPerm37: F32,
	fTempPerm38: F32,
	fTempPerm39: F32,
	fHslider16: F32,
	fVec18: [F32;2],
	fTempPerm40: F32,
	fRec46: [F32;2],
	fRec47: [F32;2],
	fTempPerm41: F32,
	fTempPerm42: F32,
	fConst23: F32,
	fTempPerm43: F32,
	fConst24: F32,
	fTempPerm44: F32,
	fTempPerm45: F32,
	fRec49: [F32;2],
	fRec48: [F32;2],
	fTempPerm46: F32,
	fConst25: F32,
	fTempPerm47: F32,
	fRec51: [F32;2],
	fConst26: F32,
	fTempPerm48: F32,
	fRec50: [F32;2],
	fTempPerm49: F32,
	fConst27: F32,
	fTempPerm50: F32,
	fRec52: [F32;2],
	fTempPerm51: F32,
	fConst28: F32,
	fTempPerm52: F32,
	fTempPerm53: F32,
	fTempPerm54: F32,
	fTempPerm55: F32,
	fRec53: [F32;3],
	fTempPerm56: F32,
	fConst29: F32,
	fConst30: F32,
	fTempPerm57: F32,
	fTempPerm58: F32,
	fTempPerm59: F32,
	fVec19: [F32;2],
	fRec45: [F32;2],
	fTempPerm60: F32,
	fTempPerm61: F32,
	fRec54: [F32;2],
	fRec55: [F32;2],
	fCheckbox0: F32,
	fTempPerm62: F32,
	fHslider17: F32,
	fRec56: [F32;2],
	iTempPerm63: i32,
	fTempPerm64: F32,
	fTempPerm65: F32,
	fTempPerm66: F32,
	fConst31: F32,
	fConst33: F32,
	fTempPerm67: F32,
	fVec20: [F32;2],
	fTempPerm68: F32,
	fConst34: F32,
	fRec58: [F32;2],
	fConst35: F32,
	fConst36: F32,
	fConst38: F32,
	fConst39: F32,
	fRec59: [F32;2],
	fConst40: F32,
	fConst41: F32,
	fTempPerm69: F32,
	fConst42: F32,
	fConst43: F32,
	fConst44: F32,
	fTempPerm70: F32,
	fVec21: [F32;2],
	fConst46: F32,
	fConst47: F32,
	fRec57: [F32;2],
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
	fTempPerm86: F32,
	fTempPerm87: F32,
	fTempPerm88: F32,
	fTempPerm89: F32,
	fTempPerm90: F32,
	fTempPerm91: F32,
	fTempPerm92: F32,
	fTempPerm93: F32,
	fTempPerm94: F32,
	fVec22: [F32;2],
	fRec41: [F32;2],
}

impl FaustDsp for MixDsp {
	type T = F32;
		
	fn new() -> MixDsp { 
		MixDsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			fVslider0: 0.0,
			fVec2: [0.0;2],
			fHslider0: 0.0,
			fRec0: [0.0;2],
			fRec1: [0.0;2],
			fVslider1: 0.0,
			fVec3: [0.0;2],
			fRec2: [0.0;2],
			fRec3: [0.0;2],
			fVslider2: 0.0,
			fVec4: [0.0;2],
			fRec4: [0.0;2],
			fRec5: [0.0;2],
			fHslider1: 0.0,
			fVec5: [0.0;2],
			fTempPerm0: 0.0,
			fRec6: [0.0;2],
			fRec7: [0.0;2],
			fTempPerm1: 0.0,
			fEntry0: 0.0,
			fSampleRate: 0,
			fConst2: 0.0,
			fHslider2: 0.0,
			fVec6: [0.0;2],
			fTempPerm2: 0.0,
			fRec12: [0.0;2],
			fRec13: [0.0;2],
			fHslider3: 0.0,
			fVec7: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;2],
			fConst3: 0.0,
			fHslider4: 0.0,
			fRec17: [0.0;2],
			iTempPerm3: 0,
			fTempPerm4: 0.0,
			fHslider5: 0.0,
			fVec8: [0.0;2],
			fHslider6: 0.0,
			fVec9: [0.0;2],
			fTempPerm5: 0.0,
			fRec18: [0.0;2],
			fRec19: [0.0;2],
			fTempPerm6: 0.0,
			iRec20: [0;2],
			fTempPerm7: 0.0,
			fConst4: 0.0,
			fTempPerm8: 0.0,
			iRec21: [0;2],
			iTempPerm9: 0,
			fTempPerm10: 0.0,
			fEntry1: 0.0,
			fHslider7: 0.0,
			fVec10: [0.0;2],
			fRec22: [0.0;2],
			fRec23: [0.0;2],
			iRec14: [0;2],
			fTempPerm11: 0.0,
			fTempPerm12: 0.0,
			fTempPerm13: 0.0,
			fVec11: [0.0;2],
			fConst5: 0.0,
			fRec11: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fConst8: 0.0,
			fRec10: [0.0;2],
			fTempPerm14: 0.0,
			iTempPerm15: 0,
			fConst9: 0.0,
			fTempPerm16: 0.0,
			fRec27: [0.0;2],
			fHslider8: 0.0,
			fTempPerm17: 0.0,
			fRec28: [0.0;2],
			iRec30: [0;2],
			iTempPerm18: 0,
			fConst11: 0.0,
			fConst13: 0.0,
			fRec29: [0.0;2],
			fHslider9: 0.0,
			fRec31: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fTempPerm19: 0.0,
			fRec34: [0.0;2],
			fRec33: [0.0;2],
			fTempPerm20: 0.0,
			fVec13: [0.0;2],
			fRec32: [0.0;2],
			fTempPerm21: 0.0,
			fHslider10: 0.0,
			fRec35: [0.0;2],
			fEntry2: 0.0,
			fHslider11: 0.0,
			fVec14: [0.0;2],
			fRec36: [0.0;2],
			fRec37: [0.0;2],
			fHslider12: 0.0,
			fHslider13: 0.0,
			fRec38: [0.0;2],
			fHslider14: 0.0,
			fTempPerm22: 0.0,
			fRec25: [0.0;2],
			iTempPerm23: 0,
			iTempPerm24: 0,
			iRec24: [0;2],
			fTempPerm25: 0.0,
			iTempPerm26: 0,
			iTempPerm27: 0,
			iRec39: [0;2],
			fTempPerm28: 0.0,
			fRec40: [0.0;2],
			fHslider15: 0.0,
			fTempPerm29: 0.0,
			fTempPerm30: 0.0,
			fTempPerm31: 0.0,
			fVec15: [0.0;2],
			fConst16: 0.0,
			fConst17: 0.0,
			fConst18: 0.0,
			fRec9: [0.0;2],
			fTempPerm32: 0.0,
			fVec16: [0.0;2],
			fRec8: [0.0;2],
			fTempPerm33: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fTempPerm34: 0.0,
			fRec44: [0.0;2],
			fRec43: [0.0;2],
			fTempPerm35: 0.0,
			fVec17: [0.0;2],
			fConst22: 0.0,
			fRec42: [0.0;2],
			fTempPerm36: 0.0,
			fTempPerm37: 0.0,
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fHslider16: 0.0,
			fVec18: [0.0;2],
			fTempPerm40: 0.0,
			fRec46: [0.0;2],
			fRec47: [0.0;2],
			fTempPerm41: 0.0,
			fTempPerm42: 0.0,
			fConst23: 0.0,
			fTempPerm43: 0.0,
			fConst24: 0.0,
			fTempPerm44: 0.0,
			fTempPerm45: 0.0,
			fRec49: [0.0;2],
			fRec48: [0.0;2],
			fTempPerm46: 0.0,
			fConst25: 0.0,
			fTempPerm47: 0.0,
			fRec51: [0.0;2],
			fConst26: 0.0,
			fTempPerm48: 0.0,
			fRec50: [0.0;2],
			fTempPerm49: 0.0,
			fConst27: 0.0,
			fTempPerm50: 0.0,
			fRec52: [0.0;2],
			fTempPerm51: 0.0,
			fConst28: 0.0,
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fTempPerm54: 0.0,
			fTempPerm55: 0.0,
			fRec53: [0.0;3],
			fTempPerm56: 0.0,
			fConst29: 0.0,
			fConst30: 0.0,
			fTempPerm57: 0.0,
			fTempPerm58: 0.0,
			fTempPerm59: 0.0,
			fVec19: [0.0;2],
			fRec45: [0.0;2],
			fTempPerm60: 0.0,
			fTempPerm61: 0.0,
			fRec54: [0.0;2],
			fRec55: [0.0;2],
			fCheckbox0: 0.0,
			fTempPerm62: 0.0,
			fHslider17: 0.0,
			fRec56: [0.0;2],
			iTempPerm63: 0,
			fTempPerm64: 0.0,
			fTempPerm65: 0.0,
			fTempPerm66: 0.0,
			fConst31: 0.0,
			fConst33: 0.0,
			fTempPerm67: 0.0,
			fVec20: [0.0;2],
			fTempPerm68: 0.0,
			fConst34: 0.0,
			fRec58: [0.0;2],
			fConst35: 0.0,
			fConst36: 0.0,
			fConst38: 0.0,
			fConst39: 0.0,
			fRec59: [0.0;2],
			fConst40: 0.0,
			fConst41: 0.0,
			fTempPerm69: 0.0,
			fConst42: 0.0,
			fConst43: 0.0,
			fConst44: 0.0,
			fTempPerm70: 0.0,
			fVec21: [0.0;2],
			fConst46: 0.0,
			fConst47: 0.0,
			fRec57: [0.0;2],
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
			fTempPerm86: 0.0,
			fTempPerm87: 0.0,
			fTempPerm88: 0.0,
			fTempPerm89: 0.0,
			fTempPerm90: 0.0,
			fTempPerm91: 0.0,
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fTempPerm94: 0.0,
			fVec22: [0.0;2],
			fRec41: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn MixDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("filename", r"plugin-mix.dsp");
		m.declare("filters.lib/dcblocker:author", r"Julius O. Smith III");
		m.declare("filters.lib/dcblocker:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/dcblocker:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/dcblockerat:author", r"Julius O. Smith III");
		m.declare("filters.lib/dcblockerat:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/dcblockerat:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/high_shelf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/high_shelf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/high_shelf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/low_shelf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/low_shelf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/low_shelf1:license", r"MIT-style STK-4.3 license");
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
		m.declare("name", r"plugin-mix");
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
		return 2;
	}
	fn get_num_outputs(&self) -> i32 {
		return 3;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: MixDspSIG0 = newMixDspSIG0();
		sig0.instance_initMixDspSIG0(sample_rate);
		sig0.fillMixDspSIG0(65536, unsafe { &mut ftbl0MixDspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fVslider0 = 0.15;
		self.fHslider0 = 64.0;
		self.fVslider1 = 0.0;
		self.fVslider2 = 1.0;
		self.fHslider1 = 0.0;
		self.fEntry0 = 1.0;
		self.fHslider2 = 0.3;
		self.fHslider3 = 0.1;
		self.fHslider4 = 1.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 6e+01;
		self.fEntry1 = 0.0;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 1.0;
		self.fHslider10 = 0.0;
		self.fEntry2 = 0.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 1.1e+02;
		self.fHslider14 = 0.0;
		self.fHslider15 = 8.0;
		self.fHslider16 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fHslider17 = 1.0;
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
		for l5 in 0..2 {
			self.fVec3[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec2[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec3[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fVec4[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec4[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec5[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fVec5[l11 as usize] = 0.0;
		}
		self.fTempPerm0 = 0.0;
		for l12 in 0..2 {
			self.fRec6[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec7[l13 as usize] = 0.0;
		}
		self.fTempPerm1 = 0.0;
		for l14 in 0..2 {
			self.fVec6[l14 as usize] = 0.0;
		}
		self.fTempPerm2 = 0.0;
		for l15 in 0..2 {
			self.fRec12[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec13[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fVec7[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec15[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec16[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec17[l20 as usize] = 0.0;
		}
		self.iTempPerm3 = 0;
		self.fTempPerm4 = 0.0;
		for l21 in 0..2 {
			self.fVec8[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec9[l22 as usize] = 0.0;
		}
		self.fTempPerm5 = 0.0;
		for l23 in 0..2 {
			self.fRec18[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec19[l24 as usize] = 0.0;
		}
		self.fTempPerm6 = 0.0;
		for l25 in 0..2 {
			self.iRec20[l25 as usize] = 0;
		}
		self.fTempPerm7 = 0.0;
		self.fTempPerm8 = 0.0;
		for l26 in 0..2 {
			self.iRec21[l26 as usize] = 0;
		}
		self.iTempPerm9 = 0;
		self.fTempPerm10 = 0.0;
		for l27 in 0..2 {
			self.fVec10[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec22[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec23[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.iRec14[l30 as usize] = 0;
		}
		self.fTempPerm11 = 0.0;
		self.fTempPerm12 = 0.0;
		self.fTempPerm13 = 0.0;
		for l31 in 0..2 {
			self.fVec11[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec11[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec10[l33 as usize] = 0.0;
		}
		self.fTempPerm14 = 0.0;
		self.iTempPerm15 = 0;
		self.fTempPerm16 = 0.0;
		for l36 in 0..2 {
			self.fRec27[l36 as usize] = 0.0;
		}
		self.fTempPerm17 = 0.0;
		for l37 in 0..2 {
			self.fRec28[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.iRec30[l38 as usize] = 0;
		}
		self.iTempPerm18 = 0;
		for l39 in 0..2 {
			self.fRec29[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec31[l40 as usize] = 0.0;
		}
		self.fTempPerm19 = 0.0;
		for l41 in 0..2 {
			self.fRec34[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec33[l42 as usize] = 0.0;
		}
		self.fTempPerm20 = 0.0;
		for l43 in 0..2 {
			self.fVec13[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec32[l44 as usize] = 0.0;
		}
		self.fTempPerm21 = 0.0;
		for l45 in 0..2 {
			self.fRec35[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec14[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec36[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec37[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec38[l49 as usize] = 0.0;
		}
		self.fTempPerm22 = 0.0;
		for l50 in 0..2 {
			self.fRec25[l50 as usize] = 0.0;
		}
		self.iTempPerm23 = 0;
		self.iTempPerm24 = 0;
		for l51 in 0..2 {
			self.iRec24[l51 as usize] = 0;
		}
		self.fTempPerm25 = 0.0;
		self.iTempPerm26 = 0;
		self.iTempPerm27 = 0;
		for l52 in 0..2 {
			self.iRec39[l52 as usize] = 0;
		}
		self.fTempPerm28 = 0.0;
		for l53 in 0..2 {
			self.fRec40[l53 as usize] = 0.0;
		}
		self.fTempPerm29 = 0.0;
		self.fTempPerm30 = 0.0;
		self.fTempPerm31 = 0.0;
		for l54 in 0..2 {
			self.fVec15[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec9[l55 as usize] = 0.0;
		}
		self.fTempPerm32 = 0.0;
		for l56 in 0..2 {
			self.fVec16[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec8[l57 as usize] = 0.0;
		}
		self.fTempPerm33 = 0.0;
		self.fTempPerm34 = 0.0;
		for l58 in 0..2 {
			self.fRec44[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec43[l59 as usize] = 0.0;
		}
		self.fTempPerm35 = 0.0;
		for l60 in 0..2 {
			self.fVec17[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec42[l61 as usize] = 0.0;
		}
		self.fTempPerm36 = 0.0;
		self.fTempPerm37 = 0.0;
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		for l62 in 0..2 {
			self.fVec18[l62 as usize] = 0.0;
		}
		self.fTempPerm40 = 0.0;
		for l63 in 0..2 {
			self.fRec46[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec47[l64 as usize] = 0.0;
		}
		self.fTempPerm41 = 0.0;
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		for l65 in 0..2 {
			self.fRec49[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec48[l66 as usize] = 0.0;
		}
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		for l67 in 0..2 {
			self.fRec51[l67 as usize] = 0.0;
		}
		self.fTempPerm48 = 0.0;
		for l68 in 0..2 {
			self.fRec50[l68 as usize] = 0.0;
		}
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		for l69 in 0..2 {
			self.fRec52[l69 as usize] = 0.0;
		}
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		self.fTempPerm54 = 0.0;
		self.fTempPerm55 = 0.0;
		for l70 in 0..3 {
			self.fRec53[l70 as usize] = 0.0;
		}
		self.fTempPerm56 = 0.0;
		self.fTempPerm57 = 0.0;
		self.fTempPerm58 = 0.0;
		self.fTempPerm59 = 0.0;
		for l71 in 0..2 {
			self.fVec19[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec45[l72 as usize] = 0.0;
		}
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		for l73 in 0..2 {
			self.fRec54[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec55[l74 as usize] = 0.0;
		}
		self.fTempPerm62 = 0.0;
		for l75 in 0..2 {
			self.fRec56[l75 as usize] = 0.0;
		}
		self.iTempPerm63 = 0;
		self.fTempPerm64 = 0.0;
		self.fTempPerm65 = 0.0;
		self.fTempPerm66 = 0.0;
		self.fTempPerm67 = 0.0;
		for l76 in 0..2 {
			self.fVec20[l76 as usize] = 0.0;
		}
		self.fTempPerm68 = 0.0;
		for l77 in 0..2 {
			self.fRec58[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec59[l78 as usize] = 0.0;
		}
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		for l79 in 0..2 {
			self.fVec21[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec57[l80 as usize] = 0.0;
		}
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
		self.fTempPerm86 = 0.0;
		self.fTempPerm87 = 0.0;
		self.fTempPerm88 = 0.0;
		self.fTempPerm89 = 0.0;
		self.fTempPerm90 = 0.0;
		self.fTempPerm91 = 0.0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		self.fTempPerm94 = 0.0;
		for l81 in 0..2 {
			self.fVec22[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec41[l82 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		let mut fConst1: F32 = 1.0 / F32::tan(18849.557 / fConst0);
		self.fConst2 = 1.0 - fConst1;
		self.fConst3 = 1.0 - 44.1 / fConst0;
		self.fConst4 = 1.0 / fConst0;
		self.fConst5 = 1.0 / (fConst1 + 1.0);
		self.fConst6 = 1.0 / F32::tan(942.4778 / fConst0);
		self.fConst7 = 1.0 - self.fConst6;
		self.fConst8 = 1.0 / (self.fConst6 + 1.0);
		self.fConst9 = 6.5 / fConst0;
		let mut fConst10: F32 = 376.99112 / fConst0;
		self.fConst11 = F32::exp(-fConst10);
		let mut fConst12: F32 = 1.0 - self.fConst11;
		self.fConst13 = 4.656613e-10 * F32::sqrt(3.0 * ((self.fConst11 + 1.0) / fConst12));
		self.fConst14 = 3.0 * fConst12;
		self.fConst15 = 3.1415927 / fConst0;
		self.fConst16 = 1.0 / F32::tan(3.1415927 * (F32::min(2e+03, 0.4 * fConst0) / fConst0));
		self.fConst17 = 1.0 - self.fConst16;
		self.fConst18 = 1.0 / (self.fConst16 + 1.0);
		let mut fConst19: F32 = 1.0 / F32::tan(1570.7964 / fConst0);
		self.fConst20 = 1.0 - fConst19;
		self.fConst21 = 5e+02 / fConst0;
		self.fConst22 = 1.0 / (fConst19 + 1.0);
		self.fConst23 = 4.41e+04 / fConst0;
		self.fConst24 = 5.0 / fConst0;
		self.fConst25 = 1.41 / fConst0;
		self.fConst26 = 1e+02 / fConst0;
		self.fConst27 = 125.0 / fConst0;
		self.fConst28 = 0.45 * fConst0;
		self.fConst29 = 666.6667 / fConst0;
		self.fConst30 = 15707.963 / fConst0;
		self.fConst31 = F32::tanh(0.12);
		let mut fConst32: F32 = 1.0 / F32::tan(fConst10);
		self.fConst33 = 1.0 - fConst32;
		self.fConst34 = 1.0 / (fConst32 + 1.0);
		self.fConst35 = 0.6666667 / (1.0 - MixDsp_faustpower2_f(self.fConst31));
		self.fConst36 = F32::tanh(-0.08);
		let mut fConst37: F32 = 1.0 / F32::tan(12566.371 / fConst0);
		self.fConst38 = 1.0 - fConst37;
		self.fConst39 = 1.0 / (fConst37 + 1.0);
		self.fConst40 = 0.8 / (1.0 - MixDsp_faustpower2_f(self.fConst36));
		self.fConst41 = F32::tanh(0.05);
		self.fConst42 = 1.1111112 / (1.0 - MixDsp_faustpower2_f(self.fConst41));
		self.fConst43 = F32::tanh(0.08);
		self.fConst44 = 0.6 / (1.0 - MixDsp_faustpower2_f(self.fConst43));
		let mut fConst45: F32 = 15.707963 / fConst0;
		self.fConst46 = 1.0 - fConst45;
		self.fConst47 = 1.0 / (fConst45 + 1.0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		MixDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin-mix");
		ui_interface.declare(None, "0.5", "");
		ui_interface.open_horizontal_box("Vintage");
		ui_interface.declare(Some(ParamIndex(0)), "1", "");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(0), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Perform");
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "st");
		ui_interface.add_horizontal_slider("Global Pitch", ParamIndex(1), 0.0, -12.0, 12.0, 0.01);
		ui_interface.declare(Some(ParamIndex(2)), "5", "");
		ui_interface.declare(Some(ParamIndex(2)), "style", "menu{'2 Semitones':0;'Octave':1;'Cutoff 50%':2;'Resonance 50%':3;'Bandpass 50%':4}");
		ui_interface.add_num_entry("Bend Mode", ParamIndex(2), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "13", "");
		ui_interface.open_horizontal_box("Settings");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("Drive Grit", ParamIndex(3), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.add_horizontal_slider("Vintage Noise", ParamIndex(4), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(5)), "2", "");
		ui_interface.add_horizontal_slider("Heat", ParamIndex(5), 1.0, 0.0, 2.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "4", "");
		ui_interface.open_horizontal_box("Noise");
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.declare(Some(ParamIndex(6)), "style", "menu{'White':0;'Clocked':1;'LFSR Long':2;'LFSR Short':3;'Crackle':4}");
		ui_interface.add_num_entry("Type", ParamIndex(6), 1.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(7)), "2", "");
		ui_interface.declare(Some(ParamIndex(7)), "unit", "oct");
		ui_interface.add_horizontal_slider("Clock Ratio", ParamIndex(7), 0.0, -4.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "3", "");
		ui_interface.add_horizontal_slider("Bits", ParamIndex(8), 8.0, 1.0, 16.0, 1.0);
		ui_interface.declare(Some(ParamIndex(9)), "4", "");
		ui_interface.add_horizontal_slider("Crackle Density", ParamIndex(9), 0.3, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(10)), "5", "");
		ui_interface.declare(Some(ParamIndex(10)), "unit", "dB");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(10), 0.0, -24.0, 24.0, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "5", "");
		ui_interface.open_horizontal_box("Transient");
		ui_interface.declare(Some(ParamIndex(11)), "1", "");
		ui_interface.declare(Some(ParamIndex(11)), "style", "menu{'Click':0;'Burst':1;'Zap':2;'Metal':3;'Chiff':4;'Jade 01':5;'Jade 02':6;'Jade 03':7;'Jade 04':8;'Jade 05':9;'Jade 06':10;'Jade 07':11;'Jade 08':12;'Jade 09':13;'Jade 10':14;'Jade 11':15;'Jade 12':16;'Jade 13':17;'Jade 14':18;'Jade 15':19;'Jade 16':20;'Jade 17':21;'Jade 18':22;'Jade 19':23;'Jade 20':24;'Jade 21':25;'Jade 22':26;'Jade 23':27;'Jade 24':28;'Jade 25':29;'Jade 26':30;'Jade 27':31;'Jade 28':32;'Jade 29':33;'Jade 30':34;'Jade 31':35;'Jade 32':36;'Jade 33':37;'Jade 34':38;'Jade 35':39;'Jade 36':40;'Jade 37':41;'Jade 38':42;'Jade 39':43;'Jade 40':44;'Jade 41':45;'Jade 42':46;'Jade 43':47;'Jade 44':48;'Jade 45':49;'Jade 46':50;'Jade 47':51;'Jade 48':52;'Jade 49':53;'Jade 50':54;'Jade 51':55;'Jade 52':56;'Jade 53':57;'Jade 54':58;'Jade 55':59;'Jade 56':60;'Jade 57':61;'Jade 58':62;'Jade 59':63;'Jade 60':64;'Jade 61':65;'Jade 62':66;'Jade 63':67;'Jade 64':68;'Jade 65':69;'Jade 66':70;'Jade 67':71;'Jade 68':72;'Jade 69':73;'Jade 70':74;'Jade 71':75;'Jade 72':76;'Jade 73':77;'Jade 74':78;'Jade 75':79;'Jade 76':80;'Jade 77':81;'Jade 78':82;'Jade 79':83;'Jade 80':84;'Jade 81':85;'Jade 82':86;'Jade 83':87;'Jade 84':88;'Jade 85':89;'Jade 86':90;'Jade 87':91;'Jade 88':92;'Jade 89':93;'Jade 90':94;'Jade 91':95;'Jade 92':96;'Jade 93':97;'Jade 94':98;'Jade 95':99;'Mech 01':100;'Mech 02':101;'Mech 03':102;'Mech 04':103;'Mech 05':104;'Mech 06':105;'Mech 07':106;'Mech 08':107;'Mech 09':108;'Mech 10':109;'Mech 11':110;'Mech 12':111;'Mech 13':112;'Mech 14':113;'Mech 15':114;'Mech 16':115}");
		ui_interface.add_num_entry("Type", ParamIndex(11), 0.0, 0.0, 115.0, 1.0);
		ui_interface.declare(Some(ParamIndex(12)), "2", "");
		ui_interface.declare(Some(ParamIndex(12)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(12), 6e+01, 5.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(13)), "3", "");
		ui_interface.add_horizontal_slider("Level", ParamIndex(13), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(14)), "4", "");
		ui_interface.add_check_button("Post-Filter", ParamIndex(14));
		ui_interface.declare(Some(ParamIndex(15)), "5", "");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "st");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(15), 0.0, -24.0, 24.0, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(16)), "1", "");
		ui_interface.add_vertical_slider("Osc 1", ParamIndex(16), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(17)), "2", "");
		ui_interface.add_vertical_slider("Osc 2", ParamIndex(17), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(18)), "3", "");
		ui_interface.add_vertical_slider("Noise", ParamIndex(18), 0.15, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "7", "");
		ui_interface.open_horizontal_box("Drive");
		ui_interface.declare(Some(ParamIndex(19)), "2", "");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(19), 0.1, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(20)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(20)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(20), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("freq", ParamIndex(21), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_button("gate", ParamIndex(22));
		ui_interface.add_horizontal_slider("glide", ParamIndex(23), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("ramp", ParamIndex(24), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("slot", ParamIndex(25), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			22 => Some(self.fButton0),
			14 => Some(self.fCheckbox0),
			6 => Some(self.fEntry0),
			11 => Some(self.fEntry1),
			2 => Some(self.fEntry2),
			24 => Some(self.fHslider0),
			10 => Some(self.fHslider1),
			20 => Some(self.fHslider10),
			1 => Some(self.fHslider11),
			23 => Some(self.fHslider12),
			21 => Some(self.fHslider13),
			7 => Some(self.fHslider14),
			8 => Some(self.fHslider15),
			15 => Some(self.fHslider16),
			5 => Some(self.fHslider17),
			9 => Some(self.fHslider2),
			19 => Some(self.fHslider3),
			3 => Some(self.fHslider4),
			13 => Some(self.fHslider5),
			12 => Some(self.fHslider6),
			0 => Some(self.fHslider7),
			25 => Some(self.fHslider8),
			4 => Some(self.fHslider9),
			18 => Some(self.fVslider0),
			17 => Some(self.fVslider1),
			16 => Some(self.fVslider2),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			22 => { self.fButton0 = value }
			14 => { self.fCheckbox0 = value }
			6 => { self.fEntry0 = value }
			11 => { self.fEntry1 = value }
			2 => { self.fEntry2 = value }
			24 => { self.fHslider0 = value }
			10 => { self.fHslider1 = value }
			20 => { self.fHslider10 = value }
			1 => { self.fHslider11 = value }
			23 => { self.fHslider12 = value }
			21 => { self.fHslider13 = value }
			7 => { self.fHslider14 = value }
			8 => { self.fHslider15 = value }
			15 => { self.fHslider16 = value }
			5 => { self.fHslider17 = value }
			9 => { self.fHslider2 = value }
			19 => { self.fHslider3 = value }
			3 => { self.fHslider4 = value }
			13 => { self.fHslider5 = value }
			12 => { self.fHslider6 = value }
			0 => { self.fHslider7 = value }
			25 => { self.fHslider8 = value }
			4 => { self.fHslider9 = value }
			18 => { self.fVslider0 = value }
			17 => { self.fVslider1 = value }
			16 => { self.fVslider2 = value }
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
		let (outputs0, outputs1, outputs2) = if let [outputs0, outputs1, outputs2, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = self.fVslider0;
		let mut fSlow2: F32 = self.fHslider0;
		let mut iSlow3: i32 = (fSlow1 != 0.0) as i32;
		let mut fSlow4: F32 = (iSlow3) as F32;
		let mut fSlow5: F32 = MixDsp_faustpower2_f(fSlow4);
		let mut fSlow6: F32 = self.fVslider1;
		let mut fSlow7: F32 = ((fSlow6 != 0.0) as i32) as u32 as F32;
		let mut fSlow8: F32 = MixDsp_faustpower2_f(fSlow7);
		let mut fSlow9: F32 = self.fVslider2;
		let mut fSlow10: F32 = ((fSlow9 != 0.0) as i32) as u32 as F32;
		let mut fSlow11: F32 = MixDsp_faustpower2_f(fSlow10);
		let mut iSlow12: i32 = (fSlow4 != 0.0) as i32;
		let mut fSlow13: F32 = self.fHslider1;
		let mut fTemp4: F32 = self.fTempPerm0;
		let mut fTemp5: F32 = self.fTempPerm1;
		let mut fSlow14: F32 = self.fEntry0;
		let mut iSlow15: i32 = (fSlow14 == 4.0) as i32;
		let mut fSlow16: F32 = (iSlow15) as F32;
		let mut iSlow17: i32 = (fSlow16 != 0.0) as i32;
		let mut fSlow18: F32 = self.fHslider2;
		let mut fTemp6: F32 = self.fTempPerm2;
		let mut iSlow19: i32 = (fSlow14 == 1.0) as i32;
		let mut fSlow20: F32 = (iSlow19) as F32;
		let mut iSlow21: i32 = (fSlow20 != 0.0) as i32;
		let mut iSlow22: i32 = (fSlow14 == 0.0) as i32;
		let mut fSlow23: F32 = (iSlow22) as F32;
		let mut iSlow24: i32 = (fSlow23 != 0.0) as i32;
		let mut fSlow25: F32 = self.fHslider3;
		let mut fSlow26: F32 = self.fHslider4;
		let mut iTemp39: i32 = self.iTempPerm3;
		let mut fTemp40: F32 = self.fTempPerm4;
		let mut fSlow27: F32 = self.fHslider5;
		let mut iSlow28: i32 = (fSlow27 != 0.0) as i32;
		let mut fSlow29: F32 = (iSlow28) as F32;
		let mut iSlow30: i32 = (fSlow29 != 0.0) as i32;
		let mut fSlow31: F32 = self.fHslider6;
		let mut fTemp41: F32 = self.fTempPerm5;
		let mut fTemp42: F32 = self.fTempPerm6;
		let mut fTemp43: F32 = self.fTempPerm7;
		let mut fTemp44: F32 = self.fTempPerm8;
		let mut iTemp45: i32 = self.iTempPerm9;
		let mut fTemp46: F32 = self.fTempPerm10;
		let mut fSlow32: F32 = self.fEntry1;
		let mut fSlow33: F32 = F32::min(fSlow32, 4.0);
		let mut iSlow34: i32 = (fSlow33 == 4.0) as i32;
		let mut fSlow35: F32 = (iSlow34) as F32;
		let mut iSlow36: i32 = (fSlow35 != 0.0) as i32;
		let mut iSlow37: i32 = (fSlow32 >= 5.0) as i32;
		let mut iSlow38: i32 = (iSlow37 == 0) as i32;
		let mut fSlow39: F32 = (iSlow38) as F32;
		let mut iSlow40: i32 = (fSlow39 != 0.0) as i32;
		let mut iSlow41: i32 = (fSlow33 == 1.0) as i32;
		let mut fSlow42: F32 = (iSlow41) as F32;
		let mut iSlow43: i32 = (fSlow42 != 0.0) as i32;
		let mut fSlow44: F32 = self.fHslider7;
		let mut fTemp50: F32 = self.fTempPerm11;
		let mut fTemp51: F32 = self.fTempPerm12;
		let mut fTemp52: F32 = self.fTempPerm13;
		let mut fTemp53: F32 = self.fTempPerm14;
		let mut fSlow45: F32 = ((iSlow15 != 0) as i32) as u32 as F32;
		let mut iSlow46: i32 = (fSlow14 == 3.0) as i32;
		let mut fSlow47: F32 = (iSlow46) as F32;
		let mut iSlow48: i32 = (fSlow47 != 0.0) as i32;
		let mut iSlow49: i32 = (fSlow14 == 2.0) as i32;
		let mut fSlow50: F32 = (iSlow49) as F32;
		let mut iSlow51: i32 = (fSlow50 != 0.0) as i32;
		let mut iSlow52: i32 = (fSlow33 == 3.0) as i32;
		let mut fSlow53: F32 = (iSlow52) as F32;
		let mut iSlow54: i32 = (fSlow53 != 0.0) as i32;
		let mut iSlow55: i32 = (fSlow33 == 2.0) as i32;
		let mut fSlow56: F32 = (iSlow55) as F32;
		let mut iSlow57: i32 = (fSlow56 != 0.0) as i32;
		let mut iTemp54: i32 = self.iTempPerm15;
		let mut fTemp55: F32 = self.fTempPerm16;
		let mut fSlow58: F32 = self.fHslider8;
		let mut fSlow59: F32 = 12.9898 * fSlow58;
		let mut fSlow60: F32 = 43758.547 * F32::sin(fSlow59 + 234.699);
		let mut fSlow61: F32 = 43758.547 * F32::sin(fSlow59 + 7979.766);
		let mut fSlow62: F32 = 2.0 * (fSlow61 - F32::floor(fSlow61)) + -1.0;
		let mut fSlow63: F32 = self.fConst4 * (0.1 * F32::max(-1.0, F32::min(1.0, 0.70710677 * (fSlow62 + 2.0 * (fSlow60 - F32::floor(fSlow60)) + -1.0))) + 0.45);
		let mut fTemp56: F32 = self.fTempPerm17;
		let mut iSlow64: i32 = (409.0 * fSlow58 + 4e+03) as i32;
		let mut iTemp57: i32 = self.iTempPerm18;
		let mut fSlow65: F32 = self.fHslider9;
		let mut fSlow66: F32 = 43758.547 * F32::sin(fSlow59 + 156.466);
		let mut fSlow67: F32 = 0.15 * F32::max(-1.0, F32::min(1.0, 1.2803688 * (0.6 * fSlow62 + 0.5 * (2.0 * (fSlow66 - F32::floor(fSlow66)) + -1.0)))) + 0.25;
		let mut fSlow68: F32 = 1.0 / F32::tan(self.fConst15 * F32::max(1.0, fSlow67));
		let mut fSlow69: F32 = 1.0 - fSlow68;
		let mut fSlow70: F32 = self.fConst4 * F32::max(fSlow67, 0.0);
		let mut fTemp58: F32 = self.fTempPerm19;
		let mut fTemp59: F32 = self.fTempPerm20;
		let mut fSlow71: F32 = 1.0 / (fSlow68 + 1.0);
		let mut fSlow72: F32 = 43758.547 * F32::sin(fSlow59 + 78.233);
		let mut fSlow73: F32 = 43758.547 * F32::sin(fSlow59 + 7901.533);
		let mut fSlow74: F32 = 8.0 * F32::max(-1.0, F32::min(1.0, 1.0976425 * (0.7 * (2.0 * (fSlow73 - F32::floor(fSlow73)) + -1.0) + 0.3 * fSlow62 + 0.5 * (2.0 * (fSlow72 - F32::floor(fSlow72)) + -1.0))));
		let mut fTemp60: F32 = self.fTempPerm21;
		let mut fSlow75: F32 = self.fHslider10;
		let mut fSlow76: F32 = self.fEntry2;
		let mut fSlow77: F32 = 2.0 * ((fSlow76 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow76 == 1.0) as i32) as u32 as F32;
		let mut fSlow78: F32 = self.fHslider11;
		let mut fSlow79: F32 = 0.33333334 * F32::max(1e-05, self.fHslider12);
		let mut iSlow80: i32 = (F32::abs(fSlow79) < 1.1920929e-07) as i32;
		let mut fSlow81: F32 = if iSlow80 != 0 {0.0} else {F32::exp(-(self.fConst4 / if iSlow80 != 0 {1.0} else {fSlow79}))};
		let mut fSlow82: F32 = F32::log(self.fHslider13, std::f32::consts::E) * (1.0 - fSlow81);
		let mut fSlow83: F32 = F32::powf(2.0, self.fHslider14);
		let mut fTemp65: F32 = self.fTempPerm22;
		let mut iTemp66: i32 = self.iTempPerm23;
		let mut iTemp67: i32 = self.iTempPerm24;
		let mut fTemp68: F32 = self.fTempPerm25;
		let mut fSlow84: F32 = ((iSlow46 != 0) as i32) as u32 as F32;
		let mut iTemp69: i32 = self.iTempPerm26;
		let mut iTemp70: i32 = self.iTempPerm27;
		let mut fTemp71: F32 = self.fTempPerm28;
		let mut fSlow85: F32 = ((iSlow49 != 0) as i32) as u32 as F32;
		let mut fSlow86: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, self.fHslider15)) + -1.0));
		let mut fSlow87: F32 = fSlow20 / fSlow86;
		let mut fTemp72: F32 = self.fTempPerm29;
		let mut fSlow88: F32 = ((iSlow19 != 0) as i32) as u32 as F32;
		let mut fSlow89: F32 = 4.656613e-10 * fSlow23;
		let mut fTemp73: F32 = self.fTempPerm30;
		let mut fSlow90: F32 = ((iSlow22 != 0) as i32) as u32 as F32;
		let mut fTemp74: F32 = self.fTempPerm31;
		let mut fTemp75: F32 = self.fTempPerm32;
		let mut fTemp76: F32 = self.fTempPerm33;
		let mut fSlow91: F32 = fSlow4 * ((iSlow3 != 0) as i32) as u32 as F32;
		let mut fTemp78: F32 = self.fTempPerm34;
		let mut fTemp79: F32 = self.fTempPerm35;
		let mut fTemp80: F32 = self.fTempPerm36;
		let mut fTemp81: F32 = self.fTempPerm37;
		let mut fTemp82: F32 = self.fTempPerm38;
		let mut fTemp83: F32 = self.fTempPerm39;
		let mut iSlow92: i32 = (iSlow37 == 1) as i32;
		let mut fSlow93: F32 = (iSlow92) as F32;
		let mut iSlow94: i32 = (fSlow93 != 0.0) as i32;
		let mut iSlow95: i32 = (F32::max(0.0, fSlow32 + -5.0)) as i32;
		let mut iSlow96: i32 = (fSlow33 == 0.0) as i32;
		let mut fSlow97: F32 = (iSlow96) as F32;
		let mut iSlow98: i32 = (fSlow97 != 0.0) as i32;
		let mut fSlow99: F32 = self.fHslider16;
		let mut fTemp84: F32 = self.fTempPerm40;
		let mut fTemp85: F32 = self.fTempPerm41;
		let mut fTemp86: F32 = self.fTempPerm42;
		let mut fTemp87: F32 = self.fTempPerm43;
		let mut fSlow100: F32 = ((iSlow92 != 0) as i32) as u32 as F32;
		let mut fTemp88: F32 = self.fTempPerm44;
		let mut fTemp89: F32 = self.fTempPerm45;
		let mut fSlow101: F32 = 0.03125 * fSlow35;
		let mut fTemp90: F32 = self.fTempPerm46;
		let mut fSlow102: F32 = ((iSlow34 != 0) as i32) as u32 as F32;
		let mut fTemp91: F32 = self.fTempPerm47;
		let mut fTemp92: F32 = self.fTempPerm48;
		let mut fTemp93: F32 = self.fTempPerm49;
		let mut fSlow103: F32 = ((iSlow52 != 0) as i32) as u32 as F32;
		let mut fTemp94: F32 = self.fTempPerm50;
		let mut fTemp95: F32 = self.fTempPerm51;
		let mut fSlow104: F32 = ((iSlow55 != 0) as i32) as u32 as F32;
		let mut fTemp96: F32 = self.fTempPerm52;
		let mut fTemp97: F32 = self.fTempPerm53;
		let mut fTemp98: F32 = self.fTempPerm54;
		let mut fTemp99: F32 = self.fTempPerm55;
		let mut fTemp100: F32 = self.fTempPerm56;
		let mut fSlow105: F32 = ((iSlow41 != 0) as i32) as u32 as F32;
		let mut fTemp101: F32 = self.fTempPerm57;
		let mut fSlow106: F32 = ((iSlow96 != 0) as i32) as u32 as F32;
		let mut fTemp102: F32 = self.fTempPerm58;
		let mut fSlow107: F32 = ((iSlow38 != 0) as i32) as u32 as F32;
		let mut fTemp103: F32 = self.fTempPerm59;
		let mut fTemp104: F32 = self.fTempPerm60;
		let mut fTemp105: F32 = self.fTempPerm61;
		let mut fSlow108: F32 = self.fCheckbox0;
		let mut fSlow109: F32 = ((iSlow28 != 0) as i32) as u32 as F32;
		let mut fSlow110: F32 = fSlow29 * fSlow109 * (1.0 - fSlow108);
		let mut fTemp108: F32 = self.fTempPerm62;
		let mut fSlow111: F32 = self.fHslider17;
		let mut iTemp109: i32 = self.iTempPerm63;
		let mut fTemp110: F32 = self.fTempPerm64;
		let mut fTemp111: F32 = self.fTempPerm65;
		let mut fTemp112: F32 = self.fTempPerm66;
		let mut fTemp113: F32 = self.fTempPerm67;
		let mut fTemp114: F32 = self.fTempPerm68;
		let mut fTemp115: F32 = self.fTempPerm69;
		let mut fTemp116: F32 = self.fTempPerm70;
		let mut fTemp117: F32 = self.fTempPerm71;
		let mut fTemp118: F32 = self.fTempPerm72;
		let mut fTemp119: F32 = self.fTempPerm73;
		let mut fTemp120: F32 = self.fTempPerm74;
		let mut fTemp121: F32 = self.fTempPerm75;
		let mut fTemp122: F32 = self.fTempPerm76;
		let mut fTemp123: F32 = self.fTempPerm77;
		let mut fTemp124: F32 = self.fTempPerm78;
		let mut fTemp125: F32 = self.fTempPerm79;
		let mut fTemp126: F32 = self.fTempPerm80;
		let mut fTemp127: F32 = self.fTempPerm81;
		let mut fTemp128: F32 = self.fTempPerm82;
		let mut fTemp129: F32 = self.fTempPerm83;
		let mut fTemp130: F32 = self.fTempPerm84;
		let mut fTemp131: F32 = self.fTempPerm85;
		let mut fTemp132: F32 = self.fTempPerm86;
		let mut fTemp134: F32 = self.fTempPerm87;
		let mut fTemp135: F32 = self.fTempPerm88;
		let mut fTemp136: F32 = self.fTempPerm89;
		let mut fTemp137: F32 = self.fTempPerm90;
		let mut fTemp138: F32 = self.fTempPerm91;
		let mut fTemp139: F32 = self.fTempPerm92;
		let mut fTemp140: F32 = self.fTempPerm93;
		let mut fTemp141: F32 = self.fTempPerm94;
		let mut fSlow112: F32 = fSlow108 * fSlow29 * fSlow109;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2);
		for ((((input0, input1), output0), output1), output2) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			self.iVec1[0] = 1;
			let mut iTemp0: i32 = (fSlow0 > self.fVec0[1]) as i32;
			self.fVec2[0] = fSlow1;
			let mut fTemp1: F32 = if ((fSlow1 != self.fVec2[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec0[1] + -1.0};
			self.fRec0[0] = fTemp1;
			self.fRec1[0] = if iTemp0 != 0 {fSlow1} else {if (fTemp1 > 0.0) as i32 != 0 {self.fRec1[1] + (fSlow1 - self.fRec1[1]) / F32::max(1.0, fTemp1)} else {fSlow1}};
			self.fVec3[0] = fSlow6;
			let mut fTemp2: F32 = if ((fSlow6 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec2[1] + -1.0};
			self.fRec2[0] = fTemp2;
			self.fRec3[0] = if iTemp0 != 0 {fSlow6} else {if (fTemp2 > 0.0) as i32 != 0 {self.fRec3[1] + (fSlow6 - self.fRec3[1]) / F32::max(1.0, fTemp2)} else {fSlow6}};
			self.fVec4[0] = fSlow9;
			let mut fTemp3: F32 = if ((fSlow9 != self.fVec4[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec4[1] + -1.0};
			self.fRec4[0] = fTemp3;
			self.fRec5[0] = if iTemp0 != 0 {fSlow9} else {if (fTemp3 > 0.0) as i32 != 0 {self.fRec5[1] + (fSlow9 - self.fRec5[1]) / F32::max(1.0, fTemp3)} else {fSlow9}};
			if iSlow12 != 0 {
				self.fVec5[0] = fSlow13;
				fTemp4 = if ((fSlow13 != self.fVec5[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec6[1] + -1.0};
				self.fRec6[0] = fTemp4;
				self.fRec7[0] = if iTemp0 != 0 {fSlow13} else {if (fTemp4 > 0.0) as i32 != 0 {self.fRec7[1] + (fSlow13 - self.fRec7[1]) / F32::max(1.0, fTemp4)} else {fSlow13}};
				fTemp5 = 0.025 * self.fRec7[0];
			}
				if iSlow12 & iSlow17 != 0 {
					self.fVec6[0] = fSlow18;
					fTemp6 = if ((fSlow18 != self.fVec6[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec12[1] + -1.0};
					self.fRec12[0] = fTemp6;
					self.fRec13[0] = if iTemp0 != 0 {fSlow18} else {if (fTemp6 > 0.0) as i32 != 0 {self.fRec13[1] + (fSlow18 - self.fRec13[1]) / F32::max(1.0, fTemp6)} else {fSlow18}};
				}
					self.fVec7[0] = fSlow25;
					let mut fTemp7: F32 = if ((fSlow25 != self.fVec7[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec15[1] + -1.0};
					self.fRec15[0] = fTemp7;
					self.fRec16[0] = if iTemp0 != 0 {fSlow25} else {if (fTemp7 > 0.0) as i32 != 0 {self.fRec16[1] + (fSlow25 - self.fRec16[1]) / F32::max(1.0, fTemp7)} else {fSlow25}};
					let mut fTemp8: F32 = F32::max(0.0, F32::min(1.0, self.fRec16[0]));
					let mut fTemp9: F32 = 4.0 * fTemp8;
					let mut fTemp10: F32 = F32::max(0.0, F32::min(2.0, F32::floor(fTemp9 + -0.5)));
					let mut fTemp11: F32 = fTemp10 + 1.0;
					let mut iTemp12: i32 = (fTemp11 == 1.0) as i32;
					let mut fTemp13: F32 = (iTemp12) as F32;
					let mut iTemp14: i32 = (fTemp13 != 0.0) as i32;
					let mut iTemp15: i32 = (fTemp11 == 0.0) as i32;
					let mut fTemp16: F32 = (iTemp15) as F32;
					let mut iTemp17: i32 = (fTemp16 != 0.0) as i32;
					let mut iTemp18: i32 = (fTemp11 == 2.0) as i32;
					let mut fTemp19: F32 = (iTemp18) as F32;
					let mut iTemp20: i32 = (fTemp19 != 0.0) as i32;
					let mut iTemp21: i32 = (fTemp11 == 3.0) as i32;
					let mut fTemp22: F32 = (iTemp21) as F32;
					let mut iTemp23: i32 = (fTemp22 != 0.0) as i32;
					let mut iTemp24: i32 = (fTemp10 == 1.0) as i32;
					let mut fTemp25: F32 = (iTemp24) as F32;
					let mut iTemp26: i32 = (fTemp25 != 0.0) as i32;
					let mut iTemp27: i32 = (fTemp10 == 0.0) as i32;
					let mut fTemp28: F32 = (iTemp27) as F32;
					let mut iTemp29: i32 = (fTemp28 != 0.0) as i32;
					let mut iTemp30: i32 = (fTemp10 == 2.0) as i32;
					let mut fTemp31: F32 = (iTemp30) as F32;
					let mut iTemp32: i32 = (fTemp31 != 0.0) as i32;
					let mut iTemp33: i32 = (fTemp10 == 3.0) as i32;
					let mut fTemp34: F32 = (iTemp33) as F32;
					let mut iTemp35: i32 = (fTemp34 != 0.0) as i32;
					let mut iTemp36: i32 = i32::wrapping_sub(1, iTemp0);
					let mut fTemp37: F32 = (iTemp36) as F32;
					let mut fTemp38: F32 = 1.0 - self.fConst3 * fTemp37;
					if iTemp35 | (iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | iTemp14)))))) != 0 {
						self.fRec17[0] = fSlow26 * fTemp38 + self.fConst3 * fTemp37 * self.fRec17[1];
						iTemp39 = (self.fRec17[0] > 0.0) as i32;
						fTemp40 = (iTemp39) as F32;
					}
						self.fVec8[0] = fSlow27;
						if iSlow30 != 0 {
							self.fVec9[0] = fSlow31;
							fTemp41 = if ((fSlow31 != self.fVec9[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec18[1] + -1.0};
							self.fRec18[0] = fTemp41;
							self.fRec19[0] = if iTemp0 != 0 {fSlow31} else {if (fTemp41 > 0.0) as i32 != 0 {self.fRec19[1] + (fSlow31 - self.fRec19[1]) / F32::max(1.0, fTemp41)} else {fSlow31}};
							fTemp42 = F32::max(0.005, 0.001 * self.fRec19[0]);
							self.iRec20[0] = i32::wrapping_mul(iTemp36, i32::wrapping_add(self.iRec20[1], 1));
							fTemp43 = (self.iRec20[0]) as F32;
							fTemp44 = self.fConst4 * fTemp43;
							self.iRec21[0] = std::cmp::max(iTemp0, self.iRec21[1]);
							iTemp45 = i32::wrapping_mul(self.iRec21[0], (fTemp44 < (fTemp42 + 0.05)) as i32);
							fTemp46 = (iTemp45) as F32;
						}
							self.fVec10[0] = fSlow44;
							let mut fTemp47: F32 = if ((fSlow44 != self.fVec10[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec22[1] + -1.0};
							self.fRec22[0] = fTemp47;
							self.fRec23[0] = if iTemp0 != 0 {fSlow44} else {if (fTemp47 > 0.0) as i32 != 0 {self.fRec23[1] + (fSlow44 - self.fRec23[1]) / F32::max(1.0, fTemp47)} else {fSlow44}};
							let mut iTemp48: i32 = (F32::abs(self.fRec23[0]) > 0.0001) as i32;
							let mut fTemp49: F32 = (iTemp48) as F32;
							if ((fTemp49 != 0.0) as i32) | ((((fTemp40 != 0.0) as i32) & iTemp35) | ((iSlow30 & (iSlow40 & (iSlow43 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32)))) | ((((fTemp40 != 0.0) as i32) & iTemp32) | ((((fTemp40 != 0.0) as i32) & iTemp29) | ((((fTemp40 != 0.0) as i32) & iTemp26) | ((((fTemp40 != 0.0) as i32) & iTemp23) | ((((fTemp40 != 0.0) as i32) & iTemp20) | ((((fTemp40 != 0.0) as i32) & iTemp17) | ((((fTemp40 != 0.0) as i32) & iTemp14) | ((iSlow12 & iSlow24) | ((iSlow12 & iSlow21) | (iSlow12 & iSlow17))))))))))))) != 0 {
								self.iRec14[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec14[1]), 12345);
								fTemp50 = (self.iRec14[0]) as F32;
							}
								if ((fTemp49 != 0.0) as i32) | ((((fTemp40 != 0.0) as i32) & iTemp35) | ((iSlow30 & (iSlow40 & (iSlow43 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32)))) | ((((fTemp40 != 0.0) as i32) & iTemp32) | ((((fTemp40 != 0.0) as i32) & iTemp29) | ((((fTemp40 != 0.0) as i32) & iTemp26) | ((((fTemp40 != 0.0) as i32) & iTemp23) | ((((fTemp40 != 0.0) as i32) & iTemp20) | ((((fTemp40 != 0.0) as i32) & iTemp17) | ((((fTemp40 != 0.0) as i32) & iTemp14) | ((iSlow12 & iSlow21) | (iSlow12 & iSlow17)))))))))))) != 0 {
									fTemp51 = 4.656613e-10 * fTemp50;
								}
									if iSlow12 & iSlow17 != 0 {
										fTemp52 = fTemp50 * ((F32::abs(fTemp51) > (1.0 - 0.002 * F32::max(0.0, F32::min(1.0, self.fRec13[0])))) as i32) as u32 as F32;
										self.fVec11[0] = fTemp52;
										self.fRec11[0] = self.fConst5 * (1.8626451e-09 * (fTemp52 + self.fVec11[1]) - self.fConst2 * self.fRec11[1]);
										self.fRec10[0] = -(self.fConst8 * (self.fConst7 * self.fRec10[1] - self.fConst6 * (self.fRec11[0] - self.fRec11[1])));
										fTemp53 = fSlow16 * self.fRec10[0];
									}
										if ((fTemp49 != 0.0) as i32) | ((((fTemp40 != 0.0) as i32) & iTemp35) | ((iSlow30 & (iSlow40 & (iSlow57 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow54 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32)))) | ((((fTemp40 != 0.0) as i32) & iTemp32) | ((((fTemp40 != 0.0) as i32) & iTemp29) | ((((fTemp40 != 0.0) as i32) & iTemp26) | ((((fTemp40 != 0.0) as i32) & iTemp23) | ((((fTemp40 != 0.0) as i32) & iTemp20) | ((((fTemp40 != 0.0) as i32) & iTemp17) | ((((fTemp40 != 0.0) as i32) & iTemp14) | ((iSlow12 & iSlow21) | ((iSlow12 & iSlow51) | (iSlow12 & iSlow48)))))))))))))) != 0 {
											iTemp54 = i32::wrapping_sub(1, self.iVec1[1]);
										}
											if (fTemp49 != 0.0) as i32 != 0 {
												fTemp55 = if iTemp54 != 0 {0.0} else {self.fConst9 + self.fRec27[1]};
												self.fRec27[0] = fTemp55 - F32::floor(fTemp55);
												fTemp56 = if iTemp54 != 0 {0.0} else {fSlow63 + self.fRec28[1]};
												self.fRec28[0] = fTemp56 - F32::floor(fTemp56);
												self.iRec30[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec30[1], i32::wrapping_mul(iSlow64, iTemp54))), 12345);
												iTemp57 = i32::wrapping_mul(1664525, self.iRec30[0] ^ (self.iRec30[0] >> 15));
												self.fRec29[0] = self.fConst11 * self.fRec29[1] + self.fConst13 * (i32::wrapping_mul(1013904223, iTemp57 ^ (iTemp57 >> 13))) as F32;
												self.fRec31[0] = fSlow65 * fTemp38 + self.fConst3 * fTemp37 * self.fRec31[1];
												fTemp58 = if iTemp54 != 0 {0.0} else {fSlow70 + self.fRec34[1]};
												self.fRec34[0] = fTemp58 - F32::floor(fTemp58);
												self.fRec33[0] = if (self.fRec34[0] < self.fRec34[1]) as i32 != 0 {fTemp51} else {self.fRec33[1]};
												fTemp59 = F32::floor(32768.0 * self.fRec33[0] + 0.5);
												self.fVec13[0] = fTemp59;
												self.fRec32[0] = fSlow71 * (3.0517578e-05 * (fTemp59 + self.fVec13[1]) - fSlow69 * self.fRec32[1]);
												fTemp60 = fTemp49 * (self.fRec23[0] * (fSlow74 + 2e+01 * self.fRec32[0] + self.fConst14 * self.fRec31[0] * self.fRec29[0]) + MixDsp_faustpower2_f(self.fRec23[0]) * (18.0 * unsafe { ftbl0MixDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec28[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl0MixDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec27[0]) as i32, 65535))) as usize] }));
											}
												self.fRec35[0] = fSlow75 * fTemp38 + self.fConst3 * fTemp37 * self.fRec35[1];
												self.fVec14[0] = fSlow78;
												let mut fTemp61: F32 = if ((fSlow78 != self.fVec14[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec36[1] + -1.0};
												self.fRec36[0] = fTemp61;
												self.fRec37[0] = if iTemp0 != 0 {fSlow78} else {if (fTemp61 > 0.0) as i32 != 0 {self.fRec37[1] + (fSlow78 - self.fRec37[1]) / F32::max(1.0, fTemp61)} else {fSlow78}};
												let mut fTemp62: F32 = F32::exp(0.057762265 * (self.fRec37[0] + fSlow77 * self.fRec35[0] + 0.01 * ((iTemp48 != 0) as i32) as u32 as F32 * fTemp60));
												self.fRec38[0] = fSlow82 + fSlow81 * self.fRec38[1];
												let mut fTemp63: F32 = F32::exp(self.fRec38[0]);
												let mut fTemp64: F32 = fTemp63 * fTemp62;
												if (iSlow12 & iSlow21) | ((iSlow12 & iSlow51) | (iSlow12 & iSlow48)) != 0 {
													fTemp65 = if iTemp54 != 0 {0.0} else {self.fRec25[1] + self.fConst4 * F32::max(fSlow83 * fTemp64, 0.0)};
													self.fRec25[0] = fTemp65 - F32::floor(fTemp65);
													iTemp66 = (self.fRec25[0] < self.fRec25[1]) as i32;
												}
													if iSlow12 & iSlow48 != 0 {
														iTemp67 = if (self.iRec24[1] == 0) as i32 != 0 {1} else {self.iRec24[1]};
														self.iRec24[0] = if iTemp66 != 0 {(iTemp67 >> 1) | (((iTemp67 ^ (iTemp67 >> 6)) & 1) << 14)} else {iTemp67};
														fTemp68 = fSlow47 * if self.iRec24[0] & 1 != 0 {1.0} else {-1.0};
													}
														if iSlow12 & iSlow51 != 0 {
															iTemp69 = if (self.iRec39[1] == 0) as i32 != 0 {1} else {self.iRec39[1]};
															iTemp70 = iTemp69 >> 1;
															self.iRec39[0] = if iTemp66 != 0 {iTemp70 | (((iTemp69 ^ iTemp70) & 1) << 14)} else {iTemp69};
															fTemp71 = fSlow50 * if self.iRec39[0] & 1 != 0 {1.0} else {-1.0};
														}
															if iSlow12 & iSlow21 != 0 {
																self.fRec40[0] = if iTemp66 != 0 {fTemp51} else {self.fRec40[1]};
																fTemp72 = fSlow87 * F32::floor(fSlow86 * self.fRec40[0] + 0.5);
															}
																if iSlow12 & iSlow24 != 0 {
																	fTemp73 = fSlow89 * fTemp50;
																}
																	if iSlow12 != 0 {
																		fTemp74 = fSlow90 * fTemp73 + fSlow88 * fTemp72 + fSlow85 * fTemp71 + fSlow84 * fTemp68 + fSlow45 * fTemp53;
																		self.fVec15[0] = fTemp74;
																		self.fRec9[0] = -(self.fConst18 * (self.fConst17 * self.fRec9[1] - (fTemp74 + self.fVec15[1])));
																		fTemp75 = fTemp74 + self.fRec9[0] * (F32::powf(1e+01, -fTemp5) + -1.0);
																		self.fVec16[0] = fTemp75;
																		self.fRec8[0] = -(self.fConst18 * (self.fConst17 * self.fRec8[1] - self.fConst16 * (fTemp75 - self.fVec16[1])));
																		fTemp76 = fSlow4 * (self.fRec9[0] * (F32::powf(1e+01, -0.025 * self.fRec7[0]) + -1.0) + fTemp74 + self.fRec8[0] * (F32::powf(1e+01, fTemp5) + -1.0));
																	}
																		let mut fTemp77: F32 = F32::powf(1e+01, 0.6 * (fTemp8 + MixDsp_faustpower4_f(fTemp8)));
																		if (((fTemp40 != 0.0) as i32) & iTemp35) | ((((fTemp40 != 0.0) as i32) & iTemp32) | ((((fTemp40 != 0.0) as i32) & iTemp29) | ((((fTemp40 != 0.0) as i32) & iTemp26) | ((((fTemp40 != 0.0) as i32) & iTemp23) | ((((fTemp40 != 0.0) as i32) & iTemp20) | ((((fTemp40 != 0.0) as i32) & iTemp17) | (((fTemp40 != 0.0) as i32) & iTemp14))))))) != 0 {
																			fTemp78 = if iTemp54 != 0 {0.0} else {self.fConst21 + self.fRec44[1]};
																			self.fRec44[0] = fTemp78 - F32::floor(fTemp78);
																			self.fRec43[0] = if (self.fRec44[0] < self.fRec44[1]) as i32 != 0 {fTemp51} else {self.fRec43[1]};
																			fTemp79 = F32::floor(32768.0 * self.fRec43[0] + 0.5);
																			self.fVec17[0] = fTemp79;
																			self.fRec42[0] = self.fConst22 * (3.0517578e-05 * (fTemp79 + self.fVec17[1]) - self.fConst20 * self.fRec42[1]);
																			fTemp80 = 0.3 * self.fRec17[0] * self.fRec42[0] * fTemp40;
																		}
																			if iTemp35 | (iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | iTemp14)))))) != 0 {
																				fTemp81 = ((iTemp39 != 0) as i32) as u32 as F32 * fTemp80;
																			}
																				if iTemp26 | iTemp14 != 0 {
																					fTemp82 = F32::max(-1.0, F32::min(1.0, fTemp81));
																				}
																					if iTemp14 != 0 {
																						fTemp83 = fTemp82 * fTemp13;
																					}
																						if (iSlow30 & (iSlow40 & (iSlow98 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow43 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow57 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow54 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32)))) | (iSlow30 & (iSlow94 & ((fTemp46 != 0.0) as i32))))))) != 0 {
																							self.fVec18[0] = fSlow99;
																							fTemp84 = if ((fSlow99 != self.fVec18[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec46[1] + -1.0};
																							self.fRec46[0] = fTemp84;
																							self.fRec47[0] = if iTemp0 != 0 {fSlow99} else {if (fTemp84 > 0.0) as i32 != 0 {self.fRec47[1] + (fSlow99 - self.fRec47[1]) / F32::max(1.0, fTemp84)} else {fSlow99}};
																							fTemp85 = F32::exp(0.057762265 * self.fRec47[0]);
																						}
																							if (iSlow30 & (iSlow40 & (iSlow98 & ((fTemp46 != 0.0) as i32)))) | (iSlow30 & (iSlow94 & ((fTemp46 != 0.0) as i32))) != 0 {
																								fTemp86 = fTemp43 * fTemp85;
																							}
																								if iSlow30 & (iSlow94 & ((fTemp46 != 0.0) as i32)) != 0 {
																									fTemp87 = fSlow93 * tr_read(iSlow95, self.fConst23 * fTemp86) * F32::min(1.0, F32::max(0.0, 2.5 * ((fTemp42 - fTemp44) / fTemp42)));
																								}
																									if (iSlow30 & (iSlow40 & (iSlow57 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow54 & ((fTemp46 != 0.0) as i32)))) | (iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32))))) != 0 {
																										fTemp88 = fTemp63 * fTemp85 * fTemp62;
																									}
																										if iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32))) != 0 {
																											fTemp89 = if iTemp54 != 0 {0.0} else {self.fRec49[1] + self.fConst4 * F32::max(4.0 * fTemp88, 0.0)};
																											self.fRec49[0] = fTemp89 - F32::floor(fTemp89);
																											self.fRec48[0] = if (self.fRec49[0] < self.fRec49[1]) as i32 != 0 {fTemp51} else {self.fRec48[1]};
																											fTemp90 = fSlow101 * F32::floor(32.0 * self.fRec48[0] + 0.5);
																										}
																											if iSlow30 & (iSlow40 & (iSlow54 & ((fTemp46 != 0.0) as i32))) != 0 {
																												fTemp91 = if iTemp54 != 0 {0.0} else {self.fRec51[1] + self.fConst25 * fTemp88};
																												self.fRec51[0] = fTemp91 - F32::floor(fTemp91);
																												fTemp92 = if iTemp54 != 0 {0.0} else {self.fRec50[1] + self.fConst4 * fTemp88 * (4.0 * F32::exp(-(self.fConst26 * fTemp43)) * unsafe { ftbl0MixDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec51[0]) as i32, 65535))) as usize] } + 3.53)};
																												self.fRec50[0] = fTemp92 - F32::floor(fTemp92);
																												fTemp93 = fSlow53 * unsafe { ftbl0MixDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec50[0]) as i32, 65535))) as usize] };
																											}
																												if iSlow30 & (iSlow40 & (iSlow57 & ((fTemp46 != 0.0) as i32))) != 0 {
																													fTemp94 = if iTemp54 != 0 {0.0} else {self.fRec52[1] + self.fConst4 * fTemp88 * (7.0 * F32::exp(-(self.fConst27 * fTemp43)) + 1.0)};
																													self.fRec52[0] = fTemp94 - F32::floor(fTemp94);
																													fTemp95 = fSlow56 * unsafe { ftbl0MixDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec52[0]) as i32, 65535))) as usize] };
																												}
																													if iSlow30 & (iSlow40 & (iSlow43 & ((fTemp46 != 0.0) as i32))) != 0 {
																														fTemp96 = F32::tan(self.fConst15 * F32::min(8e+02 * fTemp85, self.fConst28));
																														fTemp97 = 1.0 / fTemp96;
																														fTemp98 = (fTemp97 + 1.4142135) / fTemp96 + 1.0;
																														fTemp99 = MixDsp_faustpower2_f(fTemp96);
																														self.fRec53[0] = fTemp51 - (self.fRec53[2] * ((fTemp97 + -1.4142135) / fTemp96 + 1.0) + 2.0 * self.fRec53[1] * (1.0 - 1.0 / fTemp99)) / fTemp98;
																														fTemp100 = fSlow42 * ((self.fRec53[2] + (self.fRec53[0] - 2.0 * self.fRec53[1])) / (fTemp99 * fTemp98));
																													}
																														if iSlow30 & (iSlow40 & (iSlow98 & ((fTemp46 != 0.0) as i32))) != 0 {
																															fTemp101 = fSlow97 * F32::sin(self.fConst30 * fTemp86) * F32::exp(-(self.fConst29 * fTemp43));
																														}
																															if iSlow30 & (iSlow40 & ((fTemp46 != 0.0) as i32)) != 0 {
																																fTemp102 = fSlow39 * (fSlow106 * fTemp101 + fSlow105 * fTemp100 + fSlow104 * fTemp95 + fSlow103 * fTemp93 + fSlow102 * fTemp90) * F32::exp(-(self.fConst24 * (fTemp43 / fTemp42))) * ((fTemp44 < fTemp42) as i32) as u32 as F32;
																															}
																																if iSlow30 & ((fTemp46 != 0.0) as i32) != 0 {
																																	fTemp103 = (self.iRec21[0]) as F32 * (fSlow107 * fTemp102 + fSlow100 * fTemp87);
																																	self.fVec19[0] = fTemp103;
																																	self.fRec45[0] = 0.995 * self.fRec45[1] + fTemp103 - self.fVec19[1];
																																	fTemp104 = self.fRec45[0] * fTemp46;
																																}
																																	if iSlow30 != 0 {
																																		fTemp105 = fSlow29 * fTemp104 * ((iTemp45 != 0) as i32) as u32 as F32;
																																	}
																																		let mut fTemp106: F32 = if ((fSlow27 != self.fVec8[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec54[1] + -1.0};
																																		self.fRec54[0] = fTemp106;
																																		self.fRec55[0] = if iTemp0 != 0 {fSlow27} else {if (fTemp106 > 0.0) as i32 != 0 {self.fRec55[1] + (fSlow27 - self.fRec55[1]) / F32::max(1.0, fTemp106)} else {fSlow27}};
																																		let mut fTemp107: F32 = self.fRec55[0] * fTemp105;
																																		if iTemp35 | (iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | iTemp14)))))) != 0 {
																																			fTemp108 = F32::sqrt(MixDsp_faustpower2_f(fSlow10 * self.fRec5[0]) + MixDsp_faustpower2_f(fSlow4 * self.fRec1[0]) + MixDsp_faustpower2_f(fSlow7 * self.fRec3[0]));
																																			self.fRec56[0] = fSlow111 * fTemp38 + self.fConst3 * fTemp37 * self.fRec56[1];
																																			iTemp109 = (self.fRec56[0] > 0.0) as i32;
																																			fTemp110 = fSlow10 * *input0 * self.fRec5[0] + fSlow7 * *input1 * self.fRec3[0];
																																			fTemp111 = (iTemp109) as F32;
																																		}
																																			if (((fTemp111 != 0.0) as i32) & iTemp35) | ((((fTemp111 != 0.0) as i32) & iTemp32) | ((((fTemp111 != 0.0) as i32) & iTemp29) | ((((fTemp111 != 0.0) as i32) & iTemp26) | ((((fTemp111 != 0.0) as i32) & iTemp23) | ((((fTemp111 != 0.0) as i32) & iTemp20) | ((((fTemp111 != 0.0) as i32) & iTemp17) | (((fTemp111 != 0.0) as i32) & iTemp14))))))) != 0 {
																																				fTemp112 = F32::max(self.fRec56[0], 0.001);
																																				fTemp113 = fTemp112 * fTemp110;
																																				self.fVec20[0] = fTemp113;
																																				fTemp114 = 1.13003 * (fTemp113 + self.fVec20[1]);
																																				self.fRec58[0] = self.fConst34 * (fTemp114 - self.fConst33 * self.fRec58[1]);
																																				self.fRec59[0] = self.fConst39 * (fTemp114 - self.fConst38 * self.fRec59[1]);
																																				fTemp115 = 1.13003 * fTemp113;
																																				fTemp116 = (self.fConst44 * (F32::tanh(fTemp115 + 0.08) - self.fConst43) + 0.4 * (self.fConst42 * (F32::tanh(0.9 * (fTemp115 - self.fRec59[0]) + 0.05) - self.fConst41) + self.fConst40 * (F32::tanh(1.25 * (self.fRec59[0] - self.fRec58[0]) + -0.08) - self.fConst36) + self.fConst35 * (F32::tanh(1.5 * self.fRec58[0] + 0.12) - self.fConst31))) / fTemp112;
																																				self.fVec21[0] = fTemp116;
																																				self.fRec57[0] = self.fConst47 * (self.fConst46 * self.fRec57[1] + 1.25 * (fTemp116 - self.fVec21[1]));
																																				fTemp117 = self.fRec57[0] * fTemp111;
																																			}
																																				if iTemp35 | (iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | iTemp14)))))) != 0 {
																																					fTemp118 = (if iTemp109 != 0 {((iTemp109 != 0) as i32) as u32 as F32 * fTemp117} else {1.4125376 * fTemp110} * ((fTemp108 > 0.0) as i32) as u32 as F32 / F32::max(1.1920929e-07, fTemp108) + fSlow110 * fTemp107) * fTemp77 + fTemp81;
																																				}
																																					if iTemp26 | iTemp14 != 0 {
																																						fTemp119 = F32::max(-1.0, F32::min(1.0, fTemp118));
																																					}
																																						if iTemp14 != 0 {
																																							fTemp120 = fTemp119 * fTemp13;
																																						}
																																							if iTemp29 | iTemp17 != 0 {
																																								fTemp121 = F32::atan(1.5707964 * fTemp81);
																																							}
																																								if iTemp17 != 0 {
																																									fTemp122 = 0.63661975 * fTemp121 * fTemp16;
																																								}
																																									if iTemp29 | iTemp17 != 0 {
																																										fTemp123 = F32::atan(1.5707964 * fTemp118);
																																									}
																																										if iTemp17 != 0 {
																																											fTemp124 = 0.63661975 * fTemp123 * fTemp16;
																																										}
																																											if iTemp32 | iTemp20 != 0 {
																																												fTemp125 = if (fTemp81 < 0.0) as i32 != 0 {1.6 * F32::tanh(0.625 * fTemp81)} else {F32::tanh(fTemp81)};
																																											}
																																												if iTemp20 != 0 {
																																													fTemp126 = fTemp125 * fTemp19;
																																												}
																																													if iTemp32 | iTemp20 != 0 {
																																														fTemp127 = if (fTemp118 < 0.0) as i32 != 0 {1.6 * F32::tanh(0.625 * fTemp118)} else {F32::tanh(fTemp118)};
																																													}
																																														if iTemp20 != 0 {
																																															fTemp128 = fTemp127 * fTemp19;
																																														}
																																															if iTemp35 | iTemp23 != 0 {
																																																fTemp129 = F32::sin(fTemp81);
																																															}
																																																if iTemp23 != 0 {
																																																	fTemp130 = fTemp129 * fTemp22;
																																																}
																																																	if iTemp35 | iTemp23 != 0 {
																																																		fTemp131 = F32::sin(fTemp118);
																																																	}
																																																		if iTemp23 != 0 {
																																																			fTemp132 = fTemp131 * fTemp22;
																																																		}
																																																			let mut fTemp133: F32 = F32::max(0.0, F32::min(1.0, 2.5 * (fTemp9 + (-0.8 - fTemp10))));
																																																			if iTemp26 != 0 {
																																																				fTemp134 = fTemp25 * fTemp82;
																																																				fTemp135 = fTemp119 * fTemp25;
																																																			}
																																																				if iTemp29 != 0 {
																																																					fTemp136 = 0.63661975 * fTemp28 * fTemp121;
																																																					fTemp137 = 0.63661975 * fTemp123 * fTemp28;
																																																				}
																																																					if iTemp32 != 0 {
																																																						fTemp138 = fTemp31 * fTemp125;
																																																						fTemp139 = fTemp127 * fTemp31;
																																																					}
																																																						if iTemp35 != 0 {
																																																							fTemp140 = fTemp34 * fTemp129;
																																																							fTemp141 = fTemp131 * fTemp34;
																																																						}
																																																							let mut fTemp142: F32 = ((1.0 - fTemp133) * (((iTemp33 != 0) as i32) as u32 as F32 * (fTemp141 - fTemp140) + ((iTemp30 != 0) as i32) as u32 as F32 * (fTemp139 - fTemp138) + ((iTemp27 != 0) as i32) as u32 as F32 * (fTemp137 - fTemp136) + ((iTemp24 != 0) as i32) as u32 as F32 * (fTemp135 - fTemp134)) + fTemp133 * (((iTemp21 != 0) as i32) as u32 as F32 * (fTemp132 - fTemp130) + ((iTemp18 != 0) as i32) as u32 as F32 * (fTemp128 - fTemp126) + ((iTemp15 != 0) as i32) as u32 as F32 * (fTemp124 - fTemp122) + ((iTemp12 != 0) as i32) as u32 as F32 * (fTemp120 - fTemp83))) / F32::tanh(0.35 * fTemp77);
																																																							self.fVec22[0] = fTemp142;
																																																							self.fRec41[0] = 0.995 * self.fRec41[1] + 0.35 * (fTemp142 - self.fVec22[1]);
																																																							*output0 = self.fRec41[0] * F32::powf(1e+01, 0.125 * self.fRec16[0]) + fSlow91 * (self.fRec1[0] * fTemp76 / F32::max(1e-09, F32::sqrt(fSlow11 * MixDsp_faustpower2_f(self.fRec5[0]) + fSlow8 * MixDsp_faustpower2_f(self.fRec3[0]) + fSlow5 * MixDsp_faustpower2_f(self.fRec1[0]))));
																																																							*output1 = fSlow112 * fTemp107;
																																																							*output2 = fTemp64;
																																																							self.fVec0[1] = self.fVec0[0];
																																																							self.iVec1[1] = self.iVec1[0];
																																																							self.fVec2[1] = self.fVec2[0];
																																																							self.fRec0[1] = self.fRec0[0];
																																																							self.fRec1[1] = self.fRec1[0];
																																																							self.fVec3[1] = self.fVec3[0];
																																																							self.fRec2[1] = self.fRec2[0];
																																																							self.fRec3[1] = self.fRec3[0];
																																																							self.fVec4[1] = self.fVec4[0];
																																																							self.fRec4[1] = self.fRec4[0];
																																																							self.fRec5[1] = self.fRec5[0];
																																																							if iSlow12 != 0 {
																																																								self.fVec5[1] = self.fVec5[0];
																																																								self.fRec6[1] = self.fRec6[0];
																																																								self.fRec7[1] = self.fRec7[0];
																																																							}
																																																								if iSlow12 & iSlow17 != 0 {
																																																									self.fVec6[1] = self.fVec6[0];
																																																									self.fRec12[1] = self.fRec12[0];
																																																									self.fRec13[1] = self.fRec13[0];
																																																								}
																																																									self.fVec7[1] = self.fVec7[0];
																																																									self.fRec15[1] = self.fRec15[0];
																																																									self.fRec16[1] = self.fRec16[0];
																																																									if iTemp35 | (iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | iTemp14)))))) != 0 {
																																																										self.fRec17[1] = self.fRec17[0];
																																																									}
																																																										self.fVec8[1] = self.fVec8[0];
																																																										if iSlow30 != 0 {
																																																											self.fVec9[1] = self.fVec9[0];
																																																											self.fRec18[1] = self.fRec18[0];
																																																											self.fRec19[1] = self.fRec19[0];
																																																											self.iRec20[1] = self.iRec20[0];
																																																											self.iRec21[1] = self.iRec21[0];
																																																										}
																																																											self.fVec10[1] = self.fVec10[0];
																																																											self.fRec22[1] = self.fRec22[0];
																																																											self.fRec23[1] = self.fRec23[0];
																																																											if ((fTemp49 != 0.0) as i32) | ((((fTemp40 != 0.0) as i32) & iTemp35) | ((iSlow30 & (iSlow40 & (iSlow43 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32)))) | ((((fTemp40 != 0.0) as i32) & iTemp32) | ((((fTemp40 != 0.0) as i32) & iTemp29) | ((((fTemp40 != 0.0) as i32) & iTemp26) | ((((fTemp40 != 0.0) as i32) & iTemp23) | ((((fTemp40 != 0.0) as i32) & iTemp20) | ((((fTemp40 != 0.0) as i32) & iTemp17) | ((((fTemp40 != 0.0) as i32) & iTemp14) | ((iSlow12 & iSlow24) | ((iSlow12 & iSlow21) | (iSlow12 & iSlow17))))))))))))) != 0 {
																																																												self.iRec14[1] = self.iRec14[0];
																																																											}
																																																												if iSlow12 & iSlow17 != 0 {
																																																													self.fVec11[1] = self.fVec11[0];
																																																													self.fRec11[1] = self.fRec11[0];
																																																													self.fRec10[1] = self.fRec10[0];
																																																												}
																																																													if (fTemp49 != 0.0) as i32 != 0 {
																																																														self.fRec27[1] = self.fRec27[0];
																																																														self.fRec28[1] = self.fRec28[0];
																																																														self.iRec30[1] = self.iRec30[0];
																																																														self.fRec29[1] = self.fRec29[0];
																																																														self.fRec31[1] = self.fRec31[0];
																																																														self.fRec34[1] = self.fRec34[0];
																																																														self.fRec33[1] = self.fRec33[0];
																																																														self.fVec13[1] = self.fVec13[0];
																																																														self.fRec32[1] = self.fRec32[0];
																																																													}
																																																														self.fRec35[1] = self.fRec35[0];
																																																														self.fVec14[1] = self.fVec14[0];
																																																														self.fRec36[1] = self.fRec36[0];
																																																														self.fRec37[1] = self.fRec37[0];
																																																														self.fRec38[1] = self.fRec38[0];
																																																														if (iSlow12 & iSlow21) | ((iSlow12 & iSlow51) | (iSlow12 & iSlow48)) != 0 {
																																																															self.fRec25[1] = self.fRec25[0];
																																																														}
																																																															if iSlow12 & iSlow48 != 0 {
																																																																self.iRec24[1] = self.iRec24[0];
																																																															}
																																																																if iSlow12 & iSlow51 != 0 {
																																																																	self.iRec39[1] = self.iRec39[0];
																																																																}
																																																																	if iSlow12 & iSlow21 != 0 {
																																																																		self.fRec40[1] = self.fRec40[0];
																																																																	}
																																																																		if iSlow12 != 0 {
																																																																			self.fVec15[1] = self.fVec15[0];
																																																																			self.fRec9[1] = self.fRec9[0];
																																																																			self.fVec16[1] = self.fVec16[0];
																																																																			self.fRec8[1] = self.fRec8[0];
																																																																		}
																																																																			if (((fTemp40 != 0.0) as i32) & iTemp35) | ((((fTemp40 != 0.0) as i32) & iTemp32) | ((((fTemp40 != 0.0) as i32) & iTemp29) | ((((fTemp40 != 0.0) as i32) & iTemp26) | ((((fTemp40 != 0.0) as i32) & iTemp23) | ((((fTemp40 != 0.0) as i32) & iTemp20) | ((((fTemp40 != 0.0) as i32) & iTemp17) | (((fTemp40 != 0.0) as i32) & iTemp14))))))) != 0 {
																																																																				self.fRec44[1] = self.fRec44[0];
																																																																				self.fRec43[1] = self.fRec43[0];
																																																																				self.fVec17[1] = self.fVec17[0];
																																																																				self.fRec42[1] = self.fRec42[0];
																																																																			}
																																																																				if (iSlow30 & (iSlow40 & (iSlow98 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow43 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow57 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow54 & ((fTemp46 != 0.0) as i32)))) | ((iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32)))) | (iSlow30 & (iSlow94 & ((fTemp46 != 0.0) as i32))))))) != 0 {
																																																																					self.fVec18[1] = self.fVec18[0];
																																																																					self.fRec46[1] = self.fRec46[0];
																																																																					self.fRec47[1] = self.fRec47[0];
																																																																				}
																																																																					if iSlow30 & (iSlow40 & (iSlow36 & ((fTemp46 != 0.0) as i32))) != 0 {
																																																																						self.fRec49[1] = self.fRec49[0];
																																																																						self.fRec48[1] = self.fRec48[0];
																																																																					}
																																																																						if iSlow30 & (iSlow40 & (iSlow54 & ((fTemp46 != 0.0) as i32))) != 0 {
																																																																							self.fRec51[1] = self.fRec51[0];
																																																																							self.fRec50[1] = self.fRec50[0];
																																																																						}
																																																																							if iSlow30 & (iSlow40 & (iSlow57 & ((fTemp46 != 0.0) as i32))) != 0 {
																																																																								self.fRec52[1] = self.fRec52[0];
																																																																							}
																																																																								if iSlow30 & (iSlow40 & (iSlow43 & ((fTemp46 != 0.0) as i32))) != 0 {
																																																																									self.fRec53[2] = self.fRec53[1];
																																																																									self.fRec53[1] = self.fRec53[0];
																																																																								}
																																																																									if iSlow30 & ((fTemp46 != 0.0) as i32) != 0 {
																																																																										self.fVec19[1] = self.fVec19[0];
																																																																										self.fRec45[1] = self.fRec45[0];
																																																																									}
																																																																										self.fRec54[1] = self.fRec54[0];
																																																																										self.fRec55[1] = self.fRec55[0];
																																																																										if iTemp35 | (iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | iTemp14)))))) != 0 {
																																																																											self.fRec56[1] = self.fRec56[0];
																																																																										}
																																																																											if (((fTemp111 != 0.0) as i32) & iTemp35) | ((((fTemp111 != 0.0) as i32) & iTemp32) | ((((fTemp111 != 0.0) as i32) & iTemp29) | ((((fTemp111 != 0.0) as i32) & iTemp26) | ((((fTemp111 != 0.0) as i32) & iTemp23) | ((((fTemp111 != 0.0) as i32) & iTemp20) | ((((fTemp111 != 0.0) as i32) & iTemp17) | (((fTemp111 != 0.0) as i32) & iTemp14))))))) != 0 {
																																																																												self.fVec20[1] = self.fVec20[0];
																																																																												self.fRec58[1] = self.fRec58[0];
																																																																												self.fRec59[1] = self.fRec59[0];
																																																																												self.fVec21[1] = self.fVec21[0];
																																																																												self.fRec57[1] = self.fRec57[0];
																																																																											}
																																																																												self.fVec22[1] = self.fVec22[0];
																																																																												self.fRec41[1] = self.fRec41[0];
																																																																											}
																																																																											self.fTempPerm0 = fTemp4;
																																																																											self.fTempPerm1 = fTemp5;
																																																																											self.fTempPerm2 = fTemp6;
																																																																											self.iTempPerm3 = iTemp39;
																																																																											self.fTempPerm4 = fTemp40;
																																																																											self.fTempPerm5 = fTemp41;
																																																																											self.fTempPerm6 = fTemp42;
																																																																											self.fTempPerm7 = fTemp43;
																																																																											self.fTempPerm8 = fTemp44;
																																																																											self.iTempPerm9 = iTemp45;
																																																																											self.fTempPerm10 = fTemp46;
																																																																											self.fTempPerm11 = fTemp50;
																																																																											self.fTempPerm12 = fTemp51;
																																																																											self.fTempPerm13 = fTemp52;
																																																																											self.fTempPerm14 = fTemp53;
																																																																											self.iTempPerm15 = iTemp54;
																																																																											self.fTempPerm16 = fTemp55;
																																																																											self.fTempPerm17 = fTemp56;
																																																																											self.iTempPerm18 = iTemp57;
																																																																											self.fTempPerm19 = fTemp58;
																																																																											self.fTempPerm20 = fTemp59;
																																																																											self.fTempPerm21 = fTemp60;
																																																																											self.fTempPerm22 = fTemp65;
																																																																											self.iTempPerm23 = iTemp66;
																																																																											self.iTempPerm24 = iTemp67;
																																																																											self.fTempPerm25 = fTemp68;
																																																																											self.iTempPerm26 = iTemp69;
																																																																											self.iTempPerm27 = iTemp70;
																																																																											self.fTempPerm28 = fTemp71;
																																																																											self.fTempPerm29 = fTemp72;
																																																																											self.fTempPerm30 = fTemp73;
																																																																											self.fTempPerm31 = fTemp74;
																																																																											self.fTempPerm32 = fTemp75;
																																																																											self.fTempPerm33 = fTemp76;
																																																																											self.fTempPerm34 = fTemp78;
																																																																											self.fTempPerm35 = fTemp79;
																																																																											self.fTempPerm36 = fTemp80;
																																																																											self.fTempPerm37 = fTemp81;
																																																																											self.fTempPerm38 = fTemp82;
																																																																											self.fTempPerm39 = fTemp83;
																																																																											self.fTempPerm40 = fTemp84;
																																																																											self.fTempPerm41 = fTemp85;
																																																																											self.fTempPerm42 = fTemp86;
																																																																											self.fTempPerm43 = fTemp87;
																																																																											self.fTempPerm44 = fTemp88;
																																																																											self.fTempPerm45 = fTemp89;
																																																																											self.fTempPerm46 = fTemp90;
																																																																											self.fTempPerm47 = fTemp91;
																																																																											self.fTempPerm48 = fTemp92;
																																																																											self.fTempPerm49 = fTemp93;
																																																																											self.fTempPerm50 = fTemp94;
																																																																											self.fTempPerm51 = fTemp95;
																																																																											self.fTempPerm52 = fTemp96;
																																																																											self.fTempPerm53 = fTemp97;
																																																																											self.fTempPerm54 = fTemp98;
																																																																											self.fTempPerm55 = fTemp99;
																																																																											self.fTempPerm56 = fTemp100;
																																																																											self.fTempPerm57 = fTemp101;
																																																																											self.fTempPerm58 = fTemp102;
																																																																											self.fTempPerm59 = fTemp103;
																																																																											self.fTempPerm60 = fTemp104;
																																																																											self.fTempPerm61 = fTemp105;
																																																																											self.fTempPerm62 = fTemp108;
																																																																											self.iTempPerm63 = iTemp109;
																																																																											self.fTempPerm64 = fTemp110;
																																																																											self.fTempPerm65 = fTemp111;
																																																																											self.fTempPerm66 = fTemp112;
																																																																											self.fTempPerm67 = fTemp113;
																																																																											self.fTempPerm68 = fTemp114;
																																																																											self.fTempPerm69 = fTemp115;
																																																																											self.fTempPerm70 = fTemp116;
																																																																											self.fTempPerm71 = fTemp117;
																																																																											self.fTempPerm72 = fTemp118;
																																																																											self.fTempPerm73 = fTemp119;
																																																																											self.fTempPerm74 = fTemp120;
																																																																											self.fTempPerm75 = fTemp121;
																																																																											self.fTempPerm76 = fTemp122;
																																																																											self.fTempPerm77 = fTemp123;
																																																																											self.fTempPerm78 = fTemp124;
																																																																											self.fTempPerm79 = fTemp125;
																																																																											self.fTempPerm80 = fTemp126;
																																																																											self.fTempPerm81 = fTemp127;
																																																																											self.fTempPerm82 = fTemp128;
																																																																											self.fTempPerm83 = fTemp129;
																																																																											self.fTempPerm84 = fTemp130;
																																																																											self.fTempPerm85 = fTemp131;
																																																																											self.fTempPerm86 = fTemp132;
																																																																											self.fTempPerm87 = fTemp134;
																																																																											self.fTempPerm88 = fTemp135;
																																																																											self.fTempPerm89 = fTemp136;
																																																																											self.fTempPerm90 = fTemp137;
																																																																											self.fTempPerm91 = fTemp138;
																																																																											self.fTempPerm92 = fTemp139;
																																																																											self.fTempPerm93 = fTemp140;
																																																																											self.fTempPerm94 = fTemp141;
																																																																										}

}

