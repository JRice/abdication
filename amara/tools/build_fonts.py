#!/usr/bin/env python3
"""Cuts the panel's fonts down to what it draws. Usage: build_fonts.py <in.woff> <out.ttf> <chars>"""
import sys
from fontTools import subset
from fontTools.ttLib import TTFont

source, target, chars = sys.argv[1:4]
font = TTFont(source)
options = subset.Options(layout_features=["*"], hinting=False)
subsetter = subset.Subsetter(options)
subsetter.populate(text=chars)
subsetter.subset(font)
font.flavor = None
font.save(target)
