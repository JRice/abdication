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
	fHslider0: F32,
	fEntry0: F32,
	fEntry1: F32,
	fEntry2: F32,
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	fHslider1: F32,
	fVec2: [F32;2],
	fHslider2: F32,
	fRec1: [F32;2],
	fRec2: [F32;2],
	fHslider3: F32,
	fVec3: [F32;2],
	fRec3: [F32;2],
	fRec4: [F32;2],
	fEntry3: F32,
	fEntry4: F32,
	iTempPerm0: i32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fTempPerm1: F32,
	fRec6: [F32;2],
	fHslider4: F32,
	fConst2: F32,
	fTempPerm2: F32,
	fRec7: [F32;2],
	iRec9: [i32;2],
	iTempPerm3: i32,
	fConst3: F32,
	fConst5: F32,
	fRec8: [F32;2],
	fConst6: F32,
	fHslider5: F32,
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
	fHslider6: F32,
	fRec15: [F32;2],
	fEntry5: F32,
	fHslider7: F32,
	fVec7: [F32;2],
	fRec16: [F32;2],
	fRec17: [F32;2],
	fHslider8: F32,
	fVec8: [F32;2],
	fRec18: [F32;2],
	fRec19: [F32;2],
	fHslider9: F32,
	fHslider10: F32,
	fRec20: [F32;2],
	fTempPerm7: F32,
	fTempPerm8: F32,
	fRec21: [F32;2],
	fHslider11: F32,
	fHslider12: F32,
	fCheckbox0: F32,
	fEntry6: F32,
	fTempPerm9: F32,
	fRec0: [F32;2],
	iTempPerm10: i32,
	fHslider13: F32,
	fVec9: [F32;2],
	fTempPerm11: F32,
	fRec22: [F32;2],
	fRec23: [F32;2],
	fTempPerm12: F32,
	fHslider14: F32,
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
	fHslider15: F32,
	fVec38: [F32;2],
	fTempPerm241: F32,
	fRec42: [F32;2],
	fRec43: [F32;2],
	fTempPerm242: F32,
	iTempPerm243: i32,
	fTempPerm244: F32,
	iTempPerm245: i32,
	fConst16: F32,
	fConst17: F32,
	fTempPerm246: F32,
	fConst18: F32,
	fConst19: F32,
	fConst20: F32,
	fConst21: F32,
	fConst22: F32,
	iTempPerm247: i32,
	iTempPerm248: i32,
	iTempPerm249: i32,
	iTempPerm250: i32,
	iTempPerm251: i32,
	iTempPerm252: i32,
	fTempPerm253: F32,
	iTempPerm254: i32,
	iTempPerm255: i32,
	iTempPerm256: i32,
	fEntry9: F32,
	fTempPerm257: F32,
	fTempPerm258: F32,
	fTempPerm259: F32,
	fTempPerm260: F32,
	fTempPerm261: F32,
	fHslider16: F32,
	fVec39: [F32;2],
	fTempPerm262: F32,
	fRec44: [F32;2],
	fRec45: [F32;2],
	fTempPerm263: F32,
	iTempPerm264: i32,
	iTempPerm265: i32,
	iTempPerm266: i32,
	fEntry10: F32,
	fEntry11: F32,
	fEntry12: F32,
	iTempPerm267: i32,
	fTempPerm268: F32,
	fTempPerm269: F32,
	fEntry13: F32,
	iTempPerm270: i32,
	fTempPerm271: F32,
	fTempPerm272: F32,
	fTempPerm273: F32,
	fHslider17: F32,
	fVec40: [F32;2],
	fTempPerm274: F32,
	fRec46: [F32;2],
	fRec47: [F32;2],
	fTempPerm275: F32,
	fTempPerm276: F32,
	fTempPerm277: F32,
	fTempPerm278: F32,
	iTempPerm279: i32,
	iTempPerm280: i32,
	iTempPerm281: i32,
	iTempPerm282: i32,
	iTempPerm283: i32,
	iTempPerm284: i32,
	fTempPerm285: F32,
	fTempPerm286: F32,
	iTempPerm287: i32,
	iTempPerm288: i32,
	iTempPerm289: i32,
	fTempPerm290: F32,
	fTempPerm291: F32,
	fTempPerm292: F32,
	fTempPerm293: F32,
	iTempPerm294: i32,
	iTempPerm295: i32,
	fTempPerm296: F32,
	fTempPerm297: F32,
	fTempPerm298: F32,
	fTempPerm299: F32,
	iTempPerm300: i32,
	iTempPerm301: i32,
	iTempPerm302: i32,
	iTempPerm303: i32,
	iTempPerm304: i32,
	iTempPerm305: i32,
	fTempPerm306: F32,
	iTempPerm307: i32,
	iTempPerm308: i32,
	iTempPerm309: i32,
	fTempPerm310: F32,
	fTempPerm311: F32,
	fTempPerm312: F32,
	fTempPerm313: F32,
	fTempPerm314: F32,
	fTempPerm315: F32,
	fTempPerm316: F32,
	fTempPerm317: F32,
	fTempPerm318: F32,
	iTempPerm319: i32,
	iTempPerm320: i32,
	fTempPerm321: F32,
	fTempPerm322: F32,
	fTempPerm323: F32,
	fTempPerm324: F32,
	fTempPerm325: F32,
	fConst24: F32,
	fConst25: F32,
	fTempPerm326: F32,
	fTempPerm327: F32,
	fTempPerm328: F32,
	fRec48: [F32;3],
	fTempPerm329: F32,
	fTempPerm330: F32,
	fTempPerm331: F32,
	fTempPerm332: F32,
	fTempPerm333: F32,
	fRec50: [F32;2],
	fRec49: [F32;2],
	fTempPerm334: F32,
	fTempPerm335: F32,
	fTempPerm336: F32,
	fTempPerm337: F32,
	fTempPerm338: F32,
	fTempPerm339: F32,
	fTempPerm340: F32,
	fTempPerm341: F32,
	fHslider18: F32,
	fVec41: [F32;2],
	fRec52: [F32;2],
	fRec53: [F32;2],
	fTempPerm342: F32,
	fTempPerm343: F32,
	fRec51: [F32;2],
	iTempPerm344: i32,
	fHslider19: F32,
	fVec42: [F32;2],
	fTempPerm345: F32,
	fRec54: [F32;2],
	fRec55: [F32;2],
	fTempPerm346: F32,
	fHslider20: F32,
	fVec43: [F32;2],
	fTempPerm347: F32,
	fRec56: [F32;2],
	fRec57: [F32;2],
	fTempPerm348: F32,
	fTempPerm349: F32,
	fTempPerm350: F32,
	fTempPerm351: F32,
	fTempPerm352: F32,
	fTempPerm353: F32,
	fTempPerm354: F32,
	fEntry14: F32,
	fEntry15: F32,
	fTempPerm355: F32,
	fTempPerm356: F32,
	fTempPerm357: F32,
	iTempPerm358: i32,
	fTempPerm359: F32,
	fTempPerm360: F32,
	fTempPerm361: F32,
	fTempPerm362: F32,
	fTempPerm363: F32,
	fTempPerm364: F32,
	fTempPerm365: F32,
	fTempPerm366: F32,
	fTempPerm367: F32,
	fTempPerm368: F32,
	fTempPerm369: F32,
	fTempPerm370: F32,
	fTempPerm371: F32,
	fTempPerm372: F32,
	fTempPerm373: F32,
	fVec44: [F32;2],
	fVec45: [F32;2],
	fTempPerm374: F32,
	fTempPerm375: F32,
	fTempPerm376: F32,
	fRec58: [F32;2],
	fTempPerm377: F32,
	fTempPerm378: F32,
	fTempPerm379: F32,
	fTempPerm380: F32,
	fTempPerm381: F32,
	fTempPerm382: F32,
	fVec46: [F32;2],
	fTempPerm383: F32,
	iTempPerm384: i32,
	iRec59: [i32;2],
	fVec47: [F32;2],
	fTempPerm385: F32,
	fTempPerm386: F32,
	fTempPerm387: F32,
	fTempPerm388: F32,
	fTempPerm389: F32,
	fTempPerm390: F32,
	fTempPerm391: F32,
	fTempPerm392: F32,
	fTempPerm393: F32,
	fTempPerm394: F32,
	fTempPerm395: F32,
	fTempPerm396: F32,
	fTempPerm397: F32,
	fVec48: [F32;2],
	fTempPerm398: F32,
	fTempPerm399: F32,
	fVec49: [F32;2],
	fTempPerm400: F32,
	fTempPerm401: F32,
	fTempPerm402: F32,
	fTempPerm403: F32,
	fTempPerm404: F32,
	fTempPerm405: F32,
	iTempPerm406: i32,
	fTempPerm407: F32,
	fTempPerm408: F32,
	fTempPerm409: F32,
	fVec50: [F32;2],
	fVec51: [F32;2],
	fTempPerm410: F32,
	fVec52: [F32;2],
	fTempPerm411: F32,
	iTempPerm412: i32,
	fTempPerm413: F32,
	fTempPerm414: F32,
	fTempPerm415: F32,
	fTempPerm416: F32,
	fTempPerm417: F32,
	fTempPerm418: F32,
	fTempPerm419: F32,
	iTempPerm420: i32,
	iTempPerm421: i32,
	iTempPerm422: i32,
	iTempPerm423: i32,
	iTempPerm424: i32,
	iTempPerm425: i32,
	fTempPerm426: F32,
	fTempPerm427: F32,
	fTempPerm428: F32,
	iTempPerm429: i32,
	iTempPerm430: i32,
	iTempPerm431: i32,
	iTempPerm432: i32,
	fTempPerm433: F32,
	fTempPerm434: F32,
	fTempPerm435: F32,
	fVec53: [F32;2],
	fTempPerm436: F32,
	fTempPerm437: F32,
	fTempPerm438: F32,
	fTempPerm439: F32,
	fTempPerm440: F32,
	fVec54: [F32;2],
	fTempPerm441: F32,
	fTempPerm442: F32,
	fTempPerm443: F32,
	fRec60: [F32;2],
	fVec55: [F32;2],
	fTempPerm444: F32,
	fTempPerm445: F32,
	fRec61: [F32;2],
	fVec56: [F32;2],
	fTempPerm446: F32,
	fVec57: [F32;2],
	fTempPerm447: F32,
	iTempPerm448: i32,
	fTempPerm449: F32,
	fTempPerm450: F32,
	fTempPerm451: F32,
	fTempPerm452: F32,
	fTempPerm453: F32,
	fRec62: [F32;2],
	fVec58: [F32;2],
	fTempPerm454: F32,
	fTempPerm455: F32,
	fTempPerm456: F32,
	fTempPerm457: F32,
	fTempPerm458: F32,
	fRec63: [F32;2],
	fVec59: [F32;2],
	fTempPerm459: F32,
	fTempPerm460: F32,
	fTempPerm461: F32,
	fTempPerm462: F32,
	iTempPerm463: i32,
	fTempPerm464: F32,
	fTempPerm465: F32,
	fTempPerm466: F32,
	fRec64: [F32;2],
	fTempPerm467: F32,
	iTempPerm468: i32,
	fTempPerm469: F32,
	iTempPerm470: i32,
	fTempPerm471: F32,
	fTempPerm472: F32,
	fTempPerm473: F32,
	fVec60: [F32;2],
	fTempPerm474: F32,
	fTempPerm475: F32,
	iTempPerm476: i32,
	fTempPerm477: F32,
	fTempPerm478: F32,
	fTempPerm479: F32,
	fRec65: [F32;2],
	fTempPerm480: F32,
	fTempPerm481: F32,
	fTempPerm482: F32,
	fTempPerm483: F32,
	fTempPerm484: F32,
	fTempPerm485: F32,
	fTempPerm486: F32,
	fTempPerm487: F32,
	fTempPerm488: F32,
	fVec61: [F32;2],
	fTempPerm489: F32,
	fTempPerm490: F32,
	fVec62: [F32;2],
	fTempPerm491: F32,
	fTempPerm492: F32,
	fTempPerm493: F32,
	fRec66: [F32;2],
	fTempPerm494: F32,
	fTempPerm495: F32,
	fTempPerm496: F32,
	fTempPerm497: F32,
	iTempPerm498: i32,
	fRec67: [F32;2],
	fRec68: [F32;2],
	fTempPerm499: F32,
	iTempPerm500: i32,
	fTempPerm501: F32,
	iTempPerm502: i32,
	fTempPerm503: F32,
	iTempPerm504: i32,
	fTempPerm505: F32,
	fTempPerm506: F32,
	fTempPerm507: F32,
	fVec63: [F32;2],
	fTempPerm508: F32,
	iTempPerm509: i32,
	fTempPerm510: F32,
	fTempPerm511: F32,
	fTempPerm512: F32,
	fRec69: [F32;2],
	fTempPerm513: F32,
	fTempPerm514: F32,
	fTempPerm515: F32,
	fTempPerm516: F32,
	fTempPerm517: F32,
	fTempPerm518: F32,
	fTempPerm519: F32,
	fTempPerm520: F32,
	fTempPerm521: F32,
	fTempPerm522: F32,
	fTempPerm523: F32,
	fVec64: [F32;2],
	fTempPerm524: F32,
	fTempPerm525: F32,
	fVec65: [F32;2],
	fTempPerm526: F32,
	iTempPerm527: i32,
	fTempPerm528: F32,
	fTempPerm529: F32,
	fTempPerm530: F32,
	fTempPerm531: F32,
	fTempPerm532: F32,
	fTempPerm533: F32,
	fTempPerm534: F32,
	fTempPerm535: F32,
	fTempPerm536: F32,
	fTempPerm537: F32,
	fTempPerm538: F32,
	fTempPerm539: F32,
	fTempPerm540: F32,
	fVec66: [F32;2],
	fTempPerm541: F32,
	fTempPerm542: F32,
	fVec67: [F32;2],
	fTempPerm543: F32,
	fVec68: [F32;2],
	fTempPerm544: F32,
	iTempPerm545: i32,
	fTempPerm546: F32,
	fVec69: [F32;2],
	fRec70: [F32;2],
	fTempPerm547: F32,
	fTempPerm548: F32,
	fTempPerm549: F32,
	iTempPerm550: i32,
	fTempPerm551: F32,
	iTempPerm552: i32,
	fTempPerm553: F32,
	fTempPerm554: F32,
	fTempPerm555: F32,
	fRec71: [F32;2],
	fTempPerm556: F32,
	fTempPerm557: F32,
	fRec72: [F32;2],
	fTempPerm558: F32,
	fRec73: [F32;2],
	fVec70: [F32;2],
	fTempPerm559: F32,
	fTempPerm560: F32,
	fHslider21: F32,
	fVec71: [F32;2],
	fTempPerm561: F32,
	fRec74: [F32;2],
	fRec75: [F32;2],
	fTempPerm562: F32,
	iTempPerm563: i32,
	fTempPerm564: F32,
	iTempPerm565: i32,
	fTempPerm566: F32,
	iTempPerm567: i32,
	iTempPerm568: i32,
	iTempPerm569: i32,
	iTempPerm570: i32,
	iTempPerm571: i32,
	iTempPerm572: i32,
	fTempPerm573: F32,
	iTempPerm574: i32,
	iTempPerm575: i32,
	iTempPerm576: i32,
	fEntry16: F32,
	fTempPerm577: F32,
	fTempPerm578: F32,
	fTempPerm579: F32,
	fTempPerm580: F32,
	fTempPerm581: F32,
	fHslider22: F32,
	fVec72: [F32;2],
	fTempPerm582: F32,
	fRec76: [F32;2],
	fRec77: [F32;2],
	fTempPerm583: F32,
	iTempPerm584: i32,
	iTempPerm585: i32,
	iTempPerm586: i32,
	fEntry17: F32,
	fEntry18: F32,
	fEntry19: F32,
	iTempPerm587: i32,
	fTempPerm588: F32,
	fTempPerm589: F32,
	fEntry20: F32,
	iTempPerm590: i32,
	fTempPerm591: F32,
	fTempPerm592: F32,
	fTempPerm593: F32,
	fHslider23: F32,
	fVec73: [F32;2],
	fTempPerm594: F32,
	fRec78: [F32;2],
	fRec79: [F32;2],
	fTempPerm595: F32,
	fTempPerm596: F32,
	fTempPerm597: F32,
	fTempPerm598: F32,
	iTempPerm599: i32,
	iTempPerm600: i32,
	iTempPerm601: i32,
	iTempPerm602: i32,
	iTempPerm603: i32,
	iTempPerm604: i32,
	fTempPerm605: F32,
	fTempPerm606: F32,
	iTempPerm607: i32,
	iTempPerm608: i32,
	iTempPerm609: i32,
	fTempPerm610: F32,
	fTempPerm611: F32,
	fTempPerm612: F32,
	fTempPerm613: F32,
	iTempPerm614: i32,
	iTempPerm615: i32,
	fTempPerm616: F32,
	fTempPerm617: F32,
	fTempPerm618: F32,
	fTempPerm619: F32,
	iTempPerm620: i32,
	iTempPerm621: i32,
	iTempPerm622: i32,
	iTempPerm623: i32,
	iTempPerm624: i32,
	iTempPerm625: i32,
	fTempPerm626: F32,
	iTempPerm627: i32,
	iTempPerm628: i32,
	iTempPerm629: i32,
	fTempPerm630: F32,
	fTempPerm631: F32,
	fTempPerm632: F32,
	fTempPerm633: F32,
	fTempPerm634: F32,
	fTempPerm635: F32,
	fTempPerm636: F32,
	fTempPerm637: F32,
	fTempPerm638: F32,
	iTempPerm639: i32,
	iTempPerm640: i32,
	fTempPerm641: F32,
	fTempPerm642: F32,
	fTempPerm643: F32,
	fTempPerm644: F32,
	fTempPerm645: F32,
	fTempPerm646: F32,
	fTempPerm647: F32,
	fTempPerm648: F32,
	fRec80: [F32;3],
	fTempPerm649: F32,
	fTempPerm650: F32,
	fTempPerm651: F32,
	fTempPerm652: F32,
	fTempPerm653: F32,
	fRec82: [F32;2],
	fRec81: [F32;2],
	fTempPerm654: F32,
	fTempPerm655: F32,
	fTempPerm656: F32,
	fTempPerm657: F32,
	fTempPerm658: F32,
	fTempPerm659: F32,
	fTempPerm660: F32,
	fTempPerm661: F32,
	fHslider24: F32,
	fVec74: [F32;2],
	fRec83: [F32;2],
	fRec84: [F32;2],
	fHslider25: F32,
	fVec75: [F32;2],
	fRec85: [F32;2],
	fRec86: [F32;2],
	fEntry21: F32,
	fHslider26: F32,
	fCheckbox1: F32,
	fHslider27: F32,
	fEntry22: F32,
	fCheckbox2: F32,
	fHslider28: F32,
	fEntry23: F32,
	fHslider29: F32,
	fCheckbox3: F32,
	fEntry24: F32,
	fCheckbox4: F32,
	fHslider30: F32,
	fEntry25: F32,
	fCheckbox5: F32,
	fEntry26: F32,
	fCheckbox6: F32,
	fHslider31: F32,
	fEntry27: F32,
	fCheckbox7: F32,
	fEntry28: F32,
	fCheckbox8: F32,
	fHslider32: F32,
	fEntry29: F32,
	fHslider33: F32,
	fHslider34: F32,
	fVslider0: F32,
	fVslider1: F32,
	fVslider2: F32,
	fVslider3: F32,
	fHslider35: F32,
	fHslider36: F32,
	fVslider4: F32,
	fVslider5: F32,
	fVslider6: F32,
	fVslider7: F32,
	fHslider37: F32,
	fHslider38: F32,
	fVslider8: F32,
	fVslider9: F32,
	fVslider10: F32,
	fVslider11: F32,
	fCheckbox9: F32,
	fEntry30: F32,
	fHslider39: F32,
	fHslider40: F32,
	fHslider41: F32,
	fHslider42: F32,
	fEntry31: F32,
	fHslider43: F32,
	fHslider44: F32,
	fHslider45: F32,
	fHslider46: F32,
	fHslider47: F32,
	fEntry32: F32,
	fCheckbox10: F32,
	fHslider48: F32,
	fHslider49: F32,
	fHslider50: F32,
	fHslider51: F32,
	fEntry33: F32,
	fCheckbox11: F32,
	fHslider52: F32,
	fHslider53: F32,
	fHslider54: F32,
	fHslider55: F32,
	fEntry34: F32,
	fCheckbox12: F32,
	fEntry35: F32,
	fHslider56: F32,
	fHslider57: F32,
	fHslider58: F32,
	fHslider59: F32,
	fEntry36: F32,
	fCheckbox13: F32,
	fHslider60: F32,
	fHslider61: F32,
	fHslider62: F32,
	fHslider63: F32,
	fEntry37: F32,
	fCheckbox14: F32,
	fHslider64: F32,
	fHslider65: F32,
	fHslider66: F32,
	fHslider67: F32,
	fEntry38: F32,
	fCheckbox15: F32,
	fEntry39: F32,
	fHslider68: F32,
	fHslider69: F32,
	fHslider70: F32,
	fHslider71: F32,
	fHslider72: F32,
	fHslider73: F32,
	fHslider74: F32,
	fEntry40: F32,
	fEntry41: F32,
	fHslider75: F32,
	fHslider76: F32,
	fHslider77: F32,
	fHslider78: F32,
	fHslider79: F32,
	fHslider80: F32,
	fHslider81: F32,
	fEntry42: F32,
	fEntry43: F32,
	fHslider82: F32,
	fHslider83: F32,
	fHslider84: F32,
	fHslider85: F32,
	fHslider86: F32,
	fHslider87: F32,
	fHslider88: F32,
	fHslider89: F32,
	fCheckbox16: F32,
	fEntry44: F32,
	fEntry45: F32,
	fHslider90: F32,
	fHslider91: F32,
	fHslider92: F32,
	fHslider93: F32,
	fHslider94: F32,
	fHslider95: F32,
	fHslider96: F32,
	fHslider97: F32,
	fCheckbox17: F32,
	fEntry46: F32,
	fEntry47: F32,
}

impl FaustDsp for OscDsp {
	type T = F32;
		
	fn new() -> OscDsp { 
		OscDsp {
			fHslider0: 0.0,
			fEntry0: 0.0,
			fEntry1: 0.0,
			fEntry2: 0.0,
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			fHslider1: 0.0,
			fVec2: [0.0;2],
			fHslider2: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fHslider3: 0.0,
			fVec3: [0.0;2],
			fRec3: [0.0;2],
			fRec4: [0.0;2],
			fEntry3: 0.0,
			fEntry4: 0.0,
			iTempPerm0: 0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fTempPerm1: 0.0,
			fRec6: [0.0;2],
			fHslider4: 0.0,
			fConst2: 0.0,
			fTempPerm2: 0.0,
			fRec7: [0.0;2],
			iRec9: [0;2],
			iTempPerm3: 0,
			fConst3: 0.0,
			fConst5: 0.0,
			fRec8: [0.0;2],
			fConst6: 0.0,
			fHslider5: 0.0,
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
			fHslider6: 0.0,
			fRec15: [0.0;2],
			fEntry5: 0.0,
			fHslider7: 0.0,
			fVec7: [0.0;2],
			fRec16: [0.0;2],
			fRec17: [0.0;2],
			fHslider8: 0.0,
			fVec8: [0.0;2],
			fRec18: [0.0;2],
			fRec19: [0.0;2],
			fHslider9: 0.0,
			fHslider10: 0.0,
			fRec20: [0.0;2],
			fTempPerm7: 0.0,
			fTempPerm8: 0.0,
			fRec21: [0.0;2],
			fHslider11: 0.0,
			fHslider12: 0.0,
			fCheckbox0: 0.0,
			fEntry6: 0.0,
			fTempPerm9: 0.0,
			fRec0: [0.0;2],
			iTempPerm10: 0,
			fHslider13: 0.0,
			fVec9: [0.0;2],
			fTempPerm11: 0.0,
			fRec22: [0.0;2],
			fRec23: [0.0;2],
			fTempPerm12: 0.0,
			fHslider14: 0.0,
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
			fHslider15: 0.0,
			fVec38: [0.0;2],
			fTempPerm241: 0.0,
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fTempPerm242: 0.0,
			iTempPerm243: 0,
			fTempPerm244: 0.0,
			iTempPerm245: 0,
			fConst16: 0.0,
			fConst17: 0.0,
			fTempPerm246: 0.0,
			fConst18: 0.0,
			fConst19: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			iTempPerm247: 0,
			iTempPerm248: 0,
			iTempPerm249: 0,
			iTempPerm250: 0,
			iTempPerm251: 0,
			iTempPerm252: 0,
			fTempPerm253: 0.0,
			iTempPerm254: 0,
			iTempPerm255: 0,
			iTempPerm256: 0,
			fEntry9: 0.0,
			fTempPerm257: 0.0,
			fTempPerm258: 0.0,
			fTempPerm259: 0.0,
			fTempPerm260: 0.0,
			fTempPerm261: 0.0,
			fHslider16: 0.0,
			fVec39: [0.0;2],
			fTempPerm262: 0.0,
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			fTempPerm263: 0.0,
			iTempPerm264: 0,
			iTempPerm265: 0,
			iTempPerm266: 0,
			fEntry10: 0.0,
			fEntry11: 0.0,
			fEntry12: 0.0,
			iTempPerm267: 0,
			fTempPerm268: 0.0,
			fTempPerm269: 0.0,
			fEntry13: 0.0,
			iTempPerm270: 0,
			fTempPerm271: 0.0,
			fTempPerm272: 0.0,
			fTempPerm273: 0.0,
			fHslider17: 0.0,
			fVec40: [0.0;2],
			fTempPerm274: 0.0,
			fRec46: [0.0;2],
			fRec47: [0.0;2],
			fTempPerm275: 0.0,
			fTempPerm276: 0.0,
			fTempPerm277: 0.0,
			fTempPerm278: 0.0,
			iTempPerm279: 0,
			iTempPerm280: 0,
			iTempPerm281: 0,
			iTempPerm282: 0,
			iTempPerm283: 0,
			iTempPerm284: 0,
			fTempPerm285: 0.0,
			fTempPerm286: 0.0,
			iTempPerm287: 0,
			iTempPerm288: 0,
			iTempPerm289: 0,
			fTempPerm290: 0.0,
			fTempPerm291: 0.0,
			fTempPerm292: 0.0,
			fTempPerm293: 0.0,
			iTempPerm294: 0,
			iTempPerm295: 0,
			fTempPerm296: 0.0,
			fTempPerm297: 0.0,
			fTempPerm298: 0.0,
			fTempPerm299: 0.0,
			iTempPerm300: 0,
			iTempPerm301: 0,
			iTempPerm302: 0,
			iTempPerm303: 0,
			iTempPerm304: 0,
			iTempPerm305: 0,
			fTempPerm306: 0.0,
			iTempPerm307: 0,
			iTempPerm308: 0,
			iTempPerm309: 0,
			fTempPerm310: 0.0,
			fTempPerm311: 0.0,
			fTempPerm312: 0.0,
			fTempPerm313: 0.0,
			fTempPerm314: 0.0,
			fTempPerm315: 0.0,
			fTempPerm316: 0.0,
			fTempPerm317: 0.0,
			fTempPerm318: 0.0,
			iTempPerm319: 0,
			iTempPerm320: 0,
			fTempPerm321: 0.0,
			fTempPerm322: 0.0,
			fTempPerm323: 0.0,
			fTempPerm324: 0.0,
			fTempPerm325: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fTempPerm326: 0.0,
			fTempPerm327: 0.0,
			fTempPerm328: 0.0,
			fRec48: [0.0;3],
			fTempPerm329: 0.0,
			fTempPerm330: 0.0,
			fTempPerm331: 0.0,
			fTempPerm332: 0.0,
			fTempPerm333: 0.0,
			fRec50: [0.0;2],
			fRec49: [0.0;2],
			fTempPerm334: 0.0,
			fTempPerm335: 0.0,
			fTempPerm336: 0.0,
			fTempPerm337: 0.0,
			fTempPerm338: 0.0,
			fTempPerm339: 0.0,
			fTempPerm340: 0.0,
			fTempPerm341: 0.0,
			fHslider18: 0.0,
			fVec41: [0.0;2],
			fRec52: [0.0;2],
			fRec53: [0.0;2],
			fTempPerm342: 0.0,
			fTempPerm343: 0.0,
			fRec51: [0.0;2],
			iTempPerm344: 0,
			fHslider19: 0.0,
			fVec42: [0.0;2],
			fTempPerm345: 0.0,
			fRec54: [0.0;2],
			fRec55: [0.0;2],
			fTempPerm346: 0.0,
			fHslider20: 0.0,
			fVec43: [0.0;2],
			fTempPerm347: 0.0,
			fRec56: [0.0;2],
			fRec57: [0.0;2],
			fTempPerm348: 0.0,
			fTempPerm349: 0.0,
			fTempPerm350: 0.0,
			fTempPerm351: 0.0,
			fTempPerm352: 0.0,
			fTempPerm353: 0.0,
			fTempPerm354: 0.0,
			fEntry14: 0.0,
			fEntry15: 0.0,
			fTempPerm355: 0.0,
			fTempPerm356: 0.0,
			fTempPerm357: 0.0,
			iTempPerm358: 0,
			fTempPerm359: 0.0,
			fTempPerm360: 0.0,
			fTempPerm361: 0.0,
			fTempPerm362: 0.0,
			fTempPerm363: 0.0,
			fTempPerm364: 0.0,
			fTempPerm365: 0.0,
			fTempPerm366: 0.0,
			fTempPerm367: 0.0,
			fTempPerm368: 0.0,
			fTempPerm369: 0.0,
			fTempPerm370: 0.0,
			fTempPerm371: 0.0,
			fTempPerm372: 0.0,
			fTempPerm373: 0.0,
			fVec44: [0.0;2],
			fVec45: [0.0;2],
			fTempPerm374: 0.0,
			fTempPerm375: 0.0,
			fTempPerm376: 0.0,
			fRec58: [0.0;2],
			fTempPerm377: 0.0,
			fTempPerm378: 0.0,
			fTempPerm379: 0.0,
			fTempPerm380: 0.0,
			fTempPerm381: 0.0,
			fTempPerm382: 0.0,
			fVec46: [0.0;2],
			fTempPerm383: 0.0,
			iTempPerm384: 0,
			iRec59: [0;2],
			fVec47: [0.0;2],
			fTempPerm385: 0.0,
			fTempPerm386: 0.0,
			fTempPerm387: 0.0,
			fTempPerm388: 0.0,
			fTempPerm389: 0.0,
			fTempPerm390: 0.0,
			fTempPerm391: 0.0,
			fTempPerm392: 0.0,
			fTempPerm393: 0.0,
			fTempPerm394: 0.0,
			fTempPerm395: 0.0,
			fTempPerm396: 0.0,
			fTempPerm397: 0.0,
			fVec48: [0.0;2],
			fTempPerm398: 0.0,
			fTempPerm399: 0.0,
			fVec49: [0.0;2],
			fTempPerm400: 0.0,
			fTempPerm401: 0.0,
			fTempPerm402: 0.0,
			fTempPerm403: 0.0,
			fTempPerm404: 0.0,
			fTempPerm405: 0.0,
			iTempPerm406: 0,
			fTempPerm407: 0.0,
			fTempPerm408: 0.0,
			fTempPerm409: 0.0,
			fVec50: [0.0;2],
			fVec51: [0.0;2],
			fTempPerm410: 0.0,
			fVec52: [0.0;2],
			fTempPerm411: 0.0,
			iTempPerm412: 0,
			fTempPerm413: 0.0,
			fTempPerm414: 0.0,
			fTempPerm415: 0.0,
			fTempPerm416: 0.0,
			fTempPerm417: 0.0,
			fTempPerm418: 0.0,
			fTempPerm419: 0.0,
			iTempPerm420: 0,
			iTempPerm421: 0,
			iTempPerm422: 0,
			iTempPerm423: 0,
			iTempPerm424: 0,
			iTempPerm425: 0,
			fTempPerm426: 0.0,
			fTempPerm427: 0.0,
			fTempPerm428: 0.0,
			iTempPerm429: 0,
			iTempPerm430: 0,
			iTempPerm431: 0,
			iTempPerm432: 0,
			fTempPerm433: 0.0,
			fTempPerm434: 0.0,
			fTempPerm435: 0.0,
			fVec53: [0.0;2],
			fTempPerm436: 0.0,
			fTempPerm437: 0.0,
			fTempPerm438: 0.0,
			fTempPerm439: 0.0,
			fTempPerm440: 0.0,
			fVec54: [0.0;2],
			fTempPerm441: 0.0,
			fTempPerm442: 0.0,
			fTempPerm443: 0.0,
			fRec60: [0.0;2],
			fVec55: [0.0;2],
			fTempPerm444: 0.0,
			fTempPerm445: 0.0,
			fRec61: [0.0;2],
			fVec56: [0.0;2],
			fTempPerm446: 0.0,
			fVec57: [0.0;2],
			fTempPerm447: 0.0,
			iTempPerm448: 0,
			fTempPerm449: 0.0,
			fTempPerm450: 0.0,
			fTempPerm451: 0.0,
			fTempPerm452: 0.0,
			fTempPerm453: 0.0,
			fRec62: [0.0;2],
			fVec58: [0.0;2],
			fTempPerm454: 0.0,
			fTempPerm455: 0.0,
			fTempPerm456: 0.0,
			fTempPerm457: 0.0,
			fTempPerm458: 0.0,
			fRec63: [0.0;2],
			fVec59: [0.0;2],
			fTempPerm459: 0.0,
			fTempPerm460: 0.0,
			fTempPerm461: 0.0,
			fTempPerm462: 0.0,
			iTempPerm463: 0,
			fTempPerm464: 0.0,
			fTempPerm465: 0.0,
			fTempPerm466: 0.0,
			fRec64: [0.0;2],
			fTempPerm467: 0.0,
			iTempPerm468: 0,
			fTempPerm469: 0.0,
			iTempPerm470: 0,
			fTempPerm471: 0.0,
			fTempPerm472: 0.0,
			fTempPerm473: 0.0,
			fVec60: [0.0;2],
			fTempPerm474: 0.0,
			fTempPerm475: 0.0,
			iTempPerm476: 0,
			fTempPerm477: 0.0,
			fTempPerm478: 0.0,
			fTempPerm479: 0.0,
			fRec65: [0.0;2],
			fTempPerm480: 0.0,
			fTempPerm481: 0.0,
			fTempPerm482: 0.0,
			fTempPerm483: 0.0,
			fTempPerm484: 0.0,
			fTempPerm485: 0.0,
			fTempPerm486: 0.0,
			fTempPerm487: 0.0,
			fTempPerm488: 0.0,
			fVec61: [0.0;2],
			fTempPerm489: 0.0,
			fTempPerm490: 0.0,
			fVec62: [0.0;2],
			fTempPerm491: 0.0,
			fTempPerm492: 0.0,
			fTempPerm493: 0.0,
			fRec66: [0.0;2],
			fTempPerm494: 0.0,
			fTempPerm495: 0.0,
			fTempPerm496: 0.0,
			fTempPerm497: 0.0,
			iTempPerm498: 0,
			fRec67: [0.0;2],
			fRec68: [0.0;2],
			fTempPerm499: 0.0,
			iTempPerm500: 0,
			fTempPerm501: 0.0,
			iTempPerm502: 0,
			fTempPerm503: 0.0,
			iTempPerm504: 0,
			fTempPerm505: 0.0,
			fTempPerm506: 0.0,
			fTempPerm507: 0.0,
			fVec63: [0.0;2],
			fTempPerm508: 0.0,
			iTempPerm509: 0,
			fTempPerm510: 0.0,
			fTempPerm511: 0.0,
			fTempPerm512: 0.0,
			fRec69: [0.0;2],
			fTempPerm513: 0.0,
			fTempPerm514: 0.0,
			fTempPerm515: 0.0,
			fTempPerm516: 0.0,
			fTempPerm517: 0.0,
			fTempPerm518: 0.0,
			fTempPerm519: 0.0,
			fTempPerm520: 0.0,
			fTempPerm521: 0.0,
			fTempPerm522: 0.0,
			fTempPerm523: 0.0,
			fVec64: [0.0;2],
			fTempPerm524: 0.0,
			fTempPerm525: 0.0,
			fVec65: [0.0;2],
			fTempPerm526: 0.0,
			iTempPerm527: 0,
			fTempPerm528: 0.0,
			fTempPerm529: 0.0,
			fTempPerm530: 0.0,
			fTempPerm531: 0.0,
			fTempPerm532: 0.0,
			fTempPerm533: 0.0,
			fTempPerm534: 0.0,
			fTempPerm535: 0.0,
			fTempPerm536: 0.0,
			fTempPerm537: 0.0,
			fTempPerm538: 0.0,
			fTempPerm539: 0.0,
			fTempPerm540: 0.0,
			fVec66: [0.0;2],
			fTempPerm541: 0.0,
			fTempPerm542: 0.0,
			fVec67: [0.0;2],
			fTempPerm543: 0.0,
			fVec68: [0.0;2],
			fTempPerm544: 0.0,
			iTempPerm545: 0,
			fTempPerm546: 0.0,
			fVec69: [0.0;2],
			fRec70: [0.0;2],
			fTempPerm547: 0.0,
			fTempPerm548: 0.0,
			fTempPerm549: 0.0,
			iTempPerm550: 0,
			fTempPerm551: 0.0,
			iTempPerm552: 0,
			fTempPerm553: 0.0,
			fTempPerm554: 0.0,
			fTempPerm555: 0.0,
			fRec71: [0.0;2],
			fTempPerm556: 0.0,
			fTempPerm557: 0.0,
			fRec72: [0.0;2],
			fTempPerm558: 0.0,
			fRec73: [0.0;2],
			fVec70: [0.0;2],
			fTempPerm559: 0.0,
			fTempPerm560: 0.0,
			fHslider21: 0.0,
			fVec71: [0.0;2],
			fTempPerm561: 0.0,
			fRec74: [0.0;2],
			fRec75: [0.0;2],
			fTempPerm562: 0.0,
			iTempPerm563: 0,
			fTempPerm564: 0.0,
			iTempPerm565: 0,
			fTempPerm566: 0.0,
			iTempPerm567: 0,
			iTempPerm568: 0,
			iTempPerm569: 0,
			iTempPerm570: 0,
			iTempPerm571: 0,
			iTempPerm572: 0,
			fTempPerm573: 0.0,
			iTempPerm574: 0,
			iTempPerm575: 0,
			iTempPerm576: 0,
			fEntry16: 0.0,
			fTempPerm577: 0.0,
			fTempPerm578: 0.0,
			fTempPerm579: 0.0,
			fTempPerm580: 0.0,
			fTempPerm581: 0.0,
			fHslider22: 0.0,
			fVec72: [0.0;2],
			fTempPerm582: 0.0,
			fRec76: [0.0;2],
			fRec77: [0.0;2],
			fTempPerm583: 0.0,
			iTempPerm584: 0,
			iTempPerm585: 0,
			iTempPerm586: 0,
			fEntry17: 0.0,
			fEntry18: 0.0,
			fEntry19: 0.0,
			iTempPerm587: 0,
			fTempPerm588: 0.0,
			fTempPerm589: 0.0,
			fEntry20: 0.0,
			iTempPerm590: 0,
			fTempPerm591: 0.0,
			fTempPerm592: 0.0,
			fTempPerm593: 0.0,
			fHslider23: 0.0,
			fVec73: [0.0;2],
			fTempPerm594: 0.0,
			fRec78: [0.0;2],
			fRec79: [0.0;2],
			fTempPerm595: 0.0,
			fTempPerm596: 0.0,
			fTempPerm597: 0.0,
			fTempPerm598: 0.0,
			iTempPerm599: 0,
			iTempPerm600: 0,
			iTempPerm601: 0,
			iTempPerm602: 0,
			iTempPerm603: 0,
			iTempPerm604: 0,
			fTempPerm605: 0.0,
			fTempPerm606: 0.0,
			iTempPerm607: 0,
			iTempPerm608: 0,
			iTempPerm609: 0,
			fTempPerm610: 0.0,
			fTempPerm611: 0.0,
			fTempPerm612: 0.0,
			fTempPerm613: 0.0,
			iTempPerm614: 0,
			iTempPerm615: 0,
			fTempPerm616: 0.0,
			fTempPerm617: 0.0,
			fTempPerm618: 0.0,
			fTempPerm619: 0.0,
			iTempPerm620: 0,
			iTempPerm621: 0,
			iTempPerm622: 0,
			iTempPerm623: 0,
			iTempPerm624: 0,
			iTempPerm625: 0,
			fTempPerm626: 0.0,
			iTempPerm627: 0,
			iTempPerm628: 0,
			iTempPerm629: 0,
			fTempPerm630: 0.0,
			fTempPerm631: 0.0,
			fTempPerm632: 0.0,
			fTempPerm633: 0.0,
			fTempPerm634: 0.0,
			fTempPerm635: 0.0,
			fTempPerm636: 0.0,
			fTempPerm637: 0.0,
			fTempPerm638: 0.0,
			iTempPerm639: 0,
			iTempPerm640: 0,
			fTempPerm641: 0.0,
			fTempPerm642: 0.0,
			fTempPerm643: 0.0,
			fTempPerm644: 0.0,
			fTempPerm645: 0.0,
			fTempPerm646: 0.0,
			fTempPerm647: 0.0,
			fTempPerm648: 0.0,
			fRec80: [0.0;3],
			fTempPerm649: 0.0,
			fTempPerm650: 0.0,
			fTempPerm651: 0.0,
			fTempPerm652: 0.0,
			fTempPerm653: 0.0,
			fRec82: [0.0;2],
			fRec81: [0.0;2],
			fTempPerm654: 0.0,
			fTempPerm655: 0.0,
			fTempPerm656: 0.0,
			fTempPerm657: 0.0,
			fTempPerm658: 0.0,
			fTempPerm659: 0.0,
			fTempPerm660: 0.0,
			fTempPerm661: 0.0,
			fHslider24: 0.0,
			fVec74: [0.0;2],
			fRec83: [0.0;2],
			fRec84: [0.0;2],
			fHslider25: 0.0,
			fVec75: [0.0;2],
			fRec85: [0.0;2],
			fRec86: [0.0;2],
			fEntry21: 0.0,
			fHslider26: 0.0,
			fCheckbox1: 0.0,
			fHslider27: 0.0,
			fEntry22: 0.0,
			fCheckbox2: 0.0,
			fHslider28: 0.0,
			fEntry23: 0.0,
			fHslider29: 0.0,
			fCheckbox3: 0.0,
			fEntry24: 0.0,
			fCheckbox4: 0.0,
			fHslider30: 0.0,
			fEntry25: 0.0,
			fCheckbox5: 0.0,
			fEntry26: 0.0,
			fCheckbox6: 0.0,
			fHslider31: 0.0,
			fEntry27: 0.0,
			fCheckbox7: 0.0,
			fEntry28: 0.0,
			fCheckbox8: 0.0,
			fHslider32: 0.0,
			fEntry29: 0.0,
			fHslider33: 0.0,
			fHslider34: 0.0,
			fVslider0: 0.0,
			fVslider1: 0.0,
			fVslider2: 0.0,
			fVslider3: 0.0,
			fHslider35: 0.0,
			fHslider36: 0.0,
			fVslider4: 0.0,
			fVslider5: 0.0,
			fVslider6: 0.0,
			fVslider7: 0.0,
			fHslider37: 0.0,
			fHslider38: 0.0,
			fVslider8: 0.0,
			fVslider9: 0.0,
			fVslider10: 0.0,
			fVslider11: 0.0,
			fCheckbox9: 0.0,
			fEntry30: 0.0,
			fHslider39: 0.0,
			fHslider40: 0.0,
			fHslider41: 0.0,
			fHslider42: 0.0,
			fEntry31: 0.0,
			fHslider43: 0.0,
			fHslider44: 0.0,
			fHslider45: 0.0,
			fHslider46: 0.0,
			fHslider47: 0.0,
			fEntry32: 0.0,
			fCheckbox10: 0.0,
			fHslider48: 0.0,
			fHslider49: 0.0,
			fHslider50: 0.0,
			fHslider51: 0.0,
			fEntry33: 0.0,
			fCheckbox11: 0.0,
			fHslider52: 0.0,
			fHslider53: 0.0,
			fHslider54: 0.0,
			fHslider55: 0.0,
			fEntry34: 0.0,
			fCheckbox12: 0.0,
			fEntry35: 0.0,
			fHslider56: 0.0,
			fHslider57: 0.0,
			fHslider58: 0.0,
			fHslider59: 0.0,
			fEntry36: 0.0,
			fCheckbox13: 0.0,
			fHslider60: 0.0,
			fHslider61: 0.0,
			fHslider62: 0.0,
			fHslider63: 0.0,
			fEntry37: 0.0,
			fCheckbox14: 0.0,
			fHslider64: 0.0,
			fHslider65: 0.0,
			fHslider66: 0.0,
			fHslider67: 0.0,
			fEntry38: 0.0,
			fCheckbox15: 0.0,
			fEntry39: 0.0,
			fHslider68: 0.0,
			fHslider69: 0.0,
			fHslider70: 0.0,
			fHslider71: 0.0,
			fHslider72: 0.0,
			fHslider73: 0.0,
			fHslider74: 0.0,
			fEntry40: 0.0,
			fEntry41: 0.0,
			fHslider75: 0.0,
			fHslider76: 0.0,
			fHslider77: 0.0,
			fHslider78: 0.0,
			fHslider79: 0.0,
			fHslider80: 0.0,
			fHslider81: 0.0,
			fEntry42: 0.0,
			fEntry43: 0.0,
			fHslider82: 0.0,
			fHslider83: 0.0,
			fHslider84: 0.0,
			fHslider85: 0.0,
			fHslider86: 0.0,
			fHslider87: 0.0,
			fHslider88: 0.0,
			fHslider89: 0.0,
			fCheckbox16: 0.0,
			fEntry44: 0.0,
			fEntry45: 0.0,
			fHslider90: 0.0,
			fHslider91: 0.0,
			fHslider92: 0.0,
			fHslider93: 0.0,
			fHslider94: 0.0,
			fHslider95: 0.0,
			fHslider96: 0.0,
			fHslider97: 0.0,
			fCheckbox17: 0.0,
			fEntry46: 0.0,
			fEntry47: 0.0,
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
		self.fHslider0 = 0.0;
		self.fEntry0 = 0.0;
		self.fEntry1 = 0.0;
		self.fEntry2 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider1 = 0.0;
		self.fHslider2 = 64.0;
		self.fHslider3 = 0.0;
		self.fEntry3 = 0.0;
		self.fEntry4 = 1.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 1.0;
		self.fHslider6 = 0.0;
		self.fEntry5 = 0.0;
		self.fHslider7 = 0.0;
		self.fHslider8 = 1.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 1.1e+02;
		self.fHslider11 = 3.0;
		self.fHslider12 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fEntry6 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 0.5;
		self.fEntry7 = 2.0;
		self.fEntry8 = 2.0;
		self.fHslider15 = 0.0;
		self.fEntry9 = 36.0;
		self.fHslider16 = 0.0;
		self.fEntry10 = 12.0;
		self.fEntry11 = 2e+01;
		self.fEntry12 = 28.0;
		self.fEntry13 = 4.0;
		self.fHslider17 = 0.5;
		self.fHslider18 = 0.0;
		self.fHslider19 = 0.0;
		self.fHslider20 = 0.5;
		self.fEntry14 = 2.0;
		self.fEntry15 = 2.0;
		self.fHslider21 = 0.0;
		self.fEntry16 = 36.0;
		self.fHslider22 = 0.0;
		self.fEntry17 = 12.0;
		self.fEntry18 = 2e+01;
		self.fEntry19 = 28.0;
		self.fEntry20 = 4.0;
		self.fHslider23 = 0.5;
		self.fHslider24 = 64.0;
		self.fHslider25 = 64.0;
		self.fEntry21 = 8.0;
		self.fHslider26 = 0.21;
		self.fCheckbox1 = 0.0;
		self.fHslider27 = 0.5;
		self.fEntry22 = 8.0;
		self.fCheckbox2 = 0.0;
		self.fHslider28 = 0.5;
		self.fEntry23 = 1.0;
		self.fHslider29 = 5e+01;
		self.fCheckbox3 = 0.0;
		self.fEntry24 = 11.0;
		self.fCheckbox4 = 0.0;
		self.fHslider30 = 5.0;
		self.fEntry25 = 0.0;
		self.fCheckbox5 = 0.0;
		self.fEntry26 = 11.0;
		self.fCheckbox6 = 0.0;
		self.fHslider31 = 2.0;
		self.fEntry27 = 0.0;
		self.fCheckbox7 = 0.0;
		self.fEntry28 = 11.0;
		self.fCheckbox8 = 0.0;
		self.fHslider32 = 0.5;
		self.fEntry29 = 0.0;
		self.fHslider33 = -0.6;
		self.fHslider34 = 0.6;
		self.fVslider0 = 0.5;
		self.fVslider1 = 0.0;
		self.fVslider2 = 0.4;
		self.fVslider3 = 0.005;
		self.fHslider35 = -0.6;
		self.fHslider36 = 0.6;
		self.fVslider4 = 0.5;
		self.fVslider5 = 0.3;
		self.fVslider6 = 0.4;
		self.fVslider7 = 0.005;
		self.fHslider37 = -0.6;
		self.fHslider38 = 0.6;
		self.fVslider8 = 0.5;
		self.fVslider9 = 0.6;
		self.fVslider10 = 0.4;
		self.fVslider11 = 0.005;
		self.fCheckbox9 = 0.0;
		self.fEntry30 = 0.0;
		self.fHslider39 = 0.0;
		self.fHslider40 = 1.0;
		self.fHslider41 = 0.0;
		self.fHslider42 = 0.0;
		self.fEntry31 = 0.0;
		self.fHslider43 = 1e+02;
		self.fHslider44 = 0.0;
		self.fHslider45 = 0.15;
		self.fHslider46 = 0.0;
		self.fHslider47 = 0.6666667;
		self.fEntry32 = 1.0;
		self.fCheckbox10 = 0.0;
		self.fHslider48 = 0.0;
		self.fHslider49 = 0.15;
		self.fHslider50 = 0.0;
		self.fHslider51 = 0.33333334;
		self.fEntry33 = 4.0;
		self.fCheckbox11 = 0.0;
		self.fHslider52 = 0.0;
		self.fHslider53 = 0.15;
		self.fHslider54 = 0.0;
		self.fHslider55 = 0.0;
		self.fEntry34 = 0.0;
		self.fCheckbox12 = 0.0;
		self.fEntry35 = 3.0;
		self.fHslider56 = 0.0;
		self.fHslider57 = 0.15;
		self.fHslider58 = 0.0;
		self.fHslider59 = 0.6666667;
		self.fEntry36 = 1.0;
		self.fCheckbox13 = 0.0;
		self.fHslider60 = 0.0;
		self.fHslider61 = 0.15;
		self.fHslider62 = 0.0;
		self.fHslider63 = 0.33333334;
		self.fEntry37 = 4.0;
		self.fCheckbox14 = 0.0;
		self.fHslider64 = 0.0;
		self.fHslider65 = 0.15;
		self.fHslider66 = 0.0;
		self.fHslider67 = 0.0;
		self.fEntry38 = 0.0;
		self.fCheckbox15 = 0.0;
		self.fEntry39 = 3.0;
		self.fHslider68 = 0.0;
		self.fHslider69 = 0.0;
		self.fHslider70 = 0.5;
		self.fHslider71 = 1e+02;
		self.fHslider72 = 1e+02;
		self.fHslider73 = 0.0;
		self.fHslider74 = 0.0;
		self.fEntry40 = 31.0;
		self.fEntry41 = 52.0;
		self.fHslider75 = 0.0;
		self.fHslider76 = 0.0;
		self.fHslider77 = 0.5;
		self.fHslider78 = 1e+02;
		self.fHslider79 = 1e+02;
		self.fHslider80 = 0.0;
		self.fHslider81 = 0.0;
		self.fEntry42 = 39.0;
		self.fEntry43 = 34.0;
		self.fHslider82 = 0.0;
		self.fHslider83 = 0.3;
		self.fHslider84 = 0.0;
		self.fHslider85 = 0.0;
		self.fHslider86 = 0.0;
		self.fHslider87 = 0.0;
		self.fHslider88 = 0.0;
		self.fHslider89 = 0.0;
		self.fCheckbox16 = 0.0;
		self.fEntry44 = 6821.0;
		self.fEntry45 = 3.0;
		self.fHslider90 = 0.0;
		self.fHslider91 = 0.3;
		self.fHslider92 = 0.0;
		self.fHslider93 = 0.0;
		self.fHslider94 = 0.0;
		self.fHslider95 = 0.0;
		self.fHslider96 = 0.0;
		self.fHslider97 = 0.0;
		self.fCheckbox17 = 0.0;
		self.fEntry46 = 4821.0;
		self.fEntry47 = 0.0;
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
		self.fTempPerm244 = 0.0;
		self.iTempPerm245 = 0;
		self.fTempPerm246 = 0.0;
		self.iTempPerm247 = 0;
		self.iTempPerm248 = 0;
		self.iTempPerm249 = 0;
		self.iTempPerm250 = 0;
		self.iTempPerm251 = 0;
		self.iTempPerm252 = 0;
		self.fTempPerm253 = 0.0;
		self.iTempPerm254 = 0;
		self.iTempPerm255 = 0;
		self.iTempPerm256 = 0;
		self.fTempPerm257 = 0.0;
		self.fTempPerm258 = 0.0;
		self.fTempPerm259 = 0.0;
		self.fTempPerm260 = 0.0;
		self.fTempPerm261 = 0.0;
		for l83 in 0..2 {
			self.fVec39[l83 as usize] = 0.0;
		}
		self.fTempPerm262 = 0.0;
		for l84 in 0..2 {
			self.fRec44[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec45[l85 as usize] = 0.0;
		}
		self.fTempPerm263 = 0.0;
		self.iTempPerm264 = 0;
		self.iTempPerm265 = 0;
		self.iTempPerm266 = 0;
		self.iTempPerm267 = 0;
		self.fTempPerm268 = 0.0;
		self.fTempPerm269 = 0.0;
		self.iTempPerm270 = 0;
		self.fTempPerm271 = 0.0;
		self.fTempPerm272 = 0.0;
		self.fTempPerm273 = 0.0;
		for l86 in 0..2 {
			self.fVec40[l86 as usize] = 0.0;
		}
		self.fTempPerm274 = 0.0;
		for l87 in 0..2 {
			self.fRec46[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec47[l88 as usize] = 0.0;
		}
		self.fTempPerm275 = 0.0;
		self.fTempPerm276 = 0.0;
		self.fTempPerm277 = 0.0;
		self.fTempPerm278 = 0.0;
		self.iTempPerm279 = 0;
		self.iTempPerm280 = 0;
		self.iTempPerm281 = 0;
		self.iTempPerm282 = 0;
		self.iTempPerm283 = 0;
		self.iTempPerm284 = 0;
		self.fTempPerm285 = 0.0;
		self.fTempPerm286 = 0.0;
		self.iTempPerm287 = 0;
		self.iTempPerm288 = 0;
		self.iTempPerm289 = 0;
		self.fTempPerm290 = 0.0;
		self.fTempPerm291 = 0.0;
		self.fTempPerm292 = 0.0;
		self.fTempPerm293 = 0.0;
		self.iTempPerm294 = 0;
		self.iTempPerm295 = 0;
		self.fTempPerm296 = 0.0;
		self.fTempPerm297 = 0.0;
		self.fTempPerm298 = 0.0;
		self.fTempPerm299 = 0.0;
		self.iTempPerm300 = 0;
		self.iTempPerm301 = 0;
		self.iTempPerm302 = 0;
		self.iTempPerm303 = 0;
		self.iTempPerm304 = 0;
		self.iTempPerm305 = 0;
		self.fTempPerm306 = 0.0;
		self.iTempPerm307 = 0;
		self.iTempPerm308 = 0;
		self.iTempPerm309 = 0;
		self.fTempPerm310 = 0.0;
		self.fTempPerm311 = 0.0;
		self.fTempPerm312 = 0.0;
		self.fTempPerm313 = 0.0;
		self.fTempPerm314 = 0.0;
		self.fTempPerm315 = 0.0;
		self.fTempPerm316 = 0.0;
		self.fTempPerm317 = 0.0;
		self.fTempPerm318 = 0.0;
		self.iTempPerm319 = 0;
		self.iTempPerm320 = 0;
		self.fTempPerm321 = 0.0;
		self.fTempPerm322 = 0.0;
		self.fTempPerm323 = 0.0;
		self.fTempPerm324 = 0.0;
		self.fTempPerm325 = 0.0;
		self.fTempPerm326 = 0.0;
		self.fTempPerm327 = 0.0;
		self.fTempPerm328 = 0.0;
		for l89 in 0..3 {
			self.fRec48[l89 as usize] = 0.0;
		}
		self.fTempPerm329 = 0.0;
		self.fTempPerm330 = 0.0;
		self.fTempPerm331 = 0.0;
		self.fTempPerm332 = 0.0;
		self.fTempPerm333 = 0.0;
		for l90 in 0..2 {
			self.fRec50[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec49[l91 as usize] = 0.0;
		}
		self.fTempPerm334 = 0.0;
		self.fTempPerm335 = 0.0;
		self.fTempPerm336 = 0.0;
		self.fTempPerm337 = 0.0;
		self.fTempPerm338 = 0.0;
		self.fTempPerm339 = 0.0;
		self.fTempPerm340 = 0.0;
		self.fTempPerm341 = 0.0;
		for l92 in 0..2 {
			self.fVec41[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec52[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec53[l94 as usize] = 0.0;
		}
		self.fTempPerm342 = 0.0;
		self.fTempPerm343 = 0.0;
		for l95 in 0..2 {
			self.fRec51[l95 as usize] = 0.0;
		}
		self.iTempPerm344 = 0;
		for l96 in 0..2 {
			self.fVec42[l96 as usize] = 0.0;
		}
		self.fTempPerm345 = 0.0;
		for l97 in 0..2 {
			self.fRec54[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec55[l98 as usize] = 0.0;
		}
		self.fTempPerm346 = 0.0;
		for l99 in 0..2 {
			self.fVec43[l99 as usize] = 0.0;
		}
		self.fTempPerm347 = 0.0;
		for l100 in 0..2 {
			self.fRec56[l100 as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fRec57[l101 as usize] = 0.0;
		}
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
		self.iTempPerm358 = 0;
		self.fTempPerm359 = 0.0;
		self.fTempPerm360 = 0.0;
		self.fTempPerm361 = 0.0;
		self.fTempPerm362 = 0.0;
		self.fTempPerm363 = 0.0;
		self.fTempPerm364 = 0.0;
		self.fTempPerm365 = 0.0;
		self.fTempPerm366 = 0.0;
		self.fTempPerm367 = 0.0;
		self.fTempPerm368 = 0.0;
		self.fTempPerm369 = 0.0;
		self.fTempPerm370 = 0.0;
		self.fTempPerm371 = 0.0;
		self.fTempPerm372 = 0.0;
		self.fTempPerm373 = 0.0;
		for l102 in 0..2 {
			self.fVec44[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fVec45[l103 as usize] = 0.0;
		}
		self.fTempPerm374 = 0.0;
		self.fTempPerm375 = 0.0;
		self.fTempPerm376 = 0.0;
		for l104 in 0..2 {
			self.fRec58[l104 as usize] = 0.0;
		}
		self.fTempPerm377 = 0.0;
		self.fTempPerm378 = 0.0;
		self.fTempPerm379 = 0.0;
		self.fTempPerm380 = 0.0;
		self.fTempPerm381 = 0.0;
		self.fTempPerm382 = 0.0;
		for l105 in 0..2 {
			self.fVec46[l105 as usize] = 0.0;
		}
		self.fTempPerm383 = 0.0;
		self.iTempPerm384 = 0;
		for l106 in 0..2 {
			self.iRec59[l106 as usize] = 0;
		}
		for l107 in 0..2 {
			self.fVec47[l107 as usize] = 0.0;
		}
		self.fTempPerm385 = 0.0;
		self.fTempPerm386 = 0.0;
		self.fTempPerm387 = 0.0;
		self.fTempPerm388 = 0.0;
		self.fTempPerm389 = 0.0;
		self.fTempPerm390 = 0.0;
		self.fTempPerm391 = 0.0;
		self.fTempPerm392 = 0.0;
		self.fTempPerm393 = 0.0;
		self.fTempPerm394 = 0.0;
		self.fTempPerm395 = 0.0;
		self.fTempPerm396 = 0.0;
		self.fTempPerm397 = 0.0;
		for l108 in 0..2 {
			self.fVec48[l108 as usize] = 0.0;
		}
		self.fTempPerm398 = 0.0;
		self.fTempPerm399 = 0.0;
		for l109 in 0..2 {
			self.fVec49[l109 as usize] = 0.0;
		}
		self.fTempPerm400 = 0.0;
		self.fTempPerm401 = 0.0;
		self.fTempPerm402 = 0.0;
		self.fTempPerm403 = 0.0;
		self.fTempPerm404 = 0.0;
		self.fTempPerm405 = 0.0;
		self.iTempPerm406 = 0;
		self.fTempPerm407 = 0.0;
		self.fTempPerm408 = 0.0;
		self.fTempPerm409 = 0.0;
		for l110 in 0..2 {
			self.fVec50[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fVec51[l111 as usize] = 0.0;
		}
		self.fTempPerm410 = 0.0;
		for l112 in 0..2 {
			self.fVec52[l112 as usize] = 0.0;
		}
		self.fTempPerm411 = 0.0;
		self.iTempPerm412 = 0;
		self.fTempPerm413 = 0.0;
		self.fTempPerm414 = 0.0;
		self.fTempPerm415 = 0.0;
		self.fTempPerm416 = 0.0;
		self.fTempPerm417 = 0.0;
		self.fTempPerm418 = 0.0;
		self.fTempPerm419 = 0.0;
		self.iTempPerm420 = 0;
		self.iTempPerm421 = 0;
		self.iTempPerm422 = 0;
		self.iTempPerm423 = 0;
		self.iTempPerm424 = 0;
		self.iTempPerm425 = 0;
		self.fTempPerm426 = 0.0;
		self.fTempPerm427 = 0.0;
		self.fTempPerm428 = 0.0;
		self.iTempPerm429 = 0;
		self.iTempPerm430 = 0;
		self.iTempPerm431 = 0;
		self.iTempPerm432 = 0;
		self.fTempPerm433 = 0.0;
		self.fTempPerm434 = 0.0;
		self.fTempPerm435 = 0.0;
		for l113 in 0..2 {
			self.fVec53[l113 as usize] = 0.0;
		}
		self.fTempPerm436 = 0.0;
		self.fTempPerm437 = 0.0;
		self.fTempPerm438 = 0.0;
		self.fTempPerm439 = 0.0;
		self.fTempPerm440 = 0.0;
		for l114 in 0..2 {
			self.fVec54[l114 as usize] = 0.0;
		}
		self.fTempPerm441 = 0.0;
		self.fTempPerm442 = 0.0;
		self.fTempPerm443 = 0.0;
		for l115 in 0..2 {
			self.fRec60[l115 as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fVec55[l116 as usize] = 0.0;
		}
		self.fTempPerm444 = 0.0;
		self.fTempPerm445 = 0.0;
		for l117 in 0..2 {
			self.fRec61[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fVec56[l118 as usize] = 0.0;
		}
		self.fTempPerm446 = 0.0;
		for l119 in 0..2 {
			self.fVec57[l119 as usize] = 0.0;
		}
		self.fTempPerm447 = 0.0;
		self.iTempPerm448 = 0;
		self.fTempPerm449 = 0.0;
		self.fTempPerm450 = 0.0;
		self.fTempPerm451 = 0.0;
		self.fTempPerm452 = 0.0;
		self.fTempPerm453 = 0.0;
		for l120 in 0..2 {
			self.fRec62[l120 as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fVec58[l121 as usize] = 0.0;
		}
		self.fTempPerm454 = 0.0;
		self.fTempPerm455 = 0.0;
		self.fTempPerm456 = 0.0;
		self.fTempPerm457 = 0.0;
		self.fTempPerm458 = 0.0;
		for l122 in 0..2 {
			self.fRec63[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fVec59[l123 as usize] = 0.0;
		}
		self.fTempPerm459 = 0.0;
		self.fTempPerm460 = 0.0;
		self.fTempPerm461 = 0.0;
		self.fTempPerm462 = 0.0;
		self.iTempPerm463 = 0;
		self.fTempPerm464 = 0.0;
		self.fTempPerm465 = 0.0;
		self.fTempPerm466 = 0.0;
		for l124 in 0..2 {
			self.fRec64[l124 as usize] = 0.0;
		}
		self.fTempPerm467 = 0.0;
		self.iTempPerm468 = 0;
		self.fTempPerm469 = 0.0;
		self.iTempPerm470 = 0;
		self.fTempPerm471 = 0.0;
		self.fTempPerm472 = 0.0;
		self.fTempPerm473 = 0.0;
		for l125 in 0..2 {
			self.fVec60[l125 as usize] = 0.0;
		}
		self.fTempPerm474 = 0.0;
		self.fTempPerm475 = 0.0;
		self.iTempPerm476 = 0;
		self.fTempPerm477 = 0.0;
		self.fTempPerm478 = 0.0;
		self.fTempPerm479 = 0.0;
		for l126 in 0..2 {
			self.fRec65[l126 as usize] = 0.0;
		}
		self.fTempPerm480 = 0.0;
		self.fTempPerm481 = 0.0;
		self.fTempPerm482 = 0.0;
		self.fTempPerm483 = 0.0;
		self.fTempPerm484 = 0.0;
		self.fTempPerm485 = 0.0;
		self.fTempPerm486 = 0.0;
		self.fTempPerm487 = 0.0;
		self.fTempPerm488 = 0.0;
		for l127 in 0..2 {
			self.fVec61[l127 as usize] = 0.0;
		}
		self.fTempPerm489 = 0.0;
		self.fTempPerm490 = 0.0;
		for l128 in 0..2 {
			self.fVec62[l128 as usize] = 0.0;
		}
		self.fTempPerm491 = 0.0;
		self.fTempPerm492 = 0.0;
		self.fTempPerm493 = 0.0;
		for l129 in 0..2 {
			self.fRec66[l129 as usize] = 0.0;
		}
		self.fTempPerm494 = 0.0;
		self.fTempPerm495 = 0.0;
		self.fTempPerm496 = 0.0;
		self.fTempPerm497 = 0.0;
		self.iTempPerm498 = 0;
		for l130 in 0..2 {
			self.fRec67[l130 as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.fRec68[l131 as usize] = 0.0;
		}
		self.fTempPerm499 = 0.0;
		self.iTempPerm500 = 0;
		self.fTempPerm501 = 0.0;
		self.iTempPerm502 = 0;
		self.fTempPerm503 = 0.0;
		self.iTempPerm504 = 0;
		self.fTempPerm505 = 0.0;
		self.fTempPerm506 = 0.0;
		self.fTempPerm507 = 0.0;
		for l132 in 0..2 {
			self.fVec63[l132 as usize] = 0.0;
		}
		self.fTempPerm508 = 0.0;
		self.iTempPerm509 = 0;
		self.fTempPerm510 = 0.0;
		self.fTempPerm511 = 0.0;
		self.fTempPerm512 = 0.0;
		for l133 in 0..2 {
			self.fRec69[l133 as usize] = 0.0;
		}
		self.fTempPerm513 = 0.0;
		self.fTempPerm514 = 0.0;
		self.fTempPerm515 = 0.0;
		self.fTempPerm516 = 0.0;
		self.fTempPerm517 = 0.0;
		self.fTempPerm518 = 0.0;
		self.fTempPerm519 = 0.0;
		self.fTempPerm520 = 0.0;
		self.fTempPerm521 = 0.0;
		self.fTempPerm522 = 0.0;
		self.fTempPerm523 = 0.0;
		for l134 in 0..2 {
			self.fVec64[l134 as usize] = 0.0;
		}
		self.fTempPerm524 = 0.0;
		self.fTempPerm525 = 0.0;
		for l135 in 0..2 {
			self.fVec65[l135 as usize] = 0.0;
		}
		self.fTempPerm526 = 0.0;
		self.iTempPerm527 = 0;
		self.fTempPerm528 = 0.0;
		self.fTempPerm529 = 0.0;
		self.fTempPerm530 = 0.0;
		self.fTempPerm531 = 0.0;
		self.fTempPerm532 = 0.0;
		self.fTempPerm533 = 0.0;
		self.fTempPerm534 = 0.0;
		self.fTempPerm535 = 0.0;
		self.fTempPerm536 = 0.0;
		self.fTempPerm537 = 0.0;
		self.fTempPerm538 = 0.0;
		self.fTempPerm539 = 0.0;
		self.fTempPerm540 = 0.0;
		for l136 in 0..2 {
			self.fVec66[l136 as usize] = 0.0;
		}
		self.fTempPerm541 = 0.0;
		self.fTempPerm542 = 0.0;
		for l137 in 0..2 {
			self.fVec67[l137 as usize] = 0.0;
		}
		self.fTempPerm543 = 0.0;
		for l138 in 0..2 {
			self.fVec68[l138 as usize] = 0.0;
		}
		self.fTempPerm544 = 0.0;
		self.iTempPerm545 = 0;
		self.fTempPerm546 = 0.0;
		for l139 in 0..2 {
			self.fVec69[l139 as usize] = 0.0;
		}
		for l140 in 0..2 {
			self.fRec70[l140 as usize] = 0.0;
		}
		self.fTempPerm547 = 0.0;
		self.fTempPerm548 = 0.0;
		self.fTempPerm549 = 0.0;
		self.iTempPerm550 = 0;
		self.fTempPerm551 = 0.0;
		self.iTempPerm552 = 0;
		self.fTempPerm553 = 0.0;
		self.fTempPerm554 = 0.0;
		self.fTempPerm555 = 0.0;
		for l141 in 0..2 {
			self.fRec71[l141 as usize] = 0.0;
		}
		self.fTempPerm556 = 0.0;
		self.fTempPerm557 = 0.0;
		for l142 in 0..2 {
			self.fRec72[l142 as usize] = 0.0;
		}
		self.fTempPerm558 = 0.0;
		for l143 in 0..2 {
			self.fRec73[l143 as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fVec70[l144 as usize] = 0.0;
		}
		self.fTempPerm559 = 0.0;
		self.fTempPerm560 = 0.0;
		for l145 in 0..2 {
			self.fVec71[l145 as usize] = 0.0;
		}
		self.fTempPerm561 = 0.0;
		for l146 in 0..2 {
			self.fRec74[l146 as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec75[l147 as usize] = 0.0;
		}
		self.fTempPerm562 = 0.0;
		self.iTempPerm563 = 0;
		self.fTempPerm564 = 0.0;
		self.iTempPerm565 = 0;
		self.fTempPerm566 = 0.0;
		self.iTempPerm567 = 0;
		self.iTempPerm568 = 0;
		self.iTempPerm569 = 0;
		self.iTempPerm570 = 0;
		self.iTempPerm571 = 0;
		self.iTempPerm572 = 0;
		self.fTempPerm573 = 0.0;
		self.iTempPerm574 = 0;
		self.iTempPerm575 = 0;
		self.iTempPerm576 = 0;
		self.fTempPerm577 = 0.0;
		self.fTempPerm578 = 0.0;
		self.fTempPerm579 = 0.0;
		self.fTempPerm580 = 0.0;
		self.fTempPerm581 = 0.0;
		for l148 in 0..2 {
			self.fVec72[l148 as usize] = 0.0;
		}
		self.fTempPerm582 = 0.0;
		for l149 in 0..2 {
			self.fRec76[l149 as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec77[l150 as usize] = 0.0;
		}
		self.fTempPerm583 = 0.0;
		self.iTempPerm584 = 0;
		self.iTempPerm585 = 0;
		self.iTempPerm586 = 0;
		self.iTempPerm587 = 0;
		self.fTempPerm588 = 0.0;
		self.fTempPerm589 = 0.0;
		self.iTempPerm590 = 0;
		self.fTempPerm591 = 0.0;
		self.fTempPerm592 = 0.0;
		self.fTempPerm593 = 0.0;
		for l151 in 0..2 {
			self.fVec73[l151 as usize] = 0.0;
		}
		self.fTempPerm594 = 0.0;
		for l152 in 0..2 {
			self.fRec78[l152 as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec79[l153 as usize] = 0.0;
		}
		self.fTempPerm595 = 0.0;
		self.fTempPerm596 = 0.0;
		self.fTempPerm597 = 0.0;
		self.fTempPerm598 = 0.0;
		self.iTempPerm599 = 0;
		self.iTempPerm600 = 0;
		self.iTempPerm601 = 0;
		self.iTempPerm602 = 0;
		self.iTempPerm603 = 0;
		self.iTempPerm604 = 0;
		self.fTempPerm605 = 0.0;
		self.fTempPerm606 = 0.0;
		self.iTempPerm607 = 0;
		self.iTempPerm608 = 0;
		self.iTempPerm609 = 0;
		self.fTempPerm610 = 0.0;
		self.fTempPerm611 = 0.0;
		self.fTempPerm612 = 0.0;
		self.fTempPerm613 = 0.0;
		self.iTempPerm614 = 0;
		self.iTempPerm615 = 0;
		self.fTempPerm616 = 0.0;
		self.fTempPerm617 = 0.0;
		self.fTempPerm618 = 0.0;
		self.fTempPerm619 = 0.0;
		self.iTempPerm620 = 0;
		self.iTempPerm621 = 0;
		self.iTempPerm622 = 0;
		self.iTempPerm623 = 0;
		self.iTempPerm624 = 0;
		self.iTempPerm625 = 0;
		self.fTempPerm626 = 0.0;
		self.iTempPerm627 = 0;
		self.iTempPerm628 = 0;
		self.iTempPerm629 = 0;
		self.fTempPerm630 = 0.0;
		self.fTempPerm631 = 0.0;
		self.fTempPerm632 = 0.0;
		self.fTempPerm633 = 0.0;
		self.fTempPerm634 = 0.0;
		self.fTempPerm635 = 0.0;
		self.fTempPerm636 = 0.0;
		self.fTempPerm637 = 0.0;
		self.fTempPerm638 = 0.0;
		self.iTempPerm639 = 0;
		self.iTempPerm640 = 0;
		self.fTempPerm641 = 0.0;
		self.fTempPerm642 = 0.0;
		self.fTempPerm643 = 0.0;
		self.fTempPerm644 = 0.0;
		self.fTempPerm645 = 0.0;
		self.fTempPerm646 = 0.0;
		self.fTempPerm647 = 0.0;
		self.fTempPerm648 = 0.0;
		for l154 in 0..3 {
			self.fRec80[l154 as usize] = 0.0;
		}
		self.fTempPerm649 = 0.0;
		self.fTempPerm650 = 0.0;
		self.fTempPerm651 = 0.0;
		self.fTempPerm652 = 0.0;
		self.fTempPerm653 = 0.0;
		for l155 in 0..2 {
			self.fRec82[l155 as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec81[l156 as usize] = 0.0;
		}
		self.fTempPerm654 = 0.0;
		self.fTempPerm655 = 0.0;
		self.fTempPerm656 = 0.0;
		self.fTempPerm657 = 0.0;
		self.fTempPerm658 = 0.0;
		self.fTempPerm659 = 0.0;
		self.fTempPerm660 = 0.0;
		self.fTempPerm661 = 0.0;
		for l157 in 0..2 {
			self.fVec74[l157 as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.fRec83[l158 as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec84[l159 as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.fVec75[l160 as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec85[l161 as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec86[l162 as usize] = 0.0;
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
		ui_interface.declare(Some(ParamIndex(6)), "5", "");
		ui_interface.declare(Some(ParamIndex(6)), "style", "menu{'2 Semitones':0;'Octave':1;'Cutoff 50%':2;'Resonance 50%':3;'Filter B 50%':4}");
		ui_interface.add_num_entry("Bend Mode", ParamIndex(6), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "11.1", "");
		ui_interface.open_horizontal_box("LFO Y");
		ui_interface.declare(Some(ParamIndex(7)), "1", "");
		ui_interface.declare(Some(ParamIndex(7)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'Noise':4;'Brown':5;'S&H':6;'A8 01':7;'A8 02':8;'A8 03':9;'A8 04':10;'A8 05':11;'A8 06':12;'A8 07':13;'A8 08':14;'A8 09':15;'A8 10':16;'A8 11':17;'A8 12':18;'A8 13':19;'A8 14':20;'A8 15':21;'A8 16':22;'A8 17':23;'A8 18':24;'A8 19':25;'A8 20':26;'A8 21':27;'A8 22':28;'A8 23':29;'A8 24':30;'A8 25':31;'A8 26':32;'A8 27':33;'A8 28':34;'A8 29':35;'A8 30':36;'A8 31':37;'A8 32':38;'M8 01':39;'M8 02':40;'M8 03':41;'M8 04':42;'M8 05':43;'M8 06':44;'M8 07':45;'M8 08':46;'M8 09':47;'M8 10':48;'M8 11':49;'M8 12':50;'M8 13':51;'M8 14':52;'M8 15':53;'M8 16':54;'M8 17':55;'M8 18':56;'M8 19':57;'M8 20':58;'M8 21':59;'M8 22':60;'M8 23':61;'M8 24':62;'M8 25':63;'M8 26':64;'M8 27':65;'M8 28':66;'M8 29':67;'M8 30':68;'M8 31':69;'M8 32':70;'M8 33':71;'M8 34':72;'M8 35':73;'M8 36':74;'M8 37':75;'M8 38':76;'M8 39':77;'M8 40':78;'M8 41':79;'M8 42':80;'M8 43':81;'M8 44':82;'M8 45':83;'M8 46':84;'M8 47':85;'M8 48':86;'M8 49':87;'M8 50':88;'M8 51':89;'M8 52':90;'M16 01':91;'M16 02':92;'M16 03':93;'M16 04':94;'M16 05':95;'M16 06':96;'M16 07':97;'M16 08':98;'M16 09':99;'M16 10':100;'M16 11':101;'M16 12':102;'M16 13':103;'M16 14':104;'M16 15':105;'M16 16':106;'M16 17':107;'M16 18':108;'M16 19':109;'M16 20':110;'M16 21':111;'M16 22':112;'M16 23':113;'M16 24':114;'M16 25':115;'M16 26':116;'M16 27':117;'M16 28':118;'M16 29':119;'M16 30':120;'M16 31':121;'M16 32':122;'M16 33':123;'M16 34':124;'M16 35':125;'M16 36':126;'M16 37':127;'M16 38':128;'M16 39':129;'M16 40':130;'M16 41':131;'M16 42':132;'M16 43':133;'M16 44':134;'M16 45':135;'M16 46':136;'M16 47':137}");
		ui_interface.add_num_entry("Shape", ParamIndex(7), 0.0, 0.0, 137.0, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "2", "");
		ui_interface.declare(Some(ParamIndex(8)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(8), 2.0, 0.01, 5e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "3", "");
		ui_interface.add_check_button("Sync", ParamIndex(9));
		ui_interface.declare(Some(ParamIndex(10)), "4", "");
		ui_interface.declare(Some(ParamIndex(10)), "style", "menu{'4/1':0;'2/1.':1;'2/1':2;'1/1.':3;'2/1T':4;'1/1':5;'1/2.':6;'1/1T':7;'1/2':8;'1/4.':9;'1/2T':10;'1/4':11;'1/8.':12;'1/4T':13;'1/8':14;'1/16.':15;'1/8T':16;'1/16':17;'1/32.':18;'1/16T':19;'1/32':20}");
		ui_interface.add_num_entry("Division", ParamIndex(10), 11.0, 0.0, 2e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(11)), "5", "");
		ui_interface.add_check_button("Free", ParamIndex(11));
		ui_interface.close_box();
		ui_interface.declare(None, "11.2", "");
		ui_interface.open_horizontal_box("LFO Z");
		ui_interface.declare(Some(ParamIndex(12)), "1", "");
		ui_interface.declare(Some(ParamIndex(12)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'Noise':4;'Brown':5;'S&H':6;'A8 01':7;'A8 02':8;'A8 03':9;'A8 04':10;'A8 05':11;'A8 06':12;'A8 07':13;'A8 08':14;'A8 09':15;'A8 10':16;'A8 11':17;'A8 12':18;'A8 13':19;'A8 14':20;'A8 15':21;'A8 16':22;'A8 17':23;'A8 18':24;'A8 19':25;'A8 20':26;'A8 21':27;'A8 22':28;'A8 23':29;'A8 24':30;'A8 25':31;'A8 26':32;'A8 27':33;'A8 28':34;'A8 29':35;'A8 30':36;'A8 31':37;'A8 32':38;'M8 01':39;'M8 02':40;'M8 03':41;'M8 04':42;'M8 05':43;'M8 06':44;'M8 07':45;'M8 08':46;'M8 09':47;'M8 10':48;'M8 11':49;'M8 12':50;'M8 13':51;'M8 14':52;'M8 15':53;'M8 16':54;'M8 17':55;'M8 18':56;'M8 19':57;'M8 20':58;'M8 21':59;'M8 22':60;'M8 23':61;'M8 24':62;'M8 25':63;'M8 26':64;'M8 27':65;'M8 28':66;'M8 29':67;'M8 30':68;'M8 31':69;'M8 32':70;'M8 33':71;'M8 34':72;'M8 35':73;'M8 36':74;'M8 37':75;'M8 38':76;'M8 39':77;'M8 40':78;'M8 41':79;'M8 42':80;'M8 43':81;'M8 44':82;'M8 45':83;'M8 46':84;'M8 47':85;'M8 48':86;'M8 49':87;'M8 50':88;'M8 51':89;'M8 52':90;'M16 01':91;'M16 02':92;'M16 03':93;'M16 04':94;'M16 05':95;'M16 06':96;'M16 07':97;'M16 08':98;'M16 09':99;'M16 10':100;'M16 11':101;'M16 12':102;'M16 13':103;'M16 14':104;'M16 15':105;'M16 16':106;'M16 17':107;'M16 18':108;'M16 19':109;'M16 20':110;'M16 21':111;'M16 22':112;'M16 23':113;'M16 24':114;'M16 25':115;'M16 26':116;'M16 27':117;'M16 28':118;'M16 29':119;'M16 30':120;'M16 31':121;'M16 32':122;'M16 33':123;'M16 34':124;'M16 35':125;'M16 36':126;'M16 37':127;'M16 38':128;'M16 39':129;'M16 40':130;'M16 41':131;'M16 42':132;'M16 43':133;'M16 44':134;'M16 45':135;'M16 46':136;'M16 47':137}");
		ui_interface.add_num_entry("Shape", ParamIndex(12), 0.0, 0.0, 137.0, 1.0);
		ui_interface.declare(Some(ParamIndex(13)), "2", "");
		ui_interface.declare(Some(ParamIndex(13)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(13)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(13), 5.0, 0.01, 5e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(14)), "3", "");
		ui_interface.add_check_button("Sync", ParamIndex(14));
		ui_interface.declare(Some(ParamIndex(15)), "4", "");
		ui_interface.declare(Some(ParamIndex(15)), "style", "menu{'4/1':0;'2/1.':1;'2/1':2;'1/1.':3;'2/1T':4;'1/1':5;'1/2.':6;'1/1T':7;'1/2':8;'1/4.':9;'1/2T':10;'1/4':11;'1/8.':12;'1/4T':13;'1/8':14;'1/16.':15;'1/8T':16;'1/16':17;'1/32.':18;'1/16T':19;'1/32':20}");
		ui_interface.add_num_entry("Division", ParamIndex(15), 11.0, 0.0, 2e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(16)), "5", "");
		ui_interface.add_check_button("Free", ParamIndex(16));
		ui_interface.declare(Some(ParamIndex(17)), "6", "");
		ui_interface.declare(Some(ParamIndex(17)), "unit", "ct");
		ui_interface.add_horizontal_slider("Depth", ParamIndex(17), 5e+01, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(18)), "7", "");
		ui_interface.declare(Some(ParamIndex(18)), "style", "menu{'On':0;'MW':1;'AT':2}");
		ui_interface.add_num_entry("Mode", ParamIndex(18), 1.0, 0.0, 2.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "11", "");
		ui_interface.open_horizontal_box("LFO X");
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.declare(Some(ParamIndex(19)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'Noise':4;'Brown':5;'S&H':6;'A8 01':7;'A8 02':8;'A8 03':9;'A8 04':10;'A8 05':11;'A8 06':12;'A8 07':13;'A8 08':14;'A8 09':15;'A8 10':16;'A8 11':17;'A8 12':18;'A8 13':19;'A8 14':20;'A8 15':21;'A8 16':22;'A8 17':23;'A8 18':24;'A8 19':25;'A8 20':26;'A8 21':27;'A8 22':28;'A8 23':29;'A8 24':30;'A8 25':31;'A8 26':32;'A8 27':33;'A8 28':34;'A8 29':35;'A8 30':36;'A8 31':37;'A8 32':38;'M8 01':39;'M8 02':40;'M8 03':41;'M8 04':42;'M8 05':43;'M8 06':44;'M8 07':45;'M8 08':46;'M8 09':47;'M8 10':48;'M8 11':49;'M8 12':50;'M8 13':51;'M8 14':52;'M8 15':53;'M8 16':54;'M8 17':55;'M8 18':56;'M8 19':57;'M8 20':58;'M8 21':59;'M8 22':60;'M8 23':61;'M8 24':62;'M8 25':63;'M8 26':64;'M8 27':65;'M8 28':66;'M8 29':67;'M8 30':68;'M8 31':69;'M8 32':70;'M8 33':71;'M8 34':72;'M8 35':73;'M8 36':74;'M8 37':75;'M8 38':76;'M8 39':77;'M8 40':78;'M8 41':79;'M8 42':80;'M8 43':81;'M8 44':82;'M8 45':83;'M8 46':84;'M8 47':85;'M8 48':86;'M8 49':87;'M8 50':88;'M8 51':89;'M8 52':90;'M16 01':91;'M16 02':92;'M16 03':93;'M16 04':94;'M16 05':95;'M16 06':96;'M16 07':97;'M16 08':98;'M16 09':99;'M16 10':100;'M16 11':101;'M16 12':102;'M16 13':103;'M16 14':104;'M16 15':105;'M16 16':106;'M16 17':107;'M16 18':108;'M16 19':109;'M16 20':110;'M16 21':111;'M16 22':112;'M16 23':113;'M16 24':114;'M16 25':115;'M16 26':116;'M16 27':117;'M16 28':118;'M16 29':119;'M16 30':120;'M16 31':121;'M16 32':122;'M16 33':123;'M16 34':124;'M16 35':125;'M16 36':126;'M16 37':127;'M16 38':128;'M16 39':129;'M16 40':130;'M16 41':131;'M16 42':132;'M16 43':133;'M16 44':134;'M16 45':135;'M16 46':136;'M16 47':137}");
		ui_interface.add_num_entry("Shape", ParamIndex(19), 0.0, 0.0, 137.0, 1.0);
		ui_interface.declare(Some(ParamIndex(20)), "2", "");
		ui_interface.declare(Some(ParamIndex(20)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(20)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(20), 0.5, 0.01, 5e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "3", "");
		ui_interface.add_check_button("Sync", ParamIndex(21));
		ui_interface.declare(Some(ParamIndex(22)), "4", "");
		ui_interface.declare(Some(ParamIndex(22)), "style", "menu{'4/1':0;'2/1.':1;'2/1':2;'1/1.':3;'2/1T':4;'1/1':5;'1/2.':6;'1/1T':7;'1/2':8;'1/4.':9;'1/2T':10;'1/4':11;'1/8.':12;'1/4T':13;'1/8':14;'1/16.':15;'1/8T':16;'1/16':17;'1/32.':18;'1/16T':19;'1/32':20}");
		ui_interface.add_num_entry("Division", ParamIndex(22), 11.0, 0.0, 2e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(23)), "5", "");
		ui_interface.add_check_button("Free", ParamIndex(23));
		ui_interface.close_box();
		ui_interface.declare(None, "13", "");
		ui_interface.open_horizontal_box("Settings");
		ui_interface.declare(Some(ParamIndex(24)), "1", "");
		ui_interface.add_horizontal_slider("Vintage Noise", ParamIndex(24), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(25)), "3", "");
		ui_interface.declare(Some(ParamIndex(25)), "unit", "ms");
		ui_interface.add_horizontal_slider("Ortho Scatter", ParamIndex(25), 1e+02, 0.0, 5e+02, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "14", "");
		ui_interface.open_horizontal_box("Harmonic");
		ui_interface.declare(Some(ParamIndex(26)), "0", "");
		ui_interface.declare(Some(ParamIndex(26)), "style", "menu{'Off':0;'Amp 01':1;'Amp 02':2;'Amp 03':3;'Amp 04':4;'Amp 05':5;'Amp 06':6;'Bass Contour 01':7;'Bass Contour 02':8;'Bass Contour 03':9;'Bell 01':10;'Bell 02':11;'Bell 03':12;'Cream':13;'Dampen':14;'Dirt 01':15;'Dirt 02':16;'Dirt 03':17;'Dirt 04':18;'Dirt 05':19;'Dirt 06':20;'Dist 01':21;'Dist 02':22;'Dungeon':23;'EQ Highs 01':24;'EQ Highs 02':25;'EQ Highs 03':26;'EQ Lows 01':27;'EQ Lows 02':28;'EQ Lows 03':29;'EQ Mids 01':30;'EQ Mids 02':31;'EQ Mids 03':32;'Formant 01':33;'Formant 02':34;'Formant 03':35;'Formant 04':36;'Frozen 01':37;'Frozen 02':38;'Frozen 03':39;'Guitar 01':40;'Guitar 02':41;'Hiss 01':42;'Hiss 02':43;'Inharmonic 01':44;'Inharmonic 02':45;'Inharmonic 03':46;'Inharmonic 04':47;'Inharmonic 05':48;'Mechanic 01':49;'Mechanic 02':50;'Mechanic 03':51;'Mechanic 04':52;'Notch':53;'Nylon':54;'Phase 01':55;'Phase 02':56;'Plastic':57;'Pulse 01':58;'Pulse 02':59;'Pulse 03':60;'Pulse 04':61;'Pulse 05':62;'Pulse 06':63;'Pulse 07':64;'Silk':65;'Space 01':66;'Space 02':67;'Space 03':68;'Space 04':69;'Space 05':70;'Spring':71;'Tones 01':72;'Tones 02':73;'Tones 03':74;'Tones 04':75;'Tones 05':76;'Voice 01':77;'Voice 02':78;'Voice 03':79;'Voice 04':80;'Voice 05':81}");
		ui_interface.add_num_entry("Shape", ParamIndex(26), 0.0, 0.0, 81.0, 1.0);
		ui_interface.declare(Some(ParamIndex(27)), "1", "");
		ui_interface.declare(Some(ParamIndex(27)), "unit", "oct");
		ui_interface.add_horizontal_slider("Shift", ParamIndex(27), 0.0, -4.0, 4.0, 0.001);
		ui_interface.declare(Some(ParamIndex(28)), "2", "");
		ui_interface.declare(Some(ParamIndex(28)), "unit", "dB/oct");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(28), 0.0, -6.0, 6.0, 0.01);
		ui_interface.declare(Some(ParamIndex(29)), "3", "");
		ui_interface.add_horizontal_slider("Depth", ParamIndex(29), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(30)), "4", "");
		ui_interface.add_horizontal_slider("Odd Even", ParamIndex(30), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.1", "");
		ui_interface.open_horizontal_box("Dual 1");
		ui_interface.declare(Some(ParamIndex(31)), "1", "");
		ui_interface.declare(Some(ParamIndex(31)), "style", "menu{'VA Sweep':0;'Spread':1;'PWM':2;'Sync':3;'Fractal':4;'Window Amp Sync':5;'Metal':6;'Ring Mod':7;'FM':8;'Noise AM':9;'Chaos':10;'Folding':11}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(31), 0.0, 0.0, 11.0, 1.0);
		ui_interface.declare(Some(ParamIndex(32)), "2", "");
		ui_interface.declare(Some(ParamIndex(32)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(32), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(33)), "3", "");
		ui_interface.declare(Some(ParamIndex(33)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(33), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(34)), "4", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(34), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(35)), "5", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(35), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.2", "");
		ui_interface.open_horizontal_box("WT 1");
		ui_interface.declare(Some(ParamIndex(36)), "10", "");
		ui_interface.declare(Some(ParamIndex(36)), "style", "menu{'4/1':0;'2/1.':1;'2/1':2;'1/1.':3;'2/1T':4;'1/1':5;'1/2.':6;'1/1T':7;'1/2':8;'1/4.':9;'1/2T':10;'1/4':11;'1/8.':12;'1/4T':13;'1/8':14;'1/16.':15;'1/8T':16;'1/16':17;'1/32.':18;'1/16T':19;'1/32':20}");
		ui_interface.add_num_entry("Speed Division", ParamIndex(36), 8.0, 0.0, 2e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(37)), "1", "");
		ui_interface.declare(Some(ParamIndex(37)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(37), 4.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(38)), "2", "");
		ui_interface.declare(Some(ParamIndex(38)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(38), 12.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(39)), "3", "");
		ui_interface.declare(Some(ParamIndex(39)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 3", ParamIndex(39), 2e+01, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(40)), "4", "");
		ui_interface.declare(Some(ParamIndex(40)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 4", ParamIndex(40), 28.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(41)), "5", "");
		ui_interface.declare(Some(ParamIndex(41)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 5", ParamIndex(41), 36.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(42)), "6", "");
		ui_interface.declare(Some(ParamIndex(42)), "style", "menu{'Off':0;'Bits 4':1;'SR 11k':2;'SR 8k':3;'SR 4k':4;'Simplify 6k':5;'Simplify 1.5k':6;'Simplify 800':7;'Rectify':8;'Tri Fold':9;'Sine Fold':10;'Squish':11;'Ring':12;'Mirror':13;'Sign Swap':14}");
		ui_interface.add_num_entry("OscMod", ParamIndex(42), 0.0, 0.0, 14.0, 1.0);
		ui_interface.declare(Some(ParamIndex(43)), "7", "");
		ui_interface.add_horizontal_slider("OscMod Depth", ParamIndex(43), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(44)), "8", "");
		ui_interface.declare(Some(ParamIndex(44)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(44)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Speed", ParamIndex(44), 0.5, 0.02, 2e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(45)), "9", "");
		ui_interface.add_check_button("Speed Sync", ParamIndex(45));
		ui_interface.close_box();
		ui_interface.declare(None, "2.3", "");
		ui_interface.open_horizontal_box("Ray 1");
		ui_interface.declare(Some(ParamIndex(46)), "10", "");
		ui_interface.declare(Some(ParamIndex(46)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(46)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Orbit Rate", ParamIndex(46), 0.3, 0.02, 1e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(47)), "11", "");
		ui_interface.declare(Some(ParamIndex(47)), "unit", "cents");
		ui_interface.add_horizontal_slider("Fine", ParamIndex(47), 0.0, -1e+02, 1e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(48)), "1", "");
		ui_interface.declare(Some(ParamIndex(48)), "style", "menu{'Earth':0;'Water':1;'Air':2;'Fire':3;'Wood':4;'Metal':5;'Chaos':6}");
		ui_interface.add_num_entry("Element", ParamIndex(48), 0.0, 0.0, 6.0, 1.0);
		ui_interface.declare(Some(ParamIndex(49)), "2", "");
		ui_interface.add_num_entry("Chaos Seed", ParamIndex(49), 4821.0, 1.0, 99999.0, 1.0);
		ui_interface.declare(Some(ParamIndex(50)), "3", "");
		ui_interface.add_check_button("Free", ParamIndex(50));
		ui_interface.declare(Some(ParamIndex(51)), "4", "");
		ui_interface.add_horizontal_slider("Squish", ParamIndex(51), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(52)), "5", "");
		ui_interface.add_horizontal_slider("Skew", ParamIndex(52), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(53)), "6", "");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(53), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(54)), "7", "");
		ui_interface.add_horizontal_slider("Center", ParamIndex(54), 0.0, -1.3, 1.3, 0.001);
		ui_interface.declare(Some(ParamIndex(55)), "8", "");
		ui_interface.add_horizontal_slider("Ray", ParamIndex(55), 0.0, 0.0, 1.5, 0.001);
		ui_interface.declare(Some(ParamIndex(56)), "9", "");
		ui_interface.add_horizontal_slider("Orbit", ParamIndex(56), 0.0, 0.0, 0.8, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.4", "");
		ui_interface.open_horizontal_box("Ortho 1");
		ui_interface.declare(Some(ParamIndex(57)), "1", "");
		ui_interface.declare(Some(ParamIndex(57)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Top", ParamIndex(57), 34.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(58)), "2", "");
		ui_interface.declare(Some(ParamIndex(58)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Bottom", ParamIndex(58), 39.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(59)), "3", "");
		ui_interface.add_horizontal_slider("Top Position", ParamIndex(59), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(60)), "4", "");
		ui_interface.add_horizontal_slider("Bottom Position", ParamIndex(60), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(61)), "5", "");
		ui_interface.declare(Some(ParamIndex(61)), "unit", "%");
		ui_interface.add_horizontal_slider("Top Speed", ParamIndex(61), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(62)), "6", "");
		ui_interface.declare(Some(ParamIndex(62)), "unit", "%");
		ui_interface.add_horizontal_slider("Bottom Speed", ParamIndex(62), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(63)), "7", "");
		ui_interface.add_horizontal_slider("Mix", ParamIndex(63), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(64)), "8", "");
		ui_interface.declare(Some(ParamIndex(64)), "style", "pitch");
		ui_interface.add_horizontal_slider("Top Pitch", ParamIndex(64), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(65)), "9", "");
		ui_interface.declare(Some(ParamIndex(65)), "style", "pitch");
		ui_interface.add_horizontal_slider("Bottom Pitch", ParamIndex(65), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.5", "");
		ui_interface.open_horizontal_box("Windows 1");
		ui_interface.add_horizontal_slider("Int 1 Len", ParamIndex(66), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 1 On", ParamIndex(67));
		ui_interface.declare(Some(ParamIndex(68)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 1 Phase", ParamIndex(68), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(69)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 1 Pitch", ParamIndex(69), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(70)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 1 Shape", ParamIndex(70), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 1 X", ParamIndex(71), 0.0, 0.0, 0.99, 0.001);
		ui_interface.add_horizontal_slider("Int 2 Len", ParamIndex(72), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 2 On", ParamIndex(73));
		ui_interface.declare(Some(ParamIndex(74)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 2 Phase", ParamIndex(74), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(75)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 2 Pitch", ParamIndex(75), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(76)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 2 Shape", ParamIndex(76), 4.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 2 X", ParamIndex(77), 0.33333334, 0.0, 0.99, 0.001);
		ui_interface.add_horizontal_slider("Int 3 Len", ParamIndex(78), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 3 On", ParamIndex(79));
		ui_interface.declare(Some(ParamIndex(80)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 3 Phase", ParamIndex(80), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(81)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 3 Pitch", ParamIndex(81), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(82)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 3 Shape", ParamIndex(82), 1.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 3 X", ParamIndex(83), 0.6666667, 0.0, 0.99, 0.001);
		ui_interface.declare(Some(ParamIndex(84)), "1", "");
		ui_interface.declare(Some(ParamIndex(84)), "style", "menu{'Sine':0;'Cosine':1;'Triangle':2;'Saw':3;'Ramp':4;'Square':5}");
		ui_interface.add_num_entry("Base", ParamIndex(84), 3.0, 0.0, 5.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("Osc 1");
		ui_interface.declare(Some(ParamIndex(85)), "0", "");
		ui_interface.declare(Some(ParamIndex(85)), "style", "menu{'Dual':0;'Wavetable':1;'Ray':2;'Orthogonal':3;'Windows':4}");
		ui_interface.add_num_entry("Mode", ParamIndex(85), 0.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(86)), "1", "");
		ui_interface.declare(Some(ParamIndex(86)), "style", "pitch");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(86), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.1", "");
		ui_interface.open_horizontal_box("Dual 2");
		ui_interface.declare(Some(ParamIndex(87)), "1", "");
		ui_interface.declare(Some(ParamIndex(87)), "style", "menu{'VA Sweep':0;'Spread':1;'PWM':2;'Sync':3;'Fractal':4;'Window Amp Sync':5;'Metal':6;'Ring Mod':7;'FM':8;'Noise AM':9;'Chaos':10;'Folding':11}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(87), 0.0, 0.0, 11.0, 1.0);
		ui_interface.declare(Some(ParamIndex(88)), "2", "");
		ui_interface.declare(Some(ParamIndex(88)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(88), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(89)), "3", "");
		ui_interface.declare(Some(ParamIndex(89)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(89), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(90)), "4", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(90), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(91)), "5", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(91), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.2", "");
		ui_interface.open_horizontal_box("WT 2");
		ui_interface.declare(Some(ParamIndex(92)), "10", "");
		ui_interface.declare(Some(ParamIndex(92)), "style", "menu{'4/1':0;'2/1.':1;'2/1':2;'1/1.':3;'2/1T':4;'1/1':5;'1/2.':6;'1/1T':7;'1/2':8;'1/4.':9;'1/2T':10;'1/4':11;'1/8.':12;'1/4T':13;'1/8':14;'1/16.':15;'1/8T':16;'1/16':17;'1/32.':18;'1/16T':19;'1/32':20}");
		ui_interface.add_num_entry("Speed Division", ParamIndex(92), 8.0, 0.0, 2e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(93)), "1", "");
		ui_interface.declare(Some(ParamIndex(93)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(93), 4.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(94)), "2", "");
		ui_interface.declare(Some(ParamIndex(94)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(94), 12.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(95)), "3", "");
		ui_interface.declare(Some(ParamIndex(95)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 3", ParamIndex(95), 2e+01, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(96)), "4", "");
		ui_interface.declare(Some(ParamIndex(96)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 4", ParamIndex(96), 28.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(97)), "5", "");
		ui_interface.declare(Some(ParamIndex(97)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3;'A8 01':4;'A8 02':5;'A8 03':6;'A8 04':7;'A8 05':8;'A8 06':9;'A8 07':10;'A8 08':11;'A8 09':12;'A8 10':13;'A8 11':14;'A8 12':15;'A8 13':16;'A8 14':17;'A8 15':18;'A8 16':19;'A8 17':20;'A8 18':21;'A8 19':22;'A8 20':23;'A8 21':24;'A8 22':25;'A8 23':26;'A8 24':27;'A8 25':28;'A8 26':29;'A8 27':30;'A8 28':31;'A8 29':32;'A8 30':33;'A8 31':34;'A8 32':35;'M8 01':36;'M8 02':37;'M8 03':38;'M8 04':39;'M8 05':40;'M8 06':41;'M8 07':42;'M8 08':43;'M8 09':44;'M8 10':45;'M8 11':46;'M8 12':47;'M8 13':48;'M8 14':49;'M8 15':50;'M8 16':51;'M8 17':52;'M8 18':53;'M8 19':54;'M8 20':55;'M8 21':56;'M8 22':57;'M8 23':58;'M8 24':59;'M8 25':60;'M8 26':61;'M8 27':62;'M8 28':63;'M8 29':64;'M8 30':65;'M8 31':66;'M8 32':67;'M8 33':68;'M8 34':69;'M8 35':70;'M8 36':71;'M8 37':72;'M8 38':73;'M8 39':74;'M8 40':75;'M8 41':76;'M8 42':77;'M8 43':78;'M8 44':79;'M8 45':80;'M8 46':81;'M8 47':82;'M8 48':83;'M8 49':84;'M8 50':85;'M8 51':86;'M8 52':87;'M16 01':88;'M16 02':89;'M16 03':90;'M16 04':91;'M16 05':92;'M16 06':93;'M16 07':94;'M16 08':95;'M16 09':96;'M16 10':97;'M16 11':98;'M16 12':99;'M16 13':100;'M16 14':101;'M16 15':102;'M16 16':103;'M16 17':104;'M16 18':105;'M16 19':106;'M16 20':107;'M16 21':108;'M16 22':109;'M16 23':110;'M16 24':111;'M16 25':112;'M16 26':113;'M16 27':114;'M16 28':115;'M16 29':116;'M16 30':117;'M16 31':118;'M16 32':119;'M16 33':120;'M16 34':121;'M16 35':122;'M16 36':123;'M16 37':124;'M16 38':125;'M16 39':126;'M16 40':127;'M16 41':128;'M16 42':129;'M16 43':130;'M16 44':131;'M16 45':132;'M16 46':133;'M16 47':134}");
		ui_interface.add_num_entry("Wave 5", ParamIndex(97), 36.0, 0.0, 134.0, 1.0);
		ui_interface.declare(Some(ParamIndex(98)), "6", "");
		ui_interface.declare(Some(ParamIndex(98)), "style", "menu{'Off':0;'Bits 4':1;'SR 11k':2;'SR 8k':3;'SR 4k':4;'Simplify 6k':5;'Simplify 1.5k':6;'Simplify 800':7;'Rectify':8;'Tri Fold':9;'Sine Fold':10;'Squish':11;'Ring':12;'Mirror':13;'Sign Swap':14}");
		ui_interface.add_num_entry("OscMod", ParamIndex(98), 0.0, 0.0, 14.0, 1.0);
		ui_interface.declare(Some(ParamIndex(99)), "7", "");
		ui_interface.add_horizontal_slider("OscMod Depth", ParamIndex(99), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(100)), "8", "");
		ui_interface.declare(Some(ParamIndex(100)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(100)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Speed", ParamIndex(100), 0.5, 0.02, 2e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(101)), "9", "");
		ui_interface.add_check_button("Speed Sync", ParamIndex(101));
		ui_interface.close_box();
		ui_interface.declare(None, "3.3", "");
		ui_interface.open_horizontal_box("Ray 2");
		ui_interface.declare(Some(ParamIndex(102)), "10", "");
		ui_interface.declare(Some(ParamIndex(102)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(102)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Orbit Rate", ParamIndex(102), 0.3, 0.02, 1e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(103)), "11", "");
		ui_interface.declare(Some(ParamIndex(103)), "unit", "cents");
		ui_interface.add_horizontal_slider("Fine", ParamIndex(103), 0.0, -1e+02, 1e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(104)), "1", "");
		ui_interface.declare(Some(ParamIndex(104)), "style", "menu{'Earth':0;'Water':1;'Air':2;'Fire':3;'Wood':4;'Metal':5;'Chaos':6}");
		ui_interface.add_num_entry("Element", ParamIndex(104), 3.0, 0.0, 6.0, 1.0);
		ui_interface.declare(Some(ParamIndex(105)), "2", "");
		ui_interface.add_num_entry("Chaos Seed", ParamIndex(105), 6821.0, 1.0, 99999.0, 1.0);
		ui_interface.declare(Some(ParamIndex(106)), "3", "");
		ui_interface.add_check_button("Free", ParamIndex(106));
		ui_interface.declare(Some(ParamIndex(107)), "4", "");
		ui_interface.add_horizontal_slider("Squish", ParamIndex(107), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(108)), "5", "");
		ui_interface.add_horizontal_slider("Skew", ParamIndex(108), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(109)), "6", "");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(109), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(110)), "7", "");
		ui_interface.add_horizontal_slider("Center", ParamIndex(110), 0.0, -1.3, 1.3, 0.001);
		ui_interface.declare(Some(ParamIndex(111)), "8", "");
		ui_interface.add_horizontal_slider("Ray", ParamIndex(111), 0.0, 0.0, 1.5, 0.001);
		ui_interface.declare(Some(ParamIndex(112)), "9", "");
		ui_interface.add_horizontal_slider("Orbit", ParamIndex(112), 0.0, 0.0, 0.8, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.4", "");
		ui_interface.open_horizontal_box("Ortho 2");
		ui_interface.declare(Some(ParamIndex(113)), "1", "");
		ui_interface.declare(Some(ParamIndex(113)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Top", ParamIndex(113), 52.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(114)), "2", "");
		ui_interface.declare(Some(ParamIndex(114)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Bottom", ParamIndex(114), 31.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(115)), "3", "");
		ui_interface.add_horizontal_slider("Top Position", ParamIndex(115), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(116)), "4", "");
		ui_interface.add_horizontal_slider("Bottom Position", ParamIndex(116), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(117)), "5", "");
		ui_interface.declare(Some(ParamIndex(117)), "unit", "%");
		ui_interface.add_horizontal_slider("Top Speed", ParamIndex(117), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(118)), "6", "");
		ui_interface.declare(Some(ParamIndex(118)), "unit", "%");
		ui_interface.add_horizontal_slider("Bottom Speed", ParamIndex(118), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(119)), "7", "");
		ui_interface.add_horizontal_slider("Mix", ParamIndex(119), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(120)), "8", "");
		ui_interface.declare(Some(ParamIndex(120)), "style", "pitch");
		ui_interface.add_horizontal_slider("Top Pitch", ParamIndex(120), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(121)), "9", "");
		ui_interface.declare(Some(ParamIndex(121)), "style", "pitch");
		ui_interface.add_horizontal_slider("Bottom Pitch", ParamIndex(121), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.5", "");
		ui_interface.open_horizontal_box("Windows 2");
		ui_interface.add_horizontal_slider("Int 1 Len", ParamIndex(122), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 1 On", ParamIndex(123));
		ui_interface.declare(Some(ParamIndex(124)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 1 Phase", ParamIndex(124), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(125)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 1 Pitch", ParamIndex(125), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(126)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 1 Shape", ParamIndex(126), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 1 X", ParamIndex(127), 0.0, 0.0, 0.99, 0.001);
		ui_interface.add_horizontal_slider("Int 2 Len", ParamIndex(128), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 2 On", ParamIndex(129));
		ui_interface.declare(Some(ParamIndex(130)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 2 Phase", ParamIndex(130), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(131)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 2 Pitch", ParamIndex(131), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(132)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 2 Shape", ParamIndex(132), 4.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 2 X", ParamIndex(133), 0.33333334, 0.0, 0.99, 0.001);
		ui_interface.add_horizontal_slider("Int 3 Len", ParamIndex(134), 0.15, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("Int 3 On", ParamIndex(135));
		ui_interface.declare(Some(ParamIndex(136)), "unit", "°");
		ui_interface.add_horizontal_slider("Int 3 Phase", ParamIndex(136), 0.0, 0.0, 3.6e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(137)), "style", "ratio");
		ui_interface.add_horizontal_slider("Int 3 Pitch", ParamIndex(137), 0.0, -2.0, 6.0, 0.001);
		ui_interface.declare(Some(ParamIndex(138)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Ramp':3;'Square':4}");
		ui_interface.add_num_entry("Int 3 Shape", ParamIndex(138), 1.0, 0.0, 4.0, 1.0);
		ui_interface.add_horizontal_slider("Int 3 X", ParamIndex(139), 0.6666667, 0.0, 0.99, 0.001);
		ui_interface.declare(Some(ParamIndex(140)), "1", "");
		ui_interface.declare(Some(ParamIndex(140)), "style", "menu{'Sine':0;'Cosine':1;'Triangle':2;'Saw':3;'Ramp':4;'Square':5}");
		ui_interface.add_num_entry("Base", ParamIndex(140), 3.0, 0.0, 5.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("Osc 2");
		ui_interface.declare(Some(ParamIndex(141)), "0", "");
		ui_interface.declare(Some(ParamIndex(141)), "style", "menu{'Dual':0;'Wavetable':1;'Ray':2;'Orthogonal':3;'Windows':4}");
		ui_interface.add_num_entry("Mode", ParamIndex(141), 1.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(142)), "1", "");
		ui_interface.declare(Some(ParamIndex(142)), "style", "pitch");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(142), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(143)), "1", "");
		ui_interface.add_horizontal_slider("Balance", ParamIndex(143), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("Filter");
		ui_interface.declare(Some(ParamIndex(144)), "10", "");
		ui_interface.add_horizontal_slider("Env B", ParamIndex(144), 0.21, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "9.1", "");
		ui_interface.open_horizontal_box("Env B");
		ui_interface.declare(Some(ParamIndex(145)), "1", "");
		ui_interface.declare(Some(ParamIndex(145)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(145)), "unit", "s");
		ui_interface.add_vertical_slider("Attack", ParamIndex(145), 0.005, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(146)), "2", "");
		ui_interface.declare(Some(ParamIndex(146)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(146)), "unit", "s");
		ui_interface.add_vertical_slider("Decay", ParamIndex(146), 0.4, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(147)), "3", "");
		ui_interface.add_vertical_slider("Sustain", ParamIndex(147), 0.3, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(148)), "4", "");
		ui_interface.declare(Some(ParamIndex(148)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(148)), "unit", "s");
		ui_interface.add_vertical_slider("Release", ParamIndex(148), 0.5, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(149)), "5", "");
		ui_interface.add_horizontal_slider("Attack Curve", ParamIndex(149), 0.6, -1.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(150)), "6", "");
		ui_interface.add_horizontal_slider("DR Curve", ParamIndex(150), -0.6, -1.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "9.2", "");
		ui_interface.open_horizontal_box("Env C");
		ui_interface.declare(Some(ParamIndex(151)), "1", "");
		ui_interface.declare(Some(ParamIndex(151)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(151)), "unit", "s");
		ui_interface.add_vertical_slider("Attack", ParamIndex(151), 0.005, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(152)), "2", "");
		ui_interface.declare(Some(ParamIndex(152)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(152)), "unit", "s");
		ui_interface.add_vertical_slider("Decay", ParamIndex(152), 0.4, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(153)), "3", "");
		ui_interface.add_vertical_slider("Sustain", ParamIndex(153), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(154)), "4", "");
		ui_interface.declare(Some(ParamIndex(154)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(154)), "unit", "s");
		ui_interface.add_vertical_slider("Release", ParamIndex(154), 0.5, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(155)), "5", "");
		ui_interface.add_horizontal_slider("Attack Curve", ParamIndex(155), 0.6, -1.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(156)), "6", "");
		ui_interface.add_horizontal_slider("DR Curve", ParamIndex(156), -0.6, -1.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("Env A");
		ui_interface.declare(Some(ParamIndex(157)), "1", "");
		ui_interface.declare(Some(ParamIndex(157)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(157)), "unit", "s");
		ui_interface.add_vertical_slider("Attack", ParamIndex(157), 0.005, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(158)), "2", "");
		ui_interface.declare(Some(ParamIndex(158)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(158)), "unit", "s");
		ui_interface.add_vertical_slider("Decay", ParamIndex(158), 0.4, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(159)), "3", "");
		ui_interface.add_vertical_slider("Sustain", ParamIndex(159), 0.6, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(160)), "4", "");
		ui_interface.declare(Some(ParamIndex(160)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(160)), "unit", "s");
		ui_interface.add_vertical_slider("Release", ParamIndex(160), 0.5, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(161)), "5", "");
		ui_interface.add_horizontal_slider("Attack Curve", ParamIndex(161), 0.6, -1.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(162)), "6", "");
		ui_interface.add_horizontal_slider("DR Curve", ParamIndex(162), -0.6, -1.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(163)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(163)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(163), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("copy", ParamIndex(164), 3.0, 0.0, 6.0, 1.0);
		ui_interface.add_horizontal_slider("fade1", ParamIndex(165), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("fade2", ParamIndex(166), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("freq", ParamIndex(167), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_button("gate", ParamIndex(168));
		ui_interface.add_horizontal_slider("glide", ParamIndex(169), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("morph1", ParamIndex(170), 0.0, 0.0, 3.0, 0.0001);
		ui_interface.add_horizontal_slider("morph2", ParamIndex(171), 0.0, 0.0, 3.0, 0.0001);
		ui_interface.add_horizontal_slider("ramp", ParamIndex(172), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("ratio", ParamIndex(173), 1.0, 0.5, 2.0, 1e-06);
		ui_interface.add_horizontal_slider("seed", ParamIndex(174), 0.0, 0.0, 1.0, 1e-06);
		ui_interface.add_horizontal_slider("slot", ParamIndex(175), 0.0, 0.0, 64.0, 1.0);
		ui_interface.add_horizontal_slider("vibrato", ParamIndex(176), 0.0, -24.0, 24.0, 0.0001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			168 => Some(self.fButton0),
			3 => Some(self.fCheckbox0),
			101 => Some(self.fCheckbox1),
			135 => Some(self.fCheckbox10),
			129 => Some(self.fCheckbox11),
			123 => Some(self.fCheckbox12),
			79 => Some(self.fCheckbox13),
			73 => Some(self.fCheckbox14),
			67 => Some(self.fCheckbox15),
			106 => Some(self.fCheckbox16),
			50 => Some(self.fCheckbox17),
			45 => Some(self.fCheckbox2),
			16 => Some(self.fCheckbox3),
			14 => Some(self.fCheckbox4),
			11 => Some(self.fCheckbox5),
			9 => Some(self.fCheckbox6),
			23 => Some(self.fCheckbox7),
			21 => Some(self.fCheckbox8),
			5 => Some(self.fCheckbox9),
			85 => Some(self.fEntry0),
			31 => Some(self.fEntry1),
			38 => Some(self.fEntry10),
			39 => Some(self.fEntry11),
			40 => Some(self.fEntry12),
			37 => Some(self.fEntry13),
			89 => Some(self.fEntry14),
			88 => Some(self.fEntry15),
			97 => Some(self.fEntry16),
			94 => Some(self.fEntry17),
			95 => Some(self.fEntry18),
			96 => Some(self.fEntry19),
			42 => Some(self.fEntry2),
			93 => Some(self.fEntry20),
			92 => Some(self.fEntry21),
			36 => Some(self.fEntry22),
			18 => Some(self.fEntry23),
			15 => Some(self.fEntry24),
			12 => Some(self.fEntry25),
			10 => Some(self.fEntry26),
			7 => Some(self.fEntry27),
			22 => Some(self.fEntry28),
			19 => Some(self.fEntry29),
			98 => Some(self.fEntry3),
			1 => Some(self.fEntry30),
			26 => Some(self.fEntry31),
			138 => Some(self.fEntry32),
			132 => Some(self.fEntry33),
			126 => Some(self.fEntry34),
			140 => Some(self.fEntry35),
			82 => Some(self.fEntry36),
			76 => Some(self.fEntry37),
			70 => Some(self.fEntry38),
			84 => Some(self.fEntry39),
			141 => Some(self.fEntry4),
			114 => Some(self.fEntry40),
			113 => Some(self.fEntry41),
			58 => Some(self.fEntry42),
			57 => Some(self.fEntry43),
			105 => Some(self.fEntry44),
			104 => Some(self.fEntry45),
			49 => Some(self.fEntry46),
			48 => Some(self.fEntry47),
			6 => Some(self.fEntry5),
			87 => Some(self.fEntry6),
			33 => Some(self.fEntry7),
			32 => Some(self.fEntry8),
			41 => Some(self.fEntry9),
			143 => Some(self.fHslider0),
			86 => Some(self.fHslider1),
			167 => Some(self.fHslider10),
			164 => Some(self.fHslider11),
			174 => Some(self.fHslider12),
			35 => Some(self.fHslider13),
			34 => Some(self.fHslider14),
			165 => Some(self.fHslider15),
			170 => Some(self.fHslider16),
			43 => Some(self.fHslider17),
			142 => Some(self.fHslider18),
			91 => Some(self.fHslider19),
			172 => Some(self.fHslider2),
			90 => Some(self.fHslider20),
			166 => Some(self.fHslider21),
			171 => Some(self.fHslider22),
			99 => Some(self.fHslider23),
			4 => Some(self.fHslider24),
			2 => Some(self.fHslider25),
			144 => Some(self.fHslider26),
			100 => Some(self.fHslider27),
			44 => Some(self.fHslider28),
			17 => Some(self.fHslider29),
			0 => Some(self.fHslider3),
			13 => Some(self.fHslider30),
			8 => Some(self.fHslider31),
			20 => Some(self.fHslider32),
			156 => Some(self.fHslider33),
			155 => Some(self.fHslider34),
			150 => Some(self.fHslider35),
			149 => Some(self.fHslider36),
			162 => Some(self.fHslider37),
			161 => Some(self.fHslider38),
			30 => Some(self.fHslider39),
			175 => Some(self.fHslider4),
			29 => Some(self.fHslider40),
			28 => Some(self.fHslider41),
			27 => Some(self.fHslider42),
			25 => Some(self.fHslider43),
			137 => Some(self.fHslider44),
			134 => Some(self.fHslider45),
			136 => Some(self.fHslider46),
			139 => Some(self.fHslider47),
			131 => Some(self.fHslider48),
			128 => Some(self.fHslider49),
			24 => Some(self.fHslider5),
			130 => Some(self.fHslider50),
			133 => Some(self.fHslider51),
			125 => Some(self.fHslider52),
			122 => Some(self.fHslider53),
			124 => Some(self.fHslider54),
			127 => Some(self.fHslider55),
			81 => Some(self.fHslider56),
			78 => Some(self.fHslider57),
			80 => Some(self.fHslider58),
			83 => Some(self.fHslider59),
			163 => Some(self.fHslider6),
			75 => Some(self.fHslider60),
			72 => Some(self.fHslider61),
			74 => Some(self.fHslider62),
			77 => Some(self.fHslider63),
			69 => Some(self.fHslider64),
			66 => Some(self.fHslider65),
			68 => Some(self.fHslider66),
			71 => Some(self.fHslider67),
			121 => Some(self.fHslider68),
			120 => Some(self.fHslider69),
			176 => Some(self.fHslider7),
			119 => Some(self.fHslider70),
			118 => Some(self.fHslider71),
			117 => Some(self.fHslider72),
			116 => Some(self.fHslider73),
			115 => Some(self.fHslider74),
			65 => Some(self.fHslider75),
			64 => Some(self.fHslider76),
			63 => Some(self.fHslider77),
			62 => Some(self.fHslider78),
			61 => Some(self.fHslider79),
			173 => Some(self.fHslider8),
			60 => Some(self.fHslider80),
			59 => Some(self.fHslider81),
			103 => Some(self.fHslider82),
			102 => Some(self.fHslider83),
			112 => Some(self.fHslider84),
			111 => Some(self.fHslider85),
			110 => Some(self.fHslider86),
			109 => Some(self.fHslider87),
			108 => Some(self.fHslider88),
			107 => Some(self.fHslider89),
			169 => Some(self.fHslider9),
			47 => Some(self.fHslider90),
			46 => Some(self.fHslider91),
			56 => Some(self.fHslider92),
			55 => Some(self.fHslider93),
			54 => Some(self.fHslider94),
			53 => Some(self.fHslider95),
			52 => Some(self.fHslider96),
			51 => Some(self.fHslider97),
			154 => Some(self.fVslider0),
			153 => Some(self.fVslider1),
			158 => Some(self.fVslider10),
			157 => Some(self.fVslider11),
			152 => Some(self.fVslider2),
			151 => Some(self.fVslider3),
			148 => Some(self.fVslider4),
			147 => Some(self.fVslider5),
			146 => Some(self.fVslider6),
			145 => Some(self.fVslider7),
			160 => Some(self.fVslider8),
			159 => Some(self.fVslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			168 => { self.fButton0 = value }
			3 => { self.fCheckbox0 = value }
			101 => { self.fCheckbox1 = value }
			135 => { self.fCheckbox10 = value }
			129 => { self.fCheckbox11 = value }
			123 => { self.fCheckbox12 = value }
			79 => { self.fCheckbox13 = value }
			73 => { self.fCheckbox14 = value }
			67 => { self.fCheckbox15 = value }
			106 => { self.fCheckbox16 = value }
			50 => { self.fCheckbox17 = value }
			45 => { self.fCheckbox2 = value }
			16 => { self.fCheckbox3 = value }
			14 => { self.fCheckbox4 = value }
			11 => { self.fCheckbox5 = value }
			9 => { self.fCheckbox6 = value }
			23 => { self.fCheckbox7 = value }
			21 => { self.fCheckbox8 = value }
			5 => { self.fCheckbox9 = value }
			85 => { self.fEntry0 = value }
			31 => { self.fEntry1 = value }
			38 => { self.fEntry10 = value }
			39 => { self.fEntry11 = value }
			40 => { self.fEntry12 = value }
			37 => { self.fEntry13 = value }
			89 => { self.fEntry14 = value }
			88 => { self.fEntry15 = value }
			97 => { self.fEntry16 = value }
			94 => { self.fEntry17 = value }
			95 => { self.fEntry18 = value }
			96 => { self.fEntry19 = value }
			42 => { self.fEntry2 = value }
			93 => { self.fEntry20 = value }
			92 => { self.fEntry21 = value }
			36 => { self.fEntry22 = value }
			18 => { self.fEntry23 = value }
			15 => { self.fEntry24 = value }
			12 => { self.fEntry25 = value }
			10 => { self.fEntry26 = value }
			7 => { self.fEntry27 = value }
			22 => { self.fEntry28 = value }
			19 => { self.fEntry29 = value }
			98 => { self.fEntry3 = value }
			1 => { self.fEntry30 = value }
			26 => { self.fEntry31 = value }
			138 => { self.fEntry32 = value }
			132 => { self.fEntry33 = value }
			126 => { self.fEntry34 = value }
			140 => { self.fEntry35 = value }
			82 => { self.fEntry36 = value }
			76 => { self.fEntry37 = value }
			70 => { self.fEntry38 = value }
			84 => { self.fEntry39 = value }
			141 => { self.fEntry4 = value }
			114 => { self.fEntry40 = value }
			113 => { self.fEntry41 = value }
			58 => { self.fEntry42 = value }
			57 => { self.fEntry43 = value }
			105 => { self.fEntry44 = value }
			104 => { self.fEntry45 = value }
			49 => { self.fEntry46 = value }
			48 => { self.fEntry47 = value }
			6 => { self.fEntry5 = value }
			87 => { self.fEntry6 = value }
			33 => { self.fEntry7 = value }
			32 => { self.fEntry8 = value }
			41 => { self.fEntry9 = value }
			143 => { self.fHslider0 = value }
			86 => { self.fHslider1 = value }
			167 => { self.fHslider10 = value }
			164 => { self.fHslider11 = value }
			174 => { self.fHslider12 = value }
			35 => { self.fHslider13 = value }
			34 => { self.fHslider14 = value }
			165 => { self.fHslider15 = value }
			170 => { self.fHslider16 = value }
			43 => { self.fHslider17 = value }
			142 => { self.fHslider18 = value }
			91 => { self.fHslider19 = value }
			172 => { self.fHslider2 = value }
			90 => { self.fHslider20 = value }
			166 => { self.fHslider21 = value }
			171 => { self.fHslider22 = value }
			99 => { self.fHslider23 = value }
			4 => { self.fHslider24 = value }
			2 => { self.fHslider25 = value }
			144 => { self.fHslider26 = value }
			100 => { self.fHslider27 = value }
			44 => { self.fHslider28 = value }
			17 => { self.fHslider29 = value }
			0 => { self.fHslider3 = value }
			13 => { self.fHslider30 = value }
			8 => { self.fHslider31 = value }
			20 => { self.fHslider32 = value }
			156 => { self.fHslider33 = value }
			155 => { self.fHslider34 = value }
			150 => { self.fHslider35 = value }
			149 => { self.fHslider36 = value }
			162 => { self.fHslider37 = value }
			161 => { self.fHslider38 = value }
			30 => { self.fHslider39 = value }
			175 => { self.fHslider4 = value }
			29 => { self.fHslider40 = value }
			28 => { self.fHslider41 = value }
			27 => { self.fHslider42 = value }
			25 => { self.fHslider43 = value }
			137 => { self.fHslider44 = value }
			134 => { self.fHslider45 = value }
			136 => { self.fHslider46 = value }
			139 => { self.fHslider47 = value }
			131 => { self.fHslider48 = value }
			128 => { self.fHslider49 = value }
			24 => { self.fHslider5 = value }
			130 => { self.fHslider50 = value }
			133 => { self.fHslider51 = value }
			125 => { self.fHslider52 = value }
			122 => { self.fHslider53 = value }
			124 => { self.fHslider54 = value }
			127 => { self.fHslider55 = value }
			81 => { self.fHslider56 = value }
			78 => { self.fHslider57 = value }
			80 => { self.fHslider58 = value }
			83 => { self.fHslider59 = value }
			163 => { self.fHslider6 = value }
			75 => { self.fHslider60 = value }
			72 => { self.fHslider61 = value }
			74 => { self.fHslider62 = value }
			77 => { self.fHslider63 = value }
			69 => { self.fHslider64 = value }
			66 => { self.fHslider65 = value }
			68 => { self.fHslider66 = value }
			71 => { self.fHslider67 = value }
			121 => { self.fHslider68 = value }
			120 => { self.fHslider69 = value }
			176 => { self.fHslider7 = value }
			119 => { self.fHslider70 = value }
			118 => { self.fHslider71 = value }
			117 => { self.fHslider72 = value }
			116 => { self.fHslider73 = value }
			115 => { self.fHslider74 = value }
			65 => { self.fHslider75 = value }
			64 => { self.fHslider76 = value }
			63 => { self.fHslider77 = value }
			62 => { self.fHslider78 = value }
			61 => { self.fHslider79 = value }
			173 => { self.fHslider8 = value }
			60 => { self.fHslider80 = value }
			59 => { self.fHslider81 = value }
			103 => { self.fHslider82 = value }
			102 => { self.fHslider83 = value }
			112 => { self.fHslider84 = value }
			111 => { self.fHslider85 = value }
			110 => { self.fHslider86 = value }
			109 => { self.fHslider87 = value }
			108 => { self.fHslider88 = value }
			107 => { self.fHslider89 = value }
			169 => { self.fHslider9 = value }
			47 => { self.fHslider90 = value }
			46 => { self.fHslider91 = value }
			56 => { self.fHslider92 = value }
			55 => { self.fHslider93 = value }
			54 => { self.fHslider94 = value }
			53 => { self.fHslider95 = value }
			52 => { self.fHslider96 = value }
			51 => { self.fHslider97 = value }
			154 => { self.fVslider0 = value }
			153 => { self.fVslider1 = value }
			158 => { self.fVslider10 = value }
			157 => { self.fVslider11 = value }
			152 => { self.fVslider2 = value }
			151 => { self.fVslider3 = value }
			148 => { self.fVslider4 = value }
			147 => { self.fVslider5 = value }
			146 => { self.fVslider6 = value }
			145 => { self.fVslider7 = value }
			160 => { self.fVslider8 = value }
			159 => { self.fVslider9 = value }
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
		let mut fSlow0: F32 = self.fHslider0;
		let mut iSlow1: i32 = (fSlow0 < 0.9999) as i32;
		let mut fSlow2: F32 = (iSlow1) as F32;
		let mut iSlow3: i32 = (fSlow2 != 0.0) as i32;
		let mut fSlow4: F32 = self.fEntry0;
		let mut iSlow5: i32 = (fSlow4 == 0.0) as i32;
		let mut fSlow6: F32 = (iSlow5) as F32;
		let mut iSlow7: i32 = (fSlow6 != 0.0) as i32;
		let mut fSlow8: F32 = self.fEntry1;
		let mut iSlow9: i32 = (fSlow8 == 2.0) as i32;
		let mut fSlow10: F32 = (iSlow9) as F32;
		let mut iSlow11: i32 = (fSlow10 != 0.0) as i32;
		let mut iSlow12: i32 = (fSlow8 == 0.0) as i32;
		let mut fSlow13: F32 = (iSlow12) as F32;
		let mut iSlow14: i32 = (fSlow13 != 0.0) as i32;
		let mut iSlow15: i32 = (fSlow8 == 3.0) as i32;
		let mut fSlow16: F32 = (iSlow15) as F32;
		let mut iSlow17: i32 = (fSlow16 != 0.0) as i32;
		let mut iSlow18: i32 = (fSlow8 == 4.0) as i32;
		let mut fSlow19: F32 = (iSlow18) as F32;
		let mut iSlow20: i32 = (fSlow19 != 0.0) as i32;
		let mut iSlow21: i32 = (fSlow8 == 5.0) as i32;
		let mut fSlow22: F32 = (iSlow21) as F32;
		let mut iSlow23: i32 = (fSlow22 != 0.0) as i32;
		let mut iSlow24: i32 = (fSlow8 == 6.0) as i32;
		let mut fSlow25: F32 = (iSlow24) as F32;
		let mut iSlow26: i32 = (fSlow25 != 0.0) as i32;
		let mut iSlow27: i32 = (fSlow8 == 7.0) as i32;
		let mut fSlow28: F32 = (iSlow27) as F32;
		let mut iSlow29: i32 = (fSlow28 != 0.0) as i32;
		let mut iSlow30: i32 = (fSlow8 == 9.0) as i32;
		let mut fSlow31: F32 = (iSlow30) as F32;
		let mut iSlow32: i32 = (fSlow31 != 0.0) as i32;
		let mut iSlow33: i32 = (fSlow8 == 11.0) as i32;
		let mut fSlow34: F32 = (iSlow33) as F32;
		let mut iSlow35: i32 = (fSlow34 != 0.0) as i32;
		let mut fSlow36: F32 = self.fEntry2;
		let mut iSlow37: i32 = (fSlow36 == 1e+01) as i32;
		let mut fSlow38: F32 = (iSlow37) as F32;
		let mut iSlow39: i32 = (fSlow38 != 0.0) as i32;
		let mut iSlow40: i32 = (fSlow4 == 1.0) as i32;
		let mut fSlow41: F32 = (iSlow40) as F32;
		let mut iSlow42: i32 = (fSlow41 != 0.0) as i32;
		let mut iSlow43: i32 = (fSlow36 == 9.0) as i32;
		let mut fSlow44: F32 = (iSlow43) as F32;
		let mut iSlow45: i32 = (fSlow44 != 0.0) as i32;
		let mut iSlow46: i32 = (fSlow36 == 8.0) as i32;
		let mut fSlow47: F32 = (iSlow46) as F32;
		let mut iSlow48: i32 = (fSlow47 != 0.0) as i32;
		let mut iSlow49: i32 = (fSlow36 == 7.0) as i32;
		let mut fSlow50: F32 = (iSlow49) as F32;
		let mut iSlow51: i32 = (fSlow50 != 0.0) as i32;
		let mut iSlow52: i32 = (fSlow36 == 6.0) as i32;
		let mut fSlow53: F32 = (iSlow52) as F32;
		let mut iSlow54: i32 = (fSlow53 != 0.0) as i32;
		let mut iSlow55: i32 = (fSlow36 == 5.0) as i32;
		let mut fSlow56: F32 = (iSlow55) as F32;
		let mut iSlow57: i32 = (fSlow56 != 0.0) as i32;
		let mut iSlow58: i32 = (fSlow36 == 4.0) as i32;
		let mut fSlow59: F32 = (iSlow58) as F32;
		let mut iSlow60: i32 = (fSlow59 != 0.0) as i32;
		let mut iSlow61: i32 = (fSlow36 == 3.0) as i32;
		let mut fSlow62: F32 = (iSlow61) as F32;
		let mut iSlow63: i32 = (fSlow62 != 0.0) as i32;
		let mut iSlow64: i32 = (fSlow36 == 2.0) as i32;
		let mut fSlow65: F32 = (iSlow64) as F32;
		let mut iSlow66: i32 = (fSlow65 != 0.0) as i32;
		let mut iSlow67: i32 = (fSlow36 == 1.0) as i32;
		let mut fSlow68: F32 = (iSlow67) as F32;
		let mut iSlow69: i32 = (fSlow68 != 0.0) as i32;
		let mut iSlow70: i32 = (fSlow36 == 0.0) as i32;
		let mut fSlow71: F32 = (iSlow70) as F32;
		let mut iSlow72: i32 = (fSlow71 != 0.0) as i32;
		let mut iSlow73: i32 = (fSlow36 == 13.0) as i32;
		let mut fSlow74: F32 = (iSlow73) as F32;
		let mut iSlow75: i32 = (fSlow74 != 0.0) as i32;
		let mut iSlow76: i32 = (fSlow36 == 12.0) as i32;
		let mut fSlow77: F32 = (iSlow76) as F32;
		let mut iSlow78: i32 = (fSlow77 != 0.0) as i32;
		let mut iSlow79: i32 = (fSlow36 == 11.0) as i32;
		let mut fSlow80: F32 = (iSlow79) as F32;
		let mut iSlow81: i32 = (fSlow80 != 0.0) as i32;
		let mut iSlow82: i32 = (fSlow36 == 14.0) as i32;
		let mut fSlow83: F32 = (iSlow82) as F32;
		let mut iSlow84: i32 = (fSlow83 != 0.0) as i32;
		let mut iSlow85: i32 = (fSlow8 == 1.0) as i32;
		let mut fSlow86: F32 = (iSlow85) as F32;
		let mut iSlow87: i32 = (fSlow86 != 0.0) as i32;
		let mut fSlow88: F32 = self.fButton0;
		let mut fSlow89: F32 = self.fHslider1;
		let mut fSlow90: F32 = self.fHslider2;
		let mut fSlow91: F32 = self.fHslider3;
		let mut fSlow92: F32 = self.fEntry3;
		let mut iSlow93: i32 = (fSlow92 == 4.0) as i32;
		let mut fSlow94: F32 = (iSlow93) as F32;
		let mut iSlow95: i32 = (fSlow94 != 0.0) as i32;
		let mut fSlow96: F32 = self.fEntry4;
		let mut iSlow97: i32 = (fSlow96 == 1.0) as i32;
		let mut fSlow98: F32 = (iSlow97) as F32;
		let mut iSlow99: i32 = (fSlow98 != 0.0) as i32;
		let mut iSlow100: i32 = (fSlow0 > 0.0001) as i32;
		let mut fSlow101: F32 = (iSlow100) as F32;
		let mut iSlow102: i32 = (fSlow101 != 0.0) as i32;
		let mut iSlow103: i32 = (fSlow92 == 3.0) as i32;
		let mut fSlow104: F32 = (iSlow103) as F32;
		let mut iSlow105: i32 = (fSlow104 != 0.0) as i32;
		let mut iSlow106: i32 = (fSlow92 == 2.0) as i32;
		let mut fSlow107: F32 = (iSlow106) as F32;
		let mut iSlow108: i32 = (fSlow107 != 0.0) as i32;
		let mut iTemp7: i32 = self.iTempPerm0;
		let mut fTemp8: F32 = self.fTempPerm1;
		let mut fSlow109: F32 = self.fHslider4;
		let mut fSlow110: F32 = 12.9898 * fSlow109;
		let mut fSlow111: F32 = 43758.547 * F32::sin(fSlow110 + 234.699);
		let mut fSlow112: F32 = 43758.547 * F32::sin(fSlow110 + 7979.766);
		let mut fSlow113: F32 = 2.0 * (fSlow112 - F32::floor(fSlow112)) + -1.0;
		let mut fSlow114: F32 = self.fConst2 * (0.1 * F32::max(-1.0, F32::min(1.0, 0.70710677 * (fSlow113 + 2.0 * (fSlow111 - F32::floor(fSlow111)) + -1.0))) + 0.45);
		let mut fTemp9: F32 = self.fTempPerm2;
		let mut iSlow115: i32 = (409.0 * fSlow109 + 4e+03) as i32;
		let mut iTemp10: i32 = self.iTempPerm3;
		let mut fSlow116: F32 = self.fHslider5;
		let mut fSlow117: F32 = 43758.547 * F32::sin(fSlow110 + 156.466);
		let mut fSlow118: F32 = 0.15 * F32::max(-1.0, F32::min(1.0, 1.2803688 * (0.6 * fSlow113 + 0.5 * (2.0 * (fSlow117 - F32::floor(fSlow117)) + -1.0)))) + 0.25;
		let mut fSlow119: F32 = 1.0 / F32::tan(self.fConst8 * F32::max(1.0, fSlow118));
		let mut fSlow120: F32 = 1.0 - fSlow119;
		let mut fSlow121: F32 = self.fConst2 * F32::max(fSlow118, 0.0);
		let mut fTemp13: F32 = self.fTempPerm4;
		let mut fTemp16: F32 = self.fTempPerm5;
		let mut fSlow122: F32 = 1.0 / (fSlow119 + 1.0);
		let mut fSlow123: F32 = 43758.547 * F32::sin(fSlow110 + 78.233);
		let mut fSlow124: F32 = 43758.547 * F32::sin(fSlow110 + 7901.533);
		let mut fSlow125: F32 = 8.0 * F32::max(-1.0, F32::min(1.0, 1.0976425 * (0.7 * (2.0 * (fSlow124 - F32::floor(fSlow124)) + -1.0) + 0.3 * fSlow113 + 0.5 * (2.0 * (fSlow123 - F32::floor(fSlow123)) + -1.0))));
		let mut fTemp17: F32 = self.fTempPerm6;
		let mut fSlow126: F32 = self.fHslider6;
		let mut fSlow127: F32 = self.fEntry5;
		let mut fSlow128: F32 = 2.0 * ((fSlow127 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow127 == 1.0) as i32) as u32 as F32;
		let mut fSlow129: F32 = self.fHslider7;
		let mut fSlow130: F32 = self.fHslider8;
		let mut fSlow131: F32 = 0.33333334 * F32::max(1e-05, self.fHslider9);
		let mut iSlow132: i32 = (F32::abs(fSlow131) < 1.1920929e-07) as i32;
		let mut fSlow133: F32 = if iSlow132 != 0 {0.0} else {F32::exp(-(self.fConst2 / if iSlow132 != 0 {1.0} else {fSlow131}))};
		let mut fSlow134: F32 = F32::log(self.fHslider10, std::f32::consts::E) * (1.0 - fSlow133);
		let mut fTemp24: F32 = self.fTempPerm7;
		let mut fTemp25: F32 = self.fTempPerm8;
		let mut fSlow135: F32 = self.fHslider11;
		let mut fSlow136: F32 = 0.618 * fSlow135 + 7.31 * self.fHslider12 * (fSlow135 + 1.0);
		let mut fSlow137: F32 = self.fCheckbox0;
		let mut fSlow138: F32 = self.fEntry6;
		let mut iSlow139: i32 = (fSlow138 == 2.0) as i32;
		let mut fSlow140: F32 = (iSlow139) as F32;
		let mut iSlow141: i32 = (fSlow140 != 0.0) as i32;
		let mut iSlow142: i32 = (fSlow96 == 0.0) as i32;
		let mut fSlow143: F32 = (iSlow142) as F32;
		let mut iSlow144: i32 = (fSlow143 != 0.0) as i32;
		let mut iSlow145: i32 = (fSlow138 == 0.0) as i32;
		let mut fSlow146: F32 = (iSlow145) as F32;
		let mut iSlow147: i32 = (fSlow146 != 0.0) as i32;
		let mut iSlow148: i32 = (fSlow138 == 3.0) as i32;
		let mut fSlow149: F32 = (iSlow148) as F32;
		let mut iSlow150: i32 = (fSlow149 != 0.0) as i32;
		let mut iSlow151: i32 = (fSlow138 == 4.0) as i32;
		let mut fSlow152: F32 = (iSlow151) as F32;
		let mut iSlow153: i32 = (fSlow152 != 0.0) as i32;
		let mut iSlow154: i32 = (fSlow138 == 5.0) as i32;
		let mut fSlow155: F32 = (iSlow154) as F32;
		let mut iSlow156: i32 = (fSlow155 != 0.0) as i32;
		let mut iSlow157: i32 = (fSlow138 == 6.0) as i32;
		let mut fSlow158: F32 = (iSlow157) as F32;
		let mut iSlow159: i32 = (fSlow158 != 0.0) as i32;
		let mut iSlow160: i32 = (fSlow138 == 7.0) as i32;
		let mut fSlow161: F32 = (iSlow160) as F32;
		let mut iSlow162: i32 = (fSlow161 != 0.0) as i32;
		let mut iSlow163: i32 = (fSlow138 == 8.0) as i32;
		let mut fSlow164: F32 = (iSlow163) as F32;
		let mut iSlow165: i32 = (fSlow164 != 0.0) as i32;
		let mut iSlow166: i32 = (fSlow138 == 9.0) as i32;
		let mut fSlow167: F32 = (iSlow166) as F32;
		let mut iSlow168: i32 = (fSlow167 != 0.0) as i32;
		let mut iSlow169: i32 = (fSlow138 == 1e+01) as i32;
		let mut fSlow170: F32 = (iSlow169) as F32;
		let mut iSlow171: i32 = (fSlow170 != 0.0) as i32;
		let mut iSlow172: i32 = (fSlow138 == 11.0) as i32;
		let mut fSlow173: F32 = (iSlow172) as F32;
		let mut iSlow174: i32 = (fSlow173 != 0.0) as i32;
		let mut iSlow175: i32 = (fSlow138 == 1.0) as i32;
		let mut fSlow176: F32 = (iSlow175) as F32;
		let mut iSlow177: i32 = (fSlow176 != 0.0) as i32;
		let mut iSlow178: i32 = (fSlow92 == 14.0) as i32;
		let mut fSlow179: F32 = (iSlow178) as F32;
		let mut iSlow180: i32 = (fSlow179 != 0.0) as i32;
		let mut iSlow181: i32 = (fSlow92 == 13.0) as i32;
		let mut fSlow182: F32 = (iSlow181) as F32;
		let mut iSlow183: i32 = (fSlow182 != 0.0) as i32;
		let mut iSlow184: i32 = (fSlow92 == 12.0) as i32;
		let mut fSlow185: F32 = (iSlow184) as F32;
		let mut iSlow186: i32 = (fSlow185 != 0.0) as i32;
		let mut iSlow187: i32 = (fSlow92 == 11.0) as i32;
		let mut fSlow188: F32 = (iSlow187) as F32;
		let mut iSlow189: i32 = (fSlow188 != 0.0) as i32;
		let mut iSlow190: i32 = (fSlow92 == 1e+01) as i32;
		let mut fSlow191: F32 = (iSlow190) as F32;
		let mut iSlow192: i32 = (fSlow191 != 0.0) as i32;
		let mut iSlow193: i32 = (fSlow92 == 9.0) as i32;
		let mut fSlow194: F32 = (iSlow193) as F32;
		let mut iSlow195: i32 = (fSlow194 != 0.0) as i32;
		let mut iSlow196: i32 = (fSlow92 == 8.0) as i32;
		let mut fSlow197: F32 = (iSlow196) as F32;
		let mut iSlow198: i32 = (fSlow197 != 0.0) as i32;
		let mut iSlow199: i32 = (fSlow92 == 7.0) as i32;
		let mut fSlow200: F32 = (iSlow199) as F32;
		let mut iSlow201: i32 = (fSlow200 != 0.0) as i32;
		let mut iSlow202: i32 = (fSlow92 == 6.0) as i32;
		let mut fSlow203: F32 = (iSlow202) as F32;
		let mut iSlow204: i32 = (fSlow203 != 0.0) as i32;
		let mut iSlow205: i32 = (fSlow92 == 5.0) as i32;
		let mut fSlow206: F32 = (iSlow205) as F32;
		let mut iSlow207: i32 = (fSlow206 != 0.0) as i32;
		let mut iSlow208: i32 = (fSlow92 == 1.0) as i32;
		let mut fSlow209: F32 = (iSlow208) as F32;
		let mut iSlow210: i32 = (fSlow209 != 0.0) as i32;
		let mut iSlow211: i32 = (fSlow92 == 0.0) as i32;
		let mut fSlow212: F32 = (iSlow211) as F32;
		let mut iSlow213: i32 = (fSlow212 != 0.0) as i32;
		let mut iSlow214: i32 = (fSlow8 == 8.0) as i32;
		let mut fSlow215: F32 = (iSlow214) as F32;
		let mut iSlow216: i32 = (fSlow215 != 0.0) as i32;
		let mut iSlow217: i32 = (fSlow8 == 1e+01) as i32;
		let mut fSlow218: F32 = (iSlow217) as F32;
		let mut iSlow219: i32 = (fSlow218 != 0.0) as i32;
		let mut fTemp29: F32 = self.fTempPerm9;
		let mut iTemp30: i32 = self.iTempPerm10;
		let mut fSlow220: F32 = self.fHslider13;
		let mut fTemp31: F32 = self.fTempPerm11;
		let mut fTemp32: F32 = self.fTempPerm12;
		let mut fSlow221: F32 = self.fHslider14;
		let mut fTemp33: F32 = self.fTempPerm13;
		let mut fTemp34: F32 = self.fTempPerm14;
		let mut fTemp35: F32 = self.fTempPerm15;
		let mut fTemp36: F32 = self.fTempPerm16;
		let mut fTemp37: F32 = self.fTempPerm17;
		let mut fTemp38: F32 = self.fTempPerm18;
		let mut fTemp39: F32 = self.fTempPerm19;
		let mut fTemp40: F32 = self.fTempPerm20;
		let mut fSlow222: F32 = self.fEntry7;
		let mut fSlow223: F32 = self.fEntry8;
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
		let mut fSlow224: F32 = ((iSlow9 != 0) as i32) as u32 as F32;
		let mut iSlow225: i32 = (fSlow222) as i32;
		let mut fTemp61: F32 = self.fTempPerm41;
		let mut fTemp62: F32 = self.fTempPerm42;
		let mut fSlow226: F32 = 0.5 * fSlow137;
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
		let mut fSlow227: F32 = ((iSlow12 != 0) as i32) as u32 as F32;
		let mut iTemp73: i32 = self.iTempPerm53;
		let mut iSlow228: i32 = (fSlow223) as i32;
		let mut fTemp74: F32 = self.fTempPerm54;
		let mut fTemp75: F32 = self.fTempPerm55;
		let mut fTemp76: F32 = self.fTempPerm56;
		let mut fTemp77: F32 = self.fTempPerm57;
		let mut fTemp78: F32 = self.fTempPerm58;
		let mut fTemp79: F32 = self.fTempPerm59;
		let mut fTemp80: F32 = self.fTempPerm60;
		let mut fTemp81: F32 = self.fTempPerm61;
		let mut fTemp82: F32 = self.fTempPerm62;
		let mut iSlow229: i32 = (fSlow223 >= 2.0) as i32;
		let mut iSlow230: i32 = (fSlow223 >= 1.0) as i32;
		let mut fTemp83: F32 = self.fTempPerm63;
		let mut iSlow231: i32 = (fSlow223 >= 3.0) as i32;
		let mut fSlow232: F32 = (if iSlow231 != 0 {0} else {2}) as F32;
		let mut fSlow233: F32 = if iSlow229 != 0 {fSlow232} else {if iSlow230 != 0 {4.0} else {6.2831855}};
		let mut fTemp84: F32 = self.fTempPerm64;
		let mut fTemp85: F32 = self.fTempPerm65;
		let mut fTemp86: F32 = self.fTempPerm66;
		let mut fTemp87: F32 = self.fTempPerm67;
		let mut fTemp88: F32 = self.fTempPerm68;
		let mut fTemp89: F32 = self.fTempPerm69;
		let mut fSlow234: F32 = ((iSlow15 != 0) as i32) as u32 as F32;
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
		let mut fSlow235: F32 = ((iSlow18 != 0) as i32) as u32 as F32;
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
		let mut fSlow236: F32 = ((iSlow21 != 0) as i32) as u32 as F32;
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
		let mut fSlow237: F32 = ((iSlow24 != 0) as i32) as u32 as F32;
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
		let mut fSlow238: F32 = ((iSlow27 != 0) as i32) as u32 as F32;
		let mut fTemp153: F32 = self.fTempPerm133;
		let mut fTemp154: F32 = self.fTempPerm134;
		let mut fTemp155: F32 = self.fTempPerm135;
		let mut iTemp156: i32 = self.iTempPerm136;
		let mut iSlow239: i32 = (fSlow222 >= 2.0) as i32;
		let mut iSlow240: i32 = (fSlow222 >= 3.0) as i32;
		let mut iSlow241: i32 = if iSlow239 != 0 {if iSlow240 != 0 {-2} else {0}} else {0};
		let mut iSlow242: i32 = (fSlow222 >= 1.0) as i32;
		let mut fSlow243: F32 = if iSlow239 != 0 {if iSlow240 != 0 {0.5} else {0.0}} else {if iSlow242 != 0 {0.75} else {0.0}};
		let mut fTemp157: F32 = self.fTempPerm137;
		let mut fTemp158: F32 = self.fTempPerm138;
		let mut fTemp159: F32 = self.fTempPerm139;
		let mut fTemp160: F32 = self.fTempPerm140;
		let mut iTemp161: i32 = self.iTempPerm141;
		let mut iSlow244: i32 = if iSlow239 != 0 {if iSlow240 != 0 {2} else {-2}} else {0};
		let mut fSlow245: F32 = if iSlow239 != 0 {if iSlow240 != 0 {0.0} else {0.5}} else {if iSlow242 != 0 {0.25} else {0.0}};
		let mut fTemp162: F32 = self.fTempPerm142;
		let mut iTemp163: i32 = self.iTempPerm143;
		let mut fTemp164: F32 = self.fTempPerm144;
		let mut fTemp165: F32 = self.fTempPerm145;
		let mut fTemp166: F32 = self.fTempPerm146;
		let mut fTemp167: F32 = self.fTempPerm147;
		let mut fTemp168: F32 = self.fTempPerm148;
		let mut iTemp169: i32 = self.iTempPerm149;
		let mut iSlow246: i32 = if iSlow229 != 0 {if iSlow231 != 0 {-2} else {0}} else {0};
		let mut fTemp170: F32 = self.fTempPerm150;
		let mut fTemp171: F32 = self.fTempPerm151;
		let mut fSlow247: F32 = if iSlow229 != 0 {if iSlow231 != 0 {0.5} else {0.0}} else {if iSlow230 != 0 {0.75} else {0.0}};
		let mut fTemp172: F32 = self.fTempPerm152;
		let mut fTemp173: F32 = self.fTempPerm153;
		let mut fTemp174: F32 = self.fTempPerm154;
		let mut fTemp175: F32 = self.fTempPerm155;
		let mut fSlow248: F32 = (if iSlow229 != 0 {0} else {if iSlow230 != 0 {8} else {0}}) as F32;
		let mut fTemp176: F32 = self.fTempPerm156;
		let mut iSlow249: i32 = if iSlow229 != 0 {if iSlow231 != 0 {2} else {-2}} else {0};
		let mut fTemp177: F32 = self.fTempPerm157;
		let mut fSlow250: F32 = if iSlow229 != 0 {if iSlow231 != 0 {0.0} else {0.5}} else {if iSlow230 != 0 {0.25} else {0.0}};
		let mut fTemp178: F32 = self.fTempPerm158;
		let mut fTemp179: F32 = self.fTempPerm159;
		let mut fTemp180: F32 = self.fTempPerm160;
		let mut fSlow251: F32 = (if iSlow229 != 0 {0} else {if iSlow230 != 0 {-8} else {0}}) as F32;
		let mut fTemp181: F32 = self.fTempPerm161;
		let mut fTemp182: F32 = self.fTempPerm162;
		let mut fTemp183: F32 = self.fTempPerm163;
		let mut fTemp184: F32 = self.fTempPerm164;
		let mut fSlow252: F32 = ((iSlow214 != 0) as i32) as u32 as F32;
		let mut fTemp185: F32 = self.fTempPerm165;
		let mut fTemp186: F32 = self.fTempPerm166;
		let mut fTemp187: F32 = self.fTempPerm167;
		let mut fTemp188: F32 = self.fTempPerm168;
		let mut fSlow253: F32 = ((iSlow30 != 0) as i32) as u32 as F32;
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
		let mut fSlow254: F32 = ((iSlow217 != 0) as i32) as u32 as F32;
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
		let mut fSlow255: F32 = ((iSlow33 != 0) as i32) as u32 as F32;
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
		let mut fSlow256: F32 = ((iSlow85 != 0) as i32) as u32 as F32;
		let mut fSlow257: F32 = unsafe { ftbl4OscDspSIG4[((fSlow8) as i32) as usize] };
		let mut fTemp260: F32 = self.fTempPerm240;
		let mut fSlow258: F32 = ((iSlow5 != 0) as i32) as u32 as F32;
		let mut fSlow259: F32 = self.fHslider15;
		let mut fTemp261: F32 = self.fTempPerm241;
		let mut fTemp262: F32 = self.fTempPerm242;
		let mut iTemp263: i32 = self.iTempPerm243;
		let mut fTemp264: F32 = self.fTempPerm244;
		let mut iTemp265: i32 = self.iTempPerm245;
		let mut fTemp266: F32 = self.fTempPerm246;
		let mut iTemp267: i32 = self.iTempPerm247;
		let mut iTemp268: i32 = self.iTempPerm248;
		let mut iTemp269: i32 = self.iTempPerm249;
		let mut iTemp270: i32 = self.iTempPerm250;
		let mut iTemp271: i32 = self.iTempPerm251;
		let mut iTemp272: i32 = self.iTempPerm252;
		let mut fTemp273: F32 = self.fTempPerm253;
		let mut iTemp274: i32 = self.iTempPerm254;
		let mut iTemp275: i32 = self.iTempPerm255;
		let mut iTemp276: i32 = self.iTempPerm256;
		let mut iSlow260: i32 = i32::wrapping_mul(2176, (self.fEntry9) as i32);
		let mut fTemp277: F32 = self.fTempPerm257;
		let mut fTemp278: F32 = self.fTempPerm258;
		let mut fTemp279: F32 = self.fTempPerm259;
		let mut fTemp280: F32 = self.fTempPerm260;
		let mut fTemp281: F32 = self.fTempPerm261;
		let mut fSlow261: F32 = self.fHslider16;
		let mut fTemp282: F32 = self.fTempPerm262;
		let mut fTemp283: F32 = self.fTempPerm263;
		let mut iTemp284: i32 = self.iTempPerm264;
		let mut iTemp285: i32 = self.iTempPerm265;
		let mut iTemp286: i32 = self.iTempPerm266;
		let mut fSlow262: F32 = self.fEntry10;
		let mut fSlow263: F32 = self.fEntry11;
		let mut fSlow264: F32 = self.fEntry12;
		let mut iTemp287: i32 = self.iTempPerm267;
		let mut fTemp288: F32 = self.fTempPerm268;
		let mut fTemp289: F32 = self.fTempPerm269;
		let mut fSlow265: F32 = self.fEntry13;
		let mut iTemp290: i32 = self.iTempPerm270;
		let mut fTemp291: F32 = self.fTempPerm271;
		let mut fTemp292: F32 = self.fTempPerm272;
		let mut fTemp293: F32 = self.fTempPerm273;
		let mut fSlow266: F32 = self.fHslider17;
		let mut fTemp294: F32 = self.fTempPerm274;
		let mut fTemp295: F32 = self.fTempPerm275;
		let mut fTemp296: F32 = self.fTempPerm276;
		let mut fTemp297: F32 = self.fTempPerm277;
		let mut fSlow267: F32 = ((iSlow82 != 0) as i32) as u32 as F32;
		let mut fTemp298: F32 = self.fTempPerm278;
		let mut iTemp299: i32 = self.iTempPerm279;
		let mut iTemp300: i32 = self.iTempPerm280;
		let mut iTemp301: i32 = self.iTempPerm281;
		let mut iTemp302: i32 = self.iTempPerm282;
		let mut iTemp303: i32 = self.iTempPerm283;
		let mut iTemp304: i32 = self.iTempPerm284;
		let mut fTemp305: F32 = self.fTempPerm285;
		let mut fTemp306: F32 = self.fTempPerm286;
		let mut iTemp307: i32 = self.iTempPerm287;
		let mut iTemp308: i32 = self.iTempPerm288;
		let mut iTemp309: i32 = self.iTempPerm289;
		let mut fTemp310: F32 = self.fTempPerm290;
		let mut fTemp311: F32 = self.fTempPerm291;
		let mut fTemp312: F32 = self.fTempPerm292;
		let mut fTemp313: F32 = self.fTempPerm293;
		let mut iTemp314: i32 = self.iTempPerm294;
		let mut iTemp315: i32 = self.iTempPerm295;
		let mut fTemp316: F32 = self.fTempPerm296;
		let mut fSlow268: F32 = ((iSlow73 != 0) as i32) as u32 as F32;
		let mut fTemp317: F32 = self.fTempPerm297;
		let mut fSlow269: F32 = ((iSlow76 != 0) as i32) as u32 as F32;
		let mut fTemp318: F32 = self.fTempPerm298;
		let mut fTemp319: F32 = self.fTempPerm299;
		let mut iTemp320: i32 = self.iTempPerm300;
		let mut iTemp321: i32 = self.iTempPerm301;
		let mut iTemp322: i32 = self.iTempPerm302;
		let mut iTemp323: i32 = self.iTempPerm303;
		let mut iTemp324: i32 = self.iTempPerm304;
		let mut iTemp325: i32 = self.iTempPerm305;
		let mut fTemp326: F32 = self.fTempPerm306;
		let mut iTemp327: i32 = self.iTempPerm307;
		let mut iTemp328: i32 = self.iTempPerm308;
		let mut iTemp329: i32 = self.iTempPerm309;
		let mut fTemp330: F32 = self.fTempPerm310;
		let mut fTemp331: F32 = self.fTempPerm311;
		let mut fTemp332: F32 = self.fTempPerm312;
		let mut fTemp333: F32 = self.fTempPerm313;
		let mut fTemp334: F32 = self.fTempPerm314;
		let mut fSlow270: F32 = ((iSlow79 != 0) as i32) as u32 as F32;
		let mut fTemp335: F32 = self.fTempPerm315;
		let mut fSlow271: F32 = ((iSlow37 != 0) as i32) as u32 as F32;
		let mut fTemp336: F32 = self.fTempPerm316;
		let mut fTemp337: F32 = self.fTempPerm317;
		let mut fSlow272: F32 = ((iSlow43 != 0) as i32) as u32 as F32;
		let mut fTemp338: F32 = self.fTempPerm318;
		let mut iTemp339: i32 = self.iTempPerm319;
		let mut iTemp340: i32 = self.iTempPerm320;
		let mut fTemp341: F32 = self.fTempPerm321;
		let mut fTemp342: F32 = self.fTempPerm322;
		let mut fTemp343: F32 = self.fTempPerm323;
		let mut fTemp344: F32 = self.fTempPerm324;
		let mut fTemp345: F32 = self.fTempPerm325;
		let mut fSlow273: F32 = ((iSlow46 != 0) as i32) as u32 as F32;
		let mut fSlow274: F32 = F32::max(0.0, F32::min(2.0, fSlow36 + -5.0));
		let mut fSlow275: F32 = self.fConst24 * (if (fSlow274 >= 2.0) as i32 != 0 {800} else {if (fSlow274 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp346: F32 = self.fTempPerm326;
		let mut fTemp347: F32 = self.fTempPerm327;
		let mut fTemp348: F32 = self.fTempPerm328;
		let mut fTemp349: F32 = self.fTempPerm329;
		let mut fTemp350: F32 = self.fTempPerm330;
		let mut fSlow276: F32 = ((iSlow49 != 0) as i32) as u32 as F32;
		let mut fTemp351: F32 = self.fTempPerm331;
		let mut fSlow277: F32 = ((iSlow52 != 0) as i32) as u32 as F32;
		let mut fTemp352: F32 = self.fTempPerm332;
		let mut fSlow278: F32 = ((iSlow55 != 0) as i32) as u32 as F32;
		let mut fSlow279: F32 = F32::max(0.0, F32::min(2.0, fSlow36 + -2.0));
		let mut fSlow280: F32 = self.fConst2 * (if (fSlow279 >= 2.0) as i32 != 0 {4000} else {if (fSlow279 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp353: F32 = self.fTempPerm333;
		let mut fTemp354: F32 = self.fTempPerm334;
		let mut fSlow281: F32 = ((iSlow58 != 0) as i32) as u32 as F32;
		let mut fTemp355: F32 = self.fTempPerm335;
		let mut fSlow282: F32 = ((iSlow61 != 0) as i32) as u32 as F32;
		let mut fTemp356: F32 = self.fTempPerm336;
		let mut fSlow283: F32 = ((iSlow64 != 0) as i32) as u32 as F32;
		let mut fTemp357: F32 = self.fTempPerm337;
		let mut fTemp358: F32 = self.fTempPerm338;
		let mut fSlow284: F32 = ((iSlow67 != 0) as i32) as u32 as F32;
		let mut fTemp359: F32 = self.fTempPerm339;
		let mut fSlow285: F32 = ((iSlow70 != 0) as i32) as u32 as F32;
		let mut fTemp360: F32 = self.fTempPerm340;
		let mut fSlow286: F32 = ((iSlow40 != 0) as i32) as u32 as F32;
		let mut fTemp361: F32 = self.fTempPerm341;
		let mut fSlow287: F32 = ((iSlow1 != 0) as i32) as u32 as F32;
		let mut fSlow288: F32 = self.fHslider18;
		let mut fTemp366: F32 = self.fTempPerm342;
		let mut fTemp367: F32 = self.fTempPerm343;
		let mut iTemp368: i32 = self.iTempPerm344;
		let mut fSlow289: F32 = self.fHslider19;
		let mut fTemp369: F32 = self.fTempPerm345;
		let mut fTemp370: F32 = self.fTempPerm346;
		let mut fSlow290: F32 = self.fHslider20;
		let mut fTemp371: F32 = self.fTempPerm347;
		let mut fTemp372: F32 = self.fTempPerm348;
		let mut fTemp373: F32 = self.fTempPerm349;
		let mut fTemp374: F32 = self.fTempPerm350;
		let mut fTemp375: F32 = self.fTempPerm351;
		let mut fTemp376: F32 = self.fTempPerm352;
		let mut fTemp377: F32 = self.fTempPerm353;
		let mut fTemp378: F32 = self.fTempPerm354;
		let mut fSlow291: F32 = self.fEntry14;
		let mut fSlow292: F32 = self.fEntry15;
		let mut fTemp379: F32 = self.fTempPerm355;
		let mut fTemp380: F32 = self.fTempPerm356;
		let mut fTemp381: F32 = self.fTempPerm357;
		let mut iTemp382: i32 = self.iTempPerm358;
		let mut fTemp383: F32 = self.fTempPerm359;
		let mut fTemp384: F32 = self.fTempPerm360;
		let mut fTemp385: F32 = self.fTempPerm361;
		let mut fTemp386: F32 = self.fTempPerm362;
		let mut fTemp387: F32 = self.fTempPerm363;
		let mut fTemp388: F32 = self.fTempPerm364;
		let mut fTemp389: F32 = self.fTempPerm365;
		let mut fTemp390: F32 = self.fTempPerm366;
		let mut fTemp391: F32 = self.fTempPerm367;
		let mut fTemp392: F32 = self.fTempPerm368;
		let mut fTemp393: F32 = self.fTempPerm369;
		let mut fTemp394: F32 = self.fTempPerm370;
		let mut fTemp395: F32 = self.fTempPerm371;
		let mut fTemp396: F32 = self.fTempPerm372;
		let mut fTemp397: F32 = self.fTempPerm373;
		let mut fTemp398: F32 = self.fTempPerm374;
		let mut fSlow293: F32 = ((iSlow139 != 0) as i32) as u32 as F32;
		let mut iSlow294: i32 = (fSlow291) as i32;
		let mut fTemp399: F32 = self.fTempPerm375;
		let mut fTemp400: F32 = self.fTempPerm376;
		let mut fTemp401: F32 = self.fTempPerm377;
		let mut fTemp402: F32 = self.fTempPerm378;
		let mut fTemp403: F32 = self.fTempPerm379;
		let mut fTemp404: F32 = self.fTempPerm380;
		let mut fTemp405: F32 = self.fTempPerm381;
		let mut fTemp406: F32 = self.fTempPerm382;
		let mut fTemp407: F32 = self.fTempPerm383;
		let mut fSlow295: F32 = ((iSlow145 != 0) as i32) as u32 as F32;
		let mut iTemp408: i32 = self.iTempPerm384;
		let mut iSlow296: i32 = (fSlow292) as i32;
		let mut fTemp409: F32 = self.fTempPerm385;
		let mut fTemp410: F32 = self.fTempPerm386;
		let mut fTemp411: F32 = self.fTempPerm387;
		let mut fTemp412: F32 = self.fTempPerm388;
		let mut fTemp413: F32 = self.fTempPerm389;
		let mut fTemp414: F32 = self.fTempPerm390;
		let mut fTemp415: F32 = self.fTempPerm391;
		let mut fTemp416: F32 = self.fTempPerm392;
		let mut fTemp417: F32 = self.fTempPerm393;
		let mut iSlow297: i32 = (fSlow292 >= 2.0) as i32;
		let mut iSlow298: i32 = (fSlow292 >= 1.0) as i32;
		let mut fTemp418: F32 = self.fTempPerm394;
		let mut iSlow299: i32 = (fSlow292 >= 3.0) as i32;
		let mut fSlow300: F32 = (if iSlow299 != 0 {0} else {2}) as F32;
		let mut fSlow301: F32 = if iSlow297 != 0 {fSlow300} else {if iSlow298 != 0 {4.0} else {6.2831855}};
		let mut fTemp419: F32 = self.fTempPerm395;
		let mut fTemp420: F32 = self.fTempPerm396;
		let mut fTemp421: F32 = self.fTempPerm397;
		let mut fTemp422: F32 = self.fTempPerm398;
		let mut fTemp423: F32 = self.fTempPerm399;
		let mut fTemp424: F32 = self.fTempPerm400;
		let mut fSlow302: F32 = ((iSlow148 != 0) as i32) as u32 as F32;
		let mut fTemp425: F32 = self.fTempPerm401;
		let mut fTemp426: F32 = self.fTempPerm402;
		let mut fTemp427: F32 = self.fTempPerm403;
		let mut fTemp428: F32 = self.fTempPerm404;
		let mut fTemp429: F32 = self.fTempPerm405;
		let mut iTemp430: i32 = self.iTempPerm406;
		let mut fTemp431: F32 = self.fTempPerm407;
		let mut fTemp432: F32 = self.fTempPerm408;
		let mut fTemp433: F32 = self.fTempPerm409;
		let mut fTemp434: F32 = self.fTempPerm410;
		let mut fTemp435: F32 = self.fTempPerm411;
		let mut iTemp436: i32 = self.iTempPerm412;
		let mut fTemp437: F32 = self.fTempPerm413;
		let mut fTemp438: F32 = self.fTempPerm414;
		let mut fTemp439: F32 = self.fTempPerm415;
		let mut fTemp440: F32 = self.fTempPerm416;
		let mut fTemp441: F32 = self.fTempPerm417;
		let mut fTemp442: F32 = self.fTempPerm418;
		let mut fSlow303: F32 = ((iSlow151 != 0) as i32) as u32 as F32;
		let mut fTemp443: F32 = self.fTempPerm419;
		let mut iTemp444: i32 = self.iTempPerm420;
		let mut iTemp445: i32 = self.iTempPerm421;
		let mut iTemp446: i32 = self.iTempPerm422;
		let mut iTemp447: i32 = self.iTempPerm423;
		let mut iTemp448: i32 = self.iTempPerm424;
		let mut iTemp449: i32 = self.iTempPerm425;
		let mut fTemp450: F32 = self.fTempPerm426;
		let mut fTemp451: F32 = self.fTempPerm427;
		let mut fTemp452: F32 = self.fTempPerm428;
		let mut iTemp453: i32 = self.iTempPerm429;
		let mut iTemp454: i32 = self.iTempPerm430;
		let mut iTemp455: i32 = self.iTempPerm431;
		let mut iTemp456: i32 = self.iTempPerm432;
		let mut fTemp457: F32 = self.fTempPerm433;
		let mut fTemp458: F32 = self.fTempPerm434;
		let mut fTemp459: F32 = self.fTempPerm435;
		let mut fTemp460: F32 = self.fTempPerm436;
		let mut fTemp461: F32 = self.fTempPerm437;
		let mut fTemp462: F32 = self.fTempPerm438;
		let mut fTemp463: F32 = self.fTempPerm439;
		let mut fTemp464: F32 = self.fTempPerm440;
		let mut fTemp465: F32 = self.fTempPerm441;
		let mut fSlow304: F32 = ((iSlow154 != 0) as i32) as u32 as F32;
		let mut fTemp466: F32 = self.fTempPerm442;
		let mut fTemp467: F32 = self.fTempPerm443;
		let mut fTemp468: F32 = self.fTempPerm444;
		let mut fTemp469: F32 = self.fTempPerm445;
		let mut fTemp470: F32 = self.fTempPerm446;
		let mut fTemp471: F32 = self.fTempPerm447;
		let mut fSlow305: F32 = ((iSlow157 != 0) as i32) as u32 as F32;
		let mut iTemp472: i32 = self.iTempPerm448;
		let mut fTemp473: F32 = self.fTempPerm449;
		let mut fTemp474: F32 = self.fTempPerm450;
		let mut fTemp475: F32 = self.fTempPerm451;
		let mut fTemp476: F32 = self.fTempPerm452;
		let mut fTemp477: F32 = self.fTempPerm453;
		let mut fTemp478: F32 = self.fTempPerm454;
		let mut fTemp479: F32 = self.fTempPerm455;
		let mut fTemp480: F32 = self.fTempPerm456;
		let mut fTemp481: F32 = self.fTempPerm457;
		let mut fTemp482: F32 = self.fTempPerm458;
		let mut fTemp483: F32 = self.fTempPerm459;
		let mut fSlow306: F32 = ((iSlow160 != 0) as i32) as u32 as F32;
		let mut fTemp484: F32 = self.fTempPerm460;
		let mut fTemp485: F32 = self.fTempPerm461;
		let mut fTemp486: F32 = self.fTempPerm462;
		let mut iTemp487: i32 = self.iTempPerm463;
		let mut iSlow307: i32 = (fSlow291 >= 2.0) as i32;
		let mut iSlow308: i32 = (fSlow291 >= 3.0) as i32;
		let mut iSlow309: i32 = if iSlow307 != 0 {if iSlow308 != 0 {-2} else {0}} else {0};
		let mut iSlow310: i32 = (fSlow291 >= 1.0) as i32;
		let mut fSlow311: F32 = if iSlow307 != 0 {if iSlow308 != 0 {0.5} else {0.0}} else {if iSlow310 != 0 {0.75} else {0.0}};
		let mut fTemp488: F32 = self.fTempPerm464;
		let mut fTemp489: F32 = self.fTempPerm465;
		let mut fTemp490: F32 = self.fTempPerm466;
		let mut fTemp491: F32 = self.fTempPerm467;
		let mut iTemp492: i32 = self.iTempPerm468;
		let mut iSlow312: i32 = if iSlow307 != 0 {if iSlow308 != 0 {2} else {-2}} else {0};
		let mut fSlow313: F32 = if iSlow307 != 0 {if iSlow308 != 0 {0.0} else {0.5}} else {if iSlow310 != 0 {0.25} else {0.0}};
		let mut fTemp493: F32 = self.fTempPerm469;
		let mut iTemp494: i32 = self.iTempPerm470;
		let mut fTemp495: F32 = self.fTempPerm471;
		let mut fTemp496: F32 = self.fTempPerm472;
		let mut fTemp497: F32 = self.fTempPerm473;
		let mut fTemp498: F32 = self.fTempPerm474;
		let mut fTemp499: F32 = self.fTempPerm475;
		let mut iTemp500: i32 = self.iTempPerm476;
		let mut iSlow314: i32 = if iSlow297 != 0 {if iSlow299 != 0 {-2} else {0}} else {0};
		let mut fTemp501: F32 = self.fTempPerm477;
		let mut fTemp502: F32 = self.fTempPerm478;
		let mut fSlow315: F32 = if iSlow297 != 0 {if iSlow299 != 0 {0.5} else {0.0}} else {if iSlow298 != 0 {0.75} else {0.0}};
		let mut fTemp503: F32 = self.fTempPerm479;
		let mut fTemp504: F32 = self.fTempPerm480;
		let mut fTemp505: F32 = self.fTempPerm481;
		let mut fTemp506: F32 = self.fTempPerm482;
		let mut fSlow316: F32 = (if iSlow297 != 0 {0} else {if iSlow298 != 0 {8} else {0}}) as F32;
		let mut fTemp507: F32 = self.fTempPerm483;
		let mut iSlow317: i32 = if iSlow297 != 0 {if iSlow299 != 0 {2} else {-2}} else {0};
		let mut fTemp508: F32 = self.fTempPerm484;
		let mut fSlow318: F32 = if iSlow297 != 0 {if iSlow299 != 0 {0.0} else {0.5}} else {if iSlow298 != 0 {0.25} else {0.0}};
		let mut fTemp509: F32 = self.fTempPerm485;
		let mut fTemp510: F32 = self.fTempPerm486;
		let mut fTemp511: F32 = self.fTempPerm487;
		let mut fSlow319: F32 = (if iSlow297 != 0 {0} else {if iSlow298 != 0 {-8} else {0}}) as F32;
		let mut fTemp512: F32 = self.fTempPerm488;
		let mut fTemp513: F32 = self.fTempPerm489;
		let mut fTemp514: F32 = self.fTempPerm490;
		let mut fTemp515: F32 = self.fTempPerm491;
		let mut fSlow320: F32 = ((iSlow163 != 0) as i32) as u32 as F32;
		let mut fTemp516: F32 = self.fTempPerm492;
		let mut fTemp517: F32 = self.fTempPerm493;
		let mut fTemp518: F32 = self.fTempPerm494;
		let mut fSlow321: F32 = ((iSlow166 != 0) as i32) as u32 as F32;
		let mut fTemp519: F32 = self.fTempPerm495;
		let mut fTemp520: F32 = self.fTempPerm496;
		let mut fTemp521: F32 = self.fTempPerm497;
		let mut iTemp522: i32 = self.iTempPerm498;
		let mut fTemp523: F32 = self.fTempPerm499;
		let mut iTemp524: i32 = self.iTempPerm500;
		let mut fTemp525: F32 = self.fTempPerm501;
		let mut iTemp526: i32 = self.iTempPerm502;
		let mut fTemp527: F32 = self.fTempPerm503;
		let mut iTemp528: i32 = self.iTempPerm504;
		let mut fTemp529: F32 = self.fTempPerm505;
		let mut fTemp530: F32 = self.fTempPerm506;
		let mut fTemp531: F32 = self.fTempPerm507;
		let mut fTemp532: F32 = self.fTempPerm508;
		let mut iTemp533: i32 = self.iTempPerm509;
		let mut fTemp534: F32 = self.fTempPerm510;
		let mut fTemp535: F32 = self.fTempPerm511;
		let mut fTemp536: F32 = self.fTempPerm512;
		let mut fTemp537: F32 = self.fTempPerm513;
		let mut fTemp538: F32 = self.fTempPerm514;
		let mut fTemp539: F32 = self.fTempPerm515;
		let mut fTemp540: F32 = self.fTempPerm516;
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
		let mut fSlow322: F32 = ((iSlow169 != 0) as i32) as u32 as F32;
		let mut iTemp551: i32 = self.iTempPerm527;
		let mut fTemp552: F32 = self.fTempPerm528;
		let mut fTemp553: F32 = self.fTempPerm529;
		let mut fTemp554: F32 = self.fTempPerm530;
		let mut fTemp555: F32 = self.fTempPerm531;
		let mut fTemp556: F32 = self.fTempPerm532;
		let mut fTemp557: F32 = self.fTempPerm533;
		let mut fTemp558: F32 = self.fTempPerm534;
		let mut fTemp559: F32 = self.fTempPerm535;
		let mut fTemp560: F32 = self.fTempPerm536;
		let mut fTemp561: F32 = self.fTempPerm537;
		let mut fTemp562: F32 = self.fTempPerm538;
		let mut fTemp563: F32 = self.fTempPerm539;
		let mut fTemp564: F32 = self.fTempPerm540;
		let mut fTemp565: F32 = self.fTempPerm541;
		let mut fTemp566: F32 = self.fTempPerm542;
		let mut fTemp567: F32 = self.fTempPerm543;
		let mut fTemp568: F32 = self.fTempPerm544;
		let mut iTemp569: i32 = self.iTempPerm545;
		let mut fTemp570: F32 = self.fTempPerm546;
		let mut fTemp571: F32 = self.fTempPerm547;
		let mut fSlow323: F32 = ((iSlow172 != 0) as i32) as u32 as F32;
		let mut fTemp572: F32 = self.fTempPerm548;
		let mut fTemp573: F32 = self.fTempPerm549;
		let mut iTemp574: i32 = self.iTempPerm550;
		let mut fTemp575: F32 = self.fTempPerm551;
		let mut iTemp576: i32 = self.iTempPerm552;
		let mut fTemp577: F32 = self.fTempPerm553;
		let mut fTemp578: F32 = self.fTempPerm554;
		let mut fTemp579: F32 = self.fTempPerm555;
		let mut fTemp580: F32 = self.fTempPerm556;
		let mut fTemp581: F32 = self.fTempPerm557;
		let mut fTemp582: F32 = self.fTempPerm558;
		let mut fTemp583: F32 = self.fTempPerm559;
		let mut fSlow324: F32 = ((iSlow175 != 0) as i32) as u32 as F32;
		let mut fSlow325: F32 = unsafe { ftbl4OscDspSIG4[((fSlow138) as i32) as usize] };
		let mut fTemp584: F32 = self.fTempPerm560;
		let mut fSlow326: F32 = ((iSlow142 != 0) as i32) as u32 as F32;
		let mut fSlow327: F32 = self.fHslider21;
		let mut fTemp585: F32 = self.fTempPerm561;
		let mut fTemp586: F32 = self.fTempPerm562;
		let mut iTemp587: i32 = self.iTempPerm563;
		let mut fTemp588: F32 = self.fTempPerm564;
		let mut iTemp589: i32 = self.iTempPerm565;
		let mut fTemp590: F32 = self.fTempPerm566;
		let mut iTemp591: i32 = self.iTempPerm567;
		let mut iTemp592: i32 = self.iTempPerm568;
		let mut iTemp593: i32 = self.iTempPerm569;
		let mut iTemp594: i32 = self.iTempPerm570;
		let mut iTemp595: i32 = self.iTempPerm571;
		let mut iTemp596: i32 = self.iTempPerm572;
		let mut fTemp597: F32 = self.fTempPerm573;
		let mut iTemp598: i32 = self.iTempPerm574;
		let mut iTemp599: i32 = self.iTempPerm575;
		let mut iTemp600: i32 = self.iTempPerm576;
		let mut iSlow328: i32 = i32::wrapping_mul(2176, (self.fEntry16) as i32);
		let mut fTemp601: F32 = self.fTempPerm577;
		let mut fTemp602: F32 = self.fTempPerm578;
		let mut fTemp603: F32 = self.fTempPerm579;
		let mut fTemp604: F32 = self.fTempPerm580;
		let mut fTemp605: F32 = self.fTempPerm581;
		let mut fSlow329: F32 = self.fHslider22;
		let mut fTemp606: F32 = self.fTempPerm582;
		let mut fTemp607: F32 = self.fTempPerm583;
		let mut iTemp608: i32 = self.iTempPerm584;
		let mut iTemp609: i32 = self.iTempPerm585;
		let mut iTemp610: i32 = self.iTempPerm586;
		let mut fSlow330: F32 = self.fEntry17;
		let mut fSlow331: F32 = self.fEntry18;
		let mut fSlow332: F32 = self.fEntry19;
		let mut iTemp611: i32 = self.iTempPerm587;
		let mut fTemp612: F32 = self.fTempPerm588;
		let mut fTemp613: F32 = self.fTempPerm589;
		let mut fSlow333: F32 = self.fEntry20;
		let mut iTemp614: i32 = self.iTempPerm590;
		let mut fTemp615: F32 = self.fTempPerm591;
		let mut fTemp616: F32 = self.fTempPerm592;
		let mut fTemp617: F32 = self.fTempPerm593;
		let mut fSlow334: F32 = self.fHslider23;
		let mut fTemp618: F32 = self.fTempPerm594;
		let mut fTemp619: F32 = self.fTempPerm595;
		let mut fTemp620: F32 = self.fTempPerm596;
		let mut fTemp621: F32 = self.fTempPerm597;
		let mut fSlow335: F32 = ((iSlow178 != 0) as i32) as u32 as F32;
		let mut fTemp622: F32 = self.fTempPerm598;
		let mut iTemp623: i32 = self.iTempPerm599;
		let mut iTemp624: i32 = self.iTempPerm600;
		let mut iTemp625: i32 = self.iTempPerm601;
		let mut iTemp626: i32 = self.iTempPerm602;
		let mut iTemp627: i32 = self.iTempPerm603;
		let mut iTemp628: i32 = self.iTempPerm604;
		let mut fTemp629: F32 = self.fTempPerm605;
		let mut fTemp630: F32 = self.fTempPerm606;
		let mut iTemp631: i32 = self.iTempPerm607;
		let mut iTemp632: i32 = self.iTempPerm608;
		let mut iTemp633: i32 = self.iTempPerm609;
		let mut fTemp634: F32 = self.fTempPerm610;
		let mut fTemp635: F32 = self.fTempPerm611;
		let mut fTemp636: F32 = self.fTempPerm612;
		let mut fTemp637: F32 = self.fTempPerm613;
		let mut iTemp638: i32 = self.iTempPerm614;
		let mut iTemp639: i32 = self.iTempPerm615;
		let mut fTemp640: F32 = self.fTempPerm616;
		let mut fSlow336: F32 = ((iSlow181 != 0) as i32) as u32 as F32;
		let mut fTemp641: F32 = self.fTempPerm617;
		let mut fSlow337: F32 = ((iSlow184 != 0) as i32) as u32 as F32;
		let mut fTemp642: F32 = self.fTempPerm618;
		let mut fTemp643: F32 = self.fTempPerm619;
		let mut iTemp644: i32 = self.iTempPerm620;
		let mut iTemp645: i32 = self.iTempPerm621;
		let mut iTemp646: i32 = self.iTempPerm622;
		let mut iTemp647: i32 = self.iTempPerm623;
		let mut iTemp648: i32 = self.iTempPerm624;
		let mut iTemp649: i32 = self.iTempPerm625;
		let mut fTemp650: F32 = self.fTempPerm626;
		let mut iTemp651: i32 = self.iTempPerm627;
		let mut iTemp652: i32 = self.iTempPerm628;
		let mut iTemp653: i32 = self.iTempPerm629;
		let mut fTemp654: F32 = self.fTempPerm630;
		let mut fTemp655: F32 = self.fTempPerm631;
		let mut fTemp656: F32 = self.fTempPerm632;
		let mut fTemp657: F32 = self.fTempPerm633;
		let mut fTemp658: F32 = self.fTempPerm634;
		let mut fSlow338: F32 = ((iSlow187 != 0) as i32) as u32 as F32;
		let mut fTemp659: F32 = self.fTempPerm635;
		let mut fSlow339: F32 = ((iSlow190 != 0) as i32) as u32 as F32;
		let mut fTemp660: F32 = self.fTempPerm636;
		let mut fTemp661: F32 = self.fTempPerm637;
		let mut fSlow340: F32 = ((iSlow193 != 0) as i32) as u32 as F32;
		let mut fTemp662: F32 = self.fTempPerm638;
		let mut iTemp663: i32 = self.iTempPerm639;
		let mut iTemp664: i32 = self.iTempPerm640;
		let mut fTemp665: F32 = self.fTempPerm641;
		let mut fTemp666: F32 = self.fTempPerm642;
		let mut fTemp667: F32 = self.fTempPerm643;
		let mut fTemp668: F32 = self.fTempPerm644;
		let mut fTemp669: F32 = self.fTempPerm645;
		let mut fSlow341: F32 = ((iSlow196 != 0) as i32) as u32 as F32;
		let mut fSlow342: F32 = F32::max(0.0, F32::min(2.0, fSlow92 + -5.0));
		let mut fSlow343: F32 = self.fConst24 * (if (fSlow342 >= 2.0) as i32 != 0 {800} else {if (fSlow342 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp670: F32 = self.fTempPerm646;
		let mut fTemp671: F32 = self.fTempPerm647;
		let mut fTemp672: F32 = self.fTempPerm648;
		let mut fTemp673: F32 = self.fTempPerm649;
		let mut fTemp674: F32 = self.fTempPerm650;
		let mut fSlow344: F32 = ((iSlow199 != 0) as i32) as u32 as F32;
		let mut fTemp675: F32 = self.fTempPerm651;
		let mut fSlow345: F32 = ((iSlow202 != 0) as i32) as u32 as F32;
		let mut fTemp676: F32 = self.fTempPerm652;
		let mut fSlow346: F32 = ((iSlow205 != 0) as i32) as u32 as F32;
		let mut fSlow347: F32 = F32::max(0.0, F32::min(2.0, fSlow92 + -2.0));
		let mut fSlow348: F32 = self.fConst2 * (if (fSlow347 >= 2.0) as i32 != 0 {4000} else {if (fSlow347 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp677: F32 = self.fTempPerm653;
		let mut fTemp678: F32 = self.fTempPerm654;
		let mut fSlow349: F32 = ((iSlow93 != 0) as i32) as u32 as F32;
		let mut fTemp679: F32 = self.fTempPerm655;
		let mut fSlow350: F32 = ((iSlow103 != 0) as i32) as u32 as F32;
		let mut fTemp680: F32 = self.fTempPerm656;
		let mut fSlow351: F32 = ((iSlow106 != 0) as i32) as u32 as F32;
		let mut fTemp681: F32 = self.fTempPerm657;
		let mut fTemp682: F32 = self.fTempPerm658;
		let mut fSlow352: F32 = ((iSlow208 != 0) as i32) as u32 as F32;
		let mut fTemp683: F32 = self.fTempPerm659;
		let mut fSlow353: F32 = ((iSlow211 != 0) as i32) as u32 as F32;
		let mut fTemp684: F32 = self.fTempPerm660;
		let mut fSlow354: F32 = ((iSlow97 != 0) as i32) as u32 as F32;
		let mut fTemp685: F32 = self.fTempPerm661;
		let mut fSlow355: F32 = ((iSlow100 != 0) as i32) as u32 as F32;
		let mut fSlow356: F32 = self.fHslider24;
		let mut fSlow357: F32 = self.fHslider25;
		let mut fSlow358: F32 = self.fEntry47 + self.fEntry46 + self.fCheckbox17 + self.fHslider97 + self.fHslider96 + self.fHslider95 + self.fHslider94 + self.fHslider93 + self.fHslider92 + self.fHslider91 + self.fHslider90 + self.fEntry45 + self.fEntry44 + self.fCheckbox16 + self.fHslider89 + self.fHslider88 + self.fHslider87 + self.fHslider86 + self.fHslider85 + self.fHslider84 + self.fHslider83 + self.fHslider82 + self.fEntry43 + self.fEntry42 + self.fHslider81 + self.fHslider80 + self.fHslider79 + self.fHslider78 + self.fHslider77 + self.fHslider76 + self.fHslider75 + self.fEntry41 + self.fEntry40 + self.fHslider74 + self.fHslider73 + self.fHslider72 + self.fHslider71 + self.fHslider70 + self.fHslider69 + self.fHslider68 + self.fEntry39 + self.fCheckbox15 + self.fEntry38 + self.fHslider67 + self.fHslider66 + self.fHslider65 + self.fHslider64 + self.fCheckbox14 + self.fEntry37 + self.fHslider63 + self.fHslider62 + self.fHslider61 + self.fHslider60 + self.fCheckbox13 + self.fEntry36 + self.fHslider59 + self.fHslider58 + self.fHslider57 + self.fHslider56 + self.fEntry35 + self.fCheckbox12 + self.fEntry34 + self.fHslider55 + self.fHslider54 + self.fHslider53 + self.fHslider52 + self.fCheckbox11 + self.fEntry33 + self.fHslider51 + self.fHslider50 + self.fHslider49 + self.fHslider48 + self.fCheckbox10 + self.fEntry32 + self.fHslider47 + self.fHslider46 + self.fHslider45 + self.fHslider44 + self.fHslider43 + self.fEntry31 + self.fHslider42 + self.fHslider41 + self.fHslider40 + self.fHslider39 + self.fEntry30 + self.fCheckbox9 + self.fVslider11 + self.fVslider10 + self.fVslider9 + self.fVslider8 + self.fHslider38 + self.fHslider37 + self.fVslider7 + self.fVslider6 + self.fVslider5 + self.fVslider4 + self.fHslider36 + self.fHslider35 + self.fVslider3 + self.fVslider2 + self.fVslider1 + self.fVslider0 + self.fHslider34 + self.fHslider33 + self.fEntry29 + self.fHslider32 + self.fCheckbox8 + self.fEntry28 + self.fCheckbox7 + self.fEntry27 + self.fHslider31 + self.fCheckbox6 + self.fEntry26 + self.fCheckbox5 + self.fEntry25 + self.fHslider30 + self.fCheckbox4 + self.fEntry24 + self.fCheckbox3 + self.fHslider29 + self.fEntry23 + self.fHslider28 + self.fCheckbox2 + self.fEntry22 + self.fHslider27 + self.fCheckbox1 + self.fHslider26 + self.fEntry21;
		let zipped_iterators = outputs0.zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5);
		for (((((output0, output1), output2), output3), output4), output5) in zipped_iterators {
			self.fVec0[0] = fSlow88;
			self.iVec1[0] = 1;
			let mut iTemp0: i32 = (fSlow88 > self.fVec0[1]) as i32;
			self.fVec2[0] = fSlow89;
			let mut fTemp1: F32 = if ((fSlow89 != self.fVec2[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec1[1] + -1.0};
			self.fRec1[0] = fTemp1;
			self.fRec2[0] = if iTemp0 != 0 {fSlow89} else {if (fTemp1 > 0.0) as i32 != 0 {self.fRec2[1] + (fSlow89 - self.fRec2[1]) / F32::max(1.0, fTemp1)} else {fSlow89}};
			let mut fTemp2: F32 = F32::abs(self.fRec2[0]);
			let mut fTemp3: F32 = F32::exp(0.057762265 * (((self.fRec2[0] > 0.0) as i32) - ((self.fRec2[0] < 0.0) as i32)) as F32 * if (fTemp2 < 0.4) as i32 != 0 {F32::min(0.99, 2.5 * fTemp2)} else {if (fTemp2 < 0.82) as i32 != 0 {F32::min(6.0, F32::floor(16.666666 * (fTemp2 + -0.4))) + 1.0} else {(if (fTemp2 < 0.91) as i32 != 0 {12} else {24}) as F32}});
			self.fVec3[0] = fSlow91;
			let mut fTemp4: F32 = if ((fSlow91 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec3[1] + -1.0};
			self.fRec3[0] = fTemp4;
			self.fRec4[0] = if iTemp0 != 0 {fSlow91} else {if (fTemp4 > 0.0) as i32 != 0 {self.fRec4[1] + (fSlow91 - self.fRec4[1]) / F32::max(1.0, fTemp4)} else {fSlow91}};
			let mut iTemp5: i32 = (F32::abs(self.fRec4[0]) > 0.0001) as i32;
			let mut fTemp6: F32 = (iTemp5) as F32;
			if ((fTemp6 != 0.0) as i32) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | (iSlow102 & (iSlow99 & iSlow95))))))) != 0 {
				iTemp7 = i32::wrapping_sub(1, self.iVec1[1]);
			}
				if (fTemp6 != 0.0) as i32 != 0 {
					fTemp8 = if iTemp7 != 0 {0.0} else {self.fConst1 + self.fRec6[1]};
					self.fRec6[0] = fTemp8 - F32::floor(fTemp8);
					fTemp9 = if iTemp7 != 0 {0.0} else {fSlow114 + self.fRec7[1]};
					self.fRec7[0] = fTemp9 - F32::floor(fTemp9);
					self.iRec9[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec9[1], i32::wrapping_mul(iSlow115, iTemp7))), 12345);
					iTemp10 = i32::wrapping_mul(1664525, self.iRec9[0] ^ (self.iRec9[0] >> 15));
					self.fRec8[0] = self.fConst3 * self.fRec8[1] + self.fConst5 * (i32::wrapping_mul(1013904223, iTemp10 ^ (iTemp10 >> 13))) as F32;
				}
					let mut fTemp11: F32 = (i32::wrapping_sub(1, iTemp0)) as F32;
					let mut fTemp12: F32 = 1.0 - self.fConst6 * fTemp11;
					if (fTemp6 != 0.0) as i32 != 0 {
						self.fRec10[0] = fSlow116 * fTemp12 + self.fConst6 * fTemp11 * self.fRec10[1];
						fTemp13 = if iTemp7 != 0 {0.0} else {fSlow121 + self.fRec13[1]};
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
							self.fRec11[0] = fSlow122 * (3.0517578e-05 * (fTemp16 + self.fVec6[1]) - fSlow120 * self.fRec11[1]);
							fTemp17 = fTemp6 * (self.fRec4[0] * (fSlow125 + 2e+01 * self.fRec11[0] + self.fConst7 * self.fRec10[0] * self.fRec8[0]) + OscDsp_faustpower2_f(self.fRec4[0]) * (18.0 * unsafe { ftbl0OscDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec7[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl0OscDspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec6[0]) as i32, 65535))) as usize] }));
						}
							self.fRec15[0] = fSlow126 * fTemp12 + self.fConst6 * fTemp11 * self.fRec15[1];
							self.fVec7[0] = fSlow129;
							let mut fTemp18: F32 = if ((fSlow129 != self.fVec7[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec16[1] + -1.0};
							self.fRec16[0] = fTemp18;
							self.fRec17[0] = if iTemp0 != 0 {fSlow129} else {if (fTemp18 > 0.0) as i32 != 0 {self.fRec17[1] + (fSlow129 - self.fRec17[1]) / F32::max(1.0, fTemp18)} else {fSlow129}};
							let mut fTemp19: F32 = F32::exp(0.057762265 * (self.fRec17[0] + fSlow128 * self.fRec15[0] + 0.01 * ((iTemp5 != 0) as i32) as u32 as F32 * fTemp17));
							self.fVec8[0] = fSlow130;
							let mut fTemp20: F32 = if ((fSlow130 != self.fVec8[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec18[1] + -1.0};
							self.fRec18[0] = fTemp20;
							self.fRec19[0] = if iTemp0 != 0 {fSlow130} else {if (fTemp20 > 0.0) as i32 != 0 {self.fRec19[1] + (fSlow130 - self.fRec19[1]) / F32::max(1.0, fTemp20)} else {fSlow130}};
							self.fRec20[0] = fSlow134 + fSlow133 * self.fRec20[1];
							let mut fTemp21: F32 = F32::exp(self.fRec20[0]) * self.fRec19[0];
							let mut fTemp22: F32 = fTemp21 * fTemp19;
							let mut fTemp23: F32 = fTemp22 * fTemp3;
							if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | ((iSlow3 & (iSlow42 & iSlow39)) | ((iSlow3 & (iSlow7 & iSlow35)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | ((iSlow3 & (iSlow7 & iSlow17)) | ((iSlow3 & (iSlow7 & iSlow14)) | (iSlow3 & (iSlow7 & iSlow11))))))))))))))))))))))))) != 0 {
								fTemp24 = self.fConst2 * fTemp23;
								fTemp25 = fTemp24 + self.fRec0[1];
							}
								self.fRec21[0] = if iTemp0 != 0 {fTemp15} else {self.fRec21[1]};
								let mut fTemp26: F32 = 0.5 * (self.fRec21[0] + 1.0);
								let mut fTemp27: F32 = fSlow136 + (fTemp26 - F32::floor(fSlow136 + fTemp26));
								let mut fTemp28: F32 = fSlow137 * fTemp27;
								if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | ((iSlow3 & (iSlow42 & iSlow39)) | ((iSlow3 & (iSlow7 & iSlow35)) | ((iSlow3 & (iSlow7 & iSlow219)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow216)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | ((iSlow3 & (iSlow7 & iSlow17)) | ((iSlow3 & (iSlow7 & iSlow14)) | ((iSlow3 & (iSlow7 & iSlow11)) | ((iSlow102 & (iSlow99 & iSlow213)) | ((iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow189)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | ((iSlow102 & (iSlow99 & iSlow180)) | ((iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow174)) | ((iSlow102 & (iSlow144 & iSlow171)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow165)) | ((iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | ((iSlow102 & (iSlow144 & iSlow150)) | ((iSlow102 & (iSlow144 & iSlow147)) | (iSlow102 & (iSlow144 & iSlow141)))))))))))))))))))))))))))))))))))))))))))))))))))))) != 0 {
									fTemp29 = fTemp28 - F32::floor(fTemp28);
								}
									if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | ((iSlow3 & (iSlow42 & iSlow39)) | ((iSlow3 & (iSlow7 & iSlow35)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | ((iSlow3 & (iSlow7 & iSlow17)) | ((iSlow3 & (iSlow7 & iSlow14)) | (iSlow3 & (iSlow7 & iSlow11))))))))))))))))))))))))) != 0 {
										self.fRec0[0] = if iTemp0 != 0 {fTemp29} else {fTemp25 - F32::floor(fTemp25)};
									}
										if (iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow48)) | (iSlow3 & (iSlow7 & iSlow11))) != 0 {
											iTemp30 = (self.fRec0[0] < 0.5) as i32;
										}
											if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow7 & iSlow35)) | ((iSlow3 & (iSlow7 & iSlow219)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow216)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | ((iSlow3 & (iSlow7 & iSlow17)) | ((iSlow3 & (iSlow7 & iSlow14)) | (iSlow3 & (iSlow7 & iSlow11)))))))))))) != 0 {
												self.fVec9[0] = fSlow220;
												fTemp31 = if ((fSlow220 != self.fVec9[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec22[1] + -1.0};
												self.fRec22[0] = fTemp31;
												self.fRec23[0] = if iTemp0 != 0 {fSlow220} else {if (fTemp31 > 0.0) as i32 != 0 {self.fRec23[1] + (fSlow220 - self.fRec23[1]) / F32::max(1.0, fTemp31)} else {fSlow220}};
											}
												if (iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow17)) | (iSlow3 & (iSlow7 & iSlow11)))) != 0 {
													fTemp32 = 1.0 - self.fRec23[0];
												}
													if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow7 & iSlow35)) | ((iSlow3 & (iSlow7 & iSlow219)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow216)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | ((iSlow3 & (iSlow7 & iSlow17)) | ((iSlow3 & (iSlow7 & iSlow14)) | (iSlow3 & (iSlow7 & iSlow11)))))))))))) != 0 {
														self.fVec10[0] = fSlow221;
														fTemp33 = if ((fSlow221 != self.fVec10[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec24[1] + -1.0};
														self.fRec24[0] = fTemp33;
														self.fRec25[0] = if iTemp0 != 0 {fSlow221} else {if (fTemp33 > 0.0) as i32 != 0 {self.fRec25[1] + (fSlow221 - self.fRec25[1]) / F32::max(1.0, fTemp33)} else {fSlow221}};
													}
														if iSlow3 & (iSlow7 & iSlow11) != 0 {
															fTemp34 = 1.0 - self.fRec25[0];
															fTemp35 = if iTemp30 != 0 {fTemp34} else {fTemp32};
															fTemp36 = if (fTemp35 < 0.999) as i32 != 0 {F32::max(0.01, fTemp35)} else {1.0};
														}
															if (iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow7 & iSlow23)) | (iSlow3 & (iSlow7 & iSlow11))) != 0 {
																fTemp37 = 2.0 * self.fRec0[0];
															}
																if iSlow3 & (iSlow7 & iSlow11) != 0 {
																	fTemp38 = F32::floor(fTemp37);
																	fTemp39 = fTemp37 - (fTemp36 + fTemp38);
																	fTemp40 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp39 / fTemp23)));
																	fTemp41 = if iTemp30 != 0 {fSlow223} else {fSlow222};
																	fTemp42 = if (fTemp41 >= 2.0) as i32 != 0 {(if (fTemp41 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp41 >= 1.0) as i32 != 0 {4.0} else {6.2831855}};
																	fTemp43 = fTemp21 * fTemp42 * fTemp19 * fTemp3;
																	iTemp44 = (fTemp41) as i32;
																	fTemp45 = self.fConst0 * F32::abs(self.fConst11 * (fTemp23 / fTemp36));
																	fTemp46 = 0.5 * wt_basic(iTemp44, fTemp45, self.fConst0, 0.999999);
																	fTemp47 = self.fConst11 * fTemp23;
																	fTemp48 = fTemp37 - fTemp38;
																	fTemp49 = ((((fTemp36 < 1.0) as i32) & ((fTemp48 >= fTemp36) as i32)) & ((fTemp39 < fTemp47) as i32)) as F32;
																	fTemp50 = 1.0 - fTemp40;
																	fTemp51 = if iTemp30 != 0 {fSlow222} else {fSlow223};
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
																	fTemp60 = fSlow10 * (self.fVec12[1] + fTemp59 * OscDsp_faustpower2_f(fTemp56) * (fTemp55 + self.fConst10 * fTemp23 * fTemp56 * fTemp58) + self.fVec11[1] - fTemp49 * OscDsp_faustpower2_f(fTemp40) * (fTemp46 + self.fConst10 * (fTemp43 * fTemp40 / fTemp36)));
																}
																	if iSlow3 & (iSlow7 & iSlow14) != 0 {
																		fTemp61 = self.fConst12 * fTemp23 * F32::exp(0.0046209814 * self.fRec23[0]);
																		fTemp62 = fTemp61 + self.fRec26[1];
																	}
																		if (iSlow3 & (iSlow7 & iSlow14)) | (iSlow102 & (iSlow144 & iSlow147)) != 0 {
																			fTemp63 = fSlow226 * fTemp27;
																			fTemp64 = F32::floor(fTemp63);
																			fTemp65 = fTemp63 - (fTemp64 + F32::floor(fTemp63 - fTemp64));
																		}
																			if iSlow3 & (iSlow7 & iSlow14) != 0 {
																				self.fRec26[0] = if iTemp0 != 0 {fTemp65} else {fTemp62 - F32::floor(fTemp62)};
																			}
																				if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow7 & iSlow219)) | ((iSlow3 & (iSlow7 & iSlow26)) | (iSlow3 & (iSlow7 & iSlow14)))) != 0 {
																					fTemp66 = F32::max(0.0, F32::min(1.0, self.fRec25[0]));
																				}
																					if iSlow3 & (iSlow7 & iSlow14) != 0 {
																						fTemp67 = 4.0 * fTemp66;
																						fTemp68 = F32::min(3.0, fTemp67);
																					}
																						if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow7 & iSlow35)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | (iSlow3 & (iSlow7 & iSlow14)))))))) != 0 {
																							fTemp69 = F32::abs(fTemp24);
																						}
																							if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | (iSlow3 & (iSlow7 & iSlow14))))))) != 0 {
																								fTemp70 = self.fConst0 * fTemp69;
																							}
																								if iSlow3 & (iSlow7 & iSlow14) != 0 {
																									fTemp71 = wt_basic(2, fTemp70, self.fConst0, self.fRec0[0]);
																									self.fVec13[0] = wt_basic(0, fTemp70, self.fConst0, self.fRec0[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp68)) + wt_basic(1, fTemp70, self.fConst0, self.fRec0[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp68 + -1.0)) + fTemp71 * F32::max(0.0, 1.0 - F32::abs(fTemp68 + -2.0)) + (fTemp71 - wt_basic(2, fTemp70, self.fConst0, self.fRec0[0] + (-0.5 - 0.42 * F32::max(0.0, fTemp67 + -3.0)))) * F32::max(0.0, 1.0 - F32::abs(fTemp68 + -3.0)) + self.fRec23[0] * wt_basic(iSlow225, self.fConst0 * F32::abs(fTemp61), self.fConst0, self.fRec26[0]);
																									fTemp72 = fSlow13 * (self.fVec13[1] / F32::sqrt(OscDsp_faustpower2_f(self.fRec23[0]) + 1.0));
																								}
																									if (iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | (iSlow3 & (iSlow7 & iSlow17))) != 0 {
																										iTemp73 = (self.fRec0[0] < self.fRec0[1]) as i32;
																									}
																										if (iSlow3 & (iSlow7 & iSlow20)) | (iSlow3 & (iSlow7 & iSlow17)) != 0 {
																											self.iRec27[0] = (i32::wrapping_add(iTemp73, self.iRec27[1])) % 2;
																										}
																											if iSlow3 & (iSlow7 & iSlow17) != 0 {
																												self.fVec14[0] = wt_basic(iSlow225, self.fConst0 * F32::abs(self.fConst12 * fTemp23), self.fConst0, 0.5 * (self.fRec0[0] + (self.iRec27[0]) as F32));
																											}
																												if (iSlow3 & (iSlow7 & iSlow23)) | (iSlow3 & (iSlow7 & iSlow17)) != 0 {
																													fTemp74 = 15.0 * self.fRec25[0];
																													fTemp75 = fTemp74 + 1.0;
																													fTemp76 = fTemp23 * fTemp75;
																													fTemp77 = self.fConst0 * F32::abs(self.fConst2 * fTemp76);
																													fTemp78 = wt_basic(iSlow228, fTemp77, self.fConst0, fTemp75);
																													fTemp79 = wt_basic(iSlow228, fTemp77, self.fConst0, 0.0);
																												}
																													if iSlow3 & (iSlow7 & iSlow17) != 0 {
																														fTemp80 = 0.5 * (fTemp79 - fTemp78);
																													}
																														if (iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | (iSlow3 & (iSlow7 & iSlow17))) != 0 {
																															fTemp81 = F32::max(0.0, F32::min(1.0, self.fConst0 * (self.fRec0[0] / fTemp23)));
																															fTemp82 = 1.0 - fTemp81;
																														}
																															if iSlow3 & (iSlow7 & iSlow17) != 0 {
																																fTemp83 = fTemp74 + (1.0 - F32::floor(fTemp75));
																																fTemp84 = fSlow233 - if iSlow229 != 0 {fSlow232} else {if iSlow230 != 0 {(i32::wrapping_sub(4, i32::wrapping_mul(8, ((fTemp83 >= 0.25) as i32) & ((fTemp83 < 0.75) as i32)))) as F32} else {6.2831855 * F32::cos(6.2831855 * fTemp83)}};
																															}
																																if (iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | (iSlow3 & (iSlow7 & iSlow17))) != 0 {
																																	fTemp85 = (iTemp73) as F32;
																																	fTemp86 = fTemp85 * OscDsp_faustpower2_f(fTemp82);
																																}
																																	if iSlow3 & (iSlow7 & iSlow17) != 0 {
																																		self.fVec15[0] = fTemp86 * (self.fConst13 * fTemp76 * fTemp84 * fTemp82 - fTemp80);
																																	}
																																		if (iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | (iSlow3 & (iSlow7 & iSlow17))) != 0 {
																																			fTemp87 = fTemp85 * OscDsp_faustpower2_f(fTemp81);
																																		}
																																			if (iSlow3 & (iSlow7 & iSlow23)) | (iSlow3 & (iSlow7 & iSlow17)) != 0 {
																																				fTemp88 = wt_basic(iSlow228, fTemp77, self.fConst0, self.fRec0[0] * fTemp75);
																																				self.fVec16[0] = fTemp88;
																																			}
																																				if iSlow3 & (iSlow7 & iSlow17) != 0 {
																																					fTemp89 = fSlow16 * (fTemp32 * (self.fVec16[1] + fTemp87 * (fTemp80 + self.fConst13 * fTemp76 * fTemp81 * fTemp84) + self.fVec15[1]) + self.fRec23[0] * self.fVec14[1]);
																																				}
																																					if iSlow3 & (iSlow7 & iSlow20) != 0 {
																																						fTemp90 = if self.iRec27[0] != 0 {fSlow223} else {fSlow222};
																																						fTemp91 = 7.0 * self.fRec25[0] + 1.0;
																																						fTemp92 = fTemp91 * (self.fRec23[0] + 1.0);
																																						fTemp93 = if self.iRec27[0] != 0 {fTemp91} else {fTemp92};
																																						fTemp94 = if self.iRec27[0] != 0 {fSlow222} else {fSlow223};
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
																																						fTemp107 = fSlow19 * if iTemp101 != 0 {1.0 - 4.0 * F32::abs(fTemp106 + (-0.25 - F32::floor(fTemp106 + 0.25)))} else {(if (fTemp105 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp105)} else {1.0 - 0.5 * OscDsp_faustpower2_f(fTemp99 + (-2.0 - fTemp104))} - if (fTemp103 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp103)} else {1.0 - 0.5 * OscDsp_faustpower2_f(self.fVec19[1] + (-2.0 - fTemp102))}) / if iTemp101 != 0 {1.0} else {fTemp100}};
																																					}
																																						if iSlow3 & (iSlow7 & iSlow23) != 0 {
																																							fTemp108 = 4.0 * F32::max(0.0, F32::min(1.0, self.fRec23[0]));
																																							iTemp109 = (F32::min(3.0, fTemp108)) as i32;
																																							iTemp110 = i32::wrapping_add(iTemp109, 1);
																																							iTemp111 = (iTemp110 >= 3) as i32;
																																							iTemp112 = (iTemp110 >= 2) as i32;
																																							iTemp113 = (iTemp110 >= 1) as i32;
																																							iTemp114 = (iTemp110 >= 4) as i32;
																																							fTemp115 = 0.5 * (wt_basic(iSlow225, fTemp70, self.fConst0, 0.999999) + 1.0);
																																							fTemp116 = (iTemp109) as F32;
																																							fTemp117 = fTemp108 - fTemp116;
																																							iTemp118 = (iTemp109 >= 3) as i32;
																																							iTemp119 = (iTemp109 >= 2) as i32;
																																							iTemp120 = (iTemp109 >= 1) as i32;
																																							iTemp121 = (iTemp109 >= 4) as i32;
																																							fTemp122 = fTemp116 + (1.0 - fTemp108);
																																							fTemp123 = 0.5 * (wt_basic(iSlow225, fTemp70, self.fConst0, 0.0) + 1.0);
																																							fTemp124 = fTemp79 * (fTemp122 * if iTemp118 != 0 {if iTemp121 != 0 {fTemp123} else {0.0}} else {if iTemp119 != 0 {0.0} else {(if iTemp120 != 0 {0} else {1}) as F32}} + fTemp117 * if iTemp111 != 0 {if iTemp114 != 0 {fTemp123} else {0.0}} else {if iTemp112 != 0 {0.0} else {(if iTemp113 != 0 {0} else {1}) as F32}}) - fTemp78 * (fTemp122 * if iTemp118 != 0 {if iTemp121 != 0 {fTemp115} else {6e-06}} else {if iTemp119 != 0 {0.0} else {if iTemp120 != 0 {2e-06} else {1e-06}}} + fTemp117 * if iTemp111 != 0 {if iTemp114 != 0 {fTemp115} else {6e-06}} else {if iTemp112 != 0 {0.0} else {if iTemp113 != 0 {2e-06} else {1e-06}}});
																																							self.fVec20[0] = fTemp86 * fTemp124;
																																						}
																																							if (iSlow3 & (iSlow42 & iSlow75)) | (iSlow3 & (iSlow7 & iSlow23)) != 0 {
																																								fTemp125 = 1.0 - self.fRec0[0];
																																							}
																																								if iSlow3 & (iSlow7 & iSlow23) != 0 {
																																									fTemp126 = 1.0 - F32::abs(fTemp37 + -1.0);
																																									fTemp127 = F32::sin(3.1415927 * self.fRec0[0]);
																																									fTemp128 = F32::min(1.0, 6.0 * F32::min(self.fRec0[0], fTemp125));
																																									fTemp129 = 0.5 * (wt_basic(iSlow225, fTemp70, self.fConst0, self.fRec0[0]) + 1.0);
																																									self.fVec21[0] = fTemp88 * (fTemp122 * if iTemp118 != 0 {if iTemp121 != 0 {fTemp129} else {fTemp128}} else {if iTemp119 != 0 {fTemp127} else {if iTemp120 != 0 {fTemp126} else {fTemp125}}} + fTemp117 * if iTemp111 != 0 {if iTemp114 != 0 {fTemp129} else {fTemp128}} else {if iTemp112 != 0 {fTemp127} else {if iTemp113 != 0 {fTemp126} else {fTemp125}}});
																																									fTemp130 = fSlow22 * (self.fVec21[1] + 0.5 * (fTemp87 * fTemp124 - self.fVec20[1]));
																																								}
																																									if iSlow3 & (iSlow7 & iSlow26) != 0 {
																																										fTemp131 = self.fConst2 * fTemp23 * (2.17 * fTemp66 + 3.0);
																																										fTemp132 = fTemp131 + self.fRec28[1];
																																									}
																																										if (iSlow3 & (iSlow7 & iSlow26)) | (iSlow102 & (iSlow144 & iSlow159)) != 0 {
																																											fTemp133 = F32::floor(fTemp28 + 0.67);
																																											fTemp134 = fTemp28 + (0.67 - (fTemp133 + F32::floor(fTemp28 + (0.67 - fTemp133))));
																																										}
																																											if iSlow3 & (iSlow7 & iSlow26) != 0 {
																																												self.fRec28[0] = if iTemp0 != 0 {fTemp134} else {fTemp132 - F32::floor(fTemp132)};
																																												self.fVec22[0] = wt_basic(iSlow225, self.fConst0 * F32::abs(fTemp131), self.fConst0, self.fRec28[0]);
																																												fTemp135 = self.fConst2 * fTemp23 * (1.41 * fTemp66 + 2.0);
																																												fTemp136 = fTemp135 + self.fRec29[1];
																																											}
																																												if (iSlow3 & (iSlow7 & iSlow26)) | (iSlow102 & (iSlow144 & iSlow159)) != 0 {
																																													fTemp137 = F32::floor(fTemp28 + 0.31);
																																													fTemp138 = fTemp28 + (0.31 - (fTemp137 + F32::floor(fTemp28 + (0.31 - fTemp137))));
																																												}
																																													if iSlow3 & (iSlow7 & iSlow26) != 0 {
																																														self.fRec29[0] = if iTemp0 != 0 {fTemp138} else {fTemp136 - F32::floor(fTemp136)};
																																														self.fVec23[0] = wt_basic(iSlow225, self.fConst0 * F32::abs(fTemp135), self.fConst0, self.fRec29[0]);
																																													}
																																														if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow29)) | (iSlow3 & (iSlow7 & iSlow26)))) != 0 {
																																															fTemp139 = wt_basic(iSlow228, fTemp70, self.fConst0, self.fRec0[0]);
																																															self.fVec24[0] = fTemp139;
																																														}
																																															if iSlow3 & (iSlow7 & iSlow26) != 0 {
																																																fTemp140 = fSlow25 * (0.57735026 * fTemp32 * (self.fVec24[1] + self.fVec23[1] + self.fVec22[1]) + 1.5 * self.fRec23[0] * self.fVec24[1] * self.fVec23[1] * self.fVec22[1]);
																																															}
																																																if iSlow3 & (iSlow7 & iSlow29) != 0 {
																																																	iTemp141 = (F32::max(0.0, F32::min(1e+01, 10.999 * self.fRec25[0]))) as i32;
																																																	fTemp142 = unsafe { ftbl1OscDspSIG1[(i32::wrapping_add(iTemp141, 1)) as usize] };
																																																	fTemp143 = self.fConst2 * fTemp23 * fTemp142;
																																																	fTemp144 = fTemp143 + self.fRec30[1];
																																																	fTemp145 = fSlow137 * fTemp27 * fTemp142;
																																																	fTemp146 = F32::floor(fTemp145);
																																																	self.fRec30[0] = if iTemp0 != 0 {fTemp145 - (fTemp146 + F32::floor(fTemp145 - fTemp146))} else {fTemp144 - F32::floor(fTemp144)};
																																																	self.fVec25[0] = wt_basic(iSlow225, self.fConst0 * F32::abs(fTemp143), self.fConst0, self.fRec30[0]);
																																																	fTemp147 = unsafe { ftbl1OscDspSIG1[iTemp141 as usize] };
																																																	fTemp148 = self.fConst2 * fTemp23 * fTemp147;
																																																	fTemp149 = fTemp148 + self.fRec31[1];
																																																	fTemp150 = fSlow137 * fTemp27 * fTemp147;
																																																	fTemp151 = F32::floor(fTemp150);
																																																	self.fRec31[0] = if iTemp0 != 0 {fTemp150 - (fTemp151 + F32::floor(fTemp150 - fTemp151))} else {fTemp149 - F32::floor(fTemp149)};
																																																	self.fVec26[0] = wt_basic(iSlow225, self.fConst0 * F32::abs(fTemp148), self.fConst0, self.fRec31[0]);
																																																	fTemp152 = fSlow28 * self.fVec24[1] * (fTemp32 * self.fVec26[1] + self.fRec23[0] * self.fVec25[1]);
																																																}
																																																	if iSlow3 & (iSlow7 & iSlow216) != 0 {
																																																		fTemp153 = unsafe { ftbl2OscDspSIG2[((F32::max(0.0, F32::min(9.0, 9.999 * self.fRec23[0]))) as i32) as usize] };
																																																		fTemp154 = self.fConst2 * fTemp23 * fTemp153;
																																																		fTemp155 = F32::max(F32::abs(fTemp154), 1e-09);
																																																		iTemp156 = (fTemp154 < 0.0) as i32;
																																																		fTemp157 = fTemp154 + self.fRec32[1];
																																																		fTemp158 = fSlow137 * fTemp27 * fTemp153;
																																																		fTemp159 = F32::floor(fTemp158);
																																																		self.fRec32[0] = if iTemp0 != 0 {fTemp158 - (fTemp159 + F32::floor(fTemp158 - fTemp159))} else {fTemp157 - F32::floor(fTemp157)};
																																																		fTemp160 = if iTemp156 != 0 {fSlow243 - (self.fRec32[0] + F32::floor(fSlow243 - self.fRec32[0]))} else {self.fRec32[0] - (fSlow243 + F32::floor(self.fRec32[0] - fSlow243))};
																																																		iTemp161 = ((fTemp160 < fTemp155) as i32) * if iTemp156 != 0 {-iSlow241} else {iSlow241};
																																																		fTemp162 = if iTemp156 != 0 {fSlow245 - (self.fRec32[0] + F32::floor(fSlow245 - self.fRec32[0]))} else {self.fRec32[0] - (fSlow245 + F32::floor(self.fRec32[0] - fSlow245))};
																																																		iTemp163 = ((fTemp162 < fTemp155) as i32) * if iTemp156 != 0 {-iSlow244} else {iSlow244};
																																																		fTemp164 = F32::floor(self.fRec32[0]);
																																																		fTemp165 = self.fRec32[0] - fTemp164;
																																																		fTemp166 = if iSlow239 != 0 {if iSlow240 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp165 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec32[0] + (0.5 - (fTemp164 + F32::floor(self.fRec32[0] + (0.5 - fTemp164))))) + -1.0}} else {if iSlow242 != 0 {1.0 - 4.0 * F32::abs(self.fRec32[0] + (-0.25 - (fTemp164 + F32::floor(self.fRec32[0] + (0.25 - fTemp164)))))} else {F32::sin(6.2831855 * fTemp165)}};
																																																		self.fVec27[0] = fTemp166;
																																																	}
																																																		if (iSlow3 & (iSlow7 & iSlow32)) | (iSlow3 & (iSlow7 & iSlow216)) != 0 {
																																																			fTemp167 = OscDsp_faustpower2_f(self.fRec25[0]);
																																																		}
																																																			if iSlow3 & (iSlow7 & iSlow216) != 0 {
																																																				fTemp168 = self.fConst2 * fTemp23 * (8.0 * fTemp167 * fTemp153 * (0.5 * (fTemp166 + self.fVec27[1] - (i32::wrapping_add(iTemp163, iTemp161)) as F32) + (fTemp162 * (iTemp163) as F32 + fTemp160 * (iTemp161) as F32) / fTemp155) + 1.0);
																																																				iTemp169 = (fTemp168 < 0.0) as i32;
																																																				fTemp170 = 0.5 * (if iTemp169 != 0 {-iSlow246} else {iSlow246}) as F32;
																																																				fTemp171 = F32::max(F32::abs(fTemp168), 1e-09);
																																																				fTemp172 = self.fRec33[1] + fTemp168;
																																																				self.fRec33[0] = if iTemp0 != 0 {fTemp29} else {fTemp172 - F32::floor(fTemp172)};
																																																				fTemp173 = if iTemp169 != 0 {fSlow247 - (self.fRec33[0] + F32::floor(fSlow247 - self.fRec33[0]))} else {self.fRec33[0] - (fSlow247 + F32::floor(self.fRec33[0] - fSlow247))};
																																																				fTemp174 = F32::max(0.0, F32::min(1.0, fTemp173 / fTemp171));
																																																				fTemp175 = 1.0 - fTemp174;
																																																				fTemp176 = ((fTemp173 < fTemp171) as i32) as u32 as F32;
																																																				fTemp177 = 0.5 * (if iTemp169 != 0 {-iSlow249} else {iSlow249}) as F32;
																																																				fTemp178 = if iTemp169 != 0 {fSlow250 - (self.fRec33[0] + F32::floor(fSlow250 - self.fRec33[0]))} else {self.fRec33[0] - (fSlow250 + F32::floor(self.fRec33[0] - fSlow250))};
																																																				fTemp179 = F32::max(0.0, F32::min(1.0, fTemp178 / fTemp171));
																																																				fTemp180 = 1.0 - fTemp179;
																																																				fTemp181 = ((fTemp178 < fTemp171) as i32) as u32 as F32;
																																																				self.fVec28[0] = fTemp181 * OscDsp_faustpower2_f(fTemp180) * (0.16666667 * fTemp171 * fSlow251 * fTemp180 - fTemp177) + fTemp176 * OscDsp_faustpower2_f(fTemp175) * (0.16666667 * fTemp171 * fSlow248 * fTemp175 - fTemp170);
																																																				fTemp182 = F32::floor(self.fRec33[0]);
																																																				fTemp183 = self.fRec33[0] - fTemp182;
																																																				self.fVec29[0] = if iSlow229 != 0 {if iSlow231 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp183 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec33[0] + (0.5 - (fTemp182 + F32::floor(self.fRec33[0] + (0.5 - fTemp182))))) + -1.0}} else {if iSlow230 != 0 {1.0 - 4.0 * F32::abs(self.fRec33[0] + (-0.25 - (fTemp182 + F32::floor(self.fRec33[0] + (0.25 - fTemp182)))))} else {F32::sin(6.2831855 * fTemp183)}};
																																																				fTemp184 = fSlow215 * (fTemp176 * OscDsp_faustpower2_f(fTemp174) * (fTemp170 + 0.16666667 * fTemp171 * fTemp174 * fSlow248) + self.fVec29[1] + fTemp181 * OscDsp_faustpower2_f(fTemp179) * (fTemp177 + 0.16666667 * fTemp171 * fTemp179 * fSlow251) + self.fVec28[1]);
																																																			}
																																																				if iSlow3 & (iSlow7 & iSlow32) != 0 {
																																																					fTemp185 = F32::max(1e+01, F32::min(self.fConst14, fTemp23 * F32::exp(0.6931472 * (8.0 * self.fRec23[0] + -3.0))));
																																																					fTemp186 = 1.0 / F32::tan(self.fConst8 * fTemp185);
																																																				}
																																																					if (iSlow3 & (iSlow7 & iSlow32)) | (iSlow102 & (iSlow144 & iSlow168)) != 0 {
																																																						fTemp187 = 4.656613e-10 * (fTemp14 + self.fVec5[1]);
																																																					}
																																																						if iSlow3 & (iSlow7 & iSlow32) != 0 {
																																																							self.fRec34[0] = (fTemp187 - self.fRec34[1] * (1.0 - fTemp186)) / (fTemp186 + 1.0);
																																																							fTemp188 = fSlow31 * (self.fVec24[1] * (1.5 * self.fRec25[0] * self.fRec34[0] * F32::sqrt(self.fConst15 / fTemp185) + 1.0) / F32::sqrt(2.25 * fTemp167 + 1.0));
																																																						}
																																																							if iSlow3 & (iSlow7 & iSlow219) != 0 {
																																																								fTemp189 = 0.6 * fTemp66 * self.fRec36[1] + 1.0;
																																																								fTemp190 = self.fConst2 * fTemp23 * fTemp189;
																																																								fTemp191 = self.fRec35[1] + fTemp190;
																																																								iTemp192 = (fTemp191 >= 1.0) as i32;
																																																								self.fRec35[0] = if iTemp0 != 0 {fTemp29} else {fTemp191 - (iTemp192) as F32};
																																																								self.fRec36[0] = if iTemp192 != 0 {fTemp15} else {self.fRec36[1]};
																																																								fTemp193 = F32::max(F32::abs(fTemp190), 1e-09);
																																																								iTemp194 = (fTemp190 < 0.0) as i32;
																																																								fTemp195 = if iTemp194 != 0 {fSlow243 - (self.fRec35[0] + F32::floor(fSlow243 - self.fRec35[0]))} else {self.fRec35[0] - (fSlow243 + F32::floor(self.fRec35[0] - fSlow243))};
																																																								iTemp196 = ((fTemp195 < fTemp193) as i32) * if iTemp194 != 0 {-iSlow241} else {iSlow241};
																																																								fTemp197 = if iTemp194 != 0 {fSlow245 - (self.fRec35[0] + F32::floor(fSlow245 - self.fRec35[0]))} else {self.fRec35[0] - (fSlow245 + F32::floor(self.fRec35[0] - fSlow245))};
																																																								iTemp198 = ((fTemp197 < fTemp193) as i32) * if iTemp194 != 0 {-iSlow244} else {iSlow244};
																																																								fTemp199 = F32::floor(self.fRec35[0]);
																																																								fTemp200 = self.fRec35[0] - fTemp199;
																																																								fTemp201 = if iSlow239 != 0 {if iSlow240 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp200 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec35[0] + (0.5 - (fTemp199 + F32::floor(self.fRec35[0] + (0.5 - fTemp199))))) + -1.0}} else {if iSlow242 != 0 {1.0 - 4.0 * F32::abs(self.fRec35[0] + (-0.25 - (fTemp199 + F32::floor(self.fRec35[0] + (0.25 - fTemp199)))))} else {F32::sin(6.2831855 * fTemp200)}};
																																																								self.fVec30[0] = fTemp201;
																																																								fTemp202 = self.fConst2 * fTemp23 * (4.0 * self.fRec23[0] * fTemp189 * (0.5 * (fTemp201 + self.fVec30[1] - (i32::wrapping_add(iTemp198, iTemp196)) as F32) + (fTemp197 * (iTemp198) as F32 + fTemp195 * (iTemp196) as F32) / fTemp193) + 1.0);
																																																								iTemp203 = (fTemp202 < 0.0) as i32;
																																																								fTemp204 = 0.5 * (if iTemp203 != 0 {-iSlow246} else {iSlow246}) as F32;
																																																								fTemp205 = F32::max(F32::abs(fTemp202), 1e-09);
																																																								fTemp206 = self.fRec37[1] + fTemp202;
																																																								self.fRec37[0] = if iTemp0 != 0 {fTemp29} else {fTemp206 - F32::floor(fTemp206)};
																																																								fTemp207 = if iTemp203 != 0 {fSlow247 - (self.fRec37[0] + F32::floor(fSlow247 - self.fRec37[0]))} else {self.fRec37[0] - (fSlow247 + F32::floor(self.fRec37[0] - fSlow247))};
																																																								fTemp208 = F32::max(0.0, F32::min(1.0, fTemp207 / fTemp205));
																																																								fTemp209 = 1.0 - fTemp208;
																																																								fTemp210 = fSlow248 * fTemp205;
																																																								fTemp211 = ((fTemp207 < fTemp205) as i32) as u32 as F32;
																																																								fTemp212 = 0.5 * (if iTemp203 != 0 {-iSlow249} else {iSlow249}) as F32;
																																																								fTemp213 = if iTemp203 != 0 {fSlow250 - (self.fRec37[0] + F32::floor(fSlow250 - self.fRec37[0]))} else {self.fRec37[0] - (fSlow250 + F32::floor(self.fRec37[0] - fSlow250))};
																																																								fTemp214 = F32::max(0.0, F32::min(1.0, fTemp213 / fTemp205));
																																																								fTemp215 = 1.0 - fTemp214;
																																																								fTemp216 = fSlow251 * fTemp205;
																																																								fTemp217 = ((fTemp213 < fTemp205) as i32) as u32 as F32;
																																																								self.fVec31[0] = fTemp217 * OscDsp_faustpower2_f(fTemp215) * (0.16666667 * fTemp216 * fTemp215 - fTemp212) + fTemp211 * OscDsp_faustpower2_f(fTemp209) * (0.16666667 * fTemp210 * fTemp209 - fTemp204);
																																																								fTemp218 = F32::floor(self.fRec37[0]);
																																																								fTemp219 = self.fRec37[0] - fTemp218;
																																																								self.fVec32[0] = if iSlow229 != 0 {if iSlow231 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp219 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec37[0] + (0.5 - (fTemp218 + F32::floor(self.fRec37[0] + (0.5 - fTemp218))))) + -1.0}} else {if iSlow230 != 0 {1.0 - 4.0 * F32::abs(self.fRec37[0] + (-0.25 - (fTemp218 + F32::floor(self.fRec37[0] + (0.25 - fTemp218)))))} else {F32::sin(6.2831855 * fTemp219)}};
																																																								fTemp220 = fSlow218 * (fTemp211 * OscDsp_faustpower2_f(fTemp208) * (fTemp204 + 0.16666667 * fTemp210 * fTemp208) + self.fVec32[1] + fTemp217 * OscDsp_faustpower2_f(fTemp214) * (fTemp212 + 0.16666667 * fTemp216 * fTemp214) + self.fVec31[1]);
																																																							}
																																																								if iSlow3 & (iSlow7 & iSlow35) != 0 {
																																																									iTemp221 = (fTemp24 < 0.0) as i32;
																																																									fTemp222 = 0.5 * (if iTemp221 != 0 {-iSlow246} else {iSlow246}) as F32;
																																																									fTemp223 = F32::max(fTemp69, 1e-09);
																																																									fTemp224 = if iTemp221 != 0 {fSlow247 - (self.fRec0[0] + F32::floor(fSlow247 - self.fRec0[0]))} else {self.fRec0[0] - (fSlow247 + F32::floor(self.fRec0[0] - fSlow247))};
																																																									fTemp225 = F32::max(0.0, F32::min(1.0, fTemp224 / fTemp223));
																																																									fTemp226 = 1.0 - fTemp225;
																																																									fTemp227 = fSlow248 * fTemp223;
																																																									fTemp228 = ((fTemp224 < fTemp223) as i32) as u32 as F32;
																																																									fTemp229 = 0.5 * (if iTemp221 != 0 {-iSlow249} else {iSlow249}) as F32;
																																																									fTemp230 = if iTemp221 != 0 {fSlow250 - (self.fRec0[0] + F32::floor(fSlow250 - self.fRec0[0]))} else {self.fRec0[0] - (fSlow250 + F32::floor(self.fRec0[0] - fSlow250))};
																																																									fTemp231 = F32::max(0.0, F32::min(1.0, fTemp230 / fTemp223));
																																																									fTemp232 = 1.0 - fTemp231;
																																																									fTemp233 = fSlow251 * fTemp223;
																																																									fTemp234 = ((fTemp230 < fTemp223) as i32) as u32 as F32;
																																																									self.fVec33[0] = fTemp234 * OscDsp_faustpower2_f(fTemp232) * (0.16666667 * fTemp233 * fTemp232 - fTemp229) + fTemp228 * OscDsp_faustpower2_f(fTemp226) * (0.16666667 * fTemp227 * fTemp226 - fTemp222);
																																																									fTemp235 = F32::floor(self.fRec0[0]);
																																																									fTemp236 = self.fRec0[0] - fTemp235;
																																																									self.fVec34[0] = if iSlow229 != 0 {if iSlow231 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp236 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec0[0] + (0.5 - (fTemp235 + F32::floor(self.fRec0[0] + (0.5 - fTemp235))))) + -1.0}} else {if iSlow230 != 0 {1.0 - 4.0 * F32::abs(self.fRec0[0] + (-0.25 - (fTemp235 + F32::floor(self.fRec0[0] + (0.25 - fTemp235)))))} else {F32::sin(6.2831855 * fTemp236)}};
																																																									fTemp237 = (9.0 * self.fRec25[0] + 1.0) * (1.5 * self.fRec23[0] + fTemp228 * OscDsp_faustpower2_f(fTemp225) * (fTemp222 + 0.16666667 * fTemp227 * fTemp225) + self.fVec34[1] + fTemp234 * OscDsp_faustpower2_f(fTemp231) * (fTemp229 + 0.16666667 * fTemp233 * fTemp231) + self.fVec33[1]);
																																																									self.fVec35[0] = fTemp237;
																																																									fTemp238 = fTemp237 - self.fVec35[1];
																																																									iTemp239 = (F32::abs(fTemp238) < 0.0001) as i32;
																																																									fTemp240 = if iTemp239 != 0 {F32::sin(0.7853982 * (fTemp237 + self.fVec35[1]))} else {-(0.63661975 * ((F32::cos(1.5707964 * fTemp237) - F32::cos(1.5707964 * self.fVec35[1])) / if iTemp239 != 0 {1.0} else {fTemp238}))};
																																																									self.fVec36[0] = fTemp240;
																																																									self.fRec38[0] = 0.995 * self.fRec38[1] + fTemp240 - self.fVec36[1];
																																																									fTemp241 = fSlow34 * self.fRec38[0];
																																																								}
																																																									if iSlow3 & (iSlow7 & iSlow87) != 0 {
																																																										fTemp242 = 1.0 - OscDsp_faustpower2_f(1.0 - fTemp66);
																																																										fTemp243 = 1.0 - 0.5 * OscDsp_faustpower2_f(fTemp66);
																																																										iTemp244 = (self.fRec23[0] > 0.5) as i32;
																																																										fTemp245 = if iTemp244 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * self.fRec23[0])};
																																																										iTemp246 = unsafe { itbl3OscDspSIG3[((F32::max(0.0, F32::min(4.0, 1e+01 * (self.fRec23[0] + -0.5)))) as i32) as usize] };
																																																										fTemp247 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp244, iTemp246))) as F32;
																																																										fTemp248 = self.fConst2 * fTemp23 * F32::exp(0.0005776227 * (fTemp247 + 0.83 * fTemp245));
																																																										fTemp249 = fTemp248 + self.fRec39[1];
																																																									}
																																																										if (iSlow3 & (iSlow7 & iSlow87)) | (iSlow102 & (iSlow144 & iSlow177)) != 0 {
																																																											fTemp250 = F32::floor(fTemp28 + 0.87);
																																																											fTemp251 = fTemp28 + (0.87 - (fTemp250 + F32::floor(fTemp28 + (0.87 - fTemp250))));
																																																										}
																																																											if iSlow3 & (iSlow7 & iSlow87) != 0 {
																																																												self.fRec39[0] = if iTemp0 != 0 {fTemp251} else {fTemp249 - F32::floor(fTemp249)};
																																																												fTemp252 = self.fConst2 * fTemp23 * F32::exp(0.0005776227 * (0.36 * fTemp245 + fTemp247));
																																																												fTemp253 = fTemp252 + self.fRec40[1];
																																																											}
																																																												if (iSlow3 & (iSlow7 & iSlow87)) | (iSlow102 & (iSlow144 & iSlow177)) != 0 {
																																																													fTemp254 = F32::floor(fTemp28 + 0.5);
																																																													fTemp255 = fTemp28 + (0.5 - (fTemp254 + F32::floor(fTemp28 + (0.5 - fTemp254))));
																																																												}
																																																													if iSlow3 & (iSlow7 & iSlow87) != 0 {
																																																														self.fRec40[0] = if iTemp0 != 0 {fTemp255} else {fTemp253 - F32::floor(fTemp253)};
																																																														fTemp256 = self.fConst2 * fTemp23 * F32::exp(0.0005776227 * ((i32::wrapping_mul(0, iTemp246)) as F32 - fTemp245)) + self.fRec41[1];
																																																													}
																																																														if (iSlow3 & (iSlow7 & iSlow87)) | (iSlow102 & (iSlow144 & iSlow177)) != 0 {
																																																															fTemp257 = F32::floor(fTemp28 + 0.13);
																																																															fTemp258 = fTemp28 + (0.13 - (fTemp257 + F32::floor(fTemp28 + (0.13 - fTemp257))));
																																																														}
																																																															if iSlow3 & (iSlow7 & iSlow87) != 0 {
																																																																self.fRec41[0] = if iTemp0 != 0 {fTemp258} else {fTemp256 - F32::floor(fTemp256)};
																																																																self.fVec37[0] = fTemp139 * fTemp243 + 0.5 * fTemp242 * (wt_basic(iSlow225, self.fConst0 * F32::abs(self.fConst2 * fTemp23 * F32::exp(-0.0005776227 * fTemp245)), self.fConst0, self.fRec41[0]) + wt_basic(iSlow225, self.fConst0 * F32::abs(fTemp252), self.fConst0, self.fRec40[0]) + wt_basic(iSlow225, self.fConst0 * F32::abs(fTemp248), self.fConst0, self.fRec39[0]));
																																																																fTemp259 = fSlow86 * (self.fVec37[1] / F32::sqrt(OscDsp_faustpower2_f(fTemp243) + 0.75 * OscDsp_faustpower2_f(fTemp242)));
																																																															}
																																																																if iSlow3 & iSlow7 != 0 {
																																																																	fTemp260 = fSlow6 * fSlow257 * (fSlow256 * fTemp259 + fSlow255 * fTemp241 + fSlow254 * fTemp220 + fSlow253 * fTemp188 + fSlow252 * fTemp184 + fSlow238 * fTemp152 + fSlow237 * fTemp140 + fSlow236 * fTemp130 + fSlow235 * fTemp107 + fSlow234 * fTemp89 + fSlow227 * fTemp72 + fSlow224 * fTemp60);
																																																																}
																																																																	if (iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | (iSlow3 & (iSlow42 & iSlow39))))))))))))))) != 0 {
																																																																		self.fVec38[0] = fSlow259;
																																																																		fTemp261 = if ((fSlow259 != self.fVec38[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec42[1] + -1.0};
																																																																		self.fRec42[0] = fTemp261;
																																																																		self.fRec43[0] = if iTemp0 != 0 {fSlow259} else {if (fTemp261 > 0.0) as i32 != 0 {self.fRec43[1] + (fSlow259 - self.fRec43[1]) / F32::max(1.0, fTemp261)} else {fSlow259}};
																																																																		fTemp262 = F32::max(0.0, F32::min(1.0, self.fRec43[0]));
																																																																		iTemp263 = (fTemp262 > 0.0) as i32;
																																																																		fTemp264 = (iTemp263) as F32;
																																																																		iTemp265 = (fTemp264 != 0.0) as i32;
																																																																	}
																																																																		if (iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | (iSlow3 & (iSlow42 & iSlow39)))))))))))))) != 0 {
																																																																			fTemp266 = 256.0 * fTemp23;
																																																																			iTemp267 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp266 > self.fConst16) as i32) + ((fTemp266 > self.fConst22) as i32), (fTemp266 > self.fConst21) as i32), (fTemp266 > self.fConst20) as i32), (fTemp266 > self.fConst19) as i32), (fTemp266 > self.fConst18) as i32), (fTemp266 > self.fConst17) as i32);
																																																																			iTemp268 = (iTemp267 >= 4) as i32;
																																																																			iTemp269 = (iTemp267 >= 2) as i32;
																																																																			iTemp270 = (iTemp267 >= 1) as i32;
																																																																			iTemp271 = (iTemp267 >= 3) as i32;
																																																																			iTemp272 = if iTemp268 != 0 {64} else {if iTemp269 != 0 {if iTemp271 != 0 {128} else {256}} else {if iTemp270 != 0 {512} else {1024}}};
																																																																			fTemp273 = self.fRec0[0] * (iTemp272) as F32;
																																																																			iTemp274 = (fTemp273) as i32;
																																																																			iTemp275 = (i32::wrapping_add(iTemp274, 1)) % iTemp272;
																																																																			iTemp276 = if iTemp268 != 0 {if (iTemp267 >= 6) as i32 != 0 {if (iTemp267 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp267 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp269 != 0 {if iTemp271 != 0 {1792} else {1536}} else {if iTemp270 != 0 {1024} else {0}}};
																																																																			fTemp277 = (iTemp274) as F32;
																																																																			fTemp278 = fTemp273 - fTemp277;
																																																																			fTemp279 = fTemp277 + (1.0 - fTemp273);
																																																																		}
																																																																			if (iSlow3 & (iSlow42 & (iTemp265 & iSlow75))) | ((iSlow3 & (iSlow42 & (iSlow84 & iTemp265))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow78))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow39))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow72))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow69))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow66))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow63))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow60))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow57))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow54))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow51))) | ((iSlow3 & (iSlow42 & (iTemp265 & iSlow48))) | (iSlow3 & (iSlow42 & (iTemp265 & iSlow45))))))))))))))) != 0 {
																																																																				fTemp280 = fTemp264 * (fTemp279 * wt_sample(i32::wrapping_add(iSlow260, i32::wrapping_add(iTemp274, iTemp276))) + fTemp278 * wt_sample(i32::wrapping_add(iSlow260, i32::wrapping_add(iTemp276, iTemp275))));
																																																																			}
																																																																				if (iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | (iSlow3 & (iSlow42 & iSlow39))))))))))))))) != 0 {
																																																																					fTemp281 = fTemp262 * ((iTemp263 != 0) as i32) as u32 as F32;
																																																																					self.fVec39[0] = fSlow261;
																																																																					fTemp282 = if ((fSlow261 != self.fVec39[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec44[1] + -1.0};
																																																																					self.fRec44[0] = fTemp282;
																																																																					self.fRec45[0] = if iTemp0 != 0 {fSlow261} else {if (fTemp282 > 0.0) as i32 != 0 {self.fRec45[1] + (fSlow261 - self.fRec45[1]) / F32::max(1.0, fTemp282)} else {fSlow261}};
																																																																					fTemp283 = F32::max(0.0, F32::min(3.0, self.fRec45[0]));
																																																																					iTemp284 = std::cmp::min(2, (fTemp283) as i32);
																																																																					iTemp285 = (iTemp284 >= 2) as i32;
																																																																					iTemp286 = (iTemp284 >= 1) as i32;
																																																																					iTemp287 = i32::wrapping_mul(2176, (if iTemp285 != 0 {fSlow264} else {if iTemp286 != 0 {fSlow263} else {fSlow262}}) as i32);
																																																																					fTemp288 = (iTemp284) as F32;
																																																																					fTemp289 = fTemp283 - fTemp288;
																																																																					iTemp290 = i32::wrapping_mul(2176, (if iTemp285 != 0 {fSlow263} else {if iTemp286 != 0 {fSlow262} else {fSlow265}}) as i32);
																																																																					fTemp291 = fTemp288 + (1.0 - fTemp283);
																																																																					fTemp292 = 1.0 - fTemp262;
																																																																				}
																																																																					if (iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | (iSlow3 & (iSlow42 & iSlow39)))))))))))))) != 0 {
																																																																						fTemp293 = fTemp292 * (fTemp291 * (fTemp279 * wt_sample(i32::wrapping_add(iTemp276, i32::wrapping_add(iTemp274, iTemp290))) + fTemp278 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp290, iTemp276), iTemp275))) + fTemp289 * (fTemp279 * wt_sample(i32::wrapping_add(iTemp276, i32::wrapping_add(iTemp274, iTemp287))) + fTemp278 * wt_sample(i32::wrapping_add(iTemp275, i32::wrapping_add(iTemp276, iTemp287))))) + fTemp281 * fTemp280;
																																																																					}
																																																																						if (iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | (iSlow3 & (iSlow42 & iSlow39)))))))))))))) != 0 {
																																																																							self.fVec40[0] = fSlow266;
																																																																							fTemp294 = if ((fSlow266 != self.fVec40[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec46[1] + -1.0};
																																																																							self.fRec46[0] = fTemp294;
																																																																							self.fRec47[0] = if iTemp0 != 0 {fSlow266} else {if (fTemp294 > 0.0) as i32 != 0 {self.fRec47[1] + (fSlow266 - self.fRec47[1]) / F32::max(1.0, fTemp294)} else {fSlow266}};
																																																																							fTemp295 = F32::max(0.0, F32::min(1.0, self.fRec47[0]));
																																																																						}
																																																																							if (iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow48)) | (iSlow3 & (iSlow42 & iSlow39)))) != 0 {
																																																																								fTemp296 = (1.0 - fTemp295) * fTemp293;
																																																																							}
																																																																								if iSlow3 & (iSlow42 & iSlow84) != 0 {
																																																																									fTemp297 = fSlow83 * (fTemp296 + fTemp295 * if (fTemp293 > 0.0) as i32 != 0 {1.0 - fTemp293} else {-1.0 - fTemp293});
																																																																								}
																																																																									if (iSlow3 & (iSlow42 & iSlow75)) | (iSlow3 & (iSlow42 & iSlow48)) != 0 {
																																																																										fTemp298 = 512.0 * fTemp23;
																																																																										iTemp299 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp298 > self.fConst16) as i32) + ((fTemp298 > self.fConst22) as i32), (fTemp298 > self.fConst21) as i32), (fTemp298 > self.fConst20) as i32), (fTemp298 > self.fConst19) as i32), (fTemp298 > self.fConst18) as i32), (fTemp298 > self.fConst17) as i32);
																																																																										iTemp300 = (iTemp299 >= 4) as i32;
																																																																										iTemp301 = (iTemp299 >= 2) as i32;
																																																																										iTemp302 = (iTemp299 >= 1) as i32;
																																																																										iTemp303 = (iTemp299 >= 3) as i32;
																																																																										iTemp304 = if iTemp300 != 0 {64} else {if iTemp301 != 0 {if iTemp303 != 0 {128} else {256}} else {if iTemp302 != 0 {512} else {1024}}};
																																																																										fTemp305 = (iTemp304) as F32;
																																																																									}
																																																																										if iSlow3 & (iSlow42 & iSlow75) != 0 {
																																																																											fTemp306 = fTemp305 * if iTemp30 != 0 {fTemp37} else {2.0 * fTemp125};
																																																																											iTemp307 = (fTemp306) as i32;
																																																																											iTemp308 = (i32::wrapping_add(iTemp307, 1)) % iTemp304;
																																																																										}
																																																																											if (iSlow3 & (iSlow42 & iSlow75)) | (iSlow3 & (iSlow42 & iSlow48)) != 0 {
																																																																												iTemp309 = if iTemp300 != 0 {if (iTemp299 >= 6) as i32 != 0 {if (iTemp299 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp299 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp301 != 0 {if iTemp303 != 0 {1792} else {1536}} else {if iTemp302 != 0 {1024} else {0}}};
																																																																											}
																																																																												if iSlow3 & (iSlow42 & iSlow75) != 0 {
																																																																													fTemp310 = (iTemp307) as F32;
																																																																													fTemp311 = fTemp306 - fTemp310;
																																																																													fTemp312 = fTemp310 + (1.0 - fTemp306);
																																																																												}
																																																																													if iSlow3 & (iSlow42 & (iTemp265 & iSlow75)) != 0 {
																																																																														fTemp313 = fTemp264 * (fTemp312 * wt_sample(i32::wrapping_add(iSlow260, i32::wrapping_add(iTemp309, iTemp307))) + fTemp311 * wt_sample(i32::wrapping_add(iSlow260, i32::wrapping_add(iTemp309, iTemp308))));
																																																																													}
																																																																														if (iSlow3 & (iSlow42 & iSlow75)) | (iSlow3 & (iSlow42 & iSlow48)) != 0 {
																																																																															iTemp314 = i32::wrapping_add(iTemp287, iTemp309);
																																																																															iTemp315 = i32::wrapping_add(iTemp290, iTemp309);
																																																																														}
																																																																															if iSlow3 & (iSlow42 & iSlow75) != 0 {
																																																																																fTemp316 = fSlow74 * (fTemp296 + fTemp295 * (fTemp292 * (fTemp291 * (fTemp312 * wt_sample(i32::wrapping_add(iTemp315, iTemp307)) + fTemp311 * wt_sample(i32::wrapping_add(iTemp315, iTemp308))) + fTemp289 * (fTemp312 * wt_sample(i32::wrapping_add(iTemp314, iTemp307)) + fTemp311 * wt_sample(i32::wrapping_add(iTemp314, iTemp308)))) + fTemp281 * fTemp313));
																																																																															}
																																																																																if iSlow3 & (iSlow42 & iSlow78) != 0 {
																																																																																	fTemp317 = fSlow77 * fTemp293 * (1.0 - fTemp295 * (1.0 - F32::sin(6.2831855 * self.fRec0[0] * (7.0 * fTemp295 + 1.0))));
																																																																																}
																																																																																	if iSlow3 & (iSlow42 & iSlow81) != 0 {
																																																																																		fTemp318 = 2.0 * fTemp295 + 1.0;
																																																																																		fTemp319 = 256.0 * fTemp23 * fTemp318;
																																																																																		iTemp320 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp319 > self.fConst16) as i32) + ((fTemp319 > self.fConst22) as i32), (fTemp319 > self.fConst21) as i32), (fTemp319 > self.fConst20) as i32), (fTemp319 > self.fConst19) as i32), (fTemp319 > self.fConst18) as i32), (fTemp319 > self.fConst17) as i32);
																																																																																		iTemp321 = (iTemp320 >= 4) as i32;
																																																																																		iTemp322 = (iTemp320 >= 2) as i32;
																																																																																		iTemp323 = (iTemp320 >= 1) as i32;
																																																																																		iTemp324 = (iTemp320 >= 3) as i32;
																																																																																		iTemp325 = if iTemp321 != 0 {64} else {if iTemp322 != 0 {if iTemp324 != 0 {128} else {256}} else {if iTemp323 != 0 {512} else {1024}}};
																																																																																		fTemp326 = F32::powf(self.fRec0[0], fTemp318) * (iTemp325) as F32;
																																																																																		iTemp327 = (fTemp326) as i32;
																																																																																		iTemp328 = (i32::wrapping_add(iTemp327, 1)) % iTemp325;
																																																																																		iTemp329 = if iTemp321 != 0 {if (iTemp320 >= 6) as i32 != 0 {if (iTemp320 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp320 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp322 != 0 {if iTemp324 != 0 {1792} else {1536}} else {if iTemp323 != 0 {1024} else {0}}};
																																																																																		fTemp330 = (iTemp327) as F32;
																																																																																		fTemp331 = fTemp326 - fTemp330;
																																																																																		fTemp332 = fTemp330 + (1.0 - fTemp326);
																																																																																	}
																																																																																		if iSlow3 & (iSlow42 & (iTemp265 & iSlow81)) != 0 {
																																																																																			fTemp333 = fTemp264 * (fTemp332 * wt_sample(i32::wrapping_add(iSlow260, i32::wrapping_add(iTemp327, iTemp329))) + fTemp331 * wt_sample(i32::wrapping_add(iSlow260, i32::wrapping_add(iTemp329, iTemp328))));
																																																																																		}
																																																																																			if iSlow3 & (iSlow42 & iSlow81) != 0 {
																																																																																				fTemp334 = fSlow80 * (fTemp292 * (fTemp291 * (fTemp332 * wt_sample(i32::wrapping_add(iTemp329, i32::wrapping_add(iTemp290, iTemp327))) + fTemp331 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp290, iTemp329), iTemp328))) + fTemp289 * (fTemp332 * wt_sample(i32::wrapping_add(iTemp329, i32::wrapping_add(iTemp287, iTemp327))) + fTemp331 * wt_sample(i32::wrapping_add(iTemp328, i32::wrapping_add(iTemp287, iTemp329))))) + fTemp281 * fTemp333);
																																																																																			}
																																																																																				if iSlow3 & (iSlow42 & iSlow39) != 0 {
																																																																																					fTemp335 = fSlow38 * (fTemp296 + fTemp295 * F32::sin(7.8539815 * fTemp293));
																																																																																				}
																																																																																					if iSlow3 & (iSlow42 & iSlow45) != 0 {
																																																																																						fTemp336 = 0.25 * (fTemp293 * (4.0 * fTemp295 + 1.0) + 1.0);
																																																																																						fTemp337 = fSlow44 * (1.0 - 4.0 * F32::abs(fTemp336 + (-0.5 - F32::floor(fTemp336))));
																																																																																					}
																																																																																						if iSlow3 & (iSlow42 & iSlow48) != 0 {
																																																																																							fTemp338 = 2.0 * self.fRec0[0] * fTemp305;
																																																																																							iTemp339 = (fTemp338) as i32;
																																																																																							iTemp340 = (i32::wrapping_add(iTemp339, 1)) % iTemp304;
																																																																																							fTemp341 = (iTemp339) as F32;
																																																																																							fTemp342 = fTemp338 - fTemp341;
																																																																																							fTemp343 = fTemp341 + (1.0 - fTemp338);
																																																																																						}
																																																																																							if iSlow3 & (iSlow42 & (iTemp265 & iSlow48)) != 0 {
																																																																																								fTemp344 = fTemp264 * (fTemp343 * wt_sample(i32::wrapping_add(iSlow260, i32::wrapping_add(iTemp339, iTemp309))) + fTemp342 * wt_sample(i32::wrapping_add(iSlow260, i32::wrapping_add(iTemp309, iTemp340))));
																																																																																							}
																																																																																								if iSlow3 & (iSlow42 & iSlow48) != 0 {
																																																																																									fTemp345 = fSlow47 * (fTemp296 + fTemp295 * (iTemp30) as F32 * F32::abs(fTemp292 * (fTemp291 * (fTemp343 * wt_sample(i32::wrapping_add(iTemp309, i32::wrapping_add(iTemp290, iTemp339))) + fTemp342 * wt_sample(i32::wrapping_add(iTemp315, iTemp340))) + fTemp289 * (fTemp343 * wt_sample(i32::wrapping_add(iTemp309, i32::wrapping_add(iTemp287, iTemp339))) + fTemp342 * wt_sample(i32::wrapping_add(iTemp340, iTemp314)))) + fTemp281 * fTemp344));
																																																																																								}
																																																																																									if (iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | (iSlow3 & (iSlow42 & iSlow51))) != 0 {
																																																																																										fTemp346 = F32::tan(self.fConst25 * F32::powf(fSlow275, fTemp295));
																																																																																										fTemp347 = 1.0 / fTemp346;
																																																																																										fTemp348 = (fTemp347 + 1.4142135) / fTemp346 + 1.0;
																																																																																										self.fRec48[0] = fTemp293 - (self.fRec48[2] * ((fTemp347 + -1.4142135) / fTemp346 + 1.0) + 2.0 * self.fRec48[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp346))) / fTemp348;
																																																																																										fTemp349 = (self.fRec48[2] + self.fRec48[0] + 2.0 * self.fRec48[1]) / fTemp348;
																																																																																									}
																																																																																										if iSlow3 & (iSlow42 & iSlow51) != 0 {
																																																																																											fTemp350 = fSlow50 * fTemp349;
																																																																																										}
																																																																																											if iSlow3 & (iSlow42 & iSlow54) != 0 {
																																																																																												fTemp351 = fSlow53 * fTemp349;
																																																																																											}
																																																																																												if iSlow3 & (iSlow42 & iSlow57) != 0 {
																																																																																													fTemp352 = fSlow56 * fTemp349;
																																																																																												}
																																																																																													if (iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | (iSlow3 & (iSlow42 & iSlow60))) != 0 {
																																																																																														fTemp353 = if iTemp7 != 0 {0.0} else {self.fRec50[1] + self.fConst2 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow280, fTemp295)), 0.0)};
																																																																																														self.fRec50[0] = fTemp353 - F32::floor(fTemp353);
																																																																																														self.fRec49[0] = if (self.fRec50[0] < self.fRec50[1]) as i32 != 0 {fTemp293} else {self.fRec49[1]};
																																																																																													}
																																																																																														if iSlow3 & (iSlow42 & iSlow60) != 0 {
																																																																																															fTemp354 = fSlow59 * self.fRec49[0];
																																																																																														}
																																																																																															if iSlow3 & (iSlow42 & iSlow63) != 0 {
																																																																																																fTemp355 = fSlow62 * self.fRec49[0];
																																																																																															}
																																																																																																if iSlow3 & (iSlow42 & iSlow66) != 0 {
																																																																																																	fTemp356 = fSlow65 * self.fRec49[0];
																																																																																																}
																																																																																																	if iSlow3 & (iSlow42 & iSlow69) != 0 {
																																																																																																		fTemp357 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp295)) + -1.0));
																																																																																																		fTemp358 = fSlow68 * (F32::floor(fTemp293 * fTemp357 + 0.5) / fTemp357);
																																																																																																	}
																																																																																																		if iSlow3 & (iSlow42 & iSlow72) != 0 {
																																																																																																			fTemp359 = fSlow71 * fTemp293;
																																																																																																		}
																																																																																																			if iSlow3 & iSlow42 != 0 {
																																																																																																				fTemp360 = fSlow41 * (fSlow285 * fTemp359 + fSlow284 * fTemp358 + fSlow283 * fTemp356 + fSlow282 * fTemp355 + fSlow281 * fTemp354 + fSlow278 * fTemp352 + fSlow277 * fTemp351 + fSlow276 * fTemp350 + fSlow273 * fTemp345 + fSlow272 * fTemp337 + fSlow271 * fTemp335 + fSlow270 * fTemp334 + fSlow269 * fTemp317 + fSlow268 * fTemp316 + fSlow267 * fTemp297);
																																																																																																			}
																																																																																																				if iSlow3 != 0 {
																																																																																																					fTemp361 = fSlow2 * (fSlow286 * fTemp360 + fSlow258 * fTemp260);
																																																																																																				}
																																																																																																					*output0 = fSlow287 * fTemp361;
																																																																																																					self.fVec41[0] = fSlow288;
																																																																																																					let mut fTemp362: F32 = if ((fSlow288 != self.fVec41[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec52[1] + -1.0};
																																																																																																					self.fRec52[0] = fTemp362;
																																																																																																					self.fRec53[0] = if iTemp0 != 0 {fSlow288} else {if (fTemp362 > 0.0) as i32 != 0 {self.fRec53[1] + (fSlow288 - self.fRec53[1]) / F32::max(1.0, fTemp362)} else {fSlow288}};
																																																																																																					let mut fTemp363: F32 = F32::abs(self.fRec53[0]);
																																																																																																					let mut fTemp364: F32 = F32::exp(0.057762265 * (((self.fRec53[0] > 0.0) as i32) - ((self.fRec53[0] < 0.0) as i32)) as F32 * if (fTemp363 < 0.4) as i32 != 0 {F32::min(0.99, 2.5 * fTemp363)} else {if (fTemp363 < 0.82) as i32 != 0 {F32::min(6.0, F32::floor(16.666666 * (fTemp363 + -0.4))) + 1.0} else {(if (fTemp363 < 0.91) as i32 != 0 {12} else {24}) as F32}});
																																																																																																					let mut fTemp365: F32 = fTemp22 * fTemp364;
																																																																																																					if (iSlow102 & (iSlow99 & iSlow213)) | ((iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow189)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | ((iSlow102 & (iSlow99 & iSlow180)) | ((iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow174)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | ((iSlow102 & (iSlow144 & iSlow150)) | ((iSlow102 & (iSlow144 & iSlow147)) | (iSlow102 & (iSlow144 & iSlow141))))))))))))))))))))))))) != 0 {
																																																																																																						fTemp366 = self.fConst2 * fTemp365;
																																																																																																						fTemp367 = fTemp366 + self.fRec51[1];
																																																																																																						self.fRec51[0] = if iTemp0 != 0 {fTemp29} else {fTemp367 - F32::floor(fTemp367)};
																																																																																																					}
																																																																																																						if (iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow144 & iSlow141))) != 0 {
																																																																																																							iTemp368 = (self.fRec51[0] < 0.5) as i32;
																																																																																																						}
																																																																																																							if (iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow174)) | ((iSlow102 & (iSlow144 & iSlow171)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow165)) | ((iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | ((iSlow102 & (iSlow144 & iSlow150)) | ((iSlow102 & (iSlow144 & iSlow147)) | (iSlow102 & (iSlow144 & iSlow141)))))))))))) != 0 {
																																																																																																								self.fVec42[0] = fSlow289;
																																																																																																								fTemp369 = if ((fSlow289 != self.fVec42[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec54[1] + -1.0};
																																																																																																								self.fRec54[0] = fTemp369;
																																																																																																								self.fRec55[0] = if iTemp0 != 0 {fSlow289} else {if (fTemp369 > 0.0) as i32 != 0 {self.fRec55[1] + (fSlow289 - self.fRec55[1]) / F32::max(1.0, fTemp369)} else {fSlow289}};
																																																																																																							}
																																																																																																								if (iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow150)) | (iSlow102 & (iSlow144 & iSlow141)))) != 0 {
																																																																																																									fTemp370 = 1.0 - self.fRec55[0];
																																																																																																								}
																																																																																																									if (iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow174)) | ((iSlow102 & (iSlow144 & iSlow171)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow165)) | ((iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | ((iSlow102 & (iSlow144 & iSlow150)) | ((iSlow102 & (iSlow144 & iSlow147)) | (iSlow102 & (iSlow144 & iSlow141)))))))))))) != 0 {
																																																																																																										self.fVec43[0] = fSlow290;
																																																																																																										fTemp371 = if ((fSlow290 != self.fVec43[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec56[1] + -1.0};
																																																																																																										self.fRec56[0] = fTemp371;
																																																																																																										self.fRec57[0] = if iTemp0 != 0 {fSlow290} else {if (fTemp371 > 0.0) as i32 != 0 {self.fRec57[1] + (fSlow290 - self.fRec57[1]) / F32::max(1.0, fTemp371)} else {fSlow290}};
																																																																																																									}
																																																																																																										if iSlow102 & (iSlow144 & iSlow141) != 0 {
																																																																																																											fTemp372 = 1.0 - self.fRec57[0];
																																																																																																											fTemp373 = if iTemp368 != 0 {fTemp372} else {fTemp370};
																																																																																																											fTemp374 = if (fTemp373 < 0.999) as i32 != 0 {F32::max(0.01, fTemp373)} else {1.0};
																																																																																																										}
																																																																																																											if (iSlow102 & (iSlow99 & iSlow183)) | ((iSlow102 & (iSlow144 & iSlow156)) | (iSlow102 & (iSlow144 & iSlow141))) != 0 {
																																																																																																												fTemp375 = 2.0 * self.fRec51[0];
																																																																																																											}
																																																																																																												if iSlow102 & (iSlow144 & iSlow141) != 0 {
																																																																																																													fTemp376 = F32::floor(fTemp375);
																																																																																																													fTemp377 = fTemp375 - (fTemp374 + fTemp376);
																																																																																																													fTemp378 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp377 / fTemp365)));
																																																																																																													fTemp379 = if iTemp368 != 0 {fSlow292} else {fSlow291};
																																																																																																													fTemp380 = if (fTemp379 >= 2.0) as i32 != 0 {(if (fTemp379 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp379 >= 1.0) as i32 != 0 {4.0} else {6.2831855}};
																																																																																																													fTemp381 = fTemp21 * fTemp380 * fTemp19 * fTemp364;
																																																																																																													iTemp382 = (fTemp379) as i32;
																																																																																																													fTemp383 = self.fConst0 * F32::abs(self.fConst11 * (fTemp365 / fTemp374));
																																																																																																													fTemp384 = 0.5 * wt_basic(iTemp382, fTemp383, self.fConst0, 0.999999);
																																																																																																													fTemp385 = self.fConst11 * fTemp365;
																																																																																																													fTemp386 = fTemp375 - fTemp376;
																																																																																																													fTemp387 = ((((fTemp374 < 1.0) as i32) & ((fTemp386 >= fTemp374) as i32)) & ((fTemp377 < fTemp385) as i32)) as F32;
																																																																																																													fTemp388 = 1.0 - fTemp378;
																																																																																																													fTemp389 = if iTemp368 != 0 {fSlow291} else {fSlow292};
																																																																																																													fTemp390 = if iTemp368 != 0 {fTemp370} else {fTemp372};
																																																																																																													fTemp391 = if (fTemp390 < 0.999) as i32 != 0 {F32::max(0.01, fTemp390)} else {1.0};
																																																																																																													fTemp392 = ((fTemp391 == 1.0) as i32) as u32 as F32;
																																																																																																													fTemp393 = 0.5 * (wt_basic(iTemp382, fTemp383, self.fConst0, 0.0) - fTemp392 * wt_basic((fTemp389) as i32, self.fConst0 * F32::abs(self.fConst11 * (fTemp365 / fTemp391)), self.fConst0, 0.999999));
																																																																																																													fTemp394 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp386 / fTemp365)));
																																																																																																													fTemp395 = 1.0 - fTemp394;
																																																																																																													fTemp396 = fTemp380 / fTemp374 - fTemp392 * if (fTemp389 >= 2.0) as i32 != 0 {(if (fTemp389 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp389 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} / fTemp391;
																																																																																																													fTemp397 = ((fTemp386 < fTemp385) as i32) as u32 as F32;
																																																																																																													self.fVec44[0] = fTemp397 * OscDsp_faustpower2_f(fTemp395) * (self.fConst10 * fTemp365 * fTemp396 * fTemp395 - fTemp393) + fTemp387 * OscDsp_faustpower2_f(fTemp388) * (fTemp384 - self.fConst10 * (fTemp381 * fTemp388 / fTemp374));
																																																																																																													self.fVec45[0] = ((fTemp386 < fTemp374) as i32) as u32 as F32 * wt_basic(iTemp382, fTemp383, self.fConst0, fTemp386 / fTemp374);
																																																																																																													fTemp398 = fSlow140 * (self.fVec45[1] + fTemp397 * OscDsp_faustpower2_f(fTemp394) * (fTemp393 + self.fConst10 * fTemp365 * fTemp394 * fTemp396) + self.fVec44[1] - fTemp387 * OscDsp_faustpower2_f(fTemp378) * (fTemp384 + self.fConst10 * (fTemp381 * fTemp378 / fTemp374)));
																																																																																																												}
																																																																																																													if iSlow102 & (iSlow144 & iSlow147) != 0 {
																																																																																																														fTemp399 = self.fConst12 * fTemp365 * F32::exp(0.0046209814 * self.fRec55[0]);
																																																																																																														fTemp400 = fTemp399 + self.fRec58[1];
																																																																																																														self.fRec58[0] = if iTemp0 != 0 {fTemp65} else {fTemp400 - F32::floor(fTemp400)};
																																																																																																													}
																																																																																																														if (iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow171)) | ((iSlow102 & (iSlow144 & iSlow159)) | (iSlow102 & (iSlow144 & iSlow147)))) != 0 {
																																																																																																															fTemp401 = F32::max(0.0, F32::min(1.0, self.fRec57[0]));
																																																																																																														}
																																																																																																															if iSlow102 & (iSlow144 & iSlow147) != 0 {
																																																																																																																fTemp402 = 4.0 * fTemp401;
																																																																																																																fTemp403 = F32::min(3.0, fTemp402);
																																																																																																															}
																																																																																																																if (iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow174)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | (iSlow102 & (iSlow144 & iSlow147)))))))) != 0 {
																																																																																																																	fTemp404 = F32::abs(fTemp366);
																																																																																																																}
																																																																																																																	if (iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | (iSlow102 & (iSlow144 & iSlow147))))))) != 0 {
																																																																																																																		fTemp405 = self.fConst0 * fTemp404;
																																																																																																																	}
																																																																																																																		if iSlow102 & (iSlow144 & iSlow147) != 0 {
																																																																																																																			fTemp406 = wt_basic(2, fTemp405, self.fConst0, self.fRec51[0]);
																																																																																																																			self.fVec46[0] = wt_basic(0, fTemp405, self.fConst0, self.fRec51[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp403)) + wt_basic(1, fTemp405, self.fConst0, self.fRec51[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp403 + -1.0)) + fTemp406 * F32::max(0.0, 1.0 - F32::abs(fTemp403 + -2.0)) + (fTemp406 - wt_basic(2, fTemp405, self.fConst0, self.fRec51[0] + (-0.5 - 0.42 * F32::max(0.0, fTemp402 + -3.0)))) * F32::max(0.0, 1.0 - F32::abs(fTemp403 + -3.0)) + self.fRec55[0] * wt_basic(iSlow294, self.fConst0 * F32::abs(fTemp399), self.fConst0, self.fRec58[0]);
																																																																																																																			fTemp407 = fSlow146 * (self.fVec46[1] / F32::sqrt(OscDsp_faustpower2_f(self.fRec55[0]) + 1.0));
																																																																																																																		}
																																																																																																																			if (iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | (iSlow102 & (iSlow144 & iSlow150))) != 0 {
																																																																																																																				iTemp408 = (self.fRec51[0] < self.fRec51[1]) as i32;
																																																																																																																			}
																																																																																																																				if (iSlow102 & (iSlow144 & iSlow153)) | (iSlow102 & (iSlow144 & iSlow150)) != 0 {
																																																																																																																					self.iRec59[0] = (i32::wrapping_add(iTemp408, self.iRec59[1])) % 2;
																																																																																																																				}
																																																																																																																					if iSlow102 & (iSlow144 & iSlow150) != 0 {
																																																																																																																						self.fVec47[0] = wt_basic(iSlow294, self.fConst0 * F32::abs(self.fConst12 * fTemp365), self.fConst0, 0.5 * (self.fRec51[0] + (self.iRec59[0]) as F32));
																																																																																																																					}
																																																																																																																						if (iSlow102 & (iSlow144 & iSlow156)) | (iSlow102 & (iSlow144 & iSlow150)) != 0 {
																																																																																																																							fTemp409 = 15.0 * self.fRec57[0];
																																																																																																																							fTemp410 = fTemp409 + 1.0;
																																																																																																																							fTemp411 = fTemp365 * fTemp410;
																																																																																																																							fTemp412 = self.fConst0 * F32::abs(self.fConst2 * fTemp411);
																																																																																																																							fTemp413 = wt_basic(iSlow296, fTemp412, self.fConst0, fTemp410);
																																																																																																																							fTemp414 = wt_basic(iSlow296, fTemp412, self.fConst0, 0.0);
																																																																																																																						}
																																																																																																																							if iSlow102 & (iSlow144 & iSlow150) != 0 {
																																																																																																																								fTemp415 = 0.5 * (fTemp414 - fTemp413);
																																																																																																																							}
																																																																																																																								if (iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | (iSlow102 & (iSlow144 & iSlow150))) != 0 {
																																																																																																																									fTemp416 = F32::max(0.0, F32::min(1.0, self.fConst0 * (self.fRec51[0] / fTemp365)));
																																																																																																																									fTemp417 = 1.0 - fTemp416;
																																																																																																																								}
																																																																																																																									if iSlow102 & (iSlow144 & iSlow150) != 0 {
																																																																																																																										fTemp418 = fTemp409 + (1.0 - F32::floor(fTemp410));
																																																																																																																										fTemp419 = fSlow301 - if iSlow297 != 0 {fSlow300} else {if iSlow298 != 0 {(i32::wrapping_sub(4, i32::wrapping_mul(8, ((fTemp418 >= 0.25) as i32) & ((fTemp418 < 0.75) as i32)))) as F32} else {6.2831855 * F32::cos(6.2831855 * fTemp418)}};
																																																																																																																									}
																																																																																																																										if (iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | (iSlow102 & (iSlow144 & iSlow150))) != 0 {
																																																																																																																											fTemp420 = (iTemp408) as F32;
																																																																																																																											fTemp421 = fTemp420 * OscDsp_faustpower2_f(fTemp417);
																																																																																																																										}
																																																																																																																											if iSlow102 & (iSlow144 & iSlow150) != 0 {
																																																																																																																												self.fVec48[0] = fTemp421 * (self.fConst13 * fTemp411 * fTemp419 * fTemp417 - fTemp415);
																																																																																																																											}
																																																																																																																												if (iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | (iSlow102 & (iSlow144 & iSlow150))) != 0 {
																																																																																																																													fTemp422 = fTemp420 * OscDsp_faustpower2_f(fTemp416);
																																																																																																																												}
																																																																																																																													if (iSlow102 & (iSlow144 & iSlow156)) | (iSlow102 & (iSlow144 & iSlow150)) != 0 {
																																																																																																																														fTemp423 = wt_basic(iSlow296, fTemp412, self.fConst0, self.fRec51[0] * fTemp410);
																																																																																																																														self.fVec49[0] = fTemp423;
																																																																																																																													}
																																																																																																																														if iSlow102 & (iSlow144 & iSlow150) != 0 {
																																																																																																																															fTemp424 = fSlow149 * (fTemp370 * (self.fVec49[1] + fTemp422 * (fTemp415 + self.fConst13 * fTemp411 * fTemp416 * fTemp419) + self.fVec48[1]) + self.fRec55[0] * self.fVec47[1]);
																																																																																																																														}
																																																																																																																															if iSlow102 & (iSlow144 & iSlow153) != 0 {
																																																																																																																																fTemp425 = if self.iRec59[0] != 0 {fSlow292} else {fSlow291};
																																																																																																																																fTemp426 = 7.0 * self.fRec57[0] + 1.0;
																																																																																																																																fTemp427 = fTemp426 * (self.fRec55[0] + 1.0);
																																																																																																																																fTemp428 = if self.iRec59[0] != 0 {fTemp426} else {fTemp427};
																																																																																																																																fTemp429 = if self.iRec59[0] != 0 {fSlow291} else {fSlow292};
																																																																																																																																iTemp430 = (fTemp429) as i32;
																																																																																																																																fTemp431 = if self.iRec59[0] != 0 {fTemp427} else {fTemp426};
																																																																																																																																fTemp432 = 0.5 * (fTemp431 * wt_basic(iTemp430, fTemp405, self.fConst0, 0.0) - fTemp428 * wt_basic((fTemp425) as i32, fTemp405, self.fConst0, 0.999999));
																																																																																																																																fTemp433 = if (fTemp429 >= 2.0) as i32 != 0 {(if (fTemp429 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp429 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp431 - if (fTemp425 >= 2.0) as i32 != 0 {(if (fTemp425 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp425 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp428;
																																																																																																																																self.fVec50[0] = fTemp421 * (self.fConst13 * fTemp365 * fTemp417 * fTemp433 - fTemp432);
																																																																																																																																self.fVec51[0] = wt_basic(iTemp430, fTemp405, self.fConst0, self.fRec51[0]) * fTemp431;
																																																																																																																																fTemp434 = self.fVec51[1] + fTemp422 * (fTemp432 + self.fConst13 * fTemp365 * fTemp416 * fTemp433) + self.fVec50[1];
																																																																																																																																self.fVec52[0] = fTemp434;
																																																																																																																																fTemp435 = fTemp434 - self.fVec52[1];
																																																																																																																																iTemp436 = (F32::abs(fTemp435) < 0.0001) as i32;
																																																																																																																																fTemp437 = 4.0 * F32::floor(0.25 * (self.fVec52[1] + 1.0));
																																																																																																																																fTemp438 = self.fVec52[1] - fTemp437;
																																																																																																																																fTemp439 = 4.0 * F32::floor(0.25 * (fTemp434 + 1.0));
																																																																																																																																fTemp440 = fTemp434 - fTemp439;
																																																																																																																																fTemp441 = 0.125 * (fTemp434 + self.fVec52[1]);
																																																																																																																																fTemp442 = fSlow152 * if iTemp436 != 0 {1.0 - 4.0 * F32::abs(fTemp441 + (-0.25 - F32::floor(fTemp441 + 0.25)))} else {(if (fTemp440 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp440)} else {1.0 - 0.5 * OscDsp_faustpower2_f(fTemp434 + (-2.0 - fTemp439))} - if (fTemp438 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp438)} else {1.0 - 0.5 * OscDsp_faustpower2_f(self.fVec52[1] + (-2.0 - fTemp437))}) / if iTemp436 != 0 {1.0} else {fTemp435}};
																																																																																																																															}
																																																																																																																																if iSlow102 & (iSlow144 & iSlow156) != 0 {
																																																																																																																																	fTemp443 = 4.0 * F32::max(0.0, F32::min(1.0, self.fRec55[0]));
																																																																																																																																	iTemp444 = (F32::min(3.0, fTemp443)) as i32;
																																																																																																																																	iTemp445 = i32::wrapping_add(iTemp444, 1);
																																																																																																																																	iTemp446 = (iTemp445 >= 3) as i32;
																																																																																																																																	iTemp447 = (iTemp445 >= 2) as i32;
																																																																																																																																	iTemp448 = (iTemp445 >= 1) as i32;
																																																																																																																																	iTemp449 = (iTemp445 >= 4) as i32;
																																																																																																																																	fTemp450 = 0.5 * (wt_basic(iSlow294, fTemp405, self.fConst0, 0.999999) + 1.0);
																																																																																																																																	fTemp451 = (iTemp444) as F32;
																																																																																																																																	fTemp452 = fTemp443 - fTemp451;
																																																																																																																																	iTemp453 = (iTemp444 >= 3) as i32;
																																																																																																																																	iTemp454 = (iTemp444 >= 2) as i32;
																																																																																																																																	iTemp455 = (iTemp444 >= 1) as i32;
																																																																																																																																	iTemp456 = (iTemp444 >= 4) as i32;
																																																																																																																																	fTemp457 = fTemp451 + (1.0 - fTemp443);
																																																																																																																																	fTemp458 = 0.5 * (wt_basic(iSlow294, fTemp405, self.fConst0, 0.0) + 1.0);
																																																																																																																																	fTemp459 = fTemp414 * (fTemp457 * if iTemp453 != 0 {if iTemp456 != 0 {fTemp458} else {0.0}} else {if iTemp454 != 0 {0.0} else {(if iTemp455 != 0 {0} else {1}) as F32}} + fTemp452 * if iTemp446 != 0 {if iTemp449 != 0 {fTemp458} else {0.0}} else {if iTemp447 != 0 {0.0} else {(if iTemp448 != 0 {0} else {1}) as F32}}) - fTemp413 * (fTemp457 * if iTemp453 != 0 {if iTemp456 != 0 {fTemp450} else {6e-06}} else {if iTemp454 != 0 {0.0} else {if iTemp455 != 0 {2e-06} else {1e-06}}} + fTemp452 * if iTemp446 != 0 {if iTemp449 != 0 {fTemp450} else {6e-06}} else {if iTemp447 != 0 {0.0} else {if iTemp448 != 0 {2e-06} else {1e-06}}});
																																																																																																																																	self.fVec53[0] = fTemp421 * fTemp459;
																																																																																																																																}
																																																																																																																																	if (iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow144 & iSlow156)) != 0 {
																																																																																																																																		fTemp460 = 1.0 - self.fRec51[0];
																																																																																																																																	}
																																																																																																																																		if iSlow102 & (iSlow144 & iSlow156) != 0 {
																																																																																																																																			fTemp461 = 1.0 - F32::abs(fTemp375 + -1.0);
																																																																																																																																			fTemp462 = F32::sin(3.1415927 * self.fRec51[0]);
																																																																																																																																			fTemp463 = F32::min(1.0, 6.0 * F32::min(self.fRec51[0], fTemp460));
																																																																																																																																			fTemp464 = 0.5 * (wt_basic(iSlow294, fTemp405, self.fConst0, self.fRec51[0]) + 1.0);
																																																																																																																																			self.fVec54[0] = fTemp423 * (fTemp457 * if iTemp453 != 0 {if iTemp456 != 0 {fTemp464} else {fTemp463}} else {if iTemp454 != 0 {fTemp462} else {if iTemp455 != 0 {fTemp461} else {fTemp460}}} + fTemp452 * if iTemp446 != 0 {if iTemp449 != 0 {fTemp464} else {fTemp463}} else {if iTemp447 != 0 {fTemp462} else {if iTemp448 != 0 {fTemp461} else {fTemp460}}});
																																																																																																																																			fTemp465 = fSlow155 * (self.fVec54[1] + 0.5 * (fTemp422 * fTemp459 - self.fVec53[1]));
																																																																																																																																		}
																																																																																																																																			if iSlow102 & (iSlow144 & iSlow159) != 0 {
																																																																																																																																				fTemp466 = self.fConst2 * fTemp365 * (2.17 * fTemp401 + 3.0);
																																																																																																																																				fTemp467 = fTemp466 + self.fRec60[1];
																																																																																																																																				self.fRec60[0] = if iTemp0 != 0 {fTemp134} else {fTemp467 - F32::floor(fTemp467)};
																																																																																																																																				self.fVec55[0] = wt_basic(iSlow294, self.fConst0 * F32::abs(fTemp466), self.fConst0, self.fRec60[0]);
																																																																																																																																				fTemp468 = self.fConst2 * fTemp365 * (1.41 * fTemp401 + 2.0);
																																																																																																																																				fTemp469 = fTemp468 + self.fRec61[1];
																																																																																																																																				self.fRec61[0] = if iTemp0 != 0 {fTemp138} else {fTemp469 - F32::floor(fTemp469)};
																																																																																																																																				self.fVec56[0] = wt_basic(iSlow294, self.fConst0 * F32::abs(fTemp468), self.fConst0, self.fRec61[0]);
																																																																																																																																			}
																																																																																																																																				if (iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow162)) | (iSlow102 & (iSlow144 & iSlow159)))) != 0 {
																																																																																																																																					fTemp470 = wt_basic(iSlow296, fTemp405, self.fConst0, self.fRec51[0]);
																																																																																																																																					self.fVec57[0] = fTemp470;
																																																																																																																																				}
																																																																																																																																					if iSlow102 & (iSlow144 & iSlow159) != 0 {
																																																																																																																																						fTemp471 = fSlow158 * (0.57735026 * fTemp370 * (self.fVec57[1] + self.fVec56[1] + self.fVec55[1]) + 1.5 * self.fRec55[0] * self.fVec57[1] * self.fVec56[1] * self.fVec55[1]);
																																																																																																																																					}
																																																																																																																																						if iSlow102 & (iSlow144 & iSlow162) != 0 {
																																																																																																																																							iTemp472 = (F32::max(0.0, F32::min(1e+01, 10.999 * self.fRec57[0]))) as i32;
																																																																																																																																							fTemp473 = unsafe { ftbl1OscDspSIG1[(i32::wrapping_add(iTemp472, 1)) as usize] };
																																																																																																																																							fTemp474 = self.fConst2 * fTemp365 * fTemp473;
																																																																																																																																							fTemp475 = fTemp474 + self.fRec62[1];
																																																																																																																																							fTemp476 = fSlow137 * fTemp27 * fTemp473;
																																																																																																																																							fTemp477 = F32::floor(fTemp476);
																																																																																																																																							self.fRec62[0] = if iTemp0 != 0 {fTemp476 - (fTemp477 + F32::floor(fTemp476 - fTemp477))} else {fTemp475 - F32::floor(fTemp475)};
																																																																																																																																							self.fVec58[0] = wt_basic(iSlow294, self.fConst0 * F32::abs(fTemp474), self.fConst0, self.fRec62[0]);
																																																																																																																																							fTemp478 = unsafe { ftbl1OscDspSIG1[iTemp472 as usize] };
																																																																																																																																							fTemp479 = self.fConst2 * fTemp365 * fTemp478;
																																																																																																																																							fTemp480 = fTemp479 + self.fRec63[1];
																																																																																																																																							fTemp481 = fSlow137 * fTemp27 * fTemp478;
																																																																																																																																							fTemp482 = F32::floor(fTemp481);
																																																																																																																																							self.fRec63[0] = if iTemp0 != 0 {fTemp481 - (fTemp482 + F32::floor(fTemp481 - fTemp482))} else {fTemp480 - F32::floor(fTemp480)};
																																																																																																																																							self.fVec59[0] = wt_basic(iSlow294, self.fConst0 * F32::abs(fTemp479), self.fConst0, self.fRec63[0]);
																																																																																																																																							fTemp483 = fSlow161 * self.fVec57[1] * (fTemp370 * self.fVec59[1] + self.fRec55[0] * self.fVec58[1]);
																																																																																																																																						}
																																																																																																																																							if iSlow102 & (iSlow144 & iSlow165) != 0 {
																																																																																																																																								fTemp484 = unsafe { ftbl2OscDspSIG2[((F32::max(0.0, F32::min(9.0, 9.999 * self.fRec55[0]))) as i32) as usize] };
																																																																																																																																								fTemp485 = self.fConst2 * fTemp365 * fTemp484;
																																																																																																																																								fTemp486 = F32::max(F32::abs(fTemp485), 1e-09);
																																																																																																																																								iTemp487 = (fTemp485 < 0.0) as i32;
																																																																																																																																								fTemp488 = fTemp485 + self.fRec64[1];
																																																																																																																																								fTemp489 = fSlow137 * fTemp27 * fTemp484;
																																																																																																																																								fTemp490 = F32::floor(fTemp489);
																																																																																																																																								self.fRec64[0] = if iTemp0 != 0 {fTemp489 - (fTemp490 + F32::floor(fTemp489 - fTemp490))} else {fTemp488 - F32::floor(fTemp488)};
																																																																																																																																								fTemp491 = if iTemp487 != 0 {fSlow311 - (self.fRec64[0] + F32::floor(fSlow311 - self.fRec64[0]))} else {self.fRec64[0] - (fSlow311 + F32::floor(self.fRec64[0] - fSlow311))};
																																																																																																																																								iTemp492 = ((fTemp491 < fTemp486) as i32) * if iTemp487 != 0 {-iSlow309} else {iSlow309};
																																																																																																																																								fTemp493 = if iTemp487 != 0 {fSlow313 - (self.fRec64[0] + F32::floor(fSlow313 - self.fRec64[0]))} else {self.fRec64[0] - (fSlow313 + F32::floor(self.fRec64[0] - fSlow313))};
																																																																																																																																								iTemp494 = ((fTemp493 < fTemp486) as i32) * if iTemp487 != 0 {-iSlow312} else {iSlow312};
																																																																																																																																								fTemp495 = F32::floor(self.fRec64[0]);
																																																																																																																																								fTemp496 = self.fRec64[0] - fTemp495;
																																																																																																																																								fTemp497 = if iSlow307 != 0 {if iSlow308 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp496 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec64[0] + (0.5 - (fTemp495 + F32::floor(self.fRec64[0] + (0.5 - fTemp495))))) + -1.0}} else {if iSlow310 != 0 {1.0 - 4.0 * F32::abs(self.fRec64[0] + (-0.25 - (fTemp495 + F32::floor(self.fRec64[0] + (0.25 - fTemp495)))))} else {F32::sin(6.2831855 * fTemp496)}};
																																																																																																																																								self.fVec60[0] = fTemp497;
																																																																																																																																							}
																																																																																																																																								if (iSlow102 & (iSlow144 & iSlow168)) | (iSlow102 & (iSlow144 & iSlow165)) != 0 {
																																																																																																																																									fTemp498 = OscDsp_faustpower2_f(self.fRec57[0]);
																																																																																																																																								}
																																																																																																																																									if iSlow102 & (iSlow144 & iSlow165) != 0 {
																																																																																																																																										fTemp499 = self.fConst2 * fTemp365 * (8.0 * fTemp498 * fTemp484 * (0.5 * (fTemp497 + self.fVec60[1] - (i32::wrapping_add(iTemp494, iTemp492)) as F32) + (fTemp493 * (iTemp494) as F32 + fTemp491 * (iTemp492) as F32) / fTemp486) + 1.0);
																																																																																																																																										iTemp500 = (fTemp499 < 0.0) as i32;
																																																																																																																																										fTemp501 = 0.5 * (if iTemp500 != 0 {-iSlow314} else {iSlow314}) as F32;
																																																																																																																																										fTemp502 = F32::max(F32::abs(fTemp499), 1e-09);
																																																																																																																																										fTemp503 = self.fRec65[1] + fTemp499;
																																																																																																																																										self.fRec65[0] = if iTemp0 != 0 {fTemp29} else {fTemp503 - F32::floor(fTemp503)};
																																																																																																																																										fTemp504 = if iTemp500 != 0 {fSlow315 - (self.fRec65[0] + F32::floor(fSlow315 - self.fRec65[0]))} else {self.fRec65[0] - (fSlow315 + F32::floor(self.fRec65[0] - fSlow315))};
																																																																																																																																										fTemp505 = F32::max(0.0, F32::min(1.0, fTemp504 / fTemp502));
																																																																																																																																										fTemp506 = 1.0 - fTemp505;
																																																																																																																																										fTemp507 = ((fTemp504 < fTemp502) as i32) as u32 as F32;
																																																																																																																																										fTemp508 = 0.5 * (if iTemp500 != 0 {-iSlow317} else {iSlow317}) as F32;
																																																																																																																																										fTemp509 = if iTemp500 != 0 {fSlow318 - (self.fRec65[0] + F32::floor(fSlow318 - self.fRec65[0]))} else {self.fRec65[0] - (fSlow318 + F32::floor(self.fRec65[0] - fSlow318))};
																																																																																																																																										fTemp510 = F32::max(0.0, F32::min(1.0, fTemp509 / fTemp502));
																																																																																																																																										fTemp511 = 1.0 - fTemp510;
																																																																																																																																										fTemp512 = ((fTemp509 < fTemp502) as i32) as u32 as F32;
																																																																																																																																										self.fVec61[0] = fTemp512 * OscDsp_faustpower2_f(fTemp511) * (0.16666667 * fTemp502 * fSlow319 * fTemp511 - fTemp508) + fTemp507 * OscDsp_faustpower2_f(fTemp506) * (0.16666667 * fTemp502 * fSlow316 * fTemp506 - fTemp501);
																																																																																																																																										fTemp513 = F32::floor(self.fRec65[0]);
																																																																																																																																										fTemp514 = self.fRec65[0] - fTemp513;
																																																																																																																																										self.fVec62[0] = if iSlow297 != 0 {if iSlow299 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp514 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec65[0] + (0.5 - (fTemp513 + F32::floor(self.fRec65[0] + (0.5 - fTemp513))))) + -1.0}} else {if iSlow298 != 0 {1.0 - 4.0 * F32::abs(self.fRec65[0] + (-0.25 - (fTemp513 + F32::floor(self.fRec65[0] + (0.25 - fTemp513)))))} else {F32::sin(6.2831855 * fTemp514)}};
																																																																																																																																										fTemp515 = fSlow164 * (fTemp507 * OscDsp_faustpower2_f(fTemp505) * (fTemp501 + 0.16666667 * fTemp502 * fTemp505 * fSlow316) + self.fVec62[1] + fTemp512 * OscDsp_faustpower2_f(fTemp510) * (fTemp508 + 0.16666667 * fTemp502 * fTemp510 * fSlow319) + self.fVec61[1]);
																																																																																																																																									}
																																																																																																																																										if iSlow102 & (iSlow144 & iSlow168) != 0 {
																																																																																																																																											fTemp516 = F32::max(1e+01, F32::min(self.fConst14, fTemp365 * F32::exp(0.6931472 * (8.0 * self.fRec55[0] + -3.0))));
																																																																																																																																											fTemp517 = 1.0 / F32::tan(self.fConst8 * fTemp516);
																																																																																																																																											self.fRec66[0] = (fTemp187 - self.fRec66[1] * (1.0 - fTemp517)) / (fTemp517 + 1.0);
																																																																																																																																											fTemp518 = fSlow167 * (self.fVec57[1] * (1.5 * self.fRec57[0] * self.fRec66[0] * F32::sqrt(self.fConst15 / fTemp516) + 1.0) / F32::sqrt(2.25 * fTemp498 + 1.0));
																																																																																																																																										}
																																																																																																																																											if iSlow102 & (iSlow144 & iSlow171) != 0 {
																																																																																																																																												fTemp519 = 0.6 * fTemp401 * self.fRec68[1] + 1.0;
																																																																																																																																												fTemp520 = self.fConst2 * fTemp365 * fTemp519;
																																																																																																																																												fTemp521 = self.fRec67[1] + fTemp520;
																																																																																																																																												iTemp522 = (fTemp521 >= 1.0) as i32;
																																																																																																																																												self.fRec67[0] = if iTemp0 != 0 {fTemp29} else {fTemp521 - (iTemp522) as F32};
																																																																																																																																												self.fRec68[0] = if iTemp522 != 0 {fTemp15} else {self.fRec68[1]};
																																																																																																																																												fTemp523 = F32::max(F32::abs(fTemp520), 1e-09);
																																																																																																																																												iTemp524 = (fTemp520 < 0.0) as i32;
																																																																																																																																												fTemp525 = if iTemp524 != 0 {fSlow311 - (self.fRec67[0] + F32::floor(fSlow311 - self.fRec67[0]))} else {self.fRec67[0] - (fSlow311 + F32::floor(self.fRec67[0] - fSlow311))};
																																																																																																																																												iTemp526 = ((fTemp525 < fTemp523) as i32) * if iTemp524 != 0 {-iSlow309} else {iSlow309};
																																																																																																																																												fTemp527 = if iTemp524 != 0 {fSlow313 - (self.fRec67[0] + F32::floor(fSlow313 - self.fRec67[0]))} else {self.fRec67[0] - (fSlow313 + F32::floor(self.fRec67[0] - fSlow313))};
																																																																																																																																												iTemp528 = ((fTemp527 < fTemp523) as i32) * if iTemp524 != 0 {-iSlow312} else {iSlow312};
																																																																																																																																												fTemp529 = F32::floor(self.fRec67[0]);
																																																																																																																																												fTemp530 = self.fRec67[0] - fTemp529;
																																																																																																																																												fTemp531 = if iSlow307 != 0 {if iSlow308 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp530 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec67[0] + (0.5 - (fTemp529 + F32::floor(self.fRec67[0] + (0.5 - fTemp529))))) + -1.0}} else {if iSlow310 != 0 {1.0 - 4.0 * F32::abs(self.fRec67[0] + (-0.25 - (fTemp529 + F32::floor(self.fRec67[0] + (0.25 - fTemp529)))))} else {F32::sin(6.2831855 * fTemp530)}};
																																																																																																																																												self.fVec63[0] = fTemp531;
																																																																																																																																												fTemp532 = self.fConst2 * fTemp365 * (4.0 * self.fRec55[0] * fTemp519 * (0.5 * (fTemp531 + self.fVec63[1] - (i32::wrapping_add(iTemp528, iTemp526)) as F32) + (fTemp527 * (iTemp528) as F32 + fTemp525 * (iTemp526) as F32) / fTemp523) + 1.0);
																																																																																																																																												iTemp533 = (fTemp532 < 0.0) as i32;
																																																																																																																																												fTemp534 = 0.5 * (if iTemp533 != 0 {-iSlow314} else {iSlow314}) as F32;
																																																																																																																																												fTemp535 = F32::max(F32::abs(fTemp532), 1e-09);
																																																																																																																																												fTemp536 = self.fRec69[1] + fTemp532;
																																																																																																																																												self.fRec69[0] = if iTemp0 != 0 {fTemp29} else {fTemp536 - F32::floor(fTemp536)};
																																																																																																																																												fTemp537 = if iTemp533 != 0 {fSlow315 - (self.fRec69[0] + F32::floor(fSlow315 - self.fRec69[0]))} else {self.fRec69[0] - (fSlow315 + F32::floor(self.fRec69[0] - fSlow315))};
																																																																																																																																												fTemp538 = F32::max(0.0, F32::min(1.0, fTemp537 / fTemp535));
																																																																																																																																												fTemp539 = 1.0 - fTemp538;
																																																																																																																																												fTemp540 = fSlow316 * fTemp535;
																																																																																																																																												fTemp541 = ((fTemp537 < fTemp535) as i32) as u32 as F32;
																																																																																																																																												fTemp542 = 0.5 * (if iTemp533 != 0 {-iSlow317} else {iSlow317}) as F32;
																																																																																																																																												fTemp543 = if iTemp533 != 0 {fSlow318 - (self.fRec69[0] + F32::floor(fSlow318 - self.fRec69[0]))} else {self.fRec69[0] - (fSlow318 + F32::floor(self.fRec69[0] - fSlow318))};
																																																																																																																																												fTemp544 = F32::max(0.0, F32::min(1.0, fTemp543 / fTemp535));
																																																																																																																																												fTemp545 = 1.0 - fTemp544;
																																																																																																																																												fTemp546 = fSlow319 * fTemp535;
																																																																																																																																												fTemp547 = ((fTemp543 < fTemp535) as i32) as u32 as F32;
																																																																																																																																												self.fVec64[0] = fTemp547 * OscDsp_faustpower2_f(fTemp545) * (0.16666667 * fTemp546 * fTemp545 - fTemp542) + fTemp541 * OscDsp_faustpower2_f(fTemp539) * (0.16666667 * fTemp540 * fTemp539 - fTemp534);
																																																																																																																																												fTemp548 = F32::floor(self.fRec69[0]);
																																																																																																																																												fTemp549 = self.fRec69[0] - fTemp548;
																																																																																																																																												self.fVec65[0] = if iSlow297 != 0 {if iSlow299 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp549 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec69[0] + (0.5 - (fTemp548 + F32::floor(self.fRec69[0] + (0.5 - fTemp548))))) + -1.0}} else {if iSlow298 != 0 {1.0 - 4.0 * F32::abs(self.fRec69[0] + (-0.25 - (fTemp548 + F32::floor(self.fRec69[0] + (0.25 - fTemp548)))))} else {F32::sin(6.2831855 * fTemp549)}};
																																																																																																																																												fTemp550 = fSlow170 * (fTemp541 * OscDsp_faustpower2_f(fTemp538) * (fTemp534 + 0.16666667 * fTemp540 * fTemp538) + self.fVec65[1] + fTemp547 * OscDsp_faustpower2_f(fTemp544) * (fTemp542 + 0.16666667 * fTemp546 * fTemp544) + self.fVec64[1]);
																																																																																																																																											}
																																																																																																																																												if iSlow102 & (iSlow144 & iSlow174) != 0 {
																																																																																																																																													iTemp551 = (fTemp366 < 0.0) as i32;
																																																																																																																																													fTemp552 = 0.5 * (if iTemp551 != 0 {-iSlow314} else {iSlow314}) as F32;
																																																																																																																																													fTemp553 = F32::max(fTemp404, 1e-09);
																																																																																																																																													fTemp554 = if iTemp551 != 0 {fSlow315 - (self.fRec51[0] + F32::floor(fSlow315 - self.fRec51[0]))} else {self.fRec51[0] - (fSlow315 + F32::floor(self.fRec51[0] - fSlow315))};
																																																																																																																																													fTemp555 = F32::max(0.0, F32::min(1.0, fTemp554 / fTemp553));
																																																																																																																																													fTemp556 = 1.0 - fTemp555;
																																																																																																																																													fTemp557 = fSlow316 * fTemp553;
																																																																																																																																													fTemp558 = ((fTemp554 < fTemp553) as i32) as u32 as F32;
																																																																																																																																													fTemp559 = 0.5 * (if iTemp551 != 0 {-iSlow317} else {iSlow317}) as F32;
																																																																																																																																													fTemp560 = if iTemp551 != 0 {fSlow318 - (self.fRec51[0] + F32::floor(fSlow318 - self.fRec51[0]))} else {self.fRec51[0] - (fSlow318 + F32::floor(self.fRec51[0] - fSlow318))};
																																																																																																																																													fTemp561 = F32::max(0.0, F32::min(1.0, fTemp560 / fTemp553));
																																																																																																																																													fTemp562 = 1.0 - fTemp561;
																																																																																																																																													fTemp563 = fSlow319 * fTemp553;
																																																																																																																																													fTemp564 = ((fTemp560 < fTemp553) as i32) as u32 as F32;
																																																																																																																																													self.fVec66[0] = fTemp564 * OscDsp_faustpower2_f(fTemp562) * (0.16666667 * fTemp563 * fTemp562 - fTemp559) + fTemp558 * OscDsp_faustpower2_f(fTemp556) * (0.16666667 * fTemp557 * fTemp556 - fTemp552);
																																																																																																																																													fTemp565 = F32::floor(self.fRec51[0]);
																																																																																																																																													fTemp566 = self.fRec51[0] - fTemp565;
																																																																																																																																													self.fVec67[0] = if iSlow297 != 0 {if iSlow299 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp566 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec51[0] + (0.5 - (fTemp565 + F32::floor(self.fRec51[0] + (0.5 - fTemp565))))) + -1.0}} else {if iSlow298 != 0 {1.0 - 4.0 * F32::abs(self.fRec51[0] + (-0.25 - (fTemp565 + F32::floor(self.fRec51[0] + (0.25 - fTemp565)))))} else {F32::sin(6.2831855 * fTemp566)}};
																																																																																																																																													fTemp567 = (9.0 * self.fRec57[0] + 1.0) * (1.5 * self.fRec55[0] + fTemp558 * OscDsp_faustpower2_f(fTemp555) * (fTemp552 + 0.16666667 * fTemp557 * fTemp555) + self.fVec67[1] + fTemp564 * OscDsp_faustpower2_f(fTemp561) * (fTemp559 + 0.16666667 * fTemp563 * fTemp561) + self.fVec66[1]);
																																																																																																																																													self.fVec68[0] = fTemp567;
																																																																																																																																													fTemp568 = fTemp567 - self.fVec68[1];
																																																																																																																																													iTemp569 = (F32::abs(fTemp568) < 0.0001) as i32;
																																																																																																																																													fTemp570 = if iTemp569 != 0 {F32::sin(0.7853982 * (fTemp567 + self.fVec68[1]))} else {-(0.63661975 * ((F32::cos(1.5707964 * fTemp567) - F32::cos(1.5707964 * self.fVec68[1])) / if iTemp569 != 0 {1.0} else {fTemp568}))};
																																																																																																																																													self.fVec69[0] = fTemp570;
																																																																																																																																													self.fRec70[0] = 0.995 * self.fRec70[1] + fTemp570 - self.fVec69[1];
																																																																																																																																													fTemp571 = fSlow173 * self.fRec70[0];
																																																																																																																																												}
																																																																																																																																													if iSlow102 & (iSlow144 & iSlow177) != 0 {
																																																																																																																																														fTemp572 = 1.0 - OscDsp_faustpower2_f(1.0 - fTemp401);
																																																																																																																																														fTemp573 = 1.0 - 0.5 * OscDsp_faustpower2_f(fTemp401);
																																																																																																																																														iTemp574 = (self.fRec55[0] > 0.5) as i32;
																																																																																																																																														fTemp575 = if iTemp574 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * self.fRec55[0])};
																																																																																																																																														iTemp576 = unsafe { itbl3OscDspSIG3[((F32::max(0.0, F32::min(4.0, 1e+01 * (self.fRec55[0] + -0.5)))) as i32) as usize] };
																																																																																																																																														fTemp577 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp574, iTemp576))) as F32;
																																																																																																																																														fTemp578 = self.fConst2 * fTemp365 * F32::exp(0.0005776227 * (fTemp577 + 0.83 * fTemp575));
																																																																																																																																														fTemp579 = fTemp578 + self.fRec71[1];
																																																																																																																																														self.fRec71[0] = if iTemp0 != 0 {fTemp251} else {fTemp579 - F32::floor(fTemp579)};
																																																																																																																																														fTemp580 = self.fConst2 * fTemp365 * F32::exp(0.0005776227 * (0.36 * fTemp575 + fTemp577));
																																																																																																																																														fTemp581 = fTemp580 + self.fRec72[1];
																																																																																																																																														self.fRec72[0] = if iTemp0 != 0 {fTemp255} else {fTemp581 - F32::floor(fTemp581)};
																																																																																																																																														fTemp582 = self.fConst2 * fTemp365 * F32::exp(0.0005776227 * ((i32::wrapping_mul(0, iTemp576)) as F32 - fTemp575)) + self.fRec73[1];
																																																																																																																																														self.fRec73[0] = if iTemp0 != 0 {fTemp258} else {fTemp582 - F32::floor(fTemp582)};
																																																																																																																																														self.fVec70[0] = fTemp470 * fTemp573 + 0.5 * fTemp572 * (wt_basic(iSlow294, self.fConst0 * F32::abs(self.fConst2 * fTemp365 * F32::exp(-0.0005776227 * fTemp575)), self.fConst0, self.fRec73[0]) + wt_basic(iSlow294, self.fConst0 * F32::abs(fTemp580), self.fConst0, self.fRec72[0]) + wt_basic(iSlow294, self.fConst0 * F32::abs(fTemp578), self.fConst0, self.fRec71[0]));
																																																																																																																																														fTemp583 = fSlow176 * (self.fVec70[1] / F32::sqrt(OscDsp_faustpower2_f(fTemp573) + 0.75 * OscDsp_faustpower2_f(fTemp572)));
																																																																																																																																													}
																																																																																																																																														if iSlow102 & iSlow144 != 0 {
																																																																																																																																															fTemp584 = fSlow143 * fSlow325 * (fSlow324 * fTemp583 + fSlow323 * fTemp571 + fSlow322 * fTemp550 + fSlow321 * fTemp518 + fSlow320 * fTemp515 + fSlow306 * fTemp483 + fSlow305 * fTemp471 + fSlow304 * fTemp465 + fSlow303 * fTemp442 + fSlow302 * fTemp424 + fSlow295 * fTemp407 + fSlow293 * fTemp398);
																																																																																																																																														}
																																																																																																																																															if (iSlow102 & (iSlow99 & iSlow213)) | ((iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow189)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow99 & iSlow180))))))))))))))) != 0 {
																																																																																																																																																self.fVec71[0] = fSlow327;
																																																																																																																																																fTemp585 = if ((fSlow327 != self.fVec71[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec74[1] + -1.0};
																																																																																																																																																self.fRec74[0] = fTemp585;
																																																																																																																																																self.fRec75[0] = if iTemp0 != 0 {fSlow327} else {if (fTemp585 > 0.0) as i32 != 0 {self.fRec75[1] + (fSlow327 - self.fRec75[1]) / F32::max(1.0, fTemp585)} else {fSlow327}};
																																																																																																																																																fTemp586 = F32::max(0.0, F32::min(1.0, self.fRec75[0]));
																																																																																																																																																iTemp587 = (fTemp586 > 0.0) as i32;
																																																																																																																																																fTemp588 = (iTemp587) as F32;
																																																																																																																																																iTemp589 = (fTemp588 != 0.0) as i32;
																																																																																																																																															}
																																																																																																																																																if (iSlow102 & (iSlow99 & iSlow213)) | ((iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow99 & iSlow180)))))))))))))) != 0 {
																																																																																																																																																	fTemp590 = 256.0 * fTemp365;
																																																																																																																																																	iTemp591 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp590 > self.fConst16) as i32) + ((fTemp590 > self.fConst22) as i32), (fTemp590 > self.fConst21) as i32), (fTemp590 > self.fConst20) as i32), (fTemp590 > self.fConst19) as i32), (fTemp590 > self.fConst18) as i32), (fTemp590 > self.fConst17) as i32);
																																																																																																																																																	iTemp592 = (iTemp591 >= 4) as i32;
																																																																																																																																																	iTemp593 = (iTemp591 >= 2) as i32;
																																																																																																																																																	iTemp594 = (iTemp591 >= 1) as i32;
																																																																																																																																																	iTemp595 = (iTemp591 >= 3) as i32;
																																																																																																																																																	iTemp596 = if iTemp592 != 0 {64} else {if iTemp593 != 0 {if iTemp595 != 0 {128} else {256}} else {if iTemp594 != 0 {512} else {1024}}};
																																																																																																																																																	fTemp597 = self.fRec51[0] * (iTemp596) as F32;
																																																																																																																																																	iTemp598 = (fTemp597) as i32;
																																																																																																																																																	iTemp599 = (i32::wrapping_add(iTemp598, 1)) % iTemp596;
																																																																																																																																																	iTemp600 = if iTemp592 != 0 {if (iTemp591 >= 6) as i32 != 0 {if (iTemp591 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp591 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp593 != 0 {if iTemp595 != 0 {1792} else {1536}} else {if iTemp594 != 0 {1024} else {0}}};
																																																																																																																																																	fTemp601 = (iTemp598) as F32;
																																																																																																																																																	fTemp602 = fTemp597 - fTemp601;
																																																																																																																																																	fTemp603 = fTemp601 + (1.0 - fTemp597);
																																																																																																																																																}
																																																																																																																																																	if (iSlow102 & (iSlow99 & (iTemp589 & iSlow213))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow210))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow108))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow105))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow95))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow207))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow204))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow201))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow198))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow195))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow192))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow186))) | ((iSlow102 & (iSlow99 & (iTemp589 & iSlow183))) | (iSlow102 & (iSlow99 & (iSlow180 & iTemp589))))))))))))))) != 0 {
																																																																																																																																																		fTemp604 = fTemp588 * (fTemp603 * wt_sample(i32::wrapping_add(iSlow328, i32::wrapping_add(iTemp598, iTemp600))) + fTemp602 * wt_sample(i32::wrapping_add(iSlow328, i32::wrapping_add(iTemp600, iTemp599))));
																																																																																																																																																	}
																																																																																																																																																		if (iSlow102 & (iSlow99 & iSlow213)) | ((iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow189)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow99 & iSlow180))))))))))))))) != 0 {
																																																																																																																																																			fTemp605 = fTemp586 * ((iTemp587 != 0) as i32) as u32 as F32;
																																																																																																																																																			self.fVec72[0] = fSlow329;
																																																																																																																																																			fTemp606 = if ((fSlow329 != self.fVec72[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec76[1] + -1.0};
																																																																																																																																																			self.fRec76[0] = fTemp606;
																																																																																																																																																			self.fRec77[0] = if iTemp0 != 0 {fSlow329} else {if (fTemp606 > 0.0) as i32 != 0 {self.fRec77[1] + (fSlow329 - self.fRec77[1]) / F32::max(1.0, fTemp606)} else {fSlow329}};
																																																																																																																																																			fTemp607 = F32::max(0.0, F32::min(3.0, self.fRec77[0]));
																																																																																																																																																			iTemp608 = std::cmp::min(2, (fTemp607) as i32);
																																																																																																																																																			iTemp609 = (iTemp608 >= 2) as i32;
																																																																																																																																																			iTemp610 = (iTemp608 >= 1) as i32;
																																																																																																																																																			iTemp611 = i32::wrapping_mul(2176, (if iTemp609 != 0 {fSlow332} else {if iTemp610 != 0 {fSlow331} else {fSlow330}}) as i32);
																																																																																																																																																			fTemp612 = (iTemp608) as F32;
																																																																																																																																																			fTemp613 = fTemp607 - fTemp612;
																																																																																																																																																			iTemp614 = i32::wrapping_mul(2176, (if iTemp609 != 0 {fSlow331} else {if iTemp610 != 0 {fSlow330} else {fSlow333}}) as i32);
																																																																																																																																																			fTemp615 = fTemp612 + (1.0 - fTemp607);
																																																																																																																																																			fTemp616 = 1.0 - fTemp586;
																																																																																																																																																		}
																																																																																																																																																			if (iSlow102 & (iSlow99 & iSlow213)) | ((iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow99 & iSlow180)))))))))))))) != 0 {
																																																																																																																																																				fTemp617 = fTemp616 * (fTemp615 * (fTemp603 * wt_sample(i32::wrapping_add(iTemp600, i32::wrapping_add(iTemp598, iTemp614))) + fTemp602 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp614, iTemp600), iTemp599))) + fTemp613 * (fTemp603 * wt_sample(i32::wrapping_add(iTemp600, i32::wrapping_add(iTemp598, iTemp611))) + fTemp602 * wt_sample(i32::wrapping_add(iTemp599, i32::wrapping_add(iTemp600, iTemp611))))) + fTemp605 * fTemp604;
																																																																																																																																																			}
																																																																																																																																																				if (iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow189)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow99 & iSlow180)))))))))))))) != 0 {
																																																																																																																																																					self.fVec73[0] = fSlow334;
																																																																																																																																																					fTemp618 = if ((fSlow334 != self.fVec73[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec78[1] + -1.0};
																																																																																																																																																					self.fRec78[0] = fTemp618;
																																																																																																																																																					self.fRec79[0] = if iTemp0 != 0 {fSlow334} else {if (fTemp618 > 0.0) as i32 != 0 {self.fRec79[1] + (fSlow334 - self.fRec79[1]) / F32::max(1.0, fTemp618)} else {fSlow334}};
																																																																																																																																																					fTemp619 = F32::max(0.0, F32::min(1.0, self.fRec79[0]));
																																																																																																																																																				}
																																																																																																																																																					if (iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow99 & iSlow180)))) != 0 {
																																																																																																																																																						fTemp620 = (1.0 - fTemp619) * fTemp617;
																																																																																																																																																					}
																																																																																																																																																						if iSlow102 & (iSlow99 & iSlow180) != 0 {
																																																																																																																																																							fTemp621 = fSlow179 * (fTemp620 + fTemp619 * if (fTemp617 > 0.0) as i32 != 0 {1.0 - fTemp617} else {-1.0 - fTemp617});
																																																																																																																																																						}
																																																																																																																																																							if (iSlow102 & (iSlow99 & iSlow198)) | (iSlow102 & (iSlow99 & iSlow183)) != 0 {
																																																																																																																																																								fTemp622 = 512.0 * fTemp365;
																																																																																																																																																								iTemp623 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp622 > self.fConst16) as i32) + ((fTemp622 > self.fConst22) as i32), (fTemp622 > self.fConst21) as i32), (fTemp622 > self.fConst20) as i32), (fTemp622 > self.fConst19) as i32), (fTemp622 > self.fConst18) as i32), (fTemp622 > self.fConst17) as i32);
																																																																																																																																																								iTemp624 = (iTemp623 >= 4) as i32;
																																																																																																																																																								iTemp625 = (iTemp623 >= 2) as i32;
																																																																																																																																																								iTemp626 = (iTemp623 >= 1) as i32;
																																																																																																																																																								iTemp627 = (iTemp623 >= 3) as i32;
																																																																																																																																																								iTemp628 = if iTemp624 != 0 {64} else {if iTemp625 != 0 {if iTemp627 != 0 {128} else {256}} else {if iTemp626 != 0 {512} else {1024}}};
																																																																																																																																																								fTemp629 = (iTemp628) as F32;
																																																																																																																																																							}
																																																																																																																																																								if iSlow102 & (iSlow99 & iSlow183) != 0 {
																																																																																																																																																									fTemp630 = fTemp629 * if iTemp368 != 0 {fTemp375} else {2.0 * fTemp460};
																																																																																																																																																									iTemp631 = (fTemp630) as i32;
																																																																																																																																																									iTemp632 = (i32::wrapping_add(iTemp631, 1)) % iTemp628;
																																																																																																																																																								}
																																																																																																																																																									if (iSlow102 & (iSlow99 & iSlow198)) | (iSlow102 & (iSlow99 & iSlow183)) != 0 {
																																																																																																																																																										iTemp633 = if iTemp624 != 0 {if (iTemp623 >= 6) as i32 != 0 {if (iTemp623 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp623 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp625 != 0 {if iTemp627 != 0 {1792} else {1536}} else {if iTemp626 != 0 {1024} else {0}}};
																																																																																																																																																									}
																																																																																																																																																										if iSlow102 & (iSlow99 & iSlow183) != 0 {
																																																																																																																																																											fTemp634 = (iTemp631) as F32;
																																																																																																																																																											fTemp635 = fTemp630 - fTemp634;
																																																																																																																																																											fTemp636 = fTemp634 + (1.0 - fTemp630);
																																																																																																																																																										}
																																																																																																																																																											if iSlow102 & (iSlow99 & (iTemp589 & iSlow183)) != 0 {
																																																																																																																																																												fTemp637 = fTemp588 * (fTemp636 * wt_sample(i32::wrapping_add(iSlow328, i32::wrapping_add(iTemp633, iTemp631))) + fTemp635 * wt_sample(i32::wrapping_add(iSlow328, i32::wrapping_add(iTemp633, iTemp632))));
																																																																																																																																																											}
																																																																																																																																																												if (iSlow102 & (iSlow99 & iSlow198)) | (iSlow102 & (iSlow99 & iSlow183)) != 0 {
																																																																																																																																																													iTemp638 = i32::wrapping_add(iTemp611, iTemp633);
																																																																																																																																																													iTemp639 = i32::wrapping_add(iTemp614, iTemp633);
																																																																																																																																																												}
																																																																																																																																																													if iSlow102 & (iSlow99 & iSlow183) != 0 {
																																																																																																																																																														fTemp640 = fSlow182 * (fTemp620 + fTemp619 * (fTemp616 * (fTemp615 * (fTemp636 * wt_sample(i32::wrapping_add(iTemp639, iTemp631)) + fTemp635 * wt_sample(i32::wrapping_add(iTemp639, iTemp632))) + fTemp613 * (fTemp636 * wt_sample(i32::wrapping_add(iTemp638, iTemp631)) + fTemp635 * wt_sample(i32::wrapping_add(iTemp638, iTemp632)))) + fTemp605 * fTemp637));
																																																																																																																																																													}
																																																																																																																																																														if iSlow102 & (iSlow99 & iSlow186) != 0 {
																																																																																																																																																															fTemp641 = fSlow185 * fTemp617 * (1.0 - fTemp619 * (1.0 - F32::sin(6.2831855 * self.fRec51[0] * (7.0 * fTemp619 + 1.0))));
																																																																																																																																																														}
																																																																																																																																																															if iSlow102 & (iSlow99 & iSlow189) != 0 {
																																																																																																																																																																fTemp642 = 2.0 * fTemp619 + 1.0;
																																																																																																																																																																fTemp643 = 256.0 * fTemp365 * fTemp642;
																																																																																																																																																																iTemp644 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp643 > self.fConst16) as i32) + ((fTemp643 > self.fConst22) as i32), (fTemp643 > self.fConst21) as i32), (fTemp643 > self.fConst20) as i32), (fTemp643 > self.fConst19) as i32), (fTemp643 > self.fConst18) as i32), (fTemp643 > self.fConst17) as i32);
																																																																																																																																																																iTemp645 = (iTemp644 >= 4) as i32;
																																																																																																																																																																iTemp646 = (iTemp644 >= 2) as i32;
																																																																																																																																																																iTemp647 = (iTemp644 >= 1) as i32;
																																																																																																																																																																iTemp648 = (iTemp644 >= 3) as i32;
																																																																																																																																																																iTemp649 = if iTemp645 != 0 {64} else {if iTemp646 != 0 {if iTemp648 != 0 {128} else {256}} else {if iTemp647 != 0 {512} else {1024}}};
																																																																																																																																																																fTemp650 = F32::powf(self.fRec51[0], fTemp642) * (iTemp649) as F32;
																																																																																																																																																																iTemp651 = (fTemp650) as i32;
																																																																																																																																																																iTemp652 = (i32::wrapping_add(iTemp651, 1)) % iTemp649;
																																																																																																																																																																iTemp653 = if iTemp645 != 0 {if (iTemp644 >= 6) as i32 != 0 {if (iTemp644 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp644 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp646 != 0 {if iTemp648 != 0 {1792} else {1536}} else {if iTemp647 != 0 {1024} else {0}}};
																																																																																																																																																																fTemp654 = (iTemp651) as F32;
																																																																																																																																																																fTemp655 = fTemp650 - fTemp654;
																																																																																																																																																																fTemp656 = fTemp654 + (1.0 - fTemp650);
																																																																																																																																																															}
																																																																																																																																																																if iSlow102 & (iSlow99 & (iTemp589 & iSlow189)) != 0 {
																																																																																																																																																																	fTemp657 = fTemp588 * (fTemp656 * wt_sample(i32::wrapping_add(iSlow328, i32::wrapping_add(iTemp651, iTemp653))) + fTemp655 * wt_sample(i32::wrapping_add(iSlow328, i32::wrapping_add(iTemp653, iTemp652))));
																																																																																																																																																																}
																																																																																																																																																																	if iSlow102 & (iSlow99 & iSlow189) != 0 {
																																																																																																																																																																		fTemp658 = fSlow188 * (fTemp616 * (fTemp615 * (fTemp656 * wt_sample(i32::wrapping_add(iTemp653, i32::wrapping_add(iTemp614, iTemp651))) + fTemp655 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp614, iTemp653), iTemp652))) + fTemp613 * (fTemp656 * wt_sample(i32::wrapping_add(iTemp653, i32::wrapping_add(iTemp611, iTemp651))) + fTemp655 * wt_sample(i32::wrapping_add(iTemp652, i32::wrapping_add(iTemp611, iTemp653))))) + fTemp605 * fTemp657);
																																																																																																																																																																	}
																																																																																																																																																																		if iSlow102 & (iSlow99 & iSlow192) != 0 {
																																																																																																																																																																			fTemp659 = fSlow191 * (fTemp620 + fTemp619 * F32::sin(7.8539815 * fTemp617));
																																																																																																																																																																		}
																																																																																																																																																																			if iSlow102 & (iSlow99 & iSlow195) != 0 {
																																																																																																																																																																				fTemp660 = 0.25 * (fTemp617 * (4.0 * fTemp619 + 1.0) + 1.0);
																																																																																																																																																																				fTemp661 = fSlow194 * (1.0 - 4.0 * F32::abs(fTemp660 + (-0.5 - F32::floor(fTemp660))));
																																																																																																																																																																			}
																																																																																																																																																																				if iSlow102 & (iSlow99 & iSlow198) != 0 {
																																																																																																																																																																					fTemp662 = 2.0 * self.fRec51[0] * fTemp629;
																																																																																																																																																																					iTemp663 = (fTemp662) as i32;
																																																																																																																																																																					iTemp664 = (i32::wrapping_add(iTemp663, 1)) % iTemp628;
																																																																																																																																																																					fTemp665 = (iTemp663) as F32;
																																																																																																																																																																					fTemp666 = fTemp662 - fTemp665;
																																																																																																																																																																					fTemp667 = fTemp665 + (1.0 - fTemp662);
																																																																																																																																																																				}
																																																																																																																																																																					if iSlow102 & (iSlow99 & (iTemp589 & iSlow198)) != 0 {
																																																																																																																																																																						fTemp668 = fTemp588 * (fTemp667 * wt_sample(i32::wrapping_add(iSlow328, i32::wrapping_add(iTemp663, iTemp633))) + fTemp666 * wt_sample(i32::wrapping_add(iSlow328, i32::wrapping_add(iTemp633, iTemp664))));
																																																																																																																																																																					}
																																																																																																																																																																						if iSlow102 & (iSlow99 & iSlow198) != 0 {
																																																																																																																																																																							fTemp669 = fSlow197 * (fTemp620 + fTemp619 * (iTemp368) as F32 * F32::abs(fTemp616 * (fTemp615 * (fTemp667 * wt_sample(i32::wrapping_add(iTemp633, i32::wrapping_add(iTemp614, iTemp663))) + fTemp666 * wt_sample(i32::wrapping_add(iTemp639, iTemp664))) + fTemp613 * (fTemp667 * wt_sample(i32::wrapping_add(iTemp633, i32::wrapping_add(iTemp611, iTemp663))) + fTemp666 * wt_sample(i32::wrapping_add(iTemp664, iTemp638)))) + fTemp605 * fTemp668));
																																																																																																																																																																						}
																																																																																																																																																																							if (iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | (iSlow102 & (iSlow99 & iSlow201))) != 0 {
																																																																																																																																																																								fTemp670 = F32::tan(self.fConst25 * F32::powf(fSlow343, fTemp619));
																																																																																																																																																																								fTemp671 = 1.0 / fTemp670;
																																																																																																																																																																								fTemp672 = (fTemp671 + 1.4142135) / fTemp670 + 1.0;
																																																																																																																																																																								self.fRec80[0] = fTemp617 - (self.fRec80[2] * ((fTemp671 + -1.4142135) / fTemp670 + 1.0) + 2.0 * self.fRec80[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp670))) / fTemp672;
																																																																																																																																																																								fTemp673 = (self.fRec80[2] + self.fRec80[0] + 2.0 * self.fRec80[1]) / fTemp672;
																																																																																																																																																																							}
																																																																																																																																																																								if iSlow102 & (iSlow99 & iSlow201) != 0 {
																																																																																																																																																																									fTemp674 = fSlow200 * fTemp673;
																																																																																																																																																																								}
																																																																																																																																																																									if iSlow102 & (iSlow99 & iSlow204) != 0 {
																																																																																																																																																																										fTemp675 = fSlow203 * fTemp673;
																																																																																																																																																																									}
																																																																																																																																																																										if iSlow102 & (iSlow99 & iSlow207) != 0 {
																																																																																																																																																																											fTemp676 = fSlow206 * fTemp673;
																																																																																																																																																																										}
																																																																																																																																																																											if (iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | (iSlow102 & (iSlow99 & iSlow95))) != 0 {
																																																																																																																																																																												fTemp677 = if iTemp7 != 0 {0.0} else {self.fRec82[1] + self.fConst2 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow348, fTemp619)), 0.0)};
																																																																																																																																																																												self.fRec82[0] = fTemp677 - F32::floor(fTemp677);
																																																																																																																																																																												self.fRec81[0] = if (self.fRec82[0] < self.fRec82[1]) as i32 != 0 {fTemp617} else {self.fRec81[1]};
																																																																																																																																																																											}
																																																																																																																																																																												if iSlow102 & (iSlow99 & iSlow95) != 0 {
																																																																																																																																																																													fTemp678 = fSlow94 * self.fRec81[0];
																																																																																																																																																																												}
																																																																																																																																																																													if iSlow102 & (iSlow99 & iSlow105) != 0 {
																																																																																																																																																																														fTemp679 = fSlow104 * self.fRec81[0];
																																																																																																																																																																													}
																																																																																																																																																																														if iSlow102 & (iSlow99 & iSlow108) != 0 {
																																																																																																																																																																															fTemp680 = fSlow107 * self.fRec81[0];
																																																																																																																																																																														}
																																																																																																																																																																															if iSlow102 & (iSlow99 & iSlow210) != 0 {
																																																																																																																																																																																fTemp681 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp619)) + -1.0));
																																																																																																																																																																																fTemp682 = fSlow209 * (F32::floor(fTemp617 * fTemp681 + 0.5) / fTemp681);
																																																																																																																																																																															}
																																																																																																																																																																																if iSlow102 & (iSlow99 & iSlow213) != 0 {
																																																																																																																																																																																	fTemp683 = fSlow212 * fTemp617;
																																																																																																																																																																																}
																																																																																																																																																																																	if iSlow102 & iSlow99 != 0 {
																																																																																																																																																																																		fTemp684 = fSlow98 * (fSlow353 * fTemp683 + fSlow352 * fTemp682 + fSlow351 * fTemp680 + fSlow350 * fTemp679 + fSlow349 * fTemp678 + fSlow346 * fTemp676 + fSlow345 * fTemp675 + fSlow344 * fTemp674 + fSlow341 * fTemp669 + fSlow340 * fTemp661 + fSlow339 * fTemp659 + fSlow338 * fTemp658 + fSlow337 * fTemp641 + fSlow336 * fTemp640 + fSlow335 * fTemp621);
																																																																																																																																																																																	}
																																																																																																																																																																																		if iSlow102 != 0 {
																																																																																																																																																																																			fTemp685 = fSlow101 * (fSlow354 * fTemp684 + fSlow326 * fTemp584);
																																																																																																																																																																																		}
																																																																																																																																																																																			*output1 = fSlow355 * fTemp685;
																																																																																																																																																																																			self.fVec74[0] = fSlow356;
																																																																																																																																																																																			let mut fTemp686: F32 = if ((fSlow356 != self.fVec74[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec83[1] + -1.0};
																																																																																																																																																																																			self.fRec83[0] = fTemp686;
																																																																																																																																																																																			self.fRec84[0] = if iTemp0 != 0 {fSlow356} else {if (fTemp686 > 0.0) as i32 != 0 {self.fRec84[1] + (fSlow356 - self.fRec84[1]) / F32::max(1.0, fTemp686)} else {fSlow356}};
																																																																																																																																																																																			self.fVec75[0] = fSlow357;
																																																																																																																																																																																			let mut fTemp687: F32 = if ((fSlow357 != self.fVec75[1]) as i32) | iTemp0 != 0 {fSlow90} else {self.fRec85[1] + -1.0};
																																																																																																																																																																																			self.fRec85[0] = fTemp687;
																																																																																																																																																																																			self.fRec86[0] = if iTemp0 != 0 {fSlow357} else {if (fTemp687 > 0.0) as i32 != 0 {self.fRec86[1] + (fSlow357 - self.fRec86[1]) / F32::max(1.0, fTemp687)} else {fSlow357}};
																																																																																																																																																																																			*output2 = fTemp23;
																																																																																																																																																																																			*output3 = fTemp365;
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
																																																																																																																																																																																					if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | ((iSlow3 & (iSlow42 & iSlow39)) | ((iSlow3 & (iSlow7 & iSlow35)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | ((iSlow3 & (iSlow7 & iSlow17)) | ((iSlow3 & (iSlow7 & iSlow14)) | (iSlow3 & (iSlow7 & iSlow11))))))))))))))))))))))))) != 0 {
																																																																																																																																																																																						self.fRec0[1] = self.fRec0[0];
																																																																																																																																																																																					}
																																																																																																																																																																																						if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow7 & iSlow35)) | ((iSlow3 & (iSlow7 & iSlow219)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow216)) | ((iSlow3 & (iSlow7 & iSlow29)) | ((iSlow3 & (iSlow7 & iSlow26)) | ((iSlow3 & (iSlow7 & iSlow23)) | ((iSlow3 & (iSlow7 & iSlow20)) | ((iSlow3 & (iSlow7 & iSlow17)) | ((iSlow3 & (iSlow7 & iSlow14)) | (iSlow3 & (iSlow7 & iSlow11)))))))))))) != 0 {
																																																																																																																																																																																							self.fVec9[1] = self.fVec9[0];
																																																																																																																																																																																							self.fRec22[1] = self.fRec22[0];
																																																																																																																																																																																							self.fRec23[1] = self.fRec23[0];
																																																																																																																																																																																							self.fVec10[1] = self.fVec10[0];
																																																																																																																																																																																							self.fRec24[1] = self.fRec24[0];
																																																																																																																																																																																							self.fRec25[1] = self.fRec25[0];
																																																																																																																																																																																						}
																																																																																																																																																																																							if iSlow3 & (iSlow7 & iSlow11) != 0 {
																																																																																																																																																																																								self.fVec11[1] = self.fVec11[0];
																																																																																																																																																																																								self.fVec12[1] = self.fVec12[0];
																																																																																																																																																																																							}
																																																																																																																																																																																								if iSlow3 & (iSlow7 & iSlow14) != 0 {
																																																																																																																																																																																									self.fRec26[1] = self.fRec26[0];
																																																																																																																																																																																									self.fVec13[1] = self.fVec13[0];
																																																																																																																																																																																								}
																																																																																																																																																																																									if (iSlow3 & (iSlow7 & iSlow20)) | (iSlow3 & (iSlow7 & iSlow17)) != 0 {
																																																																																																																																																																																										self.iRec27[1] = self.iRec27[0];
																																																																																																																																																																																									}
																																																																																																																																																																																										if iSlow3 & (iSlow7 & iSlow17) != 0 {
																																																																																																																																																																																											self.fVec14[1] = self.fVec14[0];
																																																																																																																																																																																											self.fVec15[1] = self.fVec15[0];
																																																																																																																																																																																										}
																																																																																																																																																																																											if (iSlow3 & (iSlow7 & iSlow23)) | (iSlow3 & (iSlow7 & iSlow17)) != 0 {
																																																																																																																																																																																												self.fVec16[1] = self.fVec16[0];
																																																																																																																																																																																											}
																																																																																																																																																																																												if iSlow3 & (iSlow7 & iSlow20) != 0 {
																																																																																																																																																																																													self.fVec17[1] = self.fVec17[0];
																																																																																																																																																																																													self.fVec18[1] = self.fVec18[0];
																																																																																																																																																																																													self.fVec19[1] = self.fVec19[0];
																																																																																																																																																																																												}
																																																																																																																																																																																													if iSlow3 & (iSlow7 & iSlow23) != 0 {
																																																																																																																																																																																														self.fVec20[1] = self.fVec20[0];
																																																																																																																																																																																														self.fVec21[1] = self.fVec21[0];
																																																																																																																																																																																													}
																																																																																																																																																																																														if iSlow3 & (iSlow7 & iSlow26) != 0 {
																																																																																																																																																																																															self.fRec28[1] = self.fRec28[0];
																																																																																																																																																																																															self.fVec22[1] = self.fVec22[0];
																																																																																																																																																																																															self.fRec29[1] = self.fRec29[0];
																																																																																																																																																																																															self.fVec23[1] = self.fVec23[0];
																																																																																																																																																																																														}
																																																																																																																																																																																															if (iSlow3 & (iSlow7 & iSlow87)) | ((iSlow3 & (iSlow7 & iSlow32)) | ((iSlow3 & (iSlow7 & iSlow29)) | (iSlow3 & (iSlow7 & iSlow26)))) != 0 {
																																																																																																																																																																																																self.fVec24[1] = self.fVec24[0];
																																																																																																																																																																																															}
																																																																																																																																																																																																if iSlow3 & (iSlow7 & iSlow29) != 0 {
																																																																																																																																																																																																	self.fRec30[1] = self.fRec30[0];
																																																																																																																																																																																																	self.fVec25[1] = self.fVec25[0];
																																																																																																																																																																																																	self.fRec31[1] = self.fRec31[0];
																																																																																																																																																																																																	self.fVec26[1] = self.fVec26[0];
																																																																																																																																																																																																}
																																																																																																																																																																																																	if iSlow3 & (iSlow7 & iSlow216) != 0 {
																																																																																																																																																																																																		self.fRec32[1] = self.fRec32[0];
																																																																																																																																																																																																		self.fVec27[1] = self.fVec27[0];
																																																																																																																																																																																																		self.fRec33[1] = self.fRec33[0];
																																																																																																																																																																																																		self.fVec28[1] = self.fVec28[0];
																																																																																																																																																																																																		self.fVec29[1] = self.fVec29[0];
																																																																																																																																																																																																	}
																																																																																																																																																																																																		if iSlow3 & (iSlow7 & iSlow32) != 0 {
																																																																																																																																																																																																			self.fRec34[1] = self.fRec34[0];
																																																																																																																																																																																																		}
																																																																																																																																																																																																			if iSlow3 & (iSlow7 & iSlow219) != 0 {
																																																																																																																																																																																																				self.fRec35[1] = self.fRec35[0];
																																																																																																																																																																																																				self.fRec36[1] = self.fRec36[0];
																																																																																																																																																																																																				self.fVec30[1] = self.fVec30[0];
																																																																																																																																																																																																				self.fRec37[1] = self.fRec37[0];
																																																																																																																																																																																																				self.fVec31[1] = self.fVec31[0];
																																																																																																																																																																																																				self.fVec32[1] = self.fVec32[0];
																																																																																																																																																																																																			}
																																																																																																																																																																																																				if iSlow3 & (iSlow7 & iSlow35) != 0 {
																																																																																																																																																																																																					self.fVec33[1] = self.fVec33[0];
																																																																																																																																																																																																					self.fVec34[1] = self.fVec34[0];
																																																																																																																																																																																																					self.fVec35[1] = self.fVec35[0];
																																																																																																																																																																																																					self.fVec36[1] = self.fVec36[0];
																																																																																																																																																																																																					self.fRec38[1] = self.fRec38[0];
																																																																																																																																																																																																				}
																																																																																																																																																																																																					if iSlow3 & (iSlow7 & iSlow87) != 0 {
																																																																																																																																																																																																						self.fRec39[1] = self.fRec39[0];
																																																																																																																																																																																																						self.fRec40[1] = self.fRec40[0];
																																																																																																																																																																																																						self.fRec41[1] = self.fRec41[0];
																																																																																																																																																																																																						self.fVec37[1] = self.fVec37[0];
																																																																																																																																																																																																					}
																																																																																																																																																																																																						if (iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow72)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | (iSlow3 & (iSlow42 & iSlow39))))))))))))))) != 0 {
																																																																																																																																																																																																							self.fVec38[1] = self.fVec38[0];
																																																																																																																																																																																																							self.fRec42[1] = self.fRec42[0];
																																																																																																																																																																																																							self.fRec43[1] = self.fRec43[0];
																																																																																																																																																																																																							self.fVec39[1] = self.fVec39[0];
																																																																																																																																																																																																							self.fRec44[1] = self.fRec44[0];
																																																																																																																																																																																																							self.fRec45[1] = self.fRec45[0];
																																																																																																																																																																																																						}
																																																																																																																																																																																																							if (iSlow3 & (iSlow42 & iSlow84)) | ((iSlow3 & (iSlow42 & iSlow81)) | ((iSlow3 & (iSlow42 & iSlow78)) | ((iSlow3 & (iSlow42 & iSlow75)) | ((iSlow3 & (iSlow42 & iSlow69)) | ((iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | ((iSlow3 & (iSlow42 & iSlow60)) | ((iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | ((iSlow3 & (iSlow42 & iSlow51)) | ((iSlow3 & (iSlow42 & iSlow48)) | ((iSlow3 & (iSlow42 & iSlow45)) | (iSlow3 & (iSlow42 & iSlow39)))))))))))))) != 0 {
																																																																																																																																																																																																								self.fVec40[1] = self.fVec40[0];
																																																																																																																																																																																																								self.fRec46[1] = self.fRec46[0];
																																																																																																																																																																																																								self.fRec47[1] = self.fRec47[0];
																																																																																																																																																																																																							}
																																																																																																																																																																																																								if (iSlow3 & (iSlow42 & iSlow57)) | ((iSlow3 & (iSlow42 & iSlow54)) | (iSlow3 & (iSlow42 & iSlow51))) != 0 {
																																																																																																																																																																																																									self.fRec48[2] = self.fRec48[1];
																																																																																																																																																																																																									self.fRec48[1] = self.fRec48[0];
																																																																																																																																																																																																								}
																																																																																																																																																																																																									if (iSlow3 & (iSlow42 & iSlow66)) | ((iSlow3 & (iSlow42 & iSlow63)) | (iSlow3 & (iSlow42 & iSlow60))) != 0 {
																																																																																																																																																																																																										self.fRec50[1] = self.fRec50[0];
																																																																																																																																																																																																										self.fRec49[1] = self.fRec49[0];
																																																																																																																																																																																																									}
																																																																																																																																																																																																										self.fVec41[1] = self.fVec41[0];
																																																																																																																																																																																																										self.fRec52[1] = self.fRec52[0];
																																																																																																																																																																																																										self.fRec53[1] = self.fRec53[0];
																																																																																																																																																																																																										if (iSlow102 & (iSlow99 & iSlow213)) | ((iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow189)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | ((iSlow102 & (iSlow99 & iSlow180)) | ((iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow174)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | ((iSlow102 & (iSlow144 & iSlow150)) | ((iSlow102 & (iSlow144 & iSlow147)) | (iSlow102 & (iSlow144 & iSlow141))))))))))))))))))))))))) != 0 {
																																																																																																																																																																																																											self.fRec51[1] = self.fRec51[0];
																																																																																																																																																																																																										}
																																																																																																																																																																																																											if (iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow174)) | ((iSlow102 & (iSlow144 & iSlow171)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow165)) | ((iSlow102 & (iSlow144 & iSlow162)) | ((iSlow102 & (iSlow144 & iSlow159)) | ((iSlow102 & (iSlow144 & iSlow156)) | ((iSlow102 & (iSlow144 & iSlow153)) | ((iSlow102 & (iSlow144 & iSlow150)) | ((iSlow102 & (iSlow144 & iSlow147)) | (iSlow102 & (iSlow144 & iSlow141)))))))))))) != 0 {
																																																																																																																																																																																																												self.fVec42[1] = self.fVec42[0];
																																																																																																																																																																																																												self.fRec54[1] = self.fRec54[0];
																																																																																																																																																																																																												self.fRec55[1] = self.fRec55[0];
																																																																																																																																																																																																												self.fVec43[1] = self.fVec43[0];
																																																																																																																																																																																																												self.fRec56[1] = self.fRec56[0];
																																																																																																																																																																																																												self.fRec57[1] = self.fRec57[0];
																																																																																																																																																																																																											}
																																																																																																																																																																																																												if iSlow102 & (iSlow144 & iSlow141) != 0 {
																																																																																																																																																																																																													self.fVec44[1] = self.fVec44[0];
																																																																																																																																																																																																													self.fVec45[1] = self.fVec45[0];
																																																																																																																																																																																																												}
																																																																																																																																																																																																													if iSlow102 & (iSlow144 & iSlow147) != 0 {
																																																																																																																																																																																																														self.fRec58[1] = self.fRec58[0];
																																																																																																																																																																																																														self.fVec46[1] = self.fVec46[0];
																																																																																																																																																																																																													}
																																																																																																																																																																																																														if (iSlow102 & (iSlow144 & iSlow153)) | (iSlow102 & (iSlow144 & iSlow150)) != 0 {
																																																																																																																																																																																																															self.iRec59[1] = self.iRec59[0];
																																																																																																																																																																																																														}
																																																																																																																																																																																																															if iSlow102 & (iSlow144 & iSlow150) != 0 {
																																																																																																																																																																																																																self.fVec47[1] = self.fVec47[0];
																																																																																																																																																																																																																self.fVec48[1] = self.fVec48[0];
																																																																																																																																																																																																															}
																																																																																																																																																																																																																if (iSlow102 & (iSlow144 & iSlow156)) | (iSlow102 & (iSlow144 & iSlow150)) != 0 {
																																																																																																																																																																																																																	self.fVec49[1] = self.fVec49[0];
																																																																																																																																																																																																																}
																																																																																																																																																																																																																	if iSlow102 & (iSlow144 & iSlow153) != 0 {
																																																																																																																																																																																																																		self.fVec50[1] = self.fVec50[0];
																																																																																																																																																																																																																		self.fVec51[1] = self.fVec51[0];
																																																																																																																																																																																																																		self.fVec52[1] = self.fVec52[0];
																																																																																																																																																																																																																	}
																																																																																																																																																																																																																		if iSlow102 & (iSlow144 & iSlow156) != 0 {
																																																																																																																																																																																																																			self.fVec53[1] = self.fVec53[0];
																																																																																																																																																																																																																			self.fVec54[1] = self.fVec54[0];
																																																																																																																																																																																																																		}
																																																																																																																																																																																																																			if iSlow102 & (iSlow144 & iSlow159) != 0 {
																																																																																																																																																																																																																				self.fRec60[1] = self.fRec60[0];
																																																																																																																																																																																																																				self.fVec55[1] = self.fVec55[0];
																																																																																																																																																																																																																				self.fRec61[1] = self.fRec61[0];
																																																																																																																																																																																																																				self.fVec56[1] = self.fVec56[0];
																																																																																																																																																																																																																			}
																																																																																																																																																																																																																				if (iSlow102 & (iSlow144 & iSlow177)) | ((iSlow102 & (iSlow144 & iSlow168)) | ((iSlow102 & (iSlow144 & iSlow162)) | (iSlow102 & (iSlow144 & iSlow159)))) != 0 {
																																																																																																																																																																																																																					self.fVec57[1] = self.fVec57[0];
																																																																																																																																																																																																																				}
																																																																																																																																																																																																																					if iSlow102 & (iSlow144 & iSlow162) != 0 {
																																																																																																																																																																																																																						self.fRec62[1] = self.fRec62[0];
																																																																																																																																																																																																																						self.fVec58[1] = self.fVec58[0];
																																																																																																																																																																																																																						self.fRec63[1] = self.fRec63[0];
																																																																																																																																																																																																																						self.fVec59[1] = self.fVec59[0];
																																																																																																																																																																																																																					}
																																																																																																																																																																																																																						if iSlow102 & (iSlow144 & iSlow165) != 0 {
																																																																																																																																																																																																																							self.fRec64[1] = self.fRec64[0];
																																																																																																																																																																																																																							self.fVec60[1] = self.fVec60[0];
																																																																																																																																																																																																																							self.fRec65[1] = self.fRec65[0];
																																																																																																																																																																																																																							self.fVec61[1] = self.fVec61[0];
																																																																																																																																																																																																																							self.fVec62[1] = self.fVec62[0];
																																																																																																																																																																																																																						}
																																																																																																																																																																																																																							if iSlow102 & (iSlow144 & iSlow168) != 0 {
																																																																																																																																																																																																																								self.fRec66[1] = self.fRec66[0];
																																																																																																																																																																																																																							}
																																																																																																																																																																																																																								if iSlow102 & (iSlow144 & iSlow171) != 0 {
																																																																																																																																																																																																																									self.fRec67[1] = self.fRec67[0];
																																																																																																																																																																																																																									self.fRec68[1] = self.fRec68[0];
																																																																																																																																																																																																																									self.fVec63[1] = self.fVec63[0];
																																																																																																																																																																																																																									self.fRec69[1] = self.fRec69[0];
																																																																																																																																																																																																																									self.fVec64[1] = self.fVec64[0];
																																																																																																																																																																																																																									self.fVec65[1] = self.fVec65[0];
																																																																																																																																																																																																																								}
																																																																																																																																																																																																																									if iSlow102 & (iSlow144 & iSlow174) != 0 {
																																																																																																																																																																																																																										self.fVec66[1] = self.fVec66[0];
																																																																																																																																																																																																																										self.fVec67[1] = self.fVec67[0];
																																																																																																																																																																																																																										self.fVec68[1] = self.fVec68[0];
																																																																																																																																																																																																																										self.fVec69[1] = self.fVec69[0];
																																																																																																																																																																																																																										self.fRec70[1] = self.fRec70[0];
																																																																																																																																																																																																																									}
																																																																																																																																																																																																																										if iSlow102 & (iSlow144 & iSlow177) != 0 {
																																																																																																																																																																																																																											self.fRec71[1] = self.fRec71[0];
																																																																																																																																																																																																																											self.fRec72[1] = self.fRec72[0];
																																																																																																																																																																																																																											self.fRec73[1] = self.fRec73[0];
																																																																																																																																																																																																																											self.fVec70[1] = self.fVec70[0];
																																																																																																																																																																																																																										}
																																																																																																																																																																																																																											if (iSlow102 & (iSlow99 & iSlow213)) | ((iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow189)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow99 & iSlow180))))))))))))))) != 0 {
																																																																																																																																																																																																																												self.fVec71[1] = self.fVec71[0];
																																																																																																																																																																																																																												self.fRec74[1] = self.fRec74[0];
																																																																																																																																																																																																																												self.fRec75[1] = self.fRec75[0];
																																																																																																																																																																																																																												self.fVec72[1] = self.fVec72[0];
																																																																																																																																																																																																																												self.fRec76[1] = self.fRec76[0];
																																																																																																																																																																																																																												self.fRec77[1] = self.fRec77[0];
																																																																																																																																																																																																																											}
																																																																																																																																																																																																																												if (iSlow102 & (iSlow99 & iSlow210)) | ((iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | ((iSlow102 & (iSlow99 & iSlow95)) | ((iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | ((iSlow102 & (iSlow99 & iSlow201)) | ((iSlow102 & (iSlow99 & iSlow198)) | ((iSlow102 & (iSlow99 & iSlow195)) | ((iSlow102 & (iSlow99 & iSlow192)) | ((iSlow102 & (iSlow99 & iSlow189)) | ((iSlow102 & (iSlow99 & iSlow186)) | ((iSlow102 & (iSlow99 & iSlow183)) | (iSlow102 & (iSlow99 & iSlow180)))))))))))))) != 0 {
																																																																																																																																																																																																																													self.fVec73[1] = self.fVec73[0];
																																																																																																																																																																																																																													self.fRec78[1] = self.fRec78[0];
																																																																																																																																																																																																																													self.fRec79[1] = self.fRec79[0];
																																																																																																																																																																																																																												}
																																																																																																																																																																																																																													if (iSlow102 & (iSlow99 & iSlow207)) | ((iSlow102 & (iSlow99 & iSlow204)) | (iSlow102 & (iSlow99 & iSlow201))) != 0 {
																																																																																																																																																																																																																														self.fRec80[2] = self.fRec80[1];
																																																																																																																																																																																																																														self.fRec80[1] = self.fRec80[0];
																																																																																																																																																																																																																													}
																																																																																																																																																																																																																														if (iSlow102 & (iSlow99 & iSlow108)) | ((iSlow102 & (iSlow99 & iSlow105)) | (iSlow102 & (iSlow99 & iSlow95))) != 0 {
																																																																																																																																																																																																																															self.fRec82[1] = self.fRec82[0];
																																																																																																																																																																																																																															self.fRec81[1] = self.fRec81[0];
																																																																																																																																																																																																																														}
																																																																																																																																																																																																																															self.fVec74[1] = self.fVec74[0];
																																																																																																																																																																																																																															self.fRec83[1] = self.fRec83[0];
																																																																																																																																																																																																																															self.fRec84[1] = self.fRec84[0];
																																																																																																																																																																																																																															self.fVec75[1] = self.fVec75[0];
																																																																																																																																																																																																																															self.fRec85[1] = self.fRec85[0];
																																																																																																																																																																																																																															self.fRec86[1] = self.fRec86[0];
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
																																																																																																																																																																																																																														self.fTempPerm244 = fTemp264;
																																																																																																																																																																																																																														self.iTempPerm245 = iTemp265;
																																																																																																																																																																																																																														self.fTempPerm246 = fTemp266;
																																																																																																																																																																																																																														self.iTempPerm247 = iTemp267;
																																																																																																																																																																																																																														self.iTempPerm248 = iTemp268;
																																																																																																																																																																																																																														self.iTempPerm249 = iTemp269;
																																																																																																																																																																																																																														self.iTempPerm250 = iTemp270;
																																																																																																																																																																																																																														self.iTempPerm251 = iTemp271;
																																																																																																																																																																																																																														self.iTempPerm252 = iTemp272;
																																																																																																																																																																																																																														self.fTempPerm253 = fTemp273;
																																																																																																																																																																																																																														self.iTempPerm254 = iTemp274;
																																																																																																																																																																																																																														self.iTempPerm255 = iTemp275;
																																																																																																																																																																																																																														self.iTempPerm256 = iTemp276;
																																																																																																																																																																																																																														self.fTempPerm257 = fTemp277;
																																																																																																																																																																																																																														self.fTempPerm258 = fTemp278;
																																																																																																																																																																																																																														self.fTempPerm259 = fTemp279;
																																																																																																																																																																																																																														self.fTempPerm260 = fTemp280;
																																																																																																																																																																																																																														self.fTempPerm261 = fTemp281;
																																																																																																																																																																																																																														self.fTempPerm262 = fTemp282;
																																																																																																																																																																																																																														self.fTempPerm263 = fTemp283;
																																																																																																																																																																																																																														self.iTempPerm264 = iTemp284;
																																																																																																																																																																																																																														self.iTempPerm265 = iTemp285;
																																																																																																																																																																																																																														self.iTempPerm266 = iTemp286;
																																																																																																																																																																																																																														self.iTempPerm267 = iTemp287;
																																																																																																																																																																																																																														self.fTempPerm268 = fTemp288;
																																																																																																																																																																																																																														self.fTempPerm269 = fTemp289;
																																																																																																																																																																																																																														self.iTempPerm270 = iTemp290;
																																																																																																																																																																																																																														self.fTempPerm271 = fTemp291;
																																																																																																																																																																																																																														self.fTempPerm272 = fTemp292;
																																																																																																																																																																																																																														self.fTempPerm273 = fTemp293;
																																																																																																																																																																																																																														self.fTempPerm274 = fTemp294;
																																																																																																																																																																																																																														self.fTempPerm275 = fTemp295;
																																																																																																																																																																																																																														self.fTempPerm276 = fTemp296;
																																																																																																																																																																																																																														self.fTempPerm277 = fTemp297;
																																																																																																																																																																																																																														self.fTempPerm278 = fTemp298;
																																																																																																																																																																																																																														self.iTempPerm279 = iTemp299;
																																																																																																																																																																																																																														self.iTempPerm280 = iTemp300;
																																																																																																																																																																																																																														self.iTempPerm281 = iTemp301;
																																																																																																																																																																																																																														self.iTempPerm282 = iTemp302;
																																																																																																																																																																																																																														self.iTempPerm283 = iTemp303;
																																																																																																																																																																																																																														self.iTempPerm284 = iTemp304;
																																																																																																																																																																																																																														self.fTempPerm285 = fTemp305;
																																																																																																																																																																																																																														self.fTempPerm286 = fTemp306;
																																																																																																																																																																																																																														self.iTempPerm287 = iTemp307;
																																																																																																																																																																																																																														self.iTempPerm288 = iTemp308;
																																																																																																																																																																																																																														self.iTempPerm289 = iTemp309;
																																																																																																																																																																																																																														self.fTempPerm290 = fTemp310;
																																																																																																																																																																																																																														self.fTempPerm291 = fTemp311;
																																																																																																																																																																																																																														self.fTempPerm292 = fTemp312;
																																																																																																																																																																																																																														self.fTempPerm293 = fTemp313;
																																																																																																																																																																																																																														self.iTempPerm294 = iTemp314;
																																																																																																																																																																																																																														self.iTempPerm295 = iTemp315;
																																																																																																																																																																																																																														self.fTempPerm296 = fTemp316;
																																																																																																																																																																																																																														self.fTempPerm297 = fTemp317;
																																																																																																																																																																																																																														self.fTempPerm298 = fTemp318;
																																																																																																																																																																																																																														self.fTempPerm299 = fTemp319;
																																																																																																																																																																																																																														self.iTempPerm300 = iTemp320;
																																																																																																																																																																																																																														self.iTempPerm301 = iTemp321;
																																																																																																																																																																																																																														self.iTempPerm302 = iTemp322;
																																																																																																																																																																																																																														self.iTempPerm303 = iTemp323;
																																																																																																																																																																																																																														self.iTempPerm304 = iTemp324;
																																																																																																																																																																																																																														self.iTempPerm305 = iTemp325;
																																																																																																																																																																																																																														self.fTempPerm306 = fTemp326;
																																																																																																																																																																																																																														self.iTempPerm307 = iTemp327;
																																																																																																																																																																																																																														self.iTempPerm308 = iTemp328;
																																																																																																																																																																																																																														self.iTempPerm309 = iTemp329;
																																																																																																																																																																																																																														self.fTempPerm310 = fTemp330;
																																																																																																																																																																																																																														self.fTempPerm311 = fTemp331;
																																																																																																																																																																																																																														self.fTempPerm312 = fTemp332;
																																																																																																																																																																																																																														self.fTempPerm313 = fTemp333;
																																																																																																																																																																																																																														self.fTempPerm314 = fTemp334;
																																																																																																																																																																																																																														self.fTempPerm315 = fTemp335;
																																																																																																																																																																																																																														self.fTempPerm316 = fTemp336;
																																																																																																																																																																																																																														self.fTempPerm317 = fTemp337;
																																																																																																																																																																																																																														self.fTempPerm318 = fTemp338;
																																																																																																																																																																																																																														self.iTempPerm319 = iTemp339;
																																																																																																																																																																																																																														self.iTempPerm320 = iTemp340;
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
																																																																																																																																																																																																																														self.fTempPerm331 = fTemp351;
																																																																																																																																																																																																																														self.fTempPerm332 = fTemp352;
																																																																																																																																																																																																																														self.fTempPerm333 = fTemp353;
																																																																																																																																																																																																																														self.fTempPerm334 = fTemp354;
																																																																																																																																																																																																																														self.fTempPerm335 = fTemp355;
																																																																																																																																																																																																																														self.fTempPerm336 = fTemp356;
																																																																																																																																																																																																																														self.fTempPerm337 = fTemp357;
																																																																																																																																																																																																																														self.fTempPerm338 = fTemp358;
																																																																																																																																																																																																																														self.fTempPerm339 = fTemp359;
																																																																																																																																																																																																																														self.fTempPerm340 = fTemp360;
																																																																																																																																																																																																																														self.fTempPerm341 = fTemp361;
																																																																																																																																																																																																																														self.fTempPerm342 = fTemp366;
																																																																																																																																																																																																																														self.fTempPerm343 = fTemp367;
																																																																																																																																																																																																																														self.iTempPerm344 = iTemp368;
																																																																																																																																																																																																																														self.fTempPerm345 = fTemp369;
																																																																																																																																																																																																																														self.fTempPerm346 = fTemp370;
																																																																																																																																																																																																																														self.fTempPerm347 = fTemp371;
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
																																																																																																																																																																																																																														self.iTempPerm358 = iTemp382;
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
																																																																																																																																																																																																																														self.fTempPerm373 = fTemp397;
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
																																																																																																																																																																																																																														self.iTempPerm384 = iTemp408;
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
																																																																																																																																																																																																																														self.fTempPerm395 = fTemp419;
																																																																																																																																																																																																																														self.fTempPerm396 = fTemp420;
																																																																																																																																																																																																																														self.fTempPerm397 = fTemp421;
																																																																																																																																																																																																																														self.fTempPerm398 = fTemp422;
																																																																																																																																																																																																																														self.fTempPerm399 = fTemp423;
																																																																																																																																																																																																																														self.fTempPerm400 = fTemp424;
																																																																																																																																																																																																																														self.fTempPerm401 = fTemp425;
																																																																																																																																																																																																																														self.fTempPerm402 = fTemp426;
																																																																																																																																																																																																																														self.fTempPerm403 = fTemp427;
																																																																																																																																																																																																																														self.fTempPerm404 = fTemp428;
																																																																																																																																																																																																																														self.fTempPerm405 = fTemp429;
																																																																																																																																																																																																																														self.iTempPerm406 = iTemp430;
																																																																																																																																																																																																																														self.fTempPerm407 = fTemp431;
																																																																																																																																																																																																																														self.fTempPerm408 = fTemp432;
																																																																																																																																																																																																																														self.fTempPerm409 = fTemp433;
																																																																																																																																																																																																																														self.fTempPerm410 = fTemp434;
																																																																																																																																																																																																																														self.fTempPerm411 = fTemp435;
																																																																																																																																																																																																																														self.iTempPerm412 = iTemp436;
																																																																																																																																																																																																																														self.fTempPerm413 = fTemp437;
																																																																																																																																																																																																																														self.fTempPerm414 = fTemp438;
																																																																																																																																																																																																																														self.fTempPerm415 = fTemp439;
																																																																																																																																																																																																																														self.fTempPerm416 = fTemp440;
																																																																																																																																																																																																																														self.fTempPerm417 = fTemp441;
																																																																																																																																																																																																																														self.fTempPerm418 = fTemp442;
																																																																																																																																																																																																																														self.fTempPerm419 = fTemp443;
																																																																																																																																																																																																																														self.iTempPerm420 = iTemp444;
																																																																																																																																																																																																																														self.iTempPerm421 = iTemp445;
																																																																																																																																																																																																																														self.iTempPerm422 = iTemp446;
																																																																																																																																																																																																																														self.iTempPerm423 = iTemp447;
																																																																																																																																																																																																																														self.iTempPerm424 = iTemp448;
																																																																																																																																																																																																																														self.iTempPerm425 = iTemp449;
																																																																																																																																																																																																																														self.fTempPerm426 = fTemp450;
																																																																																																																																																																																																																														self.fTempPerm427 = fTemp451;
																																																																																																																																																																																																																														self.fTempPerm428 = fTemp452;
																																																																																																																																																																																																																														self.iTempPerm429 = iTemp453;
																																																																																																																																																																																																																														self.iTempPerm430 = iTemp454;
																																																																																																																																																																																																																														self.iTempPerm431 = iTemp455;
																																																																																																																																																																																																																														self.iTempPerm432 = iTemp456;
																																																																																																																																																																																																																														self.fTempPerm433 = fTemp457;
																																																																																																																																																																																																																														self.fTempPerm434 = fTemp458;
																																																																																																																																																																																																																														self.fTempPerm435 = fTemp459;
																																																																																																																																																																																																																														self.fTempPerm436 = fTemp460;
																																																																																																																																																																																																																														self.fTempPerm437 = fTemp461;
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
																																																																																																																																																																																																																														self.iTempPerm448 = iTemp472;
																																																																																																																																																																																																																														self.fTempPerm449 = fTemp473;
																																																																																																																																																																																																																														self.fTempPerm450 = fTemp474;
																																																																																																																																																																																																																														self.fTempPerm451 = fTemp475;
																																																																																																																																																																																																																														self.fTempPerm452 = fTemp476;
																																																																																																																																																																																																																														self.fTempPerm453 = fTemp477;
																																																																																																																																																																																																																														self.fTempPerm454 = fTemp478;
																																																																																																																																																																																																																														self.fTempPerm455 = fTemp479;
																																																																																																																																																																																																																														self.fTempPerm456 = fTemp480;
																																																																																																																																																																																																																														self.fTempPerm457 = fTemp481;
																																																																																																																																																																																																																														self.fTempPerm458 = fTemp482;
																																																																																																																																																																																																																														self.fTempPerm459 = fTemp483;
																																																																																																																																																																																																																														self.fTempPerm460 = fTemp484;
																																																																																																																																																																																																																														self.fTempPerm461 = fTemp485;
																																																																																																																																																																																																																														self.fTempPerm462 = fTemp486;
																																																																																																																																																																																																																														self.iTempPerm463 = iTemp487;
																																																																																																																																																																																																																														self.fTempPerm464 = fTemp488;
																																																																																																																																																																																																																														self.fTempPerm465 = fTemp489;
																																																																																																																																																																																																																														self.fTempPerm466 = fTemp490;
																																																																																																																																																																																																																														self.fTempPerm467 = fTemp491;
																																																																																																																																																																																																																														self.iTempPerm468 = iTemp492;
																																																																																																																																																																																																																														self.fTempPerm469 = fTemp493;
																																																																																																																																																																																																																														self.iTempPerm470 = iTemp494;
																																																																																																																																																																																																																														self.fTempPerm471 = fTemp495;
																																																																																																																																																																																																																														self.fTempPerm472 = fTemp496;
																																																																																																																																																																																																																														self.fTempPerm473 = fTemp497;
																																																																																																																																																																																																																														self.fTempPerm474 = fTemp498;
																																																																																																																																																																																																																														self.fTempPerm475 = fTemp499;
																																																																																																																																																																																																																														self.iTempPerm476 = iTemp500;
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
																																																																																																																																																																																																																														self.fTempPerm487 = fTemp511;
																																																																																																																																																																																																																														self.fTempPerm488 = fTemp512;
																																																																																																																																																																																																																														self.fTempPerm489 = fTemp513;
																																																																																																																																																																																																																														self.fTempPerm490 = fTemp514;
																																																																																																																																																																																																																														self.fTempPerm491 = fTemp515;
																																																																																																																																																																																																																														self.fTempPerm492 = fTemp516;
																																																																																																																																																																																																																														self.fTempPerm493 = fTemp517;
																																																																																																																																																																																																																														self.fTempPerm494 = fTemp518;
																																																																																																																																																																																																																														self.fTempPerm495 = fTemp519;
																																																																																																																																																																																																																														self.fTempPerm496 = fTemp520;
																																																																																																																																																																																																																														self.fTempPerm497 = fTemp521;
																																																																																																																																																																																																																														self.iTempPerm498 = iTemp522;
																																																																																																																																																																																																																														self.fTempPerm499 = fTemp523;
																																																																																																																																																																																																																														self.iTempPerm500 = iTemp524;
																																																																																																																																																																																																																														self.fTempPerm501 = fTemp525;
																																																																																																																																																																																																																														self.iTempPerm502 = iTemp526;
																																																																																																																																																																																																																														self.fTempPerm503 = fTemp527;
																																																																																																																																																																																																																														self.iTempPerm504 = iTemp528;
																																																																																																																																																																																																																														self.fTempPerm505 = fTemp529;
																																																																																																																																																																																																																														self.fTempPerm506 = fTemp530;
																																																																																																																																																																																																																														self.fTempPerm507 = fTemp531;
																																																																																																																																																																																																																														self.fTempPerm508 = fTemp532;
																																																																																																																																																																																																																														self.iTempPerm509 = iTemp533;
																																																																																																																																																																																																																														self.fTempPerm510 = fTemp534;
																																																																																																																																																																																																																														self.fTempPerm511 = fTemp535;
																																																																																																																																																																																																																														self.fTempPerm512 = fTemp536;
																																																																																																																																																																																																																														self.fTempPerm513 = fTemp537;
																																																																																																																																																																																																																														self.fTempPerm514 = fTemp538;
																																																																																																																																																																																																																														self.fTempPerm515 = fTemp539;
																																																																																																																																																																																																																														self.fTempPerm516 = fTemp540;
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
																																																																																																																																																																																																																														self.iTempPerm527 = iTemp551;
																																																																																																																																																																																																																														self.fTempPerm528 = fTemp552;
																																																																																																																																																																																																																														self.fTempPerm529 = fTemp553;
																																																																																																																																																																																																																														self.fTempPerm530 = fTemp554;
																																																																																																																																																																																																																														self.fTempPerm531 = fTemp555;
																																																																																																																																																																																																																														self.fTempPerm532 = fTemp556;
																																																																																																																																																																																																																														self.fTempPerm533 = fTemp557;
																																																																																																																																																																																																																														self.fTempPerm534 = fTemp558;
																																																																																																																																																																																																																														self.fTempPerm535 = fTemp559;
																																																																																																																																																																																																																														self.fTempPerm536 = fTemp560;
																																																																																																																																																																																																																														self.fTempPerm537 = fTemp561;
																																																																																																																																																																																																																														self.fTempPerm538 = fTemp562;
																																																																																																																																																																																																																														self.fTempPerm539 = fTemp563;
																																																																																																																																																																																																																														self.fTempPerm540 = fTemp564;
																																																																																																																																																																																																																														self.fTempPerm541 = fTemp565;
																																																																																																																																																																																																																														self.fTempPerm542 = fTemp566;
																																																																																																																																																																																																																														self.fTempPerm543 = fTemp567;
																																																																																																																																																																																																																														self.fTempPerm544 = fTemp568;
																																																																																																																																																																																																																														self.iTempPerm545 = iTemp569;
																																																																																																																																																																																																																														self.fTempPerm546 = fTemp570;
																																																																																																																																																																																																																														self.fTempPerm547 = fTemp571;
																																																																																																																																																																																																																														self.fTempPerm548 = fTemp572;
																																																																																																																																																																																																																														self.fTempPerm549 = fTemp573;
																																																																																																																																																																																																																														self.iTempPerm550 = iTemp574;
																																																																																																																																																																																																																														self.fTempPerm551 = fTemp575;
																																																																																																																																																																																																																														self.iTempPerm552 = iTemp576;
																																																																																																																																																																																																																														self.fTempPerm553 = fTemp577;
																																																																																																																																																																																																																														self.fTempPerm554 = fTemp578;
																																																																																																																																																																																																																														self.fTempPerm555 = fTemp579;
																																																																																																																																																																																																																														self.fTempPerm556 = fTemp580;
																																																																																																																																																																																																																														self.fTempPerm557 = fTemp581;
																																																																																																																																																																																																																														self.fTempPerm558 = fTemp582;
																																																																																																																																																																																																																														self.fTempPerm559 = fTemp583;
																																																																																																																																																																																																																														self.fTempPerm560 = fTemp584;
																																																																																																																																																																																																																														self.fTempPerm561 = fTemp585;
																																																																																																																																																																																																																														self.fTempPerm562 = fTemp586;
																																																																																																																																																																																																																														self.iTempPerm563 = iTemp587;
																																																																																																																																																																																																																														self.fTempPerm564 = fTemp588;
																																																																																																																																																																																																																														self.iTempPerm565 = iTemp589;
																																																																																																																																																																																																																														self.fTempPerm566 = fTemp590;
																																																																																																																																																																																																																														self.iTempPerm567 = iTemp591;
																																																																																																																																																																																																																														self.iTempPerm568 = iTemp592;
																																																																																																																																																																																																																														self.iTempPerm569 = iTemp593;
																																																																																																																																																																																																																														self.iTempPerm570 = iTemp594;
																																																																																																																																																																																																																														self.iTempPerm571 = iTemp595;
																																																																																																																																																																																																																														self.iTempPerm572 = iTemp596;
																																																																																																																																																																																																																														self.fTempPerm573 = fTemp597;
																																																																																																																																																																																																																														self.iTempPerm574 = iTemp598;
																																																																																																																																																																																																																														self.iTempPerm575 = iTemp599;
																																																																																																																																																																																																																														self.iTempPerm576 = iTemp600;
																																																																																																																																																																																																																														self.fTempPerm577 = fTemp601;
																																																																																																																																																																																																																														self.fTempPerm578 = fTemp602;
																																																																																																																																																																																																																														self.fTempPerm579 = fTemp603;
																																																																																																																																																																																																																														self.fTempPerm580 = fTemp604;
																																																																																																																																																																																																																														self.fTempPerm581 = fTemp605;
																																																																																																																																																																																																																														self.fTempPerm582 = fTemp606;
																																																																																																																																																																																																																														self.fTempPerm583 = fTemp607;
																																																																																																																																																																																																																														self.iTempPerm584 = iTemp608;
																																																																																																																																																																																																																														self.iTempPerm585 = iTemp609;
																																																																																																																																																																																																																														self.iTempPerm586 = iTemp610;
																																																																																																																																																																																																																														self.iTempPerm587 = iTemp611;
																																																																																																																																																																																																																														self.fTempPerm588 = fTemp612;
																																																																																																																																																																																																																														self.fTempPerm589 = fTemp613;
																																																																																																																																																																																																																														self.iTempPerm590 = iTemp614;
																																																																																																																																																																																																																														self.fTempPerm591 = fTemp615;
																																																																																																																																																																																																																														self.fTempPerm592 = fTemp616;
																																																																																																																																																																																																																														self.fTempPerm593 = fTemp617;
																																																																																																																																																																																																																														self.fTempPerm594 = fTemp618;
																																																																																																																																																																																																																														self.fTempPerm595 = fTemp619;
																																																																																																																																																																																																																														self.fTempPerm596 = fTemp620;
																																																																																																																																																																																																																														self.fTempPerm597 = fTemp621;
																																																																																																																																																																																																																														self.fTempPerm598 = fTemp622;
																																																																																																																																																																																																																														self.iTempPerm599 = iTemp623;
																																																																																																																																																																																																																														self.iTempPerm600 = iTemp624;
																																																																																																																																																																																																																														self.iTempPerm601 = iTemp625;
																																																																																																																																																																																																																														self.iTempPerm602 = iTemp626;
																																																																																																																																																																																																																														self.iTempPerm603 = iTemp627;
																																																																																																																																																																																																																														self.iTempPerm604 = iTemp628;
																																																																																																																																																																																																																														self.fTempPerm605 = fTemp629;
																																																																																																																																																																																																																														self.fTempPerm606 = fTemp630;
																																																																																																																																																																																																																														self.iTempPerm607 = iTemp631;
																																																																																																																																																																																																																														self.iTempPerm608 = iTemp632;
																																																																																																																																																																																																																														self.iTempPerm609 = iTemp633;
																																																																																																																																																																																																																														self.fTempPerm610 = fTemp634;
																																																																																																																																																																																																																														self.fTempPerm611 = fTemp635;
																																																																																																																																																																																																																														self.fTempPerm612 = fTemp636;
																																																																																																																																																																																																																														self.fTempPerm613 = fTemp637;
																																																																																																																																																																																																																														self.iTempPerm614 = iTemp638;
																																																																																																																																																																																																																														self.iTempPerm615 = iTemp639;
																																																																																																																																																																																																																														self.fTempPerm616 = fTemp640;
																																																																																																																																																																																																																														self.fTempPerm617 = fTemp641;
																																																																																																																																																																																																																														self.fTempPerm618 = fTemp642;
																																																																																																																																																																																																																														self.fTempPerm619 = fTemp643;
																																																																																																																																																																																																																														self.iTempPerm620 = iTemp644;
																																																																																																																																																																																																																														self.iTempPerm621 = iTemp645;
																																																																																																																																																																																																																														self.iTempPerm622 = iTemp646;
																																																																																																																																																																																																																														self.iTempPerm623 = iTemp647;
																																																																																																																																																																																																																														self.iTempPerm624 = iTemp648;
																																																																																																																																																																																																																														self.iTempPerm625 = iTemp649;
																																																																																																																																																																																																																														self.fTempPerm626 = fTemp650;
																																																																																																																																																																																																																														self.iTempPerm627 = iTemp651;
																																																																																																																																																																																																																														self.iTempPerm628 = iTemp652;
																																																																																																																																																																																																																														self.iTempPerm629 = iTemp653;
																																																																																																																																																																																																																														self.fTempPerm630 = fTemp654;
																																																																																																																																																																																																																														self.fTempPerm631 = fTemp655;
																																																																																																																																																																																																																														self.fTempPerm632 = fTemp656;
																																																																																																																																																																																																																														self.fTempPerm633 = fTemp657;
																																																																																																																																																																																																																														self.fTempPerm634 = fTemp658;
																																																																																																																																																																																																																														self.fTempPerm635 = fTemp659;
																																																																																																																																																																																																																														self.fTempPerm636 = fTemp660;
																																																																																																																																																																																																																														self.fTempPerm637 = fTemp661;
																																																																																																																																																																																																																														self.fTempPerm638 = fTemp662;
																																																																																																																																																																																																																														self.iTempPerm639 = iTemp663;
																																																																																																																																																																																																																														self.iTempPerm640 = iTemp664;
																																																																																																																																																																																																																														self.fTempPerm641 = fTemp665;
																																																																																																																																																																																																																														self.fTempPerm642 = fTemp666;
																																																																																																																																																																																																																														self.fTempPerm643 = fTemp667;
																																																																																																																																																																																																																														self.fTempPerm644 = fTemp668;
																																																																																																																																																																																																																														self.fTempPerm645 = fTemp669;
																																																																																																																																																																																																																														self.fTempPerm646 = fTemp670;
																																																																																																																																																																																																																														self.fTempPerm647 = fTemp671;
																																																																																																																																																																																																																														self.fTempPerm648 = fTemp672;
																																																																																																																																																																																																																														self.fTempPerm649 = fTemp673;
																																																																																																																																																																																																																														self.fTempPerm650 = fTemp674;
																																																																																																																																																																																																																														self.fTempPerm651 = fTemp675;
																																																																																																																																																																																																																														self.fTempPerm652 = fTemp676;
																																																																																																																																																																																																																														self.fTempPerm653 = fTemp677;
																																																																																																																																																																																																																														self.fTempPerm654 = fTemp678;
																																																																																																																																																																																																																														self.fTempPerm655 = fTemp679;
																																																																																																																																																																																																																														self.fTempPerm656 = fTemp680;
																																																																																																																																																																																																																														self.fTempPerm657 = fTemp681;
																																																																																																																																																																																																																														self.fTempPerm658 = fTemp682;
																																																																																																																																																																																																																														self.fTempPerm659 = fTemp683;
																																																																																																																																																																																																																														self.fTempPerm660 = fTemp684;
																																																																																																																																																																																																																														self.fTempPerm661 = fTemp685;
																																																																																																																																																																																																																													}

}

