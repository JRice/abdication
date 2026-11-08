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
	fConst8: F32,
	fRec23: [F32;2],
	fConst9: F32,
	fRec24: [F32;2],
	fConst10: F32,
	fRec25: [F32;2],
	fConst11: F32,
	fRec26: [F32;2],
	fConst12: F32,
	fRec27: [F32;2],
	fRec28: [F32;2],
	fRec30: [F32;2],
	fRec29: [F32;2],
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
			fConst8: 0.0,
			fRec23: [0.0;2],
			fConst9: 0.0,
			fRec24: [0.0;2],
			fConst10: 0.0,
			fRec25: [0.0;2],
			fConst11: 0.0,
			fRec26: [0.0;2],
			fConst12: 0.0,
			fRec27: [0.0;2],
			fRec28: [0.0;2],
			fRec30: [0.0;2],
			fRec29: [0.0;2],
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
		for l8 in 0..2 {
			self.fRec23[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec24[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec25[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec26[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec27[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec28[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec30[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec29[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec10[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec11[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec7[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec8[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec4[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec5[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec1[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec2[l23 as usize] = 0.0;
		}
		for l24 in 0..3 {
			self.fRec0[l24 as usize] = 0.0;
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
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(inputs4).zip(inputs5).zip(inputs6).zip(inputs7).zip(inputs8).zip(inputs9).zip(inputs10).zip(outputs0);
		for (((((((((((input0, input1), input2), input3), input4), input5), input6), input7), input8), input9), input10), output0) in zipped_iterators {
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
			let mut fTemp30: F32 = *input1;
			let mut fTemp31: F32 = F32::max(self.fConst7 * F32::min(3.897 * fTemp30, self.fConst1), 1e-06);
			let mut fTemp32: F32 = if iTemp28 != 0 {0.0} else {self.fRec23[1] + self.fConst8 * fTemp30};
			self.fRec23[0] = fTemp32 - F32::floor(fTemp32);
			let mut fTemp33: F32 = F32::max(self.fConst7 * F32::min(2.63 * fTemp30, self.fConst1), 1e-06);
			let mut fTemp34: F32 = if iTemp28 != 0 {0.0} else {self.fRec24[1] + self.fConst9 * fTemp30};
			self.fRec24[0] = fTemp34 - F32::floor(fTemp34);
			let mut fTemp35: F32 = F32::max(self.fConst7 * F32::min(2.546 * fTemp30, self.fConst1), 1e-06);
			let mut fTemp36: F32 = if iTemp28 != 0 {0.0} else {self.fRec25[1] + self.fConst10 * fTemp30};
			self.fRec25[0] = fTemp36 - F32::floor(fTemp36);
			let mut fTemp37: F32 = F32::max(self.fConst7 * F32::min(1.8 * fTemp30, self.fConst1), 1e-06);
			let mut fTemp38: F32 = if iTemp28 != 0 {0.0} else {self.fRec26[1] + self.fConst11 * fTemp30};
			self.fRec26[0] = fTemp38 - F32::floor(fTemp38);
			let mut fTemp39: F32 = F32::max(self.fConst7 * F32::min(1.483 * fTemp30, self.fConst1), 1e-06);
			let mut fTemp40: F32 = if iTemp28 != 0 {0.0} else {self.fRec27[1] + self.fConst12 * fTemp30};
			self.fRec27[0] = fTemp40 - F32::floor(fTemp40);
			let mut fTemp41: F32 = F32::max(self.fConst7 * F32::min(fTemp30, self.fConst1), 1e-06);
			let mut fTemp42: F32 = if iTemp28 != 0 {0.0} else {self.fRec28[1] + self.fConst7 * fTemp30};
			self.fRec28[0] = fTemp42 - F32::floor(fTemp42);
			let mut fTemp43: F32 = self.fRec28[0] + -1.0;
			let mut fTemp44: F32 = 1.0 - fTemp41;
			let mut fTemp45: F32 = self.fRec27[0] + -1.0;
			let mut fTemp46: F32 = 1.0 - fTemp39;
			let mut fTemp47: F32 = self.fRec26[0] + -1.0;
			let mut fTemp48: F32 = 1.0 - fTemp37;
			let mut fTemp49: F32 = self.fRec25[0] + -1.0;
			let mut fTemp50: F32 = 1.0 - fTemp35;
			let mut fTemp51: F32 = self.fRec24[0] + -1.0;
			let mut fTemp52: F32 = 1.0 - fTemp33;
			let mut fTemp53: F32 = self.fRec23[0] + -1.0;
			let mut fTemp54: F32 = 1.0 - fTemp31;
			let mut fTemp55: F32 = F32::floor(self.fRec28[0] + 0.5);
			let mut fTemp56: F32 = self.fRec28[0] + (0.5 - fTemp55);
			let mut fTemp57: F32 = self.fRec28[0] + (-0.5 - fTemp55);
			let mut fTemp58: F32 = F32::floor(self.fRec27[0] + 0.5);
			let mut fTemp59: F32 = self.fRec27[0] + (0.5 - fTemp58);
			let mut fTemp60: F32 = self.fRec27[0] + (-0.5 - fTemp58);
			let mut fTemp61: F32 = F32::floor(self.fRec26[0] + 0.5);
			let mut fTemp62: F32 = self.fRec26[0] + (0.5 - fTemp61);
			let mut fTemp63: F32 = self.fRec26[0] + (-0.5 - fTemp61);
			let mut fTemp64: F32 = F32::floor(self.fRec25[0] + 0.5);
			let mut fTemp65: F32 = self.fRec25[0] + (0.5 - fTemp64);
			let mut fTemp66: F32 = self.fRec25[0] + (-0.5 - fTemp64);
			let mut fTemp67: F32 = F32::floor(self.fRec24[0] + 0.5);
			let mut fTemp68: F32 = self.fRec24[0] + (0.5 - fTemp67);
			let mut fTemp69: F32 = self.fRec24[0] + (-0.5 - fTemp67);
			let mut fTemp70: F32 = F32::floor(self.fRec23[0] + 0.5);
			let mut fTemp71: F32 = self.fRec23[0] + (0.5 - fTemp70);
			let mut fTemp72: F32 = self.fRec23[0] + (-0.5 - fTemp70);
			let mut fTemp73: F32 = F32::cos(fTemp22) * if (fTemp23 >= 3.0) as i32 != 0 {if (fTemp23 >= 4.0) as i32 != 0 {0.23333333 * (((fTemp71 > fTemp54) as i32) as u32 as F32 * (fTemp72 * (fTemp72 / fTemp31 + 2.0) / fTemp31 + 1.0) + ((fTemp71 < fTemp31) as i32) as u32 as F32 * (fTemp71 * (2.0 - fTemp71 / fTemp31) / fTemp31 + -1.0) + ((fTemp68 > fTemp52) as i32) as u32 as F32 * (fTemp69 * (fTemp69 / fTemp33 + 2.0) / fTemp33 + 1.0) + ((fTemp68 < fTemp33) as i32) as u32 as F32 * (fTemp68 * (2.0 - fTemp68 / fTemp33) / fTemp33 + -1.0) + ((fTemp65 > fTemp50) as i32) as u32 as F32 * (fTemp66 * (fTemp66 / fTemp35 + 2.0) / fTemp35 + 1.0) + ((fTemp65 < fTemp35) as i32) as u32 as F32 * (fTemp65 * (2.0 - fTemp65 / fTemp35) / fTemp35 + -1.0) + ((fTemp62 > fTemp48) as i32) as u32 as F32 * (fTemp63 * (fTemp63 / fTemp37 + 2.0) / fTemp37 + 1.0) + ((fTemp62 < fTemp37) as i32) as u32 as F32 * (fTemp62 * (2.0 - fTemp62 / fTemp37) / fTemp37 + -1.0) + ((fTemp59 > fTemp46) as i32) as u32 as F32 * (fTemp60 * (fTemp60 / fTemp39 + 2.0) / fTemp39 + 1.0) + ((fTemp59 < fTemp39) as i32) as u32 as F32 * (fTemp59 * (2.0 - fTemp59 / fTemp39) / fTemp39 + -1.0) + ((fTemp56 > fTemp44) as i32) as u32 as F32 * (fTemp57 * (fTemp57 / fTemp41 + 2.0) / fTemp41 + 1.0) + ((fTemp56 < fTemp41) as i32) as u32 as F32 * (fTemp56 * (2.0 - fTemp56 / fTemp41) / fTemp41 + -1.0) + (if (self.fRec28[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec27[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec26[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec25[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec23[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec24[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 - (((self.fRec23[0] > fTemp54) as i32) as u32 as F32 * (fTemp53 * (fTemp53 / fTemp31 + 2.0) / fTemp31 + 1.0) + ((self.fRec24[0] > fTemp52) as i32) as u32 as F32 * (fTemp51 * (fTemp51 / fTemp33 + 2.0) / fTemp33 + 1.0) + ((self.fRec25[0] > fTemp50) as i32) as u32 as F32 * (fTemp49 * (fTemp49 / fTemp35 + 2.0) / fTemp35 + 1.0) + ((self.fRec26[0] > fTemp48) as i32) as u32 as F32 * (fTemp47 * (fTemp47 / fTemp37 + 2.0) / fTemp37 + 1.0) + ((self.fRec27[0] > fTemp46) as i32) as u32 as F32 * (fTemp45 * (fTemp45 / fTemp39 + 2.0) / fTemp39 + 1.0) + ((self.fRec28[0] < fTemp41) as i32) as u32 as F32 * (self.fRec28[0] * (2.0 - self.fRec28[0] / fTemp41) / fTemp41 + -1.0) + ((self.fRec28[0] > fTemp44) as i32) as u32 as F32 * (fTemp43 * (fTemp43 / fTemp41 + 2.0) / fTemp41 + 1.0) + ((self.fRec27[0] < fTemp39) as i32) as u32 as F32 * (self.fRec27[0] * (2.0 - self.fRec27[0] / fTemp39) / fTemp39 + -1.0) + ((self.fRec26[0] < fTemp37) as i32) as u32 as F32 * (self.fRec26[0] * (2.0 - self.fRec26[0] / fTemp37) / fTemp37 + -1.0) + ((self.fRec25[0] < fTemp35) as i32) as u32 as F32 * (self.fRec25[0] * (2.0 - self.fRec25[0] / fTemp35) / fTemp35 + -1.0) + ((self.fRec24[0] < fTemp33) as i32) as u32 as F32 * (self.fRec24[0] * (2.0 - self.fRec24[0] / fTemp33) / fTemp33 + -1.0) + ((self.fRec23[0] < fTemp31) as i32) as u32 as F32 * (self.fRec23[0] * (2.0 - self.fRec23[0] / fTemp31) / fTemp31 + -1.0)))} else {0.00048828125 * F32::floor(2048.0 * self.fRec21[0] + 0.5)}} else {if (fTemp23 >= 2.0) as i32 != 0 {F32::tanh(1.7229468e-09 * (iRec20) as F32 * (((0.5 * (4.656613e-10 * (iRec19) as F32 + 1.0)) < self.fConst5) as i32) as u32 as F32)} else {if (fTemp23 >= 1.0) as i32 != 0 {11.6 * (0.049922034 * self.fRec17[0] + 0.0506127 * self.fRec17[2] - (0.095993534 * self.fRec17[1] + 0.004408786 * self.fRec17[3]))} else {fTemp24}}} + 4.0 * sl_loop((*input3) as i32, *input4) * F32::sin(fTemp22);
			let mut fTemp74: F32 = F32::max(0.0, F32::min(1.0, fTemp21));
			let mut fTemp75: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp74)) + -1.0));
			let mut fTemp76: F32 = self.fConst0 / (23.0 * NoiseDsp_faustpower2_f(fTemp74) + 1.0);
			let mut fTemp77: F32 = if iTemp28 != 0 {0.0} else {self.fRec30[1] + self.fConst7 * F32::max(fTemp76, 0.0)};
			self.fRec30[0] = fTemp77 - F32::floor(fTemp77);
			self.fRec29[0] = if ((self.fRec30[0] < self.fRec30[1]) as i32) | ((fTemp76 >= self.fConst6) as i32) != 0 {fTemp73} else {self.fRec29[1]};
			let mut fTemp78: F32 = if (fTemp21 > 0.0001) as i32 != 0 {F32::floor(self.fRec29[0] * fTemp75 + 0.5) / fTemp75} else {fTemp73} * (fTemp19 + self.fRec13[0] * (1.0 - fTemp19));
			let mut fTemp79: F32 = fTemp78 - (5.0 * fTemp9 * fTemp17 + self.fRec11[1]);
			let mut fTemp80: F32 = fTemp8 * fTemp79 / fTemp10;
			let mut fTemp81: F32 = 5.0 * fTemp17;
			self.fRec10[0] = fTemp81 + 2.0 * fTemp80;
			let mut fTemp82: F32 = fTemp81 + fTemp80;
			let mut fTemp83: F32 = fTemp8 * fTemp82;
			self.fRec11[0] = self.fRec11[1] + 2.0 * fTemp83;
			let mut fRec12: F32 = fTemp82;
			let mut fTemp84: F32 = fTemp78 * (1.0 - fTemp16) + 1.1111112 * fRec12 * fTemp16;
			let mut fTemp85: F32 = fTemp84 - (5.0 * fTemp9 * fTemp15 + self.fRec8[1]);
			let mut fTemp86: F32 = fTemp8 * fTemp85 / fTemp10;
			let mut fTemp87: F32 = 5.0 * fTemp15;
			self.fRec7[0] = fTemp87 + 2.0 * fTemp86;
			let mut fTemp88: F32 = fTemp87 + fTemp86;
			let mut fTemp89: F32 = fTemp8 * fTemp88;
			self.fRec8[0] = self.fRec8[1] + 2.0 * fTemp89;
			let mut fRec9: F32 = fTemp88;
			let mut fTemp90: F32 = fTemp84 * (1.0 - fTemp14) + 1.1111112 * fRec9 * fTemp14;
			let mut fTemp91: F32 = fTemp90 - (5.0 * fTemp9 * fTemp13 + self.fRec5[1]);
			let mut fTemp92: F32 = fTemp8 * fTemp91 / fTemp10;
			let mut fTemp93: F32 = 5.0 * fTemp13;
			self.fRec4[0] = fTemp93 + 2.0 * fTemp92;
			let mut fTemp94: F32 = fTemp93 + fTemp92;
			let mut fTemp95: F32 = fTemp8 * fTemp94;
			self.fRec5[0] = self.fRec5[1] + 2.0 * fTemp95;
			let mut fRec6: F32 = fTemp94;
			let mut fTemp96: F32 = fTemp90 * (1.0 - fTemp12) + 1.1111112 * fRec6 * fTemp12;
			let mut fTemp97: F32 = fTemp96 - (5.0 * fTemp9 * fTemp11 + self.fRec2[1]);
			let mut fTemp98: F32 = fTemp8 * fTemp97 / fTemp10;
			let mut fTemp99: F32 = 5.0 * fTemp11;
			self.fRec1[0] = fTemp99 + 2.0 * fTemp98;
			let mut fTemp100: F32 = fTemp99 + fTemp98;
			let mut fTemp101: F32 = fTemp8 * fTemp100;
			self.fRec2[0] = self.fRec2[1] + 2.0 * fTemp101;
			let mut fRec3: F32 = fTemp100;
			self.fRec0[0] = (fTemp96 * (1.0 - fTemp6) + 1.1111112 * fRec3 * fTemp6) * F32::powf(1e+01, 0.05 * (12.0 * F32::powf(F32::max(0.0, F32::min(1.0, fTemp4)), 0.35) + -4.4)) - (self.fRec0[2] * ((fTemp1 + -1.4142135) / fTemp0 + 1.0) + 2.0 * self.fRec0[1] * (1.0 - 1.0 / fTemp3)) / fTemp2;
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
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec29[1] = self.fRec29[0];
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
	}

}

