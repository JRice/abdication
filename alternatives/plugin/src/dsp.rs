/* ------------------------------------------------------------
name: "plugin"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn AlternativesDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn AlternativesDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct AlternativesDsp {
	fHslider0: F32,
	iVec0: [i32;2],
	fRec0: [F32;2],
	fCheckbox0: F32,
	fHslider1: F32,
	fHslider2: F32,
	fRec1: [F32;2],
	fSampleRate: i32,
	fConst1: F32,
	iTempPerm0: i32,
	iTempPerm1: i32,
	fConst2: F32,
	fRec4: [F32;2],
	fHslider3: F32,
	iRec5: [i32;2],
	iTempPerm2: i32,
	fRec3: [F32;2],
	fConst3: F32,
	fRec2: [F32;2],
	fHslider4: F32,
	fRec6: [F32;2],
	fTempPerm3: F32,
	fConst4: F32,
	iTempPerm4: i32,
	iTempPerm5: i32,
	fHslider5: F32,
	fRec12: [F32;2],
	fTempPerm6: F32,
	fHslider6: F32,
	iRec13: [i32;2],
	iTempPerm7: i32,
	iTempPerm8: i32,
	iTempPerm9: i32,
	fHslider7: F32,
	fHslider8: F32,
	fHslider9: F32,
	fHslider10: F32,
	fHslider11: F32,
	fHslider12: F32,
	fHslider13: F32,
	fHslider14: F32,
	fTempPerm10: F32,
	fConst5: F32,
	iTempPerm11: i32,
	iTempPerm12: i32,
	fTempPerm13: F32,
	fTempPerm14: F32,
	iTempPerm15: i32,
	iTempPerm16: i32,
	fTempPerm17: F32,
	fTempPerm18: F32,
	iTempPerm19: i32,
	fTempPerm20: F32,
	fHslider15: F32,
	fRec14: [F32;2],
	fTempPerm21: F32,
	fRec11: [F32;2],
	fHslider16: F32,
	fRec15: [F32;2],
	fTempPerm22: F32,
	fTempPerm23: F32,
	fConst6: F32,
	fTempPerm24: F32,
	iTempPerm25: i32,
	iTempPerm26: i32,
	fTempPerm27: F32,
	fTempPerm28: F32,
	fTempPerm29: F32,
	fTempPerm30: F32,
	fRec9: [F32;2],
	fRec10: [F32;2],
	fTempPerm31: F32,
	fTempPerm32: F32,
	iTempPerm33: i32,
	fTempPerm34: F32,
	fTempPerm35: F32,
	fTempPerm36: F32,
	iTempPerm37: i32,
	fTempPerm38: F32,
	fTempPerm39: F32,
	fRec16: [F32;2],
	fTempPerm40: F32,
	fVec1: [F32;2],
	fTempPerm41: F32,
	iTempPerm42: i32,
	iTempPerm43: i32,
	iTempPerm44: i32,
	fTempPerm45: F32,
	fTempPerm46: F32,
	fRec8: [F32;2],
	fConst7: F32,
	fRec7: [F32;2],
	iTempPerm47: i32,
	iTempPerm48: i32,
	fTempPerm49: F32,
	fRec17: [F32;2],
	fTempPerm50: F32,
	fHslider17: F32,
	fRec18: [F32;2],
	fTempPerm51: F32,
	fTempPerm52: F32,
	fTempPerm53: F32,
	fHslider18: F32,
	fRec19: [F32;2],
	fTempPerm54: F32,
	fTempPerm55: F32,
	fTempPerm56: F32,
	iTempPerm57: i32,
	fTempPerm58: F32,
	fTempPerm59: F32,
	fHslider19: F32,
	fRec23: [F32;2],
	fTempPerm60: F32,
	fTempPerm61: F32,
	iTempPerm62: i32,
	iTempPerm63: i32,
	fTempPerm64: F32,
	fRec25: [F32;2],
	fRec24: [F32;2],
	fTempPerm65: F32,
	fTempPerm66: F32,
	iTempPerm67: i32,
	fTempPerm68: F32,
	fTempPerm69: F32,
	fTempPerm70: F32,
	iTempPerm71: i32,
	fTempPerm72: F32,
	fTempPerm73: F32,
	fRec26: [F32;2],
	fTempPerm74: F32,
	fTempPerm75: F32,
	fConst9: F32,
	fConst10: F32,
	fTempPerm76: F32,
	fConst11: F32,
	fTempPerm77: F32,
	fConst12: F32,
	fTempPerm78: F32,
	fTempPerm79: F32,
	iTempPerm80: i32,
	iTempPerm81: i32,
	iTempPerm82: i32,
	iTempPerm83: i32,
	iTempPerm84: i32,
	fTempPerm85: F32,
	iTempPerm86: i32,
	iTempPerm87: i32,
	fTempPerm88: F32,
	fTempPerm89: F32,
	iTempPerm90: i32,
	iTempPerm91: i32,
	fTempPerm92: F32,
	fTempPerm93: F32,
	iTempPerm94: i32,
	fTempPerm95: F32,
	fRec31: [F32;2],
	fTempPerm96: F32,
	iTempPerm97: i32,
	iTempPerm98: i32,
	fTempPerm99: F32,
	fTempPerm100: F32,
	fTempPerm101: F32,
	fTempPerm102: F32,
	fRec29: [F32;2],
	fRec30: [F32;2],
	fTempPerm103: F32,
	fTempPerm104: F32,
	iTempPerm105: i32,
	fTempPerm106: F32,
	fTempPerm107: F32,
	fTempPerm108: F32,
	iTempPerm109: i32,
	fTempPerm110: F32,
	fTempPerm111: F32,
	fRec32: [F32;2],
	fTempPerm112: F32,
	fVec2: [F32;2],
	fTempPerm113: F32,
	iTempPerm114: i32,
	iTempPerm115: i32,
	iTempPerm116: i32,
	fTempPerm117: F32,
	fTempPerm118: F32,
	fRec28: [F32;2],
	fRec27: [F32;2],
	iTempPerm119: i32,
	iTempPerm120: i32,
	fTempPerm121: F32,
	fRec33: [F32;2],
	fTempPerm122: F32,
	fTempPerm123: F32,
	fTempPerm124: F32,
	iTempPerm125: i32,
	fTempPerm126: F32,
	fTempPerm127: F32,
	iTempPerm128: i32,
	iTempPerm129: i32,
	fTempPerm130: F32,
	fRec38: [F32;2],
	fRec37: [F32;2],
	fTempPerm131: F32,
	fTempPerm132: F32,
	iTempPerm133: i32,
	fTempPerm134: F32,
	fTempPerm135: F32,
	fTempPerm136: F32,
	iTempPerm137: i32,
	fTempPerm138: F32,
	fTempPerm139: F32,
	fRec39: [F32;2],
	fTempPerm140: F32,
	fTempPerm141: F32,
	fTempPerm142: F32,
	fTempPerm143: F32,
	fTempPerm144: F32,
	iTempPerm145: i32,
	iTempPerm146: i32,
	iTempPerm147: i32,
	iTempPerm148: i32,
	iTempPerm149: i32,
	fTempPerm150: F32,
	iTempPerm151: i32,
	iTempPerm152: i32,
	fTempPerm153: F32,
	fTempPerm154: F32,
	iTempPerm155: i32,
	iTempPerm156: i32,
	fTempPerm157: F32,
	fTempPerm158: F32,
	iTempPerm159: i32,
	fTempPerm160: F32,
	fRec44: [F32;2],
	fTempPerm161: F32,
	iTempPerm162: i32,
	iTempPerm163: i32,
	fTempPerm164: F32,
	fTempPerm165: F32,
	fTempPerm166: F32,
	fTempPerm167: F32,
	fRec42: [F32;2],
	fRec43: [F32;2],
	fTempPerm168: F32,
	fTempPerm169: F32,
	iTempPerm170: i32,
	fTempPerm171: F32,
	fTempPerm172: F32,
	fTempPerm173: F32,
	iTempPerm174: i32,
	fTempPerm175: F32,
	fTempPerm176: F32,
	fRec45: [F32;2],
	fTempPerm177: F32,
	fVec3: [F32;2],
	fTempPerm178: F32,
	iTempPerm179: i32,
	iTempPerm180: i32,
	iTempPerm181: i32,
	fTempPerm182: F32,
	fTempPerm183: F32,
	fRec41: [F32;2],
	fRec40: [F32;2],
	iTempPerm184: i32,
	iTempPerm185: i32,
	fTempPerm186: F32,
	fRec46: [F32;2],
	fTempPerm187: F32,
	fTempPerm188: F32,
	fTempPerm189: F32,
	iTempPerm190: i32,
	fTempPerm191: F32,
	fTempPerm192: F32,
	iTempPerm193: i32,
	iTempPerm194: i32,
	fTempPerm195: F32,
	fRec51: [F32;2],
	fRec50: [F32;2],
	fTempPerm196: F32,
	fTempPerm197: F32,
	iTempPerm198: i32,
	fTempPerm199: F32,
	fTempPerm200: F32,
	fTempPerm201: F32,
	iTempPerm202: i32,
	fTempPerm203: F32,
	fTempPerm204: F32,
	fRec52: [F32;2],
	fTempPerm205: F32,
	fTempPerm206: F32,
	fTempPerm207: F32,
	fTempPerm208: F32,
	fTempPerm209: F32,
	iTempPerm210: i32,
	iTempPerm211: i32,
	iTempPerm212: i32,
	iTempPerm213: i32,
	iTempPerm214: i32,
	fTempPerm215: F32,
	iTempPerm216: i32,
	iTempPerm217: i32,
	fTempPerm218: F32,
	fTempPerm219: F32,
	iTempPerm220: i32,
	iTempPerm221: i32,
	fTempPerm222: F32,
	fTempPerm223: F32,
	iTempPerm224: i32,
	fTempPerm225: F32,
	fRec57: [F32;2],
	fTempPerm226: F32,
	iTempPerm227: i32,
	iTempPerm228: i32,
	fTempPerm229: F32,
	fTempPerm230: F32,
	fTempPerm231: F32,
	fTempPerm232: F32,
	fRec55: [F32;2],
	fRec56: [F32;2],
	fTempPerm233: F32,
	fTempPerm234: F32,
	iTempPerm235: i32,
	fTempPerm236: F32,
	fTempPerm237: F32,
	fTempPerm238: F32,
	iTempPerm239: i32,
	fTempPerm240: F32,
	fTempPerm241: F32,
	fRec58: [F32;2],
	fTempPerm242: F32,
	fVec4: [F32;2],
	fTempPerm243: F32,
	iTempPerm244: i32,
	iTempPerm245: i32,
	iTempPerm246: i32,
	fTempPerm247: F32,
	fTempPerm248: F32,
	fRec54: [F32;2],
	fRec53: [F32;2],
	iTempPerm249: i32,
	iTempPerm250: i32,
	fTempPerm251: F32,
	fRec59: [F32;2],
	fTempPerm252: F32,
	fTempPerm253: F32,
	fTempPerm254: F32,
	iTempPerm255: i32,
	fTempPerm256: F32,
	fTempPerm257: F32,
	iTempPerm258: i32,
	iTempPerm259: i32,
	fTempPerm260: F32,
	fRec64: [F32;2],
	fRec63: [F32;2],
	fTempPerm261: F32,
	fTempPerm262: F32,
	iTempPerm263: i32,
	fTempPerm264: F32,
	fTempPerm265: F32,
	fTempPerm266: F32,
	iTempPerm267: i32,
	fTempPerm268: F32,
	fTempPerm269: F32,
	fRec65: [F32;2],
	fTempPerm270: F32,
	fTempPerm271: F32,
	fTempPerm272: F32,
	fTempPerm273: F32,
	fTempPerm274: F32,
	iTempPerm275: i32,
	iTempPerm276: i32,
	iTempPerm277: i32,
	iTempPerm278: i32,
	iTempPerm279: i32,
	fTempPerm280: F32,
	iTempPerm281: i32,
	iTempPerm282: i32,
	fTempPerm283: F32,
	fTempPerm284: F32,
	iTempPerm285: i32,
	iTempPerm286: i32,
	fTempPerm287: F32,
	fTempPerm288: F32,
	iTempPerm289: i32,
	fTempPerm290: F32,
	fRec70: [F32;2],
	fTempPerm291: F32,
	iTempPerm292: i32,
	iTempPerm293: i32,
	fTempPerm294: F32,
	fTempPerm295: F32,
	fTempPerm296: F32,
	fTempPerm297: F32,
	fRec68: [F32;2],
	fRec69: [F32;2],
	fTempPerm298: F32,
	fTempPerm299: F32,
	iTempPerm300: i32,
	fTempPerm301: F32,
	fTempPerm302: F32,
	fTempPerm303: F32,
	iTempPerm304: i32,
	fTempPerm305: F32,
	fTempPerm306: F32,
	fRec71: [F32;2],
	fTempPerm307: F32,
	fVec5: [F32;2],
	fTempPerm308: F32,
	iTempPerm309: i32,
	iTempPerm310: i32,
	iTempPerm311: i32,
	fTempPerm312: F32,
	fTempPerm313: F32,
	fRec67: [F32;2],
	fRec66: [F32;2],
	iTempPerm314: i32,
	iTempPerm315: i32,
	fTempPerm316: F32,
	fRec72: [F32;2],
	fTempPerm317: F32,
	fTempPerm318: F32,
	fTempPerm319: F32,
	iTempPerm320: i32,
	fTempPerm321: F32,
	fTempPerm322: F32,
	iTempPerm323: i32,
	iTempPerm324: i32,
	fTempPerm325: F32,
	fRec77: [F32;2],
	fRec76: [F32;2],
	fTempPerm326: F32,
	fTempPerm327: F32,
	iTempPerm328: i32,
	fTempPerm329: F32,
	fTempPerm330: F32,
	fTempPerm331: F32,
	iTempPerm332: i32,
	fTempPerm333: F32,
	fTempPerm334: F32,
	fRec78: [F32;2],
	fTempPerm335: F32,
	fTempPerm336: F32,
	fTempPerm337: F32,
	fTempPerm338: F32,
	fTempPerm339: F32,
	iTempPerm340: i32,
	iTempPerm341: i32,
	iTempPerm342: i32,
	iTempPerm343: i32,
	iTempPerm344: i32,
	fTempPerm345: F32,
	iTempPerm346: i32,
	iTempPerm347: i32,
	fTempPerm348: F32,
	fTempPerm349: F32,
	iTempPerm350: i32,
	iTempPerm351: i32,
	fTempPerm352: F32,
	fTempPerm353: F32,
	iTempPerm354: i32,
	fTempPerm355: F32,
	fRec83: [F32;2],
	fTempPerm356: F32,
	iTempPerm357: i32,
	iTempPerm358: i32,
	fTempPerm359: F32,
	fTempPerm360: F32,
	fTempPerm361: F32,
	fTempPerm362: F32,
	fRec81: [F32;2],
	fRec82: [F32;2],
	fTempPerm363: F32,
	fTempPerm364: F32,
	iTempPerm365: i32,
	fTempPerm366: F32,
	fTempPerm367: F32,
	fTempPerm368: F32,
	iTempPerm369: i32,
	fTempPerm370: F32,
	fTempPerm371: F32,
	fRec84: [F32;2],
	fTempPerm372: F32,
	fVec6: [F32;2],
	fTempPerm373: F32,
	iTempPerm374: i32,
	iTempPerm375: i32,
	iTempPerm376: i32,
	fTempPerm377: F32,
	fTempPerm378: F32,
	fRec80: [F32;2],
	fRec79: [F32;2],
	iTempPerm379: i32,
	iTempPerm380: i32,
	fTempPerm381: F32,
	fRec85: [F32;2],
	fTempPerm382: F32,
	fTempPerm383: F32,
	fTempPerm384: F32,
	iTempPerm385: i32,
	fTempPerm386: F32,
	fTempPerm387: F32,
	iTempPerm388: i32,
	iTempPerm389: i32,
	fTempPerm390: F32,
	fRec90: [F32;2],
	fRec89: [F32;2],
	fTempPerm391: F32,
	fTempPerm392: F32,
	iTempPerm393: i32,
	fTempPerm394: F32,
	fTempPerm395: F32,
	fTempPerm396: F32,
	iTempPerm397: i32,
	fTempPerm398: F32,
	fTempPerm399: F32,
	fRec91: [F32;2],
	fTempPerm400: F32,
	fTempPerm401: F32,
	fTempPerm402: F32,
	fTempPerm403: F32,
	fTempPerm404: F32,
	iTempPerm405: i32,
	iTempPerm406: i32,
	iTempPerm407: i32,
	iTempPerm408: i32,
	iTempPerm409: i32,
	fTempPerm410: F32,
	iTempPerm411: i32,
	iTempPerm412: i32,
	fTempPerm413: F32,
	fTempPerm414: F32,
	iTempPerm415: i32,
	iTempPerm416: i32,
	fTempPerm417: F32,
	fTempPerm418: F32,
	iTempPerm419: i32,
	fTempPerm420: F32,
	fRec96: [F32;2],
	fTempPerm421: F32,
	iTempPerm422: i32,
	iTempPerm423: i32,
	fTempPerm424: F32,
	fTempPerm425: F32,
	fTempPerm426: F32,
	fTempPerm427: F32,
	fRec94: [F32;2],
	fRec95: [F32;2],
	fTempPerm428: F32,
	fTempPerm429: F32,
	iTempPerm430: i32,
	fTempPerm431: F32,
	fTempPerm432: F32,
	fTempPerm433: F32,
	iTempPerm434: i32,
	fTempPerm435: F32,
	fTempPerm436: F32,
	fRec97: [F32;2],
	fTempPerm437: F32,
	fVec7: [F32;2],
	fTempPerm438: F32,
	iTempPerm439: i32,
	iTempPerm440: i32,
	iTempPerm441: i32,
	fTempPerm442: F32,
	fTempPerm443: F32,
	fRec93: [F32;2],
	fRec92: [F32;2],
	iTempPerm444: i32,
	iTempPerm445: i32,
	fTempPerm446: F32,
	fRec98: [F32;2],
	fTempPerm447: F32,
	fTempPerm448: F32,
	fTempPerm449: F32,
	iTempPerm450: i32,
	fTempPerm451: F32,
	fTempPerm452: F32,
	iTempPerm453: i32,
	iTempPerm454: i32,
	fTempPerm455: F32,
	fRec103: [F32;2],
	fRec102: [F32;2],
	fTempPerm456: F32,
	fTempPerm457: F32,
	iTempPerm458: i32,
	fTempPerm459: F32,
	fTempPerm460: F32,
	fTempPerm461: F32,
	iTempPerm462: i32,
	fTempPerm463: F32,
	fTempPerm464: F32,
	fRec104: [F32;2],
	fTempPerm465: F32,
	fTempPerm466: F32,
	fTempPerm467: F32,
	fTempPerm468: F32,
	fTempPerm469: F32,
	iTempPerm470: i32,
	iTempPerm471: i32,
	iTempPerm472: i32,
	iTempPerm473: i32,
	iTempPerm474: i32,
	fTempPerm475: F32,
	iTempPerm476: i32,
	iTempPerm477: i32,
	fTempPerm478: F32,
	fTempPerm479: F32,
	iTempPerm480: i32,
	iTempPerm481: i32,
	fTempPerm482: F32,
	fTempPerm483: F32,
	iTempPerm484: i32,
	fTempPerm485: F32,
	fRec109: [F32;2],
	fTempPerm486: F32,
	iTempPerm487: i32,
	iTempPerm488: i32,
	fTempPerm489: F32,
	fTempPerm490: F32,
	fTempPerm491: F32,
	fTempPerm492: F32,
	fRec107: [F32;2],
	fRec108: [F32;2],
	fTempPerm493: F32,
	fTempPerm494: F32,
	iTempPerm495: i32,
	fTempPerm496: F32,
	fTempPerm497: F32,
	fTempPerm498: F32,
	iTempPerm499: i32,
	fTempPerm500: F32,
	fTempPerm501: F32,
	fRec110: [F32;2],
	fTempPerm502: F32,
	fVec8: [F32;2],
	fTempPerm503: F32,
	iTempPerm504: i32,
	iTempPerm505: i32,
	iTempPerm506: i32,
	fTempPerm507: F32,
	fTempPerm508: F32,
	fRec106: [F32;2],
	fRec105: [F32;2],
	iTempPerm509: i32,
	iTempPerm510: i32,
	fTempPerm511: F32,
	fRec111: [F32;2],
	fTempPerm512: F32,
	fTempPerm513: F32,
	fTempPerm514: F32,
	iTempPerm515: i32,
	fTempPerm516: F32,
	fTempPerm517: F32,
	iTempPerm518: i32,
	iTempPerm519: i32,
	fTempPerm520: F32,
	fRec116: [F32;2],
	fRec115: [F32;2],
	fTempPerm521: F32,
	fTempPerm522: F32,
	iTempPerm523: i32,
	fTempPerm524: F32,
	fTempPerm525: F32,
	fTempPerm526: F32,
	iTempPerm527: i32,
	fTempPerm528: F32,
	fTempPerm529: F32,
	fRec117: [F32;2],
	fTempPerm530: F32,
	fTempPerm531: F32,
	fTempPerm532: F32,
	fTempPerm533: F32,
	fTempPerm534: F32,
	iTempPerm535: i32,
	iTempPerm536: i32,
	iTempPerm537: i32,
	iTempPerm538: i32,
	iTempPerm539: i32,
	fTempPerm540: F32,
	iTempPerm541: i32,
	iTempPerm542: i32,
	fTempPerm543: F32,
	fTempPerm544: F32,
	iTempPerm545: i32,
	iTempPerm546: i32,
	fTempPerm547: F32,
	fTempPerm548: F32,
	iTempPerm549: i32,
	fTempPerm550: F32,
	fRec122: [F32;2],
	fTempPerm551: F32,
	iTempPerm552: i32,
	iTempPerm553: i32,
	fTempPerm554: F32,
	fTempPerm555: F32,
	fTempPerm556: F32,
	fTempPerm557: F32,
	fRec120: [F32;2],
	fRec121: [F32;2],
	fTempPerm558: F32,
	fTempPerm559: F32,
	iTempPerm560: i32,
	fTempPerm561: F32,
	fTempPerm562: F32,
	fTempPerm563: F32,
	iTempPerm564: i32,
	fTempPerm565: F32,
	fTempPerm566: F32,
	fRec123: [F32;2],
	fTempPerm567: F32,
	fVec9: [F32;2],
	fTempPerm568: F32,
	iTempPerm569: i32,
	iTempPerm570: i32,
	iTempPerm571: i32,
	fTempPerm572: F32,
	fTempPerm573: F32,
	fRec119: [F32;2],
	fRec118: [F32;2],
	iTempPerm574: i32,
	iTempPerm575: i32,
	fTempPerm576: F32,
	fRec124: [F32;2],
	fTempPerm577: F32,
	fTempPerm578: F32,
	fTempPerm579: F32,
	iTempPerm580: i32,
	fTempPerm581: F32,
	fTempPerm582: F32,
	iTempPerm583: i32,
	iTempPerm584: i32,
	fTempPerm585: F32,
	fRec129: [F32;2],
	fRec128: [F32;2],
	fTempPerm586: F32,
	fTempPerm587: F32,
	iTempPerm588: i32,
	fTempPerm589: F32,
	fTempPerm590: F32,
	fTempPerm591: F32,
	iTempPerm592: i32,
	fTempPerm593: F32,
	fTempPerm594: F32,
	fRec130: [F32;2],
	fTempPerm595: F32,
	fTempPerm596: F32,
	fTempPerm597: F32,
	fTempPerm598: F32,
	fTempPerm599: F32,
	iTempPerm600: i32,
	iTempPerm601: i32,
	iTempPerm602: i32,
	iTempPerm603: i32,
	iTempPerm604: i32,
	fTempPerm605: F32,
	iTempPerm606: i32,
	iTempPerm607: i32,
	fTempPerm608: F32,
	fTempPerm609: F32,
	iTempPerm610: i32,
	iTempPerm611: i32,
	fTempPerm612: F32,
	fTempPerm613: F32,
	iTempPerm614: i32,
	fTempPerm615: F32,
	fRec135: [F32;2],
	fTempPerm616: F32,
	iTempPerm617: i32,
	iTempPerm618: i32,
	fTempPerm619: F32,
	fTempPerm620: F32,
	fTempPerm621: F32,
	fTempPerm622: F32,
	fRec133: [F32;2],
	fRec134: [F32;2],
	fTempPerm623: F32,
	fTempPerm624: F32,
	iTempPerm625: i32,
	fTempPerm626: F32,
	fTempPerm627: F32,
	fTempPerm628: F32,
	iTempPerm629: i32,
	fTempPerm630: F32,
	fTempPerm631: F32,
	fRec136: [F32;2],
	fTempPerm632: F32,
	fVec10: [F32;2],
	fTempPerm633: F32,
	iTempPerm634: i32,
	iTempPerm635: i32,
	iTempPerm636: i32,
	fTempPerm637: F32,
	fTempPerm638: F32,
	fRec132: [F32;2],
	fRec131: [F32;2],
	iTempPerm639: i32,
	iTempPerm640: i32,
	fTempPerm641: F32,
	fRec137: [F32;2],
	fTempPerm642: F32,
	fTempPerm643: F32,
	fTempPerm644: F32,
	iTempPerm645: i32,
	fTempPerm646: F32,
	fTempPerm647: F32,
	iTempPerm648: i32,
	iTempPerm649: i32,
	fTempPerm650: F32,
	fRec142: [F32;2],
	fRec141: [F32;2],
	fTempPerm651: F32,
	fTempPerm652: F32,
	iTempPerm653: i32,
	fTempPerm654: F32,
	fTempPerm655: F32,
	fTempPerm656: F32,
	iTempPerm657: i32,
	fTempPerm658: F32,
	fTempPerm659: F32,
	fRec143: [F32;2],
	fTempPerm660: F32,
	fTempPerm661: F32,
	fTempPerm662: F32,
	fTempPerm663: F32,
	fTempPerm664: F32,
	iTempPerm665: i32,
	iTempPerm666: i32,
	iTempPerm667: i32,
	iTempPerm668: i32,
	iTempPerm669: i32,
	fTempPerm670: F32,
	iTempPerm671: i32,
	iTempPerm672: i32,
	fTempPerm673: F32,
	fTempPerm674: F32,
	iTempPerm675: i32,
	iTempPerm676: i32,
	fTempPerm677: F32,
	fTempPerm678: F32,
	iTempPerm679: i32,
	fTempPerm680: F32,
	fRec148: [F32;2],
	fTempPerm681: F32,
	iTempPerm682: i32,
	iTempPerm683: i32,
	fTempPerm684: F32,
	fTempPerm685: F32,
	fTempPerm686: F32,
	fTempPerm687: F32,
	fRec146: [F32;2],
	fRec147: [F32;2],
	fTempPerm688: F32,
	fTempPerm689: F32,
	iTempPerm690: i32,
	fTempPerm691: F32,
	fTempPerm692: F32,
	fTempPerm693: F32,
	iTempPerm694: i32,
	fTempPerm695: F32,
	fTempPerm696: F32,
	fRec149: [F32;2],
	fTempPerm697: F32,
	fVec11: [F32;2],
	fTempPerm698: F32,
	iTempPerm699: i32,
	iTempPerm700: i32,
	iTempPerm701: i32,
	fTempPerm702: F32,
	fTempPerm703: F32,
	fRec145: [F32;2],
	fRec144: [F32;2],
	iTempPerm704: i32,
	iTempPerm705: i32,
	fTempPerm706: F32,
	fRec150: [F32;2],
	fTempPerm707: F32,
	fTempPerm708: F32,
	fTempPerm709: F32,
	iTempPerm710: i32,
	fTempPerm711: F32,
	fTempPerm712: F32,
	iTempPerm713: i32,
	iTempPerm714: i32,
	fTempPerm715: F32,
	fRec155: [F32;2],
	fRec154: [F32;2],
	fTempPerm716: F32,
	fTempPerm717: F32,
	iTempPerm718: i32,
	fTempPerm719: F32,
	fTempPerm720: F32,
	fTempPerm721: F32,
	iTempPerm722: i32,
	fTempPerm723: F32,
	fTempPerm724: F32,
	fRec156: [F32;2],
	fTempPerm725: F32,
	fTempPerm726: F32,
	fTempPerm727: F32,
	fTempPerm728: F32,
	fTempPerm729: F32,
	iTempPerm730: i32,
	iTempPerm731: i32,
	iTempPerm732: i32,
	iTempPerm733: i32,
	iTempPerm734: i32,
	fTempPerm735: F32,
	iTempPerm736: i32,
	fTempPerm737: F32,
	fTempPerm738: F32,
	iTempPerm739: i32,
	iTempPerm740: i32,
	fTempPerm741: F32,
	fTempPerm742: F32,
	iTempPerm743: i32,
	fTempPerm744: F32,
	fRec161: [F32;2],
	fTempPerm745: F32,
	iTempPerm746: i32,
	iTempPerm747: i32,
	fTempPerm748: F32,
	fTempPerm749: F32,
	fTempPerm750: F32,
	fTempPerm751: F32,
	fRec159: [F32;2],
	fRec160: [F32;2],
	fTempPerm752: F32,
	fTempPerm753: F32,
	iTempPerm754: i32,
	fTempPerm755: F32,
	fTempPerm756: F32,
	fTempPerm757: F32,
	iTempPerm758: i32,
	fTempPerm759: F32,
	fTempPerm760: F32,
	fRec162: [F32;2],
	fTempPerm761: F32,
	fVec12: [F32;2],
	fTempPerm762: F32,
	iTempPerm763: i32,
	iTempPerm764: i32,
	iTempPerm765: i32,
	fTempPerm766: F32,
	fTempPerm767: F32,
	fRec158: [F32;2],
	fRec157: [F32;2],
	iTempPerm768: i32,
	iTempPerm769: i32,
	fTempPerm770: F32,
	fRec163: [F32;2],
	fTempPerm771: F32,
	fTempPerm772: F32,
	fTempPerm773: F32,
	iTempPerm774: i32,
	fTempPerm775: F32,
	fTempPerm776: F32,
	iTempPerm777: i32,
	iTempPerm778: i32,
	fTempPerm779: F32,
	fRec168: [F32;2],
	fRec167: [F32;2],
	fTempPerm780: F32,
	fTempPerm781: F32,
	iTempPerm782: i32,
	fTempPerm783: F32,
	fTempPerm784: F32,
	fTempPerm785: F32,
	iTempPerm786: i32,
	fTempPerm787: F32,
	fTempPerm788: F32,
	fRec169: [F32;2],
	fTempPerm789: F32,
	fTempPerm790: F32,
	fTempPerm791: F32,
	fTempPerm792: F32,
	fTempPerm793: F32,
	fTempPerm794: F32,
	fTempPerm795: F32,
	fRec164: [F32;2],
	fTempPerm796: F32,
	fConst13: F32,
	fRec165: [F32;2],
	fTempPerm797: F32,
	fTempPerm798: F32,
	fTempPerm799: F32,
	fTempPerm800: F32,
	fRec151: [F32;2],
	fTempPerm801: F32,
	fRec152: [F32;2],
	fTempPerm802: F32,
	fTempPerm803: F32,
	fTempPerm804: F32,
	fTempPerm805: F32,
	fRec138: [F32;2],
	fTempPerm806: F32,
	fRec139: [F32;2],
	fTempPerm807: F32,
	fTempPerm808: F32,
	fTempPerm809: F32,
	fTempPerm810: F32,
	fRec125: [F32;2],
	fTempPerm811: F32,
	fRec126: [F32;2],
	fTempPerm812: F32,
	fTempPerm813: F32,
	fTempPerm814: F32,
	fTempPerm815: F32,
	fRec112: [F32;2],
	fTempPerm816: F32,
	fRec113: [F32;2],
	fTempPerm817: F32,
	fTempPerm818: F32,
	fTempPerm819: F32,
	fTempPerm820: F32,
	fRec99: [F32;2],
	fTempPerm821: F32,
	fRec100: [F32;2],
	fTempPerm822: F32,
	fTempPerm823: F32,
	fTempPerm824: F32,
	fTempPerm825: F32,
	fRec86: [F32;2],
	fTempPerm826: F32,
	fRec87: [F32;2],
	fTempPerm827: F32,
	fTempPerm828: F32,
	fTempPerm829: F32,
	fTempPerm830: F32,
	fRec73: [F32;2],
	fTempPerm831: F32,
	fRec74: [F32;2],
	fTempPerm832: F32,
	fTempPerm833: F32,
	fTempPerm834: F32,
	fTempPerm835: F32,
	fRec60: [F32;2],
	fTempPerm836: F32,
	fRec61: [F32;2],
	fTempPerm837: F32,
	fTempPerm838: F32,
	fTempPerm839: F32,
	fTempPerm840: F32,
	fRec47: [F32;2],
	fTempPerm841: F32,
	fRec48: [F32;2],
	fTempPerm842: F32,
	fTempPerm843: F32,
	fTempPerm844: F32,
	fTempPerm845: F32,
	fRec34: [F32;2],
	fTempPerm846: F32,
	fRec35: [F32;2],
	fTempPerm847: F32,
	fTempPerm848: F32,
	fTempPerm849: F32,
	fTempPerm850: F32,
	fRec20: [F32;2],
	fTempPerm851: F32,
	fRec21: [F32;2],
	fTempPerm852: F32,
	fTempPerm853: F32,
	fConst14: F32,
	fConst15: F32,
	fRec170: [F32;2],
	fRec171: [F32;2],
	fHslider20: F32,
	fRec172: [F32;2],
	fTempPerm854: F32,
	fTempPerm855: F32,
	fRec206: [F32;2],
	fRec207: [F32;2],
	fTempPerm856: F32,
	fTempPerm857: F32,
	fTempPerm858: F32,
	fRec203: [F32;2],
	fRec204: [F32;2],
	fTempPerm859: F32,
	fTempPerm860: F32,
	fTempPerm861: F32,
	fRec200: [F32;2],
	fRec201: [F32;2],
	fTempPerm862: F32,
	fTempPerm863: F32,
	fTempPerm864: F32,
	fRec197: [F32;2],
	fRec198: [F32;2],
	fTempPerm865: F32,
	fTempPerm866: F32,
	fTempPerm867: F32,
	fRec194: [F32;2],
	fRec195: [F32;2],
	fTempPerm868: F32,
	fTempPerm869: F32,
	fTempPerm870: F32,
	fRec191: [F32;2],
	fRec192: [F32;2],
	fTempPerm871: F32,
	fTempPerm872: F32,
	fTempPerm873: F32,
	fRec188: [F32;2],
	fRec189: [F32;2],
	fTempPerm874: F32,
	fTempPerm875: F32,
	fTempPerm876: F32,
	fRec185: [F32;2],
	fRec186: [F32;2],
	fTempPerm877: F32,
	fTempPerm878: F32,
	fTempPerm879: F32,
	fRec182: [F32;2],
	fRec183: [F32;2],
	fTempPerm880: F32,
	fTempPerm881: F32,
	fTempPerm882: F32,
	fRec179: [F32;2],
	fRec180: [F32;2],
	fTempPerm883: F32,
	fTempPerm884: F32,
	fTempPerm885: F32,
	fRec176: [F32;2],
	fRec177: [F32;2],
	fTempPerm886: F32,
	fTempPerm887: F32,
	fTempPerm888: F32,
	fRec173: [F32;2],
	fRec174: [F32;2],
	fTempPerm889: F32,
	fRec209: [F32;2],
	fRec210: [F32;2],
}

impl FaustDsp for AlternativesDsp {
	type T = F32;
		
	fn new() -> AlternativesDsp { 
		AlternativesDsp {
			fHslider0: 0.0,
			iVec0: [0;2],
			fRec0: [0.0;2],
			fCheckbox0: 0.0,
			fHslider1: 0.0,
			fHslider2: 0.0,
			fRec1: [0.0;2],
			fSampleRate: 0,
			fConst1: 0.0,
			iTempPerm0: 0,
			iTempPerm1: 0,
			fConst2: 0.0,
			fRec4: [0.0;2],
			fHslider3: 0.0,
			iRec5: [0;2],
			iTempPerm2: 0,
			fRec3: [0.0;2],
			fConst3: 0.0,
			fRec2: [0.0;2],
			fHslider4: 0.0,
			fRec6: [0.0;2],
			fTempPerm3: 0.0,
			fConst4: 0.0,
			iTempPerm4: 0,
			iTempPerm5: 0,
			fHslider5: 0.0,
			fRec12: [0.0;2],
			fTempPerm6: 0.0,
			fHslider6: 0.0,
			iRec13: [0;2],
			iTempPerm7: 0,
			iTempPerm8: 0,
			iTempPerm9: 0,
			fHslider7: 0.0,
			fHslider8: 0.0,
			fHslider9: 0.0,
			fHslider10: 0.0,
			fHslider11: 0.0,
			fHslider12: 0.0,
			fHslider13: 0.0,
			fHslider14: 0.0,
			fTempPerm10: 0.0,
			fConst5: 0.0,
			iTempPerm11: 0,
			iTempPerm12: 0,
			fTempPerm13: 0.0,
			fTempPerm14: 0.0,
			iTempPerm15: 0,
			iTempPerm16: 0,
			fTempPerm17: 0.0,
			fTempPerm18: 0.0,
			iTempPerm19: 0,
			fTempPerm20: 0.0,
			fHslider15: 0.0,
			fRec14: [0.0;2],
			fTempPerm21: 0.0,
			fRec11: [0.0;2],
			fHslider16: 0.0,
			fRec15: [0.0;2],
			fTempPerm22: 0.0,
			fTempPerm23: 0.0,
			fConst6: 0.0,
			fTempPerm24: 0.0,
			iTempPerm25: 0,
			iTempPerm26: 0,
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fTempPerm29: 0.0,
			fTempPerm30: 0.0,
			fRec9: [0.0;2],
			fRec10: [0.0;2],
			fTempPerm31: 0.0,
			fTempPerm32: 0.0,
			iTempPerm33: 0,
			fTempPerm34: 0.0,
			fTempPerm35: 0.0,
			fTempPerm36: 0.0,
			iTempPerm37: 0,
			fTempPerm38: 0.0,
			fTempPerm39: 0.0,
			fRec16: [0.0;2],
			fTempPerm40: 0.0,
			fVec1: [0.0;2],
			fTempPerm41: 0.0,
			iTempPerm42: 0,
			iTempPerm43: 0,
			iTempPerm44: 0,
			fTempPerm45: 0.0,
			fTempPerm46: 0.0,
			fRec8: [0.0;2],
			fConst7: 0.0,
			fRec7: [0.0;2],
			iTempPerm47: 0,
			iTempPerm48: 0,
			fTempPerm49: 0.0,
			fRec17: [0.0;2],
			fTempPerm50: 0.0,
			fHslider17: 0.0,
			fRec18: [0.0;2],
			fTempPerm51: 0.0,
			fTempPerm52: 0.0,
			fTempPerm53: 0.0,
			fHslider18: 0.0,
			fRec19: [0.0;2],
			fTempPerm54: 0.0,
			fTempPerm55: 0.0,
			fTempPerm56: 0.0,
			iTempPerm57: 0,
			fTempPerm58: 0.0,
			fTempPerm59: 0.0,
			fHslider19: 0.0,
			fRec23: [0.0;2],
			fTempPerm60: 0.0,
			fTempPerm61: 0.0,
			iTempPerm62: 0,
			iTempPerm63: 0,
			fTempPerm64: 0.0,
			fRec25: [0.0;2],
			fRec24: [0.0;2],
			fTempPerm65: 0.0,
			fTempPerm66: 0.0,
			iTempPerm67: 0,
			fTempPerm68: 0.0,
			fTempPerm69: 0.0,
			fTempPerm70: 0.0,
			iTempPerm71: 0,
			fTempPerm72: 0.0,
			fTempPerm73: 0.0,
			fRec26: [0.0;2],
			fTempPerm74: 0.0,
			fTempPerm75: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fTempPerm76: 0.0,
			fConst11: 0.0,
			fTempPerm77: 0.0,
			fConst12: 0.0,
			fTempPerm78: 0.0,
			fTempPerm79: 0.0,
			iTempPerm80: 0,
			iTempPerm81: 0,
			iTempPerm82: 0,
			iTempPerm83: 0,
			iTempPerm84: 0,
			fTempPerm85: 0.0,
			iTempPerm86: 0,
			iTempPerm87: 0,
			fTempPerm88: 0.0,
			fTempPerm89: 0.0,
			iTempPerm90: 0,
			iTempPerm91: 0,
			fTempPerm92: 0.0,
			fTempPerm93: 0.0,
			iTempPerm94: 0,
			fTempPerm95: 0.0,
			fRec31: [0.0;2],
			fTempPerm96: 0.0,
			iTempPerm97: 0,
			iTempPerm98: 0,
			fTempPerm99: 0.0,
			fTempPerm100: 0.0,
			fTempPerm101: 0.0,
			fTempPerm102: 0.0,
			fRec29: [0.0;2],
			fRec30: [0.0;2],
			fTempPerm103: 0.0,
			fTempPerm104: 0.0,
			iTempPerm105: 0,
			fTempPerm106: 0.0,
			fTempPerm107: 0.0,
			fTempPerm108: 0.0,
			iTempPerm109: 0,
			fTempPerm110: 0.0,
			fTempPerm111: 0.0,
			fRec32: [0.0;2],
			fTempPerm112: 0.0,
			fVec2: [0.0;2],
			fTempPerm113: 0.0,
			iTempPerm114: 0,
			iTempPerm115: 0,
			iTempPerm116: 0,
			fTempPerm117: 0.0,
			fTempPerm118: 0.0,
			fRec28: [0.0;2],
			fRec27: [0.0;2],
			iTempPerm119: 0,
			iTempPerm120: 0,
			fTempPerm121: 0.0,
			fRec33: [0.0;2],
			fTempPerm122: 0.0,
			fTempPerm123: 0.0,
			fTempPerm124: 0.0,
			iTempPerm125: 0,
			fTempPerm126: 0.0,
			fTempPerm127: 0.0,
			iTempPerm128: 0,
			iTempPerm129: 0,
			fTempPerm130: 0.0,
			fRec38: [0.0;2],
			fRec37: [0.0;2],
			fTempPerm131: 0.0,
			fTempPerm132: 0.0,
			iTempPerm133: 0,
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			fTempPerm136: 0.0,
			iTempPerm137: 0,
			fTempPerm138: 0.0,
			fTempPerm139: 0.0,
			fRec39: [0.0;2],
			fTempPerm140: 0.0,
			fTempPerm141: 0.0,
			fTempPerm142: 0.0,
			fTempPerm143: 0.0,
			fTempPerm144: 0.0,
			iTempPerm145: 0,
			iTempPerm146: 0,
			iTempPerm147: 0,
			iTempPerm148: 0,
			iTempPerm149: 0,
			fTempPerm150: 0.0,
			iTempPerm151: 0,
			iTempPerm152: 0,
			fTempPerm153: 0.0,
			fTempPerm154: 0.0,
			iTempPerm155: 0,
			iTempPerm156: 0,
			fTempPerm157: 0.0,
			fTempPerm158: 0.0,
			iTempPerm159: 0,
			fTempPerm160: 0.0,
			fRec44: [0.0;2],
			fTempPerm161: 0.0,
			iTempPerm162: 0,
			iTempPerm163: 0,
			fTempPerm164: 0.0,
			fTempPerm165: 0.0,
			fTempPerm166: 0.0,
			fTempPerm167: 0.0,
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fTempPerm168: 0.0,
			fTempPerm169: 0.0,
			iTempPerm170: 0,
			fTempPerm171: 0.0,
			fTempPerm172: 0.0,
			fTempPerm173: 0.0,
			iTempPerm174: 0,
			fTempPerm175: 0.0,
			fTempPerm176: 0.0,
			fRec45: [0.0;2],
			fTempPerm177: 0.0,
			fVec3: [0.0;2],
			fTempPerm178: 0.0,
			iTempPerm179: 0,
			iTempPerm180: 0,
			iTempPerm181: 0,
			fTempPerm182: 0.0,
			fTempPerm183: 0.0,
			fRec41: [0.0;2],
			fRec40: [0.0;2],
			iTempPerm184: 0,
			iTempPerm185: 0,
			fTempPerm186: 0.0,
			fRec46: [0.0;2],
			fTempPerm187: 0.0,
			fTempPerm188: 0.0,
			fTempPerm189: 0.0,
			iTempPerm190: 0,
			fTempPerm191: 0.0,
			fTempPerm192: 0.0,
			iTempPerm193: 0,
			iTempPerm194: 0,
			fTempPerm195: 0.0,
			fRec51: [0.0;2],
			fRec50: [0.0;2],
			fTempPerm196: 0.0,
			fTempPerm197: 0.0,
			iTempPerm198: 0,
			fTempPerm199: 0.0,
			fTempPerm200: 0.0,
			fTempPerm201: 0.0,
			iTempPerm202: 0,
			fTempPerm203: 0.0,
			fTempPerm204: 0.0,
			fRec52: [0.0;2],
			fTempPerm205: 0.0,
			fTempPerm206: 0.0,
			fTempPerm207: 0.0,
			fTempPerm208: 0.0,
			fTempPerm209: 0.0,
			iTempPerm210: 0,
			iTempPerm211: 0,
			iTempPerm212: 0,
			iTempPerm213: 0,
			iTempPerm214: 0,
			fTempPerm215: 0.0,
			iTempPerm216: 0,
			iTempPerm217: 0,
			fTempPerm218: 0.0,
			fTempPerm219: 0.0,
			iTempPerm220: 0,
			iTempPerm221: 0,
			fTempPerm222: 0.0,
			fTempPerm223: 0.0,
			iTempPerm224: 0,
			fTempPerm225: 0.0,
			fRec57: [0.0;2],
			fTempPerm226: 0.0,
			iTempPerm227: 0,
			iTempPerm228: 0,
			fTempPerm229: 0.0,
			fTempPerm230: 0.0,
			fTempPerm231: 0.0,
			fTempPerm232: 0.0,
			fRec55: [0.0;2],
			fRec56: [0.0;2],
			fTempPerm233: 0.0,
			fTempPerm234: 0.0,
			iTempPerm235: 0,
			fTempPerm236: 0.0,
			fTempPerm237: 0.0,
			fTempPerm238: 0.0,
			iTempPerm239: 0,
			fTempPerm240: 0.0,
			fTempPerm241: 0.0,
			fRec58: [0.0;2],
			fTempPerm242: 0.0,
			fVec4: [0.0;2],
			fTempPerm243: 0.0,
			iTempPerm244: 0,
			iTempPerm245: 0,
			iTempPerm246: 0,
			fTempPerm247: 0.0,
			fTempPerm248: 0.0,
			fRec54: [0.0;2],
			fRec53: [0.0;2],
			iTempPerm249: 0,
			iTempPerm250: 0,
			fTempPerm251: 0.0,
			fRec59: [0.0;2],
			fTempPerm252: 0.0,
			fTempPerm253: 0.0,
			fTempPerm254: 0.0,
			iTempPerm255: 0,
			fTempPerm256: 0.0,
			fTempPerm257: 0.0,
			iTempPerm258: 0,
			iTempPerm259: 0,
			fTempPerm260: 0.0,
			fRec64: [0.0;2],
			fRec63: [0.0;2],
			fTempPerm261: 0.0,
			fTempPerm262: 0.0,
			iTempPerm263: 0,
			fTempPerm264: 0.0,
			fTempPerm265: 0.0,
			fTempPerm266: 0.0,
			iTempPerm267: 0,
			fTempPerm268: 0.0,
			fTempPerm269: 0.0,
			fRec65: [0.0;2],
			fTempPerm270: 0.0,
			fTempPerm271: 0.0,
			fTempPerm272: 0.0,
			fTempPerm273: 0.0,
			fTempPerm274: 0.0,
			iTempPerm275: 0,
			iTempPerm276: 0,
			iTempPerm277: 0,
			iTempPerm278: 0,
			iTempPerm279: 0,
			fTempPerm280: 0.0,
			iTempPerm281: 0,
			iTempPerm282: 0,
			fTempPerm283: 0.0,
			fTempPerm284: 0.0,
			iTempPerm285: 0,
			iTempPerm286: 0,
			fTempPerm287: 0.0,
			fTempPerm288: 0.0,
			iTempPerm289: 0,
			fTempPerm290: 0.0,
			fRec70: [0.0;2],
			fTempPerm291: 0.0,
			iTempPerm292: 0,
			iTempPerm293: 0,
			fTempPerm294: 0.0,
			fTempPerm295: 0.0,
			fTempPerm296: 0.0,
			fTempPerm297: 0.0,
			fRec68: [0.0;2],
			fRec69: [0.0;2],
			fTempPerm298: 0.0,
			fTempPerm299: 0.0,
			iTempPerm300: 0,
			fTempPerm301: 0.0,
			fTempPerm302: 0.0,
			fTempPerm303: 0.0,
			iTempPerm304: 0,
			fTempPerm305: 0.0,
			fTempPerm306: 0.0,
			fRec71: [0.0;2],
			fTempPerm307: 0.0,
			fVec5: [0.0;2],
			fTempPerm308: 0.0,
			iTempPerm309: 0,
			iTempPerm310: 0,
			iTempPerm311: 0,
			fTempPerm312: 0.0,
			fTempPerm313: 0.0,
			fRec67: [0.0;2],
			fRec66: [0.0;2],
			iTempPerm314: 0,
			iTempPerm315: 0,
			fTempPerm316: 0.0,
			fRec72: [0.0;2],
			fTempPerm317: 0.0,
			fTempPerm318: 0.0,
			fTempPerm319: 0.0,
			iTempPerm320: 0,
			fTempPerm321: 0.0,
			fTempPerm322: 0.0,
			iTempPerm323: 0,
			iTempPerm324: 0,
			fTempPerm325: 0.0,
			fRec77: [0.0;2],
			fRec76: [0.0;2],
			fTempPerm326: 0.0,
			fTempPerm327: 0.0,
			iTempPerm328: 0,
			fTempPerm329: 0.0,
			fTempPerm330: 0.0,
			fTempPerm331: 0.0,
			iTempPerm332: 0,
			fTempPerm333: 0.0,
			fTempPerm334: 0.0,
			fRec78: [0.0;2],
			fTempPerm335: 0.0,
			fTempPerm336: 0.0,
			fTempPerm337: 0.0,
			fTempPerm338: 0.0,
			fTempPerm339: 0.0,
			iTempPerm340: 0,
			iTempPerm341: 0,
			iTempPerm342: 0,
			iTempPerm343: 0,
			iTempPerm344: 0,
			fTempPerm345: 0.0,
			iTempPerm346: 0,
			iTempPerm347: 0,
			fTempPerm348: 0.0,
			fTempPerm349: 0.0,
			iTempPerm350: 0,
			iTempPerm351: 0,
			fTempPerm352: 0.0,
			fTempPerm353: 0.0,
			iTempPerm354: 0,
			fTempPerm355: 0.0,
			fRec83: [0.0;2],
			fTempPerm356: 0.0,
			iTempPerm357: 0,
			iTempPerm358: 0,
			fTempPerm359: 0.0,
			fTempPerm360: 0.0,
			fTempPerm361: 0.0,
			fTempPerm362: 0.0,
			fRec81: [0.0;2],
			fRec82: [0.0;2],
			fTempPerm363: 0.0,
			fTempPerm364: 0.0,
			iTempPerm365: 0,
			fTempPerm366: 0.0,
			fTempPerm367: 0.0,
			fTempPerm368: 0.0,
			iTempPerm369: 0,
			fTempPerm370: 0.0,
			fTempPerm371: 0.0,
			fRec84: [0.0;2],
			fTempPerm372: 0.0,
			fVec6: [0.0;2],
			fTempPerm373: 0.0,
			iTempPerm374: 0,
			iTempPerm375: 0,
			iTempPerm376: 0,
			fTempPerm377: 0.0,
			fTempPerm378: 0.0,
			fRec80: [0.0;2],
			fRec79: [0.0;2],
			iTempPerm379: 0,
			iTempPerm380: 0,
			fTempPerm381: 0.0,
			fRec85: [0.0;2],
			fTempPerm382: 0.0,
			fTempPerm383: 0.0,
			fTempPerm384: 0.0,
			iTempPerm385: 0,
			fTempPerm386: 0.0,
			fTempPerm387: 0.0,
			iTempPerm388: 0,
			iTempPerm389: 0,
			fTempPerm390: 0.0,
			fRec90: [0.0;2],
			fRec89: [0.0;2],
			fTempPerm391: 0.0,
			fTempPerm392: 0.0,
			iTempPerm393: 0,
			fTempPerm394: 0.0,
			fTempPerm395: 0.0,
			fTempPerm396: 0.0,
			iTempPerm397: 0,
			fTempPerm398: 0.0,
			fTempPerm399: 0.0,
			fRec91: [0.0;2],
			fTempPerm400: 0.0,
			fTempPerm401: 0.0,
			fTempPerm402: 0.0,
			fTempPerm403: 0.0,
			fTempPerm404: 0.0,
			iTempPerm405: 0,
			iTempPerm406: 0,
			iTempPerm407: 0,
			iTempPerm408: 0,
			iTempPerm409: 0,
			fTempPerm410: 0.0,
			iTempPerm411: 0,
			iTempPerm412: 0,
			fTempPerm413: 0.0,
			fTempPerm414: 0.0,
			iTempPerm415: 0,
			iTempPerm416: 0,
			fTempPerm417: 0.0,
			fTempPerm418: 0.0,
			iTempPerm419: 0,
			fTempPerm420: 0.0,
			fRec96: [0.0;2],
			fTempPerm421: 0.0,
			iTempPerm422: 0,
			iTempPerm423: 0,
			fTempPerm424: 0.0,
			fTempPerm425: 0.0,
			fTempPerm426: 0.0,
			fTempPerm427: 0.0,
			fRec94: [0.0;2],
			fRec95: [0.0;2],
			fTempPerm428: 0.0,
			fTempPerm429: 0.0,
			iTempPerm430: 0,
			fTempPerm431: 0.0,
			fTempPerm432: 0.0,
			fTempPerm433: 0.0,
			iTempPerm434: 0,
			fTempPerm435: 0.0,
			fTempPerm436: 0.0,
			fRec97: [0.0;2],
			fTempPerm437: 0.0,
			fVec7: [0.0;2],
			fTempPerm438: 0.0,
			iTempPerm439: 0,
			iTempPerm440: 0,
			iTempPerm441: 0,
			fTempPerm442: 0.0,
			fTempPerm443: 0.0,
			fRec93: [0.0;2],
			fRec92: [0.0;2],
			iTempPerm444: 0,
			iTempPerm445: 0,
			fTempPerm446: 0.0,
			fRec98: [0.0;2],
			fTempPerm447: 0.0,
			fTempPerm448: 0.0,
			fTempPerm449: 0.0,
			iTempPerm450: 0,
			fTempPerm451: 0.0,
			fTempPerm452: 0.0,
			iTempPerm453: 0,
			iTempPerm454: 0,
			fTempPerm455: 0.0,
			fRec103: [0.0;2],
			fRec102: [0.0;2],
			fTempPerm456: 0.0,
			fTempPerm457: 0.0,
			iTempPerm458: 0,
			fTempPerm459: 0.0,
			fTempPerm460: 0.0,
			fTempPerm461: 0.0,
			iTempPerm462: 0,
			fTempPerm463: 0.0,
			fTempPerm464: 0.0,
			fRec104: [0.0;2],
			fTempPerm465: 0.0,
			fTempPerm466: 0.0,
			fTempPerm467: 0.0,
			fTempPerm468: 0.0,
			fTempPerm469: 0.0,
			iTempPerm470: 0,
			iTempPerm471: 0,
			iTempPerm472: 0,
			iTempPerm473: 0,
			iTempPerm474: 0,
			fTempPerm475: 0.0,
			iTempPerm476: 0,
			iTempPerm477: 0,
			fTempPerm478: 0.0,
			fTempPerm479: 0.0,
			iTempPerm480: 0,
			iTempPerm481: 0,
			fTempPerm482: 0.0,
			fTempPerm483: 0.0,
			iTempPerm484: 0,
			fTempPerm485: 0.0,
			fRec109: [0.0;2],
			fTempPerm486: 0.0,
			iTempPerm487: 0,
			iTempPerm488: 0,
			fTempPerm489: 0.0,
			fTempPerm490: 0.0,
			fTempPerm491: 0.0,
			fTempPerm492: 0.0,
			fRec107: [0.0;2],
			fRec108: [0.0;2],
			fTempPerm493: 0.0,
			fTempPerm494: 0.0,
			iTempPerm495: 0,
			fTempPerm496: 0.0,
			fTempPerm497: 0.0,
			fTempPerm498: 0.0,
			iTempPerm499: 0,
			fTempPerm500: 0.0,
			fTempPerm501: 0.0,
			fRec110: [0.0;2],
			fTempPerm502: 0.0,
			fVec8: [0.0;2],
			fTempPerm503: 0.0,
			iTempPerm504: 0,
			iTempPerm505: 0,
			iTempPerm506: 0,
			fTempPerm507: 0.0,
			fTempPerm508: 0.0,
			fRec106: [0.0;2],
			fRec105: [0.0;2],
			iTempPerm509: 0,
			iTempPerm510: 0,
			fTempPerm511: 0.0,
			fRec111: [0.0;2],
			fTempPerm512: 0.0,
			fTempPerm513: 0.0,
			fTempPerm514: 0.0,
			iTempPerm515: 0,
			fTempPerm516: 0.0,
			fTempPerm517: 0.0,
			iTempPerm518: 0,
			iTempPerm519: 0,
			fTempPerm520: 0.0,
			fRec116: [0.0;2],
			fRec115: [0.0;2],
			fTempPerm521: 0.0,
			fTempPerm522: 0.0,
			iTempPerm523: 0,
			fTempPerm524: 0.0,
			fTempPerm525: 0.0,
			fTempPerm526: 0.0,
			iTempPerm527: 0,
			fTempPerm528: 0.0,
			fTempPerm529: 0.0,
			fRec117: [0.0;2],
			fTempPerm530: 0.0,
			fTempPerm531: 0.0,
			fTempPerm532: 0.0,
			fTempPerm533: 0.0,
			fTempPerm534: 0.0,
			iTempPerm535: 0,
			iTempPerm536: 0,
			iTempPerm537: 0,
			iTempPerm538: 0,
			iTempPerm539: 0,
			fTempPerm540: 0.0,
			iTempPerm541: 0,
			iTempPerm542: 0,
			fTempPerm543: 0.0,
			fTempPerm544: 0.0,
			iTempPerm545: 0,
			iTempPerm546: 0,
			fTempPerm547: 0.0,
			fTempPerm548: 0.0,
			iTempPerm549: 0,
			fTempPerm550: 0.0,
			fRec122: [0.0;2],
			fTempPerm551: 0.0,
			iTempPerm552: 0,
			iTempPerm553: 0,
			fTempPerm554: 0.0,
			fTempPerm555: 0.0,
			fTempPerm556: 0.0,
			fTempPerm557: 0.0,
			fRec120: [0.0;2],
			fRec121: [0.0;2],
			fTempPerm558: 0.0,
			fTempPerm559: 0.0,
			iTempPerm560: 0,
			fTempPerm561: 0.0,
			fTempPerm562: 0.0,
			fTempPerm563: 0.0,
			iTempPerm564: 0,
			fTempPerm565: 0.0,
			fTempPerm566: 0.0,
			fRec123: [0.0;2],
			fTempPerm567: 0.0,
			fVec9: [0.0;2],
			fTempPerm568: 0.0,
			iTempPerm569: 0,
			iTempPerm570: 0,
			iTempPerm571: 0,
			fTempPerm572: 0.0,
			fTempPerm573: 0.0,
			fRec119: [0.0;2],
			fRec118: [0.0;2],
			iTempPerm574: 0,
			iTempPerm575: 0,
			fTempPerm576: 0.0,
			fRec124: [0.0;2],
			fTempPerm577: 0.0,
			fTempPerm578: 0.0,
			fTempPerm579: 0.0,
			iTempPerm580: 0,
			fTempPerm581: 0.0,
			fTempPerm582: 0.0,
			iTempPerm583: 0,
			iTempPerm584: 0,
			fTempPerm585: 0.0,
			fRec129: [0.0;2],
			fRec128: [0.0;2],
			fTempPerm586: 0.0,
			fTempPerm587: 0.0,
			iTempPerm588: 0,
			fTempPerm589: 0.0,
			fTempPerm590: 0.0,
			fTempPerm591: 0.0,
			iTempPerm592: 0,
			fTempPerm593: 0.0,
			fTempPerm594: 0.0,
			fRec130: [0.0;2],
			fTempPerm595: 0.0,
			fTempPerm596: 0.0,
			fTempPerm597: 0.0,
			fTempPerm598: 0.0,
			fTempPerm599: 0.0,
			iTempPerm600: 0,
			iTempPerm601: 0,
			iTempPerm602: 0,
			iTempPerm603: 0,
			iTempPerm604: 0,
			fTempPerm605: 0.0,
			iTempPerm606: 0,
			iTempPerm607: 0,
			fTempPerm608: 0.0,
			fTempPerm609: 0.0,
			iTempPerm610: 0,
			iTempPerm611: 0,
			fTempPerm612: 0.0,
			fTempPerm613: 0.0,
			iTempPerm614: 0,
			fTempPerm615: 0.0,
			fRec135: [0.0;2],
			fTempPerm616: 0.0,
			iTempPerm617: 0,
			iTempPerm618: 0,
			fTempPerm619: 0.0,
			fTempPerm620: 0.0,
			fTempPerm621: 0.0,
			fTempPerm622: 0.0,
			fRec133: [0.0;2],
			fRec134: [0.0;2],
			fTempPerm623: 0.0,
			fTempPerm624: 0.0,
			iTempPerm625: 0,
			fTempPerm626: 0.0,
			fTempPerm627: 0.0,
			fTempPerm628: 0.0,
			iTempPerm629: 0,
			fTempPerm630: 0.0,
			fTempPerm631: 0.0,
			fRec136: [0.0;2],
			fTempPerm632: 0.0,
			fVec10: [0.0;2],
			fTempPerm633: 0.0,
			iTempPerm634: 0,
			iTempPerm635: 0,
			iTempPerm636: 0,
			fTempPerm637: 0.0,
			fTempPerm638: 0.0,
			fRec132: [0.0;2],
			fRec131: [0.0;2],
			iTempPerm639: 0,
			iTempPerm640: 0,
			fTempPerm641: 0.0,
			fRec137: [0.0;2],
			fTempPerm642: 0.0,
			fTempPerm643: 0.0,
			fTempPerm644: 0.0,
			iTempPerm645: 0,
			fTempPerm646: 0.0,
			fTempPerm647: 0.0,
			iTempPerm648: 0,
			iTempPerm649: 0,
			fTempPerm650: 0.0,
			fRec142: [0.0;2],
			fRec141: [0.0;2],
			fTempPerm651: 0.0,
			fTempPerm652: 0.0,
			iTempPerm653: 0,
			fTempPerm654: 0.0,
			fTempPerm655: 0.0,
			fTempPerm656: 0.0,
			iTempPerm657: 0,
			fTempPerm658: 0.0,
			fTempPerm659: 0.0,
			fRec143: [0.0;2],
			fTempPerm660: 0.0,
			fTempPerm661: 0.0,
			fTempPerm662: 0.0,
			fTempPerm663: 0.0,
			fTempPerm664: 0.0,
			iTempPerm665: 0,
			iTempPerm666: 0,
			iTempPerm667: 0,
			iTempPerm668: 0,
			iTempPerm669: 0,
			fTempPerm670: 0.0,
			iTempPerm671: 0,
			iTempPerm672: 0,
			fTempPerm673: 0.0,
			fTempPerm674: 0.0,
			iTempPerm675: 0,
			iTempPerm676: 0,
			fTempPerm677: 0.0,
			fTempPerm678: 0.0,
			iTempPerm679: 0,
			fTempPerm680: 0.0,
			fRec148: [0.0;2],
			fTempPerm681: 0.0,
			iTempPerm682: 0,
			iTempPerm683: 0,
			fTempPerm684: 0.0,
			fTempPerm685: 0.0,
			fTempPerm686: 0.0,
			fTempPerm687: 0.0,
			fRec146: [0.0;2],
			fRec147: [0.0;2],
			fTempPerm688: 0.0,
			fTempPerm689: 0.0,
			iTempPerm690: 0,
			fTempPerm691: 0.0,
			fTempPerm692: 0.0,
			fTempPerm693: 0.0,
			iTempPerm694: 0,
			fTempPerm695: 0.0,
			fTempPerm696: 0.0,
			fRec149: [0.0;2],
			fTempPerm697: 0.0,
			fVec11: [0.0;2],
			fTempPerm698: 0.0,
			iTempPerm699: 0,
			iTempPerm700: 0,
			iTempPerm701: 0,
			fTempPerm702: 0.0,
			fTempPerm703: 0.0,
			fRec145: [0.0;2],
			fRec144: [0.0;2],
			iTempPerm704: 0,
			iTempPerm705: 0,
			fTempPerm706: 0.0,
			fRec150: [0.0;2],
			fTempPerm707: 0.0,
			fTempPerm708: 0.0,
			fTempPerm709: 0.0,
			iTempPerm710: 0,
			fTempPerm711: 0.0,
			fTempPerm712: 0.0,
			iTempPerm713: 0,
			iTempPerm714: 0,
			fTempPerm715: 0.0,
			fRec155: [0.0;2],
			fRec154: [0.0;2],
			fTempPerm716: 0.0,
			fTempPerm717: 0.0,
			iTempPerm718: 0,
			fTempPerm719: 0.0,
			fTempPerm720: 0.0,
			fTempPerm721: 0.0,
			iTempPerm722: 0,
			fTempPerm723: 0.0,
			fTempPerm724: 0.0,
			fRec156: [0.0;2],
			fTempPerm725: 0.0,
			fTempPerm726: 0.0,
			fTempPerm727: 0.0,
			fTempPerm728: 0.0,
			fTempPerm729: 0.0,
			iTempPerm730: 0,
			iTempPerm731: 0,
			iTempPerm732: 0,
			iTempPerm733: 0,
			iTempPerm734: 0,
			fTempPerm735: 0.0,
			iTempPerm736: 0,
			fTempPerm737: 0.0,
			fTempPerm738: 0.0,
			iTempPerm739: 0,
			iTempPerm740: 0,
			fTempPerm741: 0.0,
			fTempPerm742: 0.0,
			iTempPerm743: 0,
			fTempPerm744: 0.0,
			fRec161: [0.0;2],
			fTempPerm745: 0.0,
			iTempPerm746: 0,
			iTempPerm747: 0,
			fTempPerm748: 0.0,
			fTempPerm749: 0.0,
			fTempPerm750: 0.0,
			fTempPerm751: 0.0,
			fRec159: [0.0;2],
			fRec160: [0.0;2],
			fTempPerm752: 0.0,
			fTempPerm753: 0.0,
			iTempPerm754: 0,
			fTempPerm755: 0.0,
			fTempPerm756: 0.0,
			fTempPerm757: 0.0,
			iTempPerm758: 0,
			fTempPerm759: 0.0,
			fTempPerm760: 0.0,
			fRec162: [0.0;2],
			fTempPerm761: 0.0,
			fVec12: [0.0;2],
			fTempPerm762: 0.0,
			iTempPerm763: 0,
			iTempPerm764: 0,
			iTempPerm765: 0,
			fTempPerm766: 0.0,
			fTempPerm767: 0.0,
			fRec158: [0.0;2],
			fRec157: [0.0;2],
			iTempPerm768: 0,
			iTempPerm769: 0,
			fTempPerm770: 0.0,
			fRec163: [0.0;2],
			fTempPerm771: 0.0,
			fTempPerm772: 0.0,
			fTempPerm773: 0.0,
			iTempPerm774: 0,
			fTempPerm775: 0.0,
			fTempPerm776: 0.0,
			iTempPerm777: 0,
			iTempPerm778: 0,
			fTempPerm779: 0.0,
			fRec168: [0.0;2],
			fRec167: [0.0;2],
			fTempPerm780: 0.0,
			fTempPerm781: 0.0,
			iTempPerm782: 0,
			fTempPerm783: 0.0,
			fTempPerm784: 0.0,
			fTempPerm785: 0.0,
			iTempPerm786: 0,
			fTempPerm787: 0.0,
			fTempPerm788: 0.0,
			fRec169: [0.0;2],
			fTempPerm789: 0.0,
			fTempPerm790: 0.0,
			fTempPerm791: 0.0,
			fTempPerm792: 0.0,
			fTempPerm793: 0.0,
			fTempPerm794: 0.0,
			fTempPerm795: 0.0,
			fRec164: [0.0;2],
			fTempPerm796: 0.0,
			fConst13: 0.0,
			fRec165: [0.0;2],
			fTempPerm797: 0.0,
			fTempPerm798: 0.0,
			fTempPerm799: 0.0,
			fTempPerm800: 0.0,
			fRec151: [0.0;2],
			fTempPerm801: 0.0,
			fRec152: [0.0;2],
			fTempPerm802: 0.0,
			fTempPerm803: 0.0,
			fTempPerm804: 0.0,
			fTempPerm805: 0.0,
			fRec138: [0.0;2],
			fTempPerm806: 0.0,
			fRec139: [0.0;2],
			fTempPerm807: 0.0,
			fTempPerm808: 0.0,
			fTempPerm809: 0.0,
			fTempPerm810: 0.0,
			fRec125: [0.0;2],
			fTempPerm811: 0.0,
			fRec126: [0.0;2],
			fTempPerm812: 0.0,
			fTempPerm813: 0.0,
			fTempPerm814: 0.0,
			fTempPerm815: 0.0,
			fRec112: [0.0;2],
			fTempPerm816: 0.0,
			fRec113: [0.0;2],
			fTempPerm817: 0.0,
			fTempPerm818: 0.0,
			fTempPerm819: 0.0,
			fTempPerm820: 0.0,
			fRec99: [0.0;2],
			fTempPerm821: 0.0,
			fRec100: [0.0;2],
			fTempPerm822: 0.0,
			fTempPerm823: 0.0,
			fTempPerm824: 0.0,
			fTempPerm825: 0.0,
			fRec86: [0.0;2],
			fTempPerm826: 0.0,
			fRec87: [0.0;2],
			fTempPerm827: 0.0,
			fTempPerm828: 0.0,
			fTempPerm829: 0.0,
			fTempPerm830: 0.0,
			fRec73: [0.0;2],
			fTempPerm831: 0.0,
			fRec74: [0.0;2],
			fTempPerm832: 0.0,
			fTempPerm833: 0.0,
			fTempPerm834: 0.0,
			fTempPerm835: 0.0,
			fRec60: [0.0;2],
			fTempPerm836: 0.0,
			fRec61: [0.0;2],
			fTempPerm837: 0.0,
			fTempPerm838: 0.0,
			fTempPerm839: 0.0,
			fTempPerm840: 0.0,
			fRec47: [0.0;2],
			fTempPerm841: 0.0,
			fRec48: [0.0;2],
			fTempPerm842: 0.0,
			fTempPerm843: 0.0,
			fTempPerm844: 0.0,
			fTempPerm845: 0.0,
			fRec34: [0.0;2],
			fTempPerm846: 0.0,
			fRec35: [0.0;2],
			fTempPerm847: 0.0,
			fTempPerm848: 0.0,
			fTempPerm849: 0.0,
			fTempPerm850: 0.0,
			fRec20: [0.0;2],
			fTempPerm851: 0.0,
			fRec21: [0.0;2],
			fTempPerm852: 0.0,
			fTempPerm853: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fRec170: [0.0;2],
			fRec171: [0.0;2],
			fHslider20: 0.0,
			fRec172: [0.0;2],
			fTempPerm854: 0.0,
			fTempPerm855: 0.0,
			fRec206: [0.0;2],
			fRec207: [0.0;2],
			fTempPerm856: 0.0,
			fTempPerm857: 0.0,
			fTempPerm858: 0.0,
			fRec203: [0.0;2],
			fRec204: [0.0;2],
			fTempPerm859: 0.0,
			fTempPerm860: 0.0,
			fTempPerm861: 0.0,
			fRec200: [0.0;2],
			fRec201: [0.0;2],
			fTempPerm862: 0.0,
			fTempPerm863: 0.0,
			fTempPerm864: 0.0,
			fRec197: [0.0;2],
			fRec198: [0.0;2],
			fTempPerm865: 0.0,
			fTempPerm866: 0.0,
			fTempPerm867: 0.0,
			fRec194: [0.0;2],
			fRec195: [0.0;2],
			fTempPerm868: 0.0,
			fTempPerm869: 0.0,
			fTempPerm870: 0.0,
			fRec191: [0.0;2],
			fRec192: [0.0;2],
			fTempPerm871: 0.0,
			fTempPerm872: 0.0,
			fTempPerm873: 0.0,
			fRec188: [0.0;2],
			fRec189: [0.0;2],
			fTempPerm874: 0.0,
			fTempPerm875: 0.0,
			fTempPerm876: 0.0,
			fRec185: [0.0;2],
			fRec186: [0.0;2],
			fTempPerm877: 0.0,
			fTempPerm878: 0.0,
			fTempPerm879: 0.0,
			fRec182: [0.0;2],
			fRec183: [0.0;2],
			fTempPerm880: 0.0,
			fTempPerm881: 0.0,
			fTempPerm882: 0.0,
			fRec179: [0.0;2],
			fRec180: [0.0;2],
			fTempPerm883: 0.0,
			fTempPerm884: 0.0,
			fTempPerm885: 0.0,
			fRec176: [0.0;2],
			fRec177: [0.0;2],
			fTempPerm886: 0.0,
			fTempPerm887: 0.0,
			fTempPerm888: 0.0,
			fRec173: [0.0;2],
			fRec174: [0.0;2],
			fTempPerm889: 0.0,
			fRec209: [0.0;2],
			fRec210: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn AlternativesDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("filename", r"plugin.dsp");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"plugin");
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
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 0.0;
		self.fCheckbox0 = 0.0;
		self.fHslider1 = 6.0;
		self.fHslider2 = 0.6;
		self.fHslider3 = 1e+03;
		self.fHslider4 = 0.275;
		self.fHslider5 = 64.0;
		self.fHslider6 = 1.0;
		self.fHslider7 = 130.81;
		self.fHslider8 = 130.81;
		self.fHslider9 = 130.81;
		self.fHslider10 = 130.81;
		self.fHslider11 = 130.81;
		self.fHslider12 = 130.81;
		self.fHslider13 = 130.81;
		self.fHslider14 = 130.81;
		self.fHslider15 = 2.0;
		self.fHslider16 = 8e+01;
		self.fHslider17 = 6.0;
		self.fHslider18 = 3.33;
		self.fHslider19 = 1.0;
		self.fHslider20 = 1.0;
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
		self.iTempPerm0 = 0;
		self.iTempPerm1 = 0;
		for l3 in 0..2 {
			self.fRec4[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.iRec5[l4 as usize] = 0;
		}
		self.iTempPerm2 = 0;
		for l5 in 0..2 {
			self.fRec3[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec2[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec6[l7 as usize] = 0.0;
		}
		self.fTempPerm3 = 0.0;
		self.iTempPerm4 = 0;
		self.iTempPerm5 = 0;
		for l8 in 0..2 {
			self.fRec12[l8 as usize] = 0.0;
		}
		self.fTempPerm6 = 0.0;
		for l9 in 0..2 {
			self.iRec13[l9 as usize] = 0;
		}
		self.iTempPerm7 = 0;
		self.iTempPerm8 = 0;
		self.iTempPerm9 = 0;
		self.fTempPerm10 = 0.0;
		self.iTempPerm11 = 0;
		self.iTempPerm12 = 0;
		self.fTempPerm13 = 0.0;
		self.fTempPerm14 = 0.0;
		self.iTempPerm15 = 0;
		self.iTempPerm16 = 0;
		self.fTempPerm17 = 0.0;
		self.fTempPerm18 = 0.0;
		self.iTempPerm19 = 0;
		self.fTempPerm20 = 0.0;
		for l10 in 0..2 {
			self.fRec14[l10 as usize] = 0.0;
		}
		self.fTempPerm21 = 0.0;
		for l11 in 0..2 {
			self.fRec11[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec15[l12 as usize] = 0.0;
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
		for l13 in 0..2 {
			self.fRec9[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec10[l14 as usize] = 0.0;
		}
		self.fTempPerm31 = 0.0;
		self.fTempPerm32 = 0.0;
		self.iTempPerm33 = 0;
		self.fTempPerm34 = 0.0;
		self.fTempPerm35 = 0.0;
		self.fTempPerm36 = 0.0;
		self.iTempPerm37 = 0;
		self.fTempPerm38 = 0.0;
		self.fTempPerm39 = 0.0;
		for l15 in 0..2 {
			self.fRec16[l15 as usize] = 0.0;
		}
		self.fTempPerm40 = 0.0;
		for l16 in 0..2 {
			self.fVec1[l16 as usize] = 0.0;
		}
		self.fTempPerm41 = 0.0;
		self.iTempPerm42 = 0;
		self.iTempPerm43 = 0;
		self.iTempPerm44 = 0;
		self.fTempPerm45 = 0.0;
		self.fTempPerm46 = 0.0;
		for l17 in 0..2 {
			self.fRec8[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec7[l18 as usize] = 0.0;
		}
		self.iTempPerm47 = 0;
		self.iTempPerm48 = 0;
		self.fTempPerm49 = 0.0;
		for l19 in 0..2 {
			self.fRec17[l19 as usize] = 0.0;
		}
		self.fTempPerm50 = 0.0;
		for l20 in 0..2 {
			self.fRec18[l20 as usize] = 0.0;
		}
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		for l21 in 0..2 {
			self.fRec19[l21 as usize] = 0.0;
		}
		self.fTempPerm54 = 0.0;
		self.fTempPerm55 = 0.0;
		self.fTempPerm56 = 0.0;
		self.iTempPerm57 = 0;
		self.fTempPerm58 = 0.0;
		self.fTempPerm59 = 0.0;
		for l22 in 0..2 {
			self.fRec23[l22 as usize] = 0.0;
		}
		self.fTempPerm60 = 0.0;
		self.fTempPerm61 = 0.0;
		self.iTempPerm62 = 0;
		self.iTempPerm63 = 0;
		self.fTempPerm64 = 0.0;
		for l23 in 0..2 {
			self.fRec25[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec24[l24 as usize] = 0.0;
		}
		self.fTempPerm65 = 0.0;
		self.fTempPerm66 = 0.0;
		self.iTempPerm67 = 0;
		self.fTempPerm68 = 0.0;
		self.fTempPerm69 = 0.0;
		self.fTempPerm70 = 0.0;
		self.iTempPerm71 = 0;
		self.fTempPerm72 = 0.0;
		self.fTempPerm73 = 0.0;
		for l25 in 0..2 {
			self.fRec26[l25 as usize] = 0.0;
		}
		self.fTempPerm74 = 0.0;
		self.fTempPerm75 = 0.0;
		self.fTempPerm76 = 0.0;
		self.fTempPerm77 = 0.0;
		self.fTempPerm78 = 0.0;
		self.fTempPerm79 = 0.0;
		self.iTempPerm80 = 0;
		self.iTempPerm81 = 0;
		self.iTempPerm82 = 0;
		self.iTempPerm83 = 0;
		self.iTempPerm84 = 0;
		self.fTempPerm85 = 0.0;
		self.iTempPerm86 = 0;
		self.iTempPerm87 = 0;
		self.fTempPerm88 = 0.0;
		self.fTempPerm89 = 0.0;
		self.iTempPerm90 = 0;
		self.iTempPerm91 = 0;
		self.fTempPerm92 = 0.0;
		self.fTempPerm93 = 0.0;
		self.iTempPerm94 = 0;
		self.fTempPerm95 = 0.0;
		for l26 in 0..2 {
			self.fRec31[l26 as usize] = 0.0;
		}
		self.fTempPerm96 = 0.0;
		self.iTempPerm97 = 0;
		self.iTempPerm98 = 0;
		self.fTempPerm99 = 0.0;
		self.fTempPerm100 = 0.0;
		self.fTempPerm101 = 0.0;
		self.fTempPerm102 = 0.0;
		for l27 in 0..2 {
			self.fRec29[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec30[l28 as usize] = 0.0;
		}
		self.fTempPerm103 = 0.0;
		self.fTempPerm104 = 0.0;
		self.iTempPerm105 = 0;
		self.fTempPerm106 = 0.0;
		self.fTempPerm107 = 0.0;
		self.fTempPerm108 = 0.0;
		self.iTempPerm109 = 0;
		self.fTempPerm110 = 0.0;
		self.fTempPerm111 = 0.0;
		for l29 in 0..2 {
			self.fRec32[l29 as usize] = 0.0;
		}
		self.fTempPerm112 = 0.0;
		for l30 in 0..2 {
			self.fVec2[l30 as usize] = 0.0;
		}
		self.fTempPerm113 = 0.0;
		self.iTempPerm114 = 0;
		self.iTempPerm115 = 0;
		self.iTempPerm116 = 0;
		self.fTempPerm117 = 0.0;
		self.fTempPerm118 = 0.0;
		for l31 in 0..2 {
			self.fRec28[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec27[l32 as usize] = 0.0;
		}
		self.iTempPerm119 = 0;
		self.iTempPerm120 = 0;
		self.fTempPerm121 = 0.0;
		for l33 in 0..2 {
			self.fRec33[l33 as usize] = 0.0;
		}
		self.fTempPerm122 = 0.0;
		self.fTempPerm123 = 0.0;
		self.fTempPerm124 = 0.0;
		self.iTempPerm125 = 0;
		self.fTempPerm126 = 0.0;
		self.fTempPerm127 = 0.0;
		self.iTempPerm128 = 0;
		self.iTempPerm129 = 0;
		self.fTempPerm130 = 0.0;
		for l34 in 0..2 {
			self.fRec38[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec37[l35 as usize] = 0.0;
		}
		self.fTempPerm131 = 0.0;
		self.fTempPerm132 = 0.0;
		self.iTempPerm133 = 0;
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		self.fTempPerm136 = 0.0;
		self.iTempPerm137 = 0;
		self.fTempPerm138 = 0.0;
		self.fTempPerm139 = 0.0;
		for l36 in 0..2 {
			self.fRec39[l36 as usize] = 0.0;
		}
		self.fTempPerm140 = 0.0;
		self.fTempPerm141 = 0.0;
		self.fTempPerm142 = 0.0;
		self.fTempPerm143 = 0.0;
		self.fTempPerm144 = 0.0;
		self.iTempPerm145 = 0;
		self.iTempPerm146 = 0;
		self.iTempPerm147 = 0;
		self.iTempPerm148 = 0;
		self.iTempPerm149 = 0;
		self.fTempPerm150 = 0.0;
		self.iTempPerm151 = 0;
		self.iTempPerm152 = 0;
		self.fTempPerm153 = 0.0;
		self.fTempPerm154 = 0.0;
		self.iTempPerm155 = 0;
		self.iTempPerm156 = 0;
		self.fTempPerm157 = 0.0;
		self.fTempPerm158 = 0.0;
		self.iTempPerm159 = 0;
		self.fTempPerm160 = 0.0;
		for l37 in 0..2 {
			self.fRec44[l37 as usize] = 0.0;
		}
		self.fTempPerm161 = 0.0;
		self.iTempPerm162 = 0;
		self.iTempPerm163 = 0;
		self.fTempPerm164 = 0.0;
		self.fTempPerm165 = 0.0;
		self.fTempPerm166 = 0.0;
		self.fTempPerm167 = 0.0;
		for l38 in 0..2 {
			self.fRec42[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec43[l39 as usize] = 0.0;
		}
		self.fTempPerm168 = 0.0;
		self.fTempPerm169 = 0.0;
		self.iTempPerm170 = 0;
		self.fTempPerm171 = 0.0;
		self.fTempPerm172 = 0.0;
		self.fTempPerm173 = 0.0;
		self.iTempPerm174 = 0;
		self.fTempPerm175 = 0.0;
		self.fTempPerm176 = 0.0;
		for l40 in 0..2 {
			self.fRec45[l40 as usize] = 0.0;
		}
		self.fTempPerm177 = 0.0;
		for l41 in 0..2 {
			self.fVec3[l41 as usize] = 0.0;
		}
		self.fTempPerm178 = 0.0;
		self.iTempPerm179 = 0;
		self.iTempPerm180 = 0;
		self.iTempPerm181 = 0;
		self.fTempPerm182 = 0.0;
		self.fTempPerm183 = 0.0;
		for l42 in 0..2 {
			self.fRec41[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec40[l43 as usize] = 0.0;
		}
		self.iTempPerm184 = 0;
		self.iTempPerm185 = 0;
		self.fTempPerm186 = 0.0;
		for l44 in 0..2 {
			self.fRec46[l44 as usize] = 0.0;
		}
		self.fTempPerm187 = 0.0;
		self.fTempPerm188 = 0.0;
		self.fTempPerm189 = 0.0;
		self.iTempPerm190 = 0;
		self.fTempPerm191 = 0.0;
		self.fTempPerm192 = 0.0;
		self.iTempPerm193 = 0;
		self.iTempPerm194 = 0;
		self.fTempPerm195 = 0.0;
		for l45 in 0..2 {
			self.fRec51[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec50[l46 as usize] = 0.0;
		}
		self.fTempPerm196 = 0.0;
		self.fTempPerm197 = 0.0;
		self.iTempPerm198 = 0;
		self.fTempPerm199 = 0.0;
		self.fTempPerm200 = 0.0;
		self.fTempPerm201 = 0.0;
		self.iTempPerm202 = 0;
		self.fTempPerm203 = 0.0;
		self.fTempPerm204 = 0.0;
		for l47 in 0..2 {
			self.fRec52[l47 as usize] = 0.0;
		}
		self.fTempPerm205 = 0.0;
		self.fTempPerm206 = 0.0;
		self.fTempPerm207 = 0.0;
		self.fTempPerm208 = 0.0;
		self.fTempPerm209 = 0.0;
		self.iTempPerm210 = 0;
		self.iTempPerm211 = 0;
		self.iTempPerm212 = 0;
		self.iTempPerm213 = 0;
		self.iTempPerm214 = 0;
		self.fTempPerm215 = 0.0;
		self.iTempPerm216 = 0;
		self.iTempPerm217 = 0;
		self.fTempPerm218 = 0.0;
		self.fTempPerm219 = 0.0;
		self.iTempPerm220 = 0;
		self.iTempPerm221 = 0;
		self.fTempPerm222 = 0.0;
		self.fTempPerm223 = 0.0;
		self.iTempPerm224 = 0;
		self.fTempPerm225 = 0.0;
		for l48 in 0..2 {
			self.fRec57[l48 as usize] = 0.0;
		}
		self.fTempPerm226 = 0.0;
		self.iTempPerm227 = 0;
		self.iTempPerm228 = 0;
		self.fTempPerm229 = 0.0;
		self.fTempPerm230 = 0.0;
		self.fTempPerm231 = 0.0;
		self.fTempPerm232 = 0.0;
		for l49 in 0..2 {
			self.fRec55[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec56[l50 as usize] = 0.0;
		}
		self.fTempPerm233 = 0.0;
		self.fTempPerm234 = 0.0;
		self.iTempPerm235 = 0;
		self.fTempPerm236 = 0.0;
		self.fTempPerm237 = 0.0;
		self.fTempPerm238 = 0.0;
		self.iTempPerm239 = 0;
		self.fTempPerm240 = 0.0;
		self.fTempPerm241 = 0.0;
		for l51 in 0..2 {
			self.fRec58[l51 as usize] = 0.0;
		}
		self.fTempPerm242 = 0.0;
		for l52 in 0..2 {
			self.fVec4[l52 as usize] = 0.0;
		}
		self.fTempPerm243 = 0.0;
		self.iTempPerm244 = 0;
		self.iTempPerm245 = 0;
		self.iTempPerm246 = 0;
		self.fTempPerm247 = 0.0;
		self.fTempPerm248 = 0.0;
		for l53 in 0..2 {
			self.fRec54[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec53[l54 as usize] = 0.0;
		}
		self.iTempPerm249 = 0;
		self.iTempPerm250 = 0;
		self.fTempPerm251 = 0.0;
		for l55 in 0..2 {
			self.fRec59[l55 as usize] = 0.0;
		}
		self.fTempPerm252 = 0.0;
		self.fTempPerm253 = 0.0;
		self.fTempPerm254 = 0.0;
		self.iTempPerm255 = 0;
		self.fTempPerm256 = 0.0;
		self.fTempPerm257 = 0.0;
		self.iTempPerm258 = 0;
		self.iTempPerm259 = 0;
		self.fTempPerm260 = 0.0;
		for l56 in 0..2 {
			self.fRec64[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec63[l57 as usize] = 0.0;
		}
		self.fTempPerm261 = 0.0;
		self.fTempPerm262 = 0.0;
		self.iTempPerm263 = 0;
		self.fTempPerm264 = 0.0;
		self.fTempPerm265 = 0.0;
		self.fTempPerm266 = 0.0;
		self.iTempPerm267 = 0;
		self.fTempPerm268 = 0.0;
		self.fTempPerm269 = 0.0;
		for l58 in 0..2 {
			self.fRec65[l58 as usize] = 0.0;
		}
		self.fTempPerm270 = 0.0;
		self.fTempPerm271 = 0.0;
		self.fTempPerm272 = 0.0;
		self.fTempPerm273 = 0.0;
		self.fTempPerm274 = 0.0;
		self.iTempPerm275 = 0;
		self.iTempPerm276 = 0;
		self.iTempPerm277 = 0;
		self.iTempPerm278 = 0;
		self.iTempPerm279 = 0;
		self.fTempPerm280 = 0.0;
		self.iTempPerm281 = 0;
		self.iTempPerm282 = 0;
		self.fTempPerm283 = 0.0;
		self.fTempPerm284 = 0.0;
		self.iTempPerm285 = 0;
		self.iTempPerm286 = 0;
		self.fTempPerm287 = 0.0;
		self.fTempPerm288 = 0.0;
		self.iTempPerm289 = 0;
		self.fTempPerm290 = 0.0;
		for l59 in 0..2 {
			self.fRec70[l59 as usize] = 0.0;
		}
		self.fTempPerm291 = 0.0;
		self.iTempPerm292 = 0;
		self.iTempPerm293 = 0;
		self.fTempPerm294 = 0.0;
		self.fTempPerm295 = 0.0;
		self.fTempPerm296 = 0.0;
		self.fTempPerm297 = 0.0;
		for l60 in 0..2 {
			self.fRec68[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec69[l61 as usize] = 0.0;
		}
		self.fTempPerm298 = 0.0;
		self.fTempPerm299 = 0.0;
		self.iTempPerm300 = 0;
		self.fTempPerm301 = 0.0;
		self.fTempPerm302 = 0.0;
		self.fTempPerm303 = 0.0;
		self.iTempPerm304 = 0;
		self.fTempPerm305 = 0.0;
		self.fTempPerm306 = 0.0;
		for l62 in 0..2 {
			self.fRec71[l62 as usize] = 0.0;
		}
		self.fTempPerm307 = 0.0;
		for l63 in 0..2 {
			self.fVec5[l63 as usize] = 0.0;
		}
		self.fTempPerm308 = 0.0;
		self.iTempPerm309 = 0;
		self.iTempPerm310 = 0;
		self.iTempPerm311 = 0;
		self.fTempPerm312 = 0.0;
		self.fTempPerm313 = 0.0;
		for l64 in 0..2 {
			self.fRec67[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec66[l65 as usize] = 0.0;
		}
		self.iTempPerm314 = 0;
		self.iTempPerm315 = 0;
		self.fTempPerm316 = 0.0;
		for l66 in 0..2 {
			self.fRec72[l66 as usize] = 0.0;
		}
		self.fTempPerm317 = 0.0;
		self.fTempPerm318 = 0.0;
		self.fTempPerm319 = 0.0;
		self.iTempPerm320 = 0;
		self.fTempPerm321 = 0.0;
		self.fTempPerm322 = 0.0;
		self.iTempPerm323 = 0;
		self.iTempPerm324 = 0;
		self.fTempPerm325 = 0.0;
		for l67 in 0..2 {
			self.fRec77[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec76[l68 as usize] = 0.0;
		}
		self.fTempPerm326 = 0.0;
		self.fTempPerm327 = 0.0;
		self.iTempPerm328 = 0;
		self.fTempPerm329 = 0.0;
		self.fTempPerm330 = 0.0;
		self.fTempPerm331 = 0.0;
		self.iTempPerm332 = 0;
		self.fTempPerm333 = 0.0;
		self.fTempPerm334 = 0.0;
		for l69 in 0..2 {
			self.fRec78[l69 as usize] = 0.0;
		}
		self.fTempPerm335 = 0.0;
		self.fTempPerm336 = 0.0;
		self.fTempPerm337 = 0.0;
		self.fTempPerm338 = 0.0;
		self.fTempPerm339 = 0.0;
		self.iTempPerm340 = 0;
		self.iTempPerm341 = 0;
		self.iTempPerm342 = 0;
		self.iTempPerm343 = 0;
		self.iTempPerm344 = 0;
		self.fTempPerm345 = 0.0;
		self.iTempPerm346 = 0;
		self.iTempPerm347 = 0;
		self.fTempPerm348 = 0.0;
		self.fTempPerm349 = 0.0;
		self.iTempPerm350 = 0;
		self.iTempPerm351 = 0;
		self.fTempPerm352 = 0.0;
		self.fTempPerm353 = 0.0;
		self.iTempPerm354 = 0;
		self.fTempPerm355 = 0.0;
		for l70 in 0..2 {
			self.fRec83[l70 as usize] = 0.0;
		}
		self.fTempPerm356 = 0.0;
		self.iTempPerm357 = 0;
		self.iTempPerm358 = 0;
		self.fTempPerm359 = 0.0;
		self.fTempPerm360 = 0.0;
		self.fTempPerm361 = 0.0;
		self.fTempPerm362 = 0.0;
		for l71 in 0..2 {
			self.fRec81[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec82[l72 as usize] = 0.0;
		}
		self.fTempPerm363 = 0.0;
		self.fTempPerm364 = 0.0;
		self.iTempPerm365 = 0;
		self.fTempPerm366 = 0.0;
		self.fTempPerm367 = 0.0;
		self.fTempPerm368 = 0.0;
		self.iTempPerm369 = 0;
		self.fTempPerm370 = 0.0;
		self.fTempPerm371 = 0.0;
		for l73 in 0..2 {
			self.fRec84[l73 as usize] = 0.0;
		}
		self.fTempPerm372 = 0.0;
		for l74 in 0..2 {
			self.fVec6[l74 as usize] = 0.0;
		}
		self.fTempPerm373 = 0.0;
		self.iTempPerm374 = 0;
		self.iTempPerm375 = 0;
		self.iTempPerm376 = 0;
		self.fTempPerm377 = 0.0;
		self.fTempPerm378 = 0.0;
		for l75 in 0..2 {
			self.fRec80[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec79[l76 as usize] = 0.0;
		}
		self.iTempPerm379 = 0;
		self.iTempPerm380 = 0;
		self.fTempPerm381 = 0.0;
		for l77 in 0..2 {
			self.fRec85[l77 as usize] = 0.0;
		}
		self.fTempPerm382 = 0.0;
		self.fTempPerm383 = 0.0;
		self.fTempPerm384 = 0.0;
		self.iTempPerm385 = 0;
		self.fTempPerm386 = 0.0;
		self.fTempPerm387 = 0.0;
		self.iTempPerm388 = 0;
		self.iTempPerm389 = 0;
		self.fTempPerm390 = 0.0;
		for l78 in 0..2 {
			self.fRec90[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec89[l79 as usize] = 0.0;
		}
		self.fTempPerm391 = 0.0;
		self.fTempPerm392 = 0.0;
		self.iTempPerm393 = 0;
		self.fTempPerm394 = 0.0;
		self.fTempPerm395 = 0.0;
		self.fTempPerm396 = 0.0;
		self.iTempPerm397 = 0;
		self.fTempPerm398 = 0.0;
		self.fTempPerm399 = 0.0;
		for l80 in 0..2 {
			self.fRec91[l80 as usize] = 0.0;
		}
		self.fTempPerm400 = 0.0;
		self.fTempPerm401 = 0.0;
		self.fTempPerm402 = 0.0;
		self.fTempPerm403 = 0.0;
		self.fTempPerm404 = 0.0;
		self.iTempPerm405 = 0;
		self.iTempPerm406 = 0;
		self.iTempPerm407 = 0;
		self.iTempPerm408 = 0;
		self.iTempPerm409 = 0;
		self.fTempPerm410 = 0.0;
		self.iTempPerm411 = 0;
		self.iTempPerm412 = 0;
		self.fTempPerm413 = 0.0;
		self.fTempPerm414 = 0.0;
		self.iTempPerm415 = 0;
		self.iTempPerm416 = 0;
		self.fTempPerm417 = 0.0;
		self.fTempPerm418 = 0.0;
		self.iTempPerm419 = 0;
		self.fTempPerm420 = 0.0;
		for l81 in 0..2 {
			self.fRec96[l81 as usize] = 0.0;
		}
		self.fTempPerm421 = 0.0;
		self.iTempPerm422 = 0;
		self.iTempPerm423 = 0;
		self.fTempPerm424 = 0.0;
		self.fTempPerm425 = 0.0;
		self.fTempPerm426 = 0.0;
		self.fTempPerm427 = 0.0;
		for l82 in 0..2 {
			self.fRec94[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec95[l83 as usize] = 0.0;
		}
		self.fTempPerm428 = 0.0;
		self.fTempPerm429 = 0.0;
		self.iTempPerm430 = 0;
		self.fTempPerm431 = 0.0;
		self.fTempPerm432 = 0.0;
		self.fTempPerm433 = 0.0;
		self.iTempPerm434 = 0;
		self.fTempPerm435 = 0.0;
		self.fTempPerm436 = 0.0;
		for l84 in 0..2 {
			self.fRec97[l84 as usize] = 0.0;
		}
		self.fTempPerm437 = 0.0;
		for l85 in 0..2 {
			self.fVec7[l85 as usize] = 0.0;
		}
		self.fTempPerm438 = 0.0;
		self.iTempPerm439 = 0;
		self.iTempPerm440 = 0;
		self.iTempPerm441 = 0;
		self.fTempPerm442 = 0.0;
		self.fTempPerm443 = 0.0;
		for l86 in 0..2 {
			self.fRec93[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec92[l87 as usize] = 0.0;
		}
		self.iTempPerm444 = 0;
		self.iTempPerm445 = 0;
		self.fTempPerm446 = 0.0;
		for l88 in 0..2 {
			self.fRec98[l88 as usize] = 0.0;
		}
		self.fTempPerm447 = 0.0;
		self.fTempPerm448 = 0.0;
		self.fTempPerm449 = 0.0;
		self.iTempPerm450 = 0;
		self.fTempPerm451 = 0.0;
		self.fTempPerm452 = 0.0;
		self.iTempPerm453 = 0;
		self.iTempPerm454 = 0;
		self.fTempPerm455 = 0.0;
		for l89 in 0..2 {
			self.fRec103[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec102[l90 as usize] = 0.0;
		}
		self.fTempPerm456 = 0.0;
		self.fTempPerm457 = 0.0;
		self.iTempPerm458 = 0;
		self.fTempPerm459 = 0.0;
		self.fTempPerm460 = 0.0;
		self.fTempPerm461 = 0.0;
		self.iTempPerm462 = 0;
		self.fTempPerm463 = 0.0;
		self.fTempPerm464 = 0.0;
		for l91 in 0..2 {
			self.fRec104[l91 as usize] = 0.0;
		}
		self.fTempPerm465 = 0.0;
		self.fTempPerm466 = 0.0;
		self.fTempPerm467 = 0.0;
		self.fTempPerm468 = 0.0;
		self.fTempPerm469 = 0.0;
		self.iTempPerm470 = 0;
		self.iTempPerm471 = 0;
		self.iTempPerm472 = 0;
		self.iTempPerm473 = 0;
		self.iTempPerm474 = 0;
		self.fTempPerm475 = 0.0;
		self.iTempPerm476 = 0;
		self.iTempPerm477 = 0;
		self.fTempPerm478 = 0.0;
		self.fTempPerm479 = 0.0;
		self.iTempPerm480 = 0;
		self.iTempPerm481 = 0;
		self.fTempPerm482 = 0.0;
		self.fTempPerm483 = 0.0;
		self.iTempPerm484 = 0;
		self.fTempPerm485 = 0.0;
		for l92 in 0..2 {
			self.fRec109[l92 as usize] = 0.0;
		}
		self.fTempPerm486 = 0.0;
		self.iTempPerm487 = 0;
		self.iTempPerm488 = 0;
		self.fTempPerm489 = 0.0;
		self.fTempPerm490 = 0.0;
		self.fTempPerm491 = 0.0;
		self.fTempPerm492 = 0.0;
		for l93 in 0..2 {
			self.fRec107[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec108[l94 as usize] = 0.0;
		}
		self.fTempPerm493 = 0.0;
		self.fTempPerm494 = 0.0;
		self.iTempPerm495 = 0;
		self.fTempPerm496 = 0.0;
		self.fTempPerm497 = 0.0;
		self.fTempPerm498 = 0.0;
		self.iTempPerm499 = 0;
		self.fTempPerm500 = 0.0;
		self.fTempPerm501 = 0.0;
		for l95 in 0..2 {
			self.fRec110[l95 as usize] = 0.0;
		}
		self.fTempPerm502 = 0.0;
		for l96 in 0..2 {
			self.fVec8[l96 as usize] = 0.0;
		}
		self.fTempPerm503 = 0.0;
		self.iTempPerm504 = 0;
		self.iTempPerm505 = 0;
		self.iTempPerm506 = 0;
		self.fTempPerm507 = 0.0;
		self.fTempPerm508 = 0.0;
		for l97 in 0..2 {
			self.fRec106[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec105[l98 as usize] = 0.0;
		}
		self.iTempPerm509 = 0;
		self.iTempPerm510 = 0;
		self.fTempPerm511 = 0.0;
		for l99 in 0..2 {
			self.fRec111[l99 as usize] = 0.0;
		}
		self.fTempPerm512 = 0.0;
		self.fTempPerm513 = 0.0;
		self.fTempPerm514 = 0.0;
		self.iTempPerm515 = 0;
		self.fTempPerm516 = 0.0;
		self.fTempPerm517 = 0.0;
		self.iTempPerm518 = 0;
		self.iTempPerm519 = 0;
		self.fTempPerm520 = 0.0;
		for l100 in 0..2 {
			self.fRec116[l100 as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fRec115[l101 as usize] = 0.0;
		}
		self.fTempPerm521 = 0.0;
		self.fTempPerm522 = 0.0;
		self.iTempPerm523 = 0;
		self.fTempPerm524 = 0.0;
		self.fTempPerm525 = 0.0;
		self.fTempPerm526 = 0.0;
		self.iTempPerm527 = 0;
		self.fTempPerm528 = 0.0;
		self.fTempPerm529 = 0.0;
		for l102 in 0..2 {
			self.fRec117[l102 as usize] = 0.0;
		}
		self.fTempPerm530 = 0.0;
		self.fTempPerm531 = 0.0;
		self.fTempPerm532 = 0.0;
		self.fTempPerm533 = 0.0;
		self.fTempPerm534 = 0.0;
		self.iTempPerm535 = 0;
		self.iTempPerm536 = 0;
		self.iTempPerm537 = 0;
		self.iTempPerm538 = 0;
		self.iTempPerm539 = 0;
		self.fTempPerm540 = 0.0;
		self.iTempPerm541 = 0;
		self.iTempPerm542 = 0;
		self.fTempPerm543 = 0.0;
		self.fTempPerm544 = 0.0;
		self.iTempPerm545 = 0;
		self.iTempPerm546 = 0;
		self.fTempPerm547 = 0.0;
		self.fTempPerm548 = 0.0;
		self.iTempPerm549 = 0;
		self.fTempPerm550 = 0.0;
		for l103 in 0..2 {
			self.fRec122[l103 as usize] = 0.0;
		}
		self.fTempPerm551 = 0.0;
		self.iTempPerm552 = 0;
		self.iTempPerm553 = 0;
		self.fTempPerm554 = 0.0;
		self.fTempPerm555 = 0.0;
		self.fTempPerm556 = 0.0;
		self.fTempPerm557 = 0.0;
		for l104 in 0..2 {
			self.fRec120[l104 as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec121[l105 as usize] = 0.0;
		}
		self.fTempPerm558 = 0.0;
		self.fTempPerm559 = 0.0;
		self.iTempPerm560 = 0;
		self.fTempPerm561 = 0.0;
		self.fTempPerm562 = 0.0;
		self.fTempPerm563 = 0.0;
		self.iTempPerm564 = 0;
		self.fTempPerm565 = 0.0;
		self.fTempPerm566 = 0.0;
		for l106 in 0..2 {
			self.fRec123[l106 as usize] = 0.0;
		}
		self.fTempPerm567 = 0.0;
		for l107 in 0..2 {
			self.fVec9[l107 as usize] = 0.0;
		}
		self.fTempPerm568 = 0.0;
		self.iTempPerm569 = 0;
		self.iTempPerm570 = 0;
		self.iTempPerm571 = 0;
		self.fTempPerm572 = 0.0;
		self.fTempPerm573 = 0.0;
		for l108 in 0..2 {
			self.fRec119[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec118[l109 as usize] = 0.0;
		}
		self.iTempPerm574 = 0;
		self.iTempPerm575 = 0;
		self.fTempPerm576 = 0.0;
		for l110 in 0..2 {
			self.fRec124[l110 as usize] = 0.0;
		}
		self.fTempPerm577 = 0.0;
		self.fTempPerm578 = 0.0;
		self.fTempPerm579 = 0.0;
		self.iTempPerm580 = 0;
		self.fTempPerm581 = 0.0;
		self.fTempPerm582 = 0.0;
		self.iTempPerm583 = 0;
		self.iTempPerm584 = 0;
		self.fTempPerm585 = 0.0;
		for l111 in 0..2 {
			self.fRec129[l111 as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec128[l112 as usize] = 0.0;
		}
		self.fTempPerm586 = 0.0;
		self.fTempPerm587 = 0.0;
		self.iTempPerm588 = 0;
		self.fTempPerm589 = 0.0;
		self.fTempPerm590 = 0.0;
		self.fTempPerm591 = 0.0;
		self.iTempPerm592 = 0;
		self.fTempPerm593 = 0.0;
		self.fTempPerm594 = 0.0;
		for l113 in 0..2 {
			self.fRec130[l113 as usize] = 0.0;
		}
		self.fTempPerm595 = 0.0;
		self.fTempPerm596 = 0.0;
		self.fTempPerm597 = 0.0;
		self.fTempPerm598 = 0.0;
		self.fTempPerm599 = 0.0;
		self.iTempPerm600 = 0;
		self.iTempPerm601 = 0;
		self.iTempPerm602 = 0;
		self.iTempPerm603 = 0;
		self.iTempPerm604 = 0;
		self.fTempPerm605 = 0.0;
		self.iTempPerm606 = 0;
		self.iTempPerm607 = 0;
		self.fTempPerm608 = 0.0;
		self.fTempPerm609 = 0.0;
		self.iTempPerm610 = 0;
		self.iTempPerm611 = 0;
		self.fTempPerm612 = 0.0;
		self.fTempPerm613 = 0.0;
		self.iTempPerm614 = 0;
		self.fTempPerm615 = 0.0;
		for l114 in 0..2 {
			self.fRec135[l114 as usize] = 0.0;
		}
		self.fTempPerm616 = 0.0;
		self.iTempPerm617 = 0;
		self.iTempPerm618 = 0;
		self.fTempPerm619 = 0.0;
		self.fTempPerm620 = 0.0;
		self.fTempPerm621 = 0.0;
		self.fTempPerm622 = 0.0;
		for l115 in 0..2 {
			self.fRec133[l115 as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec134[l116 as usize] = 0.0;
		}
		self.fTempPerm623 = 0.0;
		self.fTempPerm624 = 0.0;
		self.iTempPerm625 = 0;
		self.fTempPerm626 = 0.0;
		self.fTempPerm627 = 0.0;
		self.fTempPerm628 = 0.0;
		self.iTempPerm629 = 0;
		self.fTempPerm630 = 0.0;
		self.fTempPerm631 = 0.0;
		for l117 in 0..2 {
			self.fRec136[l117 as usize] = 0.0;
		}
		self.fTempPerm632 = 0.0;
		for l118 in 0..2 {
			self.fVec10[l118 as usize] = 0.0;
		}
		self.fTempPerm633 = 0.0;
		self.iTempPerm634 = 0;
		self.iTempPerm635 = 0;
		self.iTempPerm636 = 0;
		self.fTempPerm637 = 0.0;
		self.fTempPerm638 = 0.0;
		for l119 in 0..2 {
			self.fRec132[l119 as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec131[l120 as usize] = 0.0;
		}
		self.iTempPerm639 = 0;
		self.iTempPerm640 = 0;
		self.fTempPerm641 = 0.0;
		for l121 in 0..2 {
			self.fRec137[l121 as usize] = 0.0;
		}
		self.fTempPerm642 = 0.0;
		self.fTempPerm643 = 0.0;
		self.fTempPerm644 = 0.0;
		self.iTempPerm645 = 0;
		self.fTempPerm646 = 0.0;
		self.fTempPerm647 = 0.0;
		self.iTempPerm648 = 0;
		self.iTempPerm649 = 0;
		self.fTempPerm650 = 0.0;
		for l122 in 0..2 {
			self.fRec142[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec141[l123 as usize] = 0.0;
		}
		self.fTempPerm651 = 0.0;
		self.fTempPerm652 = 0.0;
		self.iTempPerm653 = 0;
		self.fTempPerm654 = 0.0;
		self.fTempPerm655 = 0.0;
		self.fTempPerm656 = 0.0;
		self.iTempPerm657 = 0;
		self.fTempPerm658 = 0.0;
		self.fTempPerm659 = 0.0;
		for l124 in 0..2 {
			self.fRec143[l124 as usize] = 0.0;
		}
		self.fTempPerm660 = 0.0;
		self.fTempPerm661 = 0.0;
		self.fTempPerm662 = 0.0;
		self.fTempPerm663 = 0.0;
		self.fTempPerm664 = 0.0;
		self.iTempPerm665 = 0;
		self.iTempPerm666 = 0;
		self.iTempPerm667 = 0;
		self.iTempPerm668 = 0;
		self.iTempPerm669 = 0;
		self.fTempPerm670 = 0.0;
		self.iTempPerm671 = 0;
		self.iTempPerm672 = 0;
		self.fTempPerm673 = 0.0;
		self.fTempPerm674 = 0.0;
		self.iTempPerm675 = 0;
		self.iTempPerm676 = 0;
		self.fTempPerm677 = 0.0;
		self.fTempPerm678 = 0.0;
		self.iTempPerm679 = 0;
		self.fTempPerm680 = 0.0;
		for l125 in 0..2 {
			self.fRec148[l125 as usize] = 0.0;
		}
		self.fTempPerm681 = 0.0;
		self.iTempPerm682 = 0;
		self.iTempPerm683 = 0;
		self.fTempPerm684 = 0.0;
		self.fTempPerm685 = 0.0;
		self.fTempPerm686 = 0.0;
		self.fTempPerm687 = 0.0;
		for l126 in 0..2 {
			self.fRec146[l126 as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fRec147[l127 as usize] = 0.0;
		}
		self.fTempPerm688 = 0.0;
		self.fTempPerm689 = 0.0;
		self.iTempPerm690 = 0;
		self.fTempPerm691 = 0.0;
		self.fTempPerm692 = 0.0;
		self.fTempPerm693 = 0.0;
		self.iTempPerm694 = 0;
		self.fTempPerm695 = 0.0;
		self.fTempPerm696 = 0.0;
		for l128 in 0..2 {
			self.fRec149[l128 as usize] = 0.0;
		}
		self.fTempPerm697 = 0.0;
		for l129 in 0..2 {
			self.fVec11[l129 as usize] = 0.0;
		}
		self.fTempPerm698 = 0.0;
		self.iTempPerm699 = 0;
		self.iTempPerm700 = 0;
		self.iTempPerm701 = 0;
		self.fTempPerm702 = 0.0;
		self.fTempPerm703 = 0.0;
		for l130 in 0..2 {
			self.fRec145[l130 as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.fRec144[l131 as usize] = 0.0;
		}
		self.iTempPerm704 = 0;
		self.iTempPerm705 = 0;
		self.fTempPerm706 = 0.0;
		for l132 in 0..2 {
			self.fRec150[l132 as usize] = 0.0;
		}
		self.fTempPerm707 = 0.0;
		self.fTempPerm708 = 0.0;
		self.fTempPerm709 = 0.0;
		self.iTempPerm710 = 0;
		self.fTempPerm711 = 0.0;
		self.fTempPerm712 = 0.0;
		self.iTempPerm713 = 0;
		self.iTempPerm714 = 0;
		self.fTempPerm715 = 0.0;
		for l133 in 0..2 {
			self.fRec155[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec154[l134 as usize] = 0.0;
		}
		self.fTempPerm716 = 0.0;
		self.fTempPerm717 = 0.0;
		self.iTempPerm718 = 0;
		self.fTempPerm719 = 0.0;
		self.fTempPerm720 = 0.0;
		self.fTempPerm721 = 0.0;
		self.iTempPerm722 = 0;
		self.fTempPerm723 = 0.0;
		self.fTempPerm724 = 0.0;
		for l135 in 0..2 {
			self.fRec156[l135 as usize] = 0.0;
		}
		self.fTempPerm725 = 0.0;
		self.fTempPerm726 = 0.0;
		self.fTempPerm727 = 0.0;
		self.fTempPerm728 = 0.0;
		self.fTempPerm729 = 0.0;
		self.iTempPerm730 = 0;
		self.iTempPerm731 = 0;
		self.iTempPerm732 = 0;
		self.iTempPerm733 = 0;
		self.iTempPerm734 = 0;
		self.fTempPerm735 = 0.0;
		self.iTempPerm736 = 0;
		self.fTempPerm737 = 0.0;
		self.fTempPerm738 = 0.0;
		self.iTempPerm739 = 0;
		self.iTempPerm740 = 0;
		self.fTempPerm741 = 0.0;
		self.fTempPerm742 = 0.0;
		self.iTempPerm743 = 0;
		self.fTempPerm744 = 0.0;
		for l136 in 0..2 {
			self.fRec161[l136 as usize] = 0.0;
		}
		self.fTempPerm745 = 0.0;
		self.iTempPerm746 = 0;
		self.iTempPerm747 = 0;
		self.fTempPerm748 = 0.0;
		self.fTempPerm749 = 0.0;
		self.fTempPerm750 = 0.0;
		self.fTempPerm751 = 0.0;
		for l137 in 0..2 {
			self.fRec159[l137 as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec160[l138 as usize] = 0.0;
		}
		self.fTempPerm752 = 0.0;
		self.fTempPerm753 = 0.0;
		self.iTempPerm754 = 0;
		self.fTempPerm755 = 0.0;
		self.fTempPerm756 = 0.0;
		self.fTempPerm757 = 0.0;
		self.iTempPerm758 = 0;
		self.fTempPerm759 = 0.0;
		self.fTempPerm760 = 0.0;
		for l139 in 0..2 {
			self.fRec162[l139 as usize] = 0.0;
		}
		self.fTempPerm761 = 0.0;
		for l140 in 0..2 {
			self.fVec12[l140 as usize] = 0.0;
		}
		self.fTempPerm762 = 0.0;
		self.iTempPerm763 = 0;
		self.iTempPerm764 = 0;
		self.iTempPerm765 = 0;
		self.fTempPerm766 = 0.0;
		self.fTempPerm767 = 0.0;
		for l141 in 0..2 {
			self.fRec158[l141 as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec157[l142 as usize] = 0.0;
		}
		self.iTempPerm768 = 0;
		self.iTempPerm769 = 0;
		self.fTempPerm770 = 0.0;
		for l143 in 0..2 {
			self.fRec163[l143 as usize] = 0.0;
		}
		self.fTempPerm771 = 0.0;
		self.fTempPerm772 = 0.0;
		self.fTempPerm773 = 0.0;
		self.iTempPerm774 = 0;
		self.fTempPerm775 = 0.0;
		self.fTempPerm776 = 0.0;
		self.iTempPerm777 = 0;
		self.iTempPerm778 = 0;
		self.fTempPerm779 = 0.0;
		for l144 in 0..2 {
			self.fRec168[l144 as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec167[l145 as usize] = 0.0;
		}
		self.fTempPerm780 = 0.0;
		self.fTempPerm781 = 0.0;
		self.iTempPerm782 = 0;
		self.fTempPerm783 = 0.0;
		self.fTempPerm784 = 0.0;
		self.fTempPerm785 = 0.0;
		self.iTempPerm786 = 0;
		self.fTempPerm787 = 0.0;
		self.fTempPerm788 = 0.0;
		for l146 in 0..2 {
			self.fRec169[l146 as usize] = 0.0;
		}
		self.fTempPerm789 = 0.0;
		self.fTempPerm790 = 0.0;
		self.fTempPerm791 = 0.0;
		self.fTempPerm792 = 0.0;
		self.fTempPerm793 = 0.0;
		self.fTempPerm794 = 0.0;
		self.fTempPerm795 = 0.0;
		for l147 in 0..2 {
			self.fRec164[l147 as usize] = 0.0;
		}
		self.fTempPerm796 = 0.0;
		for l148 in 0..2 {
			self.fRec165[l148 as usize] = 0.0;
		}
		self.fTempPerm797 = 0.0;
		self.fTempPerm798 = 0.0;
		self.fTempPerm799 = 0.0;
		self.fTempPerm800 = 0.0;
		for l149 in 0..2 {
			self.fRec151[l149 as usize] = 0.0;
		}
		self.fTempPerm801 = 0.0;
		for l150 in 0..2 {
			self.fRec152[l150 as usize] = 0.0;
		}
		self.fTempPerm802 = 0.0;
		self.fTempPerm803 = 0.0;
		self.fTempPerm804 = 0.0;
		self.fTempPerm805 = 0.0;
		for l151 in 0..2 {
			self.fRec138[l151 as usize] = 0.0;
		}
		self.fTempPerm806 = 0.0;
		for l152 in 0..2 {
			self.fRec139[l152 as usize] = 0.0;
		}
		self.fTempPerm807 = 0.0;
		self.fTempPerm808 = 0.0;
		self.fTempPerm809 = 0.0;
		self.fTempPerm810 = 0.0;
		for l153 in 0..2 {
			self.fRec125[l153 as usize] = 0.0;
		}
		self.fTempPerm811 = 0.0;
		for l154 in 0..2 {
			self.fRec126[l154 as usize] = 0.0;
		}
		self.fTempPerm812 = 0.0;
		self.fTempPerm813 = 0.0;
		self.fTempPerm814 = 0.0;
		self.fTempPerm815 = 0.0;
		for l155 in 0..2 {
			self.fRec112[l155 as usize] = 0.0;
		}
		self.fTempPerm816 = 0.0;
		for l156 in 0..2 {
			self.fRec113[l156 as usize] = 0.0;
		}
		self.fTempPerm817 = 0.0;
		self.fTempPerm818 = 0.0;
		self.fTempPerm819 = 0.0;
		self.fTempPerm820 = 0.0;
		for l157 in 0..2 {
			self.fRec99[l157 as usize] = 0.0;
		}
		self.fTempPerm821 = 0.0;
		for l158 in 0..2 {
			self.fRec100[l158 as usize] = 0.0;
		}
		self.fTempPerm822 = 0.0;
		self.fTempPerm823 = 0.0;
		self.fTempPerm824 = 0.0;
		self.fTempPerm825 = 0.0;
		for l159 in 0..2 {
			self.fRec86[l159 as usize] = 0.0;
		}
		self.fTempPerm826 = 0.0;
		for l160 in 0..2 {
			self.fRec87[l160 as usize] = 0.0;
		}
		self.fTempPerm827 = 0.0;
		self.fTempPerm828 = 0.0;
		self.fTempPerm829 = 0.0;
		self.fTempPerm830 = 0.0;
		for l161 in 0..2 {
			self.fRec73[l161 as usize] = 0.0;
		}
		self.fTempPerm831 = 0.0;
		for l162 in 0..2 {
			self.fRec74[l162 as usize] = 0.0;
		}
		self.fTempPerm832 = 0.0;
		self.fTempPerm833 = 0.0;
		self.fTempPerm834 = 0.0;
		self.fTempPerm835 = 0.0;
		for l163 in 0..2 {
			self.fRec60[l163 as usize] = 0.0;
		}
		self.fTempPerm836 = 0.0;
		for l164 in 0..2 {
			self.fRec61[l164 as usize] = 0.0;
		}
		self.fTempPerm837 = 0.0;
		self.fTempPerm838 = 0.0;
		self.fTempPerm839 = 0.0;
		self.fTempPerm840 = 0.0;
		for l165 in 0..2 {
			self.fRec47[l165 as usize] = 0.0;
		}
		self.fTempPerm841 = 0.0;
		for l166 in 0..2 {
			self.fRec48[l166 as usize] = 0.0;
		}
		self.fTempPerm842 = 0.0;
		self.fTempPerm843 = 0.0;
		self.fTempPerm844 = 0.0;
		self.fTempPerm845 = 0.0;
		for l167 in 0..2 {
			self.fRec34[l167 as usize] = 0.0;
		}
		self.fTempPerm846 = 0.0;
		for l168 in 0..2 {
			self.fRec35[l168 as usize] = 0.0;
		}
		self.fTempPerm847 = 0.0;
		self.fTempPerm848 = 0.0;
		self.fTempPerm849 = 0.0;
		self.fTempPerm850 = 0.0;
		for l169 in 0..2 {
			self.fRec20[l169 as usize] = 0.0;
		}
		self.fTempPerm851 = 0.0;
		for l170 in 0..2 {
			self.fRec21[l170 as usize] = 0.0;
		}
		self.fTempPerm852 = 0.0;
		self.fTempPerm853 = 0.0;
		for l171 in 0..2 {
			self.fRec170[l171 as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fRec171[l172 as usize] = 0.0;
		}
		for l173 in 0..2 {
			self.fRec172[l173 as usize] = 0.0;
		}
		self.fTempPerm854 = 0.0;
		self.fTempPerm855 = 0.0;
		for l174 in 0..2 {
			self.fRec206[l174 as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec207[l175 as usize] = 0.0;
		}
		self.fTempPerm856 = 0.0;
		self.fTempPerm857 = 0.0;
		self.fTempPerm858 = 0.0;
		for l176 in 0..2 {
			self.fRec203[l176 as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec204[l177 as usize] = 0.0;
		}
		self.fTempPerm859 = 0.0;
		self.fTempPerm860 = 0.0;
		self.fTempPerm861 = 0.0;
		for l178 in 0..2 {
			self.fRec200[l178 as usize] = 0.0;
		}
		for l179 in 0..2 {
			self.fRec201[l179 as usize] = 0.0;
		}
		self.fTempPerm862 = 0.0;
		self.fTempPerm863 = 0.0;
		self.fTempPerm864 = 0.0;
		for l180 in 0..2 {
			self.fRec197[l180 as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fRec198[l181 as usize] = 0.0;
		}
		self.fTempPerm865 = 0.0;
		self.fTempPerm866 = 0.0;
		self.fTempPerm867 = 0.0;
		for l182 in 0..2 {
			self.fRec194[l182 as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec195[l183 as usize] = 0.0;
		}
		self.fTempPerm868 = 0.0;
		self.fTempPerm869 = 0.0;
		self.fTempPerm870 = 0.0;
		for l184 in 0..2 {
			self.fRec191[l184 as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec192[l185 as usize] = 0.0;
		}
		self.fTempPerm871 = 0.0;
		self.fTempPerm872 = 0.0;
		self.fTempPerm873 = 0.0;
		for l186 in 0..2 {
			self.fRec188[l186 as usize] = 0.0;
		}
		for l187 in 0..2 {
			self.fRec189[l187 as usize] = 0.0;
		}
		self.fTempPerm874 = 0.0;
		self.fTempPerm875 = 0.0;
		self.fTempPerm876 = 0.0;
		for l188 in 0..2 {
			self.fRec185[l188 as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec186[l189 as usize] = 0.0;
		}
		self.fTempPerm877 = 0.0;
		self.fTempPerm878 = 0.0;
		self.fTempPerm879 = 0.0;
		for l190 in 0..2 {
			self.fRec182[l190 as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec183[l191 as usize] = 0.0;
		}
		self.fTempPerm880 = 0.0;
		self.fTempPerm881 = 0.0;
		self.fTempPerm882 = 0.0;
		for l192 in 0..2 {
			self.fRec179[l192 as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec180[l193 as usize] = 0.0;
		}
		self.fTempPerm883 = 0.0;
		self.fTempPerm884 = 0.0;
		self.fTempPerm885 = 0.0;
		for l194 in 0..2 {
			self.fRec176[l194 as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec177[l195 as usize] = 0.0;
		}
		self.fTempPerm886 = 0.0;
		self.fTempPerm887 = 0.0;
		self.fTempPerm888 = 0.0;
		for l196 in 0..2 {
			self.fRec173[l196 as usize] = 0.0;
		}
		for l197 in 0..2 {
			self.fRec174[l197 as usize] = 0.0;
		}
		self.fTempPerm889 = 0.0;
		for l198 in 0..2 {
			self.fRec209[l198 as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fRec210[l199 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::exp(-(0.6666667 / fConst0));
		self.fConst2 = 0.33333334 / fConst0;
		self.fConst3 = 1.0 - self.fConst1;
		self.fConst4 = F32::exp(-(83.333336 / fConst0));
		self.fConst5 = 0.45 * fConst0;
		self.fConst6 = 1.0 / fConst0;
		self.fConst7 = 1.0 - self.fConst4;
		let mut fConst8: F32 = AlternativesDsp_faustpower2_f(fConst0);
		self.fConst9 = 97.40909 / fConst8;
		self.fConst10 = 1.0 / fConst8;
		self.fConst11 = 98.696045 / fConst8;
		self.fConst12 = 3.1415927 / fConst0;
		self.fConst13 = 9.869604 / fConst0;
		self.fConst14 = F32::exp(-self.fConst2);
		self.fConst15 = 1.0 - self.fConst14;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		AlternativesDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin");
		ui_interface.add_horizontal_slider("count", ParamIndex(0), 1.0, 1.0, 8.0, 1.0);
		ui_interface.add_horizontal_slider("cut", ParamIndex(1), 3.33, 1.0, 6.0, 0.01);
		ui_interface.add_horizontal_slider("density", ParamIndex(2), 8e+01, 0.0, 1e+02, 0.1);
		ui_interface.add_horizontal_slider("depth", ParamIndex(3), 6.0, 0.0, 18.0, 0.01);
		ui_interface.add_horizontal_slider("drift", ParamIndex(4), 0.275, 0.0, 0.5, 0.001);
		ui_interface.add_check_button("hold", ParamIndex(5));
		ui_interface.add_horizontal_slider("life", ParamIndex(6), 2.0, 0.25, 4.0, 0.01);
		ui_interface.add_horizontal_slider("mix", ParamIndex(7), 1.0, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("note0", ParamIndex(8), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note1", ParamIndex(9), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note2", ParamIndex(10), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note3", ParamIndex(11), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note4", ParamIndex(12), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note5", ParamIndex(13), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note6", ParamIndex(14), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note7", ParamIndex(15), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("output", ParamIndex(16), 0.0, -24.0, 12.0, 0.01);
		ui_interface.add_horizontal_slider("reach", ParamIndex(17), 64.0, 1.0, 128.0, 1.0);
		ui_interface.add_horizontal_slider("seed", ParamIndex(18), 1e+03, 0.0, 1e+06, 1.0);
		ui_interface.add_horizontal_slider("sharp", ParamIndex(19), 1.0, 0.5, 4.0, 0.01);
		ui_interface.add_horizontal_slider("slots", ParamIndex(20), 6.0, 1.0, 12.0, 1.0);
		ui_interface.add_horizontal_slider("walk", ParamIndex(21), 0.6, 0.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			5 => Some(self.fCheckbox0),
			16 => Some(self.fHslider0),
			20 => Some(self.fHslider1),
			11 => Some(self.fHslider10),
			12 => Some(self.fHslider11),
			13 => Some(self.fHslider12),
			14 => Some(self.fHslider13),
			15 => Some(self.fHslider14),
			6 => Some(self.fHslider15),
			2 => Some(self.fHslider16),
			3 => Some(self.fHslider17),
			1 => Some(self.fHslider18),
			19 => Some(self.fHslider19),
			21 => Some(self.fHslider2),
			7 => Some(self.fHslider20),
			18 => Some(self.fHslider3),
			4 => Some(self.fHslider4),
			17 => Some(self.fHslider5),
			0 => Some(self.fHslider6),
			8 => Some(self.fHslider7),
			9 => Some(self.fHslider8),
			10 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			5 => { self.fCheckbox0 = value }
			16 => { self.fHslider0 = value }
			20 => { self.fHslider1 = value }
			11 => { self.fHslider10 = value }
			12 => { self.fHslider11 = value }
			13 => { self.fHslider12 = value }
			14 => { self.fHslider13 = value }
			15 => { self.fHslider14 = value }
			6 => { self.fHslider15 = value }
			2 => { self.fHslider16 = value }
			3 => { self.fHslider17 = value }
			1 => { self.fHslider18 = value }
			19 => { self.fHslider19 = value }
			21 => { self.fHslider2 = value }
			7 => { self.fHslider20 = value }
			18 => { self.fHslider3 = value }
			4 => { self.fHslider4 = value }
			17 => { self.fHslider5 = value }
			0 => { self.fHslider6 = value }
			8 => { self.fHslider7 = value }
			9 => { self.fHslider8 = value }
			10 => { self.fHslider9 = value }
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
		let mut fSlow0: F32 = F32::powf(1e+01, 0.05 * self.fHslider0);
		let mut iSlow1: i32 = (self.fCheckbox0) as i32;
		let mut fSlow2: F32 = self.fHslider1;
		let mut iSlow3: i32 = (11.0 < fSlow2) as i32;
		let mut fSlow4: F32 = (iSlow3) as F32;
		let mut iSlow5: i32 = (fSlow4 != 0.0) as i32;
		let mut fSlow6: F32 = self.fHslider2;
		let mut iSlow7: i32 = (1e+01 < fSlow2) as i32;
		let mut fSlow8: F32 = (iSlow7) as F32;
		let mut iSlow9: i32 = (fSlow8 != 0.0) as i32;
		let mut iSlow10: i32 = (0.0 < fSlow2) as i32;
		let mut fSlow11: F32 = (iSlow10) as F32;
		let mut iSlow12: i32 = (fSlow11 != 0.0) as i32;
		let mut iSlow13: i32 = (2.0 < fSlow2) as i32;
		let mut fSlow14: F32 = (iSlow13) as F32;
		let mut iSlow15: i32 = (fSlow14 != 0.0) as i32;
		let mut iSlow16: i32 = (1.0 < fSlow2) as i32;
		let mut fSlow17: F32 = (iSlow16) as F32;
		let mut iSlow18: i32 = (fSlow17 != 0.0) as i32;
		let mut iSlow19: i32 = (3.0 < fSlow2) as i32;
		let mut fSlow20: F32 = (iSlow19) as F32;
		let mut iSlow21: i32 = (fSlow20 != 0.0) as i32;
		let mut iSlow22: i32 = (9.0 < fSlow2) as i32;
		let mut fSlow23: F32 = (iSlow22) as F32;
		let mut iSlow24: i32 = (fSlow23 != 0.0) as i32;
		let mut iSlow25: i32 = (8.0 < fSlow2) as i32;
		let mut fSlow26: F32 = (iSlow25) as F32;
		let mut iSlow27: i32 = (fSlow26 != 0.0) as i32;
		let mut iSlow28: i32 = (7.0 < fSlow2) as i32;
		let mut fSlow29: F32 = (iSlow28) as F32;
		let mut iSlow30: i32 = (fSlow29 != 0.0) as i32;
		let mut iSlow31: i32 = (6.0 < fSlow2) as i32;
		let mut fSlow32: F32 = (iSlow31) as F32;
		let mut iSlow33: i32 = (fSlow32 != 0.0) as i32;
		let mut iSlow34: i32 = (5.0 < fSlow2) as i32;
		let mut fSlow35: F32 = (iSlow34) as F32;
		let mut iSlow36: i32 = (fSlow35 != 0.0) as i32;
		let mut iSlow37: i32 = (4.0 < fSlow2) as i32;
		let mut fSlow38: F32 = (iSlow37) as F32;
		let mut iSlow39: i32 = (fSlow38 != 0.0) as i32;
		let mut iTemp4: i32 = self.iTempPerm0;
		let mut iTemp5: i32 = self.iTempPerm1;
		let mut fSlow40: F32 = self.fHslider3;
		let mut iSlow41: i32 = (fSlow40 + 7.0) as i32;
		let mut iTemp6: i32 = self.iTempPerm2;
		let mut fSlow42: F32 = self.fHslider4;
		let mut fTemp7: F32 = self.fTempPerm3;
		let mut iTemp8: i32 = self.iTempPerm4;
		let mut iTemp9: i32 = self.iTempPerm5;
		let mut fSlow43: F32 = self.fHslider5;
		let mut fTemp10: F32 = self.fTempPerm6;
		let mut fSlow44: F32 = self.fHslider6;
		let mut fSlow45: F32 = fSlow44 + -1.0;
		let mut iSlow46: i32 = (fSlow40) as i32;
		let mut iTemp11: i32 = self.iTempPerm7;
		let mut iTemp12: i32 = self.iTempPerm8;
		let mut iTemp13: i32 = self.iTempPerm9;
		let mut fSlow47: F32 = self.fHslider7;
		let mut fSlow48: F32 = self.fHslider8;
		let mut fSlow49: F32 = self.fHslider9;
		let mut fSlow50: F32 = self.fHslider10;
		let mut fSlow51: F32 = self.fHslider11;
		let mut fSlow52: F32 = self.fHslider12;
		let mut fSlow53: F32 = self.fHslider13;
		let mut fSlow54: F32 = self.fHslider14;
		let mut fTemp14: F32 = self.fTempPerm10;
		let mut iTemp15: i32 = self.iTempPerm11;
		let mut iTemp16: i32 = self.iTempPerm12;
		let mut fTemp17: F32 = self.fTempPerm13;
		let mut fTemp18: F32 = self.fTempPerm14;
		let mut iTemp19: i32 = self.iTempPerm15;
		let mut iTemp20: i32 = self.iTempPerm16;
		let mut fTemp21: F32 = self.fTempPerm17;
		let mut fTemp22: F32 = self.fTempPerm18;
		let mut iTemp23: i32 = self.iTempPerm19;
		let mut fTemp24: F32 = self.fTempPerm20;
		let mut fSlow55: F32 = self.fHslider15;
		let mut fTemp25: F32 = self.fTempPerm21;
		let mut fSlow56: F32 = self.fHslider16;
		let mut fTemp26: F32 = self.fTempPerm22;
		let mut fTemp27: F32 = self.fTempPerm23;
		let mut fTemp28: F32 = self.fTempPerm24;
		let mut iTemp29: i32 = self.iTempPerm25;
		let mut iTemp30: i32 = self.iTempPerm26;
		let mut fTemp31: F32 = self.fTempPerm27;
		let mut fTemp32: F32 = self.fTempPerm28;
		let mut fTemp33: F32 = self.fTempPerm29;
		let mut fTemp34: F32 = self.fTempPerm30;
		let mut fTemp35: F32 = self.fTempPerm31;
		let mut fTemp36: F32 = self.fTempPerm32;
		let mut iTemp37: i32 = self.iTempPerm33;
		let mut fTemp38: F32 = self.fTempPerm34;
		let mut fTemp39: F32 = self.fTempPerm35;
		let mut fTemp40: F32 = self.fTempPerm36;
		let mut iTemp41: i32 = self.iTempPerm37;
		let mut fTemp42: F32 = self.fTempPerm38;
		let mut fTemp43: F32 = self.fTempPerm39;
		let mut fTemp44: F32 = self.fTempPerm40;
		let mut fTemp45: F32 = self.fTempPerm41;
		let mut iTemp46: i32 = self.iTempPerm42;
		let mut iTemp47: i32 = self.iTempPerm43;
		let mut iTemp48: i32 = self.iTempPerm44;
		let mut fTemp49: F32 = self.fTempPerm45;
		let mut fTemp50: F32 = self.fTempPerm46;
		let mut iTemp51: i32 = self.iTempPerm47;
		let mut iTemp52: i32 = self.iTempPerm48;
		let mut fTemp53: F32 = self.fTempPerm49;
		let mut fTemp54: F32 = self.fTempPerm50;
		let mut fSlow57: F32 = self.fHslider17;
		let mut fTemp55: F32 = self.fTempPerm51;
		let mut fTemp56: F32 = self.fTempPerm52;
		let mut fTemp57: F32 = self.fTempPerm53;
		let mut fSlow58: F32 = self.fHslider18;
		let mut fTemp58: F32 = self.fTempPerm54;
		let mut fTemp59: F32 = self.fTempPerm55;
		let mut fTemp60: F32 = self.fTempPerm56;
		let mut iTemp61: i32 = self.iTempPerm57;
		let mut fTemp62: F32 = self.fTempPerm58;
		let mut fTemp63: F32 = self.fTempPerm59;
		let mut fSlow59: F32 = self.fHslider19;
		let mut fTemp64: F32 = self.fTempPerm60;
		let mut fTemp65: F32 = self.fTempPerm61;
		let mut iTemp66: i32 = self.iTempPerm62;
		let mut iTemp67: i32 = self.iTempPerm63;
		let mut fTemp68: F32 = self.fTempPerm64;
		let mut fTemp69: F32 = self.fTempPerm65;
		let mut fTemp70: F32 = self.fTempPerm66;
		let mut iTemp71: i32 = self.iTempPerm67;
		let mut fTemp72: F32 = self.fTempPerm68;
		let mut fTemp73: F32 = self.fTempPerm69;
		let mut fTemp74: F32 = self.fTempPerm70;
		let mut iTemp75: i32 = self.iTempPerm71;
		let mut fTemp76: F32 = self.fTempPerm72;
		let mut fTemp77: F32 = self.fTempPerm73;
		let mut fTemp78: F32 = self.fTempPerm74;
		let mut fTemp79: F32 = self.fTempPerm75;
		let mut fTemp80: F32 = self.fTempPerm76;
		let mut fTemp81: F32 = self.fTempPerm77;
		let mut fTemp82: F32 = self.fTempPerm78;
		let mut fTemp83: F32 = self.fTempPerm79;
		let mut iTemp84: i32 = self.iTempPerm80;
		let mut iTemp85: i32 = self.iTempPerm81;
		let mut iTemp86: i32 = self.iTempPerm82;
		let mut iTemp87: i32 = self.iTempPerm83;
		let mut iTemp88: i32 = self.iTempPerm84;
		let mut fTemp89: F32 = self.fTempPerm85;
		let mut iTemp90: i32 = self.iTempPerm86;
		let mut iTemp91: i32 = self.iTempPerm87;
		let mut fTemp92: F32 = self.fTempPerm88;
		let mut fTemp93: F32 = self.fTempPerm89;
		let mut iTemp94: i32 = self.iTempPerm90;
		let mut iTemp95: i32 = self.iTempPerm91;
		let mut fTemp96: F32 = self.fTempPerm92;
		let mut fTemp97: F32 = self.fTempPerm93;
		let mut iTemp98: i32 = self.iTempPerm94;
		let mut fTemp99: F32 = self.fTempPerm95;
		let mut fTemp100: F32 = self.fTempPerm96;
		let mut iTemp101: i32 = self.iTempPerm97;
		let mut iTemp102: i32 = self.iTempPerm98;
		let mut fTemp103: F32 = self.fTempPerm99;
		let mut fTemp104: F32 = self.fTempPerm100;
		let mut fTemp105: F32 = self.fTempPerm101;
		let mut fTemp106: F32 = self.fTempPerm102;
		let mut fTemp107: F32 = self.fTempPerm103;
		let mut fTemp108: F32 = self.fTempPerm104;
		let mut iTemp109: i32 = self.iTempPerm105;
		let mut fTemp110: F32 = self.fTempPerm106;
		let mut fTemp111: F32 = self.fTempPerm107;
		let mut fTemp112: F32 = self.fTempPerm108;
		let mut iTemp113: i32 = self.iTempPerm109;
		let mut fTemp114: F32 = self.fTempPerm110;
		let mut fTemp115: F32 = self.fTempPerm111;
		let mut fTemp116: F32 = self.fTempPerm112;
		let mut fTemp117: F32 = self.fTempPerm113;
		let mut iTemp118: i32 = self.iTempPerm114;
		let mut iTemp119: i32 = self.iTempPerm115;
		let mut iTemp120: i32 = self.iTempPerm116;
		let mut fTemp121: F32 = self.fTempPerm117;
		let mut fTemp122: F32 = self.fTempPerm118;
		let mut iTemp123: i32 = self.iTempPerm119;
		let mut iTemp124: i32 = self.iTempPerm120;
		let mut fTemp125: F32 = self.fTempPerm121;
		let mut fTemp126: F32 = self.fTempPerm122;
		let mut fTemp127: F32 = self.fTempPerm123;
		let mut fTemp128: F32 = self.fTempPerm124;
		let mut iTemp129: i32 = self.iTempPerm125;
		let mut fTemp130: F32 = self.fTempPerm126;
		let mut fTemp131: F32 = self.fTempPerm127;
		let mut iTemp132: i32 = self.iTempPerm128;
		let mut iTemp133: i32 = self.iTempPerm129;
		let mut fTemp134: F32 = self.fTempPerm130;
		let mut fTemp135: F32 = self.fTempPerm131;
		let mut fTemp136: F32 = self.fTempPerm132;
		let mut iTemp137: i32 = self.iTempPerm133;
		let mut fTemp138: F32 = self.fTempPerm134;
		let mut fTemp139: F32 = self.fTempPerm135;
		let mut fTemp140: F32 = self.fTempPerm136;
		let mut iTemp141: i32 = self.iTempPerm137;
		let mut fTemp142: F32 = self.fTempPerm138;
		let mut fTemp143: F32 = self.fTempPerm139;
		let mut fTemp144: F32 = self.fTempPerm140;
		let mut fTemp145: F32 = self.fTempPerm141;
		let mut fTemp146: F32 = self.fTempPerm142;
		let mut fTemp147: F32 = self.fTempPerm143;
		let mut fTemp148: F32 = self.fTempPerm144;
		let mut iTemp149: i32 = self.iTempPerm145;
		let mut iTemp150: i32 = self.iTempPerm146;
		let mut iTemp151: i32 = self.iTempPerm147;
		let mut iTemp152: i32 = self.iTempPerm148;
		let mut iTemp153: i32 = self.iTempPerm149;
		let mut fTemp154: F32 = self.fTempPerm150;
		let mut iTemp155: i32 = self.iTempPerm151;
		let mut iTemp156: i32 = self.iTempPerm152;
		let mut fTemp157: F32 = self.fTempPerm153;
		let mut fTemp158: F32 = self.fTempPerm154;
		let mut iTemp159: i32 = self.iTempPerm155;
		let mut iTemp160: i32 = self.iTempPerm156;
		let mut fTemp161: F32 = self.fTempPerm157;
		let mut fTemp162: F32 = self.fTempPerm158;
		let mut iTemp163: i32 = self.iTempPerm159;
		let mut fTemp164: F32 = self.fTempPerm160;
		let mut fTemp165: F32 = self.fTempPerm161;
		let mut iTemp166: i32 = self.iTempPerm162;
		let mut iTemp167: i32 = self.iTempPerm163;
		let mut fTemp168: F32 = self.fTempPerm164;
		let mut fTemp169: F32 = self.fTempPerm165;
		let mut fTemp170: F32 = self.fTempPerm166;
		let mut fTemp171: F32 = self.fTempPerm167;
		let mut fTemp172: F32 = self.fTempPerm168;
		let mut fTemp173: F32 = self.fTempPerm169;
		let mut iTemp174: i32 = self.iTempPerm170;
		let mut fTemp175: F32 = self.fTempPerm171;
		let mut fTemp176: F32 = self.fTempPerm172;
		let mut fTemp177: F32 = self.fTempPerm173;
		let mut iTemp178: i32 = self.iTempPerm174;
		let mut fTemp179: F32 = self.fTempPerm175;
		let mut fTemp180: F32 = self.fTempPerm176;
		let mut fTemp181: F32 = self.fTempPerm177;
		let mut fTemp182: F32 = self.fTempPerm178;
		let mut iTemp183: i32 = self.iTempPerm179;
		let mut iTemp184: i32 = self.iTempPerm180;
		let mut iTemp185: i32 = self.iTempPerm181;
		let mut fTemp186: F32 = self.fTempPerm182;
		let mut fTemp187: F32 = self.fTempPerm183;
		let mut iTemp188: i32 = self.iTempPerm184;
		let mut iTemp189: i32 = self.iTempPerm185;
		let mut fTemp190: F32 = self.fTempPerm186;
		let mut fTemp191: F32 = self.fTempPerm187;
		let mut fTemp192: F32 = self.fTempPerm188;
		let mut fTemp193: F32 = self.fTempPerm189;
		let mut iTemp194: i32 = self.iTempPerm190;
		let mut fTemp195: F32 = self.fTempPerm191;
		let mut fTemp196: F32 = self.fTempPerm192;
		let mut iTemp197: i32 = self.iTempPerm193;
		let mut iTemp198: i32 = self.iTempPerm194;
		let mut fTemp199: F32 = self.fTempPerm195;
		let mut fTemp200: F32 = self.fTempPerm196;
		let mut fTemp201: F32 = self.fTempPerm197;
		let mut iTemp202: i32 = self.iTempPerm198;
		let mut fTemp203: F32 = self.fTempPerm199;
		let mut fTemp204: F32 = self.fTempPerm200;
		let mut fTemp205: F32 = self.fTempPerm201;
		let mut iTemp206: i32 = self.iTempPerm202;
		let mut fTemp207: F32 = self.fTempPerm203;
		let mut fTemp208: F32 = self.fTempPerm204;
		let mut fTemp209: F32 = self.fTempPerm205;
		let mut fTemp210: F32 = self.fTempPerm206;
		let mut fTemp211: F32 = self.fTempPerm207;
		let mut fTemp212: F32 = self.fTempPerm208;
		let mut fTemp213: F32 = self.fTempPerm209;
		let mut iTemp214: i32 = self.iTempPerm210;
		let mut iTemp215: i32 = self.iTempPerm211;
		let mut iTemp216: i32 = self.iTempPerm212;
		let mut iTemp217: i32 = self.iTempPerm213;
		let mut iTemp218: i32 = self.iTempPerm214;
		let mut fTemp219: F32 = self.fTempPerm215;
		let mut iTemp220: i32 = self.iTempPerm216;
		let mut iTemp221: i32 = self.iTempPerm217;
		let mut fTemp222: F32 = self.fTempPerm218;
		let mut fTemp223: F32 = self.fTempPerm219;
		let mut iTemp224: i32 = self.iTempPerm220;
		let mut iTemp225: i32 = self.iTempPerm221;
		let mut fTemp226: F32 = self.fTempPerm222;
		let mut fTemp227: F32 = self.fTempPerm223;
		let mut iTemp228: i32 = self.iTempPerm224;
		let mut fTemp229: F32 = self.fTempPerm225;
		let mut fTemp230: F32 = self.fTempPerm226;
		let mut iTemp231: i32 = self.iTempPerm227;
		let mut iTemp232: i32 = self.iTempPerm228;
		let mut fTemp233: F32 = self.fTempPerm229;
		let mut fTemp234: F32 = self.fTempPerm230;
		let mut fTemp235: F32 = self.fTempPerm231;
		let mut fTemp236: F32 = self.fTempPerm232;
		let mut fTemp237: F32 = self.fTempPerm233;
		let mut fTemp238: F32 = self.fTempPerm234;
		let mut iTemp239: i32 = self.iTempPerm235;
		let mut fTemp240: F32 = self.fTempPerm236;
		let mut fTemp241: F32 = self.fTempPerm237;
		let mut fTemp242: F32 = self.fTempPerm238;
		let mut iTemp243: i32 = self.iTempPerm239;
		let mut fTemp244: F32 = self.fTempPerm240;
		let mut fTemp245: F32 = self.fTempPerm241;
		let mut fTemp246: F32 = self.fTempPerm242;
		let mut fTemp247: F32 = self.fTempPerm243;
		let mut iTemp248: i32 = self.iTempPerm244;
		let mut iTemp249: i32 = self.iTempPerm245;
		let mut iTemp250: i32 = self.iTempPerm246;
		let mut fTemp251: F32 = self.fTempPerm247;
		let mut fTemp252: F32 = self.fTempPerm248;
		let mut iTemp253: i32 = self.iTempPerm249;
		let mut iTemp254: i32 = self.iTempPerm250;
		let mut fTemp255: F32 = self.fTempPerm251;
		let mut fTemp256: F32 = self.fTempPerm252;
		let mut fTemp257: F32 = self.fTempPerm253;
		let mut fTemp258: F32 = self.fTempPerm254;
		let mut iTemp259: i32 = self.iTempPerm255;
		let mut fTemp260: F32 = self.fTempPerm256;
		let mut fTemp261: F32 = self.fTempPerm257;
		let mut iTemp262: i32 = self.iTempPerm258;
		let mut iTemp263: i32 = self.iTempPerm259;
		let mut fTemp264: F32 = self.fTempPerm260;
		let mut fTemp265: F32 = self.fTempPerm261;
		let mut fTemp266: F32 = self.fTempPerm262;
		let mut iTemp267: i32 = self.iTempPerm263;
		let mut fTemp268: F32 = self.fTempPerm264;
		let mut fTemp269: F32 = self.fTempPerm265;
		let mut fTemp270: F32 = self.fTempPerm266;
		let mut iTemp271: i32 = self.iTempPerm267;
		let mut fTemp272: F32 = self.fTempPerm268;
		let mut fTemp273: F32 = self.fTempPerm269;
		let mut fTemp274: F32 = self.fTempPerm270;
		let mut fTemp275: F32 = self.fTempPerm271;
		let mut fTemp276: F32 = self.fTempPerm272;
		let mut fTemp277: F32 = self.fTempPerm273;
		let mut fTemp278: F32 = self.fTempPerm274;
		let mut iTemp279: i32 = self.iTempPerm275;
		let mut iTemp280: i32 = self.iTempPerm276;
		let mut iTemp281: i32 = self.iTempPerm277;
		let mut iTemp282: i32 = self.iTempPerm278;
		let mut iTemp283: i32 = self.iTempPerm279;
		let mut fTemp284: F32 = self.fTempPerm280;
		let mut iTemp285: i32 = self.iTempPerm281;
		let mut iTemp286: i32 = self.iTempPerm282;
		let mut fTemp287: F32 = self.fTempPerm283;
		let mut fTemp288: F32 = self.fTempPerm284;
		let mut iTemp289: i32 = self.iTempPerm285;
		let mut iTemp290: i32 = self.iTempPerm286;
		let mut fTemp291: F32 = self.fTempPerm287;
		let mut fTemp292: F32 = self.fTempPerm288;
		let mut iTemp293: i32 = self.iTempPerm289;
		let mut fTemp294: F32 = self.fTempPerm290;
		let mut fTemp295: F32 = self.fTempPerm291;
		let mut iTemp296: i32 = self.iTempPerm292;
		let mut iTemp297: i32 = self.iTempPerm293;
		let mut fTemp298: F32 = self.fTempPerm294;
		let mut fTemp299: F32 = self.fTempPerm295;
		let mut fTemp300: F32 = self.fTempPerm296;
		let mut fTemp301: F32 = self.fTempPerm297;
		let mut fTemp302: F32 = self.fTempPerm298;
		let mut fTemp303: F32 = self.fTempPerm299;
		let mut iTemp304: i32 = self.iTempPerm300;
		let mut fTemp305: F32 = self.fTempPerm301;
		let mut fTemp306: F32 = self.fTempPerm302;
		let mut fTemp307: F32 = self.fTempPerm303;
		let mut iTemp308: i32 = self.iTempPerm304;
		let mut fTemp309: F32 = self.fTempPerm305;
		let mut fTemp310: F32 = self.fTempPerm306;
		let mut fTemp311: F32 = self.fTempPerm307;
		let mut fTemp312: F32 = self.fTempPerm308;
		let mut iTemp313: i32 = self.iTempPerm309;
		let mut iTemp314: i32 = self.iTempPerm310;
		let mut iTemp315: i32 = self.iTempPerm311;
		let mut fTemp316: F32 = self.fTempPerm312;
		let mut fTemp317: F32 = self.fTempPerm313;
		let mut iTemp318: i32 = self.iTempPerm314;
		let mut iTemp319: i32 = self.iTempPerm315;
		let mut fTemp320: F32 = self.fTempPerm316;
		let mut fTemp321: F32 = self.fTempPerm317;
		let mut fTemp322: F32 = self.fTempPerm318;
		let mut fTemp323: F32 = self.fTempPerm319;
		let mut iTemp324: i32 = self.iTempPerm320;
		let mut fTemp325: F32 = self.fTempPerm321;
		let mut fTemp326: F32 = self.fTempPerm322;
		let mut iTemp327: i32 = self.iTempPerm323;
		let mut iTemp328: i32 = self.iTempPerm324;
		let mut fTemp329: F32 = self.fTempPerm325;
		let mut fTemp330: F32 = self.fTempPerm326;
		let mut fTemp331: F32 = self.fTempPerm327;
		let mut iTemp332: i32 = self.iTempPerm328;
		let mut fTemp333: F32 = self.fTempPerm329;
		let mut fTemp334: F32 = self.fTempPerm330;
		let mut fTemp335: F32 = self.fTempPerm331;
		let mut iTemp336: i32 = self.iTempPerm332;
		let mut fTemp337: F32 = self.fTempPerm333;
		let mut fTemp338: F32 = self.fTempPerm334;
		let mut fTemp339: F32 = self.fTempPerm335;
		let mut fTemp340: F32 = self.fTempPerm336;
		let mut fTemp341: F32 = self.fTempPerm337;
		let mut fTemp342: F32 = self.fTempPerm338;
		let mut fTemp343: F32 = self.fTempPerm339;
		let mut iTemp344: i32 = self.iTempPerm340;
		let mut iTemp345: i32 = self.iTempPerm341;
		let mut iTemp346: i32 = self.iTempPerm342;
		let mut iTemp347: i32 = self.iTempPerm343;
		let mut iTemp348: i32 = self.iTempPerm344;
		let mut fTemp349: F32 = self.fTempPerm345;
		let mut iTemp350: i32 = self.iTempPerm346;
		let mut iTemp351: i32 = self.iTempPerm347;
		let mut fTemp352: F32 = self.fTempPerm348;
		let mut fTemp353: F32 = self.fTempPerm349;
		let mut iTemp354: i32 = self.iTempPerm350;
		let mut iTemp355: i32 = self.iTempPerm351;
		let mut fTemp356: F32 = self.fTempPerm352;
		let mut fTemp357: F32 = self.fTempPerm353;
		let mut iTemp358: i32 = self.iTempPerm354;
		let mut fTemp359: F32 = self.fTempPerm355;
		let mut fTemp360: F32 = self.fTempPerm356;
		let mut iTemp361: i32 = self.iTempPerm357;
		let mut iTemp362: i32 = self.iTempPerm358;
		let mut fTemp363: F32 = self.fTempPerm359;
		let mut fTemp364: F32 = self.fTempPerm360;
		let mut fTemp365: F32 = self.fTempPerm361;
		let mut fTemp366: F32 = self.fTempPerm362;
		let mut fTemp367: F32 = self.fTempPerm363;
		let mut fTemp368: F32 = self.fTempPerm364;
		let mut iTemp369: i32 = self.iTempPerm365;
		let mut fTemp370: F32 = self.fTempPerm366;
		let mut fTemp371: F32 = self.fTempPerm367;
		let mut fTemp372: F32 = self.fTempPerm368;
		let mut iTemp373: i32 = self.iTempPerm369;
		let mut fTemp374: F32 = self.fTempPerm370;
		let mut fTemp375: F32 = self.fTempPerm371;
		let mut fTemp376: F32 = self.fTempPerm372;
		let mut fTemp377: F32 = self.fTempPerm373;
		let mut iTemp378: i32 = self.iTempPerm374;
		let mut iTemp379: i32 = self.iTempPerm375;
		let mut iTemp380: i32 = self.iTempPerm376;
		let mut fTemp381: F32 = self.fTempPerm377;
		let mut fTemp382: F32 = self.fTempPerm378;
		let mut iTemp383: i32 = self.iTempPerm379;
		let mut iTemp384: i32 = self.iTempPerm380;
		let mut fTemp385: F32 = self.fTempPerm381;
		let mut fTemp386: F32 = self.fTempPerm382;
		let mut fTemp387: F32 = self.fTempPerm383;
		let mut fTemp388: F32 = self.fTempPerm384;
		let mut iTemp389: i32 = self.iTempPerm385;
		let mut fTemp390: F32 = self.fTempPerm386;
		let mut fTemp391: F32 = self.fTempPerm387;
		let mut iTemp392: i32 = self.iTempPerm388;
		let mut iTemp393: i32 = self.iTempPerm389;
		let mut fTemp394: F32 = self.fTempPerm390;
		let mut fTemp395: F32 = self.fTempPerm391;
		let mut fTemp396: F32 = self.fTempPerm392;
		let mut iTemp397: i32 = self.iTempPerm393;
		let mut fTemp398: F32 = self.fTempPerm394;
		let mut fTemp399: F32 = self.fTempPerm395;
		let mut fTemp400: F32 = self.fTempPerm396;
		let mut iTemp401: i32 = self.iTempPerm397;
		let mut fTemp402: F32 = self.fTempPerm398;
		let mut fTemp403: F32 = self.fTempPerm399;
		let mut fTemp404: F32 = self.fTempPerm400;
		let mut fTemp405: F32 = self.fTempPerm401;
		let mut fTemp406: F32 = self.fTempPerm402;
		let mut fTemp407: F32 = self.fTempPerm403;
		let mut fTemp408: F32 = self.fTempPerm404;
		let mut iTemp409: i32 = self.iTempPerm405;
		let mut iTemp410: i32 = self.iTempPerm406;
		let mut iTemp411: i32 = self.iTempPerm407;
		let mut iTemp412: i32 = self.iTempPerm408;
		let mut iTemp413: i32 = self.iTempPerm409;
		let mut fTemp414: F32 = self.fTempPerm410;
		let mut iTemp415: i32 = self.iTempPerm411;
		let mut iTemp416: i32 = self.iTempPerm412;
		let mut fTemp417: F32 = self.fTempPerm413;
		let mut fTemp418: F32 = self.fTempPerm414;
		let mut iTemp419: i32 = self.iTempPerm415;
		let mut iTemp420: i32 = self.iTempPerm416;
		let mut fTemp421: F32 = self.fTempPerm417;
		let mut fTemp422: F32 = self.fTempPerm418;
		let mut iTemp423: i32 = self.iTempPerm419;
		let mut fTemp424: F32 = self.fTempPerm420;
		let mut fTemp425: F32 = self.fTempPerm421;
		let mut iTemp426: i32 = self.iTempPerm422;
		let mut iTemp427: i32 = self.iTempPerm423;
		let mut fTemp428: F32 = self.fTempPerm424;
		let mut fTemp429: F32 = self.fTempPerm425;
		let mut fTemp430: F32 = self.fTempPerm426;
		let mut fTemp431: F32 = self.fTempPerm427;
		let mut fTemp432: F32 = self.fTempPerm428;
		let mut fTemp433: F32 = self.fTempPerm429;
		let mut iTemp434: i32 = self.iTempPerm430;
		let mut fTemp435: F32 = self.fTempPerm431;
		let mut fTemp436: F32 = self.fTempPerm432;
		let mut fTemp437: F32 = self.fTempPerm433;
		let mut iTemp438: i32 = self.iTempPerm434;
		let mut fTemp439: F32 = self.fTempPerm435;
		let mut fTemp440: F32 = self.fTempPerm436;
		let mut fTemp441: F32 = self.fTempPerm437;
		let mut fTemp442: F32 = self.fTempPerm438;
		let mut iTemp443: i32 = self.iTempPerm439;
		let mut iTemp444: i32 = self.iTempPerm440;
		let mut iTemp445: i32 = self.iTempPerm441;
		let mut fTemp446: F32 = self.fTempPerm442;
		let mut fTemp447: F32 = self.fTempPerm443;
		let mut iTemp448: i32 = self.iTempPerm444;
		let mut iTemp449: i32 = self.iTempPerm445;
		let mut fTemp450: F32 = self.fTempPerm446;
		let mut fTemp451: F32 = self.fTempPerm447;
		let mut fTemp452: F32 = self.fTempPerm448;
		let mut fTemp453: F32 = self.fTempPerm449;
		let mut iTemp454: i32 = self.iTempPerm450;
		let mut fTemp455: F32 = self.fTempPerm451;
		let mut fTemp456: F32 = self.fTempPerm452;
		let mut iTemp457: i32 = self.iTempPerm453;
		let mut iTemp458: i32 = self.iTempPerm454;
		let mut fTemp459: F32 = self.fTempPerm455;
		let mut fTemp460: F32 = self.fTempPerm456;
		let mut fTemp461: F32 = self.fTempPerm457;
		let mut iTemp462: i32 = self.iTempPerm458;
		let mut fTemp463: F32 = self.fTempPerm459;
		let mut fTemp464: F32 = self.fTempPerm460;
		let mut fTemp465: F32 = self.fTempPerm461;
		let mut iTemp466: i32 = self.iTempPerm462;
		let mut fTemp467: F32 = self.fTempPerm463;
		let mut fTemp468: F32 = self.fTempPerm464;
		let mut fTemp469: F32 = self.fTempPerm465;
		let mut fTemp470: F32 = self.fTempPerm466;
		let mut fTemp471: F32 = self.fTempPerm467;
		let mut fTemp472: F32 = self.fTempPerm468;
		let mut fTemp473: F32 = self.fTempPerm469;
		let mut iTemp474: i32 = self.iTempPerm470;
		let mut iTemp475: i32 = self.iTempPerm471;
		let mut iTemp476: i32 = self.iTempPerm472;
		let mut iTemp477: i32 = self.iTempPerm473;
		let mut iTemp478: i32 = self.iTempPerm474;
		let mut fTemp479: F32 = self.fTempPerm475;
		let mut iTemp480: i32 = self.iTempPerm476;
		let mut iTemp481: i32 = self.iTempPerm477;
		let mut fTemp482: F32 = self.fTempPerm478;
		let mut fTemp483: F32 = self.fTempPerm479;
		let mut iTemp484: i32 = self.iTempPerm480;
		let mut iTemp485: i32 = self.iTempPerm481;
		let mut fTemp486: F32 = self.fTempPerm482;
		let mut fTemp487: F32 = self.fTempPerm483;
		let mut iTemp488: i32 = self.iTempPerm484;
		let mut fTemp489: F32 = self.fTempPerm485;
		let mut fTemp490: F32 = self.fTempPerm486;
		let mut iTemp491: i32 = self.iTempPerm487;
		let mut iTemp492: i32 = self.iTempPerm488;
		let mut fTemp493: F32 = self.fTempPerm489;
		let mut fTemp494: F32 = self.fTempPerm490;
		let mut fTemp495: F32 = self.fTempPerm491;
		let mut fTemp496: F32 = self.fTempPerm492;
		let mut fTemp497: F32 = self.fTempPerm493;
		let mut fTemp498: F32 = self.fTempPerm494;
		let mut iTemp499: i32 = self.iTempPerm495;
		let mut fTemp500: F32 = self.fTempPerm496;
		let mut fTemp501: F32 = self.fTempPerm497;
		let mut fTemp502: F32 = self.fTempPerm498;
		let mut iTemp503: i32 = self.iTempPerm499;
		let mut fTemp504: F32 = self.fTempPerm500;
		let mut fTemp505: F32 = self.fTempPerm501;
		let mut fTemp506: F32 = self.fTempPerm502;
		let mut fTemp507: F32 = self.fTempPerm503;
		let mut iTemp508: i32 = self.iTempPerm504;
		let mut iTemp509: i32 = self.iTempPerm505;
		let mut iTemp510: i32 = self.iTempPerm506;
		let mut fTemp511: F32 = self.fTempPerm507;
		let mut fTemp512: F32 = self.fTempPerm508;
		let mut iTemp513: i32 = self.iTempPerm509;
		let mut iTemp514: i32 = self.iTempPerm510;
		let mut fTemp515: F32 = self.fTempPerm511;
		let mut fTemp516: F32 = self.fTempPerm512;
		let mut fTemp517: F32 = self.fTempPerm513;
		let mut fTemp518: F32 = self.fTempPerm514;
		let mut iTemp519: i32 = self.iTempPerm515;
		let mut fTemp520: F32 = self.fTempPerm516;
		let mut fTemp521: F32 = self.fTempPerm517;
		let mut iTemp522: i32 = self.iTempPerm518;
		let mut iTemp523: i32 = self.iTempPerm519;
		let mut fTemp524: F32 = self.fTempPerm520;
		let mut fTemp525: F32 = self.fTempPerm521;
		let mut fTemp526: F32 = self.fTempPerm522;
		let mut iTemp527: i32 = self.iTempPerm523;
		let mut fTemp528: F32 = self.fTempPerm524;
		let mut fTemp529: F32 = self.fTempPerm525;
		let mut fTemp530: F32 = self.fTempPerm526;
		let mut iTemp531: i32 = self.iTempPerm527;
		let mut fTemp532: F32 = self.fTempPerm528;
		let mut fTemp533: F32 = self.fTempPerm529;
		let mut fTemp534: F32 = self.fTempPerm530;
		let mut fTemp535: F32 = self.fTempPerm531;
		let mut fTemp536: F32 = self.fTempPerm532;
		let mut fTemp537: F32 = self.fTempPerm533;
		let mut fTemp538: F32 = self.fTempPerm534;
		let mut iTemp539: i32 = self.iTempPerm535;
		let mut iTemp540: i32 = self.iTempPerm536;
		let mut iTemp541: i32 = self.iTempPerm537;
		let mut iTemp542: i32 = self.iTempPerm538;
		let mut iTemp543: i32 = self.iTempPerm539;
		let mut fTemp544: F32 = self.fTempPerm540;
		let mut iTemp545: i32 = self.iTempPerm541;
		let mut iTemp546: i32 = self.iTempPerm542;
		let mut fTemp547: F32 = self.fTempPerm543;
		let mut fTemp548: F32 = self.fTempPerm544;
		let mut iTemp549: i32 = self.iTempPerm545;
		let mut iTemp550: i32 = self.iTempPerm546;
		let mut fTemp551: F32 = self.fTempPerm547;
		let mut fTemp552: F32 = self.fTempPerm548;
		let mut iTemp553: i32 = self.iTempPerm549;
		let mut fTemp554: F32 = self.fTempPerm550;
		let mut fTemp555: F32 = self.fTempPerm551;
		let mut iTemp556: i32 = self.iTempPerm552;
		let mut iTemp557: i32 = self.iTempPerm553;
		let mut fTemp558: F32 = self.fTempPerm554;
		let mut fTemp559: F32 = self.fTempPerm555;
		let mut fTemp560: F32 = self.fTempPerm556;
		let mut fTemp561: F32 = self.fTempPerm557;
		let mut fTemp562: F32 = self.fTempPerm558;
		let mut fTemp563: F32 = self.fTempPerm559;
		let mut iTemp564: i32 = self.iTempPerm560;
		let mut fTemp565: F32 = self.fTempPerm561;
		let mut fTemp566: F32 = self.fTempPerm562;
		let mut fTemp567: F32 = self.fTempPerm563;
		let mut iTemp568: i32 = self.iTempPerm564;
		let mut fTemp569: F32 = self.fTempPerm565;
		let mut fTemp570: F32 = self.fTempPerm566;
		let mut fTemp571: F32 = self.fTempPerm567;
		let mut fTemp572: F32 = self.fTempPerm568;
		let mut iTemp573: i32 = self.iTempPerm569;
		let mut iTemp574: i32 = self.iTempPerm570;
		let mut iTemp575: i32 = self.iTempPerm571;
		let mut fTemp576: F32 = self.fTempPerm572;
		let mut fTemp577: F32 = self.fTempPerm573;
		let mut iTemp578: i32 = self.iTempPerm574;
		let mut iTemp579: i32 = self.iTempPerm575;
		let mut fTemp580: F32 = self.fTempPerm576;
		let mut fTemp581: F32 = self.fTempPerm577;
		let mut fTemp582: F32 = self.fTempPerm578;
		let mut fTemp583: F32 = self.fTempPerm579;
		let mut iTemp584: i32 = self.iTempPerm580;
		let mut fTemp585: F32 = self.fTempPerm581;
		let mut fTemp586: F32 = self.fTempPerm582;
		let mut iTemp587: i32 = self.iTempPerm583;
		let mut iTemp588: i32 = self.iTempPerm584;
		let mut fTemp589: F32 = self.fTempPerm585;
		let mut fTemp590: F32 = self.fTempPerm586;
		let mut fTemp591: F32 = self.fTempPerm587;
		let mut iTemp592: i32 = self.iTempPerm588;
		let mut fTemp593: F32 = self.fTempPerm589;
		let mut fTemp594: F32 = self.fTempPerm590;
		let mut fTemp595: F32 = self.fTempPerm591;
		let mut iTemp596: i32 = self.iTempPerm592;
		let mut fTemp597: F32 = self.fTempPerm593;
		let mut fTemp598: F32 = self.fTempPerm594;
		let mut fTemp599: F32 = self.fTempPerm595;
		let mut fTemp600: F32 = self.fTempPerm596;
		let mut fTemp601: F32 = self.fTempPerm597;
		let mut fTemp602: F32 = self.fTempPerm598;
		let mut fTemp603: F32 = self.fTempPerm599;
		let mut iTemp604: i32 = self.iTempPerm600;
		let mut iTemp605: i32 = self.iTempPerm601;
		let mut iTemp606: i32 = self.iTempPerm602;
		let mut iTemp607: i32 = self.iTempPerm603;
		let mut iTemp608: i32 = self.iTempPerm604;
		let mut fTemp609: F32 = self.fTempPerm605;
		let mut iTemp610: i32 = self.iTempPerm606;
		let mut iTemp611: i32 = self.iTempPerm607;
		let mut fTemp612: F32 = self.fTempPerm608;
		let mut fTemp613: F32 = self.fTempPerm609;
		let mut iTemp614: i32 = self.iTempPerm610;
		let mut iTemp615: i32 = self.iTempPerm611;
		let mut fTemp616: F32 = self.fTempPerm612;
		let mut fTemp617: F32 = self.fTempPerm613;
		let mut iTemp618: i32 = self.iTempPerm614;
		let mut fTemp619: F32 = self.fTempPerm615;
		let mut fTemp620: F32 = self.fTempPerm616;
		let mut iTemp621: i32 = self.iTempPerm617;
		let mut iTemp622: i32 = self.iTempPerm618;
		let mut fTemp623: F32 = self.fTempPerm619;
		let mut fTemp624: F32 = self.fTempPerm620;
		let mut fTemp625: F32 = self.fTempPerm621;
		let mut fTemp626: F32 = self.fTempPerm622;
		let mut fTemp627: F32 = self.fTempPerm623;
		let mut fTemp628: F32 = self.fTempPerm624;
		let mut iTemp629: i32 = self.iTempPerm625;
		let mut fTemp630: F32 = self.fTempPerm626;
		let mut fTemp631: F32 = self.fTempPerm627;
		let mut fTemp632: F32 = self.fTempPerm628;
		let mut iTemp633: i32 = self.iTempPerm629;
		let mut fTemp634: F32 = self.fTempPerm630;
		let mut fTemp635: F32 = self.fTempPerm631;
		let mut fTemp636: F32 = self.fTempPerm632;
		let mut fTemp637: F32 = self.fTempPerm633;
		let mut iTemp638: i32 = self.iTempPerm634;
		let mut iTemp639: i32 = self.iTempPerm635;
		let mut iTemp640: i32 = self.iTempPerm636;
		let mut fTemp641: F32 = self.fTempPerm637;
		let mut fTemp642: F32 = self.fTempPerm638;
		let mut iTemp643: i32 = self.iTempPerm639;
		let mut iTemp644: i32 = self.iTempPerm640;
		let mut fTemp645: F32 = self.fTempPerm641;
		let mut fTemp646: F32 = self.fTempPerm642;
		let mut fTemp647: F32 = self.fTempPerm643;
		let mut fTemp648: F32 = self.fTempPerm644;
		let mut iTemp649: i32 = self.iTempPerm645;
		let mut fTemp650: F32 = self.fTempPerm646;
		let mut fTemp651: F32 = self.fTempPerm647;
		let mut iTemp652: i32 = self.iTempPerm648;
		let mut iTemp653: i32 = self.iTempPerm649;
		let mut fTemp654: F32 = self.fTempPerm650;
		let mut fTemp655: F32 = self.fTempPerm651;
		let mut fTemp656: F32 = self.fTempPerm652;
		let mut iTemp657: i32 = self.iTempPerm653;
		let mut fTemp658: F32 = self.fTempPerm654;
		let mut fTemp659: F32 = self.fTempPerm655;
		let mut fTemp660: F32 = self.fTempPerm656;
		let mut iTemp661: i32 = self.iTempPerm657;
		let mut fTemp662: F32 = self.fTempPerm658;
		let mut fTemp663: F32 = self.fTempPerm659;
		let mut fTemp664: F32 = self.fTempPerm660;
		let mut fTemp665: F32 = self.fTempPerm661;
		let mut fTemp666: F32 = self.fTempPerm662;
		let mut fTemp667: F32 = self.fTempPerm663;
		let mut fTemp668: F32 = self.fTempPerm664;
		let mut iTemp669: i32 = self.iTempPerm665;
		let mut iTemp670: i32 = self.iTempPerm666;
		let mut iTemp671: i32 = self.iTempPerm667;
		let mut iTemp672: i32 = self.iTempPerm668;
		let mut iTemp673: i32 = self.iTempPerm669;
		let mut fTemp674: F32 = self.fTempPerm670;
		let mut iTemp675: i32 = self.iTempPerm671;
		let mut iTemp676: i32 = self.iTempPerm672;
		let mut fTemp677: F32 = self.fTempPerm673;
		let mut fTemp678: F32 = self.fTempPerm674;
		let mut iTemp679: i32 = self.iTempPerm675;
		let mut iTemp680: i32 = self.iTempPerm676;
		let mut fTemp681: F32 = self.fTempPerm677;
		let mut fTemp682: F32 = self.fTempPerm678;
		let mut iTemp683: i32 = self.iTempPerm679;
		let mut fTemp684: F32 = self.fTempPerm680;
		let mut fTemp685: F32 = self.fTempPerm681;
		let mut iTemp686: i32 = self.iTempPerm682;
		let mut iTemp687: i32 = self.iTempPerm683;
		let mut fTemp688: F32 = self.fTempPerm684;
		let mut fTemp689: F32 = self.fTempPerm685;
		let mut fTemp690: F32 = self.fTempPerm686;
		let mut fTemp691: F32 = self.fTempPerm687;
		let mut fTemp692: F32 = self.fTempPerm688;
		let mut fTemp693: F32 = self.fTempPerm689;
		let mut iTemp694: i32 = self.iTempPerm690;
		let mut fTemp695: F32 = self.fTempPerm691;
		let mut fTemp696: F32 = self.fTempPerm692;
		let mut fTemp697: F32 = self.fTempPerm693;
		let mut iTemp698: i32 = self.iTempPerm694;
		let mut fTemp699: F32 = self.fTempPerm695;
		let mut fTemp700: F32 = self.fTempPerm696;
		let mut fTemp701: F32 = self.fTempPerm697;
		let mut fTemp702: F32 = self.fTempPerm698;
		let mut iTemp703: i32 = self.iTempPerm699;
		let mut iTemp704: i32 = self.iTempPerm700;
		let mut iTemp705: i32 = self.iTempPerm701;
		let mut fTemp706: F32 = self.fTempPerm702;
		let mut fTemp707: F32 = self.fTempPerm703;
		let mut iTemp708: i32 = self.iTempPerm704;
		let mut iTemp709: i32 = self.iTempPerm705;
		let mut fTemp710: F32 = self.fTempPerm706;
		let mut fTemp711: F32 = self.fTempPerm707;
		let mut fTemp712: F32 = self.fTempPerm708;
		let mut fTemp713: F32 = self.fTempPerm709;
		let mut iTemp714: i32 = self.iTempPerm710;
		let mut fTemp715: F32 = self.fTempPerm711;
		let mut fTemp716: F32 = self.fTempPerm712;
		let mut iTemp717: i32 = self.iTempPerm713;
		let mut iTemp718: i32 = self.iTempPerm714;
		let mut fTemp719: F32 = self.fTempPerm715;
		let mut fTemp720: F32 = self.fTempPerm716;
		let mut fTemp721: F32 = self.fTempPerm717;
		let mut iTemp722: i32 = self.iTempPerm718;
		let mut fTemp723: F32 = self.fTempPerm719;
		let mut fTemp724: F32 = self.fTempPerm720;
		let mut fTemp725: F32 = self.fTempPerm721;
		let mut iTemp726: i32 = self.iTempPerm722;
		let mut fTemp727: F32 = self.fTempPerm723;
		let mut fTemp728: F32 = self.fTempPerm724;
		let mut fTemp729: F32 = self.fTempPerm725;
		let mut fTemp730: F32 = self.fTempPerm726;
		let mut fTemp731: F32 = self.fTempPerm727;
		let mut fTemp732: F32 = self.fTempPerm728;
		let mut fTemp733: F32 = self.fTempPerm729;
		let mut iTemp734: i32 = self.iTempPerm730;
		let mut iTemp735: i32 = self.iTempPerm731;
		let mut iTemp736: i32 = self.iTempPerm732;
		let mut iTemp737: i32 = self.iTempPerm733;
		let mut iTemp738: i32 = self.iTempPerm734;
		let mut fTemp739: F32 = self.fTempPerm735;
		let mut iTemp740: i32 = self.iTempPerm736;
		let mut fTemp741: F32 = self.fTempPerm737;
		let mut fTemp742: F32 = self.fTempPerm738;
		let mut iTemp743: i32 = self.iTempPerm739;
		let mut iTemp744: i32 = self.iTempPerm740;
		let mut fTemp745: F32 = self.fTempPerm741;
		let mut fTemp746: F32 = self.fTempPerm742;
		let mut iTemp747: i32 = self.iTempPerm743;
		let mut fTemp748: F32 = self.fTempPerm744;
		let mut fTemp749: F32 = self.fTempPerm745;
		let mut iTemp750: i32 = self.iTempPerm746;
		let mut iTemp751: i32 = self.iTempPerm747;
		let mut fTemp752: F32 = self.fTempPerm748;
		let mut fTemp753: F32 = self.fTempPerm749;
		let mut fTemp754: F32 = self.fTempPerm750;
		let mut fTemp755: F32 = self.fTempPerm751;
		let mut fTemp756: F32 = self.fTempPerm752;
		let mut fTemp757: F32 = self.fTempPerm753;
		let mut iTemp758: i32 = self.iTempPerm754;
		let mut fTemp759: F32 = self.fTempPerm755;
		let mut fTemp760: F32 = self.fTempPerm756;
		let mut fTemp761: F32 = self.fTempPerm757;
		let mut iTemp762: i32 = self.iTempPerm758;
		let mut fTemp763: F32 = self.fTempPerm759;
		let mut fTemp764: F32 = self.fTempPerm760;
		let mut fTemp765: F32 = self.fTempPerm761;
		let mut fTemp766: F32 = self.fTempPerm762;
		let mut iTemp767: i32 = self.iTempPerm763;
		let mut iTemp768: i32 = self.iTempPerm764;
		let mut iTemp769: i32 = self.iTempPerm765;
		let mut fTemp770: F32 = self.fTempPerm766;
		let mut fTemp771: F32 = self.fTempPerm767;
		let mut iTemp772: i32 = self.iTempPerm768;
		let mut iTemp773: i32 = self.iTempPerm769;
		let mut fTemp774: F32 = self.fTempPerm770;
		let mut fTemp775: F32 = self.fTempPerm771;
		let mut fTemp776: F32 = self.fTempPerm772;
		let mut fTemp777: F32 = self.fTempPerm773;
		let mut iTemp778: i32 = self.iTempPerm774;
		let mut fTemp779: F32 = self.fTempPerm775;
		let mut fTemp780: F32 = self.fTempPerm776;
		let mut iTemp781: i32 = self.iTempPerm777;
		let mut iTemp782: i32 = self.iTempPerm778;
		let mut fTemp783: F32 = self.fTempPerm779;
		let mut fTemp784: F32 = self.fTempPerm780;
		let mut fTemp785: F32 = self.fTempPerm781;
		let mut iTemp786: i32 = self.iTempPerm782;
		let mut fTemp787: F32 = self.fTempPerm783;
		let mut fTemp788: F32 = self.fTempPerm784;
		let mut fTemp789: F32 = self.fTempPerm785;
		let mut iTemp790: i32 = self.iTempPerm786;
		let mut fTemp791: F32 = self.fTempPerm787;
		let mut fTemp792: F32 = self.fTempPerm788;
		let mut fTemp793: F32 = self.fTempPerm789;
		let mut fTemp794: F32 = self.fTempPerm790;
		let mut fTemp795: F32 = self.fTempPerm791;
		let mut fTemp796: F32 = self.fTempPerm792;
		let mut fTemp797: F32 = self.fTempPerm793;
		let mut fTemp798: F32 = self.fTempPerm794;
		let mut fTemp799: F32 = self.fTempPerm795;
		let mut fTemp800: F32 = self.fTempPerm796;
		let mut fTemp801: F32 = self.fTempPerm797;
		let mut fTemp802: F32 = self.fTempPerm798;
		let mut fSlow60: F32 = ((iSlow10 != 0) as i32) as u32 as F32;
		let mut fTemp804: F32 = self.fTempPerm799;
		let mut fTemp805: F32 = self.fTempPerm800;
		let mut fTemp806: F32 = self.fTempPerm801;
		let mut fTemp807: F32 = self.fTempPerm802;
		let mut fTemp808: F32 = self.fTempPerm803;
		let mut fSlow61: F32 = ((iSlow16 != 0) as i32) as u32 as F32;
		let mut fTemp810: F32 = self.fTempPerm804;
		let mut fTemp811: F32 = self.fTempPerm805;
		let mut fTemp812: F32 = self.fTempPerm806;
		let mut fTemp813: F32 = self.fTempPerm807;
		let mut fTemp814: F32 = self.fTempPerm808;
		let mut fSlow62: F32 = ((iSlow13 != 0) as i32) as u32 as F32;
		let mut fTemp816: F32 = self.fTempPerm809;
		let mut fTemp817: F32 = self.fTempPerm810;
		let mut fTemp818: F32 = self.fTempPerm811;
		let mut fTemp819: F32 = self.fTempPerm812;
		let mut fTemp820: F32 = self.fTempPerm813;
		let mut fSlow63: F32 = ((iSlow19 != 0) as i32) as u32 as F32;
		let mut fTemp822: F32 = self.fTempPerm814;
		let mut fTemp823: F32 = self.fTempPerm815;
		let mut fTemp824: F32 = self.fTempPerm816;
		let mut fTemp825: F32 = self.fTempPerm817;
		let mut fTemp826: F32 = self.fTempPerm818;
		let mut fSlow64: F32 = ((iSlow37 != 0) as i32) as u32 as F32;
		let mut fTemp828: F32 = self.fTempPerm819;
		let mut fTemp829: F32 = self.fTempPerm820;
		let mut fTemp830: F32 = self.fTempPerm821;
		let mut fTemp831: F32 = self.fTempPerm822;
		let mut fTemp832: F32 = self.fTempPerm823;
		let mut fSlow65: F32 = ((iSlow34 != 0) as i32) as u32 as F32;
		let mut fTemp834: F32 = self.fTempPerm824;
		let mut fTemp835: F32 = self.fTempPerm825;
		let mut fTemp836: F32 = self.fTempPerm826;
		let mut fTemp837: F32 = self.fTempPerm827;
		let mut fTemp838: F32 = self.fTempPerm828;
		let mut fSlow66: F32 = ((iSlow31 != 0) as i32) as u32 as F32;
		let mut fTemp840: F32 = self.fTempPerm829;
		let mut fTemp841: F32 = self.fTempPerm830;
		let mut fTemp842: F32 = self.fTempPerm831;
		let mut fTemp843: F32 = self.fTempPerm832;
		let mut fTemp844: F32 = self.fTempPerm833;
		let mut fSlow67: F32 = ((iSlow28 != 0) as i32) as u32 as F32;
		let mut fTemp846: F32 = self.fTempPerm834;
		let mut fTemp847: F32 = self.fTempPerm835;
		let mut fTemp848: F32 = self.fTempPerm836;
		let mut fTemp849: F32 = self.fTempPerm837;
		let mut fTemp850: F32 = self.fTempPerm838;
		let mut fSlow68: F32 = ((iSlow25 != 0) as i32) as u32 as F32;
		let mut fTemp852: F32 = self.fTempPerm839;
		let mut fTemp853: F32 = self.fTempPerm840;
		let mut fTemp854: F32 = self.fTempPerm841;
		let mut fTemp855: F32 = self.fTempPerm842;
		let mut fTemp856: F32 = self.fTempPerm843;
		let mut fSlow69: F32 = ((iSlow22 != 0) as i32) as u32 as F32;
		let mut fTemp858: F32 = self.fTempPerm844;
		let mut fTemp859: F32 = self.fTempPerm845;
		let mut fTemp860: F32 = self.fTempPerm846;
		let mut fTemp861: F32 = self.fTempPerm847;
		let mut fTemp862: F32 = self.fTempPerm848;
		let mut fSlow70: F32 = ((iSlow7 != 0) as i32) as u32 as F32;
		let mut fTemp864: F32 = self.fTempPerm849;
		let mut fTemp865: F32 = self.fTempPerm850;
		let mut fTemp866: F32 = self.fTempPerm851;
		let mut fTemp867: F32 = self.fTempPerm852;
		let mut fTemp868: F32 = self.fTempPerm853;
		let mut fSlow71: F32 = ((iSlow3 != 0) as i32) as u32 as F32;
		let mut fSlow72: F32 = self.fHslider20;
		let mut fTemp872: F32 = self.fTempPerm854;
		let mut fTemp873: F32 = self.fTempPerm855;
		let mut fTemp874: F32 = self.fTempPerm856;
		let mut fTemp876: F32 = self.fTempPerm857;
		let mut fTemp877: F32 = self.fTempPerm858;
		let mut fTemp878: F32 = self.fTempPerm859;
		let mut fTemp880: F32 = self.fTempPerm860;
		let mut fTemp881: F32 = self.fTempPerm861;
		let mut fTemp882: F32 = self.fTempPerm862;
		let mut fTemp884: F32 = self.fTempPerm863;
		let mut fTemp885: F32 = self.fTempPerm864;
		let mut fTemp886: F32 = self.fTempPerm865;
		let mut fTemp888: F32 = self.fTempPerm866;
		let mut fTemp889: F32 = self.fTempPerm867;
		let mut fTemp890: F32 = self.fTempPerm868;
		let mut fTemp892: F32 = self.fTempPerm869;
		let mut fTemp893: F32 = self.fTempPerm870;
		let mut fTemp894: F32 = self.fTempPerm871;
		let mut fTemp896: F32 = self.fTempPerm872;
		let mut fTemp897: F32 = self.fTempPerm873;
		let mut fTemp898: F32 = self.fTempPerm874;
		let mut fTemp900: F32 = self.fTempPerm875;
		let mut fTemp901: F32 = self.fTempPerm876;
		let mut fTemp902: F32 = self.fTempPerm877;
		let mut fTemp904: F32 = self.fTempPerm878;
		let mut fTemp905: F32 = self.fTempPerm879;
		let mut fTemp906: F32 = self.fTempPerm880;
		let mut fTemp908: F32 = self.fTempPerm881;
		let mut fTemp909: F32 = self.fTempPerm882;
		let mut fTemp910: F32 = self.fTempPerm883;
		let mut fTemp912: F32 = self.fTempPerm884;
		let mut fTemp913: F32 = self.fTempPerm885;
		let mut fTemp914: F32 = self.fTempPerm886;
		let mut fTemp916: F32 = self.fTempPerm887;
		let mut fTemp917: F32 = self.fTempPerm888;
		let mut fTemp918: F32 = self.fTempPerm889;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut fRec166: F32 = 0.0;
			let mut fRec153: F32 = 0.0;
			let mut fRec140: F32 = 0.0;
			let mut fRec127: F32 = 0.0;
			let mut fRec114: F32 = 0.0;
			let mut fRec101: F32 = 0.0;
			let mut fRec88: F32 = 0.0;
			let mut fRec75: F32 = 0.0;
			let mut fRec62: F32 = 0.0;
			let mut fRec49: F32 = 0.0;
			let mut fRec36: F32 = 0.0;
			let mut fRec22: F32 = 0.0;
			let mut fRec208: F32 = 0.0;
			let mut fRec205: F32 = 0.0;
			let mut fRec202: F32 = 0.0;
			let mut fRec199: F32 = 0.0;
			let mut fRec196: F32 = 0.0;
			let mut fRec193: F32 = 0.0;
			let mut fRec190: F32 = 0.0;
			let mut fRec187: F32 = 0.0;
			let mut fRec184: F32 = 0.0;
			let mut fRec181: F32 = 0.0;
			let mut fRec178: F32 = 0.0;
			let mut fRec175: F32 = 0.0;
			self.iVec0[0] = 1;
			let mut fTemp0: F32 = (self.iVec0[1]) as F32;
			let mut fTemp1: F32 = 1.0 - 0.999 * fTemp0;
			self.fRec0[0] = fSlow0 * fTemp1 + 0.999 * fTemp0 * self.fRec0[1];
			let mut fTemp2: F32 = if (F32::abs(self.fRec0[0] - fSlow0) < 1e-06) as i32 != 0 {fSlow0} else {self.fRec0[0]};
			let mut fTemp3: F32 = *input0;
			if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
				self.fRec1[0] = fSlow6 * fTemp1 + 0.999 * fTemp0 * self.fRec1[1];
				iTemp4 = i32::wrapping_sub(1, self.iVec0[1]);
				iTemp5 = (iTemp4 > 0) as i32;
				self.fRec4[0] = self.fConst2 + (self.fRec4[1] - F32::floor(self.fConst2 + self.fRec4[1]));
				self.iRec5[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow41, iTemp4), self.iRec5[1])), 12345);
				iTemp6 = i32::wrapping_mul(1664525, self.iRec5[0] ^ (self.iRec5[0] >> 15));
				self.fRec3[0] = if ((self.fRec4[0] < self.fRec4[1]) as i32) | iTemp5 != 0 {4.656613e-10 * (i32::wrapping_mul(1013904223, iTemp6 ^ (iTemp6 >> 13))) as F32} else {self.fRec3[1]};
				self.fRec2[0] = self.fConst3 * self.fRec3[0] + self.fConst1 * self.fRec2[1];
				self.fRec6[0] = fSlow42 * fTemp1 + 0.999 * fTemp0 * self.fRec6[1];
				fTemp7 = F32::min(1.0, F32::max(0.0, if (F32::abs(self.fRec6[0] - fSlow42) < 1e-06) as i32 != 0 {fSlow42} else {self.fRec6[0]} * self.fRec2[0] + F32::min(1.0, F32::max(0.0, if (F32::abs(self.fRec1[0] - fSlow6) < 1e-06) as i32 != 0 {fSlow6} else {self.fRec1[0]}))));
			}
				if iSlow5 != 0 {
					iTemp8 = ((self.fRec9[1] >= 1.0) as i32) | iTemp5;
					iTemp9 = (self.fRec9[1] < 0.0) as i32;
				}
					if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
						self.fRec12[0] = fSlow43 * fTemp1 + 0.999 * fTemp0 * self.fRec12[1];
						fTemp10 = if (F32::abs(self.fRec12[0] - fSlow43) < 1e-06) as i32 != 0 {fSlow43} else {self.fRec12[0]};
						self.iRec13[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow46, iTemp4), self.iRec13[1])), 12345);
					}
						if iSlow5 != 0 {
							iTemp11 = i32::wrapping_add(self.iRec13[0], 8520844);
							iTemp12 = i32::wrapping_mul(1664525, iTemp11 ^ (iTemp11 >> 15));
							iTemp13 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp12 ^ (iTemp12 >> 13))) as F32 + 0.5))))) as i32;
							fTemp14 = if iTemp9 | iTemp5 != 0 {if (iTemp13 >= 4) as i32 != 0 {if (iTemp13 >= 6) as i32 != 0 {if (iTemp13 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp13 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp13 >= 2) as i32 != 0 {if (iTemp13 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp13 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec10[1]};
							iTemp15 = i32::wrapping_add(self.iRec13[0], 8449573);
							iTemp16 = i32::wrapping_mul(1664525, iTemp15 ^ (iTemp15 >> 15));
							fTemp17 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp16 ^ (iTemp16 >> 13))) as F32 + 0.5;
							fTemp18 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp14, std::f32::consts::E) + fTemp17 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp14)), std::f32::consts::E)) + -8.228819)));
							iTemp19 = i32::wrapping_add(self.iRec13[0], 8457492);
							iTemp20 = i32::wrapping_mul(1664525, iTemp19 ^ (iTemp19 >> 15));
							fTemp21 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp20 ^ (iTemp20 >> 13))) as F32;
							fTemp22 = F32::floor(fTemp21 - fTemp18);
							iTemp23 = (fTemp22) as i32;
							fTemp24 = fTemp21 - (fTemp18 + fTemp22);
						}
							if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
								self.fRec14[0] = fSlow55 * fTemp1 + 0.999 * fTemp0 * self.fRec14[1];
								fTemp25 = if (F32::abs(self.fRec14[0] - fSlow55) < 1e-06) as i32 != 0 {fSlow55} else {self.fRec14[0]};
							}
								if iSlow5 != 0 {
									self.fRec11[0] = if iTemp9 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp24 * (fTemp24 * (fTemp24 * (fTemp24 * (0.0013333 * fTemp24 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp23))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp23))) as F32))} else {self.fRec11[1]};
								}
									if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
										self.fRec15[0] = fSlow56 * fTemp1 + 0.999 * fTemp0 * self.fRec15[1];
										fTemp26 = F32::min(0.01 * if (F32::abs(self.fRec15[0] - fSlow56) < 1e-06) as i32 != 0 {fSlow56} else {self.fRec15[0]}, 0.98);
										fTemp27 = fTemp26 / (1.0 - fTemp26);
									}
										if iSlow5 != 0 {
											fTemp28 = (iTemp8) as F32;
										}
											if iSlow5 & ((fTemp28 != 0.0) as i32) != 0 {
												iTemp29 = i32::wrapping_add(self.iRec13[0], 8465411);
												iTemp30 = i32::wrapping_mul(1664525, iTemp29 ^ (iTemp29 >> 15));
												fTemp31 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp30 ^ (iTemp30 >> 13))) as F32;
												fTemp32 = fTemp31 + 1.5;
												fTemp33 = 0.5 - fTemp31;
												fTemp34 = AlternativesDsp_faustpower2_f(fTemp33) / AlternativesDsp_faustpower2_f(fTemp32);
											}
												if iSlow5 != 0 {
													self.fRec9[0] = if iTemp8 != 0 {-(2.0 * (fTemp33 * (15.0 - 4.0 * fTemp34) * fTemp28 / (fTemp32 * (15.0 - 9.0 * fTemp34))))} else {self.fRec9[1] + self.fConst6 * (if iTemp9 != 0 {fTemp27} else {1.0} / F32::max(self.fRec11[0], 0.005))};
													self.fRec10[0] = fTemp14;
													fTemp35 = F32::min(1.0, F32::max(0.0, self.fRec9[0]));
													fTemp36 = (iTemp9) as F32;
													iTemp37 = (fTemp36 != 0.0) as i32;
												}
													if iSlow5 & iTemp37 != 0 {
														fTemp38 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec10[0])), std::f32::consts::E) * fTemp17;
														fTemp39 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec10[0], std::f32::consts::E) + fTemp38) + -8.228819)));
														fTemp40 = F32::floor(fTemp21 - fTemp39);
														iTemp41 = (fTemp40) as i32;
														fTemp42 = fTemp21 - (fTemp39 + fTemp40);
														fTemp43 = fTemp36 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp42 * (fTemp42 * (fTemp42 * (fTemp42 * (0.0013333 * fTemp42 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp41))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp41))) as F32));
													}
														if iSlow5 != 0 {
															self.fRec16[0] = if iTemp9 != 0 {fTemp43} else {self.fRec16[1]};
															fTemp44 = F32::floor(41.666668 * self.fRec16[0] * fTemp35);
															self.fVec1[0] = fTemp44;
															fTemp45 = ((fTemp44 != self.fVec1[1]) as i32) as u32 as F32;
															iTemp46 = (fTemp45 != 0.0) as i32;
														}
															if iSlow5 & iTemp46 != 0 {
																iTemp47 = i32::wrapping_add(self.iRec13[0], 8497087);
																iTemp48 = i32::wrapping_mul(1664525, iTemp47 ^ (iTemp47 >> 15));
																fTemp49 = 4.656613e-10 * fTemp45 * (i32::wrapping_mul(1013904223, iTemp48 ^ (iTemp48 >> 13))) as F32;
															}
																if iSlow5 != 0 {
																	fTemp50 = ((self.fRec9[0] >= 0.0) as i32) as u32 as F32;
																	self.fRec8[0] = fTemp50 * (self.fRec8[1] + 0.35 * fTemp45 * fTemp49);
																	self.fRec7[0] = self.fConst7 * self.fRec8[0] + self.fConst4 * self.fRec7[1];
																}
																	if iSlow5 & iTemp37 != 0 {
																		iTemp51 = i32::wrapping_add(self.iRec13[0], 8481249);
																		iTemp52 = i32::wrapping_mul(1664525, iTemp51 ^ (iTemp51 >> 15));
																		fTemp53 = 0.5 * fTemp36 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp52 ^ (iTemp52 >> 13))) as F32 + 0.5 + 1.0);
																	}
																		if iSlow5 != 0 {
																			self.fRec17[0] = if iTemp9 != 0 {fTemp53} else {self.fRec17[1]};
																			fTemp54 = F32::min(1.0, F32::max(-1.0, self.fRec17[0] + self.fRec7[0] * fTemp7));
																		}
																			if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
																				self.fRec18[0] = fSlow57 * fTemp1 + 0.999 * fTemp0 * self.fRec18[1];
																				fTemp55 = if (F32::abs(self.fRec18[0] - fSlow57) < 1e-06) as i32 != 0 {fSlow57} else {self.fRec18[0]};
																				fTemp56 = ((fTemp55 > 0.0001) as i32) as u32 as F32;
																				fTemp57 = fTemp55 * fTemp56;
																				self.fRec19[0] = fSlow58 * fTemp1 + 0.999 * fTemp0 * self.fRec19[1];
																				fTemp58 = fTemp55 * if (F32::abs(self.fRec19[0] - fSlow58) < 1e-06) as i32 != 0 {fSlow58} else {self.fRec19[0]} * fTemp56;
																			}
																				if iSlow5 != 0 {
																					fTemp59 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp35) * AlternativesDsp_faustpower2_f(1.0 - fTemp35) * fTemp54 * if (fTemp54 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																					fTemp60 = F32::floor(fTemp59);
																					iTemp61 = (fTemp60) as i32;
																					fTemp62 = fTemp59 - fTemp60;
																					fTemp63 = (fTemp62 * (fTemp62 * (fTemp62 * (fTemp62 * (0.0013333 * fTemp62 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp61))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp61))) as F32 + -1.0;
																				}
																					if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
																						self.fRec23[0] = fSlow59 * fTemp1 + 0.999 * fTemp0 * self.fRec23[1];
																						fTemp64 = if (F32::abs(self.fRec23[0] - fSlow59) < 1e-06) as i32 != 0 {fSlow59} else {self.fRec23[0]};
																						fTemp65 = F32::min(1.92 / fTemp64, 0.75);
																					}
																						if iSlow5 & iTemp46 != 0 {
																							iTemp66 = i32::wrapping_add(self.iRec13[0], 8489168);
																							iTemp67 = i32::wrapping_mul(1664525, iTemp66 ^ (iTemp66 >> 15));
																							fTemp68 = 4.656613e-10 * fTemp45 * (i32::wrapping_mul(1013904223, iTemp67 ^ (iTemp67 >> 13))) as F32;
																						}
																							if iSlow5 != 0 {
																								self.fRec25[0] = fTemp50 * (self.fRec25[1] + 0.35 * fTemp45 * fTemp68);
																								self.fRec24[0] = self.fConst7 * self.fRec25[0] + self.fConst4 * self.fRec24[1];
																								fTemp69 = self.fRec24[0] * fTemp7 * fTemp65;
																								fTemp70 = F32::floor(fTemp69);
																								iTemp71 = (fTemp70) as i32;
																								fTemp72 = fTemp69 - fTemp70;
																							}
																								if iSlow5 & iTemp37 != 0 {
																									fTemp73 = 1.442695 * fTemp38;
																									fTemp74 = F32::floor(fTemp73);
																									iTemp75 = (fTemp74) as i32;
																									fTemp76 = fTemp73 - fTemp74;
																									fTemp77 = F32::round((fTemp76 * (fTemp76 * (fTemp76 * (fTemp76 * (0.0013333 * fTemp76 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp75))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp75))) as F32) * fTemp36;
																								}
																									if iSlow5 != 0 {
																										self.fRec26[0] = if iTemp9 != 0 {fTemp77} else {self.fRec26[1]};
																										fTemp78 = F32::min(self.fConst5, F32::max(2e+01, self.fRec10[0] * self.fRec26[0] * (fTemp72 * (fTemp72 * (fTemp72 * (fTemp72 * (0.0013333 * fTemp72 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp71))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp71))) as F32));
																										fTemp79 = AlternativesDsp_faustpower2_f(fTemp78);
																										fTemp80 = self.fConst10 * fTemp79 * (self.fConst9 * fTemp79 + -444.1322) + 105.0;
																										fTemp81 = fTemp78 * (105.0 - self.fConst11 * fTemp79);
																									}
																										if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
																											fTemp82 = 0.6666667 / fTemp64;
																										}
																											if iSlow5 != 0 {
																												fTemp83 = self.fConst12 * (fTemp81 * (fTemp82 + self.fConst12 * (fTemp81 / fTemp80)) / fTemp80) + 1.0;
																											}
																												if iSlow9 != 0 {
																													iTemp84 = ((self.fRec29[1] >= 1.0) as i32) | iTemp5;
																													iTemp85 = (self.fRec29[1] < 0.0) as i32;
																													iTemp86 = i32::wrapping_add(self.iRec13[0], 7752701);
																													iTemp87 = i32::wrapping_mul(1664525, iTemp86 ^ (iTemp86 >> 15));
																													iTemp88 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp87 ^ (iTemp87 >> 13))) as F32 + 0.5))))) as i32;
																													fTemp89 = if iTemp85 | iTemp5 != 0 {if (iTemp88 >= 4) as i32 != 0 {if (iTemp88 >= 6) as i32 != 0 {if (iTemp88 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp88 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp88 >= 2) as i32 != 0 {if (iTemp88 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp88 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec30[1]};
																													iTemp90 = i32::wrapping_add(self.iRec13[0], 7681430);
																													iTemp91 = i32::wrapping_mul(1664525, iTemp90 ^ (iTemp90 >> 15));
																													fTemp92 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp91 ^ (iTemp91 >> 13))) as F32 + 0.5;
																													fTemp93 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp89, std::f32::consts::E) + fTemp92 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp89)), std::f32::consts::E)) + -8.228819)));
																													iTemp94 = i32::wrapping_add(self.iRec13[0], 7689349);
																													iTemp95 = i32::wrapping_mul(1664525, iTemp94 ^ (iTemp94 >> 15));
																													fTemp96 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp95 ^ (iTemp95 >> 13))) as F32;
																													fTemp97 = F32::floor(fTemp96 - fTemp93);
																													iTemp98 = (fTemp97) as i32;
																													fTemp99 = fTemp96 - (fTemp93 + fTemp97);
																													self.fRec31[0] = if iTemp85 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp99 * (fTemp99 * (fTemp99 * (fTemp99 * (0.0013333 * fTemp99 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp98))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp98))) as F32))} else {self.fRec31[1]};
																													fTemp100 = (iTemp84) as F32;
																												}
																													if iSlow9 & ((fTemp100 != 0.0) as i32) != 0 {
																														iTemp101 = i32::wrapping_add(self.iRec13[0], 7697268);
																														iTemp102 = i32::wrapping_mul(1664525, iTemp101 ^ (iTemp101 >> 15));
																														fTemp103 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp102 ^ (iTemp102 >> 13))) as F32;
																														fTemp104 = fTemp103 + 1.5;
																														fTemp105 = 0.5 - fTemp103;
																														fTemp106 = AlternativesDsp_faustpower2_f(fTemp105) / AlternativesDsp_faustpower2_f(fTemp104);
																													}
																														if iSlow9 != 0 {
																															self.fRec29[0] = if iTemp84 != 0 {-(2.0 * (fTemp105 * (15.0 - 4.0 * fTemp106) * fTemp100 / (fTemp104 * (15.0 - 9.0 * fTemp106))))} else {self.fRec29[1] + self.fConst6 * (if iTemp85 != 0 {fTemp27} else {1.0} / F32::max(self.fRec31[0], 0.005))};
																															self.fRec30[0] = fTemp89;
																															fTemp107 = F32::min(1.0, F32::max(0.0, self.fRec29[0]));
																															fTemp108 = (iTemp85) as F32;
																															iTemp109 = (fTemp108 != 0.0) as i32;
																														}
																															if iSlow9 & iTemp109 != 0 {
																																fTemp110 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec30[0])), std::f32::consts::E) * fTemp92;
																																fTemp111 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec30[0], std::f32::consts::E) + fTemp110) + -8.228819)));
																																fTemp112 = F32::floor(fTemp96 - fTemp111);
																																iTemp113 = (fTemp112) as i32;
																																fTemp114 = fTemp96 - (fTemp111 + fTemp112);
																																fTemp115 = fTemp108 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp114 * (fTemp114 * (fTemp114 * (fTemp114 * (0.0013333 * fTemp114 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp113))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp113))) as F32));
																															}
																																if iSlow9 != 0 {
																																	self.fRec32[0] = if iTemp85 != 0 {fTemp115} else {self.fRec32[1]};
																																	fTemp116 = F32::floor(41.666668 * self.fRec32[0] * fTemp107);
																																	self.fVec2[0] = fTemp116;
																																	fTemp117 = ((fTemp116 != self.fVec2[1]) as i32) as u32 as F32;
																																	iTemp118 = (fTemp117 != 0.0) as i32;
																																}
																																	if iSlow9 & iTemp118 != 0 {
																																		iTemp119 = i32::wrapping_add(self.iRec13[0], 7728944);
																																		iTemp120 = i32::wrapping_mul(1664525, iTemp119 ^ (iTemp119 >> 15));
																																		fTemp121 = 4.656613e-10 * fTemp117 * (i32::wrapping_mul(1013904223, iTemp120 ^ (iTemp120 >> 13))) as F32;
																																	}
																																		if iSlow9 != 0 {
																																			fTemp122 = ((self.fRec29[0] >= 0.0) as i32) as u32 as F32;
																																			self.fRec28[0] = fTemp122 * (self.fRec28[1] + 0.35 * fTemp117 * fTemp121);
																																			self.fRec27[0] = self.fConst7 * self.fRec28[0] + self.fConst4 * self.fRec27[1];
																																		}
																																			if iSlow9 & iTemp109 != 0 {
																																				iTemp123 = i32::wrapping_add(self.iRec13[0], 7713106);
																																				iTemp124 = i32::wrapping_mul(1664525, iTemp123 ^ (iTemp123 >> 15));
																																				fTemp125 = 0.5 * fTemp108 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp124 ^ (iTemp124 >> 13))) as F32 + 0.5 + 1.0);
																																			}
																																				if iSlow9 != 0 {
																																					self.fRec33[0] = if iTemp85 != 0 {fTemp125} else {self.fRec33[1]};
																																					fTemp126 = F32::min(1.0, F32::max(-1.0, self.fRec33[0] + self.fRec27[0] * fTemp7));
																																					fTemp127 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp107) * AlternativesDsp_faustpower2_f(1.0 - fTemp107) * fTemp126 * if (fTemp126 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																					fTemp128 = F32::floor(fTemp127);
																																					iTemp129 = (fTemp128) as i32;
																																					fTemp130 = fTemp127 - fTemp128;
																																					fTemp131 = (fTemp130 * (fTemp130 * (fTemp130 * (fTemp130 * (0.0013333 * fTemp130 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp129))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp129))) as F32 + -1.0;
																																				}
																																					if iSlow9 & iTemp118 != 0 {
																																						iTemp132 = i32::wrapping_add(self.iRec13[0], 7721025);
																																						iTemp133 = i32::wrapping_mul(1664525, iTemp132 ^ (iTemp132 >> 15));
																																						fTemp134 = 4.656613e-10 * fTemp117 * (i32::wrapping_mul(1013904223, iTemp133 ^ (iTemp133 >> 13))) as F32;
																																					}
																																						if iSlow9 != 0 {
																																							self.fRec38[0] = fTemp122 * (self.fRec38[1] + 0.35 * fTemp117 * fTemp134);
																																							self.fRec37[0] = self.fConst7 * self.fRec38[0] + self.fConst4 * self.fRec37[1];
																																							fTemp135 = self.fRec37[0] * fTemp7 * fTemp65;
																																							fTemp136 = F32::floor(fTemp135);
																																							iTemp137 = (fTemp136) as i32;
																																							fTemp138 = fTemp135 - fTemp136;
																																						}
																																							if iSlow9 & iTemp109 != 0 {
																																								fTemp139 = 1.442695 * fTemp110;
																																								fTemp140 = F32::floor(fTemp139);
																																								iTemp141 = (fTemp140) as i32;
																																								fTemp142 = fTemp139 - fTemp140;
																																								fTemp143 = F32::round((fTemp142 * (fTemp142 * (fTemp142 * (fTemp142 * (0.0013333 * fTemp142 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp141))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp141))) as F32) * fTemp108;
																																							}
																																								if iSlow9 != 0 {
																																									self.fRec39[0] = if iTemp85 != 0 {fTemp143} else {self.fRec39[1]};
																																									fTemp144 = F32::min(self.fConst5, F32::max(2e+01, self.fRec30[0] * self.fRec39[0] * (fTemp138 * (fTemp138 * (fTemp138 * (fTemp138 * (0.0013333 * fTemp138 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp137))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp137))) as F32));
																																									fTemp145 = AlternativesDsp_faustpower2_f(fTemp144);
																																									fTemp146 = self.fConst10 * fTemp145 * (self.fConst9 * fTemp145 + -444.1322) + 105.0;
																																									fTemp147 = fTemp144 * (105.0 - self.fConst11 * fTemp145);
																																									fTemp148 = self.fConst12 * (fTemp147 * (fTemp82 + self.fConst12 * (fTemp147 / fTemp146)) / fTemp146) + 1.0;
																																								}
																																									if iSlow24 != 0 {
																																										iTemp149 = ((self.fRec42[1] >= 1.0) as i32) | iTemp5;
																																										iTemp150 = (self.fRec42[1] < 0.0) as i32;
																																										iTemp151 = i32::wrapping_add(self.iRec13[0], 6984558);
																																										iTemp152 = i32::wrapping_mul(1664525, iTemp151 ^ (iTemp151 >> 15));
																																										iTemp153 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp152 ^ (iTemp152 >> 13))) as F32 + 0.5))))) as i32;
																																										fTemp154 = if iTemp150 | iTemp5 != 0 {if (iTemp153 >= 4) as i32 != 0 {if (iTemp153 >= 6) as i32 != 0 {if (iTemp153 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp153 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp153 >= 2) as i32 != 0 {if (iTemp153 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp153 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec43[1]};
																																										iTemp155 = i32::wrapping_add(self.iRec13[0], 6913287);
																																										iTemp156 = i32::wrapping_mul(1664525, iTemp155 ^ (iTemp155 >> 15));
																																										fTemp157 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp156 ^ (iTemp156 >> 13))) as F32 + 0.5;
																																										fTemp158 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp154, std::f32::consts::E) + fTemp157 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp154)), std::f32::consts::E)) + -8.228819)));
																																										iTemp159 = i32::wrapping_add(self.iRec13[0], 6921206);
																																										iTemp160 = i32::wrapping_mul(1664525, iTemp159 ^ (iTemp159 >> 15));
																																										fTemp161 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp160 ^ (iTemp160 >> 13))) as F32;
																																										fTemp162 = F32::floor(fTemp161 - fTemp158);
																																										iTemp163 = (fTemp162) as i32;
																																										fTemp164 = fTemp161 - (fTemp158 + fTemp162);
																																										self.fRec44[0] = if iTemp150 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp164 * (fTemp164 * (fTemp164 * (fTemp164 * (0.0013333 * fTemp164 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp163))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp163))) as F32))} else {self.fRec44[1]};
																																										fTemp165 = (iTemp149) as F32;
																																									}
																																										if iSlow24 & ((fTemp165 != 0.0) as i32) != 0 {
																																											iTemp166 = i32::wrapping_add(self.iRec13[0], 6929125);
																																											iTemp167 = i32::wrapping_mul(1664525, iTemp166 ^ (iTemp166 >> 15));
																																											fTemp168 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp167 ^ (iTemp167 >> 13))) as F32;
																																											fTemp169 = fTemp168 + 1.5;
																																											fTemp170 = 0.5 - fTemp168;
																																											fTemp171 = AlternativesDsp_faustpower2_f(fTemp170) / AlternativesDsp_faustpower2_f(fTemp169);
																																										}
																																											if iSlow24 != 0 {
																																												self.fRec42[0] = if iTemp149 != 0 {-(2.0 * (fTemp170 * (15.0 - 4.0 * fTemp171) * fTemp165 / (fTemp169 * (15.0 - 9.0 * fTemp171))))} else {self.fRec42[1] + self.fConst6 * (if iTemp150 != 0 {fTemp27} else {1.0} / F32::max(self.fRec44[0], 0.005))};
																																												self.fRec43[0] = fTemp154;
																																												fTemp172 = F32::min(1.0, F32::max(0.0, self.fRec42[0]));
																																												fTemp173 = (iTemp150) as F32;
																																												iTemp174 = (fTemp173 != 0.0) as i32;
																																											}
																																												if iSlow24 & iTemp174 != 0 {
																																													fTemp175 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec43[0])), std::f32::consts::E) * fTemp157;
																																													fTemp176 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec43[0], std::f32::consts::E) + fTemp175) + -8.228819)));
																																													fTemp177 = F32::floor(fTemp161 - fTemp176);
																																													iTemp178 = (fTemp177) as i32;
																																													fTemp179 = fTemp161 - (fTemp176 + fTemp177);
																																													fTemp180 = fTemp173 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp179 * (fTemp179 * (fTemp179 * (fTemp179 * (0.0013333 * fTemp179 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp178))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp178))) as F32));
																																												}
																																													if iSlow24 != 0 {
																																														self.fRec45[0] = if iTemp150 != 0 {fTemp180} else {self.fRec45[1]};
																																														fTemp181 = F32::floor(41.666668 * self.fRec45[0] * fTemp172);
																																														self.fVec3[0] = fTemp181;
																																														fTemp182 = ((fTemp181 != self.fVec3[1]) as i32) as u32 as F32;
																																														iTemp183 = (fTemp182 != 0.0) as i32;
																																													}
																																														if iSlow24 & iTemp183 != 0 {
																																															iTemp184 = i32::wrapping_add(self.iRec13[0], 6960801);
																																															iTemp185 = i32::wrapping_mul(1664525, iTemp184 ^ (iTemp184 >> 15));
																																															fTemp186 = 4.656613e-10 * fTemp182 * (i32::wrapping_mul(1013904223, iTemp185 ^ (iTemp185 >> 13))) as F32;
																																														}
																																															if iSlow24 != 0 {
																																																fTemp187 = ((self.fRec42[0] >= 0.0) as i32) as u32 as F32;
																																																self.fRec41[0] = fTemp187 * (self.fRec41[1] + 0.35 * fTemp182 * fTemp186);
																																																self.fRec40[0] = self.fConst7 * self.fRec41[0] + self.fConst4 * self.fRec40[1];
																																															}
																																																if iSlow24 & iTemp174 != 0 {
																																																	iTemp188 = i32::wrapping_add(self.iRec13[0], 6944963);
																																																	iTemp189 = i32::wrapping_mul(1664525, iTemp188 ^ (iTemp188 >> 15));
																																																	fTemp190 = 0.5 * fTemp173 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp189 ^ (iTemp189 >> 13))) as F32 + 0.5 + 1.0);
																																																}
																																																	if iSlow24 != 0 {
																																																		self.fRec46[0] = if iTemp150 != 0 {fTemp190} else {self.fRec46[1]};
																																																		fTemp191 = F32::min(1.0, F32::max(-1.0, self.fRec46[0] + self.fRec40[0] * fTemp7));
																																																		fTemp192 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp172) * AlternativesDsp_faustpower2_f(1.0 - fTemp172) * fTemp191 * if (fTemp191 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																		fTemp193 = F32::floor(fTemp192);
																																																		iTemp194 = (fTemp193) as i32;
																																																		fTemp195 = fTemp192 - fTemp193;
																																																		fTemp196 = (fTemp195 * (fTemp195 * (fTemp195 * (fTemp195 * (0.0013333 * fTemp195 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp194))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp194))) as F32 + -1.0;
																																																	}
																																																		if iSlow24 & iTemp183 != 0 {
																																																			iTemp197 = i32::wrapping_add(self.iRec13[0], 6952882);
																																																			iTemp198 = i32::wrapping_mul(1664525, iTemp197 ^ (iTemp197 >> 15));
																																																			fTemp199 = 4.656613e-10 * fTemp182 * (i32::wrapping_mul(1013904223, iTemp198 ^ (iTemp198 >> 13))) as F32;
																																																		}
																																																			if iSlow24 != 0 {
																																																				self.fRec51[0] = fTemp187 * (self.fRec51[1] + 0.35 * fTemp182 * fTemp199);
																																																				self.fRec50[0] = self.fConst7 * self.fRec51[0] + self.fConst4 * self.fRec50[1];
																																																				fTemp200 = self.fRec50[0] * fTemp7 * fTemp65;
																																																				fTemp201 = F32::floor(fTemp200);
																																																				iTemp202 = (fTemp201) as i32;
																																																				fTemp203 = fTemp200 - fTemp201;
																																																			}
																																																				if iSlow24 & iTemp174 != 0 {
																																																					fTemp204 = 1.442695 * fTemp175;
																																																					fTemp205 = F32::floor(fTemp204);
																																																					iTemp206 = (fTemp205) as i32;
																																																					fTemp207 = fTemp204 - fTemp205;
																																																					fTemp208 = F32::round((fTemp207 * (fTemp207 * (fTemp207 * (fTemp207 * (0.0013333 * fTemp207 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp206))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp206))) as F32) * fTemp173;
																																																				}
																																																					if iSlow24 != 0 {
																																																						self.fRec52[0] = if iTemp150 != 0 {fTemp208} else {self.fRec52[1]};
																																																						fTemp209 = F32::min(self.fConst5, F32::max(2e+01, self.fRec43[0] * self.fRec52[0] * (fTemp203 * (fTemp203 * (fTemp203 * (fTemp203 * (0.0013333 * fTemp203 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp202))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp202))) as F32));
																																																						fTemp210 = AlternativesDsp_faustpower2_f(fTemp209);
																																																						fTemp211 = self.fConst10 * fTemp210 * (self.fConst9 * fTemp210 + -444.1322) + 105.0;
																																																						fTemp212 = fTemp209 * (105.0 - self.fConst11 * fTemp210);
																																																						fTemp213 = self.fConst12 * (fTemp212 * (fTemp82 + self.fConst12 * (fTemp212 / fTemp211)) / fTemp211) + 1.0;
																																																					}
																																																						if iSlow27 != 0 {
																																																							iTemp214 = ((self.fRec55[1] >= 1.0) as i32) | iTemp5;
																																																							iTemp215 = (self.fRec55[1] < 0.0) as i32;
																																																							iTemp216 = i32::wrapping_add(self.iRec13[0], 6216415);
																																																							iTemp217 = i32::wrapping_mul(1664525, iTemp216 ^ (iTemp216 >> 15));
																																																							iTemp218 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp217 ^ (iTemp217 >> 13))) as F32 + 0.5))))) as i32;
																																																							fTemp219 = if iTemp215 | iTemp5 != 0 {if (iTemp218 >= 4) as i32 != 0 {if (iTemp218 >= 6) as i32 != 0 {if (iTemp218 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp218 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp218 >= 2) as i32 != 0 {if (iTemp218 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp218 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec56[1]};
																																																							iTemp220 = i32::wrapping_add(self.iRec13[0], 6145144);
																																																							iTemp221 = i32::wrapping_mul(1664525, iTemp220 ^ (iTemp220 >> 15));
																																																							fTemp222 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp221 ^ (iTemp221 >> 13))) as F32 + 0.5;
																																																							fTemp223 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp219, std::f32::consts::E) + fTemp222 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp219)), std::f32::consts::E)) + -8.228819)));
																																																							iTemp224 = i32::wrapping_add(self.iRec13[0], 6153063);
																																																							iTemp225 = i32::wrapping_mul(1664525, iTemp224 ^ (iTemp224 >> 15));
																																																							fTemp226 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp225 ^ (iTemp225 >> 13))) as F32;
																																																							fTemp227 = F32::floor(fTemp226 - fTemp223);
																																																							iTemp228 = (fTemp227) as i32;
																																																							fTemp229 = fTemp226 - (fTemp223 + fTemp227);
																																																							self.fRec57[0] = if iTemp215 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp229 * (fTemp229 * (fTemp229 * (fTemp229 * (0.0013333 * fTemp229 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp228))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp228))) as F32))} else {self.fRec57[1]};
																																																							fTemp230 = (iTemp214) as F32;
																																																						}
																																																							if iSlow27 & ((fTemp230 != 0.0) as i32) != 0 {
																																																								iTemp231 = i32::wrapping_add(self.iRec13[0], 6160982);
																																																								iTemp232 = i32::wrapping_mul(1664525, iTemp231 ^ (iTemp231 >> 15));
																																																								fTemp233 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp232 ^ (iTemp232 >> 13))) as F32;
																																																								fTemp234 = fTemp233 + 1.5;
																																																								fTemp235 = 0.5 - fTemp233;
																																																								fTemp236 = AlternativesDsp_faustpower2_f(fTemp235) / AlternativesDsp_faustpower2_f(fTemp234);
																																																							}
																																																								if iSlow27 != 0 {
																																																									self.fRec55[0] = if iTemp214 != 0 {-(2.0 * (fTemp235 * (15.0 - 4.0 * fTemp236) * fTemp230 / (fTemp234 * (15.0 - 9.0 * fTemp236))))} else {self.fRec55[1] + self.fConst6 * (if iTemp215 != 0 {fTemp27} else {1.0} / F32::max(self.fRec57[0], 0.005))};
																																																									self.fRec56[0] = fTemp219;
																																																									fTemp237 = F32::min(1.0, F32::max(0.0, self.fRec55[0]));
																																																									fTemp238 = (iTemp215) as F32;
																																																									iTemp239 = (fTemp238 != 0.0) as i32;
																																																								}
																																																									if iSlow27 & iTemp239 != 0 {
																																																										fTemp240 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec56[0])), std::f32::consts::E) * fTemp222;
																																																										fTemp241 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec56[0], std::f32::consts::E) + fTemp240) + -8.228819)));
																																																										fTemp242 = F32::floor(fTemp226 - fTemp241);
																																																										iTemp243 = (fTemp242) as i32;
																																																										fTemp244 = fTemp226 - (fTemp241 + fTemp242);
																																																										fTemp245 = fTemp238 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp244 * (fTemp244 * (fTemp244 * (fTemp244 * (0.0013333 * fTemp244 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp243))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp243))) as F32));
																																																									}
																																																										if iSlow27 != 0 {
																																																											self.fRec58[0] = if iTemp215 != 0 {fTemp245} else {self.fRec58[1]};
																																																											fTemp246 = F32::floor(41.666668 * self.fRec58[0] * fTemp237);
																																																											self.fVec4[0] = fTemp246;
																																																											fTemp247 = ((fTemp246 != self.fVec4[1]) as i32) as u32 as F32;
																																																											iTemp248 = (fTemp247 != 0.0) as i32;
																																																										}
																																																											if iSlow27 & iTemp248 != 0 {
																																																												iTemp249 = i32::wrapping_add(self.iRec13[0], 6192658);
																																																												iTemp250 = i32::wrapping_mul(1664525, iTemp249 ^ (iTemp249 >> 15));
																																																												fTemp251 = 4.656613e-10 * fTemp247 * (i32::wrapping_mul(1013904223, iTemp250 ^ (iTemp250 >> 13))) as F32;
																																																											}
																																																												if iSlow27 != 0 {
																																																													fTemp252 = ((self.fRec55[0] >= 0.0) as i32) as u32 as F32;
																																																													self.fRec54[0] = fTemp252 * (self.fRec54[1] + 0.35 * fTemp247 * fTemp251);
																																																													self.fRec53[0] = self.fConst7 * self.fRec54[0] + self.fConst4 * self.fRec53[1];
																																																												}
																																																													if iSlow27 & iTemp239 != 0 {
																																																														iTemp253 = i32::wrapping_add(self.iRec13[0], 6176820);
																																																														iTemp254 = i32::wrapping_mul(1664525, iTemp253 ^ (iTemp253 >> 15));
																																																														fTemp255 = 0.5 * fTemp238 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp254 ^ (iTemp254 >> 13))) as F32 + 0.5 + 1.0);
																																																													}
																																																														if iSlow27 != 0 {
																																																															self.fRec59[0] = if iTemp215 != 0 {fTemp255} else {self.fRec59[1]};
																																																															fTemp256 = F32::min(1.0, F32::max(-1.0, self.fRec59[0] + self.fRec53[0] * fTemp7));
																																																															fTemp257 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp237) * AlternativesDsp_faustpower2_f(1.0 - fTemp237) * fTemp256 * if (fTemp256 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																															fTemp258 = F32::floor(fTemp257);
																																																															iTemp259 = (fTemp258) as i32;
																																																															fTemp260 = fTemp257 - fTemp258;
																																																															fTemp261 = (fTemp260 * (fTemp260 * (fTemp260 * (fTemp260 * (0.0013333 * fTemp260 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp259))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp259))) as F32 + -1.0;
																																																														}
																																																															if iSlow27 & iTemp248 != 0 {
																																																																iTemp262 = i32::wrapping_add(self.iRec13[0], 6184739);
																																																																iTemp263 = i32::wrapping_mul(1664525, iTemp262 ^ (iTemp262 >> 15));
																																																																fTemp264 = 4.656613e-10 * fTemp247 * (i32::wrapping_mul(1013904223, iTemp263 ^ (iTemp263 >> 13))) as F32;
																																																															}
																																																																if iSlow27 != 0 {
																																																																	self.fRec64[0] = fTemp252 * (self.fRec64[1] + 0.35 * fTemp247 * fTemp264);
																																																																	self.fRec63[0] = self.fConst7 * self.fRec64[0] + self.fConst4 * self.fRec63[1];
																																																																	fTemp265 = self.fRec63[0] * fTemp7 * fTemp65;
																																																																	fTemp266 = F32::floor(fTemp265);
																																																																	iTemp267 = (fTemp266) as i32;
																																																																	fTemp268 = fTemp265 - fTemp266;
																																																																}
																																																																	if iSlow27 & iTemp239 != 0 {
																																																																		fTemp269 = 1.442695 * fTemp240;
																																																																		fTemp270 = F32::floor(fTemp269);
																																																																		iTemp271 = (fTemp270) as i32;
																																																																		fTemp272 = fTemp269 - fTemp270;
																																																																		fTemp273 = F32::round((fTemp272 * (fTemp272 * (fTemp272 * (fTemp272 * (0.0013333 * fTemp272 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp271))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp271))) as F32) * fTemp238;
																																																																	}
																																																																		if iSlow27 != 0 {
																																																																			self.fRec65[0] = if iTemp215 != 0 {fTemp273} else {self.fRec65[1]};
																																																																			fTemp274 = F32::min(self.fConst5, F32::max(2e+01, self.fRec56[0] * self.fRec65[0] * (fTemp268 * (fTemp268 * (fTemp268 * (fTemp268 * (0.0013333 * fTemp268 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp267))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp267))) as F32));
																																																																			fTemp275 = AlternativesDsp_faustpower2_f(fTemp274);
																																																																			fTemp276 = self.fConst10 * fTemp275 * (self.fConst9 * fTemp275 + -444.1322) + 105.0;
																																																																			fTemp277 = fTemp274 * (105.0 - self.fConst11 * fTemp275);
																																																																			fTemp278 = self.fConst12 * (fTemp277 * (fTemp82 + self.fConst12 * (fTemp277 / fTemp276)) / fTemp276) + 1.0;
																																																																		}
																																																																			if iSlow30 != 0 {
																																																																				iTemp279 = ((self.fRec68[1] >= 1.0) as i32) | iTemp5;
																																																																				iTemp280 = (self.fRec68[1] < 0.0) as i32;
																																																																				iTemp281 = i32::wrapping_add(self.iRec13[0], 5448272);
																																																																				iTemp282 = i32::wrapping_mul(1664525, iTemp281 ^ (iTemp281 >> 15));
																																																																				iTemp283 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp282 ^ (iTemp282 >> 13))) as F32 + 0.5))))) as i32;
																																																																				fTemp284 = if iTemp280 | iTemp5 != 0 {if (iTemp283 >= 4) as i32 != 0 {if (iTemp283 >= 6) as i32 != 0 {if (iTemp283 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp283 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp283 >= 2) as i32 != 0 {if (iTemp283 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp283 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec69[1]};
																																																																				iTemp285 = i32::wrapping_add(self.iRec13[0], 5377001);
																																																																				iTemp286 = i32::wrapping_mul(1664525, iTemp285 ^ (iTemp285 >> 15));
																																																																				fTemp287 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp286 ^ (iTemp286 >> 13))) as F32 + 0.5;
																																																																				fTemp288 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp284, std::f32::consts::E) + fTemp287 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp284)), std::f32::consts::E)) + -8.228819)));
																																																																				iTemp289 = i32::wrapping_add(self.iRec13[0], 5384920);
																																																																				iTemp290 = i32::wrapping_mul(1664525, iTemp289 ^ (iTemp289 >> 15));
																																																																				fTemp291 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp290 ^ (iTemp290 >> 13))) as F32;
																																																																				fTemp292 = F32::floor(fTemp291 - fTemp288);
																																																																				iTemp293 = (fTemp292) as i32;
																																																																				fTemp294 = fTemp291 - (fTemp288 + fTemp292);
																																																																				self.fRec70[0] = if iTemp280 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp294 * (fTemp294 * (fTemp294 * (fTemp294 * (0.0013333 * fTemp294 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp293))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp293))) as F32))} else {self.fRec70[1]};
																																																																				fTemp295 = (iTemp279) as F32;
																																																																			}
																																																																				if iSlow30 & ((fTemp295 != 0.0) as i32) != 0 {
																																																																					iTemp296 = i32::wrapping_add(self.iRec13[0], 5392839);
																																																																					iTemp297 = i32::wrapping_mul(1664525, iTemp296 ^ (iTemp296 >> 15));
																																																																					fTemp298 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp297 ^ (iTemp297 >> 13))) as F32;
																																																																					fTemp299 = fTemp298 + 1.5;
																																																																					fTemp300 = 0.5 - fTemp298;
																																																																					fTemp301 = AlternativesDsp_faustpower2_f(fTemp300) / AlternativesDsp_faustpower2_f(fTemp299);
																																																																				}
																																																																					if iSlow30 != 0 {
																																																																						self.fRec68[0] = if iTemp279 != 0 {-(2.0 * (fTemp300 * (15.0 - 4.0 * fTemp301) * fTemp295 / (fTemp299 * (15.0 - 9.0 * fTemp301))))} else {self.fRec68[1] + self.fConst6 * (if iTemp280 != 0 {fTemp27} else {1.0} / F32::max(self.fRec70[0], 0.005))};
																																																																						self.fRec69[0] = fTemp284;
																																																																						fTemp302 = F32::min(1.0, F32::max(0.0, self.fRec68[0]));
																																																																						fTemp303 = (iTemp280) as F32;
																																																																						iTemp304 = (fTemp303 != 0.0) as i32;
																																																																					}
																																																																						if iSlow30 & iTemp304 != 0 {
																																																																							fTemp305 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec69[0])), std::f32::consts::E) * fTemp287;
																																																																							fTemp306 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec69[0], std::f32::consts::E) + fTemp305) + -8.228819)));
																																																																							fTemp307 = F32::floor(fTemp291 - fTemp306);
																																																																							iTemp308 = (fTemp307) as i32;
																																																																							fTemp309 = fTemp291 - (fTemp306 + fTemp307);
																																																																							fTemp310 = fTemp303 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp309 * (fTemp309 * (fTemp309 * (fTemp309 * (0.0013333 * fTemp309 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp308))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp308))) as F32));
																																																																						}
																																																																							if iSlow30 != 0 {
																																																																								self.fRec71[0] = if iTemp280 != 0 {fTemp310} else {self.fRec71[1]};
																																																																								fTemp311 = F32::floor(41.666668 * self.fRec71[0] * fTemp302);
																																																																								self.fVec5[0] = fTemp311;
																																																																								fTemp312 = ((fTemp311 != self.fVec5[1]) as i32) as u32 as F32;
																																																																								iTemp313 = (fTemp312 != 0.0) as i32;
																																																																							}
																																																																								if iSlow30 & iTemp313 != 0 {
																																																																									iTemp314 = i32::wrapping_add(self.iRec13[0], 5424515);
																																																																									iTemp315 = i32::wrapping_mul(1664525, iTemp314 ^ (iTemp314 >> 15));
																																																																									fTemp316 = 4.656613e-10 * fTemp312 * (i32::wrapping_mul(1013904223, iTemp315 ^ (iTemp315 >> 13))) as F32;
																																																																								}
																																																																									if iSlow30 != 0 {
																																																																										fTemp317 = ((self.fRec68[0] >= 0.0) as i32) as u32 as F32;
																																																																										self.fRec67[0] = fTemp317 * (self.fRec67[1] + 0.35 * fTemp312 * fTemp316);
																																																																										self.fRec66[0] = self.fConst7 * self.fRec67[0] + self.fConst4 * self.fRec66[1];
																																																																									}
																																																																										if iSlow30 & iTemp304 != 0 {
																																																																											iTemp318 = i32::wrapping_add(self.iRec13[0], 5408677);
																																																																											iTemp319 = i32::wrapping_mul(1664525, iTemp318 ^ (iTemp318 >> 15));
																																																																											fTemp320 = 0.5 * fTemp303 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp319 ^ (iTemp319 >> 13))) as F32 + 0.5 + 1.0);
																																																																										}
																																																																											if iSlow30 != 0 {
																																																																												self.fRec72[0] = if iTemp280 != 0 {fTemp320} else {self.fRec72[1]};
																																																																												fTemp321 = F32::min(1.0, F32::max(-1.0, self.fRec72[0] + self.fRec66[0] * fTemp7));
																																																																												fTemp322 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp302) * AlternativesDsp_faustpower2_f(1.0 - fTemp302) * fTemp321 * if (fTemp321 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																																												fTemp323 = F32::floor(fTemp322);
																																																																												iTemp324 = (fTemp323) as i32;
																																																																												fTemp325 = fTemp322 - fTemp323;
																																																																												fTemp326 = (fTemp325 * (fTemp325 * (fTemp325 * (fTemp325 * (0.0013333 * fTemp325 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp324))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp324))) as F32 + -1.0;
																																																																											}
																																																																												if iSlow30 & iTemp313 != 0 {
																																																																													iTemp327 = i32::wrapping_add(self.iRec13[0], 5416596);
																																																																													iTemp328 = i32::wrapping_mul(1664525, iTemp327 ^ (iTemp327 >> 15));
																																																																													fTemp329 = 4.656613e-10 * fTemp312 * (i32::wrapping_mul(1013904223, iTemp328 ^ (iTemp328 >> 13))) as F32;
																																																																												}
																																																																													if iSlow30 != 0 {
																																																																														self.fRec77[0] = fTemp317 * (self.fRec77[1] + 0.35 * fTemp312 * fTemp329);
																																																																														self.fRec76[0] = self.fConst7 * self.fRec77[0] + self.fConst4 * self.fRec76[1];
																																																																														fTemp330 = self.fRec76[0] * fTemp7 * fTemp65;
																																																																														fTemp331 = F32::floor(fTemp330);
																																																																														iTemp332 = (fTemp331) as i32;
																																																																														fTemp333 = fTemp330 - fTemp331;
																																																																													}
																																																																														if iSlow30 & iTemp304 != 0 {
																																																																															fTemp334 = 1.442695 * fTemp305;
																																																																															fTemp335 = F32::floor(fTemp334);
																																																																															iTemp336 = (fTemp335) as i32;
																																																																															fTemp337 = fTemp334 - fTemp335;
																																																																															fTemp338 = F32::round((fTemp337 * (fTemp337 * (fTemp337 * (fTemp337 * (0.0013333 * fTemp337 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp336))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp336))) as F32) * fTemp303;
																																																																														}
																																																																															if iSlow30 != 0 {
																																																																																self.fRec78[0] = if iTemp280 != 0 {fTemp338} else {self.fRec78[1]};
																																																																																fTemp339 = F32::min(self.fConst5, F32::max(2e+01, self.fRec69[0] * self.fRec78[0] * (fTemp333 * (fTemp333 * (fTemp333 * (fTemp333 * (0.0013333 * fTemp333 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp332))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp332))) as F32));
																																																																																fTemp340 = AlternativesDsp_faustpower2_f(fTemp339);
																																																																																fTemp341 = self.fConst10 * fTemp340 * (self.fConst9 * fTemp340 + -444.1322) + 105.0;
																																																																																fTemp342 = fTemp339 * (105.0 - self.fConst11 * fTemp340);
																																																																																fTemp343 = self.fConst12 * (fTemp342 * (fTemp82 + self.fConst12 * (fTemp342 / fTemp341)) / fTemp341) + 1.0;
																																																																															}
																																																																																if iSlow33 != 0 {
																																																																																	iTemp344 = ((self.fRec81[1] >= 1.0) as i32) | iTemp5;
																																																																																	iTemp345 = (self.fRec81[1] < 0.0) as i32;
																																																																																	iTemp346 = i32::wrapping_add(self.iRec13[0], 4680129);
																																																																																	iTemp347 = i32::wrapping_mul(1664525, iTemp346 ^ (iTemp346 >> 15));
																																																																																	iTemp348 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp347 ^ (iTemp347 >> 13))) as F32 + 0.5))))) as i32;
																																																																																	fTemp349 = if iTemp345 | iTemp5 != 0 {if (iTemp348 >= 4) as i32 != 0 {if (iTemp348 >= 6) as i32 != 0 {if (iTemp348 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp348 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp348 >= 2) as i32 != 0 {if (iTemp348 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp348 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec82[1]};
																																																																																	iTemp350 = i32::wrapping_add(self.iRec13[0], 4608858);
																																																																																	iTemp351 = i32::wrapping_mul(1664525, iTemp350 ^ (iTemp350 >> 15));
																																																																																	fTemp352 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp351 ^ (iTemp351 >> 13))) as F32 + 0.5;
																																																																																	fTemp353 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp349, std::f32::consts::E) + fTemp352 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp349)), std::f32::consts::E)) + -8.228819)));
																																																																																	iTemp354 = i32::wrapping_add(self.iRec13[0], 4616777);
																																																																																	iTemp355 = i32::wrapping_mul(1664525, iTemp354 ^ (iTemp354 >> 15));
																																																																																	fTemp356 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp355 ^ (iTemp355 >> 13))) as F32;
																																																																																	fTemp357 = F32::floor(fTemp356 - fTemp353);
																																																																																	iTemp358 = (fTemp357) as i32;
																																																																																	fTemp359 = fTemp356 - (fTemp353 + fTemp357);
																																																																																	self.fRec83[0] = if iTemp345 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp359 * (fTemp359 * (fTemp359 * (fTemp359 * (0.0013333 * fTemp359 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp358))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp358))) as F32))} else {self.fRec83[1]};
																																																																																	fTemp360 = (iTemp344) as F32;
																																																																																}
																																																																																	if iSlow33 & ((fTemp360 != 0.0) as i32) != 0 {
																																																																																		iTemp361 = i32::wrapping_add(self.iRec13[0], 4624696);
																																																																																		iTemp362 = i32::wrapping_mul(1664525, iTemp361 ^ (iTemp361 >> 15));
																																																																																		fTemp363 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp362 ^ (iTemp362 >> 13))) as F32;
																																																																																		fTemp364 = fTemp363 + 1.5;
																																																																																		fTemp365 = 0.5 - fTemp363;
																																																																																		fTemp366 = AlternativesDsp_faustpower2_f(fTemp365) / AlternativesDsp_faustpower2_f(fTemp364);
																																																																																	}
																																																																																		if iSlow33 != 0 {
																																																																																			self.fRec81[0] = if iTemp344 != 0 {-(2.0 * (fTemp365 * (15.0 - 4.0 * fTemp366) * fTemp360 / (fTemp364 * (15.0 - 9.0 * fTemp366))))} else {self.fRec81[1] + self.fConst6 * (if iTemp345 != 0 {fTemp27} else {1.0} / F32::max(self.fRec83[0], 0.005))};
																																																																																			self.fRec82[0] = fTemp349;
																																																																																			fTemp367 = F32::min(1.0, F32::max(0.0, self.fRec81[0]));
																																																																																			fTemp368 = (iTemp345) as F32;
																																																																																			iTemp369 = (fTemp368 != 0.0) as i32;
																																																																																		}
																																																																																			if iSlow33 & iTemp369 != 0 {
																																																																																				fTemp370 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec82[0])), std::f32::consts::E) * fTemp352;
																																																																																				fTemp371 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec82[0], std::f32::consts::E) + fTemp370) + -8.228819)));
																																																																																				fTemp372 = F32::floor(fTemp356 - fTemp371);
																																																																																				iTemp373 = (fTemp372) as i32;
																																																																																				fTemp374 = fTemp356 - (fTemp371 + fTemp372);
																																																																																				fTemp375 = fTemp368 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp374 * (fTemp374 * (fTemp374 * (fTemp374 * (0.0013333 * fTemp374 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp373))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp373))) as F32));
																																																																																			}
																																																																																				if iSlow33 != 0 {
																																																																																					self.fRec84[0] = if iTemp345 != 0 {fTemp375} else {self.fRec84[1]};
																																																																																					fTemp376 = F32::floor(41.666668 * self.fRec84[0] * fTemp367);
																																																																																					self.fVec6[0] = fTemp376;
																																																																																					fTemp377 = ((fTemp376 != self.fVec6[1]) as i32) as u32 as F32;
																																																																																					iTemp378 = (fTemp377 != 0.0) as i32;
																																																																																				}
																																																																																					if iSlow33 & iTemp378 != 0 {
																																																																																						iTemp379 = i32::wrapping_add(self.iRec13[0], 4656372);
																																																																																						iTemp380 = i32::wrapping_mul(1664525, iTemp379 ^ (iTemp379 >> 15));
																																																																																						fTemp381 = 4.656613e-10 * fTemp377 * (i32::wrapping_mul(1013904223, iTemp380 ^ (iTemp380 >> 13))) as F32;
																																																																																					}
																																																																																						if iSlow33 != 0 {
																																																																																							fTemp382 = ((self.fRec81[0] >= 0.0) as i32) as u32 as F32;
																																																																																							self.fRec80[0] = fTemp382 * (self.fRec80[1] + 0.35 * fTemp377 * fTemp381);
																																																																																							self.fRec79[0] = self.fConst7 * self.fRec80[0] + self.fConst4 * self.fRec79[1];
																																																																																						}
																																																																																							if iSlow33 & iTemp369 != 0 {
																																																																																								iTemp383 = i32::wrapping_add(self.iRec13[0], 4640534);
																																																																																								iTemp384 = i32::wrapping_mul(1664525, iTemp383 ^ (iTemp383 >> 15));
																																																																																								fTemp385 = 0.5 * fTemp368 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp384 ^ (iTemp384 >> 13))) as F32 + 0.5 + 1.0);
																																																																																							}
																																																																																								if iSlow33 != 0 {
																																																																																									self.fRec85[0] = if iTemp345 != 0 {fTemp385} else {self.fRec85[1]};
																																																																																									fTemp386 = F32::min(1.0, F32::max(-1.0, self.fRec85[0] + self.fRec79[0] * fTemp7));
																																																																																									fTemp387 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp367) * AlternativesDsp_faustpower2_f(1.0 - fTemp367) * fTemp386 * if (fTemp386 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																																																									fTemp388 = F32::floor(fTemp387);
																																																																																									iTemp389 = (fTemp388) as i32;
																																																																																									fTemp390 = fTemp387 - fTemp388;
																																																																																									fTemp391 = (fTemp390 * (fTemp390 * (fTemp390 * (fTemp390 * (0.0013333 * fTemp390 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp389))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp389))) as F32 + -1.0;
																																																																																								}
																																																																																									if iSlow33 & iTemp378 != 0 {
																																																																																										iTemp392 = i32::wrapping_add(self.iRec13[0], 4648453);
																																																																																										iTemp393 = i32::wrapping_mul(1664525, iTemp392 ^ (iTemp392 >> 15));
																																																																																										fTemp394 = 4.656613e-10 * fTemp377 * (i32::wrapping_mul(1013904223, iTemp393 ^ (iTemp393 >> 13))) as F32;
																																																																																									}
																																																																																										if iSlow33 != 0 {
																																																																																											self.fRec90[0] = fTemp382 * (self.fRec90[1] + 0.35 * fTemp377 * fTemp394);
																																																																																											self.fRec89[0] = self.fConst7 * self.fRec90[0] + self.fConst4 * self.fRec89[1];
																																																																																											fTemp395 = self.fRec89[0] * fTemp7 * fTemp65;
																																																																																											fTemp396 = F32::floor(fTemp395);
																																																																																											iTemp397 = (fTemp396) as i32;
																																																																																											fTemp398 = fTemp395 - fTemp396;
																																																																																										}
																																																																																											if iSlow33 & iTemp369 != 0 {
																																																																																												fTemp399 = 1.442695 * fTemp370;
																																																																																												fTemp400 = F32::floor(fTemp399);
																																																																																												iTemp401 = (fTemp400) as i32;
																																																																																												fTemp402 = fTemp399 - fTemp400;
																																																																																												fTemp403 = F32::round((fTemp402 * (fTemp402 * (fTemp402 * (fTemp402 * (0.0013333 * fTemp402 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp401))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp401))) as F32) * fTemp368;
																																																																																											}
																																																																																												if iSlow33 != 0 {
																																																																																													self.fRec91[0] = if iTemp345 != 0 {fTemp403} else {self.fRec91[1]};
																																																																																													fTemp404 = F32::min(self.fConst5, F32::max(2e+01, self.fRec82[0] * self.fRec91[0] * (fTemp398 * (fTemp398 * (fTemp398 * (fTemp398 * (0.0013333 * fTemp398 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp397))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp397))) as F32));
																																																																																													fTemp405 = AlternativesDsp_faustpower2_f(fTemp404);
																																																																																													fTemp406 = self.fConst10 * fTemp405 * (self.fConst9 * fTemp405 + -444.1322) + 105.0;
																																																																																													fTemp407 = fTemp404 * (105.0 - self.fConst11 * fTemp405);
																																																																																													fTemp408 = self.fConst12 * (fTemp407 * (fTemp82 + self.fConst12 * (fTemp407 / fTemp406)) / fTemp406) + 1.0;
																																																																																												}
																																																																																													if iSlow36 != 0 {
																																																																																														iTemp409 = ((self.fRec94[1] >= 1.0) as i32) | iTemp5;
																																																																																														iTemp410 = (self.fRec94[1] < 0.0) as i32;
																																																																																														iTemp411 = i32::wrapping_add(self.iRec13[0], 3911986);
																																																																																														iTemp412 = i32::wrapping_mul(1664525, iTemp411 ^ (iTemp411 >> 15));
																																																																																														iTemp413 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp412 ^ (iTemp412 >> 13))) as F32 + 0.5))))) as i32;
																																																																																														fTemp414 = if iTemp410 | iTemp5 != 0 {if (iTemp413 >= 4) as i32 != 0 {if (iTemp413 >= 6) as i32 != 0 {if (iTemp413 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp413 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp413 >= 2) as i32 != 0 {if (iTemp413 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp413 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec95[1]};
																																																																																														iTemp415 = i32::wrapping_add(self.iRec13[0], 3840715);
																																																																																														iTemp416 = i32::wrapping_mul(1664525, iTemp415 ^ (iTemp415 >> 15));
																																																																																														fTemp417 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp416 ^ (iTemp416 >> 13))) as F32 + 0.5;
																																																																																														fTemp418 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp414, std::f32::consts::E) + fTemp417 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp414)), std::f32::consts::E)) + -8.228819)));
																																																																																														iTemp419 = i32::wrapping_add(self.iRec13[0], 3848634);
																																																																																														iTemp420 = i32::wrapping_mul(1664525, iTemp419 ^ (iTemp419 >> 15));
																																																																																														fTemp421 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp420 ^ (iTemp420 >> 13))) as F32;
																																																																																														fTemp422 = F32::floor(fTemp421 - fTemp418);
																																																																																														iTemp423 = (fTemp422) as i32;
																																																																																														fTemp424 = fTemp421 - (fTemp418 + fTemp422);
																																																																																														self.fRec96[0] = if iTemp410 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp424 * (fTemp424 * (fTemp424 * (fTemp424 * (0.0013333 * fTemp424 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp423))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp423))) as F32))} else {self.fRec96[1]};
																																																																																														fTemp425 = (iTemp409) as F32;
																																																																																													}
																																																																																														if iSlow36 & ((fTemp425 != 0.0) as i32) != 0 {
																																																																																															iTemp426 = i32::wrapping_add(self.iRec13[0], 3856553);
																																																																																															iTemp427 = i32::wrapping_mul(1664525, iTemp426 ^ (iTemp426 >> 15));
																																																																																															fTemp428 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp427 ^ (iTemp427 >> 13))) as F32;
																																																																																															fTemp429 = fTemp428 + 1.5;
																																																																																															fTemp430 = 0.5 - fTemp428;
																																																																																															fTemp431 = AlternativesDsp_faustpower2_f(fTemp430) / AlternativesDsp_faustpower2_f(fTemp429);
																																																																																														}
																																																																																															if iSlow36 != 0 {
																																																																																																self.fRec94[0] = if iTemp409 != 0 {-(2.0 * (fTemp430 * (15.0 - 4.0 * fTemp431) * fTemp425 / (fTemp429 * (15.0 - 9.0 * fTemp431))))} else {self.fRec94[1] + self.fConst6 * (if iTemp410 != 0 {fTemp27} else {1.0} / F32::max(self.fRec96[0], 0.005))};
																																																																																																self.fRec95[0] = fTemp414;
																																																																																																fTemp432 = F32::min(1.0, F32::max(0.0, self.fRec94[0]));
																																																																																																fTemp433 = (iTemp410) as F32;
																																																																																																iTemp434 = (fTemp433 != 0.0) as i32;
																																																																																															}
																																																																																																if iSlow36 & iTemp434 != 0 {
																																																																																																	fTemp435 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec95[0])), std::f32::consts::E) * fTemp417;
																																																																																																	fTemp436 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec95[0], std::f32::consts::E) + fTemp435) + -8.228819)));
																																																																																																	fTemp437 = F32::floor(fTemp421 - fTemp436);
																																																																																																	iTemp438 = (fTemp437) as i32;
																																																																																																	fTemp439 = fTemp421 - (fTemp436 + fTemp437);
																																																																																																	fTemp440 = fTemp433 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp439 * (fTemp439 * (fTemp439 * (fTemp439 * (0.0013333 * fTemp439 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp438))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp438))) as F32));
																																																																																																}
																																																																																																	if iSlow36 != 0 {
																																																																																																		self.fRec97[0] = if iTemp410 != 0 {fTemp440} else {self.fRec97[1]};
																																																																																																		fTemp441 = F32::floor(41.666668 * self.fRec97[0] * fTemp432);
																																																																																																		self.fVec7[0] = fTemp441;
																																																																																																		fTemp442 = ((fTemp441 != self.fVec7[1]) as i32) as u32 as F32;
																																																																																																		iTemp443 = (fTemp442 != 0.0) as i32;
																																																																																																	}
																																																																																																		if iSlow36 & iTemp443 != 0 {
																																																																																																			iTemp444 = i32::wrapping_add(self.iRec13[0], 3888229);
																																																																																																			iTemp445 = i32::wrapping_mul(1664525, iTemp444 ^ (iTemp444 >> 15));
																																																																																																			fTemp446 = 4.656613e-10 * fTemp442 * (i32::wrapping_mul(1013904223, iTemp445 ^ (iTemp445 >> 13))) as F32;
																																																																																																		}
																																																																																																			if iSlow36 != 0 {
																																																																																																				fTemp447 = ((self.fRec94[0] >= 0.0) as i32) as u32 as F32;
																																																																																																				self.fRec93[0] = fTemp447 * (self.fRec93[1] + 0.35 * fTemp442 * fTemp446);
																																																																																																				self.fRec92[0] = self.fConst7 * self.fRec93[0] + self.fConst4 * self.fRec92[1];
																																																																																																			}
																																																																																																				if iSlow36 & iTemp434 != 0 {
																																																																																																					iTemp448 = i32::wrapping_add(self.iRec13[0], 3872391);
																																																																																																					iTemp449 = i32::wrapping_mul(1664525, iTemp448 ^ (iTemp448 >> 15));
																																																																																																					fTemp450 = 0.5 * fTemp433 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp449 ^ (iTemp449 >> 13))) as F32 + 0.5 + 1.0);
																																																																																																				}
																																																																																																					if iSlow36 != 0 {
																																																																																																						self.fRec98[0] = if iTemp410 != 0 {fTemp450} else {self.fRec98[1]};
																																																																																																						fTemp451 = F32::min(1.0, F32::max(-1.0, self.fRec98[0] + self.fRec92[0] * fTemp7));
																																																																																																						fTemp452 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp432) * AlternativesDsp_faustpower2_f(1.0 - fTemp432) * fTemp451 * if (fTemp451 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																																																																						fTemp453 = F32::floor(fTemp452);
																																																																																																						iTemp454 = (fTemp453) as i32;
																																																																																																						fTemp455 = fTemp452 - fTemp453;
																																																																																																						fTemp456 = (fTemp455 * (fTemp455 * (fTemp455 * (fTemp455 * (0.0013333 * fTemp455 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp454))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp454))) as F32 + -1.0;
																																																																																																					}
																																																																																																						if iSlow36 & iTemp443 != 0 {
																																																																																																							iTemp457 = i32::wrapping_add(self.iRec13[0], 3880310);
																																																																																																							iTemp458 = i32::wrapping_mul(1664525, iTemp457 ^ (iTemp457 >> 15));
																																																																																																							fTemp459 = 4.656613e-10 * fTemp442 * (i32::wrapping_mul(1013904223, iTemp458 ^ (iTemp458 >> 13))) as F32;
																																																																																																						}
																																																																																																							if iSlow36 != 0 {
																																																																																																								self.fRec103[0] = fTemp447 * (self.fRec103[1] + 0.35 * fTemp442 * fTemp459);
																																																																																																								self.fRec102[0] = self.fConst7 * self.fRec103[0] + self.fConst4 * self.fRec102[1];
																																																																																																								fTemp460 = self.fRec102[0] * fTemp7 * fTemp65;
																																																																																																								fTemp461 = F32::floor(fTemp460);
																																																																																																								iTemp462 = (fTemp461) as i32;
																																																																																																								fTemp463 = fTemp460 - fTemp461;
																																																																																																							}
																																																																																																								if iSlow36 & iTemp434 != 0 {
																																																																																																									fTemp464 = 1.442695 * fTemp435;
																																																																																																									fTemp465 = F32::floor(fTemp464);
																																																																																																									iTemp466 = (fTemp465) as i32;
																																																																																																									fTemp467 = fTemp464 - fTemp465;
																																																																																																									fTemp468 = F32::round((fTemp467 * (fTemp467 * (fTemp467 * (fTemp467 * (0.0013333 * fTemp467 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp466))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp466))) as F32) * fTemp433;
																																																																																																								}
																																																																																																									if iSlow36 != 0 {
																																																																																																										self.fRec104[0] = if iTemp410 != 0 {fTemp468} else {self.fRec104[1]};
																																																																																																										fTemp469 = F32::min(self.fConst5, F32::max(2e+01, self.fRec95[0] * self.fRec104[0] * (fTemp463 * (fTemp463 * (fTemp463 * (fTemp463 * (0.0013333 * fTemp463 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp462))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp462))) as F32));
																																																																																																										fTemp470 = AlternativesDsp_faustpower2_f(fTemp469);
																																																																																																										fTemp471 = self.fConst10 * fTemp470 * (self.fConst9 * fTemp470 + -444.1322) + 105.0;
																																																																																																										fTemp472 = fTemp469 * (105.0 - self.fConst11 * fTemp470);
																																																																																																										fTemp473 = self.fConst12 * (fTemp472 * (fTemp82 + self.fConst12 * (fTemp472 / fTemp471)) / fTemp471) + 1.0;
																																																																																																									}
																																																																																																										if iSlow39 != 0 {
																																																																																																											iTemp474 = ((self.fRec107[1] >= 1.0) as i32) | iTemp5;
																																																																																																											iTemp475 = (self.fRec107[1] < 0.0) as i32;
																																																																																																											iTemp476 = i32::wrapping_add(self.iRec13[0], 3143843);
																																																																																																											iTemp477 = i32::wrapping_mul(1664525, iTemp476 ^ (iTemp476 >> 15));
																																																																																																											iTemp478 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp477 ^ (iTemp477 >> 13))) as F32 + 0.5))))) as i32;
																																																																																																											fTemp479 = if iTemp475 | iTemp5 != 0 {if (iTemp478 >= 4) as i32 != 0 {if (iTemp478 >= 6) as i32 != 0 {if (iTemp478 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp478 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp478 >= 2) as i32 != 0 {if (iTemp478 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp478 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec108[1]};
																																																																																																											iTemp480 = i32::wrapping_add(self.iRec13[0], 3072572);
																																																																																																											iTemp481 = i32::wrapping_mul(1664525, iTemp480 ^ (iTemp480 >> 15));
																																																																																																											fTemp482 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp481 ^ (iTemp481 >> 13))) as F32 + 0.5;
																																																																																																											fTemp483 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp479, std::f32::consts::E) + fTemp482 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp479)), std::f32::consts::E)) + -8.228819)));
																																																																																																											iTemp484 = i32::wrapping_add(self.iRec13[0], 3080491);
																																																																																																											iTemp485 = i32::wrapping_mul(1664525, iTemp484 ^ (iTemp484 >> 15));
																																																																																																											fTemp486 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp485 ^ (iTemp485 >> 13))) as F32;
																																																																																																											fTemp487 = F32::floor(fTemp486 - fTemp483);
																																																																																																											iTemp488 = (fTemp487) as i32;
																																																																																																											fTemp489 = fTemp486 - (fTemp483 + fTemp487);
																																																																																																											self.fRec109[0] = if iTemp475 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp489 * (fTemp489 * (fTemp489 * (fTemp489 * (0.0013333 * fTemp489 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp488))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp488))) as F32))} else {self.fRec109[1]};
																																																																																																											fTemp490 = (iTemp474) as F32;
																																																																																																										}
																																																																																																											if iSlow39 & ((fTemp490 != 0.0) as i32) != 0 {
																																																																																																												iTemp491 = i32::wrapping_add(self.iRec13[0], 3088410);
																																																																																																												iTemp492 = i32::wrapping_mul(1664525, iTemp491 ^ (iTemp491 >> 15));
																																																																																																												fTemp493 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp492 ^ (iTemp492 >> 13))) as F32;
																																																																																																												fTemp494 = fTemp493 + 1.5;
																																																																																																												fTemp495 = 0.5 - fTemp493;
																																																																																																												fTemp496 = AlternativesDsp_faustpower2_f(fTemp495) / AlternativesDsp_faustpower2_f(fTemp494);
																																																																																																											}
																																																																																																												if iSlow39 != 0 {
																																																																																																													self.fRec107[0] = if iTemp474 != 0 {-(2.0 * (fTemp495 * (15.0 - 4.0 * fTemp496) * fTemp490 / (fTemp494 * (15.0 - 9.0 * fTemp496))))} else {self.fRec107[1] + self.fConst6 * (if iTemp475 != 0 {fTemp27} else {1.0} / F32::max(self.fRec109[0], 0.005))};
																																																																																																													self.fRec108[0] = fTemp479;
																																																																																																													fTemp497 = F32::min(1.0, F32::max(0.0, self.fRec107[0]));
																																																																																																													fTemp498 = (iTemp475) as F32;
																																																																																																													iTemp499 = (fTemp498 != 0.0) as i32;
																																																																																																												}
																																																																																																													if iSlow39 & iTemp499 != 0 {
																																																																																																														fTemp500 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec108[0])), std::f32::consts::E) * fTemp482;
																																																																																																														fTemp501 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec108[0], std::f32::consts::E) + fTemp500) + -8.228819)));
																																																																																																														fTemp502 = F32::floor(fTemp486 - fTemp501);
																																																																																																														iTemp503 = (fTemp502) as i32;
																																																																																																														fTemp504 = fTemp486 - (fTemp501 + fTemp502);
																																																																																																														fTemp505 = fTemp498 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp504 * (fTemp504 * (fTemp504 * (fTemp504 * (0.0013333 * fTemp504 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp503))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp503))) as F32));
																																																																																																													}
																																																																																																														if iSlow39 != 0 {
																																																																																																															self.fRec110[0] = if iTemp475 != 0 {fTemp505} else {self.fRec110[1]};
																																																																																																															fTemp506 = F32::floor(41.666668 * self.fRec110[0] * fTemp497);
																																																																																																															self.fVec8[0] = fTemp506;
																																																																																																															fTemp507 = ((fTemp506 != self.fVec8[1]) as i32) as u32 as F32;
																																																																																																															iTemp508 = (fTemp507 != 0.0) as i32;
																																																																																																														}
																																																																																																															if iSlow39 & iTemp508 != 0 {
																																																																																																																iTemp509 = i32::wrapping_add(self.iRec13[0], 3120086);
																																																																																																																iTemp510 = i32::wrapping_mul(1664525, iTemp509 ^ (iTemp509 >> 15));
																																																																																																																fTemp511 = 4.656613e-10 * fTemp507 * (i32::wrapping_mul(1013904223, iTemp510 ^ (iTemp510 >> 13))) as F32;
																																																																																																															}
																																																																																																																if iSlow39 != 0 {
																																																																																																																	fTemp512 = ((self.fRec107[0] >= 0.0) as i32) as u32 as F32;
																																																																																																																	self.fRec106[0] = fTemp512 * (self.fRec106[1] + 0.35 * fTemp507 * fTemp511);
																																																																																																																	self.fRec105[0] = self.fConst7 * self.fRec106[0] + self.fConst4 * self.fRec105[1];
																																																																																																																}
																																																																																																																	if iSlow39 & iTemp499 != 0 {
																																																																																																																		iTemp513 = i32::wrapping_add(self.iRec13[0], 3104248);
																																																																																																																		iTemp514 = i32::wrapping_mul(1664525, iTemp513 ^ (iTemp513 >> 15));
																																																																																																																		fTemp515 = 0.5 * fTemp498 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp514 ^ (iTemp514 >> 13))) as F32 + 0.5 + 1.0);
																																																																																																																	}
																																																																																																																		if iSlow39 != 0 {
																																																																																																																			self.fRec111[0] = if iTemp475 != 0 {fTemp515} else {self.fRec111[1]};
																																																																																																																			fTemp516 = F32::min(1.0, F32::max(-1.0, self.fRec111[0] + self.fRec105[0] * fTemp7));
																																																																																																																			fTemp517 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp497) * AlternativesDsp_faustpower2_f(1.0 - fTemp497) * fTemp516 * if (fTemp516 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																																																																																			fTemp518 = F32::floor(fTemp517);
																																																																																																																			iTemp519 = (fTemp518) as i32;
																																																																																																																			fTemp520 = fTemp517 - fTemp518;
																																																																																																																			fTemp521 = (fTemp520 * (fTemp520 * (fTemp520 * (fTemp520 * (0.0013333 * fTemp520 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp519))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp519))) as F32 + -1.0;
																																																																																																																		}
																																																																																																																			if iSlow39 & iTemp508 != 0 {
																																																																																																																				iTemp522 = i32::wrapping_add(self.iRec13[0], 3112167);
																																																																																																																				iTemp523 = i32::wrapping_mul(1664525, iTemp522 ^ (iTemp522 >> 15));
																																																																																																																				fTemp524 = 4.656613e-10 * fTemp507 * (i32::wrapping_mul(1013904223, iTemp523 ^ (iTemp523 >> 13))) as F32;
																																																																																																																			}
																																																																																																																				if iSlow39 != 0 {
																																																																																																																					self.fRec116[0] = fTemp512 * (self.fRec116[1] + 0.35 * fTemp507 * fTemp524);
																																																																																																																					self.fRec115[0] = self.fConst7 * self.fRec116[0] + self.fConst4 * self.fRec115[1];
																																																																																																																					fTemp525 = self.fRec115[0] * fTemp7 * fTemp65;
																																																																																																																					fTemp526 = F32::floor(fTemp525);
																																																																																																																					iTemp527 = (fTemp526) as i32;
																																																																																																																					fTemp528 = fTemp525 - fTemp526;
																																																																																																																				}
																																																																																																																					if iSlow39 & iTemp499 != 0 {
																																																																																																																						fTemp529 = 1.442695 * fTemp500;
																																																																																																																						fTemp530 = F32::floor(fTemp529);
																																																																																																																						iTemp531 = (fTemp530) as i32;
																																																																																																																						fTemp532 = fTemp529 - fTemp530;
																																																																																																																						fTemp533 = F32::round((fTemp532 * (fTemp532 * (fTemp532 * (fTemp532 * (0.0013333 * fTemp532 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp531))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp531))) as F32) * fTemp498;
																																																																																																																					}
																																																																																																																						if iSlow39 != 0 {
																																																																																																																							self.fRec117[0] = if iTemp475 != 0 {fTemp533} else {self.fRec117[1]};
																																																																																																																							fTemp534 = F32::min(self.fConst5, F32::max(2e+01, self.fRec108[0] * self.fRec117[0] * (fTemp528 * (fTemp528 * (fTemp528 * (fTemp528 * (0.0013333 * fTemp528 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp527))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp527))) as F32));
																																																																																																																							fTemp535 = AlternativesDsp_faustpower2_f(fTemp534);
																																																																																																																							fTemp536 = self.fConst10 * fTemp535 * (self.fConst9 * fTemp535 + -444.1322) + 105.0;
																																																																																																																							fTemp537 = fTemp534 * (105.0 - self.fConst11 * fTemp535);
																																																																																																																							fTemp538 = self.fConst12 * (fTemp537 * (fTemp82 + self.fConst12 * (fTemp537 / fTemp536)) / fTemp536) + 1.0;
																																																																																																																						}
																																																																																																																							if iSlow21 != 0 {
																																																																																																																								iTemp539 = ((self.fRec120[1] >= 1.0) as i32) | iTemp5;
																																																																																																																								iTemp540 = (self.fRec120[1] < 0.0) as i32;
																																																																																																																								iTemp541 = i32::wrapping_add(self.iRec13[0], 2375700);
																																																																																																																								iTemp542 = i32::wrapping_mul(1664525, iTemp541 ^ (iTemp541 >> 15));
																																																																																																																								iTemp543 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp542 ^ (iTemp542 >> 13))) as F32 + 0.5))))) as i32;
																																																																																																																								fTemp544 = if iTemp540 | iTemp5 != 0 {if (iTemp543 >= 4) as i32 != 0 {if (iTemp543 >= 6) as i32 != 0 {if (iTemp543 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp543 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp543 >= 2) as i32 != 0 {if (iTemp543 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp543 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec121[1]};
																																																																																																																								iTemp545 = i32::wrapping_add(self.iRec13[0], 2304429);
																																																																																																																								iTemp546 = i32::wrapping_mul(1664525, iTemp545 ^ (iTemp545 >> 15));
																																																																																																																								fTemp547 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp546 ^ (iTemp546 >> 13))) as F32 + 0.5;
																																																																																																																								fTemp548 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp544, std::f32::consts::E) + fTemp547 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp544)), std::f32::consts::E)) + -8.228819)));
																																																																																																																								iTemp549 = i32::wrapping_add(self.iRec13[0], 2312348);
																																																																																																																								iTemp550 = i32::wrapping_mul(1664525, iTemp549 ^ (iTemp549 >> 15));
																																																																																																																								fTemp551 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp550 ^ (iTemp550 >> 13))) as F32;
																																																																																																																								fTemp552 = F32::floor(fTemp551 - fTemp548);
																																																																																																																								iTemp553 = (fTemp552) as i32;
																																																																																																																								fTemp554 = fTemp551 - (fTemp548 + fTemp552);
																																																																																																																								self.fRec122[0] = if iTemp540 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp554 * (fTemp554 * (fTemp554 * (fTemp554 * (0.0013333 * fTemp554 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp553))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp553))) as F32))} else {self.fRec122[1]};
																																																																																																																								fTemp555 = (iTemp539) as F32;
																																																																																																																							}
																																																																																																																								if iSlow21 & ((fTemp555 != 0.0) as i32) != 0 {
																																																																																																																									iTemp556 = i32::wrapping_add(self.iRec13[0], 2320267);
																																																																																																																									iTemp557 = i32::wrapping_mul(1664525, iTemp556 ^ (iTemp556 >> 15));
																																																																																																																									fTemp558 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp557 ^ (iTemp557 >> 13))) as F32;
																																																																																																																									fTemp559 = fTemp558 + 1.5;
																																																																																																																									fTemp560 = 0.5 - fTemp558;
																																																																																																																									fTemp561 = AlternativesDsp_faustpower2_f(fTemp560) / AlternativesDsp_faustpower2_f(fTemp559);
																																																																																																																								}
																																																																																																																									if iSlow21 != 0 {
																																																																																																																										self.fRec120[0] = if iTemp539 != 0 {-(2.0 * (fTemp560 * (15.0 - 4.0 * fTemp561) * fTemp555 / (fTemp559 * (15.0 - 9.0 * fTemp561))))} else {self.fRec120[1] + self.fConst6 * (if iTemp540 != 0 {fTemp27} else {1.0} / F32::max(self.fRec122[0], 0.005))};
																																																																																																																										self.fRec121[0] = fTemp544;
																																																																																																																										fTemp562 = F32::min(1.0, F32::max(0.0, self.fRec120[0]));
																																																																																																																										fTemp563 = (iTemp540) as F32;
																																																																																																																										iTemp564 = (fTemp563 != 0.0) as i32;
																																																																																																																									}
																																																																																																																										if iSlow21 & iTemp564 != 0 {
																																																																																																																											fTemp565 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec121[0])), std::f32::consts::E) * fTemp547;
																																																																																																																											fTemp566 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec121[0], std::f32::consts::E) + fTemp565) + -8.228819)));
																																																																																																																											fTemp567 = F32::floor(fTemp551 - fTemp566);
																																																																																																																											iTemp568 = (fTemp567) as i32;
																																																																																																																											fTemp569 = fTemp551 - (fTemp566 + fTemp567);
																																																																																																																											fTemp570 = fTemp563 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp569 * (fTemp569 * (fTemp569 * (fTemp569 * (0.0013333 * fTemp569 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp568))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp568))) as F32));
																																																																																																																										}
																																																																																																																											if iSlow21 != 0 {
																																																																																																																												self.fRec123[0] = if iTemp540 != 0 {fTemp570} else {self.fRec123[1]};
																																																																																																																												fTemp571 = F32::floor(41.666668 * self.fRec123[0] * fTemp562);
																																																																																																																												self.fVec9[0] = fTemp571;
																																																																																																																												fTemp572 = ((fTemp571 != self.fVec9[1]) as i32) as u32 as F32;
																																																																																																																												iTemp573 = (fTemp572 != 0.0) as i32;
																																																																																																																											}
																																																																																																																												if iSlow21 & iTemp573 != 0 {
																																																																																																																													iTemp574 = i32::wrapping_add(self.iRec13[0], 2351943);
																																																																																																																													iTemp575 = i32::wrapping_mul(1664525, iTemp574 ^ (iTemp574 >> 15));
																																																																																																																													fTemp576 = 4.656613e-10 * fTemp572 * (i32::wrapping_mul(1013904223, iTemp575 ^ (iTemp575 >> 13))) as F32;
																																																																																																																												}
																																																																																																																													if iSlow21 != 0 {
																																																																																																																														fTemp577 = ((self.fRec120[0] >= 0.0) as i32) as u32 as F32;
																																																																																																																														self.fRec119[0] = fTemp577 * (self.fRec119[1] + 0.35 * fTemp572 * fTemp576);
																																																																																																																														self.fRec118[0] = self.fConst7 * self.fRec119[0] + self.fConst4 * self.fRec118[1];
																																																																																																																													}
																																																																																																																														if iSlow21 & iTemp564 != 0 {
																																																																																																																															iTemp578 = i32::wrapping_add(self.iRec13[0], 2336105);
																																																																																																																															iTemp579 = i32::wrapping_mul(1664525, iTemp578 ^ (iTemp578 >> 15));
																																																																																																																															fTemp580 = 0.5 * fTemp563 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp579 ^ (iTemp579 >> 13))) as F32 + 0.5 + 1.0);
																																																																																																																														}
																																																																																																																															if iSlow21 != 0 {
																																																																																																																																self.fRec124[0] = if iTemp540 != 0 {fTemp580} else {self.fRec124[1]};
																																																																																																																																fTemp581 = F32::min(1.0, F32::max(-1.0, self.fRec124[0] + self.fRec118[0] * fTemp7));
																																																																																																																																fTemp582 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp562) * AlternativesDsp_faustpower2_f(1.0 - fTemp562) * fTemp581 * if (fTemp581 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																																																																																																fTemp583 = F32::floor(fTemp582);
																																																																																																																																iTemp584 = (fTemp583) as i32;
																																																																																																																																fTemp585 = fTemp582 - fTemp583;
																																																																																																																																fTemp586 = (fTemp585 * (fTemp585 * (fTemp585 * (fTemp585 * (0.0013333 * fTemp585 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp584))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp584))) as F32 + -1.0;
																																																																																																																															}
																																																																																																																																if iSlow21 & iTemp573 != 0 {
																																																																																																																																	iTemp587 = i32::wrapping_add(self.iRec13[0], 2344024);
																																																																																																																																	iTemp588 = i32::wrapping_mul(1664525, iTemp587 ^ (iTemp587 >> 15));
																																																																																																																																	fTemp589 = 4.656613e-10 * fTemp572 * (i32::wrapping_mul(1013904223, iTemp588 ^ (iTemp588 >> 13))) as F32;
																																																																																																																																}
																																																																																																																																	if iSlow21 != 0 {
																																																																																																																																		self.fRec129[0] = fTemp577 * (self.fRec129[1] + 0.35 * fTemp572 * fTemp589);
																																																																																																																																		self.fRec128[0] = self.fConst7 * self.fRec129[0] + self.fConst4 * self.fRec128[1];
																																																																																																																																		fTemp590 = self.fRec128[0] * fTemp7 * fTemp65;
																																																																																																																																		fTemp591 = F32::floor(fTemp590);
																																																																																																																																		iTemp592 = (fTemp591) as i32;
																																																																																																																																		fTemp593 = fTemp590 - fTemp591;
																																																																																																																																	}
																																																																																																																																		if iSlow21 & iTemp564 != 0 {
																																																																																																																																			fTemp594 = 1.442695 * fTemp565;
																																																																																																																																			fTemp595 = F32::floor(fTemp594);
																																																																																																																																			iTemp596 = (fTemp595) as i32;
																																																																																																																																			fTemp597 = fTemp594 - fTemp595;
																																																																																																																																			fTemp598 = F32::round((fTemp597 * (fTemp597 * (fTemp597 * (fTemp597 * (0.0013333 * fTemp597 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp596))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp596))) as F32) * fTemp563;
																																																																																																																																		}
																																																																																																																																			if iSlow21 != 0 {
																																																																																																																																				self.fRec130[0] = if iTemp540 != 0 {fTemp598} else {self.fRec130[1]};
																																																																																																																																				fTemp599 = F32::min(self.fConst5, F32::max(2e+01, self.fRec121[0] * self.fRec130[0] * (fTemp593 * (fTemp593 * (fTemp593 * (fTemp593 * (0.0013333 * fTemp593 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp592))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp592))) as F32));
																																																																																																																																				fTemp600 = AlternativesDsp_faustpower2_f(fTemp599);
																																																																																																																																				fTemp601 = self.fConst10 * fTemp600 * (self.fConst9 * fTemp600 + -444.1322) + 105.0;
																																																																																																																																				fTemp602 = fTemp599 * (105.0 - self.fConst11 * fTemp600);
																																																																																																																																				fTemp603 = self.fConst12 * (fTemp602 * (fTemp82 + self.fConst12 * (fTemp602 / fTemp601)) / fTemp601) + 1.0;
																																																																																																																																			}
																																																																																																																																				if iSlow15 != 0 {
																																																																																																																																					iTemp604 = ((self.fRec133[1] >= 1.0) as i32) | iTemp5;
																																																																																																																																					iTemp605 = (self.fRec133[1] < 0.0) as i32;
																																																																																																																																					iTemp606 = i32::wrapping_add(self.iRec13[0], 1607557);
																																																																																																																																					iTemp607 = i32::wrapping_mul(1664525, iTemp606 ^ (iTemp606 >> 15));
																																																																																																																																					iTemp608 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp607 ^ (iTemp607 >> 13))) as F32 + 0.5))))) as i32;
																																																																																																																																					fTemp609 = if iTemp605 | iTemp5 != 0 {if (iTemp608 >= 4) as i32 != 0 {if (iTemp608 >= 6) as i32 != 0 {if (iTemp608 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp608 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp608 >= 2) as i32 != 0 {if (iTemp608 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp608 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec134[1]};
																																																																																																																																					iTemp610 = i32::wrapping_add(self.iRec13[0], 1536286);
																																																																																																																																					iTemp611 = i32::wrapping_mul(1664525, iTemp610 ^ (iTemp610 >> 15));
																																																																																																																																					fTemp612 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp611 ^ (iTemp611 >> 13))) as F32 + 0.5;
																																																																																																																																					fTemp613 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp609, std::f32::consts::E) + fTemp612 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp609)), std::f32::consts::E)) + -8.228819)));
																																																																																																																																					iTemp614 = i32::wrapping_add(self.iRec13[0], 1544205);
																																																																																																																																					iTemp615 = i32::wrapping_mul(1664525, iTemp614 ^ (iTemp614 >> 15));
																																																																																																																																					fTemp616 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp615 ^ (iTemp615 >> 13))) as F32;
																																																																																																																																					fTemp617 = F32::floor(fTemp616 - fTemp613);
																																																																																																																																					iTemp618 = (fTemp617) as i32;
																																																																																																																																					fTemp619 = fTemp616 - (fTemp613 + fTemp617);
																																																																																																																																					self.fRec135[0] = if iTemp605 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp619 * (fTemp619 * (fTemp619 * (fTemp619 * (0.0013333 * fTemp619 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp618))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp618))) as F32))} else {self.fRec135[1]};
																																																																																																																																					fTemp620 = (iTemp604) as F32;
																																																																																																																																				}
																																																																																																																																					if iSlow15 & ((fTemp620 != 0.0) as i32) != 0 {
																																																																																																																																						iTemp621 = i32::wrapping_add(self.iRec13[0], 1552124);
																																																																																																																																						iTemp622 = i32::wrapping_mul(1664525, iTemp621 ^ (iTemp621 >> 15));
																																																																																																																																						fTemp623 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp622 ^ (iTemp622 >> 13))) as F32;
																																																																																																																																						fTemp624 = fTemp623 + 1.5;
																																																																																																																																						fTemp625 = 0.5 - fTemp623;
																																																																																																																																						fTemp626 = AlternativesDsp_faustpower2_f(fTemp625) / AlternativesDsp_faustpower2_f(fTemp624);
																																																																																																																																					}
																																																																																																																																						if iSlow15 != 0 {
																																																																																																																																							self.fRec133[0] = if iTemp604 != 0 {-(2.0 * (fTemp625 * (15.0 - 4.0 * fTemp626) * fTemp620 / (fTemp624 * (15.0 - 9.0 * fTemp626))))} else {self.fRec133[1] + self.fConst6 * (if iTemp605 != 0 {fTemp27} else {1.0} / F32::max(self.fRec135[0], 0.005))};
																																																																																																																																							self.fRec134[0] = fTemp609;
																																																																																																																																							fTemp627 = F32::min(1.0, F32::max(0.0, self.fRec133[0]));
																																																																																																																																							fTemp628 = (iTemp605) as F32;
																																																																																																																																							iTemp629 = (fTemp628 != 0.0) as i32;
																																																																																																																																						}
																																																																																																																																							if iSlow15 & iTemp629 != 0 {
																																																																																																																																								fTemp630 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec134[0])), std::f32::consts::E) * fTemp612;
																																																																																																																																								fTemp631 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec134[0], std::f32::consts::E) + fTemp630) + -8.228819)));
																																																																																																																																								fTemp632 = F32::floor(fTemp616 - fTemp631);
																																																																																																																																								iTemp633 = (fTemp632) as i32;
																																																																																																																																								fTemp634 = fTemp616 - (fTemp631 + fTemp632);
																																																																																																																																								fTemp635 = fTemp628 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp634 * (fTemp634 * (fTemp634 * (fTemp634 * (0.0013333 * fTemp634 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp633))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp633))) as F32));
																																																																																																																																							}
																																																																																																																																								if iSlow15 != 0 {
																																																																																																																																									self.fRec136[0] = if iTemp605 != 0 {fTemp635} else {self.fRec136[1]};
																																																																																																																																									fTemp636 = F32::floor(41.666668 * self.fRec136[0] * fTemp627);
																																																																																																																																									self.fVec10[0] = fTemp636;
																																																																																																																																									fTemp637 = ((fTemp636 != self.fVec10[1]) as i32) as u32 as F32;
																																																																																																																																									iTemp638 = (fTemp637 != 0.0) as i32;
																																																																																																																																								}
																																																																																																																																									if iSlow15 & iTemp638 != 0 {
																																																																																																																																										iTemp639 = i32::wrapping_add(self.iRec13[0], 1583800);
																																																																																																																																										iTemp640 = i32::wrapping_mul(1664525, iTemp639 ^ (iTemp639 >> 15));
																																																																																																																																										fTemp641 = 4.656613e-10 * fTemp637 * (i32::wrapping_mul(1013904223, iTemp640 ^ (iTemp640 >> 13))) as F32;
																																																																																																																																									}
																																																																																																																																										if iSlow15 != 0 {
																																																																																																																																											fTemp642 = ((self.fRec133[0] >= 0.0) as i32) as u32 as F32;
																																																																																																																																											self.fRec132[0] = fTemp642 * (self.fRec132[1] + 0.35 * fTemp637 * fTemp641);
																																																																																																																																											self.fRec131[0] = self.fConst7 * self.fRec132[0] + self.fConst4 * self.fRec131[1];
																																																																																																																																										}
																																																																																																																																											if iSlow15 & iTemp629 != 0 {
																																																																																																																																												iTemp643 = i32::wrapping_add(self.iRec13[0], 1567962);
																																																																																																																																												iTemp644 = i32::wrapping_mul(1664525, iTemp643 ^ (iTemp643 >> 15));
																																																																																																																																												fTemp645 = 0.5 * fTemp628 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp644 ^ (iTemp644 >> 13))) as F32 + 0.5 + 1.0);
																																																																																																																																											}
																																																																																																																																												if iSlow15 != 0 {
																																																																																																																																													self.fRec137[0] = if iTemp605 != 0 {fTemp645} else {self.fRec137[1]};
																																																																																																																																													fTemp646 = F32::min(1.0, F32::max(-1.0, self.fRec137[0] + self.fRec131[0] * fTemp7));
																																																																																																																																													fTemp647 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp627) * AlternativesDsp_faustpower2_f(1.0 - fTemp627) * fTemp646 * if (fTemp646 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																																																																																																													fTemp648 = F32::floor(fTemp647);
																																																																																																																																													iTemp649 = (fTemp648) as i32;
																																																																																																																																													fTemp650 = fTemp647 - fTemp648;
																																																																																																																																													fTemp651 = (fTemp650 * (fTemp650 * (fTemp650 * (fTemp650 * (0.0013333 * fTemp650 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp649))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp649))) as F32 + -1.0;
																																																																																																																																												}
																																																																																																																																													if iSlow15 & iTemp638 != 0 {
																																																																																																																																														iTemp652 = i32::wrapping_add(self.iRec13[0], 1575881);
																																																																																																																																														iTemp653 = i32::wrapping_mul(1664525, iTemp652 ^ (iTemp652 >> 15));
																																																																																																																																														fTemp654 = 4.656613e-10 * fTemp637 * (i32::wrapping_mul(1013904223, iTemp653 ^ (iTemp653 >> 13))) as F32;
																																																																																																																																													}
																																																																																																																																														if iSlow15 != 0 {
																																																																																																																																															self.fRec142[0] = fTemp642 * (self.fRec142[1] + 0.35 * fTemp637 * fTemp654);
																																																																																																																																															self.fRec141[0] = self.fConst7 * self.fRec142[0] + self.fConst4 * self.fRec141[1];
																																																																																																																																															fTemp655 = self.fRec141[0] * fTemp7 * fTemp65;
																																																																																																																																															fTemp656 = F32::floor(fTemp655);
																																																																																																																																															iTemp657 = (fTemp656) as i32;
																																																																																																																																															fTemp658 = fTemp655 - fTemp656;
																																																																																																																																														}
																																																																																																																																															if iSlow15 & iTemp629 != 0 {
																																																																																																																																																fTemp659 = 1.442695 * fTemp630;
																																																																																																																																																fTemp660 = F32::floor(fTemp659);
																																																																																																																																																iTemp661 = (fTemp660) as i32;
																																																																																																																																																fTemp662 = fTemp659 - fTemp660;
																																																																																																																																																fTemp663 = F32::round((fTemp662 * (fTemp662 * (fTemp662 * (fTemp662 * (0.0013333 * fTemp662 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp661))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp661))) as F32) * fTemp628;
																																																																																																																																															}
																																																																																																																																																if iSlow15 != 0 {
																																																																																																																																																	self.fRec143[0] = if iTemp605 != 0 {fTemp663} else {self.fRec143[1]};
																																																																																																																																																	fTemp664 = F32::min(self.fConst5, F32::max(2e+01, self.fRec134[0] * self.fRec143[0] * (fTemp658 * (fTemp658 * (fTemp658 * (fTemp658 * (0.0013333 * fTemp658 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp657))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp657))) as F32));
																																																																																																																																																	fTemp665 = AlternativesDsp_faustpower2_f(fTemp664);
																																																																																																																																																	fTemp666 = self.fConst10 * fTemp665 * (self.fConst9 * fTemp665 + -444.1322) + 105.0;
																																																																																																																																																	fTemp667 = fTemp664 * (105.0 - self.fConst11 * fTemp665);
																																																																																																																																																	fTemp668 = self.fConst12 * (fTemp667 * (fTemp82 + self.fConst12 * (fTemp667 / fTemp666)) / fTemp666) + 1.0;
																																																																																																																																																}
																																																																																																																																																	if iSlow18 != 0 {
																																																																																																																																																		iTemp669 = ((self.fRec146[1] >= 1.0) as i32) | iTemp5;
																																																																																																																																																		iTemp670 = (self.fRec146[1] < 0.0) as i32;
																																																																																																																																																		iTemp671 = i32::wrapping_add(self.iRec13[0], 839414);
																																																																																																																																																		iTemp672 = i32::wrapping_mul(1664525, iTemp671 ^ (iTemp671 >> 15));
																																																																																																																																																		iTemp673 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp672 ^ (iTemp672 >> 13))) as F32 + 0.5))))) as i32;
																																																																																																																																																		fTemp674 = if iTemp670 | iTemp5 != 0 {if (iTemp673 >= 4) as i32 != 0 {if (iTemp673 >= 6) as i32 != 0 {if (iTemp673 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp673 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp673 >= 2) as i32 != 0 {if (iTemp673 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp673 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec147[1]};
																																																																																																																																																		iTemp675 = i32::wrapping_add(self.iRec13[0], 768143);
																																																																																																																																																		iTemp676 = i32::wrapping_mul(1664525, iTemp675 ^ (iTemp675 >> 15));
																																																																																																																																																		fTemp677 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp676 ^ (iTemp676 >> 13))) as F32 + 0.5;
																																																																																																																																																		fTemp678 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp674, std::f32::consts::E) + fTemp677 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp674)), std::f32::consts::E)) + -8.228819)));
																																																																																																																																																		iTemp679 = i32::wrapping_add(self.iRec13[0], 776062);
																																																																																																																																																		iTemp680 = i32::wrapping_mul(1664525, iTemp679 ^ (iTemp679 >> 15));
																																																																																																																																																		fTemp681 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp680 ^ (iTemp680 >> 13))) as F32;
																																																																																																																																																		fTemp682 = F32::floor(fTemp681 - fTemp678);
																																																																																																																																																		iTemp683 = (fTemp682) as i32;
																																																																																																																																																		fTemp684 = fTemp681 - (fTemp678 + fTemp682);
																																																																																																																																																		self.fRec148[0] = if iTemp670 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp684 * (fTemp684 * (fTemp684 * (fTemp684 * (0.0013333 * fTemp684 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp683))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp683))) as F32))} else {self.fRec148[1]};
																																																																																																																																																		fTemp685 = (iTemp669) as F32;
																																																																																																																																																	}
																																																																																																																																																		if iSlow18 & ((fTemp685 != 0.0) as i32) != 0 {
																																																																																																																																																			iTemp686 = i32::wrapping_add(self.iRec13[0], 783981);
																																																																																																																																																			iTemp687 = i32::wrapping_mul(1664525, iTemp686 ^ (iTemp686 >> 15));
																																																																																																																																																			fTemp688 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp687 ^ (iTemp687 >> 13))) as F32;
																																																																																																																																																			fTemp689 = fTemp688 + 1.5;
																																																																																																																																																			fTemp690 = 0.5 - fTemp688;
																																																																																																																																																			fTemp691 = AlternativesDsp_faustpower2_f(fTemp690) / AlternativesDsp_faustpower2_f(fTemp689);
																																																																																																																																																		}
																																																																																																																																																			if iSlow18 != 0 {
																																																																																																																																																				self.fRec146[0] = if iTemp669 != 0 {-(2.0 * (fTemp690 * (15.0 - 4.0 * fTemp691) * fTemp685 / (fTemp689 * (15.0 - 9.0 * fTemp691))))} else {self.fRec146[1] + self.fConst6 * (if iTemp670 != 0 {fTemp27} else {1.0} / F32::max(self.fRec148[0], 0.005))};
																																																																																																																																																				self.fRec147[0] = fTemp674;
																																																																																																																																																				fTemp692 = F32::min(1.0, F32::max(0.0, self.fRec146[0]));
																																																																																																																																																				fTemp693 = (iTemp670) as F32;
																																																																																																																																																				iTemp694 = (fTemp693 != 0.0) as i32;
																																																																																																																																																			}
																																																																																																																																																				if iSlow18 & iTemp694 != 0 {
																																																																																																																																																					fTemp695 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec147[0])), std::f32::consts::E) * fTemp677;
																																																																																																																																																					fTemp696 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec147[0], std::f32::consts::E) + fTemp695) + -8.228819)));
																																																																																																																																																					fTemp697 = F32::floor(fTemp681 - fTemp696);
																																																																																																																																																					iTemp698 = (fTemp697) as i32;
																																																																																																																																																					fTemp699 = fTemp681 - (fTemp696 + fTemp697);
																																																																																																																																																					fTemp700 = fTemp693 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp699 * (fTemp699 * (fTemp699 * (fTemp699 * (0.0013333 * fTemp699 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp698))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp698))) as F32));
																																																																																																																																																				}
																																																																																																																																																					if iSlow18 != 0 {
																																																																																																																																																						self.fRec149[0] = if iTemp670 != 0 {fTemp700} else {self.fRec149[1]};
																																																																																																																																																						fTemp701 = F32::floor(41.666668 * self.fRec149[0] * fTemp692);
																																																																																																																																																						self.fVec11[0] = fTemp701;
																																																																																																																																																						fTemp702 = ((fTemp701 != self.fVec11[1]) as i32) as u32 as F32;
																																																																																																																																																						iTemp703 = (fTemp702 != 0.0) as i32;
																																																																																																																																																					}
																																																																																																																																																						if iSlow18 & iTemp703 != 0 {
																																																																																																																																																							iTemp704 = i32::wrapping_add(self.iRec13[0], 815657);
																																																																																																																																																							iTemp705 = i32::wrapping_mul(1664525, iTemp704 ^ (iTemp704 >> 15));
																																																																																																																																																							fTemp706 = 4.656613e-10 * fTemp702 * (i32::wrapping_mul(1013904223, iTemp705 ^ (iTemp705 >> 13))) as F32;
																																																																																																																																																						}
																																																																																																																																																							if iSlow18 != 0 {
																																																																																																																																																								fTemp707 = ((self.fRec146[0] >= 0.0) as i32) as u32 as F32;
																																																																																																																																																								self.fRec145[0] = fTemp707 * (self.fRec145[1] + 0.35 * fTemp702 * fTemp706);
																																																																																																																																																								self.fRec144[0] = self.fConst7 * self.fRec145[0] + self.fConst4 * self.fRec144[1];
																																																																																																																																																							}
																																																																																																																																																								if iSlow18 & iTemp694 != 0 {
																																																																																																																																																									iTemp708 = i32::wrapping_add(self.iRec13[0], 799819);
																																																																																																																																																									iTemp709 = i32::wrapping_mul(1664525, iTemp708 ^ (iTemp708 >> 15));
																																																																																																																																																									fTemp710 = 0.5 * fTemp693 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp709 ^ (iTemp709 >> 13))) as F32 + 0.5 + 1.0);
																																																																																																																																																								}
																																																																																																																																																									if iSlow18 != 0 {
																																																																																																																																																										self.fRec150[0] = if iTemp670 != 0 {fTemp710} else {self.fRec150[1]};
																																																																																																																																																										fTemp711 = F32::min(1.0, F32::max(-1.0, self.fRec150[0] + self.fRec144[0] * fTemp7));
																																																																																																																																																										fTemp712 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp692) * AlternativesDsp_faustpower2_f(1.0 - fTemp692) * fTemp711 * if (fTemp711 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																																																																																																																										fTemp713 = F32::floor(fTemp712);
																																																																																																																																																										iTemp714 = (fTemp713) as i32;
																																																																																																																																																										fTemp715 = fTemp712 - fTemp713;
																																																																																																																																																										fTemp716 = (fTemp715 * (fTemp715 * (fTemp715 * (fTemp715 * (0.0013333 * fTemp715 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp714))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp714))) as F32 + -1.0;
																																																																																																																																																									}
																																																																																																																																																										if iSlow18 & iTemp703 != 0 {
																																																																																																																																																											iTemp717 = i32::wrapping_add(self.iRec13[0], 807738);
																																																																																																																																																											iTemp718 = i32::wrapping_mul(1664525, iTemp717 ^ (iTemp717 >> 15));
																																																																																																																																																											fTemp719 = 4.656613e-10 * fTemp702 * (i32::wrapping_mul(1013904223, iTemp718 ^ (iTemp718 >> 13))) as F32;
																																																																																																																																																										}
																																																																																																																																																											if iSlow18 != 0 {
																																																																																																																																																												self.fRec155[0] = fTemp707 * (self.fRec155[1] + 0.35 * fTemp702 * fTemp719);
																																																																																																																																																												self.fRec154[0] = self.fConst7 * self.fRec155[0] + self.fConst4 * self.fRec154[1];
																																																																																																																																																												fTemp720 = self.fRec154[0] * fTemp7 * fTemp65;
																																																																																																																																																												fTemp721 = F32::floor(fTemp720);
																																																																																																																																																												iTemp722 = (fTemp721) as i32;
																																																																																																																																																												fTemp723 = fTemp720 - fTemp721;
																																																																																																																																																											}
																																																																																																																																																												if iSlow18 & iTemp694 != 0 {
																																																																																																																																																													fTemp724 = 1.442695 * fTemp695;
																																																																																																																																																													fTemp725 = F32::floor(fTemp724);
																																																																																																																																																													iTemp726 = (fTemp725) as i32;
																																																																																																																																																													fTemp727 = fTemp724 - fTemp725;
																																																																																																																																																													fTemp728 = F32::round((fTemp727 * (fTemp727 * (fTemp727 * (fTemp727 * (0.0013333 * fTemp727 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp726))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp726))) as F32) * fTemp693;
																																																																																																																																																												}
																																																																																																																																																													if iSlow18 != 0 {
																																																																																																																																																														self.fRec156[0] = if iTemp670 != 0 {fTemp728} else {self.fRec156[1]};
																																																																																																																																																														fTemp729 = F32::min(self.fConst5, F32::max(2e+01, self.fRec147[0] * self.fRec156[0] * (fTemp723 * (fTemp723 * (fTemp723 * (fTemp723 * (0.0013333 * fTemp723 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp722))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp722))) as F32));
																																																																																																																																																														fTemp730 = AlternativesDsp_faustpower2_f(fTemp729);
																																																																																																																																																														fTemp731 = self.fConst10 * fTemp730 * (self.fConst9 * fTemp730 + -444.1322) + 105.0;
																																																																																																																																																														fTemp732 = fTemp729 * (105.0 - self.fConst11 * fTemp730);
																																																																																																																																																														fTemp733 = self.fConst12 * (fTemp732 * (fTemp82 + self.fConst12 * (fTemp732 / fTemp731)) / fTemp731) + 1.0;
																																																																																																																																																													}
																																																																																																																																																														if iSlow12 != 0 {
																																																																																																																																																															iTemp734 = ((self.fRec159[1] >= 1.0) as i32) | iTemp5;
																																																																																																																																																															iTemp735 = (self.fRec159[1] < 0.0) as i32;
																																																																																																																																																															iTemp736 = i32::wrapping_add(self.iRec13[0], 71271);
																																																																																																																																																															iTemp737 = i32::wrapping_mul(1664525, iTemp736 ^ (iTemp736 >> 15));
																																																																																																																																																															iTemp738 = (F32::min(fSlow45, F32::max(0.0, F32::floor(fSlow44 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp737 ^ (iTemp737 >> 13))) as F32 + 0.5))))) as i32;
																																																																																																																																																															fTemp739 = if iTemp735 | iTemp5 != 0 {if (iTemp738 >= 4) as i32 != 0 {if (iTemp738 >= 6) as i32 != 0 {if (iTemp738 >= 7) as i32 != 0 {fSlow54} else {fSlow53}} else {if (iTemp738 >= 5) as i32 != 0 {fSlow52} else {fSlow51}}} else {if (iTemp738 >= 2) as i32 != 0 {if (iTemp738 >= 3) as i32 != 0 {fSlow50} else {fSlow49}} else {if (iTemp738 >= 1) as i32 != 0 {fSlow48} else {fSlow47}}}} else {self.fRec160[1]};
																																																																																																																																																															iTemp740 = i32::wrapping_mul(1664525, self.iRec13[0] ^ (self.iRec13[0] >> 15));
																																																																																																																																																															fTemp741 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp740 ^ (iTemp740 >> 13))) as F32 + 0.5;
																																																																																																																																																															fTemp742 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(fTemp739, std::f32::consts::E) + fTemp741 * F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / fTemp739)), std::f32::consts::E)) + -8.228819)));
																																																																																																																																																															iTemp743 = i32::wrapping_add(self.iRec13[0], 7919);
																																																																																																																																																															iTemp744 = i32::wrapping_mul(1664525, iTemp743 ^ (iTemp743 >> 15));
																																																																																																																																																															fTemp745 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp744 ^ (iTemp744 >> 13))) as F32;
																																																																																																																																																															fTemp746 = F32::floor(fTemp745 - fTemp742);
																																																																																																																																																															iTemp747 = (fTemp746) as i32;
																																																																																																																																																															fTemp748 = fTemp745 - (fTemp742 + fTemp746);
																																																																																																																																																															self.fRec161[0] = if iTemp735 != 0 {F32::max(0.005, 0.4 * (fTemp25 * (fTemp748 * (fTemp748 * (fTemp748 * (fTemp748 * (0.0013333 * fTemp748 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp747))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp747))) as F32))} else {self.fRec161[1]};
																																																																																																																																																															fTemp749 = (iTemp734) as F32;
																																																																																																																																																														}
																																																																																																																																																															if iSlow12 & ((fTemp749 != 0.0) as i32) != 0 {
																																																																																																																																																																iTemp750 = i32::wrapping_add(self.iRec13[0], 15838);
																																																																																																																																																																iTemp751 = i32::wrapping_mul(1664525, iTemp750 ^ (iTemp750 >> 15));
																																																																																																																																																																fTemp752 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp751 ^ (iTemp751 >> 13))) as F32;
																																																																																																																																																																fTemp753 = fTemp752 + 1.5;
																																																																																																																																																																fTemp754 = 0.5 - fTemp752;
																																																																																																																																																																fTemp755 = AlternativesDsp_faustpower2_f(fTemp754) / AlternativesDsp_faustpower2_f(fTemp753);
																																																																																																																																																															}
																																																																																																																																																																if iSlow12 != 0 {
																																																																																																																																																																	self.fRec159[0] = if iTemp734 != 0 {-(2.0 * (fTemp754 * (15.0 - 4.0 * fTemp755) * fTemp749 / (fTemp753 * (15.0 - 9.0 * fTemp755))))} else {self.fRec159[1] + self.fConst6 * (if iTemp735 != 0 {fTemp27} else {1.0} / F32::max(self.fRec161[0], 0.005))};
																																																																																																																																																																	self.fRec160[0] = fTemp739;
																																																																																																																																																																	fTemp756 = F32::min(1.0, F32::max(0.0, self.fRec159[0]));
																																																																																																																																																																	fTemp757 = (iTemp735) as F32;
																																																																																																																																																																	iTemp758 = (fTemp757 != 0.0) as i32;
																																																																																																																																																																}
																																																																																																																																																																	if iSlow12 & iTemp758 != 0 {
																																																																																																																																																																		fTemp759 = F32::log(F32::min(fTemp10, F32::max(1.0, self.fConst5 / self.fRec160[0])), std::f32::consts::E) * fTemp741;
																																																																																																																																																																		fTemp760 = 3.321928 * F32::min(1.0, F32::max(0.0, 0.20379505 * (1.442695 * (F32::log(self.fRec160[0], std::f32::consts::E) + fTemp759) + -8.228819)));
																																																																																																																																																																		fTemp761 = F32::floor(fTemp745 - fTemp760);
																																																																																																																																																																		iTemp762 = (fTemp761) as i32;
																																																																																																																																																																		fTemp763 = fTemp745 - (fTemp760 + fTemp761);
																																																																																																																																																																		fTemp764 = fTemp757 * F32::max(0.005, 0.4 * (fTemp25 * (fTemp763 * (fTemp763 * (fTemp763 * (fTemp763 * (0.0013333 * fTemp763 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp762))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp762))) as F32));
																																																																																																																																																																	}
																																																																																																																																																																		if iSlow12 != 0 {
																																																																																																																																																																			self.fRec162[0] = if iTemp735 != 0 {fTemp764} else {self.fRec162[1]};
																																																																																																																																																																			fTemp765 = F32::floor(41.666668 * self.fRec162[0] * fTemp756);
																																																																																																																																																																			self.fVec12[0] = fTemp765;
																																																																																																																																																																			fTemp766 = ((fTemp765 != self.fVec12[1]) as i32) as u32 as F32;
																																																																																																																																																																			iTemp767 = (fTemp766 != 0.0) as i32;
																																																																																																																																																																		}
																																																																																																																																																																			if iSlow12 & iTemp767 != 0 {
																																																																																																																																																																				iTemp768 = i32::wrapping_add(self.iRec13[0], 47514);
																																																																																																																																																																				iTemp769 = i32::wrapping_mul(1664525, iTemp768 ^ (iTemp768 >> 15));
																																																																																																																																																																				fTemp770 = 4.656613e-10 * fTemp766 * (i32::wrapping_mul(1013904223, iTemp769 ^ (iTemp769 >> 13))) as F32;
																																																																																																																																																																			}
																																																																																																																																																																				if iSlow12 != 0 {
																																																																																																																																																																					fTemp771 = ((self.fRec159[0] >= 0.0) as i32) as u32 as F32;
																																																																																																																																																																					self.fRec158[0] = fTemp771 * (self.fRec158[1] + 0.35 * fTemp766 * fTemp770);
																																																																																																																																																																					self.fRec157[0] = self.fConst7 * self.fRec158[0] + self.fConst4 * self.fRec157[1];
																																																																																																																																																																				}
																																																																																																																																																																					if iSlow12 & iTemp758 != 0 {
																																																																																																																																																																						iTemp772 = i32::wrapping_add(self.iRec13[0], 31676);
																																																																																																																																																																						iTemp773 = i32::wrapping_mul(1664525, iTemp772 ^ (iTemp772 >> 15));
																																																																																																																																																																						fTemp774 = 0.5 * fTemp757 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp773 ^ (iTemp773 >> 13))) as F32 + 0.5 + 1.0);
																																																																																																																																																																					}
																																																																																																																																																																						if iSlow12 != 0 {
																																																																																																																																																																							self.fRec163[0] = if iTemp735 != 0 {fTemp774} else {self.fRec163[1]};
																																																																																																																																																																							fTemp775 = F32::min(1.0, F32::max(-1.0, self.fRec163[0] + self.fRec157[0] * fTemp7));
																																																																																																																																																																							fTemp776 = 2.6575425 * AlternativesDsp_faustpower2_f(fTemp756) * AlternativesDsp_faustpower2_f(1.0 - fTemp756) * fTemp775 * if (fTemp775 < 0.0) as i32 != 0 {fTemp58} else {fTemp57};
																																																																																																																																																																							fTemp777 = F32::floor(fTemp776);
																																																																																																																																																																							iTemp778 = (fTemp777) as i32;
																																																																																																																																																																							fTemp779 = fTemp776 - fTemp777;
																																																																																																																																																																							fTemp780 = (fTemp779 * (fTemp779 * (fTemp779 * (fTemp779 * (0.0013333 * fTemp779 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp778))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp778))) as F32 + -1.0;
																																																																																																																																																																						}
																																																																																																																																																																							if iSlow12 & iTemp767 != 0 {
																																																																																																																																																																								iTemp781 = i32::wrapping_add(self.iRec13[0], 39595);
																																																																																																																																																																								iTemp782 = i32::wrapping_mul(1664525, iTemp781 ^ (iTemp781 >> 15));
																																																																																																																																																																								fTemp783 = 4.656613e-10 * fTemp766 * (i32::wrapping_mul(1013904223, iTemp782 ^ (iTemp782 >> 13))) as F32;
																																																																																																																																																																							}
																																																																																																																																																																								if iSlow12 != 0 {
																																																																																																																																																																									self.fRec168[0] = fTemp771 * (self.fRec168[1] + 0.35 * fTemp766 * fTemp783);
																																																																																																																																																																									self.fRec167[0] = self.fConst7 * self.fRec168[0] + self.fConst4 * self.fRec167[1];
																																																																																																																																																																									fTemp784 = self.fRec167[0] * fTemp7 * fTemp65;
																																																																																																																																																																									fTemp785 = F32::floor(fTemp784);
																																																																																																																																																																									iTemp786 = (fTemp785) as i32;
																																																																																																																																																																									fTemp787 = fTemp784 - fTemp785;
																																																																																																																																																																								}
																																																																																																																																																																									if iSlow12 & iTemp758 != 0 {
																																																																																																																																																																										fTemp788 = 1.442695 * fTemp759;
																																																																																																																																																																										fTemp789 = F32::floor(fTemp788);
																																																																																																																																																																										iTemp790 = (fTemp789) as i32;
																																																																																																																																																																										fTemp791 = fTemp788 - fTemp789;
																																																																																																																																																																										fTemp792 = F32::round((fTemp791 * (fTemp791 * (fTemp791 * (fTemp791 * (0.0013333 * fTemp791 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp790))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp790))) as F32) * fTemp757;
																																																																																																																																																																									}
																																																																																																																																																																										if iSlow12 != 0 {
																																																																																																																																																																											self.fRec169[0] = if iTemp735 != 0 {fTemp792} else {self.fRec169[1]};
																																																																																																																																																																											fTemp793 = F32::min(self.fConst5, F32::max(2e+01, self.fRec160[0] * self.fRec169[0] * (fTemp787 * (fTemp787 * (fTemp787 * (fTemp787 * (0.0013333 * fTemp787 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp786))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp786))) as F32));
																																																																																																																																																																											fTemp794 = AlternativesDsp_faustpower2_f(fTemp793);
																																																																																																																																																																											fTemp795 = self.fConst10 * fTemp794 * (self.fConst9 * fTemp794 + -444.1322) + 105.0;
																																																																																																																																																																											fTemp796 = fTemp793 * (105.0 - self.fConst11 * fTemp794);
																																																																																																																																																																											fTemp797 = self.fConst12 * (fTemp796 * (fTemp82 + self.fConst12 * (fTemp796 / fTemp795)) / fTemp795) + 1.0;
																																																																																																																																																																											fTemp798 = fTemp796 * (fTemp3 - self.fRec165[1]) / fTemp795;
																																																																																																																																																																											fTemp799 = self.fRec164[1] + self.fConst12 * fTemp798;
																																																																																																																																																																											self.fRec164[0] = 2.0 * (fTemp799 / fTemp797) - self.fRec164[1];
																																																																																																																																																																											fTemp800 = fTemp795 * fTemp797;
																																																																																																																																																																											self.fRec165[0] = 2.0 * (self.fRec165[1] + self.fConst6 * (fTemp796 * (3.1415927 * self.fRec164[1] + self.fConst13 * fTemp798) / fTemp800)) - self.fRec165[1];
																																																																																																																																																																											fTemp801 = fTemp64 * fTemp797;
																																																																																																																																																																											fRec166 = 0.6666667 * (fTemp799 / fTemp801);
																																																																																																																																																																											fTemp802 = fSlow11 * fRec166 * fTemp780;
																																																																																																																																																																										}
																																																																																																																																																																											let mut fTemp803: F32 = fSlow60 * fTemp802;
																																																																																																																																																																											if iSlow18 != 0 {
																																																																																																																																																																												fTemp804 = fTemp732 * (fTemp3 + fTemp803 - self.fRec152[1]) / fTemp731;
																																																																																																																																																																												fTemp805 = self.fRec151[1] + self.fConst12 * fTemp804;
																																																																																																																																																																												self.fRec151[0] = 2.0 * (fTemp805 / fTemp733) - self.fRec151[1];
																																																																																																																																																																												fTemp806 = fTemp731 * fTemp733;
																																																																																																																																																																												self.fRec152[0] = 2.0 * (self.fRec152[1] + self.fConst6 * (fTemp732 * (3.1415927 * self.fRec151[1] + self.fConst13 * fTemp804) / fTemp806)) - self.fRec152[1];
																																																																																																																																																																												fTemp807 = fTemp64 * fTemp733;
																																																																																																																																																																												fRec153 = 0.6666667 * (fTemp805 / fTemp807);
																																																																																																																																																																												fTemp808 = fSlow17 * fRec153 * fTemp716;
																																																																																																																																																																											}
																																																																																																																																																																												let mut fTemp809: F32 = fTemp803 + fSlow61 * fTemp808;
																																																																																																																																																																												if iSlow15 != 0 {
																																																																																																																																																																													fTemp810 = fTemp667 * (fTemp3 + fTemp809 - self.fRec139[1]) / fTemp666;
																																																																																																																																																																													fTemp811 = self.fRec138[1] + self.fConst12 * fTemp810;
																																																																																																																																																																													self.fRec138[0] = 2.0 * (fTemp811 / fTemp668) - self.fRec138[1];
																																																																																																																																																																													fTemp812 = fTemp666 * fTemp668;
																																																																																																																																																																													self.fRec139[0] = 2.0 * (self.fRec139[1] + self.fConst6 * (fTemp667 * (3.1415927 * self.fRec138[1] + self.fConst13 * fTemp810) / fTemp812)) - self.fRec139[1];
																																																																																																																																																																													fTemp813 = fTemp64 * fTemp668;
																																																																																																																																																																													fRec140 = 0.6666667 * (fTemp811 / fTemp813);
																																																																																																																																																																													fTemp814 = fSlow14 * fRec140 * fTemp651;
																																																																																																																																																																												}
																																																																																																																																																																													let mut fTemp815: F32 = fTemp809 + fSlow62 * fTemp814;
																																																																																																																																																																													if iSlow21 != 0 {
																																																																																																																																																																														fTemp816 = fTemp602 * (fTemp3 + fTemp815 - self.fRec126[1]) / fTemp601;
																																																																																																																																																																														fTemp817 = self.fRec125[1] + self.fConst12 * fTemp816;
																																																																																																																																																																														self.fRec125[0] = 2.0 * (fTemp817 / fTemp603) - self.fRec125[1];
																																																																																																																																																																														fTemp818 = fTemp601 * fTemp603;
																																																																																																																																																																														self.fRec126[0] = 2.0 * (self.fRec126[1] + self.fConst6 * (fTemp602 * (3.1415927 * self.fRec125[1] + self.fConst13 * fTemp816) / fTemp818)) - self.fRec126[1];
																																																																																																																																																																														fTemp819 = fTemp64 * fTemp603;
																																																																																																																																																																														fRec127 = 0.6666667 * (fTemp817 / fTemp819);
																																																																																																																																																																														fTemp820 = fSlow20 * fRec127 * fTemp586;
																																																																																																																																																																													}
																																																																																																																																																																														let mut fTemp821: F32 = fTemp815 + fSlow63 * fTemp820;
																																																																																																																																																																														if iSlow39 != 0 {
																																																																																																																																																																															fTemp822 = fTemp537 * (fTemp3 + fTemp821 - self.fRec113[1]) / fTemp536;
																																																																																																																																																																															fTemp823 = self.fRec112[1] + self.fConst12 * fTemp822;
																																																																																																																																																																															self.fRec112[0] = 2.0 * (fTemp823 / fTemp538) - self.fRec112[1];
																																																																																																																																																																															fTemp824 = fTemp536 * fTemp538;
																																																																																																																																																																															self.fRec113[0] = 2.0 * (self.fRec113[1] + self.fConst6 * (fTemp537 * (3.1415927 * self.fRec112[1] + self.fConst13 * fTemp822) / fTemp824)) - self.fRec113[1];
																																																																																																																																																																															fTemp825 = fTemp64 * fTemp538;
																																																																																																																																																																															fRec114 = 0.6666667 * (fTemp823 / fTemp825);
																																																																																																																																																																															fTemp826 = fSlow38 * fRec114 * fTemp521;
																																																																																																																																																																														}
																																																																																																																																																																															let mut fTemp827: F32 = fTemp821 + fSlow64 * fTemp826;
																																																																																																																																																																															if iSlow36 != 0 {
																																																																																																																																																																																fTemp828 = fTemp472 * (fTemp3 + fTemp827 - self.fRec100[1]) / fTemp471;
																																																																																																																																																																																fTemp829 = self.fRec99[1] + self.fConst12 * fTemp828;
																																																																																																																																																																																self.fRec99[0] = 2.0 * (fTemp829 / fTemp473) - self.fRec99[1];
																																																																																																																																																																																fTemp830 = fTemp471 * fTemp473;
																																																																																																																																																																																self.fRec100[0] = 2.0 * (self.fRec100[1] + self.fConst6 * (fTemp472 * (3.1415927 * self.fRec99[1] + self.fConst13 * fTemp828) / fTemp830)) - self.fRec100[1];
																																																																																																																																																																																fTemp831 = fTemp64 * fTemp473;
																																																																																																																																																																																fRec101 = 0.6666667 * (fTemp829 / fTemp831);
																																																																																																																																																																																fTemp832 = fSlow35 * fRec101 * fTemp456;
																																																																																																																																																																															}
																																																																																																																																																																																let mut fTemp833: F32 = fTemp827 + fSlow65 * fTemp832;
																																																																																																																																																																																if iSlow33 != 0 {
																																																																																																																																																																																	fTemp834 = fTemp407 * (fTemp3 + fTemp833 - self.fRec87[1]) / fTemp406;
																																																																																																																																																																																	fTemp835 = self.fRec86[1] + self.fConst12 * fTemp834;
																																																																																																																																																																																	self.fRec86[0] = 2.0 * (fTemp835 / fTemp408) - self.fRec86[1];
																																																																																																																																																																																	fTemp836 = fTemp406 * fTemp408;
																																																																																																																																																																																	self.fRec87[0] = 2.0 * (self.fRec87[1] + self.fConst6 * (fTemp407 * (3.1415927 * self.fRec86[1] + self.fConst13 * fTemp834) / fTemp836)) - self.fRec87[1];
																																																																																																																																																																																	fTemp837 = fTemp64 * fTemp408;
																																																																																																																																																																																	fRec88 = 0.6666667 * (fTemp835 / fTemp837);
																																																																																																																																																																																	fTemp838 = fSlow32 * fRec88 * fTemp391;
																																																																																																																																																																																}
																																																																																																																																																																																	let mut fTemp839: F32 = fTemp833 + fSlow66 * fTemp838;
																																																																																																																																																																																	if iSlow30 != 0 {
																																																																																																																																																																																		fTemp840 = fTemp342 * (fTemp3 + fTemp839 - self.fRec74[1]) / fTemp341;
																																																																																																																																																																																		fTemp841 = self.fRec73[1] + self.fConst12 * fTemp840;
																																																																																																																																																																																		self.fRec73[0] = 2.0 * (fTemp841 / fTemp343) - self.fRec73[1];
																																																																																																																																																																																		fTemp842 = fTemp341 * fTemp343;
																																																																																																																																																																																		self.fRec74[0] = 2.0 * (self.fRec74[1] + self.fConst6 * (fTemp342 * (3.1415927 * self.fRec73[1] + self.fConst13 * fTemp840) / fTemp842)) - self.fRec74[1];
																																																																																																																																																																																		fTemp843 = fTemp64 * fTemp343;
																																																																																																																																																																																		fRec75 = 0.6666667 * (fTemp841 / fTemp843);
																																																																																																																																																																																		fTemp844 = fSlow29 * fRec75 * fTemp326;
																																																																																																																																																																																	}
																																																																																																																																																																																		let mut fTemp845: F32 = fTemp839 + fSlow67 * fTemp844;
																																																																																																																																																																																		if iSlow27 != 0 {
																																																																																																																																																																																			fTemp846 = fTemp277 * (fTemp3 + fTemp845 - self.fRec61[1]) / fTemp276;
																																																																																																																																																																																			fTemp847 = self.fRec60[1] + self.fConst12 * fTemp846;
																																																																																																																																																																																			self.fRec60[0] = 2.0 * (fTemp847 / fTemp278) - self.fRec60[1];
																																																																																																																																																																																			fTemp848 = fTemp276 * fTemp278;
																																																																																																																																																																																			self.fRec61[0] = 2.0 * (self.fRec61[1] + self.fConst6 * (fTemp277 * (3.1415927 * self.fRec60[1] + self.fConst13 * fTemp846) / fTemp848)) - self.fRec61[1];
																																																																																																																																																																																			fTemp849 = fTemp64 * fTemp278;
																																																																																																																																																																																			fRec62 = 0.6666667 * (fTemp847 / fTemp849);
																																																																																																																																																																																			fTemp850 = fSlow26 * fRec62 * fTemp261;
																																																																																																																																																																																		}
																																																																																																																																																																																			let mut fTemp851: F32 = fTemp845 + fSlow68 * fTemp850;
																																																																																																																																																																																			if iSlow24 != 0 {
																																																																																																																																																																																				fTemp852 = fTemp212 * (fTemp3 + fTemp851 - self.fRec48[1]) / fTemp211;
																																																																																																																																																																																				fTemp853 = self.fRec47[1] + self.fConst12 * fTemp852;
																																																																																																																																																																																				self.fRec47[0] = 2.0 * (fTemp853 / fTemp213) - self.fRec47[1];
																																																																																																																																																																																				fTemp854 = fTemp211 * fTemp213;
																																																																																																																																																																																				self.fRec48[0] = 2.0 * (self.fRec48[1] + self.fConst6 * (fTemp212 * (3.1415927 * self.fRec47[1] + self.fConst13 * fTemp852) / fTemp854)) - self.fRec48[1];
																																																																																																																																																																																				fTemp855 = fTemp64 * fTemp213;
																																																																																																																																																																																				fRec49 = 0.6666667 * (fTemp853 / fTemp855);
																																																																																																																																																																																				fTemp856 = fSlow23 * fRec49 * fTemp196;
																																																																																																																																																																																			}
																																																																																																																																																																																				let mut fTemp857: F32 = fTemp851 + fSlow69 * fTemp856;
																																																																																																																																																																																				if iSlow9 != 0 {
																																																																																																																																																																																					fTemp858 = fTemp147 * (fTemp3 + fTemp857 - self.fRec35[1]) / fTemp146;
																																																																																																																																																																																					fTemp859 = self.fRec34[1] + self.fConst12 * fTemp858;
																																																																																																																																																																																					self.fRec34[0] = 2.0 * (fTemp859 / fTemp148) - self.fRec34[1];
																																																																																																																																																																																					fTemp860 = fTemp146 * fTemp148;
																																																																																																																																																																																					self.fRec35[0] = 2.0 * (self.fRec35[1] + self.fConst6 * (fTemp147 * (3.1415927 * self.fRec34[1] + self.fConst13 * fTemp858) / fTemp860)) - self.fRec35[1];
																																																																																																																																																																																					fTemp861 = fTemp64 * fTemp148;
																																																																																																																																																																																					fRec36 = 0.6666667 * (fTemp859 / fTemp861);
																																																																																																																																																																																					fTemp862 = fSlow8 * fRec36 * fTemp131;
																																																																																																																																																																																				}
																																																																																																																																																																																					let mut fTemp863: F32 = fTemp857 + fSlow70 * fTemp862;
																																																																																																																																																																																					if iSlow5 != 0 {
																																																																																																																																																																																						fTemp864 = fTemp81 * (fTemp3 + fTemp863 - self.fRec21[1]) / fTemp80;
																																																																																																																																																																																						fTemp865 = self.fRec20[1] + self.fConst12 * fTemp864;
																																																																																																																																																																																						self.fRec20[0] = 2.0 * (fTemp865 / fTemp83) - self.fRec20[1];
																																																																																																																																																																																						fTemp866 = fTemp80 * fTemp83;
																																																																																																																																																																																						self.fRec21[0] = 2.0 * (self.fRec21[1] + self.fConst6 * (fTemp81 * (3.1415927 * self.fRec20[1] + self.fConst13 * fTemp864) / fTemp866)) - self.fRec21[1];
																																																																																																																																																																																						fTemp867 = fTemp64 * fTemp83;
																																																																																																																																																																																						fRec22 = 0.6666667 * (fTemp865 / fTemp867);
																																																																																																																																																																																						fTemp868 = fSlow4 * fRec22 * fTemp63;
																																																																																																																																																																																					}
																																																																																																																																																																																						let mut fTemp869: F32 = fTemp3 + fTemp863 + fSlow71 * fTemp868;
																																																																																																																																																																																						self.fRec170[0] = self.fConst15 * AlternativesDsp_faustpower2_f(fTemp869) + self.fConst14 * self.fRec170[1];
																																																																																																																																																																																						self.fRec171[0] = self.fConst15 * AlternativesDsp_faustpower2_f(fTemp3) + self.fConst14 * self.fRec171[1];
																																																																																																																																																																																						self.fRec172[0] = fSlow72 * fTemp1 + 0.999 * fTemp0 * self.fRec172[1];
																																																																																																																																																																																						let mut fTemp870: F32 = if (F32::abs(self.fRec172[0] - fSlow72) < 1e-06) as i32 != 0 {fSlow72} else {self.fRec172[0]};
																																																																																																																																																																																						*output0 = (fTemp3 + fTemp870 * (if iSlow1 != 0 {F32::sqrt((self.fRec171[0] + 1e-09) / (self.fRec170[0] + 1e-09)) * fTemp869} else {fTemp869} - fTemp3)) * fTemp2;
																																																																																																																																																																																						let mut fTemp871: F32 = *input1;
																																																																																																																																																																																						if iSlow12 != 0 {
																																																																																																																																																																																							fTemp872 = fTemp796 * (fTemp871 - self.fRec207[1]) / fTemp795;
																																																																																																																																																																																							fTemp873 = self.fRec206[1] + self.fConst12 * fTemp872;
																																																																																																																																																																																							self.fRec206[0] = 2.0 * (fTemp873 / fTemp797) - self.fRec206[1];
																																																																																																																																																																																							self.fRec207[0] = 2.0 * (self.fRec207[1] + self.fConst6 * (fTemp796 * (3.1415927 * self.fRec206[1] + self.fConst13 * fTemp872) / fTemp800)) - self.fRec207[1];
																																																																																																																																																																																							fRec208 = 0.6666667 * (fTemp873 / fTemp801);
																																																																																																																																																																																							fTemp874 = fSlow11 * fRec208 * fTemp780;
																																																																																																																																																																																						}
																																																																																																																																																																																							let mut fTemp875: F32 = fSlow60 * fTemp874;
																																																																																																																																																																																							if iSlow18 != 0 {
																																																																																																																																																																																								fTemp876 = fTemp732 * (fTemp871 + fTemp875 - self.fRec204[1]) / fTemp731;
																																																																																																																																																																																								fTemp877 = self.fRec203[1] + self.fConst12 * fTemp876;
																																																																																																																																																																																								self.fRec203[0] = 2.0 * (fTemp877 / fTemp733) - self.fRec203[1];
																																																																																																																																																																																								self.fRec204[0] = 2.0 * (self.fRec204[1] + self.fConst6 * (fTemp732 * (3.1415927 * self.fRec203[1] + self.fConst13 * fTemp876) / fTemp806)) - self.fRec204[1];
																																																																																																																																																																																								fRec205 = 0.6666667 * (fTemp877 / fTemp807);
																																																																																																																																																																																								fTemp878 = fSlow17 * fRec205 * fTemp716;
																																																																																																																																																																																							}
																																																																																																																																																																																								let mut fTemp879: F32 = fTemp875 + fSlow61 * fTemp878;
																																																																																																																																																																																								if iSlow15 != 0 {
																																																																																																																																																																																									fTemp880 = fTemp667 * (fTemp871 + fTemp879 - self.fRec201[1]) / fTemp666;
																																																																																																																																																																																									fTemp881 = self.fRec200[1] + self.fConst12 * fTemp880;
																																																																																																																																																																																									self.fRec200[0] = 2.0 * (fTemp881 / fTemp668) - self.fRec200[1];
																																																																																																																																																																																									self.fRec201[0] = 2.0 * (self.fRec201[1] + self.fConst6 * (fTemp667 * (3.1415927 * self.fRec200[1] + self.fConst13 * fTemp880) / fTemp812)) - self.fRec201[1];
																																																																																																																																																																																									fRec202 = 0.6666667 * (fTemp881 / fTemp813);
																																																																																																																																																																																									fTemp882 = fSlow14 * fRec202 * fTemp651;
																																																																																																																																																																																								}
																																																																																																																																																																																									let mut fTemp883: F32 = fTemp879 + fSlow62 * fTemp882;
																																																																																																																																																																																									if iSlow21 != 0 {
																																																																																																																																																																																										fTemp884 = fTemp602 * (fTemp871 + fTemp883 - self.fRec198[1]) / fTemp601;
																																																																																																																																																																																										fTemp885 = self.fRec197[1] + self.fConst12 * fTemp884;
																																																																																																																																																																																										self.fRec197[0] = 2.0 * (fTemp885 / fTemp603) - self.fRec197[1];
																																																																																																																																																																																										self.fRec198[0] = 2.0 * (self.fRec198[1] + self.fConst6 * (fTemp602 * (3.1415927 * self.fRec197[1] + self.fConst13 * fTemp884) / fTemp818)) - self.fRec198[1];
																																																																																																																																																																																										fRec199 = 0.6666667 * (fTemp885 / fTemp819);
																																																																																																																																																																																										fTemp886 = fSlow20 * fRec199 * fTemp586;
																																																																																																																																																																																									}
																																																																																																																																																																																										let mut fTemp887: F32 = fTemp883 + fSlow63 * fTemp886;
																																																																																																																																																																																										if iSlow39 != 0 {
																																																																																																																																																																																											fTemp888 = fTemp537 * (fTemp871 + fTemp887 - self.fRec195[1]) / fTemp536;
																																																																																																																																																																																											fTemp889 = self.fRec194[1] + self.fConst12 * fTemp888;
																																																																																																																																																																																											self.fRec194[0] = 2.0 * (fTemp889 / fTemp538) - self.fRec194[1];
																																																																																																																																																																																											self.fRec195[0] = 2.0 * (self.fRec195[1] + self.fConst6 * (fTemp537 * (3.1415927 * self.fRec194[1] + self.fConst13 * fTemp888) / fTemp824)) - self.fRec195[1];
																																																																																																																																																																																											fRec196 = 0.6666667 * (fTemp889 / fTemp825);
																																																																																																																																																																																											fTemp890 = fSlow38 * fRec196 * fTemp521;
																																																																																																																																																																																										}
																																																																																																																																																																																											let mut fTemp891: F32 = fTemp887 + fSlow64 * fTemp890;
																																																																																																																																																																																											if iSlow36 != 0 {
																																																																																																																																																																																												fTemp892 = fTemp472 * (fTemp871 + fTemp891 - self.fRec192[1]) / fTemp471;
																																																																																																																																																																																												fTemp893 = self.fRec191[1] + self.fConst12 * fTemp892;
																																																																																																																																																																																												self.fRec191[0] = 2.0 * (fTemp893 / fTemp473) - self.fRec191[1];
																																																																																																																																																																																												self.fRec192[0] = 2.0 * (self.fRec192[1] + self.fConst6 * (fTemp472 * (3.1415927 * self.fRec191[1] + self.fConst13 * fTemp892) / fTemp830)) - self.fRec192[1];
																																																																																																																																																																																												fRec193 = 0.6666667 * (fTemp893 / fTemp831);
																																																																																																																																																																																												fTemp894 = fSlow35 * fRec193 * fTemp456;
																																																																																																																																																																																											}
																																																																																																																																																																																												let mut fTemp895: F32 = fTemp891 + fSlow65 * fTemp894;
																																																																																																																																																																																												if iSlow33 != 0 {
																																																																																																																																																																																													fTemp896 = fTemp407 * (fTemp871 + fTemp895 - self.fRec189[1]) / fTemp406;
																																																																																																																																																																																													fTemp897 = self.fRec188[1] + self.fConst12 * fTemp896;
																																																																																																																																																																																													self.fRec188[0] = 2.0 * (fTemp897 / fTemp408) - self.fRec188[1];
																																																																																																																																																																																													self.fRec189[0] = 2.0 * (self.fRec189[1] + self.fConst6 * (fTemp407 * (3.1415927 * self.fRec188[1] + self.fConst13 * fTemp896) / fTemp836)) - self.fRec189[1];
																																																																																																																																																																																													fRec190 = 0.6666667 * (fTemp897 / fTemp837);
																																																																																																																																																																																													fTemp898 = fSlow32 * fRec190 * fTemp391;
																																																																																																																																																																																												}
																																																																																																																																																																																													let mut fTemp899: F32 = fTemp895 + fSlow66 * fTemp898;
																																																																																																																																																																																													if iSlow30 != 0 {
																																																																																																																																																																																														fTemp900 = fTemp342 * (fTemp871 + fTemp899 - self.fRec186[1]) / fTemp341;
																																																																																																																																																																																														fTemp901 = self.fRec185[1] + self.fConst12 * fTemp900;
																																																																																																																																																																																														self.fRec185[0] = 2.0 * (fTemp901 / fTemp343) - self.fRec185[1];
																																																																																																																																																																																														self.fRec186[0] = 2.0 * (self.fRec186[1] + self.fConst6 * (fTemp342 * (3.1415927 * self.fRec185[1] + self.fConst13 * fTemp900) / fTemp842)) - self.fRec186[1];
																																																																																																																																																																																														fRec187 = 0.6666667 * (fTemp901 / fTemp843);
																																																																																																																																																																																														fTemp902 = fSlow29 * fRec187 * fTemp326;
																																																																																																																																																																																													}
																																																																																																																																																																																														let mut fTemp903: F32 = fTemp899 + fSlow67 * fTemp902;
																																																																																																																																																																																														if iSlow27 != 0 {
																																																																																																																																																																																															fTemp904 = fTemp277 * (fTemp871 + fTemp903 - self.fRec183[1]) / fTemp276;
																																																																																																																																																																																															fTemp905 = self.fRec182[1] + self.fConst12 * fTemp904;
																																																																																																																																																																																															self.fRec182[0] = 2.0 * (fTemp905 / fTemp278) - self.fRec182[1];
																																																																																																																																																																																															self.fRec183[0] = 2.0 * (self.fRec183[1] + self.fConst6 * (fTemp277 * (3.1415927 * self.fRec182[1] + self.fConst13 * fTemp904) / fTemp848)) - self.fRec183[1];
																																																																																																																																																																																															fRec184 = 0.6666667 * (fTemp905 / fTemp849);
																																																																																																																																																																																															fTemp906 = fSlow26 * fRec184 * fTemp261;
																																																																																																																																																																																														}
																																																																																																																																																																																															let mut fTemp907: F32 = fTemp903 + fSlow68 * fTemp906;
																																																																																																																																																																																															if iSlow24 != 0 {
																																																																																																																																																																																																fTemp908 = fTemp212 * (fTemp871 + fTemp907 - self.fRec180[1]) / fTemp211;
																																																																																																																																																																																																fTemp909 = self.fRec179[1] + self.fConst12 * fTemp908;
																																																																																																																																																																																																self.fRec179[0] = 2.0 * (fTemp909 / fTemp213) - self.fRec179[1];
																																																																																																																																																																																																self.fRec180[0] = 2.0 * (self.fRec180[1] + self.fConst6 * (fTemp212 * (3.1415927 * self.fRec179[1] + self.fConst13 * fTemp908) / fTemp854)) - self.fRec180[1];
																																																																																																																																																																																																fRec181 = 0.6666667 * (fTemp909 / fTemp855);
																																																																																																																																																																																																fTemp910 = fSlow23 * fRec181 * fTemp196;
																																																																																																																																																																																															}
																																																																																																																																																																																																let mut fTemp911: F32 = fTemp907 + fSlow69 * fTemp910;
																																																																																																																																																																																																if iSlow9 != 0 {
																																																																																																																																																																																																	fTemp912 = fTemp147 * (fTemp871 + fTemp911 - self.fRec177[1]) / fTemp146;
																																																																																																																																																																																																	fTemp913 = self.fRec176[1] + self.fConst12 * fTemp912;
																																																																																																																																																																																																	self.fRec176[0] = 2.0 * (fTemp913 / fTemp148) - self.fRec176[1];
																																																																																																																																																																																																	self.fRec177[0] = 2.0 * (self.fRec177[1] + self.fConst6 * (fTemp147 * (3.1415927 * self.fRec176[1] + self.fConst13 * fTemp912) / fTemp860)) - self.fRec177[1];
																																																																																																																																																																																																	fRec178 = 0.6666667 * (fTemp913 / fTemp861);
																																																																																																																																																																																																	fTemp914 = fSlow8 * fRec178 * fTemp131;
																																																																																																																																																																																																}
																																																																																																																																																																																																	let mut fTemp915: F32 = fTemp911 + fSlow70 * fTemp914;
																																																																																																																																																																																																	if iSlow5 != 0 {
																																																																																																																																																																																																		fTemp916 = fTemp81 * (fTemp871 + fTemp915 - self.fRec174[1]) / fTemp80;
																																																																																																																																																																																																		fTemp917 = self.fRec173[1] + self.fConst12 * fTemp916;
																																																																																																																																																																																																		self.fRec173[0] = 2.0 * (fTemp917 / fTemp83) - self.fRec173[1];
																																																																																																																																																																																																		self.fRec174[0] = 2.0 * (self.fRec174[1] + self.fConst6 * (fTemp81 * (3.1415927 * self.fRec173[1] + self.fConst13 * fTemp916) / fTemp866)) - self.fRec174[1];
																																																																																																																																																																																																		fRec175 = 0.6666667 * (fTemp917 / fTemp867);
																																																																																																																																																																																																		fTemp918 = fSlow4 * fRec175 * fTemp63;
																																																																																																																																																																																																	}
																																																																																																																																																																																																		let mut fTemp919: F32 = fTemp871 + fTemp915 + fSlow71 * fTemp918;
																																																																																																																																																																																																		self.fRec209[0] = self.fConst15 * AlternativesDsp_faustpower2_f(fTemp919) + self.fConst14 * self.fRec209[1];
																																																																																																																																																																																																		self.fRec210[0] = self.fConst15 * AlternativesDsp_faustpower2_f(fTemp871) + self.fConst14 * self.fRec210[1];
																																																																																																																																																																																																		*output1 = fTemp2 * (fTemp871 + fTemp870 * (if iSlow1 != 0 {F32::sqrt((self.fRec210[0] + 1e-09) / (self.fRec209[0] + 1e-09)) * fTemp919} else {fTemp919} - fTemp871));
																																																																																																																																																																																																		self.iVec0[1] = self.iVec0[0];
																																																																																																																																																																																																		self.fRec0[1] = self.fRec0[0];
																																																																																																																																																																																																		if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
																																																																																																																																																																																																			self.fRec1[1] = self.fRec1[0];
																																																																																																																																																																																																			self.fRec4[1] = self.fRec4[0];
																																																																																																																																																																																																			self.iRec5[1] = self.iRec5[0];
																																																																																																																																																																																																			self.fRec3[1] = self.fRec3[0];
																																																																																																																																																																																																			self.fRec2[1] = self.fRec2[0];
																																																																																																																																																																																																			self.fRec6[1] = self.fRec6[0];
																																																																																																																																																																																																			self.fRec12[1] = self.fRec12[0];
																																																																																																																																																																																																			self.iRec13[1] = self.iRec13[0];
																																																																																																																																																																																																			self.fRec14[1] = self.fRec14[0];
																																																																																																																																																																																																		}
																																																																																																																																																																																																			if iSlow5 != 0 {
																																																																																																																																																																																																				self.fRec11[1] = self.fRec11[0];
																																																																																																																																																																																																			}
																																																																																																																																																																																																				if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
																																																																																																																																																																																																					self.fRec15[1] = self.fRec15[0];
																																																																																																																																																																																																				}
																																																																																																																																																																																																					if iSlow5 != 0 {
																																																																																																																																																																																																						self.fRec9[1] = self.fRec9[0];
																																																																																																																																																																																																						self.fRec10[1] = self.fRec10[0];
																																																																																																																																																																																																						self.fRec16[1] = self.fRec16[0];
																																																																																																																																																																																																						self.fVec1[1] = self.fVec1[0];
																																																																																																																																																																																																						self.fRec8[1] = self.fRec8[0];
																																																																																																																																																																																																						self.fRec7[1] = self.fRec7[0];
																																																																																																																																																																																																						self.fRec17[1] = self.fRec17[0];
																																																																																																																																																																																																					}
																																																																																																																																																																																																						if iSlow39 | (iSlow36 | (iSlow33 | (iSlow30 | (iSlow27 | (iSlow24 | (iSlow21 | (iSlow18 | (iSlow15 | (iSlow12 | (iSlow9 | iSlow5)))))))))) != 0 {
																																																																																																																																																																																																							self.fRec18[1] = self.fRec18[0];
																																																																																																																																																																																																							self.fRec19[1] = self.fRec19[0];
																																																																																																																																																																																																							self.fRec23[1] = self.fRec23[0];
																																																																																																																																																																																																						}
																																																																																																																																																																																																							if iSlow5 != 0 {
																																																																																																																																																																																																								self.fRec25[1] = self.fRec25[0];
																																																																																																																																																																																																								self.fRec24[1] = self.fRec24[0];
																																																																																																																																																																																																								self.fRec26[1] = self.fRec26[0];
																																																																																																																																																																																																							}
																																																																																																																																																																																																								if iSlow9 != 0 {
																																																																																																																																																																																																									self.fRec31[1] = self.fRec31[0];
																																																																																																																																																																																																									self.fRec29[1] = self.fRec29[0];
																																																																																																																																																																																																									self.fRec30[1] = self.fRec30[0];
																																																																																																																																																																																																									self.fRec32[1] = self.fRec32[0];
																																																																																																																																																																																																									self.fVec2[1] = self.fVec2[0];
																																																																																																																																																																																																									self.fRec28[1] = self.fRec28[0];
																																																																																																																																																																																																									self.fRec27[1] = self.fRec27[0];
																																																																																																																																																																																																									self.fRec33[1] = self.fRec33[0];
																																																																																																																																																																																																									self.fRec38[1] = self.fRec38[0];
																																																																																																																																																																																																									self.fRec37[1] = self.fRec37[0];
																																																																																																																																																																																																									self.fRec39[1] = self.fRec39[0];
																																																																																																																																																																																																								}
																																																																																																																																																																																																									if iSlow24 != 0 {
																																																																																																																																																																																																										self.fRec44[1] = self.fRec44[0];
																																																																																																																																																																																																										self.fRec42[1] = self.fRec42[0];
																																																																																																																																																																																																										self.fRec43[1] = self.fRec43[0];
																																																																																																																																																																																																										self.fRec45[1] = self.fRec45[0];
																																																																																																																																																																																																										self.fVec3[1] = self.fVec3[0];
																																																																																																																																																																																																										self.fRec41[1] = self.fRec41[0];
																																																																																																																																																																																																										self.fRec40[1] = self.fRec40[0];
																																																																																																																																																																																																										self.fRec46[1] = self.fRec46[0];
																																																																																																																																																																																																										self.fRec51[1] = self.fRec51[0];
																																																																																																																																																																																																										self.fRec50[1] = self.fRec50[0];
																																																																																																																																																																																																										self.fRec52[1] = self.fRec52[0];
																																																																																																																																																																																																									}
																																																																																																																																																																																																										if iSlow27 != 0 {
																																																																																																																																																																																																											self.fRec57[1] = self.fRec57[0];
																																																																																																																																																																																																											self.fRec55[1] = self.fRec55[0];
																																																																																																																																																																																																											self.fRec56[1] = self.fRec56[0];
																																																																																																																																																																																																											self.fRec58[1] = self.fRec58[0];
																																																																																																																																																																																																											self.fVec4[1] = self.fVec4[0];
																																																																																																																																																																																																											self.fRec54[1] = self.fRec54[0];
																																																																																																																																																																																																											self.fRec53[1] = self.fRec53[0];
																																																																																																																																																																																																											self.fRec59[1] = self.fRec59[0];
																																																																																																																																																																																																											self.fRec64[1] = self.fRec64[0];
																																																																																																																																																																																																											self.fRec63[1] = self.fRec63[0];
																																																																																																																																																																																																											self.fRec65[1] = self.fRec65[0];
																																																																																																																																																																																																										}
																																																																																																																																																																																																											if iSlow30 != 0 {
																																																																																																																																																																																																												self.fRec70[1] = self.fRec70[0];
																																																																																																																																																																																																												self.fRec68[1] = self.fRec68[0];
																																																																																																																																																																																																												self.fRec69[1] = self.fRec69[0];
																																																																																																																																																																																																												self.fRec71[1] = self.fRec71[0];
																																																																																																																																																																																																												self.fVec5[1] = self.fVec5[0];
																																																																																																																																																																																																												self.fRec67[1] = self.fRec67[0];
																																																																																																																																																																																																												self.fRec66[1] = self.fRec66[0];
																																																																																																																																																																																																												self.fRec72[1] = self.fRec72[0];
																																																																																																																																																																																																												self.fRec77[1] = self.fRec77[0];
																																																																																																																																																																																																												self.fRec76[1] = self.fRec76[0];
																																																																																																																																																																																																												self.fRec78[1] = self.fRec78[0];
																																																																																																																																																																																																											}
																																																																																																																																																																																																												if iSlow33 != 0 {
																																																																																																																																																																																																													self.fRec83[1] = self.fRec83[0];
																																																																																																																																																																																																													self.fRec81[1] = self.fRec81[0];
																																																																																																																																																																																																													self.fRec82[1] = self.fRec82[0];
																																																																																																																																																																																																													self.fRec84[1] = self.fRec84[0];
																																																																																																																																																																																																													self.fVec6[1] = self.fVec6[0];
																																																																																																																																																																																																													self.fRec80[1] = self.fRec80[0];
																																																																																																																																																																																																													self.fRec79[1] = self.fRec79[0];
																																																																																																																																																																																																													self.fRec85[1] = self.fRec85[0];
																																																																																																																																																																																																													self.fRec90[1] = self.fRec90[0];
																																																																																																																																																																																																													self.fRec89[1] = self.fRec89[0];
																																																																																																																																																																																																													self.fRec91[1] = self.fRec91[0];
																																																																																																																																																																																																												}
																																																																																																																																																																																																													if iSlow36 != 0 {
																																																																																																																																																																																																														self.fRec96[1] = self.fRec96[0];
																																																																																																																																																																																																														self.fRec94[1] = self.fRec94[0];
																																																																																																																																																																																																														self.fRec95[1] = self.fRec95[0];
																																																																																																																																																																																																														self.fRec97[1] = self.fRec97[0];
																																																																																																																																																																																																														self.fVec7[1] = self.fVec7[0];
																																																																																																																																																																																																														self.fRec93[1] = self.fRec93[0];
																																																																																																																																																																																																														self.fRec92[1] = self.fRec92[0];
																																																																																																																																																																																																														self.fRec98[1] = self.fRec98[0];
																																																																																																																																																																																																														self.fRec103[1] = self.fRec103[0];
																																																																																																																																																																																																														self.fRec102[1] = self.fRec102[0];
																																																																																																																																																																																																														self.fRec104[1] = self.fRec104[0];
																																																																																																																																																																																																													}
																																																																																																																																																																																																														if iSlow39 != 0 {
																																																																																																																																																																																																															self.fRec109[1] = self.fRec109[0];
																																																																																																																																																																																																															self.fRec107[1] = self.fRec107[0];
																																																																																																																																																																																																															self.fRec108[1] = self.fRec108[0];
																																																																																																																																																																																																															self.fRec110[1] = self.fRec110[0];
																																																																																																																																																																																																															self.fVec8[1] = self.fVec8[0];
																																																																																																																																																																																																															self.fRec106[1] = self.fRec106[0];
																																																																																																																																																																																																															self.fRec105[1] = self.fRec105[0];
																																																																																																																																																																																																															self.fRec111[1] = self.fRec111[0];
																																																																																																																																																																																																															self.fRec116[1] = self.fRec116[0];
																																																																																																																																																																																																															self.fRec115[1] = self.fRec115[0];
																																																																																																																																																																																																															self.fRec117[1] = self.fRec117[0];
																																																																																																																																																																																																														}
																																																																																																																																																																																																															if iSlow21 != 0 {
																																																																																																																																																																																																																self.fRec122[1] = self.fRec122[0];
																																																																																																																																																																																																																self.fRec120[1] = self.fRec120[0];
																																																																																																																																																																																																																self.fRec121[1] = self.fRec121[0];
																																																																																																																																																																																																																self.fRec123[1] = self.fRec123[0];
																																																																																																																																																																																																																self.fVec9[1] = self.fVec9[0];
																																																																																																																																																																																																																self.fRec119[1] = self.fRec119[0];
																																																																																																																																																																																																																self.fRec118[1] = self.fRec118[0];
																																																																																																																																																																																																																self.fRec124[1] = self.fRec124[0];
																																																																																																																																																																																																																self.fRec129[1] = self.fRec129[0];
																																																																																																																																																																																																																self.fRec128[1] = self.fRec128[0];
																																																																																																																																																																																																																self.fRec130[1] = self.fRec130[0];
																																																																																																																																																																																																															}
																																																																																																																																																																																																																if iSlow15 != 0 {
																																																																																																																																																																																																																	self.fRec135[1] = self.fRec135[0];
																																																																																																																																																																																																																	self.fRec133[1] = self.fRec133[0];
																																																																																																																																																																																																																	self.fRec134[1] = self.fRec134[0];
																																																																																																																																																																																																																	self.fRec136[1] = self.fRec136[0];
																																																																																																																																																																																																																	self.fVec10[1] = self.fVec10[0];
																																																																																																																																																																																																																	self.fRec132[1] = self.fRec132[0];
																																																																																																																																																																																																																	self.fRec131[1] = self.fRec131[0];
																																																																																																																																																																																																																	self.fRec137[1] = self.fRec137[0];
																																																																																																																																																																																																																	self.fRec142[1] = self.fRec142[0];
																																																																																																																																																																																																																	self.fRec141[1] = self.fRec141[0];
																																																																																																																																																																																																																	self.fRec143[1] = self.fRec143[0];
																																																																																																																																																																																																																}
																																																																																																																																																																																																																	if iSlow18 != 0 {
																																																																																																																																																																																																																		self.fRec148[1] = self.fRec148[0];
																																																																																																																																																																																																																		self.fRec146[1] = self.fRec146[0];
																																																																																																																																																																																																																		self.fRec147[1] = self.fRec147[0];
																																																																																																																																																																																																																		self.fRec149[1] = self.fRec149[0];
																																																																																																																																																																																																																		self.fVec11[1] = self.fVec11[0];
																																																																																																																																																																																																																		self.fRec145[1] = self.fRec145[0];
																																																																																																																																																																																																																		self.fRec144[1] = self.fRec144[0];
																																																																																																																																																																																																																		self.fRec150[1] = self.fRec150[0];
																																																																																																																																																																																																																		self.fRec155[1] = self.fRec155[0];
																																																																																																																																																																																																																		self.fRec154[1] = self.fRec154[0];
																																																																																																																																																																																																																		self.fRec156[1] = self.fRec156[0];
																																																																																																																																																																																																																	}
																																																																																																																																																																																																																		if iSlow12 != 0 {
																																																																																																																																																																																																																			self.fRec161[1] = self.fRec161[0];
																																																																																																																																																																																																																			self.fRec159[1] = self.fRec159[0];
																																																																																																																																																																																																																			self.fRec160[1] = self.fRec160[0];
																																																																																																																																																																																																																			self.fRec162[1] = self.fRec162[0];
																																																																																																																																																																																																																			self.fVec12[1] = self.fVec12[0];
																																																																																																																																																																																																																			self.fRec158[1] = self.fRec158[0];
																																																																																																																																																																																																																			self.fRec157[1] = self.fRec157[0];
																																																																																																																																																																																																																			self.fRec163[1] = self.fRec163[0];
																																																																																																																																																																																																																			self.fRec168[1] = self.fRec168[0];
																																																																																																																																																																																																																			self.fRec167[1] = self.fRec167[0];
																																																																																																																																																																																																																			self.fRec169[1] = self.fRec169[0];
																																																																																																																																																																																																																			self.fRec164[1] = self.fRec164[0];
																																																																																																																																																																																																																			self.fRec165[1] = self.fRec165[0];
																																																																																																																																																																																																																		}
																																																																																																																																																																																																																			if iSlow18 != 0 {
																																																																																																																																																																																																																				self.fRec151[1] = self.fRec151[0];
																																																																																																																																																																																																																				self.fRec152[1] = self.fRec152[0];
																																																																																																																																																																																																																			}
																																																																																																																																																																																																																				if iSlow15 != 0 {
																																																																																																																																																																																																																					self.fRec138[1] = self.fRec138[0];
																																																																																																																																																																																																																					self.fRec139[1] = self.fRec139[0];
																																																																																																																																																																																																																				}
																																																																																																																																																																																																																					if iSlow21 != 0 {
																																																																																																																																																																																																																						self.fRec125[1] = self.fRec125[0];
																																																																																																																																																																																																																						self.fRec126[1] = self.fRec126[0];
																																																																																																																																																																																																																					}
																																																																																																																																																																																																																						if iSlow39 != 0 {
																																																																																																																																																																																																																							self.fRec112[1] = self.fRec112[0];
																																																																																																																																																																																																																							self.fRec113[1] = self.fRec113[0];
																																																																																																																																																																																																																						}
																																																																																																																																																																																																																							if iSlow36 != 0 {
																																																																																																																																																																																																																								self.fRec99[1] = self.fRec99[0];
																																																																																																																																																																																																																								self.fRec100[1] = self.fRec100[0];
																																																																																																																																																																																																																							}
																																																																																																																																																																																																																								if iSlow33 != 0 {
																																																																																																																																																																																																																									self.fRec86[1] = self.fRec86[0];
																																																																																																																																																																																																																									self.fRec87[1] = self.fRec87[0];
																																																																																																																																																																																																																								}
																																																																																																																																																																																																																									if iSlow30 != 0 {
																																																																																																																																																																																																																										self.fRec73[1] = self.fRec73[0];
																																																																																																																																																																																																																										self.fRec74[1] = self.fRec74[0];
																																																																																																																																																																																																																									}
																																																																																																																																																																																																																										if iSlow27 != 0 {
																																																																																																																																																																																																																											self.fRec60[1] = self.fRec60[0];
																																																																																																																																																																																																																											self.fRec61[1] = self.fRec61[0];
																																																																																																																																																																																																																										}
																																																																																																																																																																																																																											if iSlow24 != 0 {
																																																																																																																																																																																																																												self.fRec47[1] = self.fRec47[0];
																																																																																																																																																																																																																												self.fRec48[1] = self.fRec48[0];
																																																																																																																																																																																																																											}
																																																																																																																																																																																																																												if iSlow9 != 0 {
																																																																																																																																																																																																																													self.fRec34[1] = self.fRec34[0];
																																																																																																																																																																																																																													self.fRec35[1] = self.fRec35[0];
																																																																																																																																																																																																																												}
																																																																																																																																																																																																																													if iSlow5 != 0 {
																																																																																																																																																																																																																														self.fRec20[1] = self.fRec20[0];
																																																																																																																																																																																																																														self.fRec21[1] = self.fRec21[0];
																																																																																																																																																																																																																													}
																																																																																																																																																																																																																														self.fRec170[1] = self.fRec170[0];
																																																																																																																																																																																																																														self.fRec171[1] = self.fRec171[0];
																																																																																																																																																																																																																														self.fRec172[1] = self.fRec172[0];
																																																																																																																																																																																																																														if iSlow12 != 0 {
																																																																																																																																																																																																																															self.fRec206[1] = self.fRec206[0];
																																																																																																																																																																																																																															self.fRec207[1] = self.fRec207[0];
																																																																																																																																																																																																																														}
																																																																																																																																																																																																																															if iSlow18 != 0 {
																																																																																																																																																																																																																																self.fRec203[1] = self.fRec203[0];
																																																																																																																																																																																																																																self.fRec204[1] = self.fRec204[0];
																																																																																																																																																																																																																															}
																																																																																																																																																																																																																																if iSlow15 != 0 {
																																																																																																																																																																																																																																	self.fRec200[1] = self.fRec200[0];
																																																																																																																																																																																																																																	self.fRec201[1] = self.fRec201[0];
																																																																																																																																																																																																																																}
																																																																																																																																																																																																																																	if iSlow21 != 0 {
																																																																																																																																																																																																																																		self.fRec197[1] = self.fRec197[0];
																																																																																																																																																																																																																																		self.fRec198[1] = self.fRec198[0];
																																																																																																																																																																																																																																	}
																																																																																																																																																																																																																																		if iSlow39 != 0 {
																																																																																																																																																																																																																																			self.fRec194[1] = self.fRec194[0];
																																																																																																																																																																																																																																			self.fRec195[1] = self.fRec195[0];
																																																																																																																																																																																																																																		}
																																																																																																																																																																																																																																			if iSlow36 != 0 {
																																																																																																																																																																																																																																				self.fRec191[1] = self.fRec191[0];
																																																																																																																																																																																																																																				self.fRec192[1] = self.fRec192[0];
																																																																																																																																																																																																																																			}
																																																																																																																																																																																																																																				if iSlow33 != 0 {
																																																																																																																																																																																																																																					self.fRec188[1] = self.fRec188[0];
																																																																																																																																																																																																																																					self.fRec189[1] = self.fRec189[0];
																																																																																																																																																																																																																																				}
																																																																																																																																																																																																																																					if iSlow30 != 0 {
																																																																																																																																																																																																																																						self.fRec185[1] = self.fRec185[0];
																																																																																																																																																																																																																																						self.fRec186[1] = self.fRec186[0];
																																																																																																																																																																																																																																					}
																																																																																																																																																																																																																																						if iSlow27 != 0 {
																																																																																																																																																																																																																																							self.fRec182[1] = self.fRec182[0];
																																																																																																																																																																																																																																							self.fRec183[1] = self.fRec183[0];
																																																																																																																																																																																																																																						}
																																																																																																																																																																																																																																							if iSlow24 != 0 {
																																																																																																																																																																																																																																								self.fRec179[1] = self.fRec179[0];
																																																																																																																																																																																																																																								self.fRec180[1] = self.fRec180[0];
																																																																																																																																																																																																																																							}
																																																																																																																																																																																																																																								if iSlow9 != 0 {
																																																																																																																																																																																																																																									self.fRec176[1] = self.fRec176[0];
																																																																																																																																																																																																																																									self.fRec177[1] = self.fRec177[0];
																																																																																																																																																																																																																																								}
																																																																																																																																																																																																																																									if iSlow5 != 0 {
																																																																																																																																																																																																																																										self.fRec173[1] = self.fRec173[0];
																																																																																																																																																																																																																																										self.fRec174[1] = self.fRec174[0];
																																																																																																																																																																																																																																									}
																																																																																																																																																																																																																																										self.fRec209[1] = self.fRec209[0];
																																																																																																																																																																																																																																										self.fRec210[1] = self.fRec210[0];
																																																																																																																																																																																																																																									}
																																																																																																																																																																																																																																									self.iTempPerm0 = iTemp4;
																																																																																																																																																																																																																																									self.iTempPerm1 = iTemp5;
																																																																																																																																																																																																																																									self.iTempPerm2 = iTemp6;
																																																																																																																																																																																																																																									self.fTempPerm3 = fTemp7;
																																																																																																																																																																																																																																									self.iTempPerm4 = iTemp8;
																																																																																																																																																																																																																																									self.iTempPerm5 = iTemp9;
																																																																																																																																																																																																																																									self.fTempPerm6 = fTemp10;
																																																																																																																																																																																																																																									self.iTempPerm7 = iTemp11;
																																																																																																																																																																																																																																									self.iTempPerm8 = iTemp12;
																																																																																																																																																																																																																																									self.iTempPerm9 = iTemp13;
																																																																																																																																																																																																																																									self.fTempPerm10 = fTemp14;
																																																																																																																																																																																																																																									self.iTempPerm11 = iTemp15;
																																																																																																																																																																																																																																									self.iTempPerm12 = iTemp16;
																																																																																																																																																																																																																																									self.fTempPerm13 = fTemp17;
																																																																																																																																																																																																																																									self.fTempPerm14 = fTemp18;
																																																																																																																																																																																																																																									self.iTempPerm15 = iTemp19;
																																																																																																																																																																																																																																									self.iTempPerm16 = iTemp20;
																																																																																																																																																																																																																																									self.fTempPerm17 = fTemp21;
																																																																																																																																																																																																																																									self.fTempPerm18 = fTemp22;
																																																																																																																																																																																																																																									self.iTempPerm19 = iTemp23;
																																																																																																																																																																																																																																									self.fTempPerm20 = fTemp24;
																																																																																																																																																																																																																																									self.fTempPerm21 = fTemp25;
																																																																																																																																																																																																																																									self.fTempPerm22 = fTemp26;
																																																																																																																																																																																																																																									self.fTempPerm23 = fTemp27;
																																																																																																																																																																																																																																									self.fTempPerm24 = fTemp28;
																																																																																																																																																																																																																																									self.iTempPerm25 = iTemp29;
																																																																																																																																																																																																																																									self.iTempPerm26 = iTemp30;
																																																																																																																																																																																																																																									self.fTempPerm27 = fTemp31;
																																																																																																																																																																																																																																									self.fTempPerm28 = fTemp32;
																																																																																																																																																																																																																																									self.fTempPerm29 = fTemp33;
																																																																																																																																																																																																																																									self.fTempPerm30 = fTemp34;
																																																																																																																																																																																																																																									self.fTempPerm31 = fTemp35;
																																																																																																																																																																																																																																									self.fTempPerm32 = fTemp36;
																																																																																																																																																																																																																																									self.iTempPerm33 = iTemp37;
																																																																																																																																																																																																																																									self.fTempPerm34 = fTemp38;
																																																																																																																																																																																																																																									self.fTempPerm35 = fTemp39;
																																																																																																																																																																																																																																									self.fTempPerm36 = fTemp40;
																																																																																																																																																																																																																																									self.iTempPerm37 = iTemp41;
																																																																																																																																																																																																																																									self.fTempPerm38 = fTemp42;
																																																																																																																																																																																																																																									self.fTempPerm39 = fTemp43;
																																																																																																																																																																																																																																									self.fTempPerm40 = fTemp44;
																																																																																																																																																																																																																																									self.fTempPerm41 = fTemp45;
																																																																																																																																																																																																																																									self.iTempPerm42 = iTemp46;
																																																																																																																																																																																																																																									self.iTempPerm43 = iTemp47;
																																																																																																																																																																																																																																									self.iTempPerm44 = iTemp48;
																																																																																																																																																																																																																																									self.fTempPerm45 = fTemp49;
																																																																																																																																																																																																																																									self.fTempPerm46 = fTemp50;
																																																																																																																																																																																																																																									self.iTempPerm47 = iTemp51;
																																																																																																																																																																																																																																									self.iTempPerm48 = iTemp52;
																																																																																																																																																																																																																																									self.fTempPerm49 = fTemp53;
																																																																																																																																																																																																																																									self.fTempPerm50 = fTemp54;
																																																																																																																																																																																																																																									self.fTempPerm51 = fTemp55;
																																																																																																																																																																																																																																									self.fTempPerm52 = fTemp56;
																																																																																																																																																																																																																																									self.fTempPerm53 = fTemp57;
																																																																																																																																																																																																																																									self.fTempPerm54 = fTemp58;
																																																																																																																																																																																																																																									self.fTempPerm55 = fTemp59;
																																																																																																																																																																																																																																									self.fTempPerm56 = fTemp60;
																																																																																																																																																																																																																																									self.iTempPerm57 = iTemp61;
																																																																																																																																																																																																																																									self.fTempPerm58 = fTemp62;
																																																																																																																																																																																																																																									self.fTempPerm59 = fTemp63;
																																																																																																																																																																																																																																									self.fTempPerm60 = fTemp64;
																																																																																																																																																																																																																																									self.fTempPerm61 = fTemp65;
																																																																																																																																																																																																																																									self.iTempPerm62 = iTemp66;
																																																																																																																																																																																																																																									self.iTempPerm63 = iTemp67;
																																																																																																																																																																																																																																									self.fTempPerm64 = fTemp68;
																																																																																																																																																																																																																																									self.fTempPerm65 = fTemp69;
																																																																																																																																																																																																																																									self.fTempPerm66 = fTemp70;
																																																																																																																																																																																																																																									self.iTempPerm67 = iTemp71;
																																																																																																																																																																																																																																									self.fTempPerm68 = fTemp72;
																																																																																																																																																																																																																																									self.fTempPerm69 = fTemp73;
																																																																																																																																																																																																																																									self.fTempPerm70 = fTemp74;
																																																																																																																																																																																																																																									self.iTempPerm71 = iTemp75;
																																																																																																																																																																																																																																									self.fTempPerm72 = fTemp76;
																																																																																																																																																																																																																																									self.fTempPerm73 = fTemp77;
																																																																																																																																																																																																																																									self.fTempPerm74 = fTemp78;
																																																																																																																																																																																																																																									self.fTempPerm75 = fTemp79;
																																																																																																																																																																																																																																									self.fTempPerm76 = fTemp80;
																																																																																																																																																																																																																																									self.fTempPerm77 = fTemp81;
																																																																																																																																																																																																																																									self.fTempPerm78 = fTemp82;
																																																																																																																																																																																																																																									self.fTempPerm79 = fTemp83;
																																																																																																																																																																																																																																									self.iTempPerm80 = iTemp84;
																																																																																																																																																																																																																																									self.iTempPerm81 = iTemp85;
																																																																																																																																																																																																																																									self.iTempPerm82 = iTemp86;
																																																																																																																																																																																																																																									self.iTempPerm83 = iTemp87;
																																																																																																																																																																																																																																									self.iTempPerm84 = iTemp88;
																																																																																																																																																																																																																																									self.fTempPerm85 = fTemp89;
																																																																																																																																																																																																																																									self.iTempPerm86 = iTemp90;
																																																																																																																																																																																																																																									self.iTempPerm87 = iTemp91;
																																																																																																																																																																																																																																									self.fTempPerm88 = fTemp92;
																																																																																																																																																																																																																																									self.fTempPerm89 = fTemp93;
																																																																																																																																																																																																																																									self.iTempPerm90 = iTemp94;
																																																																																																																																																																																																																																									self.iTempPerm91 = iTemp95;
																																																																																																																																																																																																																																									self.fTempPerm92 = fTemp96;
																																																																																																																																																																																																																																									self.fTempPerm93 = fTemp97;
																																																																																																																																																																																																																																									self.iTempPerm94 = iTemp98;
																																																																																																																																																																																																																																									self.fTempPerm95 = fTemp99;
																																																																																																																																																																																																																																									self.fTempPerm96 = fTemp100;
																																																																																																																																																																																																																																									self.iTempPerm97 = iTemp101;
																																																																																																																																																																																																																																									self.iTempPerm98 = iTemp102;
																																																																																																																																																																																																																																									self.fTempPerm99 = fTemp103;
																																																																																																																																																																																																																																									self.fTempPerm100 = fTemp104;
																																																																																																																																																																																																																																									self.fTempPerm101 = fTemp105;
																																																																																																																																																																																																																																									self.fTempPerm102 = fTemp106;
																																																																																																																																																																																																																																									self.fTempPerm103 = fTemp107;
																																																																																																																																																																																																																																									self.fTempPerm104 = fTemp108;
																																																																																																																																																																																																																																									self.iTempPerm105 = iTemp109;
																																																																																																																																																																																																																																									self.fTempPerm106 = fTemp110;
																																																																																																																																																																																																																																									self.fTempPerm107 = fTemp111;
																																																																																																																																																																																																																																									self.fTempPerm108 = fTemp112;
																																																																																																																																																																																																																																									self.iTempPerm109 = iTemp113;
																																																																																																																																																																																																																																									self.fTempPerm110 = fTemp114;
																																																																																																																																																																																																																																									self.fTempPerm111 = fTemp115;
																																																																																																																																																																																																																																									self.fTempPerm112 = fTemp116;
																																																																																																																																																																																																																																									self.fTempPerm113 = fTemp117;
																																																																																																																																																																																																																																									self.iTempPerm114 = iTemp118;
																																																																																																																																																																																																																																									self.iTempPerm115 = iTemp119;
																																																																																																																																																																																																																																									self.iTempPerm116 = iTemp120;
																																																																																																																																																																																																																																									self.fTempPerm117 = fTemp121;
																																																																																																																																																																																																																																									self.fTempPerm118 = fTemp122;
																																																																																																																																																																																																																																									self.iTempPerm119 = iTemp123;
																																																																																																																																																																																																																																									self.iTempPerm120 = iTemp124;
																																																																																																																																																																																																																																									self.fTempPerm121 = fTemp125;
																																																																																																																																																																																																																																									self.fTempPerm122 = fTemp126;
																																																																																																																																																																																																																																									self.fTempPerm123 = fTemp127;
																																																																																																																																																																																																																																									self.fTempPerm124 = fTemp128;
																																																																																																																																																																																																																																									self.iTempPerm125 = iTemp129;
																																																																																																																																																																																																																																									self.fTempPerm126 = fTemp130;
																																																																																																																																																																																																																																									self.fTempPerm127 = fTemp131;
																																																																																																																																																																																																																																									self.iTempPerm128 = iTemp132;
																																																																																																																																																																																																																																									self.iTempPerm129 = iTemp133;
																																																																																																																																																																																																																																									self.fTempPerm130 = fTemp134;
																																																																																																																																																																																																																																									self.fTempPerm131 = fTemp135;
																																																																																																																																																																																																																																									self.fTempPerm132 = fTemp136;
																																																																																																																																																																																																																																									self.iTempPerm133 = iTemp137;
																																																																																																																																																																																																																																									self.fTempPerm134 = fTemp138;
																																																																																																																																																																																																																																									self.fTempPerm135 = fTemp139;
																																																																																																																																																																																																																																									self.fTempPerm136 = fTemp140;
																																																																																																																																																																																																																																									self.iTempPerm137 = iTemp141;
																																																																																																																																																																																																																																									self.fTempPerm138 = fTemp142;
																																																																																																																																																																																																																																									self.fTempPerm139 = fTemp143;
																																																																																																																																																																																																																																									self.fTempPerm140 = fTemp144;
																																																																																																																																																																																																																																									self.fTempPerm141 = fTemp145;
																																																																																																																																																																																																																																									self.fTempPerm142 = fTemp146;
																																																																																																																																																																																																																																									self.fTempPerm143 = fTemp147;
																																																																																																																																																																																																																																									self.fTempPerm144 = fTemp148;
																																																																																																																																																																																																																																									self.iTempPerm145 = iTemp149;
																																																																																																																																																																																																																																									self.iTempPerm146 = iTemp150;
																																																																																																																																																																																																																																									self.iTempPerm147 = iTemp151;
																																																																																																																																																																																																																																									self.iTempPerm148 = iTemp152;
																																																																																																																																																																																																																																									self.iTempPerm149 = iTemp153;
																																																																																																																																																																																																																																									self.fTempPerm150 = fTemp154;
																																																																																																																																																																																																																																									self.iTempPerm151 = iTemp155;
																																																																																																																																																																																																																																									self.iTempPerm152 = iTemp156;
																																																																																																																																																																																																																																									self.fTempPerm153 = fTemp157;
																																																																																																																																																																																																																																									self.fTempPerm154 = fTemp158;
																																																																																																																																																																																																																																									self.iTempPerm155 = iTemp159;
																																																																																																																																																																																																																																									self.iTempPerm156 = iTemp160;
																																																																																																																																																																																																																																									self.fTempPerm157 = fTemp161;
																																																																																																																																																																																																																																									self.fTempPerm158 = fTemp162;
																																																																																																																																																																																																																																									self.iTempPerm159 = iTemp163;
																																																																																																																																																																																																																																									self.fTempPerm160 = fTemp164;
																																																																																																																																																																																																																																									self.fTempPerm161 = fTemp165;
																																																																																																																																																																																																																																									self.iTempPerm162 = iTemp166;
																																																																																																																																																																																																																																									self.iTempPerm163 = iTemp167;
																																																																																																																																																																																																																																									self.fTempPerm164 = fTemp168;
																																																																																																																																																																																																																																									self.fTempPerm165 = fTemp169;
																																																																																																																																																																																																																																									self.fTempPerm166 = fTemp170;
																																																																																																																																																																																																																																									self.fTempPerm167 = fTemp171;
																																																																																																																																																																																																																																									self.fTempPerm168 = fTemp172;
																																																																																																																																																																																																																																									self.fTempPerm169 = fTemp173;
																																																																																																																																																																																																																																									self.iTempPerm170 = iTemp174;
																																																																																																																																																																																																																																									self.fTempPerm171 = fTemp175;
																																																																																																																																																																																																																																									self.fTempPerm172 = fTemp176;
																																																																																																																																																																																																																																									self.fTempPerm173 = fTemp177;
																																																																																																																																																																																																																																									self.iTempPerm174 = iTemp178;
																																																																																																																																																																																																																																									self.fTempPerm175 = fTemp179;
																																																																																																																																																																																																																																									self.fTempPerm176 = fTemp180;
																																																																																																																																																																																																																																									self.fTempPerm177 = fTemp181;
																																																																																																																																																																																																																																									self.fTempPerm178 = fTemp182;
																																																																																																																																																																																																																																									self.iTempPerm179 = iTemp183;
																																																																																																																																																																																																																																									self.iTempPerm180 = iTemp184;
																																																																																																																																																																																																																																									self.iTempPerm181 = iTemp185;
																																																																																																																																																																																																																																									self.fTempPerm182 = fTemp186;
																																																																																																																																																																																																																																									self.fTempPerm183 = fTemp187;
																																																																																																																																																																																																																																									self.iTempPerm184 = iTemp188;
																																																																																																																																																																																																																																									self.iTempPerm185 = iTemp189;
																																																																																																																																																																																																																																									self.fTempPerm186 = fTemp190;
																																																																																																																																																																																																																																									self.fTempPerm187 = fTemp191;
																																																																																																																																																																																																																																									self.fTempPerm188 = fTemp192;
																																																																																																																																																																																																																																									self.fTempPerm189 = fTemp193;
																																																																																																																																																																																																																																									self.iTempPerm190 = iTemp194;
																																																																																																																																																																																																																																									self.fTempPerm191 = fTemp195;
																																																																																																																																																																																																																																									self.fTempPerm192 = fTemp196;
																																																																																																																																																																																																																																									self.iTempPerm193 = iTemp197;
																																																																																																																																																																																																																																									self.iTempPerm194 = iTemp198;
																																																																																																																																																																																																																																									self.fTempPerm195 = fTemp199;
																																																																																																																																																																																																																																									self.fTempPerm196 = fTemp200;
																																																																																																																																																																																																																																									self.fTempPerm197 = fTemp201;
																																																																																																																																																																																																																																									self.iTempPerm198 = iTemp202;
																																																																																																																																																																																																																																									self.fTempPerm199 = fTemp203;
																																																																																																																																																																																																																																									self.fTempPerm200 = fTemp204;
																																																																																																																																																																																																																																									self.fTempPerm201 = fTemp205;
																																																																																																																																																																																																																																									self.iTempPerm202 = iTemp206;
																																																																																																																																																																																																																																									self.fTempPerm203 = fTemp207;
																																																																																																																																																																																																																																									self.fTempPerm204 = fTemp208;
																																																																																																																																																																																																																																									self.fTempPerm205 = fTemp209;
																																																																																																																																																																																																																																									self.fTempPerm206 = fTemp210;
																																																																																																																																																																																																																																									self.fTempPerm207 = fTemp211;
																																																																																																																																																																																																																																									self.fTempPerm208 = fTemp212;
																																																																																																																																																																																																																																									self.fTempPerm209 = fTemp213;
																																																																																																																																																																																																																																									self.iTempPerm210 = iTemp214;
																																																																																																																																																																																																																																									self.iTempPerm211 = iTemp215;
																																																																																																																																																																																																																																									self.iTempPerm212 = iTemp216;
																																																																																																																																																																																																																																									self.iTempPerm213 = iTemp217;
																																																																																																																																																																																																																																									self.iTempPerm214 = iTemp218;
																																																																																																																																																																																																																																									self.fTempPerm215 = fTemp219;
																																																																																																																																																																																																																																									self.iTempPerm216 = iTemp220;
																																																																																																																																																																																																																																									self.iTempPerm217 = iTemp221;
																																																																																																																																																																																																																																									self.fTempPerm218 = fTemp222;
																																																																																																																																																																																																																																									self.fTempPerm219 = fTemp223;
																																																																																																																																																																																																																																									self.iTempPerm220 = iTemp224;
																																																																																																																																																																																																																																									self.iTempPerm221 = iTemp225;
																																																																																																																																																																																																																																									self.fTempPerm222 = fTemp226;
																																																																																																																																																																																																																																									self.fTempPerm223 = fTemp227;
																																																																																																																																																																																																																																									self.iTempPerm224 = iTemp228;
																																																																																																																																																																																																																																									self.fTempPerm225 = fTemp229;
																																																																																																																																																																																																																																									self.fTempPerm226 = fTemp230;
																																																																																																																																																																																																																																									self.iTempPerm227 = iTemp231;
																																																																																																																																																																																																																																									self.iTempPerm228 = iTemp232;
																																																																																																																																																																																																																																									self.fTempPerm229 = fTemp233;
																																																																																																																																																																																																																																									self.fTempPerm230 = fTemp234;
																																																																																																																																																																																																																																									self.fTempPerm231 = fTemp235;
																																																																																																																																																																																																																																									self.fTempPerm232 = fTemp236;
																																																																																																																																																																																																																																									self.fTempPerm233 = fTemp237;
																																																																																																																																																																																																																																									self.fTempPerm234 = fTemp238;
																																																																																																																																																																																																																																									self.iTempPerm235 = iTemp239;
																																																																																																																																																																																																																																									self.fTempPerm236 = fTemp240;
																																																																																																																																																																																																																																									self.fTempPerm237 = fTemp241;
																																																																																																																																																																																																																																									self.fTempPerm238 = fTemp242;
																																																																																																																																																																																																																																									self.iTempPerm239 = iTemp243;
																																																																																																																																																																																																																																									self.fTempPerm240 = fTemp244;
																																																																																																																																																																																																																																									self.fTempPerm241 = fTemp245;
																																																																																																																																																																																																																																									self.fTempPerm242 = fTemp246;
																																																																																																																																																																																																																																									self.fTempPerm243 = fTemp247;
																																																																																																																																																																																																																																									self.iTempPerm244 = iTemp248;
																																																																																																																																																																																																																																									self.iTempPerm245 = iTemp249;
																																																																																																																																																																																																																																									self.iTempPerm246 = iTemp250;
																																																																																																																																																																																																																																									self.fTempPerm247 = fTemp251;
																																																																																																																																																																																																																																									self.fTempPerm248 = fTemp252;
																																																																																																																																																																																																																																									self.iTempPerm249 = iTemp253;
																																																																																																																																																																																																																																									self.iTempPerm250 = iTemp254;
																																																																																																																																																																																																																																									self.fTempPerm251 = fTemp255;
																																																																																																																																																																																																																																									self.fTempPerm252 = fTemp256;
																																																																																																																																																																																																																																									self.fTempPerm253 = fTemp257;
																																																																																																																																																																																																																																									self.fTempPerm254 = fTemp258;
																																																																																																																																																																																																																																									self.iTempPerm255 = iTemp259;
																																																																																																																																																																																																																																									self.fTempPerm256 = fTemp260;
																																																																																																																																																																																																																																									self.fTempPerm257 = fTemp261;
																																																																																																																																																																																																																																									self.iTempPerm258 = iTemp262;
																																																																																																																																																																																																																																									self.iTempPerm259 = iTemp263;
																																																																																																																																																																																																																																									self.fTempPerm260 = fTemp264;
																																																																																																																																																																																																																																									self.fTempPerm261 = fTemp265;
																																																																																																																																																																																																																																									self.fTempPerm262 = fTemp266;
																																																																																																																																																																																																																																									self.iTempPerm263 = iTemp267;
																																																																																																																																																																																																																																									self.fTempPerm264 = fTemp268;
																																																																																																																																																																																																																																									self.fTempPerm265 = fTemp269;
																																																																																																																																																																																																																																									self.fTempPerm266 = fTemp270;
																																																																																																																																																																																																																																									self.iTempPerm267 = iTemp271;
																																																																																																																																																																																																																																									self.fTempPerm268 = fTemp272;
																																																																																																																																																																																																																																									self.fTempPerm269 = fTemp273;
																																																																																																																																																																																																																																									self.fTempPerm270 = fTemp274;
																																																																																																																																																																																																																																									self.fTempPerm271 = fTemp275;
																																																																																																																																																																																																																																									self.fTempPerm272 = fTemp276;
																																																																																																																																																																																																																																									self.fTempPerm273 = fTemp277;
																																																																																																																																																																																																																																									self.fTempPerm274 = fTemp278;
																																																																																																																																																																																																																																									self.iTempPerm275 = iTemp279;
																																																																																																																																																																																																																																									self.iTempPerm276 = iTemp280;
																																																																																																																																																																																																																																									self.iTempPerm277 = iTemp281;
																																																																																																																																																																																																																																									self.iTempPerm278 = iTemp282;
																																																																																																																																																																																																																																									self.iTempPerm279 = iTemp283;
																																																																																																																																																																																																																																									self.fTempPerm280 = fTemp284;
																																																																																																																																																																																																																																									self.iTempPerm281 = iTemp285;
																																																																																																																																																																																																																																									self.iTempPerm282 = iTemp286;
																																																																																																																																																																																																																																									self.fTempPerm283 = fTemp287;
																																																																																																																																																																																																																																									self.fTempPerm284 = fTemp288;
																																																																																																																																																																																																																																									self.iTempPerm285 = iTemp289;
																																																																																																																																																																																																																																									self.iTempPerm286 = iTemp290;
																																																																																																																																																																																																																																									self.fTempPerm287 = fTemp291;
																																																																																																																																																																																																																																									self.fTempPerm288 = fTemp292;
																																																																																																																																																																																																																																									self.iTempPerm289 = iTemp293;
																																																																																																																																																																																																																																									self.fTempPerm290 = fTemp294;
																																																																																																																																																																																																																																									self.fTempPerm291 = fTemp295;
																																																																																																																																																																																																																																									self.iTempPerm292 = iTemp296;
																																																																																																																																																																																																																																									self.iTempPerm293 = iTemp297;
																																																																																																																																																																																																																																									self.fTempPerm294 = fTemp298;
																																																																																																																																																																																																																																									self.fTempPerm295 = fTemp299;
																																																																																																																																																																																																																																									self.fTempPerm296 = fTemp300;
																																																																																																																																																																																																																																									self.fTempPerm297 = fTemp301;
																																																																																																																																																																																																																																									self.fTempPerm298 = fTemp302;
																																																																																																																																																																																																																																									self.fTempPerm299 = fTemp303;
																																																																																																																																																																																																																																									self.iTempPerm300 = iTemp304;
																																																																																																																																																																																																																																									self.fTempPerm301 = fTemp305;
																																																																																																																																																																																																																																									self.fTempPerm302 = fTemp306;
																																																																																																																																																																																																																																									self.fTempPerm303 = fTemp307;
																																																																																																																																																																																																																																									self.iTempPerm304 = iTemp308;
																																																																																																																																																																																																																																									self.fTempPerm305 = fTemp309;
																																																																																																																																																																																																																																									self.fTempPerm306 = fTemp310;
																																																																																																																																																																																																																																									self.fTempPerm307 = fTemp311;
																																																																																																																																																																																																																																									self.fTempPerm308 = fTemp312;
																																																																																																																																																																																																																																									self.iTempPerm309 = iTemp313;
																																																																																																																																																																																																																																									self.iTempPerm310 = iTemp314;
																																																																																																																																																																																																																																									self.iTempPerm311 = iTemp315;
																																																																																																																																																																																																																																									self.fTempPerm312 = fTemp316;
																																																																																																																																																																																																																																									self.fTempPerm313 = fTemp317;
																																																																																																																																																																																																																																									self.iTempPerm314 = iTemp318;
																																																																																																																																																																																																																																									self.iTempPerm315 = iTemp319;
																																																																																																																																																																																																																																									self.fTempPerm316 = fTemp320;
																																																																																																																																																																																																																																									self.fTempPerm317 = fTemp321;
																																																																																																																																																																																																																																									self.fTempPerm318 = fTemp322;
																																																																																																																																																																																																																																									self.fTempPerm319 = fTemp323;
																																																																																																																																																																																																																																									self.iTempPerm320 = iTemp324;
																																																																																																																																																																																																																																									self.fTempPerm321 = fTemp325;
																																																																																																																																																																																																																																									self.fTempPerm322 = fTemp326;
																																																																																																																																																																																																																																									self.iTempPerm323 = iTemp327;
																																																																																																																																																																																																																																									self.iTempPerm324 = iTemp328;
																																																																																																																																																																																																																																									self.fTempPerm325 = fTemp329;
																																																																																																																																																																																																																																									self.fTempPerm326 = fTemp330;
																																																																																																																																																																																																																																									self.fTempPerm327 = fTemp331;
																																																																																																																																																																																																																																									self.iTempPerm328 = iTemp332;
																																																																																																																																																																																																																																									self.fTempPerm329 = fTemp333;
																																																																																																																																																																																																																																									self.fTempPerm330 = fTemp334;
																																																																																																																																																																																																																																									self.fTempPerm331 = fTemp335;
																																																																																																																																																																																																																																									self.iTempPerm332 = iTemp336;
																																																																																																																																																																																																																																									self.fTempPerm333 = fTemp337;
																																																																																																																																																																																																																																									self.fTempPerm334 = fTemp338;
																																																																																																																																																																																																																																									self.fTempPerm335 = fTemp339;
																																																																																																																																																																																																																																									self.fTempPerm336 = fTemp340;
																																																																																																																																																																																																																																									self.fTempPerm337 = fTemp341;
																																																																																																																																																																																																																																									self.fTempPerm338 = fTemp342;
																																																																																																																																																																																																																																									self.fTempPerm339 = fTemp343;
																																																																																																																																																																																																																																									self.iTempPerm340 = iTemp344;
																																																																																																																																																																																																																																									self.iTempPerm341 = iTemp345;
																																																																																																																																																																																																																																									self.iTempPerm342 = iTemp346;
																																																																																																																																																																																																																																									self.iTempPerm343 = iTemp347;
																																																																																																																																																																																																																																									self.iTempPerm344 = iTemp348;
																																																																																																																																																																																																																																									self.fTempPerm345 = fTemp349;
																																																																																																																																																																																																																																									self.iTempPerm346 = iTemp350;
																																																																																																																																																																																																																																									self.iTempPerm347 = iTemp351;
																																																																																																																																																																																																																																									self.fTempPerm348 = fTemp352;
																																																																																																																																																																																																																																									self.fTempPerm349 = fTemp353;
																																																																																																																																																																																																																																									self.iTempPerm350 = iTemp354;
																																																																																																																																																																																																																																									self.iTempPerm351 = iTemp355;
																																																																																																																																																																																																																																									self.fTempPerm352 = fTemp356;
																																																																																																																																																																																																																																									self.fTempPerm353 = fTemp357;
																																																																																																																																																																																																																																									self.iTempPerm354 = iTemp358;
																																																																																																																																																																																																																																									self.fTempPerm355 = fTemp359;
																																																																																																																																																																																																																																									self.fTempPerm356 = fTemp360;
																																																																																																																																																																																																																																									self.iTempPerm357 = iTemp361;
																																																																																																																																																																																																																																									self.iTempPerm358 = iTemp362;
																																																																																																																																																																																																																																									self.fTempPerm359 = fTemp363;
																																																																																																																																																																																																																																									self.fTempPerm360 = fTemp364;
																																																																																																																																																																																																																																									self.fTempPerm361 = fTemp365;
																																																																																																																																																																																																																																									self.fTempPerm362 = fTemp366;
																																																																																																																																																																																																																																									self.fTempPerm363 = fTemp367;
																																																																																																																																																																																																																																									self.fTempPerm364 = fTemp368;
																																																																																																																																																																																																																																									self.iTempPerm365 = iTemp369;
																																																																																																																																																																																																																																									self.fTempPerm366 = fTemp370;
																																																																																																																																																																																																																																									self.fTempPerm367 = fTemp371;
																																																																																																																																																																																																																																									self.fTempPerm368 = fTemp372;
																																																																																																																																																																																																																																									self.iTempPerm369 = iTemp373;
																																																																																																																																																																																																																																									self.fTempPerm370 = fTemp374;
																																																																																																																																																																																																																																									self.fTempPerm371 = fTemp375;
																																																																																																																																																																																																																																									self.fTempPerm372 = fTemp376;
																																																																																																																																																																																																																																									self.fTempPerm373 = fTemp377;
																																																																																																																																																																																																																																									self.iTempPerm374 = iTemp378;
																																																																																																																																																																																																																																									self.iTempPerm375 = iTemp379;
																																																																																																																																																																																																																																									self.iTempPerm376 = iTemp380;
																																																																																																																																																																																																																																									self.fTempPerm377 = fTemp381;
																																																																																																																																																																																																																																									self.fTempPerm378 = fTemp382;
																																																																																																																																																																																																																																									self.iTempPerm379 = iTemp383;
																																																																																																																																																																																																																																									self.iTempPerm380 = iTemp384;
																																																																																																																																																																																																																																									self.fTempPerm381 = fTemp385;
																																																																																																																																																																																																																																									self.fTempPerm382 = fTemp386;
																																																																																																																																																																																																																																									self.fTempPerm383 = fTemp387;
																																																																																																																																																																																																																																									self.fTempPerm384 = fTemp388;
																																																																																																																																																																																																																																									self.iTempPerm385 = iTemp389;
																																																																																																																																																																																																																																									self.fTempPerm386 = fTemp390;
																																																																																																																																																																																																																																									self.fTempPerm387 = fTemp391;
																																																																																																																																																																																																																																									self.iTempPerm388 = iTemp392;
																																																																																																																																																																																																																																									self.iTempPerm389 = iTemp393;
																																																																																																																																																																																																																																									self.fTempPerm390 = fTemp394;
																																																																																																																																																																																																																																									self.fTempPerm391 = fTemp395;
																																																																																																																																																																																																																																									self.fTempPerm392 = fTemp396;
																																																																																																																																																																																																																																									self.iTempPerm393 = iTemp397;
																																																																																																																																																																																																																																									self.fTempPerm394 = fTemp398;
																																																																																																																																																																																																																																									self.fTempPerm395 = fTemp399;
																																																																																																																																																																																																																																									self.fTempPerm396 = fTemp400;
																																																																																																																																																																																																																																									self.iTempPerm397 = iTemp401;
																																																																																																																																																																																																																																									self.fTempPerm398 = fTemp402;
																																																																																																																																																																																																																																									self.fTempPerm399 = fTemp403;
																																																																																																																																																																																																																																									self.fTempPerm400 = fTemp404;
																																																																																																																																																																																																																																									self.fTempPerm401 = fTemp405;
																																																																																																																																																																																																																																									self.fTempPerm402 = fTemp406;
																																																																																																																																																																																																																																									self.fTempPerm403 = fTemp407;
																																																																																																																																																																																																																																									self.fTempPerm404 = fTemp408;
																																																																																																																																																																																																																																									self.iTempPerm405 = iTemp409;
																																																																																																																																																																																																																																									self.iTempPerm406 = iTemp410;
																																																																																																																																																																																																																																									self.iTempPerm407 = iTemp411;
																																																																																																																																																																																																																																									self.iTempPerm408 = iTemp412;
																																																																																																																																																																																																																																									self.iTempPerm409 = iTemp413;
																																																																																																																																																																																																																																									self.fTempPerm410 = fTemp414;
																																																																																																																																																																																																																																									self.iTempPerm411 = iTemp415;
																																																																																																																																																																																																																																									self.iTempPerm412 = iTemp416;
																																																																																																																																																																																																																																									self.fTempPerm413 = fTemp417;
																																																																																																																																																																																																																																									self.fTempPerm414 = fTemp418;
																																																																																																																																																																																																																																									self.iTempPerm415 = iTemp419;
																																																																																																																																																																																																																																									self.iTempPerm416 = iTemp420;
																																																																																																																																																																																																																																									self.fTempPerm417 = fTemp421;
																																																																																																																																																																																																																																									self.fTempPerm418 = fTemp422;
																																																																																																																																																																																																																																									self.iTempPerm419 = iTemp423;
																																																																																																																																																																																																																																									self.fTempPerm420 = fTemp424;
																																																																																																																																																																																																																																									self.fTempPerm421 = fTemp425;
																																																																																																																																																																																																																																									self.iTempPerm422 = iTemp426;
																																																																																																																																																																																																																																									self.iTempPerm423 = iTemp427;
																																																																																																																																																																																																																																									self.fTempPerm424 = fTemp428;
																																																																																																																																																																																																																																									self.fTempPerm425 = fTemp429;
																																																																																																																																																																																																																																									self.fTempPerm426 = fTemp430;
																																																																																																																																																																																																																																									self.fTempPerm427 = fTemp431;
																																																																																																																																																																																																																																									self.fTempPerm428 = fTemp432;
																																																																																																																																																																																																																																									self.fTempPerm429 = fTemp433;
																																																																																																																																																																																																																																									self.iTempPerm430 = iTemp434;
																																																																																																																																																																																																																																									self.fTempPerm431 = fTemp435;
																																																																																																																																																																																																																																									self.fTempPerm432 = fTemp436;
																																																																																																																																																																																																																																									self.fTempPerm433 = fTemp437;
																																																																																																																																																																																																																																									self.iTempPerm434 = iTemp438;
																																																																																																																																																																																																																																									self.fTempPerm435 = fTemp439;
																																																																																																																																																																																																																																									self.fTempPerm436 = fTemp440;
																																																																																																																																																																																																																																									self.fTempPerm437 = fTemp441;
																																																																																																																																																																																																																																									self.fTempPerm438 = fTemp442;
																																																																																																																																																																																																																																									self.iTempPerm439 = iTemp443;
																																																																																																																																																																																																																																									self.iTempPerm440 = iTemp444;
																																																																																																																																																																																																																																									self.iTempPerm441 = iTemp445;
																																																																																																																																																																																																																																									self.fTempPerm442 = fTemp446;
																																																																																																																																																																																																																																									self.fTempPerm443 = fTemp447;
																																																																																																																																																																																																																																									self.iTempPerm444 = iTemp448;
																																																																																																																																																																																																																																									self.iTempPerm445 = iTemp449;
																																																																																																																																																																																																																																									self.fTempPerm446 = fTemp450;
																																																																																																																																																																																																																																									self.fTempPerm447 = fTemp451;
																																																																																																																																																																																																																																									self.fTempPerm448 = fTemp452;
																																																																																																																																																																																																																																									self.fTempPerm449 = fTemp453;
																																																																																																																																																																																																																																									self.iTempPerm450 = iTemp454;
																																																																																																																																																																																																																																									self.fTempPerm451 = fTemp455;
																																																																																																																																																																																																																																									self.fTempPerm452 = fTemp456;
																																																																																																																																																																																																																																									self.iTempPerm453 = iTemp457;
																																																																																																																																																																																																																																									self.iTempPerm454 = iTemp458;
																																																																																																																																																																																																																																									self.fTempPerm455 = fTemp459;
																																																																																																																																																																																																																																									self.fTempPerm456 = fTemp460;
																																																																																																																																																																																																																																									self.fTempPerm457 = fTemp461;
																																																																																																																																																																																																																																									self.iTempPerm458 = iTemp462;
																																																																																																																																																																																																																																									self.fTempPerm459 = fTemp463;
																																																																																																																																																																																																																																									self.fTempPerm460 = fTemp464;
																																																																																																																																																																																																																																									self.fTempPerm461 = fTemp465;
																																																																																																																																																																																																																																									self.iTempPerm462 = iTemp466;
																																																																																																																																																																																																																																									self.fTempPerm463 = fTemp467;
																																																																																																																																																																																																																																									self.fTempPerm464 = fTemp468;
																																																																																																																																																																																																																																									self.fTempPerm465 = fTemp469;
																																																																																																																																																																																																																																									self.fTempPerm466 = fTemp470;
																																																																																																																																																																																																																																									self.fTempPerm467 = fTemp471;
																																																																																																																																																																																																																																									self.fTempPerm468 = fTemp472;
																																																																																																																																																																																																																																									self.fTempPerm469 = fTemp473;
																																																																																																																																																																																																																																									self.iTempPerm470 = iTemp474;
																																																																																																																																																																																																																																									self.iTempPerm471 = iTemp475;
																																																																																																																																																																																																																																									self.iTempPerm472 = iTemp476;
																																																																																																																																																																																																																																									self.iTempPerm473 = iTemp477;
																																																																																																																																																																																																																																									self.iTempPerm474 = iTemp478;
																																																																																																																																																																																																																																									self.fTempPerm475 = fTemp479;
																																																																																																																																																																																																																																									self.iTempPerm476 = iTemp480;
																																																																																																																																																																																																																																									self.iTempPerm477 = iTemp481;
																																																																																																																																																																																																																																									self.fTempPerm478 = fTemp482;
																																																																																																																																																																																																																																									self.fTempPerm479 = fTemp483;
																																																																																																																																																																																																																																									self.iTempPerm480 = iTemp484;
																																																																																																																																																																																																																																									self.iTempPerm481 = iTemp485;
																																																																																																																																																																																																																																									self.fTempPerm482 = fTemp486;
																																																																																																																																																																																																																																									self.fTempPerm483 = fTemp487;
																																																																																																																																																																																																																																									self.iTempPerm484 = iTemp488;
																																																																																																																																																																																																																																									self.fTempPerm485 = fTemp489;
																																																																																																																																																																																																																																									self.fTempPerm486 = fTemp490;
																																																																																																																																																																																																																																									self.iTempPerm487 = iTemp491;
																																																																																																																																																																																																																																									self.iTempPerm488 = iTemp492;
																																																																																																																																																																																																																																									self.fTempPerm489 = fTemp493;
																																																																																																																																																																																																																																									self.fTempPerm490 = fTemp494;
																																																																																																																																																																																																																																									self.fTempPerm491 = fTemp495;
																																																																																																																																																																																																																																									self.fTempPerm492 = fTemp496;
																																																																																																																																																																																																																																									self.fTempPerm493 = fTemp497;
																																																																																																																																																																																																																																									self.fTempPerm494 = fTemp498;
																																																																																																																																																																																																																																									self.iTempPerm495 = iTemp499;
																																																																																																																																																																																																																																									self.fTempPerm496 = fTemp500;
																																																																																																																																																																																																																																									self.fTempPerm497 = fTemp501;
																																																																																																																																																																																																																																									self.fTempPerm498 = fTemp502;
																																																																																																																																																																																																																																									self.iTempPerm499 = iTemp503;
																																																																																																																																																																																																																																									self.fTempPerm500 = fTemp504;
																																																																																																																																																																																																																																									self.fTempPerm501 = fTemp505;
																																																																																																																																																																																																																																									self.fTempPerm502 = fTemp506;
																																																																																																																																																																																																																																									self.fTempPerm503 = fTemp507;
																																																																																																																																																																																																																																									self.iTempPerm504 = iTemp508;
																																																																																																																																																																																																																																									self.iTempPerm505 = iTemp509;
																																																																																																																																																																																																																																									self.iTempPerm506 = iTemp510;
																																																																																																																																																																																																																																									self.fTempPerm507 = fTemp511;
																																																																																																																																																																																																																																									self.fTempPerm508 = fTemp512;
																																																																																																																																																																																																																																									self.iTempPerm509 = iTemp513;
																																																																																																																																																																																																																																									self.iTempPerm510 = iTemp514;
																																																																																																																																																																																																																																									self.fTempPerm511 = fTemp515;
																																																																																																																																																																																																																																									self.fTempPerm512 = fTemp516;
																																																																																																																																																																																																																																									self.fTempPerm513 = fTemp517;
																																																																																																																																																																																																																																									self.fTempPerm514 = fTemp518;
																																																																																																																																																																																																																																									self.iTempPerm515 = iTemp519;
																																																																																																																																																																																																																																									self.fTempPerm516 = fTemp520;
																																																																																																																																																																																																																																									self.fTempPerm517 = fTemp521;
																																																																																																																																																																																																																																									self.iTempPerm518 = iTemp522;
																																																																																																																																																																																																																																									self.iTempPerm519 = iTemp523;
																																																																																																																																																																																																																																									self.fTempPerm520 = fTemp524;
																																																																																																																																																																																																																																									self.fTempPerm521 = fTemp525;
																																																																																																																																																																																																																																									self.fTempPerm522 = fTemp526;
																																																																																																																																																																																																																																									self.iTempPerm523 = iTemp527;
																																																																																																																																																																																																																																									self.fTempPerm524 = fTemp528;
																																																																																																																																																																																																																																									self.fTempPerm525 = fTemp529;
																																																																																																																																																																																																																																									self.fTempPerm526 = fTemp530;
																																																																																																																																																																																																																																									self.iTempPerm527 = iTemp531;
																																																																																																																																																																																																																																									self.fTempPerm528 = fTemp532;
																																																																																																																																																																																																																																									self.fTempPerm529 = fTemp533;
																																																																																																																																																																																																																																									self.fTempPerm530 = fTemp534;
																																																																																																																																																																																																																																									self.fTempPerm531 = fTemp535;
																																																																																																																																																																																																																																									self.fTempPerm532 = fTemp536;
																																																																																																																																																																																																																																									self.fTempPerm533 = fTemp537;
																																																																																																																																																																																																																																									self.fTempPerm534 = fTemp538;
																																																																																																																																																																																																																																									self.iTempPerm535 = iTemp539;
																																																																																																																																																																																																																																									self.iTempPerm536 = iTemp540;
																																																																																																																																																																																																																																									self.iTempPerm537 = iTemp541;
																																																																																																																																																																																																																																									self.iTempPerm538 = iTemp542;
																																																																																																																																																																																																																																									self.iTempPerm539 = iTemp543;
																																																																																																																																																																																																																																									self.fTempPerm540 = fTemp544;
																																																																																																																																																																																																																																									self.iTempPerm541 = iTemp545;
																																																																																																																																																																																																																																									self.iTempPerm542 = iTemp546;
																																																																																																																																																																																																																																									self.fTempPerm543 = fTemp547;
																																																																																																																																																																																																																																									self.fTempPerm544 = fTemp548;
																																																																																																																																																																																																																																									self.iTempPerm545 = iTemp549;
																																																																																																																																																																																																																																									self.iTempPerm546 = iTemp550;
																																																																																																																																																																																																																																									self.fTempPerm547 = fTemp551;
																																																																																																																																																																																																																																									self.fTempPerm548 = fTemp552;
																																																																																																																																																																																																																																									self.iTempPerm549 = iTemp553;
																																																																																																																																																																																																																																									self.fTempPerm550 = fTemp554;
																																																																																																																																																																																																																																									self.fTempPerm551 = fTemp555;
																																																																																																																																																																																																																																									self.iTempPerm552 = iTemp556;
																																																																																																																																																																																																																																									self.iTempPerm553 = iTemp557;
																																																																																																																																																																																																																																									self.fTempPerm554 = fTemp558;
																																																																																																																																																																																																																																									self.fTempPerm555 = fTemp559;
																																																																																																																																																																																																																																									self.fTempPerm556 = fTemp560;
																																																																																																																																																																																																																																									self.fTempPerm557 = fTemp561;
																																																																																																																																																																																																																																									self.fTempPerm558 = fTemp562;
																																																																																																																																																																																																																																									self.fTempPerm559 = fTemp563;
																																																																																																																																																																																																																																									self.iTempPerm560 = iTemp564;
																																																																																																																																																																																																																																									self.fTempPerm561 = fTemp565;
																																																																																																																																																																																																																																									self.fTempPerm562 = fTemp566;
																																																																																																																																																																																																																																									self.fTempPerm563 = fTemp567;
																																																																																																																																																																																																																																									self.iTempPerm564 = iTemp568;
																																																																																																																																																																																																																																									self.fTempPerm565 = fTemp569;
																																																																																																																																																																																																																																									self.fTempPerm566 = fTemp570;
																																																																																																																																																																																																																																									self.fTempPerm567 = fTemp571;
																																																																																																																																																																																																																																									self.fTempPerm568 = fTemp572;
																																																																																																																																																																																																																																									self.iTempPerm569 = iTemp573;
																																																																																																																																																																																																																																									self.iTempPerm570 = iTemp574;
																																																																																																																																																																																																																																									self.iTempPerm571 = iTemp575;
																																																																																																																																																																																																																																									self.fTempPerm572 = fTemp576;
																																																																																																																																																																																																																																									self.fTempPerm573 = fTemp577;
																																																																																																																																																																																																																																									self.iTempPerm574 = iTemp578;
																																																																																																																																																																																																																																									self.iTempPerm575 = iTemp579;
																																																																																																																																																																																																																																									self.fTempPerm576 = fTemp580;
																																																																																																																																																																																																																																									self.fTempPerm577 = fTemp581;
																																																																																																																																																																																																																																									self.fTempPerm578 = fTemp582;
																																																																																																																																																																																																																																									self.fTempPerm579 = fTemp583;
																																																																																																																																																																																																																																									self.iTempPerm580 = iTemp584;
																																																																																																																																																																																																																																									self.fTempPerm581 = fTemp585;
																																																																																																																																																																																																																																									self.fTempPerm582 = fTemp586;
																																																																																																																																																																																																																																									self.iTempPerm583 = iTemp587;
																																																																																																																																																																																																																																									self.iTempPerm584 = iTemp588;
																																																																																																																																																																																																																																									self.fTempPerm585 = fTemp589;
																																																																																																																																																																																																																																									self.fTempPerm586 = fTemp590;
																																																																																																																																																																																																																																									self.fTempPerm587 = fTemp591;
																																																																																																																																																																																																																																									self.iTempPerm588 = iTemp592;
																																																																																																																																																																																																																																									self.fTempPerm589 = fTemp593;
																																																																																																																																																																																																																																									self.fTempPerm590 = fTemp594;
																																																																																																																																																																																																																																									self.fTempPerm591 = fTemp595;
																																																																																																																																																																																																																																									self.iTempPerm592 = iTemp596;
																																																																																																																																																																																																																																									self.fTempPerm593 = fTemp597;
																																																																																																																																																																																																																																									self.fTempPerm594 = fTemp598;
																																																																																																																																																																																																																																									self.fTempPerm595 = fTemp599;
																																																																																																																																																																																																																																									self.fTempPerm596 = fTemp600;
																																																																																																																																																																																																																																									self.fTempPerm597 = fTemp601;
																																																																																																																																																																																																																																									self.fTempPerm598 = fTemp602;
																																																																																																																																																																																																																																									self.fTempPerm599 = fTemp603;
																																																																																																																																																																																																																																									self.iTempPerm600 = iTemp604;
																																																																																																																																																																																																																																									self.iTempPerm601 = iTemp605;
																																																																																																																																																																																																																																									self.iTempPerm602 = iTemp606;
																																																																																																																																																																																																																																									self.iTempPerm603 = iTemp607;
																																																																																																																																																																																																																																									self.iTempPerm604 = iTemp608;
																																																																																																																																																																																																																																									self.fTempPerm605 = fTemp609;
																																																																																																																																																																																																																																									self.iTempPerm606 = iTemp610;
																																																																																																																																																																																																																																									self.iTempPerm607 = iTemp611;
																																																																																																																																																																																																																																									self.fTempPerm608 = fTemp612;
																																																																																																																																																																																																																																									self.fTempPerm609 = fTemp613;
																																																																																																																																																																																																																																									self.iTempPerm610 = iTemp614;
																																																																																																																																																																																																																																									self.iTempPerm611 = iTemp615;
																																																																																																																																																																																																																																									self.fTempPerm612 = fTemp616;
																																																																																																																																																																																																																																									self.fTempPerm613 = fTemp617;
																																																																																																																																																																																																																																									self.iTempPerm614 = iTemp618;
																																																																																																																																																																																																																																									self.fTempPerm615 = fTemp619;
																																																																																																																																																																																																																																									self.fTempPerm616 = fTemp620;
																																																																																																																																																																																																																																									self.iTempPerm617 = iTemp621;
																																																																																																																																																																																																																																									self.iTempPerm618 = iTemp622;
																																																																																																																																																																																																																																									self.fTempPerm619 = fTemp623;
																																																																																																																																																																																																																																									self.fTempPerm620 = fTemp624;
																																																																																																																																																																																																																																									self.fTempPerm621 = fTemp625;
																																																																																																																																																																																																																																									self.fTempPerm622 = fTemp626;
																																																																																																																																																																																																																																									self.fTempPerm623 = fTemp627;
																																																																																																																																																																																																																																									self.fTempPerm624 = fTemp628;
																																																																																																																																																																																																																																									self.iTempPerm625 = iTemp629;
																																																																																																																																																																																																																																									self.fTempPerm626 = fTemp630;
																																																																																																																																																																																																																																									self.fTempPerm627 = fTemp631;
																																																																																																																																																																																																																																									self.fTempPerm628 = fTemp632;
																																																																																																																																																																																																																																									self.iTempPerm629 = iTemp633;
																																																																																																																																																																																																																																									self.fTempPerm630 = fTemp634;
																																																																																																																																																																																																																																									self.fTempPerm631 = fTemp635;
																																																																																																																																																																																																																																									self.fTempPerm632 = fTemp636;
																																																																																																																																																																																																																																									self.fTempPerm633 = fTemp637;
																																																																																																																																																																																																																																									self.iTempPerm634 = iTemp638;
																																																																																																																																																																																																																																									self.iTempPerm635 = iTemp639;
																																																																																																																																																																																																																																									self.iTempPerm636 = iTemp640;
																																																																																																																																																																																																																																									self.fTempPerm637 = fTemp641;
																																																																																																																																																																																																																																									self.fTempPerm638 = fTemp642;
																																																																																																																																																																																																																																									self.iTempPerm639 = iTemp643;
																																																																																																																																																																																																																																									self.iTempPerm640 = iTemp644;
																																																																																																																																																																																																																																									self.fTempPerm641 = fTemp645;
																																																																																																																																																																																																																																									self.fTempPerm642 = fTemp646;
																																																																																																																																																																																																																																									self.fTempPerm643 = fTemp647;
																																																																																																																																																																																																																																									self.fTempPerm644 = fTemp648;
																																																																																																																																																																																																																																									self.iTempPerm645 = iTemp649;
																																																																																																																																																																																																																																									self.fTempPerm646 = fTemp650;
																																																																																																																																																																																																																																									self.fTempPerm647 = fTemp651;
																																																																																																																																																																																																																																									self.iTempPerm648 = iTemp652;
																																																																																																																																																																																																																																									self.iTempPerm649 = iTemp653;
																																																																																																																																																																																																																																									self.fTempPerm650 = fTemp654;
																																																																																																																																																																																																																																									self.fTempPerm651 = fTemp655;
																																																																																																																																																																																																																																									self.fTempPerm652 = fTemp656;
																																																																																																																																																																																																																																									self.iTempPerm653 = iTemp657;
																																																																																																																																																																																																																																									self.fTempPerm654 = fTemp658;
																																																																																																																																																																																																																																									self.fTempPerm655 = fTemp659;
																																																																																																																																																																																																																																									self.fTempPerm656 = fTemp660;
																																																																																																																																																																																																																																									self.iTempPerm657 = iTemp661;
																																																																																																																																																																																																																																									self.fTempPerm658 = fTemp662;
																																																																																																																																																																																																																																									self.fTempPerm659 = fTemp663;
																																																																																																																																																																																																																																									self.fTempPerm660 = fTemp664;
																																																																																																																																																																																																																																									self.fTempPerm661 = fTemp665;
																																																																																																																																																																																																																																									self.fTempPerm662 = fTemp666;
																																																																																																																																																																																																																																									self.fTempPerm663 = fTemp667;
																																																																																																																																																																																																																																									self.fTempPerm664 = fTemp668;
																																																																																																																																																																																																																																									self.iTempPerm665 = iTemp669;
																																																																																																																																																																																																																																									self.iTempPerm666 = iTemp670;
																																																																																																																																																																																																																																									self.iTempPerm667 = iTemp671;
																																																																																																																																																																																																																																									self.iTempPerm668 = iTemp672;
																																																																																																																																																																																																																																									self.iTempPerm669 = iTemp673;
																																																																																																																																																																																																																																									self.fTempPerm670 = fTemp674;
																																																																																																																																																																																																																																									self.iTempPerm671 = iTemp675;
																																																																																																																																																																																																																																									self.iTempPerm672 = iTemp676;
																																																																																																																																																																																																																																									self.fTempPerm673 = fTemp677;
																																																																																																																																																																																																																																									self.fTempPerm674 = fTemp678;
																																																																																																																																																																																																																																									self.iTempPerm675 = iTemp679;
																																																																																																																																																																																																																																									self.iTempPerm676 = iTemp680;
																																																																																																																																																																																																																																									self.fTempPerm677 = fTemp681;
																																																																																																																																																																																																																																									self.fTempPerm678 = fTemp682;
																																																																																																																																																																																																																																									self.iTempPerm679 = iTemp683;
																																																																																																																																																																																																																																									self.fTempPerm680 = fTemp684;
																																																																																																																																																																																																																																									self.fTempPerm681 = fTemp685;
																																																																																																																																																																																																																																									self.iTempPerm682 = iTemp686;
																																																																																																																																																																																																																																									self.iTempPerm683 = iTemp687;
																																																																																																																																																																																																																																									self.fTempPerm684 = fTemp688;
																																																																																																																																																																																																																																									self.fTempPerm685 = fTemp689;
																																																																																																																																																																																																																																									self.fTempPerm686 = fTemp690;
																																																																																																																																																																																																																																									self.fTempPerm687 = fTemp691;
																																																																																																																																																																																																																																									self.fTempPerm688 = fTemp692;
																																																																																																																																																																																																																																									self.fTempPerm689 = fTemp693;
																																																																																																																																																																																																																																									self.iTempPerm690 = iTemp694;
																																																																																																																																																																																																																																									self.fTempPerm691 = fTemp695;
																																																																																																																																																																																																																																									self.fTempPerm692 = fTemp696;
																																																																																																																																																																																																																																									self.fTempPerm693 = fTemp697;
																																																																																																																																																																																																																																									self.iTempPerm694 = iTemp698;
																																																																																																																																																																																																																																									self.fTempPerm695 = fTemp699;
																																																																																																																																																																																																																																									self.fTempPerm696 = fTemp700;
																																																																																																																																																																																																																																									self.fTempPerm697 = fTemp701;
																																																																																																																																																																																																																																									self.fTempPerm698 = fTemp702;
																																																																																																																																																																																																																																									self.iTempPerm699 = iTemp703;
																																																																																																																																																																																																																																									self.iTempPerm700 = iTemp704;
																																																																																																																																																																																																																																									self.iTempPerm701 = iTemp705;
																																																																																																																																																																																																																																									self.fTempPerm702 = fTemp706;
																																																																																																																																																																																																																																									self.fTempPerm703 = fTemp707;
																																																																																																																																																																																																																																									self.iTempPerm704 = iTemp708;
																																																																																																																																																																																																																																									self.iTempPerm705 = iTemp709;
																																																																																																																																																																																																																																									self.fTempPerm706 = fTemp710;
																																																																																																																																																																																																																																									self.fTempPerm707 = fTemp711;
																																																																																																																																																																																																																																									self.fTempPerm708 = fTemp712;
																																																																																																																																																																																																																																									self.fTempPerm709 = fTemp713;
																																																																																																																																																																																																																																									self.iTempPerm710 = iTemp714;
																																																																																																																																																																																																																																									self.fTempPerm711 = fTemp715;
																																																																																																																																																																																																																																									self.fTempPerm712 = fTemp716;
																																																																																																																																																																																																																																									self.iTempPerm713 = iTemp717;
																																																																																																																																																																																																																																									self.iTempPerm714 = iTemp718;
																																																																																																																																																																																																																																									self.fTempPerm715 = fTemp719;
																																																																																																																																																																																																																																									self.fTempPerm716 = fTemp720;
																																																																																																																																																																																																																																									self.fTempPerm717 = fTemp721;
																																																																																																																																																																																																																																									self.iTempPerm718 = iTemp722;
																																																																																																																																																																																																																																									self.fTempPerm719 = fTemp723;
																																																																																																																																																																																																																																									self.fTempPerm720 = fTemp724;
																																																																																																																																																																																																																																									self.fTempPerm721 = fTemp725;
																																																																																																																																																																																																																																									self.iTempPerm722 = iTemp726;
																																																																																																																																																																																																																																									self.fTempPerm723 = fTemp727;
																																																																																																																																																																																																																																									self.fTempPerm724 = fTemp728;
																																																																																																																																																																																																																																									self.fTempPerm725 = fTemp729;
																																																																																																																																																																																																																																									self.fTempPerm726 = fTemp730;
																																																																																																																																																																																																																																									self.fTempPerm727 = fTemp731;
																																																																																																																																																																																																																																									self.fTempPerm728 = fTemp732;
																																																																																																																																																																																																																																									self.fTempPerm729 = fTemp733;
																																																																																																																																																																																																																																									self.iTempPerm730 = iTemp734;
																																																																																																																																																																																																																																									self.iTempPerm731 = iTemp735;
																																																																																																																																																																																																																																									self.iTempPerm732 = iTemp736;
																																																																																																																																																																																																																																									self.iTempPerm733 = iTemp737;
																																																																																																																																																																																																																																									self.iTempPerm734 = iTemp738;
																																																																																																																																																																																																																																									self.fTempPerm735 = fTemp739;
																																																																																																																																																																																																																																									self.iTempPerm736 = iTemp740;
																																																																																																																																																																																																																																									self.fTempPerm737 = fTemp741;
																																																																																																																																																																																																																																									self.fTempPerm738 = fTemp742;
																																																																																																																																																																																																																																									self.iTempPerm739 = iTemp743;
																																																																																																																																																																																																																																									self.iTempPerm740 = iTemp744;
																																																																																																																																																																																																																																									self.fTempPerm741 = fTemp745;
																																																																																																																																																																																																																																									self.fTempPerm742 = fTemp746;
																																																																																																																																																																																																																																									self.iTempPerm743 = iTemp747;
																																																																																																																																																																																																																																									self.fTempPerm744 = fTemp748;
																																																																																																																																																																																																																																									self.fTempPerm745 = fTemp749;
																																																																																																																																																																																																																																									self.iTempPerm746 = iTemp750;
																																																																																																																																																																																																																																									self.iTempPerm747 = iTemp751;
																																																																																																																																																																																																																																									self.fTempPerm748 = fTemp752;
																																																																																																																																																																																																																																									self.fTempPerm749 = fTemp753;
																																																																																																																																																																																																																																									self.fTempPerm750 = fTemp754;
																																																																																																																																																																																																																																									self.fTempPerm751 = fTemp755;
																																																																																																																																																																																																																																									self.fTempPerm752 = fTemp756;
																																																																																																																																																																																																																																									self.fTempPerm753 = fTemp757;
																																																																																																																																																																																																																																									self.iTempPerm754 = iTemp758;
																																																																																																																																																																																																																																									self.fTempPerm755 = fTemp759;
																																																																																																																																																																																																																																									self.fTempPerm756 = fTemp760;
																																																																																																																																																																																																																																									self.fTempPerm757 = fTemp761;
																																																																																																																																																																																																																																									self.iTempPerm758 = iTemp762;
																																																																																																																																																																																																																																									self.fTempPerm759 = fTemp763;
																																																																																																																																																																																																																																									self.fTempPerm760 = fTemp764;
																																																																																																																																																																																																																																									self.fTempPerm761 = fTemp765;
																																																																																																																																																																																																																																									self.fTempPerm762 = fTemp766;
																																																																																																																																																																																																																																									self.iTempPerm763 = iTemp767;
																																																																																																																																																																																																																																									self.iTempPerm764 = iTemp768;
																																																																																																																																																																																																																																									self.iTempPerm765 = iTemp769;
																																																																																																																																																																																																																																									self.fTempPerm766 = fTemp770;
																																																																																																																																																																																																																																									self.fTempPerm767 = fTemp771;
																																																																																																																																																																																																																																									self.iTempPerm768 = iTemp772;
																																																																																																																																																																																																																																									self.iTempPerm769 = iTemp773;
																																																																																																																																																																																																																																									self.fTempPerm770 = fTemp774;
																																																																																																																																																																																																																																									self.fTempPerm771 = fTemp775;
																																																																																																																																																																																																																																									self.fTempPerm772 = fTemp776;
																																																																																																																																																																																																																																									self.fTempPerm773 = fTemp777;
																																																																																																																																																																																																																																									self.iTempPerm774 = iTemp778;
																																																																																																																																																																																																																																									self.fTempPerm775 = fTemp779;
																																																																																																																																																																																																																																									self.fTempPerm776 = fTemp780;
																																																																																																																																																																																																																																									self.iTempPerm777 = iTemp781;
																																																																																																																																																																																																																																									self.iTempPerm778 = iTemp782;
																																																																																																																																																																																																																																									self.fTempPerm779 = fTemp783;
																																																																																																																																																																																																																																									self.fTempPerm780 = fTemp784;
																																																																																																																																																																																																																																									self.fTempPerm781 = fTemp785;
																																																																																																																																																																																																																																									self.iTempPerm782 = iTemp786;
																																																																																																																																																																																																																																									self.fTempPerm783 = fTemp787;
																																																																																																																																																																																																																																									self.fTempPerm784 = fTemp788;
																																																																																																																																																																																																																																									self.fTempPerm785 = fTemp789;
																																																																																																																																																																																																																																									self.iTempPerm786 = iTemp790;
																																																																																																																																																																																																																																									self.fTempPerm787 = fTemp791;
																																																																																																																																																																																																																																									self.fTempPerm788 = fTemp792;
																																																																																																																																																																																																																																									self.fTempPerm789 = fTemp793;
																																																																																																																																																																																																																																									self.fTempPerm790 = fTemp794;
																																																																																																																																																																																																																																									self.fTempPerm791 = fTemp795;
																																																																																																																																																																																																																																									self.fTempPerm792 = fTemp796;
																																																																																																																																																																																																																																									self.fTempPerm793 = fTemp797;
																																																																																																																																																																																																																																									self.fTempPerm794 = fTemp798;
																																																																																																																																																																																																																																									self.fTempPerm795 = fTemp799;
																																																																																																																																																																																																																																									self.fTempPerm796 = fTemp800;
																																																																																																																																																																																																																																									self.fTempPerm797 = fTemp801;
																																																																																																																																																																																																																																									self.fTempPerm798 = fTemp802;
																																																																																																																																																																																																																																									self.fTempPerm799 = fTemp804;
																																																																																																																																																																																																																																									self.fTempPerm800 = fTemp805;
																																																																																																																																																																																																																																									self.fTempPerm801 = fTemp806;
																																																																																																																																																																																																																																									self.fTempPerm802 = fTemp807;
																																																																																																																																																																																																																																									self.fTempPerm803 = fTemp808;
																																																																																																																																																																																																																																									self.fTempPerm804 = fTemp810;
																																																																																																																																																																																																																																									self.fTempPerm805 = fTemp811;
																																																																																																																																																																																																																																									self.fTempPerm806 = fTemp812;
																																																																																																																																																																																																																																									self.fTempPerm807 = fTemp813;
																																																																																																																																																																																																																																									self.fTempPerm808 = fTemp814;
																																																																																																																																																																																																																																									self.fTempPerm809 = fTemp816;
																																																																																																																																																																																																																																									self.fTempPerm810 = fTemp817;
																																																																																																																																																																																																																																									self.fTempPerm811 = fTemp818;
																																																																																																																																																																																																																																									self.fTempPerm812 = fTemp819;
																																																																																																																																																																																																																																									self.fTempPerm813 = fTemp820;
																																																																																																																																																																																																																																									self.fTempPerm814 = fTemp822;
																																																																																																																																																																																																																																									self.fTempPerm815 = fTemp823;
																																																																																																																																																																																																																																									self.fTempPerm816 = fTemp824;
																																																																																																																																																																																																																																									self.fTempPerm817 = fTemp825;
																																																																																																																																																																																																																																									self.fTempPerm818 = fTemp826;
																																																																																																																																																																																																																																									self.fTempPerm819 = fTemp828;
																																																																																																																																																																																																																																									self.fTempPerm820 = fTemp829;
																																																																																																																																																																																																																																									self.fTempPerm821 = fTemp830;
																																																																																																																																																																																																																																									self.fTempPerm822 = fTemp831;
																																																																																																																																																																																																																																									self.fTempPerm823 = fTemp832;
																																																																																																																																																																																																																																									self.fTempPerm824 = fTemp834;
																																																																																																																																																																																																																																									self.fTempPerm825 = fTemp835;
																																																																																																																																																																																																																																									self.fTempPerm826 = fTemp836;
																																																																																																																																																																																																																																									self.fTempPerm827 = fTemp837;
																																																																																																																																																																																																																																									self.fTempPerm828 = fTemp838;
																																																																																																																																																																																																																																									self.fTempPerm829 = fTemp840;
																																																																																																																																																																																																																																									self.fTempPerm830 = fTemp841;
																																																																																																																																																																																																																																									self.fTempPerm831 = fTemp842;
																																																																																																																																																																																																																																									self.fTempPerm832 = fTemp843;
																																																																																																																																																																																																																																									self.fTempPerm833 = fTemp844;
																																																																																																																																																																																																																																									self.fTempPerm834 = fTemp846;
																																																																																																																																																																																																																																									self.fTempPerm835 = fTemp847;
																																																																																																																																																																																																																																									self.fTempPerm836 = fTemp848;
																																																																																																																																																																																																																																									self.fTempPerm837 = fTemp849;
																																																																																																																																																																																																																																									self.fTempPerm838 = fTemp850;
																																																																																																																																																																																																																																									self.fTempPerm839 = fTemp852;
																																																																																																																																																																																																																																									self.fTempPerm840 = fTemp853;
																																																																																																																																																																																																																																									self.fTempPerm841 = fTemp854;
																																																																																																																																																																																																																																									self.fTempPerm842 = fTemp855;
																																																																																																																																																																																																																																									self.fTempPerm843 = fTemp856;
																																																																																																																																																																																																																																									self.fTempPerm844 = fTemp858;
																																																																																																																																																																																																																																									self.fTempPerm845 = fTemp859;
																																																																																																																																																																																																																																									self.fTempPerm846 = fTemp860;
																																																																																																																																																																																																																																									self.fTempPerm847 = fTemp861;
																																																																																																																																																																																																																																									self.fTempPerm848 = fTemp862;
																																																																																																																																																																																																																																									self.fTempPerm849 = fTemp864;
																																																																																																																																																																																																																																									self.fTempPerm850 = fTemp865;
																																																																																																																																																																																																																																									self.fTempPerm851 = fTemp866;
																																																																																																																																																																																																																																									self.fTempPerm852 = fTemp867;
																																																																																																																																																																																																																																									self.fTempPerm853 = fTemp868;
																																																																																																																																																																																																																																									self.fTempPerm854 = fTemp872;
																																																																																																																																																																																																																																									self.fTempPerm855 = fTemp873;
																																																																																																																																																																																																																																									self.fTempPerm856 = fTemp874;
																																																																																																																																																																																																																																									self.fTempPerm857 = fTemp876;
																																																																																																																																																																																																																																									self.fTempPerm858 = fTemp877;
																																																																																																																																																																																																																																									self.fTempPerm859 = fTemp878;
																																																																																																																																																																																																																																									self.fTempPerm860 = fTemp880;
																																																																																																																																																																																																																																									self.fTempPerm861 = fTemp881;
																																																																																																																																																																																																																																									self.fTempPerm862 = fTemp882;
																																																																																																																																																																																																																																									self.fTempPerm863 = fTemp884;
																																																																																																																																																																																																																																									self.fTempPerm864 = fTemp885;
																																																																																																																																																																																																																																									self.fTempPerm865 = fTemp886;
																																																																																																																																																																																																																																									self.fTempPerm866 = fTemp888;
																																																																																																																																																																																																																																									self.fTempPerm867 = fTemp889;
																																																																																																																																																																																																																																									self.fTempPerm868 = fTemp890;
																																																																																																																																																																																																																																									self.fTempPerm869 = fTemp892;
																																																																																																																																																																																																																																									self.fTempPerm870 = fTemp893;
																																																																																																																																																																																																																																									self.fTempPerm871 = fTemp894;
																																																																																																																																																																																																																																									self.fTempPerm872 = fTemp896;
																																																																																																																																																																																																																																									self.fTempPerm873 = fTemp897;
																																																																																																																																																																																																																																									self.fTempPerm874 = fTemp898;
																																																																																																																																																																																																																																									self.fTempPerm875 = fTemp900;
																																																																																																																																																																																																																																									self.fTempPerm876 = fTemp901;
																																																																																																																																																																																																																																									self.fTempPerm877 = fTemp902;
																																																																																																																																																																																																																																									self.fTempPerm878 = fTemp904;
																																																																																																																																																																																																																																									self.fTempPerm879 = fTemp905;
																																																																																																																																																																																																																																									self.fTempPerm880 = fTemp906;
																																																																																																																																																																																																																																									self.fTempPerm881 = fTemp908;
																																																																																																																																																																																																																																									self.fTempPerm882 = fTemp909;
																																																																																																																																																																																																																																									self.fTempPerm883 = fTemp910;
																																																																																																																																																																																																																																									self.fTempPerm884 = fTemp912;
																																																																																																																																																																																																																																									self.fTempPerm885 = fTemp913;
																																																																																																																																																																																																																																									self.fTempPerm886 = fTemp914;
																																																																																																																																																																																																																																									self.fTempPerm887 = fTemp916;
																																																																																																																																																																																																																																									self.fTempPerm888 = fTemp917;
																																																																																																																																																																																																																																									self.fTempPerm889 = fTemp918;
																																																																																																																																																																																																																																								}

}

