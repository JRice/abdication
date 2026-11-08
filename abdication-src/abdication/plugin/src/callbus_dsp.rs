/* ------------------------------------------------------------
name: "plugin-callbus"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn CallBusDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn CallBusDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct CallBusDsp {
	fSampleRate: i32,
	fConst10: F32,
	fConst12: F32,
	fConst13: F32,
	fRec11: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fRec10: [F32;2],
	IOTA0: i32,
	fVec0: [F32;32768],
	iConst17: i32,
	fConst18: F32,
	fConst19: F32,
	fCheckbox0: F32,
	fRec13: [F32;2],
	fHslider0: F32,
	fRec14: [F32;2],
	fVec1: [F32;131072],
	iConst20: i32,
	fRec12: [F32;2],
	fVec2: [F32;8192],
	iConst21: i32,
	fVec3: [F32;8192],
	iConst22: i32,
	fRec8: [F32;2],
	fConst31: F32,
	fRec18: [F32;2],
	fConst32: F32,
	fConst33: F32,
	fRec17: [F32;2],
	fVec4: [F32;32768],
	iConst35: i32,
	fVec5: [F32;4096],
	iConst36: i32,
	fRec15: [F32;2],
	fConst45: F32,
	fRec22: [F32;2],
	fConst46: F32,
	fConst47: F32,
	fRec21: [F32;2],
	fVec6: [F32;32768],
	iConst49: i32,
	fVec7: [F32;8192],
	iConst50: i32,
	fRec19: [F32;2],
	fConst59: F32,
	fRec26: [F32;2],
	fConst60: F32,
	fConst61: F32,
	fRec25: [F32;2],
	fVec8: [F32;32768],
	iConst63: i32,
	fVec9: [F32;4096],
	iConst64: i32,
	fRec23: [F32;2],
	fConst73: F32,
	fRec30: [F32;2],
	fConst74: F32,
	fConst75: F32,
	fRec29: [F32;2],
	fVec10: [F32;65536],
	iConst77: i32,
	fVec11: [F32;131072],
	iConst78: i32,
	fRec31: [F32;2],
	fVec12: [F32;8192],
	fVec13: [F32;8192],
	iConst79: i32,
	fRec27: [F32;2],
	fConst88: F32,
	fRec35: [F32;2],
	fConst89: F32,
	fConst90: F32,
	fRec34: [F32;2],
	fVec14: [F32;32768],
	iConst92: i32,
	fVec15: [F32;8192],
	iConst93: i32,
	fRec32: [F32;2],
	fConst102: F32,
	fRec39: [F32;2],
	fConst103: F32,
	fConst104: F32,
	fRec38: [F32;2],
	fVec16: [F32;65536],
	iConst106: i32,
	fVec17: [F32;8192],
	iConst107: i32,
	fRec36: [F32;2],
	fConst116: F32,
	fRec43: [F32;2],
	fConst117: F32,
	fConst118: F32,
	fRec42: [F32;2],
	fVec18: [F32;65536],
	iConst120: i32,
	fVec19: [F32;4096],
	iConst121: i32,
	fRec40: [F32;2],
	fRec0: [F32;3],
	fRec1: [F32;3],
	fRec2: [F32;3],
	fRec3: [F32;3],
	fRec4: [F32;3],
	fRec5: [F32;3],
	fRec6: [F32;3],
	fRec7: [F32;3],
}

impl FaustDsp for CallBusDsp {
	type T = F32;
		
	fn new() -> CallBusDsp { 
		CallBusDsp {
			fSampleRate: 0,
			fConst10: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fRec11: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fRec10: [0.0;2],
			IOTA0: 0,
			fVec0: [0.0;32768],
			iConst17: 0,
			fConst18: 0.0,
			fConst19: 0.0,
			fCheckbox0: 0.0,
			fRec13: [0.0;2],
			fHslider0: 0.0,
			fRec14: [0.0;2],
			fVec1: [0.0;131072],
			iConst20: 0,
			fRec12: [0.0;2],
			fVec2: [0.0;8192],
			iConst21: 0,
			fVec3: [0.0;8192],
			iConst22: 0,
			fRec8: [0.0;2],
			fConst31: 0.0,
			fRec18: [0.0;2],
			fConst32: 0.0,
			fConst33: 0.0,
			fRec17: [0.0;2],
			fVec4: [0.0;32768],
			iConst35: 0,
			fVec5: [0.0;4096],
			iConst36: 0,
			fRec15: [0.0;2],
			fConst45: 0.0,
			fRec22: [0.0;2],
			fConst46: 0.0,
			fConst47: 0.0,
			fRec21: [0.0;2],
			fVec6: [0.0;32768],
			iConst49: 0,
			fVec7: [0.0;8192],
			iConst50: 0,
			fRec19: [0.0;2],
			fConst59: 0.0,
			fRec26: [0.0;2],
			fConst60: 0.0,
			fConst61: 0.0,
			fRec25: [0.0;2],
			fVec8: [0.0;32768],
			iConst63: 0,
			fVec9: [0.0;4096],
			iConst64: 0,
			fRec23: [0.0;2],
			fConst73: 0.0,
			fRec30: [0.0;2],
			fConst74: 0.0,
			fConst75: 0.0,
			fRec29: [0.0;2],
			fVec10: [0.0;65536],
			iConst77: 0,
			fVec11: [0.0;131072],
			iConst78: 0,
			fRec31: [0.0;2],
			fVec12: [0.0;8192],
			fVec13: [0.0;8192],
			iConst79: 0,
			fRec27: [0.0;2],
			fConst88: 0.0,
			fRec35: [0.0;2],
			fConst89: 0.0,
			fConst90: 0.0,
			fRec34: [0.0;2],
			fVec14: [0.0;32768],
			iConst92: 0,
			fVec15: [0.0;8192],
			iConst93: 0,
			fRec32: [0.0;2],
			fConst102: 0.0,
			fRec39: [0.0;2],
			fConst103: 0.0,
			fConst104: 0.0,
			fRec38: [0.0;2],
			fVec16: [0.0;65536],
			iConst106: 0,
			fVec17: [0.0;8192],
			iConst107: 0,
			fRec36: [0.0;2],
			fConst116: 0.0,
			fRec43: [0.0;2],
			fConst117: 0.0,
			fConst118: 0.0,
			fRec42: [0.0;2],
			fVec18: [0.0;65536],
			iConst120: 0,
			fVec19: [0.0;4096],
			iConst121: 0,
			fRec40: [0.0;2],
			fRec0: [0.0;3],
			fRec1: [0.0;3],
			fRec2: [0.0;3],
			fRec3: [0.0;3],
			fRec4: [0.0;3],
			fRec5: [0.0;3],
			fRec6: [0.0;3],
			fRec7: [0.0;3],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn CallBusDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("filename", r"plugin-callbus.dsp");
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
		m.declare("name", r"plugin-callbus");
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
		self.fCheckbox0 = 0.0;
		self.fHslider0 = 0.5;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec11[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec10[l1 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l2 in 0..32768 {
			self.fVec0[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec13[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec14[l4 as usize] = 0.0;
		}
		for l5 in 0..131072 {
			self.fVec1[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec12[l6 as usize] = 0.0;
		}
		for l7 in 0..8192 {
			self.fVec2[l7 as usize] = 0.0;
		}
		for l8 in 0..8192 {
			self.fVec3[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec8[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec18[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec17[l11 as usize] = 0.0;
		}
		for l12 in 0..32768 {
			self.fVec4[l12 as usize] = 0.0;
		}
		for l13 in 0..4096 {
			self.fVec5[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec15[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec22[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec21[l16 as usize] = 0.0;
		}
		for l17 in 0..32768 {
			self.fVec6[l17 as usize] = 0.0;
		}
		for l18 in 0..8192 {
			self.fVec7[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec19[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec26[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec25[l21 as usize] = 0.0;
		}
		for l22 in 0..32768 {
			self.fVec8[l22 as usize] = 0.0;
		}
		for l23 in 0..4096 {
			self.fVec9[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec23[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec30[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec29[l26 as usize] = 0.0;
		}
		for l27 in 0..65536 {
			self.fVec10[l27 as usize] = 0.0;
		}
		for l28 in 0..131072 {
			self.fVec11[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec31[l29 as usize] = 0.0;
		}
		for l30 in 0..8192 {
			self.fVec12[l30 as usize] = 0.0;
		}
		for l31 in 0..8192 {
			self.fVec13[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec27[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec35[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec34[l34 as usize] = 0.0;
		}
		for l35 in 0..32768 {
			self.fVec14[l35 as usize] = 0.0;
		}
		for l36 in 0..8192 {
			self.fVec15[l36 as usize] = 0.0;
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
			self.fVec16[l40 as usize] = 0.0;
		}
		for l41 in 0..8192 {
			self.fVec17[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec36[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec43[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec42[l44 as usize] = 0.0;
		}
		for l45 in 0..65536 {
			self.fVec18[l45 as usize] = 0.0;
		}
		for l46 in 0..4096 {
			self.fVec19[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec40[l47 as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fRec0[l48 as usize] = 0.0;
		}
		for l49 in 0..3 {
			self.fRec1[l49 as usize] = 0.0;
		}
		for l50 in 0..3 {
			self.fRec2[l50 as usize] = 0.0;
		}
		for l51 in 0..3 {
			self.fRec3[l51 as usize] = 0.0;
		}
		for l52 in 0..3 {
			self.fRec4[l52 as usize] = 0.0;
		}
		for l53 in 0..3 {
			self.fRec5[l53 as usize] = 0.0;
		}
		for l54 in 0..3 {
			self.fRec6[l54 as usize] = 0.0;
		}
		for l55 in 0..3 {
			self.fRec7[l55 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		let mut fConst1: F32 = F32::floor(0.174713 * fConst0 + 0.5);
		let mut fConst2: F32 = fConst1 / fConst0;
		let mut fConst3: F32 = F32::exp(-(1.381551 * fConst2));
		let mut fConst4: F32 = CallBusDsp_faustpower2_f(fConst3);
		let mut fConst5: F32 = 1.0 - fConst4;
		let mut fConst6: F32 = F32::cos(15707.963 / fConst0);
		let mut fConst7: F32 = 1.0 - fConst6 * fConst4;
		let mut fConst8: F32 = F32::sqrt(F32::max(0.0, CallBusDsp_faustpower2_f(fConst7) / CallBusDsp_faustpower2_f(fConst5) + -1.0));
		let mut fConst9: F32 = fConst7 / fConst5;
		self.fConst10 = fConst9 - fConst8;
		let mut fConst11: F32 = 1.0 / F32::tan(942.4778 / fConst0);
		self.fConst12 = 1.0 - fConst11;
		self.fConst13 = 1.0 / (fConst11 + 1.0);
		self.fConst14 = F32::exp(-(1.7269388 * fConst2)) / fConst3 + -1.0;
		self.fConst15 = fConst3 * (fConst8 + (1.0 - fConst9));
		let mut fConst16: F32 = F32::floor(0.022904 * fConst0 + 0.5);
		self.iConst17 = (F32::min(16384.0, F32::max(0.0, fConst1 - fConst16))) as i32;
		self.fConst18 = 44.1 / fConst0;
		self.fConst19 = 1.0 - self.fConst18;
		self.iConst20 = (0.36 * fConst0) as i32;
		self.iConst21 = (F32::min(8192.0, F32::max(0.0, 0.03 * fConst0))) as i32;
		self.iConst22 = (F32::min(4096.0, F32::max(0.0, fConst16 + -1.0))) as i32;
		let mut fConst23: F32 = F32::floor(0.153129 * fConst0 + 0.5);
		let mut fConst24: F32 = fConst23 / fConst0;
		let mut fConst25: F32 = F32::exp(-(1.381551 * fConst24));
		let mut fConst26: F32 = CallBusDsp_faustpower2_f(fConst25);
		let mut fConst27: F32 = 1.0 - fConst26;
		let mut fConst28: F32 = 1.0 - fConst6 * fConst26;
		let mut fConst29: F32 = F32::sqrt(F32::max(0.0, CallBusDsp_faustpower2_f(fConst28) / CallBusDsp_faustpower2_f(fConst27) + -1.0));
		let mut fConst30: F32 = fConst28 / fConst27;
		self.fConst31 = fConst30 - fConst29;
		self.fConst32 = F32::exp(-(1.7269388 * fConst24)) / fConst25 + -1.0;
		self.fConst33 = fConst25 * (fConst29 + (1.0 - fConst30));
		let mut fConst34: F32 = F32::floor(0.020346 * fConst0 + 0.5);
		self.iConst35 = (F32::min(16384.0, F32::max(0.0, fConst23 - fConst34))) as i32;
		self.iConst36 = (F32::min(2048.0, F32::max(0.0, fConst34 + -1.0))) as i32;
		let mut fConst37: F32 = F32::floor(0.127837 * fConst0 + 0.5);
		let mut fConst38: F32 = fConst37 / fConst0;
		let mut fConst39: F32 = F32::exp(-(1.381551 * fConst38));
		let mut fConst40: F32 = CallBusDsp_faustpower2_f(fConst39);
		let mut fConst41: F32 = 1.0 - fConst40;
		let mut fConst42: F32 = 1.0 - fConst6 * fConst40;
		let mut fConst43: F32 = F32::sqrt(F32::max(0.0, CallBusDsp_faustpower2_f(fConst42) / CallBusDsp_faustpower2_f(fConst41) + -1.0));
		let mut fConst44: F32 = fConst42 / fConst41;
		self.fConst45 = fConst44 - fConst43;
		self.fConst46 = F32::exp(-(1.7269388 * fConst38)) / fConst39 + -1.0;
		self.fConst47 = fConst39 * (fConst43 + (1.0 - fConst44));
		let mut fConst48: F32 = F32::floor(0.031604 * fConst0 + 0.5);
		self.iConst49 = (F32::min(16384.0, F32::max(0.0, fConst37 - fConst48))) as i32;
		self.iConst50 = (F32::min(4096.0, F32::max(0.0, fConst48 + -1.0))) as i32;
		let mut fConst51: F32 = F32::floor(0.125 * fConst0 + 0.5);
		let mut fConst52: F32 = fConst51 / fConst0;
		let mut fConst53: F32 = F32::exp(-(1.381551 * fConst52));
		let mut fConst54: F32 = CallBusDsp_faustpower2_f(fConst53);
		let mut fConst55: F32 = 1.0 - fConst54;
		let mut fConst56: F32 = 1.0 - fConst6 * fConst54;
		let mut fConst57: F32 = F32::sqrt(F32::max(0.0, CallBusDsp_faustpower2_f(fConst56) / CallBusDsp_faustpower2_f(fConst55) + -1.0));
		let mut fConst58: F32 = fConst56 / fConst55;
		self.fConst59 = fConst58 - fConst57;
		self.fConst60 = F32::exp(-(1.7269388 * fConst52)) / fConst53 + -1.0;
		self.fConst61 = fConst53 * (fConst57 + (1.0 - fConst58));
		let mut fConst62: F32 = F32::floor(0.013458 * fConst0 + 0.5);
		self.iConst63 = (F32::min(16384.0, F32::max(0.0, fConst51 - fConst62))) as i32;
		self.iConst64 = (F32::min(2048.0, F32::max(0.0, fConst62 + -1.0))) as i32;
		let mut fConst65: F32 = F32::floor(0.210389 * fConst0 + 0.5);
		let mut fConst66: F32 = fConst65 / fConst0;
		let mut fConst67: F32 = F32::exp(-(1.381551 * fConst66));
		let mut fConst68: F32 = CallBusDsp_faustpower2_f(fConst67);
		let mut fConst69: F32 = 1.0 - fConst68;
		let mut fConst70: F32 = 1.0 - fConst6 * fConst68;
		let mut fConst71: F32 = F32::sqrt(F32::max(0.0, CallBusDsp_faustpower2_f(fConst70) / CallBusDsp_faustpower2_f(fConst69) + -1.0));
		let mut fConst72: F32 = fConst70 / fConst69;
		self.fConst73 = fConst72 - fConst71;
		self.fConst74 = F32::exp(-(1.7269388 * fConst66)) / fConst67 + -1.0;
		self.fConst75 = fConst67 * (fConst71 + (1.0 - fConst72));
		let mut fConst76: F32 = F32::floor(0.024421 * fConst0 + 0.5);
		self.iConst77 = (F32::min(32768.0, F32::max(0.0, fConst65 - fConst76))) as i32;
		self.iConst78 = (0.48 * fConst0) as i32;
		self.iConst79 = (F32::min(4096.0, F32::max(0.0, fConst76 + -1.0))) as i32;
		let mut fConst80: F32 = F32::floor(0.192303 * fConst0 + 0.5);
		let mut fConst81: F32 = fConst80 / fConst0;
		let mut fConst82: F32 = F32::exp(-(1.381551 * fConst81));
		let mut fConst83: F32 = CallBusDsp_faustpower2_f(fConst82);
		let mut fConst84: F32 = 1.0 - fConst83;
		let mut fConst85: F32 = 1.0 - fConst6 * fConst83;
		let mut fConst86: F32 = F32::sqrt(F32::max(0.0, CallBusDsp_faustpower2_f(fConst85) / CallBusDsp_faustpower2_f(fConst84) + -1.0));
		let mut fConst87: F32 = fConst85 / fConst84;
		self.fConst88 = fConst87 - fConst86;
		self.fConst89 = F32::exp(-(1.7269388 * fConst81)) / fConst82 + -1.0;
		self.fConst90 = fConst82 * (fConst86 + (1.0 - fConst87));
		let mut fConst91: F32 = F32::floor(0.029291 * fConst0 + 0.5);
		self.iConst92 = (F32::min(16384.0, F32::max(0.0, fConst80 - fConst91))) as i32;
		self.iConst93 = (F32::min(4096.0, F32::max(0.0, fConst91 + -1.0))) as i32;
		let mut fConst94: F32 = F32::floor(0.256891 * fConst0 + 0.5);
		let mut fConst95: F32 = fConst94 / fConst0;
		let mut fConst96: F32 = F32::exp(-(1.381551 * fConst95));
		let mut fConst97: F32 = CallBusDsp_faustpower2_f(fConst96);
		let mut fConst98: F32 = 1.0 - fConst97;
		let mut fConst99: F32 = 1.0 - fConst6 * fConst97;
		let mut fConst100: F32 = F32::sqrt(F32::max(0.0, CallBusDsp_faustpower2_f(fConst99) / CallBusDsp_faustpower2_f(fConst98) + -1.0));
		let mut fConst101: F32 = fConst99 / fConst98;
		self.fConst102 = fConst101 - fConst100;
		self.fConst103 = F32::exp(-(1.7269388 * fConst95)) / fConst96 + -1.0;
		self.fConst104 = fConst96 * (fConst100 + (1.0 - fConst101));
		let mut fConst105: F32 = F32::floor(0.027333 * fConst0 + 0.5);
		self.iConst106 = (F32::min(32768.0, F32::max(0.0, fConst94 - fConst105))) as i32;
		self.iConst107 = (F32::min(4096.0, F32::max(0.0, fConst105 + -1.0))) as i32;
		let mut fConst108: F32 = F32::floor(0.219991 * fConst0 + 0.5);
		let mut fConst109: F32 = fConst108 / fConst0;
		let mut fConst110: F32 = F32::exp(-(1.381551 * fConst109));
		let mut fConst111: F32 = CallBusDsp_faustpower2_f(fConst110);
		let mut fConst112: F32 = 1.0 - fConst111;
		let mut fConst113: F32 = 1.0 - fConst6 * fConst111;
		let mut fConst114: F32 = F32::sqrt(F32::max(0.0, CallBusDsp_faustpower2_f(fConst113) / CallBusDsp_faustpower2_f(fConst112) + -1.0));
		let mut fConst115: F32 = fConst113 / fConst112;
		self.fConst116 = fConst115 - fConst114;
		self.fConst117 = F32::exp(-(1.7269388 * fConst109)) / fConst110 + -1.0;
		self.fConst118 = fConst110 * (fConst114 + (1.0 - fConst115));
		let mut fConst119: F32 = F32::floor(0.019123 * fConst0 + 0.5);
		self.iConst120 = (F32::min(32768.0, F32::max(0.0, fConst108 - fConst119))) as i32;
		self.iConst121 = (F32::min(2048.0, F32::max(0.0, fConst119 + -1.0))) as i32;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		CallBusDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("callbus");
		ui_interface.declare(None, "12", "");
		ui_interface.open_horizontal_box("Calls");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.add_check_button("Mute", ParamIndex(0));
		ui_interface.declare(Some(ParamIndex(1)), "4", "");
		ui_interface.add_horizontal_slider("Echo", ParamIndex(1), 0.5, 0.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fCheckbox0),
			1 => Some(self.fHslider0),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			0 => { self.fCheckbox0 = value }
			1 => { self.fHslider0 = value }
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
		let mut fSlow0: F32 = self.fConst18 * (1.0 - self.fCheckbox0);
		let mut fSlow1: F32 = self.fConst18 * self.fHslider0;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec11[0] = -(self.fConst13 * (self.fConst12 * self.fRec11[1] - (self.fRec4[1] + self.fRec4[2])));
			self.fRec10[0] = self.fConst15 * (self.fRec4[1] + self.fConst14 * self.fRec11[0]) + self.fConst10 * self.fRec10[1];
			self.fVec0[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec10[0] + 1e-20;
			self.fRec13[0] = fSlow0 + self.fConst19 * self.fRec13[1];
			let mut fTemp0: F32 = *input0 * self.fRec13[0];
			self.fRec14[0] = fSlow1 + self.fConst19 * self.fRec14[1];
			self.fVec1[(self.IOTA0 & 131071) as usize] = self.fRec14[0] * (0.5012 * fTemp0 + 0.9 * self.fRec12[1]);
			self.fRec12[0] = self.fVec1[((i32::wrapping_sub(self.IOTA0, self.iConst20)) & 131071) as usize];
			self.fVec2[(self.IOTA0 & 8191) as usize] = fTemp0 + self.fRec12[0];
			let mut fTemp1: F32 = 0.3 * self.fVec2[((i32::wrapping_sub(self.IOTA0, self.iConst21)) & 8191) as usize];
			let mut fTemp2: F32 = fTemp1 + self.fVec0[((i32::wrapping_sub(self.IOTA0, self.iConst17)) & 32767) as usize] - 0.6 * self.fRec8[1];
			self.fVec3[(self.IOTA0 & 8191) as usize] = fTemp2;
			self.fRec8[0] = self.fVec3[((i32::wrapping_sub(self.IOTA0, self.iConst22)) & 8191) as usize];
			let mut fRec9: F32 = 0.6 * fTemp2;
			self.fRec18[0] = -(self.fConst13 * (self.fConst12 * self.fRec18[1] - (self.fRec0[1] + self.fRec0[2])));
			self.fRec17[0] = self.fConst33 * (self.fRec0[1] + self.fConst32 * self.fRec18[0]) + self.fConst31 * self.fRec17[1];
			self.fVec4[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec17[0] + 1e-20;
			let mut fTemp3: F32 = self.fVec4[((i32::wrapping_sub(self.IOTA0, self.iConst35)) & 32767) as usize] + fTemp1 - 0.6 * self.fRec15[1];
			self.fVec5[(self.IOTA0 & 4095) as usize] = fTemp3;
			self.fRec15[0] = self.fVec5[((i32::wrapping_sub(self.IOTA0, self.iConst36)) & 4095) as usize];
			let mut fRec16: F32 = 0.6 * fTemp3;
			let mut fTemp4: F32 = fRec16 + fRec9;
			self.fRec22[0] = -(self.fConst13 * (self.fConst12 * self.fRec22[1] - (self.fRec2[1] + self.fRec2[2])));
			self.fRec21[0] = self.fConst47 * (self.fRec2[1] + self.fConst46 * self.fRec22[0]) + self.fConst45 * self.fRec21[1];
			self.fVec6[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec21[0] + 1e-20;
			let mut fTemp5: F32 = self.fVec6[((i32::wrapping_sub(self.IOTA0, self.iConst49)) & 32767) as usize] - (fTemp1 + 0.6 * self.fRec19[1]);
			self.fVec7[(self.IOTA0 & 8191) as usize] = fTemp5;
			self.fRec19[0] = self.fVec7[((i32::wrapping_sub(self.IOTA0, self.iConst50)) & 8191) as usize];
			let mut fRec20: F32 = 0.6 * fTemp5;
			self.fRec26[0] = -(self.fConst13 * (self.fConst12 * self.fRec26[1] - (self.fRec6[1] + self.fRec6[2])));
			self.fRec25[0] = self.fConst61 * (self.fRec6[1] + self.fConst60 * self.fRec26[0]) + self.fConst59 * self.fRec25[1];
			self.fVec8[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec25[0] + 1e-20;
			let mut fTemp6: F32 = self.fVec8[((i32::wrapping_sub(self.IOTA0, self.iConst63)) & 32767) as usize] - (fTemp1 + 0.6 * self.fRec23[1]);
			self.fVec9[(self.IOTA0 & 4095) as usize] = fTemp6;
			self.fRec23[0] = self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst64)) & 4095) as usize];
			let mut fRec24: F32 = 0.6 * fTemp6;
			let mut fTemp7: F32 = fRec24 + fRec20 + fTemp4;
			self.fRec30[0] = -(self.fConst13 * (self.fConst12 * self.fRec30[1] - (self.fRec1[1] + self.fRec1[2])));
			self.fRec29[0] = self.fConst75 * (self.fRec1[1] + self.fConst74 * self.fRec30[0]) + self.fConst73 * self.fRec29[1];
			self.fVec10[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec29[0] + 1e-20;
			let mut fTemp8: F32 = *input1 * self.fRec13[0];
			self.fVec11[(self.IOTA0 & 131071) as usize] = self.fRec14[0] * (0.5012 * fTemp8 + 0.9 * self.fRec31[1]);
			self.fRec31[0] = self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst78)) & 131071) as usize];
			self.fVec12[(self.IOTA0 & 8191) as usize] = fTemp8 + self.fRec31[0];
			let mut fTemp9: F32 = 0.3 * self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst21)) & 8191) as usize];
			let mut fTemp10: F32 = fTemp9 + 0.6 * self.fRec27[1] + self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst77)) & 65535) as usize];
			self.fVec13[(self.IOTA0 & 8191) as usize] = fTemp10;
			self.fRec27[0] = self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst79)) & 8191) as usize];
			let mut fRec28: F32 = -(0.6 * fTemp10);
			self.fRec35[0] = -(self.fConst13 * (self.fConst12 * self.fRec35[1] - (self.fRec5[1] + self.fRec5[2])));
			self.fRec34[0] = self.fConst90 * (self.fRec5[1] + self.fConst89 * self.fRec35[0]) + self.fConst88 * self.fRec34[1];
			self.fVec14[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec34[0] + 1e-20;
			let mut fTemp11: F32 = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst92)) & 32767) as usize] + fTemp9 + 0.6 * self.fRec32[1];
			self.fVec15[(self.IOTA0 & 8191) as usize] = fTemp11;
			self.fRec32[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst93)) & 8191) as usize];
			let mut fRec33: F32 = -(0.6 * fTemp11);
			self.fRec39[0] = -(self.fConst13 * (self.fConst12 * self.fRec39[1] - (self.fRec3[1] + self.fRec3[2])));
			self.fRec38[0] = self.fConst104 * (self.fRec3[1] + self.fConst103 * self.fRec39[0]) + self.fConst102 * self.fRec38[1];
			self.fVec16[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec38[0] + 1e-20;
			let mut fTemp12: F32 = 0.6 * self.fRec36[1] + self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst106)) & 65535) as usize];
			self.fVec17[(self.IOTA0 & 8191) as usize] = fTemp12 - fTemp9;
			self.fRec36[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst107)) & 8191) as usize];
			let mut fRec37: F32 = 0.6 * (fTemp9 - fTemp12);
			self.fRec43[0] = -(self.fConst13 * (self.fConst12 * self.fRec43[1] - (self.fRec7[1] + self.fRec7[2])));
			self.fRec42[0] = self.fConst118 * (self.fRec7[1] + self.fConst117 * self.fRec43[0]) + self.fConst116 * self.fRec42[1];
			self.fVec18[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec42[0] + 1e-20;
			let mut fTemp13: F32 = 0.6 * self.fRec40[1] + self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst120)) & 65535) as usize];
			self.fVec19[(self.IOTA0 & 4095) as usize] = fTemp13 - fTemp9;
			self.fRec40[0] = self.fVec19[((i32::wrapping_sub(self.IOTA0, self.iConst121)) & 4095) as usize];
			let mut fRec41: F32 = 0.6 * (fTemp9 - fTemp13);
			self.fRec0[0] = self.fRec40[1] + self.fRec36[1] + self.fRec32[1] + self.fRec27[1] + self.fRec23[1] + self.fRec19[1] + self.fRec8[1] + self.fRec15[1] + fRec41 + fRec37 + fRec33 + fRec28 + fTemp7;
			self.fRec1[0] = self.fRec23[1] + self.fRec19[1] + self.fRec8[1] + self.fRec15[1] + fTemp7 - (self.fRec40[1] + self.fRec36[1] + self.fRec32[1] + self.fRec27[1] + fRec41 + fRec37 + fRec28 + fRec33);
			let mut fTemp14: F32 = fRec20 + fRec24;
			self.fRec2[0] = self.fRec32[1] + self.fRec27[1] + self.fRec8[1] + self.fRec15[1] + fRec33 + fRec28 + fTemp4 - (self.fRec40[1] + self.fRec36[1] + self.fRec23[1] + self.fRec19[1] + fRec41 + fRec37 + fTemp14);
			self.fRec3[0] = self.fRec40[1] + self.fRec36[1] + self.fRec8[1] + self.fRec15[1] + fRec41 + fRec37 + fTemp4 - (self.fRec32[1] + self.fRec27[1] + self.fRec23[1] + self.fRec19[1] + fRec33 + fRec28 + fTemp14);
			let mut fTemp15: F32 = fRec9 + fRec24;
			let mut fTemp16: F32 = fRec16 + fRec20;
			self.fRec4[0] = self.fRec36[1] + self.fRec27[1] + self.fRec19[1] + self.fRec15[1] + fRec37 + fRec28 + fTemp16 - (self.fRec40[1] + self.fRec32[1] + self.fRec23[1] + self.fRec8[1] + fRec41 + fRec33 + fTemp15);
			self.fRec5[0] = self.fRec40[1] + self.fRec32[1] + self.fRec19[1] + self.fRec15[1] + fRec41 + fRec33 + fTemp16 - (self.fRec36[1] + self.fRec27[1] + self.fRec23[1] + self.fRec8[1] + fRec37 + fRec28 + fTemp15);
			let mut fTemp17: F32 = fRec9 + fRec20;
			let mut fTemp18: F32 = fRec16 + fRec24;
			self.fRec6[0] = self.fRec40[1] + self.fRec27[1] + self.fRec23[1] + self.fRec15[1] + fRec41 + fRec28 + fTemp18 - (self.fRec36[1] + self.fRec32[1] + self.fRec19[1] + self.fRec8[1] + fRec37 + fRec33 + fTemp17);
			self.fRec7[0] = self.fRec36[1] + self.fRec32[1] + self.fRec23[1] + self.fRec15[1] + fRec37 + fRec33 + fTemp18 - (self.fRec40[1] + self.fRec27[1] + self.fRec19[1] + self.fRec8[1] + fRec41 + fRec28 + fTemp17);
			*output0 = 0.37 * self.fRec13[0] * (self.fRec1[0] + self.fRec2[0]);
			*output1 = 0.37 * self.fRec13[0] * (self.fRec1[0] - self.fRec2[0]);
			self.fRec11[1] = self.fRec11[0];
			self.fRec10[1] = self.fRec10[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec40[1] = self.fRec40[0];
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
		}
	}

}

