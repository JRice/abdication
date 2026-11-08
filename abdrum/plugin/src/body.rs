/* ------------------------------------------------------------
name: "ABDrumBody"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn BodyDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn BodyDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct BodyDsp {
	fButton0: F32,
	fVec0: [F32;2],
	fSampleRate: i32,
	fConst1: F32,
	fHslider0: F32,
	fHslider1: F32,
	fHslider2: F32,
	fConst2: F32,
	fRec1: [F32;2],
	fHslider3: F32,
	fConst3: F32,
	fConst4: F32,
	fRec2: [F32;2],
	fHslider4: F32,
	fRec3: [F32;2],
	fRec0: [F32;3],
	fRec5: [F32;2],
	fRec6: [F32;2],
	fRec7: [F32;2],
	fRec4: [F32;3],
	fRec9: [F32;2],
	fRec10: [F32;2],
	fRec11: [F32;2],
	fRec8: [F32;3],
	fRec13: [F32;2],
	fRec14: [F32;2],
	fRec15: [F32;2],
	fRec12: [F32;3],
	fRec17: [F32;2],
	fRec18: [F32;2],
	fRec19: [F32;2],
	fRec16: [F32;3],
	fRec21: [F32;2],
	fRec22: [F32;2],
	fRec23: [F32;2],
	fRec20: [F32;3],
	fRec25: [F32;2],
	fRec26: [F32;2],
	fRec27: [F32;2],
	fRec24: [F32;3],
	fRec29: [F32;2],
	fRec30: [F32;2],
	fRec31: [F32;2],
	fRec28: [F32;3],
	fRec33: [F32;2],
	fRec34: [F32;2],
	fRec35: [F32;2],
	fRec32: [F32;3],
	fRec37: [F32;2],
	fRec38: [F32;2],
	fRec39: [F32;2],
	fRec36: [F32;3],
	fRec41: [F32;2],
	fRec42: [F32;2],
	fRec43: [F32;2],
	fRec40: [F32;3],
	fRec45: [F32;2],
	fRec46: [F32;2],
	fRec47: [F32;2],
	fRec44: [F32;3],
	fRec49: [F32;2],
	fRec50: [F32;2],
	fRec51: [F32;2],
	fRec48: [F32;3],
	fRec53: [F32;2],
	fRec54: [F32;2],
	fRec55: [F32;2],
	fRec52: [F32;3],
	fRec57: [F32;2],
	fRec58: [F32;2],
	fRec59: [F32;2],
	fRec56: [F32;3],
	fRec61: [F32;2],
	fRec62: [F32;2],
	fRec63: [F32;2],
	fRec60: [F32;3],
	fRec65: [F32;2],
	fRec66: [F32;2],
	fRec67: [F32;2],
	fRec64: [F32;3],
	fRec69: [F32;2],
	fRec70: [F32;2],
	fRec71: [F32;2],
	fRec68: [F32;3],
	fRec73: [F32;2],
	fRec74: [F32;2],
	fRec75: [F32;2],
	fRec72: [F32;3],
	fRec77: [F32;2],
	fRec78: [F32;2],
	fRec79: [F32;2],
	fRec76: [F32;3],
	fRec81: [F32;2],
	fRec82: [F32;2],
	fRec83: [F32;2],
	fRec80: [F32;3],
	fRec85: [F32;2],
	fRec86: [F32;2],
	fRec87: [F32;2],
	fRec84: [F32;3],
	fRec89: [F32;2],
	fRec90: [F32;2],
	fRec91: [F32;2],
	fRec88: [F32;3],
	fRec93: [F32;2],
	fRec94: [F32;2],
	fRec95: [F32;2],
	fRec92: [F32;3],
}

impl FaustDsp for BodyDsp {
	type T = F32;
		
	fn new() -> BodyDsp { 
		BodyDsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			fSampleRate: 0,
			fConst1: 0.0,
			fHslider0: 0.0,
			fHslider1: 0.0,
			fHslider2: 0.0,
			fConst2: 0.0,
			fRec1: [0.0;2],
			fHslider3: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			fRec2: [0.0;2],
			fHslider4: 0.0,
			fRec3: [0.0;2],
			fRec0: [0.0;3],
			fRec5: [0.0;2],
			fRec6: [0.0;2],
			fRec7: [0.0;2],
			fRec4: [0.0;3],
			fRec9: [0.0;2],
			fRec10: [0.0;2],
			fRec11: [0.0;2],
			fRec8: [0.0;3],
			fRec13: [0.0;2],
			fRec14: [0.0;2],
			fRec15: [0.0;2],
			fRec12: [0.0;3],
			fRec17: [0.0;2],
			fRec18: [0.0;2],
			fRec19: [0.0;2],
			fRec16: [0.0;3],
			fRec21: [0.0;2],
			fRec22: [0.0;2],
			fRec23: [0.0;2],
			fRec20: [0.0;3],
			fRec25: [0.0;2],
			fRec26: [0.0;2],
			fRec27: [0.0;2],
			fRec24: [0.0;3],
			fRec29: [0.0;2],
			fRec30: [0.0;2],
			fRec31: [0.0;2],
			fRec28: [0.0;3],
			fRec33: [0.0;2],
			fRec34: [0.0;2],
			fRec35: [0.0;2],
			fRec32: [0.0;3],
			fRec37: [0.0;2],
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fRec36: [0.0;3],
			fRec41: [0.0;2],
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fRec40: [0.0;3],
			fRec45: [0.0;2],
			fRec46: [0.0;2],
			fRec47: [0.0;2],
			fRec44: [0.0;3],
			fRec49: [0.0;2],
			fRec50: [0.0;2],
			fRec51: [0.0;2],
			fRec48: [0.0;3],
			fRec53: [0.0;2],
			fRec54: [0.0;2],
			fRec55: [0.0;2],
			fRec52: [0.0;3],
			fRec57: [0.0;2],
			fRec58: [0.0;2],
			fRec59: [0.0;2],
			fRec56: [0.0;3],
			fRec61: [0.0;2],
			fRec62: [0.0;2],
			fRec63: [0.0;2],
			fRec60: [0.0;3],
			fRec65: [0.0;2],
			fRec66: [0.0;2],
			fRec67: [0.0;2],
			fRec64: [0.0;3],
			fRec69: [0.0;2],
			fRec70: [0.0;2],
			fRec71: [0.0;2],
			fRec68: [0.0;3],
			fRec73: [0.0;2],
			fRec74: [0.0;2],
			fRec75: [0.0;2],
			fRec72: [0.0;3],
			fRec77: [0.0;2],
			fRec78: [0.0;2],
			fRec79: [0.0;2],
			fRec76: [0.0;3],
			fRec81: [0.0;2],
			fRec82: [0.0;2],
			fRec83: [0.0;2],
			fRec80: [0.0;3],
			fRec85: [0.0;2],
			fRec86: [0.0;2],
			fRec87: [0.0;2],
			fRec84: [0.0;3],
			fRec89: [0.0;2],
			fRec90: [0.0;2],
			fRec91: [0.0;2],
			fRec88: [0.0;3],
			fRec93: [0.0;2],
			fRec94: [0.0;2],
			fRec95: [0.0;2],
			fRec92: [0.0;3],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn BodyDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("drum.lib/name", r"abdrum-layers");
		m.declare("drum.lib/version", r"0.1");
		m.declare("filename", r"plugin-body.dsp");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"ABDrumBody");
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
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.4;
		self.fHslider2 = 0.4;
		self.fHslider3 = 1e+02;
		self.fHslider4 = 0.3;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec2[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec3[l3 as usize] = 0.0;
		}
		for l4 in 0..3 {
			self.fRec0[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec5[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec6[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec7[l7 as usize] = 0.0;
		}
		for l8 in 0..3 {
			self.fRec4[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec9[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec10[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec11[l11 as usize] = 0.0;
		}
		for l12 in 0..3 {
			self.fRec8[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec13[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec14[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec15[l15 as usize] = 0.0;
		}
		for l16 in 0..3 {
			self.fRec12[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec17[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec18[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec19[l19 as usize] = 0.0;
		}
		for l20 in 0..3 {
			self.fRec16[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec21[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec22[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec23[l23 as usize] = 0.0;
		}
		for l24 in 0..3 {
			self.fRec20[l24 as usize] = 0.0;
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
		for l28 in 0..3 {
			self.fRec24[l28 as usize] = 0.0;
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
		for l32 in 0..3 {
			self.fRec28[l32 as usize] = 0.0;
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
		for l36 in 0..3 {
			self.fRec32[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec37[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec38[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec39[l39 as usize] = 0.0;
		}
		for l40 in 0..3 {
			self.fRec36[l40 as usize] = 0.0;
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
		for l44 in 0..3 {
			self.fRec40[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec45[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec46[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec47[l47 as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fRec44[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec49[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec50[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec51[l51 as usize] = 0.0;
		}
		for l52 in 0..3 {
			self.fRec48[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec53[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec54[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec55[l55 as usize] = 0.0;
		}
		for l56 in 0..3 {
			self.fRec52[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec57[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec58[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec59[l59 as usize] = 0.0;
		}
		for l60 in 0..3 {
			self.fRec56[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec61[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec62[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec63[l63 as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fRec60[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec65[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec66[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec67[l67 as usize] = 0.0;
		}
		for l68 in 0..3 {
			self.fRec64[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec69[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec70[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec71[l71 as usize] = 0.0;
		}
		for l72 in 0..3 {
			self.fRec68[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec73[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec74[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec75[l75 as usize] = 0.0;
		}
		for l76 in 0..3 {
			self.fRec72[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec77[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec78[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec79[l79 as usize] = 0.0;
		}
		for l80 in 0..3 {
			self.fRec76[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec81[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec82[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec83[l83 as usize] = 0.0;
		}
		for l84 in 0..3 {
			self.fRec80[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec85[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec86[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec87[l87 as usize] = 0.0;
		}
		for l88 in 0..3 {
			self.fRec84[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec89[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec90[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec91[l91 as usize] = 0.0;
		}
		for l92 in 0..3 {
			self.fRec88[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec93[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec94[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec95[l95 as usize] = 0.0;
		}
		for l96 in 0..3 {
			self.fRec92[l96 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::exp(-(3333.3333 / fConst0));
		self.fConst2 = 6.907755 / fConst0;
		self.fConst3 = 0.45 * fConst0;
		self.fConst4 = 6.2831855 / fConst0;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		BodyDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("ABDrumBody");
		ui_interface.add_horizontal_slider("body_bright", ParamIndex(0), 0.4, 0.0, 1.0, 0.0001);
		ui_interface.add_button("body_gate", ParamIndex(1));
		ui_interface.add_horizontal_slider("body_hz", ParamIndex(2), 1e+02, 0.0, 1e+05, 0.001);
		ui_interface.add_horizontal_slider("body_material", ParamIndex(3), 0.0, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("body_position", ParamIndex(4), 0.3, 0.0, 1.0, 0.0001);
		ui_interface.add_horizontal_slider("body_t60", ParamIndex(5), 0.4, 0.0, 1e+02, 0.0001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			1 => Some(self.fButton0),
			3 => Some(self.fHslider0),
			0 => Some(self.fHslider1),
			5 => Some(self.fHslider2),
			2 => Some(self.fHslider3),
			4 => Some(self.fHslider4),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			1 => { self.fButton0 = value }
			3 => { self.fHslider0 = value }
			0 => { self.fHslider1 = value }
			5 => { self.fHslider2 = value }
			2 => { self.fHslider3 = value }
			4 => { self.fHslider4 = value }
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
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = F32::max(0.0, F32::min(1.0, self.fHslider0));
		let mut fSlow2: F32 = F32::max(0.0, 1.0 - F32::abs(-(2.0 * (1.0 - fSlow1))));
		let mut fSlow3: F32 = 2.0 * fSlow1;
		let mut fSlow4: F32 = F32::max(0.0, 1.0 - F32::abs(fSlow3 + -1.0));
		let mut fSlow5: F32 = F32::max(0.0, 1.0 - F32::abs(fSlow3));
		let mut fSlow6: F32 = F32::exp(1.7879188 * fSlow5 + 2.8011155 * fSlow4 + 5.578937 * fSlow2);
		let mut fSlow7: F32 = F32::max(0.0, F32::min(1.0, self.fHslider1)) - 1.0;
		let mut fSlow8: F32 = 1.2 * fSlow7;
		let mut fSlow9: F32 = self.fHslider2;
		let mut fSlow10: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow6, fSlow8))));
		let mut fSlow11: F32 = BodyDsp_faustpower2_f(fSlow10);
		let mut fSlow12: F32 = self.fHslider3;
		let mut fSlow13: F32 = fSlow12 * fSlow6;
		let mut fSlow14: F32 = self.fConst4 * F32::min(fSlow13, self.fConst3);
		let mut fSlow15: F32 = 2.0 * fSlow10 * F32::cos(fSlow14);
		let mut fSlow16: F32 = ((fSlow13 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow17: F32 = 1.5 * fSlow7;
		let mut fSlow18: F32 = F32::powf(fSlow6, fSlow17);
		let mut fSlow19: F32 = self.fHslider4;
		let mut fSlow20: F32 = 0.65 * F32::abs(F32::sin(75.398224 * fSlow19)) + 0.35;
		let mut fSlow21: F32 = F32::exp(1.7318325 * fSlow5 + 2.7995958 * fSlow4 + 5.495593 * fSlow2);
		let mut fSlow22: F32 = fSlow12 * fSlow21;
		let mut fSlow23: F32 = ((fSlow22 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow24: F32 = F32::powf(fSlow21, fSlow17);
		let mut fSlow25: F32 = 0.65 * F32::abs(F32::sin(72.25663 * fSlow19)) + 0.35;
		let mut fSlow26: F32 = F32::exp(1.7143383 * fSlow5 + 2.7725887 * fSlow4 + 5.4086237 * fSlow2);
		let mut fSlow27: F32 = fSlow12 * fSlow26;
		let mut fSlow28: F32 = ((fSlow27 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow29: F32 = F32::powf(fSlow26, fSlow17);
		let mut fSlow30: F32 = 0.65 * F32::abs(F32::sin(69.115036 * fSlow19)) + 0.35;
		let mut fSlow31: F32 = F32::exp(1.7119945 * fSlow5 + 2.6579492 * fSlow4 + 5.3176985 * fSlow2);
		let mut fSlow32: F32 = fSlow12 * fSlow31;
		let mut fSlow33: F32 = ((fSlow32 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow34: F32 = F32::powf(fSlow31, fSlow17);
		let mut fSlow35: F32 = 0.65 * F32::abs(F32::sin(65.97344 * fSlow19)) + 0.35;
		let mut fSlow36: F32 = F32::exp(1.6886187 * fSlow5 + 2.6436894 * fSlow4 + 5.2224402 * fSlow2);
		let mut fSlow37: F32 = fSlow12 * fSlow36;
		let mut fSlow38: F32 = ((fSlow37 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow39: F32 = F32::powf(fSlow36, fSlow17);
		let mut fSlow40: F32 = 0.65 * F32::abs(F32::sin(62.831852 * fSlow19)) + 0.35;
		let mut fSlow41: F32 = F32::exp(1.6353005 * fSlow5 + 2.5968952 * fSlow4 + 5.1224213 * fSlow2);
		let mut fSlow42: F32 = fSlow12 * fSlow41;
		let mut fSlow43: F32 = ((fSlow42 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow44: F32 = F32::powf(fSlow41, fSlow17);
		let mut fSlow45: F32 = 0.65 * F32::abs(F32::sin(59.69026 * fSlow19)) + 0.35;
		let mut fSlow46: F32 = F32::exp(1.6260984 * fSlow5 + 2.5529547 * fSlow4 + 5.017134 * fSlow2);
		let mut fSlow47: F32 = fSlow12 * fSlow46;
		let mut fSlow48: F32 = ((fSlow47 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow49: F32 = F32::powf(fSlow46, fSlow17);
		let mut fSlow50: F32 = 0.65 * F32::abs(F32::sin(56.548668 * fSlow19)) + 0.35;
		let mut fSlow51: F32 = F32::exp(1.5898472 * fSlow5 + 2.5402625 * fSlow4 + 4.905993 * fSlow2);
		let mut fSlow52: F32 = fSlow12 * fSlow51;
		let mut fSlow53: F32 = ((fSlow52 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow54: F32 = F32::powf(fSlow51, fSlow17);
		let mut fSlow55: F32 = 0.65 * F32::abs(F32::sin(53.407074 * fSlow19)) + 0.35;
		let mut fSlow56: F32 = F32::exp(1.5752605 * fSlow5 + 2.4726653 * fSlow4 + 4.7883162 * fSlow2);
		let mut fSlow57: F32 = fSlow12 * fSlow56;
		let mut fSlow58: F32 = ((fSlow57 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow59: F32 = F32::powf(fSlow56, fSlow17);
		let mut fSlow60: F32 = 0.65 * F32::abs(F32::sin(50.265484 * fSlow19)) + 0.35;
		let mut fSlow61: F32 = F32::exp(1.5282278 * fSlow5 + 2.3919687 * fSlow4 + 4.663269 * fSlow2);
		let mut fSlow62: F32 = fSlow12 * fSlow61;
		let mut fSlow63: F32 = ((fSlow62 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow64: F32 = F32::powf(fSlow61, fSlow17);
		let mut fSlow65: F32 = 0.65 * F32::abs(F32::sin(47.12389 * fSlow19)) + 0.35;
		let mut fSlow66: F32 = F32::exp(1.5262737 * fSlow5 + 2.367811 * fSlow4 + 4.5298862 * fSlow2);
		let mut fSlow67: F32 = fSlow12 * fSlow66;
		let mut fSlow68: F32 = ((fSlow67 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow69: F32 = F32::powf(fSlow66, fSlow17);
		let mut fSlow70: F32 = 0.65 * F32::abs(F32::sin(43.982296 * fSlow19)) + 0.35;
		let mut fSlow71: F32 = F32::exp(1.442202 * fSlow5 + 2.2827904 * fSlow4 + 4.3869767 * fSlow2);
		let mut fSlow72: F32 = fSlow12 * fSlow71;
		let mut fSlow73: F32 = ((fSlow72 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow74: F32 = F32::powf(fSlow71, fSlow17);
		let mut fSlow75: F32 = 0.65 * F32::abs(F32::sin(40.840706 * fSlow19)) + 0.35;
		let mut fSlow76: F32 = F32::exp(1.4187615 * fSlow5 + 2.1972246 * fSlow4 + 4.233048 * fSlow2);
		let mut fSlow77: F32 = fSlow12 * fSlow76;
		let mut fSlow78: F32 = ((fSlow77 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow79: F32 = F32::powf(fSlow76, fSlow17);
		let mut fSlow80: F32 = 0.65 * F32::abs(F32::sin(37.699112 * fSlow19)) + 0.35;
		let mut fSlow81: F32 = F32::exp(1.4009366 * fSlow5 + 2.032481 * fSlow4 + 4.066288 * fSlow2);
		let mut fSlow82: F32 = fSlow12 * fSlow81;
		let mut fSlow83: F32 = ((fSlow82 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow84: F32 = F32::powf(fSlow81, fSlow17);
		let mut fSlow85: F32 = 0.65 * F32::abs(F32::sin(34.557518 * fSlow19)) + 0.35;
		let mut fSlow86: F32 = F32::exp(1.2939049 * fSlow5 + 2.0305138 * fSlow4 + 3.8843434 * fSlow2);
		let mut fSlow87: F32 = fSlow12 * fSlow86;
		let mut fSlow88: F32 = ((fSlow87 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow89: F32 = F32::powf(fSlow86, fSlow17);
		let mut fSlow90: F32 = 0.65 * F32::abs(F32::sin(31.415926 * fSlow19)) + 0.35;
		let mut fSlow91: F32 = F32::exp(1.2803781 * fSlow5 + 1.9902004 * fSlow4 + 3.6841683 * fSlow2);
		let mut fSlow92: F32 = fSlow12 * fSlow91;
		let mut fSlow93: F32 = ((fSlow92 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow94: F32 = F32::powf(fSlow91, fSlow17);
		let mut fSlow95: F32 = 0.65 * F32::abs(F32::sin(28.274334 * fSlow19)) + 0.35;
		let mut fSlow96: F32 = F32::exp(1.2527629 * fSlow5 + 1.9153037 * fSlow4 + 3.4617279 * fSlow2);
		let mut fSlow97: F32 = fSlow12 * fSlow96;
		let mut fSlow98: F32 = ((fSlow97 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow99: F32 = F32::powf(fSlow96, fSlow17);
		let mut fSlow100: F32 = 0.65 * F32::abs(F32::sin(25.132742 * fSlow19)) + 0.35;
		let mut fSlow101: F32 = F32::exp(1.1489885 * fSlow5 + 1.68306 * fSlow4 + 3.211408 * fSlow2);
		let mut fSlow102: F32 = fSlow12 * fSlow101;
		let mut fSlow103: F32 = ((fSlow102 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow104: F32 = F32::powf(fSlow101, fSlow17);
		let mut fSlow105: F32 = 0.65 * F32::abs(F32::sin(21.991148 * fSlow19)) + 0.35;
		let mut fSlow106: F32 = F32::exp(1.0705557 * fSlow5 + 1.6382197 * fSlow4 + 2.9252026 * fSlow2);
		let mut fSlow107: F32 = fSlow12 * fSlow106;
		let mut fSlow108: F32 = ((fSlow107 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow109: F32 = F32::powf(fSlow106, fSlow17);
		let mut fSlow110: F32 = 0.65 * F32::abs(F32::sin(18.849556 * fSlow19)) + 0.35;
		let mut fSlow111: F32 = F32::exp(0.9756911 * fSlow5 + 1.3862944 * fSlow4 + 2.5910668 * fSlow2);
		let mut fSlow112: F32 = fSlow12 * fSlow111;
		let mut fSlow113: F32 = ((fSlow112 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow114: F32 = F32::powf(fSlow111, fSlow17);
		let mut fSlow115: F32 = 0.65 * F32::abs(F32::sin(15.707963 * fSlow19)) + 0.35;
		let mut fSlow116: F32 = F32::exp(0.8307328 * fSlow5 + 1.1665825 * fSlow4 + 2.1897523 * fSlow2);
		let mut fSlow117: F32 = fSlow12 * fSlow116;
		let mut fSlow118: F32 = ((fSlow117 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow119: F32 = F32::powf(fSlow116, fSlow17);
		let mut fSlow120: F32 = 0.65 * F32::abs(F32::sin(12.566371 * fSlow19)) + 0.35;
		let mut fSlow121: F32 = F32::exp(0.7589349 * fSlow5 + 1.154047 * fSlow4 + 1.6871394 * fSlow2);
		let mut fSlow122: F32 = fSlow12 * fSlow121;
		let mut fSlow123: F32 = ((fSlow122 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow124: F32 = F32::powf(fSlow121, fSlow17);
		let mut fSlow125: F32 = 0.65 * F32::abs(F32::sin(9.424778 * fSlow19)) + 0.35;
		let mut fSlow126: F32 = F32::exp(0.46561903 * fSlow5 + 0.60376936 * fSlow4 + 1.0141431 * fSlow2);
		let mut fSlow127: F32 = fSlow12 * fSlow126;
		let mut fSlow128: F32 = ((fSlow127 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow129: F32 = F32::powf(fSlow126, fSlow17);
		let mut fSlow130: F32 = 0.65 * F32::abs(F32::sin(6.2831855 * fSlow19)) + 0.35;
		let mut fSlow131: F32 = ((fSlow12 < self.fConst3) as i32) as u32 as F32;
		let mut fSlow132: F32 = F32::powf(1.0, fSlow17);
		let mut fSlow133: F32 = 0.65 * F32::abs(F32::sin(3.1415927 * fSlow19)) + 0.35;
		let mut fSlow134: F32 = F32::max(0.1, F32::sqrt(BodyDsp_faustpower2_f(fSlow133) * BodyDsp_faustpower2_f(fSlow132) * BodyDsp_faustpower2_f(fSlow131) + BodyDsp_faustpower2_f(fSlow130) * BodyDsp_faustpower2_f(fSlow129) * BodyDsp_faustpower2_f(fSlow128) + BodyDsp_faustpower2_f(fSlow125) * BodyDsp_faustpower2_f(fSlow124) * BodyDsp_faustpower2_f(fSlow123) + BodyDsp_faustpower2_f(fSlow120) * BodyDsp_faustpower2_f(fSlow119) * BodyDsp_faustpower2_f(fSlow118) + BodyDsp_faustpower2_f(fSlow115) * BodyDsp_faustpower2_f(fSlow114) * BodyDsp_faustpower2_f(fSlow113) + BodyDsp_faustpower2_f(fSlow110) * BodyDsp_faustpower2_f(fSlow109) * BodyDsp_faustpower2_f(fSlow108) + BodyDsp_faustpower2_f(fSlow105) * BodyDsp_faustpower2_f(fSlow104) * BodyDsp_faustpower2_f(fSlow103) + BodyDsp_faustpower2_f(fSlow100) * BodyDsp_faustpower2_f(fSlow99) * BodyDsp_faustpower2_f(fSlow98) + BodyDsp_faustpower2_f(fSlow95) * BodyDsp_faustpower2_f(fSlow94) * BodyDsp_faustpower2_f(fSlow93) + BodyDsp_faustpower2_f(fSlow90) * BodyDsp_faustpower2_f(fSlow89) * BodyDsp_faustpower2_f(fSlow88) + BodyDsp_faustpower2_f(fSlow85) * BodyDsp_faustpower2_f(fSlow84) * BodyDsp_faustpower2_f(fSlow83) + BodyDsp_faustpower2_f(fSlow80) * BodyDsp_faustpower2_f(fSlow79) * BodyDsp_faustpower2_f(fSlow78) + BodyDsp_faustpower2_f(fSlow75) * BodyDsp_faustpower2_f(fSlow74) * BodyDsp_faustpower2_f(fSlow73) + BodyDsp_faustpower2_f(fSlow70) * BodyDsp_faustpower2_f(fSlow69) * BodyDsp_faustpower2_f(fSlow68) + BodyDsp_faustpower2_f(fSlow65) * BodyDsp_faustpower2_f(fSlow64) * BodyDsp_faustpower2_f(fSlow63) + BodyDsp_faustpower2_f(fSlow60) * BodyDsp_faustpower2_f(fSlow59) * BodyDsp_faustpower2_f(fSlow58) + BodyDsp_faustpower2_f(fSlow55) * BodyDsp_faustpower2_f(fSlow54) * BodyDsp_faustpower2_f(fSlow53) + BodyDsp_faustpower2_f(fSlow50) * BodyDsp_faustpower2_f(fSlow49) * BodyDsp_faustpower2_f(fSlow48) + BodyDsp_faustpower2_f(fSlow45) * BodyDsp_faustpower2_f(fSlow44) * BodyDsp_faustpower2_f(fSlow43) + BodyDsp_faustpower2_f(fSlow40) * BodyDsp_faustpower2_f(fSlow39) * BodyDsp_faustpower2_f(fSlow38) + BodyDsp_faustpower2_f(fSlow35) * BodyDsp_faustpower2_f(fSlow34) * BodyDsp_faustpower2_f(fSlow33) + BodyDsp_faustpower2_f(fSlow30) * BodyDsp_faustpower2_f(fSlow29) * BodyDsp_faustpower2_f(fSlow28) + BodyDsp_faustpower2_f(fSlow25) * BodyDsp_faustpower2_f(fSlow24) * BodyDsp_faustpower2_f(fSlow23) + BodyDsp_faustpower2_f(fSlow20) * BodyDsp_faustpower2_f(fSlow18) * BodyDsp_faustpower2_f(fSlow16)));
		let mut fSlow135: F32 = 0.3 * (fSlow20 * fSlow18 * fSlow16 * F32::sin(fSlow14) / fSlow134);
		let mut fSlow136: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow21, fSlow8))));
		let mut fSlow137: F32 = BodyDsp_faustpower2_f(fSlow136);
		let mut fSlow138: F32 = self.fConst4 * F32::min(fSlow22, self.fConst3);
		let mut fSlow139: F32 = 2.0 * fSlow136 * F32::cos(fSlow138);
		let mut fSlow140: F32 = 0.3 * (fSlow25 * fSlow24 * fSlow23 * F32::sin(fSlow138) / fSlow134);
		let mut fSlow141: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow26, fSlow8))));
		let mut fSlow142: F32 = BodyDsp_faustpower2_f(fSlow141);
		let mut fSlow143: F32 = self.fConst4 * F32::min(fSlow27, self.fConst3);
		let mut fSlow144: F32 = 2.0 * fSlow141 * F32::cos(fSlow143);
		let mut fSlow145: F32 = 0.3 * (fSlow30 * fSlow29 * fSlow28 * F32::sin(fSlow143) / fSlow134);
		let mut fSlow146: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow31, fSlow8))));
		let mut fSlow147: F32 = BodyDsp_faustpower2_f(fSlow146);
		let mut fSlow148: F32 = self.fConst4 * F32::min(fSlow32, self.fConst3);
		let mut fSlow149: F32 = 2.0 * fSlow146 * F32::cos(fSlow148);
		let mut fSlow150: F32 = 0.3 * (fSlow35 * fSlow34 * fSlow33 * F32::sin(fSlow148) / fSlow134);
		let mut fSlow151: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow36, fSlow8))));
		let mut fSlow152: F32 = BodyDsp_faustpower2_f(fSlow151);
		let mut fSlow153: F32 = self.fConst4 * F32::min(fSlow37, self.fConst3);
		let mut fSlow154: F32 = 2.0 * fSlow151 * F32::cos(fSlow153);
		let mut fSlow155: F32 = 0.3 * (fSlow40 * fSlow39 * fSlow38 * F32::sin(fSlow153) / fSlow134);
		let mut fSlow156: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow41, fSlow8))));
		let mut fSlow157: F32 = BodyDsp_faustpower2_f(fSlow156);
		let mut fSlow158: F32 = self.fConst4 * F32::min(fSlow42, self.fConst3);
		let mut fSlow159: F32 = 2.0 * fSlow156 * F32::cos(fSlow158);
		let mut fSlow160: F32 = 0.3 * (fSlow45 * fSlow44 * fSlow43 * F32::sin(fSlow158) / fSlow134);
		let mut fSlow161: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow46, fSlow8))));
		let mut fSlow162: F32 = BodyDsp_faustpower2_f(fSlow161);
		let mut fSlow163: F32 = self.fConst4 * F32::min(fSlow47, self.fConst3);
		let mut fSlow164: F32 = 2.0 * fSlow161 * F32::cos(fSlow163);
		let mut fSlow165: F32 = 0.3 * (fSlow50 * fSlow49 * fSlow48 * F32::sin(fSlow163) / fSlow134);
		let mut fSlow166: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow51, fSlow8))));
		let mut fSlow167: F32 = BodyDsp_faustpower2_f(fSlow166);
		let mut fSlow168: F32 = self.fConst4 * F32::min(fSlow52, self.fConst3);
		let mut fSlow169: F32 = 2.0 * fSlow166 * F32::cos(fSlow168);
		let mut fSlow170: F32 = 0.3 * (fSlow55 * fSlow54 * fSlow53 * F32::sin(fSlow168) / fSlow134);
		let mut fSlow171: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow56, fSlow8))));
		let mut fSlow172: F32 = BodyDsp_faustpower2_f(fSlow171);
		let mut fSlow173: F32 = self.fConst4 * F32::min(fSlow57, self.fConst3);
		let mut fSlow174: F32 = 2.0 * fSlow171 * F32::cos(fSlow173);
		let mut fSlow175: F32 = 0.3 * (fSlow60 * fSlow59 * fSlow58 * F32::sin(fSlow173) / fSlow134);
		let mut fSlow176: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow61, fSlow8))));
		let mut fSlow177: F32 = BodyDsp_faustpower2_f(fSlow176);
		let mut fSlow178: F32 = self.fConst4 * F32::min(fSlow62, self.fConst3);
		let mut fSlow179: F32 = 2.0 * fSlow176 * F32::cos(fSlow178);
		let mut fSlow180: F32 = 0.3 * (fSlow65 * fSlow64 * fSlow63 * F32::sin(fSlow178) / fSlow134);
		let mut fSlow181: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow66, fSlow8))));
		let mut fSlow182: F32 = BodyDsp_faustpower2_f(fSlow181);
		let mut fSlow183: F32 = self.fConst4 * F32::min(fSlow67, self.fConst3);
		let mut fSlow184: F32 = 2.0 * fSlow181 * F32::cos(fSlow183);
		let mut fSlow185: F32 = 0.3 * (fSlow70 * fSlow69 * fSlow68 * F32::sin(fSlow183) / fSlow134);
		let mut fSlow186: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow71, fSlow8))));
		let mut fSlow187: F32 = BodyDsp_faustpower2_f(fSlow186);
		let mut fSlow188: F32 = self.fConst4 * F32::min(fSlow72, self.fConst3);
		let mut fSlow189: F32 = 2.0 * fSlow186 * F32::cos(fSlow188);
		let mut fSlow190: F32 = 0.3 * (fSlow75 * fSlow74 * fSlow73 * F32::sin(fSlow188) / fSlow134);
		let mut fSlow191: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow76, fSlow8))));
		let mut fSlow192: F32 = BodyDsp_faustpower2_f(fSlow191);
		let mut fSlow193: F32 = self.fConst4 * F32::min(fSlow77, self.fConst3);
		let mut fSlow194: F32 = 2.0 * fSlow191 * F32::cos(fSlow193);
		let mut fSlow195: F32 = 0.3 * (fSlow80 * fSlow79 * fSlow78 * F32::sin(fSlow193) / fSlow134);
		let mut fSlow196: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow81, fSlow8))));
		let mut fSlow197: F32 = BodyDsp_faustpower2_f(fSlow196);
		let mut fSlow198: F32 = self.fConst4 * F32::min(fSlow82, self.fConst3);
		let mut fSlow199: F32 = 2.0 * fSlow196 * F32::cos(fSlow198);
		let mut fSlow200: F32 = 0.3 * (fSlow85 * fSlow84 * fSlow83 * F32::sin(fSlow198) / fSlow134);
		let mut fSlow201: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow86, fSlow8))));
		let mut fSlow202: F32 = BodyDsp_faustpower2_f(fSlow201);
		let mut fSlow203: F32 = self.fConst4 * F32::min(fSlow87, self.fConst3);
		let mut fSlow204: F32 = 2.0 * fSlow201 * F32::cos(fSlow203);
		let mut fSlow205: F32 = 0.3 * (fSlow90 * fSlow89 * fSlow88 * F32::sin(fSlow203) / fSlow134);
		let mut fSlow206: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow91, fSlow8))));
		let mut fSlow207: F32 = BodyDsp_faustpower2_f(fSlow206);
		let mut fSlow208: F32 = self.fConst4 * F32::min(fSlow92, self.fConst3);
		let mut fSlow209: F32 = 2.0 * fSlow206 * F32::cos(fSlow208);
		let mut fSlow210: F32 = 0.3 * (fSlow95 * fSlow94 * fSlow93 * F32::sin(fSlow208) / fSlow134);
		let mut fSlow211: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow96, fSlow8))));
		let mut fSlow212: F32 = BodyDsp_faustpower2_f(fSlow211);
		let mut fSlow213: F32 = self.fConst4 * F32::min(fSlow97, self.fConst3);
		let mut fSlow214: F32 = 2.0 * fSlow211 * F32::cos(fSlow213);
		let mut fSlow215: F32 = 0.3 * (fSlow100 * fSlow99 * fSlow98 * F32::sin(fSlow213) / fSlow134);
		let mut fSlow216: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow101, fSlow8))));
		let mut fSlow217: F32 = BodyDsp_faustpower2_f(fSlow216);
		let mut fSlow218: F32 = self.fConst4 * F32::min(fSlow102, self.fConst3);
		let mut fSlow219: F32 = 2.0 * fSlow216 * F32::cos(fSlow218);
		let mut fSlow220: F32 = 0.3 * (fSlow105 * fSlow104 * fSlow103 * F32::sin(fSlow218) / fSlow134);
		let mut fSlow221: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow106, fSlow8))));
		let mut fSlow222: F32 = BodyDsp_faustpower2_f(fSlow221);
		let mut fSlow223: F32 = self.fConst4 * F32::min(fSlow107, self.fConst3);
		let mut fSlow224: F32 = 2.0 * fSlow221 * F32::cos(fSlow223);
		let mut fSlow225: F32 = 0.3 * (fSlow110 * fSlow109 * fSlow108 * F32::sin(fSlow223) / fSlow134);
		let mut fSlow226: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow111, fSlow8))));
		let mut fSlow227: F32 = BodyDsp_faustpower2_f(fSlow226);
		let mut fSlow228: F32 = self.fConst4 * F32::min(fSlow112, self.fConst3);
		let mut fSlow229: F32 = 2.0 * fSlow226 * F32::cos(fSlow228);
		let mut fSlow230: F32 = 0.3 * (fSlow115 * fSlow114 * fSlow113 * F32::sin(fSlow228) / fSlow134);
		let mut fSlow231: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow116, fSlow8))));
		let mut fSlow232: F32 = BodyDsp_faustpower2_f(fSlow231);
		let mut fSlow233: F32 = self.fConst4 * F32::min(fSlow117, self.fConst3);
		let mut fSlow234: F32 = 2.0 * fSlow231 * F32::cos(fSlow233);
		let mut fSlow235: F32 = 0.3 * (fSlow120 * fSlow119 * fSlow118 * F32::sin(fSlow233) / fSlow134);
		let mut fSlow236: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow121, fSlow8))));
		let mut fSlow237: F32 = BodyDsp_faustpower2_f(fSlow236);
		let mut fSlow238: F32 = self.fConst4 * F32::min(fSlow122, self.fConst3);
		let mut fSlow239: F32 = 2.0 * fSlow236 * F32::cos(fSlow238);
		let mut fSlow240: F32 = 0.3 * (fSlow125 * fSlow124 * fSlow123 * F32::sin(fSlow238) / fSlow134);
		let mut fSlow241: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(fSlow126, fSlow8))));
		let mut fSlow242: F32 = BodyDsp_faustpower2_f(fSlow241);
		let mut fSlow243: F32 = self.fConst4 * F32::min(fSlow127, self.fConst3);
		let mut fSlow244: F32 = 2.0 * fSlow241 * F32::cos(fSlow243);
		let mut fSlow245: F32 = 0.3 * (fSlow130 * fSlow129 * fSlow128 * F32::sin(fSlow243) / fSlow134);
		let mut fSlow246: F32 = F32::exp(-(self.fConst2 / F32::max(0.001, fSlow9 * F32::powf(1.0, fSlow8))));
		let mut fSlow247: F32 = BodyDsp_faustpower2_f(fSlow246);
		let mut fSlow248: F32 = self.fConst4 * F32::min(fSlow12, self.fConst3);
		let mut fSlow249: F32 = 2.0 * fSlow246 * F32::cos(fSlow248);
		let mut fSlow250: F32 = 0.3 * (fSlow133 * F32::sin(fSlow248) * fSlow132 * fSlow131 / fSlow134);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0);
		for ((input0, input1), output0) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			let mut fTemp0: F32 = (i32::wrapping_sub(1, (fSlow0 > self.fVec0[1]) as i32)) as F32;
			let mut fTemp1: F32 = 1.0 - self.fConst1 * fTemp0;
			self.fRec1[0] = fSlow11 * fTemp1 + self.fConst1 * fTemp0 * self.fRec1[1];
			self.fRec2[0] = fSlow15 * fTemp1 + self.fConst1 * fTemp0 * self.fRec2[1];
			self.fRec3[0] = fSlow135 * fTemp1 + self.fConst1 * fTemp0 * self.fRec3[1];
			let mut fTemp2: F32 = *input0;
			self.fRec0[0] = fTemp2 * self.fRec3[0] + self.fRec2[0] * self.fRec0[1] - self.fRec1[0] * self.fRec0[2];
			self.fRec5[0] = fSlow137 * fTemp1 + self.fConst1 * fTemp0 * self.fRec5[1];
			self.fRec6[0] = fSlow139 * fTemp1 + self.fConst1 * fTemp0 * self.fRec6[1];
			self.fRec7[0] = fSlow140 * fTemp1 + self.fConst1 * fTemp0 * self.fRec7[1];
			self.fRec4[0] = fTemp2 * self.fRec7[0] + self.fRec6[0] * self.fRec4[1] - self.fRec5[0] * self.fRec4[2];
			self.fRec9[0] = fSlow142 * fTemp1 + self.fConst1 * fTemp0 * self.fRec9[1];
			self.fRec10[0] = fSlow144 * fTemp1 + self.fConst1 * fTemp0 * self.fRec10[1];
			self.fRec11[0] = fSlow145 * fTemp1 + self.fConst1 * fTemp0 * self.fRec11[1];
			self.fRec8[0] = fTemp2 * self.fRec11[0] + self.fRec10[0] * self.fRec8[1] - self.fRec9[0] * self.fRec8[2];
			self.fRec13[0] = fSlow147 * fTemp1 + self.fConst1 * fTemp0 * self.fRec13[1];
			self.fRec14[0] = fSlow149 * fTemp1 + self.fConst1 * fTemp0 * self.fRec14[1];
			self.fRec15[0] = fSlow150 * fTemp1 + self.fConst1 * fTemp0 * self.fRec15[1];
			self.fRec12[0] = fTemp2 * self.fRec15[0] + self.fRec14[0] * self.fRec12[1] - self.fRec13[0] * self.fRec12[2];
			self.fRec17[0] = fSlow152 * fTemp1 + self.fConst1 * fTemp0 * self.fRec17[1];
			self.fRec18[0] = fSlow154 * fTemp1 + self.fConst1 * fTemp0 * self.fRec18[1];
			self.fRec19[0] = fSlow155 * fTemp1 + self.fConst1 * fTemp0 * self.fRec19[1];
			self.fRec16[0] = fTemp2 * self.fRec19[0] + self.fRec18[0] * self.fRec16[1] - self.fRec17[0] * self.fRec16[2];
			self.fRec21[0] = fSlow157 * fTemp1 + self.fConst1 * fTemp0 * self.fRec21[1];
			self.fRec22[0] = fSlow159 * fTemp1 + self.fConst1 * fTemp0 * self.fRec22[1];
			self.fRec23[0] = fSlow160 * fTemp1 + self.fConst1 * fTemp0 * self.fRec23[1];
			self.fRec20[0] = fTemp2 * self.fRec23[0] + self.fRec22[0] * self.fRec20[1] - self.fRec21[0] * self.fRec20[2];
			self.fRec25[0] = fSlow162 * fTemp1 + self.fConst1 * fTemp0 * self.fRec25[1];
			self.fRec26[0] = fSlow164 * fTemp1 + self.fConst1 * fTemp0 * self.fRec26[1];
			self.fRec27[0] = fSlow165 * fTemp1 + self.fConst1 * fTemp0 * self.fRec27[1];
			self.fRec24[0] = fTemp2 * self.fRec27[0] + self.fRec26[0] * self.fRec24[1] - self.fRec25[0] * self.fRec24[2];
			self.fRec29[0] = fSlow167 * fTemp1 + self.fConst1 * fTemp0 * self.fRec29[1];
			self.fRec30[0] = fSlow169 * fTemp1 + self.fConst1 * fTemp0 * self.fRec30[1];
			self.fRec31[0] = fSlow170 * fTemp1 + self.fConst1 * fTemp0 * self.fRec31[1];
			self.fRec28[0] = fTemp2 * self.fRec31[0] + self.fRec30[0] * self.fRec28[1] - self.fRec29[0] * self.fRec28[2];
			self.fRec33[0] = fSlow172 * fTemp1 + self.fConst1 * fTemp0 * self.fRec33[1];
			self.fRec34[0] = fSlow174 * fTemp1 + self.fConst1 * fTemp0 * self.fRec34[1];
			self.fRec35[0] = fSlow175 * fTemp1 + self.fConst1 * fTemp0 * self.fRec35[1];
			self.fRec32[0] = fTemp2 * self.fRec35[0] + self.fRec34[0] * self.fRec32[1] - self.fRec33[0] * self.fRec32[2];
			self.fRec37[0] = fSlow177 * fTemp1 + self.fConst1 * fTemp0 * self.fRec37[1];
			self.fRec38[0] = fSlow179 * fTemp1 + self.fConst1 * fTemp0 * self.fRec38[1];
			self.fRec39[0] = fSlow180 * fTemp1 + self.fConst1 * fTemp0 * self.fRec39[1];
			self.fRec36[0] = fTemp2 * self.fRec39[0] + self.fRec38[0] * self.fRec36[1] - self.fRec37[0] * self.fRec36[2];
			self.fRec41[0] = fSlow182 * fTemp1 + self.fConst1 * fTemp0 * self.fRec41[1];
			self.fRec42[0] = fSlow184 * fTemp1 + self.fConst1 * fTemp0 * self.fRec42[1];
			self.fRec43[0] = fSlow185 * fTemp1 + self.fConst1 * fTemp0 * self.fRec43[1];
			self.fRec40[0] = fTemp2 * self.fRec43[0] + self.fRec42[0] * self.fRec40[1] - self.fRec41[0] * self.fRec40[2];
			self.fRec45[0] = fSlow187 * fTemp1 + self.fConst1 * fTemp0 * self.fRec45[1];
			self.fRec46[0] = fSlow189 * fTemp1 + self.fConst1 * fTemp0 * self.fRec46[1];
			self.fRec47[0] = fSlow190 * fTemp1 + self.fConst1 * fTemp0 * self.fRec47[1];
			self.fRec44[0] = fTemp2 * self.fRec47[0] + self.fRec46[0] * self.fRec44[1] - self.fRec45[0] * self.fRec44[2];
			self.fRec49[0] = fSlow192 * fTemp1 + self.fConst1 * fTemp0 * self.fRec49[1];
			self.fRec50[0] = fSlow194 * fTemp1 + self.fConst1 * fTemp0 * self.fRec50[1];
			self.fRec51[0] = fSlow195 * fTemp1 + self.fConst1 * fTemp0 * self.fRec51[1];
			self.fRec48[0] = fTemp2 * self.fRec51[0] + self.fRec50[0] * self.fRec48[1] - self.fRec49[0] * self.fRec48[2];
			self.fRec53[0] = fSlow197 * fTemp1 + self.fConst1 * fTemp0 * self.fRec53[1];
			self.fRec54[0] = fSlow199 * fTemp1 + self.fConst1 * fTemp0 * self.fRec54[1];
			self.fRec55[0] = fSlow200 * fTemp1 + self.fConst1 * fTemp0 * self.fRec55[1];
			self.fRec52[0] = fTemp2 * self.fRec55[0] + self.fRec54[0] * self.fRec52[1] - self.fRec53[0] * self.fRec52[2];
			self.fRec57[0] = fSlow202 * fTemp1 + self.fConst1 * fTemp0 * self.fRec57[1];
			self.fRec58[0] = fSlow204 * fTemp1 + self.fConst1 * fTemp0 * self.fRec58[1];
			self.fRec59[0] = fSlow205 * fTemp1 + self.fConst1 * fTemp0 * self.fRec59[1];
			self.fRec56[0] = fTemp2 * self.fRec59[0] + self.fRec58[0] * self.fRec56[1] - self.fRec57[0] * self.fRec56[2];
			self.fRec61[0] = fSlow207 * fTemp1 + self.fConst1 * fTemp0 * self.fRec61[1];
			self.fRec62[0] = fSlow209 * fTemp1 + self.fConst1 * fTemp0 * self.fRec62[1];
			self.fRec63[0] = fSlow210 * fTemp1 + self.fConst1 * fTemp0 * self.fRec63[1];
			self.fRec60[0] = fTemp2 * self.fRec63[0] + self.fRec62[0] * self.fRec60[1] - self.fRec61[0] * self.fRec60[2];
			self.fRec65[0] = fSlow212 * fTemp1 + self.fConst1 * fTemp0 * self.fRec65[1];
			self.fRec66[0] = fSlow214 * fTemp1 + self.fConst1 * fTemp0 * self.fRec66[1];
			self.fRec67[0] = fSlow215 * fTemp1 + self.fConst1 * fTemp0 * self.fRec67[1];
			self.fRec64[0] = fTemp2 * self.fRec67[0] + self.fRec66[0] * self.fRec64[1] - self.fRec65[0] * self.fRec64[2];
			self.fRec69[0] = fSlow217 * fTemp1 + self.fConst1 * fTemp0 * self.fRec69[1];
			self.fRec70[0] = fSlow219 * fTemp1 + self.fConst1 * fTemp0 * self.fRec70[1];
			self.fRec71[0] = fSlow220 * fTemp1 + self.fConst1 * fTemp0 * self.fRec71[1];
			self.fRec68[0] = fTemp2 * self.fRec71[0] + self.fRec70[0] * self.fRec68[1] - self.fRec69[0] * self.fRec68[2];
			self.fRec73[0] = fSlow222 * fTemp1 + self.fConst1 * fTemp0 * self.fRec73[1];
			self.fRec74[0] = fSlow224 * fTemp1 + self.fConst1 * fTemp0 * self.fRec74[1];
			self.fRec75[0] = fSlow225 * fTemp1 + self.fConst1 * fTemp0 * self.fRec75[1];
			self.fRec72[0] = fTemp2 * self.fRec75[0] + self.fRec74[0] * self.fRec72[1] - self.fRec73[0] * self.fRec72[2];
			self.fRec77[0] = fSlow227 * fTemp1 + self.fConst1 * fTemp0 * self.fRec77[1];
			self.fRec78[0] = fSlow229 * fTemp1 + self.fConst1 * fTemp0 * self.fRec78[1];
			self.fRec79[0] = fSlow230 * fTemp1 + self.fConst1 * fTemp0 * self.fRec79[1];
			self.fRec76[0] = fTemp2 * self.fRec79[0] + self.fRec78[0] * self.fRec76[1] - self.fRec77[0] * self.fRec76[2];
			self.fRec81[0] = fSlow232 * fTemp1 + self.fConst1 * fTemp0 * self.fRec81[1];
			self.fRec82[0] = fSlow234 * fTemp1 + self.fConst1 * fTemp0 * self.fRec82[1];
			self.fRec83[0] = fSlow235 * fTemp1 + self.fConst1 * fTemp0 * self.fRec83[1];
			self.fRec80[0] = fTemp2 * self.fRec83[0] + self.fRec82[0] * self.fRec80[1] - self.fRec81[0] * self.fRec80[2];
			self.fRec85[0] = fSlow237 * fTemp1 + self.fConst1 * fTemp0 * self.fRec85[1];
			self.fRec86[0] = fSlow239 * fTemp1 + self.fConst1 * fTemp0 * self.fRec86[1];
			self.fRec87[0] = fSlow240 * fTemp1 + self.fConst1 * fTemp0 * self.fRec87[1];
			self.fRec84[0] = fTemp2 * self.fRec87[0] + self.fRec86[0] * self.fRec84[1] - self.fRec85[0] * self.fRec84[2];
			self.fRec89[0] = fSlow242 * fTemp1 + self.fConst1 * fTemp0 * self.fRec89[1];
			self.fRec90[0] = fSlow244 * fTemp1 + self.fConst1 * fTemp0 * self.fRec90[1];
			self.fRec91[0] = fSlow245 * fTemp1 + self.fConst1 * fTemp0 * self.fRec91[1];
			self.fRec88[0] = fTemp2 * self.fRec91[0] + self.fRec90[0] * self.fRec88[1] - self.fRec89[0] * self.fRec88[2];
			self.fRec93[0] = fSlow247 * fTemp1 + self.fConst1 * fTemp0 * self.fRec93[1];
			self.fRec94[0] = fSlow249 * fTemp1 + self.fConst1 * fTemp0 * self.fRec94[1];
			self.fRec95[0] = fSlow250 * fTemp1 + self.fConst1 * fTemp0 * self.fRec95[1];
			self.fRec92[0] = fTemp2 * self.fRec95[0] + self.fRec94[0] * self.fRec92[1] - self.fRec93[0] * self.fRec92[2];
			*output0 = *input1 * (self.fRec92[0] + self.fRec88[0] + self.fRec84[0] + self.fRec80[0] + self.fRec76[0] + self.fRec72[0] + self.fRec68[0] + self.fRec64[0] + self.fRec60[0] + self.fRec56[0] + self.fRec52[0] + self.fRec48[0] + self.fRec44[0] + self.fRec40[0] + self.fRec36[0] + self.fRec32[0] + self.fRec28[0] + self.fRec24[0] + self.fRec20[0] + self.fRec16[0] + self.fRec12[0] + self.fRec8[0] + self.fRec4[0] + self.fRec0[0]);
			self.fVec0[1] = self.fVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec0[2] = self.fRec0[1];
			self.fRec0[1] = self.fRec0[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec8[2] = self.fRec8[1];
			self.fRec8[1] = self.fRec8[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec16[2] = self.fRec16[1];
			self.fRec16[1] = self.fRec16[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec20[2] = self.fRec20[1];
			self.fRec20[1] = self.fRec20[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec24[2] = self.fRec24[1];
			self.fRec24[1] = self.fRec24[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec28[2] = self.fRec28[1];
			self.fRec28[1] = self.fRec28[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec32[2] = self.fRec32[1];
			self.fRec32[1] = self.fRec32[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec36[2] = self.fRec36[1];
			self.fRec36[1] = self.fRec36[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec40[2] = self.fRec40[1];
			self.fRec40[1] = self.fRec40[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec44[2] = self.fRec44[1];
			self.fRec44[1] = self.fRec44[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec48[2] = self.fRec48[1];
			self.fRec48[1] = self.fRec48[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec52[2] = self.fRec52[1];
			self.fRec52[1] = self.fRec52[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec56[2] = self.fRec56[1];
			self.fRec56[1] = self.fRec56[0];
			self.fRec61[1] = self.fRec61[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec60[2] = self.fRec60[1];
			self.fRec60[1] = self.fRec60[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec64[2] = self.fRec64[1];
			self.fRec64[1] = self.fRec64[0];
			self.fRec69[1] = self.fRec69[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec68[2] = self.fRec68[1];
			self.fRec68[1] = self.fRec68[0];
			self.fRec73[1] = self.fRec73[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec72[2] = self.fRec72[1];
			self.fRec72[1] = self.fRec72[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec79[1] = self.fRec79[0];
			self.fRec76[2] = self.fRec76[1];
			self.fRec76[1] = self.fRec76[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec80[2] = self.fRec80[1];
			self.fRec80[1] = self.fRec80[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec86[1] = self.fRec86[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec84[2] = self.fRec84[1];
			self.fRec84[1] = self.fRec84[0];
			self.fRec89[1] = self.fRec89[0];
			self.fRec90[1] = self.fRec90[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec88[2] = self.fRec88[1];
			self.fRec88[1] = self.fRec88[0];
			self.fRec93[1] = self.fRec93[0];
			self.fRec94[1] = self.fRec94[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec92[2] = self.fRec92[1];
			self.fRec92[1] = self.fRec92[0];
		}
	}

}

