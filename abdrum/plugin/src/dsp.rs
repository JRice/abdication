/* ------------------------------------------------------------
name: "ABDrum"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn VoiceDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

static mut fVoiceDspSIG0Wave0: [F32;164] = [1.402,2.0216,0.8095,1.255,1.2416,0.8105,1.2692,0.6545,1.5366,0.5419,1.3929,0.6378,0.5266,0.9656,0.7093,0.9259,0.6693,0.5651,0.5573,0.74,0.4078,0.5147,0.6403,0.5399,0.4848,0.4498,0.72,0.6251,0.6809,0.6341,0.8882,0.5196,0.7181,0.6129,0.5692,0.5459,0.5966,0.5116,0.5447,0.4685,0.7657,0.5253,0.605,0.8504,0.5213,0.3723,0.5279,1.5628,0.7337,0.5128,0.963,0.4205,0.7103,0.5264,0.4384,1.4154,0.4457,0.3772,1.6162,0.8743,3.6906,1.809,0.7174,0.9687,0.8893,1.2303,0.8873,1.8144,1.116,1.3276,1.2991,1.2458,1.0186,2.1076,1.2372,2.6622,0.6217,0.945,0.5288,0.6136,0.7369,0.8469,1.3241,0.9494,0.5912,0.5678,0.7537,0.7332,0.8347,1.404,1.0086,0.8693,0.9444,1.8899,1.7237,1.1663,0.9432,1.4018,0.7842,3.5587,2.4291,1.5232,1.0716,1.1894,0.6162,1.3279,0.8592,1.009,1.2619,0.7729,2.0267,2.2426,1.0762,1.452,1.346,2.2397,2.1081,1.6794,1.2861,4.2952,2.9563,2.0925,2.2027,1.0522,0.803,1.3659,1.7794,1.0189,2.2548,2.1778,1.9621,1.0396,1.4778,1.5363,0.7938,0.6799,1.1604,2.1864,0.9119,0.8873,1.3662,1.0248,1.3478,0.8567,0.9916,0.8439,2.8251,1.0091,1.2657,2.4727,3.5052,3.6422,2.3305,1.6641,2.3916,3.947,4.989,2.0162,1.8653,1.246,1.1458,0.982,1.5805,1.6189];

pub struct VoiceDspSIG0 {
	fVoiceDspSIG0Wave0_idx: i32,
}

impl VoiceDspSIG0 {
	
	fn get_num_inputsVoiceDspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsVoiceDspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initVoiceDspSIG0(&mut self, sample_rate: i32) {
		self.fVoiceDspSIG0Wave0_idx = 0;
	}
	
	fn fillVoiceDspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			table[i1 as usize] = unsafe { fVoiceDspSIG0Wave0[self.fVoiceDspSIG0Wave0_idx as usize] };
			self.fVoiceDspSIG0Wave0_idx = (i32::wrapping_add(1, self.fVoiceDspSIG0Wave0_idx)) % 164;
		}
	}

}


pub fn newVoiceDspSIG0() -> VoiceDspSIG0 { 
	VoiceDspSIG0 {
		fVoiceDspSIG0Wave0_idx: 0,
	}
}
fn VoiceDsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}
fn VoiceDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut ftbl0VoiceDspSIG0: [F32;164] = [0.0;164];

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct VoiceDsp {
	fButton0: F32,
	fVec0: [F32;2],
	fSampleRate: i32,
	fConst1: F32,
	fHslider0: F32,
	fRec0: [F32;2],
	fHslider1: F32,
	fRec1: [F32;2],
	fHslider2: F32,
	fRec2: [F32;2],
	fHslider3: F32,
	fRec3: [F32;2],
	iRec4: [i32;2],
	fConst2: F32,
	fHslider4: F32,
	fRec5: [F32;2],
	fHslider5: F32,
	fRec6: [F32;2],
	fHslider6: F32,
	fRec7: [F32;2],
	iRec8: [i32;2],
	fHslider7: F32,
	fRec9: [F32;2],
	fHslider8: F32,
	fHslider9: F32,
	fRec10: [F32;2],
	fHslider10: F32,
	fRec11: [F32;2],
	fConst3: F32,
	fHslider11: F32,
	fRec12: [F32;2],
	fHslider12: F32,
	fRec13: [F32;2],
	fHslider13: F32,
	fRec14: [F32;2],
	fHslider14: F32,
	fRec15: [F32;2],
	fHslider15: F32,
	fRec16: [F32;2],
	fHslider16: F32,
	fRec17: [F32;2],
	fHslider17: F32,
	fRec18: [F32;2],
	fHslider18: F32,
	fRec19: [F32;2],
	fHslider19: F32,
	fRec20: [F32;2],
	fHslider20: F32,
	fRec21: [F32;2],
	fConst4: F32,
	fHslider21: F32,
	fRec23: [F32;2],
	fHslider22: F32,
	fRec24: [F32;2],
	fHslider23: F32,
	fRec25: [F32;2],
	fHslider24: F32,
	fRec26: [F32;2],
	fHslider25: F32,
	fRec27: [F32;2],
	fHslider26: F32,
	fRec28: [F32;2],
	fHslider27: F32,
	fRec29: [F32;2],
	fHslider28: F32,
	fRec30: [F32;2],
	fConst5: F32,
	fHslider29: F32,
	fRec31: [F32;2],
	fHslider30: F32,
	fRec32: [F32;2],
	fHslider31: F32,
	fRec33: [F32;2],
	fHslider32: F32,
	fRec34: [F32;2],
	fHslider33: F32,
	fRec35: [F32;2],
	fHslider34: F32,
	fRec36: [F32;2],
	fCheckbox0: F32,
	fHslider35: F32,
	fRec37: [F32;2],
	fHslider36: F32,
	fRec38: [F32;2],
	fHslider37: F32,
	fRec22: [F32;2],
	fVec1: [F32;2],
	fHslider38: F32,
	fRec39: [F32;2],
	fHslider39: F32,
	fHslider40: F32,
	fHslider41: F32,
	fHslider42: F32,
	fHslider43: F32,
	fHslider44: F32,
	fEntry0: F32,
	fHslider45: F32,
	fHslider46: F32,
	fConst6: F32,
	fTempPerm0: F32,
	fTempPerm1: F32,
	fHslider47: F32,
	fRec40: [F32;2],
	fHslider48: F32,
	fRec41: [F32;2],
	fTempPerm2: F32,
	fTempPerm3: F32,
	fHslider49: F32,
	fRec42: [F32;2],
	fHslider50: F32,
	fRec43: [F32;2],
	fTempPerm4: F32,
	fTempPerm5: F32,
	fTempPerm6: F32,
	fTempPerm7: F32,
	fTempPerm8: F32,
	fTempPerm9: F32,
	fTempPerm10: F32,
	fTempPerm11: F32,
	fTempPerm12: F32,
	fTempPerm13: F32,
	fTempPerm14: F32,
	fTempPerm15: F32,
	fTempPerm16: F32,
	fTempPerm17: F32,
	fTempPerm18: F32,
	fTempPerm19: F32,
	fTempPerm20: F32,
	fTempPerm21: F32,
	fTempPerm22: F32,
	fTempPerm23: F32,
	fTempPerm24: F32,
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
	fTempPerm40: F32,
	fTempPerm41: F32,
	fTempPerm42: F32,
	fTempPerm43: F32,
	fTempPerm44: F32,
	fTempPerm45: F32,
	fTempPerm46: F32,
	fTempPerm47: F32,
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	fTempPerm51: F32,
	fTempPerm52: F32,
	fTempPerm53: F32,
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
	fTempPerm65: F32,
	fTempPerm66: F32,
	fTempPerm67: F32,
	fTempPerm68: F32,
	fTempPerm69: F32,
	fTempPerm70: F32,
	fTempPerm71: F32,
	fTempPerm72: F32,
	fTempPerm73: F32,
	fTempPerm74: F32,
	fTempPerm75: F32,
	fTempPerm76: F32,
	fTempPerm77: F32,
	fTempPerm78: F32,
	fTempPerm79: F32,
	fRec44: [F32;2],
	fRec45: [F32;2],
	fTempPerm80: F32,
	fTempPerm81: F32,
	fTempPerm82: F32,
	fTempPerm83: F32,
	fRec47: [F32;2],
	fRec48: [F32;2],
	fTempPerm84: F32,
	fRec49: [F32;2],
	fTempPerm85: F32,
	fVec2: [F32;2],
	fTempPerm86: F32,
	fTempPerm87: F32,
	fTempPerm88: F32,
	fRec46: [F32;2],
	fTempPerm89: F32,
	fHslider51: F32,
	fRec50: [F32;2],
	fHslider52: F32,
	fRec51: [F32;2],
	fTempPerm90: F32,
	fTempPerm91: F32,
	fTempPerm92: F32,
	fTempPerm93: F32,
	fTempPerm94: F32,
	iTempPerm95: i32,
	fTempPerm96: F32,
	fTempPerm97: F32,
	fTempPerm98: F32,
	fRec52: [F32;2],
	fTempPerm99: F32,
	fTempPerm100: F32,
	fRec53: [F32;2],
	fTempPerm101: F32,
	fTempPerm102: F32,
	fTempPerm103: F32,
	iRec55: [i32;2],
	fRec54: [F32;4],
	fRec56: [F32;2],
	fRec57: [F32;2],
	fTempPerm104: F32,
	fTempPerm105: F32,
	fRec61: [F32;2],
	fRec62: [F32;2],
	fConst7: F32,
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
	fTempPerm123: F32,
	fTempPerm124: F32,
	fTempPerm125: F32,
	fTempPerm126: F32,
	fTempPerm127: F32,
	fTempPerm128: F32,
	fTempPerm129: F32,
	fTempPerm130: F32,
	fTempPerm131: F32,
	fTempPerm132: F32,
	fTempPerm133: F32,
	fTempPerm134: F32,
	fTempPerm135: F32,
	fTempPerm136: F32,
	fTempPerm137: F32,
	fTempPerm138: F32,
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
	fRec58: [F32;2],
	fTempPerm156: F32,
	fRec59: [F32;2],
	fTempPerm157: F32,
	fTempPerm158: F32,
	fHslider53: F32,
	fRec63: [F32;2],
	fHslider54: F32,
	fRec64: [F32;2],
	fHslider55: F32,
	fRec65: [F32;2],
	fHslider56: F32,
	fRec67: [F32;2],
	fHslider57: F32,
	fRec68: [F32;2],
	fEntry1: F32,
	fEntry2: F32,
	fHslider58: F32,
	fRec69: [F32;2],
	fHslider59: F32,
	fRec70: [F32;2],
	fConst8: F32,
	fVec3: [F32;2],
	fRec66: [F32;2],
	fHslider60: F32,
	fRec71: [F32;2],
	fHslider61: F32,
	fRec72: [F32;2],
	fHslider62: F32,
	fRec73: [F32;2],
	fHslider63: F32,
	fRec75: [F32;2],
	fHslider64: F32,
	fRec76: [F32;2],
	fEntry3: F32,
	fEntry4: F32,
	fHslider65: F32,
	fRec77: [F32;2],
	fHslider66: F32,
	fRec78: [F32;2],
	fVec4: [F32;2],
	fRec74: [F32;2],
	fHslider67: F32,
	fRec79: [F32;2],
	fHslider68: F32,
	fRec80: [F32;2],
	fHslider69: F32,
	fRec81: [F32;2],
	fHslider70: F32,
	fRec82: [F32;2],
	fHslider71: F32,
	fRec83: [F32;2],
	fHslider72: F32,
	fRec84: [F32;2],
	fConst9: F32,
	fConst10: F32,
	fHslider73: F32,
	fRec85: [F32;2],
	fHslider74: F32,
	fRec86: [F32;2],
	fHslider75: F32,
	fRec87: [F32;2],
	fHslider76: F32,
	fRec88: [F32;2],
	fHslider77: F32,
	fRec89: [F32;2],
	fHslider78: F32,
	fRec90: [F32;2],
	fHslider79: F32,
	fRec91: [F32;2],
	fHslider80: F32,
	fRec92: [F32;2],
	fHslider81: F32,
	fRec93: [F32;2],
	fHslider82: F32,
	fRec94: [F32;2],
	fHslider83: F32,
	fRec95: [F32;2],
	fHslider84: F32,
	fRec96: [F32;2],
	fHslider85: F32,
	fRec97: [F32;2],
	fHslider86: F32,
	fRec98: [F32;2],
	fConst11: F32,
	fHslider87: F32,
	fRec99: [F32;2],
	fHslider88: F32,
	fRec100: [F32;2],
	fHslider89: F32,
	fRec101: [F32;2],
	fHslider90: F32,
	fRec102: [F32;2],
	fHslider91: F32,
	fRec103: [F32;2],
	fHslider92: F32,
	fRec104: [F32;2],
	fHslider93: F32,
	fRec105: [F32;2],
	fHslider94: F32,
	fRec106: [F32;2],
	fHslider95: F32,
	fRec107: [F32;2],
	fHslider96: F32,
	fRec108: [F32;2],
	fHslider97: F32,
	fRec109: [F32;2],
	fHslider98: F32,
	fRec110: [F32;2],
	fHslider99: F32,
	fRec111: [F32;2],
	fHslider100: F32,
	fRec112: [F32;2],
	fEntry5: F32,
	fHslider101: F32,
	fRec113: [F32;2],
	fHslider102: F32,
	fRec114: [F32;2],
	fHslider103: F32,
	fRec115: [F32;2],
	fHslider104: F32,
	fRec116: [F32;2],
	fEntry6: F32,
	fHslider105: F32,
	fRec117: [F32;2],
	fHslider106: F32,
	fRec118: [F32;2],
	fHslider107: F32,
	fRec119: [F32;2],
	fHslider108: F32,
	fRec120: [F32;2],
	fHslider109: F32,
	fRec121: [F32;2],
	fHslider110: F32,
	fRec122: [F32;2],
	fHslider111: F32,
	fRec123: [F32;2],
	fHslider112: F32,
	fRec124: [F32;2],
	fHslider113: F32,
	fRec125: [F32;2],
	fHslider114: F32,
	fRec126: [F32;2],
	fHslider115: F32,
	fRec127: [F32;2],
	fHslider116: F32,
	fRec128: [F32;2],
	fHslider117: F32,
	fRec129: [F32;2],
	fHslider118: F32,
	fRec130: [F32;2],
	fHslider119: F32,
	fRec131: [F32;2],
	fHslider120: F32,
	fRec132: [F32;2],
	fHslider121: F32,
	fRec133: [F32;2],
	fHslider122: F32,
	fRec134: [F32;2],
	fEntry7: F32,
}

impl FaustDsp for VoiceDsp {
	type T = F32;
		
	fn new() -> VoiceDsp { 
		VoiceDsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			fSampleRate: 0,
			fConst1: 0.0,
			fHslider0: 0.0,
			fRec0: [0.0;2],
			fHslider1: 0.0,
			fRec1: [0.0;2],
			fHslider2: 0.0,
			fRec2: [0.0;2],
			fHslider3: 0.0,
			fRec3: [0.0;2],
			iRec4: [0;2],
			fConst2: 0.0,
			fHslider4: 0.0,
			fRec5: [0.0;2],
			fHslider5: 0.0,
			fRec6: [0.0;2],
			fHslider6: 0.0,
			fRec7: [0.0;2],
			iRec8: [0;2],
			fHslider7: 0.0,
			fRec9: [0.0;2],
			fHslider8: 0.0,
			fHslider9: 0.0,
			fRec10: [0.0;2],
			fHslider10: 0.0,
			fRec11: [0.0;2],
			fConst3: 0.0,
			fHslider11: 0.0,
			fRec12: [0.0;2],
			fHslider12: 0.0,
			fRec13: [0.0;2],
			fHslider13: 0.0,
			fRec14: [0.0;2],
			fHslider14: 0.0,
			fRec15: [0.0;2],
			fHslider15: 0.0,
			fRec16: [0.0;2],
			fHslider16: 0.0,
			fRec17: [0.0;2],
			fHslider17: 0.0,
			fRec18: [0.0;2],
			fHslider18: 0.0,
			fRec19: [0.0;2],
			fHslider19: 0.0,
			fRec20: [0.0;2],
			fHslider20: 0.0,
			fRec21: [0.0;2],
			fConst4: 0.0,
			fHslider21: 0.0,
			fRec23: [0.0;2],
			fHslider22: 0.0,
			fRec24: [0.0;2],
			fHslider23: 0.0,
			fRec25: [0.0;2],
			fHslider24: 0.0,
			fRec26: [0.0;2],
			fHslider25: 0.0,
			fRec27: [0.0;2],
			fHslider26: 0.0,
			fRec28: [0.0;2],
			fHslider27: 0.0,
			fRec29: [0.0;2],
			fHslider28: 0.0,
			fRec30: [0.0;2],
			fConst5: 0.0,
			fHslider29: 0.0,
			fRec31: [0.0;2],
			fHslider30: 0.0,
			fRec32: [0.0;2],
			fHslider31: 0.0,
			fRec33: [0.0;2],
			fHslider32: 0.0,
			fRec34: [0.0;2],
			fHslider33: 0.0,
			fRec35: [0.0;2],
			fHslider34: 0.0,
			fRec36: [0.0;2],
			fCheckbox0: 0.0,
			fHslider35: 0.0,
			fRec37: [0.0;2],
			fHslider36: 0.0,
			fRec38: [0.0;2],
			fHslider37: 0.0,
			fRec22: [0.0;2],
			fVec1: [0.0;2],
			fHslider38: 0.0,
			fRec39: [0.0;2],
			fHslider39: 0.0,
			fHslider40: 0.0,
			fHslider41: 0.0,
			fHslider42: 0.0,
			fHslider43: 0.0,
			fHslider44: 0.0,
			fEntry0: 0.0,
			fHslider45: 0.0,
			fHslider46: 0.0,
			fConst6: 0.0,
			fTempPerm0: 0.0,
			fTempPerm1: 0.0,
			fHslider47: 0.0,
			fRec40: [0.0;2],
			fHslider48: 0.0,
			fRec41: [0.0;2],
			fTempPerm2: 0.0,
			fTempPerm3: 0.0,
			fHslider49: 0.0,
			fRec42: [0.0;2],
			fHslider50: 0.0,
			fRec43: [0.0;2],
			fTempPerm4: 0.0,
			fTempPerm5: 0.0,
			fTempPerm6: 0.0,
			fTempPerm7: 0.0,
			fTempPerm8: 0.0,
			fTempPerm9: 0.0,
			fTempPerm10: 0.0,
			fTempPerm11: 0.0,
			fTempPerm12: 0.0,
			fTempPerm13: 0.0,
			fTempPerm14: 0.0,
			fTempPerm15: 0.0,
			fTempPerm16: 0.0,
			fTempPerm17: 0.0,
			fTempPerm18: 0.0,
			fTempPerm19: 0.0,
			fTempPerm20: 0.0,
			fTempPerm21: 0.0,
			fTempPerm22: 0.0,
			fTempPerm23: 0.0,
			fTempPerm24: 0.0,
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
			fTempPerm40: 0.0,
			fTempPerm41: 0.0,
			fTempPerm42: 0.0,
			fTempPerm43: 0.0,
			fTempPerm44: 0.0,
			fTempPerm45: 0.0,
			fTempPerm46: 0.0,
			fTempPerm47: 0.0,
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
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
			fTempPerm65: 0.0,
			fTempPerm66: 0.0,
			fTempPerm67: 0.0,
			fTempPerm68: 0.0,
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			fTempPerm71: 0.0,
			fTempPerm72: 0.0,
			fTempPerm73: 0.0,
			fTempPerm74: 0.0,
			fTempPerm75: 0.0,
			fTempPerm76: 0.0,
			fTempPerm77: 0.0,
			fTempPerm78: 0.0,
			fTempPerm79: 0.0,
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			fTempPerm80: 0.0,
			fTempPerm81: 0.0,
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			fRec47: [0.0;2],
			fRec48: [0.0;2],
			fTempPerm84: 0.0,
			fRec49: [0.0;2],
			fTempPerm85: 0.0,
			fVec2: [0.0;2],
			fTempPerm86: 0.0,
			fTempPerm87: 0.0,
			fTempPerm88: 0.0,
			fRec46: [0.0;2],
			fTempPerm89: 0.0,
			fHslider51: 0.0,
			fRec50: [0.0;2],
			fHslider52: 0.0,
			fRec51: [0.0;2],
			fTempPerm90: 0.0,
			fTempPerm91: 0.0,
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fTempPerm94: 0.0,
			iTempPerm95: 0,
			fTempPerm96: 0.0,
			fTempPerm97: 0.0,
			fTempPerm98: 0.0,
			fRec52: [0.0;2],
			fTempPerm99: 0.0,
			fTempPerm100: 0.0,
			fRec53: [0.0;2],
			fTempPerm101: 0.0,
			fTempPerm102: 0.0,
			fTempPerm103: 0.0,
			iRec55: [0;2],
			fRec54: [0.0;4],
			fRec56: [0.0;2],
			fRec57: [0.0;2],
			fTempPerm104: 0.0,
			fTempPerm105: 0.0,
			fRec61: [0.0;2],
			fRec62: [0.0;2],
			fConst7: 0.0,
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
			fTempPerm123: 0.0,
			fTempPerm124: 0.0,
			fTempPerm125: 0.0,
			fTempPerm126: 0.0,
			fTempPerm127: 0.0,
			fTempPerm128: 0.0,
			fTempPerm129: 0.0,
			fTempPerm130: 0.0,
			fTempPerm131: 0.0,
			fTempPerm132: 0.0,
			fTempPerm133: 0.0,
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			fTempPerm136: 0.0,
			fTempPerm137: 0.0,
			fTempPerm138: 0.0,
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
			fRec58: [0.0;2],
			fTempPerm156: 0.0,
			fRec59: [0.0;2],
			fTempPerm157: 0.0,
			fTempPerm158: 0.0,
			fHslider53: 0.0,
			fRec63: [0.0;2],
			fHslider54: 0.0,
			fRec64: [0.0;2],
			fHslider55: 0.0,
			fRec65: [0.0;2],
			fHslider56: 0.0,
			fRec67: [0.0;2],
			fHslider57: 0.0,
			fRec68: [0.0;2],
			fEntry1: 0.0,
			fEntry2: 0.0,
			fHslider58: 0.0,
			fRec69: [0.0;2],
			fHslider59: 0.0,
			fRec70: [0.0;2],
			fConst8: 0.0,
			fVec3: [0.0;2],
			fRec66: [0.0;2],
			fHslider60: 0.0,
			fRec71: [0.0;2],
			fHslider61: 0.0,
			fRec72: [0.0;2],
			fHslider62: 0.0,
			fRec73: [0.0;2],
			fHslider63: 0.0,
			fRec75: [0.0;2],
			fHslider64: 0.0,
			fRec76: [0.0;2],
			fEntry3: 0.0,
			fEntry4: 0.0,
			fHslider65: 0.0,
			fRec77: [0.0;2],
			fHslider66: 0.0,
			fRec78: [0.0;2],
			fVec4: [0.0;2],
			fRec74: [0.0;2],
			fHslider67: 0.0,
			fRec79: [0.0;2],
			fHslider68: 0.0,
			fRec80: [0.0;2],
			fHslider69: 0.0,
			fRec81: [0.0;2],
			fHslider70: 0.0,
			fRec82: [0.0;2],
			fHslider71: 0.0,
			fRec83: [0.0;2],
			fHslider72: 0.0,
			fRec84: [0.0;2],
			fConst9: 0.0,
			fConst10: 0.0,
			fHslider73: 0.0,
			fRec85: [0.0;2],
			fHslider74: 0.0,
			fRec86: [0.0;2],
			fHslider75: 0.0,
			fRec87: [0.0;2],
			fHslider76: 0.0,
			fRec88: [0.0;2],
			fHslider77: 0.0,
			fRec89: [0.0;2],
			fHslider78: 0.0,
			fRec90: [0.0;2],
			fHslider79: 0.0,
			fRec91: [0.0;2],
			fHslider80: 0.0,
			fRec92: [0.0;2],
			fHslider81: 0.0,
			fRec93: [0.0;2],
			fHslider82: 0.0,
			fRec94: [0.0;2],
			fHslider83: 0.0,
			fRec95: [0.0;2],
			fHslider84: 0.0,
			fRec96: [0.0;2],
			fHslider85: 0.0,
			fRec97: [0.0;2],
			fHslider86: 0.0,
			fRec98: [0.0;2],
			fConst11: 0.0,
			fHslider87: 0.0,
			fRec99: [0.0;2],
			fHslider88: 0.0,
			fRec100: [0.0;2],
			fHslider89: 0.0,
			fRec101: [0.0;2],
			fHslider90: 0.0,
			fRec102: [0.0;2],
			fHslider91: 0.0,
			fRec103: [0.0;2],
			fHslider92: 0.0,
			fRec104: [0.0;2],
			fHslider93: 0.0,
			fRec105: [0.0;2],
			fHslider94: 0.0,
			fRec106: [0.0;2],
			fHslider95: 0.0,
			fRec107: [0.0;2],
			fHslider96: 0.0,
			fRec108: [0.0;2],
			fHslider97: 0.0,
			fRec109: [0.0;2],
			fHslider98: 0.0,
			fRec110: [0.0;2],
			fHslider99: 0.0,
			fRec111: [0.0;2],
			fHslider100: 0.0,
			fRec112: [0.0;2],
			fEntry5: 0.0,
			fHslider101: 0.0,
			fRec113: [0.0;2],
			fHslider102: 0.0,
			fRec114: [0.0;2],
			fHslider103: 0.0,
			fRec115: [0.0;2],
			fHslider104: 0.0,
			fRec116: [0.0;2],
			fEntry6: 0.0,
			fHslider105: 0.0,
			fRec117: [0.0;2],
			fHslider106: 0.0,
			fRec118: [0.0;2],
			fHslider107: 0.0,
			fRec119: [0.0;2],
			fHslider108: 0.0,
			fRec120: [0.0;2],
			fHslider109: 0.0,
			fRec121: [0.0;2],
			fHslider110: 0.0,
			fRec122: [0.0;2],
			fHslider111: 0.0,
			fRec123: [0.0;2],
			fHslider112: 0.0,
			fRec124: [0.0;2],
			fHslider113: 0.0,
			fRec125: [0.0;2],
			fHslider114: 0.0,
			fRec126: [0.0;2],
			fHslider115: 0.0,
			fRec127: [0.0;2],
			fHslider116: 0.0,
			fRec128: [0.0;2],
			fHslider117: 0.0,
			fRec129: [0.0;2],
			fHslider118: 0.0,
			fRec130: [0.0;2],
			fHslider119: 0.0,
			fRec131: [0.0;2],
			fHslider120: 0.0,
			fRec132: [0.0;2],
			fHslider121: 0.0,
			fRec133: [0.0;2],
			fHslider122: 0.0,
			fRec134: [0.0;2],
			fEntry7: 0.0,
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
		m.declare("drum.lib/name", r"abdrum-layers");
		m.declare("drum.lib/version", r"0.1");
		m.declare("filename", r"plugin-voice.dsp");
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
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/pole:author", r"Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/svf:author", r"Oleg Nesterov");
		m.declare("filters.lib/svf:copyright", r"Copyright (C) 2020 Oleg Nesterov <oleg@redhat.com>");
		m.declare("filters.lib/svf:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("filters.lib/zero:author", r"Julius O. Smith III");
		m.declare("filters.lib/zero:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/zero:license", r"MIT-style STK-4.3 license");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"ABDrum");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("routes.lib/name", r"Faust Signal Routing Library");
		m.declare("routes.lib/version", r"1.2.0");
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
		return 28;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: VoiceDspSIG0 = newVoiceDspSIG0();
		sig0.instance_initVoiceDspSIG0(sample_rate);
		sig0.fillVoiceDspSIG0(164, unsafe { &mut ftbl0VoiceDspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.383;
		self.fHslider2 = 3e+02;
		self.fHslider3 = 0.0;
		self.fHslider4 = 0.5;
		self.fHslider5 = 0.8;
		self.fHslider6 = 0.5;
		self.fHslider7 = 0.25;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 7e+02;
		self.fHslider11 = 0.0;
		self.fHslider12 = 0.3;
		self.fHslider13 = 0.0;
		self.fHslider14 = 0.6;
		self.fHslider15 = 0.0;
		self.fHslider16 = 0.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 0.2;
		self.fHslider19 = 0.0;
		self.fHslider20 = 0.0;
		self.fHslider21 = 0.0;
		self.fHslider22 = 0.0;
		self.fHslider23 = 0.0;
		self.fHslider24 = 0.0;
		self.fHslider25 = 0.0;
		self.fHslider26 = 0.4;
		self.fHslider27 = 0.0;
		self.fHslider28 = 8e+01;
		self.fHslider29 = 0.0;
		self.fHslider30 = 0.333;
		self.fHslider31 = 0.0;
		self.fHslider32 = 4.0;
		self.fHslider33 = 0.0;
		self.fHslider34 = 36.0;
		self.fCheckbox0 = 0.0;
		self.fHslider35 = 0.0;
		self.fHslider36 = 5e+01;
		self.fHslider37 = 261.6256;
		self.fHslider38 = 0.8;
		self.fHslider39 = 0.0;
		self.fHslider40 = 0.0;
		self.fHslider41 = 0.0;
		self.fHslider42 = 0.0;
		self.fHslider43 = 0.0;
		self.fHslider44 = 0.0;
		self.fEntry0 = 0.0;
		self.fHslider45 = 0.0;
		self.fHslider46 = 0.0;
		self.fHslider47 = 0.0;
		self.fHslider48 = 0.3;
		self.fHslider49 = 0.0;
		self.fHslider50 = 2.0;
		self.fHslider51 = 0.0;
		self.fHslider52 = 0.0;
		self.fHslider53 = 0.0;
		self.fHslider54 = 0.6;
		self.fHslider55 = 0.0;
		self.fHslider56 = 0.0;
		self.fHslider57 = 2e+03;
		self.fEntry1 = 0.0;
		self.fEntry2 = 1.0;
		self.fHslider58 = 0.0;
		self.fHslider59 = 0.0;
		self.fHslider60 = 0.0;
		self.fHslider61 = 0.6;
		self.fHslider62 = 0.0;
		self.fHslider63 = 0.0;
		self.fHslider64 = 2e+03;
		self.fEntry3 = 2.0;
		self.fEntry4 = 1.0;
		self.fHslider65 = 0.0;
		self.fHslider66 = 0.0;
		self.fHslider67 = 0.0;
		self.fHslider68 = 0.0;
		self.fHslider69 = 0.0;
		self.fHslider70 = 0.0;
		self.fHslider71 = 0.0;
		self.fHslider72 = 0.5;
		self.fHslider73 = 0.0;
		self.fHslider74 = 0.0;
		self.fHslider75 = 0.0;
		self.fHslider76 = 0.0;
		self.fHslider77 = 2.5e+02;
		self.fHslider78 = 0.0;
		self.fHslider79 = 0.2;
		self.fHslider80 = 0.0;
		self.fHslider81 = 0.31;
		self.fHslider82 = 0.0;
		self.fHslider83 = 0.0;
		self.fHslider84 = 0.0;
		self.fHslider85 = 0.6;
		self.fHslider86 = 0.0;
		self.fHslider87 = 0.0;
		self.fHslider88 = 0.0;
		self.fHslider89 = 0.0;
		self.fHslider90 = 0.3;
		self.fHslider91 = 0.0;
		self.fHslider92 = 0.0;
		self.fHslider93 = 0.0;
		self.fHslider94 = 0.2;
		self.fHslider95 = 0.0;
		self.fHslider96 = 4e+02;
		self.fHslider97 = 0.0;
		self.fHslider98 = 0.4;
		self.fHslider99 = 0.0;
		self.fHslider100 = 0.5;
		self.fEntry5 = 0.0;
		self.fHslider101 = 0.0;
		self.fHslider102 = 0.2;
		self.fHslider103 = 0.0;
		self.fHslider104 = 2.5e+03;
		self.fEntry6 = 0.0;
		self.fHslider105 = 0.0;
		self.fHslider106 = 0.0;
		self.fHslider107 = 0.0;
		self.fHslider108 = 0.0;
		self.fHslider109 = 0.0;
		self.fHslider110 = 0.0;
		self.fHslider111 = 0.0;
		self.fHslider112 = 1.0;
		self.fHslider113 = 0.0;
		self.fHslider114 = 0.3;
		self.fHslider115 = 0.0;
		self.fHslider116 = 3e+02;
		self.fHslider117 = 0.0;
		self.fHslider118 = 0.0;
		self.fHslider119 = 0.0;
		self.fHslider120 = 0.0;
		self.fHslider121 = 0.0;
		self.fHslider122 = 0.0;
		self.fEntry7 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec0[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec1[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec2[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec3[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.iRec4[l5 as usize] = 0;
		}
		for l6 in 0..2 {
			self.fRec5[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec6[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec7[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.iRec8[l9 as usize] = 0;
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
		for l13 in 0..2 {
			self.fRec12[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec13[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec14[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec15[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec16[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec17[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec18[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec19[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec20[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec21[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec23[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec24[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec25[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec26[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec27[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec28[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec29[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec30[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec31[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec32[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec33[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec34[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec35[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec36[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec37[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec38[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec22[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec1[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec39[l41 as usize] = 0.0;
		}
		self.fTempPerm0 = 0.0;
		self.fTempPerm1 = 0.0;
		for l42 in 0..2 {
			self.fRec40[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec41[l43 as usize] = 0.0;
		}
		self.fTempPerm2 = 0.0;
		self.fTempPerm3 = 0.0;
		for l44 in 0..2 {
			self.fRec42[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec43[l45 as usize] = 0.0;
		}
		self.fTempPerm4 = 0.0;
		self.fTempPerm5 = 0.0;
		self.fTempPerm6 = 0.0;
		self.fTempPerm7 = 0.0;
		self.fTempPerm8 = 0.0;
		self.fTempPerm9 = 0.0;
		self.fTempPerm10 = 0.0;
		self.fTempPerm11 = 0.0;
		self.fTempPerm12 = 0.0;
		self.fTempPerm13 = 0.0;
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
		self.fTempPerm24 = 0.0;
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
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
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
		self.fTempPerm67 = 0.0;
		self.fTempPerm68 = 0.0;
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		self.fTempPerm71 = 0.0;
		self.fTempPerm72 = 0.0;
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		self.fTempPerm75 = 0.0;
		self.fTempPerm76 = 0.0;
		self.fTempPerm77 = 0.0;
		self.fTempPerm78 = 0.0;
		self.fTempPerm79 = 0.0;
		for l46 in 0..2 {
			self.fRec44[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec45[l47 as usize] = 0.0;
		}
		self.fTempPerm80 = 0.0;
		self.fTempPerm81 = 0.0;
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		for l48 in 0..2 {
			self.fRec47[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec48[l49 as usize] = 0.0;
		}
		self.fTempPerm84 = 0.0;
		for l50 in 0..2 {
			self.fRec49[l50 as usize] = 0.0;
		}
		self.fTempPerm85 = 0.0;
		for l51 in 0..2 {
			self.fVec2[l51 as usize] = 0.0;
		}
		self.fTempPerm86 = 0.0;
		self.fTempPerm87 = 0.0;
		self.fTempPerm88 = 0.0;
		for l52 in 0..2 {
			self.fRec46[l52 as usize] = 0.0;
		}
		self.fTempPerm89 = 0.0;
		for l53 in 0..2 {
			self.fRec50[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec51[l54 as usize] = 0.0;
		}
		self.fTempPerm90 = 0.0;
		self.fTempPerm91 = 0.0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		self.fTempPerm94 = 0.0;
		self.iTempPerm95 = 0;
		self.fTempPerm96 = 0.0;
		self.fTempPerm97 = 0.0;
		self.fTempPerm98 = 0.0;
		for l55 in 0..2 {
			self.fRec52[l55 as usize] = 0.0;
		}
		self.fTempPerm99 = 0.0;
		self.fTempPerm100 = 0.0;
		for l56 in 0..2 {
			self.fRec53[l56 as usize] = 0.0;
		}
		self.fTempPerm101 = 0.0;
		self.fTempPerm102 = 0.0;
		self.fTempPerm103 = 0.0;
		for l57 in 0..2 {
			self.iRec55[l57 as usize] = 0;
		}
		for l58 in 0..4 {
			self.fRec54[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec56[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec57[l60 as usize] = 0.0;
		}
		self.fTempPerm104 = 0.0;
		self.fTempPerm105 = 0.0;
		for l61 in 0..2 {
			self.fRec61[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec62[l62 as usize] = 0.0;
		}
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
		self.fTempPerm123 = 0.0;
		self.fTempPerm124 = 0.0;
		self.fTempPerm125 = 0.0;
		self.fTempPerm126 = 0.0;
		self.fTempPerm127 = 0.0;
		self.fTempPerm128 = 0.0;
		self.fTempPerm129 = 0.0;
		self.fTempPerm130 = 0.0;
		self.fTempPerm131 = 0.0;
		self.fTempPerm132 = 0.0;
		self.fTempPerm133 = 0.0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		self.fTempPerm136 = 0.0;
		self.fTempPerm137 = 0.0;
		self.fTempPerm138 = 0.0;
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
		for l63 in 0..2 {
			self.fRec58[l63 as usize] = 0.0;
		}
		self.fTempPerm156 = 0.0;
		for l64 in 0..2 {
			self.fRec59[l64 as usize] = 0.0;
		}
		self.fTempPerm157 = 0.0;
		self.fTempPerm158 = 0.0;
		for l65 in 0..2 {
			self.fRec63[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec64[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec65[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec67[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec68[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec69[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec70[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec3[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec66[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec71[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec72[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec73[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec75[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec76[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec77[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec78[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fVec4[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec74[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec79[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec80[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec81[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec82[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec83[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec84[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec85[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec86[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec87[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec88[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec89[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec90[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec91[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec92[l96 as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec93[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec94[l98 as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec95[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec96[l100 as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fRec97[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec98[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec99[l103 as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec100[l104 as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec101[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec102[l106 as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec103[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec104[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec105[l109 as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec106[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec107[l111 as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec108[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec109[l113 as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec110[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec111[l115 as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec112[l116 as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec113[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec114[l118 as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec115[l119 as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec116[l120 as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec117[l121 as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec118[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec119[l123 as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.fRec120[l124 as usize] = 0.0;
		}
		for l125 in 0..2 {
			self.fRec121[l125 as usize] = 0.0;
		}
		for l126 in 0..2 {
			self.fRec122[l126 as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fRec123[l127 as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec124[l128 as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fRec125[l129 as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.fRec126[l130 as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.fRec127[l131 as usize] = 0.0;
		}
		for l132 in 0..2 {
			self.fRec128[l132 as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fRec129[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec130[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec131[l135 as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec132[l136 as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec133[l137 as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec134[l138 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 - 44.1 / fConst0;
		self.fConst2 = 1.0 / fConst0;
		self.fConst3 = 6.907755 / fConst0;
		self.fConst4 = 0.8 * fConst0;
		self.fConst5 = 3.0 / fConst0;
		self.fConst6 = 0.45 * fConst0;
		self.fConst7 = 3.1415927 / fConst0;
		self.fConst8 = 4.41e+04 / fConst0;
		self.fConst9 = 628.31854 / fConst0;
		self.fConst10 = 314.15927 / fConst0;
		self.fConst11 = 33.333332 / fConst0;
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
		ui_interface.open_vertical_box("ABDrum");
		ui_interface.declare(None, "0", "");
		ui_interface.open_horizontal_box("Voice");
		ui_interface.declare(Some(ParamIndex(0)), "1", "");
		ui_interface.declare(Some(ParamIndex(0)), "tooltip", "Hit-to-hit variation: pitch, decays, strike position, shape, level");
		ui_interface.add_horizontal_slider("Variation", ParamIndex(0), 0.25, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("Tone");
		ui_interface.declare(Some(ParamIndex(1)), "0", "");
		ui_interface.declare(Some(ParamIndex(1)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(1), 0.8, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "10", "");
		ui_interface.declare(Some(ParamIndex(2)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Snap Mod", ParamIndex(2), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(3)), "10", "");
		ui_interface.declare(Some(ParamIndex(3)), "tooltip", "How much quicker the fast decay is: 0 the same, 1 a hundredth");
		ui_interface.add_horizontal_slider("Snap", ParamIndex(3), 0.383, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "11", "");
		ui_interface.add_horizontal_slider("Tail Level", ParamIndex(4), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(5)), "11", "");
		ui_interface.declare(Some(ParamIndex(5)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Tail Level Mod", ParamIndex(5), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "12", "");
		ui_interface.add_horizontal_slider("Vel Level", ParamIndex(6), 0.6, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "12", "");
		ui_interface.declare(Some(ParamIndex(7)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Level Mod", ParamIndex(7), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "13", "");
		ui_interface.add_horizontal_slider("Vel Pitch", ParamIndex(8), 0.4, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "13", "");
		ui_interface.declare(Some(ParamIndex(9)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Pitch Mod", ParamIndex(9), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "14", "");
		ui_interface.add_horizontal_slider("Vel Shape", ParamIndex(10), 0.2, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(11)), "14", "");
		ui_interface.declare(Some(ParamIndex(11)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Shape Mod", ParamIndex(11), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "15", "");
		ui_interface.declare(Some(ParamIndex(12)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Hold Mod", ParamIndex(12), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(13)), "15", "");
		ui_interface.declare(Some(ParamIndex(13)), "tooltip", "The pitch holds at the top this long before it drops");
		ui_interface.declare(Some(ParamIndex(13)), "unit", "ms");
		ui_interface.add_horizontal_slider("Hold", ParamIndex(13), 0.0, 0.0, 5e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(14)), "16", "");
		ui_interface.declare(Some(ParamIndex(14)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Time Mod", ParamIndex(14), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(15)), "16", "");
		ui_interface.declare(Some(ParamIndex(15)), "tooltip", "Softer hits drop quicker: at full, a silent hit drops in a quarter of the time");
		ui_interface.add_horizontal_slider("Vel Time", ParamIndex(15), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(16)), "17", "");
		ui_interface.declare(Some(ParamIndex(16)), "style", "menu{'FM':0;'Tangle':1;'Shame':2;'VOSIM':3;'Surf':4}");
		ui_interface.add_num_entry("Osc 2", ParamIndex(16), 0.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(17)), "18", "");
		ui_interface.declare(Some(ParamIndex(17)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Ratio Mod", ParamIndex(17), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(18)), "18", "");
		ui_interface.declare(Some(ParamIndex(18)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(18)), "tooltip", "Osc 2's frequency, as a multiple of Tone's (it follows the drop)");
		ui_interface.add_horizontal_slider("Ratio", ParamIndex(18), 2.0, 0.25, 16.0, 0.0001);
		ui_interface.declare(Some(ParamIndex(19)), "19", "");
		ui_interface.declare(Some(ParamIndex(19)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(19), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "19", "");
		ui_interface.declare(Some(ParamIndex(20)), "tooltip", "FM: how deep Osc 2 modulates Tone; the others: how much of their sound replaces Tone's");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(20), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "1", "");
		ui_interface.declare(Some(ParamIndex(21)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Pitch Mod", ParamIndex(21), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(22)), "1", "");
		ui_interface.declare(Some(ParamIndex(22)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(22)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(22), 5e+01, 2e+01, 2e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(23)), "20", "");
		ui_interface.declare(Some(ParamIndex(23)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Feedback Mod", ParamIndex(23), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "20", "");
		ui_interface.declare(Some(ParamIndex(24)), "tooltip", "FM only: Tone modulates itself; at the top, metallic noise");
		ui_interface.add_horizontal_slider("Feedback", ParamIndex(24), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(25)), "21", "");
		ui_interface.declare(Some(ParamIndex(25)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Morph Mod", ParamIndex(25), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(26)), "21", "");
		ui_interface.declare(Some(ParamIndex(26)), "tooltip", "Tangle and Shame: walks through their ways of combining Tone and Osc 2");
		ui_interface.add_horizontal_slider("Morph", ParamIndex(26), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(27)), "22", "");
		ui_interface.declare(Some(ParamIndex(27)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Window Mod", ParamIndex(27), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(28)), "22", "");
		ui_interface.declare(Some(ParamIndex(28)), "tooltip", "VOSIM only: how fast each cycle's burst fades (a wider formant as it steepens)");
		ui_interface.add_horizontal_slider("Window", ParamIndex(28), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(29)), "23", "");
		ui_interface.declare(Some(ParamIndex(29)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Rattle Mod", ParamIndex(29), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(30)), "23", "");
		ui_interface.declare(Some(ParamIndex(30)), "tooltip", "Tone ring-modulated by pink noise: a noisy halo that rides its pitch");
		ui_interface.add_horizontal_slider("Rattle", ParamIndex(30), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "24", "");
		ui_interface.declare(Some(ParamIndex(31)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Damp Mod", ParamIndex(31), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(32)), "24", "");
		ui_interface.declare(Some(ParamIndex(32)), "tooltip", "A lowpass gate: open at the peak, closing as the hit fades (harder hits open further)");
		ui_interface.add_horizontal_slider("Damp", ParamIndex(32), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(33)), "2", "");
		ui_interface.add_check_button("Keytrack", ParamIndex(33));
		ui_interface.declare(Some(ParamIndex(34)), "3", "");
		ui_interface.add_horizontal_slider("Shape", ParamIndex(34), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(35)), "3", "");
		ui_interface.declare(Some(ParamIndex(35)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Shape Mod", ParamIndex(35), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(36)), "4", "");
		ui_interface.declare(Some(ParamIndex(36)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Drop Mod", ParamIndex(36), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(37)), "4", "");
		ui_interface.declare(Some(ParamIndex(37)), "unit", "st");
		ui_interface.add_horizontal_slider("Drop", ParamIndex(37), 36.0, 0.0, 1.2e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(38)), "5", "");
		ui_interface.declare(Some(ParamIndex(38)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Split Mod", ParamIndex(38), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(39)), "5", "");
		ui_interface.declare(Some(ParamIndex(39)), "tooltip", "Share of the Drop that is fast (the rest is slow)");
		ui_interface.add_horizontal_slider("Split", ParamIndex(39), 0.333, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(40)), "6", "");
		ui_interface.declare(Some(ParamIndex(40)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Fast Time Mod", ParamIndex(40), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(41)), "6", "");
		ui_interface.declare(Some(ParamIndex(41)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(41)), "unit", "ms");
		ui_interface.add_horizontal_slider("Fast Time", ParamIndex(41), 4.0, 1.0, 2e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(42)), "7", "");
		ui_interface.declare(Some(ParamIndex(42)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Slow Time Mod", ParamIndex(42), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(43)), "7", "");
		ui_interface.declare(Some(ParamIndex(43)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(43)), "unit", "ms");
		ui_interface.add_horizontal_slider("Slow Time", ParamIndex(43), 8e+01, 2e+01, 5e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(44)), "8", "");
		ui_interface.declare(Some(ParamIndex(44)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Attack Mod", ParamIndex(44), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(45)), "8", "");
		ui_interface.declare(Some(ParamIndex(45)), "unit", "ms");
		ui_interface.add_horizontal_slider("Attack", ParamIndex(45), 0.0, 0.0, 1e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(46)), "9", "");
		ui_interface.declare(Some(ParamIndex(46)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(46), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(47)), "9", "");
		ui_interface.declare(Some(ParamIndex(47)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(47)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(47), 7e+02, 2e+01, 4e+03, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("Body");
		ui_interface.declare(Some(ParamIndex(48)), "0", "");
		ui_interface.declare(Some(ParamIndex(48)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(48), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(49)), "10", "");
		ui_interface.declare(Some(ParamIndex(49)), "style", "menu{'Modes':0;'String':1}");
		ui_interface.add_num_entry("Model", ParamIndex(49), 0.0, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(50)), "1", "");
		ui_interface.declare(Some(ParamIndex(50)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Offset Mod", ParamIndex(50), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(51)), "1", "");
		ui_interface.declare(Some(ParamIndex(51)), "unit", "st");
		ui_interface.add_horizontal_slider("Offset", ParamIndex(51), 0.0, -24.0, 24.0, 0.01);
		ui_interface.declare(Some(ParamIndex(52)), "2", "");
		ui_interface.add_horizontal_slider("Material", ParamIndex(52), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(53)), "2", "");
		ui_interface.declare(Some(ParamIndex(53)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Material Mod", ParamIndex(53), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(54)), "3", "");
		ui_interface.declare(Some(ParamIndex(54)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Strike Mod", ParamIndex(54), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(55)), "3", "");
		ui_interface.declare(Some(ParamIndex(55)), "tooltip", "Mallet hardness, and brightness with it");
		ui_interface.add_horizontal_slider("Strike", ParamIndex(55), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(56)), "4", "");
		ui_interface.declare(Some(ParamIndex(56)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(56), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(57)), "4", "");
		ui_interface.declare(Some(ParamIndex(57)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(57)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(57), 4e+02, 2e+01, 4e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(58)), "5", "");
		ui_interface.declare(Some(ParamIndex(58)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Damping Mod", ParamIndex(58), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(59)), "5", "");
		ui_interface.declare(Some(ParamIndex(59)), "tooltip", "Above the middle the upper modes die sooner and start quieter; below, they ring brighter");
		ui_interface.add_horizontal_slider("Damping", ParamIndex(59), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(60)), "6", "");
		ui_interface.add_horizontal_slider("Vel Brightness", ParamIndex(60), 0.4, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(61)), "6", "");
		ui_interface.declare(Some(ParamIndex(61)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Brightness Mod", ParamIndex(61), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(62)), "7", "");
		ui_interface.add_horizontal_slider("Vel Bend", ParamIndex(62), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(63)), "7", "");
		ui_interface.declare(Some(ParamIndex(63)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Bend Mod", ParamIndex(63), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(64)), "8", "");
		ui_interface.add_horizontal_slider("Vel Decay", ParamIndex(64), 0.2, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(65)), "8", "");
		ui_interface.declare(Some(ParamIndex(65)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Decay Mod", ParamIndex(65), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(66)), "9", "");
		ui_interface.declare(Some(ParamIndex(66)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Clip Mod", ParamIndex(66), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(67)), "9", "");
		ui_interface.declare(Some(ParamIndex(67)), "tooltip", "Clips each mode's ring inside the resonator, harder the harder the hit: loud starts squash and ring lopsided");
		ui_interface.add_horizontal_slider("Clip", ParamIndex(67), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("Noise");
		ui_interface.declare(Some(ParamIndex(68)), "0", "");
		ui_interface.declare(Some(ParamIndex(68)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(68), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(69)), "10", "");
		ui_interface.declare(Some(ParamIndex(69)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Snap Mod", ParamIndex(69), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(70)), "10", "");
		ui_interface.declare(Some(ParamIndex(70)), "tooltip", "How much quicker the fast decay is: 0 the same, 1 a hundredth");
		ui_interface.add_horizontal_slider("Snap", ParamIndex(70), 0.31, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(71)), "11", "");
		ui_interface.add_horizontal_slider("Tail Level", ParamIndex(71), 0.2, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(72)), "11", "");
		ui_interface.declare(Some(ParamIndex(72)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Tail Level Mod", ParamIndex(72), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(73)), "12", "");
		ui_interface.add_horizontal_slider("Clap", ParamIndex(73), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(74)), "12", "");
		ui_interface.declare(Some(ParamIndex(74)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Clap Mod", ParamIndex(74), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(75)), "13", "");
		ui_interface.add_horizontal_slider("Density", ParamIndex(75), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(76)), "13", "");
		ui_interface.declare(Some(ParamIndex(76)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Density Mod", ParamIndex(76), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(77)), "14", "");
		ui_interface.add_horizontal_slider("Crush", ParamIndex(77), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(78)), "14", "");
		ui_interface.declare(Some(ParamIndex(78)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Crush Mod", ParamIndex(78), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(79)), "15", "");
		ui_interface.add_horizontal_slider("Vel Level", ParamIndex(79), 0.6, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(80)), "15", "");
		ui_interface.declare(Some(ParamIndex(80)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Level Mod", ParamIndex(80), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(81)), "16", "");
		ui_interface.declare(Some(ParamIndex(81)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Loop Pitch Mod", ParamIndex(81), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(82)), "16", "");
		ui_interface.declare(Some(ParamIndex(82)), "tooltip", "Playback pitch of the loop only");
		ui_interface.declare(Some(ParamIndex(82)), "unit", "st");
		ui_interface.add_horizontal_slider("Loop Pitch", ParamIndex(82), 0.0, -24.0, 24.0, 0.01);
		ui_interface.declare(Some(ParamIndex(83)), "17", "");
		ui_interface.declare(Some(ParamIndex(83)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Bite Mod", ParamIndex(83), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(84)), "17", "");
		ui_interface.declare(Some(ParamIndex(84)), "tooltip", "The envelope drives the noise into a transistor curve: gritty at the hit, clean as it decays");
		ui_interface.add_horizontal_slider("Bite", ParamIndex(84), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(85)), "18", "");
		ui_interface.declare(Some(ParamIndex(85)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Spread Mod", ParamIndex(85), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(86)), "18", "");
		ui_interface.declare(Some(ParamIndex(86)), "tooltip", "Metal only: 0 is the 808's ratios; down slides them onto the harmonic series, up stretches them");
		ui_interface.add_horizontal_slider("Spread", ParamIndex(86), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(87)), "1", "");
		ui_interface.declare(Some(ParamIndex(87)), "style", "menu{'White':0;'Pink':1;'Dust':2;'Sample & Hold':3;'Metal':4;'RF':5;'Ramp':6}");
		ui_interface.add_num_entry("Generator", ParamIndex(87), 0.0, 0.0, 6.0, 1.0);
		ui_interface.declare(Some(ParamIndex(88)), "2", "");
		ui_interface.declare(Some(ParamIndex(88)), "style", "menu{'Steady: 018 SH101 Noise':0;'Steady: 023 RF Nomad Tuning Noise':1;'Steady: 028 4ms SMR Resonator Constant':2;'Steady: 145 DSE Static Constant':3;'Steady: 148 DSE Static Constant':4;'Steady: 149 DSE Static Constant':5;'Steady: 156 DSE Off Station':6;'Steady: 192 Sony Static Constant':7;'Steady: 197 Sony Static Constant with Warble Tone':8;'Steady: 214 Yaesu Transceiver Static Constant':9;'Steady: 232B Yaesu Transceiver Static Constant with Tone':10;'Steady: Beehive Contact':11;'Steady: CD Glitch-bounce-6':12;'Steady: CD Glitch-bounce-7':13;'Steady: Noise-bounce-3':14;'Steady: Noise':15;'Steady: Quiet Street':16;'Crackle: 161 DSE Static with Tone':17;'Crackle: Honey Badger-bounce-2':18;'Crackle: StaticNoise_FX_SP02':19;'Crackle: StaticNoise_FX_SP03':20;'Crackle: StaticNoise_FX_SP04':21;'Crackle: StaticNoise_FX_SP05':22;'Crackle: StaticNoise_FX_SP16':23;'Crackle: StaticNoise_FX_SP17':24;'Crackle: StaticNoise_FX_SP22':25;'Crackle: StaticNoise_FX_SP23':26;'Modulated: Blue Jays Woodpecker-bounce-1':27;'Modulated: CD_Glitch_02_SP':28;'Modulated: CD_Glitch_13_SP':29;'Modulated: CD_Glitch_32_SP':30;'Modulated: CD_Glitch_39_SP':31;'Modulated: Circuit Bent-bounce-1':32;'Modulated: Jungle-bounce-1':33;'Modulated: StaticNoise_FX_SP13':34;'Modulated: StaticNoise_FX_SP18':35;'Modulated: StaticNoise_FX_SP21':36;'Modulated: StaticNoise_FX_SP25':37;'Modulated: StaticNoise_FX_SP26':38;'Modulated: StaticNoise_FX_SP27':39;'Modulated: StaticNoise_FX_SP28':40;'Modulated: StaticNoise_FX_SP29':41;'Modulated: StaticNoise_FX_SP31':42;'Modulated: StaticNoise_FX_SP33':43;'Modulated: iPad':44}");
		ui_interface.add_num_entry("Loop", ParamIndex(88), 0.0, 0.0, 44.0, 1.0);
		ui_interface.declare(Some(ParamIndex(89)), "3", "");
		ui_interface.declare(Some(ParamIndex(89)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Blend Mod", ParamIndex(89), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(90)), "3", "");
		ui_interface.declare(Some(ParamIndex(90)), "tooltip", "0 = generator, 1 = loop");
		ui_interface.add_horizontal_slider("Blend", ParamIndex(90), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(91)), "4", "");
		ui_interface.declare(Some(ParamIndex(91)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Bandpass Mod", ParamIndex(91), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(92)), "4", "");
		ui_interface.declare(Some(ParamIndex(92)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(92)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Bandpass", ParamIndex(92), 2.5e+03, 1e+02, 1.2e+04, 0.1);
		ui_interface.declare(Some(ParamIndex(93)), "5", "");
		ui_interface.declare(Some(ParamIndex(93)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Q Mod", ParamIndex(93), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(94)), "5", "");
		ui_interface.declare(Some(ParamIndex(94)), "tooltip", "0 = flat (only the highpass acts); 1 = 24 dB/oct skirts");
		ui_interface.add_horizontal_slider("Q", ParamIndex(94), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(95)), "6", "");
		ui_interface.declare(Some(ParamIndex(95)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Highpass Mod", ParamIndex(95), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(96)), "6", "");
		ui_interface.declare(Some(ParamIndex(96)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(96)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Highpass", ParamIndex(96), 3e+02, 2e+01, 8e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(97)), "7", "");
		ui_interface.add_horizontal_slider("Env Amount", ParamIndex(97), 0.2, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(98)), "7", "");
		ui_interface.declare(Some(ParamIndex(98)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Env Amount Mod", ParamIndex(98), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(99)), "8", "");
		ui_interface.declare(Some(ParamIndex(99)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Attack Mod", ParamIndex(99), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(100)), "8", "");
		ui_interface.declare(Some(ParamIndex(100)), "unit", "ms");
		ui_interface.add_horizontal_slider("Attack", ParamIndex(100), 0.0, 0.0, 1e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(101)), "9", "");
		ui_interface.declare(Some(ParamIndex(101)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(101), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(102)), "9", "");
		ui_interface.declare(Some(ParamIndex(102)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(102)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(102), 2.5e+02, 2e+01, 4e+03, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "4", "");
		ui_interface.open_horizontal_box("Sample 1");
		ui_interface.declare(Some(ParamIndex(103)), "0", "");
		ui_interface.declare(Some(ParamIndex(103)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(103), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(104)), "1", "");
		ui_interface.declare(Some(ParamIndex(104)), "style", "menu{'Thump':0;'Knock':1;'Spark':2}");
		ui_interface.add_num_entry("Set", ParamIndex(104), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(105)), "2", "");
		ui_interface.add_num_entry("Sample", ParamIndex(105), 1.0, 1.0, 58.0, 1.0);
		ui_interface.declare(Some(ParamIndex(106)), "3", "");
		ui_interface.declare(Some(ParamIndex(106)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Pitch Mod", ParamIndex(106), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(107)), "3", "");
		ui_interface.declare(Some(ParamIndex(107)), "unit", "st");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(107), 0.0, -24.0, 24.0, 0.01);
		ui_interface.declare(Some(ParamIndex(108)), "4", "");
		ui_interface.declare(Some(ParamIndex(108)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(108), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(109)), "4", "");
		ui_interface.declare(Some(ParamIndex(109)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(109)), "tooltip", "Fades the hit out; the top of the range plays it whole");
		ui_interface.declare(Some(ParamIndex(109)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(109), 2e+03, 5.0, 2e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(110)), "5", "");
		ui_interface.declare(Some(ParamIndex(110)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Body Send Mod", ParamIndex(110), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(111)), "5", "");
		ui_interface.declare(Some(ParamIndex(111)), "tooltip", "How hard this hit strikes the Body. As the sends rise, the Body's own mallet fades out");
		ui_interface.add_horizontal_slider("Body Send", ParamIndex(111), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(112)), "6", "");
		ui_interface.add_horizontal_slider("Vel Level", ParamIndex(112), 0.6, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(113)), "6", "");
		ui_interface.declare(Some(ParamIndex(113)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Level Mod", ParamIndex(113), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "5", "");
		ui_interface.open_horizontal_box("Sample 2");
		ui_interface.declare(Some(ParamIndex(114)), "0", "");
		ui_interface.declare(Some(ParamIndex(114)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(114), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(115)), "1", "");
		ui_interface.declare(Some(ParamIndex(115)), "style", "menu{'Thump':0;'Knock':1;'Spark':2}");
		ui_interface.add_num_entry("Set", ParamIndex(115), 2.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(116)), "2", "");
		ui_interface.add_num_entry("Sample", ParamIndex(116), 1.0, 1.0, 58.0, 1.0);
		ui_interface.declare(Some(ParamIndex(117)), "3", "");
		ui_interface.declare(Some(ParamIndex(117)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Pitch Mod", ParamIndex(117), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(118)), "3", "");
		ui_interface.declare(Some(ParamIndex(118)), "unit", "st");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(118), 0.0, -24.0, 24.0, 0.01);
		ui_interface.declare(Some(ParamIndex(119)), "4", "");
		ui_interface.declare(Some(ParamIndex(119)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(119), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(120)), "4", "");
		ui_interface.declare(Some(ParamIndex(120)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(120)), "tooltip", "Fades the hit out; the top of the range plays it whole");
		ui_interface.declare(Some(ParamIndex(120)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(120), 2e+03, 5.0, 2e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(121)), "5", "");
		ui_interface.declare(Some(ParamIndex(121)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Body Send Mod", ParamIndex(121), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(122)), "5", "");
		ui_interface.declare(Some(ParamIndex(122)), "tooltip", "How hard this hit strikes the Body. As the sends rise, the Body's own mallet fades out");
		ui_interface.add_horizontal_slider("Body Send", ParamIndex(122), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(123)), "6", "");
		ui_interface.add_horizontal_slider("Vel Level", ParamIndex(123), 0.6, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(124)), "6", "");
		ui_interface.declare(Some(ParamIndex(124)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Level Mod", ParamIndex(124), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Envelope");
		ui_interface.declare(Some(ParamIndex(125)), "0", "");
		ui_interface.declare(Some(ParamIndex(125)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(125)), "unit", "ms");
		ui_interface.add_horizontal_slider("Attack", ParamIndex(125), 0.0, 0.0, 1e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(126)), "1", "");
		ui_interface.declare(Some(ParamIndex(126)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(126)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(126), 3e+02, 5.0, 4e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(127)), "2", "");
		ui_interface.declare(Some(ParamIndex(127)), "tooltip", "0: a straight fall; 1: snaps down, then tails off");
		ui_interface.add_horizontal_slider("Curve", ParamIndex(127), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(128)), "3", "");
		ui_interface.declare(Some(ParamIndex(128)), "tooltip", "How much the hit's velocity scales the envelope");
		ui_interface.add_horizontal_slider("Velocity", ParamIndex(128), 0.5, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.add_horizontal_slider("freq", ParamIndex(129), 261.6256, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("gain", ParamIndex(130), 0.8, 0.0, 1.0, 0.01);
		ui_interface.add_button("gate", ParamIndex(131));
		ui_interface.add_horizontal_slider("seed", ParamIndex(132), 0.0, 0.0, 1.0, 1e-06);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			131 => Some(self.fButton0),
			33 => Some(self.fCheckbox0),
			16 => Some(self.fEntry0),
			104 => Some(self.fEntry1),
			105 => Some(self.fEntry2),
			115 => Some(self.fEntry3),
			116 => Some(self.fEntry4),
			87 => Some(self.fEntry5),
			88 => Some(self.fEntry6),
			49 => Some(self.fEntry7),
			2 => Some(self.fHslider0),
			3 => Some(self.fHslider1),
			47 => Some(self.fHslider10),
			59 => Some(self.fHslider100),
			98 => Some(self.fHslider101),
			97 => Some(self.fHslider102),
			91 => Some(self.fHslider103),
			92 => Some(self.fHslider104),
			81 => Some(self.fHslider105),
			82 => Some(self.fHslider106),
			89 => Some(self.fHslider107),
			90 => Some(self.fHslider108),
			78 => Some(self.fHslider109),
			5 => Some(self.fHslider11),
			77 => Some(self.fHslider110),
			76 => Some(self.fHslider111),
			75 => Some(self.fHslider112),
			93 => Some(self.fHslider113),
			94 => Some(self.fHslider114),
			95 => Some(self.fHslider115),
			96 => Some(self.fHslider116),
			66 => Some(self.fHslider117),
			67 => Some(self.fHslider118),
			83 => Some(self.fHslider119),
			4 => Some(self.fHslider12),
			84 => Some(self.fHslider120),
			85 => Some(self.fHslider121),
			86 => Some(self.fHslider122),
			7 => Some(self.fHslider13),
			6 => Some(self.fHslider14),
			44 => Some(self.fHslider15),
			45 => Some(self.fHslider16),
			11 => Some(self.fHslider17),
			10 => Some(self.fHslider18),
			35 => Some(self.fHslider19),
			126 => Some(self.fHslider2),
			34 => Some(self.fHslider20),
			14 => Some(self.fHslider21),
			15 => Some(self.fHslider22),
			12 => Some(self.fHslider23),
			13 => Some(self.fHslider24),
			9 => Some(self.fHslider25),
			8 => Some(self.fHslider26),
			42 => Some(self.fHslider27),
			43 => Some(self.fHslider28),
			38 => Some(self.fHslider29),
			125 => Some(self.fHslider3),
			39 => Some(self.fHslider30),
			40 => Some(self.fHslider31),
			41 => Some(self.fHslider32),
			36 => Some(self.fHslider33),
			37 => Some(self.fHslider34),
			21 => Some(self.fHslider35),
			22 => Some(self.fHslider36),
			129 => Some(self.fHslider37),
			1 => Some(self.fHslider38),
			31 => Some(self.fHslider39),
			127 => Some(self.fHslider4),
			32 => Some(self.fHslider40),
			29 => Some(self.fHslider41),
			30 => Some(self.fHslider42),
			23 => Some(self.fHslider43),
			24 => Some(self.fHslider44),
			19 => Some(self.fHslider45),
			20 => Some(self.fHslider46),
			27 => Some(self.fHslider47),
			28 => Some(self.fHslider48),
			17 => Some(self.fHslider49),
			130 => Some(self.fHslider5),
			18 => Some(self.fHslider50),
			25 => Some(self.fHslider51),
			26 => Some(self.fHslider52),
			113 => Some(self.fHslider53),
			112 => Some(self.fHslider54),
			103 => Some(self.fHslider55),
			108 => Some(self.fHslider56),
			109 => Some(self.fHslider57),
			106 => Some(self.fHslider58),
			107 => Some(self.fHslider59),
			128 => Some(self.fHslider6),
			124 => Some(self.fHslider60),
			123 => Some(self.fHslider61),
			114 => Some(self.fHslider62),
			119 => Some(self.fHslider63),
			120 => Some(self.fHslider64),
			117 => Some(self.fHslider65),
			118 => Some(self.fHslider66),
			121 => Some(self.fHslider67),
			122 => Some(self.fHslider68),
			110 => Some(self.fHslider69),
			0 => Some(self.fHslider7),
			111 => Some(self.fHslider70),
			54 => Some(self.fHslider71),
			55 => Some(self.fHslider72),
			48 => Some(self.fHslider73),
			74 => Some(self.fHslider74),
			73 => Some(self.fHslider75),
			101 => Some(self.fHslider76),
			102 => Some(self.fHslider77),
			72 => Some(self.fHslider78),
			71 => Some(self.fHslider79),
			132 => Some(self.fHslider8),
			69 => Some(self.fHslider80),
			70 => Some(self.fHslider81),
			99 => Some(self.fHslider82),
			100 => Some(self.fHslider83),
			80 => Some(self.fHslider84),
			79 => Some(self.fHslider85),
			68 => Some(self.fHslider86),
			53 => Some(self.fHslider87),
			52 => Some(self.fHslider88),
			63 => Some(self.fHslider89),
			46 => Some(self.fHslider9),
			62 => Some(self.fHslider90),
			50 => Some(self.fHslider91),
			51 => Some(self.fHslider92),
			65 => Some(self.fHslider93),
			64 => Some(self.fHslider94),
			56 => Some(self.fHslider95),
			57 => Some(self.fHslider96),
			61 => Some(self.fHslider97),
			60 => Some(self.fHslider98),
			58 => Some(self.fHslider99),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			131 => { self.fButton0 = value }
			33 => { self.fCheckbox0 = value }
			16 => { self.fEntry0 = value }
			104 => { self.fEntry1 = value }
			105 => { self.fEntry2 = value }
			115 => { self.fEntry3 = value }
			116 => { self.fEntry4 = value }
			87 => { self.fEntry5 = value }
			88 => { self.fEntry6 = value }
			49 => { self.fEntry7 = value }
			2 => { self.fHslider0 = value }
			3 => { self.fHslider1 = value }
			47 => { self.fHslider10 = value }
			59 => { self.fHslider100 = value }
			98 => { self.fHslider101 = value }
			97 => { self.fHslider102 = value }
			91 => { self.fHslider103 = value }
			92 => { self.fHslider104 = value }
			81 => { self.fHslider105 = value }
			82 => { self.fHslider106 = value }
			89 => { self.fHslider107 = value }
			90 => { self.fHslider108 = value }
			78 => { self.fHslider109 = value }
			5 => { self.fHslider11 = value }
			77 => { self.fHslider110 = value }
			76 => { self.fHslider111 = value }
			75 => { self.fHslider112 = value }
			93 => { self.fHslider113 = value }
			94 => { self.fHslider114 = value }
			95 => { self.fHslider115 = value }
			96 => { self.fHslider116 = value }
			66 => { self.fHslider117 = value }
			67 => { self.fHslider118 = value }
			83 => { self.fHslider119 = value }
			4 => { self.fHslider12 = value }
			84 => { self.fHslider120 = value }
			85 => { self.fHslider121 = value }
			86 => { self.fHslider122 = value }
			7 => { self.fHslider13 = value }
			6 => { self.fHslider14 = value }
			44 => { self.fHslider15 = value }
			45 => { self.fHslider16 = value }
			11 => { self.fHslider17 = value }
			10 => { self.fHslider18 = value }
			35 => { self.fHslider19 = value }
			126 => { self.fHslider2 = value }
			34 => { self.fHslider20 = value }
			14 => { self.fHslider21 = value }
			15 => { self.fHslider22 = value }
			12 => { self.fHslider23 = value }
			13 => { self.fHslider24 = value }
			9 => { self.fHslider25 = value }
			8 => { self.fHslider26 = value }
			42 => { self.fHslider27 = value }
			43 => { self.fHslider28 = value }
			38 => { self.fHslider29 = value }
			125 => { self.fHslider3 = value }
			39 => { self.fHslider30 = value }
			40 => { self.fHslider31 = value }
			41 => { self.fHslider32 = value }
			36 => { self.fHslider33 = value }
			37 => { self.fHslider34 = value }
			21 => { self.fHslider35 = value }
			22 => { self.fHslider36 = value }
			129 => { self.fHslider37 = value }
			1 => { self.fHslider38 = value }
			31 => { self.fHslider39 = value }
			127 => { self.fHslider4 = value }
			32 => { self.fHslider40 = value }
			29 => { self.fHslider41 = value }
			30 => { self.fHslider42 = value }
			23 => { self.fHslider43 = value }
			24 => { self.fHslider44 = value }
			19 => { self.fHslider45 = value }
			20 => { self.fHslider46 = value }
			27 => { self.fHslider47 = value }
			28 => { self.fHslider48 = value }
			17 => { self.fHslider49 = value }
			130 => { self.fHslider5 = value }
			18 => { self.fHslider50 = value }
			25 => { self.fHslider51 = value }
			26 => { self.fHslider52 = value }
			113 => { self.fHslider53 = value }
			112 => { self.fHslider54 = value }
			103 => { self.fHslider55 = value }
			108 => { self.fHslider56 = value }
			109 => { self.fHslider57 = value }
			106 => { self.fHslider58 = value }
			107 => { self.fHslider59 = value }
			128 => { self.fHslider6 = value }
			124 => { self.fHslider60 = value }
			123 => { self.fHslider61 = value }
			114 => { self.fHslider62 = value }
			119 => { self.fHslider63 = value }
			120 => { self.fHslider64 = value }
			117 => { self.fHslider65 = value }
			118 => { self.fHslider66 = value }
			121 => { self.fHslider67 = value }
			122 => { self.fHslider68 = value }
			110 => { self.fHslider69 = value }
			0 => { self.fHslider7 = value }
			111 => { self.fHslider70 = value }
			54 => { self.fHslider71 = value }
			55 => { self.fHslider72 = value }
			48 => { self.fHslider73 = value }
			74 => { self.fHslider74 = value }
			73 => { self.fHslider75 = value }
			101 => { self.fHslider76 = value }
			102 => { self.fHslider77 = value }
			72 => { self.fHslider78 = value }
			71 => { self.fHslider79 = value }
			132 => { self.fHslider8 = value }
			69 => { self.fHslider80 = value }
			70 => { self.fHslider81 = value }
			99 => { self.fHslider82 = value }
			100 => { self.fHslider83 = value }
			80 => { self.fHslider84 = value }
			79 => { self.fHslider85 = value }
			68 => { self.fHslider86 = value }
			53 => { self.fHslider87 = value }
			52 => { self.fHslider88 = value }
			63 => { self.fHslider89 = value }
			46 => { self.fHslider9 = value }
			62 => { self.fHslider90 = value }
			50 => { self.fHslider91 = value }
			51 => { self.fHslider92 = value }
			65 => { self.fHslider93 = value }
			64 => { self.fHslider94 = value }
			56 => { self.fHslider95 = value }
			57 => { self.fHslider96 = value }
			61 => { self.fHslider97 = value }
			60 => { self.fHslider98 = value }
			58 => { self.fHslider99 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10, outputs11, outputs12, outputs13, outputs14, outputs15, outputs16, outputs17, outputs18, outputs19, outputs20, outputs21, outputs22, outputs23, outputs24, outputs25, outputs26, outputs27) = if let [outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10, outputs11, outputs12, outputs13, outputs14, outputs15, outputs16, outputs17, outputs18, outputs19, outputs20, outputs21, outputs22, outputs23, outputs24, outputs25, outputs26, outputs27, ..] = outputs {
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
			let outputs11 = outputs11[..count as usize].iter_mut();
			let outputs12 = outputs12[..count as usize].iter_mut();
			let outputs13 = outputs13[..count as usize].iter_mut();
			let outputs14 = outputs14[..count as usize].iter_mut();
			let outputs15 = outputs15[..count as usize].iter_mut();
			let outputs16 = outputs16[..count as usize].iter_mut();
			let outputs17 = outputs17[..count as usize].iter_mut();
			let outputs18 = outputs18[..count as usize].iter_mut();
			let outputs19 = outputs19[..count as usize].iter_mut();
			let outputs20 = outputs20[..count as usize].iter_mut();
			let outputs21 = outputs21[..count as usize].iter_mut();
			let outputs22 = outputs22[..count as usize].iter_mut();
			let outputs23 = outputs23[..count as usize].iter_mut();
			let outputs24 = outputs24[..count as usize].iter_mut();
			let outputs25 = outputs25[..count as usize].iter_mut();
			let outputs26 = outputs26[..count as usize].iter_mut();
			let outputs27 = outputs27[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10, outputs11, outputs12, outputs13, outputs14, outputs15, outputs16, outputs17, outputs18, outputs19, outputs20, outputs21, outputs22, outputs23, outputs24, outputs25, outputs26, outputs27)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = self.fHslider0;
		let mut fSlow2: F32 = self.fHslider1;
		let mut fSlow3: F32 = self.fHslider2;
		let mut fSlow4: F32 = self.fHslider3;
		let mut fSlow5: F32 = self.fHslider4;
		let mut fSlow6: F32 = self.fHslider5;
		let mut fSlow7: F32 = self.fHslider6;
		let mut fSlow8: F32 = self.fHslider7;
		let mut fSlow9: F32 = self.fHslider8;
		let mut fSlow10: F32 = 12.9898 * fSlow9;
		let mut fSlow11: F32 = 43758.547 * F32::sin(fSlow10 + 312.932);
		let mut fSlow12: F32 = 0.1 * (2.0 * (fSlow11 - F32::floor(fSlow11)) + -1.0);
		let mut fSlow13: F32 = self.fHslider9;
		let mut fSlow14: F32 = self.fHslider10;
		let mut fSlow15: F32 = self.fHslider11;
		let mut fSlow16: F32 = self.fHslider12;
		let mut fSlow17: F32 = 43758.547 * F32::sin(fSlow10 + 391.165);
		let mut fSlow18: F32 = 0.1 * (2.0 * (fSlow17 - F32::floor(fSlow17)) + -1.0);
		let mut fSlow19: F32 = 43758.547 * F32::sin(fSlow10 + 78.233);
		let mut fSlow20: F32 = 0.075 * (2.0 * (fSlow19 - F32::floor(fSlow19)) + -1.0);
		let mut fSlow21: F32 = self.fHslider13;
		let mut fSlow22: F32 = self.fHslider14;
		let mut fSlow23: F32 = self.fHslider15;
		let mut fSlow24: F32 = self.fHslider16;
		let mut fSlow25: F32 = 43758.547 * F32::sin(fSlow10 + 234.699);
		let mut fSlow26: F32 = 0.04 * (2.0 * (fSlow25 - F32::floor(fSlow25)) + -1.0);
		let mut fSlow27: F32 = self.fHslider17;
		let mut fSlow28: F32 = self.fHslider18;
		let mut fSlow29: F32 = self.fHslider19;
		let mut fSlow30: F32 = self.fHslider20;
		let mut fSlow31: F32 = self.fHslider21;
		let mut fSlow32: F32 = self.fHslider22;
		let mut fSlow33: F32 = self.fHslider23;
		let mut fSlow34: F32 = self.fHslider24;
		let mut fSlow35: F32 = self.fHslider25;
		let mut fSlow36: F32 = self.fHslider26;
		let mut fSlow37: F32 = self.fHslider27;
		let mut fSlow38: F32 = self.fHslider28;
		let mut fSlow39: F32 = self.fHslider29;
		let mut fSlow40: F32 = self.fHslider30;
		let mut fSlow41: F32 = self.fHslider31;
		let mut fSlow42: F32 = self.fHslider32;
		let mut fSlow43: F32 = self.fHslider33;
		let mut fSlow44: F32 = self.fHslider34;
		let mut fSlow45: F32 = 43758.547 * F32::sin(fSlow10 + 156.466);
		let mut fSlow46: F32 = 0.0057762265 * (2.0 * (fSlow45 - F32::floor(fSlow45)) + -1.0);
		let mut iSlow47: i32 = (self.fCheckbox0) as i32;
		let mut fSlow48: F32 = self.fHslider35;
		let mut fSlow49: F32 = self.fHslider36;
		let mut fSlow50: F32 = 0.0038222559 * self.fHslider37;
		let mut fSlow51: F32 = self.fHslider38;
		let mut fSlow52: F32 = self.fHslider39;
		let mut fSlow53: F32 = self.fHslider40;
		let mut iSlow54: i32 = ((F32::abs(fSlow53) > 0.0001) as i32) | ((F32::abs(fSlow52) > 0.0001) as i32);
		let mut fSlow55: F32 = self.fHslider41;
		let mut fSlow56: F32 = self.fHslider42;
		let mut fSlow57: F32 = self.fHslider43;
		let mut fSlow58: F32 = self.fHslider44;
		let mut fSlow59: F32 = self.fEntry0;
		let mut iSlow60: i32 = (fSlow59 == 0.0) as i32;
		let mut fSlow61: F32 = self.fHslider45;
		let mut fSlow62: F32 = self.fHslider46;
		let mut iSlow63: i32 = (((F32::abs(fSlow62) > 0.0001) as i32) | ((F32::abs(fSlow61) > 0.0001) as i32)) | (iSlow60 & (((F32::abs(fSlow58) > 0.0001) as i32) | ((F32::abs(fSlow57) > 0.0001) as i32)));
		let mut fSlow64: F32 = ((iSlow63 | (((F32::abs(fSlow56) > 0.0001) as i32) | ((F32::abs(fSlow55) > 0.0001) as i32))) | iSlow54) as F32;
		let mut iSlow65: i32 = (fSlow64 != 0.0) as i32;
		let mut fSlow66: F32 = (iSlow63) as F32;
		let mut iSlow67: i32 = (fSlow66 != 0.0) as i32;
		let mut fSlow68: F32 = ((fSlow59 == 4.0) as i32) as u32 as F32;
		let mut iSlow69: i32 = (fSlow68 != 0.0) as i32;
		let mut fTemp55: F32 = self.fTempPerm0;
		let mut fTemp56: F32 = self.fTempPerm1;
		let mut fSlow70: F32 = ((fSlow59 == 3.0) as i32) as u32 as F32;
		let mut iSlow71: i32 = (fSlow70 != 0.0) as i32;
		let mut fSlow72: F32 = self.fHslider47;
		let mut fSlow73: F32 = self.fHslider48;
		let mut fTemp57: F32 = self.fTempPerm2;
		let mut fTemp58: F32 = self.fTempPerm3;
		let mut fSlow74: F32 = (iSlow60) as F32;
		let mut iSlow75: i32 = (fSlow74 != 0.0) as i32;
		let mut fSlow76: F32 = ((fSlow59 == 2.0) as i32) as u32 as F32;
		let mut iSlow77: i32 = (fSlow76 != 0.0) as i32;
		let mut fSlow78: F32 = ((fSlow59 == 1.0) as i32) as u32 as F32;
		let mut iSlow79: i32 = (fSlow78 != 0.0) as i32;
		let mut fSlow80: F32 = self.fHslider49;
		let mut fSlow81: F32 = self.fHslider50;
		let mut fTemp59: F32 = self.fTempPerm4;
		let mut fTemp60: F32 = self.fTempPerm5;
		let mut fTemp61: F32 = self.fTempPerm6;
		let mut fTemp62: F32 = self.fTempPerm7;
		let mut fTemp63: F32 = self.fTempPerm8;
		let mut fTemp64: F32 = self.fTempPerm9;
		let mut fTemp65: F32 = self.fTempPerm10;
		let mut fTemp66: F32 = self.fTempPerm11;
		let mut fTemp67: F32 = self.fTempPerm12;
		let mut fTemp68: F32 = self.fTempPerm13;
		let mut fTemp69: F32 = self.fTempPerm14;
		let mut fTemp70: F32 = self.fTempPerm15;
		let mut fTemp71: F32 = self.fTempPerm16;
		let mut fTemp72: F32 = self.fTempPerm17;
		let mut fTemp73: F32 = self.fTempPerm18;
		let mut fTemp74: F32 = self.fTempPerm19;
		let mut fTemp75: F32 = self.fTempPerm20;
		let mut fTemp76: F32 = self.fTempPerm21;
		let mut fTemp77: F32 = self.fTempPerm22;
		let mut fTemp78: F32 = self.fTempPerm23;
		let mut fTemp79: F32 = self.fTempPerm24;
		let mut fTemp80: F32 = self.fTempPerm25;
		let mut fTemp81: F32 = self.fTempPerm26;
		let mut fTemp82: F32 = self.fTempPerm27;
		let mut fTemp83: F32 = self.fTempPerm28;
		let mut fTemp84: F32 = self.fTempPerm29;
		let mut fTemp85: F32 = self.fTempPerm30;
		let mut fTemp86: F32 = self.fTempPerm31;
		let mut fTemp87: F32 = self.fTempPerm32;
		let mut fTemp88: F32 = self.fTempPerm33;
		let mut fTemp89: F32 = self.fTempPerm34;
		let mut fTemp90: F32 = self.fTempPerm35;
		let mut fTemp91: F32 = self.fTempPerm36;
		let mut fTemp92: F32 = self.fTempPerm37;
		let mut fTemp93: F32 = self.fTempPerm38;
		let mut fTemp94: F32 = self.fTempPerm39;
		let mut fTemp95: F32 = self.fTempPerm40;
		let mut fTemp96: F32 = self.fTempPerm41;
		let mut fTemp97: F32 = self.fTempPerm42;
		let mut fTemp98: F32 = self.fTempPerm43;
		let mut fTemp99: F32 = self.fTempPerm44;
		let mut fTemp100: F32 = self.fTempPerm45;
		let mut fTemp101: F32 = self.fTempPerm46;
		let mut fTemp102: F32 = self.fTempPerm47;
		let mut fTemp103: F32 = self.fTempPerm48;
		let mut fTemp104: F32 = self.fTempPerm49;
		let mut fTemp105: F32 = self.fTempPerm50;
		let mut fTemp106: F32 = self.fTempPerm51;
		let mut fTemp107: F32 = self.fTempPerm52;
		let mut fTemp108: F32 = self.fTempPerm53;
		let mut fTemp109: F32 = self.fTempPerm54;
		let mut fTemp110: F32 = self.fTempPerm55;
		let mut fTemp111: F32 = self.fTempPerm56;
		let mut fTemp112: F32 = self.fTempPerm57;
		let mut fTemp113: F32 = self.fTempPerm58;
		let mut fTemp114: F32 = self.fTempPerm59;
		let mut fTemp115: F32 = self.fTempPerm60;
		let mut fTemp116: F32 = self.fTempPerm61;
		let mut fTemp117: F32 = self.fTempPerm62;
		let mut fTemp118: F32 = self.fTempPerm63;
		let mut fTemp119: F32 = self.fTempPerm64;
		let mut fTemp120: F32 = self.fTempPerm65;
		let mut fTemp121: F32 = self.fTempPerm66;
		let mut fTemp122: F32 = self.fTempPerm67;
		let mut fTemp123: F32 = self.fTempPerm68;
		let mut fTemp124: F32 = self.fTempPerm69;
		let mut fTemp125: F32 = self.fTempPerm70;
		let mut fTemp126: F32 = self.fTempPerm71;
		let mut fTemp127: F32 = self.fTempPerm72;
		let mut fTemp128: F32 = self.fTempPerm73;
		let mut fTemp129: F32 = self.fTempPerm74;
		let mut fTemp130: F32 = self.fTempPerm75;
		let mut fTemp131: F32 = self.fTempPerm76;
		let mut fTemp132: F32 = self.fTempPerm77;
		let mut fTemp133: F32 = self.fTempPerm78;
		let mut fTemp134: F32 = self.fTempPerm79;
		let mut fTemp135: F32 = self.fTempPerm80;
		let mut fTemp136: F32 = self.fTempPerm81;
		let mut fTemp137: F32 = self.fTempPerm82;
		let mut fTemp138: F32 = self.fTempPerm83;
		let mut fTemp139: F32 = self.fTempPerm84;
		let mut fTemp140: F32 = self.fTempPerm85;
		let mut fTemp141: F32 = self.fTempPerm86;
		let mut fTemp142: F32 = self.fTempPerm87;
		let mut fTemp143: F32 = self.fTempPerm88;
		let mut fTemp144: F32 = self.fTempPerm89;
		let mut fSlow82: F32 = self.fHslider51;
		let mut fSlow83: F32 = self.fHslider52;
		let mut fTemp145: F32 = self.fTempPerm90;
		let mut fTemp146: F32 = self.fTempPerm91;
		let mut fTemp147: F32 = self.fTempPerm92;
		let mut fTemp148: F32 = self.fTempPerm93;
		let mut fTemp149: F32 = self.fTempPerm94;
		let mut iTemp150: i32 = self.iTempPerm95;
		let mut fTemp151: F32 = self.fTempPerm96;
		let mut fTemp152: F32 = self.fTempPerm97;
		let mut fTemp153: F32 = self.fTempPerm98;
		let mut fTemp154: F32 = self.fTempPerm99;
		let mut fTemp155: F32 = self.fTempPerm100;
		let mut fTemp156: F32 = self.fTempPerm101;
		let mut fTemp157: F32 = self.fTempPerm102;
		let mut fTemp158: F32 = self.fTempPerm103;
		let mut fTemp160: F32 = self.fTempPerm104;
		let mut fTemp161: F32 = self.fTempPerm105;
		let mut fSlow84: F32 = (iSlow54) as F32;
		let mut iSlow85: i32 = (fSlow84 != 0.0) as i32;
		let mut fTemp162: F32 = self.fTempPerm106;
		let mut fTemp163: F32 = self.fTempPerm107;
		let mut fTemp164: F32 = self.fTempPerm108;
		let mut fTemp165: F32 = self.fTempPerm109;
		let mut fTemp166: F32 = self.fTempPerm110;
		let mut fTemp167: F32 = self.fTempPerm111;
		let mut fTemp168: F32 = self.fTempPerm112;
		let mut fTemp169: F32 = self.fTempPerm113;
		let mut fTemp170: F32 = self.fTempPerm114;
		let mut fTemp171: F32 = self.fTempPerm115;
		let mut fTemp172: F32 = self.fTempPerm116;
		let mut fTemp173: F32 = self.fTempPerm117;
		let mut fTemp174: F32 = self.fTempPerm118;
		let mut fTemp175: F32 = self.fTempPerm119;
		let mut fTemp176: F32 = self.fTempPerm120;
		let mut fTemp177: F32 = self.fTempPerm121;
		let mut fTemp178: F32 = self.fTempPerm122;
		let mut fTemp179: F32 = self.fTempPerm123;
		let mut fTemp180: F32 = self.fTempPerm124;
		let mut fTemp181: F32 = self.fTempPerm125;
		let mut fTemp182: F32 = self.fTempPerm126;
		let mut fTemp183: F32 = self.fTempPerm127;
		let mut fTemp184: F32 = self.fTempPerm128;
		let mut fTemp185: F32 = self.fTempPerm129;
		let mut fTemp186: F32 = self.fTempPerm130;
		let mut fTemp187: F32 = self.fTempPerm131;
		let mut fTemp188: F32 = self.fTempPerm132;
		let mut fTemp189: F32 = self.fTempPerm133;
		let mut fTemp190: F32 = self.fTempPerm134;
		let mut fTemp191: F32 = self.fTempPerm135;
		let mut fTemp192: F32 = self.fTempPerm136;
		let mut fTemp193: F32 = self.fTempPerm137;
		let mut fTemp194: F32 = self.fTempPerm138;
		let mut fTemp195: F32 = self.fTempPerm139;
		let mut fTemp196: F32 = self.fTempPerm140;
		let mut fTemp197: F32 = self.fTempPerm141;
		let mut fTemp198: F32 = self.fTempPerm142;
		let mut fTemp199: F32 = self.fTempPerm143;
		let mut fTemp200: F32 = self.fTempPerm144;
		let mut fTemp201: F32 = self.fTempPerm145;
		let mut fTemp202: F32 = self.fTempPerm146;
		let mut fTemp203: F32 = self.fTempPerm147;
		let mut fTemp204: F32 = self.fTempPerm148;
		let mut fTemp205: F32 = self.fTempPerm149;
		let mut fTemp206: F32 = self.fTempPerm150;
		let mut fTemp207: F32 = self.fTempPerm151;
		let mut fTemp208: F32 = self.fTempPerm152;
		let mut fTemp209: F32 = self.fTempPerm153;
		let mut fTemp210: F32 = self.fTempPerm154;
		let mut fTemp211: F32 = self.fTempPerm155;
		let mut fTemp212: F32 = self.fTempPerm156;
		let mut fTemp213: F32 = self.fTempPerm157;
		let mut fTemp214: F32 = self.fTempPerm158;
		let mut fSlow86: F32 = self.fHslider53;
		let mut fSlow87: F32 = self.fHslider54;
		let mut fSlow88: F32 = self.fHslider55;
		let mut fSlow89: F32 = self.fHslider56;
		let mut fSlow90: F32 = self.fHslider57;
		let mut fSlow91: F32 = self.fEntry1;
		let mut iSlow92: i32 = (fSlow91 == 2.0) as i32;
		let mut iSlow93: i32 = (fSlow91 == 1.0) as i32;
		let mut iSlow94: i32 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_mul(58, iSlow93), i32::wrapping_mul(113, iSlow92)), (i32::wrapping_add((self.fEntry2) as i32, -1)) % (i32::wrapping_add(i32::wrapping_mul(58, (fSlow91 == 0.0) as i32), i32::wrapping_add(i32::wrapping_mul(55, iSlow93), i32::wrapping_mul(51, iSlow92)))));
		let mut fSlow95: F32 = 43758.547 * F32::sin(fSlow10 + 938.796);
		let mut fSlow96: F32 = 0.1 * (2.0 * (fSlow95 - F32::floor(fSlow95)) + -1.0);
		let mut fSlow97: F32 = self.fHslider58;
		let mut fSlow98: F32 = self.fHslider59;
		let mut fSlow99: F32 = self.fHslider60;
		let mut fSlow100: F32 = self.fHslider61;
		let mut fSlow101: F32 = self.fHslider62;
		let mut fSlow102: F32 = self.fHslider63;
		let mut fSlow103: F32 = self.fHslider64;
		let mut fSlow104: F32 = self.fEntry3;
		let mut iSlow105: i32 = (fSlow104 == 2.0) as i32;
		let mut iSlow106: i32 = (fSlow104 == 1.0) as i32;
		let mut iSlow107: i32 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_mul(58, iSlow106), i32::wrapping_mul(113, iSlow105)), (i32::wrapping_add((self.fEntry4) as i32, -1)) % (i32::wrapping_add(i32::wrapping_mul(58, (fSlow104 == 0.0) as i32), i32::wrapping_add(i32::wrapping_mul(55, iSlow106), i32::wrapping_mul(51, iSlow105)))));
		let mut fSlow108: F32 = 43758.547 * F32::sin(fSlow10 + 1017.029);
		let mut fSlow109: F32 = 0.1 * (2.0 * (fSlow108 - F32::floor(fSlow108)) + -1.0);
		let mut fSlow110: F32 = self.fHslider65;
		let mut fSlow111: F32 = self.fHslider66;
		let mut fSlow112: F32 = unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min(iSlow107, 163))) as usize] };
		let mut fSlow113: F32 = self.fHslider67;
		let mut fSlow114: F32 = self.fHslider68;
		let mut fSlow115: F32 = unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min(iSlow94, 163))) as usize] };
		let mut fSlow116: F32 = self.fHslider69;
		let mut fSlow117: F32 = self.fHslider70;
		let mut fSlow118: F32 = 43758.547 * F32::sin(fSlow10 + 625.864);
		let mut fSlow119: F32 = 0.1 * (2.0 * (fSlow118 - F32::floor(fSlow118)) + -1.0);
		let mut fSlow120: F32 = self.fHslider71;
		let mut fSlow121: F32 = self.fHslider72;
		let mut fSlow122: F32 = self.fHslider73;
		let mut fSlow123: F32 = self.fHslider74;
		let mut fSlow124: F32 = self.fHslider75;
		let mut fSlow125: F32 = 43758.547 * F32::sin(fSlow10 + 782.33);
		let mut fSlow126: F32 = 0.1 * (2.0 * (fSlow125 - F32::floor(fSlow125)) + -1.0);
		let mut fSlow127: F32 = self.fHslider76;
		let mut fSlow128: F32 = self.fHslider77;
		let mut fSlow129: F32 = self.fHslider78;
		let mut fSlow130: F32 = self.fHslider79;
		let mut fSlow131: F32 = self.fHslider80;
		let mut fSlow132: F32 = self.fHslider81;
		let mut fSlow133: F32 = 43758.547 * F32::sin(fSlow10 + 704.097);
		let mut fSlow134: F32 = 0.1 * (2.0 * (fSlow133 - F32::floor(fSlow133)) + -1.0);
		let mut fSlow135: F32 = self.fHslider82;
		let mut fSlow136: F32 = self.fHslider83;
		let mut fSlow137: F32 = self.fHslider84;
		let mut fSlow138: F32 = self.fHslider85;
		let mut fSlow139: F32 = self.fHslider86;
		let mut fSlow140: F32 = self.fHslider87;
		let mut fSlow141: F32 = self.fHslider88;
		let mut fSlow142: F32 = self.fHslider89;
		let mut fSlow143: F32 = self.fHslider90;
		let mut fSlow144: F32 = self.fHslider91;
		let mut fSlow145: F32 = self.fHslider92;
		let mut fSlow146: F32 = self.fHslider93;
		let mut fSlow147: F32 = self.fHslider94;
		let mut fSlow148: F32 = 43758.547 * F32::sin(fSlow10 + 469.398);
		let mut fSlow149: F32 = 0.1 * (2.0 * (fSlow148 - F32::floor(fSlow148)) + -1.0);
		let mut fSlow150: F32 = self.fHslider95;
		let mut fSlow151: F32 = self.fHslider96;
		let mut fSlow152: F32 = self.fHslider97;
		let mut fSlow153: F32 = self.fHslider98;
		let mut fSlow154: F32 = self.fHslider99;
		let mut fSlow155: F32 = self.fHslider100;
		let mut fSlow156: F32 = 43758.547 * F32::sin(fSlow10 + 547.631);
		let mut fSlow157: F32 = 0.15 * (2.0 * (fSlow156 - F32::floor(fSlow156)) + -1.0);
		let mut fSlow158: F32 = self.fEntry5;
		let mut fSlow159: F32 = self.fHslider101;
		let mut fSlow160: F32 = self.fHslider102;
		let mut fSlow161: F32 = 43758.547 * F32::sin(fSlow10 + 860.563);
		let mut fSlow162: F32 = 0.05 * (2.0 * (fSlow161 - F32::floor(fSlow161)) + -1.0);
		let mut fSlow163: F32 = self.fHslider103;
		let mut fSlow164: F32 = self.fHslider104;
		let mut fSlow165: F32 = self.fEntry6;
		let mut fSlow166: F32 = self.fHslider105;
		let mut fSlow167: F32 = self.fHslider106;
		let mut fSlow168: F32 = 1000003.0 * fSlow9;
		let mut fSlow169: F32 = 1048576.0 * (fSlow168 - F32::floor(fSlow168));
		let mut fSlow170: F32 = self.fHslider107;
		let mut fSlow171: F32 = self.fHslider108;
		let mut fSlow172: F32 = self.fHslider109;
		let mut fSlow173: F32 = self.fHslider110;
		let mut fSlow174: F32 = self.fHslider111;
		let mut fSlow175: F32 = self.fHslider112;
		let mut fSlow176: F32 = self.fHslider113;
		let mut fSlow177: F32 = self.fHslider114;
		let mut fSlow178: F32 = self.fHslider115;
		let mut fSlow179: F32 = self.fHslider116;
		let mut fSlow180: F32 = self.fHslider117;
		let mut fSlow181: F32 = self.fHslider118;
		let mut fSlow182: F32 = self.fHslider119;
		let mut fSlow183: F32 = self.fHslider120;
		let mut fSlow184: F32 = self.fHslider121;
		let mut fSlow185: F32 = self.fHslider122;
		let mut iSlow186: i32 = ((F32::abs(fSlow185) > 0.0001) as i32) | ((F32::abs(fSlow184) > 0.0001) as i32);
		let mut fSlow187: F32 = (iSlow186) as F32;
		let mut fSlow188: F32 = self.fEntry7;
		let zipped_iterators = outputs0.zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7).zip(outputs8).zip(outputs9).zip(outputs10).zip(outputs11).zip(outputs12).zip(outputs13).zip(outputs14).zip(outputs15).zip(outputs16).zip(outputs17).zip(outputs18).zip(outputs19).zip(outputs20).zip(outputs21).zip(outputs22).zip(outputs23).zip(outputs24).zip(outputs25).zip(outputs26).zip(outputs27);
		for (((((((((((((((((((((((((((output0, output1), output2), output3), output4), output5), output6), output7), output8), output9), output10), output11), output12), output13), output14), output15), output16), output17), output18), output19), output20), output21), output22), output23), output24), output25), output26), output27) in zipped_iterators {
			let mut fRec60: F32 = 0.0;
			self.fVec0[0] = fSlow0;
			let mut iTemp0: i32 = (fSlow0 > self.fVec0[1]) as i32;
			let mut iTemp1: i32 = i32::wrapping_sub(1, iTemp0);
			let mut fTemp2: F32 = (iTemp1) as F32;
			let mut fTemp3: F32 = 1.0 - self.fConst1 * fTemp2;
			self.fRec0[0] = fSlow1 * fTemp3 + self.fConst1 * fTemp2 * self.fRec0[1];
			self.fRec1[0] = fSlow2 * fTemp3 + self.fConst1 * fTemp2 * self.fRec1[1];
			self.fRec2[0] = fSlow3 * fTemp3 + self.fConst1 * fTemp2 * self.fRec2[1];
			self.fRec3[0] = fSlow4 * fTemp3 + self.fConst1 * fTemp2 * self.fRec3[1];
			let mut fTemp4: F32 = 0.001 * self.fRec3[0];
			self.iRec4[0] = i32::wrapping_mul(iTemp1, i32::wrapping_add(self.iRec4[1], 1));
			let mut fTemp5: F32 = (self.iRec4[0]) as F32;
			let mut fTemp6: F32 = self.fConst2 * fTemp5;
			self.fRec5[0] = fSlow5 * fTemp3 + self.fConst1 * fTemp2 * self.fRec5[1];
			let mut fTemp7: F32 = F32::powf(1.0 - F32::max(0.0, F32::min(1.0, (fTemp6 - fTemp4) / F32::max(0.0001, 0.001 * self.fRec2[0]))), 6.0 * self.fRec5[0] + 1.0);
			let mut fTemp8: F32 = F32::min(1.0, self.fConst2 * (fTemp5 / F32::max(1e-05, fTemp4)));
			self.fRec6[0] = if iTemp0 != 0 {fSlow6} else {self.fRec6[1]};
			let mut fTemp9: F32 = 1.0 - self.fRec6[0];
			self.fRec7[0] = fSlow7 * fTemp3 + self.fConst1 * fTemp2 * self.fRec7[1];
			let mut fTemp10: F32 = 1.0 - self.fRec7[0] * fTemp9;
			self.iRec8[0] = std::cmp::max(self.iRec8[1], iTemp0);
			let mut fTemp11: F32 = (self.iRec8[0]) as F32;
			let mut fTemp12: F32 = if (self.fRec0[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec1[0] + self.fRec0[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec1[0]};
			self.fRec9[0] = fSlow8 * fTemp3 + self.fConst1 * fTemp2 * self.fRec9[1];
			let mut fTemp13: F32 = fSlow12 * self.fRec9[0] + 1.0;
			self.fRec10[0] = fSlow13 * fTemp3 + self.fConst1 * fTemp2 * self.fRec10[1];
			self.fRec11[0] = fSlow14 * fTemp3 + self.fConst1 * fTemp2 * self.fRec11[1];
			let mut fTemp14: F32 = if (self.fRec10[0] != 0.0) as i32 != 0 {3.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.00025125628 * (self.fRec11[0] + -2e+01)))) + self.fRec10[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec11[0]};
			self.fRec12[0] = fSlow15 * fTemp3 + self.fConst1 * fTemp2 * self.fRec12[1];
			self.fRec13[0] = fSlow16 * fTemp3 + self.fConst1 * fTemp2 * self.fRec13[1];
			let mut fTemp15: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec12[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec13[0] + self.fRec12[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec13[0]}));
			let mut fTemp16: F32 = 1.0 - fTemp15;
			let mut fTemp17: F32 = fTemp15 * F32::exp(-(self.fConst3 * (fTemp5 / F32::max(0.0001, 0.001 * fTemp14 * (fSlow18 * self.fRec9[0] + 1.0)))));
			let mut fTemp18: F32 = fTemp17 + fTemp16 * F32::exp(-(self.fConst3 * (fTemp5 / F32::max(0.0001, 0.001 * fTemp14 * fTemp13 * F32::powf(1e+01, -2.0 * fTemp12)))));
			let mut fTemp19: F32 = F32::powf(1e+01, fSlow20 * self.fRec9[0]);
			self.fRec14[0] = fSlow21 * fTemp3 + self.fConst1 * fTemp2 * self.fRec14[1];
			self.fRec15[0] = fSlow22 * fTemp3 + self.fConst1 * fTemp2 * self.fRec15[1];
			let mut fTemp20: F32 = 1.0 - fTemp9 * if (self.fRec14[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec15[0] + self.fRec14[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec15[0]};
			self.fRec16[0] = fSlow23 * fTemp3 + self.fConst1 * fTemp2 * self.fRec16[1];
			self.fRec17[0] = fSlow24 * fTemp3 + self.fConst1 * fTemp2 * self.fRec17[1];
			let mut fTemp21: F32 = F32::min(1.0, self.fConst2 * (fTemp5 / F32::max(1e-05, 0.001 * if (self.fRec16[0] != 0.0) as i32 != 0 {1e+01 * F32::max(0.0, F32::min(1.0, 0.1 * self.fRec17[0] + self.fRec16[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec17[0]})));
			let mut fTemp22: F32 = self.fRec6[0] + -0.5;
			self.fRec18[0] = fSlow27 * fTemp3 + self.fConst1 * fTemp2 * self.fRec18[1];
			self.fRec19[0] = fSlow28 * fTemp3 + self.fConst1 * fTemp2 * self.fRec19[1];
			self.fRec20[0] = fSlow29 * fTemp3 + self.fConst1 * fTemp2 * self.fRec20[1];
			self.fRec21[0] = fSlow30 * fTemp3 + self.fConst1 * fTemp2 * self.fRec21[1];
			let mut fTemp23: F32 = 3.0 * F32::max(0.0, F32::min(1.0, if (self.fRec20[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec21[0] + self.fRec20[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec21[0]} * (if (self.fRec18[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec19[0] + self.fRec18[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec19[0]} * fTemp22 + 1.0) + fSlow26 * self.fRec9[0]));
			let mut fTemp24: F32 = 5.0 * F32::max(0.0, F32::min(1.0, fTemp23 + -1.0)) + 1.0;
			let mut fTemp25: F32 = F32::tanh(fTemp24);
			let mut fTemp26: F32 = F32::max(0.0, F32::min(1.0, fTemp23 + -2.0));
			let mut fTemp27: F32 = 3.0 * fTemp26 + 1.0;
			self.fRec23[0] = fSlow31 * fTemp3 + self.fConst1 * fTemp2 * self.fRec23[1];
			self.fRec24[0] = fSlow32 * fTemp3 + self.fConst1 * fTemp2 * self.fRec24[1];
			let mut fTemp28: F32 = if (self.fRec23[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec24[0] + self.fRec23[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec24[0]};
			self.fRec25[0] = fSlow33 * fTemp3 + self.fConst1 * fTemp2 * self.fRec25[1];
			self.fRec26[0] = fSlow34 * fTemp3 + self.fConst1 * fTemp2 * self.fRec26[1];
			let mut fTemp29: F32 = 0.001 * if (self.fRec25[0] != 0.0) as i32 != 0 {5e+01 * F32::max(0.0, F32::min(1.0, 0.02 * self.fRec26[0] + self.fRec25[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec26[0]};
			let mut iTemp30: i32 = ((fTemp29 > 1e-07) as i32) | ((fTemp28 > 0.0001) as i32);
			self.fRec27[0] = fSlow35 * fTemp3 + self.fConst1 * fTemp2 * self.fRec27[1];
			self.fRec28[0] = fSlow36 * fTemp3 + self.fConst1 * fTemp2 * self.fRec28[1];
			let mut fTemp31: F32 = 1.0 - fTemp9 * if (self.fRec27[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec28[0] + self.fRec27[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec28[0]};
			self.fRec29[0] = fSlow37 * fTemp3 + self.fConst1 * fTemp2 * self.fRec29[1];
			self.fRec30[0] = fSlow38 * fTemp3 + self.fConst1 * fTemp2 * self.fRec30[1];
			let mut fTemp32: F32 = if (self.fRec29[0] != 0.0) as i32 != 0 {4.8e+02 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0020833334 * (self.fRec30[0] + -2e+01)))) + self.fRec29[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec30[0]};
			self.fRec31[0] = fSlow39 * fTemp3 + self.fConst1 * fTemp2 * self.fRec31[1];
			self.fRec32[0] = fSlow40 * fTemp3 + self.fConst1 * fTemp2 * self.fRec32[1];
			let mut fTemp33: F32 = if (self.fRec31[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec32[0] + self.fRec31[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec32[0]};
			let mut fTemp34: F32 = 1.0 - fTemp33;
			self.fRec33[0] = fSlow41 * fTemp3 + self.fConst1 * fTemp2 * self.fRec33[1];
			self.fRec34[0] = fSlow42 * fTemp3 + self.fConst1 * fTemp2 * self.fRec34[1];
			let mut fTemp35: F32 = if (self.fRec33[0] != 0.0) as i32 != 0 {19.0 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.05263158 * (self.fRec34[0] + -1.0)))) + self.fRec33[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 1.0} else {self.fRec34[0]};
			self.fRec35[0] = fSlow43 * fTemp3 + self.fConst1 * fTemp2 * self.fRec35[1];
			self.fRec36[0] = fSlow44 * fTemp3 + self.fConst1 * fTemp2 * self.fRec36[1];
			let mut fTemp36: F32 = if (self.fRec35[0] != 0.0) as i32 != 0 {1.2e+02 * F32::max(0.0, F32::min(1.0, 0.008333334 * self.fRec36[0] + self.fRec35[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec36[0]};
			self.fRec37[0] = fSlow48 * fTemp3 + self.fConst1 * fTemp2 * self.fRec37[1];
			self.fRec38[0] = fSlow49 * fTemp3 + self.fConst1 * fTemp2 * self.fRec38[1];
			let mut fTemp37: F32 = if (self.fRec37[0] != 0.0) as i32 != 0 {1.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0005050505 * (self.fRec38[0] + -2e+01)))) + self.fRec37[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec38[0]};
			let mut fTemp38: F32 = if iSlow47 != 0 {fSlow50 * fTemp37} else {fTemp37} * F32::exp(fSlow46 * self.fRec9[0]);
			let mut fTemp39: F32 = fTemp38 * F32::exp(0.057762265 * fTemp36 * (fTemp33 * F32::exp(-(self.fConst5 * (fTemp5 / F32::max(0.0001, 0.001 * fTemp35)))) + fTemp34 * F32::exp(-(self.fConst5 * (fTemp5 / F32::max(0.0001, 0.001 * fTemp32))))) * fTemp31);
			let mut fTemp40: F32 = fTemp9 * fTemp28;
			let mut fTemp41: F32 = F32::exp(-(1.3862944 * fTemp40));
			let mut fTemp42: F32 = F32::max(0.0, fTemp6 - fTemp29);
			let mut fTemp43: F32 = fTemp36 * fTemp31;
			let mut fTemp44: F32 = F32::min(self.fConst4, if iTemp30 != 0 {fTemp38 * F32::exp(0.057762265 * fTemp43 * (fTemp33 * F32::exp(-(3.0 * (fTemp42 / F32::max(0.0001, 0.001 * fTemp35 * fTemp41)))) + fTemp34 * F32::exp(-(3.0 * (fTemp42 / F32::max(0.0001, 0.001 * fTemp32 * fTemp41))))))} else {fTemp39});
			let mut fTemp45: F32 = self.fRec22[1] + self.fConst2 * fTemp44;
			self.fRec22[0] = if iTemp0 != 0 {0.0} else {fTemp45 - F32::floor(fTemp45)};
			let mut fTemp46: F32 = F32::max(0.0, F32::min(1.0, fTemp23));
			let mut fTemp47: F32 = 1.0 - fTemp46;
			let mut fTemp48: F32 = F32::tanh((F32::sin(6.2831855 * self.fRec22[0]) * fTemp47 + fTemp46 * (1.0 - 4.0 * F32::abs(self.fRec22[0] + (-0.25 - F32::floor(self.fRec22[0] + 0.25))))) * fTemp24);
			let mut fTemp49: F32 = F32::min(1.0, 4.0 * fTemp26);
			let mut fTemp50: F32 = fTemp49 * F32::sin(1.5707964 * (fTemp48 * fTemp27 / fTemp25));
			let mut fTemp51: F32 = 1.0 - fTemp49;
			let mut fTemp52: F32 = fTemp51 * fTemp48 / fTemp25;
			let mut fTemp53: F32 = fTemp52 + fTemp50;
			self.fVec1[0] = fTemp53;
			self.fRec39[0] = fSlow51 * fTemp3 + self.fConst1 * fTemp2 * self.fRec39[1];
			let mut fTemp54: F32 = self.fRec39[0] * fTemp53 * fTemp21 * fTemp20 * fTemp19 * fTemp18;
			if iSlow65 & (iSlow67 & iSlow69) != 0 {
				fTemp55 = F32::exp(-1.3862944 * fTemp40);
				fTemp56 = F32::min(self.fConst4, if iTemp30 != 0 {fTemp38 * F32::exp(0.057762265 * fTemp43 * (fTemp33 * F32::exp(-3.0 * (fTemp42 / F32::max(0.0001, 0.001 * fTemp35 * fTemp55))) + fTemp34 * F32::exp(-3.0 * (fTemp42 / F32::max(0.0001, 0.001 * fTemp32 * fTemp55)))))} else {fTemp39});
			}
				if (iSlow65 & (iSlow67 & iSlow69)) | (iSlow65 & (iSlow67 & iSlow71)) != 0 {
					self.fRec40[0] = fSlow72 * fTemp3 + self.fConst1 * fTemp2 * self.fRec40[1];
					self.fRec41[0] = fSlow73 * fTemp3 + self.fConst1 * fTemp2 * self.fRec41[1];
					fTemp57 = F32::max(0.0, F32::min(1.0, if (self.fRec40[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec41[0] + self.fRec40[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec41[0]}));
				}
					if iSlow65 & (iSlow67 & iSlow69) != 0 {
						fTemp58 = 4.0 * fTemp57 + 0.7;
					}
						if (iSlow65 & (iSlow67 & iSlow79)) | ((iSlow65 & (iSlow67 & iSlow77)) | ((iSlow65 & (iSlow67 & iSlow69)) | ((iSlow65 & (iSlow67 & iSlow71)) | (iSlow65 & (iSlow67 & iSlow75))))) != 0 {
							self.fRec42[0] = fSlow80 * fTemp3 + self.fConst1 * fTemp2 * self.fRec42[1];
							self.fRec43[0] = fSlow81 * fTemp3 + self.fConst1 * fTemp2 * self.fRec43[1];
							fTemp59 = if (self.fRec42[0] != 0.0) as i32 != 0 {15.75 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.06349207 * (self.fRec43[0] + -0.25)))) + self.fRec42[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 0.25} else {self.fRec43[0]};
						}
							if iSlow65 & (iSlow67 & iSlow69) != 0 {
								fTemp60 = F32::max(1.0, F32::min(16.0, fTemp59));
								fTemp61 = F32::exp(-(VoiceDsp_faustpower2_f((16.0 - fTemp60) / fTemp58)));
								fTemp62 = fTemp61 * (((16.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp63 = F32::exp(-(VoiceDsp_faustpower2_f((15.0 - fTemp60) / fTemp58)));
								fTemp64 = fTemp63 * (((15.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp65 = F32::exp(-(VoiceDsp_faustpower2_f((14.0 - fTemp60) / fTemp58)));
								fTemp66 = fTemp65 * (((14.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp67 = F32::exp(-(VoiceDsp_faustpower2_f((13.0 - fTemp60) / fTemp58)));
								fTemp68 = fTemp67 * (((13.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp69 = F32::exp(-(VoiceDsp_faustpower2_f((5.0 - fTemp60) / fTemp58)));
								fTemp70 = fTemp69 * (((5.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp71 = F32::exp(-(VoiceDsp_faustpower2_f((11.0 - fTemp60) / fTemp58)));
								fTemp72 = fTemp71 * (((11.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp73 = F32::exp(-(VoiceDsp_faustpower2_f((1e+01 - fTemp60) / fTemp58)));
								fTemp74 = fTemp73 * (((1e+01 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp75 = (((4.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp76 = F32::exp(-(VoiceDsp_faustpower2_f((4.0 - fTemp60) / fTemp58)));
								fTemp77 = (((2.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp78 = F32::exp(-(VoiceDsp_faustpower2_f((2.0 - fTemp60) / fTemp58)));
								fTemp79 = ((fTemp56 < self.fConst6) as i32) as u32 as F32;
								fTemp80 = F32::exp(-(VoiceDsp_faustpower2_f((1.0 - fTemp60) / fTemp58)));
								fTemp81 = (((3.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp82 = F32::exp(-(VoiceDsp_faustpower2_f((3.0 - fTemp60) / fTemp58)));
								fTemp83 = F32::exp(-(VoiceDsp_faustpower2_f((6.0 - fTemp60) / fTemp58)));
								fTemp84 = fTemp83 * (((6.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp85 = F32::exp(-(VoiceDsp_faustpower2_f((7.0 - fTemp60) / fTemp58)));
								fTemp86 = fTemp85 * (((7.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp87 = (((8.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp88 = F32::exp(-(VoiceDsp_faustpower2_f((8.0 - fTemp60) / fTemp58)));
								fTemp89 = F32::exp(-(VoiceDsp_faustpower2_f((9.0 - fTemp60) / fTemp58)));
								fTemp90 = fTemp89 * (((9.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp91 = (((12.0 * fTemp56) < self.fConst6) as i32) as u32 as F32;
								fTemp92 = F32::exp(-(VoiceDsp_faustpower2_f((12.0 - fTemp60) / fTemp58)));
								fTemp93 = 6.2831855 * (self.fRec22[0] + (0.5 - F32::floor(self.fRec22[0] + 0.5)));
								fTemp94 = F32::cos(fTemp93);
								fTemp95 = VoiceDsp_faustpower2_f(fTemp94);
								fTemp96 = 2.0 * fTemp95 + -1.0;
								fTemp97 = 1.0 - fTemp96;
								fTemp98 = fTemp96 - (1.0 - 4.0 * fTemp95 * fTemp97);
								fTemp99 = fTemp97 - fTemp98;
								fTemp100 = 2.0 * fTemp95 * (2.0 * fTemp99 + 1.0) + -1.0;
								fTemp101 = fTemp98 + fTemp100;
								fTemp102 = 4.0 * fTemp95;
								fTemp103 = 2.0 - fTemp102;
								fTemp104 = fTemp103 + 1.0;
								fTemp105 = fTemp104 + 1.0;
								fTemp106 = 3.0 - 4.0 * fTemp95 * fTemp105;
								fTemp107 = fTemp104 + fTemp106;
								fTemp108 = F32::sin(fTemp93);
								fTemp109 = 1.0 - fTemp95 * (fTemp105 + fTemp107);
								fTemp110 = 4.0 * fTemp107 + 16.0 * fTemp109;
								fTemp111 = fTemp95 * fTemp110;
								fTemp112 = 4.0 * fTemp109;
								fTemp113 = fTemp112 + (1.0 - fTemp111);
								fTemp114 = fTemp110 + 4.0 * fTemp113;
								fTemp115 = fTemp110 + fTemp114;
								fTemp116 = fTemp112 + (2.0 - fTemp95 * fTemp115);
								fTemp117 = fTemp114 + 4.0 * fTemp116;
								fTemp118 = fTemp115 + fTemp117;
								fTemp119 = fTemp112 + (3.0 - fTemp95 * fTemp118);
								fTemp120 = fTemp117 + 4.0 * fTemp119;
								fTemp121 = fTemp108 * fTemp94;
								fTemp122 = fTemp99 + (1.0 - fTemp101);
								fTemp123 = fTemp101 - (1.0 - 4.0 * fTemp95 * fTemp122);
								fTemp124 = fTemp122 - fTemp123;
								fTemp125 = 2.0 * fTemp95 * (2.0 * fTemp124 + 1.0) + -1.0;
								fTemp126 = fTemp123 + fTemp125;
								fTemp127 = fTemp124 + (1.0 - fTemp126);
								fTemp128 = fTemp126 - (1.0 - 4.0 * fTemp95 * fTemp127);
								fTemp129 = 2.0 * fTemp95 * (2.0 * (fTemp127 - fTemp128) + 1.0) + -1.0;
								fTemp130 = 0.98078525 * fTemp108 + 0.19509032 * fTemp94;
								fTemp131 = 1.4142135 * fTemp121 + 0.70710677 * fTemp96;
								fTemp132 = fTemp102 + -1.0;
								fTemp133 = 2.0 * fTemp96 + -1.0;
								fTemp134 = 0.98078525 * fTemp94 * fTemp133 - 0.19509032 * fTemp108 * fTemp132;
							}
								if (iSlow65 & (iSlow67 & iSlow79)) | ((iSlow65 & (iSlow67 & iSlow77)) | ((iSlow65 & (iSlow67 & iSlow69)) | ((iSlow65 & (iSlow67 & iSlow71)) | (iSlow65 & (iSlow67 & iSlow75))))) != 0 {
									self.fRec44[0] = fSlow61 * fTemp3 + self.fConst1 * fTemp2 * self.fRec44[1];
									self.fRec45[0] = fSlow62 * fTemp3 + self.fConst1 * fTemp2 * self.fRec45[1];
									fTemp135 = F32::max(0.0, F32::min(1.0, if (self.fRec44[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec45[0] + self.fRec44[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec45[0]}));
								}
									if iSlow65 & (iSlow67 & iSlow69) != 0 {
										fTemp136 = fSlow68 * (fTemp53 + fTemp135 * (1.12 * ((fTemp68 * (0.19509032 * fTemp108 * (fTemp95 * fTemp117 + -1.0) + 0.98078525 * fTemp94 * (1.0 - 2.0 * fTemp128)) + fTemp90 * (0.98078525 * fTemp108 * (fTemp111 + -1.0) - 0.19509032 * fTemp94 * (1.0 - 2.0 * fTemp123)) + fTemp84 * (1.4142135 * fTemp121 * fTemp106 + 0.70710677 * fTemp100) + fTemp82 * fTemp134 * fTemp81 + fTemp78 * fTemp131 * fTemp77 + fTemp80 * fTemp130 * fTemp79 + fTemp74 * (1.4142135 * fTemp121 * fTemp113 + 0.70710677 * fTemp125) + fTemp72 * (0.19509032 * fTemp108 * (fTemp95 * fTemp114 + -1.0) - 0.98078525 * fTemp94 * (2.0 * fTemp126 + -1.0)) + fTemp64 * (0.98078525 * fTemp108 * (fTemp95 * fTemp120 + -1.0) + 0.19509032 * fTemp94 * (2.0 * (fTemp128 + fTemp129) + -1.0)) + fTemp66 * (1.4142135 * fTemp121 * fTemp119 + 0.70710677 * fTemp129) + 2.0 * fTemp121 * (fTemp76 * fTemp103 * fTemp75 + fTemp92 * fTemp116 * fTemp91 - (4.0 * fTemp88 * fTemp109 * fTemp87 + fTemp62 * (4.0 * (fTemp109 + 1.0) - fTemp95 * (fTemp118 + fTemp120)))) - (fTemp70 * (0.19509032 * fTemp108 * (4.0 * fTemp95 * fTemp104 + -1.0) + 0.98078525 * fTemp94 * (1.0 - 2.0 * fTemp98)) + fTemp86 * (0.98078525 * fTemp108 * (4.0 * fTemp95 * fTemp107 + -1.0) + 0.19509032 * fTemp94 * (2.0 * fTemp101 + -1.0)))) / F32::sqrt(F32::max(1e-06, VoiceDsp_faustpower2_f(fTemp92 * fTemp91) + VoiceDsp_faustpower2_f(fTemp90) + VoiceDsp_faustpower2_f(fTemp88 * fTemp87) + VoiceDsp_faustpower2_f(fTemp86) + VoiceDsp_faustpower2_f(fTemp84) + VoiceDsp_faustpower2_f(fTemp82 * fTemp81) + VoiceDsp_faustpower2_f(fTemp80 * fTemp79) + VoiceDsp_faustpower2_f(fTemp78 * fTemp77) + VoiceDsp_faustpower2_f(fTemp76 * fTemp75) + VoiceDsp_faustpower2_f(fTemp74) + VoiceDsp_faustpower2_f(fTemp72) + VoiceDsp_faustpower2_f(fTemp70) + VoiceDsp_faustpower2_f(fTemp68) + VoiceDsp_faustpower2_f(fTemp66) + VoiceDsp_faustpower2_f(fTemp64) + VoiceDsp_faustpower2_f(fTemp62)))) - fTemp53));
									}
										if iSlow65 & (iSlow67 & iSlow71) != 0 {
											fTemp137 = self.fRec22[0] * F32::max(1.0, fTemp59);
											fTemp138 = fSlow70 * (fTemp53 + fTemp135 * (2.4 * F32::powf(1.0 - self.fRec22[0], 7.0 * fTemp57 + 1.0) * F32::sin(6.2831855 * (fTemp137 - F32::floor(fTemp137))) - fTemp53));
										}
											if iSlow65 & (iSlow67 & iSlow75) != 0 {
												self.fRec47[0] = fSlow57 * fTemp3 + self.fConst1 * fTemp2 * self.fRec47[1];
												self.fRec48[0] = fSlow58 * fTemp3 + self.fConst1 * fTemp2 * self.fRec48[1];
											}
												if (iSlow65 & (iSlow67 & iSlow79)) | ((iSlow65 & (iSlow67 & iSlow77)) | (iSlow65 & (iSlow67 & iSlow75))) != 0 {
													fTemp139 = self.fRec49[1] + self.fConst2 * F32::min(self.fConst4, fTemp44 * fTemp59);
													self.fRec49[0] = if iTemp0 != 0 {0.0} else {fTemp139 - F32::floor(fTemp139)};
													fTemp140 = F32::sin(6.2831855 * self.fRec49[0]);
													self.fVec2[0] = fTemp140;
												}
													if iSlow65 & (iSlow67 & iSlow75) != 0 {
														fTemp141 = 1.5 * fTemp140 * VoiceDsp_faustpower2_f(fTemp135) + self.fRec22[0] + 1.6 * self.fRec46[1] * F32::max(0.0, F32::min(1.0, if (self.fRec47[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec48[0] + self.fRec47[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec48[0]}));
														fTemp142 = F32::floor(fTemp141);
														fTemp143 = F32::tanh(fTemp24 * (fTemp47 * F32::sin(6.2831855 * (fTemp141 - fTemp142)) + fTemp46 * (1.0 - 4.0 * F32::abs(fTemp141 + (-0.25 - (fTemp142 + F32::floor(fTemp141 + (0.25 - fTemp142))))))));
														self.fRec46[0] = fTemp51 * fTemp143 / fTemp25 + fTemp49 * F32::sin(1.5707964 * (fTemp27 * fTemp143 / fTemp25));
														fTemp144 = fSlow74 * self.fRec46[0];
													}
														if (iSlow65 & (iSlow67 & iSlow79)) | (iSlow65 & (iSlow67 & iSlow77)) != 0 {
															self.fRec50[0] = fSlow82 * fTemp3 + self.fConst1 * fTemp2 * self.fRec50[1];
															self.fRec51[0] = fSlow83 * fTemp3 + self.fConst1 * fTemp2 * self.fRec51[1];
															fTemp145 = F32::max(0.0, F32::min(1.0, if (self.fRec50[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec51[0] + self.fRec50[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec51[0]}));
															fTemp146 = 1.0 - fTemp145;
														}
															if iSlow65 & (iSlow67 & iSlow79) != 0 {
																fTemp147 = fTemp50 + fTemp140 + fTemp52;
																fTemp148 = F32::sin(1.5707964 * fTemp147) * F32::sin(3.1415927 * fTemp140 * fTemp53);
																fTemp149 = 5.0 * fTemp145;
																iTemp150 = (fTemp140 > 0.0) as i32;
																fTemp151 = F32::abs(fTemp53);
																fTemp152 = 0.7071 * fTemp147 * F32::max(0.0, 1.0 - F32::abs(fTemp149)) + fTemp53 * ((iTemp150) as F32 * F32::max(0.0, 1.0 - F32::abs(fTemp149 + -2.0)) + 1.4142 * fTemp140 * F32::max(0.0, 1.0 - F32::abs(fTemp149 + -1.0))) + (fTemp53 + ((F32::abs(fTemp140) > fTemp151) as i32) as u32 as F32 * (fTemp140 - fTemp53)) * F32::max(0.0, 1.0 - F32::abs(fTemp149 + -3.0)) + fTemp151 * (i32::wrapping_add(i32::wrapping_mul(2, iTemp150), -1)) as F32 * F32::max(0.0, 1.0 - F32::abs(fTemp149 + -4.0));
																fTemp153 = fSlow78 * (fTemp53 + fTemp135 * (fTemp152 + 1.7 * fTemp148 * F32::max(0.0, 1.0 - F32::abs(-5.0 * fTemp146)) - fTemp53));
															}
																if iSlow65 & (iSlow67 & iSlow77) != 0 {
																	self.fRec52[0] = if ((fTemp53 * self.fVec1[1]) < 0.0) as i32 != 0 {fTemp140} else {self.fRec52[1]};
																	fTemp154 = 3.0 * fTemp145;
																	fTemp155 = F32::max(0.0, 1.0 - F32::abs(fTemp154));
																	self.fRec53[0] = if ((fTemp140 * self.fVec2[1]) < 0.0) as i32 != 0 {fTemp53} else {self.fRec53[1]};
																	fTemp156 = self.fRec52[0] * F32::max(0.0, 1.0 - F32::abs(fTemp154 + -1.0)) + 0.7071 * (self.fRec53[0] + self.fRec52[0]) * F32::max(0.0, 1.0 - F32::abs(fTemp154 + -2.0));
																	fTemp157 = fSlow76 * (fTemp53 + fTemp135 * (fTemp156 + self.fRec53[0] * (fTemp155 + 1.4142 * self.fRec52[0] * F32::max(0.0, 1.0 - F32::abs(-3.0 * fTemp146))) - fTemp53));
																}
																	if iSlow65 & iSlow67 != 0 {
																		fTemp158 = fSlow66 * (fTemp157 + fTemp153 + fTemp144 + fTemp138 + fTemp136);
																	}
																		self.iRec55[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec55[1]), 12345);
																		let mut fTemp159: F32 = (self.iRec55[0]) as F32;
																		if iSlow65 != 0 {
																			self.fRec54[0] = 0.5221894 * self.fRec54[3] + 4.656613e-10 * fTemp159 + 2.494956 * self.fRec54[1] - 2.0172658 * self.fRec54[2];
																			self.fRec56[0] = fSlow55 * fTemp3 + self.fConst1 * fTemp2 * self.fRec56[1];
																			self.fRec57[0] = fSlow56 * fTemp3 + self.fConst1 * fTemp2 * self.fRec57[1];
																			fTemp160 = F32::max(0.0, F32::min(1.0, if (self.fRec56[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec57[0] + self.fRec56[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec57[0]})) * (21.6 * (0.049922034 * self.fRec54[0] + 0.0506127 * self.fRec54[2] - (0.095993534 * self.fRec54[1] + 0.004408786 * self.fRec54[3])) + -1.0) + 1.0;
																			fTemp161 = fTemp160 * (fTemp53 + fSlow66 * (fTemp158 - fTemp53));
																		}
																			if iSlow65 & iSlow85 != 0 {
																				self.fRec61[0] = fSlow52 * fTemp3 + self.fConst1 * fTemp2 * self.fRec61[1];
																				self.fRec62[0] = fSlow53 * fTemp3 + self.fConst1 * fTemp2 * self.fRec62[1];
																				fTemp162 = F32::tan(self.fConst7 * F32::max(4e+01, F32::min(self.fConst6, 2e+04 * F32::powf(F32::max(1e-06, self.fRec6[0] * fTemp21 * (fTemp16 * F32::exp(-(self.fConst3 * (fTemp5 / F32::max(0.0001, 0.001 * fTemp14 * F32::powf(1e+01, -(2.0 * fTemp12)) * fTemp13)))) + fTemp17)), 4.0 * F32::max(0.0, F32::min(1.0, if (self.fRec61[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec62[0] + self.fRec61[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec62[0]}))))));
																				fTemp163 = fTemp162 * (fTemp162 + 1.4142271) + 1.0;
																			}
																				if iSlow65 & (iSlow67 & (iSlow69 & iSlow85)) != 0 {
																					fTemp164 = fTemp61 * (((16.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp165 = fTemp63 * (((15.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp166 = fTemp65 * (((14.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp167 = fTemp67 * (((13.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp168 = fTemp92 * (((12.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp169 = fTemp89 * (((9.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp170 = fTemp88 * (((8.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp171 = fTemp85 * (((7.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp172 = fTemp83 * (((6.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp173 = fTemp69 * (((5.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp174 = fTemp82 * (((3.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp175 = fTemp80 * ((fTemp44 < self.fConst6) as i32) as u32 as F32;
																					fTemp176 = fTemp78 * (((2.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp177 = fTemp76 * (((4.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp178 = fTemp73 * (((1e+01 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp179 = fTemp71 * (((11.0 * fTemp44) < self.fConst6) as i32) as u32 as F32;
																					fTemp180 = 1.0 - fTemp133;
																					fTemp181 = 2.0 * (1.0 - 2.0 * fTemp95 * fTemp180) - fTemp133;
																					fTemp182 = fTemp180 + fTemp181;
																					fTemp183 = 2.0 * fTemp95 * fTemp182 + -1.0;
																					fTemp184 = 2.0 * fTemp183 - fTemp181;
																					fTemp185 = 1.0 - fTemp132;
																					fTemp186 = fTemp132 + 4.0 * fTemp95 * fTemp185;
																					fTemp187 = fTemp185 - fTemp186;
																					fTemp188 = fTemp186 + 4.0 * fTemp95 * fTemp187;
																					fTemp189 = fTemp187 - fTemp188;
																					fTemp190 = fTemp188 + 4.0 * fTemp95 * fTemp189;
																					fTemp191 = fTemp189 - fTemp190;
																					fTemp192 = fTemp190 + 4.0 * fTemp95 * fTemp191;
																					fTemp193 = fTemp191 - fTemp192;
																					fTemp194 = fTemp192 + 4.0 * fTemp95 * fTemp193;
																					fTemp195 = fTemp193 - fTemp194;
																					fTemp196 = fTemp194 + 4.0 * fTemp95 * fTemp195;
																					fTemp197 = fTemp182 - fTemp184;
																					fTemp198 = 2.0 * (1.0 - 2.0 * fTemp95 * fTemp197) - fTemp184;
																					fTemp199 = fTemp197 + fTemp198;
																					fTemp200 = 2.0 * fTemp95 * fTemp199 + -1.0;
																					fTemp201 = 2.0 * fTemp200 - fTemp198;
																					fTemp202 = fTemp199 - fTemp201;
																					fTemp203 = 2.0 * (1.0 - 2.0 * fTemp95 * fTemp202) - fTemp201;
																					fTemp204 = 2.0 * fTemp95 * (fTemp202 + fTemp203) + -1.0;
																					fTemp205 = fSlow68 * (fTemp53 + fTemp135 * (1.12 * ((fTemp165 * (0.98078525 * fTemp108 * fTemp196 + 0.19509032 * fTemp94 * (2.0 * fTemp204 - fTemp203)) + fTemp166 * (1.4142135 * fTemp121 * fTemp195 + 0.70710677 * fTemp204) + fTemp179 * (0.19509032 * fTemp108 * fTemp192 - 0.98078525 * fTemp94 * fTemp201) + fTemp178 * (1.4142135 * fTemp121 * fTemp191 + 0.70710677 * fTemp200) + fTemp175 * fTemp130 + fTemp176 * fTemp131 + fTemp174 * fTemp134 + fTemp172 * (1.4142135 * fTemp121 * fTemp187 + 0.70710677 * fTemp183) + fTemp169 * (0.98078525 * fTemp108 * fTemp190 - 0.19509032 * fTemp94 * fTemp198) + fTemp167 * (0.19509032 * fTemp108 * fTemp194 + 0.98078525 * fTemp94 * fTemp203) + 2.0 * fTemp121 * (fTemp177 * fTemp185 - fTemp170 * fTemp189 + fTemp168 * fTemp193 - fTemp164 * (fTemp195 - fTemp196)) - (fTemp173 * (0.19509032 * fTemp108 * fTemp186 + 0.98078525 * fTemp94 * fTemp181) + fTemp171 * (0.98078525 * fTemp108 * fTemp188 + 0.19509032 * fTemp94 * fTemp184))) / F32::sqrt(F32::max(1e-06, VoiceDsp_faustpower2_f(fTemp179) + VoiceDsp_faustpower2_f(fTemp178) + VoiceDsp_faustpower2_f(fTemp177) + VoiceDsp_faustpower2_f(fTemp176) + VoiceDsp_faustpower2_f(fTemp175) + VoiceDsp_faustpower2_f(fTemp174) + VoiceDsp_faustpower2_f(fTemp173) + VoiceDsp_faustpower2_f(fTemp172) + VoiceDsp_faustpower2_f(fTemp171) + VoiceDsp_faustpower2_f(fTemp170) + VoiceDsp_faustpower2_f(fTemp169) + VoiceDsp_faustpower2_f(fTemp168) + VoiceDsp_faustpower2_f(fTemp167) + VoiceDsp_faustpower2_f(fTemp166) + VoiceDsp_faustpower2_f(fTemp165) + VoiceDsp_faustpower2_f(fTemp164)))) - fTemp53));
																				}
																					if iSlow65 & (iSlow67 & (iSlow77 & iSlow85)) != 0 {
																						fTemp206 = fSlow76 * (fTemp53 + fTemp135 * (self.fRec53[0] * (fTemp155 + 1.4142 * self.fRec52[0] * F32::max(0.0, 1.0 - F32::abs(-(3.0 * fTemp146)))) + fTemp156 - fTemp53));
																					}
																						if iSlow65 & (iSlow67 & (iSlow79 & iSlow85)) != 0 {
																							fTemp207 = fSlow78 * (fTemp53 + fTemp135 * (fTemp152 + 1.7 * fTemp148 * F32::max(0.0, 1.0 - F32::abs(-(5.0 * fTemp146))) - fTemp53));
																						}
																							if iSlow65 & (iSlow67 & iSlow85) != 0 {
																								fTemp208 = fSlow66 * (fTemp144 + fTemp207 + fTemp206 + fTemp138 + fTemp205);
																							}
																								if iSlow65 & iSlow85 != 0 {
																									fTemp209 = (fTemp53 + fSlow66 * (fTemp208 - fTemp53)) * fTemp160;
																									fTemp210 = self.fRec58[1] + fTemp162 * (fTemp209 - self.fRec59[1]);
																									fTemp211 = fTemp210 / fTemp163;
																									self.fRec58[0] = 2.0 * fTemp211 - self.fRec58[1];
																									fTemp212 = self.fRec59[1] + fTemp162 * fTemp210 / fTemp163;
																									self.fRec59[0] = 2.0 * fTemp212 - self.fRec59[1];
																									fRec60 = fTemp212;
																									fTemp213 = fSlow84 * fRec60;
																								}
																									if iSlow65 != 0 {
																										fTemp214 = fSlow64 * self.fRec39[0] * fTemp21 * fTemp20 * fTemp19 * fTemp18 * (fTemp161 + fSlow84 * (fTemp213 - fTemp161));
																									}
																										*output0 = fTemp11 * (fTemp54 + fSlow64 * (fTemp214 - fTemp54));
																										self.fRec63[0] = fSlow86 * fTemp3 + self.fConst1 * fTemp2 * self.fRec63[1];
																										self.fRec64[0] = fSlow87 * fTemp3 + self.fConst1 * fTemp2 * self.fRec64[1];
																										self.fRec65[0] = fSlow88 * fTemp3 + self.fConst1 * fTemp2 * self.fRec65[1];
																										self.fRec67[0] = fSlow89 * fTemp3 + self.fConst1 * fTemp2 * self.fRec67[1];
																										self.fRec68[0] = fSlow90 * fTemp3 + self.fConst1 * fTemp2 * self.fRec68[1];
																										let mut fTemp215: F32 = F32::max(0.005, 0.001 * if (self.fRec67[0] != 0.0) as i32 != 0 {1995.0 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0005012531 * (self.fRec68[0] + -5.0)))) + self.fRec67[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 5.0} else {self.fRec68[0]});
																										self.fRec69[0] = fSlow97 * fTemp3 + self.fConst1 * fTemp2 * self.fRec69[1];
																										self.fRec70[0] = fSlow98 * fTemp3 + self.fConst1 * fTemp2 * self.fRec70[1];
																										let mut fTemp216: F32 = sl_hit(iSlow94, self.fConst8 * fTemp5 * F32::exp(0.057762265 * (if (self.fRec69[0] != 0.0) as i32 != 0 {48.0 * F32::max(0.0, F32::min(1.0, 0.020833334 * (self.fRec70[0] + 24.0) + self.fRec69[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -24.0} else {self.fRec70[0]} + fSlow96 * self.fRec9[0]))) * F32::max(0.0, F32::min(1.0, 2.5 * ((fTemp215 - fTemp6) / fTemp215)));
																										self.fVec3[0] = fTemp216;
																										self.fRec66[0] = 0.995 * self.fRec66[1] + fTemp216 - self.fVec3[1];
																										*output1 = self.fRec66[0] * self.fRec65[0] * fTemp11 * fTemp19 * (1.0 - fTemp9 * if (self.fRec63[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec64[0] + self.fRec63[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec64[0]});
																										self.fRec71[0] = fSlow99 * fTemp3 + self.fConst1 * fTemp2 * self.fRec71[1];
																										self.fRec72[0] = fSlow100 * fTemp3 + self.fConst1 * fTemp2 * self.fRec72[1];
																										self.fRec73[0] = fSlow101 * fTemp3 + self.fConst1 * fTemp2 * self.fRec73[1];
																										self.fRec75[0] = fSlow102 * fTemp3 + self.fConst1 * fTemp2 * self.fRec75[1];
																										self.fRec76[0] = fSlow103 * fTemp3 + self.fConst1 * fTemp2 * self.fRec76[1];
																										let mut fTemp217: F32 = F32::max(0.005, 0.001 * if (self.fRec75[0] != 0.0) as i32 != 0 {1995.0 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0005012531 * (self.fRec76[0] + -5.0)))) + self.fRec75[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 5.0} else {self.fRec76[0]});
																										self.fRec77[0] = fSlow110 * fTemp3 + self.fConst1 * fTemp2 * self.fRec77[1];
																										self.fRec78[0] = fSlow111 * fTemp3 + self.fConst1 * fTemp2 * self.fRec78[1];
																										let mut fTemp218: F32 = sl_hit(iSlow107, self.fConst8 * fTemp5 * F32::exp(0.057762265 * (if (self.fRec77[0] != 0.0) as i32 != 0 {48.0 * F32::max(0.0, F32::min(1.0, 0.020833334 * (self.fRec78[0] + 24.0) + self.fRec77[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -24.0} else {self.fRec78[0]} + fSlow109 * self.fRec9[0]))) * F32::max(0.0, F32::min(1.0, 2.5 * ((fTemp217 - fTemp6) / fTemp217)));
																										self.fVec4[0] = fTemp218;
																										self.fRec74[0] = 0.995 * self.fRec74[1] + fTemp218 - self.fVec4[1];
																										*output2 = self.fRec74[0] * self.fRec73[0] * fTemp11 * fTemp19 * (1.0 - fTemp9 * if (self.fRec71[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec72[0] + self.fRec71[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec72[0]});
																										self.fRec79[0] = fSlow113 * fTemp3 + self.fConst1 * fTemp2 * self.fRec79[1];
																										self.fRec80[0] = fSlow114 * fTemp3 + self.fConst1 * fTemp2 * self.fRec80[1];
																										let mut fTemp219: F32 = if (self.fRec79[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec80[0] + self.fRec79[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec80[0]};
																										self.fRec81[0] = fSlow116 * fTemp3 + self.fConst1 * fTemp2 * self.fRec81[1];
																										self.fRec82[0] = fSlow117 * fTemp3 + self.fConst1 * fTemp2 * self.fRec82[1];
																										let mut fTemp220: F32 = if (self.fRec81[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec82[0] + self.fRec81[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec82[0]};
																										self.fRec83[0] = fSlow120 * fTemp3 + self.fConst1 * fTemp2 * self.fRec83[1];
																										self.fRec84[0] = fSlow121 * fTemp3 + self.fConst1 * fTemp2 * self.fRec84[1];
																										let mut fTemp221: F32 = if (self.fRec83[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec84[0] + self.fRec83[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec84[0]};
																										let mut fTemp222: F32 = F32::max(0.0, F32::min(1.0, F32::max(0.0, F32::min(1.0, fTemp221 + fSlow119 * self.fRec9[0]))));
																										let mut fTemp223: F32 = F32::powf(0.016, fTemp222);
																										*output3 = fTemp11 * (self.fConst10 * (((fTemp6 < (0.005 * fTemp223)) as i32) as u32 as F32 * F32::sin(self.fConst9 * (fTemp5 / fTemp223)) * (2.7939678e-10 * fTemp159 * fTemp222 + 1.0) * (1.0 - F32::min(1.0, fTemp220 + fTemp219)) / fTemp223) + 0.05 * (self.fRec66[0] * fTemp220 * fSlow115 + self.fRec74[0] * fTemp219 * fSlow112));
																										self.fRec85[0] = fSlow122 * fTemp3 + self.fConst1 * fTemp2 * self.fRec85[1];
																										*output4 = self.fRec6[0] * self.fRec85[0] * fTemp11 * fTemp19;
																										self.fRec86[0] = fSlow123 * fTemp3 + self.fConst1 * fTemp2 * self.fRec86[1];
																										self.fRec87[0] = fSlow124 * fTemp3 + self.fConst1 * fTemp2 * self.fRec87[1];
																										let mut fTemp224: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec86[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec87[0] + self.fRec86[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec87[0]}));
																										let mut fTemp225: F32 = 0.01 * fTemp224 + 0.006;
																										let mut fTemp226: F32 = F32::floor(4.0 * fTemp224 + 0.5) * fTemp225;
																										self.fRec88[0] = fSlow127 * fTemp3 + self.fConst1 * fTemp2 * self.fRec88[1];
																										self.fRec89[0] = fSlow128 * fTemp3 + self.fConst1 * fTemp2 * self.fRec89[1];
																										let mut fTemp227: F32 = if (self.fRec88[0] != 0.0) as i32 != 0 {3.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.00025125628 * (self.fRec89[0] + -2e+01)))) + self.fRec88[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec89[0]};
																										let mut fTemp228: F32 = F32::max(0.0, fTemp6 - fTemp226);
																										self.fRec90[0] = fSlow129 * fTemp3 + self.fConst1 * fTemp2 * self.fRec90[1];
																										self.fRec91[0] = fSlow130 * fTemp3 + self.fConst1 * fTemp2 * self.fRec91[1];
																										let mut fTemp229: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec90[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec91[0] + self.fRec90[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec91[0]}));
																										self.fRec92[0] = fSlow131 * fTemp3 + self.fConst1 * fTemp2 * self.fRec92[1];
																										self.fRec93[0] = fSlow132 * fTemp3 + self.fConst1 * fTemp2 * self.fRec93[1];
																										self.fRec94[0] = fSlow135 * fTemp3 + self.fConst1 * fTemp2 * self.fRec94[1];
																										self.fRec95[0] = fSlow136 * fTemp3 + self.fConst1 * fTemp2 * self.fRec95[1];
																										let mut fTemp230: F32 = self.fConst2 * (fTemp5 / fTemp225);
																										let mut fTemp231: F32 = if (fTemp6 < fTemp226) as i32 != 0 {F32::exp(-2.5 * (fTemp230 - F32::floor(fTemp230)))} else {F32::min(1.0, fTemp228 / F32::max(1e-05, 0.001 * if (self.fRec94[0] != 0.0) as i32 != 0 {1e+01 * F32::max(0.0, F32::min(1.0, 0.1 * self.fRec95[0] + self.fRec94[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec95[0]})) * ((1.0 - fTemp229) * F32::exp(-6.907755 * (fTemp228 / F32::max(0.0001, 0.001 * fTemp227 * (fSlow134 * self.fRec9[0] + 1.0) * F32::powf(1e+01, -2.0 * if (self.fRec92[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec93[0] + self.fRec92[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec93[0]})))) + fTemp229 * F32::exp(-6.907755 * (fTemp228 / F32::max(0.0001, 0.001 * fTemp227 * (fSlow126 * self.fRec9[0] + 1.0)))))};
																										self.fRec96[0] = fSlow137 * fTemp3 + self.fConst1 * fTemp2 * self.fRec96[1];
																										self.fRec97[0] = fSlow138 * fTemp3 + self.fConst1 * fTemp2 * self.fRec97[1];
																										let mut fTemp232: F32 = 1.0 - fTemp9 * if (self.fRec96[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec97[0] + self.fRec96[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec97[0]};
																										self.fRec98[0] = fSlow139 * fTemp3 + self.fConst1 * fTemp2 * self.fRec98[1];
																										*output5 = self.fRec98[0] * fTemp11 * fTemp19 * fTemp232 * fTemp231;
																										self.fRec99[0] = fSlow140 * fTemp3 + self.fConst1 * fTemp2 * self.fRec99[1];
																										self.fRec100[0] = fSlow141 * fTemp3 + self.fConst1 * fTemp2 * self.fRec100[1];
																										let mut fTemp233: F32 = if (self.fRec99[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec100[0] + self.fRec99[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec100[0]};
																										let mut fTemp234: F32 = F32::max(0.0, F32::min(1.0, fTemp233));
																										let mut fTemp235: F32 = 2.0 * fTemp234;
																										let mut fTemp236: F32 = F32::max(0.0, 1.0 - F32::abs(fTemp235));
																										self.fRec101[0] = fSlow142 * fTemp3 + self.fConst1 * fTemp2 * self.fRec101[1];
																										self.fRec102[0] = fSlow143 * fTemp3 + self.fConst1 * fTemp2 * self.fRec102[1];
																										self.fRec103[0] = fSlow144 * fTemp3 + self.fConst1 * fTemp2 * self.fRec103[1];
																										self.fRec104[0] = fSlow145 * fTemp3 + self.fConst1 * fTemp2 * self.fRec104[1];
																										*output6 = fTemp38 * F32::exp(0.057762265 * (if (self.fRec103[0] != 0.0) as i32 != 0 {48.0 * F32::max(0.0, F32::min(1.0, 0.020833334 * (self.fRec104[0] + 24.0) + self.fRec103[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -24.0} else {self.fRec104[0]} + 2.0 * self.fRec6[0] * if (self.fRec101[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec102[0] + self.fRec101[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec102[0]} * fTemp236 * F32::exp(-(self.fConst11 * fTemp5))));
																										*output7 = fTemp233;
																										self.fRec105[0] = fSlow146 * fTemp3 + self.fConst1 * fTemp2 * self.fRec105[1];
																										self.fRec106[0] = fSlow147 * fTemp3 + self.fConst1 * fTemp2 * self.fRec106[1];
																										self.fRec107[0] = fSlow150 * fTemp3 + self.fConst1 * fTemp2 * self.fRec107[1];
																										self.fRec108[0] = fSlow151 * fTemp3 + self.fConst1 * fTemp2 * self.fRec108[1];
																										*output8 = 0.001 * if (self.fRec107[0] != 0.0) as i32 != 0 {3.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.00025125628 * (self.fRec108[0] + -2e+01)))) + self.fRec107[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec108[0]} * (fSlow149 * self.fRec9[0] + 1.0) * (fTemp22 * if (self.fRec105[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec106[0] + self.fRec105[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec106[0]} + 1.0);
																										self.fRec109[0] = fSlow152 * fTemp3 + self.fConst1 * fTemp2 * self.fRec109[1];
																										self.fRec110[0] = fSlow153 * fTemp3 + self.fConst1 * fTemp2 * self.fRec110[1];
																										self.fRec111[0] = fSlow154 * fTemp3 + self.fConst1 * fTemp2 * self.fRec111[1];
																										self.fRec112[0] = fSlow155 * fTemp3 + self.fConst1 * fTemp2 * self.fRec112[1];
																										*output9 = F32::max(0.0, F32::min(1.0, F32::max(0.0, F32::min(1.0, 0.8 * fTemp221 + (0.5 - if (self.fRec111[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec112[0] + self.fRec111[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec112[0]}))) + 0.5 * self.fRec6[0] * if (self.fRec109[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec110[0] + self.fRec109[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec110[0]}));
																										*output10 = F32::max(0.05, F32::min(0.95, 0.22 * F32::max(0.0, 1.0 - F32::abs(-2.0 * (1.0 - fTemp234))) + 0.3 * fTemp236 + 0.37 * F32::max(0.0, 1.0 - F32::abs(fTemp235 + -1.0)) + fSlow157 * self.fRec9[0]));
																										*output11 = fSlow158;
																										*output12 = fTemp38;
																										self.fRec113[0] = fSlow159 * fTemp3 + self.fConst1 * fTemp2 * self.fRec113[1];
																										self.fRec114[0] = fSlow160 * fTemp3 + self.fConst1 * fTemp2 * self.fRec114[1];
																										self.fRec115[0] = fSlow163 * fTemp3 + self.fConst1 * fTemp2 * self.fRec115[1];
																										self.fRec116[0] = fSlow164 * fTemp3 + self.fConst1 * fTemp2 * self.fRec116[1];
																										*output13 = if (self.fRec115[0] != 0.0) as i32 != 0 {1.19e+04 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 8.4033614e-05 * (self.fRec116[0] + -1e+02)))) + self.fRec115[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 1e+02} else {self.fRec116[0]} * (fSlow162 * self.fRec9[0] + 1.0) * F32::exp(2.7725887 * if (self.fRec113[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec114[0] + self.fRec113[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec114[0]} * fTemp231);
																										*output14 = fSlow165;
																										self.fRec117[0] = fSlow166 * fTemp3 + self.fConst1 * fTemp2 * self.fRec117[1];
																										self.fRec118[0] = fSlow167 * fTemp3 + self.fConst1 * fTemp2 * self.fRec118[1];
																										*output15 = fSlow169 + self.fConst8 * fTemp5 * F32::exp(0.057762265 * if (self.fRec117[0] != 0.0) as i32 != 0 {48.0 * F32::max(0.0, F32::min(1.0, 0.020833334 * (self.fRec118[0] + 24.0) + self.fRec117[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -24.0} else {self.fRec118[0]});
																										self.fRec119[0] = fSlow170 * fTemp3 + self.fConst1 * fTemp2 * self.fRec119[1];
																										self.fRec120[0] = fSlow171 * fTemp3 + self.fConst1 * fTemp2 * self.fRec120[1];
																										*output16 = if (self.fRec119[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec120[0] + self.fRec119[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec120[0]};
																										self.fRec121[0] = fSlow172 * fTemp3 + self.fConst1 * fTemp2 * self.fRec121[1];
																										self.fRec122[0] = fSlow173 * fTemp3 + self.fConst1 * fTemp2 * self.fRec122[1];
																										*output17 = if (self.fRec121[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec122[0] + self.fRec121[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec122[0]};
																										self.fRec123[0] = fSlow174 * fTemp3 + self.fConst1 * fTemp2 * self.fRec123[1];
																										self.fRec124[0] = fSlow175 * fTemp3 + self.fConst1 * fTemp2 * self.fRec124[1];
																										*output18 = if (self.fRec123[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec124[0] + self.fRec123[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec124[0]};
																										self.fRec125[0] = fSlow176 * fTemp3 + self.fConst1 * fTemp2 * self.fRec125[1];
																										self.fRec126[0] = fSlow177 * fTemp3 + self.fConst1 * fTemp2 * self.fRec126[1];
																										*output19 = if (self.fRec125[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec126[0] + self.fRec125[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec126[0]};
																										self.fRec127[0] = fSlow178 * fTemp3 + self.fConst1 * fTemp2 * self.fRec127[1];
																										self.fRec128[0] = fSlow179 * fTemp3 + self.fConst1 * fTemp2 * self.fRec128[1];
																										*output20 = if (self.fRec127[0] != 0.0) as i32 != 0 {7.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.00012531328 * (self.fRec128[0] + -2e+01)))) + self.fRec127[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec128[0]};
																										self.fRec129[0] = fSlow180 * fTemp3 + self.fConst1 * fTemp2 * self.fRec129[1];
																										self.fRec130[0] = fSlow181 * fTemp3 + self.fConst1 * fTemp2 * self.fRec130[1];
																										*output21 = self.fRec6[0] * if (self.fRec129[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec130[0] + self.fRec129[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec130[0]};
																										self.fRec131[0] = fSlow182 * fTemp3 + self.fConst1 * fTemp2 * self.fRec131[1];
																										self.fRec132[0] = fSlow183 * fTemp3 + self.fConst1 * fTemp2 * self.fRec132[1];
																										*output22 = if (self.fRec131[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec132[0] + self.fRec131[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec132[0]};
																										*output23 = fTemp11 * fTemp232 * fTemp231;
																										*output24 = self.fRec98[0] * fTemp19;
																										self.fRec133[0] = fSlow184 * fTemp3 + self.fConst1 * fTemp2 * self.fRec133[1];
																										self.fRec134[0] = fSlow185 * fTemp3 + self.fConst1 * fTemp2 * self.fRec134[1];
																										*output25 = fSlow187 * if (self.fRec133[0] != 0.0) as i32 != 0 {2.0 * F32::max(0.0, F32::min(1.0, 0.5 * (self.fRec134[0] + 1.0) + self.fRec133[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -1.0} else {self.fRec134[0]};
																										*output26 = (iSlow186) as F32;
																										*output27 = fSlow188;
																										self.fVec0[1] = self.fVec0[0];
																										self.fRec0[1] = self.fRec0[0];
																										self.fRec1[1] = self.fRec1[0];
																										self.fRec2[1] = self.fRec2[0];
																										self.fRec3[1] = self.fRec3[0];
																										self.iRec4[1] = self.iRec4[0];
																										self.fRec5[1] = self.fRec5[0];
																										self.fRec6[1] = self.fRec6[0];
																										self.fRec7[1] = self.fRec7[0];
																										self.iRec8[1] = self.iRec8[0];
																										self.fRec9[1] = self.fRec9[0];
																										self.fRec10[1] = self.fRec10[0];
																										self.fRec11[1] = self.fRec11[0];
																										self.fRec12[1] = self.fRec12[0];
																										self.fRec13[1] = self.fRec13[0];
																										self.fRec14[1] = self.fRec14[0];
																										self.fRec15[1] = self.fRec15[0];
																										self.fRec16[1] = self.fRec16[0];
																										self.fRec17[1] = self.fRec17[0];
																										self.fRec18[1] = self.fRec18[0];
																										self.fRec19[1] = self.fRec19[0];
																										self.fRec20[1] = self.fRec20[0];
																										self.fRec21[1] = self.fRec21[0];
																										self.fRec23[1] = self.fRec23[0];
																										self.fRec24[1] = self.fRec24[0];
																										self.fRec25[1] = self.fRec25[0];
																										self.fRec26[1] = self.fRec26[0];
																										self.fRec27[1] = self.fRec27[0];
																										self.fRec28[1] = self.fRec28[0];
																										self.fRec29[1] = self.fRec29[0];
																										self.fRec30[1] = self.fRec30[0];
																										self.fRec31[1] = self.fRec31[0];
																										self.fRec32[1] = self.fRec32[0];
																										self.fRec33[1] = self.fRec33[0];
																										self.fRec34[1] = self.fRec34[0];
																										self.fRec35[1] = self.fRec35[0];
																										self.fRec36[1] = self.fRec36[0];
																										self.fRec37[1] = self.fRec37[0];
																										self.fRec38[1] = self.fRec38[0];
																										self.fRec22[1] = self.fRec22[0];
																										self.fVec1[1] = self.fVec1[0];
																										self.fRec39[1] = self.fRec39[0];
																										if (iSlow65 & (iSlow67 & iSlow69)) | (iSlow65 & (iSlow67 & iSlow71)) != 0 {
																											self.fRec40[1] = self.fRec40[0];
																											self.fRec41[1] = self.fRec41[0];
																										}
																											if (iSlow65 & (iSlow67 & iSlow79)) | ((iSlow65 & (iSlow67 & iSlow77)) | ((iSlow65 & (iSlow67 & iSlow69)) | ((iSlow65 & (iSlow67 & iSlow71)) | (iSlow65 & (iSlow67 & iSlow75))))) != 0 {
																												self.fRec42[1] = self.fRec42[0];
																												self.fRec43[1] = self.fRec43[0];
																												self.fRec44[1] = self.fRec44[0];
																												self.fRec45[1] = self.fRec45[0];
																											}
																												if iSlow65 & (iSlow67 & iSlow75) != 0 {
																													self.fRec47[1] = self.fRec47[0];
																													self.fRec48[1] = self.fRec48[0];
																												}
																													if (iSlow65 & (iSlow67 & iSlow79)) | ((iSlow65 & (iSlow67 & iSlow77)) | (iSlow65 & (iSlow67 & iSlow75))) != 0 {
																														self.fRec49[1] = self.fRec49[0];
																														self.fVec2[1] = self.fVec2[0];
																													}
																														if iSlow65 & (iSlow67 & iSlow75) != 0 {
																															self.fRec46[1] = self.fRec46[0];
																														}
																															if (iSlow65 & (iSlow67 & iSlow79)) | (iSlow65 & (iSlow67 & iSlow77)) != 0 {
																																self.fRec50[1] = self.fRec50[0];
																																self.fRec51[1] = self.fRec51[0];
																															}
																																if iSlow65 & (iSlow67 & iSlow77) != 0 {
																																	self.fRec52[1] = self.fRec52[0];
																																	self.fRec53[1] = self.fRec53[0];
																																}
																																	self.iRec55[1] = self.iRec55[0];
																																	if iSlow65 != 0 {
																																		for j0 in (1..=3).rev() {
																																			self.fRec54[j0 as usize] = self.fRec54[(i32::wrapping_sub(j0, 1)) as usize];
																																		}
																																		self.fRec56[1] = self.fRec56[0];
																																		self.fRec57[1] = self.fRec57[0];
																																	}
																																		if iSlow65 & iSlow85 != 0 {
																																			self.fRec61[1] = self.fRec61[0];
																																			self.fRec62[1] = self.fRec62[0];
																																			self.fRec58[1] = self.fRec58[0];
																																			self.fRec59[1] = self.fRec59[0];
																																		}
																																			self.fRec63[1] = self.fRec63[0];
																																			self.fRec64[1] = self.fRec64[0];
																																			self.fRec65[1] = self.fRec65[0];
																																			self.fRec67[1] = self.fRec67[0];
																																			self.fRec68[1] = self.fRec68[0];
																																			self.fRec69[1] = self.fRec69[0];
																																			self.fRec70[1] = self.fRec70[0];
																																			self.fVec3[1] = self.fVec3[0];
																																			self.fRec66[1] = self.fRec66[0];
																																			self.fRec71[1] = self.fRec71[0];
																																			self.fRec72[1] = self.fRec72[0];
																																			self.fRec73[1] = self.fRec73[0];
																																			self.fRec75[1] = self.fRec75[0];
																																			self.fRec76[1] = self.fRec76[0];
																																			self.fRec77[1] = self.fRec77[0];
																																			self.fRec78[1] = self.fRec78[0];
																																			self.fVec4[1] = self.fVec4[0];
																																			self.fRec74[1] = self.fRec74[0];
																																			self.fRec79[1] = self.fRec79[0];
																																			self.fRec80[1] = self.fRec80[0];
																																			self.fRec81[1] = self.fRec81[0];
																																			self.fRec82[1] = self.fRec82[0];
																																			self.fRec83[1] = self.fRec83[0];
																																			self.fRec84[1] = self.fRec84[0];
																																			self.fRec85[1] = self.fRec85[0];
																																			self.fRec86[1] = self.fRec86[0];
																																			self.fRec87[1] = self.fRec87[0];
																																			self.fRec88[1] = self.fRec88[0];
																																			self.fRec89[1] = self.fRec89[0];
																																			self.fRec90[1] = self.fRec90[0];
																																			self.fRec91[1] = self.fRec91[0];
																																			self.fRec92[1] = self.fRec92[0];
																																			self.fRec93[1] = self.fRec93[0];
																																			self.fRec94[1] = self.fRec94[0];
																																			self.fRec95[1] = self.fRec95[0];
																																			self.fRec96[1] = self.fRec96[0];
																																			self.fRec97[1] = self.fRec97[0];
																																			self.fRec98[1] = self.fRec98[0];
																																			self.fRec99[1] = self.fRec99[0];
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
																																			self.fRec112[1] = self.fRec112[0];
																																			self.fRec113[1] = self.fRec113[0];
																																			self.fRec114[1] = self.fRec114[0];
																																			self.fRec115[1] = self.fRec115[0];
																																			self.fRec116[1] = self.fRec116[0];
																																			self.fRec117[1] = self.fRec117[0];
																																			self.fRec118[1] = self.fRec118[0];
																																			self.fRec119[1] = self.fRec119[0];
																																			self.fRec120[1] = self.fRec120[0];
																																			self.fRec121[1] = self.fRec121[0];
																																			self.fRec122[1] = self.fRec122[0];
																																			self.fRec123[1] = self.fRec123[0];
																																			self.fRec124[1] = self.fRec124[0];
																																			self.fRec125[1] = self.fRec125[0];
																																			self.fRec126[1] = self.fRec126[0];
																																			self.fRec127[1] = self.fRec127[0];
																																			self.fRec128[1] = self.fRec128[0];
																																			self.fRec129[1] = self.fRec129[0];
																																			self.fRec130[1] = self.fRec130[0];
																																			self.fRec131[1] = self.fRec131[0];
																																			self.fRec132[1] = self.fRec132[0];
																																			self.fRec133[1] = self.fRec133[0];
																																			self.fRec134[1] = self.fRec134[0];
																																		}
																																		self.fTempPerm0 = fTemp55;
																																		self.fTempPerm1 = fTemp56;
																																		self.fTempPerm2 = fTemp57;
																																		self.fTempPerm3 = fTemp58;
																																		self.fTempPerm4 = fTemp59;
																																		self.fTempPerm5 = fTemp60;
																																		self.fTempPerm6 = fTemp61;
																																		self.fTempPerm7 = fTemp62;
																																		self.fTempPerm8 = fTemp63;
																																		self.fTempPerm9 = fTemp64;
																																		self.fTempPerm10 = fTemp65;
																																		self.fTempPerm11 = fTemp66;
																																		self.fTempPerm12 = fTemp67;
																																		self.fTempPerm13 = fTemp68;
																																		self.fTempPerm14 = fTemp69;
																																		self.fTempPerm15 = fTemp70;
																																		self.fTempPerm16 = fTemp71;
																																		self.fTempPerm17 = fTemp72;
																																		self.fTempPerm18 = fTemp73;
																																		self.fTempPerm19 = fTemp74;
																																		self.fTempPerm20 = fTemp75;
																																		self.fTempPerm21 = fTemp76;
																																		self.fTempPerm22 = fTemp77;
																																		self.fTempPerm23 = fTemp78;
																																		self.fTempPerm24 = fTemp79;
																																		self.fTempPerm25 = fTemp80;
																																		self.fTempPerm26 = fTemp81;
																																		self.fTempPerm27 = fTemp82;
																																		self.fTempPerm28 = fTemp83;
																																		self.fTempPerm29 = fTemp84;
																																		self.fTempPerm30 = fTemp85;
																																		self.fTempPerm31 = fTemp86;
																																		self.fTempPerm32 = fTemp87;
																																		self.fTempPerm33 = fTemp88;
																																		self.fTempPerm34 = fTemp89;
																																		self.fTempPerm35 = fTemp90;
																																		self.fTempPerm36 = fTemp91;
																																		self.fTempPerm37 = fTemp92;
																																		self.fTempPerm38 = fTemp93;
																																		self.fTempPerm39 = fTemp94;
																																		self.fTempPerm40 = fTemp95;
																																		self.fTempPerm41 = fTemp96;
																																		self.fTempPerm42 = fTemp97;
																																		self.fTempPerm43 = fTemp98;
																																		self.fTempPerm44 = fTemp99;
																																		self.fTempPerm45 = fTemp100;
																																		self.fTempPerm46 = fTemp101;
																																		self.fTempPerm47 = fTemp102;
																																		self.fTempPerm48 = fTemp103;
																																		self.fTempPerm49 = fTemp104;
																																		self.fTempPerm50 = fTemp105;
																																		self.fTempPerm51 = fTemp106;
																																		self.fTempPerm52 = fTemp107;
																																		self.fTempPerm53 = fTemp108;
																																		self.fTempPerm54 = fTemp109;
																																		self.fTempPerm55 = fTemp110;
																																		self.fTempPerm56 = fTemp111;
																																		self.fTempPerm57 = fTemp112;
																																		self.fTempPerm58 = fTemp113;
																																		self.fTempPerm59 = fTemp114;
																																		self.fTempPerm60 = fTemp115;
																																		self.fTempPerm61 = fTemp116;
																																		self.fTempPerm62 = fTemp117;
																																		self.fTempPerm63 = fTemp118;
																																		self.fTempPerm64 = fTemp119;
																																		self.fTempPerm65 = fTemp120;
																																		self.fTempPerm66 = fTemp121;
																																		self.fTempPerm67 = fTemp122;
																																		self.fTempPerm68 = fTemp123;
																																		self.fTempPerm69 = fTemp124;
																																		self.fTempPerm70 = fTemp125;
																																		self.fTempPerm71 = fTemp126;
																																		self.fTempPerm72 = fTemp127;
																																		self.fTempPerm73 = fTemp128;
																																		self.fTempPerm74 = fTemp129;
																																		self.fTempPerm75 = fTemp130;
																																		self.fTempPerm76 = fTemp131;
																																		self.fTempPerm77 = fTemp132;
																																		self.fTempPerm78 = fTemp133;
																																		self.fTempPerm79 = fTemp134;
																																		self.fTempPerm80 = fTemp135;
																																		self.fTempPerm81 = fTemp136;
																																		self.fTempPerm82 = fTemp137;
																																		self.fTempPerm83 = fTemp138;
																																		self.fTempPerm84 = fTemp139;
																																		self.fTempPerm85 = fTemp140;
																																		self.fTempPerm86 = fTemp141;
																																		self.fTempPerm87 = fTemp142;
																																		self.fTempPerm88 = fTemp143;
																																		self.fTempPerm89 = fTemp144;
																																		self.fTempPerm90 = fTemp145;
																																		self.fTempPerm91 = fTemp146;
																																		self.fTempPerm92 = fTemp147;
																																		self.fTempPerm93 = fTemp148;
																																		self.fTempPerm94 = fTemp149;
																																		self.iTempPerm95 = iTemp150;
																																		self.fTempPerm96 = fTemp151;
																																		self.fTempPerm97 = fTemp152;
																																		self.fTempPerm98 = fTemp153;
																																		self.fTempPerm99 = fTemp154;
																																		self.fTempPerm100 = fTemp155;
																																		self.fTempPerm101 = fTemp156;
																																		self.fTempPerm102 = fTemp157;
																																		self.fTempPerm103 = fTemp158;
																																		self.fTempPerm104 = fTemp160;
																																		self.fTempPerm105 = fTemp161;
																																		self.fTempPerm106 = fTemp162;
																																		self.fTempPerm107 = fTemp163;
																																		self.fTempPerm108 = fTemp164;
																																		self.fTempPerm109 = fTemp165;
																																		self.fTempPerm110 = fTemp166;
																																		self.fTempPerm111 = fTemp167;
																																		self.fTempPerm112 = fTemp168;
																																		self.fTempPerm113 = fTemp169;
																																		self.fTempPerm114 = fTemp170;
																																		self.fTempPerm115 = fTemp171;
																																		self.fTempPerm116 = fTemp172;
																																		self.fTempPerm117 = fTemp173;
																																		self.fTempPerm118 = fTemp174;
																																		self.fTempPerm119 = fTemp175;
																																		self.fTempPerm120 = fTemp176;
																																		self.fTempPerm121 = fTemp177;
																																		self.fTempPerm122 = fTemp178;
																																		self.fTempPerm123 = fTemp179;
																																		self.fTempPerm124 = fTemp180;
																																		self.fTempPerm125 = fTemp181;
																																		self.fTempPerm126 = fTemp182;
																																		self.fTempPerm127 = fTemp183;
																																		self.fTempPerm128 = fTemp184;
																																		self.fTempPerm129 = fTemp185;
																																		self.fTempPerm130 = fTemp186;
																																		self.fTempPerm131 = fTemp187;
																																		self.fTempPerm132 = fTemp188;
																																		self.fTempPerm133 = fTemp189;
																																		self.fTempPerm134 = fTemp190;
																																		self.fTempPerm135 = fTemp191;
																																		self.fTempPerm136 = fTemp192;
																																		self.fTempPerm137 = fTemp193;
																																		self.fTempPerm138 = fTemp194;
																																		self.fTempPerm139 = fTemp195;
																																		self.fTempPerm140 = fTemp196;
																																		self.fTempPerm141 = fTemp197;
																																		self.fTempPerm142 = fTemp198;
																																		self.fTempPerm143 = fTemp199;
																																		self.fTempPerm144 = fTemp200;
																																		self.fTempPerm145 = fTemp201;
																																		self.fTempPerm146 = fTemp202;
																																		self.fTempPerm147 = fTemp203;
																																		self.fTempPerm148 = fTemp204;
																																		self.fTempPerm149 = fTemp205;
																																		self.fTempPerm150 = fTemp206;
																																		self.fTempPerm151 = fTemp207;
																																		self.fTempPerm152 = fTemp208;
																																		self.fTempPerm153 = fTemp209;
																																		self.fTempPerm154 = fTemp210;
																																		self.fTempPerm155 = fTemp211;
																																		self.fTempPerm156 = fTemp212;
																																		self.fTempPerm157 = fTemp213;
																																		self.fTempPerm158 = fTemp214;
																																	}

}

