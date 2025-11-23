---
layout: no_sidebar
---
# Filters

Snapshot gallery for fundamental filter types in `fundsp` (impulse response SVG + 1s WAV audio).

[Back to main index](./index.md)

## Contents

- [filter_svf_lowpass_1k_q0_707](#filter_svf_lowpass_1k_q0_707)
- [filter_svf_highpass_1k_q0_707](#filter_svf_highpass_1k_q0_707)
- [filter_svf_bandpass_1k_q1_0](#filter_svf_bandpass_1k_q1_0)
- [filter_svf_notch_1k_q1_0](#filter_svf_notch_1k_q1_0)
- [filter_peak_1k_q1_0](#filter_peak_1k_q1_0)
- [filter_bell_1k_q0_707_gain1_5](#filter_bell_1k_q0_707_gain1_5)
- [filter_lowshelf_500hz_q0_707_gain1_5](#filter_lowshelf_500hz_q0_707_gain1_5)
- [filter_highshelf_2k_q0_707_gain1_5](#filter_highshelf_2k_q0_707_gain1_5)
- [filter_moog_lowpass_1k_q0_5](#filter_moog_lowpass_1k_q0_5)
- [filter_lowrez_lowpass_1k_q1_0](#filter_lowrez_lowpass_1k_q1_0)
- [filter_bandrez_bandpass_1k_q3_0](#filter_bandrez_bandpass_1k_q3_0)
- [filter_lowpole_800hz](#filter_lowpole_800hz)
- [filter_highpole_300hz](#filter_highpole_300hz)
- [filter_morph_lowpass_1k_q0_8](#filter_morph_lowpass_1k_q0_8)
- [filter_dcblock_10hz](#filter_dcblock_10hz)
- [filter_allpass_1k_q0_707](#filter_allpass_1k_q0_707)
- [filter_butter_lowpass_1k](#filter_butter_lowpass_1k)
- [filter_fir3_gain0_50_white](#filter_fir3_gain0_50_white)
- [filter_pinkpass_white](#filter_pinkpass_white)
- [filter_follow_attack100ms](#filter_follow_attack100ms)

---

## Input

All filter examples use following input:

```rust
sine_hz::<f32>(110.0)
    + sine_hz::<f32>(220.0)
    + sine_hz::<f32>(440.0)
    + sine_hz::<f32>(880.0)
```

### filter_svf_lowpass_1k_q0_707
Lowpass 1 kHz Q≈0.707  
Expression: `lowpass_hz(1_000.0, 0.707)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_svf_lowpass_1k_q0_707.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_svf_lowpass_1k_q0_707@audio.snap.wav"></audio>

### filter_svf_highpass_1k_q0_707
Highpass 1 kHz Q≈0.707  
Expression: `highpass_hz(1_000.0, 0.707)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_svf_highpass_1k_q0_707.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_svf_highpass_1k_q0_707@audio.snap.wav"></audio>

### filter_svf_bandpass_1k_q1_0
Bandpass 1 kHz Q=1.0  
Expression: `bandpass_hz(1_000.0, 1.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_svf_bandpass_1k_q1_0.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_svf_bandpass_1k_q1_0@audio.snap.wav"></audio>

### filter_svf_notch_1k_q1_0
Notch 1 kHz Q=1.0  
Expression: `notch_hz(1_000.0, 1.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_svf_notch_1k_q1_0.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_svf_notch_1k_q1_0@audio.snap.wav"></audio>

### filter_peak_1k_q1_0
Peak 1 kHz Q=1.0  
Expression: `peak_hz(1_000.0, 1.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_peak_1k_q1_0.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_peak_1k_q1_0@audio.snap.wav"></audio>

### filter_bell_1k_q0_707_gain1_5
Bell 1 kHz Q≈0.707 gain≈+3.5 dB  
Expression: `bell_hz(1_000.0, 0.707, 1.5)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_bell_1k_q0_707_gain1_5.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_bell_1k_q0_707_gain1_5@audio.snap.wav"></audio>

### filter_lowshelf_500hz_q0_707_gain1_5
Low shelf 500 Hz Q≈0.707 gain 1.5  
Expression: `lowshelf_hz(500.0, 0.707, 1.5)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_lowshelf_500hz_q0_707_gain1_5.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_lowshelf_500hz_q0_707_gain1_5@audio.snap.wav"></audio>

### filter_highshelf_2k_q0_707_gain1_5
High shelf 2 kHz Q≈0.707 gain 1.5  
Expression: `highshelf_hz(2_000.0, 0.707, 1.5)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_highshelf_2k_q0_707_gain1_5.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_highshelf_2k_q0_707_gain1_5@audio.snap.wav"></audio>

### filter_moog_lowpass_1k_q0_5
Moog ladder lowpass 1 kHz Q=0.5  
Expression: `moog_hz(1_000.0, 0.5)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_moog_lowpass_1k_q0_5.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_moog_lowpass_1k_q0_5@audio.snap.wav"></audio>

### filter_lowrez_lowpass_1k_q1_0
Lowrez lowpass 1 kHz Q=1.0  
Expression: `lowrez_hz(1_000.0, 1.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_lowrez_lowpass_1k_q1_0.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_lowrez_lowpass_1k_q1_0@audio.snap.wav"></audio>

### filter_bandrez_bandpass_1k_q3_0
Bandrez bandpass 1 kHz Q=3.0  
Expression: `bandrez_hz(1_000.0, 3.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_bandrez_bandpass_1k_q3_0.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_bandrez_bandpass_1k_q3_0@audio.snap.wav"></audio>

### filter_lowpole_800hz
One-pole lowpass 800 Hz  
Expression: `lowpole_hz(800.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_lowpole_800hz.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_lowpole_800hz@audio.snap.wav"></audio>

### filter_highpole_300hz
One-pole highpass 300 Hz  
Expression: `highpole_hz(300.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_highpole_300hz.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_highpole_300hz@audio.snap.wav"></audio>

### filter_morph_lowpass_1k_q0_8
Morph lowpass 1 kHz Q=0.8 morph=-1.0  
Expression: `morph_hz(1_000.0, 0.8, -1.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_morph_lowpass_1k_q0_8.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_morph_lowpass_1k_q0_8@audio.snap.wav"></audio>

### filter_dcblock_10hz
DC blocker 10 Hz  
Expression: `dcblock_hz(10.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_dcblock_10hz.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_dcblock_10hz@audio.snap.wav"></audio>

### filter_allpass_1k_q0_707
Allpass 1 kHz Q≈0.707  
Expression: `allpass_hz(1_000.0, 0.707)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_allpass_1k_q0_707.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_allpass_1k_q0_707@audio.snap.wav"></audio>

### filter_butter_lowpass_1k
Butterworth lowpass 1 kHz (2nd order)  
Expression: `butterpass_hz(1_000.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_butter_lowpass_1k.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_butter_lowpass_1k@audio.snap.wav"></audio>

### filter_fir3_gain0_50_white
FIR3 Nyquist gain 0.50 (white noise input)  
Expression: `fir3(0.50)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_fir3_gain0_50_white.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_fir3_gain0_50_white@audio.snap.wav"></audio>

### filter_pinkpass_white
Pinking filter shaping white noise (≈3 dB/oct slope)  
Expression: `pinkpass::<f32>()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_pinkpass_white.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_pinkpass_white@audio.snap.wav"></audio>

### filter_follow_attack100ms
Amplitude follower (~100 ms) on modulated 220 Hz sine  
Expression: `sine_hz::<f32>(220.0) * (1.0 + 0.5 * sine_hz::<f32>(2.0)) >> follow(0.10)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_follow_attack100ms.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters__filter_follow_attack100ms@audio.snap.wav"></audio>

---
