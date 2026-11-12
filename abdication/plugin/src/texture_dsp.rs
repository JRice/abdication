/* ------------------------------------------------------------
name: "plugin-texture"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn TextureDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn TextureDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct TextureDsp {
	iVec0: [i32;2],
	fButton0: F32,
	fVec1: [F32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fCheckbox0: F32,
	fRec0: [F32;2],
	fHslider0: F32,
	fVec2: [F32;2],
	fConst2: F32,
	fHslider1: F32,
	fVec3: [F32;2],
	iTempPerm0: i32,
	iTempPerm1: i32,
	fConst3: F32,
	fRec3: [F32;2],
	fHslider2: F32,
	iRec4: [i32;2],
	iTempPerm2: i32,
	fRec2: [F32;2],
	fConst4: F32,
	fRec1: [F32;2],
	fTempPerm3: F32,
	fConst5: F32,
	iTempPerm4: i32,
	iTempPerm5: i32,
	iRec9: [i32;2],
	iTempPerm6: i32,
	iTempPerm7: i32,
	fTempPerm8: F32,
	fConst6: F32,
	fTempPerm9: F32,
	fTempPerm10: F32,
	fTempPerm11: F32,
	fTempPerm12: F32,
	iTempPerm13: i32,
	iTempPerm14: i32,
	fTempPerm15: F32,
	fTempPerm16: F32,
	iTempPerm17: i32,
	fTempPerm18: F32,
	fHslider3: F32,
	fVec4: [F32;2],
	fHslider4: F32,
	fTempPerm19: F32,
	fRec10: [F32;2],
	fRec11: [F32;2],
	fTempPerm20: F32,
	fRec8: [F32;2],
	fHslider5: F32,
	fVec5: [F32;2],
	fTempPerm21: F32,
	fRec12: [F32;2],
	fRec13: [F32;2],
	fTempPerm22: F32,
	fTempPerm23: F32,
	fConst7: F32,
	fTempPerm24: F32,
	iTempPerm25: i32,
	iTempPerm26: i32,
	fTempPerm27: F32,
	fTempPerm28: F32,
	fTempPerm29: F32,
	fTempPerm30: F32,
	fRec7: [F32;2],
	fTempPerm31: F32,
	fTempPerm32: F32,
	iTempPerm33: i32,
	fTempPerm34: F32,
	fRec14: [F32;2],
	fTempPerm35: F32,
	fVec6: [F32;2],
	fTempPerm36: F32,
	iTempPerm37: i32,
	iTempPerm38: i32,
	iTempPerm39: i32,
	fTempPerm40: F32,
	fTempPerm41: F32,
	fRec6: [F32;2],
	fConst8: F32,
	fRec5: [F32;2],
	iTempPerm42: i32,
	iTempPerm43: i32,
	fTempPerm44: F32,
	fRec15: [F32;2],
	fTempPerm45: F32,
	fTempPerm46: F32,
	fRec16: [F32;2],
	fRec17: [F32;2],
	fTempPerm47: F32,
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	fTempPerm51: F32,
	iTempPerm52: i32,
	fTempPerm53: F32,
	fHslider6: F32,
	fVec7: [F32;2],
	fTempPerm54: F32,
	fRec21: [F32;2],
	fRec22: [F32;2],
	fTempPerm55: F32,
	iTempPerm56: i32,
	iTempPerm57: i32,
	fTempPerm58: F32,
	fRec24: [F32;2],
	fRec23: [F32;2],
	fTempPerm59: F32,
	fTempPerm60: F32,
	iTempPerm61: i32,
	fTempPerm62: F32,
	fTempPerm63: F32,
	fTempPerm64: F32,
	iTempPerm65: i32,
	fTempPerm66: F32,
	fTempPerm67: F32,
	fRec25: [F32;2],
	fTempPerm68: F32,
	fTempPerm69: F32,
	fConst10: F32,
	fConst11: F32,
	fTempPerm70: F32,
	fConst12: F32,
	fTempPerm71: F32,
	fConst13: F32,
	fTempPerm72: F32,
	fTempPerm73: F32,
	iTempPerm74: i32,
	iTempPerm75: i32,
	iTempPerm76: i32,
	iTempPerm77: i32,
	fTempPerm78: F32,
	fTempPerm79: F32,
	iTempPerm80: i32,
	iTempPerm81: i32,
	fTempPerm82: F32,
	fTempPerm83: F32,
	iTempPerm84: i32,
	fTempPerm85: F32,
	fTempPerm86: F32,
	fRec29: [F32;2],
	fTempPerm87: F32,
	iTempPerm88: i32,
	iTempPerm89: i32,
	fTempPerm90: F32,
	fTempPerm91: F32,
	fTempPerm92: F32,
	fTempPerm93: F32,
	fRec28: [F32;2],
	fTempPerm94: F32,
	fTempPerm95: F32,
	iTempPerm96: i32,
	fTempPerm97: F32,
	fRec30: [F32;2],
	fTempPerm98: F32,
	fVec8: [F32;2],
	fTempPerm99: F32,
	iTempPerm100: i32,
	iTempPerm101: i32,
	iTempPerm102: i32,
	fTempPerm103: F32,
	fTempPerm104: F32,
	fRec27: [F32;2],
	fRec26: [F32;2],
	iTempPerm105: i32,
	iTempPerm106: i32,
	fTempPerm107: F32,
	fRec31: [F32;2],
	fTempPerm108: F32,
	fTempPerm109: F32,
	fTempPerm110: F32,
	iTempPerm111: i32,
	fTempPerm112: F32,
	iTempPerm113: i32,
	iTempPerm114: i32,
	fTempPerm115: F32,
	fRec36: [F32;2],
	fRec35: [F32;2],
	fTempPerm116: F32,
	fTempPerm117: F32,
	iTempPerm118: i32,
	fTempPerm119: F32,
	fTempPerm120: F32,
	fTempPerm121: F32,
	iTempPerm122: i32,
	fTempPerm123: F32,
	fTempPerm124: F32,
	fRec37: [F32;2],
	fTempPerm125: F32,
	fTempPerm126: F32,
	fTempPerm127: F32,
	fTempPerm128: F32,
	fTempPerm129: F32,
	iTempPerm130: i32,
	iTempPerm131: i32,
	iTempPerm132: i32,
	iTempPerm133: i32,
	fTempPerm134: F32,
	fTempPerm135: F32,
	iTempPerm136: i32,
	iTempPerm137: i32,
	fTempPerm138: F32,
	fTempPerm139: F32,
	iTempPerm140: i32,
	fTempPerm141: F32,
	fTempPerm142: F32,
	fRec41: [F32;2],
	fTempPerm143: F32,
	iTempPerm144: i32,
	iTempPerm145: i32,
	fTempPerm146: F32,
	fTempPerm147: F32,
	fTempPerm148: F32,
	fTempPerm149: F32,
	fRec40: [F32;2],
	fTempPerm150: F32,
	fTempPerm151: F32,
	iTempPerm152: i32,
	fTempPerm153: F32,
	fRec42: [F32;2],
	fTempPerm154: F32,
	fVec9: [F32;2],
	fTempPerm155: F32,
	iTempPerm156: i32,
	iTempPerm157: i32,
	iTempPerm158: i32,
	fTempPerm159: F32,
	fTempPerm160: F32,
	fRec39: [F32;2],
	fRec38: [F32;2],
	iTempPerm161: i32,
	iTempPerm162: i32,
	fTempPerm163: F32,
	fRec43: [F32;2],
	fTempPerm164: F32,
	fTempPerm165: F32,
	fTempPerm166: F32,
	iTempPerm167: i32,
	fTempPerm168: F32,
	iTempPerm169: i32,
	iTempPerm170: i32,
	fTempPerm171: F32,
	fRec48: [F32;2],
	fRec47: [F32;2],
	fTempPerm172: F32,
	fTempPerm173: F32,
	iTempPerm174: i32,
	fTempPerm175: F32,
	fTempPerm176: F32,
	fTempPerm177: F32,
	iTempPerm178: i32,
	fTempPerm179: F32,
	fTempPerm180: F32,
	fRec49: [F32;2],
	fTempPerm181: F32,
	fTempPerm182: F32,
	fTempPerm183: F32,
	fTempPerm184: F32,
	fTempPerm185: F32,
	iTempPerm186: i32,
	iTempPerm187: i32,
	iTempPerm188: i32,
	iTempPerm189: i32,
	fTempPerm190: F32,
	fTempPerm191: F32,
	iTempPerm192: i32,
	iTempPerm193: i32,
	fTempPerm194: F32,
	fTempPerm195: F32,
	iTempPerm196: i32,
	fTempPerm197: F32,
	fTempPerm198: F32,
	fRec53: [F32;2],
	fTempPerm199: F32,
	iTempPerm200: i32,
	iTempPerm201: i32,
	fTempPerm202: F32,
	fTempPerm203: F32,
	fTempPerm204: F32,
	fTempPerm205: F32,
	fRec52: [F32;2],
	fTempPerm206: F32,
	fTempPerm207: F32,
	iTempPerm208: i32,
	fTempPerm209: F32,
	fRec54: [F32;2],
	fTempPerm210: F32,
	fVec10: [F32;2],
	fTempPerm211: F32,
	iTempPerm212: i32,
	iTempPerm213: i32,
	iTempPerm214: i32,
	fTempPerm215: F32,
	fTempPerm216: F32,
	fRec51: [F32;2],
	fRec50: [F32;2],
	iTempPerm217: i32,
	iTempPerm218: i32,
	fTempPerm219: F32,
	fRec55: [F32;2],
	fTempPerm220: F32,
	fTempPerm221: F32,
	fTempPerm222: F32,
	iTempPerm223: i32,
	fTempPerm224: F32,
	iTempPerm225: i32,
	iTempPerm226: i32,
	fTempPerm227: F32,
	fRec60: [F32;2],
	fRec59: [F32;2],
	fTempPerm228: F32,
	fTempPerm229: F32,
	iTempPerm230: i32,
	fTempPerm231: F32,
	fTempPerm232: F32,
	fTempPerm233: F32,
	iTempPerm234: i32,
	fTempPerm235: F32,
	fTempPerm236: F32,
	fRec61: [F32;2],
	fTempPerm237: F32,
	fTempPerm238: F32,
	fTempPerm239: F32,
	fTempPerm240: F32,
	fTempPerm241: F32,
	iTempPerm242: i32,
	iTempPerm243: i32,
	iTempPerm244: i32,
	iTempPerm245: i32,
	fTempPerm246: F32,
	fTempPerm247: F32,
	iTempPerm248: i32,
	iTempPerm249: i32,
	fTempPerm250: F32,
	fTempPerm251: F32,
	iTempPerm252: i32,
	fTempPerm253: F32,
	fTempPerm254: F32,
	fRec65: [F32;2],
	fTempPerm255: F32,
	iTempPerm256: i32,
	iTempPerm257: i32,
	fTempPerm258: F32,
	fTempPerm259: F32,
	fTempPerm260: F32,
	fTempPerm261: F32,
	fRec64: [F32;2],
	fTempPerm262: F32,
	fTempPerm263: F32,
	iTempPerm264: i32,
	fTempPerm265: F32,
	fRec66: [F32;2],
	fTempPerm266: F32,
	fVec11: [F32;2],
	fTempPerm267: F32,
	iTempPerm268: i32,
	iTempPerm269: i32,
	iTempPerm270: i32,
	fTempPerm271: F32,
	fTempPerm272: F32,
	fRec63: [F32;2],
	fRec62: [F32;2],
	iTempPerm273: i32,
	iTempPerm274: i32,
	fTempPerm275: F32,
	fRec67: [F32;2],
	fTempPerm276: F32,
	fTempPerm277: F32,
	fTempPerm278: F32,
	iTempPerm279: i32,
	fTempPerm280: F32,
	iTempPerm281: i32,
	iTempPerm282: i32,
	fTempPerm283: F32,
	fRec72: [F32;2],
	fRec71: [F32;2],
	fTempPerm284: F32,
	fTempPerm285: F32,
	iTempPerm286: i32,
	fTempPerm287: F32,
	fTempPerm288: F32,
	fTempPerm289: F32,
	iTempPerm290: i32,
	fTempPerm291: F32,
	fTempPerm292: F32,
	fRec73: [F32;2],
	fTempPerm293: F32,
	fTempPerm294: F32,
	fTempPerm295: F32,
	fTempPerm296: F32,
	fTempPerm297: F32,
	iTempPerm298: i32,
	iTempPerm299: i32,
	iTempPerm300: i32,
	fTempPerm301: F32,
	fTempPerm302: F32,
	iTempPerm303: i32,
	iTempPerm304: i32,
	fTempPerm305: F32,
	fTempPerm306: F32,
	iTempPerm307: i32,
	fTempPerm308: F32,
	fTempPerm309: F32,
	fRec77: [F32;2],
	fTempPerm310: F32,
	iTempPerm311: i32,
	iTempPerm312: i32,
	fTempPerm313: F32,
	fTempPerm314: F32,
	fTempPerm315: F32,
	fTempPerm316: F32,
	fRec76: [F32;2],
	fTempPerm317: F32,
	fTempPerm318: F32,
	iTempPerm319: i32,
	fTempPerm320: F32,
	fRec78: [F32;2],
	fTempPerm321: F32,
	fVec12: [F32;2],
	fTempPerm322: F32,
	iTempPerm323: i32,
	iTempPerm324: i32,
	iTempPerm325: i32,
	fTempPerm326: F32,
	fTempPerm327: F32,
	fRec75: [F32;2],
	fRec74: [F32;2],
	iTempPerm328: i32,
	iTempPerm329: i32,
	fTempPerm330: F32,
	fRec79: [F32;2],
	fTempPerm331: F32,
	fTempPerm332: F32,
	fTempPerm333: F32,
	iTempPerm334: i32,
	fTempPerm335: F32,
	iTempPerm336: i32,
	iTempPerm337: i32,
	fTempPerm338: F32,
	fRec84: [F32;2],
	fRec83: [F32;2],
	fTempPerm339: F32,
	fTempPerm340: F32,
	iTempPerm341: i32,
	fTempPerm342: F32,
	fTempPerm343: F32,
	fTempPerm344: F32,
	iTempPerm345: i32,
	fTempPerm346: F32,
	fTempPerm347: F32,
	fRec85: [F32;2],
	fTempPerm348: F32,
	fTempPerm349: F32,
	fTempPerm350: F32,
	fTempPerm351: F32,
	fTempPerm352: F32,
	fTempPerm353: F32,
	fTempPerm354: F32,
	fRec80: [F32;2],
	fConst14: F32,
	fRec81: [F32;2],
	fTempPerm355: F32,
	fTempPerm356: F32,
	fTempPerm357: F32,
	fRec68: [F32;2],
	fRec69: [F32;2],
	fTempPerm358: F32,
	fTempPerm359: F32,
	fTempPerm360: F32,
	fRec56: [F32;2],
	fRec57: [F32;2],
	fTempPerm361: F32,
	fTempPerm362: F32,
	fTempPerm363: F32,
	fRec44: [F32;2],
	fRec45: [F32;2],
	fTempPerm364: F32,
	fTempPerm365: F32,
	fTempPerm366: F32,
	fRec32: [F32;2],
	fRec33: [F32;2],
	fTempPerm367: F32,
	fTempPerm368: F32,
	fTempPerm369: F32,
	fRec18: [F32;2],
	fRec19: [F32;2],
	fTempPerm370: F32,
	fConst15: F32,
	fConst16: F32,
	fRec86: [F32;2],
	fRec87: [F32;2],
	fTempPerm371: F32,
	fRec88: [F32;2],
	fRec89: [F32;2],
	iTempPerm372: i32,
	iRec91: [i32;2],
	iRec97: [i32;2],
	iTempPerm373: i32,
	iTempPerm374: i32,
	iTempPerm375: i32,
	iTempPerm376: i32,
	iTempPerm377: i32,
	iTempPerm378: i32,
	iTempPerm379: i32,
	iTempPerm380: i32,
	iTempPerm381: i32,
	iTempPerm382: i32,
	iTempPerm383: i32,
	iTempPerm384: i32,
	iTempPerm385: i32,
	iTempPerm386: i32,
	fTempPerm387: F32,
	fTempPerm388: F32,
	iTempPerm389: i32,
	fTempPerm390: F32,
	fTempPerm391: F32,
	iTempPerm392: i32,
	fTempPerm393: F32,
	fTempPerm394: F32,
	fTempPerm395: F32,
	fHslider7: F32,
	fVec13: [F32;2],
	fTempPerm396: F32,
	fRec98: [F32;2],
	fRec99: [F32;2],
	fTempPerm397: F32,
	fTempPerm398: F32,
	iTempPerm399: i32,
	iTempPerm400: i32,
	fTempPerm401: F32,
	iTempPerm402: i32,
	iTempPerm403: i32,
	fTempPerm404: F32,
	fTempPerm405: F32,
	fTempPerm406: F32,
	fTempPerm407: F32,
	fHslider8: F32,
	fVec14: [F32;2],
	fTempPerm408: F32,
	fRec100: [F32;2],
	fRec101: [F32;2],
	fTempPerm409: F32,
	fTempPerm410: F32,
	iTempPerm411: i32,
	fTempPerm412: F32,
	iTempPerm413: i32,
	iTempPerm414: i32,
	fTempPerm415: F32,
	fTempPerm416: F32,
	iTempPerm417: i32,
	fTempPerm418: F32,
	fConst17: F32,
	iTempPerm419: i32,
	iRec92: [i32;2],
	iRec93: [i32;2],
	iRec94: [i32;2],
	iRec95: [i32;2],
	iTempPerm420: i32,
	iTempPerm421: i32,
	iTempPerm422: i32,
	fRec90: [F32;2],
	fTempPerm423: F32,
	fTempPerm424: F32,
	fTempPerm425: F32,
	fTempPerm426: F32,
	fTempPerm427: F32,
	iTempPerm428: i32,
	iTempPerm429: i32,
	fTempPerm430: F32,
	fTempPerm431: F32,
	iTempPerm432: i32,
	fTempPerm433: F32,
	iTempPerm434: i32,
	iTempPerm435: i32,
	iTempPerm436: i32,
	iTempPerm437: i32,
	iTempPerm438: i32,
	fRec103: [F32;2],
	fTempPerm439: F32,
	iTempPerm440: i32,
	iTempPerm441: i32,
	iTempPerm442: i32,
	iTempPerm443: i32,
	fTempPerm444: F32,
	fTempPerm445: F32,
	iTempPerm446: i32,
	fTempPerm447: F32,
	fTempPerm448: F32,
	iTempPerm449: i32,
	iTempPerm450: i32,
	fTempPerm451: F32,
	fTempPerm452: F32,
	iTempPerm453: i32,
	fTempPerm454: F32,
	fTempPerm455: F32,
	fTempPerm456: F32,
	fTempPerm457: F32,
	iTempPerm458: i32,
	fTempPerm459: F32,
	fTempPerm460: F32,
	fRec104: [F32;2],
	fRec105: [F32;2],
	fTempPerm461: F32,
	fTempPerm462: F32,
	iTempPerm463: i32,
	fTempPerm464: F32,
	fTempPerm465: F32,
	fRec102: [F32;2],
	iTempPerm466: i32,
	iTempPerm467: i32,
	fRec106: [F32;2],
	fTempPerm468: F32,
	fTempPerm469: F32,
	fTempPerm470: F32,
}

impl FaustDsp for TextureDsp {
	type T = F32;
		
	fn new() -> TextureDsp { 
		TextureDsp {
			iVec0: [0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fCheckbox0: 0.0,
			fRec0: [0.0;2],
			fHslider0: 0.0,
			fVec2: [0.0;2],
			fConst2: 0.0,
			fHslider1: 0.0,
			fVec3: [0.0;2],
			iTempPerm0: 0,
			iTempPerm1: 0,
			fConst3: 0.0,
			fRec3: [0.0;2],
			fHslider2: 0.0,
			iRec4: [0;2],
			iTempPerm2: 0,
			fRec2: [0.0;2],
			fConst4: 0.0,
			fRec1: [0.0;2],
			fTempPerm3: 0.0,
			fConst5: 0.0,
			iTempPerm4: 0,
			iTempPerm5: 0,
			iRec9: [0;2],
			iTempPerm6: 0,
			iTempPerm7: 0,
			fTempPerm8: 0.0,
			fConst6: 0.0,
			fTempPerm9: 0.0,
			fTempPerm10: 0.0,
			fTempPerm11: 0.0,
			fTempPerm12: 0.0,
			iTempPerm13: 0,
			iTempPerm14: 0,
			fTempPerm15: 0.0,
			fTempPerm16: 0.0,
			iTempPerm17: 0,
			fTempPerm18: 0.0,
			fHslider3: 0.0,
			fVec4: [0.0;2],
			fHslider4: 0.0,
			fTempPerm19: 0.0,
			fRec10: [0.0;2],
			fRec11: [0.0;2],
			fTempPerm20: 0.0,
			fRec8: [0.0;2],
			fHslider5: 0.0,
			fVec5: [0.0;2],
			fTempPerm21: 0.0,
			fRec12: [0.0;2],
			fRec13: [0.0;2],
			fTempPerm22: 0.0,
			fTempPerm23: 0.0,
			fConst7: 0.0,
			fTempPerm24: 0.0,
			iTempPerm25: 0,
			iTempPerm26: 0,
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fTempPerm29: 0.0,
			fTempPerm30: 0.0,
			fRec7: [0.0;2],
			fTempPerm31: 0.0,
			fTempPerm32: 0.0,
			iTempPerm33: 0,
			fTempPerm34: 0.0,
			fRec14: [0.0;2],
			fTempPerm35: 0.0,
			fVec6: [0.0;2],
			fTempPerm36: 0.0,
			iTempPerm37: 0,
			iTempPerm38: 0,
			iTempPerm39: 0,
			fTempPerm40: 0.0,
			fTempPerm41: 0.0,
			fRec6: [0.0;2],
			fConst8: 0.0,
			fRec5: [0.0;2],
			iTempPerm42: 0,
			iTempPerm43: 0,
			fTempPerm44: 0.0,
			fRec15: [0.0;2],
			fTempPerm45: 0.0,
			fTempPerm46: 0.0,
			fRec16: [0.0;2],
			fRec17: [0.0;2],
			fTempPerm47: 0.0,
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			iTempPerm52: 0,
			fTempPerm53: 0.0,
			fHslider6: 0.0,
			fVec7: [0.0;2],
			fTempPerm54: 0.0,
			fRec21: [0.0;2],
			fRec22: [0.0;2],
			fTempPerm55: 0.0,
			iTempPerm56: 0,
			iTempPerm57: 0,
			fTempPerm58: 0.0,
			fRec24: [0.0;2],
			fRec23: [0.0;2],
			fTempPerm59: 0.0,
			fTempPerm60: 0.0,
			iTempPerm61: 0,
			fTempPerm62: 0.0,
			fTempPerm63: 0.0,
			fTempPerm64: 0.0,
			iTempPerm65: 0,
			fTempPerm66: 0.0,
			fTempPerm67: 0.0,
			fRec25: [0.0;2],
			fTempPerm68: 0.0,
			fTempPerm69: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fTempPerm70: 0.0,
			fConst12: 0.0,
			fTempPerm71: 0.0,
			fConst13: 0.0,
			fTempPerm72: 0.0,
			fTempPerm73: 0.0,
			iTempPerm74: 0,
			iTempPerm75: 0,
			iTempPerm76: 0,
			iTempPerm77: 0,
			fTempPerm78: 0.0,
			fTempPerm79: 0.0,
			iTempPerm80: 0,
			iTempPerm81: 0,
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			iTempPerm84: 0,
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			fRec29: [0.0;2],
			fTempPerm87: 0.0,
			iTempPerm88: 0,
			iTempPerm89: 0,
			fTempPerm90: 0.0,
			fTempPerm91: 0.0,
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fRec28: [0.0;2],
			fTempPerm94: 0.0,
			fTempPerm95: 0.0,
			iTempPerm96: 0,
			fTempPerm97: 0.0,
			fRec30: [0.0;2],
			fTempPerm98: 0.0,
			fVec8: [0.0;2],
			fTempPerm99: 0.0,
			iTempPerm100: 0,
			iTempPerm101: 0,
			iTempPerm102: 0,
			fTempPerm103: 0.0,
			fTempPerm104: 0.0,
			fRec27: [0.0;2],
			fRec26: [0.0;2],
			iTempPerm105: 0,
			iTempPerm106: 0,
			fTempPerm107: 0.0,
			fRec31: [0.0;2],
			fTempPerm108: 0.0,
			fTempPerm109: 0.0,
			fTempPerm110: 0.0,
			iTempPerm111: 0,
			fTempPerm112: 0.0,
			iTempPerm113: 0,
			iTempPerm114: 0,
			fTempPerm115: 0.0,
			fRec36: [0.0;2],
			fRec35: [0.0;2],
			fTempPerm116: 0.0,
			fTempPerm117: 0.0,
			iTempPerm118: 0,
			fTempPerm119: 0.0,
			fTempPerm120: 0.0,
			fTempPerm121: 0.0,
			iTempPerm122: 0,
			fTempPerm123: 0.0,
			fTempPerm124: 0.0,
			fRec37: [0.0;2],
			fTempPerm125: 0.0,
			fTempPerm126: 0.0,
			fTempPerm127: 0.0,
			fTempPerm128: 0.0,
			fTempPerm129: 0.0,
			iTempPerm130: 0,
			iTempPerm131: 0,
			iTempPerm132: 0,
			iTempPerm133: 0,
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			iTempPerm136: 0,
			iTempPerm137: 0,
			fTempPerm138: 0.0,
			fTempPerm139: 0.0,
			iTempPerm140: 0,
			fTempPerm141: 0.0,
			fTempPerm142: 0.0,
			fRec41: [0.0;2],
			fTempPerm143: 0.0,
			iTempPerm144: 0,
			iTempPerm145: 0,
			fTempPerm146: 0.0,
			fTempPerm147: 0.0,
			fTempPerm148: 0.0,
			fTempPerm149: 0.0,
			fRec40: [0.0;2],
			fTempPerm150: 0.0,
			fTempPerm151: 0.0,
			iTempPerm152: 0,
			fTempPerm153: 0.0,
			fRec42: [0.0;2],
			fTempPerm154: 0.0,
			fVec9: [0.0;2],
			fTempPerm155: 0.0,
			iTempPerm156: 0,
			iTempPerm157: 0,
			iTempPerm158: 0,
			fTempPerm159: 0.0,
			fTempPerm160: 0.0,
			fRec39: [0.0;2],
			fRec38: [0.0;2],
			iTempPerm161: 0,
			iTempPerm162: 0,
			fTempPerm163: 0.0,
			fRec43: [0.0;2],
			fTempPerm164: 0.0,
			fTempPerm165: 0.0,
			fTempPerm166: 0.0,
			iTempPerm167: 0,
			fTempPerm168: 0.0,
			iTempPerm169: 0,
			iTempPerm170: 0,
			fTempPerm171: 0.0,
			fRec48: [0.0;2],
			fRec47: [0.0;2],
			fTempPerm172: 0.0,
			fTempPerm173: 0.0,
			iTempPerm174: 0,
			fTempPerm175: 0.0,
			fTempPerm176: 0.0,
			fTempPerm177: 0.0,
			iTempPerm178: 0,
			fTempPerm179: 0.0,
			fTempPerm180: 0.0,
			fRec49: [0.0;2],
			fTempPerm181: 0.0,
			fTempPerm182: 0.0,
			fTempPerm183: 0.0,
			fTempPerm184: 0.0,
			fTempPerm185: 0.0,
			iTempPerm186: 0,
			iTempPerm187: 0,
			iTempPerm188: 0,
			iTempPerm189: 0,
			fTempPerm190: 0.0,
			fTempPerm191: 0.0,
			iTempPerm192: 0,
			iTempPerm193: 0,
			fTempPerm194: 0.0,
			fTempPerm195: 0.0,
			iTempPerm196: 0,
			fTempPerm197: 0.0,
			fTempPerm198: 0.0,
			fRec53: [0.0;2],
			fTempPerm199: 0.0,
			iTempPerm200: 0,
			iTempPerm201: 0,
			fTempPerm202: 0.0,
			fTempPerm203: 0.0,
			fTempPerm204: 0.0,
			fTempPerm205: 0.0,
			fRec52: [0.0;2],
			fTempPerm206: 0.0,
			fTempPerm207: 0.0,
			iTempPerm208: 0,
			fTempPerm209: 0.0,
			fRec54: [0.0;2],
			fTempPerm210: 0.0,
			fVec10: [0.0;2],
			fTempPerm211: 0.0,
			iTempPerm212: 0,
			iTempPerm213: 0,
			iTempPerm214: 0,
			fTempPerm215: 0.0,
			fTempPerm216: 0.0,
			fRec51: [0.0;2],
			fRec50: [0.0;2],
			iTempPerm217: 0,
			iTempPerm218: 0,
			fTempPerm219: 0.0,
			fRec55: [0.0;2],
			fTempPerm220: 0.0,
			fTempPerm221: 0.0,
			fTempPerm222: 0.0,
			iTempPerm223: 0,
			fTempPerm224: 0.0,
			iTempPerm225: 0,
			iTempPerm226: 0,
			fTempPerm227: 0.0,
			fRec60: [0.0;2],
			fRec59: [0.0;2],
			fTempPerm228: 0.0,
			fTempPerm229: 0.0,
			iTempPerm230: 0,
			fTempPerm231: 0.0,
			fTempPerm232: 0.0,
			fTempPerm233: 0.0,
			iTempPerm234: 0,
			fTempPerm235: 0.0,
			fTempPerm236: 0.0,
			fRec61: [0.0;2],
			fTempPerm237: 0.0,
			fTempPerm238: 0.0,
			fTempPerm239: 0.0,
			fTempPerm240: 0.0,
			fTempPerm241: 0.0,
			iTempPerm242: 0,
			iTempPerm243: 0,
			iTempPerm244: 0,
			iTempPerm245: 0,
			fTempPerm246: 0.0,
			fTempPerm247: 0.0,
			iTempPerm248: 0,
			iTempPerm249: 0,
			fTempPerm250: 0.0,
			fTempPerm251: 0.0,
			iTempPerm252: 0,
			fTempPerm253: 0.0,
			fTempPerm254: 0.0,
			fRec65: [0.0;2],
			fTempPerm255: 0.0,
			iTempPerm256: 0,
			iTempPerm257: 0,
			fTempPerm258: 0.0,
			fTempPerm259: 0.0,
			fTempPerm260: 0.0,
			fTempPerm261: 0.0,
			fRec64: [0.0;2],
			fTempPerm262: 0.0,
			fTempPerm263: 0.0,
			iTempPerm264: 0,
			fTempPerm265: 0.0,
			fRec66: [0.0;2],
			fTempPerm266: 0.0,
			fVec11: [0.0;2],
			fTempPerm267: 0.0,
			iTempPerm268: 0,
			iTempPerm269: 0,
			iTempPerm270: 0,
			fTempPerm271: 0.0,
			fTempPerm272: 0.0,
			fRec63: [0.0;2],
			fRec62: [0.0;2],
			iTempPerm273: 0,
			iTempPerm274: 0,
			fTempPerm275: 0.0,
			fRec67: [0.0;2],
			fTempPerm276: 0.0,
			fTempPerm277: 0.0,
			fTempPerm278: 0.0,
			iTempPerm279: 0,
			fTempPerm280: 0.0,
			iTempPerm281: 0,
			iTempPerm282: 0,
			fTempPerm283: 0.0,
			fRec72: [0.0;2],
			fRec71: [0.0;2],
			fTempPerm284: 0.0,
			fTempPerm285: 0.0,
			iTempPerm286: 0,
			fTempPerm287: 0.0,
			fTempPerm288: 0.0,
			fTempPerm289: 0.0,
			iTempPerm290: 0,
			fTempPerm291: 0.0,
			fTempPerm292: 0.0,
			fRec73: [0.0;2],
			fTempPerm293: 0.0,
			fTempPerm294: 0.0,
			fTempPerm295: 0.0,
			fTempPerm296: 0.0,
			fTempPerm297: 0.0,
			iTempPerm298: 0,
			iTempPerm299: 0,
			iTempPerm300: 0,
			fTempPerm301: 0.0,
			fTempPerm302: 0.0,
			iTempPerm303: 0,
			iTempPerm304: 0,
			fTempPerm305: 0.0,
			fTempPerm306: 0.0,
			iTempPerm307: 0,
			fTempPerm308: 0.0,
			fTempPerm309: 0.0,
			fRec77: [0.0;2],
			fTempPerm310: 0.0,
			iTempPerm311: 0,
			iTempPerm312: 0,
			fTempPerm313: 0.0,
			fTempPerm314: 0.0,
			fTempPerm315: 0.0,
			fTempPerm316: 0.0,
			fRec76: [0.0;2],
			fTempPerm317: 0.0,
			fTempPerm318: 0.0,
			iTempPerm319: 0,
			fTempPerm320: 0.0,
			fRec78: [0.0;2],
			fTempPerm321: 0.0,
			fVec12: [0.0;2],
			fTempPerm322: 0.0,
			iTempPerm323: 0,
			iTempPerm324: 0,
			iTempPerm325: 0,
			fTempPerm326: 0.0,
			fTempPerm327: 0.0,
			fRec75: [0.0;2],
			fRec74: [0.0;2],
			iTempPerm328: 0,
			iTempPerm329: 0,
			fTempPerm330: 0.0,
			fRec79: [0.0;2],
			fTempPerm331: 0.0,
			fTempPerm332: 0.0,
			fTempPerm333: 0.0,
			iTempPerm334: 0,
			fTempPerm335: 0.0,
			iTempPerm336: 0,
			iTempPerm337: 0,
			fTempPerm338: 0.0,
			fRec84: [0.0;2],
			fRec83: [0.0;2],
			fTempPerm339: 0.0,
			fTempPerm340: 0.0,
			iTempPerm341: 0,
			fTempPerm342: 0.0,
			fTempPerm343: 0.0,
			fTempPerm344: 0.0,
			iTempPerm345: 0,
			fTempPerm346: 0.0,
			fTempPerm347: 0.0,
			fRec85: [0.0;2],
			fTempPerm348: 0.0,
			fTempPerm349: 0.0,
			fTempPerm350: 0.0,
			fTempPerm351: 0.0,
			fTempPerm352: 0.0,
			fTempPerm353: 0.0,
			fTempPerm354: 0.0,
			fRec80: [0.0;2],
			fConst14: 0.0,
			fRec81: [0.0;2],
			fTempPerm355: 0.0,
			fTempPerm356: 0.0,
			fTempPerm357: 0.0,
			fRec68: [0.0;2],
			fRec69: [0.0;2],
			fTempPerm358: 0.0,
			fTempPerm359: 0.0,
			fTempPerm360: 0.0,
			fRec56: [0.0;2],
			fRec57: [0.0;2],
			fTempPerm361: 0.0,
			fTempPerm362: 0.0,
			fTempPerm363: 0.0,
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			fTempPerm364: 0.0,
			fTempPerm365: 0.0,
			fTempPerm366: 0.0,
			fRec32: [0.0;2],
			fRec33: [0.0;2],
			fTempPerm367: 0.0,
			fTempPerm368: 0.0,
			fTempPerm369: 0.0,
			fRec18: [0.0;2],
			fRec19: [0.0;2],
			fTempPerm370: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fRec86: [0.0;2],
			fRec87: [0.0;2],
			fTempPerm371: 0.0,
			fRec88: [0.0;2],
			fRec89: [0.0;2],
			iTempPerm372: 0,
			iRec91: [0;2],
			iRec97: [0;2],
			iTempPerm373: 0,
			iTempPerm374: 0,
			iTempPerm375: 0,
			iTempPerm376: 0,
			iTempPerm377: 0,
			iTempPerm378: 0,
			iTempPerm379: 0,
			iTempPerm380: 0,
			iTempPerm381: 0,
			iTempPerm382: 0,
			iTempPerm383: 0,
			iTempPerm384: 0,
			iTempPerm385: 0,
			iTempPerm386: 0,
			fTempPerm387: 0.0,
			fTempPerm388: 0.0,
			iTempPerm389: 0,
			fTempPerm390: 0.0,
			fTempPerm391: 0.0,
			iTempPerm392: 0,
			fTempPerm393: 0.0,
			fTempPerm394: 0.0,
			fTempPerm395: 0.0,
			fHslider7: 0.0,
			fVec13: [0.0;2],
			fTempPerm396: 0.0,
			fRec98: [0.0;2],
			fRec99: [0.0;2],
			fTempPerm397: 0.0,
			fTempPerm398: 0.0,
			iTempPerm399: 0,
			iTempPerm400: 0,
			fTempPerm401: 0.0,
			iTempPerm402: 0,
			iTempPerm403: 0,
			fTempPerm404: 0.0,
			fTempPerm405: 0.0,
			fTempPerm406: 0.0,
			fTempPerm407: 0.0,
			fHslider8: 0.0,
			fVec14: [0.0;2],
			fTempPerm408: 0.0,
			fRec100: [0.0;2],
			fRec101: [0.0;2],
			fTempPerm409: 0.0,
			fTempPerm410: 0.0,
			iTempPerm411: 0,
			fTempPerm412: 0.0,
			iTempPerm413: 0,
			iTempPerm414: 0,
			fTempPerm415: 0.0,
			fTempPerm416: 0.0,
			iTempPerm417: 0,
			fTempPerm418: 0.0,
			fConst17: 0.0,
			iTempPerm419: 0,
			iRec92: [0;2],
			iRec93: [0;2],
			iRec94: [0;2],
			iRec95: [0;2],
			iTempPerm420: 0,
			iTempPerm421: 0,
			iTempPerm422: 0,
			fRec90: [0.0;2],
			fTempPerm423: 0.0,
			fTempPerm424: 0.0,
			fTempPerm425: 0.0,
			fTempPerm426: 0.0,
			fTempPerm427: 0.0,
			iTempPerm428: 0,
			iTempPerm429: 0,
			fTempPerm430: 0.0,
			fTempPerm431: 0.0,
			iTempPerm432: 0,
			fTempPerm433: 0.0,
			iTempPerm434: 0,
			iTempPerm435: 0,
			iTempPerm436: 0,
			iTempPerm437: 0,
			iTempPerm438: 0,
			fRec103: [0.0;2],
			fTempPerm439: 0.0,
			iTempPerm440: 0,
			iTempPerm441: 0,
			iTempPerm442: 0,
			iTempPerm443: 0,
			fTempPerm444: 0.0,
			fTempPerm445: 0.0,
			iTempPerm446: 0,
			fTempPerm447: 0.0,
			fTempPerm448: 0.0,
			iTempPerm449: 0,
			iTempPerm450: 0,
			fTempPerm451: 0.0,
			fTempPerm452: 0.0,
			iTempPerm453: 0,
			fTempPerm454: 0.0,
			fTempPerm455: 0.0,
			fTempPerm456: 0.0,
			fTempPerm457: 0.0,
			iTempPerm458: 0,
			fTempPerm459: 0.0,
			fTempPerm460: 0.0,
			fRec104: [0.0;2],
			fRec105: [0.0;2],
			fTempPerm461: 0.0,
			fTempPerm462: 0.0,
			iTempPerm463: 0,
			fTempPerm464: 0.0,
			fTempPerm465: 0.0,
			fRec102: [0.0;2],
			iTempPerm466: 0,
			iTempPerm467: 0,
			fRec106: [0.0;2],
			fTempPerm468: 0.0,
			fTempPerm469: 0.0,
			fTempPerm470: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn TextureDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("core.lib/name", r"abdication-core");
		m.declare("core.lib/version", r"0.1");
		m.declare("filename", r"plugin-texture.dsp");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"plugin-texture");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.5.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 3;
	}
	fn get_num_outputs(&self) -> i32 {
		return 3;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fHslider0 = 0.0;
		self.fHslider1 = -8e+01;
		self.fHslider2 = 0.0;
		self.fHslider3 = 2.0;
		self.fHslider4 = 64.0;
		self.fHslider5 = 8e+01;
		self.fHslider6 = 1.0;
		self.fHslider7 = 1.0;
		self.fHslider8 = 0.45;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fVec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec0[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fVec2[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fVec3[l4 as usize] = 0.0;
		}
		self.iTempPerm0 = 0;
		self.iTempPerm1 = 0;
		for l5 in 0..2 {
			self.fRec3[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.iRec4[l6 as usize] = 0;
		}
		self.iTempPerm2 = 0;
		for l7 in 0..2 {
			self.fRec2[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec1[l8 as usize] = 0.0;
		}
		self.fTempPerm3 = 0.0;
		self.iTempPerm4 = 0;
		self.iTempPerm5 = 0;
		for l9 in 0..2 {
			self.iRec9[l9 as usize] = 0;
		}
		self.iTempPerm6 = 0;
		self.iTempPerm7 = 0;
		self.fTempPerm8 = 0.0;
		self.fTempPerm9 = 0.0;
		self.fTempPerm10 = 0.0;
		self.fTempPerm11 = 0.0;
		self.fTempPerm12 = 0.0;
		self.iTempPerm13 = 0;
		self.iTempPerm14 = 0;
		self.fTempPerm15 = 0.0;
		self.fTempPerm16 = 0.0;
		self.iTempPerm17 = 0;
		self.fTempPerm18 = 0.0;
		for l10 in 0..2 {
			self.fVec4[l10 as usize] = 0.0;
		}
		self.fTempPerm19 = 0.0;
		for l11 in 0..2 {
			self.fRec10[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec11[l12 as usize] = 0.0;
		}
		self.fTempPerm20 = 0.0;
		for l13 in 0..2 {
			self.fRec8[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fVec5[l14 as usize] = 0.0;
		}
		self.fTempPerm21 = 0.0;
		for l15 in 0..2 {
			self.fRec12[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec13[l16 as usize] = 0.0;
		}
		self.fTempPerm22 = 0.0;
		self.fTempPerm23 = 0.0;
		self.fTempPerm24 = 0.0;
		self.iTempPerm25 = 0;
		self.iTempPerm26 = 0;
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		self.fTempPerm29 = 0.0;
		self.fTempPerm30 = 0.0;
		for l17 in 0..2 {
			self.fRec7[l17 as usize] = 0.0;
		}
		self.fTempPerm31 = 0.0;
		self.fTempPerm32 = 0.0;
		self.iTempPerm33 = 0;
		self.fTempPerm34 = 0.0;
		for l18 in 0..2 {
			self.fRec14[l18 as usize] = 0.0;
		}
		self.fTempPerm35 = 0.0;
		for l19 in 0..2 {
			self.fVec6[l19 as usize] = 0.0;
		}
		self.fTempPerm36 = 0.0;
		self.iTempPerm37 = 0;
		self.iTempPerm38 = 0;
		self.iTempPerm39 = 0;
		self.fTempPerm40 = 0.0;
		self.fTempPerm41 = 0.0;
		for l20 in 0..2 {
			self.fRec6[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec5[l21 as usize] = 0.0;
		}
		self.iTempPerm42 = 0;
		self.iTempPerm43 = 0;
		self.fTempPerm44 = 0.0;
		for l22 in 0..2 {
			self.fRec15[l22 as usize] = 0.0;
		}
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		for l23 in 0..2 {
			self.fRec16[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec17[l24 as usize] = 0.0;
		}
		self.fTempPerm47 = 0.0;
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		self.iTempPerm52 = 0;
		self.fTempPerm53 = 0.0;
		for l25 in 0..2 {
			self.fVec7[l25 as usize] = 0.0;
		}
		self.fTempPerm54 = 0.0;
		for l26 in 0..2 {
			self.fRec21[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec22[l27 as usize] = 0.0;
		}
		self.fTempPerm55 = 0.0;
		self.iTempPerm56 = 0;
		self.iTempPerm57 = 0;
		self.fTempPerm58 = 0.0;
		for l28 in 0..2 {
			self.fRec24[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec23[l29 as usize] = 0.0;
		}
		self.fTempPerm59 = 0.0;
		self.fTempPerm60 = 0.0;
		self.iTempPerm61 = 0;
		self.fTempPerm62 = 0.0;
		self.fTempPerm63 = 0.0;
		self.fTempPerm64 = 0.0;
		self.iTempPerm65 = 0;
		self.fTempPerm66 = 0.0;
		self.fTempPerm67 = 0.0;
		for l30 in 0..2 {
			self.fRec25[l30 as usize] = 0.0;
		}
		self.fTempPerm68 = 0.0;
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		self.fTempPerm71 = 0.0;
		self.fTempPerm72 = 0.0;
		self.fTempPerm73 = 0.0;
		self.iTempPerm74 = 0;
		self.iTempPerm75 = 0;
		self.iTempPerm76 = 0;
		self.iTempPerm77 = 0;
		self.fTempPerm78 = 0.0;
		self.fTempPerm79 = 0.0;
		self.iTempPerm80 = 0;
		self.iTempPerm81 = 0;
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.iTempPerm84 = 0;
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		for l31 in 0..2 {
			self.fRec29[l31 as usize] = 0.0;
		}
		self.fTempPerm87 = 0.0;
		self.iTempPerm88 = 0;
		self.iTempPerm89 = 0;
		self.fTempPerm90 = 0.0;
		self.fTempPerm91 = 0.0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		for l32 in 0..2 {
			self.fRec28[l32 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		self.iTempPerm96 = 0;
		self.fTempPerm97 = 0.0;
		for l33 in 0..2 {
			self.fRec30[l33 as usize] = 0.0;
		}
		self.fTempPerm98 = 0.0;
		for l34 in 0..2 {
			self.fVec8[l34 as usize] = 0.0;
		}
		self.fTempPerm99 = 0.0;
		self.iTempPerm100 = 0;
		self.iTempPerm101 = 0;
		self.iTempPerm102 = 0;
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		for l35 in 0..2 {
			self.fRec27[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec26[l36 as usize] = 0.0;
		}
		self.iTempPerm105 = 0;
		self.iTempPerm106 = 0;
		self.fTempPerm107 = 0.0;
		for l37 in 0..2 {
			self.fRec31[l37 as usize] = 0.0;
		}
		self.fTempPerm108 = 0.0;
		self.fTempPerm109 = 0.0;
		self.fTempPerm110 = 0.0;
		self.iTempPerm111 = 0;
		self.fTempPerm112 = 0.0;
		self.iTempPerm113 = 0;
		self.iTempPerm114 = 0;
		self.fTempPerm115 = 0.0;
		for l38 in 0..2 {
			self.fRec36[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec35[l39 as usize] = 0.0;
		}
		self.fTempPerm116 = 0.0;
		self.fTempPerm117 = 0.0;
		self.iTempPerm118 = 0;
		self.fTempPerm119 = 0.0;
		self.fTempPerm120 = 0.0;
		self.fTempPerm121 = 0.0;
		self.iTempPerm122 = 0;
		self.fTempPerm123 = 0.0;
		self.fTempPerm124 = 0.0;
		for l40 in 0..2 {
			self.fRec37[l40 as usize] = 0.0;
		}
		self.fTempPerm125 = 0.0;
		self.fTempPerm126 = 0.0;
		self.fTempPerm127 = 0.0;
		self.fTempPerm128 = 0.0;
		self.fTempPerm129 = 0.0;
		self.iTempPerm130 = 0;
		self.iTempPerm131 = 0;
		self.iTempPerm132 = 0;
		self.iTempPerm133 = 0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		self.iTempPerm136 = 0;
		self.iTempPerm137 = 0;
		self.fTempPerm138 = 0.0;
		self.fTempPerm139 = 0.0;
		self.iTempPerm140 = 0;
		self.fTempPerm141 = 0.0;
		self.fTempPerm142 = 0.0;
		for l41 in 0..2 {
			self.fRec41[l41 as usize] = 0.0;
		}
		self.fTempPerm143 = 0.0;
		self.iTempPerm144 = 0;
		self.iTempPerm145 = 0;
		self.fTempPerm146 = 0.0;
		self.fTempPerm147 = 0.0;
		self.fTempPerm148 = 0.0;
		self.fTempPerm149 = 0.0;
		for l42 in 0..2 {
			self.fRec40[l42 as usize] = 0.0;
		}
		self.fTempPerm150 = 0.0;
		self.fTempPerm151 = 0.0;
		self.iTempPerm152 = 0;
		self.fTempPerm153 = 0.0;
		for l43 in 0..2 {
			self.fRec42[l43 as usize] = 0.0;
		}
		self.fTempPerm154 = 0.0;
		for l44 in 0..2 {
			self.fVec9[l44 as usize] = 0.0;
		}
		self.fTempPerm155 = 0.0;
		self.iTempPerm156 = 0;
		self.iTempPerm157 = 0;
		self.iTempPerm158 = 0;
		self.fTempPerm159 = 0.0;
		self.fTempPerm160 = 0.0;
		for l45 in 0..2 {
			self.fRec39[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec38[l46 as usize] = 0.0;
		}
		self.iTempPerm161 = 0;
		self.iTempPerm162 = 0;
		self.fTempPerm163 = 0.0;
		for l47 in 0..2 {
			self.fRec43[l47 as usize] = 0.0;
		}
		self.fTempPerm164 = 0.0;
		self.fTempPerm165 = 0.0;
		self.fTempPerm166 = 0.0;
		self.iTempPerm167 = 0;
		self.fTempPerm168 = 0.0;
		self.iTempPerm169 = 0;
		self.iTempPerm170 = 0;
		self.fTempPerm171 = 0.0;
		for l48 in 0..2 {
			self.fRec48[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec47[l49 as usize] = 0.0;
		}
		self.fTempPerm172 = 0.0;
		self.fTempPerm173 = 0.0;
		self.iTempPerm174 = 0;
		self.fTempPerm175 = 0.0;
		self.fTempPerm176 = 0.0;
		self.fTempPerm177 = 0.0;
		self.iTempPerm178 = 0;
		self.fTempPerm179 = 0.0;
		self.fTempPerm180 = 0.0;
		for l50 in 0..2 {
			self.fRec49[l50 as usize] = 0.0;
		}
		self.fTempPerm181 = 0.0;
		self.fTempPerm182 = 0.0;
		self.fTempPerm183 = 0.0;
		self.fTempPerm184 = 0.0;
		self.fTempPerm185 = 0.0;
		self.iTempPerm186 = 0;
		self.iTempPerm187 = 0;
		self.iTempPerm188 = 0;
		self.iTempPerm189 = 0;
		self.fTempPerm190 = 0.0;
		self.fTempPerm191 = 0.0;
		self.iTempPerm192 = 0;
		self.iTempPerm193 = 0;
		self.fTempPerm194 = 0.0;
		self.fTempPerm195 = 0.0;
		self.iTempPerm196 = 0;
		self.fTempPerm197 = 0.0;
		self.fTempPerm198 = 0.0;
		for l51 in 0..2 {
			self.fRec53[l51 as usize] = 0.0;
		}
		self.fTempPerm199 = 0.0;
		self.iTempPerm200 = 0;
		self.iTempPerm201 = 0;
		self.fTempPerm202 = 0.0;
		self.fTempPerm203 = 0.0;
		self.fTempPerm204 = 0.0;
		self.fTempPerm205 = 0.0;
		for l52 in 0..2 {
			self.fRec52[l52 as usize] = 0.0;
		}
		self.fTempPerm206 = 0.0;
		self.fTempPerm207 = 0.0;
		self.iTempPerm208 = 0;
		self.fTempPerm209 = 0.0;
		for l53 in 0..2 {
			self.fRec54[l53 as usize] = 0.0;
		}
		self.fTempPerm210 = 0.0;
		for l54 in 0..2 {
			self.fVec10[l54 as usize] = 0.0;
		}
		self.fTempPerm211 = 0.0;
		self.iTempPerm212 = 0;
		self.iTempPerm213 = 0;
		self.iTempPerm214 = 0;
		self.fTempPerm215 = 0.0;
		self.fTempPerm216 = 0.0;
		for l55 in 0..2 {
			self.fRec51[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec50[l56 as usize] = 0.0;
		}
		self.iTempPerm217 = 0;
		self.iTempPerm218 = 0;
		self.fTempPerm219 = 0.0;
		for l57 in 0..2 {
			self.fRec55[l57 as usize] = 0.0;
		}
		self.fTempPerm220 = 0.0;
		self.fTempPerm221 = 0.0;
		self.fTempPerm222 = 0.0;
		self.iTempPerm223 = 0;
		self.fTempPerm224 = 0.0;
		self.iTempPerm225 = 0;
		self.iTempPerm226 = 0;
		self.fTempPerm227 = 0.0;
		for l58 in 0..2 {
			self.fRec60[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec59[l59 as usize] = 0.0;
		}
		self.fTempPerm228 = 0.0;
		self.fTempPerm229 = 0.0;
		self.iTempPerm230 = 0;
		self.fTempPerm231 = 0.0;
		self.fTempPerm232 = 0.0;
		self.fTempPerm233 = 0.0;
		self.iTempPerm234 = 0;
		self.fTempPerm235 = 0.0;
		self.fTempPerm236 = 0.0;
		for l60 in 0..2 {
			self.fRec61[l60 as usize] = 0.0;
		}
		self.fTempPerm237 = 0.0;
		self.fTempPerm238 = 0.0;
		self.fTempPerm239 = 0.0;
		self.fTempPerm240 = 0.0;
		self.fTempPerm241 = 0.0;
		self.iTempPerm242 = 0;
		self.iTempPerm243 = 0;
		self.iTempPerm244 = 0;
		self.iTempPerm245 = 0;
		self.fTempPerm246 = 0.0;
		self.fTempPerm247 = 0.0;
		self.iTempPerm248 = 0;
		self.iTempPerm249 = 0;
		self.fTempPerm250 = 0.0;
		self.fTempPerm251 = 0.0;
		self.iTempPerm252 = 0;
		self.fTempPerm253 = 0.0;
		self.fTempPerm254 = 0.0;
		for l61 in 0..2 {
			self.fRec65[l61 as usize] = 0.0;
		}
		self.fTempPerm255 = 0.0;
		self.iTempPerm256 = 0;
		self.iTempPerm257 = 0;
		self.fTempPerm258 = 0.0;
		self.fTempPerm259 = 0.0;
		self.fTempPerm260 = 0.0;
		self.fTempPerm261 = 0.0;
		for l62 in 0..2 {
			self.fRec64[l62 as usize] = 0.0;
		}
		self.fTempPerm262 = 0.0;
		self.fTempPerm263 = 0.0;
		self.iTempPerm264 = 0;
		self.fTempPerm265 = 0.0;
		for l63 in 0..2 {
			self.fRec66[l63 as usize] = 0.0;
		}
		self.fTempPerm266 = 0.0;
		for l64 in 0..2 {
			self.fVec11[l64 as usize] = 0.0;
		}
		self.fTempPerm267 = 0.0;
		self.iTempPerm268 = 0;
		self.iTempPerm269 = 0;
		self.iTempPerm270 = 0;
		self.fTempPerm271 = 0.0;
		self.fTempPerm272 = 0.0;
		for l65 in 0..2 {
			self.fRec63[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec62[l66 as usize] = 0.0;
		}
		self.iTempPerm273 = 0;
		self.iTempPerm274 = 0;
		self.fTempPerm275 = 0.0;
		for l67 in 0..2 {
			self.fRec67[l67 as usize] = 0.0;
		}
		self.fTempPerm276 = 0.0;
		self.fTempPerm277 = 0.0;
		self.fTempPerm278 = 0.0;
		self.iTempPerm279 = 0;
		self.fTempPerm280 = 0.0;
		self.iTempPerm281 = 0;
		self.iTempPerm282 = 0;
		self.fTempPerm283 = 0.0;
		for l68 in 0..2 {
			self.fRec72[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec71[l69 as usize] = 0.0;
		}
		self.fTempPerm284 = 0.0;
		self.fTempPerm285 = 0.0;
		self.iTempPerm286 = 0;
		self.fTempPerm287 = 0.0;
		self.fTempPerm288 = 0.0;
		self.fTempPerm289 = 0.0;
		self.iTempPerm290 = 0;
		self.fTempPerm291 = 0.0;
		self.fTempPerm292 = 0.0;
		for l70 in 0..2 {
			self.fRec73[l70 as usize] = 0.0;
		}
		self.fTempPerm293 = 0.0;
		self.fTempPerm294 = 0.0;
		self.fTempPerm295 = 0.0;
		self.fTempPerm296 = 0.0;
		self.fTempPerm297 = 0.0;
		self.iTempPerm298 = 0;
		self.iTempPerm299 = 0;
		self.iTempPerm300 = 0;
		self.fTempPerm301 = 0.0;
		self.fTempPerm302 = 0.0;
		self.iTempPerm303 = 0;
		self.iTempPerm304 = 0;
		self.fTempPerm305 = 0.0;
		self.fTempPerm306 = 0.0;
		self.iTempPerm307 = 0;
		self.fTempPerm308 = 0.0;
		self.fTempPerm309 = 0.0;
		for l71 in 0..2 {
			self.fRec77[l71 as usize] = 0.0;
		}
		self.fTempPerm310 = 0.0;
		self.iTempPerm311 = 0;
		self.iTempPerm312 = 0;
		self.fTempPerm313 = 0.0;
		self.fTempPerm314 = 0.0;
		self.fTempPerm315 = 0.0;
		self.fTempPerm316 = 0.0;
		for l72 in 0..2 {
			self.fRec76[l72 as usize] = 0.0;
		}
		self.fTempPerm317 = 0.0;
		self.fTempPerm318 = 0.0;
		self.iTempPerm319 = 0;
		self.fTempPerm320 = 0.0;
		for l73 in 0..2 {
			self.fRec78[l73 as usize] = 0.0;
		}
		self.fTempPerm321 = 0.0;
		for l74 in 0..2 {
			self.fVec12[l74 as usize] = 0.0;
		}
		self.fTempPerm322 = 0.0;
		self.iTempPerm323 = 0;
		self.iTempPerm324 = 0;
		self.iTempPerm325 = 0;
		self.fTempPerm326 = 0.0;
		self.fTempPerm327 = 0.0;
		for l75 in 0..2 {
			self.fRec75[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec74[l76 as usize] = 0.0;
		}
		self.iTempPerm328 = 0;
		self.iTempPerm329 = 0;
		self.fTempPerm330 = 0.0;
		for l77 in 0..2 {
			self.fRec79[l77 as usize] = 0.0;
		}
		self.fTempPerm331 = 0.0;
		self.fTempPerm332 = 0.0;
		self.fTempPerm333 = 0.0;
		self.iTempPerm334 = 0;
		self.fTempPerm335 = 0.0;
		self.iTempPerm336 = 0;
		self.iTempPerm337 = 0;
		self.fTempPerm338 = 0.0;
		for l78 in 0..2 {
			self.fRec84[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec83[l79 as usize] = 0.0;
		}
		self.fTempPerm339 = 0.0;
		self.fTempPerm340 = 0.0;
		self.iTempPerm341 = 0;
		self.fTempPerm342 = 0.0;
		self.fTempPerm343 = 0.0;
		self.fTempPerm344 = 0.0;
		self.iTempPerm345 = 0;
		self.fTempPerm346 = 0.0;
		self.fTempPerm347 = 0.0;
		for l80 in 0..2 {
			self.fRec85[l80 as usize] = 0.0;
		}
		self.fTempPerm348 = 0.0;
		self.fTempPerm349 = 0.0;
		self.fTempPerm350 = 0.0;
		self.fTempPerm351 = 0.0;
		self.fTempPerm352 = 0.0;
		self.fTempPerm353 = 0.0;
		self.fTempPerm354 = 0.0;
		for l81 in 0..2 {
			self.fRec80[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec81[l82 as usize] = 0.0;
		}
		self.fTempPerm355 = 0.0;
		self.fTempPerm356 = 0.0;
		self.fTempPerm357 = 0.0;
		for l83 in 0..2 {
			self.fRec68[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec69[l84 as usize] = 0.0;
		}
		self.fTempPerm358 = 0.0;
		self.fTempPerm359 = 0.0;
		self.fTempPerm360 = 0.0;
		for l85 in 0..2 {
			self.fRec56[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec57[l86 as usize] = 0.0;
		}
		self.fTempPerm361 = 0.0;
		self.fTempPerm362 = 0.0;
		self.fTempPerm363 = 0.0;
		for l87 in 0..2 {
			self.fRec44[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec45[l88 as usize] = 0.0;
		}
		self.fTempPerm364 = 0.0;
		self.fTempPerm365 = 0.0;
		self.fTempPerm366 = 0.0;
		for l89 in 0..2 {
			self.fRec32[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec33[l90 as usize] = 0.0;
		}
		self.fTempPerm367 = 0.0;
		self.fTempPerm368 = 0.0;
		self.fTempPerm369 = 0.0;
		for l91 in 0..2 {
			self.fRec18[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec19[l92 as usize] = 0.0;
		}
		self.fTempPerm370 = 0.0;
		for l93 in 0..2 {
			self.fRec86[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec87[l94 as usize] = 0.0;
		}
		self.fTempPerm371 = 0.0;
		for l95 in 0..2 {
			self.fRec88[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec89[l96 as usize] = 0.0;
		}
		self.iTempPerm372 = 0;
		for l97 in 0..2 {
			self.iRec91[l97 as usize] = 0;
		}
		for l98 in 0..2 {
			self.iRec97[l98 as usize] = 0;
		}
		self.iTempPerm373 = 0;
		self.iTempPerm374 = 0;
		self.iTempPerm375 = 0;
		self.iTempPerm376 = 0;
		self.iTempPerm377 = 0;
		self.iTempPerm378 = 0;
		self.iTempPerm379 = 0;
		self.iTempPerm380 = 0;
		self.iTempPerm381 = 0;
		self.iTempPerm382 = 0;
		self.iTempPerm383 = 0;
		self.iTempPerm384 = 0;
		self.iTempPerm385 = 0;
		self.iTempPerm386 = 0;
		self.fTempPerm387 = 0.0;
		self.fTempPerm388 = 0.0;
		self.iTempPerm389 = 0;
		self.fTempPerm390 = 0.0;
		self.fTempPerm391 = 0.0;
		self.iTempPerm392 = 0;
		self.fTempPerm393 = 0.0;
		self.fTempPerm394 = 0.0;
		self.fTempPerm395 = 0.0;
		for l99 in 0..2 {
			self.fVec13[l99 as usize] = 0.0;
		}
		self.fTempPerm396 = 0.0;
		for l100 in 0..2 {
			self.fRec98[l100 as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fRec99[l101 as usize] = 0.0;
		}
		self.fTempPerm397 = 0.0;
		self.fTempPerm398 = 0.0;
		self.iTempPerm399 = 0;
		self.iTempPerm400 = 0;
		self.fTempPerm401 = 0.0;
		self.iTempPerm402 = 0;
		self.iTempPerm403 = 0;
		self.fTempPerm404 = 0.0;
		self.fTempPerm405 = 0.0;
		self.fTempPerm406 = 0.0;
		self.fTempPerm407 = 0.0;
		for l102 in 0..2 {
			self.fVec14[l102 as usize] = 0.0;
		}
		self.fTempPerm408 = 0.0;
		for l103 in 0..2 {
			self.fRec100[l103 as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec101[l104 as usize] = 0.0;
		}
		self.fTempPerm409 = 0.0;
		self.fTempPerm410 = 0.0;
		self.iTempPerm411 = 0;
		self.fTempPerm412 = 0.0;
		self.iTempPerm413 = 0;
		self.iTempPerm414 = 0;
		self.fTempPerm415 = 0.0;
		self.fTempPerm416 = 0.0;
		self.iTempPerm417 = 0;
		self.fTempPerm418 = 0.0;
		self.iTempPerm419 = 0;
		for l105 in 0..2 {
			self.iRec92[l105 as usize] = 0;
		}
		for l106 in 0..2 {
			self.iRec93[l106 as usize] = 0;
		}
		for l107 in 0..2 {
			self.iRec94[l107 as usize] = 0;
		}
		for l108 in 0..2 {
			self.iRec95[l108 as usize] = 0;
		}
		self.iTempPerm420 = 0;
		self.iTempPerm421 = 0;
		self.iTempPerm422 = 0;
		for l109 in 0..2 {
			self.fRec90[l109 as usize] = 0.0;
		}
		self.fTempPerm423 = 0.0;
		self.fTempPerm424 = 0.0;
		self.fTempPerm425 = 0.0;
		self.fTempPerm426 = 0.0;
		self.fTempPerm427 = 0.0;
		self.iTempPerm428 = 0;
		self.iTempPerm429 = 0;
		self.fTempPerm430 = 0.0;
		self.fTempPerm431 = 0.0;
		self.iTempPerm432 = 0;
		self.fTempPerm433 = 0.0;
		self.iTempPerm434 = 0;
		self.iTempPerm435 = 0;
		self.iTempPerm436 = 0;
		self.iTempPerm437 = 0;
		self.iTempPerm438 = 0;
		for l110 in 0..2 {
			self.fRec103[l110 as usize] = 0.0;
		}
		self.fTempPerm439 = 0.0;
		self.iTempPerm440 = 0;
		self.iTempPerm441 = 0;
		self.iTempPerm442 = 0;
		self.iTempPerm443 = 0;
		self.fTempPerm444 = 0.0;
		self.fTempPerm445 = 0.0;
		self.iTempPerm446 = 0;
		self.fTempPerm447 = 0.0;
		self.fTempPerm448 = 0.0;
		self.iTempPerm449 = 0;
		self.iTempPerm450 = 0;
		self.fTempPerm451 = 0.0;
		self.fTempPerm452 = 0.0;
		self.iTempPerm453 = 0;
		self.fTempPerm454 = 0.0;
		self.fTempPerm455 = 0.0;
		self.fTempPerm456 = 0.0;
		self.fTempPerm457 = 0.0;
		self.iTempPerm458 = 0;
		self.fTempPerm459 = 0.0;
		self.fTempPerm460 = 0.0;
		for l111 in 0..2 {
			self.fRec104[l111 as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec105[l112 as usize] = 0.0;
		}
		self.fTempPerm461 = 0.0;
		self.fTempPerm462 = 0.0;
		self.iTempPerm463 = 0;
		self.fTempPerm464 = 0.0;
		self.fTempPerm465 = 0.0;
		for l113 in 0..2 {
			self.fRec102[l113 as usize] = 0.0;
		}
		self.iTempPerm466 = 0;
		self.iTempPerm467 = 0;
		for l114 in 0..2 {
			self.fRec106[l114 as usize] = 0.0;
		}
		self.fTempPerm468 = 0.0;
		self.fTempPerm469 = 0.0;
		self.fTempPerm470 = 0.0;
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 - 44.1 / self.fConst0;
		self.fConst2 = F32::exp(-(0.6666667 / self.fConst0));
		self.fConst3 = 0.33333334 / self.fConst0;
		self.fConst4 = 1.0 - self.fConst2;
		self.fConst5 = F32::exp(-(83.333336 / self.fConst0));
		self.fConst6 = 0.45 * self.fConst0;
		self.fConst7 = 1.0 / self.fConst0;
		self.fConst8 = 1.0 - self.fConst5;
		let mut fConst9: F32 = TextureDsp_faustpower2_f(self.fConst0);
		self.fConst10 = 97.40909 / fConst9;
		self.fConst11 = 1.0 / fConst9;
		self.fConst12 = 98.696045 / fConst9;
		self.fConst13 = 3.1415927 / self.fConst0;
		self.fConst14 = 9.869604 / self.fConst0;
		self.fConst15 = F32::exp(-self.fConst3);
		self.fConst16 = 1.0 - self.fConst15;
		self.fConst17 = 0.05 * self.fConst0;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		TextureDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin-texture");
		ui_interface.declare(None, "11", "");
		ui_interface.open_horizontal_box("Events");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.add_check_button("Mute", ParamIndex(0));
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "%");
		ui_interface.add_horizontal_slider("Density", ParamIndex(1), 8e+01, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.declare(Some(ParamIndex(2)), "unit", "dB");
		ui_interface.add_horizontal_slider("Depth", ParamIndex(2), 0.0, 0.0, 12.0, 0.1);
		ui_interface.declare(Some(ParamIndex(3)), "3", "");
		ui_interface.add_horizontal_slider("Life", ParamIndex(3), 2.0, 1.0, 3.0, 0.01);
		ui_interface.declare(Some(ParamIndex(4)), "4", "");
		ui_interface.add_horizontal_slider("Sharp", ParamIndex(4), 1.0, 0.5, 4.0, 0.01);
		ui_interface.close_box();
		ui_interface.declare(None, "12", "");
		ui_interface.open_horizontal_box("Calls");
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.declare(Some(ParamIndex(5)), "unit", "dB");
		ui_interface.add_horizontal_slider("Level", ParamIndex(5), -8e+01, -8e+01, -26.0, 0.1);
		ui_interface.declare(Some(ParamIndex(6)), "2", "");
		ui_interface.add_horizontal_slider("Activity", ParamIndex(6), 0.45, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(7)), "3", "");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(7), 1.0, 0.25, 3.0, 0.01);
		ui_interface.close_box();
		ui_interface.add_button("gate", ParamIndex(8));
		ui_interface.add_horizontal_slider("ramp", ParamIndex(9), 64.0, 1.0, 4096.0, 1.0);
		ui_interface.add_horizontal_slider("slot", ParamIndex(10), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			8 => Some(self.fButton0),
			0 => Some(self.fCheckbox0),
			2 => Some(self.fHslider0),
			5 => Some(self.fHslider1),
			10 => Some(self.fHslider2),
			3 => Some(self.fHslider3),
			9 => Some(self.fHslider4),
			1 => Some(self.fHslider5),
			4 => Some(self.fHslider6),
			7 => Some(self.fHslider7),
			6 => Some(self.fHslider8),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			8 => { self.fButton0 = value }
			0 => { self.fCheckbox0 = value }
			2 => { self.fHslider0 = value }
			5 => { self.fHslider1 = value }
			10 => { self.fHslider2 = value }
			3 => { self.fHslider3 = value }
			9 => { self.fHslider4 = value }
			1 => { self.fHslider5 = value }
			4 => { self.fHslider6 = value }
			7 => { self.fHslider7 = value }
			6 => { self.fHslider8 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2) = if let [inputs0, inputs1, inputs2, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			(inputs0, inputs1, inputs2)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1, outputs2) = if let [outputs0, outputs1, outputs2, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = 1.0 - self.fCheckbox0;
		let mut fSlow2: F32 = self.fHslider0;
		let mut iSlow3: i32 = (fSlow2 != 0.0) as i32;
		let mut fSlow4: F32 = self.fHslider1;
		let mut iSlow5: i32 = (fSlow4 > -8e+01) as i32;
		let mut fSlow6: F32 = (iSlow5) as F32;
		let mut iSlow7: i32 = (fSlow6 != 0.0) as i32;
		let mut iTemp5: i32 = self.iTempPerm0;
		let mut iTemp6: i32 = self.iTempPerm1;
		let mut fSlow8: F32 = self.fHslider2;
		let mut fSlow9: F32 = 337.0 * fSlow8;
		let mut iSlow10: i32 = (fSlow9 + 1007.0) as i32;
		let mut iTemp7: i32 = self.iTempPerm2;
		let mut fTemp8: F32 = self.fTempPerm3;
		let mut iTemp9: i32 = self.iTempPerm4;
		let mut iTemp10: i32 = self.iTempPerm5;
		let mut iSlow11: i32 = (fSlow9 + 1e+03) as i32;
		let mut iTemp11: i32 = self.iTempPerm6;
		let mut iTemp12: i32 = self.iTempPerm7;
		let mut fTemp13: F32 = self.fTempPerm8;
		let mut fTemp14: F32 = self.fTempPerm9;
		let mut fTemp15: F32 = self.fTempPerm10;
		let mut fTemp16: F32 = self.fTempPerm11;
		let mut fTemp17: F32 = self.fTempPerm12;
		let mut iTemp18: i32 = self.iTempPerm13;
		let mut iTemp19: i32 = self.iTempPerm14;
		let mut fTemp20: F32 = self.fTempPerm15;
		let mut fTemp21: F32 = self.fTempPerm16;
		let mut iTemp22: i32 = self.iTempPerm17;
		let mut fTemp23: F32 = self.fTempPerm18;
		let mut fSlow12: F32 = self.fHslider3;
		let mut fSlow13: F32 = self.fHslider4;
		let mut fTemp24: F32 = self.fTempPerm19;
		let mut fTemp25: F32 = self.fTempPerm20;
		let mut fSlow14: F32 = self.fHslider5;
		let mut fTemp26: F32 = self.fTempPerm21;
		let mut fTemp27: F32 = self.fTempPerm22;
		let mut fTemp28: F32 = self.fTempPerm23;
		let mut fTemp29: F32 = self.fTempPerm24;
		let mut iTemp30: i32 = self.iTempPerm25;
		let mut iTemp31: i32 = self.iTempPerm26;
		let mut fTemp32: F32 = self.fTempPerm27;
		let mut fTemp33: F32 = self.fTempPerm28;
		let mut fTemp34: F32 = self.fTempPerm29;
		let mut fTemp35: F32 = self.fTempPerm30;
		let mut fTemp36: F32 = self.fTempPerm31;
		let mut fTemp37: F32 = self.fTempPerm32;
		let mut iTemp38: i32 = self.iTempPerm33;
		let mut fTemp39: F32 = self.fTempPerm34;
		let mut fTemp40: F32 = self.fTempPerm35;
		let mut fTemp41: F32 = self.fTempPerm36;
		let mut iTemp42: i32 = self.iTempPerm37;
		let mut iTemp43: i32 = self.iTempPerm38;
		let mut iTemp44: i32 = self.iTempPerm39;
		let mut fTemp45: F32 = self.fTempPerm40;
		let mut fTemp46: F32 = self.fTempPerm41;
		let mut iTemp47: i32 = self.iTempPerm42;
		let mut iTemp48: i32 = self.iTempPerm43;
		let mut fTemp49: F32 = self.fTempPerm44;
		let mut fTemp50: F32 = self.fTempPerm45;
		let mut fTemp51: F32 = self.fTempPerm46;
		let mut fSlow15: F32 = (iSlow3) as F32;
		let mut fTemp52: F32 = self.fTempPerm47;
		let mut fTemp53: F32 = self.fTempPerm48;
		let mut fSlow16: F32 = 3.33 * fSlow15;
		let mut fTemp54: F32 = self.fTempPerm49;
		let mut fTemp55: F32 = self.fTempPerm50;
		let mut fTemp56: F32 = self.fTempPerm51;
		let mut iTemp57: i32 = self.iTempPerm52;
		let mut fTemp58: F32 = self.fTempPerm53;
		let mut fSlow17: F32 = self.fHslider6;
		let mut fTemp59: F32 = self.fTempPerm54;
		let mut fTemp60: F32 = self.fTempPerm55;
		let mut iTemp61: i32 = self.iTempPerm56;
		let mut iTemp62: i32 = self.iTempPerm57;
		let mut fTemp63: F32 = self.fTempPerm58;
		let mut fTemp64: F32 = self.fTempPerm59;
		let mut fTemp65: F32 = self.fTempPerm60;
		let mut iTemp66: i32 = self.iTempPerm61;
		let mut fTemp67: F32 = self.fTempPerm62;
		let mut fTemp68: F32 = self.fTempPerm63;
		let mut fTemp69: F32 = self.fTempPerm64;
		let mut iTemp70: i32 = self.iTempPerm65;
		let mut fTemp71: F32 = self.fTempPerm66;
		let mut fTemp72: F32 = self.fTempPerm67;
		let mut fTemp73: F32 = self.fTempPerm68;
		let mut fTemp74: F32 = self.fTempPerm69;
		let mut fTemp75: F32 = self.fTempPerm70;
		let mut fTemp76: F32 = self.fTempPerm71;
		let mut fTemp77: F32 = self.fTempPerm72;
		let mut fTemp78: F32 = self.fTempPerm73;
		let mut iTemp79: i32 = self.iTempPerm74;
		let mut iTemp80: i32 = self.iTempPerm75;
		let mut iTemp81: i32 = self.iTempPerm76;
		let mut iTemp82: i32 = self.iTempPerm77;
		let mut fTemp83: F32 = self.fTempPerm78;
		let mut fTemp84: F32 = self.fTempPerm79;
		let mut iTemp85: i32 = self.iTempPerm80;
		let mut iTemp86: i32 = self.iTempPerm81;
		let mut fTemp87: F32 = self.fTempPerm82;
		let mut fTemp88: F32 = self.fTempPerm83;
		let mut iTemp89: i32 = self.iTempPerm84;
		let mut fTemp90: F32 = self.fTempPerm85;
		let mut fTemp91: F32 = self.fTempPerm86;
		let mut fTemp92: F32 = self.fTempPerm87;
		let mut iTemp93: i32 = self.iTempPerm88;
		let mut iTemp94: i32 = self.iTempPerm89;
		let mut fTemp95: F32 = self.fTempPerm90;
		let mut fTemp96: F32 = self.fTempPerm91;
		let mut fTemp97: F32 = self.fTempPerm92;
		let mut fTemp98: F32 = self.fTempPerm93;
		let mut fTemp99: F32 = self.fTempPerm94;
		let mut fTemp100: F32 = self.fTempPerm95;
		let mut iTemp101: i32 = self.iTempPerm96;
		let mut fTemp102: F32 = self.fTempPerm97;
		let mut fTemp103: F32 = self.fTempPerm98;
		let mut fTemp104: F32 = self.fTempPerm99;
		let mut iTemp105: i32 = self.iTempPerm100;
		let mut iTemp106: i32 = self.iTempPerm101;
		let mut iTemp107: i32 = self.iTempPerm102;
		let mut fTemp108: F32 = self.fTempPerm103;
		let mut fTemp109: F32 = self.fTempPerm104;
		let mut iTemp110: i32 = self.iTempPerm105;
		let mut iTemp111: i32 = self.iTempPerm106;
		let mut fTemp112: F32 = self.fTempPerm107;
		let mut fTemp113: F32 = self.fTempPerm108;
		let mut fTemp114: F32 = self.fTempPerm109;
		let mut fTemp115: F32 = self.fTempPerm110;
		let mut iTemp116: i32 = self.iTempPerm111;
		let mut fTemp117: F32 = self.fTempPerm112;
		let mut iTemp118: i32 = self.iTempPerm113;
		let mut iTemp119: i32 = self.iTempPerm114;
		let mut fTemp120: F32 = self.fTempPerm115;
		let mut fTemp121: F32 = self.fTempPerm116;
		let mut fTemp122: F32 = self.fTempPerm117;
		let mut iTemp123: i32 = self.iTempPerm118;
		let mut fTemp124: F32 = self.fTempPerm119;
		let mut fTemp125: F32 = self.fTempPerm120;
		let mut fTemp126: F32 = self.fTempPerm121;
		let mut iTemp127: i32 = self.iTempPerm122;
		let mut fTemp128: F32 = self.fTempPerm123;
		let mut fTemp129: F32 = self.fTempPerm124;
		let mut fTemp130: F32 = self.fTempPerm125;
		let mut fTemp131: F32 = self.fTempPerm126;
		let mut fTemp132: F32 = self.fTempPerm127;
		let mut fTemp133: F32 = self.fTempPerm128;
		let mut fTemp134: F32 = self.fTempPerm129;
		let mut iTemp135: i32 = self.iTempPerm130;
		let mut iTemp136: i32 = self.iTempPerm131;
		let mut iTemp137: i32 = self.iTempPerm132;
		let mut iTemp138: i32 = self.iTempPerm133;
		let mut fTemp139: F32 = self.fTempPerm134;
		let mut fTemp140: F32 = self.fTempPerm135;
		let mut iTemp141: i32 = self.iTempPerm136;
		let mut iTemp142: i32 = self.iTempPerm137;
		let mut fTemp143: F32 = self.fTempPerm138;
		let mut fTemp144: F32 = self.fTempPerm139;
		let mut iTemp145: i32 = self.iTempPerm140;
		let mut fTemp146: F32 = self.fTempPerm141;
		let mut fTemp147: F32 = self.fTempPerm142;
		let mut fTemp148: F32 = self.fTempPerm143;
		let mut iTemp149: i32 = self.iTempPerm144;
		let mut iTemp150: i32 = self.iTempPerm145;
		let mut fTemp151: F32 = self.fTempPerm146;
		let mut fTemp152: F32 = self.fTempPerm147;
		let mut fTemp153: F32 = self.fTempPerm148;
		let mut fTemp154: F32 = self.fTempPerm149;
		let mut fTemp155: F32 = self.fTempPerm150;
		let mut fTemp156: F32 = self.fTempPerm151;
		let mut iTemp157: i32 = self.iTempPerm152;
		let mut fTemp158: F32 = self.fTempPerm153;
		let mut fTemp159: F32 = self.fTempPerm154;
		let mut fTemp160: F32 = self.fTempPerm155;
		let mut iTemp161: i32 = self.iTempPerm156;
		let mut iTemp162: i32 = self.iTempPerm157;
		let mut iTemp163: i32 = self.iTempPerm158;
		let mut fTemp164: F32 = self.fTempPerm159;
		let mut fTemp165: F32 = self.fTempPerm160;
		let mut iTemp166: i32 = self.iTempPerm161;
		let mut iTemp167: i32 = self.iTempPerm162;
		let mut fTemp168: F32 = self.fTempPerm163;
		let mut fTemp169: F32 = self.fTempPerm164;
		let mut fTemp170: F32 = self.fTempPerm165;
		let mut fTemp171: F32 = self.fTempPerm166;
		let mut iTemp172: i32 = self.iTempPerm167;
		let mut fTemp173: F32 = self.fTempPerm168;
		let mut iTemp174: i32 = self.iTempPerm169;
		let mut iTemp175: i32 = self.iTempPerm170;
		let mut fTemp176: F32 = self.fTempPerm171;
		let mut fTemp177: F32 = self.fTempPerm172;
		let mut fTemp178: F32 = self.fTempPerm173;
		let mut iTemp179: i32 = self.iTempPerm174;
		let mut fTemp180: F32 = self.fTempPerm175;
		let mut fTemp181: F32 = self.fTempPerm176;
		let mut fTemp182: F32 = self.fTempPerm177;
		let mut iTemp183: i32 = self.iTempPerm178;
		let mut fTemp184: F32 = self.fTempPerm179;
		let mut fTemp185: F32 = self.fTempPerm180;
		let mut fTemp186: F32 = self.fTempPerm181;
		let mut fTemp187: F32 = self.fTempPerm182;
		let mut fTemp188: F32 = self.fTempPerm183;
		let mut fTemp189: F32 = self.fTempPerm184;
		let mut fTemp190: F32 = self.fTempPerm185;
		let mut iTemp191: i32 = self.iTempPerm186;
		let mut iTemp192: i32 = self.iTempPerm187;
		let mut iTemp193: i32 = self.iTempPerm188;
		let mut iTemp194: i32 = self.iTempPerm189;
		let mut fTemp195: F32 = self.fTempPerm190;
		let mut fTemp196: F32 = self.fTempPerm191;
		let mut iTemp197: i32 = self.iTempPerm192;
		let mut iTemp198: i32 = self.iTempPerm193;
		let mut fTemp199: F32 = self.fTempPerm194;
		let mut fTemp200: F32 = self.fTempPerm195;
		let mut iTemp201: i32 = self.iTempPerm196;
		let mut fTemp202: F32 = self.fTempPerm197;
		let mut fTemp203: F32 = self.fTempPerm198;
		let mut fTemp204: F32 = self.fTempPerm199;
		let mut iTemp205: i32 = self.iTempPerm200;
		let mut iTemp206: i32 = self.iTempPerm201;
		let mut fTemp207: F32 = self.fTempPerm202;
		let mut fTemp208: F32 = self.fTempPerm203;
		let mut fTemp209: F32 = self.fTempPerm204;
		let mut fTemp210: F32 = self.fTempPerm205;
		let mut fTemp211: F32 = self.fTempPerm206;
		let mut fTemp212: F32 = self.fTempPerm207;
		let mut iTemp213: i32 = self.iTempPerm208;
		let mut fTemp214: F32 = self.fTempPerm209;
		let mut fTemp215: F32 = self.fTempPerm210;
		let mut fTemp216: F32 = self.fTempPerm211;
		let mut iTemp217: i32 = self.iTempPerm212;
		let mut iTemp218: i32 = self.iTempPerm213;
		let mut iTemp219: i32 = self.iTempPerm214;
		let mut fTemp220: F32 = self.fTempPerm215;
		let mut fTemp221: F32 = self.fTempPerm216;
		let mut iTemp222: i32 = self.iTempPerm217;
		let mut iTemp223: i32 = self.iTempPerm218;
		let mut fTemp224: F32 = self.fTempPerm219;
		let mut fTemp225: F32 = self.fTempPerm220;
		let mut fTemp226: F32 = self.fTempPerm221;
		let mut fTemp227: F32 = self.fTempPerm222;
		let mut iTemp228: i32 = self.iTempPerm223;
		let mut fTemp229: F32 = self.fTempPerm224;
		let mut iTemp230: i32 = self.iTempPerm225;
		let mut iTemp231: i32 = self.iTempPerm226;
		let mut fTemp232: F32 = self.fTempPerm227;
		let mut fTemp233: F32 = self.fTempPerm228;
		let mut fTemp234: F32 = self.fTempPerm229;
		let mut iTemp235: i32 = self.iTempPerm230;
		let mut fTemp236: F32 = self.fTempPerm231;
		let mut fTemp237: F32 = self.fTempPerm232;
		let mut fTemp238: F32 = self.fTempPerm233;
		let mut iTemp239: i32 = self.iTempPerm234;
		let mut fTemp240: F32 = self.fTempPerm235;
		let mut fTemp241: F32 = self.fTempPerm236;
		let mut fTemp242: F32 = self.fTempPerm237;
		let mut fTemp243: F32 = self.fTempPerm238;
		let mut fTemp244: F32 = self.fTempPerm239;
		let mut fTemp245: F32 = self.fTempPerm240;
		let mut fTemp246: F32 = self.fTempPerm241;
		let mut iTemp247: i32 = self.iTempPerm242;
		let mut iTemp248: i32 = self.iTempPerm243;
		let mut iTemp249: i32 = self.iTempPerm244;
		let mut iTemp250: i32 = self.iTempPerm245;
		let mut fTemp251: F32 = self.fTempPerm246;
		let mut fTemp252: F32 = self.fTempPerm247;
		let mut iTemp253: i32 = self.iTempPerm248;
		let mut iTemp254: i32 = self.iTempPerm249;
		let mut fTemp255: F32 = self.fTempPerm250;
		let mut fTemp256: F32 = self.fTempPerm251;
		let mut iTemp257: i32 = self.iTempPerm252;
		let mut fTemp258: F32 = self.fTempPerm253;
		let mut fTemp259: F32 = self.fTempPerm254;
		let mut fTemp260: F32 = self.fTempPerm255;
		let mut iTemp261: i32 = self.iTempPerm256;
		let mut iTemp262: i32 = self.iTempPerm257;
		let mut fTemp263: F32 = self.fTempPerm258;
		let mut fTemp264: F32 = self.fTempPerm259;
		let mut fTemp265: F32 = self.fTempPerm260;
		let mut fTemp266: F32 = self.fTempPerm261;
		let mut fTemp267: F32 = self.fTempPerm262;
		let mut fTemp268: F32 = self.fTempPerm263;
		let mut iTemp269: i32 = self.iTempPerm264;
		let mut fTemp270: F32 = self.fTempPerm265;
		let mut fTemp271: F32 = self.fTempPerm266;
		let mut fTemp272: F32 = self.fTempPerm267;
		let mut iTemp273: i32 = self.iTempPerm268;
		let mut iTemp274: i32 = self.iTempPerm269;
		let mut iTemp275: i32 = self.iTempPerm270;
		let mut fTemp276: F32 = self.fTempPerm271;
		let mut fTemp277: F32 = self.fTempPerm272;
		let mut iTemp278: i32 = self.iTempPerm273;
		let mut iTemp279: i32 = self.iTempPerm274;
		let mut fTemp280: F32 = self.fTempPerm275;
		let mut fTemp281: F32 = self.fTempPerm276;
		let mut fTemp282: F32 = self.fTempPerm277;
		let mut fTemp283: F32 = self.fTempPerm278;
		let mut iTemp284: i32 = self.iTempPerm279;
		let mut fTemp285: F32 = self.fTempPerm280;
		let mut iTemp286: i32 = self.iTempPerm281;
		let mut iTemp287: i32 = self.iTempPerm282;
		let mut fTemp288: F32 = self.fTempPerm283;
		let mut fTemp289: F32 = self.fTempPerm284;
		let mut fTemp290: F32 = self.fTempPerm285;
		let mut iTemp291: i32 = self.iTempPerm286;
		let mut fTemp292: F32 = self.fTempPerm287;
		let mut fTemp293: F32 = self.fTempPerm288;
		let mut fTemp294: F32 = self.fTempPerm289;
		let mut iTemp295: i32 = self.iTempPerm290;
		let mut fTemp296: F32 = self.fTempPerm291;
		let mut fTemp297: F32 = self.fTempPerm292;
		let mut fTemp298: F32 = self.fTempPerm293;
		let mut fTemp299: F32 = self.fTempPerm294;
		let mut fTemp300: F32 = self.fTempPerm295;
		let mut fTemp301: F32 = self.fTempPerm296;
		let mut fTemp302: F32 = self.fTempPerm297;
		let mut iTemp303: i32 = self.iTempPerm298;
		let mut iTemp304: i32 = self.iTempPerm299;
		let mut iTemp305: i32 = self.iTempPerm300;
		let mut fTemp306: F32 = self.fTempPerm301;
		let mut fTemp307: F32 = self.fTempPerm302;
		let mut iTemp308: i32 = self.iTempPerm303;
		let mut iTemp309: i32 = self.iTempPerm304;
		let mut fTemp310: F32 = self.fTempPerm305;
		let mut fTemp311: F32 = self.fTempPerm306;
		let mut iTemp312: i32 = self.iTempPerm307;
		let mut fTemp313: F32 = self.fTempPerm308;
		let mut fTemp314: F32 = self.fTempPerm309;
		let mut fTemp315: F32 = self.fTempPerm310;
		let mut iTemp316: i32 = self.iTempPerm311;
		let mut iTemp317: i32 = self.iTempPerm312;
		let mut fTemp318: F32 = self.fTempPerm313;
		let mut fTemp319: F32 = self.fTempPerm314;
		let mut fTemp320: F32 = self.fTempPerm315;
		let mut fTemp321: F32 = self.fTempPerm316;
		let mut fTemp322: F32 = self.fTempPerm317;
		let mut fTemp323: F32 = self.fTempPerm318;
		let mut iTemp324: i32 = self.iTempPerm319;
		let mut fTemp325: F32 = self.fTempPerm320;
		let mut fTemp326: F32 = self.fTempPerm321;
		let mut fTemp327: F32 = self.fTempPerm322;
		let mut iTemp328: i32 = self.iTempPerm323;
		let mut iTemp329: i32 = self.iTempPerm324;
		let mut iTemp330: i32 = self.iTempPerm325;
		let mut fTemp331: F32 = self.fTempPerm326;
		let mut fTemp332: F32 = self.fTempPerm327;
		let mut iTemp333: i32 = self.iTempPerm328;
		let mut iTemp334: i32 = self.iTempPerm329;
		let mut fTemp335: F32 = self.fTempPerm330;
		let mut fTemp336: F32 = self.fTempPerm331;
		let mut fTemp337: F32 = self.fTempPerm332;
		let mut fTemp338: F32 = self.fTempPerm333;
		let mut iTemp339: i32 = self.iTempPerm334;
		let mut fTemp340: F32 = self.fTempPerm335;
		let mut iTemp341: i32 = self.iTempPerm336;
		let mut iTemp342: i32 = self.iTempPerm337;
		let mut fTemp343: F32 = self.fTempPerm338;
		let mut fTemp344: F32 = self.fTempPerm339;
		let mut fTemp345: F32 = self.fTempPerm340;
		let mut iTemp346: i32 = self.iTempPerm341;
		let mut fTemp347: F32 = self.fTempPerm342;
		let mut fTemp348: F32 = self.fTempPerm343;
		let mut fTemp349: F32 = self.fTempPerm344;
		let mut iTemp350: i32 = self.iTempPerm345;
		let mut fTemp351: F32 = self.fTempPerm346;
		let mut fTemp352: F32 = self.fTempPerm347;
		let mut fTemp353: F32 = self.fTempPerm348;
		let mut fTemp354: F32 = self.fTempPerm349;
		let mut fTemp355: F32 = self.fTempPerm350;
		let mut fTemp356: F32 = self.fTempPerm351;
		let mut fTemp357: F32 = self.fTempPerm352;
		let mut fTemp358: F32 = self.fTempPerm353;
		let mut fTemp359: F32 = self.fTempPerm354;
		let mut fTemp360: F32 = self.fTempPerm355;
		let mut fTemp361: F32 = self.fTempPerm356;
		let mut fTemp362: F32 = self.fTempPerm357;
		let mut fTemp363: F32 = self.fTempPerm358;
		let mut fTemp364: F32 = self.fTempPerm359;
		let mut fTemp365: F32 = self.fTempPerm360;
		let mut fTemp366: F32 = self.fTempPerm361;
		let mut fTemp367: F32 = self.fTempPerm362;
		let mut fTemp368: F32 = self.fTempPerm363;
		let mut fTemp369: F32 = self.fTempPerm364;
		let mut fTemp370: F32 = self.fTempPerm365;
		let mut fTemp371: F32 = self.fTempPerm366;
		let mut fTemp372: F32 = self.fTempPerm367;
		let mut fTemp373: F32 = self.fTempPerm368;
		let mut fTemp374: F32 = self.fTempPerm369;
		let mut fTemp375: F32 = self.fTempPerm370;
		let mut fTemp376: F32 = self.fTempPerm371;
		let mut iTemp385: i32 = self.iTempPerm372;
		let mut iSlow18: i32 = (811.0 * fSlow8 + 2e+03) as i32;
		let mut iTemp386: i32 = self.iTempPerm373;
		let mut iTemp387: i32 = self.iTempPerm374;
		let mut iTemp388: i32 = self.iTempPerm375;
		let mut iTemp389: i32 = self.iTempPerm376;
		let mut iTemp390: i32 = self.iTempPerm377;
		let mut iTemp391: i32 = self.iTempPerm378;
		let mut iTemp392: i32 = self.iTempPerm379;
		let mut iTemp393: i32 = self.iTempPerm380;
		let mut iTemp394: i32 = self.iTempPerm381;
		let mut iTemp395: i32 = self.iTempPerm382;
		let mut iTemp396: i32 = self.iTempPerm383;
		let mut iTemp397: i32 = self.iTempPerm384;
		let mut iTemp398: i32 = self.iTempPerm385;
		let mut iTemp399: i32 = self.iTempPerm386;
		let mut fTemp400: F32 = self.fTempPerm387;
		let mut fTemp401: F32 = self.fTempPerm388;
		let mut iTemp402: i32 = self.iTempPerm389;
		let mut fTemp403: F32 = self.fTempPerm390;
		let mut fTemp404: F32 = self.fTempPerm391;
		let mut iTemp405: i32 = self.iTempPerm392;
		let mut fTemp406: F32 = self.fTempPerm393;
		let mut fTemp407: F32 = self.fTempPerm394;
		let mut fTemp408: F32 = self.fTempPerm395;
		let mut fSlow19: F32 = self.fHslider7;
		let mut fTemp409: F32 = self.fTempPerm396;
		let mut fTemp410: F32 = self.fTempPerm397;
		let mut fTemp411: F32 = self.fTempPerm398;
		let mut iTemp412: i32 = self.iTempPerm399;
		let mut iTemp413: i32 = self.iTempPerm400;
		let mut fTemp414: F32 = self.fTempPerm401;
		let mut iTemp415: i32 = self.iTempPerm402;
		let mut iTemp416: i32 = self.iTempPerm403;
		let mut fTemp417: F32 = self.fTempPerm404;
		let mut fTemp418: F32 = self.fTempPerm405;
		let mut fTemp419: F32 = self.fTempPerm406;
		let mut fTemp420: F32 = self.fTempPerm407;
		let mut fSlow20: F32 = self.fHslider8;
		let mut fTemp421: F32 = self.fTempPerm408;
		let mut fTemp422: F32 = self.fTempPerm409;
		let mut fTemp423: F32 = self.fTempPerm410;
		let mut iTemp424: i32 = self.iTempPerm411;
		let mut fTemp425: F32 = self.fTempPerm412;
		let mut iTemp426: i32 = self.iTempPerm413;
		let mut iTemp427: i32 = self.iTempPerm414;
		let mut fTemp428: F32 = self.fTempPerm415;
		let mut fTemp429: F32 = self.fTempPerm416;
		let mut iTemp430: i32 = self.iTempPerm417;
		let mut fTemp431: F32 = self.fTempPerm418;
		let mut iTemp432: i32 = self.iTempPerm419;
		let mut iTemp433: i32 = self.iTempPerm420;
		let mut iTemp434: i32 = self.iTempPerm421;
		let mut iTemp435: i32 = self.iTempPerm422;
		let mut fTemp436: F32 = self.fTempPerm423;
		let mut fTemp437: F32 = self.fTempPerm424;
		let mut fTemp438: F32 = self.fTempPerm425;
		let mut fTemp439: F32 = self.fTempPerm426;
		let mut fTemp440: F32 = self.fTempPerm427;
		let mut iTemp441: i32 = self.iTempPerm428;
		let mut iTemp442: i32 = self.iTempPerm429;
		let mut fTemp443: F32 = self.fTempPerm430;
		let mut fTemp444: F32 = self.fTempPerm431;
		let mut iTemp445: i32 = self.iTempPerm432;
		let mut fTemp446: F32 = self.fTempPerm433;
		let mut iTemp447: i32 = self.iTempPerm434;
		let mut iTemp448: i32 = self.iTempPerm435;
		let mut iTemp449: i32 = self.iTempPerm436;
		let mut iTemp450: i32 = self.iTempPerm437;
		let mut iTemp451: i32 = self.iTempPerm438;
		let mut fTemp452: F32 = self.fTempPerm439;
		let mut iTemp453: i32 = self.iTempPerm440;
		let mut iTemp454: i32 = self.iTempPerm441;
		let mut iTemp455: i32 = self.iTempPerm442;
		let mut iTemp456: i32 = self.iTempPerm443;
		let mut fTemp457: F32 = self.fTempPerm444;
		let mut fTemp458: F32 = self.fTempPerm445;
		let mut iTemp459: i32 = self.iTempPerm446;
		let mut fTemp460: F32 = self.fTempPerm447;
		let mut fTemp461: F32 = self.fTempPerm448;
		let mut iTemp462: i32 = self.iTempPerm449;
		let mut iTemp463: i32 = self.iTempPerm450;
		let mut fTemp464: F32 = self.fTempPerm451;
		let mut fTemp465: F32 = self.fTempPerm452;
		let mut iTemp466: i32 = self.iTempPerm453;
		let mut fTemp467: F32 = self.fTempPerm454;
		let mut fTemp468: F32 = self.fTempPerm455;
		let mut fTemp469: F32 = self.fTempPerm456;
		let mut fTemp470: F32 = self.fTempPerm457;
		let mut iTemp471: i32 = self.iTempPerm458;
		let mut fTemp472: F32 = self.fTempPerm459;
		let mut fTemp473: F32 = self.fTempPerm460;
		let mut fTemp474: F32 = self.fTempPerm461;
		let mut fTemp475: F32 = self.fTempPerm462;
		let mut iTemp476: i32 = self.iTempPerm463;
		let mut fTemp477: F32 = self.fTempPerm464;
		let mut fTemp478: F32 = self.fTempPerm465;
		let mut iTemp479: i32 = self.iTempPerm466;
		let mut iTemp480: i32 = self.iTempPerm467;
		let mut fTemp481: F32 = self.fTempPerm468;
		let mut fTemp482: F32 = self.fTempPerm469;
		let mut fSlow21: F32 = 2.0 * fSlow6 * ((iSlow5 != 0) as i32) as u32 as F32;
		let mut fTemp484: F32 = self.fTempPerm470;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(outputs0).zip(outputs1).zip(outputs2);
		for (((((input0, input1), input2), output0), output1), output2) in zipped_iterators {
			let mut fRec82: F32 = 0.0;
			let mut fRec70: F32 = 0.0;
			let mut fRec58: F32 = 0.0;
			let mut fRec46: F32 = 0.0;
			let mut fRec34: F32 = 0.0;
			let mut fRec20: F32 = 0.0;
			let mut iRec96: i32 = 0;
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow0;
			let mut iTemp0: i32 = (fSlow0 > self.fVec1[1]) as i32;
			let mut fTemp1: F32 = (i32::wrapping_sub(1, iTemp0)) as F32;
			self.fRec0[0] = fSlow1 * (1.0 - self.fConst1 * fTemp1) + self.fConst1 * fTemp1 * self.fRec0[1];
			self.fVec2[0] = fSlow2;
			let mut iTemp2: i32 = iSlow3 & ((self.fRec0[0] > 0.0001) as i32);
			let mut fTemp3: F32 = *input0;
			let mut fTemp4: F32 = (iTemp2) as F32;
			self.fVec3[0] = fSlow4;
			if ((fTemp4 != 0.0) as i32) | iSlow7 != 0 {
				iTemp5 = i32::wrapping_sub(1, self.iVec0[1]);
				iTemp6 = (iTemp5 > 0) as i32;
			}
				if (fTemp4 != 0.0) as i32 != 0 {
					self.fRec3[0] = self.fConst3 + (self.fRec3[1] - F32::floor(self.fConst3 + self.fRec3[1]));
					self.iRec4[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec4[1], i32::wrapping_mul(iSlow10, iTemp5))), 12345);
					iTemp7 = i32::wrapping_mul(1664525, self.iRec4[0] ^ (self.iRec4[0] >> 15));
					self.fRec2[0] = if ((self.fRec3[0] < self.fRec3[1]) as i32) | iTemp6 != 0 {4.656613e-10 * (i32::wrapping_mul(1013904223, iTemp7 ^ (iTemp7 >> 13))) as F32} else {self.fRec2[1]};
					self.fRec1[0] = self.fConst4 * self.fRec2[0] + self.fConst2 * self.fRec1[1];
					fTemp8 = 0.275 * self.fRec1[0] + 0.605;
					iTemp9 = ((self.fRec7[1] >= 1.0) as i32) | iTemp6;
					iTemp10 = (self.fRec7[1] < 0.0) as i32;
					self.iRec9[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec9[1], i32::wrapping_mul(iSlow11, iTemp5))), 12345);
					iTemp11 = i32::wrapping_add(self.iRec9[0], 3840715);
					iTemp12 = i32::wrapping_mul(1664525, iTemp11 ^ (iTemp11 >> 15));
				}
					if ((fTemp4 != 0.0) as i32) | iSlow7 != 0 {
						fTemp13 = *input1;
					}
						if (fTemp4 != 0.0) as i32 != 0 {
							fTemp14 = F32::log(F32::min(64.0, F32::max(1.0, self.fConst6 / fTemp13)), std::f32::consts::E);
							fTemp15 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp12 ^ (iTemp12 >> 13))) as F32 + 0.5);
							fTemp16 = F32::log(fTemp13, std::f32::consts::E);
							fTemp17 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp15) + -8.228819)));
							iTemp18 = i32::wrapping_add(self.iRec9[0], 3848634);
							iTemp19 = i32::wrapping_mul(1664525, iTemp18 ^ (iTemp18 >> 15));
							fTemp20 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp19 ^ (iTemp19 >> 13))) as F32;
							fTemp21 = F32::floor(fTemp20 - fTemp17);
							iTemp22 = (fTemp21) as i32;
							fTemp23 = fTemp20 - (fTemp17 + fTemp21);
							self.fVec4[0] = fSlow12;
							fTemp24 = if ((fSlow12 != self.fVec4[1]) as i32) | iTemp0 != 0 {fSlow13} else {self.fRec10[1] + -1.0};
							self.fRec10[0] = fTemp24;
							self.fRec11[0] = if iTemp0 != 0 {fSlow12} else {if (fTemp24 > 0.0) as i32 != 0 {self.fRec11[1] + (fSlow12 - self.fRec11[1]) / F32::max(1.0, fTemp24)} else {fSlow12}};
							fTemp25 = F32::max(0.005, 0.4 * (self.fRec11[0] * (fTemp23 * (fTemp23 * (fTemp23 * (fTemp23 * (0.0013333 * fTemp23 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp22))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp22))) as F32));
							self.fRec8[0] = if iTemp10 != 0 {fTemp25} else {self.fRec8[1]};
							self.fVec5[0] = fSlow14;
							fTemp26 = if ((fSlow14 != self.fVec5[1]) as i32) | iTemp0 != 0 {fSlow13} else {self.fRec12[1] + -1.0};
							self.fRec12[0] = fTemp26;
							self.fRec13[0] = if iTemp0 != 0 {fSlow14} else {if (fTemp26 > 0.0) as i32 != 0 {self.fRec13[1] + (fSlow14 - self.fRec13[1]) / F32::max(1.0, fTemp26)} else {fSlow14}};
							fTemp27 = F32::min(0.01 * self.fRec13[0], 0.98);
							fTemp28 = fTemp27 / (1.0 - fTemp27);
							fTemp29 = (iTemp9) as F32;
						}
							if ((fTemp4 != 0.0) as i32) & ((fTemp29 != 0.0) as i32) != 0 {
								iTemp30 = i32::wrapping_add(self.iRec9[0], 3856553);
								iTemp31 = i32::wrapping_mul(1664525, iTemp30 ^ (iTemp30 >> 15));
								fTemp32 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp31 ^ (iTemp31 >> 13))) as F32;
								fTemp33 = fTemp32 + 1.5;
								fTemp34 = 0.5 - fTemp32;
								fTemp35 = TextureDsp_faustpower2_f(fTemp34) / TextureDsp_faustpower2_f(fTemp33);
							}
								if (fTemp4 != 0.0) as i32 != 0 {
									self.fRec7[0] = if iTemp9 != 0 {-(2.0 * (fTemp34 * (15.0 - 4.0 * fTemp35) * fTemp29 / (fTemp33 * (15.0 - 9.0 * fTemp35))))} else {self.fRec7[1] + self.fConst7 * (if iTemp10 != 0 {fTemp28} else {1.0} / F32::max(self.fRec8[0], 0.005))};
									fTemp36 = F32::min(1.0, F32::max(0.0, self.fRec7[0]));
									fTemp37 = (iTemp10) as F32;
									iTemp38 = (fTemp37 != 0.0) as i32;
								}
									if ((fTemp4 != 0.0) as i32) & iTemp38 != 0 {
										fTemp39 = fTemp25 * fTemp37;
									}
										if (fTemp4 != 0.0) as i32 != 0 {
											self.fRec14[0] = if iTemp10 != 0 {fTemp39} else {self.fRec14[1]};
											fTemp40 = F32::floor(41.666668 * self.fRec14[0] * fTemp36);
											self.fVec6[0] = fTemp40;
											fTemp41 = ((fTemp40 != self.fVec6[1]) as i32) as u32 as F32;
											iTemp42 = (fTemp41 != 0.0) as i32;
										}
											if ((fTemp4 != 0.0) as i32) & iTemp42 != 0 {
												iTemp43 = i32::wrapping_add(self.iRec9[0], 3888229);
												iTemp44 = i32::wrapping_mul(1664525, iTemp43 ^ (iTemp43 >> 15));
												fTemp45 = 4.656613e-10 * fTemp41 * (i32::wrapping_mul(1013904223, iTemp44 ^ (iTemp44 >> 13))) as F32;
											}
												if (fTemp4 != 0.0) as i32 != 0 {
													fTemp46 = ((self.fRec7[0] >= 0.0) as i32) as u32 as F32;
													self.fRec6[0] = fTemp46 * (self.fRec6[1] + 0.35 * fTemp41 * fTemp45);
													self.fRec5[0] = self.fConst8 * self.fRec6[0] + self.fConst5 * self.fRec5[1];
												}
													if ((fTemp4 != 0.0) as i32) & iTemp38 != 0 {
														iTemp47 = i32::wrapping_add(self.iRec9[0], 3872391);
														iTemp48 = i32::wrapping_mul(1664525, iTemp47 ^ (iTemp47 >> 15));
														fTemp49 = 0.5 * fTemp37 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp48 ^ (iTemp48 >> 13))) as F32 + 0.5 + 1.0);
													}
														if (fTemp4 != 0.0) as i32 != 0 {
															self.fRec15[0] = if iTemp10 != 0 {fTemp49} else {self.fRec15[1]};
															fTemp50 = F32::min(1.0, F32::max(-1.0, self.fRec15[0] + self.fRec5[0] * fTemp8));
															fTemp51 = if ((fSlow2 != self.fVec2[1]) as i32) | iTemp0 != 0 {fSlow13} else {self.fRec16[1] + -1.0};
															self.fRec16[0] = fTemp51;
															self.fRec17[0] = if iTemp0 != 0 {fSlow2} else {if (fTemp51 > 0.0) as i32 != 0 {self.fRec17[1] + (fSlow2 - self.fRec17[1]) / F32::max(1.0, fTemp51)} else {fSlow2}};
															fTemp52 = self.fRec17[0] * (((fSlow15 * self.fRec17[0]) > 0.0001) as i32) as u32 as F32;
															fTemp53 = fSlow15 * fTemp52;
															fTemp54 = fSlow16 * fTemp52;
															fTemp55 = 2.6575425 * TextureDsp_faustpower2_f(fTemp36) * TextureDsp_faustpower2_f(1.0 - fTemp36) * fTemp50 * if (fTemp50 < 0.0) as i32 != 0 {fTemp54} else {fTemp53};
															fTemp56 = F32::floor(fTemp55);
															iTemp57 = (fTemp56) as i32;
															fTemp58 = fTemp55 - fTemp56;
															self.fVec7[0] = fSlow17;
															fTemp59 = if ((fSlow17 != self.fVec7[1]) as i32) | iTemp0 != 0 {fSlow13} else {self.fRec21[1] + -1.0};
															self.fRec21[0] = fTemp59;
															self.fRec22[0] = if iTemp0 != 0 {fSlow17} else {if (fTemp59 > 0.0) as i32 != 0 {self.fRec22[1] + (fSlow17 - self.fRec22[1]) / F32::max(1.0, fTemp59)} else {fSlow17}};
															fTemp60 = F32::min(1.92 / self.fRec22[0], 0.75);
														}
															if ((fTemp4 != 0.0) as i32) & iTemp42 != 0 {
																iTemp61 = i32::wrapping_add(self.iRec9[0], 3880310);
																iTemp62 = i32::wrapping_mul(1664525, iTemp61 ^ (iTemp61 >> 15));
																fTemp63 = 4.656613e-10 * fTemp41 * (i32::wrapping_mul(1013904223, iTemp62 ^ (iTemp62 >> 13))) as F32;
															}
																if (fTemp4 != 0.0) as i32 != 0 {
																	self.fRec24[0] = fTemp46 * (self.fRec24[1] + 0.35 * fTemp41 * fTemp63);
																	self.fRec23[0] = self.fConst8 * self.fRec24[0] + self.fConst5 * self.fRec23[1];
																	fTemp64 = self.fRec23[0] * fTemp8 * fTemp60;
																	fTemp65 = F32::floor(fTemp64);
																	iTemp66 = (fTemp65) as i32;
																	fTemp67 = fTemp64 - fTemp65;
																}
																	if ((fTemp4 != 0.0) as i32) & iTemp38 != 0 {
																		fTemp68 = 1.442695 * fTemp15;
																		fTemp69 = F32::floor(fTemp68);
																		iTemp70 = (fTemp69) as i32;
																		fTemp71 = fTemp68 - fTemp69;
																		fTemp72 = F32::round((fTemp71 * (fTemp71 * (fTemp71 * (fTemp71 * (0.0013333 * fTemp71 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp70))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp70))) as F32) * fTemp37;
																	}
																		if (fTemp4 != 0.0) as i32 != 0 {
																			self.fRec25[0] = if iTemp10 != 0 {fTemp72} else {self.fRec25[1]};
																			fTemp73 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec25[0] * (fTemp67 * (fTemp67 * (fTemp67 * (fTemp67 * (0.0013333 * fTemp67 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp66))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp66))) as F32));
																			fTemp74 = TextureDsp_faustpower2_f(fTemp73);
																			fTemp75 = self.fConst11 * fTemp74 * (self.fConst10 * fTemp74 + -444.1322) + 105.0;
																			fTemp76 = fTemp73 * (105.0 - self.fConst12 * fTemp74);
																			fTemp77 = 0.6666667 / self.fRec22[0];
																			fTemp78 = self.fConst13 * (fTemp76 * (fTemp77 + self.fConst13 * (fTemp76 / fTemp75)) / fTemp75) + 1.0;
																			iTemp79 = ((self.fRec28[1] >= 1.0) as i32) | iTemp6;
																			iTemp80 = (self.fRec28[1] < 0.0) as i32;
																			iTemp81 = i32::wrapping_add(self.iRec9[0], 3072572);
																			iTemp82 = i32::wrapping_mul(1664525, iTemp81 ^ (iTemp81 >> 15));
																			fTemp83 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp82 ^ (iTemp82 >> 13))) as F32 + 0.5);
																			fTemp84 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp83) + -8.228819)));
																			iTemp85 = i32::wrapping_add(self.iRec9[0], 3080491);
																			iTemp86 = i32::wrapping_mul(1664525, iTemp85 ^ (iTemp85 >> 15));
																			fTemp87 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp86 ^ (iTemp86 >> 13))) as F32;
																			fTemp88 = F32::floor(fTemp87 - fTemp84);
																			iTemp89 = (fTemp88) as i32;
																			fTemp90 = fTemp87 - (fTemp84 + fTemp88);
																			fTemp91 = F32::max(0.005, 0.4 * (self.fRec11[0] * (fTemp90 * (fTemp90 * (fTemp90 * (fTemp90 * (0.0013333 * fTemp90 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp89))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp89))) as F32));
																			self.fRec29[0] = if iTemp80 != 0 {fTemp91} else {self.fRec29[1]};
																			fTemp92 = (iTemp79) as F32;
																		}
																			if ((fTemp4 != 0.0) as i32) & ((fTemp92 != 0.0) as i32) != 0 {
																				iTemp93 = i32::wrapping_add(self.iRec9[0], 3088410);
																				iTemp94 = i32::wrapping_mul(1664525, iTemp93 ^ (iTemp93 >> 15));
																				fTemp95 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp94 ^ (iTemp94 >> 13))) as F32;
																				fTemp96 = fTemp95 + 1.5;
																				fTemp97 = 0.5 - fTemp95;
																				fTemp98 = TextureDsp_faustpower2_f(fTemp97) / TextureDsp_faustpower2_f(fTemp96);
																			}
																				if (fTemp4 != 0.0) as i32 != 0 {
																					self.fRec28[0] = if iTemp79 != 0 {-(2.0 * (fTemp97 * (15.0 - 4.0 * fTemp98) * fTemp92 / (fTemp96 * (15.0 - 9.0 * fTemp98))))} else {self.fRec28[1] + self.fConst7 * (if iTemp80 != 0 {fTemp28} else {1.0} / F32::max(self.fRec29[0], 0.005))};
																					fTemp99 = F32::min(1.0, F32::max(0.0, self.fRec28[0]));
																					fTemp100 = (iTemp80) as F32;
																					iTemp101 = (fTemp100 != 0.0) as i32;
																				}
																					if ((fTemp4 != 0.0) as i32) & iTemp101 != 0 {
																						fTemp102 = fTemp91 * fTemp100;
																					}
																						if (fTemp4 != 0.0) as i32 != 0 {
																							self.fRec30[0] = if iTemp80 != 0 {fTemp102} else {self.fRec30[1]};
																							fTemp103 = F32::floor(41.666668 * self.fRec30[0] * fTemp99);
																							self.fVec8[0] = fTemp103;
																							fTemp104 = ((fTemp103 != self.fVec8[1]) as i32) as u32 as F32;
																							iTemp105 = (fTemp104 != 0.0) as i32;
																						}
																							if ((fTemp4 != 0.0) as i32) & iTemp105 != 0 {
																								iTemp106 = i32::wrapping_add(self.iRec9[0], 3120086);
																								iTemp107 = i32::wrapping_mul(1664525, iTemp106 ^ (iTemp106 >> 15));
																								fTemp108 = 4.656613e-10 * fTemp104 * (i32::wrapping_mul(1013904223, iTemp107 ^ (iTemp107 >> 13))) as F32;
																							}
																								if (fTemp4 != 0.0) as i32 != 0 {
																									fTemp109 = ((self.fRec28[0] >= 0.0) as i32) as u32 as F32;
																									self.fRec27[0] = fTemp109 * (self.fRec27[1] + 0.35 * fTemp104 * fTemp108);
																									self.fRec26[0] = self.fConst8 * self.fRec27[0] + self.fConst5 * self.fRec26[1];
																								}
																									if ((fTemp4 != 0.0) as i32) & iTemp101 != 0 {
																										iTemp110 = i32::wrapping_add(self.iRec9[0], 3104248);
																										iTemp111 = i32::wrapping_mul(1664525, iTemp110 ^ (iTemp110 >> 15));
																										fTemp112 = 0.5 * fTemp100 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp111 ^ (iTemp111 >> 13))) as F32 + 0.5 + 1.0);
																									}
																										if (fTemp4 != 0.0) as i32 != 0 {
																											self.fRec31[0] = if iTemp80 != 0 {fTemp112} else {self.fRec31[1]};
																											fTemp113 = F32::min(1.0, F32::max(-1.0, self.fRec31[0] + self.fRec26[0] * fTemp8));
																											fTemp114 = 2.6575425 * TextureDsp_faustpower2_f(fTemp99) * TextureDsp_faustpower2_f(1.0 - fTemp99) * fTemp113 * if (fTemp113 < 0.0) as i32 != 0 {fTemp54} else {fTemp53};
																											fTemp115 = F32::floor(fTemp114);
																											iTemp116 = (fTemp115) as i32;
																											fTemp117 = fTemp114 - fTemp115;
																										}
																											if ((fTemp4 != 0.0) as i32) & iTemp105 != 0 {
																												iTemp118 = i32::wrapping_add(self.iRec9[0], 3112167);
																												iTemp119 = i32::wrapping_mul(1664525, iTemp118 ^ (iTemp118 >> 15));
																												fTemp120 = 4.656613e-10 * fTemp104 * (i32::wrapping_mul(1013904223, iTemp119 ^ (iTemp119 >> 13))) as F32;
																											}
																												if (fTemp4 != 0.0) as i32 != 0 {
																													self.fRec36[0] = fTemp109 * (self.fRec36[1] + 0.35 * fTemp104 * fTemp120);
																													self.fRec35[0] = self.fConst8 * self.fRec36[0] + self.fConst5 * self.fRec35[1];
																													fTemp121 = self.fRec35[0] * fTemp8 * fTemp60;
																													fTemp122 = F32::floor(fTemp121);
																													iTemp123 = (fTemp122) as i32;
																													fTemp124 = fTemp121 - fTemp122;
																												}
																													if ((fTemp4 != 0.0) as i32) & iTemp101 != 0 {
																														fTemp125 = 1.442695 * fTemp83;
																														fTemp126 = F32::floor(fTemp125);
																														iTemp127 = (fTemp126) as i32;
																														fTemp128 = fTemp125 - fTemp126;
																														fTemp129 = F32::round((fTemp128 * (fTemp128 * (fTemp128 * (fTemp128 * (0.0013333 * fTemp128 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp127))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp127))) as F32) * fTemp100;
																													}
																														if (fTemp4 != 0.0) as i32 != 0 {
																															self.fRec37[0] = if iTemp80 != 0 {fTemp129} else {self.fRec37[1]};
																															fTemp130 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec37[0] * (fTemp124 * (fTemp124 * (fTemp124 * (fTemp124 * (0.0013333 * fTemp124 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp123))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp123))) as F32));
																															fTemp131 = TextureDsp_faustpower2_f(fTemp130);
																															fTemp132 = self.fConst11 * fTemp131 * (self.fConst10 * fTemp131 + -444.1322) + 105.0;
																															fTemp133 = fTemp130 * (105.0 - self.fConst12 * fTemp131);
																															fTemp134 = self.fConst13 * (fTemp133 * (fTemp77 + self.fConst13 * (fTemp133 / fTemp132)) / fTemp132) + 1.0;
																															iTemp135 = ((self.fRec40[1] >= 1.0) as i32) | iTemp6;
																															iTemp136 = (self.fRec40[1] < 0.0) as i32;
																															iTemp137 = i32::wrapping_add(self.iRec9[0], 2304429);
																															iTemp138 = i32::wrapping_mul(1664525, iTemp137 ^ (iTemp137 >> 15));
																															fTemp139 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp138 ^ (iTemp138 >> 13))) as F32 + 0.5);
																															fTemp140 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp139) + -8.228819)));
																															iTemp141 = i32::wrapping_add(self.iRec9[0], 2312348);
																															iTemp142 = i32::wrapping_mul(1664525, iTemp141 ^ (iTemp141 >> 15));
																															fTemp143 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp142 ^ (iTemp142 >> 13))) as F32;
																															fTemp144 = F32::floor(fTemp143 - fTemp140);
																															iTemp145 = (fTemp144) as i32;
																															fTemp146 = fTemp143 - (fTemp140 + fTemp144);
																															fTemp147 = F32::max(0.005, 0.4 * (self.fRec11[0] * (fTemp146 * (fTemp146 * (fTemp146 * (fTemp146 * (0.0013333 * fTemp146 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp145))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp145))) as F32));
																															self.fRec41[0] = if iTemp136 != 0 {fTemp147} else {self.fRec41[1]};
																															fTemp148 = (iTemp135) as F32;
																														}
																															if ((fTemp4 != 0.0) as i32) & ((fTemp148 != 0.0) as i32) != 0 {
																																iTemp149 = i32::wrapping_add(self.iRec9[0], 2320267);
																																iTemp150 = i32::wrapping_mul(1664525, iTemp149 ^ (iTemp149 >> 15));
																																fTemp151 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp150 ^ (iTemp150 >> 13))) as F32;
																																fTemp152 = fTemp151 + 1.5;
																																fTemp153 = 0.5 - fTemp151;
																																fTemp154 = TextureDsp_faustpower2_f(fTemp153) / TextureDsp_faustpower2_f(fTemp152);
																															}
																																if (fTemp4 != 0.0) as i32 != 0 {
																																	self.fRec40[0] = if iTemp135 != 0 {-(2.0 * (fTemp153 * (15.0 - 4.0 * fTemp154) * fTemp148 / (fTemp152 * (15.0 - 9.0 * fTemp154))))} else {self.fRec40[1] + self.fConst7 * (if iTemp136 != 0 {fTemp28} else {1.0} / F32::max(self.fRec41[0], 0.005))};
																																	fTemp155 = F32::min(1.0, F32::max(0.0, self.fRec40[0]));
																																	fTemp156 = (iTemp136) as F32;
																																	iTemp157 = (fTemp156 != 0.0) as i32;
																																}
																																	if ((fTemp4 != 0.0) as i32) & iTemp157 != 0 {
																																		fTemp158 = fTemp147 * fTemp156;
																																	}
																																		if (fTemp4 != 0.0) as i32 != 0 {
																																			self.fRec42[0] = if iTemp136 != 0 {fTemp158} else {self.fRec42[1]};
																																			fTemp159 = F32::floor(41.666668 * self.fRec42[0] * fTemp155);
																																			self.fVec9[0] = fTemp159;
																																			fTemp160 = ((fTemp159 != self.fVec9[1]) as i32) as u32 as F32;
																																			iTemp161 = (fTemp160 != 0.0) as i32;
																																		}
																																			if ((fTemp4 != 0.0) as i32) & iTemp161 != 0 {
																																				iTemp162 = i32::wrapping_add(self.iRec9[0], 2351943);
																																				iTemp163 = i32::wrapping_mul(1664525, iTemp162 ^ (iTemp162 >> 15));
																																				fTemp164 = 4.656613e-10 * fTemp160 * (i32::wrapping_mul(1013904223, iTemp163 ^ (iTemp163 >> 13))) as F32;
																																			}
																																				if (fTemp4 != 0.0) as i32 != 0 {
																																					fTemp165 = ((self.fRec40[0] >= 0.0) as i32) as u32 as F32;
																																					self.fRec39[0] = fTemp165 * (self.fRec39[1] + 0.35 * fTemp160 * fTemp164);
																																					self.fRec38[0] = self.fConst8 * self.fRec39[0] + self.fConst5 * self.fRec38[1];
																																				}
																																					if ((fTemp4 != 0.0) as i32) & iTemp157 != 0 {
																																						iTemp166 = i32::wrapping_add(self.iRec9[0], 2336105);
																																						iTemp167 = i32::wrapping_mul(1664525, iTemp166 ^ (iTemp166 >> 15));
																																						fTemp168 = 0.5 * fTemp156 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp167 ^ (iTemp167 >> 13))) as F32 + 0.5 + 1.0);
																																					}
																																						if (fTemp4 != 0.0) as i32 != 0 {
																																							self.fRec43[0] = if iTemp136 != 0 {fTemp168} else {self.fRec43[1]};
																																							fTemp169 = F32::min(1.0, F32::max(-1.0, self.fRec43[0] + self.fRec38[0] * fTemp8));
																																							fTemp170 = 2.6575425 * TextureDsp_faustpower2_f(fTemp155) * TextureDsp_faustpower2_f(1.0 - fTemp155) * fTemp169 * if (fTemp169 < 0.0) as i32 != 0 {fTemp54} else {fTemp53};
																																							fTemp171 = F32::floor(fTemp170);
																																							iTemp172 = (fTemp171) as i32;
																																							fTemp173 = fTemp170 - fTemp171;
																																						}
																																							if ((fTemp4 != 0.0) as i32) & iTemp161 != 0 {
																																								iTemp174 = i32::wrapping_add(self.iRec9[0], 2344024);
																																								iTemp175 = i32::wrapping_mul(1664525, iTemp174 ^ (iTemp174 >> 15));
																																								fTemp176 = 4.656613e-10 * fTemp160 * (i32::wrapping_mul(1013904223, iTemp175 ^ (iTemp175 >> 13))) as F32;
																																							}
																																								if (fTemp4 != 0.0) as i32 != 0 {
																																									self.fRec48[0] = fTemp165 * (self.fRec48[1] + 0.35 * fTemp160 * fTemp176);
																																									self.fRec47[0] = self.fConst8 * self.fRec48[0] + self.fConst5 * self.fRec47[1];
																																									fTemp177 = self.fRec47[0] * fTemp8 * fTemp60;
																																									fTemp178 = F32::floor(fTemp177);
																																									iTemp179 = (fTemp178) as i32;
																																									fTemp180 = fTemp177 - fTemp178;
																																								}
																																									if ((fTemp4 != 0.0) as i32) & iTemp157 != 0 {
																																										fTemp181 = 1.442695 * fTemp139;
																																										fTemp182 = F32::floor(fTemp181);
																																										iTemp183 = (fTemp182) as i32;
																																										fTemp184 = fTemp181 - fTemp182;
																																										fTemp185 = F32::round((fTemp184 * (fTemp184 * (fTemp184 * (fTemp184 * (0.0013333 * fTemp184 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp183))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp183))) as F32) * fTemp156;
																																									}
																																										if (fTemp4 != 0.0) as i32 != 0 {
																																											self.fRec49[0] = if iTemp136 != 0 {fTemp185} else {self.fRec49[1]};
																																											fTemp186 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec49[0] * (fTemp180 * (fTemp180 * (fTemp180 * (fTemp180 * (0.0013333 * fTemp180 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp179))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp179))) as F32));
																																											fTemp187 = TextureDsp_faustpower2_f(fTemp186);
																																											fTemp188 = self.fConst11 * fTemp187 * (self.fConst10 * fTemp187 + -444.1322) + 105.0;
																																											fTemp189 = fTemp186 * (105.0 - self.fConst12 * fTemp187);
																																											fTemp190 = self.fConst13 * (fTemp189 * (fTemp77 + self.fConst13 * (fTemp189 / fTemp188)) / fTemp188) + 1.0;
																																											iTemp191 = ((self.fRec52[1] >= 1.0) as i32) | iTemp6;
																																											iTemp192 = (self.fRec52[1] < 0.0) as i32;
																																											iTemp193 = i32::wrapping_add(self.iRec9[0], 1536286);
																																											iTemp194 = i32::wrapping_mul(1664525, iTemp193 ^ (iTemp193 >> 15));
																																											fTemp195 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp194 ^ (iTemp194 >> 13))) as F32 + 0.5);
																																											fTemp196 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp195) + -8.228819)));
																																											iTemp197 = i32::wrapping_add(self.iRec9[0], 1544205);
																																											iTemp198 = i32::wrapping_mul(1664525, iTemp197 ^ (iTemp197 >> 15));
																																											fTemp199 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp198 ^ (iTemp198 >> 13))) as F32;
																																											fTemp200 = F32::floor(fTemp199 - fTemp196);
																																											iTemp201 = (fTemp200) as i32;
																																											fTemp202 = fTemp199 - (fTemp196 + fTemp200);
																																											fTemp203 = F32::max(0.005, 0.4 * (self.fRec11[0] * (fTemp202 * (fTemp202 * (fTemp202 * (fTemp202 * (0.0013333 * fTemp202 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp201))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp201))) as F32));
																																											self.fRec53[0] = if iTemp192 != 0 {fTemp203} else {self.fRec53[1]};
																																											fTemp204 = (iTemp191) as F32;
																																										}
																																											if ((fTemp4 != 0.0) as i32) & ((fTemp204 != 0.0) as i32) != 0 {
																																												iTemp205 = i32::wrapping_add(self.iRec9[0], 1552124);
																																												iTemp206 = i32::wrapping_mul(1664525, iTemp205 ^ (iTemp205 >> 15));
																																												fTemp207 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp206 ^ (iTemp206 >> 13))) as F32;
																																												fTemp208 = fTemp207 + 1.5;
																																												fTemp209 = 0.5 - fTemp207;
																																												fTemp210 = TextureDsp_faustpower2_f(fTemp209) / TextureDsp_faustpower2_f(fTemp208);
																																											}
																																												if (fTemp4 != 0.0) as i32 != 0 {
																																													self.fRec52[0] = if iTemp191 != 0 {-(2.0 * (fTemp209 * (15.0 - 4.0 * fTemp210) * fTemp204 / (fTemp208 * (15.0 - 9.0 * fTemp210))))} else {self.fRec52[1] + self.fConst7 * (if iTemp192 != 0 {fTemp28} else {1.0} / F32::max(self.fRec53[0], 0.005))};
																																													fTemp211 = F32::min(1.0, F32::max(0.0, self.fRec52[0]));
																																													fTemp212 = (iTemp192) as F32;
																																													iTemp213 = (fTemp212 != 0.0) as i32;
																																												}
																																													if ((fTemp4 != 0.0) as i32) & iTemp213 != 0 {
																																														fTemp214 = fTemp203 * fTemp212;
																																													}
																																														if (fTemp4 != 0.0) as i32 != 0 {
																																															self.fRec54[0] = if iTemp192 != 0 {fTemp214} else {self.fRec54[1]};
																																															fTemp215 = F32::floor(41.666668 * self.fRec54[0] * fTemp211);
																																															self.fVec10[0] = fTemp215;
																																															fTemp216 = ((fTemp215 != self.fVec10[1]) as i32) as u32 as F32;
																																															iTemp217 = (fTemp216 != 0.0) as i32;
																																														}
																																															if ((fTemp4 != 0.0) as i32) & iTemp217 != 0 {
																																																iTemp218 = i32::wrapping_add(self.iRec9[0], 1583800);
																																																iTemp219 = i32::wrapping_mul(1664525, iTemp218 ^ (iTemp218 >> 15));
																																																fTemp220 = 4.656613e-10 * fTemp216 * (i32::wrapping_mul(1013904223, iTemp219 ^ (iTemp219 >> 13))) as F32;
																																															}
																																																if (fTemp4 != 0.0) as i32 != 0 {
																																																	fTemp221 = ((self.fRec52[0] >= 0.0) as i32) as u32 as F32;
																																																	self.fRec51[0] = fTemp221 * (self.fRec51[1] + 0.35 * fTemp216 * fTemp220);
																																																	self.fRec50[0] = self.fConst8 * self.fRec51[0] + self.fConst5 * self.fRec50[1];
																																																}
																																																	if ((fTemp4 != 0.0) as i32) & iTemp213 != 0 {
																																																		iTemp222 = i32::wrapping_add(self.iRec9[0], 1567962);
																																																		iTemp223 = i32::wrapping_mul(1664525, iTemp222 ^ (iTemp222 >> 15));
																																																		fTemp224 = 0.5 * fTemp212 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp223 ^ (iTemp223 >> 13))) as F32 + 0.5 + 1.0);
																																																	}
																																																		if (fTemp4 != 0.0) as i32 != 0 {
																																																			self.fRec55[0] = if iTemp192 != 0 {fTemp224} else {self.fRec55[1]};
																																																			fTemp225 = F32::min(1.0, F32::max(-1.0, self.fRec55[0] + self.fRec50[0] * fTemp8));
																																																			fTemp226 = 2.6575425 * TextureDsp_faustpower2_f(fTemp211) * TextureDsp_faustpower2_f(1.0 - fTemp211) * fTemp225 * if (fTemp225 < 0.0) as i32 != 0 {fTemp54} else {fTemp53};
																																																			fTemp227 = F32::floor(fTemp226);
																																																			iTemp228 = (fTemp227) as i32;
																																																			fTemp229 = fTemp226 - fTemp227;
																																																		}
																																																			if ((fTemp4 != 0.0) as i32) & iTemp217 != 0 {
																																																				iTemp230 = i32::wrapping_add(self.iRec9[0], 1575881);
																																																				iTemp231 = i32::wrapping_mul(1664525, iTemp230 ^ (iTemp230 >> 15));
																																																				fTemp232 = 4.656613e-10 * fTemp216 * (i32::wrapping_mul(1013904223, iTemp231 ^ (iTemp231 >> 13))) as F32;
																																																			}
																																																				if (fTemp4 != 0.0) as i32 != 0 {
																																																					self.fRec60[0] = fTemp221 * (self.fRec60[1] + 0.35 * fTemp216 * fTemp232);
																																																					self.fRec59[0] = self.fConst8 * self.fRec60[0] + self.fConst5 * self.fRec59[1];
																																																					fTemp233 = self.fRec59[0] * fTemp8 * fTemp60;
																																																					fTemp234 = F32::floor(fTemp233);
																																																					iTemp235 = (fTemp234) as i32;
																																																					fTemp236 = fTemp233 - fTemp234;
																																																				}
																																																					if ((fTemp4 != 0.0) as i32) & iTemp213 != 0 {
																																																						fTemp237 = 1.442695 * fTemp195;
																																																						fTemp238 = F32::floor(fTemp237);
																																																						iTemp239 = (fTemp238) as i32;
																																																						fTemp240 = fTemp237 - fTemp238;
																																																						fTemp241 = F32::round((fTemp240 * (fTemp240 * (fTemp240 * (fTemp240 * (0.0013333 * fTemp240 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp239))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp239))) as F32) * fTemp212;
																																																					}
																																																						if (fTemp4 != 0.0) as i32 != 0 {
																																																							self.fRec61[0] = if iTemp192 != 0 {fTemp241} else {self.fRec61[1]};
																																																							fTemp242 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec61[0] * (fTemp236 * (fTemp236 * (fTemp236 * (fTemp236 * (0.0013333 * fTemp236 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp235))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp235))) as F32));
																																																							fTemp243 = TextureDsp_faustpower2_f(fTemp242);
																																																							fTemp244 = self.fConst11 * fTemp243 * (self.fConst10 * fTemp243 + -444.1322) + 105.0;
																																																							fTemp245 = fTemp242 * (105.0 - self.fConst12 * fTemp243);
																																																							fTemp246 = self.fConst13 * (fTemp245 * (fTemp77 + self.fConst13 * (fTemp245 / fTemp244)) / fTemp244) + 1.0;
																																																							iTemp247 = ((self.fRec64[1] >= 1.0) as i32) | iTemp6;
																																																							iTemp248 = (self.fRec64[1] < 0.0) as i32;
																																																							iTemp249 = i32::wrapping_add(self.iRec9[0], 768143);
																																																							iTemp250 = i32::wrapping_mul(1664525, iTemp249 ^ (iTemp249 >> 15));
																																																							fTemp251 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp250 ^ (iTemp250 >> 13))) as F32 + 0.5);
																																																							fTemp252 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp251) + -8.228819)));
																																																							iTemp253 = i32::wrapping_add(self.iRec9[0], 776062);
																																																							iTemp254 = i32::wrapping_mul(1664525, iTemp253 ^ (iTemp253 >> 15));
																																																							fTemp255 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp254 ^ (iTemp254 >> 13))) as F32;
																																																							fTemp256 = F32::floor(fTemp255 - fTemp252);
																																																							iTemp257 = (fTemp256) as i32;
																																																							fTemp258 = fTemp255 - (fTemp252 + fTemp256);
																																																							fTemp259 = F32::max(0.005, 0.4 * (self.fRec11[0] * (fTemp258 * (fTemp258 * (fTemp258 * (fTemp258 * (0.0013333 * fTemp258 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp257))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp257))) as F32));
																																																							self.fRec65[0] = if iTemp248 != 0 {fTemp259} else {self.fRec65[1]};
																																																							fTemp260 = (iTemp247) as F32;
																																																						}
																																																							if ((fTemp4 != 0.0) as i32) & ((fTemp260 != 0.0) as i32) != 0 {
																																																								iTemp261 = i32::wrapping_add(self.iRec9[0], 783981);
																																																								iTemp262 = i32::wrapping_mul(1664525, iTemp261 ^ (iTemp261 >> 15));
																																																								fTemp263 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp262 ^ (iTemp262 >> 13))) as F32;
																																																								fTemp264 = fTemp263 + 1.5;
																																																								fTemp265 = 0.5 - fTemp263;
																																																								fTemp266 = TextureDsp_faustpower2_f(fTemp265) / TextureDsp_faustpower2_f(fTemp264);
																																																							}
																																																								if (fTemp4 != 0.0) as i32 != 0 {
																																																									self.fRec64[0] = if iTemp247 != 0 {-(2.0 * (fTemp265 * (15.0 - 4.0 * fTemp266) * fTemp260 / (fTemp264 * (15.0 - 9.0 * fTemp266))))} else {self.fRec64[1] + self.fConst7 * (if iTemp248 != 0 {fTemp28} else {1.0} / F32::max(self.fRec65[0], 0.005))};
																																																									fTemp267 = F32::min(1.0, F32::max(0.0, self.fRec64[0]));
																																																									fTemp268 = (iTemp248) as F32;
																																																									iTemp269 = (fTemp268 != 0.0) as i32;
																																																								}
																																																									if ((fTemp4 != 0.0) as i32) & iTemp269 != 0 {
																																																										fTemp270 = fTemp259 * fTemp268;
																																																									}
																																																										if (fTemp4 != 0.0) as i32 != 0 {
																																																											self.fRec66[0] = if iTemp248 != 0 {fTemp270} else {self.fRec66[1]};
																																																											fTemp271 = F32::floor(41.666668 * self.fRec66[0] * fTemp267);
																																																											self.fVec11[0] = fTemp271;
																																																											fTemp272 = ((fTemp271 != self.fVec11[1]) as i32) as u32 as F32;
																																																											iTemp273 = (fTemp272 != 0.0) as i32;
																																																										}
																																																											if ((fTemp4 != 0.0) as i32) & iTemp273 != 0 {
																																																												iTemp274 = i32::wrapping_add(self.iRec9[0], 815657);
																																																												iTemp275 = i32::wrapping_mul(1664525, iTemp274 ^ (iTemp274 >> 15));
																																																												fTemp276 = 4.656613e-10 * fTemp272 * (i32::wrapping_mul(1013904223, iTemp275 ^ (iTemp275 >> 13))) as F32;
																																																											}
																																																												if (fTemp4 != 0.0) as i32 != 0 {
																																																													fTemp277 = ((self.fRec64[0] >= 0.0) as i32) as u32 as F32;
																																																													self.fRec63[0] = fTemp277 * (self.fRec63[1] + 0.35 * fTemp272 * fTemp276);
																																																													self.fRec62[0] = self.fConst8 * self.fRec63[0] + self.fConst5 * self.fRec62[1];
																																																												}
																																																													if ((fTemp4 != 0.0) as i32) & iTemp269 != 0 {
																																																														iTemp278 = i32::wrapping_add(self.iRec9[0], 799819);
																																																														iTemp279 = i32::wrapping_mul(1664525, iTemp278 ^ (iTemp278 >> 15));
																																																														fTemp280 = 0.5 * fTemp268 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp279 ^ (iTemp279 >> 13))) as F32 + 0.5 + 1.0);
																																																													}
																																																														if (fTemp4 != 0.0) as i32 != 0 {
																																																															self.fRec67[0] = if iTemp248 != 0 {fTemp280} else {self.fRec67[1]};
																																																															fTemp281 = F32::min(1.0, F32::max(-1.0, self.fRec67[0] + self.fRec62[0] * fTemp8));
																																																															fTemp282 = 2.6575425 * TextureDsp_faustpower2_f(fTemp267) * TextureDsp_faustpower2_f(1.0 - fTemp267) * fTemp281 * if (fTemp281 < 0.0) as i32 != 0 {fTemp54} else {fTemp53};
																																																															fTemp283 = F32::floor(fTemp282);
																																																															iTemp284 = (fTemp283) as i32;
																																																															fTemp285 = fTemp282 - fTemp283;
																																																														}
																																																															if ((fTemp4 != 0.0) as i32) & iTemp273 != 0 {
																																																																iTemp286 = i32::wrapping_add(self.iRec9[0], 807738);
																																																																iTemp287 = i32::wrapping_mul(1664525, iTemp286 ^ (iTemp286 >> 15));
																																																																fTemp288 = 4.656613e-10 * fTemp272 * (i32::wrapping_mul(1013904223, iTemp287 ^ (iTemp287 >> 13))) as F32;
																																																															}
																																																																if (fTemp4 != 0.0) as i32 != 0 {
																																																																	self.fRec72[0] = fTemp277 * (self.fRec72[1] + 0.35 * fTemp272 * fTemp288);
																																																																	self.fRec71[0] = self.fConst8 * self.fRec72[0] + self.fConst5 * self.fRec71[1];
																																																																	fTemp289 = self.fRec71[0] * fTemp8 * fTemp60;
																																																																	fTemp290 = F32::floor(fTemp289);
																																																																	iTemp291 = (fTemp290) as i32;
																																																																	fTemp292 = fTemp289 - fTemp290;
																																																																}
																																																																	if ((fTemp4 != 0.0) as i32) & iTemp269 != 0 {
																																																																		fTemp293 = 1.442695 * fTemp251;
																																																																		fTemp294 = F32::floor(fTemp293);
																																																																		iTemp295 = (fTemp294) as i32;
																																																																		fTemp296 = fTemp293 - fTemp294;
																																																																		fTemp297 = F32::round((fTemp296 * (fTemp296 * (fTemp296 * (fTemp296 * (0.0013333 * fTemp296 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp295))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp295))) as F32) * fTemp268;
																																																																	}
																																																																		if (fTemp4 != 0.0) as i32 != 0 {
																																																																			self.fRec73[0] = if iTemp248 != 0 {fTemp297} else {self.fRec73[1]};
																																																																			fTemp298 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec73[0] * (fTemp292 * (fTemp292 * (fTemp292 * (fTemp292 * (0.0013333 * fTemp292 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp291))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp291))) as F32));
																																																																			fTemp299 = TextureDsp_faustpower2_f(fTemp298);
																																																																			fTemp300 = self.fConst11 * fTemp299 * (self.fConst10 * fTemp299 + -444.1322) + 105.0;
																																																																			fTemp301 = fTemp298 * (105.0 - self.fConst12 * fTemp299);
																																																																			fTemp302 = self.fConst13 * (fTemp301 * (fTemp77 + self.fConst13 * (fTemp301 / fTemp300)) / fTemp300) + 1.0;
																																																																			iTemp303 = ((self.fRec76[1] >= 1.0) as i32) | iTemp6;
																																																																			iTemp304 = (self.fRec76[1] < 0.0) as i32;
																																																																			iTemp305 = i32::wrapping_mul(1664525, self.iRec9[0] ^ (self.iRec9[0] >> 15));
																																																																			fTemp306 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp305 ^ (iTemp305 >> 13))) as F32 + 0.5);
																																																																			fTemp307 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp306) + -8.228819)));
																																																																			iTemp308 = i32::wrapping_add(self.iRec9[0], 7919);
																																																																			iTemp309 = i32::wrapping_mul(1664525, iTemp308 ^ (iTemp308 >> 15));
																																																																			fTemp310 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp309 ^ (iTemp309 >> 13))) as F32;
																																																																			fTemp311 = F32::floor(fTemp310 - fTemp307);
																																																																			iTemp312 = (fTemp311) as i32;
																																																																			fTemp313 = fTemp310 - (fTemp307 + fTemp311);
																																																																			fTemp314 = F32::max(0.005, 0.4 * (self.fRec11[0] * (fTemp313 * (fTemp313 * (fTemp313 * (fTemp313 * (0.0013333 * fTemp313 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp312))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp312))) as F32));
																																																																			self.fRec77[0] = if iTemp304 != 0 {fTemp314} else {self.fRec77[1]};
																																																																			fTemp315 = (iTemp303) as F32;
																																																																		}
																																																																			if ((fTemp4 != 0.0) as i32) & ((fTemp315 != 0.0) as i32) != 0 {
																																																																				iTemp316 = i32::wrapping_add(self.iRec9[0], 15838);
																																																																				iTemp317 = i32::wrapping_mul(1664525, iTemp316 ^ (iTemp316 >> 15));
																																																																				fTemp318 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp317 ^ (iTemp317 >> 13))) as F32;
																																																																				fTemp319 = fTemp318 + 1.5;
																																																																				fTemp320 = 0.5 - fTemp318;
																																																																				fTemp321 = TextureDsp_faustpower2_f(fTemp320) / TextureDsp_faustpower2_f(fTemp319);
																																																																			}
																																																																				if (fTemp4 != 0.0) as i32 != 0 {
																																																																					self.fRec76[0] = if iTemp303 != 0 {-(2.0 * (fTemp320 * (15.0 - 4.0 * fTemp321) * fTemp315 / (fTemp319 * (15.0 - 9.0 * fTemp321))))} else {self.fRec76[1] + self.fConst7 * (if iTemp304 != 0 {fTemp28} else {1.0} / F32::max(self.fRec77[0], 0.005))};
																																																																					fTemp322 = F32::min(1.0, F32::max(0.0, self.fRec76[0]));
																																																																					fTemp323 = (iTemp304) as F32;
																																																																					iTemp324 = (fTemp323 != 0.0) as i32;
																																																																				}
																																																																					if ((fTemp4 != 0.0) as i32) & iTemp324 != 0 {
																																																																						fTemp325 = fTemp314 * fTemp323;
																																																																					}
																																																																						if (fTemp4 != 0.0) as i32 != 0 {
																																																																							self.fRec78[0] = if iTemp304 != 0 {fTemp325} else {self.fRec78[1]};
																																																																							fTemp326 = F32::floor(41.666668 * self.fRec78[0] * fTemp322);
																																																																							self.fVec12[0] = fTemp326;
																																																																							fTemp327 = ((fTemp326 != self.fVec12[1]) as i32) as u32 as F32;
																																																																							iTemp328 = (fTemp327 != 0.0) as i32;
																																																																						}
																																																																							if ((fTemp4 != 0.0) as i32) & iTemp328 != 0 {
																																																																								iTemp329 = i32::wrapping_add(self.iRec9[0], 47514);
																																																																								iTemp330 = i32::wrapping_mul(1664525, iTemp329 ^ (iTemp329 >> 15));
																																																																								fTemp331 = 4.656613e-10 * fTemp327 * (i32::wrapping_mul(1013904223, iTemp330 ^ (iTemp330 >> 13))) as F32;
																																																																							}
																																																																								if (fTemp4 != 0.0) as i32 != 0 {
																																																																									fTemp332 = ((self.fRec76[0] >= 0.0) as i32) as u32 as F32;
																																																																									self.fRec75[0] = fTemp332 * (self.fRec75[1] + 0.35 * fTemp327 * fTemp331);
																																																																									self.fRec74[0] = self.fConst8 * self.fRec75[0] + self.fConst5 * self.fRec74[1];
																																																																								}
																																																																									if ((fTemp4 != 0.0) as i32) & iTemp324 != 0 {
																																																																										iTemp333 = i32::wrapping_add(self.iRec9[0], 31676);
																																																																										iTemp334 = i32::wrapping_mul(1664525, iTemp333 ^ (iTemp333 >> 15));
																																																																										fTemp335 = 0.5 * fTemp323 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp334 ^ (iTemp334 >> 13))) as F32 + 0.5 + 1.0);
																																																																									}
																																																																										if (fTemp4 != 0.0) as i32 != 0 {
																																																																											self.fRec79[0] = if iTemp304 != 0 {fTemp335} else {self.fRec79[1]};
																																																																											fTemp336 = F32::min(1.0, F32::max(-1.0, self.fRec79[0] + self.fRec74[0] * fTemp8));
																																																																											fTemp337 = 2.6575425 * TextureDsp_faustpower2_f(fTemp322) * TextureDsp_faustpower2_f(1.0 - fTemp322) * fTemp336 * if (fTemp336 < 0.0) as i32 != 0 {fTemp54} else {fTemp53};
																																																																											fTemp338 = F32::floor(fTemp337);
																																																																											iTemp339 = (fTemp338) as i32;
																																																																											fTemp340 = fTemp337 - fTemp338;
																																																																										}
																																																																											if ((fTemp4 != 0.0) as i32) & iTemp328 != 0 {
																																																																												iTemp341 = i32::wrapping_add(self.iRec9[0], 39595);
																																																																												iTemp342 = i32::wrapping_mul(1664525, iTemp341 ^ (iTemp341 >> 15));
																																																																												fTemp343 = 4.656613e-10 * fTemp327 * (i32::wrapping_mul(1013904223, iTemp342 ^ (iTemp342 >> 13))) as F32;
																																																																											}
																																																																												if (fTemp4 != 0.0) as i32 != 0 {
																																																																													self.fRec84[0] = fTemp332 * (self.fRec84[1] + 0.35 * fTemp327 * fTemp343);
																																																																													self.fRec83[0] = self.fConst8 * self.fRec84[0] + self.fConst5 * self.fRec83[1];
																																																																													fTemp344 = self.fRec83[0] * fTemp8 * fTemp60;
																																																																													fTemp345 = F32::floor(fTemp344);
																																																																													iTemp346 = (fTemp345) as i32;
																																																																													fTemp347 = fTemp344 - fTemp345;
																																																																												}
																																																																													if ((fTemp4 != 0.0) as i32) & iTemp324 != 0 {
																																																																														fTemp348 = 1.442695 * fTemp306;
																																																																														fTemp349 = F32::floor(fTemp348);
																																																																														iTemp350 = (fTemp349) as i32;
																																																																														fTemp351 = fTemp348 - fTemp349;
																																																																														fTemp352 = F32::round((fTemp351 * (fTemp351 * (fTemp351 * (fTemp351 * (0.0013333 * fTemp351 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp350))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp350))) as F32) * fTemp323;
																																																																													}
																																																																														if (fTemp4 != 0.0) as i32 != 0 {
																																																																															self.fRec85[0] = if iTemp304 != 0 {fTemp352} else {self.fRec85[1]};
																																																																															fTemp353 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec85[0] * (fTemp347 * (fTemp347 * (fTemp347 * (fTemp347 * (0.0013333 * fTemp347 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp346))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp346))) as F32));
																																																																															fTemp354 = TextureDsp_faustpower2_f(fTemp353);
																																																																															fTemp355 = self.fConst11 * fTemp354 * (self.fConst10 * fTemp354 + -444.1322) + 105.0;
																																																																															fTemp356 = fTemp353 * (105.0 - self.fConst12 * fTemp354);
																																																																															fTemp357 = self.fConst13 * (fTemp356 * (fTemp77 + self.fConst13 * (fTemp356 / fTemp355)) / fTemp355) + 1.0;
																																																																															fTemp358 = fTemp356 * (fTemp3 - self.fRec81[1]) / fTemp355;
																																																																															fTemp359 = self.fRec80[1] + self.fConst13 * fTemp358;
																																																																															self.fRec80[0] = 2.0 * (fTemp359 / fTemp357) - self.fRec80[1];
																																																																															self.fRec81[0] = 2.0 * (self.fRec81[1] + self.fConst7 * (fTemp356 * (3.1415927 * self.fRec80[1] + self.fConst14 * fTemp358) / (fTemp355 * fTemp357))) - self.fRec81[1];
																																																																															fRec82 = 0.6666667 * (fTemp359 / (self.fRec22[0] * fTemp357));
																																																																															fTemp360 = fTemp3 + fRec82 * ((fTemp340 * (fTemp340 * (fTemp340 * (fTemp340 * (0.0013333 * fTemp340 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp339))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp339))) as F32 + -1.0);
																																																																															fTemp361 = fTemp301 * (fTemp360 - self.fRec69[1]) / fTemp300;
																																																																															fTemp362 = self.fRec68[1] + self.fConst13 * fTemp361;
																																																																															self.fRec68[0] = 2.0 * (fTemp362 / fTemp302) - self.fRec68[1];
																																																																															self.fRec69[0] = 2.0 * (self.fRec69[1] + self.fConst7 * (fTemp301 * (3.1415927 * self.fRec68[1] + self.fConst14 * fTemp361) / (fTemp300 * fTemp302))) - self.fRec69[1];
																																																																															fRec70 = 0.6666667 * (fTemp362 / (self.fRec22[0] * fTemp302));
																																																																															fTemp363 = fTemp360 + fRec70 * ((fTemp285 * (fTemp285 * (fTemp285 * (fTemp285 * (0.0013333 * fTemp285 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp284))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp284))) as F32 + -1.0);
																																																																															fTemp364 = fTemp245 * (fTemp363 - self.fRec57[1]) / fTemp244;
																																																																															fTemp365 = self.fRec56[1] + self.fConst13 * fTemp364;
																																																																															self.fRec56[0] = 2.0 * (fTemp365 / fTemp246) - self.fRec56[1];
																																																																															self.fRec57[0] = 2.0 * (self.fRec57[1] + self.fConst7 * (fTemp245 * (3.1415927 * self.fRec56[1] + self.fConst14 * fTemp364) / (fTemp244 * fTemp246))) - self.fRec57[1];
																																																																															fRec58 = 0.6666667 * (fTemp365 / (self.fRec22[0] * fTemp246));
																																																																															fTemp366 = fTemp363 + fRec58 * ((fTemp229 * (fTemp229 * (fTemp229 * (fTemp229 * (0.0013333 * fTemp229 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp228))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp228))) as F32 + -1.0);
																																																																															fTemp367 = fTemp189 * (fTemp366 - self.fRec45[1]) / fTemp188;
																																																																															fTemp368 = self.fRec44[1] + self.fConst13 * fTemp367;
																																																																															self.fRec44[0] = 2.0 * (fTemp368 / fTemp190) - self.fRec44[1];
																																																																															self.fRec45[0] = 2.0 * (self.fRec45[1] + self.fConst7 * (fTemp189 * (3.1415927 * self.fRec44[1] + self.fConst14 * fTemp367) / (fTemp188 * fTemp190))) - self.fRec45[1];
																																																																															fRec46 = 0.6666667 * (fTemp368 / (self.fRec22[0] * fTemp190));
																																																																															fTemp369 = fTemp366 + fRec46 * ((fTemp173 * (fTemp173 * (fTemp173 * (fTemp173 * (0.0013333 * fTemp173 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp172))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp172))) as F32 + -1.0);
																																																																															fTemp370 = fTemp133 * (fTemp369 - self.fRec33[1]) / fTemp132;
																																																																															fTemp371 = self.fRec32[1] + self.fConst13 * fTemp370;
																																																																															self.fRec32[0] = 2.0 * (fTemp371 / fTemp134) - self.fRec32[1];
																																																																															self.fRec33[0] = 2.0 * (self.fRec33[1] + self.fConst7 * (fTemp133 * (3.1415927 * self.fRec32[1] + self.fConst14 * fTemp370) / (fTemp132 * fTemp134))) - self.fRec33[1];
																																																																															fRec34 = 0.6666667 * (fTemp371 / (self.fRec22[0] * fTemp134));
																																																																															fTemp372 = fTemp369 + fRec34 * ((fTemp117 * (fTemp117 * (fTemp117 * (fTemp117 * (0.0013333 * fTemp117 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp116))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp116))) as F32 + -1.0);
																																																																															fTemp373 = fTemp76 * (fTemp372 - self.fRec19[1]) / fTemp75;
																																																																															fTemp374 = self.fRec18[1] + self.fConst13 * fTemp373;
																																																																															self.fRec18[0] = 2.0 * (fTemp374 / fTemp78) - self.fRec18[1];
																																																																															self.fRec19[0] = 2.0 * (self.fRec19[1] + self.fConst7 * (fTemp76 * (3.1415927 * self.fRec18[1] + self.fConst14 * fTemp373) / (fTemp75 * fTemp78))) - self.fRec19[1];
																																																																															fRec20 = 0.6666667 * (fTemp374 / (self.fRec22[0] * fTemp78));
																																																																															fTemp375 = fTemp372 + fRec20 * ((fTemp58 * (fTemp58 * (fTemp58 * (fTemp58 * (0.0013333 * fTemp58 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp57))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp57))) as F32 + -1.0);
																																																																															self.fRec86[0] = self.fConst16 * TextureDsp_faustpower2_f(fTemp375) + self.fConst15 * self.fRec86[1];
																																																																															self.fRec87[0] = self.fConst16 * TextureDsp_faustpower2_f(fTemp3) + self.fConst15 * self.fRec87[1];
																																																																															fTemp376 = F32::sqrt((self.fRec87[0] + 1e-09) / (self.fRec86[0] + 1e-09)) * fTemp4 * fTemp375;
																																																																														}
																																																																															*output0 = if iTemp2 != 0 {fTemp3 + self.fRec0[0] * (((iTemp2 != 0) as i32) as u32 as F32 * fTemp376 - fTemp3)} else {fTemp3};
																																																																															let mut fTemp377: F32 = if ((fSlow4 != self.fVec3[1]) as i32) | iTemp0 != 0 {fSlow13} else {self.fRec88[1] + -1.0};
																																																																															self.fRec88[0] = fTemp377;
																																																																															self.fRec89[0] = if iTemp0 != 0 {fSlow4} else {if (fTemp377 > 0.0) as i32 != 0 {self.fRec89[1] + (fSlow4 - self.fRec89[1]) / F32::max(1.0, fTemp377)} else {fSlow4}};
																																																																															let mut fTemp378: F32 = 0.1660964 * self.fRec89[0];
																																																																															let mut fTemp379: F32 = F32::floor(fTemp378);
																																																																															let mut iTemp380: i32 = (fTemp379) as i32;
																																																																															let mut fTemp381: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iTemp380))) as F32;
																																																																															let mut fTemp382: F32 = (1 << std::cmp::min(30, std::cmp::max(0, iTemp380))) as F32;
																																																																															let mut fTemp383: F32 = fTemp378 - fTemp379;
																																																																															let mut fTemp384: F32 = fTemp383 * (fTemp383 * (fTemp383 * (fTemp383 * (0.0013333 * fTemp383 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0;
																																																																															if iSlow7 != 0 {
																																																																																iTemp385 = (self.iRec91[1] >= self.iRec92[1]) as i32;
																																																																																self.iRec91[0] = if iTemp385 != 0 {0} else {i32::wrapping_add(self.iRec91[1], 1)};
																																																																																self.iRec97[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec97[1], i32::wrapping_mul(iSlow18, iTemp5))), 12345);
																																																																																iTemp386 = i32::wrapping_add(self.iRec97[0], 104729);
																																																																																iTemp387 = i32::wrapping_mul(1664525, iTemp386 ^ (iTemp386 >> 15));
																																																																																iTemp388 = i32::wrapping_add(self.iRec95[1], 17);
																																																																																iTemp389 = i32::wrapping_mul(1664525, iTemp388 ^ (iTemp388 >> 15));
																																																																																iTemp390 = i32::wrapping_add(self.iRec94[1], 1);
																																																																																iTemp391 = iTemp385 & ((iTemp390 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp389 ^ (iTemp389 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
																																																																																iTemp392 = i32::wrapping_add(self.iRec97[0], 209458);
																																																																																iTemp393 = i32::wrapping_mul(1664525, iTemp392 ^ (iTemp392 >> 15));
																																																																																iTemp394 = if (iTemp391 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp387 ^ (iTemp387 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp6 != 0 {i32::wrapping_mul(1013904223, iTemp393 ^ (iTemp393 >> 13))} else {self.iRec95[1]};
																																																																																iTemp395 = i32::wrapping_add(iTemp394, 17);
																																																																																iTemp396 = i32::wrapping_mul(1664525, iTemp395 ^ (iTemp395 >> 15));
																																																																																iTemp397 = if iTemp385 != 0 {if iTemp391 != 0 {0} else {iTemp390}} else {self.iRec94[1]};
																																																																																iTemp398 = i32::wrapping_add(iTemp394, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp397), 1)));
																																																																																iTemp399 = i32::wrapping_mul(1664525, iTemp398 ^ (iTemp398 >> 15));
																																																																																fTemp400 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp399 ^ (iTemp399 >> 13))) as F32 + 0.5);
																																																																																fTemp401 = F32::floor(fTemp400);
																																																																																iTemp402 = (fTemp401) as i32;
																																																																																fTemp403 = 0.7213475 * F32::log(0.0038221916 * fTemp13, std::f32::consts::E);
																																																																																fTemp404 = F32::floor(fTemp403);
																																																																																iTemp405 = (fTemp404) as i32;
																																																																																fTemp406 = (1 << std::cmp::min(30, std::cmp::max(0, iTemp405))) as F32;
																																																																																fTemp407 = fTemp403 - fTemp404;
																																																																																fTemp408 = fTemp407 * (fTemp407 * (fTemp407 * (fTemp407 * (0.0013333 * fTemp407 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0;
																																																																																self.fVec13[0] = fSlow19;
																																																																																fTemp409 = if ((fSlow19 != self.fVec13[1]) as i32) | iTemp0 != 0 {fSlow13} else {self.fRec98[1] + -1.0};
																																																																																self.fRec98[0] = fTemp409;
																																																																																self.fRec99[0] = if iTemp0 != 0 {fSlow19} else {if (fTemp409 > 0.0) as i32 != 0 {self.fRec99[1] + (fSlow19 - self.fRec99[1]) / F32::max(1.0, fTemp409)} else {fSlow19}};
																																																																																fTemp410 = fTemp400 - fTemp401;
																																																																																fTemp411 = (1 << std::cmp::min(30, std::cmp::max(0, -iTemp405))) as F32;
																																																																																iTemp412 = i32::wrapping_add(self.iRec97[0], 314187);
																																																																																iTemp413 = i32::wrapping_mul(1664525, iTemp412 ^ (iTemp412 >> 15));
																																																																																fTemp414 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp413 ^ (iTemp413 >> 13))) as F32 + 1.0;
																																																																																iTemp415 = i32::wrapping_add(self.iRec97[0], 418916);
																																																																																iTemp416 = i32::wrapping_mul(1664525, iTemp415 ^ (iTemp415 >> 15));
																																																																																fTemp417 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp416 ^ (iTemp416 >> 13))) as F32;
																																																																																fTemp418 = fTemp417 + 1.5;
																																																																																fTemp419 = 0.5 - fTemp417;
																																																																																fTemp420 = TextureDsp_faustpower2_f(fTemp419) / TextureDsp_faustpower2_f(fTemp418);
																																																																																self.fVec14[0] = fSlow20;
																																																																																fTemp421 = if ((fSlow20 != self.fVec14[1]) as i32) | iTemp0 != 0 {fSlow13} else {self.fRec100[1] + -1.0};
																																																																																self.fRec100[0] = fTemp421;
																																																																																self.fRec101[0] = if iTemp0 != 0 {fSlow20} else {if (fTemp421 > 0.0) as i32 != 0 {self.fRec101[1] + (fSlow20 - self.fRec101[1]) / F32::max(1.0, fTemp421)} else {fSlow20}};
																																																																																fTemp422 = 6.9068904 * self.fRec101[0];
																																																																																fTemp423 = F32::floor(-fTemp422);
																																																																																iTemp424 = (fTemp423) as i32;
																																																																																fTemp425 = fTemp422 + fTemp423;
																																																																																iTemp426 = i32::wrapping_add(iTemp394, i32::wrapping_mul(190681, iTemp397));
																																																																																iTemp427 = i32::wrapping_mul(1664525, iTemp426 ^ (iTemp426 >> 15));
																																																																																fTemp428 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp427 ^ (iTemp427 >> 13))) as F32 + 0.5);
																																																																																fTemp429 = F32::floor(fTemp428);
																																																																																iTemp430 = (fTemp429) as i32;
																																																																																fTemp431 = fTemp428 - fTemp429;
																																																																																iTemp432 = if iTemp385 != 0 {i32::wrapping_mul(i32::wrapping_sub(1, iTemp6), (self.fConst17 * ((fTemp431 * (fTemp431 * (fTemp431 * (fTemp431 * (0.0013333 * fTemp431 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp430))) as F32 * fTemp414 * fTemp411 / (self.fRec99[0] * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp430))) as F32 * fTemp408 * fTemp406))) as i32)} else {self.iRec93[1]};
																																																																																self.iRec92[0] = if iTemp385 != 0 {i32::wrapping_add(iTemp432, (self.fConst0 * if (((i32::wrapping_add(iTemp397, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp396 ^ (iTemp396 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp6 != 0 {1.2e+02 * ((1.0 - fTemp425 * (0.693147 - fTemp425 * (0.240227 - fTemp425 * (0.0555041 - fTemp425 * (0.0096181 - 0.0013333 * fTemp425))))) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp424))) as F32 * fTemp419 * (15.0 - 4.0 * fTemp420) / ((1 << std::cmp::min(30, std::cmp::max(0, -iTemp424))) as F32 * fTemp418 * (15.0 - 9.0 * fTemp420)))} else {0.02 * (fTemp414 * fTemp411 * (fTemp410 * (fTemp410 * (fTemp410 * (fTemp410 * (0.0013333 * fTemp410 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp402))) as F32 / (self.fRec99[0] * fTemp408 * fTemp406 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp402))) as F32))}) as i32)} else {self.iRec92[1]};
																																																																																self.iRec93[0] = iTemp432;
																																																																																self.iRec94[0] = iTemp397;
																																																																																self.iRec95[0] = iTemp394;
																																																																																iRec96 = iTemp385;
																																																																																iTemp433 = i32::wrapping_mul(31, self.iRec94[0]);
																																																																																iTemp434 = i32::wrapping_add(self.iRec95[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp433, 8)));
																																																																																iTemp435 = i32::wrapping_mul(1664525, iTemp434 ^ (iTemp434 >> 15));
																																																																																self.fRec90[0] = if iRec96 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp435 ^ (iTemp435 >> 13))) as F32 + 0.5} else {self.fRec90[1]};
																																																																																fTemp436 = 1.5707964 * self.fRec90[0];
																																																																																fTemp437 = F32::min(1.0, F32::max(0.0, (self.iRec91[0]) as F32 / (std::cmp::max(self.iRec93[0], 1)) as F32));
																																																																																fTemp438 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp437)));
																																																																																fTemp439 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp437));
																																																																																fTemp440 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp437 + -0.6)));
																																																																																iTemp441 = i32::wrapping_add(self.iRec95[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp433, 7)));
																																																																																iTemp442 = i32::wrapping_mul(1664525, iTemp441 ^ (iTemp441 >> 15));
																																																																																fTemp443 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp442 ^ (iTemp442 >> 13))) as F32 + 0.5);
																																																																																fTemp444 = F32::floor(fTemp443);
																																																																																iTemp445 = (fTemp444) as i32;
																																																																																fTemp446 = fTemp443 - fTemp444;
																																																																																iTemp447 = i32::wrapping_add(self.iRec95[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp433, 6)));
																																																																																iTemp448 = i32::wrapping_mul(1664525, iTemp447 ^ (iTemp447 >> 15));
																																																																																iTemp449 = i32::wrapping_add(self.iRec95[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp433, 4)));
																																																																																iTemp450 = i32::wrapping_mul(1664525, iTemp449 ^ (iTemp449 >> 15));
																																																																																iTemp451 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp450 ^ (iTemp450 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
																																																																																self.fRec103[0] = if iRec96 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp451, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp448 ^ (iTemp448 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp446 * (fTemp446 * (fTemp446 * (fTemp446 * (0.0013333 * fTemp446 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp445))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp445))) as F32)} else {self.fRec103[1]};
																																																																																fTemp452 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp437 + -0.1)));
																																																																																iTemp453 = i32::wrapping_add(self.iRec97[0], 628374);
																																																																																iTemp454 = i32::wrapping_mul(1664525, iTemp453 ^ (iTemp453 >> 15));
																																																																																iTemp455 = i32::wrapping_add(self.iRec95[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp433, 3)));
																																																																																iTemp456 = i32::wrapping_mul(1664525, iTemp455 ^ (iTemp455 >> 15));
																																																																																fTemp457 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp456 ^ (iTemp456 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp454 ^ (iTemp454 >> 13))) as F32 + 0.5));
																																																																																fTemp458 = F32::floor(fTemp457);
																																																																																iTemp459 = (fTemp458) as i32;
																																																																																fTemp460 = fTemp457 - fTemp458;
																																																																																fTemp461 = F32::max(1.0, F32::round(5e+02 * ((fTemp460 * (fTemp460 * (fTemp460 * (fTemp460 * (0.0013333 * fTemp460 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp459))) as F32 / (fTemp13 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp459))) as F32))));
																																																																																iTemp462 = i32::wrapping_add(self.iRec95[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp433, 5)));
																																																																																iTemp463 = i32::wrapping_mul(1664525, iTemp462 ^ (iTemp462 >> 15));
																																																																																fTemp464 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp463 ^ (iTemp463 >> 13))) as F32 + 0.5);
																																																																																fTemp465 = F32::floor(fTemp464);
																																																																																iTemp466 = (fTemp465) as i32;
																																																																																fTemp467 = fTemp464 - fTemp465;
																																																																																fTemp468 = (iTemp451) as F32;
																																																																																fTemp469 = 0.16666667 * (fTemp468 * (fTemp467 * (fTemp467 * (fTemp467 * (fTemp467 * (0.0013333 * fTemp467 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp466))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp466))) as F32);
																																																																																fTemp470 = F32::floor(fTemp469);
																																																																																iTemp471 = (fTemp470) as i32;
																																																																																fTemp472 = fTemp469 - fTemp470;
																																																																																fTemp473 = F32::round(fTemp461 * (fTemp472 * (fTemp472 * (fTemp472 * (fTemp472 * (0.0013333 * fTemp472 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp471))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp471))) as F32);
																																																																																self.fRec104[0] = if iRec96 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp473 == fTemp461) as i32 != 0 {fTemp461 + fTemp468} else {fTemp473}) / fTemp461, std::f32::consts::E)} else {self.fRec104[1]};
																																																																																self.fRec105[0] = if iRec96 != 0 {1.442695 * F32::log(fTemp461, std::f32::consts::E)} else {self.fRec105[1]};
																																																																																fTemp474 = self.fRec105[0] + self.fRec104[0] * fTemp452 * (2.0 - fTemp452) + self.fRec103[0] * TextureDsp_faustpower2_f(fTemp440) * (3.0 - 2.0 * fTemp440);
																																																																																fTemp475 = F32::floor(fTemp474);
																																																																																iTemp476 = (fTemp475) as i32;
																																																																																fTemp477 = fTemp474 - fTemp475;
																																																																																fTemp478 = self.fRec102[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fTemp13 * (fTemp477 * (fTemp477 * (fTemp477 * (fTemp477 * (0.0013333 * fTemp477 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp476))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp476))) as F32));
																																																																																self.fRec102[0] = fTemp478 - F32::floor(fTemp478);
																																																																																iTemp479 = i32::wrapping_add(self.iRec95[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp433, 2)));
																																																																																iTemp480 = i32::wrapping_mul(1664525, iTemp479 ^ (iTemp479 >> 15));
																																																																																self.fRec106[0] = if iRec96 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp480 ^ (iTemp480 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec106[1]};
																																																																																fTemp481 = self.fRec106[0] * F32::sin(6.2831855 * self.fRec102[0]) * ((self.iRec93[0] > 0) as i32) as u32 as F32 * TextureDsp_faustpower2_f(fTemp439) * (3.0 - 2.0 * fTemp439) * TextureDsp_faustpower2_f(fTemp438) * (3.0 - 2.0 * fTemp438);
																																																																																fTemp482 = fSlow6 * fTemp481 * F32::cos(fTemp436);
																																																																															}
																																																																																let mut fTemp483: F32 = *input2;
																																																																																*output1 = fSlow21 * (fTemp483 * fTemp482 * fTemp384 * fTemp382 / fTemp381);
																																																																																if iSlow7 != 0 {
																																																																																	fTemp484 = fSlow6 * fTemp481 * F32::sin(fTemp436);
																																																																																}
																																																																																	*output2 = fSlow21 * (fTemp483 * fTemp384 * fTemp382 * fTemp484 / fTemp381);
																																																																																	self.iVec0[1] = self.iVec0[0];
																																																																																	self.fVec1[1] = self.fVec1[0];
																																																																																	self.fRec0[1] = self.fRec0[0];
																																																																																	self.fVec2[1] = self.fVec2[0];
																																																																																	self.fVec3[1] = self.fVec3[0];
																																																																																	if (fTemp4 != 0.0) as i32 != 0 {
																																																																																		self.fRec3[1] = self.fRec3[0];
																																																																																		self.iRec4[1] = self.iRec4[0];
																																																																																		self.fRec2[1] = self.fRec2[0];
																																																																																		self.fRec1[1] = self.fRec1[0];
																																																																																		self.iRec9[1] = self.iRec9[0];
																																																																																		self.fVec4[1] = self.fVec4[0];
																																																																																		self.fRec10[1] = self.fRec10[0];
																																																																																		self.fRec11[1] = self.fRec11[0];
																																																																																		self.fRec8[1] = self.fRec8[0];
																																																																																		self.fVec5[1] = self.fVec5[0];
																																																																																		self.fRec12[1] = self.fRec12[0];
																																																																																		self.fRec13[1] = self.fRec13[0];
																																																																																		self.fRec7[1] = self.fRec7[0];
																																																																																		self.fRec14[1] = self.fRec14[0];
																																																																																		self.fVec6[1] = self.fVec6[0];
																																																																																		self.fRec6[1] = self.fRec6[0];
																																																																																		self.fRec5[1] = self.fRec5[0];
																																																																																		self.fRec15[1] = self.fRec15[0];
																																																																																		self.fRec16[1] = self.fRec16[0];
																																																																																		self.fRec17[1] = self.fRec17[0];
																																																																																		self.fVec7[1] = self.fVec7[0];
																																																																																		self.fRec21[1] = self.fRec21[0];
																																																																																		self.fRec22[1] = self.fRec22[0];
																																																																																		self.fRec24[1] = self.fRec24[0];
																																																																																		self.fRec23[1] = self.fRec23[0];
																																																																																		self.fRec25[1] = self.fRec25[0];
																																																																																		self.fRec29[1] = self.fRec29[0];
																																																																																		self.fRec28[1] = self.fRec28[0];
																																																																																		self.fRec30[1] = self.fRec30[0];
																																																																																		self.fVec8[1] = self.fVec8[0];
																																																																																		self.fRec27[1] = self.fRec27[0];
																																																																																		self.fRec26[1] = self.fRec26[0];
																																																																																		self.fRec31[1] = self.fRec31[0];
																																																																																		self.fRec36[1] = self.fRec36[0];
																																																																																		self.fRec35[1] = self.fRec35[0];
																																																																																		self.fRec37[1] = self.fRec37[0];
																																																																																		self.fRec41[1] = self.fRec41[0];
																																																																																		self.fRec40[1] = self.fRec40[0];
																																																																																		self.fRec42[1] = self.fRec42[0];
																																																																																		self.fVec9[1] = self.fVec9[0];
																																																																																		self.fRec39[1] = self.fRec39[0];
																																																																																		self.fRec38[1] = self.fRec38[0];
																																																																																		self.fRec43[1] = self.fRec43[0];
																																																																																		self.fRec48[1] = self.fRec48[0];
																																																																																		self.fRec47[1] = self.fRec47[0];
																																																																																		self.fRec49[1] = self.fRec49[0];
																																																																																		self.fRec53[1] = self.fRec53[0];
																																																																																		self.fRec52[1] = self.fRec52[0];
																																																																																		self.fRec54[1] = self.fRec54[0];
																																																																																		self.fVec10[1] = self.fVec10[0];
																																																																																		self.fRec51[1] = self.fRec51[0];
																																																																																		self.fRec50[1] = self.fRec50[0];
																																																																																		self.fRec55[1] = self.fRec55[0];
																																																																																		self.fRec60[1] = self.fRec60[0];
																																																																																		self.fRec59[1] = self.fRec59[0];
																																																																																		self.fRec61[1] = self.fRec61[0];
																																																																																		self.fRec65[1] = self.fRec65[0];
																																																																																		self.fRec64[1] = self.fRec64[0];
																																																																																		self.fRec66[1] = self.fRec66[0];
																																																																																		self.fVec11[1] = self.fVec11[0];
																																																																																		self.fRec63[1] = self.fRec63[0];
																																																																																		self.fRec62[1] = self.fRec62[0];
																																																																																		self.fRec67[1] = self.fRec67[0];
																																																																																		self.fRec72[1] = self.fRec72[0];
																																																																																		self.fRec71[1] = self.fRec71[0];
																																																																																		self.fRec73[1] = self.fRec73[0];
																																																																																		self.fRec77[1] = self.fRec77[0];
																																																																																		self.fRec76[1] = self.fRec76[0];
																																																																																		self.fRec78[1] = self.fRec78[0];
																																																																																		self.fVec12[1] = self.fVec12[0];
																																																																																		self.fRec75[1] = self.fRec75[0];
																																																																																		self.fRec74[1] = self.fRec74[0];
																																																																																		self.fRec79[1] = self.fRec79[0];
																																																																																		self.fRec84[1] = self.fRec84[0];
																																																																																		self.fRec83[1] = self.fRec83[0];
																																																																																		self.fRec85[1] = self.fRec85[0];
																																																																																		self.fRec80[1] = self.fRec80[0];
																																																																																		self.fRec81[1] = self.fRec81[0];
																																																																																		self.fRec68[1] = self.fRec68[0];
																																																																																		self.fRec69[1] = self.fRec69[0];
																																																																																		self.fRec56[1] = self.fRec56[0];
																																																																																		self.fRec57[1] = self.fRec57[0];
																																																																																		self.fRec44[1] = self.fRec44[0];
																																																																																		self.fRec45[1] = self.fRec45[0];
																																																																																		self.fRec32[1] = self.fRec32[0];
																																																																																		self.fRec33[1] = self.fRec33[0];
																																																																																		self.fRec18[1] = self.fRec18[0];
																																																																																		self.fRec19[1] = self.fRec19[0];
																																																																																		self.fRec86[1] = self.fRec86[0];
																																																																																		self.fRec87[1] = self.fRec87[0];
																																																																																	}
																																																																																		self.fRec88[1] = self.fRec88[0];
																																																																																		self.fRec89[1] = self.fRec89[0];
																																																																																		if iSlow7 != 0 {
																																																																																			self.iRec91[1] = self.iRec91[0];
																																																																																			self.iRec97[1] = self.iRec97[0];
																																																																																			self.fVec13[1] = self.fVec13[0];
																																																																																			self.fRec98[1] = self.fRec98[0];
																																																																																			self.fRec99[1] = self.fRec99[0];
																																																																																			self.fVec14[1] = self.fVec14[0];
																																																																																			self.fRec100[1] = self.fRec100[0];
																																																																																			self.fRec101[1] = self.fRec101[0];
																																																																																			self.iRec92[1] = self.iRec92[0];
																																																																																			self.iRec93[1] = self.iRec93[0];
																																																																																			self.iRec94[1] = self.iRec94[0];
																																																																																			self.iRec95[1] = self.iRec95[0];
																																																																																			self.fRec90[1] = self.fRec90[0];
																																																																																			self.fRec103[1] = self.fRec103[0];
																																																																																			self.fRec104[1] = self.fRec104[0];
																																																																																			self.fRec105[1] = self.fRec105[0];
																																																																																			self.fRec102[1] = self.fRec102[0];
																																																																																			self.fRec106[1] = self.fRec106[0];
																																																																																		}
																																																																																		}
																																																																																		self.iTempPerm0 = iTemp5;
																																																																																		self.iTempPerm1 = iTemp6;
																																																																																		self.iTempPerm2 = iTemp7;
																																																																																		self.fTempPerm3 = fTemp8;
																																																																																		self.iTempPerm4 = iTemp9;
																																																																																		self.iTempPerm5 = iTemp10;
																																																																																		self.iTempPerm6 = iTemp11;
																																																																																		self.iTempPerm7 = iTemp12;
																																																																																		self.fTempPerm8 = fTemp13;
																																																																																		self.fTempPerm9 = fTemp14;
																																																																																		self.fTempPerm10 = fTemp15;
																																																																																		self.fTempPerm11 = fTemp16;
																																																																																		self.fTempPerm12 = fTemp17;
																																																																																		self.iTempPerm13 = iTemp18;
																																																																																		self.iTempPerm14 = iTemp19;
																																																																																		self.fTempPerm15 = fTemp20;
																																																																																		self.fTempPerm16 = fTemp21;
																																																																																		self.iTempPerm17 = iTemp22;
																																																																																		self.fTempPerm18 = fTemp23;
																																																																																		self.fTempPerm19 = fTemp24;
																																																																																		self.fTempPerm20 = fTemp25;
																																																																																		self.fTempPerm21 = fTemp26;
																																																																																		self.fTempPerm22 = fTemp27;
																																																																																		self.fTempPerm23 = fTemp28;
																																																																																		self.fTempPerm24 = fTemp29;
																																																																																		self.iTempPerm25 = iTemp30;
																																																																																		self.iTempPerm26 = iTemp31;
																																																																																		self.fTempPerm27 = fTemp32;
																																																																																		self.fTempPerm28 = fTemp33;
																																																																																		self.fTempPerm29 = fTemp34;
																																																																																		self.fTempPerm30 = fTemp35;
																																																																																		self.fTempPerm31 = fTemp36;
																																																																																		self.fTempPerm32 = fTemp37;
																																																																																		self.iTempPerm33 = iTemp38;
																																																																																		self.fTempPerm34 = fTemp39;
																																																																																		self.fTempPerm35 = fTemp40;
																																																																																		self.fTempPerm36 = fTemp41;
																																																																																		self.iTempPerm37 = iTemp42;
																																																																																		self.iTempPerm38 = iTemp43;
																																																																																		self.iTempPerm39 = iTemp44;
																																																																																		self.fTempPerm40 = fTemp45;
																																																																																		self.fTempPerm41 = fTemp46;
																																																																																		self.iTempPerm42 = iTemp47;
																																																																																		self.iTempPerm43 = iTemp48;
																																																																																		self.fTempPerm44 = fTemp49;
																																																																																		self.fTempPerm45 = fTemp50;
																																																																																		self.fTempPerm46 = fTemp51;
																																																																																		self.fTempPerm47 = fTemp52;
																																																																																		self.fTempPerm48 = fTemp53;
																																																																																		self.fTempPerm49 = fTemp54;
																																																																																		self.fTempPerm50 = fTemp55;
																																																																																		self.fTempPerm51 = fTemp56;
																																																																																		self.iTempPerm52 = iTemp57;
																																																																																		self.fTempPerm53 = fTemp58;
																																																																																		self.fTempPerm54 = fTemp59;
																																																																																		self.fTempPerm55 = fTemp60;
																																																																																		self.iTempPerm56 = iTemp61;
																																																																																		self.iTempPerm57 = iTemp62;
																																																																																		self.fTempPerm58 = fTemp63;
																																																																																		self.fTempPerm59 = fTemp64;
																																																																																		self.fTempPerm60 = fTemp65;
																																																																																		self.iTempPerm61 = iTemp66;
																																																																																		self.fTempPerm62 = fTemp67;
																																																																																		self.fTempPerm63 = fTemp68;
																																																																																		self.fTempPerm64 = fTemp69;
																																																																																		self.iTempPerm65 = iTemp70;
																																																																																		self.fTempPerm66 = fTemp71;
																																																																																		self.fTempPerm67 = fTemp72;
																																																																																		self.fTempPerm68 = fTemp73;
																																																																																		self.fTempPerm69 = fTemp74;
																																																																																		self.fTempPerm70 = fTemp75;
																																																																																		self.fTempPerm71 = fTemp76;
																																																																																		self.fTempPerm72 = fTemp77;
																																																																																		self.fTempPerm73 = fTemp78;
																																																																																		self.iTempPerm74 = iTemp79;
																																																																																		self.iTempPerm75 = iTemp80;
																																																																																		self.iTempPerm76 = iTemp81;
																																																																																		self.iTempPerm77 = iTemp82;
																																																																																		self.fTempPerm78 = fTemp83;
																																																																																		self.fTempPerm79 = fTemp84;
																																																																																		self.iTempPerm80 = iTemp85;
																																																																																		self.iTempPerm81 = iTemp86;
																																																																																		self.fTempPerm82 = fTemp87;
																																																																																		self.fTempPerm83 = fTemp88;
																																																																																		self.iTempPerm84 = iTemp89;
																																																																																		self.fTempPerm85 = fTemp90;
																																																																																		self.fTempPerm86 = fTemp91;
																																																																																		self.fTempPerm87 = fTemp92;
																																																																																		self.iTempPerm88 = iTemp93;
																																																																																		self.iTempPerm89 = iTemp94;
																																																																																		self.fTempPerm90 = fTemp95;
																																																																																		self.fTempPerm91 = fTemp96;
																																																																																		self.fTempPerm92 = fTemp97;
																																																																																		self.fTempPerm93 = fTemp98;
																																																																																		self.fTempPerm94 = fTemp99;
																																																																																		self.fTempPerm95 = fTemp100;
																																																																																		self.iTempPerm96 = iTemp101;
																																																																																		self.fTempPerm97 = fTemp102;
																																																																																		self.fTempPerm98 = fTemp103;
																																																																																		self.fTempPerm99 = fTemp104;
																																																																																		self.iTempPerm100 = iTemp105;
																																																																																		self.iTempPerm101 = iTemp106;
																																																																																		self.iTempPerm102 = iTemp107;
																																																																																		self.fTempPerm103 = fTemp108;
																																																																																		self.fTempPerm104 = fTemp109;
																																																																																		self.iTempPerm105 = iTemp110;
																																																																																		self.iTempPerm106 = iTemp111;
																																																																																		self.fTempPerm107 = fTemp112;
																																																																																		self.fTempPerm108 = fTemp113;
																																																																																		self.fTempPerm109 = fTemp114;
																																																																																		self.fTempPerm110 = fTemp115;
																																																																																		self.iTempPerm111 = iTemp116;
																																																																																		self.fTempPerm112 = fTemp117;
																																																																																		self.iTempPerm113 = iTemp118;
																																																																																		self.iTempPerm114 = iTemp119;
																																																																																		self.fTempPerm115 = fTemp120;
																																																																																		self.fTempPerm116 = fTemp121;
																																																																																		self.fTempPerm117 = fTemp122;
																																																																																		self.iTempPerm118 = iTemp123;
																																																																																		self.fTempPerm119 = fTemp124;
																																																																																		self.fTempPerm120 = fTemp125;
																																																																																		self.fTempPerm121 = fTemp126;
																																																																																		self.iTempPerm122 = iTemp127;
																																																																																		self.fTempPerm123 = fTemp128;
																																																																																		self.fTempPerm124 = fTemp129;
																																																																																		self.fTempPerm125 = fTemp130;
																																																																																		self.fTempPerm126 = fTemp131;
																																																																																		self.fTempPerm127 = fTemp132;
																																																																																		self.fTempPerm128 = fTemp133;
																																																																																		self.fTempPerm129 = fTemp134;
																																																																																		self.iTempPerm130 = iTemp135;
																																																																																		self.iTempPerm131 = iTemp136;
																																																																																		self.iTempPerm132 = iTemp137;
																																																																																		self.iTempPerm133 = iTemp138;
																																																																																		self.fTempPerm134 = fTemp139;
																																																																																		self.fTempPerm135 = fTemp140;
																																																																																		self.iTempPerm136 = iTemp141;
																																																																																		self.iTempPerm137 = iTemp142;
																																																																																		self.fTempPerm138 = fTemp143;
																																																																																		self.fTempPerm139 = fTemp144;
																																																																																		self.iTempPerm140 = iTemp145;
																																																																																		self.fTempPerm141 = fTemp146;
																																																																																		self.fTempPerm142 = fTemp147;
																																																																																		self.fTempPerm143 = fTemp148;
																																																																																		self.iTempPerm144 = iTemp149;
																																																																																		self.iTempPerm145 = iTemp150;
																																																																																		self.fTempPerm146 = fTemp151;
																																																																																		self.fTempPerm147 = fTemp152;
																																																																																		self.fTempPerm148 = fTemp153;
																																																																																		self.fTempPerm149 = fTemp154;
																																																																																		self.fTempPerm150 = fTemp155;
																																																																																		self.fTempPerm151 = fTemp156;
																																																																																		self.iTempPerm152 = iTemp157;
																																																																																		self.fTempPerm153 = fTemp158;
																																																																																		self.fTempPerm154 = fTemp159;
																																																																																		self.fTempPerm155 = fTemp160;
																																																																																		self.iTempPerm156 = iTemp161;
																																																																																		self.iTempPerm157 = iTemp162;
																																																																																		self.iTempPerm158 = iTemp163;
																																																																																		self.fTempPerm159 = fTemp164;
																																																																																		self.fTempPerm160 = fTemp165;
																																																																																		self.iTempPerm161 = iTemp166;
																																																																																		self.iTempPerm162 = iTemp167;
																																																																																		self.fTempPerm163 = fTemp168;
																																																																																		self.fTempPerm164 = fTemp169;
																																																																																		self.fTempPerm165 = fTemp170;
																																																																																		self.fTempPerm166 = fTemp171;
																																																																																		self.iTempPerm167 = iTemp172;
																																																																																		self.fTempPerm168 = fTemp173;
																																																																																		self.iTempPerm169 = iTemp174;
																																																																																		self.iTempPerm170 = iTemp175;
																																																																																		self.fTempPerm171 = fTemp176;
																																																																																		self.fTempPerm172 = fTemp177;
																																																																																		self.fTempPerm173 = fTemp178;
																																																																																		self.iTempPerm174 = iTemp179;
																																																																																		self.fTempPerm175 = fTemp180;
																																																																																		self.fTempPerm176 = fTemp181;
																																																																																		self.fTempPerm177 = fTemp182;
																																																																																		self.iTempPerm178 = iTemp183;
																																																																																		self.fTempPerm179 = fTemp184;
																																																																																		self.fTempPerm180 = fTemp185;
																																																																																		self.fTempPerm181 = fTemp186;
																																																																																		self.fTempPerm182 = fTemp187;
																																																																																		self.fTempPerm183 = fTemp188;
																																																																																		self.fTempPerm184 = fTemp189;
																																																																																		self.fTempPerm185 = fTemp190;
																																																																																		self.iTempPerm186 = iTemp191;
																																																																																		self.iTempPerm187 = iTemp192;
																																																																																		self.iTempPerm188 = iTemp193;
																																																																																		self.iTempPerm189 = iTemp194;
																																																																																		self.fTempPerm190 = fTemp195;
																																																																																		self.fTempPerm191 = fTemp196;
																																																																																		self.iTempPerm192 = iTemp197;
																																																																																		self.iTempPerm193 = iTemp198;
																																																																																		self.fTempPerm194 = fTemp199;
																																																																																		self.fTempPerm195 = fTemp200;
																																																																																		self.iTempPerm196 = iTemp201;
																																																																																		self.fTempPerm197 = fTemp202;
																																																																																		self.fTempPerm198 = fTemp203;
																																																																																		self.fTempPerm199 = fTemp204;
																																																																																		self.iTempPerm200 = iTemp205;
																																																																																		self.iTempPerm201 = iTemp206;
																																																																																		self.fTempPerm202 = fTemp207;
																																																																																		self.fTempPerm203 = fTemp208;
																																																																																		self.fTempPerm204 = fTemp209;
																																																																																		self.fTempPerm205 = fTemp210;
																																																																																		self.fTempPerm206 = fTemp211;
																																																																																		self.fTempPerm207 = fTemp212;
																																																																																		self.iTempPerm208 = iTemp213;
																																																																																		self.fTempPerm209 = fTemp214;
																																																																																		self.fTempPerm210 = fTemp215;
																																																																																		self.fTempPerm211 = fTemp216;
																																																																																		self.iTempPerm212 = iTemp217;
																																																																																		self.iTempPerm213 = iTemp218;
																																																																																		self.iTempPerm214 = iTemp219;
																																																																																		self.fTempPerm215 = fTemp220;
																																																																																		self.fTempPerm216 = fTemp221;
																																																																																		self.iTempPerm217 = iTemp222;
																																																																																		self.iTempPerm218 = iTemp223;
																																																																																		self.fTempPerm219 = fTemp224;
																																																																																		self.fTempPerm220 = fTemp225;
																																																																																		self.fTempPerm221 = fTemp226;
																																																																																		self.fTempPerm222 = fTemp227;
																																																																																		self.iTempPerm223 = iTemp228;
																																																																																		self.fTempPerm224 = fTemp229;
																																																																																		self.iTempPerm225 = iTemp230;
																																																																																		self.iTempPerm226 = iTemp231;
																																																																																		self.fTempPerm227 = fTemp232;
																																																																																		self.fTempPerm228 = fTemp233;
																																																																																		self.fTempPerm229 = fTemp234;
																																																																																		self.iTempPerm230 = iTemp235;
																																																																																		self.fTempPerm231 = fTemp236;
																																																																																		self.fTempPerm232 = fTemp237;
																																																																																		self.fTempPerm233 = fTemp238;
																																																																																		self.iTempPerm234 = iTemp239;
																																																																																		self.fTempPerm235 = fTemp240;
																																																																																		self.fTempPerm236 = fTemp241;
																																																																																		self.fTempPerm237 = fTemp242;
																																																																																		self.fTempPerm238 = fTemp243;
																																																																																		self.fTempPerm239 = fTemp244;
																																																																																		self.fTempPerm240 = fTemp245;
																																																																																		self.fTempPerm241 = fTemp246;
																																																																																		self.iTempPerm242 = iTemp247;
																																																																																		self.iTempPerm243 = iTemp248;
																																																																																		self.iTempPerm244 = iTemp249;
																																																																																		self.iTempPerm245 = iTemp250;
																																																																																		self.fTempPerm246 = fTemp251;
																																																																																		self.fTempPerm247 = fTemp252;
																																																																																		self.iTempPerm248 = iTemp253;
																																																																																		self.iTempPerm249 = iTemp254;
																																																																																		self.fTempPerm250 = fTemp255;
																																																																																		self.fTempPerm251 = fTemp256;
																																																																																		self.iTempPerm252 = iTemp257;
																																																																																		self.fTempPerm253 = fTemp258;
																																																																																		self.fTempPerm254 = fTemp259;
																																																																																		self.fTempPerm255 = fTemp260;
																																																																																		self.iTempPerm256 = iTemp261;
																																																																																		self.iTempPerm257 = iTemp262;
																																																																																		self.fTempPerm258 = fTemp263;
																																																																																		self.fTempPerm259 = fTemp264;
																																																																																		self.fTempPerm260 = fTemp265;
																																																																																		self.fTempPerm261 = fTemp266;
																																																																																		self.fTempPerm262 = fTemp267;
																																																																																		self.fTempPerm263 = fTemp268;
																																																																																		self.iTempPerm264 = iTemp269;
																																																																																		self.fTempPerm265 = fTemp270;
																																																																																		self.fTempPerm266 = fTemp271;
																																																																																		self.fTempPerm267 = fTemp272;
																																																																																		self.iTempPerm268 = iTemp273;
																																																																																		self.iTempPerm269 = iTemp274;
																																																																																		self.iTempPerm270 = iTemp275;
																																																																																		self.fTempPerm271 = fTemp276;
																																																																																		self.fTempPerm272 = fTemp277;
																																																																																		self.iTempPerm273 = iTemp278;
																																																																																		self.iTempPerm274 = iTemp279;
																																																																																		self.fTempPerm275 = fTemp280;
																																																																																		self.fTempPerm276 = fTemp281;
																																																																																		self.fTempPerm277 = fTemp282;
																																																																																		self.fTempPerm278 = fTemp283;
																																																																																		self.iTempPerm279 = iTemp284;
																																																																																		self.fTempPerm280 = fTemp285;
																																																																																		self.iTempPerm281 = iTemp286;
																																																																																		self.iTempPerm282 = iTemp287;
																																																																																		self.fTempPerm283 = fTemp288;
																																																																																		self.fTempPerm284 = fTemp289;
																																																																																		self.fTempPerm285 = fTemp290;
																																																																																		self.iTempPerm286 = iTemp291;
																																																																																		self.fTempPerm287 = fTemp292;
																																																																																		self.fTempPerm288 = fTemp293;
																																																																																		self.fTempPerm289 = fTemp294;
																																																																																		self.iTempPerm290 = iTemp295;
																																																																																		self.fTempPerm291 = fTemp296;
																																																																																		self.fTempPerm292 = fTemp297;
																																																																																		self.fTempPerm293 = fTemp298;
																																																																																		self.fTempPerm294 = fTemp299;
																																																																																		self.fTempPerm295 = fTemp300;
																																																																																		self.fTempPerm296 = fTemp301;
																																																																																		self.fTempPerm297 = fTemp302;
																																																																																		self.iTempPerm298 = iTemp303;
																																																																																		self.iTempPerm299 = iTemp304;
																																																																																		self.iTempPerm300 = iTemp305;
																																																																																		self.fTempPerm301 = fTemp306;
																																																																																		self.fTempPerm302 = fTemp307;
																																																																																		self.iTempPerm303 = iTemp308;
																																																																																		self.iTempPerm304 = iTemp309;
																																																																																		self.fTempPerm305 = fTemp310;
																																																																																		self.fTempPerm306 = fTemp311;
																																																																																		self.iTempPerm307 = iTemp312;
																																																																																		self.fTempPerm308 = fTemp313;
																																																																																		self.fTempPerm309 = fTemp314;
																																																																																		self.fTempPerm310 = fTemp315;
																																																																																		self.iTempPerm311 = iTemp316;
																																																																																		self.iTempPerm312 = iTemp317;
																																																																																		self.fTempPerm313 = fTemp318;
																																																																																		self.fTempPerm314 = fTemp319;
																																																																																		self.fTempPerm315 = fTemp320;
																																																																																		self.fTempPerm316 = fTemp321;
																																																																																		self.fTempPerm317 = fTemp322;
																																																																																		self.fTempPerm318 = fTemp323;
																																																																																		self.iTempPerm319 = iTemp324;
																																																																																		self.fTempPerm320 = fTemp325;
																																																																																		self.fTempPerm321 = fTemp326;
																																																																																		self.fTempPerm322 = fTemp327;
																																																																																		self.iTempPerm323 = iTemp328;
																																																																																		self.iTempPerm324 = iTemp329;
																																																																																		self.iTempPerm325 = iTemp330;
																																																																																		self.fTempPerm326 = fTemp331;
																																																																																		self.fTempPerm327 = fTemp332;
																																																																																		self.iTempPerm328 = iTemp333;
																																																																																		self.iTempPerm329 = iTemp334;
																																																																																		self.fTempPerm330 = fTemp335;
																																																																																		self.fTempPerm331 = fTemp336;
																																																																																		self.fTempPerm332 = fTemp337;
																																																																																		self.fTempPerm333 = fTemp338;
																																																																																		self.iTempPerm334 = iTemp339;
																																																																																		self.fTempPerm335 = fTemp340;
																																																																																		self.iTempPerm336 = iTemp341;
																																																																																		self.iTempPerm337 = iTemp342;
																																																																																		self.fTempPerm338 = fTemp343;
																																																																																		self.fTempPerm339 = fTemp344;
																																																																																		self.fTempPerm340 = fTemp345;
																																																																																		self.iTempPerm341 = iTemp346;
																																																																																		self.fTempPerm342 = fTemp347;
																																																																																		self.fTempPerm343 = fTemp348;
																																																																																		self.fTempPerm344 = fTemp349;
																																																																																		self.iTempPerm345 = iTemp350;
																																																																																		self.fTempPerm346 = fTemp351;
																																																																																		self.fTempPerm347 = fTemp352;
																																																																																		self.fTempPerm348 = fTemp353;
																																																																																		self.fTempPerm349 = fTemp354;
																																																																																		self.fTempPerm350 = fTemp355;
																																																																																		self.fTempPerm351 = fTemp356;
																																																																																		self.fTempPerm352 = fTemp357;
																																																																																		self.fTempPerm353 = fTemp358;
																																																																																		self.fTempPerm354 = fTemp359;
																																																																																		self.fTempPerm355 = fTemp360;
																																																																																		self.fTempPerm356 = fTemp361;
																																																																																		self.fTempPerm357 = fTemp362;
																																																																																		self.fTempPerm358 = fTemp363;
																																																																																		self.fTempPerm359 = fTemp364;
																																																																																		self.fTempPerm360 = fTemp365;
																																																																																		self.fTempPerm361 = fTemp366;
																																																																																		self.fTempPerm362 = fTemp367;
																																																																																		self.fTempPerm363 = fTemp368;
																																																																																		self.fTempPerm364 = fTemp369;
																																																																																		self.fTempPerm365 = fTemp370;
																																																																																		self.fTempPerm366 = fTemp371;
																																																																																		self.fTempPerm367 = fTemp372;
																																																																																		self.fTempPerm368 = fTemp373;
																																																																																		self.fTempPerm369 = fTemp374;
																																																																																		self.fTempPerm370 = fTemp375;
																																																																																		self.fTempPerm371 = fTemp376;
																																																																																		self.iTempPerm372 = iTemp385;
																																																																																		self.iTempPerm373 = iTemp386;
																																																																																		self.iTempPerm374 = iTemp387;
																																																																																		self.iTempPerm375 = iTemp388;
																																																																																		self.iTempPerm376 = iTemp389;
																																																																																		self.iTempPerm377 = iTemp390;
																																																																																		self.iTempPerm378 = iTemp391;
																																																																																		self.iTempPerm379 = iTemp392;
																																																																																		self.iTempPerm380 = iTemp393;
																																																																																		self.iTempPerm381 = iTemp394;
																																																																																		self.iTempPerm382 = iTemp395;
																																																																																		self.iTempPerm383 = iTemp396;
																																																																																		self.iTempPerm384 = iTemp397;
																																																																																		self.iTempPerm385 = iTemp398;
																																																																																		self.iTempPerm386 = iTemp399;
																																																																																		self.fTempPerm387 = fTemp400;
																																																																																		self.fTempPerm388 = fTemp401;
																																																																																		self.iTempPerm389 = iTemp402;
																																																																																		self.fTempPerm390 = fTemp403;
																																																																																		self.fTempPerm391 = fTemp404;
																																																																																		self.iTempPerm392 = iTemp405;
																																																																																		self.fTempPerm393 = fTemp406;
																																																																																		self.fTempPerm394 = fTemp407;
																																																																																		self.fTempPerm395 = fTemp408;
																																																																																		self.fTempPerm396 = fTemp409;
																																																																																		self.fTempPerm397 = fTemp410;
																																																																																		self.fTempPerm398 = fTemp411;
																																																																																		self.iTempPerm399 = iTemp412;
																																																																																		self.iTempPerm400 = iTemp413;
																																																																																		self.fTempPerm401 = fTemp414;
																																																																																		self.iTempPerm402 = iTemp415;
																																																																																		self.iTempPerm403 = iTemp416;
																																																																																		self.fTempPerm404 = fTemp417;
																																																																																		self.fTempPerm405 = fTemp418;
																																																																																		self.fTempPerm406 = fTemp419;
																																																																																		self.fTempPerm407 = fTemp420;
																																																																																		self.fTempPerm408 = fTemp421;
																																																																																		self.fTempPerm409 = fTemp422;
																																																																																		self.fTempPerm410 = fTemp423;
																																																																																		self.iTempPerm411 = iTemp424;
																																																																																		self.fTempPerm412 = fTemp425;
																																																																																		self.iTempPerm413 = iTemp426;
																																																																																		self.iTempPerm414 = iTemp427;
																																																																																		self.fTempPerm415 = fTemp428;
																																																																																		self.fTempPerm416 = fTemp429;
																																																																																		self.iTempPerm417 = iTemp430;
																																																																																		self.fTempPerm418 = fTemp431;
																																																																																		self.iTempPerm419 = iTemp432;
																																																																																		self.iTempPerm420 = iTemp433;
																																																																																		self.iTempPerm421 = iTemp434;
																																																																																		self.iTempPerm422 = iTemp435;
																																																																																		self.fTempPerm423 = fTemp436;
																																																																																		self.fTempPerm424 = fTemp437;
																																																																																		self.fTempPerm425 = fTemp438;
																																																																																		self.fTempPerm426 = fTemp439;
																																																																																		self.fTempPerm427 = fTemp440;
																																																																																		self.iTempPerm428 = iTemp441;
																																																																																		self.iTempPerm429 = iTemp442;
																																																																																		self.fTempPerm430 = fTemp443;
																																																																																		self.fTempPerm431 = fTemp444;
																																																																																		self.iTempPerm432 = iTemp445;
																																																																																		self.fTempPerm433 = fTemp446;
																																																																																		self.iTempPerm434 = iTemp447;
																																																																																		self.iTempPerm435 = iTemp448;
																																																																																		self.iTempPerm436 = iTemp449;
																																																																																		self.iTempPerm437 = iTemp450;
																																																																																		self.iTempPerm438 = iTemp451;
																																																																																		self.fTempPerm439 = fTemp452;
																																																																																		self.iTempPerm440 = iTemp453;
																																																																																		self.iTempPerm441 = iTemp454;
																																																																																		self.iTempPerm442 = iTemp455;
																																																																																		self.iTempPerm443 = iTemp456;
																																																																																		self.fTempPerm444 = fTemp457;
																																																																																		self.fTempPerm445 = fTemp458;
																																																																																		self.iTempPerm446 = iTemp459;
																																																																																		self.fTempPerm447 = fTemp460;
																																																																																		self.fTempPerm448 = fTemp461;
																																																																																		self.iTempPerm449 = iTemp462;
																																																																																		self.iTempPerm450 = iTemp463;
																																																																																		self.fTempPerm451 = fTemp464;
																																																																																		self.fTempPerm452 = fTemp465;
																																																																																		self.iTempPerm453 = iTemp466;
																																																																																		self.fTempPerm454 = fTemp467;
																																																																																		self.fTempPerm455 = fTemp468;
																																																																																		self.fTempPerm456 = fTemp469;
																																																																																		self.fTempPerm457 = fTemp470;
																																																																																		self.iTempPerm458 = iTemp471;
																																																																																		self.fTempPerm459 = fTemp472;
																																																																																		self.fTempPerm460 = fTemp473;
																																																																																		self.fTempPerm461 = fTemp474;
																																																																																		self.fTempPerm462 = fTemp475;
																																																																																		self.iTempPerm463 = iTemp476;
																																																																																		self.fTempPerm464 = fTemp477;
																																																																																		self.fTempPerm465 = fTemp478;
																																																																																		self.iTempPerm466 = iTemp479;
																																																																																		self.iTempPerm467 = iTemp480;
																																																																																		self.fTempPerm468 = fTemp481;
																																																																																		self.fTempPerm469 = fTemp482;
																																																																																		self.fTempPerm470 = fTemp484;
																																																																																	}

}

