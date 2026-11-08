# How Arecibo works

There's no recording behind this. It's a design, checked by measurement, and only ears can say whether it's any good.

## The idea

A no-input mixer is a loop of saturating gain stages and EQ, with its outputs patched back into its inputs. It oscillates wherever loop gain passes one. Arecibo puts a radio link in that loop, beside the wire, so the receiver's behaviour becomes part of what's oscillating.

Four strips feed each other through a matrix with mixed polarity, and each takes its own outside input: main left, main right, sidechain left, sidechain right. Each strip's output splits two ways, the wire straight back and a trip through the sky, and Sky blends them.

## Why the radio matters

- **Hang AGC.** After a peak, the receiver holds its gain for a while, then lets go. In a loop that starves the signal and then floods it, so the machine breathes at a rate no LFO sets. This turned out to be the main source of life: without hang, the same loop settled into steady lines.
- **Desensing.** Outside signal also reaches the AGC directly, as a strong nearby signal overloads a real receiver whatever it's tuned to. My first version left this out, and a hard hit moved the level by only about 2 dB. Now a hit starves its own station for the AGC's hold time and the loop floods back, while quiet input just gets pulled in.
- **Fading.** Two sky paths fade independently as Rayleigh processes, a simplified version of the standard HF channel models. Deep fades collapse the loop and it has to rebuild. A test checks the power distribution against the exponential that Rayleigh fading implies.
- **Mistuned sideband.** Each trip round the loop moves everything by the Tune value. Because resonances keep pulling energy back, this mostly shows up as smearing and slow slips, not as clean rising tones, unless Peak and Width leave it room.
- **Static.** The AGC hauls up whatever's there, so the silence between bursts isn't silent.
- **Whistlers.** Some lightning energy runs along the Earth's magnetic field and returns as a long falling tone. Those happen at very low frequencies, so a shortwave set wouldn't really hear them. They're here because they're wonderful.
- **Long delayed echoes.** Radio operators have reported hearing their own signal seconds to tens of seconds later since 1927, and it still isn't properly explained. Here it's rare, weather-dependent, and goes back into the loop. The machine remembers 30 seconds of itself.

## The flaws (`wear.rs`)

Nothing is a chosen glitch. It's old equipment on a shared rail, so everything slowly depends on everything else. Load sags the supply, which cuts every gain and pulls every oscillator. Warmth, gathering over tens of seconds, drifts the oscillators and sharpens the peaks. Shaking opens bad joints. The phasing network is unbalanced, so a mirror image leaks through. Pots scratch when moved. Age scales all of it.

Some of these constants are guesses at plausible hardware, not measurements of any.

## Measured

Across 50-second renders, the level at the defaults sits around -20 dB RMS and moves about 6 dB, and over five minutes each 5-second stretch stays between -21 and -19 dB, peaking well below the limiter. So it breathes without drifting away. A test also confirms it hasn't settled into a drone after two minutes. Sky near 0 runs much hotter, around -8 dB RMS, so turn Level down there.

With a hard burst every two seconds, the level fell from about -20 dB to about -23 dB after a hit on one input pair, since the other two stations carry on, and to about -56 dB after hits on both at once, recovering in about a second at the default AGC.

The worst settings used about 10 % of one core. Panel cost is separate and only runs while it's open.
