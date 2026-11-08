/* ------------------------------------------------------------
name: "plugin-voice"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn VoiceDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */


pub struct VoiceDspSIG0 {
	iVec2: [i32;2],
	iRec8: [i32;2],
}

impl VoiceDspSIG0 {
	
	fn get_num_inputsVoiceDspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsVoiceDspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initVoiceDspSIG0(&mut self, sample_rate: i32) {
		for l10 in 0..2 {
			self.iVec2[l10 as usize] = 0;
		}
		for l11 in 0..2 {
			self.iRec8[l11 as usize] = 0;
		}
	}
	
	fn fillVoiceDspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec2[0] = 1;
			self.iRec8[0] = (i32::wrapping_add(self.iVec2[1], self.iRec8[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec8[0]) as F32);
			self.iVec2[1] = self.iVec2[0];
			self.iRec8[1] = self.iRec8[0];
		}
	}

}


pub fn newVoiceDspSIG0() -> VoiceDspSIG0 { 
	VoiceDspSIG0 {
		iVec2: [0;2],
		iRec8: [0;2],
	}
}
static mut ftbl0VoiceDspSIG0: [F32;65536] = [0.0;65536];
fn VoiceDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
fn VoiceDsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct VoiceDsp {
	fHslider0: F32,
	fHslider1: F32,
	iVec0: [i32;2],
	fButton0: F32,
	fVec1: [F32;2],
	fSampleRate: i32,
	fConst1: F32,
	fHslider2: F32,
	fRec0: [F32;2],
	fEntry0: F32,
	fHslider3: F32,
	fRec1: [F32;2],
	fHslider4: F32,
	fRec2: [F32;2],
	fEntry1: F32,
	fHslider5: F32,
	fRec3: [F32;2],
	fHslider6: F32,
	fRec4: [F32;2],
	fTempPerm0: F32,
	fHslider7: F32,
	iRec6: [i32;2],
	iTempPerm1: i32,
	fConst2: F32,
	fConst4: F32,
	fRec5: [F32;2],
	fHslider8: F32,
	fRec7: [F32;2],
	fConst5: F32,
	fTempPerm2: F32,
	fHslider9: F32,
	fHslider10: F32,
	fHslider11: F32,
	fConst6: F32,
	fRec9: [F32;2],
	fHslider12: F32,
	fHslider13: F32,
	fHslider14: F32,
	fRec10: [F32;2],
	fTempPerm3: F32,
	fHslider15: F32,
	fHslider16: F32,
	fHslider17: F32,
	fTempPerm4: F32,
	fTempPerm5: F32,
	iRec11: [i32;2],
	fTempPerm6: F32,
	fConst7: F32,
	fTempPerm7: F32,
	iRec12: [i32;2],
	iTempPerm8: i32,
	fTempPerm9: F32,
	fEntry2: F32,
	fHslider18: F32,
	fHslider19: F32,
	fHslider20: F32,
	fTempPerm10: F32,
	fConst8: F32,
	fTempPerm11: F32,
	fConst9: F32,
	fConst10: F32,
	fTempPerm12: F32,
	fRec16: [F32;2],
	fTempPerm13: F32,
	fRec17: [F32;2],
	iRec19: [i32;2],
	iTempPerm14: i32,
	fConst11: F32,
	fConst13: F32,
	fRec18: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fTempPerm15: F32,
	fRec22: [F32;2],
	fEntry3: F32,
	fCheckbox0: F32,
	fRec24: [F32;2],
	fVslider0: F32,
	fHslider21: F32,
	fHslider22: F32,
	fHslider23: F32,
	fRec25: [F32;2],
	fTempPerm16: F32,
	fHslider24: F32,
	fRec26: [F32;2],
	iTempPerm17: i32,
	fTempPerm18: F32,
	iRec23: [i32;2],
	fTempPerm19: F32,
	fTempPerm20: F32,
	fRec21: [F32;2],
	fTempPerm21: F32,
	fVec3: [F32;2],
	fRec20: [F32;2],
	fTempPerm22: F32,
	fHslider25: F32,
	fRec27: [F32;2],
	fEntry4: F32,
	fHslider26: F32,
	fHslider27: F32,
	fHslider28: F32,
	fHslider29: F32,
	fRec28: [F32;2],
	fTempPerm23: F32,
	fRec15: [F32;2],
	fRec14: [F32;2],
	fTempPerm24: F32,
	fConst16: F32,
	fTempPerm25: F32,
	fRec30: [F32;2],
	fConst17: F32,
	fTempPerm26: F32,
	fRec29: [F32;2],
	fTempPerm27: F32,
	fConst18: F32,
	fTempPerm28: F32,
	fRec31: [F32;2],
	fTempPerm29: F32,
	fConst21: F32,
	fConst23: F32,
	fConst25: F32,
	fRec32: [F32;3],
	fConst26: F32,
	fTempPerm30: F32,
	fConst27: F32,
	fConst28: F32,
	fTempPerm31: F32,
	fTempPerm32: F32,
	fTempPerm33: F32,
	fVec4: [F32;2],
	fRec13: [F32;2],
	fTempPerm34: F32,
	fTempPerm35: F32,
	fCheckbox1: F32,
	fVslider1: F32,
	fVslider2: F32,
	fRec34: [F32;2],
	fRec35: [F32;2],
	fVslider3: F32,
	fRec36: [F32;2],
	fVslider4: F32,
	fVslider5: F32,
	fRec37: [F32;2],
	fVslider6: F32,
	fVslider7: F32,
	fRec38: [F32;2],
	fRec39: [F32;2],
	fRec33: [F32;2],
	fHslider30: F32,
	fHslider31: F32,
	fHslider32: F32,
	fRec41: [F32;2],
	fTempPerm36: F32,
	fCheckbox2: F32,
	fHslider33: F32,
	fHslider34: F32,
	fHslider35: F32,
	fRec42: [F32;2],
	fTempPerm37: F32,
	fHslider36: F32,
	fHslider37: F32,
	fHslider38: F32,
	fRec48: [F32;2],
	fTempPerm38: F32,
	fConst29: F32,
	fHslider39: F32,
	fHslider40: F32,
	fHslider41: F32,
	fRec49: [F32;2],
	fTempPerm39: F32,
	fCheckbox3: F32,
	fRec51: [F32;2],
	fRec52: [F32;2],
	fRec50: [F32;2],
	fHslider42: F32,
	fHslider43: F32,
	fHslider44: F32,
	fTempPerm40: F32,
	fTempPerm41: F32,
	fRec55: [F32;2],
	fRec54: [F32;2],
	fTempPerm42: F32,
	fVec5: [F32;2],
	fRec53: [F32;2],
	fTempPerm43: F32,
	fHslider45: F32,
	fHslider46: F32,
	fHslider47: F32,
	fRec56: [F32;2],
	fTempPerm44: F32,
	fHslider48: F32,
	fHslider49: F32,
	fRec57: [F32;2],
	fTempPerm45: F32,
	fHslider50: F32,
	fHslider51: F32,
	fVslider8: F32,
	fRec58: [F32;2],
	fTempPerm46: F32,
	fHslider52: F32,
	fHslider53: F32,
	fVslider9: F32,
	fRec59: [F32;2],
	fTempPerm47: F32,
	fHslider54: F32,
	fHslider55: F32,
	fVslider10: F32,
	fRec60: [F32;2],
	fTempPerm48: F32,
	fHslider56: F32,
	fHslider57: F32,
	fHslider58: F32,
	fRec61: [F32;2],
	fTempPerm49: F32,
	fHslider59: F32,
	fHslider60: F32,
	fHslider61: F32,
	fTempPerm50: F32,
	fTempPerm51: F32,
	fConst30: F32,
	fTempPerm52: F32,
	fTempPerm53: F32,
	fRec64: [F32;2],
	iRec65: [i32;2],
	iTempPerm54: i32,
	fRec63: [F32;2],
	fRec62: [F32;2],
	fTempPerm55: F32,
	fTempPerm56: F32,
	iTempPerm57: i32,
	fTempPerm58: F32,
	fHslider62: F32,
	fHslider63: F32,
	fHslider64: F32,
	fRec66: [F32;2],
	fTempPerm59: F32,
	fTempPerm60: F32,
	fConst32: F32,
	fHslider65: F32,
	fHslider66: F32,
	fHslider67: F32,
	fTempPerm61: F32,
	fTempPerm62: F32,
	fVec6: [F32;2],
	fConst33: F32,
	fRec70: [F32;2],
	fConst34: F32,
	fConst35: F32,
	fConst36: F32,
	fRec69: [F32;2],
	fTempPerm63: F32,
	fHslider68: F32,
	fTempPerm64: F32,
	fRec72: [F32;2],
	iTempPerm65: i32,
	iTempPerm66: i32,
	iRec71: [i32;2],
	fTempPerm67: F32,
	iTempPerm68: i32,
	iTempPerm69: i32,
	iRec73: [i32;2],
	fTempPerm70: F32,
	fRec74: [F32;2],
	fHslider69: F32,
	fTempPerm71: F32,
	fTempPerm72: F32,
	fTempPerm73: F32,
	fVec7: [F32;2],
	fConst37: F32,
	fConst38: F32,
	fConst39: F32,
	fRec68: [F32;2],
	fTempPerm74: F32,
	fVec8: [F32;2],
	fRec67: [F32;2],
	fTempPerm75: F32,
	fConst41: F32,
	fConst42: F32,
	fTempPerm76: F32,
	fRec78: [F32;2],
	fRec77: [F32;2],
	fTempPerm77: F32,
	fVec9: [F32;2],
	fConst43: F32,
	fRec76: [F32;2],
	fTempPerm78: F32,
	fTempPerm79: F32,
	fTempPerm80: F32,
	fTempPerm81: F32,
	fEntry5: F32,
	fEntry6: F32,
	fTempPerm82: F32,
	fTempPerm83: F32,
	fTempPerm84: F32,
	fRec79: [F32;2],
	fTempPerm85: F32,
	fTempPerm86: F32,
	fTempPerm87: F32,
	fHslider70: F32,
	fHslider71: F32,
	fHslider72: F32,
	fTempPerm88: F32,
	fRec80: [F32;2],
	fHslider73: F32,
	fHslider74: F32,
	fHslider75: F32,
	fRec81: [F32;2],
	fTempPerm89: F32,
	fHslider76: F32,
	fHslider77: F32,
	fHslider78: F32,
	fRec82: [F32;2],
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
	fTempPerm114: F32,
	fTempPerm115: F32,
	fTempPerm116: F32,
	fTempPerm117: F32,
	fTempPerm118: F32,
	fTempPerm119: F32,
	fTempPerm120: F32,
	fTempPerm121: F32,
	fTempPerm122: F32,
	fVec10: [F32;2],
	fRec75: [F32;2],
	fRec43: [F32;2],
	fRec44: [F32;2],
	fHslider79: F32,
	fHslider80: F32,
	fHslider81: F32,
	fRec83: [F32;2],
	fTempPerm123: F32,
	fTempPerm124: F32,
	fTempPerm125: F32,
	fTempPerm126: F32,
	fTempPerm127: F32,
	fRec84: [F32;2],
	fTempPerm128: F32,
	fTempPerm129: F32,
	fRec85: [F32;2],
	fTempPerm130: F32,
	fTempPerm131: F32,
	fVec11: [F32;2],
	fRec40: [F32;2],
	fHslider82: F32,
}

impl FaustDsp for VoiceDsp {
	type T = F32;
		
	fn new() -> VoiceDsp { 
		VoiceDsp {
			fHslider0: 0.0,
			fHslider1: 0.0,
			iVec0: [0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			fSampleRate: 0,
			fConst1: 0.0,
			fHslider2: 0.0,
			fRec0: [0.0;2],
			fEntry0: 0.0,
			fHslider3: 0.0,
			fRec1: [0.0;2],
			fHslider4: 0.0,
			fRec2: [0.0;2],
			fEntry1: 0.0,
			fHslider5: 0.0,
			fRec3: [0.0;2],
			fHslider6: 0.0,
			fRec4: [0.0;2],
			fTempPerm0: 0.0,
			fHslider7: 0.0,
			iRec6: [0;2],
			iTempPerm1: 0,
			fConst2: 0.0,
			fConst4: 0.0,
			fRec5: [0.0;2],
			fHslider8: 0.0,
			fRec7: [0.0;2],
			fConst5: 0.0,
			fTempPerm2: 0.0,
			fHslider9: 0.0,
			fHslider10: 0.0,
			fHslider11: 0.0,
			fConst6: 0.0,
			fRec9: [0.0;2],
			fHslider12: 0.0,
			fHslider13: 0.0,
			fHslider14: 0.0,
			fRec10: [0.0;2],
			fTempPerm3: 0.0,
			fHslider15: 0.0,
			fHslider16: 0.0,
			fHslider17: 0.0,
			fTempPerm4: 0.0,
			fTempPerm5: 0.0,
			iRec11: [0;2],
			fTempPerm6: 0.0,
			fConst7: 0.0,
			fTempPerm7: 0.0,
			iRec12: [0;2],
			iTempPerm8: 0,
			fTempPerm9: 0.0,
			fEntry2: 0.0,
			fHslider18: 0.0,
			fHslider19: 0.0,
			fHslider20: 0.0,
			fTempPerm10: 0.0,
			fConst8: 0.0,
			fTempPerm11: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fTempPerm12: 0.0,
			fRec16: [0.0;2],
			fTempPerm13: 0.0,
			fRec17: [0.0;2],
			iRec19: [0;2],
			iTempPerm14: 0,
			fConst11: 0.0,
			fConst13: 0.0,
			fRec18: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fTempPerm15: 0.0,
			fRec22: [0.0;2],
			fEntry3: 0.0,
			fCheckbox0: 0.0,
			fRec24: [0.0;2],
			fVslider0: 0.0,
			fHslider21: 0.0,
			fHslider22: 0.0,
			fHslider23: 0.0,
			fRec25: [0.0;2],
			fTempPerm16: 0.0,
			fHslider24: 0.0,
			fRec26: [0.0;2],
			iTempPerm17: 0,
			fTempPerm18: 0.0,
			iRec23: [0;2],
			fTempPerm19: 0.0,
			fTempPerm20: 0.0,
			fRec21: [0.0;2],
			fTempPerm21: 0.0,
			fVec3: [0.0;2],
			fRec20: [0.0;2],
			fTempPerm22: 0.0,
			fHslider25: 0.0,
			fRec27: [0.0;2],
			fEntry4: 0.0,
			fHslider26: 0.0,
			fHslider27: 0.0,
			fHslider28: 0.0,
			fHslider29: 0.0,
			fRec28: [0.0;2],
			fTempPerm23: 0.0,
			fRec15: [0.0;2],
			fRec14: [0.0;2],
			fTempPerm24: 0.0,
			fConst16: 0.0,
			fTempPerm25: 0.0,
			fRec30: [0.0;2],
			fConst17: 0.0,
			fTempPerm26: 0.0,
			fRec29: [0.0;2],
			fTempPerm27: 0.0,
			fConst18: 0.0,
			fTempPerm28: 0.0,
			fRec31: [0.0;2],
			fTempPerm29: 0.0,
			fConst21: 0.0,
			fConst23: 0.0,
			fConst25: 0.0,
			fRec32: [0.0;3],
			fConst26: 0.0,
			fTempPerm30: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fTempPerm31: 0.0,
			fTempPerm32: 0.0,
			fTempPerm33: 0.0,
			fVec4: [0.0;2],
			fRec13: [0.0;2],
			fTempPerm34: 0.0,
			fTempPerm35: 0.0,
			fCheckbox1: 0.0,
			fVslider1: 0.0,
			fVslider2: 0.0,
			fRec34: [0.0;2],
			fRec35: [0.0;2],
			fVslider3: 0.0,
			fRec36: [0.0;2],
			fVslider4: 0.0,
			fVslider5: 0.0,
			fRec37: [0.0;2],
			fVslider6: 0.0,
			fVslider7: 0.0,
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fRec33: [0.0;2],
			fHslider30: 0.0,
			fHslider31: 0.0,
			fHslider32: 0.0,
			fRec41: [0.0;2],
			fTempPerm36: 0.0,
			fCheckbox2: 0.0,
			fHslider33: 0.0,
			fHslider34: 0.0,
			fHslider35: 0.0,
			fRec42: [0.0;2],
			fTempPerm37: 0.0,
			fHslider36: 0.0,
			fHslider37: 0.0,
			fHslider38: 0.0,
			fRec48: [0.0;2],
			fTempPerm38: 0.0,
			fConst29: 0.0,
			fHslider39: 0.0,
			fHslider40: 0.0,
			fHslider41: 0.0,
			fRec49: [0.0;2],
			fTempPerm39: 0.0,
			fCheckbox3: 0.0,
			fRec51: [0.0;2],
			fRec52: [0.0;2],
			fRec50: [0.0;2],
			fHslider42: 0.0,
			fHslider43: 0.0,
			fHslider44: 0.0,
			fTempPerm40: 0.0,
			fTempPerm41: 0.0,
			fRec55: [0.0;2],
			fRec54: [0.0;2],
			fTempPerm42: 0.0,
			fVec5: [0.0;2],
			fRec53: [0.0;2],
			fTempPerm43: 0.0,
			fHslider45: 0.0,
			fHslider46: 0.0,
			fHslider47: 0.0,
			fRec56: [0.0;2],
			fTempPerm44: 0.0,
			fHslider48: 0.0,
			fHslider49: 0.0,
			fRec57: [0.0;2],
			fTempPerm45: 0.0,
			fHslider50: 0.0,
			fHslider51: 0.0,
			fVslider8: 0.0,
			fRec58: [0.0;2],
			fTempPerm46: 0.0,
			fHslider52: 0.0,
			fHslider53: 0.0,
			fVslider9: 0.0,
			fRec59: [0.0;2],
			fTempPerm47: 0.0,
			fHslider54: 0.0,
			fHslider55: 0.0,
			fVslider10: 0.0,
			fRec60: [0.0;2],
			fTempPerm48: 0.0,
			fHslider56: 0.0,
			fHslider57: 0.0,
			fHslider58: 0.0,
			fRec61: [0.0;2],
			fTempPerm49: 0.0,
			fHslider59: 0.0,
			fHslider60: 0.0,
			fHslider61: 0.0,
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			fConst30: 0.0,
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fRec64: [0.0;2],
			iRec65: [0;2],
			iTempPerm54: 0,
			fRec63: [0.0;2],
			fRec62: [0.0;2],
			fTempPerm55: 0.0,
			fTempPerm56: 0.0,
			iTempPerm57: 0,
			fTempPerm58: 0.0,
			fHslider62: 0.0,
			fHslider63: 0.0,
			fHslider64: 0.0,
			fRec66: [0.0;2],
			fTempPerm59: 0.0,
			fTempPerm60: 0.0,
			fConst32: 0.0,
			fHslider65: 0.0,
			fHslider66: 0.0,
			fHslider67: 0.0,
			fTempPerm61: 0.0,
			fTempPerm62: 0.0,
			fVec6: [0.0;2],
			fConst33: 0.0,
			fRec70: [0.0;2],
			fConst34: 0.0,
			fConst35: 0.0,
			fConst36: 0.0,
			fRec69: [0.0;2],
			fTempPerm63: 0.0,
			fHslider68: 0.0,
			fTempPerm64: 0.0,
			fRec72: [0.0;2],
			iTempPerm65: 0,
			iTempPerm66: 0,
			iRec71: [0;2],
			fTempPerm67: 0.0,
			iTempPerm68: 0,
			iTempPerm69: 0,
			iRec73: [0;2],
			fTempPerm70: 0.0,
			fRec74: [0.0;2],
			fHslider69: 0.0,
			fTempPerm71: 0.0,
			fTempPerm72: 0.0,
			fTempPerm73: 0.0,
			fVec7: [0.0;2],
			fConst37: 0.0,
			fConst38: 0.0,
			fConst39: 0.0,
			fRec68: [0.0;2],
			fTempPerm74: 0.0,
			fVec8: [0.0;2],
			fRec67: [0.0;2],
			fTempPerm75: 0.0,
			fConst41: 0.0,
			fConst42: 0.0,
			fTempPerm76: 0.0,
			fRec78: [0.0;2],
			fRec77: [0.0;2],
			fTempPerm77: 0.0,
			fVec9: [0.0;2],
			fConst43: 0.0,
			fRec76: [0.0;2],
			fTempPerm78: 0.0,
			fTempPerm79: 0.0,
			fTempPerm80: 0.0,
			fTempPerm81: 0.0,
			fEntry5: 0.0,
			fEntry6: 0.0,
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			fTempPerm84: 0.0,
			fRec79: [0.0;2],
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			fTempPerm87: 0.0,
			fHslider70: 0.0,
			fHslider71: 0.0,
			fHslider72: 0.0,
			fTempPerm88: 0.0,
			fRec80: [0.0;2],
			fHslider73: 0.0,
			fHslider74: 0.0,
			fHslider75: 0.0,
			fRec81: [0.0;2],
			fTempPerm89: 0.0,
			fHslider76: 0.0,
			fHslider77: 0.0,
			fHslider78: 0.0,
			fRec82: [0.0;2],
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
			fTempPerm114: 0.0,
			fTempPerm115: 0.0,
			fTempPerm116: 0.0,
			fTempPerm117: 0.0,
			fTempPerm118: 0.0,
			fTempPerm119: 0.0,
			fTempPerm120: 0.0,
			fTempPerm121: 0.0,
			fTempPerm122: 0.0,
			fVec10: [0.0;2],
			fRec75: [0.0;2],
			fRec43: [0.0;2],
			fRec44: [0.0;2],
			fHslider79: 0.0,
			fHslider80: 0.0,
			fHslider81: 0.0,
			fRec83: [0.0;2],
			fTempPerm123: 0.0,
			fTempPerm124: 0.0,
			fTempPerm125: 0.0,
			fTempPerm126: 0.0,
			fTempPerm127: 0.0,
			fRec84: [0.0;2],
			fTempPerm128: 0.0,
			fTempPerm129: 0.0,
			fRec85: [0.0;2],
			fTempPerm130: 0.0,
			fTempPerm131: 0.0,
			fVec11: [0.0;2],
			fRec40: [0.0;2],
			fHslider82: 0.0,
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
		m.declare("filters.lib/dcblocker:author", r"Julius O. Smith III");
		m.declare("filters.lib/dcblocker:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/dcblocker:license", r"MIT-style STK-4.3 license");
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
		return 4;
	}
	fn get_num_outputs(&self) -> i32 {
		return 11;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: VoiceDspSIG0 = newVoiceDspSIG0();
		sig0.instance_initVoiceDspSIG0(sample_rate);
		sig0.fillVoiceDspSIG0(65536, unsafe { &mut ftbl0VoiceDspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider2 = 0.0;
		self.fEntry0 = 1.0;
		self.fHslider3 = 5e+01;
		self.fHslider4 = 0.0;
		self.fEntry1 = 21.0;
		self.fHslider5 = 5e+01;
		self.fHslider6 = 0.0;
		self.fHslider7 = 0.0;
		self.fHslider8 = 1.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 0.5;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 0.0;
		self.fHslider15 = 0.0;
		self.fHslider16 = 0.0;
		self.fHslider17 = 6e+01;
		self.fEntry2 = 0.0;
		self.fHslider18 = 0.0;
		self.fHslider19 = 0.0;
		self.fHslider20 = 0.0;
		self.fEntry3 = 1.0;
		self.fCheckbox0 = 0.0;
		self.fVslider0 = 0.15;
		self.fHslider21 = 0.0;
		self.fHslider22 = 0.0;
		self.fHslider23 = 0.1;
		self.fHslider24 = 1.0;
		self.fHslider25 = 0.0;
		self.fEntry4 = 0.0;
		self.fHslider26 = 0.0;
		self.fHslider27 = 0.0;
		self.fHslider28 = 0.0;
		self.fHslider29 = 1.1e+02;
		self.fCheckbox1 = 0.0;
		self.fVslider1 = 0.5;
		self.fVslider2 = 0.5;
		self.fVslider3 = 0.005;
		self.fVslider4 = 0.4;
		self.fVslider5 = 0.0;
		self.fVslider6 = 0.3;
		self.fVslider7 = 0.6;
		self.fHslider30 = 0.0;
		self.fHslider31 = 0.0;
		self.fHslider32 = 2e+01;
		self.fCheckbox2 = 0.0;
		self.fHslider33 = 0.0;
		self.fHslider34 = 0.0;
		self.fHslider35 = 0.0;
		self.fHslider36 = 0.0;
		self.fHslider37 = 0.0;
		self.fHslider38 = 0.8;
		self.fHslider39 = 0.0;
		self.fHslider40 = 0.0;
		self.fHslider41 = 2.0;
		self.fCheckbox3 = 0.0;
		self.fHslider42 = 0.0;
		self.fHslider43 = 0.0;
		self.fHslider44 = 0.5;
		self.fHslider45 = 0.0;
		self.fHslider46 = 0.0;
		self.fHslider47 = 1.2e+03;
		self.fHslider48 = 0.0;
		self.fHslider49 = 0.0;
		self.fHslider50 = 0.0;
		self.fHslider51 = 0.0;
		self.fVslider8 = 0.0;
		self.fHslider52 = 0.0;
		self.fHslider53 = 0.0;
		self.fVslider9 = 0.0;
		self.fHslider54 = 0.0;
		self.fHslider55 = 0.0;
		self.fVslider10 = 1.0;
		self.fHslider56 = 0.0;
		self.fHslider57 = 0.0;
		self.fHslider58 = 0.0;
		self.fHslider59 = 0.0;
		self.fHslider60 = 0.0;
		self.fHslider61 = 1.5;
		self.fHslider62 = 0.0;
		self.fHslider63 = 0.0;
		self.fHslider64 = 0.0;
		self.fHslider65 = 0.0;
		self.fHslider66 = 0.0;
		self.fHslider67 = 0.3;
		self.fHslider68 = 0.0;
		self.fHslider69 = 8.0;
		self.fEntry5 = 2.0;
		self.fEntry6 = 0.0;
		self.fHslider70 = 0.0;
		self.fHslider71 = 0.0;
		self.fHslider72 = 0.5;
		self.fHslider73 = 0.0;
		self.fHslider74 = 0.0;
		self.fHslider75 = 0.0;
		self.fHslider76 = 0.0;
		self.fHslider77 = 0.0;
		self.fHslider78 = 0.5;
		self.fHslider79 = 0.0;
		self.fHslider80 = 0.0;
		self.fHslider81 = 0.0;
		self.fHslider82 = 0.8;
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
			self.fRec4[l6 as usize] = 0.0;
		}
		self.fTempPerm0 = 0.0;
		for l7 in 0..2 {
			self.iRec6[l7 as usize] = 0;
		}
		self.iTempPerm1 = 0;
		for l8 in 0..2 {
			self.fRec5[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec7[l9 as usize] = 0.0;
		}
		self.fTempPerm2 = 0.0;
		for l12 in 0..2 {
			self.fRec9[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec10[l13 as usize] = 0.0;
		}
		self.fTempPerm3 = 0.0;
		self.fTempPerm4 = 0.0;
		self.fTempPerm5 = 0.0;
		for l14 in 0..2 {
			self.iRec11[l14 as usize] = 0;
		}
		self.fTempPerm6 = 0.0;
		self.fTempPerm7 = 0.0;
		for l15 in 0..2 {
			self.iRec12[l15 as usize] = 0;
		}
		self.iTempPerm8 = 0;
		self.fTempPerm9 = 0.0;
		self.fTempPerm10 = 0.0;
		self.fTempPerm11 = 0.0;
		self.fTempPerm12 = 0.0;
		for l16 in 0..2 {
			self.fRec16[l16 as usize] = 0.0;
		}
		self.fTempPerm13 = 0.0;
		for l17 in 0..2 {
			self.fRec17[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.iRec19[l18 as usize] = 0;
		}
		self.iTempPerm14 = 0;
		for l19 in 0..2 {
			self.fRec18[l19 as usize] = 0.0;
		}
		self.fTempPerm15 = 0.0;
		for l20 in 0..2 {
			self.fRec22[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec24[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec25[l22 as usize] = 0.0;
		}
		self.fTempPerm16 = 0.0;
		for l23 in 0..2 {
			self.fRec26[l23 as usize] = 0.0;
		}
		self.iTempPerm17 = 0;
		self.fTempPerm18 = 0.0;
		for l24 in 0..2 {
			self.iRec23[l24 as usize] = 0;
		}
		self.fTempPerm19 = 0.0;
		self.fTempPerm20 = 0.0;
		for l25 in 0..2 {
			self.fRec21[l25 as usize] = 0.0;
		}
		self.fTempPerm21 = 0.0;
		for l26 in 0..2 {
			self.fVec3[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec20[l27 as usize] = 0.0;
		}
		self.fTempPerm22 = 0.0;
		for l28 in 0..2 {
			self.fRec27[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec28[l29 as usize] = 0.0;
		}
		self.fTempPerm23 = 0.0;
		for l30 in 0..2 {
			self.fRec15[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec14[l31 as usize] = 0.0;
		}
		self.fTempPerm24 = 0.0;
		self.fTempPerm25 = 0.0;
		for l32 in 0..2 {
			self.fRec30[l32 as usize] = 0.0;
		}
		self.fTempPerm26 = 0.0;
		for l33 in 0..2 {
			self.fRec29[l33 as usize] = 0.0;
		}
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		for l34 in 0..2 {
			self.fRec31[l34 as usize] = 0.0;
		}
		self.fTempPerm29 = 0.0;
		for l35 in 0..3 {
			self.fRec32[l35 as usize] = 0.0;
		}
		self.fTempPerm30 = 0.0;
		self.fTempPerm31 = 0.0;
		self.fTempPerm32 = 0.0;
		self.fTempPerm33 = 0.0;
		for l36 in 0..2 {
			self.fVec4[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec13[l37 as usize] = 0.0;
		}
		self.fTempPerm34 = 0.0;
		self.fTempPerm35 = 0.0;
		for l38 in 0..2 {
			self.fRec34[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec35[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec36[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec37[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec38[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec39[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec33[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec41[l45 as usize] = 0.0;
		}
		self.fTempPerm36 = 0.0;
		for l46 in 0..2 {
			self.fRec42[l46 as usize] = 0.0;
		}
		self.fTempPerm37 = 0.0;
		for l47 in 0..2 {
			self.fRec48[l47 as usize] = 0.0;
		}
		self.fTempPerm38 = 0.0;
		for l48 in 0..2 {
			self.fRec49[l48 as usize] = 0.0;
		}
		self.fTempPerm39 = 0.0;
		for l49 in 0..2 {
			self.fRec51[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec52[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec50[l51 as usize] = 0.0;
		}
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		for l52 in 0..2 {
			self.fRec55[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec54[l53 as usize] = 0.0;
		}
		self.fTempPerm42 = 0.0;
		for l54 in 0..2 {
			self.fVec5[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec53[l55 as usize] = 0.0;
		}
		self.fTempPerm43 = 0.0;
		for l56 in 0..2 {
			self.fRec56[l56 as usize] = 0.0;
		}
		self.fTempPerm44 = 0.0;
		for l57 in 0..2 {
			self.fRec57[l57 as usize] = 0.0;
		}
		self.fTempPerm45 = 0.0;
		for l58 in 0..2 {
			self.fRec58[l58 as usize] = 0.0;
		}
		self.fTempPerm46 = 0.0;
		for l59 in 0..2 {
			self.fRec59[l59 as usize] = 0.0;
		}
		self.fTempPerm47 = 0.0;
		for l60 in 0..2 {
			self.fRec60[l60 as usize] = 0.0;
		}
		self.fTempPerm48 = 0.0;
		for l61 in 0..2 {
			self.fRec61[l61 as usize] = 0.0;
		}
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		for l62 in 0..2 {
			self.fRec64[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.iRec65[l63 as usize] = 0;
		}
		self.iTempPerm54 = 0;
		for l64 in 0..2 {
			self.fRec63[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec62[l65 as usize] = 0.0;
		}
		self.fTempPerm55 = 0.0;
		self.fTempPerm56 = 0.0;
		self.iTempPerm57 = 0;
		self.fTempPerm58 = 0.0;
		for l66 in 0..2 {
			self.fRec66[l66 as usize] = 0.0;
		}
		self.fTempPerm59 = 0.0;
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		self.fTempPerm62 = 0.0;
		for l67 in 0..2 {
			self.fVec6[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec70[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec69[l69 as usize] = 0.0;
		}
		self.fTempPerm63 = 0.0;
		self.fTempPerm64 = 0.0;
		for l70 in 0..2 {
			self.fRec72[l70 as usize] = 0.0;
		}
		self.iTempPerm65 = 0;
		self.iTempPerm66 = 0;
		for l71 in 0..2 {
			self.iRec71[l71 as usize] = 0;
		}
		self.fTempPerm67 = 0.0;
		self.iTempPerm68 = 0;
		self.iTempPerm69 = 0;
		for l72 in 0..2 {
			self.iRec73[l72 as usize] = 0;
		}
		self.fTempPerm70 = 0.0;
		for l73 in 0..2 {
			self.fRec74[l73 as usize] = 0.0;
		}
		self.fTempPerm71 = 0.0;
		self.fTempPerm72 = 0.0;
		self.fTempPerm73 = 0.0;
		for l74 in 0..2 {
			self.fVec7[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec68[l75 as usize] = 0.0;
		}
		self.fTempPerm74 = 0.0;
		for l76 in 0..2 {
			self.fVec8[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec67[l77 as usize] = 0.0;
		}
		self.fTempPerm75 = 0.0;
		self.fTempPerm76 = 0.0;
		for l78 in 0..2 {
			self.fRec78[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec77[l79 as usize] = 0.0;
		}
		self.fTempPerm77 = 0.0;
		for l80 in 0..2 {
			self.fVec9[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec76[l81 as usize] = 0.0;
		}
		self.fTempPerm78 = 0.0;
		self.fTempPerm79 = 0.0;
		self.fTempPerm80 = 0.0;
		self.fTempPerm81 = 0.0;
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.fTempPerm84 = 0.0;
		for l82 in 0..2 {
			self.fRec79[l82 as usize] = 0.0;
		}
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		self.fTempPerm87 = 0.0;
		self.fTempPerm88 = 0.0;
		for l83 in 0..2 {
			self.fRec80[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec81[l84 as usize] = 0.0;
		}
		self.fTempPerm89 = 0.0;
		for l85 in 0..2 {
			self.fRec82[l85 as usize] = 0.0;
		}
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
		self.fTempPerm114 = 0.0;
		self.fTempPerm115 = 0.0;
		self.fTempPerm116 = 0.0;
		self.fTempPerm117 = 0.0;
		self.fTempPerm118 = 0.0;
		self.fTempPerm119 = 0.0;
		self.fTempPerm120 = 0.0;
		self.fTempPerm121 = 0.0;
		self.fTempPerm122 = 0.0;
		for l86 in 0..2 {
			self.fVec10[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec75[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec43[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec44[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec83[l90 as usize] = 0.0;
		}
		self.fTempPerm123 = 0.0;
		self.fTempPerm124 = 0.0;
		self.fTempPerm125 = 0.0;
		self.fTempPerm126 = 0.0;
		self.fTempPerm127 = 0.0;
		for l91 in 0..2 {
			self.fRec84[l91 as usize] = 0.0;
		}
		self.fTempPerm128 = 0.0;
		self.fTempPerm129 = 0.0;
		for l92 in 0..2 {
			self.fRec85[l92 as usize] = 0.0;
		}
		self.fTempPerm130 = 0.0;
		self.fTempPerm131 = 0.0;
		for l93 in 0..2 {
			self.fVec11[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec40[l94 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 - 44.1 / fConst0;
		self.fConst2 = F32::exp(-(157.07964 / fConst0));
		let mut fConst3: F32 = 1.0 - self.fConst2;
		self.fConst4 = 4.656613e-10 * F32::sqrt(3.0 * ((self.fConst2 + 1.0) / fConst3));
		self.fConst5 = 0.11626748 * fConst3;
		self.fConst6 = 5.5 / fConst0;
		self.fConst7 = 1.0 / fConst0;
		self.fConst8 = 4.41e+04 / fConst0;
		self.fConst9 = 5.0 / fConst0;
		self.fConst10 = 6.5 / fConst0;
		self.fConst11 = F32::exp(-(376.99112 / fConst0));
		let mut fConst12: F32 = 1.0 - self.fConst11;
		self.fConst13 = 4.656613e-10 * F32::sqrt(3.0 * ((self.fConst11 + 1.0) / fConst12));
		self.fConst14 = 3.0 * fConst12;
		self.fConst15 = 3.1415927 / fConst0;
		self.fConst16 = 1.41 / fConst0;
		self.fConst17 = 1e+02 / fConst0;
		self.fConst18 = 125.0 / fConst0;
		let mut fConst19: F32 = F32::tan(2513.2742 / fConst0);
		let mut fConst20: F32 = VoiceDsp_faustpower2_f(fConst19);
		self.fConst21 = 2.0 * (1.0 - 1.0 / fConst20);
		let mut fConst22: F32 = 1.0 / fConst19;
		self.fConst23 = (fConst22 + -1.4142135) / fConst19 + 1.0;
		let mut fConst24: F32 = (fConst22 + 1.4142135) / fConst19 + 1.0;
		self.fConst25 = 1.0 / fConst24;
		self.fConst26 = 1.0 / (fConst20 * fConst24);
		self.fConst27 = 666.6667 / fConst0;
		self.fConst28 = 15707.963 / fConst0;
		self.fConst29 = 0.45 * fConst0;
		self.fConst30 = 2.0 / fConst0;
		let mut fConst31: F32 = 1.0 / F32::tan(18849.557 / fConst0);
		self.fConst32 = 1.0 - fConst31;
		self.fConst33 = 1.0 / (fConst31 + 1.0);
		self.fConst34 = 1.0 / F32::tan(942.4778 / fConst0);
		self.fConst35 = 1.0 - self.fConst34;
		self.fConst36 = 1.0 / (self.fConst34 + 1.0);
		self.fConst37 = 1.0 / F32::tan(3.1415927 * (F32::min(2e+03, 0.4 * fConst0) / fConst0));
		self.fConst38 = 1.0 - self.fConst37;
		self.fConst39 = 1.0 / (self.fConst37 + 1.0);
		let mut fConst40: F32 = 1.0 / F32::tan(1570.7964 / fConst0);
		self.fConst41 = 1.0 - fConst40;
		self.fConst42 = 5e+02 / fConst0;
		self.fConst43 = 1.0 / (fConst40 + 1.0);
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
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.add_horizontal_slider("Amount Env", ParamIndex(1), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "1", "");
		ui_interface.add_horizontal_slider("Amount LFO", ParamIndex(2), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "0", "");
		ui_interface.open_horizontal_box("Master");
		ui_interface.declare(Some(ParamIndex(3)), "1", "");
		ui_interface.add_horizontal_slider("Volume Env", ParamIndex(3), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.add_horizontal_slider("Volume LFO", ParamIndex(4), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(5)), "2", "");
		ui_interface.add_horizontal_slider("Portamento Env", ParamIndex(5), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "2", "");
		ui_interface.add_horizontal_slider("Portamento LFO", ParamIndex(6), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Perform");
		ui_interface.declare(Some(ParamIndex(7)), "1", "");
		ui_interface.declare(Some(ParamIndex(7)), "style", "menu{'None':0;'Cutoff':1;'Resonance':2;'Morph':3;'Bandpass':4;'Filter Env Amount':5;'Drive':6;'Dual A':7;'Dual B':8;'Dual Pitch':9;'Mod Env Amount':10;'Mod LFO Amount':11;'WT Position':12;'WT LFO Rate':13;'Pulse Width':14;'PWM Depth':15;'Unison Detune':16;'Noise Level':17;'Noise Tilt':18;'Dual Level':19;'WT Level':20;'Vibrato':21;'Tremolo':22;'Level':23;'OscMod Depth':24}");
		ui_interface.add_num_entry("MW Dest", ParamIndex(7), 21.0, 0.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "2", "");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "%");
		ui_interface.add_horizontal_slider("MW Depth", ParamIndex(8), 5e+01, -1e+02, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(9)), "3", "");
		ui_interface.declare(Some(ParamIndex(9)), "style", "menu{'None':0;'Cutoff':1;'Resonance':2;'Morph':3;'Bandpass':4;'Filter Env Amount':5;'Drive':6;'Dual A':7;'Dual B':8;'Dual Pitch':9;'Mod Env Amount':10;'Mod LFO Amount':11;'WT Position':12;'WT LFO Rate':13;'Pulse Width':14;'PWM Depth':15;'Unison Detune':16;'Noise Level':17;'Noise Tilt':18;'Dual Level':19;'WT Level':20;'Vibrato':21;'Tremolo':22;'Level':23;'OscMod Depth':24}");
		ui_interface.add_num_entry("AT Dest", ParamIndex(9), 1.0, 0.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(10)), "4", "");
		ui_interface.declare(Some(ParamIndex(10)), "unit", "%");
		ui_interface.add_horizontal_slider("AT Depth", ParamIndex(10), 5e+01, -1e+02, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(11)), "5", "");
		ui_interface.declare(Some(ParamIndex(11)), "style", "menu{'2 Semitones':0;'Octave':1;'Cutoff 50%':2;'Resonance 50%':3;'Bandpass 50%':4}");
		ui_interface.add_num_entry("Bend Mode", ParamIndex(11), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "13", "");
		ui_interface.open_horizontal_box("Settings");
		ui_interface.declare(Some(ParamIndex(12)), "0", "");
		ui_interface.add_horizontal_slider("Drive Grit", ParamIndex(12), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(13)), "1", "");
		ui_interface.add_horizontal_slider("Vintage Noise", ParamIndex(13), 1.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("Fundamental");
		ui_interface.declare(Some(ParamIndex(14)), "0", "");
		ui_interface.add_num_entry("Octave", ParamIndex(14), 0.0, -2.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.declare(Some(ParamIndex(15)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'PWM':3}");
		ui_interface.add_num_entry("Wave", ParamIndex(15), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(16)), "2", "");
		ui_interface.add_horizontal_slider("Pulse Width", ParamIndex(16), 0.5, 0.05, 0.95, 0.01);
		ui_interface.declare(Some(ParamIndex(17)), "2", "");
		ui_interface.add_horizontal_slider("Pulse Width Env", ParamIndex(17), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(18)), "2", "");
		ui_interface.add_horizontal_slider("Pulse Width LFO", ParamIndex(18), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(19)), "3", "");
		ui_interface.add_horizontal_slider("PW LFO Rate Env", ParamIndex(19), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "3", "");
		ui_interface.add_horizontal_slider("PW LFO Rate LFO", ParamIndex(20), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "3", "");
		ui_interface.declare(Some(ParamIndex(21)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(21)), "unit", "Hz");
		ui_interface.add_horizontal_slider("PW LFO Rate", ParamIndex(21), 0.5, 0.01, 2e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(22)), "4", "");
		ui_interface.add_horizontal_slider("PW LFO Depth", ParamIndex(22), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(23)), "4", "");
		ui_interface.add_horizontal_slider("PW LFO Depth Env", ParamIndex(23), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "4", "");
		ui_interface.add_horizontal_slider("PW LFO Depth LFO", ParamIndex(24), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "4", "");
		ui_interface.open_horizontal_box("Noise");
		ui_interface.declare(Some(ParamIndex(25)), "1", "");
		ui_interface.declare(Some(ParamIndex(25)), "style", "menu{'White':0;'Clocked':1;'LFSR Long':2;'LFSR Short':3;'Crackle':4}");
		ui_interface.add_num_entry("Type", ParamIndex(25), 1.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(26)), "2", "");
		ui_interface.declare(Some(ParamIndex(26)), "unit", "oct");
		ui_interface.add_horizontal_slider("Clock Ratio", ParamIndex(26), 0.0, -4.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(27)), "3", "");
		ui_interface.add_horizontal_slider("Bits", ParamIndex(27), 8.0, 1.0, 16.0, 1.0);
		ui_interface.declare(Some(ParamIndex(28)), "4", "");
		ui_interface.add_horizontal_slider("Crackle Density", ParamIndex(28), 0.3, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(29)), "4", "");
		ui_interface.add_horizontal_slider("Crackle Density Env", ParamIndex(29), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(30)), "4", "");
		ui_interface.add_horizontal_slider("Crackle Density LFO", ParamIndex(30), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "5", "");
		ui_interface.add_horizontal_slider("Tilt Env", ParamIndex(31), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(32)), "5", "");
		ui_interface.add_horizontal_slider("Tilt LFO", ParamIndex(32), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(33)), "5", "");
		ui_interface.declare(Some(ParamIndex(33)), "unit", "dB");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(33), 0.0, -24.0, 24.0, 0.1);
		ui_interface.declare(Some(ParamIndex(34)), "6", "");
		ui_interface.add_horizontal_slider("Move", ParamIndex(34), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(35)), "6", "");
		ui_interface.add_horizontal_slider("Move Env", ParamIndex(35), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(36)), "6", "");
		ui_interface.add_horizontal_slider("Move LFO", ParamIndex(36), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(37)), "7", "");
		ui_interface.add_horizontal_slider("Rate Env", ParamIndex(37), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(38)), "7", "");
		ui_interface.add_horizontal_slider("Rate LFO", ParamIndex(38), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(39)), "7", "");
		ui_interface.declare(Some(ParamIndex(39)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(39)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(39), 1.5, 0.1, 8.0, 0.01);
		ui_interface.declare(Some(ParamIndex(40)), "8", "");
		ui_interface.add_check_button("Mute", ParamIndex(40));
		ui_interface.close_box();
		ui_interface.declare(None, "5", "");
		ui_interface.open_horizontal_box("Transient");
		ui_interface.declare(Some(ParamIndex(41)), "1", "");
		ui_interface.declare(Some(ParamIndex(41)), "style", "menu{'Click':0;'Burst':1;'Zap':2;'Metal':3;'Chiff':4;'Jade 01':5;'Jade 02':6;'Jade 03':7;'Jade 04':8;'Jade 05':9;'Jade 06':10;'Jade 07':11;'Jade 08':12;'Jade 09':13;'Jade 10':14;'Jade 11':15;'Jade 12':16;'Jade 13':17;'Jade 14':18;'Jade 15':19;'Jade 16':20;'Jade 17':21;'Jade 18':22;'Jade 19':23;'Jade 20':24;'Jade 21':25;'Jade 22':26;'Jade 23':27;'Jade 24':28;'Jade 25':29;'Jade 26':30;'Jade 27':31;'Jade 28':32;'Jade 29':33;'Jade 30':34;'Jade 31':35;'Jade 32':36;'Jade 33':37;'Jade 34':38;'Jade 35':39;'Jade 36':40;'Jade 37':41;'Jade 38':42;'Jade 39':43;'Jade 40':44;'Jade 41':45;'Jade 42':46;'Jade 43':47;'Jade 44':48;'Jade 45':49;'Jade 46':50;'Jade 47':51;'Jade 48':52;'Jade 49':53;'Jade 50':54;'Jade 51':55;'Jade 52':56;'Jade 53':57;'Jade 54':58;'Jade 55':59;'Jade 56':60;'Jade 57':61;'Jade 58':62;'Jade 59':63;'Jade 60':64;'Jade 61':65;'Jade 62':66;'Jade 63':67;'Jade 64':68;'Jade 65':69;'Jade 66':70;'Jade 67':71;'Jade 68':72;'Jade 69':73;'Jade 70':74;'Jade 71':75;'Jade 72':76;'Jade 73':77;'Jade 74':78;'Jade 75':79;'Jade 76':80;'Jade 77':81;'Jade 78':82;'Jade 79':83;'Jade 80':84;'Jade 81':85;'Jade 82':86;'Jade 83':87;'Jade 84':88;'Jade 85':89;'Jade 86':90;'Jade 87':91;'Jade 88':92;'Jade 89':93;'Jade 90':94;'Jade 91':95;'Jade 92':96;'Jade 93':97;'Jade 94':98;'Jade 95':99;'Mech 01':100;'Mech 02':101;'Mech 03':102;'Mech 04':103;'Mech 05':104;'Mech 06':105;'Mech 07':106;'Mech 08':107;'Mech 09':108;'Mech 10':109;'Mech 11':110;'Mech 12':111;'Mech 13':112;'Mech 14':113;'Mech 15':114;'Mech 16':115}");
		ui_interface.add_num_entry("Type", ParamIndex(41), 0.0, 0.0, 115.0, 1.0);
		ui_interface.declare(Some(ParamIndex(42)), "2", "");
		ui_interface.add_horizontal_slider("Decay Env", ParamIndex(42), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(43)), "2", "");
		ui_interface.add_horizontal_slider("Decay LFO", ParamIndex(43), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(44)), "2", "");
		ui_interface.declare(Some(ParamIndex(44)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(44), 6e+01, 5.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(45)), "3", "");
		ui_interface.add_horizontal_slider("Level", ParamIndex(45), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(46)), "3", "");
		ui_interface.add_horizontal_slider("Level Env", ParamIndex(46), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(47)), "3", "");
		ui_interface.add_horizontal_slider("Level LFO", ParamIndex(47), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(48)), "4", "");
		ui_interface.add_check_button("Post-Filter", ParamIndex(48));
		ui_interface.declare(Some(ParamIndex(49)), "5", "");
		ui_interface.add_horizontal_slider("Sample Pitch Env", ParamIndex(49), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(50)), "5", "");
		ui_interface.add_horizontal_slider("Sample Pitch LFO", ParamIndex(50), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(51)), "5", "");
		ui_interface.declare(Some(ParamIndex(51)), "unit", "st");
		ui_interface.add_horizontal_slider("Sample Pitch", ParamIndex(51), 0.0, -24.0, 24.0, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(52)), "1", "");
		ui_interface.add_vertical_slider("Fundamental", ParamIndex(52), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(53)), "1", "");
		ui_interface.add_horizontal_slider("Fundamental Env", ParamIndex(53), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(54)), "1", "");
		ui_interface.add_horizontal_slider("Fundamental LFO", ParamIndex(54), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(55)), "2", "");
		ui_interface.add_vertical_slider("Dual", ParamIndex(55), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(56)), "2", "");
		ui_interface.add_horizontal_slider("Dual Env", ParamIndex(56), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(57)), "2", "");
		ui_interface.add_horizontal_slider("Dual LFO", ParamIndex(57), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(58)), "3", "");
		ui_interface.add_vertical_slider("Wavetable", ParamIndex(58), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(59)), "3", "");
		ui_interface.add_horizontal_slider("Wavetable Env", ParamIndex(59), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(60)), "3", "");
		ui_interface.add_horizontal_slider("Wavetable LFO", ParamIndex(60), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(61)), "4", "");
		ui_interface.add_vertical_slider("Noise", ParamIndex(61), 0.15, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(62)), "4", "");
		ui_interface.add_horizontal_slider("Noise Env", ParamIndex(62), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(63)), "4", "");
		ui_interface.add_horizontal_slider("Noise LFO", ParamIndex(63), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "7", "");
		ui_interface.open_horizontal_box("Drive");
		ui_interface.declare(Some(ParamIndex(64)), "2", "");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(64), 0.1, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(65)), "2", "");
		ui_interface.add_horizontal_slider("Amount Env", ParamIndex(65), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(66)), "2", "");
		ui_interface.add_horizontal_slider("Amount LFO", ParamIndex(66), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("Filter");
		ui_interface.declare(Some(ParamIndex(67)), "1", "");
		ui_interface.add_horizontal_slider("Cutoff Env", ParamIndex(67), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(68)), "1", "");
		ui_interface.add_horizontal_slider("Cutoff LFO", ParamIndex(68), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(69)), "1", "");
		ui_interface.declare(Some(ParamIndex(69)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(69)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Cutoff", ParamIndex(69), 1.2e+03, 2e+01, 1.8e+04, 1.0);
		ui_interface.declare(Some(ParamIndex(70)), "2", "");
		ui_interface.add_horizontal_slider("Resonance", ParamIndex(70), 0.8, 0.5, 6.0, 0.01);
		ui_interface.declare(Some(ParamIndex(71)), "2", "");
		ui_interface.add_horizontal_slider("Resonance Env", ParamIndex(71), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(72)), "2", "");
		ui_interface.add_horizontal_slider("Resonance LFO", ParamIndex(72), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(73)), "3", "");
		ui_interface.add_horizontal_slider("Morph LP-Notch-HP", ParamIndex(73), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(74)), "3", "");
		ui_interface.add_horizontal_slider("Morph LP-Notch-HP Env", ParamIndex(74), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(75)), "3", "");
		ui_interface.add_horizontal_slider("Morph LP-Notch-HP LFO", ParamIndex(75), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(76)), "4", "");
		ui_interface.add_horizontal_slider("Bandpass", ParamIndex(76), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(77)), "4", "");
		ui_interface.add_horizontal_slider("Bandpass Env", ParamIndex(77), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(78)), "4", "");
		ui_interface.add_horizontal_slider("Bandpass LFO", ParamIndex(78), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(79)), "5", "");
		ui_interface.add_check_button("4-Pole", ParamIndex(79));
		ui_interface.declare(Some(ParamIndex(80)), "6", "");
		ui_interface.add_horizontal_slider("Keytrack", ParamIndex(80), 0.5, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(81)), "6", "");
		ui_interface.add_horizontal_slider("Keytrack Env", ParamIndex(81), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(82)), "6", "");
		ui_interface.add_horizontal_slider("Keytrack LFO", ParamIndex(82), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(83)), "7.5", "");
		ui_interface.add_check_button("Invert Env", ParamIndex(83));
		ui_interface.declare(Some(ParamIndex(84)), "7", "");
		ui_interface.add_horizontal_slider("Env Amount Env", ParamIndex(84), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(85)), "7", "");
		ui_interface.add_horizontal_slider("Env Amount LFO", ParamIndex(85), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(86)), "7", "");
		ui_interface.declare(Some(ParamIndex(86)), "unit", "oct");
		ui_interface.add_horizontal_slider("Env Amount", ParamIndex(86), 2.0, 0.0, 5.0, 0.01);
		ui_interface.declare(Some(ParamIndex(87)), "8", "");
		ui_interface.add_horizontal_slider("Utility HPF Env", ParamIndex(87), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(88)), "8", "");
		ui_interface.add_horizontal_slider("Utility HPF LFO", ParamIndex(88), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(89)), "8", "");
		ui_interface.declare(Some(ParamIndex(89)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(89)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Utility HPF", ParamIndex(89), 2e+01, 1e+01, 2e+03, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("Envelope");
		ui_interface.declare(Some(ParamIndex(90)), "1", "");
		ui_interface.declare(Some(ParamIndex(90)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(90)), "unit", "s");
		ui_interface.add_vertical_slider("Attack", ParamIndex(90), 0.005, 0.005, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(91)), "2", "");
		ui_interface.declare(Some(ParamIndex(91)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(91)), "unit", "s");
		ui_interface.add_vertical_slider("Decay", ParamIndex(91), 0.4, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(92)), "3", "");
		ui_interface.add_vertical_slider("Sustain", ParamIndex(92), 0.3, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(93)), "4", "");
		ui_interface.declare(Some(ParamIndex(93)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(93)), "unit", "s");
		ui_interface.add_vertical_slider("Release", ParamIndex(93), 0.5, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(94)), "5", "");
		ui_interface.add_vertical_slider("Curve", ParamIndex(94), 0.5, -1.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(95)), "6", "");
		ui_interface.add_vertical_slider("Amp Sustain +", ParamIndex(95), 0.6, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(96)), "7", "");
		ui_interface.add_vertical_slider("Amp Decay +", ParamIndex(96), 0.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(97)), "hidden", "1");
		ui_interface.add_horizontal_slider("aftertouch", ParamIndex(97), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(98)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(98)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(98), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("freq", ParamIndex(99), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("gain", ParamIndex(100), 0.8, 0.0, 1.0, 0.01);
		ui_interface.add_button("gate", ParamIndex(101));
		ui_interface.add_horizontal_slider("glide", ParamIndex(102), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.declare(Some(ParamIndex(103)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(103)), "midi", "ctrl 1");
		ui_interface.add_horizontal_slider("modwheel", ParamIndex(103), 0.0, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("slot", ParamIndex(104), 0.0, 0.0, 64.0, 1.0);
		ui_interface.add_horizontal_slider("volume", ParamIndex(105), 0.5, 0.0, 1.0, 1e-06);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			101 => Some(self.fButton0),
			40 => Some(self.fCheckbox0),
			48 => Some(self.fCheckbox1),
			79 => Some(self.fCheckbox2),
			83 => Some(self.fCheckbox3),
			9 => Some(self.fEntry0),
			7 => Some(self.fEntry1),
			41 => Some(self.fEntry2),
			25 => Some(self.fEntry3),
			11 => Some(self.fEntry4),
			15 => Some(self.fEntry5),
			14 => Some(self.fEntry6),
			2 => Some(self.fHslider0),
			1 => Some(self.fHslider1),
			3 => Some(self.fHslider10),
			105 => Some(self.fHslider11),
			47 => Some(self.fHslider12),
			46 => Some(self.fHslider13),
			45 => Some(self.fHslider14),
			43 => Some(self.fHslider15),
			42 => Some(self.fHslider16),
			44 => Some(self.fHslider17),
			50 => Some(self.fHslider18),
			49 => Some(self.fHslider19),
			0 => Some(self.fHslider2),
			51 => Some(self.fHslider20),
			66 => Some(self.fHslider21),
			65 => Some(self.fHslider22),
			64 => Some(self.fHslider23),
			12 => Some(self.fHslider24),
			98 => Some(self.fHslider25),
			102 => Some(self.fHslider26),
			6 => Some(self.fHslider27),
			5 => Some(self.fHslider28),
			99 => Some(self.fHslider29),
			10 => Some(self.fHslider3),
			88 => Some(self.fHslider30),
			87 => Some(self.fHslider31),
			89 => Some(self.fHslider32),
			78 => Some(self.fHslider33),
			77 => Some(self.fHslider34),
			76 => Some(self.fHslider35),
			72 => Some(self.fHslider36),
			71 => Some(self.fHslider37),
			70 => Some(self.fHslider38),
			85 => Some(self.fHslider39),
			97 => Some(self.fHslider4),
			84 => Some(self.fHslider40),
			86 => Some(self.fHslider41),
			82 => Some(self.fHslider42),
			81 => Some(self.fHslider43),
			80 => Some(self.fHslider44),
			68 => Some(self.fHslider45),
			67 => Some(self.fHslider46),
			69 => Some(self.fHslider47),
			63 => Some(self.fHslider48),
			62 => Some(self.fHslider49),
			8 => Some(self.fHslider5),
			60 => Some(self.fHslider50),
			59 => Some(self.fHslider51),
			57 => Some(self.fHslider52),
			56 => Some(self.fHslider53),
			54 => Some(self.fHslider54),
			53 => Some(self.fHslider55),
			36 => Some(self.fHslider56),
			35 => Some(self.fHslider57),
			34 => Some(self.fHslider58),
			38 => Some(self.fHslider59),
			103 => Some(self.fHslider6),
			37 => Some(self.fHslider60),
			39 => Some(self.fHslider61),
			32 => Some(self.fHslider62),
			31 => Some(self.fHslider63),
			33 => Some(self.fHslider64),
			30 => Some(self.fHslider65),
			29 => Some(self.fHslider66),
			28 => Some(self.fHslider67),
			26 => Some(self.fHslider68),
			27 => Some(self.fHslider69),
			104 => Some(self.fHslider7),
			20 => Some(self.fHslider70),
			19 => Some(self.fHslider71),
			21 => Some(self.fHslider72),
			24 => Some(self.fHslider73),
			23 => Some(self.fHslider74),
			22 => Some(self.fHslider75),
			18 => Some(self.fHslider76),
			17 => Some(self.fHslider77),
			16 => Some(self.fHslider78),
			75 => Some(self.fHslider79),
			13 => Some(self.fHslider8),
			74 => Some(self.fHslider80),
			73 => Some(self.fHslider81),
			100 => Some(self.fHslider82),
			4 => Some(self.fHslider9),
			61 => Some(self.fVslider0),
			94 => Some(self.fVslider1),
			52 => Some(self.fVslider10),
			93 => Some(self.fVslider2),
			90 => Some(self.fVslider3),
			91 => Some(self.fVslider4),
			96 => Some(self.fVslider5),
			92 => Some(self.fVslider6),
			95 => Some(self.fVslider7),
			58 => Some(self.fVslider8),
			55 => Some(self.fVslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			101 => { self.fButton0 = value }
			40 => { self.fCheckbox0 = value }
			48 => { self.fCheckbox1 = value }
			79 => { self.fCheckbox2 = value }
			83 => { self.fCheckbox3 = value }
			9 => { self.fEntry0 = value }
			7 => { self.fEntry1 = value }
			41 => { self.fEntry2 = value }
			25 => { self.fEntry3 = value }
			11 => { self.fEntry4 = value }
			15 => { self.fEntry5 = value }
			14 => { self.fEntry6 = value }
			2 => { self.fHslider0 = value }
			1 => { self.fHslider1 = value }
			3 => { self.fHslider10 = value }
			105 => { self.fHslider11 = value }
			47 => { self.fHslider12 = value }
			46 => { self.fHslider13 = value }
			45 => { self.fHslider14 = value }
			43 => { self.fHslider15 = value }
			42 => { self.fHslider16 = value }
			44 => { self.fHslider17 = value }
			50 => { self.fHslider18 = value }
			49 => { self.fHslider19 = value }
			0 => { self.fHslider2 = value }
			51 => { self.fHslider20 = value }
			66 => { self.fHslider21 = value }
			65 => { self.fHslider22 = value }
			64 => { self.fHslider23 = value }
			12 => { self.fHslider24 = value }
			98 => { self.fHslider25 = value }
			102 => { self.fHslider26 = value }
			6 => { self.fHslider27 = value }
			5 => { self.fHslider28 = value }
			99 => { self.fHslider29 = value }
			10 => { self.fHslider3 = value }
			88 => { self.fHslider30 = value }
			87 => { self.fHslider31 = value }
			89 => { self.fHslider32 = value }
			78 => { self.fHslider33 = value }
			77 => { self.fHslider34 = value }
			76 => { self.fHslider35 = value }
			72 => { self.fHslider36 = value }
			71 => { self.fHslider37 = value }
			70 => { self.fHslider38 = value }
			85 => { self.fHslider39 = value }
			97 => { self.fHslider4 = value }
			84 => { self.fHslider40 = value }
			86 => { self.fHslider41 = value }
			82 => { self.fHslider42 = value }
			81 => { self.fHslider43 = value }
			80 => { self.fHslider44 = value }
			68 => { self.fHslider45 = value }
			67 => { self.fHslider46 = value }
			69 => { self.fHslider47 = value }
			63 => { self.fHslider48 = value }
			62 => { self.fHslider49 = value }
			8 => { self.fHslider5 = value }
			60 => { self.fHslider50 = value }
			59 => { self.fHslider51 = value }
			57 => { self.fHslider52 = value }
			56 => { self.fHslider53 = value }
			54 => { self.fHslider54 = value }
			53 => { self.fHslider55 = value }
			36 => { self.fHslider56 = value }
			35 => { self.fHslider57 = value }
			34 => { self.fHslider58 = value }
			38 => { self.fHslider59 = value }
			103 => { self.fHslider6 = value }
			37 => { self.fHslider60 = value }
			39 => { self.fHslider61 = value }
			32 => { self.fHslider62 = value }
			31 => { self.fHslider63 = value }
			33 => { self.fHslider64 = value }
			30 => { self.fHslider65 = value }
			29 => { self.fHslider66 = value }
			28 => { self.fHslider67 = value }
			26 => { self.fHslider68 = value }
			27 => { self.fHslider69 = value }
			104 => { self.fHslider7 = value }
			20 => { self.fHslider70 = value }
			19 => { self.fHslider71 = value }
			21 => { self.fHslider72 = value }
			24 => { self.fHslider73 = value }
			23 => { self.fHslider74 = value }
			22 => { self.fHslider75 = value }
			18 => { self.fHslider76 = value }
			17 => { self.fHslider77 = value }
			16 => { self.fHslider78 = value }
			75 => { self.fHslider79 = value }
			13 => { self.fHslider8 = value }
			74 => { self.fHslider80 = value }
			73 => { self.fHslider81 = value }
			100 => { self.fHslider82 = value }
			4 => { self.fHslider9 = value }
			61 => { self.fVslider0 = value }
			94 => { self.fVslider1 = value }
			52 => { self.fVslider10 = value }
			93 => { self.fVslider2 = value }
			90 => { self.fVslider3 = value }
			91 => { self.fVslider4 = value }
			96 => { self.fVslider5 = value }
			92 => { self.fVslider6 = value }
			95 => { self.fVslider7 = value }
			58 => { self.fVslider8 = value }
			55 => { self.fVslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2, inputs3) = if let [inputs0, inputs1, inputs2, inputs3, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			let inputs3 = inputs3[..count as usize].iter();
			(inputs0, inputs1, inputs2, inputs3)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10) = if let [outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			let outputs3 = outputs3[..count as usize].iter_mut();
			let outputs4 = outputs4[..count as usize].iter_mut();
			let outputs5 = outputs5[..count as usize].iter_mut();
			let outputs6 = outputs6[..count as usize].iter_mut();
			let outputs7 = outputs7[..count as usize].iter_mut();
			let outputs8 = outputs8[..count as usize].iter_mut();
			let outputs9 = outputs9[..count as usize].iter_mut();
			let outputs10 = outputs10[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fHslider0;
		let mut fSlow1: F32 = self.fHslider1;
		let mut iSlow2: i32 = ((fSlow1 != 0.0) as i32) | ((fSlow0 != 0.0) as i32);
		let mut fSlow3: F32 = self.fButton0;
		let mut fSlow4: F32 = self.fHslider2;
		let mut fSlow5: F32 = (iSlow2) as F32;
		let mut iSlow6: i32 = (fSlow5 != 0.0) as i32;
		let mut fSlow7: F32 = self.fEntry0;
		let mut iSlow8: i32 = (fSlow7 == 1e+01) as i32;
		let mut fSlow9: F32 = 0.01 * self.fHslider3;
		let mut fSlow10: F32 = self.fHslider4;
		let mut fSlow11: F32 = self.fEntry1;
		let mut iSlow12: i32 = (fSlow11 == 1e+01) as i32;
		let mut fSlow13: F32 = 0.01 * self.fHslider5;
		let mut fSlow14: F32 = self.fHslider6;
		let mut iSlow15: i32 = (fSlow7 == 11.0) as i32;
		let mut iSlow16: i32 = (fSlow11 == 11.0) as i32;
		let mut fTemp10: F32 = self.fTempPerm0;
		let mut fSlow17: F32 = ((iSlow2 != 0) as i32) as u32 as F32;
		let mut fSlow18: F32 = self.fHslider7;
		let mut iSlow19: i32 = (521.0 * fSlow18 + 5e+03) as i32;
		let mut iTemp16: i32 = self.iTempPerm1;
		let mut fSlow20: F32 = self.fHslider8;
		let mut fTemp17: F32 = self.fTempPerm2;
		let mut fSlow21: F32 = self.fHslider9;
		let mut fSlow22: F32 = self.fHslider10;
		let mut iSlow23: i32 = ((fSlow22 != 0.0) as i32) | ((fSlow21 != 0.0) as i32);
		let mut fSlow24: F32 = self.fHslider11;
		let mut fSlow25: F32 = 1.0 / F32::max(1e-06, 1.9912813 * F32::powf(F32::max(0.0, F32::min(1.0, fSlow24)), 4.547648) + 0.0039810715);
		let mut fSlow26: F32 = ((fSlow7 == 23.0) as i32) as u32 as F32;
		let mut fSlow27: F32 = ((fSlow11 == 23.0) as i32) as u32 as F32;
		let mut fSlow28: F32 = ((fSlow7 == 22.0) as i32) as u32 as F32;
		let mut fSlow29: F32 = ((fSlow11 == 22.0) as i32) as u32 as F32;
		let mut fSlow30: F32 = self.fHslider12;
		let mut fSlow31: F32 = self.fHslider13;
		let mut iSlow32: i32 = ((fSlow31 != 0.0) as i32) | ((fSlow30 != 0.0) as i32);
		let mut fSlow33: F32 = self.fHslider14;
		let mut fSlow34: F32 = (iSlow32) as F32;
		let mut iSlow35: i32 = (fSlow34 != 0.0) as i32;
		let mut fTemp25: F32 = self.fTempPerm3;
		let mut fSlow36: F32 = ((iSlow32 != 0) as i32) as u32 as F32;
		let mut iSlow37: i32 = (fSlow33 != 0.0) as i32;
		let mut fSlow38: F32 = (iSlow37) as F32;
		let mut iSlow39: i32 = (fSlow38 != 0.0) as i32;
		let mut fSlow40: F32 = self.fHslider15;
		let mut fSlow41: F32 = self.fHslider16;
		let mut iSlow42: i32 = ((fSlow41 != 0.0) as i32) | ((fSlow40 != 0.0) as i32);
		let mut fSlow43: F32 = self.fHslider17;
		let mut fSlow44: F32 = (iSlow42) as F32;
		let mut iSlow45: i32 = (fSlow44 != 0.0) as i32;
		let mut fSlow46: F32 = F32::max(0.0, 0.0051282053 * (fSlow43 + -5.0));
		let mut fTemp26: F32 = self.fTempPerm4;
		let mut fSlow47: F32 = ((iSlow42 != 0) as i32) as u32 as F32;
		let mut fTemp27: F32 = self.fTempPerm5;
		let mut fTemp28: F32 = self.fTempPerm6;
		let mut fTemp29: F32 = self.fTempPerm7;
		let mut iTemp30: i32 = self.iTempPerm8;
		let mut fTemp31: F32 = self.fTempPerm9;
		let mut fSlow48: F32 = self.fEntry2;
		let mut iSlow49: i32 = (fSlow48 >= 5.0) as i32;
		let mut iSlow50: i32 = (iSlow49 == 1) as i32;
		let mut fSlow51: F32 = (iSlow50) as F32;
		let mut iSlow52: i32 = (fSlow51 != 0.0) as i32;
		let mut iSlow53: i32 = (F32::max(0.0, fSlow48 + -5.0)) as i32;
		let mut fSlow54: F32 = self.fHslider18;
		let mut fSlow55: F32 = self.fHslider19;
		let mut iSlow56: i32 = ((fSlow55 != 0.0) as i32) | ((fSlow54 != 0.0) as i32);
		let mut fSlow57: F32 = self.fHslider20;
		let mut fSlow58: F32 = (iSlow56) as F32;
		let mut iSlow59: i32 = (fSlow58 != 0.0) as i32;
		let mut fSlow60: F32 = F32::max(0.0, 0.020833334 * (fSlow57 + 24.0));
		let mut fTemp32: F32 = self.fTempPerm10;
		let mut fSlow61: F32 = ((iSlow56 != 0) as i32) as u32 as F32;
		let mut fTemp33: F32 = self.fTempPerm11;
		let mut fSlow62: F32 = ((iSlow50 != 0) as i32) as u32 as F32;
		let mut iSlow63: i32 = (iSlow49 == 0) as i32;
		let mut fSlow64: F32 = (iSlow63) as F32;
		let mut iSlow65: i32 = (fSlow64 != 0.0) as i32;
		let mut fSlow66: F32 = F32::min(fSlow48, 4.0);
		let mut iSlow67: i32 = (fSlow66 == 4.0) as i32;
		let mut fSlow68: F32 = (iSlow67) as F32;
		let mut iSlow69: i32 = (fSlow68 != 0.0) as i32;
		let mut fTemp34: F32 = self.fTempPerm12;
		let mut fSlow70: F32 = 12.9898 * fSlow18;
		let mut fSlow71: F32 = 43758.547 * F32::sin(fSlow70 + 234.699);
		let mut fSlow72: F32 = self.fConst7 * (0.1 * (2.0 * (fSlow71 - F32::floor(fSlow71)) + -1.0) + 0.45);
		let mut fTemp35: F32 = self.fTempPerm13;
		let mut iSlow73: i32 = (409.0 * fSlow18 + 4e+03) as i32;
		let mut iTemp36: i32 = self.iTempPerm14;
		let mut fSlow74: F32 = 43758.547 * F32::sin(fSlow70 + 156.466);
		let mut fSlow75: F32 = 0.15 * (2.0 * (fSlow74 - F32::floor(fSlow74)) + -1.0) + 0.25;
		let mut fSlow76: F32 = 1.0 / F32::tan(self.fConst15 * F32::max(1.0, fSlow75));
		let mut fSlow77: F32 = 1.0 - fSlow76;
		let mut fSlow78: F32 = self.fConst7 * F32::max(fSlow75, 0.0);
		let mut fTemp37: F32 = self.fTempPerm15;
		let mut fSlow79: F32 = self.fEntry3;
		let mut iSlow80: i32 = (fSlow79 == 4.0) as i32;
		let mut fSlow81: F32 = (iSlow80) as F32;
		let mut iSlow82: i32 = (fSlow81 != 0.0) as i32;
		let mut fSlow83: F32 = 1.0 - self.fCheckbox0;
		let mut fSlow84: F32 = self.fVslider0;
		let mut iSlow85: i32 = (fSlow84 != 0.0) as i32;
		let mut iSlow86: i32 = (fSlow79 == 1.0) as i32;
		let mut fSlow87: F32 = (iSlow86) as F32;
		let mut iSlow88: i32 = (fSlow87 != 0.0) as i32;
		let mut iSlow89: i32 = (fSlow79 == 0.0) as i32;
		let mut fSlow90: F32 = (iSlow89) as F32;
		let mut iSlow91: i32 = (fSlow90 != 0.0) as i32;
		let mut fSlow92: F32 = ((fSlow11 == 6.0) as i32) as u32 as F32;
		let mut fSlow93: F32 = self.fHslider21;
		let mut fSlow94: F32 = self.fHslider22;
		let mut iSlow95: i32 = ((fSlow94 != 0.0) as i32) | ((fSlow93 != 0.0) as i32);
		let mut fSlow96: F32 = self.fHslider23;
		let mut fSlow97: F32 = (iSlow95) as F32;
		let mut iSlow98: i32 = (fSlow97 != 0.0) as i32;
		let mut fTemp41: F32 = self.fTempPerm16;
		let mut fSlow99: F32 = ((iSlow95 != 0) as i32) as u32 as F32;
		let mut fSlow100: F32 = ((fSlow7 == 6.0) as i32) as u32 as F32;
		let mut fSlow101: F32 = self.fHslider24;
		let mut iTemp71: i32 = self.iTempPerm17;
		let mut fTemp72: F32 = self.fTempPerm18;
		let mut iSlow102: i32 = (fSlow66 == 1.0) as i32;
		let mut fSlow103: F32 = (iSlow102) as F32;
		let mut iSlow104: i32 = (fSlow103 != 0.0) as i32;
		let mut fTemp73: F32 = self.fTempPerm19;
		let mut fTemp74: F32 = self.fTempPerm20;
		let mut fTemp75: F32 = self.fTempPerm21;
		let mut fSlow105: F32 = 1.0 / (fSlow76 + 1.0);
		let mut fSlow106: F32 = 43758.547 * F32::sin(fSlow70 + 78.233);
		let mut fSlow107: F32 = 8.0 * (2.0 * (fSlow106 - F32::floor(fSlow106)) + -1.0);
		let mut fTemp76: F32 = self.fTempPerm22;
		let mut fSlow108: F32 = ((fSlow7 == 21.0) as i32) as u32 as F32;
		let mut fSlow109: F32 = ((fSlow11 == 21.0) as i32) as u32 as F32;
		let mut fSlow110: F32 = self.fHslider25;
		let mut fSlow111: F32 = self.fEntry4;
		let mut fSlow112: F32 = 2.0 * ((fSlow111 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow111 == 1.0) as i32) as u32 as F32;
		let mut fSlow113: F32 = self.fHslider26;
		let mut iSlow114: i32 = (fSlow113 > 0.0) as i32;
		let mut fSlow115: F32 = self.fHslider27;
		let mut fSlow116: F32 = self.fHslider28;
		let mut fSlow117: F32 = 0.0037037036 * (1e+03 * fSlow113 + -3e+01);
		let mut fSlow118: F32 = F32::log(self.fHslider29, std::f32::consts::E);
		let mut fTemp81: F32 = self.fTempPerm23;
		let mut fSlow119: F32 = 0.03125 * fSlow68;
		let mut fTemp82: F32 = self.fTempPerm24;
		let mut fSlow120: F32 = ((iSlow67 != 0) as i32) as u32 as F32;
		let mut iSlow121: i32 = (fSlow66 == 3.0) as i32;
		let mut fSlow122: F32 = (iSlow121) as F32;
		let mut iSlow123: i32 = (fSlow122 != 0.0) as i32;
		let mut fTemp83: F32 = self.fTempPerm25;
		let mut fTemp84: F32 = self.fTempPerm26;
		let mut fTemp85: F32 = self.fTempPerm27;
		let mut fSlow124: F32 = ((iSlow121 != 0) as i32) as u32 as F32;
		let mut iSlow125: i32 = (fSlow66 == 2.0) as i32;
		let mut fSlow126: F32 = (iSlow125) as F32;
		let mut iSlow127: i32 = (fSlow126 != 0.0) as i32;
		let mut fTemp86: F32 = self.fTempPerm28;
		let mut fTemp87: F32 = self.fTempPerm29;
		let mut fSlow128: F32 = ((iSlow125 != 0) as i32) as u32 as F32;
		let mut fSlow129: F32 = self.fConst26 * fSlow103;
		let mut fTemp88: F32 = self.fTempPerm30;
		let mut fSlow130: F32 = ((iSlow102 != 0) as i32) as u32 as F32;
		let mut iSlow131: i32 = (fSlow66 == 0.0) as i32;
		let mut fSlow132: F32 = (iSlow131) as F32;
		let mut iSlow133: i32 = (fSlow132 != 0.0) as i32;
		let mut fTemp89: F32 = self.fTempPerm31;
		let mut fSlow134: F32 = ((iSlow131 != 0) as i32) as u32 as F32;
		let mut fTemp90: F32 = self.fTempPerm32;
		let mut fSlow135: F32 = ((iSlow63 != 0) as i32) as u32 as F32;
		let mut fTemp91: F32 = self.fTempPerm33;
		let mut fTemp92: F32 = self.fTempPerm34;
		let mut fTemp93: F32 = self.fTempPerm35;
		let mut fSlow136: F32 = ((iSlow37 != 0) as i32) as u32 as F32;
		let mut fSlow137: F32 = self.fCheckbox1;
		let mut fSlow138: F32 = fSlow137 * fSlow38 * fSlow136;
		let mut iSlow139: i32 = (fSlow3) as i32;
		let mut fSlow140: F32 = 6.0 * F32::max(-1.0, F32::min(1.0, self.fVslider1));
		let mut fSlow141: F32 = if (F32::abs(-fSlow140) < 0.001) as i32 != 0 {0.001} else {-fSlow140};
		let mut fSlow142: F32 = F32::exp(fSlow141) + -1.0;
		let mut fSlow143: F32 = 43758.547 * F32::sin(fSlow70 + 625.864);
		let mut fSlow144: F32 = 0.25 * (2.0 * (fSlow143 - F32::floor(fSlow143)) + -1.0);
		let mut fSlow145: F32 = self.fVslider2;
		let mut fSlow146: F32 = 1.0 - fSlow3;
		let mut fSlow147: F32 = 43758.547 * F32::sin(fSlow70 + 469.398);
		let mut fSlow148: F32 = 0.25 * (2.0 * (fSlow147 - F32::floor(fSlow147)) + -1.0);
		let mut fSlow149: F32 = self.fVslider3;
		let mut fSlow150: F32 = 43758.547 * F32::sin(fSlow70 + 547.631);
		let mut fSlow151: F32 = 0.25 * (2.0 * (fSlow150 - F32::floor(fSlow150)) + -1.0);
		let mut fSlow152: F32 = self.fVslider4;
		let mut fSlow153: F32 = 1e+01 / fSlow152;
		let mut fSlow154: F32 = self.fVslider5;
		let mut fSlow155: F32 = self.fVslider6;
		let mut fSlow156: F32 = self.fVslider7;
		let mut fSlow157: F32 = 1.0 - fSlow155;
		let mut fSlow158: F32 = if (F32::abs(fSlow140) < 0.001) as i32 != 0 {0.001} else {fSlow140};
		let mut fSlow159: F32 = F32::exp(fSlow158) + -1.0;
		let mut fSlow160: F32 = self.fHslider30;
		let mut fSlow161: F32 = self.fHslider31;
		let mut iSlow162: i32 = ((fSlow161 != 0.0) as i32) | ((fSlow160 != 0.0) as i32);
		let mut fSlow163: F32 = self.fHslider32;
		let mut fSlow164: F32 = (iSlow162) as F32;
		let mut iSlow165: i32 = (fSlow164 != 0.0) as i32;
		let mut fTemp104: F32 = self.fTempPerm36;
		let mut fSlow166: F32 = ((iSlow162 != 0) as i32) as u32 as F32;
		let mut fSlow167: F32 = self.fCheckbox2;
		let mut iSlow168: i32 = (fSlow167) as i32;
		let mut fSlow169: F32 = ((fSlow11 == 4.0) as i32) as u32 as F32;
		let mut fSlow170: F32 = self.fHslider33;
		let mut fSlow171: F32 = self.fHslider34;
		let mut iSlow172: i32 = ((fSlow171 != 0.0) as i32) | ((fSlow170 != 0.0) as i32);
		let mut fSlow173: F32 = self.fHslider35;
		let mut fSlow174: F32 = (iSlow172) as F32;
		let mut iSlow175: i32 = (fSlow174 != 0.0) as i32;
		let mut fTemp108: F32 = self.fTempPerm37;
		let mut fSlow176: F32 = ((iSlow172 != 0) as i32) as u32 as F32;
		let mut fSlow177: F32 = ((fSlow7 == 4.0) as i32) as u32 as F32;
		let mut fSlow178: F32 = 0.5 * ((fSlow111 == 4.0) as i32) as u32 as F32;
		let mut fSlow179: F32 = 0.5 * ((fSlow111 == 3.0) as i32) as u32 as F32;
		let mut fSlow180: F32 = ((fSlow7 == 2.0) as i32) as u32 as F32;
		let mut fSlow181: F32 = ((fSlow11 == 2.0) as i32) as u32 as F32;
		let mut fSlow182: F32 = self.fHslider36;
		let mut fSlow183: F32 = self.fHslider37;
		let mut iSlow184: i32 = ((fSlow183 != 0.0) as i32) | ((fSlow182 != 0.0) as i32);
		let mut fSlow185: F32 = self.fHslider38;
		let mut fSlow186: F32 = (iSlow184) as F32;
		let mut iSlow187: i32 = (fSlow186 != 0.0) as i32;
		let mut fTemp110: F32 = self.fTempPerm38;
		let mut fSlow188: F32 = ((iSlow184 != 0) as i32) as u32 as F32;
		let mut fSlow189: F32 = ((fSlow7 == 5.0) as i32) as u32 as F32;
		let mut fSlow190: F32 = ((fSlow11 == 5.0) as i32) as u32 as F32;
		let mut fSlow191: F32 = self.fHslider39;
		let mut fSlow192: F32 = self.fHslider40;
		let mut iSlow193: i32 = ((fSlow192 != 0.0) as i32) | ((fSlow191 != 0.0) as i32);
		let mut fSlow194: F32 = self.fHslider41;
		let mut fSlow195: F32 = (iSlow193) as F32;
		let mut iSlow196: i32 = (fSlow195 != 0.0) as i32;
		let mut fSlow197: F32 = 5.0 * fSlow195;
		let mut fTemp112: F32 = self.fTempPerm39;
		let mut fSlow198: F32 = ((iSlow193 != 0) as i32) as u32 as F32;
		let mut fSlow199: F32 = (if (self.fCheckbox3) as i32 != 0 {-1} else {1}) as F32;
		let mut fSlow200: F32 = self.fHslider42;
		let mut fSlow201: F32 = self.fHslider43;
		let mut iSlow202: i32 = ((fSlow201 != 0.0) as i32) | ((fSlow200 != 0.0) as i32);
		let mut fSlow203: F32 = self.fHslider44;
		let mut fSlow204: F32 = (iSlow202) as F32;
		let mut iSlow205: i32 = (fSlow204 != 0.0) as i32;
		let mut fSlow206: F32 = F32::max(0.0, fSlow203);
		let mut fTemp113: F32 = self.fTempPerm40;
		let mut fSlow207: F32 = ((iSlow202 != 0) as i32) as u32 as F32;
		let mut fSlow208: F32 = ((fSlow7 == 1.0) as i32) as u32 as F32;
		let mut fSlow209: F32 = ((fSlow11 == 1.0) as i32) as u32 as F32;
		let mut fSlow210: F32 = 0.5 * ((fSlow111 == 2.0) as i32) as u32 as F32;
		let mut fSlow211: F32 = 43758.547 * F32::sin(fSlow70 + 391.165);
		let mut fSlow212: F32 = 0.1 * (2.0 * (fSlow211 - F32::floor(fSlow211)) + -1.0) + 0.2;
		let mut fSlow213: F32 = 1.0 / F32::tan(self.fConst15 * F32::max(1.0, fSlow212));
		let mut fSlow214: F32 = 1.0 - fSlow213;
		let mut fSlow215: F32 = self.fConst7 * F32::max(fSlow212, 0.0);
		let mut fTemp114: F32 = self.fTempPerm41;
		let mut fTemp115: F32 = self.fTempPerm42;
		let mut fSlow216: F32 = 1.0 / (fSlow213 + 1.0);
		let mut fSlow217: F32 = 43758.547 * F32::sin(fSlow70 + 312.932);
		let mut fSlow218: F32 = 0.25 * (2.0 * (fSlow217 - F32::floor(fSlow217)) + -1.0);
		let mut fTemp116: F32 = self.fTempPerm43;
		let mut fSlow219: F32 = self.fHslider45;
		let mut fSlow220: F32 = self.fHslider46;
		let mut iSlow221: i32 = ((fSlow220 != 0.0) as i32) | ((fSlow219 != 0.0) as i32);
		let mut fSlow222: F32 = self.fHslider47;
		let mut fSlow223: F32 = (iSlow221) as F32;
		let mut iSlow224: i32 = (fSlow223 != 0.0) as i32;
		let mut fTemp117: F32 = self.fTempPerm44;
		let mut fSlow225: F32 = ((iSlow221 != 0) as i32) as u32 as F32;
		let mut fSlow226: F32 = ((fSlow11 == 17.0) as i32) as u32 as F32;
		let mut fSlow227: F32 = self.fHslider48;
		let mut fSlow228: F32 = self.fHslider49;
		let mut iSlow229: i32 = ((fSlow228 != 0.0) as i32) | ((fSlow227 != 0.0) as i32);
		let mut fSlow230: F32 = (iSlow229) as F32;
		let mut iSlow231: i32 = (fSlow230 != 0.0) as i32;
		let mut fTemp123: F32 = self.fTempPerm45;
		let mut fSlow232: F32 = ((iSlow229 != 0) as i32) as u32 as F32;
		let mut fSlow233: F32 = ((fSlow7 == 17.0) as i32) as u32 as F32;
		let mut fSlow234: F32 = (iSlow85) as F32;
		let mut fSlow235: F32 = VoiceDsp_faustpower2_f(fSlow234);
		let mut fSlow236: F32 = ((fSlow11 == 2e+01) as i32) as u32 as F32;
		let mut fSlow237: F32 = self.fHslider50;
		let mut fSlow238: F32 = self.fHslider51;
		let mut iSlow239: i32 = ((fSlow238 != 0.0) as i32) | ((fSlow237 != 0.0) as i32);
		let mut fSlow240: F32 = self.fVslider8;
		let mut fSlow241: F32 = (iSlow239) as F32;
		let mut iSlow242: i32 = (fSlow241 != 0.0) as i32;
		let mut fTemp125: F32 = self.fTempPerm46;
		let mut fSlow243: F32 = ((iSlow239 != 0) as i32) as u32 as F32;
		let mut fSlow244: F32 = ((fSlow7 == 2e+01) as i32) as u32 as F32;
		let mut fSlow245: F32 = ((fSlow240 != 0.0) as i32) as u32 as F32;
		let mut fSlow246: F32 = VoiceDsp_faustpower2_f(fSlow245);
		let mut fSlow247: F32 = ((fSlow11 == 19.0) as i32) as u32 as F32;
		let mut fSlow248: F32 = self.fHslider52;
		let mut fSlow249: F32 = self.fHslider53;
		let mut iSlow250: i32 = ((fSlow249 != 0.0) as i32) | ((fSlow248 != 0.0) as i32);
		let mut fSlow251: F32 = self.fVslider9;
		let mut fSlow252: F32 = (iSlow250) as F32;
		let mut iSlow253: i32 = (fSlow252 != 0.0) as i32;
		let mut fTemp127: F32 = self.fTempPerm47;
		let mut fSlow254: F32 = ((iSlow250 != 0) as i32) as u32 as F32;
		let mut fSlow255: F32 = ((fSlow7 == 19.0) as i32) as u32 as F32;
		let mut fSlow256: F32 = ((fSlow251 != 0.0) as i32) as u32 as F32;
		let mut fSlow257: F32 = VoiceDsp_faustpower2_f(fSlow256);
		let mut fSlow258: F32 = ((fSlow11 == -1.0) as i32) as u32 as F32;
		let mut fSlow259: F32 = self.fHslider54;
		let mut fSlow260: F32 = self.fHslider55;
		let mut iSlow261: i32 = ((fSlow260 != 0.0) as i32) | ((fSlow259 != 0.0) as i32);
		let mut fSlow262: F32 = self.fVslider10;
		let mut fSlow263: F32 = (iSlow261) as F32;
		let mut iSlow264: i32 = (fSlow263 != 0.0) as i32;
		let mut fTemp129: F32 = self.fTempPerm48;
		let mut fSlow265: F32 = ((iSlow261 != 0) as i32) as u32 as F32;
		let mut fSlow266: F32 = ((fSlow7 == -1.0) as i32) as u32 as F32;
		let mut iSlow267: i32 = (fSlow262 != 0.0) as i32;
		let mut fSlow268: F32 = (iSlow267) as F32;
		let mut fSlow269: F32 = VoiceDsp_faustpower2_f(fSlow268);
		let mut fSlow270: F32 = self.fHslider56;
		let mut fSlow271: F32 = self.fHslider57;
		let mut iSlow272: i32 = ((fSlow271 != 0.0) as i32) | ((fSlow270 != 0.0) as i32);
		let mut fSlow273: F32 = self.fHslider58;
		let mut fSlow274: F32 = (iSlow272) as F32;
		let mut iSlow275: i32 = (fSlow274 != 0.0) as i32;
		let mut fTemp132: F32 = self.fTempPerm49;
		let mut fSlow276: F32 = ((iSlow272 != 0) as i32) as u32 as F32;
		let mut fSlow277: F32 = self.fHslider59;
		let mut fSlow278: F32 = self.fHslider60;
		let mut iSlow279: i32 = ((fSlow278 != 0.0) as i32) | ((fSlow277 != 0.0) as i32);
		let mut fSlow280: F32 = self.fHslider61;
		let mut fSlow281: F32 = (iSlow279) as F32;
		let mut iSlow282: i32 = (fSlow281 != 0.0) as i32;
		let mut fSlow283: F32 = F32::sqrt(F32::sqrt(F32::max(0.0, 0.12658228 * (fSlow280 + -0.1))));
		let mut fTemp133: F32 = self.fTempPerm50;
		let mut fSlow284: F32 = ((iSlow279 != 0) as i32) as u32 as F32;
		let mut fTemp134: F32 = self.fTempPerm51;
		let mut fTemp135: F32 = self.fTempPerm52;
		let mut fTemp136: F32 = self.fTempPerm53;
		let mut iSlow285: i32 = (613.0 * fSlow18 + 3e+03) as i32;
		let mut iTemp137: i32 = self.iTempPerm54;
		let mut fTemp138: F32 = self.fTempPerm55;
		let mut fTemp139: F32 = self.fTempPerm56;
		let mut iTemp140: i32 = self.iTempPerm57;
		let mut fTemp141: F32 = self.fTempPerm58;
		let mut fSlow286: F32 = ((fSlow7 == 18.0) as i32) as u32 as F32;
		let mut fSlow287: F32 = ((fSlow11 == 18.0) as i32) as u32 as F32;
		let mut fSlow288: F32 = self.fHslider62;
		let mut fSlow289: F32 = self.fHslider63;
		let mut iSlow290: i32 = ((fSlow289 != 0.0) as i32) | ((fSlow288 != 0.0) as i32);
		let mut fSlow291: F32 = self.fHslider64;
		let mut fSlow292: F32 = (iSlow290) as F32;
		let mut iSlow293: i32 = (fSlow292 != 0.0) as i32;
		let mut fTemp142: F32 = self.fTempPerm59;
		let mut fSlow294: F32 = ((iSlow290 != 0) as i32) as u32 as F32;
		let mut fTemp143: F32 = self.fTempPerm60;
		let mut fSlow295: F32 = self.fHslider65;
		let mut fSlow296: F32 = self.fHslider66;
		let mut iSlow297: i32 = ((fSlow296 != 0.0) as i32) | ((fSlow295 != 0.0) as i32);
		let mut fSlow298: F32 = self.fHslider67;
		let mut fSlow299: F32 = (iSlow297) as F32;
		let mut iSlow300: i32 = (fSlow299 != 0.0) as i32;
		let mut fSlow301: F32 = F32::max(0.0, fSlow298);
		let mut fTemp144: F32 = self.fTempPerm61;
		let mut fSlow302: F32 = ((iSlow297 != 0) as i32) as u32 as F32;
		let mut fTemp145: F32 = self.fTempPerm62;
		let mut fTemp146: F32 = self.fTempPerm63;
		let mut fSlow303: F32 = ((iSlow80 != 0) as i32) as u32 as F32;
		let mut iSlow304: i32 = (fSlow79 == 3.0) as i32;
		let mut fSlow305: F32 = (iSlow304) as F32;
		let mut iSlow306: i32 = (fSlow305 != 0.0) as i32;
		let mut iSlow307: i32 = (fSlow79 == 2.0) as i32;
		let mut fSlow308: F32 = (iSlow307) as F32;
		let mut iSlow309: i32 = (fSlow308 != 0.0) as i32;
		let mut fSlow310: F32 = F32::powf(2.0, self.fHslider68);
		let mut fTemp147: F32 = self.fTempPerm64;
		let mut iTemp148: i32 = self.iTempPerm65;
		let mut iTemp149: i32 = self.iTempPerm66;
		let mut fTemp150: F32 = self.fTempPerm67;
		let mut fSlow311: F32 = ((iSlow304 != 0) as i32) as u32 as F32;
		let mut iTemp151: i32 = self.iTempPerm68;
		let mut iTemp152: i32 = self.iTempPerm69;
		let mut fTemp153: F32 = self.fTempPerm70;
		let mut fSlow312: F32 = ((iSlow307 != 0) as i32) as u32 as F32;
		let mut fSlow313: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, self.fHslider69)) + -1.0));
		let mut fSlow314: F32 = fSlow87 / fSlow313;
		let mut fTemp154: F32 = self.fTempPerm71;
		let mut fSlow315: F32 = ((iSlow86 != 0) as i32) as u32 as F32;
		let mut fSlow316: F32 = 4.656613e-10 * fSlow90;
		let mut fTemp155: F32 = self.fTempPerm72;
		let mut fSlow317: F32 = ((iSlow89 != 0) as i32) as u32 as F32;
		let mut fTemp156: F32 = self.fTempPerm73;
		let mut fTemp157: F32 = self.fTempPerm74;
		let mut fTemp158: F32 = self.fTempPerm75;
		let mut fTemp160: F32 = self.fTempPerm76;
		let mut fTemp161: F32 = self.fTempPerm77;
		let mut fTemp162: F32 = self.fTempPerm78;
		let mut fTemp163: F32 = self.fTempPerm79;
		let mut fTemp164: F32 = self.fTempPerm80;
		let mut fTemp165: F32 = self.fTempPerm81;
		let mut fSlow318: F32 = fSlow38 * fSlow136 * (1.0 - fSlow137);
		let mut iSlow319: i32 = (fSlow268 != 0.0) as i32;
		let mut fSlow320: F32 = self.fEntry5;
		let mut iSlow321: i32 = (fSlow320 == 3.0) as i32;
		let mut fSlow322: F32 = (iSlow321) as F32;
		let mut iSlow323: i32 = (fSlow322 != 0.0) as i32;
		let mut fSlow324: F32 = self.fConst7 * F32::exp(0.6931472 * self.fEntry6);
		let mut iSlow325: i32 = (fSlow320 == 2.0) as i32;
		let mut fSlow326: F32 = (iSlow325) as F32;
		let mut iSlow327: i32 = (fSlow326 != 0.0) as i32;
		let mut iSlow328: i32 = (fSlow320 == 1.0) as i32;
		let mut fSlow329: F32 = (iSlow328) as F32;
		let mut iSlow330: i32 = (fSlow329 != 0.0) as i32;
		let mut iSlow331: i32 = (fSlow320 == 0.0) as i32;
		let mut fSlow332: F32 = (iSlow331) as F32;
		let mut iSlow333: i32 = (fSlow332 != 0.0) as i32;
		let mut fTemp166: F32 = self.fTempPerm82;
		let mut fTemp167: F32 = self.fTempPerm83;
		let mut fTemp168: F32 = self.fTempPerm84;
		let mut fTemp169: F32 = self.fTempPerm85;
		let mut fTemp170: F32 = self.fTempPerm86;
		let mut fTemp171: F32 = self.fTempPerm87;
		let mut fSlow334: F32 = 43758.547 * F32::sin(fSlow70 + 704.097);
		let mut fSlow335: F32 = 0.03 * (2.0 * (fSlow334 - F32::floor(fSlow334)) + -1.0);
		let mut fSlow336: F32 = self.fHslider70;
		let mut fSlow337: F32 = self.fHslider71;
		let mut iSlow338: i32 = ((fSlow337 != 0.0) as i32) | ((fSlow336 != 0.0) as i32);
		let mut fSlow339: F32 = 43758.547 * F32::sin(fSlow70 + 782.33);
		let mut fSlow340: F32 = 0.15 * (2.0 * (fSlow339 - F32::floor(fSlow339)) + -1.0);
		let mut fSlow341: F32 = self.fHslider72;
		let mut fSlow342: F32 = (iSlow338) as F32;
		let mut iSlow343: i32 = (fSlow342 != 0.0) as i32;
		let mut fTemp173: F32 = self.fTempPerm88;
		let mut fSlow344: F32 = ((iSlow338 != 0) as i32) as u32 as F32;
		let mut fSlow345: F32 = ((fSlow11 == 15.0) as i32) as u32 as F32;
		let mut fSlow346: F32 = self.fHslider73;
		let mut fSlow347: F32 = self.fHslider74;
		let mut iSlow348: i32 = ((fSlow347 != 0.0) as i32) | ((fSlow346 != 0.0) as i32);
		let mut fSlow349: F32 = self.fHslider75;
		let mut fSlow350: F32 = (iSlow348) as F32;
		let mut iSlow351: i32 = (fSlow350 != 0.0) as i32;
		let mut fTemp175: F32 = self.fTempPerm89;
		let mut fSlow352: F32 = ((iSlow348 != 0) as i32) as u32 as F32;
		let mut fSlow353: F32 = ((fSlow7 == 15.0) as i32) as u32 as F32;
		let mut fSlow354: F32 = ((fSlow7 == 14.0) as i32) as u32 as F32;
		let mut fSlow355: F32 = ((fSlow11 == 14.0) as i32) as u32 as F32;
		let mut fSlow356: F32 = self.fHslider76;
		let mut fSlow357: F32 = self.fHslider77;
		let mut iSlow358: i32 = ((fSlow357 != 0.0) as i32) | ((fSlow356 != 0.0) as i32);
		let mut fSlow359: F32 = self.fHslider78;
		let mut fSlow360: F32 = (iSlow358) as F32;
		let mut iSlow361: i32 = (fSlow360 != 0.0) as i32;
		let mut fTemp176: F32 = self.fTempPerm90;
		let mut fSlow362: F32 = ((iSlow358 != 0) as i32) as u32 as F32;
		let mut fTemp178: F32 = self.fTempPerm91;
		let mut fTemp179: F32 = self.fTempPerm92;
		let mut fTemp180: F32 = self.fTempPerm93;
		let mut fTemp181: F32 = self.fTempPerm94;
		let mut fTemp182: F32 = self.fTempPerm95;
		let mut fSlow363: F32 = ((iSlow321 != 0) as i32) as u32 as F32;
		let mut fTemp183: F32 = self.fTempPerm96;
		let mut fSlow364: F32 = ((iSlow325 != 0) as i32) as u32 as F32;
		let mut fTemp184: F32 = self.fTempPerm97;
		let mut fSlow365: F32 = ((iSlow328 != 0) as i32) as u32 as F32;
		let mut fTemp185: F32 = self.fTempPerm98;
		let mut fSlow366: F32 = ((iSlow331 != 0) as i32) as u32 as F32;
		let mut fTemp186: F32 = self.fTempPerm99;
		let mut fSlow367: F32 = fSlow268 * ((iSlow267 != 0) as i32) as u32 as F32;
		let mut fTemp187: F32 = self.fTempPerm100;
		let mut fTemp188: F32 = self.fTempPerm101;
		let mut fTemp189: F32 = self.fTempPerm102;
		let mut fTemp190: F32 = self.fTempPerm103;
		let mut fTemp191: F32 = self.fTempPerm104;
		let mut fTemp192: F32 = self.fTempPerm105;
		let mut fTemp193: F32 = self.fTempPerm106;
		let mut fTemp194: F32 = self.fTempPerm107;
		let mut fTemp195: F32 = self.fTempPerm108;
		let mut fTemp196: F32 = self.fTempPerm109;
		let mut fTemp197: F32 = self.fTempPerm110;
		let mut fTemp198: F32 = self.fTempPerm111;
		let mut fTemp199: F32 = self.fTempPerm112;
		let mut fTemp200: F32 = self.fTempPerm113;
		let mut fTemp201: F32 = self.fTempPerm114;
		let mut fTemp203: F32 = self.fTempPerm115;
		let mut fTemp204: F32 = self.fTempPerm116;
		let mut fTemp205: F32 = self.fTempPerm117;
		let mut fTemp206: F32 = self.fTempPerm118;
		let mut fTemp207: F32 = self.fTempPerm119;
		let mut fTemp208: F32 = self.fTempPerm120;
		let mut fTemp209: F32 = self.fTempPerm121;
		let mut fTemp210: F32 = self.fTempPerm122;
		let mut fSlow368: F32 = ((fSlow11 == 3.0) as i32) as u32 as F32;
		let mut fSlow369: F32 = self.fHslider79;
		let mut fSlow370: F32 = self.fHslider80;
		let mut iSlow371: i32 = ((fSlow370 != 0.0) as i32) | ((fSlow369 != 0.0) as i32);
		let mut fSlow372: F32 = self.fHslider81;
		let mut fSlow373: F32 = (iSlow371) as F32;
		let mut iSlow374: i32 = (fSlow373 != 0.0) as i32;
		let mut fTemp217: F32 = self.fTempPerm123;
		let mut fSlow375: F32 = ((iSlow371 != 0) as i32) as u32 as F32;
		let mut fSlow376: F32 = ((fSlow7 == 3.0) as i32) as u32 as F32;
		let mut iSlow377: i32 = (fSlow167 != 0.0) as i32;
		let mut fTemp228: F32 = self.fTempPerm124;
		let mut fTemp229: F32 = self.fTempPerm125;
		let mut fTemp230: F32 = self.fTempPerm126;
		let mut fTemp231: F32 = self.fTempPerm127;
		let mut fTemp232: F32 = self.fTempPerm128;
		let mut fTemp233: F32 = self.fTempPerm129;
		let mut fTemp234: F32 = self.fTempPerm130;
		let mut fTemp235: F32 = self.fTempPerm131;
		let mut fSlow378: F32 = (iSlow377) as F32;
		let mut fSlow379: F32 = 0.5 * self.fHslider82;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7).zip(outputs8).zip(outputs9).zip(outputs10);
		for ((((((((((((((input0, input1), input2), input3), output0), output1), output2), output3), output4), output5), output6), output7), output8), output9), output10) in zipped_iterators {
			let mut fRec86: F32 = 0.0;
			let mut fRec87: F32 = 0.0;
			let mut fRec88: F32 = 0.0;
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow3;
			let mut iTemp0: i32 = (fSlow3 > self.fVec1[1]) as i32;
			let mut iTemp1: i32 = i32::wrapping_sub(1, iTemp0);
			let mut fTemp2: F32 = (iTemp1) as F32;
			let mut fTemp3: F32 = 1.0 - self.fConst1 * fTemp2;
			self.fRec0[0] = fSlow4 * fTemp3 + self.fConst1 * fTemp2 * self.fRec0[1];
			self.fRec1[0] = fSlow9 * fTemp3 + self.fConst1 * fTemp2 * self.fRec1[1];
			self.fRec2[0] = fSlow10 * fTemp3 + self.fConst1 * fTemp2 * self.fRec2[1];
			let mut fTemp4: F32 = self.fRec1[0] * self.fRec2[0];
			let mut fTemp5: F32 = if (self.fRec1[0] >= 0.0) as i32 != 0 {1.0 - self.fRec1[0] * (1.0 - self.fRec2[0])} else {fTemp4 + 1.0};
			self.fRec3[0] = fSlow13 * fTemp3 + self.fConst1 * fTemp2 * self.fRec3[1];
			self.fRec4[0] = fSlow14 * fTemp3 + self.fConst1 * fTemp2 * self.fRec4[1];
			let mut fTemp6: F32 = self.fRec3[0] * self.fRec4[0];
			let mut fTemp7: F32 = if (self.fRec3[0] >= 0.0) as i32 != 0 {1.0 - self.fRec3[0] * (1.0 - self.fRec4[0])} else {fTemp6 + 1.0};
			let mut fTemp8: F32 = *input0 * if iSlow12 != 0 {fTemp7} else {1.0} * if iSlow8 != 0 {fTemp5} else {1.0};
			let mut fTemp9: F32 = *input1 * if iSlow16 != 0 {fTemp7} else {1.0} * if iSlow15 != 0 {fTemp5} else {1.0};
			if iSlow6 != 0 {
				fTemp10 = fSlow5 * F32::max(0.0, F32::min(1.0, fSlow0 * fTemp9 + F32::max(0.0, self.fRec0[0]) + fSlow1 * fTemp8));
			}
				let mut fTemp11: F32 = if iSlow2 != 0 {fSlow17 * fTemp10} else {self.fRec0[0]};
				let mut iTemp12: i32 = (F32::abs(fTemp11) > 0.0001) as i32;
				let mut fTemp13: F32 = (iTemp12) as F32;
				let mut iTemp14: i32 = (fTemp13 != 0.0) as i32;
				let mut iTemp15: i32 = i32::wrapping_sub(1, self.iVec0[1]);
				if iTemp14 != 0 {
					self.iRec6[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec6[1], i32::wrapping_mul(iSlow19, iTemp15))), 12345);
					iTemp16 = i32::wrapping_mul(1664525, self.iRec6[0] ^ (self.iRec6[0] >> 15));
					self.fRec5[0] = self.fConst2 * self.fRec5[1] + self.fConst4 * (i32::wrapping_mul(1013904223, iTemp16 ^ (iTemp16 >> 13))) as F32;
					self.fRec7[0] = fSlow20 * fTemp3 + self.fConst1 * fTemp2 * self.fRec7[1];
					fTemp17 = self.fConst5 * self.fRec7[0] * self.fRec5[0] * fTemp11 * fTemp13;
				}
					let mut fTemp18: F32 = ((iTemp12 != 0) as i32) as u32 as F32;
					let mut fTemp19: F32 = fTemp18 * fTemp17;
					let mut fTemp20: F32 = F32::floor(fTemp19);
					let mut iTemp21: i32 = (fTemp20) as i32;
					let mut fTemp22: F32 = fTemp19 - fTemp20;
					let mut fTemp23: F32 = if iTemp15 != 0 {0.0} else {self.fConst6 + self.fRec9[1]};
					self.fRec9[0] = fTemp23 - F32::floor(fTemp23);
					let mut fTemp24: F32 = unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec9[0]) as i32, 65535))) as usize] };
					self.fRec10[0] = fSlow33 * fTemp3 + self.fConst1 * fTemp2 * self.fRec10[1];
					if iSlow35 != 0 {
						fTemp25 = fSlow34 * F32::max(0.0, F32::min(1.0, fSlow30 * fTemp9 + F32::max(0.0, self.fRec10[0]) + fSlow31 * fTemp8));
					}
						if iSlow39 & iSlow45 != 0 {
							fTemp26 = fSlow44 * (195.0 * F32::max(0.0, F32::min(1.0, fSlow46 + fSlow41 * fTemp8 + fSlow40 * fTemp9)) + 5.0);
						}
							if iSlow39 != 0 {
								fTemp27 = F32::max(0.005, 0.001 * if iSlow42 != 0 {fSlow47 * fTemp26} else {fSlow43});
								self.iRec11[0] = i32::wrapping_mul(iTemp1, i32::wrapping_add(self.iRec11[1], 1));
								fTemp28 = (self.iRec11[0]) as F32;
								fTemp29 = self.fConst7 * fTemp28;
								self.iRec12[0] = std::cmp::max(iTemp0, self.iRec12[1]);
								iTemp30 = i32::wrapping_mul(self.iRec12[0], (fTemp29 < (fTemp27 + 0.05)) as i32);
								fTemp31 = (iTemp30) as F32;
							}
								if iSlow39 & (iSlow52 & (iSlow59 & ((fTemp31 != 0.0) as i32))) != 0 {
									fTemp32 = fSlow58 * (48.0 * F32::max(0.0, F32::min(1.0, fSlow60 + fSlow55 * fTemp8 + fSlow54 * fTemp9)) + -24.0);
								}
									if iSlow39 & (iSlow52 & ((fTemp31 != 0.0) as i32)) != 0 {
										fTemp33 = fSlow51 * tr_read(iSlow53, self.fConst8 * fTemp28 * F32::exp(0.057762265 * if iSlow56 != 0 {fSlow61 * fTemp32} else {fSlow57})) * F32::min(1.0, F32::max(0.0, 2.5 * ((fTemp27 - fTemp29) / fTemp27)));
									}
										if iTemp14 != 0 {
											fTemp34 = if iTemp15 != 0 {0.0} else {self.fConst10 + self.fRec16[1]};
											self.fRec16[0] = fTemp34 - F32::floor(fTemp34);
											fTemp35 = if iTemp15 != 0 {0.0} else {fSlow72 + self.fRec17[1]};
											self.fRec17[0] = fTemp35 - F32::floor(fTemp35);
											self.iRec19[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec19[1], i32::wrapping_mul(iSlow73, iTemp15))), 12345);
											iTemp36 = i32::wrapping_mul(1664525, self.iRec19[0] ^ (self.iRec19[0] >> 15));
											self.fRec18[0] = self.fConst11 * self.fRec18[1] + self.fConst13 * (i32::wrapping_mul(1013904223, iTemp36 ^ (iTemp36 >> 13))) as F32;
											fTemp37 = if iTemp15 != 0 {0.0} else {fSlow78 + self.fRec22[1]};
											self.fRec22[0] = fTemp37 - F32::floor(fTemp37);
										}
											self.fRec24[0] = fSlow83 * fTemp3 + self.fConst1 * fTemp2 * self.fRec24[1];
											let mut fTemp38: F32 = self.fRec24[0] * ((self.fRec24[0] > 0.0001) as i32) as u32 as F32;
											let mut iTemp39: i32 = i32::wrapping_mul(iSlow85, (fTemp38 != 0.0) as i32);
											let mut fTemp40: F32 = (iTemp39) as F32;
											self.fRec25[0] = fSlow96 * fTemp3 + self.fConst1 * fTemp2 * self.fRec25[1];
											if iSlow98 != 0 {
												fTemp41 = fSlow97 * F32::max(0.0, F32::min(1.0, fSlow93 * fTemp9 + F32::max(0.0, self.fRec25[0]) + fSlow94 * fTemp8));
											}
												let mut fTemp42: F32 = F32::max(0.0, F32::min(1.0, fSlow100 * fTemp4 + if iSlow95 != 0 {fSlow99 * fTemp41} else {self.fRec25[0]} + fSlow92 * fTemp6));
												let mut fTemp43: F32 = F32::max(0.0, F32::min(1.0, fTemp42));
												let mut fTemp44: F32 = 4.0 * fTemp43;
												let mut fTemp45: F32 = F32::max(0.0, F32::min(2.0, F32::floor(fTemp44 + -0.5)));
												let mut fTemp46: F32 = fTemp45 + 1.0;
												let mut iTemp47: i32 = (fTemp46 == 1.0) as i32;
												let mut fTemp48: F32 = (iTemp47) as F32;
												let mut iTemp49: i32 = (fTemp48 != 0.0) as i32;
												let mut iTemp50: i32 = (fTemp46 == 0.0) as i32;
												let mut fTemp51: F32 = (iTemp50) as F32;
												let mut iTemp52: i32 = (fTemp51 != 0.0) as i32;
												let mut iTemp53: i32 = (fTemp46 == 2.0) as i32;
												let mut fTemp54: F32 = (iTemp53) as F32;
												let mut iTemp55: i32 = (fTemp54 != 0.0) as i32;
												let mut iTemp56: i32 = (fTemp46 == 3.0) as i32;
												let mut fTemp57: F32 = (iTemp56) as F32;
												let mut iTemp58: i32 = (fTemp57 != 0.0) as i32;
												let mut iTemp59: i32 = (fTemp45 == 1.0) as i32;
												let mut fTemp60: F32 = (iTemp59) as F32;
												let mut iTemp61: i32 = (fTemp60 != 0.0) as i32;
												let mut iTemp62: i32 = (fTemp45 == 0.0) as i32;
												let mut fTemp63: F32 = (iTemp62) as F32;
												let mut iTemp64: i32 = (fTemp63 != 0.0) as i32;
												let mut iTemp65: i32 = (fTemp45 == 2.0) as i32;
												let mut fTemp66: F32 = (iTemp65) as F32;
												let mut iTemp67: i32 = (fTemp66 != 0.0) as i32;
												let mut iTemp68: i32 = (fTemp45 == 3.0) as i32;
												let mut fTemp69: F32 = (iTemp68) as F32;
												let mut iTemp70: i32 = (fTemp69 != 0.0) as i32;
												if iTemp70 | (iTemp67 | (iTemp64 | (iTemp61 | (iTemp58 | (iTemp55 | (iTemp52 | iTemp49)))))) != 0 {
													self.fRec26[0] = fSlow101 * fTemp3 + self.fConst1 * fTemp2 * self.fRec26[1];
													iTemp71 = (self.fRec26[0] > 0.0) as i32;
													fTemp72 = (iTemp71) as F32;
												}
													if iTemp14 | ((((fTemp72 != 0.0) as i32) & iTemp70) | ((iSlow39 & (iSlow65 & (iSlow104 & ((fTemp31 != 0.0) as i32)))) | ((iSlow39 & (iSlow65 & (iSlow69 & ((fTemp31 != 0.0) as i32)))) | ((((fTemp72 != 0.0) as i32) & iTemp67) | ((((fTemp72 != 0.0) as i32) & iTemp64) | ((((fTemp72 != 0.0) as i32) & iTemp61) | ((((fTemp72 != 0.0) as i32) & iTemp58) | ((((fTemp72 != 0.0) as i32) & iTemp55) | ((((fTemp72 != 0.0) as i32) & iTemp52) | ((((fTemp72 != 0.0) as i32) & iTemp49) | ((((fTemp40 != 0.0) as i32) & iSlow91) | ((((fTemp40 != 0.0) as i32) & iSlow88) | (((fTemp40 != 0.0) as i32) & iSlow82))))))))))))) != 0 {
														self.iRec23[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec23[1]), 12345);
														fTemp73 = (self.iRec23[0]) as F32;
													}
														if iTemp14 | ((((fTemp72 != 0.0) as i32) & iTemp70) | ((iSlow39 & (iSlow65 & (iSlow104 & ((fTemp31 != 0.0) as i32)))) | ((iSlow39 & (iSlow65 & (iSlow69 & ((fTemp31 != 0.0) as i32)))) | ((((fTemp72 != 0.0) as i32) & iTemp67) | ((((fTemp72 != 0.0) as i32) & iTemp64) | ((((fTemp72 != 0.0) as i32) & iTemp61) | ((((fTemp72 != 0.0) as i32) & iTemp58) | ((((fTemp72 != 0.0) as i32) & iTemp55) | ((((fTemp72 != 0.0) as i32) & iTemp52) | ((((fTemp72 != 0.0) as i32) & iTemp49) | ((((fTemp40 != 0.0) as i32) & iSlow88) | (((fTemp40 != 0.0) as i32) & iSlow82)))))))))))) != 0 {
															fTemp74 = 4.656613e-10 * fTemp73;
														}
															if iTemp14 != 0 {
																self.fRec21[0] = if (self.fRec22[0] < self.fRec22[1]) as i32 != 0 {fTemp74} else {self.fRec21[1]};
																fTemp75 = F32::floor(32768.0 * self.fRec21[0] + 0.5);
																self.fVec3[0] = fTemp75;
																self.fRec20[0] = fSlow105 * (3.0517578e-05 * (fTemp75 + self.fVec3[1]) - fSlow77 * self.fRec20[1]);
																fTemp76 = fTemp13 * (fTemp11 * (fSlow107 + 2e+01 * self.fRec20[0] + self.fConst14 * self.fRec7[0] * self.fRec18[0]) + VoiceDsp_faustpower2_f(fTemp11) * (18.0 * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec17[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec16[0]) as i32, 65535))) as usize] }));
															}
																self.fRec27[0] = fSlow110 * fTemp3 + self.fConst1 * fTemp2 * self.fRec27[1];
																let mut fTemp77: F32 = 0.33333334 * F32::max(1e-05, if iSlow114 != 0 {0.001 * (2.7e+02 * F32::max(0.0, F32::min(1.0, fSlow117 + fSlow116 * fTemp8 + fSlow115 * fTemp9)) + 3e+01)} else {0.0});
																let mut iTemp78: i32 = (F32::abs(fTemp77) < 1.1920929e-07) as i32;
																let mut fTemp79: F32 = if iTemp78 != 0 {0.0} else {F32::exp(-(self.fConst7 / if iTemp78 != 0 {1.0} else {fTemp77}))};
																self.fRec28[0] = fSlow118 * (1.0 - fTemp79) + fTemp79 * self.fRec28[1];
																let mut fTemp80: F32 = F32::exp(self.fRec28[0]) * F32::exp(0.057762265 * (fSlow112 * self.fRec27[0] + 0.5 * (fSlow109 * fTemp6 + fSlow108 * fTemp4) * fTemp24 + 0.01 * fTemp18 * fTemp76));
																if iSlow39 & (iSlow65 & (iSlow69 & ((fTemp31 != 0.0) as i32))) != 0 {
																	fTemp81 = if iTemp15 != 0 {0.0} else {self.fRec15[1] + self.fConst7 * F32::max(4.0 * fTemp80, 0.0)};
																	self.fRec15[0] = fTemp81 - F32::floor(fTemp81);
																	self.fRec14[0] = if (self.fRec15[0] < self.fRec15[1]) as i32 != 0 {fTemp74} else {self.fRec14[1]};
																	fTemp82 = fSlow119 * F32::floor(32.0 * self.fRec14[0] + 0.5);
																}
																	if iSlow39 & (iSlow65 & (iSlow123 & ((fTemp31 != 0.0) as i32))) != 0 {
																		fTemp83 = if iTemp15 != 0 {0.0} else {self.fRec30[1] + self.fConst16 * fTemp80};
																		self.fRec30[0] = fTemp83 - F32::floor(fTemp83);
																		fTemp84 = if iTemp15 != 0 {0.0} else {self.fRec29[1] + self.fConst7 * fTemp80 * (4.0 * F32::exp(-(self.fConst17 * fTemp28)) * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec30[0]) as i32, 65535))) as usize] } + 3.53)};
																		self.fRec29[0] = fTemp84 - F32::floor(fTemp84);
																		fTemp85 = fSlow122 * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec29[0]) as i32, 65535))) as usize] };
																	}
																		if iSlow39 & (iSlow65 & (iSlow127 & ((fTemp31 != 0.0) as i32))) != 0 {
																			fTemp86 = if iTemp15 != 0 {0.0} else {self.fRec31[1] + self.fConst7 * fTemp80 * (7.0 * F32::exp(-(self.fConst18 * fTemp28)) + 1.0)};
																			self.fRec31[0] = fTemp86 - F32::floor(fTemp86);
																			fTemp87 = fSlow126 * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec31[0]) as i32, 65535))) as usize] };
																		}
																			if iSlow39 & (iSlow65 & (iSlow104 & ((fTemp31 != 0.0) as i32))) != 0 {
																				self.fRec32[0] = fTemp74 - self.fConst25 * (self.fConst23 * self.fRec32[2] + self.fConst21 * self.fRec32[1]);
																				fTemp88 = fSlow129 * (self.fRec32[2] + (self.fRec32[0] - 2.0 * self.fRec32[1]));
																			}
																				if iSlow39 & (iSlow65 & (iSlow133 & ((fTemp31 != 0.0) as i32))) != 0 {
																					fTemp89 = fSlow132 * F32::sin(self.fConst28 * fTemp28) * F32::exp(-(self.fConst27 * fTemp28));
																				}
																					if iSlow39 & (iSlow65 & ((fTemp31 != 0.0) as i32)) != 0 {
																						fTemp90 = fSlow64 * (fSlow134 * fTemp89 + fSlow130 * fTemp88 + fSlow128 * fTemp87 + fSlow124 * fTemp85 + fSlow120 * fTemp82) * F32::exp(-(self.fConst9 * (fTemp28 / fTemp27))) * ((fTemp29 < fTemp27) as i32) as u32 as F32;
																					}
																						if iSlow39 & ((fTemp31 != 0.0) as i32) != 0 {
																							fTemp91 = (self.iRec12[0]) as F32 * (fSlow135 * fTemp90 + fSlow62 * fTemp33);
																							self.fVec4[0] = fTemp91;
																							self.fRec13[0] = 0.995 * self.fRec13[1] + fTemp91 - self.fVec4[1];
																							fTemp92 = self.fRec13[0] * fTemp31;
																						}
																							if iSlow39 != 0 {
																								fTemp93 = fSlow38 * fTemp92 * ((iTemp30 != 0) as i32) as u32 as F32;
																							}
																								let mut fTemp94: F32 = fTemp93 * if iSlow32 != 0 {fSlow36 * fTemp25} else {self.fRec10[0]};
																								self.fRec34[0] = fSlow146 * (self.fRec34[1] + 1.0);
																								let mut fTemp95: F32 = 1.0 - (F32::exp(fSlow141 * F32::min(1.0, self.fConst7 * (self.fRec34[0] / F32::max(fSlow145 * (fSlow144 * fTemp11 + 1.0), 0.0001)))) + -1.0) / fSlow142;
																								let mut iTemp96: i32 = (fSlow3 < self.fVec1[1]) as i32;
																								self.fRec35[0] = if iTemp96 != 0 {self.fRec33[1]} else {self.fRec35[1]};
																								let mut fTemp97: F32 = F32::max(F32::max(0.005, fSlow149 * (fSlow148 * fTemp11 + 1.0)), 0.0001);
																								self.fRec36[0] = fSlow3 * (self.fRec36[1] + 1.0);
																								let mut fTemp98: F32 = self.fConst7 * self.fRec36[0];
																								let mut iTemp99: i32 = (fTemp98 < fTemp97) as i32;
																								let mut fTemp100: F32 = fSlow151 * fTemp11 + 1.0;
																								self.fRec37[0] = fSlow154 * fTemp3 + self.fConst1 * fTemp2 * self.fRec37[1];
																								let mut fTemp101: F32 = fTemp98 - fTemp97;
																								self.fRec38[0] = fSlow156 * fTemp3 + self.fConst1 * fTemp2 * self.fRec38[1];
																								let mut fTemp102: F32 = fSlow157 * self.fRec38[0];
																								let mut fTemp103: F32 = F32::exp(fSlow158 * F32::min(1.0, self.fConst7 * (self.fRec36[0] / fTemp97))) + -1.0;
																								self.fRec39[0] = if iTemp0 != 0 {self.fRec33[1]} else {self.fRec39[1]};
																								self.fRec33[0] = if iSlow139 != 0 {if iTemp99 != 0 {self.fRec39[0] + (1.0 - self.fRec39[0]) * fTemp103 / fSlow159} else {fSlow155 + fTemp102 + (1.0 - fTemp102 - fSlow155) * (1.0 - (F32::exp(fSlow141 * F32::min(1.0, fTemp101 / F32::max(fSlow152 * fTemp100 * F32::powf(fSlow153 / fTemp100, self.fRec37[0]), 0.0001))) + -1.0) / fSlow142)}} else {self.fRec35[0] * fTemp95};
																								self.fRec41[0] = fSlow163 * fTemp3 + self.fConst1 * fTemp2 * self.fRec41[1];
																								if iSlow165 != 0 {
																									fTemp104 = fSlow164 * (1.99e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, fSlow160 * fTemp9 + F32::sqrt(F32::sqrt(F32::max(0.0, 0.00050251256 * (self.fRec41[0] + -1e+01)))) + fSlow161 * fTemp8))) + 1e+01);
																								}
																									let mut fTemp105: F32 = if iSlow162 != 0 {fSlow166 * fTemp104} else {self.fRec41[0]};
																									let mut fTemp106: F32 = F32::tan(self.fConst15 * F32::max(1e+01, F32::min(2e+03, fTemp105)));
																									let mut fTemp107: F32 = 1.0 / fTemp106;
																									self.fRec42[0] = fSlow173 * fTemp3 + self.fConst1 * fTemp2 * self.fRec42[1];
																									if iSlow175 != 0 {
																										fTemp108 = fSlow174 * F32::max(0.0, F32::min(1.0, fSlow170 * fTemp9 + F32::max(0.0, self.fRec42[0]) + fSlow171 * fTemp8));
																									}
																										let mut fTemp109: F32 = F32::max(0.0, F32::min(1.0, fSlow178 * self.fRec27[0] + fSlow177 * fTemp4 + if iSlow172 != 0 {fSlow176 * fTemp108} else {self.fRec42[0]} + fSlow169 * fTemp6));
																										self.fRec48[0] = fSlow185 * fTemp3 + self.fConst1 * fTemp2 * self.fRec48[1];
																										if iSlow187 != 0 {
																											fTemp110 = fSlow186 * (5.5 * F32::max(0.0, F32::min(1.0, fSlow182 * fTemp9 + F32::max(0.0, 0.18181819 * (self.fRec48[0] + -0.5)) + fSlow183 * fTemp8)) + 0.5);
																										}
																											let mut fTemp111: F32 = F32::max(0.5, F32::min(8.0, if iSlow184 != 0 {fSlow188 * fTemp110} else {self.fRec48[0]} * F32::exp(0.6931472 * (fSlow181 * fTemp6 + fSlow180 * fTemp4)) * (fSlow179 * self.fRec27[0] + 1.0)));
																											self.fRec49[0] = fSlow194 * fTemp3 + self.fConst1 * fTemp2 * self.fRec49[1];
																											if iSlow196 != 0 {
																												fTemp112 = fSlow197 * F32::max(0.0, F32::min(1.0, fSlow191 * fTemp9 + F32::max(0.0, 0.2 * self.fRec49[0]) + fSlow192 * fTemp8));
																											}
																												self.fRec51[0] = if iTemp96 != 0 {self.fRec50[1]} else {self.fRec51[1]};
																												self.fRec52[0] = if iTemp0 != 0 {self.fRec50[1]} else {self.fRec52[1]};
																												self.fRec50[0] = if iSlow139 != 0 {if iTemp99 != 0 {self.fRec52[0] + (1.0 - self.fRec52[0]) * fTemp103 / fSlow159} else {fSlow155 + fSlow157 * (1.0 - (F32::exp(fSlow141 * F32::min(1.0, fTemp101 / F32::max(fSlow152 * fTemp100, 0.0001))) + -1.0) / fSlow142)}} else {self.fRec51[0] * fTemp95};
																												if iSlow205 != 0 {
																													fTemp113 = fSlow204 * F32::max(0.0, F32::min(1.0, fSlow206 + fSlow201 * fTemp8 + fSlow200 * fTemp9));
																												}
																													if iTemp14 != 0 {
																														fTemp114 = if iTemp15 != 0 {0.0} else {fSlow215 + self.fRec55[1]};
																														self.fRec55[0] = fTemp114 - F32::floor(fTemp114);
																														self.fRec54[0] = if (self.fRec55[0] < self.fRec55[1]) as i32 != 0 {fTemp74} else {self.fRec54[1]};
																														fTemp115 = F32::floor(32768.0 * self.fRec54[0] + 0.5);
																														self.fVec5[0] = fTemp115;
																														self.fRec53[0] = fSlow216 * (3.0517578e-05 * (fTemp115 + self.fVec5[1]) - fSlow214 * self.fRec53[1]);
																														fTemp116 = fTemp11 * (fSlow218 + 0.1 * self.fRec53[0]) * fTemp13;
																													}
																														self.fRec56[0] = fSlow222 * fTemp3 + self.fConst1 * fTemp2 * self.fRec56[1];
																														if iSlow224 != 0 {
																															fTemp117 = fSlow223 * (1.798e+04 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, fSlow219 * fTemp9 + F32::sqrt(F32::sqrt(F32::max(0.0, 5.561735e-05 * (self.fRec56[0] + -2e+01)))) + fSlow220 * fTemp8))) + 2e+01);
																														}
																															let mut fTemp118: F32 = F32::max(2e+01, F32::min(1.8e+04, if iSlow221 != 0 {fSlow225 * fTemp117} else {self.fRec56[0]} * F32::exp(0.6931472 * fTemp116 * fTemp18) * (fSlow210 * self.fRec27[0] + 1.0) * F32::exp(2.7725887 * (fSlow209 * fTemp6 + fSlow208 * fTemp4)) * F32::powf(0.0038221916 * fTemp80, if iSlow202 != 0 {fSlow207 * fTemp113} else {fSlow203}) * F32::exp(0.6931472 * self.fRec50[0] * fSlow199 * F32::max(-5.0, F32::min(5.0, if iSlow193 != 0 {fSlow198 * fTemp112} else {self.fRec49[0]} + 5.0 * (fSlow190 * fTemp6 + fSlow189 * fTemp4))))));
																															let mut fTemp119: F32 = F32::tan(self.fConst15 * F32::max(2e+01, F32::min(self.fConst29, fTemp118)));
																															let mut fTemp120: F32 = fTemp119 + 1.0 / F32::max(0.5, fTemp111);
																															let mut fTemp121: F32 = fTemp119 * fTemp120 + 1.0;
																															let mut fTemp122: F32 = F32::tanh(1.7 * self.fRec43[1]);
																															self.fRec57[0] = fSlow84 * fTemp3 + self.fConst1 * fTemp2 * self.fRec57[1];
																															if iSlow231 != 0 {
																																fTemp123 = fSlow230 * F32::max(0.0, F32::min(1.0, fSlow227 * fTemp9 + F32::max(0.0, self.fRec57[0]) + fSlow228 * fTemp8));
																															}
																																let mut fTemp124: F32 = F32::max(0.0, F32::min(1.0, fSlow233 * fTemp4 + if iSlow229 != 0 {fSlow232 * fTemp123} else {self.fRec57[0]} + fSlow226 * fTemp6));
																																self.fRec58[0] = fSlow240 * fTemp3 + self.fConst1 * fTemp2 * self.fRec58[1];
																																if iSlow242 != 0 {
																																	fTemp125 = fSlow241 * F32::max(0.0, F32::min(1.0, fSlow237 * fTemp9 + F32::max(0.0, self.fRec58[0]) + fSlow238 * fTemp8));
																																}
																																	let mut fTemp126: F32 = F32::max(0.0, F32::min(1.0, fSlow244 * fTemp4 + if iSlow239 != 0 {fSlow243 * fTemp125} else {self.fRec58[0]} + fSlow236 * fTemp6));
																																	self.fRec59[0] = fSlow251 * fTemp3 + self.fConst1 * fTemp2 * self.fRec59[1];
																																	if iSlow253 != 0 {
																																		fTemp127 = fSlow252 * F32::max(0.0, F32::min(1.0, fSlow248 * fTemp9 + F32::max(0.0, self.fRec59[0]) + fSlow249 * fTemp8));
																																	}
																																		let mut fTemp128: F32 = F32::max(0.0, F32::min(1.0, fSlow255 * fTemp4 + if iSlow250 != 0 {fSlow254 * fTemp127} else {self.fRec59[0]} + fSlow247 * fTemp6));
																																		self.fRec60[0] = fSlow262 * fTemp3 + self.fConst1 * fTemp2 * self.fRec60[1];
																																		if iSlow264 != 0 {
																																			fTemp129 = fSlow263 * F32::max(0.0, F32::min(1.0, fSlow259 * fTemp9 + F32::max(0.0, self.fRec60[0]) + fSlow260 * fTemp8));
																																		}
																																			let mut fTemp130: F32 = F32::max(0.0, F32::min(1.0, fSlow266 * fTemp4 + if iSlow261 != 0 {fSlow265 * fTemp129} else {self.fRec60[0]} + fSlow258 * fTemp6));
																																			let mut fTemp131: F32 = F32::max(1e-09, F32::sqrt(fSlow269 * VoiceDsp_faustpower2_f(fTemp130) + fSlow257 * VoiceDsp_faustpower2_f(fTemp128) + fSlow246 * VoiceDsp_faustpower2_f(fTemp126) + fSlow235 * VoiceDsp_faustpower2_f(fTemp124)));
																																			if (fTemp40 != 0.0) as i32 != 0 {
																																				self.fRec61[0] = fSlow273 * fTemp3 + self.fConst1 * fTemp2 * self.fRec61[1];
																																			}
																																				if ((fTemp40 != 0.0) as i32) & iSlow275 != 0 {
																																					fTemp132 = fSlow274 * F32::max(0.0, F32::min(1.0, fSlow270 * fTemp9 + F32::max(0.0, self.fRec61[0]) + fSlow271 * fTemp8));
																																				}
																																					if ((fTemp40 != 0.0) as i32) & iSlow282 != 0 {
																																						fTemp133 = fSlow281 * (7.9 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, fSlow283 + fSlow278 * fTemp8 + fSlow277 * fTemp9))) + 0.1);
																																					}
																																						if (fTemp40 != 0.0) as i32 != 0 {
																																							fTemp134 = if iSlow279 != 0 {fSlow284 * fTemp133} else {fSlow280};
																																							fTemp135 = F32::exp(-(self.fConst30 * fTemp134));
																																							fTemp136 = self.fConst7 * fTemp134 + self.fRec64[1];
																																							self.fRec64[0] = fTemp136 - F32::floor(fTemp136);
																																							self.iRec65[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec65[1], i32::wrapping_mul(iSlow285, iTemp15))), 12345);
																																							iTemp137 = i32::wrapping_mul(1664525, self.iRec65[0] ^ (self.iRec65[0] >> 15));
																																							self.fRec63[0] = if ((self.fRec64[0] < self.fRec64[1]) as i32) | ((iTemp15 > 0) as i32) != 0 {4.656613e-10 * (i32::wrapping_mul(1013904223, iTemp137 ^ (iTemp137 >> 13))) as F32} else {self.fRec63[1]};
																																							self.fRec62[0] = self.fRec63[0] * (1.0 - fTemp135) + fTemp135 * self.fRec62[1];
																																							fTemp138 = 3.0 * self.fRec62[0] * if iSlow272 != 0 {fSlow276 * fTemp132} else {self.fRec61[0]};
																																							fTemp139 = F32::floor(fTemp138);
																																							iTemp140 = (fTemp139) as i32;
																																							fTemp141 = fTemp138 - fTemp139;
																																							self.fRec66[0] = fSlow291 * fTemp3 + self.fConst1 * fTemp2 * self.fRec66[1];
																																						}
																																							if ((fTemp40 != 0.0) as i32) & iSlow293 != 0 {
																																								fTemp142 = fSlow292 * (48.0 * F32::max(0.0, F32::min(1.0, fSlow288 * fTemp9 + F32::max(0.0, 0.020833334 * (self.fRec66[0] + 24.0)) + fSlow289 * fTemp8)) + -24.0);
																																							}
																																								if (fTemp40 != 0.0) as i32 != 0 {
																																									fTemp143 = 0.025 * F32::max(-24.0, F32::min(24.0, if iSlow290 != 0 {fSlow294 * fTemp142} else {self.fRec66[0]} + 24.0 * (fSlow287 * fTemp6 + fSlow286 * fTemp4)));
																																								}
																																									if ((fTemp40 != 0.0) as i32) & (iSlow82 & iSlow300) != 0 {
																																										fTemp144 = fSlow299 * F32::max(0.0, F32::min(1.0, fSlow301 + fSlow296 * fTemp8 + fSlow295 * fTemp9));
																																									}
																																										if ((fTemp40 != 0.0) as i32) & iSlow82 != 0 {
																																											fTemp145 = fTemp73 * ((F32::abs(fTemp74) > (1.0 - 0.002 * F32::max(0.0, F32::min(1.0, if iSlow297 != 0 {fSlow302 * fTemp144} else {fSlow298})))) as i32) as u32 as F32;
																																											self.fVec6[0] = fTemp145;
																																											self.fRec70[0] = self.fConst33 * (1.8626451e-09 * (fTemp145 + self.fVec6[1]) - self.fConst32 * self.fRec70[1]);
																																											self.fRec69[0] = -(self.fConst36 * (self.fConst35 * self.fRec69[1] - self.fConst34 * (self.fRec70[0] - self.fRec70[1])));
																																											fTemp146 = fSlow81 * self.fRec69[0];
																																										}
																																											if (((fTemp40 != 0.0) as i32) & iSlow88) | ((((fTemp40 != 0.0) as i32) & iSlow309) | (((fTemp40 != 0.0) as i32) & iSlow306)) != 0 {
																																												fTemp147 = if iTemp15 != 0 {0.0} else {self.fRec72[1] + self.fConst7 * F32::max(fSlow310 * fTemp80, 0.0)};
																																												self.fRec72[0] = fTemp147 - F32::floor(fTemp147);
																																												iTemp148 = (self.fRec72[0] < self.fRec72[1]) as i32;
																																											}
																																												if ((fTemp40 != 0.0) as i32) & iSlow306 != 0 {
																																													iTemp149 = if (self.iRec71[1] == 0) as i32 != 0 {1} else {self.iRec71[1]};
																																													self.iRec71[0] = if iTemp148 != 0 {(iTemp149 >> 1) | (((iTemp149 ^ (iTemp149 >> 6)) & 1) << 14)} else {iTemp149};
																																													fTemp150 = fSlow305 * if self.iRec71[0] & 1 != 0 {1.0} else {-1.0};
																																												}
																																													if ((fTemp40 != 0.0) as i32) & iSlow309 != 0 {
																																														iTemp151 = if (self.iRec73[1] == 0) as i32 != 0 {1} else {self.iRec73[1]};
																																														iTemp152 = iTemp151 >> 1;
																																														self.iRec73[0] = if iTemp148 != 0 {iTemp152 | (((iTemp151 ^ iTemp152) & 1) << 14)} else {iTemp151};
																																														fTemp153 = fSlow308 * if self.iRec73[0] & 1 != 0 {1.0} else {-1.0};
																																													}
																																														if ((fTemp40 != 0.0) as i32) & iSlow88 != 0 {
																																															self.fRec74[0] = if iTemp148 != 0 {fTemp74} else {self.fRec74[1]};
																																															fTemp154 = fSlow314 * F32::floor(fSlow313 * self.fRec74[0] + 0.5);
																																														}
																																															if ((fTemp40 != 0.0) as i32) & iSlow91 != 0 {
																																																fTemp155 = fSlow316 * fTemp73;
																																															}
																																																if (fTemp40 != 0.0) as i32 != 0 {
																																																	fTemp156 = fSlow317 * fTemp155 + fSlow315 * fTemp154 + fSlow312 * fTemp153 + fSlow311 * fTemp150 + fSlow303 * fTemp146;
																																																	self.fVec7[0] = fTemp156;
																																																	self.fRec68[0] = -(self.fConst39 * (self.fConst38 * self.fRec68[1] - (fTemp156 + self.fVec7[1])));
																																																	fTemp157 = fTemp156 + self.fRec68[0] * (F32::powf(1e+01, -fTemp143) + -1.0);
																																																	self.fVec8[0] = fTemp157;
																																																	self.fRec67[0] = -(self.fConst39 * (self.fConst38 * self.fRec67[1] - self.fConst37 * (fTemp157 - self.fVec8[1])));
																																																	fTemp158 = fTemp40 * (fTemp157 + self.fRec67[0] * (F32::powf(1e+01, fTemp143) + -1.0)) * (fTemp141 * (fTemp141 * (fTemp141 * (fTemp141 * (0.0013333 * fTemp141 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp140))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp140))) as F32;
																																																}
																																																	let mut fTemp159: F32 = F32::powf(1e+01, 1.2 * (0.375 * fTemp43 + 0.625 * VoiceDsp_faustpower4_f(fTemp43)));
																																																	if (((fTemp72 != 0.0) as i32) & iTemp70) | ((((fTemp72 != 0.0) as i32) & iTemp67) | ((((fTemp72 != 0.0) as i32) & iTemp64) | ((((fTemp72 != 0.0) as i32) & iTemp61) | ((((fTemp72 != 0.0) as i32) & iTemp58) | ((((fTemp72 != 0.0) as i32) & iTemp55) | ((((fTemp72 != 0.0) as i32) & iTemp52) | (((fTemp72 != 0.0) as i32) & iTemp49))))))) != 0 {
																																																		fTemp160 = if iTemp15 != 0 {0.0} else {self.fConst42 + self.fRec78[1]};
																																																		self.fRec78[0] = fTemp160 - F32::floor(fTemp160);
																																																		self.fRec77[0] = if (self.fRec78[0] < self.fRec78[1]) as i32 != 0 {fTemp74} else {self.fRec77[1]};
																																																		fTemp161 = F32::floor(32768.0 * self.fRec77[0] + 0.5);
																																																		self.fVec9[0] = fTemp161;
																																																		self.fRec76[0] = self.fConst43 * (3.0517578e-05 * (fTemp161 + self.fVec9[1]) - self.fConst41 * self.fRec76[1]);
																																																		fTemp162 = 0.3 * self.fRec26[0] * self.fRec76[0] * fTemp72;
																																																	}
																																																		if iTemp70 | (iTemp67 | (iTemp64 | (iTemp61 | (iTemp58 | (iTemp55 | (iTemp52 | iTemp49)))))) != 0 {
																																																			fTemp163 = ((iTemp71 != 0) as i32) as u32 as F32 * fTemp162;
																																																		}
																																																			if iTemp61 | iTemp49 != 0 {
																																																				fTemp164 = F32::max(-1.0, F32::min(1.0, fTemp163));
																																																			}
																																																				if iTemp49 != 0 {
																																																					fTemp165 = fTemp164 * fTemp48;
																																																				}
																																																					if (iSlow319 & (iSlow330 & iTemp70)) | ((iSlow319 & (iSlow333 & iTemp70)) | ((iSlow319 & (iSlow323 & iTemp70)) | ((iSlow319 & (iSlow327 & iTemp70)) | ((iSlow319 & (iSlow333 & iTemp67)) | ((iSlow319 & (iSlow330 & iTemp67)) | ((iSlow319 & (iSlow327 & iTemp67)) | ((iSlow319 & (iSlow323 & iTemp67)) | ((iSlow319 & (iSlow333 & iTemp64)) | ((iSlow319 & (iSlow330 & iTemp64)) | ((iSlow319 & (iSlow327 & iTemp64)) | ((iSlow319 & (iSlow323 & iTemp64)) | ((iSlow319 & (iSlow333 & iTemp61)) | ((iSlow319 & (iSlow330 & iTemp61)) | ((iSlow319 & (iSlow327 & iTemp61)) | ((iSlow319 & (iSlow323 & iTemp61)) | ((iSlow319 & (iSlow333 & iTemp58)) | ((iSlow319 & (iSlow330 & iTemp58)) | ((iSlow319 & (iSlow327 & iTemp58)) | ((iSlow319 & (iSlow323 & iTemp58)) | ((iSlow319 & (iSlow333 & iTemp55)) | ((iSlow319 & (iSlow330 & iTemp55)) | ((iSlow319 & (iSlow327 & iTemp55)) | ((iSlow319 & (iSlow323 & iTemp55)) | ((iSlow319 & (iSlow333 & iTemp52)) | ((iSlow319 & (iSlow330 & iTemp52)) | ((iSlow319 & (iSlow327 & iTemp52)) | ((iSlow319 & (iSlow323 & iTemp52)) | ((iSlow319 & (iSlow333 & iTemp49)) | ((iSlow319 & (iSlow330 & iTemp49)) | ((iSlow319 & (iSlow327 & iTemp49)) | (iSlow319 & (iSlow323 & iTemp49)))))))))))))))))))))))))))))))) != 0 {
																																																						fTemp166 = fSlow324 * fTemp80;
																																																					}
																																																						if (iSlow319 & (iSlow323 & iTemp70)) | ((iSlow319 & (iSlow327 & iTemp70)) | ((iSlow319 & (iSlow327 & iTemp67)) | ((iSlow319 & (iSlow323 & iTemp67)) | ((iSlow319 & (iSlow327 & iTemp64)) | ((iSlow319 & (iSlow323 & iTemp64)) | ((iSlow319 & (iSlow327 & iTemp61)) | ((iSlow319 & (iSlow323 & iTemp61)) | ((iSlow319 & (iSlow327 & iTemp58)) | ((iSlow319 & (iSlow323 & iTemp58)) | ((iSlow319 & (iSlow327 & iTemp55)) | ((iSlow319 & (iSlow323 & iTemp55)) | ((iSlow319 & (iSlow327 & iTemp52)) | ((iSlow319 & (iSlow323 & iTemp52)) | ((iSlow319 & (iSlow327 & iTemp49)) | (iSlow319 & (iSlow323 & iTemp49)))))))))))))))) != 0 {
																																																							fTemp167 = F32::max(fTemp166, 1e-06);
																																																						}
																																																							if (iSlow319 & (iSlow330 & iTemp70)) | ((iSlow319 & (iSlow333 & iTemp70)) | ((iSlow319 & (iSlow323 & iTemp70)) | ((iSlow319 & (iSlow327 & iTemp70)) | ((iSlow319 & (iSlow333 & iTemp67)) | ((iSlow319 & (iSlow330 & iTemp67)) | ((iSlow319 & (iSlow327 & iTemp67)) | ((iSlow319 & (iSlow323 & iTemp67)) | ((iSlow319 & (iSlow333 & iTemp64)) | ((iSlow319 & (iSlow330 & iTemp64)) | ((iSlow319 & (iSlow327 & iTemp64)) | ((iSlow319 & (iSlow323 & iTemp64)) | ((iSlow319 & (iSlow333 & iTemp61)) | ((iSlow319 & (iSlow330 & iTemp61)) | ((iSlow319 & (iSlow327 & iTemp61)) | ((iSlow319 & (iSlow323 & iTemp61)) | ((iSlow319 & (iSlow333 & iTemp58)) | ((iSlow319 & (iSlow330 & iTemp58)) | ((iSlow319 & (iSlow327 & iTemp58)) | ((iSlow319 & (iSlow323 & iTemp58)) | ((iSlow319 & (iSlow333 & iTemp55)) | ((iSlow319 & (iSlow330 & iTemp55)) | ((iSlow319 & (iSlow327 & iTemp55)) | ((iSlow319 & (iSlow323 & iTemp55)) | ((iSlow319 & (iSlow333 & iTemp52)) | ((iSlow319 & (iSlow330 & iTemp52)) | ((iSlow319 & (iSlow327 & iTemp52)) | ((iSlow319 & (iSlow323 & iTemp52)) | ((iSlow319 & (iSlow333 & iTemp49)) | ((iSlow319 & (iSlow330 & iTemp49)) | ((iSlow319 & (iSlow327 & iTemp49)) | (iSlow319 & (iSlow323 & iTemp49)))))))))))))))))))))))))))))))) != 0 {
																																																								fTemp168 = self.fRec79[1] + fTemp166;
																																																								self.fRec79[0] = if iTemp0 != 0 {0.0} else {fTemp168 - F32::floor(fTemp168)};
																																																							}
																																																								if (iSlow319 & (iSlow323 & iTemp70)) | ((iSlow319 & (iSlow327 & iTemp70)) | ((iSlow319 & (iSlow327 & iTemp67)) | ((iSlow319 & (iSlow323 & iTemp67)) | ((iSlow319 & (iSlow327 & iTemp64)) | ((iSlow319 & (iSlow323 & iTemp64)) | ((iSlow319 & (iSlow327 & iTemp61)) | ((iSlow319 & (iSlow323 & iTemp61)) | ((iSlow319 & (iSlow327 & iTemp58)) | ((iSlow319 & (iSlow323 & iTemp58)) | ((iSlow319 & (iSlow327 & iTemp55)) | ((iSlow319 & (iSlow323 & iTemp55)) | ((iSlow319 & (iSlow327 & iTemp52)) | ((iSlow319 & (iSlow323 & iTemp52)) | ((iSlow319 & (iSlow327 & iTemp49)) | (iSlow319 & (iSlow323 & iTemp49)))))))))))))))) != 0 {
																																																									fTemp169 = self.fRec79[0] + -1.0;
																																																									fTemp170 = 1.0 - fTemp167;
																																																									fTemp171 = -1.0 - (((self.fRec79[0] < fTemp167) as i32) as u32 as F32 * (self.fRec79[0] * (2.0 - self.fRec79[0] / fTemp167) / fTemp167 + -1.0) + ((self.fRec79[0] > fTemp170) as i32) as u32 as F32 * (fTemp169 * (fTemp169 / fTemp167 + 2.0) / fTemp167 + 1.0));
																																																								}
																																																									let mut fTemp172: F32 = fSlow341 * (fSlow340 * fTemp11 + 1.0);
																																																									if iSlow343 != 0 {
																																																										fTemp173 = fSlow342 * (19.99 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, fSlow336 * fTemp9 + F32::sqrt(F32::sqrt(F32::max(0.0, 0.050025012 * (fTemp172 + -0.01)))) + fSlow337 * fTemp8))) + 0.01);
																																																									}
																																																										let mut fTemp174: F32 = if iTemp15 != 0 {0.0} else {self.fRec80[1] + self.fConst7 * if iSlow338 != 0 {fSlow344 * fTemp173} else {fTemp172}};
																																																										self.fRec80[0] = fTemp174 - F32::floor(fTemp174);
																																																										self.fRec81[0] = fSlow349 * fTemp3 + self.fConst1 * fTemp2 * self.fRec81[1];
																																																										if iSlow351 != 0 {
																																																											fTemp175 = fSlow350 * F32::max(0.0, F32::min(1.0, fSlow346 * fTemp9 + F32::max(0.0, self.fRec81[0]) + fSlow347 * fTemp8));
																																																										}
																																																											self.fRec82[0] = fSlow359 * fTemp3 + self.fConst1 * fTemp2 * self.fRec82[1];
																																																											if iSlow361 != 0 {
																																																												fTemp176 = fSlow360 * (0.9 * F32::max(0.0, F32::min(1.0, fSlow356 * fTemp9 + F32::max(0.0, 1.1111112 * (self.fRec82[0] + -0.05)) + fSlow357 * fTemp8)) + 0.05);
																																																											}
																																																												let mut fTemp177: F32 = F32::max(0.05, F32::min(0.95, if iSlow358 != 0 {fSlow362 * fTemp176} else {self.fRec82[0]} + 0.45 * (fSlow355 * fTemp6 + fSlow354 * fTemp4 + F32::max(0.0, F32::min(1.0, fSlow353 * fTemp4 + if iSlow348 != 0 {fSlow352 * fTemp175} else {self.fRec81[0]} + fSlow345 * fTemp6)) * (1.0 - 4.0 * F32::abs(self.fRec80[0] + -0.5))) + fSlow335 * fTemp11));
																																																												if (iSlow319 & (iSlow323 & iTemp70)) | ((iSlow319 & (iSlow323 & iTemp67)) | ((iSlow319 & (iSlow323 & iTemp64)) | ((iSlow319 & (iSlow323 & iTemp61)) | ((iSlow319 & (iSlow323 & iTemp58)) | ((iSlow319 & (iSlow323 & iTemp55)) | ((iSlow319 & (iSlow323 & iTemp52)) | (iSlow319 & (iSlow323 & iTemp49)))))))) != 0 {
																																																													fTemp178 = F32::max(0.02, F32::min(0.98, fTemp177));
																																																													fTemp179 = fTemp178 + F32::floor(self.fRec79[0] + (1.0 - fTemp178));
																																																													fTemp180 = self.fRec79[0] + (1.0 - fTemp179);
																																																													fTemp181 = self.fRec79[0] - fTemp179;
																																																													fTemp182 = fSlow322 * (((fTemp180 > fTemp170) as i32) as u32 as F32 * (fTemp181 * (fTemp181 / fTemp167 + 2.0) / fTemp167 + 1.0) + ((fTemp180 < fTemp167) as i32) as u32 as F32 * (fTemp180 * (2.0 - fTemp180 / fTemp167) / fTemp167 + -1.0) + (i32::wrapping_add(i32::wrapping_mul(2, (self.fRec79[0] >= fTemp178) as i32), -1)) as F32 + 2.0 * fTemp178 + fTemp171);
																																																												}
																																																													if (iSlow319 & (iSlow327 & iTemp70)) | ((iSlow319 & (iSlow327 & iTemp67)) | ((iSlow319 & (iSlow327 & iTemp64)) | ((iSlow319 & (iSlow327 & iTemp61)) | ((iSlow319 & (iSlow327 & iTemp58)) | ((iSlow319 & (iSlow327 & iTemp55)) | ((iSlow319 & (iSlow327 & iTemp52)) | (iSlow319 & (iSlow327 & iTemp49)))))))) != 0 {
																																																														fTemp183 = fSlow326 * (2.0 * self.fRec79[0] + fTemp171);
																																																													}
																																																														if (iSlow319 & (iSlow330 & iTemp70)) | ((iSlow319 & (iSlow330 & iTemp67)) | ((iSlow319 & (iSlow330 & iTemp64)) | ((iSlow319 & (iSlow330 & iTemp61)) | ((iSlow319 & (iSlow330 & iTemp58)) | ((iSlow319 & (iSlow330 & iTemp55)) | ((iSlow319 & (iSlow330 & iTemp52)) | (iSlow319 & (iSlow330 & iTemp49)))))))) != 0 {
																																																															fTemp184 = fSlow329 * (1.0 - 4.0 * F32::abs(self.fRec79[0] + (-0.25 - F32::floor(self.fRec79[0] + 0.25))));
																																																														}
																																																															if (iSlow319 & (iSlow333 & iTemp70)) | ((iSlow319 & (iSlow333 & iTemp67)) | ((iSlow319 & (iSlow333 & iTemp64)) | ((iSlow319 & (iSlow333 & iTemp61)) | ((iSlow319 & (iSlow333 & iTemp58)) | ((iSlow319 & (iSlow333 & iTemp55)) | ((iSlow319 & (iSlow333 & iTemp52)) | (iSlow319 & (iSlow333 & iTemp49)))))))) != 0 {
																																																																fTemp185 = fSlow332 * F32::sin(6.2831855 * self.fRec79[0]);
																																																															}
																																																																if (iSlow319 & iTemp70) | ((iSlow319 & iTemp67) | ((iSlow319 & iTemp64) | ((iSlow319 & iTemp61) | ((iSlow319 & iTemp58) | ((iSlow319 & iTemp55) | ((iSlow319 & iTemp52) | (iSlow319 & iTemp49))))))) != 0 {
																																																																	fTemp186 = fSlow268 * (fSlow366 * fTemp185 + fSlow365 * fTemp184 + fSlow364 * fTemp183 + fSlow363 * fTemp182);
																																																																}
																																																																	if iTemp70 | (iTemp67 | (iTemp64 | (iTemp61 | (iTemp58 | (iTemp55 | (iTemp52 | iTemp49)))))) != 0 {
																																																																		fTemp187 = ((fSlow367 * fTemp186 * fTemp130 + fSlow256 * *input2 * fTemp128 + fSlow245 * *input3 * fTemp126) / fTemp131 + fSlow318 * fTemp94) * fTemp159 + fTemp163;
																																																																	}
																																																																		if iTemp61 | iTemp49 != 0 {
																																																																			fTemp188 = F32::max(-1.0, F32::min(1.0, fTemp187));
																																																																		}
																																																																			if iTemp49 != 0 {
																																																																				fTemp189 = fTemp188 * fTemp48;
																																																																			}
																																																																				if iTemp64 | iTemp52 != 0 {
																																																																					fTemp190 = F32::atan(1.5707964 * fTemp163);
																																																																				}
																																																																					if iTemp52 != 0 {
																																																																						fTemp191 = 0.63661975 * fTemp190 * fTemp51;
																																																																					}
																																																																						if iTemp64 | iTemp52 != 0 {
																																																																							fTemp192 = F32::atan(1.5707964 * fTemp187);
																																																																						}
																																																																							if iTemp52 != 0 {
																																																																								fTemp193 = 0.63661975 * fTemp192 * fTemp51;
																																																																							}
																																																																								if iTemp67 | iTemp55 != 0 {
																																																																									fTemp194 = if (fTemp163 < 0.0) as i32 != 0 {1.6 * F32::tanh(0.625 * fTemp163)} else {F32::tanh(fTemp163)};
																																																																								}
																																																																									if iTemp55 != 0 {
																																																																										fTemp195 = fTemp194 * fTemp54;
																																																																									}
																																																																										if iTemp67 | iTemp55 != 0 {
																																																																											fTemp196 = if (fTemp187 < 0.0) as i32 != 0 {1.6 * F32::tanh(0.625 * fTemp187)} else {F32::tanh(fTemp187)};
																																																																										}
																																																																											if iTemp55 != 0 {
																																																																												fTemp197 = fTemp196 * fTemp54;
																																																																											}
																																																																												if iTemp70 | iTemp58 != 0 {
																																																																													fTemp198 = F32::sin(fTemp163);
																																																																												}
																																																																													if iTemp58 != 0 {
																																																																														fTemp199 = fTemp198 * fTemp57;
																																																																													}
																																																																														if iTemp70 | iTemp58 != 0 {
																																																																															fTemp200 = F32::sin(fTemp187);
																																																																														}
																																																																															if iTemp58 != 0 {
																																																																																fTemp201 = fTemp200 * fTemp57;
																																																																															}
																																																																																let mut fTemp202: F32 = F32::max(0.0, F32::min(1.0, 2.5 * (fTemp44 + (-0.8 - fTemp45))));
																																																																																if iTemp61 != 0 {
																																																																																	fTemp203 = fTemp60 * fTemp164;
																																																																																	fTemp204 = fTemp188 * fTemp60;
																																																																																}
																																																																																	if iTemp64 != 0 {
																																																																																		fTemp205 = 0.63661975 * fTemp63 * fTemp190;
																																																																																		fTemp206 = 0.63661975 * fTemp192 * fTemp63;
																																																																																	}
																																																																																		if iTemp67 != 0 {
																																																																																			fTemp207 = fTemp66 * fTemp194;
																																																																																			fTemp208 = fTemp196 * fTemp66;
																																																																																		}
																																																																																			if iTemp70 != 0 {
																																																																																				fTemp209 = fTemp69 * fTemp198;
																																																																																				fTemp210 = fTemp200 * fTemp69;
																																																																																			}
																																																																																				let mut fTemp211: F32 = ((1.0 - fTemp202) * (((iTemp68 != 0) as i32) as u32 as F32 * (fTemp210 - fTemp209) + ((iTemp65 != 0) as i32) as u32 as F32 * (fTemp208 - fTemp207) + ((iTemp62 != 0) as i32) as u32 as F32 * (fTemp206 - fTemp205) + ((iTemp59 != 0) as i32) as u32 as F32 * (fTemp204 - fTemp203)) + fTemp202 * (((iTemp56 != 0) as i32) as u32 as F32 * (fTemp201 - fTemp199) + ((iTemp53 != 0) as i32) as u32 as F32 * (fTemp197 - fTemp195) + ((iTemp50 != 0) as i32) as u32 as F32 * (fTemp193 - fTemp191) + ((iTemp47 != 0) as i32) as u32 as F32 * (fTemp189 - fTemp165))) / F32::tanh(0.35 * fTemp159);
																																																																																				self.fVec10[0] = fTemp211;
																																																																																				self.fRec75[0] = 0.995 * self.fRec75[1] + 0.35 * (fTemp211 - self.fVec10[1]);
																																																																																				let mut fTemp212: F32 = self.fRec75[0] * F32::powf(1e+01, 0.125 * fTemp42) + fSlow234 * (fTemp38 * ((iTemp39 != 0) as i32) as u32 as F32 * fTemp124 * fTemp158 / fTemp131) - (0.5882353 * fTemp120 * fTemp122 + self.fRec44[1]);
																																																																																				let mut fTemp213: F32 = fTemp119 * fTemp212 / fTemp121;
																																																																																				let mut fTemp214: F32 = 0.5882353 * fTemp122;
																																																																																				self.fRec43[0] = fTemp214 + 2.0 * fTemp213;
																																																																																				let mut fTemp215: F32 = fTemp214 + fTemp213;
																																																																																				let mut fTemp216: F32 = fTemp119 * fTemp215;
																																																																																				self.fRec44[0] = self.fRec44[1] + 2.0 * fTemp216;
																																																																																				let mut fRec45: F32 = self.fRec44[1] + fTemp216;
																																																																																				let mut fRec46: F32 = fTemp215;
																																																																																				let mut fRec47: F32 = fTemp212 / fTemp121;
																																																																																				self.fRec83[0] = fSlow372 * fTemp3 + self.fConst1 * fTemp2 * self.fRec83[1];
																																																																																				if iSlow374 != 0 {
																																																																																					fTemp217 = fSlow373 * F32::max(0.0, F32::min(1.0, fSlow369 * fTemp9 + F32::max(0.0, self.fRec83[0]) + fSlow370 * fTemp8));
																																																																																				}
																																																																																					let mut fTemp218: F32 = F32::max(0.0, F32::min(1.0, fSlow376 * fTemp4 + if iSlow371 != 0 {fSlow375 * fTemp217} else {self.fRec83[0]} + fSlow368 * fTemp6));
																																																																																					let mut fTemp219: F32 = F32::max(0.0, F32::min(1.0, fTemp218));
																																																																																					let mut iTemp220: i32 = (fTemp219 > 0.5) as i32;
																																																																																					let mut fTemp221: F32 = fRec45 + fRec47;
																																																																																					let mut fTemp222: F32 = 2.0 * fTemp219;
																																																																																					let mut fTemp223: F32 = 1.0 - fTemp222;
																																																																																					let mut fTemp224: F32 = fTemp222 + -1.0;
																																																																																					let mut fTemp225: F32 = 1.0 - fTemp219;
																																																																																					let mut fTemp226: F32 = 1.0 - fTemp109;
																																																																																					let mut fTemp227: F32 = fTemp226 * if iTemp220 != 0 {2.0 * fTemp221 * fTemp225 + fRec47 * fTemp224} else {fRec45 * fTemp223 + 2.0 * fTemp221 * fTemp219} + fRec46 * fTemp109;
																																																																																					if iSlow377 != 0 {
																																																																																						fTemp228 = F32::tanh(1.7 * self.fRec84[1]);
																																																																																						fTemp229 = fTemp227 - (0.5882353 * fTemp120 * fTemp228 + self.fRec85[1]);
																																																																																						fTemp230 = fTemp119 * fTemp229 / fTemp121;
																																																																																						fTemp231 = 0.5882353 * fTemp228;
																																																																																						self.fRec84[0] = fTemp231 + 2.0 * fTemp230;
																																																																																						fTemp232 = fTemp231 + fTemp230;
																																																																																						fTemp233 = fTemp119 * fTemp232;
																																																																																						self.fRec85[0] = self.fRec85[1] + 2.0 * fTemp233;
																																																																																						fRec86 = self.fRec85[1] + fTemp233;
																																																																																						fRec87 = fTemp232;
																																																																																						fRec88 = fTemp229 / fTemp121;
																																																																																						fTemp234 = fRec86 + fRec88;
																																																																																						fTemp235 = fSlow167 * (fTemp226 * if iTemp220 != 0 {2.0 * fTemp234 * fTemp225 + fRec88 * fTemp224} else {fRec86 * fTemp223 + 2.0 * fTemp234 * fTemp219} + fRec87 * fTemp109);
																																																																																					}
																																																																																						let mut fTemp236: F32 = if iSlow168 != 0 {fSlow378 * fTemp235} else {fTemp227};
																																																																																						self.fVec11[0] = fTemp236;
																																																																																						self.fRec40[0] = -((self.fRec40[1] * (1.0 - fTemp107) - (fTemp236 - self.fVec11[1]) / fTemp106) / (fTemp107 + 1.0));
																																																																																						*output0 = fSlow379 * ((self.fRec40[0] * self.fRec33[0] + fSlow138 * fTemp94) * (1.0 - 0.5 * F32::abs(fSlow29 * fTemp6 + fSlow28 * fTemp4) * (fTemp24 + 1.0)) * F32::powf(1e+01, 0.6 * (fSlow27 * fTemp6 + fSlow26 * fTemp4)) * if iSlow23 != 0 {fSlow25 * (1.9912813 * F32::powf(F32::max(0.0, F32::min(1.0, fSlow24 + fSlow22 * fTemp8 + fSlow21 * fTemp9)), 4.547648) + 0.0039810715)} else {1.0} * (fTemp22 * (fTemp22 * (fTemp22 * (fTemp22 * (0.0013333 * fTemp22 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp21))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp21))) as F32);
																																																																																						*output1 = fTemp177;
																																																																																						*output2 = fTemp118;
																																																																																						*output3 = fTemp111;
																																																																																						*output4 = fTemp218;
																																																																																						*output5 = fTemp109;
																																																																																						*output6 = fTemp105;
																																																																																						*output7 = fTemp8;
																																																																																						*output8 = fTemp9;
																																																																																						*output9 = fTemp80;
																																																																																						*output10 = self.fRec33[0];
																																																																																						self.iVec0[1] = self.iVec0[0];
																																																																																						self.fVec1[1] = self.fVec1[0];
																																																																																						self.fRec0[1] = self.fRec0[0];
																																																																																						self.fRec1[1] = self.fRec1[0];
																																																																																						self.fRec2[1] = self.fRec2[0];
																																																																																						self.fRec3[1] = self.fRec3[0];
																																																																																						self.fRec4[1] = self.fRec4[0];
																																																																																						if iTemp14 != 0 {
																																																																																							self.iRec6[1] = self.iRec6[0];
																																																																																							self.fRec5[1] = self.fRec5[0];
																																																																																							self.fRec7[1] = self.fRec7[0];
																																																																																						}
																																																																																							self.fRec9[1] = self.fRec9[0];
																																																																																							self.fRec10[1] = self.fRec10[0];
																																																																																							if iSlow39 != 0 {
																																																																																								self.iRec11[1] = self.iRec11[0];
																																																																																								self.iRec12[1] = self.iRec12[0];
																																																																																							}
																																																																																								if iTemp14 != 0 {
																																																																																									self.fRec16[1] = self.fRec16[0];
																																																																																									self.fRec17[1] = self.fRec17[0];
																																																																																									self.iRec19[1] = self.iRec19[0];
																																																																																									self.fRec18[1] = self.fRec18[0];
																																																																																									self.fRec22[1] = self.fRec22[0];
																																																																																								}
																																																																																									self.fRec24[1] = self.fRec24[0];
																																																																																									self.fRec25[1] = self.fRec25[0];
																																																																																									if iTemp70 | (iTemp67 | (iTemp64 | (iTemp61 | (iTemp58 | (iTemp55 | (iTemp52 | iTemp49)))))) != 0 {
																																																																																										self.fRec26[1] = self.fRec26[0];
																																																																																									}
																																																																																										if iTemp14 | ((((fTemp72 != 0.0) as i32) & iTemp70) | ((iSlow39 & (iSlow65 & (iSlow104 & ((fTemp31 != 0.0) as i32)))) | ((iSlow39 & (iSlow65 & (iSlow69 & ((fTemp31 != 0.0) as i32)))) | ((((fTemp72 != 0.0) as i32) & iTemp67) | ((((fTemp72 != 0.0) as i32) & iTemp64) | ((((fTemp72 != 0.0) as i32) & iTemp61) | ((((fTemp72 != 0.0) as i32) & iTemp58) | ((((fTemp72 != 0.0) as i32) & iTemp55) | ((((fTemp72 != 0.0) as i32) & iTemp52) | ((((fTemp72 != 0.0) as i32) & iTemp49) | ((((fTemp40 != 0.0) as i32) & iSlow91) | ((((fTemp40 != 0.0) as i32) & iSlow88) | (((fTemp40 != 0.0) as i32) & iSlow82))))))))))))) != 0 {
																																																																																											self.iRec23[1] = self.iRec23[0];
																																																																																										}
																																																																																											if iTemp14 != 0 {
																																																																																												self.fRec21[1] = self.fRec21[0];
																																																																																												self.fVec3[1] = self.fVec3[0];
																																																																																												self.fRec20[1] = self.fRec20[0];
																																																																																											}
																																																																																												self.fRec27[1] = self.fRec27[0];
																																																																																												self.fRec28[1] = self.fRec28[0];
																																																																																												if iSlow39 & (iSlow65 & (iSlow69 & ((fTemp31 != 0.0) as i32))) != 0 {
																																																																																													self.fRec15[1] = self.fRec15[0];
																																																																																													self.fRec14[1] = self.fRec14[0];
																																																																																												}
																																																																																													if iSlow39 & (iSlow65 & (iSlow123 & ((fTemp31 != 0.0) as i32))) != 0 {
																																																																																														self.fRec30[1] = self.fRec30[0];
																																																																																														self.fRec29[1] = self.fRec29[0];
																																																																																													}
																																																																																														if iSlow39 & (iSlow65 & (iSlow127 & ((fTemp31 != 0.0) as i32))) != 0 {
																																																																																															self.fRec31[1] = self.fRec31[0];
																																																																																														}
																																																																																															if iSlow39 & (iSlow65 & (iSlow104 & ((fTemp31 != 0.0) as i32))) != 0 {
																																																																																																self.fRec32[2] = self.fRec32[1];
																																																																																																self.fRec32[1] = self.fRec32[0];
																																																																																															}
																																																																																																if iSlow39 & ((fTemp31 != 0.0) as i32) != 0 {
																																																																																																	self.fVec4[1] = self.fVec4[0];
																																																																																																	self.fRec13[1] = self.fRec13[0];
																																																																																																}
																																																																																																	self.fRec34[1] = self.fRec34[0];
																																																																																																	self.fRec35[1] = self.fRec35[0];
																																																																																																	self.fRec36[1] = self.fRec36[0];
																																																																																																	self.fRec37[1] = self.fRec37[0];
																																																																																																	self.fRec38[1] = self.fRec38[0];
																																																																																																	self.fRec39[1] = self.fRec39[0];
																																																																																																	self.fRec33[1] = self.fRec33[0];
																																																																																																	self.fRec41[1] = self.fRec41[0];
																																																																																																	self.fRec42[1] = self.fRec42[0];
																																																																																																	self.fRec48[1] = self.fRec48[0];
																																																																																																	self.fRec49[1] = self.fRec49[0];
																																																																																																	self.fRec51[1] = self.fRec51[0];
																																																																																																	self.fRec52[1] = self.fRec52[0];
																																																																																																	self.fRec50[1] = self.fRec50[0];
																																																																																																	if iTemp14 != 0 {
																																																																																																		self.fRec55[1] = self.fRec55[0];
																																																																																																		self.fRec54[1] = self.fRec54[0];
																																																																																																		self.fVec5[1] = self.fVec5[0];
																																																																																																		self.fRec53[1] = self.fRec53[0];
																																																																																																	}
																																																																																																		self.fRec56[1] = self.fRec56[0];
																																																																																																		self.fRec57[1] = self.fRec57[0];
																																																																																																		self.fRec58[1] = self.fRec58[0];
																																																																																																		self.fRec59[1] = self.fRec59[0];
																																																																																																		self.fRec60[1] = self.fRec60[0];
																																																																																																		if (fTemp40 != 0.0) as i32 != 0 {
																																																																																																			self.fRec61[1] = self.fRec61[0];
																																																																																																			self.fRec64[1] = self.fRec64[0];
																																																																																																			self.iRec65[1] = self.iRec65[0];
																																																																																																			self.fRec63[1] = self.fRec63[0];
																																																																																																			self.fRec62[1] = self.fRec62[0];
																																																																																																			self.fRec66[1] = self.fRec66[0];
																																																																																																		}
																																																																																																			if ((fTemp40 != 0.0) as i32) & iSlow82 != 0 {
																																																																																																				self.fVec6[1] = self.fVec6[0];
																																																																																																				self.fRec70[1] = self.fRec70[0];
																																																																																																				self.fRec69[1] = self.fRec69[0];
																																																																																																			}
																																																																																																				if (((fTemp40 != 0.0) as i32) & iSlow88) | ((((fTemp40 != 0.0) as i32) & iSlow309) | (((fTemp40 != 0.0) as i32) & iSlow306)) != 0 {
																																																																																																					self.fRec72[1] = self.fRec72[0];
																																																																																																				}
																																																																																																					if ((fTemp40 != 0.0) as i32) & iSlow306 != 0 {
																																																																																																						self.iRec71[1] = self.iRec71[0];
																																																																																																					}
																																																																																																						if ((fTemp40 != 0.0) as i32) & iSlow309 != 0 {
																																																																																																							self.iRec73[1] = self.iRec73[0];
																																																																																																						}
																																																																																																							if ((fTemp40 != 0.0) as i32) & iSlow88 != 0 {
																																																																																																								self.fRec74[1] = self.fRec74[0];
																																																																																																							}
																																																																																																								if (fTemp40 != 0.0) as i32 != 0 {
																																																																																																									self.fVec7[1] = self.fVec7[0];
																																																																																																									self.fRec68[1] = self.fRec68[0];
																																																																																																									self.fVec8[1] = self.fVec8[0];
																																																																																																									self.fRec67[1] = self.fRec67[0];
																																																																																																								}
																																																																																																									if (((fTemp72 != 0.0) as i32) & iTemp70) | ((((fTemp72 != 0.0) as i32) & iTemp67) | ((((fTemp72 != 0.0) as i32) & iTemp64) | ((((fTemp72 != 0.0) as i32) & iTemp61) | ((((fTemp72 != 0.0) as i32) & iTemp58) | ((((fTemp72 != 0.0) as i32) & iTemp55) | ((((fTemp72 != 0.0) as i32) & iTemp52) | (((fTemp72 != 0.0) as i32) & iTemp49))))))) != 0 {
																																																																																																										self.fRec78[1] = self.fRec78[0];
																																																																																																										self.fRec77[1] = self.fRec77[0];
																																																																																																										self.fVec9[1] = self.fVec9[0];
																																																																																																										self.fRec76[1] = self.fRec76[0];
																																																																																																									}
																																																																																																										if (iSlow319 & (iSlow330 & iTemp70)) | ((iSlow319 & (iSlow333 & iTemp70)) | ((iSlow319 & (iSlow323 & iTemp70)) | ((iSlow319 & (iSlow327 & iTemp70)) | ((iSlow319 & (iSlow333 & iTemp67)) | ((iSlow319 & (iSlow330 & iTemp67)) | ((iSlow319 & (iSlow327 & iTemp67)) | ((iSlow319 & (iSlow323 & iTemp67)) | ((iSlow319 & (iSlow333 & iTemp64)) | ((iSlow319 & (iSlow330 & iTemp64)) | ((iSlow319 & (iSlow327 & iTemp64)) | ((iSlow319 & (iSlow323 & iTemp64)) | ((iSlow319 & (iSlow333 & iTemp61)) | ((iSlow319 & (iSlow330 & iTemp61)) | ((iSlow319 & (iSlow327 & iTemp61)) | ((iSlow319 & (iSlow323 & iTemp61)) | ((iSlow319 & (iSlow333 & iTemp58)) | ((iSlow319 & (iSlow330 & iTemp58)) | ((iSlow319 & (iSlow327 & iTemp58)) | ((iSlow319 & (iSlow323 & iTemp58)) | ((iSlow319 & (iSlow333 & iTemp55)) | ((iSlow319 & (iSlow330 & iTemp55)) | ((iSlow319 & (iSlow327 & iTemp55)) | ((iSlow319 & (iSlow323 & iTemp55)) | ((iSlow319 & (iSlow333 & iTemp52)) | ((iSlow319 & (iSlow330 & iTemp52)) | ((iSlow319 & (iSlow327 & iTemp52)) | ((iSlow319 & (iSlow323 & iTemp52)) | ((iSlow319 & (iSlow333 & iTemp49)) | ((iSlow319 & (iSlow330 & iTemp49)) | ((iSlow319 & (iSlow327 & iTemp49)) | (iSlow319 & (iSlow323 & iTemp49)))))))))))))))))))))))))))))))) != 0 {
																																																																																																											self.fRec79[1] = self.fRec79[0];
																																																																																																										}
																																																																																																											self.fRec80[1] = self.fRec80[0];
																																																																																																											self.fRec81[1] = self.fRec81[0];
																																																																																																											self.fRec82[1] = self.fRec82[0];
																																																																																																											self.fVec10[1] = self.fVec10[0];
																																																																																																											self.fRec75[1] = self.fRec75[0];
																																																																																																											self.fRec43[1] = self.fRec43[0];
																																																																																																											self.fRec44[1] = self.fRec44[0];
																																																																																																											self.fRec83[1] = self.fRec83[0];
																																																																																																											if iSlow377 != 0 {
																																																																																																												self.fRec84[1] = self.fRec84[0];
																																																																																																												self.fRec85[1] = self.fRec85[0];
																																																																																																											}
																																																																																																												self.fVec11[1] = self.fVec11[0];
																																																																																																												self.fRec40[1] = self.fRec40[0];
																																																																																																											}
																																																																																																											self.fTempPerm0 = fTemp10;
																																																																																																											self.iTempPerm1 = iTemp16;
																																																																																																											self.fTempPerm2 = fTemp17;
																																																																																																											self.fTempPerm3 = fTemp25;
																																																																																																											self.fTempPerm4 = fTemp26;
																																																																																																											self.fTempPerm5 = fTemp27;
																																																																																																											self.fTempPerm6 = fTemp28;
																																																																																																											self.fTempPerm7 = fTemp29;
																																																																																																											self.iTempPerm8 = iTemp30;
																																																																																																											self.fTempPerm9 = fTemp31;
																																																																																																											self.fTempPerm10 = fTemp32;
																																																																																																											self.fTempPerm11 = fTemp33;
																																																																																																											self.fTempPerm12 = fTemp34;
																																																																																																											self.fTempPerm13 = fTemp35;
																																																																																																											self.iTempPerm14 = iTemp36;
																																																																																																											self.fTempPerm15 = fTemp37;
																																																																																																											self.fTempPerm16 = fTemp41;
																																																																																																											self.iTempPerm17 = iTemp71;
																																																																																																											self.fTempPerm18 = fTemp72;
																																																																																																											self.fTempPerm19 = fTemp73;
																																																																																																											self.fTempPerm20 = fTemp74;
																																																																																																											self.fTempPerm21 = fTemp75;
																																																																																																											self.fTempPerm22 = fTemp76;
																																																																																																											self.fTempPerm23 = fTemp81;
																																																																																																											self.fTempPerm24 = fTemp82;
																																																																																																											self.fTempPerm25 = fTemp83;
																																																																																																											self.fTempPerm26 = fTemp84;
																																																																																																											self.fTempPerm27 = fTemp85;
																																																																																																											self.fTempPerm28 = fTemp86;
																																																																																																											self.fTempPerm29 = fTemp87;
																																																																																																											self.fTempPerm30 = fTemp88;
																																																																																																											self.fTempPerm31 = fTemp89;
																																																																																																											self.fTempPerm32 = fTemp90;
																																																																																																											self.fTempPerm33 = fTemp91;
																																																																																																											self.fTempPerm34 = fTemp92;
																																																																																																											self.fTempPerm35 = fTemp93;
																																																																																																											self.fTempPerm36 = fTemp104;
																																																																																																											self.fTempPerm37 = fTemp108;
																																																																																																											self.fTempPerm38 = fTemp110;
																																																																																																											self.fTempPerm39 = fTemp112;
																																																																																																											self.fTempPerm40 = fTemp113;
																																																																																																											self.fTempPerm41 = fTemp114;
																																																																																																											self.fTempPerm42 = fTemp115;
																																																																																																											self.fTempPerm43 = fTemp116;
																																																																																																											self.fTempPerm44 = fTemp117;
																																																																																																											self.fTempPerm45 = fTemp123;
																																																																																																											self.fTempPerm46 = fTemp125;
																																																																																																											self.fTempPerm47 = fTemp127;
																																																																																																											self.fTempPerm48 = fTemp129;
																																																																																																											self.fTempPerm49 = fTemp132;
																																																																																																											self.fTempPerm50 = fTemp133;
																																																																																																											self.fTempPerm51 = fTemp134;
																																																																																																											self.fTempPerm52 = fTemp135;
																																																																																																											self.fTempPerm53 = fTemp136;
																																																																																																											self.iTempPerm54 = iTemp137;
																																																																																																											self.fTempPerm55 = fTemp138;
																																																																																																											self.fTempPerm56 = fTemp139;
																																																																																																											self.iTempPerm57 = iTemp140;
																																																																																																											self.fTempPerm58 = fTemp141;
																																																																																																											self.fTempPerm59 = fTemp142;
																																																																																																											self.fTempPerm60 = fTemp143;
																																																																																																											self.fTempPerm61 = fTemp144;
																																																																																																											self.fTempPerm62 = fTemp145;
																																																																																																											self.fTempPerm63 = fTemp146;
																																																																																																											self.fTempPerm64 = fTemp147;
																																																																																																											self.iTempPerm65 = iTemp148;
																																																																																																											self.iTempPerm66 = iTemp149;
																																																																																																											self.fTempPerm67 = fTemp150;
																																																																																																											self.iTempPerm68 = iTemp151;
																																																																																																											self.iTempPerm69 = iTemp152;
																																																																																																											self.fTempPerm70 = fTemp153;
																																																																																																											self.fTempPerm71 = fTemp154;
																																																																																																											self.fTempPerm72 = fTemp155;
																																																																																																											self.fTempPerm73 = fTemp156;
																																																																																																											self.fTempPerm74 = fTemp157;
																																																																																																											self.fTempPerm75 = fTemp158;
																																																																																																											self.fTempPerm76 = fTemp160;
																																																																																																											self.fTempPerm77 = fTemp161;
																																																																																																											self.fTempPerm78 = fTemp162;
																																																																																																											self.fTempPerm79 = fTemp163;
																																																																																																											self.fTempPerm80 = fTemp164;
																																																																																																											self.fTempPerm81 = fTemp165;
																																																																																																											self.fTempPerm82 = fTemp166;
																																																																																																											self.fTempPerm83 = fTemp167;
																																																																																																											self.fTempPerm84 = fTemp168;
																																																																																																											self.fTempPerm85 = fTemp169;
																																																																																																											self.fTempPerm86 = fTemp170;
																																																																																																											self.fTempPerm87 = fTemp171;
																																																																																																											self.fTempPerm88 = fTemp173;
																																																																																																											self.fTempPerm89 = fTemp175;
																																																																																																											self.fTempPerm90 = fTemp176;
																																																																																																											self.fTempPerm91 = fTemp178;
																																																																																																											self.fTempPerm92 = fTemp179;
																																																																																																											self.fTempPerm93 = fTemp180;
																																																																																																											self.fTempPerm94 = fTemp181;
																																																																																																											self.fTempPerm95 = fTemp182;
																																																																																																											self.fTempPerm96 = fTemp183;
																																																																																																											self.fTempPerm97 = fTemp184;
																																																																																																											self.fTempPerm98 = fTemp185;
																																																																																																											self.fTempPerm99 = fTemp186;
																																																																																																											self.fTempPerm100 = fTemp187;
																																																																																																											self.fTempPerm101 = fTemp188;
																																																																																																											self.fTempPerm102 = fTemp189;
																																																																																																											self.fTempPerm103 = fTemp190;
																																																																																																											self.fTempPerm104 = fTemp191;
																																																																																																											self.fTempPerm105 = fTemp192;
																																																																																																											self.fTempPerm106 = fTemp193;
																																																																																																											self.fTempPerm107 = fTemp194;
																																																																																																											self.fTempPerm108 = fTemp195;
																																																																																																											self.fTempPerm109 = fTemp196;
																																																																																																											self.fTempPerm110 = fTemp197;
																																																																																																											self.fTempPerm111 = fTemp198;
																																																																																																											self.fTempPerm112 = fTemp199;
																																																																																																											self.fTempPerm113 = fTemp200;
																																																																																																											self.fTempPerm114 = fTemp201;
																																																																																																											self.fTempPerm115 = fTemp203;
																																																																																																											self.fTempPerm116 = fTemp204;
																																																																																																											self.fTempPerm117 = fTemp205;
																																																																																																											self.fTempPerm118 = fTemp206;
																																																																																																											self.fTempPerm119 = fTemp207;
																																																																																																											self.fTempPerm120 = fTemp208;
																																																																																																											self.fTempPerm121 = fTemp209;
																																																																																																											self.fTempPerm122 = fTemp210;
																																																																																																											self.fTempPerm123 = fTemp217;
																																																																																																											self.fTempPerm124 = fTemp228;
																																																																																																											self.fTempPerm125 = fTemp229;
																																																																																																											self.fTempPerm126 = fTemp230;
																																																																																																											self.fTempPerm127 = fTemp231;
																																																																																																											self.fTempPerm128 = fTemp232;
																																																																																																											self.fTempPerm129 = fTemp233;
																																																																																																											self.fTempPerm130 = fTemp234;
																																																																																																											self.fTempPerm131 = fTemp235;
																																																																																																										}

}

