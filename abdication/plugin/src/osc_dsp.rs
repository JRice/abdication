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
	iVec5: [i32;2],
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
		for l11 in 0..2 {
			self.iVec5[l11 as usize] = 0;
		}
		for l12 in 0..2 {
			self.iRec7[l12 as usize] = 0;
		}
	}
	
	fn fillOscDspSIG2(&mut self, count: i32, table: &mut[F32]) {
		for i3 in 0..count {
			self.iVec5[0] = 1;
			self.iRec7[0] = (i32::wrapping_add(self.iVec5[1], self.iRec7[1])) % 65536;
			table[i3 as usize] = F32::sin(9.58738e-05 * (self.iRec7[0]) as F32);
			self.iVec5[1] = self.iVec5[0];
			self.iRec7[1] = self.iRec7[0];
		}
	}

}


pub fn newOscDspSIG2() -> OscDspSIG2 { 
	OscDspSIG2 {
		iVec5: [0;2],
		iRec7: [0;2],
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
	fEntry3: F32,
	fHslider3: F32,
	fVec3: [F32;2],
	fRec3: [F32;2],
	fRec4: [F32;2],
	fHslider4: F32,
	fVec4: [F32;2],
	fRec5: [F32;2],
	fRec6: [F32;2],
	fEntry4: F32,
	fEntry5: F32,
	fVslider1: F32,
	iTempPerm0: i32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fTempPerm1: F32,
	fRec8: [F32;2],
	fHslider5: F32,
	fConst2: F32,
	fTempPerm2: F32,
	fRec9: [F32;2],
	iRec11: [i32;2],
	iTempPerm3: i32,
	fConst3: F32,
	fConst5: F32,
	fRec10: [F32;2],
	fConst6: F32,
	fHslider6: F32,
	fRec12: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fTempPerm4: F32,
	fRec15: [F32;2],
	iRec16: [i32;2],
	fVec6: [F32;2],
	fRec14: [F32;2],
	fTempPerm5: F32,
	fVec7: [F32;2],
	fRec13: [F32;2],
	fTempPerm6: F32,
	fHslider7: F32,
	fRec17: [F32;2],
	fEntry6: F32,
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
	fCheckbox0: F32,
	fEntry7: F32,
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
	fEntry8: F32,
	fEntry9: F32,
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
	fEntry10: F32,
	fEntry11: F32,
	fEntry12: F32,
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
	fEntry13: F32,
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
	fEntry14: F32,
	fEntry15: F32,
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
	fEntry16: F32,
	fEntry17: F32,
	fEntry18: F32,
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
	fEntry19: F32,
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
	fHslider22: F32,
	fHslider23: F32,
	fHslider24: F32,
	fHslider25: F32,
	fHslider26: F32,
	fHslider27: F32,
	fHslider28: F32,
	fEntry20: F32,
	fEntry21: F32,
	fHslider29: F32,
	fHslider30: F32,
	fHslider31: F32,
	fHslider32: F32,
	fHslider33: F32,
	fHslider34: F32,
	fHslider35: F32,
	fEntry22: F32,
	fEntry23: F32,
	fHslider36: F32,
	fHslider37: F32,
	fHslider38: F32,
	fHslider39: F32,
	fHslider40: F32,
	fHslider41: F32,
	fHslider42: F32,
	fHslider43: F32,
	fCheckbox1: F32,
	fEntry24: F32,
	fEntry25: F32,
	fHslider44: F32,
	fHslider45: F32,
	fHslider46: F32,
	fHslider47: F32,
	fHslider48: F32,
	fHslider49: F32,
	fHslider50: F32,
	fHslider51: F32,
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
			fEntry3: 0.0,
			fHslider3: 0.0,
			fVec3: [0.0;2],
			fRec3: [0.0;2],
			fRec4: [0.0;2],
			fHslider4: 0.0,
			fVec4: [0.0;2],
			fRec5: [0.0;2],
			fRec6: [0.0;2],
			fEntry4: 0.0,
			fEntry5: 0.0,
			fVslider1: 0.0,
			iTempPerm0: 0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fTempPerm1: 0.0,
			fRec8: [0.0;2],
			fHslider5: 0.0,
			fConst2: 0.0,
			fTempPerm2: 0.0,
			fRec9: [0.0;2],
			iRec11: [0;2],
			iTempPerm3: 0,
			fConst3: 0.0,
			fConst5: 0.0,
			fRec10: [0.0;2],
			fConst6: 0.0,
			fHslider6: 0.0,
			fRec12: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fTempPerm4: 0.0,
			fRec15: [0.0;2],
			iRec16: [0;2],
			fVec6: [0.0;2],
			fRec14: [0.0;2],
			fTempPerm5: 0.0,
			fVec7: [0.0;2],
			fRec13: [0.0;2],
			fTempPerm6: 0.0,
			fHslider7: 0.0,
			fRec17: [0.0;2],
			fEntry6: 0.0,
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
			fCheckbox0: 0.0,
			fEntry7: 0.0,
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
			fEntry8: 0.0,
			fEntry9: 0.0,
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
			fEntry10: 0.0,
			fEntry11: 0.0,
			fEntry12: 0.0,
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
			fEntry13: 0.0,
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
			fEntry14: 0.0,
			fEntry15: 0.0,
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
			fEntry16: 0.0,
			fEntry17: 0.0,
			fEntry18: 0.0,
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
			fEntry19: 0.0,
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
			fHslider22: 0.0,
			fHslider23: 0.0,
			fHslider24: 0.0,
			fHslider25: 0.0,
			fHslider26: 0.0,
			fHslider27: 0.0,
			fHslider28: 0.0,
			fEntry20: 0.0,
			fEntry21: 0.0,
			fHslider29: 0.0,
			fHslider30: 0.0,
			fHslider31: 0.0,
			fHslider32: 0.0,
			fHslider33: 0.0,
			fHslider34: 0.0,
			fHslider35: 0.0,
			fEntry22: 0.0,
			fEntry23: 0.0,
			fHslider36: 0.0,
			fHslider37: 0.0,
			fHslider38: 0.0,
			fHslider39: 0.0,
			fHslider40: 0.0,
			fHslider41: 0.0,
			fHslider42: 0.0,
			fHslider43: 0.0,
			fCheckbox1: 0.0,
			fEntry24: 0.0,
			fEntry25: 0.0,
			fHslider44: 0.0,
			fHslider45: 0.0,
			fHslider46: 0.0,
			fHslider47: 0.0,
			fHslider48: 0.0,
			fHslider49: 0.0,
			fHslider50: 0.0,
			fHslider51: 0.0,
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
		self.fEntry2 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 64.0;
		self.fHslider2 = 3.0;
		self.fEntry3 = 0.0;
		self.fHslider3 = 6.0;
		self.fHslider4 = 0.0;
		self.fEntry4 = 0.0;
		self.fEntry5 = 1.0;
		self.fVslider1 = 0.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 1.0;
		self.fHslider7 = 0.0;
		self.fEntry6 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 1.1e+02;
		self.fHslider11 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fEntry7 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.5;
		self.fEntry8 = 2.0;
		self.fEntry9 = 2.0;
		self.fHslider14 = 0.0;
		self.fEntry10 = 4e+01;
		self.fEntry11 = 8e+01;
		self.fEntry12 = 118.0;
		self.fEntry13 = 0.0;
		self.fHslider15 = 0.5;
		self.fHslider16 = 0.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 0.5;
		self.fEntry14 = 2.0;
		self.fEntry15 = 2.0;
		self.fHslider19 = 0.0;
		self.fEntry16 = 4e+01;
		self.fEntry17 = 8e+01;
		self.fEntry18 = 118.0;
		self.fEntry19 = 0.0;
		self.fHslider20 = 0.5;
		self.fHslider21 = 0.0;
		self.fHslider22 = 1e+02;
		self.fHslider23 = 0.0;
		self.fHslider24 = 0.5;
		self.fHslider25 = 1e+02;
		self.fHslider26 = 1e+02;
		self.fHslider27 = 0.0;
		self.fHslider28 = 0.0;
		self.fEntry20 = 31.0;
		self.fEntry21 = 52.0;
		self.fHslider29 = 0.0;
		self.fHslider30 = 0.0;
		self.fHslider31 = 0.5;
		self.fHslider32 = 1e+02;
		self.fHslider33 = 1e+02;
		self.fHslider34 = 0.0;
		self.fHslider35 = 0.0;
		self.fEntry22 = 39.0;
		self.fEntry23 = 34.0;
		self.fHslider36 = 0.0;
		self.fHslider37 = 0.3;
		self.fHslider38 = 0.0;
		self.fHslider39 = 0.0;
		self.fHslider40 = 0.0;
		self.fHslider41 = 0.0;
		self.fHslider42 = 0.0;
		self.fHslider43 = 0.0;
		self.fCheckbox1 = 0.0;
		self.fEntry24 = 6821.0;
		self.fEntry25 = 3.0;
		self.fHslider44 = 0.0;
		self.fHslider45 = 0.3;
		self.fHslider46 = 0.0;
		self.fHslider47 = 0.0;
		self.fHslider48 = 0.0;
		self.fHslider49 = 0.0;
		self.fHslider50 = 0.0;
		self.fHslider51 = 0.0;
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
		for l8 in 0..2 {
			self.fVec4[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec5[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec6[l10 as usize] = 0.0;
		}
		self.iTempPerm0 = 0;
		self.fTempPerm1 = 0.0;
		for l13 in 0..2 {
			self.fRec8[l13 as usize] = 0.0;
		}
		self.fTempPerm2 = 0.0;
		for l14 in 0..2 {
			self.fRec9[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.iRec11[l15 as usize] = 0;
		}
		self.iTempPerm3 = 0;
		for l16 in 0..2 {
			self.fRec10[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec12[l17 as usize] = 0.0;
		}
		self.fTempPerm4 = 0.0;
		for l18 in 0..2 {
			self.fRec15[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.iRec16[l19 as usize] = 0;
		}
		for l20 in 0..2 {
			self.fVec6[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec14[l21 as usize] = 0.0;
		}
		self.fTempPerm5 = 0.0;
		for l22 in 0..2 {
			self.fVec7[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec13[l23 as usize] = 0.0;
		}
		self.fTempPerm6 = 0.0;
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
		ui_interface.declare(Some(ParamIndex(7)), "3", "");
		ui_interface.declare(Some(ParamIndex(7)), "unit", "ms");
		ui_interface.add_horizontal_slider("Ortho Scatter", ParamIndex(7), 1e+02, 0.0, 5e+02, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "2.1", "");
		ui_interface.open_horizontal_box("Dual 1");
		ui_interface.declare(Some(ParamIndex(8)), "1", "");
		ui_interface.declare(Some(ParamIndex(8)), "style", "menu{'VA Sweep':0;'Spread':1;'PWM':2;'Sync':3;'Fractal':4;'Window Amp Sync':5;'Metal':6;'Ring Mod':7;'FM':8;'Noise AM':9;'Chaos':10;'Folding':11}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(8), 0.0, 0.0, 11.0, 1.0);
		ui_interface.declare(Some(ParamIndex(9)), "2", "");
		ui_interface.declare(Some(ParamIndex(9)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(9), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(10)), "3", "");
		ui_interface.declare(Some(ParamIndex(10)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(10), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(11)), "4", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(11), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "5", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(12), 0.0, 0.0, 1.0, 0.001);
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
		ui_interface.declare(None, "2.4", "");
		ui_interface.open_horizontal_box("Ortho 1");
		ui_interface.declare(Some(ParamIndex(31)), "1", "");
		ui_interface.declare(Some(ParamIndex(31)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Top", ParamIndex(31), 34.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(32)), "2", "");
		ui_interface.declare(Some(ParamIndex(32)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Bottom", ParamIndex(32), 39.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(33)), "3", "");
		ui_interface.add_horizontal_slider("Top Position", ParamIndex(33), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(34)), "4", "");
		ui_interface.add_horizontal_slider("Bottom Position", ParamIndex(34), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(35)), "5", "");
		ui_interface.declare(Some(ParamIndex(35)), "unit", "%");
		ui_interface.add_horizontal_slider("Top Speed", ParamIndex(35), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(36)), "6", "");
		ui_interface.declare(Some(ParamIndex(36)), "unit", "%");
		ui_interface.add_horizontal_slider("Bottom Speed", ParamIndex(36), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(37)), "7", "");
		ui_interface.add_horizontal_slider("Mix", ParamIndex(37), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(38)), "8", "");
		ui_interface.declare(Some(ParamIndex(38)), "style", "pitch");
		ui_interface.add_horizontal_slider("Top Pitch", ParamIndex(38), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(39)), "9", "");
		ui_interface.declare(Some(ParamIndex(39)), "style", "pitch");
		ui_interface.add_horizontal_slider("Bottom Pitch", ParamIndex(39), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("Osc 1");
		ui_interface.declare(Some(ParamIndex(40)), "0", "");
		ui_interface.declare(Some(ParamIndex(40)), "style", "menu{'Dual':0;'Wavetable':1;'Ray':2;'Orthogonal':3}");
		ui_interface.add_num_entry("Mode", ParamIndex(40), 0.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(41)), "1", "");
		ui_interface.declare(Some(ParamIndex(41)), "style", "pitch");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(41), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.1", "");
		ui_interface.open_horizontal_box("Dual 2");
		ui_interface.declare(Some(ParamIndex(42)), "1", "");
		ui_interface.declare(Some(ParamIndex(42)), "style", "menu{'VA Sweep':0;'Spread':1;'PWM':2;'Sync':3;'Fractal':4;'Window Amp Sync':5;'Metal':6;'Ring Mod':7;'FM':8;'Noise AM':9;'Chaos':10;'Folding':11}");
		ui_interface.add_num_entry("Algorithm", ParamIndex(42), 0.0, 0.0, 11.0, 1.0);
		ui_interface.declare(Some(ParamIndex(43)), "2", "");
		ui_interface.declare(Some(ParamIndex(43)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(43), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(44)), "3", "");
		ui_interface.declare(Some(ParamIndex(44)), "style", "menu{'Sine':0;'Triangle':1;'Saw':2;'Square':3}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(44), 2.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(45)), "4", "");
		ui_interface.add_horizontal_slider("A", ParamIndex(45), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(46)), "5", "");
		ui_interface.add_horizontal_slider("B", ParamIndex(46), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.2", "");
		ui_interface.open_horizontal_box("WT 2");
		ui_interface.declare(Some(ParamIndex(47)), "1", "");
		ui_interface.declare(Some(ParamIndex(47)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 1", ParamIndex(47), 0.0, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(48)), "2", "");
		ui_interface.declare(Some(ParamIndex(48)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 2", ParamIndex(48), 4e+01, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(49)), "3", "");
		ui_interface.declare(Some(ParamIndex(49)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 3", ParamIndex(49), 8e+01, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(50)), "4", "");
		ui_interface.declare(Some(ParamIndex(50)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Wave 4", ParamIndex(50), 118.0, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(51)), "5", "");
		ui_interface.add_horizontal_slider("Position", ParamIndex(51), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(52)), "6", "");
		ui_interface.declare(Some(ParamIndex(52)), "style", "menu{'Off':0;'Bits 4':1;'SR 11k':2;'SR 8k':3;'SR 4k':4;'Simplify 6k':5;'Simplify 1.5k':6;'Simplify 800':7;'Rectify':8;'Tri Fold':9;'Sine Fold':10;'Squish':11;'Ring':12;'Mirror':13;'Sign Swap':14}");
		ui_interface.add_num_entry("OscMod", ParamIndex(52), 0.0, 0.0, 14.0, 1.0);
		ui_interface.declare(Some(ParamIndex(53)), "7", "");
		ui_interface.add_horizontal_slider("OscMod Depth", ParamIndex(53), 0.5, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.3", "");
		ui_interface.open_horizontal_box("Ray 2");
		ui_interface.declare(Some(ParamIndex(54)), "10", "");
		ui_interface.declare(Some(ParamIndex(54)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(54)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Orbit Rate", ParamIndex(54), 0.3, 0.02, 1e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(55)), "11", "");
		ui_interface.declare(Some(ParamIndex(55)), "unit", "cents");
		ui_interface.add_horizontal_slider("Fine", ParamIndex(55), 0.0, -1e+02, 1e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(56)), "1", "");
		ui_interface.declare(Some(ParamIndex(56)), "style", "menu{'Earth':0;'Water':1;'Air':2;'Fire':3;'Wood':4;'Metal':5;'Chaos':6}");
		ui_interface.add_num_entry("Element", ParamIndex(56), 3.0, 0.0, 6.0, 1.0);
		ui_interface.declare(Some(ParamIndex(57)), "2", "");
		ui_interface.add_num_entry("Chaos Seed", ParamIndex(57), 6821.0, 1.0, 99999.0, 1.0);
		ui_interface.declare(Some(ParamIndex(58)), "3", "");
		ui_interface.add_check_button("Free", ParamIndex(58));
		ui_interface.declare(Some(ParamIndex(59)), "4", "");
		ui_interface.add_horizontal_slider("Squish", ParamIndex(59), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(60)), "5", "");
		ui_interface.add_horizontal_slider("Skew", ParamIndex(60), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(61)), "6", "");
		ui_interface.add_horizontal_slider("Tilt", ParamIndex(61), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(62)), "7", "");
		ui_interface.add_horizontal_slider("Center", ParamIndex(62), 0.0, -1.3, 1.3, 0.001);
		ui_interface.declare(Some(ParamIndex(63)), "8", "");
		ui_interface.add_horizontal_slider("Ray", ParamIndex(63), 0.0, 0.0, 1.5, 0.001);
		ui_interface.declare(Some(ParamIndex(64)), "9", "");
		ui_interface.add_horizontal_slider("Orbit", ParamIndex(64), 0.0, 0.0, 0.8, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3.4", "");
		ui_interface.open_horizontal_box("Ortho 2");
		ui_interface.declare(Some(ParamIndex(65)), "1", "");
		ui_interface.declare(Some(ParamIndex(65)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Top", ParamIndex(65), 52.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(66)), "2", "");
		ui_interface.declare(Some(ParamIndex(66)), "style", "menu{'Age of Illumination':0;'Dark Pedal':1;'Deep Down':2;'Acidic':3;'Affla':4;'AI-bounce':5;'Ambient reeds':6;'Bel':7;'Belle Star':8;'Carpenter':9;'Catatonic':10;'Celestes of Canada':11;'Chromanium':12;'Delish':13;'Dusseldorf':14;'Ebony and Wire':15;'Fireflies':16;'Granular Bella':17;'Homb':18;'In Time':19;'JD Pulsation':20;'Marvel Bells':21;'Moon Teardrops':22;'NaturalDance':23;'Odysseus':24;'PPG Resosweep':25;'Seahorse':26;'Silvermoon':27;'SimplePoly':28;'SinesOfACrush':29;'SpektmBell':30;'Starlight':31;'Static Wine':32;'The Low-Bit VS':33;'Aurora':34;'Basis jr-bounce':35;'Disclosure-bounce':36;'Earth Rises':37;'Fragmented Space':38;'Galaxy Sleeping':39;'Love Ensemble':40;'Lush Ambient Strings':41;'Medusa':42;'ModifiedJet2':43;'Northern Light':44;'Obi-wan':45;'Origin':46;'Slow Choir':47;'Slow Motion Tron':48;'Tascam Pad':49;'Woozy':50;'Deep Breath JD-bounce':51;'Humming Pad':52;'Longing-bounce':53;'MassiveChoir-bounce':54;'Sense of Reverence':55;'Universe':56}");
		ui_interface.add_num_entry("Bottom", ParamIndex(66), 31.0, 0.0, 56.0, 1.0);
		ui_interface.declare(Some(ParamIndex(67)), "3", "");
		ui_interface.add_horizontal_slider("Top Position", ParamIndex(67), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(68)), "4", "");
		ui_interface.add_horizontal_slider("Bottom Position", ParamIndex(68), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(69)), "5", "");
		ui_interface.declare(Some(ParamIndex(69)), "unit", "%");
		ui_interface.add_horizontal_slider("Top Speed", ParamIndex(69), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(70)), "6", "");
		ui_interface.declare(Some(ParamIndex(70)), "unit", "%");
		ui_interface.add_horizontal_slider("Bottom Speed", ParamIndex(70), 1e+02, 0.0, 2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(71)), "7", "");
		ui_interface.add_horizontal_slider("Mix", ParamIndex(71), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(72)), "8", "");
		ui_interface.declare(Some(ParamIndex(72)), "style", "pitch");
		ui_interface.add_horizontal_slider("Top Pitch", ParamIndex(72), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(73)), "9", "");
		ui_interface.declare(Some(ParamIndex(73)), "style", "pitch");
		ui_interface.add_horizontal_slider("Bottom Pitch", ParamIndex(73), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("Osc 2");
		ui_interface.declare(Some(ParamIndex(74)), "0", "");
		ui_interface.declare(Some(ParamIndex(74)), "style", "menu{'Dual':0;'Wavetable':1;'Ray':2;'Orthogonal':3}");
		ui_interface.add_num_entry("Mode", ParamIndex(74), 1.0, 0.0, 3.0, 1.0);
		ui_interface.declare(Some(ParamIndex(75)), "1", "");
		ui_interface.declare(Some(ParamIndex(75)), "style", "pitch");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(75), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Mix");
		ui_interface.declare(Some(ParamIndex(76)), "1", "");
		ui_interface.add_vertical_slider("Osc 1", ParamIndex(76), 1.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(77)), "2", "");
		ui_interface.add_vertical_slider("Osc 2", ParamIndex(77), 0.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(78)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(78)), "midi", "pitchwheel");
		ui_interface.add_horizontal_slider("bend", ParamIndex(78), 0.0, -1.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("copy", ParamIndex(79), 3.0, 0.0, 6.0, 1.0);
		ui_interface.add_horizontal_slider("freq", ParamIndex(80), 1.1e+02, 2e+01, 8e+03, 0.01);
		ui_interface.add_button("gate", ParamIndex(81));
		ui_interface.add_horizontal_slider("glide", ParamIndex(82), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("ramp", ParamIndex(83), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("seed", ParamIndex(84), 0.0, 0.0, 1.0, 1e-06);
		ui_interface.add_horizontal_slider("slot", ParamIndex(85), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			81 => Some(self.fButton0),
			3 => Some(self.fCheckbox0),
			58 => Some(self.fCheckbox1),
			24 => Some(self.fCheckbox2),
			40 => Some(self.fEntry0),
			8 => Some(self.fEntry1),
			14 => Some(self.fEntry10),
			15 => Some(self.fEntry11),
			16 => Some(self.fEntry12),
			13 => Some(self.fEntry13),
			44 => Some(self.fEntry14),
			43 => Some(self.fEntry15),
			48 => Some(self.fEntry16),
			49 => Some(self.fEntry17),
			50 => Some(self.fEntry18),
			47 => Some(self.fEntry19),
			18 => Some(self.fEntry2),
			66 => Some(self.fEntry20),
			65 => Some(self.fEntry21),
			32 => Some(self.fEntry22),
			31 => Some(self.fEntry23),
			57 => Some(self.fEntry24),
			56 => Some(self.fEntry25),
			23 => Some(self.fEntry26),
			22 => Some(self.fEntry27),
			1 => Some(self.fEntry3),
			52 => Some(self.fEntry4),
			74 => Some(self.fEntry5),
			5 => Some(self.fEntry6),
			42 => Some(self.fEntry7),
			10 => Some(self.fEntry8),
			9 => Some(self.fEntry9),
			41 => Some(self.fHslider0),
			83 => Some(self.fHslider1),
			80 => Some(self.fHslider10),
			84 => Some(self.fHslider11),
			12 => Some(self.fHslider12),
			11 => Some(self.fHslider13),
			17 => Some(self.fHslider14),
			19 => Some(self.fHslider15),
			75 => Some(self.fHslider16),
			46 => Some(self.fHslider17),
			45 => Some(self.fHslider18),
			51 => Some(self.fHslider19),
			79 => Some(self.fHslider2),
			53 => Some(self.fHslider20),
			73 => Some(self.fHslider21),
			7 => Some(self.fHslider22),
			72 => Some(self.fHslider23),
			71 => Some(self.fHslider24),
			70 => Some(self.fHslider25),
			69 => Some(self.fHslider26),
			68 => Some(self.fHslider27),
			67 => Some(self.fHslider28),
			39 => Some(self.fHslider29),
			2 => Some(self.fHslider3),
			38 => Some(self.fHslider30),
			37 => Some(self.fHslider31),
			36 => Some(self.fHslider32),
			35 => Some(self.fHslider33),
			34 => Some(self.fHslider34),
			33 => Some(self.fHslider35),
			55 => Some(self.fHslider36),
			54 => Some(self.fHslider37),
			64 => Some(self.fHslider38),
			63 => Some(self.fHslider39),
			0 => Some(self.fHslider4),
			62 => Some(self.fHslider40),
			61 => Some(self.fHslider41),
			60 => Some(self.fHslider42),
			59 => Some(self.fHslider43),
			21 => Some(self.fHslider44),
			20 => Some(self.fHslider45),
			30 => Some(self.fHslider46),
			29 => Some(self.fHslider47),
			28 => Some(self.fHslider48),
			27 => Some(self.fHslider49),
			85 => Some(self.fHslider5),
			26 => Some(self.fHslider50),
			25 => Some(self.fHslider51),
			6 => Some(self.fHslider6),
			78 => Some(self.fHslider7),
			4 => Some(self.fHslider8),
			82 => Some(self.fHslider9),
			76 => Some(self.fVslider0),
			77 => Some(self.fVslider1),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			81 => { self.fButton0 = value }
			3 => { self.fCheckbox0 = value }
			58 => { self.fCheckbox1 = value }
			24 => { self.fCheckbox2 = value }
			40 => { self.fEntry0 = value }
			8 => { self.fEntry1 = value }
			14 => { self.fEntry10 = value }
			15 => { self.fEntry11 = value }
			16 => { self.fEntry12 = value }
			13 => { self.fEntry13 = value }
			44 => { self.fEntry14 = value }
			43 => { self.fEntry15 = value }
			48 => { self.fEntry16 = value }
			49 => { self.fEntry17 = value }
			50 => { self.fEntry18 = value }
			47 => { self.fEntry19 = value }
			18 => { self.fEntry2 = value }
			66 => { self.fEntry20 = value }
			65 => { self.fEntry21 = value }
			32 => { self.fEntry22 = value }
			31 => { self.fEntry23 = value }
			57 => { self.fEntry24 = value }
			56 => { self.fEntry25 = value }
			23 => { self.fEntry26 = value }
			22 => { self.fEntry27 = value }
			1 => { self.fEntry3 = value }
			52 => { self.fEntry4 = value }
			74 => { self.fEntry5 = value }
			5 => { self.fEntry6 = value }
			42 => { self.fEntry7 = value }
			10 => { self.fEntry8 = value }
			9 => { self.fEntry9 = value }
			41 => { self.fHslider0 = value }
			83 => { self.fHslider1 = value }
			80 => { self.fHslider10 = value }
			84 => { self.fHslider11 = value }
			12 => { self.fHslider12 = value }
			11 => { self.fHslider13 = value }
			17 => { self.fHslider14 = value }
			19 => { self.fHslider15 = value }
			75 => { self.fHslider16 = value }
			46 => { self.fHslider17 = value }
			45 => { self.fHslider18 = value }
			51 => { self.fHslider19 = value }
			79 => { self.fHslider2 = value }
			53 => { self.fHslider20 = value }
			73 => { self.fHslider21 = value }
			7 => { self.fHslider22 = value }
			72 => { self.fHslider23 = value }
			71 => { self.fHslider24 = value }
			70 => { self.fHslider25 = value }
			69 => { self.fHslider26 = value }
			68 => { self.fHslider27 = value }
			67 => { self.fHslider28 = value }
			39 => { self.fHslider29 = value }
			2 => { self.fHslider3 = value }
			38 => { self.fHslider30 = value }
			37 => { self.fHslider31 = value }
			36 => { self.fHslider32 = value }
			35 => { self.fHslider33 = value }
			34 => { self.fHslider34 = value }
			33 => { self.fHslider35 = value }
			55 => { self.fHslider36 = value }
			54 => { self.fHslider37 = value }
			64 => { self.fHslider38 = value }
			63 => { self.fHslider39 = value }
			0 => { self.fHslider4 = value }
			62 => { self.fHslider40 = value }
			61 => { self.fHslider41 = value }
			60 => { self.fHslider42 = value }
			59 => { self.fHslider43 = value }
			21 => { self.fHslider44 = value }
			20 => { self.fHslider45 = value }
			30 => { self.fHslider46 = value }
			29 => { self.fHslider47 = value }
			28 => { self.fHslider48 = value }
			27 => { self.fHslider49 = value }
			85 => { self.fHslider5 = value }
			26 => { self.fHslider50 = value }
			25 => { self.fHslider51 = value }
			6 => { self.fHslider6 = value }
			78 => { self.fHslider7 = value }
			4 => { self.fHslider8 = value }
			82 => { self.fHslider9 = value }
			76 => { self.fVslider0 = value }
			77 => { self.fVslider1 = value }
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
		let mut fSlow38: F32 = self.fEntry2;
		let mut iSlow39: i32 = (fSlow38 == 14.0) as i32;
		let mut fSlow40: F32 = (iSlow39) as F32;
		let mut iSlow41: i32 = (fSlow40 != 0.0) as i32;
		let mut iSlow42: i32 = (fSlow3 == 1.0) as i32;
		let mut fSlow43: F32 = (iSlow42) as F32;
		let mut iSlow44: i32 = (fSlow43 != 0.0) as i32;
		let mut iSlow45: i32 = (fSlow38 == 0.0) as i32;
		let mut fSlow46: F32 = (iSlow45) as F32;
		let mut iSlow47: i32 = (fSlow46 != 0.0) as i32;
		let mut iSlow48: i32 = (fSlow38 == 13.0) as i32;
		let mut fSlow49: F32 = (iSlow48) as F32;
		let mut iSlow50: i32 = (fSlow49 != 0.0) as i32;
		let mut iSlow51: i32 = (fSlow38 == 12.0) as i32;
		let mut fSlow52: F32 = (iSlow51) as F32;
		let mut iSlow53: i32 = (fSlow52 != 0.0) as i32;
		let mut iSlow54: i32 = (fSlow38 == 11.0) as i32;
		let mut fSlow55: F32 = (iSlow54) as F32;
		let mut iSlow56: i32 = (fSlow55 != 0.0) as i32;
		let mut iSlow57: i32 = (fSlow38 == 1e+01) as i32;
		let mut fSlow58: F32 = (iSlow57) as F32;
		let mut iSlow59: i32 = (fSlow58 != 0.0) as i32;
		let mut iSlow60: i32 = (fSlow38 == 6.0) as i32;
		let mut fSlow61: F32 = (iSlow60) as F32;
		let mut iSlow62: i32 = (fSlow61 != 0.0) as i32;
		let mut iSlow63: i32 = (fSlow38 == 9.0) as i32;
		let mut fSlow64: F32 = (iSlow63) as F32;
		let mut iSlow65: i32 = (fSlow64 != 0.0) as i32;
		let mut iSlow66: i32 = (fSlow38 == 7.0) as i32;
		let mut fSlow67: F32 = (iSlow66) as F32;
		let mut iSlow68: i32 = (fSlow67 != 0.0) as i32;
		let mut iSlow69: i32 = (fSlow38 == 4.0) as i32;
		let mut fSlow70: F32 = (iSlow69) as F32;
		let mut iSlow71: i32 = (fSlow70 != 0.0) as i32;
		let mut iSlow72: i32 = (fSlow38 == 8.0) as i32;
		let mut fSlow73: F32 = (iSlow72) as F32;
		let mut iSlow74: i32 = (fSlow73 != 0.0) as i32;
		let mut iSlow75: i32 = (fSlow38 == 3.0) as i32;
		let mut fSlow76: F32 = (iSlow75) as F32;
		let mut iSlow77: i32 = (fSlow76 != 0.0) as i32;
		let mut iSlow78: i32 = (fSlow38 == 1.0) as i32;
		let mut fSlow79: F32 = (iSlow78) as F32;
		let mut iSlow80: i32 = (fSlow79 != 0.0) as i32;
		let mut iSlow81: i32 = (fSlow38 == 2.0) as i32;
		let mut fSlow82: F32 = (iSlow81) as F32;
		let mut iSlow83: i32 = (fSlow82 != 0.0) as i32;
		let mut iSlow84: i32 = (fSlow38 == 5.0) as i32;
		let mut fSlow85: F32 = (iSlow84) as F32;
		let mut iSlow86: i32 = (fSlow85 != 0.0) as i32;
		let mut fSlow87: F32 = self.fButton0;
		let mut fSlow88: F32 = self.fHslider0;
		let mut fSlow89: F32 = self.fHslider1;
		let mut fSlow90: F32 = self.fHslider2;
		let mut iSlow91: i32 = (fSlow90) as i32;
		let mut fSlow92: F32 = unsafe { ftbl0OscDspSIG0[iSlow91 as usize] };
		let mut fSlow93: F32 = self.fEntry3;
		let mut iSlow94: i32 = (fSlow93 == 1.0) as i32;
		let mut fSlow95: F32 = (iSlow94) as F32;
		let mut fSlow96: F32 = fSlow92 + fSlow95 * (unsafe { ftbl1OscDspSIG1[iSlow91 as usize] } - fSlow92);
		let mut fSlow97: F32 = self.fHslider3;
		let mut fSlow98: F32 = self.fHslider4;
		let mut fSlow99: F32 = self.fEntry4;
		let mut iSlow100: i32 = (fSlow99 == 4.0) as i32;
		let mut fSlow101: F32 = (iSlow100) as F32;
		let mut iSlow102: i32 = (fSlow101 != 0.0) as i32;
		let mut fSlow103: F32 = self.fEntry5;
		let mut iSlow104: i32 = (fSlow103 == 1.0) as i32;
		let mut fSlow105: F32 = (iSlow104) as F32;
		let mut iSlow106: i32 = (fSlow105 != 0.0) as i32;
		let mut iSlow107: i32 = (self.fVslider1 != 0.0) as i32;
		let mut fSlow108: F32 = (iSlow107) as F32;
		let mut iSlow109: i32 = (fSlow108 != 0.0) as i32;
		let mut iSlow110: i32 = (fSlow99 == 3.0) as i32;
		let mut fSlow111: F32 = (iSlow110) as F32;
		let mut iSlow112: i32 = (fSlow111 != 0.0) as i32;
		let mut iSlow113: i32 = (fSlow99 == 2.0) as i32;
		let mut fSlow114: F32 = (iSlow113) as F32;
		let mut iSlow115: i32 = (fSlow114 != 0.0) as i32;
		let mut iTemp9: i32 = self.iTempPerm0;
		let mut fTemp10: F32 = self.fTempPerm1;
		let mut fSlow116: F32 = self.fHslider5;
		let mut fSlow117: F32 = 12.9898 * fSlow116;
		let mut fSlow118: F32 = 43758.547 * F32::sin(fSlow117 + 234.699);
		let mut fSlow119: F32 = 43758.547 * F32::sin(fSlow117 + 7979.766);
		let mut fSlow120: F32 = 2.0 * (fSlow119 - F32::floor(fSlow119)) + -1.0;
		let mut fSlow121: F32 = self.fConst2 * (0.1 * F32::max(-1.0, F32::min(1.0, 0.70710677 * (fSlow120 + 2.0 * (fSlow118 - F32::floor(fSlow118)) + -1.0))) + 0.45);
		let mut fTemp11: F32 = self.fTempPerm2;
		let mut iSlow122: i32 = (409.0 * fSlow116 + 4e+03) as i32;
		let mut iTemp12: i32 = self.iTempPerm3;
		let mut fSlow123: F32 = self.fHslider6;
		let mut fSlow124: F32 = 43758.547 * F32::sin(fSlow117 + 156.466);
		let mut fSlow125: F32 = 0.15 * F32::max(-1.0, F32::min(1.0, 1.2803688 * (0.6 * fSlow120 + 0.5 * (2.0 * (fSlow124 - F32::floor(fSlow124)) + -1.0)))) + 0.25;
		let mut fSlow126: F32 = 1.0 / F32::tan(self.fConst8 * F32::max(1.0, fSlow125));
		let mut fSlow127: F32 = 1.0 - fSlow126;
		let mut fSlow128: F32 = self.fConst2 * F32::max(fSlow125, 0.0);
		let mut fTemp15: F32 = self.fTempPerm4;
		let mut fTemp18: F32 = self.fTempPerm5;
		let mut fSlow129: F32 = 1.0 / (fSlow126 + 1.0);
		let mut fSlow130: F32 = 43758.547 * F32::sin(fSlow117 + 78.233);
		let mut fSlow131: F32 = 43758.547 * F32::sin(fSlow117 + 7901.533);
		let mut fSlow132: F32 = 8.0 * F32::max(-1.0, F32::min(1.0, 1.0976425 * (0.7 * (2.0 * (fSlow131 - F32::floor(fSlow131)) + -1.0) + 0.3 * fSlow120 + 0.5 * (2.0 * (fSlow130 - F32::floor(fSlow130)) + -1.0))));
		let mut fTemp19: F32 = self.fTempPerm6;
		let mut fSlow133: F32 = self.fHslider7;
		let mut fSlow134: F32 = self.fEntry6;
		let mut fSlow135: F32 = 2.0 * ((fSlow134 == 0.0) as i32) as u32 as F32 + 12.0 * ((fSlow134 == 1.0) as i32) as u32 as F32;
		let mut fSlow136: F32 = self.fHslider8;
		let mut fSlow137: F32 = 0.33333334 * F32::max(1e-05, self.fHslider9);
		let mut iSlow138: i32 = (F32::abs(fSlow137) < 1.1920929e-07) as i32;
		let mut fSlow139: F32 = if iSlow138 != 0 {0.0} else {F32::exp(-(self.fConst2 / if iSlow138 != 0 {1.0} else {fSlow137}))};
		let mut fSlow140: F32 = F32::log(self.fHslider10, std::f32::consts::E) * (1.0 - fSlow139);
		let mut fTemp25: F32 = self.fTempPerm7;
		let mut fTemp26: F32 = self.fTempPerm8;
		let mut fSlow141: F32 = 0.618 * fSlow90 + 7.31 * self.fHslider11 * (fSlow90 + 1.0);
		let mut fSlow142: F32 = self.fCheckbox0;
		let mut fSlow143: F32 = self.fEntry7;
		let mut iSlow144: i32 = (fSlow143 == 2.0) as i32;
		let mut fSlow145: F32 = (iSlow144) as F32;
		let mut iSlow146: i32 = (fSlow145 != 0.0) as i32;
		let mut iSlow147: i32 = (fSlow103 == 0.0) as i32;
		let mut fSlow148: F32 = (iSlow147) as F32;
		let mut iSlow149: i32 = (fSlow148 != 0.0) as i32;
		let mut iSlow150: i32 = (fSlow143 == 0.0) as i32;
		let mut fSlow151: F32 = (iSlow150) as F32;
		let mut iSlow152: i32 = (fSlow151 != 0.0) as i32;
		let mut iSlow153: i32 = (fSlow143 == 3.0) as i32;
		let mut fSlow154: F32 = (iSlow153) as F32;
		let mut iSlow155: i32 = (fSlow154 != 0.0) as i32;
		let mut iSlow156: i32 = (fSlow143 == 4.0) as i32;
		let mut fSlow157: F32 = (iSlow156) as F32;
		let mut iSlow158: i32 = (fSlow157 != 0.0) as i32;
		let mut iSlow159: i32 = (fSlow143 == 5.0) as i32;
		let mut fSlow160: F32 = (iSlow159) as F32;
		let mut iSlow161: i32 = (fSlow160 != 0.0) as i32;
		let mut iSlow162: i32 = (fSlow143 == 6.0) as i32;
		let mut fSlow163: F32 = (iSlow162) as F32;
		let mut iSlow164: i32 = (fSlow163 != 0.0) as i32;
		let mut iSlow165: i32 = (fSlow143 == 7.0) as i32;
		let mut fSlow166: F32 = (iSlow165) as F32;
		let mut iSlow167: i32 = (fSlow166 != 0.0) as i32;
		let mut iSlow168: i32 = (fSlow143 == 8.0) as i32;
		let mut fSlow169: F32 = (iSlow168) as F32;
		let mut iSlow170: i32 = (fSlow169 != 0.0) as i32;
		let mut iSlow171: i32 = (fSlow143 == 9.0) as i32;
		let mut fSlow172: F32 = (iSlow171) as F32;
		let mut iSlow173: i32 = (fSlow172 != 0.0) as i32;
		let mut iSlow174: i32 = (fSlow143 == 1e+01) as i32;
		let mut fSlow175: F32 = (iSlow174) as F32;
		let mut iSlow176: i32 = (fSlow175 != 0.0) as i32;
		let mut iSlow177: i32 = (fSlow143 == 11.0) as i32;
		let mut fSlow178: F32 = (iSlow177) as F32;
		let mut iSlow179: i32 = (fSlow178 != 0.0) as i32;
		let mut iSlow180: i32 = (fSlow143 == 1.0) as i32;
		let mut fSlow181: F32 = (iSlow180) as F32;
		let mut iSlow182: i32 = (fSlow181 != 0.0) as i32;
		let mut iSlow183: i32 = (fSlow99 == 14.0) as i32;
		let mut fSlow184: F32 = (iSlow183) as F32;
		let mut iSlow185: i32 = (fSlow184 != 0.0) as i32;
		let mut iSlow186: i32 = (fSlow99 == 13.0) as i32;
		let mut fSlow187: F32 = (iSlow186) as F32;
		let mut iSlow188: i32 = (fSlow187 != 0.0) as i32;
		let mut iSlow189: i32 = (fSlow99 == 12.0) as i32;
		let mut fSlow190: F32 = (iSlow189) as F32;
		let mut iSlow191: i32 = (fSlow190 != 0.0) as i32;
		let mut iSlow192: i32 = (fSlow99 == 11.0) as i32;
		let mut fSlow193: F32 = (iSlow192) as F32;
		let mut iSlow194: i32 = (fSlow193 != 0.0) as i32;
		let mut iSlow195: i32 = (fSlow99 == 1e+01) as i32;
		let mut fSlow196: F32 = (iSlow195) as F32;
		let mut iSlow197: i32 = (fSlow196 != 0.0) as i32;
		let mut iSlow198: i32 = (fSlow99 == 9.0) as i32;
		let mut fSlow199: F32 = (iSlow198) as F32;
		let mut iSlow200: i32 = (fSlow199 != 0.0) as i32;
		let mut iSlow201: i32 = (fSlow99 == 8.0) as i32;
		let mut fSlow202: F32 = (iSlow201) as F32;
		let mut iSlow203: i32 = (fSlow202 != 0.0) as i32;
		let mut iSlow204: i32 = (fSlow99 == 7.0) as i32;
		let mut fSlow205: F32 = (iSlow204) as F32;
		let mut iSlow206: i32 = (fSlow205 != 0.0) as i32;
		let mut iSlow207: i32 = (fSlow99 == 6.0) as i32;
		let mut fSlow208: F32 = (iSlow207) as F32;
		let mut iSlow209: i32 = (fSlow208 != 0.0) as i32;
		let mut iSlow210: i32 = (fSlow99 == 5.0) as i32;
		let mut fSlow211: F32 = (iSlow210) as F32;
		let mut iSlow212: i32 = (fSlow211 != 0.0) as i32;
		let mut iSlow213: i32 = (fSlow99 == 1.0) as i32;
		let mut fSlow214: F32 = (iSlow213) as F32;
		let mut iSlow215: i32 = (fSlow214 != 0.0) as i32;
		let mut iSlow216: i32 = (fSlow99 == 0.0) as i32;
		let mut fSlow217: F32 = (iSlow216) as F32;
		let mut iSlow218: i32 = (fSlow217 != 0.0) as i32;
		let mut iSlow219: i32 = (fSlow7 == 8.0) as i32;
		let mut fSlow220: F32 = (iSlow219) as F32;
		let mut iSlow221: i32 = (fSlow220 != 0.0) as i32;
		let mut iSlow222: i32 = (fSlow7 == 1e+01) as i32;
		let mut fSlow223: F32 = (iSlow222) as F32;
		let mut iSlow224: i32 = (fSlow223 != 0.0) as i32;
		let mut fTemp30: F32 = self.fTempPerm9;
		let mut iTemp31: i32 = self.iTempPerm10;
		let mut fSlow225: F32 = self.fHslider12;
		let mut fTemp32: F32 = self.fTempPerm11;
		let mut fTemp33: F32 = self.fTempPerm12;
		let mut fSlow226: F32 = self.fHslider13;
		let mut fTemp34: F32 = self.fTempPerm13;
		let mut fTemp35: F32 = self.fTempPerm14;
		let mut fTemp36: F32 = self.fTempPerm15;
		let mut fTemp37: F32 = self.fTempPerm16;
		let mut fTemp38: F32 = self.fTempPerm17;
		let mut fTemp39: F32 = self.fTempPerm18;
		let mut fTemp40: F32 = self.fTempPerm19;
		let mut fTemp41: F32 = self.fTempPerm20;
		let mut fSlow227: F32 = self.fEntry8;
		let mut fSlow228: F32 = self.fEntry9;
		let mut fTemp42: F32 = self.fTempPerm21;
		let mut fTemp43: F32 = self.fTempPerm22;
		let mut fTemp44: F32 = self.fTempPerm23;
		let mut iTemp45: i32 = self.iTempPerm24;
		let mut fTemp46: F32 = self.fTempPerm25;
		let mut fTemp47: F32 = self.fTempPerm26;
		let mut fTemp48: F32 = self.fTempPerm27;
		let mut fTemp49: F32 = self.fTempPerm28;
		let mut fTemp50: F32 = self.fTempPerm29;
		let mut fTemp51: F32 = self.fTempPerm30;
		let mut fTemp52: F32 = self.fTempPerm31;
		let mut fTemp53: F32 = self.fTempPerm32;
		let mut fTemp54: F32 = self.fTempPerm33;
		let mut fTemp55: F32 = self.fTempPerm34;
		let mut fTemp56: F32 = self.fTempPerm35;
		let mut fTemp57: F32 = self.fTempPerm36;
		let mut fTemp58: F32 = self.fTempPerm37;
		let mut fTemp59: F32 = self.fTempPerm38;
		let mut fTemp60: F32 = self.fTempPerm39;
		let mut fTemp61: F32 = self.fTempPerm40;
		let mut fSlow229: F32 = ((iSlow8 != 0) as i32) as u32 as F32;
		let mut iSlow230: i32 = (fSlow227) as i32;
		let mut fTemp62: F32 = self.fTempPerm41;
		let mut fTemp63: F32 = self.fTempPerm42;
		let mut fSlow231: F32 = 0.5 * fSlow142;
		let mut fTemp64: F32 = self.fTempPerm43;
		let mut fTemp65: F32 = self.fTempPerm44;
		let mut fTemp66: F32 = self.fTempPerm45;
		let mut fTemp67: F32 = self.fTempPerm46;
		let mut fTemp68: F32 = self.fTempPerm47;
		let mut fTemp69: F32 = self.fTempPerm48;
		let mut fTemp70: F32 = self.fTempPerm49;
		let mut fTemp71: F32 = self.fTempPerm50;
		let mut fTemp72: F32 = self.fTempPerm51;
		let mut fTemp73: F32 = self.fTempPerm52;
		let mut fSlow232: F32 = ((iSlow11 != 0) as i32) as u32 as F32;
		let mut iTemp74: i32 = self.iTempPerm53;
		let mut iSlow233: i32 = (fSlow228) as i32;
		let mut fTemp75: F32 = self.fTempPerm54;
		let mut fTemp76: F32 = self.fTempPerm55;
		let mut fTemp77: F32 = self.fTempPerm56;
		let mut fTemp78: F32 = self.fTempPerm57;
		let mut fTemp79: F32 = self.fTempPerm58;
		let mut fTemp80: F32 = self.fTempPerm59;
		let mut fTemp81: F32 = self.fTempPerm60;
		let mut fTemp82: F32 = self.fTempPerm61;
		let mut fTemp83: F32 = self.fTempPerm62;
		let mut iSlow234: i32 = (fSlow228 >= 2.0) as i32;
		let mut iSlow235: i32 = (fSlow228 >= 1.0) as i32;
		let mut fTemp84: F32 = self.fTempPerm63;
		let mut iSlow236: i32 = (fSlow228 >= 3.0) as i32;
		let mut fSlow237: F32 = (if iSlow236 != 0 {0} else {2}) as F32;
		let mut fSlow238: F32 = if iSlow234 != 0 {fSlow237} else {if iSlow235 != 0 {4.0} else {6.2831855}};
		let mut fTemp85: F32 = self.fTempPerm64;
		let mut fTemp86: F32 = self.fTempPerm65;
		let mut fTemp87: F32 = self.fTempPerm66;
		let mut fTemp88: F32 = self.fTempPerm67;
		let mut fTemp89: F32 = self.fTempPerm68;
		let mut fTemp90: F32 = self.fTempPerm69;
		let mut fSlow239: F32 = ((iSlow14 != 0) as i32) as u32 as F32;
		let mut fTemp91: F32 = self.fTempPerm70;
		let mut fTemp92: F32 = self.fTempPerm71;
		let mut fTemp93: F32 = self.fTempPerm72;
		let mut fTemp94: F32 = self.fTempPerm73;
		let mut fTemp95: F32 = self.fTempPerm74;
		let mut iTemp96: i32 = self.iTempPerm75;
		let mut fTemp97: F32 = self.fTempPerm76;
		let mut fTemp98: F32 = self.fTempPerm77;
		let mut fTemp99: F32 = self.fTempPerm78;
		let mut fTemp100: F32 = self.fTempPerm79;
		let mut fTemp101: F32 = self.fTempPerm80;
		let mut iTemp102: i32 = self.iTempPerm81;
		let mut fTemp103: F32 = self.fTempPerm82;
		let mut fTemp104: F32 = self.fTempPerm83;
		let mut fTemp105: F32 = self.fTempPerm84;
		let mut fTemp106: F32 = self.fTempPerm85;
		let mut fTemp107: F32 = self.fTempPerm86;
		let mut fTemp108: F32 = self.fTempPerm87;
		let mut fSlow240: F32 = ((iSlow17 != 0) as i32) as u32 as F32;
		let mut fTemp109: F32 = self.fTempPerm88;
		let mut iTemp110: i32 = self.iTempPerm89;
		let mut iTemp111: i32 = self.iTempPerm90;
		let mut iTemp112: i32 = self.iTempPerm91;
		let mut iTemp113: i32 = self.iTempPerm92;
		let mut iTemp114: i32 = self.iTempPerm93;
		let mut iTemp115: i32 = self.iTempPerm94;
		let mut fTemp116: F32 = self.fTempPerm95;
		let mut fTemp117: F32 = self.fTempPerm96;
		let mut fTemp118: F32 = self.fTempPerm97;
		let mut iTemp119: i32 = self.iTempPerm98;
		let mut iTemp120: i32 = self.iTempPerm99;
		let mut iTemp121: i32 = self.iTempPerm100;
		let mut iTemp122: i32 = self.iTempPerm101;
		let mut fTemp123: F32 = self.fTempPerm102;
		let mut fTemp124: F32 = self.fTempPerm103;
		let mut fTemp125: F32 = self.fTempPerm104;
		let mut fTemp126: F32 = self.fTempPerm105;
		let mut fTemp127: F32 = self.fTempPerm106;
		let mut fTemp128: F32 = self.fTempPerm107;
		let mut fTemp129: F32 = self.fTempPerm108;
		let mut fTemp130: F32 = self.fTempPerm109;
		let mut fTemp131: F32 = self.fTempPerm110;
		let mut fSlow241: F32 = ((iSlow20 != 0) as i32) as u32 as F32;
		let mut fTemp132: F32 = self.fTempPerm111;
		let mut fTemp133: F32 = self.fTempPerm112;
		let mut fTemp134: F32 = self.fTempPerm113;
		let mut fTemp135: F32 = self.fTempPerm114;
		let mut fTemp136: F32 = self.fTempPerm115;
		let mut fTemp137: F32 = self.fTempPerm116;
		let mut fTemp138: F32 = self.fTempPerm117;
		let mut fTemp139: F32 = self.fTempPerm118;
		let mut fTemp140: F32 = self.fTempPerm119;
		let mut fTemp141: F32 = self.fTempPerm120;
		let mut fSlow242: F32 = ((iSlow23 != 0) as i32) as u32 as F32;
		let mut iTemp142: i32 = self.iTempPerm121;
		let mut fTemp143: F32 = self.fTempPerm122;
		let mut fTemp144: F32 = self.fTempPerm123;
		let mut fTemp145: F32 = self.fTempPerm124;
		let mut fTemp146: F32 = self.fTempPerm125;
		let mut fTemp147: F32 = self.fTempPerm126;
		let mut fTemp148: F32 = self.fTempPerm127;
		let mut fTemp149: F32 = self.fTempPerm128;
		let mut fTemp150: F32 = self.fTempPerm129;
		let mut fTemp151: F32 = self.fTempPerm130;
		let mut fTemp152: F32 = self.fTempPerm131;
		let mut fTemp153: F32 = self.fTempPerm132;
		let mut fSlow243: F32 = ((iSlow26 != 0) as i32) as u32 as F32;
		let mut fTemp154: F32 = self.fTempPerm133;
		let mut fTemp155: F32 = self.fTempPerm134;
		let mut fTemp156: F32 = self.fTempPerm135;
		let mut iTemp157: i32 = self.iTempPerm136;
		let mut iSlow244: i32 = (fSlow227 >= 2.0) as i32;
		let mut iSlow245: i32 = (fSlow227 >= 3.0) as i32;
		let mut iSlow246: i32 = if iSlow244 != 0 {if iSlow245 != 0 {-2} else {0}} else {0};
		let mut iSlow247: i32 = (fSlow227 >= 1.0) as i32;
		let mut fSlow248: F32 = if iSlow244 != 0 {if iSlow245 != 0 {0.5} else {0.0}} else {if iSlow247 != 0 {0.75} else {0.0}};
		let mut fTemp158: F32 = self.fTempPerm137;
		let mut fTemp159: F32 = self.fTempPerm138;
		let mut fTemp160: F32 = self.fTempPerm139;
		let mut fTemp161: F32 = self.fTempPerm140;
		let mut iTemp162: i32 = self.iTempPerm141;
		let mut iSlow249: i32 = if iSlow244 != 0 {if iSlow245 != 0 {2} else {-2}} else {0};
		let mut fSlow250: F32 = if iSlow244 != 0 {if iSlow245 != 0 {0.0} else {0.5}} else {if iSlow247 != 0 {0.25} else {0.0}};
		let mut fTemp163: F32 = self.fTempPerm142;
		let mut iTemp164: i32 = self.iTempPerm143;
		let mut fTemp165: F32 = self.fTempPerm144;
		let mut fTemp166: F32 = self.fTempPerm145;
		let mut fTemp167: F32 = self.fTempPerm146;
		let mut fTemp168: F32 = self.fTempPerm147;
		let mut fTemp169: F32 = self.fTempPerm148;
		let mut iTemp170: i32 = self.iTempPerm149;
		let mut iSlow251: i32 = if iSlow234 != 0 {if iSlow236 != 0 {-2} else {0}} else {0};
		let mut fTemp171: F32 = self.fTempPerm150;
		let mut fTemp172: F32 = self.fTempPerm151;
		let mut fSlow252: F32 = if iSlow234 != 0 {if iSlow236 != 0 {0.5} else {0.0}} else {if iSlow235 != 0 {0.75} else {0.0}};
		let mut fTemp173: F32 = self.fTempPerm152;
		let mut fTemp174: F32 = self.fTempPerm153;
		let mut fTemp175: F32 = self.fTempPerm154;
		let mut fTemp176: F32 = self.fTempPerm155;
		let mut fSlow253: F32 = (if iSlow234 != 0 {0} else {if iSlow235 != 0 {8} else {0}}) as F32;
		let mut fTemp177: F32 = self.fTempPerm156;
		let mut iSlow254: i32 = if iSlow234 != 0 {if iSlow236 != 0 {2} else {-2}} else {0};
		let mut fTemp178: F32 = self.fTempPerm157;
		let mut fSlow255: F32 = if iSlow234 != 0 {if iSlow236 != 0 {0.0} else {0.5}} else {if iSlow235 != 0 {0.25} else {0.0}};
		let mut fTemp179: F32 = self.fTempPerm158;
		let mut fTemp180: F32 = self.fTempPerm159;
		let mut fTemp181: F32 = self.fTempPerm160;
		let mut fSlow256: F32 = (if iSlow234 != 0 {0} else {if iSlow235 != 0 {-8} else {0}}) as F32;
		let mut fTemp182: F32 = self.fTempPerm161;
		let mut fTemp183: F32 = self.fTempPerm162;
		let mut fTemp184: F32 = self.fTempPerm163;
		let mut fTemp185: F32 = self.fTempPerm164;
		let mut fSlow257: F32 = ((iSlow219 != 0) as i32) as u32 as F32;
		let mut fTemp186: F32 = self.fTempPerm165;
		let mut fTemp187: F32 = self.fTempPerm166;
		let mut fTemp188: F32 = self.fTempPerm167;
		let mut fTemp189: F32 = self.fTempPerm168;
		let mut fSlow258: F32 = ((iSlow29 != 0) as i32) as u32 as F32;
		let mut fTemp190: F32 = self.fTempPerm169;
		let mut fTemp191: F32 = self.fTempPerm170;
		let mut fTemp192: F32 = self.fTempPerm171;
		let mut iTemp193: i32 = self.iTempPerm172;
		let mut fTemp194: F32 = self.fTempPerm173;
		let mut iTemp195: i32 = self.iTempPerm174;
		let mut fTemp196: F32 = self.fTempPerm175;
		let mut iTemp197: i32 = self.iTempPerm176;
		let mut fTemp198: F32 = self.fTempPerm177;
		let mut iTemp199: i32 = self.iTempPerm178;
		let mut fTemp200: F32 = self.fTempPerm179;
		let mut fTemp201: F32 = self.fTempPerm180;
		let mut fTemp202: F32 = self.fTempPerm181;
		let mut fTemp203: F32 = self.fTempPerm182;
		let mut iTemp204: i32 = self.iTempPerm183;
		let mut fTemp205: F32 = self.fTempPerm184;
		let mut fTemp206: F32 = self.fTempPerm185;
		let mut fTemp207: F32 = self.fTempPerm186;
		let mut fTemp208: F32 = self.fTempPerm187;
		let mut fTemp209: F32 = self.fTempPerm188;
		let mut fTemp210: F32 = self.fTempPerm189;
		let mut fTemp211: F32 = self.fTempPerm190;
		let mut fTemp212: F32 = self.fTempPerm191;
		let mut fTemp213: F32 = self.fTempPerm192;
		let mut fTemp214: F32 = self.fTempPerm193;
		let mut fTemp215: F32 = self.fTempPerm194;
		let mut fTemp216: F32 = self.fTempPerm195;
		let mut fTemp217: F32 = self.fTempPerm196;
		let mut fTemp218: F32 = self.fTempPerm197;
		let mut fTemp219: F32 = self.fTempPerm198;
		let mut fTemp220: F32 = self.fTempPerm199;
		let mut fTemp221: F32 = self.fTempPerm200;
		let mut fSlow259: F32 = ((iSlow222 != 0) as i32) as u32 as F32;
		let mut iTemp222: i32 = self.iTempPerm201;
		let mut fTemp223: F32 = self.fTempPerm202;
		let mut fTemp224: F32 = self.fTempPerm203;
		let mut fTemp225: F32 = self.fTempPerm204;
		let mut fTemp226: F32 = self.fTempPerm205;
		let mut fTemp227: F32 = self.fTempPerm206;
		let mut fTemp228: F32 = self.fTempPerm207;
		let mut fTemp229: F32 = self.fTempPerm208;
		let mut fTemp230: F32 = self.fTempPerm209;
		let mut fTemp231: F32 = self.fTempPerm210;
		let mut fTemp232: F32 = self.fTempPerm211;
		let mut fTemp233: F32 = self.fTempPerm212;
		let mut fTemp234: F32 = self.fTempPerm213;
		let mut fTemp235: F32 = self.fTempPerm214;
		let mut fTemp236: F32 = self.fTempPerm215;
		let mut fTemp237: F32 = self.fTempPerm216;
		let mut fTemp238: F32 = self.fTempPerm217;
		let mut fTemp239: F32 = self.fTempPerm218;
		let mut iTemp240: i32 = self.iTempPerm219;
		let mut fTemp241: F32 = self.fTempPerm220;
		let mut fTemp242: F32 = self.fTempPerm221;
		let mut fSlow260: F32 = ((iSlow32 != 0) as i32) as u32 as F32;
		let mut fTemp243: F32 = self.fTempPerm222;
		let mut fTemp244: F32 = self.fTempPerm223;
		let mut iTemp245: i32 = self.iTempPerm224;
		let mut fTemp246: F32 = self.fTempPerm225;
		let mut iTemp247: i32 = self.iTempPerm226;
		let mut fTemp248: F32 = self.fTempPerm227;
		let mut fTemp249: F32 = self.fTempPerm228;
		let mut fTemp250: F32 = self.fTempPerm229;
		let mut fTemp251: F32 = self.fTempPerm230;
		let mut fTemp252: F32 = self.fTempPerm231;
		let mut fTemp253: F32 = self.fTempPerm232;
		let mut fTemp254: F32 = self.fTempPerm233;
		let mut fTemp255: F32 = self.fTempPerm234;
		let mut fTemp256: F32 = self.fTempPerm235;
		let mut fTemp257: F32 = self.fTempPerm236;
		let mut fTemp258: F32 = self.fTempPerm237;
		let mut fTemp259: F32 = self.fTempPerm238;
		let mut fTemp260: F32 = self.fTempPerm239;
		let mut fSlow261: F32 = ((iSlow35 != 0) as i32) as u32 as F32;
		let mut fSlow262: F32 = unsafe { ftbl6OscDspSIG6[((fSlow7) as i32) as usize] };
		let mut fTemp261: F32 = self.fTempPerm240;
		let mut fSlow263: F32 = ((iSlow4 != 0) as i32) as u32 as F32;
		let mut fSlow264: F32 = self.fHslider14;
		let mut fTemp262: F32 = self.fTempPerm241;
		let mut fTemp263: F32 = self.fTempPerm242;
		let mut iTemp264: i32 = self.iTempPerm243;
		let mut iTemp265: i32 = self.iTempPerm244;
		let mut iTemp266: i32 = self.iTempPerm245;
		let mut fSlow265: F32 = self.fEntry10;
		let mut fSlow266: F32 = self.fEntry11;
		let mut fSlow267: F32 = self.fEntry12;
		let mut iTemp267: i32 = self.iTempPerm246;
		let mut fTemp268: F32 = self.fTempPerm247;
		let mut iTemp269: i32 = self.iTempPerm248;
		let mut iTemp270: i32 = self.iTempPerm249;
		let mut iTemp271: i32 = self.iTempPerm250;
		let mut iTemp272: i32 = self.iTempPerm251;
		let mut iTemp273: i32 = self.iTempPerm252;
		let mut iTemp274: i32 = self.iTempPerm253;
		let mut iTemp275: i32 = self.iTempPerm254;
		let mut fTemp276: F32 = self.fTempPerm255;
		let mut iTemp277: i32 = self.iTempPerm256;
		let mut iTemp278: i32 = self.iTempPerm257;
		let mut fTemp279: F32 = self.fTempPerm258;
		let mut fTemp280: F32 = self.fTempPerm259;
		let mut fTemp281: F32 = self.fTempPerm260;
		let mut fTemp282: F32 = self.fTempPerm261;
		let mut fTemp283: F32 = self.fTempPerm262;
		let mut fSlow268: F32 = self.fEntry13;
		let mut iTemp284: i32 = self.iTempPerm263;
		let mut fTemp285: F32 = self.fTempPerm264;
		let mut fTemp286: F32 = self.fTempPerm265;
		let mut fSlow269: F32 = self.fHslider15;
		let mut fTemp287: F32 = self.fTempPerm266;
		let mut fTemp288: F32 = self.fTempPerm267;
		let mut fTemp289: F32 = self.fTempPerm268;
		let mut fTemp290: F32 = self.fTempPerm269;
		let mut fSlow270: F32 = ((iSlow39 != 0) as i32) as u32 as F32;
		let mut fTemp291: F32 = self.fTempPerm270;
		let mut iTemp292: i32 = self.iTempPerm271;
		let mut iTemp293: i32 = self.iTempPerm272;
		let mut iTemp294: i32 = self.iTempPerm273;
		let mut iTemp295: i32 = self.iTempPerm274;
		let mut iTemp296: i32 = self.iTempPerm275;
		let mut iTemp297: i32 = self.iTempPerm276;
		let mut fTemp298: F32 = self.fTempPerm277;
		let mut fTemp299: F32 = self.fTempPerm278;
		let mut iTemp300: i32 = self.iTempPerm279;
		let mut iTemp301: i32 = self.iTempPerm280;
		let mut iTemp302: i32 = self.iTempPerm281;
		let mut iTemp303: i32 = self.iTempPerm282;
		let mut fTemp304: F32 = self.fTempPerm283;
		let mut fTemp305: F32 = self.fTempPerm284;
		let mut fTemp306: F32 = self.fTempPerm285;
		let mut iTemp307: i32 = self.iTempPerm286;
		let mut fTemp308: F32 = self.fTempPerm287;
		let mut fSlow271: F32 = ((iSlow48 != 0) as i32) as u32 as F32;
		let mut fTemp309: F32 = self.fTempPerm288;
		let mut fSlow272: F32 = ((iSlow51 != 0) as i32) as u32 as F32;
		let mut fTemp310: F32 = self.fTempPerm289;
		let mut fTemp311: F32 = self.fTempPerm290;
		let mut iTemp312: i32 = self.iTempPerm291;
		let mut iTemp313: i32 = self.iTempPerm292;
		let mut iTemp314: i32 = self.iTempPerm293;
		let mut iTemp315: i32 = self.iTempPerm294;
		let mut iTemp316: i32 = self.iTempPerm295;
		let mut iTemp317: i32 = self.iTempPerm296;
		let mut iTemp318: i32 = self.iTempPerm297;
		let mut fTemp319: F32 = self.fTempPerm298;
		let mut iTemp320: i32 = self.iTempPerm299;
		let mut iTemp321: i32 = self.iTempPerm300;
		let mut fTemp322: F32 = self.fTempPerm301;
		let mut fTemp323: F32 = self.fTempPerm302;
		let mut fTemp324: F32 = self.fTempPerm303;
		let mut fTemp325: F32 = self.fTempPerm304;
		let mut fSlow273: F32 = ((iSlow54 != 0) as i32) as u32 as F32;
		let mut fTemp326: F32 = self.fTempPerm305;
		let mut fSlow274: F32 = ((iSlow57 != 0) as i32) as u32 as F32;
		let mut fTemp327: F32 = self.fTempPerm306;
		let mut fTemp328: F32 = self.fTempPerm307;
		let mut fSlow275: F32 = ((iSlow63 != 0) as i32) as u32 as F32;
		let mut fTemp329: F32 = self.fTempPerm308;
		let mut iTemp330: i32 = self.iTempPerm309;
		let mut iTemp331: i32 = self.iTempPerm310;
		let mut fTemp332: F32 = self.fTempPerm311;
		let mut fTemp333: F32 = self.fTempPerm312;
		let mut fTemp334: F32 = self.fTempPerm313;
		let mut fTemp335: F32 = self.fTempPerm314;
		let mut fSlow276: F32 = ((iSlow72 != 0) as i32) as u32 as F32;
		let mut fSlow277: F32 = F32::max(0.0, F32::min(2.0, fSlow38 + -5.0));
		let mut fSlow278: F32 = self.fConst24 * (if (fSlow277 >= 2.0) as i32 != 0 {800} else {if (fSlow277 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp336: F32 = self.fTempPerm315;
		let mut fTemp337: F32 = self.fTempPerm316;
		let mut fTemp338: F32 = self.fTempPerm317;
		let mut fTemp339: F32 = self.fTempPerm318;
		let mut fTemp340: F32 = self.fTempPerm319;
		let mut fSlow279: F32 = ((iSlow66 != 0) as i32) as u32 as F32;
		let mut fTemp341: F32 = self.fTempPerm320;
		let mut fSlow280: F32 = ((iSlow60 != 0) as i32) as u32 as F32;
		let mut fTemp342: F32 = self.fTempPerm321;
		let mut fSlow281: F32 = ((iSlow84 != 0) as i32) as u32 as F32;
		let mut fSlow282: F32 = F32::max(0.0, F32::min(2.0, fSlow38 + -2.0));
		let mut fSlow283: F32 = self.fConst2 * (if (fSlow282 >= 2.0) as i32 != 0 {4000} else {if (fSlow282 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp343: F32 = self.fTempPerm322;
		let mut fTemp344: F32 = self.fTempPerm323;
		let mut fSlow284: F32 = ((iSlow69 != 0) as i32) as u32 as F32;
		let mut fTemp345: F32 = self.fTempPerm324;
		let mut fSlow285: F32 = ((iSlow75 != 0) as i32) as u32 as F32;
		let mut fTemp346: F32 = self.fTempPerm325;
		let mut fSlow286: F32 = ((iSlow81 != 0) as i32) as u32 as F32;
		let mut fTemp347: F32 = self.fTempPerm326;
		let mut fTemp348: F32 = self.fTempPerm327;
		let mut fSlow287: F32 = ((iSlow78 != 0) as i32) as u32 as F32;
		let mut fTemp349: F32 = self.fTempPerm328;
		let mut fSlow288: F32 = ((iSlow45 != 0) as i32) as u32 as F32;
		let mut fTemp350: F32 = self.fTempPerm329;
		let mut fSlow289: F32 = ((iSlow42 != 0) as i32) as u32 as F32;
		let mut fTemp351: F32 = self.fTempPerm330;
		let mut iSlow290: i32 = unsafe { itbl7OscDspSIG7[iSlow91 as usize] };
		let mut fSlow291: F32 = if (fSlow93 > 0.0) as i32 != 0 {(0.049535416 * fSlow95 + 0.6165159) * F32::powf(1e+01, 0.05 * (i32::wrapping_add(iSlow290, i32::wrapping_mul(iSlow94, i32::wrapping_sub(unsafe { itbl8OscDspSIG8[iSlow91 as usize] }, iSlow290)))) as F32)} else {1.0};
		let mut fSlow292: F32 = ((iSlow0 != 0) as i32) as u32 as F32;
		let mut fSlow293: F32 = self.fHslider16;
		let mut fTemp356: F32 = self.fTempPerm331;
		let mut fTemp357: F32 = self.fTempPerm332;
		let mut iTemp358: i32 = self.iTempPerm333;
		let mut fSlow294: F32 = self.fHslider17;
		let mut fTemp359: F32 = self.fTempPerm334;
		let mut fTemp360: F32 = self.fTempPerm335;
		let mut fSlow295: F32 = self.fHslider18;
		let mut fTemp361: F32 = self.fTempPerm336;
		let mut fTemp362: F32 = self.fTempPerm337;
		let mut fTemp363: F32 = self.fTempPerm338;
		let mut fTemp364: F32 = self.fTempPerm339;
		let mut fTemp365: F32 = self.fTempPerm340;
		let mut fTemp366: F32 = self.fTempPerm341;
		let mut fTemp367: F32 = self.fTempPerm342;
		let mut fTemp368: F32 = self.fTempPerm343;
		let mut fSlow296: F32 = self.fEntry14;
		let mut fSlow297: F32 = self.fEntry15;
		let mut fTemp369: F32 = self.fTempPerm344;
		let mut fTemp370: F32 = self.fTempPerm345;
		let mut fTemp371: F32 = self.fTempPerm346;
		let mut iTemp372: i32 = self.iTempPerm347;
		let mut fTemp373: F32 = self.fTempPerm348;
		let mut fTemp374: F32 = self.fTempPerm349;
		let mut fTemp375: F32 = self.fTempPerm350;
		let mut fTemp376: F32 = self.fTempPerm351;
		let mut fTemp377: F32 = self.fTempPerm352;
		let mut fTemp378: F32 = self.fTempPerm353;
		let mut fTemp379: F32 = self.fTempPerm354;
		let mut fTemp380: F32 = self.fTempPerm355;
		let mut fTemp381: F32 = self.fTempPerm356;
		let mut fTemp382: F32 = self.fTempPerm357;
		let mut fTemp383: F32 = self.fTempPerm358;
		let mut fTemp384: F32 = self.fTempPerm359;
		let mut fTemp385: F32 = self.fTempPerm360;
		let mut fTemp386: F32 = self.fTempPerm361;
		let mut fTemp387: F32 = self.fTempPerm362;
		let mut fTemp388: F32 = self.fTempPerm363;
		let mut fSlow298: F32 = ((iSlow144 != 0) as i32) as u32 as F32;
		let mut iSlow299: i32 = (fSlow296) as i32;
		let mut fTemp389: F32 = self.fTempPerm364;
		let mut fTemp390: F32 = self.fTempPerm365;
		let mut fTemp391: F32 = self.fTempPerm366;
		let mut fTemp392: F32 = self.fTempPerm367;
		let mut fTemp393: F32 = self.fTempPerm368;
		let mut fTemp394: F32 = self.fTempPerm369;
		let mut fTemp395: F32 = self.fTempPerm370;
		let mut fTemp396: F32 = self.fTempPerm371;
		let mut fTemp397: F32 = self.fTempPerm372;
		let mut fSlow300: F32 = ((iSlow150 != 0) as i32) as u32 as F32;
		let mut iTemp398: i32 = self.iTempPerm373;
		let mut iSlow301: i32 = (fSlow297) as i32;
		let mut fTemp399: F32 = self.fTempPerm374;
		let mut fTemp400: F32 = self.fTempPerm375;
		let mut fTemp401: F32 = self.fTempPerm376;
		let mut fTemp402: F32 = self.fTempPerm377;
		let mut fTemp403: F32 = self.fTempPerm378;
		let mut fTemp404: F32 = self.fTempPerm379;
		let mut fTemp405: F32 = self.fTempPerm380;
		let mut fTemp406: F32 = self.fTempPerm381;
		let mut fTemp407: F32 = self.fTempPerm382;
		let mut iSlow302: i32 = (fSlow297 >= 2.0) as i32;
		let mut iSlow303: i32 = (fSlow297 >= 1.0) as i32;
		let mut fTemp408: F32 = self.fTempPerm383;
		let mut iSlow304: i32 = (fSlow297 >= 3.0) as i32;
		let mut fSlow305: F32 = (if iSlow304 != 0 {0} else {2}) as F32;
		let mut fSlow306: F32 = if iSlow302 != 0 {fSlow305} else {if iSlow303 != 0 {4.0} else {6.2831855}};
		let mut fTemp409: F32 = self.fTempPerm384;
		let mut fTemp410: F32 = self.fTempPerm385;
		let mut fTemp411: F32 = self.fTempPerm386;
		let mut fTemp412: F32 = self.fTempPerm387;
		let mut fTemp413: F32 = self.fTempPerm388;
		let mut fTemp414: F32 = self.fTempPerm389;
		let mut fSlow307: F32 = ((iSlow153 != 0) as i32) as u32 as F32;
		let mut fTemp415: F32 = self.fTempPerm390;
		let mut fTemp416: F32 = self.fTempPerm391;
		let mut fTemp417: F32 = self.fTempPerm392;
		let mut fTemp418: F32 = self.fTempPerm393;
		let mut fTemp419: F32 = self.fTempPerm394;
		let mut iTemp420: i32 = self.iTempPerm395;
		let mut fTemp421: F32 = self.fTempPerm396;
		let mut fTemp422: F32 = self.fTempPerm397;
		let mut fTemp423: F32 = self.fTempPerm398;
		let mut fTemp424: F32 = self.fTempPerm399;
		let mut fTemp425: F32 = self.fTempPerm400;
		let mut iTemp426: i32 = self.iTempPerm401;
		let mut fTemp427: F32 = self.fTempPerm402;
		let mut fTemp428: F32 = self.fTempPerm403;
		let mut fTemp429: F32 = self.fTempPerm404;
		let mut fTemp430: F32 = self.fTempPerm405;
		let mut fTemp431: F32 = self.fTempPerm406;
		let mut fTemp432: F32 = self.fTempPerm407;
		let mut fSlow308: F32 = ((iSlow156 != 0) as i32) as u32 as F32;
		let mut fTemp433: F32 = self.fTempPerm408;
		let mut iTemp434: i32 = self.iTempPerm409;
		let mut iTemp435: i32 = self.iTempPerm410;
		let mut iTemp436: i32 = self.iTempPerm411;
		let mut iTemp437: i32 = self.iTempPerm412;
		let mut iTemp438: i32 = self.iTempPerm413;
		let mut iTemp439: i32 = self.iTempPerm414;
		let mut fTemp440: F32 = self.fTempPerm415;
		let mut fTemp441: F32 = self.fTempPerm416;
		let mut fTemp442: F32 = self.fTempPerm417;
		let mut iTemp443: i32 = self.iTempPerm418;
		let mut iTemp444: i32 = self.iTempPerm419;
		let mut iTemp445: i32 = self.iTempPerm420;
		let mut iTemp446: i32 = self.iTempPerm421;
		let mut fTemp447: F32 = self.fTempPerm422;
		let mut fTemp448: F32 = self.fTempPerm423;
		let mut fTemp449: F32 = self.fTempPerm424;
		let mut fTemp450: F32 = self.fTempPerm425;
		let mut fTemp451: F32 = self.fTempPerm426;
		let mut fTemp452: F32 = self.fTempPerm427;
		let mut fTemp453: F32 = self.fTempPerm428;
		let mut fTemp454: F32 = self.fTempPerm429;
		let mut fTemp455: F32 = self.fTempPerm430;
		let mut fSlow309: F32 = ((iSlow159 != 0) as i32) as u32 as F32;
		let mut fTemp456: F32 = self.fTempPerm431;
		let mut fTemp457: F32 = self.fTempPerm432;
		let mut fTemp458: F32 = self.fTempPerm433;
		let mut fTemp459: F32 = self.fTempPerm434;
		let mut fTemp460: F32 = self.fTempPerm435;
		let mut fTemp461: F32 = self.fTempPerm436;
		let mut fSlow310: F32 = ((iSlow162 != 0) as i32) as u32 as F32;
		let mut iTemp462: i32 = self.iTempPerm437;
		let mut fTemp463: F32 = self.fTempPerm438;
		let mut fTemp464: F32 = self.fTempPerm439;
		let mut fTemp465: F32 = self.fTempPerm440;
		let mut fTemp466: F32 = self.fTempPerm441;
		let mut fTemp467: F32 = self.fTempPerm442;
		let mut fTemp468: F32 = self.fTempPerm443;
		let mut fTemp469: F32 = self.fTempPerm444;
		let mut fTemp470: F32 = self.fTempPerm445;
		let mut fTemp471: F32 = self.fTempPerm446;
		let mut fTemp472: F32 = self.fTempPerm447;
		let mut fTemp473: F32 = self.fTempPerm448;
		let mut fSlow311: F32 = ((iSlow165 != 0) as i32) as u32 as F32;
		let mut fTemp474: F32 = self.fTempPerm449;
		let mut fTemp475: F32 = self.fTempPerm450;
		let mut fTemp476: F32 = self.fTempPerm451;
		let mut iTemp477: i32 = self.iTempPerm452;
		let mut iSlow312: i32 = (fSlow296 >= 2.0) as i32;
		let mut iSlow313: i32 = (fSlow296 >= 3.0) as i32;
		let mut iSlow314: i32 = if iSlow312 != 0 {if iSlow313 != 0 {-2} else {0}} else {0};
		let mut iSlow315: i32 = (fSlow296 >= 1.0) as i32;
		let mut fSlow316: F32 = if iSlow312 != 0 {if iSlow313 != 0 {0.5} else {0.0}} else {if iSlow315 != 0 {0.75} else {0.0}};
		let mut fTemp478: F32 = self.fTempPerm453;
		let mut fTemp479: F32 = self.fTempPerm454;
		let mut fTemp480: F32 = self.fTempPerm455;
		let mut fTemp481: F32 = self.fTempPerm456;
		let mut iTemp482: i32 = self.iTempPerm457;
		let mut iSlow317: i32 = if iSlow312 != 0 {if iSlow313 != 0 {2} else {-2}} else {0};
		let mut fSlow318: F32 = if iSlow312 != 0 {if iSlow313 != 0 {0.0} else {0.5}} else {if iSlow315 != 0 {0.25} else {0.0}};
		let mut fTemp483: F32 = self.fTempPerm458;
		let mut iTemp484: i32 = self.iTempPerm459;
		let mut fTemp485: F32 = self.fTempPerm460;
		let mut fTemp486: F32 = self.fTempPerm461;
		let mut fTemp487: F32 = self.fTempPerm462;
		let mut fTemp488: F32 = self.fTempPerm463;
		let mut fTemp489: F32 = self.fTempPerm464;
		let mut iTemp490: i32 = self.iTempPerm465;
		let mut iSlow319: i32 = if iSlow302 != 0 {if iSlow304 != 0 {-2} else {0}} else {0};
		let mut fTemp491: F32 = self.fTempPerm466;
		let mut fTemp492: F32 = self.fTempPerm467;
		let mut fSlow320: F32 = if iSlow302 != 0 {if iSlow304 != 0 {0.5} else {0.0}} else {if iSlow303 != 0 {0.75} else {0.0}};
		let mut fTemp493: F32 = self.fTempPerm468;
		let mut fTemp494: F32 = self.fTempPerm469;
		let mut fTemp495: F32 = self.fTempPerm470;
		let mut fTemp496: F32 = self.fTempPerm471;
		let mut fSlow321: F32 = (if iSlow302 != 0 {0} else {if iSlow303 != 0 {8} else {0}}) as F32;
		let mut fTemp497: F32 = self.fTempPerm472;
		let mut iSlow322: i32 = if iSlow302 != 0 {if iSlow304 != 0 {2} else {-2}} else {0};
		let mut fTemp498: F32 = self.fTempPerm473;
		let mut fSlow323: F32 = if iSlow302 != 0 {if iSlow304 != 0 {0.0} else {0.5}} else {if iSlow303 != 0 {0.25} else {0.0}};
		let mut fTemp499: F32 = self.fTempPerm474;
		let mut fTemp500: F32 = self.fTempPerm475;
		let mut fTemp501: F32 = self.fTempPerm476;
		let mut fSlow324: F32 = (if iSlow302 != 0 {0} else {if iSlow303 != 0 {-8} else {0}}) as F32;
		let mut fTemp502: F32 = self.fTempPerm477;
		let mut fTemp503: F32 = self.fTempPerm478;
		let mut fTemp504: F32 = self.fTempPerm479;
		let mut fTemp505: F32 = self.fTempPerm480;
		let mut fSlow325: F32 = ((iSlow168 != 0) as i32) as u32 as F32;
		let mut fTemp506: F32 = self.fTempPerm481;
		let mut fTemp507: F32 = self.fTempPerm482;
		let mut fTemp508: F32 = self.fTempPerm483;
		let mut fSlow326: F32 = ((iSlow171 != 0) as i32) as u32 as F32;
		let mut fTemp509: F32 = self.fTempPerm484;
		let mut fTemp510: F32 = self.fTempPerm485;
		let mut fTemp511: F32 = self.fTempPerm486;
		let mut iTemp512: i32 = self.iTempPerm487;
		let mut fTemp513: F32 = self.fTempPerm488;
		let mut iTemp514: i32 = self.iTempPerm489;
		let mut fTemp515: F32 = self.fTempPerm490;
		let mut iTemp516: i32 = self.iTempPerm491;
		let mut fTemp517: F32 = self.fTempPerm492;
		let mut iTemp518: i32 = self.iTempPerm493;
		let mut fTemp519: F32 = self.fTempPerm494;
		let mut fTemp520: F32 = self.fTempPerm495;
		let mut fTemp521: F32 = self.fTempPerm496;
		let mut fTemp522: F32 = self.fTempPerm497;
		let mut iTemp523: i32 = self.iTempPerm498;
		let mut fTemp524: F32 = self.fTempPerm499;
		let mut fTemp525: F32 = self.fTempPerm500;
		let mut fTemp526: F32 = self.fTempPerm501;
		let mut fTemp527: F32 = self.fTempPerm502;
		let mut fTemp528: F32 = self.fTempPerm503;
		let mut fTemp529: F32 = self.fTempPerm504;
		let mut fTemp530: F32 = self.fTempPerm505;
		let mut fTemp531: F32 = self.fTempPerm506;
		let mut fTemp532: F32 = self.fTempPerm507;
		let mut fTemp533: F32 = self.fTempPerm508;
		let mut fTemp534: F32 = self.fTempPerm509;
		let mut fTemp535: F32 = self.fTempPerm510;
		let mut fTemp536: F32 = self.fTempPerm511;
		let mut fTemp537: F32 = self.fTempPerm512;
		let mut fTemp538: F32 = self.fTempPerm513;
		let mut fTemp539: F32 = self.fTempPerm514;
		let mut fTemp540: F32 = self.fTempPerm515;
		let mut fSlow327: F32 = ((iSlow174 != 0) as i32) as u32 as F32;
		let mut iTemp541: i32 = self.iTempPerm516;
		let mut fTemp542: F32 = self.fTempPerm517;
		let mut fTemp543: F32 = self.fTempPerm518;
		let mut fTemp544: F32 = self.fTempPerm519;
		let mut fTemp545: F32 = self.fTempPerm520;
		let mut fTemp546: F32 = self.fTempPerm521;
		let mut fTemp547: F32 = self.fTempPerm522;
		let mut fTemp548: F32 = self.fTempPerm523;
		let mut fTemp549: F32 = self.fTempPerm524;
		let mut fTemp550: F32 = self.fTempPerm525;
		let mut fTemp551: F32 = self.fTempPerm526;
		let mut fTemp552: F32 = self.fTempPerm527;
		let mut fTemp553: F32 = self.fTempPerm528;
		let mut fTemp554: F32 = self.fTempPerm529;
		let mut fTemp555: F32 = self.fTempPerm530;
		let mut fTemp556: F32 = self.fTempPerm531;
		let mut fTemp557: F32 = self.fTempPerm532;
		let mut fTemp558: F32 = self.fTempPerm533;
		let mut iTemp559: i32 = self.iTempPerm534;
		let mut fTemp560: F32 = self.fTempPerm535;
		let mut fTemp561: F32 = self.fTempPerm536;
		let mut fSlow328: F32 = ((iSlow177 != 0) as i32) as u32 as F32;
		let mut fTemp562: F32 = self.fTempPerm537;
		let mut fTemp563: F32 = self.fTempPerm538;
		let mut iTemp564: i32 = self.iTempPerm539;
		let mut fTemp565: F32 = self.fTempPerm540;
		let mut iTemp566: i32 = self.iTempPerm541;
		let mut fTemp567: F32 = self.fTempPerm542;
		let mut fTemp568: F32 = self.fTempPerm543;
		let mut fTemp569: F32 = self.fTempPerm544;
		let mut fTemp570: F32 = self.fTempPerm545;
		let mut fTemp571: F32 = self.fTempPerm546;
		let mut fTemp572: F32 = self.fTempPerm547;
		let mut fTemp573: F32 = self.fTempPerm548;
		let mut fSlow329: F32 = ((iSlow180 != 0) as i32) as u32 as F32;
		let mut fSlow330: F32 = unsafe { ftbl6OscDspSIG6[((fSlow143) as i32) as usize] };
		let mut fTemp574: F32 = self.fTempPerm549;
		let mut fSlow331: F32 = ((iSlow147 != 0) as i32) as u32 as F32;
		let mut fSlow332: F32 = self.fHslider19;
		let mut fTemp575: F32 = self.fTempPerm550;
		let mut fTemp576: F32 = self.fTempPerm551;
		let mut iTemp577: i32 = self.iTempPerm552;
		let mut iTemp578: i32 = self.iTempPerm553;
		let mut iTemp579: i32 = self.iTempPerm554;
		let mut fSlow333: F32 = self.fEntry16;
		let mut fSlow334: F32 = self.fEntry17;
		let mut fSlow335: F32 = self.fEntry18;
		let mut iTemp580: i32 = self.iTempPerm555;
		let mut fTemp581: F32 = self.fTempPerm556;
		let mut iTemp582: i32 = self.iTempPerm557;
		let mut iTemp583: i32 = self.iTempPerm558;
		let mut iTemp584: i32 = self.iTempPerm559;
		let mut iTemp585: i32 = self.iTempPerm560;
		let mut iTemp586: i32 = self.iTempPerm561;
		let mut iTemp587: i32 = self.iTempPerm562;
		let mut iTemp588: i32 = self.iTempPerm563;
		let mut fTemp589: F32 = self.fTempPerm564;
		let mut iTemp590: i32 = self.iTempPerm565;
		let mut iTemp591: i32 = self.iTempPerm566;
		let mut fTemp592: F32 = self.fTempPerm567;
		let mut fTemp593: F32 = self.fTempPerm568;
		let mut fTemp594: F32 = self.fTempPerm569;
		let mut fTemp595: F32 = self.fTempPerm570;
		let mut fTemp596: F32 = self.fTempPerm571;
		let mut fSlow336: F32 = self.fEntry19;
		let mut iTemp597: i32 = self.iTempPerm572;
		let mut fTemp598: F32 = self.fTempPerm573;
		let mut fTemp599: F32 = self.fTempPerm574;
		let mut fSlow337: F32 = self.fHslider20;
		let mut fTemp600: F32 = self.fTempPerm575;
		let mut fTemp601: F32 = self.fTempPerm576;
		let mut fTemp602: F32 = self.fTempPerm577;
		let mut fTemp603: F32 = self.fTempPerm578;
		let mut fSlow338: F32 = ((iSlow183 != 0) as i32) as u32 as F32;
		let mut fTemp604: F32 = self.fTempPerm579;
		let mut iTemp605: i32 = self.iTempPerm580;
		let mut iTemp606: i32 = self.iTempPerm581;
		let mut iTemp607: i32 = self.iTempPerm582;
		let mut iTemp608: i32 = self.iTempPerm583;
		let mut iTemp609: i32 = self.iTempPerm584;
		let mut iTemp610: i32 = self.iTempPerm585;
		let mut fTemp611: F32 = self.fTempPerm586;
		let mut fTemp612: F32 = self.fTempPerm587;
		let mut iTemp613: i32 = self.iTempPerm588;
		let mut iTemp614: i32 = self.iTempPerm589;
		let mut iTemp615: i32 = self.iTempPerm590;
		let mut iTemp616: i32 = self.iTempPerm591;
		let mut fTemp617: F32 = self.fTempPerm592;
		let mut fTemp618: F32 = self.fTempPerm593;
		let mut fTemp619: F32 = self.fTempPerm594;
		let mut iTemp620: i32 = self.iTempPerm595;
		let mut fTemp621: F32 = self.fTempPerm596;
		let mut fSlow339: F32 = ((iSlow186 != 0) as i32) as u32 as F32;
		let mut fTemp622: F32 = self.fTempPerm597;
		let mut fSlow340: F32 = ((iSlow189 != 0) as i32) as u32 as F32;
		let mut fTemp623: F32 = self.fTempPerm598;
		let mut fTemp624: F32 = self.fTempPerm599;
		let mut iTemp625: i32 = self.iTempPerm600;
		let mut iTemp626: i32 = self.iTempPerm601;
		let mut iTemp627: i32 = self.iTempPerm602;
		let mut iTemp628: i32 = self.iTempPerm603;
		let mut iTemp629: i32 = self.iTempPerm604;
		let mut iTemp630: i32 = self.iTempPerm605;
		let mut iTemp631: i32 = self.iTempPerm606;
		let mut fTemp632: F32 = self.fTempPerm607;
		let mut iTemp633: i32 = self.iTempPerm608;
		let mut iTemp634: i32 = self.iTempPerm609;
		let mut fTemp635: F32 = self.fTempPerm610;
		let mut fTemp636: F32 = self.fTempPerm611;
		let mut fTemp637: F32 = self.fTempPerm612;
		let mut fTemp638: F32 = self.fTempPerm613;
		let mut fSlow341: F32 = ((iSlow192 != 0) as i32) as u32 as F32;
		let mut fTemp639: F32 = self.fTempPerm614;
		let mut fSlow342: F32 = ((iSlow195 != 0) as i32) as u32 as F32;
		let mut fTemp640: F32 = self.fTempPerm615;
		let mut fTemp641: F32 = self.fTempPerm616;
		let mut fSlow343: F32 = ((iSlow198 != 0) as i32) as u32 as F32;
		let mut fTemp642: F32 = self.fTempPerm617;
		let mut iTemp643: i32 = self.iTempPerm618;
		let mut iTemp644: i32 = self.iTempPerm619;
		let mut fTemp645: F32 = self.fTempPerm620;
		let mut fTemp646: F32 = self.fTempPerm621;
		let mut fTemp647: F32 = self.fTempPerm622;
		let mut fTemp648: F32 = self.fTempPerm623;
		let mut fSlow344: F32 = ((iSlow201 != 0) as i32) as u32 as F32;
		let mut fSlow345: F32 = F32::max(0.0, F32::min(2.0, fSlow99 + -5.0));
		let mut fSlow346: F32 = self.fConst24 * (if (fSlow345 >= 2.0) as i32 != 0 {800} else {if (fSlow345 >= 1.0) as i32 != 0 {1500} else {6000}}) as F32;
		let mut fTemp649: F32 = self.fTempPerm624;
		let mut fTemp650: F32 = self.fTempPerm625;
		let mut fTemp651: F32 = self.fTempPerm626;
		let mut fTemp652: F32 = self.fTempPerm627;
		let mut fTemp653: F32 = self.fTempPerm628;
		let mut fSlow347: F32 = ((iSlow204 != 0) as i32) as u32 as F32;
		let mut fTemp654: F32 = self.fTempPerm629;
		let mut fSlow348: F32 = ((iSlow207 != 0) as i32) as u32 as F32;
		let mut fTemp655: F32 = self.fTempPerm630;
		let mut fSlow349: F32 = ((iSlow210 != 0) as i32) as u32 as F32;
		let mut fSlow350: F32 = F32::max(0.0, F32::min(2.0, fSlow99 + -2.0));
		let mut fSlow351: F32 = self.fConst2 * (if (fSlow350 >= 2.0) as i32 != 0 {4000} else {if (fSlow350 >= 1.0) as i32 != 0 {8000} else {11000}}) as F32;
		let mut fTemp656: F32 = self.fTempPerm631;
		let mut fTemp657: F32 = self.fTempPerm632;
		let mut fSlow352: F32 = ((iSlow100 != 0) as i32) as u32 as F32;
		let mut fTemp658: F32 = self.fTempPerm633;
		let mut fSlow353: F32 = ((iSlow110 != 0) as i32) as u32 as F32;
		let mut fTemp659: F32 = self.fTempPerm634;
		let mut fSlow354: F32 = ((iSlow113 != 0) as i32) as u32 as F32;
		let mut fTemp660: F32 = self.fTempPerm635;
		let mut fTemp661: F32 = self.fTempPerm636;
		let mut fSlow355: F32 = ((iSlow213 != 0) as i32) as u32 as F32;
		let mut fTemp662: F32 = self.fTempPerm637;
		let mut fSlow356: F32 = ((iSlow216 != 0) as i32) as u32 as F32;
		let mut fTemp663: F32 = self.fTempPerm638;
		let mut fSlow357: F32 = ((iSlow104 != 0) as i32) as u32 as F32;
		let mut fTemp664: F32 = self.fTempPerm639;
		let mut fSlow358: F32 = ((iSlow107 != 0) as i32) as u32 as F32;
		let mut fSlow359: F32 = self.fEntry27 + self.fEntry26 + self.fCheckbox2 + self.fHslider51 + self.fHslider50 + self.fHslider49 + self.fHslider48 + self.fHslider47 + self.fHslider46 + self.fHslider45 + self.fHslider44 + self.fEntry25 + self.fEntry24 + self.fCheckbox1 + self.fHslider43 + self.fHslider42 + self.fHslider41 + self.fHslider40 + self.fHslider39 + self.fHslider38 + self.fHslider37 + self.fHslider36 + self.fEntry23 + self.fEntry22 + self.fHslider35 + self.fHslider34 + self.fHslider33 + self.fHslider32 + self.fHslider31 + self.fHslider30 + self.fHslider29 + self.fEntry21 + self.fEntry20 + self.fHslider28 + self.fHslider27 + self.fHslider26 + self.fHslider25 + self.fHslider24 + self.fHslider23 + self.fHslider22 + self.fHslider21;
		let zipped_iterators = outputs0.zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7).zip(outputs8);
		for ((((((((output0, output1), output2), output3), output4), output5), output6), output7), output8) in zipped_iterators {
			self.fVec0[0] = fSlow87;
			self.iVec1[0] = 1;
			let mut iTemp0: i32 = (fSlow87 > self.fVec0[1]) as i32;
			self.fVec2[0] = fSlow88;
			let mut fTemp1: F32 = if ((fSlow88 != self.fVec2[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec1[1] + -1.0};
			self.fRec1[0] = fTemp1;
			self.fRec2[0] = if iTemp0 != 0 {fSlow88} else {if (fTemp1 > 0.0) as i32 != 0 {self.fRec2[1] + (fSlow88 - self.fRec2[1]) / F32::max(1.0, fTemp1)} else {fSlow88}};
			let mut fTemp2: F32 = F32::abs(self.fRec2[0]);
			let mut fTemp3: F32 = F32::exp(0.057762265 * (((self.fRec2[0] > 0.0) as i32) - ((self.fRec2[0] < 0.0) as i32)) as F32 * if (fTemp2 < 0.4) as i32 != 0 {F32::min(0.99, 2.5 * fTemp2)} else {if (fTemp2 < 0.82) as i32 != 0 {F32::min(6.0, F32::floor(16.666666 * (fTemp2 + -0.4))) + 1.0} else {(if (fTemp2 < 0.91) as i32 != 0 {12} else {24}) as F32}});
			self.fVec3[0] = fSlow97;
			let mut fTemp4: F32 = if ((fSlow97 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec3[1] + -1.0};
			self.fRec3[0] = fTemp4;
			self.fRec4[0] = if iTemp0 != 0 {fSlow97} else {if (fTemp4 > 0.0) as i32 != 0 {self.fRec4[1] + (fSlow97 - self.fRec4[1]) / F32::max(1.0, fTemp4)} else {fSlow97}};
			let mut fTemp5: F32 = F32::exp(0.0005776227 * self.fRec4[0] * fSlow96);
			self.fVec4[0] = fSlow98;
			let mut fTemp6: F32 = if ((fSlow98 != self.fVec4[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec5[1] + -1.0};
			self.fRec5[0] = fTemp6;
			self.fRec6[0] = if iTemp0 != 0 {fSlow98} else {if (fTemp6 > 0.0) as i32 != 0 {self.fRec6[1] + (fSlow98 - self.fRec6[1]) / F32::max(1.0, fTemp6)} else {fSlow98}};
			let mut iTemp7: i32 = (F32::abs(self.fRec6[0]) > 0.0001) as i32;
			let mut fTemp8: F32 = (iTemp7) as F32;
			if (iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow71)) | (((fTemp8 != 0.0) as i32) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | (iSlow109 & (iSlow106 & iSlow102))))))) != 0 {
				iTemp9 = i32::wrapping_sub(1, self.iVec1[1]);
			}
				if (fTemp8 != 0.0) as i32 != 0 {
					fTemp10 = if iTemp9 != 0 {0.0} else {self.fConst1 + self.fRec8[1]};
					self.fRec8[0] = fTemp10 - F32::floor(fTemp10);
					fTemp11 = if iTemp9 != 0 {0.0} else {fSlow121 + self.fRec9[1]};
					self.fRec9[0] = fTemp11 - F32::floor(fTemp11);
					self.iRec11[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec11[1], i32::wrapping_mul(iSlow122, iTemp9))), 12345);
					iTemp12 = i32::wrapping_mul(1664525, self.iRec11[0] ^ (self.iRec11[0] >> 15));
					self.fRec10[0] = self.fConst3 * self.fRec10[1] + self.fConst5 * (i32::wrapping_mul(1013904223, iTemp12 ^ (iTemp12 >> 13))) as F32;
				}
					let mut fTemp13: F32 = (i32::wrapping_sub(1, iTemp0)) as F32;
					let mut fTemp14: F32 = 1.0 - self.fConst6 * fTemp13;
					if (fTemp8 != 0.0) as i32 != 0 {
						self.fRec12[0] = fSlow123 * fTemp14 + self.fConst6 * fTemp13 * self.fRec12[1];
						fTemp15 = if iTemp9 != 0 {0.0} else {fSlow128 + self.fRec15[1]};
						self.fRec15[0] = fTemp15 - F32::floor(fTemp15);
					}
						self.iRec16[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec16[1]), 12345);
						let mut fTemp16: F32 = (self.iRec16[0]) as F32;
						self.fVec6[0] = fTemp16;
						let mut fTemp17: F32 = 4.656613e-10 * fTemp16;
						if (fTemp8 != 0.0) as i32 != 0 {
							self.fRec14[0] = if (self.fRec15[0] < self.fRec15[1]) as i32 != 0 {fTemp17} else {self.fRec14[1]};
							fTemp18 = F32::floor(32768.0 * self.fRec14[0] + 0.5);
							self.fVec7[0] = fTemp18;
							self.fRec13[0] = fSlow129 * (3.0517578e-05 * (fTemp18 + self.fVec7[1]) - fSlow127 * self.fRec13[1]);
							fTemp19 = fTemp8 * (self.fRec6[0] * (fSlow132 + 2e+01 * self.fRec13[0] + self.fConst7 * self.fRec12[0] * self.fRec10[0]) + OscDsp_faustpower2_f(self.fRec6[0]) * (18.0 * unsafe { ftbl2OscDspSIG2[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec9[0]) as i32, 65535))) as usize] } + 2.0 * unsafe { ftbl2OscDspSIG2[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec8[0]) as i32, 65535))) as usize] }));
						}
							self.fRec17[0] = fSlow133 * fTemp14 + self.fConst6 * fTemp13 * self.fRec17[1];
							self.fVec8[0] = fSlow136;
							let mut fTemp20: F32 = if ((fSlow136 != self.fVec8[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec18[1] + -1.0};
							self.fRec18[0] = fTemp20;
							self.fRec19[0] = if iTemp0 != 0 {fSlow136} else {if (fTemp20 > 0.0) as i32 != 0 {self.fRec19[1] + (fSlow136 - self.fRec19[1]) / F32::max(1.0, fTemp20)} else {fSlow136}};
							let mut fTemp21: F32 = F32::exp(0.057762265 * (self.fRec19[0] + fSlow135 * self.fRec17[0] + 0.01 * ((iTemp7 != 0) as i32) as u32 as F32 * fTemp19));
							self.fRec20[0] = fSlow140 + fSlow139 * self.fRec20[1];
							let mut fTemp22: F32 = F32::exp(self.fRec20[0]);
							let mut fTemp23: F32 = fTemp22 * fTemp21 * fTemp5;
							let mut fTemp24: F32 = fTemp23 * fTemp3;
							if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow44 & iSlow41)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10))))))))))))))))))))))))) != 0 {
								fTemp25 = self.fConst2 * fTemp24;
								fTemp26 = fTemp25 + self.fRec0[1];
							}
								self.fRec21[0] = if iTemp0 != 0 {fTemp17} else {self.fRec21[1]};
								let mut fTemp27: F32 = 0.5 * (self.fRec21[0] + 1.0);
								let mut fTemp28: F32 = fSlow141 + (fTemp27 - F32::floor(fSlow141 + fTemp27));
								let mut fTemp29: F32 = fSlow142 * fTemp28;
								if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow44 & iSlow41)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow224)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow221)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | ((iSlow2 & (iSlow6 & iSlow10)) | ((iSlow109 & (iSlow106 & iSlow218)) | ((iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow194)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | ((iSlow109 & (iSlow106 & iSlow185)) | ((iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow179)) | ((iSlow109 & (iSlow149 & iSlow176)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow170)) | ((iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | ((iSlow109 & (iSlow149 & iSlow155)) | ((iSlow109 & (iSlow149 & iSlow152)) | (iSlow109 & (iSlow149 & iSlow146)))))))))))))))))))))))))))))))))))))))))))))))))))))) != 0 {
									fTemp30 = fTemp29 - F32::floor(fTemp29);
								}
									if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow44 & iSlow41)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10))))))))))))))))))))))))) != 0 {
										self.fRec0[0] = if iTemp0 != 0 {fTemp30} else {fTemp26 - F32::floor(fTemp26)};
									}
										if (iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow50)) | (iSlow2 & (iSlow6 & iSlow10))) != 0 {
											iTemp31 = (self.fRec0[0] < 0.5) as i32;
										}
											if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow224)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow221)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
												self.fVec9[0] = fSlow225;
												fTemp32 = if ((fSlow225 != self.fVec9[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec22[1] + -1.0};
												self.fRec22[0] = fTemp32;
												self.fRec23[0] = if iTemp0 != 0 {fSlow225} else {if (fTemp32 > 0.0) as i32 != 0 {self.fRec23[1] + (fSlow225 - self.fRec23[1]) / F32::max(1.0, fTemp32)} else {fSlow225}};
											}
												if (iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow16)) | (iSlow2 & (iSlow6 & iSlow10)))) != 0 {
													fTemp33 = 1.0 - self.fRec23[0];
												}
													if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow224)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow221)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
														self.fVec10[0] = fSlow226;
														fTemp34 = if ((fSlow226 != self.fVec10[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec24[1] + -1.0};
														self.fRec24[0] = fTemp34;
														self.fRec25[0] = if iTemp0 != 0 {fSlow226} else {if (fTemp34 > 0.0) as i32 != 0 {self.fRec25[1] + (fSlow226 - self.fRec25[1]) / F32::max(1.0, fTemp34)} else {fSlow226}};
													}
														if iSlow2 & (iSlow6 & iSlow10) != 0 {
															fTemp35 = 1.0 - self.fRec25[0];
															fTemp36 = if iTemp31 != 0 {fTemp35} else {fTemp33};
															fTemp37 = if (fTemp36 < 0.999) as i32 != 0 {F32::max(0.01, fTemp36)} else {1.0};
														}
															if (iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow10))) != 0 {
																fTemp38 = 2.0 * self.fRec0[0];
															}
																if iSlow2 & (iSlow6 & iSlow10) != 0 {
																	fTemp39 = F32::floor(fTemp38);
																	fTemp40 = fTemp38 - (fTemp37 + fTemp39);
																	fTemp41 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp40 / fTemp24)));
																	fTemp42 = if iTemp31 != 0 {fSlow228} else {fSlow227};
																	fTemp43 = if (fTemp42 >= 2.0) as i32 != 0 {(if (fTemp42 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp42 >= 1.0) as i32 != 0 {4.0} else {6.2831855}};
																	fTemp44 = fTemp22 * fTemp43 * fTemp21 * fTemp5 * fTemp3;
																	iTemp45 = (fTemp42) as i32;
																	fTemp46 = self.fConst0 * F32::abs(self.fConst11 * (fTemp24 / fTemp37));
																	fTemp47 = 0.5 * wt_basic(iTemp45, fTemp46, self.fConst0, 0.999999);
																	fTemp48 = self.fConst11 * fTemp24;
																	fTemp49 = fTemp38 - fTemp39;
																	fTemp50 = ((((fTemp37 < 1.0) as i32) & ((fTemp49 >= fTemp37) as i32)) & ((fTemp40 < fTemp48) as i32)) as F32;
																	fTemp51 = 1.0 - fTemp41;
																	fTemp52 = if iTemp31 != 0 {fSlow227} else {fSlow228};
																	fTemp53 = if iTemp31 != 0 {fTemp33} else {fTemp35};
																	fTemp54 = if (fTemp53 < 0.999) as i32 != 0 {F32::max(0.01, fTemp53)} else {1.0};
																	fTemp55 = ((fTemp54 == 1.0) as i32) as u32 as F32;
																	fTemp56 = 0.5 * (wt_basic(iTemp45, fTemp46, self.fConst0, 0.0) - fTemp55 * wt_basic((fTemp52) as i32, self.fConst0 * F32::abs(self.fConst11 * (fTemp24 / fTemp54)), self.fConst0, 0.999999));
																	fTemp57 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp49 / fTemp24)));
																	fTemp58 = 1.0 - fTemp57;
																	fTemp59 = fTemp43 / fTemp37 - fTemp55 * if (fTemp52 >= 2.0) as i32 != 0 {(if (fTemp52 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp52 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} / fTemp54;
																	fTemp60 = ((fTemp49 < fTemp48) as i32) as u32 as F32;
																	self.fVec11[0] = fTemp60 * OscDsp_faustpower2_f(fTemp58) * (self.fConst10 * fTemp24 * fTemp59 * fTemp58 - fTemp56) + fTemp50 * OscDsp_faustpower2_f(fTemp51) * (fTemp47 - self.fConst10 * (fTemp44 * fTemp51 / fTemp37));
																	self.fVec12[0] = ((fTemp49 < fTemp37) as i32) as u32 as F32 * wt_basic(iTemp45, fTemp46, self.fConst0, fTemp49 / fTemp37);
																	fTemp61 = fSlow9 * (self.fVec12[1] + fTemp60 * OscDsp_faustpower2_f(fTemp57) * (fTemp56 + self.fConst10 * fTemp24 * fTemp57 * fTemp59) + self.fVec11[1] - fTemp50 * OscDsp_faustpower2_f(fTemp41) * (fTemp47 + self.fConst10 * (fTemp44 * fTemp41 / fTemp37)));
																}
																	if iSlow2 & (iSlow6 & iSlow13) != 0 {
																		fTemp62 = self.fConst12 * fTemp24 * F32::exp(0.0046209814 * self.fRec23[0]);
																		fTemp63 = fTemp62 + self.fRec26[1];
																	}
																		if (iSlow2 & (iSlow6 & iSlow13)) | (iSlow109 & (iSlow149 & iSlow152)) != 0 {
																			fTemp64 = fSlow231 * fTemp28;
																			fTemp65 = F32::floor(fTemp64);
																			fTemp66 = fTemp64 - (fTemp65 + F32::floor(fTemp64 - fTemp65));
																		}
																			if iSlow2 & (iSlow6 & iSlow13) != 0 {
																				self.fRec26[0] = if iTemp0 != 0 {fTemp66} else {fTemp63 - F32::floor(fTemp63)};
																			}
																				if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow224)) | ((iSlow2 & (iSlow6 & iSlow25)) | (iSlow2 & (iSlow6 & iSlow13)))) != 0 {
																					fTemp67 = F32::max(0.0, F32::min(1.0, self.fRec25[0]));
																				}
																					if iSlow2 & (iSlow6 & iSlow13) != 0 {
																						fTemp68 = 4.0 * fTemp67;
																						fTemp69 = F32::min(3.0, fTemp68);
																					}
																						if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow13)))))))) != 0 {
																							fTemp70 = F32::abs(fTemp25);
																						}
																							if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow13))))))) != 0 {
																								fTemp71 = self.fConst0 * fTemp70;
																							}
																								if iSlow2 & (iSlow6 & iSlow13) != 0 {
																									fTemp72 = wt_basic(2, fTemp71, self.fConst0, self.fRec0[0]);
																									self.fVec13[0] = wt_basic(0, fTemp71, self.fConst0, self.fRec0[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp69)) + wt_basic(1, fTemp71, self.fConst0, self.fRec0[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp69 + -1.0)) + fTemp72 * F32::max(0.0, 1.0 - F32::abs(fTemp69 + -2.0)) + (fTemp72 - wt_basic(2, fTemp71, self.fConst0, self.fRec0[0] + (-0.5 - 0.42 * F32::max(0.0, fTemp68 + -3.0)))) * F32::max(0.0, 1.0 - F32::abs(fTemp69 + -3.0)) + self.fRec23[0] * wt_basic(iSlow230, self.fConst0 * F32::abs(fTemp62), self.fConst0, self.fRec26[0]);
																									fTemp73 = fSlow12 * (self.fVec13[1] / F32::sqrt(OscDsp_faustpower2_f(self.fRec23[0]) + 1.0));
																								}
																									if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																										iTemp74 = (self.fRec0[0] < self.fRec0[1]) as i32;
																									}
																										if (iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																											self.iRec27[0] = (i32::wrapping_add(iTemp74, self.iRec27[1])) % 2;
																										}
																											if iSlow2 & (iSlow6 & iSlow16) != 0 {
																												self.fVec14[0] = wt_basic(iSlow230, self.fConst0 * F32::abs(self.fConst12 * fTemp24), self.fConst0, 0.5 * (self.fRec0[0] + (self.iRec27[0]) as F32));
																											}
																												if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																													fTemp75 = 15.0 * self.fRec25[0];
																													fTemp76 = fTemp75 + 1.0;
																													fTemp77 = fTemp24 * fTemp76;
																													fTemp78 = self.fConst0 * F32::abs(self.fConst2 * fTemp77);
																													fTemp79 = wt_basic(iSlow233, fTemp78, self.fConst0, fTemp76);
																													fTemp80 = wt_basic(iSlow233, fTemp78, self.fConst0, 0.0);
																												}
																													if iSlow2 & (iSlow6 & iSlow16) != 0 {
																														fTemp81 = 0.5 * (fTemp80 - fTemp79);
																													}
																														if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																															fTemp82 = F32::max(0.0, F32::min(1.0, self.fConst0 * (self.fRec0[0] / fTemp24)));
																															fTemp83 = 1.0 - fTemp82;
																														}
																															if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																fTemp84 = fTemp75 + (1.0 - F32::floor(fTemp76));
																																fTemp85 = fSlow238 - if iSlow234 != 0 {fSlow237} else {if iSlow235 != 0 {(i32::wrapping_sub(4, i32::wrapping_mul(8, ((fTemp84 >= 0.25) as i32) & ((fTemp84 < 0.75) as i32)))) as F32} else {6.2831855 * F32::cos(6.2831855 * fTemp84)}};
																															}
																																if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																																	fTemp86 = (iTemp74) as F32;
																																	fTemp87 = fTemp86 * OscDsp_faustpower2_f(fTemp83);
																																}
																																	if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																		self.fVec15[0] = fTemp87 * (self.fConst13 * fTemp77 * fTemp85 * fTemp83 - fTemp81);
																																	}
																																		if (iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | (iSlow2 & (iSlow6 & iSlow16))) != 0 {
																																			fTemp88 = fTemp86 * OscDsp_faustpower2_f(fTemp82);
																																		}
																																			if (iSlow2 & (iSlow6 & iSlow22)) | (iSlow2 & (iSlow6 & iSlow16)) != 0 {
																																				fTemp89 = wt_basic(iSlow233, fTemp78, self.fConst0, self.fRec0[0] * fTemp76);
																																				self.fVec16[0] = fTemp89;
																																			}
																																				if iSlow2 & (iSlow6 & iSlow16) != 0 {
																																					fTemp90 = fSlow15 * (fTemp33 * (self.fVec16[1] + fTemp88 * (fTemp81 + self.fConst13 * fTemp77 * fTemp82 * fTemp85) + self.fVec15[1]) + self.fRec23[0] * self.fVec14[1]);
																																				}
																																					if iSlow2 & (iSlow6 & iSlow19) != 0 {
																																						fTemp91 = if self.iRec27[0] != 0 {fSlow228} else {fSlow227};
																																						fTemp92 = 7.0 * self.fRec25[0] + 1.0;
																																						fTemp93 = fTemp92 * (self.fRec23[0] + 1.0);
																																						fTemp94 = if self.iRec27[0] != 0 {fTemp92} else {fTemp93};
																																						fTemp95 = if self.iRec27[0] != 0 {fSlow227} else {fSlow228};
																																						iTemp96 = (fTemp95) as i32;
																																						fTemp97 = if self.iRec27[0] != 0 {fTemp93} else {fTemp92};
																																						fTemp98 = 0.5 * (fTemp97 * wt_basic(iTemp96, fTemp71, self.fConst0, 0.0) - fTemp94 * wt_basic((fTemp91) as i32, fTemp71, self.fConst0, 0.999999));
																																						fTemp99 = if (fTemp95 >= 2.0) as i32 != 0 {(if (fTemp95 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp95 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp97 - if (fTemp91 >= 2.0) as i32 != 0 {(if (fTemp91 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp91 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp94;
																																						self.fVec17[0] = fTemp87 * (self.fConst13 * fTemp24 * fTemp83 * fTemp99 - fTemp98);
																																						self.fVec18[0] = wt_basic(iTemp96, fTemp71, self.fConst0, self.fRec0[0]) * fTemp97;
																																						fTemp100 = self.fVec18[1] + fTemp88 * (fTemp98 + self.fConst13 * fTemp24 * fTemp82 * fTemp99) + self.fVec17[1];
																																						self.fVec19[0] = fTemp100;
																																						fTemp101 = fTemp100 - self.fVec19[1];
																																						iTemp102 = (F32::abs(fTemp101) < 0.0001) as i32;
																																						fTemp103 = 4.0 * F32::floor(0.25 * (self.fVec19[1] + 1.0));
																																						fTemp104 = self.fVec19[1] - fTemp103;
																																						fTemp105 = 4.0 * F32::floor(0.25 * (fTemp100 + 1.0));
																																						fTemp106 = fTemp100 - fTemp105;
																																						fTemp107 = 0.125 * (fTemp100 + self.fVec19[1]);
																																						fTemp108 = fSlow18 * if iTemp102 != 0 {1.0 - 4.0 * F32::abs(fTemp107 + (-0.25 - F32::floor(fTemp107 + 0.25)))} else {(if (fTemp106 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp106)} else {1.0 - 0.5 * OscDsp_faustpower2_f(fTemp100 + (-2.0 - fTemp105))} - if (fTemp104 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp104)} else {1.0 - 0.5 * OscDsp_faustpower2_f(self.fVec19[1] + (-2.0 - fTemp103))}) / if iTemp102 != 0 {1.0} else {fTemp101}};
																																					}
																																						if iSlow2 & (iSlow6 & iSlow22) != 0 {
																																							fTemp109 = 4.0 * F32::max(0.0, F32::min(1.0, self.fRec23[0]));
																																							iTemp110 = (F32::min(3.0, fTemp109)) as i32;
																																							iTemp111 = i32::wrapping_add(iTemp110, 1);
																																							iTemp112 = (iTemp111 >= 3) as i32;
																																							iTemp113 = (iTemp111 >= 2) as i32;
																																							iTemp114 = (iTemp111 >= 1) as i32;
																																							iTemp115 = (iTemp111 >= 4) as i32;
																																							fTemp116 = 0.5 * (wt_basic(iSlow230, fTemp71, self.fConst0, 0.999999) + 1.0);
																																							fTemp117 = (iTemp110) as F32;
																																							fTemp118 = fTemp109 - fTemp117;
																																							iTemp119 = (iTemp110 >= 3) as i32;
																																							iTemp120 = (iTemp110 >= 2) as i32;
																																							iTemp121 = (iTemp110 >= 1) as i32;
																																							iTemp122 = (iTemp110 >= 4) as i32;
																																							fTemp123 = fTemp117 + (1.0 - fTemp109);
																																							fTemp124 = 0.5 * (wt_basic(iSlow230, fTemp71, self.fConst0, 0.0) + 1.0);
																																							fTemp125 = fTemp80 * (fTemp123 * if iTemp119 != 0 {if iTemp122 != 0 {fTemp124} else {0.0}} else {if iTemp120 != 0 {0.0} else {(if iTemp121 != 0 {0} else {1}) as F32}} + fTemp118 * if iTemp112 != 0 {if iTemp115 != 0 {fTemp124} else {0.0}} else {if iTemp113 != 0 {0.0} else {(if iTemp114 != 0 {0} else {1}) as F32}}) - fTemp79 * (fTemp123 * if iTemp119 != 0 {if iTemp122 != 0 {fTemp116} else {6e-06}} else {if iTemp120 != 0 {0.0} else {if iTemp121 != 0 {2e-06} else {1e-06}}} + fTemp118 * if iTemp112 != 0 {if iTemp115 != 0 {fTemp116} else {6e-06}} else {if iTemp113 != 0 {0.0} else {if iTemp114 != 0 {2e-06} else {1e-06}}});
																																							self.fVec20[0] = fTemp87 * fTemp125;
																																						}
																																							if (iSlow2 & (iSlow44 & iSlow50)) | (iSlow2 & (iSlow6 & iSlow22)) != 0 {
																																								fTemp126 = 1.0 - self.fRec0[0];
																																							}
																																								if iSlow2 & (iSlow6 & iSlow22) != 0 {
																																									fTemp127 = 1.0 - F32::abs(fTemp38 + -1.0);
																																									fTemp128 = F32::sin(3.1415927 * self.fRec0[0]);
																																									fTemp129 = F32::min(1.0, 6.0 * F32::min(self.fRec0[0], fTemp126));
																																									fTemp130 = 0.5 * (wt_basic(iSlow230, fTemp71, self.fConst0, self.fRec0[0]) + 1.0);
																																									self.fVec21[0] = fTemp89 * (fTemp123 * if iTemp119 != 0 {if iTemp122 != 0 {fTemp130} else {fTemp129}} else {if iTemp120 != 0 {fTemp128} else {if iTemp121 != 0 {fTemp127} else {fTemp126}}} + fTemp118 * if iTemp112 != 0 {if iTemp115 != 0 {fTemp130} else {fTemp129}} else {if iTemp113 != 0 {fTemp128} else {if iTemp114 != 0 {fTemp127} else {fTemp126}}});
																																									fTemp131 = fSlow21 * (self.fVec21[1] + 0.5 * (fTemp88 * fTemp125 - self.fVec20[1]));
																																								}
																																									if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																										fTemp132 = self.fConst2 * fTemp24 * (2.17 * fTemp67 + 3.0);
																																										fTemp133 = fTemp132 + self.fRec28[1];
																																									}
																																										if (iSlow2 & (iSlow6 & iSlow25)) | (iSlow109 & (iSlow149 & iSlow164)) != 0 {
																																											fTemp134 = F32::floor(fTemp29 + 0.67);
																																											fTemp135 = fTemp29 + (0.67 - (fTemp134 + F32::floor(fTemp29 + (0.67 - fTemp134))));
																																										}
																																											if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																												self.fRec28[0] = if iTemp0 != 0 {fTemp135} else {fTemp133 - F32::floor(fTemp133)};
																																												self.fVec22[0] = wt_basic(iSlow230, self.fConst0 * F32::abs(fTemp132), self.fConst0, self.fRec28[0]);
																																												fTemp136 = self.fConst2 * fTemp24 * (1.41 * fTemp67 + 2.0);
																																												fTemp137 = fTemp136 + self.fRec29[1];
																																											}
																																												if (iSlow2 & (iSlow6 & iSlow25)) | (iSlow109 & (iSlow149 & iSlow164)) != 0 {
																																													fTemp138 = F32::floor(fTemp29 + 0.31);
																																													fTemp139 = fTemp29 + (0.31 - (fTemp138 + F32::floor(fTemp29 + (0.31 - fTemp138))));
																																												}
																																													if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																														self.fRec29[0] = if iTemp0 != 0 {fTemp139} else {fTemp137 - F32::floor(fTemp137)};
																																														self.fVec23[0] = wt_basic(iSlow230, self.fConst0 * F32::abs(fTemp136), self.fConst0, self.fRec29[0]);
																																													}
																																														if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | (iSlow2 & (iSlow6 & iSlow25)))) != 0 {
																																															fTemp140 = wt_basic(iSlow233, fTemp71, self.fConst0, self.fRec0[0]);
																																															self.fVec24[0] = fTemp140;
																																														}
																																															if iSlow2 & (iSlow6 & iSlow25) != 0 {
																																																fTemp141 = fSlow24 * (0.57735026 * fTemp33 * (self.fVec24[1] + self.fVec23[1] + self.fVec22[1]) + 1.5 * self.fRec23[0] * self.fVec24[1] * self.fVec23[1] * self.fVec22[1]);
																																															}
																																																if iSlow2 & (iSlow6 & iSlow28) != 0 {
																																																	iTemp142 = (F32::max(0.0, F32::min(1e+01, 10.999 * self.fRec25[0]))) as i32;
																																																	fTemp143 = unsafe { ftbl3OscDspSIG3[(i32::wrapping_add(iTemp142, 1)) as usize] };
																																																	fTemp144 = self.fConst2 * fTemp24 * fTemp143;
																																																	fTemp145 = fTemp144 + self.fRec30[1];
																																																	fTemp146 = fSlow142 * fTemp28 * fTemp143;
																																																	fTemp147 = F32::floor(fTemp146);
																																																	self.fRec30[0] = if iTemp0 != 0 {fTemp146 - (fTemp147 + F32::floor(fTemp146 - fTemp147))} else {fTemp145 - F32::floor(fTemp145)};
																																																	self.fVec25[0] = wt_basic(iSlow230, self.fConst0 * F32::abs(fTemp144), self.fConst0, self.fRec30[0]);
																																																	fTemp148 = unsafe { ftbl3OscDspSIG3[iTemp142 as usize] };
																																																	fTemp149 = self.fConst2 * fTemp24 * fTemp148;
																																																	fTemp150 = fTemp149 + self.fRec31[1];
																																																	fTemp151 = fSlow142 * fTemp28 * fTemp148;
																																																	fTemp152 = F32::floor(fTemp151);
																																																	self.fRec31[0] = if iTemp0 != 0 {fTemp151 - (fTemp152 + F32::floor(fTemp151 - fTemp152))} else {fTemp150 - F32::floor(fTemp150)};
																																																	self.fVec26[0] = wt_basic(iSlow230, self.fConst0 * F32::abs(fTemp149), self.fConst0, self.fRec31[0]);
																																																	fTemp153 = fSlow27 * self.fVec24[1] * (fTemp33 * self.fVec26[1] + self.fRec23[0] * self.fVec25[1]);
																																																}
																																																	if iSlow2 & (iSlow6 & iSlow221) != 0 {
																																																		fTemp154 = unsafe { ftbl4OscDspSIG4[((F32::max(0.0, F32::min(9.0, 9.999 * self.fRec23[0]))) as i32) as usize] };
																																																		fTemp155 = self.fConst2 * fTemp24 * fTemp154;
																																																		fTemp156 = F32::max(F32::abs(fTemp155), 1e-09);
																																																		iTemp157 = (fTemp155 < 0.0) as i32;
																																																		fTemp158 = fTemp155 + self.fRec32[1];
																																																		fTemp159 = fSlow142 * fTemp28 * fTemp154;
																																																		fTemp160 = F32::floor(fTemp159);
																																																		self.fRec32[0] = if iTemp0 != 0 {fTemp159 - (fTemp160 + F32::floor(fTemp159 - fTemp160))} else {fTemp158 - F32::floor(fTemp158)};
																																																		fTemp161 = if iTemp157 != 0 {fSlow248 - (self.fRec32[0] + F32::floor(fSlow248 - self.fRec32[0]))} else {self.fRec32[0] - (fSlow248 + F32::floor(self.fRec32[0] - fSlow248))};
																																																		iTemp162 = ((fTemp161 < fTemp156) as i32) * if iTemp157 != 0 {-iSlow246} else {iSlow246};
																																																		fTemp163 = if iTemp157 != 0 {fSlow250 - (self.fRec32[0] + F32::floor(fSlow250 - self.fRec32[0]))} else {self.fRec32[0] - (fSlow250 + F32::floor(self.fRec32[0] - fSlow250))};
																																																		iTemp164 = ((fTemp163 < fTemp156) as i32) * if iTemp157 != 0 {-iSlow249} else {iSlow249};
																																																		fTemp165 = F32::floor(self.fRec32[0]);
																																																		fTemp166 = self.fRec32[0] - fTemp165;
																																																		fTemp167 = if iSlow244 != 0 {if iSlow245 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp166 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec32[0] + (0.5 - (fTemp165 + F32::floor(self.fRec32[0] + (0.5 - fTemp165))))) + -1.0}} else {if iSlow247 != 0 {1.0 - 4.0 * F32::abs(self.fRec32[0] + (-0.25 - (fTemp165 + F32::floor(self.fRec32[0] + (0.25 - fTemp165)))))} else {F32::sin(6.2831855 * fTemp166)}};
																																																		self.fVec27[0] = fTemp167;
																																																	}
																																																		if (iSlow2 & (iSlow6 & iSlow31)) | (iSlow2 & (iSlow6 & iSlow221)) != 0 {
																																																			fTemp168 = OscDsp_faustpower2_f(self.fRec25[0]);
																																																		}
																																																			if iSlow2 & (iSlow6 & iSlow221) != 0 {
																																																				fTemp169 = self.fConst2 * fTemp24 * (8.0 * fTemp168 * fTemp154 * (0.5 * (fTemp167 + self.fVec27[1] - (i32::wrapping_add(iTemp164, iTemp162)) as F32) + (fTemp163 * (iTemp164) as F32 + fTemp161 * (iTemp162) as F32) / fTemp156) + 1.0);
																																																				iTemp170 = (fTemp169 < 0.0) as i32;
																																																				fTemp171 = 0.5 * (if iTemp170 != 0 {-iSlow251} else {iSlow251}) as F32;
																																																				fTemp172 = F32::max(F32::abs(fTemp169), 1e-09);
																																																				fTemp173 = self.fRec33[1] + fTemp169;
																																																				self.fRec33[0] = if iTemp0 != 0 {fTemp30} else {fTemp173 - F32::floor(fTemp173)};
																																																				fTemp174 = if iTemp170 != 0 {fSlow252 - (self.fRec33[0] + F32::floor(fSlow252 - self.fRec33[0]))} else {self.fRec33[0] - (fSlow252 + F32::floor(self.fRec33[0] - fSlow252))};
																																																				fTemp175 = F32::max(0.0, F32::min(1.0, fTemp174 / fTemp172));
																																																				fTemp176 = 1.0 - fTemp175;
																																																				fTemp177 = ((fTemp174 < fTemp172) as i32) as u32 as F32;
																																																				fTemp178 = 0.5 * (if iTemp170 != 0 {-iSlow254} else {iSlow254}) as F32;
																																																				fTemp179 = if iTemp170 != 0 {fSlow255 - (self.fRec33[0] + F32::floor(fSlow255 - self.fRec33[0]))} else {self.fRec33[0] - (fSlow255 + F32::floor(self.fRec33[0] - fSlow255))};
																																																				fTemp180 = F32::max(0.0, F32::min(1.0, fTemp179 / fTemp172));
																																																				fTemp181 = 1.0 - fTemp180;
																																																				fTemp182 = ((fTemp179 < fTemp172) as i32) as u32 as F32;
																																																				self.fVec28[0] = fTemp182 * OscDsp_faustpower2_f(fTemp181) * (0.16666667 * fTemp172 * fSlow256 * fTemp181 - fTemp178) + fTemp177 * OscDsp_faustpower2_f(fTemp176) * (0.16666667 * fTemp172 * fSlow253 * fTemp176 - fTemp171);
																																																				fTemp183 = F32::floor(self.fRec33[0]);
																																																				fTemp184 = self.fRec33[0] - fTemp183;
																																																				self.fVec29[0] = if iSlow234 != 0 {if iSlow236 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp184 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec33[0] + (0.5 - (fTemp183 + F32::floor(self.fRec33[0] + (0.5 - fTemp183))))) + -1.0}} else {if iSlow235 != 0 {1.0 - 4.0 * F32::abs(self.fRec33[0] + (-0.25 - (fTemp183 + F32::floor(self.fRec33[0] + (0.25 - fTemp183)))))} else {F32::sin(6.2831855 * fTemp184)}};
																																																				fTemp185 = fSlow220 * (fTemp177 * OscDsp_faustpower2_f(fTemp175) * (fTemp171 + 0.16666667 * fTemp172 * fTemp175 * fSlow253) + self.fVec29[1] + fTemp182 * OscDsp_faustpower2_f(fTemp180) * (fTemp178 + 0.16666667 * fTemp172 * fTemp180 * fSlow256) + self.fVec28[1]);
																																																			}
																																																				if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																					fTemp186 = F32::max(1e+01, F32::min(self.fConst14, fTemp24 * F32::exp(0.6931472 * (8.0 * self.fRec23[0] + -3.0))));
																																																					fTemp187 = 1.0 / F32::tan(self.fConst8 * fTemp186);
																																																				}
																																																					if (iSlow2 & (iSlow6 & iSlow31)) | (iSlow109 & (iSlow149 & iSlow173)) != 0 {
																																																						fTemp188 = 4.656613e-10 * (fTemp16 + self.fVec6[1]);
																																																					}
																																																						if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																							self.fRec34[0] = (fTemp188 - self.fRec34[1] * (1.0 - fTemp187)) / (fTemp187 + 1.0);
																																																							fTemp189 = fSlow30 * (self.fVec24[1] * (1.5 * self.fRec25[0] * self.fRec34[0] * F32::sqrt(self.fConst15 / fTemp186) + 1.0) / F32::sqrt(2.25 * fTemp168 + 1.0));
																																																						}
																																																							if iSlow2 & (iSlow6 & iSlow224) != 0 {
																																																								fTemp190 = 0.6 * fTemp67 * self.fRec36[1] + 1.0;
																																																								fTemp191 = self.fConst2 * fTemp24 * fTemp190;
																																																								fTemp192 = self.fRec35[1] + fTemp191;
																																																								iTemp193 = (fTemp192 >= 1.0) as i32;
																																																								self.fRec35[0] = if iTemp0 != 0 {fTemp30} else {fTemp192 - (iTemp193) as F32};
																																																								self.fRec36[0] = if iTemp193 != 0 {fTemp17} else {self.fRec36[1]};
																																																								fTemp194 = F32::max(F32::abs(fTemp191), 1e-09);
																																																								iTemp195 = (fTemp191 < 0.0) as i32;
																																																								fTemp196 = if iTemp195 != 0 {fSlow248 - (self.fRec35[0] + F32::floor(fSlow248 - self.fRec35[0]))} else {self.fRec35[0] - (fSlow248 + F32::floor(self.fRec35[0] - fSlow248))};
																																																								iTemp197 = ((fTemp196 < fTemp194) as i32) * if iTemp195 != 0 {-iSlow246} else {iSlow246};
																																																								fTemp198 = if iTemp195 != 0 {fSlow250 - (self.fRec35[0] + F32::floor(fSlow250 - self.fRec35[0]))} else {self.fRec35[0] - (fSlow250 + F32::floor(self.fRec35[0] - fSlow250))};
																																																								iTemp199 = ((fTemp198 < fTemp194) as i32) * if iTemp195 != 0 {-iSlow249} else {iSlow249};
																																																								fTemp200 = F32::floor(self.fRec35[0]);
																																																								fTemp201 = self.fRec35[0] - fTemp200;
																																																								fTemp202 = if iSlow244 != 0 {if iSlow245 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp201 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec35[0] + (0.5 - (fTemp200 + F32::floor(self.fRec35[0] + (0.5 - fTemp200))))) + -1.0}} else {if iSlow247 != 0 {1.0 - 4.0 * F32::abs(self.fRec35[0] + (-0.25 - (fTemp200 + F32::floor(self.fRec35[0] + (0.25 - fTemp200)))))} else {F32::sin(6.2831855 * fTemp201)}};
																																																								self.fVec30[0] = fTemp202;
																																																								fTemp203 = self.fConst2 * fTemp24 * (4.0 * self.fRec23[0] * fTemp190 * (0.5 * (fTemp202 + self.fVec30[1] - (i32::wrapping_add(iTemp199, iTemp197)) as F32) + (fTemp198 * (iTemp199) as F32 + fTemp196 * (iTemp197) as F32) / fTemp194) + 1.0);
																																																								iTemp204 = (fTemp203 < 0.0) as i32;
																																																								fTemp205 = 0.5 * (if iTemp204 != 0 {-iSlow251} else {iSlow251}) as F32;
																																																								fTemp206 = F32::max(F32::abs(fTemp203), 1e-09);
																																																								fTemp207 = self.fRec37[1] + fTemp203;
																																																								self.fRec37[0] = if iTemp0 != 0 {fTemp30} else {fTemp207 - F32::floor(fTemp207)};
																																																								fTemp208 = if iTemp204 != 0 {fSlow252 - (self.fRec37[0] + F32::floor(fSlow252 - self.fRec37[0]))} else {self.fRec37[0] - (fSlow252 + F32::floor(self.fRec37[0] - fSlow252))};
																																																								fTemp209 = F32::max(0.0, F32::min(1.0, fTemp208 / fTemp206));
																																																								fTemp210 = 1.0 - fTemp209;
																																																								fTemp211 = fSlow253 * fTemp206;
																																																								fTemp212 = ((fTemp208 < fTemp206) as i32) as u32 as F32;
																																																								fTemp213 = 0.5 * (if iTemp204 != 0 {-iSlow254} else {iSlow254}) as F32;
																																																								fTemp214 = if iTemp204 != 0 {fSlow255 - (self.fRec37[0] + F32::floor(fSlow255 - self.fRec37[0]))} else {self.fRec37[0] - (fSlow255 + F32::floor(self.fRec37[0] - fSlow255))};
																																																								fTemp215 = F32::max(0.0, F32::min(1.0, fTemp214 / fTemp206));
																																																								fTemp216 = 1.0 - fTemp215;
																																																								fTemp217 = fSlow256 * fTemp206;
																																																								fTemp218 = ((fTemp214 < fTemp206) as i32) as u32 as F32;
																																																								self.fVec31[0] = fTemp218 * OscDsp_faustpower2_f(fTemp216) * (0.16666667 * fTemp217 * fTemp216 - fTemp213) + fTemp212 * OscDsp_faustpower2_f(fTemp210) * (0.16666667 * fTemp211 * fTemp210 - fTemp205);
																																																								fTemp219 = F32::floor(self.fRec37[0]);
																																																								fTemp220 = self.fRec37[0] - fTemp219;
																																																								self.fVec32[0] = if iSlow234 != 0 {if iSlow236 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp220 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec37[0] + (0.5 - (fTemp219 + F32::floor(self.fRec37[0] + (0.5 - fTemp219))))) + -1.0}} else {if iSlow235 != 0 {1.0 - 4.0 * F32::abs(self.fRec37[0] + (-0.25 - (fTemp219 + F32::floor(self.fRec37[0] + (0.25 - fTemp219)))))} else {F32::sin(6.2831855 * fTemp220)}};
																																																								fTemp221 = fSlow223 * (fTemp212 * OscDsp_faustpower2_f(fTemp209) * (fTemp205 + 0.16666667 * fTemp211 * fTemp209) + self.fVec32[1] + fTemp218 * OscDsp_faustpower2_f(fTemp215) * (fTemp213 + 0.16666667 * fTemp217 * fTemp215) + self.fVec31[1]);
																																																							}
																																																								if iSlow2 & (iSlow6 & iSlow34) != 0 {
																																																									iTemp222 = (fTemp25 < 0.0) as i32;
																																																									fTemp223 = 0.5 * (if iTemp222 != 0 {-iSlow251} else {iSlow251}) as F32;
																																																									fTemp224 = F32::max(fTemp70, 1e-09);
																																																									fTemp225 = if iTemp222 != 0 {fSlow252 - (self.fRec0[0] + F32::floor(fSlow252 - self.fRec0[0]))} else {self.fRec0[0] - (fSlow252 + F32::floor(self.fRec0[0] - fSlow252))};
																																																									fTemp226 = F32::max(0.0, F32::min(1.0, fTemp225 / fTemp224));
																																																									fTemp227 = 1.0 - fTemp226;
																																																									fTemp228 = fSlow253 * fTemp224;
																																																									fTemp229 = ((fTemp225 < fTemp224) as i32) as u32 as F32;
																																																									fTemp230 = 0.5 * (if iTemp222 != 0 {-iSlow254} else {iSlow254}) as F32;
																																																									fTemp231 = if iTemp222 != 0 {fSlow255 - (self.fRec0[0] + F32::floor(fSlow255 - self.fRec0[0]))} else {self.fRec0[0] - (fSlow255 + F32::floor(self.fRec0[0] - fSlow255))};
																																																									fTemp232 = F32::max(0.0, F32::min(1.0, fTemp231 / fTemp224));
																																																									fTemp233 = 1.0 - fTemp232;
																																																									fTemp234 = fSlow256 * fTemp224;
																																																									fTemp235 = ((fTemp231 < fTemp224) as i32) as u32 as F32;
																																																									self.fVec33[0] = fTemp235 * OscDsp_faustpower2_f(fTemp233) * (0.16666667 * fTemp234 * fTemp233 - fTemp230) + fTemp229 * OscDsp_faustpower2_f(fTemp227) * (0.16666667 * fTemp228 * fTemp227 - fTemp223);
																																																									fTemp236 = F32::floor(self.fRec0[0]);
																																																									fTemp237 = self.fRec0[0] - fTemp236;
																																																									self.fVec34[0] = if iSlow234 != 0 {if iSlow236 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp237 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec0[0] + (0.5 - (fTemp236 + F32::floor(self.fRec0[0] + (0.5 - fTemp236))))) + -1.0}} else {if iSlow235 != 0 {1.0 - 4.0 * F32::abs(self.fRec0[0] + (-0.25 - (fTemp236 + F32::floor(self.fRec0[0] + (0.25 - fTemp236)))))} else {F32::sin(6.2831855 * fTemp237)}};
																																																									fTemp238 = (9.0 * self.fRec25[0] + 1.0) * (1.5 * self.fRec23[0] + fTemp229 * OscDsp_faustpower2_f(fTemp226) * (fTemp223 + 0.16666667 * fTemp228 * fTemp226) + self.fVec34[1] + fTemp235 * OscDsp_faustpower2_f(fTemp232) * (fTemp230 + 0.16666667 * fTemp234 * fTemp232) + self.fVec33[1]);
																																																									self.fVec35[0] = fTemp238;
																																																									fTemp239 = fTemp238 - self.fVec35[1];
																																																									iTemp240 = (F32::abs(fTemp239) < 0.0001) as i32;
																																																									fTemp241 = if iTemp240 != 0 {F32::sin(0.7853982 * (fTemp238 + self.fVec35[1]))} else {-(0.63661975 * ((F32::cos(1.5707964 * fTemp238) - F32::cos(1.5707964 * self.fVec35[1])) / if iTemp240 != 0 {1.0} else {fTemp239}))};
																																																									self.fVec36[0] = fTemp241;
																																																									self.fRec38[0] = 0.995 * self.fRec38[1] + fTemp241 - self.fVec36[1];
																																																									fTemp242 = fSlow33 * self.fRec38[0];
																																																								}
																																																									if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																										fTemp243 = 1.0 - OscDsp_faustpower2_f(1.0 - fTemp67);
																																																										fTemp244 = 1.0 - 0.5 * OscDsp_faustpower2_f(fTemp67);
																																																										iTemp245 = (self.fRec23[0] > 0.5) as i32;
																																																										fTemp246 = if iTemp245 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * self.fRec23[0])};
																																																										iTemp247 = unsafe { itbl5OscDspSIG5[((F32::max(0.0, F32::min(4.0, 1e+01 * (self.fRec23[0] + -0.5)))) as i32) as usize] };
																																																										fTemp248 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp245, iTemp247))) as F32;
																																																										fTemp249 = self.fConst2 * fTemp24 * F32::exp(0.0005776227 * (fTemp248 + 0.83 * fTemp246));
																																																										fTemp250 = fTemp249 + self.fRec39[1];
																																																									}
																																																										if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow109 & (iSlow149 & iSlow182)) != 0 {
																																																											fTemp251 = F32::floor(fTemp29 + 0.87);
																																																											fTemp252 = fTemp29 + (0.87 - (fTemp251 + F32::floor(fTemp29 + (0.87 - fTemp251))));
																																																										}
																																																											if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																												self.fRec39[0] = if iTemp0 != 0 {fTemp252} else {fTemp250 - F32::floor(fTemp250)};
																																																												fTemp253 = self.fConst2 * fTemp24 * F32::exp(0.0005776227 * (0.36 * fTemp246 + fTemp248));
																																																												fTemp254 = fTemp253 + self.fRec40[1];
																																																											}
																																																												if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow109 & (iSlow149 & iSlow182)) != 0 {
																																																													fTemp255 = F32::floor(fTemp29 + 0.5);
																																																													fTemp256 = fTemp29 + (0.5 - (fTemp255 + F32::floor(fTemp29 + (0.5 - fTemp255))));
																																																												}
																																																													if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																														self.fRec40[0] = if iTemp0 != 0 {fTemp256} else {fTemp254 - F32::floor(fTemp254)};
																																																														fTemp257 = self.fConst2 * fTemp24 * F32::exp(0.0005776227 * ((i32::wrapping_mul(0, iTemp247)) as F32 - fTemp246)) + self.fRec41[1];
																																																													}
																																																														if (iSlow2 & (iSlow6 & iSlow37)) | (iSlow109 & (iSlow149 & iSlow182)) != 0 {
																																																															fTemp258 = F32::floor(fTemp29 + 0.13);
																																																															fTemp259 = fTemp29 + (0.13 - (fTemp258 + F32::floor(fTemp29 + (0.13 - fTemp258))));
																																																														}
																																																															if iSlow2 & (iSlow6 & iSlow37) != 0 {
																																																																self.fRec41[0] = if iTemp0 != 0 {fTemp259} else {fTemp257 - F32::floor(fTemp257)};
																																																																self.fVec37[0] = fTemp140 * fTemp244 + 0.5 * fTemp243 * (wt_basic(iSlow230, self.fConst0 * F32::abs(self.fConst2 * fTemp24 * F32::exp(-0.0005776227 * fTemp246)), self.fConst0, self.fRec41[0]) + wt_basic(iSlow230, self.fConst0 * F32::abs(fTemp253), self.fConst0, self.fRec40[0]) + wt_basic(iSlow230, self.fConst0 * F32::abs(fTemp249), self.fConst0, self.fRec39[0]));
																																																																fTemp260 = fSlow36 * (self.fVec37[1] / F32::sqrt(OscDsp_faustpower2_f(fTemp244) + 0.75 * OscDsp_faustpower2_f(fTemp243)));
																																																															}
																																																																if iSlow2 & iSlow6 != 0 {
																																																																	fTemp261 = fSlow5 * fSlow262 * (fSlow261 * fTemp260 + fSlow260 * fTemp242 + fSlow259 * fTemp221 + fSlow258 * fTemp189 + fSlow257 * fTemp185 + fSlow243 * fTemp153 + fSlow242 * fTemp141 + fSlow241 * fTemp131 + fSlow240 * fTemp108 + fSlow239 * fTemp90 + fSlow232 * fTemp73 + fSlow229 * fTemp61);
																																																																}
																																																																	if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41))))))))))))))) != 0 {
																																																																		self.fVec38[0] = fSlow264;
																																																																		fTemp262 = if ((fSlow264 != self.fVec38[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec42[1] + -1.0};
																																																																		self.fRec42[0] = fTemp262;
																																																																		self.fRec43[0] = if iTemp0 != 0 {fSlow264} else {if (fTemp262 > 0.0) as i32 != 0 {self.fRec43[1] + (fSlow264 - self.fRec43[1]) / F32::max(1.0, fTemp262)} else {fSlow264}};
																																																																		fTemp263 = 3.0 * F32::max(0.0, F32::min(1.0, self.fRec43[0]));
																																																																		iTemp264 = std::cmp::min(2, (fTemp263) as i32);
																																																																		iTemp265 = (iTemp264 >= 2) as i32;
																																																																		iTemp266 = (iTemp264 >= 1) as i32;
																																																																		iTemp267 = i32::wrapping_mul(2176, (if iTemp265 != 0 {fSlow267} else {if iTemp266 != 0 {fSlow266} else {fSlow265}}) as i32);
																																																																	}
																																																																		if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41)))))))))))))) != 0 {
																																																																			fTemp268 = 256.0 * fTemp24;
																																																																			iTemp269 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp268 > self.fConst16) as i32) + ((fTemp268 > self.fConst22) as i32), (fTemp268 > self.fConst21) as i32), (fTemp268 > self.fConst20) as i32), (fTemp268 > self.fConst19) as i32), (fTemp268 > self.fConst18) as i32), (fTemp268 > self.fConst17) as i32);
																																																																			iTemp270 = (iTemp269 >= 4) as i32;
																																																																			iTemp271 = (iTemp269 >= 2) as i32;
																																																																			iTemp272 = (iTemp269 >= 1) as i32;
																																																																			iTemp273 = (iTemp269 >= 3) as i32;
																																																																			iTemp274 = if iTemp270 != 0 {if (iTemp269 >= 6) as i32 != 0 {if (iTemp269 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp269 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp271 != 0 {if iTemp273 != 0 {1792} else {1536}} else {if iTemp272 != 0 {1024} else {0}}};
																																																																			iTemp275 = if iTemp270 != 0 {64} else {if iTemp271 != 0 {if iTemp273 != 0 {128} else {256}} else {if iTemp272 != 0 {512} else {1024}}};
																																																																			fTemp276 = self.fRec0[0] * (iTemp275) as F32;
																																																																			iTemp277 = (fTemp276) as i32;
																																																																			iTemp278 = (i32::wrapping_add(iTemp277, 1)) % iTemp275;
																																																																			fTemp279 = (iTemp277) as F32;
																																																																			fTemp280 = fTemp276 - fTemp279;
																																																																			fTemp281 = fTemp279 + (1.0 - fTemp276);
																																																																		}
																																																																			if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41))))))))))))))) != 0 {
																																																																				fTemp282 = (iTemp264) as F32;
																																																																				fTemp283 = fTemp263 - fTemp282;
																																																																				iTemp284 = i32::wrapping_mul(2176, (if iTemp265 != 0 {fSlow266} else {if iTemp266 != 0 {fSlow265} else {fSlow268}}) as i32);
																																																																				fTemp285 = fTemp282 + (1.0 - fTemp263);
																																																																			}
																																																																				if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | (iSlow2 & (iSlow44 & iSlow41)))))))))))))) != 0 {
																																																																					fTemp286 = fTemp285 * (fTemp281 * wt_sample(i32::wrapping_add(iTemp274, i32::wrapping_add(iTemp277, iTemp284))) + fTemp280 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp284, iTemp274), iTemp278))) + fTemp283 * (fTemp281 * wt_sample(i32::wrapping_add(iTemp274, i32::wrapping_add(iTemp277, iTemp267))) + fTemp280 * wt_sample(i32::wrapping_add(iTemp278, i32::wrapping_add(iTemp274, iTemp267))));
																																																																				}
																																																																					if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | (iSlow2 & (iSlow44 & iSlow41)))))))))))))) != 0 {
																																																																						self.fVec39[0] = fSlow269;
																																																																						fTemp287 = if ((fSlow269 != self.fVec39[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec44[1] + -1.0};
																																																																						self.fRec44[0] = fTemp287;
																																																																						self.fRec45[0] = if iTemp0 != 0 {fSlow269} else {if (fTemp287 > 0.0) as i32 != 0 {self.fRec45[1] + (fSlow269 - self.fRec45[1]) / F32::max(1.0, fTemp287)} else {fSlow269}};
																																																																						fTemp288 = F32::max(0.0, F32::min(1.0, self.fRec45[0]));
																																																																					}
																																																																						if (iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow50)) | (iSlow2 & (iSlow44 & iSlow41)))) != 0 {
																																																																							fTemp289 = (1.0 - fTemp288) * fTemp286;
																																																																						}
																																																																							if iSlow2 & (iSlow44 & iSlow41) != 0 {
																																																																								fTemp290 = fSlow40 * (fTemp289 + fTemp288 * if (fTemp286 > 0.0) as i32 != 0 {1.0 - fTemp286} else {-1.0 - fTemp286});
																																																																							}
																																																																								if (iSlow2 & (iSlow44 & iSlow74)) | (iSlow2 & (iSlow44 & iSlow50)) != 0 {
																																																																									fTemp291 = 512.0 * fTemp24;
																																																																									iTemp292 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp291 > self.fConst16) as i32) + ((fTemp291 > self.fConst22) as i32), (fTemp291 > self.fConst21) as i32), (fTemp291 > self.fConst20) as i32), (fTemp291 > self.fConst19) as i32), (fTemp291 > self.fConst18) as i32), (fTemp291 > self.fConst17) as i32);
																																																																									iTemp293 = (iTemp292 >= 4) as i32;
																																																																									iTemp294 = (iTemp292 >= 2) as i32;
																																																																									iTemp295 = (iTemp292 >= 1) as i32;
																																																																									iTemp296 = (iTemp292 >= 3) as i32;
																																																																									iTemp297 = if iTemp293 != 0 {64} else {if iTemp294 != 0 {if iTemp296 != 0 {128} else {256}} else {if iTemp295 != 0 {512} else {1024}}};
																																																																									fTemp298 = (iTemp297) as F32;
																																																																								}
																																																																									if iSlow2 & (iSlow44 & iSlow50) != 0 {
																																																																										fTemp299 = fTemp298 * if iTemp31 != 0 {fTemp38} else {2.0 * fTemp126};
																																																																										iTemp300 = (fTemp299) as i32;
																																																																										iTemp301 = (i32::wrapping_add(iTemp300, 1)) % iTemp297;
																																																																									}
																																																																										if (iSlow2 & (iSlow44 & iSlow74)) | (iSlow2 & (iSlow44 & iSlow50)) != 0 {
																																																																											iTemp302 = if iTemp293 != 0 {if (iTemp292 >= 6) as i32 != 0 {if (iTemp292 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp292 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp294 != 0 {if iTemp296 != 0 {1792} else {1536}} else {if iTemp295 != 0 {1024} else {0}}};
																																																																											iTemp303 = i32::wrapping_add(iTemp267, iTemp302);
																																																																										}
																																																																											if iSlow2 & (iSlow44 & iSlow50) != 0 {
																																																																												fTemp304 = (iTemp300) as F32;
																																																																												fTemp305 = fTemp299 - fTemp304;
																																																																												fTemp306 = fTemp304 + (1.0 - fTemp299);
																																																																											}
																																																																												if (iSlow2 & (iSlow44 & iSlow74)) | (iSlow2 & (iSlow44 & iSlow50)) != 0 {
																																																																													iTemp307 = i32::wrapping_add(iTemp284, iTemp302);
																																																																												}
																																																																													if iSlow2 & (iSlow44 & iSlow50) != 0 {
																																																																														fTemp308 = fSlow49 * (fTemp289 + fTemp288 * (fTemp285 * (fTemp306 * wt_sample(i32::wrapping_add(iTemp307, iTemp300)) + fTemp305 * wt_sample(i32::wrapping_add(iTemp307, iTemp301))) + fTemp283 * (fTemp306 * wt_sample(i32::wrapping_add(iTemp303, iTemp300)) + fTemp305 * wt_sample(i32::wrapping_add(iTemp303, iTemp301)))));
																																																																													}
																																																																														if iSlow2 & (iSlow44 & iSlow53) != 0 {
																																																																															fTemp309 = fSlow52 * fTemp286 * (1.0 - fTemp288 * (1.0 - F32::sin(6.2831855 * self.fRec0[0] * (7.0 * fTemp288 + 1.0))));
																																																																														}
																																																																															if iSlow2 & (iSlow44 & iSlow56) != 0 {
																																																																																fTemp310 = 2.0 * fTemp288 + 1.0;
																																																																																fTemp311 = 256.0 * fTemp24 * fTemp310;
																																																																																iTemp312 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp311 > self.fConst16) as i32) + ((fTemp311 > self.fConst22) as i32), (fTemp311 > self.fConst21) as i32), (fTemp311 > self.fConst20) as i32), (fTemp311 > self.fConst19) as i32), (fTemp311 > self.fConst18) as i32), (fTemp311 > self.fConst17) as i32);
																																																																																iTemp313 = (iTemp312 >= 4) as i32;
																																																																																iTemp314 = (iTemp312 >= 2) as i32;
																																																																																iTemp315 = (iTemp312 >= 1) as i32;
																																																																																iTemp316 = (iTemp312 >= 3) as i32;
																																																																																iTemp317 = if iTemp313 != 0 {if (iTemp312 >= 6) as i32 != 0 {if (iTemp312 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp312 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp314 != 0 {if iTemp316 != 0 {1792} else {1536}} else {if iTemp315 != 0 {1024} else {0}}};
																																																																																iTemp318 = if iTemp313 != 0 {64} else {if iTemp314 != 0 {if iTemp316 != 0 {128} else {256}} else {if iTemp315 != 0 {512} else {1024}}};
																																																																																fTemp319 = F32::powf(self.fRec0[0], fTemp310) * (iTemp318) as F32;
																																																																																iTemp320 = (fTemp319) as i32;
																																																																																iTemp321 = (i32::wrapping_add(iTemp320, 1)) % iTemp318;
																																																																																fTemp322 = (iTemp320) as F32;
																																																																																fTemp323 = fTemp319 - fTemp322;
																																																																																fTemp324 = fTemp322 + (1.0 - fTemp319);
																																																																																fTemp325 = fSlow55 * (fTemp285 * (fTemp324 * wt_sample(i32::wrapping_add(iTemp317, i32::wrapping_add(iTemp284, iTemp320))) + fTemp323 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp284, iTemp317), iTemp321))) + fTemp283 * (fTemp324 * wt_sample(i32::wrapping_add(iTemp317, i32::wrapping_add(iTemp267, iTemp320))) + fTemp323 * wt_sample(i32::wrapping_add(iTemp321, i32::wrapping_add(iTemp267, iTemp317)))));
																																																																															}
																																																																																if iSlow2 & (iSlow44 & iSlow59) != 0 {
																																																																																	fTemp326 = fSlow58 * (fTemp289 + fTemp288 * F32::sin(7.8539815 * fTemp286));
																																																																																}
																																																																																	if iSlow2 & (iSlow44 & iSlow65) != 0 {
																																																																																		fTemp327 = 0.25 * (fTemp286 * (4.0 * fTemp288 + 1.0) + 1.0);
																																																																																		fTemp328 = fSlow64 * (1.0 - 4.0 * F32::abs(fTemp327 + (-0.5 - F32::floor(fTemp327))));
																																																																																	}
																																																																																		if iSlow2 & (iSlow44 & iSlow74) != 0 {
																																																																																			fTemp329 = 2.0 * self.fRec0[0] * fTemp298;
																																																																																			iTemp330 = (fTemp329) as i32;
																																																																																			iTemp331 = (i32::wrapping_add(iTemp330, 1)) % iTemp297;
																																																																																			fTemp332 = (iTemp330) as F32;
																																																																																			fTemp333 = fTemp329 - fTemp332;
																																																																																			fTemp334 = fTemp332 + (1.0 - fTemp329);
																																																																																			fTemp335 = fSlow73 * (fTemp289 + fTemp288 * (iTemp31) as F32 * F32::abs(fTemp285 * (fTemp334 * wt_sample(i32::wrapping_add(iTemp302, i32::wrapping_add(iTemp284, iTemp330))) + fTemp333 * wt_sample(i32::wrapping_add(iTemp307, iTemp331))) + fTemp283 * (fTemp334 * wt_sample(i32::wrapping_add(iTemp302, i32::wrapping_add(iTemp267, iTemp330))) + fTemp333 * wt_sample(i32::wrapping_add(iTemp331, iTemp303)))));
																																																																																		}
																																																																																			if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow68)) | (iSlow2 & (iSlow44 & iSlow62))) != 0 {
																																																																																				fTemp336 = F32::tan(self.fConst25 * F32::powf(fSlow278, fTemp288));
																																																																																				fTemp337 = 1.0 / fTemp336;
																																																																																				fTemp338 = (fTemp337 + 1.4142135) / fTemp336 + 1.0;
																																																																																				self.fRec46[0] = fTemp286 - (self.fRec46[2] * ((fTemp337 + -1.4142135) / fTemp336 + 1.0) + 2.0 * self.fRec46[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp336))) / fTemp338;
																																																																																				fTemp339 = (self.fRec46[2] + self.fRec46[0] + 2.0 * self.fRec46[1]) / fTemp338;
																																																																																			}
																																																																																				if iSlow2 & (iSlow44 & iSlow68) != 0 {
																																																																																					fTemp340 = fSlow67 * fTemp339;
																																																																																				}
																																																																																					if iSlow2 & (iSlow44 & iSlow62) != 0 {
																																																																																						fTemp341 = fSlow61 * fTemp339;
																																																																																					}
																																																																																						if iSlow2 & (iSlow44 & iSlow86) != 0 {
																																																																																							fTemp342 = fSlow85 * fTemp339;
																																																																																						}
																																																																																							if (iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow77)) | (iSlow2 & (iSlow44 & iSlow71))) != 0 {
																																																																																								fTemp343 = if iTemp9 != 0 {0.0} else {self.fRec48[1] + self.fConst2 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow283, fTemp288)), 0.0)};
																																																																																								self.fRec48[0] = fTemp343 - F32::floor(fTemp343);
																																																																																								self.fRec47[0] = if (self.fRec48[0] < self.fRec48[1]) as i32 != 0 {fTemp286} else {self.fRec47[1]};
																																																																																							}
																																																																																								if iSlow2 & (iSlow44 & iSlow71) != 0 {
																																																																																									fTemp344 = fSlow70 * self.fRec47[0];
																																																																																								}
																																																																																									if iSlow2 & (iSlow44 & iSlow77) != 0 {
																																																																																										fTemp345 = fSlow76 * self.fRec47[0];
																																																																																									}
																																																																																										if iSlow2 & (iSlow44 & iSlow83) != 0 {
																																																																																											fTemp346 = fSlow82 * self.fRec47[0];
																																																																																										}
																																																																																											if iSlow2 & (iSlow44 & iSlow80) != 0 {
																																																																																												fTemp347 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp288)) + -1.0));
																																																																																												fTemp348 = fSlow79 * (F32::floor(fTemp286 * fTemp347 + 0.5) / fTemp347);
																																																																																											}
																																																																																												if iSlow2 & (iSlow44 & iSlow47) != 0 {
																																																																																													fTemp349 = fSlow46 * fTemp286;
																																																																																												}
																																																																																													if iSlow2 & iSlow44 != 0 {
																																																																																														fTemp350 = fSlow43 * (fSlow288 * fTemp349 + fSlow287 * fTemp348 + fSlow286 * fTemp346 + fSlow285 * fTemp345 + fSlow284 * fTemp344 + fSlow281 * fTemp342 + fSlow280 * fTemp341 + fSlow279 * fTemp340 + fSlow276 * fTemp335 + fSlow275 * fTemp328 + fSlow274 * fTemp326 + fSlow273 * fTemp325 + fSlow272 * fTemp309 + fSlow271 * fTemp308 + fSlow270 * fTemp290);
																																																																																													}
																																																																																														if iSlow2 != 0 {
																																																																																															fTemp351 = fSlow1 * (fSlow289 * fTemp350 + fSlow263 * fTemp261);
																																																																																														}
																																																																																															*output0 = fSlow292 * fSlow291 * fTemp351;
																																																																																															self.fVec40[0] = fSlow293;
																																																																																															let mut fTemp352: F32 = if ((fSlow293 != self.fVec40[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec50[1] + -1.0};
																																																																																															self.fRec50[0] = fTemp352;
																																																																																															self.fRec51[0] = if iTemp0 != 0 {fSlow293} else {if (fTemp352 > 0.0) as i32 != 0 {self.fRec51[1] + (fSlow293 - self.fRec51[1]) / F32::max(1.0, fTemp352)} else {fSlow293}};
																																																																																															let mut fTemp353: F32 = F32::abs(self.fRec51[0]);
																																																																																															let mut fTemp354: F32 = F32::exp(0.057762265 * (((self.fRec51[0] > 0.0) as i32) - ((self.fRec51[0] < 0.0) as i32)) as F32 * if (fTemp353 < 0.4) as i32 != 0 {F32::min(0.99, 2.5 * fTemp353)} else {if (fTemp353 < 0.82) as i32 != 0 {F32::min(6.0, F32::floor(16.666666 * (fTemp353 + -0.4))) + 1.0} else {(if (fTemp353 < 0.91) as i32 != 0 {12} else {24}) as F32}});
																																																																																															let mut fTemp355: F32 = fTemp23 * fTemp354;
																																																																																															if (iSlow109 & (iSlow106 & iSlow218)) | ((iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow194)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | ((iSlow109 & (iSlow106 & iSlow185)) | ((iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow179)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | ((iSlow109 & (iSlow149 & iSlow155)) | ((iSlow109 & (iSlow149 & iSlow152)) | (iSlow109 & (iSlow149 & iSlow146))))))))))))))))))))))))) != 0 {
																																																																																																fTemp356 = self.fConst2 * fTemp355;
																																																																																																fTemp357 = fTemp356 + self.fRec49[1];
																																																																																																self.fRec49[0] = if iTemp0 != 0 {fTemp30} else {fTemp357 - F32::floor(fTemp357)};
																																																																																															}
																																																																																																if (iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow149 & iSlow146))) != 0 {
																																																																																																	iTemp358 = (self.fRec49[0] < 0.5) as i32;
																																																																																																}
																																																																																																	if (iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow179)) | ((iSlow109 & (iSlow149 & iSlow176)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow170)) | ((iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | ((iSlow109 & (iSlow149 & iSlow155)) | ((iSlow109 & (iSlow149 & iSlow152)) | (iSlow109 & (iSlow149 & iSlow146)))))))))))) != 0 {
																																																																																																		self.fVec41[0] = fSlow294;
																																																																																																		fTemp359 = if ((fSlow294 != self.fVec41[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec52[1] + -1.0};
																																																																																																		self.fRec52[0] = fTemp359;
																																																																																																		self.fRec53[0] = if iTemp0 != 0 {fSlow294} else {if (fTemp359 > 0.0) as i32 != 0 {self.fRec53[1] + (fSlow294 - self.fRec53[1]) / F32::max(1.0, fTemp359)} else {fSlow294}};
																																																																																																	}
																																																																																																		if (iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow155)) | (iSlow109 & (iSlow149 & iSlow146)))) != 0 {
																																																																																																			fTemp360 = 1.0 - self.fRec53[0];
																																																																																																		}
																																																																																																			if (iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow179)) | ((iSlow109 & (iSlow149 & iSlow176)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow170)) | ((iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | ((iSlow109 & (iSlow149 & iSlow155)) | ((iSlow109 & (iSlow149 & iSlow152)) | (iSlow109 & (iSlow149 & iSlow146)))))))))))) != 0 {
																																																																																																				self.fVec42[0] = fSlow295;
																																																																																																				fTemp361 = if ((fSlow295 != self.fVec42[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec54[1] + -1.0};
																																																																																																				self.fRec54[0] = fTemp361;
																																																																																																				self.fRec55[0] = if iTemp0 != 0 {fSlow295} else {if (fTemp361 > 0.0) as i32 != 0 {self.fRec55[1] + (fSlow295 - self.fRec55[1]) / F32::max(1.0, fTemp361)} else {fSlow295}};
																																																																																																			}
																																																																																																				if iSlow109 & (iSlow149 & iSlow146) != 0 {
																																																																																																					fTemp362 = 1.0 - self.fRec55[0];
																																																																																																					fTemp363 = if iTemp358 != 0 {fTemp362} else {fTemp360};
																																																																																																					fTemp364 = if (fTemp363 < 0.999) as i32 != 0 {F32::max(0.01, fTemp363)} else {1.0};
																																																																																																				}
																																																																																																					if (iSlow109 & (iSlow106 & iSlow188)) | ((iSlow109 & (iSlow149 & iSlow161)) | (iSlow109 & (iSlow149 & iSlow146))) != 0 {
																																																																																																						fTemp365 = 2.0 * self.fRec49[0];
																																																																																																					}
																																																																																																						if iSlow109 & (iSlow149 & iSlow146) != 0 {
																																																																																																							fTemp366 = F32::floor(fTemp365);
																																																																																																							fTemp367 = fTemp365 - (fTemp364 + fTemp366);
																																																																																																							fTemp368 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp367 / fTemp355)));
																																																																																																							fTemp369 = if iTemp358 != 0 {fSlow297} else {fSlow296};
																																																																																																							fTemp370 = if (fTemp369 >= 2.0) as i32 != 0 {(if (fTemp369 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp369 >= 1.0) as i32 != 0 {4.0} else {6.2831855}};
																																																																																																							fTemp371 = fTemp22 * fTemp370 * fTemp21 * fTemp5 * fTemp354;
																																																																																																							iTemp372 = (fTemp369) as i32;
																																																																																																							fTemp373 = self.fConst0 * F32::abs(self.fConst11 * (fTemp355 / fTemp364));
																																																																																																							fTemp374 = 0.5 * wt_basic(iTemp372, fTemp373, self.fConst0, 0.999999);
																																																																																																							fTemp375 = self.fConst11 * fTemp355;
																																																																																																							fTemp376 = fTemp365 - fTemp366;
																																																																																																							fTemp377 = ((((fTemp364 < 1.0) as i32) & ((fTemp376 >= fTemp364) as i32)) & ((fTemp367 < fTemp375) as i32)) as F32;
																																																																																																							fTemp378 = 1.0 - fTemp368;
																																																																																																							fTemp379 = if iTemp358 != 0 {fSlow296} else {fSlow297};
																																																																																																							fTemp380 = if iTemp358 != 0 {fTemp360} else {fTemp362};
																																																																																																							fTemp381 = if (fTemp380 < 0.999) as i32 != 0 {F32::max(0.01, fTemp380)} else {1.0};
																																																																																																							fTemp382 = ((fTemp381 == 1.0) as i32) as u32 as F32;
																																																																																																							fTemp383 = 0.5 * (wt_basic(iTemp372, fTemp373, self.fConst0, 0.0) - fTemp382 * wt_basic((fTemp379) as i32, self.fConst0 * F32::abs(self.fConst11 * (fTemp355 / fTemp381)), self.fConst0, 0.999999));
																																																																																																							fTemp384 = F32::max(0.0, F32::min(1.0, self.fConst9 * (fTemp376 / fTemp355)));
																																																																																																							fTemp385 = 1.0 - fTemp384;
																																																																																																							fTemp386 = fTemp370 / fTemp364 - fTemp382 * if (fTemp379 >= 2.0) as i32 != 0 {(if (fTemp379 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp379 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} / fTemp381;
																																																																																																							fTemp387 = ((fTemp376 < fTemp375) as i32) as u32 as F32;
																																																																																																							self.fVec43[0] = fTemp387 * OscDsp_faustpower2_f(fTemp385) * (self.fConst10 * fTemp355 * fTemp386 * fTemp385 - fTemp383) + fTemp377 * OscDsp_faustpower2_f(fTemp378) * (fTemp374 - self.fConst10 * (fTemp371 * fTemp378 / fTemp364));
																																																																																																							self.fVec44[0] = ((fTemp376 < fTemp364) as i32) as u32 as F32 * wt_basic(iTemp372, fTemp373, self.fConst0, fTemp376 / fTemp364);
																																																																																																							fTemp388 = fSlow145 * (self.fVec44[1] + fTemp387 * OscDsp_faustpower2_f(fTemp384) * (fTemp383 + self.fConst10 * fTemp355 * fTemp384 * fTemp386) + self.fVec43[1] - fTemp377 * OscDsp_faustpower2_f(fTemp368) * (fTemp374 + self.fConst10 * (fTemp371 * fTemp368 / fTemp364)));
																																																																																																						}
																																																																																																							if iSlow109 & (iSlow149 & iSlow152) != 0 {
																																																																																																								fTemp389 = self.fConst12 * fTemp355 * F32::exp(0.0046209814 * self.fRec53[0]);
																																																																																																								fTemp390 = fTemp389 + self.fRec56[1];
																																																																																																								self.fRec56[0] = if iTemp0 != 0 {fTemp66} else {fTemp390 - F32::floor(fTemp390)};
																																																																																																							}
																																																																																																								if (iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow176)) | ((iSlow109 & (iSlow149 & iSlow164)) | (iSlow109 & (iSlow149 & iSlow152)))) != 0 {
																																																																																																									fTemp391 = F32::max(0.0, F32::min(1.0, self.fRec55[0]));
																																																																																																								}
																																																																																																									if iSlow109 & (iSlow149 & iSlow152) != 0 {
																																																																																																										fTemp392 = 4.0 * fTemp391;
																																																																																																										fTemp393 = F32::min(3.0, fTemp392);
																																																																																																									}
																																																																																																										if (iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow179)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | (iSlow109 & (iSlow149 & iSlow152)))))))) != 0 {
																																																																																																											fTemp394 = F32::abs(fTemp356);
																																																																																																										}
																																																																																																											if (iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | (iSlow109 & (iSlow149 & iSlow152))))))) != 0 {
																																																																																																												fTemp395 = self.fConst0 * fTemp394;
																																																																																																											}
																																																																																																												if iSlow109 & (iSlow149 & iSlow152) != 0 {
																																																																																																													fTemp396 = wt_basic(2, fTemp395, self.fConst0, self.fRec49[0]);
																																																																																																													self.fVec45[0] = wt_basic(0, fTemp395, self.fConst0, self.fRec49[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp393)) + wt_basic(1, fTemp395, self.fConst0, self.fRec49[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp393 + -1.0)) + fTemp396 * F32::max(0.0, 1.0 - F32::abs(fTemp393 + -2.0)) + (fTemp396 - wt_basic(2, fTemp395, self.fConst0, self.fRec49[0] + (-0.5 - 0.42 * F32::max(0.0, fTemp392 + -3.0)))) * F32::max(0.0, 1.0 - F32::abs(fTemp393 + -3.0)) + self.fRec53[0] * wt_basic(iSlow299, self.fConst0 * F32::abs(fTemp389), self.fConst0, self.fRec56[0]);
																																																																																																													fTemp397 = fSlow151 * (self.fVec45[1] / F32::sqrt(OscDsp_faustpower2_f(self.fRec53[0]) + 1.0));
																																																																																																												}
																																																																																																													if (iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | (iSlow109 & (iSlow149 & iSlow155))) != 0 {
																																																																																																														iTemp398 = (self.fRec49[0] < self.fRec49[1]) as i32;
																																																																																																													}
																																																																																																														if (iSlow109 & (iSlow149 & iSlow158)) | (iSlow109 & (iSlow149 & iSlow155)) != 0 {
																																																																																																															self.iRec57[0] = (i32::wrapping_add(iTemp398, self.iRec57[1])) % 2;
																																																																																																														}
																																																																																																															if iSlow109 & (iSlow149 & iSlow155) != 0 {
																																																																																																																self.fVec46[0] = wt_basic(iSlow299, self.fConst0 * F32::abs(self.fConst12 * fTemp355), self.fConst0, 0.5 * (self.fRec49[0] + (self.iRec57[0]) as F32));
																																																																																																															}
																																																																																																																if (iSlow109 & (iSlow149 & iSlow161)) | (iSlow109 & (iSlow149 & iSlow155)) != 0 {
																																																																																																																	fTemp399 = 15.0 * self.fRec55[0];
																																																																																																																	fTemp400 = fTemp399 + 1.0;
																																																																																																																	fTemp401 = fTemp355 * fTemp400;
																																																																																																																	fTemp402 = self.fConst0 * F32::abs(self.fConst2 * fTemp401);
																																																																																																																	fTemp403 = wt_basic(iSlow301, fTemp402, self.fConst0, fTemp400);
																																																																																																																	fTemp404 = wt_basic(iSlow301, fTemp402, self.fConst0, 0.0);
																																																																																																																}
																																																																																																																	if iSlow109 & (iSlow149 & iSlow155) != 0 {
																																																																																																																		fTemp405 = 0.5 * (fTemp404 - fTemp403);
																																																																																																																	}
																																																																																																																		if (iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | (iSlow109 & (iSlow149 & iSlow155))) != 0 {
																																																																																																																			fTemp406 = F32::max(0.0, F32::min(1.0, self.fConst0 * (self.fRec49[0] / fTemp355)));
																																																																																																																			fTemp407 = 1.0 - fTemp406;
																																																																																																																		}
																																																																																																																			if iSlow109 & (iSlow149 & iSlow155) != 0 {
																																																																																																																				fTemp408 = fTemp399 + (1.0 - F32::floor(fTemp400));
																																																																																																																				fTemp409 = fSlow306 - if iSlow302 != 0 {fSlow305} else {if iSlow303 != 0 {(i32::wrapping_sub(4, i32::wrapping_mul(8, ((fTemp408 >= 0.25) as i32) & ((fTemp408 < 0.75) as i32)))) as F32} else {6.2831855 * F32::cos(6.2831855 * fTemp408)}};
																																																																																																																			}
																																																																																																																				if (iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | (iSlow109 & (iSlow149 & iSlow155))) != 0 {
																																																																																																																					fTemp410 = (iTemp398) as F32;
																																																																																																																					fTemp411 = fTemp410 * OscDsp_faustpower2_f(fTemp407);
																																																																																																																				}
																																																																																																																					if iSlow109 & (iSlow149 & iSlow155) != 0 {
																																																																																																																						self.fVec47[0] = fTemp411 * (self.fConst13 * fTemp401 * fTemp409 * fTemp407 - fTemp405);
																																																																																																																					}
																																																																																																																						if (iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | (iSlow109 & (iSlow149 & iSlow155))) != 0 {
																																																																																																																							fTemp412 = fTemp410 * OscDsp_faustpower2_f(fTemp406);
																																																																																																																						}
																																																																																																																							if (iSlow109 & (iSlow149 & iSlow161)) | (iSlow109 & (iSlow149 & iSlow155)) != 0 {
																																																																																																																								fTemp413 = wt_basic(iSlow301, fTemp402, self.fConst0, self.fRec49[0] * fTemp400);
																																																																																																																								self.fVec48[0] = fTemp413;
																																																																																																																							}
																																																																																																																								if iSlow109 & (iSlow149 & iSlow155) != 0 {
																																																																																																																									fTemp414 = fSlow154 * (fTemp360 * (self.fVec48[1] + fTemp412 * (fTemp405 + self.fConst13 * fTemp401 * fTemp406 * fTemp409) + self.fVec47[1]) + self.fRec53[0] * self.fVec46[1]);
																																																																																																																								}
																																																																																																																									if iSlow109 & (iSlow149 & iSlow158) != 0 {
																																																																																																																										fTemp415 = if self.iRec57[0] != 0 {fSlow297} else {fSlow296};
																																																																																																																										fTemp416 = 7.0 * self.fRec55[0] + 1.0;
																																																																																																																										fTemp417 = fTemp416 * (self.fRec53[0] + 1.0);
																																																																																																																										fTemp418 = if self.iRec57[0] != 0 {fTemp416} else {fTemp417};
																																																																																																																										fTemp419 = if self.iRec57[0] != 0 {fSlow296} else {fSlow297};
																																																																																																																										iTemp420 = (fTemp419) as i32;
																																																																																																																										fTemp421 = if self.iRec57[0] != 0 {fTemp417} else {fTemp416};
																																																																																																																										fTemp422 = 0.5 * (fTemp421 * wt_basic(iTemp420, fTemp395, self.fConst0, 0.0) - fTemp418 * wt_basic((fTemp415) as i32, fTemp395, self.fConst0, 0.999999));
																																																																																																																										fTemp423 = if (fTemp419 >= 2.0) as i32 != 0 {(if (fTemp419 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp419 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp421 - if (fTemp415 >= 2.0) as i32 != 0 {(if (fTemp415 >= 3.0) as i32 != 0 {0} else {2}) as F32} else {if (fTemp415 >= 1.0) as i32 != 0 {4.0} else {6.2831855}} * fTemp418;
																																																																																																																										self.fVec49[0] = fTemp411 * (self.fConst13 * fTemp355 * fTemp407 * fTemp423 - fTemp422);
																																																																																																																										self.fVec50[0] = wt_basic(iTemp420, fTemp395, self.fConst0, self.fRec49[0]) * fTemp421;
																																																																																																																										fTemp424 = self.fVec50[1] + fTemp412 * (fTemp422 + self.fConst13 * fTemp355 * fTemp406 * fTemp423) + self.fVec49[1];
																																																																																																																										self.fVec51[0] = fTemp424;
																																																																																																																										fTemp425 = fTemp424 - self.fVec51[1];
																																																																																																																										iTemp426 = (F32::abs(fTemp425) < 0.0001) as i32;
																																																																																																																										fTemp427 = 4.0 * F32::floor(0.25 * (self.fVec51[1] + 1.0));
																																																																																																																										fTemp428 = self.fVec51[1] - fTemp427;
																																																																																																																										fTemp429 = 4.0 * F32::floor(0.25 * (fTemp424 + 1.0));
																																																																																																																										fTemp430 = fTemp424 - fTemp429;
																																																																																																																										fTemp431 = 0.125 * (fTemp424 + self.fVec51[1]);
																																																																																																																										fTemp432 = fSlow157 * if iTemp426 != 0 {1.0 - 4.0 * F32::abs(fTemp431 + (-0.25 - F32::floor(fTemp431 + 0.25)))} else {(if (fTemp430 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp430)} else {1.0 - 0.5 * OscDsp_faustpower2_f(fTemp424 + (-2.0 - fTemp429))} - if (fTemp428 < 1.0) as i32 != 0 {0.5 * OscDsp_faustpower2_f(fTemp428)} else {1.0 - 0.5 * OscDsp_faustpower2_f(self.fVec51[1] + (-2.0 - fTemp427))}) / if iTemp426 != 0 {1.0} else {fTemp425}};
																																																																																																																									}
																																																																																																																										if iSlow109 & (iSlow149 & iSlow161) != 0 {
																																																																																																																											fTemp433 = 4.0 * F32::max(0.0, F32::min(1.0, self.fRec53[0]));
																																																																																																																											iTemp434 = (F32::min(3.0, fTemp433)) as i32;
																																																																																																																											iTemp435 = i32::wrapping_add(iTemp434, 1);
																																																																																																																											iTemp436 = (iTemp435 >= 3) as i32;
																																																																																																																											iTemp437 = (iTemp435 >= 2) as i32;
																																																																																																																											iTemp438 = (iTemp435 >= 1) as i32;
																																																																																																																											iTemp439 = (iTemp435 >= 4) as i32;
																																																																																																																											fTemp440 = 0.5 * (wt_basic(iSlow299, fTemp395, self.fConst0, 0.999999) + 1.0);
																																																																																																																											fTemp441 = (iTemp434) as F32;
																																																																																																																											fTemp442 = fTemp433 - fTemp441;
																																																																																																																											iTemp443 = (iTemp434 >= 3) as i32;
																																																																																																																											iTemp444 = (iTemp434 >= 2) as i32;
																																																																																																																											iTemp445 = (iTemp434 >= 1) as i32;
																																																																																																																											iTemp446 = (iTemp434 >= 4) as i32;
																																																																																																																											fTemp447 = fTemp441 + (1.0 - fTemp433);
																																																																																																																											fTemp448 = 0.5 * (wt_basic(iSlow299, fTemp395, self.fConst0, 0.0) + 1.0);
																																																																																																																											fTemp449 = fTemp404 * (fTemp447 * if iTemp443 != 0 {if iTemp446 != 0 {fTemp448} else {0.0}} else {if iTemp444 != 0 {0.0} else {(if iTemp445 != 0 {0} else {1}) as F32}} + fTemp442 * if iTemp436 != 0 {if iTemp439 != 0 {fTemp448} else {0.0}} else {if iTemp437 != 0 {0.0} else {(if iTemp438 != 0 {0} else {1}) as F32}}) - fTemp403 * (fTemp447 * if iTemp443 != 0 {if iTemp446 != 0 {fTemp440} else {6e-06}} else {if iTemp444 != 0 {0.0} else {if iTemp445 != 0 {2e-06} else {1e-06}}} + fTemp442 * if iTemp436 != 0 {if iTemp439 != 0 {fTemp440} else {6e-06}} else {if iTemp437 != 0 {0.0} else {if iTemp438 != 0 {2e-06} else {1e-06}}});
																																																																																																																											self.fVec52[0] = fTemp411 * fTemp449;
																																																																																																																										}
																																																																																																																											if (iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow149 & iSlow161)) != 0 {
																																																																																																																												fTemp450 = 1.0 - self.fRec49[0];
																																																																																																																											}
																																																																																																																												if iSlow109 & (iSlow149 & iSlow161) != 0 {
																																																																																																																													fTemp451 = 1.0 - F32::abs(fTemp365 + -1.0);
																																																																																																																													fTemp452 = F32::sin(3.1415927 * self.fRec49[0]);
																																																																																																																													fTemp453 = F32::min(1.0, 6.0 * F32::min(self.fRec49[0], fTemp450));
																																																																																																																													fTemp454 = 0.5 * (wt_basic(iSlow299, fTemp395, self.fConst0, self.fRec49[0]) + 1.0);
																																																																																																																													self.fVec53[0] = fTemp413 * (fTemp447 * if iTemp443 != 0 {if iTemp446 != 0 {fTemp454} else {fTemp453}} else {if iTemp444 != 0 {fTemp452} else {if iTemp445 != 0 {fTemp451} else {fTemp450}}} + fTemp442 * if iTemp436 != 0 {if iTemp439 != 0 {fTemp454} else {fTemp453}} else {if iTemp437 != 0 {fTemp452} else {if iTemp438 != 0 {fTemp451} else {fTemp450}}});
																																																																																																																													fTemp455 = fSlow160 * (self.fVec53[1] + 0.5 * (fTemp412 * fTemp449 - self.fVec52[1]));
																																																																																																																												}
																																																																																																																													if iSlow109 & (iSlow149 & iSlow164) != 0 {
																																																																																																																														fTemp456 = self.fConst2 * fTemp355 * (2.17 * fTemp391 + 3.0);
																																																																																																																														fTemp457 = fTemp456 + self.fRec58[1];
																																																																																																																														self.fRec58[0] = if iTemp0 != 0 {fTemp135} else {fTemp457 - F32::floor(fTemp457)};
																																																																																																																														self.fVec54[0] = wt_basic(iSlow299, self.fConst0 * F32::abs(fTemp456), self.fConst0, self.fRec58[0]);
																																																																																																																														fTemp458 = self.fConst2 * fTemp355 * (1.41 * fTemp391 + 2.0);
																																																																																																																														fTemp459 = fTemp458 + self.fRec59[1];
																																																																																																																														self.fRec59[0] = if iTemp0 != 0 {fTemp139} else {fTemp459 - F32::floor(fTemp459)};
																																																																																																																														self.fVec55[0] = wt_basic(iSlow299, self.fConst0 * F32::abs(fTemp458), self.fConst0, self.fRec59[0]);
																																																																																																																													}
																																																																																																																														if (iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow167)) | (iSlow109 & (iSlow149 & iSlow164)))) != 0 {
																																																																																																																															fTemp460 = wt_basic(iSlow301, fTemp395, self.fConst0, self.fRec49[0]);
																																																																																																																															self.fVec56[0] = fTemp460;
																																																																																																																														}
																																																																																																																															if iSlow109 & (iSlow149 & iSlow164) != 0 {
																																																																																																																																fTemp461 = fSlow163 * (0.57735026 * fTemp360 * (self.fVec56[1] + self.fVec55[1] + self.fVec54[1]) + 1.5 * self.fRec53[0] * self.fVec56[1] * self.fVec55[1] * self.fVec54[1]);
																																																																																																																															}
																																																																																																																																if iSlow109 & (iSlow149 & iSlow167) != 0 {
																																																																																																																																	iTemp462 = (F32::max(0.0, F32::min(1e+01, 10.999 * self.fRec55[0]))) as i32;
																																																																																																																																	fTemp463 = unsafe { ftbl3OscDspSIG3[(i32::wrapping_add(iTemp462, 1)) as usize] };
																																																																																																																																	fTemp464 = self.fConst2 * fTemp355 * fTemp463;
																																																																																																																																	fTemp465 = fTemp464 + self.fRec60[1];
																																																																																																																																	fTemp466 = fSlow142 * fTemp28 * fTemp463;
																																																																																																																																	fTemp467 = F32::floor(fTemp466);
																																																																																																																																	self.fRec60[0] = if iTemp0 != 0 {fTemp466 - (fTemp467 + F32::floor(fTemp466 - fTemp467))} else {fTemp465 - F32::floor(fTemp465)};
																																																																																																																																	self.fVec57[0] = wt_basic(iSlow299, self.fConst0 * F32::abs(fTemp464), self.fConst0, self.fRec60[0]);
																																																																																																																																	fTemp468 = unsafe { ftbl3OscDspSIG3[iTemp462 as usize] };
																																																																																																																																	fTemp469 = self.fConst2 * fTemp355 * fTemp468;
																																																																																																																																	fTemp470 = fTemp469 + self.fRec61[1];
																																																																																																																																	fTemp471 = fSlow142 * fTemp28 * fTemp468;
																																																																																																																																	fTemp472 = F32::floor(fTemp471);
																																																																																																																																	self.fRec61[0] = if iTemp0 != 0 {fTemp471 - (fTemp472 + F32::floor(fTemp471 - fTemp472))} else {fTemp470 - F32::floor(fTemp470)};
																																																																																																																																	self.fVec58[0] = wt_basic(iSlow299, self.fConst0 * F32::abs(fTemp469), self.fConst0, self.fRec61[0]);
																																																																																																																																	fTemp473 = fSlow166 * self.fVec56[1] * (fTemp360 * self.fVec58[1] + self.fRec53[0] * self.fVec57[1]);
																																																																																																																																}
																																																																																																																																	if iSlow109 & (iSlow149 & iSlow170) != 0 {
																																																																																																																																		fTemp474 = unsafe { ftbl4OscDspSIG4[((F32::max(0.0, F32::min(9.0, 9.999 * self.fRec53[0]))) as i32) as usize] };
																																																																																																																																		fTemp475 = self.fConst2 * fTemp355 * fTemp474;
																																																																																																																																		fTemp476 = F32::max(F32::abs(fTemp475), 1e-09);
																																																																																																																																		iTemp477 = (fTemp475 < 0.0) as i32;
																																																																																																																																		fTemp478 = fTemp475 + self.fRec62[1];
																																																																																																																																		fTemp479 = fSlow142 * fTemp28 * fTemp474;
																																																																																																																																		fTemp480 = F32::floor(fTemp479);
																																																																																																																																		self.fRec62[0] = if iTemp0 != 0 {fTemp479 - (fTemp480 + F32::floor(fTemp479 - fTemp480))} else {fTemp478 - F32::floor(fTemp478)};
																																																																																																																																		fTemp481 = if iTemp477 != 0 {fSlow316 - (self.fRec62[0] + F32::floor(fSlow316 - self.fRec62[0]))} else {self.fRec62[0] - (fSlow316 + F32::floor(self.fRec62[0] - fSlow316))};
																																																																																																																																		iTemp482 = ((fTemp481 < fTemp476) as i32) * if iTemp477 != 0 {-iSlow314} else {iSlow314};
																																																																																																																																		fTemp483 = if iTemp477 != 0 {fSlow318 - (self.fRec62[0] + F32::floor(fSlow318 - self.fRec62[0]))} else {self.fRec62[0] - (fSlow318 + F32::floor(self.fRec62[0] - fSlow318))};
																																																																																																																																		iTemp484 = ((fTemp483 < fTemp476) as i32) * if iTemp477 != 0 {-iSlow317} else {iSlow317};
																																																																																																																																		fTemp485 = F32::floor(self.fRec62[0]);
																																																																																																																																		fTemp486 = self.fRec62[0] - fTemp485;
																																																																																																																																		fTemp487 = if iSlow312 != 0 {if iSlow313 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp486 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec62[0] + (0.5 - (fTemp485 + F32::floor(self.fRec62[0] + (0.5 - fTemp485))))) + -1.0}} else {if iSlow315 != 0 {1.0 - 4.0 * F32::abs(self.fRec62[0] + (-0.25 - (fTemp485 + F32::floor(self.fRec62[0] + (0.25 - fTemp485)))))} else {F32::sin(6.2831855 * fTemp486)}};
																																																																																																																																		self.fVec59[0] = fTemp487;
																																																																																																																																	}
																																																																																																																																		if (iSlow109 & (iSlow149 & iSlow173)) | (iSlow109 & (iSlow149 & iSlow170)) != 0 {
																																																																																																																																			fTemp488 = OscDsp_faustpower2_f(self.fRec55[0]);
																																																																																																																																		}
																																																																																																																																			if iSlow109 & (iSlow149 & iSlow170) != 0 {
																																																																																																																																				fTemp489 = self.fConst2 * fTemp355 * (8.0 * fTemp488 * fTemp474 * (0.5 * (fTemp487 + self.fVec59[1] - (i32::wrapping_add(iTemp484, iTemp482)) as F32) + (fTemp483 * (iTemp484) as F32 + fTemp481 * (iTemp482) as F32) / fTemp476) + 1.0);
																																																																																																																																				iTemp490 = (fTemp489 < 0.0) as i32;
																																																																																																																																				fTemp491 = 0.5 * (if iTemp490 != 0 {-iSlow319} else {iSlow319}) as F32;
																																																																																																																																				fTemp492 = F32::max(F32::abs(fTemp489), 1e-09);
																																																																																																																																				fTemp493 = self.fRec63[1] + fTemp489;
																																																																																																																																				self.fRec63[0] = if iTemp0 != 0 {fTemp30} else {fTemp493 - F32::floor(fTemp493)};
																																																																																																																																				fTemp494 = if iTemp490 != 0 {fSlow320 - (self.fRec63[0] + F32::floor(fSlow320 - self.fRec63[0]))} else {self.fRec63[0] - (fSlow320 + F32::floor(self.fRec63[0] - fSlow320))};
																																																																																																																																				fTemp495 = F32::max(0.0, F32::min(1.0, fTemp494 / fTemp492));
																																																																																																																																				fTemp496 = 1.0 - fTemp495;
																																																																																																																																				fTemp497 = ((fTemp494 < fTemp492) as i32) as u32 as F32;
																																																																																																																																				fTemp498 = 0.5 * (if iTemp490 != 0 {-iSlow322} else {iSlow322}) as F32;
																																																																																																																																				fTemp499 = if iTemp490 != 0 {fSlow323 - (self.fRec63[0] + F32::floor(fSlow323 - self.fRec63[0]))} else {self.fRec63[0] - (fSlow323 + F32::floor(self.fRec63[0] - fSlow323))};
																																																																																																																																				fTemp500 = F32::max(0.0, F32::min(1.0, fTemp499 / fTemp492));
																																																																																																																																				fTemp501 = 1.0 - fTemp500;
																																																																																																																																				fTemp502 = ((fTemp499 < fTemp492) as i32) as u32 as F32;
																																																																																																																																				self.fVec60[0] = fTemp502 * OscDsp_faustpower2_f(fTemp501) * (0.16666667 * fTemp492 * fSlow324 * fTemp501 - fTemp498) + fTemp497 * OscDsp_faustpower2_f(fTemp496) * (0.16666667 * fTemp492 * fSlow321 * fTemp496 - fTemp491);
																																																																																																																																				fTemp503 = F32::floor(self.fRec63[0]);
																																																																																																																																				fTemp504 = self.fRec63[0] - fTemp503;
																																																																																																																																				self.fVec61[0] = if iSlow302 != 0 {if iSlow304 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp504 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec63[0] + (0.5 - (fTemp503 + F32::floor(self.fRec63[0] + (0.5 - fTemp503))))) + -1.0}} else {if iSlow303 != 0 {1.0 - 4.0 * F32::abs(self.fRec63[0] + (-0.25 - (fTemp503 + F32::floor(self.fRec63[0] + (0.25 - fTemp503)))))} else {F32::sin(6.2831855 * fTemp504)}};
																																																																																																																																				fTemp505 = fSlow169 * (fTemp497 * OscDsp_faustpower2_f(fTemp495) * (fTemp491 + 0.16666667 * fTemp492 * fTemp495 * fSlow321) + self.fVec61[1] + fTemp502 * OscDsp_faustpower2_f(fTemp500) * (fTemp498 + 0.16666667 * fTemp492 * fTemp500 * fSlow324) + self.fVec60[1]);
																																																																																																																																			}
																																																																																																																																				if iSlow109 & (iSlow149 & iSlow173) != 0 {
																																																																																																																																					fTemp506 = F32::max(1e+01, F32::min(self.fConst14, fTemp355 * F32::exp(0.6931472 * (8.0 * self.fRec53[0] + -3.0))));
																																																																																																																																					fTemp507 = 1.0 / F32::tan(self.fConst8 * fTemp506);
																																																																																																																																					self.fRec64[0] = (fTemp188 - self.fRec64[1] * (1.0 - fTemp507)) / (fTemp507 + 1.0);
																																																																																																																																					fTemp508 = fSlow172 * (self.fVec56[1] * (1.5 * self.fRec55[0] * self.fRec64[0] * F32::sqrt(self.fConst15 / fTemp506) + 1.0) / F32::sqrt(2.25 * fTemp488 + 1.0));
																																																																																																																																				}
																																																																																																																																					if iSlow109 & (iSlow149 & iSlow176) != 0 {
																																																																																																																																						fTemp509 = 0.6 * fTemp391 * self.fRec66[1] + 1.0;
																																																																																																																																						fTemp510 = self.fConst2 * fTemp355 * fTemp509;
																																																																																																																																						fTemp511 = self.fRec65[1] + fTemp510;
																																																																																																																																						iTemp512 = (fTemp511 >= 1.0) as i32;
																																																																																																																																						self.fRec65[0] = if iTemp0 != 0 {fTemp30} else {fTemp511 - (iTemp512) as F32};
																																																																																																																																						self.fRec66[0] = if iTemp512 != 0 {fTemp17} else {self.fRec66[1]};
																																																																																																																																						fTemp513 = F32::max(F32::abs(fTemp510), 1e-09);
																																																																																																																																						iTemp514 = (fTemp510 < 0.0) as i32;
																																																																																																																																						fTemp515 = if iTemp514 != 0 {fSlow316 - (self.fRec65[0] + F32::floor(fSlow316 - self.fRec65[0]))} else {self.fRec65[0] - (fSlow316 + F32::floor(self.fRec65[0] - fSlow316))};
																																																																																																																																						iTemp516 = ((fTemp515 < fTemp513) as i32) * if iTemp514 != 0 {-iSlow314} else {iSlow314};
																																																																																																																																						fTemp517 = if iTemp514 != 0 {fSlow318 - (self.fRec65[0] + F32::floor(fSlow318 - self.fRec65[0]))} else {self.fRec65[0] - (fSlow318 + F32::floor(self.fRec65[0] - fSlow318))};
																																																																																																																																						iTemp518 = ((fTemp517 < fTemp513) as i32) * if iTemp514 != 0 {-iSlow317} else {iSlow317};
																																																																																																																																						fTemp519 = F32::floor(self.fRec65[0]);
																																																																																																																																						fTemp520 = self.fRec65[0] - fTemp519;
																																																																																																																																						fTemp521 = if iSlow312 != 0 {if iSlow313 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp520 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec65[0] + (0.5 - (fTemp519 + F32::floor(self.fRec65[0] + (0.5 - fTemp519))))) + -1.0}} else {if iSlow315 != 0 {1.0 - 4.0 * F32::abs(self.fRec65[0] + (-0.25 - (fTemp519 + F32::floor(self.fRec65[0] + (0.25 - fTemp519)))))} else {F32::sin(6.2831855 * fTemp520)}};
																																																																																																																																						self.fVec62[0] = fTemp521;
																																																																																																																																						fTemp522 = self.fConst2 * fTemp355 * (4.0 * self.fRec53[0] * fTemp509 * (0.5 * (fTemp521 + self.fVec62[1] - (i32::wrapping_add(iTemp518, iTemp516)) as F32) + (fTemp517 * (iTemp518) as F32 + fTemp515 * (iTemp516) as F32) / fTemp513) + 1.0);
																																																																																																																																						iTemp523 = (fTemp522 < 0.0) as i32;
																																																																																																																																						fTemp524 = 0.5 * (if iTemp523 != 0 {-iSlow319} else {iSlow319}) as F32;
																																																																																																																																						fTemp525 = F32::max(F32::abs(fTemp522), 1e-09);
																																																																																																																																						fTemp526 = self.fRec67[1] + fTemp522;
																																																																																																																																						self.fRec67[0] = if iTemp0 != 0 {fTemp30} else {fTemp526 - F32::floor(fTemp526)};
																																																																																																																																						fTemp527 = if iTemp523 != 0 {fSlow320 - (self.fRec67[0] + F32::floor(fSlow320 - self.fRec67[0]))} else {self.fRec67[0] - (fSlow320 + F32::floor(self.fRec67[0] - fSlow320))};
																																																																																																																																						fTemp528 = F32::max(0.0, F32::min(1.0, fTemp527 / fTemp525));
																																																																																																																																						fTemp529 = 1.0 - fTemp528;
																																																																																																																																						fTemp530 = fSlow321 * fTemp525;
																																																																																																																																						fTemp531 = ((fTemp527 < fTemp525) as i32) as u32 as F32;
																																																																																																																																						fTemp532 = 0.5 * (if iTemp523 != 0 {-iSlow322} else {iSlow322}) as F32;
																																																																																																																																						fTemp533 = if iTemp523 != 0 {fSlow323 - (self.fRec67[0] + F32::floor(fSlow323 - self.fRec67[0]))} else {self.fRec67[0] - (fSlow323 + F32::floor(self.fRec67[0] - fSlow323))};
																																																																																																																																						fTemp534 = F32::max(0.0, F32::min(1.0, fTemp533 / fTemp525));
																																																																																																																																						fTemp535 = 1.0 - fTemp534;
																																																																																																																																						fTemp536 = fSlow324 * fTemp525;
																																																																																																																																						fTemp537 = ((fTemp533 < fTemp525) as i32) as u32 as F32;
																																																																																																																																						self.fVec63[0] = fTemp537 * OscDsp_faustpower2_f(fTemp535) * (0.16666667 * fTemp536 * fTemp535 - fTemp532) + fTemp531 * OscDsp_faustpower2_f(fTemp529) * (0.16666667 * fTemp530 * fTemp529 - fTemp524);
																																																																																																																																						fTemp538 = F32::floor(self.fRec67[0]);
																																																																																																																																						fTemp539 = self.fRec67[0] - fTemp538;
																																																																																																																																						self.fVec64[0] = if iSlow302 != 0 {if iSlow304 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp539 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec67[0] + (0.5 - (fTemp538 + F32::floor(self.fRec67[0] + (0.5 - fTemp538))))) + -1.0}} else {if iSlow303 != 0 {1.0 - 4.0 * F32::abs(self.fRec67[0] + (-0.25 - (fTemp538 + F32::floor(self.fRec67[0] + (0.25 - fTemp538)))))} else {F32::sin(6.2831855 * fTemp539)}};
																																																																																																																																						fTemp540 = fSlow175 * (fTemp531 * OscDsp_faustpower2_f(fTemp528) * (fTemp524 + 0.16666667 * fTemp530 * fTemp528) + self.fVec64[1] + fTemp537 * OscDsp_faustpower2_f(fTemp534) * (fTemp532 + 0.16666667 * fTemp536 * fTemp534) + self.fVec63[1]);
																																																																																																																																					}
																																																																																																																																						if iSlow109 & (iSlow149 & iSlow179) != 0 {
																																																																																																																																							iTemp541 = (fTemp356 < 0.0) as i32;
																																																																																																																																							fTemp542 = 0.5 * (if iTemp541 != 0 {-iSlow319} else {iSlow319}) as F32;
																																																																																																																																							fTemp543 = F32::max(fTemp394, 1e-09);
																																																																																																																																							fTemp544 = if iTemp541 != 0 {fSlow320 - (self.fRec49[0] + F32::floor(fSlow320 - self.fRec49[0]))} else {self.fRec49[0] - (fSlow320 + F32::floor(self.fRec49[0] - fSlow320))};
																																																																																																																																							fTemp545 = F32::max(0.0, F32::min(1.0, fTemp544 / fTemp543));
																																																																																																																																							fTemp546 = 1.0 - fTemp545;
																																																																																																																																							fTemp547 = fSlow321 * fTemp543;
																																																																																																																																							fTemp548 = ((fTemp544 < fTemp543) as i32) as u32 as F32;
																																																																																																																																							fTemp549 = 0.5 * (if iTemp541 != 0 {-iSlow322} else {iSlow322}) as F32;
																																																																																																																																							fTemp550 = if iTemp541 != 0 {fSlow323 - (self.fRec49[0] + F32::floor(fSlow323 - self.fRec49[0]))} else {self.fRec49[0] - (fSlow323 + F32::floor(self.fRec49[0] - fSlow323))};
																																																																																																																																							fTemp551 = F32::max(0.0, F32::min(1.0, fTemp550 / fTemp543));
																																																																																																																																							fTemp552 = 1.0 - fTemp551;
																																																																																																																																							fTemp553 = fSlow324 * fTemp543;
																																																																																																																																							fTemp554 = ((fTemp550 < fTemp543) as i32) as u32 as F32;
																																																																																																																																							self.fVec65[0] = fTemp554 * OscDsp_faustpower2_f(fTemp552) * (0.16666667 * fTemp553 * fTemp552 - fTemp549) + fTemp548 * OscDsp_faustpower2_f(fTemp546) * (0.16666667 * fTemp547 * fTemp546 - fTemp542);
																																																																																																																																							fTemp555 = F32::floor(self.fRec49[0]);
																																																																																																																																							fTemp556 = self.fRec49[0] - fTemp555;
																																																																																																																																							self.fVec66[0] = if iSlow302 != 0 {if iSlow304 != 0 {(i32::wrapping_sub(1, i32::wrapping_mul(2, (fTemp556 >= 0.5) as i32))) as F32} else {2.0 * (self.fRec49[0] + (0.5 - (fTemp555 + F32::floor(self.fRec49[0] + (0.5 - fTemp555))))) + -1.0}} else {if iSlow303 != 0 {1.0 - 4.0 * F32::abs(self.fRec49[0] + (-0.25 - (fTemp555 + F32::floor(self.fRec49[0] + (0.25 - fTemp555)))))} else {F32::sin(6.2831855 * fTemp556)}};
																																																																																																																																							fTemp557 = (9.0 * self.fRec55[0] + 1.0) * (1.5 * self.fRec53[0] + fTemp548 * OscDsp_faustpower2_f(fTemp545) * (fTemp542 + 0.16666667 * fTemp547 * fTemp545) + self.fVec66[1] + fTemp554 * OscDsp_faustpower2_f(fTemp551) * (fTemp549 + 0.16666667 * fTemp553 * fTemp551) + self.fVec65[1]);
																																																																																																																																							self.fVec67[0] = fTemp557;
																																																																																																																																							fTemp558 = fTemp557 - self.fVec67[1];
																																																																																																																																							iTemp559 = (F32::abs(fTemp558) < 0.0001) as i32;
																																																																																																																																							fTemp560 = if iTemp559 != 0 {F32::sin(0.7853982 * (fTemp557 + self.fVec67[1]))} else {-(0.63661975 * ((F32::cos(1.5707964 * fTemp557) - F32::cos(1.5707964 * self.fVec67[1])) / if iTemp559 != 0 {1.0} else {fTemp558}))};
																																																																																																																																							self.fVec68[0] = fTemp560;
																																																																																																																																							self.fRec68[0] = 0.995 * self.fRec68[1] + fTemp560 - self.fVec68[1];
																																																																																																																																							fTemp561 = fSlow178 * self.fRec68[0];
																																																																																																																																						}
																																																																																																																																							if iSlow109 & (iSlow149 & iSlow182) != 0 {
																																																																																																																																								fTemp562 = 1.0 - OscDsp_faustpower2_f(1.0 - fTemp391);
																																																																																																																																								fTemp563 = 1.0 - 0.5 * OscDsp_faustpower2_f(fTemp391);
																																																																																																																																								iTemp564 = (self.fRec53[0] > 0.5) as i32;
																																																																																																																																								fTemp565 = if iTemp564 != 0 {2e+01} else {1e+02 * OscDsp_faustpower2_f(2.0 * self.fRec53[0])};
																																																																																																																																								iTemp566 = unsafe { itbl5OscDspSIG5[((F32::max(0.0, F32::min(4.0, 1e+01 * (self.fRec53[0] + -0.5)))) as i32) as usize] };
																																																																																																																																								fTemp567 = (i32::wrapping_mul(100, i32::wrapping_mul(iTemp564, iTemp566))) as F32;
																																																																																																																																								fTemp568 = self.fConst2 * fTemp355 * F32::exp(0.0005776227 * (fTemp567 + 0.83 * fTemp565));
																																																																																																																																								fTemp569 = fTemp568 + self.fRec69[1];
																																																																																																																																								self.fRec69[0] = if iTemp0 != 0 {fTemp252} else {fTemp569 - F32::floor(fTemp569)};
																																																																																																																																								fTemp570 = self.fConst2 * fTemp355 * F32::exp(0.0005776227 * (0.36 * fTemp565 + fTemp567));
																																																																																																																																								fTemp571 = fTemp570 + self.fRec70[1];
																																																																																																																																								self.fRec70[0] = if iTemp0 != 0 {fTemp256} else {fTemp571 - F32::floor(fTemp571)};
																																																																																																																																								fTemp572 = self.fConst2 * fTemp355 * F32::exp(0.0005776227 * ((i32::wrapping_mul(0, iTemp566)) as F32 - fTemp565)) + self.fRec71[1];
																																																																																																																																								self.fRec71[0] = if iTemp0 != 0 {fTemp259} else {fTemp572 - F32::floor(fTemp572)};
																																																																																																																																								self.fVec69[0] = fTemp460 * fTemp563 + 0.5 * fTemp562 * (wt_basic(iSlow299, self.fConst0 * F32::abs(self.fConst2 * fTemp355 * F32::exp(-0.0005776227 * fTemp565)), self.fConst0, self.fRec71[0]) + wt_basic(iSlow299, self.fConst0 * F32::abs(fTemp570), self.fConst0, self.fRec70[0]) + wt_basic(iSlow299, self.fConst0 * F32::abs(fTemp568), self.fConst0, self.fRec69[0]));
																																																																																																																																								fTemp573 = fSlow181 * (self.fVec69[1] / F32::sqrt(OscDsp_faustpower2_f(fTemp563) + 0.75 * OscDsp_faustpower2_f(fTemp562)));
																																																																																																																																							}
																																																																																																																																								if iSlow109 & iSlow149 != 0 {
																																																																																																																																									fTemp574 = fSlow148 * fSlow330 * (fSlow329 * fTemp573 + fSlow328 * fTemp561 + fSlow327 * fTemp540 + fSlow326 * fTemp508 + fSlow325 * fTemp505 + fSlow311 * fTemp473 + fSlow310 * fTemp461 + fSlow309 * fTemp455 + fSlow308 * fTemp432 + fSlow307 * fTemp414 + fSlow300 * fTemp397 + fSlow298 * fTemp388);
																																																																																																																																								}
																																																																																																																																									if (iSlow109 & (iSlow106 & iSlow218)) | ((iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow194)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow106 & iSlow185))))))))))))))) != 0 {
																																																																																																																																										self.fVec70[0] = fSlow332;
																																																																																																																																										fTemp575 = if ((fSlow332 != self.fVec70[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec72[1] + -1.0};
																																																																																																																																										self.fRec72[0] = fTemp575;
																																																																																																																																										self.fRec73[0] = if iTemp0 != 0 {fSlow332} else {if (fTemp575 > 0.0) as i32 != 0 {self.fRec73[1] + (fSlow332 - self.fRec73[1]) / F32::max(1.0, fTemp575)} else {fSlow332}};
																																																																																																																																										fTemp576 = 3.0 * F32::max(0.0, F32::min(1.0, self.fRec73[0]));
																																																																																																																																										iTemp577 = std::cmp::min(2, (fTemp576) as i32);
																																																																																																																																										iTemp578 = (iTemp577 >= 2) as i32;
																																																																																																																																										iTemp579 = (iTemp577 >= 1) as i32;
																																																																																																																																										iTemp580 = i32::wrapping_mul(2176, (if iTemp578 != 0 {fSlow335} else {if iTemp579 != 0 {fSlow334} else {fSlow333}}) as i32);
																																																																																																																																									}
																																																																																																																																										if (iSlow109 & (iSlow106 & iSlow218)) | ((iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow106 & iSlow185)))))))))))))) != 0 {
																																																																																																																																											fTemp581 = 256.0 * fTemp355;
																																																																																																																																											iTemp582 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp581 > self.fConst16) as i32) + ((fTemp581 > self.fConst22) as i32), (fTemp581 > self.fConst21) as i32), (fTemp581 > self.fConst20) as i32), (fTemp581 > self.fConst19) as i32), (fTemp581 > self.fConst18) as i32), (fTemp581 > self.fConst17) as i32);
																																																																																																																																											iTemp583 = (iTemp582 >= 4) as i32;
																																																																																																																																											iTemp584 = (iTemp582 >= 2) as i32;
																																																																																																																																											iTemp585 = (iTemp582 >= 1) as i32;
																																																																																																																																											iTemp586 = (iTemp582 >= 3) as i32;
																																																																																																																																											iTemp587 = if iTemp583 != 0 {if (iTemp582 >= 6) as i32 != 0 {if (iTemp582 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp582 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp584 != 0 {if iTemp586 != 0 {1792} else {1536}} else {if iTemp585 != 0 {1024} else {0}}};
																																																																																																																																											iTemp588 = if iTemp583 != 0 {64} else {if iTemp584 != 0 {if iTemp586 != 0 {128} else {256}} else {if iTemp585 != 0 {512} else {1024}}};
																																																																																																																																											fTemp589 = self.fRec49[0] * (iTemp588) as F32;
																																																																																																																																											iTemp590 = (fTemp589) as i32;
																																																																																																																																											iTemp591 = (i32::wrapping_add(iTemp590, 1)) % iTemp588;
																																																																																																																																											fTemp592 = (iTemp590) as F32;
																																																																																																																																											fTemp593 = fTemp589 - fTemp592;
																																																																																																																																											fTemp594 = fTemp592 + (1.0 - fTemp589);
																																																																																																																																										}
																																																																																																																																											if (iSlow109 & (iSlow106 & iSlow218)) | ((iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow194)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow106 & iSlow185))))))))))))))) != 0 {
																																																																																																																																												fTemp595 = (iTemp577) as F32;
																																																																																																																																												fTemp596 = fTemp576 - fTemp595;
																																																																																																																																												iTemp597 = i32::wrapping_mul(2176, (if iTemp578 != 0 {fSlow334} else {if iTemp579 != 0 {fSlow333} else {fSlow336}}) as i32);
																																																																																																																																												fTemp598 = fTemp595 + (1.0 - fTemp576);
																																																																																																																																											}
																																																																																																																																												if (iSlow109 & (iSlow106 & iSlow218)) | ((iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow106 & iSlow185)))))))))))))) != 0 {
																																																																																																																																													fTemp599 = fTemp598 * (fTemp594 * wt_sample(i32::wrapping_add(iTemp587, i32::wrapping_add(iTemp590, iTemp597))) + fTemp593 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp597, iTemp587), iTemp591))) + fTemp596 * (fTemp594 * wt_sample(i32::wrapping_add(iTemp587, i32::wrapping_add(iTemp590, iTemp580))) + fTemp593 * wt_sample(i32::wrapping_add(iTemp591, i32::wrapping_add(iTemp587, iTemp580))));
																																																																																																																																												}
																																																																																																																																													if (iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow194)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow106 & iSlow185)))))))))))))) != 0 {
																																																																																																																																														self.fVec71[0] = fSlow337;
																																																																																																																																														fTemp600 = if ((fSlow337 != self.fVec71[1]) as i32) | iTemp0 != 0 {fSlow89} else {self.fRec74[1] + -1.0};
																																																																																																																																														self.fRec74[0] = fTemp600;
																																																																																																																																														self.fRec75[0] = if iTemp0 != 0 {fSlow337} else {if (fTemp600 > 0.0) as i32 != 0 {self.fRec75[1] + (fSlow337 - self.fRec75[1]) / F32::max(1.0, fTemp600)} else {fSlow337}};
																																																																																																																																														fTemp601 = F32::max(0.0, F32::min(1.0, self.fRec75[0]));
																																																																																																																																													}
																																																																																																																																														if (iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow106 & iSlow185)))) != 0 {
																																																																																																																																															fTemp602 = (1.0 - fTemp601) * fTemp599;
																																																																																																																																														}
																																																																																																																																															if iSlow109 & (iSlow106 & iSlow185) != 0 {
																																																																																																																																																fTemp603 = fSlow184 * (fTemp602 + fTemp601 * if (fTemp599 > 0.0) as i32 != 0 {1.0 - fTemp599} else {-1.0 - fTemp599});
																																																																																																																																															}
																																																																																																																																																if (iSlow109 & (iSlow106 & iSlow203)) | (iSlow109 & (iSlow106 & iSlow188)) != 0 {
																																																																																																																																																	fTemp604 = 512.0 * fTemp355;
																																																																																																																																																	iTemp605 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp604 > self.fConst16) as i32) + ((fTemp604 > self.fConst22) as i32), (fTemp604 > self.fConst21) as i32), (fTemp604 > self.fConst20) as i32), (fTemp604 > self.fConst19) as i32), (fTemp604 > self.fConst18) as i32), (fTemp604 > self.fConst17) as i32);
																																																																																																																																																	iTemp606 = (iTemp605 >= 4) as i32;
																																																																																																																																																	iTemp607 = (iTemp605 >= 2) as i32;
																																																																																																																																																	iTemp608 = (iTemp605 >= 1) as i32;
																																																																																																																																																	iTemp609 = (iTemp605 >= 3) as i32;
																																																																																																																																																	iTemp610 = if iTemp606 != 0 {64} else {if iTemp607 != 0 {if iTemp609 != 0 {128} else {256}} else {if iTemp608 != 0 {512} else {1024}}};
																																																																																																																																																	fTemp611 = (iTemp610) as F32;
																																																																																																																																																}
																																																																																																																																																	if iSlow109 & (iSlow106 & iSlow188) != 0 {
																																																																																																																																																		fTemp612 = fTemp611 * if iTemp358 != 0 {fTemp365} else {2.0 * fTemp450};
																																																																																																																																																		iTemp613 = (fTemp612) as i32;
																																																																																																																																																		iTemp614 = (i32::wrapping_add(iTemp613, 1)) % iTemp610;
																																																																																																																																																	}
																																																																																																																																																		if (iSlow109 & (iSlow106 & iSlow203)) | (iSlow109 & (iSlow106 & iSlow188)) != 0 {
																																																																																																																																																			iTemp615 = if iTemp606 != 0 {if (iTemp605 >= 6) as i32 != 0 {if (iTemp605 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp605 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp607 != 0 {if iTemp609 != 0 {1792} else {1536}} else {if iTemp608 != 0 {1024} else {0}}};
																																																																																																																																																			iTemp616 = i32::wrapping_add(iTemp580, iTemp615);
																																																																																																																																																		}
																																																																																																																																																			if iSlow109 & (iSlow106 & iSlow188) != 0 {
																																																																																																																																																				fTemp617 = (iTemp613) as F32;
																																																																																																																																																				fTemp618 = fTemp612 - fTemp617;
																																																																																																																																																				fTemp619 = fTemp617 + (1.0 - fTemp612);
																																																																																																																																																			}
																																																																																																																																																				if (iSlow109 & (iSlow106 & iSlow203)) | (iSlow109 & (iSlow106 & iSlow188)) != 0 {
																																																																																																																																																					iTemp620 = i32::wrapping_add(iTemp597, iTemp615);
																																																																																																																																																				}
																																																																																																																																																					if iSlow109 & (iSlow106 & iSlow188) != 0 {
																																																																																																																																																						fTemp621 = fSlow187 * (fTemp602 + fTemp601 * (fTemp598 * (fTemp619 * wt_sample(i32::wrapping_add(iTemp620, iTemp613)) + fTemp618 * wt_sample(i32::wrapping_add(iTemp620, iTemp614))) + fTemp596 * (fTemp619 * wt_sample(i32::wrapping_add(iTemp616, iTemp613)) + fTemp618 * wt_sample(i32::wrapping_add(iTemp616, iTemp614)))));
																																																																																																																																																					}
																																																																																																																																																						if iSlow109 & (iSlow106 & iSlow191) != 0 {
																																																																																																																																																							fTemp622 = fSlow190 * fTemp599 * (1.0 - fTemp601 * (1.0 - F32::sin(6.2831855 * self.fRec49[0] * (7.0 * fTemp601 + 1.0))));
																																																																																																																																																						}
																																																																																																																																																							if iSlow109 & (iSlow106 & iSlow194) != 0 {
																																																																																																																																																								fTemp623 = 2.0 * fTemp601 + 1.0;
																																																																																																																																																								fTemp624 = 256.0 * fTemp355 * fTemp623;
																																																																																																																																																								iTemp625 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(i32::wrapping_add(((fTemp624 > self.fConst16) as i32) + ((fTemp624 > self.fConst22) as i32), (fTemp624 > self.fConst21) as i32), (fTemp624 > self.fConst20) as i32), (fTemp624 > self.fConst19) as i32), (fTemp624 > self.fConst18) as i32), (fTemp624 > self.fConst17) as i32);
																																																																																																																																																								iTemp626 = (iTemp625 >= 4) as i32;
																																																																																																																																																								iTemp627 = (iTemp625 >= 2) as i32;
																																																																																																																																																								iTemp628 = (iTemp625 >= 1) as i32;
																																																																																																																																																								iTemp629 = (iTemp625 >= 3) as i32;
																																																																																																																																																								iTemp630 = if iTemp626 != 0 {if (iTemp625 >= 6) as i32 != 0 {if (iTemp625 >= 7) as i32 != 0 {2112} else {2048}} else {if (iTemp625 >= 5) as i32 != 0 {1984} else {1920}}} else {if iTemp627 != 0 {if iTemp629 != 0 {1792} else {1536}} else {if iTemp628 != 0 {1024} else {0}}};
																																																																																																																																																								iTemp631 = if iTemp626 != 0 {64} else {if iTemp627 != 0 {if iTemp629 != 0 {128} else {256}} else {if iTemp628 != 0 {512} else {1024}}};
																																																																																																																																																								fTemp632 = F32::powf(self.fRec49[0], fTemp623) * (iTemp631) as F32;
																																																																																																																																																								iTemp633 = (fTemp632) as i32;
																																																																																																																																																								iTemp634 = (i32::wrapping_add(iTemp633, 1)) % iTemp631;
																																																																																																																																																								fTemp635 = (iTemp633) as F32;
																																																																																																																																																								fTemp636 = fTemp632 - fTemp635;
																																																																																																																																																								fTemp637 = fTemp635 + (1.0 - fTemp632);
																																																																																																																																																								fTemp638 = fSlow193 * (fTemp598 * (fTemp637 * wt_sample(i32::wrapping_add(iTemp630, i32::wrapping_add(iTemp597, iTemp633))) + fTemp636 * wt_sample(i32::wrapping_add(i32::wrapping_add(iTemp597, iTemp630), iTemp634))) + fTemp596 * (fTemp637 * wt_sample(i32::wrapping_add(iTemp630, i32::wrapping_add(iTemp580, iTemp633))) + fTemp636 * wt_sample(i32::wrapping_add(iTemp634, i32::wrapping_add(iTemp580, iTemp630)))));
																																																																																																																																																							}
																																																																																																																																																								if iSlow109 & (iSlow106 & iSlow197) != 0 {
																																																																																																																																																									fTemp639 = fSlow196 * (fTemp602 + fTemp601 * F32::sin(7.8539815 * fTemp599));
																																																																																																																																																								}
																																																																																																																																																									if iSlow109 & (iSlow106 & iSlow200) != 0 {
																																																																																																																																																										fTemp640 = 0.25 * (fTemp599 * (4.0 * fTemp601 + 1.0) + 1.0);
																																																																																																																																																										fTemp641 = fSlow199 * (1.0 - 4.0 * F32::abs(fTemp640 + (-0.5 - F32::floor(fTemp640))));
																																																																																																																																																									}
																																																																																																																																																										if iSlow109 & (iSlow106 & iSlow203) != 0 {
																																																																																																																																																											fTemp642 = 2.0 * self.fRec49[0] * fTemp611;
																																																																																																																																																											iTemp643 = (fTemp642) as i32;
																																																																																																																																																											iTemp644 = (i32::wrapping_add(iTemp643, 1)) % iTemp610;
																																																																																																																																																											fTemp645 = (iTemp643) as F32;
																																																																																																																																																											fTemp646 = fTemp642 - fTemp645;
																																																																																																																																																											fTemp647 = fTemp645 + (1.0 - fTemp642);
																																																																																																																																																											fTemp648 = fSlow202 * (fTemp602 + fTemp601 * (iTemp358) as F32 * F32::abs(fTemp598 * (fTemp647 * wt_sample(i32::wrapping_add(iTemp615, i32::wrapping_add(iTemp597, iTemp643))) + fTemp646 * wt_sample(i32::wrapping_add(iTemp620, iTemp644))) + fTemp596 * (fTemp647 * wt_sample(i32::wrapping_add(iTemp615, i32::wrapping_add(iTemp580, iTemp643))) + fTemp646 * wt_sample(i32::wrapping_add(iTemp644, iTemp616)))));
																																																																																																																																																										}
																																																																																																																																																											if (iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | (iSlow109 & (iSlow106 & iSlow206))) != 0 {
																																																																																																																																																												fTemp649 = F32::tan(self.fConst25 * F32::powf(fSlow346, fTemp601));
																																																																																																																																																												fTemp650 = 1.0 / fTemp649;
																																																																																																																																																												fTemp651 = (fTemp650 + 1.4142135) / fTemp649 + 1.0;
																																																																																																																																																												self.fRec76[0] = fTemp599 - (self.fRec76[2] * ((fTemp650 + -1.4142135) / fTemp649 + 1.0) + 2.0 * self.fRec76[1] * (1.0 - 1.0 / OscDsp_faustpower2_f(fTemp649))) / fTemp651;
																																																																																																																																																												fTemp652 = (self.fRec76[2] + self.fRec76[0] + 2.0 * self.fRec76[1]) / fTemp651;
																																																																																																																																																											}
																																																																																																																																																												if iSlow109 & (iSlow106 & iSlow206) != 0 {
																																																																																																																																																													fTemp653 = fSlow205 * fTemp652;
																																																																																																																																																												}
																																																																																																																																																													if iSlow109 & (iSlow106 & iSlow209) != 0 {
																																																																																																																																																														fTemp654 = fSlow208 * fTemp652;
																																																																																																																																																													}
																																																																																																																																																														if iSlow109 & (iSlow106 & iSlow212) != 0 {
																																																																																																																																																															fTemp655 = fSlow211 * fTemp652;
																																																																																																																																																														}
																																																																																																																																																															if (iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | (iSlow109 & (iSlow106 & iSlow102))) != 0 {
																																																																																																																																																																fTemp656 = if iTemp9 != 0 {0.0} else {self.fRec78[1] + self.fConst2 * F32::max(self.fConst0 * F32::min(0.999, F32::powf(fSlow351, fTemp601)), 0.0)};
																																																																																																																																																																self.fRec78[0] = fTemp656 - F32::floor(fTemp656);
																																																																																																																																																																self.fRec77[0] = if (self.fRec78[0] < self.fRec78[1]) as i32 != 0 {fTemp599} else {self.fRec77[1]};
																																																																																																																																																															}
																																																																																																																																																																if iSlow109 & (iSlow106 & iSlow102) != 0 {
																																																																																																																																																																	fTemp657 = fSlow101 * self.fRec77[0];
																																																																																																																																																																}
																																																																																																																																																																	if iSlow109 & (iSlow106 & iSlow112) != 0 {
																																																																																																																																																																		fTemp658 = fSlow111 * self.fRec77[0];
																																																																																																																																																																	}
																																																																																																																																																																		if iSlow109 & (iSlow106 & iSlow115) != 0 {
																																																																																																																																																																			fTemp659 = fSlow114 * self.fRec77[0];
																																																																																																																																																																		}
																																																																																																																																																																			if iSlow109 & (iSlow106 & iSlow215) != 0 {
																																																																																																																																																																				fTemp660 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp601)) + -1.0));
																																																																																																																																																																				fTemp661 = fSlow214 * (F32::floor(fTemp599 * fTemp660 + 0.5) / fTemp660);
																																																																																																																																																																			}
																																																																																																																																																																				if iSlow109 & (iSlow106 & iSlow218) != 0 {
																																																																																																																																																																					fTemp662 = fSlow217 * fTemp599;
																																																																																																																																																																				}
																																																																																																																																																																					if iSlow109 & iSlow106 != 0 {
																																																																																																																																																																						fTemp663 = fSlow105 * (fSlow356 * fTemp662 + fSlow355 * fTemp661 + fSlow354 * fTemp659 + fSlow353 * fTemp658 + fSlow352 * fTemp657 + fSlow349 * fTemp655 + fSlow348 * fTemp654 + fSlow347 * fTemp653 + fSlow344 * fTemp648 + fSlow343 * fTemp641 + fSlow342 * fTemp639 + fSlow341 * fTemp638 + fSlow340 * fTemp622 + fSlow339 * fTemp621 + fSlow338 * fTemp603);
																																																																																																																																																																					}
																																																																																																																																																																						if iSlow109 != 0 {
																																																																																																																																																																							fTemp664 = fSlow108 * (fSlow357 * fTemp663 + fSlow331 * fTemp574);
																																																																																																																																																																						}
																																																																																																																																																																							*output1 = fSlow358 * fSlow291 * fTemp664;
																																																																																																																																																																							*output2 = fSlow292 * fTemp351;
																																																																																																																																																																							*output3 = fSlow358 * fTemp664;
																																																																																																																																																																							*output4 = fTemp24;
																																																																																																																																																																							*output5 = fTemp355;
																																																																																																																																																																							*output6 = fSlow291;
																																																																																																																																																																							*output7 = fTemp29;
																																																																																																																																																																							*output8 = (iTemp0) as F32;
																																																																																																																																																																							self.fVec0[1] = self.fVec0[0];
																																																																																																																																																																							self.iVec1[1] = self.iVec1[0];
																																																																																																																																																																							self.fVec2[1] = self.fVec2[0];
																																																																																																																																																																							self.fRec1[1] = self.fRec1[0];
																																																																																																																																																																							self.fRec2[1] = self.fRec2[0];
																																																																																																																																																																							self.fVec3[1] = self.fVec3[0];
																																																																																																																																																																							self.fRec3[1] = self.fRec3[0];
																																																																																																																																																																							self.fRec4[1] = self.fRec4[0];
																																																																																																																																																																							self.fVec4[1] = self.fVec4[0];
																																																																																																																																																																							self.fRec5[1] = self.fRec5[0];
																																																																																																																																																																							self.fRec6[1] = self.fRec6[0];
																																																																																																																																																																							if (fTemp8 != 0.0) as i32 != 0 {
																																																																																																																																																																								self.fRec8[1] = self.fRec8[0];
																																																																																																																																																																								self.fRec9[1] = self.fRec9[0];
																																																																																																																																																																								self.iRec11[1] = self.iRec11[0];
																																																																																																																																																																								self.fRec10[1] = self.fRec10[0];
																																																																																																																																																																								self.fRec12[1] = self.fRec12[0];
																																																																																																																																																																								self.fRec15[1] = self.fRec15[0];
																																																																																																																																																																							}
																																																																																																																																																																								self.iRec16[1] = self.iRec16[0];
																																																																																																																																																																								self.fVec6[1] = self.fVec6[0];
																																																																																																																																																																								if (fTemp8 != 0.0) as i32 != 0 {
																																																																																																																																																																									self.fRec14[1] = self.fRec14[0];
																																																																																																																																																																									self.fVec7[1] = self.fVec7[0];
																																																																																																																																																																									self.fRec13[1] = self.fRec13[0];
																																																																																																																																																																								}
																																																																																																																																																																									self.fRec17[1] = self.fRec17[0];
																																																																																																																																																																									self.fVec8[1] = self.fVec8[0];
																																																																																																																																																																									self.fRec18[1] = self.fRec18[0];
																																																																																																																																																																									self.fRec19[1] = self.fRec19[0];
																																																																																																																																																																									self.fRec20[1] = self.fRec20[0];
																																																																																																																																																																									self.fRec21[1] = self.fRec21[0];
																																																																																																																																																																									if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | ((iSlow2 & (iSlow44 & iSlow47)) | ((iSlow2 & (iSlow44 & iSlow41)) | ((iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10))))))))))))))))))))))))) != 0 {
																																																																																																																																																																										self.fRec0[1] = self.fRec0[0];
																																																																																																																																																																									}
																																																																																																																																																																										if (iSlow2 & (iSlow6 & iSlow37)) | ((iSlow2 & (iSlow6 & iSlow34)) | ((iSlow2 & (iSlow6 & iSlow224)) | ((iSlow2 & (iSlow6 & iSlow31)) | ((iSlow2 & (iSlow6 & iSlow221)) | ((iSlow2 & (iSlow6 & iSlow28)) | ((iSlow2 & (iSlow6 & iSlow25)) | ((iSlow2 & (iSlow6 & iSlow22)) | ((iSlow2 & (iSlow6 & iSlow19)) | ((iSlow2 & (iSlow6 & iSlow16)) | ((iSlow2 & (iSlow6 & iSlow13)) | (iSlow2 & (iSlow6 & iSlow10)))))))))))) != 0 {
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
																																																																																																																																																																																					if iSlow2 & (iSlow6 & iSlow221) != 0 {
																																																																																																																																																																																						self.fRec32[1] = self.fRec32[0];
																																																																																																																																																																																						self.fVec27[1] = self.fVec27[0];
																																																																																																																																																																																						self.fRec33[1] = self.fRec33[0];
																																																																																																																																																																																						self.fVec28[1] = self.fVec28[0];
																																																																																																																																																																																						self.fVec29[1] = self.fVec29[0];
																																																																																																																																																																																					}
																																																																																																																																																																																						if iSlow2 & (iSlow6 & iSlow31) != 0 {
																																																																																																																																																																																							self.fRec34[1] = self.fRec34[0];
																																																																																																																																																																																						}
																																																																																																																																																																																							if iSlow2 & (iSlow6 & iSlow224) != 0 {
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
																																																																																																																																																																																											if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow80)) | ((iSlow2 & (iSlow44 & iSlow77)) | ((iSlow2 & (iSlow44 & iSlow74)) | ((iSlow2 & (iSlow44 & iSlow71)) | ((iSlow2 & (iSlow44 & iSlow68)) | ((iSlow2 & (iSlow44 & iSlow65)) | ((iSlow2 & (iSlow44 & iSlow62)) | ((iSlow2 & (iSlow44 & iSlow59)) | ((iSlow2 & (iSlow44 & iSlow56)) | ((iSlow2 & (iSlow44 & iSlow53)) | ((iSlow2 & (iSlow44 & iSlow50)) | (iSlow2 & (iSlow44 & iSlow41)))))))))))))) != 0 {
																																																																																																																																																																																												self.fVec39[1] = self.fVec39[0];
																																																																																																																																																																																												self.fRec44[1] = self.fRec44[0];
																																																																																																																																																																																												self.fRec45[1] = self.fRec45[0];
																																																																																																																																																																																											}
																																																																																																																																																																																												if (iSlow2 & (iSlow44 & iSlow86)) | ((iSlow2 & (iSlow44 & iSlow68)) | (iSlow2 & (iSlow44 & iSlow62))) != 0 {
																																																																																																																																																																																													self.fRec46[2] = self.fRec46[1];
																																																																																																																																																																																													self.fRec46[1] = self.fRec46[0];
																																																																																																																																																																																												}
																																																																																																																																																																																													if (iSlow2 & (iSlow44 & iSlow83)) | ((iSlow2 & (iSlow44 & iSlow77)) | (iSlow2 & (iSlow44 & iSlow71))) != 0 {
																																																																																																																																																																																														self.fRec48[1] = self.fRec48[0];
																																																																																																																																																																																														self.fRec47[1] = self.fRec47[0];
																																																																																																																																																																																													}
																																																																																																																																																																																														self.fVec40[1] = self.fVec40[0];
																																																																																																																																																																																														self.fRec50[1] = self.fRec50[0];
																																																																																																																																																																																														self.fRec51[1] = self.fRec51[0];
																																																																																																																																																																																														if (iSlow109 & (iSlow106 & iSlow218)) | ((iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow194)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | ((iSlow109 & (iSlow106 & iSlow185)) | ((iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow179)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | ((iSlow109 & (iSlow149 & iSlow155)) | ((iSlow109 & (iSlow149 & iSlow152)) | (iSlow109 & (iSlow149 & iSlow146))))))))))))))))))))))))) != 0 {
																																																																																																																																																																																															self.fRec49[1] = self.fRec49[0];
																																																																																																																																																																																														}
																																																																																																																																																																																															if (iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow179)) | ((iSlow109 & (iSlow149 & iSlow176)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow170)) | ((iSlow109 & (iSlow149 & iSlow167)) | ((iSlow109 & (iSlow149 & iSlow164)) | ((iSlow109 & (iSlow149 & iSlow161)) | ((iSlow109 & (iSlow149 & iSlow158)) | ((iSlow109 & (iSlow149 & iSlow155)) | ((iSlow109 & (iSlow149 & iSlow152)) | (iSlow109 & (iSlow149 & iSlow146)))))))))))) != 0 {
																																																																																																																																																																																																self.fVec41[1] = self.fVec41[0];
																																																																																																																																																																																																self.fRec52[1] = self.fRec52[0];
																																																																																																																																																																																																self.fRec53[1] = self.fRec53[0];
																																																																																																																																																																																																self.fVec42[1] = self.fVec42[0];
																																																																																																																																																																																																self.fRec54[1] = self.fRec54[0];
																																																																																																																																																																																																self.fRec55[1] = self.fRec55[0];
																																																																																																																																																																																															}
																																																																																																																																																																																																if iSlow109 & (iSlow149 & iSlow146) != 0 {
																																																																																																																																																																																																	self.fVec43[1] = self.fVec43[0];
																																																																																																																																																																																																	self.fVec44[1] = self.fVec44[0];
																																																																																																																																																																																																}
																																																																																																																																																																																																	if iSlow109 & (iSlow149 & iSlow152) != 0 {
																																																																																																																																																																																																		self.fRec56[1] = self.fRec56[0];
																																																																																																																																																																																																		self.fVec45[1] = self.fVec45[0];
																																																																																																																																																																																																	}
																																																																																																																																																																																																		if (iSlow109 & (iSlow149 & iSlow158)) | (iSlow109 & (iSlow149 & iSlow155)) != 0 {
																																																																																																																																																																																																			self.iRec57[1] = self.iRec57[0];
																																																																																																																																																																																																		}
																																																																																																																																																																																																			if iSlow109 & (iSlow149 & iSlow155) != 0 {
																																																																																																																																																																																																				self.fVec46[1] = self.fVec46[0];
																																																																																																																																																																																																				self.fVec47[1] = self.fVec47[0];
																																																																																																																																																																																																			}
																																																																																																																																																																																																				if (iSlow109 & (iSlow149 & iSlow161)) | (iSlow109 & (iSlow149 & iSlow155)) != 0 {
																																																																																																																																																																																																					self.fVec48[1] = self.fVec48[0];
																																																																																																																																																																																																				}
																																																																																																																																																																																																					if iSlow109 & (iSlow149 & iSlow158) != 0 {
																																																																																																																																																																																																						self.fVec49[1] = self.fVec49[0];
																																																																																																																																																																																																						self.fVec50[1] = self.fVec50[0];
																																																																																																																																																																																																						self.fVec51[1] = self.fVec51[0];
																																																																																																																																																																																																					}
																																																																																																																																																																																																						if iSlow109 & (iSlow149 & iSlow161) != 0 {
																																																																																																																																																																																																							self.fVec52[1] = self.fVec52[0];
																																																																																																																																																																																																							self.fVec53[1] = self.fVec53[0];
																																																																																																																																																																																																						}
																																																																																																																																																																																																							if iSlow109 & (iSlow149 & iSlow164) != 0 {
																																																																																																																																																																																																								self.fRec58[1] = self.fRec58[0];
																																																																																																																																																																																																								self.fVec54[1] = self.fVec54[0];
																																																																																																																																																																																																								self.fRec59[1] = self.fRec59[0];
																																																																																																																																																																																																								self.fVec55[1] = self.fVec55[0];
																																																																																																																																																																																																							}
																																																																																																																																																																																																								if (iSlow109 & (iSlow149 & iSlow182)) | ((iSlow109 & (iSlow149 & iSlow173)) | ((iSlow109 & (iSlow149 & iSlow167)) | (iSlow109 & (iSlow149 & iSlow164)))) != 0 {
																																																																																																																																																																																																									self.fVec56[1] = self.fVec56[0];
																																																																																																																																																																																																								}
																																																																																																																																																																																																									if iSlow109 & (iSlow149 & iSlow167) != 0 {
																																																																																																																																																																																																										self.fRec60[1] = self.fRec60[0];
																																																																																																																																																																																																										self.fVec57[1] = self.fVec57[0];
																																																																																																																																																																																																										self.fRec61[1] = self.fRec61[0];
																																																																																																																																																																																																										self.fVec58[1] = self.fVec58[0];
																																																																																																																																																																																																									}
																																																																																																																																																																																																										if iSlow109 & (iSlow149 & iSlow170) != 0 {
																																																																																																																																																																																																											self.fRec62[1] = self.fRec62[0];
																																																																																																																																																																																																											self.fVec59[1] = self.fVec59[0];
																																																																																																																																																																																																											self.fRec63[1] = self.fRec63[0];
																																																																																																																																																																																																											self.fVec60[1] = self.fVec60[0];
																																																																																																																																																																																																											self.fVec61[1] = self.fVec61[0];
																																																																																																																																																																																																										}
																																																																																																																																																																																																											if iSlow109 & (iSlow149 & iSlow173) != 0 {
																																																																																																																																																																																																												self.fRec64[1] = self.fRec64[0];
																																																																																																																																																																																																											}
																																																																																																																																																																																																												if iSlow109 & (iSlow149 & iSlow176) != 0 {
																																																																																																																																																																																																													self.fRec65[1] = self.fRec65[0];
																																																																																																																																																																																																													self.fRec66[1] = self.fRec66[0];
																																																																																																																																																																																																													self.fVec62[1] = self.fVec62[0];
																																																																																																																																																																																																													self.fRec67[1] = self.fRec67[0];
																																																																																																																																																																																																													self.fVec63[1] = self.fVec63[0];
																																																																																																																																																																																																													self.fVec64[1] = self.fVec64[0];
																																																																																																																																																																																																												}
																																																																																																																																																																																																													if iSlow109 & (iSlow149 & iSlow179) != 0 {
																																																																																																																																																																																																														self.fVec65[1] = self.fVec65[0];
																																																																																																																																																																																																														self.fVec66[1] = self.fVec66[0];
																																																																																																																																																																																																														self.fVec67[1] = self.fVec67[0];
																																																																																																																																																																																																														self.fVec68[1] = self.fVec68[0];
																																																																																																																																																																																																														self.fRec68[1] = self.fRec68[0];
																																																																																																																																																																																																													}
																																																																																																																																																																																																														if iSlow109 & (iSlow149 & iSlow182) != 0 {
																																																																																																																																																																																																															self.fRec69[1] = self.fRec69[0];
																																																																																																																																																																																																															self.fRec70[1] = self.fRec70[0];
																																																																																																																																																																																																															self.fRec71[1] = self.fRec71[0];
																																																																																																																																																																																																															self.fVec69[1] = self.fVec69[0];
																																																																																																																																																																																																														}
																																																																																																																																																																																																															if (iSlow109 & (iSlow106 & iSlow218)) | ((iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow194)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow106 & iSlow185))))))))))))))) != 0 {
																																																																																																																																																																																																																self.fVec70[1] = self.fVec70[0];
																																																																																																																																																																																																																self.fRec72[1] = self.fRec72[0];
																																																																																																																																																																																																																self.fRec73[1] = self.fRec73[0];
																																																																																																																																																																																																															}
																																																																																																																																																																																																																if (iSlow109 & (iSlow106 & iSlow215)) | ((iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | ((iSlow109 & (iSlow106 & iSlow102)) | ((iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | ((iSlow109 & (iSlow106 & iSlow206)) | ((iSlow109 & (iSlow106 & iSlow203)) | ((iSlow109 & (iSlow106 & iSlow200)) | ((iSlow109 & (iSlow106 & iSlow197)) | ((iSlow109 & (iSlow106 & iSlow194)) | ((iSlow109 & (iSlow106 & iSlow191)) | ((iSlow109 & (iSlow106 & iSlow188)) | (iSlow109 & (iSlow106 & iSlow185)))))))))))))) != 0 {
																																																																																																																																																																																																																	self.fVec71[1] = self.fVec71[0];
																																																																																																																																																																																																																	self.fRec74[1] = self.fRec74[0];
																																																																																																																																																																																																																	self.fRec75[1] = self.fRec75[0];
																																																																																																																																																																																																																}
																																																																																																																																																																																																																	if (iSlow109 & (iSlow106 & iSlow212)) | ((iSlow109 & (iSlow106 & iSlow209)) | (iSlow109 & (iSlow106 & iSlow206))) != 0 {
																																																																																																																																																																																																																		self.fRec76[2] = self.fRec76[1];
																																																																																																																																																																																																																		self.fRec76[1] = self.fRec76[0];
																																																																																																																																																																																																																	}
																																																																																																																																																																																																																		if (iSlow109 & (iSlow106 & iSlow115)) | ((iSlow109 & (iSlow106 & iSlow112)) | (iSlow109 & (iSlow106 & iSlow102))) != 0 {
																																																																																																																																																																																																																			self.fRec78[1] = self.fRec78[0];
																																																																																																																																																																																																																			self.fRec77[1] = self.fRec77[0];
																																																																																																																																																																																																																		}
																																																																																																																																																																																																																		}
																																																																																																																																																																																																																		self.iTempPerm0 = iTemp9;
																																																																																																																																																																																																																		self.fTempPerm1 = fTemp10;
																																																																																																																																																																																																																		self.fTempPerm2 = fTemp11;
																																																																																																																																																																																																																		self.iTempPerm3 = iTemp12;
																																																																																																																																																																																																																		self.fTempPerm4 = fTemp15;
																																																																																																																																																																																																																		self.fTempPerm5 = fTemp18;
																																																																																																																																																																																																																		self.fTempPerm6 = fTemp19;
																																																																																																																																																																																																																		self.fTempPerm7 = fTemp25;
																																																																																																																																																																																																																		self.fTempPerm8 = fTemp26;
																																																																																																																																																																																																																		self.fTempPerm9 = fTemp30;
																																																																																																																																																																																																																		self.iTempPerm10 = iTemp31;
																																																																																																																																																																																																																		self.fTempPerm11 = fTemp32;
																																																																																																																																																																																																																		self.fTempPerm12 = fTemp33;
																																																																																																																																																																																																																		self.fTempPerm13 = fTemp34;
																																																																																																																																																																																																																		self.fTempPerm14 = fTemp35;
																																																																																																																																																																																																																		self.fTempPerm15 = fTemp36;
																																																																																																																																																																																																																		self.fTempPerm16 = fTemp37;
																																																																																																																																																																																																																		self.fTempPerm17 = fTemp38;
																																																																																																																																																																																																																		self.fTempPerm18 = fTemp39;
																																																																																																																																																																																																																		self.fTempPerm19 = fTemp40;
																																																																																																																																																																																																																		self.fTempPerm20 = fTemp41;
																																																																																																																																																																																																																		self.fTempPerm21 = fTemp42;
																																																																																																																																																																																																																		self.fTempPerm22 = fTemp43;
																																																																																																																																																																																																																		self.fTempPerm23 = fTemp44;
																																																																																																																																																																																																																		self.iTempPerm24 = iTemp45;
																																																																																																																																																																																																																		self.fTempPerm25 = fTemp46;
																																																																																																																																																																																																																		self.fTempPerm26 = fTemp47;
																																																																																																																																																																																																																		self.fTempPerm27 = fTemp48;
																																																																																																																																																																																																																		self.fTempPerm28 = fTemp49;
																																																																																																																																																																																																																		self.fTempPerm29 = fTemp50;
																																																																																																																																																																																																																		self.fTempPerm30 = fTemp51;
																																																																																																																																																																																																																		self.fTempPerm31 = fTemp52;
																																																																																																																																																																																																																		self.fTempPerm32 = fTemp53;
																																																																																																																																																																																																																		self.fTempPerm33 = fTemp54;
																																																																																																																																																																																																																		self.fTempPerm34 = fTemp55;
																																																																																																																																																																																																																		self.fTempPerm35 = fTemp56;
																																																																																																																																																																																																																		self.fTempPerm36 = fTemp57;
																																																																																																																																																																																																																		self.fTempPerm37 = fTemp58;
																																																																																																																																																																																																																		self.fTempPerm38 = fTemp59;
																																																																																																																																																																																																																		self.fTempPerm39 = fTemp60;
																																																																																																																																																																																																																		self.fTempPerm40 = fTemp61;
																																																																																																																																																																																																																		self.fTempPerm41 = fTemp62;
																																																																																																																																																																																																																		self.fTempPerm42 = fTemp63;
																																																																																																																																																																																																																		self.fTempPerm43 = fTemp64;
																																																																																																																																																																																																																		self.fTempPerm44 = fTemp65;
																																																																																																																																																																																																																		self.fTempPerm45 = fTemp66;
																																																																																																																																																																																																																		self.fTempPerm46 = fTemp67;
																																																																																																																																																																																																																		self.fTempPerm47 = fTemp68;
																																																																																																																																																																																																																		self.fTempPerm48 = fTemp69;
																																																																																																																																																																																																																		self.fTempPerm49 = fTemp70;
																																																																																																																																																																																																																		self.fTempPerm50 = fTemp71;
																																																																																																																																																																																																																		self.fTempPerm51 = fTemp72;
																																																																																																																																																																																																																		self.fTempPerm52 = fTemp73;
																																																																																																																																																																																																																		self.iTempPerm53 = iTemp74;
																																																																																																																																																																																																																		self.fTempPerm54 = fTemp75;
																																																																																																																																																																																																																		self.fTempPerm55 = fTemp76;
																																																																																																																																																																																																																		self.fTempPerm56 = fTemp77;
																																																																																																																																																																																																																		self.fTempPerm57 = fTemp78;
																																																																																																																																																																																																																		self.fTempPerm58 = fTemp79;
																																																																																																																																																																																																																		self.fTempPerm59 = fTemp80;
																																																																																																																																																																																																																		self.fTempPerm60 = fTemp81;
																																																																																																																																																																																																																		self.fTempPerm61 = fTemp82;
																																																																																																																																																																																																																		self.fTempPerm62 = fTemp83;
																																																																																																																																																																																																																		self.fTempPerm63 = fTemp84;
																																																																																																																																																																																																																		self.fTempPerm64 = fTemp85;
																																																																																																																																																																																																																		self.fTempPerm65 = fTemp86;
																																																																																																																																																																																																																		self.fTempPerm66 = fTemp87;
																																																																																																																																																																																																																		self.fTempPerm67 = fTemp88;
																																																																																																																																																																																																																		self.fTempPerm68 = fTemp89;
																																																																																																																																																																																																																		self.fTempPerm69 = fTemp90;
																																																																																																																																																																																																																		self.fTempPerm70 = fTemp91;
																																																																																																																																																																																																																		self.fTempPerm71 = fTemp92;
																																																																																																																																																																																																																		self.fTempPerm72 = fTemp93;
																																																																																																																																																																																																																		self.fTempPerm73 = fTemp94;
																																																																																																																																																																																																																		self.fTempPerm74 = fTemp95;
																																																																																																																																																																																																																		self.iTempPerm75 = iTemp96;
																																																																																																																																																																																																																		self.fTempPerm76 = fTemp97;
																																																																																																																																																																																																																		self.fTempPerm77 = fTemp98;
																																																																																																																																																																																																																		self.fTempPerm78 = fTemp99;
																																																																																																																																																																																																																		self.fTempPerm79 = fTemp100;
																																																																																																																																																																																																																		self.fTempPerm80 = fTemp101;
																																																																																																																																																																																																																		self.iTempPerm81 = iTemp102;
																																																																																																																																																																																																																		self.fTempPerm82 = fTemp103;
																																																																																																																																																																																																																		self.fTempPerm83 = fTemp104;
																																																																																																																																																																																																																		self.fTempPerm84 = fTemp105;
																																																																																																																																																																																																																		self.fTempPerm85 = fTemp106;
																																																																																																																																																																																																																		self.fTempPerm86 = fTemp107;
																																																																																																																																																																																																																		self.fTempPerm87 = fTemp108;
																																																																																																																																																																																																																		self.fTempPerm88 = fTemp109;
																																																																																																																																																																																																																		self.iTempPerm89 = iTemp110;
																																																																																																																																																																																																																		self.iTempPerm90 = iTemp111;
																																																																																																																																																																																																																		self.iTempPerm91 = iTemp112;
																																																																																																																																																																																																																		self.iTempPerm92 = iTemp113;
																																																																																																																																																																																																																		self.iTempPerm93 = iTemp114;
																																																																																																																																																																																																																		self.iTempPerm94 = iTemp115;
																																																																																																																																																																																																																		self.fTempPerm95 = fTemp116;
																																																																																																																																																																																																																		self.fTempPerm96 = fTemp117;
																																																																																																																																																																																																																		self.fTempPerm97 = fTemp118;
																																																																																																																																																																																																																		self.iTempPerm98 = iTemp119;
																																																																																																																																																																																																																		self.iTempPerm99 = iTemp120;
																																																																																																																																																																																																																		self.iTempPerm100 = iTemp121;
																																																																																																																																																																																																																		self.iTempPerm101 = iTemp122;
																																																																																																																																																																																																																		self.fTempPerm102 = fTemp123;
																																																																																																																																																																																																																		self.fTempPerm103 = fTemp124;
																																																																																																																																																																																																																		self.fTempPerm104 = fTemp125;
																																																																																																																																																																																																																		self.fTempPerm105 = fTemp126;
																																																																																																																																																																																																																		self.fTempPerm106 = fTemp127;
																																																																																																																																																																																																																		self.fTempPerm107 = fTemp128;
																																																																																																																																																																																																																		self.fTempPerm108 = fTemp129;
																																																																																																																																																																																																																		self.fTempPerm109 = fTemp130;
																																																																																																																																																																																																																		self.fTempPerm110 = fTemp131;
																																																																																																																																																																																																																		self.fTempPerm111 = fTemp132;
																																																																																																																																																																																																																		self.fTempPerm112 = fTemp133;
																																																																																																																																																																																																																		self.fTempPerm113 = fTemp134;
																																																																																																																																																																																																																		self.fTempPerm114 = fTemp135;
																																																																																																																																																																																																																		self.fTempPerm115 = fTemp136;
																																																																																																																																																																																																																		self.fTempPerm116 = fTemp137;
																																																																																																																																																																																																																		self.fTempPerm117 = fTemp138;
																																																																																																																																																																																																																		self.fTempPerm118 = fTemp139;
																																																																																																																																																																																																																		self.fTempPerm119 = fTemp140;
																																																																																																																																																																																																																		self.fTempPerm120 = fTemp141;
																																																																																																																																																																																																																		self.iTempPerm121 = iTemp142;
																																																																																																																																																																																																																		self.fTempPerm122 = fTemp143;
																																																																																																																																																																																																																		self.fTempPerm123 = fTemp144;
																																																																																																																																																																																																																		self.fTempPerm124 = fTemp145;
																																																																																																																																																																																																																		self.fTempPerm125 = fTemp146;
																																																																																																																																																																																																																		self.fTempPerm126 = fTemp147;
																																																																																																																																																																																																																		self.fTempPerm127 = fTemp148;
																																																																																																																																																																																																																		self.fTempPerm128 = fTemp149;
																																																																																																																																																																																																																		self.fTempPerm129 = fTemp150;
																																																																																																																																																																																																																		self.fTempPerm130 = fTemp151;
																																																																																																																																																																																																																		self.fTempPerm131 = fTemp152;
																																																																																																																																																																																																																		self.fTempPerm132 = fTemp153;
																																																																																																																																																																																																																		self.fTempPerm133 = fTemp154;
																																																																																																																																																																																																																		self.fTempPerm134 = fTemp155;
																																																																																																																																																																																																																		self.fTempPerm135 = fTemp156;
																																																																																																																																																																																																																		self.iTempPerm136 = iTemp157;
																																																																																																																																																																																																																		self.fTempPerm137 = fTemp158;
																																																																																																																																																																																																																		self.fTempPerm138 = fTemp159;
																																																																																																																																																																																																																		self.fTempPerm139 = fTemp160;
																																																																																																																																																																																																																		self.fTempPerm140 = fTemp161;
																																																																																																																																																																																																																		self.iTempPerm141 = iTemp162;
																																																																																																																																																																																																																		self.fTempPerm142 = fTemp163;
																																																																																																																																																																																																																		self.iTempPerm143 = iTemp164;
																																																																																																																																																																																																																		self.fTempPerm144 = fTemp165;
																																																																																																																																																																																																																		self.fTempPerm145 = fTemp166;
																																																																																																																																																																																																																		self.fTempPerm146 = fTemp167;
																																																																																																																																																																																																																		self.fTempPerm147 = fTemp168;
																																																																																																																																																																																																																		self.fTempPerm148 = fTemp169;
																																																																																																																																																																																																																		self.iTempPerm149 = iTemp170;
																																																																																																																																																																																																																		self.fTempPerm150 = fTemp171;
																																																																																																																																																																																																																		self.fTempPerm151 = fTemp172;
																																																																																																																																																																																																																		self.fTempPerm152 = fTemp173;
																																																																																																																																																																																																																		self.fTempPerm153 = fTemp174;
																																																																																																																																																																																																																		self.fTempPerm154 = fTemp175;
																																																																																																																																																																																																																		self.fTempPerm155 = fTemp176;
																																																																																																																																																																																																																		self.fTempPerm156 = fTemp177;
																																																																																																																																																																																																																		self.fTempPerm157 = fTemp178;
																																																																																																																																																																																																																		self.fTempPerm158 = fTemp179;
																																																																																																																																																																																																																		self.fTempPerm159 = fTemp180;
																																																																																																																																																																																																																		self.fTempPerm160 = fTemp181;
																																																																																																																																																																																																																		self.fTempPerm161 = fTemp182;
																																																																																																																																																																																																																		self.fTempPerm162 = fTemp183;
																																																																																																																																																																																																																		self.fTempPerm163 = fTemp184;
																																																																																																																																																																																																																		self.fTempPerm164 = fTemp185;
																																																																																																																																																																																																																		self.fTempPerm165 = fTemp186;
																																																																																																																																																																																																																		self.fTempPerm166 = fTemp187;
																																																																																																																																																																																																																		self.fTempPerm167 = fTemp188;
																																																																																																																																																																																																																		self.fTempPerm168 = fTemp189;
																																																																																																																																																																																																																		self.fTempPerm169 = fTemp190;
																																																																																																																																																																																																																		self.fTempPerm170 = fTemp191;
																																																																																																																																																																																																																		self.fTempPerm171 = fTemp192;
																																																																																																																																																																																																																		self.iTempPerm172 = iTemp193;
																																																																																																																																																																																																																		self.fTempPerm173 = fTemp194;
																																																																																																																																																																																																																		self.iTempPerm174 = iTemp195;
																																																																																																																																																																																																																		self.fTempPerm175 = fTemp196;
																																																																																																																																																																																																																		self.iTempPerm176 = iTemp197;
																																																																																																																																																																																																																		self.fTempPerm177 = fTemp198;
																																																																																																																																																																																																																		self.iTempPerm178 = iTemp199;
																																																																																																																																																																																																																		self.fTempPerm179 = fTemp200;
																																																																																																																																																																																																																		self.fTempPerm180 = fTemp201;
																																																																																																																																																																																																																		self.fTempPerm181 = fTemp202;
																																																																																																																																																																																																																		self.fTempPerm182 = fTemp203;
																																																																																																																																																																																																																		self.iTempPerm183 = iTemp204;
																																																																																																																																																																																																																		self.fTempPerm184 = fTemp205;
																																																																																																																																																																																																																		self.fTempPerm185 = fTemp206;
																																																																																																																																																																																																																		self.fTempPerm186 = fTemp207;
																																																																																																																																																																																																																		self.fTempPerm187 = fTemp208;
																																																																																																																																																																																																																		self.fTempPerm188 = fTemp209;
																																																																																																																																																																																																																		self.fTempPerm189 = fTemp210;
																																																																																																																																																																																																																		self.fTempPerm190 = fTemp211;
																																																																																																																																																																																																																		self.fTempPerm191 = fTemp212;
																																																																																																																																																																																																																		self.fTempPerm192 = fTemp213;
																																																																																																																																																																																																																		self.fTempPerm193 = fTemp214;
																																																																																																																																																																																																																		self.fTempPerm194 = fTemp215;
																																																																																																																																																																																																																		self.fTempPerm195 = fTemp216;
																																																																																																																																																																																																																		self.fTempPerm196 = fTemp217;
																																																																																																																																																																																																																		self.fTempPerm197 = fTemp218;
																																																																																																																																																																																																																		self.fTempPerm198 = fTemp219;
																																																																																																																																																																																																																		self.fTempPerm199 = fTemp220;
																																																																																																																																																																																																																		self.fTempPerm200 = fTemp221;
																																																																																																																																																																																																																		self.iTempPerm201 = iTemp222;
																																																																																																																																																																																																																		self.fTempPerm202 = fTemp223;
																																																																																																																																																																																																																		self.fTempPerm203 = fTemp224;
																																																																																																																																																																																																																		self.fTempPerm204 = fTemp225;
																																																																																																																																																																																																																		self.fTempPerm205 = fTemp226;
																																																																																																																																																																																																																		self.fTempPerm206 = fTemp227;
																																																																																																																																																																																																																		self.fTempPerm207 = fTemp228;
																																																																																																																																																																																																																		self.fTempPerm208 = fTemp229;
																																																																																																																																																																																																																		self.fTempPerm209 = fTemp230;
																																																																																																																																																																																																																		self.fTempPerm210 = fTemp231;
																																																																																																																																																																																																																		self.fTempPerm211 = fTemp232;
																																																																																																																																																																																																																		self.fTempPerm212 = fTemp233;
																																																																																																																																																																																																																		self.fTempPerm213 = fTemp234;
																																																																																																																																																																																																																		self.fTempPerm214 = fTemp235;
																																																																																																																																																																																																																		self.fTempPerm215 = fTemp236;
																																																																																																																																																																																																																		self.fTempPerm216 = fTemp237;
																																																																																																																																																																																																																		self.fTempPerm217 = fTemp238;
																																																																																																																																																																																																																		self.fTempPerm218 = fTemp239;
																																																																																																																																																																																																																		self.iTempPerm219 = iTemp240;
																																																																																																																																																																																																																		self.fTempPerm220 = fTemp241;
																																																																																																																																																																																																																		self.fTempPerm221 = fTemp242;
																																																																																																																																																																																																																		self.fTempPerm222 = fTemp243;
																																																																																																																																																																																																																		self.fTempPerm223 = fTemp244;
																																																																																																																																																																																																																		self.iTempPerm224 = iTemp245;
																																																																																																																																																																																																																		self.fTempPerm225 = fTemp246;
																																																																																																																																																																																																																		self.iTempPerm226 = iTemp247;
																																																																																																																																																																																																																		self.fTempPerm227 = fTemp248;
																																																																																																																																																																																																																		self.fTempPerm228 = fTemp249;
																																																																																																																																																																																																																		self.fTempPerm229 = fTemp250;
																																																																																																																																																																																																																		self.fTempPerm230 = fTemp251;
																																																																																																																																																																																																																		self.fTempPerm231 = fTemp252;
																																																																																																																																																																																																																		self.fTempPerm232 = fTemp253;
																																																																																																																																																																																																																		self.fTempPerm233 = fTemp254;
																																																																																																																																																																																																																		self.fTempPerm234 = fTemp255;
																																																																																																																																																																																																																		self.fTempPerm235 = fTemp256;
																																																																																																																																																																																																																		self.fTempPerm236 = fTemp257;
																																																																																																																																																																																																																		self.fTempPerm237 = fTemp258;
																																																																																																																																																																																																																		self.fTempPerm238 = fTemp259;
																																																																																																																																																																																																																		self.fTempPerm239 = fTemp260;
																																																																																																																																																																																																																		self.fTempPerm240 = fTemp261;
																																																																																																																																																																																																																		self.fTempPerm241 = fTemp262;
																																																																																																																																																																																																																		self.fTempPerm242 = fTemp263;
																																																																																																																																																																																																																		self.iTempPerm243 = iTemp264;
																																																																																																																																																																																																																		self.iTempPerm244 = iTemp265;
																																																																																																																																																																																																																		self.iTempPerm245 = iTemp266;
																																																																																																																																																																																																																		self.iTempPerm246 = iTemp267;
																																																																																																																																																																																																																		self.fTempPerm247 = fTemp268;
																																																																																																																																																																																																																		self.iTempPerm248 = iTemp269;
																																																																																																																																																																																																																		self.iTempPerm249 = iTemp270;
																																																																																																																																																																																																																		self.iTempPerm250 = iTemp271;
																																																																																																																																																																																																																		self.iTempPerm251 = iTemp272;
																																																																																																																																																																																																																		self.iTempPerm252 = iTemp273;
																																																																																																																																																																																																																		self.iTempPerm253 = iTemp274;
																																																																																																																																																																																																																		self.iTempPerm254 = iTemp275;
																																																																																																																																																																																																																		self.fTempPerm255 = fTemp276;
																																																																																																																																																																																																																		self.iTempPerm256 = iTemp277;
																																																																																																																																																																																																																		self.iTempPerm257 = iTemp278;
																																																																																																																																																																																																																		self.fTempPerm258 = fTemp279;
																																																																																																																																																																																																																		self.fTempPerm259 = fTemp280;
																																																																																																																																																																																																																		self.fTempPerm260 = fTemp281;
																																																																																																																																																																																																																		self.fTempPerm261 = fTemp282;
																																																																																																																																																																																																																		self.fTempPerm262 = fTemp283;
																																																																																																																																																																																																																		self.iTempPerm263 = iTemp284;
																																																																																																																																																																																																																		self.fTempPerm264 = fTemp285;
																																																																																																																																																																																																																		self.fTempPerm265 = fTemp286;
																																																																																																																																																																																																																		self.fTempPerm266 = fTemp287;
																																																																																																																																																																																																																		self.fTempPerm267 = fTemp288;
																																																																																																																																																																																																																		self.fTempPerm268 = fTemp289;
																																																																																																																																																																																																																		self.fTempPerm269 = fTemp290;
																																																																																																																																																																																																																		self.fTempPerm270 = fTemp291;
																																																																																																																																																																																																																		self.iTempPerm271 = iTemp292;
																																																																																																																																																																																																																		self.iTempPerm272 = iTemp293;
																																																																																																																																																																																																																		self.iTempPerm273 = iTemp294;
																																																																																																																																																																																																																		self.iTempPerm274 = iTemp295;
																																																																																																																																																																																																																		self.iTempPerm275 = iTemp296;
																																																																																																																																																																																																																		self.iTempPerm276 = iTemp297;
																																																																																																																																																																																																																		self.fTempPerm277 = fTemp298;
																																																																																																																																																																																																																		self.fTempPerm278 = fTemp299;
																																																																																																																																																																																																																		self.iTempPerm279 = iTemp300;
																																																																																																																																																																																																																		self.iTempPerm280 = iTemp301;
																																																																																																																																																																																																																		self.iTempPerm281 = iTemp302;
																																																																																																																																																																																																																		self.iTempPerm282 = iTemp303;
																																																																																																																																																																																																																		self.fTempPerm283 = fTemp304;
																																																																																																																																																																																																																		self.fTempPerm284 = fTemp305;
																																																																																																																																																																																																																		self.fTempPerm285 = fTemp306;
																																																																																																																																																																																																																		self.iTempPerm286 = iTemp307;
																																																																																																																																																																																																																		self.fTempPerm287 = fTemp308;
																																																																																																																																																																																																																		self.fTempPerm288 = fTemp309;
																																																																																																																																																																																																																		self.fTempPerm289 = fTemp310;
																																																																																																																																																																																																																		self.fTempPerm290 = fTemp311;
																																																																																																																																																																																																																		self.iTempPerm291 = iTemp312;
																																																																																																																																																																																																																		self.iTempPerm292 = iTemp313;
																																																																																																																																																																																																																		self.iTempPerm293 = iTemp314;
																																																																																																																																																																																																																		self.iTempPerm294 = iTemp315;
																																																																																																																																																																																																																		self.iTempPerm295 = iTemp316;
																																																																																																																																																																																																																		self.iTempPerm296 = iTemp317;
																																																																																																																																																																																																																		self.iTempPerm297 = iTemp318;
																																																																																																																																																																																																																		self.fTempPerm298 = fTemp319;
																																																																																																																																																																																																																		self.iTempPerm299 = iTemp320;
																																																																																																																																																																																																																		self.iTempPerm300 = iTemp321;
																																																																																																																																																																																																																		self.fTempPerm301 = fTemp322;
																																																																																																																																																																																																																		self.fTempPerm302 = fTemp323;
																																																																																																																																																																																																																		self.fTempPerm303 = fTemp324;
																																																																																																																																																																																																																		self.fTempPerm304 = fTemp325;
																																																																																																																																																																																																																		self.fTempPerm305 = fTemp326;
																																																																																																																																																																																																																		self.fTempPerm306 = fTemp327;
																																																																																																																																																																																																																		self.fTempPerm307 = fTemp328;
																																																																																																																																																																																																																		self.fTempPerm308 = fTemp329;
																																																																																																																																																																																																																		self.iTempPerm309 = iTemp330;
																																																																																																																																																																																																																		self.iTempPerm310 = iTemp331;
																																																																																																																																																																																																																		self.fTempPerm311 = fTemp332;
																																																																																																																																																																																																																		self.fTempPerm312 = fTemp333;
																																																																																																																																																																																																																		self.fTempPerm313 = fTemp334;
																																																																																																																																																																																																																		self.fTempPerm314 = fTemp335;
																																																																																																																																																																																																																		self.fTempPerm315 = fTemp336;
																																																																																																																																																																																																																		self.fTempPerm316 = fTemp337;
																																																																																																																																																																																																																		self.fTempPerm317 = fTemp338;
																																																																																																																																																																																																																		self.fTempPerm318 = fTemp339;
																																																																																																																																																																																																																		self.fTempPerm319 = fTemp340;
																																																																																																																																																																																																																		self.fTempPerm320 = fTemp341;
																																																																																																																																																																																																																		self.fTempPerm321 = fTemp342;
																																																																																																																																																																																																																		self.fTempPerm322 = fTemp343;
																																																																																																																																																																																																																		self.fTempPerm323 = fTemp344;
																																																																																																																																																																																																																		self.fTempPerm324 = fTemp345;
																																																																																																																																																																																																																		self.fTempPerm325 = fTemp346;
																																																																																																																																																																																																																		self.fTempPerm326 = fTemp347;
																																																																																																																																																																																																																		self.fTempPerm327 = fTemp348;
																																																																																																																																																																																																																		self.fTempPerm328 = fTemp349;
																																																																																																																																																																																																																		self.fTempPerm329 = fTemp350;
																																																																																																																																																																																																																		self.fTempPerm330 = fTemp351;
																																																																																																																																																																																																																		self.fTempPerm331 = fTemp356;
																																																																																																																																																																																																																		self.fTempPerm332 = fTemp357;
																																																																																																																																																																																																																		self.iTempPerm333 = iTemp358;
																																																																																																																																																																																																																		self.fTempPerm334 = fTemp359;
																																																																																																																																																																																																																		self.fTempPerm335 = fTemp360;
																																																																																																																																																																																																																		self.fTempPerm336 = fTemp361;
																																																																																																																																																																																																																		self.fTempPerm337 = fTemp362;
																																																																																																																																																																																																																		self.fTempPerm338 = fTemp363;
																																																																																																																																																																																																																		self.fTempPerm339 = fTemp364;
																																																																																																																																																																																																																		self.fTempPerm340 = fTemp365;
																																																																																																																																																																																																																		self.fTempPerm341 = fTemp366;
																																																																																																																																																																																																																		self.fTempPerm342 = fTemp367;
																																																																																																																																																																																																																		self.fTempPerm343 = fTemp368;
																																																																																																																																																																																																																		self.fTempPerm344 = fTemp369;
																																																																																																																																																																																																																		self.fTempPerm345 = fTemp370;
																																																																																																																																																																																																																		self.fTempPerm346 = fTemp371;
																																																																																																																																																																																																																		self.iTempPerm347 = iTemp372;
																																																																																																																																																																																																																		self.fTempPerm348 = fTemp373;
																																																																																																																																																																																																																		self.fTempPerm349 = fTemp374;
																																																																																																																																																																																																																		self.fTempPerm350 = fTemp375;
																																																																																																																																																																																																																		self.fTempPerm351 = fTemp376;
																																																																																																																																																																																																																		self.fTempPerm352 = fTemp377;
																																																																																																																																																																																																																		self.fTempPerm353 = fTemp378;
																																																																																																																																																																																																																		self.fTempPerm354 = fTemp379;
																																																																																																																																																																																																																		self.fTempPerm355 = fTemp380;
																																																																																																																																																																																																																		self.fTempPerm356 = fTemp381;
																																																																																																																																																																																																																		self.fTempPerm357 = fTemp382;
																																																																																																																																																																																																																		self.fTempPerm358 = fTemp383;
																																																																																																																																																																																																																		self.fTempPerm359 = fTemp384;
																																																																																																																																																																																																																		self.fTempPerm360 = fTemp385;
																																																																																																																																																																																																																		self.fTempPerm361 = fTemp386;
																																																																																																																																																																																																																		self.fTempPerm362 = fTemp387;
																																																																																																																																																																																																																		self.fTempPerm363 = fTemp388;
																																																																																																																																																																																																																		self.fTempPerm364 = fTemp389;
																																																																																																																																																																																																																		self.fTempPerm365 = fTemp390;
																																																																																																																																																																																																																		self.fTempPerm366 = fTemp391;
																																																																																																																																																																																																																		self.fTempPerm367 = fTemp392;
																																																																																																																																																																																																																		self.fTempPerm368 = fTemp393;
																																																																																																																																																																																																																		self.fTempPerm369 = fTemp394;
																																																																																																																																																																																																																		self.fTempPerm370 = fTemp395;
																																																																																																																																																																																																																		self.fTempPerm371 = fTemp396;
																																																																																																																																																																																																																		self.fTempPerm372 = fTemp397;
																																																																																																																																																																																																																		self.iTempPerm373 = iTemp398;
																																																																																																																																																																																																																		self.fTempPerm374 = fTemp399;
																																																																																																																																																																																																																		self.fTempPerm375 = fTemp400;
																																																																																																																																																																																																																		self.fTempPerm376 = fTemp401;
																																																																																																																																																																																																																		self.fTempPerm377 = fTemp402;
																																																																																																																																																																																																																		self.fTempPerm378 = fTemp403;
																																																																																																																																																																																																																		self.fTempPerm379 = fTemp404;
																																																																																																																																																																																																																		self.fTempPerm380 = fTemp405;
																																																																																																																																																																																																																		self.fTempPerm381 = fTemp406;
																																																																																																																																																																																																																		self.fTempPerm382 = fTemp407;
																																																																																																																																																																																																																		self.fTempPerm383 = fTemp408;
																																																																																																																																																																																																																		self.fTempPerm384 = fTemp409;
																																																																																																																																																																																																																		self.fTempPerm385 = fTemp410;
																																																																																																																																																																																																																		self.fTempPerm386 = fTemp411;
																																																																																																																																																																																																																		self.fTempPerm387 = fTemp412;
																																																																																																																																																																																																																		self.fTempPerm388 = fTemp413;
																																																																																																																																																																																																																		self.fTempPerm389 = fTemp414;
																																																																																																																																																																																																																		self.fTempPerm390 = fTemp415;
																																																																																																																																																																																																																		self.fTempPerm391 = fTemp416;
																																																																																																																																																																																																																		self.fTempPerm392 = fTemp417;
																																																																																																																																																																																																																		self.fTempPerm393 = fTemp418;
																																																																																																																																																																																																																		self.fTempPerm394 = fTemp419;
																																																																																																																																																																																																																		self.iTempPerm395 = iTemp420;
																																																																																																																																																																																																																		self.fTempPerm396 = fTemp421;
																																																																																																																																																																																																																		self.fTempPerm397 = fTemp422;
																																																																																																																																																																																																																		self.fTempPerm398 = fTemp423;
																																																																																																																																																																																																																		self.fTempPerm399 = fTemp424;
																																																																																																																																																																																																																		self.fTempPerm400 = fTemp425;
																																																																																																																																																																																																																		self.iTempPerm401 = iTemp426;
																																																																																																																																																																																																																		self.fTempPerm402 = fTemp427;
																																																																																																																																																																																																																		self.fTempPerm403 = fTemp428;
																																																																																																																																																																																																																		self.fTempPerm404 = fTemp429;
																																																																																																																																																																																																																		self.fTempPerm405 = fTemp430;
																																																																																																																																																																																																																		self.fTempPerm406 = fTemp431;
																																																																																																																																																																																																																		self.fTempPerm407 = fTemp432;
																																																																																																																																																																																																																		self.fTempPerm408 = fTemp433;
																																																																																																																																																																																																																		self.iTempPerm409 = iTemp434;
																																																																																																																																																																																																																		self.iTempPerm410 = iTemp435;
																																																																																																																																																																																																																		self.iTempPerm411 = iTemp436;
																																																																																																																																																																																																																		self.iTempPerm412 = iTemp437;
																																																																																																																																																																																																																		self.iTempPerm413 = iTemp438;
																																																																																																																																																																																																																		self.iTempPerm414 = iTemp439;
																																																																																																																																																																																																																		self.fTempPerm415 = fTemp440;
																																																																																																																																																																																																																		self.fTempPerm416 = fTemp441;
																																																																																																																																																																																																																		self.fTempPerm417 = fTemp442;
																																																																																																																																																																																																																		self.iTempPerm418 = iTemp443;
																																																																																																																																																																																																																		self.iTempPerm419 = iTemp444;
																																																																																																																																																																																																																		self.iTempPerm420 = iTemp445;
																																																																																																																																																																																																																		self.iTempPerm421 = iTemp446;
																																																																																																																																																																																																																		self.fTempPerm422 = fTemp447;
																																																																																																																																																																																																																		self.fTempPerm423 = fTemp448;
																																																																																																																																																																																																																		self.fTempPerm424 = fTemp449;
																																																																																																																																																																																																																		self.fTempPerm425 = fTemp450;
																																																																																																																																																																																																																		self.fTempPerm426 = fTemp451;
																																																																																																																																																																																																																		self.fTempPerm427 = fTemp452;
																																																																																																																																																																																																																		self.fTempPerm428 = fTemp453;
																																																																																																																																																																																																																		self.fTempPerm429 = fTemp454;
																																																																																																																																																																																																																		self.fTempPerm430 = fTemp455;
																																																																																																																																																																																																																		self.fTempPerm431 = fTemp456;
																																																																																																																																																																																																																		self.fTempPerm432 = fTemp457;
																																																																																																																																																																																																																		self.fTempPerm433 = fTemp458;
																																																																																																																																																																																																																		self.fTempPerm434 = fTemp459;
																																																																																																																																																																																																																		self.fTempPerm435 = fTemp460;
																																																																																																																																																																																																																		self.fTempPerm436 = fTemp461;
																																																																																																																																																																																																																		self.iTempPerm437 = iTemp462;
																																																																																																																																																																																																																		self.fTempPerm438 = fTemp463;
																																																																																																																																																																																																																		self.fTempPerm439 = fTemp464;
																																																																																																																																																																																																																		self.fTempPerm440 = fTemp465;
																																																																																																																																																																																																																		self.fTempPerm441 = fTemp466;
																																																																																																																																																																																																																		self.fTempPerm442 = fTemp467;
																																																																																																																																																																																																																		self.fTempPerm443 = fTemp468;
																																																																																																																																																																																																																		self.fTempPerm444 = fTemp469;
																																																																																																																																																																																																																		self.fTempPerm445 = fTemp470;
																																																																																																																																																																																																																		self.fTempPerm446 = fTemp471;
																																																																																																																																																																																																																		self.fTempPerm447 = fTemp472;
																																																																																																																																																																																																																		self.fTempPerm448 = fTemp473;
																																																																																																																																																																																																																		self.fTempPerm449 = fTemp474;
																																																																																																																																																																																																																		self.fTempPerm450 = fTemp475;
																																																																																																																																																																																																																		self.fTempPerm451 = fTemp476;
																																																																																																																																																																																																																		self.iTempPerm452 = iTemp477;
																																																																																																																																																																																																																		self.fTempPerm453 = fTemp478;
																																																																																																																																																																																																																		self.fTempPerm454 = fTemp479;
																																																																																																																																																																																																																		self.fTempPerm455 = fTemp480;
																																																																																																																																																																																																																		self.fTempPerm456 = fTemp481;
																																																																																																																																																																																																																		self.iTempPerm457 = iTemp482;
																																																																																																																																																																																																																		self.fTempPerm458 = fTemp483;
																																																																																																																																																																																																																		self.iTempPerm459 = iTemp484;
																																																																																																																																																																																																																		self.fTempPerm460 = fTemp485;
																																																																																																																																																																																																																		self.fTempPerm461 = fTemp486;
																																																																																																																																																																																																																		self.fTempPerm462 = fTemp487;
																																																																																																																																																																																																																		self.fTempPerm463 = fTemp488;
																																																																																																																																																																																																																		self.fTempPerm464 = fTemp489;
																																																																																																																																																																																																																		self.iTempPerm465 = iTemp490;
																																																																																																																																																																																																																		self.fTempPerm466 = fTemp491;
																																																																																																																																																																																																																		self.fTempPerm467 = fTemp492;
																																																																																																																																																																																																																		self.fTempPerm468 = fTemp493;
																																																																																																																																																																																																																		self.fTempPerm469 = fTemp494;
																																																																																																																																																																																																																		self.fTempPerm470 = fTemp495;
																																																																																																																																																																																																																		self.fTempPerm471 = fTemp496;
																																																																																																																																																																																																																		self.fTempPerm472 = fTemp497;
																																																																																																																																																																																																																		self.fTempPerm473 = fTemp498;
																																																																																																																																																																																																																		self.fTempPerm474 = fTemp499;
																																																																																																																																																																																																																		self.fTempPerm475 = fTemp500;
																																																																																																																																																																																																																		self.fTempPerm476 = fTemp501;
																																																																																																																																																																																																																		self.fTempPerm477 = fTemp502;
																																																																																																																																																																																																																		self.fTempPerm478 = fTemp503;
																																																																																																																																																																																																																		self.fTempPerm479 = fTemp504;
																																																																																																																																																																																																																		self.fTempPerm480 = fTemp505;
																																																																																																																																																																																																																		self.fTempPerm481 = fTemp506;
																																																																																																																																																																																																																		self.fTempPerm482 = fTemp507;
																																																																																																																																																																																																																		self.fTempPerm483 = fTemp508;
																																																																																																																																																																																																																		self.fTempPerm484 = fTemp509;
																																																																																																																																																																																																																		self.fTempPerm485 = fTemp510;
																																																																																																																																																																																																																		self.fTempPerm486 = fTemp511;
																																																																																																																																																																																																																		self.iTempPerm487 = iTemp512;
																																																																																																																																																																																																																		self.fTempPerm488 = fTemp513;
																																																																																																																																																																																																																		self.iTempPerm489 = iTemp514;
																																																																																																																																																																																																																		self.fTempPerm490 = fTemp515;
																																																																																																																																																																																																																		self.iTempPerm491 = iTemp516;
																																																																																																																																																																																																																		self.fTempPerm492 = fTemp517;
																																																																																																																																																																																																																		self.iTempPerm493 = iTemp518;
																																																																																																																																																																																																																		self.fTempPerm494 = fTemp519;
																																																																																																																																																																																																																		self.fTempPerm495 = fTemp520;
																																																																																																																																																																																																																		self.fTempPerm496 = fTemp521;
																																																																																																																																																																																																																		self.fTempPerm497 = fTemp522;
																																																																																																																																																																																																																		self.iTempPerm498 = iTemp523;
																																																																																																																																																																																																																		self.fTempPerm499 = fTemp524;
																																																																																																																																																																																																																		self.fTempPerm500 = fTemp525;
																																																																																																																																																																																																																		self.fTempPerm501 = fTemp526;
																																																																																																																																																																																																																		self.fTempPerm502 = fTemp527;
																																																																																																																																																																																																																		self.fTempPerm503 = fTemp528;
																																																																																																																																																																																																																		self.fTempPerm504 = fTemp529;
																																																																																																																																																																																																																		self.fTempPerm505 = fTemp530;
																																																																																																																																																																																																																		self.fTempPerm506 = fTemp531;
																																																																																																																																																																																																																		self.fTempPerm507 = fTemp532;
																																																																																																																																																																																																																		self.fTempPerm508 = fTemp533;
																																																																																																																																																																																																																		self.fTempPerm509 = fTemp534;
																																																																																																																																																																																																																		self.fTempPerm510 = fTemp535;
																																																																																																																																																																																																																		self.fTempPerm511 = fTemp536;
																																																																																																																																																																																																																		self.fTempPerm512 = fTemp537;
																																																																																																																																																																																																																		self.fTempPerm513 = fTemp538;
																																																																																																																																																																																																																		self.fTempPerm514 = fTemp539;
																																																																																																																																																																																																																		self.fTempPerm515 = fTemp540;
																																																																																																																																																																																																																		self.iTempPerm516 = iTemp541;
																																																																																																																																																																																																																		self.fTempPerm517 = fTemp542;
																																																																																																																																																																																																																		self.fTempPerm518 = fTemp543;
																																																																																																																																																																																																																		self.fTempPerm519 = fTemp544;
																																																																																																																																																																																																																		self.fTempPerm520 = fTemp545;
																																																																																																																																																																																																																		self.fTempPerm521 = fTemp546;
																																																																																																																																																																																																																		self.fTempPerm522 = fTemp547;
																																																																																																																																																																																																																		self.fTempPerm523 = fTemp548;
																																																																																																																																																																																																																		self.fTempPerm524 = fTemp549;
																																																																																																																																																																																																																		self.fTempPerm525 = fTemp550;
																																																																																																																																																																																																																		self.fTempPerm526 = fTemp551;
																																																																																																																																																																																																																		self.fTempPerm527 = fTemp552;
																																																																																																																																																																																																																		self.fTempPerm528 = fTemp553;
																																																																																																																																																																																																																		self.fTempPerm529 = fTemp554;
																																																																																																																																																																																																																		self.fTempPerm530 = fTemp555;
																																																																																																																																																																																																																		self.fTempPerm531 = fTemp556;
																																																																																																																																																																																																																		self.fTempPerm532 = fTemp557;
																																																																																																																																																																																																																		self.fTempPerm533 = fTemp558;
																																																																																																																																																																																																																		self.iTempPerm534 = iTemp559;
																																																																																																																																																																																																																		self.fTempPerm535 = fTemp560;
																																																																																																																																																																																																																		self.fTempPerm536 = fTemp561;
																																																																																																																																																																																																																		self.fTempPerm537 = fTemp562;
																																																																																																																																																																																																																		self.fTempPerm538 = fTemp563;
																																																																																																																																																																																																																		self.iTempPerm539 = iTemp564;
																																																																																																																																																																																																																		self.fTempPerm540 = fTemp565;
																																																																																																																																																																																																																		self.iTempPerm541 = iTemp566;
																																																																																																																																																																																																																		self.fTempPerm542 = fTemp567;
																																																																																																																																																																																																																		self.fTempPerm543 = fTemp568;
																																																																																																																																																																																																																		self.fTempPerm544 = fTemp569;
																																																																																																																																																																																																																		self.fTempPerm545 = fTemp570;
																																																																																																																																																																																																																		self.fTempPerm546 = fTemp571;
																																																																																																																																																																																																																		self.fTempPerm547 = fTemp572;
																																																																																																																																																																																																																		self.fTempPerm548 = fTemp573;
																																																																																																																																																																																																																		self.fTempPerm549 = fTemp574;
																																																																																																																																																																																																																		self.fTempPerm550 = fTemp575;
																																																																																																																																																																																																																		self.fTempPerm551 = fTemp576;
																																																																																																																																																																																																																		self.iTempPerm552 = iTemp577;
																																																																																																																																																																																																																		self.iTempPerm553 = iTemp578;
																																																																																																																																																																																																																		self.iTempPerm554 = iTemp579;
																																																																																																																																																																																																																		self.iTempPerm555 = iTemp580;
																																																																																																																																																																																																																		self.fTempPerm556 = fTemp581;
																																																																																																																																																																																																																		self.iTempPerm557 = iTemp582;
																																																																																																																																																																																																																		self.iTempPerm558 = iTemp583;
																																																																																																																																																																																																																		self.iTempPerm559 = iTemp584;
																																																																																																																																																																																																																		self.iTempPerm560 = iTemp585;
																																																																																																																																																																																																																		self.iTempPerm561 = iTemp586;
																																																																																																																																																																																																																		self.iTempPerm562 = iTemp587;
																																																																																																																																																																																																																		self.iTempPerm563 = iTemp588;
																																																																																																																																																																																																																		self.fTempPerm564 = fTemp589;
																																																																																																																																																																																																																		self.iTempPerm565 = iTemp590;
																																																																																																																																																																																																																		self.iTempPerm566 = iTemp591;
																																																																																																																																																																																																																		self.fTempPerm567 = fTemp592;
																																																																																																																																																																																																																		self.fTempPerm568 = fTemp593;
																																																																																																																																																																																																																		self.fTempPerm569 = fTemp594;
																																																																																																																																																																																																																		self.fTempPerm570 = fTemp595;
																																																																																																																																																																																																																		self.fTempPerm571 = fTemp596;
																																																																																																																																																																																																																		self.iTempPerm572 = iTemp597;
																																																																																																																																																																																																																		self.fTempPerm573 = fTemp598;
																																																																																																																																																																																																																		self.fTempPerm574 = fTemp599;
																																																																																																																																																																																																																		self.fTempPerm575 = fTemp600;
																																																																																																																																																																																																																		self.fTempPerm576 = fTemp601;
																																																																																																																																																																																																																		self.fTempPerm577 = fTemp602;
																																																																																																																																																																																																																		self.fTempPerm578 = fTemp603;
																																																																																																																																																																																																																		self.fTempPerm579 = fTemp604;
																																																																																																																																																																																																																		self.iTempPerm580 = iTemp605;
																																																																																																																																																																																																																		self.iTempPerm581 = iTemp606;
																																																																																																																																																																																																																		self.iTempPerm582 = iTemp607;
																																																																																																																																																																																																																		self.iTempPerm583 = iTemp608;
																																																																																																																																																																																																																		self.iTempPerm584 = iTemp609;
																																																																																																																																																																																																																		self.iTempPerm585 = iTemp610;
																																																																																																																																																																																																																		self.fTempPerm586 = fTemp611;
																																																																																																																																																																																																																		self.fTempPerm587 = fTemp612;
																																																																																																																																																																																																																		self.iTempPerm588 = iTemp613;
																																																																																																																																																																																																																		self.iTempPerm589 = iTemp614;
																																																																																																																																																																																																																		self.iTempPerm590 = iTemp615;
																																																																																																																																																																																																																		self.iTempPerm591 = iTemp616;
																																																																																																																																																																																																																		self.fTempPerm592 = fTemp617;
																																																																																																																																																																																																																		self.fTempPerm593 = fTemp618;
																																																																																																																																																																																																																		self.fTempPerm594 = fTemp619;
																																																																																																																																																																																																																		self.iTempPerm595 = iTemp620;
																																																																																																																																																																																																																		self.fTempPerm596 = fTemp621;
																																																																																																																																																																																																																		self.fTempPerm597 = fTemp622;
																																																																																																																																																																																																																		self.fTempPerm598 = fTemp623;
																																																																																																																																																																																																																		self.fTempPerm599 = fTemp624;
																																																																																																																																																																																																																		self.iTempPerm600 = iTemp625;
																																																																																																																																																																																																																		self.iTempPerm601 = iTemp626;
																																																																																																																																																																																																																		self.iTempPerm602 = iTemp627;
																																																																																																																																																																																																																		self.iTempPerm603 = iTemp628;
																																																																																																																																																																																																																		self.iTempPerm604 = iTemp629;
																																																																																																																																																																																																																		self.iTempPerm605 = iTemp630;
																																																																																																																																																																																																																		self.iTempPerm606 = iTemp631;
																																																																																																																																																																																																																		self.fTempPerm607 = fTemp632;
																																																																																																																																																																																																																		self.iTempPerm608 = iTemp633;
																																																																																																																																																																																																																		self.iTempPerm609 = iTemp634;
																																																																																																																																																																																																																		self.fTempPerm610 = fTemp635;
																																																																																																																																																																																																																		self.fTempPerm611 = fTemp636;
																																																																																																																																																																																																																		self.fTempPerm612 = fTemp637;
																																																																																																																																																																																																																		self.fTempPerm613 = fTemp638;
																																																																																																																																																																																																																		self.fTempPerm614 = fTemp639;
																																																																																																																																																																																																																		self.fTempPerm615 = fTemp640;
																																																																																																																																																																																																																		self.fTempPerm616 = fTemp641;
																																																																																																																																																																																																																		self.fTempPerm617 = fTemp642;
																																																																																																																																																																																																																		self.iTempPerm618 = iTemp643;
																																																																																																																																																																																																																		self.iTempPerm619 = iTemp644;
																																																																																																																																																																																																																		self.fTempPerm620 = fTemp645;
																																																																																																																																																																																																																		self.fTempPerm621 = fTemp646;
																																																																																																																																																																																																																		self.fTempPerm622 = fTemp647;
																																																																																																																																																																																																																		self.fTempPerm623 = fTemp648;
																																																																																																																																																																																																																		self.fTempPerm624 = fTemp649;
																																																																																																																																																																																																																		self.fTempPerm625 = fTemp650;
																																																																																																																																																																																																																		self.fTempPerm626 = fTemp651;
																																																																																																																																																																																																																		self.fTempPerm627 = fTemp652;
																																																																																																																																																																																																																		self.fTempPerm628 = fTemp653;
																																																																																																																																																																																																																		self.fTempPerm629 = fTemp654;
																																																																																																																																																																																																																		self.fTempPerm630 = fTemp655;
																																																																																																																																																																																																																		self.fTempPerm631 = fTemp656;
																																																																																																																																																																																																																		self.fTempPerm632 = fTemp657;
																																																																																																																																																																																																																		self.fTempPerm633 = fTemp658;
																																																																																																																																																																																																																		self.fTempPerm634 = fTemp659;
																																																																																																																																																																																																																		self.fTempPerm635 = fTemp660;
																																																																																																																																																																																																																		self.fTempPerm636 = fTemp661;
																																																																																																																																																																																																																		self.fTempPerm637 = fTemp662;
																																																																																																																																																																																																																		self.fTempPerm638 = fTemp663;
																																																																																																																																																																																																																		self.fTempPerm639 = fTemp664;
																																																																																																																																																																																																																	}

}

