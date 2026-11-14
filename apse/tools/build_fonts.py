#!/usr/bin/env python3
"""Cuts a panel font down to what it draws, fixing a variable font's weight first.

    build_fonts.py <in.ttf> <out.ttf> <weight or -> <chars>

The panel's fonts came from Google Fonts' repository (google/fonts, ofl/): Cinzel[wght].ttf at
400 and 600, CormorantGaramond-Italic[wght].ttf at 400, with the printable ASCII plus "·–−".
"""
import sys
from fontTools import subset
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer

source, target, weight, chars = sys.argv[1:5]
font = TTFont(source)
if weight != "-":
    font = instancer.instantiateVariableFont(font, {"wght": float(weight)})
options = subset.Options(layout_features=["*"], hinting=False)
subsetter = subset.Subsetter(options)
subsetter.populate(text=chars)
subsetter.subset(font)
font.flavor = None
font.save(target)
