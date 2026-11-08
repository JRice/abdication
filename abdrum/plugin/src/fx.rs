/* ------------------------------------------------------------
name: "ABDrumFX"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn FxDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn FxDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct FxDsp {
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fHslider0: F32,
	fRec0: [F32;2],
	fHslider1: F32,
	fRec1: [F32;2],
	fHslider2: F32,
	fConst3: F32,
	fRec2: [F32;2],
	fHslider3: F32,
	fRec3: [F32;2],
	fButton0: F32,
	fVec1: [F32;2],
	iRec4: [i32;2],
	fConst4: F32,
	fHslider4: F32,
	fRec5: [F32;2],
	fHslider5: F32,
	fRec6: [F32;2],
	fHslider6: F32,
	fRec7: [F32;2],
	iRec8: [i32;2],
	fHslider7: F32,
	fRec10: [F32;2],
	fHslider8: F32,
	fRec11: [F32;2],
	fHslider9: F32,
	fRec12: [F32;2],
	fHslider10: F32,
	fRec13: [F32;2],
	fHslider11: F32,
	fRec14: [F32;2],
	fHslider12: F32,
	fRec15: [F32;2],
	fHslider13: F32,
	fRec16: [F32;2],
	fHslider14: F32,
	fRec17: [F32;2],
	fConst5: F32,
	fConst6: F32,
	fRec19: [F32;2],
	fRec18: [F32;2],
	fVec2: [F32;2],
	fEntry0: F32,
	fConst7: F32,
	fRec24: [F32;2],
	fVec3: [F32;2],
	fRec23: [F32;2],
	fVec4: [F32;2],
	fVec5: [F32;2],
	fRec22: [F32;2],
	fVec6: [F32;2],
	fRec21: [F32;2],
	fConst9: F32,
	fConst10: F32,
	fRec20: [F32;2],
	fVec7: [F32;2],
	fConst11: F32,
	fConst12: F32,
	fConst13: F32,
	fRec25: [F32;2],
	fConst14: F32,
	fHslider15: F32,
	fRec26: [F32;2],
	IOTA0: i32,
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fConst18: F32,
	fVec8: [F32;2],
	fRec32: [F32;2],
	fRec31: [F32;2],
	fRec30: [F32;2],
	fRec29: [F32;2],
	fConst20: F32,
	fConst21: F32,
	fRec28: [F32;2],
	fRec27: [F32;8192],
	fEntry1: F32,
	fConst22: F32,
	fRec9: [F32;2],
}

impl FaustDsp for FxDsp {
	type T = F32;
		
	fn new() -> FxDsp { 
		FxDsp {
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider0: 0.0,
			fRec0: [0.0;2],
			fHslider1: 0.0,
			fRec1: [0.0;2],
			fHslider2: 0.0,
			fConst3: 0.0,
			fRec2: [0.0;2],
			fHslider3: 0.0,
			fRec3: [0.0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			iRec4: [0;2],
			fConst4: 0.0,
			fHslider4: 0.0,
			fRec5: [0.0;2],
			fHslider5: 0.0,
			fRec6: [0.0;2],
			fHslider6: 0.0,
			fRec7: [0.0;2],
			iRec8: [0;2],
			fHslider7: 0.0,
			fRec10: [0.0;2],
			fHslider8: 0.0,
			fRec11: [0.0;2],
			fHslider9: 0.0,
			fRec12: [0.0;2],
			fHslider10: 0.0,
			fRec13: [0.0;2],
			fHslider11: 0.0,
			fRec14: [0.0;2],
			fHslider12: 0.0,
			fRec15: [0.0;2],
			fHslider13: 0.0,
			fRec16: [0.0;2],
			fHslider14: 0.0,
			fRec17: [0.0;2],
			fConst5: 0.0,
			fConst6: 0.0,
			fRec19: [0.0;2],
			fRec18: [0.0;2],
			fVec2: [0.0;2],
			fEntry0: 0.0,
			fConst7: 0.0,
			fRec24: [0.0;2],
			fVec3: [0.0;2],
			fRec23: [0.0;2],
			fVec4: [0.0;2],
			fVec5: [0.0;2],
			fRec22: [0.0;2],
			fVec6: [0.0;2],
			fRec21: [0.0;2],
			fConst9: 0.0,
			fConst10: 0.0,
			fRec20: [0.0;2],
			fVec7: [0.0;2],
			fConst11: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fRec25: [0.0;2],
			fConst14: 0.0,
			fHslider15: 0.0,
			fRec26: [0.0;2],
			IOTA0: 0,
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fConst18: 0.0,
			fVec8: [0.0;2],
			fRec32: [0.0;2],
			fRec31: [0.0;2],
			fRec30: [0.0;2],
			fRec29: [0.0;2],
			fConst20: 0.0,
			fConst21: 0.0,
			fRec28: [0.0;2],
			fRec27: [0.0;8192],
			fEntry1: 0.0,
			fConst22: 0.0,
			fRec9: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn FxDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("drum.lib/name", r"abdrum-layers");
		m.declare("drum.lib/version", r"0.1");
		m.declare("filename", r"plugin-fx.dsp");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("fx.lib/name", r"abdrum-fx");
		m.declare("fx.lib/version", r"0.1");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"ABDrumFX");
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
		return 1;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.0;
		self.fHslider2 = 3e+02;
		self.fHslider3 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider4 = 0.5;
		self.fHslider5 = 1.0;
		self.fHslider6 = 0.5;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 0.0;
		self.fEntry0 = 0.0;
		self.fHslider15 = 5e+01;
		self.fEntry1 = 2.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
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
			self.fVec1[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.iRec4[l6 as usize] = 0;
		}
		for l7 in 0..2 {
			self.fRec5[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec6[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec7[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.iRec8[l10 as usize] = 0;
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
			self.fRec19[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec18[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fVec2[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec24[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fVec3[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec23[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec4[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fVec5[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec22[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fVec6[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec21[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec20[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec7[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec25[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec26[l33 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l34 in 0..2 {
			self.fVec8[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec32[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec31[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec30[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec29[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec28[l39 as usize] = 0.0;
		}
		for l40 in 0..8192 {
			self.fRec27[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec9[l41 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 0.0441 / self.fConst0;
		self.fConst4 = 1.0 / self.fConst0;
		self.fConst5 = 0.9999 * self.fConst0;
		self.fConst6 = 2e+03 / self.fConst0;
		self.fConst7 = 2.0 * self.fConst0;
		let mut fConst8: F32 = 1.0 / F32::tan(21991.148 / self.fConst0);
		self.fConst9 = 1.0 - fConst8;
		self.fConst10 = 1.0 / (fConst8 + 1.0);
		self.fConst11 = 1.0 / F32::tan(15.707963 / self.fConst0);
		self.fConst12 = 1.0 - self.fConst11;
		self.fConst13 = 1.0 / (self.fConst11 + 1.0);
		self.fConst14 = 0.45 * self.fConst0;
		self.fConst15 = 0.2 * self.fConst0;
		self.fConst16 = 6.2831855 / self.fConst0;
		self.fConst17 = 9.424778 / self.fConst0;
		self.fConst18 = 0.15915494 * self.fConst0;
		let mut fConst19: F32 = 1.0 / F32::tan(28274.334 / self.fConst0);
		self.fConst20 = 1.0 - fConst19;
		self.fConst21 = 1.0 / (fConst19 + 1.0);
		self.fConst22 = 1e+03 / self.fConst0;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		FxDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("ABDrumFX");
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Compressor");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.add_horizontal_slider("Threshold Mod", ParamIndex(0), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(1)), "0", "");
		ui_interface.declare(Some(ParamIndex(1)), "tooltip", "0 dB is off. 4:1, hard knee, auto makeup");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "dB");
		ui_interface.add_horizontal_slider("Threshold", ParamIndex(1), 0.0, -4e+01, 0.0, 0.1);
		ui_interface.declare(Some(ParamIndex(2)), "1", "");
		ui_interface.declare(Some(ParamIndex(2)), "style", "menu{'Squash 0.3/50 ms':0;'Tight 1/80 ms':1;'Punch 3/120 ms':2;'Snap 10/200 ms':3;'Open 30/400 ms':4}");
		ui_interface.add_num_entry("Timing", ParamIndex(2), 2.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Crush");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(3), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "0", "");
		ui_interface.declare(Some(ParamIndex(4)), "tooltip", "Lowers the sample rate (to 2 kHz), then the bit depth (to 4)");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(4), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "7", "");
		ui_interface.open_horizontal_box("Drive");
		ui_interface.declare(Some(ParamIndex(5)), "0", "");
		ui_interface.declare(Some(ParamIndex(5)), "style", "menu{'Tape':0;'Tube':1;'Clip':2}");
		ui_interface.add_num_entry("Type", ParamIndex(5), 0.0, 0.0, 2.0, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(6), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "1", "");
		ui_interface.declare(Some(ParamIndex(7)), "tooltip", "First third: blends the drive under the clean hit. Above: all drive, harder");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(7), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("Ring Mod");
		ui_interface.declare(Some(ParamIndex(8)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(8), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "0", "");
		ui_interface.declare(Some(ParamIndex(9)), "tooltip", "Deepens the ring mod as its carrier climbs from 1.07x to 3.7x Tone's pitch");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(9), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("Comb");
		ui_interface.declare(Some(ParamIndex(10)), "0", "");
		ui_interface.add_horizontal_slider("Amount Mod", ParamIndex(10), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(11)), "0", "");
		ui_interface.declare(Some(ParamIndex(11)), "tooltip", "Feedback (ring length) and mix together. Tuned to Tone's pitch, inharmonic");
		ui_interface.add_horizontal_slider("Amount", ParamIndex(11), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.add_horizontal_slider("env_attack", ParamIndex(12), 0.0, 0.0, 1e+03, 0.1);
		ui_interface.add_horizontal_slider("env_curve", ParamIndex(13), 0.5, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("env_decay", ParamIndex(14), 3e+02, 5.0, 4e+03, 0.1);
		ui_interface.add_horizontal_slider("env_velocity", ParamIndex(15), 0.5, 0.0, 1.0, 0.001);
		ui_interface.add_button("hit", ParamIndex(16));
		ui_interface.add_horizontal_slider("tune", ParamIndex(17), 5e+01, 1.0, 2e+04, 0.01);
		ui_interface.add_horizontal_slider("vel", ParamIndex(18), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			16 => Some(self.fButton0),
			5 => Some(self.fEntry0),
			2 => Some(self.fEntry1),
			0 => Some(self.fHslider0),
			1 => Some(self.fHslider1),
			9 => Some(self.fHslider10),
			6 => Some(self.fHslider11),
			7 => Some(self.fHslider12),
			3 => Some(self.fHslider13),
			4 => Some(self.fHslider14),
			17 => Some(self.fHslider15),
			14 => Some(self.fHslider2),
			12 => Some(self.fHslider3),
			13 => Some(self.fHslider4),
			18 => Some(self.fHslider5),
			15 => Some(self.fHslider6),
			10 => Some(self.fHslider7),
			11 => Some(self.fHslider8),
			8 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			16 => { self.fButton0 = value }
			5 => { self.fEntry0 = value }
			2 => { self.fEntry1 = value }
			0 => { self.fHslider0 = value }
			1 => { self.fHslider1 = value }
			9 => { self.fHslider10 = value }
			6 => { self.fHslider11 = value }
			7 => { self.fHslider12 = value }
			3 => { self.fHslider13 = value }
			4 => { self.fHslider14 = value }
			17 => { self.fHslider15 = value }
			14 => { self.fHslider2 = value }
			12 => { self.fHslider3 = value }
			13 => { self.fHslider4 = value }
			18 => { self.fHslider5 = value }
			15 => { self.fHslider6 = value }
			10 => { self.fHslider7 = value }
			11 => { self.fHslider8 = value }
			8 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0) = if let [inputs0, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			(inputs0)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fConst1 * self.fHslider0;
		let mut fSlow1: F32 = self.fConst1 * self.fHslider1;
		let mut fSlow2: F32 = self.fConst3 * self.fHslider2;
		let mut fSlow3: F32 = self.fConst3 * self.fHslider3;
		let mut fSlow4: F32 = self.fButton0;
		let mut fSlow5: F32 = self.fConst1 * self.fHslider4;
		let mut fSlow6: F32 = self.fHslider5;
		let mut fSlow7: F32 = self.fConst1 * self.fHslider6;
		let mut fSlow8: F32 = self.fConst1 * self.fHslider7;
		let mut fSlow9: F32 = self.fConst1 * self.fHslider8;
		let mut fSlow10: F32 = self.fConst1 * self.fHslider9;
		let mut fSlow11: F32 = self.fConst1 * self.fHslider10;
		let mut fSlow12: F32 = self.fConst1 * self.fHslider11;
		let mut fSlow13: F32 = self.fConst1 * self.fHslider12;
		let mut fSlow14: F32 = self.fConst1 * self.fHslider13;
		let mut fSlow15: F32 = self.fConst1 * self.fHslider14;
		let mut fSlow16: F32 = self.fEntry0;
		let mut iSlow17: i32 = (fSlow16 == 1.0) as i32;
		let mut iSlow18: i32 = (fSlow16 >= 2.0) as i32;
		let mut iSlow19: i32 = (fSlow16 >= 1.0) as i32;
		let mut iSlow20: i32 = if iSlow18 != 0 {0} else {if iSlow19 != 0 {0} else {6}};
		let mut fSlow21: F32 = F32::sqrt(F32::powf(1e+01, 0.05 * (-iSlow20) as F32));
		let mut fSlow22: F32 = 942.4778 / fSlow21;
		let mut fSlow23: F32 = self.fConst7 + fSlow22;
		let mut fSlow24: F32 = F32::powf(1e+01, 0.05 * (if iSlow18 != 0 {6} else {0}) as F32);
		let mut fSlow25: F32 = 942.4778 * fSlow21;
		let mut fSlow26: F32 = self.fConst7 + fSlow25;
		let mut fSlow27: F32 = F32::sqrt(fSlow24);
		let mut fSlow28: F32 = 18849.557 / fSlow27;
		let mut fSlow29: F32 = self.fConst7 + fSlow28;
		let mut fSlow30: F32 = fSlow28 - self.fConst7;
		let mut iSlow31: i32 = if iSlow18 != 0 {15} else {if iSlow19 != 0 {12} else {9}};
		let mut fSlow32: F32 = (i32::wrapping_sub(if iSlow18 != 0 {40} else {if iSlow19 != 0 {30} else {24}}, iSlow31)) as F32;
		let mut fSlow33: F32 = (iSlow31) as F32;
		let mut fSlow34: F32 = ((fSlow16 == 0.0) as i32) as u32 as F32;
		let mut fSlow35: F32 = 0.12 * fSlow34;
		let mut fSlow36: F32 = 18849.557 * fSlow27;
		let mut fSlow37: F32 = self.fConst7 + fSlow36;
		let mut fSlow38: F32 = F32::sqrt(F32::powf(1e+01, 0.05 * (iSlow20) as F32));
		let mut fSlow39: F32 = 942.4778 / fSlow38;
		let mut fSlow40: F32 = self.fConst7 + fSlow39;
		let mut fSlow41: F32 = 942.4778 * fSlow38;
		let mut fSlow42: F32 = fSlow41 - self.fConst7;
		let mut fSlow43: F32 = self.fConst7 + fSlow41;
		let mut fSlow44: F32 = fSlow39 - self.fConst7;
		let mut fSlow45: F32 = fSlow36 - self.fConst7;
		let mut fSlow46: F32 = fSlow25 - self.fConst7;
		let mut fSlow47: F32 = fSlow22 - self.fConst7;
		let mut fSlow48: F32 = self.fHslider15;
		let mut fSlow49: F32 = 1.07 * fSlow48;
		let mut fSlow50: F32 = F32::max(2e+01, F32::min(self.fConst15, fSlow48));
		let mut fSlow51: F32 = F32::atan(0.00011111111 * fSlow50);
		let mut fSlow52: F32 = self.fConst16 * fSlow50;
		let mut fSlow53: F32 = self.fConst17 * fSlow50;
		let mut fSlow54: F32 = F32::max(-0.9999, F32::min(-0.0001, -((1.0 - fSlow53) / (fSlow53 + 1.0))));
		let mut fSlow55: F32 = 4.0 * (fSlow52 - 2.0 * F32::atan2(fSlow54 * F32::sin(fSlow52), fSlow54 * F32::cos(fSlow52) + 1.0));
		let mut iSlow56: i32 = (F32::max(1.0, F32::min(4094.0, self.fConst18 * ((6.2831855 - (fSlow55 + fSlow51)) / fSlow50) + -1.0))) as i32;
		let mut iSlow57: i32 = i32::wrapping_add(std::cmp::min(4097, std::cmp::max(0, i32::wrapping_add(iSlow56, 1))), 1);
		let mut fSlow58: F32 = F32::max(1.0, F32::min(4094.0, self.fConst18 * ((6.2831855 - fSlow55 - fSlow51) / fSlow50) + -1.0));
		let mut fSlow59: F32 = F32::floor(fSlow58);
		let mut fSlow60: F32 = fSlow58 - fSlow59;
		let mut iSlow61: i32 = i32::wrapping_add(std::cmp::min(4097, std::cmp::max(0, iSlow56)), 1);
		let mut fSlow62: F32 = fSlow59 + (1.0 - fSlow58);
		let mut iSlow63: i32 = (self.fEntry1) as i32;
		let mut iSlow64: i32 = (iSlow63 >= 3) as i32;
		let mut iSlow65: i32 = (iSlow63 >= 2) as i32;
		let mut iSlow66: i32 = (iSlow63 >= 1) as i32;
		let mut iSlow67: i32 = (iSlow63 >= 4) as i32;
		let mut fSlow68: F32 = F32::exp(-(self.fConst22 / (if iSlow64 != 0 {if iSlow67 != 0 {400} else {200}} else {if iSlow65 != 0 {120} else {if iSlow66 != 0 {80} else {50}}}) as F32));
		let mut fSlow69: F32 = F32::exp(-(self.fConst22 / if iSlow64 != 0 {(if iSlow67 != 0 {30} else {10}) as F32} else {if iSlow65 != 0 {3.0} else {if iSlow66 != 0 {1.0} else {0.3}}}));
		let zipped_iterators = inputs0.zip(outputs0);
		for (input0, output0) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			let mut iTemp0: i32 = (self.fRec0[0] != 0.0) as i32;
			self.fRec1[0] = fSlow1 + self.fConst2 * self.fRec1[1];
			self.fRec2[0] = fSlow2 + self.fConst2 * self.fRec2[1];
			self.fRec3[0] = fSlow3 + self.fConst2 * self.fRec3[1];
			self.fVec1[0] = fSlow4;
			let mut iTemp1: i32 = (fSlow4 > self.fVec1[1]) as i32;
			self.iRec4[0] = i32::wrapping_mul(i32::wrapping_sub(1, iTemp1), i32::wrapping_add(self.iRec4[1], 1));
			let mut fTemp2: F32 = (self.iRec4[0]) as F32;
			self.fRec5[0] = fSlow5 + self.fConst2 * self.fRec5[1];
			let mut fTemp3: F32 = F32::powf(1.0 - F32::max(0.0, F32::min(1.0, (self.fConst4 * fTemp2 - self.fRec3[0]) / F32::max(0.0001, self.fRec2[0]))), 6.0 * self.fRec5[0] + 1.0);
			let mut fTemp4: F32 = F32::min(1.0, self.fConst4 * (fTemp2 / F32::max(1e-05, self.fRec3[0])));
			self.fRec6[0] = if iTemp1 != 0 {fSlow6} else {self.fRec6[1]};
			self.fRec7[0] = fSlow7 + self.fConst2 * self.fRec7[1];
			let mut fTemp5: F32 = 1.0 - self.fRec7[0] * (1.0 - self.fRec6[0]);
			self.iRec8[0] = std::cmp::max(self.iRec8[1], iTemp1);
			let mut fTemp6: F32 = (self.iRec8[0]) as F32;
			let mut fTemp7: F32 = 1.0 - F32::max(0.0, F32::min(1.0, 0.025 * (self.fRec1[0] + 4e+01) + self.fRec0[0] * fTemp6 * fTemp5 * fTemp4 * fTemp3));
			self.fRec10[0] = fSlow8 + self.fConst2 * self.fRec10[1];
			self.fRec11[0] = fSlow9 + self.fConst2 * self.fRec11[1];
			let mut fTemp8: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec10[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec11[0] + self.fRec10[0] * fTemp6 * fTemp5 * fTemp4 * fTemp3))} else {self.fRec11[0]}));
			let mut iTemp9: i32 = (fTemp8 > 0.0) as i32;
			self.fRec12[0] = fSlow10 + self.fConst2 * self.fRec12[1];
			self.fRec13[0] = fSlow11 + self.fConst2 * self.fRec13[1];
			let mut fTemp10: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec12[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec13[0] + self.fRec12[0] * fTemp6 * fTemp5 * fTemp4 * fTemp3))} else {self.fRec13[0]}));
			let mut iTemp11: i32 = (fTemp10 > 0.0) as i32;
			self.fRec14[0] = fSlow12 + self.fConst2 * self.fRec14[1];
			self.fRec15[0] = fSlow13 + self.fConst2 * self.fRec15[1];
			let mut fTemp12: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec14[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec15[0] + self.fRec14[0] * fTemp6 * fTemp5 * fTemp4 * fTemp3))} else {self.fRec15[0]}));
			self.fRec16[0] = fSlow14 + self.fConst2 * self.fRec16[1];
			self.fRec17[0] = fSlow15 + self.fConst2 * self.fRec17[1];
			let mut fTemp13: F32 = F32::max(0.0, F32::min(1.0, if (self.fRec16[0] != 0.0) as i32 != 0 {F32::max(0.0, F32::min(1.0, self.fRec17[0] + self.fRec16[0] * fTemp6 * fTemp5 * fTemp4 * fTemp3))} else {self.fRec17[0]}));
			let mut fTemp14: F32 = *input0;
			let mut fTemp15: F32 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * F32::max(0.0, F32::min(1.0, 1.6666666 * (fTemp13 + -0.4))))) + -1.0));
			let mut fTemp16: F32 = self.fConst0 * F32::powf(self.fConst6, F32::max(0.0, F32::min(1.0, 1.6666666 * fTemp13)));
			let mut fTemp17: F32 = if i32::wrapping_sub(1, self.iVec0[1]) != 0 {0.0} else {self.fRec19[1] + self.fConst4 * F32::max(fTemp16, 0.0)};
			self.fRec19[0] = fTemp17 - F32::floor(fTemp17);
			self.fRec18[0] = if ((self.fRec19[0] < self.fRec19[1]) as i32) | ((fTemp16 >= self.fConst5) as i32) != 0 {fTemp14} else {self.fRec18[1]};
			let mut fTemp18: F32 = if (fTemp13 > 0.0001) as i32 != 0 {F32::floor(self.fRec18[0] * fTemp15 + 0.5) / fTemp15} else {fTemp14};
			self.fVec2[0] = fTemp18;
			let mut fTemp19: F32 = fTemp12 + -0.33333334;
			let mut fTemp20: F32 = F32::max(0.0, F32::min(1.0, 1.5 * fTemp19));
			let mut fTemp21: F32 = F32::powf(1e+01, 0.05 * (fSlow33 + fSlow32 * F32::powf(fTemp20, 1.5)));
			let mut fTemp22: F32 = 0.8 * fTemp21;
			let mut fTemp23: F32 = F32::tanh(fTemp22);
			let mut fTemp24: F32 = if iSlow18 != 0 {F32::max(-1.0, F32::min(1.0, fTemp22))} else {if iSlow19 != 0 {fTemp23 * ((fTemp22 < 0.0) as i32) as u32 as F32 + (i32::wrapping_mul(2, (fTemp22 >= 0.0) as i32)) as F32 * F32::tanh(0.4 * fTemp21)} else {fTemp23}};
			let mut fTemp25: F32 = F32::powf(1e+01, -(fSlow35 * fTemp20));
			self.fRec24[0] = -((self.fRec24[1] * fSlow44 - (fTemp18 * fSlow43 + self.fVec2[1] * fSlow42)) / fSlow40);
			let mut fTemp26: F32 = self.fRec24[0] * fSlow24;
			self.fVec3[0] = fTemp26;
			self.fRec23[0] = -((self.fRec23[1] * fSlow45 - (fTemp26 * fSlow29 + fSlow30 * self.fVec3[1])) / fSlow37);
			let mut fTemp27: F32 = self.fRec23[0] * fTemp21;
			self.fVec4[0] = fTemp27;
			let mut fTemp28: F32 = fTemp27 + self.fVec4[1];
			let mut fTemp29: F32 = 0.5 * fTemp28;
			let mut fTemp30: F32 = F32::tanh(fTemp29);
			let mut fTemp31: F32 = fTemp27 - self.fVec4[1];
			let mut iTemp32: i32 = (F32::abs(fTemp31) > 0.0001) as i32;
			let mut fTemp33: F32 = (i32::wrapping_sub(1, iTemp32)) as F32;
			let mut fTemp34: F32 = F32::abs(self.fVec4[1]);
			let mut fTemp35: F32 = fTemp34 + F32::log(F32::exp(-(2.0 * fTemp34)) + 1.0, std::f32::consts::E);
			let mut fTemp36: F32 = F32::abs(fTemp27);
			let mut fTemp37: F32 = fTemp36 + F32::log(F32::exp(-(2.0 * fTemp36)) + 1.0, std::f32::consts::E);
			let mut fTemp38: F32 = fTemp27 * self.fVec4[1];
			let mut fTemp39: F32 = F32::max(fTemp36, fTemp34);
			let mut fTemp40: F32 = F32::exp(-(2.0 * fTemp39)) + 1.0;
			let mut fTemp41: F32 = F32::min(fTemp36, fTemp34);
			let mut fTemp42: F32 = fTemp41 - fTemp39;
			let mut fTemp43: F32 = 2.0 * fTemp42;
			let mut fTemp44: F32 = F32::abs(fTemp43);
			let mut fTemp45: F32 = F32::exp(-(2.0 * fTemp41)) * (2.0 * fTemp42 * ((fTemp44 < 0.001) as i32) as u32 as F32 * (fTemp42 * (0.6666667 * fTemp42 + 1.0) + 1.0) + ((fTemp44 >= 0.001) as i32) as u32 as F32 * (F32::exp(F32::max(fTemp43, -8e+01)) + -1.0));
			let mut fTemp46: F32 = fTemp45 / fTemp40;
			let mut fTemp47: F32 = F32::abs(-fTemp46);
			let mut fTemp48: F32 = fTemp36 + (i32::wrapping_add(i32::wrapping_mul(2, (fTemp36 <= fTemp34) as i32), -1)) as F32 * (((fTemp47 >= 0.001) as i32) as u32 as F32 * F32::log(1.0 - fTemp46, std::f32::consts::E) - fTemp45 * ((fTemp47 < 0.001) as i32) as u32 as F32 * (fTemp45 * (0.33333334 * fTemp46 + 0.5) / fTemp40 + 1.0) / fTemp40) - fTemp34;
			let mut fTemp49: F32 = F32::abs(0.5 * self.fVec4[1]);
			let mut iTemp50: i32 = (self.fVec4[1] >= 0.0) as i32;
			let mut iTemp51: i32 = (self.fVec4[1] < 0.0) as i32;
			let mut fTemp52: F32 = F32::abs(0.5 * fTemp27);
			let mut iTemp53: i32 = (fTemp27 >= 0.0) as i32;
			let mut iTemp54: i32 = (fTemp27 < 0.0) as i32;
			let mut iTemp55: i32 = i32::wrapping_mul(iTemp53, iTemp50);
			let mut iTemp56: i32 = i32::wrapping_mul(iTemp54, iTemp51);
			let mut fTemp57: F32 = F32::max(fTemp52, fTemp49);
			let mut fTemp58: F32 = F32::exp(-(2.0 * fTemp57)) + 1.0;
			let mut fTemp59: F32 = F32::min(fTemp52, fTemp49);
			let mut fTemp60: F32 = fTemp59 - fTemp57;
			let mut fTemp61: F32 = 2.0 * fTemp60;
			let mut fTemp62: F32 = F32::abs(fTemp61);
			let mut fTemp63: F32 = F32::exp(-(2.0 * fTemp59)) * (2.0 * fTemp60 * ((fTemp62 < 0.001) as i32) as u32 as F32 * (fTemp60 * (0.6666667 * fTemp60 + 1.0) + 1.0) + ((fTemp62 >= 0.001) as i32) as u32 as F32 * (F32::exp(F32::max(fTemp61, -8e+01)) + -1.0));
			let mut fTemp64: F32 = fTemp63 / fTemp58;
			let mut fTemp65: F32 = F32::abs(-fTemp64);
			let mut iTemp66: i32 = (fTemp34 <= 1.0) as i32;
			let mut iTemp67: i32 = (fTemp36 <= 1.0) as i32;
			let mut iTemp68: i32 = i32::wrapping_mul(iTemp67, iTemp66);
			let mut fTemp69: F32 = (iTemp32) as F32 * if iSlow18 != 0 {0.5 * fTemp31 * (iTemp68) as F32 * fTemp28 + (i32::wrapping_sub(1, iTemp68)) as F32 * (((fTemp36 > 1.0) as i32) as u32 as F32 * (fTemp36 + -0.5) + 0.5 * (FxDsp_faustpower2_f(self.fRec23[0]) * FxDsp_faustpower2_f(fTemp21) * (iTemp67) as F32 - FxDsp_faustpower2_f(self.fVec4[1]) * (iTemp66) as F32) - ((fTemp34 > 1.0) as i32) as u32 as F32 * (fTemp34 + -0.5))} else {if iSlow19 != 0 {fTemp48 * (iTemp56) as F32 + (i32::wrapping_mul(4, iTemp55)) as F32 * (fTemp52 + (i32::wrapping_add(i32::wrapping_mul(2, (fTemp52 <= fTemp49) as i32), -1)) as F32 * (((fTemp65 >= 0.001) as i32) as u32 as F32 * F32::log(1.0 - fTemp64, std::f32::consts::E) - fTemp63 * ((fTemp65 < 0.001) as i32) as u32 as F32 * (fTemp63 * (0.33333334 * fTemp64 + 0.5) / fTemp58 + 1.0) / fTemp58) - fTemp49) + (i32::wrapping_sub(1, i32::wrapping_add(iTemp56, iTemp55))) as F32 * ((fTemp37 + -0.6931472) * (iTemp54) as F32 + (i32::wrapping_mul(4, iTemp53)) as F32 * (fTemp52 + F32::log(F32::exp(-(2.0 * fTemp52)) + 1.0, std::f32::consts::E) + -0.6931472) - ((fTemp35 + -0.6931472) * (iTemp51) as F32 + (i32::wrapping_mul(4, iTemp50)) as F32 * (fTemp49 + F32::log(F32::exp(-(2.0 * fTemp49)) + 1.0, std::f32::consts::E) + -0.6931472)))} else {((fTemp38 >= 0.0) as i32) as u32 as F32 * fTemp48 + ((fTemp38 < 0.0) as i32) as u32 as F32 * (fTemp37 - fTemp35)}} / (fTemp27 + fTemp33 - self.fVec4[1]) + fTemp33 * if iSlow18 != 0 {F32::max(-1.0, F32::min(1.0, fTemp29))} else {if iSlow19 != 0 {fTemp30 * ((fTemp29 < 0.0) as i32) as u32 as F32 + (i32::wrapping_mul(2, (fTemp29 >= 0.0) as i32)) as F32 * F32::tanh(0.25 * fTemp28)} else {fTemp30}};
			let mut fTemp70: F32 = if (F32::abs(fTemp69) > 1e+06) as i32 != 0 {0.0} else {fTemp69};
			self.fVec5[0] = fTemp70 * fTemp25 / fTemp24;
			self.fRec22[0] = (0.8 * (fSlow45 * self.fVec5[1] + fSlow37 * fTemp70 * fTemp25 / fTemp24) - self.fRec22[1] * fSlow30) / fSlow29;
			self.fVec6[0] = self.fRec22[0] / fSlow24;
			self.fRec21[0] = -((self.fRec21[1] * fSlow47 - (self.fVec6[1] * fSlow46 + self.fRec22[0] * fSlow26 / fSlow24)) / fSlow23);
			self.fRec20[0] = -(self.fConst10 * (self.fConst9 * self.fRec20[1] - (self.fRec21[0] + self.fRec21[1])));
			let mut fTemp71: F32 = self.fRec21[0] * (1.0 - fSlow34 * fTemp20) + fSlow34 * self.fRec20[0] * fTemp20;
			self.fVec7[0] = fTemp71;
			self.fRec25[0] = -(self.fConst13 * (self.fConst12 * self.fRec25[1] - self.fConst11 * (fTemp71 - self.fVec7[1])));
			let mut fTemp72: F32 = if iSlow17 != 0 {self.fRec25[0]} else {fTemp71};
			let mut fTemp73: F32 = F32::max(0.0, F32::min(1.0, 3.0 * fTemp12)) + F32::max(0.0, F32::min(1.0, 6.0 * fTemp19));
			let mut fTemp74: F32 = if (fTemp12 > 0.0) as i32 != 0 {fTemp18 * (1.0 - 0.5 * fTemp73) + 0.5 * fTemp73 * if (F32::abs(fTemp72) > 1e+06) as i32 != 0 {0.0} else {fTemp72}} else {fTemp18};
			let mut fTemp75: F32 = F32::powf(fTemp10, 0.7);
			let mut fTemp76: F32 = self.fRec26[1] + self.fConst4 * F32::max(1.0, F32::min(self.fConst14, fSlow49 * F32::powf(3.457944, fTemp10)));
			self.fRec26[0] = if iTemp1 != 0 {0.0} else {fTemp76 - F32::floor(fTemp76)};
			let mut fTemp77: F32 = F32::sin(6.2831855 * self.fRec26[0]);
			let mut fTemp78: F32 = if iTemp11 != 0 {fTemp74 * (1.0 - fTemp75 + fTemp77 * fTemp75)} else {fTemp74};
			let mut fTemp79: F32 = F32::sqrt(fTemp8);
			let mut fTemp80: F32 = fSlow62 * self.fRec27[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 8191) as usize] + fSlow60 * self.fRec27[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 8191) as usize];
			self.fVec8[0] = fTemp80;
			self.fRec32[0] = self.fVec8[1] - fSlow54 * (self.fRec32[1] - fTemp80);
			self.fRec31[0] = self.fRec32[1] - fSlow54 * (self.fRec31[1] - self.fRec32[0]);
			self.fRec30[0] = self.fRec31[1] - fSlow54 * (self.fRec30[1] - self.fRec31[0]);
			self.fRec29[0] = self.fRec30[1] - fSlow54 * (self.fRec29[1] - self.fRec30[0]);
			self.fRec28[0] = -(self.fConst21 * (self.fConst20 * self.fRec28[1] - (self.fRec29[0] + self.fRec29[1])));
			self.fRec27[(self.IOTA0 & 8191) as usize] = 0.96 * self.fRec28[0] * fTemp79 + 2.0 * fTemp78 * (1.0 - 0.96 * fTemp79);
			let mut fTemp81: F32 = self.fRec27[(self.IOTA0 & 8191) as usize];
			let mut fTemp82: F32 = fTemp8 * if (F32::abs(fTemp81) > 1e+06) as i32 != 0 {0.0} else {fTemp81};
			let mut fTemp83: F32 = 1.0 - fTemp8;
			let mut fTemp84: F32 = F32::max(0.0, 2e+01 * F32::log10(F32::max(1.1754944e-38, F32::abs(if iTemp9 != 0 {fTemp78 * fTemp83 + fTemp82} else {fTemp78}) + 1e-09)) - if iTemp0 != 0 {-(4e+01 * fTemp7)} else {self.fRec1[0]});
			let mut fTemp85: F32 = if ((0.75 * fTemp84) > self.fRec9[1]) as i32 != 0 {fSlow69} else {fSlow68};
			self.fRec9[0] = self.fRec9[1] * fTemp85 + 0.75 * fTemp84 * (1.0 - fTemp85);
			let mut fTemp86: F32 = if iTemp11 != 0 {fTemp74 * (1.0 - fTemp75 * (1.0 - fTemp77))} else {fTemp74};
			*output0 = if iTemp9 != 0 {fTemp82 + fTemp83 * fTemp86} else {fTemp86} * F32::powf(1e+01, -0.05 * (self.fRec9[0] + 0.375 * if iTemp0 != 0 {-4e+01 * fTemp7} else {self.fRec1[0]}));
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec3[1] = self.fRec3[0];
			self.fVec1[1] = self.fVec1[0];
			self.iRec4[1] = self.iRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.iRec8[1] = self.iRec8[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec18[1] = self.fRec18[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec24[1] = self.fRec24[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec23[1] = self.fRec23[0];
			self.fVec4[1] = self.fVec4[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec22[1] = self.fRec22[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec20[1] = self.fRec20[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fVec8[1] = self.fVec8[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec9[1] = self.fRec9[0];
		}
	}

}

