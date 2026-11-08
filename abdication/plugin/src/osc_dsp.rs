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
	iVec2: [i32;2],
	iRec7: [i32;2],
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
			self.iVec2[l8 as usize] = 0;
		}
		for l9 in 0..2 {
			self.iRec7[l9 as usize] = 0;
		}
	}
	
	fn fillOscDspSIG2(&mut self, count: i32, table: &mut[F32]) {
		for i3 in 0..count {
			self.iVec2[0] = 1;
			self.iRec7[0] = (i32::wrapping_add(self.iVec2[1], self.iRec7[1])) % 65536;
			table[i3 as usize] = F32::sin(9.58738e-05 * (self.iRec7[0]) as F32);
			self.iVec2[1] = self.iVec2[0];
			self.iRec7[1] = self.iRec7[0];
		}
	}

}


pub fn newOscDspSIG2() -> OscDspSIG2 { 
	OscDspSIG2 {
		iVec2: [0;2],
		iRec7: [0;2],
	}
}
static mut iOscDspSIG3Wave0: [i32;9] = [0,1,2,3,1,0,3,1,0];

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
			self.iOscDspSIG3Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG3Wave0_idx)) % 9;
		}
	}

}


pub fn newOscDspSIG3() -> OscDspSIG3 { 
	OscDspSIG3 {
		iOscDspSIG3Wave0_idx: 0,
	}
}
static mut iOscDspSIG4Wave0: [i32;9] = [0,1,2,3,3,3,2,2,2];

pub struct OscDspSIG4 {
	iOscDspSIG4Wave0_idx: i32,
}

impl OscDspSIG4 {
	
	fn get_num_inputsOscDspSIG4(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG4(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG4(&mut self, sample_rate: i32) {
		self.iOscDspSIG4Wave0_idx = 0;
	}
	
	fn fillOscDspSIG4(&mut self, count: i32, table: &mut[i32]) {
		for i5 in 0..count {
			table[i5 as usize] = unsafe { iOscDspSIG4Wave0[self.iOscDspSIG4Wave0_idx as usize] };
			self.iOscDspSIG4Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG4Wave0_idx)) % 9;
		}
	}

}


pub fn newOscDspSIG4() -> OscDspSIG4 { 
	OscDspSIG4 {
		iOscDspSIG4Wave0_idx: 0,
	}
}
static mut fOscDspSIG5Wave0: [F32;10] = [0.5,1.0,1.5,2.0,3.0,4.0,5.0,6.0,7.0,8.0];

pub struct OscDspSIG5 {
	fOscDspSIG5Wave0_idx: i32,
}

impl OscDspSIG5 {
	
	fn get_num_inputsOscDspSIG5(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG5(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG5(&mut self, sample_rate: i32) {
		self.fOscDspSIG5Wave0_idx = 0;
	}
	
	fn fillOscDspSIG5(&mut self, count: i32, table: &mut[F32]) {
		for i6 in 0..count {
			table[i6 as usize] = unsafe { fOscDspSIG5Wave0[self.fOscDspSIG5Wave0_idx as usize] };
			self.fOscDspSIG5Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG5Wave0_idx)) % 10;
		}
	}

}


pub fn newOscDspSIG5() -> OscDspSIG5 { 
	OscDspSIG5 {
		fOscDspSIG5Wave0_idx: 0,
	}
}
static mut fOscDspSIG6Wave0: [F32;12] = [0.5,1.0,1.5,2.0,2.5,3.0,3.5,4.0,5.0,6.0,7.0,8.0];

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
static mut iOscDspSIG7Wave0: [i32;5] = [3,4,5,7,12];

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
			self.iOscDspSIG7Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG7Wave0_idx)) % 5;
		}
	}

}


pub fn newOscDspSIG7() -> OscDspSIG7 { 
	OscDspSIG7 {
		iOscDspSIG7Wave0_idx: 0,
	}
}
static mut fOscDspSIG8Wave0: [F32;8] = [1.4,0.8,1.0,0.95,1.1,1.0,1.1,0.77];

pub struct OscDspSIG8 {
	fOscDspSIG8Wave0_idx: i32,
}

impl OscDspSIG8 {
	
	fn get_num_inputsOscDspSIG8(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG8(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG8(&mut self, sample_rate: i32) {
		self.fOscDspSIG8Wave0_idx = 0;
	}
	
	fn fillOscDspSIG8(&mut self, count: i32, table: &mut[F32]) {
		for i9 in 0..count {
			table[i9 as usize] = unsafe { fOscDspSIG8Wave0[self.fOscDspSIG8Wave0_idx as usize] };
			self.fOscDspSIG8Wave0_idx = (i32::wrapping_add(1, self.fOscDspSIG8Wave0_idx)) % 8;
		}
	}

}


pub fn newOscDspSIG8() -> OscDspSIG8 { 
	OscDspSIG8 {
		fOscDspSIG8Wave0_idx: 0,
	}
}
static mut iOscDspSIG9Wave0: [i32;7] = [-12,-6,-3,0,-3,-6,-12];

pub struct OscDspSIG9 {
	iOscDspSIG9Wave0_idx: i32,
}

impl OscDspSIG9 {
	
	fn get_num_inputsOscDspSIG9(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG9(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG9(&mut self, sample_rate: i32) {
		self.iOscDspSIG9Wave0_idx = 0;
	}
	
	fn fillOscDspSIG9(&mut self, count: i32, table: &mut[i32]) {
		for i10 in 0..count {
			table[i10 as usize] = unsafe { iOscDspSIG9Wave0[self.iOscDspSIG9Wave0_idx as usize] };
			self.iOscDspSIG9Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG9Wave0_idx)) % 7;
		}
	}

}


pub fn newOscDspSIG9() -> OscDspSIG9 { 
	OscDspSIG9 {
		iOscDspSIG9Wave0_idx: 0,
	}
}
static mut iOscDspSIG10Wave0: [i32;7] = [-9,-90,-3,0,-3,-90,-9];

pub struct OscDspSIG10 {
	iOscDspSIG10Wave0_idx: i32,
}

impl OscDspSIG10 {
	
	fn get_num_inputsOscDspSIG10(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsOscDspSIG10(&self) -> i32 {
		return 1;
	}
	
	fn instance_initOscDspSIG10(&mut self, sample_rate: i32) {
		self.iOscDspSIG10Wave0_idx = 0;
	}
	
	fn fillOscDspSIG10(&mut self, count: i32, table: &mut[i32]) {
		for i11 in 0..count {
			table[i11 as usize] = unsafe { iOscDspSIG10Wave0[self.iOscDspSIG10Wave0_idx as usize] };
			self.iOscDspSIG10Wave0_idx = (i32::wrapping_add(1, self.iOscDspSIG10Wave0_idx)) % 7;
		}
	}

}


pub fn newOscDspSIG10() -> OscDspSIG10 { 
	OscDspSIG10 {
		iOscDspSIG10Wave0_idx: 0,
	}
}
static mut ftbl0OscDspSIG0: [F32;7] = [0.0;7];
static mut ftbl1OscDspSIG1: [F32;7] = [0.0;7];
static mut ftbl2OscDspSIG2: [F32;65536] = [0.0;65536];
fn OscDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut itbl3OscDspSIG3: [i32;9] = [0;9];
static mut itbl4OscDspSIG4: [i32;9] = [0;9];
static mut ftbl5OscDspSIG5: [F32;10] = [0.0;10];
static mut ftbl6OscDspSIG6: [F32;12] = [0.0;12];
static mut itbl7OscDspSIG7: [i32;5] = [0;5];
static mut ftbl8OscDspSIG8: [F32;8] = [0.0;8];
static mut itbl9OscDspSIG9: [i32;7] = [0;7];
static mut itbl10OscDspSIG10: [i32;7] = [0;7];
fn OscDsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct OscDsp {
	fVslider0: F32,
	fEntry0: F32,
	fButton0: F32,
	fEntry1: F32,
	fVslider1: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	iTempPerm0: i32,
	iTempPerm1: i32,
	fTempPerm2: F32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fTempPerm3: F32,
	fHslider0: F32,
	fRec1: [F32;2],
	fHslider1: F32,
	fRec2: [F32;2],
	fTempPerm4: F32,
	fEntry2: F32,
	fHslider2: F32,
	fRec3: [F32;2],
	fHslider3: F32,
	fRec4: [F32;2],
	fTempPerm5: F32,
	fEntry3: F32,
	fEntry4: F32,
	fEntry5: F32,
	fHslider4: F32,
	fHslider5: F32,
	fHslider6: F32,
	fRec5: [F32;2],
	fTempPerm6: F32,
	fTempPerm7: F32,
	fTempPerm8: F32,
	fTempPerm9: F32,
	fTempPerm10: F32,
	fHslider7: F32,
	fEntry6: F32,
	fHslider8: F32,
	fHslider9: F32,
	fHslider10: F32,
	fRec6: [F32;2],
	fTempPerm11: F32,
	fTempPerm12: F32,
	iTempPerm13: i32,
	fTempPerm14: F32,
	iTempPerm15: i32,
	fConst2: F32,
	fTempPerm16: F32,
	fRec8: [F32;2],
	fHslider11: F32,
	fConst3: F32,
	fTempPerm17: F32,
	fRec9: [F32;2],
	iRec11: [i32;2],
	iTempPerm18: i32,
	fConst4: F32,
	fConst6: F32,
	fRec10: [F32;2],
	fHslider12: F32,
	fRec12: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fTempPerm19: F32,
	fRec15: [F32;2],
	iRec16: [i32;2],
	fTempPerm20: F32,
	fRec14: [F32;2],
	fTempPerm21: F32,
	fVec3: [F32;2],
	fRec13: [F32;2],
	fTempPerm22: F32,
	fConst9: F32,
	fTempPerm23: F32,
	fRec17: [F32;2],
	fHslider13: F32,
	fRec18: [F32;2],
	fEntry7: F32,
	fHslider14: F32,
	fHslider15: F32,
	fHslider16: F32,
	fTempPerm24: F32,
	iTempPerm25: i32,
	fTempPerm26: F32,
	fHslider17: F32,
	fRec19: [F32;2],
	fTempPerm27: F32,
	fTempPerm28: F32,
	fTempPerm29: F32,
	fRec20: [F32;2],
	fTempPerm30: F32,
	fHslider18: F32,
	fTempPerm31: F32,
	fCheckbox0: F32,
	fTempPerm32: F32,
	fTempPerm33: F32,
	fRec0: [F32;2],
	iTempPerm34: i32,
	fHslider19: F32,
	fHslider20: F32,
	fHslider21: F32,
	fRec21: [F32;2],
	fTempPerm35: F32,
	fTempPerm36: F32,
	fTempPerm37: F32,
	fHslider22: F32,
	fHslider23: F32,
	fHslider24: F32,
	fRec22: [F32;2],
	fTempPerm38: F32,
	fTempPerm39: F32,
	fTempPerm40: F32,
	fTempPerm41: F32,
	fTempPerm42: F32,
	fEntry8: F32,
	fTempPerm43: F32,
	fTempPerm44: F32,
	iRec23: [i32;2],
	fTempPerm45: F32,
	fTempPerm46: F32,
	fTempPerm47: F32,
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	fTempPerm51: F32,
	fTempPerm52: F32,
	iTempPerm53: i32,
	iTempPerm54: i32,
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
	fRec24: [F32;2],
	fTempPerm66: F32,
	iTempPerm67: i32,
	fTempPerm68: F32,
	fTempPerm69: F32,
	fTempPerm70: F32,
	fTempPerm71: F32,
	fRec25: [F32;2],
	fTempPerm72: F32,
	fTempPerm73: F32,
	fTempPerm74: F32,
	fTempPerm75: F32,
	fRec26: [F32;2],
	fTempPerm76: F32,
	fTempPerm77: F32,
	fVec4: [F32;2],
	fRec27: [F32;2],
	fTempPerm78: F32,
	iTempPerm79: i32,
	fTempPerm80: F32,
	fTempPerm81: F32,
	iTempPerm82: i32,
	fTempPerm83: F32,
	fTempPerm84: F32,
	fTempPerm85: F32,
	fTempPerm86: F32,
	fRec28: [F32;2],
	fTempPerm87: F32,
	fTempPerm88: F32,
	fTempPerm89: F32,
	fTempPerm90: F32,
	fRec29: [F32;2],
	fTempPerm91: F32,
	fTempPerm92: F32,
	fTempPerm93: F32,
	fRec30: [F32;2],
	fTempPerm94: F32,
	fTempPerm95: F32,
	fTempPerm96: F32,
	fRec31: [F32;2],
	fTempPerm97: F32,
	fTempPerm98: F32,
	fRec32: [F32;2],
	fTempPerm99: F32,
	fTempPerm100: F32,
	fRec33: [F32;2],
	fTempPerm101: F32,
	fTempPerm102: F32,
	fTempPerm103: F32,
	fVslider2: F32,
	fTempPerm104: F32,
	iRec34: [i32;2],
	fTempPerm105: F32,
	iTempPerm106: i32,
	fVslider3: F32,
	fTempPerm107: F32,
	fTempPerm108: F32,
	iTempPerm109: i32,
	fHslider25: F32,
	fHslider26: F32,
	fHslider27: F32,
	fTempPerm110: F32,
	fTempPerm111: F32,
	fTempPerm112: F32,
	fVslider4: F32,
	fTempPerm113: F32,
	fTempPerm114: F32,
	fTempPerm115: F32,
	fTempPerm116: F32,
	iTempPerm117: i32,
	fEntry9: F32,
	fEntry10: F32,
	iTempPerm118: i32,
	fConst11: F32,
	fConst12: F32,
	fEntry11: F32,
	fTempPerm119: F32,
	fConst13: F32,
	fConst14: F32,
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	iTempPerm120: i32,
	iTempPerm121: i32,
	iTempPerm122: i32,
	iTempPerm123: i32,
	iTempPerm124: i32,
	iTempPerm125: i32,
	fTempPerm126: F32,
	fRec35: [F32;2],
	fTempPerm127: F32,
	iTempPerm128: i32,
	iTempPerm129: i32,
	iTempPerm130: i32,
	iTempPerm131: i32,
	fTempPerm132: F32,
	fTempPerm133: F32,
	iTempPerm134: i32,
	fTempPerm135: F32,
	fTempPerm136: F32,
	fTempPerm137: F32,
	fEntry12: F32,
	iTempPerm138: i32,
	fTempPerm139: F32,
	fTempPerm140: F32,
	fHslider28: F32,
	fHslider29: F32,
	fHslider30: F32,
	fRec36: [F32;2],
	fTempPerm141: F32,
	fTempPerm142: F32,
	fTempPerm143: F32,
	fTempPerm144: F32,
	fTempPerm145: F32,
	iTempPerm146: i32,
	iTempPerm147: i32,
	iTempPerm148: i32,
	iTempPerm149: i32,
	iTempPerm150: i32,
	iTempPerm151: i32,
	iTempPerm152: i32,
	fTempPerm153: F32,
	fTempPerm154: F32,
	iTempPerm155: i32,
	iTempPerm156: i32,
	iTempPerm157: i32,
	fTempPerm158: F32,
	fTempPerm159: F32,
	iTempPerm160: i32,
	fTempPerm161: F32,
	fTempPerm162: F32,
	fTempPerm163: F32,
	fTempPerm164: F32,
	fTempPerm165: F32,
	iTempPerm166: i32,
	iTempPerm167: i32,
	iTempPerm168: i32,
	iTempPerm169: i32,
	iTempPerm170: i32,
	iTempPerm171: i32,
	fTempPerm172: F32,
	iTempPerm173: i32,
	iTempPerm174: i32,
	iTempPerm175: i32,
	fTempPerm176: F32,
	fTempPerm177: F32,
	iTempPerm178: i32,
	fTempPerm179: F32,
	fTempPerm180: F32,
	fTempPerm181: F32,
	fTempPerm182: F32,
	fTempPerm183: F32,
	fTempPerm184: F32,
	iTempPerm185: i32,
	iTempPerm186: i32,
	fTempPerm187: F32,
	fTempPerm188: F32,
	iTempPerm189: i32,
	fTempPerm190: F32,
	fTempPerm191: F32,
	fTempPerm192: F32,
	fRec38: [F32;2],
	fTempPerm193: F32,
	iTempPerm194: i32,
	iTempPerm195: i32,
	fTempPerm196: F32,
	fTempPerm197: F32,
	iTempPerm198: i32,
	fTempPerm199: F32,
	fRec37: [F32;2],
	fTempPerm200: F32,
	fTempPerm201: F32,
	fTempPerm202: F32,
	fConst19: F32,
	fConst20: F32,
	fTempPerm203: F32,
	fTempPerm204: F32,
	fTempPerm205: F32,
	fRec39: [F32;3],
	fTempPerm206: F32,
	fTempPerm207: F32,
	fTempPerm208: F32,
	fTempPerm209: F32,
	fTempPerm210: F32,
	fTempPerm211: F32,
	fTempPerm212: F32,
	fTempPerm213: F32,
}

impl FaustDsp for OscDsp {
	type T = F32;
		
	fn new() -> OscDsp { 
		OscDsp {
			fVslider0: 0.0,
			fEntry0: 0.0,
			fButton0: 0.0,
			fEntry1: 0.0,
			fVslider1: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			iTempPerm0: 0,
			iTempPerm1: 0,
			fTempPerm2: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fTempPerm3: 0.0,
			fHslider0: 0.0,
			fRec1: [0.0;2],
			fHslider1: 0.0,
			fRec2: [0.0;2],
			fTempPerm4: 0.0,
			fEntry2: 0.0,
			fHslider2: 0.0,
			fRec3: [0.0;2],
			fHslider3: 0.0,
			fRec4: [0.0;2],
			fTempPerm5: 0.0,
			fEntry3: 0.0,
			fEntry4: 0.0,
			fEntry5: 0.0,
			fHslider4: 0.0,
			fHslider5: 0.0,
			fHslider6: 0.0,
			fRec5: [0.0;2],
			fTempPerm6: 0.0,
			fTempPerm7: 0.0,
			fTempPerm8: 0.0,
			fTempPerm9: 0.0,
			fTempPerm10: 0.0,
			fHslider7: 0.0,
			fEntry6: 0.0,
			fHslider8: 0.0,
			fHslider9: 0.0,
			fHslider10: 0.0,
			fRec6: [0.0;2],
			fTempPerm11: 0.0,
			fTempPerm12: 0.0,
			iTempPerm13: 0,
			fTempPerm14: 0.0,
			iTempPerm15: 0,
			fConst2: 0.0,
			fTempPerm16: 0.0,
			fRec8: [0.0;2],
			fHslider11: 0.0,
			fConst3: 0.0,
			fTempPerm17: 0.0,
			fRec9: [0.0;2],
			iRec11: [0;2],
			iTempPerm18: 0,
			fConst4: 0.0,
			fConst6: 0.0,
			fRec10: [0.0;2],
			fHslider12: 0.0,
			fRec12: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fTempPerm19: 0.0,
			fRec15: [0.0;2],
			iRec16: [0;2],
			fTempPerm20: 0.0,
			fRec14: [0.0;2],
			fTempPerm21: 0.0,
			fVec3: [0.0;2],
			fRec13: [0.0;2],
			fTempPerm22: 0.0,
			fConst9: 0.0,
			fTempPerm23: 0.0,
			fRec17: [0.0;2],
			fHslider13: 0.0,
			fRec18: [0.0;2],
			fEntry7: 0.0,
			fHslider14: 0.0,
			fHslider15: 0.0,
			fHslider16: 0.0,
			fTempPerm24: 0.0,
			iTempPerm25: 0,
			fTempPerm26: 0.0,
			fHslider17: 0.0,
			fRec19: [0.0;2],
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fTempPerm29: 0.0,
			fRec20: [0.0;2],
			fTempPerm30: 0.0,
			fHslider18: 0.0,
			fTempPerm31: 0.0,
			fCheckbox0: 0.0,
			fTempPerm32: 0.0,
			fTempPerm33: 0.0,
			fRec0: [0.0;2],
			iTempPerm34: 0,
			fHslider19: 0.0,
			fHslider20: 0.0,
			fHslider21: 0.0,
			fRec21: [0.0;2],
			fTempPerm35: 0.0,
			fTempPerm36: 0.0,
			fTempPerm37: 0.0,
			fHslider22: 0.0,
			fHslider23: 0.0,
			fHslider24: 0.0,
			fRec22: [0.0;2],
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fTempPerm40: 0.0,
			fTempPerm41: 0.0,
			fTempPerm42: 0.0,
			fEntry8: 0.0,
			fTempPerm43: 0.0,
			fTempPerm44: 0.0,
			iRec23: [0;2],
			fTempPerm45: 0.0,
			fTempPerm46: 0.0,
			fTempPerm47: 0.0,
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			fTempPerm52: 0.0,
			iTempPerm53: 0,
			iTempPerm54: 0,
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
			fRec24: [0.0;2],
			fTempPerm66: 0.0,
			iTempPerm67: 0,
			fTempPerm68: 0.0,
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			fTempPerm71: 0.0,
			fRec25: [0.0;2],
			fTempPerm72: 0.0,
			fTempPerm73: 0.0,
			fTempPerm74: 0.0,
			fTempPerm75: 0.0,
			fRec26: [0.0;2],
			fTempPerm76: 0.0,
			fTempPerm77: 0.0,
			fVec4: [0.0;2],
			fRec27: [0.0;2],
			fTempPerm78: 0.0,
			iTempPerm79: 0,
			fTempPerm80: 0.0,
			fTempPerm81: 0.0,
			iTempPerm82: 0,
			fTempPerm83: 0.0,
			fTempPerm84: 0.0,
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			fRec28: [0.0;2],
			fTempPerm87: 0.0,
			fTempPerm88: 0.0,
			fTempPerm89: 0.0,
			fTempPerm90: 0.0,
			fRec29: [0.0;2],
			fTempPerm91: 0.0,
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fRec30: [0.0;2],
			fTempPerm94: 0.0,
			fTempPerm95: 0.0,
			fTempPerm96: 0.0,
			fRec31: [0.0;2],
			fTempPerm97: 0.0,
			fTempPerm98: 0.0,
			fRec32: [0.0;2],
			fTempPerm99: 0.0,
			fTempPerm100: 0.0,
			fRec33: [0.0;2],
			fTempPerm101: 0.0,
			fTempPerm102: 0.0,
			fTempPerm103: 0.0,
			fVslider2: 0.0,
			fTempPerm104: 0.0,
			iRec34: [0;2],
			fTempPerm105: 0.0,
			iTempPerm106: 0,
			fVslider3: 0.0,
			fTempPerm107: 0.0,
			fTempPerm108: 0.0,
			iTempPerm109: 0,
			fHslider25: 0.0,
			fHslider26: 0.0,
			fHslider27: 0.0,
			fTempPerm110: 0.0,
			fTempPerm111: 0.0,
			fTempPerm112: 0.0,
			fVslider4: 0.0,
			fTempPerm113: 0.0,
			fTempPerm114: 0.0,
			fTempPerm115: 0.0,
			fTempPerm116: 0.0,
			iTempPerm117: 0,
			fEntry9: 0.0,
			fEntry10: 0.0,
			iTempPerm118: 0,
			fConst11: 0.0,
			fConst12: 0.0,
			fEntry11: 0.0,
			fTempPerm119: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			iTempPerm120: 0,
			iTempPerm121: 0,
			iTempPerm122: 0,
			iTempPerm123: 0,
			iTempPerm124: 0,
			iTempPerm125: 0,
			fTempPerm126: 0.0,
			fRec35: [0.0;2],
			fTempPerm127: 0.0,
			iTempPerm128: 0,
			iTempPerm129: 0,
			iTempPerm130: 0,
			iTempPerm131: 0,
			fTempPerm132: 0.0,
			fTempPerm133: 0.0,
			iTempPerm134: 0,
			fTempPerm135: 0.0,
			fTempPerm136: 0.0,
			fTempPerm137: 0.0,
			fEntry12: 0.0,
			iTempPerm138: 0,
			fTempPerm139: 0.0,
			fTempPerm140: 0.0,
			fHslider28: 0.0,
			fHslider29: 0.0,
			fHslider30: 0.0,
			fRec36: [0.0;2],
			fTempPerm141: 0.0,
			fTempPerm142: 0.0,
			fTempPerm143: 0.0,
			fTempPerm144: 0.0,
			fTempPerm145: 0.0,
			iTempPerm146: 0,
			iTempPerm147: 0,
			iTempPerm148: 0,
			iTempPerm149: 0,
			iTempPerm150: 0,
			iTempPerm151: 0,
			iTempPerm152: 0,
			fTempPerm153: 0.0,
			fTempPerm154: 0.0,
			iTempPerm155: 0,
			iTempPerm156: 0,
			iTempPerm157: 0,
			fTempPerm158: 0.0,
			fTempPerm159: 0.0,
			iTempPerm160: 0,
			fTempPerm161: 0.0,
			fTempPerm162: 0.0,
			fTempPerm163: 0.0,
			fTempPerm164: 0.0,
			fTempPerm165: 0.0,
			iTempPerm166: 0,
			iTempPerm167: 0,
			iTempPerm168: 0,
			iTempPerm169: 0,
			iTempPerm170: 0,
			iTempPerm171: 0,
			fTempPerm172: 0.0,
			iTempPerm173: 0,
			iTempPerm174: 0,
			iTempPerm175: 0,
			fTempPerm176: 0.0,
			fTempPerm177: 0.0,
			iTempPerm178: 0,
			fTempPerm179: 0.0,
			fTempPerm180: 0.0,
			fTempPerm181: 0.0,
			fTempPerm182: 0.0,
			fTempPerm183: 0.0,
			fTempPerm184: 0.0,
			iTempPerm185: 0,
			iTempPerm186: 0,
			fTempPerm187: 0.0,
			fTempPerm188: 0.0,
			iTempPerm189: 0,
			fTempPerm190: 0.0,
			fTempPerm191: 0.0,
			fTempPerm192: 0.0,
			fRec38: [0.0;2],
			fTempPerm193: 0.0,
			iTempPerm194: 0,
			iTempPerm195: 0,
			fTempPerm196: 0.0,
			fTempPerm197: 0.0,
			iTempPerm198: 0,
			fTempPerm199: 0.0,
			fRec37: [0.0;2],
			fTempPerm200: 0.0,
			fTempPerm201: 0.0,
			fTempPerm202: 0.0,
			fConst19: 0.0,
			fConst20: 0.0,
			fTempPerm203: 0.0,
			fTempPerm204: 0.0,
			fTempPerm205: 0.0,
			fRec39: [0.0;3],
			fTempPerm206: 0.0,
			fTempPerm207: 0.0,
			fTempPerm208: 0.0,
			fTempPerm209: 0.0,
			fTempPerm210: 0.0,
			fTempPerm211: 0.0,
			fTempPerm212: 0.0,
			fTempPerm213: 0.0,
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
		return 2;
	}
	fn get_num_outputs(&self) -> i32 {
		return 4;
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
		sig3.fillOscDspSIG3(9, unsafe { &mut itbl3OscDspSIG3 });
		let mut sig4: OscDspSIG4 = newOscDspSIG4();
		sig4.instance_initOscDspSIG4(sample_rate);
		sig4.fillOscDspSIG4(9, unsafe { &mut itbl4OscDspSIG4 });
		let mut sig5: OscDspSIG5 = newOscDspSIG5();
		sig5.instance_initOscDspSIG5(sample_rate);
		sig5.fillOscDspSIG5(10, unsafe { &mut ftbl5OscDspSIG5 });
		let mut sig6: OscDspSIG6 = newOscDspSIG6();
		sig6.instance_initOscDspSIG6(sample_rate);
		sig6.fillOscDspSIG6(12, unsafe { &mut ftbl6OscDspSIG6 });
		let mut sig7: OscDspSIG7 = newOscDspSIG7();
		sig7.instance_initOscDspSIG7(sample_rate);
		sig7.fillOscDspSIG7(5, unsafe { &mut itbl7OscDspSIG7 });
		let mut sig8: OscDspSIG8 = newOscDspSIG8();
		sig8.instance_initOscDspSIG8(sample_rate);
		sig8.fillOscDspSIG8(8, unsafe { &mut ftbl8OscDspSIG8 });
		let mut sig9: OscDspSIG9 = newOscDspSIG9();
		sig9.instance_initOscDspSIG9(sample_rate);
		sig9.fillOscDspSIG9(7, unsafe { &mut itbl9OscDspSIG9 });
		let mut sig10: OscDspSIG10 = newOscDspSIG10();
		sig10.instance_initOscDspSIG10(sample_rate);
		sig10.fillOscDspSIG10(7, unsafe { &mut itbl10OscDspSIG10 });
	}
	fn instance_reset_params(&mut self) {
		self.fVslider0 = 0.0;
		self.fEntry0 = 0.0;
		self.fButton0 = 0.0;
		self.fEntry1 = 0.0;
		self.fVslider1 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 5e+01;
		self.fEntry2 = 1.0;
		self.fHslider2 = 0.0;
		self.fHslider3 = 5e+01;
		self.fEntry3 = 21.0;
		self.fEntry4 = 0.0;
		self.fEntry5 = 0.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 6.0;
		self.fHslider7 = 3.0;
		self.fEntry6 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 1.0;
		self.fHslider13 = 0.0;
		self.fEntry7 = 0.0;
		self.fHslider14 = 0.0;
		self.fHslider15 = 0.0;
		self.fHslider16 = 0.0;
		self.fHslider17 = 1.1e+02;
		self.fHslider18 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fHslider19 = 0.0;
		self.fHslider20 = 0.0;
		self.fHslider21 = 0.0;
		self.fHslider22 = 0.0;
		self.fHslider23 = 0.0;
		self.fHslider24 = 0.3;
		self.fEntry8 = 1.0;
		self.fVslider2 = 0.005;
		self.fVslider3 = 0.4;
		self.fHslider25 = 0.0;
		self.fHslider26 = 0.0;
		self.fHslider27 = 2.0;
		self.fVslider4 = 0.5;
		self.fEntry9 = 4e+01;
		self.fEntry10 = 8e+01;
		self.fEntry11 = 0.0;
		self.fEntry12 = 0.0;
		self.fHslider28 = 0.0;
		self.fHslider29 = 0.0;
		self.fHslider30 = 0.5;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.iVec1[l1 as usize] = 0;
		}
		self.iTempPerm0 = 0;
		self.iTempPerm1 = 0;
		self.fTempPerm2 = 0.0;
		self.fTempPerm3 = 0.0;
		for l2 in 0..2 {
			self.fRec1[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec2[l3 as usize] = 0.0;
		}
		self.fTempPerm4 = 0.0;
		for l4 in 0..2 {
			self.fRec3[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec4[l5 as usize] = 0.0;
		}
		self.fTempPerm5 = 0.0;
		for l6 in 0..2 {
			self.fRec5[l6 as usize] = 0.0;
		}
		self.fTempPerm6 = 0.0;
		self.fTempPerm7 = 0.0;
		self.fTempPerm8 = 0.0;
		self.fTempPerm9 = 0.0;
		self.fTempPerm10 = 0.0;
		for l7 in 0..2 {
			self.fRec6[l7 as usize] = 0.0;
		}
		self.fTempPerm11 = 0.0;
		self.fTempPerm12 = 0.0;
		self.iTempPerm13 = 0;
		self.fTempPerm14 = 0.0;
		self.iTempPerm15 = 0;
		self.fTempPerm16 = 0.0;
		for l10 in 0..2 {
			self.fRec8[l10 as usize] = 0.0;
		}
		self.fTempPerm17 = 0.0;
		for l11 in 0..2 {
			self.fRec9[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.iRec11[l12 as usize] = 0;
		}
		self.iTempPerm18 = 0;
		for l13 in 0..2 {
			self.fRec10[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec12[l14 as usize] = 0.0;
		}
		self.fTempPerm19 = 0.0;
		for l15 in 0..2 {
			self.fRec15[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.iRec16[l16 as usize] = 0;
		}
		self.fTempPerm20 = 0.0;
		for l17 in 0..2 {
			self.fRec14[l17 as usize] = 0.0;
		}
		self.fTempPerm21 = 0.0;
		for l18 in 0..2 {
			self.fVec3[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec13[l19 as usize] = 0.0;
		}
		self.fTempPerm22 = 0.0;
		self.fTempPerm23 = 0.0;
		for l20 in 0..2 {
			self.fRec17[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec18[l21 as usize] = 0.0;
		}
		self.fTempPerm24 = 0.0;
		self.iTempPerm25 = 0;
		self.fTempPerm26 = 0.0;
		for l22 in 0..2 {
			self.fRec19[l22 as usize] = 0.0;
		}
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		self.fTempPerm29 = 0.0;
		for l23 in 0..2 {
			self.fRec20[l23 as usize] = 0.0;
		}
		self.fTempPerm30 = 0.0;
		self.fTempPerm31 = 0.0;
		self.fTempPerm32 = 0.0;
		self.fTempPerm33 = 0.0;
		for l24 in 0..2 {
			self.fRec0[l24 as usize] = 0.0;
		}
		self.iTempPerm34 = 0;
		for l25 in 0..2 {
			self.fRec21[l25 as usize] = 0.0;
		}
		self.fTempPerm35 = 0.0;
		self.fTempPerm36 = 0.0;
		self.fTempPerm37 = 0.0;
		for l26 in 0..2 {
			self.fRec22[l26 as usize] = 0.0;
		}
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		self.fTempPerm44 = 0.0;
		for l27 in 0..2 {
			self.iRec23[l27 as usize] = 0;
		}
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		self.iTempPerm53 = 0;
		self.iTempPerm54 = 0;
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
		for l28 in 0..2 {
			self.fRec24[l28 as usize] = 0.0;
		}
		self.fTempPerm66 = 0.0;
		self.iTempPerm67 = 0;
		self.fTempPerm68 = 0.0;
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		self.fTempPerm71 = 0.0;
		for l29 in 0..2 {
			self.fRec25[l29 as usize] = 0.0;
		}
		self.fTempPerm72 = 0.0;
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		self.fTempPerm75 = 0.0;
		for l30 in 0..2 {
			self.fRec26[l30 as usize] = 0.0;
		}
		self.fTempPerm76 = 0.0;
		self.fTempPerm77 = 0.0;
		for l31 in 0..2 {
			self.fVec4[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec27[l32 as usize] = 0.0;
		}
		self.fTempPerm78 = 0.0;
		self.iTempPerm79 = 0;
		self.fTempPerm80 = 0.0;
		self.fTempPerm81 = 0.0;
		self.iTempPerm82 = 0;
		self.fTempPerm83 = 0.0;
		self.fTempPerm84 = 0.0;
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		for l33 in 0..2 {
			self.fRec28[l33 as usize] = 0.0;
		}
		self.fTempPerm87 = 0.0;
		self.fTempPerm88 = 0.0;
		self.fTempPerm89 = 0.0;
		self.fTempPerm90 = 0.0;
		for l34 in 0..2 {
			self.fRec29[l34 as usize] = 0.0;
		}
		self.fTempPerm91 = 0.0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		for l35 in 0..2 {
			self.fRec30[l35 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		self.fTempPerm96 = 0.0;
		for l36 in 0..2 {
			self.fRec31[l36 as usize] = 0.0;
		}
		self.fTempPerm97 = 0.0;
		self.fTempPerm98 = 0.0;
		for l37 in 0..2 {
			self.fRec32[l37 as usize] = 0.0;
		}
		self.fTempPerm99 = 0.0;
		self.fTempPerm100 = 0.0;
		for l38 in 0..2 {
			self.fRec33[l38 as usize] = 0.0;
		}
		self.fTempPerm101 = 0.0;
		self.fTempPerm102 = 0.0;
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		for l39 in 0..2 {
			self.iRec34[l39 as usize] = 0;
		}
		self.fTempPerm105 = 0.0;
		self.iTempPerm106 = 0;
		self.fTempPerm107 = 0.0;
		self.fTempPerm108 = 0.0;
		self.iTempPerm109 = 0;
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		self.fTempPerm112 = 0.0;
		self.fTempPerm113 = 0.0;
		self.fTempPerm114 = 0.0;
		self.fTempPerm115 = 0.0;
		self.fTempPerm116 = 0.0;
		self.iTempPerm117 = 0;
		self.iTempPerm118 = 0;
		self.fTempPerm119 = 0.0;
		self.iTempPerm120 = 0;
		self.iTempPerm121 = 0;
		self.iTempPerm122 = 0;
		self.iTempPerm123 = 0;
		self.iTempPerm124 = 0;
		self.iTempPerm125 = 0;
		self.fTempPerm126 = 0.0;
		for l40 in 0..2 {
			self.fRec35[l40 as usize] = 0.0;
		}
		self.fTempPerm127 = 0.0;
		self.iTempPerm128 = 0;
		self.iTempPerm129 = 0;
		self.iTempPerm130 = 0;
		self.iTempPerm131 = 0;
		self.fTempPerm132 = 0.0;
		self.fTempPerm133 = 0.0;
		self.iTempPerm134 = 0;
		self.fTempPerm135 = 0.0;
		self.fTempPerm136 = 0.0;
		self.fTempPerm137 = 0.0;
		self.iTempPerm138 = 0;
		self.fTempPerm139 = 0.0;
		self.fTempPerm140 = 0.0;
		for l41 in 0..2 {
			self.fRec36[l41 as usize] = 0.0;
		}
		self.fTempPerm141 = 0.0;
		self.fTempPerm142 = 0.0;
		self.fTempPerm143 = 0.0;
		self.fTempPerm144 = 0.0;
		self.fTempPerm145 = 0.0;
		self.iTempPerm146 = 0;
		self.iTempPerm147 = 0;
		self.iTempPerm148 = 0;
		self.iTempPerm149 = 0;
		self.iTempPerm150 = 0;
		self.iTempPerm151 = 0;
		self.iTempPerm152 = 0;
		self.fTempPerm153 = 0.0;
		self.fTempPerm154 = 0.0;
		self.iTempPerm155 = 0;
		self.iTempPerm156 = 0;
		self.iTempPerm157 = 0;
		self.fTempPerm158 = 0.0;
		self.fTempPerm159 = 0.0;
		self.iTempPerm160 = 0;
		self.fTempPerm161 = 0.0;
		self.fTempPerm162 = 0.0;
		self.fTempPerm163 = 0.0;
		self.fTempPerm164 = 0.0;
		self.fTempPerm165 = 0.0;
		self.iTempPerm166 = 0;
		self.iTempPerm167 = 0;
		self.iTempPerm168 = 0;
		self.iTempPerm169 = 0;
		self.iTempPerm170 = 0;
		self.iTempPerm171 = 0;
		self.fTempPerm172 = 0.0;
		self.iTempPerm173 = 0;
		self.iTempPerm174 = 0;
		self.iTempPerm175 = 0;
		self.fTempPerm176 = 0.0;
		self.fTempPerm177 = 0.0;
		self.iTempPerm178 = 0;
		self.fTempPerm179 = 0.0;
		self.fTempPerm180 = 0.0;
		self.fTempPerm181 = 0.0;
		self.fTempPerm182 = 0.0;
		self.fTempPerm183 = 0.0;
		self.fTempPerm184 = 0.0;
		self.iTempPerm185 = 0;
		self.iTempPerm186 = 0;
		self.fTempPerm187 = 0.0;
		self.fTempPerm188 = 0.0;
		self.iTempPerm189 = 0;
		self.fTempPerm190 = 0.0;
		self.fTempPerm191 = 0.0;
		self.fTempPerm192 = 0.0;
		for l42 in 0..2 {
			self.fRec38[l42 as usize] = 0.0;
		}
		self.fTempPerm193 = 0.0;
		self.iTempPerm194 = 0;
		self.iTempPerm195 = 0;
		self.fTempPerm196 = 0.0;
		self.fTempPerm197 = 0.0;
		self.iTempPerm198 = 0;
		self.fTempPerm199 = 0.0;
		for l43 in 0..2 {
			self.fRec37[l43 as usize] = 0.0;
		}
		self.fTempPerm200 = 0.0;
		self.fTempPerm201 = 0.0;
		self.fTempPerm202 = 0.0;
		self.fTempPerm203 = 0.0;
		self.fTempPerm204 = 0.0;
		self.fTempPerm205 = 0.0;
		for l44 in 0..3 {
			self.fRec39[l44 as usize] = 0.0;
		}
		self.fTempPerm206 = 0.0;
		self.fTempPerm207 = 0.0;
		self.fTempPerm208 = 0.0;
		self.fTempPerm209 = 0.0;
		self.fTempPerm210 = 0.0;
		self.fTempPerm211 = 0.0;
		self.fTempPerm212 = 0.0;
		self.fTempPerm213 = 0.0;
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 - 44.1 / self.fConst0;
		self.fConst2 = 6.5 / self.fConst0;
		self.fConst3 = 1.0 / self.fConst0;
		self.fConst4 = F32::exp(-(376.99112 / self.fConst0));
		let mut fConst5: F32 = 1.0 - self.fConst4;
		self.fConst6 = 4.656613e-10 * F32::sqrt(3.0 * ((self.fConst4 + 1.0) / fConst5));
		self.fConst7 = 3.0 * fConst5;
		self.fConst8 = 3.1415927 / self.fConst0;
		self.fConst9 = 5.5 / self.fConst0;
		let mut fConst10: F32 = 0.45 * self.fConst0;
		self.fConst11 = F32::min(2e+04, fConst10);
		self.fConst12 = 64.0 * self.fConst11;
		self.fConst13 = 32.0 * self.fConst11;
		self.fConst14 = 16.0 * self.fConst11;
		self.fConst15 = 8.0 * self.fConst11;
		self.fConst16 = 4.0 * self.fConst11;
		self.fConst17 = 2.0 * self.fConst11;
		let mut fConst18: F32 = F32::min(2.2e+04, fConst10);
		self.fConst19 = 1.0 / fConst18;
		self.fConst20 = 3.1415927 * (fConst18 / self.fConst0);
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
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.add_horizontal_slider("Amount Env", ParamIndex(1), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "1", "");
		ui_interface.add_horizontal_slider("Amount LFO", ParamIndex(2), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "0", "");
		ui_interface.open_horizontal_box("Master");
		ui_interface.declare(Some(ParamIndex(3)), "2", "");
		ui_interface.add_horizontal_slider("Portamento Env", ParamIndex(3), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "2", "");
		ui_interface.add_horizontal_slider("Portamento LFO", ParamIndex(4), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1.5", "");
		ui_interface.open_horizontal_box("Unison");
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.declare(Some(ParamIndex(5)), "style", "menu{'Off':0;'U5':1;'U7':2}");
		ui_interface.add_num_entry("Mode", ParamIndex(5), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "2", "");
		ui_interface.add_horizontal_slider("Detune Env", ParamIndex(6), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "2", "");
		ui_interface.add_horizontal_slider("Detune LFO", ParamIndex(7), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "2", "");
		ui_interface.declare(Some(ParamIndex(8)), "skew", "0.4");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "cents");
		ui_interface.add_horizontal_slider("Detune", ParamIndex(8), 6.0, 0.0, 5e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(9)), "3", "");
		ui_interface.add_check_button("Random Phase", ParamIndex(9));
		ui_interface.close_box();
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Perform");
		ui_interface.declare(Some(ParamIndex(10)), "1", "");
		ui_interface.declare(Some(ParamIndex(10)), "style", "menu{'None':0;'Cutoff':1;'Resonance':2;'Morph':3;'Bandpass':4;'Filter Env Amount':5;'Drive':6;'Dual A':7;'Dual B':8;'Dual Pitch':9;'Mod Env Amount':10;'Mod LFO Amount':11;'WT Position':12;'WT LFO Rate':13;'Pulse Width':14;'PWM Depth':15;'Unison Detune':16;'Noise Level':17;'Noise Tilt':18;'Dual Level':19;'WT Level':20;'Vibrato':21;'Tremolo':22;'Level':23;'OscMod Depth':24}");
		ui_interface.add_num_entry("MW Dest", ParamIndex(10), 21.0, 0.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(11)), "2", "");
		ui_interface.declare(Some(ParamIndex(11)), "unit", "%");
		ui_interface.add_horizontal_slider("MW Depth", ParamIndex(11), 5e+01, -1e+02, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(12)), "3", "");
		ui_interface.declare(Some(ParamIndex(12)), "style", "menu{'None':0;'Cutoff':1;'Resonance':2;'Morph':3;'Bandpass':4;'Filter Env Amount':5;'Drive':6;'Dual A':7;'Dual B':8;'Dual Pitch':9;'Mod Env Amount':10;'Mod LFO Amount':11;'WT Position':12;'WT LFO Rate':13;'Pulse Width':14;'PWM Depth':15;'Unison Detune':16;'Noise Level':17;'Noise Tilt':18;'Dual Level':19;'WT Level':20;'Vibrato':21;'Tremolo':22;'Level':23;'OscMod Depth':24}");
		ui_interface.add_num_entry("AT Dest", ParamIndex(12), 1.0, 0.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(13)), "4", "");
		ui_interface.declare(Some(ParamIndex(13)), "unit", "%");
		ui_interface.add_horizontal_slider("AT Depth", ParamIndex(13), 5e+01, -1e+02, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(14)), "5", "");
		ui_interface.declare(Some(ParamIndex(14)), "style", "menu{'2 Semitones':0;'Octave':1;'Cutoff 50%':2;'Resonance 50%':3;'Bandpass 50%':4}");
		ui_interface.add_num_entry("Bend Mode", ParamIndex(14), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "13", "");
		ui_interface.open_horizontal_box("Settings");
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.add_horizontal_slider("Vintage Noise", ParamIndex(15), 1.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("Dual");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.add_num_entry("Octave", ParamIndex(16), 0.0, -2.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(17)), "0", "");
		ui_interface.declare(Some(ParamIndex(17)), "style", "menu{'sin/sin':0;'tri/tri':1;'saw/saw':2;'sqr/sqr':3;'sqr/tri':4;'sqr/sin':5;'saw/sqr':6;'saw/tri':7;'saw/sin':8}");
		ui_interface.add_num_entry("Waves", ParamIndex(17), 1.0, 0.0, 8.0, 1.0);
		ui_interface.declare(Some(ParamIndex(18)), "1", "");
		ui_interface.declare(Some(ParamIndex(18)), "style", "menu{'Ring Mod':0;'FM':1;'Window Amp Sync':2;'Fractal':3;'Sync':4;'Spread':5;'PWM':6;'Folding':7}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(18), 0.0, 0.0, 7.0, 1.0);
		ui_interface.declare(Some(ParamIndex(19)), "2", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(19), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "2", "");
		ui_interface.add_horizontal_slider("A Env", ParamIndex(20), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "2", "");
		ui_interface.add_horizontal_slider("A LFO", ParamIndex(21), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(22)), "3", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(22), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(23)), "3", "");
		ui_interface.add_horizontal_slider("B Env", ParamIndex(23), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "3", "");
		ui_interface.add_horizontal_slider("B LFO", ParamIndex(24), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(25)), "4", "");
		ui_interface.add_num_entry("Semi", ParamIndex(25), 0.0, -12.0, 12.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("Wavetable");
		ui_interface.declare(Some(ParamIndex(26)), "0", "");
		ui_interface.add_num_entry("Octave", ParamIndex(26), 0.0, -2.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(27)), "1", "");
		ui_interface.declare(Some(ParamIndex(27)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Start", ParamIndex(27), 0.0, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(28)), "2", "");
		ui_interface.declare(Some(ParamIndex(28)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Mid", ParamIndex(28), 4e+01, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(29)), "3", "");
		ui_interface.declare(Some(ParamIndex(29)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("End", ParamIndex(29), 8e+01, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(30)), "4", "");
		ui_interface.add_horizontal_slider("LFO Rate Env", ParamIndex(30), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "4", "");
		ui_interface.add_horizontal_slider("LFO Rate LFO", ParamIndex(31), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(32)), "4", "");
		ui_interface.declare(Some(ParamIndex(32)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(32)), "unit", "s");
		ui_interface.add_horizontal_slider("LFO Rate", ParamIndex(32), 2.0, 0.1, 1e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(33)), "5", "");
		ui_interface.declare(Some(ParamIndex(33)), "style", "menu{'Off':0;'Bits 4':1;'SR 11k':2;'SR 8k':3;'SR 4k':4;'Simplify 6k':5;'Simplify 1.5k':6;'Simplify 800':7;'Rectify':8;'Tri Fold':9;'Sine Fold':10;'Squish':11;'Ring':12;'Mirror':13;'Sign Swap':14}");
		ui_interface.add_num_entry("OscMod", ParamIndex(33), 0.0, 0.0, 14.0, 1.0);
		ui_interface.declare(Some(ParamIndex(34)), "6", "");
		ui_interface.add_horizontal_slider("OscMod Depth", ParamIndex(34), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(35)), "6", "");
		ui_interface.add_horizontal_slider("OscMod Depth Env", ParamIndex(35), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(36)), "6", "");
		ui_interface.add_horizontal_slider("OscMod Depth LFO", ParamIndex(36), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(37)), "2", "");
		ui_interface.add_vertical_slider("Dual", ParamIndex(37), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(38)), "3", "");
		ui_interface.add_vertical_slider("Wavetable", ParamIndex(38), 0.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("Envelope");
		ui_interface.declare(Some(ParamIndex(39)), "1", "");
		ui_interface.declare(Some(ParamIndex(39)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(39)), "unit", "s");
		ui_interface.add_vertical_slider("Attack", ParamIndex(39), 0.005, 0.005, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(40)), "2", "");
		ui_interface.declare(Some(ParamIndex(40)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(40)), "unit", "s");
		ui_interface.add_vertical_slider("Decay", ParamIndex(40), 0.4, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(41)), "5", "");
		ui_interface.add_vertical_slider("Curve", ParamIndex(41), 0.5, -1.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(42)), "hidden", "1");
		ui_interface.add_horizontal_slider("aftertouch", ParamIndex(42), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(43)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(43)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(43), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("copy", ParamIndex(44), 3.0, 0.0, 6.0, 1.0);
		ui_interface.add_horizontal_slider("freq", ParamIndex(45), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_button("gate", ParamIndex(46));
		ui_interface.add_horizontal_slider("glide", ParamIndex(47), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.declare(Some(ParamIndex(48)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(48)), "midi", "ctrl 1");
		ui_interface.add_horizontal_slider("modwheel", ParamIndex(48), 0.0, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("seed", ParamIndex(49), 0.0, 0.0, 1.0, 1e-06);
		ui_interface.add_horizontal_slider("slot", ParamIndex(50), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			46 => Some(self.fButton0),
			9 => Some(self.fCheckbox0),
			18 => Some(self.fEntry0),
			33 => Some(self.fEntry1),
			29 => Some(self.fEntry10),
			26 => Some(self.fEntry11),
			27 => Some(self.fEntry12),
			12 => Some(self.fEntry2),
			10 => Some(self.fEntry3),
			25 => Some(self.fEntry4),
			16 => Some(self.fEntry5),
			5 => Some(self.fEntry6),
			14 => Some(self.fEntry7),
			17 => Some(self.fEntry8),
			28 => Some(self.fEntry9),
			42 => Some(self.fHslider0),
			13 => Some(self.fHslider1),
			0 => Some(self.fHslider10),
			50 => Some(self.fHslider11),
			15 => Some(self.fHslider12),
			43 => Some(self.fHslider13),
			47 => Some(self.fHslider14),
			4 => Some(self.fHslider15),
			3 => Some(self.fHslider16),
			45 => Some(self.fHslider17),
			49 => Some(self.fHslider18),
			24 => Some(self.fHslider19),
			48 => Some(self.fHslider2),
			23 => Some(self.fHslider20),
			22 => Some(self.fHslider21),
			21 => Some(self.fHslider22),
			20 => Some(self.fHslider23),
			19 => Some(self.fHslider24),
			31 => Some(self.fHslider25),
			30 => Some(self.fHslider26),
			32 => Some(self.fHslider27),
			36 => Some(self.fHslider28),
			35 => Some(self.fHslider29),
			11 => Some(self.fHslider3),
			34 => Some(self.fHslider30),
			7 => Some(self.fHslider4),
			6 => Some(self.fHslider5),
			8 => Some(self.fHslider6),
			44 => Some(self.fHslider7),
			2 => Some(self.fHslider8),
			1 => Some(self.fHslider9),
			37 => Some(self.fVslider0),
			38 => Some(self.fVslider1),
			39 => Some(self.fVslider2),
			40 => Some(self.fVslider3),
			41 => Some(self.fVslider4),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			46 => { self.fButton0 = value }
			9 => { self.fCheckbox0 = value }
			18 => { self.fEntry0 = value }
			33 => { self.fEntry1 = value }
			29 => { self.fEntry10 = value }
			26 => { self.fEntry11 = value }
			27 => { self.fEntry12 = value }
			12 => { self.fEntry2 = value }
			10 => { self.fEntry3 = value }
			25 => { self.fEntry4 = value }
			16 => { self.fEntry5 = value }
			5 => { self.fEntry6 = value }
			14 => { self.fEntry7 = value }
			17 => { self.fEntry8 = value }
			28 => { self.fEntry9 = value }
			42 => { self.fHslider0 = value }
			13 => { self.fHslider1 = value }
			0 => { self.fHslider10 = value }
			50 => { self.fHslider11 = value }
			15 => { self.fHslider12 = value }
			43 => { self.fHslider13 = value }
			47 => { self.fHslider14 = value }
			4 => { self.fHslider15 = value }
			3 => { self.fHslider16 = value }
			45 => { self.fHslider17 = value }
			49 => { self.fHslider18 = value }
			24 => { self.fHslider19 = value }
			48 => { self.fHslider2 = value }
			23 => { self.fHslider20 = value }
			22 => { self.fHslider21 = value }
			21 => { self.fHslider22 = value }
			20 => { self.fHslider23 = value }
			19 => { self.fHslider24 = value }
			31 => { self.fHslider25 = value }
			30 => { self.fHslider26 = value }
			32 => { self.fHslider27 = value }
			36 => { self.fHslider28 = value }
			35 => { self.fHslider29 = value }
			11 => { self.fHslider3 = value }
			34 => { self.fHslider30 = value }
			7 => { self.fHslider4 = value }
			6 => { self.fHslider5 = value }
			8 => { self.fHslider6 = value }
			44 => { self.fHslider7 = value }
			2 => { self.fHslider8 = value }
			1 => { self.fHslider9 = value }
			37 => { self.fVslider0 = value }
			38 => { self.fVslider1 = value }
			39 => { self.fVslider2 = value }
			40 => { self.fVslider3 = value }
			41 => { self.fVslider4 = value }
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
		let mut iSlow0: i32 = (self.fVslider0 != 0.0) as i32;
		let mut fSlow1: F32 = (iSlow0) as F32;
		let mut iSlow2: i32 = (fSlow1 != 0.0) as i32;
		let mut fSlow3: F32 = self.fEntry0;
		let mut iSlow4: i32 = (fSlow3 == 6.0) as i32;
		let mut fSlow5: F32 = (iSlow4) as F32;
		let mut iSlow6: i32 = (fSlow5 != 0.0) as i32;
		let mut iSlow7: i32 = (fSlow3 == 4.0) as i32;
		let mut fSlow8: F32 = (iSlow7) as F32;
		let mut iSlow9: i32 = (fSlow8 != 0.0) as i32;
		let mut iSlow10: i32 = (fSlow3 == 3.0) as i32;
		let mut fSlow11: F32 = (iSlow10) as F32;
		let mut iSlow12: i32 = (fSlow11 != 0.0) as i32;
		let mut iSlow13: i32 = (fSlow3 == 5.0) as i32;
		let mut fSlow14: F32 = (iSlow13) as F32;
		let mut iSlow15: i32 = (fSlow14 != 0.0) as i32;
		let mut iSlow16: i32 = (fSlow3 == 0.0) as i32;
		let mut fSlow17: F32 = (iSlow16) as F32;
		let mut iSlow18: i32 = (fSlow17 != 0.0) as i32;
		let mut iSlow19: i32 = (fSlow3 == 2.0) as i32;
		let mut fSlow20: F32 = (iSlow19) as F32;
		let mut iSlow21: i32 = (fSlow20 != 0.0) as i32;
		let mut iSlow22: i32 = (fSlow3 == 7.0) as i32;
		let mut fSlow23: F32 = (iSlow22) as F32;
		let mut iSlow24: i32 = (fSlow23 != 0.0) as i32;
		let mut iSlow25: i32 = (fSlow3 == 1.0) as i32;
		let mut fSlow26: F32 = (iSlow25) as F32;
		let mut iSlow27: i32 = (fSlow26 != 0.0) as i32;
		let mut fSlow28: F32 = self.fButton0;
		let mut fSlow29: F32 = self.fEntry1;
		let mut iSlow30: i32 = (fSlow29 == 14.0) as i32;
		let mut fSlow31: F32 = (iSlow30) as F32;
		let mut iSlow32: i32 = (fSlow31 != 0.0) as i32;
		let mut iSlow33: i32 = (self.fVslider1 != 0.0) as i32;
		let mut fSlow34: F32 = (iSlow33) as F32;
		let mut iSlow35: i32 = (fSlow34 != 0.0) as i32;
		let mut iSlow36: i32 = (fSlow29 == 13.0) as i32;
		let mut fSlow37: F32 = (iSlow36) as F32;
		let mut iSlow38: i32 = (fSlow37 != 0.0) as i32;
		let mut iSlow39: i32 = (fSlow29 == 12.0) as i32;
		let mut fSlow40: F32 = (iSlow39) as F32;
		let mut iSlow41: i32 = (fSlow40 != 0.0) as i32;
		let mut iSlow42: i32 = (fSlow29 == 11.0) as i32;
		let mut fSlow43: F32 = (iSlow42) as F32;
		let mut iSlow44: i32 = (fSlow43 != 0.0) as i32;
		let mut iSlow45: i32 = (fSlow29 == 1e+01) as i32;
		let mut fSlow46: F32 = (iSlow45) as F32;
		let mut iSlow47: i32 = (fSlow46 != 0.0) as i32;
		let mut iSlow48: i32 = (fSlow29 == 9.0) as i32;
		let mut fSlow49: F32 = (iSlow48) as F32;
		let mut iSlow50: i32 = (fSlow49 != 0.0) as i32;
		let mut iSlow51: i32 = (fSlow29 == 8.0) as i32;
		let mut fSlow52: F32 = (iSlow51) as F32;
		let mut iSlow53: i32 = (fSlow52 != 0.0) as i32;
		let mut iSlow54: i32 = (fSlow29 == 3.0) as i32;
		let mut fSlow55: F32 = (iSlow54) as F32;
		let mut iSlow56: i32 = (fSlow55 != 0.0) as i32;
		let mut iSlow57: i32 = (fSlow29 == 2.0) as i32;
		let mut fSlow58: F32 = (iSlow57) as F32;
		let mut iSlow59: i32 = (fSlow58 != 0.0) as i32;
		let mut iSlow60: i32 = (fSlow29 == 4.0) as i32;
		let mut fSlow61: F32 = (iSlow60) as F32;
		let mut iSlow62: i32 = (fSlow61 != 0.0) as i32;
		let mut iSlow63: i32 = (fSlow29 == 5.0) as i32;
		let mut fSlow64: F32 = (iSlow63) as F32;
		let mut iSlow65: i32 = (fSlow64 != 0.0) as i32;
		let mut iSlow66: i32 = (fSlow29 == 6.0) as i32;
		let mut fSlow67: F32 = (iSlow66) as F32;
		let mut iSlow68: i32 = (fSlow67 != 0.0) as i32;
		let mut iSlow69: i32 = (fSlow29 == 7.0) as i32;
		let mut fSlow70: F32 = (iSlow69) as F32;
		let mut iSlow71: i32 = (fSlow70 != 0.0) as i32;
		let mut iSlow72: i32 = (fSlow29 == 0.0) as i32;
		let mut fSlow73: F32 = (iSlow72) as F32;
		let mut iSlow74: i32 = (fSlow73 != 0.0) as i32;
		let mut iSlow75: i32 = (fSlow29 == 1.0) as i32;
		let mut fSlow76: F32 = (iSlow75) as F32;
		let mut iSlow77: i32 = (fSlow76 != 0.0) as i32;
		let mut iTemp0: i32 = self.iTempPerm0;
		let mut iTemp1: i32 = self.iTempPerm1;
		let mut fTemp2: F32 = self.fTempPerm2;
		let mut fTemp3: F32 = self.fTempPerm3;
		let mut fSlow78: F32 = self.fHslider0;
		let mut fSlow79: F32 = 0.01 * self.fHslider1;
		let mut fTemp4: F32 = self.fTempPerm4;
		let mut fSlow80: F32 = self.fEntry2;
		let mut fSlow81: F32 = ((fSlow80 == 9.0) as i32) as u32 as F32;
		let mut fSlow82: F32 = self.fHslider2;
		let mut fSlow83: F32 = 0.01 * self.fHslider3;
		let mut fTemp5: F32 = self.fTempPerm5;
		let mut fSlow84: F32 = self.fEntry3;
		let mut fSlow85: F32 = ((fSlow84 == 9.0) as i32) as u32 as F32;
		let mut fSlow86: F32 = self.fEntry4;
		let mut fSlow87: F32 = self.fEntry5;
		let mut fSlow88: F32 = ((fSlow80 == 16.0) as i32) as u32 as F32;
		let mut fSlow89: F32 = ((fSlow84 == 16.0) as i32) as u32 as F32;
		let mut fSlow90: F32 = self.fHslider4;
		let mut fSlow91: F32 = self.fHslider5;
		let mut iSlow92: i32 = ((fSlow91 != 0.0) as i32) | ((fSlow90 != 0.0) as i32);
		let mut fSlow93: F32 = self.fHslider6;
		let mut fSlow94: F32 = (iSlow92) as F32;
		let mut iSlow95: i32 = (fSlow94 != 0.0) as i32;
		let mut iSlow96: i32 = (fSlow80 == 1e+01) as i32;
		let mut fTemp6: F32 = self.fTempPerm6;
		let mut iSlow97: i32 = (fSlow84 == 1e+01) as i32;
		let mut fTemp7: F32 = self.fTempPerm7;
		let mut fTemp8: F32 = self.fTempPerm8;
		let mut iSlow98: i32 = (fSlow80 == 11.0) as i32;
		let mut iSlow99: i32 = (fSlow84 == 11.0) as i32;
		let mut fTemp9: F32 = self.fTempPerm9;
		let mut fSlow100: F32 = 5e+01 * fSlow94;
		let mut fTemp10: F32 = self.fTempPerm10;
		let mut fSlow101: F32 = ((iSlow92 != 0) as i32) as u32 as F32;
		let mut fSlow102: F32 = self.fHslider7;
		let mut iSlow103: i32 = (fSlow102) as i32;
		let mut fSlow104: F32 = unsafe { ftbl0OscDspSIG0[iSlow103 as usize] };
		let mut fSlow105: F32 = self.fEntry6;
		let mut iSlow106: i32 = (fSlow105 == 1.0) as i32;
		let mut fSlow107: F32 = (iSlow106) as F32;
		let mut fSlow108: F32 = fSlow104 + fSlow107 * (unsafe { ftbl1OscDspSIG1[iSlow103 as usize] } - fSlow104);
		let mut fSlow109: F32 = self.fHslider8;
		let mut fSlow110: F32 = self.fHslider9;
		let mut iSlow111: i32 = ((fSlow110 != 0.0) as i32) | ((fSlow109 != 0.0) as i32);
		let mut fSlow112: F32 = self.fHslider10;
		let mut fSlow113: F32 = (iSlow111) as F32;
		let mut iSlow114: i32 = (fSlow113 != 0.0) as i32;
		let mut fTemp11: F32 = self.fTempPerm11;
		let mut fSlow115: F32 = ((iSlow111 != 0) as i32) as u32 as F32;
		let mut fTemp12: F32 = self.fTempPerm12;
		let mut iTemp13: i32 = self.iTempPerm13;
		let mut fTemp14: F32 = self.fTempPerm14;
		let mut iTemp15: i32 = self.iTempPerm15;
		let mut fTemp16: F32 = self.fTempPerm16;
		let mut fSlow116: F32 = self.fHslider11;
		let mut fSlow117: F32 = 12.9898 * fSlow116;
		let mut fSlow118: F32 = 43758.547 * F32::sin(fSlow117 + 234.699);
		let mut fSlow119: F32 = self.fConst3 * (0.1 * (2.0 * (fSlow118 - F32::floor(fSlow118)) + -1.0) + 0.45);
		let mut fTemp17: F32 = self.fTempPerm17;
		let mut iSlow120: i32 = (409.0 * fSlow116 + 4e+03) as i32;
		let mut iTemp18: i32 = self.iTempPerm18;
		let mut fSlow121: F32 = self.fHslider12;
		let mut fSlow122: F32 = 43758.547 * F32::sin(fSlow117 + 156.466);
		let mut fSlow123: F32 = 0.15 * (2.0 * (fSlow122 - F32::floor(fSlow122)) + -1.0) + 0.25;
		let mut fSlow124: F32 = 1.0 / F32::tan(self.fConst8 * F32::max(1.0, fSlow123));
		let mut fSlow125: F32 = 1.0 - fSlow124;
		let mut fSlow126: F32 = self.fConst3 * F32::max(fSlow123, 0.0);
		let mut fTemp19: F32 = self.fTempPerm19;
		let mut fTemp20: F32 = self.fTempPerm20;
		let mut fTemp21: F32 = self.fTempPerm21;
		let mut fSlow127: F32 = 1.0 / (fSlow124 + 1.0);
		let mut fSlow128: F32 = 43758.547 * F32::sin(fSlow117 + 78.233);
		let mut fSlow129: F32 = 8.0 * (2.0 * (fSlow128 - F32::floor(fSlow128)) + -1.0);
		let mut fTemp22: F32 = self.fTempPerm22;
		let mut fTemp23: F32 = self.fTempPerm23;
		let mut fSlow130: F32 = ((fSlow80 == 21.0) as i32) as u32 as F32;
		let mut fSlow131: F32 = ((fSlow84 == 21.0) as i32) as u32 as F32;
		let mut fSlow132: F32 = self.fHslider13;
		let mut fSlow133: F32 = self.fEntry7;
		let mut fSlow134: F32 = 2.0 * ((fSlow133 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow133 == 1.0) as i32) as u32 as F32;
		let mut fSlow135: F32 = self.fHslider14;
		let mut iSlow136: i32 = (fSlow135 > 0.0) as i32;
		let mut fSlow137: F32 = self.fHslider15;
		let mut fSlow138: F32 = self.fHslider16;
		let mut fSlow139: F32 = 0.0037037036 * (1e+03 * fSlow135 + -3e+01);
		let mut fTemp24: F32 = self.fTempPerm24;
		let mut iTemp25: i32 = self.iTempPerm25;
		let mut fTemp26: F32 = self.fTempPerm26;
		let mut fSlow140: F32 = F32::log(self.fHslider17, std::f32::consts::E);
		let mut fTemp27: F32 = self.fTempPerm27;
		let mut fTemp28: F32 = self.fTempPerm28;
		let mut fTemp29: F32 = self.fTempPerm29;
		let mut fTemp30: F32 = self.fTempPerm30;
		let mut fSlow141: F32 = 0.618 * fSlow102 + 7.31 * self.fHslider18 * (fSlow102 + 1.0);
		let mut fTemp31: F32 = self.fTempPerm31;
		let mut fSlow142: F32 = self.fCheckbox0;
		let mut fTemp32: F32 = self.fTempPerm32;
		let mut fTemp33: F32 = self.fTempPerm33;
		let mut iTemp34: i32 = self.iTempPerm34;
		let mut fSlow143: F32 = ((fSlow84 == 8.0) as i32) as u32 as F32;
		let mut fSlow144: F32 = self.fHslider19;
		let mut fSlow145: F32 = self.fHslider20;
		let mut iSlow146: i32 = ((fSlow145 != 0.0) as i32) | ((fSlow144 != 0.0) as i32);
		let mut fSlow147: F32 = self.fHslider21;
		let mut fSlow148: F32 = (iSlow146) as F32;
		let mut iSlow149: i32 = (fSlow148 != 0.0) as i32;
		let mut fTemp35: F32 = self.fTempPerm35;
		let mut fSlow150: F32 = ((iSlow146 != 0) as i32) as u32 as F32;
		let mut fSlow151: F32 = ((fSlow80 == 8.0) as i32) as u32 as F32;
		let mut fTemp36: F32 = self.fTempPerm36;
		let mut fTemp37: F32 = self.fTempPerm37;
		let mut fSlow152: F32 = ((fSlow84 == 7.0) as i32) as u32 as F32;
		let mut fSlow153: F32 = self.fHslider22;
		let mut fSlow154: F32 = self.fHslider23;
		let mut iSlow155: i32 = ((fSlow154 != 0.0) as i32) | ((fSlow153 != 0.0) as i32);
		let mut fSlow156: F32 = self.fHslider24;
		let mut fSlow157: F32 = (iSlow155) as F32;
		let mut iSlow158: i32 = (fSlow157 != 0.0) as i32;
		let mut fTemp38: F32 = self.fTempPerm38;
		let mut fSlow159: F32 = ((iSlow155 != 0) as i32) as u32 as F32;
		let mut fSlow160: F32 = ((fSlow80 == 7.0) as i32) as u32 as F32;
		let mut fTemp39: F32 = self.fTempPerm39;
		let mut fTemp40: F32 = self.fTempPerm40;
		let mut fTemp41: F32 = self.fTempPerm41;
		let mut fTemp42: F32 = self.fTempPerm42;
		let mut iSlow161: i32 = (self.fEntry8) as i32;
		let mut iSlow162: i32 = unsafe { itbl3OscDspSIG3[iSlow161 as usize] };
		let mut iSlow163: i32 = unsafe { itbl4OscDspSIG4[iSlow161 as usize] };
		let mut fTemp43: F32 = self.fTempPerm43;
		let mut fTemp44: F32 = self.fTempPerm44;
		let mut fSlow164: F32 = ((iSlow4 != 0) as i32) as u32 as F32;
		let mut iSlow165: i32 = if iSlow7 != 0 {iSlow162} else {-1};
		let mut iSlow166: i32 = if iSlow7 != 0 {iSlow163} else {-1};
		let mut fTemp45: F32 = self.fTempPerm45;
		let mut fTemp46: F32 = self.fTempPerm46;
		let mut fTemp47: F32 = self.fTempPerm47;
		let mut fTemp48: F32 = self.fTempPerm48;
		let mut fSlow167: F32 = ((iSlow7 != 0) as i32) as u32 as F32;
		let mut fTemp49: F32 = self.fTempPerm49;
		let mut fTemp50: F32 = self.fTempPerm50;
		let mut fTemp51: F32 = self.fTempPerm51;
		let mut fSlow168: F32 = ((iSlow10 != 0) as i32) as u32 as F32;
		let mut fTemp52: F32 = self.fTempPerm52;
		let mut iTemp53: i32 = self.iTempPerm53;
		let mut iTemp54: i32 = self.iTempPerm54;
		let mut fTemp55: F32 = self.fTempPerm55;
		let mut fTemp56: F32 = self.fTempPerm56;
		let mut fTemp57: F32 = self.fTempPerm57;
		let mut fTemp58: F32 = self.fTempPerm58;
		let mut iSlow169: i32 = if iSlow19 != 0 {iSlow162} else {-1};
		let mut fTemp59: F32 = self.fTempPerm59;
		let mut fTemp60: F32 = self.fTempPerm60;
		let mut iSlow170: i32 = if iSlow19 != 0 {iSlow163} else {-1};
		let mut fTemp61: F32 = self.fTempPerm61;
		let mut fSlow171: F32 = ((iSlow19 != 0) as i32) as u32 as F32;
		let mut iSlow172: i32 = if iSlow25 != 0 {iSlow163} else {-1};
		let mut fTemp62: F32 = self.fTempPerm62;
		let mut iSlow173: i32 = if iSlow25 != 0 {iSlow162} else {-1};
		let mut fTemp63: F32 = self.fTempPerm63;
		let mut fTemp64: F32 = self.fTempPerm64;
		let mut fTemp65: F32 = self.fTempPerm65;
		let mut fTemp66: F32 = self.fTempPerm66;
		let mut fSlow174: F32 = ((iSlow25 != 0) as i32) as u32 as F32;
		let mut iSlow175: i32 = if iSlow16 != 0 {iSlow162} else {-1};
		let mut iTemp67: i32 = self.iTempPerm67;
		let mut fTemp68: F32 = self.fTempPerm68;
		let mut fTemp69: F32 = self.fTempPerm69;
		let mut fTemp70: F32 = self.fTempPerm70;
		let mut fTemp71: F32 = self.fTempPerm71;
		let mut fTemp72: F32 = self.fTempPerm72;
		let mut fTemp73: F32 = self.fTempPerm73;
		let mut fTemp74: F32 = self.fTempPerm74;
		let mut fTemp75: F32 = self.fTempPerm75;
		let mut iSlow176: i32 = if iSlow16 != 0 {iSlow163} else {-1};
		let mut fTemp76: F32 = self.fTempPerm76;
		let mut fSlow177: F32 = ((iSlow16 != 0) as i32) as u32 as F32;
		let mut iSlow178: i32 = if iSlow22 != 0 {iSlow163} else {-1};
		let mut fTemp77: F32 = self.fTempPerm77;
		let mut fTemp78: F32 = self.fTempPerm78;
		let mut fSlow179: F32 = ((iSlow22 != 0) as i32) as u32 as F32;
		let mut iSlow180: i32 = if iSlow13 != 0 {iSlow162} else {-1};
		let mut iTemp79: i32 = self.iTempPerm79;
		let mut fTemp80: F32 = self.fTempPerm80;
		let mut fTemp81: F32 = self.fTempPerm81;
		let mut iTemp82: i32 = self.iTempPerm82;
		let mut fTemp83: F32 = self.fTempPerm83;
		let mut fTemp84: F32 = self.fTempPerm84;
		let mut fTemp85: F32 = self.fTempPerm85;
		let mut fTemp86: F32 = self.fTempPerm86;
		let mut fTemp87: F32 = self.fTempPerm87;
		let mut fTemp88: F32 = self.fTempPerm88;
		let mut fTemp89: F32 = self.fTempPerm89;
		let mut fTemp90: F32 = self.fTempPerm90;
		let mut fTemp91: F32 = self.fTempPerm91;
		let mut fTemp92: F32 = self.fTempPerm92;
		let mut fTemp93: F32 = self.fTempPerm93;
		let mut fTemp94: F32 = self.fTempPerm94;
		let mut fTemp95: F32 = self.fTempPerm95;
		let mut fTemp96: F32 = self.fTempPerm96;
		let mut fTemp97: F32 = self.fTempPerm97;
		let mut fTemp98: F32 = self.fTempPerm98;
		let mut fTemp99: F32 = self.fTempPerm99;
		let mut fTemp100: F32 = self.fTempPerm100;
		let mut fTemp101: F32 = self.fTempPerm101;
		let mut iSlow181: i32 = if iSlow13 != 0 {iSlow163} else {-1};
		let mut fTemp102: F32 = self.fTempPerm102;
		let mut fSlow182: F32 = ((iSlow13 != 0) as i32) as u32 as F32;
		let mut fSlow183: F32 = unsafe { ftbl8OscDspSIG8[((fSlow3) as i32) as usize] };
		let mut fTemp103: F32 = self.fTempPerm103;
		let mut iSlow184: i32 = unsafe { itbl9OscDspSIG9[iSlow103 as usize] };
		let mut fSlow185: F32 = if (fSlow105 > 0.0) as i32 != 0 {(0.049535416 * fSlow107 + 0.6165159) * F32::powf(1e+01, 0.05 * (i32::wrapping_add(iSlow184, i32::wrapping_mul(iSlow106, i32::wrapping_sub(unsafe { itbl10OscDspSIG10[iSlow103 as usize] }, iSlow184)))) as F32)} else {1.0};
		let mut fSlow186: F32 = ((iSlow0 != 0) as i32) as u32 as F32;
		let mut fSlow187: F32 = 43758.547 * F32::sin(fSlow117 + 469.398);
		let mut fSlow188: F32 = 0.25 * (2.0 * (fSlow187 - F32::floor(fSlow187)) + -1.0);
		let mut fSlow189: F32 = self.fVslider2;
		let mut fTemp104: F32 = self.fTempPerm104;
		let mut fTemp105: F32 = self.fTempPerm105;
		let mut iTemp106: i32 = self.iTempPerm106;
		let mut fSlow190: F32 = 43758.547 * F32::sin(fSlow117 + 547.631);
		let mut fSlow191: F32 = 0.25 * (2.0 * (fSlow190 - F32::floor(fSlow190)) + -1.0);
		let mut fSlow192: F32 = self.fVslider3;
		let mut fTemp107: F32 = self.fTempPerm107;
		let mut fTemp108: F32 = self.fTempPerm108;
		let mut iTemp109: i32 = self.iTempPerm109;
		let mut fSlow193: F32 = 43758.547 * F32::sin(fSlow117 + 1017.029);
		let mut fSlow194: F32 = 0.15 * (2.0 * (fSlow193 - F32::floor(fSlow193)) + -1.0);
		let mut fSlow195: F32 = self.fHslider25;
		let mut fSlow196: F32 = self.fHslider26;
		let mut iSlow197: i32 = ((fSlow196 != 0.0) as i32) | ((fSlow195 != 0.0) as i32);
		let mut fSlow198: F32 = self.fHslider27;
		let mut fSlow199: F32 = (iSlow197) as F32;
		let mut iSlow200: i32 = (fSlow199 != 0.0) as i32;
		let mut fSlow201: F32 = F32::sqrt(F32::sqrt(F32::max(0.0, 0.1010101 * (fSlow198 + -0.1))));
		let mut fTemp110: F32 = self.fTempPerm110;
		let mut fSlow202: F32 = ((iSlow197 != 0) as i32) as u32 as F32;
		let mut fSlow203: F32 = ((fSlow80 == 13.0) as i32) as u32 as F32;
		let mut fSlow204: F32 = ((fSlow84 == 13.0) as i32) as u32 as F32;
		let mut fTemp111: F32 = self.fTempPerm111;
		let mut fTemp112: F32 = self.fTempPerm112;
		let mut fSlow205: F32 = F32::max(-1.0, F32::min(1.0, self.fVslider4));
		let mut fSlow206: F32 = -6.0 * fSlow205;
		let mut fSlow207: F32 = if (F32::abs(fSlow206) < 0.001) as i32 != 0 {0.001} else {fSlow206};
		let mut fSlow208: F32 = F32::exp(fSlow207) + -1.0;
		let mut fTemp113: F32 = self.fTempPerm113;
		let mut fSlow209: F32 = ((fSlow80 == 12.0) as i32) as u32 as F32;
		let mut fTemp114: F32 = self.fTempPerm114;
		let mut fSlow210: F32 = ((fSlow84 == 12.0) as i32) as u32 as F32;
		let mut fTemp115: F32 = self.fTempPerm115;
		let mut fTemp116: F32 = self.fTempPerm116;
		let mut iTemp117: i32 = self.iTempPerm117;
		let mut fSlow211: F32 = self.fEntry9;
		let mut fSlow212: F32 = self.fEntry10;
		let mut iTemp118: i32 = self.iTempPerm118;
		let mut fSlow213: F32 = F32::exp(0.6931472 * self.fEntry11);
		let mut fSlow214: F32 = 256.0 * fSlow213;
		let mut fTemp119: F32 = self.fTempPerm119;
		let mut iTemp120: i32 = self.iTempPerm120;
		let mut iTemp121: i32 = self.iTempPerm121;
		let mut iTemp122: i32 = self.iTempPerm122;
		let mut iTemp123: i32 = self.iTempPerm123;
		let mut iTemp124: i32 = self.iTempPerm124;
		let mut iTemp125: i32 = self.iTempPerm125;
		let mut fSlow215: F32 = self.fConst3 * fSlow213;
		let mut fTemp126: F32 = self.fTempPerm126;
		let mut fTemp127: F32 = self.fTempPerm127;
		let mut iTemp128: i32 = self.iTempPerm128;
		let mut iTemp129: i32 = self.iTempPerm129;
		let mut iTemp130: i32 = self.iTempPerm130;
		let mut iTemp131: i32 = self.iTempPerm131;
		let mut fTemp132: F32 = self.fTempPerm132;
		let mut fTemp133: F32 = self.fTempPerm133;
		let mut iTemp134: i32 = self.iTempPerm134;
		let mut fTemp135: F32 = self.fTempPerm135;
		let mut fTemp136: F32 = self.fTempPerm136;
		let mut fTemp137: F32 = self.fTempPerm137;
		let mut fSlow216: F32 = self.fEntry12;
		let mut iTemp138: i32 = self.iTempPerm138;
		let mut fTemp139: F32 = self.fTempPerm139;
		let mut fTemp140: F32 = self.fTempPerm140;
		let mut fSlow217: F32 = ((fSlow84 == 24.0) as i32) as u32 as F32;
		let mut fSlow218: F32 = self.fHslider28;
		let mut fSlow219: F32 = self.fHslider29;
		let mut iSlow220: i32 = ((fSlow219 != 0.0) as i32) | ((fSlow218 != 0.0) as i32);
		let mut fSlow221: F32 = self.fHslider30;
		let mut fSlow222: F32 = (iSlow220) as F32;
		let mut iSlow223: i32 = (fSlow222 != 0.0) as i32;
		let mut fTemp141: F32 = self.fTempPerm141;
		let mut fSlow224: F32 = ((iSlow220 != 0) as i32) as u32 as F32;
		let mut fSlow225: F32 = ((fSlow80 == 24.0) as i32) as u32 as F32;
		let mut fTemp142: F32 = self.fTempPerm142;
		let mut fTemp143: F32 = self.fTempPerm143;
		let mut fTemp144: F32 = self.fTempPerm144;
		let mut fSlow226: F32 = ((iSlow30 != 0) as i32) as u32 as F32;
		let mut fSlow227: F32 = 512.0 * fSlow213;
		let mut fTemp145: F32 = self.fTempPerm145;
		let mut iTemp146: i32 = self.iTempPerm146;
		let mut iTemp147: i32 = self.iTempPerm147;
		let mut iTemp148: i32 = self.iTempPerm148;
		let mut iTemp149: i32 = self.iTempPerm149;
		let mut iTemp150: i32 = self.iTempPerm150;
		let mut iTemp151: i32 = self.iTempPerm151;
		let mut iTemp152: i32 = self.iTempPerm152;
		let mut fTemp153: F32 = self.fTempPerm153;
		let mut fTemp154: F32 = self.fTempPerm154;
		let mut iTemp155: i32 = self.iTempPerm155;
		let mut iTemp156: i32 = self.iTempPerm156;
		let mut iTemp157: i32 = self.iTempPerm157;
		let mut fTemp158: F32 = self.fTempPerm158;
		let mut fTemp159: F32 = self.fTempPerm159;
		let mut iTemp160: i32 = self.iTempPerm160;
		let mut fTemp161: F32 = self.fTempPerm161;
		let mut fTemp162: F32 = self.fTempPerm162;
		let mut fSlow228: F32 = ((iSlow36 != 0) as i32) as u32 as F32;
		let mut fTemp163: F32 = self.fTempPerm163;
		let mut fSlow229: F32 = ((iSlow39 != 0) as i32) as u32 as F32;
		let mut fTemp164: F32 = self.fTempPerm164;
		let mut fTemp165: F32 = self.fTempPerm165;
		let mut iTemp166: i32 = self.iTempPerm166;
		let mut iTemp167: i32 = self.iTempPerm167;
		let mut iTemp168: i32 = self.iTempPerm168;
		let mut iTemp169: i32 = self.iTempPerm169;
		let mut iTemp170: i32 = self.iTempPerm170;
		let mut iTemp171: i32 = self.iTempPerm171;
		let mut fTemp172: F32 = self.fTempPerm172;
		let mut iTemp173: i32 = self.iTempPerm173;
		let mut iTemp174: i32 = self.iTempPerm174;
		let mut iTemp175: i32 = self.iTempPerm175;
		let mut fTemp176: F32 = self.fTempPerm176;
		let mut fTemp177: F32 = self.fTempPerm177;
		let mut iTemp178: i32 = self.iTempPerm178;
		let mut fTemp179: F32 = self.fTempPerm179;
		let mut fTemp180: F32 = self.fTempPerm180;
		let mut fSlow230: F32 = ((iSlow42 != 0) as i32) as u32 as F32;
		let mut fTemp181: F32 = self.fTempPerm181;
		let mut fSlow231: F32 = ((iSlow45 != 0) as i32) as u32 as F32;
		let mut fTemp182: F32 = self.fTempPerm182;
		let mut fTemp183: F32 = self.fTempPerm183;
		let mut fSlow232: F32 = ((iSlow48 != 0) as i32) as u32 as F32;
		let mut fTemp184: F32 = self.fTempPerm184;
		let mut iTemp185: i32 = self.iTempPerm185;
		let mut iTemp186: i32 = self.iTempPerm186;
		let mut fTemp187: F32 = self.fTempPerm187;
		let mut fTemp188: F32 = self.fTempPerm188;
		let mut iTemp189: i32 = self.iTempPerm189;
		let mut fTemp190: F32 = self.fTempPerm190;
		let mut fTemp191: F32 = self.fTempPerm191;
		let mut fSlow233: F32 = ((iSlow51 != 0) as i32) as u32 as F32;
		let mut fSlow234: F32 = F32::max(0.0, F32::min(2.0, fSlow29 + -2.0));
		let mut fSlow235: F32 = self.fConst3 * (if (fSlow234 >= 2.0) as i32 != 0 {4000} else {if (fSlow234 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp192: F32 = self.fTempPerm192;
		let mut fSlow236: F32 = 6.0 * fSlow205;
		let mut fSlow237: F32 = if (F32::abs(-fSlow236) < 0.001) as i32 != 0 {0.001} else {-fSlow236};
		let mut fSlow238: F32 = F32::exp(fSlow237) + -1.0;
		let mut fTemp193: F32 = self.fTempPerm193;
		let mut iTemp194: i32 = self.iTempPerm194;
		let mut iTemp195: i32 = self.iTempPerm195;
		let mut fTemp196: F32 = self.fTempPerm196;
		let mut fTemp197: F32 = self.fTempPerm197;
		let mut iTemp198: i32 = self.iTempPerm198;
		let mut fTemp199: F32 = self.fTempPerm199;
		let mut fTemp200: F32 = self.fTempPerm200;
		let mut fSlow239: F32 = ((iSlow54 != 0) as i32) as u32 as F32;
		let mut fTemp201: F32 = self.fTempPerm201;
		let mut fSlow240: F32 = ((iSlow57 != 0) as i32) as u32 as F32;
		let mut fTemp202: F32 = self.fTempPerm202;
		let mut fSlow241: F32 = ((iSlow60 != 0) as i32) as u32 as F32;
		let mut fSlow242: F32 = F32::max(0.0, F32::min(2.0, fSlow29 + -5.0));
		let mut fSlow243: F32 = self.fConst19 * (if (fSlow242 >= 2.0) as i32 != 0 {800} else {if (fSlow242 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp203: F32 = self.fTempPerm203;
		let mut fTemp204: F32 = self.fTempPerm204;
		let mut fTemp205: F32 = self.fTempPerm205;
		let mut fTemp206: F32 = self.fTempPerm206;
		let mut fTemp207: F32 = self.fTempPerm207;
		let mut fSlow244: F32 = ((iSlow63 != 0) as i32) as u32 as F32;
		let mut fTemp208: F32 = self.fTempPerm208;
		let mut fSlow245: F32 = ((iSlow66 != 0) as i32) as u32 as F32;
		let mut fTemp209: F32 = self.fTempPerm209;
		let mut fSlow246: F32 = ((iSlow69 != 0) as i32) as u32 as F32;
		let mut fTemp210: F32 = self.fTempPerm210;
		let mut fSlow247: F32 = ((iSlow72 != 0) as i32) as u32 as F32;
		let mut fTemp211: F32 = self.fTempPerm211;
		let mut fTemp212: F32 = self.fTempPerm212;
		let mut fSlow248: F32 = ((iSlow75 != 0) as i32) as u32 as F32;
		let mut fTemp213: F32 = self.fTempPerm213;
		let mut fSlow249: F32 = ((iSlow33 != 0) as i32) as u32 as F32;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3);
		for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
			if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
				self.fVec0[0] = fSlow28;
				self.iVec1[0] = 1;
				iTemp0 = (fSlow28 > self.fVec0[1]) as i32;
				iTemp1 = i32::wrapping_sub(1, iTemp0);
				fTemp2 = (iTemp1) as F32;
				fTemp3 = 1.0 - self.fConst1 * fTemp2;
				self.fRec1[0] = fSlow78 * fTemp3 + self.fConst1 * fTemp2 * self.fRec1[1];
				self.fRec2[0] = fSlow79 * fTemp3 + self.fConst1 * fTemp2 * self.fRec2[1];
				fTemp4 = self.fRec2[0] * self.fRec1[0];
				self.fRec3[0] = fSlow82 * fTemp3 + self.fConst1 * fTemp2 * self.fRec3[1];
				self.fRec4[0] = fSlow83 * fTemp3 + self.fConst1 * fTemp2 * self.fRec4[1];
				fTemp5 = self.fRec4[0] * self.fRec3[0];
				self.fRec5[0] = fSlow93 * fTemp3 + self.fConst1 * fTemp2 * self.fRec5[1];
				fTemp6 = if (self.fRec2[0] >= 0.0) as i32 != 0 {1.0 - self.fRec2[0] * (1.0 - self.fRec1[0])} else {fTemp4 + 1.0};
				fTemp7 = if (self.fRec4[0] >= 0.0) as i32 != 0 {1.0 - self.fRec4[0] * (1.0 - self.fRec3[0])} else {fTemp5 + 1.0};
				fTemp8 = *input0 * if iSlow97 != 0 {fTemp7} else {1.0} * if iSlow96 != 0 {fTemp6} else {1.0};
				fTemp9 = *input1 * if iSlow99 != 0 {fTemp7} else {1.0} * if iSlow98 != 0 {fTemp6} else {1.0};
			}
				if (iSlow95 & (iSlow2 & iSlow18)) | ((iSlow95 & (iSlow2 & iSlow24)) | ((iSlow95 & (iSlow2 & iSlow15)) | ((iSlow95 & (iSlow2 & iSlow21)) | ((iSlow95 & (iSlow2 & iSlow27)) | ((iSlow95 & (iSlow2 & iSlow12)) | ((iSlow95 & (iSlow2 & iSlow9)) | ((iSlow95 & (iSlow2 & iSlow6)) | ((iSlow35 & (iSlow95 & iSlow77)) | ((iSlow35 & (iSlow95 & iSlow74)) | ((iSlow35 & (iSlow95 & iSlow71)) | ((iSlow35 & (iSlow95 & iSlow68)) | ((iSlow35 & (iSlow95 & iSlow65)) | ((iSlow35 & (iSlow95 & iSlow62)) | ((iSlow35 & (iSlow95 & iSlow59)) | ((iSlow35 & (iSlow95 & iSlow56)) | ((iSlow35 & (iSlow95 & iSlow53)) | ((iSlow35 & (iSlow95 & iSlow50)) | ((iSlow35 & (iSlow95 & iSlow47)) | ((iSlow35 & (iSlow95 & iSlow44)) | ((iSlow35 & (iSlow95 & iSlow41)) | ((iSlow35 & (iSlow95 & iSlow38)) | (iSlow35 & (iSlow32 & iSlow95))))))))))))))))))))))) != 0 {
					fTemp10 = fSlow100 * F32::powf(F32::max(0.0, F32::min(1.0, fSlow90 * fTemp9 + F32::powf(F32::max(0.0, 0.02 * self.fRec5[0]), 0.4) + fSlow91 * fTemp8)), 2.5);
				}
					if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
						self.fRec6[0] = fSlow112 * fTemp3 + self.fConst1 * fTemp2 * self.fRec6[1];
					}
						if (iSlow114 & (iSlow2 & iSlow24)) | ((iSlow114 & (iSlow2 & iSlow27)) | ((iSlow114 & (iSlow2 & iSlow18)) | ((iSlow114 & (iSlow2 & iSlow21)) | ((iSlow114 & (iSlow2 & iSlow15)) | ((iSlow114 & (iSlow2 & iSlow12)) | ((iSlow114 & (iSlow2 & iSlow9)) | ((iSlow114 & (iSlow2 & iSlow6)) | ((iSlow35 & (iSlow114 & iSlow77)) | ((iSlow35 & (iSlow114 & iSlow74)) | ((iSlow35 & (iSlow114 & iSlow71)) | ((iSlow35 & (iSlow114 & iSlow68)) | ((iSlow35 & (iSlow114 & iSlow65)) | ((iSlow35 & (iSlow114 & iSlow62)) | ((iSlow35 & (iSlow114 & iSlow59)) | ((iSlow35 & (iSlow114 & iSlow56)) | ((iSlow35 & (iSlow114 & iSlow53)) | ((iSlow35 & (iSlow114 & iSlow50)) | ((iSlow35 & (iSlow114 & iSlow47)) | ((iSlow35 & (iSlow114 & iSlow44)) | ((iSlow35 & (iSlow114 & iSlow41)) | ((iSlow35 & (iSlow114 & iSlow38)) | (iSlow35 & (iSlow32 & iSlow114))))))))))))))))))))))) != 0 {
							fTemp11 = fSlow113 * F32::max(0.0, F32::min(1.0, fSlow109 * fTemp9 + F32::max(0.0, self.fRec6[0]) + fSlow110 * fTemp8));
						}
							if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
								fTemp12 = if iSlow111 != 0 {fSlow115 * fTemp11} else {self.fRec6[0]};
								iTemp13 = (F32::abs(fTemp12) > 0.0001) as i32;
								fTemp14 = (iTemp13) as F32;
								iTemp15 = i32::wrapping_sub(1, self.iVec1[1]);
							}
								if (iSlow2 & (iSlow24 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow15 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow18 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow21 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow27 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow12 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow9 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow6 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow77 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow74 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow71 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow68 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow65 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow62 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow59 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow56 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow53 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow50 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow47 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow44 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow41 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow38 & ((fTemp14 != 0.0) as i32))) | (iSlow35 & (iSlow32 & ((fTemp14 != 0.0) as i32)))))))))))))))))))))))) != 0 {
									fTemp16 = if iTemp15 != 0 {0.0} else {self.fConst2 + self.fRec8[1]};
									self.fRec8[0] = fTemp16 - F32::floor(fTemp16);
									fTemp17 = if iTemp15 != 0 {0.0} else {fSlow119 + self.fRec9[1]};
									self.fRec9[0] = fTemp17 - F32::floor(fTemp17);
									self.iRec11[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec11[1], i32::wrapping_mul(iSlow120, iTemp15))), 12345);
									iTemp18 = i32::wrapping_mul(1664525, self.iRec11[0] ^ (self.iRec11[0] >> 15));
									self.fRec10[0] = self.fConst4 * self.fRec10[1] + self.fConst6 * (i32::wrapping_mul(1013904223, iTemp18 ^ (iTemp18 >> 13))) as F32;
									self.fRec12[0] = fSlow121 * fTemp3 + self.fConst1 * fTemp2 * self.fRec12[1];
									fTemp19 = if iTemp15 != 0 {0.0} else {fSlow126 + self.fRec15[1]};
									self.fRec15[0] = fTemp19 - F32::floor(fTemp19);
								}
									if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
										self.iRec16[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec16[1]), 12345);
										fTemp20 = 4.656613e-10 * (self.iRec16[0]) as F32;
									}
										if (iSlow2 & (iSlow24 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow15 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow18 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow21 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow27 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow12 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow9 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow6 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow77 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow74 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow71 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow68 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow65 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow62 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow59 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow56 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow53 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow50 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow47 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow44 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow41 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow38 & ((fTemp14 != 0.0) as i32))) | (iSlow35 & (iSlow32 & ((fTemp14 != 0.0) as i32)))))))))))))))))))))))) != 0 {
											self.fRec14[0] = if (self.fRec15[0] < self.fRec15[1]) as i32 != 0 {fTemp20} else {self.fRec14[1]};
											fTemp21 = F32::floor(32768.0 * self.fRec14[0] + 0.5);
											self.fVec3[0] = fTemp21;
											self.fRec13[0] = fSlow127 * (3.0517578e-05 * (fTemp21 + self.fVec3[1]) - fSlow125 * self.fRec13[1]);
											fTemp22 = (fTemp12 * (fSlow129 + 2e+01 * self.fRec13[0] + self.fConst7 * self.fRec12[0] * self.fRec10[0]) + OscDsp_faustpower2_f(fTemp12) * (18.0 * unsafe { ftbl2OscDspSIG2[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec9[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl2OscDspSIG2[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec8[0]) as i32, 65535))) as usize] })) * fTemp14;
										}
											if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
												fTemp23 = if iTemp15 != 0 {0.0} else {self.fConst9 + self.fRec17[1]};
												self.fRec17[0] = fTemp23 - F32::floor(fTemp23);
												self.fRec18[0] = fSlow132 * fTemp3 + self.fConst1 * fTemp2 * self.fRec18[1];
												fTemp24 = 0.33333334 * F32::max(1e-05, if iSlow136 != 0 {0.001 * (2.7e+02 * F32::max(0.0, F32::min(1.0, fSlow139 + fSlow138 * fTemp8 + fSlow137 * fTemp9)) + 3e+01)} else {0.0});
												iTemp25 = (F32::abs(fTemp24) < 1.1920929e-07) as i32;
												fTemp26 = if iTemp25 != 0 {0.0} else {F32::exp(-(self.fConst3 / if iTemp25 != 0 {1.0} else {fTemp24}))};
												self.fRec19[0] = fSlow140 * (1.0 - fTemp26) + fTemp26 * self.fRec19[1];
												fTemp27 = F32::exp(self.fRec19[0]) * F32::exp(0.057762265 * (fSlow134 * self.fRec18[0] + 0.5 * (fSlow131 * fTemp5 + fSlow130 * fTemp4) * unsafe { ftbl2OscDspSIG2[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec17[0]) as i32, 65535))) as usize] } + 0.01 * fTemp22 * ((iTemp13 != 0) as i32) as u32 as F32)) * F32::exp(0.0005776227 * fSlow108 * F32::max(0.0, F32::min(5e+01, if iSlow92 != 0 {fSlow101 * fTemp10} else {self.fRec5[0]} + 5e+01 * (fSlow89 * fTemp5 + fSlow88 * fTemp4))));
											}
												if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | (iSlow2 & iSlow6))))))) != 0 {
													fTemp28 = fTemp27 * F32::exp(0.6931472 * (fSlow87 + 0.083333336 * (fSlow86 + 12.0 * (fSlow85 * fTemp5 + fSlow81 * fTemp4))));
													fTemp29 = self.fRec0[1] + self.fConst3 * fTemp28;
												}
													if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
														self.fRec20[0] = if iTemp0 != 0 {fTemp20} else {self.fRec20[1]};
														fTemp30 = 0.5 * (self.fRec20[0] + 1.0);
														fTemp31 = fSlow141 + (fTemp30 - F32::floor(fSlow141 + fTemp30));
														fTemp32 = fSlow142 * fTemp31;
														fTemp33 = fTemp32 - F32::floor(fTemp32);
													}
														if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | (iSlow2 & iSlow6))))))) != 0 {
															self.fRec0[0] = if iTemp0 != 0 {fTemp33} else {fTemp29 - F32::floor(fTemp29)};
														}
															if iSlow2 & iSlow6 != 0 {
																iTemp34 = (self.fRec0[0] < 0.5) as i32;
															}
																if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | (iSlow2 & iSlow6))))))) != 0 {
																	self.fRec21[0] = fSlow147 * fTemp3 + self.fConst1 * fTemp2 * self.fRec21[1];
																}
																	if (iSlow2 & (iSlow24 & iSlow149)) | ((iSlow2 & (iSlow149 & iSlow27)) | ((iSlow2 & (iSlow149 & iSlow18)) | ((iSlow2 & (iSlow149 & iSlow21)) | ((iSlow2 & (iSlow149 & iSlow15)) | ((iSlow2 & (iSlow149 & iSlow12)) | ((iSlow2 & (iSlow149 & iSlow9)) | (iSlow2 & (iSlow149 & iSlow6)))))))) != 0 {
																		fTemp35 = fSlow148 * F32::max(0.0, F32::min(1.0, fSlow144 * fTemp9 + F32::max(0.0, self.fRec21[0]) + fSlow145 * fTemp8));
																	}
																		if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | (iSlow2 & iSlow6))))))) != 0 {
																			fTemp36 = F32::max(0.0, F32::min(1.0, fSlow151 * fTemp4 + if iSlow146 != 0 {fSlow150 * fTemp35} else {self.fRec21[0]} + fSlow143 * fTemp5));
																		}
																			if (iSlow2 & iSlow18) | ((iSlow2 & iSlow9) | (iSlow2 & iSlow6)) != 0 {
																				fTemp37 = 1.0 - fTemp36;
																			}
																				if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | (iSlow2 & iSlow6))))))) != 0 {
																					self.fRec22[0] = fSlow156 * fTemp3 + self.fConst1 * fTemp2 * self.fRec22[1];
																				}
																					if (iSlow2 & (iSlow24 & iSlow158)) | ((iSlow2 & (iSlow158 & iSlow21)) | ((iSlow2 & (iSlow158 & iSlow27)) | ((iSlow2 & (iSlow158 & iSlow18)) | ((iSlow2 & (iSlow158 & iSlow15)) | ((iSlow2 & (iSlow158 & iSlow12)) | ((iSlow2 & (iSlow158 & iSlow9)) | (iSlow2 & (iSlow158 & iSlow6)))))))) != 0 {
																						fTemp38 = fSlow157 * F32::max(0.0, F32::min(1.0, fSlow153 * fTemp9 + F32::max(0.0, self.fRec22[0]) + fSlow154 * fTemp8));
																					}
																						if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | (iSlow2 & iSlow6))))))) != 0 {
																							fTemp39 = F32::max(0.0, F32::min(1.0, fSlow160 * fTemp4 + if iSlow155 != 0 {fSlow159 * fTemp38} else {self.fRec22[0]} + fSlow152 * fTemp5));
																						}
																							if iSlow2 & iSlow6 != 0 {
																								fTemp40 = if iTemp34 != 0 {1.0 - fTemp39} else {fTemp37};
																							}
																								if (iSlow2 & iSlow21) | (iSlow2 & iSlow6) != 0 {
																									fTemp41 = 2.0 * self.fRec0[0];
																								}
																									if iSlow2 & iSlow6 != 0 {
																										fTemp42 = fTemp41 - F32::floor(fTemp41);
																										fTemp43 = F32::max(0.005, fTemp40);
																										fTemp44 = fSlow5 * wt_basic(if iSlow4 != 0 {if iTemp34 != 0 {iSlow163} else {iSlow162}} else {-1}, 2.0 * (fTemp28 / fTemp43), self.fConst0, fTemp42 / fTemp43) * ((fTemp42 < fTemp40) as i32) as u32 as F32;
																									}
																										if (iSlow2 & iSlow12) | (iSlow2 & iSlow9) != 0 {
																											self.iRec23[0] = (i32::wrapping_add(self.iRec23[1], (self.fRec0[0] < self.fRec0[1]) as i32)) % 2;
																										}
																											if (iSlow2 & iSlow21) | (iSlow2 & iSlow9) != 0 {
																												fTemp45 = 15.0 * fTemp39 + 1.0;
																												fTemp46 = fTemp28 * fTemp45;
																												fTemp47 = self.fRec0[0] * fTemp45;
																											}
																												if iSlow2 & iSlow9 != 0 {
																													fTemp48 = fSlow8 * (fTemp37 * wt_basic(iSlow166, fTemp46, self.fConst0, fTemp47) + fTemp36 * wt_basic(iSlow165, 0.5 * fTemp28, self.fConst0, 0.5 * (self.fRec0[0] + (self.iRec23[0]) as F32)));
																												}
																													if iSlow2 & iSlow12 != 0 {
																														fTemp49 = 7.0 * fTemp39 + 1.0;
																														fTemp50 = 0.25 * (if self.iRec23[0] != 0 {fTemp49 * (fTemp36 + 1.0)} else {fTemp49} * wt_basic(if iSlow10 != 0 {if self.iRec23[0] != 0 {iSlow162} else {iSlow163}} else {-1}, fTemp28, self.fConst0, self.fRec0[0]) + 1.0);
																														fTemp51 = fSlow11 * (1.0 - 4.0 * F32::abs(fTemp50 + (-0.5 - F32::floor(fTemp50))));
																													}
																														if iSlow2 & iSlow21 != 0 {
																															fTemp52 = 4.0 * F32::max(0.0, F32::min(1.0, fTemp36));
																															iTemp53 = (F32::min(3.0, fTemp52)) as i32;
																															iTemp54 = i32::wrapping_add(iTemp53, 1);
																															fTemp55 = 1.0 - self.fRec0[0];
																															fTemp56 = 1.0 - F32::abs(fTemp41 + -1.0);
																															fTemp57 = F32::sin(3.1415927 * self.fRec0[0]);
																															fTemp58 = F32::min(1.0, 6.0 * F32::min(self.fRec0[0], fTemp55));
																															fTemp59 = 0.5 * (wt_basic(iSlow169, fTemp28, self.fConst0, self.fRec0[0]) + 1.0);
																															fTemp60 = (iTemp53) as F32;
																															fTemp61 = fSlow20 * wt_basic(iSlow170, fTemp46, self.fConst0, fTemp47) * ((fTemp60 + (1.0 - fTemp52)) * if (iTemp53 >= 3) as i32 != 0 {if (iTemp53 >= 4) as i32 != 0 {fTemp59} else {fTemp58}} else {if (iTemp53 >= 2) as i32 != 0 {fTemp57} else {if (iTemp53 >= 1) as i32 != 0 {fTemp56} else {fTemp55}}} + (fTemp52 - fTemp60) * if (iTemp54 >= 3) as i32 != 0 {if (iTemp54 >= 4) as i32 != 0 {fTemp59} else {fTemp58}} else {if (iTemp54 >= 2) as i32 != 0 {fTemp57} else {if (iTemp54 >= 1) as i32 != 0 {fTemp56} else {fTemp55}}});
																														}
																															if iSlow2 & iSlow27 != 0 {
																																fTemp62 = unsafe { ftbl5OscDspSIG5[((F32::max(0.0, F32::min(9.0, 9.999 * fTemp36))) as i32) as usize] };
																																fTemp63 = fTemp28 * fTemp62;
																																fTemp64 = self.fRec24[1] + self.fConst3 * fTemp63;
																																fTemp65 = fSlow142 * fTemp31 * fTemp62;
																																self.fRec24[0] = if iTemp0 != 0 {fTemp65 - F32::floor(fTemp65)} else {fTemp64 - F32::floor(fTemp64)};
																																fTemp66 = fSlow26 * wt_basic(iSlow172, fTemp28 * (2.0 * fTemp39 * fTemp62 + 1.0), self.fConst0, self.fRec0[0] + 2.0 * fTemp39 * wt_basic(iSlow173, fTemp63, self.fConst0, self.fRec24[0]));
																															}
																																if iSlow2 & iSlow18 != 0 {
																																	iTemp67 = (F32::max(0.0, F32::min(1e+01, 10.999 * fTemp39))) as i32;
																																	fTemp68 = unsafe { ftbl6OscDspSIG6[(i32::wrapping_add(iTemp67, 1)) as usize] };
																																	fTemp69 = fTemp28 * fTemp68;
																																	fTemp70 = self.fRec25[1] + self.fConst3 * fTemp69;
																																	fTemp71 = fSlow142 * fTemp31 * fTemp68;
																																	self.fRec25[0] = if iTemp0 != 0 {fTemp71 - F32::floor(fTemp71)} else {fTemp70 - F32::floor(fTemp70)};
																																	fTemp72 = unsafe { ftbl6OscDspSIG6[iTemp67 as usize] };
																																	fTemp73 = fTemp28 * fTemp72;
																																	fTemp74 = self.fRec26[1] + self.fConst3 * fTemp73;
																																	fTemp75 = fSlow142 * fTemp31 * fTemp72;
																																	self.fRec26[0] = if iTemp0 != 0 {fTemp75 - F32::floor(fTemp75)} else {fTemp74 - F32::floor(fTemp74)};
																																	fTemp76 = fSlow17 * wt_basic(iSlow176, fTemp28, self.fConst0, self.fRec0[0]) * (fTemp37 * wt_basic(iSlow175, fTemp73, self.fConst0, self.fRec26[0]) + fTemp36 * wt_basic(iSlow175, fTemp69, self.fConst0, self.fRec25[0]));
																																}
																																	if iSlow2 & iSlow24 != 0 {
																																		fTemp77 = F32::sin(1.5707964 * (9.0 * fTemp39 + 1.0) * (wt_basic(iSlow178, fTemp28, self.fConst0, self.fRec0[0]) + 1.5 * fTemp36));
																																		self.fVec4[0] = fTemp77;
																																		self.fRec27[0] = 0.995 * self.fRec27[1] + fTemp77 - self.fVec4[1];
																																		fTemp78 = fSlow23 * self.fRec27[0];
																																	}
																																		if iSlow2 & iSlow15 != 0 {
																																			iTemp79 = (fTemp36 > 0.5) as i32;
																																			fTemp80 = if iTemp79 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * fTemp36)};
																																			fTemp81 = 0.3 * fTemp80;
																																			iTemp82 = unsafe { itbl7OscDspSIG7[((F32::max(0.0, F32::min(4.0, 1e+01 * (fTemp36 + -0.5)))) as i32) as usize] };
																																			fTemp83 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp79, iTemp82))) as F32;
																																			fTemp84 = fTemp28 * F32::exp(0.0005776227 * (fTemp83 + fTemp81));
																																			fTemp85 = self.fRec28[1] + self.fConst3 * fTemp84;
																																			fTemp86 = F32::floor(fTemp32 + 1.24);
																																			self.fRec28[0] = if iTemp0 != 0 {fTemp32 + (1.24 - (fTemp86 + F32::floor(fTemp32 + (1.24 - fTemp86))))} else {fTemp85 - F32::floor(fTemp85)};
																																			fTemp87 = 0.62 * fTemp80;
																																			fTemp88 = fTemp28 * F32::exp(0.0005776227 * (fTemp83 - fTemp87));
																																			fTemp89 = self.fRec29[1] + self.fConst3 * fTemp88;
																																			fTemp90 = F32::floor(fTemp32 + 0.5);
																																			self.fRec29[0] = if iTemp0 != 0 {fTemp32 + (0.5 - (fTemp90 + F32::floor(fTemp32 + (0.5 - fTemp90))))} else {fTemp89 - F32::floor(fTemp89)};
																																			fTemp91 = fTemp28 * F32::exp(0.0005776227 * (fTemp80 + fTemp83));
																																			fTemp92 = self.fRec30[1] + self.fConst3 * fTemp91;
																																			fTemp93 = F32::floor(fTemp32 + 1.98);
																																			self.fRec30[0] = if iTemp0 != 0 {fTemp32 + (1.98 - (fTemp93 + F32::floor(fTemp32 + (1.98 - fTemp93))))} else {fTemp92 - F32::floor(fTemp92)};
																																			fTemp94 = (i32::wrapping_mul(0, iTemp82)) as F32;
																																			fTemp95 = self.fRec31[1] + self.fConst3 * fTemp28 * F32::exp(0.0005776227 * (fTemp94 - fTemp80));
																																			fTemp96 = F32::floor(fTemp32 + 0.13);
																																			self.fRec31[0] = if iTemp0 != 0 {fTemp32 + (0.13 - (fTemp96 + F32::floor(fTemp32 + (0.13 - fTemp96))))} else {fTemp95 - F32::floor(fTemp95)};
																																			fTemp97 = self.fRec32[1] + self.fConst3 * fTemp28 * F32::exp(0.0005776227 * (fTemp94 - fTemp81));
																																			fTemp98 = F32::floor(fTemp32 + 0.87);
																																			self.fRec32[0] = if iTemp0 != 0 {fTemp32 + (0.87 - (fTemp98 + F32::floor(fTemp32 + (0.87 - fTemp98))))} else {fTemp97 - F32::floor(fTemp97)};
																																			fTemp99 = self.fRec33[1] + self.fConst3 * fTemp28 * F32::exp(0.0005776227 * (fTemp94 + fTemp87));
																																			fTemp100 = F32::floor(fTemp32 + 1.61);
																																			self.fRec33[0] = if iTemp0 != 0 {fTemp32 + (1.61 - (fTemp100 + F32::floor(fTemp32 + (1.61 - fTemp100))))} else {fTemp99 - F32::floor(fTemp99)};
																																			fTemp101 = 1.5707964 * fTemp39;
																																			fTemp102 = fSlow14 * (F32::cos(fTemp101) * wt_basic(iSlow181, fTemp28, self.fConst0, self.fRec0[0]) + 0.5 * F32::sin(fTemp101) * (wt_basic(iSlow180, fTemp28 * F32::exp(0.00035812604 * fTemp80), self.fConst0, self.fRec33[0]) + wt_basic(iSlow180, fTemp28 * F32::exp(-0.0005776227 * fTemp81), self.fConst0, self.fRec32[0]) + wt_basic(iSlow180, fTemp28 * F32::exp(-0.0005776227 * fTemp80), self.fConst0, self.fRec31[0]) + wt_basic(iSlow180, fTemp91, self.fConst0, self.fRec30[0]) + wt_basic(iSlow180, fTemp88, self.fConst0, self.fRec29[0]) + wt_basic(iSlow180, fTemp84, self.fConst0, self.fRec28[0])));
																																		}
																																			if iSlow2 != 0 {
																																				fTemp103 = fSlow1 * fSlow183 * (fSlow182 * fTemp102 + fSlow179 * fTemp78 + fSlow177 * fTemp76 + fSlow174 * fTemp66 + fSlow171 * fTemp61 + fSlow168 * fTemp51 + fSlow167 * fTemp48 + fSlow164 * fTemp44);
																																			}
																																				*output0 = fSlow186 * fSlow185 * fTemp103;
																																				if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))) != 0 {
																																					fTemp104 = F32::max(F32::max(0.005, fSlow189 * (fSlow188 * fTemp12 + 1.0)), 0.0001);
																																					self.iRec34[0] = i32::wrapping_mul(iTemp1, i32::wrapping_add(self.iRec34[1], 1));
																																					fTemp105 = self.fConst3 * (self.iRec34[0]) as F32;
																																					iTemp106 = (fTemp105 < fTemp104) as i32;
																																					fTemp107 = F32::max(fSlow192 * (fSlow191 * fTemp12 + 1.0), 0.0001);
																																					fTemp108 = fTemp104 + fTemp107;
																																					iTemp109 = (fTemp105 < fTemp108) as i32;
																																				}
																																					if (iSlow35 & (iSlow200 & iSlow77)) | ((iSlow35 & (iSlow200 & iSlow74)) | ((iSlow35 & (iSlow200 & iSlow71)) | ((iSlow35 & (iSlow200 & iSlow68)) | ((iSlow35 & (iSlow200 & iSlow65)) | ((iSlow35 & (iSlow200 & iSlow62)) | ((iSlow35 & (iSlow200 & iSlow59)) | ((iSlow35 & (iSlow200 & iSlow56)) | ((iSlow35 & (iSlow200 & iSlow53)) | ((iSlow35 & (iSlow200 & iSlow50)) | ((iSlow35 & (iSlow200 & iSlow47)) | ((iSlow35 & (iSlow200 & iSlow44)) | ((iSlow35 & (iSlow200 & iSlow41)) | ((iSlow35 & (iSlow200 & iSlow38)) | (iSlow35 & (iSlow32 & iSlow200))))))))))))))) != 0 {
																																						fTemp110 = fSlow199 * (9.9 * OscDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, fSlow201 + fSlow196 * fTemp8 + fSlow195 * fTemp9))) + 0.1);
																																					}
																																						if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))) != 0 {
																																							fTemp111 = (fTemp105 - fTemp108) * F32::exp(1.3862944 * (fSlow204 * fTemp5 + fSlow203 * fTemp4)) / (if iSlow197 != 0 {fSlow202 * fTemp110} else {fSlow198} * (fSlow194 * fTemp12 + 1.0));
																																							fTemp112 = F32::abs(1.0 - 2.0 * (fTemp111 - F32::floor(fTemp111)));
																																							fTemp113 = fTemp105 - fTemp104;
																																							fTemp114 = fSlow209 * fTemp4;
																																							fTemp115 = fSlow210 * fTemp5;
																																						}
																																							if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))) != 0 {
																																								fTemp116 = F32::max(0.0, F32::min(1.0, F32::max(0.0, F32::min(1.0, fTemp115 + fTemp114 + if iTemp106 != 0 {0.0} else {if iTemp109 != 0 {0.5 * ((F32::exp(fTemp113 * fSlow207 / fTemp107) + -1.0) / fSlow208)} else {0.5 * (2.0 - fTemp112)}}))));
																																								iTemp117 = (fTemp116 > 0.5) as i32;
																																								iTemp118 = i32::wrapping_mul(2176, (if iTemp117 != 0 {fSlow212} else {fSlow211}) as i32);
																																							}
																																								if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32))))))))))))) != 0 {
																																									fTemp119 = fSlow214 * fTemp27;
																																									iTemp120 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp119 > self.fConst11) as i32) + ((fTemp119 > self.fConst17) as i32), (fTemp119 > self.fConst16) as i32), (fTemp119 > self.fConst15) as i32), (fTemp119 > self.fConst14) as i32), (fTemp119 > self.fConst13) as i32), (fTemp119 > self.fConst12) as i32);
																																									iTemp121 = (iTemp120 >= 4) as i32;
																																									iTemp122 = (iTemp120 >= 2) as i32;
																																									iTemp123 = (iTemp120 >= 1) as i32;
																																									iTemp124 = (iTemp120 >= 3) as i32;
																																									iTemp125 = if iTemp121 != 0 {64} else {if iTemp122 != 0 {if iTemp124 != 0 {128} else {256}} else {if iTemp123 != 0 {512} else {1024}}};
																																								}
																																									if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))) != 0 {
																																										fTemp126 = self.fRec35[1] + fSlow215 * fTemp27;
																																										self.fRec35[0] = if iTemp0 != 0 {fTemp33} else {fTemp126 - F32::floor(fTemp126)};
																																									}
																																										if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32))))))))))))) != 0 {
																																											fTemp127 = self.fRec35[0] * (iTemp125) as F32;
																																											iTemp128 = (fTemp127) as i32;
																																											iTemp129 = (i32::wrapping_add(iTemp128, 1)) % iTemp125;
																																											iTemp130 = if iTemp121 != 0 {if (iTemp120 >= 6) as i32 != 0 {if (iTemp120 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp120 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp122 != 0 {if iTemp124 != 0 {1792} else {1536}} else {if iTemp123 != 0 {1024} else {0}}};
																																											iTemp131 = i32::wrapping_add(iTemp130, iTemp129);
																																											fTemp132 = (iTemp128) as F32;
																																											fTemp133 = fTemp127 - fTemp132;
																																											iTemp134 = i32::wrapping_add(iTemp128, iTemp130);
																																											fTemp135 = fTemp132 + (1.0 - fTemp127);
																																										}
																																											if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))) != 0 {
																																												fTemp136 = 2.0 * fTemp116;
																																												fTemp137 = if iTemp117 != 0 {fTemp136 + -1.0} else {fTemp136};
																																												iTemp138 = i32::wrapping_mul(2176, (if iTemp117 != 0 {fSlow211} else {fSlow216}) as i32);
																																												fTemp139 = 1.0 - fTemp137;
																																											}
																																												if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32))))))) != 0 {
																																													fTemp140 = fTemp139 * (fTemp135 * wt_sample(i32::wrapping_add(iTemp134, iTemp138)) + fTemp133 * wt_sample(i32::wrapping_add(iTemp131, iTemp138))) + fTemp137 * (fTemp135 * wt_sample(i32::wrapping_add(iTemp134, iTemp118)) + fTemp133 * wt_sample(i32::wrapping_add(iTemp131, iTemp118)));
																																												}
																																													if (iSlow35 & iSlow77) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32))))))))))))) != 0 {
																																														self.fRec36[0] = fSlow221 * fTemp3 + self.fConst1 * fTemp2 * self.fRec36[1];
																																													}
																																														if (iSlow35 & (iSlow223 & iSlow77)) | ((iSlow35 & (iSlow223 & iSlow71)) | ((iSlow35 & (iSlow223 & iSlow68)) | ((iSlow35 & (iSlow223 & iSlow65)) | ((iSlow35 & (iSlow223 & iSlow62)) | ((iSlow35 & (iSlow223 & iSlow59)) | ((iSlow35 & (iSlow223 & iSlow56)) | ((iSlow35 & (iSlow223 & iSlow53)) | ((iSlow35 & (iSlow223 & iSlow50)) | ((iSlow35 & (iSlow223 & iSlow47)) | ((iSlow35 & (iSlow223 & iSlow44)) | ((iSlow35 & (iSlow223 & iSlow41)) | ((iSlow35 & (iSlow223 & iSlow38)) | (iSlow35 & (iSlow32 & iSlow223)))))))))))))) != 0 {
																																															fTemp141 = fSlow222 * F32::max(0.0, F32::min(1.0, fSlow218 * fTemp9 + F32::max(0.0, self.fRec36[0]) + fSlow219 * fTemp8));
																																														}
																																															if (iSlow35 & iSlow77) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32))))))))))))) != 0 {
																																																fTemp142 = F32::max(0.0, F32::min(1.0, F32::max(0.0, F32::min(1.0, fSlow225 * fTemp4 + if iSlow220 != 0 {fSlow224 * fTemp141} else {self.fRec36[0]} + fSlow217 * fTemp5))));
																																															}
																																																if (iSlow35 & iSlow53) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32))) != 0 {
																																																	fTemp143 = (1.0 - fTemp142) * fTemp140;
																																																}
																																																	if iSlow35 & iSlow32 != 0 {
																																																		fTemp144 = fSlow31 * (fTemp143 + fTemp142 * if (fTemp140 > 0.0) as i32 != 0 {1.0 - fTemp140} else {-1.0 - fTemp140});
																																																	}
																																																		if (iSlow35 & iSlow53) | (iSlow35 & iSlow38) != 0 {
																																																			fTemp145 = fSlow227 * fTemp27;
																																																			iTemp146 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp145 > self.fConst11) as i32) + ((fTemp145 > self.fConst17) as i32), (fTemp145 > self.fConst16) as i32), (fTemp145 > self.fConst15) as i32), (fTemp145 > self.fConst14) as i32), (fTemp145 > self.fConst13) as i32), (fTemp145 > self.fConst12) as i32);
																																																			iTemp147 = (iTemp146 >= 4) as i32;
																																																			iTemp148 = (iTemp146 >= 2) as i32;
																																																			iTemp149 = (iTemp146 >= 1) as i32;
																																																			iTemp150 = (iTemp146 >= 3) as i32;
																																																			iTemp151 = if iTemp147 != 0 {64} else {if iTemp148 != 0 {if iTemp150 != 0 {128} else {256}} else {if iTemp149 != 0 {512} else {1024}}};
																																																			iTemp152 = (self.fRec35[0] < 0.5) as i32;
																																																			fTemp153 = (iTemp151) as F32;
																																																		}
																																																			if iSlow35 & iSlow38 != 0 {
																																																				fTemp154 = fTemp153 * if iTemp152 != 0 {2.0 * self.fRec35[0]} else {2.0 * (1.0 - self.fRec35[0])};
																																																				iTemp155 = (fTemp154) as i32;
																																																			}
																																																				if (iSlow35 & iSlow53) | (iSlow35 & iSlow38) != 0 {
																																																					iTemp156 = if iTemp147 != 0 {if (iTemp146 >= 6) as i32 != 0 {if (iTemp146 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp146 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp148 != 0 {if iTemp150 != 0 {1792} else {1536}} else {if iTemp149 != 0 {1024} else {0}}};
																																																				}
																																																					if iSlow35 & iSlow38 != 0 {
																																																						iTemp157 = i32::wrapping_add(iTemp156, (i32::wrapping_add(iTemp155, 1)) % iTemp151);
																																																						fTemp158 = (iTemp155) as F32;
																																																						fTemp159 = fTemp154 - fTemp158;
																																																						iTemp160 = i32::wrapping_add(iTemp156, iTemp155);
																																																						fTemp161 = fTemp158 + (1.0 - fTemp154);
																																																						fTemp162 = fSlow37 * (fTemp143 + fTemp142 * (fTemp139 * (fTemp161 * wt_sample(i32::wrapping_add(iTemp138, iTemp160)) + fTemp159 * wt_sample(i32::wrapping_add(iTemp138, iTemp157))) + fTemp137 * (fTemp161 * wt_sample(i32::wrapping_add(iTemp118, iTemp160)) + fTemp159 * wt_sample(i32::wrapping_add(iTemp118, iTemp157)))));
																																																					}
																																																						if iSlow35 & iSlow41 != 0 {
																																																							fTemp163 = fSlow40 * (1.0 - fTemp142 * (1.0 - F32::sin(6.2831855 * self.fRec35[0] * (7.0 * fTemp142 + 1.0)))) * fTemp140;
																																																						}
																																																							if iSlow35 & iSlow44 != 0 {
																																																								fTemp164 = 2.0 * fTemp142 + 1.0;
																																																								fTemp165 = fSlow214 * fTemp27 * fTemp164;
																																																								iTemp166 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp165 > self.fConst11) as i32) + ((fTemp165 > self.fConst17) as i32), (fTemp165 > self.fConst16) as i32), (fTemp165 > self.fConst15) as i32), (fTemp165 > self.fConst14) as i32), (fTemp165 > self.fConst13) as i32), (fTemp165 > self.fConst12) as i32);
																																																								iTemp167 = (iTemp166 >= 4) as i32;
																																																								iTemp168 = (iTemp166 >= 2) as i32;
																																																								iTemp169 = (iTemp166 >= 1) as i32;
																																																								iTemp170 = (iTemp166 >= 3) as i32;
																																																								iTemp171 = if iTemp167 != 0 {64} else {if iTemp168 != 0 {if iTemp170 != 0 {128} else {256}} else {if iTemp169 != 0 {512} else {1024}}};
																																																								fTemp172 = F32::powf(self.fRec35[0], fTemp164) * (iTemp171) as F32;
																																																								iTemp173 = (fTemp172) as i32;
																																																								iTemp174 = if iTemp167 != 0 {if (iTemp166 >= 6) as i32 != 0 {if (iTemp166 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp166 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp168 != 0 {if iTemp170 != 0 {1792} else {1536}} else {if iTemp169 != 0 {1024} else {0}}};
																																																								iTemp175 = i32::wrapping_add(iTemp174, (i32::wrapping_add(iTemp173, 1)) % iTemp171);
																																																								fTemp176 = (iTemp173) as F32;
																																																								fTemp177 = fTemp172 - fTemp176;
																																																								iTemp178 = i32::wrapping_add(iTemp173, iTemp174);
																																																								fTemp179 = fTemp176 + (1.0 - fTemp172);
																																																								fTemp180 = fSlow43 * (fTemp139 * (fTemp179 * wt_sample(i32::wrapping_add(iTemp138, iTemp178)) + fTemp177 * wt_sample(i32::wrapping_add(iTemp138, iTemp175))) + fTemp137 * (fTemp179 * wt_sample(i32::wrapping_add(iTemp118, iTemp178)) + fTemp177 * wt_sample(i32::wrapping_add(iTemp118, iTemp175))));
																																																							}
																																																								if iSlow35 & iSlow47 != 0 {
																																																									fTemp181 = fSlow46 * (fTemp143 + fTemp142 * F32::sin(7.8539815 * fTemp140));
																																																								}
																																																									if iSlow35 & iSlow50 != 0 {
																																																										fTemp182 = 0.25 * ((4.0 * fTemp142 + 1.0) * fTemp140 + 1.0);
																																																										fTemp183 = fSlow49 * (1.0 - 4.0 * F32::abs(fTemp182 + (-0.5 - F32::floor(fTemp182))));
																																																									}
																																																										if iSlow35 & iSlow53 != 0 {
																																																											fTemp184 = 2.0 * self.fRec35[0] * fTemp153;
																																																											iTemp185 = (fTemp184) as i32;
																																																											iTemp186 = i32::wrapping_add(iTemp156, (i32::wrapping_add(iTemp185, 1)) % iTemp151);
																																																											fTemp187 = (iTemp185) as F32;
																																																											fTemp188 = fTemp184 - fTemp187;
																																																											iTemp189 = i32::wrapping_add(iTemp185, iTemp156);
																																																											fTemp190 = fTemp187 + (1.0 - fTemp184);
																																																											fTemp191 = fSlow52 * (fTemp143 + (iTemp152) as F32 * fTemp142 * F32::abs(fTemp139 * (fTemp190 * wt_sample(i32::wrapping_add(iTemp138, iTemp189)) + fTemp188 * wt_sample(i32::wrapping_add(iTemp138, iTemp186))) + fTemp137 * (fTemp190 * wt_sample(i32::wrapping_add(iTemp118, iTemp189)) + fTemp188 * wt_sample(i32::wrapping_add(iTemp118, iTemp186)))));
																																																										}
																																																											if (iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | (iSlow35 & iSlow56)) != 0 {
																																																												fTemp192 = if iTemp15 != 0 {0.0} else {self.fRec38[1] + self.fConst3 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow235, fTemp142)), 0.0)};
																																																												self.fRec38[0] = fTemp192 - F32::floor(fTemp192);
																																																											}
																																																												if (iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | (iSlow35 & iSlow56))))) != 0 {
																																																													fTemp193 = F32::max(0.0, F32::min(1.0, F32::max(0.0, F32::min(1.0, fTemp114 + if iTemp106 != 0 {0.0} else {if iTemp109 != 0 {0.5 * ((F32::exp(fTemp113 * fSlow237 / fTemp107) + -1.0) / fSlow238)} else {0.5 * (1.0 - fTemp112 + 1.0)}} + fTemp115))));
																																																													iTemp194 = (fTemp193 > 0.5) as i32;
																																																													iTemp195 = i32::wrapping_mul(2176, (if iTemp194 != 0 {fSlow212} else {fSlow211}) as i32);
																																																													fTemp196 = 2.0 * fTemp193;
																																																													fTemp197 = if iTemp194 != 0 {fTemp196 + -1.0} else {fTemp196};
																																																													iTemp198 = i32::wrapping_mul(2176, (if iTemp194 != 0 {fSlow211} else {fSlow216}) as i32);
																																																													fTemp199 = (1.0 - fTemp197) * (fTemp135 * wt_sample(i32::wrapping_add(iTemp130, i32::wrapping_add(iTemp128, iTemp198))) + fTemp133 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp198, iTemp130), iTemp129))) + fTemp197 * (fTemp135 * wt_sample(i32::wrapping_add(iTemp195, iTemp134)) + fTemp133 * wt_sample(i32::wrapping_add(iTemp195, iTemp131)));
																																																												}
																																																													if (iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | (iSlow35 & iSlow56)) != 0 {
																																																														self.fRec37[0] = if (self.fRec38[0] < self.fRec38[1]) as i32 != 0 {fTemp199} else {self.fRec37[1]};
																																																													}
																																																														if iSlow35 & iSlow56 != 0 {
																																																															fTemp200 = fSlow55 * self.fRec37[0];
																																																														}
																																																															if iSlow35 & iSlow59 != 0 {
																																																																fTemp201 = fSlow58 * self.fRec37[0];
																																																															}
																																																																if iSlow35 & iSlow62 != 0 {
																																																																	fTemp202 = fSlow61 * self.fRec37[0];
																																																																}
																																																																	if (iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | (iSlow35 & iSlow65)) != 0 {
																																																																		fTemp203 = F32::tan(self.fConst20 * F32::powf(fSlow243, fTemp142));
																																																																		fTemp204 = 1.0 / fTemp203;
																																																																		fTemp205 = (fTemp204 + 1.4142135) / fTemp203 + 1.0;
																																																																		self.fRec39[0] = fTemp199 - (self.fRec39[2] * ((fTemp204 + -1.4142135) / fTemp203 + 1.0) + 2.0 * self.fRec39[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp203))) / fTemp205;
																																																																		fTemp206 = (self.fRec39[2] + self.fRec39[0] + 2.0 * self.fRec39[1]) / fTemp205;
																																																																	}
																																																																		if iSlow35 & iSlow65 != 0 {
																																																																			fTemp207 = fSlow64 * fTemp206;
																																																																		}
																																																																			if iSlow35 & iSlow68 != 0 {
																																																																				fTemp208 = fSlow67 * fTemp206;
																																																																			}
																																																																				if iSlow35 & iSlow71 != 0 {
																																																																					fTemp209 = fSlow70 * fTemp206;
																																																																				}
																																																																					if iSlow35 & iSlow74 != 0 {
																																																																						fTemp210 = fSlow73 * fTemp140;
																																																																					}
																																																																						if iSlow35 & iSlow77 != 0 {
																																																																							fTemp211 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp142)) + -1.0));
																																																																							fTemp212 = fSlow76 * (F32::floor(fTemp211 * fTemp140 + 0.5) / fTemp211);
																																																																						}
																																																																							if iSlow35 != 0 {
																																																																								fTemp213 = fSlow34 * (fSlow248 * fTemp212 + fSlow247 * fTemp210 + fSlow246 * fTemp209 + fSlow245 * fTemp208 + fSlow244 * fTemp207 + fSlow241 * fTemp202 + fSlow240 * fTemp201 + fSlow239 * fTemp200 + fSlow233 * fTemp191 + fSlow232 * fTemp183 + fSlow231 * fTemp181 + fSlow230 * fTemp180 + fSlow229 * fTemp163 + fSlow228 * fTemp162 + fSlow226 * fTemp144);
																																																																							}
																																																																								*output1 = fSlow249 * fSlow185 * fTemp213;
																																																																								*output2 = fSlow186 * fTemp103;
																																																																								*output3 = fSlow249 * fTemp213;
																																																																								if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
																																																																									self.fVec0[1] = self.fVec0[0];
																																																																									self.iVec1[1] = self.iVec1[0];
																																																																									self.fRec1[1] = self.fRec1[0];
																																																																									self.fRec2[1] = self.fRec2[0];
																																																																									self.fRec3[1] = self.fRec3[0];
																																																																									self.fRec4[1] = self.fRec4[0];
																																																																									self.fRec5[1] = self.fRec5[0];
																																																																									self.fRec6[1] = self.fRec6[0];
																																																																								}
																																																																									if (iSlow2 & (iSlow24 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow15 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow18 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow21 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow27 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow12 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow9 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow6 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow77 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow74 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow71 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow68 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow65 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow62 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow59 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow56 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow53 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow50 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow47 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow44 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow41 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow38 & ((fTemp14 != 0.0) as i32))) | (iSlow35 & (iSlow32 & ((fTemp14 != 0.0) as i32)))))))))))))))))))))))) != 0 {
																																																																										self.fRec8[1] = self.fRec8[0];
																																																																										self.fRec9[1] = self.fRec9[0];
																																																																										self.iRec11[1] = self.iRec11[0];
																																																																										self.fRec10[1] = self.fRec10[0];
																																																																										self.fRec12[1] = self.fRec12[0];
																																																																										self.fRec15[1] = self.fRec15[0];
																																																																									}
																																																																										if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
																																																																											self.iRec16[1] = self.iRec16[0];
																																																																										}
																																																																											if (iSlow2 & (iSlow24 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow15 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow18 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow21 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow27 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow12 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow9 & ((fTemp14 != 0.0) as i32))) | ((iSlow2 & (iSlow6 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow77 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow74 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow71 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow68 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow65 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow62 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow59 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow56 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow53 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow50 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow47 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow44 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow41 & ((fTemp14 != 0.0) as i32))) | ((iSlow35 & (iSlow38 & ((fTemp14 != 0.0) as i32))) | (iSlow35 & (iSlow32 & ((fTemp14 != 0.0) as i32)))))))))))))))))))))))) != 0 {
																																																																												self.fRec14[1] = self.fRec14[0];
																																																																												self.fVec3[1] = self.fVec3[0];
																																																																												self.fRec13[1] = self.fRec13[0];
																																																																											}
																																																																												if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | ((iSlow2 & iSlow6) | ((iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))))))))))) != 0 {
																																																																													self.fRec17[1] = self.fRec17[0];
																																																																													self.fRec18[1] = self.fRec18[0];
																																																																													self.fRec19[1] = self.fRec19[0];
																																																																													self.fRec20[1] = self.fRec20[0];
																																																																												}
																																																																													if (iSlow2 & iSlow27) | ((iSlow2 & iSlow24) | ((iSlow2 & iSlow21) | ((iSlow2 & iSlow18) | ((iSlow2 & iSlow15) | ((iSlow2 & iSlow12) | ((iSlow2 & iSlow9) | (iSlow2 & iSlow6))))))) != 0 {
																																																																														self.fRec0[1] = self.fRec0[0];
																																																																														self.fRec21[1] = self.fRec21[0];
																																																																														self.fRec22[1] = self.fRec22[0];
																																																																													}
																																																																														if (iSlow2 & iSlow12) | (iSlow2 & iSlow9) != 0 {
																																																																															self.iRec23[1] = self.iRec23[0];
																																																																														}
																																																																															if iSlow2 & iSlow27 != 0 {
																																																																																self.fRec24[1] = self.fRec24[0];
																																																																															}
																																																																																if iSlow2 & iSlow18 != 0 {
																																																																																	self.fRec25[1] = self.fRec25[0];
																																																																																	self.fRec26[1] = self.fRec26[0];
																																																																																}
																																																																																	if iSlow2 & iSlow24 != 0 {
																																																																																		self.fVec4[1] = self.fVec4[0];
																																																																																		self.fRec27[1] = self.fRec27[0];
																																																																																	}
																																																																																		if iSlow2 & iSlow15 != 0 {
																																																																																			self.fRec28[1] = self.fRec28[0];
																																																																																			self.fRec29[1] = self.fRec29[0];
																																																																																			self.fRec30[1] = self.fRec30[0];
																																																																																			self.fRec31[1] = self.fRec31[0];
																																																																																			self.fRec32[1] = self.fRec32[0];
																																																																																			self.fRec33[1] = self.fRec33[0];
																																																																																		}
																																																																																			if (iSlow35 & iSlow77) | ((iSlow35 & iSlow74) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32)))))))))))))) != 0 {
																																																																																				self.iRec34[1] = self.iRec34[0];
																																																																																				self.fRec35[1] = self.fRec35[0];
																																																																																			}
																																																																																				if (iSlow35 & iSlow77) | ((iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | ((iSlow35 & iSlow65) | ((iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | ((iSlow35 & iSlow56) | ((iSlow35 & iSlow53) | ((iSlow35 & iSlow50) | ((iSlow35 & iSlow47) | ((iSlow35 & iSlow44) | ((iSlow35 & iSlow41) | ((iSlow35 & iSlow38) | (iSlow35 & iSlow32))))))))))))) != 0 {
																																																																																					self.fRec36[1] = self.fRec36[0];
																																																																																				}
																																																																																					if (iSlow35 & iSlow62) | ((iSlow35 & iSlow59) | (iSlow35 & iSlow56)) != 0 {
																																																																																						self.fRec38[1] = self.fRec38[0];
																																																																																						self.fRec37[1] = self.fRec37[0];
																																																																																					}
																																																																																						if (iSlow35 & iSlow71) | ((iSlow35 & iSlow68) | (iSlow35 & iSlow65)) != 0 {
																																																																																							self.fRec39[2] = self.fRec39[1];
																																																																																							self.fRec39[1] = self.fRec39[0];
																																																																																						}
																																																																																						}
																																																																																						self.iTempPerm0 = iTemp0;
																																																																																						self.iTempPerm1 = iTemp1;
																																																																																						self.fTempPerm2 = fTemp2;
																																																																																						self.fTempPerm3 = fTemp3;
																																																																																						self.fTempPerm4 = fTemp4;
																																																																																						self.fTempPerm5 = fTemp5;
																																																																																						self.fTempPerm6 = fTemp6;
																																																																																						self.fTempPerm7 = fTemp7;
																																																																																						self.fTempPerm8 = fTemp8;
																																																																																						self.fTempPerm9 = fTemp9;
																																																																																						self.fTempPerm10 = fTemp10;
																																																																																						self.fTempPerm11 = fTemp11;
																																																																																						self.fTempPerm12 = fTemp12;
																																																																																						self.iTempPerm13 = iTemp13;
																																																																																						self.fTempPerm14 = fTemp14;
																																																																																						self.iTempPerm15 = iTemp15;
																																																																																						self.fTempPerm16 = fTemp16;
																																																																																						self.fTempPerm17 = fTemp17;
																																																																																						self.iTempPerm18 = iTemp18;
																																																																																						self.fTempPerm19 = fTemp19;
																																																																																						self.fTempPerm20 = fTemp20;
																																																																																						self.fTempPerm21 = fTemp21;
																																																																																						self.fTempPerm22 = fTemp22;
																																																																																						self.fTempPerm23 = fTemp23;
																																																																																						self.fTempPerm24 = fTemp24;
																																																																																						self.iTempPerm25 = iTemp25;
																																																																																						self.fTempPerm26 = fTemp26;
																																																																																						self.fTempPerm27 = fTemp27;
																																																																																						self.fTempPerm28 = fTemp28;
																																																																																						self.fTempPerm29 = fTemp29;
																																																																																						self.fTempPerm30 = fTemp30;
																																																																																						self.fTempPerm31 = fTemp31;
																																																																																						self.fTempPerm32 = fTemp32;
																																																																																						self.fTempPerm33 = fTemp33;
																																																																																						self.iTempPerm34 = iTemp34;
																																																																																						self.fTempPerm35 = fTemp35;
																																																																																						self.fTempPerm36 = fTemp36;
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
																																																																																						self.fTempPerm51 = fTemp51;
																																																																																						self.fTempPerm52 = fTemp52;
																																																																																						self.iTempPerm53 = iTemp53;
																																																																																						self.iTempPerm54 = iTemp54;
																																																																																						self.fTempPerm55 = fTemp55;
																																																																																						self.fTempPerm56 = fTemp56;
																																																																																						self.fTempPerm57 = fTemp57;
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
																																																																																						self.fTempPerm77 = fTemp77;
																																																																																						self.fTempPerm78 = fTemp78;
																																																																																						self.iTempPerm79 = iTemp79;
																																																																																						self.fTempPerm80 = fTemp80;
																																																																																						self.fTempPerm81 = fTemp81;
																																																																																						self.iTempPerm82 = iTemp82;
																																																																																						self.fTempPerm83 = fTemp83;
																																																																																						self.fTempPerm84 = fTemp84;
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
																																																																																						self.fTempPerm96 = fTemp96;
																																																																																						self.fTempPerm97 = fTemp97;
																																																																																						self.fTempPerm98 = fTemp98;
																																																																																						self.fTempPerm99 = fTemp99;
																																																																																						self.fTempPerm100 = fTemp100;
																																																																																						self.fTempPerm101 = fTemp101;
																																																																																						self.fTempPerm102 = fTemp102;
																																																																																						self.fTempPerm103 = fTemp103;
																																																																																						self.fTempPerm104 = fTemp104;
																																																																																						self.fTempPerm105 = fTemp105;
																																																																																						self.iTempPerm106 = iTemp106;
																																																																																						self.fTempPerm107 = fTemp107;
																																																																																						self.fTempPerm108 = fTemp108;
																																																																																						self.iTempPerm109 = iTemp109;
																																																																																						self.fTempPerm110 = fTemp110;
																																																																																						self.fTempPerm111 = fTemp111;
																																																																																						self.fTempPerm112 = fTemp112;
																																																																																						self.fTempPerm113 = fTemp113;
																																																																																						self.fTempPerm114 = fTemp114;
																																																																																						self.fTempPerm115 = fTemp115;
																																																																																						self.fTempPerm116 = fTemp116;
																																																																																						self.iTempPerm117 = iTemp117;
																																																																																						self.iTempPerm118 = iTemp118;
																																																																																						self.fTempPerm119 = fTemp119;
																																																																																						self.iTempPerm120 = iTemp120;
																																																																																						self.iTempPerm121 = iTemp121;
																																																																																						self.iTempPerm122 = iTemp122;
																																																																																						self.iTempPerm123 = iTemp123;
																																																																																						self.iTempPerm124 = iTemp124;
																																																																																						self.iTempPerm125 = iTemp125;
																																																																																						self.fTempPerm126 = fTemp126;
																																																																																						self.fTempPerm127 = fTemp127;
																																																																																						self.iTempPerm128 = iTemp128;
																																																																																						self.iTempPerm129 = iTemp129;
																																																																																						self.iTempPerm130 = iTemp130;
																																																																																						self.iTempPerm131 = iTemp131;
																																																																																						self.fTempPerm132 = fTemp132;
																																																																																						self.fTempPerm133 = fTemp133;
																																																																																						self.iTempPerm134 = iTemp134;
																																																																																						self.fTempPerm135 = fTemp135;
																																																																																						self.fTempPerm136 = fTemp136;
																																																																																						self.fTempPerm137 = fTemp137;
																																																																																						self.iTempPerm138 = iTemp138;
																																																																																						self.fTempPerm139 = fTemp139;
																																																																																						self.fTempPerm140 = fTemp140;
																																																																																						self.fTempPerm141 = fTemp141;
																																																																																						self.fTempPerm142 = fTemp142;
																																																																																						self.fTempPerm143 = fTemp143;
																																																																																						self.fTempPerm144 = fTemp144;
																																																																																						self.fTempPerm145 = fTemp145;
																																																																																						self.iTempPerm146 = iTemp146;
																																																																																						self.iTempPerm147 = iTemp147;
																																																																																						self.iTempPerm148 = iTemp148;
																																																																																						self.iTempPerm149 = iTemp149;
																																																																																						self.iTempPerm150 = iTemp150;
																																																																																						self.iTempPerm151 = iTemp151;
																																																																																						self.iTempPerm152 = iTemp152;
																																																																																						self.fTempPerm153 = fTemp153;
																																																																																						self.fTempPerm154 = fTemp154;
																																																																																						self.iTempPerm155 = iTemp155;
																																																																																						self.iTempPerm156 = iTemp156;
																																																																																						self.iTempPerm157 = iTemp157;
																																																																																						self.fTempPerm158 = fTemp158;
																																																																																						self.fTempPerm159 = fTemp159;
																																																																																						self.iTempPerm160 = iTemp160;
																																																																																						self.fTempPerm161 = fTemp161;
																																																																																						self.fTempPerm162 = fTemp162;
																																																																																						self.fTempPerm163 = fTemp163;
																																																																																						self.fTempPerm164 = fTemp164;
																																																																																						self.fTempPerm165 = fTemp165;
																																																																																						self.iTempPerm166 = iTemp166;
																																																																																						self.iTempPerm167 = iTemp167;
																																																																																						self.iTempPerm168 = iTemp168;
																																																																																						self.iTempPerm169 = iTemp169;
																																																																																						self.iTempPerm170 = iTemp170;
																																																																																						self.iTempPerm171 = iTemp171;
																																																																																						self.fTempPerm172 = fTemp172;
																																																																																						self.iTempPerm173 = iTemp173;
																																																																																						self.iTempPerm174 = iTemp174;
																																																																																						self.iTempPerm175 = iTemp175;
																																																																																						self.fTempPerm176 = fTemp176;
																																																																																						self.fTempPerm177 = fTemp177;
																																																																																						self.iTempPerm178 = iTemp178;
																																																																																						self.fTempPerm179 = fTemp179;
																																																																																						self.fTempPerm180 = fTemp180;
																																																																																						self.fTempPerm181 = fTemp181;
																																																																																						self.fTempPerm182 = fTemp182;
																																																																																						self.fTempPerm183 = fTemp183;
																																																																																						self.fTempPerm184 = fTemp184;
																																																																																						self.iTempPerm185 = iTemp185;
																																																																																						self.iTempPerm186 = iTemp186;
																																																																																						self.fTempPerm187 = fTemp187;
																																																																																						self.fTempPerm188 = fTemp188;
																																																																																						self.iTempPerm189 = iTemp189;
																																																																																						self.fTempPerm190 = fTemp190;
																																																																																						self.fTempPerm191 = fTemp191;
																																																																																						self.fTempPerm192 = fTemp192;
																																																																																						self.fTempPerm193 = fTemp193;
																																																																																						self.iTempPerm194 = iTemp194;
																																																																																						self.iTempPerm195 = iTemp195;
																																																																																						self.fTempPerm196 = fTemp196;
																																																																																						self.fTempPerm197 = fTemp197;
																																																																																						self.iTempPerm198 = iTemp198;
																																																																																						self.fTempPerm199 = fTemp199;
																																																																																						self.fTempPerm200 = fTemp200;
																																																																																						self.fTempPerm201 = fTemp201;
																																																																																						self.fTempPerm202 = fTemp202;
																																																																																						self.fTempPerm203 = fTemp203;
																																																																																						self.fTempPerm204 = fTemp204;
																																																																																						self.fTempPerm205 = fTemp205;
																																																																																						self.fTempPerm206 = fTemp206;
																																																																																						self.fTempPerm207 = fTemp207;
																																																																																						self.fTempPerm208 = fTemp208;
																																																																																						self.fTempPerm209 = fTemp209;
																																																																																						self.fTempPerm210 = fTemp210;
																																																																																						self.fTempPerm211 = fTemp211;
																																																																																						self.fTempPerm212 = fTemp212;
																																																																																						self.fTempPerm213 = fTemp213;
																																																																																					}

}

