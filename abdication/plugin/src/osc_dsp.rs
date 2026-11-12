/* ------------------------------------------------------------
name: "plugin-osc"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn OscDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

static mut fOscDspSIG0Wave0: [F32;7] = [-1.0,-0.55,-0.22,0.0,0.22,0.55,1.0];

pub struct OscDspSIG0 {
	fOscDspSIG0Wave0_idx: i32,
}

impl OscDspSIG0 {
	
	fn get_num_inputsOscDspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG0(&mut self, sample_rate: i32) {
		self.fOscDspSIG0Wave0_idx = 0;
	}
	
	fn fillOscDspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			table[i1 as usize] = unsafe { fOscDspSIG0Wave0[self.fOscDspSIG0Wave0_idx as usize] };
			self.fOscDspSIG0Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG0Wave0_idx)) % 7;
		}
	}

}


pub fn newOscDspSIG0() -> OscDspSIG0 { 
	OscDspSIG0 {
		fOscDspSIG0Wave0_idx: 0,
	}
}
static mut fOscDspSIG1Wave0: [F32;7] = [-1.0,0.0,-0.4,0.0,0.4,0.0,1.0];

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
			self.fOscDspSIG1Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG1Wave0_idx)) % 7;
		}
	}

}


pub fn newOscDspSIG1() -> OscDspSIG1 { 
	OscDspSIG1 {
		fOscDspSIG1Wave0_idx: 0,
	}
}

pub struct OscDspSIG2 {
	iVec4: [i32;2],
	iRec5: [i32;2],
}

impl OscDspSIG2 {
	
	fn get_num_inputsOscDspSIG2(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG2(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG2(&mut self, sample_rate: i32) {
		for l8 in 0..2 {
			self.iVec4[l8 as usize] = 0;
		}
		for l9 in 0..2 {
			self.iRec5[l9 as usize] = 0;
		}
	}
	
	fn fillOscDspSIG2(&mut self, count: i32, table: &mut[F32]) {
		for i3 in 0..count {
			self.iVec4[0] = 1;
			self.iRec5[0] = (i32::wrapping_add(self.iVec4[1], self.iRec5[1])) % 65536;
			table[i3 as usize] = F32::sin(9.58738e-05 * (self.iRec5[0]) as F32);
			self.iVec4[1] = self.iVec4[0];
			self.iRec5[1] = self.iRec5[0];
		}
	}

}


pub fn newOscDspSIG2() -> OscDspSIG2 { 
	OscDspSIG2 {
		iVec4: [0;2],
		iRec5: [0;2],
	}
}
static mut fOscDspSIG3Wave0: [F32;12] = [0.5,1.0,1.5,2.0,2.5,3.0,3.5,4.0,5.0,6.0,7.0,8.0];

pub struct OscDspSIG3 {
	fOscDspSIG3Wave0_idx: i32,
}

impl OscDspSIG3 {
	
	fn get_num_inputsOscDspSIG3(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG3(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG3(&mut self, sample_rate: i32) {
		self.fOscDspSIG3Wave0_idx = 0;
	}
	
	fn fillOscDspSIG3(&mut self, count: i32, table: &mut[F32]) {
		for i4 in 0..count {
			table[i4 as usize] = unsafe { fOscDspSIG3Wave0[self.fOscDspSIG3Wave0_idx as usize] };
			self.fOscDspSIG3Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG3Wave0_idx)) % 12;
		}
	}

}


pub fn newOscDspSIG3() -> OscDspSIG3 { 
	OscDspSIG3 {
		fOscDspSIG3Wave0_idx: 0,
	}
}
static mut fOscDspSIG4Wave0: [F32;10] = [0.5,1.0,1.5,2.0,3.0,4.0,5.0,6.0,7.0,8.0];

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
			self.fOscDspSIG4Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG4Wave0_idx)) % 10;
		}
	}

}


pub fn newOscDspSIG4() -> OscDspSIG4 { 
	OscDspSIG4 {
		fOscDspSIG4Wave0_idx: 0,
	}
}
static mut iOscDspSIG5Wave0: [i32;5] = [3,4,5,7,12];

pub struct OscDspSIG5 {
	iOscDspSIG5Wave0_idx: i32,
}

impl OscDspSIG5 {
	
	fn get_num_inputsOscDspSIG5(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG5(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG5(&mut self, sample_rate: i32) {
		self.iOscDspSIG5Wave0_idx = 0;
	}
	
	fn fillOscDspSIG5(&mut self, count: i32, table: &mut[i32]) {
		for i6 in 0..count {
			table[i6 as usize] = unsafe { iOscDspSIG5Wave0[self.iOscDspSIG5Wave0_idx as usize] };
			self.iOscDspSIG5Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG5Wave0_idx)) % 5;
		}
	}

}


pub fn newOscDspSIG5() -> OscDspSIG5 { 
	OscDspSIG5 {
		iOscDspSIG5Wave0_idx: 0,
	}
}
static mut fOscDspSIG6Wave0: [F32;12] = [1.07,0.76,1.24,1.26,0.98,1.13,1.43,1.72,0.77,0.88,0.88,0.76];

pub struct OscDspSIG6 {
	fOscDspSIG6Wave0_idx: i32,
}

impl OscDspSIG6 {
	
	fn get_num_inputsOscDspSIG6(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG6(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG6(&mut self, sample_rate: i32) {
		self.fOscDspSIG6Wave0_idx = 0;
	}
	
	fn fillOscDspSIG6(&mut self, count: i32, table: &mut[F32]) {
		for i7 in 0..count {
			table[i7 as usize] = unsafe { fOscDspSIG6Wave0[self.fOscDspSIG6Wave0_idx as usize] };
			self.fOscDspSIG6Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG6Wave0_idx)) % 12;
		}
	}

}


pub fn newOscDspSIG6() -> OscDspSIG6 { 
	OscDspSIG6 {
		fOscDspSIG6Wave0_idx: 0,
	}
}
static mut iOscDspSIG7Wave0: [i32;7] = [-12,-6,-3,0,-3,-6,-12];

pub struct OscDspSIG7 {
	iOscDspSIG7Wave0_idx: i32,
}

impl OscDspSIG7 {
	
	fn get_num_inputsOscDspSIG7(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG7(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG7(&mut self, sample_rate: i32) {
		self.iOscDspSIG7Wave0_idx = 0;
	}
	
	fn fillOscDspSIG7(&mut self, count: i32, table: &mut[i32]) {
		for i8 in 0..count {
			table[i8 as usize] = unsafe { iOscDspSIG7Wave0[self.iOscDspSIG7Wave0_idx as usize] };
			self.iOscDspSIG7Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG7Wave0_idx)) % 7;
		}
	}

}


pub fn newOscDspSIG7() -> OscDspSIG7 { 
	OscDspSIG7 {
		iOscDspSIG7Wave0_idx: 0,
	}
}
static mut iOscDspSIG8Wave0: [i32;7] = [-9,-90,-3,0,-3,-90,-9];

pub struct OscDspSIG8 {
	iOscDspSIG8Wave0_idx: i32,
}

impl OscDspSIG8 {
	
	fn get_num_inputsOscDspSIG8(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG8(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG8(&mut self, sample_rate: i32) {
		self.iOscDspSIG8Wave0_idx = 0;
	}
	
	fn fillOscDspSIG8(&mut self, count: i32, table: &mut[i32]) {
		for i9 in 0..count {
			table[i9 as usize] = unsafe { iOscDspSIG8Wave0[self.iOscDspSIG8Wave0_idx as usize] };
			self.iOscDspSIG8Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG8Wave0_idx)) % 7;
		}
	}

}


pub fn newOscDspSIG8() -> OscDspSIG8 { 
	OscDspSIG8 {
		iOscDspSIG8Wave0_idx: 0,
	}
}
static mut ftbl0OscDspSIG0: [F32;7] = [0.0;7];
static mut ftbl1OscDspSIG1: [F32;7] = [0.0;7];
static mut ftbl2OscDspSIG2: [F32;65536] = [0.0;65536];
fn OscDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut ftbl3OscDspSIG3: [F32;12] = [0.0;12];
static mut ftbl4OscDspSIG4: [F32;10] = [0.0;10];
static mut itbl5OscDspSIG5: [i32;5] = [0;5];
static mut ftbl6OscDspSIG6: [F32;12] = [0.0;12];
static mut itbl7OscDspSIG7: [i32;7] = [0;7];
static mut itbl8OscDspSIG8: [i32;7] = [0;7];

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct OscDsp {
	fVslider0: F32,
	fEntry0: F32,
	fEntry1: F32,
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	fHslider0: F32,
	fEntry2: F32,
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
	fVslider1: F32,
	fEntry5: F32,
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
	fEntry6: F32,
	fHslider7: F32,
	fVec7: [F32;2],
	fRec16: [F32;2],
	fRec17: [F32;2],
	fHslider8: F32,
	fHslider9: F32,
	fRec18: [F32;2],
	fEntry7: F32,
	fEntry8: F32,
	fTempPerm7: F32,
	fTempPerm8: F32,
	fRec19: [F32;2],
	fHslider10: F32,
	fCheckbox0: F32,
	fEntry9: F32,
	fTempPerm9: F32,
	fRec0: [F32;2],
	iTempPerm10: i32,
	fHslider11: F32,
	fVec8: [F32;2],
	fTempPerm11: F32,
	fRec20: [F32;2],
	fRec21: [F32;2],
	fTempPerm12: F32,
	fHslider12: F32,
	fVec9: [F32;2],
	fTempPerm13: F32,
	fRec22: [F32;2],
	fRec23: [F32;2],
	fTempPerm14: F32,
	fTempPerm15: F32,
	fTempPerm16: F32,
	fTempPerm17: F32,
	fTempPerm18: F32,
	fTempPerm19: F32,
	fConst9: F32,
	fTempPerm20: F32,
	fEntry10: F32,
	fEntry11: F32,
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
	fVec10: [F32;2],
	fVec11: [F32;2],
	fTempPerm40: F32,
	fConst12: F32,
	fTempPerm41: F32,
	fTempPerm42: F32,
	fTempPerm43: F32,
	fTempPerm44: F32,
	fTempPerm45: F32,
	fRec24: [F32;2],
	fTempPerm46: F32,
	fTempPerm47: F32,
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	fTempPerm51: F32,
	fVec12: [F32;2],
	fTempPerm52: F32,
	iTempPerm53: i32,
	iRec25: [i32;2],
	fVec13: [F32;2],
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
	fVec14: [F32;2],
	fTempPerm67: F32,
	fTempPerm68: F32,
	fVec15: [F32;2],
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
	fVec16: [F32;2],
	fVec17: [F32;2],
	fTempPerm79: F32,
	fVec18: [F32;2],
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
	fVec19: [F32;2],
	fTempPerm105: F32,
	fTempPerm106: F32,
	fTempPerm107: F32,
	fTempPerm108: F32,
	fTempPerm109: F32,
	fVec20: [F32;2],
	fTempPerm110: F32,
	fTempPerm111: F32,
	fTempPerm112: F32,
	fTempPerm113: F32,
	fTempPerm114: F32,
	fRec26: [F32;2],
	fVec21: [F32;2],
	fTempPerm115: F32,
	fTempPerm116: F32,
	fTempPerm117: F32,
	fTempPerm118: F32,
	fRec27: [F32;2],
	fVec22: [F32;2],
	fTempPerm119: F32,
	fVec23: [F32;2],
	fTempPerm120: F32,
	iTempPerm121: i32,
	fTempPerm122: F32,
	fTempPerm123: F32,
	fTempPerm124: F32,
	fTempPerm125: F32,
	fTempPerm126: F32,
	fRec28: [F32;2],
	fVec24: [F32;2],
	fTempPerm127: F32,
	fTempPerm128: F32,
	fTempPerm129: F32,
	fTempPerm130: F32,
	fTempPerm131: F32,
	fRec29: [F32;2],
	fVec25: [F32;2],
	fTempPerm132: F32,
	fTempPerm133: F32,
	fTempPerm134: F32,
	fTempPerm135: F32,
	iTempPerm136: i32,
	fTempPerm137: F32,
	fTempPerm138: F32,
	fTempPerm139: F32,
	fRec30: [F32;2],
	fTempPerm140: F32,
	iTempPerm141: i32,
	fTempPerm142: F32,
	iTempPerm143: i32,
	fTempPerm144: F32,
	fTempPerm145: F32,
	fTempPerm146: F32,
	fVec26: [F32;2],
	fTempPerm147: F32,
	fTempPerm148: F32,
	iTempPerm149: i32,
	fTempPerm150: F32,
	fTempPerm151: F32,
	fTempPerm152: F32,
	fRec31: [F32;2],
	fTempPerm153: F32,
	fTempPerm154: F32,
	fTempPerm155: F32,
	fTempPerm156: F32,
	fTempPerm157: F32,
	fTempPerm158: F32,
	fTempPerm159: F32,
	fTempPerm160: F32,
	fTempPerm161: F32,
	fVec27: [F32;2],
	fTempPerm162: F32,
	fTempPerm163: F32,
	fVec28: [F32;2],
	fTempPerm164: F32,
	fConst14: F32,
	fTempPerm165: F32,
	fConst15: F32,
	fTempPerm166: F32,
	fTempPerm167: F32,
	fRec32: [F32;2],
	fTempPerm168: F32,
	fTempPerm169: F32,
	fTempPerm170: F32,
	fTempPerm171: F32,
	iTempPerm172: i32,
	fRec33: [F32;2],
	fRec34: [F32;2],
	fTempPerm173: F32,
	iTempPerm174: i32,
	fTempPerm175: F32,
	iTempPerm176: i32,
	fTempPerm177: F32,
	iTempPerm178: i32,
	fTempPerm179: F32,
	fTempPerm180: F32,
	fTempPerm181: F32,
	fVec29: [F32;2],
	fTempPerm182: F32,
	iTempPerm183: i32,
	fTempPerm184: F32,
	fTempPerm185: F32,
	fTempPerm186: F32,
	fRec35: [F32;2],
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
	fVec30: [F32;2],
	fTempPerm198: F32,
	fTempPerm199: F32,
	fVec31: [F32;2],
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
	fVec32: [F32;2],
	fTempPerm215: F32,
	fTempPerm216: F32,
	fVec33: [F32;2],
	fTempPerm217: F32,
	fVec34: [F32;2],
	fTempPerm218: F32,
	iTempPerm219: i32,
	fTempPerm220: F32,
	fVec35: [F32;2],
	fRec36: [F32;2],
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
	fRec37: [F32;2],
	fTempPerm232: F32,
	fTempPerm233: F32,
	fTempPerm234: F32,
	fTempPerm235: F32,
	fRec38: [F32;2],
	fTempPerm236: F32,
	fTempPerm237: F32,
	fTempPerm238: F32,
	fRec39: [F32;2],
	fVec36: [F32;2],
	fTempPerm239: F32,
	fTempPerm240: F32,
	fHslider13: F32,
	fVec37: [F32;2],
	fTempPerm241: F32,
	fRec40: [F32;2],
	fRec41: [F32;2],
	fTempPerm242: F32,
	iTempPerm243: i32,
	iTempPerm244: i32,
	iTempPerm245: i32,
	fEntry12: F32,
	fEntry13: F32,
	fEntry14: F32,
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
	fRec42: [F32;2],
	fTempPerm256: F32,
	iTempPerm257: i32,
	iTempPerm258: i32,
	fTempPerm259: F32,
	fTempPerm260: F32,
	fTempPerm261: F32,
	fTempPerm262: F32,
	fTempPerm263: F32,
	fEntry15: F32,
	iTempPerm264: i32,
	fTempPerm265: F32,
	fTempPerm266: F32,
	fHslider14: F32,
	fVec38: [F32;2],
	fTempPerm267: F32,
	fRec43: [F32;2],
	fRec44: [F32;2],
	fTempPerm268: F32,
	fTempPerm269: F32,
	fTempPerm270: F32,
	fTempPerm271: F32,
	iTempPerm272: i32,
	iTempPerm273: i32,
	iTempPerm274: i32,
	iTempPerm275: i32,
	iTempPerm276: i32,
	iTempPerm277: i32,
	iTempPerm278: i32,
	fTempPerm279: F32,
	fTempPerm280: F32,
	iTempPerm281: i32,
	iTempPerm282: i32,
	iTempPerm283: i32,
	iTempPerm284: i32,
	fTempPerm285: F32,
	fTempPerm286: F32,
	fTempPerm287: F32,
	iTempPerm288: i32,
	fTempPerm289: F32,
	fTempPerm290: F32,
	fTempPerm291: F32,
	fTempPerm292: F32,
	iTempPerm293: i32,
	iTempPerm294: i32,
	iTempPerm295: i32,
	iTempPerm296: i32,
	iTempPerm297: i32,
	iTempPerm298: i32,
	iTempPerm299: i32,
	fTempPerm300: F32,
	iTempPerm301: i32,
	iTempPerm302: i32,
	fTempPerm303: F32,
	fTempPerm304: F32,
	fTempPerm305: F32,
	fTempPerm306: F32,
	fTempPerm307: F32,
	fTempPerm308: F32,
	fTempPerm309: F32,
	fTempPerm310: F32,
	iTempPerm311: i32,
	iTempPerm312: i32,
	fTempPerm313: F32,
	fTempPerm314: F32,
	fTempPerm315: F32,
	fTempPerm316: F32,
	fConst24: F32,
	fConst25: F32,
	fTempPerm317: F32,
	fTempPerm318: F32,
	fTempPerm319: F32,
	fRec45: [F32;3],
	fTempPerm320: F32,
	fTempPerm321: F32,
	fTempPerm322: F32,
	fTempPerm323: F32,
	fTempPerm324: F32,
	fRec47: [F32;2],
	fRec46: [F32;2],
	fTempPerm325: F32,
	fTempPerm326: F32,
	fTempPerm327: F32,
	fTempPerm328: F32,
	fTempPerm329: F32,
	fTempPerm330: F32,
	fTempPerm331: F32,
	fTempPerm332: F32,
	fEntry16: F32,
	fEntry17: F32,
	fTempPerm333: F32,
	fTempPerm334: F32,
	fRec48: [F32;2],
	iTempPerm335: i32,
	fHslider15: F32,
	fVec39: [F32;2],
	fTempPerm336: F32,
	fRec49: [F32;2],
	fRec50: [F32;2],
	fTempPerm337: F32,
	fHslider16: F32,
	fVec40: [F32;2],
	fTempPerm338: F32,
	fRec51: [F32;2],
	fRec52: [F32;2],
	fTempPerm339: F32,
	fTempPerm340: F32,
	fTempPerm341: F32,
	fTempPerm342: F32,
	fTempPerm343: F32,
	fTempPerm344: F32,
	fTempPerm345: F32,
	fEntry18: F32,
	fEntry19: F32,
	fTempPerm346: F32,
	fTempPerm347: F32,
	fTempPerm348: F32,
	iTempPerm349: i32,
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
	fTempPerm363: F32,
	fTempPerm364: F32,
	fVec41: [F32;2],
	fVec42: [F32;2],
	fTempPerm365: F32,
	fTempPerm366: F32,
	fTempPerm367: F32,
	fRec53: [F32;2],
	fTempPerm368: F32,
	fTempPerm369: F32,
	fTempPerm370: F32,
	fTempPerm371: F32,
	fTempPerm372: F32,
	fTempPerm373: F32,
	fVec43: [F32;2],
	fTempPerm374: F32,
	iTempPerm375: i32,
	iRec54: [i32;2],
	fVec44: [F32;2],
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
	fTempPerm387: F32,
	fTempPerm388: F32,
	fVec45: [F32;2],
	fTempPerm389: F32,
	fTempPerm390: F32,
	fVec46: [F32;2],
	fTempPerm391: F32,
	fTempPerm392: F32,
	fTempPerm393: F32,
	fTempPerm394: F32,
	fTempPerm395: F32,
	fTempPerm396: F32,
	iTempPerm397: i32,
	fTempPerm398: F32,
	fTempPerm399: F32,
	fTempPerm400: F32,
	fVec47: [F32;2],
	fTempPerm401: F32,
	fVec48: [F32;2],
	fTempPerm402: F32,
	fVec49: [F32;2],
	fTempPerm403: F32,
	iTempPerm404: i32,
	fTempPerm405: F32,
	fTempPerm406: F32,
	fTempPerm407: F32,
	fTempPerm408: F32,
	fTempPerm409: F32,
	fTempPerm410: F32,
	fTempPerm411: F32,
	iTempPerm412: i32,
	iTempPerm413: i32,
	iTempPerm414: i32,
	iTempPerm415: i32,
	iTempPerm416: i32,
	iTempPerm417: i32,
	fTempPerm418: F32,
	fTempPerm419: F32,
	fTempPerm420: F32,
	iTempPerm421: i32,
	iTempPerm422: i32,
	iTempPerm423: i32,
	iTempPerm424: i32,
	fTempPerm425: F32,
	fTempPerm426: F32,
	fTempPerm427: F32,
	fVec50: [F32;2],
	fTempPerm428: F32,
	fTempPerm429: F32,
	fTempPerm430: F32,
	fTempPerm431: F32,
	fTempPerm432: F32,
	fVec51: [F32;2],
	fTempPerm433: F32,
	fTempPerm434: F32,
	fTempPerm435: F32,
	fRec55: [F32;2],
	fVec52: [F32;2],
	fTempPerm436: F32,
	fTempPerm437: F32,
	fRec56: [F32;2],
	fVec53: [F32;2],
	fTempPerm438: F32,
	fVec54: [F32;2],
	fTempPerm439: F32,
	iTempPerm440: i32,
	fTempPerm441: F32,
	fTempPerm442: F32,
	fTempPerm443: F32,
	fTempPerm444: F32,
	fTempPerm445: F32,
	fRec57: [F32;2],
	fVec55: [F32;2],
	fTempPerm446: F32,
	fTempPerm447: F32,
	fTempPerm448: F32,
	fTempPerm449: F32,
	fTempPerm450: F32,
	fRec58: [F32;2],
	fVec56: [F32;2],
	fTempPerm451: F32,
	fTempPerm452: F32,
	fTempPerm453: F32,
	fTempPerm454: F32,
	iTempPerm455: i32,
	fTempPerm456: F32,
	fTempPerm457: F32,
	fTempPerm458: F32,
	fRec59: [F32;2],
	fTempPerm459: F32,
	iTempPerm460: i32,
	fTempPerm461: F32,
	iTempPerm462: i32,
	fTempPerm463: F32,
	fTempPerm464: F32,
	fTempPerm465: F32,
	fVec57: [F32;2],
	fTempPerm466: F32,
	fTempPerm467: F32,
	iTempPerm468: i32,
	fTempPerm469: F32,
	fTempPerm470: F32,
	fTempPerm471: F32,
	fRec60: [F32;2],
	fTempPerm472: F32,
	fTempPerm473: F32,
	fTempPerm474: F32,
	fTempPerm475: F32,
	fTempPerm476: F32,
	fTempPerm477: F32,
	fTempPerm478: F32,
	fTempPerm479: F32,
	fTempPerm480: F32,
	fVec58: [F32;2],
	fTempPerm481: F32,
	fTempPerm482: F32,
	fVec59: [F32;2],
	fTempPerm483: F32,
	fTempPerm484: F32,
	fTempPerm485: F32,
	fRec61: [F32;2],
	fTempPerm486: F32,
	fTempPerm487: F32,
	fTempPerm488: F32,
	fTempPerm489: F32,
	iTempPerm490: i32,
	fRec62: [F32;2],
	fRec63: [F32;2],
	fTempPerm491: F32,
	iTempPerm492: i32,
	fTempPerm493: F32,
	iTempPerm494: i32,
	fTempPerm495: F32,
	iTempPerm496: i32,
	fTempPerm497: F32,
	fTempPerm498: F32,
	fTempPerm499: F32,
	fVec60: [F32;2],
	fTempPerm500: F32,
	iTempPerm501: i32,
	fTempPerm502: F32,
	fTempPerm503: F32,
	fTempPerm504: F32,
	fRec64: [F32;2],
	fTempPerm505: F32,
	fTempPerm506: F32,
	fTempPerm507: F32,
	fTempPerm508: F32,
	fTempPerm509: F32,
	fTempPerm510: F32,
	fTempPerm511: F32,
	fTempPerm512: F32,
	fTempPerm513: F32,
	fTempPerm514: F32,
	fTempPerm515: F32,
	fVec61: [F32;2],
	fTempPerm516: F32,
	fTempPerm517: F32,
	fVec62: [F32;2],
	fTempPerm518: F32,
	iTempPerm519: i32,
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
	fTempPerm530: F32,
	fTempPerm531: F32,
	fTempPerm532: F32,
	fVec63: [F32;2],
	fTempPerm533: F32,
	fTempPerm534: F32,
	fVec64: [F32;2],
	fTempPerm535: F32,
	fVec65: [F32;2],
	fTempPerm536: F32,
	iTempPerm537: i32,
	fTempPerm538: F32,
	fVec66: [F32;2],
	fRec65: [F32;2],
	fTempPerm539: F32,
	fTempPerm540: F32,
	fTempPerm541: F32,
	iTempPerm542: i32,
	fTempPerm543: F32,
	iTempPerm544: i32,
	fTempPerm545: F32,
	fTempPerm546: F32,
	fTempPerm547: F32,
	fRec66: [F32;2],
	fTempPerm548: F32,
	fTempPerm549: F32,
	fRec67: [F32;2],
	fTempPerm550: F32,
	fRec68: [F32;2],
	fVec67: [F32;2],
	fTempPerm551: F32,
	fTempPerm552: F32,
	fHslider17: F32,
	fVec68: [F32;2],
	fTempPerm553: F32,
	fRec69: [F32;2],
	fRec70: [F32;2],
	fTempPerm554: F32,
	iTempPerm555: i32,
	iTempPerm556: i32,
	iTempPerm557: i32,
	fEntry20: F32,
	fEntry21: F32,
	fEntry22: F32,
	iTempPerm558: i32,
	fTempPerm559: F32,
	iTempPerm560: i32,
	iTempPerm561: i32,
	iTempPerm562: i32,
	iTempPerm563: i32,
	iTempPerm564: i32,
	iTempPerm565: i32,
	iTempPerm566: i32,
	fTempPerm567: F32,
	fRec71: [F32;2],
	fTempPerm568: F32,
	iTempPerm569: i32,
	iTempPerm570: i32,
	fTempPerm571: F32,
	fTempPerm572: F32,
	fTempPerm573: F32,
	fTempPerm574: F32,
	fTempPerm575: F32,
	fEntry23: F32,
	iTempPerm576: i32,
	fTempPerm577: F32,
	fTempPerm578: F32,
	fHslider18: F32,
	fVec69: [F32;2],
	fTempPerm579: F32,
	fRec72: [F32;2],
	fRec73: [F32;2],
	fTempPerm580: F32,
	fTempPerm581: F32,
	fTempPerm582: F32,
	fTempPerm583: F32,
	iTempPerm584: i32,
	iTempPerm585: i32,
	iTempPerm586: i32,
	iTempPerm587: i32,
	iTempPerm588: i32,
	iTempPerm589: i32,
	iTempPerm590: i32,
	fTempPerm591: F32,
	fTempPerm592: F32,
	iTempPerm593: i32,
	iTempPerm594: i32,
	iTempPerm595: i32,
	iTempPerm596: i32,
	fTempPerm597: F32,
	fTempPerm598: F32,
	fTempPerm599: F32,
	iTempPerm600: i32,
	fTempPerm601: F32,
	fTempPerm602: F32,
	fTempPerm603: F32,
	fTempPerm604: F32,
	iTempPerm605: i32,
	iTempPerm606: i32,
	iTempPerm607: i32,
	iTempPerm608: i32,
	iTempPerm609: i32,
	iTempPerm610: i32,
	iTempPerm611: i32,
	fTempPerm612: F32,
	iTempPerm613: i32,
	iTempPerm614: i32,
	fTempPerm615: F32,
	fTempPerm616: F32,
	fTempPerm617: F32,
	fTempPerm618: F32,
	fTempPerm619: F32,
	fTempPerm620: F32,
	fTempPerm621: F32,
	fTempPerm622: F32,
	iTempPerm623: i32,
	iTempPerm624: i32,
	fTempPerm625: F32,
	fTempPerm626: F32,
	fTempPerm627: F32,
	fTempPerm628: F32,
	fTempPerm629: F32,
	fTempPerm630: F32,
	fTempPerm631: F32,
	fRec74: [F32;3],
	fTempPerm632: F32,
	fTempPerm633: F32,
	fTempPerm634: F32,
	fTempPerm635: F32,
	fTempPerm636: F32,
	fRec76: [F32;2],
	fRec75: [F32;2],
	fTempPerm637: F32,
	fTempPerm638: F32,
	fTempPerm639: F32,
	fTempPerm640: F32,
	fTempPerm641: F32,
	fTempPerm642: F32,
	fTempPerm643: F32,
	fTempPerm644: F32,
	fHslider19: F32,
	fHslider20: F32,
	fHslider21: F32,
	fHslider22: F32,
	fHslider23: F32,
	fHslider24: F32,
	fHslider25: F32,
	fHslider26: F32,
	fCheckbox1: F32,
	fEntry24: F32,
	fEntry25: F32,
	fHslider27: F32,
	fHslider28: F32,
	fHslider29: F32,
	fHslider30: F32,
	fHslider31: F32,
	fHslider32: F32,
	fHslider33: F32,
	fHslider34: F32,
	fCheckbox2: F32,
	fEntry26: F32,
	fEntry27: F32,
}

impl FaustDsp for OscDsp {
	type T = F32;
		
	fn new() -> OscDsp { 
		OscDsp {
			fVslider0: 0.0,
			fEntry0: 0.0,
			fEntry1: 0.0,
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			fHslider0: 0.0,
			fEntry2: 0.0,
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
			fVslider1: 0.0,
			fEntry5: 0.0,
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
			fEntry6: 0.0,
			fHslider7: 0.0,
			fVec7: [0.0;2],
			fRec16: [0.0;2],
			fRec17: [0.0;2],
			fHslider8: 0.0,
			fHslider9: 0.0,
			fRec18: [0.0;2],
			fEntry7: 0.0,
			fEntry8: 0.0,
			fTempPerm7: 0.0,
			fTempPerm8: 0.0,
			fRec19: [0.0;2],
			fHslider10: 0.0,
			fCheckbox0: 0.0,
			fEntry9: 0.0,
			fTempPerm9: 0.0,
			fRec0: [0.0;2],
			iTempPerm10: 0,
			fHslider11: 0.0,
			fVec8: [0.0;2],
			fTempPerm11: 0.0,
			fRec20: [0.0;2],
			fRec21: [0.0;2],
			fTempPerm12: 0.0,
			fHslider12: 0.0,
			fVec9: [0.0;2],
			fTempPerm13: 0.0,
			fRec22: [0.0;2],
			fRec23: [0.0;2],
			fTempPerm14: 0.0,
			fTempPerm15: 0.0,
			fTempPerm16: 0.0,
			fTempPerm17: 0.0,
			fTempPerm18: 0.0,
			fTempPerm19: 0.0,
			fConst9: 0.0,
			fTempPerm20: 0.0,
			fEntry10: 0.0,
			fEntry11: 0.0,
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
			fVec10: [0.0;2],
			fVec11: [0.0;2],
			fTempPerm40: 0.0,
			fConst12: 0.0,
			fTempPerm41: 0.0,
			fTempPerm42: 0.0,
			fTempPerm43: 0.0,
			fTempPerm44: 0.0,
			fTempPerm45: 0.0,
			fRec24: [0.0;2],
			fTempPerm46: 0.0,
			fTempPerm47: 0.0,
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			fVec12: [0.0;2],
			fTempPerm52: 0.0,
			iTempPerm53: 0,
			iRec25: [0;2],
			fVec13: [0.0;2],
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
			fVec14: [0.0;2],
			fTempPerm67: 0.0,
			fTempPerm68: 0.0,
			fVec15: [0.0;2],
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
			fVec16: [0.0;2],
			fVec17: [0.0;2],
			fTempPerm79: 0.0,
			fVec18: [0.0;2],
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
			fVec19: [0.0;2],
			fTempPerm105: 0.0,
			fTempPerm106: 0.0,
			fTempPerm107: 0.0,
			fTempPerm108: 0.0,
			fTempPerm109: 0.0,
			fVec20: [0.0;2],
			fTempPerm110: 0.0,
			fTempPerm111: 0.0,
			fTempPerm112: 0.0,
			fTempPerm113: 0.0,
			fTempPerm114: 0.0,
			fRec26: [0.0;2],
			fVec21: [0.0;2],
			fTempPerm115: 0.0,
			fTempPerm116: 0.0,
			fTempPerm117: 0.0,
			fTempPerm118: 0.0,
			fRec27: [0.0;2],
			fVec22: [0.0;2],
			fTempPerm119: 0.0,
			fVec23: [0.0;2],
			fTempPerm120: 0.0,
			iTempPerm121: 0,
			fTempPerm122: 0.0,
			fTempPerm123: 0.0,
			fTempPerm124: 0.0,
			fTempPerm125: 0.0,
			fTempPerm126: 0.0,
			fRec28: [0.0;2],
			fVec24: [0.0;2],
			fTempPerm127: 0.0,
			fTempPerm128: 0.0,
			fTempPerm129: 0.0,
			fTempPerm130: 0.0,
			fTempPerm131: 0.0,
			fRec29: [0.0;2],
			fVec25: [0.0;2],
			fTempPerm132: 0.0,
			fTempPerm133: 0.0,
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			iTempPerm136: 0,
			fTempPerm137: 0.0,
			fTempPerm138: 0.0,
			fTempPerm139: 0.0,
			fRec30: [0.0;2],
			fTempPerm140: 0.0,
			iTempPerm141: 0,
			fTempPerm142: 0.0,
			iTempPerm143: 0,
			fTempPerm144: 0.0,
			fTempPerm145: 0.0,
			fTempPerm146: 0.0,
			fVec26: [0.0;2],
			fTempPerm147: 0.0,
			fTempPerm148: 0.0,
			iTempPerm149: 0,
			fTempPerm150: 0.0,
			fTempPerm151: 0.0,
			fTempPerm152: 0.0,
			fRec31: [0.0;2],
			fTempPerm153: 0.0,
			fTempPerm154: 0.0,
			fTempPerm155: 0.0,
			fTempPerm156: 0.0,
			fTempPerm157: 0.0,
			fTempPerm158: 0.0,
			fTempPerm159: 0.0,
			fTempPerm160: 0.0,
			fTempPerm161: 0.0,
			fVec27: [0.0;2],
			fTempPerm162: 0.0,
			fTempPerm163: 0.0,
			fVec28: [0.0;2],
			fTempPerm164: 0.0,
			fConst14: 0.0,
			fTempPerm165: 0.0,
			fConst15: 0.0,
			fTempPerm166: 0.0,
			fTempPerm167: 0.0,
			fRec32: [0.0;2],
			fTempPerm168: 0.0,
			fTempPerm169: 0.0,
			fTempPerm170: 0.0,
			fTempPerm171: 0.0,
			iTempPerm172: 0,
			fRec33: [0.0;2],
			fRec34: [0.0;2],
			fTempPerm173: 0.0,
			iTempPerm174: 0,
			fTempPerm175: 0.0,
			iTempPerm176: 0,
			fTempPerm177: 0.0,
			iTempPerm178: 0,
			fTempPerm179: 0.0,
			fTempPerm180: 0.0,
			fTempPerm181: 0.0,
			fVec29: [0.0;2],
			fTempPerm182: 0.0,
			iTempPerm183: 0,
			fTempPerm184: 0.0,
			fTempPerm185: 0.0,
			fTempPerm186: 0.0,
			fRec35: [0.0;2],
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
			fVec30: [0.0;2],
			fTempPerm198: 0.0,
			fTempPerm199: 0.0,
			fVec31: [0.0;2],
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
			fVec32: [0.0;2],
			fTempPerm215: 0.0,
			fTempPerm216: 0.0,
			fVec33: [0.0;2],
			fTempPerm217: 0.0,
			fVec34: [0.0;2],
			fTempPerm218: 0.0,
			iTempPerm219: 0,
			fTempPerm220: 0.0,
			fVec35: [0.0;2],
			fRec36: [0.0;2],
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
			fRec37: [0.0;2],
			fTempPerm232: 0.0,
			fTempPerm233: 0.0,
			fTempPerm234: 0.0,
			fTempPerm235: 0.0,
			fRec38: [0.0;2],
			fTempPerm236: 0.0,
			fTempPerm237: 0.0,
			fTempPerm238: 0.0,
			fRec39: [0.0;2],
			fVec36: [0.0;2],
			fTempPerm239: 0.0,
			fTempPerm240: 0.0,
			fHslider13: 0.0,
			fVec37: [0.0;2],
			fTempPerm241: 0.0,
			fRec40: [0.0;2],
			fRec41: [0.0;2],
			fTempPerm242: 0.0,
			iTempPerm243: 0,
			iTempPerm244: 0,
			iTempPerm245: 0,
			fEntry12: 0.0,
			fEntry13: 0.0,
			fEntry14: 0.0,
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
			fRec42: [0.0;2],
			fTempPerm256: 0.0,
			iTempPerm257: 0,
			iTempPerm258: 0,
			fTempPerm259: 0.0,
			fTempPerm260: 0.0,
			fTempPerm261: 0.0,
			fTempPerm262: 0.0,
			fTempPerm263: 0.0,
			fEntry15: 0.0,
			iTempPerm264: 0,
			fTempPerm265: 0.0,
			fTempPerm266: 0.0,
			fHslider14: 0.0,
			fVec38: [0.0;2],
			fTempPerm267: 0.0,
			fRec43: [0.0;2],
			fRec44: [0.0;2],
			fTempPerm268: 0.0,
			fTempPerm269: 0.0,
			fTempPerm270: 0.0,
			fTempPerm271: 0.0,
			iTempPerm272: 0,
			iTempPerm273: 0,
			iTempPerm274: 0,
			iTempPerm275: 0,
			iTempPerm276: 0,
			iTempPerm277: 0,
			iTempPerm278: 0,
			fTempPerm279: 0.0,
			fTempPerm280: 0.0,
			iTempPerm281: 0,
			iTempPerm282: 0,
			iTempPerm283: 0,
			iTempPerm284: 0,
			fTempPerm285: 0.0,
			fTempPerm286: 0.0,
			fTempPerm287: 0.0,
			iTempPerm288: 0,
			fTempPerm289: 0.0,
			fTempPerm290: 0.0,
			fTempPerm291: 0.0,
			fTempPerm292: 0.0,
			iTempPerm293: 0,
			iTempPerm294: 0,
			iTempPerm295: 0,
			iTempPerm296: 0,
			iTempPerm297: 0,
			iTempPerm298: 0,
			iTempPerm299: 0,
			fTempPerm300: 0.0,
			iTempPerm301: 0,
			iTempPerm302: 0,
			fTempPerm303: 0.0,
			fTempPerm304: 0.0,
			fTempPerm305: 0.0,
			fTempPerm306: 0.0,
			fTempPerm307: 0.0,
			fTempPerm308: 0.0,
			fTempPerm309: 0.0,
			fTempPerm310: 0.0,
			iTempPerm311: 0,
			iTempPerm312: 0,
			fTempPerm313: 0.0,
			fTempPerm314: 0.0,
			fTempPerm315: 0.0,
			fTempPerm316: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fTempPerm317: 0.0,
			fTempPerm318: 0.0,
			fTempPerm319: 0.0,
			fRec45: [0.0;3],
			fTempPerm320: 0.0,
			fTempPerm321: 0.0,
			fTempPerm322: 0.0,
			fTempPerm323: 0.0,
			fTempPerm324: 0.0,
			fRec47: [0.0;2],
			fRec46: [0.0;2],
			fTempPerm325: 0.0,
			fTempPerm326: 0.0,
			fTempPerm327: 0.0,
			fTempPerm328: 0.0,
			fTempPerm329: 0.0,
			fTempPerm330: 0.0,
			fTempPerm331: 0.0,
			fTempPerm332: 0.0,
			fEntry16: 0.0,
			fEntry17: 0.0,
			fTempPerm333: 0.0,
			fTempPerm334: 0.0,
			fRec48: [0.0;2],
			iTempPerm335: 0,
			fHslider15: 0.0,
			fVec39: [0.0;2],
			fTempPerm336: 0.0,
			fRec49: [0.0;2],
			fRec50: [0.0;2],
			fTempPerm337: 0.0,
			fHslider16: 0.0,
			fVec40: [0.0;2],
			fTempPerm338: 0.0,
			fRec51: [0.0;2],
			fRec52: [0.0;2],
			fTempPerm339: 0.0,
			fTempPerm340: 0.0,
			fTempPerm341: 0.0,
			fTempPerm342: 0.0,
			fTempPerm343: 0.0,
			fTempPerm344: 0.0,
			fTempPerm345: 0.0,
			fEntry18: 0.0,
			fEntry19: 0.0,
			fTempPerm346: 0.0,
			fTempPerm347: 0.0,
			fTempPerm348: 0.0,
			iTempPerm349: 0,
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
			fTempPerm363: 0.0,
			fTempPerm364: 0.0,
			fVec41: [0.0;2],
			fVec42: [0.0;2],
			fTempPerm365: 0.0,
			fTempPerm366: 0.0,
			fTempPerm367: 0.0,
			fRec53: [0.0;2],
			fTempPerm368: 0.0,
			fTempPerm369: 0.0,
			fTempPerm370: 0.0,
			fTempPerm371: 0.0,
			fTempPerm372: 0.0,
			fTempPerm373: 0.0,
			fVec43: [0.0;2],
			fTempPerm374: 0.0,
			iTempPerm375: 0,
			iRec54: [0;2],
			fVec44: [0.0;2],
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
			fTempPerm387: 0.0,
			fTempPerm388: 0.0,
			fVec45: [0.0;2],
			fTempPerm389: 0.0,
			fTempPerm390: 0.0,
			fVec46: [0.0;2],
			fTempPerm391: 0.0,
			fTempPerm392: 0.0,
			fTempPerm393: 0.0,
			fTempPerm394: 0.0,
			fTempPerm395: 0.0,
			fTempPerm396: 0.0,
			iTempPerm397: 0,
			fTempPerm398: 0.0,
			fTempPerm399: 0.0,
			fTempPerm400: 0.0,
			fVec47: [0.0;2],
			fTempPerm401: 0.0,
			fVec48: [0.0;2],
			fTempPerm402: 0.0,
			fVec49: [0.0;2],
			fTempPerm403: 0.0,
			iTempPerm404: 0,
			fTempPerm405: 0.0,
			fTempPerm406: 0.0,
			fTempPerm407: 0.0,
			fTempPerm408: 0.0,
			fTempPerm409: 0.0,
			fTempPerm410: 0.0,
			fTempPerm411: 0.0,
			iTempPerm412: 0,
			iTempPerm413: 0,
			iTempPerm414: 0,
			iTempPerm415: 0,
			iTempPerm416: 0,
			iTempPerm417: 0,
			fTempPerm418: 0.0,
			fTempPerm419: 0.0,
			fTempPerm420: 0.0,
			iTempPerm421: 0,
			iTempPerm422: 0,
			iTempPerm423: 0,
			iTempPerm424: 0,
			fTempPerm425: 0.0,
			fTempPerm426: 0.0,
			fTempPerm427: 0.0,
			fVec50: [0.0;2],
			fTempPerm428: 0.0,
			fTempPerm429: 0.0,
			fTempPerm430: 0.0,
			fTempPerm431: 0.0,
			fTempPerm432: 0.0,
			fVec51: [0.0;2],
			fTempPerm433: 0.0,
			fTempPerm434: 0.0,
			fTempPerm435: 0.0,
			fRec55: [0.0;2],
			fVec52: [0.0;2],
			fTempPerm436: 0.0,
			fTempPerm437: 0.0,
			fRec56: [0.0;2],
			fVec53: [0.0;2],
			fTempPerm438: 0.0,
			fVec54: [0.0;2],
			fTempPerm439: 0.0,
			iTempPerm440: 0,
			fTempPerm441: 0.0,
			fTempPerm442: 0.0,
			fTempPerm443: 0.0,
			fTempPerm444: 0.0,
			fTempPerm445: 0.0,
			fRec57: [0.0;2],
			fVec55: [0.0;2],
			fTempPerm446: 0.0,
			fTempPerm447: 0.0,
			fTempPerm448: 0.0,
			fTempPerm449: 0.0,
			fTempPerm450: 0.0,
			fRec58: [0.0;2],
			fVec56: [0.0;2],
			fTempPerm451: 0.0,
			fTempPerm452: 0.0,
			fTempPerm453: 0.0,
			fTempPerm454: 0.0,
			iTempPerm455: 0,
			fTempPerm456: 0.0,
			fTempPerm457: 0.0,
			fTempPerm458: 0.0,
			fRec59: [0.0;2],
			fTempPerm459: 0.0,
			iTempPerm460: 0,
			fTempPerm461: 0.0,
			iTempPerm462: 0,
			fTempPerm463: 0.0,
			fTempPerm464: 0.0,
			fTempPerm465: 0.0,
			fVec57: [0.0;2],
			fTempPerm466: 0.0,
			fTempPerm467: 0.0,
			iTempPerm468: 0,
			fTempPerm469: 0.0,
			fTempPerm470: 0.0,
			fTempPerm471: 0.0,
			fRec60: [0.0;2],
			fTempPerm472: 0.0,
			fTempPerm473: 0.0,
			fTempPerm474: 0.0,
			fTempPerm475: 0.0,
			fTempPerm476: 0.0,
			fTempPerm477: 0.0,
			fTempPerm478: 0.0,
			fTempPerm479: 0.0,
			fTempPerm480: 0.0,
			fVec58: [0.0;2],
			fTempPerm481: 0.0,
			fTempPerm482: 0.0,
			fVec59: [0.0;2],
			fTempPerm483: 0.0,
			fTempPerm484: 0.0,
			fTempPerm485: 0.0,
			fRec61: [0.0;2],
			fTempPerm486: 0.0,
			fTempPerm487: 0.0,
			fTempPerm488: 0.0,
			fTempPerm489: 0.0,
			iTempPerm490: 0,
			fRec62: [0.0;2],
			fRec63: [0.0;2],
			fTempPerm491: 0.0,
			iTempPerm492: 0,
			fTempPerm493: 0.0,
			iTempPerm494: 0,
			fTempPerm495: 0.0,
			iTempPerm496: 0,
			fTempPerm497: 0.0,
			fTempPerm498: 0.0,
			fTempPerm499: 0.0,
			fVec60: [0.0;2],
			fTempPerm500: 0.0,
			iTempPerm501: 0,
			fTempPerm502: 0.0,
			fTempPerm503: 0.0,
			fTempPerm504: 0.0,
			fRec64: [0.0;2],
			fTempPerm505: 0.0,
			fTempPerm506: 0.0,
			fTempPerm507: 0.0,
			fTempPerm508: 0.0,
			fTempPerm509: 0.0,
			fTempPerm510: 0.0,
			fTempPerm511: 0.0,
			fTempPerm512: 0.0,
			fTempPerm513: 0.0,
			fTempPerm514: 0.0,
			fTempPerm515: 0.0,
			fVec61: [0.0;2],
			fTempPerm516: 0.0,
			fTempPerm517: 0.0,
			fVec62: [0.0;2],
			fTempPerm518: 0.0,
			iTempPerm519: 0,
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
			fTempPerm530: 0.0,
			fTempPerm531: 0.0,
			fTempPerm532: 0.0,
			fVec63: [0.0;2],
			fTempPerm533: 0.0,
			fTempPerm534: 0.0,
			fVec64: [0.0;2],
			fTempPerm535: 0.0,
			fVec65: [0.0;2],
			fTempPerm536: 0.0,
			iTempPerm537: 0,
			fTempPerm538: 0.0,
			fVec66: [0.0;2],
			fRec65: [0.0;2],
			fTempPerm539: 0.0,
			fTempPerm540: 0.0,
			fTempPerm541: 0.0,
			iTempPerm542: 0,
			fTempPerm543: 0.0,
			iTempPerm544: 0,
			fTempPerm545: 0.0,
			fTempPerm546: 0.0,
			fTempPerm547: 0.0,
			fRec66: [0.0;2],
			fTempPerm548: 0.0,
			fTempPerm549: 0.0,
			fRec67: [0.0;2],
			fTempPerm550: 0.0,
			fRec68: [0.0;2],
			fVec67: [0.0;2],
			fTempPerm551: 0.0,
			fTempPerm552: 0.0,
			fHslider17: 0.0,
			fVec68: [0.0;2],
			fTempPerm553: 0.0,
			fRec69: [0.0;2],
			fRec70: [0.0;2],
			fTempPerm554: 0.0,
			iTempPerm555: 0,
			iTempPerm556: 0,
			iTempPerm557: 0,
			fEntry20: 0.0,
			fEntry21: 0.0,
			fEntry22: 0.0,
			iTempPerm558: 0,
			fTempPerm559: 0.0,
			iTempPerm560: 0,
			iTempPerm561: 0,
			iTempPerm562: 0,
			iTempPerm563: 0,
			iTempPerm564: 0,
			iTempPerm565: 0,
			iTempPerm566: 0,
			fTempPerm567: 0.0,
			fRec71: [0.0;2],
			fTempPerm568: 0.0,
			iTempPerm569: 0,
			iTempPerm570: 0,
			fTempPerm571: 0.0,
			fTempPerm572: 0.0,
			fTempPerm573: 0.0,
			fTempPerm574: 0.0,
			fTempPerm575: 0.0,
			fEntry23: 0.0,
			iTempPerm576: 0,
			fTempPerm577: 0.0,
			fTempPerm578: 0.0,
			fHslider18: 0.0,
			fVec69: [0.0;2],
			fTempPerm579: 0.0,
			fRec72: [0.0;2],
			fRec73: [0.0;2],
			fTempPerm580: 0.0,
			fTempPerm581: 0.0,
			fTempPerm582: 0.0,
			fTempPerm583: 0.0,
			iTempPerm584: 0,
			iTempPerm585: 0,
			iTempPerm586: 0,
			iTempPerm587: 0,
			iTempPerm588: 0,
			iTempPerm589: 0,
			iTempPerm590: 0,
			fTempPerm591: 0.0,
			fTempPerm592: 0.0,
			iTempPerm593: 0,
			iTempPerm594: 0,
			iTempPerm595: 0,
			iTempPerm596: 0,
			fTempPerm597: 0.0,
			fTempPerm598: 0.0,
			fTempPerm599: 0.0,
			iTempPerm600: 0,
			fTempPerm601: 0.0,
			fTempPerm602: 0.0,
			fTempPerm603: 0.0,
			fTempPerm604: 0.0,
			iTempPerm605: 0,
			iTempPerm606: 0,
			iTempPerm607: 0,
			iTempPerm608: 0,
			iTempPerm609: 0,
			iTempPerm610: 0,
			iTempPerm611: 0,
			fTempPerm612: 0.0,
			iTempPerm613: 0,
			iTempPerm614: 0,
			fTempPerm615: 0.0,
			fTempPerm616: 0.0,
			fTempPerm617: 0.0,
			fTempPerm618: 0.0,
			fTempPerm619: 0.0,
			fTempPerm620: 0.0,
			fTempPerm621: 0.0,
			fTempPerm622: 0.0,
			iTempPerm623: 0,
			iTempPerm624: 0,
			fTempPerm625: 0.0,
			fTempPerm626: 0.0,
			fTempPerm627: 0.0,
			fTempPerm628: 0.0,
			fTempPerm629: 0.0,
			fTempPerm630: 0.0,
			fTempPerm631: 0.0,
			fRec74: [0.0;3],
			fTempPerm632: 0.0,
			fTempPerm633: 0.0,
			fTempPerm634: 0.0,
			fTempPerm635: 0.0,
			fTempPerm636: 0.0,
			fRec76: [0.0;2],
			fRec75: [0.0;2],
			fTempPerm637: 0.0,
			fTempPerm638: 0.0,
			fTempPerm639: 0.0,
			fTempPerm640: 0.0,
			fTempPerm641: 0.0,
			fTempPerm642: 0.0,
			fTempPerm643: 0.0,
			fTempPerm644: 0.0,
			fHslider19: 0.0,
			fHslider20: 0.0,
			fHslider21: 0.0,
			fHslider22: 0.0,
			fHslider23: 0.0,
			fHslider24: 0.0,
			fHslider25: 0.0,
			fHslider26: 0.0,
			fCheckbox1: 0.0,
			fEntry24: 0.0,
			fEntry25: 0.0,
			fHslider27: 0.0,
			fHslider28: 0.0,
			fHslider29: 0.0,
			fHslider30: 0.0,
			fHslider31: 0.0,
			fHslider32: 0.0,
			fHslider33: 0.0,
			fHslider34: 0.0,
			fCheckbox2: 0.0,
			fEntry26: 0.0,
			fEntry27: 0.0,
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
		return 9;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: OscDspSIG0 = newOscDspSIG0();
		sig0.instance_initOscDspSIG0(sample_rate);
		sig0.fillOscDspSIG0(7, unsafe { &mut ftbl0OscDspSIG0 });
		let mut sig1: OscDspSIG1 = newOscDspSIG1();
		sig1.instance_initOscDspSIG1(sample_rate);
		sig1.fillOscDspSIG1(7, unsafe { &mut ftbl1OscDspSIG1 });
		let mut sig2: OscDspSIG2 = newOscDspSIG2();
		sig2.instance_initOscDspSIG2(sample_rate);
		sig2.fillOscDspSIG2(65536, unsafe { &mut ftbl2OscDspSIG2 });
		let mut sig3: OscDspSIG3 = newOscDspSIG3();
		sig3.instance_initOscDspSIG3(sample_rate);
		sig3.fillOscDspSIG3(12, unsafe { &mut ftbl3OscDspSIG3 });
		let mut sig4: OscDspSIG4 = newOscDspSIG4();
		sig4.instance_initOscDspSIG4(sample_rate);
		sig4.fillOscDspSIG4(10, unsafe { &mut ftbl4OscDspSIG4 });
		let mut sig5: OscDspSIG5 = newOscDspSIG5();
		sig5.instance_initOscDspSIG5(sample_rate);
		sig5.fillOscDspSIG5(5, unsafe { &mut itbl5OscDspSIG5 });
		let mut sig6: OscDspSIG6 = newOscDspSIG6();
		sig6.instance_initOscDspSIG6(sample_rate);
		sig6.fillOscDspSIG6(12, unsafe { &mut ftbl6OscDspSIG6 });
		let mut sig7: OscDspSIG7 = newOscDspSIG7();
		sig7.instance_initOscDspSIG7(sample_rate);
		sig7.fillOscDspSIG7(7, unsafe { &mut itbl7OscDspSIG7 });
		let mut sig8: OscDspSIG8 = newOscDspSIG8();
		sig8.instance_initOscDspSIG8(sample_rate);
		sig8.fillOscDspSIG8(7, unsafe { &mut itbl8OscDspSIG8 });
	}
	fn instance_reset_params(&mut self) {
		self.fVslider0 = 1.0;
		self.fEntry0 = 0.0;
		self.fEntry1 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider0 = 3.0;
		self.fEntry2 = 0.0;
		self.fHslider1 = 6.0;
		self.fHslider2 = 64.0;
		self.fHslider3 = 0.0;
		self.fEntry3 = 0.0;
		self.fEntry4 = 1.0;
		self.fVslider1 = 0.0;
		self.fEntry5 = 0.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 1.0;
		self.fHslider6 = 0.0;
		self.fEntry6 = 0.0;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 1.1e+02;
		self.fEntry7 = 0.0;
		self.fEntry8 = 0.0;
		self.fHslider10 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fEntry9 = 0.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 0.5;
		self.fEntry10 = 2.0;
		self.fEntry11 = 2.0;
		self.fHslider13 = 0.0;
		self.fEntry12 = 4e+01;
		self.fEntry13 = 8e+01;
		self.fEntry14 = 118.0;
		self.fEntry15 = 0.0;
		self.fHslider14 = 0.5;
		self.fEntry16 = 0.0;
		self.fEntry17 = 0.0;
		self.fHslider15 = 0.0;
		self.fHslider16 = 0.5;
		self.fEntry18 = 2.0;
		self.fEntry19 = 2.0;
		self.fHslider17 = 0.0;
		self.fEntry20 = 4e+01;
		self.fEntry21 = 8e+01;
		self.fEntry22 = 118.0;
		self.fEntry23 = 0.0;
		self.fHslider18 = 0.5;
		self.fHslider19 = 0.3;
		self.fHslider20 = 0.0;
		self.fHslider21 = 0.0;
		self.fHslider22 = 0.0;
		self.fHslider23 = 0.0;
		self.fHslider24 = 0.0;
		self.fHslider25 = 0.0;
		self.fHslider26 = 0.0;
		self.fCheckbox1 = 0.0;
		self.fEntry24 = 6821.0;
		self.fEntry25 = 3.0;
		self.fHslider27 = 0.0;
		self.fHslider28 = 0.3;
		self.fHslider29 = 0.0;
		self.fHslider30 = 0.0;
		self.fHslider31 = 0.0;
		self.fHslider32 = 0.0;
		self.fHslider33 = 0.0;
		self.fHslider34 = 0.0;
		self.fCheckbox2 = 0.0;
		self.fEntry26 = 4821.0;
		self.fEntry27 = 0.0;
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
			self.fRec18[l25 as usize] = 0.0;
		}
		self.fTempPerm7 = 0.0;
		self.fTempPerm8 = 0.0;
		for l26 in 0..2 {
			self.fRec19[l26 as usize] = 0.0;
		}
		self.fTempPerm9 = 0.0;
		for l27 in 0..2 {
			self.fRec0[l27 as usize] = 0.0;
		}
		self.iTempPerm10 = 0;
		for l28 in 0..2 {
			self.fVec8[l28 as usize] = 0.0;
		}
		self.fTempPerm11 = 0.0;
		for l29 in 0..2 {
			self.fRec20[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec21[l30 as usize] = 0.0;
		}
		self.fTempPerm12 = 0.0;
		for l31 in 0..2 {
			self.fVec9[l31 as usize] = 0.0;
		}
		self.fTempPerm13 = 0.0;
		for l32 in 0..2 {
			self.fRec22[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec23[l33 as usize] = 0.0;
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
		for l34 in 0..2 {
			self.fVec10[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fVec11[l35 as usize] = 0.0;
		}
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		for l36 in 0..2 {
			self.fRec24[l36 as usize] = 0.0;
		}
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		for l37 in 0..2 {
			self.fVec12[l37 as usize] = 0.0;
		}
		self.fTempPerm52 = 0.0;
		self.iTempPerm53 = 0;
		for l38 in 0..2 {
			self.iRec25[l38 as usize] = 0;
		}
		for l39 in 0..2 {
			self.fVec13[l39 as usize] = 0.0;
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
		for l40 in 0..2 {
			self.fVec14[l40 as usize] = 0.0;
		}
		self.fTempPerm67 = 0.0;
		self.fTempPerm68 = 0.0;
		for l41 in 0..2 {
			self.fVec15[l41 as usize] = 0.0;
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
		for l42 in 0..2 {
			self.fVec16[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fVec17[l43 as usize] = 0.0;
		}
		self.fTempPerm79 = 0.0;
		for l44 in 0..2 {
			self.fVec18[l44 as usize] = 0.0;
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
		for l45 in 0..2 {
			self.fVec19[l45 as usize] = 0.0;
		}
		self.fTempPerm105 = 0.0;
		self.fTempPerm106 = 0.0;
		self.fTempPerm107 = 0.0;
		self.fTempPerm108 = 0.0;
		self.fTempPerm109 = 0.0;
		for l46 in 0..2 {
			self.fVec20[l46 as usize] = 0.0;
		}
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		self.fTempPerm112 = 0.0;
		self.fTempPerm113 = 0.0;
		self.fTempPerm114 = 0.0;
		for l47 in 0..2 {
			self.fRec26[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fVec21[l48 as usize] = 0.0;
		}
		self.fTempPerm115 = 0.0;
		self.fTempPerm116 = 0.0;
		self.fTempPerm117 = 0.0;
		self.fTempPerm118 = 0.0;
		for l49 in 0..2 {
			self.fRec27[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fVec22[l50 as usize] = 0.0;
		}
		self.fTempPerm119 = 0.0;
		for l51 in 0..2 {
			self.fVec23[l51 as usize] = 0.0;
		}
		self.fTempPerm120 = 0.0;
		self.iTempPerm121 = 0;
		self.fTempPerm122 = 0.0;
		self.fTempPerm123 = 0.0;
		self.fTempPerm124 = 0.0;
		self.fTempPerm125 = 0.0;
		self.fTempPerm126 = 0.0;
		for l52 in 0..2 {
			self.fRec28[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fVec24[l53 as usize] = 0.0;
		}
		self.fTempPerm127 = 0.0;
		self.fTempPerm128 = 0.0;
		self.fTempPerm129 = 0.0;
		self.fTempPerm130 = 0.0;
		self.fTempPerm131 = 0.0;
		for l54 in 0..2 {
			self.fRec29[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fVec25[l55 as usize] = 0.0;
		}
		self.fTempPerm132 = 0.0;
		self.fTempPerm133 = 0.0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		self.iTempPerm136 = 0;
		self.fTempPerm137 = 0.0;
		self.fTempPerm138 = 0.0;
		self.fTempPerm139 = 0.0;
		for l56 in 0..2 {
			self.fRec30[l56 as usize] = 0.0;
		}
		self.fTempPerm140 = 0.0;
		self.iTempPerm141 = 0;
		self.fTempPerm142 = 0.0;
		self.iTempPerm143 = 0;
		self.fTempPerm144 = 0.0;
		self.fTempPerm145 = 0.0;
		self.fTempPerm146 = 0.0;
		for l57 in 0..2 {
			self.fVec26[l57 as usize] = 0.0;
		}
		self.fTempPerm147 = 0.0;
		self.fTempPerm148 = 0.0;
		self.iTempPerm149 = 0;
		self.fTempPerm150 = 0.0;
		self.fTempPerm151 = 0.0;
		self.fTempPerm152 = 0.0;
		for l58 in 0..2 {
			self.fRec31[l58 as usize] = 0.0;
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
		for l59 in 0..2 {
			self.fVec27[l59 as usize] = 0.0;
		}
		self.fTempPerm162 = 0.0;
		self.fTempPerm163 = 0.0;
		for l60 in 0..2 {
			self.fVec28[l60 as usize] = 0.0;
		}
		self.fTempPerm164 = 0.0;
		self.fTempPerm165 = 0.0;
		self.fTempPerm166 = 0.0;
		self.fTempPerm167 = 0.0;
		for l61 in 0..2 {
			self.fRec32[l61 as usize] = 0.0;
		}
		self.fTempPerm168 = 0.0;
		self.fTempPerm169 = 0.0;
		self.fTempPerm170 = 0.0;
		self.fTempPerm171 = 0.0;
		self.iTempPerm172 = 0;
		for l62 in 0..2 {
			self.fRec33[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec34[l63 as usize] = 0.0;
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
		for l64 in 0..2 {
			self.fVec29[l64 as usize] = 0.0;
		}
		self.fTempPerm182 = 0.0;
		self.iTempPerm183 = 0;
		self.fTempPerm184 = 0.0;
		self.fTempPerm185 = 0.0;
		self.fTempPerm186 = 0.0;
		for l65 in 0..2 {
			self.fRec35[l65 as usize] = 0.0;
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
		for l66 in 0..2 {
			self.fVec30[l66 as usize] = 0.0;
		}
		self.fTempPerm198 = 0.0;
		self.fTempPerm199 = 0.0;
		for l67 in 0..2 {
			self.fVec31[l67 as usize] = 0.0;
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
		for l68 in 0..2 {
			self.fVec32[l68 as usize] = 0.0;
		}
		self.fTempPerm215 = 0.0;
		self.fTempPerm216 = 0.0;
		for l69 in 0..2 {
			self.fVec33[l69 as usize] = 0.0;
		}
		self.fTempPerm217 = 0.0;
		for l70 in 0..2 {
			self.fVec34[l70 as usize] = 0.0;
		}
		self.fTempPerm218 = 0.0;
		self.iTempPerm219 = 0;
		self.fTempPerm220 = 0.0;
		for l71 in 0..2 {
			self.fVec35[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec36[l72 as usize] = 0.0;
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
		for l73 in 0..2 {
			self.fRec37[l73 as usize] = 0.0;
		}
		self.fTempPerm232 = 0.0;
		self.fTempPerm233 = 0.0;
		self.fTempPerm234 = 0.0;
		self.fTempPerm235 = 0.0;
		for l74 in 0..2 {
			self.fRec38[l74 as usize] = 0.0;
		}
		self.fTempPerm236 = 0.0;
		self.fTempPerm237 = 0.0;
		self.fTempPerm238 = 0.0;
		for l75 in 0..2 {
			self.fRec39[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fVec36[l76 as usize] = 0.0;
		}
		self.fTempPerm239 = 0.0;
		self.fTempPerm240 = 0.0;
		for l77 in 0..2 {
			self.fVec37[l77 as usize] = 0.0;
		}
		self.fTempPerm241 = 0.0;
		for l78 in 0..2 {
			self.fRec40[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec41[l79 as usize] = 0.0;
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
		for l80 in 0..2 {
			self.fRec42[l80 as usize] = 0.0;
		}
		self.fTempPerm256 = 0.0;
		self.iTempPerm257 = 0;
		self.iTempPerm258 = 0;
		self.fTempPerm259 = 0.0;
		self.fTempPerm260 = 0.0;
		self.fTempPerm261 = 0.0;
		self.fTempPerm262 = 0.0;
		self.fTempPerm263 = 0.0;
		self.iTempPerm264 = 0;
		self.fTempPerm265 = 0.0;
		self.fTempPerm266 = 0.0;
		for l81 in 0..2 {
			self.fVec38[l81 as usize] = 0.0;
		}
		self.fTempPerm267 = 0.0;
		for l82 in 0..2 {
			self.fRec43[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec44[l83 as usize] = 0.0;
		}
		self.fTempPerm268 = 0.0;
		self.fTempPerm269 = 0.0;
		self.fTempPerm270 = 0.0;
		self.fTempPerm271 = 0.0;
		self.iTempPerm272 = 0;
		self.iTempPerm273 = 0;
		self.iTempPerm274 = 0;
		self.iTempPerm275 = 0;
		self.iTempPerm276 = 0;
		self.iTempPerm277 = 0;
		self.iTempPerm278 = 0;
		self.fTempPerm279 = 0.0;
		self.fTempPerm280 = 0.0;
		self.iTempPerm281 = 0;
		self.iTempPerm282 = 0;
		self.iTempPerm283 = 0;
		self.iTempPerm284 = 0;
		self.fTempPerm285 = 0.0;
		self.fTempPerm286 = 0.0;
		self.fTempPerm287 = 0.0;
		self.iTempPerm288 = 0;
		self.fTempPerm289 = 0.0;
		self.fTempPerm290 = 0.0;
		self.fTempPerm291 = 0.0;
		self.fTempPerm292 = 0.0;
		self.iTempPerm293 = 0;
		self.iTempPerm294 = 0;
		self.iTempPerm295 = 0;
		self.iTempPerm296 = 0;
		self.iTempPerm297 = 0;
		self.iTempPerm298 = 0;
		self.iTempPerm299 = 0;
		self.fTempPerm300 = 0.0;
		self.iTempPerm301 = 0;
		self.iTempPerm302 = 0;
		self.fTempPerm303 = 0.0;
		self.fTempPerm304 = 0.0;
		self.fTempPerm305 = 0.0;
		self.fTempPerm306 = 0.0;
		self.fTempPerm307 = 0.0;
		self.fTempPerm308 = 0.0;
		self.fTempPerm309 = 0.0;
		self.fTempPerm310 = 0.0;
		self.iTempPerm311 = 0;
		self.iTempPerm312 = 0;
		self.fTempPerm313 = 0.0;
		self.fTempPerm314 = 0.0;
		self.fTempPerm315 = 0.0;
		self.fTempPerm316 = 0.0;
		self.fTempPerm317 = 0.0;
		self.fTempPerm318 = 0.0;
		self.fTempPerm319 = 0.0;
		for l84 in 0..3 {
			self.fRec45[l84 as usize] = 0.0;
		}
		self.fTempPerm320 = 0.0;
		self.fTempPerm321 = 0.0;
		self.fTempPerm322 = 0.0;
		self.fTempPerm323 = 0.0;
		self.fTempPerm324 = 0.0;
		for l85 in 0..2 {
			self.fRec47[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec46[l86 as usize] = 0.0;
		}
		self.fTempPerm325 = 0.0;
		self.fTempPerm326 = 0.0;
		self.fTempPerm327 = 0.0;
		self.fTempPerm328 = 0.0;
		self.fTempPerm329 = 0.0;
		self.fTempPerm330 = 0.0;
		self.fTempPerm331 = 0.0;
		self.fTempPerm332 = 0.0;
		self.fTempPerm333 = 0.0;
		self.fTempPerm334 = 0.0;
		for l87 in 0..2 {
			self.fRec48[l87 as usize] = 0.0;
		}
		self.iTempPerm335 = 0;
		for l88 in 0..2 {
			self.fVec39[l88 as usize] = 0.0;
		}
		self.fTempPerm336 = 0.0;
		for l89 in 0..2 {
			self.fRec49[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec50[l90 as usize] = 0.0;
		}
		self.fTempPerm337 = 0.0;
		for l91 in 0..2 {
			self.fVec40[l91 as usize] = 0.0;
		}
		self.fTempPerm338 = 0.0;
		for l92 in 0..2 {
			self.fRec51[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec52[l93 as usize] = 0.0;
		}
		self.fTempPerm339 = 0.0;
		self.fTempPerm340 = 0.0;
		self.fTempPerm341 = 0.0;
		self.fTempPerm342 = 0.0;
		self.fTempPerm343 = 0.0;
		self.fTempPerm344 = 0.0;
		self.fTempPerm345 = 0.0;
		self.fTempPerm346 = 0.0;
		self.fTempPerm347 = 0.0;
		self.fTempPerm348 = 0.0;
		self.iTempPerm349 = 0;
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
		self.fTempPerm363 = 0.0;
		self.fTempPerm364 = 0.0;
		for l94 in 0..2 {
			self.fVec41[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fVec42[l95 as usize] = 0.0;
		}
		self.fTempPerm365 = 0.0;
		self.fTempPerm366 = 0.0;
		self.fTempPerm367 = 0.0;
		for l96 in 0..2 {
			self.fRec53[l96 as usize] = 0.0;
		}
		self.fTempPerm368 = 0.0;
		self.fTempPerm369 = 0.0;
		self.fTempPerm370 = 0.0;
		self.fTempPerm371 = 0.0;
		self.fTempPerm372 = 0.0;
		self.fTempPerm373 = 0.0;
		for l97 in 0..2 {
			self.fVec43[l97 as usize] = 0.0;
		}
		self.fTempPerm374 = 0.0;
		self.iTempPerm375 = 0;
		for l98 in 0..2 {
			self.iRec54[l98 as usize] = 0;
		}
		for l99 in 0..2 {
			self.fVec44[l99 as usize] = 0.0;
		}
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
		self.fTempPerm387 = 0.0;
		self.fTempPerm388 = 0.0;
		for l100 in 0..2 {
			self.fVec45[l100 as usize] = 0.0;
		}
		self.fTempPerm389 = 0.0;
		self.fTempPerm390 = 0.0;
		for l101 in 0..2 {
			self.fVec46[l101 as usize] = 0.0;
		}
		self.fTempPerm391 = 0.0;
		self.fTempPerm392 = 0.0;
		self.fTempPerm393 = 0.0;
		self.fTempPerm394 = 0.0;
		self.fTempPerm395 = 0.0;
		self.fTempPerm396 = 0.0;
		self.iTempPerm397 = 0;
		self.fTempPerm398 = 0.0;
		self.fTempPerm399 = 0.0;
		self.fTempPerm400 = 0.0;
		for l102 in 0..2 {
			self.fVec47[l102 as usize] = 0.0;
		}
		self.fTempPerm401 = 0.0;
		for l103 in 0..2 {
			self.fVec48[l103 as usize] = 0.0;
		}
		self.fTempPerm402 = 0.0;
		for l104 in 0..2 {
			self.fVec49[l104 as usize] = 0.0;
		}
		self.fTempPerm403 = 0.0;
		self.iTempPerm404 = 0;
		self.fTempPerm405 = 0.0;
		self.fTempPerm406 = 0.0;
		self.fTempPerm407 = 0.0;
		self.fTempPerm408 = 0.0;
		self.fTempPerm409 = 0.0;
		self.fTempPerm410 = 0.0;
		self.fTempPerm411 = 0.0;
		self.iTempPerm412 = 0;
		self.iTempPerm413 = 0;
		self.iTempPerm414 = 0;
		self.iTempPerm415 = 0;
		self.iTempPerm416 = 0;
		self.iTempPerm417 = 0;
		self.fTempPerm418 = 0.0;
		self.fTempPerm419 = 0.0;
		self.fTempPerm420 = 0.0;
		self.iTempPerm421 = 0;
		self.iTempPerm422 = 0;
		self.iTempPerm423 = 0;
		self.iTempPerm424 = 0;
		self.fTempPerm425 = 0.0;
		self.fTempPerm426 = 0.0;
		self.fTempPerm427 = 0.0;
		for l105 in 0..2 {
			self.fVec50[l105 as usize] = 0.0;
		}
		self.fTempPerm428 = 0.0;
		self.fTempPerm429 = 0.0;
		self.fTempPerm430 = 0.0;
		self.fTempPerm431 = 0.0;
		self.fTempPerm432 = 0.0;
		for l106 in 0..2 {
			self.fVec51[l106 as usize] = 0.0;
		}
		self.fTempPerm433 = 0.0;
		self.fTempPerm434 = 0.0;
		self.fTempPerm435 = 0.0;
		for l107 in 0..2 {
			self.fRec55[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fVec52[l108 as usize] = 0.0;
		}
		self.fTempPerm436 = 0.0;
		self.fTempPerm437 = 0.0;
		for l109 in 0..2 {
			self.fRec56[l109 as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fVec53[l110 as usize] = 0.0;
		}
		self.fTempPerm438 = 0.0;
		for l111 in 0..2 {
			self.fVec54[l111 as usize] = 0.0;
		}
		self.fTempPerm439 = 0.0;
		self.iTempPerm440 = 0;
		self.fTempPerm441 = 0.0;
		self.fTempPerm442 = 0.0;
		self.fTempPerm443 = 0.0;
		self.fTempPerm444 = 0.0;
		self.fTempPerm445 = 0.0;
		for l112 in 0..2 {
			self.fRec57[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fVec55[l113 as usize] = 0.0;
		}
		self.fTempPerm446 = 0.0;
		self.fTempPerm447 = 0.0;
		self.fTempPerm448 = 0.0;
		self.fTempPerm449 = 0.0;
		self.fTempPerm450 = 0.0;
		for l114 in 0..2 {
			self.fRec58[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fVec56[l115 as usize] = 0.0;
		}
		self.fTempPerm451 = 0.0;
		self.fTempPerm452 = 0.0;
		self.fTempPerm453 = 0.0;
		self.fTempPerm454 = 0.0;
		self.iTempPerm455 = 0;
		self.fTempPerm456 = 0.0;
		self.fTempPerm457 = 0.0;
		self.fTempPerm458 = 0.0;
		for l116 in 0..2 {
			self.fRec59[l116 as usize] = 0.0;
		}
		self.fTempPerm459 = 0.0;
		self.iTempPerm460 = 0;
		self.fTempPerm461 = 0.0;
		self.iTempPerm462 = 0;
		self.fTempPerm463 = 0.0;
		self.fTempPerm464 = 0.0;
		self.fTempPerm465 = 0.0;
		for l117 in 0..2 {
			self.fVec57[l117 as usize] = 0.0;
		}
		self.fTempPerm466 = 0.0;
		self.fTempPerm467 = 0.0;
		self.iTempPerm468 = 0;
		self.fTempPerm469 = 0.0;
		self.fTempPerm470 = 0.0;
		self.fTempPerm471 = 0.0;
		for l118 in 0..2 {
			self.fRec60[l118 as usize] = 0.0;
		}
		self.fTempPerm472 = 0.0;
		self.fTempPerm473 = 0.0;
		self.fTempPerm474 = 0.0;
		self.fTempPerm475 = 0.0;
		self.fTempPerm476 = 0.0;
		self.fTempPerm477 = 0.0;
		self.fTempPerm478 = 0.0;
		self.fTempPerm479 = 0.0;
		self.fTempPerm480 = 0.0;
		for l119 in 0..2 {
			self.fVec58[l119 as usize] = 0.0;
		}
		self.fTempPerm481 = 0.0;
		self.fTempPerm482 = 0.0;
		for l120 in 0..2 {
			self.fVec59[l120 as usize] = 0.0;
		}
		self.fTempPerm483 = 0.0;
		self.fTempPerm484 = 0.0;
		self.fTempPerm485 = 0.0;
		for l121 in 0..2 {
			self.fRec61[l121 as usize] = 0.0;
		}
		self.fTempPerm486 = 0.0;
		self.fTempPerm487 = 0.0;
		self.fTempPerm488 = 0.0;
		self.fTempPerm489 = 0.0;
		self.iTempPerm490 = 0;
		for l122 in 0..2 {
			self.fRec62[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec63[l123 as usize] = 0.0;
		}
		self.fTempPerm491 = 0.0;
		self.iTempPerm492 = 0;
		self.fTempPerm493 = 0.0;
		self.iTempPerm494 = 0;
		self.fTempPerm495 = 0.0;
		self.iTempPerm496 = 0;
		self.fTempPerm497 = 0.0;
		self.fTempPerm498 = 0.0;
		self.fTempPerm499 = 0.0;
		for l124 in 0..2 {
			self.fVec60[l124 as usize] = 0.0;
		}
		self.fTempPerm500 = 0.0;
		self.iTempPerm501 = 0;
		self.fTempPerm502 = 0.0;
		self.fTempPerm503 = 0.0;
		self.fTempPerm504 = 0.0;
		for l125 in 0..2 {
			self.fRec64[l125 as usize] = 0.0;
		}
		self.fTempPerm505 = 0.0;
		self.fTempPerm506 = 0.0;
		self.fTempPerm507 = 0.0;
		self.fTempPerm508 = 0.0;
		self.fTempPerm509 = 0.0;
		self.fTempPerm510 = 0.0;
		self.fTempPerm511 = 0.0;
		self.fTempPerm512 = 0.0;
		self.fTempPerm513 = 0.0;
		self.fTempPerm514 = 0.0;
		self.fTempPerm515 = 0.0;
		for l126 in 0..2 {
			self.fVec61[l126 as usize] = 0.0;
		}
		self.fTempPerm516 = 0.0;
		self.fTempPerm517 = 0.0;
		for l127 in 0..2 {
			self.fVec62[l127 as usize] = 0.0;
		}
		self.fTempPerm518 = 0.0;
		self.iTempPerm519 = 0;
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
		self.fTempPerm530 = 0.0;
		self.fTempPerm531 = 0.0;
		self.fTempPerm532 = 0.0;
		for l128 in 0..2 {
			self.fVec63[l128 as usize] = 0.0;
		}
		self.fTempPerm533 = 0.0;
		self.fTempPerm534 = 0.0;
		for l129 in 0..2 {
			self.fVec64[l129 as usize] = 0.0;
		}
		self.fTempPerm535 = 0.0;
		for l130 in 0..2 {
			self.fVec65[l130 as usize] = 0.0;
		}
		self.fTempPerm536 = 0.0;
		self.iTempPerm537 = 0;
		self.fTempPerm538 = 0.0;
		for l131 in 0..2 {
			self.fVec66[l131 as usize] = 0.0;
		}
		for l132 in 0..2 {
			self.fRec65[l132 as usize] = 0.0;
		}
		self.fTempPerm539 = 0.0;
		self.fTempPerm540 = 0.0;
		self.fTempPerm541 = 0.0;
		self.iTempPerm542 = 0;
		self.fTempPerm543 = 0.0;
		self.iTempPerm544 = 0;
		self.fTempPerm545 = 0.0;
		self.fTempPerm546 = 0.0;
		self.fTempPerm547 = 0.0;
		for l133 in 0..2 {
			self.fRec66[l133 as usize] = 0.0;
		}
		self.fTempPerm548 = 0.0;
		self.fTempPerm549 = 0.0;
		for l134 in 0..2 {
			self.fRec67[l134 as usize] = 0.0;
		}
		self.fTempPerm550 = 0.0;
		for l135 in 0..2 {
			self.fRec68[l135 as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fVec67[l136 as usize] = 0.0;
		}
		self.fTempPerm551 = 0.0;
		self.fTempPerm552 = 0.0;
		for l137 in 0..2 {
			self.fVec68[l137 as usize] = 0.0;
		}
		self.fTempPerm553 = 0.0;
		for l138 in 0..2 {
			self.fRec69[l138 as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec70[l139 as usize] = 0.0;
		}
		self.fTempPerm554 = 0.0;
		self.iTempPerm555 = 0;
		self.iTempPerm556 = 0;
		self.iTempPerm557 = 0;
		self.iTempPerm558 = 0;
		self.fTempPerm559 = 0.0;
		self.iTempPerm560 = 0;
		self.iTempPerm561 = 0;
		self.iTempPerm562 = 0;
		self.iTempPerm563 = 0;
		self.iTempPerm564 = 0;
		self.iTempPerm565 = 0;
		self.iTempPerm566 = 0;
		self.fTempPerm567 = 0.0;
		for l140 in 0..2 {
			self.fRec71[l140 as usize] = 0.0;
		}
		self.fTempPerm568 = 0.0;
		self.iTempPerm569 = 0;
		self.iTempPerm570 = 0;
		self.fTempPerm571 = 0.0;
		self.fTempPerm572 = 0.0;
		self.fTempPerm573 = 0.0;
		self.fTempPerm574 = 0.0;
		self.fTempPerm575 = 0.0;
		self.iTempPerm576 = 0;
		self.fTempPerm577 = 0.0;
		self.fTempPerm578 = 0.0;
		for l141 in 0..2 {
			self.fVec69[l141 as usize] = 0.0;
		}
		self.fTempPerm579 = 0.0;
		for l142 in 0..2 {
			self.fRec72[l142 as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec73[l143 as usize] = 0.0;
		}
		self.fTempPerm580 = 0.0;
		self.fTempPerm581 = 0.0;
		self.fTempPerm582 = 0.0;
		self.fTempPerm583 = 0.0;
		self.iTempPerm584 = 0;
		self.iTempPerm585 = 0;
		self.iTempPerm586 = 0;
		self.iTempPerm587 = 0;
		self.iTempPerm588 = 0;
		self.iTempPerm589 = 0;
		self.iTempPerm590 = 0;
		self.fTempPerm591 = 0.0;
		self.fTempPerm592 = 0.0;
		self.iTempPerm593 = 0;
		self.iTempPerm594 = 0;
		self.iTempPerm595 = 0;
		self.iTempPerm596 = 0;
		self.fTempPerm597 = 0.0;
		self.fTempPerm598 = 0.0;
		self.fTempPerm599 = 0.0;
		self.iTempPerm600 = 0;
		self.fTempPerm601 = 0.0;
		self.fTempPerm602 = 0.0;
		self.fTempPerm603 = 0.0;
		self.fTempPerm604 = 0.0;
		self.iTempPerm605 = 0;
		self.iTempPerm606 = 0;
		self.iTempPerm607 = 0;
		self.iTempPerm608 = 0;
		self.iTempPerm609 = 0;
		self.iTempPerm610 = 0;
		self.iTempPerm611 = 0;
		self.fTempPerm612 = 0.0;
		self.iTempPerm613 = 0;
		self.iTempPerm614 = 0;
		self.fTempPerm615 = 0.0;
		self.fTempPerm616 = 0.0;
		self.fTempPerm617 = 0.0;
		self.fTempPerm618 = 0.0;
		self.fTempPerm619 = 0.0;
		self.fTempPerm620 = 0.0;
		self.fTempPerm621 = 0.0;
		self.fTempPerm622 = 0.0;
		self.iTempPerm623 = 0;
		self.iTempPerm624 = 0;
		self.fTempPerm625 = 0.0;
		self.fTempPerm626 = 0.0;
		self.fTempPerm627 = 0.0;
		self.fTempPerm628 = 0.0;
		self.fTempPerm629 = 0.0;
		self.fTempPerm630 = 0.0;
		self.fTempPerm631 = 0.0;
		for l144 in 0..3 {
			self.fRec74[l144 as usize] = 0.0;
		}
		self.fTempPerm632 = 0.0;
		self.fTempPerm633 = 0.0;
		self.fTempPerm634 = 0.0;
		self.fTempPerm635 = 0.0;
		self.fTempPerm636 = 0.0;
		for l145 in 0..2 {
			self.fRec76[l145 as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec75[l146 as usize] = 0.0;
		}
		self.fTempPerm637 = 0.0;
		self.fTempPerm638 = 0.0;
		self.fTempPerm639 = 0.0;
		self.fTempPerm640 = 0.0;
		self.fTempPerm641 = 0.0;
		self.fTempPerm642 = 0.0;
		self.fTempPerm643 = 0.0;
		self.fTempPerm644 = 0.0;
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
		ui_interface.declare(Some(ParamIndex(2)), "skew", "0.4");
		ui_interface.declare(Some(ParamIndex(2)), "unit", "cents");
		ui_interface.add_horizontal_slider("Detune", ParamIndex(2), 6.0, 0.0, 5e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(3)), "3", "");
		ui_interface.add_check_button("Random Phase", ParamIndex(3));
		ui_interface.close_box();
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Perform");
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.declare(Some(ParamIndex(4)), "unit", "st");
		ui_interface.add_horizontal_slider("Global Pitch", ParamIndex(4), 0.0, -12.0, 12.0, 0.01);
		ui_interface.declare(Some(ParamIndex(5)), "5", "");
		ui_interface.declare(Some(ParamIndex(5)), "style", "menu{'2 Semitones':0;'Octave':1;'Cutoff 50%':2;'Resonance 50%':3;'Bandpass 50%':4}");
		ui_interface.add_num_entry("Bend Mode", ParamIndex(5), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "13", "");
		ui_interface.open_horizontal_box("Settings");
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.add_horizontal_slider("Vintage Noise", ParamIndex(6), 1.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "2.1", "");
		ui_interface.open_horizontal_box("Dual 1");
		ui_interface.declare(Some(ParamIndex(7)), "1", "");
		ui_interface.declare(Some(ParamIndex(7)), "style", "menu{'VA Sweep':0;'Spread':1;'PWM':2;'Sync':3;'Fractal':4;'Window Amp Sync':5;'Metal':6;'Ring Mod':7;'FM':8;'Noise AM':9;'Chaos':10;'Folding':11}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(7), 0.0, 0.0, 11.0, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "2", "");
		ui_interface.declare(Some(ParamIndex(8)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(8), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(9)), "3", "");
		ui_interface.declare(Some(ParamIndex(9)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(9), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(10)), "4", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(10), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(11)), "5", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(11), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "6", "");
		ui_interface.add_num_entry("Semi", ParamIndex(12), 0.0, -12.0, 12.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "2.2", "");
		ui_interface.open_horizontal_box("WT 1");
		ui_interface.declare(Some(ParamIndex(13)), "1", "");
		ui_interface.declare(Some(ParamIndex(13)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(13), 0.0, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(14)), "2", "");
		ui_interface.declare(Some(ParamIndex(14)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(14), 4e+01, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(15)), "3", "");
		ui_interface.declare(Some(ParamIndex(15)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 3", ParamIndex(15), 8e+01, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(16)), "4", "");
		ui_interface.declare(Some(ParamIndex(16)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 4", ParamIndex(16), 118.0, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(17)), "5", "");
		ui_interface.add_horizontal_slider("Position", ParamIndex(17), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(18)), "6", "");
		ui_interface.declare(Some(ParamIndex(18)), "style", "menu{'Off':0;'Bits 4':1;'SR 11k':2;'SR 8k':3;'SR 4k':4;'Simplify 6k':5;'Simplify 1.5k':6;'Simplify 800':7;'Rectify':8;'Tri Fold':9;'Sine Fold':10;'Squish':11;'Ring':12;'Mirror':13;'Sign Swap':14}");
		ui_interface.add_num_entry("OscMod", ParamIndex(18), 0.0, 0.0, 14.0, 1.0);
		ui_interface.declare(Some(ParamIndex(19)), "7", "");
		ui_interface.add_horizontal_slider("OscMod Depth", ParamIndex(19), 0.5, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2.3", "");
		ui_interface.open_horizontal_box("Ray 1");
		ui_interface.declare(Some(ParamIndex(20)), "10", "");
		ui_interface.declare(Some(ParamIndex(20)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(20)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Orbit Rate", ParamIndex(20), 0.3, 0.02, 1e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(21)), "11", "");
		ui_interface.declare(Some(ParamIndex(21)), "unit", "cents");
		ui_interface.add_horizontal_slider("Fine", ParamIndex(21), 0.0, -1e+02, 1e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(22)), "1", "");
		ui_interface.declare(Some(ParamIndex(22)), "style", "menu{'Earth':0;'Water':1;'Air':2;'Fire':3;'Wood':4;'Metal':5;'Chaos':6}");
		ui_interface.add_num_entry("Element", ParamIndex(22), 0.0, 0.0, 6.0, 1.0);
		ui_interface.declare(Some(ParamIndex(23)), "2", "");
		ui_interface.add_num_entry("Chaos Seed", ParamIndex(23), 4821.0, 1.0, 99999.0, 1.0);
		ui_interface.declare(Some(ParamIndex(24)), "3", "");
		ui_interface.add_check_button("Free", ParamIndex(24));
		ui_interface.declare(Some(ParamIndex(25)), "4", "");
		ui_interface.add_horizontal_slider("Squish", ParamIndex(25), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(26)), "5", "");
		ui_interface.add_horizontal_slider("Skew", ParamIndex(26), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(27)), "6", "");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(27), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(28)), "7", "");
		ui_interface.add_horizontal_slider("Center", ParamIndex(28), 0.0, -1.3, 1.3, 0.001);
		ui_interface.declare(Some(ParamIndex(29)), "8", "");
		ui_interface.add_horizontal_slider("Ray", ParamIndex(29), 0.0, 0.0, 1.5, 0.001);
		ui_interface.declare(Some(ParamIndex(30)), "9", "");
		ui_interface.add_horizontal_slider("Orbit", ParamIndex(30), 0.0, 0.0, 0.8, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("Osc 1");
		ui_interface.declare(Some(ParamIndex(31)), "0", "");
		ui_interface.declare(Some(ParamIndex(31)), "style", "menu{'Dual':0;'Wavetable':1;'Ray':2}");
		ui_interface.add_num_entry("Mode", ParamIndex(31), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(32)), "1", "");
		ui_interface.add_num_entry("Octave", ParamIndex(32), 0.0, -2.0, 2.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "3.1", "");
		ui_interface.open_horizontal_box("Dual 2");
		ui_interface.declare(Some(ParamIndex(33)), "1", "");
		ui_interface.declare(Some(ParamIndex(33)), "style", "menu{'VA Sweep':0;'Spread':1;'PWM':2;'Sync':3;'Fractal':4;'Window Amp Sync':5;'Metal':6;'Ring Mod':7;'FM':8;'Noise AM':9;'Chaos':10;'Folding':11}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(33), 0.0, 0.0, 11.0, 1.0);
		ui_interface.declare(Some(ParamIndex(34)), "2", "");
		ui_interface.declare(Some(ParamIndex(34)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(34), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(35)), "3", "");
		ui_interface.declare(Some(ParamIndex(35)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(35), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(36)), "4", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(36), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(37)), "5", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(37), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(38)), "6", "");
		ui_interface.add_num_entry("Semi", ParamIndex(38), 0.0, -12.0, 12.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "3.2", "");
		ui_interface.open_horizontal_box("WT 2");
		ui_interface.declare(Some(ParamIndex(39)), "1", "");
		ui_interface.declare(Some(ParamIndex(39)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(39), 0.0, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(40)), "2", "");
		ui_interface.declare(Some(ParamIndex(40)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(40), 4e+01, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(41)), "3", "");
		ui_interface.declare(Some(ParamIndex(41)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 3", ParamIndex(41), 8e+01, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(42)), "4", "");
		ui_interface.declare(Some(ParamIndex(42)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 4", ParamIndex(42), 118.0, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(43)), "5", "");
		ui_interface.add_horizontal_slider("Position", ParamIndex(43), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(44)), "6", "");
		ui_interface.declare(Some(ParamIndex(44)), "style", "menu{'Off':0;'Bits 4':1;'SR 11k':2;'SR 8k':3;'SR 4k':4;'Simplify 6k':5;'Simplify 1.5k':6;'Simplify 800':7;'Rectify':8;'Tri Fold':9;'Sine Fold':10;'Squish':11;'Ring':12;'Mirror':13;'Sign Swap':14}");
		ui_interface.add_num_entry("OscMod", ParamIndex(44), 0.0, 0.0, 14.0, 1.0);
		ui_interface.declare(Some(ParamIndex(45)), "7", "");
		ui_interface.add_horizontal_slider("OscMod Depth", ParamIndex(45), 0.5, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.3", "");
		ui_interface.open_horizontal_box("Ray 2");
		ui_interface.declare(Some(ParamIndex(46)), "10", "");
		ui_interface.declare(Some(ParamIndex(46)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(46)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Orbit Rate", ParamIndex(46), 0.3, 0.02, 1e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(47)), "11", "");
		ui_interface.declare(Some(ParamIndex(47)), "unit", "cents");
		ui_interface.add_horizontal_slider("Fine", ParamIndex(47), 0.0, -1e+02, 1e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(48)), "1", "");
		ui_interface.declare(Some(ParamIndex(48)), "style", "menu{'Earth':0;'Water':1;'Air':2;'Fire':3;'Wood':4;'Metal':5;'Chaos':6}");
		ui_interface.add_num_entry("Element", ParamIndex(48), 3.0, 0.0, 6.0, 1.0);
		ui_interface.declare(Some(ParamIndex(49)), "2", "");
		ui_interface.add_num_entry("Chaos Seed", ParamIndex(49), 6821.0, 1.0, 99999.0, 1.0);
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
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("Osc 2");
		ui_interface.declare(Some(ParamIndex(57)), "0", "");
		ui_interface.declare(Some(ParamIndex(57)), "style", "menu{'Dual':0;'Wavetable':1;'Ray':2}");
		ui_interface.add_num_entry("Mode", ParamIndex(57), 1.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(58)), "1", "");
		ui_interface.add_num_entry("Octave", ParamIndex(58), 0.0, -2.0, 2.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(59)), "1", "");
		ui_interface.add_vertical_slider("Osc 1", ParamIndex(59), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(60)), "2", "");
		ui_interface.add_vertical_slider("Osc 2", ParamIndex(60), 0.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(61)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(61)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(61), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("copy", ParamIndex(62), 3.0, 0.0, 6.0, 1.0);
		ui_interface.add_horizontal_slider("freq", ParamIndex(63), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_button("gate", ParamIndex(64));
		ui_interface.add_horizontal_slider("glide", ParamIndex(65), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("ramp", ParamIndex(66), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("seed", ParamIndex(67), 0.0, 0.0, 1.0, 1e-06);
		ui_interface.add_horizontal_slider("slot", ParamIndex(68), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			64 => Some(self.fButton0),
			3 => Some(self.fCheckbox0),
			50 => Some(self.fCheckbox1),
			24 => Some(self.fCheckbox2),
			31 => Some(self.fEntry0),
			7 => Some(self.fEntry1),
			9 => Some(self.fEntry10),
			8 => Some(self.fEntry11),
			14 => Some(self.fEntry12),
			15 => Some(self.fEntry13),
			16 => Some(self.fEntry14),
			13 => Some(self.fEntry15),
			38 => Some(self.fEntry16),
			58 => Some(self.fEntry17),
			35 => Some(self.fEntry18),
			34 => Some(self.fEntry19),
			1 => Some(self.fEntry2),
			40 => Some(self.fEntry20),
			41 => Some(self.fEntry21),
			42 => Some(self.fEntry22),
			39 => Some(self.fEntry23),
			49 => Some(self.fEntry24),
			48 => Some(self.fEntry25),
			23 => Some(self.fEntry26),
			22 => Some(self.fEntry27),
			44 => Some(self.fEntry3),
			57 => Some(self.fEntry4),
			18 => Some(self.fEntry5),
			5 => Some(self.fEntry6),
			12 => Some(self.fEntry7),
			32 => Some(self.fEntry8),
			33 => Some(self.fEntry9),
			62 => Some(self.fHslider0),
			2 => Some(self.fHslider1),
			67 => Some(self.fHslider10),
			11 => Some(self.fHslider11),
			10 => Some(self.fHslider12),
			17 => Some(self.fHslider13),
			19 => Some(self.fHslider14),
			37 => Some(self.fHslider15),
			36 => Some(self.fHslider16),
			43 => Some(self.fHslider17),
			45 => Some(self.fHslider18),
			46 => Some(self.fHslider19),
			66 => Some(self.fHslider2),
			47 => Some(self.fHslider20),
			56 => Some(self.fHslider21),
			55 => Some(self.fHslider22),
			54 => Some(self.fHslider23),
			53 => Some(self.fHslider24),
			52 => Some(self.fHslider25),
			51 => Some(self.fHslider26),
			21 => Some(self.fHslider27),
			20 => Some(self.fHslider28),
			30 => Some(self.fHslider29),
			0 => Some(self.fHslider3),
			29 => Some(self.fHslider30),
			28 => Some(self.fHslider31),
			27 => Some(self.fHslider32),
			26 => Some(self.fHslider33),
			25 => Some(self.fHslider34),
			68 => Some(self.fHslider4),
			6 => Some(self.fHslider5),
			61 => Some(self.fHslider6),
			4 => Some(self.fHslider7),
			65 => Some(self.fHslider8),
			63 => Some(self.fHslider9),
			59 => Some(self.fVslider0),
			60 => Some(self.fVslider1),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			64 => { self.fButton0 = value }
			3 => { self.fCheckbox0 = value }
			50 => { self.fCheckbox1 = value }
			24 => { self.fCheckbox2 = value }
			31 => { self.fEntry0 = value }
			7 => { self.fEntry1 = value }
			9 => { self.fEntry10 = value }
			8 => { self.fEntry11 = value }
			14 => { self.fEntry12 = value }
			15 => { self.fEntry13 = value }
			16 => { self.fEntry14 = value }
			13 => { self.fEntry15 = value }
			38 => { self.fEntry16 = value }
			58 => { self.fEntry17 = value }
			35 => { self.fEntry18 = value }
			34 => { self.fEntry19 = value }
			1 => { self.fEntry2 = value }
			40 => { self.fEntry20 = value }
			41 => { self.fEntry21 = value }
			42 => { self.fEntry22 = value }
			39 => { self.fEntry23 = value }
			49 => { self.fEntry24 = value }
			48 => { self.fEntry25 = value }
			23 => { self.fEntry26 = value }
			22 => { self.fEntry27 = value }
			44 => { self.fEntry3 = value }
			57 => { self.fEntry4 = value }
			18 => { self.fEntry5 = value }
			5 => { self.fEntry6 = value }
			12 => { self.fEntry7 = value }
			32 => { self.fEntry8 = value }
			33 => { self.fEntry9 = value }
			62 => { self.fHslider0 = value }
			2 => { self.fHslider1 = value }
			67 => { self.fHslider10 = value }
			11 => { self.fHslider11 = value }
			10 => { self.fHslider12 = value }
			17 => { self.fHslider13 = value }
			19 => { self.fHslider14 = value }
			37 => { self.fHslider15 = value }
			36 => { self.fHslider16 = value }
			43 => { self.fHslider17 = value }
			45 => { self.fHslider18 = value }
			46 => { self.fHslider19 = value }
			66 => { self.fHslider2 = value }
			47 => { self.fHslider20 = value }
			56 => { self.fHslider21 = value }
			55 => { self.fHslider22 = value }
			54 => { self.fHslider23 = value }
			53 => { self.fHslider24 = value }
			52 => { self.fHslider25 = value }
			51 => { self.fHslider26 = value }
			21 => { self.fHslider27 = value }
			20 => { self.fHslider28 = value }
			30 => { self.fHslider29 = value }
			0 => { self.fHslider3 = value }
			29 => { self.fHslider30 = value }
			28 => { self.fHslider31 = value }
			27 => { self.fHslider32 = value }
			26 => { self.fHslider33 = value }
			25 => { self.fHslider34 = value }
			68 => { self.fHslider4 = value }
			6 => { self.fHslider5 = value }
			61 => { self.fHslider6 = value }
			4 => { self.fHslider7 = value }
			65 => { self.fHslider8 = value }
			63 => { self.fHslider9 = value }
			59 => { self.fVslider0 = value }
			60 => { self.fVslider1 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8) = if let [outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			let outputs3 = outputs3[..count as usize].iter_mut();
			let outputs4 = outputs4[..count as usize].iter_mut();
			let outputs5 = outputs5[..count as usize].iter_mut();
			let outputs6 = outputs6[..count as usize].iter_mut();
			let outputs7 = outputs7[..count as usize].iter_mut();
			let outputs8 = outputs8[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8)
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
		let mut fSlow38: F32 = self.fButton0;
		let mut fSlow39: F32 = self.fHslider0;
		let mut iSlow40: i32 = (fSlow39) as i32;
		let mut fSlow41: F32 = unsafe { ftbl0OscDspSIG0[iSlow40 as usize] };
		let mut fSlow42: F32 = self.fEntry2;
		let mut iSlow43: i32 = (fSlow42 == 1.0) as i32;
		let mut fSlow44: F32 = (iSlow43) as F32;
		let mut fSlow45: F32 = fSlow41 + fSlow44 * (unsafe { ftbl1OscDspSIG1[iSlow40 as usize] } - fSlow41);
		let mut fSlow46: F32 = self.fHslider1;
		let mut fSlow47: F32 = self.fHslider2;
		let mut fSlow48: F32 = self.fHslider3;
		let mut fSlow49: F32 = self.fEntry3;
		let mut iSlow50: i32 = (fSlow49 == 4.0) as i32;
		let mut fSlow51: F32 = (iSlow50) as F32;
		let mut iSlow52: i32 = (fSlow51 != 0.0) as i32;
		let mut fSlow53: F32 = self.fEntry4;
		let mut iSlow54: i32 = (fSlow53 == 1.0) as i32;
		let mut fSlow55: F32 = (iSlow54) as F32;
		let mut iSlow56: i32 = (fSlow55 != 0.0) as i32;
		let mut iSlow57: i32 = (self.fVslider1 != 0.0) as i32;
		let mut fSlow58: F32 = (iSlow57) as F32;
		let mut iSlow59: i32 = (fSlow58 != 0.0) as i32;
		let mut iSlow60: i32 = (fSlow49 == 3.0) as i32;
		let mut fSlow61: F32 = (iSlow60) as F32;
		let mut iSlow62: i32 = (fSlow61 != 0.0) as i32;
		let mut iSlow63: i32 = (fSlow49 == 2.0) as i32;
		let mut fSlow64: F32 = (iSlow63) as F32;
		let mut iSlow65: i32 = (fSlow64 != 0.0) as i32;
		let mut fSlow66: F32 = self.fEntry5;
		let mut iSlow67: i32 = (fSlow66 == 3.0) as i32;
		let mut fSlow68: F32 = (iSlow67) as F32;
		let mut iSlow69: i32 = (fSlow68 != 0.0) as i32;
		let mut iSlow70: i32 = (fSlow3 == 1.0) as i32;
		let mut fSlow71: F32 = (iSlow70) as F32;
		let mut iSlow72: i32 = (fSlow71 != 0.0) as i32;
		let mut iSlow73: i32 = (fSlow66 == 4.0) as i32;
		let mut fSlow74: F32 = (iSlow73) as F32;
		let mut iSlow75: i32 = (fSlow74 != 0.0) as i32;
		let mut iSlow76: i32 = (fSlow66 == 2.0) as i32;
		let mut fSlow77: F32 = (iSlow76) as F32;
		let mut iSlow78: i32 = (fSlow77 != 0.0) as i32;
		let mut iTemp6: i32 = self.iTempPerm0;
		let mut fTemp7: F32 = self.fTempPerm1;
		let mut fSlow79: F32 = self.fHslider4;
		let mut fSlow80: F32 = 12.9898 * fSlow79;
		let mut fSlow81: F32 = 43758.547 * F32::sin(fSlow80 + 234.699);
		let mut fSlow82: F32 = 43758.547 * F32::sin(fSlow80 + 7979.766);
		let mut fSlow83: F32 = 2.0 * (fSlow82 - F32::floor(fSlow82)) + -1.0;
		let mut fSlow84: F32 = self.fConst2 * (0.1 * F32::max(-1.0, F32::min(1.0, 0.70710677 * (fSlow83 + 2.0 * (fSlow81 - F32::floor(fSlow81)) + -1.0))) + 0.45);
		let mut fTemp8: F32 = self.fTempPerm2;
		let mut iSlow85: i32 = (409.0 * fSlow79 + 4e+03) as i32;
		let mut iTemp9: i32 = self.iTempPerm3;
		let mut fSlow86: F32 = self.fHslider5;
		let mut fSlow87: F32 = 43758.547 * F32::sin(fSlow80 + 156.466);
		let mut fSlow88: F32 = 0.15 * F32::max(-1.0, F32::min(1.0, 1.2803688 * (0.6 * fSlow83 + 0.5 * (2.0 * (fSlow87 - F32::floor(fSlow87)) + -1.0)))) + 0.25;
		let mut fSlow89: F32 = 1.0 / F32::tan(self.fConst8 * F32::max(1.0, fSlow88));
		let mut fSlow90: F32 = 1.0 - fSlow89;
		let mut fSlow91: F32 = self.fConst2 * F32::max(fSlow88, 0.0);
		let mut fTemp12: F32 = self.fTempPerm4;
		let mut fTemp15: F32 = self.fTempPerm5;
		let mut fSlow92: F32 = 1.0 / (fSlow89 + 1.0);
		let mut fSlow93: F32 = 43758.547 * F32::sin(fSlow80 + 78.233);
		let mut fSlow94: F32 = 43758.547 * F32::sin(fSlow80 + 7901.533);
		let mut fSlow95: F32 = 8.0 * F32::max(-1.0, F32::min(1.0, 1.0976425 * (0.7 * (2.0 * (fSlow94 - F32::floor(fSlow94)) + -1.0) + 0.3 * fSlow83 + 0.5 * (2.0 * (fSlow93 - F32::floor(fSlow93)) + -1.0))));
		let mut fTemp16: F32 = self.fTempPerm6;
		let mut fSlow96: F32 = self.fHslider6;
		let mut fSlow97: F32 = self.fEntry6;
		let mut fSlow98: F32 = 2.0 * ((fSlow97 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow97 == 1.0) as i32) as u32 as F32;
		let mut fSlow99: F32 = self.fHslider7;
		let mut fSlow100: F32 = 0.33333334 * F32::max(1e-05, self.fHslider8);
		let mut iSlow101: i32 = (F32::abs(fSlow100) < 1.1920929e-07) as i32;
		let mut fSlow102: F32 = if iSlow101 != 0 {0.0} else {F32::exp(-(self.fConst2 / if iSlow101 != 0 {1.0} else {fSlow100}))};
		let mut fSlow103: F32 = F32::log(self.fHslider9, std::f32::consts::E) * (1.0 - fSlow102);
		let mut fSlow104: F32 = F32::exp(0.6931472 * self.fEntry8);
		let mut fSlow105: F32 = fSlow104 * F32::exp(0.057762265 * self.fEntry7);
		let mut fSlow106: F32 = self.fConst2 * fSlow105;
		let mut fTemp21: F32 = self.fTempPerm7;
		let mut fTemp22: F32 = self.fTempPerm8;
		let mut fSlow107: F32 = 0.618 * fSlow39 + 7.31 * self.fHslider10 * (fSlow39 + 1.0);
		let mut fSlow108: F32 = self.fCheckbox0;
		let mut fSlow109: F32 = self.fEntry9;
		let mut iSlow110: i32 = (fSlow109 == 2.0) as i32;
		let mut fSlow111: F32 = (iSlow110) as F32;
		let mut iSlow112: i32 = (fSlow111 != 0.0) as i32;
		let mut iSlow113: i32 = (fSlow53 == 0.0) as i32;
		let mut fSlow114: F32 = (iSlow113) as F32;
		let mut iSlow115: i32 = (fSlow114 != 0.0) as i32;
		let mut iSlow116: i32 = (fSlow109 == 0.0) as i32;
		let mut fSlow117: F32 = (iSlow116) as F32;
		let mut iSlow118: i32 = (fSlow117 != 0.0) as i32;
		let mut iSlow119: i32 = (fSlow109 == 3.0) as i32;
		let mut fSlow120: F32 = (iSlow119) as F32;
		let mut iSlow121: i32 = (fSlow120 != 0.0) as i32;
		let mut iSlow122: i32 = (fSlow109 == 4.0) as i32;
		let mut fSlow123: F32 = (iSlow122) as F32;
		let mut iSlow124: i32 = (fSlow123 != 0.0) as i32;
		let mut iSlow125: i32 = (fSlow109 == 5.0) as i32;
		let mut fSlow126: F32 = (iSlow125) as F32;
		let mut iSlow127: i32 = (fSlow126 != 0.0) as i32;
		let mut iSlow128: i32 = (fSlow109 == 6.0) as i32;
		let mut fSlow129: F32 = (iSlow128) as F32;
		let mut iSlow130: i32 = (fSlow129 != 0.0) as i32;
		let mut iSlow131: i32 = (fSlow109 == 7.0) as i32;
		let mut fSlow132: F32 = (iSlow131) as F32;
		let mut iSlow133: i32 = (fSlow132 != 0.0) as i32;
		let mut iSlow134: i32 = (fSlow109 == 8.0) as i32;
		let mut fSlow135: F32 = (iSlow134) as F32;
		let mut iSlow136: i32 = (fSlow135 != 0.0) as i32;
		let mut iSlow137: i32 = (fSlow109 == 9.0) as i32;
		let mut fSlow138: F32 = (iSlow137) as F32;
		let mut iSlow139: i32 = (fSlow138 != 0.0) as i32;
		let mut iSlow140: i32 = (fSlow109 == 1e+01) as i32;
		let mut fSlow141: F32 = (iSlow140) as F32;
		let mut iSlow142: i32 = (fSlow141 != 0.0) as i32;
		let mut iSlow143: i32 = (fSlow109 == 11.0) as i32;
		let mut fSlow144: F32 = (iSlow143) as F32;
		let mut iSlow145: i32 = (fSlow144 != 0.0) as i32;
		let mut iSlow146: i32 = (fSlow109 == 1.0) as i32;
		let mut fSlow147: F32 = (iSlow146) as F32;
		let mut iSlow148: i32 = (fSlow147 != 0.0) as i32;
		let mut iSlow149: i32 = (fSlow49 == 14.0) as i32;
		let mut fSlow150: F32 = (iSlow149) as F32;
		let mut iSlow151: i32 = (fSlow150 != 0.0) as i32;
		let mut iSlow152: i32 = (fSlow49 == 13.0) as i32;
		let mut fSlow153: F32 = (iSlow152) as F32;
		let mut iSlow154: i32 = (fSlow153 != 0.0) as i32;
		let mut iSlow155: i32 = (fSlow49 == 12.0) as i32;
		let mut fSlow156: F32 = (iSlow155) as F32;
		let mut iSlow157: i32 = (fSlow156 != 0.0) as i32;
		let mut iSlow158: i32 = (fSlow49 == 11.0) as i32;
		let mut fSlow159: F32 = (iSlow158) as F32;
		let mut iSlow160: i32 = (fSlow159 != 0.0) as i32;
		let mut iSlow161: i32 = (fSlow49 == 1e+01) as i32;
		let mut fSlow162: F32 = (iSlow161) as F32;
		let mut iSlow163: i32 = (fSlow162 != 0.0) as i32;
		let mut iSlow164: i32 = (fSlow49 == 9.0) as i32;
		let mut fSlow165: F32 = (iSlow164) as F32;
		let mut iSlow166: i32 = (fSlow165 != 0.0) as i32;
		let mut iSlow167: i32 = (fSlow49 == 8.0) as i32;
		let mut fSlow168: F32 = (iSlow167) as F32;
		let mut iSlow169: i32 = (fSlow168 != 0.0) as i32;
		let mut iSlow170: i32 = (fSlow49 == 7.0) as i32;
		let mut fSlow171: F32 = (iSlow170) as F32;
		let mut iSlow172: i32 = (fSlow171 != 0.0) as i32;
		let mut iSlow173: i32 = (fSlow49 == 6.0) as i32;
		let mut fSlow174: F32 = (iSlow173) as F32;
		let mut iSlow175: i32 = (fSlow174 != 0.0) as i32;
		let mut iSlow176: i32 = (fSlow49 == 5.0) as i32;
		let mut fSlow177: F32 = (iSlow176) as F32;
		let mut iSlow178: i32 = (fSlow177 != 0.0) as i32;
		let mut iSlow179: i32 = (fSlow49 == 1.0) as i32;
		let mut fSlow180: F32 = (iSlow179) as F32;
		let mut iSlow181: i32 = (fSlow180 != 0.0) as i32;
		let mut iSlow182: i32 = (fSlow49 == 0.0) as i32;
		let mut fSlow183: F32 = (iSlow182) as F32;
		let mut iSlow184: i32 = (fSlow183 != 0.0) as i32;
		let mut iSlow185: i32 = (fSlow7 == 8.0) as i32;
		let mut fSlow186: F32 = (iSlow185) as F32;
		let mut iSlow187: i32 = (fSlow186 != 0.0) as i32;
		let mut iSlow188: i32 = (fSlow7 == 1e+01) as i32;
		let mut fSlow189: F32 = (iSlow188) as F32;
		let mut iSlow190: i32 = (fSlow189 != 0.0) as i32;
		let mut iSlow191: i32 = (fSlow66 == 14.0) as i32;
		let mut fSlow192: F32 = (iSlow191) as F32;
		let mut iSlow193: i32 = (fSlow192 != 0.0) as i32;
		let mut iSlow194: i32 = (fSlow66 == 13.0) as i32;
		let mut fSlow195: F32 = (iSlow194) as F32;
		let mut iSlow196: i32 = (fSlow195 != 0.0) as i32;
		let mut iSlow197: i32 = (fSlow66 == 0.0) as i32;
		let mut fSlow198: F32 = (iSlow197) as F32;
		let mut iSlow199: i32 = (fSlow198 != 0.0) as i32;
		let mut iSlow200: i32 = (fSlow66 == 1e+01) as i32;
		let mut fSlow201: F32 = (iSlow200) as F32;
		let mut iSlow202: i32 = (fSlow201 != 0.0) as i32;
		let mut iSlow203: i32 = (fSlow66 == 6.0) as i32;
		let mut fSlow204: F32 = (iSlow203) as F32;
		let mut iSlow205: i32 = (fSlow204 != 0.0) as i32;
		let mut iSlow206: i32 = (fSlow66 == 9.0) as i32;
		let mut fSlow207: F32 = (iSlow206) as F32;
		let mut iSlow208: i32 = (fSlow207 != 0.0) as i32;
		let mut iSlow209: i32 = (fSlow66 == 11.0) as i32;
		let mut fSlow210: F32 = (iSlow209) as F32;
		let mut iSlow211: i32 = (fSlow210 != 0.0) as i32;
		let mut iSlow212: i32 = (fSlow66 == 12.0) as i32;
		let mut fSlow213: F32 = (iSlow212) as F32;
		let mut iSlow214: i32 = (fSlow213 != 0.0) as i32;
		let mut iSlow215: i32 = (fSlow66 == 8.0) as i32;
		let mut fSlow216: F32 = (iSlow215) as F32;
		let mut iSlow217: i32 = (fSlow216 != 0.0) as i32;
		let mut iSlow218: i32 = (fSlow66 == 5.0) as i32;
		let mut fSlow219: F32 = (iSlow218) as F32;
		let mut iSlow220: i32 = (fSlow219 != 0.0) as i32;
		let mut iSlow221: i32 = (fSlow66 == 7.0) as i32;
		let mut fSlow222: F32 = (iSlow221) as F32;
		let mut iSlow223: i32 = (fSlow222 != 0.0) as i32;
		let mut iSlow224: i32 = (fSlow66 == 1.0) as i32;
		let mut fSlow225: F32 = (iSlow224) as F32;
		let mut iSlow226: i32 = (fSlow225 != 0.0) as i32;
		let mut fTemp26: F32 = self.fTempPerm9;
		let mut iTemp27: i32 = self.iTempPerm10;
		let mut fSlow227: F32 = self.fHslider11;
		let mut fTemp28: F32 = self.fTempPerm11;
		let mut fTemp29: F32 = self.fTempPerm12;
		let mut fSlow228: F32 = self.fHslider12;
		let mut fTemp30: F32 = self.fTempPerm13;
		let mut fTemp31: F32 = self.fTempPerm14;
		let mut fTemp32: F32 = self.fTempPerm15;
		let mut fTemp33: F32 = self.fTempPerm16;
		let mut fTemp34: F32 = self.fTempPerm17;
		let mut fTemp35: F32 = self.fTempPerm18;
		let mut fTemp36: F32 = self.fTempPerm19;
		let mut fSlow229: F32 = self.fConst9 / fSlow105;
		let mut fTemp37: F32 = self.fTempPerm20;
		let mut fSlow230: F32 = self.fEntry10;
		let mut fSlow231: F32 = self.fEntry11;
		let mut fTemp38: F32 = self.fTempPerm21;
		let mut fTemp39: F32 = self.fTempPerm22;
		let mut fTemp40: F32 = self.fTempPerm23;
		let mut fSlow232: F32 = self.fConst10 * fSlow105;
		let mut iTemp41: i32 = self.iTempPerm24;
		let mut fSlow233: F32 = self.fConst11 * fSlow105;
		let mut fTemp42: F32 = self.fTempPerm25;
		let mut fTemp43: F32 = self.fTempPerm26;
		let mut fTemp44: F32 = self.fTempPerm27;
		let mut fTemp45: F32 = self.fTempPerm28;
		let mut fTemp46: F32 = self.fTempPerm29;
		let mut fTemp47: F32 = self.fTempPerm30;
		let mut fTemp48: F32 = self.fTempPerm31;
		let mut fTemp49: F32 = self.fTempPerm32;
		let mut fTemp50: F32 = self.fTempPerm33;
		let mut fTemp51: F32 = self.fTempPerm34;
		let mut fTemp52: F32 = self.fTempPerm35;
		let mut fTemp53: F32 = self.fTempPerm36;
		let mut fTemp54: F32 = self.fTempPerm37;
		let mut fTemp55: F32 = self.fTempPerm38;
		let mut fTemp56: F32 = self.fTempPerm39;
		let mut fTemp57: F32 = self.fTempPerm40;
		let mut fSlow234: F32 = ((iSlow8 != 0) as i32) as u32 as F32;
		let mut iSlow235: i32 = (fSlow230) as i32;
		let mut fSlow236: F32 = self.fConst12 * fSlow105;
		let mut fTemp58: F32 = self.fTempPerm41;
		let mut fTemp59: F32 = self.fTempPerm42;
		let mut fSlow237: F32 = 0.5 * fSlow108;
		let mut fTemp60: F32 = self.fTempPerm43;
		let mut fTemp61: F32 = self.fTempPerm44;
		let mut fTemp62: F32 = self.fTempPerm45;
		let mut fTemp63: F32 = self.fTempPerm46;
		let mut fTemp64: F32 = self.fTempPerm47;
		let mut fTemp65: F32 = self.fTempPerm48;
		let mut fTemp66: F32 = self.fTempPerm49;
		let mut fTemp67: F32 = self.fTempPerm50;
		let mut fTemp68: F32 = self.fTempPerm51;
		let mut fTemp69: F32 = self.fTempPerm52;
		let mut fSlow238: F32 = ((iSlow11 != 0) as i32) as u32 as F32;
		let mut iTemp70: i32 = self.iTempPerm53;
		let mut iSlow239: i32 = (fSlow231) as i32;
		let mut fTemp71: F32 = self.fTempPerm54;
		let mut fTemp72: F32 = self.fTempPerm55;
		let mut fTemp73: F32 = self.fTempPerm56;
		let mut fTemp74: F32 = self.fTempPerm57;
		let mut fTemp75: F32 = self.fTempPerm58;
		let mut fTemp76: F32 = self.fTempPerm59;
		let mut fTemp77: F32 = self.fTempPerm60;
		let mut fSlow240: F32 = self.fConst0 / fSlow105;
		let mut fTemp78: F32 = self.fTempPerm61;
		let mut fTemp79: F32 = self.fTempPerm62;
		let mut iSlow241: i32 = (fSlow231 >= 2.0) as i32;
		let mut iSlow242: i32 = (fSlow231 >= 1.0) as i32;
		let mut fTemp80: F32 = self.fTempPerm63;
		let mut iSlow243: i32 = (fSlow231 >= 3.0) as i32;
		let mut fSlow244: F32 = (if iSlow243 != 0 {0} else {2}) as F32;
		let mut fSlow245: F32 = if iSlow241 != 0 {fSlow244} else {if iSlow242 != 0 {4.0} else {6.2831855}};
		let mut fTemp81: F32 = self.fTempPerm64;
		let mut fSlow246: F32 = self.fConst13 * fSlow105;
		let mut fTemp82: F32 = self.fTempPerm65;
		let mut fTemp83: F32 = self.fTempPerm66;
		let mut fTemp84: F32 = self.fTempPerm67;
		let mut fTemp85: F32 = self.fTempPerm68;
		let mut fTemp86: F32 = self.fTempPerm69;
		let mut fSlow247: F32 = ((iSlow14 != 0) as i32) as u32 as F32;
		let mut fTemp87: F32 = self.fTempPerm70;
		let mut fTemp88: F32 = self.fTempPerm71;
		let mut fTemp89: F32 = self.fTempPerm72;
		let mut fTemp90: F32 = self.fTempPerm73;
		let mut fTemp91: F32 = self.fTempPerm74;
		let mut iTemp92: i32 = self.iTempPerm75;
		let mut fTemp93: F32 = self.fTempPerm76;
		let mut fTemp94: F32 = self.fTempPerm77;
		let mut fTemp95: F32 = self.fTempPerm78;
		let mut fTemp96: F32 = self.fTempPerm79;
		let mut fTemp97: F32 = self.fTempPerm80;
		let mut iTemp98: i32 = self.iTempPerm81;
		let mut fTemp99: F32 = self.fTempPerm82;
		let mut fTemp100: F32 = self.fTempPerm83;
		let mut fTemp101: F32 = self.fTempPerm84;
		let mut fTemp102: F32 = self.fTempPerm85;
		let mut fTemp103: F32 = self.fTempPerm86;
		let mut fTemp104: F32 = self.fTempPerm87;
		let mut fSlow248: F32 = ((iSlow17 != 0) as i32) as u32 as F32;
		let mut fTemp105: F32 = self.fTempPerm88;
		let mut iTemp106: i32 = self.iTempPerm89;
		let mut iTemp107: i32 = self.iTempPerm90;
		let mut iTemp108: i32 = self.iTempPerm91;
		let mut iTemp109: i32 = self.iTempPerm92;
		let mut iTemp110: i32 = self.iTempPerm93;
		let mut iTemp111: i32 = self.iTempPerm94;
		let mut fTemp112: F32 = self.fTempPerm95;
		let mut fTemp113: F32 = self.fTempPerm96;
		let mut fTemp114: F32 = self.fTempPerm97;
		let mut iTemp115: i32 = self.iTempPerm98;
		let mut iTemp116: i32 = self.iTempPerm99;
		let mut iTemp117: i32 = self.iTempPerm100;
		let mut iTemp118: i32 = self.iTempPerm101;
		let mut fTemp119: F32 = self.fTempPerm102;
		let mut fTemp120: F32 = self.fTempPerm103;
		let mut fTemp121: F32 = self.fTempPerm104;
		let mut fTemp122: F32 = self.fTempPerm105;
		let mut fTemp123: F32 = self.fTempPerm106;
		let mut fTemp124: F32 = self.fTempPerm107;
		let mut fTemp125: F32 = self.fTempPerm108;
		let mut fTemp126: F32 = self.fTempPerm109;
		let mut fTemp127: F32 = self.fTempPerm110;
		let mut fSlow249: F32 = ((iSlow20 != 0) as i32) as u32 as F32;
		let mut fTemp128: F32 = self.fTempPerm111;
		let mut fTemp129: F32 = self.fTempPerm112;
		let mut fTemp130: F32 = self.fTempPerm113;
		let mut fTemp131: F32 = self.fTempPerm114;
		let mut fTemp132: F32 = self.fTempPerm115;
		let mut fTemp133: F32 = self.fTempPerm116;
		let mut fTemp134: F32 = self.fTempPerm117;
		let mut fTemp135: F32 = self.fTempPerm118;
		let mut fTemp136: F32 = self.fTempPerm119;
		let mut fTemp137: F32 = self.fTempPerm120;
		let mut fSlow250: F32 = ((iSlow23 != 0) as i32) as u32 as F32;
		let mut iTemp138: i32 = self.iTempPerm121;
		let mut fTemp139: F32 = self.fTempPerm122;
		let mut fTemp140: F32 = self.fTempPerm123;
		let mut fTemp141: F32 = self.fTempPerm124;
		let mut fTemp142: F32 = self.fTempPerm125;
		let mut fTemp143: F32 = self.fTempPerm126;
		let mut fTemp144: F32 = self.fTempPerm127;
		let mut fTemp145: F32 = self.fTempPerm128;
		let mut fTemp146: F32 = self.fTempPerm129;
		let mut fTemp147: F32 = self.fTempPerm130;
		let mut fTemp148: F32 = self.fTempPerm131;
		let mut fTemp149: F32 = self.fTempPerm132;
		let mut fSlow251: F32 = ((iSlow26 != 0) as i32) as u32 as F32;
		let mut fTemp150: F32 = self.fTempPerm133;
		let mut fTemp151: F32 = self.fTempPerm134;
		let mut fTemp152: F32 = self.fTempPerm135;
		let mut iTemp153: i32 = self.iTempPerm136;
		let mut iSlow252: i32 = (fSlow230 >= 2.0) as i32;
		let mut iSlow253: i32 = (fSlow230 >= 3.0) as i32;
		let mut iSlow254: i32 = if iSlow252 != 0 {if iSlow253 != 0 {-2} else {0}} else {0};
		let mut iSlow255: i32 = (fSlow230 >= 1.0) as i32;
		let mut fSlow256: F32 = if iSlow252 != 0 {if iSlow253 != 0 {0.5} else {0.0}} else {if iSlow255 != 0 {0.75} else {0.0}};
		let mut fTemp154: F32 = self.fTempPerm137;
		let mut fTemp155: F32 = self.fTempPerm138;
		let mut fTemp156: F32 = self.fTempPerm139;
		let mut fTemp157: F32 = self.fTempPerm140;
		let mut iTemp158: i32 = self.iTempPerm141;
		let mut iSlow257: i32 = if iSlow252 != 0 {if iSlow253 != 0 {2} else {-2}} else {0};
		let mut fSlow258: F32 = if iSlow252 != 0 {if iSlow253 != 0 {0.0} else {0.5}} else {if iSlow255 != 0 {0.25} else {0.0}};
		let mut fTemp159: F32 = self.fTempPerm142;
		let mut iTemp160: i32 = self.iTempPerm143;
		let mut fTemp161: F32 = self.fTempPerm144;
		let mut fTemp162: F32 = self.fTempPerm145;
		let mut fTemp163: F32 = self.fTempPerm146;
		let mut fTemp164: F32 = self.fTempPerm147;
		let mut fTemp165: F32 = self.fTempPerm148;
		let mut iTemp166: i32 = self.iTempPerm149;
		let mut iSlow259: i32 = if iSlow241 != 0 {if iSlow243 != 0 {-2} else {0}} else {0};
		let mut fTemp167: F32 = self.fTempPerm150;
		let mut fTemp168: F32 = self.fTempPerm151;
		let mut fSlow260: F32 = if iSlow241 != 0 {if iSlow243 != 0 {0.5} else {0.0}} else {if iSlow242 != 0 {0.75} else {0.0}};
		let mut fTemp169: F32 = self.fTempPerm152;
		let mut fTemp170: F32 = self.fTempPerm153;
		let mut fTemp171: F32 = self.fTempPerm154;
		let mut fTemp172: F32 = self.fTempPerm155;
		let mut fSlow261: F32 = (if iSlow241 != 0 {0} else {if iSlow242 != 0 {8} else {0}}) as F32;
		let mut fTemp173: F32 = self.fTempPerm156;
		let mut iSlow262: i32 = if iSlow241 != 0 {if iSlow243 != 0 {2} else {-2}} else {0};
		let mut fTemp174: F32 = self.fTempPerm157;
		let mut fSlow263: F32 = if iSlow241 != 0 {if iSlow243 != 0 {0.0} else {0.5}} else {if iSlow242 != 0 {0.25} else {0.0}};
		let mut fTemp175: F32 = self.fTempPerm158;
		let mut fTemp176: F32 = self.fTempPerm159;
		let mut fTemp177: F32 = self.fTempPerm160;
		let mut fSlow264: F32 = (if iSlow241 != 0 {0} else {if iSlow242 != 0 {-8} else {0}}) as F32;
		let mut fTemp178: F32 = self.fTempPerm161;
		let mut fTemp179: F32 = self.fTempPerm162;
		let mut fTemp180: F32 = self.fTempPerm163;
		let mut fTemp181: F32 = self.fTempPerm164;
		let mut fSlow265: F32 = ((iSlow185 != 0) as i32) as u32 as F32;
		let mut fTemp182: F32 = self.fTempPerm165;
		let mut fTemp183: F32 = self.fTempPerm166;
		let mut fTemp184: F32 = self.fTempPerm167;
		let mut fTemp185: F32 = self.fTempPerm168;
		let mut fSlow266: F32 = ((iSlow29 != 0) as i32) as u32 as F32;
		let mut fTemp186: F32 = self.fTempPerm169;
		let mut fTemp187: F32 = self.fTempPerm170;
		let mut fTemp188: F32 = self.fTempPerm171;
		let mut iTemp189: i32 = self.iTempPerm172;
		let mut fTemp190: F32 = self.fTempPerm173;
		let mut iTemp191: i32 = self.iTempPerm174;
		let mut fTemp192: F32 = self.fTempPerm175;
		let mut iTemp193: i32 = self.iTempPerm176;
		let mut fTemp194: F32 = self.fTempPerm177;
		let mut iTemp195: i32 = self.iTempPerm178;
		let mut fTemp196: F32 = self.fTempPerm179;
		let mut fTemp197: F32 = self.fTempPerm180;
		let mut fTemp198: F32 = self.fTempPerm181;
		let mut fTemp199: F32 = self.fTempPerm182;
		let mut iTemp200: i32 = self.iTempPerm183;
		let mut fTemp201: F32 = self.fTempPerm184;
		let mut fTemp202: F32 = self.fTempPerm185;
		let mut fTemp203: F32 = self.fTempPerm186;
		let mut fTemp204: F32 = self.fTempPerm187;
		let mut fTemp205: F32 = self.fTempPerm188;
		let mut fTemp206: F32 = self.fTempPerm189;
		let mut fTemp207: F32 = self.fTempPerm190;
		let mut fTemp208: F32 = self.fTempPerm191;
		let mut fTemp209: F32 = self.fTempPerm192;
		let mut fTemp210: F32 = self.fTempPerm193;
		let mut fTemp211: F32 = self.fTempPerm194;
		let mut fTemp212: F32 = self.fTempPerm195;
		let mut fTemp213: F32 = self.fTempPerm196;
		let mut fTemp214: F32 = self.fTempPerm197;
		let mut fTemp215: F32 = self.fTempPerm198;
		let mut fTemp216: F32 = self.fTempPerm199;
		let mut fTemp217: F32 = self.fTempPerm200;
		let mut fSlow267: F32 = ((iSlow188 != 0) as i32) as u32 as F32;
		let mut iTemp218: i32 = self.iTempPerm201;
		let mut fTemp219: F32 = self.fTempPerm202;
		let mut fTemp220: F32 = self.fTempPerm203;
		let mut fTemp221: F32 = self.fTempPerm204;
		let mut fTemp222: F32 = self.fTempPerm205;
		let mut fTemp223: F32 = self.fTempPerm206;
		let mut fTemp224: F32 = self.fTempPerm207;
		let mut fTemp225: F32 = self.fTempPerm208;
		let mut fTemp226: F32 = self.fTempPerm209;
		let mut fTemp227: F32 = self.fTempPerm210;
		let mut fTemp228: F32 = self.fTempPerm211;
		let mut fTemp229: F32 = self.fTempPerm212;
		let mut fTemp230: F32 = self.fTempPerm213;
		let mut fTemp231: F32 = self.fTempPerm214;
		let mut fTemp232: F32 = self.fTempPerm215;
		let mut fTemp233: F32 = self.fTempPerm216;
		let mut fTemp234: F32 = self.fTempPerm217;
		let mut fTemp235: F32 = self.fTempPerm218;
		let mut iTemp236: i32 = self.iTempPerm219;
		let mut fTemp237: F32 = self.fTempPerm220;
		let mut fTemp238: F32 = self.fTempPerm221;
		let mut fSlow268: F32 = ((iSlow32 != 0) as i32) as u32 as F32;
		let mut fTemp239: F32 = self.fTempPerm222;
		let mut fTemp240: F32 = self.fTempPerm223;
		let mut iTemp241: i32 = self.iTempPerm224;
		let mut fTemp242: F32 = self.fTempPerm225;
		let mut iTemp243: i32 = self.iTempPerm226;
		let mut fTemp244: F32 = self.fTempPerm227;
		let mut fTemp245: F32 = self.fTempPerm228;
		let mut fTemp246: F32 = self.fTempPerm229;
		let mut fTemp247: F32 = self.fTempPerm230;
		let mut fTemp248: F32 = self.fTempPerm231;
		let mut fTemp249: F32 = self.fTempPerm232;
		let mut fTemp250: F32 = self.fTempPerm233;
		let mut fTemp251: F32 = self.fTempPerm234;
		let mut fTemp252: F32 = self.fTempPerm235;
		let mut fTemp253: F32 = self.fTempPerm236;
		let mut fTemp254: F32 = self.fTempPerm237;
		let mut fTemp255: F32 = self.fTempPerm238;
		let mut fTemp256: F32 = self.fTempPerm239;
		let mut fSlow269: F32 = ((iSlow35 != 0) as i32) as u32 as F32;
		let mut fSlow270: F32 = unsafe { ftbl6OscDspSIG6[((fSlow7) as i32) as usize] };
		let mut fTemp257: F32 = self.fTempPerm240;
		let mut fSlow271: F32 = ((iSlow4 != 0) as i32) as u32 as F32;
		let mut fSlow272: F32 = self.fHslider13;
		let mut fTemp258: F32 = self.fTempPerm241;
		let mut fTemp259: F32 = self.fTempPerm242;
		let mut iTemp260: i32 = self.iTempPerm243;
		let mut iTemp261: i32 = self.iTempPerm244;
		let mut iTemp262: i32 = self.iTempPerm245;
		let mut fSlow273: F32 = self.fEntry12;
		let mut fSlow274: F32 = self.fEntry13;
		let mut fSlow275: F32 = self.fEntry14;
		let mut iTemp263: i32 = self.iTempPerm246;
		let mut fSlow276: F32 = 256.0 * fSlow104;
		let mut fTemp264: F32 = self.fTempPerm247;
		let mut iTemp265: i32 = self.iTempPerm248;
		let mut iTemp266: i32 = self.iTempPerm249;
		let mut iTemp267: i32 = self.iTempPerm250;
		let mut iTemp268: i32 = self.iTempPerm251;
		let mut iTemp269: i32 = self.iTempPerm252;
		let mut iTemp270: i32 = self.iTempPerm253;
		let mut iTemp271: i32 = self.iTempPerm254;
		let mut fSlow277: F32 = self.fConst2 * fSlow104;
		let mut fTemp272: F32 = self.fTempPerm255;
		let mut fTemp273: F32 = self.fTempPerm256;
		let mut iTemp274: i32 = self.iTempPerm257;
		let mut iTemp275: i32 = self.iTempPerm258;
		let mut fTemp276: F32 = self.fTempPerm259;
		let mut fTemp277: F32 = self.fTempPerm260;
		let mut fTemp278: F32 = self.fTempPerm261;
		let mut fTemp279: F32 = self.fTempPerm262;
		let mut fTemp280: F32 = self.fTempPerm263;
		let mut fSlow278: F32 = self.fEntry15;
		let mut iTemp281: i32 = self.iTempPerm264;
		let mut fTemp282: F32 = self.fTempPerm265;
		let mut fTemp283: F32 = self.fTempPerm266;
		let mut fSlow279: F32 = self.fHslider14;
		let mut fTemp284: F32 = self.fTempPerm267;
		let mut fTemp285: F32 = self.fTempPerm268;
		let mut fTemp286: F32 = self.fTempPerm269;
		let mut fTemp287: F32 = self.fTempPerm270;
		let mut fSlow280: F32 = ((iSlow191 != 0) as i32) as u32 as F32;
		let mut fSlow281: F32 = 512.0 * fSlow104;
		let mut fTemp288: F32 = self.fTempPerm271;
		let mut iTemp289: i32 = self.iTempPerm272;
		let mut iTemp290: i32 = self.iTempPerm273;
		let mut iTemp291: i32 = self.iTempPerm274;
		let mut iTemp292: i32 = self.iTempPerm275;
		let mut iTemp293: i32 = self.iTempPerm276;
		let mut iTemp294: i32 = self.iTempPerm277;
		let mut iTemp295: i32 = self.iTempPerm278;
		let mut fTemp296: F32 = self.fTempPerm279;
		let mut fTemp297: F32 = self.fTempPerm280;
		let mut iTemp298: i32 = self.iTempPerm281;
		let mut iTemp299: i32 = self.iTempPerm282;
		let mut iTemp300: i32 = self.iTempPerm283;
		let mut iTemp301: i32 = self.iTempPerm284;
		let mut fTemp302: F32 = self.fTempPerm285;
		let mut fTemp303: F32 = self.fTempPerm286;
		let mut fTemp304: F32 = self.fTempPerm287;
		let mut iTemp305: i32 = self.iTempPerm288;
		let mut fTemp306: F32 = self.fTempPerm289;
		let mut fSlow282: F32 = ((iSlow194 != 0) as i32) as u32 as F32;
		let mut fTemp307: F32 = self.fTempPerm290;
		let mut fSlow283: F32 = ((iSlow212 != 0) as i32) as u32 as F32;
		let mut fTemp308: F32 = self.fTempPerm291;
		let mut fTemp309: F32 = self.fTempPerm292;
		let mut iTemp310: i32 = self.iTempPerm293;
		let mut iTemp311: i32 = self.iTempPerm294;
		let mut iTemp312: i32 = self.iTempPerm295;
		let mut iTemp313: i32 = self.iTempPerm296;
		let mut iTemp314: i32 = self.iTempPerm297;
		let mut iTemp315: i32 = self.iTempPerm298;
		let mut iTemp316: i32 = self.iTempPerm299;
		let mut fTemp317: F32 = self.fTempPerm300;
		let mut iTemp318: i32 = self.iTempPerm301;
		let mut iTemp319: i32 = self.iTempPerm302;
		let mut fTemp320: F32 = self.fTempPerm303;
		let mut fTemp321: F32 = self.fTempPerm304;
		let mut fTemp322: F32 = self.fTempPerm305;
		let mut fTemp323: F32 = self.fTempPerm306;
		let mut fSlow284: F32 = ((iSlow209 != 0) as i32) as u32 as F32;
		let mut fTemp324: F32 = self.fTempPerm307;
		let mut fSlow285: F32 = ((iSlow200 != 0) as i32) as u32 as F32;
		let mut fTemp325: F32 = self.fTempPerm308;
		let mut fTemp326: F32 = self.fTempPerm309;
		let mut fSlow286: F32 = ((iSlow206 != 0) as i32) as u32 as F32;
		let mut fTemp327: F32 = self.fTempPerm310;
		let mut iTemp328: i32 = self.iTempPerm311;
		let mut iTemp329: i32 = self.iTempPerm312;
		let mut fTemp330: F32 = self.fTempPerm313;
		let mut fTemp331: F32 = self.fTempPerm314;
		let mut fTemp332: F32 = self.fTempPerm315;
		let mut fTemp333: F32 = self.fTempPerm316;
		let mut fSlow287: F32 = ((iSlow215 != 0) as i32) as u32 as F32;
		let mut fSlow288: F32 = F32::max(0.0, F32::min(2.0, fSlow66 + -5.0));
		let mut fSlow289: F32 = self.fConst24 * (if (fSlow288 >= 2.0) as i32 != 0 {800} else {if (fSlow288 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp334: F32 = self.fTempPerm317;
		let mut fTemp335: F32 = self.fTempPerm318;
		let mut fTemp336: F32 = self.fTempPerm319;
		let mut fTemp337: F32 = self.fTempPerm320;
		let mut fTemp338: F32 = self.fTempPerm321;
		let mut fSlow290: F32 = ((iSlow221 != 0) as i32) as u32 as F32;
		let mut fTemp339: F32 = self.fTempPerm322;
		let mut fSlow291: F32 = ((iSlow203 != 0) as i32) as u32 as F32;
		let mut fTemp340: F32 = self.fTempPerm323;
		let mut fSlow292: F32 = ((iSlow218 != 0) as i32) as u32 as F32;
		let mut fSlow293: F32 = F32::max(0.0, F32::min(2.0, fSlow66 + -2.0));
		let mut fSlow294: F32 = self.fConst2 * (if (fSlow293 >= 2.0) as i32 != 0 {4000} else {if (fSlow293 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp341: F32 = self.fTempPerm324;
		let mut fTemp342: F32 = self.fTempPerm325;
		let mut fSlow295: F32 = ((iSlow73 != 0) as i32) as u32 as F32;
		let mut fTemp343: F32 = self.fTempPerm326;
		let mut fSlow296: F32 = ((iSlow67 != 0) as i32) as u32 as F32;
		let mut fTemp344: F32 = self.fTempPerm327;
		let mut fSlow297: F32 = ((iSlow76 != 0) as i32) as u32 as F32;
		let mut fTemp345: F32 = self.fTempPerm328;
		let mut fTemp346: F32 = self.fTempPerm329;
		let mut fSlow298: F32 = ((iSlow224 != 0) as i32) as u32 as F32;
		let mut fTemp347: F32 = self.fTempPerm330;
		let mut fSlow299: F32 = ((iSlow197 != 0) as i32) as u32 as F32;
		let mut fTemp348: F32 = self.fTempPerm331;
		let mut fSlow300: F32 = ((iSlow70 != 0) as i32) as u32 as F32;
		let mut fTemp349: F32 = self.fTempPerm332;
		let mut iSlow301: i32 = unsafe { itbl7OscDspSIG7[iSlow40 as usize] };
		let mut fSlow302: F32 = if (fSlow42 > 0.0) as i32 != 0 {(0.049535416 * fSlow44 + 0.6165159) * F32::powf(1e+01, 0.05 * (i32::wrapping_add(iSlow301, i32::wrapping_mul(iSlow43, i32::wrapping_sub(unsafe { itbl8OscDspSIG8[iSlow40 as usize] }, iSlow301)))) as F32)} else {1.0};
		let mut fSlow303: F32 = ((iSlow0 != 0) as i32) as u32 as F32;
		let mut fSlow304: F32 = F32::exp(0.6931472 * self.fEntry17);
		let mut fSlow305: F32 = fSlow304 * F32::exp(0.057762265 * self.fEntry16);
		let mut fSlow306: F32 = self.fConst2 * fSlow305;
		let mut fTemp350: F32 = self.fTempPerm333;
		let mut fTemp351: F32 = self.fTempPerm334;
		let mut iTemp352: i32 = self.iTempPerm335;
		let mut fSlow307: F32 = self.fHslider15;
		let mut fTemp353: F32 = self.fTempPerm336;
		let mut fTemp354: F32 = self.fTempPerm337;
		let mut fSlow308: F32 = self.fHslider16;
		let mut fTemp355: F32 = self.fTempPerm338;
		let mut fTemp356: F32 = self.fTempPerm339;
		let mut fTemp357: F32 = self.fTempPerm340;
		let mut fTemp358: F32 = self.fTempPerm341;
		let mut fTemp359: F32 = self.fTempPerm342;
		let mut fTemp360: F32 = self.fTempPerm343;
		let mut fTemp361: F32 = self.fTempPerm344;
		let mut fSlow309: F32 = self.fConst9 / fSlow305;
		let mut fTemp362: F32 = self.fTempPerm345;
		let mut fSlow310: F32 = self.fEntry18;
		let mut fSlow311: F32 = self.fEntry19;
		let mut fTemp363: F32 = self.fTempPerm346;
		let mut fTemp364: F32 = self.fTempPerm347;
		let mut fTemp365: F32 = self.fTempPerm348;
		let mut fSlow312: F32 = self.fConst10 * fSlow305;
		let mut iTemp366: i32 = self.iTempPerm349;
		let mut fSlow313: F32 = self.fConst11 * fSlow305;
		let mut fTemp367: F32 = self.fTempPerm350;
		let mut fTemp368: F32 = self.fTempPerm351;
		let mut fTemp369: F32 = self.fTempPerm352;
		let mut fTemp370: F32 = self.fTempPerm353;
		let mut fTemp371: F32 = self.fTempPerm354;
		let mut fTemp372: F32 = self.fTempPerm355;
		let mut fTemp373: F32 = self.fTempPerm356;
		let mut fTemp374: F32 = self.fTempPerm357;
		let mut fTemp375: F32 = self.fTempPerm358;
		let mut fTemp376: F32 = self.fTempPerm359;
		let mut fTemp377: F32 = self.fTempPerm360;
		let mut fTemp378: F32 = self.fTempPerm361;
		let mut fTemp379: F32 = self.fTempPerm362;
		let mut fTemp380: F32 = self.fTempPerm363;
		let mut fTemp381: F32 = self.fTempPerm364;
		let mut fTemp382: F32 = self.fTempPerm365;
		let mut fSlow314: F32 = ((iSlow110 != 0) as i32) as u32 as F32;
		let mut iSlow315: i32 = (fSlow310) as i32;
		let mut fSlow316: F32 = self.fConst12 * fSlow305;
		let mut fTemp383: F32 = self.fTempPerm366;
		let mut fTemp384: F32 = self.fTempPerm367;
		let mut fTemp385: F32 = self.fTempPerm368;
		let mut fTemp386: F32 = self.fTempPerm369;
		let mut fTemp387: F32 = self.fTempPerm370;
		let mut fTemp388: F32 = self.fTempPerm371;
		let mut fTemp389: F32 = self.fTempPerm372;
		let mut fTemp390: F32 = self.fTempPerm373;
		let mut fTemp391: F32 = self.fTempPerm374;
		let mut fSlow317: F32 = ((iSlow116 != 0) as i32) as u32 as F32;
		let mut iTemp392: i32 = self.iTempPerm375;
		let mut iSlow318: i32 = (fSlow311) as i32;
		let mut fTemp393: F32 = self.fTempPerm376;
		let mut fTemp394: F32 = self.fTempPerm377;
		let mut fTemp395: F32 = self.fTempPerm378;
		let mut fTemp396: F32 = self.fTempPerm379;
		let mut fTemp397: F32 = self.fTempPerm380;
		let mut fTemp398: F32 = self.fTempPerm381;
		let mut fTemp399: F32 = self.fTempPerm382;
		let mut fSlow319: F32 = self.fConst0 / fSlow305;
		let mut fTemp400: F32 = self.fTempPerm383;
		let mut fTemp401: F32 = self.fTempPerm384;
		let mut iSlow320: i32 = (fSlow311 >= 2.0) as i32;
		let mut iSlow321: i32 = (fSlow311 >= 1.0) as i32;
		let mut fTemp402: F32 = self.fTempPerm385;
		let mut iSlow322: i32 = (fSlow311 >= 3.0) as i32;
		let mut fSlow323: F32 = (if iSlow322 != 0 {0} else {2}) as F32;
		let mut fSlow324: F32 = if iSlow320 != 0 {fSlow323} else {if iSlow321 != 0 {4.0} else {6.2831855}};
		let mut fTemp403: F32 = self.fTempPerm386;
		let mut fSlow325: F32 = self.fConst13 * fSlow305;
		let mut fTemp404: F32 = self.fTempPerm387;
		let mut fTemp405: F32 = self.fTempPerm388;
		let mut fTemp406: F32 = self.fTempPerm389;
		let mut fTemp407: F32 = self.fTempPerm390;
		let mut fTemp408: F32 = self.fTempPerm391;
		let mut fSlow326: F32 = ((iSlow119 != 0) as i32) as u32 as F32;
		let mut fTemp409: F32 = self.fTempPerm392;
		let mut fTemp410: F32 = self.fTempPerm393;
		let mut fTemp411: F32 = self.fTempPerm394;
		let mut fTemp412: F32 = self.fTempPerm395;
		let mut fTemp413: F32 = self.fTempPerm396;
		let mut iTemp414: i32 = self.iTempPerm397;
		let mut fTemp415: F32 = self.fTempPerm398;
		let mut fTemp416: F32 = self.fTempPerm399;
		let mut fTemp417: F32 = self.fTempPerm400;
		let mut fTemp418: F32 = self.fTempPerm401;
		let mut fTemp419: F32 = self.fTempPerm402;
		let mut fTemp420: F32 = self.fTempPerm403;
		let mut iTemp421: i32 = self.iTempPerm404;
		let mut fTemp422: F32 = self.fTempPerm405;
		let mut fTemp423: F32 = self.fTempPerm406;
		let mut fTemp424: F32 = self.fTempPerm407;
		let mut fTemp425: F32 = self.fTempPerm408;
		let mut fTemp426: F32 = self.fTempPerm409;
		let mut fTemp427: F32 = self.fTempPerm410;
		let mut fSlow327: F32 = ((iSlow122 != 0) as i32) as u32 as F32;
		let mut fTemp428: F32 = self.fTempPerm411;
		let mut iTemp429: i32 = self.iTempPerm412;
		let mut iTemp430: i32 = self.iTempPerm413;
		let mut iTemp431: i32 = self.iTempPerm414;
		let mut iTemp432: i32 = self.iTempPerm415;
		let mut iTemp433: i32 = self.iTempPerm416;
		let mut iTemp434: i32 = self.iTempPerm417;
		let mut fTemp435: F32 = self.fTempPerm418;
		let mut fTemp436: F32 = self.fTempPerm419;
		let mut fTemp437: F32 = self.fTempPerm420;
		let mut iTemp438: i32 = self.iTempPerm421;
		let mut iTemp439: i32 = self.iTempPerm422;
		let mut iTemp440: i32 = self.iTempPerm423;
		let mut iTemp441: i32 = self.iTempPerm424;
		let mut fTemp442: F32 = self.fTempPerm425;
		let mut fTemp443: F32 = self.fTempPerm426;
		let mut fTemp444: F32 = self.fTempPerm427;
		let mut fTemp445: F32 = self.fTempPerm428;
		let mut fTemp446: F32 = self.fTempPerm429;
		let mut fTemp447: F32 = self.fTempPerm430;
		let mut fTemp448: F32 = self.fTempPerm431;
		let mut fTemp449: F32 = self.fTempPerm432;
		let mut fTemp450: F32 = self.fTempPerm433;
		let mut fSlow328: F32 = ((iSlow125 != 0) as i32) as u32 as F32;
		let mut fTemp451: F32 = self.fTempPerm434;
		let mut fTemp452: F32 = self.fTempPerm435;
		let mut fTemp453: F32 = self.fTempPerm436;
		let mut fTemp454: F32 = self.fTempPerm437;
		let mut fTemp455: F32 = self.fTempPerm438;
		let mut fTemp456: F32 = self.fTempPerm439;
		let mut fSlow329: F32 = ((iSlow128 != 0) as i32) as u32 as F32;
		let mut iTemp457: i32 = self.iTempPerm440;
		let mut fTemp458: F32 = self.fTempPerm441;
		let mut fTemp459: F32 = self.fTempPerm442;
		let mut fTemp460: F32 = self.fTempPerm443;
		let mut fTemp461: F32 = self.fTempPerm444;
		let mut fTemp462: F32 = self.fTempPerm445;
		let mut fTemp463: F32 = self.fTempPerm446;
		let mut fTemp464: F32 = self.fTempPerm447;
		let mut fTemp465: F32 = self.fTempPerm448;
		let mut fTemp466: F32 = self.fTempPerm449;
		let mut fTemp467: F32 = self.fTempPerm450;
		let mut fTemp468: F32 = self.fTempPerm451;
		let mut fSlow330: F32 = ((iSlow131 != 0) as i32) as u32 as F32;
		let mut fTemp469: F32 = self.fTempPerm452;
		let mut fTemp470: F32 = self.fTempPerm453;
		let mut fTemp471: F32 = self.fTempPerm454;
		let mut iTemp472: i32 = self.iTempPerm455;
		let mut iSlow331: i32 = (fSlow310 >= 2.0) as i32;
		let mut iSlow332: i32 = (fSlow310 >= 3.0) as i32;
		let mut iSlow333: i32 = if iSlow331 != 0 {if iSlow332 != 0 {-2} else {0}} else {0};
		let mut iSlow334: i32 = (fSlow310 >= 1.0) as i32;
		let mut fSlow335: F32 = if iSlow331 != 0 {if iSlow332 != 0 {0.5} else {0.0}} else {if iSlow334 != 0 {0.75} else {0.0}};
		let mut fTemp473: F32 = self.fTempPerm456;
		let mut fTemp474: F32 = self.fTempPerm457;
		let mut fTemp475: F32 = self.fTempPerm458;
		let mut fTemp476: F32 = self.fTempPerm459;
		let mut iTemp477: i32 = self.iTempPerm460;
		let mut iSlow336: i32 = if iSlow331 != 0 {if iSlow332 != 0 {2} else {-2}} else {0};
		let mut fSlow337: F32 = if iSlow331 != 0 {if iSlow332 != 0 {0.0} else {0.5}} else {if iSlow334 != 0 {0.25} else {0.0}};
		let mut fTemp478: F32 = self.fTempPerm461;
		let mut iTemp479: i32 = self.iTempPerm462;
		let mut fTemp480: F32 = self.fTempPerm463;
		let mut fTemp481: F32 = self.fTempPerm464;
		let mut fTemp482: F32 = self.fTempPerm465;
		let mut fTemp483: F32 = self.fTempPerm466;
		let mut fTemp484: F32 = self.fTempPerm467;
		let mut iTemp485: i32 = self.iTempPerm468;
		let mut iSlow338: i32 = if iSlow320 != 0 {if iSlow322 != 0 {-2} else {0}} else {0};
		let mut fTemp486: F32 = self.fTempPerm469;
		let mut fTemp487: F32 = self.fTempPerm470;
		let mut fSlow339: F32 = if iSlow320 != 0 {if iSlow322 != 0 {0.5} else {0.0}} else {if iSlow321 != 0 {0.75} else {0.0}};
		let mut fTemp488: F32 = self.fTempPerm471;
		let mut fTemp489: F32 = self.fTempPerm472;
		let mut fTemp490: F32 = self.fTempPerm473;
		let mut fTemp491: F32 = self.fTempPerm474;
		let mut fSlow340: F32 = (if iSlow320 != 0 {0} else {if iSlow321 != 0 {8} else {0}}) as F32;
		let mut fTemp492: F32 = self.fTempPerm475;
		let mut iSlow341: i32 = if iSlow320 != 0 {if iSlow322 != 0 {2} else {-2}} else {0};
		let mut fTemp493: F32 = self.fTempPerm476;
		let mut fSlow342: F32 = if iSlow320 != 0 {if iSlow322 != 0 {0.0} else {0.5}} else {if iSlow321 != 0 {0.25} else {0.0}};
		let mut fTemp494: F32 = self.fTempPerm477;
		let mut fTemp495: F32 = self.fTempPerm478;
		let mut fTemp496: F32 = self.fTempPerm479;
		let mut fSlow343: F32 = (if iSlow320 != 0 {0} else {if iSlow321 != 0 {-8} else {0}}) as F32;
		let mut fTemp497: F32 = self.fTempPerm480;
		let mut fTemp498: F32 = self.fTempPerm481;
		let mut fTemp499: F32 = self.fTempPerm482;
		let mut fTemp500: F32 = self.fTempPerm483;
		let mut fSlow344: F32 = ((iSlow134 != 0) as i32) as u32 as F32;
		let mut fTemp501: F32 = self.fTempPerm484;
		let mut fTemp502: F32 = self.fTempPerm485;
		let mut fTemp503: F32 = self.fTempPerm486;
		let mut fSlow345: F32 = ((iSlow137 != 0) as i32) as u32 as F32;
		let mut fTemp504: F32 = self.fTempPerm487;
		let mut fTemp505: F32 = self.fTempPerm488;
		let mut fTemp506: F32 = self.fTempPerm489;
		let mut iTemp507: i32 = self.iTempPerm490;
		let mut fTemp508: F32 = self.fTempPerm491;
		let mut iTemp509: i32 = self.iTempPerm492;
		let mut fTemp510: F32 = self.fTempPerm493;
		let mut iTemp511: i32 = self.iTempPerm494;
		let mut fTemp512: F32 = self.fTempPerm495;
		let mut iTemp513: i32 = self.iTempPerm496;
		let mut fTemp514: F32 = self.fTempPerm497;
		let mut fTemp515: F32 = self.fTempPerm498;
		let mut fTemp516: F32 = self.fTempPerm499;
		let mut fTemp517: F32 = self.fTempPerm500;
		let mut iTemp518: i32 = self.iTempPerm501;
		let mut fTemp519: F32 = self.fTempPerm502;
		let mut fTemp520: F32 = self.fTempPerm503;
		let mut fTemp521: F32 = self.fTempPerm504;
		let mut fTemp522: F32 = self.fTempPerm505;
		let mut fTemp523: F32 = self.fTempPerm506;
		let mut fTemp524: F32 = self.fTempPerm507;
		let mut fTemp525: F32 = self.fTempPerm508;
		let mut fTemp526: F32 = self.fTempPerm509;
		let mut fTemp527: F32 = self.fTempPerm510;
		let mut fTemp528: F32 = self.fTempPerm511;
		let mut fTemp529: F32 = self.fTempPerm512;
		let mut fTemp530: F32 = self.fTempPerm513;
		let mut fTemp531: F32 = self.fTempPerm514;
		let mut fTemp532: F32 = self.fTempPerm515;
		let mut fTemp533: F32 = self.fTempPerm516;
		let mut fTemp534: F32 = self.fTempPerm517;
		let mut fTemp535: F32 = self.fTempPerm518;
		let mut fSlow346: F32 = ((iSlow140 != 0) as i32) as u32 as F32;
		let mut iTemp536: i32 = self.iTempPerm519;
		let mut fTemp537: F32 = self.fTempPerm520;
		let mut fTemp538: F32 = self.fTempPerm521;
		let mut fTemp539: F32 = self.fTempPerm522;
		let mut fTemp540: F32 = self.fTempPerm523;
		let mut fTemp541: F32 = self.fTempPerm524;
		let mut fTemp542: F32 = self.fTempPerm525;
		let mut fTemp543: F32 = self.fTempPerm526;
		let mut fTemp544: F32 = self.fTempPerm527;
		let mut fTemp545: F32 = self.fTempPerm528;
		let mut fTemp546: F32 = self.fTempPerm529;
		let mut fTemp547: F32 = self.fTempPerm530;
		let mut fTemp548: F32 = self.fTempPerm531;
		let mut fTemp549: F32 = self.fTempPerm532;
		let mut fTemp550: F32 = self.fTempPerm533;
		let mut fTemp551: F32 = self.fTempPerm534;
		let mut fTemp552: F32 = self.fTempPerm535;
		let mut fTemp553: F32 = self.fTempPerm536;
		let mut iTemp554: i32 = self.iTempPerm537;
		let mut fTemp555: F32 = self.fTempPerm538;
		let mut fTemp556: F32 = self.fTempPerm539;
		let mut fSlow347: F32 = ((iSlow143 != 0) as i32) as u32 as F32;
		let mut fTemp557: F32 = self.fTempPerm540;
		let mut fTemp558: F32 = self.fTempPerm541;
		let mut iTemp559: i32 = self.iTempPerm542;
		let mut fTemp560: F32 = self.fTempPerm543;
		let mut iTemp561: i32 = self.iTempPerm544;
		let mut fTemp562: F32 = self.fTempPerm545;
		let mut fTemp563: F32 = self.fTempPerm546;
		let mut fTemp564: F32 = self.fTempPerm547;
		let mut fTemp565: F32 = self.fTempPerm548;
		let mut fTemp566: F32 = self.fTempPerm549;
		let mut fTemp567: F32 = self.fTempPerm550;
		let mut fTemp568: F32 = self.fTempPerm551;
		let mut fSlow348: F32 = ((iSlow146 != 0) as i32) as u32 as F32;
		let mut fSlow349: F32 = unsafe { ftbl6OscDspSIG6[((fSlow109) as i32) as usize] };
		let mut fTemp569: F32 = self.fTempPerm552;
		let mut fSlow350: F32 = ((iSlow113 != 0) as i32) as u32 as F32;
		let mut fSlow351: F32 = self.fHslider17;
		let mut fTemp570: F32 = self.fTempPerm553;
		let mut fTemp571: F32 = self.fTempPerm554;
		let mut iTemp572: i32 = self.iTempPerm555;
		let mut iTemp573: i32 = self.iTempPerm556;
		let mut iTemp574: i32 = self.iTempPerm557;
		let mut fSlow352: F32 = self.fEntry20;
		let mut fSlow353: F32 = self.fEntry21;
		let mut fSlow354: F32 = self.fEntry22;
		let mut iTemp575: i32 = self.iTempPerm558;
		let mut fSlow355: F32 = 256.0 * fSlow304;
		let mut fTemp576: F32 = self.fTempPerm559;
		let mut iTemp577: i32 = self.iTempPerm560;
		let mut iTemp578: i32 = self.iTempPerm561;
		let mut iTemp579: i32 = self.iTempPerm562;
		let mut iTemp580: i32 = self.iTempPerm563;
		let mut iTemp581: i32 = self.iTempPerm564;
		let mut iTemp582: i32 = self.iTempPerm565;
		let mut iTemp583: i32 = self.iTempPerm566;
		let mut fSlow356: F32 = self.fConst2 * fSlow304;
		let mut fTemp584: F32 = self.fTempPerm567;
		let mut fTemp585: F32 = self.fTempPerm568;
		let mut iTemp586: i32 = self.iTempPerm569;
		let mut iTemp587: i32 = self.iTempPerm570;
		let mut fTemp588: F32 = self.fTempPerm571;
		let mut fTemp589: F32 = self.fTempPerm572;
		let mut fTemp590: F32 = self.fTempPerm573;
		let mut fTemp591: F32 = self.fTempPerm574;
		let mut fTemp592: F32 = self.fTempPerm575;
		let mut fSlow357: F32 = self.fEntry23;
		let mut iTemp593: i32 = self.iTempPerm576;
		let mut fTemp594: F32 = self.fTempPerm577;
		let mut fTemp595: F32 = self.fTempPerm578;
		let mut fSlow358: F32 = self.fHslider18;
		let mut fTemp596: F32 = self.fTempPerm579;
		let mut fTemp597: F32 = self.fTempPerm580;
		let mut fTemp598: F32 = self.fTempPerm581;
		let mut fTemp599: F32 = self.fTempPerm582;
		let mut fSlow359: F32 = ((iSlow149 != 0) as i32) as u32 as F32;
		let mut fSlow360: F32 = 512.0 * fSlow304;
		let mut fTemp600: F32 = self.fTempPerm583;
		let mut iTemp601: i32 = self.iTempPerm584;
		let mut iTemp602: i32 = self.iTempPerm585;
		let mut iTemp603: i32 = self.iTempPerm586;
		let mut iTemp604: i32 = self.iTempPerm587;
		let mut iTemp605: i32 = self.iTempPerm588;
		let mut iTemp606: i32 = self.iTempPerm589;
		let mut iTemp607: i32 = self.iTempPerm590;
		let mut fTemp608: F32 = self.fTempPerm591;
		let mut fTemp609: F32 = self.fTempPerm592;
		let mut iTemp610: i32 = self.iTempPerm593;
		let mut iTemp611: i32 = self.iTempPerm594;
		let mut iTemp612: i32 = self.iTempPerm595;
		let mut iTemp613: i32 = self.iTempPerm596;
		let mut fTemp614: F32 = self.fTempPerm597;
		let mut fTemp615: F32 = self.fTempPerm598;
		let mut fTemp616: F32 = self.fTempPerm599;
		let mut iTemp617: i32 = self.iTempPerm600;
		let mut fTemp618: F32 = self.fTempPerm601;
		let mut fSlow361: F32 = ((iSlow152 != 0) as i32) as u32 as F32;
		let mut fTemp619: F32 = self.fTempPerm602;
		let mut fSlow362: F32 = ((iSlow155 != 0) as i32) as u32 as F32;
		let mut fTemp620: F32 = self.fTempPerm603;
		let mut fTemp621: F32 = self.fTempPerm604;
		let mut iTemp622: i32 = self.iTempPerm605;
		let mut iTemp623: i32 = self.iTempPerm606;
		let mut iTemp624: i32 = self.iTempPerm607;
		let mut iTemp625: i32 = self.iTempPerm608;
		let mut iTemp626: i32 = self.iTempPerm609;
		let mut iTemp627: i32 = self.iTempPerm610;
		let mut iTemp628: i32 = self.iTempPerm611;
		let mut fTemp629: F32 = self.fTempPerm612;
		let mut iTemp630: i32 = self.iTempPerm613;
		let mut iTemp631: i32 = self.iTempPerm614;
		let mut fTemp632: F32 = self.fTempPerm615;
		let mut fTemp633: F32 = self.fTempPerm616;
		let mut fTemp634: F32 = self.fTempPerm617;
		let mut fTemp635: F32 = self.fTempPerm618;
		let mut fSlow363: F32 = ((iSlow158 != 0) as i32) as u32 as F32;
		let mut fTemp636: F32 = self.fTempPerm619;
		let mut fSlow364: F32 = ((iSlow161 != 0) as i32) as u32 as F32;
		let mut fTemp637: F32 = self.fTempPerm620;
		let mut fTemp638: F32 = self.fTempPerm621;
		let mut fSlow365: F32 = ((iSlow164 != 0) as i32) as u32 as F32;
		let mut fTemp639: F32 = self.fTempPerm622;
		let mut iTemp640: i32 = self.iTempPerm623;
		let mut iTemp641: i32 = self.iTempPerm624;
		let mut fTemp642: F32 = self.fTempPerm625;
		let mut fTemp643: F32 = self.fTempPerm626;
		let mut fTemp644: F32 = self.fTempPerm627;
		let mut fTemp645: F32 = self.fTempPerm628;
		let mut fSlow366: F32 = ((iSlow167 != 0) as i32) as u32 as F32;
		let mut fSlow367: F32 = F32::max(0.0, F32::min(2.0, fSlow49 + -5.0));
		let mut fSlow368: F32 = self.fConst24 * (if (fSlow367 >= 2.0) as i32 != 0 {800} else {if (fSlow367 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp646: F32 = self.fTempPerm629;
		let mut fTemp647: F32 = self.fTempPerm630;
		let mut fTemp648: F32 = self.fTempPerm631;
		let mut fTemp649: F32 = self.fTempPerm632;
		let mut fTemp650: F32 = self.fTempPerm633;
		let mut fSlow369: F32 = ((iSlow170 != 0) as i32) as u32 as F32;
		let mut fTemp651: F32 = self.fTempPerm634;
		let mut fSlow370: F32 = ((iSlow173 != 0) as i32) as u32 as F32;
		let mut fTemp652: F32 = self.fTempPerm635;
		let mut fSlow371: F32 = ((iSlow176 != 0) as i32) as u32 as F32;
		let mut fSlow372: F32 = F32::max(0.0, F32::min(2.0, fSlow49 + -2.0));
		let mut fSlow373: F32 = self.fConst2 * (if (fSlow372 >= 2.0) as i32 != 0 {4000} else {if (fSlow372 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp653: F32 = self.fTempPerm636;
		let mut fTemp654: F32 = self.fTempPerm637;
		let mut fSlow374: F32 = ((iSlow50 != 0) as i32) as u32 as F32;
		let mut fTemp655: F32 = self.fTempPerm638;
		let mut fSlow375: F32 = ((iSlow60 != 0) as i32) as u32 as F32;
		let mut fTemp656: F32 = self.fTempPerm639;
		let mut fSlow376: F32 = ((iSlow63 != 0) as i32) as u32 as F32;
		let mut fTemp657: F32 = self.fTempPerm640;
		let mut fTemp658: F32 = self.fTempPerm641;
		let mut fSlow377: F32 = ((iSlow179 != 0) as i32) as u32 as F32;
		let mut fTemp659: F32 = self.fTempPerm642;
		let mut fSlow378: F32 = ((iSlow182 != 0) as i32) as u32 as F32;
		let mut fTemp660: F32 = self.fTempPerm643;
		let mut fSlow379: F32 = ((iSlow54 != 0) as i32) as u32 as F32;
		let mut fTemp661: F32 = self.fTempPerm644;
		let mut fSlow380: F32 = ((iSlow57 != 0) as i32) as u32 as F32;
		let mut fSlow381: F32 = self.fEntry27 + self.fEntry26 + self.fCheckbox2 + self.fHslider34 + self.fHslider33 + self.fHslider32 + self.fHslider31 + self.fHslider30 + self.fHslider29 + self.fHslider28 + self.fHslider27 + self.fEntry25 + self.fEntry24 + self.fCheckbox1 + self.fHslider26 + self.fHslider25 + self.fHslider24 + self.fHslider23 + self.fHslider22 + self.fHslider21 + self.fHslider20 + self.fHslider19;
		let zipped_iterators = outputs0.zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7).zip(outputs8);
		for ((((((((output0, output1), output2), output3), output4), output5), output6), output7), output8) in zipped_iterators {
			self.fVec0[0] = fSlow38;
			self.iVec1[0] = 1;
			let mut iTemp0: i32 = (fSlow38 > self.fVec0[1]) as i32;
			self.fVec2[0] = fSlow46;
			let mut fTemp1: F32 = if ((fSlow46 != self.fVec2[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec1[1] + -1.0};
			self.fRec1[0] = fTemp1;
			self.fRec2[0] = if iTemp0 != 0 {fSlow46} else {if (fTemp1 > 0.0) as i32 != 0 {self.fRec2[1] + (fSlow46 - self.fRec2[1]) / F32::max(1.0, fTemp1)} else {fSlow46}};
			let mut fTemp2: F32 = F32::exp(0.0005776227 * self.fRec2[0] * fSlow45);
			self.fVec3[0] = fSlow48;
			let mut fTemp3: F32 = if ((fSlow48 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec3[1] + -1.0};
			self.fRec3[0] = fTemp3;
			self.fRec4[0] = if iTemp0 != 0 {fSlow48} else {if (fTemp3 > 0.0) as i32 != 0 {self.fRec4[1] + (fSlow48 - self.fRec4[1]) / F32::max(1.0, fTemp3)} else {fSlow48}};
			let mut iTemp4: i32 = (F32::abs(self.fRec4[0]) > 0.0001) as i32;
			let mut fTemp5: F32 = (iTemp4) as F32;
			if (iSlow2 & (iSlow72 & iSlow78)) | (((fTemp5 != 0.0) as i32) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | (iSlow59 & (iSlow56 & iSlow52))))))) != 0 {
				iTemp6 = i32::wrapping_sub(1, self.iVec1[1]);
			}
				if (fTemp5 != 0.0) as i32 != 0 {
					fTemp7 = if iTemp6 != 0 {0.0} else {self.fConst1 + self.fRec6[1]};
					self.fRec6[0] = fTemp7 - F32::floor(fTemp7);
					fTemp8 = if iTemp6 != 0 {0.0} else {fSlow84 + self.fRec7[1]};
					self.fRec7[0] = fTemp8 - F32::floor(fTemp8);
					self.iRec9[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec9[1], i32::wrapping_mul(iSlow85, iTemp6))), 12345);
					iTemp9 = i32::wrapping_mul(1664525, self.iRec9[0] ^ (self.iRec9[0] >> 15));
					self.fRec8[0] = self.fConst3 * self.fRec8[1] + self.fConst5 * (i32::wrapping_mul(1013904223, iTemp9 ^ (iTemp9 >> 13))) as F32;
				}
					let mut fTemp10: F32 = (i32::wrapping_sub(1, iTemp0)) as F32;
					let mut fTemp11: F32 = 1.0 - self.fConst6 * fTemp10;
					if (fTemp5 != 0.0) as i32 != 0 {
						self.fRec10[0] = fSlow86 * fTemp11 + self.fConst6 * fTemp10 * self.fRec10[1];
						fTemp12 = if iTemp6 != 0 {0.0} else {fSlow91 + self.fRec13[1]};
						self.fRec13[0] = fTemp12 - F32::floor(fTemp12);
					}
						self.iRec14[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec14[1]), 12345);
						let mut fTemp13: F32 = (self.iRec14[0]) as F32;
						self.fVec5[0] = fTemp13;
						let mut fTemp14: F32 = 4.656613e-10 * fTemp13;
						if (fTemp5 != 0.0) as i32 != 0 {
							self.fRec12[0] = if (self.fRec13[0] < self.fRec13[1]) as i32 != 0 {fTemp14} else {self.fRec12[1]};
							fTemp15 = F32::floor(32768.0 * self.fRec12[0] + 0.5);
							self.fVec6[0] = fTemp15;
							self.fRec11[0] = fSlow92 * (3.0517578e-05 * (fTemp15 + self.fVec6[1]) - fSlow90 * self.fRec11[1]);
							fTemp16 = fTemp5 * (self.fRec4[0] * (fSlow95 + 2e+01 * self.fRec11[0] + self.fConst7 * self.fRec10[0] * self.fRec8[0]) + OscDsp_faustpower2_f(self.fRec4[0]) * (18.0 * unsafe { ftbl2OscDspSIG2[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec7[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl2OscDspSIG2[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec6[0]) as i32, 65535))) as usize] }));
						}
							self.fRec15[0] = fSlow96 * fTemp11 + self.fConst6 * fTemp10 * self.fRec15[1];
							self.fVec7[0] = fSlow99;
							let mut fTemp17: F32 = if ((fSlow99 != self.fVec7[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec16[1] + -1.0};
							self.fRec16[0] = fTemp17;
							self.fRec17[0] = if iTemp0 != 0 {fSlow99} else {if (fTemp17 > 0.0) as i32 != 0 {self.fRec17[1] + (fSlow99 - self.fRec17[1]) / F32::max(1.0, fTemp17)} else {fSlow99}};
							let mut fTemp18: F32 = F32::exp(0.057762265 * (self.fRec17[0] + fSlow98 * self.fRec15[0] + 0.01 * ((iTemp4 != 0) as i32) as u32 as F32 * fTemp16));
							self.fRec18[0] = fSlow103 + fSlow102 * self.fRec18[1];
							let mut fTemp19: F32 = F32::exp(self.fRec18[0]);
							let mut fTemp20: F32 = fTemp19 * fTemp18 * fTemp2;
							if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))) != 0 {
								fTemp21 = fSlow106 * fTemp20;
								fTemp22 = fTemp21 + self.fRec0[1];
							}
								self.fRec19[0] = if iTemp0 != 0 {fTemp14} else {self.fRec19[1]};
								let mut fTemp23: F32 = 0.5 * (self.fRec19[0] + 1.0);
								let mut fTemp24: F32 = fSlow107 + (fTemp23 - F32::floor(fSlow107 + fTemp23));
								let mut fTemp25: F32 = fSlow108 * fTemp24;
								if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow211)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow199)) | ((iSlow2 & (iSlow72 & iSlow196)) | ((iSlow2 & (iSlow72 & iSlow193)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow190)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow187)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | ((iSlow2 & (iSlow6 & iSlow10)) | ((iSlow59 & (iSlow56 & iSlow184)) | ((iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow160)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | ((iSlow59 & (iSlow56 & iSlow151)) | ((iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow145)) | ((iSlow59 & (iSlow115 & iSlow142)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow136)) | ((iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | ((iSlow59 & (iSlow115 & iSlow121)) | ((iSlow59 & (iSlow115 & iSlow118)) | (iSlow59 & (iSlow115 & iSlow112)))))))))))))))))))))))))))))))))))))))))))))))))))))) != 0 {
									fTemp26 = fTemp25 - F32::floor(fTemp25);
								}
									if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))) != 0 {
										self.fRec0[0] = if iTemp0 != 0 {fTemp26} else {fTemp22 - F32::floor(fTemp22)};
									}
										if iSlow2 & (iSlow6 & iSlow10) != 0 {
											iTemp27 = (self.fRec0[0] < 0.5) as i32;
										}
											if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow190)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow187)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
												self.fVec8[0] = fSlow227;
												fTemp28 = if ((fSlow227 != self.fVec8[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec20[1] + -1.0};
												self.fRec20[0] = fTemp28;
												self.fRec21[0] = if iTemp0 != 0 {fSlow227} else {if (fTemp28 > 0.0) as i32 != 0 {self.fRec21[1] + (fSlow227 - self.fRec21[1]) / F32::max(1.0, fTemp28)} else {fSlow227}};
											}
												if (iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow16)) | (iSlow2 & (iSlow6 & iSlow10)))) != 0 {
													fTemp29 = 1.0 - self.fRec21[0];
												}
													if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow190)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow187)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
														self.fVec9[0] = fSlow228;
														fTemp30 = if ((fSlow228 != self.fVec9[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec22[1] + -1.0};
														self.fRec22[0] = fTemp30;
														self.fRec23[0] = if iTemp0 != 0 {fSlow228} else {if (fTemp30 > 0.0) as i32 != 0 {self.fRec23[1] + (fSlow228 - self.fRec23[1]) / F32::max(1.0, fTemp30)} else {fSlow228}};
													}
														if iSlow2 & (iSlow6 & iSlow10) != 0 {
															fTemp31 = 1.0 - self.fRec23[0];
															fTemp32 = if iTemp27 != 0 {fTemp31} else {fTemp29};
															fTemp33 = if (fTemp32 < 0.999) as i32 != 0 {F32::max(0.01, fTemp32)} else {1.0};
														}
															if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow10)) != 0 {
																fTemp34 = 2.0 * self.fRec0[0];
															}
																if iSlow2 & (iSlow6 & iSlow10) != 0 {
																	fTemp35 = F32::floor(fTemp34);
																	fTemp36 = fTemp34 - (fTemp33 + fTemp35);
																	fTemp37 = F32::max(0.0, F32::min(1.0, fSlow229 * (fTemp36 / fTemp20)));
																	fTemp38 = if iTemp27 != 0 {fSlow231} else {fSlow230};
																	fTemp39 = if (fTemp38 >= 2.0) as i32 != 0 {(if (fTemp38 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp38 >= 1.0) as i32 != 0 {4.0} else {6.2831855}};
																	fTemp40 = fTemp19 * fTemp39 * fTemp18 * fTemp2;
																	iTemp41 = (fTemp38) as i32;
																	fTemp42 = self.fConst0 * F32::abs(fSlow233 * (fTemp20 / fTemp33));
																	fTemp43 = 0.5 * wt_basic(iTemp41, fTemp42, self.fConst0, 0.999999);
																	fTemp44 = fSlow233 * fTemp20;
																	fTemp45 = fTemp34 - fTemp35;
																	fTemp46 = ((((fTemp33 < 1.0) as i32) & ((fTemp45 >= fTemp33) as i32)) & ((fTemp36 < fTemp44) as i32)) as F32;
																	fTemp47 = 1.0 - fTemp37;
																	fTemp48 = if iTemp27 != 0 {fSlow230} else {fSlow231};
																	fTemp49 = if iTemp27 != 0 {fTemp29} else {fTemp31};
																	fTemp50 = if (fTemp49 < 0.999) as i32 != 0 {F32::max(0.01, fTemp49)} else {1.0};
																	fTemp51 = ((fTemp50 == 1.0) as i32) as u32 as F32;
																	fTemp52 = 0.5 * (wt_basic(iTemp41, fTemp42, self.fConst0, 0.0) - fTemp51 * wt_basic((fTemp48) as i32, self.fConst0 * F32::abs(fSlow233 * (fTemp20 / fTemp50)), self.fConst0, 0.999999));
																	fTemp53 = F32::max(0.0, F32::min(1.0, fSlow229 * (fTemp45 / fTemp20)));
																	fTemp54 = 1.0 - fTemp53;
																	fTemp55 = fTemp39 / fTemp33 - fTemp51 * if (fTemp48 >= 2.0) as i32 != 0 {(if (fTemp48 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp48 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} / fTemp50;
																	fTemp56 = ((fTemp45 < fTemp44) as i32) as u32 as F32;
																	self.fVec10[0] = fTemp56 * OscDsp_faustpower2_f(fTemp54) * (fSlow232 * fTemp20 * fTemp55 * fTemp54 - fTemp52) + fTemp46 * OscDsp_faustpower2_f(fTemp47) * (fTemp43 - fSlow232 * (fTemp40 * fTemp47 / fTemp33));
																	self.fVec11[0] = ((fTemp45 < fTemp33) as i32) as u32 as F32 * wt_basic(iTemp41, fTemp42, self.fConst0, fTemp45 / fTemp33);
																	fTemp57 = fSlow9 * (self.fVec11[1] + fTemp56 * OscDsp_faustpower2_f(fTemp53) * (fTemp52 + fSlow232 * fTemp20 * fTemp53 * fTemp55) + self.fVec10[1] - fTemp46 * OscDsp_faustpower2_f(fTemp37) * (fTemp43 + fSlow232 * (fTemp40 * fTemp37 / fTemp33)));
																}
																	if iSlow2 & (iSlow6 & iSlow13) != 0 {
																		fTemp58 = fSlow236 * fTemp20 * F32::exp(0.0046209814 * self.fRec21[0]);
																		fTemp59 = fTemp58 + self.fRec24[1];
																	}
																		if (iSlow2 & (iSlow6 & iSlow13)) | (iSlow59 & (iSlow115 & iSlow118)) != 0 {
																			fTemp60 = fSlow237 * fTemp24;
																			fTemp61 = F32::floor(fTemp60);
																			fTemp62 = fTemp60 - (fTemp61 + F32::floor(fTemp60 - fTemp61));
																		}
																			if iSlow2 & (iSlow6 & iSlow13) != 0 {
																				self.fRec24[0] = if iTemp0 != 0 {fTemp62} else {fTemp59 - F32::floor(fTemp59)};
																			}
																				if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow190)) | ((iSlow2 & (iSlow6 & iSlow25)) | (iSlow2 & (iSlow6 & iSlow13)))) != 0 {
																					fTemp63 = F32::max(0.0, F32::min(1.0, self.fRec23[0]));
																				}
																					if iSlow2 & (iSlow6 & iSlow13) != 0 {
																						fTemp64 = 4.0 * fTemp63;
																						fTemp65 = F32::min(3.0, fTemp64);
																					}
																						if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow13)))))))) != 0 {
																							fTemp66 = F32::abs(fTemp21);
																						}
																							if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow13))))))) != 0 {
																								fTemp67 = self.fConst0 * fTemp66;
																							}
																								if iSlow2 & (iSlow6 & iSlow13) != 0 {
																									fTemp68 = wt_basic(2, fTemp67, self.fConst0, self.fRec0[0]);
																									self.fVec12[0] = wt_basic(0, fTemp67, self.fConst0, self.fRec0[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp65)) + wt_basic(1, fTemp67, self.fConst0, self.fRec0[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp65 + -1.0)) + fTemp68 * F32::max(0.0, 1.0 - F32::abs(fTemp65 + -2.0)) + (fTemp68 - wt_basic(2, fTemp67, self.fConst0, self.fRec0[0] + (-0.5 - 0.42 * F32::max(0.0, fTemp64 + -3.0)))) * F32::max(0.0, 1.0 - F32::abs(fTemp65 + -3.0)) + self.fRec21[0] * wt_basic(iSlow235, self.fConst0 * F32::abs(fTemp58), self.fConst0, self.fRec24[0]);
																									fTemp69 = fSlow12 * (self.fVec12[1] / F32::sqrt(OscDsp_faustpower2_f(self.fRec21[0]) + 1.0));
																								}
																									if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																										iTemp70 = (self.fRec0[0] < self.fRec0[1]) as i32;
																									}
																										if (iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																											self.iRec25[0] = (i32::wrapping_add(iTemp70, self.iRec25[1])) % 2;
																										}
																											if iSlow2 & (iSlow6 & iSlow16) != 0 {
																												self.fVec13[0] = wt_basic(iSlow235, self.fConst0 * F32::abs(fSlow236 * fTemp20), self.fConst0, 0.5 * (self.fRec0[0] + (self.iRec25[0]) as F32));
																											}
																												if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																													fTemp71 = 15.0 * self.fRec23[0];
																													fTemp72 = fTemp71 + 1.0;
																													fTemp73 = fTemp20 * fTemp72;
																													fTemp74 = self.fConst0 * F32::abs(fSlow106 * fTemp73);
																													fTemp75 = wt_basic(iSlow239, fTemp74, self.fConst0, fTemp72);
																													fTemp76 = wt_basic(iSlow239, fTemp74, self.fConst0, 0.0);
																												}
																													if iSlow2 & (iSlow6 & iSlow16) != 0 {
																														fTemp77 = 0.5 * (fTemp76 - fTemp75);
																													}
																														if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																															fTemp78 = F32::max(0.0, F32::min(1.0, fSlow240 * (self.fRec0[0] / fTemp20)));
																															fTemp79 = 1.0 - fTemp78;
																														}
																															if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																fTemp80 = fTemp71 + (1.0 - F32::floor(fTemp72));
																																fTemp81 = fSlow245 - if iSlow241 != 0 {fSlow244} else {if iSlow242 != 0 {(i32::wrapping_sub(4, i32::wrapping_mul(8, ((fTemp80 >= 0.25) as i32) & ((fTemp80 < 0.75) as i32)))) as F32} else {6.2831855 * F32::cos(6.2831855 * fTemp80)}};
																															}
																																if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																																	fTemp82 = (iTemp70) as F32;
																																	fTemp83 = fTemp82 * OscDsp_faustpower2_f(fTemp79);
																																}
																																	if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																		self.fVec14[0] = fTemp83 * (fSlow246 * fTemp73 * fTemp81 * fTemp79 - fTemp77);
																																	}
																																		if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																																			fTemp84 = fTemp82 * OscDsp_faustpower2_f(fTemp78);
																																		}
																																			if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																																				fTemp85 = wt_basic(iSlow239, fTemp74, self.fConst0, self.fRec0[0] * fTemp72);
																																				self.fVec15[0] = fTemp85;
																																			}
																																				if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																					fTemp86 = fSlow15 * (fTemp29 * (self.fVec15[1] + fTemp84 * (fTemp77 + fSlow246 * fTemp73 * fTemp78 * fTemp81) + self.fVec14[1]) + self.fRec21[0] * self.fVec13[1]);
																																				}
																																					if iSlow2 & (iSlow6 & iSlow19) != 0 {
																																						fTemp87 = if self.iRec25[0] != 0 {fSlow231} else {fSlow230};
																																						fTemp88 = 7.0 * self.fRec23[0] + 1.0;
																																						fTemp89 = fTemp88 * (self.fRec21[0] + 1.0);
																																						fTemp90 = if self.iRec25[0] != 0 {fTemp88} else {fTemp89};
																																						fTemp91 = if self.iRec25[0] != 0 {fSlow230} else {fSlow231};
																																						iTemp92 = (fTemp91) as i32;
																																						fTemp93 = if self.iRec25[0] != 0 {fTemp89} else {fTemp88};
																																						fTemp94 = 0.5 * (fTemp93 * wt_basic(iTemp92, fTemp67, self.fConst0, 0.0) - fTemp90 * wt_basic((fTemp87) as i32, fTemp67, self.fConst0, 0.999999));
																																						fTemp95 = if (fTemp91 >= 2.0) as i32 != 0 {(if (fTemp91 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp91 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp93 - if (fTemp87 >= 2.0) as i32 != 0 {(if (fTemp87 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp87 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp90;
																																						self.fVec16[0] = fTemp83 * (fSlow246 * fTemp20 * fTemp79 * fTemp95 - fTemp94);
																																						self.fVec17[0] = wt_basic(iTemp92, fTemp67, self.fConst0, self.fRec0[0]) * fTemp93;
																																						fTemp96 = self.fVec17[1] + fTemp84 * (fTemp94 + fSlow246 * fTemp20 * fTemp78 * fTemp95) + self.fVec16[1];
																																						self.fVec18[0] = fTemp96;
																																						fTemp97 = fTemp96 - self.fVec18[1];
																																						iTemp98 = (F32::abs(fTemp97) < 0.0001) as i32;
																																						fTemp99 = 4.0 * F32::floor(0.25 * (self.fVec18[1] + 1.0));
																																						fTemp100 = self.fVec18[1] - fTemp99;
																																						fTemp101 = 4.0 * F32::floor(0.25 * (fTemp96 + 1.0));
																																						fTemp102 = fTemp96 - fTemp101;
																																						fTemp103 = 0.125 * (fTemp96 + self.fVec18[1]);
																																						fTemp104 = fSlow18 * if iTemp98 != 0 {1.0 - 4.0 * F32::abs(fTemp103 + (-0.25 - F32::floor(fTemp103 + 0.25)))} else {(if (fTemp102 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp102)} else {1.0 - 0.5 * OscDsp_faustpower2_f(fTemp96 + (-2.0 - fTemp101))} - if (fTemp100 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp100)} else {1.0 - 0.5 * OscDsp_faustpower2_f(self.fVec18[1] + (-2.0 - fTemp99))}) / if iTemp98 != 0 {1.0} else {fTemp97}};
																																					}
																																						if iSlow2 & (iSlow6 & iSlow22) != 0 {
																																							fTemp105 = 4.0 * F32::max(0.0, F32::min(1.0, self.fRec21[0]));
																																							iTemp106 = (F32::min(3.0, fTemp105)) as i32;
																																							iTemp107 = i32::wrapping_add(iTemp106, 1);
																																							iTemp108 = (iTemp107 >= 3) as i32;
																																							iTemp109 = (iTemp107 >= 2) as i32;
																																							iTemp110 = (iTemp107 >= 1) as i32;
																																							iTemp111 = (iTemp107 >= 4) as i32;
																																							fTemp112 = 0.5 * (wt_basic(iSlow235, fTemp67, self.fConst0, 0.999999) + 1.0);
																																							fTemp113 = (iTemp106) as F32;
																																							fTemp114 = fTemp105 - fTemp113;
																																							iTemp115 = (iTemp106 >= 3) as i32;
																																							iTemp116 = (iTemp106 >= 2) as i32;
																																							iTemp117 = (iTemp106 >= 1) as i32;
																																							iTemp118 = (iTemp106 >= 4) as i32;
																																							fTemp119 = fTemp113 + (1.0 - fTemp105);
																																							fTemp120 = 0.5 * (wt_basic(iSlow235, fTemp67, self.fConst0, 0.0) + 1.0);
																																							fTemp121 = fTemp76 * (fTemp119 * if iTemp115 != 0 {if iTemp118 != 0 {fTemp120} else {0.0}} else {if iTemp116 != 0 {0.0} else {(if iTemp117 != 0 {0} else {1}) as F32}} + fTemp114 * if iTemp108 != 0 {if iTemp111 != 0 {fTemp120} else {0.0}} else {if iTemp109 != 0 {0.0} else {(if iTemp110 != 0 {0} else {1}) as F32}}) - fTemp75 * (fTemp119 * if iTemp115 != 0 {if iTemp118 != 0 {fTemp112} else {6e-06}} else {if iTemp116 != 0 {0.0} else {if iTemp117 != 0 {2e-06} else {1e-06}}} + fTemp114 * if iTemp108 != 0 {if iTemp111 != 0 {fTemp112} else {6e-06}} else {if iTemp109 != 0 {0.0} else {if iTemp110 != 0 {2e-06} else {1e-06}}});
																																							self.fVec19[0] = fTemp83 * fTemp121;
																																							fTemp122 = 1.0 - self.fRec0[0];
																																							fTemp123 = 1.0 - F32::abs(fTemp34 + -1.0);
																																							fTemp124 = F32::sin(3.1415927 * self.fRec0[0]);
																																							fTemp125 = F32::min(1.0, 6.0 * F32::min(self.fRec0[0], fTemp122));
																																							fTemp126 = 0.5 * (wt_basic(iSlow235, fTemp67, self.fConst0, self.fRec0[0]) + 1.0);
																																							self.fVec20[0] = fTemp85 * (fTemp119 * if iTemp115 != 0 {if iTemp118 != 0 {fTemp126} else {fTemp125}} else {if iTemp116 != 0 {fTemp124} else {if iTemp117 != 0 {fTemp123} else {fTemp122}}} + fTemp114 * if iTemp108 != 0 {if iTemp111 != 0 {fTemp126} else {fTemp125}} else {if iTemp109 != 0 {fTemp124} else {if iTemp110 != 0 {fTemp123} else {fTemp122}}});
																																							fTemp127 = fSlow21 * (self.fVec20[1] + 0.5 * (fTemp84 * fTemp121 - self.fVec19[1]));
																																						}
																																							if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																								fTemp128 = fSlow106 * fTemp20 * (2.17 * fTemp63 + 3.0);
																																								fTemp129 = fTemp128 + self.fRec26[1];
																																							}
																																								if (iSlow2 & (iSlow6 & iSlow25)) | (iSlow59 & (iSlow115 & iSlow130)) != 0 {
																																									fTemp130 = F32::floor(fTemp25 + 0.67);
																																									fTemp131 = fTemp25 + (0.67 - (fTemp130 + F32::floor(fTemp25 + (0.67 - fTemp130))));
																																								}
																																									if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																										self.fRec26[0] = if iTemp0 != 0 {fTemp131} else {fTemp129 - F32::floor(fTemp129)};
																																										self.fVec21[0] = wt_basic(iSlow235, self.fConst0 * F32::abs(fTemp128), self.fConst0, self.fRec26[0]);
																																										fTemp132 = fSlow106 * fTemp20 * (1.41 * fTemp63 + 2.0);
																																										fTemp133 = fTemp132 + self.fRec27[1];
																																									}
																																										if (iSlow2 & (iSlow6 & iSlow25)) | (iSlow59 & (iSlow115 & iSlow130)) != 0 {
																																											fTemp134 = F32::floor(fTemp25 + 0.31);
																																											fTemp135 = fTemp25 + (0.31 - (fTemp134 + F32::floor(fTemp25 + (0.31 - fTemp134))));
																																										}
																																											if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																												self.fRec27[0] = if iTemp0 != 0 {fTemp135} else {fTemp133 - F32::floor(fTemp133)};
																																												self.fVec22[0] = wt_basic(iSlow235, self.fConst0 * F32::abs(fTemp132), self.fConst0, self.fRec27[0]);
																																											}
																																												if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | (iSlow2 & (iSlow6 & iSlow25)))) != 0 {
																																													fTemp136 = wt_basic(iSlow239, fTemp67, self.fConst0, self.fRec0[0]);
																																													self.fVec23[0] = fTemp136;
																																												}
																																													if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																														fTemp137 = fSlow24 * (0.57735026 * fTemp29 * (self.fVec23[1] + self.fVec22[1] + self.fVec21[1]) + 1.5 * self.fRec21[0] * self.fVec23[1] * self.fVec22[1] * self.fVec21[1]);
																																													}
																																														if iSlow2 & (iSlow6 & iSlow28) != 0 {
																																															iTemp138 = (F32::max(0.0, F32::min(1e+01, 10.999 * self.fRec23[0]))) as i32;
																																															fTemp139 = unsafe { ftbl3OscDspSIG3[(i32::wrapping_add(iTemp138, 1)) as usize] };
																																															fTemp140 = fSlow106 * fTemp20 * fTemp139;
																																															fTemp141 = fTemp140 + self.fRec28[1];
																																															fTemp142 = fSlow108 * fTemp24 * fTemp139;
																																															fTemp143 = F32::floor(fTemp142);
																																															self.fRec28[0] = if iTemp0 != 0 {fTemp142 - (fTemp143 + F32::floor(fTemp142 - fTemp143))} else {fTemp141 - F32::floor(fTemp141)};
																																															self.fVec24[0] = wt_basic(iSlow235, self.fConst0 * F32::abs(fTemp140), self.fConst0, self.fRec28[0]);
																																															fTemp144 = unsafe { ftbl3OscDspSIG3[iTemp138 as usize] };
																																															fTemp145 = fSlow106 * fTemp20 * fTemp144;
																																															fTemp146 = fTemp145 + self.fRec29[1];
																																															fTemp147 = fSlow108 * fTemp24 * fTemp144;
																																															fTemp148 = F32::floor(fTemp147);
																																															self.fRec29[0] = if iTemp0 != 0 {fTemp147 - (fTemp148 + F32::floor(fTemp147 - fTemp148))} else {fTemp146 - F32::floor(fTemp146)};
																																															self.fVec25[0] = wt_basic(iSlow235, self.fConst0 * F32::abs(fTemp145), self.fConst0, self.fRec29[0]);
																																															fTemp149 = fSlow27 * self.fVec23[1] * (fTemp29 * self.fVec25[1] + self.fRec21[0] * self.fVec24[1]);
																																														}
																																															if iSlow2 & (iSlow6 & iSlow187) != 0 {
																																																fTemp150 = unsafe { ftbl4OscDspSIG4[((F32::max(0.0, F32::min(9.0, 9.999 * self.fRec21[0]))) as i32) as usize] };
																																																fTemp151 = fSlow106 * fTemp20 * fTemp150;
																																																fTemp152 = F32::max(F32::abs(fTemp151), 1e-09);
																																																iTemp153 = (fTemp151 < 0.0) as i32;
																																																fTemp154 = fTemp151 + self.fRec30[1];
																																																fTemp155 = fSlow108 * fTemp24 * fTemp150;
																																																fTemp156 = F32::floor(fTemp155);
																																																self.fRec30[0] = if iTemp0 != 0 {fTemp155 - (fTemp156 + F32::floor(fTemp155 - fTemp156))} else {fTemp154 - F32::floor(fTemp154)};
																																																fTemp157 = if iTemp153 != 0 {fSlow256 - (self.fRec30[0] + F32::floor(fSlow256 - self.fRec30[0]))} else {self.fRec30[0] - (fSlow256 + F32::floor(self.fRec30[0] - fSlow256))};
																																																iTemp158 = ((fTemp157 < fTemp152) as i32) * if iTemp153 != 0 {-iSlow254} else {iSlow254};
																																																fTemp159 = if iTemp153 != 0 {fSlow258 - (self.fRec30[0] + F32::floor(fSlow258 - self.fRec30[0]))} else {self.fRec30[0] - (fSlow258 + F32::floor(self.fRec30[0] - fSlow258))};
																																																iTemp160 = ((fTemp159 < fTemp152) as i32) * if iTemp153 != 0 {-iSlow257} else {iSlow257};
																																																fTemp161 = F32::floor(self.fRec30[0]);
																																																fTemp162 = self.fRec30[0] - fTemp161;
																																																fTemp163 = if iSlow252 != 0 {if iSlow253 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp162 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec30[0] + (0.5 - (fTemp161 + F32::floor(self.fRec30[0] + (0.5 - fTemp161))))) + -1.0}} else {if iSlow255 != 0 {1.0 - 4.0 * F32::abs(self.fRec30[0] + (-0.25 - (fTemp161 + F32::floor(self.fRec30[0] + (0.25 - fTemp161)))))} else {F32::sin(6.2831855 * fTemp162)}};
																																																self.fVec26[0] = fTemp163;
																																															}
																																																if (iSlow2 & (iSlow6 & iSlow31)) | (iSlow2 & (iSlow6 & iSlow187)) != 0 {
																																																	fTemp164 = OscDsp_faustpower2_f(self.fRec23[0]);
																																																}
																																																	if iSlow2 & (iSlow6 & iSlow187) != 0 {
																																																		fTemp165 = fSlow106 * fTemp20 * (8.0 * fTemp164 * fTemp150 * (0.5 * (fTemp163 + self.fVec26[1] - (i32::wrapping_add(iTemp160, iTemp158)) as F32) + (fTemp159 * (iTemp160) as F32 + fTemp157 * (iTemp158) as F32) / fTemp152) + 1.0);
																																																		iTemp166 = (fTemp165 < 0.0) as i32;
																																																		fTemp167 = 0.5 * (if iTemp166 != 0 {-iSlow259} else {iSlow259}) as F32;
																																																		fTemp168 = F32::max(F32::abs(fTemp165), 1e-09);
																																																		fTemp169 = self.fRec31[1] + fTemp165;
																																																		self.fRec31[0] = if iTemp0 != 0 {fTemp26} else {fTemp169 - F32::floor(fTemp169)};
																																																		fTemp170 = if iTemp166 != 0 {fSlow260 - (self.fRec31[0] + F32::floor(fSlow260 - self.fRec31[0]))} else {self.fRec31[0] - (fSlow260 + F32::floor(self.fRec31[0] - fSlow260))};
																																																		fTemp171 = F32::max(0.0, F32::min(1.0, fTemp170 / fTemp168));
																																																		fTemp172 = 1.0 - fTemp171;
																																																		fTemp173 = ((fTemp170 < fTemp168) as i32) as u32 as F32;
																																																		fTemp174 = 0.5 * (if iTemp166 != 0 {-iSlow262} else {iSlow262}) as F32;
																																																		fTemp175 = if iTemp166 != 0 {fSlow263 - (self.fRec31[0] + F32::floor(fSlow263 - self.fRec31[0]))} else {self.fRec31[0] - (fSlow263 + F32::floor(self.fRec31[0] - fSlow263))};
																																																		fTemp176 = F32::max(0.0, F32::min(1.0, fTemp175 / fTemp168));
																																																		fTemp177 = 1.0 - fTemp176;
																																																		fTemp178 = ((fTemp175 < fTemp168) as i32) as u32 as F32;
																																																		self.fVec27[0] = fTemp178 * OscDsp_faustpower2_f(fTemp177) * (0.16666667 * fTemp168 * fSlow264 * fTemp177 - fTemp174) + fTemp173 * OscDsp_faustpower2_f(fTemp172) * (0.16666667 * fTemp168 * fSlow261 * fTemp172 - fTemp167);
																																																		fTemp179 = F32::floor(self.fRec31[0]);
																																																		fTemp180 = self.fRec31[0] - fTemp179;
																																																		self.fVec28[0] = if iSlow241 != 0 {if iSlow243 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp180 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec31[0] + (0.5 - (fTemp179 + F32::floor(self.fRec31[0] + (0.5 - fTemp179))))) + -1.0}} else {if iSlow242 != 0 {1.0 - 4.0 * F32::abs(self.fRec31[0] + (-0.25 - (fTemp179 + F32::floor(self.fRec31[0] + (0.25 - fTemp179)))))} else {F32::sin(6.2831855 * fTemp180)}};
																																																		fTemp181 = fSlow186 * (fTemp173 * OscDsp_faustpower2_f(fTemp171) * (fTemp167 + 0.16666667 * fTemp168 * fTemp171 * fSlow261) + self.fVec28[1] + fTemp178 * OscDsp_faustpower2_f(fTemp176) * (fTemp174 + 0.16666667 * fTemp168 * fTemp176 * fSlow264) + self.fVec27[1]);
																																																	}
																																																		if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																			fTemp182 = F32::max(1e+01, F32::min(self.fConst14, fSlow105 * fTemp20 * F32::exp(0.6931472 * (8.0 * self.fRec21[0] + -3.0))));
																																																			fTemp183 = 1.0 / F32::tan(self.fConst8 * fTemp182);
																																																		}
																																																			if (iSlow2 & (iSlow6 & iSlow31)) | (iSlow59 & (iSlow115 & iSlow139)) != 0 {
																																																				fTemp184 = 4.656613e-10 * (fTemp13 + self.fVec5[1]);
																																																			}
																																																				if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																					self.fRec32[0] = (fTemp184 - self.fRec32[1] * (1.0 - fTemp183)) / (fTemp183 + 1.0);
																																																					fTemp185 = fSlow30 * (self.fVec23[1] * (1.5 * self.fRec23[0] * self.fRec32[0] * F32::sqrt(self.fConst15 / fTemp182) + 1.0) / F32::sqrt(2.25 * fTemp164 + 1.0));
																																																				}
																																																					if iSlow2 & (iSlow6 & iSlow190) != 0 {
																																																						fTemp186 = 0.6 * fTemp63 * self.fRec34[1] + 1.0;
																																																						fTemp187 = fSlow106 * fTemp20 * fTemp186;
																																																						fTemp188 = self.fRec33[1] + fTemp187;
																																																						iTemp189 = (fTemp188 >= 1.0) as i32;
																																																						self.fRec33[0] = if iTemp0 != 0 {fTemp26} else {fTemp188 - (iTemp189) as F32};
																																																						self.fRec34[0] = if iTemp189 != 0 {fTemp14} else {self.fRec34[1]};
																																																						fTemp190 = F32::max(F32::abs(fTemp187), 1e-09);
																																																						iTemp191 = (fTemp187 < 0.0) as i32;
																																																						fTemp192 = if iTemp191 != 0 {fSlow256 - (self.fRec33[0] + F32::floor(fSlow256 - self.fRec33[0]))} else {self.fRec33[0] - (fSlow256 + F32::floor(self.fRec33[0] - fSlow256))};
																																																						iTemp193 = ((fTemp192 < fTemp190) as i32) * if iTemp191 != 0 {-iSlow254} else {iSlow254};
																																																						fTemp194 = if iTemp191 != 0 {fSlow258 - (self.fRec33[0] + F32::floor(fSlow258 - self.fRec33[0]))} else {self.fRec33[0] - (fSlow258 + F32::floor(self.fRec33[0] - fSlow258))};
																																																						iTemp195 = ((fTemp194 < fTemp190) as i32) * if iTemp191 != 0 {-iSlow257} else {iSlow257};
																																																						fTemp196 = F32::floor(self.fRec33[0]);
																																																						fTemp197 = self.fRec33[0] - fTemp196;
																																																						fTemp198 = if iSlow252 != 0 {if iSlow253 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp197 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec33[0] + (0.5 - (fTemp196 + F32::floor(self.fRec33[0] + (0.5 - fTemp196))))) + -1.0}} else {if iSlow255 != 0 {1.0 - 4.0 * F32::abs(self.fRec33[0] + (-0.25 - (fTemp196 + F32::floor(self.fRec33[0] + (0.25 - fTemp196)))))} else {F32::sin(6.2831855 * fTemp197)}};
																																																						self.fVec29[0] = fTemp198;
																																																						fTemp199 = fSlow106 * fTemp20 * (4.0 * self.fRec21[0] * fTemp186 * (0.5 * (fTemp198 + self.fVec29[1] - (i32::wrapping_add(iTemp195, iTemp193)) as F32) + (fTemp194 * (iTemp195) as F32 + fTemp192 * (iTemp193) as F32) / fTemp190) + 1.0);
																																																						iTemp200 = (fTemp199 < 0.0) as i32;
																																																						fTemp201 = 0.5 * (if iTemp200 != 0 {-iSlow259} else {iSlow259}) as F32;
																																																						fTemp202 = F32::max(F32::abs(fTemp199), 1e-09);
																																																						fTemp203 = self.fRec35[1] + fTemp199;
																																																						self.fRec35[0] = if iTemp0 != 0 {fTemp26} else {fTemp203 - F32::floor(fTemp203)};
																																																						fTemp204 = if iTemp200 != 0 {fSlow260 - (self.fRec35[0] + F32::floor(fSlow260 - self.fRec35[0]))} else {self.fRec35[0] - (fSlow260 + F32::floor(self.fRec35[0] - fSlow260))};
																																																						fTemp205 = F32::max(0.0, F32::min(1.0, fTemp204 / fTemp202));
																																																						fTemp206 = 1.0 - fTemp205;
																																																						fTemp207 = fSlow261 * fTemp202;
																																																						fTemp208 = ((fTemp204 < fTemp202) as i32) as u32 as F32;
																																																						fTemp209 = 0.5 * (if iTemp200 != 0 {-iSlow262} else {iSlow262}) as F32;
																																																						fTemp210 = if iTemp200 != 0 {fSlow263 - (self.fRec35[0] + F32::floor(fSlow263 - self.fRec35[0]))} else {self.fRec35[0] - (fSlow263 + F32::floor(self.fRec35[0] - fSlow263))};
																																																						fTemp211 = F32::max(0.0, F32::min(1.0, fTemp210 / fTemp202));
																																																						fTemp212 = 1.0 - fTemp211;
																																																						fTemp213 = fSlow264 * fTemp202;
																																																						fTemp214 = ((fTemp210 < fTemp202) as i32) as u32 as F32;
																																																						self.fVec30[0] = fTemp214 * OscDsp_faustpower2_f(fTemp212) * (0.16666667 * fTemp213 * fTemp212 - fTemp209) + fTemp208 * OscDsp_faustpower2_f(fTemp206) * (0.16666667 * fTemp207 * fTemp206 - fTemp201);
																																																						fTemp215 = F32::floor(self.fRec35[0]);
																																																						fTemp216 = self.fRec35[0] - fTemp215;
																																																						self.fVec31[0] = if iSlow241 != 0 {if iSlow243 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp216 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec35[0] + (0.5 - (fTemp215 + F32::floor(self.fRec35[0] + (0.5 - fTemp215))))) + -1.0}} else {if iSlow242 != 0 {1.0 - 4.0 * F32::abs(self.fRec35[0] + (-0.25 - (fTemp215 + F32::floor(self.fRec35[0] + (0.25 - fTemp215)))))} else {F32::sin(6.2831855 * fTemp216)}};
																																																						fTemp217 = fSlow189 * (fTemp208 * OscDsp_faustpower2_f(fTemp205) * (fTemp201 + 0.16666667 * fTemp207 * fTemp205) + self.fVec31[1] + fTemp214 * OscDsp_faustpower2_f(fTemp211) * (fTemp209 + 0.16666667 * fTemp213 * fTemp211) + self.fVec30[1]);
																																																					}
																																																						if iSlow2 & (iSlow6 & iSlow34) != 0 {
																																																							iTemp218 = (fTemp21 < 0.0) as i32;
																																																							fTemp219 = 0.5 * (if iTemp218 != 0 {-iSlow259} else {iSlow259}) as F32;
																																																							fTemp220 = F32::max(fTemp66, 1e-09);
																																																							fTemp221 = if iTemp218 != 0 {fSlow260 - (self.fRec0[0] + F32::floor(fSlow260 - self.fRec0[0]))} else {self.fRec0[0] - (fSlow260 + F32::floor(self.fRec0[0] - fSlow260))};
																																																							fTemp222 = F32::max(0.0, F32::min(1.0, fTemp221 / fTemp220));
																																																							fTemp223 = 1.0 - fTemp222;
																																																							fTemp224 = fSlow261 * fTemp220;
																																																							fTemp225 = ((fTemp221 < fTemp220) as i32) as u32 as F32;
																																																							fTemp226 = 0.5 * (if iTemp218 != 0 {-iSlow262} else {iSlow262}) as F32;
																																																							fTemp227 = if iTemp218 != 0 {fSlow263 - (self.fRec0[0] + F32::floor(fSlow263 - self.fRec0[0]))} else {self.fRec0[0] - (fSlow263 + F32::floor(self.fRec0[0] - fSlow263))};
																																																							fTemp228 = F32::max(0.0, F32::min(1.0, fTemp227 / fTemp220));
																																																							fTemp229 = 1.0 - fTemp228;
																																																							fTemp230 = fSlow264 * fTemp220;
																																																							fTemp231 = ((fTemp227 < fTemp220) as i32) as u32 as F32;
																																																							self.fVec32[0] = fTemp231 * OscDsp_faustpower2_f(fTemp229) * (0.16666667 * fTemp230 * fTemp229 - fTemp226) + fTemp225 * OscDsp_faustpower2_f(fTemp223) * (0.16666667 * fTemp224 * fTemp223 - fTemp219);
																																																							fTemp232 = F32::floor(self.fRec0[0]);
																																																							fTemp233 = self.fRec0[0] - fTemp232;
																																																							self.fVec33[0] = if iSlow241 != 0 {if iSlow243 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp233 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec0[0] + (0.5 - (fTemp232 + F32::floor(self.fRec0[0] + (0.5 - fTemp232))))) + -1.0}} else {if iSlow242 != 0 {1.0 - 4.0 * F32::abs(self.fRec0[0] + (-0.25 - (fTemp232 + F32::floor(self.fRec0[0] + (0.25 - fTemp232)))))} else {F32::sin(6.2831855 * fTemp233)}};
																																																							fTemp234 = (9.0 * self.fRec23[0] + 1.0) * (1.5 * self.fRec21[0] + fTemp225 * OscDsp_faustpower2_f(fTemp222) * (fTemp219 + 0.16666667 * fTemp224 * fTemp222) + self.fVec33[1] + fTemp231 * OscDsp_faustpower2_f(fTemp228) * (fTemp226 + 0.16666667 * fTemp230 * fTemp228) + self.fVec32[1]);
																																																							self.fVec34[0] = fTemp234;
																																																							fTemp235 = fTemp234 - self.fVec34[1];
																																																							iTemp236 = (F32::abs(fTemp235) < 0.0001) as i32;
																																																							fTemp237 = if iTemp236 != 0 {F32::sin(0.7853982 * (fTemp234 + self.fVec34[1]))} else {-(0.63661975 * ((F32::cos(1.5707964 * fTemp234) - F32::cos(1.5707964 * self.fVec34[1])) / if iTemp236 != 0 {1.0} else {fTemp235}))};
																																																							self.fVec35[0] = fTemp237;
																																																							self.fRec36[0] = 0.995 * self.fRec36[1] + fTemp237 - self.fVec35[1];
																																																							fTemp238 = fSlow33 * self.fRec36[0];
																																																						}
																																																							if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																								fTemp239 = 1.0 - OscDsp_faustpower2_f(1.0 - fTemp63);
																																																								fTemp240 = 1.0 - 0.5 * OscDsp_faustpower2_f(fTemp63);
																																																								iTemp241 = (self.fRec21[0] > 0.5) as i32;
																																																								fTemp242 = if iTemp241 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * self.fRec21[0])};
																																																								iTemp243 = unsafe { itbl5OscDspSIG5[((F32::max(0.0, F32::min(4.0, 1e+01 * (self.fRec21[0] + -0.5)))) as i32) as usize] };
																																																								fTemp244 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp241, iTemp243))) as F32;
																																																								fTemp245 = fSlow106 * fTemp20 * F32::exp(0.0005776227 * (fTemp244 + 0.83 * fTemp242));
																																																								fTemp246 = fTemp245 + self.fRec37[1];
																																																							}
																																																								if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow59 & (iSlow115 & iSlow148)) != 0 {
																																																									fTemp247 = F32::floor(fTemp25 + 0.87);
																																																									fTemp248 = fTemp25 + (0.87 - (fTemp247 + F32::floor(fTemp25 + (0.87 - fTemp247))));
																																																								}
																																																									if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																										self.fRec37[0] = if iTemp0 != 0 {fTemp248} else {fTemp246 - F32::floor(fTemp246)};
																																																										fTemp249 = fSlow106 * fTemp20 * F32::exp(0.0005776227 * (0.36 * fTemp242 + fTemp244));
																																																										fTemp250 = fTemp249 + self.fRec38[1];
																																																									}
																																																										if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow59 & (iSlow115 & iSlow148)) != 0 {
																																																											fTemp251 = F32::floor(fTemp25 + 0.5);
																																																											fTemp252 = fTemp25 + (0.5 - (fTemp251 + F32::floor(fTemp25 + (0.5 - fTemp251))));
																																																										}
																																																											if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																												self.fRec38[0] = if iTemp0 != 0 {fTemp252} else {fTemp250 - F32::floor(fTemp250)};
																																																												fTemp253 = fSlow106 * fTemp20 * F32::exp(0.0005776227 * ((i32::wrapping_mul(0, iTemp243)) as F32 - fTemp242)) + self.fRec39[1];
																																																											}
																																																												if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow59 & (iSlow115 & iSlow148)) != 0 {
																																																													fTemp254 = F32::floor(fTemp25 + 0.13);
																																																													fTemp255 = fTemp25 + (0.13 - (fTemp254 + F32::floor(fTemp25 + (0.13 - fTemp254))));
																																																												}
																																																													if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																														self.fRec39[0] = if iTemp0 != 0 {fTemp255} else {fTemp253 - F32::floor(fTemp253)};
																																																														self.fVec36[0] = fTemp136 * fTemp240 + 0.5 * fTemp239 * (wt_basic(iSlow235, self.fConst0 * F32::abs(fSlow106 * fTemp20 * F32::exp(-0.0005776227 * fTemp242)), self.fConst0, self.fRec39[0]) + wt_basic(iSlow235, self.fConst0 * F32::abs(fTemp249), self.fConst0, self.fRec38[0]) + wt_basic(iSlow235, self.fConst0 * F32::abs(fTemp245), self.fConst0, self.fRec37[0]));
																																																														fTemp256 = fSlow36 * (self.fVec36[1] / F32::sqrt(OscDsp_faustpower2_f(fTemp240) + 0.75 * OscDsp_faustpower2_f(fTemp239)));
																																																													}
																																																														if iSlow2 & iSlow6 != 0 {
																																																															fTemp257 = fSlow5 * fSlow270 * (fSlow269 * fTemp256 + fSlow268 * fTemp238 + fSlow267 * fTemp217 + fSlow266 * fTemp185 + fSlow265 * fTemp181 + fSlow251 * fTemp149 + fSlow250 * fTemp137 + fSlow249 * fTemp127 + fSlow248 * fTemp104 + fSlow247 * fTemp86 + fSlow238 * fTemp69 + fSlow234 * fTemp57);
																																																														}
																																																															if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow211)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow199)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193))))))))))))))) != 0 {
																																																																self.fVec37[0] = fSlow272;
																																																																fTemp258 = if ((fSlow272 != self.fVec37[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec40[1] + -1.0};
																																																																self.fRec40[0] = fTemp258;
																																																																self.fRec41[0] = if iTemp0 != 0 {fSlow272} else {if (fTemp258 > 0.0) as i32 != 0 {self.fRec41[1] + (fSlow272 - self.fRec41[1]) / F32::max(1.0, fTemp258)} else {fSlow272}};
																																																																fTemp259 = 3.0 * F32::max(0.0, F32::min(1.0, self.fRec41[0]));
																																																																iTemp260 = std::cmp::min(2, (fTemp259) as i32);
																																																																iTemp261 = (iTemp260 >= 2) as i32;
																																																																iTemp262 = (iTemp260 >= 1) as i32;
																																																																iTemp263 = i32::wrapping_mul(2176, (if iTemp261 != 0 {fSlow275} else {if iTemp262 != 0 {fSlow274} else {fSlow273}}) as i32);
																																																															}
																																																																if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow199)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193)))))))))))))) != 0 {
																																																																	fTemp264 = fSlow276 * fTemp20;
																																																																	iTemp265 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp264 > self.fConst16) as i32) + ((fTemp264 > self.fConst22) as i32), (fTemp264 > self.fConst21) as i32), (fTemp264 > self.fConst20) as i32), (fTemp264 > self.fConst19) as i32), (fTemp264 > self.fConst18) as i32), (fTemp264 > self.fConst17) as i32);
																																																																	iTemp266 = (iTemp265 >= 4) as i32;
																																																																	iTemp267 = (iTemp265 >= 2) as i32;
																																																																	iTemp268 = (iTemp265 >= 1) as i32;
																																																																	iTemp269 = (iTemp265 >= 3) as i32;
																																																																	iTemp270 = if iTemp266 != 0 {if (iTemp265 >= 6) as i32 != 0 {if (iTemp265 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp265 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp267 != 0 {if iTemp269 != 0 {1792} else {1536}} else {if iTemp268 != 0 {1024} else {0}}};
																																																																	iTemp271 = if iTemp266 != 0 {64} else {if iTemp267 != 0 {if iTemp269 != 0 {128} else {256}} else {if iTemp268 != 0 {512} else {1024}}};
																																																																}
																																																																	if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow211)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow199)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193))))))))))))))) != 0 {
																																																																		fTemp272 = self.fRec42[1] + fSlow277 * fTemp20;
																																																																		self.fRec42[0] = if iTemp0 != 0 {fTemp26} else {fTemp272 - F32::floor(fTemp272)};
																																																																	}
																																																																		if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow199)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193)))))))))))))) != 0 {
																																																																			fTemp273 = self.fRec42[0] * (iTemp271) as F32;
																																																																			iTemp274 = (fTemp273) as i32;
																																																																			iTemp275 = (i32::wrapping_add(iTemp274, 1)) % iTemp271;
																																																																			fTemp276 = (iTemp274) as F32;
																																																																			fTemp277 = fTemp273 - fTemp276;
																																																																			fTemp278 = fTemp276 + (1.0 - fTemp273);
																																																																		}
																																																																			if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow211)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow199)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193))))))))))))))) != 0 {
																																																																				fTemp279 = (iTemp260) as F32;
																																																																				fTemp280 = fTemp259 - fTemp279;
																																																																				iTemp281 = i32::wrapping_mul(2176, (if iTemp261 != 0 {fSlow274} else {if iTemp262 != 0 {fSlow273} else {fSlow278}}) as i32);
																																																																				fTemp282 = fTemp279 + (1.0 - fTemp259);
																																																																			}
																																																																				if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow199)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193)))))))))))))) != 0 {
																																																																					fTemp283 = fTemp282 * (fTemp278 * wt_sample(i32::wrapping_add(iTemp270, i32::wrapping_add(iTemp274, iTemp281))) + fTemp277 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp281, iTemp270), iTemp275))) + fTemp280 * (fTemp278 * wt_sample(i32::wrapping_add(iTemp270, i32::wrapping_add(iTemp274, iTemp263))) + fTemp277 * wt_sample(i32::wrapping_add(iTemp275, i32::wrapping_add(iTemp270, iTemp263))));
																																																																				}
																																																																					if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow211)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193)))))))))))))) != 0 {
																																																																						self.fVec38[0] = fSlow279;
																																																																						fTemp284 = if ((fSlow279 != self.fVec38[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec43[1] + -1.0};
																																																																						self.fRec43[0] = fTemp284;
																																																																						self.fRec44[0] = if iTemp0 != 0 {fSlow279} else {if (fTemp284 > 0.0) as i32 != 0 {self.fRec44[1] + (fSlow279 - self.fRec44[1]) / F32::max(1.0, fTemp284)} else {fSlow279}};
																																																																						fTemp285 = F32::max(0.0, F32::min(1.0, self.fRec44[0]));
																																																																					}
																																																																						if (iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193)))) != 0 {
																																																																							fTemp286 = (1.0 - fTemp285) * fTemp283;
																																																																						}
																																																																							if iSlow2 & (iSlow72 & iSlow193) != 0 {
																																																																								fTemp287 = fSlow192 * (fTemp286 + fTemp285 * if (fTemp283 > 0.0) as i32 != 0 {1.0 - fTemp283} else {-1.0 - fTemp283});
																																																																							}
																																																																								if (iSlow2 & (iSlow72 & iSlow217)) | (iSlow2 & (iSlow72 & iSlow196)) != 0 {
																																																																									fTemp288 = fSlow281 * fTemp20;
																																																																									iTemp289 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp288 > self.fConst16) as i32) + ((fTemp288 > self.fConst22) as i32), (fTemp288 > self.fConst21) as i32), (fTemp288 > self.fConst20) as i32), (fTemp288 > self.fConst19) as i32), (fTemp288 > self.fConst18) as i32), (fTemp288 > self.fConst17) as i32);
																																																																									iTemp290 = (iTemp289 >= 4) as i32;
																																																																									iTemp291 = (iTemp289 >= 2) as i32;
																																																																									iTemp292 = (iTemp289 >= 1) as i32;
																																																																									iTemp293 = (iTemp289 >= 3) as i32;
																																																																									iTemp294 = if iTemp290 != 0 {64} else {if iTemp291 != 0 {if iTemp293 != 0 {128} else {256}} else {if iTemp292 != 0 {512} else {1024}}};
																																																																									iTemp295 = (self.fRec42[0] < 0.5) as i32;
																																																																									fTemp296 = (iTemp294) as F32;
																																																																								}
																																																																									if iSlow2 & (iSlow72 & iSlow196) != 0 {
																																																																										fTemp297 = fTemp296 * if iTemp295 != 0 {2.0 * self.fRec42[0]} else {2.0 * (1.0 - self.fRec42[0])};
																																																																										iTemp298 = (fTemp297) as i32;
																																																																										iTemp299 = (i32::wrapping_add(iTemp298, 1)) % iTemp294;
																																																																									}
																																																																										if (iSlow2 & (iSlow72 & iSlow217)) | (iSlow2 & (iSlow72 & iSlow196)) != 0 {
																																																																											iTemp300 = if iTemp290 != 0 {if (iTemp289 >= 6) as i32 != 0 {if (iTemp289 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp289 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp291 != 0 {if iTemp293 != 0 {1792} else {1536}} else {if iTemp292 != 0 {1024} else {0}}};
																																																																											iTemp301 = i32::wrapping_add(iTemp263, iTemp300);
																																																																										}
																																																																											if iSlow2 & (iSlow72 & iSlow196) != 0 {
																																																																												fTemp302 = (iTemp298) as F32;
																																																																												fTemp303 = fTemp297 - fTemp302;
																																																																												fTemp304 = fTemp302 + (1.0 - fTemp297);
																																																																											}
																																																																												if (iSlow2 & (iSlow72 & iSlow217)) | (iSlow2 & (iSlow72 & iSlow196)) != 0 {
																																																																													iTemp305 = i32::wrapping_add(iTemp281, iTemp300);
																																																																												}
																																																																													if iSlow2 & (iSlow72 & iSlow196) != 0 {
																																																																														fTemp306 = fSlow195 * (fTemp286 + fTemp285 * (fTemp282 * (fTemp304 * wt_sample(i32::wrapping_add(iTemp305, iTemp298)) + fTemp303 * wt_sample(i32::wrapping_add(iTemp305, iTemp299))) + fTemp280 * (fTemp304 * wt_sample(i32::wrapping_add(iTemp301, iTemp298)) + fTemp303 * wt_sample(i32::wrapping_add(iTemp301, iTemp299)))));
																																																																													}
																																																																														if iSlow2 & (iSlow72 & iSlow214) != 0 {
																																																																															fTemp307 = fSlow213 * fTemp283 * (1.0 - fTemp285 * (1.0 - F32::sin(6.2831855 * self.fRec42[0] * (7.0 * fTemp285 + 1.0))));
																																																																														}
																																																																															if iSlow2 & (iSlow72 & iSlow211) != 0 {
																																																																																fTemp308 = 2.0 * fTemp285 + 1.0;
																																																																																fTemp309 = fSlow276 * fTemp20 * fTemp308;
																																																																																iTemp310 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp309 > self.fConst16) as i32) + ((fTemp309 > self.fConst22) as i32), (fTemp309 > self.fConst21) as i32), (fTemp309 > self.fConst20) as i32), (fTemp309 > self.fConst19) as i32), (fTemp309 > self.fConst18) as i32), (fTemp309 > self.fConst17) as i32);
																																																																																iTemp311 = (iTemp310 >= 4) as i32;
																																																																																iTemp312 = (iTemp310 >= 2) as i32;
																																																																																iTemp313 = (iTemp310 >= 1) as i32;
																																																																																iTemp314 = (iTemp310 >= 3) as i32;
																																																																																iTemp315 = if iTemp311 != 0 {if (iTemp310 >= 6) as i32 != 0 {if (iTemp310 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp310 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp312 != 0 {if iTemp314 != 0 {1792} else {1536}} else {if iTemp313 != 0 {1024} else {0}}};
																																																																																iTemp316 = if iTemp311 != 0 {64} else {if iTemp312 != 0 {if iTemp314 != 0 {128} else {256}} else {if iTemp313 != 0 {512} else {1024}}};
																																																																																fTemp317 = F32::powf(self.fRec42[0], fTemp308) * (iTemp316) as F32;
																																																																																iTemp318 = (fTemp317) as i32;
																																																																																iTemp319 = (i32::wrapping_add(iTemp318, 1)) % iTemp316;
																																																																																fTemp320 = (iTemp318) as F32;
																																																																																fTemp321 = fTemp317 - fTemp320;
																																																																																fTemp322 = fTemp320 + (1.0 - fTemp317);
																																																																																fTemp323 = fSlow210 * (fTemp282 * (fTemp322 * wt_sample(i32::wrapping_add(iTemp315, i32::wrapping_add(iTemp281, iTemp318))) + fTemp321 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp281, iTemp315), iTemp319))) + fTemp280 * (fTemp322 * wt_sample(i32::wrapping_add(iTemp315, i32::wrapping_add(iTemp263, iTemp318))) + fTemp321 * wt_sample(i32::wrapping_add(iTemp319, i32::wrapping_add(iTemp263, iTemp315)))));
																																																																															}
																																																																																if iSlow2 & (iSlow72 & iSlow202) != 0 {
																																																																																	fTemp324 = fSlow201 * (fTemp286 + fTemp285 * F32::sin(7.8539815 * fTemp283));
																																																																																}
																																																																																	if iSlow2 & (iSlow72 & iSlow208) != 0 {
																																																																																		fTemp325 = 0.25 * (fTemp283 * (4.0 * fTemp285 + 1.0) + 1.0);
																																																																																		fTemp326 = fSlow207 * (1.0 - 4.0 * F32::abs(fTemp325 + (-0.5 - F32::floor(fTemp325))));
																																																																																	}
																																																																																		if iSlow2 & (iSlow72 & iSlow217) != 0 {
																																																																																			fTemp327 = 2.0 * self.fRec42[0] * fTemp296;
																																																																																			iTemp328 = (fTemp327) as i32;
																																																																																			iTemp329 = (i32::wrapping_add(iTemp328, 1)) % iTemp294;
																																																																																			fTemp330 = (iTemp328) as F32;
																																																																																			fTemp331 = fTemp327 - fTemp330;
																																																																																			fTemp332 = fTemp330 + (1.0 - fTemp327);
																																																																																			fTemp333 = fSlow216 * (fTemp286 + fTemp285 * (iTemp295) as F32 * F32::abs(fTemp282 * (fTemp332 * wt_sample(i32::wrapping_add(iTemp300, i32::wrapping_add(iTemp281, iTemp328))) + fTemp331 * wt_sample(i32::wrapping_add(iTemp305, iTemp329))) + fTemp280 * (fTemp332 * wt_sample(i32::wrapping_add(iTemp300, i32::wrapping_add(iTemp263, iTemp328))) + fTemp331 * wt_sample(i32::wrapping_add(iTemp329, iTemp301)))));
																																																																																		}
																																																																																			if (iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow220)) | (iSlow2 & (iSlow72 & iSlow205))) != 0 {
																																																																																				fTemp334 = F32::tan(self.fConst25 * F32::powf(fSlow289, fTemp285));
																																																																																				fTemp335 = 1.0 / fTemp334;
																																																																																				fTemp336 = (fTemp335 + 1.4142135) / fTemp334 + 1.0;
																																																																																				self.fRec45[0] = fTemp283 - (self.fRec45[2] * ((fTemp335 + -1.4142135) / fTemp334 + 1.0) + 2.0 * self.fRec45[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp334))) / fTemp336;
																																																																																				fTemp337 = (self.fRec45[2] + self.fRec45[0] + 2.0 * self.fRec45[1]) / fTemp336;
																																																																																			}
																																																																																				if iSlow2 & (iSlow72 & iSlow223) != 0 {
																																																																																					fTemp338 = fSlow222 * fTemp337;
																																																																																				}
																																																																																					if iSlow2 & (iSlow72 & iSlow205) != 0 {
																																																																																						fTemp339 = fSlow204 * fTemp337;
																																																																																					}
																																																																																						if iSlow2 & (iSlow72 & iSlow220) != 0 {
																																																																																							fTemp340 = fSlow219 * fTemp337;
																																																																																						}
																																																																																							if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow75)) | (iSlow2 & (iSlow72 & iSlow69))) != 0 {
																																																																																								fTemp341 = if iTemp6 != 0 {0.0} else {self.fRec47[1] + self.fConst2 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow294, fTemp285)), 0.0)};
																																																																																								self.fRec47[0] = fTemp341 - F32::floor(fTemp341);
																																																																																								self.fRec46[0] = if (self.fRec47[0] < self.fRec47[1]) as i32 != 0 {fTemp283} else {self.fRec46[1]};
																																																																																							}
																																																																																								if iSlow2 & (iSlow72 & iSlow75) != 0 {
																																																																																									fTemp342 = fSlow74 * self.fRec46[0];
																																																																																								}
																																																																																									if iSlow2 & (iSlow72 & iSlow69) != 0 {
																																																																																										fTemp343 = fSlow68 * self.fRec46[0];
																																																																																									}
																																																																																										if iSlow2 & (iSlow72 & iSlow78) != 0 {
																																																																																											fTemp344 = fSlow77 * self.fRec46[0];
																																																																																										}
																																																																																											if iSlow2 & (iSlow72 & iSlow226) != 0 {
																																																																																												fTemp345 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp285)) + -1.0));
																																																																																												fTemp346 = fSlow225 * (F32::floor(fTemp283 * fTemp345 + 0.5) / fTemp345);
																																																																																											}
																																																																																												if iSlow2 & (iSlow72 & iSlow199) != 0 {
																																																																																													fTemp347 = fSlow198 * fTemp283;
																																																																																												}
																																																																																													if iSlow2 & iSlow72 != 0 {
																																																																																														fTemp348 = fSlow71 * (fSlow299 * fTemp347 + fSlow298 * fTemp346 + fSlow297 * fTemp344 + fSlow296 * fTemp343 + fSlow295 * fTemp342 + fSlow292 * fTemp340 + fSlow291 * fTemp339 + fSlow290 * fTemp338 + fSlow287 * fTemp333 + fSlow286 * fTemp326 + fSlow285 * fTemp324 + fSlow284 * fTemp323 + fSlow283 * fTemp307 + fSlow282 * fTemp306 + fSlow280 * fTemp287);
																																																																																													}
																																																																																														if iSlow2 != 0 {
																																																																																															fTemp349 = fSlow1 * (fSlow300 * fTemp348 + fSlow271 * fTemp257);
																																																																																														}
																																																																																															*output0 = fSlow303 * fSlow302 * fTemp349;
																																																																																															if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow145)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | ((iSlow59 & (iSlow115 & iSlow121)) | ((iSlow59 & (iSlow115 & iSlow118)) | (iSlow59 & (iSlow115 & iSlow112)))))))))) != 0 {
																																																																																																fTemp350 = fSlow306 * fTemp20;
																																																																																																fTemp351 = fTemp350 + self.fRec48[1];
																																																																																																self.fRec48[0] = if iTemp0 != 0 {fTemp26} else {fTemp351 - F32::floor(fTemp351)};
																																																																																															}
																																																																																																if iSlow59 & (iSlow115 & iSlow112) != 0 {
																																																																																																	iTemp352 = (self.fRec48[0] < 0.5) as i32;
																																																																																																}
																																																																																																	if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow145)) | ((iSlow59 & (iSlow115 & iSlow142)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow136)) | ((iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | ((iSlow59 & (iSlow115 & iSlow121)) | ((iSlow59 & (iSlow115 & iSlow118)) | (iSlow59 & (iSlow115 & iSlow112)))))))))))) != 0 {
																																																																																																		self.fVec39[0] = fSlow307;
																																																																																																		fTemp353 = if ((fSlow307 != self.fVec39[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec49[1] + -1.0};
																																																																																																		self.fRec49[0] = fTemp353;
																																																																																																		self.fRec50[0] = if iTemp0 != 0 {fSlow307} else {if (fTemp353 > 0.0) as i32 != 0 {self.fRec50[1] + (fSlow307 - self.fRec50[1]) / F32::max(1.0, fTemp353)} else {fSlow307}};
																																																																																																	}
																																																																																																		if (iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow121)) | (iSlow59 & (iSlow115 & iSlow112)))) != 0 {
																																																																																																			fTemp354 = 1.0 - self.fRec50[0];
																																																																																																		}
																																																																																																			if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow145)) | ((iSlow59 & (iSlow115 & iSlow142)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow136)) | ((iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | ((iSlow59 & (iSlow115 & iSlow121)) | ((iSlow59 & (iSlow115 & iSlow118)) | (iSlow59 & (iSlow115 & iSlow112)))))))))))) != 0 {
																																																																																																				self.fVec40[0] = fSlow308;
																																																																																																				fTemp355 = if ((fSlow308 != self.fVec40[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec51[1] + -1.0};
																																																																																																				self.fRec51[0] = fTemp355;
																																																																																																				self.fRec52[0] = if iTemp0 != 0 {fSlow308} else {if (fTemp355 > 0.0) as i32 != 0 {self.fRec52[1] + (fSlow308 - self.fRec52[1]) / F32::max(1.0, fTemp355)} else {fSlow308}};
																																																																																																			}
																																																																																																				if iSlow59 & (iSlow115 & iSlow112) != 0 {
																																																																																																					fTemp356 = 1.0 - self.fRec52[0];
																																																																																																					fTemp357 = if iTemp352 != 0 {fTemp356} else {fTemp354};
																																																																																																					fTemp358 = if (fTemp357 < 0.999) as i32 != 0 {F32::max(0.01, fTemp357)} else {1.0};
																																																																																																				}
																																																																																																					if (iSlow59 & (iSlow115 & iSlow127)) | (iSlow59 & (iSlow115 & iSlow112)) != 0 {
																																																																																																						fTemp359 = 2.0 * self.fRec48[0];
																																																																																																					}
																																																																																																						if iSlow59 & (iSlow115 & iSlow112) != 0 {
																																																																																																							fTemp360 = F32::floor(fTemp359);
																																																																																																							fTemp361 = fTemp359 - (fTemp358 + fTemp360);
																																																																																																							fTemp362 = F32::max(0.0, F32::min(1.0, fSlow309 * (fTemp361 / fTemp20)));
																																																																																																							fTemp363 = if iTemp352 != 0 {fSlow311} else {fSlow310};
																																																																																																							fTemp364 = if (fTemp363 >= 2.0) as i32 != 0 {(if (fTemp363 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp363 >= 1.0) as i32 != 0 {4.0} else {6.2831855}};
																																																																																																							fTemp365 = fTemp19 * fTemp364 * fTemp18 * fTemp2;
																																																																																																							iTemp366 = (fTemp363) as i32;
																																																																																																							fTemp367 = self.fConst0 * F32::abs(fSlow313 * (fTemp20 / fTemp358));
																																																																																																							fTemp368 = 0.5 * wt_basic(iTemp366, fTemp367, self.fConst0, 0.999999);
																																																																																																							fTemp369 = fSlow313 * fTemp20;
																																																																																																							fTemp370 = fTemp359 - fTemp360;
																																																																																																							fTemp371 = ((((fTemp358 < 1.0) as i32) & ((fTemp370 >= fTemp358) as i32)) & ((fTemp361 < fTemp369) as i32)) as F32;
																																																																																																							fTemp372 = 1.0 - fTemp362;
																																																																																																							fTemp373 = if iTemp352 != 0 {fSlow310} else {fSlow311};
																																																																																																							fTemp374 = if iTemp352 != 0 {fTemp354} else {fTemp356};
																																																																																																							fTemp375 = if (fTemp374 < 0.999) as i32 != 0 {F32::max(0.01, fTemp374)} else {1.0};
																																																																																																							fTemp376 = ((fTemp375 == 1.0) as i32) as u32 as F32;
																																																																																																							fTemp377 = 0.5 * (wt_basic(iTemp366, fTemp367, self.fConst0, 0.0) - fTemp376 * wt_basic((fTemp373) as i32, self.fConst0 * F32::abs(fSlow313 * (fTemp20 / fTemp375)), self.fConst0, 0.999999));
																																																																																																							fTemp378 = F32::max(0.0, F32::min(1.0, fSlow309 * (fTemp370 / fTemp20)));
																																																																																																							fTemp379 = 1.0 - fTemp378;
																																																																																																							fTemp380 = fTemp364 / fTemp358 - fTemp376 * if (fTemp373 >= 2.0) as i32 != 0 {(if (fTemp373 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp373 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} / fTemp375;
																																																																																																							fTemp381 = ((fTemp370 < fTemp369) as i32) as u32 as F32;
																																																																																																							self.fVec41[0] = fTemp381 * OscDsp_faustpower2_f(fTemp379) * (fSlow312 * fTemp20 * fTemp380 * fTemp379 - fTemp377) + fTemp371 * OscDsp_faustpower2_f(fTemp372) * (fTemp368 - fSlow312 * (fTemp365 * fTemp372 / fTemp358));
																																																																																																							self.fVec42[0] = ((fTemp370 < fTemp358) as i32) as u32 as F32 * wt_basic(iTemp366, fTemp367, self.fConst0, fTemp370 / fTemp358);
																																																																																																							fTemp382 = fSlow111 * (self.fVec42[1] + fTemp381 * OscDsp_faustpower2_f(fTemp378) * (fTemp377 + fSlow312 * fTemp20 * fTemp378 * fTemp380) + self.fVec41[1] - fTemp371 * OscDsp_faustpower2_f(fTemp362) * (fTemp368 + fSlow312 * (fTemp365 * fTemp362 / fTemp358)));
																																																																																																						}
																																																																																																							if iSlow59 & (iSlow115 & iSlow118) != 0 {
																																																																																																								fTemp383 = fSlow316 * fTemp20 * F32::exp(0.0046209814 * self.fRec50[0]);
																																																																																																								fTemp384 = fTemp383 + self.fRec53[1];
																																																																																																								self.fRec53[0] = if iTemp0 != 0 {fTemp62} else {fTemp384 - F32::floor(fTemp384)};
																																																																																																							}
																																																																																																								if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow142)) | ((iSlow59 & (iSlow115 & iSlow130)) | (iSlow59 & (iSlow115 & iSlow118)))) != 0 {
																																																																																																									fTemp385 = F32::max(0.0, F32::min(1.0, self.fRec52[0]));
																																																																																																								}
																																																																																																									if iSlow59 & (iSlow115 & iSlow118) != 0 {
																																																																																																										fTemp386 = 4.0 * fTemp385;
																																																																																																										fTemp387 = F32::min(3.0, fTemp386);
																																																																																																									}
																																																																																																										if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow145)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | (iSlow59 & (iSlow115 & iSlow118)))))))) != 0 {
																																																																																																											fTemp388 = F32::abs(fTemp350);
																																																																																																										}
																																																																																																											if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | (iSlow59 & (iSlow115 & iSlow118))))))) != 0 {
																																																																																																												fTemp389 = self.fConst0 * fTemp388;
																																																																																																											}
																																																																																																												if iSlow59 & (iSlow115 & iSlow118) != 0 {
																																																																																																													fTemp390 = wt_basic(2, fTemp389, self.fConst0, self.fRec48[0]);
																																																																																																													self.fVec43[0] = wt_basic(0, fTemp389, self.fConst0, self.fRec48[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp387)) + wt_basic(1, fTemp389, self.fConst0, self.fRec48[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp387 + -1.0)) + fTemp390 * F32::max(0.0, 1.0 - F32::abs(fTemp387 + -2.0)) + (fTemp390 - wt_basic(2, fTemp389, self.fConst0, self.fRec48[0] + (-0.5 - 0.42 * F32::max(0.0, fTemp386 + -3.0)))) * F32::max(0.0, 1.0 - F32::abs(fTemp387 + -3.0)) + self.fRec50[0] * wt_basic(iSlow315, self.fConst0 * F32::abs(fTemp383), self.fConst0, self.fRec53[0]);
																																																																																																													fTemp391 = fSlow117 * (self.fVec43[1] / F32::sqrt(OscDsp_faustpower2_f(self.fRec50[0]) + 1.0));
																																																																																																												}
																																																																																																													if (iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | (iSlow59 & (iSlow115 & iSlow121))) != 0 {
																																																																																																														iTemp392 = (self.fRec48[0] < self.fRec48[1]) as i32;
																																																																																																													}
																																																																																																														if (iSlow59 & (iSlow115 & iSlow124)) | (iSlow59 & (iSlow115 & iSlow121)) != 0 {
																																																																																																															self.iRec54[0] = (i32::wrapping_add(iTemp392, self.iRec54[1])) % 2;
																																																																																																														}
																																																																																																															if iSlow59 & (iSlow115 & iSlow121) != 0 {
																																																																																																																self.fVec44[0] = wt_basic(iSlow315, self.fConst0 * F32::abs(fSlow316 * fTemp20), self.fConst0, 0.5 * (self.fRec48[0] + (self.iRec54[0]) as F32));
																																																																																																															}
																																																																																																																if (iSlow59 & (iSlow115 & iSlow127)) | (iSlow59 & (iSlow115 & iSlow121)) != 0 {
																																																																																																																	fTemp393 = 15.0 * self.fRec52[0];
																																																																																																																	fTemp394 = fTemp393 + 1.0;
																																																																																																																	fTemp395 = fTemp20 * fTemp394;
																																																																																																																	fTemp396 = self.fConst0 * F32::abs(fSlow306 * fTemp395);
																																																																																																																	fTemp397 = wt_basic(iSlow318, fTemp396, self.fConst0, fTemp394);
																																																																																																																	fTemp398 = wt_basic(iSlow318, fTemp396, self.fConst0, 0.0);
																																																																																																																}
																																																																																																																	if iSlow59 & (iSlow115 & iSlow121) != 0 {
																																																																																																																		fTemp399 = 0.5 * (fTemp398 - fTemp397);
																																																																																																																	}
																																																																																																																		if (iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | (iSlow59 & (iSlow115 & iSlow121))) != 0 {
																																																																																																																			fTemp400 = F32::max(0.0, F32::min(1.0, fSlow319 * (self.fRec48[0] / fTemp20)));
																																																																																																																			fTemp401 = 1.0 - fTemp400;
																																																																																																																		}
																																																																																																																			if iSlow59 & (iSlow115 & iSlow121) != 0 {
																																																																																																																				fTemp402 = fTemp393 + (1.0 - F32::floor(fTemp394));
																																																																																																																				fTemp403 = fSlow324 - if iSlow320 != 0 {fSlow323} else {if iSlow321 != 0 {(i32::wrapping_sub(4, i32::wrapping_mul(8, ((fTemp402 >= 0.25) as i32) & ((fTemp402 < 0.75) as i32)))) as F32} else {6.2831855 * F32::cos(6.2831855 * fTemp402)}};
																																																																																																																			}
																																																																																																																				if (iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | (iSlow59 & (iSlow115 & iSlow121))) != 0 {
																																																																																																																					fTemp404 = (iTemp392) as F32;
																																																																																																																					fTemp405 = fTemp404 * OscDsp_faustpower2_f(fTemp401);
																																																																																																																				}
																																																																																																																					if iSlow59 & (iSlow115 & iSlow121) != 0 {
																																																																																																																						self.fVec45[0] = fTemp405 * (fSlow325 * fTemp395 * fTemp403 * fTemp401 - fTemp399);
																																																																																																																					}
																																																																																																																						if (iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | (iSlow59 & (iSlow115 & iSlow121))) != 0 {
																																																																																																																							fTemp406 = fTemp404 * OscDsp_faustpower2_f(fTemp400);
																																																																																																																						}
																																																																																																																							if (iSlow59 & (iSlow115 & iSlow127)) | (iSlow59 & (iSlow115 & iSlow121)) != 0 {
																																																																																																																								fTemp407 = wt_basic(iSlow318, fTemp396, self.fConst0, self.fRec48[0] * fTemp394);
																																																																																																																								self.fVec46[0] = fTemp407;
																																																																																																																							}
																																																																																																																								if iSlow59 & (iSlow115 & iSlow121) != 0 {
																																																																																																																									fTemp408 = fSlow120 * (fTemp354 * (self.fVec46[1] + fTemp406 * (fTemp399 + fSlow325 * fTemp395 * fTemp400 * fTemp403) + self.fVec45[1]) + self.fRec50[0] * self.fVec44[1]);
																																																																																																																								}
																																																																																																																									if iSlow59 & (iSlow115 & iSlow124) != 0 {
																																																																																																																										fTemp409 = if self.iRec54[0] != 0 {fSlow311} else {fSlow310};
																																																																																																																										fTemp410 = 7.0 * self.fRec52[0] + 1.0;
																																																																																																																										fTemp411 = fTemp410 * (self.fRec50[0] + 1.0);
																																																																																																																										fTemp412 = if self.iRec54[0] != 0 {fTemp410} else {fTemp411};
																																																																																																																										fTemp413 = if self.iRec54[0] != 0 {fSlow310} else {fSlow311};
																																																																																																																										iTemp414 = (fTemp413) as i32;
																																																																																																																										fTemp415 = if self.iRec54[0] != 0 {fTemp411} else {fTemp410};
																																																																																																																										fTemp416 = 0.5 * (fTemp415 * wt_basic(iTemp414, fTemp389, self.fConst0, 0.0) - fTemp412 * wt_basic((fTemp409) as i32, fTemp389, self.fConst0, 0.999999));
																																																																																																																										fTemp417 = if (fTemp413 >= 2.0) as i32 != 0 {(if (fTemp413 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp413 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp415 - if (fTemp409 >= 2.0) as i32 != 0 {(if (fTemp409 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp409 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp412;
																																																																																																																										self.fVec47[0] = fTemp405 * (fSlow325 * fTemp20 * fTemp401 * fTemp417 - fTemp416);
																																																																																																																										fTemp418 = fTemp406 * (fTemp416 + fSlow325 * fTemp20 * fTemp400 * fTemp417);
																																																																																																																										self.fVec48[0] = wt_basic(iTemp414, fTemp389, self.fConst0, self.fRec48[0]) * fTemp415;
																																																																																																																										fTemp419 = self.fVec48[1] + fTemp418 + self.fVec47[1];
																																																																																																																										self.fVec49[0] = fTemp419;
																																																																																																																										fTemp420 = fTemp419 - self.fVec49[1];
																																																																																																																										iTemp421 = (F32::abs(fTemp420) < 0.0001) as i32;
																																																																																																																										fTemp422 = 4.0 * F32::floor(0.25 * (self.fVec49[1] + 1.0));
																																																																																																																										fTemp423 = self.fVec49[1] - fTemp422;
																																																																																																																										fTemp424 = 4.0 * F32::floor(0.25 * (fTemp419 + 1.0));
																																																																																																																										fTemp425 = fTemp419 - fTemp424;
																																																																																																																										fTemp426 = 0.125 * (self.fVec47[1] + fTemp418 + self.fVec48[1] + self.fVec49[1]);
																																																																																																																										fTemp427 = fSlow123 * if iTemp421 != 0 {1.0 - 4.0 * F32::abs(fTemp426 + (-0.25 - F32::floor(fTemp426 + 0.25)))} else {(if (fTemp425 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp425)} else {1.0 - 0.5 * OscDsp_faustpower2_f(fTemp419 + (-2.0 - fTemp424))} - if (fTemp423 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp423)} else {1.0 - 0.5 * OscDsp_faustpower2_f(self.fVec49[1] + (-2.0 - fTemp422))}) / if iTemp421 != 0 {1.0} else {fTemp420}};
																																																																																																																									}
																																																																																																																										if iSlow59 & (iSlow115 & iSlow127) != 0 {
																																																																																																																											fTemp428 = 4.0 * F32::max(0.0, F32::min(1.0, self.fRec50[0]));
																																																																																																																											iTemp429 = (F32::min(3.0, fTemp428)) as i32;
																																																																																																																											iTemp430 = i32::wrapping_add(iTemp429, 1);
																																																																																																																											iTemp431 = (iTemp430 >= 3) as i32;
																																																																																																																											iTemp432 = (iTemp430 >= 2) as i32;
																																																																																																																											iTemp433 = (iTemp430 >= 1) as i32;
																																																																																																																											iTemp434 = (iTemp430 >= 4) as i32;
																																																																																																																											fTemp435 = 0.5 * (wt_basic(iSlow315, fTemp389, self.fConst0, 0.999999) + 1.0);
																																																																																																																											fTemp436 = (iTemp429) as F32;
																																																																																																																											fTemp437 = fTemp428 - fTemp436;
																																																																																																																											iTemp438 = (iTemp429 >= 3) as i32;
																																																																																																																											iTemp439 = (iTemp429 >= 2) as i32;
																																																																																																																											iTemp440 = (iTemp429 >= 1) as i32;
																																																																																																																											iTemp441 = (iTemp429 >= 4) as i32;
																																																																																																																											fTemp442 = fTemp436 + (1.0 - fTemp428);
																																																																																																																											fTemp443 = 0.5 * (wt_basic(iSlow315, fTemp389, self.fConst0, 0.0) + 1.0);
																																																																																																																											fTemp444 = fTemp398 * (fTemp442 * if iTemp438 != 0 {if iTemp441 != 0 {fTemp443} else {0.0}} else {if iTemp439 != 0 {0.0} else {(if iTemp440 != 0 {0} else {1}) as F32}} + fTemp437 * if iTemp431 != 0 {if iTemp434 != 0 {fTemp443} else {0.0}} else {if iTemp432 != 0 {0.0} else {(if iTemp433 != 0 {0} else {1}) as F32}}) - fTemp397 * (fTemp442 * if iTemp438 != 0 {if iTemp441 != 0 {fTemp435} else {6e-06}} else {if iTemp439 != 0 {0.0} else {if iTemp440 != 0 {2e-06} else {1e-06}}} + fTemp437 * if iTemp431 != 0 {if iTemp434 != 0 {fTemp435} else {6e-06}} else {if iTemp432 != 0 {0.0} else {if iTemp433 != 0 {2e-06} else {1e-06}}});
																																																																																																																											self.fVec50[0] = fTemp405 * fTemp444;
																																																																																																																											fTemp445 = 1.0 - self.fRec48[0];
																																																																																																																											fTemp446 = 1.0 - F32::abs(fTemp359 + -1.0);
																																																																																																																											fTemp447 = F32::sin(3.1415927 * self.fRec48[0]);
																																																																																																																											fTemp448 = F32::min(1.0, 6.0 * F32::min(self.fRec48[0], fTemp445));
																																																																																																																											fTemp449 = 0.5 * (wt_basic(iSlow315, fTemp389, self.fConst0, self.fRec48[0]) + 1.0);
																																																																																																																											self.fVec51[0] = fTemp407 * (fTemp442 * if iTemp438 != 0 {if iTemp441 != 0 {fTemp449} else {fTemp448}} else {if iTemp439 != 0 {fTemp447} else {if iTemp440 != 0 {fTemp446} else {fTemp445}}} + fTemp437 * if iTemp431 != 0 {if iTemp434 != 0 {fTemp449} else {fTemp448}} else {if iTemp432 != 0 {fTemp447} else {if iTemp433 != 0 {fTemp446} else {fTemp445}}});
																																																																																																																											fTemp450 = fSlow126 * (self.fVec51[1] + 0.5 * (fTemp406 * fTemp444 - self.fVec50[1]));
																																																																																																																										}
																																																																																																																											if iSlow59 & (iSlow115 & iSlow130) != 0 {
																																																																																																																												fTemp451 = fSlow306 * fTemp20 * (2.17 * fTemp385 + 3.0);
																																																																																																																												fTemp452 = fTemp451 + self.fRec55[1];
																																																																																																																												self.fRec55[0] = if iTemp0 != 0 {fTemp131} else {fTemp452 - F32::floor(fTemp452)};
																																																																																																																												self.fVec52[0] = wt_basic(iSlow315, self.fConst0 * F32::abs(fTemp451), self.fConst0, self.fRec55[0]);
																																																																																																																												fTemp453 = fSlow306 * fTemp20 * (1.41 * fTemp385 + 2.0);
																																																																																																																												fTemp454 = fTemp453 + self.fRec56[1];
																																																																																																																												self.fRec56[0] = if iTemp0 != 0 {fTemp135} else {fTemp454 - F32::floor(fTemp454)};
																																																																																																																												self.fVec53[0] = wt_basic(iSlow315, self.fConst0 * F32::abs(fTemp453), self.fConst0, self.fRec56[0]);
																																																																																																																											}
																																																																																																																												if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow133)) | (iSlow59 & (iSlow115 & iSlow130)))) != 0 {
																																																																																																																													fTemp455 = wt_basic(iSlow318, fTemp389, self.fConst0, self.fRec48[0]);
																																																																																																																													self.fVec54[0] = fTemp455;
																																																																																																																												}
																																																																																																																													if iSlow59 & (iSlow115 & iSlow130) != 0 {
																																																																																																																														fTemp456 = fSlow129 * (0.57735026 * fTemp354 * (self.fVec54[1] + self.fVec53[1] + self.fVec52[1]) + 1.5 * self.fRec50[0] * self.fVec54[1] * self.fVec53[1] * self.fVec52[1]);
																																																																																																																													}
																																																																																																																														if iSlow59 & (iSlow115 & iSlow133) != 0 {
																																																																																																																															iTemp457 = (F32::max(0.0, F32::min(1e+01, 10.999 * self.fRec52[0]))) as i32;
																																																																																																																															fTemp458 = unsafe { ftbl3OscDspSIG3[(i32::wrapping_add(iTemp457, 1)) as usize] };
																																																																																																																															fTemp459 = fSlow306 * fTemp20 * fTemp458;
																																																																																																																															fTemp460 = fTemp459 + self.fRec57[1];
																																																																																																																															fTemp461 = fSlow108 * fTemp24 * fTemp458;
																																																																																																																															fTemp462 = F32::floor(fTemp461);
																																																																																																																															self.fRec57[0] = if iTemp0 != 0 {fTemp461 - (fTemp462 + F32::floor(fTemp461 - fTemp462))} else {fTemp460 - F32::floor(fTemp460)};
																																																																																																																															self.fVec55[0] = wt_basic(iSlow315, self.fConst0 * F32::abs(fTemp459), self.fConst0, self.fRec57[0]);
																																																																																																																															fTemp463 = unsafe { ftbl3OscDspSIG3[iTemp457 as usize] };
																																																																																																																															fTemp464 = fSlow306 * fTemp20 * fTemp463;
																																																																																																																															fTemp465 = fTemp464 + self.fRec58[1];
																																																																																																																															fTemp466 = fSlow108 * fTemp24 * fTemp463;
																																																																																																																															fTemp467 = F32::floor(fTemp466);
																																																																																																																															self.fRec58[0] = if iTemp0 != 0 {fTemp466 - (fTemp467 + F32::floor(fTemp466 - fTemp467))} else {fTemp465 - F32::floor(fTemp465)};
																																																																																																																															self.fVec56[0] = wt_basic(iSlow315, self.fConst0 * F32::abs(fTemp464), self.fConst0, self.fRec58[0]);
																																																																																																																															fTemp468 = fSlow132 * self.fVec54[1] * (fTemp354 * self.fVec56[1] + self.fRec50[0] * self.fVec55[1]);
																																																																																																																														}
																																																																																																																															if iSlow59 & (iSlow115 & iSlow136) != 0 {
																																																																																																																																fTemp469 = unsafe { ftbl4OscDspSIG4[((F32::max(0.0, F32::min(9.0, 9.999 * self.fRec50[0]))) as i32) as usize] };
																																																																																																																																fTemp470 = fSlow306 * fTemp20 * fTemp469;
																																																																																																																																fTemp471 = F32::max(F32::abs(fTemp470), 1e-09);
																																																																																																																																iTemp472 = (fTemp470 < 0.0) as i32;
																																																																																																																																fTemp473 = fTemp470 + self.fRec59[1];
																																																																																																																																fTemp474 = fSlow108 * fTemp24 * fTemp469;
																																																																																																																																fTemp475 = F32::floor(fTemp474);
																																																																																																																																self.fRec59[0] = if iTemp0 != 0 {fTemp474 - (fTemp475 + F32::floor(fTemp474 - fTemp475))} else {fTemp473 - F32::floor(fTemp473)};
																																																																																																																																fTemp476 = if iTemp472 != 0 {fSlow335 - (self.fRec59[0] + F32::floor(fSlow335 - self.fRec59[0]))} else {self.fRec59[0] - (fSlow335 + F32::floor(self.fRec59[0] - fSlow335))};
																																																																																																																																iTemp477 = ((fTemp476 < fTemp471) as i32) * if iTemp472 != 0 {-iSlow333} else {iSlow333};
																																																																																																																																fTemp478 = if iTemp472 != 0 {fSlow337 - (self.fRec59[0] + F32::floor(fSlow337 - self.fRec59[0]))} else {self.fRec59[0] - (fSlow337 + F32::floor(self.fRec59[0] - fSlow337))};
																																																																																																																																iTemp479 = ((fTemp478 < fTemp471) as i32) * if iTemp472 != 0 {-iSlow336} else {iSlow336};
																																																																																																																																fTemp480 = F32::floor(self.fRec59[0]);
																																																																																																																																fTemp481 = self.fRec59[0] - fTemp480;
																																																																																																																																fTemp482 = if iSlow331 != 0 {if iSlow332 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp481 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec59[0] + (0.5 - (fTemp480 + F32::floor(self.fRec59[0] + (0.5 - fTemp480))))) + -1.0}} else {if iSlow334 != 0 {1.0 - 4.0 * F32::abs(self.fRec59[0] + (-0.25 - (fTemp480 + F32::floor(self.fRec59[0] + (0.25 - fTemp480)))))} else {F32::sin(6.2831855 * fTemp481)}};
																																																																																																																																self.fVec57[0] = fTemp482;
																																																																																																																															}
																																																																																																																																if (iSlow59 & (iSlow115 & iSlow139)) | (iSlow59 & (iSlow115 & iSlow136)) != 0 {
																																																																																																																																	fTemp483 = OscDsp_faustpower2_f(self.fRec52[0]);
																																																																																																																																}
																																																																																																																																	if iSlow59 & (iSlow115 & iSlow136) != 0 {
																																																																																																																																		fTemp484 = fSlow306 * fTemp20 * (8.0 * fTemp483 * fTemp469 * (0.5 * (fTemp482 + self.fVec57[1] - (i32::wrapping_add(iTemp479, iTemp477)) as F32) + (fTemp478 * (iTemp479) as F32 + fTemp476 * (iTemp477) as F32) / fTemp471) + 1.0);
																																																																																																																																		iTemp485 = (fTemp484 < 0.0) as i32;
																																																																																																																																		fTemp486 = 0.5 * (if iTemp485 != 0 {-iSlow338} else {iSlow338}) as F32;
																																																																																																																																		fTemp487 = F32::max(F32::abs(fTemp484), 1e-09);
																																																																																																																																		fTemp488 = self.fRec60[1] + fTemp484;
																																																																																																																																		self.fRec60[0] = if iTemp0 != 0 {fTemp26} else {fTemp488 - F32::floor(fTemp488)};
																																																																																																																																		fTemp489 = if iTemp485 != 0 {fSlow339 - (self.fRec60[0] + F32::floor(fSlow339 - self.fRec60[0]))} else {self.fRec60[0] - (fSlow339 + F32::floor(self.fRec60[0] - fSlow339))};
																																																																																																																																		fTemp490 = F32::max(0.0, F32::min(1.0, fTemp489 / fTemp487));
																																																																																																																																		fTemp491 = 1.0 - fTemp490;
																																																																																																																																		fTemp492 = ((fTemp489 < fTemp487) as i32) as u32 as F32;
																																																																																																																																		fTemp493 = 0.5 * (if iTemp485 != 0 {-iSlow341} else {iSlow341}) as F32;
																																																																																																																																		fTemp494 = if iTemp485 != 0 {fSlow342 - (self.fRec60[0] + F32::floor(fSlow342 - self.fRec60[0]))} else {self.fRec60[0] - (fSlow342 + F32::floor(self.fRec60[0] - fSlow342))};
																																																																																																																																		fTemp495 = F32::max(0.0, F32::min(1.0, fTemp494 / fTemp487));
																																																																																																																																		fTemp496 = 1.0 - fTemp495;
																																																																																																																																		fTemp497 = ((fTemp494 < fTemp487) as i32) as u32 as F32;
																																																																																																																																		self.fVec58[0] = fTemp497 * OscDsp_faustpower2_f(fTemp496) * (0.16666667 * fTemp487 * fSlow343 * fTemp496 - fTemp493) + fTemp492 * OscDsp_faustpower2_f(fTemp491) * (0.16666667 * fTemp487 * fSlow340 * fTemp491 - fTemp486);
																																																																																																																																		fTemp498 = F32::floor(self.fRec60[0]);
																																																																																																																																		fTemp499 = self.fRec60[0] - fTemp498;
																																																																																																																																		self.fVec59[0] = if iSlow320 != 0 {if iSlow322 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp499 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec60[0] + (0.5 - (fTemp498 + F32::floor(self.fRec60[0] + (0.5 - fTemp498))))) + -1.0}} else {if iSlow321 != 0 {1.0 - 4.0 * F32::abs(self.fRec60[0] + (-0.25 - (fTemp498 + F32::floor(self.fRec60[0] + (0.25 - fTemp498)))))} else {F32::sin(6.2831855 * fTemp499)}};
																																																																																																																																		fTemp500 = fSlow135 * (fTemp492 * OscDsp_faustpower2_f(fTemp490) * (fTemp486 + 0.16666667 * fTemp487 * fTemp490 * fSlow340) + self.fVec59[1] + fTemp497 * OscDsp_faustpower2_f(fTemp495) * (fTemp493 + 0.16666667 * fTemp487 * fTemp495 * fSlow343) + self.fVec58[1]);
																																																																																																																																	}
																																																																																																																																		if iSlow59 & (iSlow115 & iSlow139) != 0 {
																																																																																																																																			fTemp501 = F32::max(1e+01, F32::min(self.fConst14, fSlow305 * fTemp20 * F32::exp(0.6931472 * (8.0 * self.fRec50[0] + -3.0))));
																																																																																																																																			fTemp502 = 1.0 / F32::tan(self.fConst8 * fTemp501);
																																																																																																																																			self.fRec61[0] = (fTemp184 - self.fRec61[1] * (1.0 - fTemp502)) / (fTemp502 + 1.0);
																																																																																																																																			fTemp503 = fSlow138 * (self.fVec54[1] * (1.5 * self.fRec52[0] * self.fRec61[0] * F32::sqrt(self.fConst15 / fTemp501) + 1.0) / F32::sqrt(2.25 * fTemp483 + 1.0));
																																																																																																																																		}
																																																																																																																																			if iSlow59 & (iSlow115 & iSlow142) != 0 {
																																																																																																																																				fTemp504 = 0.6 * fTemp385 * self.fRec63[1] + 1.0;
																																																																																																																																				fTemp505 = fSlow306 * fTemp20 * fTemp504;
																																																																																																																																				fTemp506 = self.fRec62[1] + fTemp505;
																																																																																																																																				iTemp507 = (fTemp506 >= 1.0) as i32;
																																																																																																																																				self.fRec62[0] = if iTemp0 != 0 {fTemp26} else {fTemp506 - (iTemp507) as F32};
																																																																																																																																				self.fRec63[0] = if iTemp507 != 0 {fTemp14} else {self.fRec63[1]};
																																																																																																																																				fTemp508 = F32::max(F32::abs(fTemp505), 1e-09);
																																																																																																																																				iTemp509 = (fTemp505 < 0.0) as i32;
																																																																																																																																				fTemp510 = if iTemp509 != 0 {fSlow335 - (self.fRec62[0] + F32::floor(fSlow335 - self.fRec62[0]))} else {self.fRec62[0] - (fSlow335 + F32::floor(self.fRec62[0] - fSlow335))};
																																																																																																																																				iTemp511 = ((fTemp510 < fTemp508) as i32) * if iTemp509 != 0 {-iSlow333} else {iSlow333};
																																																																																																																																				fTemp512 = if iTemp509 != 0 {fSlow337 - (self.fRec62[0] + F32::floor(fSlow337 - self.fRec62[0]))} else {self.fRec62[0] - (fSlow337 + F32::floor(self.fRec62[0] - fSlow337))};
																																																																																																																																				iTemp513 = ((fTemp512 < fTemp508) as i32) * if iTemp509 != 0 {-iSlow336} else {iSlow336};
																																																																																																																																				fTemp514 = F32::floor(self.fRec62[0]);
																																																																																																																																				fTemp515 = self.fRec62[0] - fTemp514;
																																																																																																																																				fTemp516 = if iSlow331 != 0 {if iSlow332 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp515 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec62[0] + (0.5 - (fTemp514 + F32::floor(self.fRec62[0] + (0.5 - fTemp514))))) + -1.0}} else {if iSlow334 != 0 {1.0 - 4.0 * F32::abs(self.fRec62[0] + (-0.25 - (fTemp514 + F32::floor(self.fRec62[0] + (0.25 - fTemp514)))))} else {F32::sin(6.2831855 * fTemp515)}};
																																																																																																																																				self.fVec60[0] = fTemp516;
																																																																																																																																				fTemp517 = fSlow306 * fTemp20 * (4.0 * self.fRec50[0] * fTemp504 * (0.5 * (fTemp516 + self.fVec60[1] - (i32::wrapping_add(iTemp513, iTemp511)) as F32) + (fTemp512 * (iTemp513) as F32 + fTemp510 * (iTemp511) as F32) / fTemp508) + 1.0);
																																																																																																																																				iTemp518 = (fTemp517 < 0.0) as i32;
																																																																																																																																				fTemp519 = 0.5 * (if iTemp518 != 0 {-iSlow338} else {iSlow338}) as F32;
																																																																																																																																				fTemp520 = F32::max(F32::abs(fTemp517), 1e-09);
																																																																																																																																				fTemp521 = self.fRec64[1] + fTemp517;
																																																																																																																																				self.fRec64[0] = if iTemp0 != 0 {fTemp26} else {fTemp521 - F32::floor(fTemp521)};
																																																																																																																																				fTemp522 = if iTemp518 != 0 {fSlow339 - (self.fRec64[0] + F32::floor(fSlow339 - self.fRec64[0]))} else {self.fRec64[0] - (fSlow339 + F32::floor(self.fRec64[0] - fSlow339))};
																																																																																																																																				fTemp523 = F32::max(0.0, F32::min(1.0, fTemp522 / fTemp520));
																																																																																																																																				fTemp524 = 1.0 - fTemp523;
																																																																																																																																				fTemp525 = fSlow340 * fTemp520;
																																																																																																																																				fTemp526 = ((fTemp522 < fTemp520) as i32) as u32 as F32;
																																																																																																																																				fTemp527 = 0.5 * (if iTemp518 != 0 {-iSlow341} else {iSlow341}) as F32;
																																																																																																																																				fTemp528 = if iTemp518 != 0 {fSlow342 - (self.fRec64[0] + F32::floor(fSlow342 - self.fRec64[0]))} else {self.fRec64[0] - (fSlow342 + F32::floor(self.fRec64[0] - fSlow342))};
																																																																																																																																				fTemp529 = F32::max(0.0, F32::min(1.0, fTemp528 / fTemp520));
																																																																																																																																				fTemp530 = 1.0 - fTemp529;
																																																																																																																																				fTemp531 = fSlow343 * fTemp520;
																																																																																																																																				fTemp532 = ((fTemp528 < fTemp520) as i32) as u32 as F32;
																																																																																																																																				self.fVec61[0] = fTemp532 * OscDsp_faustpower2_f(fTemp530) * (0.16666667 * fTemp531 * fTemp530 - fTemp527) + fTemp526 * OscDsp_faustpower2_f(fTemp524) * (0.16666667 * fTemp525 * fTemp524 - fTemp519);
																																																																																																																																				fTemp533 = F32::floor(self.fRec64[0]);
																																																																																																																																				fTemp534 = self.fRec64[0] - fTemp533;
																																																																																																																																				self.fVec62[0] = if iSlow320 != 0 {if iSlow322 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp534 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec64[0] + (0.5 - (fTemp533 + F32::floor(self.fRec64[0] + (0.5 - fTemp533))))) + -1.0}} else {if iSlow321 != 0 {1.0 - 4.0 * F32::abs(self.fRec64[0] + (-0.25 - (fTemp533 + F32::floor(self.fRec64[0] + (0.25 - fTemp533)))))} else {F32::sin(6.2831855 * fTemp534)}};
																																																																																																																																				fTemp535 = fSlow141 * (fTemp526 * OscDsp_faustpower2_f(fTemp523) * (fTemp519 + 0.16666667 * fTemp525 * fTemp523) + self.fVec62[1] + fTemp532 * OscDsp_faustpower2_f(fTemp529) * (fTemp527 + 0.16666667 * fTemp531 * fTemp529) + self.fVec61[1]);
																																																																																																																																			}
																																																																																																																																				if iSlow59 & (iSlow115 & iSlow145) != 0 {
																																																																																																																																					iTemp536 = (fTemp350 < 0.0) as i32;
																																																																																																																																					fTemp537 = 0.5 * (if iTemp536 != 0 {-iSlow338} else {iSlow338}) as F32;
																																																																																																																																					fTemp538 = F32::max(fTemp388, 1e-09);
																																																																																																																																					fTemp539 = if iTemp536 != 0 {fSlow339 - (self.fRec48[0] + F32::floor(fSlow339 - self.fRec48[0]))} else {self.fRec48[0] - (fSlow339 + F32::floor(self.fRec48[0] - fSlow339))};
																																																																																																																																					fTemp540 = F32::max(0.0, F32::min(1.0, fTemp539 / fTemp538));
																																																																																																																																					fTemp541 = 1.0 - fTemp540;
																																																																																																																																					fTemp542 = fSlow340 * fTemp538;
																																																																																																																																					fTemp543 = ((fTemp539 < fTemp538) as i32) as u32 as F32;
																																																																																																																																					fTemp544 = 0.5 * (if iTemp536 != 0 {-iSlow341} else {iSlow341}) as F32;
																																																																																																																																					fTemp545 = if iTemp536 != 0 {fSlow342 - (self.fRec48[0] + F32::floor(fSlow342 - self.fRec48[0]))} else {self.fRec48[0] - (fSlow342 + F32::floor(self.fRec48[0] - fSlow342))};
																																																																																																																																					fTemp546 = F32::max(0.0, F32::min(1.0, fTemp545 / fTemp538));
																																																																																																																																					fTemp547 = 1.0 - fTemp546;
																																																																																																																																					fTemp548 = fSlow343 * fTemp538;
																																																																																																																																					fTemp549 = ((fTemp545 < fTemp538) as i32) as u32 as F32;
																																																																																																																																					self.fVec63[0] = fTemp549 * OscDsp_faustpower2_f(fTemp547) * (0.16666667 * fTemp548 * fTemp547 - fTemp544) + fTemp543 * OscDsp_faustpower2_f(fTemp541) * (0.16666667 * fTemp542 * fTemp541 - fTemp537);
																																																																																																																																					fTemp550 = F32::floor(self.fRec48[0]);
																																																																																																																																					fTemp551 = self.fRec48[0] - fTemp550;
																																																																																																																																					self.fVec64[0] = if iSlow320 != 0 {if iSlow322 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp551 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec48[0] + (0.5 - (fTemp550 + F32::floor(self.fRec48[0] + (0.5 - fTemp550))))) + -1.0}} else {if iSlow321 != 0 {1.0 - 4.0 * F32::abs(self.fRec48[0] + (-0.25 - (fTemp550 + F32::floor(self.fRec48[0] + (0.25 - fTemp550)))))} else {F32::sin(6.2831855 * fTemp551)}};
																																																																																																																																					fTemp552 = (9.0 * self.fRec52[0] + 1.0) * (1.5 * self.fRec50[0] + fTemp543 * OscDsp_faustpower2_f(fTemp540) * (fTemp537 + 0.16666667 * fTemp542 * fTemp540) + self.fVec64[1] + fTemp549 * OscDsp_faustpower2_f(fTemp546) * (fTemp544 + 0.16666667 * fTemp548 * fTemp546) + self.fVec63[1]);
																																																																																																																																					self.fVec65[0] = fTemp552;
																																																																																																																																					fTemp553 = fTemp552 - self.fVec65[1];
																																																																																																																																					iTemp554 = (F32::abs(fTemp553) < 0.0001) as i32;
																																																																																																																																					fTemp555 = if iTemp554 != 0 {F32::sin(0.7853982 * (fTemp552 + self.fVec65[1]))} else {-(0.63661975 * ((F32::cos(1.5707964 * fTemp552) - F32::cos(1.5707964 * self.fVec65[1])) / if iTemp554 != 0 {1.0} else {fTemp553}))};
																																																																																																																																					self.fVec66[0] = fTemp555;
																																																																																																																																					self.fRec65[0] = 0.995 * self.fRec65[1] + fTemp555 - self.fVec66[1];
																																																																																																																																					fTemp556 = fSlow144 * self.fRec65[0];
																																																																																																																																				}
																																																																																																																																					if iSlow59 & (iSlow115 & iSlow148) != 0 {
																																																																																																																																						fTemp557 = 1.0 - OscDsp_faustpower2_f(1.0 - fTemp385);
																																																																																																																																						fTemp558 = 1.0 - 0.5 * OscDsp_faustpower2_f(fTemp385);
																																																																																																																																						iTemp559 = (self.fRec50[0] > 0.5) as i32;
																																																																																																																																						fTemp560 = if iTemp559 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * self.fRec50[0])};
																																																																																																																																						iTemp561 = unsafe { itbl5OscDspSIG5[((F32::max(0.0, F32::min(4.0, 1e+01 * (self.fRec50[0] + -0.5)))) as i32) as usize] };
																																																																																																																																						fTemp562 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp559, iTemp561))) as F32;
																																																																																																																																						fTemp563 = fSlow306 * fTemp20 * F32::exp(0.0005776227 * (fTemp562 + 0.83 * fTemp560));
																																																																																																																																						fTemp564 = fTemp563 + self.fRec66[1];
																																																																																																																																						self.fRec66[0] = if iTemp0 != 0 {fTemp248} else {fTemp564 - F32::floor(fTemp564)};
																																																																																																																																						fTemp565 = fSlow306 * fTemp20 * F32::exp(0.0005776227 * (0.36 * fTemp560 + fTemp562));
																																																																																																																																						fTemp566 = fTemp565 + self.fRec67[1];
																																																																																																																																						self.fRec67[0] = if iTemp0 != 0 {fTemp252} else {fTemp566 - F32::floor(fTemp566)};
																																																																																																																																						fTemp567 = fSlow306 * fTemp20 * F32::exp(0.0005776227 * ((i32::wrapping_mul(0, iTemp561)) as F32 - fTemp560)) + self.fRec68[1];
																																																																																																																																						self.fRec68[0] = if iTemp0 != 0 {fTemp255} else {fTemp567 - F32::floor(fTemp567)};
																																																																																																																																						self.fVec67[0] = fTemp455 * fTemp558 + 0.5 * fTemp557 * (wt_basic(iSlow315, self.fConst0 * F32::abs(fSlow306 * fTemp20 * F32::exp(-0.0005776227 * fTemp560)), self.fConst0, self.fRec68[0]) + wt_basic(iSlow315, self.fConst0 * F32::abs(fTemp565), self.fConst0, self.fRec67[0]) + wt_basic(iSlow315, self.fConst0 * F32::abs(fTemp563), self.fConst0, self.fRec66[0]));
																																																																																																																																						fTemp568 = fSlow147 * (self.fVec67[1] / F32::sqrt(OscDsp_faustpower2_f(fTemp558) + 0.75 * OscDsp_faustpower2_f(fTemp557)));
																																																																																																																																					}
																																																																																																																																						if iSlow59 & iSlow115 != 0 {
																																																																																																																																							fTemp569 = fSlow114 * fSlow349 * (fSlow348 * fTemp568 + fSlow347 * fTemp556 + fSlow346 * fTemp535 + fSlow345 * fTemp503 + fSlow344 * fTemp500 + fSlow330 * fTemp468 + fSlow329 * fTemp456 + fSlow328 * fTemp450 + fSlow327 * fTemp427 + fSlow326 * fTemp408 + fSlow317 * fTemp391 + fSlow314 * fTemp382);
																																																																																																																																						}
																																																																																																																																							if (iSlow59 & (iSlow56 & iSlow184)) | ((iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow160)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151))))))))))))))) != 0 {
																																																																																																																																								self.fVec68[0] = fSlow351;
																																																																																																																																								fTemp570 = if ((fSlow351 != self.fVec68[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec69[1] + -1.0};
																																																																																																																																								self.fRec69[0] = fTemp570;
																																																																																																																																								self.fRec70[0] = if iTemp0 != 0 {fSlow351} else {if (fTemp570 > 0.0) as i32 != 0 {self.fRec70[1] + (fSlow351 - self.fRec70[1]) / F32::max(1.0, fTemp570)} else {fSlow351}};
																																																																																																																																								fTemp571 = 3.0 * F32::max(0.0, F32::min(1.0, self.fRec70[0]));
																																																																																																																																								iTemp572 = std::cmp::min(2, (fTemp571) as i32);
																																																																																																																																								iTemp573 = (iTemp572 >= 2) as i32;
																																																																																																																																								iTemp574 = (iTemp572 >= 1) as i32;
																																																																																																																																								iTemp575 = i32::wrapping_mul(2176, (if iTemp573 != 0 {fSlow354} else {if iTemp574 != 0 {fSlow353} else {fSlow352}}) as i32);
																																																																																																																																							}
																																																																																																																																								if (iSlow59 & (iSlow56 & iSlow184)) | ((iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151)))))))))))))) != 0 {
																																																																																																																																									fTemp576 = fSlow355 * fTemp20;
																																																																																																																																									iTemp577 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp576 > self.fConst16) as i32) + ((fTemp576 > self.fConst22) as i32), (fTemp576 > self.fConst21) as i32), (fTemp576 > self.fConst20) as i32), (fTemp576 > self.fConst19) as i32), (fTemp576 > self.fConst18) as i32), (fTemp576 > self.fConst17) as i32);
																																																																																																																																									iTemp578 = (iTemp577 >= 4) as i32;
																																																																																																																																									iTemp579 = (iTemp577 >= 2) as i32;
																																																																																																																																									iTemp580 = (iTemp577 >= 1) as i32;
																																																																																																																																									iTemp581 = (iTemp577 >= 3) as i32;
																																																																																																																																									iTemp582 = if iTemp578 != 0 {if (iTemp577 >= 6) as i32 != 0 {if (iTemp577 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp577 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp579 != 0 {if iTemp581 != 0 {1792} else {1536}} else {if iTemp580 != 0 {1024} else {0}}};
																																																																																																																																									iTemp583 = if iTemp578 != 0 {64} else {if iTemp579 != 0 {if iTemp581 != 0 {128} else {256}} else {if iTemp580 != 0 {512} else {1024}}};
																																																																																																																																								}
																																																																																																																																									if (iSlow59 & (iSlow56 & iSlow184)) | ((iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow160)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151))))))))))))))) != 0 {
																																																																																																																																										fTemp584 = self.fRec71[1] + fSlow356 * fTemp20;
																																																																																																																																										self.fRec71[0] = if iTemp0 != 0 {fTemp26} else {fTemp584 - F32::floor(fTemp584)};
																																																																																																																																									}
																																																																																																																																										if (iSlow59 & (iSlow56 & iSlow184)) | ((iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151)))))))))))))) != 0 {
																																																																																																																																											fTemp585 = self.fRec71[0] * (iTemp583) as F32;
																																																																																																																																											iTemp586 = (fTemp585) as i32;
																																																																																																																																											iTemp587 = (i32::wrapping_add(iTemp586, 1)) % iTemp583;
																																																																																																																																											fTemp588 = (iTemp586) as F32;
																																																																																																																																											fTemp589 = fTemp585 - fTemp588;
																																																																																																																																											fTemp590 = fTemp588 + (1.0 - fTemp585);
																																																																																																																																										}
																																																																																																																																											if (iSlow59 & (iSlow56 & iSlow184)) | ((iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow160)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151))))))))))))))) != 0 {
																																																																																																																																												fTemp591 = (iTemp572) as F32;
																																																																																																																																												fTemp592 = fTemp571 - fTemp591;
																																																																																																																																												iTemp593 = i32::wrapping_mul(2176, (if iTemp573 != 0 {fSlow353} else {if iTemp574 != 0 {fSlow352} else {fSlow357}}) as i32);
																																																																																																																																												fTemp594 = fTemp591 + (1.0 - fTemp571);
																																																																																																																																											}
																																																																																																																																												if (iSlow59 & (iSlow56 & iSlow184)) | ((iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151)))))))))))))) != 0 {
																																																																																																																																													fTemp595 = fTemp594 * (fTemp590 * wt_sample(i32::wrapping_add(iTemp582, i32::wrapping_add(iTemp586, iTemp593))) + fTemp589 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp593, iTemp582), iTemp587))) + fTemp592 * (fTemp590 * wt_sample(i32::wrapping_add(iTemp582, i32::wrapping_add(iTemp586, iTemp575))) + fTemp589 * wt_sample(i32::wrapping_add(iTemp587, i32::wrapping_add(iTemp582, iTemp575))));
																																																																																																																																												}
																																																																																																																																													if (iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow160)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151)))))))))))))) != 0 {
																																																																																																																																														self.fVec69[0] = fSlow358;
																																																																																																																																														fTemp596 = if ((fSlow358 != self.fVec69[1]) as i32) | iTemp0 != 0 {fSlow47} else {self.fRec72[1] + -1.0};
																																																																																																																																														self.fRec72[0] = fTemp596;
																																																																																																																																														self.fRec73[0] = if iTemp0 != 0 {fSlow358} else {if (fTemp596 > 0.0) as i32 != 0 {self.fRec73[1] + (fSlow358 - self.fRec73[1]) / F32::max(1.0, fTemp596)} else {fSlow358}};
																																																																																																																																														fTemp597 = F32::max(0.0, F32::min(1.0, self.fRec73[0]));
																																																																																																																																													}
																																																																																																																																														if (iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151)))) != 0 {
																																																																																																																																															fTemp598 = (1.0 - fTemp597) * fTemp595;
																																																																																																																																														}
																																																																																																																																															if iSlow59 & (iSlow56 & iSlow151) != 0 {
																																																																																																																																																fTemp599 = fSlow150 * (fTemp598 + fTemp597 * if (fTemp595 > 0.0) as i32 != 0 {1.0 - fTemp595} else {-1.0 - fTemp595});
																																																																																																																																															}
																																																																																																																																																if (iSlow59 & (iSlow56 & iSlow169)) | (iSlow59 & (iSlow56 & iSlow154)) != 0 {
																																																																																																																																																	fTemp600 = fSlow360 * fTemp20;
																																																																																																																																																	iTemp601 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp600 > self.fConst16) as i32) + ((fTemp600 > self.fConst22) as i32), (fTemp600 > self.fConst21) as i32), (fTemp600 > self.fConst20) as i32), (fTemp600 > self.fConst19) as i32), (fTemp600 > self.fConst18) as i32), (fTemp600 > self.fConst17) as i32);
																																																																																																																																																	iTemp602 = (iTemp601 >= 4) as i32;
																																																																																																																																																	iTemp603 = (iTemp601 >= 2) as i32;
																																																																																																																																																	iTemp604 = (iTemp601 >= 1) as i32;
																																																																																																																																																	iTemp605 = (iTemp601 >= 3) as i32;
																																																																																																																																																	iTemp606 = if iTemp602 != 0 {64} else {if iTemp603 != 0 {if iTemp605 != 0 {128} else {256}} else {if iTemp604 != 0 {512} else {1024}}};
																																																																																																																																																	iTemp607 = (self.fRec71[0] < 0.5) as i32;
																																																																																																																																																	fTemp608 = (iTemp606) as F32;
																																																																																																																																																}
																																																																																																																																																	if iSlow59 & (iSlow56 & iSlow154) != 0 {
																																																																																																																																																		fTemp609 = fTemp608 * if iTemp607 != 0 {2.0 * self.fRec71[0]} else {2.0 * (1.0 - self.fRec71[0])};
																																																																																																																																																		iTemp610 = (fTemp609) as i32;
																																																																																																																																																		iTemp611 = (i32::wrapping_add(iTemp610, 1)) % iTemp606;
																																																																																																																																																	}
																																																																																																																																																		if (iSlow59 & (iSlow56 & iSlow169)) | (iSlow59 & (iSlow56 & iSlow154)) != 0 {
																																																																																																																																																			iTemp612 = if iTemp602 != 0 {if (iTemp601 >= 6) as i32 != 0 {if (iTemp601 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp601 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp603 != 0 {if iTemp605 != 0 {1792} else {1536}} else {if iTemp604 != 0 {1024} else {0}}};
																																																																																																																																																			iTemp613 = i32::wrapping_add(iTemp575, iTemp612);
																																																																																																																																																		}
																																																																																																																																																			if iSlow59 & (iSlow56 & iSlow154) != 0 {
																																																																																																																																																				fTemp614 = (iTemp610) as F32;
																																																																																																																																																				fTemp615 = fTemp609 - fTemp614;
																																																																																																																																																				fTemp616 = fTemp614 + (1.0 - fTemp609);
																																																																																																																																																			}
																																																																																																																																																				if (iSlow59 & (iSlow56 & iSlow169)) | (iSlow59 & (iSlow56 & iSlow154)) != 0 {
																																																																																																																																																					iTemp617 = i32::wrapping_add(iTemp593, iTemp612);
																																																																																																																																																				}
																																																																																																																																																					if iSlow59 & (iSlow56 & iSlow154) != 0 {
																																																																																																																																																						fTemp618 = fSlow153 * (fTemp598 + fTemp597 * (fTemp594 * (fTemp616 * wt_sample(i32::wrapping_add(iTemp617, iTemp610)) + fTemp615 * wt_sample(i32::wrapping_add(iTemp617, iTemp611))) + fTemp592 * (fTemp616 * wt_sample(i32::wrapping_add(iTemp613, iTemp610)) + fTemp615 * wt_sample(i32::wrapping_add(iTemp613, iTemp611)))));
																																																																																																																																																					}
																																																																																																																																																						if iSlow59 & (iSlow56 & iSlow157) != 0 {
																																																																																																																																																							fTemp619 = fSlow156 * fTemp595 * (1.0 - fTemp597 * (1.0 - F32::sin(6.2831855 * self.fRec71[0] * (7.0 * fTemp597 + 1.0))));
																																																																																																																																																						}
																																																																																																																																																							if iSlow59 & (iSlow56 & iSlow160) != 0 {
																																																																																																																																																								fTemp620 = 2.0 * fTemp597 + 1.0;
																																																																																																																																																								fTemp621 = fSlow355 * fTemp20 * fTemp620;
																																																																																																																																																								iTemp622 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp621 > self.fConst16) as i32) + ((fTemp621 > self.fConst22) as i32), (fTemp621 > self.fConst21) as i32), (fTemp621 > self.fConst20) as i32), (fTemp621 > self.fConst19) as i32), (fTemp621 > self.fConst18) as i32), (fTemp621 > self.fConst17) as i32);
																																																																																																																																																								iTemp623 = (iTemp622 >= 4) as i32;
																																																																																																																																																								iTemp624 = (iTemp622 >= 2) as i32;
																																																																																																																																																								iTemp625 = (iTemp622 >= 1) as i32;
																																																																																																																																																								iTemp626 = (iTemp622 >= 3) as i32;
																																																																																																																																																								iTemp627 = if iTemp623 != 0 {if (iTemp622 >= 6) as i32 != 0 {if (iTemp622 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp622 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp624 != 0 {if iTemp626 != 0 {1792} else {1536}} else {if iTemp625 != 0 {1024} else {0}}};
																																																																																																																																																								iTemp628 = if iTemp623 != 0 {64} else {if iTemp624 != 0 {if iTemp626 != 0 {128} else {256}} else {if iTemp625 != 0 {512} else {1024}}};
																																																																																																																																																								fTemp629 = F32::powf(self.fRec71[0], fTemp620) * (iTemp628) as F32;
																																																																																																																																																								iTemp630 = (fTemp629) as i32;
																																																																																																																																																								iTemp631 = (i32::wrapping_add(iTemp630, 1)) % iTemp628;
																																																																																																																																																								fTemp632 = (iTemp630) as F32;
																																																																																																																																																								fTemp633 = fTemp629 - fTemp632;
																																																																																																																																																								fTemp634 = fTemp632 + (1.0 - fTemp629);
																																																																																																																																																								fTemp635 = fSlow159 * (fTemp594 * (fTemp634 * wt_sample(i32::wrapping_add(iTemp627, i32::wrapping_add(iTemp593, iTemp630))) + fTemp633 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp593, iTemp627), iTemp631))) + fTemp592 * (fTemp634 * wt_sample(i32::wrapping_add(iTemp627, i32::wrapping_add(iTemp575, iTemp630))) + fTemp633 * wt_sample(i32::wrapping_add(iTemp631, i32::wrapping_add(iTemp575, iTemp627)))));
																																																																																																																																																							}
																																																																																																																																																								if iSlow59 & (iSlow56 & iSlow163) != 0 {
																																																																																																																																																									fTemp636 = fSlow162 * (fTemp598 + fTemp597 * F32::sin(7.8539815 * fTemp595));
																																																																																																																																																								}
																																																																																																																																																									if iSlow59 & (iSlow56 & iSlow166) != 0 {
																																																																																																																																																										fTemp637 = 0.25 * (fTemp595 * (4.0 * fTemp597 + 1.0) + 1.0);
																																																																																																																																																										fTemp638 = fSlow165 * (1.0 - 4.0 * F32::abs(fTemp637 + (-0.5 - F32::floor(fTemp637))));
																																																																																																																																																									}
																																																																																																																																																										if iSlow59 & (iSlow56 & iSlow169) != 0 {
																																																																																																																																																											fTemp639 = 2.0 * self.fRec71[0] * fTemp608;
																																																																																																																																																											iTemp640 = (fTemp639) as i32;
																																																																																																																																																											iTemp641 = (i32::wrapping_add(iTemp640, 1)) % iTemp606;
																																																																																																																																																											fTemp642 = (iTemp640) as F32;
																																																																																																																																																											fTemp643 = fTemp639 - fTemp642;
																																																																																																																																																											fTemp644 = fTemp642 + (1.0 - fTemp639);
																																																																																																																																																											fTemp645 = fSlow168 * (fTemp598 + fTemp597 * (iTemp607) as F32 * F32::abs(fTemp594 * (fTemp644 * wt_sample(i32::wrapping_add(iTemp612, i32::wrapping_add(iTemp593, iTemp640))) + fTemp643 * wt_sample(i32::wrapping_add(iTemp617, iTemp641))) + fTemp592 * (fTemp644 * wt_sample(i32::wrapping_add(iTemp612, i32::wrapping_add(iTemp575, iTemp640))) + fTemp643 * wt_sample(i32::wrapping_add(iTemp641, iTemp613)))));
																																																																																																																																																										}
																																																																																																																																																											if (iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | (iSlow59 & (iSlow56 & iSlow172))) != 0 {
																																																																																																																																																												fTemp646 = F32::tan(self.fConst25 * F32::powf(fSlow368, fTemp597));
																																																																																																																																																												fTemp647 = 1.0 / fTemp646;
																																																																																																																																																												fTemp648 = (fTemp647 + 1.4142135) / fTemp646 + 1.0;
																																																																																																																																																												self.fRec74[0] = fTemp595 - (self.fRec74[2] * ((fTemp647 + -1.4142135) / fTemp646 + 1.0) + 2.0 * self.fRec74[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp646))) / fTemp648;
																																																																																																																																																												fTemp649 = (self.fRec74[2] + self.fRec74[0] + 2.0 * self.fRec74[1]) / fTemp648;
																																																																																																																																																											}
																																																																																																																																																												if iSlow59 & (iSlow56 & iSlow172) != 0 {
																																																																																																																																																													fTemp650 = fSlow171 * fTemp649;
																																																																																																																																																												}
																																																																																																																																																													if iSlow59 & (iSlow56 & iSlow175) != 0 {
																																																																																																																																																														fTemp651 = fSlow174 * fTemp649;
																																																																																																																																																													}
																																																																																																																																																														if iSlow59 & (iSlow56 & iSlow178) != 0 {
																																																																																																																																																															fTemp652 = fSlow177 * fTemp649;
																																																																																																																																																														}
																																																																																																																																																															if (iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | (iSlow59 & (iSlow56 & iSlow52))) != 0 {
																																																																																																																																																																fTemp653 = if iTemp6 != 0 {0.0} else {self.fRec76[1] + self.fConst2 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow373, fTemp597)), 0.0)};
																																																																																																																																																																self.fRec76[0] = fTemp653 - F32::floor(fTemp653);
																																																																																																																																																																self.fRec75[0] = if (self.fRec76[0] < self.fRec76[1]) as i32 != 0 {fTemp595} else {self.fRec75[1]};
																																																																																																																																																															}
																																																																																																																																																																if iSlow59 & (iSlow56 & iSlow52) != 0 {
																																																																																																																																																																	fTemp654 = fSlow51 * self.fRec75[0];
																																																																																																																																																																}
																																																																																																																																																																	if iSlow59 & (iSlow56 & iSlow62) != 0 {
																																																																																																																																																																		fTemp655 = fSlow61 * self.fRec75[0];
																																																																																																																																																																	}
																																																																																																																																																																		if iSlow59 & (iSlow56 & iSlow65) != 0 {
																																																																																																																																																																			fTemp656 = fSlow64 * self.fRec75[0];
																																																																																																																																																																		}
																																																																																																																																																																			if iSlow59 & (iSlow56 & iSlow181) != 0 {
																																																																																																																																																																				fTemp657 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp597)) + -1.0));
																																																																																																																																																																				fTemp658 = fSlow180 * (F32::floor(fTemp595 * fTemp657 + 0.5) / fTemp657);
																																																																																																																																																																			}
																																																																																																																																																																				if iSlow59 & (iSlow56 & iSlow184) != 0 {
																																																																																																																																																																					fTemp659 = fSlow183 * fTemp595;
																																																																																																																																																																				}
																																																																																																																																																																					if iSlow59 & iSlow56 != 0 {
																																																																																																																																																																						fTemp660 = fSlow55 * (fSlow378 * fTemp659 + fSlow377 * fTemp658 + fSlow376 * fTemp656 + fSlow375 * fTemp655 + fSlow374 * fTemp654 + fSlow371 * fTemp652 + fSlow370 * fTemp651 + fSlow369 * fTemp650 + fSlow366 * fTemp645 + fSlow365 * fTemp638 + fSlow364 * fTemp636 + fSlow363 * fTemp635 + fSlow362 * fTemp619 + fSlow361 * fTemp618 + fSlow359 * fTemp599);
																																																																																																																																																																					}
																																																																																																																																																																						if iSlow59 != 0 {
																																																																																																																																																																							fTemp661 = fSlow58 * (fSlow379 * fTemp660 + fSlow350 * fTemp569);
																																																																																																																																																																						}
																																																																																																																																																																							*output1 = fSlow380 * fSlow302 * fTemp661;
																																																																																																																																																																							*output2 = fSlow303 * fTemp349;
																																																																																																																																																																							*output3 = fSlow380 * fTemp661;
																																																																																																																																																																							*output4 = fSlow104 * fTemp20;
																																																																																																																																																																							*output5 = fSlow304 * fTemp20;
																																																																																																																																																																							*output6 = fSlow302;
																																																																																																																																																																							*output7 = fTemp25;
																																																																																																																																																																							*output8 = (iTemp0) as F32;
																																																																																																																																																																							self.fVec0[1] = self.fVec0[0];
																																																																																																																																																																							self.iVec1[1] = self.iVec1[0];
																																																																																																																																																																							self.fVec2[1] = self.fVec2[0];
																																																																																																																																																																							self.fRec1[1] = self.fRec1[0];
																																																																																																																																																																							self.fRec2[1] = self.fRec2[0];
																																																																																																																																																																							self.fVec3[1] = self.fVec3[0];
																																																																																																																																																																							self.fRec3[1] = self.fRec3[0];
																																																																																																																																																																							self.fRec4[1] = self.fRec4[0];
																																																																																																																																																																							if (fTemp5 != 0.0) as i32 != 0 {
																																																																																																																																																																								self.fRec6[1] = self.fRec6[0];
																																																																																																																																																																								self.fRec7[1] = self.fRec7[0];
																																																																																																																																																																								self.iRec9[1] = self.iRec9[0];
																																																																																																																																																																								self.fRec8[1] = self.fRec8[0];
																																																																																																																																																																								self.fRec10[1] = self.fRec10[0];
																																																																																																																																																																								self.fRec13[1] = self.fRec13[0];
																																																																																																																																																																							}
																																																																																																																																																																								self.iRec14[1] = self.iRec14[0];
																																																																																																																																																																								self.fVec5[1] = self.fVec5[0];
																																																																																																																																																																								if (fTemp5 != 0.0) as i32 != 0 {
																																																																																																																																																																									self.fRec12[1] = self.fRec12[0];
																																																																																																																																																																									self.fVec6[1] = self.fVec6[0];
																																																																																																																																																																									self.fRec11[1] = self.fRec11[0];
																																																																																																																																																																								}
																																																																																																																																																																									self.fRec15[1] = self.fRec15[0];
																																																																																																																																																																									self.fVec7[1] = self.fVec7[0];
																																																																																																																																																																									self.fRec16[1] = self.fRec16[0];
																																																																																																																																																																									self.fRec17[1] = self.fRec17[0];
																																																																																																																																																																									self.fRec18[1] = self.fRec18[0];
																																																																																																																																																																									self.fRec19[1] = self.fRec19[0];
																																																																																																																																																																									if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))) != 0 {
																																																																																																																																																																										self.fRec0[1] = self.fRec0[0];
																																																																																																																																																																									}
																																																																																																																																																																										if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow190)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow187)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
																																																																																																																																																																											self.fVec8[1] = self.fVec8[0];
																																																																																																																																																																											self.fRec20[1] = self.fRec20[0];
																																																																																																																																																																											self.fRec21[1] = self.fRec21[0];
																																																																																																																																																																											self.fVec9[1] = self.fVec9[0];
																																																																																																																																																																											self.fRec22[1] = self.fRec22[0];
																																																																																																																																																																											self.fRec23[1] = self.fRec23[0];
																																																																																																																																																																										}
																																																																																																																																																																											if iSlow2 & (iSlow6 & iSlow10) != 0 {
																																																																																																																																																																												self.fVec10[1] = self.fVec10[0];
																																																																																																																																																																												self.fVec11[1] = self.fVec11[0];
																																																																																																																																																																											}
																																																																																																																																																																												if iSlow2 & (iSlow6 & iSlow13) != 0 {
																																																																																																																																																																													self.fRec24[1] = self.fRec24[0];
																																																																																																																																																																													self.fVec12[1] = self.fVec12[0];
																																																																																																																																																																												}
																																																																																																																																																																													if (iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																																																																																																																																																																														self.iRec25[1] = self.iRec25[0];
																																																																																																																																																																													}
																																																																																																																																																																														if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																																																																																																																																																															self.fVec13[1] = self.fVec13[0];
																																																																																																																																																																															self.fVec14[1] = self.fVec14[0];
																																																																																																																																																																														}
																																																																																																																																																																															if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																																																																																																																																																																																self.fVec15[1] = self.fVec15[0];
																																																																																																																																																																															}
																																																																																																																																																																																if iSlow2 & (iSlow6 & iSlow19) != 0 {
																																																																																																																																																																																	self.fVec16[1] = self.fVec16[0];
																																																																																																																																																																																	self.fVec17[1] = self.fVec17[0];
																																																																																																																																																																																	self.fVec18[1] = self.fVec18[0];
																																																																																																																																																																																}
																																																																																																																																																																																	if iSlow2 & (iSlow6 & iSlow22) != 0 {
																																																																																																																																																																																		self.fVec19[1] = self.fVec19[0];
																																																																																																																																																																																		self.fVec20[1] = self.fVec20[0];
																																																																																																																																																																																	}
																																																																																																																																																																																		if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																																																																																																																																																																			self.fRec26[1] = self.fRec26[0];
																																																																																																																																																																																			self.fVec21[1] = self.fVec21[0];
																																																																																																																																																																																			self.fRec27[1] = self.fRec27[0];
																																																																																																																																																																																			self.fVec22[1] = self.fVec22[0];
																																																																																																																																																																																		}
																																																																																																																																																																																			if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | (iSlow2 & (iSlow6 & iSlow25)))) != 0 {
																																																																																																																																																																																				self.fVec23[1] = self.fVec23[0];
																																																																																																																																																																																			}
																																																																																																																																																																																				if iSlow2 & (iSlow6 & iSlow28) != 0 {
																																																																																																																																																																																					self.fRec28[1] = self.fRec28[0];
																																																																																																																																																																																					self.fVec24[1] = self.fVec24[0];
																																																																																																																																																																																					self.fRec29[1] = self.fRec29[0];
																																																																																																																																																																																					self.fVec25[1] = self.fVec25[0];
																																																																																																																																																																																				}
																																																																																																																																																																																					if iSlow2 & (iSlow6 & iSlow187) != 0 {
																																																																																																																																																																																						self.fRec30[1] = self.fRec30[0];
																																																																																																																																																																																						self.fVec26[1] = self.fVec26[0];
																																																																																																																																																																																						self.fRec31[1] = self.fRec31[0];
																																																																																																																																																																																						self.fVec27[1] = self.fVec27[0];
																																																																																																																																																																																						self.fVec28[1] = self.fVec28[0];
																																																																																																																																																																																					}
																																																																																																																																																																																						if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																																																																																																																																																							self.fRec32[1] = self.fRec32[0];
																																																																																																																																																																																						}
																																																																																																																																																																																							if iSlow2 & (iSlow6 & iSlow190) != 0 {
																																																																																																																																																																																								self.fRec33[1] = self.fRec33[0];
																																																																																																																																																																																								self.fRec34[1] = self.fRec34[0];
																																																																																																																																																																																								self.fVec29[1] = self.fVec29[0];
																																																																																																																																																																																								self.fRec35[1] = self.fRec35[0];
																																																																																																																																																																																								self.fVec30[1] = self.fVec30[0];
																																																																																																																																																																																								self.fVec31[1] = self.fVec31[0];
																																																																																																																																																																																							}
																																																																																																																																																																																								if iSlow2 & (iSlow6 & iSlow34) != 0 {
																																																																																																																																																																																									self.fVec32[1] = self.fVec32[0];
																																																																																																																																																																																									self.fVec33[1] = self.fVec33[0];
																																																																																																																																																																																									self.fVec34[1] = self.fVec34[0];
																																																																																																																																																																																									self.fVec35[1] = self.fVec35[0];
																																																																																																																																																																																									self.fRec36[1] = self.fRec36[0];
																																																																																																																																																																																								}
																																																																																																																																																																																									if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																																																																																																																																																										self.fRec37[1] = self.fRec37[0];
																																																																																																																																																																																										self.fRec38[1] = self.fRec38[0];
																																																																																																																																																																																										self.fRec39[1] = self.fRec39[0];
																																																																																																																																																																																										self.fVec36[1] = self.fVec36[0];
																																																																																																																																																																																									}
																																																																																																																																																																																										if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow211)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow199)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193))))))))))))))) != 0 {
																																																																																																																																																																																											self.fVec37[1] = self.fVec37[0];
																																																																																																																																																																																											self.fRec40[1] = self.fRec40[0];
																																																																																																																																																																																											self.fRec41[1] = self.fRec41[0];
																																																																																																																																																																																											self.fRec42[1] = self.fRec42[0];
																																																																																																																																																																																										}
																																																																																																																																																																																											if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow226)) | ((iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow75)) | ((iSlow2 & (iSlow72 & iSlow69)) | ((iSlow2 & (iSlow72 & iSlow220)) | ((iSlow2 & (iSlow72 & iSlow217)) | ((iSlow2 & (iSlow72 & iSlow214)) | ((iSlow2 & (iSlow72 & iSlow211)) | ((iSlow2 & (iSlow72 & iSlow208)) | ((iSlow2 & (iSlow72 & iSlow205)) | ((iSlow2 & (iSlow72 & iSlow202)) | ((iSlow2 & (iSlow72 & iSlow196)) | (iSlow2 & (iSlow72 & iSlow193)))))))))))))) != 0 {
																																																																																																																																																																																												self.fVec38[1] = self.fVec38[0];
																																																																																																																																																																																												self.fRec43[1] = self.fRec43[0];
																																																																																																																																																																																												self.fRec44[1] = self.fRec44[0];
																																																																																																																																																																																											}
																																																																																																																																																																																												if (iSlow2 & (iSlow72 & iSlow223)) | ((iSlow2 & (iSlow72 & iSlow220)) | (iSlow2 & (iSlow72 & iSlow205))) != 0 {
																																																																																																																																																																																													self.fRec45[2] = self.fRec45[1];
																																																																																																																																																																																													self.fRec45[1] = self.fRec45[0];
																																																																																																																																																																																												}
																																																																																																																																																																																													if (iSlow2 & (iSlow72 & iSlow78)) | ((iSlow2 & (iSlow72 & iSlow75)) | (iSlow2 & (iSlow72 & iSlow69))) != 0 {
																																																																																																																																																																																														self.fRec47[1] = self.fRec47[0];
																																																																																																																																																																																														self.fRec46[1] = self.fRec46[0];
																																																																																																																																																																																													}
																																																																																																																																																																																														if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow145)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | ((iSlow59 & (iSlow115 & iSlow121)) | ((iSlow59 & (iSlow115 & iSlow118)) | (iSlow59 & (iSlow115 & iSlow112)))))))))) != 0 {
																																																																																																																																																																																															self.fRec48[1] = self.fRec48[0];
																																																																																																																																																																																														}
																																																																																																																																																																																															if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow145)) | ((iSlow59 & (iSlow115 & iSlow142)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow136)) | ((iSlow59 & (iSlow115 & iSlow133)) | ((iSlow59 & (iSlow115 & iSlow130)) | ((iSlow59 & (iSlow115 & iSlow127)) | ((iSlow59 & (iSlow115 & iSlow124)) | ((iSlow59 & (iSlow115 & iSlow121)) | ((iSlow59 & (iSlow115 & iSlow118)) | (iSlow59 & (iSlow115 & iSlow112)))))))))))) != 0 {
																																																																																																																																																																																																self.fVec39[1] = self.fVec39[0];
																																																																																																																																																																																																self.fRec49[1] = self.fRec49[0];
																																																																																																																																																																																																self.fRec50[1] = self.fRec50[0];
																																																																																																																																																																																																self.fVec40[1] = self.fVec40[0];
																																																																																																																																																																																																self.fRec51[1] = self.fRec51[0];
																																																																																																																																																																																																self.fRec52[1] = self.fRec52[0];
																																																																																																																																																																																															}
																																																																																																																																																																																																if iSlow59 & (iSlow115 & iSlow112) != 0 {
																																																																																																																																																																																																	self.fVec41[1] = self.fVec41[0];
																																																																																																																																																																																																	self.fVec42[1] = self.fVec42[0];
																																																																																																																																																																																																}
																																																																																																																																																																																																	if iSlow59 & (iSlow115 & iSlow118) != 0 {
																																																																																																																																																																																																		self.fRec53[1] = self.fRec53[0];
																																																																																																																																																																																																		self.fVec43[1] = self.fVec43[0];
																																																																																																																																																																																																	}
																																																																																																																																																																																																		if (iSlow59 & (iSlow115 & iSlow124)) | (iSlow59 & (iSlow115 & iSlow121)) != 0 {
																																																																																																																																																																																																			self.iRec54[1] = self.iRec54[0];
																																																																																																																																																																																																		}
																																																																																																																																																																																																			if iSlow59 & (iSlow115 & iSlow121) != 0 {
																																																																																																																																																																																																				self.fVec44[1] = self.fVec44[0];
																																																																																																																																																																																																				self.fVec45[1] = self.fVec45[0];
																																																																																																																																																																																																			}
																																																																																																																																																																																																				if (iSlow59 & (iSlow115 & iSlow127)) | (iSlow59 & (iSlow115 & iSlow121)) != 0 {
																																																																																																																																																																																																					self.fVec46[1] = self.fVec46[0];
																																																																																																																																																																																																				}
																																																																																																																																																																																																					if iSlow59 & (iSlow115 & iSlow124) != 0 {
																																																																																																																																																																																																						self.fVec47[1] = self.fVec47[0];
																																																																																																																																																																																																						self.fVec48[1] = self.fVec48[0];
																																																																																																																																																																																																						self.fVec49[1] = self.fVec49[0];
																																																																																																																																																																																																					}
																																																																																																																																																																																																						if iSlow59 & (iSlow115 & iSlow127) != 0 {
																																																																																																																																																																																																							self.fVec50[1] = self.fVec50[0];
																																																																																																																																																																																																							self.fVec51[1] = self.fVec51[0];
																																																																																																																																																																																																						}
																																																																																																																																																																																																							if iSlow59 & (iSlow115 & iSlow130) != 0 {
																																																																																																																																																																																																								self.fRec55[1] = self.fRec55[0];
																																																																																																																																																																																																								self.fVec52[1] = self.fVec52[0];
																																																																																																																																																																																																								self.fRec56[1] = self.fRec56[0];
																																																																																																																																																																																																								self.fVec53[1] = self.fVec53[0];
																																																																																																																																																																																																							}
																																																																																																																																																																																																								if (iSlow59 & (iSlow115 & iSlow148)) | ((iSlow59 & (iSlow115 & iSlow139)) | ((iSlow59 & (iSlow115 & iSlow133)) | (iSlow59 & (iSlow115 & iSlow130)))) != 0 {
																																																																																																																																																																																																									self.fVec54[1] = self.fVec54[0];
																																																																																																																																																																																																								}
																																																																																																																																																																																																									if iSlow59 & (iSlow115 & iSlow133) != 0 {
																																																																																																																																																																																																										self.fRec57[1] = self.fRec57[0];
																																																																																																																																																																																																										self.fVec55[1] = self.fVec55[0];
																																																																																																																																																																																																										self.fRec58[1] = self.fRec58[0];
																																																																																																																																																																																																										self.fVec56[1] = self.fVec56[0];
																																																																																																																																																																																																									}
																																																																																																																																																																																																										if iSlow59 & (iSlow115 & iSlow136) != 0 {
																																																																																																																																																																																																											self.fRec59[1] = self.fRec59[0];
																																																																																																																																																																																																											self.fVec57[1] = self.fVec57[0];
																																																																																																																																																																																																											self.fRec60[1] = self.fRec60[0];
																																																																																																																																																																																																											self.fVec58[1] = self.fVec58[0];
																																																																																																																																																																																																											self.fVec59[1] = self.fVec59[0];
																																																																																																																																																																																																										}
																																																																																																																																																																																																											if iSlow59 & (iSlow115 & iSlow139) != 0 {
																																																																																																																																																																																																												self.fRec61[1] = self.fRec61[0];
																																																																																																																																																																																																											}
																																																																																																																																																																																																												if iSlow59 & (iSlow115 & iSlow142) != 0 {
																																																																																																																																																																																																													self.fRec62[1] = self.fRec62[0];
																																																																																																																																																																																																													self.fRec63[1] = self.fRec63[0];
																																																																																																																																																																																																													self.fVec60[1] = self.fVec60[0];
																																																																																																																																																																																																													self.fRec64[1] = self.fRec64[0];
																																																																																																																																																																																																													self.fVec61[1] = self.fVec61[0];
																																																																																																																																																																																																													self.fVec62[1] = self.fVec62[0];
																																																																																																																																																																																																												}
																																																																																																																																																																																																													if iSlow59 & (iSlow115 & iSlow145) != 0 {
																																																																																																																																																																																																														self.fVec63[1] = self.fVec63[0];
																																																																																																																																																																																																														self.fVec64[1] = self.fVec64[0];
																																																																																																																																																																																																														self.fVec65[1] = self.fVec65[0];
																																																																																																																																																																																																														self.fVec66[1] = self.fVec66[0];
																																																																																																																																																																																																														self.fRec65[1] = self.fRec65[0];
																																																																																																																																																																																																													}
																																																																																																																																																																																																														if iSlow59 & (iSlow115 & iSlow148) != 0 {
																																																																																																																																																																																																															self.fRec66[1] = self.fRec66[0];
																																																																																																																																																																																																															self.fRec67[1] = self.fRec67[0];
																																																																																																																																																																																																															self.fRec68[1] = self.fRec68[0];
																																																																																																																																																																																																															self.fVec67[1] = self.fVec67[0];
																																																																																																																																																																																																														}
																																																																																																																																																																																																															if (iSlow59 & (iSlow56 & iSlow184)) | ((iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow160)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151))))))))))))))) != 0 {
																																																																																																																																																																																																																self.fVec68[1] = self.fVec68[0];
																																																																																																																																																																																																																self.fRec69[1] = self.fRec69[0];
																																																																																																																																																																																																																self.fRec70[1] = self.fRec70[0];
																																																																																																																																																																																																																self.fRec71[1] = self.fRec71[0];
																																																																																																																																																																																																															}
																																																																																																																																																																																																																if (iSlow59 & (iSlow56 & iSlow181)) | ((iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | ((iSlow59 & (iSlow56 & iSlow52)) | ((iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | ((iSlow59 & (iSlow56 & iSlow172)) | ((iSlow59 & (iSlow56 & iSlow169)) | ((iSlow59 & (iSlow56 & iSlow166)) | ((iSlow59 & (iSlow56 & iSlow163)) | ((iSlow59 & (iSlow56 & iSlow160)) | ((iSlow59 & (iSlow56 & iSlow157)) | ((iSlow59 & (iSlow56 & iSlow154)) | (iSlow59 & (iSlow56 & iSlow151)))))))))))))) != 0 {
																																																																																																																																																																																																																	self.fVec69[1] = self.fVec69[0];
																																																																																																																																																																																																																	self.fRec72[1] = self.fRec72[0];
																																																																																																																																																																																																																	self.fRec73[1] = self.fRec73[0];
																																																																																																																																																																																																																}
																																																																																																																																																																																																																	if (iSlow59 & (iSlow56 & iSlow178)) | ((iSlow59 & (iSlow56 & iSlow175)) | (iSlow59 & (iSlow56 & iSlow172))) != 0 {
																																																																																																																																																																																																																		self.fRec74[2] = self.fRec74[1];
																																																																																																																																																																																																																		self.fRec74[1] = self.fRec74[0];
																																																																																																																																																																																																																	}
																																																																																																																																																																																																																		if (iSlow59 & (iSlow56 & iSlow65)) | ((iSlow59 & (iSlow56 & iSlow62)) | (iSlow59 & (iSlow56 & iSlow52))) != 0 {
																																																																																																																																																																																																																			self.fRec76[1] = self.fRec76[0];
																																																																																																																																																																																																																			self.fRec75[1] = self.fRec75[0];
																																																																																																																																																																																																																		}
																																																																																																																																																																																																																		}
																																																																																																																																																																																																																		self.iTempPerm0 = iTemp6;
																																																																																																																																																																																																																		self.fTempPerm1 = fTemp7;
																																																																																																																																																																																																																		self.fTempPerm2 = fTemp8;
																																																																																																																																																																																																																		self.iTempPerm3 = iTemp9;
																																																																																																																																																																																																																		self.fTempPerm4 = fTemp12;
																																																																																																																																																																																																																		self.fTempPerm5 = fTemp15;
																																																																																																																																																																																																																		self.fTempPerm6 = fTemp16;
																																																																																																																																																																																																																		self.fTempPerm7 = fTemp21;
																																																																																																																																																																																																																		self.fTempPerm8 = fTemp22;
																																																																																																																																																																																																																		self.fTempPerm9 = fTemp26;
																																																																																																																																																																																																																		self.iTempPerm10 = iTemp27;
																																																																																																																																																																																																																		self.fTempPerm11 = fTemp28;
																																																																																																																																																																																																																		self.fTempPerm12 = fTemp29;
																																																																																																																																																																																																																		self.fTempPerm13 = fTemp30;
																																																																																																																																																																																																																		self.fTempPerm14 = fTemp31;
																																																																																																																																																																																																																		self.fTempPerm15 = fTemp32;
																																																																																																																																																																																																																		self.fTempPerm16 = fTemp33;
																																																																																																																																																																																																																		self.fTempPerm17 = fTemp34;
																																																																																																																																																																																																																		self.fTempPerm18 = fTemp35;
																																																																																																																																																																																																																		self.fTempPerm19 = fTemp36;
																																																																																																																																																																																																																		self.fTempPerm20 = fTemp37;
																																																																																																																																																																																																																		self.fTempPerm21 = fTemp38;
																																																																																																																																																																																																																		self.fTempPerm22 = fTemp39;
																																																																																																																																																																																																																		self.fTempPerm23 = fTemp40;
																																																																																																																																																																																																																		self.iTempPerm24 = iTemp41;
																																																																																																																																																																																																																		self.fTempPerm25 = fTemp42;
																																																																																																																																																																																																																		self.fTempPerm26 = fTemp43;
																																																																																																																																																																																																																		self.fTempPerm27 = fTemp44;
																																																																																																																																																																																																																		self.fTempPerm28 = fTemp45;
																																																																																																																																																																																																																		self.fTempPerm29 = fTemp46;
																																																																																																																																																																																																																		self.fTempPerm30 = fTemp47;
																																																																																																																																																																																																																		self.fTempPerm31 = fTemp48;
																																																																																																																																																																																																																		self.fTempPerm32 = fTemp49;
																																																																																																																																																																																																																		self.fTempPerm33 = fTemp50;
																																																																																																																																																																																																																		self.fTempPerm34 = fTemp51;
																																																																																																																																																																																																																		self.fTempPerm35 = fTemp52;
																																																																																																																																																																																																																		self.fTempPerm36 = fTemp53;
																																																																																																																																																																																																																		self.fTempPerm37 = fTemp54;
																																																																																																																																																																																																																		self.fTempPerm38 = fTemp55;
																																																																																																																																																																																																																		self.fTempPerm39 = fTemp56;
																																																																																																																																																																																																																		self.fTempPerm40 = fTemp57;
																																																																																																																																																																																																																		self.fTempPerm41 = fTemp58;
																																																																																																																																																																																																																		self.fTempPerm42 = fTemp59;
																																																																																																																																																																																																																		self.fTempPerm43 = fTemp60;
																																																																																																																																																																																																																		self.fTempPerm44 = fTemp61;
																																																																																																																																																																																																																		self.fTempPerm45 = fTemp62;
																																																																																																																																																																																																																		self.fTempPerm46 = fTemp63;
																																																																																																																																																																																																																		self.fTempPerm47 = fTemp64;
																																																																																																																																																																																																																		self.fTempPerm48 = fTemp65;
																																																																																																																																																																																																																		self.fTempPerm49 = fTemp66;
																																																																																																																																																																																																																		self.fTempPerm50 = fTemp67;
																																																																																																																																																																																																																		self.fTempPerm51 = fTemp68;
																																																																																																																																																																																																																		self.fTempPerm52 = fTemp69;
																																																																																																																																																																																																																		self.iTempPerm53 = iTemp70;
																																																																																																																																																																																																																		self.fTempPerm54 = fTemp71;
																																																																																																																																																																																																																		self.fTempPerm55 = fTemp72;
																																																																																																																																																																																																																		self.fTempPerm56 = fTemp73;
																																																																																																																																																																																																																		self.fTempPerm57 = fTemp74;
																																																																																																																																																																																																																		self.fTempPerm58 = fTemp75;
																																																																																																																																																																																																																		self.fTempPerm59 = fTemp76;
																																																																																																																																																																																																																		self.fTempPerm60 = fTemp77;
																																																																																																																																																																																																																		self.fTempPerm61 = fTemp78;
																																																																																																																																																																																																																		self.fTempPerm62 = fTemp79;
																																																																																																																																																																																																																		self.fTempPerm63 = fTemp80;
																																																																																																																																																																																																																		self.fTempPerm64 = fTemp81;
																																																																																																																																																																																																																		self.fTempPerm65 = fTemp82;
																																																																																																																																																																																																																		self.fTempPerm66 = fTemp83;
																																																																																																																																																																																																																		self.fTempPerm67 = fTemp84;
																																																																																																																																																																																																																		self.fTempPerm68 = fTemp85;
																																																																																																																																																																																																																		self.fTempPerm69 = fTemp86;
																																																																																																																																																																																																																		self.fTempPerm70 = fTemp87;
																																																																																																																																																																																																																		self.fTempPerm71 = fTemp88;
																																																																																																																																																																																																																		self.fTempPerm72 = fTemp89;
																																																																																																																																																																																																																		self.fTempPerm73 = fTemp90;
																																																																																																																																																																																																																		self.fTempPerm74 = fTemp91;
																																																																																																																																																																																																																		self.iTempPerm75 = iTemp92;
																																																																																																																																																																																																																		self.fTempPerm76 = fTemp93;
																																																																																																																																																																																																																		self.fTempPerm77 = fTemp94;
																																																																																																																																																																																																																		self.fTempPerm78 = fTemp95;
																																																																																																																																																																																																																		self.fTempPerm79 = fTemp96;
																																																																																																																																																																																																																		self.fTempPerm80 = fTemp97;
																																																																																																																																																																																																																		self.iTempPerm81 = iTemp98;
																																																																																																																																																																																																																		self.fTempPerm82 = fTemp99;
																																																																																																																																																																																																																		self.fTempPerm83 = fTemp100;
																																																																																																																																																																																																																		self.fTempPerm84 = fTemp101;
																																																																																																																																																																																																																		self.fTempPerm85 = fTemp102;
																																																																																																																																																																																																																		self.fTempPerm86 = fTemp103;
																																																																																																																																																																																																																		self.fTempPerm87 = fTemp104;
																																																																																																																																																																																																																		self.fTempPerm88 = fTemp105;
																																																																																																																																																																																																																		self.iTempPerm89 = iTemp106;
																																																																																																																																																																																																																		self.iTempPerm90 = iTemp107;
																																																																																																																																																																																																																		self.iTempPerm91 = iTemp108;
																																																																																																																																																																																																																		self.iTempPerm92 = iTemp109;
																																																																																																																																																																																																																		self.iTempPerm93 = iTemp110;
																																																																																																																																																																																																																		self.iTempPerm94 = iTemp111;
																																																																																																																																																																																																																		self.fTempPerm95 = fTemp112;
																																																																																																																																																																																																																		self.fTempPerm96 = fTemp113;
																																																																																																																																																																																																																		self.fTempPerm97 = fTemp114;
																																																																																																																																																																																																																		self.iTempPerm98 = iTemp115;
																																																																																																																																																																																																																		self.iTempPerm99 = iTemp116;
																																																																																																																																																																																																																		self.iTempPerm100 = iTemp117;
																																																																																																																																																																																																																		self.iTempPerm101 = iTemp118;
																																																																																																																																																																																																																		self.fTempPerm102 = fTemp119;
																																																																																																																																																																																																																		self.fTempPerm103 = fTemp120;
																																																																																																																																																																																																																		self.fTempPerm104 = fTemp121;
																																																																																																																																																																																																																		self.fTempPerm105 = fTemp122;
																																																																																																																																																																																																																		self.fTempPerm106 = fTemp123;
																																																																																																																																																																																																																		self.fTempPerm107 = fTemp124;
																																																																																																																																																																																																																		self.fTempPerm108 = fTemp125;
																																																																																																																																																																																																																		self.fTempPerm109 = fTemp126;
																																																																																																																																																																																																																		self.fTempPerm110 = fTemp127;
																																																																																																																																																																																																																		self.fTempPerm111 = fTemp128;
																																																																																																																																																																																																																		self.fTempPerm112 = fTemp129;
																																																																																																																																																																																																																		self.fTempPerm113 = fTemp130;
																																																																																																																																																																																																																		self.fTempPerm114 = fTemp131;
																																																																																																																																																																																																																		self.fTempPerm115 = fTemp132;
																																																																																																																																																																																																																		self.fTempPerm116 = fTemp133;
																																																																																																																																																																																																																		self.fTempPerm117 = fTemp134;
																																																																																																																																																																																																																		self.fTempPerm118 = fTemp135;
																																																																																																																																																																																																																		self.fTempPerm119 = fTemp136;
																																																																																																																																																																																																																		self.fTempPerm120 = fTemp137;
																																																																																																																																																																																																																		self.iTempPerm121 = iTemp138;
																																																																																																																																																																																																																		self.fTempPerm122 = fTemp139;
																																																																																																																																																																																																																		self.fTempPerm123 = fTemp140;
																																																																																																																																																																																																																		self.fTempPerm124 = fTemp141;
																																																																																																																																																																																																																		self.fTempPerm125 = fTemp142;
																																																																																																																																																																																																																		self.fTempPerm126 = fTemp143;
																																																																																																																																																																																																																		self.fTempPerm127 = fTemp144;
																																																																																																																																																																																																																		self.fTempPerm128 = fTemp145;
																																																																																																																																																																																																																		self.fTempPerm129 = fTemp146;
																																																																																																																																																																																																																		self.fTempPerm130 = fTemp147;
																																																																																																																																																																																																																		self.fTempPerm131 = fTemp148;
																																																																																																																																																																																																																		self.fTempPerm132 = fTemp149;
																																																																																																																																																																																																																		self.fTempPerm133 = fTemp150;
																																																																																																																																																																																																																		self.fTempPerm134 = fTemp151;
																																																																																																																																																																																																																		self.fTempPerm135 = fTemp152;
																																																																																																																																																																																																																		self.iTempPerm136 = iTemp153;
																																																																																																																																																																																																																		self.fTempPerm137 = fTemp154;
																																																																																																																																																																																																																		self.fTempPerm138 = fTemp155;
																																																																																																																																																																																																																		self.fTempPerm139 = fTemp156;
																																																																																																																																																																																																																		self.fTempPerm140 = fTemp157;
																																																																																																																																																																																																																		self.iTempPerm141 = iTemp158;
																																																																																																																																																																																																																		self.fTempPerm142 = fTemp159;
																																																																																																																																																																																																																		self.iTempPerm143 = iTemp160;
																																																																																																																																																																																																																		self.fTempPerm144 = fTemp161;
																																																																																																																																																																																																																		self.fTempPerm145 = fTemp162;
																																																																																																																																																																																																																		self.fTempPerm146 = fTemp163;
																																																																																																																																																																																																																		self.fTempPerm147 = fTemp164;
																																																																																																																																																																																																																		self.fTempPerm148 = fTemp165;
																																																																																																																																																																																																																		self.iTempPerm149 = iTemp166;
																																																																																																																																																																																																																		self.fTempPerm150 = fTemp167;
																																																																																																																																																																																																																		self.fTempPerm151 = fTemp168;
																																																																																																																																																																																																																		self.fTempPerm152 = fTemp169;
																																																																																																																																																																																																																		self.fTempPerm153 = fTemp170;
																																																																																																																																																																																																																		self.fTempPerm154 = fTemp171;
																																																																																																																																																																																																																		self.fTempPerm155 = fTemp172;
																																																																																																																																																																																																																		self.fTempPerm156 = fTemp173;
																																																																																																																																																																																																																		self.fTempPerm157 = fTemp174;
																																																																																																																																																																																																																		self.fTempPerm158 = fTemp175;
																																																																																																																																																																																																																		self.fTempPerm159 = fTemp176;
																																																																																																																																																																																																																		self.fTempPerm160 = fTemp177;
																																																																																																																																																																																																																		self.fTempPerm161 = fTemp178;
																																																																																																																																																																																																																		self.fTempPerm162 = fTemp179;
																																																																																																																																																																																																																		self.fTempPerm163 = fTemp180;
																																																																																																																																																																																																																		self.fTempPerm164 = fTemp181;
																																																																																																																																																																																																																		self.fTempPerm165 = fTemp182;
																																																																																																																																																																																																																		self.fTempPerm166 = fTemp183;
																																																																																																																																																																																																																		self.fTempPerm167 = fTemp184;
																																																																																																																																																																																																																		self.fTempPerm168 = fTemp185;
																																																																																																																																																																																																																		self.fTempPerm169 = fTemp186;
																																																																																																																																																																																																																		self.fTempPerm170 = fTemp187;
																																																																																																																																																																																																																		self.fTempPerm171 = fTemp188;
																																																																																																																																																																																																																		self.iTempPerm172 = iTemp189;
																																																																																																																																																																																																																		self.fTempPerm173 = fTemp190;
																																																																																																																																																																																																																		self.iTempPerm174 = iTemp191;
																																																																																																																																																																																																																		self.fTempPerm175 = fTemp192;
																																																																																																																																																																																																																		self.iTempPerm176 = iTemp193;
																																																																																																																																																																																																																		self.fTempPerm177 = fTemp194;
																																																																																																																																																																																																																		self.iTempPerm178 = iTemp195;
																																																																																																																																																																																																																		self.fTempPerm179 = fTemp196;
																																																																																																																																																																																																																		self.fTempPerm180 = fTemp197;
																																																																																																																																																																																																																		self.fTempPerm181 = fTemp198;
																																																																																																																																																																																																																		self.fTempPerm182 = fTemp199;
																																																																																																																																																																																																																		self.iTempPerm183 = iTemp200;
																																																																																																																																																																																																																		self.fTempPerm184 = fTemp201;
																																																																																																																																																																																																																		self.fTempPerm185 = fTemp202;
																																																																																																																																																																																																																		self.fTempPerm186 = fTemp203;
																																																																																																																																																																																																																		self.fTempPerm187 = fTemp204;
																																																																																																																																																																																																																		self.fTempPerm188 = fTemp205;
																																																																																																																																																																																																																		self.fTempPerm189 = fTemp206;
																																																																																																																																																																																																																		self.fTempPerm190 = fTemp207;
																																																																																																																																																																																																																		self.fTempPerm191 = fTemp208;
																																																																																																																																																																																																																		self.fTempPerm192 = fTemp209;
																																																																																																																																																																																																																		self.fTempPerm193 = fTemp210;
																																																																																																																																																																																																																		self.fTempPerm194 = fTemp211;
																																																																																																																																																																																																																		self.fTempPerm195 = fTemp212;
																																																																																																																																																																																																																		self.fTempPerm196 = fTemp213;
																																																																																																																																																																																																																		self.fTempPerm197 = fTemp214;
																																																																																																																																																																																																																		self.fTempPerm198 = fTemp215;
																																																																																																																																																																																																																		self.fTempPerm199 = fTemp216;
																																																																																																																																																																																																																		self.fTempPerm200 = fTemp217;
																																																																																																																																																																																																																		self.iTempPerm201 = iTemp218;
																																																																																																																																																																																																																		self.fTempPerm202 = fTemp219;
																																																																																																																																																																																																																		self.fTempPerm203 = fTemp220;
																																																																																																																																																																																																																		self.fTempPerm204 = fTemp221;
																																																																																																																																																																																																																		self.fTempPerm205 = fTemp222;
																																																																																																																																																																																																																		self.fTempPerm206 = fTemp223;
																																																																																																																																																																																																																		self.fTempPerm207 = fTemp224;
																																																																																																																																																																																																																		self.fTempPerm208 = fTemp225;
																																																																																																																																																																																																																		self.fTempPerm209 = fTemp226;
																																																																																																																																																																																																																		self.fTempPerm210 = fTemp227;
																																																																																																																																																																																																																		self.fTempPerm211 = fTemp228;
																																																																																																																																																																																																																		self.fTempPerm212 = fTemp229;
																																																																																																																																																																																																																		self.fTempPerm213 = fTemp230;
																																																																																																																																																																																																																		self.fTempPerm214 = fTemp231;
																																																																																																																																																																																																																		self.fTempPerm215 = fTemp232;
																																																																																																																																																																																																																		self.fTempPerm216 = fTemp233;
																																																																																																																																																																																																																		self.fTempPerm217 = fTemp234;
																																																																																																																																																																																																																		self.fTempPerm218 = fTemp235;
																																																																																																																																																																																																																		self.iTempPerm219 = iTemp236;
																																																																																																																																																																																																																		self.fTempPerm220 = fTemp237;
																																																																																																																																																																																																																		self.fTempPerm221 = fTemp238;
																																																																																																																																																																																																																		self.fTempPerm222 = fTemp239;
																																																																																																																																																																																																																		self.fTempPerm223 = fTemp240;
																																																																																																																																																																																																																		self.iTempPerm224 = iTemp241;
																																																																																																																																																																																																																		self.fTempPerm225 = fTemp242;
																																																																																																																																																																																																																		self.iTempPerm226 = iTemp243;
																																																																																																																																																																																																																		self.fTempPerm227 = fTemp244;
																																																																																																																																																																																																																		self.fTempPerm228 = fTemp245;
																																																																																																																																																																																																																		self.fTempPerm229 = fTemp246;
																																																																																																																																																																																																																		self.fTempPerm230 = fTemp247;
																																																																																																																																																																																																																		self.fTempPerm231 = fTemp248;
																																																																																																																																																																																																																		self.fTempPerm232 = fTemp249;
																																																																																																																																																																																																																		self.fTempPerm233 = fTemp250;
																																																																																																																																																																																																																		self.fTempPerm234 = fTemp251;
																																																																																																																																																																																																																		self.fTempPerm235 = fTemp252;
																																																																																																																																																																																																																		self.fTempPerm236 = fTemp253;
																																																																																																																																																																																																																		self.fTempPerm237 = fTemp254;
																																																																																																																																																																																																																		self.fTempPerm238 = fTemp255;
																																																																																																																																																																																																																		self.fTempPerm239 = fTemp256;
																																																																																																																																																																																																																		self.fTempPerm240 = fTemp257;
																																																																																																																																																																																																																		self.fTempPerm241 = fTemp258;
																																																																																																																																																																																																																		self.fTempPerm242 = fTemp259;
																																																																																																																																																																																																																		self.iTempPerm243 = iTemp260;
																																																																																																																																																																																																																		self.iTempPerm244 = iTemp261;
																																																																																																																																																																																																																		self.iTempPerm245 = iTemp262;
																																																																																																																																																																																																																		self.iTempPerm246 = iTemp263;
																																																																																																																																																																																																																		self.fTempPerm247 = fTemp264;
																																																																																																																																																																																																																		self.iTempPerm248 = iTemp265;
																																																																																																																																																																																																																		self.iTempPerm249 = iTemp266;
																																																																																																																																																																																																																		self.iTempPerm250 = iTemp267;
																																																																																																																																																																																																																		self.iTempPerm251 = iTemp268;
																																																																																																																																																																																																																		self.iTempPerm252 = iTemp269;
																																																																																																																																																																																																																		self.iTempPerm253 = iTemp270;
																																																																																																																																																																																																																		self.iTempPerm254 = iTemp271;
																																																																																																																																																																																																																		self.fTempPerm255 = fTemp272;
																																																																																																																																																																																																																		self.fTempPerm256 = fTemp273;
																																																																																																																																																																																																																		self.iTempPerm257 = iTemp274;
																																																																																																																																																																																																																		self.iTempPerm258 = iTemp275;
																																																																																																																																																																																																																		self.fTempPerm259 = fTemp276;
																																																																																																																																																																																																																		self.fTempPerm260 = fTemp277;
																																																																																																																																																																																																																		self.fTempPerm261 = fTemp278;
																																																																																																																																																																																																																		self.fTempPerm262 = fTemp279;
																																																																																																																																																																																																																		self.fTempPerm263 = fTemp280;
																																																																																																																																																																																																																		self.iTempPerm264 = iTemp281;
																																																																																																																																																																																																																		self.fTempPerm265 = fTemp282;
																																																																																																																																																																																																																		self.fTempPerm266 = fTemp283;
																																																																																																																																																																																																																		self.fTempPerm267 = fTemp284;
																																																																																																																																																																																																																		self.fTempPerm268 = fTemp285;
																																																																																																																																																																																																																		self.fTempPerm269 = fTemp286;
																																																																																																																																																																																																																		self.fTempPerm270 = fTemp287;
																																																																																																																																																																																																																		self.fTempPerm271 = fTemp288;
																																																																																																																																																																																																																		self.iTempPerm272 = iTemp289;
																																																																																																																																																																																																																		self.iTempPerm273 = iTemp290;
																																																																																																																																																																																																																		self.iTempPerm274 = iTemp291;
																																																																																																																																																																																																																		self.iTempPerm275 = iTemp292;
																																																																																																																																																																																																																		self.iTempPerm276 = iTemp293;
																																																																																																																																																																																																																		self.iTempPerm277 = iTemp294;
																																																																																																																																																																																																																		self.iTempPerm278 = iTemp295;
																																																																																																																																																																																																																		self.fTempPerm279 = fTemp296;
																																																																																																																																																																																																																		self.fTempPerm280 = fTemp297;
																																																																																																																																																																																																																		self.iTempPerm281 = iTemp298;
																																																																																																																																																																																																																		self.iTempPerm282 = iTemp299;
																																																																																																																																																																																																																		self.iTempPerm283 = iTemp300;
																																																																																																																																																																																																																		self.iTempPerm284 = iTemp301;
																																																																																																																																																																																																																		self.fTempPerm285 = fTemp302;
																																																																																																																																																																																																																		self.fTempPerm286 = fTemp303;
																																																																																																																																																																																																																		self.fTempPerm287 = fTemp304;
																																																																																																																																																																																																																		self.iTempPerm288 = iTemp305;
																																																																																																																																																																																																																		self.fTempPerm289 = fTemp306;
																																																																																																																																																																																																																		self.fTempPerm290 = fTemp307;
																																																																																																																																																																																																																		self.fTempPerm291 = fTemp308;
																																																																																																																																																																																																																		self.fTempPerm292 = fTemp309;
																																																																																																																																																																																																																		self.iTempPerm293 = iTemp310;
																																																																																																																																																																																																																		self.iTempPerm294 = iTemp311;
																																																																																																																																																																																																																		self.iTempPerm295 = iTemp312;
																																																																																																																																																																																																																		self.iTempPerm296 = iTemp313;
																																																																																																																																																																																																																		self.iTempPerm297 = iTemp314;
																																																																																																																																																																																																																		self.iTempPerm298 = iTemp315;
																																																																																																																																																																																																																		self.iTempPerm299 = iTemp316;
																																																																																																																																																																																																																		self.fTempPerm300 = fTemp317;
																																																																																																																																																																																																																		self.iTempPerm301 = iTemp318;
																																																																																																																																																																																																																		self.iTempPerm302 = iTemp319;
																																																																																																																																																																																																																		self.fTempPerm303 = fTemp320;
																																																																																																																																																																																																																		self.fTempPerm304 = fTemp321;
																																																																																																																																																																																																																		self.fTempPerm305 = fTemp322;
																																																																																																																																																																																																																		self.fTempPerm306 = fTemp323;
																																																																																																																																																																																																																		self.fTempPerm307 = fTemp324;
																																																																																																																																																																																																																		self.fTempPerm308 = fTemp325;
																																																																																																																																																																																																																		self.fTempPerm309 = fTemp326;
																																																																																																																																																																																																																		self.fTempPerm310 = fTemp327;
																																																																																																																																																																																																																		self.iTempPerm311 = iTemp328;
																																																																																																																																																																																																																		self.iTempPerm312 = iTemp329;
																																																																																																																																																																																																																		self.fTempPerm313 = fTemp330;
																																																																																																																																																																																																																		self.fTempPerm314 = fTemp331;
																																																																																																																																																																																																																		self.fTempPerm315 = fTemp332;
																																																																																																																																																																																																																		self.fTempPerm316 = fTemp333;
																																																																																																																																																																																																																		self.fTempPerm317 = fTemp334;
																																																																																																																																																																																																																		self.fTempPerm318 = fTemp335;
																																																																																																																																																																																																																		self.fTempPerm319 = fTemp336;
																																																																																																																																																																																																																		self.fTempPerm320 = fTemp337;
																																																																																																																																																																																																																		self.fTempPerm321 = fTemp338;
																																																																																																																																																																																																																		self.fTempPerm322 = fTemp339;
																																																																																																																																																																																																																		self.fTempPerm323 = fTemp340;
																																																																																																																																																																																																																		self.fTempPerm324 = fTemp341;
																																																																																																																																																																																																																		self.fTempPerm325 = fTemp342;
																																																																																																																																																																																																																		self.fTempPerm326 = fTemp343;
																																																																																																																																																																																																																		self.fTempPerm327 = fTemp344;
																																																																																																																																																																																																																		self.fTempPerm328 = fTemp345;
																																																																																																																																																																																																																		self.fTempPerm329 = fTemp346;
																																																																																																																																																																																																																		self.fTempPerm330 = fTemp347;
																																																																																																																																																																																																																		self.fTempPerm331 = fTemp348;
																																																																																																																																																																																																																		self.fTempPerm332 = fTemp349;
																																																																																																																																																																																																																		self.fTempPerm333 = fTemp350;
																																																																																																																																																																																																																		self.fTempPerm334 = fTemp351;
																																																																																																																																																																																																																		self.iTempPerm335 = iTemp352;
																																																																																																																																																																																																																		self.fTempPerm336 = fTemp353;
																																																																																																																																																																																																																		self.fTempPerm337 = fTemp354;
																																																																																																																																																																																																																		self.fTempPerm338 = fTemp355;
																																																																																																																																																																																																																		self.fTempPerm339 = fTemp356;
																																																																																																																																																																																																																		self.fTempPerm340 = fTemp357;
																																																																																																																																																																																																																		self.fTempPerm341 = fTemp358;
																																																																																																																																																																																																																		self.fTempPerm342 = fTemp359;
																																																																																																																																																																																																																		self.fTempPerm343 = fTemp360;
																																																																																																																																																																																																																		self.fTempPerm344 = fTemp361;
																																																																																																																																																																																																																		self.fTempPerm345 = fTemp362;
																																																																																																																																																																																																																		self.fTempPerm346 = fTemp363;
																																																																																																																																																																																																																		self.fTempPerm347 = fTemp364;
																																																																																																																																																																																																																		self.fTempPerm348 = fTemp365;
																																																																																																																																																																																																																		self.iTempPerm349 = iTemp366;
																																																																																																																																																																																																																		self.fTempPerm350 = fTemp367;
																																																																																																																																																																																																																		self.fTempPerm351 = fTemp368;
																																																																																																																																																																																																																		self.fTempPerm352 = fTemp369;
																																																																																																																																																																																																																		self.fTempPerm353 = fTemp370;
																																																																																																																																																																																																																		self.fTempPerm354 = fTemp371;
																																																																																																																																																																																																																		self.fTempPerm355 = fTemp372;
																																																																																																																																																																																																																		self.fTempPerm356 = fTemp373;
																																																																																																																																																																																																																		self.fTempPerm357 = fTemp374;
																																																																																																																																																																																																																		self.fTempPerm358 = fTemp375;
																																																																																																																																																																																																																		self.fTempPerm359 = fTemp376;
																																																																																																																																																																																																																		self.fTempPerm360 = fTemp377;
																																																																																																																																																																																																																		self.fTempPerm361 = fTemp378;
																																																																																																																																																																																																																		self.fTempPerm362 = fTemp379;
																																																																																																																																																																																																																		self.fTempPerm363 = fTemp380;
																																																																																																																																																																																																																		self.fTempPerm364 = fTemp381;
																																																																																																																																																																																																																		self.fTempPerm365 = fTemp382;
																																																																																																																																																																																																																		self.fTempPerm366 = fTemp383;
																																																																																																																																																																																																																		self.fTempPerm367 = fTemp384;
																																																																																																																																																																																																																		self.fTempPerm368 = fTemp385;
																																																																																																																																																																																																																		self.fTempPerm369 = fTemp386;
																																																																																																																																																																																																																		self.fTempPerm370 = fTemp387;
																																																																																																																																																																																																																		self.fTempPerm371 = fTemp388;
																																																																																																																																																																																																																		self.fTempPerm372 = fTemp389;
																																																																																																																																																																																																																		self.fTempPerm373 = fTemp390;
																																																																																																																																																																																																																		self.fTempPerm374 = fTemp391;
																																																																																																																																																																																																																		self.iTempPerm375 = iTemp392;
																																																																																																																																																																																																																		self.fTempPerm376 = fTemp393;
																																																																																																																																																																																																																		self.fTempPerm377 = fTemp394;
																																																																																																																																																																																																																		self.fTempPerm378 = fTemp395;
																																																																																																																																																																																																																		self.fTempPerm379 = fTemp396;
																																																																																																																																																																																																																		self.fTempPerm380 = fTemp397;
																																																																																																																																																																																																																		self.fTempPerm381 = fTemp398;
																																																																																																																																																																																																																		self.fTempPerm382 = fTemp399;
																																																																																																																																																																																																																		self.fTempPerm383 = fTemp400;
																																																																																																																																																																																																																		self.fTempPerm384 = fTemp401;
																																																																																																																																																																																																																		self.fTempPerm385 = fTemp402;
																																																																																																																																																																																																																		self.fTempPerm386 = fTemp403;
																																																																																																																																																																																																																		self.fTempPerm387 = fTemp404;
																																																																																																																																																																																																																		self.fTempPerm388 = fTemp405;
																																																																																																																																																																																																																		self.fTempPerm389 = fTemp406;
																																																																																																																																																																																																																		self.fTempPerm390 = fTemp407;
																																																																																																																																																																																																																		self.fTempPerm391 = fTemp408;
																																																																																																																																																																																																																		self.fTempPerm392 = fTemp409;
																																																																																																																																																																																																																		self.fTempPerm393 = fTemp410;
																																																																																																																																																																																																																		self.fTempPerm394 = fTemp411;
																																																																																																																																																																																																																		self.fTempPerm395 = fTemp412;
																																																																																																																																																																																																																		self.fTempPerm396 = fTemp413;
																																																																																																																																																																																																																		self.iTempPerm397 = iTemp414;
																																																																																																																																																																																																																		self.fTempPerm398 = fTemp415;
																																																																																																																																																																																																																		self.fTempPerm399 = fTemp416;
																																																																																																																																																																																																																		self.fTempPerm400 = fTemp417;
																																																																																																																																																																																																																		self.fTempPerm401 = fTemp418;
																																																																																																																																																																																																																		self.fTempPerm402 = fTemp419;
																																																																																																																																																																																																																		self.fTempPerm403 = fTemp420;
																																																																																																																																																																																																																		self.iTempPerm404 = iTemp421;
																																																																																																																																																																																																																		self.fTempPerm405 = fTemp422;
																																																																																																																																																																																																																		self.fTempPerm406 = fTemp423;
																																																																																																																																																																																																																		self.fTempPerm407 = fTemp424;
																																																																																																																																																																																																																		self.fTempPerm408 = fTemp425;
																																																																																																																																																																																																																		self.fTempPerm409 = fTemp426;
																																																																																																																																																																																																																		self.fTempPerm410 = fTemp427;
																																																																																																																																																																																																																		self.fTempPerm411 = fTemp428;
																																																																																																																																																																																																																		self.iTempPerm412 = iTemp429;
																																																																																																																																																																																																																		self.iTempPerm413 = iTemp430;
																																																																																																																																																																																																																		self.iTempPerm414 = iTemp431;
																																																																																																																																																																																																																		self.iTempPerm415 = iTemp432;
																																																																																																																																																																																																																		self.iTempPerm416 = iTemp433;
																																																																																																																																																																																																																		self.iTempPerm417 = iTemp434;
																																																																																																																																																																																																																		self.fTempPerm418 = fTemp435;
																																																																																																																																																																																																																		self.fTempPerm419 = fTemp436;
																																																																																																																																																																																																																		self.fTempPerm420 = fTemp437;
																																																																																																																																																																																																																		self.iTempPerm421 = iTemp438;
																																																																																																																																																																																																																		self.iTempPerm422 = iTemp439;
																																																																																																																																																																																																																		self.iTempPerm423 = iTemp440;
																																																																																																																																																																																																																		self.iTempPerm424 = iTemp441;
																																																																																																																																																																																																																		self.fTempPerm425 = fTemp442;
																																																																																																																																																																																																																		self.fTempPerm426 = fTemp443;
																																																																																																																																																																																																																		self.fTempPerm427 = fTemp444;
																																																																																																																																																																																																																		self.fTempPerm428 = fTemp445;
																																																																																																																																																																																																																		self.fTempPerm429 = fTemp446;
																																																																																																																																																																																																																		self.fTempPerm430 = fTemp447;
																																																																																																																																																																																																																		self.fTempPerm431 = fTemp448;
																																																																																																																																																																																																																		self.fTempPerm432 = fTemp449;
																																																																																																																																																																																																																		self.fTempPerm433 = fTemp450;
																																																																																																																																																																																																																		self.fTempPerm434 = fTemp451;
																																																																																																																																																																																																																		self.fTempPerm435 = fTemp452;
																																																																																																																																																																																																																		self.fTempPerm436 = fTemp453;
																																																																																																																																																																																																																		self.fTempPerm437 = fTemp454;
																																																																																																																																																																																																																		self.fTempPerm438 = fTemp455;
																																																																																																																																																																																																																		self.fTempPerm439 = fTemp456;
																																																																																																																																																																																																																		self.iTempPerm440 = iTemp457;
																																																																																																																																																																																																																		self.fTempPerm441 = fTemp458;
																																																																																																																																																																																																																		self.fTempPerm442 = fTemp459;
																																																																																																																																																																																																																		self.fTempPerm443 = fTemp460;
																																																																																																																																																																																																																		self.fTempPerm444 = fTemp461;
																																																																																																																																																																																																																		self.fTempPerm445 = fTemp462;
																																																																																																																																																																																																																		self.fTempPerm446 = fTemp463;
																																																																																																																																																																																																																		self.fTempPerm447 = fTemp464;
																																																																																																																																																																																																																		self.fTempPerm448 = fTemp465;
																																																																																																																																																																																																																		self.fTempPerm449 = fTemp466;
																																																																																																																																																																																																																		self.fTempPerm450 = fTemp467;
																																																																																																																																																																																																																		self.fTempPerm451 = fTemp468;
																																																																																																																																																																																																																		self.fTempPerm452 = fTemp469;
																																																																																																																																																																																																																		self.fTempPerm453 = fTemp470;
																																																																																																																																																																																																																		self.fTempPerm454 = fTemp471;
																																																																																																																																																																																																																		self.iTempPerm455 = iTemp472;
																																																																																																																																																																																																																		self.fTempPerm456 = fTemp473;
																																																																																																																																																																																																																		self.fTempPerm457 = fTemp474;
																																																																																																																																																																																																																		self.fTempPerm458 = fTemp475;
																																																																																																																																																																																																																		self.fTempPerm459 = fTemp476;
																																																																																																																																																																																																																		self.iTempPerm460 = iTemp477;
																																																																																																																																																																																																																		self.fTempPerm461 = fTemp478;
																																																																																																																																																																																																																		self.iTempPerm462 = iTemp479;
																																																																																																																																																																																																																		self.fTempPerm463 = fTemp480;
																																																																																																																																																																																																																		self.fTempPerm464 = fTemp481;
																																																																																																																																																																																																																		self.fTempPerm465 = fTemp482;
																																																																																																																																																																																																																		self.fTempPerm466 = fTemp483;
																																																																																																																																																																																																																		self.fTempPerm467 = fTemp484;
																																																																																																																																																																																																																		self.iTempPerm468 = iTemp485;
																																																																																																																																																																																																																		self.fTempPerm469 = fTemp486;
																																																																																																																																																																																																																		self.fTempPerm470 = fTemp487;
																																																																																																																																																																																																																		self.fTempPerm471 = fTemp488;
																																																																																																																																																																																																																		self.fTempPerm472 = fTemp489;
																																																																																																																																																																																																																		self.fTempPerm473 = fTemp490;
																																																																																																																																																																																																																		self.fTempPerm474 = fTemp491;
																																																																																																																																																																																																																		self.fTempPerm475 = fTemp492;
																																																																																																																																																																																																																		self.fTempPerm476 = fTemp493;
																																																																																																																																																																																																																		self.fTempPerm477 = fTemp494;
																																																																																																																																																																																																																		self.fTempPerm478 = fTemp495;
																																																																																																																																																																																																																		self.fTempPerm479 = fTemp496;
																																																																																																																																																																																																																		self.fTempPerm480 = fTemp497;
																																																																																																																																																																																																																		self.fTempPerm481 = fTemp498;
																																																																																																																																																																																																																		self.fTempPerm482 = fTemp499;
																																																																																																																																																																																																																		self.fTempPerm483 = fTemp500;
																																																																																																																																																																																																																		self.fTempPerm484 = fTemp501;
																																																																																																																																																																																																																		self.fTempPerm485 = fTemp502;
																																																																																																																																																																																																																		self.fTempPerm486 = fTemp503;
																																																																																																																																																																																																																		self.fTempPerm487 = fTemp504;
																																																																																																																																																																																																																		self.fTempPerm488 = fTemp505;
																																																																																																																																																																																																																		self.fTempPerm489 = fTemp506;
																																																																																																																																																																																																																		self.iTempPerm490 = iTemp507;
																																																																																																																																																																																																																		self.fTempPerm491 = fTemp508;
																																																																																																																																																																																																																		self.iTempPerm492 = iTemp509;
																																																																																																																																																																																																																		self.fTempPerm493 = fTemp510;
																																																																																																																																																																																																																		self.iTempPerm494 = iTemp511;
																																																																																																																																																																																																																		self.fTempPerm495 = fTemp512;
																																																																																																																																																																																																																		self.iTempPerm496 = iTemp513;
																																																																																																																																																																																																																		self.fTempPerm497 = fTemp514;
																																																																																																																																																																																																																		self.fTempPerm498 = fTemp515;
																																																																																																																																																																																																																		self.fTempPerm499 = fTemp516;
																																																																																																																																																																																																																		self.fTempPerm500 = fTemp517;
																																																																																																																																																																																																																		self.iTempPerm501 = iTemp518;
																																																																																																																																																																																																																		self.fTempPerm502 = fTemp519;
																																																																																																																																																																																																																		self.fTempPerm503 = fTemp520;
																																																																																																																																																																																																																		self.fTempPerm504 = fTemp521;
																																																																																																																																																																																																																		self.fTempPerm505 = fTemp522;
																																																																																																																																																																																																																		self.fTempPerm506 = fTemp523;
																																																																																																																																																																																																																		self.fTempPerm507 = fTemp524;
																																																																																																																																																																																																																		self.fTempPerm508 = fTemp525;
																																																																																																																																																																																																																		self.fTempPerm509 = fTemp526;
																																																																																																																																																																																																																		self.fTempPerm510 = fTemp527;
																																																																																																																																																																																																																		self.fTempPerm511 = fTemp528;
																																																																																																																																																																																																																		self.fTempPerm512 = fTemp529;
																																																																																																																																																																																																																		self.fTempPerm513 = fTemp530;
																																																																																																																																																																																																																		self.fTempPerm514 = fTemp531;
																																																																																																																																																																																																																		self.fTempPerm515 = fTemp532;
																																																																																																																																																																																																																		self.fTempPerm516 = fTemp533;
																																																																																																																																																																																																																		self.fTempPerm517 = fTemp534;
																																																																																																																																																																																																																		self.fTempPerm518 = fTemp535;
																																																																																																																																																																																																																		self.iTempPerm519 = iTemp536;
																																																																																																																																																																																																																		self.fTempPerm520 = fTemp537;
																																																																																																																																																																																																																		self.fTempPerm521 = fTemp538;
																																																																																																																																																																																																																		self.fTempPerm522 = fTemp539;
																																																																																																																																																																																																																		self.fTempPerm523 = fTemp540;
																																																																																																																																																																																																																		self.fTempPerm524 = fTemp541;
																																																																																																																																																																																																																		self.fTempPerm525 = fTemp542;
																																																																																																																																																																																																																		self.fTempPerm526 = fTemp543;
																																																																																																																																																																																																																		self.fTempPerm527 = fTemp544;
																																																																																																																																																																																																																		self.fTempPerm528 = fTemp545;
																																																																																																																																																																																																																		self.fTempPerm529 = fTemp546;
																																																																																																																																																																																																																		self.fTempPerm530 = fTemp547;
																																																																																																																																																																																																																		self.fTempPerm531 = fTemp548;
																																																																																																																																																																																																																		self.fTempPerm532 = fTemp549;
																																																																																																																																																																																																																		self.fTempPerm533 = fTemp550;
																																																																																																																																																																																																																		self.fTempPerm534 = fTemp551;
																																																																																																																																																																																																																		self.fTempPerm535 = fTemp552;
																																																																																																																																																																																																																		self.fTempPerm536 = fTemp553;
																																																																																																																																																																																																																		self.iTempPerm537 = iTemp554;
																																																																																																																																																																																																																		self.fTempPerm538 = fTemp555;
																																																																																																																																																																																																																		self.fTempPerm539 = fTemp556;
																																																																																																																																																																																																																		self.fTempPerm540 = fTemp557;
																																																																																																																																																																																																																		self.fTempPerm541 = fTemp558;
																																																																																																																																																																																																																		self.iTempPerm542 = iTemp559;
																																																																																																																																																																																																																		self.fTempPerm543 = fTemp560;
																																																																																																																																																																																																																		self.iTempPerm544 = iTemp561;
																																																																																																																																																																																																																		self.fTempPerm545 = fTemp562;
																																																																																																																																																																																																																		self.fTempPerm546 = fTemp563;
																																																																																																																																																																																																																		self.fTempPerm547 = fTemp564;
																																																																																																																																																																																																																		self.fTempPerm548 = fTemp565;
																																																																																																																																																																																																																		self.fTempPerm549 = fTemp566;
																																																																																																																																																																																																																		self.fTempPerm550 = fTemp567;
																																																																																																																																																																																																																		self.fTempPerm551 = fTemp568;
																																																																																																																																																																																																																		self.fTempPerm552 = fTemp569;
																																																																																																																																																																																																																		self.fTempPerm553 = fTemp570;
																																																																																																																																																																																																																		self.fTempPerm554 = fTemp571;
																																																																																																																																																																																																																		self.iTempPerm555 = iTemp572;
																																																																																																																																																																																																																		self.iTempPerm556 = iTemp573;
																																																																																																																																																																																																																		self.iTempPerm557 = iTemp574;
																																																																																																																																																																																																																		self.iTempPerm558 = iTemp575;
																																																																																																																																																																																																																		self.fTempPerm559 = fTemp576;
																																																																																																																																																																																																																		self.iTempPerm560 = iTemp577;
																																																																																																																																																																																																																		self.iTempPerm561 = iTemp578;
																																																																																																																																																																																																																		self.iTempPerm562 = iTemp579;
																																																																																																																																																																																																																		self.iTempPerm563 = iTemp580;
																																																																																																																																																																																																																		self.iTempPerm564 = iTemp581;
																																																																																																																																																																																																																		self.iTempPerm565 = iTemp582;
																																																																																																																																																																																																																		self.iTempPerm566 = iTemp583;
																																																																																																																																																																																																																		self.fTempPerm567 = fTemp584;
																																																																																																																																																																																																																		self.fTempPerm568 = fTemp585;
																																																																																																																																																																																																																		self.iTempPerm569 = iTemp586;
																																																																																																																																																																																																																		self.iTempPerm570 = iTemp587;
																																																																																																																																																																																																																		self.fTempPerm571 = fTemp588;
																																																																																																																																																																																																																		self.fTempPerm572 = fTemp589;
																																																																																																																																																																																																																		self.fTempPerm573 = fTemp590;
																																																																																																																																																																																																																		self.fTempPerm574 = fTemp591;
																																																																																																																																																																																																																		self.fTempPerm575 = fTemp592;
																																																																																																																																																																																																																		self.iTempPerm576 = iTemp593;
																																																																																																																																																																																																																		self.fTempPerm577 = fTemp594;
																																																																																																																																																																																																																		self.fTempPerm578 = fTemp595;
																																																																																																																																																																																																																		self.fTempPerm579 = fTemp596;
																																																																																																																																																																																																																		self.fTempPerm580 = fTemp597;
																																																																																																																																																																																																																		self.fTempPerm581 = fTemp598;
																																																																																																																																																																																																																		self.fTempPerm582 = fTemp599;
																																																																																																																																																																																																																		self.fTempPerm583 = fTemp600;
																																																																																																																																																																																																																		self.iTempPerm584 = iTemp601;
																																																																																																																																																																																																																		self.iTempPerm585 = iTemp602;
																																																																																																																																																																																																																		self.iTempPerm586 = iTemp603;
																																																																																																																																																																																																																		self.iTempPerm587 = iTemp604;
																																																																																																																																																																																																																		self.iTempPerm588 = iTemp605;
																																																																																																																																																																																																																		self.iTempPerm589 = iTemp606;
																																																																																																																																																																																																																		self.iTempPerm590 = iTemp607;
																																																																																																																																																																																																																		self.fTempPerm591 = fTemp608;
																																																																																																																																																																																																																		self.fTempPerm592 = fTemp609;
																																																																																																																																																																																																																		self.iTempPerm593 = iTemp610;
																																																																																																																																																																																																																		self.iTempPerm594 = iTemp611;
																																																																																																																																																																																																																		self.iTempPerm595 = iTemp612;
																																																																																																																																																																																																																		self.iTempPerm596 = iTemp613;
																																																																																																																																																																																																																		self.fTempPerm597 = fTemp614;
																																																																																																																																																																																																																		self.fTempPerm598 = fTemp615;
																																																																																																																																																																																																																		self.fTempPerm599 = fTemp616;
																																																																																																																																																																																																																		self.iTempPerm600 = iTemp617;
																																																																																																																																																																																																																		self.fTempPerm601 = fTemp618;
																																																																																																																																																																																																																		self.fTempPerm602 = fTemp619;
																																																																																																																																																																																																																		self.fTempPerm603 = fTemp620;
																																																																																																																																																																																																																		self.fTempPerm604 = fTemp621;
																																																																																																																																																																																																																		self.iTempPerm605 = iTemp622;
																																																																																																																																																																																																																		self.iTempPerm606 = iTemp623;
																																																																																																																																																																																																																		self.iTempPerm607 = iTemp624;
																																																																																																																																																																																																																		self.iTempPerm608 = iTemp625;
																																																																																																																																																																																																																		self.iTempPerm609 = iTemp626;
																																																																																																																																																																																																																		self.iTempPerm610 = iTemp627;
																																																																																																																																																																																																																		self.iTempPerm611 = iTemp628;
																																																																																																																																																																																																																		self.fTempPerm612 = fTemp629;
																																																																																																																																																																																																																		self.iTempPerm613 = iTemp630;
																																																																																																																																																																																																																		self.iTempPerm614 = iTemp631;
																																																																																																																																																																																																																		self.fTempPerm615 = fTemp632;
																																																																																																																																																																																																																		self.fTempPerm616 = fTemp633;
																																																																																																																																																																																																																		self.fTempPerm617 = fTemp634;
																																																																																																																																																																																																																		self.fTempPerm618 = fTemp635;
																																																																																																																																																																																																																		self.fTempPerm619 = fTemp636;
																																																																																																																																																																																																																		self.fTempPerm620 = fTemp637;
																																																																																																																																																																																																																		self.fTempPerm621 = fTemp638;
																																																																																																																																																																																																																		self.fTempPerm622 = fTemp639;
																																																																																																																																																																																																																		self.iTempPerm623 = iTemp640;
																																																																																																																																																																																																																		self.iTempPerm624 = iTemp641;
																																																																																																																																																																																																																		self.fTempPerm625 = fTemp642;
																																																																																																																																																																																																																		self.fTempPerm626 = fTemp643;
																																																																																																																																																																																																																		self.fTempPerm627 = fTemp644;
																																																																																																																																																																																																																		self.fTempPerm628 = fTemp645;
																																																																																																																																																																																																																		self.fTempPerm629 = fTemp646;
																																																																																																																																																																																																																		self.fTempPerm630 = fTemp647;
																																																																																																																																																																																																																		self.fTempPerm631 = fTemp648;
																																																																																																																																																																																																																		self.fTempPerm632 = fTemp649;
																																																																																																																																																																																																																		self.fTempPerm633 = fTemp650;
																																																																																																																																																																																																																		self.fTempPerm634 = fTemp651;
																																																																																																																																																																																																																		self.fTempPerm635 = fTemp652;
																																																																																																																																																																																																																		self.fTempPerm636 = fTemp653;
																																																																																																																																																																																																																		self.fTempPerm637 = fTemp654;
																																																																																																																																																																																																																		self.fTempPerm638 = fTemp655;
																																																																																																																																																																																																																		self.fTempPerm639 = fTemp656;
																																																																																																																																																																																																																		self.fTempPerm640 = fTemp657;
																																																																																																																																																																																																																		self.fTempPerm641 = fTemp658;
																																																																																																																																																																																																																		self.fTempPerm642 = fTemp659;
																																																																																																																																																																																																																		self.fTempPerm643 = fTemp660;
																																																																																																																																																																																																																		self.fTempPerm644 = fTemp661;
																																																																																																																																																																																																																	}

}

