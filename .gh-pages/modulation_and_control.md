---
layout: no_sidebar
---
# modulation_and_control

Snapshot gallery for basic modulation and control sources/effects. Each entry:
- Terse description (WHY it’s representative)
- Core expression
- SVG chart (2000 samples)
- 1 s WAV audio

[Back to main index](./index.md)

---

## Contents
- [mod_adsr_a10ms_d100ms_s0_70_r200ms](#mod_adsr_a10ms_d100ms_s0_70_r200ms)
- [mod_lfo](#mod_lfo)
- [mod_sample_hold_5hz_var0_2](#mod_sample_hold_5hz_var0_2)
- [mod_follow_attack100ms](#mod_follow_attack100ms)

---

### mod_adsr_a10ms_d100ms_s0_70_r200ms
ADSR envelope (A=10 ms, D=100 ms, S=0.70, R=200 ms) applied to a 220 Hz sine.  
Expression: `(impulse() >> adsr_live(0.01, 0.10, 0.70, 0.20)) * sine_hz::<f32>(220.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/modulation_and_control__mod_adsr_a10ms_d100ms_s0_70_r200ms.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/modulation_and_control__mod_adsr_a10ms_d100ms_s0_70_r200ms@audio.snap.wav"></audio>

### mod_lfo
Exponential-decaying frequency LFO (native generator).  
Expression: `lfo(|t: f32| exp(-t))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/modulation_and_control__mod_lfo.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/modulation_and_control__mod_lfo@audio.snap.wav"></audio>

### mod_sample_hold_5hz_var0_2
Stepped random control source: hold_hz(5 Hz, variability 0.2) internally generates new levels every 200 ms.  
Expression: `hold_hz(5.0, 0.2)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/modulation_and_control__mod_sample_hold_5hz_var0_2.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/modulation_and_control__mod_sample_hold_5hz_var0_2@audio.snap.wav"></audio>

### mod_follow_attack100ms
Amplitude follower: tracks a slowly modulated 220 Hz source (2 Hz depth 50%). Attack ≈100 ms. Chart includes input & follower output for comparison.  
Expression: `(sine_hz::<f32>(220.0) * (1.0 + 0.5 * sine_hz::<f32>(2.0))) >> follow(0.10)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/modulation_and_control__mod_follow_attack100ms.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/modulation_and_control__mod_follow_attack100ms@audio.snap.wav"></audio>

---