/* ------------------------------------------------------------
name: "plugin-mix"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn MixDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */


pub struct MixDspSIG0 {
	iVec10: [i32;2],
	iRec20: [i32;2],
}

impl MixDspSIG0 {
	
	fn get_num_inputsMixDspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsMixDspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initMixDspSIG0(&mut self, sample_rate: i32) {
		for l26 in 0..2 {
			self.iVec10[l26 as usize] = 0;
		}
		for l27 in 0..2 {
			self.iRec20[l27 as usize] = 0;
		}
	}
	
	fn fillMixDspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec10[0] = 1;
			self.iRec20[0] = (i32::wrapping_add(self.iVec10[1], self.iRec20[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec20[0]) as F32);
			self.iVec10[1] = self.iVec10[0];
			self.iRec20[1] = self.iRec20[0];
		}
	}

}


pub fn newMixDspSIG0() -> MixDspSIG0 { 
	MixDspSIG0 {
		iVec10: [0;2],
		iRec20: [0;2],
	}
}
static mut ftbl0MixDspSIG0: [F32;65536] = [0.0;65536];
fn MixDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
fn MixDsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}
fn MixDsp_faustpower3_f(value: F32) -> F32 {
	return value * value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct MixDsp {
	fHslider0: F32,
	fEntry0: F32,
	fVslider0: F32,
	fVec0: [F32;2],
	fButton0: F32,
	fVec1: [F32;2],
	iVec2: [i32;2],
	fHslider1: F32,
	fVec3: [F32;2],
	fHslider2: F32,
	fTempPerm0: F32,
	fRec0: [F32;2],
	fRec1: [F32;2],
	fTempPerm1: F32,
	fEntry1: F32,
	fSampleRate: i32,
	fConst2: F32,
	fHslider3: F32,
	fVec4: [F32;2],
	fTempPerm2: F32,
	fRec6: [F32;2],
	fRec7: [F32;2],
	fHslider4: F32,
	fVec5: [F32;2],
	fRec9: [F32;2],
	fRec10: [F32;2],
	fConst3: F32,
	fHslider5: F32,
	fRec11: [F32;2],
	iTempPerm3: i32,
	fTempPerm4: F32,
	fHslider6: F32,
	fVec6: [F32;2],
	fHslider7: F32,
	fVec7: [F32;2],
	fTempPerm5: F32,
	fRec12: [F32;2],
	fRec13: [F32;2],
	fEntry2: F32,
	fTempPerm6: F32,
	iRec14: [i32;2],
	fTempPerm7: F32,
	fConst4: F32,
	fTempPerm8: F32,
	iRec15: [i32;2],
	iTempPerm9: i32,
	fTempPerm10: F32,
	fHslider8: F32,
	fVec8: [F32;2],
	fRec16: [F32;2],
	fRec17: [F32;2],
	iRec8: [i32;2],
	fTempPerm11: F32,
	fTempPerm12: F32,
	fTempPerm13: F32,
	fVec9: [F32;2],
	fConst5: F32,
	fRec5: [F32;2],
	fConst6: F32,
	fConst7: F32,
	fConst8: F32,
	fRec4: [F32;2],
	fTempPerm14: F32,
	iTempPerm15: i32,
	fConst9: F32,
	fTempPerm16: F32,
	fRec21: [F32;2],
	fHslider9: F32,
	fTempPerm17: F32,
	fRec22: [F32;2],
	iRec24: [i32;2],
	iTempPerm18: i32,
	fConst11: F32,
	fConst13: F32,
	fRec23: [F32;2],
	fHslider10: F32,
	fRec25: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fTempPerm19: F32,
	fRec28: [F32;2],
	fRec27: [F32;2],
	fTempPerm20: F32,
	fVec11: [F32;2],
	fRec26: [F32;2],
	fTempPerm21: F32,
	fHslider11: F32,
	fRec29: [F32;2],
	fEntry3: F32,
	fHslider12: F32,
	fVec12: [F32;2],
	fRec30: [F32;2],
	fRec31: [F32;2],
	fHslider13: F32,
	fHslider14: F32,
	fRec32: [F32;2],
	fHslider15: F32,
	fTempPerm22: F32,
	fRec19: [F32;2],
	iTempPerm23: i32,
	iTempPerm24: i32,
	iRec18: [i32;2],
	fTempPerm25: F32,
	iTempPerm26: i32,
	iTempPerm27: i32,
	iRec33: [i32;2],
	fTempPerm28: F32,
	fRec34: [F32;2],
	fHslider16: F32,
	fTempPerm29: F32,
	fTempPerm30: F32,
	fTempPerm31: F32,
	fVec13: [F32;2],
	fConst16: F32,
	fConst17: F32,
	fConst18: F32,
	fConst19: F32,
	fRec3: [F32;2],
	fTempPerm32: F32,
	fVec14: [F32;2],
	fRec2: [F32;2],
	fTempPerm33: F32,
	fRec35: [F32;2],
	fRec36: [F32;2],
	fEntry4: F32,
	fEntry5: F32,
	fHslider17: F32,
	fRec37: [F32;2],
	iTempPerm34: i32,
	fHslider18: F32,
	fVec15: [F32;2],
	fTempPerm35: F32,
	fRec38: [F32;2],
	fRec39: [F32;2],
	fTempPerm36: F32,
	fTempPerm37: F32,
	fTempPerm38: F32,
	fTempPerm39: F32,
	fTempPerm40: F32,
	iTempPerm41: i32,
	fTempPerm42: F32,
	fConst20: F32,
	fConst22: F32,
	fTempPerm43: F32,
	fVec16: [F32;2],
	fTempPerm44: F32,
	fConst23: F32,
	fRec41: [F32;2],
	fConst24: F32,
	fConst25: F32,
	fConst27: F32,
	fConst28: F32,
	fRec42: [F32;2],
	fConst29: F32,
	fConst30: F32,
	fTempPerm45: F32,
	fConst31: F32,
	fConst32: F32,
	fConst33: F32,
	fTempPerm46: F32,
	fVec17: [F32;2],
	fConst35: F32,
	fConst36: F32,
	fRec40: [F32;2],
	fTempPerm47: F32,
	fTempPerm48: F32,
	fTempPerm49: F32,
	fConst38: F32,
	fConst39: F32,
	fTempPerm50: F32,
	fRec46: [F32;2],
	fRec45: [F32;2],
	fTempPerm51: F32,
	fVec18: [F32;2],
	fConst40: F32,
	fRec44: [F32;2],
	fTempPerm52: F32,
	fTempPerm53: F32,
	fTempPerm54: F32,
	fTempPerm55: F32,
	fHslider19: F32,
	fVec19: [F32;2],
	fTempPerm56: F32,
	fRec48: [F32;2],
	fRec49: [F32;2],
	fTempPerm57: F32,
	fTempPerm58: F32,
	fConst41: F32,
	fTempPerm59: F32,
	fTempPerm60: F32,
	fConst42: F32,
	fTempPerm61: F32,
	fTempPerm62: F32,
	fRec51: [F32;2],
	fRec50: [F32;2],
	fTempPerm63: F32,
	fTempPerm64: F32,
	fEntry6: F32,
	fTempPerm65: F32,
	fHslider20: F32,
	fVec20: [F32;2],
	fTempPerm66: F32,
	fRec52: [F32;2],
	fRec53: [F32;2],
	fTempPerm67: F32,
	fTempPerm68: F32,
	fRec54: [F32;2],
	fTempPerm69: F32,
	fTempPerm70: F32,
	fRec55: [F32;2],
	fTempPerm71: F32,
	fConst43: F32,
	fTempPerm72: F32,
	fRec56: [F32;2],
	fTempPerm73: F32,
	fConst44: F32,
	fTempPerm74: F32,
	fTempPerm75: F32,
	fTempPerm76: F32,
	fTempPerm77: F32,
	fRec57: [F32;3],
	fTempPerm78: F32,
	fConst45: F32,
	fConst46: F32,
	fTempPerm79: F32,
	fTempPerm80: F32,
	fTempPerm81: F32,
	fVec21: [F32;2],
	fRec47: [F32;2],
	fTempPerm82: F32,
	fTempPerm83: F32,
	fRec58: [F32;2],
	fRec59: [F32;2],
	fEntry7: F32,
	fTempPerm84: F32,
	fTempPerm85: F32,
	fVec22: [F32;2],
	fTempPerm86: F32,
	fRec61: [F32;2],
	fRec62: [F32;2],
	fTempPerm87: F32,
	fTempPerm88: F32,
	fVec23: [F32;2],
	fRec60: [F32;2],
	fTempPerm89: F32,
	fTempPerm90: F32,
	fTempPerm91: F32,
	fTempPerm92: F32,
	fTempPerm93: F32,
	fTempPerm94: F32,
	fTempPerm95: F32,
	fTempPerm96: F32,
	fTempPerm97: F32,
	fTempPerm98: F32,
	fTempPerm99: F32,
	fTempPerm100: F32,
	fTempPerm101: F32,
	fTempPerm102: F32,
	fTempPerm103: F32,
	fTempPerm104: F32,
	fTempPerm105: F32,
	fTempPerm106: F32,
	fTempPerm107: F32,
	fTempPerm108: F32,
	fTempPerm109: F32,
	fTempPerm110: F32,
	fTempPerm111: F32,
	fTempPerm112: F32,
	fTempPerm113: F32,
	fVec24: [F32;2],
	fRec43: [F32;2],
}

impl FaustDsp for MixDsp {
	type T = F32;
		
	fn new() -> MixDsp { 
		MixDsp {
			fHslider0: 0.0,
			fEntry0: 0.0,
			fVslider0: 0.0,
			fVec0: [0.0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			iVec2: [0;2],
			fHslider1: 0.0,
			fVec3: [0.0;2],
			fHslider2: 0.0,
			fTempPerm0: 0.0,
			fRec0: [0.0;2],
			fRec1: [0.0;2],
			fTempPerm1: 0.0,
			fEntry1: 0.0,
			fSampleRate: 0,
			fConst2: 0.0,
			fHslider3: 0.0,
			fVec4: [0.0;2],
			fTempPerm2: 0.0,
			fRec6: [0.0;2],
			fRec7: [0.0;2],
			fHslider4: 0.0,
			fVec5: [0.0;2],
			fRec9: [0.0;2],
			fRec10: [0.0;2],
			fConst3: 0.0,
			fHslider5: 0.0,
			fRec11: [0.0;2],
			iTempPerm3: 0,
			fTempPerm4: 0.0,
			fHslider6: 0.0,
			fVec6: [0.0;2],
			fHslider7: 0.0,
			fVec7: [0.0;2],
			fTempPerm5: 0.0,
			fRec12: [0.0;2],
			fRec13: [0.0;2],
			fEntry2: 0.0,
			fTempPerm6: 0.0,
			iRec14: [0;2],
			fTempPerm7: 0.0,
			fConst4: 0.0,
			fTempPerm8: 0.0,
			iRec15: [0;2],
			iTempPerm9: 0,
			fTempPerm10: 0.0,
			fHslider8: 0.0,
			fVec8: [0.0;2],
			fRec16: [0.0;2],
			fRec17: [0.0;2],
			iRec8: [0;2],
			fTempPerm11: 0.0,
			fTempPerm12: 0.0,
			fTempPerm13: 0.0,
			fVec9: [0.0;2],
			fConst5: 0.0,
			fRec5: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fConst8: 0.0,
			fRec4: [0.0;2],
			fTempPerm14: 0.0,
			iTempPerm15: 0,
			fConst9: 0.0,
			fTempPerm16: 0.0,
			fRec21: [0.0;2],
			fHslider9: 0.0,
			fTempPerm17: 0.0,
			fRec22: [0.0;2],
			iRec24: [0;2],
			iTempPerm18: 0,
			fConst11: 0.0,
			fConst13: 0.0,
			fRec23: [0.0;2],
			fHslider10: 0.0,
			fRec25: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fTempPerm19: 0.0,
			fRec28: [0.0;2],
			fRec27: [0.0;2],
			fTempPerm20: 0.0,
			fVec11: [0.0;2],
			fRec26: [0.0;2],
			fTempPerm21: 0.0,
			fHslider11: 0.0,
			fRec29: [0.0;2],
			fEntry3: 0.0,
			fHslider12: 0.0,
			fVec12: [0.0;2],
			fRec30: [0.0;2],
			fRec31: [0.0;2],
			fHslider13: 0.0,
			fHslider14: 0.0,
			fRec32: [0.0;2],
			fHslider15: 0.0,
			fTempPerm22: 0.0,
			fRec19: [0.0;2],
			iTempPerm23: 0,
			iTempPerm24: 0,
			iRec18: [0;2],
			fTempPerm25: 0.0,
			iTempPerm26: 0,
			iTempPerm27: 0,
			iRec33: [0;2],
			fTempPerm28: 0.0,
			fRec34: [0.0;2],
			fHslider16: 0.0,
			fTempPerm29: 0.0,
			fTempPerm30: 0.0,
			fTempPerm31: 0.0,
			fVec13: [0.0;2],
			fConst16: 0.0,
			fConst17: 0.0,
			fConst18: 0.0,
			fConst19: 0.0,
			fRec3: [0.0;2],
			fTempPerm32: 0.0,
			fVec14: [0.0;2],
			fRec2: [0.0;2],
			fTempPerm33: 0.0,
			fRec35: [0.0;2],
			fRec36: [0.0;2],
			fEntry4: 0.0,
			fEntry5: 0.0,
			fHslider17: 0.0,
			fRec37: [0.0;2],
			iTempPerm34: 0,
			fHslider18: 0.0,
			fVec15: [0.0;2],
			fTempPerm35: 0.0,
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fTempPerm36: 0.0,
			fTempPerm37: 0.0,
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fTempPerm40: 0.0,
			iTempPerm41: 0,
			fTempPerm42: 0.0,
			fConst20: 0.0,
			fConst22: 0.0,
			fTempPerm43: 0.0,
			fVec16: [0.0;2],
			fTempPerm44: 0.0,
			fConst23: 0.0,
			fRec41: [0.0;2],
			fConst24: 0.0,
			fConst25: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fRec42: [0.0;2],
			fConst29: 0.0,
			fConst30: 0.0,
			fTempPerm45: 0.0,
			fConst31: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fTempPerm46: 0.0,
			fVec17: [0.0;2],
			fConst35: 0.0,
			fConst36: 0.0,
			fRec40: [0.0;2],
			fTempPerm47: 0.0,
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fConst38: 0.0,
			fConst39: 0.0,
			fTempPerm50: 0.0,
			fRec46: [0.0;2],
			fRec45: [0.0;2],
			fTempPerm51: 0.0,
			fVec18: [0.0;2],
			fConst40: 0.0,
			fRec44: [0.0;2],
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fTempPerm54: 0.0,
			fTempPerm55: 0.0,
			fHslider19: 0.0,
			fVec19: [0.0;2],
			fTempPerm56: 0.0,
			fRec48: [0.0;2],
			fRec49: [0.0;2],
			fTempPerm57: 0.0,
			fTempPerm58: 0.0,
			fConst41: 0.0,
			fTempPerm59: 0.0,
			fTempPerm60: 0.0,
			fConst42: 0.0,
			fTempPerm61: 0.0,
			fTempPerm62: 0.0,
			fRec51: [0.0;2],
			fRec50: [0.0;2],
			fTempPerm63: 0.0,
			fTempPerm64: 0.0,
			fEntry6: 0.0,
			fTempPerm65: 0.0,
			fHslider20: 0.0,
			fVec20: [0.0;2],
			fTempPerm66: 0.0,
			fRec52: [0.0;2],
			fRec53: [0.0;2],
			fTempPerm67: 0.0,
			fTempPerm68: 0.0,
			fRec54: [0.0;2],
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			fRec55: [0.0;2],
			fTempPerm71: 0.0,
			fConst43: 0.0,
			fTempPerm72: 0.0,
			fRec56: [0.0;2],
			fTempPerm73: 0.0,
			fConst44: 0.0,
			fTempPerm74: 0.0,
			fTempPerm75: 0.0,
			fTempPerm76: 0.0,
			fTempPerm77: 0.0,
			fRec57: [0.0;3],
			fTempPerm78: 0.0,
			fConst45: 0.0,
			fConst46: 0.0,
			fTempPerm79: 0.0,
			fTempPerm80: 0.0,
			fTempPerm81: 0.0,
			fVec21: [0.0;2],
			fRec47: [0.0;2],
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			fRec58: [0.0;2],
			fRec59: [0.0;2],
			fEntry7: 0.0,
			fTempPerm84: 0.0,
			fTempPerm85: 0.0,
			fVec22: [0.0;2],
			fTempPerm86: 0.0,
			fRec61: [0.0;2],
			fRec62: [0.0;2],
			fTempPerm87: 0.0,
			fTempPerm88: 0.0,
			fVec23: [0.0;2],
			fRec60: [0.0;2],
			fTempPerm89: 0.0,
			fTempPerm90: 0.0,
			fTempPerm91: 0.0,
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fTempPerm94: 0.0,
			fTempPerm95: 0.0,
			fTempPerm96: 0.0,
			fTempPerm97: 0.0,
			fTempPerm98: 0.0,
			fTempPerm99: 0.0,
			fTempPerm100: 0.0,
			fTempPerm101: 0.0,
			fTempPerm102: 0.0,
			fTempPerm103: 0.0,
			fTempPerm104: 0.0,
			fTempPerm105: 0.0,
			fTempPerm106: 0.0,
			fTempPerm107: 0.0,
			fTempPerm108: 0.0,
			fTempPerm109: 0.0,
			fTempPerm110: 0.0,
			fTempPerm111: 0.0,
			fTempPerm112: 0.0,
			fTempPerm113: 0.0,
			fVec24: [0.0;2],
			fRec43: [0.0;2],
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
		return 4;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: MixDspSIG0 = newMixDspSIG0();
		sig0.instance_initMixDspSIG0(sample_rate);
		sig0.fillMixDspSIG0(65536, unsafe { &mut ftbl0MixDspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 0.0;
		self.fEntry0 = 0.0;
		self.fVslider0 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider1 = 0.0;
		self.fHslider2 = 64.0;
		self.fEntry1 = 1.0;
		self.fHslider3 = 0.3;
		self.fHslider4 = 0.1;
		self.fHslider5 = 1.0;
		self.fHslider6 = 0.0;
		self.fHslider7 = 6e+01;
		self.fEntry2 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 1.0;
		self.fHslider11 = 0.0;
		self.fEntry3 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 1.1e+02;
		self.fHslider15 = 0.0;
		self.fHslider16 = 8.0;
		self.fEntry4 = 1.0;
		self.fEntry5 = 1.0;
		self.fHslider17 = 1.0;
		self.fHslider18 = 0.0;
		self.fHslider19 = 0.0;
		self.fEntry6 = 6.0;
		self.fHslider20 = 0.5;
		self.fEntry7 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fVec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.iVec2[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.fVec3[l3 as usize] = 0.0;
		}
		self.fTempPerm0 = 0.0;
		for l4 in 0..2 {
			self.fRec0[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec1[l5 as usize] = 0.0;
		}
		self.fTempPerm1 = 0.0;
		for l6 in 0..2 {
			self.fVec4[l6 as usize] = 0.0;
		}
		self.fTempPerm2 = 0.0;
		for l7 in 0..2 {
			self.fRec6[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec7[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fVec5[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec9[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec10[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec11[l12 as usize] = 0.0;
		}
		self.iTempPerm3 = 0;
		self.fTempPerm4 = 0.0;
		for l13 in 0..2 {
			self.fVec6[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fVec7[l14 as usize] = 0.0;
		}
		self.fTempPerm5 = 0.0;
		for l15 in 0..2 {
			self.fRec12[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec13[l16 as usize] = 0.0;
		}
		self.fTempPerm6 = 0.0;
		for l17 in 0..2 {
			self.iRec14[l17 as usize] = 0;
		}
		self.fTempPerm7 = 0.0;
		self.fTempPerm8 = 0.0;
		for l18 in 0..2 {
			self.iRec15[l18 as usize] = 0;
		}
		self.iTempPerm9 = 0;
		self.fTempPerm10 = 0.0;
		for l19 in 0..2 {
			self.fVec8[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec16[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec17[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.iRec8[l22 as usize] = 0;
		}
		self.fTempPerm11 = 0.0;
		self.fTempPerm12 = 0.0;
		self.fTempPerm13 = 0.0;
		for l23 in 0..2 {
			self.fVec9[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec5[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec4[l25 as usize] = 0.0;
		}
		self.fTempPerm14 = 0.0;
		self.iTempPerm15 = 0;
		self.fTempPerm16 = 0.0;
		for l28 in 0..2 {
			self.fRec21[l28 as usize] = 0.0;
		}
		self.fTempPerm17 = 0.0;
		for l29 in 0..2 {
			self.fRec22[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.iRec24[l30 as usize] = 0;
		}
		self.iTempPerm18 = 0;
		for l31 in 0..2 {
			self.fRec23[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec25[l32 as usize] = 0.0;
		}
		self.fTempPerm19 = 0.0;
		for l33 in 0..2 {
			self.fRec28[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec27[l34 as usize] = 0.0;
		}
		self.fTempPerm20 = 0.0;
		for l35 in 0..2 {
			self.fVec11[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec26[l36 as usize] = 0.0;
		}
		self.fTempPerm21 = 0.0;
		for l37 in 0..2 {
			self.fRec29[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fVec12[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec30[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec31[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec32[l41 as usize] = 0.0;
		}
		self.fTempPerm22 = 0.0;
		for l42 in 0..2 {
			self.fRec19[l42 as usize] = 0.0;
		}
		self.iTempPerm23 = 0;
		self.iTempPerm24 = 0;
		for l43 in 0..2 {
			self.iRec18[l43 as usize] = 0;
		}
		self.fTempPerm25 = 0.0;
		self.iTempPerm26 = 0;
		self.iTempPerm27 = 0;
		for l44 in 0..2 {
			self.iRec33[l44 as usize] = 0;
		}
		self.fTempPerm28 = 0.0;
		for l45 in 0..2 {
			self.fRec34[l45 as usize] = 0.0;
		}
		self.fTempPerm29 = 0.0;
		self.fTempPerm30 = 0.0;
		self.fTempPerm31 = 0.0;
		for l46 in 0..2 {
			self.fVec13[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec3[l47 as usize] = 0.0;
		}
		self.fTempPerm32 = 0.0;
		for l48 in 0..2 {
			self.fVec14[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec2[l49 as usize] = 0.0;
		}
		self.fTempPerm33 = 0.0;
		for l50 in 0..2 {
			self.fRec35[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec36[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec37[l52 as usize] = 0.0;
		}
		self.iTempPerm34 = 0;
		for l53 in 0..2 {
			self.fVec15[l53 as usize] = 0.0;
		}
		self.fTempPerm35 = 0.0;
		for l54 in 0..2 {
			self.fRec38[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec39[l55 as usize] = 0.0;
		}
		self.fTempPerm36 = 0.0;
		self.fTempPerm37 = 0.0;
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		self.fTempPerm40 = 0.0;
		self.iTempPerm41 = 0;
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		for l56 in 0..2 {
			self.fVec16[l56 as usize] = 0.0;
		}
		self.fTempPerm44 = 0.0;
		for l57 in 0..2 {
			self.fRec41[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec42[l58 as usize] = 0.0;
		}
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		for l59 in 0..2 {
			self.fVec17[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec40[l60 as usize] = 0.0;
		}
		self.fTempPerm47 = 0.0;
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		for l61 in 0..2 {
			self.fRec46[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec45[l62 as usize] = 0.0;
		}
		self.fTempPerm51 = 0.0;
		for l63 in 0..2 {
			self.fVec18[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec44[l64 as usize] = 0.0;
		}
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		self.fTempPerm54 = 0.0;
		self.fTempPerm55 = 0.0;
		for l65 in 0..2 {
			self.fVec19[l65 as usize] = 0.0;
		}
		self.fTempPerm56 = 0.0;
		for l66 in 0..2 {
			self.fRec48[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec49[l67 as usize] = 0.0;
		}
		self.fTempPerm57 = 0.0;
		self.fTempPerm58 = 0.0;
		self.fTempPerm59 = 0.0;
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		self.fTempPerm62 = 0.0;
		for l68 in 0..2 {
			self.fRec51[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec50[l69 as usize] = 0.0;
		}
		self.fTempPerm63 = 0.0;
		self.fTempPerm64 = 0.0;
		self.fTempPerm65 = 0.0;
		for l70 in 0..2 {
			self.fVec20[l70 as usize] = 0.0;
		}
		self.fTempPerm66 = 0.0;
		for l71 in 0..2 {
			self.fRec52[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec53[l72 as usize] = 0.0;
		}
		self.fTempPerm67 = 0.0;
		self.fTempPerm68 = 0.0;
		for l73 in 0..2 {
			self.fRec54[l73 as usize] = 0.0;
		}
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		for l74 in 0..2 {
			self.fRec55[l74 as usize] = 0.0;
		}
		self.fTempPerm71 = 0.0;
		self.fTempPerm72 = 0.0;
		for l75 in 0..2 {
			self.fRec56[l75 as usize] = 0.0;
		}
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		self.fTempPerm75 = 0.0;
		self.fTempPerm76 = 0.0;
		self.fTempPerm77 = 0.0;
		for l76 in 0..3 {
			self.fRec57[l76 as usize] = 0.0;
		}
		self.fTempPerm78 = 0.0;
		self.fTempPerm79 = 0.0;
		self.fTempPerm80 = 0.0;
		self.fTempPerm81 = 0.0;
		for l77 in 0..2 {
			self.fVec21[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec47[l78 as usize] = 0.0;
		}
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		for l79 in 0..2 {
			self.fRec58[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec59[l80 as usize] = 0.0;
		}
		self.fTempPerm84 = 0.0;
		self.fTempPerm85 = 0.0;
		for l81 in 0..2 {
			self.fVec22[l81 as usize] = 0.0;
		}
		self.fTempPerm86 = 0.0;
		for l82 in 0..2 {
			self.fRec61[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec62[l83 as usize] = 0.0;
		}
		self.fTempPerm87 = 0.0;
		self.fTempPerm88 = 0.0;
		for l84 in 0..2 {
			self.fVec23[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec60[l85 as usize] = 0.0;
		}
		self.fTempPerm89 = 0.0;
		self.fTempPerm90 = 0.0;
		self.fTempPerm91 = 0.0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		self.fTempPerm96 = 0.0;
		self.fTempPerm97 = 0.0;
		self.fTempPerm98 = 0.0;
		self.fTempPerm99 = 0.0;
		self.fTempPerm100 = 0.0;
		self.fTempPerm101 = 0.0;
		self.fTempPerm102 = 0.0;
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		self.fTempPerm105 = 0.0;
		self.fTempPerm106 = 0.0;
		self.fTempPerm107 = 0.0;
		self.fTempPerm108 = 0.0;
		self.fTempPerm109 = 0.0;
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		self.fTempPerm112 = 0.0;
		self.fTempPerm113 = 0.0;
		for l86 in 0..2 {
			self.fVec24[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec43[l87 as usize] = 0.0;
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
		self.fConst16 = 0.4 * fConst0;
		self.fConst17 = 1.0 / F32::tan(3.1415927 * (F32::min(2e+03, self.fConst16) / fConst0));
		self.fConst18 = 1.0 - self.fConst17;
		self.fConst19 = 1.0 / (self.fConst17 + 1.0);
		self.fConst20 = F32::tanh(0.12);
		let mut fConst21: F32 = 1.0 / F32::tan(fConst10);
		self.fConst22 = 1.0 - fConst21;
		self.fConst23 = 1.0 / (fConst21 + 1.0);
		self.fConst24 = 0.6666667 / (1.0 - MixDsp_faustpower2_f(self.fConst20));
		self.fConst25 = F32::tanh(-0.08);
		let mut fConst26: F32 = 1.0 / F32::tan(12566.371 / fConst0);
		self.fConst27 = 1.0 - fConst26;
		self.fConst28 = 1.0 / (fConst26 + 1.0);
		self.fConst29 = 0.8 / (1.0 - MixDsp_faustpower2_f(self.fConst25));
		self.fConst30 = F32::tanh(0.05);
		self.fConst31 = 1.1111112 / (1.0 - MixDsp_faustpower2_f(self.fConst30));
		self.fConst32 = F32::tanh(0.08);
		self.fConst33 = 0.6 / (1.0 - MixDsp_faustpower2_f(self.fConst32));
		let mut fConst34: F32 = 15.707963 / fConst0;
		self.fConst35 = 1.0 - fConst34;
		self.fConst36 = 1.0 / (fConst34 + 1.0);
		let mut fConst37: F32 = 1.0 / F32::tan(1570.7964 / fConst0);
		self.fConst38 = 1.0 - fConst37;
		self.fConst39 = 5e+02 / fConst0;
		self.fConst40 = 1.0 / (fConst37 + 1.0);
		self.fConst41 = 4.41e+04 / fConst0;
		self.fConst42 = 5.0 / fConst0;
		self.fConst43 = 125.0 / fConst0;
		self.fConst44 = 0.45 * fConst0;
		self.fConst45 = 666.6667 / fConst0;
		self.fConst46 = 15707.963 / fConst0;
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
		ui_interface.declare(Some(ParamIndex(1)), "5", "");
		ui_interface.declare(Some(ParamIndex(1)), "style", "menu{'2 Semitones':0;'Octave':1;'Cutoff 50%':2;'Resonance 50%':3;'Filter B 50%':4}");
		ui_interface.add_num_entry("Bend Mode", ParamIndex(1), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "13", "");
		ui_interface.open_horizontal_box("Settings");
		ui_interface.declare(Some(ParamIndex(2)), "0", "");
		ui_interface.add_horizontal_slider("Drive Grit", ParamIndex(2), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(3)), "1", "");
		ui_interface.add_horizontal_slider("Vintage Noise", ParamIndex(3), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(4)), "2", "");
		ui_interface.add_horizontal_slider("Heat", ParamIndex(4), 1.0, 0.0, 2.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "4", "");
		ui_interface.open_horizontal_box("Noise");
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.declare(Some(ParamIndex(5)), "style", "menu{'White':0;'Clocked':1;'LFSR Long':2;'LFSR Short':3;'Crackle':4}");
		ui_interface.add_num_entry("Type", ParamIndex(5), 1.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "2", "");
		ui_interface.declare(Some(ParamIndex(6)), "unit", "oct");
		ui_interface.add_horizontal_slider("Clock Ratio", ParamIndex(6), 0.0, -4.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(7)), "3", "");
		ui_interface.add_horizontal_slider("Bits", ParamIndex(7), 8.0, 1.0, 16.0, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "4", "");
		ui_interface.add_horizontal_slider("Crackle Density", ParamIndex(8), 0.3, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(9)), "5", "");
		ui_interface.declare(Some(ParamIndex(9)), "unit", "dB");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(9), 0.0, -24.0, 24.0, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "5", "");
		ui_interface.open_horizontal_box("Transient");
		ui_interface.declare(Some(ParamIndex(10)), "1", "");
		ui_interface.declare(Some(ParamIndex(10)), "style", "menu{'Click':0;'Burst':1;'Zap':2;'FM':3;'Chiff':4;'Jade 01':5;'Jade 02':6;'Jade 03':7;'Jade 04':8;'Jade 05':9;'Jade 06':10;'Jade 07':11;'Jade 08':12;'Jade 09':13;'Jade 10':14;'Jade 11':15;'Jade 12':16;'Jade 13':17;'Jade 14':18;'Jade 15':19;'Jade 16':20;'Jade 17':21;'Jade 18':22;'Jade 19':23;'Jade 20':24;'Jade 21':25;'Jade 22':26;'Jade 23':27;'Jade 24':28;'Jade 25':29;'Jade 26':30;'Jade 27':31;'Jade 28':32;'Jade 29':33;'Jade 30':34;'Jade 31':35;'Jade 32':36;'Jade 33':37;'Jade 34':38;'Jade 35':39;'Jade 36':40;'Jade 37':41;'Jade 38':42;'Jade 39':43;'Jade 40':44;'Jade 41':45;'Jade 42':46;'Jade 43':47;'Jade 44':48;'Jade 45':49;'Jade 46':50;'Jade 47':51;'Jade 48':52;'Jade 49':53;'Jade 50':54;'Jade 51':55;'Jade 52':56;'Jade 53':57;'Jade 54':58;'Jade 55':59;'Jade 56':60;'Jade 57':61;'Jade 58':62;'Jade 59':63;'Jade 60':64;'Jade 61':65;'Jade 62':66;'Jade 63':67;'Jade 64':68;'Jade 65':69;'Jade 66':70;'Jade 67':71;'Jade 68':72;'Jade 69':73;'Jade 70':74;'Jade 71':75;'Jade 72':76;'Jade 73':77;'Jade 74':78;'Jade 75':79;'Jade 76':80;'Jade 77':81;'Jade 78':82;'Jade 79':83;'Jade 80':84;'Jade 81':85;'Jade 82':86;'Jade 83':87;'Jade 84':88;'Jade 85':89;'Jade 86':90;'Jade 87':91;'Jade 88':92;'Jade 89':93;'Jade 90':94;'Jade 91':95;'Jade 92':96;'Jade 93':97;'Jade 94':98;'Jade 95':99;'Mech 01':100;'Mech 02':101;'Mech 03':102;'Mech 04':103;'Mech 05':104;'Mech 06':105;'Mech 07':106;'Mech 08':107;'Mech 09':108;'Mech 10':109;'Mech 11':110;'Mech 12':111;'Mech 13':112;'Mech 14':113;'Mech 15':114;'Mech 16':115}");
		ui_interface.add_num_entry("Type", ParamIndex(10), 0.0, 0.0, 115.0, 1.0);
		ui_interface.declare(Some(ParamIndex(11)), "2", "");
		ui_interface.declare(Some(ParamIndex(11)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(11), 6e+01, 5.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(12)), "3", "");
		ui_interface.add_horizontal_slider("Level", ParamIndex(12), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(13)), "4", "");
		ui_interface.declare(Some(ParamIndex(13)), "style", "menu{'Off':0;'On':1}");
		ui_interface.add_num_entry("Post-Filter", ParamIndex(13), 1.0, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(14)), "5", "");
		ui_interface.declare(Some(ParamIndex(14)), "unit", "st");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(14), 0.0, -24.0, 24.0, 0.1);
		ui_interface.declare(Some(ParamIndex(15)), "6", "");
		ui_interface.add_horizontal_slider("Depth", ParamIndex(15), 0.5, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(16)), "7", "");
		ui_interface.declare(Some(ParamIndex(16)), "style", "menu{'2:1':0;'2.25:1':1;'2.5:1':2;'2.75:1':3;'3:1':4;'3.25:1':5;'3.5:1':6;'3.75:1':7;'4:1':8;'4.25:1':9;'4.5:1':10;'4.75:1':11;'5:1':12;'5.25:1':13;'5.5:1':14;'5.75:1':15;'6:1':16;'6.25:1':17;'6.5:1':18;'6.75:1':19;'7:1':20}");
		ui_interface.add_num_entry("Ratio", ParamIndex(16), 6.0, 0.0, 2e+01, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("Balance", ParamIndex(17), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(18)), "3", "");
		ui_interface.add_vertical_slider("Noise", ParamIndex(18), 0.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "7", "");
		ui_interface.open_horizontal_box("Drive");
		ui_interface.declare(Some(ParamIndex(19)), "2", "");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(19), 0.1, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "3", "");
		ui_interface.declare(Some(ParamIndex(20)), "style", "menu{'Off':0;'On':1}");
		ui_interface.add_num_entry("Osc I", ParamIndex(20), 1.0, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(21)), "4", "");
		ui_interface.declare(Some(ParamIndex(21)), "style", "menu{'Off':0;'On':1}");
		ui_interface.add_num_entry("Osc II", ParamIndex(21), 1.0, 0.0, 1.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("Filter");
		ui_interface.declare(Some(ParamIndex(22)), "0", "");
		ui_interface.declare(Some(ParamIndex(22)), "style", "menu{'SVF':0;'Ladder':1;'101':2}");
		ui_interface.add_num_entry("Type", ParamIndex(22), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(23)), "4", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(23), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(24)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(24)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(24), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("freq", ParamIndex(25), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_button("gate", ParamIndex(26));
		ui_interface.add_horizontal_slider("glide", ParamIndex(27), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("ramp", ParamIndex(28), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("slot", ParamIndex(29), 0.0, 0.0, 64.0, 1.0);
		ui_interface.add_horizontal_slider("vibrato", ParamIndex(30), 0.0, -24.0, 24.0, 0.0001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			26 => Some(self.fButton0),
			22 => Some(self.fEntry0),
			5 => Some(self.fEntry1),
			10 => Some(self.fEntry2),
			1 => Some(self.fEntry3),
			21 => Some(self.fEntry4),
			20 => Some(self.fEntry5),
			16 => Some(self.fEntry6),
			13 => Some(self.fEntry7),
			23 => Some(self.fHslider0),
			9 => Some(self.fHslider1),
			3 => Some(self.fHslider10),
			24 => Some(self.fHslider11),
			30 => Some(self.fHslider12),
			27 => Some(self.fHslider13),
			25 => Some(self.fHslider14),
			6 => Some(self.fHslider15),
			7 => Some(self.fHslider16),
			4 => Some(self.fHslider17),
			17 => Some(self.fHslider18),
			14 => Some(self.fHslider19),
			28 => Some(self.fHslider2),
			15 => Some(self.fHslider20),
			8 => Some(self.fHslider3),
			19 => Some(self.fHslider4),
			2 => Some(self.fHslider5),
			12 => Some(self.fHslider6),
			11 => Some(self.fHslider7),
			0 => Some(self.fHslider8),
			29 => Some(self.fHslider9),
			18 => Some(self.fVslider0),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			26 => { self.fButton0 = value }
			22 => { self.fEntry0 = value }
			5 => { self.fEntry1 = value }
			10 => { self.fEntry2 = value }
			1 => { self.fEntry3 = value }
			21 => { self.fEntry4 = value }
			20 => { self.fEntry5 = value }
			16 => { self.fEntry6 = value }
			13 => { self.fEntry7 = value }
			23 => { self.fHslider0 = value }
			9 => { self.fHslider1 = value }
			3 => { self.fHslider10 = value }
			24 => { self.fHslider11 = value }
			30 => { self.fHslider12 = value }
			27 => { self.fHslider13 = value }
			25 => { self.fHslider14 = value }
			6 => { self.fHslider15 = value }
			7 => { self.fHslider16 = value }
			4 => { self.fHslider17 = value }
			17 => { self.fHslider18 = value }
			14 => { self.fHslider19 = value }
			28 => { self.fHslider2 = value }
			15 => { self.fHslider20 = value }
			8 => { self.fHslider3 = value }
			19 => { self.fHslider4 = value }
			2 => { self.fHslider5 = value }
			12 => { self.fHslider6 = value }
			11 => { self.fHslider7 = value }
			0 => { self.fHslider8 = value }
			29 => { self.fHslider9 = value }
			18 => { self.fVslider0 = value }
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
		let (outputs0, outputs1, outputs2, outputs3) = if let [outputs0, outputs1, outputs2, outputs3, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			let outputs3 = outputs3[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2, outputs3)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fVslider0;
		let mut iSlow1: i32 = (fSlow0 != 0.0) as i32;
		let mut iSlow2: i32 = iSlow1 | (((self.fEntry0 > 0.0) as i32) & ((self.fHslider0 > 0.0) as i32));
		let mut fSlow3: F32 = (iSlow2) as F32;
		let mut iSlow4: i32 = (fSlow3 != 0.0) as i32;
		let mut fSlow5: F32 = self.fButton0;
		let mut fSlow6: F32 = self.fHslider1;
		let mut fSlow7: F32 = self.fHslider2;
		let mut fTemp1: F32 = self.fTempPerm0;
		let mut fTemp2: F32 = self.fTempPerm1;
		let mut fSlow8: F32 = self.fEntry1;
		let mut iSlow9: i32 = (fSlow8 == 4.0) as i32;
		let mut fSlow10: F32 = (iSlow9) as F32;
		let mut iSlow11: i32 = (fSlow10 != 0.0) as i32;
		let mut fSlow12: F32 = self.fHslider3;
		let mut fTemp3: F32 = self.fTempPerm2;
		let mut iSlow13: i32 = (fSlow8 == 1.0) as i32;
		let mut fSlow14: F32 = (iSlow13) as F32;
		let mut iSlow15: i32 = (fSlow14 != 0.0) as i32;
		let mut iSlow16: i32 = (fSlow8 == 0.0) as i32;
		let mut fSlow17: F32 = (iSlow16) as F32;
		let mut iSlow18: i32 = (fSlow17 != 0.0) as i32;
		let mut fSlow19: F32 = self.fHslider4;
		let mut fSlow20: F32 = self.fHslider5;
		let mut iTemp36: i32 = self.iTempPerm3;
		let mut fTemp37: F32 = self.fTempPerm4;
		let mut fSlow21: F32 = self.fHslider6;
		let mut iSlow22: i32 = (fSlow21 != 0.0) as i32;
		let mut fSlow23: F32 = (iSlow22) as F32;
		let mut iSlow24: i32 = (fSlow23 != 0.0) as i32;
		let mut fSlow25: F32 = self.fHslider7;
		let mut fTemp38: F32 = self.fTempPerm5;
		let mut fSlow26: F32 = self.fEntry2;
		let mut iSlow27: i32 = (fSlow26 == 3.0) as i32;
		let mut fSlow28: F32 = (if iSlow27 != 0 {2} else {1}) as F32;
		let mut fTemp39: F32 = self.fTempPerm6;
		let mut fTemp40: F32 = self.fTempPerm7;
		let mut fTemp41: F32 = self.fTempPerm8;
		let mut iTemp42: i32 = self.iTempPerm9;
		let mut fTemp43: F32 = self.fTempPerm10;
		let mut fSlow29: F32 = F32::min(fSlow26, 4.0);
		let mut iSlow30: i32 = (fSlow29 == 4.0) as i32;
		let mut fSlow31: F32 = (iSlow30) as F32;
		let mut iSlow32: i32 = (fSlow31 != 0.0) as i32;
		let mut iSlow33: i32 = (fSlow26 >= 5.0) as i32;
		let mut iSlow34: i32 = (iSlow33 == 0) as i32;
		let mut fSlow35: F32 = (iSlow34) as F32;
		let mut iSlow36: i32 = (fSlow35 != 0.0) as i32;
		let mut iSlow37: i32 = (fSlow29 == 1.0) as i32;
		let mut fSlow38: F32 = (iSlow37) as F32;
		let mut iSlow39: i32 = (fSlow38 != 0.0) as i32;
		let mut fSlow40: F32 = self.fHslider8;
		let mut fTemp47: F32 = self.fTempPerm11;
		let mut fTemp48: F32 = self.fTempPerm12;
		let mut fTemp49: F32 = self.fTempPerm13;
		let mut fTemp50: F32 = self.fTempPerm14;
		let mut fSlow41: F32 = ((iSlow9 != 0) as i32) as u32 as F32;
		let mut iSlow42: i32 = (fSlow8 == 3.0) as i32;
		let mut fSlow43: F32 = (iSlow42) as F32;
		let mut iSlow44: i32 = (fSlow43 != 0.0) as i32;
		let mut iSlow45: i32 = (fSlow8 == 2.0) as i32;
		let mut fSlow46: F32 = (iSlow45) as F32;
		let mut iSlow47: i32 = (fSlow46 != 0.0) as i32;
		let mut iSlow48: i32 = (fSlow29 == 2.0) as i32;
		let mut fSlow49: F32 = (iSlow48) as F32;
		let mut iSlow50: i32 = (fSlow49 != 0.0) as i32;
		let mut iTemp51: i32 = self.iTempPerm15;
		let mut fTemp52: F32 = self.fTempPerm16;
		let mut fSlow51: F32 = self.fHslider9;
		let mut fSlow52: F32 = 12.9898 * fSlow51;
		let mut fSlow53: F32 = 43758.547 * F32::sin(fSlow52 + 234.699);
		let mut fSlow54: F32 = 43758.547 * F32::sin(fSlow52 + 7979.766);
		let mut fSlow55: F32 = 2.0 * (fSlow54 - F32::floor(fSlow54)) + -1.0;
		let mut fSlow56: F32 = self.fConst4 * (0.1 * F32::max(-1.0, F32::min(1.0, 0.70710677 * (fSlow55 + 2.0 * (fSlow53 - F32::floor(fSlow53)) + -1.0))) + 0.45);
		let mut fTemp53: F32 = self.fTempPerm17;
		let mut iSlow57: i32 = (409.0 * fSlow51 + 4e+03) as i32;
		let mut iTemp54: i32 = self.iTempPerm18;
		let mut fSlow58: F32 = self.fHslider10;
		let mut fSlow59: F32 = 43758.547 * F32::sin(fSlow52 + 156.466);
		let mut fSlow60: F32 = 0.15 * F32::max(-1.0, F32::min(1.0, 1.2803688 * (0.6 * fSlow55 + 0.5 * (2.0 * (fSlow59 - F32::floor(fSlow59)) + -1.0)))) + 0.25;
		let mut fSlow61: F32 = 1.0 / F32::tan(self.fConst15 * F32::max(1.0, fSlow60));
		let mut fSlow62: F32 = 1.0 - fSlow61;
		let mut fSlow63: F32 = self.fConst4 * F32::max(fSlow60, 0.0);
		let mut fTemp55: F32 = self.fTempPerm19;
		let mut fTemp56: F32 = self.fTempPerm20;
		let mut fSlow64: F32 = 1.0 / (fSlow61 + 1.0);
		let mut fSlow65: F32 = 43758.547 * F32::sin(fSlow52 + 78.233);
		let mut fSlow66: F32 = 43758.547 * F32::sin(fSlow52 + 7901.533);
		let mut fSlow67: F32 = 8.0 * F32::max(-1.0, F32::min(1.0, 1.0976425 * (0.7 * (2.0 * (fSlow66 - F32::floor(fSlow66)) + -1.0) + 0.3 * fSlow55 + 0.5 * (2.0 * (fSlow65 - F32::floor(fSlow65)) + -1.0))));
		let mut fTemp57: F32 = self.fTempPerm21;
		let mut fSlow68: F32 = self.fHslider11;
		let mut fSlow69: F32 = self.fEntry3;
		let mut fSlow70: F32 = 2.0 * ((fSlow69 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow69 == 1.0) as i32) as u32 as F32;
		let mut fSlow71: F32 = self.fHslider12;
		let mut fSlow72: F32 = 0.33333334 * F32::max(1e-05, self.fHslider13);
		let mut iSlow73: i32 = (F32::abs(fSlow72) < 1.1920929e-07) as i32;
		let mut fSlow74: F32 = if iSlow73 != 0 {0.0} else {F32::exp(-(self.fConst4 / if iSlow73 != 0 {1.0} else {fSlow72}))};
		let mut fSlow75: F32 = F32::log(self.fHslider14, std::f32::consts::E) * (1.0 - fSlow74);
		let mut fSlow76: F32 = F32::powf(2.0, self.fHslider15);
		let mut fTemp62: F32 = self.fTempPerm22;
		let mut iTemp63: i32 = self.iTempPerm23;
		let mut iTemp64: i32 = self.iTempPerm24;
		let mut fTemp65: F32 = self.fTempPerm25;
		let mut fSlow77: F32 = ((iSlow42 != 0) as i32) as u32 as F32;
		let mut iTemp66: i32 = self.iTempPerm26;
		let mut iTemp67: i32 = self.iTempPerm27;
		let mut fTemp68: F32 = self.fTempPerm28;
		let mut fSlow78: F32 = ((iSlow45 != 0) as i32) as u32 as F32;
		let mut fSlow79: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, self.fHslider16)) + -1.0));
		let mut fSlow80: F32 = fSlow14 / fSlow79;
		let mut fTemp69: F32 = self.fTempPerm29;
		let mut fSlow81: F32 = ((iSlow13 != 0) as i32) as u32 as F32;
		let mut fSlow82: F32 = 4.656613e-10 * fSlow17;
		let mut fTemp70: F32 = self.fTempPerm30;
		let mut fSlow83: F32 = ((iSlow16 != 0) as i32) as u32 as F32;
		let mut fTemp71: F32 = self.fTempPerm31;
		let mut fTemp72: F32 = self.fTempPerm32;
		let mut fTemp73: F32 = self.fTempPerm33;
		let mut fSlow84: F32 = ((iSlow2 != 0) as i32) as u32 as F32;
		let mut fSlow85: F32 = fSlow84 * (iSlow1) as F32;
		let mut iSlow86: i32 = (self.fEntry4 == 1.0) as i32;
		let mut iSlow87: i32 = (iSlow86 == 0) as i32;
		let mut iSlow88: i32 = (self.fEntry5 == 1.0) as i32;
		let mut iSlow89: i32 = (iSlow88 == 0) as i32;
		let mut iSlow90: i32 = iSlow89 | iSlow87;
		let mut fSlow91: F32 = (iSlow90) as F32;
		let mut iSlow92: i32 = (fSlow91 != 0.0) as i32;
		let mut iSlow93: i32 = iSlow88 | iSlow86;
		let mut fSlow94: F32 = (iSlow93) as F32;
		let mut iSlow95: i32 = (fSlow94 != 0.0) as i32;
		let mut fSlow96: F32 = self.fHslider17;
		let mut iTemp75: i32 = self.iTempPerm34;
		let mut fSlow97: F32 = self.fHslider18;
		let mut fTemp76: F32 = self.fTempPerm35;
		let mut fTemp77: F32 = self.fTempPerm36;
		let mut fTemp78: F32 = self.fTempPerm37;
		let mut fSlow98: F32 = (iSlow87) as F32;
		let mut fTemp79: F32 = self.fTempPerm38;
		let mut fSlow99: F32 = (iSlow89) as F32;
		let mut fTemp80: F32 = self.fTempPerm39;
		let mut fTemp81: F32 = self.fTempPerm40;
		let mut iTemp82: i32 = self.iTempPerm41;
		let mut fTemp83: F32 = self.fTempPerm42;
		let mut fTemp84: F32 = self.fTempPerm43;
		let mut fTemp85: F32 = self.fTempPerm44;
		let mut fTemp86: F32 = self.fTempPerm45;
		let mut fTemp87: F32 = self.fTempPerm46;
		let mut fTemp88: F32 = self.fTempPerm47;
		let mut fTemp89: F32 = self.fTempPerm48;
		let mut fTemp90: F32 = self.fTempPerm49;
		let mut fSlow100: F32 = ((iSlow90 != 0) as i32) as u32 as F32;
		let mut fTemp92: F32 = self.fTempPerm50;
		let mut fTemp93: F32 = self.fTempPerm51;
		let mut fTemp94: F32 = self.fTempPerm52;
		let mut fTemp95: F32 = self.fTempPerm53;
		let mut fTemp96: F32 = self.fTempPerm54;
		let mut fTemp97: F32 = self.fTempPerm55;
		let mut iSlow101: i32 = (iSlow33 == 1) as i32;
		let mut fSlow102: F32 = (iSlow101) as F32;
		let mut iSlow103: i32 = (fSlow102 != 0.0) as i32;
		let mut iSlow104: i32 = (F32::max(0.0, fSlow26 + -5.0)) as i32;
		let mut iSlow105: i32 = (fSlow29 == 3.0) as i32;
		let mut fSlow106: F32 = (iSlow105) as F32;
		let mut iSlow107: i32 = (fSlow106 != 0.0) as i32;
		let mut iSlow108: i32 = (fSlow29 == 0.0) as i32;
		let mut fSlow109: F32 = (iSlow108) as F32;
		let mut iSlow110: i32 = (fSlow109 != 0.0) as i32;
		let mut fSlow111: F32 = self.fHslider19;
		let mut fTemp98: F32 = self.fTempPerm56;
		let mut fTemp99: F32 = self.fTempPerm57;
		let mut fTemp100: F32 = self.fTempPerm58;
		let mut fTemp101: F32 = self.fTempPerm59;
		let mut fSlow112: F32 = ((iSlow101 != 0) as i32) as u32 as F32;
		let mut fTemp102: F32 = self.fTempPerm60;
		let mut fTemp103: F32 = self.fTempPerm61;
		let mut fTemp104: F32 = self.fTempPerm62;
		let mut fSlow113: F32 = 0.03125 * fSlow31;
		let mut fTemp105: F32 = self.fTempPerm63;
		let mut fSlow114: F32 = ((iSlow30 != 0) as i32) as u32 as F32;
		let mut fTemp106: F32 = self.fTempPerm64;
		let mut fSlow115: F32 = 0.25 * self.fEntry6 + 2.0;
		let mut fSlow116: F32 = 1.0 / fSlow115;
		let mut fTemp107: F32 = self.fTempPerm65;
		let mut fSlow117: F32 = self.fHslider20;
		let mut fTemp108: F32 = self.fTempPerm66;
		let mut fTemp109: F32 = self.fTempPerm67;
		let mut fSlow118: F32 = self.fConst4 * fSlow115;
		let mut fTemp110: F32 = self.fTempPerm68;
		let mut fTemp111: F32 = self.fTempPerm69;
		let mut fTemp112: F32 = self.fTempPerm70;
		let mut fTemp113: F32 = self.fTempPerm71;
		let mut fSlow119: F32 = ((iSlow105 != 0) as i32) as u32 as F32;
		let mut fTemp114: F32 = self.fTempPerm72;
		let mut fTemp115: F32 = self.fTempPerm73;
		let mut fSlow120: F32 = ((iSlow48 != 0) as i32) as u32 as F32;
		let mut fTemp116: F32 = self.fTempPerm74;
		let mut fTemp117: F32 = self.fTempPerm75;
		let mut fTemp118: F32 = self.fTempPerm76;
		let mut fTemp119: F32 = self.fTempPerm77;
		let mut fTemp120: F32 = self.fTempPerm78;
		let mut fSlow121: F32 = ((iSlow37 != 0) as i32) as u32 as F32;
		let mut fTemp121: F32 = self.fTempPerm79;
		let mut fSlow122: F32 = ((iSlow108 != 0) as i32) as u32 as F32;
		let mut fTemp122: F32 = self.fTempPerm80;
		let mut fSlow123: F32 = ((iSlow34 != 0) as i32) as u32 as F32;
		let mut fTemp123: F32 = self.fTempPerm81;
		let mut fTemp124: F32 = self.fTempPerm82;
		let mut fTemp125: F32 = self.fTempPerm83;
		let mut fSlow124: F32 = self.fEntry7;
		let mut fSlow125: F32 = ((iSlow22 != 0) as i32) as u32 as F32;
		let mut fSlow126: F32 = fSlow23 * fSlow125 * (1.0 - fSlow124);
		let mut fSlow127: F32 = ((iSlow86 == 1) as i32) as u32 as F32;
		let mut fSlow128: F32 = ((iSlow88 == 1) as i32) as u32 as F32;
		let mut fTemp128: F32 = self.fTempPerm84;
		let mut fTemp129: F32 = self.fTempPerm85;
		let mut fTemp130: F32 = self.fTempPerm86;
		let mut fTemp131: F32 = self.fTempPerm87;
		let mut fTemp132: F32 = self.fTempPerm88;
		let mut fTemp133: F32 = self.fTempPerm89;
		let mut fTemp134: F32 = self.fTempPerm90;
		let mut fSlow129: F32 = ((iSlow93 != 0) as i32) as u32 as F32;
		let mut fTemp135: F32 = self.fTempPerm91;
		let mut fTemp136: F32 = self.fTempPerm92;
		let mut fTemp137: F32 = self.fTempPerm93;
		let mut fTemp138: F32 = self.fTempPerm94;
		let mut fTemp139: F32 = self.fTempPerm95;
		let mut fTemp140: F32 = self.fTempPerm96;
		let mut fTemp141: F32 = self.fTempPerm97;
		let mut fTemp142: F32 = self.fTempPerm98;
		let mut fTemp143: F32 = self.fTempPerm99;
		let mut fTemp144: F32 = self.fTempPerm100;
		let mut fTemp145: F32 = self.fTempPerm101;
		let mut fTemp146: F32 = self.fTempPerm102;
		let mut fTemp147: F32 = self.fTempPerm103;
		let mut fTemp148: F32 = self.fTempPerm104;
		let mut fTemp149: F32 = self.fTempPerm105;
		let mut fTemp151: F32 = self.fTempPerm106;
		let mut fTemp152: F32 = self.fTempPerm107;
		let mut fTemp153: F32 = self.fTempPerm108;
		let mut fTemp154: F32 = self.fTempPerm109;
		let mut fTemp155: F32 = self.fTempPerm110;
		let mut fTemp156: F32 = self.fTempPerm111;
		let mut fTemp157: F32 = self.fTempPerm112;
		let mut fTemp158: F32 = self.fTempPerm113;
		let mut fSlow130: F32 = fSlow124 * fSlow23 * fSlow125;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3);
		for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			self.fVec1[0] = fSlow5;
			self.iVec2[0] = 1;
			let mut iTemp0: i32 = (fSlow5 > self.fVec1[1]) as i32;
			if iSlow4 != 0 {
				self.fVec3[0] = fSlow6;
				fTemp1 = if ((fSlow6 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec0[1] + -1.0};
				self.fRec0[0] = fTemp1;
				self.fRec1[0] = if iTemp0 != 0 {fSlow6} else {if (fTemp1 > 0.0) as i32 != 0 {self.fRec1[1] + (fSlow6 - self.fRec1[1]) / F32::max(1.0, fTemp1)} else {fSlow6}};
				fTemp2 = 0.025 * self.fRec1[0];
			}
				if iSlow4 & iSlow11 != 0 {
					self.fVec4[0] = fSlow12;
					fTemp3 = if ((fSlow12 != self.fVec4[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec6[1] + -1.0};
					self.fRec6[0] = fTemp3;
					self.fRec7[0] = if iTemp0 != 0 {fSlow12} else {if (fTemp3 > 0.0) as i32 != 0 {self.fRec7[1] + (fSlow12 - self.fRec7[1]) / F32::max(1.0, fTemp3)} else {fSlow12}};
				}
					self.fVec5[0] = fSlow19;
					let mut fTemp4: F32 = if ((fSlow19 != self.fVec5[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec9[1] + -1.0};
					self.fRec9[0] = fTemp4;
					self.fRec10[0] = if iTemp0 != 0 {fSlow19} else {if (fTemp4 > 0.0) as i32 != 0 {self.fRec10[1] + (fSlow19 - self.fRec10[1]) / F32::max(1.0, fTemp4)} else {fSlow19}};
					let mut fTemp5: F32 = F32::max(0.0, F32::min(1.0, self.fRec10[0]));
					let mut fTemp6: F32 = 4.0 * fTemp5;
					let mut fTemp7: F32 = F32::max(0.0, F32::min(2.0, F32::floor(fTemp6 + -0.5)));
					let mut fTemp8: F32 = fTemp7 + 1.0;
					let mut iTemp9: i32 = (fTemp8 == 1.0) as i32;
					let mut fTemp10: F32 = (iTemp9) as F32;
					let mut iTemp11: i32 = (fTemp10 != 0.0) as i32;
					let mut iTemp12: i32 = (fTemp8 == 0.0) as i32;
					let mut fTemp13: F32 = (iTemp12) as F32;
					let mut iTemp14: i32 = (fTemp13 != 0.0) as i32;
					let mut iTemp15: i32 = (fTemp8 == 2.0) as i32;
					let mut fTemp16: F32 = (iTemp15) as F32;
					let mut iTemp17: i32 = (fTemp16 != 0.0) as i32;
					let mut iTemp18: i32 = (fTemp8 == 3.0) as i32;
					let mut fTemp19: F32 = (iTemp18) as F32;
					let mut iTemp20: i32 = (fTemp19 != 0.0) as i32;
					let mut iTemp21: i32 = (fTemp7 == 1.0) as i32;
					let mut fTemp22: F32 = (iTemp21) as F32;
					let mut iTemp23: i32 = (fTemp22 != 0.0) as i32;
					let mut iTemp24: i32 = (fTemp7 == 0.0) as i32;
					let mut fTemp25: F32 = (iTemp24) as F32;
					let mut iTemp26: i32 = (fTemp25 != 0.0) as i32;
					let mut iTemp27: i32 = (fTemp7 == 2.0) as i32;
					let mut fTemp28: F32 = (iTemp27) as F32;
					let mut iTemp29: i32 = (fTemp28 != 0.0) as i32;
					let mut iTemp30: i32 = (fTemp7 == 3.0) as i32;
					let mut fTemp31: F32 = (iTemp30) as F32;
					let mut iTemp32: i32 = (fTemp31 != 0.0) as i32;
					let mut iTemp33: i32 = i32::wrapping_sub(1, iTemp0);
					let mut fTemp34: F32 = (iTemp33) as F32;
					let mut fTemp35: F32 = 1.0 - self.fConst3 * fTemp34;
					if iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | (iTemp14 | iTemp11)))))) != 0 {
						self.fRec11[0] = fSlow20 * fTemp35 + self.fConst3 * fTemp34 * self.fRec11[1];
						iTemp36 = (self.fRec11[0] > 0.0) as i32;
						fTemp37 = (iTemp36) as F32;
					}
						self.fVec6[0] = fSlow21;
						if iSlow24 != 0 {
							self.fVec7[0] = fSlow25;
							fTemp38 = if ((fSlow25 != self.fVec7[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec12[1] + -1.0};
							self.fRec12[0] = fTemp38;
							self.fRec13[0] = if iTemp0 != 0 {fSlow25} else {if (fTemp38 > 0.0) as i32 != 0 {self.fRec13[1] + (fSlow25 - self.fRec13[1]) / F32::max(1.0, fTemp38)} else {fSlow25}};
							fTemp39 = fSlow28 * F32::max(0.005, 0.001 * self.fRec13[0]);
							self.iRec14[0] = i32::wrapping_mul(iTemp33, i32::wrapping_add(self.iRec14[1], 1));
							fTemp40 = (self.iRec14[0]) as F32;
							fTemp41 = self.fConst4 * fTemp40;
							self.iRec15[0] = std::cmp::max(iTemp0, self.iRec15[1]);
							iTemp42 = i32::wrapping_mul(self.iRec15[0], (fTemp41 < (fTemp39 + 0.05)) as i32);
							fTemp43 = (iTemp42) as F32;
						}
							self.fVec8[0] = fSlow40;
							let mut fTemp44: F32 = if ((fSlow40 != self.fVec8[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec16[1] + -1.0};
							self.fRec16[0] = fTemp44;
							self.fRec17[0] = if iTemp0 != 0 {fSlow40} else {if (fTemp44 > 0.0) as i32 != 0 {self.fRec17[1] + (fSlow40 - self.fRec17[1]) / F32::max(1.0, fTemp44)} else {fSlow40}};
							let mut iTemp45: i32 = (F32::abs(self.fRec17[0]) > 0.0001) as i32;
							let mut fTemp46: F32 = (iTemp45) as F32;
							if ((fTemp46 != 0.0) as i32) | ((((fTemp37 != 0.0) as i32) & iTemp32) | ((iSlow24 & (iSlow36 & (iSlow39 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32)))) | ((((fTemp37 != 0.0) as i32) & iTemp29) | ((((fTemp37 != 0.0) as i32) & iTemp26) | ((((fTemp37 != 0.0) as i32) & iTemp23) | ((((fTemp37 != 0.0) as i32) & iTemp20) | ((((fTemp37 != 0.0) as i32) & iTemp17) | ((((fTemp37 != 0.0) as i32) & iTemp14) | ((((fTemp37 != 0.0) as i32) & iTemp11) | ((iSlow4 & iSlow18) | ((iSlow4 & iSlow15) | (iSlow4 & iSlow11))))))))))))) != 0 {
								self.iRec8[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec8[1]), 12345);
								fTemp47 = (self.iRec8[0]) as F32;
							}
								if ((fTemp46 != 0.0) as i32) | ((((fTemp37 != 0.0) as i32) & iTemp32) | ((iSlow24 & (iSlow36 & (iSlow39 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32)))) | ((((fTemp37 != 0.0) as i32) & iTemp29) | ((((fTemp37 != 0.0) as i32) & iTemp26) | ((((fTemp37 != 0.0) as i32) & iTemp23) | ((((fTemp37 != 0.0) as i32) & iTemp20) | ((((fTemp37 != 0.0) as i32) & iTemp17) | ((((fTemp37 != 0.0) as i32) & iTemp14) | ((((fTemp37 != 0.0) as i32) & iTemp11) | ((iSlow4 & iSlow15) | (iSlow4 & iSlow11)))))))))))) != 0 {
									fTemp48 = 4.656613e-10 * fTemp47;
								}
									if iSlow4 & iSlow11 != 0 {
										fTemp49 = fTemp47 * ((F32::abs(fTemp48) > (1.0 - 0.002 * F32::max(0.0, F32::min(1.0, self.fRec7[0])))) as i32) as u32 as F32;
										self.fVec9[0] = fTemp49;
										self.fRec5[0] = self.fConst5 * (1.8626451e-09 * (fTemp49 + self.fVec9[1]) - self.fConst2 * self.fRec5[1]);
										self.fRec4[0] = -(self.fConst8 * (self.fConst7 * self.fRec4[1] - self.fConst6 * (self.fRec5[0] - self.fRec5[1])));
										fTemp50 = fSlow10 * self.fRec4[0];
									}
										if ((fTemp46 != 0.0) as i32) | ((((fTemp37 != 0.0) as i32) & iTemp32) | ((iSlow24 & (iSlow36 & (iSlow50 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32)))) | ((((fTemp37 != 0.0) as i32) & iTemp29) | ((((fTemp37 != 0.0) as i32) & iTemp26) | ((((fTemp37 != 0.0) as i32) & iTemp23) | ((((fTemp37 != 0.0) as i32) & iTemp20) | ((((fTemp37 != 0.0) as i32) & iTemp17) | ((((fTemp37 != 0.0) as i32) & iTemp14) | ((((fTemp37 != 0.0) as i32) & iTemp11) | ((iSlow4 & iSlow15) | ((iSlow4 & iSlow47) | (iSlow4 & iSlow44))))))))))))) != 0 {
											iTemp51 = i32::wrapping_sub(1, self.iVec2[1]);
										}
											if (fTemp46 != 0.0) as i32 != 0 {
												fTemp52 = if iTemp51 != 0 {0.0} else {self.fConst9 + self.fRec21[1]};
												self.fRec21[0] = fTemp52 - F32::floor(fTemp52);
												fTemp53 = if iTemp51 != 0 {0.0} else {fSlow56 + self.fRec22[1]};
												self.fRec22[0] = fTemp53 - F32::floor(fTemp53);
												self.iRec24[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec24[1], i32::wrapping_mul(iSlow57, iTemp51))), 12345);
												iTemp54 = i32::wrapping_mul(1664525, self.iRec24[0] ^ (self.iRec24[0] >> 15));
												self.fRec23[0] = self.fConst11 * self.fRec23[1] + self.fConst13 * (i32::wrapping_mul(1013904223, iTemp54 ^ (iTemp54 >> 13))) as F32;
												self.fRec25[0] = fSlow58 * fTemp35 + self.fConst3 * fTemp34 * self.fRec25[1];
												fTemp55 = if iTemp51 != 0 {0.0} else {fSlow63 + self.fRec28[1]};
												self.fRec28[0] = fTemp55 - F32::floor(fTemp55);
												self.fRec27[0] = if (self.fRec28[0] < self.fRec28[1]) as i32 != 0 {fTemp48} else {self.fRec27[1]};
												fTemp56 = F32::floor(32768.0 * self.fRec27[0] + 0.5);
												self.fVec11[0] = fTemp56;
												self.fRec26[0] = fSlow64 * (3.0517578e-05 * (fTemp56 + self.fVec11[1]) - fSlow62 * self.fRec26[1]);
												fTemp57 = fTemp46 * (self.fRec17[0] * (fSlow67 + 2e+01 * self.fRec26[0] + self.fConst14 * self.fRec25[0] * self.fRec23[0]) + MixDsp_faustpower2_f(self.fRec17[0]) * (18.0 * unsafe { ftbl0MixDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec22[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl0MixDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec21[0]) as i32, 65535))) as usize] }));
											}
												self.fRec29[0] = fSlow68 * fTemp35 + self.fConst3 * fTemp34 * self.fRec29[1];
												self.fVec12[0] = fSlow71;
												let mut fTemp58: F32 = if ((fSlow71 != self.fVec12[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec30[1] + -1.0};
												self.fRec30[0] = fTemp58;
												self.fRec31[0] = if iTemp0 != 0 {fSlow71} else {if (fTemp58 > 0.0) as i32 != 0 {self.fRec31[1] + (fSlow71 - self.fRec31[1]) / F32::max(1.0, fTemp58)} else {fSlow71}};
												let mut fTemp59: F32 = F32::exp(0.057762265 * (self.fRec31[0] + fSlow70 * self.fRec29[0] + 0.01 * ((iTemp45 != 0) as i32) as u32 as F32 * fTemp57));
												self.fRec32[0] = fSlow75 + fSlow74 * self.fRec32[1];
												let mut fTemp60: F32 = F32::exp(self.fRec32[0]);
												let mut fTemp61: F32 = fTemp60 * fTemp59;
												if (iSlow4 & iSlow15) | ((iSlow4 & iSlow47) | (iSlow4 & iSlow44)) != 0 {
													fTemp62 = if iTemp51 != 0 {0.0} else {self.fRec19[1] + self.fConst4 * F32::max(fSlow76 * fTemp61, 0.0)};
													self.fRec19[0] = fTemp62 - F32::floor(fTemp62);
													iTemp63 = (self.fRec19[0] < self.fRec19[1]) as i32;
												}
													if iSlow4 & iSlow44 != 0 {
														iTemp64 = if (self.iRec18[1] == 0) as i32 != 0 {1} else {self.iRec18[1]};
														self.iRec18[0] = if iTemp63 != 0 {(iTemp64 >> 1) | (((iTemp64 ^ (iTemp64 >> 6)) & 1) << 14)} else {iTemp64};
														fTemp65 = fSlow43 * if self.iRec18[0] & 1 != 0 {1.0} else {-1.0};
													}
														if iSlow4 & iSlow47 != 0 {
															iTemp66 = if (self.iRec33[1] == 0) as i32 != 0 {1} else {self.iRec33[1]};
															iTemp67 = iTemp66 >> 1;
															self.iRec33[0] = if iTemp63 != 0 {iTemp67 | (((iTemp66 ^ iTemp67) & 1) << 14)} else {iTemp66};
															fTemp68 = fSlow46 * if self.iRec33[0] & 1 != 0 {1.0} else {-1.0};
														}
															if iSlow4 & iSlow15 != 0 {
																self.fRec34[0] = if iTemp63 != 0 {fTemp48} else {self.fRec34[1]};
																fTemp69 = fSlow80 * F32::floor(fSlow79 * self.fRec34[0] + 0.5);
															}
																if iSlow4 & iSlow18 != 0 {
																	fTemp70 = fSlow82 * fTemp47;
																}
																	if iSlow4 != 0 {
																		fTemp71 = fSlow83 * fTemp70 + fSlow81 * fTemp69 + fSlow78 * fTemp68 + fSlow77 * fTemp65 + fSlow41 * fTemp50;
																		self.fVec13[0] = fTemp71;
																		self.fRec3[0] = -(self.fConst19 * (self.fConst18 * self.fRec3[1] - (fTemp71 + self.fVec13[1])));
																		fTemp72 = fTemp71 + self.fRec3[0] * (F32::powf(1e+01, -fTemp2) + -1.0);
																		self.fVec14[0] = fTemp72;
																		self.fRec2[0] = -(self.fConst19 * (self.fConst18 * self.fRec2[1] - self.fConst17 * (fTemp72 - self.fVec14[1])));
																		fTemp73 = fSlow3 * (self.fRec3[0] * (F32::powf(1e+01, -0.025 * self.fRec1[0]) + -1.0) + fTemp71 + self.fRec2[0] * (F32::powf(1e+01, fTemp2) + -1.0));
																	}
																		let mut fTemp74: F32 = if ((fSlow0 != self.fVec0[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec35[1] + -1.0};
																		self.fRec35[0] = fTemp74;
																		self.fRec36[0] = if iTemp0 != 0 {fSlow0} else {if (fTemp74 > 0.0) as i32 != 0 {self.fRec36[1] + (fSlow0 - self.fRec36[1]) / F32::max(1.0, fTemp74)} else {fSlow0}};
																		if (iSlow95 & iTemp32) | ((iSlow95 & iTemp29) | ((iSlow95 & iTemp26) | ((iSlow95 & iTemp23) | ((iSlow95 & iTemp20) | ((iSlow95 & iTemp17) | ((iSlow95 & iTemp14) | ((iSlow95 & iTemp11) | iSlow92))))))) != 0 {
																			self.fRec37[0] = fSlow96 * fTemp35 + self.fConst3 * fTemp34 * self.fRec37[1];
																			iTemp75 = (self.fRec37[0] > 0.0) as i32;
																			self.fVec15[0] = fSlow97;
																			fTemp76 = if ((fSlow97 != self.fVec15[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec38[1] + -1.0};
																			self.fRec38[0] = fTemp76;
																			self.fRec39[0] = if iTemp0 != 0 {fSlow97} else {if (fTemp76 > 0.0) as i32 != 0 {self.fRec39[1] + (fSlow97 - self.fRec39[1]) / F32::max(1.0, fTemp76)} else {fSlow97}};
																			fTemp77 = 1.5707964 * self.fRec39[0];
																			fTemp78 = *input1 * F32::sin(fTemp77);
																			fTemp79 = *input0 * F32::cos(fTemp77);
																		}
																			if iSlow92 != 0 {
																				fTemp80 = fSlow99 * fTemp79 + fSlow98 * fTemp78;
																			}
																				if (iSlow95 & iTemp32) | ((iSlow95 & iTemp29) | ((iSlow95 & iTemp26) | ((iSlow95 & iTemp23) | ((iSlow95 & iTemp20) | ((iSlow95 & iTemp17) | ((iSlow95 & iTemp14) | ((iSlow95 & iTemp11) | iSlow92))))))) != 0 {
																					fTemp81 = (iTemp75) as F32;
																					iTemp82 = (fTemp81 != 0.0) as i32;
																				}
																					if (iTemp82 & (iSlow95 & iTemp32)) | ((iTemp82 & (iSlow95 & iTemp29)) | ((iTemp82 & (iSlow95 & iTemp26)) | ((iTemp82 & (iSlow95 & iTemp23)) | ((iTemp82 & (iSlow95 & iTemp20)) | ((iTemp82 & (iSlow95 & iTemp17)) | ((iTemp82 & (iSlow95 & iTemp14)) | ((iTemp82 & (iSlow95 & iTemp11)) | (iSlow92 & iTemp82)))))))) != 0 {
																						fTemp83 = F32::max(self.fRec37[0], 0.001);
																					}
																						if iSlow92 & iTemp82 != 0 {
																							fTemp84 = fTemp83 * fTemp80;
																							self.fVec16[0] = fTemp84;
																							fTemp85 = 1.13003 * (fTemp84 + self.fVec16[1]);
																							self.fRec41[0] = self.fConst23 * (fTemp85 - self.fConst22 * self.fRec41[1]);
																							self.fRec42[0] = self.fConst28 * (fTemp85 - self.fConst27 * self.fRec42[1]);
																							fTemp86 = 1.13003 * fTemp84;
																							fTemp87 = (self.fConst33 * (F32::tanh(fTemp86 + 0.08) - self.fConst32) + 0.4 * (self.fConst31 * (F32::tanh(0.9 * (fTemp86 - self.fRec42[0]) + 0.05) - self.fConst30) + self.fConst29 * (F32::tanh(1.25 * (self.fRec42[0] - self.fRec41[0]) + -0.08) - self.fConst25) + self.fConst24 * (F32::tanh(1.5 * self.fRec41[0] + 0.12) - self.fConst20))) / fTemp83;
																							self.fVec17[0] = fTemp87;
																							self.fRec40[0] = self.fConst36 * (self.fConst35 * self.fRec40[1] + 1.25 * (fTemp87 - self.fVec17[1]));
																							fTemp88 = self.fRec40[0] * fTemp81;
																						}
																							if (iSlow95 & iTemp32) | ((iSlow95 & iTemp29) | ((iSlow95 & iTemp26) | ((iSlow95 & iTemp23) | ((iSlow95 & iTemp20) | ((iSlow95 & iTemp17) | ((iSlow95 & iTemp14) | ((iSlow95 & iTemp11) | iSlow92))))))) != 0 {
																								fTemp89 = ((iTemp75 != 0) as i32) as u32 as F32;
																							}
																								if iSlow92 != 0 {
																									fTemp90 = fSlow91 * if iTemp75 != 0 {fTemp89 * fTemp88} else {1.4125376 * fTemp80};
																								}
																									let mut fTemp91: F32 = F32::powf(1e+01, 0.6 * (fTemp5 + MixDsp_faustpower4_f(fTemp5)));
																									if (((fTemp37 != 0.0) as i32) & iTemp32) | ((((fTemp37 != 0.0) as i32) & iTemp29) | ((((fTemp37 != 0.0) as i32) & iTemp26) | ((((fTemp37 != 0.0) as i32) & iTemp23) | ((((fTemp37 != 0.0) as i32) & iTemp20) | ((((fTemp37 != 0.0) as i32) & iTemp17) | ((((fTemp37 != 0.0) as i32) & iTemp14) | (((fTemp37 != 0.0) as i32) & iTemp11))))))) != 0 {
																										fTemp92 = if iTemp51 != 0 {0.0} else {self.fConst39 + self.fRec46[1]};
																										self.fRec46[0] = fTemp92 - F32::floor(fTemp92);
																										self.fRec45[0] = if (self.fRec46[0] < self.fRec46[1]) as i32 != 0 {fTemp48} else {self.fRec45[1]};
																										fTemp93 = F32::floor(32768.0 * self.fRec45[0] + 0.5);
																										self.fVec18[0] = fTemp93;
																										self.fRec44[0] = self.fConst40 * (3.0517578e-05 * (fTemp93 + self.fVec18[1]) - self.fConst38 * self.fRec44[1]);
																										fTemp94 = 0.3 * self.fRec11[0] * self.fRec44[0] * fTemp37;
																									}
																										if iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | (iTemp14 | iTemp11)))))) != 0 {
																											fTemp95 = ((iTemp36 != 0) as i32) as u32 as F32 * fTemp94;
																										}
																											if iTemp23 | iTemp11 != 0 {
																												fTemp96 = F32::max(-1.0, F32::min(1.0, fTemp95));
																											}
																												if iTemp11 != 0 {
																													fTemp97 = fTemp96 * fTemp10;
																												}
																													if (iSlow24 & (iSlow36 & (iSlow110 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow39 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow50 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow107 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32)))) | (iSlow24 & (iSlow103 & ((fTemp43 != 0.0) as i32))))))) != 0 {
																														self.fVec19[0] = fSlow111;
																														fTemp98 = if ((fSlow111 != self.fVec19[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec48[1] + -1.0};
																														self.fRec48[0] = fTemp98;
																														self.fRec49[0] = if iTemp0 != 0 {fSlow111} else {if (fTemp98 > 0.0) as i32 != 0 {self.fRec49[1] + (fSlow111 - self.fRec49[1]) / F32::max(1.0, fTemp98)} else {fSlow111}};
																														fTemp99 = F32::exp(0.057762265 * self.fRec49[0]);
																													}
																														if (iSlow24 & (iSlow36 & (iSlow110 & ((fTemp43 != 0.0) as i32)))) | (iSlow24 & (iSlow103 & ((fTemp43 != 0.0) as i32))) != 0 {
																															fTemp100 = fTemp40 * fTemp99;
																														}
																															if iSlow24 & (iSlow103 & ((fTemp43 != 0.0) as i32)) != 0 {
																																fTemp101 = fSlow102 * tr_read(iSlow104, self.fConst41 * fTemp100) * F32::min(1.0, F32::max(0.0, 2.5 * ((fTemp39 - fTemp41) / fTemp39)));
																															}
																																if iSlow24 & (iSlow36 & ((fTemp43 != 0.0) as i32)) != 0 {
																																	fTemp102 = fTemp40 / fTemp39;
																																}
																																	if (iSlow24 & (iSlow36 & (iSlow50 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow107 & ((fTemp43 != 0.0) as i32)))) | (iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32))))) != 0 {
																																		fTemp103 = fTemp60 * fTemp99 * fTemp59;
																																	}
																																		if iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32))) != 0 {
																																			fTemp104 = if iTemp51 != 0 {0.0} else {self.fRec51[1] + self.fConst4 * F32::max(4.0 * fTemp103, 0.0)};
																																			self.fRec51[0] = fTemp104 - F32::floor(fTemp104);
																																			self.fRec50[0] = if (self.fRec51[0] < self.fRec51[1]) as i32 != 0 {fTemp48} else {self.fRec50[1]};
																																			fTemp105 = fSlow113 * F32::floor(32.0 * self.fRec50[0] + 0.5);
																																		}
																																			if iSlow24 & (iSlow36 & (iSlow107 & ((fTemp43 != 0.0) as i32))) != 0 {
																																				fTemp106 = F32::max(0.0, 1.0 - self.fConst4 * fTemp102);
																																				fTemp107 = F32::max(0.0, fSlow116 * ((self.fConst16 - fTemp103) / fTemp103) + -1.0);
																																				self.fVec20[0] = fSlow117;
																																				fTemp108 = if ((fSlow117 != self.fVec20[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec52[1] + -1.0};
																																				self.fRec52[0] = fTemp108;
																																				self.fRec53[0] = if iTemp0 != 0 {fSlow117} else {if (fTemp108 > 0.0) as i32 != 0 {self.fRec53[1] + (fSlow117 - self.fRec53[1]) / F32::max(1.0, fTemp108)} else {fSlow117}};
																																				fTemp109 = F32::min(fTemp107, 8.0 * F32::max(0.05, self.fRec53[0]) * MixDsp_faustpower3_f(fTemp106));
																																				fTemp110 = fTemp34 * self.fRec54[1] + fSlow118 * fTemp103;
																																				self.fRec54[0] = fTemp110 - F32::floor(fTemp110);
																																				fTemp111 = 6.2831855 * self.fRec54[0];
																																				fTemp112 = fTemp34 * self.fRec55[1] + self.fConst4 * fTemp103;
																																				self.fRec55[0] = fTemp112 - F32::floor(fTemp112);
																																				fTemp113 = fSlow106 * F32::sin(6.2831855 * self.fRec55[0] + fTemp109 * F32::sin(fTemp111 + F32::sin(fTemp111) * F32::min(F32::max(0.0, fTemp107 / F32::max(fTemp109, 1e-09) + -1.0), 8.0 * if (self.fRec53[0] < 0.25) as i32 != 0 {0.0} else {if (self.fRec53[0] < 0.75) as i32 != 0 {0.5 * (self.fRec53[0] + -0.25)} else {3.0 * (self.fRec53[0] + -0.75) + 0.25}} * MixDsp_faustpower4_f(fTemp106)))) * MixDsp_faustpower2_f(fTemp106);
																																			}
																																				if iSlow24 & (iSlow36 & (iSlow50 & ((fTemp43 != 0.0) as i32))) != 0 {
																																					fTemp114 = if iTemp51 != 0 {0.0} else {self.fRec56[1] + self.fConst4 * fTemp103 * (7.0 * F32::exp(-(self.fConst43 * fTemp40)) + 1.0)};
																																					self.fRec56[0] = fTemp114 - F32::floor(fTemp114);
																																					fTemp115 = fSlow49 * unsafe { ftbl0MixDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec56[0]) as i32, 65535))) as usize] };
																																				}
																																					if iSlow24 & (iSlow36 & (iSlow39 & ((fTemp43 != 0.0) as i32))) != 0 {
																																						fTemp116 = F32::tan(self.fConst15 * F32::min(8e+02 * fTemp99, self.fConst44));
																																						fTemp117 = 1.0 / fTemp116;
																																						fTemp118 = (fTemp117 + 1.4142135) / fTemp116 + 1.0;
																																						fTemp119 = MixDsp_faustpower2_f(fTemp116);
																																						self.fRec57[0] = fTemp48 - (self.fRec57[2] * ((fTemp117 + -1.4142135) / fTemp116 + 1.0) + 2.0 * self.fRec57[1] * (1.0 - 1.0 / fTemp119)) / fTemp118;
																																						fTemp120 = fSlow38 * ((self.fRec57[2] + (self.fRec57[0] - 2.0 * self.fRec57[1])) / (fTemp119 * fTemp118));
																																					}
																																						if iSlow24 & (iSlow36 & (iSlow110 & ((fTemp43 != 0.0) as i32))) != 0 {
																																							fTemp121 = fSlow109 * F32::sin(self.fConst46 * fTemp100) * F32::exp(-(self.fConst45 * fTemp40));
																																						}
																																							if iSlow24 & (iSlow36 & ((fTemp43 != 0.0) as i32)) != 0 {
																																								fTemp122 = fSlow35 * (fSlow122 * fTemp121 + fSlow121 * fTemp120 + fSlow120 * fTemp115 + fSlow119 * fTemp113 + fSlow114 * fTemp105) * if iSlow27 != 0 {1.0} else {F32::exp(-(self.fConst42 * fTemp102)) * ((fTemp41 < fTemp39) as i32) as u32 as F32};
																																							}
																																								if iSlow24 & ((fTemp43 != 0.0) as i32) != 0 {
																																									fTemp123 = (self.iRec15[0]) as F32 * (fSlow123 * fTemp122 + fSlow112 * fTemp101);
																																									self.fVec21[0] = fTemp123;
																																									self.fRec47[0] = 0.995 * self.fRec47[1] + fTemp123 - self.fVec21[1];
																																									fTemp124 = self.fRec47[0] * fTemp43;
																																								}
																																									if iSlow24 != 0 {
																																										fTemp125 = fSlow23 * fTemp124 * ((iTemp42 != 0) as i32) as u32 as F32;
																																									}
																																										let mut fTemp126: F32 = if ((fSlow21 != self.fVec6[1]) as i32) | iTemp0 != 0 {fSlow7} else {self.fRec58[1] + -1.0};
																																										self.fRec58[0] = fTemp126;
																																										self.fRec59[0] = if iTemp0 != 0 {fSlow21} else {if (fTemp126 > 0.0) as i32 != 0 {self.fRec59[1] + (fSlow21 - self.fRec59[1]) / F32::max(1.0, fTemp126)} else {fSlow21}};
																																										let mut fTemp127: F32 = self.fRec59[0] * fTemp125;
																																										if (iSlow95 & iTemp32) | ((iSlow95 & iTemp29) | ((iSlow95 & iTemp26) | ((iSlow95 & iTemp23) | ((iSlow95 & iTemp20) | ((iSlow95 & iTemp17) | ((iSlow95 & iTemp14) | (iSlow95 & iTemp11))))))) != 0 {
																																											fTemp128 = fSlow128 * fTemp79 + fSlow127 * fTemp78;
																																										}
																																											if (iTemp82 & (iSlow95 & iTemp32)) | ((iTemp82 & (iSlow95 & iTemp29)) | ((iTemp82 & (iSlow95 & iTemp26)) | ((iTemp82 & (iSlow95 & iTemp23)) | ((iTemp82 & (iSlow95 & iTemp20)) | ((iTemp82 & (iSlow95 & iTemp17)) | ((iTemp82 & (iSlow95 & iTemp14)) | (iTemp82 & (iSlow95 & iTemp11)))))))) != 0 {
																																												fTemp129 = fTemp83 * fTemp128;
																																												self.fVec22[0] = fTemp129;
																																												fTemp130 = 1.13003 * (fTemp129 + self.fVec22[1]);
																																												self.fRec61[0] = self.fConst23 * (fTemp130 - self.fConst22 * self.fRec61[1]);
																																												self.fRec62[0] = self.fConst28 * (fTemp130 - self.fConst27 * self.fRec62[1]);
																																												fTemp131 = 1.13003 * fTemp129;
																																												fTemp132 = (self.fConst33 * (F32::tanh(fTemp131 + 0.08) - self.fConst32) + 0.4 * (self.fConst31 * (F32::tanh(0.9 * (fTemp131 - self.fRec62[0]) + 0.05) - self.fConst30) + self.fConst29 * (F32::tanh(1.25 * (self.fRec62[0] - self.fRec61[0]) + -0.08) - self.fConst25) + self.fConst24 * (F32::tanh(1.5 * self.fRec61[0] + 0.12) - self.fConst20))) / fTemp83;
																																												self.fVec23[0] = fTemp132;
																																												self.fRec60[0] = self.fConst36 * (self.fConst35 * self.fRec60[1] + 1.25 * (fTemp132 - self.fVec23[1]));
																																												fTemp133 = self.fRec60[0] * fTemp81;
																																											}
																																												if (iSlow95 & iTemp32) | ((iSlow95 & iTemp29) | ((iSlow95 & iTemp26) | ((iSlow95 & iTemp23) | ((iSlow95 & iTemp20) | ((iSlow95 & iTemp17) | ((iSlow95 & iTemp14) | (iSlow95 & iTemp11))))))) != 0 {
																																													fTemp134 = fSlow94 * if iTemp75 != 0 {fTemp89 * fTemp133} else {1.4125376 * fTemp128};
																																												}
																																													if iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | (iTemp14 | iTemp11)))))) != 0 {
																																														fTemp135 = (fSlow129 * fTemp134 + fSlow126 * fTemp127) * fTemp91 + fTemp95;
																																													}
																																														if iTemp23 | iTemp11 != 0 {
																																															fTemp136 = F32::max(-1.0, F32::min(1.0, fTemp135));
																																														}
																																															if iTemp11 != 0 {
																																																fTemp137 = fTemp136 * fTemp10;
																																															}
																																																if iTemp26 | iTemp14 != 0 {
																																																	fTemp138 = F32::atan(1.5707964 * fTemp95);
																																																}
																																																	if iTemp14 != 0 {
																																																		fTemp139 = 0.63661975 * fTemp138 * fTemp13;
																																																	}
																																																		if iTemp26 | iTemp14 != 0 {
																																																			fTemp140 = F32::atan(1.5707964 * fTemp135);
																																																		}
																																																			if iTemp14 != 0 {
																																																				fTemp141 = 0.63661975 * fTemp140 * fTemp13;
																																																			}
																																																				if iTemp29 | iTemp17 != 0 {
																																																					fTemp142 = if (fTemp95 < 0.0) as i32 != 0 {1.6 * F32::tanh(0.625 * fTemp95)} else {F32::tanh(fTemp95)};
																																																				}
																																																					if iTemp17 != 0 {
																																																						fTemp143 = fTemp142 * fTemp16;
																																																					}
																																																						if iTemp29 | iTemp17 != 0 {
																																																							fTemp144 = if (fTemp135 < 0.0) as i32 != 0 {1.6 * F32::tanh(0.625 * fTemp135)} else {F32::tanh(fTemp135)};
																																																						}
																																																							if iTemp17 != 0 {
																																																								fTemp145 = fTemp144 * fTemp16;
																																																							}
																																																								if iTemp32 | iTemp20 != 0 {
																																																									fTemp146 = F32::sin(fTemp95);
																																																								}
																																																									if iTemp20 != 0 {
																																																										fTemp147 = fTemp146 * fTemp19;
																																																									}
																																																										if iTemp32 | iTemp20 != 0 {
																																																											fTemp148 = F32::sin(fTemp135);
																																																										}
																																																											if iTemp20 != 0 {
																																																												fTemp149 = fTemp148 * fTemp19;
																																																											}
																																																												let mut fTemp150: F32 = F32::max(0.0, F32::min(1.0, 2.5 * (fTemp6 + (-0.8 - fTemp7))));
																																																												if iTemp23 != 0 {
																																																													fTemp151 = fTemp22 * fTemp96;
																																																													fTemp152 = fTemp136 * fTemp22;
																																																												}
																																																													if iTemp26 != 0 {
																																																														fTemp153 = 0.63661975 * fTemp25 * fTemp138;
																																																														fTemp154 = 0.63661975 * fTemp140 * fTemp25;
																																																													}
																																																														if iTemp29 != 0 {
																																																															fTemp155 = fTemp28 * fTemp142;
																																																															fTemp156 = fTemp144 * fTemp28;
																																																														}
																																																															if iTemp32 != 0 {
																																																																fTemp157 = fTemp31 * fTemp146;
																																																																fTemp158 = fTemp148 * fTemp31;
																																																															}
																																																																let mut fTemp159: F32 = ((1.0 - fTemp150) * (((iTemp30 != 0) as i32) as u32 as F32 * (fTemp158 - fTemp157) + ((iTemp27 != 0) as i32) as u32 as F32 * (fTemp156 - fTemp155) + ((iTemp24 != 0) as i32) as u32 as F32 * (fTemp154 - fTemp153) + ((iTemp21 != 0) as i32) as u32 as F32 * (fTemp152 - fTemp151)) + fTemp150 * (((iTemp18 != 0) as i32) as u32 as F32 * (fTemp149 - fTemp147) + ((iTemp15 != 0) as i32) as u32 as F32 * (fTemp145 - fTemp143) + ((iTemp12 != 0) as i32) as u32 as F32 * (fTemp141 - fTemp139) + ((iTemp9 != 0) as i32) as u32 as F32 * (fTemp137 - fTemp97))) / F32::tanh(0.35 * fTemp91);
																																																																self.fVec24[0] = fTemp159;
																																																																self.fRec43[0] = 0.995 * self.fRec43[1] + 0.35 * (fTemp159 - self.fVec24[1]);
																																																																*output0 = self.fRec43[0] * F32::powf(1e+01, 0.125 * self.fRec10[0]) + fSlow100 * fTemp90 + fSlow85 * self.fRec36[0] * fTemp73;
																																																																*output1 = fSlow130 * fTemp127;
																																																																*output2 = fTemp61;
																																																																*output3 = fSlow84 * fTemp73;
																																																																self.fVec0[1] = self.fVec0[0];
																																																																self.fVec1[1] = self.fVec1[0];
																																																																self.iVec2[1] = self.iVec2[0];
																																																																if iSlow4 != 0 {
																																																																	self.fVec3[1] = self.fVec3[0];
																																																																	self.fRec0[1] = self.fRec0[0];
																																																																	self.fRec1[1] = self.fRec1[0];
																																																																}
																																																																	if iSlow4 & iSlow11 != 0 {
																																																																		self.fVec4[1] = self.fVec4[0];
																																																																		self.fRec6[1] = self.fRec6[0];
																																																																		self.fRec7[1] = self.fRec7[0];
																																																																	}
																																																																		self.fVec5[1] = self.fVec5[0];
																																																																		self.fRec9[1] = self.fRec9[0];
																																																																		self.fRec10[1] = self.fRec10[0];
																																																																		if iTemp32 | (iTemp29 | (iTemp26 | (iTemp23 | (iTemp20 | (iTemp17 | (iTemp14 | iTemp11)))))) != 0 {
																																																																			self.fRec11[1] = self.fRec11[0];
																																																																		}
																																																																			self.fVec6[1] = self.fVec6[0];
																																																																			if iSlow24 != 0 {
																																																																				self.fVec7[1] = self.fVec7[0];
																																																																				self.fRec12[1] = self.fRec12[0];
																																																																				self.fRec13[1] = self.fRec13[0];
																																																																				self.iRec14[1] = self.iRec14[0];
																																																																				self.iRec15[1] = self.iRec15[0];
																																																																			}
																																																																				self.fVec8[1] = self.fVec8[0];
																																																																				self.fRec16[1] = self.fRec16[0];
																																																																				self.fRec17[1] = self.fRec17[0];
																																																																				if ((fTemp46 != 0.0) as i32) | ((((fTemp37 != 0.0) as i32) & iTemp32) | ((iSlow24 & (iSlow36 & (iSlow39 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32)))) | ((((fTemp37 != 0.0) as i32) & iTemp29) | ((((fTemp37 != 0.0) as i32) & iTemp26) | ((((fTemp37 != 0.0) as i32) & iTemp23) | ((((fTemp37 != 0.0) as i32) & iTemp20) | ((((fTemp37 != 0.0) as i32) & iTemp17) | ((((fTemp37 != 0.0) as i32) & iTemp14) | ((((fTemp37 != 0.0) as i32) & iTemp11) | ((iSlow4 & iSlow18) | ((iSlow4 & iSlow15) | (iSlow4 & iSlow11))))))))))))) != 0 {
																																																																					self.iRec8[1] = self.iRec8[0];
																																																																				}
																																																																					if iSlow4 & iSlow11 != 0 {
																																																																						self.fVec9[1] = self.fVec9[0];
																																																																						self.fRec5[1] = self.fRec5[0];
																																																																						self.fRec4[1] = self.fRec4[0];
																																																																					}
																																																																						if (fTemp46 != 0.0) as i32 != 0 {
																																																																							self.fRec21[1] = self.fRec21[0];
																																																																							self.fRec22[1] = self.fRec22[0];
																																																																							self.iRec24[1] = self.iRec24[0];
																																																																							self.fRec23[1] = self.fRec23[0];
																																																																							self.fRec25[1] = self.fRec25[0];
																																																																							self.fRec28[1] = self.fRec28[0];
																																																																							self.fRec27[1] = self.fRec27[0];
																																																																							self.fVec11[1] = self.fVec11[0];
																																																																							self.fRec26[1] = self.fRec26[0];
																																																																						}
																																																																							self.fRec29[1] = self.fRec29[0];
																																																																							self.fVec12[1] = self.fVec12[0];
																																																																							self.fRec30[1] = self.fRec30[0];
																																																																							self.fRec31[1] = self.fRec31[0];
																																																																							self.fRec32[1] = self.fRec32[0];
																																																																							if (iSlow4 & iSlow15) | ((iSlow4 & iSlow47) | (iSlow4 & iSlow44)) != 0 {
																																																																								self.fRec19[1] = self.fRec19[0];
																																																																							}
																																																																								if iSlow4 & iSlow44 != 0 {
																																																																									self.iRec18[1] = self.iRec18[0];
																																																																								}
																																																																									if iSlow4 & iSlow47 != 0 {
																																																																										self.iRec33[1] = self.iRec33[0];
																																																																									}
																																																																										if iSlow4 & iSlow15 != 0 {
																																																																											self.fRec34[1] = self.fRec34[0];
																																																																										}
																																																																											if iSlow4 != 0 {
																																																																												self.fVec13[1] = self.fVec13[0];
																																																																												self.fRec3[1] = self.fRec3[0];
																																																																												self.fVec14[1] = self.fVec14[0];
																																																																												self.fRec2[1] = self.fRec2[0];
																																																																											}
																																																																												self.fRec35[1] = self.fRec35[0];
																																																																												self.fRec36[1] = self.fRec36[0];
																																																																												if (iSlow95 & iTemp32) | ((iSlow95 & iTemp29) | ((iSlow95 & iTemp26) | ((iSlow95 & iTemp23) | ((iSlow95 & iTemp20) | ((iSlow95 & iTemp17) | ((iSlow95 & iTemp14) | ((iSlow95 & iTemp11) | iSlow92))))))) != 0 {
																																																																													self.fRec37[1] = self.fRec37[0];
																																																																													self.fVec15[1] = self.fVec15[0];
																																																																													self.fRec38[1] = self.fRec38[0];
																																																																													self.fRec39[1] = self.fRec39[0];
																																																																												}
																																																																													if iSlow92 & iTemp82 != 0 {
																																																																														self.fVec16[1] = self.fVec16[0];
																																																																														self.fRec41[1] = self.fRec41[0];
																																																																														self.fRec42[1] = self.fRec42[0];
																																																																														self.fVec17[1] = self.fVec17[0];
																																																																														self.fRec40[1] = self.fRec40[0];
																																																																													}
																																																																														if (((fTemp37 != 0.0) as i32) & iTemp32) | ((((fTemp37 != 0.0) as i32) & iTemp29) | ((((fTemp37 != 0.0) as i32) & iTemp26) | ((((fTemp37 != 0.0) as i32) & iTemp23) | ((((fTemp37 != 0.0) as i32) & iTemp20) | ((((fTemp37 != 0.0) as i32) & iTemp17) | ((((fTemp37 != 0.0) as i32) & iTemp14) | (((fTemp37 != 0.0) as i32) & iTemp11))))))) != 0 {
																																																																															self.fRec46[1] = self.fRec46[0];
																																																																															self.fRec45[1] = self.fRec45[0];
																																																																															self.fVec18[1] = self.fVec18[0];
																																																																															self.fRec44[1] = self.fRec44[0];
																																																																														}
																																																																															if (iSlow24 & (iSlow36 & (iSlow110 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow39 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow50 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow107 & ((fTemp43 != 0.0) as i32)))) | ((iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32)))) | (iSlow24 & (iSlow103 & ((fTemp43 != 0.0) as i32))))))) != 0 {
																																																																																self.fVec19[1] = self.fVec19[0];
																																																																																self.fRec48[1] = self.fRec48[0];
																																																																																self.fRec49[1] = self.fRec49[0];
																																																																															}
																																																																																if iSlow24 & (iSlow36 & (iSlow32 & ((fTemp43 != 0.0) as i32))) != 0 {
																																																																																	self.fRec51[1] = self.fRec51[0];
																																																																																	self.fRec50[1] = self.fRec50[0];
																																																																																}
																																																																																	if iSlow24 & (iSlow36 & (iSlow107 & ((fTemp43 != 0.0) as i32))) != 0 {
																																																																																		self.fVec20[1] = self.fVec20[0];
																																																																																		self.fRec52[1] = self.fRec52[0];
																																																																																		self.fRec53[1] = self.fRec53[0];
																																																																																		self.fRec54[1] = self.fRec54[0];
																																																																																		self.fRec55[1] = self.fRec55[0];
																																																																																	}
																																																																																		if iSlow24 & (iSlow36 & (iSlow50 & ((fTemp43 != 0.0) as i32))) != 0 {
																																																																																			self.fRec56[1] = self.fRec56[0];
																																																																																		}
																																																																																			if iSlow24 & (iSlow36 & (iSlow39 & ((fTemp43 != 0.0) as i32))) != 0 {
																																																																																				self.fRec57[2] = self.fRec57[1];
																																																																																				self.fRec57[1] = self.fRec57[0];
																																																																																			}
																																																																																				if iSlow24 & ((fTemp43 != 0.0) as i32) != 0 {
																																																																																					self.fVec21[1] = self.fVec21[0];
																																																																																					self.fRec47[1] = self.fRec47[0];
																																																																																				}
																																																																																					self.fRec58[1] = self.fRec58[0];
																																																																																					self.fRec59[1] = self.fRec59[0];
																																																																																					if (iTemp82 & (iSlow95 & iTemp32)) | ((iTemp82 & (iSlow95 & iTemp29)) | ((iTemp82 & (iSlow95 & iTemp26)) | ((iTemp82 & (iSlow95 & iTemp23)) | ((iTemp82 & (iSlow95 & iTemp20)) | ((iTemp82 & (iSlow95 & iTemp17)) | ((iTemp82 & (iSlow95 & iTemp14)) | (iTemp82 & (iSlow95 & iTemp11)))))))) != 0 {
																																																																																						self.fVec22[1] = self.fVec22[0];
																																																																																						self.fRec61[1] = self.fRec61[0];
																																																																																						self.fRec62[1] = self.fRec62[0];
																																																																																						self.fVec23[1] = self.fVec23[0];
																																																																																						self.fRec60[1] = self.fRec60[0];
																																																																																					}
																																																																																						self.fVec24[1] = self.fVec24[0];
																																																																																						self.fRec43[1] = self.fRec43[0];
																																																																																					}
																																																																																					self.fTempPerm0 = fTemp1;
																																																																																					self.fTempPerm1 = fTemp2;
																																																																																					self.fTempPerm2 = fTemp3;
																																																																																					self.iTempPerm3 = iTemp36;
																																																																																					self.fTempPerm4 = fTemp37;
																																																																																					self.fTempPerm5 = fTemp38;
																																																																																					self.fTempPerm6 = fTemp39;
																																																																																					self.fTempPerm7 = fTemp40;
																																																																																					self.fTempPerm8 = fTemp41;
																																																																																					self.iTempPerm9 = iTemp42;
																																																																																					self.fTempPerm10 = fTemp43;
																																																																																					self.fTempPerm11 = fTemp47;
																																																																																					self.fTempPerm12 = fTemp48;
																																																																																					self.fTempPerm13 = fTemp49;
																																																																																					self.fTempPerm14 = fTemp50;
																																																																																					self.iTempPerm15 = iTemp51;
																																																																																					self.fTempPerm16 = fTemp52;
																																																																																					self.fTempPerm17 = fTemp53;
																																																																																					self.iTempPerm18 = iTemp54;
																																																																																					self.fTempPerm19 = fTemp55;
																																																																																					self.fTempPerm20 = fTemp56;
																																																																																					self.fTempPerm21 = fTemp57;
																																																																																					self.fTempPerm22 = fTemp62;
																																																																																					self.iTempPerm23 = iTemp63;
																																																																																					self.iTempPerm24 = iTemp64;
																																																																																					self.fTempPerm25 = fTemp65;
																																																																																					self.iTempPerm26 = iTemp66;
																																																																																					self.iTempPerm27 = iTemp67;
																																																																																					self.fTempPerm28 = fTemp68;
																																																																																					self.fTempPerm29 = fTemp69;
																																																																																					self.fTempPerm30 = fTemp70;
																																																																																					self.fTempPerm31 = fTemp71;
																																																																																					self.fTempPerm32 = fTemp72;
																																																																																					self.fTempPerm33 = fTemp73;
																																																																																					self.iTempPerm34 = iTemp75;
																																																																																					self.fTempPerm35 = fTemp76;
																																																																																					self.fTempPerm36 = fTemp77;
																																																																																					self.fTempPerm37 = fTemp78;
																																																																																					self.fTempPerm38 = fTemp79;
																																																																																					self.fTempPerm39 = fTemp80;
																																																																																					self.fTempPerm40 = fTemp81;
																																																																																					self.iTempPerm41 = iTemp82;
																																																																																					self.fTempPerm42 = fTemp83;
																																																																																					self.fTempPerm43 = fTemp84;
																																																																																					self.fTempPerm44 = fTemp85;
																																																																																					self.fTempPerm45 = fTemp86;
																																																																																					self.fTempPerm46 = fTemp87;
																																																																																					self.fTempPerm47 = fTemp88;
																																																																																					self.fTempPerm48 = fTemp89;
																																																																																					self.fTempPerm49 = fTemp90;
																																																																																					self.fTempPerm50 = fTemp92;
																																																																																					self.fTempPerm51 = fTemp93;
																																																																																					self.fTempPerm52 = fTemp94;
																																																																																					self.fTempPerm53 = fTemp95;
																																																																																					self.fTempPerm54 = fTemp96;
																																																																																					self.fTempPerm55 = fTemp97;
																																																																																					self.fTempPerm56 = fTemp98;
																																																																																					self.fTempPerm57 = fTemp99;
																																																																																					self.fTempPerm58 = fTemp100;
																																																																																					self.fTempPerm59 = fTemp101;
																																																																																					self.fTempPerm60 = fTemp102;
																																																																																					self.fTempPerm61 = fTemp103;
																																																																																					self.fTempPerm62 = fTemp104;
																																																																																					self.fTempPerm63 = fTemp105;
																																																																																					self.fTempPerm64 = fTemp106;
																																																																																					self.fTempPerm65 = fTemp107;
																																																																																					self.fTempPerm66 = fTemp108;
																																																																																					self.fTempPerm67 = fTemp109;
																																																																																					self.fTempPerm68 = fTemp110;
																																																																																					self.fTempPerm69 = fTemp111;
																																																																																					self.fTempPerm70 = fTemp112;
																																																																																					self.fTempPerm71 = fTemp113;
																																																																																					self.fTempPerm72 = fTemp114;
																																																																																					self.fTempPerm73 = fTemp115;
																																																																																					self.fTempPerm74 = fTemp116;
																																																																																					self.fTempPerm75 = fTemp117;
																																																																																					self.fTempPerm76 = fTemp118;
																																																																																					self.fTempPerm77 = fTemp119;
																																																																																					self.fTempPerm78 = fTemp120;
																																																																																					self.fTempPerm79 = fTemp121;
																																																																																					self.fTempPerm80 = fTemp122;
																																																																																					self.fTempPerm81 = fTemp123;
																																																																																					self.fTempPerm82 = fTemp124;
																																																																																					self.fTempPerm83 = fTemp125;
																																																																																					self.fTempPerm84 = fTemp128;
																																																																																					self.fTempPerm85 = fTemp129;
																																																																																					self.fTempPerm86 = fTemp130;
																																																																																					self.fTempPerm87 = fTemp131;
																																																																																					self.fTempPerm88 = fTemp132;
																																																																																					self.fTempPerm89 = fTemp133;
																																																																																					self.fTempPerm90 = fTemp134;
																																																																																					self.fTempPerm91 = fTemp135;
																																																																																					self.fTempPerm92 = fTemp136;
																																																																																					self.fTempPerm93 = fTemp137;
																																																																																					self.fTempPerm94 = fTemp138;
																																																																																					self.fTempPerm95 = fTemp139;
																																																																																					self.fTempPerm96 = fTemp140;
																																																																																					self.fTempPerm97 = fTemp141;
																																																																																					self.fTempPerm98 = fTemp142;
																																																																																					self.fTempPerm99 = fTemp143;
																																																																																					self.fTempPerm100 = fTemp144;
																																																																																					self.fTempPerm101 = fTemp145;
																																																																																					self.fTempPerm102 = fTemp146;
																																																																																					self.fTempPerm103 = fTemp147;
																																																																																					self.fTempPerm104 = fTemp148;
																																																																																					self.fTempPerm105 = fTemp149;
																																																																																					self.fTempPerm106 = fTemp151;
																																																																																					self.fTempPerm107 = fTemp152;
																																																																																					self.fTempPerm108 = fTemp153;
																																																																																					self.fTempPerm109 = fTemp154;
																																																																																					self.fTempPerm110 = fTemp155;
																																																																																					self.fTempPerm111 = fTemp156;
																																																																																					self.fTempPerm112 = fTemp157;
																																																																																					self.fTempPerm113 = fTemp158;
																																																																																				}

}

