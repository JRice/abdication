ABDrum sample library
=====================

Each folder is one selectable list in ABDrum. The folders are the source of truth:
add, delete, or rename .wav files freely. List order is alphabetical, so prefix a
name (e.g. "00 ") to move it to the top.

transients/   short attack layers, peak-normalized to -1 dBFS
  thump       low and tonal (~300 Hz center): Jade, plus kick/tom heads high-passed at 500 Hz
  knock       mid and tonal (~2 kHz center): Jade, hat heads, rims, kick clicks
  spark       bright and noisy (~9 kHz center): Mech, hat and shaker heads, Aes Dana
noise/        seamless loops, mono 44.1 kHz, RMS-matched to -20 dBFS (crackle is peak-capped, so quieter)
  steady      1 s loops: start point is irrelevant, the character is in the color
  crackle     4 s loops: impulsive; random start points give real variation
  modulated   4 s loops: level and spectrum move; densest window chosen

"... head" files are the first 12 ms of a hit from drum samples/, with a 5 ms fade.

Provenance: abdrum tools/build_library.py. It builds each group once and then
leaves it alone (the hidden .complete marker); it never overwrites a file.
