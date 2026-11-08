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
	fConst5: F32,
	fHslider25: F32,
	fRec27: [F32;2],
	fHslider26: F32,
	fRec28: [F32;2],
	fHslider27: F32,
	fRec29: [F32;2],
	fHslider28: F32,
	fRec30: [F32;2],
	fHslider29: F32,
	fRec31: [F32;2],
	fHslider30: F32,
	fRec32: [F32;2],
	fCheckbox0: F32,
	fHslider31: F32,
	fRec33: [F32;2],
	fHslider32: F32,
	fRec34: [F32;2],
	fHslider33: F32,
	fRec22: [F32;2],
	fHslider34: F32,
	fRec35: [F32;2],
	fHslider35: F32,
	fRec36: [F32;2],
	fHslider36: F32,
	fRec37: [F32;2],
	fHslider37: F32,
	fRec38: [F32;2],
	fHslider38: F32,
	fRec40: [F32;2],
	fHslider39: F32,
	fRec41: [F32;2],
	fEntry0: F32,
	fEntry1: F32,
	fHslider40: F32,
	fRec42: [F32;2],
	fHslider41: F32,
	fRec43: [F32;2],
	fConst6: F32,
	fVec1: [F32;2],
	fRec39: [F32;2],
	fHslider42: F32,
	fRec44: [F32;2],
	fHslider43: F32,
	fRec45: [F32;2],
	fHslider44: F32,
	fRec46: [F32;2],
	fHslider45: F32,
	fRec48: [F32;2],
	fHslider46: F32,
	fRec49: [F32;2],
	fEntry2: F32,
	fEntry3: F32,
	fHslider47: F32,
	fRec50: [F32;2],
	fHslider48: F32,
	fRec51: [F32;2],
	fVec2: [F32;2],
	fRec47: [F32;2],
	fHslider49: F32,
	fRec52: [F32;2],
	fHslider50: F32,
	fRec53: [F32;2],
	fHslider51: F32,
	fRec54: [F32;2],
	fHslider52: F32,
	fRec55: [F32;2],
	fHslider53: F32,
	fRec56: [F32;2],
	fHslider54: F32,
	fRec57: [F32;2],
	iRec58: [i32;2],
	fConst7: F32,
	fConst8: F32,
	fHslider55: F32,
	fRec59: [F32;2],
	fHslider56: F32,
	fRec60: [F32;2],
	fHslider57: F32,
	fRec61: [F32;2],
	fHslider58: F32,
	fRec62: [F32;2],
	fHslider59: F32,
	fRec63: [F32;2],
	fHslider60: F32,
	fRec64: [F32;2],
	fHslider61: F32,
	fRec65: [F32;2],
	fHslider62: F32,
	fRec66: [F32;2],
	fHslider63: F32,
	fRec67: [F32;2],
	fHslider64: F32,
	fRec68: [F32;2],
	fHslider65: F32,
	fRec69: [F32;2],
	fHslider66: F32,
	fRec70: [F32;2],
	fHslider67: F32,
	fRec71: [F32;2],
	fHslider68: F32,
	fRec72: [F32;2],
	fConst9: F32,
	fHslider69: F32,
	fRec73: [F32;2],
	fHslider70: F32,
	fRec74: [F32;2],
	fHslider71: F32,
	fRec75: [F32;2],
	fHslider72: F32,
	fRec76: [F32;2],
	fHslider73: F32,
	fRec77: [F32;2],
	fHslider74: F32,
	fRec78: [F32;2],
	fHslider75: F32,
	fRec79: [F32;2],
	fHslider76: F32,
	fRec80: [F32;2],
	fHslider77: F32,
	fRec81: [F32;2],
	fHslider78: F32,
	fRec82: [F32;2],
	fHslider79: F32,
	fRec83: [F32;2],
	fHslider80: F32,
	fRec84: [F32;2],
	fHslider81: F32,
	fRec85: [F32;2],
	fHslider82: F32,
	fRec86: [F32;2],
	fEntry4: F32,
	fHslider83: F32,
	fRec87: [F32;2],
	fHslider84: F32,
	fRec88: [F32;2],
	fHslider85: F32,
	fRec89: [F32;2],
	fHslider86: F32,
	fRec90: [F32;2],
	fEntry5: F32,
	fHslider87: F32,
	fRec91: [F32;2],
	fHslider88: F32,
	fRec92: [F32;2],
	fHslider89: F32,
	fRec93: [F32;2],
	fHslider90: F32,
	fRec94: [F32;2],
	fHslider91: F32,
	fRec95: [F32;2],
	fHslider92: F32,
	fRec96: [F32;2],
	fHslider93: F32,
	fRec97: [F32;2],
	fHslider94: F32,
	fRec98: [F32;2],
	fHslider95: F32,
	fRec99: [F32;2],
	fHslider96: F32,
	fRec100: [F32;2],
	fHslider97: F32,
	fRec101: [F32;2],
	fHslider98: F32,
	fRec102: [F32;2],
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
			fConst5: 0.0,
			fHslider25: 0.0,
			fRec27: [0.0;2],
			fHslider26: 0.0,
			fRec28: [0.0;2],
			fHslider27: 0.0,
			fRec29: [0.0;2],
			fHslider28: 0.0,
			fRec30: [0.0;2],
			fHslider29: 0.0,
			fRec31: [0.0;2],
			fHslider30: 0.0,
			fRec32: [0.0;2],
			fCheckbox0: 0.0,
			fHslider31: 0.0,
			fRec33: [0.0;2],
			fHslider32: 0.0,
			fRec34: [0.0;2],
			fHslider33: 0.0,
			fRec22: [0.0;2],
			fHslider34: 0.0,
			fRec35: [0.0;2],
			fHslider35: 0.0,
			fRec36: [0.0;2],
			fHslider36: 0.0,
			fRec37: [0.0;2],
			fHslider37: 0.0,
			fRec38: [0.0;2],
			fHslider38: 0.0,
			fRec40: [0.0;2],
			fHslider39: 0.0,
			fRec41: [0.0;2],
			fEntry0: 0.0,
			fEntry1: 0.0,
			fHslider40: 0.0,
			fRec42: [0.0;2],
			fHslider41: 0.0,
			fRec43: [0.0;2],
			fConst6: 0.0,
			fVec1: [0.0;2],
			fRec39: [0.0;2],
			fHslider42: 0.0,
			fRec44: [0.0;2],
			fHslider43: 0.0,
			fRec45: [0.0;2],
			fHslider44: 0.0,
			fRec46: [0.0;2],
			fHslider45: 0.0,
			fRec48: [0.0;2],
			fHslider46: 0.0,
			fRec49: [0.0;2],
			fEntry2: 0.0,
			fEntry3: 0.0,
			fHslider47: 0.0,
			fRec50: [0.0;2],
			fHslider48: 0.0,
			fRec51: [0.0;2],
			fVec2: [0.0;2],
			fRec47: [0.0;2],
			fHslider49: 0.0,
			fRec52: [0.0;2],
			fHslider50: 0.0,
			fRec53: [0.0;2],
			fHslider51: 0.0,
			fRec54: [0.0;2],
			fHslider52: 0.0,
			fRec55: [0.0;2],
			fHslider53: 0.0,
			fRec56: [0.0;2],
			fHslider54: 0.0,
			fRec57: [0.0;2],
			iRec58: [0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fHslider55: 0.0,
			fRec59: [0.0;2],
			fHslider56: 0.0,
			fRec60: [0.0;2],
			fHslider57: 0.0,
			fRec61: [0.0;2],
			fHslider58: 0.0,
			fRec62: [0.0;2],
			fHslider59: 0.0,
			fRec63: [0.0;2],
			fHslider60: 0.0,
			fRec64: [0.0;2],
			fHslider61: 0.0,
			fRec65: [0.0;2],
			fHslider62: 0.0,
			fRec66: [0.0;2],
			fHslider63: 0.0,
			fRec67: [0.0;2],
			fHslider64: 0.0,
			fRec68: [0.0;2],
			fHslider65: 0.0,
			fRec69: [0.0;2],
			fHslider66: 0.0,
			fRec70: [0.0;2],
			fHslider67: 0.0,
			fRec71: [0.0;2],
			fHslider68: 0.0,
			fRec72: [0.0;2],
			fConst9: 0.0,
			fHslider69: 0.0,
			fRec73: [0.0;2],
			fHslider70: 0.0,
			fRec74: [0.0;2],
			fHslider71: 0.0,
			fRec75: [0.0;2],
			fHslider72: 0.0,
			fRec76: [0.0;2],
			fHslider73: 0.0,
			fRec77: [0.0;2],
			fHslider74: 0.0,
			fRec78: [0.0;2],
			fHslider75: 0.0,
			fRec79: [0.0;2],
			fHslider76: 0.0,
			fRec80: [0.0;2],
			fHslider77: 0.0,
			fRec81: [0.0;2],
			fHslider78: 0.0,
			fRec82: [0.0;2],
			fHslider79: 0.0,
			fRec83: [0.0;2],
			fHslider80: 0.0,
			fRec84: [0.0;2],
			fHslider81: 0.0,
			fRec85: [0.0;2],
			fHslider82: 0.0,
			fRec86: [0.0;2],
			fEntry4: 0.0,
			fHslider83: 0.0,
			fRec87: [0.0;2],
			fHslider84: 0.0,
			fRec88: [0.0;2],
			fHslider85: 0.0,
			fRec89: [0.0;2],
			fHslider86: 0.0,
			fRec90: [0.0;2],
			fEntry5: 0.0,
			fHslider87: 0.0,
			fRec91: [0.0;2],
			fHslider88: 0.0,
			fRec92: [0.0;2],
			fHslider89: 0.0,
			fRec93: [0.0;2],
			fHslider90: 0.0,
			fRec94: [0.0;2],
			fHslider91: 0.0,
			fRec95: [0.0;2],
			fHslider92: 0.0,
			fRec96: [0.0;2],
			fHslider93: 0.0,
			fRec97: [0.0;2],
			fHslider94: 0.0,
			fRec98: [0.0;2],
			fHslider95: 0.0,
			fRec99: [0.0;2],
			fHslider96: 0.0,
			fRec100: [0.0;2],
			fHslider97: 0.0,
			fRec101: [0.0;2],
			fHslider98: 0.0,
			fRec102: [0.0;2],
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
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/pole:author", r"Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", r"MIT-style STK-4.3 license");
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
		return 21;
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
		self.fHslider22 = 0.4;
		self.fHslider23 = 0.0;
		self.fHslider24 = 8e+01;
		self.fHslider25 = 0.0;
		self.fHslider26 = 0.333;
		self.fHslider27 = 0.0;
		self.fHslider28 = 4.0;
		self.fHslider29 = 0.0;
		self.fHslider30 = 36.0;
		self.fCheckbox0 = 0.0;
		self.fHslider31 = 0.0;
		self.fHslider32 = 5e+01;
		self.fHslider33 = 261.6256;
		self.fHslider34 = 0.8;
		self.fHslider35 = 0.0;
		self.fHslider36 = 0.6;
		self.fHslider37 = 0.0;
		self.fHslider38 = 0.0;
		self.fHslider39 = 2e+03;
		self.fEntry0 = 0.0;
		self.fEntry1 = 1.0;
		self.fHslider40 = 0.0;
		self.fHslider41 = 0.0;
		self.fHslider42 = 0.0;
		self.fHslider43 = 0.6;
		self.fHslider44 = 0.0;
		self.fHslider45 = 0.0;
		self.fHslider46 = 2e+03;
		self.fEntry2 = 2.0;
		self.fEntry3 = 1.0;
		self.fHslider47 = 0.0;
		self.fHslider48 = 0.0;
		self.fHslider49 = 0.0;
		self.fHslider50 = 0.0;
		self.fHslider51 = 0.0;
		self.fHslider52 = 0.0;
		self.fHslider53 = 0.0;
		self.fHslider54 = 0.5;
		self.fHslider55 = 0.0;
		self.fHslider56 = 0.0;
		self.fHslider57 = 0.0;
		self.fHslider58 = 0.0;
		self.fHslider59 = 2.5e+02;
		self.fHslider60 = 0.0;
		self.fHslider61 = 0.2;
		self.fHslider62 = 0.0;
		self.fHslider63 = 0.31;
		self.fHslider64 = 0.0;
		self.fHslider65 = 0.0;
		self.fHslider66 = 0.0;
		self.fHslider67 = 0.6;
		self.fHslider68 = 0.0;
		self.fHslider69 = 0.0;
		self.fHslider70 = 0.0;
		self.fHslider71 = 0.0;
		self.fHslider72 = 0.3;
		self.fHslider73 = 0.0;
		self.fHslider74 = 0.0;
		self.fHslider75 = 0.0;
		self.fHslider76 = 0.2;
		self.fHslider77 = 0.0;
		self.fHslider78 = 4e+02;
		self.fHslider79 = 0.0;
		self.fHslider80 = 0.4;
		self.fHslider81 = 0.0;
		self.fHslider82 = 0.5;
		self.fEntry4 = 0.0;
		self.fHslider83 = 0.0;
		self.fHslider84 = 0.2;
		self.fHslider85 = 0.0;
		self.fHslider86 = 2.5e+03;
		self.fEntry5 = 0.0;
		self.fHslider87 = 0.0;
		self.fHslider88 = 0.0;
		self.fHslider89 = 0.0;
		self.fHslider90 = 0.0;
		self.fHslider91 = 0.0;
		self.fHslider92 = 0.0;
		self.fHslider93 = 0.0;
		self.fHslider94 = 1.0;
		self.fHslider95 = 0.0;
		self.fHslider96 = 0.3;
		self.fHslider97 = 0.0;
		self.fHslider98 = 3e+02;
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
			self.fRec22[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec35[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec36[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec37[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec38[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec40[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec41[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec42[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec43[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fVec1[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec39[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec44[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec45[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec46[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec48[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec49[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec50[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec51[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fVec2[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec47[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec52[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec53[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec54[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec55[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec56[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec57[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.iRec58[l61 as usize] = 0;
		}
		for l62 in 0..2 {
			self.fRec59[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec60[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec61[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec62[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec63[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec64[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec65[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec66[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec67[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec68[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec69[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec70[l73 as usize] = 0.0;
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
			self.fRec74[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec75[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec76[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec77[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec78[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec79[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec80[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec81[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec82[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec83[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec84[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec85[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec86[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec87[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec88[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec89[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec90[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec91[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec92[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec93[l96 as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec94[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec95[l98 as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec96[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec97[l100 as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fRec98[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec99[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec100[l103 as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec101[l104 as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec102[l105 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 - 44.1 / fConst0;
		self.fConst2 = 1.0 / fConst0;
		self.fConst3 = 6.907755 / fConst0;
		self.fConst4 = 0.45 * fConst0;
		self.fConst5 = 3.0 / fConst0;
		self.fConst6 = 4.41e+04 / fConst0;
		self.fConst7 = 628.31854 / fConst0;
		self.fConst8 = 314.15927 / fConst0;
		self.fConst9 = 33.333332 / fConst0;
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
		ui_interface.declare(Some(ParamIndex(12)), "1", "");
		ui_interface.declare(Some(ParamIndex(12)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Pitch Mod", ParamIndex(12), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(13)), "1", "");
		ui_interface.declare(Some(ParamIndex(13)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(13)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(13), 5e+01, 2e+01, 2e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(14)), "2", "");
		ui_interface.add_check_button("Keytrack", ParamIndex(14));
		ui_interface.declare(Some(ParamIndex(15)), "3", "");
		ui_interface.add_horizontal_slider("Shape", ParamIndex(15), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(16)), "3", "");
		ui_interface.declare(Some(ParamIndex(16)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Shape Mod", ParamIndex(16), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(17)), "4", "");
		ui_interface.declare(Some(ParamIndex(17)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Drop Mod", ParamIndex(17), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(18)), "4", "");
		ui_interface.declare(Some(ParamIndex(18)), "unit", "st");
		ui_interface.add_horizontal_slider("Drop", ParamIndex(18), 36.0, 0.0, 72.0, 0.01);
		ui_interface.declare(Some(ParamIndex(19)), "5", "");
		ui_interface.declare(Some(ParamIndex(19)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Split Mod", ParamIndex(19), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "5", "");
		ui_interface.declare(Some(ParamIndex(20)), "tooltip", "Share of the Drop that is fast (the rest is slow)");
		ui_interface.add_horizontal_slider("Split", ParamIndex(20), 0.333, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "6", "");
		ui_interface.declare(Some(ParamIndex(21)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Fast Time Mod", ParamIndex(21), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(22)), "6", "");
		ui_interface.declare(Some(ParamIndex(22)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(22)), "unit", "ms");
		ui_interface.add_horizontal_slider("Fast Time", ParamIndex(22), 4.0, 1.0, 2e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(23)), "7", "");
		ui_interface.declare(Some(ParamIndex(23)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Slow Time Mod", ParamIndex(23), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "7", "");
		ui_interface.declare(Some(ParamIndex(24)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(24)), "unit", "ms");
		ui_interface.add_horizontal_slider("Slow Time", ParamIndex(24), 8e+01, 2e+01, 5e+02, 0.1);
		ui_interface.declare(Some(ParamIndex(25)), "8", "");
		ui_interface.declare(Some(ParamIndex(25)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Attack Mod", ParamIndex(25), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(26)), "8", "");
		ui_interface.declare(Some(ParamIndex(26)), "unit", "ms");
		ui_interface.add_horizontal_slider("Attack", ParamIndex(26), 0.0, 0.0, 1e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(27)), "9", "");
		ui_interface.declare(Some(ParamIndex(27)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(27), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(28)), "9", "");
		ui_interface.declare(Some(ParamIndex(28)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(28)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(28), 7e+02, 2e+01, 4e+03, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("Body");
		ui_interface.declare(Some(ParamIndex(29)), "0", "");
		ui_interface.declare(Some(ParamIndex(29)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(29), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(30)), "1", "");
		ui_interface.declare(Some(ParamIndex(30)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Offset Mod", ParamIndex(30), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "1", "");
		ui_interface.declare(Some(ParamIndex(31)), "unit", "st");
		ui_interface.add_horizontal_slider("Offset", ParamIndex(31), 0.0, -24.0, 24.0, 0.01);
		ui_interface.declare(Some(ParamIndex(32)), "2", "");
		ui_interface.add_horizontal_slider("Material", ParamIndex(32), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(33)), "2", "");
		ui_interface.declare(Some(ParamIndex(33)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Material Mod", ParamIndex(33), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(34)), "3", "");
		ui_interface.declare(Some(ParamIndex(34)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Strike Mod", ParamIndex(34), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(35)), "3", "");
		ui_interface.declare(Some(ParamIndex(35)), "tooltip", "Mallet hardness, and brightness with it");
		ui_interface.add_horizontal_slider("Strike", ParamIndex(35), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(36)), "4", "");
		ui_interface.declare(Some(ParamIndex(36)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(36), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(37)), "4", "");
		ui_interface.declare(Some(ParamIndex(37)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(37)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(37), 4e+02, 2e+01, 4e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(38)), "5", "");
		ui_interface.declare(Some(ParamIndex(38)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Damping Mod", ParamIndex(38), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(39)), "5", "");
		ui_interface.declare(Some(ParamIndex(39)), "tooltip", "Above the middle the upper modes die sooner and start quieter; below, they ring brighter");
		ui_interface.add_horizontal_slider("Damping", ParamIndex(39), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(40)), "6", "");
		ui_interface.add_horizontal_slider("Vel Brightness", ParamIndex(40), 0.4, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(41)), "6", "");
		ui_interface.declare(Some(ParamIndex(41)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Brightness Mod", ParamIndex(41), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(42)), "7", "");
		ui_interface.add_horizontal_slider("Vel Bend", ParamIndex(42), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(43)), "7", "");
		ui_interface.declare(Some(ParamIndex(43)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Bend Mod", ParamIndex(43), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(44)), "8", "");
		ui_interface.add_horizontal_slider("Vel Decay", ParamIndex(44), 0.2, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(45)), "8", "");
		ui_interface.declare(Some(ParamIndex(45)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Decay Mod", ParamIndex(45), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("Noise");
		ui_interface.declare(Some(ParamIndex(46)), "0", "");
		ui_interface.declare(Some(ParamIndex(46)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(46), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(47)), "10", "");
		ui_interface.declare(Some(ParamIndex(47)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Snap Mod", ParamIndex(47), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(48)), "10", "");
		ui_interface.declare(Some(ParamIndex(48)), "tooltip", "How much quicker the fast decay is: 0 the same, 1 a hundredth");
		ui_interface.add_horizontal_slider("Snap", ParamIndex(48), 0.31, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(49)), "11", "");
		ui_interface.add_horizontal_slider("Tail Level", ParamIndex(49), 0.2, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(50)), "11", "");
		ui_interface.declare(Some(ParamIndex(50)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Tail Level Mod", ParamIndex(50), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(51)), "12", "");
		ui_interface.add_horizontal_slider("Clap", ParamIndex(51), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(52)), "12", "");
		ui_interface.declare(Some(ParamIndex(52)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Clap Mod", ParamIndex(52), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(53)), "13", "");
		ui_interface.add_horizontal_slider("Density", ParamIndex(53), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(54)), "13", "");
		ui_interface.declare(Some(ParamIndex(54)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Density Mod", ParamIndex(54), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(55)), "14", "");
		ui_interface.add_horizontal_slider("Crush", ParamIndex(55), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(56)), "14", "");
		ui_interface.declare(Some(ParamIndex(56)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Crush Mod", ParamIndex(56), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(57)), "15", "");
		ui_interface.add_horizontal_slider("Vel Level", ParamIndex(57), 0.6, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(58)), "15", "");
		ui_interface.declare(Some(ParamIndex(58)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Level Mod", ParamIndex(58), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(59)), "16", "");
		ui_interface.declare(Some(ParamIndex(59)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Loop Pitch Mod", ParamIndex(59), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(60)), "16", "");
		ui_interface.declare(Some(ParamIndex(60)), "tooltip", "Playback pitch of the loop only");
		ui_interface.declare(Some(ParamIndex(60)), "unit", "st");
		ui_interface.add_horizontal_slider("Loop Pitch", ParamIndex(60), 0.0, -24.0, 24.0, 0.01);
		ui_interface.declare(Some(ParamIndex(61)), "1", "");
		ui_interface.declare(Some(ParamIndex(61)), "style", "menu{'White':0;'Pink':1;'Dust':2;'Sample & Hold':3;'Metal':4}");
		ui_interface.add_num_entry("Generator", ParamIndex(61), 0.0, 0.0, 4.0, 1.0);
		ui_interface.declare(Some(ParamIndex(62)), "2", "");
		ui_interface.declare(Some(ParamIndex(62)), "style", "menu{'Steady: 018 SH101 Noise':0;'Steady: 023 RF Nomad Tuning Noise':1;'Steady: 028 4ms SMR Resonator Constant':2;'Steady: 145 DSE Static Constant':3;'Steady: 148 DSE Static Constant':4;'Steady: 149 DSE Static Constant':5;'Steady: 156 DSE Off Station':6;'Steady: 192 Sony Static Constant':7;'Steady: 197 Sony Static Constant with Warble Tone':8;'Steady: 214 Yaesu Transceiver Static Constant':9;'Steady: 232B Yaesu Transceiver Static Constant with Tone':10;'Steady: Beehive Contact':11;'Steady: CD Glitch-bounce-6':12;'Steady: CD Glitch-bounce-7':13;'Steady: Noise-bounce-3':14;'Steady: Noise':15;'Steady: Quiet Street':16;'Crackle: 161 DSE Static with Tone':17;'Crackle: Honey Badger-bounce-2':18;'Crackle: StaticNoise_FX_SP02':19;'Crackle: StaticNoise_FX_SP03':20;'Crackle: StaticNoise_FX_SP04':21;'Crackle: StaticNoise_FX_SP05':22;'Crackle: StaticNoise_FX_SP16':23;'Crackle: StaticNoise_FX_SP17':24;'Crackle: StaticNoise_FX_SP22':25;'Crackle: StaticNoise_FX_SP23':26;'Modulated: Blue Jays Woodpecker-bounce-1':27;'Modulated: CD_Glitch_02_SP':28;'Modulated: CD_Glitch_13_SP':29;'Modulated: CD_Glitch_32_SP':30;'Modulated: CD_Glitch_39_SP':31;'Modulated: Circuit Bent-bounce-1':32;'Modulated: Jungle-bounce-1':33;'Modulated: StaticNoise_FX_SP13':34;'Modulated: StaticNoise_FX_SP18':35;'Modulated: StaticNoise_FX_SP21':36;'Modulated: StaticNoise_FX_SP25':37;'Modulated: StaticNoise_FX_SP26':38;'Modulated: StaticNoise_FX_SP27':39;'Modulated: StaticNoise_FX_SP28':40;'Modulated: StaticNoise_FX_SP29':41;'Modulated: StaticNoise_FX_SP31':42;'Modulated: StaticNoise_FX_SP33':43;'Modulated: iPad':44}");
		ui_interface.add_num_entry("Loop", ParamIndex(62), 0.0, 0.0, 44.0, 1.0);
		ui_interface.declare(Some(ParamIndex(63)), "3", "");
		ui_interface.declare(Some(ParamIndex(63)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Blend Mod", ParamIndex(63), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(64)), "3", "");
		ui_interface.declare(Some(ParamIndex(64)), "tooltip", "0 = generator, 1 = loop");
		ui_interface.add_horizontal_slider("Blend", ParamIndex(64), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(65)), "4", "");
		ui_interface.declare(Some(ParamIndex(65)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Bandpass Mod", ParamIndex(65), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(66)), "4", "");
		ui_interface.declare(Some(ParamIndex(66)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(66)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Bandpass", ParamIndex(66), 2.5e+03, 1e+02, 1.2e+04, 0.1);
		ui_interface.declare(Some(ParamIndex(67)), "5", "");
		ui_interface.declare(Some(ParamIndex(67)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Q Mod", ParamIndex(67), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(68)), "5", "");
		ui_interface.declare(Some(ParamIndex(68)), "tooltip", "0 = flat (only the highpass acts); 1 = 24 dB/oct skirts");
		ui_interface.add_horizontal_slider("Q", ParamIndex(68), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(69)), "6", "");
		ui_interface.declare(Some(ParamIndex(69)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Highpass Mod", ParamIndex(69), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(70)), "6", "");
		ui_interface.declare(Some(ParamIndex(70)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(70)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Highpass", ParamIndex(70), 3e+02, 2e+01, 8e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(71)), "7", "");
		ui_interface.add_horizontal_slider("Env Amount", ParamIndex(71), 0.2, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(72)), "7", "");
		ui_interface.declare(Some(ParamIndex(72)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Env Amount Mod", ParamIndex(72), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(73)), "8", "");
		ui_interface.declare(Some(ParamIndex(73)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Attack Mod", ParamIndex(73), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(74)), "8", "");
		ui_interface.declare(Some(ParamIndex(74)), "unit", "ms");
		ui_interface.add_horizontal_slider("Attack", ParamIndex(74), 0.0, 0.0, 1e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(75)), "9", "");
		ui_interface.declare(Some(ParamIndex(75)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(75), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(76)), "9", "");
		ui_interface.declare(Some(ParamIndex(76)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(76)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(76), 2.5e+02, 2e+01, 4e+03, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "4", "");
		ui_interface.open_horizontal_box("Sample 1");
		ui_interface.declare(Some(ParamIndex(77)), "0", "");
		ui_interface.declare(Some(ParamIndex(77)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(77), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(78)), "1", "");
		ui_interface.declare(Some(ParamIndex(78)), "style", "menu{'Thump':0;'Knock':1;'Spark':2}");
		ui_interface.add_num_entry("Set", ParamIndex(78), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(79)), "2", "");
		ui_interface.add_num_entry("Sample", ParamIndex(79), 1.0, 1.0, 58.0, 1.0);
		ui_interface.declare(Some(ParamIndex(80)), "3", "");
		ui_interface.declare(Some(ParamIndex(80)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Pitch Mod", ParamIndex(80), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(81)), "3", "");
		ui_interface.declare(Some(ParamIndex(81)), "unit", "st");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(81), 0.0, -24.0, 24.0, 0.01);
		ui_interface.declare(Some(ParamIndex(82)), "4", "");
		ui_interface.declare(Some(ParamIndex(82)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(82), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(83)), "4", "");
		ui_interface.declare(Some(ParamIndex(83)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(83)), "tooltip", "Fades the hit out; the top of the range plays it whole");
		ui_interface.declare(Some(ParamIndex(83)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(83), 2e+03, 5.0, 2e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(84)), "5", "");
		ui_interface.declare(Some(ParamIndex(84)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Body Send Mod", ParamIndex(84), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(85)), "5", "");
		ui_interface.declare(Some(ParamIndex(85)), "tooltip", "How hard this hit strikes the Body. As the sends rise, the Body's own mallet fades out");
		ui_interface.add_horizontal_slider("Body Send", ParamIndex(85), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(86)), "6", "");
		ui_interface.add_horizontal_slider("Vel Level", ParamIndex(86), 0.6, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(87)), "6", "");
		ui_interface.declare(Some(ParamIndex(87)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Level Mod", ParamIndex(87), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "5", "");
		ui_interface.open_horizontal_box("Sample 2");
		ui_interface.declare(Some(ParamIndex(88)), "0", "");
		ui_interface.declare(Some(ParamIndex(88)), "scale", "gain");
		ui_interface.add_horizontal_slider("Level", ParamIndex(88), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(89)), "1", "");
		ui_interface.declare(Some(ParamIndex(89)), "style", "menu{'Thump':0;'Knock':1;'Spark':2}");
		ui_interface.add_num_entry("Set", ParamIndex(89), 2.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(90)), "2", "");
		ui_interface.add_num_entry("Sample", ParamIndex(90), 1.0, 1.0, 58.0, 1.0);
		ui_interface.declare(Some(ParamIndex(91)), "3", "");
		ui_interface.declare(Some(ParamIndex(91)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Pitch Mod", ParamIndex(91), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(92)), "3", "");
		ui_interface.declare(Some(ParamIndex(92)), "unit", "st");
		ui_interface.add_horizontal_slider("Pitch", ParamIndex(92), 0.0, -24.0, 24.0, 0.01);
		ui_interface.declare(Some(ParamIndex(93)), "4", "");
		ui_interface.declare(Some(ParamIndex(93)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Decay Mod", ParamIndex(93), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(94)), "4", "");
		ui_interface.declare(Some(ParamIndex(94)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(94)), "tooltip", "Fades the hit out; the top of the range plays it whole");
		ui_interface.declare(Some(ParamIndex(94)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(94), 2e+03, 5.0, 2e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(95)), "5", "");
		ui_interface.declare(Some(ParamIndex(95)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Body Send Mod", ParamIndex(95), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(96)), "5", "");
		ui_interface.declare(Some(ParamIndex(96)), "tooltip", "How hard this hit strikes the Body. As the sends rise, the Body's own mallet fades out");
		ui_interface.add_horizontal_slider("Body Send", ParamIndex(96), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(97)), "6", "");
		ui_interface.add_horizontal_slider("Vel Level", ParamIndex(97), 0.6, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(98)), "6", "");
		ui_interface.declare(Some(ParamIndex(98)), "tooltip", "Envelope send, -1..1 of the knob's travel");
		ui_interface.add_horizontal_slider("Vel Level Mod", ParamIndex(98), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Envelope");
		ui_interface.declare(Some(ParamIndex(99)), "0", "");
		ui_interface.declare(Some(ParamIndex(99)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(99)), "unit", "ms");
		ui_interface.add_horizontal_slider("Attack", ParamIndex(99), 0.0, 0.0, 1e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(100)), "1", "");
		ui_interface.declare(Some(ParamIndex(100)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(100)), "unit", "ms");
		ui_interface.add_horizontal_slider("Decay", ParamIndex(100), 3e+02, 5.0, 4e+03, 0.1);
		ui_interface.declare(Some(ParamIndex(101)), "2", "");
		ui_interface.declare(Some(ParamIndex(101)), "tooltip", "0: a straight fall; 1: snaps down, then tails off");
		ui_interface.add_horizontal_slider("Curve", ParamIndex(101), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(102)), "3", "");
		ui_interface.declare(Some(ParamIndex(102)), "tooltip", "How much the hit's velocity scales the envelope");
		ui_interface.add_horizontal_slider("Velocity", ParamIndex(102), 0.5, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.add_horizontal_slider("freq", ParamIndex(103), 261.6256, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("gain", ParamIndex(104), 0.8, 0.0, 1.0, 0.01);
		ui_interface.add_button("gate", ParamIndex(105));
		ui_interface.add_horizontal_slider("seed", ParamIndex(106), 0.0, 0.0, 1.0, 1e-06);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			105 => Some(self.fButton0),
			14 => Some(self.fCheckbox0),
			78 => Some(self.fEntry0),
			79 => Some(self.fEntry1),
			89 => Some(self.fEntry2),
			90 => Some(self.fEntry3),
			61 => Some(self.fEntry4),
			62 => Some(self.fEntry5),
			2 => Some(self.fHslider0),
			3 => Some(self.fHslider1),
			28 => Some(self.fHslider10),
			5 => Some(self.fHslider11),
			4 => Some(self.fHslider12),
			7 => Some(self.fHslider13),
			6 => Some(self.fHslider14),
			25 => Some(self.fHslider15),
			26 => Some(self.fHslider16),
			11 => Some(self.fHslider17),
			10 => Some(self.fHslider18),
			16 => Some(self.fHslider19),
			100 => Some(self.fHslider2),
			15 => Some(self.fHslider20),
			9 => Some(self.fHslider21),
			8 => Some(self.fHslider22),
			23 => Some(self.fHslider23),
			24 => Some(self.fHslider24),
			19 => Some(self.fHslider25),
			20 => Some(self.fHslider26),
			21 => Some(self.fHslider27),
			22 => Some(self.fHslider28),
			17 => Some(self.fHslider29),
			99 => Some(self.fHslider3),
			18 => Some(self.fHslider30),
			12 => Some(self.fHslider31),
			13 => Some(self.fHslider32),
			103 => Some(self.fHslider33),
			1 => Some(self.fHslider34),
			87 => Some(self.fHslider35),
			86 => Some(self.fHslider36),
			77 => Some(self.fHslider37),
			82 => Some(self.fHslider38),
			83 => Some(self.fHslider39),
			101 => Some(self.fHslider4),
			80 => Some(self.fHslider40),
			81 => Some(self.fHslider41),
			98 => Some(self.fHslider42),
			97 => Some(self.fHslider43),
			88 => Some(self.fHslider44),
			93 => Some(self.fHslider45),
			94 => Some(self.fHslider46),
			91 => Some(self.fHslider47),
			92 => Some(self.fHslider48),
			95 => Some(self.fHslider49),
			104 => Some(self.fHslider5),
			96 => Some(self.fHslider50),
			84 => Some(self.fHslider51),
			85 => Some(self.fHslider52),
			34 => Some(self.fHslider53),
			35 => Some(self.fHslider54),
			29 => Some(self.fHslider55),
			52 => Some(self.fHslider56),
			51 => Some(self.fHslider57),
			75 => Some(self.fHslider58),
			76 => Some(self.fHslider59),
			102 => Some(self.fHslider6),
			50 => Some(self.fHslider60),
			49 => Some(self.fHslider61),
			47 => Some(self.fHslider62),
			48 => Some(self.fHslider63),
			73 => Some(self.fHslider64),
			74 => Some(self.fHslider65),
			58 => Some(self.fHslider66),
			57 => Some(self.fHslider67),
			46 => Some(self.fHslider68),
			33 => Some(self.fHslider69),
			0 => Some(self.fHslider7),
			32 => Some(self.fHslider70),
			43 => Some(self.fHslider71),
			42 => Some(self.fHslider72),
			30 => Some(self.fHslider73),
			31 => Some(self.fHslider74),
			45 => Some(self.fHslider75),
			44 => Some(self.fHslider76),
			36 => Some(self.fHslider77),
			37 => Some(self.fHslider78),
			41 => Some(self.fHslider79),
			106 => Some(self.fHslider8),
			40 => Some(self.fHslider80),
			38 => Some(self.fHslider81),
			39 => Some(self.fHslider82),
			72 => Some(self.fHslider83),
			71 => Some(self.fHslider84),
			65 => Some(self.fHslider85),
			66 => Some(self.fHslider86),
			59 => Some(self.fHslider87),
			60 => Some(self.fHslider88),
			63 => Some(self.fHslider89),
			27 => Some(self.fHslider9),
			64 => Some(self.fHslider90),
			56 => Some(self.fHslider91),
			55 => Some(self.fHslider92),
			54 => Some(self.fHslider93),
			53 => Some(self.fHslider94),
			67 => Some(self.fHslider95),
			68 => Some(self.fHslider96),
			69 => Some(self.fHslider97),
			70 => Some(self.fHslider98),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			105 => { self.fButton0 = value }
			14 => { self.fCheckbox0 = value }
			78 => { self.fEntry0 = value }
			79 => { self.fEntry1 = value }
			89 => { self.fEntry2 = value }
			90 => { self.fEntry3 = value }
			61 => { self.fEntry4 = value }
			62 => { self.fEntry5 = value }
			2 => { self.fHslider0 = value }
			3 => { self.fHslider1 = value }
			28 => { self.fHslider10 = value }
			5 => { self.fHslider11 = value }
			4 => { self.fHslider12 = value }
			7 => { self.fHslider13 = value }
			6 => { self.fHslider14 = value }
			25 => { self.fHslider15 = value }
			26 => { self.fHslider16 = value }
			11 => { self.fHslider17 = value }
			10 => { self.fHslider18 = value }
			16 => { self.fHslider19 = value }
			100 => { self.fHslider2 = value }
			15 => { self.fHslider20 = value }
			9 => { self.fHslider21 = value }
			8 => { self.fHslider22 = value }
			23 => { self.fHslider23 = value }
			24 => { self.fHslider24 = value }
			19 => { self.fHslider25 = value }
			20 => { self.fHslider26 = value }
			21 => { self.fHslider27 = value }
			22 => { self.fHslider28 = value }
			17 => { self.fHslider29 = value }
			99 => { self.fHslider3 = value }
			18 => { self.fHslider30 = value }
			12 => { self.fHslider31 = value }
			13 => { self.fHslider32 = value }
			103 => { self.fHslider33 = value }
			1 => { self.fHslider34 = value }
			87 => { self.fHslider35 = value }
			86 => { self.fHslider36 = value }
			77 => { self.fHslider37 = value }
			82 => { self.fHslider38 = value }
			83 => { self.fHslider39 = value }
			101 => { self.fHslider4 = value }
			80 => { self.fHslider40 = value }
			81 => { self.fHslider41 = value }
			98 => { self.fHslider42 = value }
			97 => { self.fHslider43 = value }
			88 => { self.fHslider44 = value }
			93 => { self.fHslider45 = value }
			94 => { self.fHslider46 = value }
			91 => { self.fHslider47 = value }
			92 => { self.fHslider48 = value }
			95 => { self.fHslider49 = value }
			104 => { self.fHslider5 = value }
			96 => { self.fHslider50 = value }
			84 => { self.fHslider51 = value }
			85 => { self.fHslider52 = value }
			34 => { self.fHslider53 = value }
			35 => { self.fHslider54 = value }
			29 => { self.fHslider55 = value }
			52 => { self.fHslider56 = value }
			51 => { self.fHslider57 = value }
			75 => { self.fHslider58 = value }
			76 => { self.fHslider59 = value }
			102 => { self.fHslider6 = value }
			50 => { self.fHslider60 = value }
			49 => { self.fHslider61 = value }
			47 => { self.fHslider62 = value }
			48 => { self.fHslider63 = value }
			73 => { self.fHslider64 = value }
			74 => { self.fHslider65 = value }
			58 => { self.fHslider66 = value }
			57 => { self.fHslider67 = value }
			46 => { self.fHslider68 = value }
			33 => { self.fHslider69 = value }
			0 => { self.fHslider7 = value }
			32 => { self.fHslider70 = value }
			43 => { self.fHslider71 = value }
			42 => { self.fHslider72 = value }
			30 => { self.fHslider73 = value }
			31 => { self.fHslider74 = value }
			45 => { self.fHslider75 = value }
			44 => { self.fHslider76 = value }
			36 => { self.fHslider77 = value }
			37 => { self.fHslider78 = value }
			41 => { self.fHslider79 = value }
			106 => { self.fHslider8 = value }
			40 => { self.fHslider80 = value }
			38 => { self.fHslider81 = value }
			39 => { self.fHslider82 = value }
			72 => { self.fHslider83 = value }
			71 => { self.fHslider84 = value }
			65 => { self.fHslider85 = value }
			66 => { self.fHslider86 = value }
			59 => { self.fHslider87 = value }
			60 => { self.fHslider88 = value }
			63 => { self.fHslider89 = value }
			27 => { self.fHslider9 = value }
			64 => { self.fHslider90 = value }
			56 => { self.fHslider91 = value }
			55 => { self.fHslider92 = value }
			54 => { self.fHslider93 = value }
			53 => { self.fHslider94 = value }
			67 => { self.fHslider95 = value }
			68 => { self.fHslider96 = value }
			69 => { self.fHslider97 = value }
			70 => { self.fHslider98 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10, outputs11, outputs12, outputs13, outputs14, outputs15, outputs16, outputs17, outputs18, outputs19, outputs20) = if let [outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10, outputs11, outputs12, outputs13, outputs14, outputs15, outputs16, outputs17, outputs18, outputs19, outputs20, ..] = outputs {
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
			(outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, outputs8, outputs9, outputs10, outputs11, outputs12, outputs13, outputs14, outputs15, outputs16, outputs17, outputs18, outputs19, outputs20)
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
		let mut fSlow41: F32 = 43758.547 * F32::sin(fSlow10 + 156.466);
		let mut fSlow42: F32 = 0.0057762265 * (2.0 * (fSlow41 - F32::floor(fSlow41)) + -1.0);
		let mut iSlow43: i32 = (self.fCheckbox0) as i32;
		let mut fSlow44: F32 = self.fHslider31;
		let mut fSlow45: F32 = self.fHslider32;
		let mut fSlow46: F32 = 0.0038222559 * self.fHslider33;
		let mut fSlow47: F32 = self.fHslider34;
		let mut fSlow48: F32 = self.fHslider35;
		let mut fSlow49: F32 = self.fHslider36;
		let mut fSlow50: F32 = self.fHslider37;
		let mut fSlow51: F32 = self.fHslider38;
		let mut fSlow52: F32 = self.fHslider39;
		let mut fSlow53: F32 = self.fEntry0;
		let mut iSlow54: i32 = (fSlow53 == 2.0) as i32;
		let mut iSlow55: i32 = (fSlow53 == 1.0) as i32;
		let mut iSlow56: i32 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_mul(58, iSlow55), i32::wrapping_mul(113, iSlow54)), (i32::wrapping_add((self.fEntry1) as i32, -1)) % (i32::wrapping_add(i32::wrapping_mul(58, (fSlow53 == 0.0) as i32), i32::wrapping_add(i32::wrapping_mul(55, iSlow55), i32::wrapping_mul(51, iSlow54)))));
		let mut fSlow57: F32 = 43758.547 * F32::sin(fSlow10 + 938.796);
		let mut fSlow58: F32 = 0.1 * (2.0 * (fSlow57 - F32::floor(fSlow57)) + -1.0);
		let mut fSlow59: F32 = self.fHslider40;
		let mut fSlow60: F32 = self.fHslider41;
		let mut fSlow61: F32 = self.fHslider42;
		let mut fSlow62: F32 = self.fHslider43;
		let mut fSlow63: F32 = self.fHslider44;
		let mut fSlow64: F32 = self.fHslider45;
		let mut fSlow65: F32 = self.fHslider46;
		let mut fSlow66: F32 = self.fEntry2;
		let mut iSlow67: i32 = (fSlow66 == 2.0) as i32;
		let mut iSlow68: i32 = (fSlow66 == 1.0) as i32;
		let mut iSlow69: i32 = i32::wrapping_add(i32::wrapping_add(i32::wrapping_mul(58, iSlow68), i32::wrapping_mul(113, iSlow67)), (i32::wrapping_add((self.fEntry3) as i32, -1)) % (i32::wrapping_add(i32::wrapping_mul(58, (fSlow66 == 0.0) as i32), i32::wrapping_add(i32::wrapping_mul(55, iSlow68), i32::wrapping_mul(51, iSlow67)))));
		let mut fSlow70: F32 = 43758.547 * F32::sin(fSlow10 + 1017.029);
		let mut fSlow71: F32 = 0.1 * (2.0 * (fSlow70 - F32::floor(fSlow70)) + -1.0);
		let mut fSlow72: F32 = self.fHslider47;
		let mut fSlow73: F32 = self.fHslider48;
		let mut fSlow74: F32 = unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min(iSlow69, 163))) as usize] };
		let mut fSlow75: F32 = self.fHslider49;
		let mut fSlow76: F32 = self.fHslider50;
		let mut fSlow77: F32 = unsafe { ftbl0VoiceDspSIG0[(std::cmp::max(0, std::cmp::min(iSlow56, 163))) as usize] };
		let mut fSlow78: F32 = self.fHslider51;
		let mut fSlow79: F32 = self.fHslider52;
		let mut fSlow80: F32 = 43758.547 * F32::sin(fSlow10 + 625.864);
		let mut fSlow81: F32 = 0.1 * (2.0 * (fSlow80 - F32::floor(fSlow80)) + -1.0);
		let mut fSlow82: F32 = self.fHslider53;
		let mut fSlow83: F32 = self.fHslider54;
		let mut fSlow84: F32 = self.fHslider55;
		let mut fSlow85: F32 = self.fHslider56;
		let mut fSlow86: F32 = self.fHslider57;
		let mut fSlow87: F32 = 43758.547 * F32::sin(fSlow10 + 782.33);
		let mut fSlow88: F32 = 0.1 * (2.0 * (fSlow87 - F32::floor(fSlow87)) + -1.0);
		let mut fSlow89: F32 = self.fHslider58;
		let mut fSlow90: F32 = self.fHslider59;
		let mut fSlow91: F32 = self.fHslider60;
		let mut fSlow92: F32 = self.fHslider61;
		let mut fSlow93: F32 = self.fHslider62;
		let mut fSlow94: F32 = self.fHslider63;
		let mut fSlow95: F32 = 43758.547 * F32::sin(fSlow10 + 704.097);
		let mut fSlow96: F32 = 0.1 * (2.0 * (fSlow95 - F32::floor(fSlow95)) + -1.0);
		let mut fSlow97: F32 = self.fHslider64;
		let mut fSlow98: F32 = self.fHslider65;
		let mut fSlow99: F32 = self.fHslider66;
		let mut fSlow100: F32 = self.fHslider67;
		let mut fSlow101: F32 = self.fHslider68;
		let mut fSlow102: F32 = self.fHslider69;
		let mut fSlow103: F32 = self.fHslider70;
		let mut fSlow104: F32 = self.fHslider71;
		let mut fSlow105: F32 = self.fHslider72;
		let mut fSlow106: F32 = self.fHslider73;
		let mut fSlow107: F32 = self.fHslider74;
		let mut fSlow108: F32 = self.fHslider75;
		let mut fSlow109: F32 = self.fHslider76;
		let mut fSlow110: F32 = 43758.547 * F32::sin(fSlow10 + 469.398);
		let mut fSlow111: F32 = 0.1 * (2.0 * (fSlow110 - F32::floor(fSlow110)) + -1.0);
		let mut fSlow112: F32 = self.fHslider77;
		let mut fSlow113: F32 = self.fHslider78;
		let mut fSlow114: F32 = self.fHslider79;
		let mut fSlow115: F32 = self.fHslider80;
		let mut fSlow116: F32 = self.fHslider81;
		let mut fSlow117: F32 = self.fHslider82;
		let mut fSlow118: F32 = 43758.547 * F32::sin(fSlow10 + 547.631);
		let mut fSlow119: F32 = 0.15 * (2.0 * (fSlow118 - F32::floor(fSlow118)) + -1.0);
		let mut fSlow120: F32 = self.fEntry4;
		let mut fSlow121: F32 = self.fHslider83;
		let mut fSlow122: F32 = self.fHslider84;
		let mut fSlow123: F32 = 43758.547 * F32::sin(fSlow10 + 860.563);
		let mut fSlow124: F32 = 0.05 * (2.0 * (fSlow123 - F32::floor(fSlow123)) + -1.0);
		let mut fSlow125: F32 = self.fHslider85;
		let mut fSlow126: F32 = self.fHslider86;
		let mut fSlow127: F32 = self.fEntry5;
		let mut fSlow128: F32 = self.fHslider87;
		let mut fSlow129: F32 = self.fHslider88;
		let mut fSlow130: F32 = 1000003.0 * fSlow9;
		let mut fSlow131: F32 = 1048576.0 * (fSlow130 - F32::floor(fSlow130));
		let mut fSlow132: F32 = self.fHslider89;
		let mut fSlow133: F32 = self.fHslider90;
		let mut fSlow134: F32 = self.fHslider91;
		let mut fSlow135: F32 = self.fHslider92;
		let mut fSlow136: F32 = self.fHslider93;
		let mut fSlow137: F32 = self.fHslider94;
		let mut fSlow138: F32 = self.fHslider95;
		let mut fSlow139: F32 = self.fHslider96;
		let mut fSlow140: F32 = self.fHslider97;
		let mut fSlow141: F32 = self.fHslider98;
		let zipped_iterators = outputs0.zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7).zip(outputs8).zip(outputs9).zip(outputs10).zip(outputs11).zip(outputs12).zip(outputs13).zip(outputs14).zip(outputs15).zip(outputs16).zip(outputs17).zip(outputs18).zip(outputs19).zip(outputs20);
		for ((((((((((((((((((((output0, output1), output2), output3), output4), output5), output6), output7), output8), output9), output10), output11), output12), output13), output14), output15), output16), output17), output18), output19), output20) in zipped_iterators {
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
			self.fRec9[0] = fSlow8 * fTemp3 + self.fConst1 * fTemp2 * self.fRec9[1];
			self.fRec10[0] = fSlow13 * fTemp3 + self.fConst1 * fTemp2 * self.fRec10[1];
			self.fRec11[0] = fSlow14 * fTemp3 + self.fConst1 * fTemp2 * self.fRec11[1];
			let mut fTemp12: F32 = if (self.fRec10[0] != 0.0) as i32 != 0 {3.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.00025125628 * (self.fRec11[0] + -2e+01)))) + self.fRec10[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec11[0]};
			self.fRec12[0] = fSlow15 * fTemp3 + self.fConst1 * fTemp2 * self.fRec12[1];
			self.fRec13[0] = fSlow16 * fTemp3 + self.fConst1 * fTemp2 * self.fRec13[1];
			let mut fTemp13: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec12[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec13[0] + self.fRec12[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec13[0]}));
			let mut fTemp14: F32 = F32::powf(1e+01, fSlow20 * self.fRec9[0]);
			self.fRec14[0] = fSlow21 * fTemp3 + self.fConst1 * fTemp2 * self.fRec14[1];
			self.fRec15[0] = fSlow22 * fTemp3 + self.fConst1 * fTemp2 * self.fRec15[1];
			self.fRec16[0] = fSlow23 * fTemp3 + self.fConst1 * fTemp2 * self.fRec16[1];
			self.fRec17[0] = fSlow24 * fTemp3 + self.fConst1 * fTemp2 * self.fRec17[1];
			let mut fTemp15: F32 = self.fRec6[0] + -0.5;
			self.fRec18[0] = fSlow27 * fTemp3 + self.fConst1 * fTemp2 * self.fRec18[1];
			self.fRec19[0] = fSlow28 * fTemp3 + self.fConst1 * fTemp2 * self.fRec19[1];
			self.fRec20[0] = fSlow29 * fTemp3 + self.fConst1 * fTemp2 * self.fRec20[1];
			self.fRec21[0] = fSlow30 * fTemp3 + self.fConst1 * fTemp2 * self.fRec21[1];
			let mut fTemp16: F32 = 3.0 * F32::max(0.0, F32::min(1.0, if (self.fRec20[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec21[0] + self.fRec20[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec21[0]} * (if (self.fRec18[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec19[0] + self.fRec18[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec19[0]} * fTemp15 + 1.0) + fSlow26 * self.fRec9[0]));
			let mut fTemp17: F32 = 5.0 * F32::max(0.0, F32::min(1.0, fTemp16 + -1.0)) + 1.0;
			let mut fTemp18: F32 = F32::tanh(fTemp17);
			let mut fTemp19: F32 = F32::max(0.0, F32::min(1.0, fTemp16 + -2.0));
			self.fRec23[0] = fSlow31 * fTemp3 + self.fConst1 * fTemp2 * self.fRec23[1];
			self.fRec24[0] = fSlow32 * fTemp3 + self.fConst1 * fTemp2 * self.fRec24[1];
			self.fRec25[0] = fSlow33 * fTemp3 + self.fConst1 * fTemp2 * self.fRec25[1];
			self.fRec26[0] = fSlow34 * fTemp3 + self.fConst1 * fTemp2 * self.fRec26[1];
			self.fRec27[0] = fSlow35 * fTemp3 + self.fConst1 * fTemp2 * self.fRec27[1];
			self.fRec28[0] = fSlow36 * fTemp3 + self.fConst1 * fTemp2 * self.fRec28[1];
			let mut fTemp20: F32 = if (self.fRec27[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec28[0] + self.fRec27[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec28[0]};
			self.fRec29[0] = fSlow37 * fTemp3 + self.fConst1 * fTemp2 * self.fRec29[1];
			self.fRec30[0] = fSlow38 * fTemp3 + self.fConst1 * fTemp2 * self.fRec30[1];
			self.fRec31[0] = fSlow39 * fTemp3 + self.fConst1 * fTemp2 * self.fRec31[1];
			self.fRec32[0] = fSlow40 * fTemp3 + self.fConst1 * fTemp2 * self.fRec32[1];
			self.fRec33[0] = fSlow44 * fTemp3 + self.fConst1 * fTemp2 * self.fRec33[1];
			self.fRec34[0] = fSlow45 * fTemp3 + self.fConst1 * fTemp2 * self.fRec34[1];
			let mut fTemp21: F32 = if (self.fRec33[0] != 0.0) as i32 != 0 {1.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0005050505 * (self.fRec34[0] + -2e+01)))) + self.fRec33[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec34[0]};
			let mut fTemp22: F32 = if iSlow43 != 0 {fSlow46 * fTemp21} else {fTemp21} * F32::exp(fSlow42 * self.fRec9[0]);
			let mut fTemp23: F32 = self.fRec22[1] + self.fConst2 * F32::min(self.fConst4, fTemp22 * F32::exp(0.057762265 * if (self.fRec31[0] != 0.0) as i32 != 0 {72.0 * F32::max(0.0, F32::min(1.0, 0.013888889 * self.fRec32[0] + self.fRec31[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec32[0]} * (fTemp20 * F32::exp(-(self.fConst5 * (fTemp5 / F32::max(0.0001, 0.001 * if (self.fRec29[0] != 0.0) as i32 != 0 {19.0 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.05263158 * (self.fRec30[0] + -1.0)))) + self.fRec29[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 1.0} else {self.fRec30[0]})))) + (1.0 - fTemp20) * F32::exp(-(self.fConst5 * (fTemp5 / F32::max(0.0001, 0.001 * if (self.fRec25[0] != 0.0) as i32 != 0 {4.8e+02 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0020833334 * (self.fRec26[0] + -2e+01)))) + self.fRec25[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec26[0]}))))) * (1.0 - fTemp9 * if (self.fRec23[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec24[0] + self.fRec23[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec24[0]})));
			self.fRec22[0] = if iTemp0 != 0 {0.0} else {fTemp23 - F32::floor(fTemp23)};
			let mut fTemp24: F32 = F32::max(0.0, F32::min(1.0, fTemp16));
			let mut fTemp25: F32 = F32::tanh((F32::sin(6.2831855 * self.fRec22[0]) * (1.0 - fTemp24) + fTemp24 * (1.0 - 4.0 * F32::abs(self.fRec22[0] + (-0.25 - F32::floor(self.fRec22[0] + 0.25))))) * fTemp17);
			let mut fTemp26: F32 = F32::min(1.0, 4.0 * fTemp19);
			self.fRec35[0] = fSlow47 * fTemp3 + self.fConst1 * fTemp2 * self.fRec35[1];
			*output0 = self.fRec35[0] * fTemp11 * ((1.0 - fTemp26) * fTemp25 / fTemp18 + fTemp26 * F32::sin(1.5707964 * (fTemp25 * (3.0 * fTemp19 + 1.0) / fTemp18))) * F32::min(1.0, self.fConst2 * (fTemp5 / F32::max(1e-05, 0.001 * if (self.fRec16[0] != 0.0) as i32 != 0 {1e+01 * F32::max(0.0, F32::min(1.0, 0.1 * self.fRec17[0] + self.fRec16[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec17[0]}))) * (1.0 - fTemp9 * if (self.fRec14[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec15[0] + self.fRec14[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec15[0]}) * fTemp14 * (fTemp13 * F32::exp(-(self.fConst3 * (fTemp5 / F32::max(0.0001, 0.001 * fTemp12 * (fSlow18 * self.fRec9[0] + 1.0))))) + (1.0 - fTemp13) * F32::exp(-(self.fConst3 * (fTemp5 / F32::max(0.0001, 0.001 * fTemp12 * (fSlow12 * self.fRec9[0] + 1.0) * F32::powf(1e+01, -2.0 * if (self.fRec0[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec1[0] + self.fRec0[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec1[0]}))))));
			self.fRec36[0] = fSlow48 * fTemp3 + self.fConst1 * fTemp2 * self.fRec36[1];
			self.fRec37[0] = fSlow49 * fTemp3 + self.fConst1 * fTemp2 * self.fRec37[1];
			self.fRec38[0] = fSlow50 * fTemp3 + self.fConst1 * fTemp2 * self.fRec38[1];
			self.fRec40[0] = fSlow51 * fTemp3 + self.fConst1 * fTemp2 * self.fRec40[1];
			self.fRec41[0] = fSlow52 * fTemp3 + self.fConst1 * fTemp2 * self.fRec41[1];
			let mut fTemp27: F32 = F32::max(0.005, 0.001 * if (self.fRec40[0] != 0.0) as i32 != 0 {1995.0 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0005012531 * (self.fRec41[0] + -5.0)))) + self.fRec40[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 5.0} else {self.fRec41[0]});
			self.fRec42[0] = fSlow59 * fTemp3 + self.fConst1 * fTemp2 * self.fRec42[1];
			self.fRec43[0] = fSlow60 * fTemp3 + self.fConst1 * fTemp2 * self.fRec43[1];
			let mut fTemp28: F32 = sl_hit(iSlow56, self.fConst6 * fTemp5 * F32::exp(0.057762265 * (if (self.fRec42[0] != 0.0) as i32 != 0 {48.0 * F32::max(0.0, F32::min(1.0, 0.020833334 * (self.fRec43[0] + 24.0) + self.fRec42[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -24.0} else {self.fRec43[0]} + fSlow58 * self.fRec9[0]))) * F32::max(0.0, F32::min(1.0, 2.5 * ((fTemp27 - fTemp6) / fTemp27)));
			self.fVec1[0] = fTemp28;
			self.fRec39[0] = 0.995 * self.fRec39[1] + fTemp28 - self.fVec1[1];
			*output1 = self.fRec39[0] * self.fRec38[0] * fTemp11 * fTemp14 * (1.0 - fTemp9 * if (self.fRec36[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec37[0] + self.fRec36[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec37[0]});
			self.fRec44[0] = fSlow61 * fTemp3 + self.fConst1 * fTemp2 * self.fRec44[1];
			self.fRec45[0] = fSlow62 * fTemp3 + self.fConst1 * fTemp2 * self.fRec45[1];
			self.fRec46[0] = fSlow63 * fTemp3 + self.fConst1 * fTemp2 * self.fRec46[1];
			self.fRec48[0] = fSlow64 * fTemp3 + self.fConst1 * fTemp2 * self.fRec48[1];
			self.fRec49[0] = fSlow65 * fTemp3 + self.fConst1 * fTemp2 * self.fRec49[1];
			let mut fTemp29: F32 = F32::max(0.005, 0.001 * if (self.fRec48[0] != 0.0) as i32 != 0 {1995.0 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.0005012531 * (self.fRec49[0] + -5.0)))) + self.fRec48[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 5.0} else {self.fRec49[0]});
			self.fRec50[0] = fSlow72 * fTemp3 + self.fConst1 * fTemp2 * self.fRec50[1];
			self.fRec51[0] = fSlow73 * fTemp3 + self.fConst1 * fTemp2 * self.fRec51[1];
			let mut fTemp30: F32 = sl_hit(iSlow69, self.fConst6 * fTemp5 * F32::exp(0.057762265 * (if (self.fRec50[0] != 0.0) as i32 != 0 {48.0 * F32::max(0.0, F32::min(1.0, 0.020833334 * (self.fRec51[0] + 24.0) + self.fRec50[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -24.0} else {self.fRec51[0]} + fSlow71 * self.fRec9[0]))) * F32::max(0.0, F32::min(1.0, 2.5 * ((fTemp29 - fTemp6) / fTemp29)));
			self.fVec2[0] = fTemp30;
			self.fRec47[0] = 0.995 * self.fRec47[1] + fTemp30 - self.fVec2[1];
			*output2 = self.fRec47[0] * self.fRec46[0] * fTemp11 * fTemp14 * (1.0 - fTemp9 * if (self.fRec44[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec45[0] + self.fRec44[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec45[0]});
			self.fRec52[0] = fSlow75 * fTemp3 + self.fConst1 * fTemp2 * self.fRec52[1];
			self.fRec53[0] = fSlow76 * fTemp3 + self.fConst1 * fTemp2 * self.fRec53[1];
			let mut fTemp31: F32 = if (self.fRec52[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec53[0] + self.fRec52[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec53[0]};
			self.fRec54[0] = fSlow78 * fTemp3 + self.fConst1 * fTemp2 * self.fRec54[1];
			self.fRec55[0] = fSlow79 * fTemp3 + self.fConst1 * fTemp2 * self.fRec55[1];
			let mut fTemp32: F32 = if (self.fRec54[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec55[0] + self.fRec54[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec55[0]};
			self.fRec56[0] = fSlow82 * fTemp3 + self.fConst1 * fTemp2 * self.fRec56[1];
			self.fRec57[0] = fSlow83 * fTemp3 + self.fConst1 * fTemp2 * self.fRec57[1];
			let mut fTemp33: F32 = if (self.fRec56[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec57[0] + self.fRec56[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec57[0]};
			let mut fTemp34: F32 = F32::max(0.0, F32::min(1.0, F32::max(0.0, F32::min(1.0, fTemp33 + fSlow81 * self.fRec9[0]))));
			let mut fTemp35: F32 = F32::powf(0.016, fTemp34);
			self.iRec58[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec58[1]), 12345);
			*output3 = fTemp11 * (self.fConst8 * (((fTemp6 < (0.005 * fTemp35)) as i32) as u32 as F32 * F32::sin(self.fConst7 * (fTemp5 / fTemp35)) * (2.7939678e-10 * (self.iRec58[0]) as F32 * fTemp34 + 1.0) * (1.0 - F32::min(1.0, fTemp32 + fTemp31)) / fTemp35) + 0.05 * (self.fRec39[0] * fTemp32 * fSlow77 + self.fRec47[0] * fTemp31 * fSlow74));
			self.fRec59[0] = fSlow84 * fTemp3 + self.fConst1 * fTemp2 * self.fRec59[1];
			*output4 = self.fRec6[0] * self.fRec59[0] * fTemp11 * fTemp14;
			self.fRec60[0] = fSlow85 * fTemp3 + self.fConst1 * fTemp2 * self.fRec60[1];
			self.fRec61[0] = fSlow86 * fTemp3 + self.fConst1 * fTemp2 * self.fRec61[1];
			let mut fTemp36: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec60[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec61[0] + self.fRec60[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec61[0]}));
			let mut fTemp37: F32 = 0.01 * fTemp36 + 0.006;
			let mut fTemp38: F32 = F32::floor(4.0 * fTemp36 + 0.5) * fTemp37;
			self.fRec62[0] = fSlow89 * fTemp3 + self.fConst1 * fTemp2 * self.fRec62[1];
			self.fRec63[0] = fSlow90 * fTemp3 + self.fConst1 * fTemp2 * self.fRec63[1];
			let mut fTemp39: F32 = if (self.fRec62[0] != 0.0) as i32 != 0 {3.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.00025125628 * (self.fRec63[0] + -2e+01)))) + self.fRec62[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec63[0]};
			let mut fTemp40: F32 = F32::max(0.0, fTemp6 - fTemp38);
			self.fRec64[0] = fSlow91 * fTemp3 + self.fConst1 * fTemp2 * self.fRec64[1];
			self.fRec65[0] = fSlow92 * fTemp3 + self.fConst1 * fTemp2 * self.fRec65[1];
			let mut fTemp41: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec64[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec65[0] + self.fRec64[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec65[0]}));
			self.fRec66[0] = fSlow93 * fTemp3 + self.fConst1 * fTemp2 * self.fRec66[1];
			self.fRec67[0] = fSlow94 * fTemp3 + self.fConst1 * fTemp2 * self.fRec67[1];
			self.fRec68[0] = fSlow97 * fTemp3 + self.fConst1 * fTemp2 * self.fRec68[1];
			self.fRec69[0] = fSlow98 * fTemp3 + self.fConst1 * fTemp2 * self.fRec69[1];
			let mut fTemp42: F32 = self.fConst2 * (fTemp5 / fTemp37);
			let mut fTemp43: F32 = if (fTemp6 < fTemp38) as i32 != 0 {F32::exp(-2.5 * (fTemp42 - F32::floor(fTemp42)))} else {F32::min(1.0, fTemp40 / F32::max(1e-05, 0.001 * if (self.fRec68[0] != 0.0) as i32 != 0 {1e+01 * F32::max(0.0, F32::min(1.0, 0.1 * self.fRec69[0] + self.fRec68[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec69[0]})) * ((1.0 - fTemp41) * F32::exp(-6.907755 * (fTemp40 / F32::max(0.0001, 0.001 * fTemp39 * (fSlow96 * self.fRec9[0] + 1.0) * F32::powf(1e+01, -2.0 * if (self.fRec66[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec67[0] + self.fRec66[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec67[0]})))) + fTemp41 * F32::exp(-6.907755 * (fTemp40 / F32::max(0.0001, 0.001 * fTemp39 * (fSlow88 * self.fRec9[0] + 1.0)))))};
			self.fRec70[0] = fSlow99 * fTemp3 + self.fConst1 * fTemp2 * self.fRec70[1];
			self.fRec71[0] = fSlow100 * fTemp3 + self.fConst1 * fTemp2 * self.fRec71[1];
			self.fRec72[0] = fSlow101 * fTemp3 + self.fConst1 * fTemp2 * self.fRec72[1];
			*output5 = self.fRec72[0] * fTemp11 * fTemp14 * (1.0 - fTemp9 * if (self.fRec70[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec71[0] + self.fRec70[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec71[0]}) * fTemp43;
			self.fRec73[0] = fSlow102 * fTemp3 + self.fConst1 * fTemp2 * self.fRec73[1];
			self.fRec74[0] = fSlow103 * fTemp3 + self.fConst1 * fTemp2 * self.fRec74[1];
			let mut fTemp44: F32 = if (self.fRec73[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec74[0] + self.fRec73[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec74[0]};
			let mut fTemp45: F32 = F32::max(0.0, F32::min(1.0, fTemp44));
			let mut fTemp46: F32 = 2.0 * fTemp45;
			let mut fTemp47: F32 = F32::max(0.0, 1.0 - F32::abs(fTemp46));
			self.fRec75[0] = fSlow104 * fTemp3 + self.fConst1 * fTemp2 * self.fRec75[1];
			self.fRec76[0] = fSlow105 * fTemp3 + self.fConst1 * fTemp2 * self.fRec76[1];
			self.fRec77[0] = fSlow106 * fTemp3 + self.fConst1 * fTemp2 * self.fRec77[1];
			self.fRec78[0] = fSlow107 * fTemp3 + self.fConst1 * fTemp2 * self.fRec78[1];
			*output6 = fTemp22 * F32::exp(0.057762265 * (if (self.fRec77[0] != 0.0) as i32 != 0 {48.0 * F32::max(0.0, F32::min(1.0, 0.020833334 * (self.fRec78[0] + 24.0) + self.fRec77[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -24.0} else {self.fRec78[0]} + 2.0 * self.fRec6[0] * if (self.fRec75[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec76[0] + self.fRec75[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec76[0]} * fTemp47 * F32::exp(-(self.fConst9 * fTemp5))));
			*output7 = fTemp44;
			self.fRec79[0] = fSlow108 * fTemp3 + self.fConst1 * fTemp2 * self.fRec79[1];
			self.fRec80[0] = fSlow109 * fTemp3 + self.fConst1 * fTemp2 * self.fRec80[1];
			self.fRec81[0] = fSlow112 * fTemp3 + self.fConst1 * fTemp2 * self.fRec81[1];
			self.fRec82[0] = fSlow113 * fTemp3 + self.fConst1 * fTemp2 * self.fRec82[1];
			*output8 = 0.001 * if (self.fRec81[0] != 0.0) as i32 != 0 {3.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.00025125628 * (self.fRec82[0] + -2e+01)))) + self.fRec81[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec82[0]} * (fSlow111 * self.fRec9[0] + 1.0) * (fTemp15 * if (self.fRec79[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec80[0] + self.fRec79[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec80[0]} + 1.0);
			self.fRec83[0] = fSlow114 * fTemp3 + self.fConst1 * fTemp2 * self.fRec83[1];
			self.fRec84[0] = fSlow115 * fTemp3 + self.fConst1 * fTemp2 * self.fRec84[1];
			self.fRec85[0] = fSlow116 * fTemp3 + self.fConst1 * fTemp2 * self.fRec85[1];
			self.fRec86[0] = fSlow117 * fTemp3 + self.fConst1 * fTemp2 * self.fRec86[1];
			*output9 = F32::max(0.0, F32::min(1.0, F32::max(0.0, F32::min(1.0, 0.8 * fTemp33 + (0.5 - if (self.fRec85[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec86[0] + self.fRec85[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec86[0]}))) + 0.5 * self.fRec6[0] * if (self.fRec83[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec84[0] + self.fRec83[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec84[0]}));
			*output10 = F32::max(0.05, F32::min(0.95, 0.22 * F32::max(0.0, 1.0 - F32::abs(-2.0 * (1.0 - fTemp45))) + 0.3 * fTemp47 + 0.37 * F32::max(0.0, 1.0 - F32::abs(fTemp46 + -1.0)) + fSlow119 * self.fRec9[0]));
			*output11 = fSlow120;
			*output12 = fTemp22;
			self.fRec87[0] = fSlow121 * fTemp3 + self.fConst1 * fTemp2 * self.fRec87[1];
			self.fRec88[0] = fSlow122 * fTemp3 + self.fConst1 * fTemp2 * self.fRec88[1];
			self.fRec89[0] = fSlow125 * fTemp3 + self.fConst1 * fTemp2 * self.fRec89[1];
			self.fRec90[0] = fSlow126 * fTemp3 + self.fConst1 * fTemp2 * self.fRec90[1];
			*output13 = if (self.fRec89[0] != 0.0) as i32 != 0 {1.19e+04 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 8.4033614e-05 * (self.fRec90[0] + -1e+02)))) + self.fRec89[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 1e+02} else {self.fRec90[0]} * (fSlow124 * self.fRec9[0] + 1.0) * F32::exp(2.7725887 * if (self.fRec87[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec88[0] + self.fRec87[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec88[0]} * fTemp43);
			*output14 = fSlow127;
			self.fRec91[0] = fSlow128 * fTemp3 + self.fConst1 * fTemp2 * self.fRec91[1];
			self.fRec92[0] = fSlow129 * fTemp3 + self.fConst1 * fTemp2 * self.fRec92[1];
			*output15 = fSlow131 + self.fConst6 * fTemp5 * F32::exp(0.057762265 * if (self.fRec91[0] != 0.0) as i32 != 0 {48.0 * F32::max(0.0, F32::min(1.0, 0.020833334 * (self.fRec92[0] + 24.0) + self.fRec91[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7)) + -24.0} else {self.fRec92[0]});
			self.fRec93[0] = fSlow132 * fTemp3 + self.fConst1 * fTemp2 * self.fRec93[1];
			self.fRec94[0] = fSlow133 * fTemp3 + self.fConst1 * fTemp2 * self.fRec94[1];
			*output16 = if (self.fRec93[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec94[0] + self.fRec93[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec94[0]};
			self.fRec95[0] = fSlow134 * fTemp3 + self.fConst1 * fTemp2 * self.fRec95[1];
			self.fRec96[0] = fSlow135 * fTemp3 + self.fConst1 * fTemp2 * self.fRec96[1];
			*output17 = if (self.fRec95[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec96[0] + self.fRec95[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec96[0]};
			self.fRec97[0] = fSlow136 * fTemp3 + self.fConst1 * fTemp2 * self.fRec97[1];
			self.fRec98[0] = fSlow137 * fTemp3 + self.fConst1 * fTemp2 * self.fRec98[1];
			*output18 = if (self.fRec97[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec98[0] + self.fRec97[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec98[0]};
			self.fRec99[0] = fSlow138 * fTemp3 + self.fConst1 * fTemp2 * self.fRec99[1];
			self.fRec100[0] = fSlow139 * fTemp3 + self.fConst1 * fTemp2 * self.fRec100[1];
			*output19 = if (self.fRec99[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec100[0] + self.fRec99[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))} else {self.fRec100[0]};
			self.fRec101[0] = fSlow140 * fTemp3 + self.fConst1 * fTemp2 * self.fRec101[1];
			self.fRec102[0] = fSlow141 * fTemp3 + self.fConst1 * fTemp2 * self.fRec102[1];
			*output20 = if (self.fRec101[0] != 0.0) as i32 != 0 {7.98e+03 * VoiceDsp_faustpower4_f(F32::max(0.0, F32::min(1.0, F32::sqrt(F32::sqrt(F32::max(0.0, 0.00012531328 * (self.fRec102[0] + -2e+01)))) + self.fRec101[0] * fTemp11 * fTemp10 * fTemp8 * fTemp7))) + 2e+01} else {self.fRec102[0]};
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
			self.fRec22[1] = self.fRec22[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec51[1] = self.fRec51[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec57[1] = self.fRec57[0];
			self.iRec58[1] = self.iRec58[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec61[1] = self.fRec61[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec69[1] = self.fRec69[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec73[1] = self.fRec73[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec78[1] = self.fRec78[0];
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
		}
	}

}

