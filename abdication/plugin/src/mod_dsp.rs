/* ------------------------------------------------------------
name: "plugin-mod"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn ModDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

static mut fModDspSIG0Wave0: [F32;15] = [16.0,8.0,4.0,3.0,2.0,1.5,1.3333333,1.0,0.75,0.6666667,0.5,0.3333333,0.25,0.1666667,0.125];

pub struct ModDspSIG0 {
	fModDspSIG0Wave0_idx: i32,
}

impl ModDspSIG0 {
	
	fn get_num_inputsModDspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsModDspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initModDspSIG0(&mut self, sample_rate: i32) {
		self.fModDspSIG0Wave0_idx = 0;
	}
	
	fn fillModDspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			table[i1 as usize] = unsafe { fModDspSIG0Wave0[self.fModDspSIG0Wave0_idx as usize] };
			self.fModDspSIG0Wave0_idx = (i32::wrapping_add(1, self.fModDspSIG0Wave0_idx)) % 15;
		}
	}

}


pub fn newModDspSIG0() -> ModDspSIG0 { 
	ModDspSIG0 {
		fModDspSIG0Wave0_idx: 0,
	}
}
fn ModDsp_faustpower4_f(value: F32) -> F32 {
	return value * value * value * value;
}
static mut ftbl0ModDspSIG0: [F32;15] = [0.0;15];

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct ModDsp {
	fButton0: F32,
	fVec0: [F32;2],
	fVslider0: F32,
	fRec1: [F32;2],
	fVslider1: F32,
	fSampleRate: i32,
	fConst1: F32,
	fRec2: [F32;2],
	fVslider2: F32,
	fRec3: [F32;2],
	fVslider3: F32,
	fVslider4: F32,
	fRec4: [F32;2],
	fRec0: [F32;2],
	fCheckbox0: F32,
	fConst2: F32,
	fHslider0: F32,
	fRec6: [F32;2],
	fHslider1: F32,
	fRec5: [F32;2],
	fHslider2: F32,
	fVec1: [F32;2],
	iRec7: [i32;2],
	fHslider3: F32,
	fConst3: F32,
	fEntry0: F32,
}

impl FaustDsp for ModDsp {
	type T = F32;
		
	fn new() -> ModDsp { 
		ModDsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			fVslider0: 0.0,
			fRec1: [0.0;2],
			fVslider1: 0.0,
			fSampleRate: 0,
			fConst1: 0.0,
			fRec2: [0.0;2],
			fVslider2: 0.0,
			fRec3: [0.0;2],
			fVslider3: 0.0,
			fVslider4: 0.0,
			fRec4: [0.0;2],
			fRec0: [0.0;2],
			fCheckbox0: 0.0,
			fConst2: 0.0,
			fHslider0: 0.0,
			fRec6: [0.0;2],
			fHslider1: 0.0,
			fRec5: [0.0;2],
			fHslider2: 0.0,
			fVec1: [0.0;2],
			iRec7: [0;2],
			fHslider3: 0.0,
			fConst3: 0.0,
			fEntry0: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn ModDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("filename", r"plugin-mod.dsp");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"plugin-mod");
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
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: ModDspSIG0 = newModDspSIG0();
		sig0.instance_initModDspSIG0(sample_rate);
		sig0.fillModDspSIG0(15, unsafe { &mut ftbl0ModDspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fVslider0 = 0.5;
		self.fVslider1 = 0.5;
		self.fVslider2 = 0.005;
		self.fVslider3 = 0.5;
		self.fVslider4 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fHslider0 = 1.0;
		self.fHslider1 = 0.0;
		self.fHslider2 = 0.0;
		self.fHslider3 = 1.2e+02;
		self.fEntry0 = 217.0;
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
		for l4 in 0..2 {
			self.fRec4[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec0[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec6[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec5[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fVec1[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.iRec7[l9 as usize] = 0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 / fConst0;
		self.fConst2 = 1.0 - 44.1 / fConst0;
		self.fConst3 = 0.016666668 / fConst0;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		ModDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin-mod");
		ui_interface.declare(None, "2.1", "");
		ui_interface.open_horizontal_box("Mod Env");
		ui_interface.declare(Some(ParamIndex(0)), "1", "");
		ui_interface.declare(Some(ParamIndex(0)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(0)), "unit", "s");
		ui_interface.add_vertical_slider("Attack", ParamIndex(0), 0.005, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(1)), "2", "");
		ui_interface.declare(Some(ParamIndex(1)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "s");
		ui_interface.add_vertical_slider("Decay", ParamIndex(1), 0.5, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "3", "");
		ui_interface.add_vertical_slider("Sustain", ParamIndex(2), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(3)), "4", "");
		ui_interface.declare(Some(ParamIndex(3)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "s");
		ui_interface.add_vertical_slider("Release", ParamIndex(3), 0.5, 0.001, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "5", "");
		ui_interface.add_vertical_slider("Curve", ParamIndex(4), 0.5, -1.0, 1.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "2.2", "");
		ui_interface.open_horizontal_box("Mod LFO");
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.add_horizontal_slider("Rate Env", ParamIndex(5), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.declare(Some(ParamIndex(6)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(6)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(6), 1.0, 0.02, 2e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "2", "");
		ui_interface.declare(Some(ParamIndex(7)), "style", "menu{'A8 001':0;'A8 002':1;'A8 003':2;'A8 004':3;'A8 005':4;'A8 006':5;'A8 007':6;'A8 008':7;'A8 009':8;'A8 010':9;'A8 011':10;'A8 012':11;'A8 013':12;'A8 014':13;'A8 015':14;'A8 016':15;'A8 017':16;'A8 018':17;'A8 019':18;'A8 020':19;'A8 021':20;'A8 022':21;'A8 023':22;'A8 024':23;'A8 025':24;'A8 026':25;'A8 027':26;'A8 028':27;'A8 029':28;'A8 030':29;'A8 031':30;'A8 032':31;'A8 033':32;'A8 034':33;'A8 035':34;'A8 036':35;'A8 037':36;'A8 038':37;'A8 039':38;'A8 040':39;'A8 041':40;'A8 042':41;'A8 043':42;'A8 044':43;'A8 045':44;'A8 046':45;'A8 047':46;'A8 048':47;'A8 049':48;'A8 050':49;'A8 051':50;'A8 052':51;'A8 053':52;'A8 054':53;'A8 055':54;'A8 056':55;'A8 057':56;'A8 058':57;'A8 059':58;'A8 060':59;'A8 061':60;'A8 062':61;'A8 063':62;'A8 064':63;'A8 065':64;'A8 066':65;'A8 067':66;'A8 068':67;'A8 069':68;'A8 070':69;'A8 071':70;'A8 072':71;'A8 073':72;'A8 074':73;'A8 075':74;'A8 076':75;'A8 077':76;'A8 078':77;'A8 079':78;'A8 080':79;'A8 081':80;'A8 082':81;'A8 083':82;'A8 084':83;'A8 085':84;'A8 086':85;'A8 087':86;'A8 088':87;'A8 089':88;'A8 090':89;'A8 091':90;'A8 092':91;'A8 093':92;'A8 094':93;'A8 095':94;'A8 096':95;'A8 097':96;'A8 098':97;'A8 099':98;'A8 100':99;'A8 101':100;'A8 102':101;'A8 103':102;'A8 104':103;'A8 105':104;'A8 106':105;'A8 107':106;'A8 108':107;'A8 109':108;'A8 110':109;'A8 111':110;'A8 112':111;'A8 113':112;'A8 114':113;'A8 115':114;'A8 116':115;'A8 117':116;'A8 118':117;'M8 01':118;'M8 02':119;'M8 03':120;'M8 04':121;'M8 05':122;'M8 06':123;'M8 07':124;'M8 08':125;'M8 09':126;'M8 10':127;'M8 11':128;'M8 12':129;'M8 13':130;'M8 14':131;'M8 15':132;'M8 16':133;'M8 17':134;'M8 18':135;'M8 19':136;'M8 20':137;'M8 21':138;'M8 22':139;'M8 23':140;'M8 24':141;'M8 25':142;'M8 26':143;'M8 27':144;'M8 28':145;'M8 29':146;'M8 30':147;'M8 31':148;'M8 32':149;'M8 33':150;'M8 34':151;'M8 35':152;'M8 36':153;'M8 37':154;'M8 38':155;'M8 39':156;'M8 40':157;'M8 41':158;'M8 42':159;'M8 43':160;'M8 44':161;'M8 45':162;'M8 46':163;'M8 47':164;'M8 48':165;'M8 49':166;'M8 50':167;'M8 51':168;'M8 52':169;'M16 01':170;'M16 02':171;'M16 03':172;'M16 04':173;'M16 05':174;'M16 06':175;'M16 07':176;'M16 08':177;'M16 09':178;'M16 10':179;'M16 11':180;'M16 12':181;'M16 13':182;'M16 14':183;'M16 15':184;'M16 16':185;'M16 17':186;'M16 18':187;'M16 19':188;'M16 20':189;'M16 21':190;'M16 22':191;'M16 23':192;'M16 24':193;'M16 25':194;'M16 26':195;'M16 27':196;'M16 28':197;'M16 29':198;'M16 30':199;'M16 31':200;'M16 32':201;'M16 33':202;'M16 34':203;'M16 35':204;'M16 36':205;'M16 37':206;'M16 38':207;'M16 39':208;'M16 40':209;'M16 41':210;'M16 42':211;'M16 43':212;'M16 44':213;'M16 45':214;'M16 46':215;'M16 47':216;'Sine':217;'Triangle':218;'Saw':219;'Square':220}");
		ui_interface.add_num_entry("Shape", ParamIndex(7), 217.0, 0.0, 2.2e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "3", "");
		ui_interface.add_check_button("Sync", ParamIndex(8));
		ui_interface.close_box();
		ui_interface.add_horizontal_slider("beats", ParamIndex(9), 0.0, 0.0, 48.0, 1e-06);
		ui_interface.add_button("gate", ParamIndex(10));
		ui_interface.add_horizontal_slider("tempo", ParamIndex(11), 1.2e+02, 1.0, 999.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			10 => Some(self.fButton0),
			8 => Some(self.fCheckbox0),
			7 => Some(self.fEntry0),
			6 => Some(self.fHslider0),
			5 => Some(self.fHslider1),
			9 => Some(self.fHslider2),
			11 => Some(self.fHslider3),
			4 => Some(self.fVslider0),
			3 => Some(self.fVslider1),
			0 => Some(self.fVslider2),
			1 => Some(self.fVslider3),
			2 => Some(self.fVslider4),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			10 => { self.fButton0 = value }
			8 => { self.fCheckbox0 = value }
			7 => { self.fEntry0 = value }
			6 => { self.fHslider0 = value }
			5 => { self.fHslider1 = value }
			9 => { self.fHslider2 = value }
			11 => { self.fHslider3 = value }
			4 => { self.fVslider0 = value }
			3 => { self.fVslider1 = value }
			0 => { self.fVslider2 = value }
			1 => { self.fVslider3 = value }
			2 => { self.fVslider4 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut iSlow1: i32 = (fSlow0) as i32;
		let mut fSlow2: F32 = 6.0 * F32::max(-1.0, F32::min(1.0, self.fVslider0));
		let mut fSlow3: F32 = if (F32::abs(-fSlow2) < 0.001) as i32 != 0 {0.001} else {-fSlow2};
		let mut fSlow4: F32 = F32::exp(fSlow3) + -1.0;
		let mut fSlow5: F32 = 1.0 - fSlow0;
		let mut fSlow6: F32 = self.fConst1 / F32::max(self.fVslider1, 0.0001);
		let mut fSlow7: F32 = F32::max(self.fVslider2, 0.0001);
		let mut fSlow8: F32 = 1.0 / F32::max(self.fVslider3, 0.0001);
		let mut fSlow9: F32 = self.fVslider4;
		let mut fSlow10: F32 = 1.0 - fSlow9;
		let mut fSlow11: F32 = if (F32::abs(fSlow2) < 0.001) as i32 != 0 {0.001} else {fSlow2};
		let mut fSlow12: F32 = F32::exp(fSlow11) + -1.0;
		let mut fSlow13: F32 = self.fConst1 / fSlow7;
		let mut iSlow14: i32 = (self.fCheckbox0) as i32;
		let mut fSlow15: F32 = self.fHslider0;
		let mut fSlow16: F32 = self.fHslider1;
		let mut fSlow17: F32 = self.fHslider2;
		let mut fSlow18: F32 = self.fConst3 * self.fHslider3;
		let mut iSlow19: i32 = i32::wrapping_mul(2176, (self.fEntry0) as i32);
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			self.fRec1[0] = fSlow5 * (self.fRec1[1] + 1.0);
			self.fRec2[0] = if (fSlow0 < self.fVec0[1]) as i32 != 0 {self.fRec0[1]} else {self.fRec2[1]};
			self.fRec3[0] = fSlow0 * (self.fRec3[1] + 1.0);
			let mut fTemp0: F32 = self.fConst1 * self.fRec3[0];
			let mut iTemp1: i32 = (fSlow0 > self.fVec0[1]) as i32;
			self.fRec4[0] = if iTemp1 != 0 {self.fRec0[1]} else {self.fRec4[1]};
			self.fRec0[0] = if iSlow1 != 0 {if (fTemp0 < fSlow7) as i32 != 0 {self.fRec4[0] + (1.0 - self.fRec4[0]) * (F32::exp(fSlow11 * F32::min(1.0, fSlow13 * self.fRec3[0])) + -1.0) / fSlow12} else {fSlow9 + fSlow10 * (1.0 - (F32::exp(fSlow3 * F32::min(1.0, fSlow8 * (fTemp0 - fSlow7))) + -1.0) / fSlow4)}} else {self.fRec2[0] * (1.0 - (F32::exp(fSlow3 * F32::min(1.0, fSlow6 * self.fRec1[0])) + -1.0) / fSlow4)};
			*output0 = self.fRec0[0];
			let mut fTemp2: F32 = (i32::wrapping_sub(1, iTemp1)) as F32;
			self.fRec6[0] = fSlow15 * (1.0 - self.fConst2 * fTemp2) + self.fConst2 * fTemp2 * self.fRec6[1];
			let mut fTemp3: F32 = F32::max(0.0, F32::min(1.0, fSlow16 * self.fRec0[0] + F32::sqrt(F32::sqrt(F32::max(0.0, 0.05005005 * (self.fRec6[0] + -0.02))))));
			let mut fTemp4: F32 = self.fRec5[1] + self.fConst1 * (19.98 * ModDsp_faustpower4_f(fTemp3) + 0.02);
			self.fRec5[0] = if iTemp1 != 0 {0.0} else {fTemp4 - F32::floor(fTemp4)};
			self.fVec1[0] = fSlow17;
			self.iRec7[0] = i32::wrapping_mul(i32::wrapping_sub(1, (fSlow17 != self.fVec1[1]) as i32), i32::wrapping_add(self.iRec7[1], 1));
			let mut fTemp5: F32 = (fSlow17 + fSlow18 * (self.iRec7[0]) as F32) / unsafe { ftbl0ModDspSIG0[((14.0 * fTemp3 + 0.5) as i32) as usize] };
			let mut fTemp6: F32 = 1024.0 * if iSlow14 != 0 {fTemp5 - F32::floor(fTemp5)} else {self.fRec5[0]};
			let mut iTemp7: i32 = (fTemp6) as i32;
			let mut fTemp8: F32 = (iTemp7) as F32;
			*output1 = (fTemp8 + (1.0 - fTemp6)) * wt_sample(i32::wrapping_add(iSlow19, iTemp7)) + (fTemp6 - fTemp8) * wt_sample(i32::wrapping_add(iSlow19, (i32::wrapping_add(iTemp7, 1)) % 1024));
			self.fVec0[1] = self.fVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec5[1] = self.fRec5[0];
			self.fVec1[1] = self.fVec1[0];
			self.iRec7[1] = self.iRec7[0];
		}
	}

}

