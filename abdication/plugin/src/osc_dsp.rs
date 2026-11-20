/* ------------------------------------------------------------
name: "plugin-osc"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn OscDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */


pub struct OscDspSIG0 {
	iVec4: [i32;2],
	iRec5: [i32;2],
}

impl OscDspSIG0 {
	
	fn get_num_inputsOscDspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG0(&mut self, sample_rate: i32) {
		for l8 in 0..2 {
			self.iVec4[l8 as usize] = 0;
		}
		for l9 in 0..2 {
			self.iRec5[l9 as usize] = 0;
		}
	}
	
	fn fillOscDspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec4[0] = 1;
			self.iRec5[0] = (i32::wrapping_add(self.iVec4[1], self.iRec5[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec5[0]) as F32);
			self.iVec4[1] = self.iVec4[0];
			self.iRec5[1] = self.iRec5[0];
		}
	}

}


pub fn newOscDspSIG0() -> OscDspSIG0 { 
	OscDspSIG0 {
		iVec4: [0;2],
		iRec5: [0;2],
	}
}
static mut fOscDspSIG1Wave0: [F32;12] = [0.5,1.0,1.5,2.0,2.5,3.0,3.5,4.0,5.0,6.0,7.0,8.0];

pub struct OscDspSIG1 {
	fOscDspSIG1Wave0_idx: i32,
}

impl OscDspSIG1 {
	
	fn get_num_inputsOscDspSIG1(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG1(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG1(&mut self, sample_rate: i32) {
		self.fOscDspSIG1Wave0_idx = 0;
	}
	
	fn fillOscDspSIG1(&mut self, count: i32, table: &mut[F32]) {
		for i2 in 0..count {
			table[i2 as usize] = unsafe { fOscDspSIG1Wave0[self.fOscDspSIG1Wave0_idx as usize] };
			self.fOscDspSIG1Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG1Wave0_idx)) % 12;
		}
	}

}


pub fn newOscDspSIG1() -> OscDspSIG1 { 
	OscDspSIG1 {
		fOscDspSIG1Wave0_idx: 0,
	}
}
static mut fOscDspSIG2Wave0: [F32;10] = [0.5,1.0,1.5,2.0,3.0,4.0,5.0,6.0,7.0,8.0];

pub struct OscDspSIG2 {
	fOscDspSIG2Wave0_idx: i32,
}

impl OscDspSIG2 {
	
	fn get_num_inputsOscDspSIG2(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG2(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG2(&mut self, sample_rate: i32) {
		self.fOscDspSIG2Wave0_idx = 0;
	}
	
	fn fillOscDspSIG2(&mut self, count: i32, table: &mut[F32]) {
		for i3 in 0..count {
			table[i3 as usize] = unsafe { fOscDspSIG2Wave0[self.fOscDspSIG2Wave0_idx as usize] };
			self.fOscDspSIG2Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG2Wave0_idx)) % 10;
		}
	}

}


pub fn newOscDspSIG2() -> OscDspSIG2 { 
	OscDspSIG2 {
		fOscDspSIG2Wave0_idx: 0,
	}
}
static mut iOscDspSIG3Wave0: [i32;5] = [3,4,5,7,12];

pub struct OscDspSIG3 {
	iOscDspSIG3Wave0_idx: i32,
}

impl OscDspSIG3 {
	
	fn get_num_inputsOscDspSIG3(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG3(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG3(&mut self, sample_rate: i32) {
		self.iOscDspSIG3Wave0_idx = 0;
	}
	
	fn fillOscDspSIG3(&mut self, count: i32, table: &mut[i32]) {
		for i4 in 0..count {
			table[i4 as usize] = unsafe { iOscDspSIG3Wave0[self.iOscDspSIG3Wave0_idx as usize] };
			self.iOscDspSIG3Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG3Wave0_idx)) % 5;
		}
	}

}


pub fn newOscDspSIG3() -> OscDspSIG3 { 
	OscDspSIG3 {
		iOscDspSIG3Wave0_idx: 0,
	}
}
static mut fOscDspSIG4Wave0: [F32;12] = [1.07,0.76,1.24,1.26,0.98,1.13,1.43,1.72,0.77,0.88,0.88,0.76];

pub struct OscDspSIG4 {
	fOscDspSIG4Wave0_idx: i32,
}

impl OscDspSIG4 {
	
	fn get_num_inputsOscDspSIG4(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG4(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG4(&mut self, sample_rate: i32) {
		self.fOscDspSIG4Wave0_idx = 0;
	}
	
	fn fillOscDspSIG4(&mut self, count: i32, table: &mut[F32]) {
		for i5 in 0..count {
			table[i5 as usize] = unsafe { fOscDspSIG4Wave0[self.fOscDspSIG4Wave0_idx as usize] };
			self.fOscDspSIG4Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG4Wave0_idx)) % 12;
		}
	}

}


pub fn newOscDspSIG4() -> OscDspSIG4 { 
	OscDspSIG4 {
		fOscDspSIG4Wave0_idx: 0,
	}
}
static mut ftbl0OscDspSIG0: [F32;65536] = [0.0;65536];
fn OscDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut ftbl1OscDspSIG1: [F32;12] = [0.0;12];
static mut ftbl2OscDspSIG2: [F32;10] = [0.0;10];
static mut itbl3OscDspSIG3: [i32;5] = [0;5];
static mut ftbl4OscDspSIG4: [F32;12] = [0.0;12];

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct OscDsp {
	fVslider0: F32,
	fEntry0: F32,
	fEntry1: F32,
	fEntry2: F32,
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	fHslider0: F32,
	fVec2: [F32;2],
	fHslider1: F32,
	fRec1: [F32;2],
	fRec2: [F32;2],
	fHslider2: F32,
	fVec3: [F32;2],
	fRec3: [F32;2],
	fRec4: [F32;2],
	fEntry3: F32,
	fEntry4: F32,
	fVslider1: F32,
	iTempPerm0: i32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fTempPerm1: F32,
	fRec6: [F32;2],
	fHslider3: F32,
	fConst2: F32,
	fTempPerm2: F32,
	fRec7: [F32;2],
	iRec9: [i32;2],
	iTempPerm3: i32,
	fConst3: F32,
	fConst5: F32,
	fRec8: [F32;2],
	fConst6: F32,
	fHslider4: F32,
	fRec10: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fTempPerm4: F32,
	fRec13: [F32;2],
	iRec14: [i32;2],
	fVec5: [F32;2],
	fRec12: [F32;2],
	fTempPerm5: F32,
	fVec6: [F32;2],
	fRec11: [F32;2],
	fTempPerm6: F32,
	fHslider5: F32,
	fRec15: [F32;2],
	fEntry5: F32,
	fHslider6: F32,
	fVec7: [F32;2],
	fRec16: [F32;2],
	fRec17: [F32;2],
	fHslider7: F32,
	fVec8: [F32;2],
	fRec18: [F32;2],
	fRec19: [F32;2],
	fHslider8: F32,
	fHslider9: F32,
	fRec20: [F32;2],
	fTempPerm7: F32,
	fTempPerm8: F32,
	fRec21: [F32;2],
	fHslider10: F32,
	fHslider11: F32,
	fCheckbox0: F32,
	fEntry6: F32,
	fTempPerm9: F32,
	fRec0: [F32;2],
	iTempPerm10: i32,
	fHslider12: F32,
	fVec9: [F32;2],
	fTempPerm11: F32,
	fRec22: [F32;2],
	fRec23: [F32;2],
	fTempPerm12: F32,
	fHslider13: F32,
	fVec10: [F32;2],
	fTempPerm13: F32,
	fRec24: [F32;2],
	fRec25: [F32;2],
	fTempPerm14: F32,
	fTempPerm15: F32,
	fTempPerm16: F32,
	fTempPerm17: F32,
	fTempPerm18: F32,
	fTempPerm19: F32,
	fConst9: F32,
	fTempPerm20: F32,
	fEntry7: F32,
	fEntry8: F32,
	fTempPerm21: F32,
	fTempPerm22: F32,
	fTempPerm23: F32,
	fConst10: F32,
	iTempPerm24: i32,
	fConst11: F32,
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
	fTempPerm38: F32,
	fTempPerm39: F32,
	fVec11: [F32;2],
	fVec12: [F32;2],
	fTempPerm40: F32,
	fConst12: F32,
	fTempPerm41: F32,
	fTempPerm42: F32,
	fTempPerm43: F32,
	fTempPerm44: F32,
	fTempPerm45: F32,
	fRec26: [F32;2],
	fTempPerm46: F32,
	fTempPerm47: F32,
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	fTempPerm51: F32,
	fVec13: [F32;2],
	fTempPerm52: F32,
	iTempPerm53: i32,
	iRec27: [i32;2],
	fVec14: [F32;2],
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
	fConst13: F32,
	fTempPerm65: F32,
	fTempPerm66: F32,
	fVec15: [F32;2],
	fTempPerm67: F32,
	fTempPerm68: F32,
	fVec16: [F32;2],
	fTempPerm69: F32,
	fTempPerm70: F32,
	fTempPerm71: F32,
	fTempPerm72: F32,
	fTempPerm73: F32,
	fTempPerm74: F32,
	iTempPerm75: i32,
	fTempPerm76: F32,
	fTempPerm77: F32,
	fTempPerm78: F32,
	fVec17: [F32;2],
	fVec18: [F32;2],
	fTempPerm79: F32,
	fVec19: [F32;2],
	fTempPerm80: F32,
	iTempPerm81: i32,
	fTempPerm82: F32,
	fTempPerm83: F32,
	fTempPerm84: F32,
	fTempPerm85: F32,
	fTempPerm86: F32,
	fTempPerm87: F32,
	fTempPerm88: F32,
	iTempPerm89: i32,
	iTempPerm90: i32,
	iTempPerm91: i32,
	iTempPerm92: i32,
	iTempPerm93: i32,
	iTempPerm94: i32,
	fTempPerm95: F32,
	fTempPerm96: F32,
	fTempPerm97: F32,
	iTempPerm98: i32,
	iTempPerm99: i32,
	iTempPerm100: i32,
	iTempPerm101: i32,
	fTempPerm102: F32,
	fTempPerm103: F32,
	fTempPerm104: F32,
	fVec20: [F32;2],
	fTempPerm105: F32,
	fTempPerm106: F32,
	fTempPerm107: F32,
	fTempPerm108: F32,
	fTempPerm109: F32,
	fVec21: [F32;2],
	fTempPerm110: F32,
	fTempPerm111: F32,
	fTempPerm112: F32,
	fTempPerm113: F32,
	fTempPerm114: F32,
	fRec28: [F32;2],
	fVec22: [F32;2],
	fTempPerm115: F32,
	fTempPerm116: F32,
	fTempPerm117: F32,
	fTempPerm118: F32,
	fRec29: [F32;2],
	fVec23: [F32;2],
	fTempPerm119: F32,
	fVec24: [F32;2],
	fTempPerm120: F32,
	iTempPerm121: i32,
	fTempPerm122: F32,
	fTempPerm123: F32,
	fTempPerm124: F32,
	fTempPerm125: F32,
	fTempPerm126: F32,
	fRec30: [F32;2],
	fVec25: [F32;2],
	fTempPerm127: F32,
	fTempPerm128: F32,
	fTempPerm129: F32,
	fTempPerm130: F32,
	fTempPerm131: F32,
	fRec31: [F32;2],
	fVec26: [F32;2],
	fTempPerm132: F32,
	fTempPerm133: F32,
	fTempPerm134: F32,
	fTempPerm135: F32,
	iTempPerm136: i32,
	fTempPerm137: F32,
	fTempPerm138: F32,
	fTempPerm139: F32,
	fRec32: [F32;2],
	fTempPerm140: F32,
	iTempPerm141: i32,
	fTempPerm142: F32,
	iTempPerm143: i32,
	fTempPerm144: F32,
	fTempPerm145: F32,
	fTempPerm146: F32,
	fVec27: [F32;2],
	fTempPerm147: F32,
	fTempPerm148: F32,
	iTempPerm149: i32,
	fTempPerm150: F32,
	fTempPerm151: F32,
	fTempPerm152: F32,
	fRec33: [F32;2],
	fTempPerm153: F32,
	fTempPerm154: F32,
	fTempPerm155: F32,
	fTempPerm156: F32,
	fTempPerm157: F32,
	fTempPerm158: F32,
	fTempPerm159: F32,
	fTempPerm160: F32,
	fTempPerm161: F32,
	fVec28: [F32;2],
	fTempPerm162: F32,
	fTempPerm163: F32,
	fVec29: [F32;2],
	fTempPerm164: F32,
	fConst14: F32,
	fTempPerm165: F32,
	fConst15: F32,
	fTempPerm166: F32,
	fTempPerm167: F32,
	fRec34: [F32;2],
	fTempPerm168: F32,
	fTempPerm169: F32,
	fTempPerm170: F32,
	fTempPerm171: F32,
	iTempPerm172: i32,
	fRec35: [F32;2],
	fRec36: [F32;2],
	fTempPerm173: F32,
	iTempPerm174: i32,
	fTempPerm175: F32,
	iTempPerm176: i32,
	fTempPerm177: F32,
	iTempPerm178: i32,
	fTempPerm179: F32,
	fTempPerm180: F32,
	fTempPerm181: F32,
	fVec30: [F32;2],
	fTempPerm182: F32,
	iTempPerm183: i32,
	fTempPerm184: F32,
	fTempPerm185: F32,
	fTempPerm186: F32,
	fRec37: [F32;2],
	fTempPerm187: F32,
	fTempPerm188: F32,
	fTempPerm189: F32,
	fTempPerm190: F32,
	fTempPerm191: F32,
	fTempPerm192: F32,
	fTempPerm193: F32,
	fTempPerm194: F32,
	fTempPerm195: F32,
	fTempPerm196: F32,
	fTempPerm197: F32,
	fVec31: [F32;2],
	fTempPerm198: F32,
	fTempPerm199: F32,
	fVec32: [F32;2],
	fTempPerm200: F32,
	iTempPerm201: i32,
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
	fVec33: [F32;2],
	fTempPerm215: F32,
	fTempPerm216: F32,
	fVec34: [F32;2],
	fTempPerm217: F32,
	fVec35: [F32;2],
	fTempPerm218: F32,
	iTempPerm219: i32,
	fTempPerm220: F32,
	fVec36: [F32;2],
	fRec38: [F32;2],
	fTempPerm221: F32,
	fTempPerm222: F32,
	fTempPerm223: F32,
	iTempPerm224: i32,
	fTempPerm225: F32,
	iTempPerm226: i32,
	fTempPerm227: F32,
	fTempPerm228: F32,
	fTempPerm229: F32,
	fTempPerm230: F32,
	fTempPerm231: F32,
	fRec39: [F32;2],
	fTempPerm232: F32,
	fTempPerm233: F32,
	fTempPerm234: F32,
	fTempPerm235: F32,
	fRec40: [F32;2],
	fTempPerm236: F32,
	fTempPerm237: F32,
	fTempPerm238: F32,
	fRec41: [F32;2],
	fVec37: [F32;2],
	fTempPerm239: F32,
	fTempPerm240: F32,
	fHslider14: F32,
	fVec38: [F32;2],
	fTempPerm241: F32,
	fRec42: [F32;2],
	fRec43: [F32;2],
	fTempPerm242: F32,
	iTempPerm243: i32,
	iTempPerm244: i32,
	iTempPerm245: i32,
	fEntry9: F32,
	fEntry10: F32,
	fEntry11: F32,
	iTempPerm246: i32,
	fConst16: F32,
	fConst17: F32,
	fTempPerm247: F32,
	fConst18: F32,
	fConst19: F32,
	fConst20: F32,
	fConst21: F32,
	fConst22: F32,
	iTempPerm248: i32,
	iTempPerm249: i32,
	iTempPerm250: i32,
	iTempPerm251: i32,
	iTempPerm252: i32,
	iTempPerm253: i32,
	iTempPerm254: i32,
	fTempPerm255: F32,
	iTempPerm256: i32,
	iTempPerm257: i32,
	fTempPerm258: F32,
	fTempPerm259: F32,
	fTempPerm260: F32,
	fTempPerm261: F32,
	fTempPerm262: F32,
	fEntry12: F32,
	iTempPerm263: i32,
	fTempPerm264: F32,
	fTempPerm265: F32,
	fHslider15: F32,
	fVec39: [F32;2],
	fTempPerm266: F32,
	fRec44: [F32;2],
	fRec45: [F32;2],
	fTempPerm267: F32,
	fTempPerm268: F32,
	fTempPerm269: F32,
	fTempPerm270: F32,
	iTempPerm271: i32,
	iTempPerm272: i32,
	iTempPerm273: i32,
	iTempPerm274: i32,
	iTempPerm275: i32,
	iTempPerm276: i32,
	fTempPerm277: F32,
	fTempPerm278: F32,
	iTempPerm279: i32,
	iTempPerm280: i32,
	iTempPerm281: i32,
	iTempPerm282: i32,
	fTempPerm283: F32,
	fTempPerm284: F32,
	fTempPerm285: F32,
	iTempPerm286: i32,
	fTempPerm287: F32,
	fTempPerm288: F32,
	fTempPerm289: F32,
	fTempPerm290: F32,
	iTempPerm291: i32,
	iTempPerm292: i32,
	iTempPerm293: i32,
	iTempPerm294: i32,
	iTempPerm295: i32,
	iTempPerm296: i32,
	iTempPerm297: i32,
	fTempPerm298: F32,
	iTempPerm299: i32,
	iTempPerm300: i32,
	fTempPerm301: F32,
	fTempPerm302: F32,
	fTempPerm303: F32,
	fTempPerm304: F32,
	fTempPerm305: F32,
	fTempPerm306: F32,
	fTempPerm307: F32,
	fTempPerm308: F32,
	iTempPerm309: i32,
	iTempPerm310: i32,
	fTempPerm311: F32,
	fTempPerm312: F32,
	fTempPerm313: F32,
	fTempPerm314: F32,
	fConst24: F32,
	fConst25: F32,
	fTempPerm315: F32,
	fTempPerm316: F32,
	fTempPerm317: F32,
	fRec46: [F32;3],
	fTempPerm318: F32,
	fTempPerm319: F32,
	fTempPerm320: F32,
	fTempPerm321: F32,
	fTempPerm322: F32,
	fRec48: [F32;2],
	fRec47: [F32;2],
	fTempPerm323: F32,
	fTempPerm324: F32,
	fTempPerm325: F32,
	fTempPerm326: F32,
	fTempPerm327: F32,
	fTempPerm328: F32,
	fTempPerm329: F32,
	fTempPerm330: F32,
	fHslider16: F32,
	fVec40: [F32;2],
	fRec50: [F32;2],
	fRec51: [F32;2],
	fTempPerm331: F32,
	fTempPerm332: F32,
	fRec49: [F32;2],
	iTempPerm333: i32,
	fHslider17: F32,
	fVec41: [F32;2],
	fTempPerm334: F32,
	fRec52: [F32;2],
	fRec53: [F32;2],
	fTempPerm335: F32,
	fHslider18: F32,
	fVec42: [F32;2],
	fTempPerm336: F32,
	fRec54: [F32;2],
	fRec55: [F32;2],
	fTempPerm337: F32,
	fTempPerm338: F32,
	fTempPerm339: F32,
	fTempPerm340: F32,
	fTempPerm341: F32,
	fTempPerm342: F32,
	fTempPerm343: F32,
	fEntry13: F32,
	fEntry14: F32,
	fTempPerm344: F32,
	fTempPerm345: F32,
	fTempPerm346: F32,
	iTempPerm347: i32,
	fTempPerm348: F32,
	fTempPerm349: F32,
	fTempPerm350: F32,
	fTempPerm351: F32,
	fTempPerm352: F32,
	fTempPerm353: F32,
	fTempPerm354: F32,
	fTempPerm355: F32,
	fTempPerm356: F32,
	fTempPerm357: F32,
	fTempPerm358: F32,
	fTempPerm359: F32,
	fTempPerm360: F32,
	fTempPerm361: F32,
	fTempPerm362: F32,
	fVec43: [F32;2],
	fVec44: [F32;2],
	fTempPerm363: F32,
	fTempPerm364: F32,
	fTempPerm365: F32,
	fRec56: [F32;2],
	fTempPerm366: F32,
	fTempPerm367: F32,
	fTempPerm368: F32,
	fTempPerm369: F32,
	fTempPerm370: F32,
	fTempPerm371: F32,
	fVec45: [F32;2],
	fTempPerm372: F32,
	iTempPerm373: i32,
	iRec57: [i32;2],
	fVec46: [F32;2],
	fTempPerm374: F32,
	fTempPerm375: F32,
	fTempPerm376: F32,
	fTempPerm377: F32,
	fTempPerm378: F32,
	fTempPerm379: F32,
	fTempPerm380: F32,
	fTempPerm381: F32,
	fTempPerm382: F32,
	fTempPerm383: F32,
	fTempPerm384: F32,
	fTempPerm385: F32,
	fTempPerm386: F32,
	fVec47: [F32;2],
	fTempPerm387: F32,
	fTempPerm388: F32,
	fVec48: [F32;2],
	fTempPerm389: F32,
	fTempPerm390: F32,
	fTempPerm391: F32,
	fTempPerm392: F32,
	fTempPerm393: F32,
	fTempPerm394: F32,
	iTempPerm395: i32,
	fTempPerm396: F32,
	fTempPerm397: F32,
	fTempPerm398: F32,
	fVec49: [F32;2],
	fVec50: [F32;2],
	fTempPerm399: F32,
	fVec51: [F32;2],
	fTempPerm400: F32,
	iTempPerm401: i32,
	fTempPerm402: F32,
	fTempPerm403: F32,
	fTempPerm404: F32,
	fTempPerm405: F32,
	fTempPerm406: F32,
	fTempPerm407: F32,
	fTempPerm408: F32,
	iTempPerm409: i32,
	iTempPerm410: i32,
	iTempPerm411: i32,
	iTempPerm412: i32,
	iTempPerm413: i32,
	iTempPerm414: i32,
	fTempPerm415: F32,
	fTempPerm416: F32,
	fTempPerm417: F32,
	iTempPerm418: i32,
	iTempPerm419: i32,
	iTempPerm420: i32,
	iTempPerm421: i32,
	fTempPerm422: F32,
	fTempPerm423: F32,
	fTempPerm424: F32,
	fVec52: [F32;2],
	fTempPerm425: F32,
	fTempPerm426: F32,
	fTempPerm427: F32,
	fTempPerm428: F32,
	fTempPerm429: F32,
	fVec53: [F32;2],
	fTempPerm430: F32,
	fTempPerm431: F32,
	fTempPerm432: F32,
	fRec58: [F32;2],
	fVec54: [F32;2],
	fTempPerm433: F32,
	fTempPerm434: F32,
	fRec59: [F32;2],
	fVec55: [F32;2],
	fTempPerm435: F32,
	fVec56: [F32;2],
	fTempPerm436: F32,
	iTempPerm437: i32,
	fTempPerm438: F32,
	fTempPerm439: F32,
	fTempPerm440: F32,
	fTempPerm441: F32,
	fTempPerm442: F32,
	fRec60: [F32;2],
	fVec57: [F32;2],
	fTempPerm443: F32,
	fTempPerm444: F32,
	fTempPerm445: F32,
	fTempPerm446: F32,
	fTempPerm447: F32,
	fRec61: [F32;2],
	fVec58: [F32;2],
	fTempPerm448: F32,
	fTempPerm449: F32,
	fTempPerm450: F32,
	fTempPerm451: F32,
	iTempPerm452: i32,
	fTempPerm453: F32,
	fTempPerm454: F32,
	fTempPerm455: F32,
	fRec62: [F32;2],
	fTempPerm456: F32,
	iTempPerm457: i32,
	fTempPerm458: F32,
	iTempPerm459: i32,
	fTempPerm460: F32,
	fTempPerm461: F32,
	fTempPerm462: F32,
	fVec59: [F32;2],
	fTempPerm463: F32,
	fTempPerm464: F32,
	iTempPerm465: i32,
	fTempPerm466: F32,
	fTempPerm467: F32,
	fTempPerm468: F32,
	fRec63: [F32;2],
	fTempPerm469: F32,
	fTempPerm470: F32,
	fTempPerm471: F32,
	fTempPerm472: F32,
	fTempPerm473: F32,
	fTempPerm474: F32,
	fTempPerm475: F32,
	fTempPerm476: F32,
	fTempPerm477: F32,
	fVec60: [F32;2],
	fTempPerm478: F32,
	fTempPerm479: F32,
	fVec61: [F32;2],
	fTempPerm480: F32,
	fTempPerm481: F32,
	fTempPerm482: F32,
	fRec64: [F32;2],
	fTempPerm483: F32,
	fTempPerm484: F32,
	fTempPerm485: F32,
	fTempPerm486: F32,
	iTempPerm487: i32,
	fRec65: [F32;2],
	fRec66: [F32;2],
	fTempPerm488: F32,
	iTempPerm489: i32,
	fTempPerm490: F32,
	iTempPerm491: i32,
	fTempPerm492: F32,
	iTempPerm493: i32,
	fTempPerm494: F32,
	fTempPerm495: F32,
	fTempPerm496: F32,
	fVec62: [F32;2],
	fTempPerm497: F32,
	iTempPerm498: i32,
	fTempPerm499: F32,
	fTempPerm500: F32,
	fTempPerm501: F32,
	fRec67: [F32;2],
	fTempPerm502: F32,
	fTempPerm503: F32,
	fTempPerm504: F32,
	fTempPerm505: F32,
	fTempPerm506: F32,
	fTempPerm507: F32,
	fTempPerm508: F32,
	fTempPerm509: F32,
	fTempPerm510: F32,
	fTempPerm511: F32,
	fTempPerm512: F32,
	fVec63: [F32;2],
	fTempPerm513: F32,
	fTempPerm514: F32,
	fVec64: [F32;2],
	fTempPerm515: F32,
	iTempPerm516: i32,
	fTempPerm517: F32,
	fTempPerm518: F32,
	fTempPerm519: F32,
	fTempPerm520: F32,
	fTempPerm521: F32,
	fTempPerm522: F32,
	fTempPerm523: F32,
	fTempPerm524: F32,
	fTempPerm525: F32,
	fTempPerm526: F32,
	fTempPerm527: F32,
	fTempPerm528: F32,
	fTempPerm529: F32,
	fVec65: [F32;2],
	fTempPerm530: F32,
	fTempPerm531: F32,
	fVec66: [F32;2],
	fTempPerm532: F32,
	fVec67: [F32;2],
	fTempPerm533: F32,
	iTempPerm534: i32,
	fTempPerm535: F32,
	fVec68: [F32;2],
	fRec68: [F32;2],
	fTempPerm536: F32,
	fTempPerm537: F32,
	fTempPerm538: F32,
	iTempPerm539: i32,
	fTempPerm540: F32,
	iTempPerm541: i32,
	fTempPerm542: F32,
	fTempPerm543: F32,
	fTempPerm544: F32,
	fRec69: [F32;2],
	fTempPerm545: F32,
	fTempPerm546: F32,
	fRec70: [F32;2],
	fTempPerm547: F32,
	fRec71: [F32;2],
	fVec69: [F32;2],
	fTempPerm548: F32,
	fTempPerm549: F32,
	fHslider19: F32,
	fVec70: [F32;2],
	fTempPerm550: F32,
	fRec72: [F32;2],
	fRec73: [F32;2],
	fTempPerm551: F32,
	iTempPerm552: i32,
	iTempPerm553: i32,
	iTempPerm554: i32,
	fEntry15: F32,
	fEntry16: F32,
	fEntry17: F32,
	iTempPerm555: i32,
	fTempPerm556: F32,
	iTempPerm557: i32,
	iTempPerm558: i32,
	iTempPerm559: i32,
	iTempPerm560: i32,
	iTempPerm561: i32,
	iTempPerm562: i32,
	iTempPerm563: i32,
	fTempPerm564: F32,
	iTempPerm565: i32,
	iTempPerm566: i32,
	fTempPerm567: F32,
	fTempPerm568: F32,
	fTempPerm569: F32,
	fTempPerm570: F32,
	fTempPerm571: F32,
	fEntry18: F32,
	iTempPerm572: i32,
	fTempPerm573: F32,
	fTempPerm574: F32,
	fHslider20: F32,
	fVec71: [F32;2],
	fTempPerm575: F32,
	fRec74: [F32;2],
	fRec75: [F32;2],
	fTempPerm576: F32,
	fTempPerm577: F32,
	fTempPerm578: F32,
	fTempPerm579: F32,
	iTempPerm580: i32,
	iTempPerm581: i32,
	iTempPerm582: i32,
	iTempPerm583: i32,
	iTempPerm584: i32,
	iTempPerm585: i32,
	fTempPerm586: F32,
	fTempPerm587: F32,
	iTempPerm588: i32,
	iTempPerm589: i32,
	iTempPerm590: i32,
	iTempPerm591: i32,
	fTempPerm592: F32,
	fTempPerm593: F32,
	fTempPerm594: F32,
	iTempPerm595: i32,
	fTempPerm596: F32,
	fTempPerm597: F32,
	fTempPerm598: F32,
	fTempPerm599: F32,
	iTempPerm600: i32,
	iTempPerm601: i32,
	iTempPerm602: i32,
	iTempPerm603: i32,
	iTempPerm604: i32,
	iTempPerm605: i32,
	iTempPerm606: i32,
	fTempPerm607: F32,
	iTempPerm608: i32,
	iTempPerm609: i32,
	fTempPerm610: F32,
	fTempPerm611: F32,
	fTempPerm612: F32,
	fTempPerm613: F32,
	fTempPerm614: F32,
	fTempPerm615: F32,
	fTempPerm616: F32,
	fTempPerm617: F32,
	iTempPerm618: i32,
	iTempPerm619: i32,
	fTempPerm620: F32,
	fTempPerm621: F32,
	fTempPerm622: F32,
	fTempPerm623: F32,
	fTempPerm624: F32,
	fTempPerm625: F32,
	fTempPerm626: F32,
	fRec76: [F32;3],
	fTempPerm627: F32,
	fTempPerm628: F32,
	fTempPerm629: F32,
	fTempPerm630: F32,
	fTempPerm631: F32,
	fRec78: [F32;2],
	fRec77: [F32;2],
	fTempPerm632: F32,
	fTempPerm633: F32,
	fTempPerm634: F32,
	fTempPerm635: F32,
	fTempPerm636: F32,
	fTempPerm637: F32,
	fTempPerm638: F32,
	fTempPerm639: F32,
	fHslider21: F32,
	fVec72: [F32;2],
	fRec79: [F32;2],
	fRec80: [F32;2],
	fHslider22: F32,
	fVec73: [F32;2],
	fRec81: [F32;2],
	fRec82: [F32;2],
	fEntry19: F32,
	fCheckbox1: F32,
	fHslider23: F32,
	fHslider24: F32,
	fHslider25: F32,
	fHslider26: F32,
	fEntry20: F32,
	fHslider27: F32,
	fHslider28: F32,
	fHslider29: F32,
	fHslider30: F32,
	fHslider31: F32,
	fEntry21: F32,
	fCheckbox2: F32,
	fHslider32: F32,
	fHslider33: F32,
	fHslider34: F32,
	fHslider35: F32,
	fEntry22: F32,
	fCheckbox3: F32,
	fHslider36: F32,
	fHslider37: F32,
	fHslider38: F32,
	fHslider39: F32,
	fEntry23: F32,
	fCheckbox4: F32,
	fEntry24: F32,
	fHslider40: F32,
	fHslider41: F32,
	fHslider42: F32,
	fHslider43: F32,
	fEntry25: F32,
	fCheckbox5: F32,
	fHslider44: F32,
	fHslider45: F32,
	fHslider46: F32,
	fHslider47: F32,
	fEntry26: F32,
	fCheckbox6: F32,
	fHslider48: F32,
	fHslider49: F32,
	fHslider50: F32,
	fHslider51: F32,
	fEntry27: F32,
	fCheckbox7: F32,
	fEntry28: F32,
	fHslider52: F32,
	fHslider53: F32,
	fHslider54: F32,
	fHslider55: F32,
	fHslider56: F32,
	fHslider57: F32,
	fHslider58: F32,
	fEntry29: F32,
	fEntry30: F32,
	fHslider59: F32,
	fHslider60: F32,
	fHslider61: F32,
	fHslider62: F32,
	fHslider63: F32,
	fHslider64: F32,
	fHslider65: F32,
	fEntry31: F32,
	fEntry32: F32,
	fHslider66: F32,
	fHslider67: F32,
	fHslider68: F32,
	fHslider69: F32,
	fHslider70: F32,
	fHslider71: F32,
	fHslider72: F32,
	fHslider73: F32,
	fCheckbox8: F32,
	fEntry33: F32,
	fEntry34: F32,
	fHslider74: F32,
	fHslider75: F32,
	fHslider76: F32,
	fHslider77: F32,
	fHslider78: F32,
	fHslider79: F32,
	fHslider80: F32,
	fHslider81: F32,
	fCheckbox9: F32,
	fEntry35: F32,
	fEntry36: F32,
}

impl FaustDsp for OscDsp {
	type T = F32;
		
	fn new() -> OscDsp { 
		OscDsp {
			fVslider0: 0.0,
			fEntry0: 0.0,
			fEntry1: 0.0,
			fEntry2: 0.0,
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			fHslider0: 0.0,
			fVec2: [0.0;2],
			fHslider1: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fHslider2: 0.0,
			fVec3: [0.0;2],
			fRec3: [0.0;2],
			fRec4: [0.0;2],
			fEntry3: 0.0,
			fEntry4: 0.0,
			fVslider1: 0.0,
			iTempPerm0: 0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fTempPerm1: 0.0,
			fRec6: [0.0;2],
			fHslider3: 0.0,
			fConst2: 0.0,
			fTempPerm2: 0.0,
			fRec7: [0.0;2],
			iRec9: [0;2],
			iTempPerm3: 0,
			fConst3: 0.0,
			fConst5: 0.0,
			fRec8: [0.0;2],
			fConst6: 0.0,
			fHslider4: 0.0,
			fRec10: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fTempPerm4: 0.0,
			fRec13: [0.0;2],
			iRec14: [0;2],
			fVec5: [0.0;2],
			fRec12: [0.0;2],
			fTempPerm5: 0.0,
			fVec6: [0.0;2],
			fRec11: [0.0;2],
			fTempPerm6: 0.0,
			fHslider5: 0.0,
			fRec15: [0.0;2],
			fEntry5: 0.0,
			fHslider6: 0.0,
			fVec7: [0.0;2],
			fRec16: [0.0;2],
			fRec17: [0.0;2],
			fHslider7: 0.0,
			fVec8: [0.0;2],
			fRec18: [0.0;2],
			fRec19: [0.0;2],
			fHslider8: 0.0,
			fHslider9: 0.0,
			fRec20: [0.0;2],
			fTempPerm7: 0.0,
			fTempPerm8: 0.0,
			fRec21: [0.0;2],
			fHslider10: 0.0,
			fHslider11: 0.0,
			fCheckbox0: 0.0,
			fEntry6: 0.0,
			fTempPerm9: 0.0,
			fRec0: [0.0;2],
			iTempPerm10: 0,
			fHslider12: 0.0,
			fVec9: [0.0;2],
			fTempPerm11: 0.0,
			fRec22: [0.0;2],
			fRec23: [0.0;2],
			fTempPerm12: 0.0,
			fHslider13: 0.0,
			fVec10: [0.0;2],
			fTempPerm13: 0.0,
			fRec24: [0.0;2],
			fRec25: [0.0;2],
			fTempPerm14: 0.0,
			fTempPerm15: 0.0,
			fTempPerm16: 0.0,
			fTempPerm17: 0.0,
			fTempPerm18: 0.0,
			fTempPerm19: 0.0,
			fConst9: 0.0,
			fTempPerm20: 0.0,
			fEntry7: 0.0,
			fEntry8: 0.0,
			fTempPerm21: 0.0,
			fTempPerm22: 0.0,
			fTempPerm23: 0.0,
			fConst10: 0.0,
			iTempPerm24: 0,
			fConst11: 0.0,
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
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fVec11: [0.0;2],
			fVec12: [0.0;2],
			fTempPerm40: 0.0,
			fConst12: 0.0,
			fTempPerm41: 0.0,
			fTempPerm42: 0.0,
			fTempPerm43: 0.0,
			fTempPerm44: 0.0,
			fTempPerm45: 0.0,
			fRec26: [0.0;2],
			fTempPerm46: 0.0,
			fTempPerm47: 0.0,
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			fVec13: [0.0;2],
			fTempPerm52: 0.0,
			iTempPerm53: 0,
			iRec27: [0;2],
			fVec14: [0.0;2],
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
			fConst13: 0.0,
			fTempPerm65: 0.0,
			fTempPerm66: 0.0,
			fVec15: [0.0;2],
			fTempPerm67: 0.0,
			fTempPerm68: 0.0,
			fVec16: [0.0;2],
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			fTempPerm71: 0.0,
			fTempPerm72: 0.0,
			fTempPerm73: 0.0,
			fTempPerm74: 0.0,
			iTempPerm75: 0,
			fTempPerm76: 0.0,
			fTempPerm77: 0.0,
			fTempPerm78: 0.0,
			fVec17: [0.0;2],
			fVec18: [0.0;2],
			fTempPerm79: 0.0,
			fVec19: [0.0;2],
			fTempPerm80: 0.0,
			iTempPerm81: 0,
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			fTempPerm84: 0.0,
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			fTempPerm87: 0.0,
			fTempPerm88: 0.0,
			iTempPerm89: 0,
			iTempPerm90: 0,
			iTempPerm91: 0,
			iTempPerm92: 0,
			iTempPerm93: 0,
			iTempPerm94: 0,
			fTempPerm95: 0.0,
			fTempPerm96: 0.0,
			fTempPerm97: 0.0,
			iTempPerm98: 0,
			iTempPerm99: 0,
			iTempPerm100: 0,
			iTempPerm101: 0,
			fTempPerm102: 0.0,
			fTempPerm103: 0.0,
			fTempPerm104: 0.0,
			fVec20: [0.0;2],
			fTempPerm105: 0.0,
			fTempPerm106: 0.0,
			fTempPerm107: 0.0,
			fTempPerm108: 0.0,
			fTempPerm109: 0.0,
			fVec21: [0.0;2],
			fTempPerm110: 0.0,
			fTempPerm111: 0.0,
			fTempPerm112: 0.0,
			fTempPerm113: 0.0,
			fTempPerm114: 0.0,
			fRec28: [0.0;2],
			fVec22: [0.0;2],
			fTempPerm115: 0.0,
			fTempPerm116: 0.0,
			fTempPerm117: 0.0,
			fTempPerm118: 0.0,
			fRec29: [0.0;2],
			fVec23: [0.0;2],
			fTempPerm119: 0.0,
			fVec24: [0.0;2],
			fTempPerm120: 0.0,
			iTempPerm121: 0,
			fTempPerm122: 0.0,
			fTempPerm123: 0.0,
			fTempPerm124: 0.0,
			fTempPerm125: 0.0,
			fTempPerm126: 0.0,
			fRec30: [0.0;2],
			fVec25: [0.0;2],
			fTempPerm127: 0.0,
			fTempPerm128: 0.0,
			fTempPerm129: 0.0,
			fTempPerm130: 0.0,
			fTempPerm131: 0.0,
			fRec31: [0.0;2],
			fVec26: [0.0;2],
			fTempPerm132: 0.0,
			fTempPerm133: 0.0,
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			iTempPerm136: 0,
			fTempPerm137: 0.0,
			fTempPerm138: 0.0,
			fTempPerm139: 0.0,
			fRec32: [0.0;2],
			fTempPerm140: 0.0,
			iTempPerm141: 0,
			fTempPerm142: 0.0,
			iTempPerm143: 0,
			fTempPerm144: 0.0,
			fTempPerm145: 0.0,
			fTempPerm146: 0.0,
			fVec27: [0.0;2],
			fTempPerm147: 0.0,
			fTempPerm148: 0.0,
			iTempPerm149: 0,
			fTempPerm150: 0.0,
			fTempPerm151: 0.0,
			fTempPerm152: 0.0,
			fRec33: [0.0;2],
			fTempPerm153: 0.0,
			fTempPerm154: 0.0,
			fTempPerm155: 0.0,
			fTempPerm156: 0.0,
			fTempPerm157: 0.0,
			fTempPerm158: 0.0,
			fTempPerm159: 0.0,
			fTempPerm160: 0.0,
			fTempPerm161: 0.0,
			fVec28: [0.0;2],
			fTempPerm162: 0.0,
			fTempPerm163: 0.0,
			fVec29: [0.0;2],
			fTempPerm164: 0.0,
			fConst14: 0.0,
			fTempPerm165: 0.0,
			fConst15: 0.0,
			fTempPerm166: 0.0,
			fTempPerm167: 0.0,
			fRec34: [0.0;2],
			fTempPerm168: 0.0,
			fTempPerm169: 0.0,
			fTempPerm170: 0.0,
			fTempPerm171: 0.0,
			iTempPerm172: 0,
			fRec35: [0.0;2],
			fRec36: [0.0;2],
			fTempPerm173: 0.0,
			iTempPerm174: 0,
			fTempPerm175: 0.0,
			iTempPerm176: 0,
			fTempPerm177: 0.0,
			iTempPerm178: 0,
			fTempPerm179: 0.0,
			fTempPerm180: 0.0,
			fTempPerm181: 0.0,
			fVec30: [0.0;2],
			fTempPerm182: 0.0,
			iTempPerm183: 0,
			fTempPerm184: 0.0,
			fTempPerm185: 0.0,
			fTempPerm186: 0.0,
			fRec37: [0.0;2],
			fTempPerm187: 0.0,
			fTempPerm188: 0.0,
			fTempPerm189: 0.0,
			fTempPerm190: 0.0,
			fTempPerm191: 0.0,
			fTempPerm192: 0.0,
			fTempPerm193: 0.0,
			fTempPerm194: 0.0,
			fTempPerm195: 0.0,
			fTempPerm196: 0.0,
			fTempPerm197: 0.0,
			fVec31: [0.0;2],
			fTempPerm198: 0.0,
			fTempPerm199: 0.0,
			fVec32: [0.0;2],
			fTempPerm200: 0.0,
			iTempPerm201: 0,
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
			fVec33: [0.0;2],
			fTempPerm215: 0.0,
			fTempPerm216: 0.0,
			fVec34: [0.0;2],
			fTempPerm217: 0.0,
			fVec35: [0.0;2],
			fTempPerm218: 0.0,
			iTempPerm219: 0,
			fTempPerm220: 0.0,
			fVec36: [0.0;2],
			fRec38: [0.0;2],
			fTempPerm221: 0.0,
			fTempPerm222: 0.0,
			fTempPerm223: 0.0,
			iTempPerm224: 0,
			fTempPerm225: 0.0,
			iTempPerm226: 0,
			fTempPerm227: 0.0,
			fTempPerm228: 0.0,
			fTempPerm229: 0.0,
			fTempPerm230: 0.0,
			fTempPerm231: 0.0,
			fRec39: [0.0;2],
			fTempPerm232: 0.0,
			fTempPerm233: 0.0,
			fTempPerm234: 0.0,
			fTempPerm235: 0.0,
			fRec40: [0.0;2],
			fTempPerm236: 0.0,
			fTempPerm237: 0.0,
			fTempPerm238: 0.0,
			fRec41: [0.0;2],
			fVec37: [0.0;2],
			fTempPerm239: 0.0,
			fTempPerm240: 0.0,
			fHslider14: 0.0,
			fVec38: [0.0;2],
			fTempPerm241: 0.0,
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fTempPerm242: 0.0,
			iTempPerm243: 0,
			iTempPerm244: 0,
			iTempPerm245: 0,
			fEntry9: 0.0,
			fEntry10: 0.0,
			fEntry11: 0.0,
			iTempPerm246: 0,
			fConst16: 0.0,
			fConst17: 0.0,
			fTempPerm247: 0.0,
			fConst18: 0.0,
			fConst19: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			iTempPerm248: 0,
			iTempPerm249: 0,
			iTempPerm250: 0,
			iTempPerm251: 0,
			iTempPerm252: 0,
			iTempPerm253: 0,
			iTempPerm254: 0,
			fTempPerm255: 0.0,
			iTempPerm256: 0,
			iTempPerm257: 0,
			fTempPerm258: 0.0,
			fTempPerm259: 0.0,
			fTempPerm260: 0.0,
			fTempPerm261: 0.0,
			fTempPerm262: 0.0,
			fEntry12: 0.0,
			iTempPerm263: 0,
			fTempPerm264: 0.0,
			fTempPerm265: 0.0,
			fHslider15: 0.0,
			fVec39: [0.0;2],
			fTempPerm266: 0.0,
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			fTempPerm267: 0.0,
			fTempPerm268: 0.0,
			fTempPerm269: 0.0,
			fTempPerm270: 0.0,
			iTempPerm271: 0,
			iTempPerm272: 0,
			iTempPerm273: 0,
			iTempPerm274: 0,
			iTempPerm275: 0,
			iTempPerm276: 0,
			fTempPerm277: 0.0,
			fTempPerm278: 0.0,
			iTempPerm279: 0,
			iTempPerm280: 0,
			iTempPerm281: 0,
			iTempPerm282: 0,
			fTempPerm283: 0.0,
			fTempPerm284: 0.0,
			fTempPerm285: 0.0,
			iTempPerm286: 0,
			fTempPerm287: 0.0,
			fTempPerm288: 0.0,
			fTempPerm289: 0.0,
			fTempPerm290: 0.0,
			iTempPerm291: 0,
			iTempPerm292: 0,
			iTempPerm293: 0,
			iTempPerm294: 0,
			iTempPerm295: 0,
			iTempPerm296: 0,
			iTempPerm297: 0,
			fTempPerm298: 0.0,
			iTempPerm299: 0,
			iTempPerm300: 0,
			fTempPerm301: 0.0,
			fTempPerm302: 0.0,
			fTempPerm303: 0.0,
			fTempPerm304: 0.0,
			fTempPerm305: 0.0,
			fTempPerm306: 0.0,
			fTempPerm307: 0.0,
			fTempPerm308: 0.0,
			iTempPerm309: 0,
			iTempPerm310: 0,
			fTempPerm311: 0.0,
			fTempPerm312: 0.0,
			fTempPerm313: 0.0,
			fTempPerm314: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fTempPerm315: 0.0,
			fTempPerm316: 0.0,
			fTempPerm317: 0.0,
			fRec46: [0.0;3],
			fTempPerm318: 0.0,
			fTempPerm319: 0.0,
			fTempPerm320: 0.0,
			fTempPerm321: 0.0,
			fTempPerm322: 0.0,
			fRec48: [0.0;2],
			fRec47: [0.0;2],
			fTempPerm323: 0.0,
			fTempPerm324: 0.0,
			fTempPerm325: 0.0,
			fTempPerm326: 0.0,
			fTempPerm327: 0.0,
			fTempPerm328: 0.0,
			fTempPerm329: 0.0,
			fTempPerm330: 0.0,
			fHslider16: 0.0,
			fVec40: [0.0;2],
			fRec50: [0.0;2],
			fRec51: [0.0;2],
			fTempPerm331: 0.0,
			fTempPerm332: 0.0,
			fRec49: [0.0;2],
			iTempPerm333: 0,
			fHslider17: 0.0,
			fVec41: [0.0;2],
			fTempPerm334: 0.0,
			fRec52: [0.0;2],
			fRec53: [0.0;2],
			fTempPerm335: 0.0,
			fHslider18: 0.0,
			fVec42: [0.0;2],
			fTempPerm336: 0.0,
			fRec54: [0.0;2],
			fRec55: [0.0;2],
			fTempPerm337: 0.0,
			fTempPerm338: 0.0,
			fTempPerm339: 0.0,
			fTempPerm340: 0.0,
			fTempPerm341: 0.0,
			fTempPerm342: 0.0,
			fTempPerm343: 0.0,
			fEntry13: 0.0,
			fEntry14: 0.0,
			fTempPerm344: 0.0,
			fTempPerm345: 0.0,
			fTempPerm346: 0.0,
			iTempPerm347: 0,
			fTempPerm348: 0.0,
			fTempPerm349: 0.0,
			fTempPerm350: 0.0,
			fTempPerm351: 0.0,
			fTempPerm352: 0.0,
			fTempPerm353: 0.0,
			fTempPerm354: 0.0,
			fTempPerm355: 0.0,
			fTempPerm356: 0.0,
			fTempPerm357: 0.0,
			fTempPerm358: 0.0,
			fTempPerm359: 0.0,
			fTempPerm360: 0.0,
			fTempPerm361: 0.0,
			fTempPerm362: 0.0,
			fVec43: [0.0;2],
			fVec44: [0.0;2],
			fTempPerm363: 0.0,
			fTempPerm364: 0.0,
			fTempPerm365: 0.0,
			fRec56: [0.0;2],
			fTempPerm366: 0.0,
			fTempPerm367: 0.0,
			fTempPerm368: 0.0,
			fTempPerm369: 0.0,
			fTempPerm370: 0.0,
			fTempPerm371: 0.0,
			fVec45: [0.0;2],
			fTempPerm372: 0.0,
			iTempPerm373: 0,
			iRec57: [0;2],
			fVec46: [0.0;2],
			fTempPerm374: 0.0,
			fTempPerm375: 0.0,
			fTempPerm376: 0.0,
			fTempPerm377: 0.0,
			fTempPerm378: 0.0,
			fTempPerm379: 0.0,
			fTempPerm380: 0.0,
			fTempPerm381: 0.0,
			fTempPerm382: 0.0,
			fTempPerm383: 0.0,
			fTempPerm384: 0.0,
			fTempPerm385: 0.0,
			fTempPerm386: 0.0,
			fVec47: [0.0;2],
			fTempPerm387: 0.0,
			fTempPerm388: 0.0,
			fVec48: [0.0;2],
			fTempPerm389: 0.0,
			fTempPerm390: 0.0,
			fTempPerm391: 0.0,
			fTempPerm392: 0.0,
			fTempPerm393: 0.0,
			fTempPerm394: 0.0,
			iTempPerm395: 0,
			fTempPerm396: 0.0,
			fTempPerm397: 0.0,
			fTempPerm398: 0.0,
			fVec49: [0.0;2],
			fVec50: [0.0;2],
			fTempPerm399: 0.0,
			fVec51: [0.0;2],
			fTempPerm400: 0.0,
			iTempPerm401: 0,
			fTempPerm402: 0.0,
			fTempPerm403: 0.0,
			fTempPerm404: 0.0,
			fTempPerm405: 0.0,
			fTempPerm406: 0.0,
			fTempPerm407: 0.0,
			fTempPerm408: 0.0,
			iTempPerm409: 0,
			iTempPerm410: 0,
			iTempPerm411: 0,
			iTempPerm412: 0,
			iTempPerm413: 0,
			iTempPerm414: 0,
			fTempPerm415: 0.0,
			fTempPerm416: 0.0,
			fTempPerm417: 0.0,
			iTempPerm418: 0,
			iTempPerm419: 0,
			iTempPerm420: 0,
			iTempPerm421: 0,
			fTempPerm422: 0.0,
			fTempPerm423: 0.0,
			fTempPerm424: 0.0,
			fVec52: [0.0;2],
			fTempPerm425: 0.0,
			fTempPerm426: 0.0,
			fTempPerm427: 0.0,
			fTempPerm428: 0.0,
			fTempPerm429: 0.0,
			fVec53: [0.0;2],
			fTempPerm430: 0.0,
			fTempPerm431: 0.0,
			fTempPerm432: 0.0,
			fRec58: [0.0;2],
			fVec54: [0.0;2],
			fTempPerm433: 0.0,
			fTempPerm434: 0.0,
			fRec59: [0.0;2],
			fVec55: [0.0;2],
			fTempPerm435: 0.0,
			fVec56: [0.0;2],
			fTempPerm436: 0.0,
			iTempPerm437: 0,
			fTempPerm438: 0.0,
			fTempPerm439: 0.0,
			fTempPerm440: 0.0,
			fTempPerm441: 0.0,
			fTempPerm442: 0.0,
			fRec60: [0.0;2],
			fVec57: [0.0;2],
			fTempPerm443: 0.0,
			fTempPerm444: 0.0,
			fTempPerm445: 0.0,
			fTempPerm446: 0.0,
			fTempPerm447: 0.0,
			fRec61: [0.0;2],
			fVec58: [0.0;2],
			fTempPerm448: 0.0,
			fTempPerm449: 0.0,
			fTempPerm450: 0.0,
			fTempPerm451: 0.0,
			iTempPerm452: 0,
			fTempPerm453: 0.0,
			fTempPerm454: 0.0,
			fTempPerm455: 0.0,
			fRec62: [0.0;2],
			fTempPerm456: 0.0,
			iTempPerm457: 0,
			fTempPerm458: 0.0,
			iTempPerm459: 0,
			fTempPerm460: 0.0,
			fTempPerm461: 0.0,
			fTempPerm462: 0.0,
			fVec59: [0.0;2],
			fTempPerm463: 0.0,
			fTempPerm464: 0.0,
			iTempPerm465: 0,
			fTempPerm466: 0.0,
			fTempPerm467: 0.0,
			fTempPerm468: 0.0,
			fRec63: [0.0;2],
			fTempPerm469: 0.0,
			fTempPerm470: 0.0,
			fTempPerm471: 0.0,
			fTempPerm472: 0.0,
			fTempPerm473: 0.0,
			fTempPerm474: 0.0,
			fTempPerm475: 0.0,
			fTempPerm476: 0.0,
			fTempPerm477: 0.0,
			fVec60: [0.0;2],
			fTempPerm478: 0.0,
			fTempPerm479: 0.0,
			fVec61: [0.0;2],
			fTempPerm480: 0.0,
			fTempPerm481: 0.0,
			fTempPerm482: 0.0,
			fRec64: [0.0;2],
			fTempPerm483: 0.0,
			fTempPerm484: 0.0,
			fTempPerm485: 0.0,
			fTempPerm486: 0.0,
			iTempPerm487: 0,
			fRec65: [0.0;2],
			fRec66: [0.0;2],
			fTempPerm488: 0.0,
			iTempPerm489: 0,
			fTempPerm490: 0.0,
			iTempPerm491: 0,
			fTempPerm492: 0.0,
			iTempPerm493: 0,
			fTempPerm494: 0.0,
			fTempPerm495: 0.0,
			fTempPerm496: 0.0,
			fVec62: [0.0;2],
			fTempPerm497: 0.0,
			iTempPerm498: 0,
			fTempPerm499: 0.0,
			fTempPerm500: 0.0,
			fTempPerm501: 0.0,
			fRec67: [0.0;2],
			fTempPerm502: 0.0,
			fTempPerm503: 0.0,
			fTempPerm504: 0.0,
			fTempPerm505: 0.0,
			fTempPerm506: 0.0,
			fTempPerm507: 0.0,
			fTempPerm508: 0.0,
			fTempPerm509: 0.0,
			fTempPerm510: 0.0,
			fTempPerm511: 0.0,
			fTempPerm512: 0.0,
			fVec63: [0.0;2],
			fTempPerm513: 0.0,
			fTempPerm514: 0.0,
			fVec64: [0.0;2],
			fTempPerm515: 0.0,
			iTempPerm516: 0,
			fTempPerm517: 0.0,
			fTempPerm518: 0.0,
			fTempPerm519: 0.0,
			fTempPerm520: 0.0,
			fTempPerm521: 0.0,
			fTempPerm522: 0.0,
			fTempPerm523: 0.0,
			fTempPerm524: 0.0,
			fTempPerm525: 0.0,
			fTempPerm526: 0.0,
			fTempPerm527: 0.0,
			fTempPerm528: 0.0,
			fTempPerm529: 0.0,
			fVec65: [0.0;2],
			fTempPerm530: 0.0,
			fTempPerm531: 0.0,
			fVec66: [0.0;2],
			fTempPerm532: 0.0,
			fVec67: [0.0;2],
			fTempPerm533: 0.0,
			iTempPerm534: 0,
			fTempPerm535: 0.0,
			fVec68: [0.0;2],
			fRec68: [0.0;2],
			fTempPerm536: 0.0,
			fTempPerm537: 0.0,
			fTempPerm538: 0.0,
			iTempPerm539: 0,
			fTempPerm540: 0.0,
			iTempPerm541: 0,
			fTempPerm542: 0.0,
			fTempPerm543: 0.0,
			fTempPerm544: 0.0,
			fRec69: [0.0;2],
			fTempPerm545: 0.0,
			fTempPerm546: 0.0,
			fRec70: [0.0;2],
			fTempPerm547: 0.0,
			fRec71: [0.0;2],
			fVec69: [0.0;2],
			fTempPerm548: 0.0,
			fTempPerm549: 0.0,
			fHslider19: 0.0,
			fVec70: [0.0;2],
			fTempPerm550: 0.0,
			fRec72: [0.0;2],
			fRec73: [0.0;2],
			fTempPerm551: 0.0,
			iTempPerm552: 0,
			iTempPerm553: 0,
			iTempPerm554: 0,
			fEntry15: 0.0,
			fEntry16: 0.0,
			fEntry17: 0.0,
			iTempPerm555: 0,
			fTempPerm556: 0.0,
			iTempPerm557: 0,
			iTempPerm558: 0,
			iTempPerm559: 0,
			iTempPerm560: 0,
			iTempPerm561: 0,
			iTempPerm562: 0,
			iTempPerm563: 0,
			fTempPerm564: 0.0,
			iTempPerm565: 0,
			iTempPerm566: 0,
			fTempPerm567: 0.0,
			fTempPerm568: 0.0,
			fTempPerm569: 0.0,
			fTempPerm570: 0.0,
			fTempPerm571: 0.0,
			fEntry18: 0.0,
			iTempPerm572: 0,
			fTempPerm573: 0.0,
			fTempPerm574: 0.0,
			fHslider20: 0.0,
			fVec71: [0.0;2],
			fTempPerm575: 0.0,
			fRec74: [0.0;2],
			fRec75: [0.0;2],
			fTempPerm576: 0.0,
			fTempPerm577: 0.0,
			fTempPerm578: 0.0,
			fTempPerm579: 0.0,
			iTempPerm580: 0,
			iTempPerm581: 0,
			iTempPerm582: 0,
			iTempPerm583: 0,
			iTempPerm584: 0,
			iTempPerm585: 0,
			fTempPerm586: 0.0,
			fTempPerm587: 0.0,
			iTempPerm588: 0,
			iTempPerm589: 0,
			iTempPerm590: 0,
			iTempPerm591: 0,
			fTempPerm592: 0.0,
			fTempPerm593: 0.0,
			fTempPerm594: 0.0,
			iTempPerm595: 0,
			fTempPerm596: 0.0,
			fTempPerm597: 0.0,
			fTempPerm598: 0.0,
			fTempPerm599: 0.0,
			iTempPerm600: 0,
			iTempPerm601: 0,
			iTempPerm602: 0,
			iTempPerm603: 0,
			iTempPerm604: 0,
			iTempPerm605: 0,
			iTempPerm606: 0,
			fTempPerm607: 0.0,
			iTempPerm608: 0,
			iTempPerm609: 0,
			fTempPerm610: 0.0,
			fTempPerm611: 0.0,
			fTempPerm612: 0.0,
			fTempPerm613: 0.0,
			fTempPerm614: 0.0,
			fTempPerm615: 0.0,
			fTempPerm616: 0.0,
			fTempPerm617: 0.0,
			iTempPerm618: 0,
			iTempPerm619: 0,
			fTempPerm620: 0.0,
			fTempPerm621: 0.0,
			fTempPerm622: 0.0,
			fTempPerm623: 0.0,
			fTempPerm624: 0.0,
			fTempPerm625: 0.0,
			fTempPerm626: 0.0,
			fRec76: [0.0;3],
			fTempPerm627: 0.0,
			fTempPerm628: 0.0,
			fTempPerm629: 0.0,
			fTempPerm630: 0.0,
			fTempPerm631: 0.0,
			fRec78: [0.0;2],
			fRec77: [0.0;2],
			fTempPerm632: 0.0,
			fTempPerm633: 0.0,
			fTempPerm634: 0.0,
			fTempPerm635: 0.0,
			fTempPerm636: 0.0,
			fTempPerm637: 0.0,
			fTempPerm638: 0.0,
			fTempPerm639: 0.0,
			fHslider21: 0.0,
			fVec72: [0.0;2],
			fRec79: [0.0;2],
			fRec80: [0.0;2],
			fHslider22: 0.0,
			fVec73: [0.0;2],
			fRec81: [0.0;2],
			fRec82: [0.0;2],
			fEntry19: 0.0,
			fCheckbox1: 0.0,
			fHslider23: 0.0,
			fHslider24: 0.0,
			fHslider25: 0.0,
			fHslider26: 0.0,
			fEntry20: 0.0,
			fHslider27: 0.0,
			fHslider28: 0.0,
			fHslider29: 0.0,
			fHslider30: 0.0,
			fHslider31: 0.0,
			fEntry21: 0.0,
			fCheckbox2: 0.0,
			fHslider32: 0.0,
			fHslider33: 0.0,
			fHslider34: 0.0,
			fHslider35: 0.0,
			fEntry22: 0.0,
			fCheckbox3: 0.0,
			fHslider36: 0.0,
			fHslider37: 0.0,
			fHslider38: 0.0,
			fHslider39: 0.0,
			fEntry23: 0.0,
			fCheckbox4: 0.0,
			fEntry24: 0.0,
			fHslider40: 0.0,
			fHslider41: 0.0,
			fHslider42: 0.0,
			fHslider43: 0.0,
			fEntry25: 0.0,
			fCheckbox5: 0.0,
			fHslider44: 0.0,
			fHslider45: 0.0,
			fHslider46: 0.0,
			fHslider47: 0.0,
			fEntry26: 0.0,
			fCheckbox6: 0.0,
			fHslider48: 0.0,
			fHslider49: 0.0,
			fHslider50: 0.0,
			fHslider51: 0.0,
			fEntry27: 0.0,
			fCheckbox7: 0.0,
			fEntry28: 0.0,
			fHslider52: 0.0,
			fHslider53: 0.0,
			fHslider54: 0.0,
			fHslider55: 0.0,
			fHslider56: 0.0,
			fHslider57: 0.0,
			fHslider58: 0.0,
			fEntry29: 0.0,
			fEntry30: 0.0,
			fHslider59: 0.0,
			fHslider60: 0.0,
			fHslider61: 0.0,
			fHslider62: 0.0,
			fHslider63: 0.0,
			fHslider64: 0.0,
			fHslider65: 0.0,
			fEntry31: 0.0,
			fEntry32: 0.0,
			fHslider66: 0.0,
			fHslider67: 0.0,
			fHslider68: 0.0,
			fHslider69: 0.0,
			fHslider70: 0.0,
			fHslider71: 0.0,
			fHslider72: 0.0,
			fHslider73: 0.0,
			fCheckbox8: 0.0,
			fEntry33: 0.0,
			fEntry34: 0.0,
			fHslider74: 0.0,
			fHslider75: 0.0,
			fHslider76: 0.0,
			fHslider77: 0.0,
			fHslider78: 0.0,
			fHslider79: 0.0,
			fHslider80: 0.0,
			fHslider81: 0.0,
			fCheckbox9: 0.0,
			fEntry35: 0.0,
			fEntry36: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn OscDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("filename", r"plugin-osc.dsp");
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
		m.declare("name", r"plugin-osc");
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
		return 0;
	}
	fn get_num_outputs(&self) -> i32 {
		return 6;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: OscDspSIG0 = newOscDspSIG0();
		sig0.instance_initOscDspSIG0(sample_rate);
		sig0.fillOscDspSIG0(65536, unsafe { &mut ftbl0OscDspSIG0 });
		let mut sig1: OscDspSIG1 = newOscDspSIG1();
		sig1.instance_initOscDspSIG1(sample_rate);
		sig1.fillOscDspSIG1(12, unsafe { &mut ftbl1OscDspSIG1 });
		let mut sig2: OscDspSIG2 = newOscDspSIG2();
		sig2.instance_initOscDspSIG2(sample_rate);
		sig2.fillOscDspSIG2(10, unsafe { &mut ftbl2OscDspSIG2 });
		let mut sig3: OscDspSIG3 = newOscDspSIG3();
		sig3.instance_initOscDspSIG3(sample_rate);
		sig3.fillOscDspSIG3(5, unsafe { &mut itbl3OscDspSIG3 });
		let mut sig4: OscDspSIG4 = newOscDspSIG4();
		sig4.instance_initOscDspSIG4(sample_rate);
		sig4.fillOscDspSIG4(12, unsafe { &mut ftbl4OscDspSIG4 });
	}
	fn instance_reset_params(&mut self) {
		self.fVslider0 = 1.0;
		self.fEntry0 = 0.0;
		self.fEntry1 = 0.0;
		self.fEntry2 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 64.0;
		self.fHslider2 = 0.0;
		self.fEntry3 = 0.0;
		self.fEntry4 = 1.0;
		self.fVslider1 = 0.0;
		self.fHslider3 = 0.0;
		self.fHslider4 = 1.0;
		self.fHslider5 = 0.0;
		self.fEntry5 = 0.0;
		self.fHslider6 = 0.0;
		self.fHslider7 = 1.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 1.1e+02;
		self.fHslider10 = 3.0;
		self.fHslider11 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fEntry6 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.5;
		self.fEntry7 = 2.0;
		self.fEntry8 = 2.0;
		self.fHslider14 = 0.0;
		self.fEntry9 = 11.0;
		self.fEntry10 = 21.0;
		self.fEntry11 = 32.0;
		self.fEntry12 = 0.0;
		self.fHslider15 = 0.5;
		self.fHslider16 = 0.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 0.5;
		self.fEntry13 = 2.0;
		self.fEntry14 = 2.0;
		self.fHslider19 = 0.0;
		self.fEntry15 = 11.0;
		self.fEntry16 = 21.0;
		self.fEntry17 = 32.0;
		self.fEntry18 = 0.0;
		self.fHslider20 = 0.5;
		self.fHslider21 = 64.0;
		self.fHslider22 = 64.0;
		self.fEntry19 = 0.0;
		self.fCheckbox1 = 0.0;
		self.fHslider23 = 0.0;
		self.fHslider24 = 1.0;
		self.fHslider25 = 0.0;
		self.fHslider26 = 0.0;
		self.fEntry20 = 0.0;
		self.fHslider27 = 1e+02;
		self.fHslider28 = 0.0;
		self.fHslider29 = 0.15;
		self.fHslider30 = 0.0;
		self.fHslider31 = 0.6666667;
		self.fEntry21 = 1.0;
		self.fCheckbox2 = 0.0;
		self.fHslider32 = 0.0;
		self.fHslider33 = 0.15;
		self.fHslider34 = 0.0;
		self.fHslider35 = 0.33333334;
		self.fEntry22 = 4.0;
		self.fCheckbox3 = 0.0;
		self.fHslider36 = 0.0;
		self.fHslider37 = 0.15;
		self.fHslider38 = 0.0;
		self.fHslider39 = 0.0;
		self.fEntry23 = 0.0;
		self.fCheckbox4 = 0.0;
		self.fEntry24 = 3.0;
		self.fHslider40 = 0.0;
		self.fHslider41 = 0.15;
		self.fHslider42 = 0.0;
		self.fHslider43 = 0.6666667;
		self.fEntry25 = 1.0;
		self.fCheckbox5 = 0.0;
		self.fHslider44 = 0.0;
		self.fHslider45 = 0.15;
		self.fHslider46 = 0.0;
		self.fHslider47 = 0.33333334;
		self.fEntry26 = 4.0;
		self.fCheckbox6 = 0.0;
		self.fHslider48 = 0.0;
		self.fHslider49 = 0.15;
		self.fHslider50 = 0.0;
		self.fHslider51 = 0.0;
		self.fEntry27 = 0.0;
		self.fCheckbox7 = 0.0;
		self.fEntry28 = 3.0;
		self.fHslider52 = 0.0;
		self.fHslider53 = 0.0;
		self.fHslider54 = 0.5;
		self.fHslider55 = 1e+02;
		self.fHslider56 = 1e+02;
		self.fHslider57 = 0.0;
		self.fHslider58 = 0.0;
		self.fEntry29 = 31.0;
		self.fEntry30 = 52.0;
		self.fHslider59 = 0.0;
		self.fHslider60 = 0.0;
		self.fHslider61 = 0.5;
		self.fHslider62 = 1e+02;
		self.fHslider63 = 1e+02;
		self.fHslider64 = 0.0;
		self.fHslider65 = 0.0;
		self.fEntry31 = 39.0;
		self.fEntry32 = 34.0;
		self.fHslider66 = 0.0;
		self.fHslider67 = 0.3;
		self.fHslider68 = 0.0;
		self.fHslider69 = 0.0;
		self.fHslider70 = 0.0;
		self.fHslider71 = 0.0;
		self.fHslider72 = 0.0;
		self.fHslider73 = 0.0;
		self.fCheckbox8 = 0.0;
		self.fEntry33 = 6821.0;
		self.fEntry34 = 3.0;
		self.fHslider74 = 0.0;
		self.fHslider75 = 0.3;
		self.fHslider76 = 0.0;
		self.fHslider77 = 0.0;
		self.fHslider78 = 0.0;
		self.fHslider79 = 0.0;
		self.fHslider80 = 0.0;
		self.fHslider81 = 0.0;
		self.fCheckbox9 = 0.0;
		self.fEntry35 = 4821.0;
		self.fEntry36 = 0.0;
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
			self.fRec1[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec2[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fVec3[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec3[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec4[l7 as usize] = 0.0;
		}
		self.iTempPerm0 = 0;
		self.fTempPerm1 = 0.0;
		for l10 in 0..2 {
			self.fRec6[l10 as usize] = 0.0;
		}
		self.fTempPerm2 = 0.0;
		for l11 in 0..2 {
			self.fRec7[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.iRec9[l12 as usize] = 0;
		}
		self.iTempPerm3 = 0;
		for l13 in 0..2 {
			self.fRec8[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec10[l14 as usize] = 0.0;
		}
		self.fTempPerm4 = 0.0;
		for l15 in 0..2 {
			self.fRec13[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.iRec14[l16 as usize] = 0;
		}
		for l17 in 0..2 {
			self.fVec5[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec12[l18 as usize] = 0.0;
		}
		self.fTempPerm5 = 0.0;
		for l19 in 0..2 {
			self.fVec6[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec11[l20 as usize] = 0.0;
		}
		self.fTempPerm6 = 0.0;
		for l21 in 0..2 {
			self.fRec15[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec7[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec16[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec17[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec8[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec18[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec19[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec20[l28 as usize] = 0.0;
		}
		self.fTempPerm7 = 0.0;
		self.fTempPerm8 = 0.0;
		for l29 in 0..2 {
			self.fRec21[l29 as usize] = 0.0;
		}
		self.fTempPerm9 = 0.0;
		for l30 in 0..2 {
			self.fRec0[l30 as usize] = 0.0;
		}
		self.iTempPerm10 = 0;
		for l31 in 0..2 {
			self.fVec9[l31 as usize] = 0.0;
		}
		self.fTempPerm11 = 0.0;
		for l32 in 0..2 {
			self.fRec22[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec23[l33 as usize] = 0.0;
		}
		self.fTempPerm12 = 0.0;
		for l34 in 0..2 {
			self.fVec10[l34 as usize] = 0.0;
		}
		self.fTempPerm13 = 0.0;
		for l35 in 0..2 {
			self.fRec24[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec25[l36 as usize] = 0.0;
		}
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
		self.iTempPerm24 = 0;
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
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		for l37 in 0..2 {
			self.fVec11[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fVec12[l38 as usize] = 0.0;
		}
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		for l39 in 0..2 {
			self.fRec26[l39 as usize] = 0.0;
		}
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		for l40 in 0..2 {
			self.fVec13[l40 as usize] = 0.0;
		}
		self.fTempPerm52 = 0.0;
		self.iTempPerm53 = 0;
		for l41 in 0..2 {
			self.iRec27[l41 as usize] = 0;
		}
		for l42 in 0..2 {
			self.fVec14[l42 as usize] = 0.0;
		}
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
		for l43 in 0..2 {
			self.fVec15[l43 as usize] = 0.0;
		}
		self.fTempPerm67 = 0.0;
		self.fTempPerm68 = 0.0;
		for l44 in 0..2 {
			self.fVec16[l44 as usize] = 0.0;
		}
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		self.fTempPerm71 = 0.0;
		self.fTempPerm72 = 0.0;
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		self.iTempPerm75 = 0;
		self.fTempPerm76 = 0.0;
		self.fTempPerm77 = 0.0;
		self.fTempPerm78 = 0.0;
		for l45 in 0..2 {
			self.fVec17[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec18[l46 as usize] = 0.0;
		}
		self.fTempPerm79 = 0.0;
		for l47 in 0..2 {
			self.fVec19[l47 as usize] = 0.0;
		}
		self.fTempPerm80 = 0.0;
		self.iTempPerm81 = 0;
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.fTempPerm84 = 0.0;
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		self.fTempPerm87 = 0.0;
		self.fTempPerm88 = 0.0;
		self.iTempPerm89 = 0;
		self.iTempPerm90 = 0;
		self.iTempPerm91 = 0;
		self.iTempPerm92 = 0;
		self.iTempPerm93 = 0;
		self.iTempPerm94 = 0;
		self.fTempPerm95 = 0.0;
		self.fTempPerm96 = 0.0;
		self.fTempPerm97 = 0.0;
		self.iTempPerm98 = 0;
		self.iTempPerm99 = 0;
		self.iTempPerm100 = 0;
		self.iTempPerm101 = 0;
		self.fTempPerm102 = 0.0;
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		for l48 in 0..2 {
			self.fVec20[l48 as usize] = 0.0;
		}
		self.fTempPerm105 = 0.0;
		self.fTempPerm106 = 0.0;
		self.fTempPerm107 = 0.0;
		self.fTempPerm108 = 0.0;
		self.fTempPerm109 = 0.0;
		for l49 in 0..2 {
			self.fVec21[l49 as usize] = 0.0;
		}
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		self.fTempPerm112 = 0.0;
		self.fTempPerm113 = 0.0;
		self.fTempPerm114 = 0.0;
		for l50 in 0..2 {
			self.fRec28[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fVec22[l51 as usize] = 0.0;
		}
		self.fTempPerm115 = 0.0;
		self.fTempPerm116 = 0.0;
		self.fTempPerm117 = 0.0;
		self.fTempPerm118 = 0.0;
		for l52 in 0..2 {
			self.fRec29[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fVec23[l53 as usize] = 0.0;
		}
		self.fTempPerm119 = 0.0;
		for l54 in 0..2 {
			self.fVec24[l54 as usize] = 0.0;
		}
		self.fTempPerm120 = 0.0;
		self.iTempPerm121 = 0;
		self.fTempPerm122 = 0.0;
		self.fTempPerm123 = 0.0;
		self.fTempPerm124 = 0.0;
		self.fTempPerm125 = 0.0;
		self.fTempPerm126 = 0.0;
		for l55 in 0..2 {
			self.fRec30[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fVec25[l56 as usize] = 0.0;
		}
		self.fTempPerm127 = 0.0;
		self.fTempPerm128 = 0.0;
		self.fTempPerm129 = 0.0;
		self.fTempPerm130 = 0.0;
		self.fTempPerm131 = 0.0;
		for l57 in 0..2 {
			self.fRec31[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fVec26[l58 as usize] = 0.0;
		}
		self.fTempPerm132 = 0.0;
		self.fTempPerm133 = 0.0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		self.iTempPerm136 = 0;
		self.fTempPerm137 = 0.0;
		self.fTempPerm138 = 0.0;
		self.fTempPerm139 = 0.0;
		for l59 in 0..2 {
			self.fRec32[l59 as usize] = 0.0;
		}
		self.fTempPerm140 = 0.0;
		self.iTempPerm141 = 0;
		self.fTempPerm142 = 0.0;
		self.iTempPerm143 = 0;
		self.fTempPerm144 = 0.0;
		self.fTempPerm145 = 0.0;
		self.fTempPerm146 = 0.0;
		for l60 in 0..2 {
			self.fVec27[l60 as usize] = 0.0;
		}
		self.fTempPerm147 = 0.0;
		self.fTempPerm148 = 0.0;
		self.iTempPerm149 = 0;
		self.fTempPerm150 = 0.0;
		self.fTempPerm151 = 0.0;
		self.fTempPerm152 = 0.0;
		for l61 in 0..2 {
			self.fRec33[l61 as usize] = 0.0;
		}
		self.fTempPerm153 = 0.0;
		self.fTempPerm154 = 0.0;
		self.fTempPerm155 = 0.0;
		self.fTempPerm156 = 0.0;
		self.fTempPerm157 = 0.0;
		self.fTempPerm158 = 0.0;
		self.fTempPerm159 = 0.0;
		self.fTempPerm160 = 0.0;
		self.fTempPerm161 = 0.0;
		for l62 in 0..2 {
			self.fVec28[l62 as usize] = 0.0;
		}
		self.fTempPerm162 = 0.0;
		self.fTempPerm163 = 0.0;
		for l63 in 0..2 {
			self.fVec29[l63 as usize] = 0.0;
		}
		self.fTempPerm164 = 0.0;
		self.fTempPerm165 = 0.0;
		self.fTempPerm166 = 0.0;
		self.fTempPerm167 = 0.0;
		for l64 in 0..2 {
			self.fRec34[l64 as usize] = 0.0;
		}
		self.fTempPerm168 = 0.0;
		self.fTempPerm169 = 0.0;
		self.fTempPerm170 = 0.0;
		self.fTempPerm171 = 0.0;
		self.iTempPerm172 = 0;
		for l65 in 0..2 {
			self.fRec35[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec36[l66 as usize] = 0.0;
		}
		self.fTempPerm173 = 0.0;
		self.iTempPerm174 = 0;
		self.fTempPerm175 = 0.0;
		self.iTempPerm176 = 0;
		self.fTempPerm177 = 0.0;
		self.iTempPerm178 = 0;
		self.fTempPerm179 = 0.0;
		self.fTempPerm180 = 0.0;
		self.fTempPerm181 = 0.0;
		for l67 in 0..2 {
			self.fVec30[l67 as usize] = 0.0;
		}
		self.fTempPerm182 = 0.0;
		self.iTempPerm183 = 0;
		self.fTempPerm184 = 0.0;
		self.fTempPerm185 = 0.0;
		self.fTempPerm186 = 0.0;
		for l68 in 0..2 {
			self.fRec37[l68 as usize] = 0.0;
		}
		self.fTempPerm187 = 0.0;
		self.fTempPerm188 = 0.0;
		self.fTempPerm189 = 0.0;
		self.fTempPerm190 = 0.0;
		self.fTempPerm191 = 0.0;
		self.fTempPerm192 = 0.0;
		self.fTempPerm193 = 0.0;
		self.fTempPerm194 = 0.0;
		self.fTempPerm195 = 0.0;
		self.fTempPerm196 = 0.0;
		self.fTempPerm197 = 0.0;
		for l69 in 0..2 {
			self.fVec31[l69 as usize] = 0.0;
		}
		self.fTempPerm198 = 0.0;
		self.fTempPerm199 = 0.0;
		for l70 in 0..2 {
			self.fVec32[l70 as usize] = 0.0;
		}
		self.fTempPerm200 = 0.0;
		self.iTempPerm201 = 0;
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
		for l71 in 0..2 {
			self.fVec33[l71 as usize] = 0.0;
		}
		self.fTempPerm215 = 0.0;
		self.fTempPerm216 = 0.0;
		for l72 in 0..2 {
			self.fVec34[l72 as usize] = 0.0;
		}
		self.fTempPerm217 = 0.0;
		for l73 in 0..2 {
			self.fVec35[l73 as usize] = 0.0;
		}
		self.fTempPerm218 = 0.0;
		self.iTempPerm219 = 0;
		self.fTempPerm220 = 0.0;
		for l74 in 0..2 {
			self.fVec36[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec38[l75 as usize] = 0.0;
		}
		self.fTempPerm221 = 0.0;
		self.fTempPerm222 = 0.0;
		self.fTempPerm223 = 0.0;
		self.iTempPerm224 = 0;
		self.fTempPerm225 = 0.0;
		self.iTempPerm226 = 0;
		self.fTempPerm227 = 0.0;
		self.fTempPerm228 = 0.0;
		self.fTempPerm229 = 0.0;
		self.fTempPerm230 = 0.0;
		self.fTempPerm231 = 0.0;
		for l76 in 0..2 {
			self.fRec39[l76 as usize] = 0.0;
		}
		self.fTempPerm232 = 0.0;
		self.fTempPerm233 = 0.0;
		self.fTempPerm234 = 0.0;
		self.fTempPerm235 = 0.0;
		for l77 in 0..2 {
			self.fRec40[l77 as usize] = 0.0;
		}
		self.fTempPerm236 = 0.0;
		self.fTempPerm237 = 0.0;
		self.fTempPerm238 = 0.0;
		for l78 in 0..2 {
			self.fRec41[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fVec37[l79 as usize] = 0.0;
		}
		self.fTempPerm239 = 0.0;
		self.fTempPerm240 = 0.0;
		for l80 in 0..2 {
			self.fVec38[l80 as usize] = 0.0;
		}
		self.fTempPerm241 = 0.0;
		for l81 in 0..2 {
			self.fRec42[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec43[l82 as usize] = 0.0;
		}
		self.fTempPerm242 = 0.0;
		self.iTempPerm243 = 0;
		self.iTempPerm244 = 0;
		self.iTempPerm245 = 0;
		self.iTempPerm246 = 0;
		self.fTempPerm247 = 0.0;
		self.iTempPerm248 = 0;
		self.iTempPerm249 = 0;
		self.iTempPerm250 = 0;
		self.iTempPerm251 = 0;
		self.iTempPerm252 = 0;
		self.iTempPerm253 = 0;
		self.iTempPerm254 = 0;
		self.fTempPerm255 = 0.0;
		self.iTempPerm256 = 0;
		self.iTempPerm257 = 0;
		self.fTempPerm258 = 0.0;
		self.fTempPerm259 = 0.0;
		self.fTempPerm260 = 0.0;
		self.fTempPerm261 = 0.0;
		self.fTempPerm262 = 0.0;
		self.iTempPerm263 = 0;
		self.fTempPerm264 = 0.0;
		self.fTempPerm265 = 0.0;
		for l83 in 0..2 {
			self.fVec39[l83 as usize] = 0.0;
		}
		self.fTempPerm266 = 0.0;
		for l84 in 0..2 {
			self.fRec44[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec45[l85 as usize] = 0.0;
		}
		self.fTempPerm267 = 0.0;
		self.fTempPerm268 = 0.0;
		self.fTempPerm269 = 0.0;
		self.fTempPerm270 = 0.0;
		self.iTempPerm271 = 0;
		self.iTempPerm272 = 0;
		self.iTempPerm273 = 0;
		self.iTempPerm274 = 0;
		self.iTempPerm275 = 0;
		self.iTempPerm276 = 0;
		self.fTempPerm277 = 0.0;
		self.fTempPerm278 = 0.0;
		self.iTempPerm279 = 0;
		self.iTempPerm280 = 0;
		self.iTempPerm281 = 0;
		self.iTempPerm282 = 0;
		self.fTempPerm283 = 0.0;
		self.fTempPerm284 = 0.0;
		self.fTempPerm285 = 0.0;
		self.iTempPerm286 = 0;
		self.fTempPerm287 = 0.0;
		self.fTempPerm288 = 0.0;
		self.fTempPerm289 = 0.0;
		self.fTempPerm290 = 0.0;
		self.iTempPerm291 = 0;
		self.iTempPerm292 = 0;
		self.iTempPerm293 = 0;
		self.iTempPerm294 = 0;
		self.iTempPerm295 = 0;
		self.iTempPerm296 = 0;
		self.iTempPerm297 = 0;
		self.fTempPerm298 = 0.0;
		self.iTempPerm299 = 0;
		self.iTempPerm300 = 0;
		self.fTempPerm301 = 0.0;
		self.fTempPerm302 = 0.0;
		self.fTempPerm303 = 0.0;
		self.fTempPerm304 = 0.0;
		self.fTempPerm305 = 0.0;
		self.fTempPerm306 = 0.0;
		self.fTempPerm307 = 0.0;
		self.fTempPerm308 = 0.0;
		self.iTempPerm309 = 0;
		self.iTempPerm310 = 0;
		self.fTempPerm311 = 0.0;
		self.fTempPerm312 = 0.0;
		self.fTempPerm313 = 0.0;
		self.fTempPerm314 = 0.0;
		self.fTempPerm315 = 0.0;
		self.fTempPerm316 = 0.0;
		self.fTempPerm317 = 0.0;
		for l86 in 0..3 {
			self.fRec46[l86 as usize] = 0.0;
		}
		self.fTempPerm318 = 0.0;
		self.fTempPerm319 = 0.0;
		self.fTempPerm320 = 0.0;
		self.fTempPerm321 = 0.0;
		self.fTempPerm322 = 0.0;
		for l87 in 0..2 {
			self.fRec48[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec47[l88 as usize] = 0.0;
		}
		self.fTempPerm323 = 0.0;
		self.fTempPerm324 = 0.0;
		self.fTempPerm325 = 0.0;
		self.fTempPerm326 = 0.0;
		self.fTempPerm327 = 0.0;
		self.fTempPerm328 = 0.0;
		self.fTempPerm329 = 0.0;
		self.fTempPerm330 = 0.0;
		for l89 in 0..2 {
			self.fVec40[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec50[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec51[l91 as usize] = 0.0;
		}
		self.fTempPerm331 = 0.0;
		self.fTempPerm332 = 0.0;
		for l92 in 0..2 {
			self.fRec49[l92 as usize] = 0.0;
		}
		self.iTempPerm333 = 0;
		for l93 in 0..2 {
			self.fVec41[l93 as usize] = 0.0;
		}
		self.fTempPerm334 = 0.0;
		for l94 in 0..2 {
			self.fRec52[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec53[l95 as usize] = 0.0;
		}
		self.fTempPerm335 = 0.0;
		for l96 in 0..2 {
			self.fVec42[l96 as usize] = 0.0;
		}
		self.fTempPerm336 = 0.0;
		for l97 in 0..2 {
			self.fRec54[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec55[l98 as usize] = 0.0;
		}
		self.fTempPerm337 = 0.0;
		self.fTempPerm338 = 0.0;
		self.fTempPerm339 = 0.0;
		self.fTempPerm340 = 0.0;
		self.fTempPerm341 = 0.0;
		self.fTempPerm342 = 0.0;
		self.fTempPerm343 = 0.0;
		self.fTempPerm344 = 0.0;
		self.fTempPerm345 = 0.0;
		self.fTempPerm346 = 0.0;
		self.iTempPerm347 = 0;
		self.fTempPerm348 = 0.0;
		self.fTempPerm349 = 0.0;
		self.fTempPerm350 = 0.0;
		self.fTempPerm351 = 0.0;
		self.fTempPerm352 = 0.0;
		self.fTempPerm353 = 0.0;
		self.fTempPerm354 = 0.0;
		self.fTempPerm355 = 0.0;
		self.fTempPerm356 = 0.0;
		self.fTempPerm357 = 0.0;
		self.fTempPerm358 = 0.0;
		self.fTempPerm359 = 0.0;
		self.fTempPerm360 = 0.0;
		self.fTempPerm361 = 0.0;
		self.fTempPerm362 = 0.0;
		for l99 in 0..2 {
			self.fVec43[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fVec44[l100 as usize] = 0.0;
		}
		self.fTempPerm363 = 0.0;
		self.fTempPerm364 = 0.0;
		self.fTempPerm365 = 0.0;
		for l101 in 0..2 {
			self.fRec56[l101 as usize] = 0.0;
		}
		self.fTempPerm366 = 0.0;
		self.fTempPerm367 = 0.0;
		self.fTempPerm368 = 0.0;
		self.fTempPerm369 = 0.0;
		self.fTempPerm370 = 0.0;
		self.fTempPerm371 = 0.0;
		for l102 in 0..2 {
			self.fVec45[l102 as usize] = 0.0;
		}
		self.fTempPerm372 = 0.0;
		self.iTempPerm373 = 0;
		for l103 in 0..2 {
			self.iRec57[l103 as usize] = 0;
		}
		for l104 in 0..2 {
			self.fVec46[l104 as usize] = 0.0;
		}
		self.fTempPerm374 = 0.0;
		self.fTempPerm375 = 0.0;
		self.fTempPerm376 = 0.0;
		self.fTempPerm377 = 0.0;
		self.fTempPerm378 = 0.0;
		self.fTempPerm379 = 0.0;
		self.fTempPerm380 = 0.0;
		self.fTempPerm381 = 0.0;
		self.fTempPerm382 = 0.0;
		self.fTempPerm383 = 0.0;
		self.fTempPerm384 = 0.0;
		self.fTempPerm385 = 0.0;
		self.fTempPerm386 = 0.0;
		for l105 in 0..2 {
			self.fVec47[l105 as usize] = 0.0;
		}
		self.fTempPerm387 = 0.0;
		self.fTempPerm388 = 0.0;
		for l106 in 0..2 {
			self.fVec48[l106 as usize] = 0.0;
		}
		self.fTempPerm389 = 0.0;
		self.fTempPerm390 = 0.0;
		self.fTempPerm391 = 0.0;
		self.fTempPerm392 = 0.0;
		self.fTempPerm393 = 0.0;
		self.fTempPerm394 = 0.0;
		self.iTempPerm395 = 0;
		self.fTempPerm396 = 0.0;
		self.fTempPerm397 = 0.0;
		self.fTempPerm398 = 0.0;
		for l107 in 0..2 {
			self.fVec49[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fVec50[l108 as usize] = 0.0;
		}
		self.fTempPerm399 = 0.0;
		for l109 in 0..2 {
			self.fVec51[l109 as usize] = 0.0;
		}
		self.fTempPerm400 = 0.0;
		self.iTempPerm401 = 0;
		self.fTempPerm402 = 0.0;
		self.fTempPerm403 = 0.0;
		self.fTempPerm404 = 0.0;
		self.fTempPerm405 = 0.0;
		self.fTempPerm406 = 0.0;
		self.fTempPerm407 = 0.0;
		self.fTempPerm408 = 0.0;
		self.iTempPerm409 = 0;
		self.iTempPerm410 = 0;
		self.iTempPerm411 = 0;
		self.iTempPerm412 = 0;
		self.iTempPerm413 = 0;
		self.iTempPerm414 = 0;
		self.fTempPerm415 = 0.0;
		self.fTempPerm416 = 0.0;
		self.fTempPerm417 = 0.0;
		self.iTempPerm418 = 0;
		self.iTempPerm419 = 0;
		self.iTempPerm420 = 0;
		self.iTempPerm421 = 0;
		self.fTempPerm422 = 0.0;
		self.fTempPerm423 = 0.0;
		self.fTempPerm424 = 0.0;
		for l110 in 0..2 {
			self.fVec52[l110 as usize] = 0.0;
		}
		self.fTempPerm425 = 0.0;
		self.fTempPerm426 = 0.0;
		self.fTempPerm427 = 0.0;
		self.fTempPerm428 = 0.0;
		self.fTempPerm429 = 0.0;
		for l111 in 0..2 {
			self.fVec53[l111 as usize] = 0.0;
		}
		self.fTempPerm430 = 0.0;
		self.fTempPerm431 = 0.0;
		self.fTempPerm432 = 0.0;
		for l112 in 0..2 {
			self.fRec58[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fVec54[l113 as usize] = 0.0;
		}
		self.fTempPerm433 = 0.0;
		self.fTempPerm434 = 0.0;
		for l114 in 0..2 {
			self.fRec59[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fVec55[l115 as usize] = 0.0;
		}
		self.fTempPerm435 = 0.0;
		for l116 in 0..2 {
			self.fVec56[l116 as usize] = 0.0;
		}
		self.fTempPerm436 = 0.0;
		self.iTempPerm437 = 0;
		self.fTempPerm438 = 0.0;
		self.fTempPerm439 = 0.0;
		self.fTempPerm440 = 0.0;
		self.fTempPerm441 = 0.0;
		self.fTempPerm442 = 0.0;
		for l117 in 0..2 {
			self.fRec60[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fVec57[l118 as usize] = 0.0;
		}
		self.fTempPerm443 = 0.0;
		self.fTempPerm444 = 0.0;
		self.fTempPerm445 = 0.0;
		self.fTempPerm446 = 0.0;
		self.fTempPerm447 = 0.0;
		for l119 in 0..2 {
			self.fRec61[l119 as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fVec58[l120 as usize] = 0.0;
		}
		self.fTempPerm448 = 0.0;
		self.fTempPerm449 = 0.0;
		self.fTempPerm450 = 0.0;
		self.fTempPerm451 = 0.0;
		self.iTempPerm452 = 0;
		self.fTempPerm453 = 0.0;
		self.fTempPerm454 = 0.0;
		self.fTempPerm455 = 0.0;
		for l121 in 0..2 {
			self.fRec62[l121 as usize] = 0.0;
		}
		self.fTempPerm456 = 0.0;
		self.iTempPerm457 = 0;
		self.fTempPerm458 = 0.0;
		self.iTempPerm459 = 0;
		self.fTempPerm460 = 0.0;
		self.fTempPerm461 = 0.0;
		self.fTempPerm462 = 0.0;
		for l122 in 0..2 {
			self.fVec59[l122 as usize] = 0.0;
		}
		self.fTempPerm463 = 0.0;
		self.fTempPerm464 = 0.0;
		self.iTempPerm465 = 0;
		self.fTempPerm466 = 0.0;
		self.fTempPerm467 = 0.0;
		self.fTempPerm468 = 0.0;
		for l123 in 0..2 {
			self.fRec63[l123 as usize] = 0.0;
		}
		self.fTempPerm469 = 0.0;
		self.fTempPerm470 = 0.0;
		self.fTempPerm471 = 0.0;
		self.fTempPerm472 = 0.0;
		self.fTempPerm473 = 0.0;
		self.fTempPerm474 = 0.0;
		self.fTempPerm475 = 0.0;
		self.fTempPerm476 = 0.0;
		self.fTempPerm477 = 0.0;
		for l124 in 0..2 {
			self.fVec60[l124 as usize] = 0.0;
		}
		self.fTempPerm478 = 0.0;
		self.fTempPerm479 = 0.0;
		for l125 in 0..2 {
			self.fVec61[l125 as usize] = 0.0;
		}
		self.fTempPerm480 = 0.0;
		self.fTempPerm481 = 0.0;
		self.fTempPerm482 = 0.0;
		for l126 in 0..2 {
			self.fRec64[l126 as usize] = 0.0;
		}
		self.fTempPerm483 = 0.0;
		self.fTempPerm484 = 0.0;
		self.fTempPerm485 = 0.0;
		self.fTempPerm486 = 0.0;
		self.iTempPerm487 = 0;
		for l127 in 0..2 {
			self.fRec65[l127 as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec66[l128 as usize] = 0.0;
		}
		self.fTempPerm488 = 0.0;
		self.iTempPerm489 = 0;
		self.fTempPerm490 = 0.0;
		self.iTempPerm491 = 0;
		self.fTempPerm492 = 0.0;
		self.iTempPerm493 = 0;
		self.fTempPerm494 = 0.0;
		self.fTempPerm495 = 0.0;
		self.fTempPerm496 = 0.0;
		for l129 in 0..2 {
			self.fVec62[l129 as usize] = 0.0;
		}
		self.fTempPerm497 = 0.0;
		self.iTempPerm498 = 0;
		self.fTempPerm499 = 0.0;
		self.fTempPerm500 = 0.0;
		self.fTempPerm501 = 0.0;
		for l130 in 0..2 {
			self.fRec67[l130 as usize] = 0.0;
		}
		self.fTempPerm502 = 0.0;
		self.fTempPerm503 = 0.0;
		self.fTempPerm504 = 0.0;
		self.fTempPerm505 = 0.0;
		self.fTempPerm506 = 0.0;
		self.fTempPerm507 = 0.0;
		self.fTempPerm508 = 0.0;
		self.fTempPerm509 = 0.0;
		self.fTempPerm510 = 0.0;
		self.fTempPerm511 = 0.0;
		self.fTempPerm512 = 0.0;
		for l131 in 0..2 {
			self.fVec63[l131 as usize] = 0.0;
		}
		self.fTempPerm513 = 0.0;
		self.fTempPerm514 = 0.0;
		for l132 in 0..2 {
			self.fVec64[l132 as usize] = 0.0;
		}
		self.fTempPerm515 = 0.0;
		self.iTempPerm516 = 0;
		self.fTempPerm517 = 0.0;
		self.fTempPerm518 = 0.0;
		self.fTempPerm519 = 0.0;
		self.fTempPerm520 = 0.0;
		self.fTempPerm521 = 0.0;
		self.fTempPerm522 = 0.0;
		self.fTempPerm523 = 0.0;
		self.fTempPerm524 = 0.0;
		self.fTempPerm525 = 0.0;
		self.fTempPerm526 = 0.0;
		self.fTempPerm527 = 0.0;
		self.fTempPerm528 = 0.0;
		self.fTempPerm529 = 0.0;
		for l133 in 0..2 {
			self.fVec65[l133 as usize] = 0.0;
		}
		self.fTempPerm530 = 0.0;
		self.fTempPerm531 = 0.0;
		for l134 in 0..2 {
			self.fVec66[l134 as usize] = 0.0;
		}
		self.fTempPerm532 = 0.0;
		for l135 in 0..2 {
			self.fVec67[l135 as usize] = 0.0;
		}
		self.fTempPerm533 = 0.0;
		self.iTempPerm534 = 0;
		self.fTempPerm535 = 0.0;
		for l136 in 0..2 {
			self.fVec68[l136 as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec68[l137 as usize] = 0.0;
		}
		self.fTempPerm536 = 0.0;
		self.fTempPerm537 = 0.0;
		self.fTempPerm538 = 0.0;
		self.iTempPerm539 = 0;
		self.fTempPerm540 = 0.0;
		self.iTempPerm541 = 0;
		self.fTempPerm542 = 0.0;
		self.fTempPerm543 = 0.0;
		self.fTempPerm544 = 0.0;
		for l138 in 0..2 {
			self.fRec69[l138 as usize] = 0.0;
		}
		self.fTempPerm545 = 0.0;
		self.fTempPerm546 = 0.0;
		for l139 in 0..2 {
			self.fRec70[l139 as usize] = 0.0;
		}
		self.fTempPerm547 = 0.0;
		for l140 in 0..2 {
			self.fRec71[l140 as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fVec69[l141 as usize] = 0.0;
		}
		self.fTempPerm548 = 0.0;
		self.fTempPerm549 = 0.0;
		for l142 in 0..2 {
			self.fVec70[l142 as usize] = 0.0;
		}
		self.fTempPerm550 = 0.0;
		for l143 in 0..2 {
			self.fRec72[l143 as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec73[l144 as usize] = 0.0;
		}
		self.fTempPerm551 = 0.0;
		self.iTempPerm552 = 0;
		self.iTempPerm553 = 0;
		self.iTempPerm554 = 0;
		self.iTempPerm555 = 0;
		self.fTempPerm556 = 0.0;
		self.iTempPerm557 = 0;
		self.iTempPerm558 = 0;
		self.iTempPerm559 = 0;
		self.iTempPerm560 = 0;
		self.iTempPerm561 = 0;
		self.iTempPerm562 = 0;
		self.iTempPerm563 = 0;
		self.fTempPerm564 = 0.0;
		self.iTempPerm565 = 0;
		self.iTempPerm566 = 0;
		self.fTempPerm567 = 0.0;
		self.fTempPerm568 = 0.0;
		self.fTempPerm569 = 0.0;
		self.fTempPerm570 = 0.0;
		self.fTempPerm571 = 0.0;
		self.iTempPerm572 = 0;
		self.fTempPerm573 = 0.0;
		self.fTempPerm574 = 0.0;
		for l145 in 0..2 {
			self.fVec71[l145 as usize] = 0.0;
		}
		self.fTempPerm575 = 0.0;
		for l146 in 0..2 {
			self.fRec74[l146 as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec75[l147 as usize] = 0.0;
		}
		self.fTempPerm576 = 0.0;
		self.fTempPerm577 = 0.0;
		self.fTempPerm578 = 0.0;
		self.fTempPerm579 = 0.0;
		self.iTempPerm580 = 0;
		self.iTempPerm581 = 0;
		self.iTempPerm582 = 0;
		self.iTempPerm583 = 0;
		self.iTempPerm584 = 0;
		self.iTempPerm585 = 0;
		self.fTempPerm586 = 0.0;
		self.fTempPerm587 = 0.0;
		self.iTempPerm588 = 0;
		self.iTempPerm589 = 0;
		self.iTempPerm590 = 0;
		self.iTempPerm591 = 0;
		self.fTempPerm592 = 0.0;
		self.fTempPerm593 = 0.0;
		self.fTempPerm594 = 0.0;
		self.iTempPerm595 = 0;
		self.fTempPerm596 = 0.0;
		self.fTempPerm597 = 0.0;
		self.fTempPerm598 = 0.0;
		self.fTempPerm599 = 0.0;
		self.iTempPerm600 = 0;
		self.iTempPerm601 = 0;
		self.iTempPerm602 = 0;
		self.iTempPerm603 = 0;
		self.iTempPerm604 = 0;
		self.iTempPerm605 = 0;
		self.iTempPerm606 = 0;
		self.fTempPerm607 = 0.0;
		self.iTempPerm608 = 0;
		self.iTempPerm609 = 0;
		self.fTempPerm610 = 0.0;
		self.fTempPerm611 = 0.0;
		self.fTempPerm612 = 0.0;
		self.fTempPerm613 = 0.0;
		self.fTempPerm614 = 0.0;
		self.fTempPerm615 = 0.0;
		self.fTempPerm616 = 0.0;
		self.fTempPerm617 = 0.0;
		self.iTempPerm618 = 0;
		self.iTempPerm619 = 0;
		self.fTempPerm620 = 0.0;
		self.fTempPerm621 = 0.0;
		self.fTempPerm622 = 0.0;
		self.fTempPerm623 = 0.0;
		self.fTempPerm624 = 0.0;
		self.fTempPerm625 = 0.0;
		self.fTempPerm626 = 0.0;
		for l148 in 0..3 {
			self.fRec76[l148 as usize] = 0.0;
		}
		self.fTempPerm627 = 0.0;
		self.fTempPerm628 = 0.0;
		self.fTempPerm629 = 0.0;
		self.fTempPerm630 = 0.0;
		self.fTempPerm631 = 0.0;
		for l149 in 0..2 {
			self.fRec78[l149 as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec77[l150 as usize] = 0.0;
		}
		self.fTempPerm632 = 0.0;
		self.fTempPerm633 = 0.0;
		self.fTempPerm634 = 0.0;
		self.fTempPerm635 = 0.0;
		self.fTempPerm636 = 0.0;
		self.fTempPerm637 = 0.0;
		self.fTempPerm638 = 0.0;
		self.fTempPerm639 = 0.0;
		for l151 in 0..2 {
			self.fVec72[l151 as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec79[l152 as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec80[l153 as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fVec73[l154 as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fRec81[l155 as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec82[l156 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 6.5 / self.fConst0;
		self.fConst2 = 1.0 / self.fConst0;
		self.fConst3 = F32::exp(-(376.99112 / self.fConst0));
		let mut fConst4: F32 = 1.0 - self.fConst3;
		self.fConst5 = 4.656613e-10 * F32::sqrt(3.0 * ((self.fConst3 + 1.0) / fConst4));
		self.fConst6 = 1.0 - 44.1 / self.fConst0;
		self.fConst7 = 3.0 * fConst4;
		self.fConst8 = 3.1415927 / self.fConst0;
		self.fConst9 = 0.5 * self.fConst0;
		self.fConst10 = 0.33333334 / self.fConst0;
		self.fConst11 = 2.0 / self.fConst0;
		self.fConst12 = 0.5 / self.fConst0;
		self.fConst13 = 0.16666667 / self.fConst0;
		self.fConst14 = 0.45 * self.fConst0;
		self.fConst15 = 0.95492965 * self.fConst0;
		self.fConst16 = F32::min(2e+04, self.fConst14);
		self.fConst17 = 64.0 * self.fConst16;
		self.fConst18 = 32.0 * self.fConst16;
		self.fConst19 = 16.0 * self.fConst16;
		self.fConst20 = 8.0 * self.fConst16;
		self.fConst21 = 4.0 * self.fConst16;
		self.fConst22 = 2.0 * self.fConst16;
		let mut fConst23: F32 = F32::min(2.2e+04, self.fConst14);
		self.fConst24 = 1.0 / fConst23;
		self.fConst25 = 3.1415927 * (fConst23 / self.fConst0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		OscDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin-osc");
		ui_interface.declare(None, "0.5", "");
		ui_interface.open_horizontal_box("Vintage");
		ui_interface.declare(Some(ParamIndex(0)), "1", "");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(0), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1.5", "");
		ui_interface.open_horizontal_box("Unison");
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.declare(Some(ParamIndex(1)), "style", "menu{'Off':0;'U5':1;'U7':2}");
		ui_interface.add_num_entry("Mode", ParamIndex(1), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.declare(Some(ParamIndex(2)), "style", "detune");
		ui_interface.add_horizontal_slider("Detune", ParamIndex(2), 64.0, 0.0, 127.0, 1.0);
		ui_interface.declare(Some(ParamIndex(3)), "3", "");
		ui_interface.add_check_button("Random Phase", ParamIndex(3));
		ui_interface.declare(Some(ParamIndex(4)), "4", "");
		ui_interface.add_horizontal_slider("Spread", ParamIndex(4), 64.0, 0.0, 127.0, 1.0);
		ui_interface.declare(Some(ParamIndex(5)), "5", "");
		ui_interface.add_check_button("Eco", ParamIndex(5));
		ui_interface.close_box();
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Perform");
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.declare(Some(ParamIndex(6)), "unit", "st");
		ui_interface.add_horizontal_slider("Global Pitch", ParamIndex(6), 0.0, -12.0, 12.0, 0.01);
		ui_interface.declare(Some(ParamIndex(7)), "5", "");
		ui_interface.declare(Some(ParamIndex(7)), "style", "menu{'2 Semitones':0;'Octave':1;'Cutoff 50%':2;'Resonance 50%':3;'Bandpass 50%':4}");
		ui_interface.add_num_entry("Bend Mode", ParamIndex(7), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "13", "");
		ui_interface.open_horizontal_box("Settings");
		ui_interface.declare(Some(ParamIndex(8)), "1", "");
		ui_interface.add_horizontal_slider("Vintage Noise", ParamIndex(8), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(9)), "3", "");
		ui_interface.declare(Some(ParamIndex(9)), "unit", "ms");
		ui_interface.add_horizontal_slider("Ortho Scatter", ParamIndex(9), 1e+02, 0.0, 5e+02, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "14", "");
		ui_interface.open_horizontal_box("Harmonic");
		ui_interface.declare(Some(ParamIndex(10)), "0", "");
		ui_interface.declare(Some(ParamIndex(10)), "style", "menu{'Off':0;'Amp 01':1;'Amp 02':2;'Amp 03':3;'Amp 04':4;'Amp 05':5;'Amp 06':6;'Bass Contour 01':7;'Bass Contour 02':8;'Bass Contour 03':9;'Bell 01':10;'Bell 02':11;'Bell 03':12;'Cream':13;'Dampen':14;'Dirt 01':15;'Dirt 02':16;'Dirt 03':17;'Dirt 04':18;'Dirt 05':19;'Dirt 06':20;'Dist 01':21;'Dist 02':22;'Dungeon':23;'EQ Highs 01':24;'EQ Highs 02':25;'EQ Highs 03':26;'EQ Lows 01':27;'EQ Lows 02':28;'EQ Lows 03':29;'EQ Mids 01':30;'EQ Mids 02':31;'EQ Mids 03':32;'Formant 01':33;'Formant 02':34;'Formant 03':35;'Formant 04':36;'Frozen 01':37;'Frozen 02':38;'Frozen 03':39;'Guitar 01':40;'Guitar 02':41;'Hiss 01':42;'Hiss 02':43;'Inharmonic 01':44;'Inharmonic 02':45;'Inharmonic 03':46;'Inharmonic 04':47;'Inharmonic 05':48;'Mechanic 01':49;'Mechanic 02':50;'Mechanic 03':51;'Mechanic 04':52;'Notch':53;'Nylon':54;'Phase 01':55;'Phase 02':56;'Plastic':57;'Pulse 01':58;'Pulse 02':59;'Pulse 03':60;'Pulse 04':61;'Pulse 05':62;'Pulse 06':63;'Pulse 07':64;'Silk':65;'Space 01':66;'Space 02':67;'Space 03':68;'Space 04':69;'Space 05':70;'Spring':71;'Tones 01':72;'Tones 02':73;'Tones 03':74;'Tones 04':75;'Tones 05':76;'Voice 01':77;'Voice 02':78;'Voice 03':79;'Voice 04':80;'Voice 05':81}");
		ui_interface.add_num_entry("Shape", ParamIndex(10), 0.0, 0.0, 81.0, 1.0);
		ui_interface.declare(Some(ParamIndex(11)), "1", "");
		ui_interface.declare(Some(ParamIndex(11)), "unit", "oct");
		ui_interface.add_horizontal_slider("Shift", ParamIndex(11), 0.0, -4.0, 4.0, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "2", "");
		ui_interface.declare(Some(ParamIndex(12)), "unit", "dB/oct");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(12), 0.0, -6.0, 6.0, 0.01);
		ui_interface.declare(Some(ParamIndex(13)), "3", "");
		ui_interface.add_horizontal_slider("Depth", ParamIndex(13), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(14)), "4", "");
		ui_interface.add_horizontal_slider("Odd Even", ParamIndex(14), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.1", "");
		ui_interface.open_horizontal_box("Dual 1");
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.declare(Some(ParamIndex(15)), "style", "menu{'VA Sweep':0;'Spread':1;'PWM':2;'Sync':3;'Fractal':4;'Window Amp Sync':5;'Metal':6;'Ring Mod':7;'FM':8;'Noise AM':9;'Chaos':10;'Folding':11}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(15), 0.0, 0.0, 11.0, 1.0);
		ui_interface.declare(Some(ParamIndex(16)), "2", "");
		ui_interface.declare(Some(ParamIndex(16)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(16), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(17)), "3", "");
		ui_interface.declare(Some(ParamIndex(17)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(17), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(18)), "4", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(18), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(19)), "5", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(19), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.2", "");
		ui_interface.open_horizontal_box("WT 1");
		ui_interface.declare(Some(ParamIndex(20)), "1", "");
		ui_interface.declare(Some(ParamIndex(20)), "style", "menu{'A8 01':0;'A8 02':1;'A8 03':2;'A8 04':3;'A8 05':4;'A8 06':5;'A8 07':6;'A8 08':7;'A8 09':8;'A8 10':9;'A8 11':10;'A8 12':11;'A8 13':12;'A8 14':13;'A8 15':14;'A8 16':15;'A8 17':16;'A8 18':17;'A8 19':18;'A8 20':19;'A8 21':20;'A8 22':21;'A8 23':22;'A8 24':23;'A8 25':24;'A8 26':25;'A8 27':26;'A8 28':27;'A8 29':28;'A8 30':29;'A8 31':30;'A8 32':31;'M8 01':32;'M8 02':33;'M8 03':34;'M8 04':35;'M8 05':36;'M8 06':37;'M8 07':38;'M8 08':39;'M8 09':40;'M8 10':41;'M8 11':42;'M8 12':43;'M8 13':44;'M8 14':45;'M8 15':46;'M8 16':47;'M8 17':48;'M8 18':49;'M8 19':50;'M8 20':51;'M8 21':52;'M8 22':53;'M8 23':54;'M8 24':55;'M8 25':56;'M8 26':57;'M8 27':58;'M8 28':59;'M8 29':60;'M8 30':61;'M8 31':62;'M8 32':63;'M8 33':64;'M8 34':65;'M8 35':66;'M8 36':67;'M8 37':68;'M8 38':69;'M8 39':70;'M8 40':71;'M8 41':72;'M8 42':73;'M8 43':74;'M8 44':75;'M8 45':76;'M8 46':77;'M8 47':78;'M8 48':79;'M8 49':80;'M8 50':81;'M8 51':82;'M8 52':83;'M16 01':84;'M16 02':85;'M16 03':86;'M16 04':87;'M16 05':88;'M16 06':89;'M16 07':90;'M16 08':91;'M16 09':92;'M16 10':93;'M16 11':94;'M16 12':95;'M16 13':96;'M16 14':97;'M16 15':98;'M16 16':99;'M16 17':100;'M16 18':101;'M16 19':102;'M16 20':103;'M16 21':104;'M16 22':105;'M16 23':106;'M16 24':107;'M16 25':108;'M16 26':109;'M16 27':110;'M16 28':111;'M16 29':112;'M16 30':113;'M16 31':114;'M16 32':115;'M16 33':116;'M16 34':117;'M16 35':118;'M16 36':119;'M16 37':120;'M16 38':121;'M16 39':122;'M16 40':123;'M16 41':124;'M16 42':125;'M16 43':126;'M16 44':127;'M16 45':128;'M16 46':129;'M16 47':130;'Sine':131;'Triangle':132;'Saw':133;'Square':134}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(20), 0.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(21)), "2", "");
		ui_interface.declare(Some(ParamIndex(21)), "style", "menu{'A8 01':0;'A8 02':1;'A8 03':2;'A8 04':3;'A8 05':4;'A8 06':5;'A8 07':6;'A8 08':7;'A8 09':8;'A8 10':9;'A8 11':10;'A8 12':11;'A8 13':12;'A8 14':13;'A8 15':14;'A8 16':15;'A8 17':16;'A8 18':17;'A8 19':18;'A8 20':19;'A8 21':20;'A8 22':21;'A8 23':22;'A8 24':23;'A8 25':24;'A8 26':25;'A8 27':26;'A8 28':27;'A8 29':28;'A8 30':29;'A8 31':30;'A8 32':31;'M8 01':32;'M8 02':33;'M8 03':34;'M8 04':35;'M8 05':36;'M8 06':37;'M8 07':38;'M8 08':39;'M8 09':40;'M8 10':41;'M8 11':42;'M8 12':43;'M8 13':44;'M8 14':45;'M8 15':46;'M8 16':47;'M8 17':48;'M8 18':49;'M8 19':50;'M8 20':51;'M8 21':52;'M8 22':53;'M8 23':54;'M8 24':55;'M8 25':56;'M8 26':57;'M8 27':58;'M8 28':59;'M8 29':60;'M8 30':61;'M8 31':62;'M8 32':63;'M8 33':64;'M8 34':65;'M8 35':66;'M8 36':67;'M8 37':68;'M8 38':69;'M8 39':70;'M8 40':71;'M8 41':72;'M8 42':73;'M8 43':74;'M8 44':75;'M8 45':76;'M8 46':77;'M8 47':78;'M8 48':79;'M8 49':80;'M8 50':81;'M8 51':82;'M8 52':83;'M16 01':84;'M16 02':85;'M16 03':86;'M16 04':87;'M16 05':88;'M16 06':89;'M16 07':90;'M16 08':91;'M16 09':92;'M16 10':93;'M16 11':94;'M16 12':95;'M16 13':96;'M16 14':97;'M16 15':98;'M16 16':99;'M16 17':100;'M16 18':101;'M16 19':102;'M16 20':103;'M16 21':104;'M16 22':105;'M16 23':106;'M16 24':107;'M16 25':108;'M16 26':109;'M16 27':110;'M16 28':111;'M16 29':112;'M16 30':113;'M16 31':114;'M16 32':115;'M16 33':116;'M16 34':117;'M16 35':118;'M16 36':119;'M16 37':120;'M16 38':121;'M16 39':122;'M16 40':123;'M16 41':124;'M16 42':125;'M16 43':126;'M16 44':127;'M16 45':128;'M16 46':129;'M16 47':130;'Sine':131;'Triangle':132;'Saw':133;'Square':134}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(21), 11.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(22)), "3", "");
		ui_interface.declare(Some(ParamIndex(22)), "style", "menu{'A8 01':0;'A8 02':1;'A8 03':2;'A8 04':3;'A8 05':4;'A8 06':5;'A8 07':6;'A8 08':7;'A8 09':8;'A8 10':9;'A8 11':10;'A8 12':11;'A8 13':12;'A8 14':13;'A8 15':14;'A8 16':15;'A8 17':16;'A8 18':17;'A8 19':18;'A8 20':19;'A8 21':20;'A8 22':21;'A8 23':22;'A8 24':23;'A8 25':24;'A8 26':25;'A8 27':26;'A8 28':27;'A8 29':28;'A8 30':29;'A8 31':30;'A8 32':31;'M8 01':32;'M8 02':33;'M8 03':34;'M8 04':35;'M8 05':36;'M8 06':37;'M8 07':38;'M8 08':39;'M8 09':40;'M8 10':41;'M8 11':42;'M8 12':43;'M8 13':44;'M8 14':45;'M8 15':46;'M8 16':47;'M8 17':48;'M8 18':49;'M8 19':50;'M8 20':51;'M8 21':52;'M8 22':53;'M8 23':54;'M8 24':55;'M8 25':56;'M8 26':57;'M8 27':58;'M8 28':59;'M8 29':60;'M8 30':61;'M8 31':62;'M8 32':63;'M8 33':64;'M8 34':65;'M8 35':66;'M8 36':67;'M8 37':68;'M8 38':69;'M8 39':70;'M8 40':71;'M8 41':72;'M8 42':73;'M8 43':74;'M8 44':75;'M8 45':76;'M8 46':77;'M8 47':78;'M8 48':79;'M8 49':80;'M8 50':81;'M8 51':82;'M8 52':83;'M16 01':84;'M16 02':85;'M16 03':86;'M16 04':87;'M16 05':88;'M16 06':89;'M16 07':90;'M16 08':91;'M16 09':92;'M16 10':93;'M16 11':94;'M16 12':95;'M16 13':96;'M16 14':97;'M16 15':98;'M16 16':99;'M16 17':100;'M16 18':101;'M16 19':102;'M16 20':103;'M16 21':104;'M16 22':105;'M16 23':106;'M16 24':107;'M16 25':108;'M16 26':109;'M16 27':110;'M16 28':111;'M16 29':112;'M16 30':113;'M16 31':114;'M16 32':115;'M16 33':116;'M16 34':117;'M16 35':118;'M16 36':119;'M16 37':120;'M16 38':121;'M16 39':122;'M16 40':123;'M16 41':124;'M16 42':125;'M16 43':126;'M16 44':127;'M16 45':128;'M16 46':129;'M16 47':130;'Sine':131;'Triangle':132;'Saw':133;'Square':134}");
		ui_interface.add_num_entry("Wave 3", ParamIndex(22), 21.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(23)), "4", "");
		ui_interface.declare(Some(ParamIndex(23)), "style", "menu{'A8 01':0;'A8 02':1;'A8 03':2;'A8 04':3;'A8 05':4;'A8 06':5;'A8 07':6;'A8 08':7;'A8 09':8;'A8 10':9;'A8 11':10;'A8 12':11;'A8 13':12;'A8 14':13;'A8 15':14;'A8 16':15;'A8 17':16;'A8 18':17;'A8 19':18;'A8 20':19;'A8 21':20;'A8 22':21;'A8 23':22;'A8 24':23;'A8 25':24;'A8 26':25;'A8 27':26;'A8 28':27;'A8 29':28;'A8 30':29;'A8 31':30;'A8 32':31;'M8 01':32;'M8 02':33;'M8 03':34;'M8 04':35;'M8 05':36;'M8 06':37;'M8 07':38;'M8 08':39;'M8 09':40;'M8 10':41;'M8 11':42;'M8 12':43;'M8 13':44;'M8 14':45;'M8 15':46;'M8 16':47;'M8 17':48;'M8 18':49;'M8 19':50;'M8 20':51;'M8 21':52;'M8 22':53;'M8 23':54;'M8 24':55;'M8 25':56;'M8 26':57;'M8 27':58;'M8 28':59;'M8 29':60;'M8 30':61;'M8 31':62;'M8 32':63;'M8 33':64;'M8 34':65;'M8 35':66;'M8 36':67;'M8 37':68;'M8 38':69;'M8 39':70;'M8 40':71;'M8 41':72;'M8 42':73;'M8 43':74;'M8 44':75;'M8 45':76;'M8 46':77;'M8 47':78;'M8 48':79;'M8 49':80;'M8 50':81;'M8 51':82;'M8 52':83;'M16 01':84;'M16 02':85;'M16 03':86;'M16 04':87;'M16 05':88;'M16 06':89;'M16 07':90;'M16 08':91;'M16 09':92;'M16 10':93;'M16 11':94;'M16 12':95;'M16 13':96;'M16 14':97;'M16 15':98;'M16 16':99;'M16 17':100;'M16 18':101;'M16 19':102;'M16 20':103;'M16 21':104;'M16 22':105;'M16 23':106;'M16 24':107;'M16 25':108;'M16 26':109;'M16 27':110;'M16 28':111;'M16 29':112;'M16 30':113;'M16 31':114;'M16 32':115;'M16 33':116;'M16 34':117;'M16 35':118;'M16 36':119;'M16 37':120;'M16 38':121;'M16 39':122;'M16 40':123;'M16 41':124;'M16 42':125;'M16 43':126;'M16 44':127;'M16 45':128;'M16 46':129;'M16 47':130;'Sine':131;'Triangle':132;'Saw':133;'Square':134}");
		ui_interface.add_num_entry("Wave 4", ParamIndex(23), 32.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(24)), "5", "");
		ui_interface.add_horizontal_slider("Position", ParamIndex(24), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(25)), "6", "");
		ui_interface.declare(Some(ParamIndex(25)), "style", "menu{'Off':0;'Bits 4':1;'SR 11k':2;'SR 8k':3;'SR 4k':4;'Simplify 6k':5;'Simplify 1.5k':6;'Simplify 800':7;'Rectify':8;'Tri Fold':9;'Sine Fold':10;'Squish':11;'Ring':12;'Mirror':13;'Sign Swap':14}");
		ui_interface.add_num_entry("OscMod", ParamIndex(25), 0.0, 0.0, 14.0, 1.0);
		ui_interface.declare(Some(ParamIndex(26)), "7", "");
		ui_interface.add_horizontal_slider("OscMod Depth", ParamIndex(26), 0.5, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.3", "");
		ui_interface.open_horizontal_box("Ray 1");
		ui_interface.declare(Some(ParamIndex(27)), "10", "");
		ui_interface.declare(Some(ParamIndex(27)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(27)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Orbit Rate", ParamIndex(27), 0.3, 0.02, 1e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(28)), "11", "");
		ui_interface.declare(Some(ParamIndex(28)), "unit", "cents");
		ui_interface.add_horizontal_slider("Fine", ParamIndex(28), 0.0, -1e+02, 1e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(29)), "1", "");
		ui_interface.declare(Some(ParamIndex(29)), "style", "menu{'Earth':0;'Water':1;'Air':2;'Fire':3;'Wood':4;'Metal':5;'Chaos':6}");
		ui_interface.add_num_entry("Element", ParamIndex(29), 0.0, 0.0, 6.0, 1.0);
		ui_interface.declare(Some(ParamIndex(30)), "2", "");
		ui_interface.add_num_entry("Chaos Seed", ParamIndex(30), 4821.0, 1.0, 99999.0, 1.0);
		ui_interface.declare(Some(ParamIndex(31)), "3", "");
		ui_interface.add_check_button("Free", ParamIndex(31));
		ui_interface.declare(Some(ParamIndex(32)), "4", "");
		ui_interface.add_horizontal_slider("Squish", ParamIndex(32), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(33)), "5", "");
		ui_interface.add_horizontal_slider("Skew", ParamIndex(33), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(34)), "6", "");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(34), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(35)), "7", "");
		ui_interface.add_horizontal_slider("Center", ParamIndex(35), 0.0, -1.3, 1.3, 0.001);
		ui_interface.declare(Some(ParamIndex(36)), "8", "");
		ui_interface.add_horizontal_slider("Ray", ParamIndex(36), 0.0, 0.0, 1.5, 0.001);
		ui_interface.declare(Some(ParamIndex(37)), "9", "");
		ui_interface.add_horizontal_slider("Orbit", ParamIndex(37), 0.0, 0.0, 0.8, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.4", "");
		ui_interface.open_horizontal_box("Ortho 1");
		ui_interface.declare(Some(ParamIndex(38)), "1", "");
		ui_interface.declare(Some(ParamIndex(38)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Top", ParamIndex(38), 34.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(39)), "2", "");
		ui_interface.declare(Some(ParamIndex(39)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Bottom", ParamIndex(39), 39.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(40)), "3", "");
		ui_interface.add_horizontal_slider("Top Position", ParamIndex(40), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(41)), "4", "");
		ui_interface.add_horizontal_slider("Bottom Position", ParamIndex(41), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(42)), "5", "");
		ui_interface.declare(Some(ParamIndex(42)), "unit", "%");
		ui_interface.add_horizontal_slider("Top Speed", ParamIndex(42), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(43)), "6", "");
		ui_interface.declare(Some(ParamIndex(43)), "unit", "%");
		ui_interface.add_horizontal_slider("Bottom Speed", ParamIndex(43), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(44)), "7", "");
		ui_interface.add_horizontal_slider("Mix", ParamIndex(44), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(45)), "8", "");
		ui_interface.declare(Some(ParamIndex(45)), "style", "pitch");
		ui_interface.add_horizontal_slider("Top Pitch", ParamIndex(45), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(46)), "9", "");
		ui_interface.declare(Some(ParamIndex(46)), "style", "pitch");
		ui_interface.add_horizontal_slider("Bottom Pitch", ParamIndex(46), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.5", "");
		ui_interface.open_horizontal_box("Windows 1");
		ui_interface.add_horizontal_slider("Int 1 Len", ParamIndex(47), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 1 On", ParamIndex(48));
		ui_interface.declare(Some(ParamIndex(49)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 1 Phase", ParamIndex(49), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(50)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 1 Pitch", ParamIndex(50), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(51)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 1 Shape", ParamIndex(51), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 1 X", ParamIndex(52), 0.0, 0.0, 0.99, 0.001);
		ui_interface.add_horizontal_slider("Int 2 Len", ParamIndex(53), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 2 On", ParamIndex(54));
		ui_interface.declare(Some(ParamIndex(55)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 2 Phase", ParamIndex(55), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(56)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 2 Pitch", ParamIndex(56), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(57)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 2 Shape", ParamIndex(57), 4.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 2 X", ParamIndex(58), 0.33333334, 0.0, 0.99, 0.001);
		ui_interface.add_horizontal_slider("Int 3 Len", ParamIndex(59), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 3 On", ParamIndex(60));
		ui_interface.declare(Some(ParamIndex(61)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 3 Phase", ParamIndex(61), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(62)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 3 Pitch", ParamIndex(62), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(63)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 3 Shape", ParamIndex(63), 1.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 3 X", ParamIndex(64), 0.6666667, 0.0, 0.99, 0.001);
		ui_interface.declare(Some(ParamIndex(65)), "1", "");
		ui_interface.declare(Some(ParamIndex(65)), "style", "menu{'Sine':0;'Cosine':1;'Triangle':2;'Saw':3;'Ramp':4;'Square':5}");
		ui_interface.add_num_entry("Base", ParamIndex(65), 3.0, 0.0, 5.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("Osc 1");
		ui_interface.declare(Some(ParamIndex(66)), "0", "");
		ui_interface.declare(Some(ParamIndex(66)), "style", "menu{'Dual':0;'Wavetable':1;'Ray':2;'Orthogonal':3;'Windows':4}");
		ui_interface.add_num_entry("Mode", ParamIndex(66), 0.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(67)), "1", "");
		ui_interface.declare(Some(ParamIndex(67)), "style", "pitch");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(67), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.1", "");
		ui_interface.open_horizontal_box("Dual 2");
		ui_interface.declare(Some(ParamIndex(68)), "1", "");
		ui_interface.declare(Some(ParamIndex(68)), "style", "menu{'VA Sweep':0;'Spread':1;'PWM':2;'Sync':3;'Fractal':4;'Window Amp Sync':5;'Metal':6;'Ring Mod':7;'FM':8;'Noise AM':9;'Chaos':10;'Folding':11}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(68), 0.0, 0.0, 11.0, 1.0);
		ui_interface.declare(Some(ParamIndex(69)), "2", "");
		ui_interface.declare(Some(ParamIndex(69)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(69), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(70)), "3", "");
		ui_interface.declare(Some(ParamIndex(70)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(70), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(71)), "4", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(71), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(72)), "5", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(72), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.2", "");
		ui_interface.open_horizontal_box("WT 2");
		ui_interface.declare(Some(ParamIndex(73)), "1", "");
		ui_interface.declare(Some(ParamIndex(73)), "style", "menu{'A8 01':0;'A8 02':1;'A8 03':2;'A8 04':3;'A8 05':4;'A8 06':5;'A8 07':6;'A8 08':7;'A8 09':8;'A8 10':9;'A8 11':10;'A8 12':11;'A8 13':12;'A8 14':13;'A8 15':14;'A8 16':15;'A8 17':16;'A8 18':17;'A8 19':18;'A8 20':19;'A8 21':20;'A8 22':21;'A8 23':22;'A8 24':23;'A8 25':24;'A8 26':25;'A8 27':26;'A8 28':27;'A8 29':28;'A8 30':29;'A8 31':30;'A8 32':31;'M8 01':32;'M8 02':33;'M8 03':34;'M8 04':35;'M8 05':36;'M8 06':37;'M8 07':38;'M8 08':39;'M8 09':40;'M8 10':41;'M8 11':42;'M8 12':43;'M8 13':44;'M8 14':45;'M8 15':46;'M8 16':47;'M8 17':48;'M8 18':49;'M8 19':50;'M8 20':51;'M8 21':52;'M8 22':53;'M8 23':54;'M8 24':55;'M8 25':56;'M8 26':57;'M8 27':58;'M8 28':59;'M8 29':60;'M8 30':61;'M8 31':62;'M8 32':63;'M8 33':64;'M8 34':65;'M8 35':66;'M8 36':67;'M8 37':68;'M8 38':69;'M8 39':70;'M8 40':71;'M8 41':72;'M8 42':73;'M8 43':74;'M8 44':75;'M8 45':76;'M8 46':77;'M8 47':78;'M8 48':79;'M8 49':80;'M8 50':81;'M8 51':82;'M8 52':83;'M16 01':84;'M16 02':85;'M16 03':86;'M16 04':87;'M16 05':88;'M16 06':89;'M16 07':90;'M16 08':91;'M16 09':92;'M16 10':93;'M16 11':94;'M16 12':95;'M16 13':96;'M16 14':97;'M16 15':98;'M16 16':99;'M16 17':100;'M16 18':101;'M16 19':102;'M16 20':103;'M16 21':104;'M16 22':105;'M16 23':106;'M16 24':107;'M16 25':108;'M16 26':109;'M16 27':110;'M16 28':111;'M16 29':112;'M16 30':113;'M16 31':114;'M16 32':115;'M16 33':116;'M16 34':117;'M16 35':118;'M16 36':119;'M16 37':120;'M16 38':121;'M16 39':122;'M16 40':123;'M16 41':124;'M16 42':125;'M16 43':126;'M16 44':127;'M16 45':128;'M16 46':129;'M16 47':130;'Sine':131;'Triangle':132;'Saw':133;'Square':134}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(73), 0.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(74)), "2", "");
		ui_interface.declare(Some(ParamIndex(74)), "style", "menu{'A8 01':0;'A8 02':1;'A8 03':2;'A8 04':3;'A8 05':4;'A8 06':5;'A8 07':6;'A8 08':7;'A8 09':8;'A8 10':9;'A8 11':10;'A8 12':11;'A8 13':12;'A8 14':13;'A8 15':14;'A8 16':15;'A8 17':16;'A8 18':17;'A8 19':18;'A8 20':19;'A8 21':20;'A8 22':21;'A8 23':22;'A8 24':23;'A8 25':24;'A8 26':25;'A8 27':26;'A8 28':27;'A8 29':28;'A8 30':29;'A8 31':30;'A8 32':31;'M8 01':32;'M8 02':33;'M8 03':34;'M8 04':35;'M8 05':36;'M8 06':37;'M8 07':38;'M8 08':39;'M8 09':40;'M8 10':41;'M8 11':42;'M8 12':43;'M8 13':44;'M8 14':45;'M8 15':46;'M8 16':47;'M8 17':48;'M8 18':49;'M8 19':50;'M8 20':51;'M8 21':52;'M8 22':53;'M8 23':54;'M8 24':55;'M8 25':56;'M8 26':57;'M8 27':58;'M8 28':59;'M8 29':60;'M8 30':61;'M8 31':62;'M8 32':63;'M8 33':64;'M8 34':65;'M8 35':66;'M8 36':67;'M8 37':68;'M8 38':69;'M8 39':70;'M8 40':71;'M8 41':72;'M8 42':73;'M8 43':74;'M8 44':75;'M8 45':76;'M8 46':77;'M8 47':78;'M8 48':79;'M8 49':80;'M8 50':81;'M8 51':82;'M8 52':83;'M16 01':84;'M16 02':85;'M16 03':86;'M16 04':87;'M16 05':88;'M16 06':89;'M16 07':90;'M16 08':91;'M16 09':92;'M16 10':93;'M16 11':94;'M16 12':95;'M16 13':96;'M16 14':97;'M16 15':98;'M16 16':99;'M16 17':100;'M16 18':101;'M16 19':102;'M16 20':103;'M16 21':104;'M16 22':105;'M16 23':106;'M16 24':107;'M16 25':108;'M16 26':109;'M16 27':110;'M16 28':111;'M16 29':112;'M16 30':113;'M16 31':114;'M16 32':115;'M16 33':116;'M16 34':117;'M16 35':118;'M16 36':119;'M16 37':120;'M16 38':121;'M16 39':122;'M16 40':123;'M16 41':124;'M16 42':125;'M16 43':126;'M16 44':127;'M16 45':128;'M16 46':129;'M16 47':130;'Sine':131;'Triangle':132;'Saw':133;'Square':134}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(74), 11.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(75)), "3", "");
		ui_interface.declare(Some(ParamIndex(75)), "style", "menu{'A8 01':0;'A8 02':1;'A8 03':2;'A8 04':3;'A8 05':4;'A8 06':5;'A8 07':6;'A8 08':7;'A8 09':8;'A8 10':9;'A8 11':10;'A8 12':11;'A8 13':12;'A8 14':13;'A8 15':14;'A8 16':15;'A8 17':16;'A8 18':17;'A8 19':18;'A8 20':19;'A8 21':20;'A8 22':21;'A8 23':22;'A8 24':23;'A8 25':24;'A8 26':25;'A8 27':26;'A8 28':27;'A8 29':28;'A8 30':29;'A8 31':30;'A8 32':31;'M8 01':32;'M8 02':33;'M8 03':34;'M8 04':35;'M8 05':36;'M8 06':37;'M8 07':38;'M8 08':39;'M8 09':40;'M8 10':41;'M8 11':42;'M8 12':43;'M8 13':44;'M8 14':45;'M8 15':46;'M8 16':47;'M8 17':48;'M8 18':49;'M8 19':50;'M8 20':51;'M8 21':52;'M8 22':53;'M8 23':54;'M8 24':55;'M8 25':56;'M8 26':57;'M8 27':58;'M8 28':59;'M8 29':60;'M8 30':61;'M8 31':62;'M8 32':63;'M8 33':64;'M8 34':65;'M8 35':66;'M8 36':67;'M8 37':68;'M8 38':69;'M8 39':70;'M8 40':71;'M8 41':72;'M8 42':73;'M8 43':74;'M8 44':75;'M8 45':76;'M8 46':77;'M8 47':78;'M8 48':79;'M8 49':80;'M8 50':81;'M8 51':82;'M8 52':83;'M16 01':84;'M16 02':85;'M16 03':86;'M16 04':87;'M16 05':88;'M16 06':89;'M16 07':90;'M16 08':91;'M16 09':92;'M16 10':93;'M16 11':94;'M16 12':95;'M16 13':96;'M16 14':97;'M16 15':98;'M16 16':99;'M16 17':100;'M16 18':101;'M16 19':102;'M16 20':103;'M16 21':104;'M16 22':105;'M16 23':106;'M16 24':107;'M16 25':108;'M16 26':109;'M16 27':110;'M16 28':111;'M16 29':112;'M16 30':113;'M16 31':114;'M16 32':115;'M16 33':116;'M16 34':117;'M16 35':118;'M16 36':119;'M16 37':120;'M16 38':121;'M16 39':122;'M16 40':123;'M16 41':124;'M16 42':125;'M16 43':126;'M16 44':127;'M16 45':128;'M16 46':129;'M16 47':130;'Sine':131;'Triangle':132;'Saw':133;'Square':134}");
		ui_interface.add_num_entry("Wave 3", ParamIndex(75), 21.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(76)), "4", "");
		ui_interface.declare(Some(ParamIndex(76)), "style", "menu{'A8 01':0;'A8 02':1;'A8 03':2;'A8 04':3;'A8 05':4;'A8 06':5;'A8 07':6;'A8 08':7;'A8 09':8;'A8 10':9;'A8 11':10;'A8 12':11;'A8 13':12;'A8 14':13;'A8 15':14;'A8 16':15;'A8 17':16;'A8 18':17;'A8 19':18;'A8 20':19;'A8 21':20;'A8 22':21;'A8 23':22;'A8 24':23;'A8 25':24;'A8 26':25;'A8 27':26;'A8 28':27;'A8 29':28;'A8 30':29;'A8 31':30;'A8 32':31;'M8 01':32;'M8 02':33;'M8 03':34;'M8 04':35;'M8 05':36;'M8 06':37;'M8 07':38;'M8 08':39;'M8 09':40;'M8 10':41;'M8 11':42;'M8 12':43;'M8 13':44;'M8 14':45;'M8 15':46;'M8 16':47;'M8 17':48;'M8 18':49;'M8 19':50;'M8 20':51;'M8 21':52;'M8 22':53;'M8 23':54;'M8 24':55;'M8 25':56;'M8 26':57;'M8 27':58;'M8 28':59;'M8 29':60;'M8 30':61;'M8 31':62;'M8 32':63;'M8 33':64;'M8 34':65;'M8 35':66;'M8 36':67;'M8 37':68;'M8 38':69;'M8 39':70;'M8 40':71;'M8 41':72;'M8 42':73;'M8 43':74;'M8 44':75;'M8 45':76;'M8 46':77;'M8 47':78;'M8 48':79;'M8 49':80;'M8 50':81;'M8 51':82;'M8 52':83;'M16 01':84;'M16 02':85;'M16 03':86;'M16 04':87;'M16 05':88;'M16 06':89;'M16 07':90;'M16 08':91;'M16 09':92;'M16 10':93;'M16 11':94;'M16 12':95;'M16 13':96;'M16 14':97;'M16 15':98;'M16 16':99;'M16 17':100;'M16 18':101;'M16 19':102;'M16 20':103;'M16 21':104;'M16 22':105;'M16 23':106;'M16 24':107;'M16 25':108;'M16 26':109;'M16 27':110;'M16 28':111;'M16 29':112;'M16 30':113;'M16 31':114;'M16 32':115;'M16 33':116;'M16 34':117;'M16 35':118;'M16 36':119;'M16 37':120;'M16 38':121;'M16 39':122;'M16 40':123;'M16 41':124;'M16 42':125;'M16 43':126;'M16 44':127;'M16 45':128;'M16 46':129;'M16 47':130;'Sine':131;'Triangle':132;'Saw':133;'Square':134}");
		ui_interface.add_num_entry("Wave 4", ParamIndex(76), 32.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(77)), "5", "");
		ui_interface.add_horizontal_slider("Position", ParamIndex(77), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(78)), "6", "");
		ui_interface.declare(Some(ParamIndex(78)), "style", "menu{'Off':0;'Bits 4':1;'SR 11k':2;'SR 8k':3;'SR 4k':4;'Simplify 6k':5;'Simplify 1.5k':6;'Simplify 800':7;'Rectify':8;'Tri Fold':9;'Sine Fold':10;'Squish':11;'Ring':12;'Mirror':13;'Sign Swap':14}");
		ui_interface.add_num_entry("OscMod", ParamIndex(78), 0.0, 0.0, 14.0, 1.0);
		ui_interface.declare(Some(ParamIndex(79)), "7", "");
		ui_interface.add_horizontal_slider("OscMod Depth", ParamIndex(79), 0.5, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.3", "");
		ui_interface.open_horizontal_box("Ray 2");
		ui_interface.declare(Some(ParamIndex(80)), "10", "");
		ui_interface.declare(Some(ParamIndex(80)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(80)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Orbit Rate", ParamIndex(80), 0.3, 0.02, 1e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(81)), "11", "");
		ui_interface.declare(Some(ParamIndex(81)), "unit", "cents");
		ui_interface.add_horizontal_slider("Fine", ParamIndex(81), 0.0, -1e+02, 1e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(82)), "1", "");
		ui_interface.declare(Some(ParamIndex(82)), "style", "menu{'Earth':0;'Water':1;'Air':2;'Fire':3;'Wood':4;'Metal':5;'Chaos':6}");
		ui_interface.add_num_entry("Element", ParamIndex(82), 3.0, 0.0, 6.0, 1.0);
		ui_interface.declare(Some(ParamIndex(83)), "2", "");
		ui_interface.add_num_entry("Chaos Seed", ParamIndex(83), 6821.0, 1.0, 99999.0, 1.0);
		ui_interface.declare(Some(ParamIndex(84)), "3", "");
		ui_interface.add_check_button("Free", ParamIndex(84));
		ui_interface.declare(Some(ParamIndex(85)), "4", "");
		ui_interface.add_horizontal_slider("Squish", ParamIndex(85), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(86)), "5", "");
		ui_interface.add_horizontal_slider("Skew", ParamIndex(86), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(87)), "6", "");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(87), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(88)), "7", "");
		ui_interface.add_horizontal_slider("Center", ParamIndex(88), 0.0, -1.3, 1.3, 0.001);
		ui_interface.declare(Some(ParamIndex(89)), "8", "");
		ui_interface.add_horizontal_slider("Ray", ParamIndex(89), 0.0, 0.0, 1.5, 0.001);
		ui_interface.declare(Some(ParamIndex(90)), "9", "");
		ui_interface.add_horizontal_slider("Orbit", ParamIndex(90), 0.0, 0.0, 0.8, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.4", "");
		ui_interface.open_horizontal_box("Ortho 2");
		ui_interface.declare(Some(ParamIndex(91)), "1", "");
		ui_interface.declare(Some(ParamIndex(91)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Top", ParamIndex(91), 52.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(92)), "2", "");
		ui_interface.declare(Some(ParamIndex(92)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Bottom", ParamIndex(92), 31.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(93)), "3", "");
		ui_interface.add_horizontal_slider("Top Position", ParamIndex(93), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(94)), "4", "");
		ui_interface.add_horizontal_slider("Bottom Position", ParamIndex(94), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(95)), "5", "");
		ui_interface.declare(Some(ParamIndex(95)), "unit", "%");
		ui_interface.add_horizontal_slider("Top Speed", ParamIndex(95), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(96)), "6", "");
		ui_interface.declare(Some(ParamIndex(96)), "unit", "%");
		ui_interface.add_horizontal_slider("Bottom Speed", ParamIndex(96), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(97)), "7", "");
		ui_interface.add_horizontal_slider("Mix", ParamIndex(97), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(98)), "8", "");
		ui_interface.declare(Some(ParamIndex(98)), "style", "pitch");
		ui_interface.add_horizontal_slider("Top Pitch", ParamIndex(98), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(99)), "9", "");
		ui_interface.declare(Some(ParamIndex(99)), "style", "pitch");
		ui_interface.add_horizontal_slider("Bottom Pitch", ParamIndex(99), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.5", "");
		ui_interface.open_horizontal_box("Windows 2");
		ui_interface.add_horizontal_slider("Int 1 Len", ParamIndex(100), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 1 On", ParamIndex(101));
		ui_interface.declare(Some(ParamIndex(102)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 1 Phase", ParamIndex(102), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(103)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 1 Pitch", ParamIndex(103), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(104)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 1 Shape", ParamIndex(104), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 1 X", ParamIndex(105), 0.0, 0.0, 0.99, 0.001);
		ui_interface.add_horizontal_slider("Int 2 Len", ParamIndex(106), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 2 On", ParamIndex(107));
		ui_interface.declare(Some(ParamIndex(108)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 2 Phase", ParamIndex(108), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(109)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 2 Pitch", ParamIndex(109), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(110)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 2 Shape", ParamIndex(110), 4.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 2 X", ParamIndex(111), 0.33333334, 0.0, 0.99, 0.001);
		ui_interface.add_horizontal_slider("Int 3 Len", ParamIndex(112), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 3 On", ParamIndex(113));
		ui_interface.declare(Some(ParamIndex(114)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 3 Phase", ParamIndex(114), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(115)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 3 Pitch", ParamIndex(115), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(116)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 3 Shape", ParamIndex(116), 1.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 3 X", ParamIndex(117), 0.6666667, 0.0, 0.99, 0.001);
		ui_interface.declare(Some(ParamIndex(118)), "1", "");
		ui_interface.declare(Some(ParamIndex(118)), "style", "menu{'Sine':0;'Cosine':1;'Triangle':2;'Saw':3;'Ramp':4;'Square':5}");
		ui_interface.add_num_entry("Base", ParamIndex(118), 3.0, 0.0, 5.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("Osc 2");
		ui_interface.declare(Some(ParamIndex(119)), "0", "");
		ui_interface.declare(Some(ParamIndex(119)), "style", "menu{'Dual':0;'Wavetable':1;'Ray':2;'Orthogonal':3;'Windows':4}");
		ui_interface.add_num_entry("Mode", ParamIndex(119), 1.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(120)), "1", "");
		ui_interface.declare(Some(ParamIndex(120)), "style", "pitch");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(120), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(121)), "1", "");
		ui_interface.add_vertical_slider("Osc 1", ParamIndex(121), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(122)), "2", "");
		ui_interface.add_vertical_slider("Osc 2", ParamIndex(122), 0.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(123)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(123)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(123), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("copy", ParamIndex(124), 3.0, 0.0, 6.0, 1.0);
		ui_interface.add_horizontal_slider("freq", ParamIndex(125), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_button("gate", ParamIndex(126));
		ui_interface.add_horizontal_slider("glide", ParamIndex(127), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("ramp", ParamIndex(128), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("ratio", ParamIndex(129), 1.0, 0.5, 2.0, 1e-06);
		ui_interface.add_horizontal_slider("seed", ParamIndex(130), 0.0, 0.0, 1.0, 1e-06);
		ui_interface.add_horizontal_slider("slot", ParamIndex(131), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			126 => Some(self.fButton0),
			3 => Some(self.fCheckbox0),
			5 => Some(self.fCheckbox1),
			113 => Some(self.fCheckbox2),
			107 => Some(self.fCheckbox3),
			101 => Some(self.fCheckbox4),
			60 => Some(self.fCheckbox5),
			54 => Some(self.fCheckbox6),
			48 => Some(self.fCheckbox7),
			84 => Some(self.fCheckbox8),
			31 => Some(self.fCheckbox9),
			66 => Some(self.fEntry0),
			15 => Some(self.fEntry1),
			22 => Some(self.fEntry10),
			23 => Some(self.fEntry11),
			20 => Some(self.fEntry12),
			70 => Some(self.fEntry13),
			69 => Some(self.fEntry14),
			74 => Some(self.fEntry15),
			75 => Some(self.fEntry16),
			76 => Some(self.fEntry17),
			73 => Some(self.fEntry18),
			1 => Some(self.fEntry19),
			25 => Some(self.fEntry2),
			10 => Some(self.fEntry20),
			116 => Some(self.fEntry21),
			110 => Some(self.fEntry22),
			104 => Some(self.fEntry23),
			118 => Some(self.fEntry24),
			63 => Some(self.fEntry25),
			57 => Some(self.fEntry26),
			51 => Some(self.fEntry27),
			65 => Some(self.fEntry28),
			92 => Some(self.fEntry29),
			78 => Some(self.fEntry3),
			91 => Some(self.fEntry30),
			39 => Some(self.fEntry31),
			38 => Some(self.fEntry32),
			83 => Some(self.fEntry33),
			82 => Some(self.fEntry34),
			30 => Some(self.fEntry35),
			29 => Some(self.fEntry36),
			119 => Some(self.fEntry4),
			7 => Some(self.fEntry5),
			68 => Some(self.fEntry6),
			17 => Some(self.fEntry7),
			16 => Some(self.fEntry8),
			21 => Some(self.fEntry9),
			67 => Some(self.fHslider0),
			128 => Some(self.fHslider1),
			124 => Some(self.fHslider10),
			130 => Some(self.fHslider11),
			19 => Some(self.fHslider12),
			18 => Some(self.fHslider13),
			24 => Some(self.fHslider14),
			26 => Some(self.fHslider15),
			120 => Some(self.fHslider16),
			72 => Some(self.fHslider17),
			71 => Some(self.fHslider18),
			77 => Some(self.fHslider19),
			0 => Some(self.fHslider2),
			79 => Some(self.fHslider20),
			4 => Some(self.fHslider21),
			2 => Some(self.fHslider22),
			14 => Some(self.fHslider23),
			13 => Some(self.fHslider24),
			12 => Some(self.fHslider25),
			11 => Some(self.fHslider26),
			9 => Some(self.fHslider27),
			115 => Some(self.fHslider28),
			112 => Some(self.fHslider29),
			131 => Some(self.fHslider3),
			114 => Some(self.fHslider30),
			117 => Some(self.fHslider31),
			109 => Some(self.fHslider32),
			106 => Some(self.fHslider33),
			108 => Some(self.fHslider34),
			111 => Some(self.fHslider35),
			103 => Some(self.fHslider36),
			100 => Some(self.fHslider37),
			102 => Some(self.fHslider38),
			105 => Some(self.fHslider39),
			8 => Some(self.fHslider4),
			62 => Some(self.fHslider40),
			59 => Some(self.fHslider41),
			61 => Some(self.fHslider42),
			64 => Some(self.fHslider43),
			56 => Some(self.fHslider44),
			53 => Some(self.fHslider45),
			55 => Some(self.fHslider46),
			58 => Some(self.fHslider47),
			50 => Some(self.fHslider48),
			47 => Some(self.fHslider49),
			123 => Some(self.fHslider5),
			49 => Some(self.fHslider50),
			52 => Some(self.fHslider51),
			99 => Some(self.fHslider52),
			98 => Some(self.fHslider53),
			97 => Some(self.fHslider54),
			96 => Some(self.fHslider55),
			95 => Some(self.fHslider56),
			94 => Some(self.fHslider57),
			93 => Some(self.fHslider58),
			46 => Some(self.fHslider59),
			6 => Some(self.fHslider6),
			45 => Some(self.fHslider60),
			44 => Some(self.fHslider61),
			43 => Some(self.fHslider62),
			42 => Some(self.fHslider63),
			41 => Some(self.fHslider64),
			40 => Some(self.fHslider65),
			81 => Some(self.fHslider66),
			80 => Some(self.fHslider67),
			90 => Some(self.fHslider68),
			89 => Some(self.fHslider69),
			129 => Some(self.fHslider7),
			88 => Some(self.fHslider70),
			87 => Some(self.fHslider71),
			86 => Some(self.fHslider72),
			85 => Some(self.fHslider73),
			28 => Some(self.fHslider74),
			27 => Some(self.fHslider75),
			37 => Some(self.fHslider76),
			36 => Some(self.fHslider77),
			35 => Some(self.fHslider78),
			34 => Some(self.fHslider79),
			127 => Some(self.fHslider8),
			33 => Some(self.fHslider80),
			32 => Some(self.fHslider81),
			125 => Some(self.fHslider9),
			121 => Some(self.fVslider0),
			122 => Some(self.fVslider1),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			126 => { self.fButton0 = value }
			3 => { self.fCheckbox0 = value }
			5 => { self.fCheckbox1 = value }
			113 => { self.fCheckbox2 = value }
			107 => { self.fCheckbox3 = value }
			101 => { self.fCheckbox4 = value }
			60 => { self.fCheckbox5 = value }
			54 => { self.fCheckbox6 = value }
			48 => { self.fCheckbox7 = value }
			84 => { self.fCheckbox8 = value }
			31 => { self.fCheckbox9 = value }
			66 => { self.fEntry0 = value }
			15 => { self.fEntry1 = value }
			22 => { self.fEntry10 = value }
			23 => { self.fEntry11 = value }
			20 => { self.fEntry12 = value }
			70 => { self.fEntry13 = value }
			69 => { self.fEntry14 = value }
			74 => { self.fEntry15 = value }
			75 => { self.fEntry16 = value }
			76 => { self.fEntry17 = value }
			73 => { self.fEntry18 = value }
			1 => { self.fEntry19 = value }
			25 => { self.fEntry2 = value }
			10 => { self.fEntry20 = value }
			116 => { self.fEntry21 = value }
			110 => { self.fEntry22 = value }
			104 => { self.fEntry23 = value }
			118 => { self.fEntry24 = value }
			63 => { self.fEntry25 = value }
			57 => { self.fEntry26 = value }
			51 => { self.fEntry27 = value }
			65 => { self.fEntry28 = value }
			92 => { self.fEntry29 = value }
			78 => { self.fEntry3 = value }
			91 => { self.fEntry30 = value }
			39 => { self.fEntry31 = value }
			38 => { self.fEntry32 = value }
			83 => { self.fEntry33 = value }
			82 => { self.fEntry34 = value }
			30 => { self.fEntry35 = value }
			29 => { self.fEntry36 = value }
			119 => { self.fEntry4 = value }
			7 => { self.fEntry5 = value }
			68 => { self.fEntry6 = value }
			17 => { self.fEntry7 = value }
			16 => { self.fEntry8 = value }
			21 => { self.fEntry9 = value }
			67 => { self.fHslider0 = value }
			128 => { self.fHslider1 = value }
			124 => { self.fHslider10 = value }
			130 => { self.fHslider11 = value }
			19 => { self.fHslider12 = value }
			18 => { self.fHslider13 = value }
			24 => { self.fHslider14 = value }
			26 => { self.fHslider15 = value }
			120 => { self.fHslider16 = value }
			72 => { self.fHslider17 = value }
			71 => { self.fHslider18 = value }
			77 => { self.fHslider19 = value }
			0 => { self.fHslider2 = value }
			79 => { self.fHslider20 = value }
			4 => { self.fHslider21 = value }
			2 => { self.fHslider22 = value }
			14 => { self.fHslider23 = value }
			13 => { self.fHslider24 = value }
			12 => { self.fHslider25 = value }
			11 => { self.fHslider26 = value }
			9 => { self.fHslider27 = value }
			115 => { self.fHslider28 = value }
			112 => { self.fHslider29 = value }
			131 => { self.fHslider3 = value }
			114 => { self.fHslider30 = value }
			117 => { self.fHslider31 = value }
			109 => { self.fHslider32 = value }
			106 => { self.fHslider33 = value }
			108 => { self.fHslider34 = value }
			111 => { self.fHslider35 = value }
			103 => { self.fHslider36 = value }
			100 => { self.fHslider37 = value }
			102 => { self.fHslider38 = value }
			105 => { self.fHslider39 = value }
			8 => { self.fHslider4 = value }
			62 => { self.fHslider40 = value }
			59 => { self.fHslider41 = value }
			61 => { self.fHslider42 = value }
			64 => { self.fHslider43 = value }
			56 => { self.fHslider44 = value }
			53 => { self.fHslider45 = value }
			55 => { self.fHslider46 = value }
			58 => { self.fHslider47 = value }
			50 => { self.fHslider48 = value }
			47 => { self.fHslider49 = value }
			123 => { self.fHslider5 = value }
			49 => { self.fHslider50 = value }
			52 => { self.fHslider51 = value }
			99 => { self.fHslider52 = value }
			98 => { self.fHslider53 = value }
			97 => { self.fHslider54 = value }
			96 => { self.fHslider55 = value }
			95 => { self.fHslider56 = value }
			94 => { self.fHslider57 = value }
			93 => { self.fHslider58 = value }
			46 => { self.fHslider59 = value }
			6 => { self.fHslider6 = value }
			45 => { self.fHslider60 = value }
			44 => { self.fHslider61 = value }
			43 => { self.fHslider62 = value }
			42 => { self.fHslider63 = value }
			41 => { self.fHslider64 = value }
			40 => { self.fHslider65 = value }
			81 => { self.fHslider66 = value }
			80 => { self.fHslider67 = value }
			90 => { self.fHslider68 = value }
			89 => { self.fHslider69 = value }
			129 => { self.fHslider7 = value }
			88 => { self.fHslider70 = value }
			87 => { self.fHslider71 = value }
			86 => { self.fHslider72 = value }
			85 => { self.fHslider73 = value }
			28 => { self.fHslider74 = value }
			27 => { self.fHslider75 = value }
			37 => { self.fHslider76 = value }
			36 => { self.fHslider77 = value }
			35 => { self.fHslider78 = value }
			34 => { self.fHslider79 = value }
			127 => { self.fHslider8 = value }
			33 => { self.fHslider80 = value }
			32 => { self.fHslider81 = value }
			125 => { self.fHslider9 = value }
			121 => { self.fVslider0 = value }
			122 => { self.fVslider1 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
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
		let mut iSlow0: i32 = (self.fVslider0 != 0.0) as i32;
		let mut fSlow1: F32 = (iSlow0) as F32;
		let mut iSlow2: i32 = (fSlow1 != 0.0) as i32;
		let mut fSlow3: F32 = self.fEntry0;
		let mut iSlow4: i32 = (fSlow3 == 0.0) as i32;
		let mut fSlow5: F32 = (iSlow4) as F32;
		let mut iSlow6: i32 = (fSlow5 != 0.0) as i32;
		let mut fSlow7: F32 = self.fEntry1;
		let mut iSlow8: i32 = (fSlow7 == 2.0) as i32;
		let mut fSlow9: F32 = (iSlow8) as F32;
		let mut iSlow10: i32 = (fSlow9 != 0.0) as i32;
		let mut iSlow11: i32 = (fSlow7 == 0.0) as i32;
		let mut fSlow12: F32 = (iSlow11) as F32;
		let mut iSlow13: i32 = (fSlow12 != 0.0) as i32;
		let mut iSlow14: i32 = (fSlow7 == 3.0) as i32;
		let mut fSlow15: F32 = (iSlow14) as F32;
		let mut iSlow16: i32 = (fSlow15 != 0.0) as i32;
		let mut iSlow17: i32 = (fSlow7 == 4.0) as i32;
		let mut fSlow18: F32 = (iSlow17) as F32;
		let mut iSlow19: i32 = (fSlow18 != 0.0) as i32;
		let mut iSlow20: i32 = (fSlow7 == 5.0) as i32;
		let mut fSlow21: F32 = (iSlow20) as F32;
		let mut iSlow22: i32 = (fSlow21 != 0.0) as i32;
		let mut iSlow23: i32 = (fSlow7 == 6.0) as i32;
		let mut fSlow24: F32 = (iSlow23) as F32;
		let mut iSlow25: i32 = (fSlow24 != 0.0) as i32;
		let mut iSlow26: i32 = (fSlow7 == 7.0) as i32;
		let mut fSlow27: F32 = (iSlow26) as F32;
		let mut iSlow28: i32 = (fSlow27 != 0.0) as i32;
		let mut iSlow29: i32 = (fSlow7 == 9.0) as i32;
		let mut fSlow30: F32 = (iSlow29) as F32;
		let mut iSlow31: i32 = (fSlow30 != 0.0) as i32;
		let mut iSlow32: i32 = (fSlow7 == 11.0) as i32;
		let mut fSlow33: F32 = (iSlow32) as F32;
		let mut iSlow34: i32 = (fSlow33 != 0.0) as i32;
		let mut iSlow35: i32 = (fSlow7 == 1.0) as i32;
		let mut fSlow36: F32 = (iSlow35) as F32;
		let mut iSlow37: i32 = (fSlow36 != 0.0) as i32;
		let mut fSlow38: F32 = self.fEntry2;
		let mut iSlow39: i32 = (fSlow38 == 14.0) as i32;
		let mut fSlow40: F32 = (iSlow39) as F32;
		let mut iSlow41: i32 = (fSlow40 != 0.0) as i32;
		let mut iSlow42: i32 = (fSlow3 == 1.0) as i32;
		let mut fSlow43: F32 = (iSlow42) as F32;
		let mut iSlow44: i32 = (fSlow43 != 0.0) as i32;
		let mut iSlow45: i32 = (fSlow38 == 13.0) as i32;
		let mut fSlow46: F32 = (iSlow45) as F32;
		let mut iSlow47: i32 = (fSlow46 != 0.0) as i32;
		let mut iSlow48: i32 = (fSlow38 == 12.0) as i32;
		let mut fSlow49: F32 = (iSlow48) as F32;
		let mut iSlow50: i32 = (fSlow49 != 0.0) as i32;
		let mut iSlow51: i32 = (fSlow38 == 11.0) as i32;
		let mut fSlow52: F32 = (iSlow51) as F32;
		let mut iSlow53: i32 = (fSlow52 != 0.0) as i32;
		let mut iSlow54: i32 = (fSlow38 == 1e+01) as i32;
		let mut fSlow55: F32 = (iSlow54) as F32;
		let mut iSlow56: i32 = (fSlow55 != 0.0) as i32;
		let mut iSlow57: i32 = (fSlow38 == 9.0) as i32;
		let mut fSlow58: F32 = (iSlow57) as F32;
		let mut iSlow59: i32 = (fSlow58 != 0.0) as i32;
		let mut iSlow60: i32 = (fSlow38 == 8.0) as i32;
		let mut fSlow61: F32 = (iSlow60) as F32;
		let mut iSlow62: i32 = (fSlow61 != 0.0) as i32;
		let mut iSlow63: i32 = (fSlow38 == 7.0) as i32;
		let mut fSlow64: F32 = (iSlow63) as F32;
		let mut iSlow65: i32 = (fSlow64 != 0.0) as i32;
		let mut iSlow66: i32 = (fSlow38 == 6.0) as i32;
		let mut fSlow67: F32 = (iSlow66) as F32;
		let mut iSlow68: i32 = (fSlow67 != 0.0) as i32;
		let mut iSlow69: i32 = (fSlow38 == 1.0) as i32;
		let mut fSlow70: F32 = (iSlow69) as F32;
		let mut iSlow71: i32 = (fSlow70 != 0.0) as i32;
		let mut iSlow72: i32 = (fSlow38 == 5.0) as i32;
		let mut fSlow73: F32 = (iSlow72) as F32;
		let mut iSlow74: i32 = (fSlow73 != 0.0) as i32;
		let mut iSlow75: i32 = (fSlow38 == 4.0) as i32;
		let mut fSlow76: F32 = (iSlow75) as F32;
		let mut iSlow77: i32 = (fSlow76 != 0.0) as i32;
		let mut iSlow78: i32 = (fSlow38 == 3.0) as i32;
		let mut fSlow79: F32 = (iSlow78) as F32;
		let mut iSlow80: i32 = (fSlow79 != 0.0) as i32;
		let mut iSlow81: i32 = (fSlow38 == 2.0) as i32;
		let mut fSlow82: F32 = (iSlow81) as F32;
		let mut iSlow83: i32 = (fSlow82 != 0.0) as i32;
		let mut iSlow84: i32 = (fSlow38 == 0.0) as i32;
		let mut fSlow85: F32 = (iSlow84) as F32;
		let mut iSlow86: i32 = (fSlow85 != 0.0) as i32;
		let mut fSlow87: F32 = self.fButton0;
		let mut fSlow88: F32 = self.fHslider0;
		let mut fSlow89: F32 = self.fHslider1;
		let mut fSlow90: F32 = self.fHslider2;
		let mut fSlow91: F32 = self.fEntry3;
		let mut iSlow92: i32 = (fSlow91 == 4.0) as i32;
		let mut fSlow93: F32 = (iSlow92) as F32;
		let mut iSlow94: i32 = (fSlow93 != 0.0) as i32;
		let mut fSlow95: F32 = self.fEntry4;
		let mut iSlow96: i32 = (fSlow95 == 1.0) as i32;
		let mut fSlow97: F32 = (iSlow96) as F32;
		let mut iSlow98: i32 = (fSlow97 != 0.0) as i32;
		let mut iSlow99: i32 = (self.fVslider1 != 0.0) as i32;
		let mut fSlow100: F32 = (iSlow99) as F32;
		let mut iSlow101: i32 = (fSlow100 != 0.0) as i32;
		let mut iSlow102: i32 = (fSlow91 == 3.0) as i32;
		let mut fSlow103: F32 = (iSlow102) as F32;
		let mut iSlow104: i32 = (fSlow103 != 0.0) as i32;
		let mut iSlow105: i32 = (fSlow91 == 2.0) as i32;
		let mut fSlow106: F32 = (iSlow105) as F32;
		let mut iSlow107: i32 = (fSlow106 != 0.0) as i32;
		let mut iTemp7: i32 = self.iTempPerm0;
		let mut fTemp8: F32 = self.fTempPerm1;
		let mut fSlow108: F32 = self.fHslider3;
		let mut fSlow109: F32 = 12.9898 * fSlow108;
		let mut fSlow110: F32 = 43758.547 * F32::sin(fSlow109 + 234.699);
		let mut fSlow111: F32 = 43758.547 * F32::sin(fSlow109 + 7979.766);
		let mut fSlow112: F32 = 2.0 * (fSlow111 - F32::floor(fSlow111)) + -1.0;
		let mut fSlow113: F32 = self.fConst2 * (0.1 * F32::max(-1.0, F32::min(1.0, 0.70710677 * (fSlow112 + 2.0 * (fSlow110 - F32::floor(fSlow110)) + -1.0))) + 0.45);
		let mut fTemp9: F32 = self.fTempPerm2;
		let mut iSlow114: i32 = (409.0 * fSlow108 + 4e+03) as i32;
		let mut iTemp10: i32 = self.iTempPerm3;
		let mut fSlow115: F32 = self.fHslider4;
		let mut fSlow116: F32 = 43758.547 * F32::sin(fSlow109 + 156.466);
		let mut fSlow117: F32 = 0.15 * F32::max(-1.0, F32::min(1.0, 1.2803688 * (0.6 * fSlow112 + 0.5 * (2.0 * (fSlow116 - F32::floor(fSlow116)) + -1.0)))) + 0.25;
		let mut fSlow118: F32 = 1.0 / F32::tan(self.fConst8 * F32::max(1.0, fSlow117));
		let mut fSlow119: F32 = 1.0 - fSlow118;
		let mut fSlow120: F32 = self.fConst2 * F32::max(fSlow117, 0.0);
		let mut fTemp13: F32 = self.fTempPerm4;
		let mut fTemp16: F32 = self.fTempPerm5;
		let mut fSlow121: F32 = 1.0 / (fSlow118 + 1.0);
		let mut fSlow122: F32 = 43758.547 * F32::sin(fSlow109 + 78.233);
		let mut fSlow123: F32 = 43758.547 * F32::sin(fSlow109 + 7901.533);
		let mut fSlow124: F32 = 8.0 * F32::max(-1.0, F32::min(1.0, 1.0976425 * (0.7 * (2.0 * (fSlow123 - F32::floor(fSlow123)) + -1.0) + 0.3 * fSlow112 + 0.5 * (2.0 * (fSlow122 - F32::floor(fSlow122)) + -1.0))));
		let mut fTemp17: F32 = self.fTempPerm6;
		let mut fSlow125: F32 = self.fHslider5;
		let mut fSlow126: F32 = self.fEntry5;
		let mut fSlow127: F32 = 2.0 * ((fSlow126 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow126 == 1.0) as i32) as u32 as F32;
		let mut fSlow128: F32 = self.fHslider6;
		let mut fSlow129: F32 = self.fHslider7;
		let mut fSlow130: F32 = 0.33333334 * F32::max(1e-05, self.fHslider8);
		let mut iSlow131: i32 = (F32::abs(fSlow130) < 1.1920929e-07) as i32;
		let mut fSlow132: F32 = if iSlow131 != 0 {0.0} else {F32::exp(-(self.fConst2 / if iSlow131 != 0 {1.0} else {fSlow130}))};
		let mut fSlow133: F32 = F32::log(self.fHslider9, std::f32::consts::E) * (1.0 - fSlow132);
		let mut fTemp24: F32 = self.fTempPerm7;
		let mut fTemp25: F32 = self.fTempPerm8;
		let mut fSlow134: F32 = self.fHslider10;
		let mut fSlow135: F32 = 0.618 * fSlow134 + 7.31 * self.fHslider11 * (fSlow134 + 1.0);
		let mut fSlow136: F32 = self.fCheckbox0;
		let mut fSlow137: F32 = self.fEntry6;
		let mut iSlow138: i32 = (fSlow137 == 2.0) as i32;
		let mut fSlow139: F32 = (iSlow138) as F32;
		let mut iSlow140: i32 = (fSlow139 != 0.0) as i32;
		let mut iSlow141: i32 = (fSlow95 == 0.0) as i32;
		let mut fSlow142: F32 = (iSlow141) as F32;
		let mut iSlow143: i32 = (fSlow142 != 0.0) as i32;
		let mut iSlow144: i32 = (fSlow137 == 0.0) as i32;
		let mut fSlow145: F32 = (iSlow144) as F32;
		let mut iSlow146: i32 = (fSlow145 != 0.0) as i32;
		let mut iSlow147: i32 = (fSlow137 == 3.0) as i32;
		let mut fSlow148: F32 = (iSlow147) as F32;
		let mut iSlow149: i32 = (fSlow148 != 0.0) as i32;
		let mut iSlow150: i32 = (fSlow137 == 4.0) as i32;
		let mut fSlow151: F32 = (iSlow150) as F32;
		let mut iSlow152: i32 = (fSlow151 != 0.0) as i32;
		let mut iSlow153: i32 = (fSlow137 == 5.0) as i32;
		let mut fSlow154: F32 = (iSlow153) as F32;
		let mut iSlow155: i32 = (fSlow154 != 0.0) as i32;
		let mut iSlow156: i32 = (fSlow137 == 6.0) as i32;
		let mut fSlow157: F32 = (iSlow156) as F32;
		let mut iSlow158: i32 = (fSlow157 != 0.0) as i32;
		let mut iSlow159: i32 = (fSlow137 == 7.0) as i32;
		let mut fSlow160: F32 = (iSlow159) as F32;
		let mut iSlow161: i32 = (fSlow160 != 0.0) as i32;
		let mut iSlow162: i32 = (fSlow137 == 8.0) as i32;
		let mut fSlow163: F32 = (iSlow162) as F32;
		let mut iSlow164: i32 = (fSlow163 != 0.0) as i32;
		let mut iSlow165: i32 = (fSlow137 == 9.0) as i32;
		let mut fSlow166: F32 = (iSlow165) as F32;
		let mut iSlow167: i32 = (fSlow166 != 0.0) as i32;
		let mut iSlow168: i32 = (fSlow137 == 1e+01) as i32;
		let mut fSlow169: F32 = (iSlow168) as F32;
		let mut iSlow170: i32 = (fSlow169 != 0.0) as i32;
		let mut iSlow171: i32 = (fSlow137 == 11.0) as i32;
		let mut fSlow172: F32 = (iSlow171) as F32;
		let mut iSlow173: i32 = (fSlow172 != 0.0) as i32;
		let mut iSlow174: i32 = (fSlow137 == 1.0) as i32;
		let mut fSlow175: F32 = (iSlow174) as F32;
		let mut iSlow176: i32 = (fSlow175 != 0.0) as i32;
		let mut iSlow177: i32 = (fSlow91 == 14.0) as i32;
		let mut fSlow178: F32 = (iSlow177) as F32;
		let mut iSlow179: i32 = (fSlow178 != 0.0) as i32;
		let mut iSlow180: i32 = (fSlow91 == 13.0) as i32;
		let mut fSlow181: F32 = (iSlow180) as F32;
		let mut iSlow182: i32 = (fSlow181 != 0.0) as i32;
		let mut iSlow183: i32 = (fSlow91 == 12.0) as i32;
		let mut fSlow184: F32 = (iSlow183) as F32;
		let mut iSlow185: i32 = (fSlow184 != 0.0) as i32;
		let mut iSlow186: i32 = (fSlow91 == 11.0) as i32;
		let mut fSlow187: F32 = (iSlow186) as F32;
		let mut iSlow188: i32 = (fSlow187 != 0.0) as i32;
		let mut iSlow189: i32 = (fSlow91 == 1e+01) as i32;
		let mut fSlow190: F32 = (iSlow189) as F32;
		let mut iSlow191: i32 = (fSlow190 != 0.0) as i32;
		let mut iSlow192: i32 = (fSlow91 == 9.0) as i32;
		let mut fSlow193: F32 = (iSlow192) as F32;
		let mut iSlow194: i32 = (fSlow193 != 0.0) as i32;
		let mut iSlow195: i32 = (fSlow91 == 8.0) as i32;
		let mut fSlow196: F32 = (iSlow195) as F32;
		let mut iSlow197: i32 = (fSlow196 != 0.0) as i32;
		let mut iSlow198: i32 = (fSlow91 == 7.0) as i32;
		let mut fSlow199: F32 = (iSlow198) as F32;
		let mut iSlow200: i32 = (fSlow199 != 0.0) as i32;
		let mut iSlow201: i32 = (fSlow91 == 6.0) as i32;
		let mut fSlow202: F32 = (iSlow201) as F32;
		let mut iSlow203: i32 = (fSlow202 != 0.0) as i32;
		let mut iSlow204: i32 = (fSlow91 == 5.0) as i32;
		let mut fSlow205: F32 = (iSlow204) as F32;
		let mut iSlow206: i32 = (fSlow205 != 0.0) as i32;
		let mut iSlow207: i32 = (fSlow91 == 1.0) as i32;
		let mut fSlow208: F32 = (iSlow207) as F32;
		let mut iSlow209: i32 = (fSlow208 != 0.0) as i32;
		let mut iSlow210: i32 = (fSlow91 == 0.0) as i32;
		let mut fSlow211: F32 = (iSlow210) as F32;
		let mut iSlow212: i32 = (fSlow211 != 0.0) as i32;
		let mut iSlow213: i32 = (fSlow7 == 8.0) as i32;
		let mut fSlow214: F32 = (iSlow213) as F32;
		let mut iSlow215: i32 = (fSlow214 != 0.0) as i32;
		let mut iSlow216: i32 = (fSlow7 == 1e+01) as i32;
		let mut fSlow217: F32 = (iSlow216) as F32;
		let mut iSlow218: i32 = (fSlow217 != 0.0) as i32;
		let mut fTemp29: F32 = self.fTempPerm9;
		let mut iTemp30: i32 = self.iTempPerm10;
		let mut fSlow219: F32 = self.fHslider12;
		let mut fTemp31: F32 = self.fTempPerm11;
		let mut fTemp32: F32 = self.fTempPerm12;
		let mut fSlow220: F32 = self.fHslider13;
		let mut fTemp33: F32 = self.fTempPerm13;
		let mut fTemp34: F32 = self.fTempPerm14;
		let mut fTemp35: F32 = self.fTempPerm15;
		let mut fTemp36: F32 = self.fTempPerm16;
		let mut fTemp37: F32 = self.fTempPerm17;
		let mut fTemp38: F32 = self.fTempPerm18;
		let mut fTemp39: F32 = self.fTempPerm19;
		let mut fTemp40: F32 = self.fTempPerm20;
		let mut fSlow221: F32 = self.fEntry7;
		let mut fSlow222: F32 = self.fEntry8;
		let mut fTemp41: F32 = self.fTempPerm21;
		let mut fTemp42: F32 = self.fTempPerm22;
		let mut fTemp43: F32 = self.fTempPerm23;
		let mut iTemp44: i32 = self.iTempPerm24;
		let mut fTemp45: F32 = self.fTempPerm25;
		let mut fTemp46: F32 = self.fTempPerm26;
		let mut fTemp47: F32 = self.fTempPerm27;
		let mut fTemp48: F32 = self.fTempPerm28;
		let mut fTemp49: F32 = self.fTempPerm29;
		let mut fTemp50: F32 = self.fTempPerm30;
		let mut fTemp51: F32 = self.fTempPerm31;
		let mut fTemp52: F32 = self.fTempPerm32;
		let mut fTemp53: F32 = self.fTempPerm33;
		let mut fTemp54: F32 = self.fTempPerm34;
		let mut fTemp55: F32 = self.fTempPerm35;
		let mut fTemp56: F32 = self.fTempPerm36;
		let mut fTemp57: F32 = self.fTempPerm37;
		let mut fTemp58: F32 = self.fTempPerm38;
		let mut fTemp59: F32 = self.fTempPerm39;
		let mut fTemp60: F32 = self.fTempPerm40;
		let mut fSlow223: F32 = ((iSlow8 != 0) as i32) as u32 as F32;
		let mut iSlow224: i32 = (fSlow221) as i32;
		let mut fTemp61: F32 = self.fTempPerm41;
		let mut fTemp62: F32 = self.fTempPerm42;
		let mut fSlow225: F32 = 0.5 * fSlow136;
		let mut fTemp63: F32 = self.fTempPerm43;
		let mut fTemp64: F32 = self.fTempPerm44;
		let mut fTemp65: F32 = self.fTempPerm45;
		let mut fTemp66: F32 = self.fTempPerm46;
		let mut fTemp67: F32 = self.fTempPerm47;
		let mut fTemp68: F32 = self.fTempPerm48;
		let mut fTemp69: F32 = self.fTempPerm49;
		let mut fTemp70: F32 = self.fTempPerm50;
		let mut fTemp71: F32 = self.fTempPerm51;
		let mut fTemp72: F32 = self.fTempPerm52;
		let mut fSlow226: F32 = ((iSlow11 != 0) as i32) as u32 as F32;
		let mut iTemp73: i32 = self.iTempPerm53;
		let mut iSlow227: i32 = (fSlow222) as i32;
		let mut fTemp74: F32 = self.fTempPerm54;
		let mut fTemp75: F32 = self.fTempPerm55;
		let mut fTemp76: F32 = self.fTempPerm56;
		let mut fTemp77: F32 = self.fTempPerm57;
		let mut fTemp78: F32 = self.fTempPerm58;
		let mut fTemp79: F32 = self.fTempPerm59;
		let mut fTemp80: F32 = self.fTempPerm60;
		let mut fTemp81: F32 = self.fTempPerm61;
		let mut fTemp82: F32 = self.fTempPerm62;
		let mut iSlow228: i32 = (fSlow222 >= 2.0) as i32;
		let mut iSlow229: i32 = (fSlow222 >= 1.0) as i32;
		let mut fTemp83: F32 = self.fTempPerm63;
		let mut iSlow230: i32 = (fSlow222 >= 3.0) as i32;
		let mut fSlow231: F32 = (if iSlow230 != 0 {0} else {2}) as F32;
		let mut fSlow232: F32 = if iSlow228 != 0 {fSlow231} else {if iSlow229 != 0 {4.0} else {6.2831855}};
		let mut fTemp84: F32 = self.fTempPerm64;
		let mut fTemp85: F32 = self.fTempPerm65;
		let mut fTemp86: F32 = self.fTempPerm66;
		let mut fTemp87: F32 = self.fTempPerm67;
		let mut fTemp88: F32 = self.fTempPerm68;
		let mut fTemp89: F32 = self.fTempPerm69;
		let mut fSlow233: F32 = ((iSlow14 != 0) as i32) as u32 as F32;
		let mut fTemp90: F32 = self.fTempPerm70;
		let mut fTemp91: F32 = self.fTempPerm71;
		let mut fTemp92: F32 = self.fTempPerm72;
		let mut fTemp93: F32 = self.fTempPerm73;
		let mut fTemp94: F32 = self.fTempPerm74;
		let mut iTemp95: i32 = self.iTempPerm75;
		let mut fTemp96: F32 = self.fTempPerm76;
		let mut fTemp97: F32 = self.fTempPerm77;
		let mut fTemp98: F32 = self.fTempPerm78;
		let mut fTemp99: F32 = self.fTempPerm79;
		let mut fTemp100: F32 = self.fTempPerm80;
		let mut iTemp101: i32 = self.iTempPerm81;
		let mut fTemp102: F32 = self.fTempPerm82;
		let mut fTemp103: F32 = self.fTempPerm83;
		let mut fTemp104: F32 = self.fTempPerm84;
		let mut fTemp105: F32 = self.fTempPerm85;
		let mut fTemp106: F32 = self.fTempPerm86;
		let mut fTemp107: F32 = self.fTempPerm87;
		let mut fSlow234: F32 = ((iSlow17 != 0) as i32) as u32 as F32;
		let mut fTemp108: F32 = self.fTempPerm88;
		let mut iTemp109: i32 = self.iTempPerm89;
		let mut iTemp110: i32 = self.iTempPerm90;
		let mut iTemp111: i32 = self.iTempPerm91;
		let mut iTemp112: i32 = self.iTempPerm92;
		let mut iTemp113: i32 = self.iTempPerm93;
		let mut iTemp114: i32 = self.iTempPerm94;
		let mut fTemp115: F32 = self.fTempPerm95;
		let mut fTemp116: F32 = self.fTempPerm96;
		let mut fTemp117: F32 = self.fTempPerm97;
		let mut iTemp118: i32 = self.iTempPerm98;
		let mut iTemp119: i32 = self.iTempPerm99;
		let mut iTemp120: i32 = self.iTempPerm100;
		let mut iTemp121: i32 = self.iTempPerm101;
		let mut fTemp122: F32 = self.fTempPerm102;
		let mut fTemp123: F32 = self.fTempPerm103;
		let mut fTemp124: F32 = self.fTempPerm104;
		let mut fTemp125: F32 = self.fTempPerm105;
		let mut fTemp126: F32 = self.fTempPerm106;
		let mut fTemp127: F32 = self.fTempPerm107;
		let mut fTemp128: F32 = self.fTempPerm108;
		let mut fTemp129: F32 = self.fTempPerm109;
		let mut fTemp130: F32 = self.fTempPerm110;
		let mut fSlow235: F32 = ((iSlow20 != 0) as i32) as u32 as F32;
		let mut fTemp131: F32 = self.fTempPerm111;
		let mut fTemp132: F32 = self.fTempPerm112;
		let mut fTemp133: F32 = self.fTempPerm113;
		let mut fTemp134: F32 = self.fTempPerm114;
		let mut fTemp135: F32 = self.fTempPerm115;
		let mut fTemp136: F32 = self.fTempPerm116;
		let mut fTemp137: F32 = self.fTempPerm117;
		let mut fTemp138: F32 = self.fTempPerm118;
		let mut fTemp139: F32 = self.fTempPerm119;
		let mut fTemp140: F32 = self.fTempPerm120;
		let mut fSlow236: F32 = ((iSlow23 != 0) as i32) as u32 as F32;
		let mut iTemp141: i32 = self.iTempPerm121;
		let mut fTemp142: F32 = self.fTempPerm122;
		let mut fTemp143: F32 = self.fTempPerm123;
		let mut fTemp144: F32 = self.fTempPerm124;
		let mut fTemp145: F32 = self.fTempPerm125;
		let mut fTemp146: F32 = self.fTempPerm126;
		let mut fTemp147: F32 = self.fTempPerm127;
		let mut fTemp148: F32 = self.fTempPerm128;
		let mut fTemp149: F32 = self.fTempPerm129;
		let mut fTemp150: F32 = self.fTempPerm130;
		let mut fTemp151: F32 = self.fTempPerm131;
		let mut fTemp152: F32 = self.fTempPerm132;
		let mut fSlow237: F32 = ((iSlow26 != 0) as i32) as u32 as F32;
		let mut fTemp153: F32 = self.fTempPerm133;
		let mut fTemp154: F32 = self.fTempPerm134;
		let mut fTemp155: F32 = self.fTempPerm135;
		let mut iTemp156: i32 = self.iTempPerm136;
		let mut iSlow238: i32 = (fSlow221 >= 2.0) as i32;
		let mut iSlow239: i32 = (fSlow221 >= 3.0) as i32;
		let mut iSlow240: i32 = if iSlow238 != 0 {if iSlow239 != 0 {-2} else {0}} else {0};
		let mut iSlow241: i32 = (fSlow221 >= 1.0) as i32;
		let mut fSlow242: F32 = if iSlow238 != 0 {if iSlow239 != 0 {0.5} else {0.0}} else {if iSlow241 != 0 {0.75} else {0.0}};
		let mut fTemp157: F32 = self.fTempPerm137;
		let mut fTemp158: F32 = self.fTempPerm138;
		let mut fTemp159: F32 = self.fTempPerm139;
		let mut fTemp160: F32 = self.fTempPerm140;
		let mut iTemp161: i32 = self.iTempPerm141;
		let mut iSlow243: i32 = if iSlow238 != 0 {if iSlow239 != 0 {2} else {-2}} else {0};
		let mut fSlow244: F32 = if iSlow238 != 0 {if iSlow239 != 0 {0.0} else {0.5}} else {if iSlow241 != 0 {0.25} else {0.0}};
		let mut fTemp162: F32 = self.fTempPerm142;
		let mut iTemp163: i32 = self.iTempPerm143;
		let mut fTemp164: F32 = self.fTempPerm144;
		let mut fTemp165: F32 = self.fTempPerm145;
		let mut fTemp166: F32 = self.fTempPerm146;
		let mut fTemp167: F32 = self.fTempPerm147;
		let mut fTemp168: F32 = self.fTempPerm148;
		let mut iTemp169: i32 = self.iTempPerm149;
		let mut iSlow245: i32 = if iSlow228 != 0 {if iSlow230 != 0 {-2} else {0}} else {0};
		let mut fTemp170: F32 = self.fTempPerm150;
		let mut fTemp171: F32 = self.fTempPerm151;
		let mut fSlow246: F32 = if iSlow228 != 0 {if iSlow230 != 0 {0.5} else {0.0}} else {if iSlow229 != 0 {0.75} else {0.0}};
		let mut fTemp172: F32 = self.fTempPerm152;
		let mut fTemp173: F32 = self.fTempPerm153;
		let mut fTemp174: F32 = self.fTempPerm154;
		let mut fTemp175: F32 = self.fTempPerm155;
		let mut fSlow247: F32 = (if iSlow228 != 0 {0} else {if iSlow229 != 0 {8} else {0}}) as F32;
		let mut fTemp176: F32 = self.fTempPerm156;
		let mut iSlow248: i32 = if iSlow228 != 0 {if iSlow230 != 0 {2} else {-2}} else {0};
		let mut fTemp177: F32 = self.fTempPerm157;
		let mut fSlow249: F32 = if iSlow228 != 0 {if iSlow230 != 0 {0.0} else {0.5}} else {if iSlow229 != 0 {0.25} else {0.0}};
		let mut fTemp178: F32 = self.fTempPerm158;
		let mut fTemp179: F32 = self.fTempPerm159;
		let mut fTemp180: F32 = self.fTempPerm160;
		let mut fSlow250: F32 = (if iSlow228 != 0 {0} else {if iSlow229 != 0 {-8} else {0}}) as F32;
		let mut fTemp181: F32 = self.fTempPerm161;
		let mut fTemp182: F32 = self.fTempPerm162;
		let mut fTemp183: F32 = self.fTempPerm163;
		let mut fTemp184: F32 = self.fTempPerm164;
		let mut fSlow251: F32 = ((iSlow213 != 0) as i32) as u32 as F32;
		let mut fTemp185: F32 = self.fTempPerm165;
		let mut fTemp186: F32 = self.fTempPerm166;
		let mut fTemp187: F32 = self.fTempPerm167;
		let mut fTemp188: F32 = self.fTempPerm168;
		let mut fSlow252: F32 = ((iSlow29 != 0) as i32) as u32 as F32;
		let mut fTemp189: F32 = self.fTempPerm169;
		let mut fTemp190: F32 = self.fTempPerm170;
		let mut fTemp191: F32 = self.fTempPerm171;
		let mut iTemp192: i32 = self.iTempPerm172;
		let mut fTemp193: F32 = self.fTempPerm173;
		let mut iTemp194: i32 = self.iTempPerm174;
		let mut fTemp195: F32 = self.fTempPerm175;
		let mut iTemp196: i32 = self.iTempPerm176;
		let mut fTemp197: F32 = self.fTempPerm177;
		let mut iTemp198: i32 = self.iTempPerm178;
		let mut fTemp199: F32 = self.fTempPerm179;
		let mut fTemp200: F32 = self.fTempPerm180;
		let mut fTemp201: F32 = self.fTempPerm181;
		let mut fTemp202: F32 = self.fTempPerm182;
		let mut iTemp203: i32 = self.iTempPerm183;
		let mut fTemp204: F32 = self.fTempPerm184;
		let mut fTemp205: F32 = self.fTempPerm185;
		let mut fTemp206: F32 = self.fTempPerm186;
		let mut fTemp207: F32 = self.fTempPerm187;
		let mut fTemp208: F32 = self.fTempPerm188;
		let mut fTemp209: F32 = self.fTempPerm189;
		let mut fTemp210: F32 = self.fTempPerm190;
		let mut fTemp211: F32 = self.fTempPerm191;
		let mut fTemp212: F32 = self.fTempPerm192;
		let mut fTemp213: F32 = self.fTempPerm193;
		let mut fTemp214: F32 = self.fTempPerm194;
		let mut fTemp215: F32 = self.fTempPerm195;
		let mut fTemp216: F32 = self.fTempPerm196;
		let mut fTemp217: F32 = self.fTempPerm197;
		let mut fTemp218: F32 = self.fTempPerm198;
		let mut fTemp219: F32 = self.fTempPerm199;
		let mut fTemp220: F32 = self.fTempPerm200;
		let mut fSlow253: F32 = ((iSlow216 != 0) as i32) as u32 as F32;
		let mut iTemp221: i32 = self.iTempPerm201;
		let mut fTemp222: F32 = self.fTempPerm202;
		let mut fTemp223: F32 = self.fTempPerm203;
		let mut fTemp224: F32 = self.fTempPerm204;
		let mut fTemp225: F32 = self.fTempPerm205;
		let mut fTemp226: F32 = self.fTempPerm206;
		let mut fTemp227: F32 = self.fTempPerm207;
		let mut fTemp228: F32 = self.fTempPerm208;
		let mut fTemp229: F32 = self.fTempPerm209;
		let mut fTemp230: F32 = self.fTempPerm210;
		let mut fTemp231: F32 = self.fTempPerm211;
		let mut fTemp232: F32 = self.fTempPerm212;
		let mut fTemp233: F32 = self.fTempPerm213;
		let mut fTemp234: F32 = self.fTempPerm214;
		let mut fTemp235: F32 = self.fTempPerm215;
		let mut fTemp236: F32 = self.fTempPerm216;
		let mut fTemp237: F32 = self.fTempPerm217;
		let mut fTemp238: F32 = self.fTempPerm218;
		let mut iTemp239: i32 = self.iTempPerm219;
		let mut fTemp240: F32 = self.fTempPerm220;
		let mut fTemp241: F32 = self.fTempPerm221;
		let mut fSlow254: F32 = ((iSlow32 != 0) as i32) as u32 as F32;
		let mut fTemp242: F32 = self.fTempPerm222;
		let mut fTemp243: F32 = self.fTempPerm223;
		let mut iTemp244: i32 = self.iTempPerm224;
		let mut fTemp245: F32 = self.fTempPerm225;
		let mut iTemp246: i32 = self.iTempPerm226;
		let mut fTemp247: F32 = self.fTempPerm227;
		let mut fTemp248: F32 = self.fTempPerm228;
		let mut fTemp249: F32 = self.fTempPerm229;
		let mut fTemp250: F32 = self.fTempPerm230;
		let mut fTemp251: F32 = self.fTempPerm231;
		let mut fTemp252: F32 = self.fTempPerm232;
		let mut fTemp253: F32 = self.fTempPerm233;
		let mut fTemp254: F32 = self.fTempPerm234;
		let mut fTemp255: F32 = self.fTempPerm235;
		let mut fTemp256: F32 = self.fTempPerm236;
		let mut fTemp257: F32 = self.fTempPerm237;
		let mut fTemp258: F32 = self.fTempPerm238;
		let mut fTemp259: F32 = self.fTempPerm239;
		let mut fSlow255: F32 = ((iSlow35 != 0) as i32) as u32 as F32;
		let mut fSlow256: F32 = unsafe { ftbl4OscDspSIG4[((fSlow7) as i32) as usize] };
		let mut fTemp260: F32 = self.fTempPerm240;
		let mut fSlow257: F32 = ((iSlow4 != 0) as i32) as u32 as F32;
		let mut fSlow258: F32 = self.fHslider14;
		let mut fTemp261: F32 = self.fTempPerm241;
		let mut fTemp262: F32 = self.fTempPerm242;
		let mut iTemp263: i32 = self.iTempPerm243;
		let mut iTemp264: i32 = self.iTempPerm244;
		let mut iTemp265: i32 = self.iTempPerm245;
		let mut fSlow259: F32 = self.fEntry9;
		let mut fSlow260: F32 = self.fEntry10;
		let mut fSlow261: F32 = self.fEntry11;
		let mut iTemp266: i32 = self.iTempPerm246;
		let mut fTemp267: F32 = self.fTempPerm247;
		let mut iTemp268: i32 = self.iTempPerm248;
		let mut iTemp269: i32 = self.iTempPerm249;
		let mut iTemp270: i32 = self.iTempPerm250;
		let mut iTemp271: i32 = self.iTempPerm251;
		let mut iTemp272: i32 = self.iTempPerm252;
		let mut iTemp273: i32 = self.iTempPerm253;
		let mut iTemp274: i32 = self.iTempPerm254;
		let mut fTemp275: F32 = self.fTempPerm255;
		let mut iTemp276: i32 = self.iTempPerm256;
		let mut iTemp277: i32 = self.iTempPerm257;
		let mut fTemp278: F32 = self.fTempPerm258;
		let mut fTemp279: F32 = self.fTempPerm259;
		let mut fTemp280: F32 = self.fTempPerm260;
		let mut fTemp281: F32 = self.fTempPerm261;
		let mut fTemp282: F32 = self.fTempPerm262;
		let mut fSlow262: F32 = self.fEntry12;
		let mut iTemp283: i32 = self.iTempPerm263;
		let mut fTemp284: F32 = self.fTempPerm264;
		let mut fTemp285: F32 = self.fTempPerm265;
		let mut fSlow263: F32 = self.fHslider15;
		let mut fTemp286: F32 = self.fTempPerm266;
		let mut fTemp287: F32 = self.fTempPerm267;
		let mut fTemp288: F32 = self.fTempPerm268;
		let mut fTemp289: F32 = self.fTempPerm269;
		let mut fSlow264: F32 = ((iSlow39 != 0) as i32) as u32 as F32;
		let mut fTemp290: F32 = self.fTempPerm270;
		let mut iTemp291: i32 = self.iTempPerm271;
		let mut iTemp292: i32 = self.iTempPerm272;
		let mut iTemp293: i32 = self.iTempPerm273;
		let mut iTemp294: i32 = self.iTempPerm274;
		let mut iTemp295: i32 = self.iTempPerm275;
		let mut iTemp296: i32 = self.iTempPerm276;
		let mut fTemp297: F32 = self.fTempPerm277;
		let mut fTemp298: F32 = self.fTempPerm278;
		let mut iTemp299: i32 = self.iTempPerm279;
		let mut iTemp300: i32 = self.iTempPerm280;
		let mut iTemp301: i32 = self.iTempPerm281;
		let mut iTemp302: i32 = self.iTempPerm282;
		let mut fTemp303: F32 = self.fTempPerm283;
		let mut fTemp304: F32 = self.fTempPerm284;
		let mut fTemp305: F32 = self.fTempPerm285;
		let mut iTemp306: i32 = self.iTempPerm286;
		let mut fTemp307: F32 = self.fTempPerm287;
		let mut fSlow265: F32 = ((iSlow45 != 0) as i32) as u32 as F32;
		let mut fTemp308: F32 = self.fTempPerm288;
		let mut fSlow266: F32 = ((iSlow48 != 0) as i32) as u32 as F32;
		let mut fTemp309: F32 = self.fTempPerm289;
		let mut fTemp310: F32 = self.fTempPerm290;
		let mut iTemp311: i32 = self.iTempPerm291;
		let mut iTemp312: i32 = self.iTempPerm292;
		let mut iTemp313: i32 = self.iTempPerm293;
		let mut iTemp314: i32 = self.iTempPerm294;
		let mut iTemp315: i32 = self.iTempPerm295;
		let mut iTemp316: i32 = self.iTempPerm296;
		let mut iTemp317: i32 = self.iTempPerm297;
		let mut fTemp318: F32 = self.fTempPerm298;
		let mut iTemp319: i32 = self.iTempPerm299;
		let mut iTemp320: i32 = self.iTempPerm300;
		let mut fTemp321: F32 = self.fTempPerm301;
		let mut fTemp322: F32 = self.fTempPerm302;
		let mut fTemp323: F32 = self.fTempPerm303;
		let mut fTemp324: F32 = self.fTempPerm304;
		let mut fSlow267: F32 = ((iSlow51 != 0) as i32) as u32 as F32;
		let mut fTemp325: F32 = self.fTempPerm305;
		let mut fSlow268: F32 = ((iSlow54 != 0) as i32) as u32 as F32;
		let mut fTemp326: F32 = self.fTempPerm306;
		let mut fTemp327: F32 = self.fTempPerm307;
		let mut fSlow269: F32 = ((iSlow57 != 0) as i32) as u32 as F32;
		let mut fTemp328: F32 = self.fTempPerm308;
		let mut iTemp329: i32 = self.iTempPerm309;
		let mut iTemp330: i32 = self.iTempPerm310;
		let mut fTemp331: F32 = self.fTempPerm311;
		let mut fTemp332: F32 = self.fTempPerm312;
		let mut fTemp333: F32 = self.fTempPerm313;
		let mut fTemp334: F32 = self.fTempPerm314;
		let mut fSlow270: F32 = ((iSlow60 != 0) as i32) as u32 as F32;
		let mut fSlow271: F32 = F32::max(0.0, F32::min(2.0, fSlow38 + -5.0));
		let mut fSlow272: F32 = self.fConst24 * (if (fSlow271 >= 2.0) as i32 != 0 {800} else {if (fSlow271 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp335: F32 = self.fTempPerm315;
		let mut fTemp336: F32 = self.fTempPerm316;
		let mut fTemp337: F32 = self.fTempPerm317;
		let mut fTemp338: F32 = self.fTempPerm318;
		let mut fTemp339: F32 = self.fTempPerm319;
		let mut fSlow273: F32 = ((iSlow63 != 0) as i32) as u32 as F32;
		let mut fTemp340: F32 = self.fTempPerm320;
		let mut fSlow274: F32 = ((iSlow66 != 0) as i32) as u32 as F32;
		let mut fTemp341: F32 = self.fTempPerm321;
		let mut fSlow275: F32 = ((iSlow72 != 0) as i32) as u32 as F32;
		let mut fSlow276: F32 = F32::max(0.0, F32::min(2.0, fSlow38 + -2.0));
		let mut fSlow277: F32 = self.fConst2 * (if (fSlow276 >= 2.0) as i32 != 0 {4000} else {if (fSlow276 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp342: F32 = self.fTempPerm322;
		let mut fTemp343: F32 = self.fTempPerm323;
		let mut fSlow278: F32 = ((iSlow75 != 0) as i32) as u32 as F32;
		let mut fTemp344: F32 = self.fTempPerm324;
		let mut fSlow279: F32 = ((iSlow78 != 0) as i32) as u32 as F32;
		let mut fTemp345: F32 = self.fTempPerm325;
		let mut fSlow280: F32 = ((iSlow81 != 0) as i32) as u32 as F32;
		let mut fTemp346: F32 = self.fTempPerm326;
		let mut fTemp347: F32 = self.fTempPerm327;
		let mut fSlow281: F32 = ((iSlow69 != 0) as i32) as u32 as F32;
		let mut fTemp348: F32 = self.fTempPerm328;
		let mut fSlow282: F32 = ((iSlow84 != 0) as i32) as u32 as F32;
		let mut fTemp349: F32 = self.fTempPerm329;
		let mut fSlow283: F32 = ((iSlow42 != 0) as i32) as u32 as F32;
		let mut fTemp350: F32 = self.fTempPerm330;
		let mut fSlow284: F32 = ((iSlow0 != 0) as i32) as u32 as F32;
		let mut fSlow285: F32 = self.fHslider16;
		let mut fTemp355: F32 = self.fTempPerm331;
		let mut fTemp356: F32 = self.fTempPerm332;
		let mut iTemp357: i32 = self.iTempPerm333;
		let mut fSlow286: F32 = self.fHslider17;
		let mut fTemp358: F32 = self.fTempPerm334;
		let mut fTemp359: F32 = self.fTempPerm335;
		let mut fSlow287: F32 = self.fHslider18;
		let mut fTemp360: F32 = self.fTempPerm336;
		let mut fTemp361: F32 = self.fTempPerm337;
		let mut fTemp362: F32 = self.fTempPerm338;
		let mut fTemp363: F32 = self.fTempPerm339;
		let mut fTemp364: F32 = self.fTempPerm340;
		let mut fTemp365: F32 = self.fTempPerm341;
		let mut fTemp366: F32 = self.fTempPerm342;
		let mut fTemp367: F32 = self.fTempPerm343;
		let mut fSlow288: F32 = self.fEntry13;
		let mut fSlow289: F32 = self.fEntry14;
		let mut fTemp368: F32 = self.fTempPerm344;
		let mut fTemp369: F32 = self.fTempPerm345;
		let mut fTemp370: F32 = self.fTempPerm346;
		let mut iTemp371: i32 = self.iTempPerm347;
		let mut fTemp372: F32 = self.fTempPerm348;
		let mut fTemp373: F32 = self.fTempPerm349;
		let mut fTemp374: F32 = self.fTempPerm350;
		let mut fTemp375: F32 = self.fTempPerm351;
		let mut fTemp376: F32 = self.fTempPerm352;
		let mut fTemp377: F32 = self.fTempPerm353;
		let mut fTemp378: F32 = self.fTempPerm354;
		let mut fTemp379: F32 = self.fTempPerm355;
		let mut fTemp380: F32 = self.fTempPerm356;
		let mut fTemp381: F32 = self.fTempPerm357;
		let mut fTemp382: F32 = self.fTempPerm358;
		let mut fTemp383: F32 = self.fTempPerm359;
		let mut fTemp384: F32 = self.fTempPerm360;
		let mut fTemp385: F32 = self.fTempPerm361;
		let mut fTemp386: F32 = self.fTempPerm362;
		let mut fTemp387: F32 = self.fTempPerm363;
		let mut fSlow290: F32 = ((iSlow138 != 0) as i32) as u32 as F32;
		let mut iSlow291: i32 = (fSlow288) as i32;
		let mut fTemp388: F32 = self.fTempPerm364;
		let mut fTemp389: F32 = self.fTempPerm365;
		let mut fTemp390: F32 = self.fTempPerm366;
		let mut fTemp391: F32 = self.fTempPerm367;
		let mut fTemp392: F32 = self.fTempPerm368;
		let mut fTemp393: F32 = self.fTempPerm369;
		let mut fTemp394: F32 = self.fTempPerm370;
		let mut fTemp395: F32 = self.fTempPerm371;
		let mut fTemp396: F32 = self.fTempPerm372;
		let mut fSlow292: F32 = ((iSlow144 != 0) as i32) as u32 as F32;
		let mut iTemp397: i32 = self.iTempPerm373;
		let mut iSlow293: i32 = (fSlow289) as i32;
		let mut fTemp398: F32 = self.fTempPerm374;
		let mut fTemp399: F32 = self.fTempPerm375;
		let mut fTemp400: F32 = self.fTempPerm376;
		let mut fTemp401: F32 = self.fTempPerm377;
		let mut fTemp402: F32 = self.fTempPerm378;
		let mut fTemp403: F32 = self.fTempPerm379;
		let mut fTemp404: F32 = self.fTempPerm380;
		let mut fTemp405: F32 = self.fTempPerm381;
		let mut fTemp406: F32 = self.fTempPerm382;
		let mut iSlow294: i32 = (fSlow289 >= 2.0) as i32;
		let mut iSlow295: i32 = (fSlow289 >= 1.0) as i32;
		let mut fTemp407: F32 = self.fTempPerm383;
		let mut iSlow296: i32 = (fSlow289 >= 3.0) as i32;
		let mut fSlow297: F32 = (if iSlow296 != 0 {0} else {2}) as F32;
		let mut fSlow298: F32 = if iSlow294 != 0 {fSlow297} else {if iSlow295 != 0 {4.0} else {6.2831855}};
		let mut fTemp408: F32 = self.fTempPerm384;
		let mut fTemp409: F32 = self.fTempPerm385;
		let mut fTemp410: F32 = self.fTempPerm386;
		let mut fTemp411: F32 = self.fTempPerm387;
		let mut fTemp412: F32 = self.fTempPerm388;
		let mut fTemp413: F32 = self.fTempPerm389;
		let mut fSlow299: F32 = ((iSlow147 != 0) as i32) as u32 as F32;
		let mut fTemp414: F32 = self.fTempPerm390;
		let mut fTemp415: F32 = self.fTempPerm391;
		let mut fTemp416: F32 = self.fTempPerm392;
		let mut fTemp417: F32 = self.fTempPerm393;
		let mut fTemp418: F32 = self.fTempPerm394;
		let mut iTemp419: i32 = self.iTempPerm395;
		let mut fTemp420: F32 = self.fTempPerm396;
		let mut fTemp421: F32 = self.fTempPerm397;
		let mut fTemp422: F32 = self.fTempPerm398;
		let mut fTemp423: F32 = self.fTempPerm399;
		let mut fTemp424: F32 = self.fTempPerm400;
		let mut iTemp425: i32 = self.iTempPerm401;
		let mut fTemp426: F32 = self.fTempPerm402;
		let mut fTemp427: F32 = self.fTempPerm403;
		let mut fTemp428: F32 = self.fTempPerm404;
		let mut fTemp429: F32 = self.fTempPerm405;
		let mut fTemp430: F32 = self.fTempPerm406;
		let mut fTemp431: F32 = self.fTempPerm407;
		let mut fSlow300: F32 = ((iSlow150 != 0) as i32) as u32 as F32;
		let mut fTemp432: F32 = self.fTempPerm408;
		let mut iTemp433: i32 = self.iTempPerm409;
		let mut iTemp434: i32 = self.iTempPerm410;
		let mut iTemp435: i32 = self.iTempPerm411;
		let mut iTemp436: i32 = self.iTempPerm412;
		let mut iTemp437: i32 = self.iTempPerm413;
		let mut iTemp438: i32 = self.iTempPerm414;
		let mut fTemp439: F32 = self.fTempPerm415;
		let mut fTemp440: F32 = self.fTempPerm416;
		let mut fTemp441: F32 = self.fTempPerm417;
		let mut iTemp442: i32 = self.iTempPerm418;
		let mut iTemp443: i32 = self.iTempPerm419;
		let mut iTemp444: i32 = self.iTempPerm420;
		let mut iTemp445: i32 = self.iTempPerm421;
		let mut fTemp446: F32 = self.fTempPerm422;
		let mut fTemp447: F32 = self.fTempPerm423;
		let mut fTemp448: F32 = self.fTempPerm424;
		let mut fTemp449: F32 = self.fTempPerm425;
		let mut fTemp450: F32 = self.fTempPerm426;
		let mut fTemp451: F32 = self.fTempPerm427;
		let mut fTemp452: F32 = self.fTempPerm428;
		let mut fTemp453: F32 = self.fTempPerm429;
		let mut fTemp454: F32 = self.fTempPerm430;
		let mut fSlow301: F32 = ((iSlow153 != 0) as i32) as u32 as F32;
		let mut fTemp455: F32 = self.fTempPerm431;
		let mut fTemp456: F32 = self.fTempPerm432;
		let mut fTemp457: F32 = self.fTempPerm433;
		let mut fTemp458: F32 = self.fTempPerm434;
		let mut fTemp459: F32 = self.fTempPerm435;
		let mut fTemp460: F32 = self.fTempPerm436;
		let mut fSlow302: F32 = ((iSlow156 != 0) as i32) as u32 as F32;
		let mut iTemp461: i32 = self.iTempPerm437;
		let mut fTemp462: F32 = self.fTempPerm438;
		let mut fTemp463: F32 = self.fTempPerm439;
		let mut fTemp464: F32 = self.fTempPerm440;
		let mut fTemp465: F32 = self.fTempPerm441;
		let mut fTemp466: F32 = self.fTempPerm442;
		let mut fTemp467: F32 = self.fTempPerm443;
		let mut fTemp468: F32 = self.fTempPerm444;
		let mut fTemp469: F32 = self.fTempPerm445;
		let mut fTemp470: F32 = self.fTempPerm446;
		let mut fTemp471: F32 = self.fTempPerm447;
		let mut fTemp472: F32 = self.fTempPerm448;
		let mut fSlow303: F32 = ((iSlow159 != 0) as i32) as u32 as F32;
		let mut fTemp473: F32 = self.fTempPerm449;
		let mut fTemp474: F32 = self.fTempPerm450;
		let mut fTemp475: F32 = self.fTempPerm451;
		let mut iTemp476: i32 = self.iTempPerm452;
		let mut iSlow304: i32 = (fSlow288 >= 2.0) as i32;
		let mut iSlow305: i32 = (fSlow288 >= 3.0) as i32;
		let mut iSlow306: i32 = if iSlow304 != 0 {if iSlow305 != 0 {-2} else {0}} else {0};
		let mut iSlow307: i32 = (fSlow288 >= 1.0) as i32;
		let mut fSlow308: F32 = if iSlow304 != 0 {if iSlow305 != 0 {0.5} else {0.0}} else {if iSlow307 != 0 {0.75} else {0.0}};
		let mut fTemp477: F32 = self.fTempPerm453;
		let mut fTemp478: F32 = self.fTempPerm454;
		let mut fTemp479: F32 = self.fTempPerm455;
		let mut fTemp480: F32 = self.fTempPerm456;
		let mut iTemp481: i32 = self.iTempPerm457;
		let mut iSlow309: i32 = if iSlow304 != 0 {if iSlow305 != 0 {2} else {-2}} else {0};
		let mut fSlow310: F32 = if iSlow304 != 0 {if iSlow305 != 0 {0.0} else {0.5}} else {if iSlow307 != 0 {0.25} else {0.0}};
		let mut fTemp482: F32 = self.fTempPerm458;
		let mut iTemp483: i32 = self.iTempPerm459;
		let mut fTemp484: F32 = self.fTempPerm460;
		let mut fTemp485: F32 = self.fTempPerm461;
		let mut fTemp486: F32 = self.fTempPerm462;
		let mut fTemp487: F32 = self.fTempPerm463;
		let mut fTemp488: F32 = self.fTempPerm464;
		let mut iTemp489: i32 = self.iTempPerm465;
		let mut iSlow311: i32 = if iSlow294 != 0 {if iSlow296 != 0 {-2} else {0}} else {0};
		let mut fTemp490: F32 = self.fTempPerm466;
		let mut fTemp491: F32 = self.fTempPerm467;
		let mut fSlow312: F32 = if iSlow294 != 0 {if iSlow296 != 0 {0.5} else {0.0}} else {if iSlow295 != 0 {0.75} else {0.0}};
		let mut fTemp492: F32 = self.fTempPerm468;
		let mut fTemp493: F32 = self.fTempPerm469;
		let mut fTemp494: F32 = self.fTempPerm470;
		let mut fTemp495: F32 = self.fTempPerm471;
		let mut fSlow313: F32 = (if iSlow294 != 0 {0} else {if iSlow295 != 0 {8} else {0}}) as F32;
		let mut fTemp496: F32 = self.fTempPerm472;
		let mut iSlow314: i32 = if iSlow294 != 0 {if iSlow296 != 0 {2} else {-2}} else {0};
		let mut fTemp497: F32 = self.fTempPerm473;
		let mut fSlow315: F32 = if iSlow294 != 0 {if iSlow296 != 0 {0.0} else {0.5}} else {if iSlow295 != 0 {0.25} else {0.0}};
		let mut fTemp498: F32 = self.fTempPerm474;
		let mut fTemp499: F32 = self.fTempPerm475;
		let mut fTemp500: F32 = self.fTempPerm476;
		let mut fSlow316: F32 = (if iSlow294 != 0 {0} else {if iSlow295 != 0 {-8} else {0}}) as F32;
		let mut fTemp501: F32 = self.fTempPerm477;
		let mut fTemp502: F32 = self.fTempPerm478;
		let mut fTemp503: F32 = self.fTempPerm479;
		let mut fTemp504: F32 = self.fTempPerm480;
		let mut fSlow317: F32 = ((iSlow162 != 0) as i32) as u32 as F32;
		let mut fTemp505: F32 = self.fTempPerm481;
		let mut fTemp506: F32 = self.fTempPerm482;
		let mut fTemp507: F32 = self.fTempPerm483;
		let mut fSlow318: F32 = ((iSlow165 != 0) as i32) as u32 as F32;
		let mut fTemp508: F32 = self.fTempPerm484;
		let mut fTemp509: F32 = self.fTempPerm485;
		let mut fTemp510: F32 = self.fTempPerm486;
		let mut iTemp511: i32 = self.iTempPerm487;
		let mut fTemp512: F32 = self.fTempPerm488;
		let mut iTemp513: i32 = self.iTempPerm489;
		let mut fTemp514: F32 = self.fTempPerm490;
		let mut iTemp515: i32 = self.iTempPerm491;
		let mut fTemp516: F32 = self.fTempPerm492;
		let mut iTemp517: i32 = self.iTempPerm493;
		let mut fTemp518: F32 = self.fTempPerm494;
		let mut fTemp519: F32 = self.fTempPerm495;
		let mut fTemp520: F32 = self.fTempPerm496;
		let mut fTemp521: F32 = self.fTempPerm497;
		let mut iTemp522: i32 = self.iTempPerm498;
		let mut fTemp523: F32 = self.fTempPerm499;
		let mut fTemp524: F32 = self.fTempPerm500;
		let mut fTemp525: F32 = self.fTempPerm501;
		let mut fTemp526: F32 = self.fTempPerm502;
		let mut fTemp527: F32 = self.fTempPerm503;
		let mut fTemp528: F32 = self.fTempPerm504;
		let mut fTemp529: F32 = self.fTempPerm505;
		let mut fTemp530: F32 = self.fTempPerm506;
		let mut fTemp531: F32 = self.fTempPerm507;
		let mut fTemp532: F32 = self.fTempPerm508;
		let mut fTemp533: F32 = self.fTempPerm509;
		let mut fTemp534: F32 = self.fTempPerm510;
		let mut fTemp535: F32 = self.fTempPerm511;
		let mut fTemp536: F32 = self.fTempPerm512;
		let mut fTemp537: F32 = self.fTempPerm513;
		let mut fTemp538: F32 = self.fTempPerm514;
		let mut fTemp539: F32 = self.fTempPerm515;
		let mut fSlow319: F32 = ((iSlow168 != 0) as i32) as u32 as F32;
		let mut iTemp540: i32 = self.iTempPerm516;
		let mut fTemp541: F32 = self.fTempPerm517;
		let mut fTemp542: F32 = self.fTempPerm518;
		let mut fTemp543: F32 = self.fTempPerm519;
		let mut fTemp544: F32 = self.fTempPerm520;
		let mut fTemp545: F32 = self.fTempPerm521;
		let mut fTemp546: F32 = self.fTempPerm522;
		let mut fTemp547: F32 = self.fTempPerm523;
		let mut fTemp548: F32 = self.fTempPerm524;
		let mut fTemp549: F32 = self.fTempPerm525;
		let mut fTemp550: F32 = self.fTempPerm526;
		let mut fTemp551: F32 = self.fTempPerm527;
		let mut fTemp552: F32 = self.fTempPerm528;
		let mut fTemp553: F32 = self.fTempPerm529;
		let mut fTemp554: F32 = self.fTempPerm530;
		let mut fTemp555: F32 = self.fTempPerm531;
		let mut fTemp556: F32 = self.fTempPerm532;
		let mut fTemp557: F32 = self.fTempPerm533;
		let mut iTemp558: i32 = self.iTempPerm534;
		let mut fTemp559: F32 = self.fTempPerm535;
		let mut fTemp560: F32 = self.fTempPerm536;
		let mut fSlow320: F32 = ((iSlow171 != 0) as i32) as u32 as F32;
		let mut fTemp561: F32 = self.fTempPerm537;
		let mut fTemp562: F32 = self.fTempPerm538;
		let mut iTemp563: i32 = self.iTempPerm539;
		let mut fTemp564: F32 = self.fTempPerm540;
		let mut iTemp565: i32 = self.iTempPerm541;
		let mut fTemp566: F32 = self.fTempPerm542;
		let mut fTemp567: F32 = self.fTempPerm543;
		let mut fTemp568: F32 = self.fTempPerm544;
		let mut fTemp569: F32 = self.fTempPerm545;
		let mut fTemp570: F32 = self.fTempPerm546;
		let mut fTemp571: F32 = self.fTempPerm547;
		let mut fTemp572: F32 = self.fTempPerm548;
		let mut fSlow321: F32 = ((iSlow174 != 0) as i32) as u32 as F32;
		let mut fSlow322: F32 = unsafe { ftbl4OscDspSIG4[((fSlow137) as i32) as usize] };
		let mut fTemp573: F32 = self.fTempPerm549;
		let mut fSlow323: F32 = ((iSlow141 != 0) as i32) as u32 as F32;
		let mut fSlow324: F32 = self.fHslider19;
		let mut fTemp574: F32 = self.fTempPerm550;
		let mut fTemp575: F32 = self.fTempPerm551;
		let mut iTemp576: i32 = self.iTempPerm552;
		let mut iTemp577: i32 = self.iTempPerm553;
		let mut iTemp578: i32 = self.iTempPerm554;
		let mut fSlow325: F32 = self.fEntry15;
		let mut fSlow326: F32 = self.fEntry16;
		let mut fSlow327: F32 = self.fEntry17;
		let mut iTemp579: i32 = self.iTempPerm555;
		let mut fTemp580: F32 = self.fTempPerm556;
		let mut iTemp581: i32 = self.iTempPerm557;
		let mut iTemp582: i32 = self.iTempPerm558;
		let mut iTemp583: i32 = self.iTempPerm559;
		let mut iTemp584: i32 = self.iTempPerm560;
		let mut iTemp585: i32 = self.iTempPerm561;
		let mut iTemp586: i32 = self.iTempPerm562;
		let mut iTemp587: i32 = self.iTempPerm563;
		let mut fTemp588: F32 = self.fTempPerm564;
		let mut iTemp589: i32 = self.iTempPerm565;
		let mut iTemp590: i32 = self.iTempPerm566;
		let mut fTemp591: F32 = self.fTempPerm567;
		let mut fTemp592: F32 = self.fTempPerm568;
		let mut fTemp593: F32 = self.fTempPerm569;
		let mut fTemp594: F32 = self.fTempPerm570;
		let mut fTemp595: F32 = self.fTempPerm571;
		let mut fSlow328: F32 = self.fEntry18;
		let mut iTemp596: i32 = self.iTempPerm572;
		let mut fTemp597: F32 = self.fTempPerm573;
		let mut fTemp598: F32 = self.fTempPerm574;
		let mut fSlow329: F32 = self.fHslider20;
		let mut fTemp599: F32 = self.fTempPerm575;
		let mut fTemp600: F32 = self.fTempPerm576;
		let mut fTemp601: F32 = self.fTempPerm577;
		let mut fTemp602: F32 = self.fTempPerm578;
		let mut fSlow330: F32 = ((iSlow177 != 0) as i32) as u32 as F32;
		let mut fTemp603: F32 = self.fTempPerm579;
		let mut iTemp604: i32 = self.iTempPerm580;
		let mut iTemp605: i32 = self.iTempPerm581;
		let mut iTemp606: i32 = self.iTempPerm582;
		let mut iTemp607: i32 = self.iTempPerm583;
		let mut iTemp608: i32 = self.iTempPerm584;
		let mut iTemp609: i32 = self.iTempPerm585;
		let mut fTemp610: F32 = self.fTempPerm586;
		let mut fTemp611: F32 = self.fTempPerm587;
		let mut iTemp612: i32 = self.iTempPerm588;
		let mut iTemp613: i32 = self.iTempPerm589;
		let mut iTemp614: i32 = self.iTempPerm590;
		let mut iTemp615: i32 = self.iTempPerm591;
		let mut fTemp616: F32 = self.fTempPerm592;
		let mut fTemp617: F32 = self.fTempPerm593;
		let mut fTemp618: F32 = self.fTempPerm594;
		let mut iTemp619: i32 = self.iTempPerm595;
		let mut fTemp620: F32 = self.fTempPerm596;
		let mut fSlow331: F32 = ((iSlow180 != 0) as i32) as u32 as F32;
		let mut fTemp621: F32 = self.fTempPerm597;
		let mut fSlow332: F32 = ((iSlow183 != 0) as i32) as u32 as F32;
		let mut fTemp622: F32 = self.fTempPerm598;
		let mut fTemp623: F32 = self.fTempPerm599;
		let mut iTemp624: i32 = self.iTempPerm600;
		let mut iTemp625: i32 = self.iTempPerm601;
		let mut iTemp626: i32 = self.iTempPerm602;
		let mut iTemp627: i32 = self.iTempPerm603;
		let mut iTemp628: i32 = self.iTempPerm604;
		let mut iTemp629: i32 = self.iTempPerm605;
		let mut iTemp630: i32 = self.iTempPerm606;
		let mut fTemp631: F32 = self.fTempPerm607;
		let mut iTemp632: i32 = self.iTempPerm608;
		let mut iTemp633: i32 = self.iTempPerm609;
		let mut fTemp634: F32 = self.fTempPerm610;
		let mut fTemp635: F32 = self.fTempPerm611;
		let mut fTemp636: F32 = self.fTempPerm612;
		let mut fTemp637: F32 = self.fTempPerm613;
		let mut fSlow333: F32 = ((iSlow186 != 0) as i32) as u32 as F32;
		let mut fTemp638: F32 = self.fTempPerm614;
		let mut fSlow334: F32 = ((iSlow189 != 0) as i32) as u32 as F32;
		let mut fTemp639: F32 = self.fTempPerm615;
		let mut fTemp640: F32 = self.fTempPerm616;
		let mut fSlow335: F32 = ((iSlow192 != 0) as i32) as u32 as F32;
		let mut fTemp641: F32 = self.fTempPerm617;
		let mut iTemp642: i32 = self.iTempPerm618;
		let mut iTemp643: i32 = self.iTempPerm619;
		let mut fTemp644: F32 = self.fTempPerm620;
		let mut fTemp645: F32 = self.fTempPerm621;
		let mut fTemp646: F32 = self.fTempPerm622;
		let mut fTemp647: F32 = self.fTempPerm623;
		let mut fSlow336: F32 = ((iSlow195 != 0) as i32) as u32 as F32;
		let mut fSlow337: F32 = F32::max(0.0, F32::min(2.0, fSlow91 + -5.0));
		let mut fSlow338: F32 = self.fConst24 * (if (fSlow337 >= 2.0) as i32 != 0 {800} else {if (fSlow337 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp648: F32 = self.fTempPerm624;
		let mut fTemp649: F32 = self.fTempPerm625;
		let mut fTemp650: F32 = self.fTempPerm626;
		let mut fTemp651: F32 = self.fTempPerm627;
		let mut fTemp652: F32 = self.fTempPerm628;
		let mut fSlow339: F32 = ((iSlow198 != 0) as i32) as u32 as F32;
		let mut fTemp653: F32 = self.fTempPerm629;
		let mut fSlow340: F32 = ((iSlow201 != 0) as i32) as u32 as F32;
		let mut fTemp654: F32 = self.fTempPerm630;
		let mut fSlow341: F32 = ((iSlow204 != 0) as i32) as u32 as F32;
		let mut fSlow342: F32 = F32::max(0.0, F32::min(2.0, fSlow91 + -2.0));
		let mut fSlow343: F32 = self.fConst2 * (if (fSlow342 >= 2.0) as i32 != 0 {4000} else {if (fSlow342 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp655: F32 = self.fTempPerm631;
		let mut fTemp656: F32 = self.fTempPerm632;
		let mut fSlow344: F32 = ((iSlow92 != 0) as i32) as u32 as F32;
		let mut fTemp657: F32 = self.fTempPerm633;
		let mut fSlow345: F32 = ((iSlow102 != 0) as i32) as u32 as F32;
		let mut fTemp658: F32 = self.fTempPerm634;
		let mut fSlow346: F32 = ((iSlow105 != 0) as i32) as u32 as F32;
		let mut fTemp659: F32 = self.fTempPerm635;
		let mut fTemp660: F32 = self.fTempPerm636;
		let mut fSlow347: F32 = ((iSlow207 != 0) as i32) as u32 as F32;
		let mut fTemp661: F32 = self.fTempPerm637;
		let mut fSlow348: F32 = ((iSlow210 != 0) as i32) as u32 as F32;
		let mut fTemp662: F32 = self.fTempPerm638;
		let mut fSlow349: F32 = ((iSlow96 != 0) as i32) as u32 as F32;
		let mut fTemp663: F32 = self.fTempPerm639;
		let mut fSlow350: F32 = ((iSlow99 != 0) as i32) as u32 as F32;
		let mut fSlow351: F32 = self.fHslider21;
		let mut fSlow352: F32 = self.fHslider22;
		let mut fSlow353: F32 = self.fEntry36 + self.fEntry35 + self.fCheckbox9 + self.fHslider81 + self.fHslider80 + self.fHslider79 + self.fHslider78 + self.fHslider77 + self.fHslider76 + self.fHslider75 + self.fHslider74 + self.fEntry34 + self.fEntry33 + self.fCheckbox8 + self.fHslider73 + self.fHslider72 + self.fHslider71 + self.fHslider70 + self.fHslider69 + self.fHslider68 + self.fHslider67 + self.fHslider66 + self.fEntry32 + self.fEntry31 + self.fHslider65 + self.fHslider64 + self.fHslider63 + self.fHslider62 + self.fHslider61 + self.fHslider60 + self.fHslider59 + self.fEntry30 + self.fEntry29 + self.fHslider58 + self.fHslider57 + self.fHslider56 + self.fHslider55 + self.fHslider54 + self.fHslider53 + self.fHslider52 + self.fEntry28 + self.fCheckbox7 + self.fEntry27 + self.fHslider51 + self.fHslider50 + self.fHslider49 + self.fHslider48 + self.fCheckbox6 + self.fEntry26 + self.fHslider47 + self.fHslider46 + self.fHslider45 + self.fHslider44 + self.fCheckbox5 + self.fEntry25 + self.fHslider43 + self.fHslider42 + self.fHslider41 + self.fHslider40 + self.fEntry24 + self.fCheckbox4 + self.fEntry23 + self.fHslider39 + self.fHslider38 + self.fHslider37 + self.fHslider36 + self.fCheckbox3 + self.fEntry22 + self.fHslider35 + self.fHslider34 + self.fHslider33 + self.fHslider32 + self.fCheckbox2 + self.fEntry21 + self.fHslider31 + self.fHslider30 + self.fHslider29 + self.fHslider28 + self.fHslider27 + self.fEntry20 + self.fHslider26 + self.fHslider25 + self.fHslider24 + self.fHslider23 + self.fCheckbox1 + self.fEntry19;
		let zipped_iterators = outputs0.zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5);
		for (((((output0, output1), output2), output3), output4), output5) in zipped_iterators {
			self.fVec0[0] = fSlow87;
			self.iVec1[0] = 1;
			let mut iTemp0: i32 = (fSlow87 > self.fVec0[1]) as i32;
			self.fVec2[0] = fSlow88;
			let mut fTemp1: F32 = if ((fSlow88 != self.fVec2[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec1[1] + -1.0};
			self.fRec1[0] = fTemp1;
			self.fRec2[0] = if iTemp0 != 0 {fSlow88} else {if (fTemp1 > 0.0) as i32 != 0 {self.fRec2[1] + (fSlow88 - self.fRec2[1]) / F32::max(1.0, fTemp1)} else {fSlow88}};
			let mut fTemp2: F32 = F32::abs(self.fRec2[0]);
			let mut fTemp3: F32 = F32::exp(0.057762265 * (((self.fRec2[0] > 0.0) as i32) - ((self.fRec2[0] < 0.0) as i32)) as F32 * if (fTemp2 < 0.4) as i32 != 0 {F32::min(0.99, 2.5 * fTemp2)} else {if (fTemp2 < 0.82) as i32 != 0 {F32::min(6.0, F32::floor(16.666666 * (fTemp2 + -0.4))) + 1.0} else {(if (fTemp2 < 0.91) as i32 != 0 {12} else {24}) as F32}});
			self.fVec3[0] = fSlow90;
			let mut fTemp4: F32 = if ((fSlow90 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec3[1] + -1.0};
			self.fRec3[0] = fTemp4;
			self.fRec4[0] = if iTemp0 != 0 {fSlow90} else {if (fTemp4 > 0.0) as i32 != 0 {self.fRec4[1] + (fSlow90 - self.fRec4[1]) / F32::max(1.0, fTemp4)} else {fSlow90}};
			let mut iTemp5: i32 = (F32::abs(self.fRec4[0]) > 0.0001) as i32;
			let mut fTemp6: F32 = (iTemp5) as F32;
			if ((fTemp6 != 0.0) as i32) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | (iSlow101 & (iSlow98 & iSlow94))))))) != 0 {
				iTemp7 = i32::wrapping_sub(1, self.iVec1[1]);
			}
				if (fTemp6 != 0.0) as i32 != 0 {
					fTemp8 = if iTemp7 != 0 {0.0} else {self.fConst1 + self.fRec6[1]};
					self.fRec6[0] = fTemp8 - F32::floor(fTemp8);
					fTemp9 = if iTemp7 != 0 {0.0} else {fSlow113 + self.fRec7[1]};
					self.fRec7[0] = fTemp9 - F32::floor(fTemp9);
					self.iRec9[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec9[1], i32::wrapping_mul(iSlow114, iTemp7))), 12345);
					iTemp10 = i32::wrapping_mul(1664525, self.iRec9[0] ^ (self.iRec9[0] >> 15));
					self.fRec8[0] = self.fConst3 * self.fRec8[1] + self.fConst5 * (i32::wrapping_mul(1013904223, iTemp10 ^ (iTemp10 >> 13))) as F32;
				}
					let mut fTemp11: F32 = (i32::wrapping_sub(1, iTemp0)) as F32;
					let mut fTemp12: F32 = 1.0 - self.fConst6 * fTemp11;
					if (fTemp6 != 0.0) as i32 != 0 {
						self.fRec10[0] = fSlow115 * fTemp12 + self.fConst6 * fTemp11 * self.fRec10[1];
						fTemp13 = if iTemp7 != 0 {0.0} else {fSlow120 + self.fRec13[1]};
						self.fRec13[0] = fTemp13 - F32::floor(fTemp13);
					}
						self.iRec14[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec14[1]), 12345);
						let mut fTemp14: F32 = (self.iRec14[0]) as F32;
						self.fVec5[0] = fTemp14;
						let mut fTemp15: F32 = 4.656613e-10 * fTemp14;
						if (fTemp6 != 0.0) as i32 != 0 {
							self.fRec12[0] = if (self.fRec13[0] < self.fRec13[1]) as i32 != 0 {fTemp15} else {self.fRec12[1]};
							fTemp16 = F32::floor(32768.0 * self.fRec12[0] + 0.5);
							self.fVec6[0] = fTemp16;
							self.fRec11[0] = fSlow121 * (3.0517578e-05 * (fTemp16 + self.fVec6[1]) - fSlow119 * self.fRec11[1]);
							fTemp17 = fTemp6 * (self.fRec4[0] * (fSlow124 + 2e+01 * self.fRec11[0] + self.fConst7 * self.fRec10[0] * self.fRec8[0]) + OscDsp_faustpower2_f(self.fRec4[0]) * (18.0 * unsafe { ftbl0OscDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec7[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl0OscDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec6[0]) as i32, 65535))) as usize] }));
						}
							self.fRec15[0] = fSlow125 * fTemp12 + self.fConst6 * fTemp11 * self.fRec15[1];
							self.fVec7[0] = fSlow128;
							let mut fTemp18: F32 = if ((fSlow128 != self.fVec7[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec16[1] + -1.0};
							self.fRec16[0] = fTemp18;
							self.fRec17[0] = if iTemp0 != 0 {fSlow128} else {if (fTemp18 > 0.0) as i32 != 0 {self.fRec17[1] + (fSlow128 - self.fRec17[1]) / F32::max(1.0, fTemp18)} else {fSlow128}};
							let mut fTemp19: F32 = F32::exp(0.057762265 * (self.fRec17[0] + fSlow127 * self.fRec15[0] + 0.01 * ((iTemp5 != 0) as i32) as u32 as F32 * fTemp17));
							self.fVec8[0] = fSlow129;
							let mut fTemp20: F32 = if ((fSlow129 != self.fVec8[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec18[1] + -1.0};
							self.fRec18[0] = fTemp20;
							self.fRec19[0] = if iTemp0 != 0 {fSlow129} else {if (fTemp20 > 0.0) as i32 != 0 {self.fRec19[1] + (fSlow129 - self.fRec19[1]) / F32::max(1.0, fTemp20)} else {fSlow129}};
							self.fRec20[0] = fSlow133 + fSlow132 * self.fRec20[1];
							let mut fTemp21: F32 = F32::exp(self.fRec20[0]) * self.fRec19[0];
							let mut fTemp22: F32 = fTemp21 * fTemp19;
							let mut fTemp23: F32 = fTemp22 * fTemp3;
							if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow44 & iSlow41)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10))))))))))))))))))))))))) != 0 {
								fTemp24 = self.fConst2 * fTemp23;
								fTemp25 = fTemp24 + self.fRec0[1];
							}
								self.fRec21[0] = if iTemp0 != 0 {fTemp15} else {self.fRec21[1]};
								let mut fTemp26: F32 = 0.5 * (self.fRec21[0] + 1.0);
								let mut fTemp27: F32 = fSlow135 + (fTemp26 - F32::floor(fSlow135 + fTemp26));
								let mut fTemp28: F32 = fSlow136 * fTemp27;
								if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow44 & iSlow41)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow218)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow215)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | ((iSlow2 & (iSlow6 & iSlow10)) | ((iSlow101 & (iSlow98 & iSlow212)) | ((iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow188)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | ((iSlow101 & (iSlow98 & iSlow179)) | ((iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow173)) | ((iSlow101 & (iSlow143 & iSlow170)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow164)) | ((iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | ((iSlow101 & (iSlow143 & iSlow149)) | ((iSlow101 & (iSlow143 & iSlow146)) | (iSlow101 & (iSlow143 & iSlow140)))))))))))))))))))))))))))))))))))))))))))))))))))))) != 0 {
									fTemp29 = fTemp28 - F32::floor(fTemp28);
								}
									if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow44 & iSlow41)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10))))))))))))))))))))))))) != 0 {
										self.fRec0[0] = if iTemp0 != 0 {fTemp29} else {fTemp25 - F32::floor(fTemp25)};
									}
										if (iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow6 & iSlow10))) != 0 {
											iTemp30 = (self.fRec0[0] < 0.5) as i32;
										}
											if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow218)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow215)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
												self.fVec9[0] = fSlow219;
												fTemp31 = if ((fSlow219 != self.fVec9[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec22[1] + -1.0};
												self.fRec22[0] = fTemp31;
												self.fRec23[0] = if iTemp0 != 0 {fSlow219} else {if (fTemp31 > 0.0) as i32 != 0 {self.fRec23[1] + (fSlow219 - self.fRec23[1]) / F32::max(1.0, fTemp31)} else {fSlow219}};
											}
												if (iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow16)) | (iSlow2 & (iSlow6 & iSlow10)))) != 0 {
													fTemp32 = 1.0 - self.fRec23[0];
												}
													if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow218)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow215)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
														self.fVec10[0] = fSlow220;
														fTemp33 = if ((fSlow220 != self.fVec10[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec24[1] + -1.0};
														self.fRec24[0] = fTemp33;
														self.fRec25[0] = if iTemp0 != 0 {fSlow220} else {if (fTemp33 > 0.0) as i32 != 0 {self.fRec25[1] + (fSlow220 - self.fRec25[1]) / F32::max(1.0, fTemp33)} else {fSlow220}};
													}
														if iSlow2 & (iSlow6 & iSlow10) != 0 {
															fTemp34 = 1.0 - self.fRec25[0];
															fTemp35 = if iTemp30 != 0 {fTemp34} else {fTemp32};
															fTemp36 = if (fTemp35 < 0.999) as i32 != 0 {F32::max(0.01, fTemp35)} else {1.0};
														}
															if (iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow10))) != 0 {
																fTemp37 = 2.0 * self.fRec0[0];
															}
																if iSlow2 & (iSlow6 & iSlow10) != 0 {
																	fTemp38 = F32::floor(fTemp37);
																	fTemp39 = fTemp37 - (fTemp36 + fTemp38);
																	fTemp40 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp39 / fTemp23)));
																	fTemp41 = if iTemp30 != 0 {fSlow222} else {fSlow221};
																	fTemp42 = if (fTemp41 >= 2.0) as i32 != 0 {(if (fTemp41 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp41 >= 1.0) as i32 != 0 {4.0} else {6.2831855}};
																	fTemp43 = fTemp21 * fTemp42 * fTemp19 * fTemp3;
																	iTemp44 = (fTemp41) as i32;
																	fTemp45 = self.fConst0 * F32::abs(self.fConst11 * (fTemp23 / fTemp36));
																	fTemp46 = 0.5 * wt_basic(iTemp44, fTemp45, self.fConst0, 0.999999);
																	fTemp47 = self.fConst11 * fTemp23;
																	fTemp48 = fTemp37 - fTemp38;
																	fTemp49 = ((((fTemp36 < 1.0) as i32) & ((fTemp48 >= fTemp36) as i32)) & ((fTemp39 < fTemp47) as i32)) as F32;
																	fTemp50 = 1.0 - fTemp40;
																	fTemp51 = if iTemp30 != 0 {fSlow221} else {fSlow222};
																	fTemp52 = if iTemp30 != 0 {fTemp32} else {fTemp34};
																	fTemp53 = if (fTemp52 < 0.999) as i32 != 0 {F32::max(0.01, fTemp52)} else {1.0};
																	fTemp54 = ((fTemp53 == 1.0) as i32) as u32 as F32;
																	fTemp55 = 0.5 * (wt_basic(iTemp44, fTemp45, self.fConst0, 0.0) - fTemp54 * wt_basic((fTemp51) as i32, self.fConst0 * F32::abs(self.fConst11 * (fTemp23 / fTemp53)), self.fConst0, 0.999999));
																	fTemp56 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp48 / fTemp23)));
																	fTemp57 = 1.0 - fTemp56;
																	fTemp58 = fTemp42 / fTemp36 - fTemp54 * if (fTemp51 >= 2.0) as i32 != 0 {(if (fTemp51 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp51 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} / fTemp53;
																	fTemp59 = ((fTemp48 < fTemp47) as i32) as u32 as F32;
																	self.fVec11[0] = fTemp59 * OscDsp_faustpower2_f(fTemp57) * (self.fConst10 * fTemp23 * fTemp58 * fTemp57 - fTemp55) + fTemp49 * OscDsp_faustpower2_f(fTemp50) * (fTemp46 - self.fConst10 * (fTemp43 * fTemp50 / fTemp36));
																	self.fVec12[0] = ((fTemp48 < fTemp36) as i32) as u32 as F32 * wt_basic(iTemp44, fTemp45, self.fConst0, fTemp48 / fTemp36);
																	fTemp60 = fSlow9 * (self.fVec12[1] + fTemp59 * OscDsp_faustpower2_f(fTemp56) * (fTemp55 + self.fConst10 * fTemp23 * fTemp56 * fTemp58) + self.fVec11[1] - fTemp49 * OscDsp_faustpower2_f(fTemp40) * (fTemp46 + self.fConst10 * (fTemp43 * fTemp40 / fTemp36)));
																}
																	if iSlow2 & (iSlow6 & iSlow13) != 0 {
																		fTemp61 = self.fConst12 * fTemp23 * F32::exp(0.0046209814 * self.fRec23[0]);
																		fTemp62 = fTemp61 + self.fRec26[1];
																	}
																		if (iSlow2 & (iSlow6 & iSlow13)) | (iSlow101 & (iSlow143 & iSlow146)) != 0 {
																			fTemp63 = fSlow225 * fTemp27;
																			fTemp64 = F32::floor(fTemp63);
																			fTemp65 = fTemp63 - (fTemp64 + F32::floor(fTemp63 - fTemp64));
																		}
																			if iSlow2 & (iSlow6 & iSlow13) != 0 {
																				self.fRec26[0] = if iTemp0 != 0 {fTemp65} else {fTemp62 - F32::floor(fTemp62)};
																			}
																				if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow218)) | ((iSlow2 & (iSlow6 & iSlow25)) | (iSlow2 & (iSlow6 & iSlow13)))) != 0 {
																					fTemp66 = F32::max(0.0, F32::min(1.0, self.fRec25[0]));
																				}
																					if iSlow2 & (iSlow6 & iSlow13) != 0 {
																						fTemp67 = 4.0 * fTemp66;
																						fTemp68 = F32::min(3.0, fTemp67);
																					}
																						if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow13)))))))) != 0 {
																							fTemp69 = F32::abs(fTemp24);
																						}
																							if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow13))))))) != 0 {
																								fTemp70 = self.fConst0 * fTemp69;
																							}
																								if iSlow2 & (iSlow6 & iSlow13) != 0 {
																									fTemp71 = wt_basic(2, fTemp70, self.fConst0, self.fRec0[0]);
																									self.fVec13[0] = wt_basic(0, fTemp70, self.fConst0, self.fRec0[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp68)) + wt_basic(1, fTemp70, self.fConst0, self.fRec0[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp68 + -1.0)) + fTemp71 * F32::max(0.0, 1.0 - F32::abs(fTemp68 + -2.0)) + (fTemp71 - wt_basic(2, fTemp70, self.fConst0, self.fRec0[0] + (-0.5 - 0.42 * F32::max(0.0, fTemp67 + -3.0)))) * F32::max(0.0, 1.0 - F32::abs(fTemp68 + -3.0)) + self.fRec23[0] * wt_basic(iSlow224, self.fConst0 * F32::abs(fTemp61), self.fConst0, self.fRec26[0]);
																									fTemp72 = fSlow12 * (self.fVec13[1] / F32::sqrt(OscDsp_faustpower2_f(self.fRec23[0]) + 1.0));
																								}
																									if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																										iTemp73 = (self.fRec0[0] < self.fRec0[1]) as i32;
																									}
																										if (iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																											self.iRec27[0] = (i32::wrapping_add(iTemp73, self.iRec27[1])) % 2;
																										}
																											if iSlow2 & (iSlow6 & iSlow16) != 0 {
																												self.fVec14[0] = wt_basic(iSlow224, self.fConst0 * F32::abs(self.fConst12 * fTemp23), self.fConst0, 0.5 * (self.fRec0[0] + (self.iRec27[0]) as F32));
																											}
																												if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																													fTemp74 = 15.0 * self.fRec25[0];
																													fTemp75 = fTemp74 + 1.0;
																													fTemp76 = fTemp23 * fTemp75;
																													fTemp77 = self.fConst0 * F32::abs(self.fConst2 * fTemp76);
																													fTemp78 = wt_basic(iSlow227, fTemp77, self.fConst0, fTemp75);
																													fTemp79 = wt_basic(iSlow227, fTemp77, self.fConst0, 0.0);
																												}
																													if iSlow2 & (iSlow6 & iSlow16) != 0 {
																														fTemp80 = 0.5 * (fTemp79 - fTemp78);
																													}
																														if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																															fTemp81 = F32::max(0.0, F32::min(1.0, self.fConst0 * (self.fRec0[0] / fTemp23)));
																															fTemp82 = 1.0 - fTemp81;
																														}
																															if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																fTemp83 = fTemp74 + (1.0 - F32::floor(fTemp75));
																																fTemp84 = fSlow232 - if iSlow228 != 0 {fSlow231} else {if iSlow229 != 0 {(i32::wrapping_sub(4, i32::wrapping_mul(8, ((fTemp83 >= 0.25) as i32) & ((fTemp83 < 0.75) as i32)))) as F32} else {6.2831855 * F32::cos(6.2831855 * fTemp83)}};
																															}
																																if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																																	fTemp85 = (iTemp73) as F32;
																																	fTemp86 = fTemp85 * OscDsp_faustpower2_f(fTemp82);
																																}
																																	if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																		self.fVec15[0] = fTemp86 * (self.fConst13 * fTemp76 * fTemp84 * fTemp82 - fTemp80);
																																	}
																																		if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																																			fTemp87 = fTemp85 * OscDsp_faustpower2_f(fTemp81);
																																		}
																																			if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																																				fTemp88 = wt_basic(iSlow227, fTemp77, self.fConst0, self.fRec0[0] * fTemp75);
																																				self.fVec16[0] = fTemp88;
																																			}
																																				if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																					fTemp89 = fSlow15 * (fTemp32 * (self.fVec16[1] + fTemp87 * (fTemp80 + self.fConst13 * fTemp76 * fTemp81 * fTemp84) + self.fVec15[1]) + self.fRec23[0] * self.fVec14[1]);
																																				}
																																					if iSlow2 & (iSlow6 & iSlow19) != 0 {
																																						fTemp90 = if self.iRec27[0] != 0 {fSlow222} else {fSlow221};
																																						fTemp91 = 7.0 * self.fRec25[0] + 1.0;
																																						fTemp92 = fTemp91 * (self.fRec23[0] + 1.0);
																																						fTemp93 = if self.iRec27[0] != 0 {fTemp91} else {fTemp92};
																																						fTemp94 = if self.iRec27[0] != 0 {fSlow221} else {fSlow222};
																																						iTemp95 = (fTemp94) as i32;
																																						fTemp96 = if self.iRec27[0] != 0 {fTemp92} else {fTemp91};
																																						fTemp97 = 0.5 * (fTemp96 * wt_basic(iTemp95, fTemp70, self.fConst0, 0.0) - fTemp93 * wt_basic((fTemp90) as i32, fTemp70, self.fConst0, 0.999999));
																																						fTemp98 = if (fTemp94 >= 2.0) as i32 != 0 {(if (fTemp94 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp94 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp96 - if (fTemp90 >= 2.0) as i32 != 0 {(if (fTemp90 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp90 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp93;
																																						self.fVec17[0] = fTemp86 * (self.fConst13 * fTemp23 * fTemp82 * fTemp98 - fTemp97);
																																						self.fVec18[0] = wt_basic(iTemp95, fTemp70, self.fConst0, self.fRec0[0]) * fTemp96;
																																						fTemp99 = self.fVec18[1] + fTemp87 * (fTemp97 + self.fConst13 * fTemp23 * fTemp81 * fTemp98) + self.fVec17[1];
																																						self.fVec19[0] = fTemp99;
																																						fTemp100 = fTemp99 - self.fVec19[1];
																																						iTemp101 = (F32::abs(fTemp100) < 0.0001) as i32;
																																						fTemp102 = 4.0 * F32::floor(0.25 * (self.fVec19[1] + 1.0));
																																						fTemp103 = self.fVec19[1] - fTemp102;
																																						fTemp104 = 4.0 * F32::floor(0.25 * (fTemp99 + 1.0));
																																						fTemp105 = fTemp99 - fTemp104;
																																						fTemp106 = 0.125 * (fTemp99 + self.fVec19[1]);
																																						fTemp107 = fSlow18 * if iTemp101 != 0 {1.0 - 4.0 * F32::abs(fTemp106 + (-0.25 - F32::floor(fTemp106 + 0.25)))} else {(if (fTemp105 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp105)} else {1.0 - 0.5 * OscDsp_faustpower2_f(fTemp99 + (-2.0 - fTemp104))} - if (fTemp103 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp103)} else {1.0 - 0.5 * OscDsp_faustpower2_f(self.fVec19[1] + (-2.0 - fTemp102))}) / if iTemp101 != 0 {1.0} else {fTemp100}};
																																					}
																																						if iSlow2 & (iSlow6 & iSlow22) != 0 {
																																							fTemp108 = 4.0 * F32::max(0.0, F32::min(1.0, self.fRec23[0]));
																																							iTemp109 = (F32::min(3.0, fTemp108)) as i32;
																																							iTemp110 = i32::wrapping_add(iTemp109, 1);
																																							iTemp111 = (iTemp110 >= 3) as i32;
																																							iTemp112 = (iTemp110 >= 2) as i32;
																																							iTemp113 = (iTemp110 >= 1) as i32;
																																							iTemp114 = (iTemp110 >= 4) as i32;
																																							fTemp115 = 0.5 * (wt_basic(iSlow224, fTemp70, self.fConst0, 0.999999) + 1.0);
																																							fTemp116 = (iTemp109) as F32;
																																							fTemp117 = fTemp108 - fTemp116;
																																							iTemp118 = (iTemp109 >= 3) as i32;
																																							iTemp119 = (iTemp109 >= 2) as i32;
																																							iTemp120 = (iTemp109 >= 1) as i32;
																																							iTemp121 = (iTemp109 >= 4) as i32;
																																							fTemp122 = fTemp116 + (1.0 - fTemp108);
																																							fTemp123 = 0.5 * (wt_basic(iSlow224, fTemp70, self.fConst0, 0.0) + 1.0);
																																							fTemp124 = fTemp79 * (fTemp122 * if iTemp118 != 0 {if iTemp121 != 0 {fTemp123} else {0.0}} else {if iTemp119 != 0 {0.0} else {(if iTemp120 != 0 {0} else {1}) as F32}} + fTemp117 * if iTemp111 != 0 {if iTemp114 != 0 {fTemp123} else {0.0}} else {if iTemp112 != 0 {0.0} else {(if iTemp113 != 0 {0} else {1}) as F32}}) - fTemp78 * (fTemp122 * if iTemp118 != 0 {if iTemp121 != 0 {fTemp115} else {6e-06}} else {if iTemp119 != 0 {0.0} else {if iTemp120 != 0 {2e-06} else {1e-06}}} + fTemp117 * if iTemp111 != 0 {if iTemp114 != 0 {fTemp115} else {6e-06}} else {if iTemp112 != 0 {0.0} else {if iTemp113 != 0 {2e-06} else {1e-06}}});
																																							self.fVec20[0] = fTemp86 * fTemp124;
																																						}
																																							if (iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow6 & iSlow22)) != 0 {
																																								fTemp125 = 1.0 - self.fRec0[0];
																																							}
																																								if iSlow2 & (iSlow6 & iSlow22) != 0 {
																																									fTemp126 = 1.0 - F32::abs(fTemp37 + -1.0);
																																									fTemp127 = F32::sin(3.1415927 * self.fRec0[0]);
																																									fTemp128 = F32::min(1.0, 6.0 * F32::min(self.fRec0[0], fTemp125));
																																									fTemp129 = 0.5 * (wt_basic(iSlow224, fTemp70, self.fConst0, self.fRec0[0]) + 1.0);
																																									self.fVec21[0] = fTemp88 * (fTemp122 * if iTemp118 != 0 {if iTemp121 != 0 {fTemp129} else {fTemp128}} else {if iTemp119 != 0 {fTemp127} else {if iTemp120 != 0 {fTemp126} else {fTemp125}}} + fTemp117 * if iTemp111 != 0 {if iTemp114 != 0 {fTemp129} else {fTemp128}} else {if iTemp112 != 0 {fTemp127} else {if iTemp113 != 0 {fTemp126} else {fTemp125}}});
																																									fTemp130 = fSlow21 * (self.fVec21[1] + 0.5 * (fTemp87 * fTemp124 - self.fVec20[1]));
																																								}
																																									if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																										fTemp131 = self.fConst2 * fTemp23 * (2.17 * fTemp66 + 3.0);
																																										fTemp132 = fTemp131 + self.fRec28[1];
																																									}
																																										if (iSlow2 & (iSlow6 & iSlow25)) | (iSlow101 & (iSlow143 & iSlow158)) != 0 {
																																											fTemp133 = F32::floor(fTemp28 + 0.67);
																																											fTemp134 = fTemp28 + (0.67 - (fTemp133 + F32::floor(fTemp28 + (0.67 - fTemp133))));
																																										}
																																											if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																												self.fRec28[0] = if iTemp0 != 0 {fTemp134} else {fTemp132 - F32::floor(fTemp132)};
																																												self.fVec22[0] = wt_basic(iSlow224, self.fConst0 * F32::abs(fTemp131), self.fConst0, self.fRec28[0]);
																																												fTemp135 = self.fConst2 * fTemp23 * (1.41 * fTemp66 + 2.0);
																																												fTemp136 = fTemp135 + self.fRec29[1];
																																											}
																																												if (iSlow2 & (iSlow6 & iSlow25)) | (iSlow101 & (iSlow143 & iSlow158)) != 0 {
																																													fTemp137 = F32::floor(fTemp28 + 0.31);
																																													fTemp138 = fTemp28 + (0.31 - (fTemp137 + F32::floor(fTemp28 + (0.31 - fTemp137))));
																																												}
																																													if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																														self.fRec29[0] = if iTemp0 != 0 {fTemp138} else {fTemp136 - F32::floor(fTemp136)};
																																														self.fVec23[0] = wt_basic(iSlow224, self.fConst0 * F32::abs(fTemp135), self.fConst0, self.fRec29[0]);
																																													}
																																														if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | (iSlow2 & (iSlow6 & iSlow25)))) != 0 {
																																															fTemp139 = wt_basic(iSlow227, fTemp70, self.fConst0, self.fRec0[0]);
																																															self.fVec24[0] = fTemp139;
																																														}
																																															if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																																fTemp140 = fSlow24 * (0.57735026 * fTemp32 * (self.fVec24[1] + self.fVec23[1] + self.fVec22[1]) + 1.5 * self.fRec23[0] * self.fVec24[1] * self.fVec23[1] * self.fVec22[1]);
																																															}
																																																if iSlow2 & (iSlow6 & iSlow28) != 0 {
																																																	iTemp141 = (F32::max(0.0, F32::min(1e+01, 10.999 * self.fRec25[0]))) as i32;
																																																	fTemp142 = unsafe { ftbl1OscDspSIG1[(i32::wrapping_add(iTemp141, 1)) as usize] };
																																																	fTemp143 = self.fConst2 * fTemp23 * fTemp142;
																																																	fTemp144 = fTemp143 + self.fRec30[1];
																																																	fTemp145 = fSlow136 * fTemp27 * fTemp142;
																																																	fTemp146 = F32::floor(fTemp145);
																																																	self.fRec30[0] = if iTemp0 != 0 {fTemp145 - (fTemp146 + F32::floor(fTemp145 - fTemp146))} else {fTemp144 - F32::floor(fTemp144)};
																																																	self.fVec25[0] = wt_basic(iSlow224, self.fConst0 * F32::abs(fTemp143), self.fConst0, self.fRec30[0]);
																																																	fTemp147 = unsafe { ftbl1OscDspSIG1[iTemp141 as usize] };
																																																	fTemp148 = self.fConst2 * fTemp23 * fTemp147;
																																																	fTemp149 = fTemp148 + self.fRec31[1];
																																																	fTemp150 = fSlow136 * fTemp27 * fTemp147;
																																																	fTemp151 = F32::floor(fTemp150);
																																																	self.fRec31[0] = if iTemp0 != 0 {fTemp150 - (fTemp151 + F32::floor(fTemp150 - fTemp151))} else {fTemp149 - F32::floor(fTemp149)};
																																																	self.fVec26[0] = wt_basic(iSlow224, self.fConst0 * F32::abs(fTemp148), self.fConst0, self.fRec31[0]);
																																																	fTemp152 = fSlow27 * self.fVec24[1] * (fTemp32 * self.fVec26[1] + self.fRec23[0] * self.fVec25[1]);
																																																}
																																																	if iSlow2 & (iSlow6 & iSlow215) != 0 {
																																																		fTemp153 = unsafe { ftbl2OscDspSIG2[((F32::max(0.0, F32::min(9.0, 9.999 * self.fRec23[0]))) as i32) as usize] };
																																																		fTemp154 = self.fConst2 * fTemp23 * fTemp153;
																																																		fTemp155 = F32::max(F32::abs(fTemp154), 1e-09);
																																																		iTemp156 = (fTemp154 < 0.0) as i32;
																																																		fTemp157 = fTemp154 + self.fRec32[1];
																																																		fTemp158 = fSlow136 * fTemp27 * fTemp153;
																																																		fTemp159 = F32::floor(fTemp158);
																																																		self.fRec32[0] = if iTemp0 != 0 {fTemp158 - (fTemp159 + F32::floor(fTemp158 - fTemp159))} else {fTemp157 - F32::floor(fTemp157)};
																																																		fTemp160 = if iTemp156 != 0 {fSlow242 - (self.fRec32[0] + F32::floor(fSlow242 - self.fRec32[0]))} else {self.fRec32[0] - (fSlow242 + F32::floor(self.fRec32[0] - fSlow242))};
																																																		iTemp161 = ((fTemp160 < fTemp155) as i32) * if iTemp156 != 0 {-iSlow240} else {iSlow240};
																																																		fTemp162 = if iTemp156 != 0 {fSlow244 - (self.fRec32[0] + F32::floor(fSlow244 - self.fRec32[0]))} else {self.fRec32[0] - (fSlow244 + F32::floor(self.fRec32[0] - fSlow244))};
																																																		iTemp163 = ((fTemp162 < fTemp155) as i32) * if iTemp156 != 0 {-iSlow243} else {iSlow243};
																																																		fTemp164 = F32::floor(self.fRec32[0]);
																																																		fTemp165 = self.fRec32[0] - fTemp164;
																																																		fTemp166 = if iSlow238 != 0 {if iSlow239 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp165 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec32[0] + (0.5 - (fTemp164 + F32::floor(self.fRec32[0] + (0.5 - fTemp164))))) + -1.0}} else {if iSlow241 != 0 {1.0 - 4.0 * F32::abs(self.fRec32[0] + (-0.25 - (fTemp164 + F32::floor(self.fRec32[0] + (0.25 - fTemp164)))))} else {F32::sin(6.2831855 * fTemp165)}};
																																																		self.fVec27[0] = fTemp166;
																																																	}
																																																		if (iSlow2 & (iSlow6 & iSlow31)) | (iSlow2 & (iSlow6 & iSlow215)) != 0 {
																																																			fTemp167 = OscDsp_faustpower2_f(self.fRec25[0]);
																																																		}
																																																			if iSlow2 & (iSlow6 & iSlow215) != 0 {
																																																				fTemp168 = self.fConst2 * fTemp23 * (8.0 * fTemp167 * fTemp153 * (0.5 * (fTemp166 + self.fVec27[1] - (i32::wrapping_add(iTemp163, iTemp161)) as F32) + (fTemp162 * (iTemp163) as F32 + fTemp160 * (iTemp161) as F32) / fTemp155) + 1.0);
																																																				iTemp169 = (fTemp168 < 0.0) as i32;
																																																				fTemp170 = 0.5 * (if iTemp169 != 0 {-iSlow245} else {iSlow245}) as F32;
																																																				fTemp171 = F32::max(F32::abs(fTemp168), 1e-09);
																																																				fTemp172 = self.fRec33[1] + fTemp168;
																																																				self.fRec33[0] = if iTemp0 != 0 {fTemp29} else {fTemp172 - F32::floor(fTemp172)};
																																																				fTemp173 = if iTemp169 != 0 {fSlow246 - (self.fRec33[0] + F32::floor(fSlow246 - self.fRec33[0]))} else {self.fRec33[0] - (fSlow246 + F32::floor(self.fRec33[0] - fSlow246))};
																																																				fTemp174 = F32::max(0.0, F32::min(1.0, fTemp173 / fTemp171));
																																																				fTemp175 = 1.0 - fTemp174;
																																																				fTemp176 = ((fTemp173 < fTemp171) as i32) as u32 as F32;
																																																				fTemp177 = 0.5 * (if iTemp169 != 0 {-iSlow248} else {iSlow248}) as F32;
																																																				fTemp178 = if iTemp169 != 0 {fSlow249 - (self.fRec33[0] + F32::floor(fSlow249 - self.fRec33[0]))} else {self.fRec33[0] - (fSlow249 + F32::floor(self.fRec33[0] - fSlow249))};
																																																				fTemp179 = F32::max(0.0, F32::min(1.0, fTemp178 / fTemp171));
																																																				fTemp180 = 1.0 - fTemp179;
																																																				fTemp181 = ((fTemp178 < fTemp171) as i32) as u32 as F32;
																																																				self.fVec28[0] = fTemp181 * OscDsp_faustpower2_f(fTemp180) * (0.16666667 * fTemp171 * fSlow250 * fTemp180 - fTemp177) + fTemp176 * OscDsp_faustpower2_f(fTemp175) * (0.16666667 * fTemp171 * fSlow247 * fTemp175 - fTemp170);
																																																				fTemp182 = F32::floor(self.fRec33[0]);
																																																				fTemp183 = self.fRec33[0] - fTemp182;
																																																				self.fVec29[0] = if iSlow228 != 0 {if iSlow230 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp183 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec33[0] + (0.5 - (fTemp182 + F32::floor(self.fRec33[0] + (0.5 - fTemp182))))) + -1.0}} else {if iSlow229 != 0 {1.0 - 4.0 * F32::abs(self.fRec33[0] + (-0.25 - (fTemp182 + F32::floor(self.fRec33[0] + (0.25 - fTemp182)))))} else {F32::sin(6.2831855 * fTemp183)}};
																																																				fTemp184 = fSlow214 * (fTemp176 * OscDsp_faustpower2_f(fTemp174) * (fTemp170 + 0.16666667 * fTemp171 * fTemp174 * fSlow247) + self.fVec29[1] + fTemp181 * OscDsp_faustpower2_f(fTemp179) * (fTemp177 + 0.16666667 * fTemp171 * fTemp179 * fSlow250) + self.fVec28[1]);
																																																			}
																																																				if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																					fTemp185 = F32::max(1e+01, F32::min(self.fConst14, fTemp23 * F32::exp(0.6931472 * (8.0 * self.fRec23[0] + -3.0))));
																																																					fTemp186 = 1.0 / F32::tan(self.fConst8 * fTemp185);
																																																				}
																																																					if (iSlow2 & (iSlow6 & iSlow31)) | (iSlow101 & (iSlow143 & iSlow167)) != 0 {
																																																						fTemp187 = 4.656613e-10 * (fTemp14 + self.fVec5[1]);
																																																					}
																																																						if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																							self.fRec34[0] = (fTemp187 - self.fRec34[1] * (1.0 - fTemp186)) / (fTemp186 + 1.0);
																																																							fTemp188 = fSlow30 * (self.fVec24[1] * (1.5 * self.fRec25[0] * self.fRec34[0] * F32::sqrt(self.fConst15 / fTemp185) + 1.0) / F32::sqrt(2.25 * fTemp167 + 1.0));
																																																						}
																																																							if iSlow2 & (iSlow6 & iSlow218) != 0 {
																																																								fTemp189 = 0.6 * fTemp66 * self.fRec36[1] + 1.0;
																																																								fTemp190 = self.fConst2 * fTemp23 * fTemp189;
																																																								fTemp191 = self.fRec35[1] + fTemp190;
																																																								iTemp192 = (fTemp191 >= 1.0) as i32;
																																																								self.fRec35[0] = if iTemp0 != 0 {fTemp29} else {fTemp191 - (iTemp192) as F32};
																																																								self.fRec36[0] = if iTemp192 != 0 {fTemp15} else {self.fRec36[1]};
																																																								fTemp193 = F32::max(F32::abs(fTemp190), 1e-09);
																																																								iTemp194 = (fTemp190 < 0.0) as i32;
																																																								fTemp195 = if iTemp194 != 0 {fSlow242 - (self.fRec35[0] + F32::floor(fSlow242 - self.fRec35[0]))} else {self.fRec35[0] - (fSlow242 + F32::floor(self.fRec35[0] - fSlow242))};
																																																								iTemp196 = ((fTemp195 < fTemp193) as i32) * if iTemp194 != 0 {-iSlow240} else {iSlow240};
																																																								fTemp197 = if iTemp194 != 0 {fSlow244 - (self.fRec35[0] + F32::floor(fSlow244 - self.fRec35[0]))} else {self.fRec35[0] - (fSlow244 + F32::floor(self.fRec35[0] - fSlow244))};
																																																								iTemp198 = ((fTemp197 < fTemp193) as i32) * if iTemp194 != 0 {-iSlow243} else {iSlow243};
																																																								fTemp199 = F32::floor(self.fRec35[0]);
																																																								fTemp200 = self.fRec35[0] - fTemp199;
																																																								fTemp201 = if iSlow238 != 0 {if iSlow239 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp200 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec35[0] + (0.5 - (fTemp199 + F32::floor(self.fRec35[0] + (0.5 - fTemp199))))) + -1.0}} else {if iSlow241 != 0 {1.0 - 4.0 * F32::abs(self.fRec35[0] + (-0.25 - (fTemp199 + F32::floor(self.fRec35[0] + (0.25 - fTemp199)))))} else {F32::sin(6.2831855 * fTemp200)}};
																																																								self.fVec30[0] = fTemp201;
																																																								fTemp202 = self.fConst2 * fTemp23 * (4.0 * self.fRec23[0] * fTemp189 * (0.5 * (fTemp201 + self.fVec30[1] - (i32::wrapping_add(iTemp198, iTemp196)) as F32) + (fTemp197 * (iTemp198) as F32 + fTemp195 * (iTemp196) as F32) / fTemp193) + 1.0);
																																																								iTemp203 = (fTemp202 < 0.0) as i32;
																																																								fTemp204 = 0.5 * (if iTemp203 != 0 {-iSlow245} else {iSlow245}) as F32;
																																																								fTemp205 = F32::max(F32::abs(fTemp202), 1e-09);
																																																								fTemp206 = self.fRec37[1] + fTemp202;
																																																								self.fRec37[0] = if iTemp0 != 0 {fTemp29} else {fTemp206 - F32::floor(fTemp206)};
																																																								fTemp207 = if iTemp203 != 0 {fSlow246 - (self.fRec37[0] + F32::floor(fSlow246 - self.fRec37[0]))} else {self.fRec37[0] - (fSlow246 + F32::floor(self.fRec37[0] - fSlow246))};
																																																								fTemp208 = F32::max(0.0, F32::min(1.0, fTemp207 / fTemp205));
																																																								fTemp209 = 1.0 - fTemp208;
																																																								fTemp210 = fSlow247 * fTemp205;
																																																								fTemp211 = ((fTemp207 < fTemp205) as i32) as u32 as F32;
																																																								fTemp212 = 0.5 * (if iTemp203 != 0 {-iSlow248} else {iSlow248}) as F32;
																																																								fTemp213 = if iTemp203 != 0 {fSlow249 - (self.fRec37[0] + F32::floor(fSlow249 - self.fRec37[0]))} else {self.fRec37[0] - (fSlow249 + F32::floor(self.fRec37[0] - fSlow249))};
																																																								fTemp214 = F32::max(0.0, F32::min(1.0, fTemp213 / fTemp205));
																																																								fTemp215 = 1.0 - fTemp214;
																																																								fTemp216 = fSlow250 * fTemp205;
																																																								fTemp217 = ((fTemp213 < fTemp205) as i32) as u32 as F32;
																																																								self.fVec31[0] = fTemp217 * OscDsp_faustpower2_f(fTemp215) * (0.16666667 * fTemp216 * fTemp215 - fTemp212) + fTemp211 * OscDsp_faustpower2_f(fTemp209) * (0.16666667 * fTemp210 * fTemp209 - fTemp204);
																																																								fTemp218 = F32::floor(self.fRec37[0]);
																																																								fTemp219 = self.fRec37[0] - fTemp218;
																																																								self.fVec32[0] = if iSlow228 != 0 {if iSlow230 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp219 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec37[0] + (0.5 - (fTemp218 + F32::floor(self.fRec37[0] + (0.5 - fTemp218))))) + -1.0}} else {if iSlow229 != 0 {1.0 - 4.0 * F32::abs(self.fRec37[0] + (-0.25 - (fTemp218 + F32::floor(self.fRec37[0] + (0.25 - fTemp218)))))} else {F32::sin(6.2831855 * fTemp219)}};
																																																								fTemp220 = fSlow217 * (fTemp211 * OscDsp_faustpower2_f(fTemp208) * (fTemp204 + 0.16666667 * fTemp210 * fTemp208) + self.fVec32[1] + fTemp217 * OscDsp_faustpower2_f(fTemp214) * (fTemp212 + 0.16666667 * fTemp216 * fTemp214) + self.fVec31[1]);
																																																							}
																																																								if iSlow2 & (iSlow6 & iSlow34) != 0 {
																																																									iTemp221 = (fTemp24 < 0.0) as i32;
																																																									fTemp222 = 0.5 * (if iTemp221 != 0 {-iSlow245} else {iSlow245}) as F32;
																																																									fTemp223 = F32::max(fTemp69, 1e-09);
																																																									fTemp224 = if iTemp221 != 0 {fSlow246 - (self.fRec0[0] + F32::floor(fSlow246 - self.fRec0[0]))} else {self.fRec0[0] - (fSlow246 + F32::floor(self.fRec0[0] - fSlow246))};
																																																									fTemp225 = F32::max(0.0, F32::min(1.0, fTemp224 / fTemp223));
																																																									fTemp226 = 1.0 - fTemp225;
																																																									fTemp227 = fSlow247 * fTemp223;
																																																									fTemp228 = ((fTemp224 < fTemp223) as i32) as u32 as F32;
																																																									fTemp229 = 0.5 * (if iTemp221 != 0 {-iSlow248} else {iSlow248}) as F32;
																																																									fTemp230 = if iTemp221 != 0 {fSlow249 - (self.fRec0[0] + F32::floor(fSlow249 - self.fRec0[0]))} else {self.fRec0[0] - (fSlow249 + F32::floor(self.fRec0[0] - fSlow249))};
																																																									fTemp231 = F32::max(0.0, F32::min(1.0, fTemp230 / fTemp223));
																																																									fTemp232 = 1.0 - fTemp231;
																																																									fTemp233 = fSlow250 * fTemp223;
																																																									fTemp234 = ((fTemp230 < fTemp223) as i32) as u32 as F32;
																																																									self.fVec33[0] = fTemp234 * OscDsp_faustpower2_f(fTemp232) * (0.16666667 * fTemp233 * fTemp232 - fTemp229) + fTemp228 * OscDsp_faustpower2_f(fTemp226) * (0.16666667 * fTemp227 * fTemp226 - fTemp222);
																																																									fTemp235 = F32::floor(self.fRec0[0]);
																																																									fTemp236 = self.fRec0[0] - fTemp235;
																																																									self.fVec34[0] = if iSlow228 != 0 {if iSlow230 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp236 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec0[0] + (0.5 - (fTemp235 + F32::floor(self.fRec0[0] + (0.5 - fTemp235))))) + -1.0}} else {if iSlow229 != 0 {1.0 - 4.0 * F32::abs(self.fRec0[0] + (-0.25 - (fTemp235 + F32::floor(self.fRec0[0] + (0.25 - fTemp235)))))} else {F32::sin(6.2831855 * fTemp236)}};
																																																									fTemp237 = (9.0 * self.fRec25[0] + 1.0) * (1.5 * self.fRec23[0] + fTemp228 * OscDsp_faustpower2_f(fTemp225) * (fTemp222 + 0.16666667 * fTemp227 * fTemp225) + self.fVec34[1] + fTemp234 * OscDsp_faustpower2_f(fTemp231) * (fTemp229 + 0.16666667 * fTemp233 * fTemp231) + self.fVec33[1]);
																																																									self.fVec35[0] = fTemp237;
																																																									fTemp238 = fTemp237 - self.fVec35[1];
																																																									iTemp239 = (F32::abs(fTemp238) < 0.0001) as i32;
																																																									fTemp240 = if iTemp239 != 0 {F32::sin(0.7853982 * (fTemp237 + self.fVec35[1]))} else {-(0.63661975 * ((F32::cos(1.5707964 * fTemp237) - F32::cos(1.5707964 * self.fVec35[1])) / if iTemp239 != 0 {1.0} else {fTemp238}))};
																																																									self.fVec36[0] = fTemp240;
																																																									self.fRec38[0] = 0.995 * self.fRec38[1] + fTemp240 - self.fVec36[1];
																																																									fTemp241 = fSlow33 * self.fRec38[0];
																																																								}
																																																									if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																										fTemp242 = 1.0 - OscDsp_faustpower2_f(1.0 - fTemp66);
																																																										fTemp243 = 1.0 - 0.5 * OscDsp_faustpower2_f(fTemp66);
																																																										iTemp244 = (self.fRec23[0] > 0.5) as i32;
																																																										fTemp245 = if iTemp244 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * self.fRec23[0])};
																																																										iTemp246 = unsafe { itbl3OscDspSIG3[((F32::max(0.0, F32::min(4.0, 1e+01 * (self.fRec23[0] + -0.5)))) as i32) as usize] };
																																																										fTemp247 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp244, iTemp246))) as F32;
																																																										fTemp248 = self.fConst2 * fTemp23 * F32::exp(0.0005776227 * (fTemp247 + 0.83 * fTemp245));
																																																										fTemp249 = fTemp248 + self.fRec39[1];
																																																									}
																																																										if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow101 & (iSlow143 & iSlow176)) != 0 {
																																																											fTemp250 = F32::floor(fTemp28 + 0.87);
																																																											fTemp251 = fTemp28 + (0.87 - (fTemp250 + F32::floor(fTemp28 + (0.87 - fTemp250))));
																																																										}
																																																											if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																												self.fRec39[0] = if iTemp0 != 0 {fTemp251} else {fTemp249 - F32::floor(fTemp249)};
																																																												fTemp252 = self.fConst2 * fTemp23 * F32::exp(0.0005776227 * (0.36 * fTemp245 + fTemp247));
																																																												fTemp253 = fTemp252 + self.fRec40[1];
																																																											}
																																																												if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow101 & (iSlow143 & iSlow176)) != 0 {
																																																													fTemp254 = F32::floor(fTemp28 + 0.5);
																																																													fTemp255 = fTemp28 + (0.5 - (fTemp254 + F32::floor(fTemp28 + (0.5 - fTemp254))));
																																																												}
																																																													if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																														self.fRec40[0] = if iTemp0 != 0 {fTemp255} else {fTemp253 - F32::floor(fTemp253)};
																																																														fTemp256 = self.fConst2 * fTemp23 * F32::exp(0.0005776227 * ((i32::wrapping_mul(0, iTemp246)) as F32 - fTemp245)) + self.fRec41[1];
																																																													}
																																																														if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow101 & (iSlow143 & iSlow176)) != 0 {
																																																															fTemp257 = F32::floor(fTemp28 + 0.13);
																																																															fTemp258 = fTemp28 + (0.13 - (fTemp257 + F32::floor(fTemp28 + (0.13 - fTemp257))));
																																																														}
																																																															if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																																self.fRec41[0] = if iTemp0 != 0 {fTemp258} else {fTemp256 - F32::floor(fTemp256)};
																																																																self.fVec37[0] = fTemp139 * fTemp243 + 0.5 * fTemp242 * (wt_basic(iSlow224, self.fConst0 * F32::abs(self.fConst2 * fTemp23 * F32::exp(-0.0005776227 * fTemp245)), self.fConst0, self.fRec41[0]) + wt_basic(iSlow224, self.fConst0 * F32::abs(fTemp252), self.fConst0, self.fRec40[0]) + wt_basic(iSlow224, self.fConst0 * F32::abs(fTemp248), self.fConst0, self.fRec39[0]));
																																																																fTemp259 = fSlow36 * (self.fVec37[1] / F32::sqrt(OscDsp_faustpower2_f(fTemp243) + 0.75 * OscDsp_faustpower2_f(fTemp242)));
																																																															}
																																																																if iSlow2 & iSlow6 != 0 {
																																																																	fTemp260 = fSlow5 * fSlow256 * (fSlow255 * fTemp259 + fSlow254 * fTemp241 + fSlow253 * fTemp220 + fSlow252 * fTemp188 + fSlow251 * fTemp184 + fSlow237 * fTemp152 + fSlow236 * fTemp140 + fSlow235 * fTemp130 + fSlow234 * fTemp107 + fSlow233 * fTemp89 + fSlow226 * fTemp72 + fSlow223 * fTemp60);
																																																																}
																																																																	if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41))))))))))))))) != 0 {
																																																																		self.fVec38[0] = fSlow258;
																																																																		fTemp261 = if ((fSlow258 != self.fVec38[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec42[1] + -1.0};
																																																																		self.fRec42[0] = fTemp261;
																																																																		self.fRec43[0] = if iTemp0 != 0 {fSlow258} else {if (fTemp261 > 0.0) as i32 != 0 {self.fRec43[1] + (fSlow258 - self.fRec43[1]) / F32::max(1.0, fTemp261)} else {fSlow258}};
																																																																		fTemp262 = 3.0 * F32::max(0.0, F32::min(1.0, self.fRec43[0]));
																																																																		iTemp263 = std::cmp::min(2, (fTemp262) as i32);
																																																																		iTemp264 = (iTemp263 >= 2) as i32;
																																																																		iTemp265 = (iTemp263 >= 1) as i32;
																																																																		iTemp266 = i32::wrapping_mul(2176, (if iTemp264 != 0 {fSlow261} else {if iTemp265 != 0 {fSlow260} else {fSlow259}}) as i32);
																																																																	}
																																																																		if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41)))))))))))))) != 0 {
																																																																			fTemp267 = 256.0 * fTemp23;
																																																																			iTemp268 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp267 > self.fConst16) as i32) + ((fTemp267 > self.fConst22) as i32), (fTemp267 > self.fConst21) as i32), (fTemp267 > self.fConst20) as i32), (fTemp267 > self.fConst19) as i32), (fTemp267 > self.fConst18) as i32), (fTemp267 > self.fConst17) as i32);
																																																																			iTemp269 = (iTemp268 >= 4) as i32;
																																																																			iTemp270 = (iTemp268 >= 2) as i32;
																																																																			iTemp271 = (iTemp268 >= 1) as i32;
																																																																			iTemp272 = (iTemp268 >= 3) as i32;
																																																																			iTemp273 = if iTemp269 != 0 {if (iTemp268 >= 6) as i32 != 0 {if (iTemp268 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp268 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp270 != 0 {if iTemp272 != 0 {1792} else {1536}} else {if iTemp271 != 0 {1024} else {0}}};
																																																																			iTemp274 = if iTemp269 != 0 {64} else {if iTemp270 != 0 {if iTemp272 != 0 {128} else {256}} else {if iTemp271 != 0 {512} else {1024}}};
																																																																			fTemp275 = self.fRec0[0] * (iTemp274) as F32;
																																																																			iTemp276 = (fTemp275) as i32;
																																																																			iTemp277 = (i32::wrapping_add(iTemp276, 1)) % iTemp274;
																																																																			fTemp278 = (iTemp276) as F32;
																																																																			fTemp279 = fTemp275 - fTemp278;
																																																																			fTemp280 = fTemp278 + (1.0 - fTemp275);
																																																																		}
																																																																			if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41))))))))))))))) != 0 {
																																																																				fTemp281 = (iTemp263) as F32;
																																																																				fTemp282 = fTemp262 - fTemp281;
																																																																				iTemp283 = i32::wrapping_mul(2176, (if iTemp264 != 0 {fSlow260} else {if iTemp265 != 0 {fSlow259} else {fSlow262}}) as i32);
																																																																				fTemp284 = fTemp281 + (1.0 - fTemp262);
																																																																			}
																																																																				if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41)))))))))))))) != 0 {
																																																																					fTemp285 = fTemp284 * (fTemp280 * wt_sample(i32::wrapping_add(iTemp273, i32::wrapping_add(iTemp276, iTemp283))) + fTemp279 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp283, iTemp273), iTemp277))) + fTemp282 * (fTemp280 * wt_sample(i32::wrapping_add(iTemp273, i32::wrapping_add(iTemp276, iTemp266))) + fTemp279 * wt_sample(i32::wrapping_add(iTemp277, i32::wrapping_add(iTemp273, iTemp266))));
																																																																				}
																																																																					if (iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41)))))))))))))) != 0 {
																																																																						self.fVec39[0] = fSlow263;
																																																																						fTemp286 = if ((fSlow263 != self.fVec39[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec44[1] + -1.0};
																																																																						self.fRec44[0] = fTemp286;
																																																																						self.fRec45[0] = if iTemp0 != 0 {fSlow263} else {if (fTemp286 > 0.0) as i32 != 0 {self.fRec45[1] + (fSlow263 - self.fRec45[1]) / F32::max(1.0, fTemp286)} else {fSlow263}};
																																																																						fTemp287 = F32::max(0.0, F32::min(1.0, self.fRec45[0]));
																																																																					}
																																																																						if (iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41)))) != 0 {
																																																																							fTemp288 = (1.0 - fTemp287) * fTemp285;
																																																																						}
																																																																							if iSlow2 & (iSlow44 & iSlow41) != 0 {
																																																																								fTemp289 = fSlow40 * (fTemp288 + fTemp287 * if (fTemp285 > 0.0) as i32 != 0 {1.0 - fTemp285} else {-1.0 - fTemp285});
																																																																							}
																																																																								if (iSlow2 & (iSlow44 & iSlow62)) | (iSlow2 & (iSlow44 & iSlow47)) != 0 {
																																																																									fTemp290 = 512.0 * fTemp23;
																																																																									iTemp291 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp290 > self.fConst16) as i32) + ((fTemp290 > self.fConst22) as i32), (fTemp290 > self.fConst21) as i32), (fTemp290 > self.fConst20) as i32), (fTemp290 > self.fConst19) as i32), (fTemp290 > self.fConst18) as i32), (fTemp290 > self.fConst17) as i32);
																																																																									iTemp292 = (iTemp291 >= 4) as i32;
																																																																									iTemp293 = (iTemp291 >= 2) as i32;
																																																																									iTemp294 = (iTemp291 >= 1) as i32;
																																																																									iTemp295 = (iTemp291 >= 3) as i32;
																																																																									iTemp296 = if iTemp292 != 0 {64} else {if iTemp293 != 0 {if iTemp295 != 0 {128} else {256}} else {if iTemp294 != 0 {512} else {1024}}};
																																																																									fTemp297 = (iTemp296) as F32;
																																																																								}
																																																																									if iSlow2 & (iSlow44 & iSlow47) != 0 {
																																																																										fTemp298 = fTemp297 * if iTemp30 != 0 {fTemp37} else {2.0 * fTemp125};
																																																																										iTemp299 = (fTemp298) as i32;
																																																																										iTemp300 = (i32::wrapping_add(iTemp299, 1)) % iTemp296;
																																																																									}
																																																																										if (iSlow2 & (iSlow44 & iSlow62)) | (iSlow2 & (iSlow44 & iSlow47)) != 0 {
																																																																											iTemp301 = if iTemp292 != 0 {if (iTemp291 >= 6) as i32 != 0 {if (iTemp291 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp291 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp293 != 0 {if iTemp295 != 0 {1792} else {1536}} else {if iTemp294 != 0 {1024} else {0}}};
																																																																											iTemp302 = i32::wrapping_add(iTemp266, iTemp301);
																																																																										}
																																																																											if iSlow2 & (iSlow44 & iSlow47) != 0 {
																																																																												fTemp303 = (iTemp299) as F32;
																																																																												fTemp304 = fTemp298 - fTemp303;
																																																																												fTemp305 = fTemp303 + (1.0 - fTemp298);
																																																																											}
																																																																												if (iSlow2 & (iSlow44 & iSlow62)) | (iSlow2 & (iSlow44 & iSlow47)) != 0 {
																																																																													iTemp306 = i32::wrapping_add(iTemp283, iTemp301);
																																																																												}
																																																																													if iSlow2 & (iSlow44 & iSlow47) != 0 {
																																																																														fTemp307 = fSlow46 * (fTemp288 + fTemp287 * (fTemp284 * (fTemp305 * wt_sample(i32::wrapping_add(iTemp306, iTemp299)) + fTemp304 * wt_sample(i32::wrapping_add(iTemp306, iTemp300))) + fTemp282 * (fTemp305 * wt_sample(i32::wrapping_add(iTemp302, iTemp299)) + fTemp304 * wt_sample(i32::wrapping_add(iTemp302, iTemp300)))));
																																																																													}
																																																																														if iSlow2 & (iSlow44 & iSlow50) != 0 {
																																																																															fTemp308 = fSlow49 * fTemp285 * (1.0 - fTemp287 * (1.0 - F32::sin(6.2831855 * self.fRec0[0] * (7.0 * fTemp287 + 1.0))));
																																																																														}
																																																																															if iSlow2 & (iSlow44 & iSlow53) != 0 {
																																																																																fTemp309 = 2.0 * fTemp287 + 1.0;
																																																																																fTemp310 = 256.0 * fTemp23 * fTemp309;
																																																																																iTemp311 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp310 > self.fConst16) as i32) + ((fTemp310 > self.fConst22) as i32), (fTemp310 > self.fConst21) as i32), (fTemp310 > self.fConst20) as i32), (fTemp310 > self.fConst19) as i32), (fTemp310 > self.fConst18) as i32), (fTemp310 > self.fConst17) as i32);
																																																																																iTemp312 = (iTemp311 >= 4) as i32;
																																																																																iTemp313 = (iTemp311 >= 2) as i32;
																																																																																iTemp314 = (iTemp311 >= 1) as i32;
																																																																																iTemp315 = (iTemp311 >= 3) as i32;
																																																																																iTemp316 = if iTemp312 != 0 {if (iTemp311 >= 6) as i32 != 0 {if (iTemp311 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp311 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp313 != 0 {if iTemp315 != 0 {1792} else {1536}} else {if iTemp314 != 0 {1024} else {0}}};
																																																																																iTemp317 = if iTemp312 != 0 {64} else {if iTemp313 != 0 {if iTemp315 != 0 {128} else {256}} else {if iTemp314 != 0 {512} else {1024}}};
																																																																																fTemp318 = F32::powf(self.fRec0[0], fTemp309) * (iTemp317) as F32;
																																																																																iTemp319 = (fTemp318) as i32;
																																																																																iTemp320 = (i32::wrapping_add(iTemp319, 1)) % iTemp317;
																																																																																fTemp321 = (iTemp319) as F32;
																																																																																fTemp322 = fTemp318 - fTemp321;
																																																																																fTemp323 = fTemp321 + (1.0 - fTemp318);
																																																																																fTemp324 = fSlow52 * (fTemp284 * (fTemp323 * wt_sample(i32::wrapping_add(iTemp316, i32::wrapping_add(iTemp283, iTemp319))) + fTemp322 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp283, iTemp316), iTemp320))) + fTemp282 * (fTemp323 * wt_sample(i32::wrapping_add(iTemp316, i32::wrapping_add(iTemp266, iTemp319))) + fTemp322 * wt_sample(i32::wrapping_add(iTemp320, i32::wrapping_add(iTemp266, iTemp316)))));
																																																																															}
																																																																																if iSlow2 & (iSlow44 & iSlow56) != 0 {
																																																																																	fTemp325 = fSlow55 * (fTemp288 + fTemp287 * F32::sin(7.8539815 * fTemp285));
																																																																																}
																																																																																	if iSlow2 & (iSlow44 & iSlow59) != 0 {
																																																																																		fTemp326 = 0.25 * (fTemp285 * (4.0 * fTemp287 + 1.0) + 1.0);
																																																																																		fTemp327 = fSlow58 * (1.0 - 4.0 * F32::abs(fTemp326 + (-0.5 - F32::floor(fTemp326))));
																																																																																	}
																																																																																		if iSlow2 & (iSlow44 & iSlow62) != 0 {
																																																																																			fTemp328 = 2.0 * self.fRec0[0] * fTemp297;
																																																																																			iTemp329 = (fTemp328) as i32;
																																																																																			iTemp330 = (i32::wrapping_add(iTemp329, 1)) % iTemp296;
																																																																																			fTemp331 = (iTemp329) as F32;
																																																																																			fTemp332 = fTemp328 - fTemp331;
																																																																																			fTemp333 = fTemp331 + (1.0 - fTemp328);
																																																																																			fTemp334 = fSlow61 * (fTemp288 + fTemp287 * (iTemp30) as F32 * F32::abs(fTemp284 * (fTemp333 * wt_sample(i32::wrapping_add(iTemp301, i32::wrapping_add(iTemp283, iTemp329))) + fTemp332 * wt_sample(i32::wrapping_add(iTemp306, iTemp330))) + fTemp282 * (fTemp333 * wt_sample(i32::wrapping_add(iTemp301, i32::wrapping_add(iTemp266, iTemp329))) + fTemp332 * wt_sample(i32::wrapping_add(iTemp330, iTemp302)))));
																																																																																		}
																																																																																			if (iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow68)) | (iSlow2 & (iSlow44 & iSlow65))) != 0 {
																																																																																				fTemp335 = F32::tan(self.fConst25 * F32::powf(fSlow272, fTemp287));
																																																																																				fTemp336 = 1.0 / fTemp335;
																																																																																				fTemp337 = (fTemp336 + 1.4142135) / fTemp335 + 1.0;
																																																																																				self.fRec46[0] = fTemp285 - (self.fRec46[2] * ((fTemp336 + -1.4142135) / fTemp335 + 1.0) + 2.0 * self.fRec46[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp335))) / fTemp337;
																																																																																				fTemp338 = (self.fRec46[2] + self.fRec46[0] + 2.0 * self.fRec46[1]) / fTemp337;
																																																																																			}
																																																																																				if iSlow2 & (iSlow44 & iSlow65) != 0 {
																																																																																					fTemp339 = fSlow64 * fTemp338;
																																																																																				}
																																																																																					if iSlow2 & (iSlow44 & iSlow68) != 0 {
																																																																																						fTemp340 = fSlow67 * fTemp338;
																																																																																					}
																																																																																						if iSlow2 & (iSlow44 & iSlow74) != 0 {
																																																																																							fTemp341 = fSlow73 * fTemp338;
																																																																																						}
																																																																																							if (iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | (iSlow2 & (iSlow44 & iSlow77))) != 0 {
																																																																																								fTemp342 = if iTemp7 != 0 {0.0} else {self.fRec48[1] + self.fConst2 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow277, fTemp287)), 0.0)};
																																																																																								self.fRec48[0] = fTemp342 - F32::floor(fTemp342);
																																																																																								self.fRec47[0] = if (self.fRec48[0] < self.fRec48[1]) as i32 != 0 {fTemp285} else {self.fRec47[1]};
																																																																																							}
																																																																																								if iSlow2 & (iSlow44 & iSlow77) != 0 {
																																																																																									fTemp343 = fSlow76 * self.fRec47[0];
																																																																																								}
																																																																																									if iSlow2 & (iSlow44 & iSlow80) != 0 {
																																																																																										fTemp344 = fSlow79 * self.fRec47[0];
																																																																																									}
																																																																																										if iSlow2 & (iSlow44 & iSlow83) != 0 {
																																																																																											fTemp345 = fSlow82 * self.fRec47[0];
																																																																																										}
																																																																																											if iSlow2 & (iSlow44 & iSlow71) != 0 {
																																																																																												fTemp346 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp287)) + -1.0));
																																																																																												fTemp347 = fSlow70 * (F32::floor(fTemp285 * fTemp346 + 0.5) / fTemp346);
																																																																																											}
																																																																																												if iSlow2 & (iSlow44 & iSlow86) != 0 {
																																																																																													fTemp348 = fSlow85 * fTemp285;
																																																																																												}
																																																																																													if iSlow2 & iSlow44 != 0 {
																																																																																														fTemp349 = fSlow43 * (fSlow282 * fTemp348 + fSlow281 * fTemp347 + fSlow280 * fTemp345 + fSlow279 * fTemp344 + fSlow278 * fTemp343 + fSlow275 * fTemp341 + fSlow274 * fTemp340 + fSlow273 * fTemp339 + fSlow270 * fTemp334 + fSlow269 * fTemp327 + fSlow268 * fTemp325 + fSlow267 * fTemp324 + fSlow266 * fTemp308 + fSlow265 * fTemp307 + fSlow264 * fTemp289);
																																																																																													}
																																																																																														if iSlow2 != 0 {
																																																																																															fTemp350 = fSlow1 * (fSlow283 * fTemp349 + fSlow257 * fTemp260);
																																																																																														}
																																																																																															*output0 = fSlow284 * fTemp350;
																																																																																															self.fVec40[0] = fSlow285;
																																																																																															let mut fTemp351: F32 = if ((fSlow285 != self.fVec40[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec50[1] + -1.0};
																																																																																															self.fRec50[0] = fTemp351;
																																																																																															self.fRec51[0] = if iTemp0 != 0 {fSlow285} else {if (fTemp351 > 0.0) as i32 != 0 {self.fRec51[1] + (fSlow285 - self.fRec51[1]) / F32::max(1.0, fTemp351)} else {fSlow285}};
																																																																																															let mut fTemp352: F32 = F32::abs(self.fRec51[0]);
																																																																																															let mut fTemp353: F32 = F32::exp(0.057762265 * (((self.fRec51[0] > 0.0) as i32) - ((self.fRec51[0] < 0.0) as i32)) as F32 * if (fTemp352 < 0.4) as i32 != 0 {F32::min(0.99, 2.5 * fTemp352)} else {if (fTemp352 < 0.82) as i32 != 0 {F32::min(6.0, F32::floor(16.666666 * (fTemp352 + -0.4))) + 1.0} else {(if (fTemp352 < 0.91) as i32 != 0 {12} else {24}) as F32}});
																																																																																															let mut fTemp354: F32 = fTemp22 * fTemp353;
																																																																																															if (iSlow101 & (iSlow98 & iSlow212)) | ((iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow188)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | ((iSlow101 & (iSlow98 & iSlow179)) | ((iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow173)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | ((iSlow101 & (iSlow143 & iSlow149)) | ((iSlow101 & (iSlow143 & iSlow146)) | (iSlow101 & (iSlow143 & iSlow140))))))))))))))))))))))))) != 0 {
																																																																																																fTemp355 = self.fConst2 * fTemp354;
																																																																																																fTemp356 = fTemp355 + self.fRec49[1];
																																																																																																self.fRec49[0] = if iTemp0 != 0 {fTemp29} else {fTemp356 - F32::floor(fTemp356)};
																																																																																															}
																																																																																																if (iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow143 & iSlow140))) != 0 {
																																																																																																	iTemp357 = (self.fRec49[0] < 0.5) as i32;
																																																																																																}
																																																																																																	if (iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow173)) | ((iSlow101 & (iSlow143 & iSlow170)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow164)) | ((iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | ((iSlow101 & (iSlow143 & iSlow149)) | ((iSlow101 & (iSlow143 & iSlow146)) | (iSlow101 & (iSlow143 & iSlow140)))))))))))) != 0 {
																																																																																																		self.fVec41[0] = fSlow286;
																																																																																																		fTemp358 = if ((fSlow286 != self.fVec41[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec52[1] + -1.0};
																																																																																																		self.fRec52[0] = fTemp358;
																																																																																																		self.fRec53[0] = if iTemp0 != 0 {fSlow286} else {if (fTemp358 > 0.0) as i32 != 0 {self.fRec53[1] + (fSlow286 - self.fRec53[1]) / F32::max(1.0, fTemp358)} else {fSlow286}};
																																																																																																	}
																																																																																																		if (iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow149)) | (iSlow101 & (iSlow143 & iSlow140)))) != 0 {
																																																																																																			fTemp359 = 1.0 - self.fRec53[0];
																																																																																																		}
																																																																																																			if (iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow173)) | ((iSlow101 & (iSlow143 & iSlow170)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow164)) | ((iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | ((iSlow101 & (iSlow143 & iSlow149)) | ((iSlow101 & (iSlow143 & iSlow146)) | (iSlow101 & (iSlow143 & iSlow140)))))))))))) != 0 {
																																																																																																				self.fVec42[0] = fSlow287;
																																																																																																				fTemp360 = if ((fSlow287 != self.fVec42[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec54[1] + -1.0};
																																																																																																				self.fRec54[0] = fTemp360;
																																																																																																				self.fRec55[0] = if iTemp0 != 0 {fSlow287} else {if (fTemp360 > 0.0) as i32 != 0 {self.fRec55[1] + (fSlow287 - self.fRec55[1]) / F32::max(1.0, fTemp360)} else {fSlow287}};
																																																																																																			}
																																																																																																				if iSlow101 & (iSlow143 & iSlow140) != 0 {
																																																																																																					fTemp361 = 1.0 - self.fRec55[0];
																																																																																																					fTemp362 = if iTemp357 != 0 {fTemp361} else {fTemp359};
																																																																																																					fTemp363 = if (fTemp362 < 0.999) as i32 != 0 {F32::max(0.01, fTemp362)} else {1.0};
																																																																																																				}
																																																																																																					if (iSlow101 & (iSlow98 & iSlow182)) | ((iSlow101 & (iSlow143 & iSlow155)) | (iSlow101 & (iSlow143 & iSlow140))) != 0 {
																																																																																																						fTemp364 = 2.0 * self.fRec49[0];
																																																																																																					}
																																																																																																						if iSlow101 & (iSlow143 & iSlow140) != 0 {
																																																																																																							fTemp365 = F32::floor(fTemp364);
																																																																																																							fTemp366 = fTemp364 - (fTemp363 + fTemp365);
																																																																																																							fTemp367 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp366 / fTemp354)));
																																																																																																							fTemp368 = if iTemp357 != 0 {fSlow289} else {fSlow288};
																																																																																																							fTemp369 = if (fTemp368 >= 2.0) as i32 != 0 {(if (fTemp368 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp368 >= 1.0) as i32 != 0 {4.0} else {6.2831855}};
																																																																																																							fTemp370 = fTemp21 * fTemp369 * fTemp19 * fTemp353;
																																																																																																							iTemp371 = (fTemp368) as i32;
																																																																																																							fTemp372 = self.fConst0 * F32::abs(self.fConst11 * (fTemp354 / fTemp363));
																																																																																																							fTemp373 = 0.5 * wt_basic(iTemp371, fTemp372, self.fConst0, 0.999999);
																																																																																																							fTemp374 = self.fConst11 * fTemp354;
																																																																																																							fTemp375 = fTemp364 - fTemp365;
																																																																																																							fTemp376 = ((((fTemp363 < 1.0) as i32) & ((fTemp375 >= fTemp363) as i32)) & ((fTemp366 < fTemp374) as i32)) as F32;
																																																																																																							fTemp377 = 1.0 - fTemp367;
																																																																																																							fTemp378 = if iTemp357 != 0 {fSlow288} else {fSlow289};
																																																																																																							fTemp379 = if iTemp357 != 0 {fTemp359} else {fTemp361};
																																																																																																							fTemp380 = if (fTemp379 < 0.999) as i32 != 0 {F32::max(0.01, fTemp379)} else {1.0};
																																																																																																							fTemp381 = ((fTemp380 == 1.0) as i32) as u32 as F32;
																																																																																																							fTemp382 = 0.5 * (wt_basic(iTemp371, fTemp372, self.fConst0, 0.0) - fTemp381 * wt_basic((fTemp378) as i32, self.fConst0 * F32::abs(self.fConst11 * (fTemp354 / fTemp380)), self.fConst0, 0.999999));
																																																																																																							fTemp383 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp375 / fTemp354)));
																																																																																																							fTemp384 = 1.0 - fTemp383;
																																																																																																							fTemp385 = fTemp369 / fTemp363 - fTemp381 * if (fTemp378 >= 2.0) as i32 != 0 {(if (fTemp378 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp378 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} / fTemp380;
																																																																																																							fTemp386 = ((fTemp375 < fTemp374) as i32) as u32 as F32;
																																																																																																							self.fVec43[0] = fTemp386 * OscDsp_faustpower2_f(fTemp384) * (self.fConst10 * fTemp354 * fTemp385 * fTemp384 - fTemp382) + fTemp376 * OscDsp_faustpower2_f(fTemp377) * (fTemp373 - self.fConst10 * (fTemp370 * fTemp377 / fTemp363));
																																																																																																							self.fVec44[0] = ((fTemp375 < fTemp363) as i32) as u32 as F32 * wt_basic(iTemp371, fTemp372, self.fConst0, fTemp375 / fTemp363);
																																																																																																							fTemp387 = fSlow139 * (self.fVec44[1] + fTemp386 * OscDsp_faustpower2_f(fTemp383) * (fTemp382 + self.fConst10 * fTemp354 * fTemp383 * fTemp385) + self.fVec43[1] - fTemp376 * OscDsp_faustpower2_f(fTemp367) * (fTemp373 + self.fConst10 * (fTemp370 * fTemp367 / fTemp363)));
																																																																																																						}
																																																																																																							if iSlow101 & (iSlow143 & iSlow146) != 0 {
																																																																																																								fTemp388 = self.fConst12 * fTemp354 * F32::exp(0.0046209814 * self.fRec53[0]);
																																																																																																								fTemp389 = fTemp388 + self.fRec56[1];
																																																																																																								self.fRec56[0] = if iTemp0 != 0 {fTemp65} else {fTemp389 - F32::floor(fTemp389)};
																																																																																																							}
																																																																																																								if (iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow170)) | ((iSlow101 & (iSlow143 & iSlow158)) | (iSlow101 & (iSlow143 & iSlow146)))) != 0 {
																																																																																																									fTemp390 = F32::max(0.0, F32::min(1.0, self.fRec55[0]));
																																																																																																								}
																																																																																																									if iSlow101 & (iSlow143 & iSlow146) != 0 {
																																																																																																										fTemp391 = 4.0 * fTemp390;
																																																																																																										fTemp392 = F32::min(3.0, fTemp391);
																																																																																																									}
																																																																																																										if (iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow173)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | (iSlow101 & (iSlow143 & iSlow146)))))))) != 0 {
																																																																																																											fTemp393 = F32::abs(fTemp355);
																																																																																																										}
																																																																																																											if (iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | (iSlow101 & (iSlow143 & iSlow146))))))) != 0 {
																																																																																																												fTemp394 = self.fConst0 * fTemp393;
																																																																																																											}
																																																																																																												if iSlow101 & (iSlow143 & iSlow146) != 0 {
																																																																																																													fTemp395 = wt_basic(2, fTemp394, self.fConst0, self.fRec49[0]);
																																																																																																													self.fVec45[0] = wt_basic(0, fTemp394, self.fConst0, self.fRec49[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp392)) + wt_basic(1, fTemp394, self.fConst0, self.fRec49[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp392 + -1.0)) + fTemp395 * F32::max(0.0, 1.0 - F32::abs(fTemp392 + -2.0)) + (fTemp395 - wt_basic(2, fTemp394, self.fConst0, self.fRec49[0] + (-0.5 - 0.42 * F32::max(0.0, fTemp391 + -3.0)))) * F32::max(0.0, 1.0 - F32::abs(fTemp392 + -3.0)) + self.fRec53[0] * wt_basic(iSlow291, self.fConst0 * F32::abs(fTemp388), self.fConst0, self.fRec56[0]);
																																																																																																													fTemp396 = fSlow145 * (self.fVec45[1] / F32::sqrt(OscDsp_faustpower2_f(self.fRec53[0]) + 1.0));
																																																																																																												}
																																																																																																													if (iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | (iSlow101 & (iSlow143 & iSlow149))) != 0 {
																																																																																																														iTemp397 = (self.fRec49[0] < self.fRec49[1]) as i32;
																																																																																																													}
																																																																																																														if (iSlow101 & (iSlow143 & iSlow152)) | (iSlow101 & (iSlow143 & iSlow149)) != 0 {
																																																																																																															self.iRec57[0] = (i32::wrapping_add(iTemp397, self.iRec57[1])) % 2;
																																																																																																														}
																																																																																																															if iSlow101 & (iSlow143 & iSlow149) != 0 {
																																																																																																																self.fVec46[0] = wt_basic(iSlow291, self.fConst0 * F32::abs(self.fConst12 * fTemp354), self.fConst0, 0.5 * (self.fRec49[0] + (self.iRec57[0]) as F32));
																																																																																																															}
																																																																																																																if (iSlow101 & (iSlow143 & iSlow155)) | (iSlow101 & (iSlow143 & iSlow149)) != 0 {
																																																																																																																	fTemp398 = 15.0 * self.fRec55[0];
																																																																																																																	fTemp399 = fTemp398 + 1.0;
																																																																																																																	fTemp400 = fTemp354 * fTemp399;
																																																																																																																	fTemp401 = self.fConst0 * F32::abs(self.fConst2 * fTemp400);
																																																																																																																	fTemp402 = wt_basic(iSlow293, fTemp401, self.fConst0, fTemp399);
																																																																																																																	fTemp403 = wt_basic(iSlow293, fTemp401, self.fConst0, 0.0);
																																																																																																																}
																																																																																																																	if iSlow101 & (iSlow143 & iSlow149) != 0 {
																																																																																																																		fTemp404 = 0.5 * (fTemp403 - fTemp402);
																																																																																																																	}
																																																																																																																		if (iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | (iSlow101 & (iSlow143 & iSlow149))) != 0 {
																																																																																																																			fTemp405 = F32::max(0.0, F32::min(1.0, self.fConst0 * (self.fRec49[0] / fTemp354)));
																																																																																																																			fTemp406 = 1.0 - fTemp405;
																																																																																																																		}
																																																																																																																			if iSlow101 & (iSlow143 & iSlow149) != 0 {
																																																																																																																				fTemp407 = fTemp398 + (1.0 - F32::floor(fTemp399));
																																																																																																																				fTemp408 = fSlow298 - if iSlow294 != 0 {fSlow297} else {if iSlow295 != 0 {(i32::wrapping_sub(4, i32::wrapping_mul(8, ((fTemp407 >= 0.25) as i32) & ((fTemp407 < 0.75) as i32)))) as F32} else {6.2831855 * F32::cos(6.2831855 * fTemp407)}};
																																																																																																																			}
																																																																																																																				if (iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | (iSlow101 & (iSlow143 & iSlow149))) != 0 {
																																																																																																																					fTemp409 = (iTemp397) as F32;
																																																																																																																					fTemp410 = fTemp409 * OscDsp_faustpower2_f(fTemp406);
																																																																																																																				}
																																																																																																																					if iSlow101 & (iSlow143 & iSlow149) != 0 {
																																																																																																																						self.fVec47[0] = fTemp410 * (self.fConst13 * fTemp400 * fTemp408 * fTemp406 - fTemp404);
																																																																																																																					}
																																																																																																																						if (iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | (iSlow101 & (iSlow143 & iSlow149))) != 0 {
																																																																																																																							fTemp411 = fTemp409 * OscDsp_faustpower2_f(fTemp405);
																																																																																																																						}
																																																																																																																							if (iSlow101 & (iSlow143 & iSlow155)) | (iSlow101 & (iSlow143 & iSlow149)) != 0 {
																																																																																																																								fTemp412 = wt_basic(iSlow293, fTemp401, self.fConst0, self.fRec49[0] * fTemp399);
																																																																																																																								self.fVec48[0] = fTemp412;
																																																																																																																							}
																																																																																																																								if iSlow101 & (iSlow143 & iSlow149) != 0 {
																																																																																																																									fTemp413 = fSlow148 * (fTemp359 * (self.fVec48[1] + fTemp411 * (fTemp404 + self.fConst13 * fTemp400 * fTemp405 * fTemp408) + self.fVec47[1]) + self.fRec53[0] * self.fVec46[1]);
																																																																																																																								}
																																																																																																																									if iSlow101 & (iSlow143 & iSlow152) != 0 {
																																																																																																																										fTemp414 = if self.iRec57[0] != 0 {fSlow289} else {fSlow288};
																																																																																																																										fTemp415 = 7.0 * self.fRec55[0] + 1.0;
																																																																																																																										fTemp416 = fTemp415 * (self.fRec53[0] + 1.0);
																																																																																																																										fTemp417 = if self.iRec57[0] != 0 {fTemp415} else {fTemp416};
																																																																																																																										fTemp418 = if self.iRec57[0] != 0 {fSlow288} else {fSlow289};
																																																																																																																										iTemp419 = (fTemp418) as i32;
																																																																																																																										fTemp420 = if self.iRec57[0] != 0 {fTemp416} else {fTemp415};
																																																																																																																										fTemp421 = 0.5 * (fTemp420 * wt_basic(iTemp419, fTemp394, self.fConst0, 0.0) - fTemp417 * wt_basic((fTemp414) as i32, fTemp394, self.fConst0, 0.999999));
																																																																																																																										fTemp422 = if (fTemp418 >= 2.0) as i32 != 0 {(if (fTemp418 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp418 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp420 - if (fTemp414 >= 2.0) as i32 != 0 {(if (fTemp414 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp414 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp417;
																																																																																																																										self.fVec49[0] = fTemp410 * (self.fConst13 * fTemp354 * fTemp406 * fTemp422 - fTemp421);
																																																																																																																										self.fVec50[0] = wt_basic(iTemp419, fTemp394, self.fConst0, self.fRec49[0]) * fTemp420;
																																																																																																																										fTemp423 = self.fVec50[1] + fTemp411 * (fTemp421 + self.fConst13 * fTemp354 * fTemp405 * fTemp422) + self.fVec49[1];
																																																																																																																										self.fVec51[0] = fTemp423;
																																																																																																																										fTemp424 = fTemp423 - self.fVec51[1];
																																																																																																																										iTemp425 = (F32::abs(fTemp424) < 0.0001) as i32;
																																																																																																																										fTemp426 = 4.0 * F32::floor(0.25 * (self.fVec51[1] + 1.0));
																																																																																																																										fTemp427 = self.fVec51[1] - fTemp426;
																																																																																																																										fTemp428 = 4.0 * F32::floor(0.25 * (fTemp423 + 1.0));
																																																																																																																										fTemp429 = fTemp423 - fTemp428;
																																																																																																																										fTemp430 = 0.125 * (fTemp423 + self.fVec51[1]);
																																																																																																																										fTemp431 = fSlow151 * if iTemp425 != 0 {1.0 - 4.0 * F32::abs(fTemp430 + (-0.25 - F32::floor(fTemp430 + 0.25)))} else {(if (fTemp429 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp429)} else {1.0 - 0.5 * OscDsp_faustpower2_f(fTemp423 + (-2.0 - fTemp428))} - if (fTemp427 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp427)} else {1.0 - 0.5 * OscDsp_faustpower2_f(self.fVec51[1] + (-2.0 - fTemp426))}) / if iTemp425 != 0 {1.0} else {fTemp424}};
																																																																																																																									}
																																																																																																																										if iSlow101 & (iSlow143 & iSlow155) != 0 {
																																																																																																																											fTemp432 = 4.0 * F32::max(0.0, F32::min(1.0, self.fRec53[0]));
																																																																																																																											iTemp433 = (F32::min(3.0, fTemp432)) as i32;
																																																																																																																											iTemp434 = i32::wrapping_add(iTemp433, 1);
																																																																																																																											iTemp435 = (iTemp434 >= 3) as i32;
																																																																																																																											iTemp436 = (iTemp434 >= 2) as i32;
																																																																																																																											iTemp437 = (iTemp434 >= 1) as i32;
																																																																																																																											iTemp438 = (iTemp434 >= 4) as i32;
																																																																																																																											fTemp439 = 0.5 * (wt_basic(iSlow291, fTemp394, self.fConst0, 0.999999) + 1.0);
																																																																																																																											fTemp440 = (iTemp433) as F32;
																																																																																																																											fTemp441 = fTemp432 - fTemp440;
																																																																																																																											iTemp442 = (iTemp433 >= 3) as i32;
																																																																																																																											iTemp443 = (iTemp433 >= 2) as i32;
																																																																																																																											iTemp444 = (iTemp433 >= 1) as i32;
																																																																																																																											iTemp445 = (iTemp433 >= 4) as i32;
																																																																																																																											fTemp446 = fTemp440 + (1.0 - fTemp432);
																																																																																																																											fTemp447 = 0.5 * (wt_basic(iSlow291, fTemp394, self.fConst0, 0.0) + 1.0);
																																																																																																																											fTemp448 = fTemp403 * (fTemp446 * if iTemp442 != 0 {if iTemp445 != 0 {fTemp447} else {0.0}} else {if iTemp443 != 0 {0.0} else {(if iTemp444 != 0 {0} else {1}) as F32}} + fTemp441 * if iTemp435 != 0 {if iTemp438 != 0 {fTemp447} else {0.0}} else {if iTemp436 != 0 {0.0} else {(if iTemp437 != 0 {0} else {1}) as F32}}) - fTemp402 * (fTemp446 * if iTemp442 != 0 {if iTemp445 != 0 {fTemp439} else {6e-06}} else {if iTemp443 != 0 {0.0} else {if iTemp444 != 0 {2e-06} else {1e-06}}} + fTemp441 * if iTemp435 != 0 {if iTemp438 != 0 {fTemp439} else {6e-06}} else {if iTemp436 != 0 {0.0} else {if iTemp437 != 0 {2e-06} else {1e-06}}});
																																																																																																																											self.fVec52[0] = fTemp410 * fTemp448;
																																																																																																																										}
																																																																																																																											if (iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow143 & iSlow155)) != 0 {
																																																																																																																												fTemp449 = 1.0 - self.fRec49[0];
																																																																																																																											}
																																																																																																																												if iSlow101 & (iSlow143 & iSlow155) != 0 {
																																																																																																																													fTemp450 = 1.0 - F32::abs(fTemp364 + -1.0);
																																																																																																																													fTemp451 = F32::sin(3.1415927 * self.fRec49[0]);
																																																																																																																													fTemp452 = F32::min(1.0, 6.0 * F32::min(self.fRec49[0], fTemp449));
																																																																																																																													fTemp453 = 0.5 * (wt_basic(iSlow291, fTemp394, self.fConst0, self.fRec49[0]) + 1.0);
																																																																																																																													self.fVec53[0] = fTemp412 * (fTemp446 * if iTemp442 != 0 {if iTemp445 != 0 {fTemp453} else {fTemp452}} else {if iTemp443 != 0 {fTemp451} else {if iTemp444 != 0 {fTemp450} else {fTemp449}}} + fTemp441 * if iTemp435 != 0 {if iTemp438 != 0 {fTemp453} else {fTemp452}} else {if iTemp436 != 0 {fTemp451} else {if iTemp437 != 0 {fTemp450} else {fTemp449}}});
																																																																																																																													fTemp454 = fSlow154 * (self.fVec53[1] + 0.5 * (fTemp411 * fTemp448 - self.fVec52[1]));
																																																																																																																												}
																																																																																																																													if iSlow101 & (iSlow143 & iSlow158) != 0 {
																																																																																																																														fTemp455 = self.fConst2 * fTemp354 * (2.17 * fTemp390 + 3.0);
																																																																																																																														fTemp456 = fTemp455 + self.fRec58[1];
																																																																																																																														self.fRec58[0] = if iTemp0 != 0 {fTemp134} else {fTemp456 - F32::floor(fTemp456)};
																																																																																																																														self.fVec54[0] = wt_basic(iSlow291, self.fConst0 * F32::abs(fTemp455), self.fConst0, self.fRec58[0]);
																																																																																																																														fTemp457 = self.fConst2 * fTemp354 * (1.41 * fTemp390 + 2.0);
																																																																																																																														fTemp458 = fTemp457 + self.fRec59[1];
																																																																																																																														self.fRec59[0] = if iTemp0 != 0 {fTemp138} else {fTemp458 - F32::floor(fTemp458)};
																																																																																																																														self.fVec55[0] = wt_basic(iSlow291, self.fConst0 * F32::abs(fTemp457), self.fConst0, self.fRec59[0]);
																																																																																																																													}
																																																																																																																														if (iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow161)) | (iSlow101 & (iSlow143 & iSlow158)))) != 0 {
																																																																																																																															fTemp459 = wt_basic(iSlow293, fTemp394, self.fConst0, self.fRec49[0]);
																																																																																																																															self.fVec56[0] = fTemp459;
																																																																																																																														}
																																																																																																																															if iSlow101 & (iSlow143 & iSlow158) != 0 {
																																																																																																																																fTemp460 = fSlow157 * (0.57735026 * fTemp359 * (self.fVec56[1] + self.fVec55[1] + self.fVec54[1]) + 1.5 * self.fRec53[0] * self.fVec56[1] * self.fVec55[1] * self.fVec54[1]);
																																																																																																																															}
																																																																																																																																if iSlow101 & (iSlow143 & iSlow161) != 0 {
																																																																																																																																	iTemp461 = (F32::max(0.0, F32::min(1e+01, 10.999 * self.fRec55[0]))) as i32;
																																																																																																																																	fTemp462 = unsafe { ftbl1OscDspSIG1[(i32::wrapping_add(iTemp461, 1)) as usize] };
																																																																																																																																	fTemp463 = self.fConst2 * fTemp354 * fTemp462;
																																																																																																																																	fTemp464 = fTemp463 + self.fRec60[1];
																																																																																																																																	fTemp465 = fSlow136 * fTemp27 * fTemp462;
																																																																																																																																	fTemp466 = F32::floor(fTemp465);
																																																																																																																																	self.fRec60[0] = if iTemp0 != 0 {fTemp465 - (fTemp466 + F32::floor(fTemp465 - fTemp466))} else {fTemp464 - F32::floor(fTemp464)};
																																																																																																																																	self.fVec57[0] = wt_basic(iSlow291, self.fConst0 * F32::abs(fTemp463), self.fConst0, self.fRec60[0]);
																																																																																																																																	fTemp467 = unsafe { ftbl1OscDspSIG1[iTemp461 as usize] };
																																																																																																																																	fTemp468 = self.fConst2 * fTemp354 * fTemp467;
																																																																																																																																	fTemp469 = fTemp468 + self.fRec61[1];
																																																																																																																																	fTemp470 = fSlow136 * fTemp27 * fTemp467;
																																																																																																																																	fTemp471 = F32::floor(fTemp470);
																																																																																																																																	self.fRec61[0] = if iTemp0 != 0 {fTemp470 - (fTemp471 + F32::floor(fTemp470 - fTemp471))} else {fTemp469 - F32::floor(fTemp469)};
																																																																																																																																	self.fVec58[0] = wt_basic(iSlow291, self.fConst0 * F32::abs(fTemp468), self.fConst0, self.fRec61[0]);
																																																																																																																																	fTemp472 = fSlow160 * self.fVec56[1] * (fTemp359 * self.fVec58[1] + self.fRec53[0] * self.fVec57[1]);
																																																																																																																																}
																																																																																																																																	if iSlow101 & (iSlow143 & iSlow164) != 0 {
																																																																																																																																		fTemp473 = unsafe { ftbl2OscDspSIG2[((F32::max(0.0, F32::min(9.0, 9.999 * self.fRec53[0]))) as i32) as usize] };
																																																																																																																																		fTemp474 = self.fConst2 * fTemp354 * fTemp473;
																																																																																																																																		fTemp475 = F32::max(F32::abs(fTemp474), 1e-09);
																																																																																																																																		iTemp476 = (fTemp474 < 0.0) as i32;
																																																																																																																																		fTemp477 = fTemp474 + self.fRec62[1];
																																																																																																																																		fTemp478 = fSlow136 * fTemp27 * fTemp473;
																																																																																																																																		fTemp479 = F32::floor(fTemp478);
																																																																																																																																		self.fRec62[0] = if iTemp0 != 0 {fTemp478 - (fTemp479 + F32::floor(fTemp478 - fTemp479))} else {fTemp477 - F32::floor(fTemp477)};
																																																																																																																																		fTemp480 = if iTemp476 != 0 {fSlow308 - (self.fRec62[0] + F32::floor(fSlow308 - self.fRec62[0]))} else {self.fRec62[0] - (fSlow308 + F32::floor(self.fRec62[0] - fSlow308))};
																																																																																																																																		iTemp481 = ((fTemp480 < fTemp475) as i32) * if iTemp476 != 0 {-iSlow306} else {iSlow306};
																																																																																																																																		fTemp482 = if iTemp476 != 0 {fSlow310 - (self.fRec62[0] + F32::floor(fSlow310 - self.fRec62[0]))} else {self.fRec62[0] - (fSlow310 + F32::floor(self.fRec62[0] - fSlow310))};
																																																																																																																																		iTemp483 = ((fTemp482 < fTemp475) as i32) * if iTemp476 != 0 {-iSlow309} else {iSlow309};
																																																																																																																																		fTemp484 = F32::floor(self.fRec62[0]);
																																																																																																																																		fTemp485 = self.fRec62[0] - fTemp484;
																																																																																																																																		fTemp486 = if iSlow304 != 0 {if iSlow305 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp485 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec62[0] + (0.5 - (fTemp484 + F32::floor(self.fRec62[0] + (0.5 - fTemp484))))) + -1.0}} else {if iSlow307 != 0 {1.0 - 4.0 * F32::abs(self.fRec62[0] + (-0.25 - (fTemp484 + F32::floor(self.fRec62[0] + (0.25 - fTemp484)))))} else {F32::sin(6.2831855 * fTemp485)}};
																																																																																																																																		self.fVec59[0] = fTemp486;
																																																																																																																																	}
																																																																																																																																		if (iSlow101 & (iSlow143 & iSlow167)) | (iSlow101 & (iSlow143 & iSlow164)) != 0 {
																																																																																																																																			fTemp487 = OscDsp_faustpower2_f(self.fRec55[0]);
																																																																																																																																		}
																																																																																																																																			if iSlow101 & (iSlow143 & iSlow164) != 0 {
																																																																																																																																				fTemp488 = self.fConst2 * fTemp354 * (8.0 * fTemp487 * fTemp473 * (0.5 * (fTemp486 + self.fVec59[1] - (i32::wrapping_add(iTemp483, iTemp481)) as F32) + (fTemp482 * (iTemp483) as F32 + fTemp480 * (iTemp481) as F32) / fTemp475) + 1.0);
																																																																																																																																				iTemp489 = (fTemp488 < 0.0) as i32;
																																																																																																																																				fTemp490 = 0.5 * (if iTemp489 != 0 {-iSlow311} else {iSlow311}) as F32;
																																																																																																																																				fTemp491 = F32::max(F32::abs(fTemp488), 1e-09);
																																																																																																																																				fTemp492 = self.fRec63[1] + fTemp488;
																																																																																																																																				self.fRec63[0] = if iTemp0 != 0 {fTemp29} else {fTemp492 - F32::floor(fTemp492)};
																																																																																																																																				fTemp493 = if iTemp489 != 0 {fSlow312 - (self.fRec63[0] + F32::floor(fSlow312 - self.fRec63[0]))} else {self.fRec63[0] - (fSlow312 + F32::floor(self.fRec63[0] - fSlow312))};
																																																																																																																																				fTemp494 = F32::max(0.0, F32::min(1.0, fTemp493 / fTemp491));
																																																																																																																																				fTemp495 = 1.0 - fTemp494;
																																																																																																																																				fTemp496 = ((fTemp493 < fTemp491) as i32) as u32 as F32;
																																																																																																																																				fTemp497 = 0.5 * (if iTemp489 != 0 {-iSlow314} else {iSlow314}) as F32;
																																																																																																																																				fTemp498 = if iTemp489 != 0 {fSlow315 - (self.fRec63[0] + F32::floor(fSlow315 - self.fRec63[0]))} else {self.fRec63[0] - (fSlow315 + F32::floor(self.fRec63[0] - fSlow315))};
																																																																																																																																				fTemp499 = F32::max(0.0, F32::min(1.0, fTemp498 / fTemp491));
																																																																																																																																				fTemp500 = 1.0 - fTemp499;
																																																																																																																																				fTemp501 = ((fTemp498 < fTemp491) as i32) as u32 as F32;
																																																																																																																																				self.fVec60[0] = fTemp501 * OscDsp_faustpower2_f(fTemp500) * (0.16666667 * fTemp491 * fSlow316 * fTemp500 - fTemp497) + fTemp496 * OscDsp_faustpower2_f(fTemp495) * (0.16666667 * fTemp491 * fSlow313 * fTemp495 - fTemp490);
																																																																																																																																				fTemp502 = F32::floor(self.fRec63[0]);
																																																																																																																																				fTemp503 = self.fRec63[0] - fTemp502;
																																																																																																																																				self.fVec61[0] = if iSlow294 != 0 {if iSlow296 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp503 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec63[0] + (0.5 - (fTemp502 + F32::floor(self.fRec63[0] + (0.5 - fTemp502))))) + -1.0}} else {if iSlow295 != 0 {1.0 - 4.0 * F32::abs(self.fRec63[0] + (-0.25 - (fTemp502 + F32::floor(self.fRec63[0] + (0.25 - fTemp502)))))} else {F32::sin(6.2831855 * fTemp503)}};
																																																																																																																																				fTemp504 = fSlow163 * (fTemp496 * OscDsp_faustpower2_f(fTemp494) * (fTemp490 + 0.16666667 * fTemp491 * fTemp494 * fSlow313) + self.fVec61[1] + fTemp501 * OscDsp_faustpower2_f(fTemp499) * (fTemp497 + 0.16666667 * fTemp491 * fTemp499 * fSlow316) + self.fVec60[1]);
																																																																																																																																			}
																																																																																																																																				if iSlow101 & (iSlow143 & iSlow167) != 0 {
																																																																																																																																					fTemp505 = F32::max(1e+01, F32::min(self.fConst14, fTemp354 * F32::exp(0.6931472 * (8.0 * self.fRec53[0] + -3.0))));
																																																																																																																																					fTemp506 = 1.0 / F32::tan(self.fConst8 * fTemp505);
																																																																																																																																					self.fRec64[0] = (fTemp187 - self.fRec64[1] * (1.0 - fTemp506)) / (fTemp506 + 1.0);
																																																																																																																																					fTemp507 = fSlow166 * (self.fVec56[1] * (1.5 * self.fRec55[0] * self.fRec64[0] * F32::sqrt(self.fConst15 / fTemp505) + 1.0) / F32::sqrt(2.25 * fTemp487 + 1.0));
																																																																																																																																				}
																																																																																																																																					if iSlow101 & (iSlow143 & iSlow170) != 0 {
																																																																																																																																						fTemp508 = 0.6 * fTemp390 * self.fRec66[1] + 1.0;
																																																																																																																																						fTemp509 = self.fConst2 * fTemp354 * fTemp508;
																																																																																																																																						fTemp510 = self.fRec65[1] + fTemp509;
																																																																																																																																						iTemp511 = (fTemp510 >= 1.0) as i32;
																																																																																																																																						self.fRec65[0] = if iTemp0 != 0 {fTemp29} else {fTemp510 - (iTemp511) as F32};
																																																																																																																																						self.fRec66[0] = if iTemp511 != 0 {fTemp15} else {self.fRec66[1]};
																																																																																																																																						fTemp512 = F32::max(F32::abs(fTemp509), 1e-09);
																																																																																																																																						iTemp513 = (fTemp509 < 0.0) as i32;
																																																																																																																																						fTemp514 = if iTemp513 != 0 {fSlow308 - (self.fRec65[0] + F32::floor(fSlow308 - self.fRec65[0]))} else {self.fRec65[0] - (fSlow308 + F32::floor(self.fRec65[0] - fSlow308))};
																																																																																																																																						iTemp515 = ((fTemp514 < fTemp512) as i32) * if iTemp513 != 0 {-iSlow306} else {iSlow306};
																																																																																																																																						fTemp516 = if iTemp513 != 0 {fSlow310 - (self.fRec65[0] + F32::floor(fSlow310 - self.fRec65[0]))} else {self.fRec65[0] - (fSlow310 + F32::floor(self.fRec65[0] - fSlow310))};
																																																																																																																																						iTemp517 = ((fTemp516 < fTemp512) as i32) * if iTemp513 != 0 {-iSlow309} else {iSlow309};
																																																																																																																																						fTemp518 = F32::floor(self.fRec65[0]);
																																																																																																																																						fTemp519 = self.fRec65[0] - fTemp518;
																																																																																																																																						fTemp520 = if iSlow304 != 0 {if iSlow305 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp519 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec65[0] + (0.5 - (fTemp518 + F32::floor(self.fRec65[0] + (0.5 - fTemp518))))) + -1.0}} else {if iSlow307 != 0 {1.0 - 4.0 * F32::abs(self.fRec65[0] + (-0.25 - (fTemp518 + F32::floor(self.fRec65[0] + (0.25 - fTemp518)))))} else {F32::sin(6.2831855 * fTemp519)}};
																																																																																																																																						self.fVec62[0] = fTemp520;
																																																																																																																																						fTemp521 = self.fConst2 * fTemp354 * (4.0 * self.fRec53[0] * fTemp508 * (0.5 * (fTemp520 + self.fVec62[1] - (i32::wrapping_add(iTemp517, iTemp515)) as F32) + (fTemp516 * (iTemp517) as F32 + fTemp514 * (iTemp515) as F32) / fTemp512) + 1.0);
																																																																																																																																						iTemp522 = (fTemp521 < 0.0) as i32;
																																																																																																																																						fTemp523 = 0.5 * (if iTemp522 != 0 {-iSlow311} else {iSlow311}) as F32;
																																																																																																																																						fTemp524 = F32::max(F32::abs(fTemp521), 1e-09);
																																																																																																																																						fTemp525 = self.fRec67[1] + fTemp521;
																																																																																																																																						self.fRec67[0] = if iTemp0 != 0 {fTemp29} else {fTemp525 - F32::floor(fTemp525)};
																																																																																																																																						fTemp526 = if iTemp522 != 0 {fSlow312 - (self.fRec67[0] + F32::floor(fSlow312 - self.fRec67[0]))} else {self.fRec67[0] - (fSlow312 + F32::floor(self.fRec67[0] - fSlow312))};
																																																																																																																																						fTemp527 = F32::max(0.0, F32::min(1.0, fTemp526 / fTemp524));
																																																																																																																																						fTemp528 = 1.0 - fTemp527;
																																																																																																																																						fTemp529 = fSlow313 * fTemp524;
																																																																																																																																						fTemp530 = ((fTemp526 < fTemp524) as i32) as u32 as F32;
																																																																																																																																						fTemp531 = 0.5 * (if iTemp522 != 0 {-iSlow314} else {iSlow314}) as F32;
																																																																																																																																						fTemp532 = if iTemp522 != 0 {fSlow315 - (self.fRec67[0] + F32::floor(fSlow315 - self.fRec67[0]))} else {self.fRec67[0] - (fSlow315 + F32::floor(self.fRec67[0] - fSlow315))};
																																																																																																																																						fTemp533 = F32::max(0.0, F32::min(1.0, fTemp532 / fTemp524));
																																																																																																																																						fTemp534 = 1.0 - fTemp533;
																																																																																																																																						fTemp535 = fSlow316 * fTemp524;
																																																																																																																																						fTemp536 = ((fTemp532 < fTemp524) as i32) as u32 as F32;
																																																																																																																																						self.fVec63[0] = fTemp536 * OscDsp_faustpower2_f(fTemp534) * (0.16666667 * fTemp535 * fTemp534 - fTemp531) + fTemp530 * OscDsp_faustpower2_f(fTemp528) * (0.16666667 * fTemp529 * fTemp528 - fTemp523);
																																																																																																																																						fTemp537 = F32::floor(self.fRec67[0]);
																																																																																																																																						fTemp538 = self.fRec67[0] - fTemp537;
																																																																																																																																						self.fVec64[0] = if iSlow294 != 0 {if iSlow296 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp538 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec67[0] + (0.5 - (fTemp537 + F32::floor(self.fRec67[0] + (0.5 - fTemp537))))) + -1.0}} else {if iSlow295 != 0 {1.0 - 4.0 * F32::abs(self.fRec67[0] + (-0.25 - (fTemp537 + F32::floor(self.fRec67[0] + (0.25 - fTemp537)))))} else {F32::sin(6.2831855 * fTemp538)}};
																																																																																																																																						fTemp539 = fSlow169 * (fTemp530 * OscDsp_faustpower2_f(fTemp527) * (fTemp523 + 0.16666667 * fTemp529 * fTemp527) + self.fVec64[1] + fTemp536 * OscDsp_faustpower2_f(fTemp533) * (fTemp531 + 0.16666667 * fTemp535 * fTemp533) + self.fVec63[1]);
																																																																																																																																					}
																																																																																																																																						if iSlow101 & (iSlow143 & iSlow173) != 0 {
																																																																																																																																							iTemp540 = (fTemp355 < 0.0) as i32;
																																																																																																																																							fTemp541 = 0.5 * (if iTemp540 != 0 {-iSlow311} else {iSlow311}) as F32;
																																																																																																																																							fTemp542 = F32::max(fTemp393, 1e-09);
																																																																																																																																							fTemp543 = if iTemp540 != 0 {fSlow312 - (self.fRec49[0] + F32::floor(fSlow312 - self.fRec49[0]))} else {self.fRec49[0] - (fSlow312 + F32::floor(self.fRec49[0] - fSlow312))};
																																																																																																																																							fTemp544 = F32::max(0.0, F32::min(1.0, fTemp543 / fTemp542));
																																																																																																																																							fTemp545 = 1.0 - fTemp544;
																																																																																																																																							fTemp546 = fSlow313 * fTemp542;
																																																																																																																																							fTemp547 = ((fTemp543 < fTemp542) as i32) as u32 as F32;
																																																																																																																																							fTemp548 = 0.5 * (if iTemp540 != 0 {-iSlow314} else {iSlow314}) as F32;
																																																																																																																																							fTemp549 = if iTemp540 != 0 {fSlow315 - (self.fRec49[0] + F32::floor(fSlow315 - self.fRec49[0]))} else {self.fRec49[0] - (fSlow315 + F32::floor(self.fRec49[0] - fSlow315))};
																																																																																																																																							fTemp550 = F32::max(0.0, F32::min(1.0, fTemp549 / fTemp542));
																																																																																																																																							fTemp551 = 1.0 - fTemp550;
																																																																																																																																							fTemp552 = fSlow316 * fTemp542;
																																																																																																																																							fTemp553 = ((fTemp549 < fTemp542) as i32) as u32 as F32;
																																																																																																																																							self.fVec65[0] = fTemp553 * OscDsp_faustpower2_f(fTemp551) * (0.16666667 * fTemp552 * fTemp551 - fTemp548) + fTemp547 * OscDsp_faustpower2_f(fTemp545) * (0.16666667 * fTemp546 * fTemp545 - fTemp541);
																																																																																																																																							fTemp554 = F32::floor(self.fRec49[0]);
																																																																																																																																							fTemp555 = self.fRec49[0] - fTemp554;
																																																																																																																																							self.fVec66[0] = if iSlow294 != 0 {if iSlow296 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp555 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec49[0] + (0.5 - (fTemp554 + F32::floor(self.fRec49[0] + (0.5 - fTemp554))))) + -1.0}} else {if iSlow295 != 0 {1.0 - 4.0 * F32::abs(self.fRec49[0] + (-0.25 - (fTemp554 + F32::floor(self.fRec49[0] + (0.25 - fTemp554)))))} else {F32::sin(6.2831855 * fTemp555)}};
																																																																																																																																							fTemp556 = (9.0 * self.fRec55[0] + 1.0) * (1.5 * self.fRec53[0] + fTemp547 * OscDsp_faustpower2_f(fTemp544) * (fTemp541 + 0.16666667 * fTemp546 * fTemp544) + self.fVec66[1] + fTemp553 * OscDsp_faustpower2_f(fTemp550) * (fTemp548 + 0.16666667 * fTemp552 * fTemp550) + self.fVec65[1]);
																																																																																																																																							self.fVec67[0] = fTemp556;
																																																																																																																																							fTemp557 = fTemp556 - self.fVec67[1];
																																																																																																																																							iTemp558 = (F32::abs(fTemp557) < 0.0001) as i32;
																																																																																																																																							fTemp559 = if iTemp558 != 0 {F32::sin(0.7853982 * (fTemp556 + self.fVec67[1]))} else {-(0.63661975 * ((F32::cos(1.5707964 * fTemp556) - F32::cos(1.5707964 * self.fVec67[1])) / if iTemp558 != 0 {1.0} else {fTemp557}))};
																																																																																																																																							self.fVec68[0] = fTemp559;
																																																																																																																																							self.fRec68[0] = 0.995 * self.fRec68[1] + fTemp559 - self.fVec68[1];
																																																																																																																																							fTemp560 = fSlow172 * self.fRec68[0];
																																																																																																																																						}
																																																																																																																																							if iSlow101 & (iSlow143 & iSlow176) != 0 {
																																																																																																																																								fTemp561 = 1.0 - OscDsp_faustpower2_f(1.0 - fTemp390);
																																																																																																																																								fTemp562 = 1.0 - 0.5 * OscDsp_faustpower2_f(fTemp390);
																																																																																																																																								iTemp563 = (self.fRec53[0] > 0.5) as i32;
																																																																																																																																								fTemp564 = if iTemp563 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * self.fRec53[0])};
																																																																																																																																								iTemp565 = unsafe { itbl3OscDspSIG3[((F32::max(0.0, F32::min(4.0, 1e+01 * (self.fRec53[0] + -0.5)))) as i32) as usize] };
																																																																																																																																								fTemp566 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp563, iTemp565))) as F32;
																																																																																																																																								fTemp567 = self.fConst2 * fTemp354 * F32::exp(0.0005776227 * (fTemp566 + 0.83 * fTemp564));
																																																																																																																																								fTemp568 = fTemp567 + self.fRec69[1];
																																																																																																																																								self.fRec69[0] = if iTemp0 != 0 {fTemp251} else {fTemp568 - F32::floor(fTemp568)};
																																																																																																																																								fTemp569 = self.fConst2 * fTemp354 * F32::exp(0.0005776227 * (0.36 * fTemp564 + fTemp566));
																																																																																																																																								fTemp570 = fTemp569 + self.fRec70[1];
																																																																																																																																								self.fRec70[0] = if iTemp0 != 0 {fTemp255} else {fTemp570 - F32::floor(fTemp570)};
																																																																																																																																								fTemp571 = self.fConst2 * fTemp354 * F32::exp(0.0005776227 * ((i32::wrapping_mul(0, iTemp565)) as F32 - fTemp564)) + self.fRec71[1];
																																																																																																																																								self.fRec71[0] = if iTemp0 != 0 {fTemp258} else {fTemp571 - F32::floor(fTemp571)};
																																																																																																																																								self.fVec69[0] = fTemp459 * fTemp562 + 0.5 * fTemp561 * (wt_basic(iSlow291, self.fConst0 * F32::abs(self.fConst2 * fTemp354 * F32::exp(-0.0005776227 * fTemp564)), self.fConst0, self.fRec71[0]) + wt_basic(iSlow291, self.fConst0 * F32::abs(fTemp569), self.fConst0, self.fRec70[0]) + wt_basic(iSlow291, self.fConst0 * F32::abs(fTemp567), self.fConst0, self.fRec69[0]));
																																																																																																																																								fTemp572 = fSlow175 * (self.fVec69[1] / F32::sqrt(OscDsp_faustpower2_f(fTemp562) + 0.75 * OscDsp_faustpower2_f(fTemp561)));
																																																																																																																																							}
																																																																																																																																								if iSlow101 & iSlow143 != 0 {
																																																																																																																																									fTemp573 = fSlow142 * fSlow322 * (fSlow321 * fTemp572 + fSlow320 * fTemp560 + fSlow319 * fTemp539 + fSlow318 * fTemp507 + fSlow317 * fTemp504 + fSlow303 * fTemp472 + fSlow302 * fTemp460 + fSlow301 * fTemp454 + fSlow300 * fTemp431 + fSlow299 * fTemp413 + fSlow292 * fTemp396 + fSlow290 * fTemp387);
																																																																																																																																								}
																																																																																																																																									if (iSlow101 & (iSlow98 & iSlow212)) | ((iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow188)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow98 & iSlow179))))))))))))))) != 0 {
																																																																																																																																										self.fVec70[0] = fSlow324;
																																																																																																																																										fTemp574 = if ((fSlow324 != self.fVec70[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec72[1] + -1.0};
																																																																																																																																										self.fRec72[0] = fTemp574;
																																																																																																																																										self.fRec73[0] = if iTemp0 != 0 {fSlow324} else {if (fTemp574 > 0.0) as i32 != 0 {self.fRec73[1] + (fSlow324 - self.fRec73[1]) / F32::max(1.0, fTemp574)} else {fSlow324}};
																																																																																																																																										fTemp575 = 3.0 * F32::max(0.0, F32::min(1.0, self.fRec73[0]));
																																																																																																																																										iTemp576 = std::cmp::min(2, (fTemp575) as i32);
																																																																																																																																										iTemp577 = (iTemp576 >= 2) as i32;
																																																																																																																																										iTemp578 = (iTemp576 >= 1) as i32;
																																																																																																																																										iTemp579 = i32::wrapping_mul(2176, (if iTemp577 != 0 {fSlow327} else {if iTemp578 != 0 {fSlow326} else {fSlow325}}) as i32);
																																																																																																																																									}
																																																																																																																																										if (iSlow101 & (iSlow98 & iSlow212)) | ((iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow98 & iSlow179)))))))))))))) != 0 {
																																																																																																																																											fTemp580 = 256.0 * fTemp354;
																																																																																																																																											iTemp581 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp580 > self.fConst16) as i32) + ((fTemp580 > self.fConst22) as i32), (fTemp580 > self.fConst21) as i32), (fTemp580 > self.fConst20) as i32), (fTemp580 > self.fConst19) as i32), (fTemp580 > self.fConst18) as i32), (fTemp580 > self.fConst17) as i32);
																																																																																																																																											iTemp582 = (iTemp581 >= 4) as i32;
																																																																																																																																											iTemp583 = (iTemp581 >= 2) as i32;
																																																																																																																																											iTemp584 = (iTemp581 >= 1) as i32;
																																																																																																																																											iTemp585 = (iTemp581 >= 3) as i32;
																																																																																																																																											iTemp586 = if iTemp582 != 0 {if (iTemp581 >= 6) as i32 != 0 {if (iTemp581 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp581 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp583 != 0 {if iTemp585 != 0 {1792} else {1536}} else {if iTemp584 != 0 {1024} else {0}}};
																																																																																																																																											iTemp587 = if iTemp582 != 0 {64} else {if iTemp583 != 0 {if iTemp585 != 0 {128} else {256}} else {if iTemp584 != 0 {512} else {1024}}};
																																																																																																																																											fTemp588 = self.fRec49[0] * (iTemp587) as F32;
																																																																																																																																											iTemp589 = (fTemp588) as i32;
																																																																																																																																											iTemp590 = (i32::wrapping_add(iTemp589, 1)) % iTemp587;
																																																																																																																																											fTemp591 = (iTemp589) as F32;
																																																																																																																																											fTemp592 = fTemp588 - fTemp591;
																																																																																																																																											fTemp593 = fTemp591 + (1.0 - fTemp588);
																																																																																																																																										}
																																																																																																																																											if (iSlow101 & (iSlow98 & iSlow212)) | ((iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow188)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow98 & iSlow179))))))))))))))) != 0 {
																																																																																																																																												fTemp594 = (iTemp576) as F32;
																																																																																																																																												fTemp595 = fTemp575 - fTemp594;
																																																																																																																																												iTemp596 = i32::wrapping_mul(2176, (if iTemp577 != 0 {fSlow326} else {if iTemp578 != 0 {fSlow325} else {fSlow328}}) as i32);
																																																																																																																																												fTemp597 = fTemp594 + (1.0 - fTemp575);
																																																																																																																																											}
																																																																																																																																												if (iSlow101 & (iSlow98 & iSlow212)) | ((iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow98 & iSlow179)))))))))))))) != 0 {
																																																																																																																																													fTemp598 = fTemp597 * (fTemp593 * wt_sample(i32::wrapping_add(iTemp586, i32::wrapping_add(iTemp589, iTemp596))) + fTemp592 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp596, iTemp586), iTemp590))) + fTemp595 * (fTemp593 * wt_sample(i32::wrapping_add(iTemp586, i32::wrapping_add(iTemp589, iTemp579))) + fTemp592 * wt_sample(i32::wrapping_add(iTemp590, i32::wrapping_add(iTemp586, iTemp579))));
																																																																																																																																												}
																																																																																																																																													if (iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow188)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow98 & iSlow179)))))))))))))) != 0 {
																																																																																																																																														self.fVec71[0] = fSlow329;
																																																																																																																																														fTemp599 = if ((fSlow329 != self.fVec71[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec74[1] + -1.0};
																																																																																																																																														self.fRec74[0] = fTemp599;
																																																																																																																																														self.fRec75[0] = if iTemp0 != 0 {fSlow329} else {if (fTemp599 > 0.0) as i32 != 0 {self.fRec75[1] + (fSlow329 - self.fRec75[1]) / F32::max(1.0, fTemp599)} else {fSlow329}};
																																																																																																																																														fTemp600 = F32::max(0.0, F32::min(1.0, self.fRec75[0]));
																																																																																																																																													}
																																																																																																																																														if (iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow98 & iSlow179)))) != 0 {
																																																																																																																																															fTemp601 = (1.0 - fTemp600) * fTemp598;
																																																																																																																																														}
																																																																																																																																															if iSlow101 & (iSlow98 & iSlow179) != 0 {
																																																																																																																																																fTemp602 = fSlow178 * (fTemp601 + fTemp600 * if (fTemp598 > 0.0) as i32 != 0 {1.0 - fTemp598} else {-1.0 - fTemp598});
																																																																																																																																															}
																																																																																																																																																if (iSlow101 & (iSlow98 & iSlow197)) | (iSlow101 & (iSlow98 & iSlow182)) != 0 {
																																																																																																																																																	fTemp603 = 512.0 * fTemp354;
																																																																																																																																																	iTemp604 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp603 > self.fConst16) as i32) + ((fTemp603 > self.fConst22) as i32), (fTemp603 > self.fConst21) as i32), (fTemp603 > self.fConst20) as i32), (fTemp603 > self.fConst19) as i32), (fTemp603 > self.fConst18) as i32), (fTemp603 > self.fConst17) as i32);
																																																																																																																																																	iTemp605 = (iTemp604 >= 4) as i32;
																																																																																																																																																	iTemp606 = (iTemp604 >= 2) as i32;
																																																																																																																																																	iTemp607 = (iTemp604 >= 1) as i32;
																																																																																																																																																	iTemp608 = (iTemp604 >= 3) as i32;
																																																																																																																																																	iTemp609 = if iTemp605 != 0 {64} else {if iTemp606 != 0 {if iTemp608 != 0 {128} else {256}} else {if iTemp607 != 0 {512} else {1024}}};
																																																																																																																																																	fTemp610 = (iTemp609) as F32;
																																																																																																																																																}
																																																																																																																																																	if iSlow101 & (iSlow98 & iSlow182) != 0 {
																																																																																																																																																		fTemp611 = fTemp610 * if iTemp357 != 0 {fTemp364} else {2.0 * fTemp449};
																																																																																																																																																		iTemp612 = (fTemp611) as i32;
																																																																																																																																																		iTemp613 = (i32::wrapping_add(iTemp612, 1)) % iTemp609;
																																																																																																																																																	}
																																																																																																																																																		if (iSlow101 & (iSlow98 & iSlow197)) | (iSlow101 & (iSlow98 & iSlow182)) != 0 {
																																																																																																																																																			iTemp614 = if iTemp605 != 0 {if (iTemp604 >= 6) as i32 != 0 {if (iTemp604 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp604 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp606 != 0 {if iTemp608 != 0 {1792} else {1536}} else {if iTemp607 != 0 {1024} else {0}}};
																																																																																																																																																			iTemp615 = i32::wrapping_add(iTemp579, iTemp614);
																																																																																																																																																		}
																																																																																																																																																			if iSlow101 & (iSlow98 & iSlow182) != 0 {
																																																																																																																																																				fTemp616 = (iTemp612) as F32;
																																																																																																																																																				fTemp617 = fTemp611 - fTemp616;
																																																																																																																																																				fTemp618 = fTemp616 + (1.0 - fTemp611);
																																																																																																																																																			}
																																																																																																																																																				if (iSlow101 & (iSlow98 & iSlow197)) | (iSlow101 & (iSlow98 & iSlow182)) != 0 {
																																																																																																																																																					iTemp619 = i32::wrapping_add(iTemp596, iTemp614);
																																																																																																																																																				}
																																																																																																																																																					if iSlow101 & (iSlow98 & iSlow182) != 0 {
																																																																																																																																																						fTemp620 = fSlow181 * (fTemp601 + fTemp600 * (fTemp597 * (fTemp618 * wt_sample(i32::wrapping_add(iTemp619, iTemp612)) + fTemp617 * wt_sample(i32::wrapping_add(iTemp619, iTemp613))) + fTemp595 * (fTemp618 * wt_sample(i32::wrapping_add(iTemp615, iTemp612)) + fTemp617 * wt_sample(i32::wrapping_add(iTemp615, iTemp613)))));
																																																																																																																																																					}
																																																																																																																																																						if iSlow101 & (iSlow98 & iSlow185) != 0 {
																																																																																																																																																							fTemp621 = fSlow184 * fTemp598 * (1.0 - fTemp600 * (1.0 - F32::sin(6.2831855 * self.fRec49[0] * (7.0 * fTemp600 + 1.0))));
																																																																																																																																																						}
																																																																																																																																																							if iSlow101 & (iSlow98 & iSlow188) != 0 {
																																																																																																																																																								fTemp622 = 2.0 * fTemp600 + 1.0;
																																																																																																																																																								fTemp623 = 256.0 * fTemp354 * fTemp622;
																																																																																																																																																								iTemp624 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp623 > self.fConst16) as i32) + ((fTemp623 > self.fConst22) as i32), (fTemp623 > self.fConst21) as i32), (fTemp623 > self.fConst20) as i32), (fTemp623 > self.fConst19) as i32), (fTemp623 > self.fConst18) as i32), (fTemp623 > self.fConst17) as i32);
																																																																																																																																																								iTemp625 = (iTemp624 >= 4) as i32;
																																																																																																																																																								iTemp626 = (iTemp624 >= 2) as i32;
																																																																																																																																																								iTemp627 = (iTemp624 >= 1) as i32;
																																																																																																																																																								iTemp628 = (iTemp624 >= 3) as i32;
																																																																																																																																																								iTemp629 = if iTemp625 != 0 {if (iTemp624 >= 6) as i32 != 0 {if (iTemp624 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp624 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp626 != 0 {if iTemp628 != 0 {1792} else {1536}} else {if iTemp627 != 0 {1024} else {0}}};
																																																																																																																																																								iTemp630 = if iTemp625 != 0 {64} else {if iTemp626 != 0 {if iTemp628 != 0 {128} else {256}} else {if iTemp627 != 0 {512} else {1024}}};
																																																																																																																																																								fTemp631 = F32::powf(self.fRec49[0], fTemp622) * (iTemp630) as F32;
																																																																																																																																																								iTemp632 = (fTemp631) as i32;
																																																																																																																																																								iTemp633 = (i32::wrapping_add(iTemp632, 1)) % iTemp630;
																																																																																																																																																								fTemp634 = (iTemp632) as F32;
																																																																																																																																																								fTemp635 = fTemp631 - fTemp634;
																																																																																																																																																								fTemp636 = fTemp634 + (1.0 - fTemp631);
																																																																																																																																																								fTemp637 = fSlow187 * (fTemp597 * (fTemp636 * wt_sample(i32::wrapping_add(iTemp629, i32::wrapping_add(iTemp596, iTemp632))) + fTemp635 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp596, iTemp629), iTemp633))) + fTemp595 * (fTemp636 * wt_sample(i32::wrapping_add(iTemp629, i32::wrapping_add(iTemp579, iTemp632))) + fTemp635 * wt_sample(i32::wrapping_add(iTemp633, i32::wrapping_add(iTemp579, iTemp629)))));
																																																																																																																																																							}
																																																																																																																																																								if iSlow101 & (iSlow98 & iSlow191) != 0 {
																																																																																																																																																									fTemp638 = fSlow190 * (fTemp601 + fTemp600 * F32::sin(7.8539815 * fTemp598));
																																																																																																																																																								}
																																																																																																																																																									if iSlow101 & (iSlow98 & iSlow194) != 0 {
																																																																																																																																																										fTemp639 = 0.25 * (fTemp598 * (4.0 * fTemp600 + 1.0) + 1.0);
																																																																																																																																																										fTemp640 = fSlow193 * (1.0 - 4.0 * F32::abs(fTemp639 + (-0.5 - F32::floor(fTemp639))));
																																																																																																																																																									}
																																																																																																																																																										if iSlow101 & (iSlow98 & iSlow197) != 0 {
																																																																																																																																																											fTemp641 = 2.0 * self.fRec49[0] * fTemp610;
																																																																																																																																																											iTemp642 = (fTemp641) as i32;
																																																																																																																																																											iTemp643 = (i32::wrapping_add(iTemp642, 1)) % iTemp609;
																																																																																																																																																											fTemp644 = (iTemp642) as F32;
																																																																																																																																																											fTemp645 = fTemp641 - fTemp644;
																																																																																																																																																											fTemp646 = fTemp644 + (1.0 - fTemp641);
																																																																																																																																																											fTemp647 = fSlow196 * (fTemp601 + fTemp600 * (iTemp357) as F32 * F32::abs(fTemp597 * (fTemp646 * wt_sample(i32::wrapping_add(iTemp614, i32::wrapping_add(iTemp596, iTemp642))) + fTemp645 * wt_sample(i32::wrapping_add(iTemp619, iTemp643))) + fTemp595 * (fTemp646 * wt_sample(i32::wrapping_add(iTemp614, i32::wrapping_add(iTemp579, iTemp642))) + fTemp645 * wt_sample(i32::wrapping_add(iTemp643, iTemp615)))));
																																																																																																																																																										}
																																																																																																																																																											if (iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | (iSlow101 & (iSlow98 & iSlow200))) != 0 {
																																																																																																																																																												fTemp648 = F32::tan(self.fConst25 * F32::powf(fSlow338, fTemp600));
																																																																																																																																																												fTemp649 = 1.0 / fTemp648;
																																																																																																																																																												fTemp650 = (fTemp649 + 1.4142135) / fTemp648 + 1.0;
																																																																																																																																																												self.fRec76[0] = fTemp598 - (self.fRec76[2] * ((fTemp649 + -1.4142135) / fTemp648 + 1.0) + 2.0 * self.fRec76[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp648))) / fTemp650;
																																																																																																																																																												fTemp651 = (self.fRec76[2] + self.fRec76[0] + 2.0 * self.fRec76[1]) / fTemp650;
																																																																																																																																																											}
																																																																																																																																																												if iSlow101 & (iSlow98 & iSlow200) != 0 {
																																																																																																																																																													fTemp652 = fSlow199 * fTemp651;
																																																																																																																																																												}
																																																																																																																																																													if iSlow101 & (iSlow98 & iSlow203) != 0 {
																																																																																																																																																														fTemp653 = fSlow202 * fTemp651;
																																																																																																																																																													}
																																																																																																																																																														if iSlow101 & (iSlow98 & iSlow206) != 0 {
																																																																																																																																																															fTemp654 = fSlow205 * fTemp651;
																																																																																																																																																														}
																																																																																																																																																															if (iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | (iSlow101 & (iSlow98 & iSlow94))) != 0 {
																																																																																																																																																																fTemp655 = if iTemp7 != 0 {0.0} else {self.fRec78[1] + self.fConst2 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow343, fTemp600)), 0.0)};
																																																																																																																																																																self.fRec78[0] = fTemp655 - F32::floor(fTemp655);
																																																																																																																																																																self.fRec77[0] = if (self.fRec78[0] < self.fRec78[1]) as i32 != 0 {fTemp598} else {self.fRec77[1]};
																																																																																																																																																															}
																																																																																																																																																																if iSlow101 & (iSlow98 & iSlow94) != 0 {
																																																																																																																																																																	fTemp656 = fSlow93 * self.fRec77[0];
																																																																																																																																																																}
																																																																																																																																																																	if iSlow101 & (iSlow98 & iSlow104) != 0 {
																																																																																																																																																																		fTemp657 = fSlow103 * self.fRec77[0];
																																																																																																																																																																	}
																																																																																																																																																																		if iSlow101 & (iSlow98 & iSlow107) != 0 {
																																																																																																																																																																			fTemp658 = fSlow106 * self.fRec77[0];
																																																																																																																																																																		}
																																																																																																																																																																			if iSlow101 & (iSlow98 & iSlow209) != 0 {
																																																																																																																																																																				fTemp659 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp600)) + -1.0));
																																																																																																																																																																				fTemp660 = fSlow208 * (F32::floor(fTemp598 * fTemp659 + 0.5) / fTemp659);
																																																																																																																																																																			}
																																																																																																																																																																				if iSlow101 & (iSlow98 & iSlow212) != 0 {
																																																																																																																																																																					fTemp661 = fSlow211 * fTemp598;
																																																																																																																																																																				}
																																																																																																																																																																					if iSlow101 & iSlow98 != 0 {
																																																																																																																																																																						fTemp662 = fSlow97 * (fSlow348 * fTemp661 + fSlow347 * fTemp660 + fSlow346 * fTemp658 + fSlow345 * fTemp657 + fSlow344 * fTemp656 + fSlow341 * fTemp654 + fSlow340 * fTemp653 + fSlow339 * fTemp652 + fSlow336 * fTemp647 + fSlow335 * fTemp640 + fSlow334 * fTemp638 + fSlow333 * fTemp637 + fSlow332 * fTemp621 + fSlow331 * fTemp620 + fSlow330 * fTemp602);
																																																																																																																																																																					}
																																																																																																																																																																						if iSlow101 != 0 {
																																																																																																																																																																							fTemp663 = fSlow100 * (fSlow349 * fTemp662 + fSlow323 * fTemp573);
																																																																																																																																																																						}
																																																																																																																																																																							*output1 = fSlow350 * fTemp663;
																																																																																																																																																																							self.fVec72[0] = fSlow351;
																																																																																																																																																																							let mut fTemp664: F32 = if ((fSlow351 != self.fVec72[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec79[1] + -1.0};
																																																																																																																																																																							self.fRec79[0] = fTemp664;
																																																																																																																																																																							self.fRec80[0] = if iTemp0 != 0 {fSlow351} else {if (fTemp664 > 0.0) as i32 != 0 {self.fRec80[1] + (fSlow351 - self.fRec80[1]) / F32::max(1.0, fTemp664)} else {fSlow351}};
																																																																																																																																																																							self.fVec73[0] = fSlow352;
																																																																																																																																																																							let mut fTemp665: F32 = if ((fSlow352 != self.fVec73[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec81[1] + -1.0};
																																																																																																																																																																							self.fRec81[0] = fTemp665;
																																																																																																																																																																							self.fRec82[0] = if iTemp0 != 0 {fSlow352} else {if (fTemp665 > 0.0) as i32 != 0 {self.fRec82[1] + (fSlow352 - self.fRec82[1]) / F32::max(1.0, fTemp665)} else {fSlow352}};
																																																																																																																																																																							*output2 = fTemp23;
																																																																																																																																																																							*output3 = fTemp354;
																																																																																																																																																																							*output4 = fTemp28;
																																																																																																																																																																							*output5 = (iTemp0) as F32;
																																																																																																																																																																							self.fVec0[1] = self.fVec0[0];
																																																																																																																																																																							self.iVec1[1] = self.iVec1[0];
																																																																																																																																																																							self.fVec2[1] = self.fVec2[0];
																																																																																																																																																																							self.fRec1[1] = self.fRec1[0];
																																																																																																																																																																							self.fRec2[1] = self.fRec2[0];
																																																																																																																																																																							self.fVec3[1] = self.fVec3[0];
																																																																																																																																																																							self.fRec3[1] = self.fRec3[0];
																																																																																																																																																																							self.fRec4[1] = self.fRec4[0];
																																																																																																																																																																							if (fTemp6 != 0.0) as i32 != 0 {
																																																																																																																																																																								self.fRec6[1] = self.fRec6[0];
																																																																																																																																																																								self.fRec7[1] = self.fRec7[0];
																																																																																																																																																																								self.iRec9[1] = self.iRec9[0];
																																																																																																																																																																								self.fRec8[1] = self.fRec8[0];
																																																																																																																																																																								self.fRec10[1] = self.fRec10[0];
																																																																																																																																																																								self.fRec13[1] = self.fRec13[0];
																																																																																																																																																																							}
																																																																																																																																																																								self.iRec14[1] = self.iRec14[0];
																																																																																																																																																																								self.fVec5[1] = self.fVec5[0];
																																																																																																																																																																								if (fTemp6 != 0.0) as i32 != 0 {
																																																																																																																																																																									self.fRec12[1] = self.fRec12[0];
																																																																																																																																																																									self.fVec6[1] = self.fVec6[0];
																																																																																																																																																																									self.fRec11[1] = self.fRec11[0];
																																																																																																																																																																								}
																																																																																																																																																																									self.fRec15[1] = self.fRec15[0];
																																																																																																																																																																									self.fVec7[1] = self.fVec7[0];
																																																																																																																																																																									self.fRec16[1] = self.fRec16[0];
																																																																																																																																																																									self.fRec17[1] = self.fRec17[0];
																																																																																																																																																																									self.fVec8[1] = self.fVec8[0];
																																																																																																																																																																									self.fRec18[1] = self.fRec18[0];
																																																																																																																																																																									self.fRec19[1] = self.fRec19[0];
																																																																																																																																																																									self.fRec20[1] = self.fRec20[0];
																																																																																																																																																																									self.fRec21[1] = self.fRec21[0];
																																																																																																																																																																									if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow44 & iSlow41)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10))))))))))))))))))))))))) != 0 {
																																																																																																																																																																										self.fRec0[1] = self.fRec0[0];
																																																																																																																																																																									}
																																																																																																																																																																										if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow218)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow215)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
																																																																																																																																																																											self.fVec9[1] = self.fVec9[0];
																																																																																																																																																																											self.fRec22[1] = self.fRec22[0];
																																																																																																																																																																											self.fRec23[1] = self.fRec23[0];
																																																																																																																																																																											self.fVec10[1] = self.fVec10[0];
																																																																																																																																																																											self.fRec24[1] = self.fRec24[0];
																																																																																																																																																																											self.fRec25[1] = self.fRec25[0];
																																																																																																																																																																										}
																																																																																																																																																																											if iSlow2 & (iSlow6 & iSlow10) != 0 {
																																																																																																																																																																												self.fVec11[1] = self.fVec11[0];
																																																																																																																																																																												self.fVec12[1] = self.fVec12[0];
																																																																																																																																																																											}
																																																																																																																																																																												if iSlow2 & (iSlow6 & iSlow13) != 0 {
																																																																																																																																																																													self.fRec26[1] = self.fRec26[0];
																																																																																																																																																																													self.fVec13[1] = self.fVec13[0];
																																																																																																																																																																												}
																																																																																																																																																																													if (iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																																																																																																																																																																														self.iRec27[1] = self.iRec27[0];
																																																																																																																																																																													}
																																																																																																																																																																														if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																																																																																																																																																															self.fVec14[1] = self.fVec14[0];
																																																																																																																																																																															self.fVec15[1] = self.fVec15[0];
																																																																																																																																																																														}
																																																																																																																																																																															if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																																																																																																																																																																																self.fVec16[1] = self.fVec16[0];
																																																																																																																																																																															}
																																																																																																																																																																																if iSlow2 & (iSlow6 & iSlow19) != 0 {
																																																																																																																																																																																	self.fVec17[1] = self.fVec17[0];
																																																																																																																																																																																	self.fVec18[1] = self.fVec18[0];
																																																																																																																																																																																	self.fVec19[1] = self.fVec19[0];
																																																																																																																																																																																}
																																																																																																																																																																																	if iSlow2 & (iSlow6 & iSlow22) != 0 {
																																																																																																																																																																																		self.fVec20[1] = self.fVec20[0];
																																																																																																																																																																																		self.fVec21[1] = self.fVec21[0];
																																																																																																																																																																																	}
																																																																																																																																																																																		if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																																																																																																																																																																			self.fRec28[1] = self.fRec28[0];
																																																																																																																																																																																			self.fVec22[1] = self.fVec22[0];
																																																																																																																																																																																			self.fRec29[1] = self.fRec29[0];
																																																																																																																																																																																			self.fVec23[1] = self.fVec23[0];
																																																																																																																																																																																		}
																																																																																																																																																																																			if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | (iSlow2 & (iSlow6 & iSlow25)))) != 0 {
																																																																																																																																																																																				self.fVec24[1] = self.fVec24[0];
																																																																																																																																																																																			}
																																																																																																																																																																																				if iSlow2 & (iSlow6 & iSlow28) != 0 {
																																																																																																																																																																																					self.fRec30[1] = self.fRec30[0];
																																																																																																																																																																																					self.fVec25[1] = self.fVec25[0];
																																																																																																																																																																																					self.fRec31[1] = self.fRec31[0];
																																																																																																																																																																																					self.fVec26[1] = self.fVec26[0];
																																																																																																																																																																																				}
																																																																																																																																																																																					if iSlow2 & (iSlow6 & iSlow215) != 0 {
																																																																																																																																																																																						self.fRec32[1] = self.fRec32[0];
																																																																																																																																																																																						self.fVec27[1] = self.fVec27[0];
																																																																																																																																																																																						self.fRec33[1] = self.fRec33[0];
																																																																																																																																																																																						self.fVec28[1] = self.fVec28[0];
																																																																																																																																																																																						self.fVec29[1] = self.fVec29[0];
																																																																																																																																																																																					}
																																																																																																																																																																																						if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																																																																																																																																																							self.fRec34[1] = self.fRec34[0];
																																																																																																																																																																																						}
																																																																																																																																																																																							if iSlow2 & (iSlow6 & iSlow218) != 0 {
																																																																																																																																																																																								self.fRec35[1] = self.fRec35[0];
																																																																																																																																																																																								self.fRec36[1] = self.fRec36[0];
																																																																																																																																																																																								self.fVec30[1] = self.fVec30[0];
																																																																																																																																																																																								self.fRec37[1] = self.fRec37[0];
																																																																																																																																																																																								self.fVec31[1] = self.fVec31[0];
																																																																																																																																																																																								self.fVec32[1] = self.fVec32[0];
																																																																																																																																																																																							}
																																																																																																																																																																																								if iSlow2 & (iSlow6 & iSlow34) != 0 {
																																																																																																																																																																																									self.fVec33[1] = self.fVec33[0];
																																																																																																																																																																																									self.fVec34[1] = self.fVec34[0];
																																																																																																																																																																																									self.fVec35[1] = self.fVec35[0];
																																																																																																																																																																																									self.fVec36[1] = self.fVec36[0];
																																																																																																																																																																																									self.fRec38[1] = self.fRec38[0];
																																																																																																																																																																																								}
																																																																																																																																																																																									if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																																																																																																																																																										self.fRec39[1] = self.fRec39[0];
																																																																																																																																																																																										self.fRec40[1] = self.fRec40[0];
																																																																																																																																																																																										self.fRec41[1] = self.fRec41[0];
																																																																																																																																																																																										self.fVec37[1] = self.fVec37[0];
																																																																																																																																																																																									}
																																																																																																																																																																																										if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41))))))))))))))) != 0 {
																																																																																																																																																																																											self.fVec38[1] = self.fVec38[0];
																																																																																																																																																																																											self.fRec42[1] = self.fRec42[0];
																																																																																																																																																																																											self.fRec43[1] = self.fRec43[0];
																																																																																																																																																																																										}
																																																																																																																																																																																											if (iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41)))))))))))))) != 0 {
																																																																																																																																																																																												self.fVec39[1] = self.fVec39[0];
																																																																																																																																																																																												self.fRec44[1] = self.fRec44[0];
																																																																																																																																																																																												self.fRec45[1] = self.fRec45[0];
																																																																																																																																																																																											}
																																																																																																																																																																																												if (iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow68)) | (iSlow2 & (iSlow44 & iSlow65))) != 0 {
																																																																																																																																																																																													self.fRec46[2] = self.fRec46[1];
																																																																																																																																																																																													self.fRec46[1] = self.fRec46[0];
																																																																																																																																																																																												}
																																																																																																																																																																																													if (iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | (iSlow2 & (iSlow44 & iSlow77))) != 0 {
																																																																																																																																																																																														self.fRec48[1] = self.fRec48[0];
																																																																																																																																																																																														self.fRec47[1] = self.fRec47[0];
																																																																																																																																																																																													}
																																																																																																																																																																																														self.fVec40[1] = self.fVec40[0];
																																																																																																																																																																																														self.fRec50[1] = self.fRec50[0];
																																																																																																																																																																																														self.fRec51[1] = self.fRec51[0];
																																																																																																																																																																																														if (iSlow101 & (iSlow98 & iSlow212)) | ((iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow188)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | ((iSlow101 & (iSlow98 & iSlow179)) | ((iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow173)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | ((iSlow101 & (iSlow143 & iSlow149)) | ((iSlow101 & (iSlow143 & iSlow146)) | (iSlow101 & (iSlow143 & iSlow140))))))))))))))))))))))))) != 0 {
																																																																																																																																																																																															self.fRec49[1] = self.fRec49[0];
																																																																																																																																																																																														}
																																																																																																																																																																																															if (iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow173)) | ((iSlow101 & (iSlow143 & iSlow170)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow164)) | ((iSlow101 & (iSlow143 & iSlow161)) | ((iSlow101 & (iSlow143 & iSlow158)) | ((iSlow101 & (iSlow143 & iSlow155)) | ((iSlow101 & (iSlow143 & iSlow152)) | ((iSlow101 & (iSlow143 & iSlow149)) | ((iSlow101 & (iSlow143 & iSlow146)) | (iSlow101 & (iSlow143 & iSlow140)))))))))))) != 0 {
																																																																																																																																																																																																self.fVec41[1] = self.fVec41[0];
																																																																																																																																																																																																self.fRec52[1] = self.fRec52[0];
																																																																																																																																																																																																self.fRec53[1] = self.fRec53[0];
																																																																																																																																																																																																self.fVec42[1] = self.fVec42[0];
																																																																																																																																																																																																self.fRec54[1] = self.fRec54[0];
																																																																																																																																																																																																self.fRec55[1] = self.fRec55[0];
																																																																																																																																																																																															}
																																																																																																																																																																																																if iSlow101 & (iSlow143 & iSlow140) != 0 {
																																																																																																																																																																																																	self.fVec43[1] = self.fVec43[0];
																																																																																																																																																																																																	self.fVec44[1] = self.fVec44[0];
																																																																																																																																																																																																}
																																																																																																																																																																																																	if iSlow101 & (iSlow143 & iSlow146) != 0 {
																																																																																																																																																																																																		self.fRec56[1] = self.fRec56[0];
																																																																																																																																																																																																		self.fVec45[1] = self.fVec45[0];
																																																																																																																																																																																																	}
																																																																																																																																																																																																		if (iSlow101 & (iSlow143 & iSlow152)) | (iSlow101 & (iSlow143 & iSlow149)) != 0 {
																																																																																																																																																																																																			self.iRec57[1] = self.iRec57[0];
																																																																																																																																																																																																		}
																																																																																																																																																																																																			if iSlow101 & (iSlow143 & iSlow149) != 0 {
																																																																																																																																																																																																				self.fVec46[1] = self.fVec46[0];
																																																																																																																																																																																																				self.fVec47[1] = self.fVec47[0];
																																																																																																																																																																																																			}
																																																																																																																																																																																																				if (iSlow101 & (iSlow143 & iSlow155)) | (iSlow101 & (iSlow143 & iSlow149)) != 0 {
																																																																																																																																																																																																					self.fVec48[1] = self.fVec48[0];
																																																																																																																																																																																																				}
																																																																																																																																																																																																					if iSlow101 & (iSlow143 & iSlow152) != 0 {
																																																																																																																																																																																																						self.fVec49[1] = self.fVec49[0];
																																																																																																																																																																																																						self.fVec50[1] = self.fVec50[0];
																																																																																																																																																																																																						self.fVec51[1] = self.fVec51[0];
																																																																																																																																																																																																					}
																																																																																																																																																																																																						if iSlow101 & (iSlow143 & iSlow155) != 0 {
																																																																																																																																																																																																							self.fVec52[1] = self.fVec52[0];
																																																																																																																																																																																																							self.fVec53[1] = self.fVec53[0];
																																																																																																																																																																																																						}
																																																																																																																																																																																																							if iSlow101 & (iSlow143 & iSlow158) != 0 {
																																																																																																																																																																																																								self.fRec58[1] = self.fRec58[0];
																																																																																																																																																																																																								self.fVec54[1] = self.fVec54[0];
																																																																																																																																																																																																								self.fRec59[1] = self.fRec59[0];
																																																																																																																																																																																																								self.fVec55[1] = self.fVec55[0];
																																																																																																																																																																																																							}
																																																																																																																																																																																																								if (iSlow101 & (iSlow143 & iSlow176)) | ((iSlow101 & (iSlow143 & iSlow167)) | ((iSlow101 & (iSlow143 & iSlow161)) | (iSlow101 & (iSlow143 & iSlow158)))) != 0 {
																																																																																																																																																																																																									self.fVec56[1] = self.fVec56[0];
																																																																																																																																																																																																								}
																																																																																																																																																																																																									if iSlow101 & (iSlow143 & iSlow161) != 0 {
																																																																																																																																																																																																										self.fRec60[1] = self.fRec60[0];
																																																																																																																																																																																																										self.fVec57[1] = self.fVec57[0];
																																																																																																																																																																																																										self.fRec61[1] = self.fRec61[0];
																																																																																																																																																																																																										self.fVec58[1] = self.fVec58[0];
																																																																																																																																																																																																									}
																																																																																																																																																																																																										if iSlow101 & (iSlow143 & iSlow164) != 0 {
																																																																																																																																																																																																											self.fRec62[1] = self.fRec62[0];
																																																																																																																																																																																																											self.fVec59[1] = self.fVec59[0];
																																																																																																																																																																																																											self.fRec63[1] = self.fRec63[0];
																																																																																																																																																																																																											self.fVec60[1] = self.fVec60[0];
																																																																																																																																																																																																											self.fVec61[1] = self.fVec61[0];
																																																																																																																																																																																																										}
																																																																																																																																																																																																											if iSlow101 & (iSlow143 & iSlow167) != 0 {
																																																																																																																																																																																																												self.fRec64[1] = self.fRec64[0];
																																																																																																																																																																																																											}
																																																																																																																																																																																																												if iSlow101 & (iSlow143 & iSlow170) != 0 {
																																																																																																																																																																																																													self.fRec65[1] = self.fRec65[0];
																																																																																																																																																																																																													self.fRec66[1] = self.fRec66[0];
																																																																																																																																																																																																													self.fVec62[1] = self.fVec62[0];
																																																																																																																																																																																																													self.fRec67[1] = self.fRec67[0];
																																																																																																																																																																																																													self.fVec63[1] = self.fVec63[0];
																																																																																																																																																																																																													self.fVec64[1] = self.fVec64[0];
																																																																																																																																																																																																												}
																																																																																																																																																																																																													if iSlow101 & (iSlow143 & iSlow173) != 0 {
																																																																																																																																																																																																														self.fVec65[1] = self.fVec65[0];
																																																																																																																																																																																																														self.fVec66[1] = self.fVec66[0];
																																																																																																																																																																																																														self.fVec67[1] = self.fVec67[0];
																																																																																																																																																																																																														self.fVec68[1] = self.fVec68[0];
																																																																																																																																																																																																														self.fRec68[1] = self.fRec68[0];
																																																																																																																																																																																																													}
																																																																																																																																																																																																														if iSlow101 & (iSlow143 & iSlow176) != 0 {
																																																																																																																																																																																																															self.fRec69[1] = self.fRec69[0];
																																																																																																																																																																																																															self.fRec70[1] = self.fRec70[0];
																																																																																																																																																																																																															self.fRec71[1] = self.fRec71[0];
																																																																																																																																																																																																															self.fVec69[1] = self.fVec69[0];
																																																																																																																																																																																																														}
																																																																																																																																																																																																															if (iSlow101 & (iSlow98 & iSlow212)) | ((iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow188)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow98 & iSlow179))))))))))))))) != 0 {
																																																																																																																																																																																																																self.fVec70[1] = self.fVec70[0];
																																																																																																																																																																																																																self.fRec72[1] = self.fRec72[0];
																																																																																																																																																																																																																self.fRec73[1] = self.fRec73[0];
																																																																																																																																																																																																															}
																																																																																																																																																																																																																if (iSlow101 & (iSlow98 & iSlow209)) | ((iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | ((iSlow101 & (iSlow98 & iSlow94)) | ((iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | ((iSlow101 & (iSlow98 & iSlow200)) | ((iSlow101 & (iSlow98 & iSlow197)) | ((iSlow101 & (iSlow98 & iSlow194)) | ((iSlow101 & (iSlow98 & iSlow191)) | ((iSlow101 & (iSlow98 & iSlow188)) | ((iSlow101 & (iSlow98 & iSlow185)) | ((iSlow101 & (iSlow98 & iSlow182)) | (iSlow101 & (iSlow98 & iSlow179)))))))))))))) != 0 {
																																																																																																																																																																																																																	self.fVec71[1] = self.fVec71[0];
																																																																																																																																																																																																																	self.fRec74[1] = self.fRec74[0];
																																																																																																																																																																																																																	self.fRec75[1] = self.fRec75[0];
																																																																																																																																																																																																																}
																																																																																																																																																																																																																	if (iSlow101 & (iSlow98 & iSlow206)) | ((iSlow101 & (iSlow98 & iSlow203)) | (iSlow101 & (iSlow98 & iSlow200))) != 0 {
																																																																																																																																																																																																																		self.fRec76[2] = self.fRec76[1];
																																																																																																																																																																																																																		self.fRec76[1] = self.fRec76[0];
																																																																																																																																																																																																																	}
																																																																																																																																																																																																																		if (iSlow101 & (iSlow98 & iSlow107)) | ((iSlow101 & (iSlow98 & iSlow104)) | (iSlow101 & (iSlow98 & iSlow94))) != 0 {
																																																																																																																																																																																																																			self.fRec78[1] = self.fRec78[0];
																																																																																																																																																																																																																			self.fRec77[1] = self.fRec77[0];
																																																																																																																																																																																																																		}
																																																																																																																																																																																																																			self.fVec72[1] = self.fVec72[0];
																																																																																																																																																																																																																			self.fRec79[1] = self.fRec79[0];
																																																																																																																																																																																																																			self.fRec80[1] = self.fRec80[0];
																																																																																																																																																																																																																			self.fVec73[1] = self.fVec73[0];
																																																																																																																																																																																																																			self.fRec81[1] = self.fRec81[0];
																																																																																																																																																																																																																			self.fRec82[1] = self.fRec82[0];
																																																																																																																																																																																																																		}
																																																																																																																																																																																																																		self.iTempPerm0 = iTemp7;
																																																																																																																																																																																																																		self.fTempPerm1 = fTemp8;
																																																																																																																																																																																																																		self.fTempPerm2 = fTemp9;
																																																																																																																																																																																																																		self.iTempPerm3 = iTemp10;
																																																																																																																																																																																																																		self.fTempPerm4 = fTemp13;
																																																																																																																																																																																																																		self.fTempPerm5 = fTemp16;
																																																																																																																																																																																																																		self.fTempPerm6 = fTemp17;
																																																																																																																																																																																																																		self.fTempPerm7 = fTemp24;
																																																																																																																																																																																																																		self.fTempPerm8 = fTemp25;
																																																																																																																																																																																																																		self.fTempPerm9 = fTemp29;
																																																																																																																																																																																																																		self.iTempPerm10 = iTemp30;
																																																																																																																																																																																																																		self.fTempPerm11 = fTemp31;
																																																																																																																																																																																																																		self.fTempPerm12 = fTemp32;
																																																																																																																																																																																																																		self.fTempPerm13 = fTemp33;
																																																																																																																																																																																																																		self.fTempPerm14 = fTemp34;
																																																																																																																																																																																																																		self.fTempPerm15 = fTemp35;
																																																																																																																																																																																																																		self.fTempPerm16 = fTemp36;
																																																																																																																																																																																																																		self.fTempPerm17 = fTemp37;
																																																																																																																																																																																																																		self.fTempPerm18 = fTemp38;
																																																																																																																																																																																																																		self.fTempPerm19 = fTemp39;
																																																																																																																																																																																																																		self.fTempPerm20 = fTemp40;
																																																																																																																																																																																																																		self.fTempPerm21 = fTemp41;
																																																																																																																																																																																																																		self.fTempPerm22 = fTemp42;
																																																																																																																																																																																																																		self.fTempPerm23 = fTemp43;
																																																																																																																																																																																																																		self.iTempPerm24 = iTemp44;
																																																																																																																																																																																																																		self.fTempPerm25 = fTemp45;
																																																																																																																																																																																																																		self.fTempPerm26 = fTemp46;
																																																																																																																																																																																																																		self.fTempPerm27 = fTemp47;
																																																																																																																																																																																																																		self.fTempPerm28 = fTemp48;
																																																																																																																																																																																																																		self.fTempPerm29 = fTemp49;
																																																																																																																																																																																																																		self.fTempPerm30 = fTemp50;
																																																																																																																																																																																																																		self.fTempPerm31 = fTemp51;
																																																																																																																																																																																																																		self.fTempPerm32 = fTemp52;
																																																																																																																																																																																																																		self.fTempPerm33 = fTemp53;
																																																																																																																																																																																																																		self.fTempPerm34 = fTemp54;
																																																																																																																																																																																																																		self.fTempPerm35 = fTemp55;
																																																																																																																																																																																																																		self.fTempPerm36 = fTemp56;
																																																																																																																																																																																																																		self.fTempPerm37 = fTemp57;
																																																																																																																																																																																																																		self.fTempPerm38 = fTemp58;
																																																																																																																																																																																																																		self.fTempPerm39 = fTemp59;
																																																																																																																																																																																																																		self.fTempPerm40 = fTemp60;
																																																																																																																																																																																																																		self.fTempPerm41 = fTemp61;
																																																																																																																																																																																																																		self.fTempPerm42 = fTemp62;
																																																																																																																																																																																																																		self.fTempPerm43 = fTemp63;
																																																																																																																																																																																																																		self.fTempPerm44 = fTemp64;
																																																																																																																																																																																																																		self.fTempPerm45 = fTemp65;
																																																																																																																																																																																																																		self.fTempPerm46 = fTemp66;
																																																																																																																																																																																																																		self.fTempPerm47 = fTemp67;
																																																																																																																																																																																																																		self.fTempPerm48 = fTemp68;
																																																																																																																																																																																																																		self.fTempPerm49 = fTemp69;
																																																																																																																																																																																																																		self.fTempPerm50 = fTemp70;
																																																																																																																																																																																																																		self.fTempPerm51 = fTemp71;
																																																																																																																																																																																																																		self.fTempPerm52 = fTemp72;
																																																																																																																																																																																																																		self.iTempPerm53 = iTemp73;
																																																																																																																																																																																																																		self.fTempPerm54 = fTemp74;
																																																																																																																																																																																																																		self.fTempPerm55 = fTemp75;
																																																																																																																																																																																																																		self.fTempPerm56 = fTemp76;
																																																																																																																																																																																																																		self.fTempPerm57 = fTemp77;
																																																																																																																																																																																																																		self.fTempPerm58 = fTemp78;
																																																																																																																																																																																																																		self.fTempPerm59 = fTemp79;
																																																																																																																																																																																																																		self.fTempPerm60 = fTemp80;
																																																																																																																																																																																																																		self.fTempPerm61 = fTemp81;
																																																																																																																																																																																																																		self.fTempPerm62 = fTemp82;
																																																																																																																																																																																																																		self.fTempPerm63 = fTemp83;
																																																																																																																																																																																																																		self.fTempPerm64 = fTemp84;
																																																																																																																																																																																																																		self.fTempPerm65 = fTemp85;
																																																																																																																																																																																																																		self.fTempPerm66 = fTemp86;
																																																																																																																																																																																																																		self.fTempPerm67 = fTemp87;
																																																																																																																																																																																																																		self.fTempPerm68 = fTemp88;
																																																																																																																																																																																																																		self.fTempPerm69 = fTemp89;
																																																																																																																																																																																																																		self.fTempPerm70 = fTemp90;
																																																																																																																																																																																																																		self.fTempPerm71 = fTemp91;
																																																																																																																																																																																																																		self.fTempPerm72 = fTemp92;
																																																																																																																																																																																																																		self.fTempPerm73 = fTemp93;
																																																																																																																																																																																																																		self.fTempPerm74 = fTemp94;
																																																																																																																																																																																																																		self.iTempPerm75 = iTemp95;
																																																																																																																																																																																																																		self.fTempPerm76 = fTemp96;
																																																																																																																																																																																																																		self.fTempPerm77 = fTemp97;
																																																																																																																																																																																																																		self.fTempPerm78 = fTemp98;
																																																																																																																																																																																																																		self.fTempPerm79 = fTemp99;
																																																																																																																																																																																																																		self.fTempPerm80 = fTemp100;
																																																																																																																																																																																																																		self.iTempPerm81 = iTemp101;
																																																																																																																																																																																																																		self.fTempPerm82 = fTemp102;
																																																																																																																																																																																																																		self.fTempPerm83 = fTemp103;
																																																																																																																																																																																																																		self.fTempPerm84 = fTemp104;
																																																																																																																																																																																																																		self.fTempPerm85 = fTemp105;
																																																																																																																																																																																																																		self.fTempPerm86 = fTemp106;
																																																																																																																																																																																																																		self.fTempPerm87 = fTemp107;
																																																																																																																																																																																																																		self.fTempPerm88 = fTemp108;
																																																																																																																																																																																																																		self.iTempPerm89 = iTemp109;
																																																																																																																																																																																																																		self.iTempPerm90 = iTemp110;
																																																																																																																																																																																																																		self.iTempPerm91 = iTemp111;
																																																																																																																																																																																																																		self.iTempPerm92 = iTemp112;
																																																																																																																																																																																																																		self.iTempPerm93 = iTemp113;
																																																																																																																																																																																																																		self.iTempPerm94 = iTemp114;
																																																																																																																																																																																																																		self.fTempPerm95 = fTemp115;
																																																																																																																																																																																																																		self.fTempPerm96 = fTemp116;
																																																																																																																																																																																																																		self.fTempPerm97 = fTemp117;
																																																																																																																																																																																																																		self.iTempPerm98 = iTemp118;
																																																																																																																																																																																																																		self.iTempPerm99 = iTemp119;
																																																																																																																																																																																																																		self.iTempPerm100 = iTemp120;
																																																																																																																																																																																																																		self.iTempPerm101 = iTemp121;
																																																																																																																																																																																																																		self.fTempPerm102 = fTemp122;
																																																																																																																																																																																																																		self.fTempPerm103 = fTemp123;
																																																																																																																																																																																																																		self.fTempPerm104 = fTemp124;
																																																																																																																																																																																																																		self.fTempPerm105 = fTemp125;
																																																																																																																																																																																																																		self.fTempPerm106 = fTemp126;
																																																																																																																																																																																																																		self.fTempPerm107 = fTemp127;
																																																																																																																																																																																																																		self.fTempPerm108 = fTemp128;
																																																																																																																																																																																																																		self.fTempPerm109 = fTemp129;
																																																																																																																																																																																																																		self.fTempPerm110 = fTemp130;
																																																																																																																																																																																																																		self.fTempPerm111 = fTemp131;
																																																																																																																																																																																																																		self.fTempPerm112 = fTemp132;
																																																																																																																																																																																																																		self.fTempPerm113 = fTemp133;
																																																																																																																																																																																																																		self.fTempPerm114 = fTemp134;
																																																																																																																																																																																																																		self.fTempPerm115 = fTemp135;
																																																																																																																																																																																																																		self.fTempPerm116 = fTemp136;
																																																																																																																																																																																																																		self.fTempPerm117 = fTemp137;
																																																																																																																																																																																																																		self.fTempPerm118 = fTemp138;
																																																																																																																																																																																																																		self.fTempPerm119 = fTemp139;
																																																																																																																																																																																																																		self.fTempPerm120 = fTemp140;
																																																																																																																																																																																																																		self.iTempPerm121 = iTemp141;
																																																																																																																																																																																																																		self.fTempPerm122 = fTemp142;
																																																																																																																																																																																																																		self.fTempPerm123 = fTemp143;
																																																																																																																																																																																																																		self.fTempPerm124 = fTemp144;
																																																																																																																																																																																																																		self.fTempPerm125 = fTemp145;
																																																																																																																																																																																																																		self.fTempPerm126 = fTemp146;
																																																																																																																																																																																																																		self.fTempPerm127 = fTemp147;
																																																																																																																																																																																																																		self.fTempPerm128 = fTemp148;
																																																																																																																																																																																																																		self.fTempPerm129 = fTemp149;
																																																																																																																																																																																																																		self.fTempPerm130 = fTemp150;
																																																																																																																																																																																																																		self.fTempPerm131 = fTemp151;
																																																																																																																																																																																																																		self.fTempPerm132 = fTemp152;
																																																																																																																																																																																																																		self.fTempPerm133 = fTemp153;
																																																																																																																																																																																																																		self.fTempPerm134 = fTemp154;
																																																																																																																																																																																																																		self.fTempPerm135 = fTemp155;
																																																																																																																																																																																																																		self.iTempPerm136 = iTemp156;
																																																																																																																																																																																																																		self.fTempPerm137 = fTemp157;
																																																																																																																																																																																																																		self.fTempPerm138 = fTemp158;
																																																																																																																																																																																																																		self.fTempPerm139 = fTemp159;
																																																																																																																																																																																																																		self.fTempPerm140 = fTemp160;
																																																																																																																																																																																																																		self.iTempPerm141 = iTemp161;
																																																																																																																																																																																																																		self.fTempPerm142 = fTemp162;
																																																																																																																																																																																																																		self.iTempPerm143 = iTemp163;
																																																																																																																																																																																																																		self.fTempPerm144 = fTemp164;
																																																																																																																																																																																																																		self.fTempPerm145 = fTemp165;
																																																																																																																																																																																																																		self.fTempPerm146 = fTemp166;
																																																																																																																																																																																																																		self.fTempPerm147 = fTemp167;
																																																																																																																																																																																																																		self.fTempPerm148 = fTemp168;
																																																																																																																																																																																																																		self.iTempPerm149 = iTemp169;
																																																																																																																																																																																																																		self.fTempPerm150 = fTemp170;
																																																																																																																																																																																																																		self.fTempPerm151 = fTemp171;
																																																																																																																																																																																																																		self.fTempPerm152 = fTemp172;
																																																																																																																																																																																																																		self.fTempPerm153 = fTemp173;
																																																																																																																																																																																																																		self.fTempPerm154 = fTemp174;
																																																																																																																																																																																																																		self.fTempPerm155 = fTemp175;
																																																																																																																																																																																																																		self.fTempPerm156 = fTemp176;
																																																																																																																																																																																																																		self.fTempPerm157 = fTemp177;
																																																																																																																																																																																																																		self.fTempPerm158 = fTemp178;
																																																																																																																																																																																																																		self.fTempPerm159 = fTemp179;
																																																																																																																																																																																																																		self.fTempPerm160 = fTemp180;
																																																																																																																																																																																																																		self.fTempPerm161 = fTemp181;
																																																																																																																																																																																																																		self.fTempPerm162 = fTemp182;
																																																																																																																																																																																																																		self.fTempPerm163 = fTemp183;
																																																																																																																																																																																																																		self.fTempPerm164 = fTemp184;
																																																																																																																																																																																																																		self.fTempPerm165 = fTemp185;
																																																																																																																																																																																																																		self.fTempPerm166 = fTemp186;
																																																																																																																																																																																																																		self.fTempPerm167 = fTemp187;
																																																																																																																																																																																																																		self.fTempPerm168 = fTemp188;
																																																																																																																																																																																																																		self.fTempPerm169 = fTemp189;
																																																																																																																																																																																																																		self.fTempPerm170 = fTemp190;
																																																																																																																																																																																																																		self.fTempPerm171 = fTemp191;
																																																																																																																																																																																																																		self.iTempPerm172 = iTemp192;
																																																																																																																																																																																																																		self.fTempPerm173 = fTemp193;
																																																																																																																																																																																																																		self.iTempPerm174 = iTemp194;
																																																																																																																																																																																																																		self.fTempPerm175 = fTemp195;
																																																																																																																																																																																																																		self.iTempPerm176 = iTemp196;
																																																																																																																																																																																																																		self.fTempPerm177 = fTemp197;
																																																																																																																																																																																																																		self.iTempPerm178 = iTemp198;
																																																																																																																																																																																																																		self.fTempPerm179 = fTemp199;
																																																																																																																																																																																																																		self.fTempPerm180 = fTemp200;
																																																																																																																																																																																																																		self.fTempPerm181 = fTemp201;
																																																																																																																																																																																																																		self.fTempPerm182 = fTemp202;
																																																																																																																																																																																																																		self.iTempPerm183 = iTemp203;
																																																																																																																																																																																																																		self.fTempPerm184 = fTemp204;
																																																																																																																																																																																																																		self.fTempPerm185 = fTemp205;
																																																																																																																																																																																																																		self.fTempPerm186 = fTemp206;
																																																																																																																																																																																																																		self.fTempPerm187 = fTemp207;
																																																																																																																																																																																																																		self.fTempPerm188 = fTemp208;
																																																																																																																																																																																																																		self.fTempPerm189 = fTemp209;
																																																																																																																																																																																																																		self.fTempPerm190 = fTemp210;
																																																																																																																																																																																																																		self.fTempPerm191 = fTemp211;
																																																																																																																																																																																																																		self.fTempPerm192 = fTemp212;
																																																																																																																																																																																																																		self.fTempPerm193 = fTemp213;
																																																																																																																																																																																																																		self.fTempPerm194 = fTemp214;
																																																																																																																																																																																																																		self.fTempPerm195 = fTemp215;
																																																																																																																																																																																																																		self.fTempPerm196 = fTemp216;
																																																																																																																																																																																																																		self.fTempPerm197 = fTemp217;
																																																																																																																																																																																																																		self.fTempPerm198 = fTemp218;
																																																																																																																																																																																																																		self.fTempPerm199 = fTemp219;
																																																																																																																																																																																																																		self.fTempPerm200 = fTemp220;
																																																																																																																																																																																																																		self.iTempPerm201 = iTemp221;
																																																																																																																																																																																																																		self.fTempPerm202 = fTemp222;
																																																																																																																																																																																																																		self.fTempPerm203 = fTemp223;
																																																																																																																																																																																																																		self.fTempPerm204 = fTemp224;
																																																																																																																																																																																																																		self.fTempPerm205 = fTemp225;
																																																																																																																																																																																																																		self.fTempPerm206 = fTemp226;
																																																																																																																																																																																																																		self.fTempPerm207 = fTemp227;
																																																																																																																																																																																																																		self.fTempPerm208 = fTemp228;
																																																																																																																																																																																																																		self.fTempPerm209 = fTemp229;
																																																																																																																																																																																																																		self.fTempPerm210 = fTemp230;
																																																																																																																																																																																																																		self.fTempPerm211 = fTemp231;
																																																																																																																																																																																																																		self.fTempPerm212 = fTemp232;
																																																																																																																																																																																																																		self.fTempPerm213 = fTemp233;
																																																																																																																																																																																																																		self.fTempPerm214 = fTemp234;
																																																																																																																																																																																																																		self.fTempPerm215 = fTemp235;
																																																																																																																																																																																																																		self.fTempPerm216 = fTemp236;
																																																																																																																																																																																																																		self.fTempPerm217 = fTemp237;
																																																																																																																																																																																																																		self.fTempPerm218 = fTemp238;
																																																																																																																																																																																																																		self.iTempPerm219 = iTemp239;
																																																																																																																																																																																																																		self.fTempPerm220 = fTemp240;
																																																																																																																																																																																																																		self.fTempPerm221 = fTemp241;
																																																																																																																																																																																																																		self.fTempPerm222 = fTemp242;
																																																																																																																																																																																																																		self.fTempPerm223 = fTemp243;
																																																																																																																																																																																																																		self.iTempPerm224 = iTemp244;
																																																																																																																																																																																																																		self.fTempPerm225 = fTemp245;
																																																																																																																																																																																																																		self.iTempPerm226 = iTemp246;
																																																																																																																																																																																																																		self.fTempPerm227 = fTemp247;
																																																																																																																																																																																																																		self.fTempPerm228 = fTemp248;
																																																																																																																																																																																																																		self.fTempPerm229 = fTemp249;
																																																																																																																																																																																																																		self.fTempPerm230 = fTemp250;
																																																																																																																																																																																																																		self.fTempPerm231 = fTemp251;
																																																																																																																																																																																																																		self.fTempPerm232 = fTemp252;
																																																																																																																																																																																																																		self.fTempPerm233 = fTemp253;
																																																																																																																																																																																																																		self.fTempPerm234 = fTemp254;
																																																																																																																																																																																																																		self.fTempPerm235 = fTemp255;
																																																																																																																																																																																																																		self.fTempPerm236 = fTemp256;
																																																																																																																																																																																																																		self.fTempPerm237 = fTemp257;
																																																																																																																																																																																																																		self.fTempPerm238 = fTemp258;
																																																																																																																																																																																																																		self.fTempPerm239 = fTemp259;
																																																																																																																																																																																																																		self.fTempPerm240 = fTemp260;
																																																																																																																																																																																																																		self.fTempPerm241 = fTemp261;
																																																																																																																																																																																																																		self.fTempPerm242 = fTemp262;
																																																																																																																																																																																																																		self.iTempPerm243 = iTemp263;
																																																																																																																																																																																																																		self.iTempPerm244 = iTemp264;
																																																																																																																																																																																																																		self.iTempPerm245 = iTemp265;
																																																																																																																																																																																																																		self.iTempPerm246 = iTemp266;
																																																																																																																																																																																																																		self.fTempPerm247 = fTemp267;
																																																																																																																																																																																																																		self.iTempPerm248 = iTemp268;
																																																																																																																																																																																																																		self.iTempPerm249 = iTemp269;
																																																																																																																																																																																																																		self.iTempPerm250 = iTemp270;
																																																																																																																																																																																																																		self.iTempPerm251 = iTemp271;
																																																																																																																																																																																																																		self.iTempPerm252 = iTemp272;
																																																																																																																																																																																																																		self.iTempPerm253 = iTemp273;
																																																																																																																																																																																																																		self.iTempPerm254 = iTemp274;
																																																																																																																																																																																																																		self.fTempPerm255 = fTemp275;
																																																																																																																																																																																																																		self.iTempPerm256 = iTemp276;
																																																																																																																																																																																																																		self.iTempPerm257 = iTemp277;
																																																																																																																																																																																																																		self.fTempPerm258 = fTemp278;
																																																																																																																																																																																																																		self.fTempPerm259 = fTemp279;
																																																																																																																																																																																																																		self.fTempPerm260 = fTemp280;
																																																																																																																																																																																																																		self.fTempPerm261 = fTemp281;
																																																																																																																																																																																																																		self.fTempPerm262 = fTemp282;
																																																																																																																																																																																																																		self.iTempPerm263 = iTemp283;
																																																																																																																																																																																																																		self.fTempPerm264 = fTemp284;
																																																																																																																																																																																																																		self.fTempPerm265 = fTemp285;
																																																																																																																																																																																																																		self.fTempPerm266 = fTemp286;
																																																																																																																																																																																																																		self.fTempPerm267 = fTemp287;
																																																																																																																																																																																																																		self.fTempPerm268 = fTemp288;
																																																																																																																																																																																																																		self.fTempPerm269 = fTemp289;
																																																																																																																																																																																																																		self.fTempPerm270 = fTemp290;
																																																																																																																																																																																																																		self.iTempPerm271 = iTemp291;
																																																																																																																																																																																																																		self.iTempPerm272 = iTemp292;
																																																																																																																																																																																																																		self.iTempPerm273 = iTemp293;
																																																																																																																																																																																																																		self.iTempPerm274 = iTemp294;
																																																																																																																																																																																																																		self.iTempPerm275 = iTemp295;
																																																																																																																																																																																																																		self.iTempPerm276 = iTemp296;
																																																																																																																																																																																																																		self.fTempPerm277 = fTemp297;
																																																																																																																																																																																																																		self.fTempPerm278 = fTemp298;
																																																																																																																																																																																																																		self.iTempPerm279 = iTemp299;
																																																																																																																																																																																																																		self.iTempPerm280 = iTemp300;
																																																																																																																																																																																																																		self.iTempPerm281 = iTemp301;
																																																																																																																																																																																																																		self.iTempPerm282 = iTemp302;
																																																																																																																																																																																																																		self.fTempPerm283 = fTemp303;
																																																																																																																																																																																																																		self.fTempPerm284 = fTemp304;
																																																																																																																																																																																																																		self.fTempPerm285 = fTemp305;
																																																																																																																																																																																																																		self.iTempPerm286 = iTemp306;
																																																																																																																																																																																																																		self.fTempPerm287 = fTemp307;
																																																																																																																																																																																																																		self.fTempPerm288 = fTemp308;
																																																																																																																																																																																																																		self.fTempPerm289 = fTemp309;
																																																																																																																																																																																																																		self.fTempPerm290 = fTemp310;
																																																																																																																																																																																																																		self.iTempPerm291 = iTemp311;
																																																																																																																																																																																																																		self.iTempPerm292 = iTemp312;
																																																																																																																																																																																																																		self.iTempPerm293 = iTemp313;
																																																																																																																																																																																																																		self.iTempPerm294 = iTemp314;
																																																																																																																																																																																																																		self.iTempPerm295 = iTemp315;
																																																																																																																																																																																																																		self.iTempPerm296 = iTemp316;
																																																																																																																																																																																																																		self.iTempPerm297 = iTemp317;
																																																																																																																																																																																																																		self.fTempPerm298 = fTemp318;
																																																																																																																																																																																																																		self.iTempPerm299 = iTemp319;
																																																																																																																																																																																																																		self.iTempPerm300 = iTemp320;
																																																																																																																																																																																																																		self.fTempPerm301 = fTemp321;
																																																																																																																																																																																																																		self.fTempPerm302 = fTemp322;
																																																																																																																																																																																																																		self.fTempPerm303 = fTemp323;
																																																																																																																																																																																																																		self.fTempPerm304 = fTemp324;
																																																																																																																																																																																																																		self.fTempPerm305 = fTemp325;
																																																																																																																																																																																																																		self.fTempPerm306 = fTemp326;
																																																																																																																																																																																																																		self.fTempPerm307 = fTemp327;
																																																																																																																																																																																																																		self.fTempPerm308 = fTemp328;
																																																																																																																																																																																																																		self.iTempPerm309 = iTemp329;
																																																																																																																																																																																																																		self.iTempPerm310 = iTemp330;
																																																																																																																																																																																																																		self.fTempPerm311 = fTemp331;
																																																																																																																																																																																																																		self.fTempPerm312 = fTemp332;
																																																																																																																																																																																																																		self.fTempPerm313 = fTemp333;
																																																																																																																																																																																																																		self.fTempPerm314 = fTemp334;
																																																																																																																																																																																																																		self.fTempPerm315 = fTemp335;
																																																																																																																																																																																																																		self.fTempPerm316 = fTemp336;
																																																																																																																																																																																																																		self.fTempPerm317 = fTemp337;
																																																																																																																																																																																																																		self.fTempPerm318 = fTemp338;
																																																																																																																																																																																																																		self.fTempPerm319 = fTemp339;
																																																																																																																																																																																																																		self.fTempPerm320 = fTemp340;
																																																																																																																																																																																																																		self.fTempPerm321 = fTemp341;
																																																																																																																																																																																																																		self.fTempPerm322 = fTemp342;
																																																																																																																																																																																																																		self.fTempPerm323 = fTemp343;
																																																																																																																																																																																																																		self.fTempPerm324 = fTemp344;
																																																																																																																																																																																																																		self.fTempPerm325 = fTemp345;
																																																																																																																																																																																																																		self.fTempPerm326 = fTemp346;
																																																																																																																																																																																																																		self.fTempPerm327 = fTemp347;
																																																																																																																																																																																																																		self.fTempPerm328 = fTemp348;
																																																																																																																																																																																																																		self.fTempPerm329 = fTemp349;
																																																																																																																																																																																																																		self.fTempPerm330 = fTemp350;
																																																																																																																																																																																																																		self.fTempPerm331 = fTemp355;
																																																																																																																																																																																																																		self.fTempPerm332 = fTemp356;
																																																																																																																																																																																																																		self.iTempPerm333 = iTemp357;
																																																																																																																																																																																																																		self.fTempPerm334 = fTemp358;
																																																																																																																																																																																																																		self.fTempPerm335 = fTemp359;
																																																																																																																																																																																																																		self.fTempPerm336 = fTemp360;
																																																																																																																																																																																																																		self.fTempPerm337 = fTemp361;
																																																																																																																																																																																																																		self.fTempPerm338 = fTemp362;
																																																																																																																																																																																																																		self.fTempPerm339 = fTemp363;
																																																																																																																																																																																																																		self.fTempPerm340 = fTemp364;
																																																																																																																																																																																																																		self.fTempPerm341 = fTemp365;
																																																																																																																																																																																																																		self.fTempPerm342 = fTemp366;
																																																																																																																																																																																																																		self.fTempPerm343 = fTemp367;
																																																																																																																																																																																																																		self.fTempPerm344 = fTemp368;
																																																																																																																																																																																																																		self.fTempPerm345 = fTemp369;
																																																																																																																																																																																																																		self.fTempPerm346 = fTemp370;
																																																																																																																																																																																																																		self.iTempPerm347 = iTemp371;
																																																																																																																																																																																																																		self.fTempPerm348 = fTemp372;
																																																																																																																																																																																																																		self.fTempPerm349 = fTemp373;
																																																																																																																																																																																																																		self.fTempPerm350 = fTemp374;
																																																																																																																																																																																																																		self.fTempPerm351 = fTemp375;
																																																																																																																																																																																																																		self.fTempPerm352 = fTemp376;
																																																																																																																																																																																																																		self.fTempPerm353 = fTemp377;
																																																																																																																																																																																																																		self.fTempPerm354 = fTemp378;
																																																																																																																																																																																																																		self.fTempPerm355 = fTemp379;
																																																																																																																																																																																																																		self.fTempPerm356 = fTemp380;
																																																																																																																																																																																																																		self.fTempPerm357 = fTemp381;
																																																																																																																																																																																																																		self.fTempPerm358 = fTemp382;
																																																																																																																																																																																																																		self.fTempPerm359 = fTemp383;
																																																																																																																																																																																																																		self.fTempPerm360 = fTemp384;
																																																																																																																																																																																																																		self.fTempPerm361 = fTemp385;
																																																																																																																																																																																																																		self.fTempPerm362 = fTemp386;
																																																																																																																																																																																																																		self.fTempPerm363 = fTemp387;
																																																																																																																																																																																																																		self.fTempPerm364 = fTemp388;
																																																																																																																																																																																																																		self.fTempPerm365 = fTemp389;
																																																																																																																																																																																																																		self.fTempPerm366 = fTemp390;
																																																																																																																																																																																																																		self.fTempPerm367 = fTemp391;
																																																																																																																																																																																																																		self.fTempPerm368 = fTemp392;
																																																																																																																																																																																																																		self.fTempPerm369 = fTemp393;
																																																																																																																																																																																																																		self.fTempPerm370 = fTemp394;
																																																																																																																																																																																																																		self.fTempPerm371 = fTemp395;
																																																																																																																																																																																																																		self.fTempPerm372 = fTemp396;
																																																																																																																																																																																																																		self.iTempPerm373 = iTemp397;
																																																																																																																																																																																																																		self.fTempPerm374 = fTemp398;
																																																																																																																																																																																																																		self.fTempPerm375 = fTemp399;
																																																																																																																																																																																																																		self.fTempPerm376 = fTemp400;
																																																																																																																																																																																																																		self.fTempPerm377 = fTemp401;
																																																																																																																																																																																																																		self.fTempPerm378 = fTemp402;
																																																																																																																																																																																																																		self.fTempPerm379 = fTemp403;
																																																																																																																																																																																																																		self.fTempPerm380 = fTemp404;
																																																																																																																																																																																																																		self.fTempPerm381 = fTemp405;
																																																																																																																																																																																																																		self.fTempPerm382 = fTemp406;
																																																																																																																																																																																																																		self.fTempPerm383 = fTemp407;
																																																																																																																																																																																																																		self.fTempPerm384 = fTemp408;
																																																																																																																																																																																																																		self.fTempPerm385 = fTemp409;
																																																																																																																																																																																																																		self.fTempPerm386 = fTemp410;
																																																																																																																																																																																																																		self.fTempPerm387 = fTemp411;
																																																																																																																																																																																																																		self.fTempPerm388 = fTemp412;
																																																																																																																																																																																																																		self.fTempPerm389 = fTemp413;
																																																																																																																																																																																																																		self.fTempPerm390 = fTemp414;
																																																																																																																																																																																																																		self.fTempPerm391 = fTemp415;
																																																																																																																																																																																																																		self.fTempPerm392 = fTemp416;
																																																																																																																																																																																																																		self.fTempPerm393 = fTemp417;
																																																																																																																																																																																																																		self.fTempPerm394 = fTemp418;
																																																																																																																																																																																																																		self.iTempPerm395 = iTemp419;
																																																																																																																																																																																																																		self.fTempPerm396 = fTemp420;
																																																																																																																																																																																																																		self.fTempPerm397 = fTemp421;
																																																																																																																																																																																																																		self.fTempPerm398 = fTemp422;
																																																																																																																																																																																																																		self.fTempPerm399 = fTemp423;
																																																																																																																																																																																																																		self.fTempPerm400 = fTemp424;
																																																																																																																																																																																																																		self.iTempPerm401 = iTemp425;
																																																																																																																																																																																																																		self.fTempPerm402 = fTemp426;
																																																																																																																																																																																																																		self.fTempPerm403 = fTemp427;
																																																																																																																																																																																																																		self.fTempPerm404 = fTemp428;
																																																																																																																																																																																																																		self.fTempPerm405 = fTemp429;
																																																																																																																																																																																																																		self.fTempPerm406 = fTemp430;
																																																																																																																																																																																																																		self.fTempPerm407 = fTemp431;
																																																																																																																																																																																																																		self.fTempPerm408 = fTemp432;
																																																																																																																																																																																																																		self.iTempPerm409 = iTemp433;
																																																																																																																																																																																																																		self.iTempPerm410 = iTemp434;
																																																																																																																																																																																																																		self.iTempPerm411 = iTemp435;
																																																																																																																																																																																																																		self.iTempPerm412 = iTemp436;
																																																																																																																																																																																																																		self.iTempPerm413 = iTemp437;
																																																																																																																																																																																																																		self.iTempPerm414 = iTemp438;
																																																																																																																																																																																																																		self.fTempPerm415 = fTemp439;
																																																																																																																																																																																																																		self.fTempPerm416 = fTemp440;
																																																																																																																																																																																																																		self.fTempPerm417 = fTemp441;
																																																																																																																																																																																																																		self.iTempPerm418 = iTemp442;
																																																																																																																																																																																																																		self.iTempPerm419 = iTemp443;
																																																																																																																																																																																																																		self.iTempPerm420 = iTemp444;
																																																																																																																																																																																																																		self.iTempPerm421 = iTemp445;
																																																																																																																																																																																																																		self.fTempPerm422 = fTemp446;
																																																																																																																																																																																																																		self.fTempPerm423 = fTemp447;
																																																																																																																																																																																																																		self.fTempPerm424 = fTemp448;
																																																																																																																																																																																																																		self.fTempPerm425 = fTemp449;
																																																																																																																																																																																																																		self.fTempPerm426 = fTemp450;
																																																																																																																																																																																																																		self.fTempPerm427 = fTemp451;
																																																																																																																																																																																																																		self.fTempPerm428 = fTemp452;
																																																																																																																																																																																																																		self.fTempPerm429 = fTemp453;
																																																																																																																																																																																																																		self.fTempPerm430 = fTemp454;
																																																																																																																																																																																																																		self.fTempPerm431 = fTemp455;
																																																																																																																																																																																																																		self.fTempPerm432 = fTemp456;
																																																																																																																																																																																																																		self.fTempPerm433 = fTemp457;
																																																																																																																																																																																																																		self.fTempPerm434 = fTemp458;
																																																																																																																																																																																																																		self.fTempPerm435 = fTemp459;
																																																																																																																																																																																																																		self.fTempPerm436 = fTemp460;
																																																																																																																																																																																																																		self.iTempPerm437 = iTemp461;
																																																																																																																																																																																																																		self.fTempPerm438 = fTemp462;
																																																																																																																																																																																																																		self.fTempPerm439 = fTemp463;
																																																																																																																																																																																																																		self.fTempPerm440 = fTemp464;
																																																																																																																																																																																																																		self.fTempPerm441 = fTemp465;
																																																																																																																																																																																																																		self.fTempPerm442 = fTemp466;
																																																																																																																																																																																																																		self.fTempPerm443 = fTemp467;
																																																																																																																																																																																																																		self.fTempPerm444 = fTemp468;
																																																																																																																																																																																																																		self.fTempPerm445 = fTemp469;
																																																																																																																																																																																																																		self.fTempPerm446 = fTemp470;
																																																																																																																																																																																																																		self.fTempPerm447 = fTemp471;
																																																																																																																																																																																																																		self.fTempPerm448 = fTemp472;
																																																																																																																																																																																																																		self.fTempPerm449 = fTemp473;
																																																																																																																																																																																																																		self.fTempPerm450 = fTemp474;
																																																																																																																																																																																																																		self.fTempPerm451 = fTemp475;
																																																																																																																																																																																																																		self.iTempPerm452 = iTemp476;
																																																																																																																																																																																																																		self.fTempPerm453 = fTemp477;
																																																																																																																																																																																																																		self.fTempPerm454 = fTemp478;
																																																																																																																																																																																																																		self.fTempPerm455 = fTemp479;
																																																																																																																																																																																																																		self.fTempPerm456 = fTemp480;
																																																																																																																																																																																																																		self.iTempPerm457 = iTemp481;
																																																																																																																																																																																																																		self.fTempPerm458 = fTemp482;
																																																																																																																																																																																																																		self.iTempPerm459 = iTemp483;
																																																																																																																																																																																																																		self.fTempPerm460 = fTemp484;
																																																																																																																																																																																																																		self.fTempPerm461 = fTemp485;
																																																																																																																																																																																																																		self.fTempPerm462 = fTemp486;
																																																																																																																																																																																																																		self.fTempPerm463 = fTemp487;
																																																																																																																																																																																																																		self.fTempPerm464 = fTemp488;
																																																																																																																																																																																																																		self.iTempPerm465 = iTemp489;
																																																																																																																																																																																																																		self.fTempPerm466 = fTemp490;
																																																																																																																																																																																																																		self.fTempPerm467 = fTemp491;
																																																																																																																																																																																																																		self.fTempPerm468 = fTemp492;
																																																																																																																																																																																																																		self.fTempPerm469 = fTemp493;
																																																																																																																																																																																																																		self.fTempPerm470 = fTemp494;
																																																																																																																																																																																																																		self.fTempPerm471 = fTemp495;
																																																																																																																																																																																																																		self.fTempPerm472 = fTemp496;
																																																																																																																																																																																																																		self.fTempPerm473 = fTemp497;
																																																																																																																																																																																																																		self.fTempPerm474 = fTemp498;
																																																																																																																																																																																																																		self.fTempPerm475 = fTemp499;
																																																																																																																																																																																																																		self.fTempPerm476 = fTemp500;
																																																																																																																																																																																																																		self.fTempPerm477 = fTemp501;
																																																																																																																																																																																																																		self.fTempPerm478 = fTemp502;
																																																																																																																																																																																																																		self.fTempPerm479 = fTemp503;
																																																																																																																																																																																																																		self.fTempPerm480 = fTemp504;
																																																																																																																																																																																																																		self.fTempPerm481 = fTemp505;
																																																																																																																																																																																																																		self.fTempPerm482 = fTemp506;
																																																																																																																																																																																																																		self.fTempPerm483 = fTemp507;
																																																																																																																																																																																																																		self.fTempPerm484 = fTemp508;
																																																																																																																																																																																																																		self.fTempPerm485 = fTemp509;
																																																																																																																																																																																																																		self.fTempPerm486 = fTemp510;
																																																																																																																																																																																																																		self.iTempPerm487 = iTemp511;
																																																																																																																																																																																																																		self.fTempPerm488 = fTemp512;
																																																																																																																																																																																																																		self.iTempPerm489 = iTemp513;
																																																																																																																																																																																																																		self.fTempPerm490 = fTemp514;
																																																																																																																																																																																																																		self.iTempPerm491 = iTemp515;
																																																																																																																																																																																																																		self.fTempPerm492 = fTemp516;
																																																																																																																																																																																																																		self.iTempPerm493 = iTemp517;
																																																																																																																																																																																																																		self.fTempPerm494 = fTemp518;
																																																																																																																																																																																																																		self.fTempPerm495 = fTemp519;
																																																																																																																																																																																																																		self.fTempPerm496 = fTemp520;
																																																																																																																																																																																																																		self.fTempPerm497 = fTemp521;
																																																																																																																																																																																																																		self.iTempPerm498 = iTemp522;
																																																																																																																																																																																																																		self.fTempPerm499 = fTemp523;
																																																																																																																																																																																																																		self.fTempPerm500 = fTemp524;
																																																																																																																																																																																																																		self.fTempPerm501 = fTemp525;
																																																																																																																																																																																																																		self.fTempPerm502 = fTemp526;
																																																																																																																																																																																																																		self.fTempPerm503 = fTemp527;
																																																																																																																																																																																																																		self.fTempPerm504 = fTemp528;
																																																																																																																																																																																																																		self.fTempPerm505 = fTemp529;
																																																																																																																																																																																																																		self.fTempPerm506 = fTemp530;
																																																																																																																																																																																																																		self.fTempPerm507 = fTemp531;
																																																																																																																																																																																																																		self.fTempPerm508 = fTemp532;
																																																																																																																																																																																																																		self.fTempPerm509 = fTemp533;
																																																																																																																																																																																																																		self.fTempPerm510 = fTemp534;
																																																																																																																																																																																																																		self.fTempPerm511 = fTemp535;
																																																																																																																																																																																																																		self.fTempPerm512 = fTemp536;
																																																																																																																																																																																																																		self.fTempPerm513 = fTemp537;
																																																																																																																																																																																																																		self.fTempPerm514 = fTemp538;
																																																																																																																																																																																																																		self.fTempPerm515 = fTemp539;
																																																																																																																																																																																																																		self.iTempPerm516 = iTemp540;
																																																																																																																																																																																																																		self.fTempPerm517 = fTemp541;
																																																																																																																																																																																																																		self.fTempPerm518 = fTemp542;
																																																																																																																																																																																																																		self.fTempPerm519 = fTemp543;
																																																																																																																																																																																																																		self.fTempPerm520 = fTemp544;
																																																																																																																																																																																																																		self.fTempPerm521 = fTemp545;
																																																																																																																																																																																																																		self.fTempPerm522 = fTemp546;
																																																																																																																																																																																																																		self.fTempPerm523 = fTemp547;
																																																																																																																																																																																																																		self.fTempPerm524 = fTemp548;
																																																																																																																																																																																																																		self.fTempPerm525 = fTemp549;
																																																																																																																																																																																																																		self.fTempPerm526 = fTemp550;
																																																																																																																																																																																																																		self.fTempPerm527 = fTemp551;
																																																																																																																																																																																																																		self.fTempPerm528 = fTemp552;
																																																																																																																																																																																																																		self.fTempPerm529 = fTemp553;
																																																																																																																																																																																																																		self.fTempPerm530 = fTemp554;
																																																																																																																																																																																																																		self.fTempPerm531 = fTemp555;
																																																																																																																																																																																																																		self.fTempPerm532 = fTemp556;
																																																																																																																																																																																																																		self.fTempPerm533 = fTemp557;
																																																																																																																																																																																																																		self.iTempPerm534 = iTemp558;
																																																																																																																																																																																																																		self.fTempPerm535 = fTemp559;
																																																																																																																																																																																																																		self.fTempPerm536 = fTemp560;
																																																																																																																																																																																																																		self.fTempPerm537 = fTemp561;
																																																																																																																																																																																																																		self.fTempPerm538 = fTemp562;
																																																																																																																																																																																																																		self.iTempPerm539 = iTemp563;
																																																																																																																																																																																																																		self.fTempPerm540 = fTemp564;
																																																																																																																																																																																																																		self.iTempPerm541 = iTemp565;
																																																																																																																																																																																																																		self.fTempPerm542 = fTemp566;
																																																																																																																																																																																																																		self.fTempPerm543 = fTemp567;
																																																																																																																																																																																																																		self.fTempPerm544 = fTemp568;
																																																																																																																																																																																																																		self.fTempPerm545 = fTemp569;
																																																																																																																																																																																																																		self.fTempPerm546 = fTemp570;
																																																																																																																																																																																																																		self.fTempPerm547 = fTemp571;
																																																																																																																																																																																																																		self.fTempPerm548 = fTemp572;
																																																																																																																																																																																																																		self.fTempPerm549 = fTemp573;
																																																																																																																																																																																																																		self.fTempPerm550 = fTemp574;
																																																																																																																																																																																																																		self.fTempPerm551 = fTemp575;
																																																																																																																																																																																																																		self.iTempPerm552 = iTemp576;
																																																																																																																																																																																																																		self.iTempPerm553 = iTemp577;
																																																																																																																																																																																																																		self.iTempPerm554 = iTemp578;
																																																																																																																																																																																																																		self.iTempPerm555 = iTemp579;
																																																																																																																																																																																																																		self.fTempPerm556 = fTemp580;
																																																																																																																																																																																																																		self.iTempPerm557 = iTemp581;
																																																																																																																																																																																																																		self.iTempPerm558 = iTemp582;
																																																																																																																																																																																																																		self.iTempPerm559 = iTemp583;
																																																																																																																																																																																																																		self.iTempPerm560 = iTemp584;
																																																																																																																																																																																																																		self.iTempPerm561 = iTemp585;
																																																																																																																																																																																																																		self.iTempPerm562 = iTemp586;
																																																																																																																																																																																																																		self.iTempPerm563 = iTemp587;
																																																																																																																																																																																																																		self.fTempPerm564 = fTemp588;
																																																																																																																																																																																																																		self.iTempPerm565 = iTemp589;
																																																																																																																																																																																																																		self.iTempPerm566 = iTemp590;
																																																																																																																																																																																																																		self.fTempPerm567 = fTemp591;
																																																																																																																																																																																																																		self.fTempPerm568 = fTemp592;
																																																																																																																																																																																																																		self.fTempPerm569 = fTemp593;
																																																																																																																																																																																																																		self.fTempPerm570 = fTemp594;
																																																																																																																																																																																																																		self.fTempPerm571 = fTemp595;
																																																																																																																																																																																																																		self.iTempPerm572 = iTemp596;
																																																																																																																																																																																																																		self.fTempPerm573 = fTemp597;
																																																																																																																																																																																																																		self.fTempPerm574 = fTemp598;
																																																																																																																																																																																																																		self.fTempPerm575 = fTemp599;
																																																																																																																																																																																																																		self.fTempPerm576 = fTemp600;
																																																																																																																																																																																																																		self.fTempPerm577 = fTemp601;
																																																																																																																																																																																																																		self.fTempPerm578 = fTemp602;
																																																																																																																																																																																																																		self.fTempPerm579 = fTemp603;
																																																																																																																																																																																																																		self.iTempPerm580 = iTemp604;
																																																																																																																																																																																																																		self.iTempPerm581 = iTemp605;
																																																																																																																																																																																																																		self.iTempPerm582 = iTemp606;
																																																																																																																																																																																																																		self.iTempPerm583 = iTemp607;
																																																																																																																																																																																																																		self.iTempPerm584 = iTemp608;
																																																																																																																																																																																																																		self.iTempPerm585 = iTemp609;
																																																																																																																																																																																																																		self.fTempPerm586 = fTemp610;
																																																																																																																																																																																																																		self.fTempPerm587 = fTemp611;
																																																																																																																																																																																																																		self.iTempPerm588 = iTemp612;
																																																																																																																																																																																																																		self.iTempPerm589 = iTemp613;
																																																																																																																																																																																																																		self.iTempPerm590 = iTemp614;
																																																																																																																																																																																																																		self.iTempPerm591 = iTemp615;
																																																																																																																																																																																																																		self.fTempPerm592 = fTemp616;
																																																																																																																																																																																																																		self.fTempPerm593 = fTemp617;
																																																																																																																																																																																																																		self.fTempPerm594 = fTemp618;
																																																																																																																																																																																																																		self.iTempPerm595 = iTemp619;
																																																																																																																																																																																																																		self.fTempPerm596 = fTemp620;
																																																																																																																																																																																																																		self.fTempPerm597 = fTemp621;
																																																																																																																																																																																																																		self.fTempPerm598 = fTemp622;
																																																																																																																																																																																																																		self.fTempPerm599 = fTemp623;
																																																																																																																																																																																																																		self.iTempPerm600 = iTemp624;
																																																																																																																																																																																																																		self.iTempPerm601 = iTemp625;
																																																																																																																																																																																																																		self.iTempPerm602 = iTemp626;
																																																																																																																																																																																																																		self.iTempPerm603 = iTemp627;
																																																																																																																																																																																																																		self.iTempPerm604 = iTemp628;
																																																																																																																																																																																																																		self.iTempPerm605 = iTemp629;
																																																																																																																																																																																																																		self.iTempPerm606 = iTemp630;
																																																																																																																																																																																																																		self.fTempPerm607 = fTemp631;
																																																																																																																																																																																																																		self.iTempPerm608 = iTemp632;
																																																																																																																																																																																																																		self.iTempPerm609 = iTemp633;
																																																																																																																																																																																																																		self.fTempPerm610 = fTemp634;
																																																																																																																																																																																																																		self.fTempPerm611 = fTemp635;
																																																																																																																																																																																																																		self.fTempPerm612 = fTemp636;
																																																																																																																																																																																																																		self.fTempPerm613 = fTemp637;
																																																																																																																																																																																																																		self.fTempPerm614 = fTemp638;
																																																																																																																																																																																																																		self.fTempPerm615 = fTemp639;
																																																																																																																																																																																																																		self.fTempPerm616 = fTemp640;
																																																																																																																																																																																																																		self.fTempPerm617 = fTemp641;
																																																																																																																																																																																																																		self.iTempPerm618 = iTemp642;
																																																																																																																																																																																																																		self.iTempPerm619 = iTemp643;
																																																																																																																																																																																																																		self.fTempPerm620 = fTemp644;
																																																																																																																																																																																																																		self.fTempPerm621 = fTemp645;
																																																																																																																																																																																																																		self.fTempPerm622 = fTemp646;
																																																																																																																																																																																																																		self.fTempPerm623 = fTemp647;
																																																																																																																																																																																																																		self.fTempPerm624 = fTemp648;
																																																																																																																																																																																																																		self.fTempPerm625 = fTemp649;
																																																																																																																																																																																																																		self.fTempPerm626 = fTemp650;
																																																																																																																																																																																																																		self.fTempPerm627 = fTemp651;
																																																																																																																																																																																																																		self.fTempPerm628 = fTemp652;
																																																																																																																																																																																																																		self.fTempPerm629 = fTemp653;
																																																																																																																																																																																																																		self.fTempPerm630 = fTemp654;
																																																																																																																																																																																																																		self.fTempPerm631 = fTemp655;
																																																																																																																																																																																																																		self.fTempPerm632 = fTemp656;
																																																																																																																																																																																																																		self.fTempPerm633 = fTemp657;
																																																																																																																																																																																																																		self.fTempPerm634 = fTemp658;
																																																																																																																																																																																																																		self.fTempPerm635 = fTemp659;
																																																																																																																																																																																																																		self.fTempPerm636 = fTemp660;
																																																																																																																																																																																																																		self.fTempPerm637 = fTemp661;
																																																																																																																																																																																																																		self.fTempPerm638 = fTemp662;
																																																																																																																																																																																																																		self.fTempPerm639 = fTemp663;
																																																																																																																																																																																																																	}

}

