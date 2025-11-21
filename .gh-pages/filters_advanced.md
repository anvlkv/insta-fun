---
layout: no_sidebar
---
# filters_advanced

Advanced nonlinear & morphing filter snapshots (`hacker32` API).  
Each entry shows a short description, the core expression, an SVG chart image, and a WAV audio player.

[Back to main index](./index.md)

---

## Contents

- [adv_dirty_lowpass_1000hz_q0_70_shape0_50](#adv_dirty_lowpass_1000hz_q0_70_shape0_50)
- [adv_dirty_highpass_1000hz_q0_70_shape0_50](#adv_dirty_highpass_1000hz_q0_70_shape0_50)
- [adv_dirty_resonator_1000hz_q5_00_shape0_80](#adv_dirty_resonator_1000hz_q5_00_shape0_80)
- [adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50](#adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50)
- [adv_fb_lowpass_1000hz_q0_70_shape0_50](#adv_fb_lowpass_1000hz_q0_70_shape0_50)
- [adv_fb_highpass_1000hz_q0_70_shape0_50](#adv_fb_highpass_1000hz_q0_70_shape0_50)
- [adv_fb_resonator_1000hz_q6_00_shape0_80](#adv_fb_resonator_1000hz_q6_00_shape0_80)
- [adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50](#adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50)
- [adv_morph_1000hz_q0_80_m_lp](#adv_morph_1000hz_q0_80_m_lp)
- [adv_morph_1000hz_q0_80_m_lp_mid](#adv_morph_1000hz_q0_80_m_lp_mid)
- [adv_morph_1000hz_q0_80_m_peak](#adv_morph_1000hz_q0_80_m_peak)
- [adv_morph_1000hz_q0_80_m_hp_mid](#adv_morph_1000hz_q0_80_m_hp_mid)
- [adv_morph_1000hz_q0_80_m_hp](#adv_morph_1000hz_q0_80_m_hp)

---

### adv_dirty_lowpass_1000hz_q0_70_shape0_50
Dirty lowpass 1 kHz Q=0.70 shape=0.50.  
Expression: `dlowpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_dirty_lowpass_1000hz_q0_70_shape0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_dirty_lowpass_1000hz_q0_70_shape0_50.snap.wav"></audio>

### adv_dirty_highpass_1000hz_q0_70_shape0_50
Dirty highpass 1 kHz Q=0.70 shape=0.50.  
Expression: `dhighpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_dirty_highpass_1000hz_q0_70_shape0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_dirty_highpass_1000hz_q0_70_shape0_50.snap.wav"></audio>

### adv_dirty_resonator_1000hz_q5_00_shape0_80
Dirty resonator 1 kHz Q=5 shape=0.80.  
Expression: `dresonator_hz(SoftCrush(0.80f32), 1000.0f32, 5.00f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_dirty_resonator_1000hz_q5_00_shape0_80.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_dirty_resonator_1000hz_q5_00_shape0_80.snap.wav"></audio>

### adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50
Dirty bell 1 kHz Q=0.70 gain=1.50 shape=0.50.  
Expression: `dbell_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32, 1.50f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50.snap.wav"></audio>

### adv_fb_lowpass_1000hz_q0_70_shape0_50
Feedback lowpass 1 kHz Q=0.70 shape=0.50.  
Expression: `flowpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_fb_lowpass_1000hz_q0_70_shape0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_fb_lowpass_1000hz_q0_70_shape0_50.snap.wav"></audio>

### adv_fb_highpass_1000hz_q0_70_shape0_50
Feedback highpass 1 kHz Q=0.70 shape=0.50.  
Expression: `fhighpass_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_fb_highpass_1000hz_q0_70_shape0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_fb_highpass_1000hz_q0_70_shape0_50.snap.wav"></audio>

### adv_fb_resonator_1000hz_q6_00_shape0_80
Feedback resonator 1 kHz Q=6 shape=0.80.  
Expression: `fresonator_hz(SoftCrush(0.80f32), 1000.0f32, 6.00f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_fb_resonator_1000hz_q6_00_shape0_80.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_fb_resonator_1000hz_q6_00_shape0_80.snap.wav"></audio>

### adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50
Feedback bell 1 kHz Q=0.70 gain=1.50 shape=0.50.  
Expression: `fbell_hz(SoftCrush(0.50f32), 1000.0f32, 0.70f32, 1.50f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50.snap.wav"></audio>

### adv_morph_1000hz_q0_80_m_lp
Morph lowpass pos -1.0 Q=0.80.  
Expression: `morph_hz(1000.0f32, 0.80f32, -1.0f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_lp.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_lp.snap.wav"></audio>

### adv_morph_1000hz_q0_80_m_lp_mid
Morph mid toward peak (-0.5).  
Expression: `morph_hz(1000.0f32, 0.80f32, -0.5f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_lp_mid.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_lp_mid.snap.wav"></audio>

### adv_morph_1000hz_q0_80_m_peak
Morph peak pos 0.0.  
Expression: `morph_hz(1000.0f32, 0.80f32, 0.0f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_peak.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_peak.snap.wav"></audio>

### adv_morph_1000hz_q0_80_m_hp_mid
Morph mid toward highpass (0.5).  
Expression: `morph_hz(1000.0f32, 0.80f32, 0.5f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_hp_mid.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_hp_mid.snap.wav"></audio>

### adv_morph_1000hz_q0_80_m_hp
Morph highpass pos 1.0.  
Expression: `morph_hz(1000.0f32, 0.80f32, 1.0f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_hp.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/filters_advanced__adv_morph_1000hz_q0_80_m_hp.snap.wav"></audio>

---