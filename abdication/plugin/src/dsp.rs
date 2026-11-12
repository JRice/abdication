/* ------------------------------------------------------------
name: "plugin-voice"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn VoiceDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */


pub struct VoiceDspSIG0 {
	iVec8: [i32;2],
	iRec18: [i32;2],
}

impl VoiceDspSIG0 {
	
	fn get_num_inputsVoiceDspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsVoiceDspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initVoiceDspSIG0(&mut self, sample_rate: i32) {
		for l23 in 0..2 {
			self.iVec8[l23 as usize] = 0;
		}
		for l24 in 0..2 {
			self.iRec18[l24 as usize] = 0;
		}
	}
	
	fn fillVoiceDspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec8[0] = 1;
			self.iRec18[0] = (i32::wrapping_add(self.iVec8[1], self.iRec18[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec18[0]) as F32);
			self.iVec8[1] = self.iVec8[0];
			self.iRec18[1] = self.iRec18[0];
		}
	}

}


pub fn newVoiceDspSIG0() -> VoiceDspSIG0 { 
	VoiceDspSIG0 {
		iVec8: [0;2],
		iRec18: [0;2],
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
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	fHslider0: F32,
	fVec2: [F32;2],
	fHslider1: F32,
	fRec0: [F32;2],
	fRec1: [F32;2],
	fEntry0: F32,
	fSampleRate: i32,
	fConst1: F32,
	fCheckbox0: F32,
	fRec4: [F32;2],
	fTempPerm0: F32,
	fVslider0: F32,
	fVec3: [F32;2],
	iTempPerm1: i32,
	fTempPerm2: F32,
	fHslider2: F32,
	fVec4: [F32;2],
	fTempPerm3: F32,
	fRec5: [F32;2],
	fRec6: [F32;2],
	fTempPerm4: F32,
	fTempPerm5: F32,
	fTempPerm6: F32,
	fTempPerm7: F32,
	iTempPerm8: i32,
	fTempPerm9: F32,
	iTempPerm10: i32,
	iTempPerm11: i32,
	fTempPerm12: F32,
	iTempPerm13: i32,
	iTempPerm14: i32,
	fTempPerm15: F32,
	iTempPerm16: i32,
	iTempPerm17: i32,
	fTempPerm18: F32,
	iTempPerm19: i32,
	iTempPerm20: i32,
	fTempPerm21: F32,
	iTempPerm22: i32,
	iTempPerm23: i32,
	fTempPerm24: F32,
	iTempPerm25: i32,
	iTempPerm26: i32,
	fTempPerm27: F32,
	iTempPerm28: i32,
	iTempPerm29: i32,
	fTempPerm30: F32,
	iTempPerm31: i32,
	fHslider3: F32,
	fRec7: [F32;2],
	iTempPerm32: i32,
	fTempPerm33: F32,
	fHslider4: F32,
	fVec5: [F32;2],
	fHslider5: F32,
	fVec6: [F32;2],
	fTempPerm34: F32,
	fRec8: [F32;2],
	fRec9: [F32;2],
	fTempPerm35: F32,
	iRec10: [i32;2],
	fTempPerm36: F32,
	fConst2: F32,
	fTempPerm37: F32,
	iRec11: [i32;2],
	iTempPerm38: i32,
	fTempPerm39: F32,
	fEntry1: F32,
	iTempPerm40: i32,
	fHslider6: F32,
	iRec3: [i32;2],
	iTempPerm41: i32,
	fConst3: F32,
	fConst5: F32,
	fRec2: [F32;2],
	fHslider7: F32,
	fRec12: [F32;2],
	fConst6: F32,
	fTempPerm42: F32,
	fHslider8: F32,
	fVec7: [F32;2],
	fTempPerm43: F32,
	fRec14: [F32;2],
	fRec15: [F32;2],
	fTempPerm44: F32,
	fTempPerm45: F32,
	fConst7: F32,
	fTempPerm46: F32,
	fConst8: F32,
	fConst9: F32,
	fTempPerm47: F32,
	fRec19: [F32;2],
	fTempPerm48: F32,
	fRec20: [F32;2],
	iRec22: [i32;2],
	iTempPerm49: i32,
	fConst11: F32,
	fConst13: F32,
	fRec21: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fTempPerm50: F32,
	fRec25: [F32;2],
	fEntry2: F32,
	iRec26: [i32;2],
	fTempPerm51: F32,
	fTempPerm52: F32,
	fRec24: [F32;2],
	fTempPerm53: F32,
	fVec9: [F32;2],
	fRec23: [F32;2],
	fTempPerm54: F32,
	fHslider9: F32,
	fRec27: [F32;2],
	fEntry3: F32,
	fHslider10: F32,
	fVec10: [F32;2],
	fRec28: [F32;2],
	fRec29: [F32;2],
	fHslider11: F32,
	fHslider12: F32,
	fRec30: [F32;2],
	fTempPerm55: F32,
	fTempPerm56: F32,
	fRec17: [F32;2],
	fRec16: [F32;2],
	fTempPerm57: F32,
	fConst16: F32,
	fTempPerm58: F32,
	fRec32: [F32;2],
	fConst17: F32,
	fTempPerm59: F32,
	fRec31: [F32;2],
	fTempPerm60: F32,
	fConst18: F32,
	fTempPerm61: F32,
	fRec33: [F32;2],
	fTempPerm62: F32,
	fConst19: F32,
	fTempPerm63: F32,
	fTempPerm64: F32,
	fTempPerm65: F32,
	fTempPerm66: F32,
	fRec34: [F32;3],
	fTempPerm67: F32,
	fConst20: F32,
	fConst21: F32,
	fTempPerm68: F32,
	fTempPerm69: F32,
	fTempPerm70: F32,
	fVec11: [F32;2],
	fRec13: [F32;2],
	fTempPerm71: F32,
	fTempPerm72: F32,
	fRec35: [F32;2],
	fRec36: [F32;2],
	fCheckbox1: F32,
	fRec38: [F32;2],
	fVslider1: F32,
	fVslider2: F32,
	fRec39: [F32;2],
	fVslider3: F32,
	fRec40: [F32;2],
	fVslider4: F32,
	fVslider5: F32,
	fVslider6: F32,
	fRec41: [F32;2],
	fVslider7: F32,
	fRec37: [F32;2],
	fHslider13: F32,
	fVec12: [F32;2],
	fRec43: [F32;2],
	fRec44: [F32;2],
	fHslider14: F32,
	fRec45: [F32;2],
	fTempPerm73: F32,
	fTempPerm74: F32,
	fRec62: [F32;2],
	fRec61: [F32;2],
	fTempPerm75: F32,
	fVec13: [F32;2],
	fRec60: [F32;2],
	fTempPerm76: F32,
	fHslider15: F32,
	fVec14: [F32;2],
	fRec63: [F32;2],
	fRec64: [F32;2],
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
	fHslider16: F32,
	fVec15: [F32;2],
	fRec65: [F32;2],
	fRec66: [F32;2],
	fTempPerm92: F32,
	fTempPerm93: F32,
	fTempPerm94: F32,
	fTempPerm95: F32,
	fTempPerm96: F32,
	fRec67: [F32;2],
	fRec68: [F32;2],
	fVslider8: F32,
	fVec16: [F32;2],
	fTempPerm97: F32,
	fRec69: [F32;2],
	fRec70: [F32;2],
	fVslider9: F32,
	fVec17: [F32;2],
	fTempPerm98: F32,
	fRec71: [F32;2],
	fRec72: [F32;2],
	fHslider17: F32,
	fVec18: [F32;2],
	fTempPerm99: F32,
	fRec74: [F32;2],
	fRec75: [F32;2],
	fConst22: F32,
	fTempPerm100: F32,
	fTempPerm101: F32,
	fRec77: [F32;2],
	iRec78: [i32;2],
	iTempPerm102: i32,
	fRec76: [F32;2],
	fRec73: [F32;2],
	fHslider18: F32,
	fVec19: [F32;2],
	fTempPerm103: F32,
	fRec79: [F32;2],
	fRec80: [F32;2],
	fTempPerm104: F32,
	fTempPerm105: F32,
	iTempPerm106: i32,
	fTempPerm107: F32,
	fHslider19: F32,
	fVec20: [F32;2],
	fTempPerm108: F32,
	fRec81: [F32;2],
	fRec82: [F32;2],
	fTempPerm109: F32,
	fConst24: F32,
	fHslider20: F32,
	fVec21: [F32;2],
	fTempPerm110: F32,
	fRec87: [F32;2],
	fRec88: [F32;2],
	fTempPerm111: F32,
	fVec22: [F32;2],
	fConst25: F32,
	fRec86: [F32;2],
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fRec85: [F32;2],
	fTempPerm112: F32,
	fHslider21: F32,
	fTempPerm113: F32,
	fRec90: [F32;2],
	iTempPerm114: i32,
	iTempPerm115: i32,
	iRec89: [i32;2],
	fTempPerm116: F32,
	iTempPerm117: i32,
	iTempPerm118: i32,
	iRec91: [i32;2],
	fTempPerm119: F32,
	fRec92: [F32;2],
	fHslider22: F32,
	fTempPerm120: F32,
	fTempPerm121: F32,
	fTempPerm122: F32,
	fVec23: [F32;2],
	fConst29: F32,
	fConst30: F32,
	fConst31: F32,
	fRec84: [F32;2],
	fTempPerm123: F32,
	fVec24: [F32;2],
	fRec83: [F32;2],
	fTempPerm124: F32,
	fTempPerm125: F32,
	fConst33: F32,
	fConst34: F32,
	fTempPerm126: F32,
	fRec96: [F32;2],
	fRec95: [F32;2],
	fTempPerm127: F32,
	fVec25: [F32;2],
	fConst35: F32,
	fRec94: [F32;2],
	fTempPerm128: F32,
	fTempPerm129: F32,
	fTempPerm130: F32,
	fTempPerm131: F32,
	fTempPerm132: F32,
	fTempPerm133: F32,
	fTempPerm134: F32,
	fConst36: F32,
	fConst38: F32,
	fTempPerm135: F32,
	fVec26: [F32;2],
	fTempPerm136: F32,
	fConst39: F32,
	fRec98: [F32;2],
	fConst40: F32,
	fConst41: F32,
	fConst43: F32,
	fConst44: F32,
	fRec99: [F32;2],
	fConst45: F32,
	fConst46: F32,
	fTempPerm137: F32,
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fTempPerm138: F32,
	fVec27: [F32;2],
	fConst51: F32,
	fConst52: F32,
	fRec97: [F32;2],
	fTempPerm139: F32,
	fTempPerm140: F32,
	fTempPerm141: F32,
	fTempPerm142: F32,
	fTempPerm143: F32,
	fTempPerm144: F32,
	fTempPerm145: F32,
	fTempPerm146: F32,
	fTempPerm147: F32,
	fTempPerm148: F32,
	fTempPerm149: F32,
	fTempPerm150: F32,
	fTempPerm151: F32,
	fTempPerm152: F32,
	fTempPerm153: F32,
	fTempPerm154: F32,
	fTempPerm155: F32,
	fTempPerm156: F32,
	fTempPerm157: F32,
	fTempPerm158: F32,
	fTempPerm159: F32,
	fTempPerm160: F32,
	fTempPerm161: F32,
	fTempPerm162: F32,
	fTempPerm163: F32,
	fTempPerm164: F32,
	fVec28: [F32;2],
	fRec93: [F32;2],
	fTempPerm165: F32,
	fTempPerm166: F32,
	fTempPerm167: F32,
	fTempPerm168: F32,
	fTempPerm169: F32,
	fTempPerm170: F32,
	fTempPerm171: F32,
	fTempPerm172: F32,
	fTempPerm173: F32,
	fTempPerm174: F32,
	fTempPerm175: F32,
	fTempPerm176: F32,
	fRec46: [F32;2],
	fTempPerm177: F32,
	fTempPerm178: F32,
	fRec47: [F32;2],
	fTempPerm179: F32,
	fTempPerm180: F32,
	fRec48: [F32;2],
	fTempPerm181: F32,
	fTempPerm182: F32,
	fRec49: [F32;2],
	fTempPerm183: F32,
	fTempPerm184: F32,
	fTempPerm185: F32,
	fTempPerm186: F32,
	fRec50: [F32;2],
	fTempPerm187: F32,
	fTempPerm188: F32,
	fTempPerm189: F32,
	fTempPerm190: F32,
	fRec51: [F32;2],
	fTempPerm191: F32,
	fTempPerm192: F32,
	fTempPerm193: F32,
	fTempPerm194: F32,
	fRec52: [F32;2],
	fTempPerm195: F32,
	fTempPerm196: F32,
	fTempPerm197: F32,
	fTempPerm198: F32,
	fRec53: [F32;2],
	fRec54: [F32;2],
	fRec55: [F32;2],
	fRec56: [F32;2],
	fRec57: [F32;2],
	fTempPerm199: F32,
	fTempPerm200: F32,
	fRec58: [F32;2],
	fTempPerm201: F32,
	fTempPerm202: F32,
	fTempPerm203: F32,
	fTempPerm204: F32,
	fTempPerm205: F32,
	fTempPerm206: F32,
	fTempPerm207: F32,
	fTempPerm208: F32,
	fTempPerm209: F32,
	fTempPerm210: F32,
	fTempPerm211: F32,
	fTempPerm212: F32,
	fTempPerm213: F32,
	fTempPerm214: F32,
	fTempPerm215: F32,
	fTempPerm216: F32,
	fTempPerm217: F32,
	fTempPerm218: F32,
	fTempPerm219: F32,
	fTempPerm220: F32,
	fTempPerm221: F32,
	fTempPerm222: F32,
	fTempPerm223: F32,
	fTempPerm224: F32,
	fRec100: [F32;2],
	fTempPerm225: F32,
	fTempPerm226: F32,
	fRec101: [F32;2],
	fTempPerm227: F32,
	fTempPerm228: F32,
	fRec102: [F32;2],
	fTempPerm229: F32,
	fTempPerm230: F32,
	fRec103: [F32;2],
	fTempPerm231: F32,
	fTempPerm232: F32,
	fRec104: [F32;2],
	fTempPerm233: F32,
	fTempPerm234: F32,
	fTempPerm235: F32,
	fRec105: [F32;2],
	fTempPerm236: F32,
	fTempPerm237: F32,
	fTempPerm238: F32,
	fRec106: [F32;2],
	fTempPerm239: F32,
	fTempPerm240: F32,
	fTempPerm241: F32,
	fRec107: [F32;2],
	fRec108: [F32;2],
	fRec109: [F32;2],
	fRec110: [F32;2],
	fTempPerm242: F32,
	fTempPerm243: F32,
	fRec111: [F32;2],
	fRec112: [F32;2],
	fTempPerm244: F32,
	fCheckbox2: F32,
	fHslider23: F32,
	fVec29: [F32;2],
	fRec114: [F32;2],
	fRec115: [F32;2],
	fTempPerm245: F32,
	fTempPerm246: F32,
	fTempPerm247: F32,
	fTempPerm248: F32,
	fConst53: F32,
	fTempPerm249: F32,
	fConst54: F32,
	fTempPerm250: F32,
	fTempPerm251: F32,
	fTempPerm252: F32,
	fRec116: [F32;2],
	fTempPerm253: F32,
	fTempPerm254: F32,
	fRec117: [F32;2],
	fHslider24: F32,
	fVec30: [F32;2],
	fRec121: [F32;2],
	fRec122: [F32;2],
	fTempPerm255: F32,
	iTempPerm256: i32,
	fTempPerm257: F32,
	fTempPerm258: F32,
	fTempPerm259: F32,
	fTempPerm260: F32,
	fTempPerm261: F32,
	fTempPerm262: F32,
	fTempPerm263: F32,
	fTempPerm264: F32,
	fTempPerm265: F32,
	fTempPerm266: F32,
	fTempPerm267: F32,
	fRec123: [F32;2],
	fTempPerm268: F32,
	fTempPerm269: F32,
	fRec124: [F32;2],
	fTempPerm270: F32,
	fTempPerm271: F32,
	fTempPerm272: F32,
	fTempPerm273: F32,
	fVec31: [F32;2],
	fRec42: [F32;2],
	fHslider25: F32,
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
			fEntry0: 0.0,
			fSampleRate: 0,
			fConst1: 0.0,
			fCheckbox0: 0.0,
			fRec4: [0.0;2],
			fTempPerm0: 0.0,
			fVslider0: 0.0,
			fVec3: [0.0;2],
			iTempPerm1: 0,
			fTempPerm2: 0.0,
			fHslider2: 0.0,
			fVec4: [0.0;2],
			fTempPerm3: 0.0,
			fRec5: [0.0;2],
			fRec6: [0.0;2],
			fTempPerm4: 0.0,
			fTempPerm5: 0.0,
			fTempPerm6: 0.0,
			fTempPerm7: 0.0,
			iTempPerm8: 0,
			fTempPerm9: 0.0,
			iTempPerm10: 0,
			iTempPerm11: 0,
			fTempPerm12: 0.0,
			iTempPerm13: 0,
			iTempPerm14: 0,
			fTempPerm15: 0.0,
			iTempPerm16: 0,
			iTempPerm17: 0,
			fTempPerm18: 0.0,
			iTempPerm19: 0,
			iTempPerm20: 0,
			fTempPerm21: 0.0,
			iTempPerm22: 0,
			iTempPerm23: 0,
			fTempPerm24: 0.0,
			iTempPerm25: 0,
			iTempPerm26: 0,
			fTempPerm27: 0.0,
			iTempPerm28: 0,
			iTempPerm29: 0,
			fTempPerm30: 0.0,
			iTempPerm31: 0,
			fHslider3: 0.0,
			fRec7: [0.0;2],
			iTempPerm32: 0,
			fTempPerm33: 0.0,
			fHslider4: 0.0,
			fVec5: [0.0;2],
			fHslider5: 0.0,
			fVec6: [0.0;2],
			fTempPerm34: 0.0,
			fRec8: [0.0;2],
			fRec9: [0.0;2],
			fTempPerm35: 0.0,
			iRec10: [0;2],
			fTempPerm36: 0.0,
			fConst2: 0.0,
			fTempPerm37: 0.0,
			iRec11: [0;2],
			iTempPerm38: 0,
			fTempPerm39: 0.0,
			fEntry1: 0.0,
			iTempPerm40: 0,
			fHslider6: 0.0,
			iRec3: [0;2],
			iTempPerm41: 0,
			fConst3: 0.0,
			fConst5: 0.0,
			fRec2: [0.0;2],
			fHslider7: 0.0,
			fRec12: [0.0;2],
			fConst6: 0.0,
			fTempPerm42: 0.0,
			fHslider8: 0.0,
			fVec7: [0.0;2],
			fTempPerm43: 0.0,
			fRec14: [0.0;2],
			fRec15: [0.0;2],
			fTempPerm44: 0.0,
			fTempPerm45: 0.0,
			fConst7: 0.0,
			fTempPerm46: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fTempPerm47: 0.0,
			fRec19: [0.0;2],
			fTempPerm48: 0.0,
			fRec20: [0.0;2],
			iRec22: [0;2],
			iTempPerm49: 0,
			fConst11: 0.0,
			fConst13: 0.0,
			fRec21: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fTempPerm50: 0.0,
			fRec25: [0.0;2],
			fEntry2: 0.0,
			iRec26: [0;2],
			fTempPerm51: 0.0,
			fTempPerm52: 0.0,
			fRec24: [0.0;2],
			fTempPerm53: 0.0,
			fVec9: [0.0;2],
			fRec23: [0.0;2],
			fTempPerm54: 0.0,
			fHslider9: 0.0,
			fRec27: [0.0;2],
			fEntry3: 0.0,
			fHslider10: 0.0,
			fVec10: [0.0;2],
			fRec28: [0.0;2],
			fRec29: [0.0;2],
			fHslider11: 0.0,
			fHslider12: 0.0,
			fRec30: [0.0;2],
			fTempPerm55: 0.0,
			fTempPerm56: 0.0,
			fRec17: [0.0;2],
			fRec16: [0.0;2],
			fTempPerm57: 0.0,
			fConst16: 0.0,
			fTempPerm58: 0.0,
			fRec32: [0.0;2],
			fConst17: 0.0,
			fTempPerm59: 0.0,
			fRec31: [0.0;2],
			fTempPerm60: 0.0,
			fConst18: 0.0,
			fTempPerm61: 0.0,
			fRec33: [0.0;2],
			fTempPerm62: 0.0,
			fConst19: 0.0,
			fTempPerm63: 0.0,
			fTempPerm64: 0.0,
			fTempPerm65: 0.0,
			fTempPerm66: 0.0,
			fRec34: [0.0;3],
			fTempPerm67: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fTempPerm68: 0.0,
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			fVec11: [0.0;2],
			fRec13: [0.0;2],
			fTempPerm71: 0.0,
			fTempPerm72: 0.0,
			fRec35: [0.0;2],
			fRec36: [0.0;2],
			fCheckbox1: 0.0,
			fRec38: [0.0;2],
			fVslider1: 0.0,
			fVslider2: 0.0,
			fRec39: [0.0;2],
			fVslider3: 0.0,
			fRec40: [0.0;2],
			fVslider4: 0.0,
			fVslider5: 0.0,
			fVslider6: 0.0,
			fRec41: [0.0;2],
			fVslider7: 0.0,
			fRec37: [0.0;2],
			fHslider13: 0.0,
			fVec12: [0.0;2],
			fRec43: [0.0;2],
			fRec44: [0.0;2],
			fHslider14: 0.0,
			fRec45: [0.0;2],
			fTempPerm73: 0.0,
			fTempPerm74: 0.0,
			fRec62: [0.0;2],
			fRec61: [0.0;2],
			fTempPerm75: 0.0,
			fVec13: [0.0;2],
			fRec60: [0.0;2],
			fTempPerm76: 0.0,
			fHslider15: 0.0,
			fVec14: [0.0;2],
			fRec63: [0.0;2],
			fRec64: [0.0;2],
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
			fHslider16: 0.0,
			fVec15: [0.0;2],
			fRec65: [0.0;2],
			fRec66: [0.0;2],
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fTempPerm94: 0.0,
			fTempPerm95: 0.0,
			fTempPerm96: 0.0,
			fRec67: [0.0;2],
			fRec68: [0.0;2],
			fVslider8: 0.0,
			fVec16: [0.0;2],
			fTempPerm97: 0.0,
			fRec69: [0.0;2],
			fRec70: [0.0;2],
			fVslider9: 0.0,
			fVec17: [0.0;2],
			fTempPerm98: 0.0,
			fRec71: [0.0;2],
			fRec72: [0.0;2],
			fHslider17: 0.0,
			fVec18: [0.0;2],
			fTempPerm99: 0.0,
			fRec74: [0.0;2],
			fRec75: [0.0;2],
			fConst22: 0.0,
			fTempPerm100: 0.0,
			fTempPerm101: 0.0,
			fRec77: [0.0;2],
			iRec78: [0;2],
			iTempPerm102: 0,
			fRec76: [0.0;2],
			fRec73: [0.0;2],
			fHslider18: 0.0,
			fVec19: [0.0;2],
			fTempPerm103: 0.0,
			fRec79: [0.0;2],
			fRec80: [0.0;2],
			fTempPerm104: 0.0,
			fTempPerm105: 0.0,
			iTempPerm106: 0,
			fTempPerm107: 0.0,
			fHslider19: 0.0,
			fVec20: [0.0;2],
			fTempPerm108: 0.0,
			fRec81: [0.0;2],
			fRec82: [0.0;2],
			fTempPerm109: 0.0,
			fConst24: 0.0,
			fHslider20: 0.0,
			fVec21: [0.0;2],
			fTempPerm110: 0.0,
			fRec87: [0.0;2],
			fRec88: [0.0;2],
			fTempPerm111: 0.0,
			fVec22: [0.0;2],
			fConst25: 0.0,
			fRec86: [0.0;2],
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fRec85: [0.0;2],
			fTempPerm112: 0.0,
			fHslider21: 0.0,
			fTempPerm113: 0.0,
			fRec90: [0.0;2],
			iTempPerm114: 0,
			iTempPerm115: 0,
			iRec89: [0;2],
			fTempPerm116: 0.0,
			iTempPerm117: 0,
			iTempPerm118: 0,
			iRec91: [0;2],
			fTempPerm119: 0.0,
			fRec92: [0.0;2],
			fHslider22: 0.0,
			fTempPerm120: 0.0,
			fTempPerm121: 0.0,
			fTempPerm122: 0.0,
			fVec23: [0.0;2],
			fConst29: 0.0,
			fConst30: 0.0,
			fConst31: 0.0,
			fRec84: [0.0;2],
			fTempPerm123: 0.0,
			fVec24: [0.0;2],
			fRec83: [0.0;2],
			fTempPerm124: 0.0,
			fTempPerm125: 0.0,
			fConst33: 0.0,
			fConst34: 0.0,
			fTempPerm126: 0.0,
			fRec96: [0.0;2],
			fRec95: [0.0;2],
			fTempPerm127: 0.0,
			fVec25: [0.0;2],
			fConst35: 0.0,
			fRec94: [0.0;2],
			fTempPerm128: 0.0,
			fTempPerm129: 0.0,
			fTempPerm130: 0.0,
			fTempPerm131: 0.0,
			fTempPerm132: 0.0,
			fTempPerm133: 0.0,
			fTempPerm134: 0.0,
			fConst36: 0.0,
			fConst38: 0.0,
			fTempPerm135: 0.0,
			fVec26: [0.0;2],
			fTempPerm136: 0.0,
			fConst39: 0.0,
			fRec98: [0.0;2],
			fConst40: 0.0,
			fConst41: 0.0,
			fConst43: 0.0,
			fConst44: 0.0,
			fRec99: [0.0;2],
			fConst45: 0.0,
			fConst46: 0.0,
			fTempPerm137: 0.0,
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fTempPerm138: 0.0,
			fVec27: [0.0;2],
			fConst51: 0.0,
			fConst52: 0.0,
			fRec97: [0.0;2],
			fTempPerm139: 0.0,
			fTempPerm140: 0.0,
			fTempPerm141: 0.0,
			fTempPerm142: 0.0,
			fTempPerm143: 0.0,
			fTempPerm144: 0.0,
			fTempPerm145: 0.0,
			fTempPerm146: 0.0,
			fTempPerm147: 0.0,
			fTempPerm148: 0.0,
			fTempPerm149: 0.0,
			fTempPerm150: 0.0,
			fTempPerm151: 0.0,
			fTempPerm152: 0.0,
			fTempPerm153: 0.0,
			fTempPerm154: 0.0,
			fTempPerm155: 0.0,
			fTempPerm156: 0.0,
			fTempPerm157: 0.0,
			fTempPerm158: 0.0,
			fTempPerm159: 0.0,
			fTempPerm160: 0.0,
			fTempPerm161: 0.0,
			fTempPerm162: 0.0,
			fTempPerm163: 0.0,
			fTempPerm164: 0.0,
			fVec28: [0.0;2],
			fRec93: [0.0;2],
			fTempPerm165: 0.0,
			fTempPerm166: 0.0,
			fTempPerm167: 0.0,
			fTempPerm168: 0.0,
			fTempPerm169: 0.0,
			fTempPerm170: 0.0,
			fTempPerm171: 0.0,
			fTempPerm172: 0.0,
			fTempPerm173: 0.0,
			fTempPerm174: 0.0,
			fTempPerm175: 0.0,
			fTempPerm176: 0.0,
			fRec46: [0.0;2],
			fTempPerm177: 0.0,
			fTempPerm178: 0.0,
			fRec47: [0.0;2],
			fTempPerm179: 0.0,
			fTempPerm180: 0.0,
			fRec48: [0.0;2],
			fTempPerm181: 0.0,
			fTempPerm182: 0.0,
			fRec49: [0.0;2],
			fTempPerm183: 0.0,
			fTempPerm184: 0.0,
			fTempPerm185: 0.0,
			fTempPerm186: 0.0,
			fRec50: [0.0;2],
			fTempPerm187: 0.0,
			fTempPerm188: 0.0,
			fTempPerm189: 0.0,
			fTempPerm190: 0.0,
			fRec51: [0.0;2],
			fTempPerm191: 0.0,
			fTempPerm192: 0.0,
			fTempPerm193: 0.0,
			fTempPerm194: 0.0,
			fRec52: [0.0;2],
			fTempPerm195: 0.0,
			fTempPerm196: 0.0,
			fTempPerm197: 0.0,
			fTempPerm198: 0.0,
			fRec53: [0.0;2],
			fRec54: [0.0;2],
			fRec55: [0.0;2],
			fRec56: [0.0;2],
			fRec57: [0.0;2],
			fTempPerm199: 0.0,
			fTempPerm200: 0.0,
			fRec58: [0.0;2],
			fTempPerm201: 0.0,
			fTempPerm202: 0.0,
			fTempPerm203: 0.0,
			fTempPerm204: 0.0,
			fTempPerm205: 0.0,
			fTempPerm206: 0.0,
			fTempPerm207: 0.0,
			fTempPerm208: 0.0,
			fTempPerm209: 0.0,
			fTempPerm210: 0.0,
			fTempPerm211: 0.0,
			fTempPerm212: 0.0,
			fTempPerm213: 0.0,
			fTempPerm214: 0.0,
			fTempPerm215: 0.0,
			fTempPerm216: 0.0,
			fTempPerm217: 0.0,
			fTempPerm218: 0.0,
			fTempPerm219: 0.0,
			fTempPerm220: 0.0,
			fTempPerm221: 0.0,
			fTempPerm222: 0.0,
			fTempPerm223: 0.0,
			fTempPerm224: 0.0,
			fRec100: [0.0;2],
			fTempPerm225: 0.0,
			fTempPerm226: 0.0,
			fRec101: [0.0;2],
			fTempPerm227: 0.0,
			fTempPerm228: 0.0,
			fRec102: [0.0;2],
			fTempPerm229: 0.0,
			fTempPerm230: 0.0,
			fRec103: [0.0;2],
			fTempPerm231: 0.0,
			fTempPerm232: 0.0,
			fRec104: [0.0;2],
			fTempPerm233: 0.0,
			fTempPerm234: 0.0,
			fTempPerm235: 0.0,
			fRec105: [0.0;2],
			fTempPerm236: 0.0,
			fTempPerm237: 0.0,
			fTempPerm238: 0.0,
			fRec106: [0.0;2],
			fTempPerm239: 0.0,
			fTempPerm240: 0.0,
			fTempPerm241: 0.0,
			fRec107: [0.0;2],
			fRec108: [0.0;2],
			fRec109: [0.0;2],
			fRec110: [0.0;2],
			fTempPerm242: 0.0,
			fTempPerm243: 0.0,
			fRec111: [0.0;2],
			fRec112: [0.0;2],
			fTempPerm244: 0.0,
			fCheckbox2: 0.0,
			fHslider23: 0.0,
			fVec29: [0.0;2],
			fRec114: [0.0;2],
			fRec115: [0.0;2],
			fTempPerm245: 0.0,
			fTempPerm246: 0.0,
			fTempPerm247: 0.0,
			fTempPerm248: 0.0,
			fConst53: 0.0,
			fTempPerm249: 0.0,
			fConst54: 0.0,
			fTempPerm250: 0.0,
			fTempPerm251: 0.0,
			fTempPerm252: 0.0,
			fRec116: [0.0;2],
			fTempPerm253: 0.0,
			fTempPerm254: 0.0,
			fRec117: [0.0;2],
			fHslider24: 0.0,
			fVec30: [0.0;2],
			fRec121: [0.0;2],
			fRec122: [0.0;2],
			fTempPerm255: 0.0,
			iTempPerm256: 0,
			fTempPerm257: 0.0,
			fTempPerm258: 0.0,
			fTempPerm259: 0.0,
			fTempPerm260: 0.0,
			fTempPerm261: 0.0,
			fTempPerm262: 0.0,
			fTempPerm263: 0.0,
			fTempPerm264: 0.0,
			fTempPerm265: 0.0,
			fTempPerm266: 0.0,
			fTempPerm267: 0.0,
			fRec123: [0.0;2],
			fTempPerm268: 0.0,
			fTempPerm269: 0.0,
			fRec124: [0.0;2],
			fTempPerm270: 0.0,
			fTempPerm271: 0.0,
			fTempPerm272: 0.0,
			fTempPerm273: 0.0,
			fVec31: [0.0;2],
			fRec42: [0.0;2],
			fHslider25: 0.0,
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
		return 2;
	}
	fn get_num_outputs(&self) -> i32 {
		return 8;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: VoiceDspSIG0 = newVoiceDspSIG0();
		sig0.instance_initVoiceDspSIG0(sample_rate);
		sig0.fillVoiceDspSIG0(65536, unsafe { &mut ftbl0VoiceDspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 64.0;
		self.fEntry0 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fVslider0 = 0.15;
		self.fHslider2 = 0.1;
		self.fHslider3 = 1.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 6e+01;
		self.fEntry1 = 0.0;
		self.fHslider6 = 0.0;
		self.fHslider7 = 1.0;
		self.fHslider8 = 0.0;
		self.fEntry2 = 1.0;
		self.fHslider9 = 0.0;
		self.fEntry3 = 0.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 1.1e+02;
		self.fCheckbox1 = 0.0;
		self.fVslider1 = -0.6;
		self.fVslider2 = 0.5;
		self.fVslider3 = 0.005;
		self.fVslider4 = 0.6;
		self.fVslider5 = -0.6;
		self.fVslider6 = 0.4;
		self.fVslider7 = 0.6;
		self.fHslider13 = 2e+01;
		self.fHslider14 = 1.0;
		self.fHslider15 = 1.2e+03;
		self.fHslider16 = 0.8;
		self.fVslider8 = 0.0;
		self.fVslider9 = 1.0;
		self.fHslider17 = 1.5;
		self.fHslider18 = 0.0;
		self.fHslider19 = 0.0;
		self.fHslider20 = 0.3;
		self.fHslider21 = 0.0;
		self.fHslider22 = 8.0;
		self.fCheckbox2 = 0.0;
		self.fHslider23 = 0.0;
		self.fHslider24 = 0.0;
		self.fHslider25 = 0.8;
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
			self.fRec4[l5 as usize] = 0.0;
		}
		self.fTempPerm0 = 0.0;
		for l6 in 0..2 {
			self.fVec3[l6 as usize] = 0.0;
		}
		self.iTempPerm1 = 0;
		self.fTempPerm2 = 0.0;
		for l7 in 0..2 {
			self.fVec4[l7 as usize] = 0.0;
		}
		self.fTempPerm3 = 0.0;
		for l8 in 0..2 {
			self.fRec5[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec6[l9 as usize] = 0.0;
		}
		self.fTempPerm4 = 0.0;
		self.fTempPerm5 = 0.0;
		self.fTempPerm6 = 0.0;
		self.fTempPerm7 = 0.0;
		self.iTempPerm8 = 0;
		self.fTempPerm9 = 0.0;
		self.iTempPerm10 = 0;
		self.iTempPerm11 = 0;
		self.fTempPerm12 = 0.0;
		self.iTempPerm13 = 0;
		self.iTempPerm14 = 0;
		self.fTempPerm15 = 0.0;
		self.iTempPerm16 = 0;
		self.iTempPerm17 = 0;
		self.fTempPerm18 = 0.0;
		self.iTempPerm19 = 0;
		self.iTempPerm20 = 0;
		self.fTempPerm21 = 0.0;
		self.iTempPerm22 = 0;
		self.iTempPerm23 = 0;
		self.fTempPerm24 = 0.0;
		self.iTempPerm25 = 0;
		self.iTempPerm26 = 0;
		self.fTempPerm27 = 0.0;
		self.iTempPerm28 = 0;
		self.iTempPerm29 = 0;
		self.fTempPerm30 = 0.0;
		self.iTempPerm31 = 0;
		for l10 in 0..2 {
			self.fRec7[l10 as usize] = 0.0;
		}
		self.iTempPerm32 = 0;
		self.fTempPerm33 = 0.0;
		for l11 in 0..2 {
			self.fVec5[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fVec6[l12 as usize] = 0.0;
		}
		self.fTempPerm34 = 0.0;
		for l13 in 0..2 {
			self.fRec8[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec9[l14 as usize] = 0.0;
		}
		self.fTempPerm35 = 0.0;
		for l15 in 0..2 {
			self.iRec10[l15 as usize] = 0;
		}
		self.fTempPerm36 = 0.0;
		self.fTempPerm37 = 0.0;
		for l16 in 0..2 {
			self.iRec11[l16 as usize] = 0;
		}
		self.iTempPerm38 = 0;
		self.fTempPerm39 = 0.0;
		self.iTempPerm40 = 0;
		for l17 in 0..2 {
			self.iRec3[l17 as usize] = 0;
		}
		self.iTempPerm41 = 0;
		for l18 in 0..2 {
			self.fRec2[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec12[l19 as usize] = 0.0;
		}
		self.fTempPerm42 = 0.0;
		for l20 in 0..2 {
			self.fVec7[l20 as usize] = 0.0;
		}
		self.fTempPerm43 = 0.0;
		for l21 in 0..2 {
			self.fRec14[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec15[l22 as usize] = 0.0;
		}
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		for l25 in 0..2 {
			self.fRec19[l25 as usize] = 0.0;
		}
		self.fTempPerm48 = 0.0;
		for l26 in 0..2 {
			self.fRec20[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.iRec22[l27 as usize] = 0;
		}
		self.iTempPerm49 = 0;
		for l28 in 0..2 {
			self.fRec21[l28 as usize] = 0.0;
		}
		self.fTempPerm50 = 0.0;
		for l29 in 0..2 {
			self.fRec25[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.iRec26[l30 as usize] = 0;
		}
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		for l31 in 0..2 {
			self.fRec24[l31 as usize] = 0.0;
		}
		self.fTempPerm53 = 0.0;
		for l32 in 0..2 {
			self.fVec9[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec23[l33 as usize] = 0.0;
		}
		self.fTempPerm54 = 0.0;
		for l34 in 0..2 {
			self.fRec27[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fVec10[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec28[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec29[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec30[l38 as usize] = 0.0;
		}
		self.fTempPerm55 = 0.0;
		self.fTempPerm56 = 0.0;
		for l39 in 0..2 {
			self.fRec17[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec16[l40 as usize] = 0.0;
		}
		self.fTempPerm57 = 0.0;
		self.fTempPerm58 = 0.0;
		for l41 in 0..2 {
			self.fRec32[l41 as usize] = 0.0;
		}
		self.fTempPerm59 = 0.0;
		for l42 in 0..2 {
			self.fRec31[l42 as usize] = 0.0;
		}
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		for l43 in 0..2 {
			self.fRec33[l43 as usize] = 0.0;
		}
		self.fTempPerm62 = 0.0;
		self.fTempPerm63 = 0.0;
		self.fTempPerm64 = 0.0;
		self.fTempPerm65 = 0.0;
		self.fTempPerm66 = 0.0;
		for l44 in 0..3 {
			self.fRec34[l44 as usize] = 0.0;
		}
		self.fTempPerm67 = 0.0;
		self.fTempPerm68 = 0.0;
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		for l45 in 0..2 {
			self.fVec11[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec13[l46 as usize] = 0.0;
		}
		self.fTempPerm71 = 0.0;
		self.fTempPerm72 = 0.0;
		for l47 in 0..2 {
			self.fRec35[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec36[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec38[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec39[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec40[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec41[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec37[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec12[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec43[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec44[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec45[l57 as usize] = 0.0;
		}
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		for l58 in 0..2 {
			self.fRec62[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec61[l59 as usize] = 0.0;
		}
		self.fTempPerm75 = 0.0;
		for l60 in 0..2 {
			self.fVec13[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec60[l61 as usize] = 0.0;
		}
		self.fTempPerm76 = 0.0;
		for l62 in 0..2 {
			self.fVec14[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec63[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec64[l64 as usize] = 0.0;
		}
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
		for l65 in 0..2 {
			self.fVec15[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec65[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec66[l67 as usize] = 0.0;
		}
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		self.fTempPerm96 = 0.0;
		for l68 in 0..2 {
			self.fRec67[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec68[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fVec16[l70 as usize] = 0.0;
		}
		self.fTempPerm97 = 0.0;
		for l71 in 0..2 {
			self.fRec69[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec70[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fVec17[l73 as usize] = 0.0;
		}
		self.fTempPerm98 = 0.0;
		for l74 in 0..2 {
			self.fRec71[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec72[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fVec18[l76 as usize] = 0.0;
		}
		self.fTempPerm99 = 0.0;
		for l77 in 0..2 {
			self.fRec74[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec75[l78 as usize] = 0.0;
		}
		self.fTempPerm100 = 0.0;
		self.fTempPerm101 = 0.0;
		for l79 in 0..2 {
			self.fRec77[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.iRec78[l80 as usize] = 0;
		}
		self.iTempPerm102 = 0;
		for l81 in 0..2 {
			self.fRec76[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec73[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fVec19[l83 as usize] = 0.0;
		}
		self.fTempPerm103 = 0.0;
		for l84 in 0..2 {
			self.fRec79[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec80[l85 as usize] = 0.0;
		}
		self.fTempPerm104 = 0.0;
		self.fTempPerm105 = 0.0;
		self.iTempPerm106 = 0;
		self.fTempPerm107 = 0.0;
		for l86 in 0..2 {
			self.fVec20[l86 as usize] = 0.0;
		}
		self.fTempPerm108 = 0.0;
		for l87 in 0..2 {
			self.fRec81[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec82[l88 as usize] = 0.0;
		}
		self.fTempPerm109 = 0.0;
		for l89 in 0..2 {
			self.fVec21[l89 as usize] = 0.0;
		}
		self.fTempPerm110 = 0.0;
		for l90 in 0..2 {
			self.fRec87[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec88[l91 as usize] = 0.0;
		}
		self.fTempPerm111 = 0.0;
		for l92 in 0..2 {
			self.fVec22[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec86[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec85[l94 as usize] = 0.0;
		}
		self.fTempPerm112 = 0.0;
		self.fTempPerm113 = 0.0;
		for l95 in 0..2 {
			self.fRec90[l95 as usize] = 0.0;
		}
		self.iTempPerm114 = 0;
		self.iTempPerm115 = 0;
		for l96 in 0..2 {
			self.iRec89[l96 as usize] = 0;
		}
		self.fTempPerm116 = 0.0;
		self.iTempPerm117 = 0;
		self.iTempPerm118 = 0;
		for l97 in 0..2 {
			self.iRec91[l97 as usize] = 0;
		}
		self.fTempPerm119 = 0.0;
		for l98 in 0..2 {
			self.fRec92[l98 as usize] = 0.0;
		}
		self.fTempPerm120 = 0.0;
		self.fTempPerm121 = 0.0;
		self.fTempPerm122 = 0.0;
		for l99 in 0..2 {
			self.fVec23[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec84[l100 as usize] = 0.0;
		}
		self.fTempPerm123 = 0.0;
		for l101 in 0..2 {
			self.fVec24[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec83[l102 as usize] = 0.0;
		}
		self.fTempPerm124 = 0.0;
		self.fTempPerm125 = 0.0;
		self.fTempPerm126 = 0.0;
		for l103 in 0..2 {
			self.fRec96[l103 as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec95[l104 as usize] = 0.0;
		}
		self.fTempPerm127 = 0.0;
		for l105 in 0..2 {
			self.fVec25[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec94[l106 as usize] = 0.0;
		}
		self.fTempPerm128 = 0.0;
		self.fTempPerm129 = 0.0;
		self.fTempPerm130 = 0.0;
		self.fTempPerm131 = 0.0;
		self.fTempPerm132 = 0.0;
		self.fTempPerm133 = 0.0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		for l107 in 0..2 {
			self.fVec26[l107 as usize] = 0.0;
		}
		self.fTempPerm136 = 0.0;
		for l108 in 0..2 {
			self.fRec98[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec99[l109 as usize] = 0.0;
		}
		self.fTempPerm137 = 0.0;
		self.fTempPerm138 = 0.0;
		for l110 in 0..2 {
			self.fVec27[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec97[l111 as usize] = 0.0;
		}
		self.fTempPerm139 = 0.0;
		self.fTempPerm140 = 0.0;
		self.fTempPerm141 = 0.0;
		self.fTempPerm142 = 0.0;
		self.fTempPerm143 = 0.0;
		self.fTempPerm144 = 0.0;
		self.fTempPerm145 = 0.0;
		self.fTempPerm146 = 0.0;
		self.fTempPerm147 = 0.0;
		self.fTempPerm148 = 0.0;
		self.fTempPerm149 = 0.0;
		self.fTempPerm150 = 0.0;
		self.fTempPerm151 = 0.0;
		self.fTempPerm152 = 0.0;
		self.fTempPerm153 = 0.0;
		self.fTempPerm154 = 0.0;
		self.fTempPerm155 = 0.0;
		self.fTempPerm156 = 0.0;
		self.fTempPerm157 = 0.0;
		self.fTempPerm158 = 0.0;
		self.fTempPerm159 = 0.0;
		self.fTempPerm160 = 0.0;
		self.fTempPerm161 = 0.0;
		self.fTempPerm162 = 0.0;
		self.fTempPerm163 = 0.0;
		self.fTempPerm164 = 0.0;
		for l112 in 0..2 {
			self.fVec28[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec93[l113 as usize] = 0.0;
		}
		self.fTempPerm165 = 0.0;
		self.fTempPerm166 = 0.0;
		self.fTempPerm167 = 0.0;
		self.fTempPerm168 = 0.0;
		self.fTempPerm169 = 0.0;
		self.fTempPerm170 = 0.0;
		self.fTempPerm171 = 0.0;
		self.fTempPerm172 = 0.0;
		self.fTempPerm173 = 0.0;
		self.fTempPerm174 = 0.0;
		self.fTempPerm175 = 0.0;
		self.fTempPerm176 = 0.0;
		for l114 in 0..2 {
			self.fRec46[l114 as usize] = 0.0;
		}
		self.fTempPerm177 = 0.0;
		self.fTempPerm178 = 0.0;
		for l115 in 0..2 {
			self.fRec47[l115 as usize] = 0.0;
		}
		self.fTempPerm179 = 0.0;
		self.fTempPerm180 = 0.0;
		for l116 in 0..2 {
			self.fRec48[l116 as usize] = 0.0;
		}
		self.fTempPerm181 = 0.0;
		self.fTempPerm182 = 0.0;
		for l117 in 0..2 {
			self.fRec49[l117 as usize] = 0.0;
		}
		self.fTempPerm183 = 0.0;
		self.fTempPerm184 = 0.0;
		self.fTempPerm185 = 0.0;
		self.fTempPerm186 = 0.0;
		for l118 in 0..2 {
			self.fRec50[l118 as usize] = 0.0;
		}
		self.fTempPerm187 = 0.0;
		self.fTempPerm188 = 0.0;
		self.fTempPerm189 = 0.0;
		self.fTempPerm190 = 0.0;
		for l119 in 0..2 {
			self.fRec51[l119 as usize] = 0.0;
		}
		self.fTempPerm191 = 0.0;
		self.fTempPerm192 = 0.0;
		self.fTempPerm193 = 0.0;
		self.fTempPerm194 = 0.0;
		for l120 in 0..2 {
			self.fRec52[l120 as usize] = 0.0;
		}
		self.fTempPerm195 = 0.0;
		self.fTempPerm196 = 0.0;
		self.fTempPerm197 = 0.0;
		self.fTempPerm198 = 0.0;
		for l121 in 0..2 {
			self.fRec53[l121 as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec54[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec55[l123 as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.fRec56[l124 as usize] = 0.0;
		}
		for l125 in 0..2 {
			self.fRec57[l125 as usize] = 0.0;
		}
		self.fTempPerm199 = 0.0;
		self.fTempPerm200 = 0.0;
		for l126 in 0..2 {
			self.fRec58[l126 as usize] = 0.0;
		}
		self.fTempPerm201 = 0.0;
		self.fTempPerm202 = 0.0;
		self.fTempPerm203 = 0.0;
		self.fTempPerm204 = 0.0;
		self.fTempPerm205 = 0.0;
		self.fTempPerm206 = 0.0;
		self.fTempPerm207 = 0.0;
		self.fTempPerm208 = 0.0;
		self.fTempPerm209 = 0.0;
		self.fTempPerm210 = 0.0;
		self.fTempPerm211 = 0.0;
		self.fTempPerm212 = 0.0;
		self.fTempPerm213 = 0.0;
		self.fTempPerm214 = 0.0;
		self.fTempPerm215 = 0.0;
		self.fTempPerm216 = 0.0;
		self.fTempPerm217 = 0.0;
		self.fTempPerm218 = 0.0;
		self.fTempPerm219 = 0.0;
		self.fTempPerm220 = 0.0;
		self.fTempPerm221 = 0.0;
		self.fTempPerm222 = 0.0;
		self.fTempPerm223 = 0.0;
		self.fTempPerm224 = 0.0;
		for l127 in 0..2 {
			self.fRec100[l127 as usize] = 0.0;
		}
		self.fTempPerm225 = 0.0;
		self.fTempPerm226 = 0.0;
		for l128 in 0..2 {
			self.fRec101[l128 as usize] = 0.0;
		}
		self.fTempPerm227 = 0.0;
		self.fTempPerm228 = 0.0;
		for l129 in 0..2 {
			self.fRec102[l129 as usize] = 0.0;
		}
		self.fTempPerm229 = 0.0;
		self.fTempPerm230 = 0.0;
		for l130 in 0..2 {
			self.fRec103[l130 as usize] = 0.0;
		}
		self.fTempPerm231 = 0.0;
		self.fTempPerm232 = 0.0;
		for l131 in 0..2 {
			self.fRec104[l131 as usize] = 0.0;
		}
		self.fTempPerm233 = 0.0;
		self.fTempPerm234 = 0.0;
		self.fTempPerm235 = 0.0;
		for l132 in 0..2 {
			self.fRec105[l132 as usize] = 0.0;
		}
		self.fTempPerm236 = 0.0;
		self.fTempPerm237 = 0.0;
		self.fTempPerm238 = 0.0;
		for l133 in 0..2 {
			self.fRec106[l133 as usize] = 0.0;
		}
		self.fTempPerm239 = 0.0;
		self.fTempPerm240 = 0.0;
		self.fTempPerm241 = 0.0;
		for l134 in 0..2 {
			self.fRec107[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec108[l135 as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec109[l136 as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec110[l137 as usize] = 0.0;
		}
		self.fTempPerm242 = 0.0;
		self.fTempPerm243 = 0.0;
		for l138 in 0..2 {
			self.fRec111[l138 as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec112[l139 as usize] = 0.0;
		}
		self.fTempPerm244 = 0.0;
		for l140 in 0..2 {
			self.fVec29[l140 as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec114[l141 as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec115[l142 as usize] = 0.0;
		}
		self.fTempPerm245 = 0.0;
		self.fTempPerm246 = 0.0;
		self.fTempPerm247 = 0.0;
		self.fTempPerm248 = 0.0;
		self.fTempPerm249 = 0.0;
		self.fTempPerm250 = 0.0;
		self.fTempPerm251 = 0.0;
		self.fTempPerm252 = 0.0;
		for l143 in 0..2 {
			self.fRec116[l143 as usize] = 0.0;
		}
		self.fTempPerm253 = 0.0;
		self.fTempPerm254 = 0.0;
		for l144 in 0..2 {
			self.fRec117[l144 as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fVec30[l145 as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec121[l146 as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec122[l147 as usize] = 0.0;
		}
		self.fTempPerm255 = 0.0;
		self.iTempPerm256 = 0;
		self.fTempPerm257 = 0.0;
		self.fTempPerm258 = 0.0;
		self.fTempPerm259 = 0.0;
		self.fTempPerm260 = 0.0;
		self.fTempPerm261 = 0.0;
		self.fTempPerm262 = 0.0;
		self.fTempPerm263 = 0.0;
		self.fTempPerm264 = 0.0;
		self.fTempPerm265 = 0.0;
		self.fTempPerm266 = 0.0;
		self.fTempPerm267 = 0.0;
		for l148 in 0..2 {
			self.fRec123[l148 as usize] = 0.0;
		}
		self.fTempPerm268 = 0.0;
		self.fTempPerm269 = 0.0;
		for l149 in 0..2 {
			self.fRec124[l149 as usize] = 0.0;
		}
		self.fTempPerm270 = 0.0;
		self.fTempPerm271 = 0.0;
		self.fTempPerm272 = 0.0;
		self.fTempPerm273 = 0.0;
		for l150 in 0..2 {
			self.fVec31[l150 as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec42[l151 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 - 44.1 / fConst0;
		self.fConst2 = 1.0 / fConst0;
		self.fConst3 = F32::exp(-(157.07964 / fConst0));
		let mut fConst4: F32 = 1.0 - self.fConst3;
		self.fConst5 = 4.656613e-10 * F32::sqrt(3.0 * ((self.fConst3 + 1.0) / fConst4));
		self.fConst6 = 0.11626748 * fConst4;
		self.fConst7 = 4.41e+04 / fConst0;
		self.fConst8 = 5.0 / fConst0;
		self.fConst9 = 6.5 / fConst0;
		let mut fConst10: F32 = 376.99112 / fConst0;
		self.fConst11 = F32::exp(-fConst10);
		let mut fConst12: F32 = 1.0 - self.fConst11;
		self.fConst13 = 4.656613e-10 * F32::sqrt(3.0 * ((self.fConst11 + 1.0) / fConst12));
		self.fConst14 = 3.0 * fConst12;
		self.fConst15 = 3.1415927 / fConst0;
		self.fConst16 = 1.41 / fConst0;
		self.fConst17 = 1e+02 / fConst0;
		self.fConst18 = 125.0 / fConst0;
		self.fConst19 = 0.45 * fConst0;
		self.fConst20 = 666.6667 / fConst0;
		self.fConst21 = 15707.963 / fConst0;
		self.fConst22 = 2.0 / fConst0;
		let mut fConst23: F32 = 1.0 / F32::tan(18849.557 / fConst0);
		self.fConst24 = 1.0 - fConst23;
		self.fConst25 = 1.0 / (fConst23 + 1.0);
		self.fConst26 = 1.0 / F32::tan(942.4778 / fConst0);
		self.fConst27 = 1.0 - self.fConst26;
		self.fConst28 = 1.0 / (self.fConst26 + 1.0);
		self.fConst29 = 1.0 / F32::tan(3.1415927 * (F32::min(2e+03, 0.4 * fConst0) / fConst0));
		self.fConst30 = 1.0 - self.fConst29;
		self.fConst31 = 1.0 / (self.fConst29 + 1.0);
		let mut fConst32: F32 = 1.0 / F32::tan(1570.7964 / fConst0);
		self.fConst33 = 1.0 - fConst32;
		self.fConst34 = 5e+02 / fConst0;
		self.fConst35 = 1.0 / (fConst32 + 1.0);
		self.fConst36 = F32::tanh(0.12);
		let mut fConst37: F32 = 1.0 / F32::tan(fConst10);
		self.fConst38 = 1.0 - fConst37;
		self.fConst39 = 1.0 / (fConst37 + 1.0);
		self.fConst40 = 0.6666667 / (1.0 - VoiceDsp_faustpower2_f(self.fConst36));
		self.fConst41 = F32::tanh(-0.08);
		let mut fConst42: F32 = 1.0 / F32::tan(12566.371 / fConst0);
		self.fConst43 = 1.0 - fConst42;
		self.fConst44 = 1.0 / (fConst42 + 1.0);
		self.fConst45 = 0.8 / (1.0 - VoiceDsp_faustpower2_f(self.fConst41));
		self.fConst46 = F32::tanh(0.05);
		self.fConst47 = 1.1111112 / (1.0 - VoiceDsp_faustpower2_f(self.fConst46));
		self.fConst48 = F32::tanh(0.08);
		self.fConst49 = 0.6 / (1.0 - VoiceDsp_faustpower2_f(self.fConst48));
		let mut fConst50: F32 = 15.707963 / fConst0;
		self.fConst51 = 1.0 - fConst50;
		self.fConst52 = 1.0 / (fConst50 + 1.0);
		self.fConst53 = F32::tanh(0.15);
		self.fConst54 = 0.5882353 / (1.0 - VoiceDsp_faustpower2_f(self.fConst53));
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
		ui_interface.declare(Some(ParamIndex(11)), "6", "");
		ui_interface.add_horizontal_slider("Move", ParamIndex(11), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(12)), "7", "");
		ui_interface.declare(Some(ParamIndex(12)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(12)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(12), 1.5, 0.1, 8.0, 0.01);
		ui_interface.declare(Some(ParamIndex(13)), "8", "");
		ui_interface.add_check_button("Mute", ParamIndex(13));
		ui_interface.close_box();
		ui_interface.declare(None, "5", "");
		ui_interface.open_horizontal_box("Transient");
		ui_interface.declare(Some(ParamIndex(14)), "1", "");
		ui_interface.declare(Some(ParamIndex(14)), "style", "menu{'Click':0;'Burst':1;'Zap':2;'Metal':3;'Chiff':4;'Jade 01':5;'Jade 02':6;'Jade 03':7;'Jade 04':8;'Jade 05':9;'Jade 06':10;'Jade 07':11;'Jade 08':12;'Jade 09':13;'Jade 10':14;'Jade 11':15;'Jade 12':16;'Jade 13':17;'Jade 14':18;'Jade 15':19;'Jade 16':20;'Jade 17':21;'Jade 18':22;'Jade 19':23;'Jade 20':24;'Jade 21':25;'Jade 22':26;'Jade 23':27;'Jade 24':28;'Jade 25':29;'Jade 26':30;'Jade 27':31;'Jade 28':32;'Jade 29':33;'Jade 30':34;'Jade 31':35;'Jade 32':36;'Jade 33':37;'Jade 34':38;'Jade 35':39;'Jade 36':40;'Jade 37':41;'Jade 38':42;'Jade 39':43;'Jade 40':44;'Jade 41':45;'Jade 42':46;'Jade 43':47;'Jade 44':48;'Jade 45':49;'Jade 46':50;'Jade 47':51;'Jade 48':52;'Jade 49':53;'Jade 50':54;'Jade 51':55;'Jade 52':56;'Jade 53':57;'Jade 54':58;'Jade 55':59;'Jade 56':60;'Jade 57':61;'Jade 58':62;'Jade 59':63;'Jade 60':64;'Jade 61':65;'Jade 62':66;'Jade 63':67;'Jade 64':68;'Jade 65':69;'Jade 66':70;'Jade 67':71;'Jade 68':72;'Jade 69':73;'Jade 70':74;'Jade 71':75;'Jade 72':76;'Jade 73':77;'Jade 74':78;'Jade 75':79;'Jade 76':80;'Jade 77':81;'Jade 78':82;'Jade 79':83;'Jade 80':84;'Jade 81':85;'Jade 82':86;'Jade 83':87;'Jade 84':88;'Jade 85':89;'Jade 86':90;'Jade 87':91;'Jade 88':92;'Jade 89':93;'Jade 90':94;'Jade 91':95;'Jade 92':96;'Jade 93':97;'Jade 94':98;'Jade 95':99;'Mech 01':100;'Mech 02':101;'Mech 03':102;'Mech 04':103;'Mech 05':104;'Mech 06':105;'Mech 07':106;'Mech 08':107;'Mech 09':108;'Mech 10':109;'Mech 11':110;'Mech 12':111;'Mech 13':112;'Mech 14':113;'Mech 15':114;'Mech 16':115}");
		ui_interface.add_num_entry("Type", ParamIndex(14), 0.0, 0.0, 115.0, 1.0);
		ui_interface.declare(Some(ParamIndex(15)), "2", "");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(15), 6e+01, 5.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(16)), "3", "");
		ui_interface.add_horizontal_slider("Level", ParamIndex(16), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(17)), "4", "");
		ui_interface.add_check_button("Post-Filter", ParamIndex(17));
		ui_interface.declare(Some(ParamIndex(18)), "5", "");
		ui_interface.declare(Some(ParamIndex(18)), "unit", "st");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(18), 0.0, -24.0, 24.0, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_vertical_slider("Osc 1", ParamIndex(19), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(20)), "2", "");
		ui_interface.add_vertical_slider("Osc 2", ParamIndex(20), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(21)), "3", "");
		ui_interface.add_vertical_slider("Noise", ParamIndex(21), 0.15, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "7", "");
		ui_interface.open_horizontal_box("Drive");
		ui_interface.declare(Some(ParamIndex(22)), "2", "");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(22), 0.1, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("Filter");
		ui_interface.declare(Some(ParamIndex(23)), "0", "");
		ui_interface.declare(Some(ParamIndex(23)), "style", "menu{'SVF':0;'Ladder':1;'SH-101':2}");
		ui_interface.add_num_entry("Type", ParamIndex(23), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(24)), "1", "");
		ui_interface.declare(Some(ParamIndex(24)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(24)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Cutoff", ParamIndex(24), 1.2e+03, 2e+01, 1.8e+04, 1.0);
		ui_interface.declare(Some(ParamIndex(25)), "2", "");
		ui_interface.add_horizontal_slider("Resonance", ParamIndex(25), 0.8, 0.5, 6.0, 0.01);
		ui_interface.declare(Some(ParamIndex(26)), "3", "");
		ui_interface.add_horizontal_slider("Morph LP-Notch-HP", ParamIndex(26), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(27)), "4", "");
		ui_interface.add_horizontal_slider("Bandpass", ParamIndex(27), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(28)), "5", "");
		ui_interface.add_check_button("4-Pole", ParamIndex(28));
		ui_interface.declare(Some(ParamIndex(29)), "8", "");
		ui_interface.declare(Some(ParamIndex(29)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(29)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Utility HPF", ParamIndex(29), 2e+01, 1e+01, 2e+03, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("Amp");
		ui_interface.declare(Some(ParamIndex(30)), "1", "");
		ui_interface.declare(Some(ParamIndex(30)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(30)), "unit", "s");
		ui_interface.add_vertical_slider("Attack", ParamIndex(30), 0.005, 0.005, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "2", "");
		ui_interface.declare(Some(ParamIndex(31)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(31)), "unit", "s");
		ui_interface.add_vertical_slider("Decay", ParamIndex(31), 0.4, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(32)), "3", "");
		ui_interface.add_vertical_slider("Sustain", ParamIndex(32), 0.6, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(33)), "4", "");
		ui_interface.declare(Some(ParamIndex(33)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(33)), "unit", "s");
		ui_interface.add_vertical_slider("Release", ParamIndex(33), 0.5, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(34)), "5", "");
		ui_interface.add_vertical_slider("Attack Curve", ParamIndex(34), 0.6, -1.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(35)), "6", "");
		ui_interface.add_vertical_slider("Decay Curve", ParamIndex(35), -0.6, -1.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(36)), "7", "");
		ui_interface.add_vertical_slider("Release Curve", ParamIndex(36), -0.6, -1.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(37)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(37)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(37), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("freq", ParamIndex(38), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("gain", ParamIndex(39), 0.8, 0.0, 1.0, 0.01);
		ui_interface.add_button("gate", ParamIndex(40));
		ui_interface.add_horizontal_slider("glide", ParamIndex(41), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("ramp", ParamIndex(42), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("slot", ParamIndex(43), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			40 => Some(self.fButton0),
			13 => Some(self.fCheckbox0),
			17 => Some(self.fCheckbox1),
			28 => Some(self.fCheckbox2),
			23 => Some(self.fEntry0),
			14 => Some(self.fEntry1),
			6 => Some(self.fEntry2),
			2 => Some(self.fEntry3),
			0 => Some(self.fHslider0),
			42 => Some(self.fHslider1),
			1 => Some(self.fHslider10),
			41 => Some(self.fHslider11),
			38 => Some(self.fHslider12),
			29 => Some(self.fHslider13),
			5 => Some(self.fHslider14),
			24 => Some(self.fHslider15),
			25 => Some(self.fHslider16),
			12 => Some(self.fHslider17),
			11 => Some(self.fHslider18),
			10 => Some(self.fHslider19),
			22 => Some(self.fHslider2),
			9 => Some(self.fHslider20),
			7 => Some(self.fHslider21),
			8 => Some(self.fHslider22),
			27 => Some(self.fHslider23),
			26 => Some(self.fHslider24),
			39 => Some(self.fHslider25),
			3 => Some(self.fHslider3),
			16 => Some(self.fHslider4),
			15 => Some(self.fHslider5),
			43 => Some(self.fHslider6),
			4 => Some(self.fHslider7),
			18 => Some(self.fHslider8),
			37 => Some(self.fHslider9),
			21 => Some(self.fVslider0),
			36 => Some(self.fVslider1),
			33 => Some(self.fVslider2),
			30 => Some(self.fVslider3),
			32 => Some(self.fVslider4),
			35 => Some(self.fVslider5),
			31 => Some(self.fVslider6),
			34 => Some(self.fVslider7),
			20 => Some(self.fVslider8),
			19 => Some(self.fVslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			40 => { self.fButton0 = value }
			13 => { self.fCheckbox0 = value }
			17 => { self.fCheckbox1 = value }
			28 => { self.fCheckbox2 = value }
			23 => { self.fEntry0 = value }
			14 => { self.fEntry1 = value }
			6 => { self.fEntry2 = value }
			2 => { self.fEntry3 = value }
			0 => { self.fHslider0 = value }
			42 => { self.fHslider1 = value }
			1 => { self.fHslider10 = value }
			41 => { self.fHslider11 = value }
			38 => { self.fHslider12 = value }
			29 => { self.fHslider13 = value }
			5 => { self.fHslider14 = value }
			24 => { self.fHslider15 = value }
			25 => { self.fHslider16 = value }
			12 => { self.fHslider17 = value }
			11 => { self.fHslider18 = value }
			10 => { self.fHslider19 = value }
			22 => { self.fHslider2 = value }
			9 => { self.fHslider20 = value }
			7 => { self.fHslider21 = value }
			8 => { self.fHslider22 = value }
			27 => { self.fHslider23 = value }
			26 => { self.fHslider24 = value }
			39 => { self.fHslider25 = value }
			3 => { self.fHslider3 = value }
			16 => { self.fHslider4 = value }
			15 => { self.fHslider5 = value }
			43 => { self.fHslider6 = value }
			4 => { self.fHslider7 = value }
			18 => { self.fHslider8 = value }
			37 => { self.fHslider9 = value }
			21 => { self.fVslider0 = value }
			36 => { self.fVslider1 = value }
			33 => { self.fVslider2 = value }
			30 => { self.fVslider3 = value }
			32 => { self.fVslider4 = value }
			35 => { self.fVslider5 = value }
			31 => { self.fVslider6 = value }
			34 => { self.fVslider7 = value }
			20 => { self.fVslider8 = value }
			19 => { self.fVslider9 = value }
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
		let (outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7) = if let [outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			let outputs3 = outputs3[..count as usize].iter_mut();
			let outputs4 = outputs4[..count as usize].iter_mut();
			let outputs5 = outputs5[..count as usize].iter_mut();
			let outputs6 = outputs6[..count as usize].iter_mut();
			let outputs7 = outputs7[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = self.fHslider0;
		let mut fSlow2: F32 = self.fHslider1;
		let mut fSlow3: F32 = self.fEntry0;
		let mut iSlow4: i32 = (fSlow3 == 2.0) as i32;
		let mut fSlow5: F32 = (iSlow4) as F32;
		let mut iSlow6: i32 = (fSlow5 != 0.0) as i32;
		let mut iSlow7: i32 = (fSlow3 == 1.0) as i32;
		let mut fSlow8: F32 = (iSlow7) as F32;
		let mut iSlow9: i32 = (fSlow8 != 0.0) as i32;
		let mut iSlow10: i32 = (fSlow3 == 0.0) as i32;
		let mut fSlow11: F32 = (iSlow10) as F32;
		let mut iSlow12: i32 = (fSlow11 != 0.0) as i32;
		let mut fSlow13: F32 = 1.0 - self.fCheckbox0;
		let mut fTemp9: F32 = self.fTempPerm0;
		let mut fSlow14: F32 = self.fVslider0;
		let mut iSlow15: i32 = (fSlow14 != 0.0) as i32;
		let mut iTemp10: i32 = self.iTempPerm1;
		let mut fTemp11: F32 = self.fTempPerm2;
		let mut fSlow16: F32 = self.fHslider2;
		let mut fTemp12: F32 = self.fTempPerm3;
		let mut fTemp13: F32 = self.fTempPerm4;
		let mut fTemp14: F32 = self.fTempPerm5;
		let mut fTemp15: F32 = self.fTempPerm6;
		let mut fTemp16: F32 = self.fTempPerm7;
		let mut iTemp17: i32 = self.iTempPerm8;
		let mut fTemp18: F32 = self.fTempPerm9;
		let mut iTemp19: i32 = self.iTempPerm10;
		let mut iTemp20: i32 = self.iTempPerm11;
		let mut fTemp21: F32 = self.fTempPerm12;
		let mut iTemp22: i32 = self.iTempPerm13;
		let mut iTemp23: i32 = self.iTempPerm14;
		let mut fTemp24: F32 = self.fTempPerm15;
		let mut iTemp25: i32 = self.iTempPerm16;
		let mut iTemp26: i32 = self.iTempPerm17;
		let mut fTemp27: F32 = self.fTempPerm18;
		let mut iTemp28: i32 = self.iTempPerm19;
		let mut iTemp29: i32 = self.iTempPerm20;
		let mut fTemp30: F32 = self.fTempPerm21;
		let mut iTemp31: i32 = self.iTempPerm22;
		let mut iTemp32: i32 = self.iTempPerm23;
		let mut fTemp33: F32 = self.fTempPerm24;
		let mut iTemp34: i32 = self.iTempPerm25;
		let mut iTemp35: i32 = self.iTempPerm26;
		let mut fTemp36: F32 = self.fTempPerm27;
		let mut iTemp37: i32 = self.iTempPerm28;
		let mut iTemp38: i32 = self.iTempPerm29;
		let mut fTemp39: F32 = self.fTempPerm30;
		let mut iTemp40: i32 = self.iTempPerm31;
		let mut fSlow17: F32 = self.fHslider3;
		let mut iTemp41: i32 = self.iTempPerm32;
		let mut fTemp42: F32 = self.fTempPerm33;
		let mut fSlow18: F32 = self.fHslider4;
		let mut iSlow19: i32 = (fSlow18 != 0.0) as i32;
		let mut fSlow20: F32 = (iSlow19) as F32;
		let mut iSlow21: i32 = (fSlow20 != 0.0) as i32;
		let mut fSlow22: F32 = self.fHslider5;
		let mut fTemp43: F32 = self.fTempPerm34;
		let mut fTemp44: F32 = self.fTempPerm35;
		let mut fTemp45: F32 = self.fTempPerm36;
		let mut fTemp46: F32 = self.fTempPerm37;
		let mut iTemp47: i32 = self.iTempPerm38;
		let mut fTemp48: F32 = self.fTempPerm39;
		let mut fSlow23: F32 = self.fEntry1;
		let mut fSlow24: F32 = F32::min(fSlow23, 4.0);
		let mut iSlow25: i32 = (fSlow24 == 4.0) as i32;
		let mut fSlow26: F32 = (iSlow25) as F32;
		let mut iSlow27: i32 = (fSlow26 != 0.0) as i32;
		let mut iSlow28: i32 = (fSlow23 >= 5.0) as i32;
		let mut iSlow29: i32 = (iSlow28 == 0) as i32;
		let mut fSlow30: F32 = (iSlow29) as F32;
		let mut iSlow31: i32 = (fSlow30 != 0.0) as i32;
		let mut iSlow32: i32 = (fSlow24 == 3.0) as i32;
		let mut fSlow33: F32 = (iSlow32) as F32;
		let mut iSlow34: i32 = (fSlow33 != 0.0) as i32;
		let mut iSlow35: i32 = (fSlow24 == 2.0) as i32;
		let mut fSlow36: F32 = (iSlow35) as F32;
		let mut iSlow37: i32 = (fSlow36 != 0.0) as i32;
		let mut iTemp5: i32 = self.iTempPerm40;
		let mut fSlow38: F32 = self.fHslider6;
		let mut iSlow39: i32 = (521.0 * fSlow38 + 5e+03) as i32;
		let mut iTemp49: i32 = self.iTempPerm41;
		let mut fSlow40: F32 = self.fHslider7;
		let mut fTemp50: F32 = self.fTempPerm42;
		let mut iSlow41: i32 = (iSlow28 == 1) as i32;
		let mut fSlow42: F32 = (iSlow41) as F32;
		let mut iSlow43: i32 = (fSlow42 != 0.0) as i32;
		let mut iSlow44: i32 = (F32::max(0.0, fSlow23 + -5.0)) as i32;
		let mut iSlow45: i32 = (fSlow24 == 1.0) as i32;
		let mut fSlow46: F32 = (iSlow45) as F32;
		let mut iSlow47: i32 = (fSlow46 != 0.0) as i32;
		let mut iSlow48: i32 = (fSlow24 == 0.0) as i32;
		let mut fSlow49: F32 = (iSlow48) as F32;
		let mut iSlow50: i32 = (fSlow49 != 0.0) as i32;
		let mut fSlow51: F32 = self.fHslider8;
		let mut fTemp56: F32 = self.fTempPerm43;
		let mut fTemp57: F32 = self.fTempPerm44;
		let mut fTemp58: F32 = self.fTempPerm45;
		let mut fTemp59: F32 = self.fTempPerm46;
		let mut fSlow52: F32 = ((iSlow41 != 0) as i32) as u32 as F32;
		let mut fTemp60: F32 = self.fTempPerm47;
		let mut fSlow53: F32 = 12.9898 * fSlow38;
		let mut fSlow54: F32 = 43758.547 * F32::sin(fSlow53 + 234.699);
		let mut fSlow55: F32 = 43758.547 * F32::sin(fSlow53 + 7979.766);
		let mut fSlow56: F32 = 2.0 * (fSlow55 - F32::floor(fSlow55)) + -1.0;
		let mut fSlow57: F32 = self.fConst2 * (0.1 * F32::max(-1.0, F32::min(1.0, 0.70710677 * (fSlow56 + 2.0 * (fSlow54 - F32::floor(fSlow54)) + -1.0))) + 0.45);
		let mut fTemp61: F32 = self.fTempPerm48;
		let mut iSlow58: i32 = (409.0 * fSlow38 + 4e+03) as i32;
		let mut iTemp62: i32 = self.iTempPerm49;
		let mut fSlow59: F32 = 43758.547 * F32::sin(fSlow53 + 156.466);
		let mut fSlow60: F32 = 0.6 * fSlow56;
		let mut fSlow61: F32 = 0.15 * F32::max(-1.0, F32::min(1.0, 1.2803688 * (fSlow60 + 0.5 * (2.0 * (fSlow59 - F32::floor(fSlow59)) + -1.0)))) + 0.25;
		let mut fSlow62: F32 = 1.0 / F32::tan(self.fConst15 * F32::max(1.0, fSlow61));
		let mut fSlow63: F32 = 1.0 - fSlow62;
		let mut fSlow64: F32 = self.fConst2 * F32::max(fSlow61, 0.0);
		let mut fTemp63: F32 = self.fTempPerm50;
		let mut fSlow65: F32 = self.fEntry2;
		let mut iSlow66: i32 = (fSlow65 == 4.0) as i32;
		let mut fSlow67: F32 = (iSlow66) as F32;
		let mut iSlow68: i32 = (fSlow67 != 0.0) as i32;
		let mut iSlow69: i32 = (fSlow65 == 1.0) as i32;
		let mut fSlow70: F32 = (iSlow69) as F32;
		let mut iSlow71: i32 = (fSlow70 != 0.0) as i32;
		let mut iSlow72: i32 = (fSlow65 == 0.0) as i32;
		let mut fSlow73: F32 = (iSlow72) as F32;
		let mut iSlow74: i32 = (fSlow73 != 0.0) as i32;
		let mut fTemp64: F32 = self.fTempPerm51;
		let mut fTemp65: F32 = self.fTempPerm52;
		let mut fTemp66: F32 = self.fTempPerm53;
		let mut fSlow75: F32 = 1.0 / (fSlow62 + 1.0);
		let mut fSlow76: F32 = 43758.547 * F32::sin(fSlow53 + 78.233);
		let mut fSlow77: F32 = 43758.547 * F32::sin(fSlow53 + 7901.533);
		let mut fSlow78: F32 = 2.0 * (fSlow77 - F32::floor(fSlow77)) + -1.0;
		let mut fSlow79: F32 = 0.3 * fSlow56;
		let mut fSlow80: F32 = 8.0 * F32::max(-1.0, F32::min(1.0, 1.0976425 * (fSlow79 + 0.7 * fSlow78 + 0.5 * (2.0 * (fSlow76 - F32::floor(fSlow76)) + -1.0))));
		let mut fTemp67: F32 = self.fTempPerm54;
		let mut fSlow81: F32 = self.fHslider9;
		let mut fSlow82: F32 = self.fEntry3;
		let mut fSlow83: F32 = 2.0 * ((fSlow82 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow82 == 1.0) as i32) as u32 as F32;
		let mut fSlow84: F32 = self.fHslider10;
		let mut fSlow85: F32 = 0.33333334 * F32::max(1e-05, self.fHslider11);
		let mut iSlow86: i32 = (F32::abs(fSlow85) < 1.1920929e-07) as i32;
		let mut fSlow87: F32 = if iSlow86 != 0 {0.0} else {F32::exp(-(self.fConst2 / if iSlow86 != 0 {1.0} else {fSlow85}))};
		let mut fSlow88: F32 = F32::log(self.fHslider12, std::f32::consts::E) * (1.0 - fSlow87);
		let mut fTemp71: F32 = self.fTempPerm55;
		let mut fTemp72: F32 = self.fTempPerm56;
		let mut fSlow89: F32 = 0.03125 * fSlow26;
		let mut fTemp73: F32 = self.fTempPerm57;
		let mut fSlow90: F32 = ((iSlow25 != 0) as i32) as u32 as F32;
		let mut fTemp74: F32 = self.fTempPerm58;
		let mut fTemp75: F32 = self.fTempPerm59;
		let mut fTemp76: F32 = self.fTempPerm60;
		let mut fSlow91: F32 = ((iSlow32 != 0) as i32) as u32 as F32;
		let mut fTemp77: F32 = self.fTempPerm61;
		let mut fTemp78: F32 = self.fTempPerm62;
		let mut fSlow92: F32 = ((iSlow35 != 0) as i32) as u32 as F32;
		let mut fTemp79: F32 = self.fTempPerm63;
		let mut fTemp80: F32 = self.fTempPerm64;
		let mut fTemp81: F32 = self.fTempPerm65;
		let mut fTemp82: F32 = self.fTempPerm66;
		let mut fTemp83: F32 = self.fTempPerm67;
		let mut fSlow93: F32 = ((iSlow45 != 0) as i32) as u32 as F32;
		let mut fTemp84: F32 = self.fTempPerm68;
		let mut fSlow94: F32 = ((iSlow48 != 0) as i32) as u32 as F32;
		let mut fTemp85: F32 = self.fTempPerm69;
		let mut fSlow95: F32 = ((iSlow29 != 0) as i32) as u32 as F32;
		let mut fTemp86: F32 = self.fTempPerm70;
		let mut fTemp87: F32 = self.fTempPerm71;
		let mut fTemp88: F32 = self.fTempPerm72;
		let mut fSlow96: F32 = ((iSlow19 != 0) as i32) as u32 as F32;
		let mut fSlow97: F32 = self.fCheckbox1;
		let mut fSlow98: F32 = fSlow97 * fSlow20 * fSlow96;
		let mut iSlow99: i32 = (fSlow0) as i32;
		let mut fSlow100: F32 = self.fVslider1;
		let mut fSlow101: F32 = 0.4 * fSlow78;
		let mut fSlow102: F32 = 43758.547 * F32::sin(fSlow53 + 625.864);
		let mut fSlow103: F32 = 0.25 * F32::max(-1.0, F32::min(1.0, 1.230915 * (0.5 * (fSlow56 + 2.0 * (fSlow102 - F32::floor(fSlow102)) + -1.0) - fSlow101)));
		let mut fSlow104: F32 = self.fVslider2;
		let mut fSlow105: F32 = 1.0 - fSlow0;
		let mut fSlow106: F32 = 43758.547 * F32::sin(fSlow53 + 469.398);
		let mut fSlow107: F32 = 0.25 * F32::max(-1.0, F32::min(1.0, 1.1952286 * (fSlow60 + 0.5 * (2.0 * (fSlow106 - F32::floor(fSlow106)) + -1.0) - 0.3 * fSlow78)));
		let mut fSlow108: F32 = self.fVslider3;
		let mut fSlow109: F32 = self.fVslider4;
		let mut fSlow110: F32 = self.fVslider5;
		let mut fSlow111: F32 = if (fSlow109 >= 1.0) as i32 != 0 {fSlow110} else {-fSlow110};
		let mut fSlow112: F32 = 5.0 * fSlow111;
		let mut iSlow113: i32 = (F32::abs(fSlow112) < 0.001) as i32;
		let mut fSlow114: F32 = 1.0 - F32::exp(-fSlow112);
		let mut fSlow115: F32 = 43758.547 * F32::sin(fSlow53 + 547.631);
		let mut fSlow116: F32 = 0.25 * F32::max(-1.0, F32::min(1.0, 1.230915 * (0.5 * (fSlow56 + 2.0 * (fSlow115 - F32::floor(fSlow115)) + -1.0) - fSlow101)));
		let mut fSlow117: F32 = self.fVslider6;
		let mut fSlow118: F32 = fSlow109 + -1.0;
		let mut fSlow119: F32 = self.fVslider7;
		let mut fSlow120: F32 = self.fHslider13;
		let mut fSlow121: F32 = self.fHslider14;
		let mut fTemp104: F32 = self.fTempPerm73;
		let mut fSlow122: F32 = 0.5 * ((fSlow82 == 2.0) as i32) as u32 as F32;
		let mut fSlow123: F32 = 43758.547 * F32::sin(fSlow53 + 391.165);
		let mut fSlow124: F32 = 0.1 * F32::max(-1.0, F32::min(1.0, 1.7149858 * (fSlow79 + 0.5 * (2.0 * (fSlow123 - F32::floor(fSlow123)) + -1.0)))) + 0.2;
		let mut fSlow125: F32 = 1.0 / F32::tan(self.fConst15 * F32::max(1.0, fSlow124));
		let mut fSlow126: F32 = 1.0 - fSlow125;
		let mut fSlow127: F32 = self.fConst2 * F32::max(fSlow124, 0.0);
		let mut fTemp105: F32 = self.fTempPerm74;
		let mut fTemp106: F32 = self.fTempPerm75;
		let mut fSlow128: F32 = 1.0 / (fSlow125 + 1.0);
		let mut fSlow129: F32 = 43758.547 * F32::sin(fSlow53 + 312.932);
		let mut fSlow130: F32 = 0.25 * F32::max(-1.0, F32::min(1.0, 1.1396058 * (0.6 * fSlow78 + 0.5 * (2.0 * (fSlow129 - F32::floor(fSlow129)) + -1.0) - 0.4 * fSlow56)));
		let mut fTemp107: F32 = self.fTempPerm76;
		let mut fSlow131: F32 = self.fHslider15;
		let mut fTemp110: F32 = self.fTempPerm77;
		let mut fTemp111: F32 = self.fTempPerm78;
		let mut fTemp112: F32 = self.fTempPerm79;
		let mut fTemp113: F32 = self.fTempPerm80;
		let mut fTemp114: F32 = self.fTempPerm81;
		let mut fTemp115: F32 = self.fTempPerm82;
		let mut fTemp116: F32 = self.fTempPerm83;
		let mut fTemp117: F32 = self.fTempPerm84;
		let mut fTemp118: F32 = self.fTempPerm85;
		let mut fTemp119: F32 = self.fTempPerm86;
		let mut fTemp120: F32 = self.fTempPerm87;
		let mut fTemp121: F32 = self.fTempPerm88;
		let mut fTemp122: F32 = self.fTempPerm89;
		let mut fTemp123: F32 = self.fTempPerm90;
		let mut fTemp124: F32 = self.fTempPerm91;
		let mut fSlow132: F32 = 0.5 * ((fSlow82 == 3.0) as i32) as u32 as F32;
		let mut fSlow133: F32 = self.fHslider16;
		let mut fTemp127: F32 = self.fTempPerm92;
		let mut fTemp128: F32 = self.fTempPerm93;
		let mut fTemp129: F32 = self.fTempPerm94;
		let mut fTemp130: F32 = self.fTempPerm95;
		let mut fTemp131: F32 = self.fTempPerm96;
		let mut fSlow134: F32 = (iSlow15) as F32;
		let mut fSlow135: F32 = VoiceDsp_faustpower2_f(fSlow134);
		let mut fSlow136: F32 = self.fVslider8;
		let mut fTemp132: F32 = self.fTempPerm97;
		let mut fSlow137: F32 = ((fSlow136 != 0.0) as i32) as u32 as F32;
		let mut fSlow138: F32 = VoiceDsp_faustpower2_f(fSlow137);
		let mut fSlow139: F32 = self.fVslider9;
		let mut fTemp133: F32 = self.fTempPerm98;
		let mut fSlow140: F32 = ((fSlow139 != 0.0) as i32) as u32 as F32;
		let mut fSlow141: F32 = VoiceDsp_faustpower2_f(fSlow140);
		let mut fSlow142: F32 = self.fHslider17;
		let mut fTemp134: F32 = self.fTempPerm99;
		let mut fTemp135: F32 = self.fTempPerm100;
		let mut fTemp136: F32 = self.fTempPerm101;
		let mut iSlow143: i32 = (613.0 * fSlow38 + 3e+03) as i32;
		let mut iTemp137: i32 = self.iTempPerm102;
		let mut fSlow144: F32 = self.fHslider18;
		let mut fTemp138: F32 = self.fTempPerm103;
		let mut fTemp139: F32 = self.fTempPerm104;
		let mut fTemp140: F32 = self.fTempPerm105;
		let mut iTemp141: i32 = self.iTempPerm106;
		let mut fTemp142: F32 = self.fTempPerm107;
		let mut fSlow145: F32 = self.fHslider19;
		let mut fTemp143: F32 = self.fTempPerm108;
		let mut fTemp144: F32 = self.fTempPerm109;
		let mut fSlow146: F32 = self.fHslider20;
		let mut fTemp145: F32 = self.fTempPerm110;
		let mut fTemp146: F32 = self.fTempPerm111;
		let mut fTemp147: F32 = self.fTempPerm112;
		let mut fSlow147: F32 = ((iSlow66 != 0) as i32) as u32 as F32;
		let mut iSlow148: i32 = (fSlow65 == 3.0) as i32;
		let mut fSlow149: F32 = (iSlow148) as F32;
		let mut iSlow150: i32 = (fSlow149 != 0.0) as i32;
		let mut iSlow151: i32 = (fSlow65 == 2.0) as i32;
		let mut fSlow152: F32 = (iSlow151) as F32;
		let mut iSlow153: i32 = (fSlow152 != 0.0) as i32;
		let mut fSlow154: F32 = F32::powf(2.0, self.fHslider21);
		let mut fTemp149: F32 = self.fTempPerm113;
		let mut iTemp150: i32 = self.iTempPerm114;
		let mut iTemp151: i32 = self.iTempPerm115;
		let mut fTemp152: F32 = self.fTempPerm116;
		let mut fSlow155: F32 = ((iSlow148 != 0) as i32) as u32 as F32;
		let mut iTemp153: i32 = self.iTempPerm117;
		let mut iTemp154: i32 = self.iTempPerm118;
		let mut fTemp155: F32 = self.fTempPerm119;
		let mut fSlow156: F32 = ((iSlow151 != 0) as i32) as u32 as F32;
		let mut fSlow157: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, self.fHslider22)) + -1.0));
		let mut fSlow158: F32 = fSlow70 / fSlow157;
		let mut fTemp156: F32 = self.fTempPerm120;
		let mut fSlow159: F32 = ((iSlow69 != 0) as i32) as u32 as F32;
		let mut fSlow160: F32 = 4.656613e-10 * fSlow73;
		let mut fTemp157: F32 = self.fTempPerm121;
		let mut fSlow161: F32 = ((iSlow72 != 0) as i32) as u32 as F32;
		let mut fTemp158: F32 = self.fTempPerm122;
		let mut fTemp159: F32 = self.fTempPerm123;
		let mut fTemp160: F32 = self.fTempPerm124;
		let mut fTemp161: F32 = self.fTempPerm125;
		let mut fTemp162: F32 = self.fTempPerm126;
		let mut fTemp163: F32 = self.fTempPerm127;
		let mut fTemp164: F32 = self.fTempPerm128;
		let mut fTemp165: F32 = self.fTempPerm129;
		let mut fTemp166: F32 = self.fTempPerm130;
		let mut fTemp167: F32 = self.fTempPerm131;
		let mut fSlow162: F32 = fSlow20 * fSlow96 * (1.0 - fSlow97);
		let mut fTemp168: F32 = self.fTempPerm132;
		let mut fTemp169: F32 = self.fTempPerm133;
		let mut fTemp172: F32 = self.fTempPerm134;
		let mut fTemp173: F32 = self.fTempPerm135;
		let mut fTemp174: F32 = self.fTempPerm136;
		let mut fTemp175: F32 = self.fTempPerm137;
		let mut fTemp176: F32 = self.fTempPerm138;
		let mut fTemp177: F32 = self.fTempPerm139;
		let mut fTemp179: F32 = self.fTempPerm140;
		let mut fTemp180: F32 = self.fTempPerm141;
		let mut fTemp181: F32 = self.fTempPerm142;
		let mut fTemp182: F32 = self.fTempPerm143;
		let mut fTemp183: F32 = self.fTempPerm144;
		let mut fTemp184: F32 = self.fTempPerm145;
		let mut fTemp185: F32 = self.fTempPerm146;
		let mut fTemp186: F32 = self.fTempPerm147;
		let mut fTemp187: F32 = self.fTempPerm148;
		let mut fTemp188: F32 = self.fTempPerm149;
		let mut fTemp189: F32 = self.fTempPerm150;
		let mut fTemp190: F32 = self.fTempPerm151;
		let mut fTemp191: F32 = self.fTempPerm152;
		let mut fTemp192: F32 = self.fTempPerm153;
		let mut fTemp193: F32 = self.fTempPerm154;
		let mut fTemp194: F32 = self.fTempPerm155;
		let mut fTemp195: F32 = self.fTempPerm156;
		let mut fTemp196: F32 = self.fTempPerm157;
		let mut fTemp197: F32 = self.fTempPerm158;
		let mut fTemp198: F32 = self.fTempPerm159;
		let mut fTemp199: F32 = self.fTempPerm160;
		let mut fTemp200: F32 = self.fTempPerm161;
		let mut fTemp201: F32 = self.fTempPerm162;
		let mut fTemp202: F32 = self.fTempPerm163;
		let mut fTemp203: F32 = self.fTempPerm164;
		let mut fTemp204: F32 = self.fTempPerm165;
		let mut fTemp205: F32 = self.fTempPerm166;
		let mut fTemp206: F32 = self.fTempPerm167;
		let mut fTemp207: F32 = self.fTempPerm168;
		let mut fTemp208: F32 = self.fTempPerm169;
		let mut fTemp209: F32 = self.fTempPerm170;
		let mut fTemp210: F32 = self.fTempPerm171;
		let mut fTemp211: F32 = self.fTempPerm172;
		let mut fTemp212: F32 = self.fTempPerm173;
		let mut fTemp213: F32 = self.fTempPerm174;
		let mut fTemp214: F32 = self.fTempPerm175;
		let mut fTemp215: F32 = self.fTempPerm176;
		let mut fTemp216: F32 = self.fTempPerm177;
		let mut fTemp217: F32 = self.fTempPerm178;
		let mut fTemp218: F32 = self.fTempPerm179;
		let mut fTemp219: F32 = self.fTempPerm180;
		let mut fTemp220: F32 = self.fTempPerm181;
		let mut fTemp221: F32 = self.fTempPerm182;
		let mut fTemp222: F32 = self.fTempPerm183;
		let mut fTemp223: F32 = self.fTempPerm184;
		let mut fTemp224: F32 = self.fTempPerm185;
		let mut fTemp225: F32 = self.fTempPerm186;
		let mut fTemp226: F32 = self.fTempPerm187;
		let mut fTemp227: F32 = self.fTempPerm188;
		let mut fTemp228: F32 = self.fTempPerm189;
		let mut fTemp229: F32 = self.fTempPerm190;
		let mut fTemp230: F32 = self.fTempPerm191;
		let mut fTemp231: F32 = self.fTempPerm192;
		let mut fTemp232: F32 = self.fTempPerm193;
		let mut fTemp233: F32 = self.fTempPerm194;
		let mut fTemp234: F32 = self.fTempPerm195;
		let mut fTemp235: F32 = self.fTempPerm196;
		let mut fTemp236: F32 = self.fTempPerm197;
		let mut fTemp237: F32 = self.fTempPerm198;
		let mut fTemp238: F32 = self.fTempPerm199;
		let mut fTemp239: F32 = self.fTempPerm200;
		let mut fSlow163: F32 = 2.0 * fSlow5;
		let mut fTemp240: F32 = self.fTempPerm201;
		let mut fSlow164: F32 = ((iSlow4 != 0) as i32) as u32 as F32;
		let mut fTemp241: F32 = self.fTempPerm202;
		let mut fTemp242: F32 = self.fTempPerm203;
		let mut fTemp243: F32 = self.fTempPerm204;
		let mut fTemp244: F32 = self.fTempPerm205;
		let mut fTemp245: F32 = self.fTempPerm206;
		let mut fTemp246: F32 = self.fTempPerm207;
		let mut fTemp247: F32 = self.fTempPerm208;
		let mut fTemp248: F32 = self.fTempPerm209;
		let mut fTemp249: F32 = self.fTempPerm210;
		let mut fTemp250: F32 = self.fTempPerm211;
		let mut fTemp251: F32 = self.fTempPerm212;
		let mut fTemp252: F32 = self.fTempPerm213;
		let mut fTemp253: F32 = self.fTempPerm214;
		let mut fTemp254: F32 = self.fTempPerm215;
		let mut fTemp255: F32 = self.fTempPerm216;
		let mut fTemp256: F32 = self.fTempPerm217;
		let mut fTemp257: F32 = self.fTempPerm218;
		let mut fTemp258: F32 = self.fTempPerm219;
		let mut fTemp259: F32 = self.fTempPerm220;
		let mut fTemp260: F32 = self.fTempPerm221;
		let mut fTemp261: F32 = self.fTempPerm222;
		let mut fTemp262: F32 = self.fTempPerm223;
		let mut fTemp263: F32 = self.fTempPerm224;
		let mut fTemp264: F32 = self.fTempPerm225;
		let mut fTemp265: F32 = self.fTempPerm226;
		let mut fTemp266: F32 = self.fTempPerm227;
		let mut fTemp267: F32 = self.fTempPerm228;
		let mut fTemp268: F32 = self.fTempPerm229;
		let mut fTemp269: F32 = self.fTempPerm230;
		let mut fTemp270: F32 = self.fTempPerm231;
		let mut fTemp271: F32 = self.fTempPerm232;
		let mut fTemp272: F32 = self.fTempPerm233;
		let mut fTemp273: F32 = self.fTempPerm234;
		let mut fTemp274: F32 = self.fTempPerm235;
		let mut fTemp275: F32 = self.fTempPerm236;
		let mut fTemp276: F32 = self.fTempPerm237;
		let mut fTemp277: F32 = self.fTempPerm238;
		let mut fTemp278: F32 = self.fTempPerm239;
		let mut fTemp279: F32 = self.fTempPerm240;
		let mut fTemp280: F32 = self.fTempPerm241;
		let mut fTemp281: F32 = self.fTempPerm242;
		let mut fTemp282: F32 = self.fTempPerm243;
		let mut fSlow165: F32 = 2.0 * fSlow8;
		let mut fTemp283: F32 = self.fTempPerm244;
		let mut fSlow166: F32 = ((iSlow7 != 0) as i32) as u32 as F32;
		let mut fSlow167: F32 = self.fCheckbox2;
		let mut iSlow168: i32 = (fSlow167) as i32;
		let mut fSlow169: F32 = 0.5 * ((fSlow82 == 4.0) as i32) as u32 as F32;
		let mut fSlow170: F32 = self.fHslider23;
		let mut fTemp286: F32 = self.fTempPerm245;
		let mut fTemp287: F32 = self.fTempPerm246;
		let mut fTemp288: F32 = self.fTempPerm247;
		let mut fTemp289: F32 = self.fTempPerm248;
		let mut fTemp290: F32 = self.fTempPerm249;
		let mut fTemp291: F32 = self.fTempPerm250;
		let mut fTemp292: F32 = self.fTempPerm251;
		let mut fTemp293: F32 = self.fTempPerm252;
		let mut fTemp294: F32 = self.fTempPerm253;
		let mut fTemp295: F32 = self.fTempPerm254;
		let mut fSlow171: F32 = self.fHslider24;
		let mut fTemp297: F32 = self.fTempPerm255;
		let mut iTemp298: i32 = self.iTempPerm256;
		let mut fTemp299: F32 = self.fTempPerm257;
		let mut fTemp300: F32 = self.fTempPerm258;
		let mut fTemp301: F32 = self.fTempPerm259;
		let mut fTemp302: F32 = self.fTempPerm260;
		let mut fTemp303: F32 = self.fTempPerm261;
		let mut fTemp304: F32 = self.fTempPerm262;
		let mut fTemp305: F32 = self.fTempPerm263;
		let mut iSlow172: i32 = (fSlow167 != 0.0) as i32;
		let mut fTemp306: F32 = self.fTempPerm264;
		let mut fTemp307: F32 = self.fTempPerm265;
		let mut fTemp308: F32 = self.fTempPerm266;
		let mut fTemp309: F32 = self.fTempPerm267;
		let mut fTemp310: F32 = self.fTempPerm268;
		let mut fTemp311: F32 = self.fTempPerm269;
		let mut fTemp312: F32 = self.fTempPerm270;
		let mut fTemp313: F32 = self.fTempPerm271;
		let mut fSlow173: F32 = (iSlow172) as F32;
		let mut fTemp314: F32 = self.fTempPerm272;
		let mut fSlow174: F32 = ((iSlow10 != 0) as i32) as u32 as F32;
		let mut fTemp316: F32 = self.fTempPerm273;
		let mut fSlow175: F32 = 0.5 * self.fHslider25;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7);
		for (((((((((input0, input1), output0), output1), output2), output3), output4), output5), output6), output7) in zipped_iterators {
			let mut fRec59: F32 = 0.0;
			let mut fRec113: F32 = 0.0;
			let mut fRec118: F32 = 0.0;
			let mut fRec119: F32 = 0.0;
			let mut fRec120: F32 = 0.0;
			let mut fRec125: F32 = 0.0;
			let mut fRec126: F32 = 0.0;
			let mut fRec127: F32 = 0.0;
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
			let mut iTemp6: i32 = i32::wrapping_sub(1, iTemp0);
			let mut fTemp7: F32 = (iTemp6) as F32;
			let mut fTemp8: F32 = 1.0 - self.fConst1 * fTemp7;
			if iSlow12 | (iSlow9 | iSlow6) != 0 {
				self.fRec4[0] = fSlow13 * fTemp8 + self.fConst1 * fTemp7 * self.fRec4[1];
				fTemp9 = ((self.fRec4[0] > 0.0001) as i32) as u32 as F32;
				self.fVec3[0] = fSlow14;
				iTemp10 = i32::wrapping_mul(iSlow15, ((self.fRec4[0] * fTemp9) != 0.0) as i32);
				fTemp11 = (iTemp10) as F32;
				self.fVec4[0] = fSlow16;
				fTemp12 = if ((fSlow16 != self.fVec4[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec5[1] + -1.0};
				self.fRec5[0] = fTemp12;
				self.fRec6[0] = if iTemp0 != 0 {fSlow16} else {if (fTemp12 > 0.0) as i32 != 0 {self.fRec6[1] + (fSlow16 - self.fRec6[1]) / F32::max(1.0, fTemp12)} else {fSlow16}};
				fTemp13 = F32::max(0.0, F32::min(1.0, self.fRec6[0]));
				fTemp14 = 4.0 * fTemp13;
				fTemp15 = F32::max(0.0, F32::min(2.0, F32::floor(fTemp14 + -0.5)));
				fTemp16 = fTemp15 + 1.0;
				iTemp17 = (fTemp16 == 1.0) as i32;
				fTemp18 = (iTemp17) as F32;
				iTemp19 = (fTemp18 != 0.0) as i32;
				iTemp20 = (fTemp16 == 0.0) as i32;
				fTemp21 = (iTemp20) as F32;
				iTemp22 = (fTemp21 != 0.0) as i32;
				iTemp23 = (fTemp16 == 2.0) as i32;
				fTemp24 = (iTemp23) as F32;
				iTemp25 = (fTemp24 != 0.0) as i32;
				iTemp26 = (fTemp16 == 3.0) as i32;
				fTemp27 = (iTemp26) as F32;
				iTemp28 = (fTemp27 != 0.0) as i32;
				iTemp29 = (fTemp15 == 1.0) as i32;
				fTemp30 = (iTemp29) as F32;
				iTemp31 = (fTemp30 != 0.0) as i32;
				iTemp32 = (fTemp15 == 0.0) as i32;
				fTemp33 = (iTemp32) as F32;
				iTemp34 = (fTemp33 != 0.0) as i32;
				iTemp35 = (fTemp15 == 2.0) as i32;
				fTemp36 = (iTemp35) as F32;
				iTemp37 = (fTemp36 != 0.0) as i32;
				iTemp38 = (fTemp15 == 3.0) as i32;
				fTemp39 = (iTemp38) as F32;
				iTemp40 = (fTemp39 != 0.0) as i32;
			}
				if (iSlow12 & iTemp40) | ((iSlow12 & iTemp37) | ((iSlow12 & iTemp34) | ((iSlow12 & iTemp31) | ((iSlow12 & iTemp28) | ((iSlow12 & iTemp25) | ((iSlow12 & iTemp22) | ((iSlow12 & iTemp19) | ((iSlow9 & iTemp40) | ((iSlow9 & iTemp37) | ((iSlow9 & iTemp34) | ((iSlow9 & iTemp31) | ((iSlow9 & iTemp28) | ((iSlow9 & iTemp25) | ((iSlow9 & iTemp22) | ((iSlow9 & iTemp19) | ((iSlow6 & iTemp40) | ((iSlow6 & iTemp37) | ((iSlow6 & iTemp34) | ((iSlow6 & iTemp31) | ((iSlow6 & iTemp28) | ((iSlow6 & iTemp25) | ((iSlow6 & iTemp22) | (iSlow6 & iTemp19))))))))))))))))))))))) != 0 {
					self.fRec7[0] = fSlow17 * fTemp8 + self.fConst1 * fTemp7 * self.fRec7[1];
					iTemp41 = (self.fRec7[0] > 0.0) as i32;
					fTemp42 = (iTemp41) as F32;
				}
					self.fVec5[0] = fSlow18;
					if iSlow21 != 0 {
						self.fVec6[0] = fSlow22;
						fTemp43 = if ((fSlow22 != self.fVec6[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec8[1] + -1.0};
						self.fRec8[0] = fTemp43;
						self.fRec9[0] = if iTemp0 != 0 {fSlow22} else {if (fTemp43 > 0.0) as i32 != 0 {self.fRec9[1] + (fSlow22 - self.fRec9[1]) / F32::max(1.0, fTemp43)} else {fSlow22}};
						fTemp44 = F32::max(0.005, 0.001 * self.fRec9[0]);
						self.iRec10[0] = i32::wrapping_mul(iTemp6, i32::wrapping_add(self.iRec10[1], 1));
						fTemp45 = (self.iRec10[0]) as F32;
						fTemp46 = self.fConst2 * fTemp45;
						self.iRec11[0] = std::cmp::max(iTemp0, self.iRec11[1]);
						iTemp47 = i32::wrapping_mul(self.iRec11[0], (fTemp46 < (fTemp44 + 0.05)) as i32);
						fTemp48 = (iTemp47) as F32;
					}
						if iTemp4 | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp19)) | ((((fTemp11 != 0.0) as i32) & iSlow12) | ((iSlow21 & (iSlow31 & (iSlow37 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow34 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32)))) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp19)) | ((((fTemp11 != 0.0) as i32) & iSlow9) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp40)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp37)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp34)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp31)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp28)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp25)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp22)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp19)) | (iSlow6 & ((fTemp11 != 0.0) as i32))))))))))))))))))))))))))))))) != 0 {
							iTemp5 = i32::wrapping_sub(1, self.iVec1[1]);
						}
							if iTemp4 != 0 {
								self.iRec3[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec3[1], i32::wrapping_mul(iSlow39, iTemp5))), 12345);
								iTemp49 = i32::wrapping_mul(1664525, self.iRec3[0] ^ (self.iRec3[0] >> 15));
								self.fRec2[0] = self.fConst3 * self.fRec2[1] + self.fConst5 * (i32::wrapping_mul(1013904223, iTemp49 ^ (iTemp49 >> 13))) as F32;
								self.fRec12[0] = fSlow40 * fTemp8 + self.fConst1 * fTemp7 * self.fRec12[1];
								fTemp50 = self.fConst6 * self.fRec1[0] * self.fRec12[0] * self.fRec2[0] * fTemp3;
							}
								let mut fTemp51: F32 = ((iTemp2 != 0) as i32) as u32 as F32;
								let mut fTemp52: F32 = fTemp51 * fTemp50;
								let mut fTemp53: F32 = F32::floor(fTemp52);
								let mut iTemp54: i32 = (fTemp53) as i32;
								let mut fTemp55: F32 = fTemp52 - fTemp53;
								if (iSlow21 & (iSlow31 & (iSlow50 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow47 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow37 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow34 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32)))) | (iSlow21 & (iSlow43 & ((fTemp48 != 0.0) as i32))))))) != 0 {
									self.fVec7[0] = fSlow51;
									fTemp56 = if ((fSlow51 != self.fVec7[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec14[1] + -1.0};
									self.fRec14[0] = fTemp56;
									self.fRec15[0] = if iTemp0 != 0 {fSlow51} else {if (fTemp56 > 0.0) as i32 != 0 {self.fRec15[1] + (fSlow51 - self.fRec15[1]) / F32::max(1.0, fTemp56)} else {fSlow51}};
									fTemp57 = F32::exp(0.057762265 * self.fRec15[0]);
								}
									if (iSlow21 & (iSlow31 & (iSlow50 & ((fTemp48 != 0.0) as i32)))) | (iSlow21 & (iSlow43 & ((fTemp48 != 0.0) as i32))) != 0 {
										fTemp58 = fTemp45 * fTemp57;
									}
										if iSlow21 & (iSlow43 & ((fTemp48 != 0.0) as i32)) != 0 {
											fTemp59 = fSlow42 * tr_read(iSlow44, self.fConst7 * fTemp58) * F32::min(1.0, F32::max(0.0, 2.5 * ((fTemp44 - fTemp46) / fTemp44)));
										}
											if iTemp4 != 0 {
												fTemp60 = if iTemp5 != 0 {0.0} else {self.fConst9 + self.fRec19[1]};
												self.fRec19[0] = fTemp60 - F32::floor(fTemp60);
												fTemp61 = if iTemp5 != 0 {0.0} else {fSlow57 + self.fRec20[1]};
												self.fRec20[0] = fTemp61 - F32::floor(fTemp61);
												self.iRec22[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec22[1], i32::wrapping_mul(iSlow58, iTemp5))), 12345);
												iTemp62 = i32::wrapping_mul(1664525, self.iRec22[0] ^ (self.iRec22[0] >> 15));
												self.fRec21[0] = self.fConst11 * self.fRec21[1] + self.fConst13 * (i32::wrapping_mul(1013904223, iTemp62 ^ (iTemp62 >> 13))) as F32;
												fTemp63 = if iTemp5 != 0 {0.0} else {fSlow64 + self.fRec25[1]};
												self.fRec25[0] = fTemp63 - F32::floor(fTemp63);
											}
												if iTemp4 | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp19)) | ((((fTemp11 != 0.0) as i32) & (iSlow74 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow12)) | ((iSlow21 & (iSlow31 & (iSlow47 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32)))) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp19)) | ((((fTemp11 != 0.0) as i32) & (iSlow74 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow9)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp40)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp37)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp34)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp31)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp28)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp25)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp22)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp19)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow74)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow71)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow68)))))))))))))))))))))))))))))))))))) != 0 {
													self.iRec26[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec26[1]), 12345);
													fTemp64 = (self.iRec26[0]) as F32;
												}
													if iTemp4 | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp19)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow12)) | ((iSlow21 & (iSlow31 & (iSlow47 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32)))) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp19)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow9)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp40)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp37)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp34)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp31)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp28)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp25)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp22)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp19)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow71)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow68))))))))))))))))))))))))))))))))) != 0 {
														fTemp65 = 4.656613e-10 * fTemp64;
													}
														if iTemp4 != 0 {
															self.fRec24[0] = if (self.fRec25[0] < self.fRec25[1]) as i32 != 0 {fTemp65} else {self.fRec24[1]};
															fTemp66 = F32::floor(32768.0 * self.fRec24[0] + 0.5);
															self.fVec9[0] = fTemp66;
															self.fRec23[0] = fSlow75 * (3.0517578e-05 * (fTemp66 + self.fVec9[1]) - fSlow63 * self.fRec23[1]);
															fTemp67 = fTemp3 * (self.fRec1[0] * (fSlow80 + 2e+01 * self.fRec23[0] + self.fConst14 * self.fRec12[0] * self.fRec21[0]) + VoiceDsp_faustpower2_f(self.fRec1[0]) * (18.0 * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec20[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec19[0]) as i32, 65535))) as usize] }));
														}
															self.fRec27[0] = fSlow81 * fTemp8 + self.fConst1 * fTemp7 * self.fRec27[1];
															self.fVec10[0] = fSlow84;
															let mut fTemp68: F32 = if ((fSlow84 != self.fVec10[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec28[1] + -1.0};
															self.fRec28[0] = fTemp68;
															self.fRec29[0] = if iTemp0 != 0 {fSlow84} else {if (fTemp68 > 0.0) as i32 != 0 {self.fRec29[1] + (fSlow84 - self.fRec29[1]) / F32::max(1.0, fTemp68)} else {fSlow84}};
															let mut fTemp69: F32 = F32::exp(0.057762265 * (self.fRec29[0] + fSlow83 * self.fRec27[0] + 0.01 * fTemp51 * fTemp67));
															self.fRec30[0] = fSlow88 + fSlow87 * self.fRec30[1];
															let mut fTemp70: F32 = F32::exp(self.fRec30[0]);
															if (iSlow21 & (iSlow31 & (iSlow37 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow34 & ((fTemp48 != 0.0) as i32)))) | (iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32))))) != 0 {
																fTemp71 = fTemp70 * fTemp57 * fTemp69;
															}
																if iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32))) != 0 {
																	fTemp72 = if iTemp5 != 0 {0.0} else {self.fRec17[1] + self.fConst2 * F32::max(4.0 * fTemp71, 0.0)};
																	self.fRec17[0] = fTemp72 - F32::floor(fTemp72);
																	self.fRec16[0] = if (self.fRec17[0] < self.fRec17[1]) as i32 != 0 {fTemp65} else {self.fRec16[1]};
																	fTemp73 = fSlow89 * F32::floor(32.0 * self.fRec16[0] + 0.5);
																}
																	if iSlow21 & (iSlow31 & (iSlow34 & ((fTemp48 != 0.0) as i32))) != 0 {
																		fTemp74 = if iTemp5 != 0 {0.0} else {self.fRec32[1] + self.fConst16 * fTemp71};
																		self.fRec32[0] = fTemp74 - F32::floor(fTemp74);
																		fTemp75 = if iTemp5 != 0 {0.0} else {self.fRec31[1] + self.fConst2 * fTemp71 * (4.0 * F32::exp(-(self.fConst17 * fTemp45)) * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec32[0]) as i32, 65535))) as usize] } + 3.53)};
																		self.fRec31[0] = fTemp75 - F32::floor(fTemp75);
																		fTemp76 = fSlow33 * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec31[0]) as i32, 65535))) as usize] };
																	}
																		if iSlow21 & (iSlow31 & (iSlow37 & ((fTemp48 != 0.0) as i32))) != 0 {
																			fTemp77 = if iTemp5 != 0 {0.0} else {self.fRec33[1] + self.fConst2 * fTemp71 * (7.0 * F32::exp(-(self.fConst18 * fTemp45)) + 1.0)};
																			self.fRec33[0] = fTemp77 - F32::floor(fTemp77);
																			fTemp78 = fSlow36 * unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec33[0]) as i32, 65535))) as usize] };
																		}
																			if iSlow21 & (iSlow31 & (iSlow47 & ((fTemp48 != 0.0) as i32))) != 0 {
																				fTemp79 = F32::tan(self.fConst15 * F32::min(8e+02 * fTemp57, self.fConst19));
																				fTemp80 = 1.0 / fTemp79;
																				fTemp81 = (fTemp80 + 1.4142135) / fTemp79 + 1.0;
																				fTemp82 = VoiceDsp_faustpower2_f(fTemp79);
																				self.fRec34[0] = fTemp65 - (self.fRec34[2] * ((fTemp80 + -1.4142135) / fTemp79 + 1.0) + 2.0 * self.fRec34[1] * (1.0 - 1.0 / fTemp82)) / fTemp81;
																				fTemp83 = fSlow46 * ((self.fRec34[2] + (self.fRec34[0] - 2.0 * self.fRec34[1])) / (fTemp82 * fTemp81));
																			}
																				if iSlow21 & (iSlow31 & (iSlow50 & ((fTemp48 != 0.0) as i32))) != 0 {
																					fTemp84 = fSlow49 * F32::sin(self.fConst21 * fTemp58) * F32::exp(-(self.fConst20 * fTemp45));
																				}
																					if iSlow21 & (iSlow31 & ((fTemp48 != 0.0) as i32)) != 0 {
																						fTemp85 = fSlow30 * (fSlow94 * fTemp84 + fSlow93 * fTemp83 + fSlow92 * fTemp78 + fSlow91 * fTemp76 + fSlow90 * fTemp73) * F32::exp(-(self.fConst8 * (fTemp45 / fTemp44))) * ((fTemp46 < fTemp44) as i32) as u32 as F32;
																					}
																						if iSlow21 & ((fTemp48 != 0.0) as i32) != 0 {
																							fTemp86 = (self.iRec11[0]) as F32 * (fSlow95 * fTemp85 + fSlow52 * fTemp59);
																							self.fVec11[0] = fTemp86;
																							self.fRec13[0] = 0.995 * self.fRec13[1] + fTemp86 - self.fVec11[1];
																							fTemp87 = self.fRec13[0] * fTemp48;
																						}
																							if iSlow21 != 0 {
																								fTemp88 = fSlow20 * fTemp87 * ((iTemp47 != 0) as i32) as u32 as F32;
																							}
																								let mut fTemp89: F32 = if ((fSlow18 != self.fVec5[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec35[1] + -1.0};
																								self.fRec35[0] = fTemp89;
																								self.fRec36[0] = if iTemp0 != 0 {fSlow18} else {if (fTemp89 > 0.0) as i32 != 0 {self.fRec36[1] + (fSlow18 - self.fRec36[1]) / F32::max(1.0, fTemp89)} else {fSlow18}};
																								let mut fTemp90: F32 = self.fRec36[0] * fTemp88;
																								self.fRec38[0] = if (fSlow0 < self.fVec0[1]) as i32 != 0 {self.fRec37[1]} else {self.fRec38[1]};
																								let mut fTemp91: F32 = if (0.0 >= self.fRec38[0]) as i32 != 0 {fSlow100} else {-fSlow100};
																								let mut fTemp92: F32 = 5.0 * fTemp91;
																								self.fRec39[0] = fSlow105 * (self.fRec39[1] + 1.0);
																								let mut fTemp93: F32 = F32::max(0.0, F32::min(1.0, self.fConst2 * (self.fRec39[0] / F32::max(fSlow104 * (fSlow103 * self.fRec1[0] + 1.0), 0.0001))));
																								let mut fTemp94: F32 = F32::max(F32::max(0.005, fSlow108 * (fSlow107 * self.fRec1[0] + 1.0)), 0.0001);
																								self.fRec40[0] = fSlow0 * (self.fRec40[1] + 1.0);
																								let mut fTemp95: F32 = self.fConst2 * self.fRec40[0];
																								let mut fTemp96: F32 = F32::max(0.0, F32::min(1.0, (fTemp95 - fTemp94) / F32::max(fSlow117 * (fSlow116 * self.fRec1[0] + 1.0), 0.0001)));
																								self.fRec41[0] = if iTemp0 != 0 {self.fRec37[1]} else {self.fRec41[1]};
																								let mut fTemp97: F32 = if (1.0 >= self.fRec41[0]) as i32 != 0 {fSlow119} else {-fSlow119};
																								let mut fTemp98: F32 = 5.0 * fTemp97;
																								let mut fTemp99: F32 = F32::max(0.0, F32::min(1.0, self.fConst2 * (self.fRec40[0] / fTemp94)));
																								self.fRec37[0] = if iSlow99 != 0 {if (fTemp95 < fTemp94) as i32 != 0 {self.fRec41[0] + (1.0 - self.fRec41[0]) * if (F32::abs(fTemp98) < 0.001) as i32 != 0 {fTemp99} else {(1.0 - F32::exp(-(5.0 * fTemp97 * fTemp99))) / (1.0 - F32::exp(-fTemp98))}} else {fSlow118 * if iSlow113 != 0 {fTemp96} else {(1.0 - F32::exp(-(5.0 * fSlow111 * fTemp96))) / fSlow114} + 1.0}} else {self.fRec38[0] * (1.0 - if (F32::abs(fTemp92) < 0.001) as i32 != 0 {fTemp93} else {(1.0 - F32::exp(-(5.0 * fTemp91 * fTemp93))) / (1.0 - F32::exp(-fTemp92))})};
																								self.fVec12[0] = fSlow120;
																								let mut fTemp100: F32 = if ((fSlow120 != self.fVec12[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec43[1] + -1.0};
																								self.fRec43[0] = fTemp100;
																								self.fRec44[0] = if iTemp0 != 0 {fSlow120} else {if (fTemp100 > 0.0) as i32 != 0 {self.fRec44[1] + (fSlow120 - self.fRec44[1]) / F32::max(1.0, fTemp100)} else {fSlow120}};
																								let mut fTemp101: F32 = F32::tan(self.fConst15 * F32::max(1e+01, F32::min(2e+03, self.fRec44[0])));
																								let mut fTemp102: F32 = 1.0 / fTemp101;
																								self.fRec45[0] = fSlow121 * fTemp8 + self.fConst1 * fTemp7 * self.fRec45[1];
																								let mut iTemp103: i32 = (self.fRec45[0] > 0.0) as i32;
																								if iSlow6 != 0 {
																									fTemp104 = 1.0 - self.fRec54[1];
																								}
																									if iTemp4 != 0 {
																										fTemp105 = if iTemp5 != 0 {0.0} else {fSlow127 + self.fRec62[1]};
																										self.fRec62[0] = fTemp105 - F32::floor(fTemp105);
																										self.fRec61[0] = if (self.fRec62[0] < self.fRec62[1]) as i32 != 0 {fTemp65} else {self.fRec61[1]};
																										fTemp106 = F32::floor(32768.0 * self.fRec61[0] + 0.5);
																										self.fVec13[0] = fTemp106;
																										self.fRec60[0] = fSlow128 * (3.0517578e-05 * (fTemp106 + self.fVec13[1]) - fSlow126 * self.fRec60[1]);
																										fTemp107 = self.fRec1[0] * fTemp3 * (fSlow130 + 0.1 * self.fRec60[0]);
																									}
																										self.fVec14[0] = fSlow131;
																										let mut fTemp108: F32 = if ((fSlow131 != self.fVec14[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec63[1] + -1.0};
																										self.fRec63[0] = fTemp108;
																										self.fRec64[0] = if iTemp0 != 0 {fSlow131} else {if (fTemp108 > 0.0) as i32 != 0 {self.fRec64[1] + (fSlow131 - self.fRec64[1]) / F32::max(1.0, fTemp108)} else {fSlow131}};
																										let mut fTemp109: F32 = F32::max(2e+01, F32::min(1.8e+04, self.fRec64[0] * F32::exp(0.6931472 * fTemp51 * fTemp107) * (fSlow122 * self.fRec27[0] + 1.0)));
																										if iSlow12 | (iSlow9 | iSlow6) != 0 {
																											fTemp110 = F32::min(self.fConst19, fTemp109);
																										}
																											if iSlow9 | iSlow6 != 0 {
																												fTemp111 = F32::tan(self.fConst15 * F32::max(1e+01, fTemp110));
																											}
																												if iSlow6 != 0 {
																													fTemp112 = fTemp111 * fTemp104 + 1.0;
																													fTemp113 = 1.0 - self.fRec57[1];
																													fTemp114 = fTemp111 * fTemp113 + 1.0;
																													fTemp115 = 1.0 - self.fRec56[1];
																													fTemp116 = fTemp111 * fTemp115 + 1.0;
																													fTemp117 = 1.0 - self.fRec55[1];
																													fTemp118 = fTemp111 * fTemp117 + 1.0;
																													fTemp119 = fTemp112 * fTemp118 * fTemp116;
																													fTemp120 = 1.0 - self.fRec53[1];
																													fTemp121 = 1.0 - self.fRec52[1];
																													fTemp122 = 1.0 - self.fRec51[1];
																													fTemp123 = 1.0 - self.fRec58[1];
																													fTemp124 = 1.0 - self.fRec50[1];
																												}
																													self.fVec15[0] = fSlow133;
																													let mut fTemp125: F32 = if ((fSlow133 != self.fVec15[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec65[1] + -1.0};
																													self.fRec65[0] = fTemp125;
																													self.fRec66[0] = if iTemp0 != 0 {fSlow133} else {if (fTemp125 > 0.0) as i32 != 0 {self.fRec66[1] + (fSlow133 - self.fRec66[1]) / F32::max(1.0, fTemp125)} else {fSlow133}};
																													let mut fTemp126: F32 = F32::max(0.5, F32::min(8.0, self.fRec66[0] * (fSlow132 * self.fRec27[0] + 1.0)));
																													if iSlow9 | iSlow6 != 0 {
																														fTemp127 = F32::max(0.0, F32::min(1.0, 0.18181819 * (fTemp126 + -0.5)));
																														fTemp128 = VoiceDsp_faustpower4_f(fTemp111);
																													}
																														if iSlow6 != 0 {
																															fTemp129 = 4.4 * (fTemp128 * fTemp127 * fTemp124 * fTemp123 * fTemp122 * fTemp121 * fTemp120 / (fTemp119 * fTemp114)) + 1.0;
																															fTemp130 = fTemp114 * fTemp129;
																														}
																															if iSlow12 | (iSlow9 | iSlow6) != 0 {
																																fTemp131 = if ((fSlow14 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec67[1] + -1.0};
																																self.fRec67[0] = fTemp131;
																																self.fRec68[0] = if iTemp0 != 0 {fSlow14} else {if (fTemp131 > 0.0) as i32 != 0 {self.fRec68[1] + (fSlow14 - self.fRec68[1]) / F32::max(1.0, fTemp131)} else {fSlow14}};
																																self.fVec16[0] = fSlow136;
																																fTemp132 = if ((fSlow136 != self.fVec16[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec69[1] + -1.0};
																																self.fRec69[0] = fTemp132;
																																self.fRec70[0] = if iTemp0 != 0 {fSlow136} else {if (fTemp132 > 0.0) as i32 != 0 {self.fRec70[1] + (fSlow136 - self.fRec70[1]) / F32::max(1.0, fTemp132)} else {fSlow136}};
																																self.fVec17[0] = fSlow139;
																																fTemp133 = if ((fSlow139 != self.fVec17[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec71[1] + -1.0};
																																self.fRec71[0] = fTemp133;
																																self.fRec72[0] = if iTemp0 != 0 {fSlow139} else {if (fTemp133 > 0.0) as i32 != 0 {self.fRec72[1] + (fSlow139 - self.fRec72[1]) / F32::max(1.0, fTemp133)} else {fSlow139}};
																															}
																																if (((fTemp11 != 0.0) as i32) & iSlow12) | ((((fTemp11 != 0.0) as i32) & iSlow9) | (iSlow6 & ((fTemp11 != 0.0) as i32))) != 0 {
																																	self.fVec18[0] = fSlow142;
																																	fTemp134 = if ((fSlow142 != self.fVec18[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec74[1] + -1.0};
																																	self.fRec74[0] = fTemp134;
																																	self.fRec75[0] = if iTemp0 != 0 {fSlow142} else {if (fTemp134 > 0.0) as i32 != 0 {self.fRec75[1] + (fSlow142 - self.fRec75[1]) / F32::max(1.0, fTemp134)} else {fSlow142}};
																																	fTemp135 = F32::exp(-(self.fConst22 * self.fRec75[0]));
																																	fTemp136 = self.fConst2 * self.fRec75[0] + self.fRec77[1];
																																	self.fRec77[0] = fTemp136 - F32::floor(fTemp136);
																																	self.iRec78[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec78[1], i32::wrapping_mul(iSlow143, iTemp5))), 12345);
																																	iTemp137 = i32::wrapping_mul(1664525, self.iRec78[0] ^ (self.iRec78[0] >> 15));
																																	self.fRec76[0] = if ((self.fRec77[0] < self.fRec77[1]) as i32) | ((iTemp5 > 0) as i32) != 0 {4.656613e-10 * (i32::wrapping_mul(1013904223, iTemp137 ^ (iTemp137 >> 13))) as F32} else {self.fRec76[1]};
																																	self.fRec73[0] = self.fRec76[0] * (1.0 - fTemp135) + fTemp135 * self.fRec73[1];
																																	self.fVec19[0] = fSlow144;
																																	fTemp138 = if ((fSlow144 != self.fVec19[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec79[1] + -1.0};
																																	self.fRec79[0] = fTemp138;
																																	self.fRec80[0] = if iTemp0 != 0 {fSlow144} else {if (fTemp138 > 0.0) as i32 != 0 {self.fRec80[1] + (fSlow144 - self.fRec80[1]) / F32::max(1.0, fTemp138)} else {fSlow144}};
																																	fTemp139 = 3.0 * self.fRec80[0] * self.fRec73[0];
																																	fTemp140 = F32::floor(fTemp139);
																																	iTemp141 = (fTemp140) as i32;
																																	fTemp142 = fTemp139 - fTemp140;
																																	self.fVec20[0] = fSlow145;
																																	fTemp143 = if ((fSlow145 != self.fVec20[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec81[1] + -1.0};
																																	self.fRec81[0] = fTemp143;
																																	self.fRec82[0] = if iTemp0 != 0 {fSlow145} else {if (fTemp143 > 0.0) as i32 != 0 {self.fRec82[1] + (fSlow145 - self.fRec82[1]) / F32::max(1.0, fTemp143)} else {fSlow145}};
																																	fTemp144 = 0.025 * self.fRec82[0];
																																}
																																	if (((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow68))) != 0 {
																																		self.fVec21[0] = fSlow146;
																																		fTemp145 = if ((fSlow146 != self.fVec21[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec87[1] + -1.0};
																																		self.fRec87[0] = fTemp145;
																																		self.fRec88[0] = if iTemp0 != 0 {fSlow146} else {if (fTemp145 > 0.0) as i32 != 0 {self.fRec88[1] + (fSlow146 - self.fRec88[1]) / F32::max(1.0, fTemp145)} else {fSlow146}};
																																		fTemp146 = fTemp64 * ((F32::abs(fTemp65) > (1.0 - 0.002 * F32::max(0.0, F32::min(1.0, self.fRec88[0])))) as i32) as u32 as F32;
																																		self.fVec22[0] = fTemp146;
																																		self.fRec86[0] = self.fConst25 * (1.8626451e-09 * (fTemp146 + self.fVec22[1]) - self.fConst24 * self.fRec86[1]);
																																		self.fRec85[0] = -(self.fConst28 * (self.fConst27 * self.fRec85[1] - self.fConst26 * (self.fRec86[0] - self.fRec86[1])));
																																		fTemp147 = fSlow67 * self.fRec85[0];
																																	}
																																		let mut fTemp148: F32 = fTemp70 * fTemp69;
																																		if (((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow153 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow150 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow153 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow150 & iSlow9)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow71)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow153)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow150))))))))) != 0 {
																																			fTemp149 = if iTemp5 != 0 {0.0} else {self.fRec90[1] + self.fConst2 * F32::max(fSlow154 * fTemp148, 0.0)};
																																			self.fRec90[0] = fTemp149 - F32::floor(fTemp149);
																																			iTemp150 = (self.fRec90[0] < self.fRec90[1]) as i32;
																																		}
																																			if (((fTemp11 != 0.0) as i32) & (iSlow150 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow150 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow150))) != 0 {
																																				iTemp151 = if (self.iRec89[1] == 0) as i32 != 0 {1} else {self.iRec89[1]};
																																				self.iRec89[0] = if iTemp150 != 0 {(iTemp151 >> 1) | (((iTemp151 ^ (iTemp151 >> 6)) & 1) << 14)} else {iTemp151};
																																				fTemp152 = fSlow149 * if self.iRec89[0] & 1 != 0 {1.0} else {-1.0};
																																			}
																																				if (((fTemp11 != 0.0) as i32) & (iSlow153 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow153 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow153))) != 0 {
																																					iTemp153 = if (self.iRec91[1] == 0) as i32 != 0 {1} else {self.iRec91[1]};
																																					iTemp154 = iTemp153 >> 1;
																																					self.iRec91[0] = if iTemp150 != 0 {iTemp154 | (((iTemp153 ^ iTemp154) & 1) << 14)} else {iTemp153};
																																					fTemp155 = fSlow152 * if self.iRec91[0] & 1 != 0 {1.0} else {-1.0};
																																				}
																																					if (((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow71))) != 0 {
																																						self.fRec92[0] = if iTemp150 != 0 {fTemp65} else {self.fRec92[1]};
																																						fTemp156 = fSlow158 * F32::floor(fSlow157 * self.fRec92[0] + 0.5);
																																					}
																																						if (((fTemp11 != 0.0) as i32) & (iSlow74 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow74 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow74))) != 0 {
																																							fTemp157 = fSlow160 * fTemp64;
																																						}
																																							if (((fTemp11 != 0.0) as i32) & iSlow12) | ((((fTemp11 != 0.0) as i32) & iSlow9) | (iSlow6 & ((fTemp11 != 0.0) as i32))) != 0 {
																																								fTemp158 = fSlow161 * fTemp157 + fSlow159 * fTemp156 + fSlow156 * fTemp155 + fSlow155 * fTemp152 + fSlow147 * fTemp147;
																																								self.fVec23[0] = fTemp158;
																																								self.fRec84[0] = -(self.fConst31 * (self.fConst30 * self.fRec84[1] - (fTemp158 + self.fVec23[1])));
																																								fTemp159 = fTemp158 + self.fRec84[0] * (F32::powf(1e+01, -fTemp144) + -1.0);
																																								self.fVec24[0] = fTemp159;
																																								self.fRec83[0] = -(self.fConst31 * (self.fConst30 * self.fRec83[1] - self.fConst29 * (fTemp159 - self.fVec24[1])));
																																								fTemp160 = fTemp11 * (fTemp159 + self.fRec83[0] * (F32::powf(1e+01, fTemp144) + -1.0)) * (fTemp142 * (fTemp142 * (fTemp142 * (fTemp142 * (0.0013333 * fTemp142 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp141))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp141))) as F32;
																																							}
																																								if iSlow12 | (iSlow9 | iSlow6) != 0 {
																																									fTemp161 = F32::powf(1e+01, 1.2 * (0.625 * VoiceDsp_faustpower4_f(fTemp13) + 0.375 * fTemp13));
																																								}
																																									if (((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp19)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp19)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp40)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp37)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp34)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp31)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp28)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp25)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp22)) | (iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp19)))))))))))))))))))))))) != 0 {
																																										fTemp162 = if iTemp5 != 0 {0.0} else {self.fConst34 + self.fRec96[1]};
																																										self.fRec96[0] = fTemp162 - F32::floor(fTemp162);
																																										self.fRec95[0] = if (self.fRec96[0] < self.fRec96[1]) as i32 != 0 {fTemp65} else {self.fRec95[1]};
																																										fTemp163 = F32::floor(32768.0 * self.fRec95[0] + 0.5);
																																										self.fVec25[0] = fTemp163;
																																										self.fRec94[0] = self.fConst35 * (3.0517578e-05 * (fTemp163 + self.fVec25[1]) - self.fConst33 * self.fRec94[1]);
																																										fTemp164 = 0.3 * self.fRec7[0] * self.fRec94[0] * fTemp42;
																																									}
																																										if (iSlow12 & iTemp40) | ((iSlow12 & iTemp37) | ((iSlow12 & iTemp34) | ((iSlow12 & iTemp31) | ((iSlow12 & iTemp28) | ((iSlow12 & iTemp25) | ((iSlow12 & iTemp22) | ((iSlow12 & iTemp19) | ((iSlow9 & iTemp40) | ((iSlow9 & iTemp37) | ((iSlow9 & iTemp34) | ((iSlow9 & iTemp31) | ((iSlow9 & iTemp28) | ((iSlow9 & iTemp25) | ((iSlow9 & iTemp22) | ((iSlow9 & iTemp19) | ((iSlow6 & iTemp40) | ((iSlow6 & iTemp37) | ((iSlow6 & iTemp34) | ((iSlow6 & iTemp31) | ((iSlow6 & iTemp28) | ((iSlow6 & iTemp25) | ((iSlow6 & iTemp22) | (iSlow6 & iTemp19))))))))))))))))))))))) != 0 {
																																											fTemp165 = ((iTemp41 != 0) as i32) as u32 as F32 * fTemp164;
																																										}
																																											if (iSlow12 & iTemp31) | ((iSlow12 & iTemp19) | ((iSlow9 & iTemp31) | ((iSlow9 & iTemp19) | ((iSlow6 & iTemp31) | (iSlow6 & iTemp19))))) != 0 {
																																												fTemp166 = F32::max(-1.0, F32::min(1.0, fTemp165));
																																											}
																																												if (iSlow12 & iTemp19) | ((iSlow9 & iTemp19) | (iSlow6 & iTemp19)) != 0 {
																																													fTemp167 = fTemp166 * fTemp18;
																																												}
																																													if (iSlow12 & iTemp40) | ((iSlow12 & iTemp37) | ((iSlow12 & iTemp34) | ((iSlow12 & iTemp31) | ((iSlow12 & iTemp28) | ((iSlow12 & iTemp25) | ((iSlow12 & iTemp22) | ((iSlow12 & iTemp19) | ((iSlow9 & iTemp40) | ((iSlow9 & iTemp37) | ((iSlow9 & iTemp34) | ((iSlow9 & iTemp31) | ((iSlow9 & iTemp28) | ((iSlow9 & iTemp25) | ((iSlow9 & iTemp22) | ((iSlow9 & iTemp19) | ((iSlow6 & iTemp40) | ((iSlow6 & iTemp37) | ((iSlow6 & iTemp34) | ((iSlow6 & iTemp31) | ((iSlow6 & iTemp28) | ((iSlow6 & iTemp25) | ((iSlow6 & iTemp22) | (iSlow6 & iTemp19))))))))))))))))))))))) != 0 {
																																														fTemp168 = F32::sqrt(VoiceDsp_faustpower2_f(fSlow140 * self.fRec72[0]) + VoiceDsp_faustpower2_f(fSlow134 * self.fRec68[0]) + VoiceDsp_faustpower2_f(fSlow137 * self.fRec70[0]));
																																														fTemp169 = fSlow140 * *input0 * self.fRec72[0] + fSlow137 * *input1 * self.fRec70[0];
																																													}
																																														let mut fTemp170: F32 = (iTemp103) as F32;
																																														let mut iTemp171: i32 = (fTemp170 != 0.0) as i32;
																																														if iTemp171 != 0 {
																																															fTemp172 = F32::max(self.fRec45[0], 0.001);
																																														}
																																															if (iTemp171 & (iSlow12 & iTemp40)) | ((iTemp171 & (iSlow12 & iTemp37)) | ((iTemp171 & (iSlow12 & iTemp34)) | ((iTemp171 & (iSlow12 & iTemp31)) | ((iTemp171 & (iSlow12 & iTemp28)) | ((iTemp171 & (iSlow12 & iTemp25)) | ((iTemp171 & (iSlow12 & iTemp22)) | ((iTemp171 & (iSlow12 & iTemp19)) | ((iTemp171 & (iSlow9 & iTemp40)) | ((iTemp171 & (iSlow9 & iTemp37)) | ((iTemp171 & (iSlow9 & iTemp34)) | ((iTemp171 & (iSlow9 & iTemp31)) | ((iTemp171 & (iSlow9 & iTemp28)) | ((iTemp171 & (iSlow9 & iTemp25)) | ((iTemp171 & (iSlow9 & iTemp22)) | ((iTemp171 & (iSlow9 & iTemp19)) | ((iTemp171 & (iSlow6 & iTemp40)) | ((iTemp171 & (iSlow6 & iTemp37)) | ((iTemp171 & (iSlow6 & iTemp34)) | ((iTemp171 & (iSlow6 & iTemp31)) | ((iTemp171 & (iSlow6 & iTemp28)) | ((iTemp171 & (iSlow6 & iTemp25)) | ((iTemp171 & (iSlow6 & iTemp22)) | (iTemp171 & (iSlow6 & iTemp19)))))))))))))))))))))))) != 0 {
																																																fTemp173 = fTemp172 * fTemp169;
																																																self.fVec26[0] = fTemp173;
																																																fTemp174 = 0.8 * (fTemp173 + self.fVec26[1]);
																																																self.fRec98[0] = self.fConst39 * (fTemp174 - self.fConst38 * self.fRec98[1]);
																																																self.fRec99[0] = self.fConst44 * (fTemp174 - self.fConst43 * self.fRec99[1]);
																																																fTemp175 = 0.8 * fTemp173;
																																																fTemp176 = (self.fConst49 * (F32::tanh(fTemp175 + 0.08) - self.fConst48) + 0.4 * (self.fConst47 * (F32::tanh(0.9 * (fTemp175 - self.fRec99[0]) + 0.05) - self.fConst46) + self.fConst45 * (F32::tanh(1.25 * (self.fRec99[0] - self.fRec98[0]) + -0.08) - self.fConst41) + self.fConst40 * (F32::tanh(1.5 * self.fRec98[0] + 0.12) - self.fConst36))) / fTemp172;
																																																self.fVec27[0] = fTemp176;
																																																self.fRec97[0] = self.fConst52 * (self.fConst51 * self.fRec97[1] + 1.25 * (fTemp176 - self.fVec27[1]));
																																																fTemp177 = self.fRec97[0] * fTemp170;
																																															}
																																																let mut fTemp178: F32 = ((iTemp103 != 0) as i32) as u32 as F32;
																																																if (iSlow12 & iTemp40) | ((iSlow12 & iTemp37) | ((iSlow12 & iTemp34) | ((iSlow12 & iTemp31) | ((iSlow12 & iTemp28) | ((iSlow12 & iTemp25) | ((iSlow12 & iTemp22) | ((iSlow12 & iTemp19) | ((iSlow9 & iTemp40) | ((iSlow9 & iTemp37) | ((iSlow9 & iTemp34) | ((iSlow9 & iTemp31) | ((iSlow9 & iTemp28) | ((iSlow9 & iTemp25) | ((iSlow9 & iTemp22) | ((iSlow9 & iTemp19) | ((iSlow6 & iTemp40) | ((iSlow6 & iTemp37) | ((iSlow6 & iTemp34) | ((iSlow6 & iTemp31) | ((iSlow6 & iTemp28) | ((iSlow6 & iTemp25) | ((iSlow6 & iTemp22) | (iSlow6 & iTemp19))))))))))))))))))))))) != 0 {
																																																	fTemp179 = (if iTemp103 != 0 {fTemp178 * fTemp177} else {fTemp169} * ((fTemp168 > 0.0) as i32) as u32 as F32 / F32::max(1.1920929e-07, fTemp168) + fSlow162 * fTemp90) * fTemp161 + fTemp165;
																																																}
																																																	if (iSlow12 & iTemp31) | ((iSlow12 & iTemp19) | ((iSlow9 & iTemp31) | ((iSlow9 & iTemp19) | ((iSlow6 & iTemp31) | (iSlow6 & iTemp19))))) != 0 {
																																																		fTemp180 = F32::max(-1.0, F32::min(1.0, fTemp179));
																																																	}
																																																		if (iSlow12 & iTemp19) | ((iSlow9 & iTemp19) | (iSlow6 & iTemp19)) != 0 {
																																																			fTemp181 = fTemp180 * fTemp18;
																																																		}
																																																			if (iSlow12 & iTemp34) | ((iSlow12 & iTemp22) | ((iSlow9 & iTemp34) | ((iSlow9 & iTemp22) | ((iSlow6 & iTemp34) | (iSlow6 & iTemp22))))) != 0 {
																																																				fTemp182 = F32::atan(1.5707964 * fTemp165);
																																																			}
																																																				if (iSlow12 & iTemp22) | ((iSlow9 & iTemp22) | (iSlow6 & iTemp22)) != 0 {
																																																					fTemp183 = 0.63661975 * fTemp182 * fTemp21;
																																																				}
																																																					if (iSlow12 & iTemp34) | ((iSlow12 & iTemp22) | ((iSlow9 & iTemp34) | ((iSlow9 & iTemp22) | ((iSlow6 & iTemp34) | (iSlow6 & iTemp22))))) != 0 {
																																																						fTemp184 = F32::atan(1.5707964 * fTemp179);
																																																					}
																																																						if (iSlow12 & iTemp22) | ((iSlow9 & iTemp22) | (iSlow6 & iTemp22)) != 0 {
																																																							fTemp185 = 0.63661975 * fTemp184 * fTemp21;
																																																						}
																																																							if (iSlow12 & iTemp37) | ((iSlow12 & iTemp25) | ((iSlow9 & iTemp37) | ((iSlow9 & iTemp25) | ((iSlow6 & iTemp37) | (iSlow6 & iTemp25))))) != 0 {
																																																								fTemp186 = if (fTemp165 < 0.0) as i32 != 0 {1.6 * F32::tanh(0.625 * fTemp165)} else {F32::tanh(fTemp165)};
																																																							}
																																																								if (iSlow12 & iTemp25) | ((iSlow9 & iTemp25) | (iSlow6 & iTemp25)) != 0 {
																																																									fTemp187 = fTemp186 * fTemp24;
																																																								}
																																																									if (iSlow12 & iTemp37) | ((iSlow12 & iTemp25) | ((iSlow9 & iTemp37) | ((iSlow9 & iTemp25) | ((iSlow6 & iTemp37) | (iSlow6 & iTemp25))))) != 0 {
																																																										fTemp188 = if (fTemp179 < 0.0) as i32 != 0 {1.6 * F32::tanh(0.625 * fTemp179)} else {F32::tanh(fTemp179)};
																																																									}
																																																										if (iSlow12 & iTemp25) | ((iSlow9 & iTemp25) | (iSlow6 & iTemp25)) != 0 {
																																																											fTemp189 = fTemp188 * fTemp24;
																																																										}
																																																											if (iSlow12 & iTemp40) | ((iSlow12 & iTemp28) | ((iSlow9 & iTemp40) | ((iSlow9 & iTemp28) | ((iSlow6 & iTemp40) | (iSlow6 & iTemp28))))) != 0 {
																																																												fTemp190 = F32::sin(fTemp165);
																																																											}
																																																												if (iSlow12 & iTemp28) | ((iSlow9 & iTemp28) | (iSlow6 & iTemp28)) != 0 {
																																																													fTemp191 = fTemp190 * fTemp27;
																																																												}
																																																													if (iSlow12 & iTemp40) | ((iSlow12 & iTemp28) | ((iSlow9 & iTemp40) | ((iSlow9 & iTemp28) | ((iSlow6 & iTemp40) | (iSlow6 & iTemp28))))) != 0 {
																																																														fTemp192 = F32::sin(fTemp179);
																																																													}
																																																														if (iSlow12 & iTemp28) | ((iSlow9 & iTemp28) | (iSlow6 & iTemp28)) != 0 {
																																																															fTemp193 = fTemp192 * fTemp27;
																																																														}
																																																															if iSlow12 | (iSlow9 | iSlow6) != 0 {
																																																																fTemp194 = F32::max(0.0, F32::min(1.0, 2.5 * (fTemp14 + (-0.8 - fTemp15))));
																																																															}
																																																																if (iSlow12 & iTemp31) | ((iSlow9 & iTemp31) | (iSlow6 & iTemp31)) != 0 {
																																																																	fTemp195 = fTemp30 * fTemp166;
																																																																	fTemp196 = fTemp180 * fTemp30;
																																																																}
																																																																	if (iSlow12 & iTemp34) | ((iSlow9 & iTemp34) | (iSlow6 & iTemp34)) != 0 {
																																																																		fTemp197 = 0.63661975 * fTemp33 * fTemp182;
																																																																		fTemp198 = 0.63661975 * fTemp184 * fTemp33;
																																																																	}
																																																																		if (iSlow12 & iTemp37) | ((iSlow9 & iTemp37) | (iSlow6 & iTemp37)) != 0 {
																																																																			fTemp199 = fTemp36 * fTemp186;
																																																																			fTemp200 = fTemp188 * fTemp36;
																																																																		}
																																																																			if (iSlow12 & iTemp40) | ((iSlow9 & iTemp40) | (iSlow6 & iTemp40)) != 0 {
																																																																				fTemp201 = fTemp39 * fTemp190;
																																																																				fTemp202 = fTemp192 * fTemp39;
																																																																			}
																																																																				if iSlow12 | (iSlow9 | iSlow6) != 0 {
																																																																					fTemp203 = ((1.0 - fTemp194) * (((iTemp38 != 0) as i32) as u32 as F32 * (fTemp202 - fTemp201) + ((iTemp35 != 0) as i32) as u32 as F32 * (fTemp200 - fTemp199) + ((iTemp32 != 0) as i32) as u32 as F32 * (fTemp198 - fTemp197) + ((iTemp29 != 0) as i32) as u32 as F32 * (fTemp196 - fTemp195)) + fTemp194 * (((iTemp26 != 0) as i32) as u32 as F32 * (fTemp193 - fTemp191) + ((iTemp23 != 0) as i32) as u32 as F32 * (fTemp189 - fTemp187) + ((iTemp20 != 0) as i32) as u32 as F32 * (fTemp185 - fTemp183) + ((iTemp17 != 0) as i32) as u32 as F32 * (fTemp181 - fTemp167))) / F32::tanh(0.35 * fTemp161);
																																																																					self.fVec28[0] = fTemp203;
																																																																					self.fRec93[0] = 0.995 * self.fRec93[1] + 0.35 * (fTemp203 - self.fVec28[1]);
																																																																					fTemp204 = self.fRec93[0] * F32::powf(1e+01, 0.125 * self.fRec6[0]) + fSlow134 * (self.fRec68[0] * self.fRec4[0] * fTemp9 * ((iTemp10 != 0) as i32) as u32 as F32 * fTemp160 / F32::max(1e-09, F32::sqrt(fSlow141 * VoiceDsp_faustpower2_f(self.fRec72[0]) + fSlow138 * VoiceDsp_faustpower2_f(self.fRec70[0]) + fSlow135 * VoiceDsp_faustpower2_f(self.fRec68[0]))));
																																																																				}
																																																																					if iSlow9 | iSlow6 != 0 {
																																																																						fTemp205 = fTemp204 * fTemp128;
																																																																					}
																																																																						if iSlow6 != 0 {
																																																																							fTemp206 = fTemp111 * fTemp122;
																																																																							fTemp207 = fTemp111 * fTemp121;
																																																																							fTemp208 = fTemp111 * fTemp120;
																																																																							fTemp209 = self.fRec49[1] + fTemp208 * (self.fRec48[1] + fTemp207 * (self.fRec47[1] + fTemp206 * self.fRec46[1] / fTemp112) / fTemp118) / fTemp116 + 0.5 * (fTemp205 * fTemp124 * fTemp122 * fTemp121 * fTemp120 / fTemp119);
																																																																							fTemp210 = fTemp127 * fTemp123;
																																																																							fTemp211 = 4.4 * (fTemp210 * fTemp209 / fTemp130);
																																																																							fTemp212 = 0.5 * fTemp204;
																																																																							fTemp213 = fTemp212 - fTemp211;
																																																																							fTemp214 = self.fRec46[1] + fTemp111 * fTemp124 * fTemp213;
																																																																							fTemp215 = fTemp214 / fTemp112;
																																																																							self.fRec46[0] = fTemp215 + fTemp111 * (fTemp124 * fTemp213 - fTemp104 * fTemp214 / fTemp112);
																																																																							fTemp216 = self.fRec47[1] + fTemp206 * fTemp214 / fTemp112;
																																																																							fTemp217 = fTemp216 / fTemp118;
																																																																							self.fRec47[0] = fTemp217 + fTemp111 * (fTemp122 * fTemp214 / fTemp112 - fTemp117 * fTemp216 / fTemp118);
																																																																							fTemp218 = self.fRec48[1] + fTemp207 * fTemp216 / fTemp118;
																																																																							fTemp219 = fTemp218 / fTemp116;
																																																																							self.fRec48[0] = fTemp219 + fTemp111 * (fTemp121 * fTemp216 / fTemp118 - fTemp115 * fTemp218 / fTemp116);
																																																																							fTemp220 = self.fRec49[1] + fTemp208 * fTemp218 / fTemp116;
																																																																							fTemp221 = fTemp220 / fTemp114;
																																																																							self.fRec49[0] = fTemp221 + fTemp111 * (fTemp120 * fTemp218 / fTemp116 - fTemp113 * fTemp220 / fTemp114);
																																																																							fTemp222 = fTemp212 - (fTemp211 + fTemp215);
																																																																							fTemp223 = VoiceDsp_faustpower2_f(fTemp222);
																																																																							fTemp224 = 0.25 * fTemp223;
																																																																							fTemp225 = 1.0 - if (fTemp224 < 9.0) as i32 != 0 {(fTemp224 + 27.0) / (2.25 * fTemp223 + 27.0)} else {1.0 / F32::max(F32::abs(0.5 * fTemp222), 1e-09)};
																																																																							self.fRec50[0] = fTemp225;
																																																																							fTemp226 = fTemp215 - fTemp217;
																																																																							fTemp227 = VoiceDsp_faustpower2_f(fTemp226);
																																																																							fTemp228 = 0.25 * fTemp227;
																																																																							fTemp229 = 1.0 - if (fTemp228 < 9.0) as i32 != 0 {(fTemp228 + 27.0) / (2.25 * fTemp227 + 27.0)} else {1.0 / F32::max(F32::abs(0.5 * fTemp226), 1e-09)};
																																																																							self.fRec51[0] = fTemp229;
																																																																							fTemp230 = fTemp217 - fTemp219;
																																																																							fTemp231 = VoiceDsp_faustpower2_f(fTemp230);
																																																																							fTemp232 = 0.25 * fTemp231;
																																																																							fTemp233 = 1.0 - if (fTemp232 < 9.0) as i32 != 0 {(fTemp232 + 27.0) / (2.25 * fTemp231 + 27.0)} else {1.0 / F32::max(F32::abs(0.5 * fTemp230), 1e-09)};
																																																																							self.fRec52[0] = fTemp233;
																																																																							fTemp234 = fTemp219 - fTemp221;
																																																																							fTemp235 = VoiceDsp_faustpower2_f(fTemp234);
																																																																							fTemp236 = 0.25 * fTemp235;
																																																																							fTemp237 = 1.0 - if (fTemp236 < 9.0) as i32 != 0 {(fTemp236 + 27.0) / (2.25 * fTemp235 + 27.0)} else {1.0 / F32::max(F32::abs(0.5 * fTemp234), 1e-09)};
																																																																							self.fRec53[0] = fTemp237;
																																																																							self.fRec54[0] = fTemp225;
																																																																							self.fRec55[0] = fTemp229;
																																																																							self.fRec56[0] = fTemp233;
																																																																							self.fRec57[0] = fTemp237;
																																																																							fTemp238 = VoiceDsp_faustpower2_f(fTemp209) / (VoiceDsp_faustpower2_f(fTemp114) * VoiceDsp_faustpower2_f(fTemp129));
																																																																							fTemp239 = 25.0 * fTemp238;
																																																																							self.fRec58[0] = 1.0 - if (fTemp239 < 9.0) as i32 != 0 {(fTemp239 + 27.0) / (225.0 * fTemp238 + 27.0)} else {1.0 / F32::max(F32::abs(5.0 * (fTemp209 / fTemp130)), 1e-09)};
																																																																							fRec59 = fTemp209 * (2.2 * fTemp210 + 1.0) / fTemp130;
																																																																							fTemp240 = fSlow163 * fRec59;
																																																																						}
																																																																							if iSlow9 != 0 {
																																																																								fTemp241 = 1.0 - self.fRec108[1];
																																																																								fTemp242 = fTemp111 * fTemp241 + 1.0;
																																																																								fTemp243 = 1.0 - self.fRec111[1];
																																																																								fTemp244 = fTemp111 * fTemp243 + 1.0;
																																																																								fTemp245 = 1.0 - self.fRec110[1];
																																																																								fTemp246 = fTemp111 * fTemp245 + 1.0;
																																																																								fTemp247 = 1.0 - self.fRec109[1];
																																																																								fTemp248 = fTemp111 * fTemp247 + 1.0;
																																																																								fTemp249 = fTemp242 * fTemp248 * fTemp246;
																																																																								fTemp250 = 1.0 - self.fRec107[1];
																																																																								fTemp251 = 1.0 - self.fRec106[1];
																																																																								fTemp252 = 1.0 - self.fRec105[1];
																																																																								fTemp253 = 1.0 - self.fRec112[1];
																																																																								fTemp254 = 1.0 - self.fRec104[1];
																																																																								fTemp255 = fTemp244 * (4.2 * (fTemp128 * fTemp254 * fTemp127 * fTemp253 * fTemp252 * fTemp251 * fTemp250 / (fTemp249 * fTemp244)) + 1.0);
																																																																								fTemp256 = 1.47 * fTemp127 + 1.0;
																																																																								fTemp257 = fTemp111 * fTemp252;
																																																																								fTemp258 = fTemp111 * fTemp251;
																																																																								fTemp259 = fTemp111 * fTemp250;
																																																																								fTemp260 = self.fRec103[1] + fTemp259 * (self.fRec102[1] + fTemp258 * (self.fRec101[1] + fTemp257 * self.fRec100[1] / fTemp242) / fTemp248) / fTemp246 + 0.5 * (fTemp205 * fTemp254 * fTemp256 * fTemp252 * fTemp251 * fTemp250 / fTemp249);
																																																																								fTemp261 = 0.5 * fTemp204 * fTemp256 - 4.2 * (fTemp127 * fTemp253 * fTemp260 / fTemp255);
																																																																								fTemp262 = self.fRec100[1] + fTemp111 * fTemp254 * fTemp261;
																																																																								fTemp263 = fTemp262 / fTemp242;
																																																																								self.fRec100[0] = fTemp263 + fTemp111 * (fTemp254 * fTemp261 - fTemp241 * fTemp262 / fTemp242);
																																																																								fTemp264 = self.fRec101[1] + fTemp257 * fTemp262 / fTemp242;
																																																																								fTemp265 = fTemp264 / fTemp248;
																																																																								self.fRec101[0] = fTemp265 + fTemp111 * (fTemp252 * fTemp262 / fTemp242 - fTemp247 * fTemp264 / fTemp248);
																																																																								fTemp266 = self.fRec102[1] + fTemp258 * fTemp264 / fTemp248;
																																																																								fTemp267 = fTemp266 / fTemp246;
																																																																								self.fRec102[0] = fTemp267 + fTemp111 * (fTemp251 * fTemp264 / fTemp248 - fTemp245 * fTemp266 / fTemp246);
																																																																								fTemp268 = self.fRec103[1] + fTemp259 * fTemp266 / fTemp246;
																																																																								fTemp269 = fTemp268 / fTemp244;
																																																																								self.fRec103[0] = fTemp269 + fTemp111 * (fTemp250 * fTemp266 / fTemp246 - fTemp243 * fTemp268 / fTemp244);
																																																																								fTemp270 = VoiceDsp_faustpower2_f(fTemp261);
																																																																								fTemp271 = 2.56 * fTemp270;
																																																																								self.fRec104[0] = 1.0 - if (fTemp271 < 9.0) as i32 != 0 {(fTemp271 + 27.0) / (23.04 * fTemp270 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp261), 1e-09)};
																																																																								fTemp272 = VoiceDsp_faustpower2_f(fTemp262) / VoiceDsp_faustpower2_f(fTemp242);
																																																																								fTemp273 = 2.56 * fTemp272;
																																																																								fTemp274 = 1.0 - if (fTemp273 < 9.0) as i32 != 0 {(fTemp273 + 27.0) / (23.04 * fTemp272 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp263), 1e-09)};
																																																																								self.fRec105[0] = fTemp274;
																																																																								fTemp275 = VoiceDsp_faustpower2_f(fTemp264) / VoiceDsp_faustpower2_f(fTemp248);
																																																																								fTemp276 = 2.56 * fTemp275;
																																																																								fTemp277 = 1.0 - if (fTemp276 < 9.0) as i32 != 0 {(fTemp276 + 27.0) / (23.04 * fTemp275 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp265), 1e-09)};
																																																																								self.fRec106[0] = fTemp277;
																																																																								fTemp278 = VoiceDsp_faustpower2_f(fTemp266) / VoiceDsp_faustpower2_f(fTemp246);
																																																																								fTemp279 = 2.56 * fTemp278;
																																																																								fTemp280 = 1.0 - if (fTemp279 < 9.0) as i32 != 0 {(fTemp279 + 27.0) / (23.04 * fTemp278 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp267), 1e-09)};
																																																																								self.fRec107[0] = fTemp280;
																																																																								self.fRec108[0] = fTemp274;
																																																																								self.fRec109[0] = fTemp277;
																																																																								self.fRec110[0] = fTemp280;
																																																																								fTemp281 = VoiceDsp_faustpower2_f(fTemp268) / VoiceDsp_faustpower2_f(fTemp244);
																																																																								fTemp282 = 2.56 * fTemp281;
																																																																								self.fRec111[0] = 1.0 - if (fTemp282 < 9.0) as i32 != 0 {(fTemp282 + 27.0) / (23.04 * fTemp281 + 27.0)} else {1.0 / F32::max(F32::abs(1.6 * fTemp269), 1e-09)};
																																																																							}
																																																																								self.fRec112[0] = 0.0;
																																																																								if iSlow9 != 0 {
																																																																									fRec113 = fTemp260 / fTemp255;
																																																																									fTemp283 = fSlow165 * fRec113;
																																																																								}
																																																																									self.fVec29[0] = fSlow170;
																																																																									let mut fTemp284: F32 = if ((fSlow170 != self.fVec29[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec114[1] + -1.0};
																																																																									self.fRec114[0] = fTemp284;
																																																																									self.fRec115[0] = if iTemp0 != 0 {fSlow170} else {if (fTemp284 > 0.0) as i32 != 0 {self.fRec115[1] + (fSlow170 - self.fRec115[1]) / F32::max(1.0, fTemp284)} else {fSlow170}};
																																																																									let mut fTemp285: F32 = F32::max(0.0, F32::min(1.0, self.fRec115[0] + fSlow169 * self.fRec27[0]));
																																																																									if iSlow12 != 0 {
																																																																										fTemp286 = F32::max(2e+01, fTemp110);
																																																																										fTemp287 = F32::tan(self.fConst15 * fTemp286);
																																																																										fTemp288 = fTemp287 + 1.0 / F32::max(0.5, fTemp126 / (1.2 * VoiceDsp_faustpower2_f(F32::max(0.0, 1.0 - 0.0013333333 * fTemp286)) + 1.0));
																																																																										fTemp289 = fTemp287 * fTemp288 + 1.0;
																																																																										fTemp290 = F32::tanh(1.7 * self.fRec116[1] + 0.15) - self.fConst53;
																																																																										fTemp291 = fTemp204 - (self.fConst54 * fTemp288 * fTemp290 + self.fRec117[1]);
																																																																										fTemp292 = fTemp287 * fTemp291 / fTemp289;
																																																																										fTemp293 = self.fConst54 * fTemp290;
																																																																										self.fRec116[0] = fTemp293 + 2.0 * fTemp292;
																																																																										fTemp294 = fTemp293 + fTemp292;
																																																																										fTemp295 = fTemp287 * fTemp294;
																																																																										self.fRec117[0] = self.fRec117[1] + 2.0 * fTemp295;
																																																																										fRec118 = self.fRec117[1] + fTemp295;
																																																																										fRec119 = fTemp294;
																																																																										fRec120 = fTemp291 / fTemp289;
																																																																									}
																																																																										self.fVec30[0] = fSlow171;
																																																																										let mut fTemp296: F32 = if ((fSlow171 != self.fVec30[1]) as i32) | iTemp0 != 0 {fSlow2} else {self.fRec121[1] + -1.0};
																																																																										self.fRec121[0] = fTemp296;
																																																																										self.fRec122[0] = if iTemp0 != 0 {fSlow171} else {if (fTemp296 > 0.0) as i32 != 0 {self.fRec122[1] + (fSlow171 - self.fRec122[1]) / F32::max(1.0, fTemp296)} else {fSlow171}};
																																																																										if iSlow12 != 0 {
																																																																											fTemp297 = F32::max(0.0, F32::min(1.0, self.fRec122[0]));
																																																																											iTemp298 = (fTemp297 > 0.5) as i32;
																																																																											fTemp299 = fRec118 + fRec120;
																																																																											fTemp300 = 2.0 * fTemp297;
																																																																											fTemp301 = 1.0 - fTemp300;
																																																																											fTemp302 = fTemp300 + -1.0;
																																																																											fTemp303 = 1.0 - fTemp297;
																																																																											fTemp304 = 1.0 - fTemp285;
																																																																											fTemp305 = fTemp304 * if iTemp298 != 0 {2.0 * fTemp299 * fTemp303 + fRec120 * fTemp302} else {fRec118 * fTemp301 + 2.0 * fTemp299 * fTemp297} + fRec119 * fTemp285;
																																																																										}
																																																																											if iSlow172 & iSlow12 != 0 {
																																																																												fTemp306 = F32::tanh(1.7 * self.fRec123[1] + 0.15) - self.fConst53;
																																																																												fTemp307 = fTemp305 - (self.fConst54 * fTemp288 * fTemp306 + self.fRec124[1]);
																																																																												fTemp308 = fTemp287 * fTemp307 / fTemp289;
																																																																												fTemp309 = self.fConst54 * fTemp306;
																																																																												self.fRec123[0] = fTemp309 + 2.0 * fTemp308;
																																																																												fTemp310 = fTemp309 + fTemp308;
																																																																												fTemp311 = fTemp287 * fTemp310;
																																																																												self.fRec124[0] = self.fRec124[1] + 2.0 * fTemp311;
																																																																												fRec125 = self.fRec124[1] + fTemp311;
																																																																												fRec126 = fTemp310;
																																																																												fRec127 = fTemp307 / fTemp289;
																																																																												fTemp312 = fRec125 + fRec127;
																																																																												fTemp313 = fSlow167 * (fTemp304 * if iTemp298 != 0 {2.0 * fTemp312 * fTemp303 + fRec127 * fTemp302} else {fRec125 * fTemp301 + 2.0 * fTemp312 * fTemp297} + fRec126 * fTemp285);
																																																																											}
																																																																												if iSlow12 != 0 {
																																																																													fTemp314 = fSlow11 * if iSlow168 != 0 {fSlow173 * fTemp313} else {fTemp305};
																																																																												}
																																																																													let mut fTemp315: F32 = fSlow174 * fTemp314 + fSlow166 * fTemp283 + fSlow164 * fTemp240;
																																																																													if iTemp171 != 0 {
																																																																														fTemp316 = 0.8 * (fTemp170 * F32::tanh(1.25 * fTemp172 * fTemp315) * F32::powf(1e+01, 0.1 * self.fRec45[0]) / fTemp172);
																																																																													}
																																																																														let mut fTemp317: F32 = if iTemp103 != 0 {fTemp178 * fTemp316} else {fTemp315};
																																																																														self.fVec31[0] = fTemp317;
																																																																														self.fRec42[0] = -((self.fRec42[1] * (1.0 - fTemp102) - (fTemp317 - self.fVec31[1]) / fTemp101) / (fTemp102 + 1.0));
																																																																														*output0 = fSlow175 * ((self.fRec42[0] * self.fRec37[0] + fSlow98 * fTemp90) * (fTemp55 * (fTemp55 * (fTemp55 * (fTemp55 * (0.0013333 * fTemp55 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp54))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp54))) as F32);
																																																																														*output1 = fTemp109;
																																																																														*output2 = fTemp126;
																																																																														*output3 = self.fRec122[0];
																																																																														*output4 = fTemp285;
																																																																														*output5 = self.fRec44[0];
																																																																														*output6 = fTemp148;
																																																																														*output7 = self.fRec37[0];
																																																																														self.fVec0[1] = self.fVec0[0];
																																																																														self.iVec1[1] = self.iVec1[0];
																																																																														self.fVec2[1] = self.fVec2[0];
																																																																														self.fRec0[1] = self.fRec0[0];
																																																																														self.fRec1[1] = self.fRec1[0];
																																																																														if iSlow12 | (iSlow9 | iSlow6) != 0 {
																																																																															self.fRec4[1] = self.fRec4[0];
																																																																															self.fVec3[1] = self.fVec3[0];
																																																																															self.fVec4[1] = self.fVec4[0];
																																																																															self.fRec5[1] = self.fRec5[0];
																																																																															self.fRec6[1] = self.fRec6[0];
																																																																														}
																																																																															if (iSlow12 & iTemp40) | ((iSlow12 & iTemp37) | ((iSlow12 & iTemp34) | ((iSlow12 & iTemp31) | ((iSlow12 & iTemp28) | ((iSlow12 & iTemp25) | ((iSlow12 & iTemp22) | ((iSlow12 & iTemp19) | ((iSlow9 & iTemp40) | ((iSlow9 & iTemp37) | ((iSlow9 & iTemp34) | ((iSlow9 & iTemp31) | ((iSlow9 & iTemp28) | ((iSlow9 & iTemp25) | ((iSlow9 & iTemp22) | ((iSlow9 & iTemp19) | ((iSlow6 & iTemp40) | ((iSlow6 & iTemp37) | ((iSlow6 & iTemp34) | ((iSlow6 & iTemp31) | ((iSlow6 & iTemp28) | ((iSlow6 & iTemp25) | ((iSlow6 & iTemp22) | (iSlow6 & iTemp19))))))))))))))))))))))) != 0 {
																																																																																self.fRec7[1] = self.fRec7[0];
																																																																															}
																																																																																self.fVec5[1] = self.fVec5[0];
																																																																																if iSlow21 != 0 {
																																																																																	self.fVec6[1] = self.fVec6[0];
																																																																																	self.fRec8[1] = self.fRec8[0];
																																																																																	self.fRec9[1] = self.fRec9[0];
																																																																																	self.iRec10[1] = self.iRec10[0];
																																																																																	self.iRec11[1] = self.iRec11[0];
																																																																																}
																																																																																	if iTemp4 != 0 {
																																																																																		self.iRec3[1] = self.iRec3[0];
																																																																																		self.fRec2[1] = self.fRec2[0];
																																																																																		self.fRec12[1] = self.fRec12[0];
																																																																																	}
																																																																																		if (iSlow21 & (iSlow31 & (iSlow50 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow47 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow37 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow34 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32)))) | (iSlow21 & (iSlow43 & ((fTemp48 != 0.0) as i32))))))) != 0 {
																																																																																			self.fVec7[1] = self.fVec7[0];
																																																																																			self.fRec14[1] = self.fRec14[0];
																																																																																			self.fRec15[1] = self.fRec15[0];
																																																																																		}
																																																																																			if iTemp4 != 0 {
																																																																																				self.fRec19[1] = self.fRec19[0];
																																																																																				self.fRec20[1] = self.fRec20[0];
																																																																																				self.iRec22[1] = self.iRec22[0];
																																																																																				self.fRec21[1] = self.fRec21[0];
																																																																																				self.fRec25[1] = self.fRec25[0];
																																																																																			}
																																																																																				if iTemp4 | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp19)) | ((((fTemp11 != 0.0) as i32) & (iSlow74 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow12)) | ((iSlow21 & (iSlow31 & (iSlow47 & ((fTemp48 != 0.0) as i32)))) | ((iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32)))) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp19)) | ((((fTemp11 != 0.0) as i32) & (iSlow74 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow9)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp40)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp37)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp34)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp31)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp28)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp25)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp22)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp19)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow74)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow71)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow68)))))))))))))))))))))))))))))))))))) != 0 {
																																																																																					self.iRec26[1] = self.iRec26[0];
																																																																																				}
																																																																																					if iTemp4 != 0 {
																																																																																						self.fRec24[1] = self.fRec24[0];
																																																																																						self.fVec9[1] = self.fVec9[0];
																																																																																						self.fRec23[1] = self.fRec23[0];
																																																																																					}
																																																																																						self.fRec27[1] = self.fRec27[0];
																																																																																						self.fVec10[1] = self.fVec10[0];
																																																																																						self.fRec28[1] = self.fRec28[0];
																																																																																						self.fRec29[1] = self.fRec29[0];
																																																																																						self.fRec30[1] = self.fRec30[0];
																																																																																						if iSlow21 & (iSlow31 & (iSlow27 & ((fTemp48 != 0.0) as i32))) != 0 {
																																																																																							self.fRec17[1] = self.fRec17[0];
																																																																																							self.fRec16[1] = self.fRec16[0];
																																																																																						}
																																																																																							if iSlow21 & (iSlow31 & (iSlow34 & ((fTemp48 != 0.0) as i32))) != 0 {
																																																																																								self.fRec32[1] = self.fRec32[0];
																																																																																								self.fRec31[1] = self.fRec31[0];
																																																																																							}
																																																																																								if iSlow21 & (iSlow31 & (iSlow37 & ((fTemp48 != 0.0) as i32))) != 0 {
																																																																																									self.fRec33[1] = self.fRec33[0];
																																																																																								}
																																																																																									if iSlow21 & (iSlow31 & (iSlow47 & ((fTemp48 != 0.0) as i32))) != 0 {
																																																																																										self.fRec34[2] = self.fRec34[1];
																																																																																										self.fRec34[1] = self.fRec34[0];
																																																																																									}
																																																																																										if iSlow21 & ((fTemp48 != 0.0) as i32) != 0 {
																																																																																											self.fVec11[1] = self.fVec11[0];
																																																																																											self.fRec13[1] = self.fRec13[0];
																																																																																										}
																																																																																											self.fRec35[1] = self.fRec35[0];
																																																																																											self.fRec36[1] = self.fRec36[0];
																																																																																											self.fRec38[1] = self.fRec38[0];
																																																																																											self.fRec39[1] = self.fRec39[0];
																																																																																											self.fRec40[1] = self.fRec40[0];
																																																																																											self.fRec41[1] = self.fRec41[0];
																																																																																											self.fRec37[1] = self.fRec37[0];
																																																																																											self.fVec12[1] = self.fVec12[0];
																																																																																											self.fRec43[1] = self.fRec43[0];
																																																																																											self.fRec44[1] = self.fRec44[0];
																																																																																											self.fRec45[1] = self.fRec45[0];
																																																																																											if iTemp4 != 0 {
																																																																																												self.fRec62[1] = self.fRec62[0];
																																																																																												self.fRec61[1] = self.fRec61[0];
																																																																																												self.fVec13[1] = self.fVec13[0];
																																																																																												self.fRec60[1] = self.fRec60[0];
																																																																																											}
																																																																																												self.fVec14[1] = self.fVec14[0];
																																																																																												self.fRec63[1] = self.fRec63[0];
																																																																																												self.fRec64[1] = self.fRec64[0];
																																																																																												self.fVec15[1] = self.fVec15[0];
																																																																																												self.fRec65[1] = self.fRec65[0];
																																																																																												self.fRec66[1] = self.fRec66[0];
																																																																																												if iSlow12 | (iSlow9 | iSlow6) != 0 {
																																																																																													self.fRec67[1] = self.fRec67[0];
																																																																																													self.fRec68[1] = self.fRec68[0];
																																																																																													self.fVec16[1] = self.fVec16[0];
																																																																																													self.fRec69[1] = self.fRec69[0];
																																																																																													self.fRec70[1] = self.fRec70[0];
																																																																																													self.fVec17[1] = self.fVec17[0];
																																																																																													self.fRec71[1] = self.fRec71[0];
																																																																																													self.fRec72[1] = self.fRec72[0];
																																																																																												}
																																																																																													if (((fTemp11 != 0.0) as i32) & iSlow12) | ((((fTemp11 != 0.0) as i32) & iSlow9) | (iSlow6 & ((fTemp11 != 0.0) as i32))) != 0 {
																																																																																														self.fVec18[1] = self.fVec18[0];
																																																																																														self.fRec74[1] = self.fRec74[0];
																																																																																														self.fRec75[1] = self.fRec75[0];
																																																																																														self.fRec77[1] = self.fRec77[0];
																																																																																														self.iRec78[1] = self.iRec78[0];
																																																																																														self.fRec76[1] = self.fRec76[0];
																																																																																														self.fRec73[1] = self.fRec73[0];
																																																																																														self.fVec19[1] = self.fVec19[0];
																																																																																														self.fRec79[1] = self.fRec79[0];
																																																																																														self.fRec80[1] = self.fRec80[0];
																																																																																														self.fVec20[1] = self.fVec20[0];
																																																																																														self.fRec81[1] = self.fRec81[0];
																																																																																														self.fRec82[1] = self.fRec82[0];
																																																																																													}
																																																																																														if (((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow68 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow68))) != 0 {
																																																																																															self.fVec21[1] = self.fVec21[0];
																																																																																															self.fRec87[1] = self.fRec87[0];
																																																																																															self.fRec88[1] = self.fRec88[0];
																																																																																															self.fVec22[1] = self.fVec22[0];
																																																																																															self.fRec86[1] = self.fRec86[0];
																																																																																															self.fRec85[1] = self.fRec85[0];
																																																																																														}
																																																																																															if (((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow153 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow150 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow153 & iSlow9)) | ((((fTemp11 != 0.0) as i32) & (iSlow150 & iSlow9)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow71)) | ((iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow153)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow150))))))))) != 0 {
																																																																																																self.fRec90[1] = self.fRec90[0];
																																																																																															}
																																																																																																if (((fTemp11 != 0.0) as i32) & (iSlow150 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow150 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow150))) != 0 {
																																																																																																	self.iRec89[1] = self.iRec89[0];
																																																																																																}
																																																																																																	if (((fTemp11 != 0.0) as i32) & (iSlow153 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow153 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow153))) != 0 {
																																																																																																		self.iRec91[1] = self.iRec91[0];
																																																																																																	}
																																																																																																		if (((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow12)) | ((((fTemp11 != 0.0) as i32) & (iSlow71 & iSlow9)) | (iSlow6 & (((fTemp11 != 0.0) as i32) & iSlow71))) != 0 {
																																																																																																			self.fRec92[1] = self.fRec92[0];
																																																																																																		}
																																																																																																			if (((fTemp11 != 0.0) as i32) & iSlow12) | ((((fTemp11 != 0.0) as i32) & iSlow9) | (iSlow6 & ((fTemp11 != 0.0) as i32))) != 0 {
																																																																																																				self.fVec23[1] = self.fVec23[0];
																																																																																																				self.fRec84[1] = self.fRec84[0];
																																																																																																				self.fVec24[1] = self.fVec24[0];
																																																																																																				self.fRec83[1] = self.fRec83[0];
																																																																																																			}
																																																																																																				if (((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow12 & iTemp19)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp40)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp37)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp34)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp31)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp28)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp25)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp22)) | ((((fTemp42 != 0.0) as i32) & (iSlow9 & iTemp19)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp40)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp37)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp34)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp31)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp28)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp25)) | ((iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp22)) | (iSlow6 & (((fTemp42 != 0.0) as i32) & iTemp19)))))))))))))))))))))))) != 0 {
																																																																																																					self.fRec96[1] = self.fRec96[0];
																																																																																																					self.fRec95[1] = self.fRec95[0];
																																																																																																					self.fVec25[1] = self.fVec25[0];
																																																																																																					self.fRec94[1] = self.fRec94[0];
																																																																																																				}
																																																																																																					if (iTemp171 & (iSlow12 & iTemp40)) | ((iTemp171 & (iSlow12 & iTemp37)) | ((iTemp171 & (iSlow12 & iTemp34)) | ((iTemp171 & (iSlow12 & iTemp31)) | ((iTemp171 & (iSlow12 & iTemp28)) | ((iTemp171 & (iSlow12 & iTemp25)) | ((iTemp171 & (iSlow12 & iTemp22)) | ((iTemp171 & (iSlow12 & iTemp19)) | ((iTemp171 & (iSlow9 & iTemp40)) | ((iTemp171 & (iSlow9 & iTemp37)) | ((iTemp171 & (iSlow9 & iTemp34)) | ((iTemp171 & (iSlow9 & iTemp31)) | ((iTemp171 & (iSlow9 & iTemp28)) | ((iTemp171 & (iSlow9 & iTemp25)) | ((iTemp171 & (iSlow9 & iTemp22)) | ((iTemp171 & (iSlow9 & iTemp19)) | ((iTemp171 & (iSlow6 & iTemp40)) | ((iTemp171 & (iSlow6 & iTemp37)) | ((iTemp171 & (iSlow6 & iTemp34)) | ((iTemp171 & (iSlow6 & iTemp31)) | ((iTemp171 & (iSlow6 & iTemp28)) | ((iTemp171 & (iSlow6 & iTemp25)) | ((iTemp171 & (iSlow6 & iTemp22)) | (iTemp171 & (iSlow6 & iTemp19)))))))))))))))))))))))) != 0 {
																																																																																																						self.fVec26[1] = self.fVec26[0];
																																																																																																						self.fRec98[1] = self.fRec98[0];
																																																																																																						self.fRec99[1] = self.fRec99[0];
																																																																																																						self.fVec27[1] = self.fVec27[0];
																																																																																																						self.fRec97[1] = self.fRec97[0];
																																																																																																					}
																																																																																																						if iSlow12 | (iSlow9 | iSlow6) != 0 {
																																																																																																							self.fVec28[1] = self.fVec28[0];
																																																																																																							self.fRec93[1] = self.fRec93[0];
																																																																																																						}
																																																																																																							if iSlow6 != 0 {
																																																																																																								self.fRec46[1] = self.fRec46[0];
																																																																																																								self.fRec47[1] = self.fRec47[0];
																																																																																																								self.fRec48[1] = self.fRec48[0];
																																																																																																								self.fRec49[1] = self.fRec49[0];
																																																																																																								self.fRec50[1] = self.fRec50[0];
																																																																																																								self.fRec51[1] = self.fRec51[0];
																																																																																																								self.fRec52[1] = self.fRec52[0];
																																																																																																								self.fRec53[1] = self.fRec53[0];
																																																																																																								self.fRec54[1] = self.fRec54[0];
																																																																																																								self.fRec55[1] = self.fRec55[0];
																																																																																																								self.fRec56[1] = self.fRec56[0];
																																																																																																								self.fRec57[1] = self.fRec57[0];
																																																																																																								self.fRec58[1] = self.fRec58[0];
																																																																																																							}
																																																																																																								if iSlow9 != 0 {
																																																																																																									self.fRec100[1] = self.fRec100[0];
																																																																																																									self.fRec101[1] = self.fRec101[0];
																																																																																																									self.fRec102[1] = self.fRec102[0];
																																																																																																									self.fRec103[1] = self.fRec103[0];
																																																																																																									self.fRec104[1] = self.fRec104[0];
																																																																																																									self.fRec105[1] = self.fRec105[0];
																																																																																																									self.fRec106[1] = self.fRec106[0];
																																																																																																									self.fRec107[1] = self.fRec107[0];
																																																																																																									self.fRec108[1] = self.fRec108[0];
																																																																																																									self.fRec109[1] = self.fRec109[0];
																																																																																																									self.fRec110[1] = self.fRec110[0];
																																																																																																									self.fRec111[1] = self.fRec111[0];
																																																																																																								}
																																																																																																									self.fRec112[1] = self.fRec112[0];
																																																																																																									self.fVec29[1] = self.fVec29[0];
																																																																																																									self.fRec114[1] = self.fRec114[0];
																																																																																																									self.fRec115[1] = self.fRec115[0];
																																																																																																									if iSlow12 != 0 {
																																																																																																										self.fRec116[1] = self.fRec116[0];
																																																																																																										self.fRec117[1] = self.fRec117[0];
																																																																																																									}
																																																																																																										self.fVec30[1] = self.fVec30[0];
																																																																																																										self.fRec121[1] = self.fRec121[0];
																																																																																																										self.fRec122[1] = self.fRec122[0];
																																																																																																										if iSlow172 & iSlow12 != 0 {
																																																																																																											self.fRec123[1] = self.fRec123[0];
																																																																																																											self.fRec124[1] = self.fRec124[0];
																																																																																																										}
																																																																																																											self.fVec31[1] = self.fVec31[0];
																																																																																																											self.fRec42[1] = self.fRec42[0];
																																																																																																										}
																																																																																																										self.fTempPerm0 = fTemp9;
																																																																																																										self.iTempPerm1 = iTemp10;
																																																																																																										self.fTempPerm2 = fTemp11;
																																																																																																										self.fTempPerm3 = fTemp12;
																																																																																																										self.fTempPerm4 = fTemp13;
																																																																																																										self.fTempPerm5 = fTemp14;
																																																																																																										self.fTempPerm6 = fTemp15;
																																																																																																										self.fTempPerm7 = fTemp16;
																																																																																																										self.iTempPerm8 = iTemp17;
																																																																																																										self.fTempPerm9 = fTemp18;
																																																																																																										self.iTempPerm10 = iTemp19;
																																																																																																										self.iTempPerm11 = iTemp20;
																																																																																																										self.fTempPerm12 = fTemp21;
																																																																																																										self.iTempPerm13 = iTemp22;
																																																																																																										self.iTempPerm14 = iTemp23;
																																																																																																										self.fTempPerm15 = fTemp24;
																																																																																																										self.iTempPerm16 = iTemp25;
																																																																																																										self.iTempPerm17 = iTemp26;
																																																																																																										self.fTempPerm18 = fTemp27;
																																																																																																										self.iTempPerm19 = iTemp28;
																																																																																																										self.iTempPerm20 = iTemp29;
																																																																																																										self.fTempPerm21 = fTemp30;
																																																																																																										self.iTempPerm22 = iTemp31;
																																																																																																										self.iTempPerm23 = iTemp32;
																																																																																																										self.fTempPerm24 = fTemp33;
																																																																																																										self.iTempPerm25 = iTemp34;
																																																																																																										self.iTempPerm26 = iTemp35;
																																																																																																										self.fTempPerm27 = fTemp36;
																																																																																																										self.iTempPerm28 = iTemp37;
																																																																																																										self.iTempPerm29 = iTemp38;
																																																																																																										self.fTempPerm30 = fTemp39;
																																																																																																										self.iTempPerm31 = iTemp40;
																																																																																																										self.iTempPerm32 = iTemp41;
																																																																																																										self.fTempPerm33 = fTemp42;
																																																																																																										self.fTempPerm34 = fTemp43;
																																																																																																										self.fTempPerm35 = fTemp44;
																																																																																																										self.fTempPerm36 = fTemp45;
																																																																																																										self.fTempPerm37 = fTemp46;
																																																																																																										self.iTempPerm38 = iTemp47;
																																																																																																										self.fTempPerm39 = fTemp48;
																																																																																																										self.iTempPerm40 = iTemp5;
																																																																																																										self.iTempPerm41 = iTemp49;
																																																																																																										self.fTempPerm42 = fTemp50;
																																																																																																										self.fTempPerm43 = fTemp56;
																																																																																																										self.fTempPerm44 = fTemp57;
																																																																																																										self.fTempPerm45 = fTemp58;
																																																																																																										self.fTempPerm46 = fTemp59;
																																																																																																										self.fTempPerm47 = fTemp60;
																																																																																																										self.fTempPerm48 = fTemp61;
																																																																																																										self.iTempPerm49 = iTemp62;
																																																																																																										self.fTempPerm50 = fTemp63;
																																																																																																										self.fTempPerm51 = fTemp64;
																																																																																																										self.fTempPerm52 = fTemp65;
																																																																																																										self.fTempPerm53 = fTemp66;
																																																																																																										self.fTempPerm54 = fTemp67;
																																																																																																										self.fTempPerm55 = fTemp71;
																																																																																																										self.fTempPerm56 = fTemp72;
																																																																																																										self.fTempPerm57 = fTemp73;
																																																																																																										self.fTempPerm58 = fTemp74;
																																																																																																										self.fTempPerm59 = fTemp75;
																																																																																																										self.fTempPerm60 = fTemp76;
																																																																																																										self.fTempPerm61 = fTemp77;
																																																																																																										self.fTempPerm62 = fTemp78;
																																																																																																										self.fTempPerm63 = fTemp79;
																																																																																																										self.fTempPerm64 = fTemp80;
																																																																																																										self.fTempPerm65 = fTemp81;
																																																																																																										self.fTempPerm66 = fTemp82;
																																																																																																										self.fTempPerm67 = fTemp83;
																																																																																																										self.fTempPerm68 = fTemp84;
																																																																																																										self.fTempPerm69 = fTemp85;
																																																																																																										self.fTempPerm70 = fTemp86;
																																																																																																										self.fTempPerm71 = fTemp87;
																																																																																																										self.fTempPerm72 = fTemp88;
																																																																																																										self.fTempPerm73 = fTemp104;
																																																																																																										self.fTempPerm74 = fTemp105;
																																																																																																										self.fTempPerm75 = fTemp106;
																																																																																																										self.fTempPerm76 = fTemp107;
																																																																																																										self.fTempPerm77 = fTemp110;
																																																																																																										self.fTempPerm78 = fTemp111;
																																																																																																										self.fTempPerm79 = fTemp112;
																																																																																																										self.fTempPerm80 = fTemp113;
																																																																																																										self.fTempPerm81 = fTemp114;
																																																																																																										self.fTempPerm82 = fTemp115;
																																																																																																										self.fTempPerm83 = fTemp116;
																																																																																																										self.fTempPerm84 = fTemp117;
																																																																																																										self.fTempPerm85 = fTemp118;
																																																																																																										self.fTempPerm86 = fTemp119;
																																																																																																										self.fTempPerm87 = fTemp120;
																																																																																																										self.fTempPerm88 = fTemp121;
																																																																																																										self.fTempPerm89 = fTemp122;
																																																																																																										self.fTempPerm90 = fTemp123;
																																																																																																										self.fTempPerm91 = fTemp124;
																																																																																																										self.fTempPerm92 = fTemp127;
																																																																																																										self.fTempPerm93 = fTemp128;
																																																																																																										self.fTempPerm94 = fTemp129;
																																																																																																										self.fTempPerm95 = fTemp130;
																																																																																																										self.fTempPerm96 = fTemp131;
																																																																																																										self.fTempPerm97 = fTemp132;
																																																																																																										self.fTempPerm98 = fTemp133;
																																																																																																										self.fTempPerm99 = fTemp134;
																																																																																																										self.fTempPerm100 = fTemp135;
																																																																																																										self.fTempPerm101 = fTemp136;
																																																																																																										self.iTempPerm102 = iTemp137;
																																																																																																										self.fTempPerm103 = fTemp138;
																																																																																																										self.fTempPerm104 = fTemp139;
																																																																																																										self.fTempPerm105 = fTemp140;
																																																																																																										self.iTempPerm106 = iTemp141;
																																																																																																										self.fTempPerm107 = fTemp142;
																																																																																																										self.fTempPerm108 = fTemp143;
																																																																																																										self.fTempPerm109 = fTemp144;
																																																																																																										self.fTempPerm110 = fTemp145;
																																																																																																										self.fTempPerm111 = fTemp146;
																																																																																																										self.fTempPerm112 = fTemp147;
																																																																																																										self.fTempPerm113 = fTemp149;
																																																																																																										self.iTempPerm114 = iTemp150;
																																																																																																										self.iTempPerm115 = iTemp151;
																																																																																																										self.fTempPerm116 = fTemp152;
																																																																																																										self.iTempPerm117 = iTemp153;
																																																																																																										self.iTempPerm118 = iTemp154;
																																																																																																										self.fTempPerm119 = fTemp155;
																																																																																																										self.fTempPerm120 = fTemp156;
																																																																																																										self.fTempPerm121 = fTemp157;
																																																																																																										self.fTempPerm122 = fTemp158;
																																																																																																										self.fTempPerm123 = fTemp159;
																																																																																																										self.fTempPerm124 = fTemp160;
																																																																																																										self.fTempPerm125 = fTemp161;
																																																																																																										self.fTempPerm126 = fTemp162;
																																																																																																										self.fTempPerm127 = fTemp163;
																																																																																																										self.fTempPerm128 = fTemp164;
																																																																																																										self.fTempPerm129 = fTemp165;
																																																																																																										self.fTempPerm130 = fTemp166;
																																																																																																										self.fTempPerm131 = fTemp167;
																																																																																																										self.fTempPerm132 = fTemp168;
																																																																																																										self.fTempPerm133 = fTemp169;
																																																																																																										self.fTempPerm134 = fTemp172;
																																																																																																										self.fTempPerm135 = fTemp173;
																																																																																																										self.fTempPerm136 = fTemp174;
																																																																																																										self.fTempPerm137 = fTemp175;
																																																																																																										self.fTempPerm138 = fTemp176;
																																																																																																										self.fTempPerm139 = fTemp177;
																																																																																																										self.fTempPerm140 = fTemp179;
																																																																																																										self.fTempPerm141 = fTemp180;
																																																																																																										self.fTempPerm142 = fTemp181;
																																																																																																										self.fTempPerm143 = fTemp182;
																																																																																																										self.fTempPerm144 = fTemp183;
																																																																																																										self.fTempPerm145 = fTemp184;
																																																																																																										self.fTempPerm146 = fTemp185;
																																																																																																										self.fTempPerm147 = fTemp186;
																																																																																																										self.fTempPerm148 = fTemp187;
																																																																																																										self.fTempPerm149 = fTemp188;
																																																																																																										self.fTempPerm150 = fTemp189;
																																																																																																										self.fTempPerm151 = fTemp190;
																																																																																																										self.fTempPerm152 = fTemp191;
																																																																																																										self.fTempPerm153 = fTemp192;
																																																																																																										self.fTempPerm154 = fTemp193;
																																																																																																										self.fTempPerm155 = fTemp194;
																																																																																																										self.fTempPerm156 = fTemp195;
																																																																																																										self.fTempPerm157 = fTemp196;
																																																																																																										self.fTempPerm158 = fTemp197;
																																																																																																										self.fTempPerm159 = fTemp198;
																																																																																																										self.fTempPerm160 = fTemp199;
																																																																																																										self.fTempPerm161 = fTemp200;
																																																																																																										self.fTempPerm162 = fTemp201;
																																																																																																										self.fTempPerm163 = fTemp202;
																																																																																																										self.fTempPerm164 = fTemp203;
																																																																																																										self.fTempPerm165 = fTemp204;
																																																																																																										self.fTempPerm166 = fTemp205;
																																																																																																										self.fTempPerm167 = fTemp206;
																																																																																																										self.fTempPerm168 = fTemp207;
																																																																																																										self.fTempPerm169 = fTemp208;
																																																																																																										self.fTempPerm170 = fTemp209;
																																																																																																										self.fTempPerm171 = fTemp210;
																																																																																																										self.fTempPerm172 = fTemp211;
																																																																																																										self.fTempPerm173 = fTemp212;
																																																																																																										self.fTempPerm174 = fTemp213;
																																																																																																										self.fTempPerm175 = fTemp214;
																																																																																																										self.fTempPerm176 = fTemp215;
																																																																																																										self.fTempPerm177 = fTemp216;
																																																																																																										self.fTempPerm178 = fTemp217;
																																																																																																										self.fTempPerm179 = fTemp218;
																																																																																																										self.fTempPerm180 = fTemp219;
																																																																																																										self.fTempPerm181 = fTemp220;
																																																																																																										self.fTempPerm182 = fTemp221;
																																																																																																										self.fTempPerm183 = fTemp222;
																																																																																																										self.fTempPerm184 = fTemp223;
																																																																																																										self.fTempPerm185 = fTemp224;
																																																																																																										self.fTempPerm186 = fTemp225;
																																																																																																										self.fTempPerm187 = fTemp226;
																																																																																																										self.fTempPerm188 = fTemp227;
																																																																																																										self.fTempPerm189 = fTemp228;
																																																																																																										self.fTempPerm190 = fTemp229;
																																																																																																										self.fTempPerm191 = fTemp230;
																																																																																																										self.fTempPerm192 = fTemp231;
																																																																																																										self.fTempPerm193 = fTemp232;
																																																																																																										self.fTempPerm194 = fTemp233;
																																																																																																										self.fTempPerm195 = fTemp234;
																																																																																																										self.fTempPerm196 = fTemp235;
																																																																																																										self.fTempPerm197 = fTemp236;
																																																																																																										self.fTempPerm198 = fTemp237;
																																																																																																										self.fTempPerm199 = fTemp238;
																																																																																																										self.fTempPerm200 = fTemp239;
																																																																																																										self.fTempPerm201 = fTemp240;
																																																																																																										self.fTempPerm202 = fTemp241;
																																																																																																										self.fTempPerm203 = fTemp242;
																																																																																																										self.fTempPerm204 = fTemp243;
																																																																																																										self.fTempPerm205 = fTemp244;
																																																																																																										self.fTempPerm206 = fTemp245;
																																																																																																										self.fTempPerm207 = fTemp246;
																																																																																																										self.fTempPerm208 = fTemp247;
																																																																																																										self.fTempPerm209 = fTemp248;
																																																																																																										self.fTempPerm210 = fTemp249;
																																																																																																										self.fTempPerm211 = fTemp250;
																																																																																																										self.fTempPerm212 = fTemp251;
																																																																																																										self.fTempPerm213 = fTemp252;
																																																																																																										self.fTempPerm214 = fTemp253;
																																																																																																										self.fTempPerm215 = fTemp254;
																																																																																																										self.fTempPerm216 = fTemp255;
																																																																																																										self.fTempPerm217 = fTemp256;
																																																																																																										self.fTempPerm218 = fTemp257;
																																																																																																										self.fTempPerm219 = fTemp258;
																																																																																																										self.fTempPerm220 = fTemp259;
																																																																																																										self.fTempPerm221 = fTemp260;
																																																																																																										self.fTempPerm222 = fTemp261;
																																																																																																										self.fTempPerm223 = fTemp262;
																																																																																																										self.fTempPerm224 = fTemp263;
																																																																																																										self.fTempPerm225 = fTemp264;
																																																																																																										self.fTempPerm226 = fTemp265;
																																																																																																										self.fTempPerm227 = fTemp266;
																																																																																																										self.fTempPerm228 = fTemp267;
																																																																																																										self.fTempPerm229 = fTemp268;
																																																																																																										self.fTempPerm230 = fTemp269;
																																																																																																										self.fTempPerm231 = fTemp270;
																																																																																																										self.fTempPerm232 = fTemp271;
																																																																																																										self.fTempPerm233 = fTemp272;
																																																																																																										self.fTempPerm234 = fTemp273;
																																																																																																										self.fTempPerm235 = fTemp274;
																																																																																																										self.fTempPerm236 = fTemp275;
																																																																																																										self.fTempPerm237 = fTemp276;
																																																																																																										self.fTempPerm238 = fTemp277;
																																																																																																										self.fTempPerm239 = fTemp278;
																																																																																																										self.fTempPerm240 = fTemp279;
																																																																																																										self.fTempPerm241 = fTemp280;
																																																																																																										self.fTempPerm242 = fTemp281;
																																																																																																										self.fTempPerm243 = fTemp282;
																																																																																																										self.fTempPerm244 = fTemp283;
																																																																																																										self.fTempPerm245 = fTemp286;
																																																																																																										self.fTempPerm246 = fTemp287;
																																																																																																										self.fTempPerm247 = fTemp288;
																																																																																																										self.fTempPerm248 = fTemp289;
																																																																																																										self.fTempPerm249 = fTemp290;
																																																																																																										self.fTempPerm250 = fTemp291;
																																																																																																										self.fTempPerm251 = fTemp292;
																																																																																																										self.fTempPerm252 = fTemp293;
																																																																																																										self.fTempPerm253 = fTemp294;
																																																																																																										self.fTempPerm254 = fTemp295;
																																																																																																										self.fTempPerm255 = fTemp297;
																																																																																																										self.iTempPerm256 = iTemp298;
																																																																																																										self.fTempPerm257 = fTemp299;
																																																																																																										self.fTempPerm258 = fTemp300;
																																																																																																										self.fTempPerm259 = fTemp301;
																																																																																																										self.fTempPerm260 = fTemp302;
																																																																																																										self.fTempPerm261 = fTemp303;
																																																																																																										self.fTempPerm262 = fTemp304;
																																																																																																										self.fTempPerm263 = fTemp305;
																																																																																																										self.fTempPerm264 = fTemp306;
																																																																																																										self.fTempPerm265 = fTemp307;
																																																																																																										self.fTempPerm266 = fTemp308;
																																																																																																										self.fTempPerm267 = fTemp309;
																																																																																																										self.fTempPerm268 = fTemp310;
																																																																																																										self.fTempPerm269 = fTemp311;
																																																																																																										self.fTempPerm270 = fTemp312;
																																																																																																										self.fTempPerm271 = fTemp313;
																																																																																																										self.fTempPerm272 = fTemp314;
																																																																																																										self.fTempPerm273 = fTemp316;
																																																																																																									}

}

