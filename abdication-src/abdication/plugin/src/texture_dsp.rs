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
	fConst2: F32,
	fHslider1: F32,
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
	fHslider4: F32,
	fHslider5: F32,
	fRec10: [F32;2],
	fEntry0: F32,
	fHslider6: F32,
	fHslider7: F32,
	fHslider8: F32,
	fHslider9: F32,
	fHslider10: F32,
	fHslider11: F32,
	fHslider12: F32,
	fHslider13: F32,
	fHslider14: F32,
	fHslider15: F32,
	fHslider16: F32,
	fHslider17: F32,
	fHslider18: F32,
	fRec11: [F32;2],
	fHslider19: F32,
	fRec12: [F32;2],
	fTempPerm19: F32,
	fEntry1: F32,
	fHslider20: F32,
	fRec13: [F32;2],
	fHslider21: F32,
	fRec14: [F32;2],
	fTempPerm20: F32,
	fTempPerm21: F32,
	fTempPerm22: F32,
	fTempPerm23: F32,
	fTempPerm24: F32,
	fTempPerm25: F32,
	fRec8: [F32;2],
	fHslider22: F32,
	fRec15: [F32;2],
	fTempPerm26: F32,
	fTempPerm27: F32,
	fTempPerm28: F32,
	fConst7: F32,
	fTempPerm29: F32,
	iTempPerm30: i32,
	iTempPerm31: i32,
	fTempPerm32: F32,
	fTempPerm33: F32,
	fTempPerm34: F32,
	fTempPerm35: F32,
	fRec7: [F32;2],
	fTempPerm36: F32,
	fTempPerm37: F32,
	iTempPerm38: i32,
	fTempPerm39: F32,
	fRec16: [F32;2],
	fTempPerm40: F32,
	fVec2: [F32;2],
	fTempPerm41: F32,
	iTempPerm42: i32,
	iTempPerm43: i32,
	iTempPerm44: i32,
	fTempPerm45: F32,
	fTempPerm46: F32,
	fRec6: [F32;2],
	fConst8: F32,
	fRec5: [F32;2],
	iTempPerm47: i32,
	iTempPerm48: i32,
	fTempPerm49: F32,
	fRec17: [F32;2],
	fTempPerm50: F32,
	fRec18: [F32;2],
	fTempPerm51: F32,
	fTempPerm52: F32,
	fTempPerm53: F32,
	fTempPerm54: F32,
	fTempPerm55: F32,
	fTempPerm56: F32,
	fTempPerm57: F32,
	iTempPerm58: i32,
	fTempPerm59: F32,
	fHslider23: F32,
	fRec22: [F32;2],
	fTempPerm60: F32,
	fTempPerm61: F32,
	fTempPerm62: F32,
	iTempPerm63: i32,
	iTempPerm64: i32,
	fTempPerm65: F32,
	fRec24: [F32;2],
	fRec23: [F32;2],
	fTempPerm66: F32,
	fTempPerm67: F32,
	iTempPerm68: i32,
	fTempPerm69: F32,
	fTempPerm70: F32,
	fTempPerm71: F32,
	iTempPerm72: i32,
	fTempPerm73: F32,
	fTempPerm74: F32,
	fRec25: [F32;2],
	fTempPerm75: F32,
	fTempPerm76: F32,
	fConst10: F32,
	fConst11: F32,
	fTempPerm77: F32,
	fConst12: F32,
	fTempPerm78: F32,
	fConst13: F32,
	fTempPerm79: F32,
	fTempPerm80: F32,
	iTempPerm81: i32,
	iTempPerm82: i32,
	iTempPerm83: i32,
	iTempPerm84: i32,
	fTempPerm85: F32,
	fTempPerm86: F32,
	iTempPerm87: i32,
	iTempPerm88: i32,
	fTempPerm89: F32,
	fTempPerm90: F32,
	iTempPerm91: i32,
	fTempPerm92: F32,
	fTempPerm93: F32,
	fRec29: [F32;2],
	fTempPerm94: F32,
	iTempPerm95: i32,
	iTempPerm96: i32,
	fTempPerm97: F32,
	fTempPerm98: F32,
	fTempPerm99: F32,
	fTempPerm100: F32,
	fRec28: [F32;2],
	fTempPerm101: F32,
	fTempPerm102: F32,
	iTempPerm103: i32,
	fTempPerm104: F32,
	fRec30: [F32;2],
	fTempPerm105: F32,
	fVec3: [F32;2],
	fTempPerm106: F32,
	iTempPerm107: i32,
	iTempPerm108: i32,
	iTempPerm109: i32,
	fTempPerm110: F32,
	fTempPerm111: F32,
	fRec27: [F32;2],
	fRec26: [F32;2],
	iTempPerm112: i32,
	iTempPerm113: i32,
	fTempPerm114: F32,
	fRec31: [F32;2],
	fTempPerm115: F32,
	fTempPerm116: F32,
	fTempPerm117: F32,
	iTempPerm118: i32,
	fTempPerm119: F32,
	iTempPerm120: i32,
	iTempPerm121: i32,
	fTempPerm122: F32,
	fRec36: [F32;2],
	fRec35: [F32;2],
	fTempPerm123: F32,
	fTempPerm124: F32,
	iTempPerm125: i32,
	fTempPerm126: F32,
	fTempPerm127: F32,
	fTempPerm128: F32,
	iTempPerm129: i32,
	fTempPerm130: F32,
	fTempPerm131: F32,
	fRec37: [F32;2],
	fTempPerm132: F32,
	fTempPerm133: F32,
	fTempPerm134: F32,
	fTempPerm135: F32,
	fTempPerm136: F32,
	iTempPerm137: i32,
	iTempPerm138: i32,
	iTempPerm139: i32,
	iTempPerm140: i32,
	fTempPerm141: F32,
	fTempPerm142: F32,
	iTempPerm143: i32,
	iTempPerm144: i32,
	fTempPerm145: F32,
	fTempPerm146: F32,
	iTempPerm147: i32,
	fTempPerm148: F32,
	fTempPerm149: F32,
	fRec41: [F32;2],
	fTempPerm150: F32,
	iTempPerm151: i32,
	iTempPerm152: i32,
	fTempPerm153: F32,
	fTempPerm154: F32,
	fTempPerm155: F32,
	fTempPerm156: F32,
	fRec40: [F32;2],
	fTempPerm157: F32,
	fTempPerm158: F32,
	iTempPerm159: i32,
	fTempPerm160: F32,
	fRec42: [F32;2],
	fTempPerm161: F32,
	fVec4: [F32;2],
	fTempPerm162: F32,
	iTempPerm163: i32,
	iTempPerm164: i32,
	iTempPerm165: i32,
	fTempPerm166: F32,
	fTempPerm167: F32,
	fRec39: [F32;2],
	fRec38: [F32;2],
	iTempPerm168: i32,
	iTempPerm169: i32,
	fTempPerm170: F32,
	fRec43: [F32;2],
	fTempPerm171: F32,
	fTempPerm172: F32,
	fTempPerm173: F32,
	iTempPerm174: i32,
	fTempPerm175: F32,
	iTempPerm176: i32,
	iTempPerm177: i32,
	fTempPerm178: F32,
	fRec48: [F32;2],
	fRec47: [F32;2],
	fTempPerm179: F32,
	fTempPerm180: F32,
	iTempPerm181: i32,
	fTempPerm182: F32,
	fTempPerm183: F32,
	fTempPerm184: F32,
	iTempPerm185: i32,
	fTempPerm186: F32,
	fTempPerm187: F32,
	fRec49: [F32;2],
	fTempPerm188: F32,
	fTempPerm189: F32,
	fTempPerm190: F32,
	fTempPerm191: F32,
	fTempPerm192: F32,
	iTempPerm193: i32,
	iTempPerm194: i32,
	iTempPerm195: i32,
	iTempPerm196: i32,
	fTempPerm197: F32,
	fTempPerm198: F32,
	iTempPerm199: i32,
	iTempPerm200: i32,
	fTempPerm201: F32,
	fTempPerm202: F32,
	iTempPerm203: i32,
	fTempPerm204: F32,
	fTempPerm205: F32,
	fRec53: [F32;2],
	fTempPerm206: F32,
	iTempPerm207: i32,
	iTempPerm208: i32,
	fTempPerm209: F32,
	fTempPerm210: F32,
	fTempPerm211: F32,
	fTempPerm212: F32,
	fRec52: [F32;2],
	fTempPerm213: F32,
	fTempPerm214: F32,
	iTempPerm215: i32,
	fTempPerm216: F32,
	fRec54: [F32;2],
	fTempPerm217: F32,
	fVec5: [F32;2],
	fTempPerm218: F32,
	iTempPerm219: i32,
	iTempPerm220: i32,
	iTempPerm221: i32,
	fTempPerm222: F32,
	fTempPerm223: F32,
	fRec51: [F32;2],
	fRec50: [F32;2],
	iTempPerm224: i32,
	iTempPerm225: i32,
	fTempPerm226: F32,
	fRec55: [F32;2],
	fTempPerm227: F32,
	fTempPerm228: F32,
	fTempPerm229: F32,
	iTempPerm230: i32,
	fTempPerm231: F32,
	iTempPerm232: i32,
	iTempPerm233: i32,
	fTempPerm234: F32,
	fRec60: [F32;2],
	fRec59: [F32;2],
	fTempPerm235: F32,
	fTempPerm236: F32,
	iTempPerm237: i32,
	fTempPerm238: F32,
	fTempPerm239: F32,
	fTempPerm240: F32,
	iTempPerm241: i32,
	fTempPerm242: F32,
	fTempPerm243: F32,
	fRec61: [F32;2],
	fTempPerm244: F32,
	fTempPerm245: F32,
	fTempPerm246: F32,
	fTempPerm247: F32,
	fTempPerm248: F32,
	iTempPerm249: i32,
	iTempPerm250: i32,
	iTempPerm251: i32,
	iTempPerm252: i32,
	fTempPerm253: F32,
	fTempPerm254: F32,
	iTempPerm255: i32,
	iTempPerm256: i32,
	fTempPerm257: F32,
	fTempPerm258: F32,
	iTempPerm259: i32,
	fTempPerm260: F32,
	fTempPerm261: F32,
	fRec65: [F32;2],
	fTempPerm262: F32,
	iTempPerm263: i32,
	iTempPerm264: i32,
	fTempPerm265: F32,
	fTempPerm266: F32,
	fTempPerm267: F32,
	fTempPerm268: F32,
	fRec64: [F32;2],
	fTempPerm269: F32,
	fTempPerm270: F32,
	iTempPerm271: i32,
	fTempPerm272: F32,
	fRec66: [F32;2],
	fTempPerm273: F32,
	fVec6: [F32;2],
	fTempPerm274: F32,
	iTempPerm275: i32,
	iTempPerm276: i32,
	iTempPerm277: i32,
	fTempPerm278: F32,
	fTempPerm279: F32,
	fRec63: [F32;2],
	fRec62: [F32;2],
	iTempPerm280: i32,
	iTempPerm281: i32,
	fTempPerm282: F32,
	fRec67: [F32;2],
	fTempPerm283: F32,
	fTempPerm284: F32,
	fTempPerm285: F32,
	iTempPerm286: i32,
	fTempPerm287: F32,
	iTempPerm288: i32,
	iTempPerm289: i32,
	fTempPerm290: F32,
	fRec72: [F32;2],
	fRec71: [F32;2],
	fTempPerm291: F32,
	fTempPerm292: F32,
	iTempPerm293: i32,
	fTempPerm294: F32,
	fTempPerm295: F32,
	fTempPerm296: F32,
	iTempPerm297: i32,
	fTempPerm298: F32,
	fTempPerm299: F32,
	fRec73: [F32;2],
	fTempPerm300: F32,
	fTempPerm301: F32,
	fTempPerm302: F32,
	fTempPerm303: F32,
	fTempPerm304: F32,
	iTempPerm305: i32,
	iTempPerm306: i32,
	iTempPerm307: i32,
	fTempPerm308: F32,
	fTempPerm309: F32,
	iTempPerm310: i32,
	iTempPerm311: i32,
	fTempPerm312: F32,
	fTempPerm313: F32,
	iTempPerm314: i32,
	fTempPerm315: F32,
	fTempPerm316: F32,
	fRec77: [F32;2],
	fTempPerm317: F32,
	iTempPerm318: i32,
	iTempPerm319: i32,
	fTempPerm320: F32,
	fTempPerm321: F32,
	fTempPerm322: F32,
	fTempPerm323: F32,
	fRec76: [F32;2],
	fTempPerm324: F32,
	fTempPerm325: F32,
	iTempPerm326: i32,
	fTempPerm327: F32,
	fRec78: [F32;2],
	fTempPerm328: F32,
	fVec7: [F32;2],
	fTempPerm329: F32,
	iTempPerm330: i32,
	iTempPerm331: i32,
	iTempPerm332: i32,
	fTempPerm333: F32,
	fTempPerm334: F32,
	fRec75: [F32;2],
	fRec74: [F32;2],
	iTempPerm335: i32,
	iTempPerm336: i32,
	fTempPerm337: F32,
	fRec79: [F32;2],
	fTempPerm338: F32,
	fTempPerm339: F32,
	fTempPerm340: F32,
	iTempPerm341: i32,
	fTempPerm342: F32,
	iTempPerm343: i32,
	iTempPerm344: i32,
	fTempPerm345: F32,
	fRec84: [F32;2],
	fRec83: [F32;2],
	fTempPerm346: F32,
	fTempPerm347: F32,
	iTempPerm348: i32,
	fTempPerm349: F32,
	fTempPerm350: F32,
	fTempPerm351: F32,
	iTempPerm352: i32,
	fTempPerm353: F32,
	fTempPerm354: F32,
	fRec85: [F32;2],
	fTempPerm355: F32,
	fTempPerm356: F32,
	fTempPerm357: F32,
	fTempPerm358: F32,
	fTempPerm359: F32,
	fTempPerm360: F32,
	fTempPerm361: F32,
	fRec80: [F32;2],
	fConst14: F32,
	fRec81: [F32;2],
	fTempPerm362: F32,
	fTempPerm363: F32,
	fTempPerm364: F32,
	fRec68: [F32;2],
	fRec69: [F32;2],
	fTempPerm365: F32,
	fTempPerm366: F32,
	fTempPerm367: F32,
	fRec56: [F32;2],
	fRec57: [F32;2],
	fTempPerm368: F32,
	fTempPerm369: F32,
	fTempPerm370: F32,
	fRec44: [F32;2],
	fRec45: [F32;2],
	fTempPerm371: F32,
	fTempPerm372: F32,
	fTempPerm373: F32,
	fRec32: [F32;2],
	fRec33: [F32;2],
	fTempPerm374: F32,
	fTempPerm375: F32,
	fTempPerm376: F32,
	fRec19: [F32;2],
	fRec20: [F32;2],
	fTempPerm377: F32,
	fConst15: F32,
	fConst16: F32,
	fRec86: [F32;2],
	fRec87: [F32;2],
	fTempPerm378: F32,
	fRec88: [F32;2],
	fTempPerm379: F32,
	iTempPerm380: i32,
	iRec90: [i32;2],
	iRec96: [i32;2],
	iTempPerm381: i32,
	iTempPerm382: i32,
	iTempPerm383: i32,
	iTempPerm384: i32,
	iTempPerm385: i32,
	iTempPerm386: i32,
	iTempPerm387: i32,
	iTempPerm388: i32,
	iTempPerm389: i32,
	iTempPerm390: i32,
	iTempPerm391: i32,
	iTempPerm392: i32,
	iTempPerm393: i32,
	iTempPerm394: i32,
	fTempPerm395: F32,
	fTempPerm396: F32,
	iTempPerm397: i32,
	fTempPerm398: F32,
	fTempPerm399: F32,
	iTempPerm400: i32,
	fTempPerm401: F32,
	fTempPerm402: F32,
	fTempPerm403: F32,
	fHslider24: F32,
	fRec97: [F32;2],
	fTempPerm404: F32,
	fTempPerm405: F32,
	fTempPerm406: F32,
	fTempPerm407: F32,
	iTempPerm408: i32,
	iTempPerm409: i32,
	fTempPerm410: F32,
	iTempPerm411: i32,
	iTempPerm412: i32,
	fTempPerm413: F32,
	fTempPerm414: F32,
	fTempPerm415: F32,
	fTempPerm416: F32,
	fHslider25: F32,
	fRec98: [F32;2],
	fTempPerm417: F32,
	fTempPerm418: F32,
	fTempPerm419: F32,
	iTempPerm420: i32,
	fTempPerm421: F32,
	iTempPerm422: i32,
	iTempPerm423: i32,
	fTempPerm424: F32,
	fTempPerm425: F32,
	iTempPerm426: i32,
	fTempPerm427: F32,
	fConst17: F32,
	iTempPerm428: i32,
	iRec91: [i32;2],
	iRec92: [i32;2],
	iRec93: [i32;2],
	iRec94: [i32;2],
	iTempPerm429: i32,
	iTempPerm430: i32,
	iTempPerm431: i32,
	fRec89: [F32;2],
	fTempPerm432: F32,
	fTempPerm433: F32,
	fTempPerm434: F32,
	fTempPerm435: F32,
	fTempPerm436: F32,
	iTempPerm437: i32,
	iTempPerm438: i32,
	fTempPerm439: F32,
	fTempPerm440: F32,
	iTempPerm441: i32,
	fTempPerm442: F32,
	iTempPerm443: i32,
	iTempPerm444: i32,
	iTempPerm445: i32,
	iTempPerm446: i32,
	iTempPerm447: i32,
	fRec100: [F32;2],
	fTempPerm448: F32,
	iTempPerm449: i32,
	iTempPerm450: i32,
	iTempPerm451: i32,
	iTempPerm452: i32,
	fTempPerm453: F32,
	fTempPerm454: F32,
	iTempPerm455: i32,
	fTempPerm456: F32,
	fTempPerm457: F32,
	iTempPerm458: i32,
	iTempPerm459: i32,
	fTempPerm460: F32,
	fTempPerm461: F32,
	iTempPerm462: i32,
	fTempPerm463: F32,
	fTempPerm464: F32,
	fTempPerm465: F32,
	fTempPerm466: F32,
	iTempPerm467: i32,
	fTempPerm468: F32,
	fTempPerm469: F32,
	fRec101: [F32;2],
	fRec102: [F32;2],
	fTempPerm470: F32,
	fTempPerm471: F32,
	iTempPerm472: i32,
	fTempPerm473: F32,
	fTempPerm474: F32,
	fRec99: [F32;2],
	iTempPerm475: i32,
	iTempPerm476: i32,
	fRec103: [F32;2],
	fTempPerm477: F32,
	fTempPerm478: F32,
	fTempPerm479: F32,
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
			fConst2: 0.0,
			fHslider1: 0.0,
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
			fHslider4: 0.0,
			fHslider5: 0.0,
			fRec10: [0.0;2],
			fEntry0: 0.0,
			fHslider6: 0.0,
			fHslider7: 0.0,
			fHslider8: 0.0,
			fHslider9: 0.0,
			fHslider10: 0.0,
			fHslider11: 0.0,
			fHslider12: 0.0,
			fHslider13: 0.0,
			fHslider14: 0.0,
			fHslider15: 0.0,
			fHslider16: 0.0,
			fHslider17: 0.0,
			fHslider18: 0.0,
			fRec11: [0.0;2],
			fHslider19: 0.0,
			fRec12: [0.0;2],
			fTempPerm19: 0.0,
			fEntry1: 0.0,
			fHslider20: 0.0,
			fRec13: [0.0;2],
			fHslider21: 0.0,
			fRec14: [0.0;2],
			fTempPerm20: 0.0,
			fTempPerm21: 0.0,
			fTempPerm22: 0.0,
			fTempPerm23: 0.0,
			fTempPerm24: 0.0,
			fTempPerm25: 0.0,
			fRec8: [0.0;2],
			fHslider22: 0.0,
			fRec15: [0.0;2],
			fTempPerm26: 0.0,
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fConst7: 0.0,
			fTempPerm29: 0.0,
			iTempPerm30: 0,
			iTempPerm31: 0,
			fTempPerm32: 0.0,
			fTempPerm33: 0.0,
			fTempPerm34: 0.0,
			fTempPerm35: 0.0,
			fRec7: [0.0;2],
			fTempPerm36: 0.0,
			fTempPerm37: 0.0,
			iTempPerm38: 0,
			fTempPerm39: 0.0,
			fRec16: [0.0;2],
			fTempPerm40: 0.0,
			fVec2: [0.0;2],
			fTempPerm41: 0.0,
			iTempPerm42: 0,
			iTempPerm43: 0,
			iTempPerm44: 0,
			fTempPerm45: 0.0,
			fTempPerm46: 0.0,
			fRec6: [0.0;2],
			fConst8: 0.0,
			fRec5: [0.0;2],
			iTempPerm47: 0,
			iTempPerm48: 0,
			fTempPerm49: 0.0,
			fRec17: [0.0;2],
			fTempPerm50: 0.0,
			fRec18: [0.0;2],
			fTempPerm51: 0.0,
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fTempPerm54: 0.0,
			fTempPerm55: 0.0,
			fTempPerm56: 0.0,
			fTempPerm57: 0.0,
			iTempPerm58: 0,
			fTempPerm59: 0.0,
			fHslider23: 0.0,
			fRec22: [0.0;2],
			fTempPerm60: 0.0,
			fTempPerm61: 0.0,
			fTempPerm62: 0.0,
			iTempPerm63: 0,
			iTempPerm64: 0,
			fTempPerm65: 0.0,
			fRec24: [0.0;2],
			fRec23: [0.0;2],
			fTempPerm66: 0.0,
			fTempPerm67: 0.0,
			iTempPerm68: 0,
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			fTempPerm71: 0.0,
			iTempPerm72: 0,
			fTempPerm73: 0.0,
			fTempPerm74: 0.0,
			fRec25: [0.0;2],
			fTempPerm75: 0.0,
			fTempPerm76: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fTempPerm77: 0.0,
			fConst12: 0.0,
			fTempPerm78: 0.0,
			fConst13: 0.0,
			fTempPerm79: 0.0,
			fTempPerm80: 0.0,
			iTempPerm81: 0,
			iTempPerm82: 0,
			iTempPerm83: 0,
			iTempPerm84: 0,
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			iTempPerm87: 0,
			iTempPerm88: 0,
			fTempPerm89: 0.0,
			fTempPerm90: 0.0,
			iTempPerm91: 0,
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			fRec29: [0.0;2],
			fTempPerm94: 0.0,
			iTempPerm95: 0,
			iTempPerm96: 0,
			fTempPerm97: 0.0,
			fTempPerm98: 0.0,
			fTempPerm99: 0.0,
			fTempPerm100: 0.0,
			fRec28: [0.0;2],
			fTempPerm101: 0.0,
			fTempPerm102: 0.0,
			iTempPerm103: 0,
			fTempPerm104: 0.0,
			fRec30: [0.0;2],
			fTempPerm105: 0.0,
			fVec3: [0.0;2],
			fTempPerm106: 0.0,
			iTempPerm107: 0,
			iTempPerm108: 0,
			iTempPerm109: 0,
			fTempPerm110: 0.0,
			fTempPerm111: 0.0,
			fRec27: [0.0;2],
			fRec26: [0.0;2],
			iTempPerm112: 0,
			iTempPerm113: 0,
			fTempPerm114: 0.0,
			fRec31: [0.0;2],
			fTempPerm115: 0.0,
			fTempPerm116: 0.0,
			fTempPerm117: 0.0,
			iTempPerm118: 0,
			fTempPerm119: 0.0,
			iTempPerm120: 0,
			iTempPerm121: 0,
			fTempPerm122: 0.0,
			fRec36: [0.0;2],
			fRec35: [0.0;2],
			fTempPerm123: 0.0,
			fTempPerm124: 0.0,
			iTempPerm125: 0,
			fTempPerm126: 0.0,
			fTempPerm127: 0.0,
			fTempPerm128: 0.0,
			iTempPerm129: 0,
			fTempPerm130: 0.0,
			fTempPerm131: 0.0,
			fRec37: [0.0;2],
			fTempPerm132: 0.0,
			fTempPerm133: 0.0,
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			fTempPerm136: 0.0,
			iTempPerm137: 0,
			iTempPerm138: 0,
			iTempPerm139: 0,
			iTempPerm140: 0,
			fTempPerm141: 0.0,
			fTempPerm142: 0.0,
			iTempPerm143: 0,
			iTempPerm144: 0,
			fTempPerm145: 0.0,
			fTempPerm146: 0.0,
			iTempPerm147: 0,
			fTempPerm148: 0.0,
			fTempPerm149: 0.0,
			fRec41: [0.0;2],
			fTempPerm150: 0.0,
			iTempPerm151: 0,
			iTempPerm152: 0,
			fTempPerm153: 0.0,
			fTempPerm154: 0.0,
			fTempPerm155: 0.0,
			fTempPerm156: 0.0,
			fRec40: [0.0;2],
			fTempPerm157: 0.0,
			fTempPerm158: 0.0,
			iTempPerm159: 0,
			fTempPerm160: 0.0,
			fRec42: [0.0;2],
			fTempPerm161: 0.0,
			fVec4: [0.0;2],
			fTempPerm162: 0.0,
			iTempPerm163: 0,
			iTempPerm164: 0,
			iTempPerm165: 0,
			fTempPerm166: 0.0,
			fTempPerm167: 0.0,
			fRec39: [0.0;2],
			fRec38: [0.0;2],
			iTempPerm168: 0,
			iTempPerm169: 0,
			fTempPerm170: 0.0,
			fRec43: [0.0;2],
			fTempPerm171: 0.0,
			fTempPerm172: 0.0,
			fTempPerm173: 0.0,
			iTempPerm174: 0,
			fTempPerm175: 0.0,
			iTempPerm176: 0,
			iTempPerm177: 0,
			fTempPerm178: 0.0,
			fRec48: [0.0;2],
			fRec47: [0.0;2],
			fTempPerm179: 0.0,
			fTempPerm180: 0.0,
			iTempPerm181: 0,
			fTempPerm182: 0.0,
			fTempPerm183: 0.0,
			fTempPerm184: 0.0,
			iTempPerm185: 0,
			fTempPerm186: 0.0,
			fTempPerm187: 0.0,
			fRec49: [0.0;2],
			fTempPerm188: 0.0,
			fTempPerm189: 0.0,
			fTempPerm190: 0.0,
			fTempPerm191: 0.0,
			fTempPerm192: 0.0,
			iTempPerm193: 0,
			iTempPerm194: 0,
			iTempPerm195: 0,
			iTempPerm196: 0,
			fTempPerm197: 0.0,
			fTempPerm198: 0.0,
			iTempPerm199: 0,
			iTempPerm200: 0,
			fTempPerm201: 0.0,
			fTempPerm202: 0.0,
			iTempPerm203: 0,
			fTempPerm204: 0.0,
			fTempPerm205: 0.0,
			fRec53: [0.0;2],
			fTempPerm206: 0.0,
			iTempPerm207: 0,
			iTempPerm208: 0,
			fTempPerm209: 0.0,
			fTempPerm210: 0.0,
			fTempPerm211: 0.0,
			fTempPerm212: 0.0,
			fRec52: [0.0;2],
			fTempPerm213: 0.0,
			fTempPerm214: 0.0,
			iTempPerm215: 0,
			fTempPerm216: 0.0,
			fRec54: [0.0;2],
			fTempPerm217: 0.0,
			fVec5: [0.0;2],
			fTempPerm218: 0.0,
			iTempPerm219: 0,
			iTempPerm220: 0,
			iTempPerm221: 0,
			fTempPerm222: 0.0,
			fTempPerm223: 0.0,
			fRec51: [0.0;2],
			fRec50: [0.0;2],
			iTempPerm224: 0,
			iTempPerm225: 0,
			fTempPerm226: 0.0,
			fRec55: [0.0;2],
			fTempPerm227: 0.0,
			fTempPerm228: 0.0,
			fTempPerm229: 0.0,
			iTempPerm230: 0,
			fTempPerm231: 0.0,
			iTempPerm232: 0,
			iTempPerm233: 0,
			fTempPerm234: 0.0,
			fRec60: [0.0;2],
			fRec59: [0.0;2],
			fTempPerm235: 0.0,
			fTempPerm236: 0.0,
			iTempPerm237: 0,
			fTempPerm238: 0.0,
			fTempPerm239: 0.0,
			fTempPerm240: 0.0,
			iTempPerm241: 0,
			fTempPerm242: 0.0,
			fTempPerm243: 0.0,
			fRec61: [0.0;2],
			fTempPerm244: 0.0,
			fTempPerm245: 0.0,
			fTempPerm246: 0.0,
			fTempPerm247: 0.0,
			fTempPerm248: 0.0,
			iTempPerm249: 0,
			iTempPerm250: 0,
			iTempPerm251: 0,
			iTempPerm252: 0,
			fTempPerm253: 0.0,
			fTempPerm254: 0.0,
			iTempPerm255: 0,
			iTempPerm256: 0,
			fTempPerm257: 0.0,
			fTempPerm258: 0.0,
			iTempPerm259: 0,
			fTempPerm260: 0.0,
			fTempPerm261: 0.0,
			fRec65: [0.0;2],
			fTempPerm262: 0.0,
			iTempPerm263: 0,
			iTempPerm264: 0,
			fTempPerm265: 0.0,
			fTempPerm266: 0.0,
			fTempPerm267: 0.0,
			fTempPerm268: 0.0,
			fRec64: [0.0;2],
			fTempPerm269: 0.0,
			fTempPerm270: 0.0,
			iTempPerm271: 0,
			fTempPerm272: 0.0,
			fRec66: [0.0;2],
			fTempPerm273: 0.0,
			fVec6: [0.0;2],
			fTempPerm274: 0.0,
			iTempPerm275: 0,
			iTempPerm276: 0,
			iTempPerm277: 0,
			fTempPerm278: 0.0,
			fTempPerm279: 0.0,
			fRec63: [0.0;2],
			fRec62: [0.0;2],
			iTempPerm280: 0,
			iTempPerm281: 0,
			fTempPerm282: 0.0,
			fRec67: [0.0;2],
			fTempPerm283: 0.0,
			fTempPerm284: 0.0,
			fTempPerm285: 0.0,
			iTempPerm286: 0,
			fTempPerm287: 0.0,
			iTempPerm288: 0,
			iTempPerm289: 0,
			fTempPerm290: 0.0,
			fRec72: [0.0;2],
			fRec71: [0.0;2],
			fTempPerm291: 0.0,
			fTempPerm292: 0.0,
			iTempPerm293: 0,
			fTempPerm294: 0.0,
			fTempPerm295: 0.0,
			fTempPerm296: 0.0,
			iTempPerm297: 0,
			fTempPerm298: 0.0,
			fTempPerm299: 0.0,
			fRec73: [0.0;2],
			fTempPerm300: 0.0,
			fTempPerm301: 0.0,
			fTempPerm302: 0.0,
			fTempPerm303: 0.0,
			fTempPerm304: 0.0,
			iTempPerm305: 0,
			iTempPerm306: 0,
			iTempPerm307: 0,
			fTempPerm308: 0.0,
			fTempPerm309: 0.0,
			iTempPerm310: 0,
			iTempPerm311: 0,
			fTempPerm312: 0.0,
			fTempPerm313: 0.0,
			iTempPerm314: 0,
			fTempPerm315: 0.0,
			fTempPerm316: 0.0,
			fRec77: [0.0;2],
			fTempPerm317: 0.0,
			iTempPerm318: 0,
			iTempPerm319: 0,
			fTempPerm320: 0.0,
			fTempPerm321: 0.0,
			fTempPerm322: 0.0,
			fTempPerm323: 0.0,
			fRec76: [0.0;2],
			fTempPerm324: 0.0,
			fTempPerm325: 0.0,
			iTempPerm326: 0,
			fTempPerm327: 0.0,
			fRec78: [0.0;2],
			fTempPerm328: 0.0,
			fVec7: [0.0;2],
			fTempPerm329: 0.0,
			iTempPerm330: 0,
			iTempPerm331: 0,
			iTempPerm332: 0,
			fTempPerm333: 0.0,
			fTempPerm334: 0.0,
			fRec75: [0.0;2],
			fRec74: [0.0;2],
			iTempPerm335: 0,
			iTempPerm336: 0,
			fTempPerm337: 0.0,
			fRec79: [0.0;2],
			fTempPerm338: 0.0,
			fTempPerm339: 0.0,
			fTempPerm340: 0.0,
			iTempPerm341: 0,
			fTempPerm342: 0.0,
			iTempPerm343: 0,
			iTempPerm344: 0,
			fTempPerm345: 0.0,
			fRec84: [0.0;2],
			fRec83: [0.0;2],
			fTempPerm346: 0.0,
			fTempPerm347: 0.0,
			iTempPerm348: 0,
			fTempPerm349: 0.0,
			fTempPerm350: 0.0,
			fTempPerm351: 0.0,
			iTempPerm352: 0,
			fTempPerm353: 0.0,
			fTempPerm354: 0.0,
			fRec85: [0.0;2],
			fTempPerm355: 0.0,
			fTempPerm356: 0.0,
			fTempPerm357: 0.0,
			fTempPerm358: 0.0,
			fTempPerm359: 0.0,
			fTempPerm360: 0.0,
			fTempPerm361: 0.0,
			fRec80: [0.0;2],
			fConst14: 0.0,
			fRec81: [0.0;2],
			fTempPerm362: 0.0,
			fTempPerm363: 0.0,
			fTempPerm364: 0.0,
			fRec68: [0.0;2],
			fRec69: [0.0;2],
			fTempPerm365: 0.0,
			fTempPerm366: 0.0,
			fTempPerm367: 0.0,
			fRec56: [0.0;2],
			fRec57: [0.0;2],
			fTempPerm368: 0.0,
			fTempPerm369: 0.0,
			fTempPerm370: 0.0,
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			fTempPerm371: 0.0,
			fTempPerm372: 0.0,
			fTempPerm373: 0.0,
			fRec32: [0.0;2],
			fRec33: [0.0;2],
			fTempPerm374: 0.0,
			fTempPerm375: 0.0,
			fTempPerm376: 0.0,
			fRec19: [0.0;2],
			fRec20: [0.0;2],
			fTempPerm377: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fRec86: [0.0;2],
			fRec87: [0.0;2],
			fTempPerm378: 0.0,
			fRec88: [0.0;2],
			fTempPerm379: 0.0,
			iTempPerm380: 0,
			iRec90: [0;2],
			iRec96: [0;2],
			iTempPerm381: 0,
			iTempPerm382: 0,
			iTempPerm383: 0,
			iTempPerm384: 0,
			iTempPerm385: 0,
			iTempPerm386: 0,
			iTempPerm387: 0,
			iTempPerm388: 0,
			iTempPerm389: 0,
			iTempPerm390: 0,
			iTempPerm391: 0,
			iTempPerm392: 0,
			iTempPerm393: 0,
			iTempPerm394: 0,
			fTempPerm395: 0.0,
			fTempPerm396: 0.0,
			iTempPerm397: 0,
			fTempPerm398: 0.0,
			fTempPerm399: 0.0,
			iTempPerm400: 0,
			fTempPerm401: 0.0,
			fTempPerm402: 0.0,
			fTempPerm403: 0.0,
			fHslider24: 0.0,
			fRec97: [0.0;2],
			fTempPerm404: 0.0,
			fTempPerm405: 0.0,
			fTempPerm406: 0.0,
			fTempPerm407: 0.0,
			iTempPerm408: 0,
			iTempPerm409: 0,
			fTempPerm410: 0.0,
			iTempPerm411: 0,
			iTempPerm412: 0,
			fTempPerm413: 0.0,
			fTempPerm414: 0.0,
			fTempPerm415: 0.0,
			fTempPerm416: 0.0,
			fHslider25: 0.0,
			fRec98: [0.0;2],
			fTempPerm417: 0.0,
			fTempPerm418: 0.0,
			fTempPerm419: 0.0,
			iTempPerm420: 0,
			fTempPerm421: 0.0,
			iTempPerm422: 0,
			iTempPerm423: 0,
			fTempPerm424: 0.0,
			fTempPerm425: 0.0,
			iTempPerm426: 0,
			fTempPerm427: 0.0,
			fConst17: 0.0,
			iTempPerm428: 0,
			iRec91: [0;2],
			iRec92: [0;2],
			iRec93: [0;2],
			iRec94: [0;2],
			iTempPerm429: 0,
			iTempPerm430: 0,
			iTempPerm431: 0,
			fRec89: [0.0;2],
			fTempPerm432: 0.0,
			fTempPerm433: 0.0,
			fTempPerm434: 0.0,
			fTempPerm435: 0.0,
			fTempPerm436: 0.0,
			iTempPerm437: 0,
			iTempPerm438: 0,
			fTempPerm439: 0.0,
			fTempPerm440: 0.0,
			iTempPerm441: 0,
			fTempPerm442: 0.0,
			iTempPerm443: 0,
			iTempPerm444: 0,
			iTempPerm445: 0,
			iTempPerm446: 0,
			iTempPerm447: 0,
			fRec100: [0.0;2],
			fTempPerm448: 0.0,
			iTempPerm449: 0,
			iTempPerm450: 0,
			iTempPerm451: 0,
			iTempPerm452: 0,
			fTempPerm453: 0.0,
			fTempPerm454: 0.0,
			iTempPerm455: 0,
			fTempPerm456: 0.0,
			fTempPerm457: 0.0,
			iTempPerm458: 0,
			iTempPerm459: 0,
			fTempPerm460: 0.0,
			fTempPerm461: 0.0,
			iTempPerm462: 0,
			fTempPerm463: 0.0,
			fTempPerm464: 0.0,
			fTempPerm465: 0.0,
			fTempPerm466: 0.0,
			iTempPerm467: 0,
			fTempPerm468: 0.0,
			fTempPerm469: 0.0,
			fRec101: [0.0;2],
			fRec102: [0.0;2],
			fTempPerm470: 0.0,
			fTempPerm471: 0.0,
			iTempPerm472: 0,
			fTempPerm473: 0.0,
			fTempPerm474: 0.0,
			fRec99: [0.0;2],
			iTempPerm475: 0,
			iTempPerm476: 0,
			fRec103: [0.0;2],
			fTempPerm477: 0.0,
			fTempPerm478: 0.0,
			fTempPerm479: 0.0,
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
		return 5;
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
		self.fHslider3 = 0.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 2.0;
		self.fEntry0 = 1.0;
		self.fHslider6 = 0.0;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 0.0;
		self.fHslider15 = 0.0;
		self.fHslider16 = 0.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 5e+01;
		self.fHslider19 = 0.0;
		self.fEntry1 = 21.0;
		self.fHslider20 = 5e+01;
		self.fHslider21 = 0.0;
		self.fHslider22 = 8e+01;
		self.fHslider23 = 1.0;
		self.fHslider24 = 1.0;
		self.fHslider25 = 0.45;
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
		self.iTempPerm0 = 0;
		self.iTempPerm1 = 0;
		for l3 in 0..2 {
			self.fRec3[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.iRec4[l4 as usize] = 0;
		}
		self.iTempPerm2 = 0;
		for l5 in 0..2 {
			self.fRec2[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec1[l6 as usize] = 0.0;
		}
		self.fTempPerm3 = 0.0;
		self.iTempPerm4 = 0;
		self.iTempPerm5 = 0;
		for l7 in 0..2 {
			self.iRec9[l7 as usize] = 0;
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
		for l8 in 0..2 {
			self.fRec10[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec11[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec12[l10 as usize] = 0.0;
		}
		self.fTempPerm19 = 0.0;
		for l11 in 0..2 {
			self.fRec13[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec14[l12 as usize] = 0.0;
		}
		self.fTempPerm20 = 0.0;
		self.fTempPerm21 = 0.0;
		self.fTempPerm22 = 0.0;
		self.fTempPerm23 = 0.0;
		self.fTempPerm24 = 0.0;
		self.fTempPerm25 = 0.0;
		for l13 in 0..2 {
			self.fRec8[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec15[l14 as usize] = 0.0;
		}
		self.fTempPerm26 = 0.0;
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		self.fTempPerm29 = 0.0;
		self.iTempPerm30 = 0;
		self.iTempPerm31 = 0;
		self.fTempPerm32 = 0.0;
		self.fTempPerm33 = 0.0;
		self.fTempPerm34 = 0.0;
		self.fTempPerm35 = 0.0;
		for l15 in 0..2 {
			self.fRec7[l15 as usize] = 0.0;
		}
		self.fTempPerm36 = 0.0;
		self.fTempPerm37 = 0.0;
		self.iTempPerm38 = 0;
		self.fTempPerm39 = 0.0;
		for l16 in 0..2 {
			self.fRec16[l16 as usize] = 0.0;
		}
		self.fTempPerm40 = 0.0;
		for l17 in 0..2 {
			self.fVec2[l17 as usize] = 0.0;
		}
		self.fTempPerm41 = 0.0;
		self.iTempPerm42 = 0;
		self.iTempPerm43 = 0;
		self.iTempPerm44 = 0;
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		for l18 in 0..2 {
			self.fRec6[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec5[l19 as usize] = 0.0;
		}
		self.iTempPerm47 = 0;
		self.iTempPerm48 = 0;
		self.fTempPerm49 = 0.0;
		for l20 in 0..2 {
			self.fRec17[l20 as usize] = 0.0;
		}
		self.fTempPerm50 = 0.0;
		for l21 in 0..2 {
			self.fRec18[l21 as usize] = 0.0;
		}
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		self.fTempPerm54 = 0.0;
		self.fTempPerm55 = 0.0;
		self.fTempPerm56 = 0.0;
		self.fTempPerm57 = 0.0;
		self.iTempPerm58 = 0;
		self.fTempPerm59 = 0.0;
		for l22 in 0..2 {
			self.fRec22[l22 as usize] = 0.0;
		}
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		self.fTempPerm62 = 0.0;
		self.iTempPerm63 = 0;
		self.iTempPerm64 = 0;
		self.fTempPerm65 = 0.0;
		for l23 in 0..2 {
			self.fRec24[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec23[l24 as usize] = 0.0;
		}
		self.fTempPerm66 = 0.0;
		self.fTempPerm67 = 0.0;
		self.iTempPerm68 = 0;
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		self.fTempPerm71 = 0.0;
		self.iTempPerm72 = 0;
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		for l25 in 0..2 {
			self.fRec25[l25 as usize] = 0.0;
		}
		self.fTempPerm75 = 0.0;
		self.fTempPerm76 = 0.0;
		self.fTempPerm77 = 0.0;
		self.fTempPerm78 = 0.0;
		self.fTempPerm79 = 0.0;
		self.fTempPerm80 = 0.0;
		self.iTempPerm81 = 0;
		self.iTempPerm82 = 0;
		self.iTempPerm83 = 0;
		self.iTempPerm84 = 0;
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		self.iTempPerm87 = 0;
		self.iTempPerm88 = 0;
		self.fTempPerm89 = 0.0;
		self.fTempPerm90 = 0.0;
		self.iTempPerm91 = 0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		for l26 in 0..2 {
			self.fRec29[l26 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
		self.iTempPerm95 = 0;
		self.iTempPerm96 = 0;
		self.fTempPerm97 = 0.0;
		self.fTempPerm98 = 0.0;
		self.fTempPerm99 = 0.0;
		self.fTempPerm100 = 0.0;
		for l27 in 0..2 {
			self.fRec28[l27 as usize] = 0.0;
		}
		self.fTempPerm101 = 0.0;
		self.fTempPerm102 = 0.0;
		self.iTempPerm103 = 0;
		self.fTempPerm104 = 0.0;
		for l28 in 0..2 {
			self.fRec30[l28 as usize] = 0.0;
		}
		self.fTempPerm105 = 0.0;
		for l29 in 0..2 {
			self.fVec3[l29 as usize] = 0.0;
		}
		self.fTempPerm106 = 0.0;
		self.iTempPerm107 = 0;
		self.iTempPerm108 = 0;
		self.iTempPerm109 = 0;
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		for l30 in 0..2 {
			self.fRec27[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec26[l31 as usize] = 0.0;
		}
		self.iTempPerm112 = 0;
		self.iTempPerm113 = 0;
		self.fTempPerm114 = 0.0;
		for l32 in 0..2 {
			self.fRec31[l32 as usize] = 0.0;
		}
		self.fTempPerm115 = 0.0;
		self.fTempPerm116 = 0.0;
		self.fTempPerm117 = 0.0;
		self.iTempPerm118 = 0;
		self.fTempPerm119 = 0.0;
		self.iTempPerm120 = 0;
		self.iTempPerm121 = 0;
		self.fTempPerm122 = 0.0;
		for l33 in 0..2 {
			self.fRec36[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec35[l34 as usize] = 0.0;
		}
		self.fTempPerm123 = 0.0;
		self.fTempPerm124 = 0.0;
		self.iTempPerm125 = 0;
		self.fTempPerm126 = 0.0;
		self.fTempPerm127 = 0.0;
		self.fTempPerm128 = 0.0;
		self.iTempPerm129 = 0;
		self.fTempPerm130 = 0.0;
		self.fTempPerm131 = 0.0;
		for l35 in 0..2 {
			self.fRec37[l35 as usize] = 0.0;
		}
		self.fTempPerm132 = 0.0;
		self.fTempPerm133 = 0.0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		self.fTempPerm136 = 0.0;
		self.iTempPerm137 = 0;
		self.iTempPerm138 = 0;
		self.iTempPerm139 = 0;
		self.iTempPerm140 = 0;
		self.fTempPerm141 = 0.0;
		self.fTempPerm142 = 0.0;
		self.iTempPerm143 = 0;
		self.iTempPerm144 = 0;
		self.fTempPerm145 = 0.0;
		self.fTempPerm146 = 0.0;
		self.iTempPerm147 = 0;
		self.fTempPerm148 = 0.0;
		self.fTempPerm149 = 0.0;
		for l36 in 0..2 {
			self.fRec41[l36 as usize] = 0.0;
		}
		self.fTempPerm150 = 0.0;
		self.iTempPerm151 = 0;
		self.iTempPerm152 = 0;
		self.fTempPerm153 = 0.0;
		self.fTempPerm154 = 0.0;
		self.fTempPerm155 = 0.0;
		self.fTempPerm156 = 0.0;
		for l37 in 0..2 {
			self.fRec40[l37 as usize] = 0.0;
		}
		self.fTempPerm157 = 0.0;
		self.fTempPerm158 = 0.0;
		self.iTempPerm159 = 0;
		self.fTempPerm160 = 0.0;
		for l38 in 0..2 {
			self.fRec42[l38 as usize] = 0.0;
		}
		self.fTempPerm161 = 0.0;
		for l39 in 0..2 {
			self.fVec4[l39 as usize] = 0.0;
		}
		self.fTempPerm162 = 0.0;
		self.iTempPerm163 = 0;
		self.iTempPerm164 = 0;
		self.iTempPerm165 = 0;
		self.fTempPerm166 = 0.0;
		self.fTempPerm167 = 0.0;
		for l40 in 0..2 {
			self.fRec39[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec38[l41 as usize] = 0.0;
		}
		self.iTempPerm168 = 0;
		self.iTempPerm169 = 0;
		self.fTempPerm170 = 0.0;
		for l42 in 0..2 {
			self.fRec43[l42 as usize] = 0.0;
		}
		self.fTempPerm171 = 0.0;
		self.fTempPerm172 = 0.0;
		self.fTempPerm173 = 0.0;
		self.iTempPerm174 = 0;
		self.fTempPerm175 = 0.0;
		self.iTempPerm176 = 0;
		self.iTempPerm177 = 0;
		self.fTempPerm178 = 0.0;
		for l43 in 0..2 {
			self.fRec48[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec47[l44 as usize] = 0.0;
		}
		self.fTempPerm179 = 0.0;
		self.fTempPerm180 = 0.0;
		self.iTempPerm181 = 0;
		self.fTempPerm182 = 0.0;
		self.fTempPerm183 = 0.0;
		self.fTempPerm184 = 0.0;
		self.iTempPerm185 = 0;
		self.fTempPerm186 = 0.0;
		self.fTempPerm187 = 0.0;
		for l45 in 0..2 {
			self.fRec49[l45 as usize] = 0.0;
		}
		self.fTempPerm188 = 0.0;
		self.fTempPerm189 = 0.0;
		self.fTempPerm190 = 0.0;
		self.fTempPerm191 = 0.0;
		self.fTempPerm192 = 0.0;
		self.iTempPerm193 = 0;
		self.iTempPerm194 = 0;
		self.iTempPerm195 = 0;
		self.iTempPerm196 = 0;
		self.fTempPerm197 = 0.0;
		self.fTempPerm198 = 0.0;
		self.iTempPerm199 = 0;
		self.iTempPerm200 = 0;
		self.fTempPerm201 = 0.0;
		self.fTempPerm202 = 0.0;
		self.iTempPerm203 = 0;
		self.fTempPerm204 = 0.0;
		self.fTempPerm205 = 0.0;
		for l46 in 0..2 {
			self.fRec53[l46 as usize] = 0.0;
		}
		self.fTempPerm206 = 0.0;
		self.iTempPerm207 = 0;
		self.iTempPerm208 = 0;
		self.fTempPerm209 = 0.0;
		self.fTempPerm210 = 0.0;
		self.fTempPerm211 = 0.0;
		self.fTempPerm212 = 0.0;
		for l47 in 0..2 {
			self.fRec52[l47 as usize] = 0.0;
		}
		self.fTempPerm213 = 0.0;
		self.fTempPerm214 = 0.0;
		self.iTempPerm215 = 0;
		self.fTempPerm216 = 0.0;
		for l48 in 0..2 {
			self.fRec54[l48 as usize] = 0.0;
		}
		self.fTempPerm217 = 0.0;
		for l49 in 0..2 {
			self.fVec5[l49 as usize] = 0.0;
		}
		self.fTempPerm218 = 0.0;
		self.iTempPerm219 = 0;
		self.iTempPerm220 = 0;
		self.iTempPerm221 = 0;
		self.fTempPerm222 = 0.0;
		self.fTempPerm223 = 0.0;
		for l50 in 0..2 {
			self.fRec51[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec50[l51 as usize] = 0.0;
		}
		self.iTempPerm224 = 0;
		self.iTempPerm225 = 0;
		self.fTempPerm226 = 0.0;
		for l52 in 0..2 {
			self.fRec55[l52 as usize] = 0.0;
		}
		self.fTempPerm227 = 0.0;
		self.fTempPerm228 = 0.0;
		self.fTempPerm229 = 0.0;
		self.iTempPerm230 = 0;
		self.fTempPerm231 = 0.0;
		self.iTempPerm232 = 0;
		self.iTempPerm233 = 0;
		self.fTempPerm234 = 0.0;
		for l53 in 0..2 {
			self.fRec60[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec59[l54 as usize] = 0.0;
		}
		self.fTempPerm235 = 0.0;
		self.fTempPerm236 = 0.0;
		self.iTempPerm237 = 0;
		self.fTempPerm238 = 0.0;
		self.fTempPerm239 = 0.0;
		self.fTempPerm240 = 0.0;
		self.iTempPerm241 = 0;
		self.fTempPerm242 = 0.0;
		self.fTempPerm243 = 0.0;
		for l55 in 0..2 {
			self.fRec61[l55 as usize] = 0.0;
		}
		self.fTempPerm244 = 0.0;
		self.fTempPerm245 = 0.0;
		self.fTempPerm246 = 0.0;
		self.fTempPerm247 = 0.0;
		self.fTempPerm248 = 0.0;
		self.iTempPerm249 = 0;
		self.iTempPerm250 = 0;
		self.iTempPerm251 = 0;
		self.iTempPerm252 = 0;
		self.fTempPerm253 = 0.0;
		self.fTempPerm254 = 0.0;
		self.iTempPerm255 = 0;
		self.iTempPerm256 = 0;
		self.fTempPerm257 = 0.0;
		self.fTempPerm258 = 0.0;
		self.iTempPerm259 = 0;
		self.fTempPerm260 = 0.0;
		self.fTempPerm261 = 0.0;
		for l56 in 0..2 {
			self.fRec65[l56 as usize] = 0.0;
		}
		self.fTempPerm262 = 0.0;
		self.iTempPerm263 = 0;
		self.iTempPerm264 = 0;
		self.fTempPerm265 = 0.0;
		self.fTempPerm266 = 0.0;
		self.fTempPerm267 = 0.0;
		self.fTempPerm268 = 0.0;
		for l57 in 0..2 {
			self.fRec64[l57 as usize] = 0.0;
		}
		self.fTempPerm269 = 0.0;
		self.fTempPerm270 = 0.0;
		self.iTempPerm271 = 0;
		self.fTempPerm272 = 0.0;
		for l58 in 0..2 {
			self.fRec66[l58 as usize] = 0.0;
		}
		self.fTempPerm273 = 0.0;
		for l59 in 0..2 {
			self.fVec6[l59 as usize] = 0.0;
		}
		self.fTempPerm274 = 0.0;
		self.iTempPerm275 = 0;
		self.iTempPerm276 = 0;
		self.iTempPerm277 = 0;
		self.fTempPerm278 = 0.0;
		self.fTempPerm279 = 0.0;
		for l60 in 0..2 {
			self.fRec63[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec62[l61 as usize] = 0.0;
		}
		self.iTempPerm280 = 0;
		self.iTempPerm281 = 0;
		self.fTempPerm282 = 0.0;
		for l62 in 0..2 {
			self.fRec67[l62 as usize] = 0.0;
		}
		self.fTempPerm283 = 0.0;
		self.fTempPerm284 = 0.0;
		self.fTempPerm285 = 0.0;
		self.iTempPerm286 = 0;
		self.fTempPerm287 = 0.0;
		self.iTempPerm288 = 0;
		self.iTempPerm289 = 0;
		self.fTempPerm290 = 0.0;
		for l63 in 0..2 {
			self.fRec72[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec71[l64 as usize] = 0.0;
		}
		self.fTempPerm291 = 0.0;
		self.fTempPerm292 = 0.0;
		self.iTempPerm293 = 0;
		self.fTempPerm294 = 0.0;
		self.fTempPerm295 = 0.0;
		self.fTempPerm296 = 0.0;
		self.iTempPerm297 = 0;
		self.fTempPerm298 = 0.0;
		self.fTempPerm299 = 0.0;
		for l65 in 0..2 {
			self.fRec73[l65 as usize] = 0.0;
		}
		self.fTempPerm300 = 0.0;
		self.fTempPerm301 = 0.0;
		self.fTempPerm302 = 0.0;
		self.fTempPerm303 = 0.0;
		self.fTempPerm304 = 0.0;
		self.iTempPerm305 = 0;
		self.iTempPerm306 = 0;
		self.iTempPerm307 = 0;
		self.fTempPerm308 = 0.0;
		self.fTempPerm309 = 0.0;
		self.iTempPerm310 = 0;
		self.iTempPerm311 = 0;
		self.fTempPerm312 = 0.0;
		self.fTempPerm313 = 0.0;
		self.iTempPerm314 = 0;
		self.fTempPerm315 = 0.0;
		self.fTempPerm316 = 0.0;
		for l66 in 0..2 {
			self.fRec77[l66 as usize] = 0.0;
		}
		self.fTempPerm317 = 0.0;
		self.iTempPerm318 = 0;
		self.iTempPerm319 = 0;
		self.fTempPerm320 = 0.0;
		self.fTempPerm321 = 0.0;
		self.fTempPerm322 = 0.0;
		self.fTempPerm323 = 0.0;
		for l67 in 0..2 {
			self.fRec76[l67 as usize] = 0.0;
		}
		self.fTempPerm324 = 0.0;
		self.fTempPerm325 = 0.0;
		self.iTempPerm326 = 0;
		self.fTempPerm327 = 0.0;
		for l68 in 0..2 {
			self.fRec78[l68 as usize] = 0.0;
		}
		self.fTempPerm328 = 0.0;
		for l69 in 0..2 {
			self.fVec7[l69 as usize] = 0.0;
		}
		self.fTempPerm329 = 0.0;
		self.iTempPerm330 = 0;
		self.iTempPerm331 = 0;
		self.iTempPerm332 = 0;
		self.fTempPerm333 = 0.0;
		self.fTempPerm334 = 0.0;
		for l70 in 0..2 {
			self.fRec75[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec74[l71 as usize] = 0.0;
		}
		self.iTempPerm335 = 0;
		self.iTempPerm336 = 0;
		self.fTempPerm337 = 0.0;
		for l72 in 0..2 {
			self.fRec79[l72 as usize] = 0.0;
		}
		self.fTempPerm338 = 0.0;
		self.fTempPerm339 = 0.0;
		self.fTempPerm340 = 0.0;
		self.iTempPerm341 = 0;
		self.fTempPerm342 = 0.0;
		self.iTempPerm343 = 0;
		self.iTempPerm344 = 0;
		self.fTempPerm345 = 0.0;
		for l73 in 0..2 {
			self.fRec84[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec83[l74 as usize] = 0.0;
		}
		self.fTempPerm346 = 0.0;
		self.fTempPerm347 = 0.0;
		self.iTempPerm348 = 0;
		self.fTempPerm349 = 0.0;
		self.fTempPerm350 = 0.0;
		self.fTempPerm351 = 0.0;
		self.iTempPerm352 = 0;
		self.fTempPerm353 = 0.0;
		self.fTempPerm354 = 0.0;
		for l75 in 0..2 {
			self.fRec85[l75 as usize] = 0.0;
		}
		self.fTempPerm355 = 0.0;
		self.fTempPerm356 = 0.0;
		self.fTempPerm357 = 0.0;
		self.fTempPerm358 = 0.0;
		self.fTempPerm359 = 0.0;
		self.fTempPerm360 = 0.0;
		self.fTempPerm361 = 0.0;
		for l76 in 0..2 {
			self.fRec80[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec81[l77 as usize] = 0.0;
		}
		self.fTempPerm362 = 0.0;
		self.fTempPerm363 = 0.0;
		self.fTempPerm364 = 0.0;
		for l78 in 0..2 {
			self.fRec68[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec69[l79 as usize] = 0.0;
		}
		self.fTempPerm365 = 0.0;
		self.fTempPerm366 = 0.0;
		self.fTempPerm367 = 0.0;
		for l80 in 0..2 {
			self.fRec56[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec57[l81 as usize] = 0.0;
		}
		self.fTempPerm368 = 0.0;
		self.fTempPerm369 = 0.0;
		self.fTempPerm370 = 0.0;
		for l82 in 0..2 {
			self.fRec44[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec45[l83 as usize] = 0.0;
		}
		self.fTempPerm371 = 0.0;
		self.fTempPerm372 = 0.0;
		self.fTempPerm373 = 0.0;
		for l84 in 0..2 {
			self.fRec32[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec33[l85 as usize] = 0.0;
		}
		self.fTempPerm374 = 0.0;
		self.fTempPerm375 = 0.0;
		self.fTempPerm376 = 0.0;
		for l86 in 0..2 {
			self.fRec19[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec20[l87 as usize] = 0.0;
		}
		self.fTempPerm377 = 0.0;
		for l88 in 0..2 {
			self.fRec86[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec87[l89 as usize] = 0.0;
		}
		self.fTempPerm378 = 0.0;
		for l90 in 0..2 {
			self.fRec88[l90 as usize] = 0.0;
		}
		self.fTempPerm379 = 0.0;
		self.iTempPerm380 = 0;
		for l91 in 0..2 {
			self.iRec90[l91 as usize] = 0;
		}
		for l92 in 0..2 {
			self.iRec96[l92 as usize] = 0;
		}
		self.iTempPerm381 = 0;
		self.iTempPerm382 = 0;
		self.iTempPerm383 = 0;
		self.iTempPerm384 = 0;
		self.iTempPerm385 = 0;
		self.iTempPerm386 = 0;
		self.iTempPerm387 = 0;
		self.iTempPerm388 = 0;
		self.iTempPerm389 = 0;
		self.iTempPerm390 = 0;
		self.iTempPerm391 = 0;
		self.iTempPerm392 = 0;
		self.iTempPerm393 = 0;
		self.iTempPerm394 = 0;
		self.fTempPerm395 = 0.0;
		self.fTempPerm396 = 0.0;
		self.iTempPerm397 = 0;
		self.fTempPerm398 = 0.0;
		self.fTempPerm399 = 0.0;
		self.iTempPerm400 = 0;
		self.fTempPerm401 = 0.0;
		self.fTempPerm402 = 0.0;
		self.fTempPerm403 = 0.0;
		for l93 in 0..2 {
			self.fRec97[l93 as usize] = 0.0;
		}
		self.fTempPerm404 = 0.0;
		self.fTempPerm405 = 0.0;
		self.fTempPerm406 = 0.0;
		self.fTempPerm407 = 0.0;
		self.iTempPerm408 = 0;
		self.iTempPerm409 = 0;
		self.fTempPerm410 = 0.0;
		self.iTempPerm411 = 0;
		self.iTempPerm412 = 0;
		self.fTempPerm413 = 0.0;
		self.fTempPerm414 = 0.0;
		self.fTempPerm415 = 0.0;
		self.fTempPerm416 = 0.0;
		for l94 in 0..2 {
			self.fRec98[l94 as usize] = 0.0;
		}
		self.fTempPerm417 = 0.0;
		self.fTempPerm418 = 0.0;
		self.fTempPerm419 = 0.0;
		self.iTempPerm420 = 0;
		self.fTempPerm421 = 0.0;
		self.iTempPerm422 = 0;
		self.iTempPerm423 = 0;
		self.fTempPerm424 = 0.0;
		self.fTempPerm425 = 0.0;
		self.iTempPerm426 = 0;
		self.fTempPerm427 = 0.0;
		self.iTempPerm428 = 0;
		for l95 in 0..2 {
			self.iRec91[l95 as usize] = 0;
		}
		for l96 in 0..2 {
			self.iRec92[l96 as usize] = 0;
		}
		for l97 in 0..2 {
			self.iRec93[l97 as usize] = 0;
		}
		for l98 in 0..2 {
			self.iRec94[l98 as usize] = 0;
		}
		self.iTempPerm429 = 0;
		self.iTempPerm430 = 0;
		self.iTempPerm431 = 0;
		for l99 in 0..2 {
			self.fRec89[l99 as usize] = 0.0;
		}
		self.fTempPerm432 = 0.0;
		self.fTempPerm433 = 0.0;
		self.fTempPerm434 = 0.0;
		self.fTempPerm435 = 0.0;
		self.fTempPerm436 = 0.0;
		self.iTempPerm437 = 0;
		self.iTempPerm438 = 0;
		self.fTempPerm439 = 0.0;
		self.fTempPerm440 = 0.0;
		self.iTempPerm441 = 0;
		self.fTempPerm442 = 0.0;
		self.iTempPerm443 = 0;
		self.iTempPerm444 = 0;
		self.iTempPerm445 = 0;
		self.iTempPerm446 = 0;
		self.iTempPerm447 = 0;
		for l100 in 0..2 {
			self.fRec100[l100 as usize] = 0.0;
		}
		self.fTempPerm448 = 0.0;
		self.iTempPerm449 = 0;
		self.iTempPerm450 = 0;
		self.iTempPerm451 = 0;
		self.iTempPerm452 = 0;
		self.fTempPerm453 = 0.0;
		self.fTempPerm454 = 0.0;
		self.iTempPerm455 = 0;
		self.fTempPerm456 = 0.0;
		self.fTempPerm457 = 0.0;
		self.iTempPerm458 = 0;
		self.iTempPerm459 = 0;
		self.fTempPerm460 = 0.0;
		self.fTempPerm461 = 0.0;
		self.iTempPerm462 = 0;
		self.fTempPerm463 = 0.0;
		self.fTempPerm464 = 0.0;
		self.fTempPerm465 = 0.0;
		self.fTempPerm466 = 0.0;
		self.iTempPerm467 = 0;
		self.fTempPerm468 = 0.0;
		self.fTempPerm469 = 0.0;
		for l101 in 0..2 {
			self.fRec101[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec102[l102 as usize] = 0.0;
		}
		self.fTempPerm470 = 0.0;
		self.fTempPerm471 = 0.0;
		self.iTempPerm472 = 0;
		self.fTempPerm473 = 0.0;
		self.fTempPerm474 = 0.0;
		for l103 in 0..2 {
			self.fRec99[l103 as usize] = 0.0;
		}
		self.iTempPerm475 = 0;
		self.iTempPerm476 = 0;
		for l104 in 0..2 {
			self.fRec103[l104 as usize] = 0.0;
		}
		self.fTempPerm477 = 0.0;
		self.fTempPerm478 = 0.0;
		self.fTempPerm479 = 0.0;
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
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("Perform");
		ui_interface.declare(Some(ParamIndex(0)), "1", "");
		ui_interface.declare(Some(ParamIndex(0)), "style", "menu{'None':0;'Cutoff':1;'Resonance':2;'Morph':3;'Bandpass':4;'Filter Env Amount':5;'Drive':6;'Dual A':7;'Dual B':8;'Dual Pitch':9;'Mod Env Amount':10;'Mod LFO Amount':11;'WT Position':12;'WT LFO Rate':13;'Pulse Width':14;'PWM Depth':15;'Unison Detune':16;'Noise Level':17;'Noise Tilt':18;'Dual Level':19;'WT Level':20;'Vibrato':21;'Tremolo':22;'Level':23;'OscMod Depth':24}");
		ui_interface.add_num_entry("MW Dest", ParamIndex(0), 21.0, 0.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(1)), "2", "");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "%");
		ui_interface.add_horizontal_slider("MW Depth", ParamIndex(1), 5e+01, -1e+02, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(2)), "3", "");
		ui_interface.declare(Some(ParamIndex(2)), "style", "menu{'None':0;'Cutoff':1;'Resonance':2;'Morph':3;'Bandpass':4;'Filter Env Amount':5;'Drive':6;'Dual A':7;'Dual B':8;'Dual Pitch':9;'Mod Env Amount':10;'Mod LFO Amount':11;'WT Position':12;'WT LFO Rate':13;'Pulse Width':14;'PWM Depth':15;'Unison Detune':16;'Noise Level':17;'Noise Tilt':18;'Dual Level':19;'WT Level':20;'Vibrato':21;'Tremolo':22;'Level':23;'OscMod Depth':24}");
		ui_interface.add_num_entry("AT Dest", ParamIndex(2), 1.0, 0.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(3)), "4", "");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "%");
		ui_interface.add_horizontal_slider("AT Depth", ParamIndex(3), 5e+01, -1e+02, 1e+02, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "11", "");
		ui_interface.open_horizontal_box("Events");
		ui_interface.declare(Some(ParamIndex(4)), "0", "");
		ui_interface.add_check_button("Mute", ParamIndex(4));
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.add_horizontal_slider("Density Env", ParamIndex(5), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.add_horizontal_slider("Density LFO", ParamIndex(6), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "1", "");
		ui_interface.declare(Some(ParamIndex(7)), "unit", "%");
		ui_interface.add_horizontal_slider("Density", ParamIndex(7), 8e+01, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "2", "");
		ui_interface.add_horizontal_slider("Depth Env", ParamIndex(8), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "2", "");
		ui_interface.add_horizontal_slider("Depth LFO", ParamIndex(9), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "2", "");
		ui_interface.declare(Some(ParamIndex(10)), "unit", "dB");
		ui_interface.add_horizontal_slider("Depth", ParamIndex(10), 0.0, 0.0, 12.0, 0.1);
		ui_interface.declare(Some(ParamIndex(11)), "3", "");
		ui_interface.add_horizontal_slider("Life", ParamIndex(11), 2.0, 1.0, 3.0, 0.01);
		ui_interface.declare(Some(ParamIndex(12)), "3", "");
		ui_interface.add_horizontal_slider("Life Env", ParamIndex(12), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(13)), "3", "");
		ui_interface.add_horizontal_slider("Life LFO", ParamIndex(13), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(14)), "4", "");
		ui_interface.add_horizontal_slider("Sharp", ParamIndex(14), 1.0, 0.5, 4.0, 0.01);
		ui_interface.declare(Some(ParamIndex(15)), "4", "");
		ui_interface.add_horizontal_slider("Sharp Env", ParamIndex(15), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(16)), "4", "");
		ui_interface.add_horizontal_slider("Sharp LFO", ParamIndex(16), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "12", "");
		ui_interface.open_horizontal_box("Calls");
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("Level Env", ParamIndex(17), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(18)), "1", "");
		ui_interface.add_horizontal_slider("Level LFO", ParamIndex(18), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.declare(Some(ParamIndex(19)), "unit", "dB");
		ui_interface.add_horizontal_slider("Level", ParamIndex(19), -8e+01, -8e+01, -26.0, 0.1);
		ui_interface.declare(Some(ParamIndex(20)), "2", "");
		ui_interface.add_horizontal_slider("Activity", ParamIndex(20), 0.45, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(21)), "2", "");
		ui_interface.add_horizontal_slider("Activity Env", ParamIndex(21), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(22)), "2", "");
		ui_interface.add_horizontal_slider("Activity LFO", ParamIndex(22), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(23)), "3", "");
		ui_interface.add_horizontal_slider("Rate", ParamIndex(23), 1.0, 0.25, 3.0, 0.01);
		ui_interface.declare(Some(ParamIndex(24)), "3", "");
		ui_interface.add_horizontal_slider("Rate Env", ParamIndex(24), 0.0, -1.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(25)), "3", "");
		ui_interface.add_horizontal_slider("Rate LFO", ParamIndex(25), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(26)), "hidden", "1");
		ui_interface.add_horizontal_slider("aftertouch", ParamIndex(26), 0.0, 0.0, 1.0, 0.001);
		ui_interface.add_button("gate", ParamIndex(27));
		ui_interface.declare(Some(ParamIndex(28)), "hidden", "1");
		ui_interface.declare(Some(ParamIndex(28)), "midi", "ctrl 1");
		ui_interface.add_horizontal_slider("modwheel", ParamIndex(28), 0.0, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("slot", ParamIndex(29), 0.0, 0.0, 64.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			27 => Some(self.fButton0),
			4 => Some(self.fCheckbox0),
			2 => Some(self.fEntry0),
			0 => Some(self.fEntry1),
			10 => Some(self.fHslider0),
			19 => Some(self.fHslider1),
			6 => Some(self.fHslider10),
			5 => Some(self.fHslider11),
			25 => Some(self.fHslider12),
			24 => Some(self.fHslider13),
			16 => Some(self.fHslider14),
			15 => Some(self.fHslider15),
			9 => Some(self.fHslider16),
			8 => Some(self.fHslider17),
			3 => Some(self.fHslider18),
			26 => Some(self.fHslider19),
			29 => Some(self.fHslider2),
			1 => Some(self.fHslider20),
			28 => Some(self.fHslider21),
			7 => Some(self.fHslider22),
			14 => Some(self.fHslider23),
			23 => Some(self.fHslider24),
			20 => Some(self.fHslider25),
			13 => Some(self.fHslider3),
			12 => Some(self.fHslider4),
			11 => Some(self.fHslider5),
			18 => Some(self.fHslider6),
			17 => Some(self.fHslider7),
			22 => Some(self.fHslider8),
			21 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			27 => { self.fButton0 = value }
			4 => { self.fCheckbox0 = value }
			2 => { self.fEntry0 = value }
			0 => { self.fEntry1 = value }
			10 => { self.fHslider0 = value }
			19 => { self.fHslider1 = value }
			6 => { self.fHslider10 = value }
			5 => { self.fHslider11 = value }
			25 => { self.fHslider12 = value }
			24 => { self.fHslider13 = value }
			16 => { self.fHslider14 = value }
			15 => { self.fHslider15 = value }
			9 => { self.fHslider16 = value }
			8 => { self.fHslider17 = value }
			3 => { self.fHslider18 = value }
			26 => { self.fHslider19 = value }
			29 => { self.fHslider2 = value }
			1 => { self.fHslider20 = value }
			28 => { self.fHslider21 = value }
			7 => { self.fHslider22 = value }
			14 => { self.fHslider23 = value }
			23 => { self.fHslider24 = value }
			20 => { self.fHslider25 = value }
			13 => { self.fHslider3 = value }
			12 => { self.fHslider4 = value }
			11 => { self.fHslider5 = value }
			18 => { self.fHslider6 = value }
			17 => { self.fHslider7 = value }
			22 => { self.fHslider8 = value }
			21 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2, inputs3, inputs4) = if let [inputs0, inputs1, inputs2, inputs3, inputs4, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			let inputs3 = inputs3[..count as usize].iter();
			let inputs4 = inputs4[..count as usize].iter();
			(inputs0, inputs1, inputs2, inputs3, inputs4)
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
		let mut iSlow14: i32 = ((fSlow13 != 0.0) as i32) | ((fSlow12 != 0.0) as i32);
		let mut fSlow15: F32 = self.fHslider5;
		let mut fSlow16: F32 = (iSlow14) as F32;
		let mut iSlow17: i32 = (fSlow16 != 0.0) as i32;
		let mut fSlow18: F32 = self.fEntry0;
		let mut iSlow19: i32 = (fSlow18 == 1e+01) as i32;
		let mut fSlow20: F32 = self.fHslider6;
		let mut fSlow21: F32 = self.fHslider7;
		let mut iSlow22: i32 = ((fSlow21 != 0.0) as i32) | ((fSlow20 != 0.0) as i32);
		let mut fSlow23: F32 = (iSlow22) as F32;
		let mut iSlow24: i32 = (fSlow23 != 0.0) as i32;
		let mut fSlow25: F32 = self.fHslider8;
		let mut fSlow26: F32 = self.fHslider9;
		let mut iSlow27: i32 = ((fSlow26 != 0.0) as i32) | ((fSlow25 != 0.0) as i32);
		let mut fSlow28: F32 = (iSlow27) as F32;
		let mut iSlow29: i32 = (fSlow28 != 0.0) as i32;
		let mut fSlow30: F32 = self.fHslider10;
		let mut fSlow31: F32 = self.fHslider11;
		let mut iSlow32: i32 = ((fSlow31 != 0.0) as i32) | ((fSlow30 != 0.0) as i32);
		let mut fSlow33: F32 = (iSlow32) as F32;
		let mut iSlow34: i32 = (fSlow33 != 0.0) as i32;
		let mut fSlow35: F32 = self.fHslider12;
		let mut fSlow36: F32 = self.fHslider13;
		let mut iSlow37: i32 = ((fSlow36 != 0.0) as i32) | ((fSlow35 != 0.0) as i32);
		let mut fSlow38: F32 = (iSlow37) as F32;
		let mut iSlow39: i32 = (fSlow38 != 0.0) as i32;
		let mut fSlow40: F32 = self.fHslider14;
		let mut fSlow41: F32 = self.fHslider15;
		let mut iSlow42: i32 = ((fSlow41 != 0.0) as i32) | ((fSlow40 != 0.0) as i32);
		let mut fSlow43: F32 = (iSlow42) as F32;
		let mut iSlow44: i32 = (fSlow43 != 0.0) as i32;
		let mut fSlow45: F32 = self.fHslider16;
		let mut fSlow46: F32 = self.fHslider17;
		let mut iSlow47: i32 = ((fSlow46 != 0.0) as i32) | ((fSlow45 != 0.0) as i32);
		let mut fSlow48: F32 = (iSlow47) as F32;
		let mut iSlow49: i32 = (fSlow48 != 0.0) as i32;
		let mut fSlow50: F32 = 0.01 * self.fHslider18;
		let mut fSlow51: F32 = self.fHslider19;
		let mut fTemp24: F32 = self.fTempPerm19;
		let mut fSlow52: F32 = self.fEntry1;
		let mut iSlow53: i32 = (fSlow52 == 1e+01) as i32;
		let mut fSlow54: F32 = 0.01 * self.fHslider20;
		let mut fSlow55: F32 = self.fHslider21;
		let mut fTemp25: F32 = self.fTempPerm20;
		let mut fTemp26: F32 = self.fTempPerm21;
		let mut iSlow56: i32 = (fSlow18 == 11.0) as i32;
		let mut iSlow57: i32 = (fSlow52 == 11.0) as i32;
		let mut fTemp27: F32 = self.fTempPerm22;
		let mut fTemp28: F32 = self.fTempPerm23;
		let mut fSlow58: F32 = ((iSlow14 != 0) as i32) as u32 as F32;
		let mut fTemp29: F32 = self.fTempPerm24;
		let mut fTemp30: F32 = self.fTempPerm25;
		let mut fSlow59: F32 = self.fHslider22;
		let mut fSlow60: F32 = 1e+02 * fSlow33;
		let mut fTemp31: F32 = self.fTempPerm26;
		let mut fSlow61: F32 = ((iSlow32 != 0) as i32) as u32 as F32;
		let mut fTemp32: F32 = self.fTempPerm27;
		let mut fTemp33: F32 = self.fTempPerm28;
		let mut fTemp34: F32 = self.fTempPerm29;
		let mut iTemp35: i32 = self.iTempPerm30;
		let mut iTemp36: i32 = self.iTempPerm31;
		let mut fTemp37: F32 = self.fTempPerm32;
		let mut fTemp38: F32 = self.fTempPerm33;
		let mut fTemp39: F32 = self.fTempPerm34;
		let mut fTemp40: F32 = self.fTempPerm35;
		let mut fTemp41: F32 = self.fTempPerm36;
		let mut fTemp42: F32 = self.fTempPerm37;
		let mut iTemp43: i32 = self.iTempPerm38;
		let mut fTemp44: F32 = self.fTempPerm39;
		let mut fTemp45: F32 = self.fTempPerm40;
		let mut fTemp46: F32 = self.fTempPerm41;
		let mut iTemp47: i32 = self.iTempPerm42;
		let mut iTemp48: i32 = self.iTempPerm43;
		let mut iTemp49: i32 = self.iTempPerm44;
		let mut fTemp50: F32 = self.fTempPerm45;
		let mut fTemp51: F32 = self.fTempPerm46;
		let mut iTemp52: i32 = self.iTempPerm47;
		let mut iTemp53: i32 = self.iTempPerm48;
		let mut fTemp54: F32 = self.fTempPerm49;
		let mut fTemp55: F32 = self.fTempPerm50;
		let mut fSlow62: F32 = 12.0 * fSlow48;
		let mut fTemp56: F32 = self.fTempPerm51;
		let mut fSlow63: F32 = ((iSlow47 != 0) as i32) as u32 as F32;
		let mut fTemp57: F32 = self.fTempPerm52;
		let mut fSlow64: F32 = (iSlow3) as F32;
		let mut fTemp58: F32 = self.fTempPerm53;
		let mut fTemp59: F32 = self.fTempPerm54;
		let mut fSlow65: F32 = 3.33 * fSlow64;
		let mut fTemp60: F32 = self.fTempPerm55;
		let mut fTemp61: F32 = self.fTempPerm56;
		let mut fTemp62: F32 = self.fTempPerm57;
		let mut iTemp63: i32 = self.iTempPerm58;
		let mut fTemp64: F32 = self.fTempPerm59;
		let mut fSlow66: F32 = self.fHslider23;
		let mut fTemp65: F32 = self.fTempPerm60;
		let mut fSlow67: F32 = ((iSlow42 != 0) as i32) as u32 as F32;
		let mut fTemp66: F32 = self.fTempPerm61;
		let mut fTemp67: F32 = self.fTempPerm62;
		let mut iTemp68: i32 = self.iTempPerm63;
		let mut iTemp69: i32 = self.iTempPerm64;
		let mut fTemp70: F32 = self.fTempPerm65;
		let mut fTemp71: F32 = self.fTempPerm66;
		let mut fTemp72: F32 = self.fTempPerm67;
		let mut iTemp73: i32 = self.iTempPerm68;
		let mut fTemp74: F32 = self.fTempPerm69;
		let mut fTemp75: F32 = self.fTempPerm70;
		let mut fTemp76: F32 = self.fTempPerm71;
		let mut iTemp77: i32 = self.iTempPerm72;
		let mut fTemp78: F32 = self.fTempPerm73;
		let mut fTemp79: F32 = self.fTempPerm74;
		let mut fTemp80: F32 = self.fTempPerm75;
		let mut fTemp81: F32 = self.fTempPerm76;
		let mut fTemp82: F32 = self.fTempPerm77;
		let mut fTemp83: F32 = self.fTempPerm78;
		let mut fTemp84: F32 = self.fTempPerm79;
		let mut fTemp85: F32 = self.fTempPerm80;
		let mut iTemp86: i32 = self.iTempPerm81;
		let mut iTemp87: i32 = self.iTempPerm82;
		let mut iTemp88: i32 = self.iTempPerm83;
		let mut iTemp89: i32 = self.iTempPerm84;
		let mut fTemp90: F32 = self.fTempPerm85;
		let mut fTemp91: F32 = self.fTempPerm86;
		let mut iTemp92: i32 = self.iTempPerm87;
		let mut iTemp93: i32 = self.iTempPerm88;
		let mut fTemp94: F32 = self.fTempPerm89;
		let mut fTemp95: F32 = self.fTempPerm90;
		let mut iTemp96: i32 = self.iTempPerm91;
		let mut fTemp97: F32 = self.fTempPerm92;
		let mut fTemp98: F32 = self.fTempPerm93;
		let mut fTemp99: F32 = self.fTempPerm94;
		let mut iTemp100: i32 = self.iTempPerm95;
		let mut iTemp101: i32 = self.iTempPerm96;
		let mut fTemp102: F32 = self.fTempPerm97;
		let mut fTemp103: F32 = self.fTempPerm98;
		let mut fTemp104: F32 = self.fTempPerm99;
		let mut fTemp105: F32 = self.fTempPerm100;
		let mut fTemp106: F32 = self.fTempPerm101;
		let mut fTemp107: F32 = self.fTempPerm102;
		let mut iTemp108: i32 = self.iTempPerm103;
		let mut fTemp109: F32 = self.fTempPerm104;
		let mut fTemp110: F32 = self.fTempPerm105;
		let mut fTemp111: F32 = self.fTempPerm106;
		let mut iTemp112: i32 = self.iTempPerm107;
		let mut iTemp113: i32 = self.iTempPerm108;
		let mut iTemp114: i32 = self.iTempPerm109;
		let mut fTemp115: F32 = self.fTempPerm110;
		let mut fTemp116: F32 = self.fTempPerm111;
		let mut iTemp117: i32 = self.iTempPerm112;
		let mut iTemp118: i32 = self.iTempPerm113;
		let mut fTemp119: F32 = self.fTempPerm114;
		let mut fTemp120: F32 = self.fTempPerm115;
		let mut fTemp121: F32 = self.fTempPerm116;
		let mut fTemp122: F32 = self.fTempPerm117;
		let mut iTemp123: i32 = self.iTempPerm118;
		let mut fTemp124: F32 = self.fTempPerm119;
		let mut iTemp125: i32 = self.iTempPerm120;
		let mut iTemp126: i32 = self.iTempPerm121;
		let mut fTemp127: F32 = self.fTempPerm122;
		let mut fTemp128: F32 = self.fTempPerm123;
		let mut fTemp129: F32 = self.fTempPerm124;
		let mut iTemp130: i32 = self.iTempPerm125;
		let mut fTemp131: F32 = self.fTempPerm126;
		let mut fTemp132: F32 = self.fTempPerm127;
		let mut fTemp133: F32 = self.fTempPerm128;
		let mut iTemp134: i32 = self.iTempPerm129;
		let mut fTemp135: F32 = self.fTempPerm130;
		let mut fTemp136: F32 = self.fTempPerm131;
		let mut fTemp137: F32 = self.fTempPerm132;
		let mut fTemp138: F32 = self.fTempPerm133;
		let mut fTemp139: F32 = self.fTempPerm134;
		let mut fTemp140: F32 = self.fTempPerm135;
		let mut fTemp141: F32 = self.fTempPerm136;
		let mut iTemp142: i32 = self.iTempPerm137;
		let mut iTemp143: i32 = self.iTempPerm138;
		let mut iTemp144: i32 = self.iTempPerm139;
		let mut iTemp145: i32 = self.iTempPerm140;
		let mut fTemp146: F32 = self.fTempPerm141;
		let mut fTemp147: F32 = self.fTempPerm142;
		let mut iTemp148: i32 = self.iTempPerm143;
		let mut iTemp149: i32 = self.iTempPerm144;
		let mut fTemp150: F32 = self.fTempPerm145;
		let mut fTemp151: F32 = self.fTempPerm146;
		let mut iTemp152: i32 = self.iTempPerm147;
		let mut fTemp153: F32 = self.fTempPerm148;
		let mut fTemp154: F32 = self.fTempPerm149;
		let mut fTemp155: F32 = self.fTempPerm150;
		let mut iTemp156: i32 = self.iTempPerm151;
		let mut iTemp157: i32 = self.iTempPerm152;
		let mut fTemp158: F32 = self.fTempPerm153;
		let mut fTemp159: F32 = self.fTempPerm154;
		let mut fTemp160: F32 = self.fTempPerm155;
		let mut fTemp161: F32 = self.fTempPerm156;
		let mut fTemp162: F32 = self.fTempPerm157;
		let mut fTemp163: F32 = self.fTempPerm158;
		let mut iTemp164: i32 = self.iTempPerm159;
		let mut fTemp165: F32 = self.fTempPerm160;
		let mut fTemp166: F32 = self.fTempPerm161;
		let mut fTemp167: F32 = self.fTempPerm162;
		let mut iTemp168: i32 = self.iTempPerm163;
		let mut iTemp169: i32 = self.iTempPerm164;
		let mut iTemp170: i32 = self.iTempPerm165;
		let mut fTemp171: F32 = self.fTempPerm166;
		let mut fTemp172: F32 = self.fTempPerm167;
		let mut iTemp173: i32 = self.iTempPerm168;
		let mut iTemp174: i32 = self.iTempPerm169;
		let mut fTemp175: F32 = self.fTempPerm170;
		let mut fTemp176: F32 = self.fTempPerm171;
		let mut fTemp177: F32 = self.fTempPerm172;
		let mut fTemp178: F32 = self.fTempPerm173;
		let mut iTemp179: i32 = self.iTempPerm174;
		let mut fTemp180: F32 = self.fTempPerm175;
		let mut iTemp181: i32 = self.iTempPerm176;
		let mut iTemp182: i32 = self.iTempPerm177;
		let mut fTemp183: F32 = self.fTempPerm178;
		let mut fTemp184: F32 = self.fTempPerm179;
		let mut fTemp185: F32 = self.fTempPerm180;
		let mut iTemp186: i32 = self.iTempPerm181;
		let mut fTemp187: F32 = self.fTempPerm182;
		let mut fTemp188: F32 = self.fTempPerm183;
		let mut fTemp189: F32 = self.fTempPerm184;
		let mut iTemp190: i32 = self.iTempPerm185;
		let mut fTemp191: F32 = self.fTempPerm186;
		let mut fTemp192: F32 = self.fTempPerm187;
		let mut fTemp193: F32 = self.fTempPerm188;
		let mut fTemp194: F32 = self.fTempPerm189;
		let mut fTemp195: F32 = self.fTempPerm190;
		let mut fTemp196: F32 = self.fTempPerm191;
		let mut fTemp197: F32 = self.fTempPerm192;
		let mut iTemp198: i32 = self.iTempPerm193;
		let mut iTemp199: i32 = self.iTempPerm194;
		let mut iTemp200: i32 = self.iTempPerm195;
		let mut iTemp201: i32 = self.iTempPerm196;
		let mut fTemp202: F32 = self.fTempPerm197;
		let mut fTemp203: F32 = self.fTempPerm198;
		let mut iTemp204: i32 = self.iTempPerm199;
		let mut iTemp205: i32 = self.iTempPerm200;
		let mut fTemp206: F32 = self.fTempPerm201;
		let mut fTemp207: F32 = self.fTempPerm202;
		let mut iTemp208: i32 = self.iTempPerm203;
		let mut fTemp209: F32 = self.fTempPerm204;
		let mut fTemp210: F32 = self.fTempPerm205;
		let mut fTemp211: F32 = self.fTempPerm206;
		let mut iTemp212: i32 = self.iTempPerm207;
		let mut iTemp213: i32 = self.iTempPerm208;
		let mut fTemp214: F32 = self.fTempPerm209;
		let mut fTemp215: F32 = self.fTempPerm210;
		let mut fTemp216: F32 = self.fTempPerm211;
		let mut fTemp217: F32 = self.fTempPerm212;
		let mut fTemp218: F32 = self.fTempPerm213;
		let mut fTemp219: F32 = self.fTempPerm214;
		let mut iTemp220: i32 = self.iTempPerm215;
		let mut fTemp221: F32 = self.fTempPerm216;
		let mut fTemp222: F32 = self.fTempPerm217;
		let mut fTemp223: F32 = self.fTempPerm218;
		let mut iTemp224: i32 = self.iTempPerm219;
		let mut iTemp225: i32 = self.iTempPerm220;
		let mut iTemp226: i32 = self.iTempPerm221;
		let mut fTemp227: F32 = self.fTempPerm222;
		let mut fTemp228: F32 = self.fTempPerm223;
		let mut iTemp229: i32 = self.iTempPerm224;
		let mut iTemp230: i32 = self.iTempPerm225;
		let mut fTemp231: F32 = self.fTempPerm226;
		let mut fTemp232: F32 = self.fTempPerm227;
		let mut fTemp233: F32 = self.fTempPerm228;
		let mut fTemp234: F32 = self.fTempPerm229;
		let mut iTemp235: i32 = self.iTempPerm230;
		let mut fTemp236: F32 = self.fTempPerm231;
		let mut iTemp237: i32 = self.iTempPerm232;
		let mut iTemp238: i32 = self.iTempPerm233;
		let mut fTemp239: F32 = self.fTempPerm234;
		let mut fTemp240: F32 = self.fTempPerm235;
		let mut fTemp241: F32 = self.fTempPerm236;
		let mut iTemp242: i32 = self.iTempPerm237;
		let mut fTemp243: F32 = self.fTempPerm238;
		let mut fTemp244: F32 = self.fTempPerm239;
		let mut fTemp245: F32 = self.fTempPerm240;
		let mut iTemp246: i32 = self.iTempPerm241;
		let mut fTemp247: F32 = self.fTempPerm242;
		let mut fTemp248: F32 = self.fTempPerm243;
		let mut fTemp249: F32 = self.fTempPerm244;
		let mut fTemp250: F32 = self.fTempPerm245;
		let mut fTemp251: F32 = self.fTempPerm246;
		let mut fTemp252: F32 = self.fTempPerm247;
		let mut fTemp253: F32 = self.fTempPerm248;
		let mut iTemp254: i32 = self.iTempPerm249;
		let mut iTemp255: i32 = self.iTempPerm250;
		let mut iTemp256: i32 = self.iTempPerm251;
		let mut iTemp257: i32 = self.iTempPerm252;
		let mut fTemp258: F32 = self.fTempPerm253;
		let mut fTemp259: F32 = self.fTempPerm254;
		let mut iTemp260: i32 = self.iTempPerm255;
		let mut iTemp261: i32 = self.iTempPerm256;
		let mut fTemp262: F32 = self.fTempPerm257;
		let mut fTemp263: F32 = self.fTempPerm258;
		let mut iTemp264: i32 = self.iTempPerm259;
		let mut fTemp265: F32 = self.fTempPerm260;
		let mut fTemp266: F32 = self.fTempPerm261;
		let mut fTemp267: F32 = self.fTempPerm262;
		let mut iTemp268: i32 = self.iTempPerm263;
		let mut iTemp269: i32 = self.iTempPerm264;
		let mut fTemp270: F32 = self.fTempPerm265;
		let mut fTemp271: F32 = self.fTempPerm266;
		let mut fTemp272: F32 = self.fTempPerm267;
		let mut fTemp273: F32 = self.fTempPerm268;
		let mut fTemp274: F32 = self.fTempPerm269;
		let mut fTemp275: F32 = self.fTempPerm270;
		let mut iTemp276: i32 = self.iTempPerm271;
		let mut fTemp277: F32 = self.fTempPerm272;
		let mut fTemp278: F32 = self.fTempPerm273;
		let mut fTemp279: F32 = self.fTempPerm274;
		let mut iTemp280: i32 = self.iTempPerm275;
		let mut iTemp281: i32 = self.iTempPerm276;
		let mut iTemp282: i32 = self.iTempPerm277;
		let mut fTemp283: F32 = self.fTempPerm278;
		let mut fTemp284: F32 = self.fTempPerm279;
		let mut iTemp285: i32 = self.iTempPerm280;
		let mut iTemp286: i32 = self.iTempPerm281;
		let mut fTemp287: F32 = self.fTempPerm282;
		let mut fTemp288: F32 = self.fTempPerm283;
		let mut fTemp289: F32 = self.fTempPerm284;
		let mut fTemp290: F32 = self.fTempPerm285;
		let mut iTemp291: i32 = self.iTempPerm286;
		let mut fTemp292: F32 = self.fTempPerm287;
		let mut iTemp293: i32 = self.iTempPerm288;
		let mut iTemp294: i32 = self.iTempPerm289;
		let mut fTemp295: F32 = self.fTempPerm290;
		let mut fTemp296: F32 = self.fTempPerm291;
		let mut fTemp297: F32 = self.fTempPerm292;
		let mut iTemp298: i32 = self.iTempPerm293;
		let mut fTemp299: F32 = self.fTempPerm294;
		let mut fTemp300: F32 = self.fTempPerm295;
		let mut fTemp301: F32 = self.fTempPerm296;
		let mut iTemp302: i32 = self.iTempPerm297;
		let mut fTemp303: F32 = self.fTempPerm298;
		let mut fTemp304: F32 = self.fTempPerm299;
		let mut fTemp305: F32 = self.fTempPerm300;
		let mut fTemp306: F32 = self.fTempPerm301;
		let mut fTemp307: F32 = self.fTempPerm302;
		let mut fTemp308: F32 = self.fTempPerm303;
		let mut fTemp309: F32 = self.fTempPerm304;
		let mut iTemp310: i32 = self.iTempPerm305;
		let mut iTemp311: i32 = self.iTempPerm306;
		let mut iTemp312: i32 = self.iTempPerm307;
		let mut fTemp313: F32 = self.fTempPerm308;
		let mut fTemp314: F32 = self.fTempPerm309;
		let mut iTemp315: i32 = self.iTempPerm310;
		let mut iTemp316: i32 = self.iTempPerm311;
		let mut fTemp317: F32 = self.fTempPerm312;
		let mut fTemp318: F32 = self.fTempPerm313;
		let mut iTemp319: i32 = self.iTempPerm314;
		let mut fTemp320: F32 = self.fTempPerm315;
		let mut fTemp321: F32 = self.fTempPerm316;
		let mut fTemp322: F32 = self.fTempPerm317;
		let mut iTemp323: i32 = self.iTempPerm318;
		let mut iTemp324: i32 = self.iTempPerm319;
		let mut fTemp325: F32 = self.fTempPerm320;
		let mut fTemp326: F32 = self.fTempPerm321;
		let mut fTemp327: F32 = self.fTempPerm322;
		let mut fTemp328: F32 = self.fTempPerm323;
		let mut fTemp329: F32 = self.fTempPerm324;
		let mut fTemp330: F32 = self.fTempPerm325;
		let mut iTemp331: i32 = self.iTempPerm326;
		let mut fTemp332: F32 = self.fTempPerm327;
		let mut fTemp333: F32 = self.fTempPerm328;
		let mut fTemp334: F32 = self.fTempPerm329;
		let mut iTemp335: i32 = self.iTempPerm330;
		let mut iTemp336: i32 = self.iTempPerm331;
		let mut iTemp337: i32 = self.iTempPerm332;
		let mut fTemp338: F32 = self.fTempPerm333;
		let mut fTemp339: F32 = self.fTempPerm334;
		let mut iTemp340: i32 = self.iTempPerm335;
		let mut iTemp341: i32 = self.iTempPerm336;
		let mut fTemp342: F32 = self.fTempPerm337;
		let mut fTemp343: F32 = self.fTempPerm338;
		let mut fTemp344: F32 = self.fTempPerm339;
		let mut fTemp345: F32 = self.fTempPerm340;
		let mut iTemp346: i32 = self.iTempPerm341;
		let mut fTemp347: F32 = self.fTempPerm342;
		let mut iTemp348: i32 = self.iTempPerm343;
		let mut iTemp349: i32 = self.iTempPerm344;
		let mut fTemp350: F32 = self.fTempPerm345;
		let mut fTemp351: F32 = self.fTempPerm346;
		let mut fTemp352: F32 = self.fTempPerm347;
		let mut iTemp353: i32 = self.iTempPerm348;
		let mut fTemp354: F32 = self.fTempPerm349;
		let mut fTemp355: F32 = self.fTempPerm350;
		let mut fTemp356: F32 = self.fTempPerm351;
		let mut iTemp357: i32 = self.iTempPerm352;
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
		let mut fTemp377: F32 = self.fTempPerm372;
		let mut fTemp378: F32 = self.fTempPerm373;
		let mut fTemp379: F32 = self.fTempPerm374;
		let mut fTemp380: F32 = self.fTempPerm375;
		let mut fTemp381: F32 = self.fTempPerm376;
		let mut fTemp382: F32 = self.fTempPerm377;
		let mut fTemp383: F32 = self.fTempPerm378;
		let mut fTemp384: F32 = self.fTempPerm379;
		let mut fSlow68: F32 = ((iSlow22 != 0) as i32) as u32 as F32;
		let mut iTemp392: i32 = self.iTempPerm380;
		let mut iSlow69: i32 = (811.0 * fSlow8 + 2e+03) as i32;
		let mut iTemp393: i32 = self.iTempPerm381;
		let mut iTemp394: i32 = self.iTempPerm382;
		let mut iTemp395: i32 = self.iTempPerm383;
		let mut iTemp396: i32 = self.iTempPerm384;
		let mut iTemp397: i32 = self.iTempPerm385;
		let mut iTemp398: i32 = self.iTempPerm386;
		let mut iTemp399: i32 = self.iTempPerm387;
		let mut iTemp400: i32 = self.iTempPerm388;
		let mut iTemp401: i32 = self.iTempPerm389;
		let mut iTemp402: i32 = self.iTempPerm390;
		let mut iTemp403: i32 = self.iTempPerm391;
		let mut iTemp404: i32 = self.iTempPerm392;
		let mut iTemp405: i32 = self.iTempPerm393;
		let mut iTemp406: i32 = self.iTempPerm394;
		let mut fTemp407: F32 = self.fTempPerm395;
		let mut fTemp408: F32 = self.fTempPerm396;
		let mut iTemp409: i32 = self.iTempPerm397;
		let mut fTemp410: F32 = self.fTempPerm398;
		let mut fTemp411: F32 = self.fTempPerm399;
		let mut iTemp412: i32 = self.iTempPerm400;
		let mut fTemp413: F32 = self.fTempPerm401;
		let mut fTemp414: F32 = self.fTempPerm402;
		let mut fTemp415: F32 = self.fTempPerm403;
		let mut fSlow70: F32 = self.fHslider24;
		let mut fTemp416: F32 = self.fTempPerm404;
		let mut fSlow71: F32 = ((iSlow37 != 0) as i32) as u32 as F32;
		let mut fTemp417: F32 = self.fTempPerm405;
		let mut fTemp418: F32 = self.fTempPerm406;
		let mut fTemp419: F32 = self.fTempPerm407;
		let mut iTemp420: i32 = self.iTempPerm408;
		let mut iTemp421: i32 = self.iTempPerm409;
		let mut fTemp422: F32 = self.fTempPerm410;
		let mut iTemp423: i32 = self.iTempPerm411;
		let mut iTemp424: i32 = self.iTempPerm412;
		let mut fTemp425: F32 = self.fTempPerm413;
		let mut fTemp426: F32 = self.fTempPerm414;
		let mut fTemp427: F32 = self.fTempPerm415;
		let mut fTemp428: F32 = self.fTempPerm416;
		let mut fSlow72: F32 = self.fHslider25;
		let mut fTemp429: F32 = self.fTempPerm417;
		let mut fSlow73: F32 = ((iSlow27 != 0) as i32) as u32 as F32;
		let mut fTemp430: F32 = self.fTempPerm418;
		let mut fTemp431: F32 = self.fTempPerm419;
		let mut iTemp432: i32 = self.iTempPerm420;
		let mut fTemp433: F32 = self.fTempPerm421;
		let mut iTemp434: i32 = self.iTempPerm422;
		let mut iTemp435: i32 = self.iTempPerm423;
		let mut fTemp436: F32 = self.fTempPerm424;
		let mut fTemp437: F32 = self.fTempPerm425;
		let mut iTemp438: i32 = self.iTempPerm426;
		let mut fTemp439: F32 = self.fTempPerm427;
		let mut iTemp440: i32 = self.iTempPerm428;
		let mut iTemp441: i32 = self.iTempPerm429;
		let mut iTemp442: i32 = self.iTempPerm430;
		let mut iTemp443: i32 = self.iTempPerm431;
		let mut fTemp444: F32 = self.fTempPerm432;
		let mut fTemp445: F32 = self.fTempPerm433;
		let mut fTemp446: F32 = self.fTempPerm434;
		let mut fTemp447: F32 = self.fTempPerm435;
		let mut fTemp448: F32 = self.fTempPerm436;
		let mut iTemp449: i32 = self.iTempPerm437;
		let mut iTemp450: i32 = self.iTempPerm438;
		let mut fTemp451: F32 = self.fTempPerm439;
		let mut fTemp452: F32 = self.fTempPerm440;
		let mut iTemp453: i32 = self.iTempPerm441;
		let mut fTemp454: F32 = self.fTempPerm442;
		let mut iTemp455: i32 = self.iTempPerm443;
		let mut iTemp456: i32 = self.iTempPerm444;
		let mut iTemp457: i32 = self.iTempPerm445;
		let mut iTemp458: i32 = self.iTempPerm446;
		let mut iTemp459: i32 = self.iTempPerm447;
		let mut fTemp460: F32 = self.fTempPerm448;
		let mut iTemp461: i32 = self.iTempPerm449;
		let mut iTemp462: i32 = self.iTempPerm450;
		let mut iTemp463: i32 = self.iTempPerm451;
		let mut iTemp464: i32 = self.iTempPerm452;
		let mut fTemp465: F32 = self.fTempPerm453;
		let mut fTemp466: F32 = self.fTempPerm454;
		let mut iTemp467: i32 = self.iTempPerm455;
		let mut fTemp468: F32 = self.fTempPerm456;
		let mut fTemp469: F32 = self.fTempPerm457;
		let mut iTemp470: i32 = self.iTempPerm458;
		let mut iTemp471: i32 = self.iTempPerm459;
		let mut fTemp472: F32 = self.fTempPerm460;
		let mut fTemp473: F32 = self.fTempPerm461;
		let mut iTemp474: i32 = self.iTempPerm462;
		let mut fTemp475: F32 = self.fTempPerm463;
		let mut fTemp476: F32 = self.fTempPerm464;
		let mut fTemp477: F32 = self.fTempPerm465;
		let mut fTemp478: F32 = self.fTempPerm466;
		let mut iTemp479: i32 = self.iTempPerm467;
		let mut fTemp480: F32 = self.fTempPerm468;
		let mut fTemp481: F32 = self.fTempPerm469;
		let mut fTemp482: F32 = self.fTempPerm470;
		let mut fTemp483: F32 = self.fTempPerm471;
		let mut iTemp484: i32 = self.iTempPerm472;
		let mut fTemp485: F32 = self.fTempPerm473;
		let mut fTemp486: F32 = self.fTempPerm474;
		let mut iTemp487: i32 = self.iTempPerm475;
		let mut iTemp488: i32 = self.iTempPerm476;
		let mut fTemp489: F32 = self.fTempPerm477;
		let mut fTemp490: F32 = self.fTempPerm478;
		let mut fSlow74: F32 = 2.0 * fSlow6 * ((iSlow5 != 0) as i32) as u32 as F32;
		let mut fTemp492: F32 = self.fTempPerm479;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(inputs4).zip(outputs0).zip(outputs1).zip(outputs2);
		for (((((((input0, input1), input2), input3), input4), output0), output1), output2) in zipped_iterators {
			let mut fRec82: F32 = 0.0;
			let mut fRec70: F32 = 0.0;
			let mut fRec58: F32 = 0.0;
			let mut fRec46: F32 = 0.0;
			let mut fRec34: F32 = 0.0;
			let mut fRec21: F32 = 0.0;
			let mut iRec95: i32 = 0;
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow0;
			let mut fTemp0: F32 = (i32::wrapping_sub(1, (fSlow0 > self.fVec1[1]) as i32)) as F32;
			let mut fTemp1: F32 = 1.0 - self.fConst1 * fTemp0;
			self.fRec0[0] = fSlow1 * fTemp1 + self.fConst1 * fTemp0 * self.fRec0[1];
			let mut iTemp2: i32 = iSlow3 & ((self.fRec0[0] > 0.0001) as i32);
			let mut fTemp3: F32 = *input2;
			let mut fTemp4: F32 = (iTemp2) as F32;
			if iSlow7 | ((fTemp4 != 0.0) as i32) != 0 {
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
					if iSlow7 | ((fTemp4 != 0.0) as i32) != 0 {
						fTemp13 = *input3;
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
							self.fRec10[0] = fSlow15 * fTemp1 + self.fConst1 * fTemp0 * self.fRec10[1];
						}
							if (((fTemp4 != 0.0) as i32) & iSlow49) | ((((fTemp4 != 0.0) as i32) & iSlow17) | ((((fTemp4 != 0.0) as i32) & iSlow44) | ((iSlow7 & iSlow39) | ((((fTemp4 != 0.0) as i32) & iSlow34) | ((iSlow7 & iSlow29) | iSlow24))))) != 0 {
								self.fRec11[0] = fSlow50 * fTemp1 + self.fConst1 * fTemp0 * self.fRec11[1];
								self.fRec12[0] = fSlow51 * fTemp1 + self.fConst1 * fTemp0 * self.fRec12[1];
								fTemp24 = if (self.fRec11[0] >= 0.0) as i32 != 0 {1.0 - self.fRec11[0] * (1.0 - self.fRec12[0])} else {self.fRec11[0] * self.fRec12[0] + 1.0};
								self.fRec13[0] = fSlow54 * fTemp1 + self.fConst1 * fTemp0 * self.fRec13[1];
								self.fRec14[0] = fSlow55 * fTemp1 + self.fConst1 * fTemp0 * self.fRec14[1];
								fTemp25 = if (self.fRec13[0] >= 0.0) as i32 != 0 {1.0 - self.fRec13[0] * (1.0 - self.fRec14[0])} else {self.fRec13[0] * self.fRec14[0] + 1.0};
								fTemp26 = *input0 * if iSlow53 != 0 {fTemp25} else {1.0} * if iSlow19 != 0 {fTemp24} else {1.0};
								fTemp27 = *input1 * if iSlow57 != 0 {fTemp25} else {1.0} * if iSlow56 != 0 {fTemp24} else {1.0};
							}
								if ((fTemp4 != 0.0) as i32) & iSlow17 != 0 {
									fTemp28 = fSlow16 * (2.0 * F32::max(0.0, F32::min(1.0, fSlow12 * fTemp27 + F32::max(0.0, 0.5 * (self.fRec10[0] + -1.0)) + fSlow13 * fTemp26)) + 1.0);
								}
									if (fTemp4 != 0.0) as i32 != 0 {
										fTemp29 = if iSlow14 != 0 {fSlow58 * fTemp28} else {self.fRec10[0]};
										fTemp30 = F32::max(0.005, 0.4 * (fTemp29 * (fTemp23 * (fTemp23 * (fTemp23 * (fTemp23 * (0.0013333 * fTemp23 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp22))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp22))) as F32));
										self.fRec8[0] = if iTemp10 != 0 {fTemp30} else {self.fRec8[1]};
										self.fRec15[0] = fSlow59 * fTemp1 + self.fConst1 * fTemp0 * self.fRec15[1];
									}
										if ((fTemp4 != 0.0) as i32) & iSlow34 != 0 {
											fTemp31 = fSlow60 * F32::max(0.0, F32::min(1.0, fSlow30 * fTemp27 + F32::max(0.0, 0.01 * self.fRec15[0]) + fSlow31 * fTemp26));
										}
											if (fTemp4 != 0.0) as i32 != 0 {
												fTemp32 = F32::min(0.01 * if iSlow32 != 0 {fSlow61 * fTemp31} else {self.fRec15[0]}, 0.98);
												fTemp33 = fTemp32 / (1.0 - fTemp32);
												fTemp34 = (iTemp9) as F32;
											}
												if ((fTemp4 != 0.0) as i32) & ((fTemp34 != 0.0) as i32) != 0 {
													iTemp35 = i32::wrapping_add(self.iRec9[0], 3856553);
													iTemp36 = i32::wrapping_mul(1664525, iTemp35 ^ (iTemp35 >> 15));
													fTemp37 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp36 ^ (iTemp36 >> 13))) as F32;
													fTemp38 = fTemp37 + 1.5;
													fTemp39 = 0.5 - fTemp37;
													fTemp40 = TextureDsp_faustpower2_f(fTemp39) / TextureDsp_faustpower2_f(fTemp38);
												}
													if (fTemp4 != 0.0) as i32 != 0 {
														self.fRec7[0] = if iTemp9 != 0 {-(2.0 * (fTemp39 * (15.0 - 4.0 * fTemp40) * fTemp34 / (fTemp38 * (15.0 - 9.0 * fTemp40))))} else {self.fRec7[1] + self.fConst7 * (if iTemp10 != 0 {fTemp33} else {1.0} / F32::max(self.fRec8[0], 0.005))};
														fTemp41 = F32::min(1.0, F32::max(0.0, self.fRec7[0]));
														fTemp42 = (iTemp10) as F32;
														iTemp43 = (fTemp42 != 0.0) as i32;
													}
														if ((fTemp4 != 0.0) as i32) & iTemp43 != 0 {
															fTemp44 = fTemp30 * fTemp42;
														}
															if (fTemp4 != 0.0) as i32 != 0 {
																self.fRec16[0] = if iTemp10 != 0 {fTemp44} else {self.fRec16[1]};
																fTemp45 = F32::floor(41.666668 * self.fRec16[0] * fTemp41);
																self.fVec2[0] = fTemp45;
																fTemp46 = ((fTemp45 != self.fVec2[1]) as i32) as u32 as F32;
																iTemp47 = (fTemp46 != 0.0) as i32;
															}
																if ((fTemp4 != 0.0) as i32) & iTemp47 != 0 {
																	iTemp48 = i32::wrapping_add(self.iRec9[0], 3888229);
																	iTemp49 = i32::wrapping_mul(1664525, iTemp48 ^ (iTemp48 >> 15));
																	fTemp50 = 4.656613e-10 * fTemp46 * (i32::wrapping_mul(1013904223, iTemp49 ^ (iTemp49 >> 13))) as F32;
																}
																	if (fTemp4 != 0.0) as i32 != 0 {
																		fTemp51 = ((self.fRec7[0] >= 0.0) as i32) as u32 as F32;
																		self.fRec6[0] = fTemp51 * (self.fRec6[1] + 0.35 * fTemp46 * fTemp50);
																		self.fRec5[0] = self.fConst8 * self.fRec6[0] + self.fConst5 * self.fRec5[1];
																	}
																		if ((fTemp4 != 0.0) as i32) & iTemp43 != 0 {
																			iTemp52 = i32::wrapping_add(self.iRec9[0], 3872391);
																			iTemp53 = i32::wrapping_mul(1664525, iTemp52 ^ (iTemp52 >> 15));
																			fTemp54 = 0.5 * fTemp42 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp53 ^ (iTemp53 >> 13))) as F32 + 0.5 + 1.0);
																		}
																			if (fTemp4 != 0.0) as i32 != 0 {
																				self.fRec17[0] = if iTemp10 != 0 {fTemp54} else {self.fRec17[1]};
																				fTemp55 = F32::min(1.0, F32::max(-1.0, self.fRec17[0] + self.fRec5[0] * fTemp8));
																				self.fRec18[0] = fSlow2 * fTemp1 + self.fConst1 * fTemp0 * self.fRec18[1];
																			}
																				if ((fTemp4 != 0.0) as i32) & iSlow49 != 0 {
																					fTemp56 = fSlow62 * F32::max(0.0, F32::min(1.0, fSlow45 * fTemp27 + F32::max(0.0, 0.083333336 * self.fRec18[0]) + fSlow46 * fTemp26));
																				}
																					if (fTemp4 != 0.0) as i32 != 0 {
																						fTemp57 = if iSlow47 != 0 {fSlow63 * fTemp56} else {self.fRec18[0]};
																						fTemp58 = fTemp57 * (((fSlow64 * fTemp57) > 0.0001) as i32) as u32 as F32;
																						fTemp59 = fSlow64 * fTemp58;
																						fTemp60 = fSlow65 * fTemp58;
																						fTemp61 = 2.6575425 * TextureDsp_faustpower2_f(fTemp41) * TextureDsp_faustpower2_f(1.0 - fTemp41) * fTemp55 * if (fTemp55 < 0.0) as i32 != 0 {fTemp60} else {fTemp59};
																						fTemp62 = F32::floor(fTemp61);
																						iTemp63 = (fTemp62) as i32;
																						fTemp64 = fTemp61 - fTemp62;
																						self.fRec22[0] = fSlow66 * fTemp1 + self.fConst1 * fTemp0 * self.fRec22[1];
																					}
																						if ((fTemp4 != 0.0) as i32) & iSlow44 != 0 {
																							fTemp65 = fSlow43 * (3.5 * F32::max(0.0, F32::min(1.0, fSlow40 * fTemp27 + F32::max(0.0, 0.2857143 * (self.fRec22[0] + -0.5)) + fSlow41 * fTemp26)) + 0.5);
																						}
																							if (fTemp4 != 0.0) as i32 != 0 {
																								fTemp66 = if iSlow42 != 0 {fSlow67 * fTemp65} else {self.fRec22[0]};
																								fTemp67 = F32::min(1.92 / fTemp66, 0.75);
																							}
																								if ((fTemp4 != 0.0) as i32) & iTemp47 != 0 {
																									iTemp68 = i32::wrapping_add(self.iRec9[0], 3880310);
																									iTemp69 = i32::wrapping_mul(1664525, iTemp68 ^ (iTemp68 >> 15));
																									fTemp70 = 4.656613e-10 * fTemp46 * (i32::wrapping_mul(1013904223, iTemp69 ^ (iTemp69 >> 13))) as F32;
																								}
																									if (fTemp4 != 0.0) as i32 != 0 {
																										self.fRec24[0] = fTemp51 * (self.fRec24[1] + 0.35 * fTemp46 * fTemp70);
																										self.fRec23[0] = self.fConst8 * self.fRec24[0] + self.fConst5 * self.fRec23[1];
																										fTemp71 = self.fRec23[0] * fTemp8 * fTemp67;
																										fTemp72 = F32::floor(fTemp71);
																										iTemp73 = (fTemp72) as i32;
																										fTemp74 = fTemp71 - fTemp72;
																									}
																										if ((fTemp4 != 0.0) as i32) & iTemp43 != 0 {
																											fTemp75 = 1.442695 * fTemp15;
																											fTemp76 = F32::floor(fTemp75);
																											iTemp77 = (fTemp76) as i32;
																											fTemp78 = fTemp75 - fTemp76;
																											fTemp79 = F32::round((fTemp78 * (fTemp78 * (fTemp78 * (fTemp78 * (0.0013333 * fTemp78 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp77))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp77))) as F32) * fTemp42;
																										}
																											if (fTemp4 != 0.0) as i32 != 0 {
																												self.fRec25[0] = if iTemp10 != 0 {fTemp79} else {self.fRec25[1]};
																												fTemp80 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec25[0] * (fTemp74 * (fTemp74 * (fTemp74 * (fTemp74 * (0.0013333 * fTemp74 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp73))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp73))) as F32));
																												fTemp81 = TextureDsp_faustpower2_f(fTemp80);
																												fTemp82 = self.fConst11 * fTemp81 * (self.fConst10 * fTemp81 + -444.1322) + 105.0;
																												fTemp83 = fTemp80 * (105.0 - self.fConst12 * fTemp81);
																												fTemp84 = 0.6666667 / fTemp66;
																												fTemp85 = self.fConst13 * (fTemp83 * (fTemp84 + self.fConst13 * (fTemp83 / fTemp82)) / fTemp82) + 1.0;
																												iTemp86 = ((self.fRec28[1] >= 1.0) as i32) | iTemp6;
																												iTemp87 = (self.fRec28[1] < 0.0) as i32;
																												iTemp88 = i32::wrapping_add(self.iRec9[0], 3072572);
																												iTemp89 = i32::wrapping_mul(1664525, iTemp88 ^ (iTemp88 >> 15));
																												fTemp90 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp89 ^ (iTemp89 >> 13))) as F32 + 0.5);
																												fTemp91 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp90) + -8.228819)));
																												iTemp92 = i32::wrapping_add(self.iRec9[0], 3080491);
																												iTemp93 = i32::wrapping_mul(1664525, iTemp92 ^ (iTemp92 >> 15));
																												fTemp94 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp93 ^ (iTemp93 >> 13))) as F32;
																												fTemp95 = F32::floor(fTemp94 - fTemp91);
																												iTemp96 = (fTemp95) as i32;
																												fTemp97 = fTemp94 - (fTemp91 + fTemp95);
																												fTemp98 = F32::max(0.005, 0.4 * (fTemp29 * (fTemp97 * (fTemp97 * (fTemp97 * (fTemp97 * (0.0013333 * fTemp97 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp96))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp96))) as F32));
																												self.fRec29[0] = if iTemp87 != 0 {fTemp98} else {self.fRec29[1]};
																												fTemp99 = (iTemp86) as F32;
																											}
																												if ((fTemp4 != 0.0) as i32) & ((fTemp99 != 0.0) as i32) != 0 {
																													iTemp100 = i32::wrapping_add(self.iRec9[0], 3088410);
																													iTemp101 = i32::wrapping_mul(1664525, iTemp100 ^ (iTemp100 >> 15));
																													fTemp102 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp101 ^ (iTemp101 >> 13))) as F32;
																													fTemp103 = fTemp102 + 1.5;
																													fTemp104 = 0.5 - fTemp102;
																													fTemp105 = TextureDsp_faustpower2_f(fTemp104) / TextureDsp_faustpower2_f(fTemp103);
																												}
																													if (fTemp4 != 0.0) as i32 != 0 {
																														self.fRec28[0] = if iTemp86 != 0 {-(2.0 * (fTemp104 * (15.0 - 4.0 * fTemp105) * fTemp99 / (fTemp103 * (15.0 - 9.0 * fTemp105))))} else {self.fRec28[1] + self.fConst7 * (if iTemp87 != 0 {fTemp33} else {1.0} / F32::max(self.fRec29[0], 0.005))};
																														fTemp106 = F32::min(1.0, F32::max(0.0, self.fRec28[0]));
																														fTemp107 = (iTemp87) as F32;
																														iTemp108 = (fTemp107 != 0.0) as i32;
																													}
																														if ((fTemp4 != 0.0) as i32) & iTemp108 != 0 {
																															fTemp109 = fTemp98 * fTemp107;
																														}
																															if (fTemp4 != 0.0) as i32 != 0 {
																																self.fRec30[0] = if iTemp87 != 0 {fTemp109} else {self.fRec30[1]};
																																fTemp110 = F32::floor(41.666668 * self.fRec30[0] * fTemp106);
																																self.fVec3[0] = fTemp110;
																																fTemp111 = ((fTemp110 != self.fVec3[1]) as i32) as u32 as F32;
																																iTemp112 = (fTemp111 != 0.0) as i32;
																															}
																																if ((fTemp4 != 0.0) as i32) & iTemp112 != 0 {
																																	iTemp113 = i32::wrapping_add(self.iRec9[0], 3120086);
																																	iTemp114 = i32::wrapping_mul(1664525, iTemp113 ^ (iTemp113 >> 15));
																																	fTemp115 = 4.656613e-10 * fTemp111 * (i32::wrapping_mul(1013904223, iTemp114 ^ (iTemp114 >> 13))) as F32;
																																}
																																	if (fTemp4 != 0.0) as i32 != 0 {
																																		fTemp116 = ((self.fRec28[0] >= 0.0) as i32) as u32 as F32;
																																		self.fRec27[0] = fTemp116 * (self.fRec27[1] + 0.35 * fTemp111 * fTemp115);
																																		self.fRec26[0] = self.fConst8 * self.fRec27[0] + self.fConst5 * self.fRec26[1];
																																	}
																																		if ((fTemp4 != 0.0) as i32) & iTemp108 != 0 {
																																			iTemp117 = i32::wrapping_add(self.iRec9[0], 3104248);
																																			iTemp118 = i32::wrapping_mul(1664525, iTemp117 ^ (iTemp117 >> 15));
																																			fTemp119 = 0.5 * fTemp107 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp118 ^ (iTemp118 >> 13))) as F32 + 0.5 + 1.0);
																																		}
																																			if (fTemp4 != 0.0) as i32 != 0 {
																																				self.fRec31[0] = if iTemp87 != 0 {fTemp119} else {self.fRec31[1]};
																																				fTemp120 = F32::min(1.0, F32::max(-1.0, self.fRec31[0] + self.fRec26[0] * fTemp8));
																																				fTemp121 = 2.6575425 * TextureDsp_faustpower2_f(fTemp106) * TextureDsp_faustpower2_f(1.0 - fTemp106) * fTemp120 * if (fTemp120 < 0.0) as i32 != 0 {fTemp60} else {fTemp59};
																																				fTemp122 = F32::floor(fTemp121);
																																				iTemp123 = (fTemp122) as i32;
																																				fTemp124 = fTemp121 - fTemp122;
																																			}
																																				if ((fTemp4 != 0.0) as i32) & iTemp112 != 0 {
																																					iTemp125 = i32::wrapping_add(self.iRec9[0], 3112167);
																																					iTemp126 = i32::wrapping_mul(1664525, iTemp125 ^ (iTemp125 >> 15));
																																					fTemp127 = 4.656613e-10 * fTemp111 * (i32::wrapping_mul(1013904223, iTemp126 ^ (iTemp126 >> 13))) as F32;
																																				}
																																					if (fTemp4 != 0.0) as i32 != 0 {
																																						self.fRec36[0] = fTemp116 * (self.fRec36[1] + 0.35 * fTemp111 * fTemp127);
																																						self.fRec35[0] = self.fConst8 * self.fRec36[0] + self.fConst5 * self.fRec35[1];
																																						fTemp128 = self.fRec35[0] * fTemp8 * fTemp67;
																																						fTemp129 = F32::floor(fTemp128);
																																						iTemp130 = (fTemp129) as i32;
																																						fTemp131 = fTemp128 - fTemp129;
																																					}
																																						if ((fTemp4 != 0.0) as i32) & iTemp108 != 0 {
																																							fTemp132 = 1.442695 * fTemp90;
																																							fTemp133 = F32::floor(fTemp132);
																																							iTemp134 = (fTemp133) as i32;
																																							fTemp135 = fTemp132 - fTemp133;
																																							fTemp136 = F32::round((fTemp135 * (fTemp135 * (fTemp135 * (fTemp135 * (0.0013333 * fTemp135 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp134))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp134))) as F32) * fTemp107;
																																						}
																																							if (fTemp4 != 0.0) as i32 != 0 {
																																								self.fRec37[0] = if iTemp87 != 0 {fTemp136} else {self.fRec37[1]};
																																								fTemp137 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec37[0] * (fTemp131 * (fTemp131 * (fTemp131 * (fTemp131 * (0.0013333 * fTemp131 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp130))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp130))) as F32));
																																								fTemp138 = TextureDsp_faustpower2_f(fTemp137);
																																								fTemp139 = self.fConst11 * fTemp138 * (self.fConst10 * fTemp138 + -444.1322) + 105.0;
																																								fTemp140 = fTemp137 * (105.0 - self.fConst12 * fTemp138);
																																								fTemp141 = self.fConst13 * (fTemp140 * (fTemp84 + self.fConst13 * (fTemp140 / fTemp139)) / fTemp139) + 1.0;
																																								iTemp142 = ((self.fRec40[1] >= 1.0) as i32) | iTemp6;
																																								iTemp143 = (self.fRec40[1] < 0.0) as i32;
																																								iTemp144 = i32::wrapping_add(self.iRec9[0], 2304429);
																																								iTemp145 = i32::wrapping_mul(1664525, iTemp144 ^ (iTemp144 >> 15));
																																								fTemp146 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp145 ^ (iTemp145 >> 13))) as F32 + 0.5);
																																								fTemp147 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp146) + -8.228819)));
																																								iTemp148 = i32::wrapping_add(self.iRec9[0], 2312348);
																																								iTemp149 = i32::wrapping_mul(1664525, iTemp148 ^ (iTemp148 >> 15));
																																								fTemp150 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp149 ^ (iTemp149 >> 13))) as F32;
																																								fTemp151 = F32::floor(fTemp150 - fTemp147);
																																								iTemp152 = (fTemp151) as i32;
																																								fTemp153 = fTemp150 - (fTemp147 + fTemp151);
																																								fTemp154 = F32::max(0.005, 0.4 * (fTemp29 * (fTemp153 * (fTemp153 * (fTemp153 * (fTemp153 * (0.0013333 * fTemp153 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp152))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp152))) as F32));
																																								self.fRec41[0] = if iTemp143 != 0 {fTemp154} else {self.fRec41[1]};
																																								fTemp155 = (iTemp142) as F32;
																																							}
																																								if ((fTemp4 != 0.0) as i32) & ((fTemp155 != 0.0) as i32) != 0 {
																																									iTemp156 = i32::wrapping_add(self.iRec9[0], 2320267);
																																									iTemp157 = i32::wrapping_mul(1664525, iTemp156 ^ (iTemp156 >> 15));
																																									fTemp158 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp157 ^ (iTemp157 >> 13))) as F32;
																																									fTemp159 = fTemp158 + 1.5;
																																									fTemp160 = 0.5 - fTemp158;
																																									fTemp161 = TextureDsp_faustpower2_f(fTemp160) / TextureDsp_faustpower2_f(fTemp159);
																																								}
																																									if (fTemp4 != 0.0) as i32 != 0 {
																																										self.fRec40[0] = if iTemp142 != 0 {-(2.0 * (fTemp160 * (15.0 - 4.0 * fTemp161) * fTemp155 / (fTemp159 * (15.0 - 9.0 * fTemp161))))} else {self.fRec40[1] + self.fConst7 * (if iTemp143 != 0 {fTemp33} else {1.0} / F32::max(self.fRec41[0], 0.005))};
																																										fTemp162 = F32::min(1.0, F32::max(0.0, self.fRec40[0]));
																																										fTemp163 = (iTemp143) as F32;
																																										iTemp164 = (fTemp163 != 0.0) as i32;
																																									}
																																										if ((fTemp4 != 0.0) as i32) & iTemp164 != 0 {
																																											fTemp165 = fTemp154 * fTemp163;
																																										}
																																											if (fTemp4 != 0.0) as i32 != 0 {
																																												self.fRec42[0] = if iTemp143 != 0 {fTemp165} else {self.fRec42[1]};
																																												fTemp166 = F32::floor(41.666668 * self.fRec42[0] * fTemp162);
																																												self.fVec4[0] = fTemp166;
																																												fTemp167 = ((fTemp166 != self.fVec4[1]) as i32) as u32 as F32;
																																												iTemp168 = (fTemp167 != 0.0) as i32;
																																											}
																																												if ((fTemp4 != 0.0) as i32) & iTemp168 != 0 {
																																													iTemp169 = i32::wrapping_add(self.iRec9[0], 2351943);
																																													iTemp170 = i32::wrapping_mul(1664525, iTemp169 ^ (iTemp169 >> 15));
																																													fTemp171 = 4.656613e-10 * fTemp167 * (i32::wrapping_mul(1013904223, iTemp170 ^ (iTemp170 >> 13))) as F32;
																																												}
																																													if (fTemp4 != 0.0) as i32 != 0 {
																																														fTemp172 = ((self.fRec40[0] >= 0.0) as i32) as u32 as F32;
																																														self.fRec39[0] = fTemp172 * (self.fRec39[1] + 0.35 * fTemp167 * fTemp171);
																																														self.fRec38[0] = self.fConst8 * self.fRec39[0] + self.fConst5 * self.fRec38[1];
																																													}
																																														if ((fTemp4 != 0.0) as i32) & iTemp164 != 0 {
																																															iTemp173 = i32::wrapping_add(self.iRec9[0], 2336105);
																																															iTemp174 = i32::wrapping_mul(1664525, iTemp173 ^ (iTemp173 >> 15));
																																															fTemp175 = 0.5 * fTemp163 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp174 ^ (iTemp174 >> 13))) as F32 + 0.5 + 1.0);
																																														}
																																															if (fTemp4 != 0.0) as i32 != 0 {
																																																self.fRec43[0] = if iTemp143 != 0 {fTemp175} else {self.fRec43[1]};
																																																fTemp176 = F32::min(1.0, F32::max(-1.0, self.fRec43[0] + self.fRec38[0] * fTemp8));
																																																fTemp177 = 2.6575425 * TextureDsp_faustpower2_f(fTemp162) * TextureDsp_faustpower2_f(1.0 - fTemp162) * fTemp176 * if (fTemp176 < 0.0) as i32 != 0 {fTemp60} else {fTemp59};
																																																fTemp178 = F32::floor(fTemp177);
																																																iTemp179 = (fTemp178) as i32;
																																																fTemp180 = fTemp177 - fTemp178;
																																															}
																																																if ((fTemp4 != 0.0) as i32) & iTemp168 != 0 {
																																																	iTemp181 = i32::wrapping_add(self.iRec9[0], 2344024);
																																																	iTemp182 = i32::wrapping_mul(1664525, iTemp181 ^ (iTemp181 >> 15));
																																																	fTemp183 = 4.656613e-10 * fTemp167 * (i32::wrapping_mul(1013904223, iTemp182 ^ (iTemp182 >> 13))) as F32;
																																																}
																																																	if (fTemp4 != 0.0) as i32 != 0 {
																																																		self.fRec48[0] = fTemp172 * (self.fRec48[1] + 0.35 * fTemp167 * fTemp183);
																																																		self.fRec47[0] = self.fConst8 * self.fRec48[0] + self.fConst5 * self.fRec47[1];
																																																		fTemp184 = self.fRec47[0] * fTemp8 * fTemp67;
																																																		fTemp185 = F32::floor(fTemp184);
																																																		iTemp186 = (fTemp185) as i32;
																																																		fTemp187 = fTemp184 - fTemp185;
																																																	}
																																																		if ((fTemp4 != 0.0) as i32) & iTemp164 != 0 {
																																																			fTemp188 = 1.442695 * fTemp146;
																																																			fTemp189 = F32::floor(fTemp188);
																																																			iTemp190 = (fTemp189) as i32;
																																																			fTemp191 = fTemp188 - fTemp189;
																																																			fTemp192 = F32::round((fTemp191 * (fTemp191 * (fTemp191 * (fTemp191 * (0.0013333 * fTemp191 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp190))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp190))) as F32) * fTemp163;
																																																		}
																																																			if (fTemp4 != 0.0) as i32 != 0 {
																																																				self.fRec49[0] = if iTemp143 != 0 {fTemp192} else {self.fRec49[1]};
																																																				fTemp193 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec49[0] * (fTemp187 * (fTemp187 * (fTemp187 * (fTemp187 * (0.0013333 * fTemp187 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp186))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp186))) as F32));
																																																				fTemp194 = TextureDsp_faustpower2_f(fTemp193);
																																																				fTemp195 = self.fConst11 * fTemp194 * (self.fConst10 * fTemp194 + -444.1322) + 105.0;
																																																				fTemp196 = fTemp193 * (105.0 - self.fConst12 * fTemp194);
																																																				fTemp197 = self.fConst13 * (fTemp196 * (fTemp84 + self.fConst13 * (fTemp196 / fTemp195)) / fTemp195) + 1.0;
																																																				iTemp198 = ((self.fRec52[1] >= 1.0) as i32) | iTemp6;
																																																				iTemp199 = (self.fRec52[1] < 0.0) as i32;
																																																				iTemp200 = i32::wrapping_add(self.iRec9[0], 1536286);
																																																				iTemp201 = i32::wrapping_mul(1664525, iTemp200 ^ (iTemp200 >> 15));
																																																				fTemp202 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp201 ^ (iTemp201 >> 13))) as F32 + 0.5);
																																																				fTemp203 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp202) + -8.228819)));
																																																				iTemp204 = i32::wrapping_add(self.iRec9[0], 1544205);
																																																				iTemp205 = i32::wrapping_mul(1664525, iTemp204 ^ (iTemp204 >> 15));
																																																				fTemp206 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp205 ^ (iTemp205 >> 13))) as F32;
																																																				fTemp207 = F32::floor(fTemp206 - fTemp203);
																																																				iTemp208 = (fTemp207) as i32;
																																																				fTemp209 = fTemp206 - (fTemp203 + fTemp207);
																																																				fTemp210 = F32::max(0.005, 0.4 * (fTemp29 * (fTemp209 * (fTemp209 * (fTemp209 * (fTemp209 * (0.0013333 * fTemp209 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp208))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp208))) as F32));
																																																				self.fRec53[0] = if iTemp199 != 0 {fTemp210} else {self.fRec53[1]};
																																																				fTemp211 = (iTemp198) as F32;
																																																			}
																																																				if ((fTemp4 != 0.0) as i32) & ((fTemp211 != 0.0) as i32) != 0 {
																																																					iTemp212 = i32::wrapping_add(self.iRec9[0], 1552124);
																																																					iTemp213 = i32::wrapping_mul(1664525, iTemp212 ^ (iTemp212 >> 15));
																																																					fTemp214 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp213 ^ (iTemp213 >> 13))) as F32;
																																																					fTemp215 = fTemp214 + 1.5;
																																																					fTemp216 = 0.5 - fTemp214;
																																																					fTemp217 = TextureDsp_faustpower2_f(fTemp216) / TextureDsp_faustpower2_f(fTemp215);
																																																				}
																																																					if (fTemp4 != 0.0) as i32 != 0 {
																																																						self.fRec52[0] = if iTemp198 != 0 {-(2.0 * (fTemp216 * (15.0 - 4.0 * fTemp217) * fTemp211 / (fTemp215 * (15.0 - 9.0 * fTemp217))))} else {self.fRec52[1] + self.fConst7 * (if iTemp199 != 0 {fTemp33} else {1.0} / F32::max(self.fRec53[0], 0.005))};
																																																						fTemp218 = F32::min(1.0, F32::max(0.0, self.fRec52[0]));
																																																						fTemp219 = (iTemp199) as F32;
																																																						iTemp220 = (fTemp219 != 0.0) as i32;
																																																					}
																																																						if ((fTemp4 != 0.0) as i32) & iTemp220 != 0 {
																																																							fTemp221 = fTemp210 * fTemp219;
																																																						}
																																																							if (fTemp4 != 0.0) as i32 != 0 {
																																																								self.fRec54[0] = if iTemp199 != 0 {fTemp221} else {self.fRec54[1]};
																																																								fTemp222 = F32::floor(41.666668 * self.fRec54[0] * fTemp218);
																																																								self.fVec5[0] = fTemp222;
																																																								fTemp223 = ((fTemp222 != self.fVec5[1]) as i32) as u32 as F32;
																																																								iTemp224 = (fTemp223 != 0.0) as i32;
																																																							}
																																																								if ((fTemp4 != 0.0) as i32) & iTemp224 != 0 {
																																																									iTemp225 = i32::wrapping_add(self.iRec9[0], 1583800);
																																																									iTemp226 = i32::wrapping_mul(1664525, iTemp225 ^ (iTemp225 >> 15));
																																																									fTemp227 = 4.656613e-10 * fTemp223 * (i32::wrapping_mul(1013904223, iTemp226 ^ (iTemp226 >> 13))) as F32;
																																																								}
																																																									if (fTemp4 != 0.0) as i32 != 0 {
																																																										fTemp228 = ((self.fRec52[0] >= 0.0) as i32) as u32 as F32;
																																																										self.fRec51[0] = fTemp228 * (self.fRec51[1] + 0.35 * fTemp223 * fTemp227);
																																																										self.fRec50[0] = self.fConst8 * self.fRec51[0] + self.fConst5 * self.fRec50[1];
																																																									}
																																																										if ((fTemp4 != 0.0) as i32) & iTemp220 != 0 {
																																																											iTemp229 = i32::wrapping_add(self.iRec9[0], 1567962);
																																																											iTemp230 = i32::wrapping_mul(1664525, iTemp229 ^ (iTemp229 >> 15));
																																																											fTemp231 = 0.5 * fTemp219 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp230 ^ (iTemp230 >> 13))) as F32 + 0.5 + 1.0);
																																																										}
																																																											if (fTemp4 != 0.0) as i32 != 0 {
																																																												self.fRec55[0] = if iTemp199 != 0 {fTemp231} else {self.fRec55[1]};
																																																												fTemp232 = F32::min(1.0, F32::max(-1.0, self.fRec55[0] + self.fRec50[0] * fTemp8));
																																																												fTemp233 = 2.6575425 * TextureDsp_faustpower2_f(fTemp218) * TextureDsp_faustpower2_f(1.0 - fTemp218) * fTemp232 * if (fTemp232 < 0.0) as i32 != 0 {fTemp60} else {fTemp59};
																																																												fTemp234 = F32::floor(fTemp233);
																																																												iTemp235 = (fTemp234) as i32;
																																																												fTemp236 = fTemp233 - fTemp234;
																																																											}
																																																												if ((fTemp4 != 0.0) as i32) & iTemp224 != 0 {
																																																													iTemp237 = i32::wrapping_add(self.iRec9[0], 1575881);
																																																													iTemp238 = i32::wrapping_mul(1664525, iTemp237 ^ (iTemp237 >> 15));
																																																													fTemp239 = 4.656613e-10 * fTemp223 * (i32::wrapping_mul(1013904223, iTemp238 ^ (iTemp238 >> 13))) as F32;
																																																												}
																																																													if (fTemp4 != 0.0) as i32 != 0 {
																																																														self.fRec60[0] = fTemp228 * (self.fRec60[1] + 0.35 * fTemp223 * fTemp239);
																																																														self.fRec59[0] = self.fConst8 * self.fRec60[0] + self.fConst5 * self.fRec59[1];
																																																														fTemp240 = self.fRec59[0] * fTemp8 * fTemp67;
																																																														fTemp241 = F32::floor(fTemp240);
																																																														iTemp242 = (fTemp241) as i32;
																																																														fTemp243 = fTemp240 - fTemp241;
																																																													}
																																																														if ((fTemp4 != 0.0) as i32) & iTemp220 != 0 {
																																																															fTemp244 = 1.442695 * fTemp202;
																																																															fTemp245 = F32::floor(fTemp244);
																																																															iTemp246 = (fTemp245) as i32;
																																																															fTemp247 = fTemp244 - fTemp245;
																																																															fTemp248 = F32::round((fTemp247 * (fTemp247 * (fTemp247 * (fTemp247 * (0.0013333 * fTemp247 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp246))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp246))) as F32) * fTemp219;
																																																														}
																																																															if (fTemp4 != 0.0) as i32 != 0 {
																																																																self.fRec61[0] = if iTemp199 != 0 {fTemp248} else {self.fRec61[1]};
																																																																fTemp249 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec61[0] * (fTemp243 * (fTemp243 * (fTemp243 * (fTemp243 * (0.0013333 * fTemp243 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp242))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp242))) as F32));
																																																																fTemp250 = TextureDsp_faustpower2_f(fTemp249);
																																																																fTemp251 = self.fConst11 * fTemp250 * (self.fConst10 * fTemp250 + -444.1322) + 105.0;
																																																																fTemp252 = fTemp249 * (105.0 - self.fConst12 * fTemp250);
																																																																fTemp253 = self.fConst13 * (fTemp252 * (fTemp84 + self.fConst13 * (fTemp252 / fTemp251)) / fTemp251) + 1.0;
																																																																iTemp254 = ((self.fRec64[1] >= 1.0) as i32) | iTemp6;
																																																																iTemp255 = (self.fRec64[1] < 0.0) as i32;
																																																																iTemp256 = i32::wrapping_add(self.iRec9[0], 768143);
																																																																iTemp257 = i32::wrapping_mul(1664525, iTemp256 ^ (iTemp256 >> 15));
																																																																fTemp258 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp257 ^ (iTemp257 >> 13))) as F32 + 0.5);
																																																																fTemp259 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp258) + -8.228819)));
																																																																iTemp260 = i32::wrapping_add(self.iRec9[0], 776062);
																																																																iTemp261 = i32::wrapping_mul(1664525, iTemp260 ^ (iTemp260 >> 15));
																																																																fTemp262 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp261 ^ (iTemp261 >> 13))) as F32;
																																																																fTemp263 = F32::floor(fTemp262 - fTemp259);
																																																																iTemp264 = (fTemp263) as i32;
																																																																fTemp265 = fTemp262 - (fTemp259 + fTemp263);
																																																																fTemp266 = F32::max(0.005, 0.4 * (fTemp29 * (fTemp265 * (fTemp265 * (fTemp265 * (fTemp265 * (0.0013333 * fTemp265 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp264))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp264))) as F32));
																																																																self.fRec65[0] = if iTemp255 != 0 {fTemp266} else {self.fRec65[1]};
																																																																fTemp267 = (iTemp254) as F32;
																																																															}
																																																																if ((fTemp4 != 0.0) as i32) & ((fTemp267 != 0.0) as i32) != 0 {
																																																																	iTemp268 = i32::wrapping_add(self.iRec9[0], 783981);
																																																																	iTemp269 = i32::wrapping_mul(1664525, iTemp268 ^ (iTemp268 >> 15));
																																																																	fTemp270 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp269 ^ (iTemp269 >> 13))) as F32;
																																																																	fTemp271 = fTemp270 + 1.5;
																																																																	fTemp272 = 0.5 - fTemp270;
																																																																	fTemp273 = TextureDsp_faustpower2_f(fTemp272) / TextureDsp_faustpower2_f(fTemp271);
																																																																}
																																																																	if (fTemp4 != 0.0) as i32 != 0 {
																																																																		self.fRec64[0] = if iTemp254 != 0 {-(2.0 * (fTemp272 * (15.0 - 4.0 * fTemp273) * fTemp267 / (fTemp271 * (15.0 - 9.0 * fTemp273))))} else {self.fRec64[1] + self.fConst7 * (if iTemp255 != 0 {fTemp33} else {1.0} / F32::max(self.fRec65[0], 0.005))};
																																																																		fTemp274 = F32::min(1.0, F32::max(0.0, self.fRec64[0]));
																																																																		fTemp275 = (iTemp255) as F32;
																																																																		iTemp276 = (fTemp275 != 0.0) as i32;
																																																																	}
																																																																		if ((fTemp4 != 0.0) as i32) & iTemp276 != 0 {
																																																																			fTemp277 = fTemp266 * fTemp275;
																																																																		}
																																																																			if (fTemp4 != 0.0) as i32 != 0 {
																																																																				self.fRec66[0] = if iTemp255 != 0 {fTemp277} else {self.fRec66[1]};
																																																																				fTemp278 = F32::floor(41.666668 * self.fRec66[0] * fTemp274);
																																																																				self.fVec6[0] = fTemp278;
																																																																				fTemp279 = ((fTemp278 != self.fVec6[1]) as i32) as u32 as F32;
																																																																				iTemp280 = (fTemp279 != 0.0) as i32;
																																																																			}
																																																																				if ((fTemp4 != 0.0) as i32) & iTemp280 != 0 {
																																																																					iTemp281 = i32::wrapping_add(self.iRec9[0], 815657);
																																																																					iTemp282 = i32::wrapping_mul(1664525, iTemp281 ^ (iTemp281 >> 15));
																																																																					fTemp283 = 4.656613e-10 * fTemp279 * (i32::wrapping_mul(1013904223, iTemp282 ^ (iTemp282 >> 13))) as F32;
																																																																				}
																																																																					if (fTemp4 != 0.0) as i32 != 0 {
																																																																						fTemp284 = ((self.fRec64[0] >= 0.0) as i32) as u32 as F32;
																																																																						self.fRec63[0] = fTemp284 * (self.fRec63[1] + 0.35 * fTemp279 * fTemp283);
																																																																						self.fRec62[0] = self.fConst8 * self.fRec63[0] + self.fConst5 * self.fRec62[1];
																																																																					}
																																																																						if ((fTemp4 != 0.0) as i32) & iTemp276 != 0 {
																																																																							iTemp285 = i32::wrapping_add(self.iRec9[0], 799819);
																																																																							iTemp286 = i32::wrapping_mul(1664525, iTemp285 ^ (iTemp285 >> 15));
																																																																							fTemp287 = 0.5 * fTemp275 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp286 ^ (iTemp286 >> 13))) as F32 + 0.5 + 1.0);
																																																																						}
																																																																							if (fTemp4 != 0.0) as i32 != 0 {
																																																																								self.fRec67[0] = if iTemp255 != 0 {fTemp287} else {self.fRec67[1]};
																																																																								fTemp288 = F32::min(1.0, F32::max(-1.0, self.fRec67[0] + self.fRec62[0] * fTemp8));
																																																																								fTemp289 = 2.6575425 * TextureDsp_faustpower2_f(fTemp274) * TextureDsp_faustpower2_f(1.0 - fTemp274) * fTemp288 * if (fTemp288 < 0.0) as i32 != 0 {fTemp60} else {fTemp59};
																																																																								fTemp290 = F32::floor(fTemp289);
																																																																								iTemp291 = (fTemp290) as i32;
																																																																								fTemp292 = fTemp289 - fTemp290;
																																																																							}
																																																																								if ((fTemp4 != 0.0) as i32) & iTemp280 != 0 {
																																																																									iTemp293 = i32::wrapping_add(self.iRec9[0], 807738);
																																																																									iTemp294 = i32::wrapping_mul(1664525, iTemp293 ^ (iTemp293 >> 15));
																																																																									fTemp295 = 4.656613e-10 * fTemp279 * (i32::wrapping_mul(1013904223, iTemp294 ^ (iTemp294 >> 13))) as F32;
																																																																								}
																																																																									if (fTemp4 != 0.0) as i32 != 0 {
																																																																										self.fRec72[0] = fTemp284 * (self.fRec72[1] + 0.35 * fTemp279 * fTemp295);
																																																																										self.fRec71[0] = self.fConst8 * self.fRec72[0] + self.fConst5 * self.fRec71[1];
																																																																										fTemp296 = self.fRec71[0] * fTemp8 * fTemp67;
																																																																										fTemp297 = F32::floor(fTemp296);
																																																																										iTemp298 = (fTemp297) as i32;
																																																																										fTemp299 = fTemp296 - fTemp297;
																																																																									}
																																																																										if ((fTemp4 != 0.0) as i32) & iTemp276 != 0 {
																																																																											fTemp300 = 1.442695 * fTemp258;
																																																																											fTemp301 = F32::floor(fTemp300);
																																																																											iTemp302 = (fTemp301) as i32;
																																																																											fTemp303 = fTemp300 - fTemp301;
																																																																											fTemp304 = F32::round((fTemp303 * (fTemp303 * (fTemp303 * (fTemp303 * (0.0013333 * fTemp303 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp302))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp302))) as F32) * fTemp275;
																																																																										}
																																																																											if (fTemp4 != 0.0) as i32 != 0 {
																																																																												self.fRec73[0] = if iTemp255 != 0 {fTemp304} else {self.fRec73[1]};
																																																																												fTemp305 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec73[0] * (fTemp299 * (fTemp299 * (fTemp299 * (fTemp299 * (0.0013333 * fTemp299 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp298))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp298))) as F32));
																																																																												fTemp306 = TextureDsp_faustpower2_f(fTemp305);
																																																																												fTemp307 = self.fConst11 * fTemp306 * (self.fConst10 * fTemp306 + -444.1322) + 105.0;
																																																																												fTemp308 = fTemp305 * (105.0 - self.fConst12 * fTemp306);
																																																																												fTemp309 = self.fConst13 * (fTemp308 * (fTemp84 + self.fConst13 * (fTemp308 / fTemp307)) / fTemp307) + 1.0;
																																																																												iTemp310 = ((self.fRec76[1] >= 1.0) as i32) | iTemp6;
																																																																												iTemp311 = (self.fRec76[1] < 0.0) as i32;
																																																																												iTemp312 = i32::wrapping_mul(1664525, self.iRec9[0] ^ (self.iRec9[0] >> 15));
																																																																												fTemp313 = fTemp14 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp312 ^ (iTemp312 >> 13))) as F32 + 0.5);
																																																																												fTemp314 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (fTemp16 + fTemp313) + -8.228819)));
																																																																												iTemp315 = i32::wrapping_add(self.iRec9[0], 7919);
																																																																												iTemp316 = i32::wrapping_mul(1664525, iTemp315 ^ (iTemp315 >> 15));
																																																																												fTemp317 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp316 ^ (iTemp316 >> 13))) as F32;
																																																																												fTemp318 = F32::floor(fTemp317 - fTemp314);
																																																																												iTemp319 = (fTemp318) as i32;
																																																																												fTemp320 = fTemp317 - (fTemp314 + fTemp318);
																																																																												fTemp321 = F32::max(0.005, 0.4 * (fTemp29 * (fTemp320 * (fTemp320 * (fTemp320 * (fTemp320 * (0.0013333 * fTemp320 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp319))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp319))) as F32));
																																																																												self.fRec77[0] = if iTemp311 != 0 {fTemp321} else {self.fRec77[1]};
																																																																												fTemp322 = (iTemp310) as F32;
																																																																											}
																																																																												if ((fTemp4 != 0.0) as i32) & ((fTemp322 != 0.0) as i32) != 0 {
																																																																													iTemp323 = i32::wrapping_add(self.iRec9[0], 15838);
																																																																													iTemp324 = i32::wrapping_mul(1664525, iTemp323 ^ (iTemp323 >> 15));
																																																																													fTemp325 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp324 ^ (iTemp324 >> 13))) as F32;
																																																																													fTemp326 = fTemp325 + 1.5;
																																																																													fTemp327 = 0.5 - fTemp325;
																																																																													fTemp328 = TextureDsp_faustpower2_f(fTemp327) / TextureDsp_faustpower2_f(fTemp326);
																																																																												}
																																																																													if (fTemp4 != 0.0) as i32 != 0 {
																																																																														self.fRec76[0] = if iTemp310 != 0 {-(2.0 * (fTemp327 * (15.0 - 4.0 * fTemp328) * fTemp322 / (fTemp326 * (15.0 - 9.0 * fTemp328))))} else {self.fRec76[1] + self.fConst7 * (if iTemp311 != 0 {fTemp33} else {1.0} / F32::max(self.fRec77[0], 0.005))};
																																																																														fTemp329 = F32::min(1.0, F32::max(0.0, self.fRec76[0]));
																																																																														fTemp330 = (iTemp311) as F32;
																																																																														iTemp331 = (fTemp330 != 0.0) as i32;
																																																																													}
																																																																														if ((fTemp4 != 0.0) as i32) & iTemp331 != 0 {
																																																																															fTemp332 = fTemp321 * fTemp330;
																																																																														}
																																																																															if (fTemp4 != 0.0) as i32 != 0 {
																																																																																self.fRec78[0] = if iTemp311 != 0 {fTemp332} else {self.fRec78[1]};
																																																																																fTemp333 = F32::floor(41.666668 * self.fRec78[0] * fTemp329);
																																																																																self.fVec7[0] = fTemp333;
																																																																																fTemp334 = ((fTemp333 != self.fVec7[1]) as i32) as u32 as F32;
																																																																																iTemp335 = (fTemp334 != 0.0) as i32;
																																																																															}
																																																																																if ((fTemp4 != 0.0) as i32) & iTemp335 != 0 {
																																																																																	iTemp336 = i32::wrapping_add(self.iRec9[0], 47514);
																																																																																	iTemp337 = i32::wrapping_mul(1664525, iTemp336 ^ (iTemp336 >> 15));
																																																																																	fTemp338 = 4.656613e-10 * fTemp334 * (i32::wrapping_mul(1013904223, iTemp337 ^ (iTemp337 >> 13))) as F32;
																																																																																}
																																																																																	if (fTemp4 != 0.0) as i32 != 0 {
																																																																																		fTemp339 = ((self.fRec76[0] >= 0.0) as i32) as u32 as F32;
																																																																																		self.fRec75[0] = fTemp339 * (self.fRec75[1] + 0.35 * fTemp334 * fTemp338);
																																																																																		self.fRec74[0] = self.fConst8 * self.fRec75[0] + self.fConst5 * self.fRec74[1];
																																																																																	}
																																																																																		if ((fTemp4 != 0.0) as i32) & iTemp331 != 0 {
																																																																																			iTemp340 = i32::wrapping_add(self.iRec9[0], 31676);
																																																																																			iTemp341 = i32::wrapping_mul(1664525, iTemp340 ^ (iTemp340 >> 15));
																																																																																			fTemp342 = 0.5 * fTemp330 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp341 ^ (iTemp341 >> 13))) as F32 + 0.5 + 1.0);
																																																																																		}
																																																																																			if (fTemp4 != 0.0) as i32 != 0 {
																																																																																				self.fRec79[0] = if iTemp311 != 0 {fTemp342} else {self.fRec79[1]};
																																																																																				fTemp343 = F32::min(1.0, F32::max(-1.0, self.fRec79[0] + self.fRec74[0] * fTemp8));
																																																																																				fTemp344 = 2.6575425 * TextureDsp_faustpower2_f(fTemp329) * TextureDsp_faustpower2_f(1.0 - fTemp329) * fTemp343 * if (fTemp343 < 0.0) as i32 != 0 {fTemp60} else {fTemp59};
																																																																																				fTemp345 = F32::floor(fTemp344);
																																																																																				iTemp346 = (fTemp345) as i32;
																																																																																				fTemp347 = fTemp344 - fTemp345;
																																																																																			}
																																																																																				if ((fTemp4 != 0.0) as i32) & iTemp335 != 0 {
																																																																																					iTemp348 = i32::wrapping_add(self.iRec9[0], 39595);
																																																																																					iTemp349 = i32::wrapping_mul(1664525, iTemp348 ^ (iTemp348 >> 15));
																																																																																					fTemp350 = 4.656613e-10 * fTemp334 * (i32::wrapping_mul(1013904223, iTemp349 ^ (iTemp349 >> 13))) as F32;
																																																																																				}
																																																																																					if (fTemp4 != 0.0) as i32 != 0 {
																																																																																						self.fRec84[0] = fTemp339 * (self.fRec84[1] + 0.35 * fTemp334 * fTemp350);
																																																																																						self.fRec83[0] = self.fConst8 * self.fRec84[0] + self.fConst5 * self.fRec83[1];
																																																																																						fTemp351 = self.fRec83[0] * fTemp8 * fTemp67;
																																																																																						fTemp352 = F32::floor(fTemp351);
																																																																																						iTemp353 = (fTemp352) as i32;
																																																																																						fTemp354 = fTemp351 - fTemp352;
																																																																																					}
																																																																																						if ((fTemp4 != 0.0) as i32) & iTemp331 != 0 {
																																																																																							fTemp355 = 1.442695 * fTemp313;
																																																																																							fTemp356 = F32::floor(fTemp355);
																																																																																							iTemp357 = (fTemp356) as i32;
																																																																																							fTemp358 = fTemp355 - fTemp356;
																																																																																							fTemp359 = F32::round((fTemp358 * (fTemp358 * (fTemp358 * (fTemp358 * (0.0013333 * fTemp358 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp357))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp357))) as F32) * fTemp330;
																																																																																						}
																																																																																							if (fTemp4 != 0.0) as i32 != 0 {
																																																																																								self.fRec85[0] = if iTemp311 != 0 {fTemp359} else {self.fRec85[1]};
																																																																																								fTemp360 = F32::min(self.fConst6, F32::max(2e+01, fTemp13 * self.fRec85[0] * (fTemp354 * (fTemp354 * (fTemp354 * (fTemp354 * (0.0013333 * fTemp354 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp353))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp353))) as F32));
																																																																																								fTemp361 = TextureDsp_faustpower2_f(fTemp360);
																																																																																								fTemp362 = self.fConst11 * fTemp361 * (self.fConst10 * fTemp361 + -444.1322) + 105.0;
																																																																																								fTemp363 = fTemp360 * (105.0 - self.fConst12 * fTemp361);
																																																																																								fTemp364 = self.fConst13 * (fTemp363 * (fTemp84 + self.fConst13 * (fTemp363 / fTemp362)) / fTemp362) + 1.0;
																																																																																								fTemp365 = fTemp363 * (fTemp3 - self.fRec81[1]) / fTemp362;
																																																																																								fTemp366 = self.fRec80[1] + self.fConst13 * fTemp365;
																																																																																								self.fRec80[0] = 2.0 * (fTemp366 / fTemp364) - self.fRec80[1];
																																																																																								self.fRec81[0] = 2.0 * (self.fRec81[1] + self.fConst7 * (fTemp363 * (3.1415927 * self.fRec80[1] + self.fConst14 * fTemp365) / (fTemp362 * fTemp364))) - self.fRec81[1];
																																																																																								fRec82 = 0.6666667 * (fTemp366 / (fTemp66 * fTemp364));
																																																																																								fTemp367 = fTemp3 + fRec82 * ((fTemp347 * (fTemp347 * (fTemp347 * (fTemp347 * (0.0013333 * fTemp347 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp346))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp346))) as F32 + -1.0);
																																																																																								fTemp368 = fTemp308 * (fTemp367 - self.fRec69[1]) / fTemp307;
																																																																																								fTemp369 = self.fRec68[1] + self.fConst13 * fTemp368;
																																																																																								self.fRec68[0] = 2.0 * (fTemp369 / fTemp309) - self.fRec68[1];
																																																																																								self.fRec69[0] = 2.0 * (self.fRec69[1] + self.fConst7 * (fTemp308 * (3.1415927 * self.fRec68[1] + self.fConst14 * fTemp368) / (fTemp307 * fTemp309))) - self.fRec69[1];
																																																																																								fRec70 = 0.6666667 * (fTemp369 / (fTemp66 * fTemp309));
																																																																																								fTemp370 = fTemp367 + fRec70 * ((fTemp292 * (fTemp292 * (fTemp292 * (fTemp292 * (0.0013333 * fTemp292 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp291))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp291))) as F32 + -1.0);
																																																																																								fTemp371 = fTemp252 * (fTemp370 - self.fRec57[1]) / fTemp251;
																																																																																								fTemp372 = self.fRec56[1] + self.fConst13 * fTemp371;
																																																																																								self.fRec56[0] = 2.0 * (fTemp372 / fTemp253) - self.fRec56[1];
																																																																																								self.fRec57[0] = 2.0 * (self.fRec57[1] + self.fConst7 * (fTemp252 * (3.1415927 * self.fRec56[1] + self.fConst14 * fTemp371) / (fTemp251 * fTemp253))) - self.fRec57[1];
																																																																																								fRec58 = 0.6666667 * (fTemp372 / (fTemp66 * fTemp253));
																																																																																								fTemp373 = fTemp370 + fRec58 * ((fTemp236 * (fTemp236 * (fTemp236 * (fTemp236 * (0.0013333 * fTemp236 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp235))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp235))) as F32 + -1.0);
																																																																																								fTemp374 = fTemp196 * (fTemp373 - self.fRec45[1]) / fTemp195;
																																																																																								fTemp375 = self.fRec44[1] + self.fConst13 * fTemp374;
																																																																																								self.fRec44[0] = 2.0 * (fTemp375 / fTemp197) - self.fRec44[1];
																																																																																								self.fRec45[0] = 2.0 * (self.fRec45[1] + self.fConst7 * (fTemp196 * (3.1415927 * self.fRec44[1] + self.fConst14 * fTemp374) / (fTemp195 * fTemp197))) - self.fRec45[1];
																																																																																								fRec46 = 0.6666667 * (fTemp375 / (fTemp66 * fTemp197));
																																																																																								fTemp376 = fTemp373 + fRec46 * ((fTemp180 * (fTemp180 * (fTemp180 * (fTemp180 * (0.0013333 * fTemp180 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp179))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp179))) as F32 + -1.0);
																																																																																								fTemp377 = fTemp140 * (fTemp376 - self.fRec33[1]) / fTemp139;
																																																																																								fTemp378 = self.fRec32[1] + self.fConst13 * fTemp377;
																																																																																								self.fRec32[0] = 2.0 * (fTemp378 / fTemp141) - self.fRec32[1];
																																																																																								self.fRec33[0] = 2.0 * (self.fRec33[1] + self.fConst7 * (fTemp140 * (3.1415927 * self.fRec32[1] + self.fConst14 * fTemp377) / (fTemp139 * fTemp141))) - self.fRec33[1];
																																																																																								fRec34 = 0.6666667 * (fTemp378 / (fTemp66 * fTemp141));
																																																																																								fTemp379 = fTemp376 + fRec34 * ((fTemp124 * (fTemp124 * (fTemp124 * (fTemp124 * (0.0013333 * fTemp124 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp123))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp123))) as F32 + -1.0);
																																																																																								fTemp380 = fTemp83 * (fTemp379 - self.fRec20[1]) / fTemp82;
																																																																																								fTemp381 = self.fRec19[1] + self.fConst13 * fTemp380;
																																																																																								self.fRec19[0] = 2.0 * (fTemp381 / fTemp85) - self.fRec19[1];
																																																																																								self.fRec20[0] = 2.0 * (self.fRec20[1] + self.fConst7 * (fTemp83 * (3.1415927 * self.fRec19[1] + self.fConst14 * fTemp380) / (fTemp82 * fTemp85))) - self.fRec20[1];
																																																																																								fRec21 = 0.6666667 * (fTemp381 / (fTemp66 * fTemp85));
																																																																																								fTemp382 = fTemp379 + fRec21 * ((fTemp64 * (fTemp64 * (fTemp64 * (fTemp64 * (0.0013333 * fTemp64 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp63))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp63))) as F32 + -1.0);
																																																																																								self.fRec86[0] = self.fConst16 * TextureDsp_faustpower2_f(fTemp382) + self.fConst15 * self.fRec86[1];
																																																																																								self.fRec87[0] = self.fConst16 * TextureDsp_faustpower2_f(fTemp3) + self.fConst15 * self.fRec87[1];
																																																																																								fTemp383 = F32::sqrt((self.fRec87[0] + 1e-09) / (self.fRec86[0] + 1e-09)) * fTemp4 * fTemp382;
																																																																																							}
																																																																																								*output0 = if iTemp2 != 0 {fTemp3 + self.fRec0[0] * (((iTemp2 != 0) as i32) as u32 as F32 * fTemp383 - fTemp3)} else {fTemp3};
																																																																																								self.fRec88[0] = fSlow4 * fTemp1 + self.fConst1 * fTemp0 * self.fRec88[1];
																																																																																								if iSlow24 != 0 {
																																																																																									fTemp384 = fSlow23 * (54.0 * F32::max(0.0, F32::min(1.0, fSlow20 * fTemp27 + F32::max(0.0, 0.018518519 * (self.fRec88[0] + 8e+01)) + fSlow21 * fTemp26)) + -8e+01);
																																																																																								}
																																																																																									let mut fTemp385: F32 = 0.1660964 * if iSlow22 != 0 {fSlow68 * fTemp384} else {self.fRec88[0]};
																																																																																									let mut fTemp386: F32 = F32::floor(fTemp385);
																																																																																									let mut iTemp387: i32 = (fTemp386) as i32;
																																																																																									let mut fTemp388: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iTemp387))) as F32;
																																																																																									let mut fTemp389: F32 = (1 << std::cmp::min(30, std::cmp::max(0, iTemp387))) as F32;
																																																																																									let mut fTemp390: F32 = fTemp385 - fTemp386;
																																																																																									let mut fTemp391: F32 = fTemp390 * (fTemp390 * (fTemp390 * (fTemp390 * (0.0013333 * fTemp390 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0;
																																																																																									if iSlow7 != 0 {
																																																																																										iTemp392 = (self.iRec90[1] >= self.iRec91[1]) as i32;
																																																																																										self.iRec90[0] = if iTemp392 != 0 {0} else {i32::wrapping_add(self.iRec90[1], 1)};
																																																																																										self.iRec96[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec96[1], i32::wrapping_mul(iSlow69, iTemp5))), 12345);
																																																																																										iTemp393 = i32::wrapping_add(self.iRec96[0], 104729);
																																																																																										iTemp394 = i32::wrapping_mul(1664525, iTemp393 ^ (iTemp393 >> 15));
																																																																																										iTemp395 = i32::wrapping_add(self.iRec94[1], 17);
																																																																																										iTemp396 = i32::wrapping_mul(1664525, iTemp395 ^ (iTemp395 >> 15));
																																																																																										iTemp397 = i32::wrapping_add(self.iRec93[1], 1);
																																																																																										iTemp398 = iTemp392 & ((iTemp397 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp396 ^ (iTemp396 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
																																																																																										iTemp399 = i32::wrapping_add(self.iRec96[0], 209458);
																																																																																										iTemp400 = i32::wrapping_mul(1664525, iTemp399 ^ (iTemp399 >> 15));
																																																																																										iTemp401 = if (iTemp398 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp394 ^ (iTemp394 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp6 != 0 {i32::wrapping_mul(1013904223, iTemp400 ^ (iTemp400 >> 13))} else {self.iRec94[1]};
																																																																																										iTemp402 = i32::wrapping_add(iTemp401, 17);
																																																																																										iTemp403 = i32::wrapping_mul(1664525, iTemp402 ^ (iTemp402 >> 15));
																																																																																										iTemp404 = if iTemp392 != 0 {if iTemp398 != 0 {0} else {iTemp397}} else {self.iRec93[1]};
																																																																																										iTemp405 = i32::wrapping_add(iTemp401, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp404), 1)));
																																																																																										iTemp406 = i32::wrapping_mul(1664525, iTemp405 ^ (iTemp405 >> 15));
																																																																																										fTemp407 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp406 ^ (iTemp406 >> 13))) as F32 + 0.5);
																																																																																										fTemp408 = F32::floor(fTemp407);
																																																																																										iTemp409 = (fTemp408) as i32;
																																																																																										fTemp410 = 0.7213475 * F32::log(0.0038221916 * fTemp13, std::f32::consts::E);
																																																																																										fTemp411 = F32::floor(fTemp410);
																																																																																										iTemp412 = (fTemp411) as i32;
																																																																																										fTemp413 = (1 << std::cmp::min(30, std::cmp::max(0, iTemp412))) as F32;
																																																																																										fTemp414 = fTemp410 - fTemp411;
																																																																																										fTemp415 = fTemp414 * (fTemp414 * (fTemp414 * (fTemp414 * (0.0013333 * fTemp414 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0;
																																																																																										self.fRec97[0] = fSlow70 * fTemp1 + self.fConst1 * fTemp0 * self.fRec97[1];
																																																																																									}
																																																																																										if iSlow7 & iSlow39 != 0 {
																																																																																											fTemp416 = fSlow38 * (2.75 * F32::max(0.0, F32::min(1.0, fSlow35 * fTemp27 + F32::max(0.0, 0.36363637 * (self.fRec97[0] + -0.25)) + fSlow36 * fTemp26)) + 0.25);
																																																																																										}
																																																																																											if iSlow7 != 0 {
																																																																																												fTemp417 = if iSlow37 != 0 {fSlow71 * fTemp416} else {self.fRec97[0]};
																																																																																												fTemp418 = fTemp407 - fTemp408;
																																																																																												fTemp419 = (1 << std::cmp::min(30, std::cmp::max(0, -iTemp412))) as F32;
																																																																																												iTemp420 = i32::wrapping_add(self.iRec96[0], 314187);
																																																																																												iTemp421 = i32::wrapping_mul(1664525, iTemp420 ^ (iTemp420 >> 15));
																																																																																												fTemp422 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp421 ^ (iTemp421 >> 13))) as F32 + 1.0;
																																																																																												iTemp423 = i32::wrapping_add(self.iRec96[0], 418916);
																																																																																												iTemp424 = i32::wrapping_mul(1664525, iTemp423 ^ (iTemp423 >> 15));
																																																																																												fTemp425 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp424 ^ (iTemp424 >> 13))) as F32;
																																																																																												fTemp426 = fTemp425 + 1.5;
																																																																																												fTemp427 = 0.5 - fTemp425;
																																																																																												fTemp428 = TextureDsp_faustpower2_f(fTemp427) / TextureDsp_faustpower2_f(fTemp426);
																																																																																												self.fRec98[0] = fSlow72 * fTemp1 + self.fConst1 * fTemp0 * self.fRec98[1];
																																																																																											}
																																																																																												if iSlow7 & iSlow29 != 0 {
																																																																																													fTemp429 = fSlow28 * F32::max(0.0, F32::min(1.0, fSlow25 * fTemp27 + F32::max(0.0, self.fRec98[0]) + fSlow26 * fTemp26));
																																																																																												}
																																																																																													if iSlow7 != 0 {
																																																																																														fTemp430 = 6.9068904 * if iSlow27 != 0 {fSlow73 * fTemp429} else {self.fRec98[0]};
																																																																																														fTemp431 = F32::floor(-fTemp430);
																																																																																														iTemp432 = (fTemp431) as i32;
																																																																																														fTemp433 = fTemp430 + fTemp431;
																																																																																														iTemp434 = i32::wrapping_add(iTemp401, i32::wrapping_mul(190681, iTemp404));
																																																																																														iTemp435 = i32::wrapping_mul(1664525, iTemp434 ^ (iTemp434 >> 15));
																																																																																														fTemp436 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp435 ^ (iTemp435 >> 13))) as F32 + 0.5);
																																																																																														fTemp437 = F32::floor(fTemp436);
																																																																																														iTemp438 = (fTemp437) as i32;
																																																																																														fTemp439 = fTemp436 - fTemp437;
																																																																																														iTemp440 = if iTemp392 != 0 {i32::wrapping_mul(i32::wrapping_sub(1, iTemp6), (self.fConst17 * ((fTemp439 * (fTemp439 * (fTemp439 * (fTemp439 * (0.0013333 * fTemp439 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp438))) as F32 * fTemp422 * fTemp419 / ((1 << std::cmp::min(30, std::cmp::max(0, -iTemp438))) as F32 * fTemp417 * fTemp415 * fTemp413))) as i32)} else {self.iRec92[1]};
																																																																																														self.iRec91[0] = if iTemp392 != 0 {i32::wrapping_add(iTemp440, (self.fConst0 * if (((i32::wrapping_add(iTemp404, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp403 ^ (iTemp403 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp6 != 0 {1.2e+02 * ((1.0 - fTemp433 * (0.693147 - fTemp433 * (0.240227 - fTemp433 * (0.0555041 - fTemp433 * (0.0096181 - 0.0013333 * fTemp433))))) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp432))) as F32 * fTemp427 * (15.0 - 4.0 * fTemp428) / ((1 << std::cmp::min(30, std::cmp::max(0, -iTemp432))) as F32 * fTemp426 * (15.0 - 9.0 * fTemp428)))} else {0.02 * (fTemp422 * fTemp419 * (fTemp418 * (fTemp418 * (fTemp418 * (fTemp418 * (0.0013333 * fTemp418 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp409))) as F32 / (fTemp417 * fTemp415 * fTemp413 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp409))) as F32))}) as i32)} else {self.iRec91[1]};
																																																																																														self.iRec92[0] = iTemp440;
																																																																																														self.iRec93[0] = iTemp404;
																																																																																														self.iRec94[0] = iTemp401;
																																																																																														iRec95 = iTemp392;
																																																																																														iTemp441 = i32::wrapping_mul(31, self.iRec93[0]);
																																																																																														iTemp442 = i32::wrapping_add(self.iRec94[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp441, 8)));
																																																																																														iTemp443 = i32::wrapping_mul(1664525, iTemp442 ^ (iTemp442 >> 15));
																																																																																														self.fRec89[0] = if iRec95 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp443 ^ (iTemp443 >> 13))) as F32 + 0.5} else {self.fRec89[1]};
																																																																																														fTemp444 = 1.5707964 * self.fRec89[0];
																																																																																														fTemp445 = F32::min(1.0, F32::max(0.0, (self.iRec90[0]) as F32 / (std::cmp::max(self.iRec92[0], 1)) as F32));
																																																																																														fTemp446 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp445)));
																																																																																														fTemp447 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp445));
																																																																																														fTemp448 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp445 + -0.6)));
																																																																																														iTemp449 = i32::wrapping_add(self.iRec94[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp441, 7)));
																																																																																														iTemp450 = i32::wrapping_mul(1664525, iTemp449 ^ (iTemp449 >> 15));
																																																																																														fTemp451 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp450 ^ (iTemp450 >> 13))) as F32 + 0.5);
																																																																																														fTemp452 = F32::floor(fTemp451);
																																																																																														iTemp453 = (fTemp452) as i32;
																																																																																														fTemp454 = fTemp451 - fTemp452;
																																																																																														iTemp455 = i32::wrapping_add(self.iRec94[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp441, 6)));
																																																																																														iTemp456 = i32::wrapping_mul(1664525, iTemp455 ^ (iTemp455 >> 15));
																																																																																														iTemp457 = i32::wrapping_add(self.iRec94[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp441, 4)));
																																																																																														iTemp458 = i32::wrapping_mul(1664525, iTemp457 ^ (iTemp457 >> 15));
																																																																																														iTemp459 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp458 ^ (iTemp458 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
																																																																																														self.fRec100[0] = if iRec95 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp459, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp456 ^ (iTemp456 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp454 * (fTemp454 * (fTemp454 * (fTemp454 * (0.0013333 * fTemp454 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp453))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp453))) as F32)} else {self.fRec100[1]};
																																																																																														fTemp460 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp445 + -0.1)));
																																																																																														iTemp461 = i32::wrapping_add(self.iRec96[0], 628374);
																																																																																														iTemp462 = i32::wrapping_mul(1664525, iTemp461 ^ (iTemp461 >> 15));
																																																																																														iTemp463 = i32::wrapping_add(self.iRec94[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp441, 3)));
																																																																																														iTemp464 = i32::wrapping_mul(1664525, iTemp463 ^ (iTemp463 >> 15));
																																																																																														fTemp465 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp464 ^ (iTemp464 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp462 ^ (iTemp462 >> 13))) as F32 + 0.5));
																																																																																														fTemp466 = F32::floor(fTemp465);
																																																																																														iTemp467 = (fTemp466) as i32;
																																																																																														fTemp468 = fTemp465 - fTemp466;
																																																																																														fTemp469 = F32::max(1.0, F32::round(5e+02 * ((fTemp468 * (fTemp468 * (fTemp468 * (fTemp468 * (0.0013333 * fTemp468 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp467))) as F32 / (fTemp13 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp467))) as F32))));
																																																																																														iTemp470 = i32::wrapping_add(self.iRec94[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp441, 5)));
																																																																																														iTemp471 = i32::wrapping_mul(1664525, iTemp470 ^ (iTemp470 >> 15));
																																																																																														fTemp472 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp471 ^ (iTemp471 >> 13))) as F32 + 0.5);
																																																																																														fTemp473 = F32::floor(fTemp472);
																																																																																														iTemp474 = (fTemp473) as i32;
																																																																																														fTemp475 = fTemp472 - fTemp473;
																																																																																														fTemp476 = (iTemp459) as F32;
																																																																																														fTemp477 = 0.16666667 * (fTemp476 * (fTemp475 * (fTemp475 * (fTemp475 * (fTemp475 * (0.0013333 * fTemp475 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp474))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp474))) as F32);
																																																																																														fTemp478 = F32::floor(fTemp477);
																																																																																														iTemp479 = (fTemp478) as i32;
																																																																																														fTemp480 = fTemp477 - fTemp478;
																																																																																														fTemp481 = F32::round(fTemp469 * (fTemp480 * (fTemp480 * (fTemp480 * (fTemp480 * (0.0013333 * fTemp480 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp479))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp479))) as F32);
																																																																																														self.fRec101[0] = if iRec95 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp481 == fTemp469) as i32 != 0 {fTemp469 + fTemp476} else {fTemp481}) / fTemp469, std::f32::consts::E)} else {self.fRec101[1]};
																																																																																														self.fRec102[0] = if iRec95 != 0 {1.442695 * F32::log(fTemp469, std::f32::consts::E)} else {self.fRec102[1]};
																																																																																														fTemp482 = self.fRec102[0] + self.fRec101[0] * fTemp460 * (2.0 - fTemp460) + self.fRec100[0] * TextureDsp_faustpower2_f(fTemp448) * (3.0 - 2.0 * fTemp448);
																																																																																														fTemp483 = F32::floor(fTemp482);
																																																																																														iTemp484 = (fTemp483) as i32;
																																																																																														fTemp485 = fTemp482 - fTemp483;
																																																																																														fTemp486 = self.fRec99[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fTemp13 * (fTemp485 * (fTemp485 * (fTemp485 * (fTemp485 * (0.0013333 * fTemp485 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp484))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp484))) as F32));
																																																																																														self.fRec99[0] = fTemp486 - F32::floor(fTemp486);
																																																																																														iTemp487 = i32::wrapping_add(self.iRec94[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp441, 2)));
																																																																																														iTemp488 = i32::wrapping_mul(1664525, iTemp487 ^ (iTemp487 >> 15));
																																																																																														self.fRec103[0] = if iRec95 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp488 ^ (iTemp488 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec103[1]};
																																																																																														fTemp489 = self.fRec103[0] * F32::sin(6.2831855 * self.fRec99[0]) * ((self.iRec92[0] > 0) as i32) as u32 as F32 * TextureDsp_faustpower2_f(fTemp447) * (3.0 - 2.0 * fTemp447) * TextureDsp_faustpower2_f(fTemp446) * (3.0 - 2.0 * fTemp446);
																																																																																														fTemp490 = fSlow6 * fTemp489 * F32::cos(fTemp444);
																																																																																													}
																																																																																														let mut fTemp491: F32 = *input4;
																																																																																														*output1 = fSlow74 * (fTemp491 * fTemp490 * fTemp391 * fTemp389 / fTemp388);
																																																																																														if iSlow7 != 0 {
																																																																																															fTemp492 = fSlow6 * fTemp489 * F32::sin(fTemp444);
																																																																																														}
																																																																																															*output2 = fSlow74 * (fTemp491 * fTemp391 * fTemp389 * fTemp492 / fTemp388);
																																																																																															self.iVec0[1] = self.iVec0[0];
																																																																																															self.fVec1[1] = self.fVec1[0];
																																																																																															self.fRec0[1] = self.fRec0[0];
																																																																																															if (fTemp4 != 0.0) as i32 != 0 {
																																																																																																self.fRec3[1] = self.fRec3[0];
																																																																																																self.iRec4[1] = self.iRec4[0];
																																																																																																self.fRec2[1] = self.fRec2[0];
																																																																																																self.fRec1[1] = self.fRec1[0];
																																																																																																self.iRec9[1] = self.iRec9[0];
																																																																																																self.fRec10[1] = self.fRec10[0];
																																																																																															}
																																																																																																if (((fTemp4 != 0.0) as i32) & iSlow49) | ((((fTemp4 != 0.0) as i32) & iSlow17) | ((((fTemp4 != 0.0) as i32) & iSlow44) | ((iSlow7 & iSlow39) | ((((fTemp4 != 0.0) as i32) & iSlow34) | ((iSlow7 & iSlow29) | iSlow24))))) != 0 {
																																																																																																	self.fRec11[1] = self.fRec11[0];
																																																																																																	self.fRec12[1] = self.fRec12[0];
																																																																																																	self.fRec13[1] = self.fRec13[0];
																																																																																																	self.fRec14[1] = self.fRec14[0];
																																																																																																}
																																																																																																	if (fTemp4 != 0.0) as i32 != 0 {
																																																																																																		self.fRec8[1] = self.fRec8[0];
																																																																																																		self.fRec15[1] = self.fRec15[0];
																																																																																																		self.fRec7[1] = self.fRec7[0];
																																																																																																		self.fRec16[1] = self.fRec16[0];
																																																																																																		self.fVec2[1] = self.fVec2[0];
																																																																																																		self.fRec6[1] = self.fRec6[0];
																																																																																																		self.fRec5[1] = self.fRec5[0];
																																																																																																		self.fRec17[1] = self.fRec17[0];
																																																																																																		self.fRec18[1] = self.fRec18[0];
																																																																																																		self.fRec22[1] = self.fRec22[0];
																																																																																																		self.fRec24[1] = self.fRec24[0];
																																																																																																		self.fRec23[1] = self.fRec23[0];
																																																																																																		self.fRec25[1] = self.fRec25[0];
																																																																																																		self.fRec29[1] = self.fRec29[0];
																																																																																																		self.fRec28[1] = self.fRec28[0];
																																																																																																		self.fRec30[1] = self.fRec30[0];
																																																																																																		self.fVec3[1] = self.fVec3[0];
																																																																																																		self.fRec27[1] = self.fRec27[0];
																																																																																																		self.fRec26[1] = self.fRec26[0];
																																																																																																		self.fRec31[1] = self.fRec31[0];
																																																																																																		self.fRec36[1] = self.fRec36[0];
																																																																																																		self.fRec35[1] = self.fRec35[0];
																																																																																																		self.fRec37[1] = self.fRec37[0];
																																																																																																		self.fRec41[1] = self.fRec41[0];
																																																																																																		self.fRec40[1] = self.fRec40[0];
																																																																																																		self.fRec42[1] = self.fRec42[0];
																																																																																																		self.fVec4[1] = self.fVec4[0];
																																																																																																		self.fRec39[1] = self.fRec39[0];
																																																																																																		self.fRec38[1] = self.fRec38[0];
																																																																																																		self.fRec43[1] = self.fRec43[0];
																																																																																																		self.fRec48[1] = self.fRec48[0];
																																																																																																		self.fRec47[1] = self.fRec47[0];
																																																																																																		self.fRec49[1] = self.fRec49[0];
																																																																																																		self.fRec53[1] = self.fRec53[0];
																																																																																																		self.fRec52[1] = self.fRec52[0];
																																																																																																		self.fRec54[1] = self.fRec54[0];
																																																																																																		self.fVec5[1] = self.fVec5[0];
																																																																																																		self.fRec51[1] = self.fRec51[0];
																																																																																																		self.fRec50[1] = self.fRec50[0];
																																																																																																		self.fRec55[1] = self.fRec55[0];
																																																																																																		self.fRec60[1] = self.fRec60[0];
																																																																																																		self.fRec59[1] = self.fRec59[0];
																																																																																																		self.fRec61[1] = self.fRec61[0];
																																																																																																		self.fRec65[1] = self.fRec65[0];
																																																																																																		self.fRec64[1] = self.fRec64[0];
																																																																																																		self.fRec66[1] = self.fRec66[0];
																																																																																																		self.fVec6[1] = self.fVec6[0];
																																																																																																		self.fRec63[1] = self.fRec63[0];
																																																																																																		self.fRec62[1] = self.fRec62[0];
																																																																																																		self.fRec67[1] = self.fRec67[0];
																																																																																																		self.fRec72[1] = self.fRec72[0];
																																																																																																		self.fRec71[1] = self.fRec71[0];
																																																																																																		self.fRec73[1] = self.fRec73[0];
																																																																																																		self.fRec77[1] = self.fRec77[0];
																																																																																																		self.fRec76[1] = self.fRec76[0];
																																																																																																		self.fRec78[1] = self.fRec78[0];
																																																																																																		self.fVec7[1] = self.fVec7[0];
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
																																																																																																		self.fRec19[1] = self.fRec19[0];
																																																																																																		self.fRec20[1] = self.fRec20[0];
																																																																																																		self.fRec86[1] = self.fRec86[0];
																																																																																																		self.fRec87[1] = self.fRec87[0];
																																																																																																	}
																																																																																																		self.fRec88[1] = self.fRec88[0];
																																																																																																		if iSlow7 != 0 {
																																																																																																			self.iRec90[1] = self.iRec90[0];
																																																																																																			self.iRec96[1] = self.iRec96[0];
																																																																																																			self.fRec97[1] = self.fRec97[0];
																																																																																																			self.fRec98[1] = self.fRec98[0];
																																																																																																			self.iRec91[1] = self.iRec91[0];
																																																																																																			self.iRec92[1] = self.iRec92[0];
																																																																																																			self.iRec93[1] = self.iRec93[0];
																																																																																																			self.iRec94[1] = self.iRec94[0];
																																																																																																			self.fRec89[1] = self.fRec89[0];
																																																																																																			self.fRec100[1] = self.fRec100[0];
																																																																																																			self.fRec101[1] = self.fRec101[0];
																																																																																																			self.fRec102[1] = self.fRec102[0];
																																																																																																			self.fRec99[1] = self.fRec99[0];
																																																																																																			self.fRec103[1] = self.fRec103[0];
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
																																																																																																		self.fTempPerm25 = fTemp30;
																																																																																																		self.fTempPerm26 = fTemp31;
																																																																																																		self.fTempPerm27 = fTemp32;
																																																																																																		self.fTempPerm28 = fTemp33;
																																																																																																		self.fTempPerm29 = fTemp34;
																																																																																																		self.iTempPerm30 = iTemp35;
																																																																																																		self.iTempPerm31 = iTemp36;
																																																																																																		self.fTempPerm32 = fTemp37;
																																																																																																		self.fTempPerm33 = fTemp38;
																																																																																																		self.fTempPerm34 = fTemp39;
																																																																																																		self.fTempPerm35 = fTemp40;
																																																																																																		self.fTempPerm36 = fTemp41;
																																																																																																		self.fTempPerm37 = fTemp42;
																																																																																																		self.iTempPerm38 = iTemp43;
																																																																																																		self.fTempPerm39 = fTemp44;
																																																																																																		self.fTempPerm40 = fTemp45;
																																																																																																		self.fTempPerm41 = fTemp46;
																																																																																																		self.iTempPerm42 = iTemp47;
																																																																																																		self.iTempPerm43 = iTemp48;
																																																																																																		self.iTempPerm44 = iTemp49;
																																																																																																		self.fTempPerm45 = fTemp50;
																																																																																																		self.fTempPerm46 = fTemp51;
																																																																																																		self.iTempPerm47 = iTemp52;
																																																																																																		self.iTempPerm48 = iTemp53;
																																																																																																		self.fTempPerm49 = fTemp54;
																																																																																																		self.fTempPerm50 = fTemp55;
																																																																																																		self.fTempPerm51 = fTemp56;
																																																																																																		self.fTempPerm52 = fTemp57;
																																																																																																		self.fTempPerm53 = fTemp58;
																																																																																																		self.fTempPerm54 = fTemp59;
																																																																																																		self.fTempPerm55 = fTemp60;
																																																																																																		self.fTempPerm56 = fTemp61;
																																																																																																		self.fTempPerm57 = fTemp62;
																																																																																																		self.iTempPerm58 = iTemp63;
																																																																																																		self.fTempPerm59 = fTemp64;
																																																																																																		self.fTempPerm60 = fTemp65;
																																																																																																		self.fTempPerm61 = fTemp66;
																																																																																																		self.fTempPerm62 = fTemp67;
																																																																																																		self.iTempPerm63 = iTemp68;
																																																																																																		self.iTempPerm64 = iTemp69;
																																																																																																		self.fTempPerm65 = fTemp70;
																																																																																																		self.fTempPerm66 = fTemp71;
																																																																																																		self.fTempPerm67 = fTemp72;
																																																																																																		self.iTempPerm68 = iTemp73;
																																																																																																		self.fTempPerm69 = fTemp74;
																																																																																																		self.fTempPerm70 = fTemp75;
																																																																																																		self.fTempPerm71 = fTemp76;
																																																																																																		self.iTempPerm72 = iTemp77;
																																																																																																		self.fTempPerm73 = fTemp78;
																																																																																																		self.fTempPerm74 = fTemp79;
																																																																																																		self.fTempPerm75 = fTemp80;
																																																																																																		self.fTempPerm76 = fTemp81;
																																																																																																		self.fTempPerm77 = fTemp82;
																																																																																																		self.fTempPerm78 = fTemp83;
																																																																																																		self.fTempPerm79 = fTemp84;
																																																																																																		self.fTempPerm80 = fTemp85;
																																																																																																		self.iTempPerm81 = iTemp86;
																																																																																																		self.iTempPerm82 = iTemp87;
																																																																																																		self.iTempPerm83 = iTemp88;
																																																																																																		self.iTempPerm84 = iTemp89;
																																																																																																		self.fTempPerm85 = fTemp90;
																																																																																																		self.fTempPerm86 = fTemp91;
																																																																																																		self.iTempPerm87 = iTemp92;
																																																																																																		self.iTempPerm88 = iTemp93;
																																																																																																		self.fTempPerm89 = fTemp94;
																																																																																																		self.fTempPerm90 = fTemp95;
																																																																																																		self.iTempPerm91 = iTemp96;
																																																																																																		self.fTempPerm92 = fTemp97;
																																																																																																		self.fTempPerm93 = fTemp98;
																																																																																																		self.fTempPerm94 = fTemp99;
																																																																																																		self.iTempPerm95 = iTemp100;
																																																																																																		self.iTempPerm96 = iTemp101;
																																																																																																		self.fTempPerm97 = fTemp102;
																																																																																																		self.fTempPerm98 = fTemp103;
																																																																																																		self.fTempPerm99 = fTemp104;
																																																																																																		self.fTempPerm100 = fTemp105;
																																																																																																		self.fTempPerm101 = fTemp106;
																																																																																																		self.fTempPerm102 = fTemp107;
																																																																																																		self.iTempPerm103 = iTemp108;
																																																																																																		self.fTempPerm104 = fTemp109;
																																																																																																		self.fTempPerm105 = fTemp110;
																																																																																																		self.fTempPerm106 = fTemp111;
																																																																																																		self.iTempPerm107 = iTemp112;
																																																																																																		self.iTempPerm108 = iTemp113;
																																																																																																		self.iTempPerm109 = iTemp114;
																																																																																																		self.fTempPerm110 = fTemp115;
																																																																																																		self.fTempPerm111 = fTemp116;
																																																																																																		self.iTempPerm112 = iTemp117;
																																																																																																		self.iTempPerm113 = iTemp118;
																																																																																																		self.fTempPerm114 = fTemp119;
																																																																																																		self.fTempPerm115 = fTemp120;
																																																																																																		self.fTempPerm116 = fTemp121;
																																																																																																		self.fTempPerm117 = fTemp122;
																																																																																																		self.iTempPerm118 = iTemp123;
																																																																																																		self.fTempPerm119 = fTemp124;
																																																																																																		self.iTempPerm120 = iTemp125;
																																																																																																		self.iTempPerm121 = iTemp126;
																																																																																																		self.fTempPerm122 = fTemp127;
																																																																																																		self.fTempPerm123 = fTemp128;
																																																																																																		self.fTempPerm124 = fTemp129;
																																																																																																		self.iTempPerm125 = iTemp130;
																																																																																																		self.fTempPerm126 = fTemp131;
																																																																																																		self.fTempPerm127 = fTemp132;
																																																																																																		self.fTempPerm128 = fTemp133;
																																																																																																		self.iTempPerm129 = iTemp134;
																																																																																																		self.fTempPerm130 = fTemp135;
																																																																																																		self.fTempPerm131 = fTemp136;
																																																																																																		self.fTempPerm132 = fTemp137;
																																																																																																		self.fTempPerm133 = fTemp138;
																																																																																																		self.fTempPerm134 = fTemp139;
																																																																																																		self.fTempPerm135 = fTemp140;
																																																																																																		self.fTempPerm136 = fTemp141;
																																																																																																		self.iTempPerm137 = iTemp142;
																																																																																																		self.iTempPerm138 = iTemp143;
																																																																																																		self.iTempPerm139 = iTemp144;
																																																																																																		self.iTempPerm140 = iTemp145;
																																																																																																		self.fTempPerm141 = fTemp146;
																																																																																																		self.fTempPerm142 = fTemp147;
																																																																																																		self.iTempPerm143 = iTemp148;
																																																																																																		self.iTempPerm144 = iTemp149;
																																																																																																		self.fTempPerm145 = fTemp150;
																																																																																																		self.fTempPerm146 = fTemp151;
																																																																																																		self.iTempPerm147 = iTemp152;
																																																																																																		self.fTempPerm148 = fTemp153;
																																																																																																		self.fTempPerm149 = fTemp154;
																																																																																																		self.fTempPerm150 = fTemp155;
																																																																																																		self.iTempPerm151 = iTemp156;
																																																																																																		self.iTempPerm152 = iTemp157;
																																																																																																		self.fTempPerm153 = fTemp158;
																																																																																																		self.fTempPerm154 = fTemp159;
																																																																																																		self.fTempPerm155 = fTemp160;
																																																																																																		self.fTempPerm156 = fTemp161;
																																																																																																		self.fTempPerm157 = fTemp162;
																																																																																																		self.fTempPerm158 = fTemp163;
																																																																																																		self.iTempPerm159 = iTemp164;
																																																																																																		self.fTempPerm160 = fTemp165;
																																																																																																		self.fTempPerm161 = fTemp166;
																																																																																																		self.fTempPerm162 = fTemp167;
																																																																																																		self.iTempPerm163 = iTemp168;
																																																																																																		self.iTempPerm164 = iTemp169;
																																																																																																		self.iTempPerm165 = iTemp170;
																																																																																																		self.fTempPerm166 = fTemp171;
																																																																																																		self.fTempPerm167 = fTemp172;
																																																																																																		self.iTempPerm168 = iTemp173;
																																																																																																		self.iTempPerm169 = iTemp174;
																																																																																																		self.fTempPerm170 = fTemp175;
																																																																																																		self.fTempPerm171 = fTemp176;
																																																																																																		self.fTempPerm172 = fTemp177;
																																																																																																		self.fTempPerm173 = fTemp178;
																																																																																																		self.iTempPerm174 = iTemp179;
																																																																																																		self.fTempPerm175 = fTemp180;
																																																																																																		self.iTempPerm176 = iTemp181;
																																																																																																		self.iTempPerm177 = iTemp182;
																																																																																																		self.fTempPerm178 = fTemp183;
																																																																																																		self.fTempPerm179 = fTemp184;
																																																																																																		self.fTempPerm180 = fTemp185;
																																																																																																		self.iTempPerm181 = iTemp186;
																																																																																																		self.fTempPerm182 = fTemp187;
																																																																																																		self.fTempPerm183 = fTemp188;
																																																																																																		self.fTempPerm184 = fTemp189;
																																																																																																		self.iTempPerm185 = iTemp190;
																																																																																																		self.fTempPerm186 = fTemp191;
																																																																																																		self.fTempPerm187 = fTemp192;
																																																																																																		self.fTempPerm188 = fTemp193;
																																																																																																		self.fTempPerm189 = fTemp194;
																																																																																																		self.fTempPerm190 = fTemp195;
																																																																																																		self.fTempPerm191 = fTemp196;
																																																																																																		self.fTempPerm192 = fTemp197;
																																																																																																		self.iTempPerm193 = iTemp198;
																																																																																																		self.iTempPerm194 = iTemp199;
																																																																																																		self.iTempPerm195 = iTemp200;
																																																																																																		self.iTempPerm196 = iTemp201;
																																																																																																		self.fTempPerm197 = fTemp202;
																																																																																																		self.fTempPerm198 = fTemp203;
																																																																																																		self.iTempPerm199 = iTemp204;
																																																																																																		self.iTempPerm200 = iTemp205;
																																																																																																		self.fTempPerm201 = fTemp206;
																																																																																																		self.fTempPerm202 = fTemp207;
																																																																																																		self.iTempPerm203 = iTemp208;
																																																																																																		self.fTempPerm204 = fTemp209;
																																																																																																		self.fTempPerm205 = fTemp210;
																																																																																																		self.fTempPerm206 = fTemp211;
																																																																																																		self.iTempPerm207 = iTemp212;
																																																																																																		self.iTempPerm208 = iTemp213;
																																																																																																		self.fTempPerm209 = fTemp214;
																																																																																																		self.fTempPerm210 = fTemp215;
																																																																																																		self.fTempPerm211 = fTemp216;
																																																																																																		self.fTempPerm212 = fTemp217;
																																																																																																		self.fTempPerm213 = fTemp218;
																																																																																																		self.fTempPerm214 = fTemp219;
																																																																																																		self.iTempPerm215 = iTemp220;
																																																																																																		self.fTempPerm216 = fTemp221;
																																																																																																		self.fTempPerm217 = fTemp222;
																																																																																																		self.fTempPerm218 = fTemp223;
																																																																																																		self.iTempPerm219 = iTemp224;
																																																																																																		self.iTempPerm220 = iTemp225;
																																																																																																		self.iTempPerm221 = iTemp226;
																																																																																																		self.fTempPerm222 = fTemp227;
																																																																																																		self.fTempPerm223 = fTemp228;
																																																																																																		self.iTempPerm224 = iTemp229;
																																																																																																		self.iTempPerm225 = iTemp230;
																																																																																																		self.fTempPerm226 = fTemp231;
																																																																																																		self.fTempPerm227 = fTemp232;
																																																																																																		self.fTempPerm228 = fTemp233;
																																																																																																		self.fTempPerm229 = fTemp234;
																																																																																																		self.iTempPerm230 = iTemp235;
																																																																																																		self.fTempPerm231 = fTemp236;
																																																																																																		self.iTempPerm232 = iTemp237;
																																																																																																		self.iTempPerm233 = iTemp238;
																																																																																																		self.fTempPerm234 = fTemp239;
																																																																																																		self.fTempPerm235 = fTemp240;
																																																																																																		self.fTempPerm236 = fTemp241;
																																																																																																		self.iTempPerm237 = iTemp242;
																																																																																																		self.fTempPerm238 = fTemp243;
																																																																																																		self.fTempPerm239 = fTemp244;
																																																																																																		self.fTempPerm240 = fTemp245;
																																																																																																		self.iTempPerm241 = iTemp246;
																																																																																																		self.fTempPerm242 = fTemp247;
																																																																																																		self.fTempPerm243 = fTemp248;
																																																																																																		self.fTempPerm244 = fTemp249;
																																																																																																		self.fTempPerm245 = fTemp250;
																																																																																																		self.fTempPerm246 = fTemp251;
																																																																																																		self.fTempPerm247 = fTemp252;
																																																																																																		self.fTempPerm248 = fTemp253;
																																																																																																		self.iTempPerm249 = iTemp254;
																																																																																																		self.iTempPerm250 = iTemp255;
																																																																																																		self.iTempPerm251 = iTemp256;
																																																																																																		self.iTempPerm252 = iTemp257;
																																																																																																		self.fTempPerm253 = fTemp258;
																																																																																																		self.fTempPerm254 = fTemp259;
																																																																																																		self.iTempPerm255 = iTemp260;
																																																																																																		self.iTempPerm256 = iTemp261;
																																																																																																		self.fTempPerm257 = fTemp262;
																																																																																																		self.fTempPerm258 = fTemp263;
																																																																																																		self.iTempPerm259 = iTemp264;
																																																																																																		self.fTempPerm260 = fTemp265;
																																																																																																		self.fTempPerm261 = fTemp266;
																																																																																																		self.fTempPerm262 = fTemp267;
																																																																																																		self.iTempPerm263 = iTemp268;
																																																																																																		self.iTempPerm264 = iTemp269;
																																																																																																		self.fTempPerm265 = fTemp270;
																																																																																																		self.fTempPerm266 = fTemp271;
																																																																																																		self.fTempPerm267 = fTemp272;
																																																																																																		self.fTempPerm268 = fTemp273;
																																																																																																		self.fTempPerm269 = fTemp274;
																																																																																																		self.fTempPerm270 = fTemp275;
																																																																																																		self.iTempPerm271 = iTemp276;
																																																																																																		self.fTempPerm272 = fTemp277;
																																																																																																		self.fTempPerm273 = fTemp278;
																																																																																																		self.fTempPerm274 = fTemp279;
																																																																																																		self.iTempPerm275 = iTemp280;
																																																																																																		self.iTempPerm276 = iTemp281;
																																																																																																		self.iTempPerm277 = iTemp282;
																																																																																																		self.fTempPerm278 = fTemp283;
																																																																																																		self.fTempPerm279 = fTemp284;
																																																																																																		self.iTempPerm280 = iTemp285;
																																																																																																		self.iTempPerm281 = iTemp286;
																																																																																																		self.fTempPerm282 = fTemp287;
																																																																																																		self.fTempPerm283 = fTemp288;
																																																																																																		self.fTempPerm284 = fTemp289;
																																																																																																		self.fTempPerm285 = fTemp290;
																																																																																																		self.iTempPerm286 = iTemp291;
																																																																																																		self.fTempPerm287 = fTemp292;
																																																																																																		self.iTempPerm288 = iTemp293;
																																																																																																		self.iTempPerm289 = iTemp294;
																																																																																																		self.fTempPerm290 = fTemp295;
																																																																																																		self.fTempPerm291 = fTemp296;
																																																																																																		self.fTempPerm292 = fTemp297;
																																																																																																		self.iTempPerm293 = iTemp298;
																																																																																																		self.fTempPerm294 = fTemp299;
																																																																																																		self.fTempPerm295 = fTemp300;
																																																																																																		self.fTempPerm296 = fTemp301;
																																																																																																		self.iTempPerm297 = iTemp302;
																																																																																																		self.fTempPerm298 = fTemp303;
																																																																																																		self.fTempPerm299 = fTemp304;
																																																																																																		self.fTempPerm300 = fTemp305;
																																																																																																		self.fTempPerm301 = fTemp306;
																																																																																																		self.fTempPerm302 = fTemp307;
																																																																																																		self.fTempPerm303 = fTemp308;
																																																																																																		self.fTempPerm304 = fTemp309;
																																																																																																		self.iTempPerm305 = iTemp310;
																																																																																																		self.iTempPerm306 = iTemp311;
																																																																																																		self.iTempPerm307 = iTemp312;
																																																																																																		self.fTempPerm308 = fTemp313;
																																																																																																		self.fTempPerm309 = fTemp314;
																																																																																																		self.iTempPerm310 = iTemp315;
																																																																																																		self.iTempPerm311 = iTemp316;
																																																																																																		self.fTempPerm312 = fTemp317;
																																																																																																		self.fTempPerm313 = fTemp318;
																																																																																																		self.iTempPerm314 = iTemp319;
																																																																																																		self.fTempPerm315 = fTemp320;
																																																																																																		self.fTempPerm316 = fTemp321;
																																																																																																		self.fTempPerm317 = fTemp322;
																																																																																																		self.iTempPerm318 = iTemp323;
																																																																																																		self.iTempPerm319 = iTemp324;
																																																																																																		self.fTempPerm320 = fTemp325;
																																																																																																		self.fTempPerm321 = fTemp326;
																																																																																																		self.fTempPerm322 = fTemp327;
																																																																																																		self.fTempPerm323 = fTemp328;
																																																																																																		self.fTempPerm324 = fTemp329;
																																																																																																		self.fTempPerm325 = fTemp330;
																																																																																																		self.iTempPerm326 = iTemp331;
																																																																																																		self.fTempPerm327 = fTemp332;
																																																																																																		self.fTempPerm328 = fTemp333;
																																																																																																		self.fTempPerm329 = fTemp334;
																																																																																																		self.iTempPerm330 = iTemp335;
																																																																																																		self.iTempPerm331 = iTemp336;
																																																																																																		self.iTempPerm332 = iTemp337;
																																																																																																		self.fTempPerm333 = fTemp338;
																																																																																																		self.fTempPerm334 = fTemp339;
																																																																																																		self.iTempPerm335 = iTemp340;
																																																																																																		self.iTempPerm336 = iTemp341;
																																																																																																		self.fTempPerm337 = fTemp342;
																																																																																																		self.fTempPerm338 = fTemp343;
																																																																																																		self.fTempPerm339 = fTemp344;
																																																																																																		self.fTempPerm340 = fTemp345;
																																																																																																		self.iTempPerm341 = iTemp346;
																																																																																																		self.fTempPerm342 = fTemp347;
																																																																																																		self.iTempPerm343 = iTemp348;
																																																																																																		self.iTempPerm344 = iTemp349;
																																																																																																		self.fTempPerm345 = fTemp350;
																																																																																																		self.fTempPerm346 = fTemp351;
																																																																																																		self.fTempPerm347 = fTemp352;
																																																																																																		self.iTempPerm348 = iTemp353;
																																																																																																		self.fTempPerm349 = fTemp354;
																																																																																																		self.fTempPerm350 = fTemp355;
																																																																																																		self.fTempPerm351 = fTemp356;
																																																																																																		self.iTempPerm352 = iTemp357;
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
																																																																																																		self.fTempPerm372 = fTemp377;
																																																																																																		self.fTempPerm373 = fTemp378;
																																																																																																		self.fTempPerm374 = fTemp379;
																																																																																																		self.fTempPerm375 = fTemp380;
																																																																																																		self.fTempPerm376 = fTemp381;
																																																																																																		self.fTempPerm377 = fTemp382;
																																																																																																		self.fTempPerm378 = fTemp383;
																																																																																																		self.fTempPerm379 = fTemp384;
																																																																																																		self.iTempPerm380 = iTemp392;
																																																																																																		self.iTempPerm381 = iTemp393;
																																																																																																		self.iTempPerm382 = iTemp394;
																																																																																																		self.iTempPerm383 = iTemp395;
																																																																																																		self.iTempPerm384 = iTemp396;
																																																																																																		self.iTempPerm385 = iTemp397;
																																																																																																		self.iTempPerm386 = iTemp398;
																																																																																																		self.iTempPerm387 = iTemp399;
																																																																																																		self.iTempPerm388 = iTemp400;
																																																																																																		self.iTempPerm389 = iTemp401;
																																																																																																		self.iTempPerm390 = iTemp402;
																																																																																																		self.iTempPerm391 = iTemp403;
																																																																																																		self.iTempPerm392 = iTemp404;
																																																																																																		self.iTempPerm393 = iTemp405;
																																																																																																		self.iTempPerm394 = iTemp406;
																																																																																																		self.fTempPerm395 = fTemp407;
																																																																																																		self.fTempPerm396 = fTemp408;
																																																																																																		self.iTempPerm397 = iTemp409;
																																																																																																		self.fTempPerm398 = fTemp410;
																																																																																																		self.fTempPerm399 = fTemp411;
																																																																																																		self.iTempPerm400 = iTemp412;
																																																																																																		self.fTempPerm401 = fTemp413;
																																																																																																		self.fTempPerm402 = fTemp414;
																																																																																																		self.fTempPerm403 = fTemp415;
																																																																																																		self.fTempPerm404 = fTemp416;
																																																																																																		self.fTempPerm405 = fTemp417;
																																																																																																		self.fTempPerm406 = fTemp418;
																																																																																																		self.fTempPerm407 = fTemp419;
																																																																																																		self.iTempPerm408 = iTemp420;
																																																																																																		self.iTempPerm409 = iTemp421;
																																																																																																		self.fTempPerm410 = fTemp422;
																																																																																																		self.iTempPerm411 = iTemp423;
																																																																																																		self.iTempPerm412 = iTemp424;
																																																																																																		self.fTempPerm413 = fTemp425;
																																																																																																		self.fTempPerm414 = fTemp426;
																																																																																																		self.fTempPerm415 = fTemp427;
																																																																																																		self.fTempPerm416 = fTemp428;
																																																																																																		self.fTempPerm417 = fTemp429;
																																																																																																		self.fTempPerm418 = fTemp430;
																																																																																																		self.fTempPerm419 = fTemp431;
																																																																																																		self.iTempPerm420 = iTemp432;
																																																																																																		self.fTempPerm421 = fTemp433;
																																																																																																		self.iTempPerm422 = iTemp434;
																																																																																																		self.iTempPerm423 = iTemp435;
																																																																																																		self.fTempPerm424 = fTemp436;
																																																																																																		self.fTempPerm425 = fTemp437;
																																																																																																		self.iTempPerm426 = iTemp438;
																																																																																																		self.fTempPerm427 = fTemp439;
																																																																																																		self.iTempPerm428 = iTemp440;
																																																																																																		self.iTempPerm429 = iTemp441;
																																																																																																		self.iTempPerm430 = iTemp442;
																																																																																																		self.iTempPerm431 = iTemp443;
																																																																																																		self.fTempPerm432 = fTemp444;
																																																																																																		self.fTempPerm433 = fTemp445;
																																																																																																		self.fTempPerm434 = fTemp446;
																																																																																																		self.fTempPerm435 = fTemp447;
																																																																																																		self.fTempPerm436 = fTemp448;
																																																																																																		self.iTempPerm437 = iTemp449;
																																																																																																		self.iTempPerm438 = iTemp450;
																																																																																																		self.fTempPerm439 = fTemp451;
																																																																																																		self.fTempPerm440 = fTemp452;
																																																																																																		self.iTempPerm441 = iTemp453;
																																																																																																		self.fTempPerm442 = fTemp454;
																																																																																																		self.iTempPerm443 = iTemp455;
																																																																																																		self.iTempPerm444 = iTemp456;
																																																																																																		self.iTempPerm445 = iTemp457;
																																																																																																		self.iTempPerm446 = iTemp458;
																																																																																																		self.iTempPerm447 = iTemp459;
																																																																																																		self.fTempPerm448 = fTemp460;
																																																																																																		self.iTempPerm449 = iTemp461;
																																																																																																		self.iTempPerm450 = iTemp462;
																																																																																																		self.iTempPerm451 = iTemp463;
																																																																																																		self.iTempPerm452 = iTemp464;
																																																																																																		self.fTempPerm453 = fTemp465;
																																																																																																		self.fTempPerm454 = fTemp466;
																																																																																																		self.iTempPerm455 = iTemp467;
																																																																																																		self.fTempPerm456 = fTemp468;
																																																																																																		self.fTempPerm457 = fTemp469;
																																																																																																		self.iTempPerm458 = iTemp470;
																																																																																																		self.iTempPerm459 = iTemp471;
																																																																																																		self.fTempPerm460 = fTemp472;
																																																																																																		self.fTempPerm461 = fTemp473;
																																																																																																		self.iTempPerm462 = iTemp474;
																																																																																																		self.fTempPerm463 = fTemp475;
																																																																																																		self.fTempPerm464 = fTemp476;
																																																																																																		self.fTempPerm465 = fTemp477;
																																																																																																		self.fTempPerm466 = fTemp478;
																																																																																																		self.iTempPerm467 = iTemp479;
																																																																																																		self.fTempPerm468 = fTemp480;
																																																																																																		self.fTempPerm469 = fTemp481;
																																																																																																		self.fTempPerm470 = fTemp482;
																																																																																																		self.fTempPerm471 = fTemp483;
																																																																																																		self.iTempPerm472 = iTemp484;
																																																																																																		self.fTempPerm473 = fTemp485;
																																																																																																		self.fTempPerm474 = fTemp486;
																																																																																																		self.iTempPerm475 = iTemp487;
																																																																																																		self.iTempPerm476 = iTemp488;
																																																																																																		self.fTempPerm477 = fTemp489;
																																																																																																		self.fTempPerm478 = fTemp490;
																																																																																																		self.fTempPerm479 = fTemp492;
																																																																																																	}

}

