// plugin-fx.dsp — ABDrum's effects, run once per pad on the sum of its
// overlapping hits. The plugin drives `tune` (Tone's pitch, keytracked to the
// latest note when Keytrack is on), `hit` (a pulse on every note-on) and
// `vel` (its velocity), and passes on the voice's Envelope settings, so the
// effects' sends (their rings) follow the same envelope, restarted by every hit.
//
//   Crush -> Drive -> Ring mod -> Comb -> Compressor

declare name "ABDrumFX";
import("stdfaust.lib");
fx = library("fx.lib");
md = library("mod.lib");

smooth = si.smoo;
tune = hslider("tune", 50, 1, 20000, 0.01);
hit  = button("hit") <: _ > _';
vel  = hslider("vel", 1, 0, 1, 0.001);

modEnv = md.envelope(hslider("env_attack", 0, 0, 1000, 0.1) / 1000 : smooth, hslider("env_decay", 300, 5, 4000, 0.1) / 1000 : smooth,
                     hslider("env_curve", 0.5, 0, 1, 0.001) : smooth, hslider("env_velocity", 0.5, 0, 1, 0.001) : smooth, vel, hit);
// Each control below takes a send ("Mod": its ring), -1..1 of its travel.

cr(x) = hgroup("[6]Crush", x);
crAmount = cr(hslider("[0]Amount [tooltip:Lowers the sample rate (to 2 kHz), then the bit depth (to 4)]", 0, 0, 1, 0.001)) : smooth
         : md.lin(0, 1, (cr(hslider("[0]Amount Mod", 0, -1, 1, 0.001)) : smooth), modEnv);

dv(x) = hgroup("[7]Drive", x);
dvType   = dv(nentry("[0]Type [style:menu{'Tape':0;'Tube':1;'Clip':2}]", 0, 0, 2, 1));
dvAmount = dv(hslider("[1]Amount [tooltip:First third: blends the drive under the clean hit. Above: all drive, harder]", 0, 0, 1, 0.001)) : smooth
         : md.lin(0, 1, (dv(hslider("[1]Amount Mod", 0, -1, 1, 0.001)) : smooth), modEnv);

rmg(x) = hgroup("[8]Ring Mod", x);
rmAmount = rmg(hslider("[0]Amount [tooltip:Deepens the ring mod as its carrier climbs from 1.07x to 3.7x Tone's pitch]", 0, 0, 1, 0.001)) : smooth
         : md.lin(0, 1, (rmg(hslider("[0]Amount Mod", 0, -1, 1, 0.001)) : smooth), modEnv);

cb(x) = hgroup("[9]Comb", x);
cbAmount = cb(hslider("[0]Amount [tooltip:Feedback (ring length) and mix together. Tuned to Tone's pitch, inharmonic]", 0, 0, 1, 0.001)) : smooth
         : md.lin(0, 1, (cb(hslider("[0]Amount Mod", 0, -1, 1, 0.001)) : smooth), modEnv);

cp(x) = hgroup("[10]Compressor", x);
cpThreshold = cp(hslider("[0]Threshold [unit:dB] [tooltip:0 dB is off. 4:1, hard knee, auto makeup]", 0, -40, 0, 0.1)) : smooth
            : md.lin(-40, 0, (cp(hslider("[0]Threshold Mod", 0, -1, 1, 0.001)) : smooth), modEnv);
cpTiming    = cp(nentry("[1]Timing [style:menu{'Squash 0.3/50 ms':0;'Tight 1/80 ms':1;'Punch 3/120 ms':2;'Snap 10/200 ms':3;'Open 30/400 ms':4}]", 2, 0, 4, 1));

process = fx.crush(crAmount) : fx.drive(dvType, dvAmount) : fx.ringMod(rmAmount, tune, hit) : fx.comb(cbAmount, tune) : fx.compressor(cpThreshold, cpTiming);
