#!/usr/bin/env python3
"""Build the panel's fonts from the @fontsource/noto-serif-tibetan npm package.

    build_fonts.py <package folder> <output folder>

The Tibetan files lack the decimal point and signs that the readouts need, and the
panel's text engine won't fall back to another font in the middle of a word, so those
glyphs get copied in from the same family's Latin file.
"""
import sys
from pathlib import Path

from fontTools import subset
from fontTools.ttLib import TTFont

TIBETAN = list(range(0x0F00, 0x1000)) + [0x20, 0xA0]
DIGITS = list(range(0x0F20, 0x0F2A)) + [0x20]
SIGNS = [0x2B, 0x2D, 0x2E]


def load(package, script, weight):
    return TTFont(package / "files" / f"noto-serif-tibetan-{script}-{weight}-normal.woff2")


def cut(font, unicodes):
    options = subset.Options()
    options.layout_features = ["*"]
    options.notdef_outline = True
    options.hinting = False
    subsetter = subset.Subsetter(options)
    subsetter.populate(unicodes=unicodes)
    subsetter.subset(font)
    font.flavor = None


def add_signs(font, latin):
    source_map = latin.getBestCmap()
    for code in SIGNS:
        name = source_map[code]
        font["glyf"][name] = latin["glyf"][name]
        font["hmtx"][name] = latin["hmtx"][name]
        for table in font["cmap"].tables:
            if table.isUnicode():
                table.cmap[code] = name
    font.setGlyphOrder(font["glyf"].glyphOrder)


def main():
    package, out = Path(sys.argv[1]), Path(sys.argv[2])
    latin = load(package, "latin", 400)
    cut(latin, SIGNS)
    regular = load(package, "tibetan", 400)
    cut(regular, TIBETAN)
    add_signs(regular, latin)
    regular.save(out / "NotoSerifTibetan-Regular.ttf")
    semibold = load(package, "tibetan", 600)
    cut(semibold, TIBETAN)
    semibold.save(out / "NotoSerifTibetan-SemiBold.ttf")
    digits = load(package, "tibetan", 900)
    cut(digits, DIGITS)
    digits.save(out / "NotoSerifTibetan-Digits.ttf")


if __name__ == "__main__":
    main()
