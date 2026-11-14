/* ------------------------------------------------------------
name: "ABDrumNoise"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn NoiseDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn NoiseDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct NoiseDsp {
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	iVec0: [i32;2],
	fConst3: F32,
	fConst4: F32,
	iRec14: [i32;2],
	fRec13: [F32;2],
	iRec16: [i32;2],
	fRec17: [F32;4],
	fConst5: F32,
	iRec18: [i32;2],
	fConst6: F32,
	fConst7: F32,
	fRec22: [F32;2],
	fRec21: [F32;2],
	fTempPerm0: F32,
	fTempPerm1: F32,
	fConst8: F32,
	fTempPerm2: F32,
	fRec23: [F32;2],
	fTempPerm3: F32,
	fConst9: F32,
	fTempPerm4: F32,
	fRec24: [F32;2],
	fTempPerm5: F32,
	fConst10: F32,
	fTempPerm6: F32,
	fRec25: [F32;2],
	fTempPerm7: F32,
	fConst11: F32,
	fTempPerm8: F32,
	fRec26: [F32;2],
	fTempPerm9: F32,
	fConst12: F32,
	fTempPerm10: F32,
	fRec27: [F32;2],
	fTempPerm11: F32,
	fTempPerm12: F32,
	fRec28: [F32;2],
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
	fConst13: F32,
	fTempPerm44: F32,
	fTempPerm45: F32,
	fRec31: [F32;2],
	iTempPerm46: i32,
	iTempPerm47: i32,
	iTempPerm48: i32,
	iRec32: [i32;2],
	fRec30: [F32;2],
	iTempPerm49: i32,
	fConst14: F32,
	fTempPerm50: F32,
	fRec37: [F32;2],
	fRec36: [F32;2],
	fTempPerm51: F32,
	fRec35: [F32;2],
	fTempPerm52: F32,
	fTempPerm53: F32,
	fVec1: [F32;2],
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fRec29: [F32;2],
	fTempPerm54: F32,
	fRec39: [F32;2],
	fRec38: [F32;2],
	fRec10: [F32;2],
	fRec11: [F32;2],
	fRec7: [F32;2],
	fRec8: [F32;2],
	fRec4: [F32;2],
	fRec5: [F32;2],
	fRec1: [F32;2],
	fRec2: [F32;2],
	fRec0: [F32;3],
}

impl FaustDsp for NoiseDsp {
	type T = F32;
		
	fn new() -> NoiseDsp { 
		NoiseDsp {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			iVec0: [0;2],
			fConst3: 0.0,
			fConst4: 0.0,
			iRec14: [0;2],
			fRec13: [0.0;2],
			iRec16: [0;2],
			fRec17: [0.0;4],
			fConst5: 0.0,
			iRec18: [0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fRec22: [0.0;2],
			fRec21: [0.0;2],
			fTempPerm0: 0.0,
			fTempPerm1: 0.0,
			fConst8: 0.0,
			fTempPerm2: 0.0,
			fRec23: [0.0;2],
			fTempPerm3: 0.0,
			fConst9: 0.0,
			fTempPerm4: 0.0,
			fRec24: [0.0;2],
			fTempPerm5: 0.0,
			fConst10: 0.0,
			fTempPerm6: 0.0,
			fRec25: [0.0;2],
			fTempPerm7: 0.0,
			fConst11: 0.0,
			fTempPerm8: 0.0,
			fRec26: [0.0;2],
			fTempPerm9: 0.0,
			fConst12: 0.0,
			fTempPerm10: 0.0,
			fRec27: [0.0;2],
			fTempPerm11: 0.0,
			fTempPerm12: 0.0,
			fRec28: [0.0;2],
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
			fConst13: 0.0,
			fTempPerm44: 0.0,
			fTempPerm45: 0.0,
			fRec31: [0.0;2],
			iTempPerm46: 0,
			iTempPerm47: 0,
			iTempPerm48: 0,
			iRec32: [0;2],
			fRec30: [0.0;2],
			iTempPerm49: 0,
			fConst14: 0.0,
			fTempPerm50: 0.0,
			fRec37: [0.0;2],
			fRec36: [0.0;2],
			fTempPerm51: 0.0,
			fRec35: [0.0;2],
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fVec1: [0.0;2],
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fRec29: [0.0;2],
			fTempPerm54: 0.0,
			fRec39: [0.0;2],
			fRec38: [0.0;2],
			fRec10: [0.0;2],
			fRec11: [0.0;2],
			fRec7: [0.0;2],
			fRec8: [0.0;2],
			fRec4: [0.0;2],
			fRec5: [0.0;2],
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fRec0: [0.0;3],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn NoiseDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("drum.lib/name", r"abdrum-layers");
		m.declare("drum.lib/version", r"0.1");
		m.declare("filename", r"plugin-noise.dsp");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/name", r"Faust Filters Library");
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
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"ABDrumNoise");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.0");
		m.declare("oscillators.lib/lf_sawpos:author", r"Bart Brouns, revised by Stéphane Letz");
		m.declare("oscillators.lib/lf_sawpos:licence", r"STK-4.3");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/version", r"1.5.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 11;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.iRec14[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec13[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.iRec16[l3 as usize] = 0;
		}
		for l4 in 0..4 {
			self.fRec17[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.iRec18[l5 as usize] = 0;
		}
		for l6 in 0..2 {
			self.fRec22[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec21[l7 as usize] = 0.0;
		}
		self.fTempPerm0 = 0.0;
		self.fTempPerm1 = 0.0;
		self.fTempPerm2 = 0.0;
		for l8 in 0..2 {
			self.fRec23[l8 as usize] = 0.0;
		}
		self.fTempPerm3 = 0.0;
		self.fTempPerm4 = 0.0;
		for l9 in 0..2 {
			self.fRec24[l9 as usize] = 0.0;
		}
		self.fTempPerm5 = 0.0;
		self.fTempPerm6 = 0.0;
		for l10 in 0..2 {
			self.fRec25[l10 as usize] = 0.0;
		}
		self.fTempPerm7 = 0.0;
		self.fTempPerm8 = 0.0;
		for l11 in 0..2 {
			self.fRec26[l11 as usize] = 0.0;
		}
		self.fTempPerm9 = 0.0;
		self.fTempPerm10 = 0.0;
		for l12 in 0..2 {
			self.fRec27[l12 as usize] = 0.0;
		}
		self.fTempPerm11 = 0.0;
		self.fTempPerm12 = 0.0;
		for l13 in 0..2 {
			self.fRec28[l13 as usize] = 0.0;
		}
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
		for l14 in 0..2 {
			self.fRec31[l14 as usize] = 0.0;
		}
		self.iTempPerm46 = 0;
		self.iTempPerm47 = 0;
		self.iTempPerm48 = 0;
		for l15 in 0..2 {
			self.iRec32[l15 as usize] = 0;
		}
		for l16 in 0..2 {
			self.fRec30[l16 as usize] = 0.0;
		}
		self.iTempPerm49 = 0;
		self.fTempPerm50 = 0.0;
		for l17 in 0..2 {
			self.fRec37[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec36[l18 as usize] = 0.0;
		}
		self.fTempPerm51 = 0.0;
		for l19 in 0..2 {
			self.fRec35[l19 as usize] = 0.0;
		}
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		for l20 in 0..2 {
			self.fVec1[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec29[l21 as usize] = 0.0;
		}
		self.fTempPerm54 = 0.0;
		for l22 in 0..2 {
			self.fRec39[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec38[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec10[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec11[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec7[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec8[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec4[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec5[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec1[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec2[l31 as usize] = 0.0;
		}
		for l32 in 0..3 {
			self.fRec0[l32 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 0.45 * self.fConst0;
		self.fConst2 = 3.1415927 / self.fConst0;
		self.fConst3 = F32::exp(-(1.25e+03 / self.fConst0));
		self.fConst4 = 3e+01 / self.fConst0;
		self.fConst5 = 3e+03 / self.fConst0;
		self.fConst6 = 0.9999 * self.fConst0;
		self.fConst7 = 1.0 / self.fConst0;
		self.fConst8 = 3.897 / self.fConst0;
		self.fConst9 = 2.63 / self.fConst0;
		self.fConst10 = 2.546 / self.fConst0;
		self.fConst11 = 1.8 / self.fConst0;
		self.fConst12 = 1.483 / self.fConst0;
		self.fConst13 = 0.1 * self.fConst0;
		self.fConst14 = 12.0 / self.fConst0;
		self.fConst15 = 1.0 / F32::tan(62.831852 / self.fConst0);
		self.fConst16 = 1.0 - self.fConst15;
		self.fConst17 = 1.0 / (self.fConst15 + 1.0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		NoiseDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("ABDrumNoise");
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10) = if let [inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			let inputs3 = inputs3[..count as usize].iter();
			let inputs4 = inputs4[..count as usize].iter();
			let inputs5 = inputs5[..count as usize].iter();
			let inputs6 = inputs6[..count as usize].iter();
			let inputs7 = inputs7[..count as usize].iter();
			let inputs8 = inputs8[..count as usize].iter();
			let inputs9 = inputs9[..count as usize].iter();
			let inputs10 = inputs10[..count as usize].iter();
			(inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fTemp31: F32 = self.fTempPerm0;
		let mut fTemp33: F32 = self.fTempPerm1;
		let mut fTemp34: F32 = self.fTempPerm2;
		let mut fTemp35: F32 = self.fTempPerm3;
		let mut fTemp36: F32 = self.fTempPerm4;
		let mut fTemp37: F32 = self.fTempPerm5;
		let mut fTemp38: F32 = self.fTempPerm6;
		let mut fTemp39: F32 = self.fTempPerm7;
		let mut fTemp40: F32 = self.fTempPerm8;
		let mut fTemp41: F32 = self.fTempPerm9;
		let mut fTemp42: F32 = self.fTempPerm10;
		let mut fTemp43: F32 = self.fTempPerm11;
		let mut fTemp44: F32 = self.fTempPerm12;
		let mut fTemp45: F32 = self.fTempPerm13;
		let mut fTemp46: F32 = self.fTempPerm14;
		let mut fTemp47: F32 = self.fTempPerm15;
		let mut fTemp48: F32 = self.fTempPerm16;
		let mut fTemp49: F32 = self.fTempPerm17;
		let mut fTemp50: F32 = self.fTempPerm18;
		let mut fTemp51: F32 = self.fTempPerm19;
		let mut fTemp52: F32 = self.fTempPerm20;
		let mut fTemp53: F32 = self.fTempPerm21;
		let mut fTemp54: F32 = self.fTempPerm22;
		let mut fTemp55: F32 = self.fTempPerm23;
		let mut fTemp56: F32 = self.fTempPerm24;
		let mut fTemp57: F32 = self.fTempPerm25;
		let mut fTemp58: F32 = self.fTempPerm26;
		let mut fTemp59: F32 = self.fTempPerm27;
		let mut fTemp60: F32 = self.fTempPerm28;
		let mut fTemp61: F32 = self.fTempPerm29;
		let mut fTemp62: F32 = self.fTempPerm30;
		let mut fTemp63: F32 = self.fTempPerm31;
		let mut fTemp64: F32 = self.fTempPerm32;
		let mut fTemp65: F32 = self.fTempPerm33;
		let mut fTemp66: F32 = self.fTempPerm34;
		let mut fTemp67: F32 = self.fTempPerm35;
		let mut fTemp68: F32 = self.fTempPerm36;
		let mut fTemp69: F32 = self.fTempPerm37;
		let mut fTemp70: F32 = self.fTempPerm38;
		let mut fTemp71: F32 = self.fTempPerm39;
		let mut fTemp72: F32 = self.fTempPerm40;
		let mut fTemp73: F32 = self.fTempPerm41;
		let mut fTemp74: F32 = self.fTempPerm42;
		let mut fTemp75: F32 = self.fTempPerm43;
		let mut fTemp76: F32 = self.fTempPerm44;
		let mut fTemp77: F32 = self.fTempPerm45;
		let mut iTemp78: i32 = self.iTempPerm46;
		let mut iTemp79: i32 = self.iTempPerm47;
		let mut iTemp80: i32 = self.iTempPerm48;
		let mut iTemp81: i32 = self.iTempPerm49;
		let mut fTemp82: F32 = self.fTempPerm50;
		let mut fTemp83: F32 = self.fTempPerm51;
		let mut fTemp84: F32 = self.fTempPerm52;
		let mut fTemp85: F32 = self.fTempPerm53;
		let mut fTemp86: F32 = self.fTempPerm54;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(inputs4).zip(inputs5).zip(inputs6).zip(inputs7).zip(inputs8).zip(inputs9).zip(inputs10).zip(outputs0);
		for (((((((((((input0, input1), input2), input3), input4), input5), input6), input7), input8), input9), input10), output0) in zipped_iterators {
			let mut iRec33: i32 = 0;
			let mut iRec34: i32 = 0;
			let mut fTemp0: F32 = F32::tan(self.fConst2 * F32::max(2e+01, F32::min(self.fConst1, *input9)));
			let mut fTemp1: F32 = 1.0 / fTemp0;
			let mut fTemp2: F32 = (fTemp1 + 1.4142135) / fTemp0 + 1.0;
			let mut fTemp3: F32 = NoiseDsp_faustpower2_f(fTemp0);
			self.iVec0[0] = 1;
			let mut fTemp4: F32 = *input8;
			let mut fTemp5: F32 = 4.0 * fTemp4;
			let mut fTemp6: F32 = F32::max(0.0, F32::min(1.0, fTemp5 + -3.0));
			let mut fTemp7: F32 = *input2;
			let mut fTemp8: F32 = F32::tan(self.fConst2 * F32::max(2e+01, F32::min(self.fConst1, F32::max(2e+01, F32::min(self.fConst1, fTemp7)))));
			let mut fTemp9: F32 = fTemp8 + 1.1111112;
			let mut fTemp10: F32 = fTemp8 * fTemp9 + 1.0;
			let mut fTemp11: F32 = F32::tanh(0.2 * self.fRec1[1]);
			let mut fTemp12: F32 = F32::max(0.0, F32::min(1.0, fTemp5 + -2.0));
			let mut fTemp13: F32 = F32::tanh(0.2 * self.fRec4[1]);
			let mut fTemp14: F32 = F32::max(0.0, F32::min(1.0, fTemp5 + -1.0));
			let mut fTemp15: F32 = F32::tanh(0.2 * self.fRec7[1]);
			let mut fTemp16: F32 = F32::max(0.0, F32::min(1.0, fTemp5));
			let mut fTemp17: F32 = F32::tanh(0.2 * self.fRec10[1]);
			let mut fTemp18: F32 = F32::max(0.0, F32::min(1.0, *input7));
			let mut fTemp19: F32 = F32::max(0.0, F32::min(1.0, 6.6666665 * (fTemp18 + -0.85)));
			let mut iTemp20: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec14[1], 12345));
			self.iRec14[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp20, 12345));
			let mut iRec15: i32 = iTemp20;
			self.fRec13[0] = F32::max(self.fConst3 * self.fRec13[1], (((0.5 * (4.656613e-10 * (iRec15) as F32 + 1.0)) < (self.fConst4 * F32::powf(666.0, fTemp18))) as i32) as u32 as F32);
			let mut fTemp21: F32 = *input6;
			let mut fTemp22: F32 = 1.5707964 * *input5;
			let mut fTemp23: F32 = *input0;
			self.iRec16[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec16[1]), 12345);
			let mut fTemp24: F32 = 4.656613e-10 * (self.iRec16[0]) as F32;
			self.fRec17[0] = 0.5221894 * self.fRec17[3] + fTemp24 + 2.494956 * self.fRec17[1] - 2.0172658 * self.fRec17[2];
			let mut iTemp25: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec18[1], 12345));
			let mut iTemp26: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp25, 12345));
			self.iRec18[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp26, 12345));
			let mut iRec19: i32 = iTemp26;
			let mut iRec20: i32 = iTemp25;
			let mut fTemp27: F32 = 2.0 * fTemp7;
			let mut iTemp28: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			let mut fTemp29: F32 = if iTemp28 != 0 {0.0} else {self.fRec22[1] + self.fConst7 * F32::max(fTemp27, 0.0)};
			self.fRec22[0] = fTemp29 - F32::floor(fTemp29);
			self.fRec21[0] = if ((self.fRec22[0] < self.fRec22[1]) as i32) | ((fTemp27 >= self.fConst6) as i32) != 0 {fTemp24} else {self.fRec21[1]};
			let mut fTemp30: F32 = ((fTemp23 == 4.0) as i32) as u32 as F32;
			let mut fTemp32: F32 = ((fTemp23 == 5.0) as i32) as u32 as F32;
			if ((fTemp30 != 0.0) as i32) | ((fTemp32 != 0.0) as i32) != 0 {
				fTemp31 = *input1;
			}
				if (fTemp30 != 0.0) as i32 != 0 {
					fTemp33 = F32::max(self.fConst7 * F32::min(3.897 * fTemp31, self.fConst1), 1e-06);
					fTemp34 = if iTemp28 != 0 {0.0} else {self.fRec23[1] + self.fConst8 * fTemp31};
					self.fRec23[0] = fTemp34 - F32::floor(fTemp34);
					fTemp35 = F32::max(self.fConst7 * F32::min(2.63 * fTemp31, self.fConst1), 1e-06);
					fTemp36 = if iTemp28 != 0 {0.0} else {self.fRec24[1] + self.fConst9 * fTemp31};
					self.fRec24[0] = fTemp36 - F32::floor(fTemp36);
					fTemp37 = F32::max(self.fConst7 * F32::min(2.546 * fTemp31, self.fConst1), 1e-06);
					fTemp38 = if iTemp28 != 0 {0.0} else {self.fRec25[1] + self.fConst10 * fTemp31};
					self.fRec25[0] = fTemp38 - F32::floor(fTemp38);
					fTemp39 = F32::max(self.fConst7 * F32::min(1.8 * fTemp31, self.fConst1), 1e-06);
					fTemp40 = if iTemp28 != 0 {0.0} else {self.fRec26[1] + self.fConst11 * fTemp31};
					self.fRec26[0] = fTemp40 - F32::floor(fTemp40);
					fTemp41 = F32::max(self.fConst7 * F32::min(1.483 * fTemp31, self.fConst1), 1e-06);
					fTemp42 = if iTemp28 != 0 {0.0} else {self.fRec27[1] + self.fConst12 * fTemp31};
					self.fRec27[0] = fTemp42 - F32::floor(fTemp42);
					fTemp43 = F32::max(self.fConst7 * F32::min(fTemp31, self.fConst1), 1e-06);
					fTemp44 = if iTemp28 != 0 {0.0} else {self.fRec28[1] + self.fConst7 * fTemp31};
					self.fRec28[0] = fTemp44 - F32::floor(fTemp44);
					fTemp45 = self.fRec28[0] + -1.0;
					fTemp46 = 1.0 - fTemp43;
					fTemp47 = self.fRec27[0] + -1.0;
					fTemp48 = 1.0 - fTemp41;
					fTemp49 = self.fRec26[0] + -1.0;
					fTemp50 = 1.0 - fTemp39;
					fTemp51 = self.fRec25[0] + -1.0;
					fTemp52 = 1.0 - fTemp37;
					fTemp53 = self.fRec24[0] + -1.0;
					fTemp54 = 1.0 - fTemp35;
					fTemp55 = self.fRec23[0] + -1.0;
					fTemp56 = 1.0 - fTemp33;
					fTemp57 = F32::floor(self.fRec28[0] + 0.5);
					fTemp58 = self.fRec28[0] + (0.5 - fTemp57);
					fTemp59 = self.fRec28[0] + (-0.5 - fTemp57);
					fTemp60 = F32::floor(self.fRec27[0] + 0.5);
					fTemp61 = self.fRec27[0] + (0.5 - fTemp60);
					fTemp62 = self.fRec27[0] + (-0.5 - fTemp60);
					fTemp63 = F32::floor(self.fRec26[0] + 0.5);
					fTemp64 = self.fRec26[0] + (0.5 - fTemp63);
					fTemp65 = self.fRec26[0] + (-0.5 - fTemp63);
					fTemp66 = F32::floor(self.fRec25[0] + 0.5);
					fTemp67 = self.fRec25[0] + (0.5 - fTemp66);
					fTemp68 = self.fRec25[0] + (-0.5 - fTemp66);
					fTemp69 = F32::floor(self.fRec24[0] + 0.5);
					fTemp70 = self.fRec24[0] + (0.5 - fTemp69);
					fTemp71 = self.fRec24[0] + (-0.5 - fTemp69);
					fTemp72 = F32::floor(self.fRec23[0] + 0.5);
					fTemp73 = self.fRec23[0] + (0.5 - fTemp72);
					fTemp74 = self.fRec23[0] + (-0.5 - fTemp72);
					fTemp75 = 0.23333333 * fTemp30 * (((fTemp73 > fTemp56) as i32) as u32 as F32 * (fTemp74 * (fTemp74 / fTemp33 + 2.0) / fTemp33 + 1.0) + ((fTemp73 < fTemp33) as i32) as u32 as F32 * (fTemp73 * (2.0 - fTemp73 / fTemp33) / fTemp33 + -1.0) + ((fTemp70 > fTemp54) as i32) as u32 as F32 * (fTemp71 * (fTemp71 / fTemp35 + 2.0) / fTemp35 + 1.0) + ((fTemp70 < fTemp35) as i32) as u32 as F32 * (fTemp70 * (2.0 - fTemp70 / fTemp35) / fTemp35 + -1.0) + ((fTemp67 > fTemp52) as i32) as u32 as F32 * (fTemp68 * (fTemp68 / fTemp37 + 2.0) / fTemp37 + 1.0) + ((fTemp67 < fTemp37) as i32) as u32 as F32 * (fTemp67 * (2.0 - fTemp67 / fTemp37) / fTemp37 + -1.0) + ((fTemp64 > fTemp50) as i32) as u32 as F32 * (fTemp65 * (fTemp65 / fTemp39 + 2.0) / fTemp39 + 1.0) + ((fTemp64 < fTemp39) as i32) as u32 as F32 * (fTemp64 * (2.0 - fTemp64 / fTemp39) / fTemp39 + -1.0) + ((fTemp61 > fTemp48) as i32) as u32 as F32 * (fTemp62 * (fTemp62 / fTemp41 + 2.0) / fTemp41 + 1.0) + ((fTemp61 < fTemp41) as i32) as u32 as F32 * (fTemp61 * (2.0 - fTemp61 / fTemp41) / fTemp41 + -1.0) + ((fTemp58 > fTemp46) as i32) as u32 as F32 * (fTemp59 * (fTemp59 / fTemp43 + 2.0) / fTemp43 + 1.0) + ((fTemp58 < fTemp43) as i32) as u32 as F32 * (fTemp58 * (2.0 - fTemp58 / fTemp43) / fTemp43 + -1.0) + (if (self.fRec28[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec27[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec26[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec25[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec23[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec24[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 - (((self.fRec23[0] > fTemp56) as i32) as u32 as F32 * (fTemp55 * (fTemp55 / fTemp33 + 2.0) / fTemp33 + 1.0) + ((self.fRec24[0] > fTemp54) as i32) as u32 as F32 * (fTemp53 * (fTemp53 / fTemp35 + 2.0) / fTemp35 + 1.0) + ((self.fRec25[0] > fTemp52) as i32) as u32 as F32 * (fTemp51 * (fTemp51 / fTemp37 + 2.0) / fTemp37 + 1.0) + ((self.fRec26[0] > fTemp50) as i32) as u32 as F32 * (fTemp49 * (fTemp49 / fTemp39 + 2.0) / fTemp39 + 1.0) + ((self.fRec27[0] > fTemp48) as i32) as u32 as F32 * (fTemp47 * (fTemp47 / fTemp41 + 2.0) / fTemp41 + 1.0) + ((self.fRec28[0] < fTemp43) as i32) as u32 as F32 * (self.fRec28[0] * (2.0 - self.fRec28[0] / fTemp43) / fTemp43 + -1.0) + ((self.fRec28[0] > fTemp46) as i32) as u32 as F32 * (fTemp45 * (fTemp45 / fTemp43 + 2.0) / fTemp43 + 1.0) + ((self.fRec27[0] < fTemp41) as i32) as u32 as F32 * (self.fRec27[0] * (2.0 - self.fRec27[0] / fTemp41) / fTemp41 + -1.0) + ((self.fRec26[0] < fTemp39) as i32) as u32 as F32 * (self.fRec26[0] * (2.0 - self.fRec26[0] / fTemp39) / fTemp39 + -1.0) + ((self.fRec25[0] < fTemp37) as i32) as u32 as F32 * (self.fRec25[0] * (2.0 - self.fRec25[0] / fTemp37) / fTemp37 + -1.0) + ((self.fRec24[0] < fTemp35) as i32) as u32 as F32 * (self.fRec24[0] * (2.0 - self.fRec24[0] / fTemp35) / fTemp35 + -1.0) + ((self.fRec23[0] < fTemp33) as i32) as u32 as F32 * (self.fRec23[0] * (2.0 - self.fRec23[0] / fTemp33) / fTemp33 + -1.0)));
				}
					if (fTemp32 != 0.0) as i32 != 0 {
						fTemp76 = F32::max(1.0, F32::min(self.fConst13, fTemp31));
						fTemp77 = if iTemp28 != 0 {0.0} else {self.fRec31[1] + self.fConst7 * fTemp76};
						self.fRec31[0] = fTemp77 - F32::floor(fTemp77);
						iTemp78 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec32[1], 12345));
						iTemp79 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp78, 12345));
						iTemp80 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp79, 12345));
						self.iRec32[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp80, 12345));
						iRec33 = iTemp80;
						iRec34 = iTemp79;
						self.fRec30[0] = if (self.fRec31[0] < self.fRec31[1]) as i32 != 0 {4.656613e-10 * (self.iRec32[0]) as F32} else {self.fRec30[1]};
						iTemp81 = ((self.fRec31[0] < 0.3) as i32) * ((self.fRec30[0] > -0.6) as i32);
						fTemp82 = if iTemp28 != 0 {0.0} else {self.fRec37[1] + self.fConst14 * fTemp76};
						self.fRec37[0] = fTemp82 - F32::floor(fTemp82);
						self.fRec36[0] = if (self.fRec37[0] < self.fRec37[1]) as i32 != 0 {4.656613e-10 * (iRec33) as F32} else {self.fRec36[1]};
						fTemp83 = if iTemp28 != 0 {0.0} else {self.fRec35[1] + self.fConst7 * F32::max(2e+01, F32::min(self.fConst1, fTemp7 * F32::exp(0.1732868 * (F32::floor(F32::max(0.0, F32::min(3.999, 2.0 * (self.fRec36[0] + 1.0)))) + -1.5)) * (0.3 * NoiseDsp_faustpower2_f(F32::max(0.0, F32::min(1.0, 1.0 - 3.3333333 * self.fRec31[0]))) + 1.0)))};
						self.fRec35[0] = fTemp83 - F32::floor(fTemp83);
						fTemp84 = F32::sin(6.2831855 * self.fRec35[0]);
						fTemp85 = F32::tanh(3.0 * (fTemp84 + 0.8 * F32::abs(fTemp84))) * (iTemp81) as F32 + 5.5879353e-11 * (iRec34) as F32 * (i32::wrapping_sub(1, iTemp81)) as F32;
						self.fVec1[0] = fTemp85;
						self.fRec29[0] = -(self.fConst17 * (self.fConst16 * self.fRec29[1] - self.fConst15 * (fTemp85 - self.fVec1[1])));
						fTemp86 = 1.7 * self.fRec29[0] * fTemp32;
					}
						let mut fTemp87: F32 = F32::cos(fTemp22) * if (fTemp23 >= 3.0) as i32 != 0 {if (fTemp23 >= 5.0) as i32 != 0 {fTemp86} else {if (fTemp23 >= 4.0) as i32 != 0 {fTemp75} else {0.00048828125 * F32::floor(2048.0 * self.fRec21[0] + 0.5)}}} else {if (fTemp23 >= 2.0) as i32 != 0 {F32::tanh(1.7229468e-09 * (iRec20) as F32 * (((0.5 * (4.656613e-10 * (iRec19) as F32 + 1.0)) < self.fConst5) as i32) as u32 as F32)} else {if (fTemp23 >= 1.0) as i32 != 0 {11.6 * (0.049922034 * self.fRec17[0] + 0.0506127 * self.fRec17[2] - (0.095993534 * self.fRec17[1] + 0.004408786 * self.fRec17[3]))} else {fTemp24}}} + 4.0 * sl_loop((*input3) as i32, *input4) * F32::sin(fTemp22);
						let mut fTemp88: F32 = F32::max(0.0, F32::min(1.0, fTemp21));
						let mut fTemp89: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp88)) + -1.0));
						let mut fTemp90: F32 = self.fConst0 / (23.0 * NoiseDsp_faustpower2_f(fTemp88) + 1.0);
						let mut fTemp91: F32 = if iTemp28 != 0 {0.0} else {self.fRec39[1] + self.fConst7 * F32::max(fTemp90, 0.0)};
						self.fRec39[0] = fTemp91 - F32::floor(fTemp91);
						self.fRec38[0] = if ((self.fRec39[0] < self.fRec39[1]) as i32) | ((fTemp90 >= self.fConst6) as i32) != 0 {fTemp87} else {self.fRec38[1]};
						let mut fTemp92: F32 = if (fTemp21 > 0.0001) as i32 != 0 {F32::floor(self.fRec38[0] * fTemp89 + 0.5) / fTemp89} else {fTemp87} * (fTemp19 + self.fRec13[0] * (1.0 - fTemp19));
						let mut fTemp93: F32 = fTemp92 - (5.0 * fTemp9 * fTemp17 + self.fRec11[1]);
						let mut fTemp94: F32 = fTemp8 * fTemp93 / fTemp10;
						let mut fTemp95: F32 = 5.0 * fTemp17;
						self.fRec10[0] = fTemp95 + 2.0 * fTemp94;
						let mut fTemp96: F32 = fTemp95 + fTemp94;
						let mut fTemp97: F32 = fTemp8 * fTemp96;
						self.fRec11[0] = self.fRec11[1] + 2.0 * fTemp97;
						let mut fRec12: F32 = fTemp96;
						let mut fTemp98: F32 = fTemp92 * (1.0 - fTemp16) + 1.1111112 * fRec12 * fTemp16;
						let mut fTemp99: F32 = fTemp98 - (5.0 * fTemp9 * fTemp15 + self.fRec8[1]);
						let mut fTemp100: F32 = fTemp8 * fTemp99 / fTemp10;
						let mut fTemp101: F32 = 5.0 * fTemp15;
						self.fRec7[0] = fTemp101 + 2.0 * fTemp100;
						let mut fTemp102: F32 = fTemp101 + fTemp100;
						let mut fTemp103: F32 = fTemp8 * fTemp102;
						self.fRec8[0] = self.fRec8[1] + 2.0 * fTemp103;
						let mut fRec9: F32 = fTemp102;
						let mut fTemp104: F32 = fTemp98 * (1.0 - fTemp14) + 1.1111112 * fRec9 * fTemp14;
						let mut fTemp105: F32 = fTemp104 - (5.0 * fTemp9 * fTemp13 + self.fRec5[1]);
						let mut fTemp106: F32 = fTemp8 * fTemp105 / fTemp10;
						let mut fTemp107: F32 = 5.0 * fTemp13;
						self.fRec4[0] = fTemp107 + 2.0 * fTemp106;
						let mut fTemp108: F32 = fTemp107 + fTemp106;
						let mut fTemp109: F32 = fTemp8 * fTemp108;
						self.fRec5[0] = self.fRec5[1] + 2.0 * fTemp109;
						let mut fRec6: F32 = fTemp108;
						let mut fTemp110: F32 = fTemp104 * (1.0 - fTemp12) + 1.1111112 * fRec6 * fTemp12;
						let mut fTemp111: F32 = fTemp110 - (5.0 * fTemp9 * fTemp11 + self.fRec2[1]);
						let mut fTemp112: F32 = fTemp8 * fTemp111 / fTemp10;
						let mut fTemp113: F32 = 5.0 * fTemp11;
						self.fRec1[0] = fTemp113 + 2.0 * fTemp112;
						let mut fTemp114: F32 = fTemp113 + fTemp112;
						let mut fTemp115: F32 = fTemp8 * fTemp114;
						self.fRec2[0] = self.fRec2[1] + 2.0 * fTemp115;
						let mut fRec3: F32 = fTemp114;
						self.fRec0[0] = (fTemp110 * (1.0 - fTemp6) + 1.1111112 * fRec3 * fTemp6) * F32::powf(1e+01, 0.05 * (12.0 * F32::powf(F32::max(0.0, F32::min(1.0, fTemp4)), 0.35) + -4.4)) - (self.fRec0[2] * ((fTemp1 + -1.4142135) / fTemp0 + 1.0) + 2.0 * self.fRec0[1] * (1.0 - 1.0 / fTemp3)) / fTemp2;
						*output0 = *input10 * (self.fRec0[0] + self.fRec0[2] - 2.0 * self.fRec0[1]) / (fTemp3 * fTemp2);
						self.iVec0[1] = self.iVec0[0];
						self.iRec14[1] = self.iRec14[0];
						self.fRec13[1] = self.fRec13[0];
						self.iRec16[1] = self.iRec16[0];
						for j0 in (1..=3).rev() {
							self.fRec17[j0 as usize] = self.fRec17[(i32::wrapping_sub(j0, 1)) as usize];
						}
						self.iRec18[1] = self.iRec18[0];
						self.fRec22[1] = self.fRec22[0];
						self.fRec21[1] = self.fRec21[0];
						if (fTemp30 != 0.0) as i32 != 0 {
							self.fRec23[1] = self.fRec23[0];
							self.fRec24[1] = self.fRec24[0];
							self.fRec25[1] = self.fRec25[0];
							self.fRec26[1] = self.fRec26[0];
							self.fRec27[1] = self.fRec27[0];
							self.fRec28[1] = self.fRec28[0];
						}
							if (fTemp32 != 0.0) as i32 != 0 {
								self.fRec31[1] = self.fRec31[0];
								self.iRec32[1] = self.iRec32[0];
								self.fRec30[1] = self.fRec30[0];
								self.fRec37[1] = self.fRec37[0];
								self.fRec36[1] = self.fRec36[0];
								self.fRec35[1] = self.fRec35[0];
								self.fVec1[1] = self.fVec1[0];
								self.fRec29[1] = self.fRec29[0];
							}
								self.fRec39[1] = self.fRec39[0];
								self.fRec38[1] = self.fRec38[0];
								self.fRec10[1] = self.fRec10[0];
								self.fRec11[1] = self.fRec11[0];
								self.fRec7[1] = self.fRec7[0];
								self.fRec8[1] = self.fRec8[0];
								self.fRec4[1] = self.fRec4[0];
								self.fRec5[1] = self.fRec5[0];
								self.fRec1[1] = self.fRec1[0];
								self.fRec2[1] = self.fRec2[0];
								self.fRec0[2] = self.fRec0[1];
								self.fRec0[1] = self.fRec0[0];
							}
							self.fTempPerm0 = fTemp31;
							self.fTempPerm1 = fTemp33;
							self.fTempPerm2 = fTemp34;
							self.fTempPerm3 = fTemp35;
							self.fTempPerm4 = fTemp36;
							self.fTempPerm5 = fTemp37;
							self.fTempPerm6 = fTemp38;
							self.fTempPerm7 = fTemp39;
							self.fTempPerm8 = fTemp40;
							self.fTempPerm9 = fTemp41;
							self.fTempPerm10 = fTemp42;
							self.fTempPerm11 = fTemp43;
							self.fTempPerm12 = fTemp44;
							self.fTempPerm13 = fTemp45;
							self.fTempPerm14 = fTemp46;
							self.fTempPerm15 = fTemp47;
							self.fTempPerm16 = fTemp48;
							self.fTempPerm17 = fTemp49;
							self.fTempPerm18 = fTemp50;
							self.fTempPerm19 = fTemp51;
							self.fTempPerm20 = fTemp52;
							self.fTempPerm21 = fTemp53;
							self.fTempPerm22 = fTemp54;
							self.fTempPerm23 = fTemp55;
							self.fTempPerm24 = fTemp56;
							self.fTempPerm25 = fTemp57;
							self.fTempPerm26 = fTemp58;
							self.fTempPerm27 = fTemp59;
							self.fTempPerm28 = fTemp60;
							self.fTempPerm29 = fTemp61;
							self.fTempPerm30 = fTemp62;
							self.fTempPerm31 = fTemp63;
							self.fTempPerm32 = fTemp64;
							self.fTempPerm33 = fTemp65;
							self.fTempPerm34 = fTemp66;
							self.fTempPerm35 = fTemp67;
							self.fTempPerm36 = fTemp68;
							self.fTempPerm37 = fTemp69;
							self.fTempPerm38 = fTemp70;
							self.fTempPerm39 = fTemp71;
							self.fTempPerm40 = fTemp72;
							self.fTempPerm41 = fTemp73;
							self.fTempPerm42 = fTemp74;
							self.fTempPerm43 = fTemp75;
							self.fTempPerm44 = fTemp76;
							self.fTempPerm45 = fTemp77;
							self.iTempPerm46 = iTemp78;
							self.iTempPerm47 = iTemp79;
							self.iTempPerm48 = iTemp80;
							self.iTempPerm49 = iTemp81;
							self.fTempPerm50 = fTemp82;
							self.fTempPerm51 = fTemp83;
							self.fTempPerm52 = fTemp84;
							self.fTempPerm53 = fTemp85;
							self.fTempPerm54 = fTemp86;
						}

}

