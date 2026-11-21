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
	fTempPerm0: F32,
	fTempPerm1: F32,
	fTempPerm2: F32,
	fTempPerm3: F32,
	fTempPerm4: F32,
	iVec0: [i32;2],
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
	fConst3: F32,
	fConst4: F32,
	iTempPerm26: i32,
	iRec14: [i32;2],
	fRec13: [F32;2],
	fTempPerm27: F32,
	fTempPerm28: F32,
	fTempPerm29: F32,
	fTempPerm30: F32,
	iRec16: [i32;2],
	fTempPerm31: F32,
	fRec17: [F32;4],
	fConst5: F32,
	iTempPerm32: i32,
	iTempPerm33: i32,
	iRec18: [i32;2],
	fConst6: F32,
	fTempPerm34: F32,
	iTempPerm35: i32,
	fConst7: F32,
	fTempPerm36: F32,
	fRec22: [F32;2],
	fRec21: [F32;2],
	fTempPerm37: F32,
	fTempPerm38: F32,
	fTempPerm39: F32,
	fTempPerm40: F32,
	fTempPerm41: F32,
	fTempPerm42: F32,
	fConst8: F32,
	fTempPerm43: F32,
	fRec23: [F32;2],
	fTempPerm44: F32,
	fConst9: F32,
	fTempPerm45: F32,
	fRec24: [F32;2],
	fTempPerm46: F32,
	fConst10: F32,
	fTempPerm47: F32,
	fRec25: [F32;2],
	fTempPerm48: F32,
	fConst11: F32,
	fTempPerm49: F32,
	fRec26: [F32;2],
	fTempPerm50: F32,
	fConst12: F32,
	fTempPerm51: F32,
	fRec27: [F32;2],
	fTempPerm52: F32,
	fTempPerm53: F32,
	fRec28: [F32;2],
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
	fTempPerm80: F32,
	fTempPerm81: F32,
	fTempPerm82: F32,
	fTempPerm83: F32,
	fTempPerm84: F32,
	fTempPerm85: F32,
	fTempPerm86: F32,
	iTempPerm87: i32,
	fTempPerm88: F32,
	fTempPerm89: F32,
	fTempPerm90: F32,
	fTempPerm91: F32,
	fTempPerm92: F32,
	fTempPerm93: F32,
	fRec29: [F32;2],
	fTempPerm94: F32,
	fTempPerm95: F32,
	fTempPerm96: F32,
	fRec30: [F32;2],
	fTempPerm97: F32,
	fTempPerm98: F32,
	fTempPerm99: F32,
	fRec31: [F32;2],
	fTempPerm100: F32,
	fTempPerm101: F32,
	fTempPerm102: F32,
	fRec32: [F32;2],
	fTempPerm103: F32,
	fTempPerm104: F32,
	fTempPerm105: F32,
	fRec33: [F32;2],
	fTempPerm106: F32,
	fTempPerm107: F32,
	fTempPerm108: F32,
	fRec34: [F32;2],
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
	fConst13: F32,
	fTempPerm141: F32,
	fTempPerm142: F32,
	fRec37: [F32;2],
	fTempPerm143: F32,
	iTempPerm144: i32,
	iTempPerm145: i32,
	iTempPerm146: i32,
	iRec38: [i32;2],
	fRec36: [F32;2],
	iTempPerm147: i32,
	fConst14: F32,
	fTempPerm148: F32,
	fRec44: [F32;2],
	fRec43: [F32;2],
	fTempPerm149: F32,
	fRec42: [F32;2],
	fTempPerm150: F32,
	fTempPerm151: F32,
	fVec1: [F32;2],
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fRec35: [F32;2],
	fTempPerm152: F32,
	fConst18: F32,
	fConst20: F32,
	fTempPerm153: F32,
	fVec2: [F32;2],
	fConst21: F32,
	fRec47: [F32;2],
	fRec46: [F32;2],
	fRec45: [F32;2],
	fTempPerm154: F32,
	fTempPerm155: F32,
	fTempPerm156: F32,
	fTempPerm157: F32,
	fTempPerm158: F32,
	fTempPerm159: F32,
	fRec49: [F32;2],
	fRec48: [F32;2],
	fTempPerm160: F32,
	fTempPerm161: F32,
	fTempPerm162: F32,
	fTempPerm163: F32,
	fTempPerm164: F32,
	fTempPerm165: F32,
	fRec10: [F32;2],
	fTempPerm166: F32,
	fTempPerm167: F32,
	fRec11: [F32;2],
	fTempPerm168: F32,
	fTempPerm169: F32,
	fTempPerm170: F32,
	fTempPerm171: F32,
	fTempPerm172: F32,
	fRec7: [F32;2],
	fTempPerm173: F32,
	fTempPerm174: F32,
	fRec8: [F32;2],
	fTempPerm175: F32,
	fTempPerm176: F32,
	fTempPerm177: F32,
	fTempPerm178: F32,
	fTempPerm179: F32,
	fRec4: [F32;2],
	fTempPerm180: F32,
	fTempPerm181: F32,
	fRec5: [F32;2],
	fTempPerm182: F32,
	fTempPerm183: F32,
	fTempPerm184: F32,
	fTempPerm185: F32,
	fTempPerm186: F32,
	fRec1: [F32;2],
	fTempPerm187: F32,
	fTempPerm188: F32,
	fRec2: [F32;2],
	fTempPerm189: F32,
	fTempPerm190: F32,
	fRec0: [F32;3],
	fTempPerm191: F32,
	fTempPerm192: F32,
	fTempPerm193: F32,
	fTempPerm194: F32,
	fTempPerm195: F32,
	fTempPerm196: F32,
	fTempPerm197: F32,
	fTempPerm198: F32,
	fTempPerm199: F32,
	fRec60: [F32;2],
	fTempPerm200: F32,
	fTempPerm201: F32,
	fRec61: [F32;2],
	fTempPerm202: F32,
	fTempPerm203: F32,
	fTempPerm204: F32,
	fTempPerm205: F32,
	fRec57: [F32;2],
	fTempPerm206: F32,
	fTempPerm207: F32,
	fRec58: [F32;2],
	fTempPerm208: F32,
	fTempPerm209: F32,
	fTempPerm210: F32,
	fTempPerm211: F32,
	fRec54: [F32;2],
	fTempPerm212: F32,
	fTempPerm213: F32,
	fRec55: [F32;2],
	fTempPerm214: F32,
	fTempPerm215: F32,
	fTempPerm216: F32,
	fTempPerm217: F32,
	fRec51: [F32;2],
	fTempPerm218: F32,
	fTempPerm219: F32,
	fRec52: [F32;2],
	fRec50: [F32;3],
	fTempPerm220: F32,
}

impl FaustDsp for NoiseDsp {
	type T = F32;
		
	fn new() -> NoiseDsp { 
		NoiseDsp {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fTempPerm0: 0.0,
			fTempPerm1: 0.0,
			fTempPerm2: 0.0,
			fTempPerm3: 0.0,
			fTempPerm4: 0.0,
			iVec0: [0;2],
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
			fConst3: 0.0,
			fConst4: 0.0,
			iTempPerm26: 0,
			iRec14: [0;2],
			fRec13: [0.0;2],
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fTempPerm29: 0.0,
			fTempPerm30: 0.0,
			iRec16: [0;2],
			fTempPerm31: 0.0,
			fRec17: [0.0;4],
			fConst5: 0.0,
			iTempPerm32: 0,
			iTempPerm33: 0,
			iRec18: [0;2],
			fConst6: 0.0,
			fTempPerm34: 0.0,
			iTempPerm35: 0,
			fConst7: 0.0,
			fTempPerm36: 0.0,
			fRec22: [0.0;2],
			fRec21: [0.0;2],
			fTempPerm37: 0.0,
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fTempPerm40: 0.0,
			fTempPerm41: 0.0,
			fTempPerm42: 0.0,
			fConst8: 0.0,
			fTempPerm43: 0.0,
			fRec23: [0.0;2],
			fTempPerm44: 0.0,
			fConst9: 0.0,
			fTempPerm45: 0.0,
			fRec24: [0.0;2],
			fTempPerm46: 0.0,
			fConst10: 0.0,
			fTempPerm47: 0.0,
			fRec25: [0.0;2],
			fTempPerm48: 0.0,
			fConst11: 0.0,
			fTempPerm49: 0.0,
			fRec26: [0.0;2],
			fTempPerm50: 0.0,
			fConst12: 0.0,
			fTempPerm51: 0.0,
			fRec27: [0.0;2],
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fRec28: [0.0;2],
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
			fTempPerm80: 0.0,
			fTempPerm81: 0.0,
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			fTempPerm84: 0.0,
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			iTempPerm87: 0,
			fTempPerm88: 0.0,
			fTempPerm89: 0.0,
			fTempPerm90: 0.0,
			fTempPerm91: 0.0,
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fRec29: [0.0;2],
			fTempPerm94: 0.0,
			fTempPerm95: 0.0,
			fTempPerm96: 0.0,
			fRec30: [0.0;2],
			fTempPerm97: 0.0,
			fTempPerm98: 0.0,
			fTempPerm99: 0.0,
			fRec31: [0.0;2],
			fTempPerm100: 0.0,
			fTempPerm101: 0.0,
			fTempPerm102: 0.0,
			fRec32: [0.0;2],
			fTempPerm103: 0.0,
			fTempPerm104: 0.0,
			fTempPerm105: 0.0,
			fRec33: [0.0;2],
			fTempPerm106: 0.0,
			fTempPerm107: 0.0,
			fTempPerm108: 0.0,
			fRec34: [0.0;2],
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
			fConst13: 0.0,
			fTempPerm141: 0.0,
			fTempPerm142: 0.0,
			fRec37: [0.0;2],
			fTempPerm143: 0.0,
			iTempPerm144: 0,
			iTempPerm145: 0,
			iTempPerm146: 0,
			iRec38: [0;2],
			fRec36: [0.0;2],
			iTempPerm147: 0,
			fConst14: 0.0,
			fTempPerm148: 0.0,
			fRec44: [0.0;2],
			fRec43: [0.0;2],
			fTempPerm149: 0.0,
			fRec42: [0.0;2],
			fTempPerm150: 0.0,
			fTempPerm151: 0.0,
			fVec1: [0.0;2],
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fRec35: [0.0;2],
			fTempPerm152: 0.0,
			fConst18: 0.0,
			fConst20: 0.0,
			fTempPerm153: 0.0,
			fVec2: [0.0;2],
			fConst21: 0.0,
			fRec47: [0.0;2],
			fRec46: [0.0;2],
			fRec45: [0.0;2],
			fTempPerm154: 0.0,
			fTempPerm155: 0.0,
			fTempPerm156: 0.0,
			fTempPerm157: 0.0,
			fTempPerm158: 0.0,
			fTempPerm159: 0.0,
			fRec49: [0.0;2],
			fRec48: [0.0;2],
			fTempPerm160: 0.0,
			fTempPerm161: 0.0,
			fTempPerm162: 0.0,
			fTempPerm163: 0.0,
			fTempPerm164: 0.0,
			fTempPerm165: 0.0,
			fRec10: [0.0;2],
			fTempPerm166: 0.0,
			fTempPerm167: 0.0,
			fRec11: [0.0;2],
			fTempPerm168: 0.0,
			fTempPerm169: 0.0,
			fTempPerm170: 0.0,
			fTempPerm171: 0.0,
			fTempPerm172: 0.0,
			fRec7: [0.0;2],
			fTempPerm173: 0.0,
			fTempPerm174: 0.0,
			fRec8: [0.0;2],
			fTempPerm175: 0.0,
			fTempPerm176: 0.0,
			fTempPerm177: 0.0,
			fTempPerm178: 0.0,
			fTempPerm179: 0.0,
			fRec4: [0.0;2],
			fTempPerm180: 0.0,
			fTempPerm181: 0.0,
			fRec5: [0.0;2],
			fTempPerm182: 0.0,
			fTempPerm183: 0.0,
			fTempPerm184: 0.0,
			fTempPerm185: 0.0,
			fTempPerm186: 0.0,
			fRec1: [0.0;2],
			fTempPerm187: 0.0,
			fTempPerm188: 0.0,
			fRec2: [0.0;2],
			fTempPerm189: 0.0,
			fTempPerm190: 0.0,
			fRec0: [0.0;3],
			fTempPerm191: 0.0,
			fTempPerm192: 0.0,
			fTempPerm193: 0.0,
			fTempPerm194: 0.0,
			fTempPerm195: 0.0,
			fTempPerm196: 0.0,
			fTempPerm197: 0.0,
			fTempPerm198: 0.0,
			fTempPerm199: 0.0,
			fRec60: [0.0;2],
			fTempPerm200: 0.0,
			fTempPerm201: 0.0,
			fRec61: [0.0;2],
			fTempPerm202: 0.0,
			fTempPerm203: 0.0,
			fTempPerm204: 0.0,
			fTempPerm205: 0.0,
			fRec57: [0.0;2],
			fTempPerm206: 0.0,
			fTempPerm207: 0.0,
			fRec58: [0.0;2],
			fTempPerm208: 0.0,
			fTempPerm209: 0.0,
			fTempPerm210: 0.0,
			fTempPerm211: 0.0,
			fRec54: [0.0;2],
			fTempPerm212: 0.0,
			fTempPerm213: 0.0,
			fRec55: [0.0;2],
			fTempPerm214: 0.0,
			fTempPerm215: 0.0,
			fTempPerm216: 0.0,
			fTempPerm217: 0.0,
			fRec51: [0.0;2],
			fTempPerm218: 0.0,
			fTempPerm219: 0.0,
			fRec52: [0.0;2],
			fRec50: [0.0;3],
			fTempPerm220: 0.0,
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
		return 16;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
	}
	fn instance_clear(&mut self) {
		self.fTempPerm0 = 0.0;
		self.fTempPerm1 = 0.0;
		self.fTempPerm2 = 0.0;
		self.fTempPerm3 = 0.0;
		self.fTempPerm4 = 0.0;
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
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
		self.iTempPerm26 = 0;
		for l1 in 0..2 {
			self.iRec14[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec13[l2 as usize] = 0.0;
		}
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		self.fTempPerm29 = 0.0;
		self.fTempPerm30 = 0.0;
		for l3 in 0..2 {
			self.iRec16[l3 as usize] = 0;
		}
		self.fTempPerm31 = 0.0;
		for l4 in 0..4 {
			self.fRec17[l4 as usize] = 0.0;
		}
		self.iTempPerm32 = 0;
		self.iTempPerm33 = 0;
		for l5 in 0..2 {
			self.iRec18[l5 as usize] = 0;
		}
		self.fTempPerm34 = 0.0;
		self.iTempPerm35 = 0;
		self.fTempPerm36 = 0.0;
		for l6 in 0..2 {
			self.fRec22[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec21[l7 as usize] = 0.0;
		}
		self.fTempPerm37 = 0.0;
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		self.fTempPerm42 = 0.0;
		self.fTempPerm43 = 0.0;
		for l8 in 0..2 {
			self.fRec23[l8 as usize] = 0.0;
		}
		self.fTempPerm44 = 0.0;
		self.fTempPerm45 = 0.0;
		for l9 in 0..2 {
			self.fRec24[l9 as usize] = 0.0;
		}
		self.fTempPerm46 = 0.0;
		self.fTempPerm47 = 0.0;
		for l10 in 0..2 {
			self.fRec25[l10 as usize] = 0.0;
		}
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		for l11 in 0..2 {
			self.fRec26[l11 as usize] = 0.0;
		}
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		for l12 in 0..2 {
			self.fRec27[l12 as usize] = 0.0;
		}
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		for l13 in 0..2 {
			self.fRec28[l13 as usize] = 0.0;
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
		self.fTempPerm80 = 0.0;
		self.fTempPerm81 = 0.0;
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.fTempPerm84 = 0.0;
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		self.iTempPerm87 = 0;
		self.fTempPerm88 = 0.0;
		self.fTempPerm89 = 0.0;
		self.fTempPerm90 = 0.0;
		self.fTempPerm91 = 0.0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		for l14 in 0..2 {
			self.fRec29[l14 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		self.fTempPerm96 = 0.0;
		for l15 in 0..2 {
			self.fRec30[l15 as usize] = 0.0;
		}
		self.fTempPerm97 = 0.0;
		self.fTempPerm98 = 0.0;
		self.fTempPerm99 = 0.0;
		for l16 in 0..2 {
			self.fRec31[l16 as usize] = 0.0;
		}
		self.fTempPerm100 = 0.0;
		self.fTempPerm101 = 0.0;
		self.fTempPerm102 = 0.0;
		for l17 in 0..2 {
			self.fRec32[l17 as usize] = 0.0;
		}
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		self.fTempPerm105 = 0.0;
		for l18 in 0..2 {
			self.fRec33[l18 as usize] = 0.0;
		}
		self.fTempPerm106 = 0.0;
		self.fTempPerm107 = 0.0;
		self.fTempPerm108 = 0.0;
		for l19 in 0..2 {
			self.fRec34[l19 as usize] = 0.0;
		}
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
		for l20 in 0..2 {
			self.fRec37[l20 as usize] = 0.0;
		}
		self.fTempPerm143 = 0.0;
		self.iTempPerm144 = 0;
		self.iTempPerm145 = 0;
		self.iTempPerm146 = 0;
		for l21 in 0..2 {
			self.iRec38[l21 as usize] = 0;
		}
		for l22 in 0..2 {
			self.fRec36[l22 as usize] = 0.0;
		}
		self.iTempPerm147 = 0;
		self.fTempPerm148 = 0.0;
		for l23 in 0..2 {
			self.fRec44[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec43[l24 as usize] = 0.0;
		}
		self.fTempPerm149 = 0.0;
		for l25 in 0..2 {
			self.fRec42[l25 as usize] = 0.0;
		}
		self.fTempPerm150 = 0.0;
		self.fTempPerm151 = 0.0;
		for l26 in 0..2 {
			self.fVec1[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec35[l27 as usize] = 0.0;
		}
		self.fTempPerm152 = 0.0;
		self.fTempPerm153 = 0.0;
		for l28 in 0..2 {
			self.fVec2[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec47[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec46[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec45[l31 as usize] = 0.0;
		}
		self.fTempPerm154 = 0.0;
		self.fTempPerm155 = 0.0;
		self.fTempPerm156 = 0.0;
		self.fTempPerm157 = 0.0;
		self.fTempPerm158 = 0.0;
		self.fTempPerm159 = 0.0;
		for l32 in 0..2 {
			self.fRec49[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec48[l33 as usize] = 0.0;
		}
		self.fTempPerm160 = 0.0;
		self.fTempPerm161 = 0.0;
		self.fTempPerm162 = 0.0;
		self.fTempPerm163 = 0.0;
		self.fTempPerm164 = 0.0;
		self.fTempPerm165 = 0.0;
		for l34 in 0..2 {
			self.fRec10[l34 as usize] = 0.0;
		}
		self.fTempPerm166 = 0.0;
		self.fTempPerm167 = 0.0;
		for l35 in 0..2 {
			self.fRec11[l35 as usize] = 0.0;
		}
		self.fTempPerm168 = 0.0;
		self.fTempPerm169 = 0.0;
		self.fTempPerm170 = 0.0;
		self.fTempPerm171 = 0.0;
		self.fTempPerm172 = 0.0;
		for l36 in 0..2 {
			self.fRec7[l36 as usize] = 0.0;
		}
		self.fTempPerm173 = 0.0;
		self.fTempPerm174 = 0.0;
		for l37 in 0..2 {
			self.fRec8[l37 as usize] = 0.0;
		}
		self.fTempPerm175 = 0.0;
		self.fTempPerm176 = 0.0;
		self.fTempPerm177 = 0.0;
		self.fTempPerm178 = 0.0;
		self.fTempPerm179 = 0.0;
		for l38 in 0..2 {
			self.fRec4[l38 as usize] = 0.0;
		}
		self.fTempPerm180 = 0.0;
		self.fTempPerm181 = 0.0;
		for l39 in 0..2 {
			self.fRec5[l39 as usize] = 0.0;
		}
		self.fTempPerm182 = 0.0;
		self.fTempPerm183 = 0.0;
		self.fTempPerm184 = 0.0;
		self.fTempPerm185 = 0.0;
		self.fTempPerm186 = 0.0;
		for l40 in 0..2 {
			self.fRec1[l40 as usize] = 0.0;
		}
		self.fTempPerm187 = 0.0;
		self.fTempPerm188 = 0.0;
		for l41 in 0..2 {
			self.fRec2[l41 as usize] = 0.0;
		}
		self.fTempPerm189 = 0.0;
		self.fTempPerm190 = 0.0;
		for l42 in 0..3 {
			self.fRec0[l42 as usize] = 0.0;
		}
		self.fTempPerm191 = 0.0;
		self.fTempPerm192 = 0.0;
		self.fTempPerm193 = 0.0;
		self.fTempPerm194 = 0.0;
		self.fTempPerm195 = 0.0;
		self.fTempPerm196 = 0.0;
		self.fTempPerm197 = 0.0;
		self.fTempPerm198 = 0.0;
		self.fTempPerm199 = 0.0;
		for l43 in 0..2 {
			self.fRec60[l43 as usize] = 0.0;
		}
		self.fTempPerm200 = 0.0;
		self.fTempPerm201 = 0.0;
		for l44 in 0..2 {
			self.fRec61[l44 as usize] = 0.0;
		}
		self.fTempPerm202 = 0.0;
		self.fTempPerm203 = 0.0;
		self.fTempPerm204 = 0.0;
		self.fTempPerm205 = 0.0;
		for l45 in 0..2 {
			self.fRec57[l45 as usize] = 0.0;
		}
		self.fTempPerm206 = 0.0;
		self.fTempPerm207 = 0.0;
		for l46 in 0..2 {
			self.fRec58[l46 as usize] = 0.0;
		}
		self.fTempPerm208 = 0.0;
		self.fTempPerm209 = 0.0;
		self.fTempPerm210 = 0.0;
		self.fTempPerm211 = 0.0;
		for l47 in 0..2 {
			self.fRec54[l47 as usize] = 0.0;
		}
		self.fTempPerm212 = 0.0;
		self.fTempPerm213 = 0.0;
		for l48 in 0..2 {
			self.fRec55[l48 as usize] = 0.0;
		}
		self.fTempPerm214 = 0.0;
		self.fTempPerm215 = 0.0;
		self.fTempPerm216 = 0.0;
		self.fTempPerm217 = 0.0;
		for l49 in 0..2 {
			self.fRec51[l49 as usize] = 0.0;
		}
		self.fTempPerm218 = 0.0;
		self.fTempPerm219 = 0.0;
		for l50 in 0..2 {
			self.fRec52[l50 as usize] = 0.0;
		}
		for l51 in 0..3 {
			self.fRec50[l51 as usize] = 0.0;
		}
		self.fTempPerm220 = 0.0;
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
		self.fConst18 = 1.0 - F32::exp(-(9090.909 / self.fConst0));
		let mut fConst19: F32 = 1.0 / F32::tan(6283.1855 / self.fConst0);
		self.fConst20 = 1.0 - fConst19;
		self.fConst21 = 1.0 / (fConst19 + 1.0);
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
		let (inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10, inputs11, inputs12, inputs13, inputs14, inputs15) = if let [inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10, inputs11, inputs12, inputs13, inputs14, inputs15, ..] = inputs {
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
			let inputs11 = inputs11[..count as usize].iter();
			let inputs12 = inputs12[..count as usize].iter();
			let inputs13 = inputs13[..count as usize].iter();
			let inputs14 = inputs14[..count as usize].iter();
			let inputs15 = inputs15[..count as usize].iter();
			(inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, inputs8, inputs9, inputs10, inputs11, inputs12, inputs13, inputs14, inputs15)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fTemp2: F32 = self.fTempPerm0;
		let mut fTemp4: F32 = self.fTempPerm1;
		let mut fTemp5: F32 = self.fTempPerm2;
		let mut fTemp6: F32 = self.fTempPerm3;
		let mut fTemp7: F32 = self.fTempPerm4;
		let mut fTemp8: F32 = self.fTempPerm5;
		let mut fTemp9: F32 = self.fTempPerm6;
		let mut fTemp10: F32 = self.fTempPerm7;
		let mut fTemp11: F32 = self.fTempPerm8;
		let mut fTemp12: F32 = self.fTempPerm9;
		let mut fTemp13: F32 = self.fTempPerm10;
		let mut fTemp14: F32 = self.fTempPerm11;
		let mut fTemp15: F32 = self.fTempPerm12;
		let mut fTemp16: F32 = self.fTempPerm13;
		let mut fTemp17: F32 = self.fTempPerm14;
		let mut fTemp18: F32 = self.fTempPerm15;
		let mut fTemp19: F32 = self.fTempPerm16;
		let mut fTemp20: F32 = self.fTempPerm17;
		let mut fTemp21: F32 = self.fTempPerm18;
		let mut fTemp22: F32 = self.fTempPerm19;
		let mut fTemp23: F32 = self.fTempPerm20;
		let mut fTemp24: F32 = self.fTempPerm21;
		let mut fTemp25: F32 = self.fTempPerm22;
		let mut fTemp26: F32 = self.fTempPerm23;
		let mut fTemp27: F32 = self.fTempPerm24;
		let mut fTemp28: F32 = self.fTempPerm25;
		let mut iTemp29: i32 = self.iTempPerm26;
		let mut fTemp30: F32 = self.fTempPerm27;
		let mut fTemp31: F32 = self.fTempPerm28;
		let mut fTemp32: F32 = self.fTempPerm29;
		let mut fTemp33: F32 = self.fTempPerm30;
		let mut fTemp34: F32 = self.fTempPerm31;
		let mut iTemp35: i32 = self.iTempPerm32;
		let mut iTemp36: i32 = self.iTempPerm33;
		let mut fTemp37: F32 = self.fTempPerm34;
		let mut iTemp38: i32 = self.iTempPerm35;
		let mut fTemp39: F32 = self.fTempPerm36;
		let mut fTemp40: F32 = self.fTempPerm37;
		let mut fTemp41: F32 = self.fTempPerm38;
		let mut fTemp42: F32 = self.fTempPerm39;
		let mut fTemp44: F32 = self.fTempPerm40;
		let mut fTemp43: F32 = self.fTempPerm41;
		let mut fTemp45: F32 = self.fTempPerm42;
		let mut fTemp46: F32 = self.fTempPerm43;
		let mut fTemp47: F32 = self.fTempPerm44;
		let mut fTemp48: F32 = self.fTempPerm45;
		let mut fTemp49: F32 = self.fTempPerm46;
		let mut fTemp50: F32 = self.fTempPerm47;
		let mut fTemp51: F32 = self.fTempPerm48;
		let mut fTemp52: F32 = self.fTempPerm49;
		let mut fTemp53: F32 = self.fTempPerm50;
		let mut fTemp54: F32 = self.fTempPerm51;
		let mut fTemp55: F32 = self.fTempPerm52;
		let mut fTemp56: F32 = self.fTempPerm53;
		let mut fTemp57: F32 = self.fTempPerm54;
		let mut fTemp58: F32 = self.fTempPerm55;
		let mut fTemp59: F32 = self.fTempPerm56;
		let mut fTemp60: F32 = self.fTempPerm57;
		let mut fTemp61: F32 = self.fTempPerm58;
		let mut fTemp62: F32 = self.fTempPerm59;
		let mut fTemp63: F32 = self.fTempPerm60;
		let mut fTemp64: F32 = self.fTempPerm61;
		let mut fTemp65: F32 = self.fTempPerm62;
		let mut fTemp66: F32 = self.fTempPerm63;
		let mut fTemp67: F32 = self.fTempPerm64;
		let mut fTemp68: F32 = self.fTempPerm65;
		let mut fTemp69: F32 = self.fTempPerm66;
		let mut fTemp70: F32 = self.fTempPerm67;
		let mut fTemp71: F32 = self.fTempPerm68;
		let mut fTemp72: F32 = self.fTempPerm69;
		let mut fTemp73: F32 = self.fTempPerm70;
		let mut fTemp74: F32 = self.fTempPerm71;
		let mut fTemp75: F32 = self.fTempPerm72;
		let mut fTemp76: F32 = self.fTempPerm73;
		let mut fTemp77: F32 = self.fTempPerm74;
		let mut fTemp78: F32 = self.fTempPerm75;
		let mut fTemp79: F32 = self.fTempPerm76;
		let mut fTemp80: F32 = self.fTempPerm77;
		let mut fTemp81: F32 = self.fTempPerm78;
		let mut fTemp82: F32 = self.fTempPerm79;
		let mut fTemp83: F32 = self.fTempPerm80;
		let mut fTemp84: F32 = self.fTempPerm81;
		let mut fTemp85: F32 = self.fTempPerm82;
		let mut fTemp86: F32 = self.fTempPerm83;
		let mut fTemp87: F32 = self.fTempPerm84;
		let mut fTemp88: F32 = self.fTempPerm85;
		let mut fTemp89: F32 = self.fTempPerm86;
		let mut iTemp90: i32 = self.iTempPerm87;
		let mut fTemp91: F32 = self.fTempPerm88;
		let mut fTemp92: F32 = self.fTempPerm89;
		let mut fTemp93: F32 = self.fTempPerm90;
		let mut fTemp94: F32 = self.fTempPerm91;
		let mut fTemp95: F32 = self.fTempPerm92;
		let mut fTemp96: F32 = self.fTempPerm93;
		let mut fTemp97: F32 = self.fTempPerm94;
		let mut fTemp98: F32 = self.fTempPerm95;
		let mut fTemp99: F32 = self.fTempPerm96;
		let mut fTemp100: F32 = self.fTempPerm97;
		let mut fTemp101: F32 = self.fTempPerm98;
		let mut fTemp102: F32 = self.fTempPerm99;
		let mut fTemp103: F32 = self.fTempPerm100;
		let mut fTemp104: F32 = self.fTempPerm101;
		let mut fTemp105: F32 = self.fTempPerm102;
		let mut fTemp106: F32 = self.fTempPerm103;
		let mut fTemp107: F32 = self.fTempPerm104;
		let mut fTemp108: F32 = self.fTempPerm105;
		let mut fTemp109: F32 = self.fTempPerm106;
		let mut fTemp110: F32 = self.fTempPerm107;
		let mut fTemp111: F32 = self.fTempPerm108;
		let mut fTemp112: F32 = self.fTempPerm109;
		let mut fTemp113: F32 = self.fTempPerm110;
		let mut fTemp114: F32 = self.fTempPerm111;
		let mut fTemp115: F32 = self.fTempPerm112;
		let mut fTemp116: F32 = self.fTempPerm113;
		let mut fTemp117: F32 = self.fTempPerm114;
		let mut fTemp118: F32 = self.fTempPerm115;
		let mut fTemp119: F32 = self.fTempPerm116;
		let mut fTemp120: F32 = self.fTempPerm117;
		let mut fTemp121: F32 = self.fTempPerm118;
		let mut fTemp122: F32 = self.fTempPerm119;
		let mut fTemp123: F32 = self.fTempPerm120;
		let mut fTemp124: F32 = self.fTempPerm121;
		let mut fTemp125: F32 = self.fTempPerm122;
		let mut fTemp126: F32 = self.fTempPerm123;
		let mut fTemp127: F32 = self.fTempPerm124;
		let mut fTemp128: F32 = self.fTempPerm125;
		let mut fTemp129: F32 = self.fTempPerm126;
		let mut fTemp130: F32 = self.fTempPerm127;
		let mut fTemp131: F32 = self.fTempPerm128;
		let mut fTemp132: F32 = self.fTempPerm129;
		let mut fTemp133: F32 = self.fTempPerm130;
		let mut fTemp134: F32 = self.fTempPerm131;
		let mut fTemp135: F32 = self.fTempPerm132;
		let mut fTemp136: F32 = self.fTempPerm133;
		let mut fTemp137: F32 = self.fTempPerm134;
		let mut fTemp138: F32 = self.fTempPerm135;
		let mut fTemp139: F32 = self.fTempPerm136;
		let mut fTemp140: F32 = self.fTempPerm137;
		let mut fTemp141: F32 = self.fTempPerm138;
		let mut fTemp142: F32 = self.fTempPerm139;
		let mut fTemp143: F32 = self.fTempPerm140;
		let mut fTemp144: F32 = self.fTempPerm141;
		let mut fTemp145: F32 = self.fTempPerm142;
		let mut fTemp146: F32 = self.fTempPerm143;
		let mut iTemp147: i32 = self.iTempPerm144;
		let mut iTemp148: i32 = self.iTempPerm145;
		let mut iTemp149: i32 = self.iTempPerm146;
		let mut iTemp150: i32 = self.iTempPerm147;
		let mut fTemp151: F32 = self.fTempPerm148;
		let mut fTemp152: F32 = self.fTempPerm149;
		let mut fTemp153: F32 = self.fTempPerm150;
		let mut fTemp154: F32 = self.fTempPerm151;
		let mut fTemp155: F32 = self.fTempPerm152;
		let mut fTemp156: F32 = self.fTempPerm153;
		let mut fTemp157: F32 = self.fTempPerm154;
		let mut fTemp158: F32 = self.fTempPerm155;
		let mut fTemp159: F32 = self.fTempPerm156;
		let mut fTemp160: F32 = self.fTempPerm157;
		let mut fTemp161: F32 = self.fTempPerm158;
		let mut fTemp162: F32 = self.fTempPerm159;
		let mut fTemp163: F32 = self.fTempPerm160;
		let mut fTemp164: F32 = self.fTempPerm161;
		let mut fTemp165: F32 = self.fTempPerm162;
		let mut fTemp166: F32 = self.fTempPerm163;
		let mut fTemp167: F32 = self.fTempPerm164;
		let mut fTemp168: F32 = self.fTempPerm165;
		let mut fTemp169: F32 = self.fTempPerm166;
		let mut fTemp170: F32 = self.fTempPerm167;
		let mut fTemp171: F32 = self.fTempPerm168;
		let mut fTemp172: F32 = self.fTempPerm169;
		let mut fTemp173: F32 = self.fTempPerm170;
		let mut fTemp174: F32 = self.fTempPerm171;
		let mut fTemp175: F32 = self.fTempPerm172;
		let mut fTemp176: F32 = self.fTempPerm173;
		let mut fTemp177: F32 = self.fTempPerm174;
		let mut fTemp178: F32 = self.fTempPerm175;
		let mut fTemp179: F32 = self.fTempPerm176;
		let mut fTemp180: F32 = self.fTempPerm177;
		let mut fTemp181: F32 = self.fTempPerm178;
		let mut fTemp182: F32 = self.fTempPerm179;
		let mut fTemp183: F32 = self.fTempPerm180;
		let mut fTemp184: F32 = self.fTempPerm181;
		let mut fTemp185: F32 = self.fTempPerm182;
		let mut fTemp186: F32 = self.fTempPerm183;
		let mut fTemp187: F32 = self.fTempPerm184;
		let mut fTemp188: F32 = self.fTempPerm185;
		let mut fTemp189: F32 = self.fTempPerm186;
		let mut fTemp190: F32 = self.fTempPerm187;
		let mut fTemp191: F32 = self.fTempPerm188;
		let mut fTemp192: F32 = self.fTempPerm189;
		let mut fTemp193: F32 = self.fTempPerm190;
		let mut fTemp194: F32 = self.fTempPerm191;
		let mut fTemp195: F32 = self.fTempPerm192;
		let mut fTemp196: F32 = self.fTempPerm193;
		let mut fTemp197: F32 = self.fTempPerm194;
		let mut fTemp198: F32 = self.fTempPerm195;
		let mut fTemp199: F32 = self.fTempPerm196;
		let mut fTemp200: F32 = self.fTempPerm197;
		let mut fTemp201: F32 = self.fTempPerm198;
		let mut fTemp202: F32 = self.fTempPerm199;
		let mut fTemp203: F32 = self.fTempPerm200;
		let mut fTemp204: F32 = self.fTempPerm201;
		let mut fTemp205: F32 = self.fTempPerm202;
		let mut fTemp206: F32 = self.fTempPerm203;
		let mut fTemp207: F32 = self.fTempPerm204;
		let mut fTemp208: F32 = self.fTempPerm205;
		let mut fTemp209: F32 = self.fTempPerm206;
		let mut fTemp210: F32 = self.fTempPerm207;
		let mut fTemp211: F32 = self.fTempPerm208;
		let mut fTemp212: F32 = self.fTempPerm209;
		let mut fTemp213: F32 = self.fTempPerm210;
		let mut fTemp214: F32 = self.fTempPerm211;
		let mut fTemp215: F32 = self.fTempPerm212;
		let mut fTemp216: F32 = self.fTempPerm213;
		let mut fTemp217: F32 = self.fTempPerm214;
		let mut fTemp218: F32 = self.fTempPerm215;
		let mut fTemp219: F32 = self.fTempPerm216;
		let mut fTemp220: F32 = self.fTempPerm217;
		let mut fTemp221: F32 = self.fTempPerm218;
		let mut fTemp222: F32 = self.fTempPerm219;
		let mut fTemp223: F32 = self.fTempPerm220;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(inputs4).zip(inputs5).zip(inputs6).zip(inputs7).zip(inputs8).zip(inputs9).zip(inputs10).zip(inputs11).zip(inputs12).zip(inputs13).zip(inputs14).zip(inputs15).zip(outputs0);
		for ((((((((((((((((input0, input1), input2), input3), input4), input5), input6), input7), input8), input9), input10), input11), input12), input13), input14), input15), output0) in zipped_iterators {
			let mut iRec15: i32 = 0;
			let mut iRec19: i32 = 0;
			let mut iRec20: i32 = 0;
			let mut iRec39: i32 = 0;
			let mut iRec40: i32 = 0;
			let mut iRec41: i32 = 0;
			let mut fRec12: F32 = 0.0;
			let mut fRec9: F32 = 0.0;
			let mut fRec6: F32 = 0.0;
			let mut fRec3: F32 = 0.0;
			let mut fRec62: F32 = 0.0;
			let mut fRec59: F32 = 0.0;
			let mut fRec56: F32 = 0.0;
			let mut fRec53: F32 = 0.0;
			let mut fTemp0: F32 = *input10;
			let mut fTemp1: F32 = ((fTemp0 > 0.0001) as i32) as u32 as F32;
			let mut fTemp3: F32 = ((fTemp0 <= 0.0001) as i32) as u32 as F32;
			if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
				fTemp2 = F32::tan(self.fConst2 * F32::max(2e+01, F32::min(self.fConst1, *input9)));
				fTemp4 = 1.0 / fTemp2;
				fTemp5 = (fTemp4 + 1.4142135) / fTemp2 + 1.0;
				fTemp6 = NoiseDsp_faustpower2_f(fTemp2);
				fTemp7 = fTemp6 * fTemp5;
				self.iVec0[0] = 1;
				fTemp8 = 1.0 - 1.0 / fTemp6;
				fTemp9 = (fTemp4 + -1.4142135) / fTemp2 + 1.0;
				fTemp10 = *input8;
				fTemp11 = 4.0 * fTemp10;
				fTemp12 = F32::max(0.0, F32::min(1.0, fTemp11 + -3.0));
				fTemp13 = *input2;
				fTemp14 = F32::tan(self.fConst2 * F32::max(2e+01, F32::min(self.fConst1, F32::max(2e+01, F32::min(self.fConst1, fTemp13)))));
				fTemp15 = fTemp14 + 1.1111112;
				fTemp16 = fTemp14 * fTemp15 + 1.0;
			}
				if (fTemp1 != 0.0) as i32 != 0 {
					fTemp17 = F32::tanh(0.2 * self.fRec1[1]);
				}
					if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
						fTemp18 = F32::max(0.0, F32::min(1.0, fTemp11 + -2.0));
					}
						if (fTemp1 != 0.0) as i32 != 0 {
							fTemp19 = F32::tanh(0.2 * self.fRec4[1]);
						}
							if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
								fTemp20 = F32::max(0.0, F32::min(1.0, fTemp11 + -1.0));
							}
								if (fTemp1 != 0.0) as i32 != 0 {
									fTemp21 = F32::tanh(0.2 * self.fRec7[1]);
								}
									if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
										fTemp22 = F32::max(0.0, F32::min(1.0, fTemp11));
									}
										if (fTemp1 != 0.0) as i32 != 0 {
											fTemp23 = F32::tanh(0.2 * self.fRec10[1]);
											fTemp24 = F32::max(0.0, F32::min(1.0, fTemp0));
											fTemp25 = F32::max(0.0, *input11);
											fTemp26 = 39.0 * NoiseDsp_faustpower2_f(fTemp24) + 1.0;
										}
											if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
												fTemp27 = F32::max(0.0, F32::min(1.0, *input7));
												fTemp28 = F32::max(0.0, F32::min(1.0, 6.6666665 * (fTemp27 + -0.85)));
												iTemp29 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec14[1], 12345));
												self.iRec14[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp29, 12345));
												iRec15 = iTemp29;
												self.fRec13[0] = F32::max(self.fConst3 * self.fRec13[1], (((0.5 * (4.656613e-10 * (iRec15) as F32 + 1.0)) < (self.fConst4 * F32::powf(666.0, fTemp27))) as i32) as u32 as F32);
												fTemp30 = fTemp28 + self.fRec13[0] * (1.0 - fTemp28);
												fTemp31 = *input6;
												fTemp32 = 1.5707964 * *input5;
												fTemp33 = *input0;
												self.iRec16[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec16[1]), 12345);
												fTemp34 = 4.656613e-10 * (self.iRec16[0]) as F32;
												self.fRec17[0] = 0.5221894 * self.fRec17[3] + fTemp34 + 2.494956 * self.fRec17[1] - 2.0172658 * self.fRec17[2];
												iTemp35 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec18[1], 12345));
												iTemp36 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp35, 12345));
												self.iRec18[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp36, 12345));
												iRec19 = iTemp36;
												iRec20 = iTemp35;
												fTemp37 = 2.0 * fTemp13;
												iTemp38 = i32::wrapping_sub(1, self.iVec0[1]);
												fTemp39 = if iTemp38 != 0 {0.0} else {self.fRec22[1] + self.fConst7 * F32::max(fTemp37, 0.0)};
												self.fRec22[0] = fTemp39 - F32::floor(fTemp39);
												self.fRec21[0] = if ((self.fRec22[0] < self.fRec22[1]) as i32) | ((fTemp37 >= self.fConst6) as i32) != 0 {fTemp34} else {self.fRec21[1]};
												fTemp40 = ((fTemp33 == 4.0) as i32) as u32 as F32;
											}
												if (((fTemp40 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp40 != 0.0) as i32)) != 0 {
													fTemp41 = *input14;
													fTemp42 = 1.0 - fTemp41;
												}
													if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
														fTemp44 = ((fTemp33 == 5.0) as i32) as u32 as F32;
													}
														if (((fTemp41 != 0.0) as i32) & (((fTemp40 != 0.0) as i32) & ((fTemp3 != 0.0) as i32))) | ((((fTemp44 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | ((((fTemp40 != 0.0) as i32) & (((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32))) | ((((fTemp1 != 0.0) as i32) & (((fTemp40 != 0.0) as i32) & ((fTemp42 != 0.0) as i32))) | ((((fTemp41 != 0.0) as i32) & (((fTemp1 != 0.0) as i32) & ((fTemp40 != 0.0) as i32))) | (((fTemp1 != 0.0) as i32) & ((fTemp44 != 0.0) as i32)))))) != 0 {
															fTemp43 = *input1;
														}
															if (((fTemp40 != 0.0) as i32) & (((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32))) | (((fTemp1 != 0.0) as i32) & (((fTemp40 != 0.0) as i32) & ((fTemp42 != 0.0) as i32))) != 0 {
																fTemp45 = F32::max(self.fConst7 * F32::min(3.897 * fTemp43, self.fConst1), 1e-06);
																fTemp46 = if iTemp38 != 0 {0.0} else {self.fRec23[1] + self.fConst8 * fTemp43};
																self.fRec23[0] = fTemp46 - F32::floor(fTemp46);
																fTemp47 = F32::max(self.fConst7 * F32::min(2.63 * fTemp43, self.fConst1), 1e-06);
																fTemp48 = if iTemp38 != 0 {0.0} else {self.fRec24[1] + self.fConst9 * fTemp43};
																self.fRec24[0] = fTemp48 - F32::floor(fTemp48);
																fTemp49 = F32::max(self.fConst7 * F32::min(2.546 * fTemp43, self.fConst1), 1e-06);
																fTemp50 = if iTemp38 != 0 {0.0} else {self.fRec25[1] + self.fConst10 * fTemp43};
																self.fRec25[0] = fTemp50 - F32::floor(fTemp50);
																fTemp51 = F32::max(self.fConst7 * F32::min(1.8 * fTemp43, self.fConst1), 1e-06);
																fTemp52 = if iTemp38 != 0 {0.0} else {self.fRec26[1] + self.fConst11 * fTemp43};
																self.fRec26[0] = fTemp52 - F32::floor(fTemp52);
																fTemp53 = F32::max(self.fConst7 * F32::min(1.483 * fTemp43, self.fConst1), 1e-06);
																fTemp54 = if iTemp38 != 0 {0.0} else {self.fRec27[1] + self.fConst12 * fTemp43};
																self.fRec27[0] = fTemp54 - F32::floor(fTemp54);
																fTemp55 = F32::max(self.fConst7 * F32::min(fTemp43, self.fConst1), 1e-06);
																fTemp56 = if iTemp38 != 0 {0.0} else {self.fRec28[1] + self.fConst7 * fTemp43};
																self.fRec28[0] = fTemp56 - F32::floor(fTemp56);
																fTemp57 = self.fRec28[0] + -1.0;
																fTemp58 = 1.0 - fTemp55;
																fTemp59 = self.fRec27[0] + -1.0;
																fTemp60 = 1.0 - fTemp53;
																fTemp61 = self.fRec26[0] + -1.0;
																fTemp62 = 1.0 - fTemp51;
																fTemp63 = self.fRec25[0] + -1.0;
																fTemp64 = 1.0 - fTemp49;
																fTemp65 = self.fRec24[0] + -1.0;
																fTemp66 = 1.0 - fTemp47;
																fTemp67 = self.fRec23[0] + -1.0;
																fTemp68 = 1.0 - fTemp45;
																fTemp69 = F32::floor(self.fRec28[0] + 0.5);
																fTemp70 = self.fRec28[0] + (0.5 - fTemp69);
																fTemp71 = self.fRec28[0] + (-0.5 - fTemp69);
																fTemp72 = F32::floor(self.fRec27[0] + 0.5);
																fTemp73 = self.fRec27[0] + (0.5 - fTemp72);
																fTemp74 = self.fRec27[0] + (-0.5 - fTemp72);
																fTemp75 = F32::floor(self.fRec26[0] + 0.5);
																fTemp76 = self.fRec26[0] + (0.5 - fTemp75);
																fTemp77 = self.fRec26[0] + (-0.5 - fTemp75);
																fTemp78 = F32::floor(self.fRec25[0] + 0.5);
																fTemp79 = self.fRec25[0] + (0.5 - fTemp78);
																fTemp80 = self.fRec25[0] + (-0.5 - fTemp78);
																fTemp81 = F32::floor(self.fRec24[0] + 0.5);
																fTemp82 = self.fRec24[0] + (0.5 - fTemp81);
																fTemp83 = self.fRec24[0] + (-0.5 - fTemp81);
																fTemp84 = F32::floor(self.fRec23[0] + 0.5);
																fTemp85 = self.fRec23[0] + (0.5 - fTemp84);
																fTemp86 = self.fRec23[0] + (-0.5 - fTemp84);
																fTemp87 = 0.23333333 * fTemp42 * (((fTemp85 > fTemp68) as i32) as u32 as F32 * (fTemp86 * (fTemp86 / fTemp45 + 2.0) / fTemp45 + 1.0) + ((fTemp85 < fTemp45) as i32) as u32 as F32 * (fTemp85 * (2.0 - fTemp85 / fTemp45) / fTemp45 + -1.0) + ((fTemp82 > fTemp66) as i32) as u32 as F32 * (fTemp83 * (fTemp83 / fTemp47 + 2.0) / fTemp47 + 1.0) + ((fTemp82 < fTemp47) as i32) as u32 as F32 * (fTemp82 * (2.0 - fTemp82 / fTemp47) / fTemp47 + -1.0) + ((fTemp79 > fTemp64) as i32) as u32 as F32 * (fTemp80 * (fTemp80 / fTemp49 + 2.0) / fTemp49 + 1.0) + ((fTemp79 < fTemp49) as i32) as u32 as F32 * (fTemp79 * (2.0 - fTemp79 / fTemp49) / fTemp49 + -1.0) + ((fTemp76 > fTemp62) as i32) as u32 as F32 * (fTemp77 * (fTemp77 / fTemp51 + 2.0) / fTemp51 + 1.0) + ((fTemp76 < fTemp51) as i32) as u32 as F32 * (fTemp76 * (2.0 - fTemp76 / fTemp51) / fTemp51 + -1.0) + ((fTemp73 > fTemp60) as i32) as u32 as F32 * (fTemp74 * (fTemp74 / fTemp53 + 2.0) / fTemp53 + 1.0) + ((fTemp73 < fTemp53) as i32) as u32 as F32 * (fTemp73 * (2.0 - fTemp73 / fTemp53) / fTemp53 + -1.0) + ((fTemp70 > fTemp58) as i32) as u32 as F32 * (fTemp71 * (fTemp71 / fTemp55 + 2.0) / fTemp55 + 1.0) + ((fTemp70 < fTemp55) as i32) as u32 as F32 * (fTemp70 * (2.0 - fTemp70 / fTemp55) / fTemp55 + -1.0) + (if (self.fRec28[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec27[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec26[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec25[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec23[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec24[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 - (((self.fRec23[0] > fTemp68) as i32) as u32 as F32 * (fTemp67 * (fTemp67 / fTemp45 + 2.0) / fTemp45 + 1.0) + ((self.fRec24[0] > fTemp66) as i32) as u32 as F32 * (fTemp65 * (fTemp65 / fTemp47 + 2.0) / fTemp47 + 1.0) + ((self.fRec25[0] > fTemp64) as i32) as u32 as F32 * (fTemp63 * (fTemp63 / fTemp49 + 2.0) / fTemp49 + 1.0) + ((self.fRec26[0] > fTemp62) as i32) as u32 as F32 * (fTemp61 * (fTemp61 / fTemp51 + 2.0) / fTemp51 + 1.0) + ((self.fRec27[0] > fTemp60) as i32) as u32 as F32 * (fTemp59 * (fTemp59 / fTemp53 + 2.0) / fTemp53 + 1.0) + ((self.fRec28[0] < fTemp55) as i32) as u32 as F32 * (self.fRec28[0] * (2.0 - self.fRec28[0] / fTemp55) / fTemp55 + -1.0) + ((self.fRec28[0] > fTemp58) as i32) as u32 as F32 * (fTemp57 * (fTemp57 / fTemp55 + 2.0) / fTemp55 + 1.0) + ((self.fRec27[0] < fTemp53) as i32) as u32 as F32 * (self.fRec27[0] * (2.0 - self.fRec27[0] / fTemp53) / fTemp53 + -1.0) + ((self.fRec26[0] < fTemp51) as i32) as u32 as F32 * (self.fRec26[0] * (2.0 - self.fRec26[0] / fTemp51) / fTemp51 + -1.0) + ((self.fRec25[0] < fTemp49) as i32) as u32 as F32 * (self.fRec25[0] * (2.0 - self.fRec25[0] / fTemp49) / fTemp49 + -1.0) + ((self.fRec24[0] < fTemp47) as i32) as u32 as F32 * (self.fRec24[0] * (2.0 - self.fRec24[0] / fTemp47) / fTemp47 + -1.0) + ((self.fRec23[0] < fTemp45) as i32) as u32 as F32 * (self.fRec23[0] * (2.0 - self.fRec23[0] / fTemp45) / fTemp45 + -1.0)));
															}
																if (((fTemp41 != 0.0) as i32) & (((fTemp40 != 0.0) as i32) & ((fTemp3 != 0.0) as i32))) | (((fTemp41 != 0.0) as i32) & (((fTemp1 != 0.0) as i32) & ((fTemp40 != 0.0) as i32))) != 0 {
																	fTemp88 = F32::max(-1.0, F32::min(1.0, *input13));
																	fTemp89 = fTemp88 + 1.0;
																	iTemp90 = (fTemp88 < 0.0) as i32;
																	fTemp91 = (iTemp90) as F32;
																	fTemp92 = 0.6 * fTemp88 + 1.0;
																	fTemp93 = (i32::wrapping_sub(1, iTemp90)) as F32;
																	fTemp94 = fTemp43 * (fTemp93 * F32::powf(3.897, fTemp92) + fTemp91 * F32::exp(1.3602071 * fTemp89 - 1.7917595 * fTemp88));
																	fTemp95 = F32::max(self.fConst7 * F32::min(fTemp94, self.fConst1), 1e-06);
																	fTemp96 = if iTemp38 != 0 {0.0} else {self.fRec29[1] + self.fConst7 * fTemp94};
																	self.fRec29[0] = fTemp96 - F32::floor(fTemp96);
																	fTemp97 = fTemp43 * (fTemp93 * F32::powf(2.63, fTemp92) + fTemp91 * F32::exp(0.96698385 * fTemp89 - 1.609438 * fTemp88));
																	fTemp98 = F32::max(self.fConst7 * F32::min(fTemp97, self.fConst1), 1e-06);
																	fTemp99 = if iTemp38 != 0 {0.0} else {self.fRec30[1] + self.fConst7 * fTemp97};
																	self.fRec30[0] = fTemp99 - F32::floor(fTemp99);
																	fTemp100 = fTemp43 * (fTemp93 * F32::powf(2.546, fTemp92) + fTemp91 * F32::exp(0.9345235 * fTemp89 - 1.3862944 * fTemp88));
																	fTemp101 = F32::max(self.fConst7 * F32::min(fTemp100, self.fConst1), 1e-06);
																	fTemp102 = if iTemp38 != 0 {0.0} else {self.fRec31[1] + self.fConst7 * fTemp100};
																	self.fRec31[0] = fTemp102 - F32::floor(fTemp102);
																	fTemp103 = fTemp43 * (fTemp93 * F32::powf(1.8, fTemp92) + fTemp91 * F32::exp(0.5877867 * fTemp89 - 1.0986123 * fTemp88));
																	fTemp104 = F32::max(self.fConst7 * F32::min(fTemp103, self.fConst1), 1e-06);
																	fTemp105 = if iTemp38 != 0 {0.0} else {self.fRec32[1] + self.fConst7 * fTemp103};
																	self.fRec32[0] = fTemp105 - F32::floor(fTemp105);
																	fTemp106 = fTemp43 * (fTemp93 * F32::powf(1.483, fTemp92) + fTemp91 * F32::exp(0.39406705 * fTemp89 - 0.6931472 * fTemp88));
																	fTemp107 = F32::max(self.fConst7 * F32::min(fTemp106, self.fConst1), 1e-06);
																	fTemp108 = if iTemp38 != 0 {0.0} else {self.fRec33[1] + self.fConst7 * fTemp106};
																	self.fRec33[0] = fTemp108 - F32::floor(fTemp108);
																	fTemp109 = fTemp43 * (fTemp91 + fTemp93 * F32::powf(1.0, fTemp92));
																	fTemp110 = F32::max(self.fConst7 * F32::min(fTemp109, self.fConst1), 1e-06);
																	fTemp111 = if iTemp38 != 0 {0.0} else {self.fRec34[1] + self.fConst7 * fTemp109};
																	self.fRec34[0] = fTemp111 - F32::floor(fTemp111);
																	fTemp112 = self.fRec34[0] + -1.0;
																	fTemp113 = 1.0 - fTemp110;
																	fTemp114 = self.fRec33[0] + -1.0;
																	fTemp115 = 1.0 - fTemp107;
																	fTemp116 = self.fRec32[0] + -1.0;
																	fTemp117 = 1.0 - fTemp104;
																	fTemp118 = self.fRec31[0] + -1.0;
																	fTemp119 = 1.0 - fTemp101;
																	fTemp120 = self.fRec30[0] + -1.0;
																	fTemp121 = 1.0 - fTemp98;
																	fTemp122 = self.fRec29[0] + -1.0;
																	fTemp123 = 1.0 - fTemp95;
																	fTemp124 = F32::floor(self.fRec34[0] + 0.5);
																	fTemp125 = self.fRec34[0] + (0.5 - fTemp124);
																	fTemp126 = self.fRec34[0] + (-0.5 - fTemp124);
																	fTemp127 = F32::floor(self.fRec33[0] + 0.5);
																	fTemp128 = self.fRec33[0] + (0.5 - fTemp127);
																	fTemp129 = self.fRec33[0] + (-0.5 - fTemp127);
																	fTemp130 = F32::floor(self.fRec32[0] + 0.5);
																	fTemp131 = self.fRec32[0] + (0.5 - fTemp130);
																	fTemp132 = self.fRec32[0] + (-0.5 - fTemp130);
																	fTemp133 = F32::floor(self.fRec31[0] + 0.5);
																	fTemp134 = self.fRec31[0] + (0.5 - fTemp133);
																	fTemp135 = self.fRec31[0] + (-0.5 - fTemp133);
																	fTemp136 = F32::floor(self.fRec30[0] + 0.5);
																	fTemp137 = self.fRec30[0] + (0.5 - fTemp136);
																	fTemp138 = self.fRec30[0] + (-0.5 - fTemp136);
																	fTemp139 = F32::floor(self.fRec29[0] + 0.5);
																	fTemp140 = self.fRec29[0] + (0.5 - fTemp139);
																	fTemp141 = self.fRec29[0] + (-0.5 - fTemp139);
																	fTemp142 = 0.23333333 * fTemp41 * (((fTemp140 > fTemp123) as i32) as u32 as F32 * (fTemp141 * (fTemp141 / fTemp95 + 2.0) / fTemp95 + 1.0) + ((fTemp140 < fTemp95) as i32) as u32 as F32 * (fTemp140 * (2.0 - fTemp140 / fTemp95) / fTemp95 + -1.0) + ((fTemp137 > fTemp121) as i32) as u32 as F32 * (fTemp138 * (fTemp138 / fTemp98 + 2.0) / fTemp98 + 1.0) + ((fTemp137 < fTemp98) as i32) as u32 as F32 * (fTemp137 * (2.0 - fTemp137 / fTemp98) / fTemp98 + -1.0) + ((fTemp134 > fTemp119) as i32) as u32 as F32 * (fTemp135 * (fTemp135 / fTemp101 + 2.0) / fTemp101 + 1.0) + ((fTemp134 < fTemp101) as i32) as u32 as F32 * (fTemp134 * (2.0 - fTemp134 / fTemp101) / fTemp101 + -1.0) + ((fTemp131 > fTemp117) as i32) as u32 as F32 * (fTemp132 * (fTemp132 / fTemp104 + 2.0) / fTemp104 + 1.0) + ((fTemp131 < fTemp104) as i32) as u32 as F32 * (fTemp131 * (2.0 - fTemp131 / fTemp104) / fTemp104 + -1.0) + ((fTemp128 > fTemp115) as i32) as u32 as F32 * (fTemp129 * (fTemp129 / fTemp107 + 2.0) / fTemp107 + 1.0) + ((fTemp128 < fTemp107) as i32) as u32 as F32 * (fTemp128 * (2.0 - fTemp128 / fTemp107) / fTemp107 + -1.0) + ((fTemp125 > fTemp113) as i32) as u32 as F32 * (fTemp126 * (fTemp126 / fTemp110 + 2.0) / fTemp110 + 1.0) + ((fTemp125 < fTemp110) as i32) as u32 as F32 * (fTemp125 * (2.0 - fTemp125 / fTemp110) / fTemp110 + -1.0) + (if (self.fRec34[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec33[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec32[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec31[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec29[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 + (if (self.fRec30[0] < 0.5) as i32 != 0 {1} else {-1}) as F32 - (((self.fRec29[0] > fTemp123) as i32) as u32 as F32 * (fTemp122 * (fTemp122 / fTemp95 + 2.0) / fTemp95 + 1.0) + ((self.fRec30[0] > fTemp121) as i32) as u32 as F32 * (fTemp120 * (fTemp120 / fTemp98 + 2.0) / fTemp98 + 1.0) + ((self.fRec31[0] > fTemp119) as i32) as u32 as F32 * (fTemp118 * (fTemp118 / fTemp101 + 2.0) / fTemp101 + 1.0) + ((self.fRec32[0] > fTemp117) as i32) as u32 as F32 * (fTemp116 * (fTemp116 / fTemp104 + 2.0) / fTemp104 + 1.0) + ((self.fRec33[0] > fTemp115) as i32) as u32 as F32 * (fTemp114 * (fTemp114 / fTemp107 + 2.0) / fTemp107 + 1.0) + ((self.fRec34[0] < fTemp110) as i32) as u32 as F32 * (self.fRec34[0] * (2.0 - self.fRec34[0] / fTemp110) / fTemp110 + -1.0) + ((self.fRec34[0] > fTemp113) as i32) as u32 as F32 * (fTemp112 * (fTemp112 / fTemp110 + 2.0) / fTemp110 + 1.0) + ((self.fRec33[0] < fTemp107) as i32) as u32 as F32 * (self.fRec33[0] * (2.0 - self.fRec33[0] / fTemp107) / fTemp107 + -1.0) + ((self.fRec32[0] < fTemp104) as i32) as u32 as F32 * (self.fRec32[0] * (2.0 - self.fRec32[0] / fTemp104) / fTemp104 + -1.0) + ((self.fRec31[0] < fTemp101) as i32) as u32 as F32 * (self.fRec31[0] * (2.0 - self.fRec31[0] / fTemp101) / fTemp101 + -1.0) + ((self.fRec30[0] < fTemp98) as i32) as u32 as F32 * (self.fRec30[0] * (2.0 - self.fRec30[0] / fTemp98) / fTemp98 + -1.0) + ((self.fRec29[0] < fTemp95) as i32) as u32 as F32 * (self.fRec29[0] * (2.0 - self.fRec29[0] / fTemp95) / fTemp95 + -1.0)));
																}
																	if (((fTemp40 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp40 != 0.0) as i32)) != 0 {
																		fTemp143 = fTemp40 * (fTemp87 + fTemp41 * (fTemp142 - fTemp87));
																	}
																		if (((fTemp44 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp44 != 0.0) as i32)) != 0 {
																			fTemp144 = F32::max(1.0, F32::min(self.fConst13, fTemp43));
																			fTemp145 = if iTemp38 != 0 {0.0} else {self.fRec37[1] + self.fConst7 * fTemp144};
																			self.fRec37[0] = fTemp145 - F32::floor(fTemp145);
																		}
																			if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
																				fTemp146 = ((fTemp33 == 6.0) as i32) as u32 as F32;
																			}
																				if (((fTemp44 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | ((((fTemp146 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | ((((fTemp1 != 0.0) as i32) & ((fTemp44 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp146 != 0.0) as i32)))) != 0 {
																					iTemp147 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec38[1], 12345));
																					iTemp148 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp147, 12345));
																					iTemp149 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp148, 12345));
																					self.iRec38[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp149, 12345));
																					iRec39 = iTemp149;
																					iRec40 = iTemp148;
																					iRec41 = iTemp147;
																				}
																					if (((fTemp44 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp44 != 0.0) as i32)) != 0 {
																						self.fRec36[0] = if (self.fRec37[0] < self.fRec37[1]) as i32 != 0 {4.656613e-10 * (self.iRec38[0]) as F32} else {self.fRec36[1]};
																						iTemp150 = ((self.fRec37[0] < 0.3) as i32) * ((self.fRec36[0] > -0.6) as i32);
																						fTemp151 = if iTemp38 != 0 {0.0} else {self.fRec44[1] + self.fConst14 * fTemp144};
																						self.fRec44[0] = fTemp151 - F32::floor(fTemp151);
																						self.fRec43[0] = if (self.fRec44[0] < self.fRec44[1]) as i32 != 0 {4.656613e-10 * (iRec39) as F32} else {self.fRec43[1]};
																						fTemp152 = if iTemp38 != 0 {0.0} else {self.fRec42[1] + self.fConst7 * F32::max(2e+01, F32::min(self.fConst1, fTemp13 * F32::exp(0.1732868 * (F32::floor(F32::max(0.0, F32::min(3.999, 2.0 * (self.fRec43[0] + 1.0)))) + -1.5)) * (0.3 * NoiseDsp_faustpower2_f(F32::max(0.0, F32::min(1.0, 1.0 - 3.3333333 * self.fRec37[0]))) + 1.0)))};
																						self.fRec42[0] = fTemp152 - F32::floor(fTemp152);
																						fTemp153 = F32::sin(6.2831855 * self.fRec42[0]);
																						fTemp154 = F32::tanh(3.0 * (fTemp153 + 0.8 * F32::abs(fTemp153))) * (iTemp150) as F32 + 5.5879353e-11 * (iRec40) as F32 * (i32::wrapping_sub(1, iTemp150)) as F32;
																						self.fVec1[0] = fTemp154;
																						self.fRec35[0] = -(self.fConst17 * (self.fConst16 * self.fRec35[1] - self.fConst15 * (fTemp154 - self.fVec1[1])));
																						fTemp155 = 1.7 * self.fRec35[0] * fTemp44;
																					}
																						if (((fTemp146 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp146 != 0.0) as i32)) != 0 {
																							fTemp156 = (iRec41) as F32;
																							self.fVec2[0] = fTemp156;
																							self.fRec47[0] = self.fConst21 * (4.656613e-10 * (fTemp156 + self.fVec2[1]) - self.fConst20 * self.fRec47[1]);
																							self.fRec46[0] = ((self.fRec47[0] >= 0.0) as i32) as u32 as F32 * (self.fRec46[1] + self.fConst18 * (1.0 - self.fRec46[1]));
																							self.fRec45[0] = -(self.fConst17 * (self.fConst16 * self.fRec45[1] - self.fConst15 * (self.fRec46[0] - self.fRec46[1])));
																							fTemp157 = 1.67 * self.fRec45[0] * fTemp146;
																						}
																							if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
																								fTemp158 = F32::cos(fTemp32) * if (fTemp33 >= 4.0) as i32 != 0 {if (fTemp33 >= 6.0) as i32 != 0 {fTemp157} else {if (fTemp33 >= 5.0) as i32 != 0 {fTemp155} else {fTemp143}}} else {if (fTemp33 >= 2.0) as i32 != 0 {if (fTemp33 >= 3.0) as i32 != 0 {0.00048828125 * F32::floor(2048.0 * self.fRec21[0] + 0.5)} else {F32::tanh(1.7229468e-09 * (iRec20) as F32 * (((0.5 * (4.656613e-10 * (iRec19) as F32 + 1.0)) < self.fConst5) as i32) as u32 as F32)}} else {if (fTemp33 >= 1.0) as i32 != 0 {11.6 * (0.049922034 * self.fRec17[0] + 0.0506127 * self.fRec17[2] - (0.095993534 * self.fRec17[1] + 0.004408786 * self.fRec17[3]))} else {fTemp34}}} + 4.0 * sl_loop((*input3) as i32, *input4) * F32::sin(fTemp32);
																								fTemp159 = F32::max(0.0, F32::min(1.0, fTemp31));
																								fTemp160 = F32::exp(0.6931472 * (F32::max(1.0, F32::min(16.0, 16.0 - 12.0 * fTemp159)) + -1.0));
																								fTemp161 = self.fConst0 / (23.0 * NoiseDsp_faustpower2_f(fTemp159) + 1.0);
																								fTemp162 = if iTemp38 != 0 {0.0} else {self.fRec49[1] + self.fConst7 * F32::max(fTemp161, 0.0)};
																								self.fRec49[0] = fTemp162 - F32::floor(fTemp162);
																								self.fRec48[0] = if ((self.fRec49[0] < self.fRec49[1]) as i32) | ((fTemp161 >= self.fConst6) as i32) != 0 {fTemp158} else {self.fRec48[1]};
																								fTemp163 = if (fTemp31 > 0.0001) as i32 != 0 {F32::floor(self.fRec48[0] * fTemp160 + 0.5) / fTemp160} else {fTemp158};
																							}
																								if (fTemp1 != 0.0) as i32 != 0 {
																									fTemp164 = fTemp25 * fTemp163 * fTemp30 * fTemp26;
																									fTemp165 = (F32::min(fTemp164, 0.0) + (1.0 - F32::exp(-(F32::max(fTemp164, 0.0))))) * (0.75 * fTemp24 * F32::min(1.0, fTemp25) + 1.0) / fTemp26 + 0.2 * fTemp25 * fTemp24;
																									fTemp166 = fTemp165 - (5.0 * fTemp15 * fTemp23 + self.fRec11[1]);
																									fTemp167 = fTemp14 * fTemp166 / fTemp16;
																									fTemp168 = 5.0 * fTemp23;
																									self.fRec10[0] = fTemp168 + 2.0 * fTemp167;
																									fTemp169 = fTemp168 + fTemp167;
																									fTemp170 = fTemp14 * fTemp169;
																									self.fRec11[0] = self.fRec11[1] + 2.0 * fTemp170;
																									fRec12 = fTemp169;
																								}
																									if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
																										fTemp171 = 1.0 - fTemp22;
																									}
																										if (fTemp1 != 0.0) as i32 != 0 {
																											fTemp172 = fTemp171 * fTemp165 + 1.1111112 * fRec12 * fTemp22;
																											fTemp173 = fTemp172 - (5.0 * fTemp15 * fTemp21 + self.fRec8[1]);
																											fTemp174 = fTemp14 * fTemp173 / fTemp16;
																											fTemp175 = 5.0 * fTemp21;
																											self.fRec7[0] = fTemp175 + 2.0 * fTemp174;
																											fTemp176 = fTemp175 + fTemp174;
																											fTemp177 = fTemp14 * fTemp176;
																											self.fRec8[0] = self.fRec8[1] + 2.0 * fTemp177;
																											fRec9 = fTemp176;
																										}
																											if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
																												fTemp178 = 1.0 - fTemp20;
																											}
																												if (fTemp1 != 0.0) as i32 != 0 {
																													fTemp179 = fTemp178 * fTemp172 + 1.1111112 * fRec9 * fTemp20;
																													fTemp180 = fTemp179 - (5.0 * fTemp15 * fTemp19 + self.fRec5[1]);
																													fTemp181 = fTemp14 * fTemp180 / fTemp16;
																													fTemp182 = 5.0 * fTemp19;
																													self.fRec4[0] = fTemp182 + 2.0 * fTemp181;
																													fTemp183 = fTemp182 + fTemp181;
																													fTemp184 = fTemp14 * fTemp183;
																													self.fRec5[0] = self.fRec5[1] + 2.0 * fTemp184;
																													fRec6 = fTemp183;
																												}
																													if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
																														fTemp185 = 1.0 - fTemp18;
																													}
																														if (fTemp1 != 0.0) as i32 != 0 {
																															fTemp186 = fTemp185 * fTemp179 + 1.1111112 * fRec6 * fTemp18;
																															fTemp187 = fTemp186 - (5.0 * fTemp15 * fTemp17 + self.fRec2[1]);
																															fTemp188 = fTemp14 * fTemp187 / fTemp16;
																															fTemp189 = 5.0 * fTemp17;
																															self.fRec1[0] = fTemp189 + 2.0 * fTemp188;
																															fTemp190 = fTemp189 + fTemp188;
																															fTemp191 = fTemp14 * fTemp190;
																															self.fRec2[0] = self.fRec2[1] + 2.0 * fTemp191;
																															fRec3 = fTemp190;
																														}
																															if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
																																fTemp192 = 1.0 - fTemp12;
																																fTemp193 = F32::powf(1e+01, 0.05 * (12.0 * F32::powf(F32::max(0.0, F32::min(1.0, fTemp10)), 0.35) + -4.4));
																															}
																																if (fTemp1 != 0.0) as i32 != 0 {
																																	self.fRec0[0] = fTemp193 * (fTemp192 * fTemp186 + 1.1111112 * fRec3 * fTemp12) - (fTemp9 * self.fRec0[2] + 2.0 * fTemp8 * self.fRec0[1]) / fTemp5;
																																	fTemp194 = *input12 * fTemp1 * (self.fRec0[0] + self.fRec0[2] - 2.0 * self.fRec0[1]) / fTemp7;
																																}
																																	if (fTemp3 != 0.0) as i32 != 0 {
																																		fTemp195 = F32::tanh(0.2 * self.fRec51[1]);
																																		fTemp196 = F32::tanh(0.2 * self.fRec54[1]);
																																		fTemp197 = F32::tanh(0.2 * self.fRec57[1]);
																																		fTemp198 = F32::tanh(0.2 * self.fRec60[1]);
																																		fTemp199 = fTemp163 * fTemp30;
																																		fTemp200 = fTemp199 - (5.0 * fTemp15 * fTemp198 + self.fRec61[1]);
																																		fTemp201 = fTemp14 * fTemp200 / fTemp16;
																																		fTemp202 = 5.0 * fTemp198;
																																		self.fRec60[0] = fTemp202 + 2.0 * fTemp201;
																																		fTemp203 = fTemp202 + fTemp201;
																																		fTemp204 = fTemp14 * fTemp203;
																																		self.fRec61[0] = self.fRec61[1] + 2.0 * fTemp204;
																																		fRec62 = fTemp203;
																																		fTemp205 = fTemp199 * fTemp171 + 1.1111112 * fRec62 * fTemp22;
																																		fTemp206 = fTemp205 - (5.0 * fTemp15 * fTemp197 + self.fRec58[1]);
																																		fTemp207 = fTemp14 * fTemp206 / fTemp16;
																																		fTemp208 = 5.0 * fTemp197;
																																		self.fRec57[0] = fTemp208 + 2.0 * fTemp207;
																																		fTemp209 = fTemp208 + fTemp207;
																																		fTemp210 = fTemp14 * fTemp209;
																																		self.fRec58[0] = self.fRec58[1] + 2.0 * fTemp210;
																																		fRec59 = fTemp209;
																																		fTemp211 = fTemp205 * fTemp178 + 1.1111112 * fRec59 * fTemp20;
																																		fTemp212 = fTemp211 - (5.0 * fTemp15 * fTemp196 + self.fRec55[1]);
																																		fTemp213 = fTemp14 * fTemp212 / fTemp16;
																																		fTemp214 = 5.0 * fTemp196;
																																		self.fRec54[0] = fTemp214 + 2.0 * fTemp213;
																																		fTemp215 = fTemp214 + fTemp213;
																																		fTemp216 = fTemp14 * fTemp215;
																																		self.fRec55[0] = self.fRec55[1] + 2.0 * fTemp216;
																																		fRec56 = fTemp215;
																																		fTemp217 = fTemp211 * fTemp185 + 1.1111112 * fRec56 * fTemp18;
																																		fTemp218 = fTemp217 - (5.0 * fTemp15 * fTemp195 + self.fRec52[1]);
																																		fTemp219 = fTemp14 * fTemp218 / fTemp16;
																																		fTemp220 = 5.0 * fTemp195;
																																		self.fRec51[0] = fTemp220 + 2.0 * fTemp219;
																																		fTemp221 = fTemp220 + fTemp219;
																																		fTemp222 = fTemp14 * fTemp221;
																																		self.fRec52[0] = self.fRec52[1] + 2.0 * fTemp222;
																																		fRec53 = fTemp221;
																																		self.fRec50[0] = (fTemp217 * fTemp192 + 1.1111112 * fRec53 * fTemp12) * fTemp193 - (self.fRec50[2] * fTemp9 + 2.0 * self.fRec50[1] * fTemp8) / fTemp5;
																																		fTemp223 = *input15 * fTemp3 * (self.fRec50[0] + self.fRec50[2] - 2.0 * self.fRec50[1]) / fTemp7;
																																	}
																																		*output0 = fTemp223 + fTemp194;
																																		if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
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
																																		}
																																			if (((fTemp40 != 0.0) as i32) & (((fTemp42 != 0.0) as i32) & ((fTemp3 != 0.0) as i32))) | (((fTemp1 != 0.0) as i32) & (((fTemp40 != 0.0) as i32) & ((fTemp42 != 0.0) as i32))) != 0 {
																																				self.fRec23[1] = self.fRec23[0];
																																				self.fRec24[1] = self.fRec24[0];
																																				self.fRec25[1] = self.fRec25[0];
																																				self.fRec26[1] = self.fRec26[0];
																																				self.fRec27[1] = self.fRec27[0];
																																				self.fRec28[1] = self.fRec28[0];
																																			}
																																				if (((fTemp41 != 0.0) as i32) & (((fTemp40 != 0.0) as i32) & ((fTemp3 != 0.0) as i32))) | (((fTemp41 != 0.0) as i32) & (((fTemp1 != 0.0) as i32) & ((fTemp40 != 0.0) as i32))) != 0 {
																																					self.fRec29[1] = self.fRec29[0];
																																					self.fRec30[1] = self.fRec30[0];
																																					self.fRec31[1] = self.fRec31[0];
																																					self.fRec32[1] = self.fRec32[0];
																																					self.fRec33[1] = self.fRec33[0];
																																					self.fRec34[1] = self.fRec34[0];
																																				}
																																					if (((fTemp44 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp44 != 0.0) as i32)) != 0 {
																																						self.fRec37[1] = self.fRec37[0];
																																					}
																																						if (((fTemp44 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | ((((fTemp146 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | ((((fTemp1 != 0.0) as i32) & ((fTemp44 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp146 != 0.0) as i32)))) != 0 {
																																							self.iRec38[1] = self.iRec38[0];
																																						}
																																							if (((fTemp44 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp44 != 0.0) as i32)) != 0 {
																																								self.fRec36[1] = self.fRec36[0];
																																								self.fRec44[1] = self.fRec44[0];
																																								self.fRec43[1] = self.fRec43[0];
																																								self.fRec42[1] = self.fRec42[0];
																																								self.fVec1[1] = self.fVec1[0];
																																								self.fRec35[1] = self.fRec35[0];
																																							}
																																								if (((fTemp146 != 0.0) as i32) & ((fTemp3 != 0.0) as i32)) | (((fTemp1 != 0.0) as i32) & ((fTemp146 != 0.0) as i32)) != 0 {
																																									self.fVec2[1] = self.fVec2[0];
																																									self.fRec47[1] = self.fRec47[0];
																																									self.fRec46[1] = self.fRec46[0];
																																									self.fRec45[1] = self.fRec45[0];
																																								}
																																									if ((fTemp3 != 0.0) as i32) | ((fTemp1 != 0.0) as i32) != 0 {
																																										self.fRec49[1] = self.fRec49[0];
																																										self.fRec48[1] = self.fRec48[0];
																																									}
																																										if (fTemp1 != 0.0) as i32 != 0 {
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
																																											if (fTemp3 != 0.0) as i32 != 0 {
																																												self.fRec60[1] = self.fRec60[0];
																																												self.fRec61[1] = self.fRec61[0];
																																												self.fRec57[1] = self.fRec57[0];
																																												self.fRec58[1] = self.fRec58[0];
																																												self.fRec54[1] = self.fRec54[0];
																																												self.fRec55[1] = self.fRec55[0];
																																												self.fRec51[1] = self.fRec51[0];
																																												self.fRec52[1] = self.fRec52[0];
																																												self.fRec50[2] = self.fRec50[1];
																																												self.fRec50[1] = self.fRec50[0];
																																											}
																																											}
																																											self.fTempPerm0 = fTemp2;
																																											self.fTempPerm1 = fTemp4;
																																											self.fTempPerm2 = fTemp5;
																																											self.fTempPerm3 = fTemp6;
																																											self.fTempPerm4 = fTemp7;
																																											self.fTempPerm5 = fTemp8;
																																											self.fTempPerm6 = fTemp9;
																																											self.fTempPerm7 = fTemp10;
																																											self.fTempPerm8 = fTemp11;
																																											self.fTempPerm9 = fTemp12;
																																											self.fTempPerm10 = fTemp13;
																																											self.fTempPerm11 = fTemp14;
																																											self.fTempPerm12 = fTemp15;
																																											self.fTempPerm13 = fTemp16;
																																											self.fTempPerm14 = fTemp17;
																																											self.fTempPerm15 = fTemp18;
																																											self.fTempPerm16 = fTemp19;
																																											self.fTempPerm17 = fTemp20;
																																											self.fTempPerm18 = fTemp21;
																																											self.fTempPerm19 = fTemp22;
																																											self.fTempPerm20 = fTemp23;
																																											self.fTempPerm21 = fTemp24;
																																											self.fTempPerm22 = fTemp25;
																																											self.fTempPerm23 = fTemp26;
																																											self.fTempPerm24 = fTemp27;
																																											self.fTempPerm25 = fTemp28;
																																											self.iTempPerm26 = iTemp29;
																																											self.fTempPerm27 = fTemp30;
																																											self.fTempPerm28 = fTemp31;
																																											self.fTempPerm29 = fTemp32;
																																											self.fTempPerm30 = fTemp33;
																																											self.fTempPerm31 = fTemp34;
																																											self.iTempPerm32 = iTemp35;
																																											self.iTempPerm33 = iTemp36;
																																											self.fTempPerm34 = fTemp37;
																																											self.iTempPerm35 = iTemp38;
																																											self.fTempPerm36 = fTemp39;
																																											self.fTempPerm37 = fTemp40;
																																											self.fTempPerm38 = fTemp41;
																																											self.fTempPerm39 = fTemp42;
																																											self.fTempPerm40 = fTemp44;
																																											self.fTempPerm41 = fTemp43;
																																											self.fTempPerm42 = fTemp45;
																																											self.fTempPerm43 = fTemp46;
																																											self.fTempPerm44 = fTemp47;
																																											self.fTempPerm45 = fTemp48;
																																											self.fTempPerm46 = fTemp49;
																																											self.fTempPerm47 = fTemp50;
																																											self.fTempPerm48 = fTemp51;
																																											self.fTempPerm49 = fTemp52;
																																											self.fTempPerm50 = fTemp53;
																																											self.fTempPerm51 = fTemp54;
																																											self.fTempPerm52 = fTemp55;
																																											self.fTempPerm53 = fTemp56;
																																											self.fTempPerm54 = fTemp57;
																																											self.fTempPerm55 = fTemp58;
																																											self.fTempPerm56 = fTemp59;
																																											self.fTempPerm57 = fTemp60;
																																											self.fTempPerm58 = fTemp61;
																																											self.fTempPerm59 = fTemp62;
																																											self.fTempPerm60 = fTemp63;
																																											self.fTempPerm61 = fTemp64;
																																											self.fTempPerm62 = fTemp65;
																																											self.fTempPerm63 = fTemp66;
																																											self.fTempPerm64 = fTemp67;
																																											self.fTempPerm65 = fTemp68;
																																											self.fTempPerm66 = fTemp69;
																																											self.fTempPerm67 = fTemp70;
																																											self.fTempPerm68 = fTemp71;
																																											self.fTempPerm69 = fTemp72;
																																											self.fTempPerm70 = fTemp73;
																																											self.fTempPerm71 = fTemp74;
																																											self.fTempPerm72 = fTemp75;
																																											self.fTempPerm73 = fTemp76;
																																											self.fTempPerm74 = fTemp77;
																																											self.fTempPerm75 = fTemp78;
																																											self.fTempPerm76 = fTemp79;
																																											self.fTempPerm77 = fTemp80;
																																											self.fTempPerm78 = fTemp81;
																																											self.fTempPerm79 = fTemp82;
																																											self.fTempPerm80 = fTemp83;
																																											self.fTempPerm81 = fTemp84;
																																											self.fTempPerm82 = fTemp85;
																																											self.fTempPerm83 = fTemp86;
																																											self.fTempPerm84 = fTemp87;
																																											self.fTempPerm85 = fTemp88;
																																											self.fTempPerm86 = fTemp89;
																																											self.iTempPerm87 = iTemp90;
																																											self.fTempPerm88 = fTemp91;
																																											self.fTempPerm89 = fTemp92;
																																											self.fTempPerm90 = fTemp93;
																																											self.fTempPerm91 = fTemp94;
																																											self.fTempPerm92 = fTemp95;
																																											self.fTempPerm93 = fTemp96;
																																											self.fTempPerm94 = fTemp97;
																																											self.fTempPerm95 = fTemp98;
																																											self.fTempPerm96 = fTemp99;
																																											self.fTempPerm97 = fTemp100;
																																											self.fTempPerm98 = fTemp101;
																																											self.fTempPerm99 = fTemp102;
																																											self.fTempPerm100 = fTemp103;
																																											self.fTempPerm101 = fTemp104;
																																											self.fTempPerm102 = fTemp105;
																																											self.fTempPerm103 = fTemp106;
																																											self.fTempPerm104 = fTemp107;
																																											self.fTempPerm105 = fTemp108;
																																											self.fTempPerm106 = fTemp109;
																																											self.fTempPerm107 = fTemp110;
																																											self.fTempPerm108 = fTemp111;
																																											self.fTempPerm109 = fTemp112;
																																											self.fTempPerm110 = fTemp113;
																																											self.fTempPerm111 = fTemp114;
																																											self.fTempPerm112 = fTemp115;
																																											self.fTempPerm113 = fTemp116;
																																											self.fTempPerm114 = fTemp117;
																																											self.fTempPerm115 = fTemp118;
																																											self.fTempPerm116 = fTemp119;
																																											self.fTempPerm117 = fTemp120;
																																											self.fTempPerm118 = fTemp121;
																																											self.fTempPerm119 = fTemp122;
																																											self.fTempPerm120 = fTemp123;
																																											self.fTempPerm121 = fTemp124;
																																											self.fTempPerm122 = fTemp125;
																																											self.fTempPerm123 = fTemp126;
																																											self.fTempPerm124 = fTemp127;
																																											self.fTempPerm125 = fTemp128;
																																											self.fTempPerm126 = fTemp129;
																																											self.fTempPerm127 = fTemp130;
																																											self.fTempPerm128 = fTemp131;
																																											self.fTempPerm129 = fTemp132;
																																											self.fTempPerm130 = fTemp133;
																																											self.fTempPerm131 = fTemp134;
																																											self.fTempPerm132 = fTemp135;
																																											self.fTempPerm133 = fTemp136;
																																											self.fTempPerm134 = fTemp137;
																																											self.fTempPerm135 = fTemp138;
																																											self.fTempPerm136 = fTemp139;
																																											self.fTempPerm137 = fTemp140;
																																											self.fTempPerm138 = fTemp141;
																																											self.fTempPerm139 = fTemp142;
																																											self.fTempPerm140 = fTemp143;
																																											self.fTempPerm141 = fTemp144;
																																											self.fTempPerm142 = fTemp145;
																																											self.fTempPerm143 = fTemp146;
																																											self.iTempPerm144 = iTemp147;
																																											self.iTempPerm145 = iTemp148;
																																											self.iTempPerm146 = iTemp149;
																																											self.iTempPerm147 = iTemp150;
																																											self.fTempPerm148 = fTemp151;
																																											self.fTempPerm149 = fTemp152;
																																											self.fTempPerm150 = fTemp153;
																																											self.fTempPerm151 = fTemp154;
																																											self.fTempPerm152 = fTemp155;
																																											self.fTempPerm153 = fTemp156;
																																											self.fTempPerm154 = fTemp157;
																																											self.fTempPerm155 = fTemp158;
																																											self.fTempPerm156 = fTemp159;
																																											self.fTempPerm157 = fTemp160;
																																											self.fTempPerm158 = fTemp161;
																																											self.fTempPerm159 = fTemp162;
																																											self.fTempPerm160 = fTemp163;
																																											self.fTempPerm161 = fTemp164;
																																											self.fTempPerm162 = fTemp165;
																																											self.fTempPerm163 = fTemp166;
																																											self.fTempPerm164 = fTemp167;
																																											self.fTempPerm165 = fTemp168;
																																											self.fTempPerm166 = fTemp169;
																																											self.fTempPerm167 = fTemp170;
																																											self.fTempPerm168 = fTemp171;
																																											self.fTempPerm169 = fTemp172;
																																											self.fTempPerm170 = fTemp173;
																																											self.fTempPerm171 = fTemp174;
																																											self.fTempPerm172 = fTemp175;
																																											self.fTempPerm173 = fTemp176;
																																											self.fTempPerm174 = fTemp177;
																																											self.fTempPerm175 = fTemp178;
																																											self.fTempPerm176 = fTemp179;
																																											self.fTempPerm177 = fTemp180;
																																											self.fTempPerm178 = fTemp181;
																																											self.fTempPerm179 = fTemp182;
																																											self.fTempPerm180 = fTemp183;
																																											self.fTempPerm181 = fTemp184;
																																											self.fTempPerm182 = fTemp185;
																																											self.fTempPerm183 = fTemp186;
																																											self.fTempPerm184 = fTemp187;
																																											self.fTempPerm185 = fTemp188;
																																											self.fTempPerm186 = fTemp189;
																																											self.fTempPerm187 = fTemp190;
																																											self.fTempPerm188 = fTemp191;
																																											self.fTempPerm189 = fTemp192;
																																											self.fTempPerm190 = fTemp193;
																																											self.fTempPerm191 = fTemp194;
																																											self.fTempPerm192 = fTemp195;
																																											self.fTempPerm193 = fTemp196;
																																											self.fTempPerm194 = fTemp197;
																																											self.fTempPerm195 = fTemp198;
																																											self.fTempPerm196 = fTemp199;
																																											self.fTempPerm197 = fTemp200;
																																											self.fTempPerm198 = fTemp201;
																																											self.fTempPerm199 = fTemp202;
																																											self.fTempPerm200 = fTemp203;
																																											self.fTempPerm201 = fTemp204;
																																											self.fTempPerm202 = fTemp205;
																																											self.fTempPerm203 = fTemp206;
																																											self.fTempPerm204 = fTemp207;
																																											self.fTempPerm205 = fTemp208;
																																											self.fTempPerm206 = fTemp209;
																																											self.fTempPerm207 = fTemp210;
																																											self.fTempPerm208 = fTemp211;
																																											self.fTempPerm209 = fTemp212;
																																											self.fTempPerm210 = fTemp213;
																																											self.fTempPerm211 = fTemp214;
																																											self.fTempPerm212 = fTemp215;
																																											self.fTempPerm213 = fTemp216;
																																											self.fTempPerm214 = fTemp217;
																																											self.fTempPerm215 = fTemp218;
																																											self.fTempPerm216 = fTemp219;
																																											self.fTempPerm217 = fTemp220;
																																											self.fTempPerm218 = fTemp221;
																																											self.fTempPerm219 = fTemp222;
																																											self.fTempPerm220 = fTemp223;
																																										}

}

