/* ------------------------------------------------------------
name: "plugin-calls"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -cn CallsDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

fn CallsDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct CallsDsp {
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fRec0: [F32;2],
	fHslider0: F32,
	fRec1: [F32;2],
	fHslider1: F32,
	fRec2: [F32;2],
	fHslider2: F32,
	fConst3: F32,
	fConst4: F32,
	fRec3: [F32;2],
	iTempPerm0: i32,
	iRec5: [i32;2],
	fHslider3: F32,
	fRec11: [F32;2],
	fHslider4: F32,
	fRec12: [F32;2],
	fHslider5: F32,
	fRec13: [F32;2],
	fHslider6: F32,
	fRec14: [F32;2],
	fHslider7: F32,
	fRec15: [F32;2],
	fHslider8: F32,
	fRec16: [F32;2],
	fHslider9: F32,
	fRec17: [F32;2],
	iTempPerm1: i32,
	iTempPerm2: i32,
	fHslider10: F32,
	iRec18: [i32;2],
	iTempPerm3: i32,
	iTempPerm4: i32,
	iTempPerm5: i32,
	iTempPerm6: i32,
	iTempPerm7: i32,
	iTempPerm8: i32,
	iTempPerm9: i32,
	iTempPerm10: i32,
	iTempPerm11: i32,
	iTempPerm12: i32,
	iTempPerm13: i32,
	iTempPerm14: i32,
	iTempPerm15: i32,
	iTempPerm16: i32,
	fTempPerm17: F32,
	fTempPerm18: F32,
	iTempPerm19: i32,
	fHslider11: F32,
	fRec19: [F32;2],
	fTempPerm20: F32,
	fTempPerm21: F32,
	iTempPerm22: i32,
	iTempPerm23: i32,
	fTempPerm24: F32,
	fHslider12: F32,
	iTempPerm25: i32,
	iTempPerm26: i32,
	fTempPerm27: F32,
	fTempPerm28: F32,
	fTempPerm29: F32,
	fTempPerm30: F32,
	fHslider13: F32,
	fRec20: [F32;2],
	fTempPerm31: F32,
	fTempPerm32: F32,
	iTempPerm33: i32,
	fTempPerm34: F32,
	fTempPerm35: F32,
	fTempPerm36: F32,
	iTempPerm37: i32,
	iTempPerm38: i32,
	fTempPerm39: F32,
	fTempPerm40: F32,
	iTempPerm41: i32,
	fTempPerm42: F32,
	fConst5: F32,
	iTempPerm43: i32,
	iTempPerm44: i32,
	iRec6: [i32;2],
	iRec7: [i32;2],
	iRec8: [i32;2],
	iRec9: [i32;2],
	iTempPerm45: i32,
	iTempPerm46: i32,
	iTempPerm47: i32,
	fRec4: [F32;2],
	fHslider14: F32,
	fRec21: [F32;2],
	fTempPerm48: F32,
	fTempPerm49: F32,
	fTempPerm50: F32,
	fTempPerm51: F32,
	fTempPerm52: F32,
	fConst6: F32,
	fTempPerm53: F32,
	iTempPerm54: i32,
	iTempPerm55: i32,
	fTempPerm56: F32,
	fTempPerm57: F32,
	iTempPerm58: i32,
	fTempPerm59: F32,
	iTempPerm60: i32,
	iTempPerm61: i32,
	iTempPerm62: i32,
	iTempPerm63: i32,
	iTempPerm64: i32,
	fRec23: [F32;2],
	fTempPerm65: F32,
	iTempPerm66: i32,
	iTempPerm67: i32,
	iTempPerm68: i32,
	iTempPerm69: i32,
	fTempPerm70: F32,
	fTempPerm71: F32,
	iTempPerm72: i32,
	fTempPerm73: F32,
	fTempPerm74: F32,
	iTempPerm75: i32,
	iTempPerm76: i32,
	fTempPerm77: F32,
	fTempPerm78: F32,
	iTempPerm79: i32,
	fTempPerm80: F32,
	fTempPerm81: F32,
	fTempPerm82: F32,
	fTempPerm83: F32,
	iTempPerm84: i32,
	fTempPerm85: F32,
	fTempPerm86: F32,
	fRec24: [F32;2],
	fRec25: [F32;2],
	fTempPerm87: F32,
	fTempPerm88: F32,
	iTempPerm89: i32,
	fTempPerm90: F32,
	fConst7: F32,
	fTempPerm91: F32,
	fRec22: [F32;2],
	iTempPerm92: i32,
	iTempPerm93: i32,
	fRec26: [F32;2],
	fTempPerm94: F32,
	fTempPerm95: F32,
	iTempPerm96: i32,
	iRec28: [i32;2],
	iRec34: [i32;2],
	iTempPerm97: i32,
	iTempPerm98: i32,
	iTempPerm99: i32,
	iTempPerm100: i32,
	iTempPerm101: i32,
	iTempPerm102: i32,
	iTempPerm103: i32,
	iTempPerm104: i32,
	iTempPerm105: i32,
	iTempPerm106: i32,
	iTempPerm107: i32,
	iTempPerm108: i32,
	iTempPerm109: i32,
	iTempPerm110: i32,
	fTempPerm111: F32,
	fTempPerm112: F32,
	iTempPerm113: i32,
	fTempPerm114: F32,
	iTempPerm115: i32,
	iTempPerm116: i32,
	fTempPerm117: F32,
	fHslider15: F32,
	iTempPerm118: i32,
	iTempPerm119: i32,
	fTempPerm120: F32,
	fTempPerm121: F32,
	fTempPerm122: F32,
	fTempPerm123: F32,
	iTempPerm124: i32,
	iTempPerm125: i32,
	fTempPerm126: F32,
	fTempPerm127: F32,
	iTempPerm128: i32,
	fTempPerm129: F32,
	iTempPerm130: i32,
	iRec29: [i32;2],
	iRec30: [i32;2],
	iRec31: [i32;2],
	iRec32: [i32;2],
	iTempPerm131: i32,
	iTempPerm132: i32,
	iTempPerm133: i32,
	fRec27: [F32;2],
	fTempPerm134: F32,
	fTempPerm135: F32,
	fTempPerm136: F32,
	fTempPerm137: F32,
	fTempPerm138: F32,
	iTempPerm139: i32,
	iTempPerm140: i32,
	fTempPerm141: F32,
	fTempPerm142: F32,
	iTempPerm143: i32,
	fTempPerm144: F32,
	iTempPerm145: i32,
	iTempPerm146: i32,
	iTempPerm147: i32,
	iTempPerm148: i32,
	iTempPerm149: i32,
	fRec36: [F32;2],
	fTempPerm150: F32,
	iTempPerm151: i32,
	iTempPerm152: i32,
	iTempPerm153: i32,
	iTempPerm154: i32,
	fTempPerm155: F32,
	fTempPerm156: F32,
	iTempPerm157: i32,
	fTempPerm158: F32,
	fTempPerm159: F32,
	iTempPerm160: i32,
	iTempPerm161: i32,
	fTempPerm162: F32,
	fTempPerm163: F32,
	iTempPerm164: i32,
	fTempPerm165: F32,
	fTempPerm166: F32,
	fTempPerm167: F32,
	fTempPerm168: F32,
	iTempPerm169: i32,
	fTempPerm170: F32,
	fTempPerm171: F32,
	fRec37: [F32;2],
	fRec38: [F32;2],
	fTempPerm172: F32,
	fTempPerm173: F32,
	iTempPerm174: i32,
	fTempPerm175: F32,
	fTempPerm176: F32,
	fRec35: [F32;2],
	iTempPerm177: i32,
	iTempPerm178: i32,
	fRec39: [F32;2],
	fTempPerm179: F32,
	fTempPerm180: F32,
	iTempPerm181: i32,
	iRec41: [i32;2],
	iRec47: [i32;2],
	iTempPerm182: i32,
	iTempPerm183: i32,
	iTempPerm184: i32,
	iTempPerm185: i32,
	iTempPerm186: i32,
	iTempPerm187: i32,
	iTempPerm188: i32,
	iTempPerm189: i32,
	iTempPerm190: i32,
	iTempPerm191: i32,
	iTempPerm192: i32,
	iTempPerm193: i32,
	iTempPerm194: i32,
	iTempPerm195: i32,
	fTempPerm196: F32,
	fTempPerm197: F32,
	iTempPerm198: i32,
	fTempPerm199: F32,
	iTempPerm200: i32,
	iTempPerm201: i32,
	fTempPerm202: F32,
	fHslider16: F32,
	iTempPerm203: i32,
	iTempPerm204: i32,
	fTempPerm205: F32,
	fTempPerm206: F32,
	fTempPerm207: F32,
	fTempPerm208: F32,
	iTempPerm209: i32,
	iTempPerm210: i32,
	fTempPerm211: F32,
	fTempPerm212: F32,
	iTempPerm213: i32,
	fTempPerm214: F32,
	iTempPerm215: i32,
	iRec42: [i32;2],
	iRec43: [i32;2],
	iRec44: [i32;2],
	iRec45: [i32;2],
	iTempPerm216: i32,
	iTempPerm217: i32,
	iTempPerm218: i32,
	fRec40: [F32;2],
	fTempPerm219: F32,
	fTempPerm220: F32,
	fTempPerm221: F32,
	fTempPerm222: F32,
	fTempPerm223: F32,
	iTempPerm224: i32,
	iTempPerm225: i32,
	fTempPerm226: F32,
	fTempPerm227: F32,
	iTempPerm228: i32,
	fTempPerm229: F32,
	iTempPerm230: i32,
	iTempPerm231: i32,
	iTempPerm232: i32,
	iTempPerm233: i32,
	iTempPerm234: i32,
	fRec49: [F32;2],
	fTempPerm235: F32,
	iTempPerm236: i32,
	iTempPerm237: i32,
	iTempPerm238: i32,
	iTempPerm239: i32,
	fTempPerm240: F32,
	fTempPerm241: F32,
	iTempPerm242: i32,
	fTempPerm243: F32,
	fTempPerm244: F32,
	iTempPerm245: i32,
	iTempPerm246: i32,
	fTempPerm247: F32,
	fTempPerm248: F32,
	iTempPerm249: i32,
	fTempPerm250: F32,
	fTempPerm251: F32,
	fTempPerm252: F32,
	fTempPerm253: F32,
	iTempPerm254: i32,
	fTempPerm255: F32,
	fTempPerm256: F32,
	fRec50: [F32;2],
	fRec51: [F32;2],
	fTempPerm257: F32,
	fTempPerm258: F32,
	iTempPerm259: i32,
	fTempPerm260: F32,
	fTempPerm261: F32,
	fRec48: [F32;2],
	iTempPerm262: i32,
	iTempPerm263: i32,
	fRec52: [F32;2],
	fTempPerm264: F32,
	fTempPerm265: F32,
	iTempPerm266: i32,
	iRec54: [i32;2],
	iRec60: [i32;2],
	iTempPerm267: i32,
	iTempPerm268: i32,
	iTempPerm269: i32,
	iTempPerm270: i32,
	iTempPerm271: i32,
	iTempPerm272: i32,
	iTempPerm273: i32,
	iTempPerm274: i32,
	iTempPerm275: i32,
	iTempPerm276: i32,
	iTempPerm277: i32,
	iTempPerm278: i32,
	iTempPerm279: i32,
	iTempPerm280: i32,
	fTempPerm281: F32,
	fTempPerm282: F32,
	iTempPerm283: i32,
	fTempPerm284: F32,
	iTempPerm285: i32,
	iTempPerm286: i32,
	fTempPerm287: F32,
	fHslider17: F32,
	iTempPerm288: i32,
	iTempPerm289: i32,
	fTempPerm290: F32,
	fTempPerm291: F32,
	fTempPerm292: F32,
	fTempPerm293: F32,
	iTempPerm294: i32,
	iTempPerm295: i32,
	fTempPerm296: F32,
	fTempPerm297: F32,
	iTempPerm298: i32,
	fTempPerm299: F32,
	iTempPerm300: i32,
	iRec55: [i32;2],
	iRec56: [i32;2],
	iRec57: [i32;2],
	iRec58: [i32;2],
	iTempPerm301: i32,
	iTempPerm302: i32,
	iTempPerm303: i32,
	fRec53: [F32;2],
	fTempPerm304: F32,
	fTempPerm305: F32,
	fTempPerm306: F32,
	fTempPerm307: F32,
	fTempPerm308: F32,
	iTempPerm309: i32,
	iTempPerm310: i32,
	fTempPerm311: F32,
	fTempPerm312: F32,
	iTempPerm313: i32,
	fTempPerm314: F32,
	iTempPerm315: i32,
	iTempPerm316: i32,
	iTempPerm317: i32,
	iTempPerm318: i32,
	iTempPerm319: i32,
	fRec62: [F32;2],
	fTempPerm320: F32,
	iTempPerm321: i32,
	iTempPerm322: i32,
	iTempPerm323: i32,
	iTempPerm324: i32,
	fTempPerm325: F32,
	fTempPerm326: F32,
	iTempPerm327: i32,
	fTempPerm328: F32,
	fTempPerm329: F32,
	iTempPerm330: i32,
	iTempPerm331: i32,
	fTempPerm332: F32,
	fTempPerm333: F32,
	iTempPerm334: i32,
	fTempPerm335: F32,
	fTempPerm336: F32,
	fTempPerm337: F32,
	fTempPerm338: F32,
	iTempPerm339: i32,
	fTempPerm340: F32,
	fTempPerm341: F32,
	fRec63: [F32;2],
	fRec64: [F32;2],
	fTempPerm342: F32,
	fTempPerm343: F32,
	iTempPerm344: i32,
	fTempPerm345: F32,
	fTempPerm346: F32,
	fRec61: [F32;2],
	iTempPerm347: i32,
	iTempPerm348: i32,
	fRec65: [F32;2],
	fTempPerm349: F32,
	fTempPerm350: F32,
	iTempPerm351: i32,
	iRec67: [i32;2],
	iRec73: [i32;2],
	iTempPerm352: i32,
	iTempPerm353: i32,
	iTempPerm354: i32,
	iTempPerm355: i32,
	iTempPerm356: i32,
	iTempPerm357: i32,
	iTempPerm358: i32,
	iTempPerm359: i32,
	iTempPerm360: i32,
	iTempPerm361: i32,
	iTempPerm362: i32,
	iTempPerm363: i32,
	iTempPerm364: i32,
	iTempPerm365: i32,
	fTempPerm366: F32,
	fTempPerm367: F32,
	iTempPerm368: i32,
	fTempPerm369: F32,
	iTempPerm370: i32,
	iTempPerm371: i32,
	fTempPerm372: F32,
	fHslider18: F32,
	iTempPerm373: i32,
	iTempPerm374: i32,
	fTempPerm375: F32,
	fTempPerm376: F32,
	fTempPerm377: F32,
	fTempPerm378: F32,
	iTempPerm379: i32,
	iTempPerm380: i32,
	fTempPerm381: F32,
	fTempPerm382: F32,
	iTempPerm383: i32,
	fTempPerm384: F32,
	iTempPerm385: i32,
	iRec68: [i32;2],
	iRec69: [i32;2],
	iRec70: [i32;2],
	iRec71: [i32;2],
	iTempPerm386: i32,
	iTempPerm387: i32,
	iTempPerm388: i32,
	fRec66: [F32;2],
	fTempPerm389: F32,
	fTempPerm390: F32,
	fTempPerm391: F32,
	fTempPerm392: F32,
	fTempPerm393: F32,
	iTempPerm394: i32,
	iTempPerm395: i32,
	fTempPerm396: F32,
	fTempPerm397: F32,
	iTempPerm398: i32,
	fTempPerm399: F32,
	iTempPerm400: i32,
	iTempPerm401: i32,
	iTempPerm402: i32,
	iTempPerm403: i32,
	iTempPerm404: i32,
	fRec75: [F32;2],
	fTempPerm405: F32,
	iTempPerm406: i32,
	iTempPerm407: i32,
	iTempPerm408: i32,
	iTempPerm409: i32,
	fTempPerm410: F32,
	fTempPerm411: F32,
	iTempPerm412: i32,
	fTempPerm413: F32,
	fTempPerm414: F32,
	iTempPerm415: i32,
	iTempPerm416: i32,
	fTempPerm417: F32,
	fTempPerm418: F32,
	iTempPerm419: i32,
	fTempPerm420: F32,
	fTempPerm421: F32,
	fTempPerm422: F32,
	fTempPerm423: F32,
	iTempPerm424: i32,
	fTempPerm425: F32,
	fTempPerm426: F32,
	fRec76: [F32;2],
	fRec77: [F32;2],
	fTempPerm427: F32,
	fTempPerm428: F32,
	iTempPerm429: i32,
	fTempPerm430: F32,
	fTempPerm431: F32,
	fRec74: [F32;2],
	iTempPerm432: i32,
	iTempPerm433: i32,
	fRec78: [F32;2],
	fTempPerm434: F32,
	fTempPerm435: F32,
	iTempPerm436: i32,
	iRec80: [i32;2],
	iRec86: [i32;2],
	iTempPerm437: i32,
	iTempPerm438: i32,
	iTempPerm439: i32,
	iTempPerm440: i32,
	iTempPerm441: i32,
	iTempPerm442: i32,
	iTempPerm443: i32,
	iTempPerm444: i32,
	iTempPerm445: i32,
	iTempPerm446: i32,
	iTempPerm447: i32,
	iTempPerm448: i32,
	iTempPerm449: i32,
	iTempPerm450: i32,
	fTempPerm451: F32,
	fTempPerm452: F32,
	iTempPerm453: i32,
	fTempPerm454: F32,
	iTempPerm455: i32,
	iTempPerm456: i32,
	fTempPerm457: F32,
	fHslider19: F32,
	iTempPerm458: i32,
	iTempPerm459: i32,
	fTempPerm460: F32,
	fTempPerm461: F32,
	fTempPerm462: F32,
	fTempPerm463: F32,
	iTempPerm464: i32,
	iTempPerm465: i32,
	fTempPerm466: F32,
	fTempPerm467: F32,
	iTempPerm468: i32,
	fTempPerm469: F32,
	iTempPerm470: i32,
	iRec81: [i32;2],
	iRec82: [i32;2],
	iRec83: [i32;2],
	iRec84: [i32;2],
	iTempPerm471: i32,
	iTempPerm472: i32,
	iTempPerm473: i32,
	fRec79: [F32;2],
	fTempPerm474: F32,
	fTempPerm475: F32,
	fTempPerm476: F32,
	fTempPerm477: F32,
	fTempPerm478: F32,
	iTempPerm479: i32,
	iTempPerm480: i32,
	fTempPerm481: F32,
	fTempPerm482: F32,
	iTempPerm483: i32,
	fTempPerm484: F32,
	iTempPerm485: i32,
	iTempPerm486: i32,
	iTempPerm487: i32,
	iTempPerm488: i32,
	iTempPerm489: i32,
	fRec88: [F32;2],
	fTempPerm490: F32,
	iTempPerm491: i32,
	iTempPerm492: i32,
	iTempPerm493: i32,
	iTempPerm494: i32,
	fTempPerm495: F32,
	fTempPerm496: F32,
	iTempPerm497: i32,
	fTempPerm498: F32,
	fTempPerm499: F32,
	iTempPerm500: i32,
	iTempPerm501: i32,
	fTempPerm502: F32,
	fTempPerm503: F32,
	iTempPerm504: i32,
	fTempPerm505: F32,
	fTempPerm506: F32,
	fTempPerm507: F32,
	fTempPerm508: F32,
	iTempPerm509: i32,
	fTempPerm510: F32,
	fTempPerm511: F32,
	fRec89: [F32;2],
	fRec90: [F32;2],
	fTempPerm512: F32,
	fTempPerm513: F32,
	iTempPerm514: i32,
	fTempPerm515: F32,
	fTempPerm516: F32,
	fRec87: [F32;2],
	iTempPerm517: i32,
	iTempPerm518: i32,
	fRec91: [F32;2],
	fTempPerm519: F32,
	fTempPerm520: F32,
	iTempPerm521: i32,
	iRec93: [i32;2],
	iRec99: [i32;2],
	iTempPerm522: i32,
	iTempPerm523: i32,
	iTempPerm524: i32,
	iTempPerm525: i32,
	iTempPerm526: i32,
	iTempPerm527: i32,
	iTempPerm528: i32,
	iTempPerm529: i32,
	iTempPerm530: i32,
	iTempPerm531: i32,
	iTempPerm532: i32,
	iTempPerm533: i32,
	iTempPerm534: i32,
	iTempPerm535: i32,
	fTempPerm536: F32,
	fTempPerm537: F32,
	iTempPerm538: i32,
	fTempPerm539: F32,
	iTempPerm540: i32,
	iTempPerm541: i32,
	fTempPerm542: F32,
	fHslider20: F32,
	iTempPerm543: i32,
	iTempPerm544: i32,
	fTempPerm545: F32,
	fTempPerm546: F32,
	fTempPerm547: F32,
	fTempPerm548: F32,
	iTempPerm549: i32,
	iTempPerm550: i32,
	fTempPerm551: F32,
	fTempPerm552: F32,
	iTempPerm553: i32,
	fTempPerm554: F32,
	iTempPerm555: i32,
	iRec94: [i32;2],
	iRec95: [i32;2],
	iRec96: [i32;2],
	iRec97: [i32;2],
	iTempPerm556: i32,
	iTempPerm557: i32,
	iTempPerm558: i32,
	fRec92: [F32;2],
	fTempPerm559: F32,
	fTempPerm560: F32,
	fTempPerm561: F32,
	fTempPerm562: F32,
	fTempPerm563: F32,
	iTempPerm564: i32,
	iTempPerm565: i32,
	fTempPerm566: F32,
	fTempPerm567: F32,
	iTempPerm568: i32,
	fTempPerm569: F32,
	iTempPerm570: i32,
	iTempPerm571: i32,
	iTempPerm572: i32,
	iTempPerm573: i32,
	iTempPerm574: i32,
	fRec101: [F32;2],
	fTempPerm575: F32,
	iTempPerm576: i32,
	iTempPerm577: i32,
	iTempPerm578: i32,
	iTempPerm579: i32,
	fTempPerm580: F32,
	fTempPerm581: F32,
	iTempPerm582: i32,
	fTempPerm583: F32,
	fTempPerm584: F32,
	iTempPerm585: i32,
	iTempPerm586: i32,
	fTempPerm587: F32,
	fTempPerm588: F32,
	iTempPerm589: i32,
	fTempPerm590: F32,
	fTempPerm591: F32,
	fTempPerm592: F32,
	fTempPerm593: F32,
	iTempPerm594: i32,
	fTempPerm595: F32,
	fTempPerm596: F32,
	fRec102: [F32;2],
	fRec103: [F32;2],
	fTempPerm597: F32,
	fTempPerm598: F32,
	iTempPerm599: i32,
	fTempPerm600: F32,
	fTempPerm601: F32,
	fRec100: [F32;2],
	iTempPerm602: i32,
	iTempPerm603: i32,
	fRec104: [F32;2],
	fTempPerm604: F32,
	fTempPerm605: F32,
	iTempPerm606: i32,
	iRec106: [i32;2],
	iRec112: [i32;2],
	iTempPerm607: i32,
	iTempPerm608: i32,
	iTempPerm609: i32,
	iTempPerm610: i32,
	iTempPerm611: i32,
	iTempPerm612: i32,
	iTempPerm613: i32,
	iTempPerm614: i32,
	iTempPerm615: i32,
	iTempPerm616: i32,
	iTempPerm617: i32,
	iTempPerm618: i32,
	iTempPerm619: i32,
	iTempPerm620: i32,
	fTempPerm621: F32,
	fTempPerm622: F32,
	iTempPerm623: i32,
	fTempPerm624: F32,
	iTempPerm625: i32,
	iTempPerm626: i32,
	fTempPerm627: F32,
	fHslider21: F32,
	iTempPerm628: i32,
	iTempPerm629: i32,
	fTempPerm630: F32,
	fTempPerm631: F32,
	fTempPerm632: F32,
	fTempPerm633: F32,
	iTempPerm634: i32,
	iTempPerm635: i32,
	fTempPerm636: F32,
	fTempPerm637: F32,
	iTempPerm638: i32,
	fTempPerm639: F32,
	iTempPerm640: i32,
	iRec107: [i32;2],
	iRec108: [i32;2],
	iRec109: [i32;2],
	iRec110: [i32;2],
	iTempPerm641: i32,
	iTempPerm642: i32,
	iTempPerm643: i32,
	fRec105: [F32;2],
	fTempPerm644: F32,
	fTempPerm645: F32,
	fTempPerm646: F32,
	fTempPerm647: F32,
	fTempPerm648: F32,
	iTempPerm649: i32,
	iTempPerm650: i32,
	fTempPerm651: F32,
	fTempPerm652: F32,
	iTempPerm653: i32,
	fTempPerm654: F32,
	iTempPerm655: i32,
	iTempPerm656: i32,
	iTempPerm657: i32,
	iTempPerm658: i32,
	iTempPerm659: i32,
	fRec114: [F32;2],
	fTempPerm660: F32,
	iTempPerm661: i32,
	iTempPerm662: i32,
	iTempPerm663: i32,
	iTempPerm664: i32,
	fTempPerm665: F32,
	fTempPerm666: F32,
	iTempPerm667: i32,
	fTempPerm668: F32,
	fTempPerm669: F32,
	iTempPerm670: i32,
	iTempPerm671: i32,
	fTempPerm672: F32,
	fTempPerm673: F32,
	iTempPerm674: i32,
	fTempPerm675: F32,
	fTempPerm676: F32,
	fTempPerm677: F32,
	fTempPerm678: F32,
	iTempPerm679: i32,
	fTempPerm680: F32,
	fTempPerm681: F32,
	fRec115: [F32;2],
	fRec116: [F32;2],
	fTempPerm682: F32,
	fTempPerm683: F32,
	iTempPerm684: i32,
	fTempPerm685: F32,
	fTempPerm686: F32,
	fRec113: [F32;2],
	iTempPerm687: i32,
	iTempPerm688: i32,
	fRec117: [F32;2],
	fTempPerm689: F32,
	fTempPerm690: F32,
	fTempPerm691: F32,
	fTempPerm692: F32,
	fTempPerm693: F32,
	fTempPerm694: F32,
	fTempPerm695: F32,
	fTempPerm696: F32,
	fTempPerm697: F32,
	fTempPerm698: F32,
}

impl FaustDsp for CallsDsp {
	type T = F32;
		
	fn new() -> CallsDsp { 
		CallsDsp {
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fRec0: [0.0;2],
			fHslider0: 0.0,
			fRec1: [0.0;2],
			fHslider1: 0.0,
			fRec2: [0.0;2],
			fHslider2: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			fRec3: [0.0;2],
			iTempPerm0: 0,
			iRec5: [0;2],
			fHslider3: 0.0,
			fRec11: [0.0;2],
			fHslider4: 0.0,
			fRec12: [0.0;2],
			fHslider5: 0.0,
			fRec13: [0.0;2],
			fHslider6: 0.0,
			fRec14: [0.0;2],
			fHslider7: 0.0,
			fRec15: [0.0;2],
			fHslider8: 0.0,
			fRec16: [0.0;2],
			fHslider9: 0.0,
			fRec17: [0.0;2],
			iTempPerm1: 0,
			iTempPerm2: 0,
			fHslider10: 0.0,
			iRec18: [0;2],
			iTempPerm3: 0,
			iTempPerm4: 0,
			iTempPerm5: 0,
			iTempPerm6: 0,
			iTempPerm7: 0,
			iTempPerm8: 0,
			iTempPerm9: 0,
			iTempPerm10: 0,
			iTempPerm11: 0,
			iTempPerm12: 0,
			iTempPerm13: 0,
			iTempPerm14: 0,
			iTempPerm15: 0,
			iTempPerm16: 0,
			fTempPerm17: 0.0,
			fTempPerm18: 0.0,
			iTempPerm19: 0,
			fHslider11: 0.0,
			fRec19: [0.0;2],
			fTempPerm20: 0.0,
			fTempPerm21: 0.0,
			iTempPerm22: 0,
			iTempPerm23: 0,
			fTempPerm24: 0.0,
			fHslider12: 0.0,
			iTempPerm25: 0,
			iTempPerm26: 0,
			fTempPerm27: 0.0,
			fTempPerm28: 0.0,
			fTempPerm29: 0.0,
			fTempPerm30: 0.0,
			fHslider13: 0.0,
			fRec20: [0.0;2],
			fTempPerm31: 0.0,
			fTempPerm32: 0.0,
			iTempPerm33: 0,
			fTempPerm34: 0.0,
			fTempPerm35: 0.0,
			fTempPerm36: 0.0,
			iTempPerm37: 0,
			iTempPerm38: 0,
			fTempPerm39: 0.0,
			fTempPerm40: 0.0,
			iTempPerm41: 0,
			fTempPerm42: 0.0,
			fConst5: 0.0,
			iTempPerm43: 0,
			iTempPerm44: 0,
			iRec6: [0;2],
			iRec7: [0;2],
			iRec8: [0;2],
			iRec9: [0;2],
			iTempPerm45: 0,
			iTempPerm46: 0,
			iTempPerm47: 0,
			fRec4: [0.0;2],
			fHslider14: 0.0,
			fRec21: [0.0;2],
			fTempPerm48: 0.0,
			fTempPerm49: 0.0,
			fTempPerm50: 0.0,
			fTempPerm51: 0.0,
			fTempPerm52: 0.0,
			fConst6: 0.0,
			fTempPerm53: 0.0,
			iTempPerm54: 0,
			iTempPerm55: 0,
			fTempPerm56: 0.0,
			fTempPerm57: 0.0,
			iTempPerm58: 0,
			fTempPerm59: 0.0,
			iTempPerm60: 0,
			iTempPerm61: 0,
			iTempPerm62: 0,
			iTempPerm63: 0,
			iTempPerm64: 0,
			fRec23: [0.0;2],
			fTempPerm65: 0.0,
			iTempPerm66: 0,
			iTempPerm67: 0,
			iTempPerm68: 0,
			iTempPerm69: 0,
			fTempPerm70: 0.0,
			fTempPerm71: 0.0,
			iTempPerm72: 0,
			fTempPerm73: 0.0,
			fTempPerm74: 0.0,
			iTempPerm75: 0,
			iTempPerm76: 0,
			fTempPerm77: 0.0,
			fTempPerm78: 0.0,
			iTempPerm79: 0,
			fTempPerm80: 0.0,
			fTempPerm81: 0.0,
			fTempPerm82: 0.0,
			fTempPerm83: 0.0,
			iTempPerm84: 0,
			fTempPerm85: 0.0,
			fTempPerm86: 0.0,
			fRec24: [0.0;2],
			fRec25: [0.0;2],
			fTempPerm87: 0.0,
			fTempPerm88: 0.0,
			iTempPerm89: 0,
			fTempPerm90: 0.0,
			fConst7: 0.0,
			fTempPerm91: 0.0,
			fRec22: [0.0;2],
			iTempPerm92: 0,
			iTempPerm93: 0,
			fRec26: [0.0;2],
			fTempPerm94: 0.0,
			fTempPerm95: 0.0,
			iTempPerm96: 0,
			iRec28: [0;2],
			iRec34: [0;2],
			iTempPerm97: 0,
			iTempPerm98: 0,
			iTempPerm99: 0,
			iTempPerm100: 0,
			iTempPerm101: 0,
			iTempPerm102: 0,
			iTempPerm103: 0,
			iTempPerm104: 0,
			iTempPerm105: 0,
			iTempPerm106: 0,
			iTempPerm107: 0,
			iTempPerm108: 0,
			iTempPerm109: 0,
			iTempPerm110: 0,
			fTempPerm111: 0.0,
			fTempPerm112: 0.0,
			iTempPerm113: 0,
			fTempPerm114: 0.0,
			iTempPerm115: 0,
			iTempPerm116: 0,
			fTempPerm117: 0.0,
			fHslider15: 0.0,
			iTempPerm118: 0,
			iTempPerm119: 0,
			fTempPerm120: 0.0,
			fTempPerm121: 0.0,
			fTempPerm122: 0.0,
			fTempPerm123: 0.0,
			iTempPerm124: 0,
			iTempPerm125: 0,
			fTempPerm126: 0.0,
			fTempPerm127: 0.0,
			iTempPerm128: 0,
			fTempPerm129: 0.0,
			iTempPerm130: 0,
			iRec29: [0;2],
			iRec30: [0;2],
			iRec31: [0;2],
			iRec32: [0;2],
			iTempPerm131: 0,
			iTempPerm132: 0,
			iTempPerm133: 0,
			fRec27: [0.0;2],
			fTempPerm134: 0.0,
			fTempPerm135: 0.0,
			fTempPerm136: 0.0,
			fTempPerm137: 0.0,
			fTempPerm138: 0.0,
			iTempPerm139: 0,
			iTempPerm140: 0,
			fTempPerm141: 0.0,
			fTempPerm142: 0.0,
			iTempPerm143: 0,
			fTempPerm144: 0.0,
			iTempPerm145: 0,
			iTempPerm146: 0,
			iTempPerm147: 0,
			iTempPerm148: 0,
			iTempPerm149: 0,
			fRec36: [0.0;2],
			fTempPerm150: 0.0,
			iTempPerm151: 0,
			iTempPerm152: 0,
			iTempPerm153: 0,
			iTempPerm154: 0,
			fTempPerm155: 0.0,
			fTempPerm156: 0.0,
			iTempPerm157: 0,
			fTempPerm158: 0.0,
			fTempPerm159: 0.0,
			iTempPerm160: 0,
			iTempPerm161: 0,
			fTempPerm162: 0.0,
			fTempPerm163: 0.0,
			iTempPerm164: 0,
			fTempPerm165: 0.0,
			fTempPerm166: 0.0,
			fTempPerm167: 0.0,
			fTempPerm168: 0.0,
			iTempPerm169: 0,
			fTempPerm170: 0.0,
			fTempPerm171: 0.0,
			fRec37: [0.0;2],
			fRec38: [0.0;2],
			fTempPerm172: 0.0,
			fTempPerm173: 0.0,
			iTempPerm174: 0,
			fTempPerm175: 0.0,
			fTempPerm176: 0.0,
			fRec35: [0.0;2],
			iTempPerm177: 0,
			iTempPerm178: 0,
			fRec39: [0.0;2],
			fTempPerm179: 0.0,
			fTempPerm180: 0.0,
			iTempPerm181: 0,
			iRec41: [0;2],
			iRec47: [0;2],
			iTempPerm182: 0,
			iTempPerm183: 0,
			iTempPerm184: 0,
			iTempPerm185: 0,
			iTempPerm186: 0,
			iTempPerm187: 0,
			iTempPerm188: 0,
			iTempPerm189: 0,
			iTempPerm190: 0,
			iTempPerm191: 0,
			iTempPerm192: 0,
			iTempPerm193: 0,
			iTempPerm194: 0,
			iTempPerm195: 0,
			fTempPerm196: 0.0,
			fTempPerm197: 0.0,
			iTempPerm198: 0,
			fTempPerm199: 0.0,
			iTempPerm200: 0,
			iTempPerm201: 0,
			fTempPerm202: 0.0,
			fHslider16: 0.0,
			iTempPerm203: 0,
			iTempPerm204: 0,
			fTempPerm205: 0.0,
			fTempPerm206: 0.0,
			fTempPerm207: 0.0,
			fTempPerm208: 0.0,
			iTempPerm209: 0,
			iTempPerm210: 0,
			fTempPerm211: 0.0,
			fTempPerm212: 0.0,
			iTempPerm213: 0,
			fTempPerm214: 0.0,
			iTempPerm215: 0,
			iRec42: [0;2],
			iRec43: [0;2],
			iRec44: [0;2],
			iRec45: [0;2],
			iTempPerm216: 0,
			iTempPerm217: 0,
			iTempPerm218: 0,
			fRec40: [0.0;2],
			fTempPerm219: 0.0,
			fTempPerm220: 0.0,
			fTempPerm221: 0.0,
			fTempPerm222: 0.0,
			fTempPerm223: 0.0,
			iTempPerm224: 0,
			iTempPerm225: 0,
			fTempPerm226: 0.0,
			fTempPerm227: 0.0,
			iTempPerm228: 0,
			fTempPerm229: 0.0,
			iTempPerm230: 0,
			iTempPerm231: 0,
			iTempPerm232: 0,
			iTempPerm233: 0,
			iTempPerm234: 0,
			fRec49: [0.0;2],
			fTempPerm235: 0.0,
			iTempPerm236: 0,
			iTempPerm237: 0,
			iTempPerm238: 0,
			iTempPerm239: 0,
			fTempPerm240: 0.0,
			fTempPerm241: 0.0,
			iTempPerm242: 0,
			fTempPerm243: 0.0,
			fTempPerm244: 0.0,
			iTempPerm245: 0,
			iTempPerm246: 0,
			fTempPerm247: 0.0,
			fTempPerm248: 0.0,
			iTempPerm249: 0,
			fTempPerm250: 0.0,
			fTempPerm251: 0.0,
			fTempPerm252: 0.0,
			fTempPerm253: 0.0,
			iTempPerm254: 0,
			fTempPerm255: 0.0,
			fTempPerm256: 0.0,
			fRec50: [0.0;2],
			fRec51: [0.0;2],
			fTempPerm257: 0.0,
			fTempPerm258: 0.0,
			iTempPerm259: 0,
			fTempPerm260: 0.0,
			fTempPerm261: 0.0,
			fRec48: [0.0;2],
			iTempPerm262: 0,
			iTempPerm263: 0,
			fRec52: [0.0;2],
			fTempPerm264: 0.0,
			fTempPerm265: 0.0,
			iTempPerm266: 0,
			iRec54: [0;2],
			iRec60: [0;2],
			iTempPerm267: 0,
			iTempPerm268: 0,
			iTempPerm269: 0,
			iTempPerm270: 0,
			iTempPerm271: 0,
			iTempPerm272: 0,
			iTempPerm273: 0,
			iTempPerm274: 0,
			iTempPerm275: 0,
			iTempPerm276: 0,
			iTempPerm277: 0,
			iTempPerm278: 0,
			iTempPerm279: 0,
			iTempPerm280: 0,
			fTempPerm281: 0.0,
			fTempPerm282: 0.0,
			iTempPerm283: 0,
			fTempPerm284: 0.0,
			iTempPerm285: 0,
			iTempPerm286: 0,
			fTempPerm287: 0.0,
			fHslider17: 0.0,
			iTempPerm288: 0,
			iTempPerm289: 0,
			fTempPerm290: 0.0,
			fTempPerm291: 0.0,
			fTempPerm292: 0.0,
			fTempPerm293: 0.0,
			iTempPerm294: 0,
			iTempPerm295: 0,
			fTempPerm296: 0.0,
			fTempPerm297: 0.0,
			iTempPerm298: 0,
			fTempPerm299: 0.0,
			iTempPerm300: 0,
			iRec55: [0;2],
			iRec56: [0;2],
			iRec57: [0;2],
			iRec58: [0;2],
			iTempPerm301: 0,
			iTempPerm302: 0,
			iTempPerm303: 0,
			fRec53: [0.0;2],
			fTempPerm304: 0.0,
			fTempPerm305: 0.0,
			fTempPerm306: 0.0,
			fTempPerm307: 0.0,
			fTempPerm308: 0.0,
			iTempPerm309: 0,
			iTempPerm310: 0,
			fTempPerm311: 0.0,
			fTempPerm312: 0.0,
			iTempPerm313: 0,
			fTempPerm314: 0.0,
			iTempPerm315: 0,
			iTempPerm316: 0,
			iTempPerm317: 0,
			iTempPerm318: 0,
			iTempPerm319: 0,
			fRec62: [0.0;2],
			fTempPerm320: 0.0,
			iTempPerm321: 0,
			iTempPerm322: 0,
			iTempPerm323: 0,
			iTempPerm324: 0,
			fTempPerm325: 0.0,
			fTempPerm326: 0.0,
			iTempPerm327: 0,
			fTempPerm328: 0.0,
			fTempPerm329: 0.0,
			iTempPerm330: 0,
			iTempPerm331: 0,
			fTempPerm332: 0.0,
			fTempPerm333: 0.0,
			iTempPerm334: 0,
			fTempPerm335: 0.0,
			fTempPerm336: 0.0,
			fTempPerm337: 0.0,
			fTempPerm338: 0.0,
			iTempPerm339: 0,
			fTempPerm340: 0.0,
			fTempPerm341: 0.0,
			fRec63: [0.0;2],
			fRec64: [0.0;2],
			fTempPerm342: 0.0,
			fTempPerm343: 0.0,
			iTempPerm344: 0,
			fTempPerm345: 0.0,
			fTempPerm346: 0.0,
			fRec61: [0.0;2],
			iTempPerm347: 0,
			iTempPerm348: 0,
			fRec65: [0.0;2],
			fTempPerm349: 0.0,
			fTempPerm350: 0.0,
			iTempPerm351: 0,
			iRec67: [0;2],
			iRec73: [0;2],
			iTempPerm352: 0,
			iTempPerm353: 0,
			iTempPerm354: 0,
			iTempPerm355: 0,
			iTempPerm356: 0,
			iTempPerm357: 0,
			iTempPerm358: 0,
			iTempPerm359: 0,
			iTempPerm360: 0,
			iTempPerm361: 0,
			iTempPerm362: 0,
			iTempPerm363: 0,
			iTempPerm364: 0,
			iTempPerm365: 0,
			fTempPerm366: 0.0,
			fTempPerm367: 0.0,
			iTempPerm368: 0,
			fTempPerm369: 0.0,
			iTempPerm370: 0,
			iTempPerm371: 0,
			fTempPerm372: 0.0,
			fHslider18: 0.0,
			iTempPerm373: 0,
			iTempPerm374: 0,
			fTempPerm375: 0.0,
			fTempPerm376: 0.0,
			fTempPerm377: 0.0,
			fTempPerm378: 0.0,
			iTempPerm379: 0,
			iTempPerm380: 0,
			fTempPerm381: 0.0,
			fTempPerm382: 0.0,
			iTempPerm383: 0,
			fTempPerm384: 0.0,
			iTempPerm385: 0,
			iRec68: [0;2],
			iRec69: [0;2],
			iRec70: [0;2],
			iRec71: [0;2],
			iTempPerm386: 0,
			iTempPerm387: 0,
			iTempPerm388: 0,
			fRec66: [0.0;2],
			fTempPerm389: 0.0,
			fTempPerm390: 0.0,
			fTempPerm391: 0.0,
			fTempPerm392: 0.0,
			fTempPerm393: 0.0,
			iTempPerm394: 0,
			iTempPerm395: 0,
			fTempPerm396: 0.0,
			fTempPerm397: 0.0,
			iTempPerm398: 0,
			fTempPerm399: 0.0,
			iTempPerm400: 0,
			iTempPerm401: 0,
			iTempPerm402: 0,
			iTempPerm403: 0,
			iTempPerm404: 0,
			fRec75: [0.0;2],
			fTempPerm405: 0.0,
			iTempPerm406: 0,
			iTempPerm407: 0,
			iTempPerm408: 0,
			iTempPerm409: 0,
			fTempPerm410: 0.0,
			fTempPerm411: 0.0,
			iTempPerm412: 0,
			fTempPerm413: 0.0,
			fTempPerm414: 0.0,
			iTempPerm415: 0,
			iTempPerm416: 0,
			fTempPerm417: 0.0,
			fTempPerm418: 0.0,
			iTempPerm419: 0,
			fTempPerm420: 0.0,
			fTempPerm421: 0.0,
			fTempPerm422: 0.0,
			fTempPerm423: 0.0,
			iTempPerm424: 0,
			fTempPerm425: 0.0,
			fTempPerm426: 0.0,
			fRec76: [0.0;2],
			fRec77: [0.0;2],
			fTempPerm427: 0.0,
			fTempPerm428: 0.0,
			iTempPerm429: 0,
			fTempPerm430: 0.0,
			fTempPerm431: 0.0,
			fRec74: [0.0;2],
			iTempPerm432: 0,
			iTempPerm433: 0,
			fRec78: [0.0;2],
			fTempPerm434: 0.0,
			fTempPerm435: 0.0,
			iTempPerm436: 0,
			iRec80: [0;2],
			iRec86: [0;2],
			iTempPerm437: 0,
			iTempPerm438: 0,
			iTempPerm439: 0,
			iTempPerm440: 0,
			iTempPerm441: 0,
			iTempPerm442: 0,
			iTempPerm443: 0,
			iTempPerm444: 0,
			iTempPerm445: 0,
			iTempPerm446: 0,
			iTempPerm447: 0,
			iTempPerm448: 0,
			iTempPerm449: 0,
			iTempPerm450: 0,
			fTempPerm451: 0.0,
			fTempPerm452: 0.0,
			iTempPerm453: 0,
			fTempPerm454: 0.0,
			iTempPerm455: 0,
			iTempPerm456: 0,
			fTempPerm457: 0.0,
			fHslider19: 0.0,
			iTempPerm458: 0,
			iTempPerm459: 0,
			fTempPerm460: 0.0,
			fTempPerm461: 0.0,
			fTempPerm462: 0.0,
			fTempPerm463: 0.0,
			iTempPerm464: 0,
			iTempPerm465: 0,
			fTempPerm466: 0.0,
			fTempPerm467: 0.0,
			iTempPerm468: 0,
			fTempPerm469: 0.0,
			iTempPerm470: 0,
			iRec81: [0;2],
			iRec82: [0;2],
			iRec83: [0;2],
			iRec84: [0;2],
			iTempPerm471: 0,
			iTempPerm472: 0,
			iTempPerm473: 0,
			fRec79: [0.0;2],
			fTempPerm474: 0.0,
			fTempPerm475: 0.0,
			fTempPerm476: 0.0,
			fTempPerm477: 0.0,
			fTempPerm478: 0.0,
			iTempPerm479: 0,
			iTempPerm480: 0,
			fTempPerm481: 0.0,
			fTempPerm482: 0.0,
			iTempPerm483: 0,
			fTempPerm484: 0.0,
			iTempPerm485: 0,
			iTempPerm486: 0,
			iTempPerm487: 0,
			iTempPerm488: 0,
			iTempPerm489: 0,
			fRec88: [0.0;2],
			fTempPerm490: 0.0,
			iTempPerm491: 0,
			iTempPerm492: 0,
			iTempPerm493: 0,
			iTempPerm494: 0,
			fTempPerm495: 0.0,
			fTempPerm496: 0.0,
			iTempPerm497: 0,
			fTempPerm498: 0.0,
			fTempPerm499: 0.0,
			iTempPerm500: 0,
			iTempPerm501: 0,
			fTempPerm502: 0.0,
			fTempPerm503: 0.0,
			iTempPerm504: 0,
			fTempPerm505: 0.0,
			fTempPerm506: 0.0,
			fTempPerm507: 0.0,
			fTempPerm508: 0.0,
			iTempPerm509: 0,
			fTempPerm510: 0.0,
			fTempPerm511: 0.0,
			fRec89: [0.0;2],
			fRec90: [0.0;2],
			fTempPerm512: 0.0,
			fTempPerm513: 0.0,
			iTempPerm514: 0,
			fTempPerm515: 0.0,
			fTempPerm516: 0.0,
			fRec87: [0.0;2],
			iTempPerm517: 0,
			iTempPerm518: 0,
			fRec91: [0.0;2],
			fTempPerm519: 0.0,
			fTempPerm520: 0.0,
			iTempPerm521: 0,
			iRec93: [0;2],
			iRec99: [0;2],
			iTempPerm522: 0,
			iTempPerm523: 0,
			iTempPerm524: 0,
			iTempPerm525: 0,
			iTempPerm526: 0,
			iTempPerm527: 0,
			iTempPerm528: 0,
			iTempPerm529: 0,
			iTempPerm530: 0,
			iTempPerm531: 0,
			iTempPerm532: 0,
			iTempPerm533: 0,
			iTempPerm534: 0,
			iTempPerm535: 0,
			fTempPerm536: 0.0,
			fTempPerm537: 0.0,
			iTempPerm538: 0,
			fTempPerm539: 0.0,
			iTempPerm540: 0,
			iTempPerm541: 0,
			fTempPerm542: 0.0,
			fHslider20: 0.0,
			iTempPerm543: 0,
			iTempPerm544: 0,
			fTempPerm545: 0.0,
			fTempPerm546: 0.0,
			fTempPerm547: 0.0,
			fTempPerm548: 0.0,
			iTempPerm549: 0,
			iTempPerm550: 0,
			fTempPerm551: 0.0,
			fTempPerm552: 0.0,
			iTempPerm553: 0,
			fTempPerm554: 0.0,
			iTempPerm555: 0,
			iRec94: [0;2],
			iRec95: [0;2],
			iRec96: [0;2],
			iRec97: [0;2],
			iTempPerm556: 0,
			iTempPerm557: 0,
			iTempPerm558: 0,
			fRec92: [0.0;2],
			fTempPerm559: 0.0,
			fTempPerm560: 0.0,
			fTempPerm561: 0.0,
			fTempPerm562: 0.0,
			fTempPerm563: 0.0,
			iTempPerm564: 0,
			iTempPerm565: 0,
			fTempPerm566: 0.0,
			fTempPerm567: 0.0,
			iTempPerm568: 0,
			fTempPerm569: 0.0,
			iTempPerm570: 0,
			iTempPerm571: 0,
			iTempPerm572: 0,
			iTempPerm573: 0,
			iTempPerm574: 0,
			fRec101: [0.0;2],
			fTempPerm575: 0.0,
			iTempPerm576: 0,
			iTempPerm577: 0,
			iTempPerm578: 0,
			iTempPerm579: 0,
			fTempPerm580: 0.0,
			fTempPerm581: 0.0,
			iTempPerm582: 0,
			fTempPerm583: 0.0,
			fTempPerm584: 0.0,
			iTempPerm585: 0,
			iTempPerm586: 0,
			fTempPerm587: 0.0,
			fTempPerm588: 0.0,
			iTempPerm589: 0,
			fTempPerm590: 0.0,
			fTempPerm591: 0.0,
			fTempPerm592: 0.0,
			fTempPerm593: 0.0,
			iTempPerm594: 0,
			fTempPerm595: 0.0,
			fTempPerm596: 0.0,
			fRec102: [0.0;2],
			fRec103: [0.0;2],
			fTempPerm597: 0.0,
			fTempPerm598: 0.0,
			iTempPerm599: 0,
			fTempPerm600: 0.0,
			fTempPerm601: 0.0,
			fRec100: [0.0;2],
			iTempPerm602: 0,
			iTempPerm603: 0,
			fRec104: [0.0;2],
			fTempPerm604: 0.0,
			fTempPerm605: 0.0,
			iTempPerm606: 0,
			iRec106: [0;2],
			iRec112: [0;2],
			iTempPerm607: 0,
			iTempPerm608: 0,
			iTempPerm609: 0,
			iTempPerm610: 0,
			iTempPerm611: 0,
			iTempPerm612: 0,
			iTempPerm613: 0,
			iTempPerm614: 0,
			iTempPerm615: 0,
			iTempPerm616: 0,
			iTempPerm617: 0,
			iTempPerm618: 0,
			iTempPerm619: 0,
			iTempPerm620: 0,
			fTempPerm621: 0.0,
			fTempPerm622: 0.0,
			iTempPerm623: 0,
			fTempPerm624: 0.0,
			iTempPerm625: 0,
			iTempPerm626: 0,
			fTempPerm627: 0.0,
			fHslider21: 0.0,
			iTempPerm628: 0,
			iTempPerm629: 0,
			fTempPerm630: 0.0,
			fTempPerm631: 0.0,
			fTempPerm632: 0.0,
			fTempPerm633: 0.0,
			iTempPerm634: 0,
			iTempPerm635: 0,
			fTempPerm636: 0.0,
			fTempPerm637: 0.0,
			iTempPerm638: 0,
			fTempPerm639: 0.0,
			iTempPerm640: 0,
			iRec107: [0;2],
			iRec108: [0;2],
			iRec109: [0;2],
			iRec110: [0;2],
			iTempPerm641: 0,
			iTempPerm642: 0,
			iTempPerm643: 0,
			fRec105: [0.0;2],
			fTempPerm644: 0.0,
			fTempPerm645: 0.0,
			fTempPerm646: 0.0,
			fTempPerm647: 0.0,
			fTempPerm648: 0.0,
			iTempPerm649: 0,
			iTempPerm650: 0,
			fTempPerm651: 0.0,
			fTempPerm652: 0.0,
			iTempPerm653: 0,
			fTempPerm654: 0.0,
			iTempPerm655: 0,
			iTempPerm656: 0,
			iTempPerm657: 0,
			iTempPerm658: 0,
			iTempPerm659: 0,
			fRec114: [0.0;2],
			fTempPerm660: 0.0,
			iTempPerm661: 0,
			iTempPerm662: 0,
			iTempPerm663: 0,
			iTempPerm664: 0,
			fTempPerm665: 0.0,
			fTempPerm666: 0.0,
			iTempPerm667: 0,
			fTempPerm668: 0.0,
			fTempPerm669: 0.0,
			iTempPerm670: 0,
			iTempPerm671: 0,
			fTempPerm672: 0.0,
			fTempPerm673: 0.0,
			iTempPerm674: 0,
			fTempPerm675: 0.0,
			fTempPerm676: 0.0,
			fTempPerm677: 0.0,
			fTempPerm678: 0.0,
			iTempPerm679: 0,
			fTempPerm680: 0.0,
			fTempPerm681: 0.0,
			fRec115: [0.0;2],
			fRec116: [0.0;2],
			fTempPerm682: 0.0,
			fTempPerm683: 0.0,
			iTempPerm684: 0,
			fTempPerm685: 0.0,
			fTempPerm686: 0.0,
			fRec113: [0.0;2],
			iTempPerm687: 0,
			iTempPerm688: 0,
			fRec117: [0.0;2],
			fTempPerm689: 0.0,
			fTempPerm690: 0.0,
			fTempPerm691: 0.0,
			fTempPerm692: 0.0,
			fTempPerm693: 0.0,
			fTempPerm694: 0.0,
			fTempPerm695: 0.0,
			fTempPerm696: 0.0,
			fTempPerm697: 0.0,
			fTempPerm698: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/amp_follower_ar:author", r"Jonatan Liljedahl, revised by Romain Michon");
		m.declare("analyzers.lib/name", r"Faust Analyzer Library");
		m.declare("analyzers.lib/version", r"1.2.0");
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -cn CallsDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("filename", r"plugin-calls.dsp");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"plugin-calls");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/onePoleSwitching:author", r"Jonatan Liljedahl, revised by Dario Sanfilippo");
		m.declare("signals.lib/onePoleSwitching:licence", r"STK-4.3");
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
		self.fHslider0 = 1.0;
		self.fHslider1 = -24.0;
		self.fHslider2 = 0.0;
		self.fHslider3 = 0.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 0.0;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 2e+03;
		self.fHslider11 = 1.0;
		self.fHslider12 = 130.81;
		self.fHslider13 = 0.45;
		self.fHslider14 = 1.0;
		self.fHslider15 = 130.81;
		self.fHslider16 = 130.81;
		self.fHslider17 = 130.81;
		self.fHslider18 = 130.81;
		self.fHslider19 = 130.81;
		self.fHslider20 = 130.81;
		self.fHslider21 = 130.81;
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
		self.iTempPerm0 = 0;
		for l5 in 0..2 {
			self.iRec5[l5 as usize] = 0;
		}
		for l6 in 0..2 {
			self.fRec11[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec12[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec13[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec14[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec15[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec16[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec17[l12 as usize] = 0.0;
		}
		self.iTempPerm1 = 0;
		self.iTempPerm2 = 0;
		for l13 in 0..2 {
			self.iRec18[l13 as usize] = 0;
		}
		self.iTempPerm3 = 0;
		self.iTempPerm4 = 0;
		self.iTempPerm5 = 0;
		self.iTempPerm6 = 0;
		self.iTempPerm7 = 0;
		self.iTempPerm8 = 0;
		self.iTempPerm9 = 0;
		self.iTempPerm10 = 0;
		self.iTempPerm11 = 0;
		self.iTempPerm12 = 0;
		self.iTempPerm13 = 0;
		self.iTempPerm14 = 0;
		self.iTempPerm15 = 0;
		self.iTempPerm16 = 0;
		self.fTempPerm17 = 0.0;
		self.fTempPerm18 = 0.0;
		self.iTempPerm19 = 0;
		for l14 in 0..2 {
			self.fRec19[l14 as usize] = 0.0;
		}
		self.fTempPerm20 = 0.0;
		self.fTempPerm21 = 0.0;
		self.iTempPerm22 = 0;
		self.iTempPerm23 = 0;
		self.fTempPerm24 = 0.0;
		self.iTempPerm25 = 0;
		self.iTempPerm26 = 0;
		self.fTempPerm27 = 0.0;
		self.fTempPerm28 = 0.0;
		self.fTempPerm29 = 0.0;
		self.fTempPerm30 = 0.0;
		for l15 in 0..2 {
			self.fRec20[l15 as usize] = 0.0;
		}
		self.fTempPerm31 = 0.0;
		self.fTempPerm32 = 0.0;
		self.iTempPerm33 = 0;
		self.fTempPerm34 = 0.0;
		self.fTempPerm35 = 0.0;
		self.fTempPerm36 = 0.0;
		self.iTempPerm37 = 0;
		self.iTempPerm38 = 0;
		self.fTempPerm39 = 0.0;
		self.fTempPerm40 = 0.0;
		self.iTempPerm41 = 0;
		self.fTempPerm42 = 0.0;
		self.iTempPerm43 = 0;
		self.iTempPerm44 = 0;
		for l16 in 0..2 {
			self.iRec6[l16 as usize] = 0;
		}
		for l17 in 0..2 {
			self.iRec7[l17 as usize] = 0;
		}
		for l18 in 0..2 {
			self.iRec8[l18 as usize] = 0;
		}
		for l19 in 0..2 {
			self.iRec9[l19 as usize] = 0;
		}
		self.iTempPerm45 = 0;
		self.iTempPerm46 = 0;
		self.iTempPerm47 = 0;
		for l20 in 0..2 {
			self.fRec4[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec21[l21 as usize] = 0.0;
		}
		self.fTempPerm48 = 0.0;
		self.fTempPerm49 = 0.0;
		self.fTempPerm50 = 0.0;
		self.fTempPerm51 = 0.0;
		self.fTempPerm52 = 0.0;
		self.fTempPerm53 = 0.0;
		self.iTempPerm54 = 0;
		self.iTempPerm55 = 0;
		self.fTempPerm56 = 0.0;
		self.fTempPerm57 = 0.0;
		self.iTempPerm58 = 0;
		self.fTempPerm59 = 0.0;
		self.iTempPerm60 = 0;
		self.iTempPerm61 = 0;
		self.iTempPerm62 = 0;
		self.iTempPerm63 = 0;
		self.iTempPerm64 = 0;
		for l22 in 0..2 {
			self.fRec23[l22 as usize] = 0.0;
		}
		self.fTempPerm65 = 0.0;
		self.iTempPerm66 = 0;
		self.iTempPerm67 = 0;
		self.iTempPerm68 = 0;
		self.iTempPerm69 = 0;
		self.fTempPerm70 = 0.0;
		self.fTempPerm71 = 0.0;
		self.iTempPerm72 = 0;
		self.fTempPerm73 = 0.0;
		self.fTempPerm74 = 0.0;
		self.iTempPerm75 = 0;
		self.iTempPerm76 = 0;
		self.fTempPerm77 = 0.0;
		self.fTempPerm78 = 0.0;
		self.iTempPerm79 = 0;
		self.fTempPerm80 = 0.0;
		self.fTempPerm81 = 0.0;
		self.fTempPerm82 = 0.0;
		self.fTempPerm83 = 0.0;
		self.iTempPerm84 = 0;
		self.fTempPerm85 = 0.0;
		self.fTempPerm86 = 0.0;
		for l23 in 0..2 {
			self.fRec24[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec25[l24 as usize] = 0.0;
		}
		self.fTempPerm87 = 0.0;
		self.fTempPerm88 = 0.0;
		self.iTempPerm89 = 0;
		self.fTempPerm90 = 0.0;
		self.fTempPerm91 = 0.0;
		for l25 in 0..2 {
			self.fRec22[l25 as usize] = 0.0;
		}
		self.iTempPerm92 = 0;
		self.iTempPerm93 = 0;
		for l26 in 0..2 {
			self.fRec26[l26 as usize] = 0.0;
		}
		self.fTempPerm94 = 0.0;
		self.fTempPerm95 = 0.0;
		self.iTempPerm96 = 0;
		for l27 in 0..2 {
			self.iRec28[l27 as usize] = 0;
		}
		for l28 in 0..2 {
			self.iRec34[l28 as usize] = 0;
		}
		self.iTempPerm97 = 0;
		self.iTempPerm98 = 0;
		self.iTempPerm99 = 0;
		self.iTempPerm100 = 0;
		self.iTempPerm101 = 0;
		self.iTempPerm102 = 0;
		self.iTempPerm103 = 0;
		self.iTempPerm104 = 0;
		self.iTempPerm105 = 0;
		self.iTempPerm106 = 0;
		self.iTempPerm107 = 0;
		self.iTempPerm108 = 0;
		self.iTempPerm109 = 0;
		self.iTempPerm110 = 0;
		self.fTempPerm111 = 0.0;
		self.fTempPerm112 = 0.0;
		self.iTempPerm113 = 0;
		self.fTempPerm114 = 0.0;
		self.iTempPerm115 = 0;
		self.iTempPerm116 = 0;
		self.fTempPerm117 = 0.0;
		self.iTempPerm118 = 0;
		self.iTempPerm119 = 0;
		self.fTempPerm120 = 0.0;
		self.fTempPerm121 = 0.0;
		self.fTempPerm122 = 0.0;
		self.fTempPerm123 = 0.0;
		self.iTempPerm124 = 0;
		self.iTempPerm125 = 0;
		self.fTempPerm126 = 0.0;
		self.fTempPerm127 = 0.0;
		self.iTempPerm128 = 0;
		self.fTempPerm129 = 0.0;
		self.iTempPerm130 = 0;
		for l29 in 0..2 {
			self.iRec29[l29 as usize] = 0;
		}
		for l30 in 0..2 {
			self.iRec30[l30 as usize] = 0;
		}
		for l31 in 0..2 {
			self.iRec31[l31 as usize] = 0;
		}
		for l32 in 0..2 {
			self.iRec32[l32 as usize] = 0;
		}
		self.iTempPerm131 = 0;
		self.iTempPerm132 = 0;
		self.iTempPerm133 = 0;
		for l33 in 0..2 {
			self.fRec27[l33 as usize] = 0.0;
		}
		self.fTempPerm134 = 0.0;
		self.fTempPerm135 = 0.0;
		self.fTempPerm136 = 0.0;
		self.fTempPerm137 = 0.0;
		self.fTempPerm138 = 0.0;
		self.iTempPerm139 = 0;
		self.iTempPerm140 = 0;
		self.fTempPerm141 = 0.0;
		self.fTempPerm142 = 0.0;
		self.iTempPerm143 = 0;
		self.fTempPerm144 = 0.0;
		self.iTempPerm145 = 0;
		self.iTempPerm146 = 0;
		self.iTempPerm147 = 0;
		self.iTempPerm148 = 0;
		self.iTempPerm149 = 0;
		for l34 in 0..2 {
			self.fRec36[l34 as usize] = 0.0;
		}
		self.fTempPerm150 = 0.0;
		self.iTempPerm151 = 0;
		self.iTempPerm152 = 0;
		self.iTempPerm153 = 0;
		self.iTempPerm154 = 0;
		self.fTempPerm155 = 0.0;
		self.fTempPerm156 = 0.0;
		self.iTempPerm157 = 0;
		self.fTempPerm158 = 0.0;
		self.fTempPerm159 = 0.0;
		self.iTempPerm160 = 0;
		self.iTempPerm161 = 0;
		self.fTempPerm162 = 0.0;
		self.fTempPerm163 = 0.0;
		self.iTempPerm164 = 0;
		self.fTempPerm165 = 0.0;
		self.fTempPerm166 = 0.0;
		self.fTempPerm167 = 0.0;
		self.fTempPerm168 = 0.0;
		self.iTempPerm169 = 0;
		self.fTempPerm170 = 0.0;
		self.fTempPerm171 = 0.0;
		for l35 in 0..2 {
			self.fRec37[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec38[l36 as usize] = 0.0;
		}
		self.fTempPerm172 = 0.0;
		self.fTempPerm173 = 0.0;
		self.iTempPerm174 = 0;
		self.fTempPerm175 = 0.0;
		self.fTempPerm176 = 0.0;
		for l37 in 0..2 {
			self.fRec35[l37 as usize] = 0.0;
		}
		self.iTempPerm177 = 0;
		self.iTempPerm178 = 0;
		for l38 in 0..2 {
			self.fRec39[l38 as usize] = 0.0;
		}
		self.fTempPerm179 = 0.0;
		self.fTempPerm180 = 0.0;
		self.iTempPerm181 = 0;
		for l39 in 0..2 {
			self.iRec41[l39 as usize] = 0;
		}
		for l40 in 0..2 {
			self.iRec47[l40 as usize] = 0;
		}
		self.iTempPerm182 = 0;
		self.iTempPerm183 = 0;
		self.iTempPerm184 = 0;
		self.iTempPerm185 = 0;
		self.iTempPerm186 = 0;
		self.iTempPerm187 = 0;
		self.iTempPerm188 = 0;
		self.iTempPerm189 = 0;
		self.iTempPerm190 = 0;
		self.iTempPerm191 = 0;
		self.iTempPerm192 = 0;
		self.iTempPerm193 = 0;
		self.iTempPerm194 = 0;
		self.iTempPerm195 = 0;
		self.fTempPerm196 = 0.0;
		self.fTempPerm197 = 0.0;
		self.iTempPerm198 = 0;
		self.fTempPerm199 = 0.0;
		self.iTempPerm200 = 0;
		self.iTempPerm201 = 0;
		self.fTempPerm202 = 0.0;
		self.iTempPerm203 = 0;
		self.iTempPerm204 = 0;
		self.fTempPerm205 = 0.0;
		self.fTempPerm206 = 0.0;
		self.fTempPerm207 = 0.0;
		self.fTempPerm208 = 0.0;
		self.iTempPerm209 = 0;
		self.iTempPerm210 = 0;
		self.fTempPerm211 = 0.0;
		self.fTempPerm212 = 0.0;
		self.iTempPerm213 = 0;
		self.fTempPerm214 = 0.0;
		self.iTempPerm215 = 0;
		for l41 in 0..2 {
			self.iRec42[l41 as usize] = 0;
		}
		for l42 in 0..2 {
			self.iRec43[l42 as usize] = 0;
		}
		for l43 in 0..2 {
			self.iRec44[l43 as usize] = 0;
		}
		for l44 in 0..2 {
			self.iRec45[l44 as usize] = 0;
		}
		self.iTempPerm216 = 0;
		self.iTempPerm217 = 0;
		self.iTempPerm218 = 0;
		for l45 in 0..2 {
			self.fRec40[l45 as usize] = 0.0;
		}
		self.fTempPerm219 = 0.0;
		self.fTempPerm220 = 0.0;
		self.fTempPerm221 = 0.0;
		self.fTempPerm222 = 0.0;
		self.fTempPerm223 = 0.0;
		self.iTempPerm224 = 0;
		self.iTempPerm225 = 0;
		self.fTempPerm226 = 0.0;
		self.fTempPerm227 = 0.0;
		self.iTempPerm228 = 0;
		self.fTempPerm229 = 0.0;
		self.iTempPerm230 = 0;
		self.iTempPerm231 = 0;
		self.iTempPerm232 = 0;
		self.iTempPerm233 = 0;
		self.iTempPerm234 = 0;
		for l46 in 0..2 {
			self.fRec49[l46 as usize] = 0.0;
		}
		self.fTempPerm235 = 0.0;
		self.iTempPerm236 = 0;
		self.iTempPerm237 = 0;
		self.iTempPerm238 = 0;
		self.iTempPerm239 = 0;
		self.fTempPerm240 = 0.0;
		self.fTempPerm241 = 0.0;
		self.iTempPerm242 = 0;
		self.fTempPerm243 = 0.0;
		self.fTempPerm244 = 0.0;
		self.iTempPerm245 = 0;
		self.iTempPerm246 = 0;
		self.fTempPerm247 = 0.0;
		self.fTempPerm248 = 0.0;
		self.iTempPerm249 = 0;
		self.fTempPerm250 = 0.0;
		self.fTempPerm251 = 0.0;
		self.fTempPerm252 = 0.0;
		self.fTempPerm253 = 0.0;
		self.iTempPerm254 = 0;
		self.fTempPerm255 = 0.0;
		self.fTempPerm256 = 0.0;
		for l47 in 0..2 {
			self.fRec50[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec51[l48 as usize] = 0.0;
		}
		self.fTempPerm257 = 0.0;
		self.fTempPerm258 = 0.0;
		self.iTempPerm259 = 0;
		self.fTempPerm260 = 0.0;
		self.fTempPerm261 = 0.0;
		for l49 in 0..2 {
			self.fRec48[l49 as usize] = 0.0;
		}
		self.iTempPerm262 = 0;
		self.iTempPerm263 = 0;
		for l50 in 0..2 {
			self.fRec52[l50 as usize] = 0.0;
		}
		self.fTempPerm264 = 0.0;
		self.fTempPerm265 = 0.0;
		self.iTempPerm266 = 0;
		for l51 in 0..2 {
			self.iRec54[l51 as usize] = 0;
		}
		for l52 in 0..2 {
			self.iRec60[l52 as usize] = 0;
		}
		self.iTempPerm267 = 0;
		self.iTempPerm268 = 0;
		self.iTempPerm269 = 0;
		self.iTempPerm270 = 0;
		self.iTempPerm271 = 0;
		self.iTempPerm272 = 0;
		self.iTempPerm273 = 0;
		self.iTempPerm274 = 0;
		self.iTempPerm275 = 0;
		self.iTempPerm276 = 0;
		self.iTempPerm277 = 0;
		self.iTempPerm278 = 0;
		self.iTempPerm279 = 0;
		self.iTempPerm280 = 0;
		self.fTempPerm281 = 0.0;
		self.fTempPerm282 = 0.0;
		self.iTempPerm283 = 0;
		self.fTempPerm284 = 0.0;
		self.iTempPerm285 = 0;
		self.iTempPerm286 = 0;
		self.fTempPerm287 = 0.0;
		self.iTempPerm288 = 0;
		self.iTempPerm289 = 0;
		self.fTempPerm290 = 0.0;
		self.fTempPerm291 = 0.0;
		self.fTempPerm292 = 0.0;
		self.fTempPerm293 = 0.0;
		self.iTempPerm294 = 0;
		self.iTempPerm295 = 0;
		self.fTempPerm296 = 0.0;
		self.fTempPerm297 = 0.0;
		self.iTempPerm298 = 0;
		self.fTempPerm299 = 0.0;
		self.iTempPerm300 = 0;
		for l53 in 0..2 {
			self.iRec55[l53 as usize] = 0;
		}
		for l54 in 0..2 {
			self.iRec56[l54 as usize] = 0;
		}
		for l55 in 0..2 {
			self.iRec57[l55 as usize] = 0;
		}
		for l56 in 0..2 {
			self.iRec58[l56 as usize] = 0;
		}
		self.iTempPerm301 = 0;
		self.iTempPerm302 = 0;
		self.iTempPerm303 = 0;
		for l57 in 0..2 {
			self.fRec53[l57 as usize] = 0.0;
		}
		self.fTempPerm304 = 0.0;
		self.fTempPerm305 = 0.0;
		self.fTempPerm306 = 0.0;
		self.fTempPerm307 = 0.0;
		self.fTempPerm308 = 0.0;
		self.iTempPerm309 = 0;
		self.iTempPerm310 = 0;
		self.fTempPerm311 = 0.0;
		self.fTempPerm312 = 0.0;
		self.iTempPerm313 = 0;
		self.fTempPerm314 = 0.0;
		self.iTempPerm315 = 0;
		self.iTempPerm316 = 0;
		self.iTempPerm317 = 0;
		self.iTempPerm318 = 0;
		self.iTempPerm319 = 0;
		for l58 in 0..2 {
			self.fRec62[l58 as usize] = 0.0;
		}
		self.fTempPerm320 = 0.0;
		self.iTempPerm321 = 0;
		self.iTempPerm322 = 0;
		self.iTempPerm323 = 0;
		self.iTempPerm324 = 0;
		self.fTempPerm325 = 0.0;
		self.fTempPerm326 = 0.0;
		self.iTempPerm327 = 0;
		self.fTempPerm328 = 0.0;
		self.fTempPerm329 = 0.0;
		self.iTempPerm330 = 0;
		self.iTempPerm331 = 0;
		self.fTempPerm332 = 0.0;
		self.fTempPerm333 = 0.0;
		self.iTempPerm334 = 0;
		self.fTempPerm335 = 0.0;
		self.fTempPerm336 = 0.0;
		self.fTempPerm337 = 0.0;
		self.fTempPerm338 = 0.0;
		self.iTempPerm339 = 0;
		self.fTempPerm340 = 0.0;
		self.fTempPerm341 = 0.0;
		for l59 in 0..2 {
			self.fRec63[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec64[l60 as usize] = 0.0;
		}
		self.fTempPerm342 = 0.0;
		self.fTempPerm343 = 0.0;
		self.iTempPerm344 = 0;
		self.fTempPerm345 = 0.0;
		self.fTempPerm346 = 0.0;
		for l61 in 0..2 {
			self.fRec61[l61 as usize] = 0.0;
		}
		self.iTempPerm347 = 0;
		self.iTempPerm348 = 0;
		for l62 in 0..2 {
			self.fRec65[l62 as usize] = 0.0;
		}
		self.fTempPerm349 = 0.0;
		self.fTempPerm350 = 0.0;
		self.iTempPerm351 = 0;
		for l63 in 0..2 {
			self.iRec67[l63 as usize] = 0;
		}
		for l64 in 0..2 {
			self.iRec73[l64 as usize] = 0;
		}
		self.iTempPerm352 = 0;
		self.iTempPerm353 = 0;
		self.iTempPerm354 = 0;
		self.iTempPerm355 = 0;
		self.iTempPerm356 = 0;
		self.iTempPerm357 = 0;
		self.iTempPerm358 = 0;
		self.iTempPerm359 = 0;
		self.iTempPerm360 = 0;
		self.iTempPerm361 = 0;
		self.iTempPerm362 = 0;
		self.iTempPerm363 = 0;
		self.iTempPerm364 = 0;
		self.iTempPerm365 = 0;
		self.fTempPerm366 = 0.0;
		self.fTempPerm367 = 0.0;
		self.iTempPerm368 = 0;
		self.fTempPerm369 = 0.0;
		self.iTempPerm370 = 0;
		self.iTempPerm371 = 0;
		self.fTempPerm372 = 0.0;
		self.iTempPerm373 = 0;
		self.iTempPerm374 = 0;
		self.fTempPerm375 = 0.0;
		self.fTempPerm376 = 0.0;
		self.fTempPerm377 = 0.0;
		self.fTempPerm378 = 0.0;
		self.iTempPerm379 = 0;
		self.iTempPerm380 = 0;
		self.fTempPerm381 = 0.0;
		self.fTempPerm382 = 0.0;
		self.iTempPerm383 = 0;
		self.fTempPerm384 = 0.0;
		self.iTempPerm385 = 0;
		for l65 in 0..2 {
			self.iRec68[l65 as usize] = 0;
		}
		for l66 in 0..2 {
			self.iRec69[l66 as usize] = 0;
		}
		for l67 in 0..2 {
			self.iRec70[l67 as usize] = 0;
		}
		for l68 in 0..2 {
			self.iRec71[l68 as usize] = 0;
		}
		self.iTempPerm386 = 0;
		self.iTempPerm387 = 0;
		self.iTempPerm388 = 0;
		for l69 in 0..2 {
			self.fRec66[l69 as usize] = 0.0;
		}
		self.fTempPerm389 = 0.0;
		self.fTempPerm390 = 0.0;
		self.fTempPerm391 = 0.0;
		self.fTempPerm392 = 0.0;
		self.fTempPerm393 = 0.0;
		self.iTempPerm394 = 0;
		self.iTempPerm395 = 0;
		self.fTempPerm396 = 0.0;
		self.fTempPerm397 = 0.0;
		self.iTempPerm398 = 0;
		self.fTempPerm399 = 0.0;
		self.iTempPerm400 = 0;
		self.iTempPerm401 = 0;
		self.iTempPerm402 = 0;
		self.iTempPerm403 = 0;
		self.iTempPerm404 = 0;
		for l70 in 0..2 {
			self.fRec75[l70 as usize] = 0.0;
		}
		self.fTempPerm405 = 0.0;
		self.iTempPerm406 = 0;
		self.iTempPerm407 = 0;
		self.iTempPerm408 = 0;
		self.iTempPerm409 = 0;
		self.fTempPerm410 = 0.0;
		self.fTempPerm411 = 0.0;
		self.iTempPerm412 = 0;
		self.fTempPerm413 = 0.0;
		self.fTempPerm414 = 0.0;
		self.iTempPerm415 = 0;
		self.iTempPerm416 = 0;
		self.fTempPerm417 = 0.0;
		self.fTempPerm418 = 0.0;
		self.iTempPerm419 = 0;
		self.fTempPerm420 = 0.0;
		self.fTempPerm421 = 0.0;
		self.fTempPerm422 = 0.0;
		self.fTempPerm423 = 0.0;
		self.iTempPerm424 = 0;
		self.fTempPerm425 = 0.0;
		self.fTempPerm426 = 0.0;
		for l71 in 0..2 {
			self.fRec76[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec77[l72 as usize] = 0.0;
		}
		self.fTempPerm427 = 0.0;
		self.fTempPerm428 = 0.0;
		self.iTempPerm429 = 0;
		self.fTempPerm430 = 0.0;
		self.fTempPerm431 = 0.0;
		for l73 in 0..2 {
			self.fRec74[l73 as usize] = 0.0;
		}
		self.iTempPerm432 = 0;
		self.iTempPerm433 = 0;
		for l74 in 0..2 {
			self.fRec78[l74 as usize] = 0.0;
		}
		self.fTempPerm434 = 0.0;
		self.fTempPerm435 = 0.0;
		self.iTempPerm436 = 0;
		for l75 in 0..2 {
			self.iRec80[l75 as usize] = 0;
		}
		for l76 in 0..2 {
			self.iRec86[l76 as usize] = 0;
		}
		self.iTempPerm437 = 0;
		self.iTempPerm438 = 0;
		self.iTempPerm439 = 0;
		self.iTempPerm440 = 0;
		self.iTempPerm441 = 0;
		self.iTempPerm442 = 0;
		self.iTempPerm443 = 0;
		self.iTempPerm444 = 0;
		self.iTempPerm445 = 0;
		self.iTempPerm446 = 0;
		self.iTempPerm447 = 0;
		self.iTempPerm448 = 0;
		self.iTempPerm449 = 0;
		self.iTempPerm450 = 0;
		self.fTempPerm451 = 0.0;
		self.fTempPerm452 = 0.0;
		self.iTempPerm453 = 0;
		self.fTempPerm454 = 0.0;
		self.iTempPerm455 = 0;
		self.iTempPerm456 = 0;
		self.fTempPerm457 = 0.0;
		self.iTempPerm458 = 0;
		self.iTempPerm459 = 0;
		self.fTempPerm460 = 0.0;
		self.fTempPerm461 = 0.0;
		self.fTempPerm462 = 0.0;
		self.fTempPerm463 = 0.0;
		self.iTempPerm464 = 0;
		self.iTempPerm465 = 0;
		self.fTempPerm466 = 0.0;
		self.fTempPerm467 = 0.0;
		self.iTempPerm468 = 0;
		self.fTempPerm469 = 0.0;
		self.iTempPerm470 = 0;
		for l77 in 0..2 {
			self.iRec81[l77 as usize] = 0;
		}
		for l78 in 0..2 {
			self.iRec82[l78 as usize] = 0;
		}
		for l79 in 0..2 {
			self.iRec83[l79 as usize] = 0;
		}
		for l80 in 0..2 {
			self.iRec84[l80 as usize] = 0;
		}
		self.iTempPerm471 = 0;
		self.iTempPerm472 = 0;
		self.iTempPerm473 = 0;
		for l81 in 0..2 {
			self.fRec79[l81 as usize] = 0.0;
		}
		self.fTempPerm474 = 0.0;
		self.fTempPerm475 = 0.0;
		self.fTempPerm476 = 0.0;
		self.fTempPerm477 = 0.0;
		self.fTempPerm478 = 0.0;
		self.iTempPerm479 = 0;
		self.iTempPerm480 = 0;
		self.fTempPerm481 = 0.0;
		self.fTempPerm482 = 0.0;
		self.iTempPerm483 = 0;
		self.fTempPerm484 = 0.0;
		self.iTempPerm485 = 0;
		self.iTempPerm486 = 0;
		self.iTempPerm487 = 0;
		self.iTempPerm488 = 0;
		self.iTempPerm489 = 0;
		for l82 in 0..2 {
			self.fRec88[l82 as usize] = 0.0;
		}
		self.fTempPerm490 = 0.0;
		self.iTempPerm491 = 0;
		self.iTempPerm492 = 0;
		self.iTempPerm493 = 0;
		self.iTempPerm494 = 0;
		self.fTempPerm495 = 0.0;
		self.fTempPerm496 = 0.0;
		self.iTempPerm497 = 0;
		self.fTempPerm498 = 0.0;
		self.fTempPerm499 = 0.0;
		self.iTempPerm500 = 0;
		self.iTempPerm501 = 0;
		self.fTempPerm502 = 0.0;
		self.fTempPerm503 = 0.0;
		self.iTempPerm504 = 0;
		self.fTempPerm505 = 0.0;
		self.fTempPerm506 = 0.0;
		self.fTempPerm507 = 0.0;
		self.fTempPerm508 = 0.0;
		self.iTempPerm509 = 0;
		self.fTempPerm510 = 0.0;
		self.fTempPerm511 = 0.0;
		for l83 in 0..2 {
			self.fRec89[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec90[l84 as usize] = 0.0;
		}
		self.fTempPerm512 = 0.0;
		self.fTempPerm513 = 0.0;
		self.iTempPerm514 = 0;
		self.fTempPerm515 = 0.0;
		self.fTempPerm516 = 0.0;
		for l85 in 0..2 {
			self.fRec87[l85 as usize] = 0.0;
		}
		self.iTempPerm517 = 0;
		self.iTempPerm518 = 0;
		for l86 in 0..2 {
			self.fRec91[l86 as usize] = 0.0;
		}
		self.fTempPerm519 = 0.0;
		self.fTempPerm520 = 0.0;
		self.iTempPerm521 = 0;
		for l87 in 0..2 {
			self.iRec93[l87 as usize] = 0;
		}
		for l88 in 0..2 {
			self.iRec99[l88 as usize] = 0;
		}
		self.iTempPerm522 = 0;
		self.iTempPerm523 = 0;
		self.iTempPerm524 = 0;
		self.iTempPerm525 = 0;
		self.iTempPerm526 = 0;
		self.iTempPerm527 = 0;
		self.iTempPerm528 = 0;
		self.iTempPerm529 = 0;
		self.iTempPerm530 = 0;
		self.iTempPerm531 = 0;
		self.iTempPerm532 = 0;
		self.iTempPerm533 = 0;
		self.iTempPerm534 = 0;
		self.iTempPerm535 = 0;
		self.fTempPerm536 = 0.0;
		self.fTempPerm537 = 0.0;
		self.iTempPerm538 = 0;
		self.fTempPerm539 = 0.0;
		self.iTempPerm540 = 0;
		self.iTempPerm541 = 0;
		self.fTempPerm542 = 0.0;
		self.iTempPerm543 = 0;
		self.iTempPerm544 = 0;
		self.fTempPerm545 = 0.0;
		self.fTempPerm546 = 0.0;
		self.fTempPerm547 = 0.0;
		self.fTempPerm548 = 0.0;
		self.iTempPerm549 = 0;
		self.iTempPerm550 = 0;
		self.fTempPerm551 = 0.0;
		self.fTempPerm552 = 0.0;
		self.iTempPerm553 = 0;
		self.fTempPerm554 = 0.0;
		self.iTempPerm555 = 0;
		for l89 in 0..2 {
			self.iRec94[l89 as usize] = 0;
		}
		for l90 in 0..2 {
			self.iRec95[l90 as usize] = 0;
		}
		for l91 in 0..2 {
			self.iRec96[l91 as usize] = 0;
		}
		for l92 in 0..2 {
			self.iRec97[l92 as usize] = 0;
		}
		self.iTempPerm556 = 0;
		self.iTempPerm557 = 0;
		self.iTempPerm558 = 0;
		for l93 in 0..2 {
			self.fRec92[l93 as usize] = 0.0;
		}
		self.fTempPerm559 = 0.0;
		self.fTempPerm560 = 0.0;
		self.fTempPerm561 = 0.0;
		self.fTempPerm562 = 0.0;
		self.fTempPerm563 = 0.0;
		self.iTempPerm564 = 0;
		self.iTempPerm565 = 0;
		self.fTempPerm566 = 0.0;
		self.fTempPerm567 = 0.0;
		self.iTempPerm568 = 0;
		self.fTempPerm569 = 0.0;
		self.iTempPerm570 = 0;
		self.iTempPerm571 = 0;
		self.iTempPerm572 = 0;
		self.iTempPerm573 = 0;
		self.iTempPerm574 = 0;
		for l94 in 0..2 {
			self.fRec101[l94 as usize] = 0.0;
		}
		self.fTempPerm575 = 0.0;
		self.iTempPerm576 = 0;
		self.iTempPerm577 = 0;
		self.iTempPerm578 = 0;
		self.iTempPerm579 = 0;
		self.fTempPerm580 = 0.0;
		self.fTempPerm581 = 0.0;
		self.iTempPerm582 = 0;
		self.fTempPerm583 = 0.0;
		self.fTempPerm584 = 0.0;
		self.iTempPerm585 = 0;
		self.iTempPerm586 = 0;
		self.fTempPerm587 = 0.0;
		self.fTempPerm588 = 0.0;
		self.iTempPerm589 = 0;
		self.fTempPerm590 = 0.0;
		self.fTempPerm591 = 0.0;
		self.fTempPerm592 = 0.0;
		self.fTempPerm593 = 0.0;
		self.iTempPerm594 = 0;
		self.fTempPerm595 = 0.0;
		self.fTempPerm596 = 0.0;
		for l95 in 0..2 {
			self.fRec102[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec103[l96 as usize] = 0.0;
		}
		self.fTempPerm597 = 0.0;
		self.fTempPerm598 = 0.0;
		self.iTempPerm599 = 0;
		self.fTempPerm600 = 0.0;
		self.fTempPerm601 = 0.0;
		for l97 in 0..2 {
			self.fRec100[l97 as usize] = 0.0;
		}
		self.iTempPerm602 = 0;
		self.iTempPerm603 = 0;
		for l98 in 0..2 {
			self.fRec104[l98 as usize] = 0.0;
		}
		self.fTempPerm604 = 0.0;
		self.fTempPerm605 = 0.0;
		self.iTempPerm606 = 0;
		for l99 in 0..2 {
			self.iRec106[l99 as usize] = 0;
		}
		for l100 in 0..2 {
			self.iRec112[l100 as usize] = 0;
		}
		self.iTempPerm607 = 0;
		self.iTempPerm608 = 0;
		self.iTempPerm609 = 0;
		self.iTempPerm610 = 0;
		self.iTempPerm611 = 0;
		self.iTempPerm612 = 0;
		self.iTempPerm613 = 0;
		self.iTempPerm614 = 0;
		self.iTempPerm615 = 0;
		self.iTempPerm616 = 0;
		self.iTempPerm617 = 0;
		self.iTempPerm618 = 0;
		self.iTempPerm619 = 0;
		self.iTempPerm620 = 0;
		self.fTempPerm621 = 0.0;
		self.fTempPerm622 = 0.0;
		self.iTempPerm623 = 0;
		self.fTempPerm624 = 0.0;
		self.iTempPerm625 = 0;
		self.iTempPerm626 = 0;
		self.fTempPerm627 = 0.0;
		self.iTempPerm628 = 0;
		self.iTempPerm629 = 0;
		self.fTempPerm630 = 0.0;
		self.fTempPerm631 = 0.0;
		self.fTempPerm632 = 0.0;
		self.fTempPerm633 = 0.0;
		self.iTempPerm634 = 0;
		self.iTempPerm635 = 0;
		self.fTempPerm636 = 0.0;
		self.fTempPerm637 = 0.0;
		self.iTempPerm638 = 0;
		self.fTempPerm639 = 0.0;
		self.iTempPerm640 = 0;
		for l101 in 0..2 {
			self.iRec107[l101 as usize] = 0;
		}
		for l102 in 0..2 {
			self.iRec108[l102 as usize] = 0;
		}
		for l103 in 0..2 {
			self.iRec109[l103 as usize] = 0;
		}
		for l104 in 0..2 {
			self.iRec110[l104 as usize] = 0;
		}
		self.iTempPerm641 = 0;
		self.iTempPerm642 = 0;
		self.iTempPerm643 = 0;
		for l105 in 0..2 {
			self.fRec105[l105 as usize] = 0.0;
		}
		self.fTempPerm644 = 0.0;
		self.fTempPerm645 = 0.0;
		self.fTempPerm646 = 0.0;
		self.fTempPerm647 = 0.0;
		self.fTempPerm648 = 0.0;
		self.iTempPerm649 = 0;
		self.iTempPerm650 = 0;
		self.fTempPerm651 = 0.0;
		self.fTempPerm652 = 0.0;
		self.iTempPerm653 = 0;
		self.fTempPerm654 = 0.0;
		self.iTempPerm655 = 0;
		self.iTempPerm656 = 0;
		self.iTempPerm657 = 0;
		self.iTempPerm658 = 0;
		self.iTempPerm659 = 0;
		for l106 in 0..2 {
			self.fRec114[l106 as usize] = 0.0;
		}
		self.fTempPerm660 = 0.0;
		self.iTempPerm661 = 0;
		self.iTempPerm662 = 0;
		self.iTempPerm663 = 0;
		self.iTempPerm664 = 0;
		self.fTempPerm665 = 0.0;
		self.fTempPerm666 = 0.0;
		self.iTempPerm667 = 0;
		self.fTempPerm668 = 0.0;
		self.fTempPerm669 = 0.0;
		self.iTempPerm670 = 0;
		self.iTempPerm671 = 0;
		self.fTempPerm672 = 0.0;
		self.fTempPerm673 = 0.0;
		self.iTempPerm674 = 0;
		self.fTempPerm675 = 0.0;
		self.fTempPerm676 = 0.0;
		self.fTempPerm677 = 0.0;
		self.fTempPerm678 = 0.0;
		self.iTempPerm679 = 0;
		self.fTempPerm680 = 0.0;
		self.fTempPerm681 = 0.0;
		for l107 in 0..2 {
			self.fRec115[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec116[l108 as usize] = 0.0;
		}
		self.fTempPerm682 = 0.0;
		self.fTempPerm683 = 0.0;
		self.iTempPerm684 = 0;
		self.fTempPerm685 = 0.0;
		self.fTempPerm686 = 0.0;
		for l109 in 0..2 {
			self.fRec113[l109 as usize] = 0.0;
		}
		self.iTempPerm687 = 0;
		self.iTempPerm688 = 0;
		for l110 in 0..2 {
			self.fRec117[l110 as usize] = 0.0;
		}
		self.fTempPerm689 = 0.0;
		self.fTempPerm690 = 0.0;
		self.fTempPerm691 = 0.0;
		self.fTempPerm692 = 0.0;
		self.fTempPerm693 = 0.0;
		self.fTempPerm694 = 0.0;
		self.fTempPerm695 = 0.0;
		self.fTempPerm696 = 0.0;
		self.fTempPerm697 = 0.0;
		self.fTempPerm698 = 0.0;
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::exp(-(4.0 / self.fConst0));
		self.fConst2 = F32::exp(-(2e+02 / self.fConst0));
		self.fConst3 = F32::exp(-(5.0 / self.fConst0));
		self.fConst4 = F32::exp(-(1e+02 / self.fConst0));
		self.fConst5 = 0.05 * self.fConst0;
		self.fConst6 = 0.45 * self.fConst0;
		self.fConst7 = 1.0 / self.fConst0;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		CallsDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("plugin-calls");
		ui_interface.add_horizontal_slider("activity", ParamIndex(0), 0.45, 0.0, 1.0, 0.01);
		ui_interface.add_horizontal_slider("follow", ParamIndex(1), 1.0, 0.0, 1.0, 0.01);
		ui_interface.add_horizontal_slider("gate0", ParamIndex(2), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("gate1", ParamIndex(3), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("gate2", ParamIndex(4), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("gate3", ParamIndex(5), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("gate4", ParamIndex(6), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("gate5", ParamIndex(7), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("gate6", ParamIndex(8), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("gate7", ParamIndex(9), 0.0, 0.0, 1.0, 1.0);
		ui_interface.add_horizontal_slider("level", ParamIndex(10), -24.0, -6e+01, 0.0, 0.1);
		ui_interface.add_horizontal_slider("note0", ParamIndex(11), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note1", ParamIndex(12), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note2", ParamIndex(13), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note3", ParamIndex(14), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note4", ParamIndex(15), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note5", ParamIndex(16), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note6", ParamIndex(17), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("note7", ParamIndex(18), 130.81, 2e+01, 8e+03, 0.01);
		ui_interface.add_horizontal_slider("rate", ParamIndex(19), 1.0, 0.25, 3.0, 0.01);
		ui_interface.add_horizontal_slider("seed", ParamIndex(20), 2e+03, 0.0, 1e+06, 1.0);
		ui_interface.add_horizontal_slider("width", ParamIndex(21), 1.0, 0.0, 1.0, 0.01);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			1 => Some(self.fHslider0),
			10 => Some(self.fHslider1),
			20 => Some(self.fHslider10),
			19 => Some(self.fHslider11),
			18 => Some(self.fHslider12),
			0 => Some(self.fHslider13),
			21 => Some(self.fHslider14),
			17 => Some(self.fHslider15),
			16 => Some(self.fHslider16),
			15 => Some(self.fHslider17),
			14 => Some(self.fHslider18),
			13 => Some(self.fHslider19),
			9 => Some(self.fHslider2),
			12 => Some(self.fHslider20),
			11 => Some(self.fHslider21),
			2 => Some(self.fHslider3),
			8 => Some(self.fHslider4),
			7 => Some(self.fHslider5),
			6 => Some(self.fHslider6),
			5 => Some(self.fHslider7),
			4 => Some(self.fHslider8),
			3 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			1 => { self.fHslider0 = value }
			10 => { self.fHslider1 = value }
			20 => { self.fHslider10 = value }
			19 => { self.fHslider11 = value }
			18 => { self.fHslider12 = value }
			0 => { self.fHslider13 = value }
			21 => { self.fHslider14 = value }
			17 => { self.fHslider15 = value }
			16 => { self.fHslider16 = value }
			15 => { self.fHslider17 = value }
			14 => { self.fHslider18 = value }
			13 => { self.fHslider19 = value }
			9 => { self.fHslider2 = value }
			12 => { self.fHslider20 = value }
			11 => { self.fHslider21 = value }
			2 => { self.fHslider3 = value }
			8 => { self.fHslider4 = value }
			7 => { self.fHslider5 = value }
			6 => { self.fHslider6 = value }
			5 => { self.fHslider7 = value }
			4 => { self.fHslider8 = value }
			3 => { self.fHslider9 = value }
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
		let mut fSlow1: F32 = F32::powf(1e+01, 0.05 * self.fHslider1);
		let mut fSlow2: F32 = self.fHslider2;
		let mut fSlow3: F32 = F32::abs(fSlow2);
		let mut iSlow4: i32 = (fSlow2 > 0.0) as i32;
		let mut iTemp10: i32 = self.iTempPerm0;
		let mut fSlow5: F32 = self.fHslider3;
		let mut fSlow6: F32 = F32::abs(fSlow5);
		let mut iSlow7: i32 = (fSlow5 > 0.0) as i32;
		let mut fSlow8: F32 = self.fHslider4;
		let mut fSlow9: F32 = F32::abs(fSlow8);
		let mut iSlow10: i32 = (fSlow8 > 0.0) as i32;
		let mut fSlow11: F32 = self.fHslider5;
		let mut fSlow12: F32 = F32::abs(fSlow11);
		let mut iSlow13: i32 = (fSlow11 > 0.0) as i32;
		let mut fSlow14: F32 = self.fHslider6;
		let mut fSlow15: F32 = F32::abs(fSlow14);
		let mut iSlow16: i32 = (fSlow14 > 0.0) as i32;
		let mut fSlow17: F32 = self.fHslider7;
		let mut fSlow18: F32 = F32::abs(fSlow17);
		let mut iSlow19: i32 = (fSlow17 > 0.0) as i32;
		let mut fSlow20: F32 = self.fHslider8;
		let mut fSlow21: F32 = F32::abs(fSlow20);
		let mut iSlow22: i32 = (fSlow20 > 0.0) as i32;
		let mut fSlow23: F32 = self.fHslider9;
		let mut fSlow24: F32 = F32::abs(fSlow23);
		let mut iSlow25: i32 = (fSlow23 > 0.0) as i32;
		let mut iTemp11: i32 = self.iTempPerm1;
		let mut iTemp40: i32 = self.iTempPerm2;
		let mut fSlow26: F32 = self.fHslider10;
		let mut iSlow27: i32 = (fSlow26 + 5677.0) as i32;
		let mut iTemp41: i32 = self.iTempPerm3;
		let mut iTemp42: i32 = self.iTempPerm4;
		let mut iTemp43: i32 = self.iTempPerm5;
		let mut iTemp44: i32 = self.iTempPerm6;
		let mut iTemp45: i32 = self.iTempPerm7;
		let mut iTemp46: i32 = self.iTempPerm8;
		let mut iTemp47: i32 = self.iTempPerm9;
		let mut iTemp48: i32 = self.iTempPerm10;
		let mut iTemp49: i32 = self.iTempPerm11;
		let mut iTemp50: i32 = self.iTempPerm12;
		let mut iTemp51: i32 = self.iTempPerm13;
		let mut iTemp52: i32 = self.iTempPerm14;
		let mut iTemp53: i32 = self.iTempPerm15;
		let mut iTemp54: i32 = self.iTempPerm16;
		let mut fTemp55: F32 = self.fTempPerm17;
		let mut fTemp56: F32 = self.fTempPerm18;
		let mut iTemp57: i32 = self.iTempPerm19;
		let mut fSlow28: F32 = self.fHslider11;
		let mut fTemp58: F32 = self.fTempPerm20;
		let mut fTemp59: F32 = self.fTempPerm21;
		let mut iTemp60: i32 = self.iTempPerm22;
		let mut iTemp61: i32 = self.iTempPerm23;
		let mut fTemp62: F32 = self.fTempPerm24;
		let mut fSlow29: F32 = self.fHslider12;
		let mut fSlow30: F32 = 0.7213475 * F32::log(0.0038221916 * fSlow29, std::f32::consts::E);
		let mut fSlow31: F32 = F32::floor(fSlow30);
		let mut fSlow32: F32 = fSlow30 - fSlow31;
		let mut iSlow33: i32 = (fSlow31) as i32;
		let mut fSlow34: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iSlow33))) as F32 / ((1 << std::cmp::min(30, std::cmp::max(0, iSlow33))) as F32 * (fSlow32 * (fSlow32 * (fSlow32 * (fSlow32 * (0.0013333 * fSlow32 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0));
		let mut fSlow35: F32 = 0.02 * fSlow34;
		let mut iTemp63: i32 = self.iTempPerm25;
		let mut iTemp64: i32 = self.iTempPerm26;
		let mut fTemp65: F32 = self.fTempPerm27;
		let mut fTemp66: F32 = self.fTempPerm28;
		let mut fTemp67: F32 = self.fTempPerm29;
		let mut fTemp68: F32 = self.fTempPerm30;
		let mut fSlow36: F32 = self.fHslider13;
		let mut fTemp69: F32 = self.fTempPerm31;
		let mut fTemp70: F32 = self.fTempPerm32;
		let mut iTemp71: i32 = self.iTempPerm33;
		let mut fTemp72: F32 = self.fTempPerm34;
		let mut fTemp73: F32 = self.fTempPerm35;
		let mut fTemp74: F32 = self.fTempPerm36;
		let mut iTemp75: i32 = self.iTempPerm37;
		let mut iTemp76: i32 = self.iTempPerm38;
		let mut fTemp77: F32 = self.fTempPerm39;
		let mut fTemp78: F32 = self.fTempPerm40;
		let mut iTemp79: i32 = self.iTempPerm41;
		let mut fTemp80: F32 = self.fTempPerm42;
		let mut fSlow37: F32 = self.fConst5 * fSlow34;
		let mut iTemp81: i32 = self.iTempPerm43;
		let mut iTemp82: i32 = self.iTempPerm44;
		let mut iTemp83: i32 = self.iTempPerm45;
		let mut iTemp84: i32 = self.iTempPerm46;
		let mut iTemp85: i32 = self.iTempPerm47;
		let mut fSlow38: F32 = self.fHslider14;
		let mut fTemp86: F32 = self.fTempPerm48;
		let mut fTemp87: F32 = self.fTempPerm49;
		let mut fTemp88: F32 = self.fTempPerm50;
		let mut fTemp89: F32 = self.fTempPerm51;
		let mut fTemp90: F32 = self.fTempPerm52;
		let mut fTemp91: F32 = self.fTempPerm53;
		let mut iTemp92: i32 = self.iTempPerm54;
		let mut iTemp93: i32 = self.iTempPerm55;
		let mut fTemp94: F32 = self.fTempPerm56;
		let mut fTemp95: F32 = self.fTempPerm57;
		let mut iTemp96: i32 = self.iTempPerm58;
		let mut fTemp97: F32 = self.fTempPerm59;
		let mut iTemp98: i32 = self.iTempPerm60;
		let mut iTemp99: i32 = self.iTempPerm61;
		let mut iTemp100: i32 = self.iTempPerm62;
		let mut iTemp101: i32 = self.iTempPerm63;
		let mut iTemp102: i32 = self.iTempPerm64;
		let mut fTemp103: F32 = self.fTempPerm65;
		let mut iTemp104: i32 = self.iTempPerm66;
		let mut iTemp105: i32 = self.iTempPerm67;
		let mut iTemp106: i32 = self.iTempPerm68;
		let mut iTemp107: i32 = self.iTempPerm69;
		let mut fTemp108: F32 = self.fTempPerm70;
		let mut fTemp109: F32 = self.fTempPerm71;
		let mut iTemp110: i32 = self.iTempPerm72;
		let mut fTemp111: F32 = self.fTempPerm73;
		let mut fSlow39: F32 = 5e+02 / fSlow29;
		let mut fTemp112: F32 = self.fTempPerm74;
		let mut iTemp113: i32 = self.iTempPerm75;
		let mut iTemp114: i32 = self.iTempPerm76;
		let mut fTemp115: F32 = self.fTempPerm77;
		let mut fTemp116: F32 = self.fTempPerm78;
		let mut iTemp117: i32 = self.iTempPerm79;
		let mut fTemp118: F32 = self.fTempPerm80;
		let mut fTemp119: F32 = self.fTempPerm81;
		let mut fTemp120: F32 = self.fTempPerm82;
		let mut fTemp121: F32 = self.fTempPerm83;
		let mut iTemp122: i32 = self.iTempPerm84;
		let mut fTemp123: F32 = self.fTempPerm85;
		let mut fTemp124: F32 = self.fTempPerm86;
		let mut fTemp125: F32 = self.fTempPerm87;
		let mut fTemp126: F32 = self.fTempPerm88;
		let mut iTemp127: i32 = self.iTempPerm89;
		let mut fTemp128: F32 = self.fTempPerm90;
		let mut fTemp129: F32 = self.fTempPerm91;
		let mut iTemp130: i32 = self.iTempPerm92;
		let mut iTemp131: i32 = self.iTempPerm93;
		let mut fTemp132: F32 = self.fTempPerm94;
		let mut fTemp133: F32 = self.fTempPerm95;
		let mut iTemp135: i32 = self.iTempPerm96;
		let mut iSlow40: i32 = (fSlow26 + 4866.0) as i32;
		let mut iTemp136: i32 = self.iTempPerm97;
		let mut iTemp137: i32 = self.iTempPerm98;
		let mut iTemp138: i32 = self.iTempPerm99;
		let mut iTemp139: i32 = self.iTempPerm100;
		let mut iTemp140: i32 = self.iTempPerm101;
		let mut iTemp141: i32 = self.iTempPerm102;
		let mut iTemp142: i32 = self.iTempPerm103;
		let mut iTemp143: i32 = self.iTempPerm104;
		let mut iTemp144: i32 = self.iTempPerm105;
		let mut iTemp145: i32 = self.iTempPerm106;
		let mut iTemp146: i32 = self.iTempPerm107;
		let mut iTemp147: i32 = self.iTempPerm108;
		let mut iTemp148: i32 = self.iTempPerm109;
		let mut iTemp149: i32 = self.iTempPerm110;
		let mut fTemp150: F32 = self.fTempPerm111;
		let mut fTemp151: F32 = self.fTempPerm112;
		let mut iTemp152: i32 = self.iTempPerm113;
		let mut fTemp153: F32 = self.fTempPerm114;
		let mut iTemp154: i32 = self.iTempPerm115;
		let mut iTemp155: i32 = self.iTempPerm116;
		let mut fTemp156: F32 = self.fTempPerm117;
		let mut fSlow41: F32 = self.fHslider15;
		let mut fSlow42: F32 = 0.7213475 * F32::log(0.0038221916 * fSlow41, std::f32::consts::E);
		let mut fSlow43: F32 = F32::floor(fSlow42);
		let mut fSlow44: F32 = fSlow42 - fSlow43;
		let mut iSlow45: i32 = (fSlow43) as i32;
		let mut fSlow46: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iSlow45))) as F32 / ((1 << std::cmp::min(30, std::cmp::max(0, iSlow45))) as F32 * (fSlow44 * (fSlow44 * (fSlow44 * (fSlow44 * (0.0013333 * fSlow44 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0));
		let mut fSlow47: F32 = 0.02 * fSlow46;
		let mut iTemp157: i32 = self.iTempPerm118;
		let mut iTemp158: i32 = self.iTempPerm119;
		let mut fTemp159: F32 = self.fTempPerm120;
		let mut fTemp160: F32 = self.fTempPerm121;
		let mut fTemp161: F32 = self.fTempPerm122;
		let mut fTemp162: F32 = self.fTempPerm123;
		let mut iTemp163: i32 = self.iTempPerm124;
		let mut iTemp164: i32 = self.iTempPerm125;
		let mut fTemp165: F32 = self.fTempPerm126;
		let mut fTemp166: F32 = self.fTempPerm127;
		let mut iTemp167: i32 = self.iTempPerm128;
		let mut fTemp168: F32 = self.fTempPerm129;
		let mut fSlow48: F32 = self.fConst5 * fSlow46;
		let mut iTemp169: i32 = self.iTempPerm130;
		let mut iTemp170: i32 = self.iTempPerm131;
		let mut iTemp171: i32 = self.iTempPerm132;
		let mut iTemp172: i32 = self.iTempPerm133;
		let mut fTemp173: F32 = self.fTempPerm134;
		let mut fTemp174: F32 = self.fTempPerm135;
		let mut fTemp175: F32 = self.fTempPerm136;
		let mut fTemp176: F32 = self.fTempPerm137;
		let mut fTemp177: F32 = self.fTempPerm138;
		let mut iTemp178: i32 = self.iTempPerm139;
		let mut iTemp179: i32 = self.iTempPerm140;
		let mut fTemp180: F32 = self.fTempPerm141;
		let mut fTemp181: F32 = self.fTempPerm142;
		let mut iTemp182: i32 = self.iTempPerm143;
		let mut fTemp183: F32 = self.fTempPerm144;
		let mut iTemp184: i32 = self.iTempPerm145;
		let mut iTemp185: i32 = self.iTempPerm146;
		let mut iTemp186: i32 = self.iTempPerm147;
		let mut iTemp187: i32 = self.iTempPerm148;
		let mut iTemp188: i32 = self.iTempPerm149;
		let mut fTemp189: F32 = self.fTempPerm150;
		let mut iTemp190: i32 = self.iTempPerm151;
		let mut iTemp191: i32 = self.iTempPerm152;
		let mut iTemp192: i32 = self.iTempPerm153;
		let mut iTemp193: i32 = self.iTempPerm154;
		let mut fTemp194: F32 = self.fTempPerm155;
		let mut fTemp195: F32 = self.fTempPerm156;
		let mut iTemp196: i32 = self.iTempPerm157;
		let mut fTemp197: F32 = self.fTempPerm158;
		let mut fSlow49: F32 = 5e+02 / fSlow41;
		let mut fTemp198: F32 = self.fTempPerm159;
		let mut iTemp199: i32 = self.iTempPerm160;
		let mut iTemp200: i32 = self.iTempPerm161;
		let mut fTemp201: F32 = self.fTempPerm162;
		let mut fTemp202: F32 = self.fTempPerm163;
		let mut iTemp203: i32 = self.iTempPerm164;
		let mut fTemp204: F32 = self.fTempPerm165;
		let mut fTemp205: F32 = self.fTempPerm166;
		let mut fTemp206: F32 = self.fTempPerm167;
		let mut fTemp207: F32 = self.fTempPerm168;
		let mut iTemp208: i32 = self.iTempPerm169;
		let mut fTemp209: F32 = self.fTempPerm170;
		let mut fTemp210: F32 = self.fTempPerm171;
		let mut fTemp211: F32 = self.fTempPerm172;
		let mut fTemp212: F32 = self.fTempPerm173;
		let mut iTemp213: i32 = self.iTempPerm174;
		let mut fTemp214: F32 = self.fTempPerm175;
		let mut fTemp215: F32 = self.fTempPerm176;
		let mut iTemp216: i32 = self.iTempPerm177;
		let mut iTemp217: i32 = self.iTempPerm178;
		let mut fTemp218: F32 = self.fTempPerm179;
		let mut fTemp219: F32 = self.fTempPerm180;
		let mut iTemp221: i32 = self.iTempPerm181;
		let mut iSlow50: i32 = (fSlow26 + 4055.0) as i32;
		let mut iTemp222: i32 = self.iTempPerm182;
		let mut iTemp223: i32 = self.iTempPerm183;
		let mut iTemp224: i32 = self.iTempPerm184;
		let mut iTemp225: i32 = self.iTempPerm185;
		let mut iTemp226: i32 = self.iTempPerm186;
		let mut iTemp227: i32 = self.iTempPerm187;
		let mut iTemp228: i32 = self.iTempPerm188;
		let mut iTemp229: i32 = self.iTempPerm189;
		let mut iTemp230: i32 = self.iTempPerm190;
		let mut iTemp231: i32 = self.iTempPerm191;
		let mut iTemp232: i32 = self.iTempPerm192;
		let mut iTemp233: i32 = self.iTempPerm193;
		let mut iTemp234: i32 = self.iTempPerm194;
		let mut iTemp235: i32 = self.iTempPerm195;
		let mut fTemp236: F32 = self.fTempPerm196;
		let mut fTemp237: F32 = self.fTempPerm197;
		let mut iTemp238: i32 = self.iTempPerm198;
		let mut fTemp239: F32 = self.fTempPerm199;
		let mut iTemp240: i32 = self.iTempPerm200;
		let mut iTemp241: i32 = self.iTempPerm201;
		let mut fTemp242: F32 = self.fTempPerm202;
		let mut fSlow51: F32 = self.fHslider16;
		let mut fSlow52: F32 = 0.7213475 * F32::log(0.0038221916 * fSlow51, std::f32::consts::E);
		let mut fSlow53: F32 = F32::floor(fSlow52);
		let mut fSlow54: F32 = fSlow52 - fSlow53;
		let mut iSlow55: i32 = (fSlow53) as i32;
		let mut fSlow56: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iSlow55))) as F32 / ((1 << std::cmp::min(30, std::cmp::max(0, iSlow55))) as F32 * (fSlow54 * (fSlow54 * (fSlow54 * (fSlow54 * (0.0013333 * fSlow54 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0));
		let mut fSlow57: F32 = 0.02 * fSlow56;
		let mut iTemp243: i32 = self.iTempPerm203;
		let mut iTemp244: i32 = self.iTempPerm204;
		let mut fTemp245: F32 = self.fTempPerm205;
		let mut fTemp246: F32 = self.fTempPerm206;
		let mut fTemp247: F32 = self.fTempPerm207;
		let mut fTemp248: F32 = self.fTempPerm208;
		let mut iTemp249: i32 = self.iTempPerm209;
		let mut iTemp250: i32 = self.iTempPerm210;
		let mut fTemp251: F32 = self.fTempPerm211;
		let mut fTemp252: F32 = self.fTempPerm212;
		let mut iTemp253: i32 = self.iTempPerm213;
		let mut fTemp254: F32 = self.fTempPerm214;
		let mut fSlow58: F32 = self.fConst5 * fSlow56;
		let mut iTemp255: i32 = self.iTempPerm215;
		let mut iTemp256: i32 = self.iTempPerm216;
		let mut iTemp257: i32 = self.iTempPerm217;
		let mut iTemp258: i32 = self.iTempPerm218;
		let mut fTemp259: F32 = self.fTempPerm219;
		let mut fTemp260: F32 = self.fTempPerm220;
		let mut fTemp261: F32 = self.fTempPerm221;
		let mut fTemp262: F32 = self.fTempPerm222;
		let mut fTemp263: F32 = self.fTempPerm223;
		let mut iTemp264: i32 = self.iTempPerm224;
		let mut iTemp265: i32 = self.iTempPerm225;
		let mut fTemp266: F32 = self.fTempPerm226;
		let mut fTemp267: F32 = self.fTempPerm227;
		let mut iTemp268: i32 = self.iTempPerm228;
		let mut fTemp269: F32 = self.fTempPerm229;
		let mut iTemp270: i32 = self.iTempPerm230;
		let mut iTemp271: i32 = self.iTempPerm231;
		let mut iTemp272: i32 = self.iTempPerm232;
		let mut iTemp273: i32 = self.iTempPerm233;
		let mut iTemp274: i32 = self.iTempPerm234;
		let mut fTemp275: F32 = self.fTempPerm235;
		let mut iTemp276: i32 = self.iTempPerm236;
		let mut iTemp277: i32 = self.iTempPerm237;
		let mut iTemp278: i32 = self.iTempPerm238;
		let mut iTemp279: i32 = self.iTempPerm239;
		let mut fTemp280: F32 = self.fTempPerm240;
		let mut fTemp281: F32 = self.fTempPerm241;
		let mut iTemp282: i32 = self.iTempPerm242;
		let mut fTemp283: F32 = self.fTempPerm243;
		let mut fSlow59: F32 = 5e+02 / fSlow51;
		let mut fTemp284: F32 = self.fTempPerm244;
		let mut iTemp285: i32 = self.iTempPerm245;
		let mut iTemp286: i32 = self.iTempPerm246;
		let mut fTemp287: F32 = self.fTempPerm247;
		let mut fTemp288: F32 = self.fTempPerm248;
		let mut iTemp289: i32 = self.iTempPerm249;
		let mut fTemp290: F32 = self.fTempPerm250;
		let mut fTemp291: F32 = self.fTempPerm251;
		let mut fTemp292: F32 = self.fTempPerm252;
		let mut fTemp293: F32 = self.fTempPerm253;
		let mut iTemp294: i32 = self.iTempPerm254;
		let mut fTemp295: F32 = self.fTempPerm255;
		let mut fTemp296: F32 = self.fTempPerm256;
		let mut fTemp297: F32 = self.fTempPerm257;
		let mut fTemp298: F32 = self.fTempPerm258;
		let mut iTemp299: i32 = self.iTempPerm259;
		let mut fTemp300: F32 = self.fTempPerm260;
		let mut fTemp301: F32 = self.fTempPerm261;
		let mut iTemp302: i32 = self.iTempPerm262;
		let mut iTemp303: i32 = self.iTempPerm263;
		let mut fTemp304: F32 = self.fTempPerm264;
		let mut fTemp305: F32 = self.fTempPerm265;
		let mut iTemp307: i32 = self.iTempPerm266;
		let mut iSlow60: i32 = (fSlow26 + 3244.0) as i32;
		let mut iTemp308: i32 = self.iTempPerm267;
		let mut iTemp309: i32 = self.iTempPerm268;
		let mut iTemp310: i32 = self.iTempPerm269;
		let mut iTemp311: i32 = self.iTempPerm270;
		let mut iTemp312: i32 = self.iTempPerm271;
		let mut iTemp313: i32 = self.iTempPerm272;
		let mut iTemp314: i32 = self.iTempPerm273;
		let mut iTemp315: i32 = self.iTempPerm274;
		let mut iTemp316: i32 = self.iTempPerm275;
		let mut iTemp317: i32 = self.iTempPerm276;
		let mut iTemp318: i32 = self.iTempPerm277;
		let mut iTemp319: i32 = self.iTempPerm278;
		let mut iTemp320: i32 = self.iTempPerm279;
		let mut iTemp321: i32 = self.iTempPerm280;
		let mut fTemp322: F32 = self.fTempPerm281;
		let mut fTemp323: F32 = self.fTempPerm282;
		let mut iTemp324: i32 = self.iTempPerm283;
		let mut fTemp325: F32 = self.fTempPerm284;
		let mut iTemp326: i32 = self.iTempPerm285;
		let mut iTemp327: i32 = self.iTempPerm286;
		let mut fTemp328: F32 = self.fTempPerm287;
		let mut fSlow61: F32 = self.fHslider17;
		let mut fSlow62: F32 = 0.7213475 * F32::log(0.0038221916 * fSlow61, std::f32::consts::E);
		let mut fSlow63: F32 = F32::floor(fSlow62);
		let mut fSlow64: F32 = fSlow62 - fSlow63;
		let mut iSlow65: i32 = (fSlow63) as i32;
		let mut fSlow66: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iSlow65))) as F32 / ((1 << std::cmp::min(30, std::cmp::max(0, iSlow65))) as F32 * (fSlow64 * (fSlow64 * (fSlow64 * (fSlow64 * (0.0013333 * fSlow64 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0));
		let mut fSlow67: F32 = 0.02 * fSlow66;
		let mut iTemp329: i32 = self.iTempPerm288;
		let mut iTemp330: i32 = self.iTempPerm289;
		let mut fTemp331: F32 = self.fTempPerm290;
		let mut fTemp332: F32 = self.fTempPerm291;
		let mut fTemp333: F32 = self.fTempPerm292;
		let mut fTemp334: F32 = self.fTempPerm293;
		let mut iTemp335: i32 = self.iTempPerm294;
		let mut iTemp336: i32 = self.iTempPerm295;
		let mut fTemp337: F32 = self.fTempPerm296;
		let mut fTemp338: F32 = self.fTempPerm297;
		let mut iTemp339: i32 = self.iTempPerm298;
		let mut fTemp340: F32 = self.fTempPerm299;
		let mut fSlow68: F32 = self.fConst5 * fSlow66;
		let mut iTemp341: i32 = self.iTempPerm300;
		let mut iTemp342: i32 = self.iTempPerm301;
		let mut iTemp343: i32 = self.iTempPerm302;
		let mut iTemp344: i32 = self.iTempPerm303;
		let mut fTemp345: F32 = self.fTempPerm304;
		let mut fTemp346: F32 = self.fTempPerm305;
		let mut fTemp347: F32 = self.fTempPerm306;
		let mut fTemp348: F32 = self.fTempPerm307;
		let mut fTemp349: F32 = self.fTempPerm308;
		let mut iTemp350: i32 = self.iTempPerm309;
		let mut iTemp351: i32 = self.iTempPerm310;
		let mut fTemp352: F32 = self.fTempPerm311;
		let mut fTemp353: F32 = self.fTempPerm312;
		let mut iTemp354: i32 = self.iTempPerm313;
		let mut fTemp355: F32 = self.fTempPerm314;
		let mut iTemp356: i32 = self.iTempPerm315;
		let mut iTemp357: i32 = self.iTempPerm316;
		let mut iTemp358: i32 = self.iTempPerm317;
		let mut iTemp359: i32 = self.iTempPerm318;
		let mut iTemp360: i32 = self.iTempPerm319;
		let mut fTemp361: F32 = self.fTempPerm320;
		let mut iTemp362: i32 = self.iTempPerm321;
		let mut iTemp363: i32 = self.iTempPerm322;
		let mut iTemp364: i32 = self.iTempPerm323;
		let mut iTemp365: i32 = self.iTempPerm324;
		let mut fTemp366: F32 = self.fTempPerm325;
		let mut fTemp367: F32 = self.fTempPerm326;
		let mut iTemp368: i32 = self.iTempPerm327;
		let mut fTemp369: F32 = self.fTempPerm328;
		let mut fSlow69: F32 = 5e+02 / fSlow61;
		let mut fTemp370: F32 = self.fTempPerm329;
		let mut iTemp371: i32 = self.iTempPerm330;
		let mut iTemp372: i32 = self.iTempPerm331;
		let mut fTemp373: F32 = self.fTempPerm332;
		let mut fTemp374: F32 = self.fTempPerm333;
		let mut iTemp375: i32 = self.iTempPerm334;
		let mut fTemp376: F32 = self.fTempPerm335;
		let mut fTemp377: F32 = self.fTempPerm336;
		let mut fTemp378: F32 = self.fTempPerm337;
		let mut fTemp379: F32 = self.fTempPerm338;
		let mut iTemp380: i32 = self.iTempPerm339;
		let mut fTemp381: F32 = self.fTempPerm340;
		let mut fTemp382: F32 = self.fTempPerm341;
		let mut fTemp383: F32 = self.fTempPerm342;
		let mut fTemp384: F32 = self.fTempPerm343;
		let mut iTemp385: i32 = self.iTempPerm344;
		let mut fTemp386: F32 = self.fTempPerm345;
		let mut fTemp387: F32 = self.fTempPerm346;
		let mut iTemp388: i32 = self.iTempPerm347;
		let mut iTemp389: i32 = self.iTempPerm348;
		let mut fTemp390: F32 = self.fTempPerm349;
		let mut fTemp391: F32 = self.fTempPerm350;
		let mut iTemp393: i32 = self.iTempPerm351;
		let mut iSlow70: i32 = (fSlow26 + 2433.0) as i32;
		let mut iTemp394: i32 = self.iTempPerm352;
		let mut iTemp395: i32 = self.iTempPerm353;
		let mut iTemp396: i32 = self.iTempPerm354;
		let mut iTemp397: i32 = self.iTempPerm355;
		let mut iTemp398: i32 = self.iTempPerm356;
		let mut iTemp399: i32 = self.iTempPerm357;
		let mut iTemp400: i32 = self.iTempPerm358;
		let mut iTemp401: i32 = self.iTempPerm359;
		let mut iTemp402: i32 = self.iTempPerm360;
		let mut iTemp403: i32 = self.iTempPerm361;
		let mut iTemp404: i32 = self.iTempPerm362;
		let mut iTemp405: i32 = self.iTempPerm363;
		let mut iTemp406: i32 = self.iTempPerm364;
		let mut iTemp407: i32 = self.iTempPerm365;
		let mut fTemp408: F32 = self.fTempPerm366;
		let mut fTemp409: F32 = self.fTempPerm367;
		let mut iTemp410: i32 = self.iTempPerm368;
		let mut fTemp411: F32 = self.fTempPerm369;
		let mut iTemp412: i32 = self.iTempPerm370;
		let mut iTemp413: i32 = self.iTempPerm371;
		let mut fTemp414: F32 = self.fTempPerm372;
		let mut fSlow71: F32 = self.fHslider18;
		let mut fSlow72: F32 = 0.7213475 * F32::log(0.0038221916 * fSlow71, std::f32::consts::E);
		let mut fSlow73: F32 = F32::floor(fSlow72);
		let mut fSlow74: F32 = fSlow72 - fSlow73;
		let mut iSlow75: i32 = (fSlow73) as i32;
		let mut fSlow76: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iSlow75))) as F32 / ((1 << std::cmp::min(30, std::cmp::max(0, iSlow75))) as F32 * (fSlow74 * (fSlow74 * (fSlow74 * (fSlow74 * (0.0013333 * fSlow74 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0));
		let mut fSlow77: F32 = 0.02 * fSlow76;
		let mut iTemp415: i32 = self.iTempPerm373;
		let mut iTemp416: i32 = self.iTempPerm374;
		let mut fTemp417: F32 = self.fTempPerm375;
		let mut fTemp418: F32 = self.fTempPerm376;
		let mut fTemp419: F32 = self.fTempPerm377;
		let mut fTemp420: F32 = self.fTempPerm378;
		let mut iTemp421: i32 = self.iTempPerm379;
		let mut iTemp422: i32 = self.iTempPerm380;
		let mut fTemp423: F32 = self.fTempPerm381;
		let mut fTemp424: F32 = self.fTempPerm382;
		let mut iTemp425: i32 = self.iTempPerm383;
		let mut fTemp426: F32 = self.fTempPerm384;
		let mut fSlow78: F32 = self.fConst5 * fSlow76;
		let mut iTemp427: i32 = self.iTempPerm385;
		let mut iTemp428: i32 = self.iTempPerm386;
		let mut iTemp429: i32 = self.iTempPerm387;
		let mut iTemp430: i32 = self.iTempPerm388;
		let mut fTemp431: F32 = self.fTempPerm389;
		let mut fTemp432: F32 = self.fTempPerm390;
		let mut fTemp433: F32 = self.fTempPerm391;
		let mut fTemp434: F32 = self.fTempPerm392;
		let mut fTemp435: F32 = self.fTempPerm393;
		let mut iTemp436: i32 = self.iTempPerm394;
		let mut iTemp437: i32 = self.iTempPerm395;
		let mut fTemp438: F32 = self.fTempPerm396;
		let mut fTemp439: F32 = self.fTempPerm397;
		let mut iTemp440: i32 = self.iTempPerm398;
		let mut fTemp441: F32 = self.fTempPerm399;
		let mut iTemp442: i32 = self.iTempPerm400;
		let mut iTemp443: i32 = self.iTempPerm401;
		let mut iTemp444: i32 = self.iTempPerm402;
		let mut iTemp445: i32 = self.iTempPerm403;
		let mut iTemp446: i32 = self.iTempPerm404;
		let mut fTemp447: F32 = self.fTempPerm405;
		let mut iTemp448: i32 = self.iTempPerm406;
		let mut iTemp449: i32 = self.iTempPerm407;
		let mut iTemp450: i32 = self.iTempPerm408;
		let mut iTemp451: i32 = self.iTempPerm409;
		let mut fTemp452: F32 = self.fTempPerm410;
		let mut fTemp453: F32 = self.fTempPerm411;
		let mut iTemp454: i32 = self.iTempPerm412;
		let mut fTemp455: F32 = self.fTempPerm413;
		let mut fSlow79: F32 = 5e+02 / fSlow71;
		let mut fTemp456: F32 = self.fTempPerm414;
		let mut iTemp457: i32 = self.iTempPerm415;
		let mut iTemp458: i32 = self.iTempPerm416;
		let mut fTemp459: F32 = self.fTempPerm417;
		let mut fTemp460: F32 = self.fTempPerm418;
		let mut iTemp461: i32 = self.iTempPerm419;
		let mut fTemp462: F32 = self.fTempPerm420;
		let mut fTemp463: F32 = self.fTempPerm421;
		let mut fTemp464: F32 = self.fTempPerm422;
		let mut fTemp465: F32 = self.fTempPerm423;
		let mut iTemp466: i32 = self.iTempPerm424;
		let mut fTemp467: F32 = self.fTempPerm425;
		let mut fTemp468: F32 = self.fTempPerm426;
		let mut fTemp469: F32 = self.fTempPerm427;
		let mut fTemp470: F32 = self.fTempPerm428;
		let mut iTemp471: i32 = self.iTempPerm429;
		let mut fTemp472: F32 = self.fTempPerm430;
		let mut fTemp473: F32 = self.fTempPerm431;
		let mut iTemp474: i32 = self.iTempPerm432;
		let mut iTemp475: i32 = self.iTempPerm433;
		let mut fTemp476: F32 = self.fTempPerm434;
		let mut fTemp477: F32 = self.fTempPerm435;
		let mut iTemp479: i32 = self.iTempPerm436;
		let mut iSlow80: i32 = (fSlow26 + 1622.0) as i32;
		let mut iTemp480: i32 = self.iTempPerm437;
		let mut iTemp481: i32 = self.iTempPerm438;
		let mut iTemp482: i32 = self.iTempPerm439;
		let mut iTemp483: i32 = self.iTempPerm440;
		let mut iTemp484: i32 = self.iTempPerm441;
		let mut iTemp485: i32 = self.iTempPerm442;
		let mut iTemp486: i32 = self.iTempPerm443;
		let mut iTemp487: i32 = self.iTempPerm444;
		let mut iTemp488: i32 = self.iTempPerm445;
		let mut iTemp489: i32 = self.iTempPerm446;
		let mut iTemp490: i32 = self.iTempPerm447;
		let mut iTemp491: i32 = self.iTempPerm448;
		let mut iTemp492: i32 = self.iTempPerm449;
		let mut iTemp493: i32 = self.iTempPerm450;
		let mut fTemp494: F32 = self.fTempPerm451;
		let mut fTemp495: F32 = self.fTempPerm452;
		let mut iTemp496: i32 = self.iTempPerm453;
		let mut fTemp497: F32 = self.fTempPerm454;
		let mut iTemp498: i32 = self.iTempPerm455;
		let mut iTemp499: i32 = self.iTempPerm456;
		let mut fTemp500: F32 = self.fTempPerm457;
		let mut fSlow81: F32 = self.fHslider19;
		let mut fSlow82: F32 = 0.7213475 * F32::log(0.0038221916 * fSlow81, std::f32::consts::E);
		let mut fSlow83: F32 = F32::floor(fSlow82);
		let mut fSlow84: F32 = fSlow82 - fSlow83;
		let mut iSlow85: i32 = (fSlow83) as i32;
		let mut fSlow86: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iSlow85))) as F32 / ((1 << std::cmp::min(30, std::cmp::max(0, iSlow85))) as F32 * (fSlow84 * (fSlow84 * (fSlow84 * (fSlow84 * (0.0013333 * fSlow84 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0));
		let mut fSlow87: F32 = 0.02 * fSlow86;
		let mut iTemp501: i32 = self.iTempPerm458;
		let mut iTemp502: i32 = self.iTempPerm459;
		let mut fTemp503: F32 = self.fTempPerm460;
		let mut fTemp504: F32 = self.fTempPerm461;
		let mut fTemp505: F32 = self.fTempPerm462;
		let mut fTemp506: F32 = self.fTempPerm463;
		let mut iTemp507: i32 = self.iTempPerm464;
		let mut iTemp508: i32 = self.iTempPerm465;
		let mut fTemp509: F32 = self.fTempPerm466;
		let mut fTemp510: F32 = self.fTempPerm467;
		let mut iTemp511: i32 = self.iTempPerm468;
		let mut fTemp512: F32 = self.fTempPerm469;
		let mut fSlow88: F32 = self.fConst5 * fSlow86;
		let mut iTemp513: i32 = self.iTempPerm470;
		let mut iTemp514: i32 = self.iTempPerm471;
		let mut iTemp515: i32 = self.iTempPerm472;
		let mut iTemp516: i32 = self.iTempPerm473;
		let mut fTemp517: F32 = self.fTempPerm474;
		let mut fTemp518: F32 = self.fTempPerm475;
		let mut fTemp519: F32 = self.fTempPerm476;
		let mut fTemp520: F32 = self.fTempPerm477;
		let mut fTemp521: F32 = self.fTempPerm478;
		let mut iTemp522: i32 = self.iTempPerm479;
		let mut iTemp523: i32 = self.iTempPerm480;
		let mut fTemp524: F32 = self.fTempPerm481;
		let mut fTemp525: F32 = self.fTempPerm482;
		let mut iTemp526: i32 = self.iTempPerm483;
		let mut fTemp527: F32 = self.fTempPerm484;
		let mut iTemp528: i32 = self.iTempPerm485;
		let mut iTemp529: i32 = self.iTempPerm486;
		let mut iTemp530: i32 = self.iTempPerm487;
		let mut iTemp531: i32 = self.iTempPerm488;
		let mut iTemp532: i32 = self.iTempPerm489;
		let mut fTemp533: F32 = self.fTempPerm490;
		let mut iTemp534: i32 = self.iTempPerm491;
		let mut iTemp535: i32 = self.iTempPerm492;
		let mut iTemp536: i32 = self.iTempPerm493;
		let mut iTemp537: i32 = self.iTempPerm494;
		let mut fTemp538: F32 = self.fTempPerm495;
		let mut fTemp539: F32 = self.fTempPerm496;
		let mut iTemp540: i32 = self.iTempPerm497;
		let mut fTemp541: F32 = self.fTempPerm498;
		let mut fSlow89: F32 = 5e+02 / fSlow81;
		let mut fTemp542: F32 = self.fTempPerm499;
		let mut iTemp543: i32 = self.iTempPerm500;
		let mut iTemp544: i32 = self.iTempPerm501;
		let mut fTemp545: F32 = self.fTempPerm502;
		let mut fTemp546: F32 = self.fTempPerm503;
		let mut iTemp547: i32 = self.iTempPerm504;
		let mut fTemp548: F32 = self.fTempPerm505;
		let mut fTemp549: F32 = self.fTempPerm506;
		let mut fTemp550: F32 = self.fTempPerm507;
		let mut fTemp551: F32 = self.fTempPerm508;
		let mut iTemp552: i32 = self.iTempPerm509;
		let mut fTemp553: F32 = self.fTempPerm510;
		let mut fTemp554: F32 = self.fTempPerm511;
		let mut fTemp555: F32 = self.fTempPerm512;
		let mut fTemp556: F32 = self.fTempPerm513;
		let mut iTemp557: i32 = self.iTempPerm514;
		let mut fTemp558: F32 = self.fTempPerm515;
		let mut fTemp559: F32 = self.fTempPerm516;
		let mut iTemp560: i32 = self.iTempPerm517;
		let mut iTemp561: i32 = self.iTempPerm518;
		let mut fTemp562: F32 = self.fTempPerm519;
		let mut fTemp563: F32 = self.fTempPerm520;
		let mut iTemp565: i32 = self.iTempPerm521;
		let mut iSlow90: i32 = (fSlow26 + 811.0) as i32;
		let mut iTemp566: i32 = self.iTempPerm522;
		let mut iTemp567: i32 = self.iTempPerm523;
		let mut iTemp568: i32 = self.iTempPerm524;
		let mut iTemp569: i32 = self.iTempPerm525;
		let mut iTemp570: i32 = self.iTempPerm526;
		let mut iTemp571: i32 = self.iTempPerm527;
		let mut iTemp572: i32 = self.iTempPerm528;
		let mut iTemp573: i32 = self.iTempPerm529;
		let mut iTemp574: i32 = self.iTempPerm530;
		let mut iTemp575: i32 = self.iTempPerm531;
		let mut iTemp576: i32 = self.iTempPerm532;
		let mut iTemp577: i32 = self.iTempPerm533;
		let mut iTemp578: i32 = self.iTempPerm534;
		let mut iTemp579: i32 = self.iTempPerm535;
		let mut fTemp580: F32 = self.fTempPerm536;
		let mut fTemp581: F32 = self.fTempPerm537;
		let mut iTemp582: i32 = self.iTempPerm538;
		let mut fTemp583: F32 = self.fTempPerm539;
		let mut iTemp584: i32 = self.iTempPerm540;
		let mut iTemp585: i32 = self.iTempPerm541;
		let mut fTemp586: F32 = self.fTempPerm542;
		let mut fSlow91: F32 = self.fHslider20;
		let mut fSlow92: F32 = 0.7213475 * F32::log(0.0038221916 * fSlow91, std::f32::consts::E);
		let mut fSlow93: F32 = F32::floor(fSlow92);
		let mut fSlow94: F32 = fSlow92 - fSlow93;
		let mut iSlow95: i32 = (fSlow93) as i32;
		let mut fSlow96: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iSlow95))) as F32 / ((1 << std::cmp::min(30, std::cmp::max(0, iSlow95))) as F32 * (fSlow94 * (fSlow94 * (fSlow94 * (fSlow94 * (0.0013333 * fSlow94 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0));
		let mut fSlow97: F32 = 0.02 * fSlow96;
		let mut iTemp587: i32 = self.iTempPerm543;
		let mut iTemp588: i32 = self.iTempPerm544;
		let mut fTemp589: F32 = self.fTempPerm545;
		let mut fTemp590: F32 = self.fTempPerm546;
		let mut fTemp591: F32 = self.fTempPerm547;
		let mut fTemp592: F32 = self.fTempPerm548;
		let mut iTemp593: i32 = self.iTempPerm549;
		let mut iTemp594: i32 = self.iTempPerm550;
		let mut fTemp595: F32 = self.fTempPerm551;
		let mut fTemp596: F32 = self.fTempPerm552;
		let mut iTemp597: i32 = self.iTempPerm553;
		let mut fTemp598: F32 = self.fTempPerm554;
		let mut fSlow98: F32 = self.fConst5 * fSlow96;
		let mut iTemp599: i32 = self.iTempPerm555;
		let mut iTemp600: i32 = self.iTempPerm556;
		let mut iTemp601: i32 = self.iTempPerm557;
		let mut iTemp602: i32 = self.iTempPerm558;
		let mut fTemp603: F32 = self.fTempPerm559;
		let mut fTemp604: F32 = self.fTempPerm560;
		let mut fTemp605: F32 = self.fTempPerm561;
		let mut fTemp606: F32 = self.fTempPerm562;
		let mut fTemp607: F32 = self.fTempPerm563;
		let mut iTemp608: i32 = self.iTempPerm564;
		let mut iTemp609: i32 = self.iTempPerm565;
		let mut fTemp610: F32 = self.fTempPerm566;
		let mut fTemp611: F32 = self.fTempPerm567;
		let mut iTemp612: i32 = self.iTempPerm568;
		let mut fTemp613: F32 = self.fTempPerm569;
		let mut iTemp614: i32 = self.iTempPerm570;
		let mut iTemp615: i32 = self.iTempPerm571;
		let mut iTemp616: i32 = self.iTempPerm572;
		let mut iTemp617: i32 = self.iTempPerm573;
		let mut iTemp618: i32 = self.iTempPerm574;
		let mut fTemp619: F32 = self.fTempPerm575;
		let mut iTemp620: i32 = self.iTempPerm576;
		let mut iTemp621: i32 = self.iTempPerm577;
		let mut iTemp622: i32 = self.iTempPerm578;
		let mut iTemp623: i32 = self.iTempPerm579;
		let mut fTemp624: F32 = self.fTempPerm580;
		let mut fTemp625: F32 = self.fTempPerm581;
		let mut iTemp626: i32 = self.iTempPerm582;
		let mut fTemp627: F32 = self.fTempPerm583;
		let mut fSlow99: F32 = 5e+02 / fSlow91;
		let mut fTemp628: F32 = self.fTempPerm584;
		let mut iTemp629: i32 = self.iTempPerm585;
		let mut iTemp630: i32 = self.iTempPerm586;
		let mut fTemp631: F32 = self.fTempPerm587;
		let mut fTemp632: F32 = self.fTempPerm588;
		let mut iTemp633: i32 = self.iTempPerm589;
		let mut fTemp634: F32 = self.fTempPerm590;
		let mut fTemp635: F32 = self.fTempPerm591;
		let mut fTemp636: F32 = self.fTempPerm592;
		let mut fTemp637: F32 = self.fTempPerm593;
		let mut iTemp638: i32 = self.iTempPerm594;
		let mut fTemp639: F32 = self.fTempPerm595;
		let mut fTemp640: F32 = self.fTempPerm596;
		let mut fTemp641: F32 = self.fTempPerm597;
		let mut fTemp642: F32 = self.fTempPerm598;
		let mut iTemp643: i32 = self.iTempPerm599;
		let mut fTemp644: F32 = self.fTempPerm600;
		let mut fTemp645: F32 = self.fTempPerm601;
		let mut iTemp646: i32 = self.iTempPerm602;
		let mut iTemp647: i32 = self.iTempPerm603;
		let mut fTemp648: F32 = self.fTempPerm604;
		let mut fTemp649: F32 = self.fTempPerm605;
		let mut iTemp651: i32 = self.iTempPerm606;
		let mut iSlow100: i32 = (fSlow26) as i32;
		let mut iTemp652: i32 = self.iTempPerm607;
		let mut iTemp653: i32 = self.iTempPerm608;
		let mut iTemp654: i32 = self.iTempPerm609;
		let mut iTemp655: i32 = self.iTempPerm610;
		let mut iTemp656: i32 = self.iTempPerm611;
		let mut iTemp657: i32 = self.iTempPerm612;
		let mut iTemp658: i32 = self.iTempPerm613;
		let mut iTemp659: i32 = self.iTempPerm614;
		let mut iTemp660: i32 = self.iTempPerm615;
		let mut iTemp661: i32 = self.iTempPerm616;
		let mut iTemp662: i32 = self.iTempPerm617;
		let mut iTemp663: i32 = self.iTempPerm618;
		let mut iTemp664: i32 = self.iTempPerm619;
		let mut iTemp665: i32 = self.iTempPerm620;
		let mut fTemp666: F32 = self.fTempPerm621;
		let mut fTemp667: F32 = self.fTempPerm622;
		let mut iTemp668: i32 = self.iTempPerm623;
		let mut fTemp669: F32 = self.fTempPerm624;
		let mut iTemp670: i32 = self.iTempPerm625;
		let mut iTemp671: i32 = self.iTempPerm626;
		let mut fTemp672: F32 = self.fTempPerm627;
		let mut fSlow101: F32 = self.fHslider21;
		let mut fSlow102: F32 = 0.7213475 * F32::log(0.0038221916 * fSlow101, std::f32::consts::E);
		let mut fSlow103: F32 = F32::floor(fSlow102);
		let mut fSlow104: F32 = fSlow102 - fSlow103;
		let mut iSlow105: i32 = (fSlow103) as i32;
		let mut fSlow106: F32 = (1 << std::cmp::min(30, std::cmp::max(0, -iSlow105))) as F32 / ((1 << std::cmp::min(30, std::cmp::max(0, iSlow105))) as F32 * (fSlow104 * (fSlow104 * (fSlow104 * (fSlow104 * (0.0013333 * fSlow104 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0));
		let mut fSlow107: F32 = 0.02 * fSlow106;
		let mut iTemp673: i32 = self.iTempPerm628;
		let mut iTemp674: i32 = self.iTempPerm629;
		let mut fTemp675: F32 = self.fTempPerm630;
		let mut fTemp676: F32 = self.fTempPerm631;
		let mut fTemp677: F32 = self.fTempPerm632;
		let mut fTemp678: F32 = self.fTempPerm633;
		let mut iTemp679: i32 = self.iTempPerm634;
		let mut iTemp680: i32 = self.iTempPerm635;
		let mut fTemp681: F32 = self.fTempPerm636;
		let mut fTemp682: F32 = self.fTempPerm637;
		let mut iTemp683: i32 = self.iTempPerm638;
		let mut fTemp684: F32 = self.fTempPerm639;
		let mut fSlow108: F32 = self.fConst5 * fSlow106;
		let mut iTemp685: i32 = self.iTempPerm640;
		let mut iTemp686: i32 = self.iTempPerm641;
		let mut iTemp687: i32 = self.iTempPerm642;
		let mut iTemp688: i32 = self.iTempPerm643;
		let mut fTemp689: F32 = self.fTempPerm644;
		let mut fTemp690: F32 = self.fTempPerm645;
		let mut fTemp691: F32 = self.fTempPerm646;
		let mut fTemp692: F32 = self.fTempPerm647;
		let mut fTemp693: F32 = self.fTempPerm648;
		let mut iTemp694: i32 = self.iTempPerm649;
		let mut iTemp695: i32 = self.iTempPerm650;
		let mut fTemp696: F32 = self.fTempPerm651;
		let mut fTemp697: F32 = self.fTempPerm652;
		let mut iTemp698: i32 = self.iTempPerm653;
		let mut fTemp699: F32 = self.fTempPerm654;
		let mut iTemp700: i32 = self.iTempPerm655;
		let mut iTemp701: i32 = self.iTempPerm656;
		let mut iTemp702: i32 = self.iTempPerm657;
		let mut iTemp703: i32 = self.iTempPerm658;
		let mut iTemp704: i32 = self.iTempPerm659;
		let mut fTemp705: F32 = self.fTempPerm660;
		let mut iTemp706: i32 = self.iTempPerm661;
		let mut iTemp707: i32 = self.iTempPerm662;
		let mut iTemp708: i32 = self.iTempPerm663;
		let mut iTemp709: i32 = self.iTempPerm664;
		let mut fTemp710: F32 = self.fTempPerm665;
		let mut fTemp711: F32 = self.fTempPerm666;
		let mut iTemp712: i32 = self.iTempPerm667;
		let mut fTemp713: F32 = self.fTempPerm668;
		let mut fSlow109: F32 = 5e+02 / fSlow101;
		let mut fTemp714: F32 = self.fTempPerm669;
		let mut iTemp715: i32 = self.iTempPerm670;
		let mut iTemp716: i32 = self.iTempPerm671;
		let mut fTemp717: F32 = self.fTempPerm672;
		let mut fTemp718: F32 = self.fTempPerm673;
		let mut iTemp719: i32 = self.iTempPerm674;
		let mut fTemp720: F32 = self.fTempPerm675;
		let mut fTemp721: F32 = self.fTempPerm676;
		let mut fTemp722: F32 = self.fTempPerm677;
		let mut fTemp723: F32 = self.fTempPerm678;
		let mut iTemp724: i32 = self.iTempPerm679;
		let mut fTemp725: F32 = self.fTempPerm680;
		let mut fTemp726: F32 = self.fTempPerm681;
		let mut fTemp727: F32 = self.fTempPerm682;
		let mut fTemp728: F32 = self.fTempPerm683;
		let mut iTemp729: i32 = self.iTempPerm684;
		let mut fTemp730: F32 = self.fTempPerm685;
		let mut fTemp731: F32 = self.fTempPerm686;
		let mut iTemp732: i32 = self.iTempPerm687;
		let mut iTemp733: i32 = self.iTempPerm688;
		let mut fTemp734: F32 = self.fTempPerm689;
		let mut fTemp735: F32 = self.fTempPerm690;
		let mut fTemp737: F32 = self.fTempPerm691;
		let mut fTemp738: F32 = self.fTempPerm692;
		let mut fTemp739: F32 = self.fTempPerm693;
		let mut fTemp740: F32 = self.fTempPerm694;
		let mut fTemp741: F32 = self.fTempPerm695;
		let mut fTemp742: F32 = self.fTempPerm696;
		let mut fTemp743: F32 = self.fTempPerm697;
		let mut fTemp744: F32 = self.fTempPerm698;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut iRec10: i32 = 0;
			let mut iRec33: i32 = 0;
			let mut iRec46: i32 = 0;
			let mut iRec59: i32 = 0;
			let mut iRec72: i32 = 0;
			let mut iRec85: i32 = 0;
			let mut iRec98: i32 = 0;
			let mut iRec111: i32 = 0;
			self.iVec0[0] = 1;
			let mut fTemp0: F32 = F32::abs(F32::max(F32::abs(*input0), F32::abs(*input1)));
			let mut fTemp1: F32 = if (fTemp0 > self.fRec0[1]) as i32 != 0 {self.fConst2} else {self.fConst1};
			self.fRec0[0] = fTemp0 * (1.0 - fTemp1) + self.fRec0[1] * fTemp1;
			let mut fTemp2: F32 = (self.iVec0[1]) as F32;
			let mut fTemp3: F32 = 1.0 - 0.999 * fTemp2;
			self.fRec1[0] = fSlow0 * fTemp3 + 0.999 * fTemp2 * self.fRec1[1];
			let mut fTemp4: F32 = 1.0 - if (F32::abs(self.fRec1[0] - fSlow0) < 1e-06) as i32 != 0 {fSlow0} else {self.fRec1[0]} * (1.0 - F32::min(1.0, F32::max(0.0, 0.025 * (2e+01 * F32::log10(F32::max(1.1754944e-38, F32::max(self.fRec0[0], 1e-06))) + 6e+01))));
			self.fRec2[0] = fSlow1 * fTemp3 + 0.999 * fTemp2 * self.fRec2[1];
			let mut fTemp5: F32 = if (F32::abs(self.fRec2[0] - fSlow1) < 1e-06) as i32 != 0 {fSlow1} else {self.fRec2[0]};
			let mut fTemp6: F32 = if (fSlow3 > self.fRec3[1]) as i32 != 0 {self.fConst4} else {self.fConst3};
			self.fRec3[0] = fSlow3 * (1.0 - fTemp6) + self.fRec3[1] * fTemp6;
			let mut iTemp7: i32 = iSlow4 | ((self.fRec3[0] > 1e-05) as i32);
			let mut fTemp8: F32 = (iTemp7) as F32;
			let mut iTemp9: i32 = (fTemp8 != 0.0) as i32;
			if iTemp9 != 0 {
				iTemp10 = (self.iRec5[1] >= self.iRec6[1]) as i32;
				self.iRec5[0] = if iTemp10 != 0 {0} else {i32::wrapping_add(self.iRec5[1], 1)};
			}
				let mut fTemp12: F32 = if (fSlow6 > self.fRec11[1]) as i32 != 0 {self.fConst4} else {self.fConst3};
				self.fRec11[0] = fSlow6 * (1.0 - fTemp12) + self.fRec11[1] * fTemp12;
				let mut iTemp13: i32 = iSlow7 | ((self.fRec11[0] > 1e-05) as i32);
				let mut fTemp14: F32 = (iTemp13) as F32;
				let mut iTemp15: i32 = (fTemp14 != 0.0) as i32;
				let mut fTemp16: F32 = if (fSlow9 > self.fRec12[1]) as i32 != 0 {self.fConst4} else {self.fConst3};
				self.fRec12[0] = fSlow9 * (1.0 - fTemp16) + self.fRec12[1] * fTemp16;
				let mut iTemp17: i32 = iSlow10 | ((self.fRec12[0] > 1e-05) as i32);
				let mut fTemp18: F32 = (iTemp17) as F32;
				let mut iTemp19: i32 = (fTemp18 != 0.0) as i32;
				let mut fTemp20: F32 = if (fSlow12 > self.fRec13[1]) as i32 != 0 {self.fConst4} else {self.fConst3};
				self.fRec13[0] = fSlow12 * (1.0 - fTemp20) + self.fRec13[1] * fTemp20;
				let mut iTemp21: i32 = iSlow13 | ((self.fRec13[0] > 1e-05) as i32);
				let mut fTemp22: F32 = (iTemp21) as F32;
				let mut iTemp23: i32 = (fTemp22 != 0.0) as i32;
				let mut fTemp24: F32 = if (fSlow15 > self.fRec14[1]) as i32 != 0 {self.fConst4} else {self.fConst3};
				self.fRec14[0] = fSlow15 * (1.0 - fTemp24) + self.fRec14[1] * fTemp24;
				let mut iTemp25: i32 = iSlow16 | ((self.fRec14[0] > 1e-05) as i32);
				let mut fTemp26: F32 = (iTemp25) as F32;
				let mut iTemp27: i32 = (fTemp26 != 0.0) as i32;
				let mut fTemp28: F32 = if (fSlow18 > self.fRec15[1]) as i32 != 0 {self.fConst4} else {self.fConst3};
				self.fRec15[0] = fSlow18 * (1.0 - fTemp28) + self.fRec15[1] * fTemp28;
				let mut iTemp29: i32 = iSlow19 | ((self.fRec15[0] > 1e-05) as i32);
				let mut fTemp30: F32 = (iTemp29) as F32;
				let mut iTemp31: i32 = (fTemp30 != 0.0) as i32;
				let mut fTemp32: F32 = if (fSlow21 > self.fRec16[1]) as i32 != 0 {self.fConst4} else {self.fConst3};
				self.fRec16[0] = fSlow21 * (1.0 - fTemp32) + self.fRec16[1] * fTemp32;
				let mut iTemp33: i32 = iSlow22 | ((self.fRec16[0] > 1e-05) as i32);
				let mut fTemp34: F32 = (iTemp33) as F32;
				let mut iTemp35: i32 = (fTemp34 != 0.0) as i32;
				let mut fTemp36: F32 = if (fSlow24 > self.fRec17[1]) as i32 != 0 {self.fConst4} else {self.fConst3};
				self.fRec17[0] = fSlow24 * (1.0 - fTemp36) + self.fRec17[1] * fTemp36;
				let mut iTemp37: i32 = iSlow25 | ((self.fRec17[0] > 1e-05) as i32);
				let mut fTemp38: F32 = (iTemp37) as F32;
				let mut iTemp39: i32 = (fTemp38 != 0.0) as i32;
				if iTemp39 | (iTemp35 | (iTemp31 | (iTemp27 | (iTemp23 | (iTemp19 | (iTemp9 | iTemp15)))))) != 0 {
					iTemp11 = i32::wrapping_sub(1, self.iVec0[1]);
					iTemp40 = (iTemp11 > 0) as i32;
				}
					if iTemp9 != 0 {
						self.iRec18[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow27, iTemp11), self.iRec18[1])), 12345);
						iTemp41 = i32::wrapping_add(self.iRec18[0], 104729);
						iTemp42 = i32::wrapping_mul(1664525, iTemp41 ^ (iTemp41 >> 15));
						iTemp43 = i32::wrapping_add(self.iRec9[1], 17);
						iTemp44 = i32::wrapping_mul(1664525, iTemp43 ^ (iTemp43 >> 15));
						iTemp45 = i32::wrapping_add(self.iRec8[1], 1);
						iTemp46 = iTemp10 & ((iTemp45 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp44 ^ (iTemp44 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
						iTemp47 = i32::wrapping_add(self.iRec18[0], 209458);
						iTemp48 = i32::wrapping_mul(1664525, iTemp47 ^ (iTemp47 >> 15));
						iTemp49 = if (iTemp46 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp42 ^ (iTemp42 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp40 != 0 {i32::wrapping_mul(1013904223, iTemp48 ^ (iTemp48 >> 13))} else {self.iRec9[1]};
						iTemp50 = i32::wrapping_add(iTemp49, 17);
						iTemp51 = i32::wrapping_mul(1664525, iTemp50 ^ (iTemp50 >> 15));
						iTemp52 = if iTemp10 != 0 {if iTemp46 != 0 {0} else {iTemp45}} else {self.iRec8[1]};
						iTemp53 = i32::wrapping_add(iTemp49, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp52), 1)));
						iTemp54 = i32::wrapping_mul(1664525, iTemp53 ^ (iTemp53 >> 15));
						fTemp55 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp54 ^ (iTemp54 >> 13))) as F32 + 0.5);
						fTemp56 = F32::floor(fTemp55);
						iTemp57 = (fTemp56) as i32;
					}
						if iTemp39 | (iTemp35 | (iTemp31 | (iTemp27 | (iTemp23 | (iTemp19 | (iTemp9 | iTemp15)))))) != 0 {
							self.fRec19[0] = fSlow28 * fTemp3 + 0.999 * fTemp2 * self.fRec19[1];
							fTemp58 = if (F32::abs(self.fRec19[0] - fSlow28) < 1e-06) as i32 != 0 {fSlow28} else {self.fRec19[0]};
						}
							if iTemp9 != 0 {
								fTemp59 = fTemp55 - fTemp56;
								iTemp60 = i32::wrapping_add(self.iRec18[0], 314187);
								iTemp61 = i32::wrapping_mul(1664525, iTemp60 ^ (iTemp60 >> 15));
								fTemp62 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp61 ^ (iTemp61 >> 13))) as F32 + 1.0;
								iTemp63 = i32::wrapping_add(self.iRec18[0], 418916);
								iTemp64 = i32::wrapping_mul(1664525, iTemp63 ^ (iTemp63 >> 15));
								fTemp65 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp64 ^ (iTemp64 >> 13))) as F32;
								fTemp66 = fTemp65 + 1.5;
								fTemp67 = 0.5 - fTemp65;
								fTemp68 = CallsDsp_faustpower2_f(fTemp67) / CallsDsp_faustpower2_f(fTemp66);
							}
								if iTemp39 | (iTemp35 | (iTemp31 | (iTemp27 | (iTemp23 | (iTemp19 | (iTemp9 | iTemp15)))))) != 0 {
									self.fRec20[0] = fSlow36 * fTemp3 + 0.999 * fTemp2 * self.fRec20[1];
									fTemp69 = 6.9068904 * if (F32::abs(self.fRec20[0] - fSlow36) < 1e-06) as i32 != 0 {fSlow36} else {self.fRec20[0]};
									fTemp70 = F32::floor(-fTemp69);
									iTemp71 = (fTemp70) as i32;
									fTemp72 = (1 << std::cmp::min(30, std::cmp::max(0, -iTemp71))) as F32;
									fTemp73 = fTemp69 + fTemp70;
									fTemp74 = (1.0 - fTemp73 * (0.693147 - fTemp73 * (0.240227 - fTemp73 * (0.0555041 - fTemp73 * (0.0096181 - 0.0013333 * fTemp73))))) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp71))) as F32;
								}
									if iTemp9 != 0 {
										iTemp75 = i32::wrapping_add(iTemp49, i32::wrapping_mul(190681, iTemp52));
										iTemp76 = i32::wrapping_mul(1664525, iTemp75 ^ (iTemp75 >> 15));
										fTemp77 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp76 ^ (iTemp76 >> 13))) as F32 + 0.5);
										fTemp78 = F32::floor(fTemp77);
										iTemp79 = (fTemp78) as i32;
										fTemp80 = fTemp77 - fTemp78;
									}
										if iTemp39 | (iTemp35 | (iTemp31 | (iTemp27 | (iTemp23 | (iTemp19 | (iTemp9 | iTemp15)))))) != 0 {
											iTemp81 = i32::wrapping_sub(1, iTemp40);
										}
											if iTemp9 != 0 {
												iTemp82 = if iTemp10 != 0 {i32::wrapping_mul(iTemp81, (fSlow37 * ((fTemp80 * (fTemp80 * (fTemp80 * (fTemp80 * (0.0013333 * fTemp80 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp79))) as F32 * fTemp62 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp79))) as F32))) as i32)} else {self.iRec7[1]};
												self.iRec6[0] = if iTemp10 != 0 {i32::wrapping_add(iTemp82, (self.fConst0 * if (((i32::wrapping_add(iTemp52, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp51 ^ (iTemp51 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp40 != 0 {1.2e+02 * (fTemp74 * fTemp67 * (15.0 - 4.0 * fTemp68) / (fTemp72 * fTemp66 * (15.0 - 9.0 * fTemp68)))} else {fSlow35 * (fTemp62 * (fTemp59 * (fTemp59 * (fTemp59 * (fTemp59 * (0.0013333 * fTemp59 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp57))) as F32 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp57))) as F32))}) as i32)} else {self.iRec6[1]};
												self.iRec7[0] = iTemp82;
												self.iRec8[0] = iTemp52;
												self.iRec9[0] = iTemp49;
												iRec10 = iTemp10;
												iTemp83 = i32::wrapping_mul(31, self.iRec8[0]);
												iTemp84 = i32::wrapping_add(self.iRec9[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp83, 8)));
												iTemp85 = i32::wrapping_mul(1664525, iTemp84 ^ (iTemp84 >> 15));
												self.fRec4[0] = if iRec10 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp85 ^ (iTemp85 >> 13))) as F32 + 0.5} else {self.fRec4[1]};
											}
												if iTemp39 | (iTemp35 | (iTemp31 | (iTemp27 | (iTemp23 | (iTemp19 | (iTemp9 | iTemp15)))))) != 0 {
													self.fRec21[0] = fSlow38 * fTemp3 + 0.999 * fTemp2 * self.fRec21[1];
													fTemp86 = if (F32::abs(self.fRec21[0] - fSlow38) < 1e-06) as i32 != 0 {fSlow38} else {self.fRec21[0]};
												}
													if iTemp9 != 0 {
														fTemp87 = 1.5707964 * (fTemp86 * (self.fRec4[0] + -0.5) + 0.5);
														fTemp88 = F32::min(1.0, F32::max(0.0, (self.iRec5[0]) as F32 / (std::cmp::max(self.iRec7[0], 1)) as F32));
														fTemp89 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp88)));
														fTemp90 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp88));
														fTemp91 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp88 + -0.6)));
														iTemp92 = i32::wrapping_add(self.iRec9[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp83, 7)));
														iTemp93 = i32::wrapping_mul(1664525, iTemp92 ^ (iTemp92 >> 15));
														fTemp94 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp93 ^ (iTemp93 >> 13))) as F32 + 0.5);
														fTemp95 = F32::floor(fTemp94);
														iTemp96 = (fTemp95) as i32;
														fTemp97 = fTemp94 - fTemp95;
														iTemp98 = i32::wrapping_add(self.iRec9[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp83, 6)));
														iTemp99 = i32::wrapping_mul(1664525, iTemp98 ^ (iTemp98 >> 15));
														iTemp100 = i32::wrapping_add(self.iRec9[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp83, 4)));
														iTemp101 = i32::wrapping_mul(1664525, iTemp100 ^ (iTemp100 >> 15));
														iTemp102 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp101 ^ (iTemp101 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
														self.fRec23[0] = if iRec10 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp102, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp99 ^ (iTemp99 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp97 * (fTemp97 * (fTemp97 * (fTemp97 * (0.0013333 * fTemp97 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp96))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp96))) as F32)} else {self.fRec23[1]};
														fTemp103 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp88 + -0.1)));
														iTemp104 = i32::wrapping_add(self.iRec18[0], 628374);
														iTemp105 = i32::wrapping_mul(1664525, iTemp104 ^ (iTemp104 >> 15));
														iTemp106 = i32::wrapping_add(self.iRec9[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp83, 3)));
														iTemp107 = i32::wrapping_mul(1664525, iTemp106 ^ (iTemp106 >> 15));
														fTemp108 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp107 ^ (iTemp107 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp105 ^ (iTemp105 >> 13))) as F32 + 0.5));
														fTemp109 = F32::floor(fTemp108);
														iTemp110 = (fTemp109) as i32;
														fTemp111 = fTemp108 - fTemp109;
														fTemp112 = F32::max(1.0, F32::round(fSlow39 * ((fTemp111 * (fTemp111 * (fTemp111 * (fTemp111 * (0.0013333 * fTemp111 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp110))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp110))) as F32)));
														iTemp113 = i32::wrapping_add(self.iRec9[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp83, 5)));
														iTemp114 = i32::wrapping_mul(1664525, iTemp113 ^ (iTemp113 >> 15));
														fTemp115 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp114 ^ (iTemp114 >> 13))) as F32 + 0.5);
														fTemp116 = F32::floor(fTemp115);
														iTemp117 = (fTemp116) as i32;
														fTemp118 = fTemp115 - fTemp116;
														fTemp119 = (iTemp102) as F32;
														fTemp120 = 0.16666667 * (fTemp119 * (fTemp118 * (fTemp118 * (fTemp118 * (fTemp118 * (0.0013333 * fTemp118 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp117))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp117))) as F32);
														fTemp121 = F32::floor(fTemp120);
														iTemp122 = (fTemp121) as i32;
														fTemp123 = fTemp120 - fTemp121;
														fTemp124 = F32::round(fTemp112 * (fTemp123 * (fTemp123 * (fTemp123 * (fTemp123 * (0.0013333 * fTemp123 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp122))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp122))) as F32);
														self.fRec24[0] = if iRec10 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp124 == fTemp112) as i32 != 0 {fTemp112 + fTemp119} else {fTemp124}) / fTemp112, std::f32::consts::E)} else {self.fRec24[1]};
														self.fRec25[0] = if iRec10 != 0 {1.442695 * F32::log(fTemp112, std::f32::consts::E)} else {self.fRec25[1]};
														fTemp125 = self.fRec25[0] + self.fRec24[0] * fTemp103 * (2.0 - fTemp103) + self.fRec23[0] * CallsDsp_faustpower2_f(fTemp91) * (3.0 - 2.0 * fTemp91);
														fTemp126 = F32::floor(fTemp125);
														iTemp127 = (fTemp126) as i32;
														fTemp128 = fTemp125 - fTemp126;
														fTemp129 = self.fRec22[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fSlow29 * ((fTemp128 * (fTemp128 * (fTemp128 * (fTemp128 * (0.0013333 * fTemp128 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp127))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp127))) as F32)));
														self.fRec22[0] = fTemp129 - F32::floor(fTemp129);
														iTemp130 = i32::wrapping_add(self.iRec9[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp83, 2)));
														iTemp131 = i32::wrapping_mul(1664525, iTemp130 ^ (iTemp130 >> 15));
														self.fRec26[0] = if iRec10 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp131 ^ (iTemp131 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec26[1]};
														fTemp132 = self.fRec26[0] * F32::sin(6.2831855 * self.fRec22[0]) * self.fRec3[0] * fTemp8 * ((self.iRec7[0] > 0) as i32) as u32 as F32 * CallsDsp_faustpower2_f(fTemp90) * (3.0 - 2.0 * fTemp90) * CallsDsp_faustpower2_f(fTemp89) * (3.0 - 2.0 * fTemp89);
														fTemp133 = fTemp132 * F32::cos(fTemp87);
													}
														let mut fTemp134: F32 = ((iTemp7 != 0) as i32) as u32 as F32;
														if iTemp19 != 0 {
															iTemp135 = (self.iRec28[1] >= self.iRec29[1]) as i32;
															self.iRec28[0] = if iTemp135 != 0 {0} else {i32::wrapping_add(self.iRec28[1], 1)};
															self.iRec34[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow40, iTemp11), self.iRec34[1])), 12345);
															iTemp136 = i32::wrapping_add(self.iRec34[0], 104729);
															iTemp137 = i32::wrapping_mul(1664525, iTemp136 ^ (iTemp136 >> 15));
															iTemp138 = i32::wrapping_add(self.iRec32[1], 17);
															iTemp139 = i32::wrapping_mul(1664525, iTemp138 ^ (iTemp138 >> 15));
															iTemp140 = i32::wrapping_add(self.iRec31[1], 1);
															iTemp141 = iTemp135 & ((iTemp140 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp139 ^ (iTemp139 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
															iTemp142 = i32::wrapping_add(self.iRec34[0], 209458);
															iTemp143 = i32::wrapping_mul(1664525, iTemp142 ^ (iTemp142 >> 15));
															iTemp144 = if (iTemp141 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp137 ^ (iTemp137 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp40 != 0 {i32::wrapping_mul(1013904223, iTemp143 ^ (iTemp143 >> 13))} else {self.iRec32[1]};
															iTemp145 = i32::wrapping_add(iTemp144, 17);
															iTemp146 = i32::wrapping_mul(1664525, iTemp145 ^ (iTemp145 >> 15));
															iTemp147 = if iTemp135 != 0 {if iTemp141 != 0 {0} else {iTemp140}} else {self.iRec31[1]};
															iTemp148 = i32::wrapping_add(iTemp144, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp147), 1)));
															iTemp149 = i32::wrapping_mul(1664525, iTemp148 ^ (iTemp148 >> 15));
															fTemp150 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp149 ^ (iTemp149 >> 13))) as F32 + 0.5);
															fTemp151 = F32::floor(fTemp150);
															iTemp152 = (fTemp151) as i32;
															fTemp153 = fTemp150 - fTemp151;
															iTemp154 = i32::wrapping_add(self.iRec34[0], 314187);
															iTemp155 = i32::wrapping_mul(1664525, iTemp154 ^ (iTemp154 >> 15));
															fTemp156 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp155 ^ (iTemp155 >> 13))) as F32 + 1.0;
															iTemp157 = i32::wrapping_add(self.iRec34[0], 418916);
															iTemp158 = i32::wrapping_mul(1664525, iTemp157 ^ (iTemp157 >> 15));
															fTemp159 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp158 ^ (iTemp158 >> 13))) as F32;
															fTemp160 = fTemp159 + 1.5;
															fTemp161 = 0.5 - fTemp159;
															fTemp162 = CallsDsp_faustpower2_f(fTemp161) / CallsDsp_faustpower2_f(fTemp160);
															iTemp163 = i32::wrapping_add(iTemp144, i32::wrapping_mul(190681, iTemp147));
															iTemp164 = i32::wrapping_mul(1664525, iTemp163 ^ (iTemp163 >> 15));
															fTemp165 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp164 ^ (iTemp164 >> 13))) as F32 + 0.5);
															fTemp166 = F32::floor(fTemp165);
															iTemp167 = (fTemp166) as i32;
															fTemp168 = fTemp165 - fTemp166;
															iTemp169 = if iTemp135 != 0 {i32::wrapping_mul(iTemp81, (fSlow48 * ((fTemp168 * (fTemp168 * (fTemp168 * (fTemp168 * (0.0013333 * fTemp168 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp167))) as F32 * fTemp156 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp167))) as F32))) as i32)} else {self.iRec30[1]};
															self.iRec29[0] = if iTemp135 != 0 {i32::wrapping_add(iTemp169, (self.fConst0 * if (((i32::wrapping_add(iTemp147, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp146 ^ (iTemp146 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp40 != 0 {1.2e+02 * (fTemp74 * fTemp161 * (15.0 - 4.0 * fTemp162) / (fTemp72 * fTemp160 * (15.0 - 9.0 * fTemp162)))} else {fSlow47 * (fTemp156 * (fTemp153 * (fTemp153 * (fTemp153 * (fTemp153 * (0.0013333 * fTemp153 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp152))) as F32 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp152))) as F32))}) as i32)} else {self.iRec29[1]};
															self.iRec30[0] = iTemp169;
															self.iRec31[0] = iTemp147;
															self.iRec32[0] = iTemp144;
															iRec33 = iTemp135;
															iTemp170 = i32::wrapping_mul(31, self.iRec31[0]);
															iTemp171 = i32::wrapping_add(self.iRec32[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp170, 8)));
															iTemp172 = i32::wrapping_mul(1664525, iTemp171 ^ (iTemp171 >> 15));
															self.fRec27[0] = if iRec33 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp172 ^ (iTemp172 >> 13))) as F32 + 0.5} else {self.fRec27[1]};
															fTemp173 = 1.5707964 * (fTemp86 * (self.fRec27[0] + -0.5) + 0.5);
															fTemp174 = F32::min(1.0, F32::max(0.0, (self.iRec28[0]) as F32 / (std::cmp::max(self.iRec30[0], 1)) as F32));
															fTemp175 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp174)));
															fTemp176 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp174));
															fTemp177 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp174 + -0.6)));
															iTemp178 = i32::wrapping_add(self.iRec32[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp170, 7)));
															iTemp179 = i32::wrapping_mul(1664525, iTemp178 ^ (iTemp178 >> 15));
															fTemp180 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp179 ^ (iTemp179 >> 13))) as F32 + 0.5);
															fTemp181 = F32::floor(fTemp180);
															iTemp182 = (fTemp181) as i32;
															fTemp183 = fTemp180 - fTemp181;
															iTemp184 = i32::wrapping_add(self.iRec32[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp170, 6)));
															iTemp185 = i32::wrapping_mul(1664525, iTemp184 ^ (iTemp184 >> 15));
															iTemp186 = i32::wrapping_add(self.iRec32[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp170, 4)));
															iTemp187 = i32::wrapping_mul(1664525, iTemp186 ^ (iTemp186 >> 15));
															iTemp188 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp187 ^ (iTemp187 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
															self.fRec36[0] = if iRec33 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp188, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp185 ^ (iTemp185 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp183 * (fTemp183 * (fTemp183 * (fTemp183 * (0.0013333 * fTemp183 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp182))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp182))) as F32)} else {self.fRec36[1]};
															fTemp189 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp174 + -0.1)));
															iTemp190 = i32::wrapping_add(self.iRec34[0], 628374);
															iTemp191 = i32::wrapping_mul(1664525, iTemp190 ^ (iTemp190 >> 15));
															iTemp192 = i32::wrapping_add(self.iRec32[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp170, 3)));
															iTemp193 = i32::wrapping_mul(1664525, iTemp192 ^ (iTemp192 >> 15));
															fTemp194 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp193 ^ (iTemp193 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp191 ^ (iTemp191 >> 13))) as F32 + 0.5));
															fTemp195 = F32::floor(fTemp194);
															iTemp196 = (fTemp195) as i32;
															fTemp197 = fTemp194 - fTemp195;
															fTemp198 = F32::max(1.0, F32::round(fSlow49 * ((fTemp197 * (fTemp197 * (fTemp197 * (fTemp197 * (0.0013333 * fTemp197 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp196))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp196))) as F32)));
															iTemp199 = i32::wrapping_add(self.iRec32[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp170, 5)));
															iTemp200 = i32::wrapping_mul(1664525, iTemp199 ^ (iTemp199 >> 15));
															fTemp201 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp200 ^ (iTemp200 >> 13))) as F32 + 0.5);
															fTemp202 = F32::floor(fTemp201);
															iTemp203 = (fTemp202) as i32;
															fTemp204 = fTemp201 - fTemp202;
															fTemp205 = (iTemp188) as F32;
															fTemp206 = 0.16666667 * (fTemp205 * (fTemp204 * (fTemp204 * (fTemp204 * (fTemp204 * (0.0013333 * fTemp204 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp203))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp203))) as F32);
															fTemp207 = F32::floor(fTemp206);
															iTemp208 = (fTemp207) as i32;
															fTemp209 = fTemp206 - fTemp207;
															fTemp210 = F32::round(fTemp198 * (fTemp209 * (fTemp209 * (fTemp209 * (fTemp209 * (0.0013333 * fTemp209 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp208))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp208))) as F32);
															self.fRec37[0] = if iRec33 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp210 == fTemp198) as i32 != 0 {fTemp198 + fTemp205} else {fTemp210}) / fTemp198, std::f32::consts::E)} else {self.fRec37[1]};
															self.fRec38[0] = if iRec33 != 0 {1.442695 * F32::log(fTemp198, std::f32::consts::E)} else {self.fRec38[1]};
															fTemp211 = self.fRec38[0] + self.fRec37[0] * fTemp189 * (2.0 - fTemp189) + self.fRec36[0] * CallsDsp_faustpower2_f(fTemp177) * (3.0 - 2.0 * fTemp177);
															fTemp212 = F32::floor(fTemp211);
															iTemp213 = (fTemp212) as i32;
															fTemp214 = fTemp211 - fTemp212;
															fTemp215 = self.fRec35[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fSlow41 * ((fTemp214 * (fTemp214 * (fTemp214 * (fTemp214 * (0.0013333 * fTemp214 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp213))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp213))) as F32)));
															self.fRec35[0] = fTemp215 - F32::floor(fTemp215);
															iTemp216 = i32::wrapping_add(self.iRec32[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp170, 2)));
															iTemp217 = i32::wrapping_mul(1664525, iTemp216 ^ (iTemp216 >> 15));
															self.fRec39[0] = if iRec33 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp217 ^ (iTemp217 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec39[1]};
															fTemp218 = self.fRec39[0] * F32::sin(6.2831855 * self.fRec35[0]) * self.fRec12[0] * fTemp18 * ((self.iRec30[0] > 0) as i32) as u32 as F32 * CallsDsp_faustpower2_f(fTemp176) * (3.0 - 2.0 * fTemp176) * CallsDsp_faustpower2_f(fTemp175) * (3.0 - 2.0 * fTemp175);
															fTemp219 = fTemp218 * F32::cos(fTemp173);
														}
															let mut fTemp220: F32 = ((iTemp17 != 0) as i32) as u32 as F32;
															if iTemp23 != 0 {
																iTemp221 = (self.iRec41[1] >= self.iRec42[1]) as i32;
																self.iRec41[0] = if iTemp221 != 0 {0} else {i32::wrapping_add(self.iRec41[1], 1)};
																self.iRec47[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow50, iTemp11), self.iRec47[1])), 12345);
																iTemp222 = i32::wrapping_add(self.iRec47[0], 104729);
																iTemp223 = i32::wrapping_mul(1664525, iTemp222 ^ (iTemp222 >> 15));
																iTemp224 = i32::wrapping_add(self.iRec45[1], 17);
																iTemp225 = i32::wrapping_mul(1664525, iTemp224 ^ (iTemp224 >> 15));
																iTemp226 = i32::wrapping_add(self.iRec44[1], 1);
																iTemp227 = iTemp221 & ((iTemp226 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp225 ^ (iTemp225 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
																iTemp228 = i32::wrapping_add(self.iRec47[0], 209458);
																iTemp229 = i32::wrapping_mul(1664525, iTemp228 ^ (iTemp228 >> 15));
																iTemp230 = if (iTemp227 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp223 ^ (iTemp223 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp40 != 0 {i32::wrapping_mul(1013904223, iTemp229 ^ (iTemp229 >> 13))} else {self.iRec45[1]};
																iTemp231 = i32::wrapping_add(iTemp230, 17);
																iTemp232 = i32::wrapping_mul(1664525, iTemp231 ^ (iTemp231 >> 15));
																iTemp233 = if iTemp221 != 0 {if iTemp227 != 0 {0} else {iTemp226}} else {self.iRec44[1]};
																iTemp234 = i32::wrapping_add(iTemp230, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp233), 1)));
																iTemp235 = i32::wrapping_mul(1664525, iTemp234 ^ (iTemp234 >> 15));
																fTemp236 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp235 ^ (iTemp235 >> 13))) as F32 + 0.5);
																fTemp237 = F32::floor(fTemp236);
																iTemp238 = (fTemp237) as i32;
																fTemp239 = fTemp236 - fTemp237;
																iTemp240 = i32::wrapping_add(self.iRec47[0], 314187);
																iTemp241 = i32::wrapping_mul(1664525, iTemp240 ^ (iTemp240 >> 15));
																fTemp242 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp241 ^ (iTemp241 >> 13))) as F32 + 1.0;
																iTemp243 = i32::wrapping_add(self.iRec47[0], 418916);
																iTemp244 = i32::wrapping_mul(1664525, iTemp243 ^ (iTemp243 >> 15));
																fTemp245 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp244 ^ (iTemp244 >> 13))) as F32;
																fTemp246 = fTemp245 + 1.5;
																fTemp247 = 0.5 - fTemp245;
																fTemp248 = CallsDsp_faustpower2_f(fTemp247) / CallsDsp_faustpower2_f(fTemp246);
																iTemp249 = i32::wrapping_add(iTemp230, i32::wrapping_mul(190681, iTemp233));
																iTemp250 = i32::wrapping_mul(1664525, iTemp249 ^ (iTemp249 >> 15));
																fTemp251 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp250 ^ (iTemp250 >> 13))) as F32 + 0.5);
																fTemp252 = F32::floor(fTemp251);
																iTemp253 = (fTemp252) as i32;
																fTemp254 = fTemp251 - fTemp252;
																iTemp255 = if iTemp221 != 0 {i32::wrapping_mul(iTemp81, (fSlow58 * ((fTemp254 * (fTemp254 * (fTemp254 * (fTemp254 * (0.0013333 * fTemp254 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp253))) as F32 * fTemp242 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp253))) as F32))) as i32)} else {self.iRec43[1]};
																self.iRec42[0] = if iTemp221 != 0 {i32::wrapping_add(iTemp255, (self.fConst0 * if (((i32::wrapping_add(iTemp233, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp232 ^ (iTemp232 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp40 != 0 {1.2e+02 * (fTemp74 * fTemp247 * (15.0 - 4.0 * fTemp248) / (fTemp72 * fTemp246 * (15.0 - 9.0 * fTemp248)))} else {fSlow57 * (fTemp242 * (fTemp239 * (fTemp239 * (fTemp239 * (fTemp239 * (0.0013333 * fTemp239 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp238))) as F32 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp238))) as F32))}) as i32)} else {self.iRec42[1]};
																self.iRec43[0] = iTemp255;
																self.iRec44[0] = iTemp233;
																self.iRec45[0] = iTemp230;
																iRec46 = iTemp221;
																iTemp256 = i32::wrapping_mul(31, self.iRec44[0]);
																iTemp257 = i32::wrapping_add(self.iRec45[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp256, 8)));
																iTemp258 = i32::wrapping_mul(1664525, iTemp257 ^ (iTemp257 >> 15));
																self.fRec40[0] = if iRec46 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp258 ^ (iTemp258 >> 13))) as F32 + 0.5} else {self.fRec40[1]};
																fTemp259 = 1.5707964 * (fTemp86 * (self.fRec40[0] + -0.5) + 0.5);
																fTemp260 = F32::min(1.0, F32::max(0.0, (self.iRec41[0]) as F32 / (std::cmp::max(self.iRec43[0], 1)) as F32));
																fTemp261 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp260)));
																fTemp262 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp260));
																fTemp263 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp260 + -0.6)));
																iTemp264 = i32::wrapping_add(self.iRec45[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp256, 7)));
																iTemp265 = i32::wrapping_mul(1664525, iTemp264 ^ (iTemp264 >> 15));
																fTemp266 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp265 ^ (iTemp265 >> 13))) as F32 + 0.5);
																fTemp267 = F32::floor(fTemp266);
																iTemp268 = (fTemp267) as i32;
																fTemp269 = fTemp266 - fTemp267;
																iTemp270 = i32::wrapping_add(self.iRec45[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp256, 6)));
																iTemp271 = i32::wrapping_mul(1664525, iTemp270 ^ (iTemp270 >> 15));
																iTemp272 = i32::wrapping_add(self.iRec45[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp256, 4)));
																iTemp273 = i32::wrapping_mul(1664525, iTemp272 ^ (iTemp272 >> 15));
																iTemp274 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp273 ^ (iTemp273 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
																self.fRec49[0] = if iRec46 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp274, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp271 ^ (iTemp271 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp269 * (fTemp269 * (fTemp269 * (fTemp269 * (0.0013333 * fTemp269 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp268))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp268))) as F32)} else {self.fRec49[1]};
																fTemp275 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp260 + -0.1)));
																iTemp276 = i32::wrapping_add(self.iRec47[0], 628374);
																iTemp277 = i32::wrapping_mul(1664525, iTemp276 ^ (iTemp276 >> 15));
																iTemp278 = i32::wrapping_add(self.iRec45[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp256, 3)));
																iTemp279 = i32::wrapping_mul(1664525, iTemp278 ^ (iTemp278 >> 15));
																fTemp280 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp279 ^ (iTemp279 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp277 ^ (iTemp277 >> 13))) as F32 + 0.5));
																fTemp281 = F32::floor(fTemp280);
																iTemp282 = (fTemp281) as i32;
																fTemp283 = fTemp280 - fTemp281;
																fTemp284 = F32::max(1.0, F32::round(fSlow59 * ((fTemp283 * (fTemp283 * (fTemp283 * (fTemp283 * (0.0013333 * fTemp283 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp282))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp282))) as F32)));
																iTemp285 = i32::wrapping_add(self.iRec45[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp256, 5)));
																iTemp286 = i32::wrapping_mul(1664525, iTemp285 ^ (iTemp285 >> 15));
																fTemp287 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp286 ^ (iTemp286 >> 13))) as F32 + 0.5);
																fTemp288 = F32::floor(fTemp287);
																iTemp289 = (fTemp288) as i32;
																fTemp290 = fTemp287 - fTemp288;
																fTemp291 = (iTemp274) as F32;
																fTemp292 = 0.16666667 * (fTemp291 * (fTemp290 * (fTemp290 * (fTemp290 * (fTemp290 * (0.0013333 * fTemp290 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp289))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp289))) as F32);
																fTemp293 = F32::floor(fTemp292);
																iTemp294 = (fTemp293) as i32;
																fTemp295 = fTemp292 - fTemp293;
																fTemp296 = F32::round(fTemp284 * (fTemp295 * (fTemp295 * (fTemp295 * (fTemp295 * (0.0013333 * fTemp295 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp294))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp294))) as F32);
																self.fRec50[0] = if iRec46 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp296 == fTemp284) as i32 != 0 {fTemp284 + fTemp291} else {fTemp296}) / fTemp284, std::f32::consts::E)} else {self.fRec50[1]};
																self.fRec51[0] = if iRec46 != 0 {1.442695 * F32::log(fTemp284, std::f32::consts::E)} else {self.fRec51[1]};
																fTemp297 = self.fRec51[0] + self.fRec50[0] * fTemp275 * (2.0 - fTemp275) + self.fRec49[0] * CallsDsp_faustpower2_f(fTemp263) * (3.0 - 2.0 * fTemp263);
																fTemp298 = F32::floor(fTemp297);
																iTemp299 = (fTemp298) as i32;
																fTemp300 = fTemp297 - fTemp298;
																fTemp301 = self.fRec48[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fSlow51 * ((fTemp300 * (fTemp300 * (fTemp300 * (fTemp300 * (0.0013333 * fTemp300 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp299))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp299))) as F32)));
																self.fRec48[0] = fTemp301 - F32::floor(fTemp301);
																iTemp302 = i32::wrapping_add(self.iRec45[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp256, 2)));
																iTemp303 = i32::wrapping_mul(1664525, iTemp302 ^ (iTemp302 >> 15));
																self.fRec52[0] = if iRec46 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp303 ^ (iTemp303 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec52[1]};
																fTemp304 = self.fRec52[0] * F32::sin(6.2831855 * self.fRec48[0]) * self.fRec13[0] * fTemp22 * ((self.iRec43[0] > 0) as i32) as u32 as F32 * CallsDsp_faustpower2_f(fTemp262) * (3.0 - 2.0 * fTemp262) * CallsDsp_faustpower2_f(fTemp261) * (3.0 - 2.0 * fTemp261);
																fTemp305 = fTemp304 * F32::cos(fTemp259);
															}
																let mut fTemp306: F32 = ((iTemp21 != 0) as i32) as u32 as F32;
																if iTemp27 != 0 {
																	iTemp307 = (self.iRec54[1] >= self.iRec55[1]) as i32;
																	self.iRec54[0] = if iTemp307 != 0 {0} else {i32::wrapping_add(self.iRec54[1], 1)};
																	self.iRec60[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow60, iTemp11), self.iRec60[1])), 12345);
																	iTemp308 = i32::wrapping_add(self.iRec60[0], 104729);
																	iTemp309 = i32::wrapping_mul(1664525, iTemp308 ^ (iTemp308 >> 15));
																	iTemp310 = i32::wrapping_add(self.iRec58[1], 17);
																	iTemp311 = i32::wrapping_mul(1664525, iTemp310 ^ (iTemp310 >> 15));
																	iTemp312 = i32::wrapping_add(self.iRec57[1], 1);
																	iTemp313 = iTemp307 & ((iTemp312 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp311 ^ (iTemp311 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
																	iTemp314 = i32::wrapping_add(self.iRec60[0], 209458);
																	iTemp315 = i32::wrapping_mul(1664525, iTemp314 ^ (iTemp314 >> 15));
																	iTemp316 = if (iTemp313 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp309 ^ (iTemp309 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp40 != 0 {i32::wrapping_mul(1013904223, iTemp315 ^ (iTemp315 >> 13))} else {self.iRec58[1]};
																	iTemp317 = i32::wrapping_add(iTemp316, 17);
																	iTemp318 = i32::wrapping_mul(1664525, iTemp317 ^ (iTemp317 >> 15));
																	iTemp319 = if iTemp307 != 0 {if iTemp313 != 0 {0} else {iTemp312}} else {self.iRec57[1]};
																	iTemp320 = i32::wrapping_add(iTemp316, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp319), 1)));
																	iTemp321 = i32::wrapping_mul(1664525, iTemp320 ^ (iTemp320 >> 15));
																	fTemp322 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp321 ^ (iTemp321 >> 13))) as F32 + 0.5);
																	fTemp323 = F32::floor(fTemp322);
																	iTemp324 = (fTemp323) as i32;
																	fTemp325 = fTemp322 - fTemp323;
																	iTemp326 = i32::wrapping_add(self.iRec60[0], 314187);
																	iTemp327 = i32::wrapping_mul(1664525, iTemp326 ^ (iTemp326 >> 15));
																	fTemp328 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp327 ^ (iTemp327 >> 13))) as F32 + 1.0;
																	iTemp329 = i32::wrapping_add(self.iRec60[0], 418916);
																	iTemp330 = i32::wrapping_mul(1664525, iTemp329 ^ (iTemp329 >> 15));
																	fTemp331 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp330 ^ (iTemp330 >> 13))) as F32;
																	fTemp332 = fTemp331 + 1.5;
																	fTemp333 = 0.5 - fTemp331;
																	fTemp334 = CallsDsp_faustpower2_f(fTemp333) / CallsDsp_faustpower2_f(fTemp332);
																	iTemp335 = i32::wrapping_add(iTemp316, i32::wrapping_mul(190681, iTemp319));
																	iTemp336 = i32::wrapping_mul(1664525, iTemp335 ^ (iTemp335 >> 15));
																	fTemp337 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp336 ^ (iTemp336 >> 13))) as F32 + 0.5);
																	fTemp338 = F32::floor(fTemp337);
																	iTemp339 = (fTemp338) as i32;
																	fTemp340 = fTemp337 - fTemp338;
																	iTemp341 = if iTemp307 != 0 {i32::wrapping_mul(iTemp81, (fSlow68 * ((fTemp340 * (fTemp340 * (fTemp340 * (fTemp340 * (0.0013333 * fTemp340 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp339))) as F32 * fTemp328 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp339))) as F32))) as i32)} else {self.iRec56[1]};
																	self.iRec55[0] = if iTemp307 != 0 {i32::wrapping_add(iTemp341, (self.fConst0 * if (((i32::wrapping_add(iTemp319, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp318 ^ (iTemp318 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp40 != 0 {1.2e+02 * (fTemp74 * fTemp333 * (15.0 - 4.0 * fTemp334) / (fTemp72 * fTemp332 * (15.0 - 9.0 * fTemp334)))} else {fSlow67 * (fTemp328 * (fTemp325 * (fTemp325 * (fTemp325 * (fTemp325 * (0.0013333 * fTemp325 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp324))) as F32 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp324))) as F32))}) as i32)} else {self.iRec55[1]};
																	self.iRec56[0] = iTemp341;
																	self.iRec57[0] = iTemp319;
																	self.iRec58[0] = iTemp316;
																	iRec59 = iTemp307;
																	iTemp342 = i32::wrapping_mul(31, self.iRec57[0]);
																	iTemp343 = i32::wrapping_add(self.iRec58[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp342, 8)));
																	iTemp344 = i32::wrapping_mul(1664525, iTemp343 ^ (iTemp343 >> 15));
																	self.fRec53[0] = if iRec59 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp344 ^ (iTemp344 >> 13))) as F32 + 0.5} else {self.fRec53[1]};
																	fTemp345 = 1.5707964 * (fTemp86 * (self.fRec53[0] + -0.5) + 0.5);
																	fTemp346 = F32::min(1.0, F32::max(0.0, (self.iRec54[0]) as F32 / (std::cmp::max(self.iRec56[0], 1)) as F32));
																	fTemp347 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp346)));
																	fTemp348 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp346));
																	fTemp349 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp346 + -0.6)));
																	iTemp350 = i32::wrapping_add(self.iRec58[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp342, 7)));
																	iTemp351 = i32::wrapping_mul(1664525, iTemp350 ^ (iTemp350 >> 15));
																	fTemp352 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp351 ^ (iTemp351 >> 13))) as F32 + 0.5);
																	fTemp353 = F32::floor(fTemp352);
																	iTemp354 = (fTemp353) as i32;
																	fTemp355 = fTemp352 - fTemp353;
																	iTemp356 = i32::wrapping_add(self.iRec58[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp342, 6)));
																	iTemp357 = i32::wrapping_mul(1664525, iTemp356 ^ (iTemp356 >> 15));
																	iTemp358 = i32::wrapping_add(self.iRec58[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp342, 4)));
																	iTemp359 = i32::wrapping_mul(1664525, iTemp358 ^ (iTemp358 >> 15));
																	iTemp360 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp359 ^ (iTemp359 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
																	self.fRec62[0] = if iRec59 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp360, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp357 ^ (iTemp357 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp355 * (fTemp355 * (fTemp355 * (fTemp355 * (0.0013333 * fTemp355 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp354))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp354))) as F32)} else {self.fRec62[1]};
																	fTemp361 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp346 + -0.1)));
																	iTemp362 = i32::wrapping_add(self.iRec60[0], 628374);
																	iTemp363 = i32::wrapping_mul(1664525, iTemp362 ^ (iTemp362 >> 15));
																	iTemp364 = i32::wrapping_add(self.iRec58[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp342, 3)));
																	iTemp365 = i32::wrapping_mul(1664525, iTemp364 ^ (iTemp364 >> 15));
																	fTemp366 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp365 ^ (iTemp365 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp363 ^ (iTemp363 >> 13))) as F32 + 0.5));
																	fTemp367 = F32::floor(fTemp366);
																	iTemp368 = (fTemp367) as i32;
																	fTemp369 = fTemp366 - fTemp367;
																	fTemp370 = F32::max(1.0, F32::round(fSlow69 * ((fTemp369 * (fTemp369 * (fTemp369 * (fTemp369 * (0.0013333 * fTemp369 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp368))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp368))) as F32)));
																	iTemp371 = i32::wrapping_add(self.iRec58[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp342, 5)));
																	iTemp372 = i32::wrapping_mul(1664525, iTemp371 ^ (iTemp371 >> 15));
																	fTemp373 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp372 ^ (iTemp372 >> 13))) as F32 + 0.5);
																	fTemp374 = F32::floor(fTemp373);
																	iTemp375 = (fTemp374) as i32;
																	fTemp376 = fTemp373 - fTemp374;
																	fTemp377 = (iTemp360) as F32;
																	fTemp378 = 0.16666667 * (fTemp377 * (fTemp376 * (fTemp376 * (fTemp376 * (fTemp376 * (0.0013333 * fTemp376 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp375))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp375))) as F32);
																	fTemp379 = F32::floor(fTemp378);
																	iTemp380 = (fTemp379) as i32;
																	fTemp381 = fTemp378 - fTemp379;
																	fTemp382 = F32::round(fTemp370 * (fTemp381 * (fTemp381 * (fTemp381 * (fTemp381 * (0.0013333 * fTemp381 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp380))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp380))) as F32);
																	self.fRec63[0] = if iRec59 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp382 == fTemp370) as i32 != 0 {fTemp370 + fTemp377} else {fTemp382}) / fTemp370, std::f32::consts::E)} else {self.fRec63[1]};
																	self.fRec64[0] = if iRec59 != 0 {1.442695 * F32::log(fTemp370, std::f32::consts::E)} else {self.fRec64[1]};
																	fTemp383 = self.fRec64[0] + self.fRec63[0] * fTemp361 * (2.0 - fTemp361) + self.fRec62[0] * CallsDsp_faustpower2_f(fTemp349) * (3.0 - 2.0 * fTemp349);
																	fTemp384 = F32::floor(fTemp383);
																	iTemp385 = (fTemp384) as i32;
																	fTemp386 = fTemp383 - fTemp384;
																	fTemp387 = self.fRec61[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fSlow61 * ((fTemp386 * (fTemp386 * (fTemp386 * (fTemp386 * (0.0013333 * fTemp386 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp385))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp385))) as F32)));
																	self.fRec61[0] = fTemp387 - F32::floor(fTemp387);
																	iTemp388 = i32::wrapping_add(self.iRec58[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp342, 2)));
																	iTemp389 = i32::wrapping_mul(1664525, iTemp388 ^ (iTemp388 >> 15));
																	self.fRec65[0] = if iRec59 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp389 ^ (iTemp389 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec65[1]};
																	fTemp390 = self.fRec65[0] * F32::sin(6.2831855 * self.fRec61[0]) * self.fRec14[0] * fTemp26 * ((self.iRec56[0] > 0) as i32) as u32 as F32 * CallsDsp_faustpower2_f(fTemp348) * (3.0 - 2.0 * fTemp348) * CallsDsp_faustpower2_f(fTemp347) * (3.0 - 2.0 * fTemp347);
																	fTemp391 = fTemp390 * F32::cos(fTemp345);
																}
																	let mut fTemp392: F32 = ((iTemp25 != 0) as i32) as u32 as F32;
																	if iTemp31 != 0 {
																		iTemp393 = (self.iRec67[1] >= self.iRec68[1]) as i32;
																		self.iRec67[0] = if iTemp393 != 0 {0} else {i32::wrapping_add(self.iRec67[1], 1)};
																		self.iRec73[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow70, iTemp11), self.iRec73[1])), 12345);
																		iTemp394 = i32::wrapping_add(self.iRec73[0], 104729);
																		iTemp395 = i32::wrapping_mul(1664525, iTemp394 ^ (iTemp394 >> 15));
																		iTemp396 = i32::wrapping_add(self.iRec71[1], 17);
																		iTemp397 = i32::wrapping_mul(1664525, iTemp396 ^ (iTemp396 >> 15));
																		iTemp398 = i32::wrapping_add(self.iRec70[1], 1);
																		iTemp399 = iTemp393 & ((iTemp398 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp397 ^ (iTemp397 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
																		iTemp400 = i32::wrapping_add(self.iRec73[0], 209458);
																		iTemp401 = i32::wrapping_mul(1664525, iTemp400 ^ (iTemp400 >> 15));
																		iTemp402 = if (iTemp399 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp395 ^ (iTemp395 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp40 != 0 {i32::wrapping_mul(1013904223, iTemp401 ^ (iTemp401 >> 13))} else {self.iRec71[1]};
																		iTemp403 = i32::wrapping_add(iTemp402, 17);
																		iTemp404 = i32::wrapping_mul(1664525, iTemp403 ^ (iTemp403 >> 15));
																		iTemp405 = if iTemp393 != 0 {if iTemp399 != 0 {0} else {iTemp398}} else {self.iRec70[1]};
																		iTemp406 = i32::wrapping_add(iTemp402, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp405), 1)));
																		iTemp407 = i32::wrapping_mul(1664525, iTemp406 ^ (iTemp406 >> 15));
																		fTemp408 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp407 ^ (iTemp407 >> 13))) as F32 + 0.5);
																		fTemp409 = F32::floor(fTemp408);
																		iTemp410 = (fTemp409) as i32;
																		fTemp411 = fTemp408 - fTemp409;
																		iTemp412 = i32::wrapping_add(self.iRec73[0], 314187);
																		iTemp413 = i32::wrapping_mul(1664525, iTemp412 ^ (iTemp412 >> 15));
																		fTemp414 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp413 ^ (iTemp413 >> 13))) as F32 + 1.0;
																		iTemp415 = i32::wrapping_add(self.iRec73[0], 418916);
																		iTemp416 = i32::wrapping_mul(1664525, iTemp415 ^ (iTemp415 >> 15));
																		fTemp417 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp416 ^ (iTemp416 >> 13))) as F32;
																		fTemp418 = fTemp417 + 1.5;
																		fTemp419 = 0.5 - fTemp417;
																		fTemp420 = CallsDsp_faustpower2_f(fTemp419) / CallsDsp_faustpower2_f(fTemp418);
																		iTemp421 = i32::wrapping_add(iTemp402, i32::wrapping_mul(190681, iTemp405));
																		iTemp422 = i32::wrapping_mul(1664525, iTemp421 ^ (iTemp421 >> 15));
																		fTemp423 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp422 ^ (iTemp422 >> 13))) as F32 + 0.5);
																		fTemp424 = F32::floor(fTemp423);
																		iTemp425 = (fTemp424) as i32;
																		fTemp426 = fTemp423 - fTemp424;
																		iTemp427 = if iTemp393 != 0 {i32::wrapping_mul(iTemp81, (fSlow78 * ((fTemp426 * (fTemp426 * (fTemp426 * (fTemp426 * (0.0013333 * fTemp426 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp425))) as F32 * fTemp414 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp425))) as F32))) as i32)} else {self.iRec69[1]};
																		self.iRec68[0] = if iTemp393 != 0 {i32::wrapping_add(iTemp427, (self.fConst0 * if (((i32::wrapping_add(iTemp405, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp404 ^ (iTemp404 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp40 != 0 {1.2e+02 * (fTemp74 * fTemp419 * (15.0 - 4.0 * fTemp420) / (fTemp72 * fTemp418 * (15.0 - 9.0 * fTemp420)))} else {fSlow77 * (fTemp414 * (fTemp411 * (fTemp411 * (fTemp411 * (fTemp411 * (0.0013333 * fTemp411 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp410))) as F32 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp410))) as F32))}) as i32)} else {self.iRec68[1]};
																		self.iRec69[0] = iTemp427;
																		self.iRec70[0] = iTemp405;
																		self.iRec71[0] = iTemp402;
																		iRec72 = iTemp393;
																		iTemp428 = i32::wrapping_mul(31, self.iRec70[0]);
																		iTemp429 = i32::wrapping_add(self.iRec71[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp428, 8)));
																		iTemp430 = i32::wrapping_mul(1664525, iTemp429 ^ (iTemp429 >> 15));
																		self.fRec66[0] = if iRec72 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp430 ^ (iTemp430 >> 13))) as F32 + 0.5} else {self.fRec66[1]};
																		fTemp431 = 1.5707964 * (fTemp86 * (self.fRec66[0] + -0.5) + 0.5);
																		fTemp432 = F32::min(1.0, F32::max(0.0, (self.iRec67[0]) as F32 / (std::cmp::max(self.iRec69[0], 1)) as F32));
																		fTemp433 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp432)));
																		fTemp434 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp432));
																		fTemp435 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp432 + -0.6)));
																		iTemp436 = i32::wrapping_add(self.iRec71[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp428, 7)));
																		iTemp437 = i32::wrapping_mul(1664525, iTemp436 ^ (iTemp436 >> 15));
																		fTemp438 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp437 ^ (iTemp437 >> 13))) as F32 + 0.5);
																		fTemp439 = F32::floor(fTemp438);
																		iTemp440 = (fTemp439) as i32;
																		fTemp441 = fTemp438 - fTemp439;
																		iTemp442 = i32::wrapping_add(self.iRec71[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp428, 6)));
																		iTemp443 = i32::wrapping_mul(1664525, iTemp442 ^ (iTemp442 >> 15));
																		iTemp444 = i32::wrapping_add(self.iRec71[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp428, 4)));
																		iTemp445 = i32::wrapping_mul(1664525, iTemp444 ^ (iTemp444 >> 15));
																		iTemp446 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp445 ^ (iTemp445 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
																		self.fRec75[0] = if iRec72 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp446, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp443 ^ (iTemp443 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp441 * (fTemp441 * (fTemp441 * (fTemp441 * (0.0013333 * fTemp441 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp440))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp440))) as F32)} else {self.fRec75[1]};
																		fTemp447 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp432 + -0.1)));
																		iTemp448 = i32::wrapping_add(self.iRec73[0], 628374);
																		iTemp449 = i32::wrapping_mul(1664525, iTemp448 ^ (iTemp448 >> 15));
																		iTemp450 = i32::wrapping_add(self.iRec71[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp428, 3)));
																		iTemp451 = i32::wrapping_mul(1664525, iTemp450 ^ (iTemp450 >> 15));
																		fTemp452 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp451 ^ (iTemp451 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp449 ^ (iTemp449 >> 13))) as F32 + 0.5));
																		fTemp453 = F32::floor(fTemp452);
																		iTemp454 = (fTemp453) as i32;
																		fTemp455 = fTemp452 - fTemp453;
																		fTemp456 = F32::max(1.0, F32::round(fSlow79 * ((fTemp455 * (fTemp455 * (fTemp455 * (fTemp455 * (0.0013333 * fTemp455 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp454))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp454))) as F32)));
																		iTemp457 = i32::wrapping_add(self.iRec71[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp428, 5)));
																		iTemp458 = i32::wrapping_mul(1664525, iTemp457 ^ (iTemp457 >> 15));
																		fTemp459 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp458 ^ (iTemp458 >> 13))) as F32 + 0.5);
																		fTemp460 = F32::floor(fTemp459);
																		iTemp461 = (fTemp460) as i32;
																		fTemp462 = fTemp459 - fTemp460;
																		fTemp463 = (iTemp446) as F32;
																		fTemp464 = 0.16666667 * (fTemp463 * (fTemp462 * (fTemp462 * (fTemp462 * (fTemp462 * (0.0013333 * fTemp462 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp461))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp461))) as F32);
																		fTemp465 = F32::floor(fTemp464);
																		iTemp466 = (fTemp465) as i32;
																		fTemp467 = fTemp464 - fTemp465;
																		fTemp468 = F32::round(fTemp456 * (fTemp467 * (fTemp467 * (fTemp467 * (fTemp467 * (0.0013333 * fTemp467 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp466))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp466))) as F32);
																		self.fRec76[0] = if iRec72 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp468 == fTemp456) as i32 != 0 {fTemp456 + fTemp463} else {fTemp468}) / fTemp456, std::f32::consts::E)} else {self.fRec76[1]};
																		self.fRec77[0] = if iRec72 != 0 {1.442695 * F32::log(fTemp456, std::f32::consts::E)} else {self.fRec77[1]};
																		fTemp469 = self.fRec77[0] + self.fRec76[0] * fTemp447 * (2.0 - fTemp447) + self.fRec75[0] * CallsDsp_faustpower2_f(fTemp435) * (3.0 - 2.0 * fTemp435);
																		fTemp470 = F32::floor(fTemp469);
																		iTemp471 = (fTemp470) as i32;
																		fTemp472 = fTemp469 - fTemp470;
																		fTemp473 = self.fRec74[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fSlow71 * ((fTemp472 * (fTemp472 * (fTemp472 * (fTemp472 * (0.0013333 * fTemp472 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp471))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp471))) as F32)));
																		self.fRec74[0] = fTemp473 - F32::floor(fTemp473);
																		iTemp474 = i32::wrapping_add(self.iRec71[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp428, 2)));
																		iTemp475 = i32::wrapping_mul(1664525, iTemp474 ^ (iTemp474 >> 15));
																		self.fRec78[0] = if iRec72 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp475 ^ (iTemp475 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec78[1]};
																		fTemp476 = self.fRec78[0] * F32::sin(6.2831855 * self.fRec74[0]) * self.fRec15[0] * fTemp30 * ((self.iRec69[0] > 0) as i32) as u32 as F32 * CallsDsp_faustpower2_f(fTemp434) * (3.0 - 2.0 * fTemp434) * CallsDsp_faustpower2_f(fTemp433) * (3.0 - 2.0 * fTemp433);
																		fTemp477 = fTemp476 * F32::cos(fTemp431);
																	}
																		let mut fTemp478: F32 = ((iTemp29 != 0) as i32) as u32 as F32;
																		if iTemp35 != 0 {
																			iTemp479 = (self.iRec80[1] >= self.iRec81[1]) as i32;
																			self.iRec80[0] = if iTemp479 != 0 {0} else {i32::wrapping_add(self.iRec80[1], 1)};
																			self.iRec86[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow80, iTemp11), self.iRec86[1])), 12345);
																			iTemp480 = i32::wrapping_add(self.iRec86[0], 104729);
																			iTemp481 = i32::wrapping_mul(1664525, iTemp480 ^ (iTemp480 >> 15));
																			iTemp482 = i32::wrapping_add(self.iRec84[1], 17);
																			iTemp483 = i32::wrapping_mul(1664525, iTemp482 ^ (iTemp482 >> 15));
																			iTemp484 = i32::wrapping_add(self.iRec83[1], 1);
																			iTemp485 = iTemp479 & ((iTemp484 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp483 ^ (iTemp483 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
																			iTemp486 = i32::wrapping_add(self.iRec86[0], 209458);
																			iTemp487 = i32::wrapping_mul(1664525, iTemp486 ^ (iTemp486 >> 15));
																			iTemp488 = if (iTemp485 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp481 ^ (iTemp481 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp40 != 0 {i32::wrapping_mul(1013904223, iTemp487 ^ (iTemp487 >> 13))} else {self.iRec84[1]};
																			iTemp489 = i32::wrapping_add(iTemp488, 17);
																			iTemp490 = i32::wrapping_mul(1664525, iTemp489 ^ (iTemp489 >> 15));
																			iTemp491 = if iTemp479 != 0 {if iTemp485 != 0 {0} else {iTemp484}} else {self.iRec83[1]};
																			iTemp492 = i32::wrapping_add(iTemp488, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp491), 1)));
																			iTemp493 = i32::wrapping_mul(1664525, iTemp492 ^ (iTemp492 >> 15));
																			fTemp494 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp493 ^ (iTemp493 >> 13))) as F32 + 0.5);
																			fTemp495 = F32::floor(fTemp494);
																			iTemp496 = (fTemp495) as i32;
																			fTemp497 = fTemp494 - fTemp495;
																			iTemp498 = i32::wrapping_add(self.iRec86[0], 314187);
																			iTemp499 = i32::wrapping_mul(1664525, iTemp498 ^ (iTemp498 >> 15));
																			fTemp500 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp499 ^ (iTemp499 >> 13))) as F32 + 1.0;
																			iTemp501 = i32::wrapping_add(self.iRec86[0], 418916);
																			iTemp502 = i32::wrapping_mul(1664525, iTemp501 ^ (iTemp501 >> 15));
																			fTemp503 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp502 ^ (iTemp502 >> 13))) as F32;
																			fTemp504 = fTemp503 + 1.5;
																			fTemp505 = 0.5 - fTemp503;
																			fTemp506 = CallsDsp_faustpower2_f(fTemp505) / CallsDsp_faustpower2_f(fTemp504);
																			iTemp507 = i32::wrapping_add(iTemp488, i32::wrapping_mul(190681, iTemp491));
																			iTemp508 = i32::wrapping_mul(1664525, iTemp507 ^ (iTemp507 >> 15));
																			fTemp509 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp508 ^ (iTemp508 >> 13))) as F32 + 0.5);
																			fTemp510 = F32::floor(fTemp509);
																			iTemp511 = (fTemp510) as i32;
																			fTemp512 = fTemp509 - fTemp510;
																			iTemp513 = if iTemp479 != 0 {i32::wrapping_mul(iTemp81, (fSlow88 * ((fTemp512 * (fTemp512 * (fTemp512 * (fTemp512 * (0.0013333 * fTemp512 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp511))) as F32 * fTemp500 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp511))) as F32))) as i32)} else {self.iRec82[1]};
																			self.iRec81[0] = if iTemp479 != 0 {i32::wrapping_add(iTemp513, (self.fConst0 * if (((i32::wrapping_add(iTemp491, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp490 ^ (iTemp490 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp40 != 0 {1.2e+02 * (fTemp74 * fTemp505 * (15.0 - 4.0 * fTemp506) / (fTemp72 * fTemp504 * (15.0 - 9.0 * fTemp506)))} else {fSlow87 * (fTemp500 * (fTemp497 * (fTemp497 * (fTemp497 * (fTemp497 * (0.0013333 * fTemp497 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp496))) as F32 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp496))) as F32))}) as i32)} else {self.iRec81[1]};
																			self.iRec82[0] = iTemp513;
																			self.iRec83[0] = iTemp491;
																			self.iRec84[0] = iTemp488;
																			iRec85 = iTemp479;
																			iTemp514 = i32::wrapping_mul(31, self.iRec83[0]);
																			iTemp515 = i32::wrapping_add(self.iRec84[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp514, 8)));
																			iTemp516 = i32::wrapping_mul(1664525, iTemp515 ^ (iTemp515 >> 15));
																			self.fRec79[0] = if iRec85 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp516 ^ (iTemp516 >> 13))) as F32 + 0.5} else {self.fRec79[1]};
																			fTemp517 = 1.5707964 * (fTemp86 * (self.fRec79[0] + -0.5) + 0.5);
																			fTemp518 = F32::min(1.0, F32::max(0.0, (self.iRec80[0]) as F32 / (std::cmp::max(self.iRec82[0], 1)) as F32));
																			fTemp519 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp518)));
																			fTemp520 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp518));
																			fTemp521 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp518 + -0.6)));
																			iTemp522 = i32::wrapping_add(self.iRec84[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp514, 7)));
																			iTemp523 = i32::wrapping_mul(1664525, iTemp522 ^ (iTemp522 >> 15));
																			fTemp524 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp523 ^ (iTemp523 >> 13))) as F32 + 0.5);
																			fTemp525 = F32::floor(fTemp524);
																			iTemp526 = (fTemp525) as i32;
																			fTemp527 = fTemp524 - fTemp525;
																			iTemp528 = i32::wrapping_add(self.iRec84[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp514, 6)));
																			iTemp529 = i32::wrapping_mul(1664525, iTemp528 ^ (iTemp528 >> 15));
																			iTemp530 = i32::wrapping_add(self.iRec84[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp514, 4)));
																			iTemp531 = i32::wrapping_mul(1664525, iTemp530 ^ (iTemp530 >> 15));
																			iTemp532 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp531 ^ (iTemp531 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
																			self.fRec88[0] = if iRec85 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp532, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp529 ^ (iTemp529 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp527 * (fTemp527 * (fTemp527 * (fTemp527 * (0.0013333 * fTemp527 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp526))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp526))) as F32)} else {self.fRec88[1]};
																			fTemp533 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp518 + -0.1)));
																			iTemp534 = i32::wrapping_add(self.iRec86[0], 628374);
																			iTemp535 = i32::wrapping_mul(1664525, iTemp534 ^ (iTemp534 >> 15));
																			iTemp536 = i32::wrapping_add(self.iRec84[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp514, 3)));
																			iTemp537 = i32::wrapping_mul(1664525, iTemp536 ^ (iTemp536 >> 15));
																			fTemp538 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp537 ^ (iTemp537 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp535 ^ (iTemp535 >> 13))) as F32 + 0.5));
																			fTemp539 = F32::floor(fTemp538);
																			iTemp540 = (fTemp539) as i32;
																			fTemp541 = fTemp538 - fTemp539;
																			fTemp542 = F32::max(1.0, F32::round(fSlow89 * ((fTemp541 * (fTemp541 * (fTemp541 * (fTemp541 * (0.0013333 * fTemp541 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp540))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp540))) as F32)));
																			iTemp543 = i32::wrapping_add(self.iRec84[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp514, 5)));
																			iTemp544 = i32::wrapping_mul(1664525, iTemp543 ^ (iTemp543 >> 15));
																			fTemp545 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp544 ^ (iTemp544 >> 13))) as F32 + 0.5);
																			fTemp546 = F32::floor(fTemp545);
																			iTemp547 = (fTemp546) as i32;
																			fTemp548 = fTemp545 - fTemp546;
																			fTemp549 = (iTemp532) as F32;
																			fTemp550 = 0.16666667 * (fTemp549 * (fTemp548 * (fTemp548 * (fTemp548 * (fTemp548 * (0.0013333 * fTemp548 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp547))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp547))) as F32);
																			fTemp551 = F32::floor(fTemp550);
																			iTemp552 = (fTemp551) as i32;
																			fTemp553 = fTemp550 - fTemp551;
																			fTemp554 = F32::round(fTemp542 * (fTemp553 * (fTemp553 * (fTemp553 * (fTemp553 * (0.0013333 * fTemp553 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp552))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp552))) as F32);
																			self.fRec89[0] = if iRec85 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp554 == fTemp542) as i32 != 0 {fTemp542 + fTemp549} else {fTemp554}) / fTemp542, std::f32::consts::E)} else {self.fRec89[1]};
																			self.fRec90[0] = if iRec85 != 0 {1.442695 * F32::log(fTemp542, std::f32::consts::E)} else {self.fRec90[1]};
																			fTemp555 = self.fRec90[0] + self.fRec89[0] * fTemp533 * (2.0 - fTemp533) + self.fRec88[0] * CallsDsp_faustpower2_f(fTemp521) * (3.0 - 2.0 * fTemp521);
																			fTemp556 = F32::floor(fTemp555);
																			iTemp557 = (fTemp556) as i32;
																			fTemp558 = fTemp555 - fTemp556;
																			fTemp559 = self.fRec87[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fSlow81 * ((fTemp558 * (fTemp558 * (fTemp558 * (fTemp558 * (0.0013333 * fTemp558 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp557))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp557))) as F32)));
																			self.fRec87[0] = fTemp559 - F32::floor(fTemp559);
																			iTemp560 = i32::wrapping_add(self.iRec84[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp514, 2)));
																			iTemp561 = i32::wrapping_mul(1664525, iTemp560 ^ (iTemp560 >> 15));
																			self.fRec91[0] = if iRec85 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp561 ^ (iTemp561 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec91[1]};
																			fTemp562 = self.fRec91[0] * F32::sin(6.2831855 * self.fRec87[0]) * self.fRec16[0] * fTemp34 * ((self.iRec82[0] > 0) as i32) as u32 as F32 * CallsDsp_faustpower2_f(fTemp520) * (3.0 - 2.0 * fTemp520) * CallsDsp_faustpower2_f(fTemp519) * (3.0 - 2.0 * fTemp519);
																			fTemp563 = fTemp562 * F32::cos(fTemp517);
																		}
																			let mut fTemp564: F32 = ((iTemp33 != 0) as i32) as u32 as F32;
																			if iTemp39 != 0 {
																				iTemp565 = (self.iRec93[1] >= self.iRec94[1]) as i32;
																				self.iRec93[0] = if iTemp565 != 0 {0} else {i32::wrapping_add(self.iRec93[1], 1)};
																				self.iRec99[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow90, iTemp11), self.iRec99[1])), 12345);
																				iTemp566 = i32::wrapping_add(self.iRec99[0], 104729);
																				iTemp567 = i32::wrapping_mul(1664525, iTemp566 ^ (iTemp566 >> 15));
																				iTemp568 = i32::wrapping_add(self.iRec97[1], 17);
																				iTemp569 = i32::wrapping_mul(1664525, iTemp568 ^ (iTemp568 >> 15));
																				iTemp570 = i32::wrapping_add(self.iRec96[1], 1);
																				iTemp571 = iTemp565 & ((iTemp570 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp569 ^ (iTemp569 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
																				iTemp572 = i32::wrapping_add(self.iRec99[0], 209458);
																				iTemp573 = i32::wrapping_mul(1664525, iTemp572 ^ (iTemp572 >> 15));
																				iTemp574 = if (iTemp571 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp567 ^ (iTemp567 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp40 != 0 {i32::wrapping_mul(1013904223, iTemp573 ^ (iTemp573 >> 13))} else {self.iRec97[1]};
																				iTemp575 = i32::wrapping_add(iTemp574, 17);
																				iTemp576 = i32::wrapping_mul(1664525, iTemp575 ^ (iTemp575 >> 15));
																				iTemp577 = if iTemp565 != 0 {if iTemp571 != 0 {0} else {iTemp570}} else {self.iRec96[1]};
																				iTemp578 = i32::wrapping_add(iTemp574, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp577), 1)));
																				iTemp579 = i32::wrapping_mul(1664525, iTemp578 ^ (iTemp578 >> 15));
																				fTemp580 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp579 ^ (iTemp579 >> 13))) as F32 + 0.5);
																				fTemp581 = F32::floor(fTemp580);
																				iTemp582 = (fTemp581) as i32;
																				fTemp583 = fTemp580 - fTemp581;
																				iTemp584 = i32::wrapping_add(self.iRec99[0], 314187);
																				iTemp585 = i32::wrapping_mul(1664525, iTemp584 ^ (iTemp584 >> 15));
																				fTemp586 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp585 ^ (iTemp585 >> 13))) as F32 + 1.0;
																				iTemp587 = i32::wrapping_add(self.iRec99[0], 418916);
																				iTemp588 = i32::wrapping_mul(1664525, iTemp587 ^ (iTemp587 >> 15));
																				fTemp589 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp588 ^ (iTemp588 >> 13))) as F32;
																				fTemp590 = fTemp589 + 1.5;
																				fTemp591 = 0.5 - fTemp589;
																				fTemp592 = CallsDsp_faustpower2_f(fTemp591) / CallsDsp_faustpower2_f(fTemp590);
																				iTemp593 = i32::wrapping_add(iTemp574, i32::wrapping_mul(190681, iTemp577));
																				iTemp594 = i32::wrapping_mul(1664525, iTemp593 ^ (iTemp593 >> 15));
																				fTemp595 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp594 ^ (iTemp594 >> 13))) as F32 + 0.5);
																				fTemp596 = F32::floor(fTemp595);
																				iTemp597 = (fTemp596) as i32;
																				fTemp598 = fTemp595 - fTemp596;
																				iTemp599 = if iTemp565 != 0 {i32::wrapping_mul(iTemp81, (fSlow98 * ((fTemp598 * (fTemp598 * (fTemp598 * (fTemp598 * (0.0013333 * fTemp598 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp597))) as F32 * fTemp586 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp597))) as F32))) as i32)} else {self.iRec95[1]};
																				self.iRec94[0] = if iTemp565 != 0 {i32::wrapping_add(iTemp599, (self.fConst0 * if (((i32::wrapping_add(iTemp577, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp576 ^ (iTemp576 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp40 != 0 {1.2e+02 * (fTemp74 * fTemp591 * (15.0 - 4.0 * fTemp592) / (fTemp72 * fTemp590 * (15.0 - 9.0 * fTemp592)))} else {fSlow97 * (fTemp586 * (fTemp583 * (fTemp583 * (fTemp583 * (fTemp583 * (0.0013333 * fTemp583 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp582))) as F32 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp582))) as F32))}) as i32)} else {self.iRec94[1]};
																				self.iRec95[0] = iTemp599;
																				self.iRec96[0] = iTemp577;
																				self.iRec97[0] = iTemp574;
																				iRec98 = iTemp565;
																				iTemp600 = i32::wrapping_mul(31, self.iRec96[0]);
																				iTemp601 = i32::wrapping_add(self.iRec97[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp600, 8)));
																				iTemp602 = i32::wrapping_mul(1664525, iTemp601 ^ (iTemp601 >> 15));
																				self.fRec92[0] = if iRec98 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp602 ^ (iTemp602 >> 13))) as F32 + 0.5} else {self.fRec92[1]};
																				fTemp603 = 1.5707964 * (fTemp86 * (self.fRec92[0] + -0.5) + 0.5);
																				fTemp604 = F32::min(1.0, F32::max(0.0, (self.iRec93[0]) as F32 / (std::cmp::max(self.iRec95[0], 1)) as F32));
																				fTemp605 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp604)));
																				fTemp606 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp604));
																				fTemp607 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp604 + -0.6)));
																				iTemp608 = i32::wrapping_add(self.iRec97[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp600, 7)));
																				iTemp609 = i32::wrapping_mul(1664525, iTemp608 ^ (iTemp608 >> 15));
																				fTemp610 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp609 ^ (iTemp609 >> 13))) as F32 + 0.5);
																				fTemp611 = F32::floor(fTemp610);
																				iTemp612 = (fTemp611) as i32;
																				fTemp613 = fTemp610 - fTemp611;
																				iTemp614 = i32::wrapping_add(self.iRec97[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp600, 6)));
																				iTemp615 = i32::wrapping_mul(1664525, iTemp614 ^ (iTemp614 >> 15));
																				iTemp616 = i32::wrapping_add(self.iRec97[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp600, 4)));
																				iTemp617 = i32::wrapping_mul(1664525, iTemp616 ^ (iTemp616 >> 15));
																				iTemp618 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp617 ^ (iTemp617 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
																				self.fRec101[0] = if iRec98 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp618, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp615 ^ (iTemp615 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp613 * (fTemp613 * (fTemp613 * (fTemp613 * (0.0013333 * fTemp613 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp612))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp612))) as F32)} else {self.fRec101[1]};
																				fTemp619 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp604 + -0.1)));
																				iTemp620 = i32::wrapping_add(self.iRec99[0], 628374);
																				iTemp621 = i32::wrapping_mul(1664525, iTemp620 ^ (iTemp620 >> 15));
																				iTemp622 = i32::wrapping_add(self.iRec97[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp600, 3)));
																				iTemp623 = i32::wrapping_mul(1664525, iTemp622 ^ (iTemp622 >> 15));
																				fTemp624 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp623 ^ (iTemp623 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp621 ^ (iTemp621 >> 13))) as F32 + 0.5));
																				fTemp625 = F32::floor(fTemp624);
																				iTemp626 = (fTemp625) as i32;
																				fTemp627 = fTemp624 - fTemp625;
																				fTemp628 = F32::max(1.0, F32::round(fSlow99 * ((fTemp627 * (fTemp627 * (fTemp627 * (fTemp627 * (0.0013333 * fTemp627 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp626))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp626))) as F32)));
																				iTemp629 = i32::wrapping_add(self.iRec97[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp600, 5)));
																				iTemp630 = i32::wrapping_mul(1664525, iTemp629 ^ (iTemp629 >> 15));
																				fTemp631 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp630 ^ (iTemp630 >> 13))) as F32 + 0.5);
																				fTemp632 = F32::floor(fTemp631);
																				iTemp633 = (fTemp632) as i32;
																				fTemp634 = fTemp631 - fTemp632;
																				fTemp635 = (iTemp618) as F32;
																				fTemp636 = 0.16666667 * (fTemp635 * (fTemp634 * (fTemp634 * (fTemp634 * (fTemp634 * (0.0013333 * fTemp634 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp633))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp633))) as F32);
																				fTemp637 = F32::floor(fTemp636);
																				iTemp638 = (fTemp637) as i32;
																				fTemp639 = fTemp636 - fTemp637;
																				fTemp640 = F32::round(fTemp628 * (fTemp639 * (fTemp639 * (fTemp639 * (fTemp639 * (0.0013333 * fTemp639 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp638))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp638))) as F32);
																				self.fRec102[0] = if iRec98 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp640 == fTemp628) as i32 != 0 {fTemp628 + fTemp635} else {fTemp640}) / fTemp628, std::f32::consts::E)} else {self.fRec102[1]};
																				self.fRec103[0] = if iRec98 != 0 {1.442695 * F32::log(fTemp628, std::f32::consts::E)} else {self.fRec103[1]};
																				fTemp641 = self.fRec103[0] + self.fRec102[0] * fTemp619 * (2.0 - fTemp619) + self.fRec101[0] * CallsDsp_faustpower2_f(fTemp607) * (3.0 - 2.0 * fTemp607);
																				fTemp642 = F32::floor(fTemp641);
																				iTemp643 = (fTemp642) as i32;
																				fTemp644 = fTemp641 - fTemp642;
																				fTemp645 = self.fRec100[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fSlow91 * ((fTemp644 * (fTemp644 * (fTemp644 * (fTemp644 * (0.0013333 * fTemp644 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp643))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp643))) as F32)));
																				self.fRec100[0] = fTemp645 - F32::floor(fTemp645);
																				iTemp646 = i32::wrapping_add(self.iRec97[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp600, 2)));
																				iTemp647 = i32::wrapping_mul(1664525, iTemp646 ^ (iTemp646 >> 15));
																				self.fRec104[0] = if iRec98 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp647 ^ (iTemp647 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec104[1]};
																				fTemp648 = self.fRec104[0] * F32::sin(6.2831855 * self.fRec100[0]) * self.fRec17[0] * fTemp38 * ((self.iRec95[0] > 0) as i32) as u32 as F32 * CallsDsp_faustpower2_f(fTemp606) * (3.0 - 2.0 * fTemp606) * CallsDsp_faustpower2_f(fTemp605) * (3.0 - 2.0 * fTemp605);
																				fTemp649 = fTemp648 * F32::cos(fTemp603);
																			}
																				let mut fTemp650: F32 = ((iTemp37 != 0) as i32) as u32 as F32;
																				if iTemp15 != 0 {
																					iTemp651 = (self.iRec106[1] >= self.iRec107[1]) as i32;
																					self.iRec106[0] = if iTemp651 != 0 {0} else {i32::wrapping_add(self.iRec106[1], 1)};
																					self.iRec112[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, i32::wrapping_add(i32::wrapping_mul(iSlow100, iTemp11), self.iRec112[1])), 12345);
																					iTemp652 = i32::wrapping_add(self.iRec112[0], 104729);
																					iTemp653 = i32::wrapping_mul(1664525, iTemp652 ^ (iTemp652 >> 15));
																					iTemp654 = i32::wrapping_add(self.iRec110[1], 17);
																					iTemp655 = i32::wrapping_mul(1664525, iTemp654 ^ (iTemp654 >> 15));
																					iTemp656 = i32::wrapping_add(self.iRec109[1], 1);
																					iTemp657 = iTemp651 & ((iTemp656 >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp655 ^ (iTemp655 >> 13))) as F32 + 0.5)) as i32, 2))) as i32);
																					iTemp658 = i32::wrapping_add(self.iRec112[0], 209458);
																					iTemp659 = i32::wrapping_mul(1664525, iTemp658 ^ (iTemp658 >> 15));
																					iTemp660 = if (iTemp657 & (((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp653 ^ (iTemp653 >> 13))) as F32 + 0.5) < 0.25) as i32)) | iTemp40 != 0 {i32::wrapping_mul(1013904223, iTemp659 ^ (iTemp659 >> 13))} else {self.iRec110[1]};
																					iTemp661 = i32::wrapping_add(iTemp660, 17);
																					iTemp662 = i32::wrapping_mul(1664525, iTemp661 ^ (iTemp661 >> 15));
																					iTemp663 = if iTemp651 != 0 {if iTemp657 != 0 {0} else {iTemp656}} else {self.iRec109[1]};
																					iTemp664 = i32::wrapping_add(iTemp660, i32::wrapping_mul(6151, i32::wrapping_add(i32::wrapping_mul(31, iTemp663), 1)));
																					iTemp665 = i32::wrapping_mul(1664525, iTemp664 ^ (iTemp664 >> 15));
																					fTemp666 = 3.321928 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp665 ^ (iTemp665 >> 13))) as F32 + 0.5);
																					fTemp667 = F32::floor(fTemp666);
																					iTemp668 = (fTemp667) as i32;
																					fTemp669 = fTemp666 - fTemp667;
																					iTemp670 = i32::wrapping_add(self.iRec112[0], 314187);
																					iTemp671 = i32::wrapping_mul(1664525, iTemp670 ^ (iTemp670 >> 15));
																					fTemp672 = 3.4924598e-11 * (i32::wrapping_mul(1013904223, iTemp671 ^ (iTemp671 >> 13))) as F32 + 1.0;
																					iTemp673 = i32::wrapping_add(self.iRec112[0], 418916);
																					iTemp674 = i32::wrapping_mul(1664525, iTemp673 ^ (iTemp673 >> 15));
																					fTemp675 = 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp674 ^ (iTemp674 >> 13))) as F32;
																					fTemp676 = fTemp675 + 1.5;
																					fTemp677 = 0.5 - fTemp675;
																					fTemp678 = CallsDsp_faustpower2_f(fTemp677) / CallsDsp_faustpower2_f(fTemp676);
																					iTemp679 = i32::wrapping_add(iTemp660, i32::wrapping_mul(190681, iTemp663));
																					iTemp680 = i32::wrapping_mul(1664525, iTemp679 ^ (iTemp679 >> 15));
																					fTemp681 = 3.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp680 ^ (iTemp680 >> 13))) as F32 + 0.5);
																					fTemp682 = F32::floor(fTemp681);
																					iTemp683 = (fTemp682) as i32;
																					fTemp684 = fTemp681 - fTemp682;
																					iTemp685 = if iTemp651 != 0 {i32::wrapping_mul(iTemp81, (fSlow108 * ((fTemp684 * (fTemp684 * (fTemp684 * (fTemp684 * (0.0013333 * fTemp684 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp683))) as F32 * fTemp672 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp683))) as F32))) as i32)} else {self.iRec108[1]};
																					self.iRec107[0] = if iTemp651 != 0 {i32::wrapping_add(iTemp685, (self.fConst0 * if (((i32::wrapping_add(iTemp663, 1)) >= (i32::wrapping_add((5.0 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp662 ^ (iTemp662 >> 13))) as F32 + 0.5)) as i32, 2))) as i32) | iTemp40 != 0 {1.2e+02 * (fTemp74 * fTemp677 * (15.0 - 4.0 * fTemp678) / (fTemp72 * fTemp676 * (15.0 - 9.0 * fTemp678)))} else {fSlow107 * (fTemp672 * (fTemp669 * (fTemp669 * (fTemp669 * (fTemp669 * (0.0013333 * fTemp669 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp668))) as F32 / (fTemp58 * (1 << std::cmp::min(30, std::cmp::max(0, -iTemp668))) as F32))}) as i32)} else {self.iRec107[1]};
																					self.iRec108[0] = iTemp685;
																					self.iRec109[0] = iTemp663;
																					self.iRec110[0] = iTemp660;
																					iRec111 = iTemp651;
																					iTemp686 = i32::wrapping_mul(31, self.iRec109[0]);
																					iTemp687 = i32::wrapping_add(self.iRec110[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp686, 8)));
																					iTemp688 = i32::wrapping_mul(1664525, iTemp687 ^ (iTemp687 >> 15));
																					self.fRec105[0] = if iRec111 != 0 {2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp688 ^ (iTemp688 >> 13))) as F32 + 0.5} else {self.fRec105[1]};
																					fTemp689 = 1.5707964 * (fTemp86 * (self.fRec105[0] + -0.5) + 0.5);
																					fTemp690 = F32::min(1.0, F32::max(0.0, (self.iRec106[0]) as F32 / (std::cmp::max(self.iRec108[0], 1)) as F32));
																					fTemp691 = F32::min(1.0, F32::max(0.0, 4.0 * (1.0 - fTemp690)));
																					fTemp692 = F32::min(1.0, F32::max(0.0, 6.6666665 * fTemp690));
																					fTemp693 = F32::min(1.0, F32::max(0.0, 4.0 * (fTemp690 + -0.6)));
																					iTemp694 = i32::wrapping_add(self.iRec110[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp686, 7)));
																					iTemp695 = i32::wrapping_mul(1664525, iTemp694 ^ (iTemp694 >> 15));
																					fTemp696 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp695 ^ (iTemp695 >> 13))) as F32 + 0.5);
																					fTemp697 = F32::floor(fTemp696);
																					iTemp698 = (fTemp697) as i32;
																					fTemp699 = fTemp696 - fTemp697;
																					iTemp700 = i32::wrapping_add(self.iRec110[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp686, 6)));
																					iTemp701 = i32::wrapping_mul(1664525, iTemp700 ^ (iTemp700 >> 15));
																					iTemp702 = i32::wrapping_add(self.iRec110[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp686, 4)));
																					iTemp703 = i32::wrapping_mul(1664525, iTemp702 ^ (iTemp702 >> 15));
																					iTemp704 = if ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp703 ^ (iTemp703 >> 13))) as F32 + 0.5) < 0.5) as i32 != 0 {-1} else {1};
																					self.fRec114[0] = if iRec111 != 0 {0.041666668 * ((-(i32::wrapping_mul(iTemp704, ((2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp701 ^ (iTemp701 >> 13))) as F32 + 0.5) < 0.35) as i32))) as F32 * (fTemp699 * (fTemp699 * (fTemp699 * (fTemp699 * (0.0013333 * fTemp699 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp698))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp698))) as F32)} else {self.fRec114[1]};
																					fTemp705 = F32::min(1.0, F32::max(0.0, 2.0 * (fTemp690 + -0.1)));
																					iTemp706 = i32::wrapping_add(self.iRec112[0], 628374);
																					iTemp707 = i32::wrapping_mul(1664525, iTemp706 ^ (iTemp706 >> 15));
																					iTemp708 = i32::wrapping_add(self.iRec110[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp686, 3)));
																					iTemp709 = i32::wrapping_mul(1664525, iTemp708 ^ (iTemp708 >> 15));
																					fTemp710 = 5.0 * F32::min(1.0, F32::max(0.0, 2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp709 ^ (iTemp709 >> 13))) as F32 + 9.3132255e-12 * (i32::wrapping_mul(1013904223, iTemp707 ^ (iTemp707 >> 13))) as F32 + 0.5));
																					fTemp711 = F32::floor(fTemp710);
																					iTemp712 = (fTemp711) as i32;
																					fTemp713 = fTemp710 - fTemp711;
																					fTemp714 = F32::max(1.0, F32::round(fSlow109 * ((fTemp713 * (fTemp713 * (fTemp713 * (fTemp713 * (0.0013333 * fTemp713 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp712))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp712))) as F32)));
																					iTemp715 = i32::wrapping_add(self.iRec110[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp686, 5)));
																					iTemp716 = i32::wrapping_mul(1664525, iTemp715 ^ (iTemp715 >> 15));
																					fTemp717 = 1.5849625 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp716 ^ (iTemp716 >> 13))) as F32 + 0.5);
																					fTemp718 = F32::floor(fTemp717);
																					iTemp719 = (fTemp718) as i32;
																					fTemp720 = fTemp717 - fTemp718;
																					fTemp721 = (iTemp704) as F32;
																					fTemp722 = 0.16666667 * (fTemp721 * (fTemp720 * (fTemp720 * (fTemp720 * (fTemp720 * (0.0013333 * fTemp720 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp719))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp719))) as F32);
																					fTemp723 = F32::floor(fTemp722);
																					iTemp724 = (fTemp723) as i32;
																					fTemp725 = fTemp722 - fTemp723;
																					fTemp726 = F32::round(fTemp714 * (fTemp725 * (fTemp725 * (fTemp725 * (fTemp725 * (0.0013333 * fTemp725 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp724))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp724))) as F32);
																					self.fRec115[0] = if iRec111 != 0 {1.442695 * F32::log(F32::max(1.0, if (fTemp726 == fTemp714) as i32 != 0 {fTemp714 + fTemp721} else {fTemp726}) / fTemp714, std::f32::consts::E)} else {self.fRec115[1]};
																					self.fRec116[0] = if iRec111 != 0 {1.442695 * F32::log(fTemp714, std::f32::consts::E)} else {self.fRec116[1]};
																					fTemp727 = self.fRec116[0] + self.fRec115[0] * fTemp705 * (2.0 - fTemp705) + self.fRec114[0] * CallsDsp_faustpower2_f(fTemp693) * (3.0 - 2.0 * fTemp693);
																					fTemp728 = F32::floor(fTemp727);
																					iTemp729 = (fTemp728) as i32;
																					fTemp730 = fTemp727 - fTemp728;
																					fTemp731 = self.fRec113[1] + self.fConst7 * F32::min(self.fConst6, F32::max(2e+01, fSlow101 * ((fTemp730 * (fTemp730 * (fTemp730 * (fTemp730 * (0.0013333 * fTemp730 + 0.0096181) + 0.0555041) + 0.240227) + 0.693147) + 1.0) * (1 << std::cmp::min(30, std::cmp::max(0, iTemp729))) as F32 / (1 << std::cmp::min(30, std::cmp::max(0, -iTemp729))) as F32)));
																					self.fRec113[0] = fTemp731 - F32::floor(fTemp731);
																					iTemp732 = i32::wrapping_add(self.iRec110[0], i32::wrapping_mul(6151, i32::wrapping_add(iTemp686, 2)));
																					iTemp733 = i32::wrapping_mul(1664525, iTemp732 ^ (iTemp732 >> 15));
																					self.fRec117[0] = if iRec111 != 0 {0.4 * (2.3283064e-10 * (i32::wrapping_mul(1013904223, iTemp733 ^ (iTemp733 >> 13))) as F32 + 0.5) + 0.6} else {self.fRec117[1]};
																					fTemp734 = self.fRec117[0] * F32::sin(6.2831855 * self.fRec113[0]) * self.fRec11[0] * fTemp14 * ((self.iRec108[0] > 0) as i32) as u32 as F32 * CallsDsp_faustpower2_f(fTemp692) * (3.0 - 2.0 * fTemp692) * CallsDsp_faustpower2_f(fTemp691) * (3.0 - 2.0 * fTemp691);
																					fTemp735 = fTemp734 * F32::cos(fTemp689);
																				}
																					let mut fTemp736: F32 = ((iTemp13 != 0) as i32) as u32 as F32;
																					*output0 = (fTemp736 * fTemp735 + fTemp650 * fTemp649 + fTemp564 * fTemp563 + fTemp478 * fTemp477 + fTemp392 * fTemp391 + fTemp306 * fTemp305 + fTemp220 * fTemp219 + fTemp134 * fTemp133) * fTemp5 * fTemp4;
																					if iTemp9 != 0 {
																						fTemp737 = fTemp132 * F32::sin(fTemp87);
																					}
																						if iTemp19 != 0 {
																							fTemp738 = fTemp218 * F32::sin(fTemp173);
																						}
																							if iTemp23 != 0 {
																								fTemp739 = fTemp304 * F32::sin(fTemp259);
																							}
																								if iTemp27 != 0 {
																									fTemp740 = fTemp390 * F32::sin(fTemp345);
																								}
																									if iTemp31 != 0 {
																										fTemp741 = fTemp476 * F32::sin(fTemp431);
																									}
																										if iTemp35 != 0 {
																											fTemp742 = fTemp562 * F32::sin(fTemp517);
																										}
																											if iTemp39 != 0 {
																												fTemp743 = fTemp648 * F32::sin(fTemp603);
																											}
																												if iTemp15 != 0 {
																													fTemp744 = fTemp734 * F32::sin(fTemp689);
																												}
																													*output1 = fTemp5 * fTemp4 * (fTemp736 * fTemp744 + fTemp650 * fTemp743 + fTemp564 * fTemp742 + fTemp478 * fTemp741 + fTemp392 * fTemp740 + fTemp306 * fTemp739 + fTemp220 * fTemp738 + fTemp134 * fTemp737);
																													self.iVec0[1] = self.iVec0[0];
																													self.fRec0[1] = self.fRec0[0];
																													self.fRec1[1] = self.fRec1[0];
																													self.fRec2[1] = self.fRec2[0];
																													self.fRec3[1] = self.fRec3[0];
																													if iTemp9 != 0 {
																														self.iRec5[1] = self.iRec5[0];
																													}
																														self.fRec11[1] = self.fRec11[0];
																														self.fRec12[1] = self.fRec12[0];
																														self.fRec13[1] = self.fRec13[0];
																														self.fRec14[1] = self.fRec14[0];
																														self.fRec15[1] = self.fRec15[0];
																														self.fRec16[1] = self.fRec16[0];
																														self.fRec17[1] = self.fRec17[0];
																														if iTemp9 != 0 {
																															self.iRec18[1] = self.iRec18[0];
																														}
																															if iTemp39 | (iTemp35 | (iTemp31 | (iTemp27 | (iTemp23 | (iTemp19 | (iTemp9 | iTemp15)))))) != 0 {
																																self.fRec19[1] = self.fRec19[0];
																																self.fRec20[1] = self.fRec20[0];
																															}
																																if iTemp9 != 0 {
																																	self.iRec6[1] = self.iRec6[0];
																																	self.iRec7[1] = self.iRec7[0];
																																	self.iRec8[1] = self.iRec8[0];
																																	self.iRec9[1] = self.iRec9[0];
																																	self.fRec4[1] = self.fRec4[0];
																																}
																																	if iTemp39 | (iTemp35 | (iTemp31 | (iTemp27 | (iTemp23 | (iTemp19 | (iTemp9 | iTemp15)))))) != 0 {
																																		self.fRec21[1] = self.fRec21[0];
																																	}
																																		if iTemp9 != 0 {
																																			self.fRec23[1] = self.fRec23[0];
																																			self.fRec24[1] = self.fRec24[0];
																																			self.fRec25[1] = self.fRec25[0];
																																			self.fRec22[1] = self.fRec22[0];
																																			self.fRec26[1] = self.fRec26[0];
																																		}
																																			if iTemp19 != 0 {
																																				self.iRec28[1] = self.iRec28[0];
																																				self.iRec34[1] = self.iRec34[0];
																																				self.iRec29[1] = self.iRec29[0];
																																				self.iRec30[1] = self.iRec30[0];
																																				self.iRec31[1] = self.iRec31[0];
																																				self.iRec32[1] = self.iRec32[0];
																																				self.fRec27[1] = self.fRec27[0];
																																				self.fRec36[1] = self.fRec36[0];
																																				self.fRec37[1] = self.fRec37[0];
																																				self.fRec38[1] = self.fRec38[0];
																																				self.fRec35[1] = self.fRec35[0];
																																				self.fRec39[1] = self.fRec39[0];
																																			}
																																				if iTemp23 != 0 {
																																					self.iRec41[1] = self.iRec41[0];
																																					self.iRec47[1] = self.iRec47[0];
																																					self.iRec42[1] = self.iRec42[0];
																																					self.iRec43[1] = self.iRec43[0];
																																					self.iRec44[1] = self.iRec44[0];
																																					self.iRec45[1] = self.iRec45[0];
																																					self.fRec40[1] = self.fRec40[0];
																																					self.fRec49[1] = self.fRec49[0];
																																					self.fRec50[1] = self.fRec50[0];
																																					self.fRec51[1] = self.fRec51[0];
																																					self.fRec48[1] = self.fRec48[0];
																																					self.fRec52[1] = self.fRec52[0];
																																				}
																																					if iTemp27 != 0 {
																																						self.iRec54[1] = self.iRec54[0];
																																						self.iRec60[1] = self.iRec60[0];
																																						self.iRec55[1] = self.iRec55[0];
																																						self.iRec56[1] = self.iRec56[0];
																																						self.iRec57[1] = self.iRec57[0];
																																						self.iRec58[1] = self.iRec58[0];
																																						self.fRec53[1] = self.fRec53[0];
																																						self.fRec62[1] = self.fRec62[0];
																																						self.fRec63[1] = self.fRec63[0];
																																						self.fRec64[1] = self.fRec64[0];
																																						self.fRec61[1] = self.fRec61[0];
																																						self.fRec65[1] = self.fRec65[0];
																																					}
																																						if iTemp31 != 0 {
																																							self.iRec67[1] = self.iRec67[0];
																																							self.iRec73[1] = self.iRec73[0];
																																							self.iRec68[1] = self.iRec68[0];
																																							self.iRec69[1] = self.iRec69[0];
																																							self.iRec70[1] = self.iRec70[0];
																																							self.iRec71[1] = self.iRec71[0];
																																							self.fRec66[1] = self.fRec66[0];
																																							self.fRec75[1] = self.fRec75[0];
																																							self.fRec76[1] = self.fRec76[0];
																																							self.fRec77[1] = self.fRec77[0];
																																							self.fRec74[1] = self.fRec74[0];
																																							self.fRec78[1] = self.fRec78[0];
																																						}
																																							if iTemp35 != 0 {
																																								self.iRec80[1] = self.iRec80[0];
																																								self.iRec86[1] = self.iRec86[0];
																																								self.iRec81[1] = self.iRec81[0];
																																								self.iRec82[1] = self.iRec82[0];
																																								self.iRec83[1] = self.iRec83[0];
																																								self.iRec84[1] = self.iRec84[0];
																																								self.fRec79[1] = self.fRec79[0];
																																								self.fRec88[1] = self.fRec88[0];
																																								self.fRec89[1] = self.fRec89[0];
																																								self.fRec90[1] = self.fRec90[0];
																																								self.fRec87[1] = self.fRec87[0];
																																								self.fRec91[1] = self.fRec91[0];
																																							}
																																								if iTemp39 != 0 {
																																									self.iRec93[1] = self.iRec93[0];
																																									self.iRec99[1] = self.iRec99[0];
																																									self.iRec94[1] = self.iRec94[0];
																																									self.iRec95[1] = self.iRec95[0];
																																									self.iRec96[1] = self.iRec96[0];
																																									self.iRec97[1] = self.iRec97[0];
																																									self.fRec92[1] = self.fRec92[0];
																																									self.fRec101[1] = self.fRec101[0];
																																									self.fRec102[1] = self.fRec102[0];
																																									self.fRec103[1] = self.fRec103[0];
																																									self.fRec100[1] = self.fRec100[0];
																																									self.fRec104[1] = self.fRec104[0];
																																								}
																																									if iTemp15 != 0 {
																																										self.iRec106[1] = self.iRec106[0];
																																										self.iRec112[1] = self.iRec112[0];
																																										self.iRec107[1] = self.iRec107[0];
																																										self.iRec108[1] = self.iRec108[0];
																																										self.iRec109[1] = self.iRec109[0];
																																										self.iRec110[1] = self.iRec110[0];
																																										self.fRec105[1] = self.fRec105[0];
																																										self.fRec114[1] = self.fRec114[0];
																																										self.fRec115[1] = self.fRec115[0];
																																										self.fRec116[1] = self.fRec116[0];
																																										self.fRec113[1] = self.fRec113[0];
																																										self.fRec117[1] = self.fRec117[0];
																																									}
																																									}
																																									self.iTempPerm0 = iTemp10;
																																									self.iTempPerm1 = iTemp11;
																																									self.iTempPerm2 = iTemp40;
																																									self.iTempPerm3 = iTemp41;
																																									self.iTempPerm4 = iTemp42;
																																									self.iTempPerm5 = iTemp43;
																																									self.iTempPerm6 = iTemp44;
																																									self.iTempPerm7 = iTemp45;
																																									self.iTempPerm8 = iTemp46;
																																									self.iTempPerm9 = iTemp47;
																																									self.iTempPerm10 = iTemp48;
																																									self.iTempPerm11 = iTemp49;
																																									self.iTempPerm12 = iTemp50;
																																									self.iTempPerm13 = iTemp51;
																																									self.iTempPerm14 = iTemp52;
																																									self.iTempPerm15 = iTemp53;
																																									self.iTempPerm16 = iTemp54;
																																									self.fTempPerm17 = fTemp55;
																																									self.fTempPerm18 = fTemp56;
																																									self.iTempPerm19 = iTemp57;
																																									self.fTempPerm20 = fTemp58;
																																									self.fTempPerm21 = fTemp59;
																																									self.iTempPerm22 = iTemp60;
																																									self.iTempPerm23 = iTemp61;
																																									self.fTempPerm24 = fTemp62;
																																									self.iTempPerm25 = iTemp63;
																																									self.iTempPerm26 = iTemp64;
																																									self.fTempPerm27 = fTemp65;
																																									self.fTempPerm28 = fTemp66;
																																									self.fTempPerm29 = fTemp67;
																																									self.fTempPerm30 = fTemp68;
																																									self.fTempPerm31 = fTemp69;
																																									self.fTempPerm32 = fTemp70;
																																									self.iTempPerm33 = iTemp71;
																																									self.fTempPerm34 = fTemp72;
																																									self.fTempPerm35 = fTemp73;
																																									self.fTempPerm36 = fTemp74;
																																									self.iTempPerm37 = iTemp75;
																																									self.iTempPerm38 = iTemp76;
																																									self.fTempPerm39 = fTemp77;
																																									self.fTempPerm40 = fTemp78;
																																									self.iTempPerm41 = iTemp79;
																																									self.fTempPerm42 = fTemp80;
																																									self.iTempPerm43 = iTemp81;
																																									self.iTempPerm44 = iTemp82;
																																									self.iTempPerm45 = iTemp83;
																																									self.iTempPerm46 = iTemp84;
																																									self.iTempPerm47 = iTemp85;
																																									self.fTempPerm48 = fTemp86;
																																									self.fTempPerm49 = fTemp87;
																																									self.fTempPerm50 = fTemp88;
																																									self.fTempPerm51 = fTemp89;
																																									self.fTempPerm52 = fTemp90;
																																									self.fTempPerm53 = fTemp91;
																																									self.iTempPerm54 = iTemp92;
																																									self.iTempPerm55 = iTemp93;
																																									self.fTempPerm56 = fTemp94;
																																									self.fTempPerm57 = fTemp95;
																																									self.iTempPerm58 = iTemp96;
																																									self.fTempPerm59 = fTemp97;
																																									self.iTempPerm60 = iTemp98;
																																									self.iTempPerm61 = iTemp99;
																																									self.iTempPerm62 = iTemp100;
																																									self.iTempPerm63 = iTemp101;
																																									self.iTempPerm64 = iTemp102;
																																									self.fTempPerm65 = fTemp103;
																																									self.iTempPerm66 = iTemp104;
																																									self.iTempPerm67 = iTemp105;
																																									self.iTempPerm68 = iTemp106;
																																									self.iTempPerm69 = iTemp107;
																																									self.fTempPerm70 = fTemp108;
																																									self.fTempPerm71 = fTemp109;
																																									self.iTempPerm72 = iTemp110;
																																									self.fTempPerm73 = fTemp111;
																																									self.fTempPerm74 = fTemp112;
																																									self.iTempPerm75 = iTemp113;
																																									self.iTempPerm76 = iTemp114;
																																									self.fTempPerm77 = fTemp115;
																																									self.fTempPerm78 = fTemp116;
																																									self.iTempPerm79 = iTemp117;
																																									self.fTempPerm80 = fTemp118;
																																									self.fTempPerm81 = fTemp119;
																																									self.fTempPerm82 = fTemp120;
																																									self.fTempPerm83 = fTemp121;
																																									self.iTempPerm84 = iTemp122;
																																									self.fTempPerm85 = fTemp123;
																																									self.fTempPerm86 = fTemp124;
																																									self.fTempPerm87 = fTemp125;
																																									self.fTempPerm88 = fTemp126;
																																									self.iTempPerm89 = iTemp127;
																																									self.fTempPerm90 = fTemp128;
																																									self.fTempPerm91 = fTemp129;
																																									self.iTempPerm92 = iTemp130;
																																									self.iTempPerm93 = iTemp131;
																																									self.fTempPerm94 = fTemp132;
																																									self.fTempPerm95 = fTemp133;
																																									self.iTempPerm96 = iTemp135;
																																									self.iTempPerm97 = iTemp136;
																																									self.iTempPerm98 = iTemp137;
																																									self.iTempPerm99 = iTemp138;
																																									self.iTempPerm100 = iTemp139;
																																									self.iTempPerm101 = iTemp140;
																																									self.iTempPerm102 = iTemp141;
																																									self.iTempPerm103 = iTemp142;
																																									self.iTempPerm104 = iTemp143;
																																									self.iTempPerm105 = iTemp144;
																																									self.iTempPerm106 = iTemp145;
																																									self.iTempPerm107 = iTemp146;
																																									self.iTempPerm108 = iTemp147;
																																									self.iTempPerm109 = iTemp148;
																																									self.iTempPerm110 = iTemp149;
																																									self.fTempPerm111 = fTemp150;
																																									self.fTempPerm112 = fTemp151;
																																									self.iTempPerm113 = iTemp152;
																																									self.fTempPerm114 = fTemp153;
																																									self.iTempPerm115 = iTemp154;
																																									self.iTempPerm116 = iTemp155;
																																									self.fTempPerm117 = fTemp156;
																																									self.iTempPerm118 = iTemp157;
																																									self.iTempPerm119 = iTemp158;
																																									self.fTempPerm120 = fTemp159;
																																									self.fTempPerm121 = fTemp160;
																																									self.fTempPerm122 = fTemp161;
																																									self.fTempPerm123 = fTemp162;
																																									self.iTempPerm124 = iTemp163;
																																									self.iTempPerm125 = iTemp164;
																																									self.fTempPerm126 = fTemp165;
																																									self.fTempPerm127 = fTemp166;
																																									self.iTempPerm128 = iTemp167;
																																									self.fTempPerm129 = fTemp168;
																																									self.iTempPerm130 = iTemp169;
																																									self.iTempPerm131 = iTemp170;
																																									self.iTempPerm132 = iTemp171;
																																									self.iTempPerm133 = iTemp172;
																																									self.fTempPerm134 = fTemp173;
																																									self.fTempPerm135 = fTemp174;
																																									self.fTempPerm136 = fTemp175;
																																									self.fTempPerm137 = fTemp176;
																																									self.fTempPerm138 = fTemp177;
																																									self.iTempPerm139 = iTemp178;
																																									self.iTempPerm140 = iTemp179;
																																									self.fTempPerm141 = fTemp180;
																																									self.fTempPerm142 = fTemp181;
																																									self.iTempPerm143 = iTemp182;
																																									self.fTempPerm144 = fTemp183;
																																									self.iTempPerm145 = iTemp184;
																																									self.iTempPerm146 = iTemp185;
																																									self.iTempPerm147 = iTemp186;
																																									self.iTempPerm148 = iTemp187;
																																									self.iTempPerm149 = iTemp188;
																																									self.fTempPerm150 = fTemp189;
																																									self.iTempPerm151 = iTemp190;
																																									self.iTempPerm152 = iTemp191;
																																									self.iTempPerm153 = iTemp192;
																																									self.iTempPerm154 = iTemp193;
																																									self.fTempPerm155 = fTemp194;
																																									self.fTempPerm156 = fTemp195;
																																									self.iTempPerm157 = iTemp196;
																																									self.fTempPerm158 = fTemp197;
																																									self.fTempPerm159 = fTemp198;
																																									self.iTempPerm160 = iTemp199;
																																									self.iTempPerm161 = iTemp200;
																																									self.fTempPerm162 = fTemp201;
																																									self.fTempPerm163 = fTemp202;
																																									self.iTempPerm164 = iTemp203;
																																									self.fTempPerm165 = fTemp204;
																																									self.fTempPerm166 = fTemp205;
																																									self.fTempPerm167 = fTemp206;
																																									self.fTempPerm168 = fTemp207;
																																									self.iTempPerm169 = iTemp208;
																																									self.fTempPerm170 = fTemp209;
																																									self.fTempPerm171 = fTemp210;
																																									self.fTempPerm172 = fTemp211;
																																									self.fTempPerm173 = fTemp212;
																																									self.iTempPerm174 = iTemp213;
																																									self.fTempPerm175 = fTemp214;
																																									self.fTempPerm176 = fTemp215;
																																									self.iTempPerm177 = iTemp216;
																																									self.iTempPerm178 = iTemp217;
																																									self.fTempPerm179 = fTemp218;
																																									self.fTempPerm180 = fTemp219;
																																									self.iTempPerm181 = iTemp221;
																																									self.iTempPerm182 = iTemp222;
																																									self.iTempPerm183 = iTemp223;
																																									self.iTempPerm184 = iTemp224;
																																									self.iTempPerm185 = iTemp225;
																																									self.iTempPerm186 = iTemp226;
																																									self.iTempPerm187 = iTemp227;
																																									self.iTempPerm188 = iTemp228;
																																									self.iTempPerm189 = iTemp229;
																																									self.iTempPerm190 = iTemp230;
																																									self.iTempPerm191 = iTemp231;
																																									self.iTempPerm192 = iTemp232;
																																									self.iTempPerm193 = iTemp233;
																																									self.iTempPerm194 = iTemp234;
																																									self.iTempPerm195 = iTemp235;
																																									self.fTempPerm196 = fTemp236;
																																									self.fTempPerm197 = fTemp237;
																																									self.iTempPerm198 = iTemp238;
																																									self.fTempPerm199 = fTemp239;
																																									self.iTempPerm200 = iTemp240;
																																									self.iTempPerm201 = iTemp241;
																																									self.fTempPerm202 = fTemp242;
																																									self.iTempPerm203 = iTemp243;
																																									self.iTempPerm204 = iTemp244;
																																									self.fTempPerm205 = fTemp245;
																																									self.fTempPerm206 = fTemp246;
																																									self.fTempPerm207 = fTemp247;
																																									self.fTempPerm208 = fTemp248;
																																									self.iTempPerm209 = iTemp249;
																																									self.iTempPerm210 = iTemp250;
																																									self.fTempPerm211 = fTemp251;
																																									self.fTempPerm212 = fTemp252;
																																									self.iTempPerm213 = iTemp253;
																																									self.fTempPerm214 = fTemp254;
																																									self.iTempPerm215 = iTemp255;
																																									self.iTempPerm216 = iTemp256;
																																									self.iTempPerm217 = iTemp257;
																																									self.iTempPerm218 = iTemp258;
																																									self.fTempPerm219 = fTemp259;
																																									self.fTempPerm220 = fTemp260;
																																									self.fTempPerm221 = fTemp261;
																																									self.fTempPerm222 = fTemp262;
																																									self.fTempPerm223 = fTemp263;
																																									self.iTempPerm224 = iTemp264;
																																									self.iTempPerm225 = iTemp265;
																																									self.fTempPerm226 = fTemp266;
																																									self.fTempPerm227 = fTemp267;
																																									self.iTempPerm228 = iTemp268;
																																									self.fTempPerm229 = fTemp269;
																																									self.iTempPerm230 = iTemp270;
																																									self.iTempPerm231 = iTemp271;
																																									self.iTempPerm232 = iTemp272;
																																									self.iTempPerm233 = iTemp273;
																																									self.iTempPerm234 = iTemp274;
																																									self.fTempPerm235 = fTemp275;
																																									self.iTempPerm236 = iTemp276;
																																									self.iTempPerm237 = iTemp277;
																																									self.iTempPerm238 = iTemp278;
																																									self.iTempPerm239 = iTemp279;
																																									self.fTempPerm240 = fTemp280;
																																									self.fTempPerm241 = fTemp281;
																																									self.iTempPerm242 = iTemp282;
																																									self.fTempPerm243 = fTemp283;
																																									self.fTempPerm244 = fTemp284;
																																									self.iTempPerm245 = iTemp285;
																																									self.iTempPerm246 = iTemp286;
																																									self.fTempPerm247 = fTemp287;
																																									self.fTempPerm248 = fTemp288;
																																									self.iTempPerm249 = iTemp289;
																																									self.fTempPerm250 = fTemp290;
																																									self.fTempPerm251 = fTemp291;
																																									self.fTempPerm252 = fTemp292;
																																									self.fTempPerm253 = fTemp293;
																																									self.iTempPerm254 = iTemp294;
																																									self.fTempPerm255 = fTemp295;
																																									self.fTempPerm256 = fTemp296;
																																									self.fTempPerm257 = fTemp297;
																																									self.fTempPerm258 = fTemp298;
																																									self.iTempPerm259 = iTemp299;
																																									self.fTempPerm260 = fTemp300;
																																									self.fTempPerm261 = fTemp301;
																																									self.iTempPerm262 = iTemp302;
																																									self.iTempPerm263 = iTemp303;
																																									self.fTempPerm264 = fTemp304;
																																									self.fTempPerm265 = fTemp305;
																																									self.iTempPerm266 = iTemp307;
																																									self.iTempPerm267 = iTemp308;
																																									self.iTempPerm268 = iTemp309;
																																									self.iTempPerm269 = iTemp310;
																																									self.iTempPerm270 = iTemp311;
																																									self.iTempPerm271 = iTemp312;
																																									self.iTempPerm272 = iTemp313;
																																									self.iTempPerm273 = iTemp314;
																																									self.iTempPerm274 = iTemp315;
																																									self.iTempPerm275 = iTemp316;
																																									self.iTempPerm276 = iTemp317;
																																									self.iTempPerm277 = iTemp318;
																																									self.iTempPerm278 = iTemp319;
																																									self.iTempPerm279 = iTemp320;
																																									self.iTempPerm280 = iTemp321;
																																									self.fTempPerm281 = fTemp322;
																																									self.fTempPerm282 = fTemp323;
																																									self.iTempPerm283 = iTemp324;
																																									self.fTempPerm284 = fTemp325;
																																									self.iTempPerm285 = iTemp326;
																																									self.iTempPerm286 = iTemp327;
																																									self.fTempPerm287 = fTemp328;
																																									self.iTempPerm288 = iTemp329;
																																									self.iTempPerm289 = iTemp330;
																																									self.fTempPerm290 = fTemp331;
																																									self.fTempPerm291 = fTemp332;
																																									self.fTempPerm292 = fTemp333;
																																									self.fTempPerm293 = fTemp334;
																																									self.iTempPerm294 = iTemp335;
																																									self.iTempPerm295 = iTemp336;
																																									self.fTempPerm296 = fTemp337;
																																									self.fTempPerm297 = fTemp338;
																																									self.iTempPerm298 = iTemp339;
																																									self.fTempPerm299 = fTemp340;
																																									self.iTempPerm300 = iTemp341;
																																									self.iTempPerm301 = iTemp342;
																																									self.iTempPerm302 = iTemp343;
																																									self.iTempPerm303 = iTemp344;
																																									self.fTempPerm304 = fTemp345;
																																									self.fTempPerm305 = fTemp346;
																																									self.fTempPerm306 = fTemp347;
																																									self.fTempPerm307 = fTemp348;
																																									self.fTempPerm308 = fTemp349;
																																									self.iTempPerm309 = iTemp350;
																																									self.iTempPerm310 = iTemp351;
																																									self.fTempPerm311 = fTemp352;
																																									self.fTempPerm312 = fTemp353;
																																									self.iTempPerm313 = iTemp354;
																																									self.fTempPerm314 = fTemp355;
																																									self.iTempPerm315 = iTemp356;
																																									self.iTempPerm316 = iTemp357;
																																									self.iTempPerm317 = iTemp358;
																																									self.iTempPerm318 = iTemp359;
																																									self.iTempPerm319 = iTemp360;
																																									self.fTempPerm320 = fTemp361;
																																									self.iTempPerm321 = iTemp362;
																																									self.iTempPerm322 = iTemp363;
																																									self.iTempPerm323 = iTemp364;
																																									self.iTempPerm324 = iTemp365;
																																									self.fTempPerm325 = fTemp366;
																																									self.fTempPerm326 = fTemp367;
																																									self.iTempPerm327 = iTemp368;
																																									self.fTempPerm328 = fTemp369;
																																									self.fTempPerm329 = fTemp370;
																																									self.iTempPerm330 = iTemp371;
																																									self.iTempPerm331 = iTemp372;
																																									self.fTempPerm332 = fTemp373;
																																									self.fTempPerm333 = fTemp374;
																																									self.iTempPerm334 = iTemp375;
																																									self.fTempPerm335 = fTemp376;
																																									self.fTempPerm336 = fTemp377;
																																									self.fTempPerm337 = fTemp378;
																																									self.fTempPerm338 = fTemp379;
																																									self.iTempPerm339 = iTemp380;
																																									self.fTempPerm340 = fTemp381;
																																									self.fTempPerm341 = fTemp382;
																																									self.fTempPerm342 = fTemp383;
																																									self.fTempPerm343 = fTemp384;
																																									self.iTempPerm344 = iTemp385;
																																									self.fTempPerm345 = fTemp386;
																																									self.fTempPerm346 = fTemp387;
																																									self.iTempPerm347 = iTemp388;
																																									self.iTempPerm348 = iTemp389;
																																									self.fTempPerm349 = fTemp390;
																																									self.fTempPerm350 = fTemp391;
																																									self.iTempPerm351 = iTemp393;
																																									self.iTempPerm352 = iTemp394;
																																									self.iTempPerm353 = iTemp395;
																																									self.iTempPerm354 = iTemp396;
																																									self.iTempPerm355 = iTemp397;
																																									self.iTempPerm356 = iTemp398;
																																									self.iTempPerm357 = iTemp399;
																																									self.iTempPerm358 = iTemp400;
																																									self.iTempPerm359 = iTemp401;
																																									self.iTempPerm360 = iTemp402;
																																									self.iTempPerm361 = iTemp403;
																																									self.iTempPerm362 = iTemp404;
																																									self.iTempPerm363 = iTemp405;
																																									self.iTempPerm364 = iTemp406;
																																									self.iTempPerm365 = iTemp407;
																																									self.fTempPerm366 = fTemp408;
																																									self.fTempPerm367 = fTemp409;
																																									self.iTempPerm368 = iTemp410;
																																									self.fTempPerm369 = fTemp411;
																																									self.iTempPerm370 = iTemp412;
																																									self.iTempPerm371 = iTemp413;
																																									self.fTempPerm372 = fTemp414;
																																									self.iTempPerm373 = iTemp415;
																																									self.iTempPerm374 = iTemp416;
																																									self.fTempPerm375 = fTemp417;
																																									self.fTempPerm376 = fTemp418;
																																									self.fTempPerm377 = fTemp419;
																																									self.fTempPerm378 = fTemp420;
																																									self.iTempPerm379 = iTemp421;
																																									self.iTempPerm380 = iTemp422;
																																									self.fTempPerm381 = fTemp423;
																																									self.fTempPerm382 = fTemp424;
																																									self.iTempPerm383 = iTemp425;
																																									self.fTempPerm384 = fTemp426;
																																									self.iTempPerm385 = iTemp427;
																																									self.iTempPerm386 = iTemp428;
																																									self.iTempPerm387 = iTemp429;
																																									self.iTempPerm388 = iTemp430;
																																									self.fTempPerm389 = fTemp431;
																																									self.fTempPerm390 = fTemp432;
																																									self.fTempPerm391 = fTemp433;
																																									self.fTempPerm392 = fTemp434;
																																									self.fTempPerm393 = fTemp435;
																																									self.iTempPerm394 = iTemp436;
																																									self.iTempPerm395 = iTemp437;
																																									self.fTempPerm396 = fTemp438;
																																									self.fTempPerm397 = fTemp439;
																																									self.iTempPerm398 = iTemp440;
																																									self.fTempPerm399 = fTemp441;
																																									self.iTempPerm400 = iTemp442;
																																									self.iTempPerm401 = iTemp443;
																																									self.iTempPerm402 = iTemp444;
																																									self.iTempPerm403 = iTemp445;
																																									self.iTempPerm404 = iTemp446;
																																									self.fTempPerm405 = fTemp447;
																																									self.iTempPerm406 = iTemp448;
																																									self.iTempPerm407 = iTemp449;
																																									self.iTempPerm408 = iTemp450;
																																									self.iTempPerm409 = iTemp451;
																																									self.fTempPerm410 = fTemp452;
																																									self.fTempPerm411 = fTemp453;
																																									self.iTempPerm412 = iTemp454;
																																									self.fTempPerm413 = fTemp455;
																																									self.fTempPerm414 = fTemp456;
																																									self.iTempPerm415 = iTemp457;
																																									self.iTempPerm416 = iTemp458;
																																									self.fTempPerm417 = fTemp459;
																																									self.fTempPerm418 = fTemp460;
																																									self.iTempPerm419 = iTemp461;
																																									self.fTempPerm420 = fTemp462;
																																									self.fTempPerm421 = fTemp463;
																																									self.fTempPerm422 = fTemp464;
																																									self.fTempPerm423 = fTemp465;
																																									self.iTempPerm424 = iTemp466;
																																									self.fTempPerm425 = fTemp467;
																																									self.fTempPerm426 = fTemp468;
																																									self.fTempPerm427 = fTemp469;
																																									self.fTempPerm428 = fTemp470;
																																									self.iTempPerm429 = iTemp471;
																																									self.fTempPerm430 = fTemp472;
																																									self.fTempPerm431 = fTemp473;
																																									self.iTempPerm432 = iTemp474;
																																									self.iTempPerm433 = iTemp475;
																																									self.fTempPerm434 = fTemp476;
																																									self.fTempPerm435 = fTemp477;
																																									self.iTempPerm436 = iTemp479;
																																									self.iTempPerm437 = iTemp480;
																																									self.iTempPerm438 = iTemp481;
																																									self.iTempPerm439 = iTemp482;
																																									self.iTempPerm440 = iTemp483;
																																									self.iTempPerm441 = iTemp484;
																																									self.iTempPerm442 = iTemp485;
																																									self.iTempPerm443 = iTemp486;
																																									self.iTempPerm444 = iTemp487;
																																									self.iTempPerm445 = iTemp488;
																																									self.iTempPerm446 = iTemp489;
																																									self.iTempPerm447 = iTemp490;
																																									self.iTempPerm448 = iTemp491;
																																									self.iTempPerm449 = iTemp492;
																																									self.iTempPerm450 = iTemp493;
																																									self.fTempPerm451 = fTemp494;
																																									self.fTempPerm452 = fTemp495;
																																									self.iTempPerm453 = iTemp496;
																																									self.fTempPerm454 = fTemp497;
																																									self.iTempPerm455 = iTemp498;
																																									self.iTempPerm456 = iTemp499;
																																									self.fTempPerm457 = fTemp500;
																																									self.iTempPerm458 = iTemp501;
																																									self.iTempPerm459 = iTemp502;
																																									self.fTempPerm460 = fTemp503;
																																									self.fTempPerm461 = fTemp504;
																																									self.fTempPerm462 = fTemp505;
																																									self.fTempPerm463 = fTemp506;
																																									self.iTempPerm464 = iTemp507;
																																									self.iTempPerm465 = iTemp508;
																																									self.fTempPerm466 = fTemp509;
																																									self.fTempPerm467 = fTemp510;
																																									self.iTempPerm468 = iTemp511;
																																									self.fTempPerm469 = fTemp512;
																																									self.iTempPerm470 = iTemp513;
																																									self.iTempPerm471 = iTemp514;
																																									self.iTempPerm472 = iTemp515;
																																									self.iTempPerm473 = iTemp516;
																																									self.fTempPerm474 = fTemp517;
																																									self.fTempPerm475 = fTemp518;
																																									self.fTempPerm476 = fTemp519;
																																									self.fTempPerm477 = fTemp520;
																																									self.fTempPerm478 = fTemp521;
																																									self.iTempPerm479 = iTemp522;
																																									self.iTempPerm480 = iTemp523;
																																									self.fTempPerm481 = fTemp524;
																																									self.fTempPerm482 = fTemp525;
																																									self.iTempPerm483 = iTemp526;
																																									self.fTempPerm484 = fTemp527;
																																									self.iTempPerm485 = iTemp528;
																																									self.iTempPerm486 = iTemp529;
																																									self.iTempPerm487 = iTemp530;
																																									self.iTempPerm488 = iTemp531;
																																									self.iTempPerm489 = iTemp532;
																																									self.fTempPerm490 = fTemp533;
																																									self.iTempPerm491 = iTemp534;
																																									self.iTempPerm492 = iTemp535;
																																									self.iTempPerm493 = iTemp536;
																																									self.iTempPerm494 = iTemp537;
																																									self.fTempPerm495 = fTemp538;
																																									self.fTempPerm496 = fTemp539;
																																									self.iTempPerm497 = iTemp540;
																																									self.fTempPerm498 = fTemp541;
																																									self.fTempPerm499 = fTemp542;
																																									self.iTempPerm500 = iTemp543;
																																									self.iTempPerm501 = iTemp544;
																																									self.fTempPerm502 = fTemp545;
																																									self.fTempPerm503 = fTemp546;
																																									self.iTempPerm504 = iTemp547;
																																									self.fTempPerm505 = fTemp548;
																																									self.fTempPerm506 = fTemp549;
																																									self.fTempPerm507 = fTemp550;
																																									self.fTempPerm508 = fTemp551;
																																									self.iTempPerm509 = iTemp552;
																																									self.fTempPerm510 = fTemp553;
																																									self.fTempPerm511 = fTemp554;
																																									self.fTempPerm512 = fTemp555;
																																									self.fTempPerm513 = fTemp556;
																																									self.iTempPerm514 = iTemp557;
																																									self.fTempPerm515 = fTemp558;
																																									self.fTempPerm516 = fTemp559;
																																									self.iTempPerm517 = iTemp560;
																																									self.iTempPerm518 = iTemp561;
																																									self.fTempPerm519 = fTemp562;
																																									self.fTempPerm520 = fTemp563;
																																									self.iTempPerm521 = iTemp565;
																																									self.iTempPerm522 = iTemp566;
																																									self.iTempPerm523 = iTemp567;
																																									self.iTempPerm524 = iTemp568;
																																									self.iTempPerm525 = iTemp569;
																																									self.iTempPerm526 = iTemp570;
																																									self.iTempPerm527 = iTemp571;
																																									self.iTempPerm528 = iTemp572;
																																									self.iTempPerm529 = iTemp573;
																																									self.iTempPerm530 = iTemp574;
																																									self.iTempPerm531 = iTemp575;
																																									self.iTempPerm532 = iTemp576;
																																									self.iTempPerm533 = iTemp577;
																																									self.iTempPerm534 = iTemp578;
																																									self.iTempPerm535 = iTemp579;
																																									self.fTempPerm536 = fTemp580;
																																									self.fTempPerm537 = fTemp581;
																																									self.iTempPerm538 = iTemp582;
																																									self.fTempPerm539 = fTemp583;
																																									self.iTempPerm540 = iTemp584;
																																									self.iTempPerm541 = iTemp585;
																																									self.fTempPerm542 = fTemp586;
																																									self.iTempPerm543 = iTemp587;
																																									self.iTempPerm544 = iTemp588;
																																									self.fTempPerm545 = fTemp589;
																																									self.fTempPerm546 = fTemp590;
																																									self.fTempPerm547 = fTemp591;
																																									self.fTempPerm548 = fTemp592;
																																									self.iTempPerm549 = iTemp593;
																																									self.iTempPerm550 = iTemp594;
																																									self.fTempPerm551 = fTemp595;
																																									self.fTempPerm552 = fTemp596;
																																									self.iTempPerm553 = iTemp597;
																																									self.fTempPerm554 = fTemp598;
																																									self.iTempPerm555 = iTemp599;
																																									self.iTempPerm556 = iTemp600;
																																									self.iTempPerm557 = iTemp601;
																																									self.iTempPerm558 = iTemp602;
																																									self.fTempPerm559 = fTemp603;
																																									self.fTempPerm560 = fTemp604;
																																									self.fTempPerm561 = fTemp605;
																																									self.fTempPerm562 = fTemp606;
																																									self.fTempPerm563 = fTemp607;
																																									self.iTempPerm564 = iTemp608;
																																									self.iTempPerm565 = iTemp609;
																																									self.fTempPerm566 = fTemp610;
																																									self.fTempPerm567 = fTemp611;
																																									self.iTempPerm568 = iTemp612;
																																									self.fTempPerm569 = fTemp613;
																																									self.iTempPerm570 = iTemp614;
																																									self.iTempPerm571 = iTemp615;
																																									self.iTempPerm572 = iTemp616;
																																									self.iTempPerm573 = iTemp617;
																																									self.iTempPerm574 = iTemp618;
																																									self.fTempPerm575 = fTemp619;
																																									self.iTempPerm576 = iTemp620;
																																									self.iTempPerm577 = iTemp621;
																																									self.iTempPerm578 = iTemp622;
																																									self.iTempPerm579 = iTemp623;
																																									self.fTempPerm580 = fTemp624;
																																									self.fTempPerm581 = fTemp625;
																																									self.iTempPerm582 = iTemp626;
																																									self.fTempPerm583 = fTemp627;
																																									self.fTempPerm584 = fTemp628;
																																									self.iTempPerm585 = iTemp629;
																																									self.iTempPerm586 = iTemp630;
																																									self.fTempPerm587 = fTemp631;
																																									self.fTempPerm588 = fTemp632;
																																									self.iTempPerm589 = iTemp633;
																																									self.fTempPerm590 = fTemp634;
																																									self.fTempPerm591 = fTemp635;
																																									self.fTempPerm592 = fTemp636;
																																									self.fTempPerm593 = fTemp637;
																																									self.iTempPerm594 = iTemp638;
																																									self.fTempPerm595 = fTemp639;
																																									self.fTempPerm596 = fTemp640;
																																									self.fTempPerm597 = fTemp641;
																																									self.fTempPerm598 = fTemp642;
																																									self.iTempPerm599 = iTemp643;
																																									self.fTempPerm600 = fTemp644;
																																									self.fTempPerm601 = fTemp645;
																																									self.iTempPerm602 = iTemp646;
																																									self.iTempPerm603 = iTemp647;
																																									self.fTempPerm604 = fTemp648;
																																									self.fTempPerm605 = fTemp649;
																																									self.iTempPerm606 = iTemp651;
																																									self.iTempPerm607 = iTemp652;
																																									self.iTempPerm608 = iTemp653;
																																									self.iTempPerm609 = iTemp654;
																																									self.iTempPerm610 = iTemp655;
																																									self.iTempPerm611 = iTemp656;
																																									self.iTempPerm612 = iTemp657;
																																									self.iTempPerm613 = iTemp658;
																																									self.iTempPerm614 = iTemp659;
																																									self.iTempPerm615 = iTemp660;
																																									self.iTempPerm616 = iTemp661;
																																									self.iTempPerm617 = iTemp662;
																																									self.iTempPerm618 = iTemp663;
																																									self.iTempPerm619 = iTemp664;
																																									self.iTempPerm620 = iTemp665;
																																									self.fTempPerm621 = fTemp666;
																																									self.fTempPerm622 = fTemp667;
																																									self.iTempPerm623 = iTemp668;
																																									self.fTempPerm624 = fTemp669;
																																									self.iTempPerm625 = iTemp670;
																																									self.iTempPerm626 = iTemp671;
																																									self.fTempPerm627 = fTemp672;
																																									self.iTempPerm628 = iTemp673;
																																									self.iTempPerm629 = iTemp674;
																																									self.fTempPerm630 = fTemp675;
																																									self.fTempPerm631 = fTemp676;
																																									self.fTempPerm632 = fTemp677;
																																									self.fTempPerm633 = fTemp678;
																																									self.iTempPerm634 = iTemp679;
																																									self.iTempPerm635 = iTemp680;
																																									self.fTempPerm636 = fTemp681;
																																									self.fTempPerm637 = fTemp682;
																																									self.iTempPerm638 = iTemp683;
																																									self.fTempPerm639 = fTemp684;
																																									self.iTempPerm640 = iTemp685;
																																									self.iTempPerm641 = iTemp686;
																																									self.iTempPerm642 = iTemp687;
																																									self.iTempPerm643 = iTemp688;
																																									self.fTempPerm644 = fTemp689;
																																									self.fTempPerm645 = fTemp690;
																																									self.fTempPerm646 = fTemp691;
																																									self.fTempPerm647 = fTemp692;
																																									self.fTempPerm648 = fTemp693;
																																									self.iTempPerm649 = iTemp694;
																																									self.iTempPerm650 = iTemp695;
																																									self.fTempPerm651 = fTemp696;
																																									self.fTempPerm652 = fTemp697;
																																									self.iTempPerm653 = iTemp698;
																																									self.fTempPerm654 = fTemp699;
																																									self.iTempPerm655 = iTemp700;
																																									self.iTempPerm656 = iTemp701;
																																									self.iTempPerm657 = iTemp702;
																																									self.iTempPerm658 = iTemp703;
																																									self.iTempPerm659 = iTemp704;
																																									self.fTempPerm660 = fTemp705;
																																									self.iTempPerm661 = iTemp706;
																																									self.iTempPerm662 = iTemp707;
																																									self.iTempPerm663 = iTemp708;
																																									self.iTempPerm664 = iTemp709;
																																									self.fTempPerm665 = fTemp710;
																																									self.fTempPerm666 = fTemp711;
																																									self.iTempPerm667 = iTemp712;
																																									self.fTempPerm668 = fTemp713;
																																									self.fTempPerm669 = fTemp714;
																																									self.iTempPerm670 = iTemp715;
																																									self.iTempPerm671 = iTemp716;
																																									self.fTempPerm672 = fTemp717;
																																									self.fTempPerm673 = fTemp718;
																																									self.iTempPerm674 = iTemp719;
																																									self.fTempPerm675 = fTemp720;
																																									self.fTempPerm676 = fTemp721;
																																									self.fTempPerm677 = fTemp722;
																																									self.fTempPerm678 = fTemp723;
																																									self.iTempPerm679 = iTemp724;
																																									self.fTempPerm680 = fTemp725;
																																									self.fTempPerm681 = fTemp726;
																																									self.fTempPerm682 = fTemp727;
																																									self.fTempPerm683 = fTemp728;
																																									self.iTempPerm684 = iTemp729;
																																									self.fTempPerm685 = fTemp730;
																																									self.fTempPerm686 = fTemp731;
																																									self.iTempPerm687 = iTemp732;
																																									self.iTempPerm688 = iTemp733;
																																									self.fTempPerm689 = fTemp734;
																																									self.fTempPerm690 = fTemp735;
																																									self.fTempPerm691 = fTemp737;
																																									self.fTempPerm692 = fTemp738;
																																									self.fTempPerm693 = fTemp739;
																																									self.fTempPerm694 = fTemp740;
																																									self.fTempPerm695 = fTemp741;
																																									self.fTempPerm696 = fTemp742;
																																									self.fTempPerm697 = fTemp743;
																																									self.fTempPerm698 = fTemp744;
																																								}

}

