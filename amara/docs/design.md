# How Amara works

Nothing is sampled, and there's no recording behind it. It's a rule-based speech synthesizer writing into a real low-bitrate vocoder, so its "accuracy" can only be judged by ear.

## Language (`language.rs`)

A seed picks about 10-15 common consonants, weighted roughly by how many of the world's languages have them, with `n` and `k` always among them. Those weights are approximate, from memory of PHOIBLE's figures, not read from its data.

Strangeness then adds a small signature of register sounds (uvulars, pharyngeals, ejectives, pharyngealized consonants), and sets how much they're used. That's how such sounds behave in real languages: Arabic's throat sounds sit on an ordinary consonant system. Implications are honoured, so ejective and voiced stops bring their plain partners along.

Vowels come as systems, not draws: always a low vowel, `i` and `u`, with five qualities the likeliest size, as in WALS (188 of 564 languages). The corners are also used more often than the rest.

It also sets syllable-shape weights, a stress rule (initial, penultimate or final), a rhythm (unstressed vowels shrunk hard for stress-timed, barely for syllable-timed), a long-vowel rate, and optionally backness harmony, final devoicing and intervocalic spirantization. Clusters must rise toward the nucleus and fall away from it in sonority, by at least two steps.

There's no grammar, but there is a pool of 36 roots, 6 affixes and 4 particles chosen with a Zipf-like bias. A handful of morphemes does much of the talking, which is what makes it sound as though it might mean something.

## Prosody and articulation (`speaker.rs`)

A phrase has 2-6 words, accents on stressed syllables that step down through the phrase, one focused word, a falling or occasionally rising boundary tone, final lengthening and declination.

Durations follow Klatt's rule: only the part of a phone above its floor stretches or shrinks.

Consonants aim at their place's locus, blended toward the next vowel. Vowels back and lower beside throat consonants. Stops are a closure, a burst and aspiration; ejectives add a shut glottis and a creaky onset; trills are real beats. Formants glide through two smoothing stages, which gives the transitions their shape.

Temper is a phrase-level value that wanders, plus an arch through the phrase, scaled by Swing. It moves pitch, range, effort, spectral tilt and roughness.

## Codec and line (`codec.rs`, `voice.rs`)

Frication, bursts, aspiration and whisper use broad resonances and no voiced spectral slope, because noise through a vowel's narrow resonance is a whistle. A test holds every fricative's sharpest decoded peak to at least 300 Hz across rates and tract lengths.

Each frame's formants become an all-pole filter, then 14 reflection coefficients carried as log area ratios (60 bits), plus pitch, voicing, roughness and gain (17 bits). The 6-16 kHz rate and 10-30 ms frames set by Fidelity bracket real vocoders: LPC-10 was 8 kHz with 22.5 ms frames. This order and bit layout are its own.

The coefficient quantizer can say exactly zero, so it doesn't invent resonances. Log area ratios mean no bit pattern can go unstable, but they can land very close, and those are the whistles. Output level is normalized from the residual, as in LPC-10, so a wild filter stays at the frame's level, and a soft clip catches what's left.

The line has bursty errors with a good and a bad state. Lost frames repeat, fading.

## Possession (`possession.rs`)

Events arrive at exponentially distributed gaps and work on the tract before coding (giant, mite, fry, howl), on the line (scramble), or on decoded frames (whistle, stuck, reverse, stutter, legion). The language keeps moving underneath throughout, and playback uses about eight seconds of frame history.

## Checked by measurement

Syllable rate, silence, pitch range and level respond to their knobs in the expected directions, in tests and on full renders. Spectrograms show formant transitions, closures, frication and pitch movement like speech's. Across languages, tests also check the corner vowels, the share of exotic consonants by Strangeness, and implications. Every language was checked for sonority, and extremes for stability. None of that shows it sounds good, which is your ears' job.

## Per-note mode

The language's stream of words carries on, but only a note moves it on by one syllable, keeping its stress and recurrences. The prosody leaves pitch alone so it stays in tune, and velocity sets a per-syllable temper. A note forces a new codec frame, so timing doesn't wait on the frame grid. It still takes a frame to fade in, and a stop's closure comes before its burst.

## Tentacles (`editor/tentacles.rs`)

At most three, each living about 30 s with a smooth fade. Each is a chain of 26 fixed-length links from a root below the window. Slow waves along the chain set how much each link turns from the last, so links can't stretch or drift, and no physics needs solving. Under software rendering they added about 3 % of a core to the panel, against about 41 % for the rest of it. That's a pessimistic setting, and on a real graphics card it hasn't been measured.
