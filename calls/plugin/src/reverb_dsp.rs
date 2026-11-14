/* ------------------------------------------------------------
name: "plugin-reverb"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn ReverbDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn ReverbDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct ReverbDsp {
	IOTA0: i32,
	fVec0: [F32;16384],
	iVec1: [i32;2],
	fHslider0: F32,
	fSampleRate: i32,
	fConst3: F32,
	fHslider1: F32,
	fConst4: F32,
	fConst6: F32,
	fConst7: F32,
	fRec11: [F32;2],
	fConst8: F32,
	fRec10: [F32;2],
	fVec2: [F32;32768],
	iConst10: i32,
	fConst11: F32,
	fHslider2: F32,
	fConst12: F32,
	fVec3: [F32;8192],
	iConst13: i32,
	fRec8: [F32;2],
	fConst16: F32,
	fRec15: [F32;2],
	fConst17: F32,
	fRec14: [F32;2],
	fVec4: [F32;32768],
	iConst19: i32,
	fVec5: [F32;4096],
	iConst20: i32,
	fRec12: [F32;2],
	fConst23: F32,
	fRec19: [F32;2],
	fConst24: F32,
	fRec18: [F32;2],
	fVec6: [F32;32768],
	iConst26: i32,
	fVec7: [F32;8192],
	iConst27: i32,
	fRec16: [F32;2],
	fConst30: F32,
	fRec23: [F32;2],
	fConst31: F32,
	fRec22: [F32;2],
	fVec8: [F32;32768],
	iConst33: i32,
	fVec9: [F32;4096],
	iConst34: i32,
	fRec20: [F32;2],
	fConst37: F32,
	fRec27: [F32;2],
	fConst38: F32,
	fRec26: [F32;2],
	fVec10: [F32;65536],
	iConst40: i32,
	fVec11: [F32;16384],
	fVec12: [F32;8192],
	iConst41: i32,
	fRec24: [F32;2],
	fConst44: F32,
	fRec31: [F32;2],
	fConst45: F32,
	fRec30: [F32;2],
	fVec13: [F32;32768],
	iConst47: i32,
	fVec14: [F32;8192],
	iConst48: i32,
	fRec28: [F32;2],
	fConst51: F32,
	fRec35: [F32;2],
	fConst52: F32,
	fRec34: [F32;2],
	fVec15: [F32;65536],
	iConst54: i32,
	fVec16: [F32;8192],
	iConst55: i32,
	fRec32: [F32;2],
	fConst58: F32,
	fRec39: [F32;2],
	fConst59: F32,
	fRec38: [F32;2],
	fVec17: [F32;65536],
	iConst61: i32,
	fVec18: [F32;4096],
	iConst62: i32,
	fRec36: [F32;2],
	fRec0: [F32;3],
	fRec1: [F32;3],
	fRec2: [F32;3],
	fRec3: [F32;3],
	fRec4: [F32;3],
	fRec5: [F32;3],
	fRec6: [F32;3],
	fRec7: [F32;3],
	fHslider3: F32,
	fRec40: [F32;2],
}

impl FaustDsp for ReverbDsp {
	type T = F32;
		
	fn new() -> ReverbDsp { 
		ReverbDsp {
			IOTA0: 0,
			fVec0: [0.0;16384],
			iVec1: [0;2],
			fHslider0: 0.0,
			fSampleRate: 0,
			fConst3: 0.0,
			fHslider1: 0.0,
			fConst4: 0.0,
			fConst6: 0.0,
			fConst7: 0.0,
			fRec11: [0.0;2],
			fConst8: 0.0,
			fRec10: [0.0;2],
			fVec2: [0.0;32768],
			iConst10: 0,
			fConst11: 0.0,
			fHslider2: 0.0,
			fConst12: 0.0,
			fVec3: [0.0;8192],
			iConst13: 0,
			fRec8: [0.0;2],
			fConst16: 0.0,
			fRec15: [0.0;2],
			fConst17: 0.0,
			fRec14: [0.0;2],
			fVec4: [0.0;32768],
			iConst19: 0,
			fVec5: [0.0;4096],
			iConst20: 0,
			fRec12: [0.0;2],
			fConst23: 0.0,
			fRec19: [0.0;2],
			fConst24: 0.0,
			fRec18: [0.0;2],
			fVec6: [0.0;32768],
			iConst26: 0,
			fVec7: [0.0;8192],
			iConst27: 0,
			fRec16: [0.0;2],
			fConst30: 0.0,
			fRec23: [0.0;2],
			fConst31: 0.0,
			fRec22: [0.0;2],
			fVec8: [0.0;32768],
			iConst33: 0,
			fVec9: [0.0;4096],
			iConst34: 0,
			fRec20: [0.0;2],
			fConst37: 0.0,
			fRec27: [0.0;2],
			fConst38: 0.0,
			fRec26: [0.0;2],
			fVec10: [0.0;65536],
			iConst40: 0,
			fVec11: [0.0;16384],
			fVec12: [0.0;8192],
			iConst41: 0,
			fRec24: [0.0;2],
			fConst44: 0.0,
			fRec31: [0.0;2],
			fConst45: 0.0,
			fRec30: [0.0;2],
			fVec13: [0.0;32768],
			iConst47: 0,
			fVec14: [0.0;8192],
			iConst48: 0,
			fRec28: [0.0;2],
			fConst51: 0.0,
			fRec35: [0.0;2],
			fConst52: 0.0,
			fRec34: [0.0;2],
			fVec15: [0.0;65536],
			iConst54: 0,
			fVec16: [0.0;8192],
			iConst55: 0,
			fRec32: [0.0;2],
			fConst58: 0.0,
			fRec39: [0.0;2],
			fConst59: 0.0,
			fRec38: [0.0;2],
			fVec17: [0.0;65536],
			iConst61: 0,
			fVec18: [0.0;4096],
			iConst62: 0,
			fRec36: [0.0;2],
			fRec0: [0.0;3],
			fRec1: [0.0;3],
			fRec2: [0.0;3],
			fRec3: [0.0;3],
			fRec4: [0.0;3],
			fRec5: [0.0;3],
			fRec6: [0.0;3],
			fRec7: [0.0;3],
			fHslider3: 0.0,
			fRec40: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn ReverbDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("filename", r"plugin-reverb.dsp");
		m.declare("filters.lib/allpass_comb:author", r"Julius O. Smith III");
		m.declare("filters.lib/allpass_comb:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/allpass_comb:license", r"MIT-style STK-4.3 license");
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
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"plugin-reverb");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("reverbs.lib/name", r"Faust Reverb Library");
		m.declare("reverbs.lib/version", r"1.2.1");
		m.declare("routes.lib/hadamard:author", r"Remy Muller, revised by Romain Michon");
		m.declare("routes.lib/name", r"Faust Signal Routing Library");
		m.declare("routes.lib/version", r"1.2.0");
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
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 5.0;
		self.fHslider1 = 2.5e+03;
		self.fHslider2 = 3e+01;
		self.fHslider3 = 1.0;
	}
	fn instance_clear(&mut self) {
		self.IOTA0 = 0;
		for l0 in 0..16384 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.iVec1[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec11[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec10[l3 as usize] = 0.0;
		}
		for l4 in 0..32768 {
			self.fVec2[l4 as usize] = 0.0;
		}
		for l5 in 0..8192 {
			self.fVec3[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec8[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec15[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec14[l8 as usize] = 0.0;
		}
		for l9 in 0..32768 {
			self.fVec4[l9 as usize] = 0.0;
		}
		for l10 in 0..4096 {
			self.fVec5[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec12[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec19[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec18[l13 as usize] = 0.0;
		}
		for l14 in 0..32768 {
			self.fVec6[l14 as usize] = 0.0;
		}
		for l15 in 0..8192 {
			self.fVec7[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec16[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec23[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec22[l18 as usize] = 0.0;
		}
		for l19 in 0..32768 {
			self.fVec8[l19 as usize] = 0.0;
		}
		for l20 in 0..4096 {
			self.fVec9[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec20[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec27[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec26[l23 as usize] = 0.0;
		}
		for l24 in 0..65536 {
			self.fVec10[l24 as usize] = 0.0;
		}
		for l25 in 0..16384 {
			self.fVec11[l25 as usize] = 0.0;
		}
		for l26 in 0..8192 {
			self.fVec12[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec24[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec31[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec30[l29 as usize] = 0.0;
		}
		for l30 in 0..32768 {
			self.fVec13[l30 as usize] = 0.0;
		}
		for l31 in 0..8192 {
			self.fVec14[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec28[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec35[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec34[l34 as usize] = 0.0;
		}
		for l35 in 0..65536 {
			self.fVec15[l35 as usize] = 0.0;
		}
		for l36 in 0..8192 {
			self.fVec16[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec32[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec39[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec38[l39 as usize] = 0.0;
		}
		for l40 in 0..65536 {
			self.fVec17[l40 as usize] = 0.0;
		}
		for l41 in 0..4096 {
			self.fVec18[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec36[l42 as usize] = 0.0;
		}
		for l43 in 0..3 {
			self.fRec0[l43 as usize] = 0.0;
		}
		for l44 in 0..3 {
			self.fRec1[l44 as usize] = 0.0;
		}
		for l45 in 0..3 {
			self.fRec2[l45 as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fRec3[l46 as usize] = 0.0;
		}
		for l47 in 0..3 {
			self.fRec4[l47 as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fRec5[l48 as usize] = 0.0;
		}
		for l49 in 0..3 {
			self.fRec6[l49 as usize] = 0.0;
		}
		for l50 in 0..3 {
			self.fRec7[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec40[l51 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		let mut fConst1: F32 = F32::floor(0.174713 * fConst0 + 0.5);
		let mut fConst2: F32 = fConst1 / fConst0;
		self.fConst3 = 6.9077554 * fConst2;
		self.fConst4 = 6.2831855 / fConst0;
		let mut fConst5: F32 = 1.0 / F32::tan(942.4778 / fConst0);
		self.fConst6 = 1.0 - fConst5;
		self.fConst7 = 1.0 / (fConst5 + 1.0);
		self.fConst8 = 8.634694 * fConst2;
		let mut fConst9: F32 = F32::floor(0.022904 * fConst0 + 0.5);
		self.iConst10 = (F32::min(16384.0, F32::max(0.0, fConst1 - fConst9))) as i32;
		self.fConst11 = 8e+06 / fConst0;
		self.fConst12 = 0.001 * fConst0;
		self.iConst13 = (F32::min(4096.0, F32::max(0.0, fConst9 + -1.0))) as i32;
		let mut fConst14: F32 = F32::floor(0.153129 * fConst0 + 0.5);
		let mut fConst15: F32 = fConst14 / fConst0;
		self.fConst16 = 6.9077554 * fConst15;
		self.fConst17 = 8.634694 * fConst15;
		let mut fConst18: F32 = F32::floor(0.020346 * fConst0 + 0.5);
		self.iConst19 = (F32::min(16384.0, F32::max(0.0, fConst14 - fConst18))) as i32;
		self.iConst20 = (F32::min(2048.0, F32::max(0.0, fConst18 + -1.0))) as i32;
		let mut fConst21: F32 = F32::floor(0.127837 * fConst0 + 0.5);
		let mut fConst22: F32 = fConst21 / fConst0;
		self.fConst23 = 6.9077554 * fConst22;
		self.fConst24 = 8.634694 * fConst22;
		let mut fConst25: F32 = F32::floor(0.031604 * fConst0 + 0.5);
		self.iConst26 = (F32::min(16384.0, F32::max(0.0, fConst21 - fConst25))) as i32;
		self.iConst27 = (F32::min(4096.0, F32::max(0.0, fConst25 + -1.0))) as i32;
		let mut fConst28: F32 = F32::floor(0.125 * fConst0 + 0.5);
		let mut fConst29: F32 = fConst28 / fConst0;
		self.fConst30 = 6.9077554 * fConst29;
		self.fConst31 = 8.634694 * fConst29;
		let mut fConst32: F32 = F32::floor(0.013458 * fConst0 + 0.5);
		self.iConst33 = (F32::min(16384.0, F32::max(0.0, fConst28 - fConst32))) as i32;
		self.iConst34 = (F32::min(2048.0, F32::max(0.0, fConst32 + -1.0))) as i32;
		let mut fConst35: F32 = F32::floor(0.210389 * fConst0 + 0.5);
		let mut fConst36: F32 = fConst35 / fConst0;
		self.fConst37 = 6.9077554 * fConst36;
		self.fConst38 = 8.634694 * fConst36;
		let mut fConst39: F32 = F32::floor(0.024421 * fConst0 + 0.5);
		self.iConst40 = (F32::min(32768.0, F32::max(0.0, fConst35 - fConst39))) as i32;
		self.iConst41 = (F32::min(4096.0, F32::max(0.0, fConst39 + -1.0))) as i32;
		let mut fConst42: F32 = F32::floor(0.192303 * fConst0 + 0.5);
		let mut fConst43: F32 = fConst42 / fConst0;
		self.fConst44 = 6.9077554 * fConst43;
		self.fConst45 = 8.634694 * fConst43;
		let mut fConst46: F32 = F32::floor(0.029291 * fConst0 + 0.5);
		self.iConst47 = (F32::min(16384.0, F32::max(0.0, fConst42 - fConst46))) as i32;
		self.iConst48 = (F32::min(4096.0, F32::max(0.0, fConst46 + -1.0))) as i32;
		let mut fConst49: F32 = F32::floor(0.256891 * fConst0 + 0.5);
		let mut fConst50: F32 = fConst49 / fConst0;
		self.fConst51 = 6.9077554 * fConst50;
		self.fConst52 = 8.634694 * fConst50;
		let mut fConst53: F32 = F32::floor(0.027333 * fConst0 + 0.5);
		self.iConst54 = (F32::min(32768.0, F32::max(0.0, fConst49 - fConst53))) as i32;
		self.iConst55 = (F32::min(4096.0, F32::max(0.0, fConst53 + -1.0))) as i32;
		let mut fConst56: F32 = F32::floor(0.219991 * fConst0 + 0.5);
		let mut fConst57: F32 = fConst56 / fConst0;
		self.fConst58 = 6.9077554 * fConst57;
		self.fConst59 = 8.634694 * fConst57;
		let mut fConst60: F32 = F32::floor(0.019123 * fConst0 + 0.5);
		self.iConst61 = (F32::min(32768.0, F32::max(0.0, fConst56 - fConst60))) as i32;
		self.iConst62 = (F32::min(2048.0, F32::max(0.0, fConst60 + -1.0))) as i32;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		ReverbDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin-reverb");
		ui_interface.add_horizontal_slider("damp", ParamIndex(0), 2.5e+03, 1e+03, 1.2e+04, 1.0);
		ui_interface.add_horizontal_slider("decay", ParamIndex(1), 5.0, 0.5, 2e+01, 0.01);
		ui_interface.add_horizontal_slider("mix", ParamIndex(2), 1.0, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("predelay", ParamIndex(3), 3e+01, 0.0, 1.5e+02, 0.1);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			1 => Some(self.fHslider0),
			0 => Some(self.fHslider1),
			3 => Some(self.fHslider2),
			2 => Some(self.fHslider3),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			1 => { self.fHslider0 = value }
			0 => { self.fHslider1 = value }
			3 => { self.fHslider2 = value }
			2 => { self.fHslider3 = value }
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
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fHslider0;
		let mut fSlow1: F32 = F32::exp(-(self.fConst3 / fSlow0));
		let mut fSlow2: F32 = ReverbDsp_faustpower2_f(fSlow1);
		let mut fSlow3: F32 = 1.0 - fSlow2;
		let mut fSlow4: F32 = F32::cos(self.fConst4 * self.fHslider1);
		let mut fSlow5: F32 = 1.0 - fSlow4 * fSlow2;
		let mut fSlow6: F32 = F32::sqrt(F32::max(0.0, ReverbDsp_faustpower2_f(fSlow5) / ReverbDsp_faustpower2_f(fSlow3) + -1.0));
		let mut fSlow7: F32 = fSlow5 / fSlow3;
		let mut fSlow8: F32 = fSlow7 - fSlow6;
		let mut fSlow9: F32 = F32::exp(-(self.fConst8 / fSlow0)) / fSlow1 + -1.0;
		let mut fSlow10: F32 = fSlow1 * (fSlow6 + (1.0 - fSlow7));
		let mut iSlow11: i32 = (F32::min(8192.0, F32::max(0.0, self.fConst12 * F32::min(self.fConst11, self.fHslider2)))) as i32;
		let mut fSlow12: F32 = F32::exp(-(self.fConst16 / fSlow0));
		let mut fSlow13: F32 = ReverbDsp_faustpower2_f(fSlow12);
		let mut fSlow14: F32 = 1.0 - fSlow13;
		let mut fSlow15: F32 = 1.0 - fSlow13 * fSlow4;
		let mut fSlow16: F32 = F32::sqrt(F32::max(0.0, ReverbDsp_faustpower2_f(fSlow15) / ReverbDsp_faustpower2_f(fSlow14) + -1.0));
		let mut fSlow17: F32 = fSlow15 / fSlow14;
		let mut fSlow18: F32 = fSlow17 - fSlow16;
		let mut fSlow19: F32 = F32::exp(-(self.fConst17 / fSlow0)) / fSlow12 + -1.0;
		let mut fSlow20: F32 = fSlow12 * (fSlow16 + (1.0 - fSlow17));
		let mut fSlow21: F32 = F32::exp(-(self.fConst23 / fSlow0));
		let mut fSlow22: F32 = ReverbDsp_faustpower2_f(fSlow21);
		let mut fSlow23: F32 = 1.0 - fSlow22;
		let mut fSlow24: F32 = 1.0 - fSlow4 * fSlow22;
		let mut fSlow25: F32 = F32::sqrt(F32::max(0.0, ReverbDsp_faustpower2_f(fSlow24) / ReverbDsp_faustpower2_f(fSlow23) + -1.0));
		let mut fSlow26: F32 = fSlow24 / fSlow23;
		let mut fSlow27: F32 = fSlow26 - fSlow25;
		let mut fSlow28: F32 = F32::exp(-(self.fConst24 / fSlow0)) / fSlow21 + -1.0;
		let mut fSlow29: F32 = fSlow21 * (fSlow25 + (1.0 - fSlow26));
		let mut fSlow30: F32 = F32::exp(-(self.fConst30 / fSlow0));
		let mut fSlow31: F32 = ReverbDsp_faustpower2_f(fSlow30);
		let mut fSlow32: F32 = 1.0 - fSlow31;
		let mut fSlow33: F32 = 1.0 - fSlow4 * fSlow31;
		let mut fSlow34: F32 = F32::sqrt(F32::max(0.0, ReverbDsp_faustpower2_f(fSlow33) / ReverbDsp_faustpower2_f(fSlow32) + -1.0));
		let mut fSlow35: F32 = fSlow33 / fSlow32;
		let mut fSlow36: F32 = fSlow35 - fSlow34;
		let mut fSlow37: F32 = F32::exp(-(self.fConst31 / fSlow0)) / fSlow30 + -1.0;
		let mut fSlow38: F32 = fSlow30 * (fSlow34 + (1.0 - fSlow35));
		let mut fSlow39: F32 = F32::exp(-(self.fConst37 / fSlow0));
		let mut fSlow40: F32 = ReverbDsp_faustpower2_f(fSlow39);
		let mut fSlow41: F32 = 1.0 - fSlow40;
		let mut fSlow42: F32 = 1.0 - fSlow4 * fSlow40;
		let mut fSlow43: F32 = F32::sqrt(F32::max(0.0, ReverbDsp_faustpower2_f(fSlow42) / ReverbDsp_faustpower2_f(fSlow41) + -1.0));
		let mut fSlow44: F32 = fSlow42 / fSlow41;
		let mut fSlow45: F32 = fSlow44 - fSlow43;
		let mut fSlow46: F32 = F32::exp(-(self.fConst38 / fSlow0)) / fSlow39 + -1.0;
		let mut fSlow47: F32 = fSlow39 * (fSlow43 + (1.0 - fSlow44));
		let mut fSlow48: F32 = F32::exp(-(self.fConst44 / fSlow0));
		let mut fSlow49: F32 = ReverbDsp_faustpower2_f(fSlow48);
		let mut fSlow50: F32 = 1.0 - fSlow49;
		let mut fSlow51: F32 = 1.0 - fSlow4 * fSlow49;
		let mut fSlow52: F32 = F32::sqrt(F32::max(0.0, ReverbDsp_faustpower2_f(fSlow51) / ReverbDsp_faustpower2_f(fSlow50) + -1.0));
		let mut fSlow53: F32 = fSlow51 / fSlow50;
		let mut fSlow54: F32 = fSlow53 - fSlow52;
		let mut fSlow55: F32 = F32::exp(-(self.fConst45 / fSlow0)) / fSlow48 + -1.0;
		let mut fSlow56: F32 = fSlow48 * (fSlow52 + (1.0 - fSlow53));
		let mut fSlow57: F32 = F32::exp(-(self.fConst51 / fSlow0));
		let mut fSlow58: F32 = ReverbDsp_faustpower2_f(fSlow57);
		let mut fSlow59: F32 = 1.0 - fSlow58;
		let mut fSlow60: F32 = 1.0 - fSlow4 * fSlow58;
		let mut fSlow61: F32 = F32::sqrt(F32::max(0.0, ReverbDsp_faustpower2_f(fSlow60) / ReverbDsp_faustpower2_f(fSlow59) + -1.0));
		let mut fSlow62: F32 = fSlow60 / fSlow59;
		let mut fSlow63: F32 = fSlow62 - fSlow61;
		let mut fSlow64: F32 = F32::exp(-(self.fConst52 / fSlow0)) / fSlow57 + -1.0;
		let mut fSlow65: F32 = fSlow57 * (fSlow61 + (1.0 - fSlow62));
		let mut fSlow66: F32 = F32::exp(-(self.fConst58 / fSlow0));
		let mut fSlow67: F32 = ReverbDsp_faustpower2_f(fSlow66);
		let mut fSlow68: F32 = 1.0 - fSlow67;
		let mut fSlow69: F32 = 1.0 - fSlow4 * fSlow67;
		let mut fSlow70: F32 = F32::sqrt(F32::max(0.0, ReverbDsp_faustpower2_f(fSlow69) / ReverbDsp_faustpower2_f(fSlow68) + -1.0));
		let mut fSlow71: F32 = fSlow69 / fSlow68;
		let mut fSlow72: F32 = fSlow71 - fSlow70;
		let mut fSlow73: F32 = F32::exp(-(self.fConst59 / fSlow0)) / fSlow66 + -1.0;
		let mut fSlow74: F32 = fSlow66 * (fSlow70 + (1.0 - fSlow71));
		let mut fSlow75: F32 = self.fHslider3;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut fTemp0: F32 = *input0;
			self.fVec0[(self.IOTA0 & 16383) as usize] = fTemp0;
			self.iVec1[0] = 1;
			self.fRec11[0] = -(self.fConst7 * (self.fConst6 * self.fRec11[1] - (self.fRec4[1] + self.fRec4[2])));
			self.fRec10[0] = fSlow10 * (self.fRec4[1] + fSlow9 * self.fRec11[0]) + fSlow8 * self.fRec10[1];
			self.fVec2[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec10[0] + 1e-20;
			let mut fTemp1: F32 = 0.3 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow11)) & 16383) as usize];
			let mut fTemp2: F32 = fTemp1 + self.fVec2[((i32::wrapping_sub(self.IOTA0, self.iConst10)) & 32767) as usize] - 0.6 * self.fRec8[1];
			self.fVec3[(self.IOTA0 & 8191) as usize] = fTemp2;
			self.fRec8[0] = self.fVec3[((i32::wrapping_sub(self.IOTA0, self.iConst13)) & 8191) as usize];
			let mut fRec9: F32 = 0.6 * fTemp2;
			self.fRec15[0] = -(self.fConst7 * (self.fConst6 * self.fRec15[1] - (self.fRec0[1] + self.fRec0[2])));
			self.fRec14[0] = fSlow20 * (self.fRec0[1] + fSlow19 * self.fRec15[0]) + fSlow18 * self.fRec14[1];
			self.fVec4[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec14[0] + 1e-20;
			let mut fTemp3: F32 = self.fVec4[((i32::wrapping_sub(self.IOTA0, self.iConst19)) & 32767) as usize] + fTemp1 - 0.6 * self.fRec12[1];
			self.fVec5[(self.IOTA0 & 4095) as usize] = fTemp3;
			self.fRec12[0] = self.fVec5[((i32::wrapping_sub(self.IOTA0, self.iConst20)) & 4095) as usize];
			let mut fRec13: F32 = 0.6 * fTemp3;
			let mut fTemp4: F32 = fRec13 + fRec9;
			self.fRec19[0] = -(self.fConst7 * (self.fConst6 * self.fRec19[1] - (self.fRec2[1] + self.fRec2[2])));
			self.fRec18[0] = fSlow29 * (self.fRec2[1] + fSlow28 * self.fRec19[0]) + fSlow27 * self.fRec18[1];
			self.fVec6[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec18[0] + 1e-20;
			let mut fTemp5: F32 = self.fVec6[((i32::wrapping_sub(self.IOTA0, self.iConst26)) & 32767) as usize] - (fTemp1 + 0.6 * self.fRec16[1]);
			self.fVec7[(self.IOTA0 & 8191) as usize] = fTemp5;
			self.fRec16[0] = self.fVec7[((i32::wrapping_sub(self.IOTA0, self.iConst27)) & 8191) as usize];
			let mut fRec17: F32 = 0.6 * fTemp5;
			self.fRec23[0] = -(self.fConst7 * (self.fConst6 * self.fRec23[1] - (self.fRec6[1] + self.fRec6[2])));
			self.fRec22[0] = fSlow38 * (self.fRec6[1] + fSlow37 * self.fRec23[0]) + fSlow36 * self.fRec22[1];
			self.fVec8[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec22[0] + 1e-20;
			let mut fTemp6: F32 = self.fVec8[((i32::wrapping_sub(self.IOTA0, self.iConst33)) & 32767) as usize] - (fTemp1 + 0.6 * self.fRec20[1]);
			self.fVec9[(self.IOTA0 & 4095) as usize] = fTemp6;
			self.fRec20[0] = self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst34)) & 4095) as usize];
			let mut fRec21: F32 = 0.6 * fTemp6;
			let mut fTemp7: F32 = fRec21 + fRec17 + fTemp4;
			self.fRec27[0] = -(self.fConst7 * (self.fConst6 * self.fRec27[1] - (self.fRec1[1] + self.fRec1[2])));
			self.fRec26[0] = fSlow47 * (self.fRec1[1] + fSlow46 * self.fRec27[0]) + fSlow45 * self.fRec26[1];
			self.fVec10[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec26[0] + 1e-20;
			let mut fTemp8: F32 = *input1;
			self.fVec11[(self.IOTA0 & 16383) as usize] = fTemp8;
			let mut fTemp9: F32 = 0.3 * self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow11)) & 16383) as usize];
			let mut fTemp10: F32 = fTemp9 + 0.6 * self.fRec24[1] + self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst40)) & 65535) as usize];
			self.fVec12[(self.IOTA0 & 8191) as usize] = fTemp10;
			self.fRec24[0] = self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst41)) & 8191) as usize];
			let mut fRec25: F32 = -(0.6 * fTemp10);
			self.fRec31[0] = -(self.fConst7 * (self.fConst6 * self.fRec31[1] - (self.fRec5[1] + self.fRec5[2])));
			self.fRec30[0] = fSlow56 * (self.fRec5[1] + fSlow55 * self.fRec31[0]) + fSlow54 * self.fRec30[1];
			self.fVec13[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec30[0] + 1e-20;
			let mut fTemp11: F32 = self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst47)) & 32767) as usize] + fTemp9 + 0.6 * self.fRec28[1];
			self.fVec14[(self.IOTA0 & 8191) as usize] = fTemp11;
			self.fRec28[0] = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst48)) & 8191) as usize];
			let mut fRec29: F32 = -(0.6 * fTemp11);
			self.fRec35[0] = -(self.fConst7 * (self.fConst6 * self.fRec35[1] - (self.fRec3[1] + self.fRec3[2])));
			self.fRec34[0] = fSlow65 * (self.fRec3[1] + fSlow64 * self.fRec35[0]) + fSlow63 * self.fRec34[1];
			self.fVec15[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec34[0] + 1e-20;
			let mut fTemp12: F32 = 0.6 * self.fRec32[1] + self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst54)) & 65535) as usize];
			self.fVec16[(self.IOTA0 & 8191) as usize] = fTemp12 - fTemp9;
			self.fRec32[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst55)) & 8191) as usize];
			let mut fRec33: F32 = 0.6 * (fTemp9 - fTemp12);
			self.fRec39[0] = -(self.fConst7 * (self.fConst6 * self.fRec39[1] - (self.fRec7[1] + self.fRec7[2])));
			self.fRec38[0] = fSlow74 * (self.fRec7[1] + fSlow73 * self.fRec39[0]) + fSlow72 * self.fRec38[1];
			self.fVec17[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec38[0] + 1e-20;
			let mut fTemp13: F32 = 0.6 * self.fRec36[1] + self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst61)) & 65535) as usize];
			self.fVec18[(self.IOTA0 & 4095) as usize] = fTemp13 - fTemp9;
			self.fRec36[0] = self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst62)) & 4095) as usize];
			let mut fRec37: F32 = 0.6 * (fTemp9 - fTemp13);
			self.fRec0[0] = self.fRec36[1] + self.fRec32[1] + self.fRec28[1] + self.fRec24[1] + self.fRec20[1] + self.fRec16[1] + self.fRec8[1] + self.fRec12[1] + fRec37 + fRec33 + fRec29 + fRec25 + fTemp7;
			self.fRec1[0] = self.fRec20[1] + self.fRec16[1] + self.fRec8[1] + self.fRec12[1] + fTemp7 - (self.fRec36[1] + self.fRec32[1] + self.fRec28[1] + self.fRec24[1] + fRec37 + fRec33 + fRec25 + fRec29);
			let mut fTemp14: F32 = fRec17 + fRec21;
			self.fRec2[0] = self.fRec28[1] + self.fRec24[1] + self.fRec8[1] + self.fRec12[1] + fRec29 + fRec25 + fTemp4 - (self.fRec36[1] + self.fRec32[1] + self.fRec20[1] + self.fRec16[1] + fRec37 + fRec33 + fTemp14);
			self.fRec3[0] = self.fRec36[1] + self.fRec32[1] + self.fRec8[1] + self.fRec12[1] + fRec37 + fRec33 + fTemp4 - (self.fRec28[1] + self.fRec24[1] + self.fRec20[1] + self.fRec16[1] + fRec29 + fRec25 + fTemp14);
			let mut fTemp15: F32 = fRec9 + fRec21;
			let mut fTemp16: F32 = fRec13 + fRec17;
			self.fRec4[0] = self.fRec32[1] + self.fRec24[1] + self.fRec16[1] + self.fRec12[1] + fRec33 + fRec25 + fTemp16 - (self.fRec36[1] + self.fRec28[1] + self.fRec20[1] + self.fRec8[1] + fRec37 + fRec29 + fTemp15);
			self.fRec5[0] = self.fRec36[1] + self.fRec28[1] + self.fRec16[1] + self.fRec12[1] + fRec37 + fRec29 + fTemp16 - (self.fRec32[1] + self.fRec24[1] + self.fRec20[1] + self.fRec8[1] + fRec33 + fRec25 + fTemp15);
			let mut fTemp17: F32 = fRec9 + fRec17;
			let mut fTemp18: F32 = fRec13 + fRec21;
			self.fRec6[0] = self.fRec36[1] + self.fRec24[1] + self.fRec20[1] + self.fRec12[1] + fRec37 + fRec25 + fTemp18 - (self.fRec32[1] + self.fRec28[1] + self.fRec16[1] + self.fRec8[1] + fRec33 + fRec29 + fTemp17);
			self.fRec7[0] = self.fRec32[1] + self.fRec28[1] + self.fRec20[1] + self.fRec12[1] + fRec33 + fRec29 + fTemp18 - (self.fRec36[1] + self.fRec24[1] + self.fRec16[1] + self.fRec8[1] + fRec37 + fRec25 + fTemp17);
			let mut fTemp19: F32 = (self.iVec1[1]) as F32;
			self.fRec40[0] = fSlow75 * (1.0 - 0.999 * fTemp19) + 0.999 * fTemp19 * self.fRec40[1];
			let mut fTemp20: F32 = if (F32::abs(self.fRec40[0] - fSlow75) < 1e-06) as i32 != 0 {fSlow75} else {self.fRec40[0]};
			*output0 = fTemp0 + fTemp20 * (0.37 * (self.fRec1[0] + self.fRec2[0]) - fTemp0);
			*output1 = fTemp8 + fTemp20 * (0.37 * (self.fRec1[0] - self.fRec2[0]) - fTemp8);
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.iVec1[1] = self.iVec1[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec0[2] = self.fRec0[1];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[2] = self.fRec1[1];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[2] = self.fRec2[1];
			self.fRec2[1] = self.fRec2[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[2] = self.fRec5[1];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[2] = self.fRec6[1];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[2] = self.fRec7[1];
			self.fRec7[1] = self.fRec7[0];
			self.fRec40[1] = self.fRec40[0];
		}
	}

}

