---
layout: no_sidebar
---
# Advanced Oscillators

Back to [index](./index.html)

Minimal snapshots of advanced oscillator variants (hacker32 API). Each entry:
- SVG waveform
- 1 s WAV audio
- Expression (short form)

---

## Contents

- [DSF Saw Roughness Sweep](#dsf-saw-roughness-sweep)
  - [adv_dsf_saw_440hz_rough0_00](#adv_dsf_saw_440hz_rough0_00)
  - [adv_dsf_saw_440hz_rough0_50](#adv_dsf_saw_440hz_rough0_50)
  - [adv_dsf_saw_440hz_rough1_00](#adv_dsf_saw_440hz_rough1_00)
  - [adv_dsf_saw_440hz_rough0_99](#adv_dsf_saw_440hz_rough0_99)
- [DSF Square Roughness Sweep](#dsf-square-roughness-sweep)
  - [adv_dsf_square_440hz_rough0_00](#adv_dsf_square_440hz_rough0_00)
  - [adv_dsf_square_440hz_rough0_50](#adv_dsf_square_440hz_rough0_50)
  - [adv_dsf_square_440hz_rough1_00](#adv_dsf_square_440hz_rough1_00)
  - [adv_dsf_square_440hz_rough0_99](#adv_dsf_square_440hz_rough0_99)
- [Pulse Width Sweep](#pulse-width-sweep)
  - [adv_pulse_440hz_width0_10](#adv_pulse_440hz_width0_10)
  - [adv_pulse_440hz_width0_25](#adv_pulse_440hz_width0_25)
  - [adv_pulse_440hz_width0_50](#adv_pulse_440hz_width0_50)
  - [adv_pulse_440hz_width0_75](#adv_pulse_440hz_width0_75)
- [Synth Threshold Pulses](#synth-threshold-pulses)
  - [adv_synth_pulse_440hz_duty0_10](#adv_synth_pulse_440hz_duty0_10)
  - [adv_synth_pulse_440hz_duty0_25](#adv_synth_pulse_440hz_duty0_25)
  - [adv_synth_pulse_440hz_duty0_75](#adv_synth_pulse_440hz_duty0_75)
- [Simple PWM](#simple-pwm)
  - [adv_pwm_sine_440hz_lfo1hz](#adv_pwm_sine_440hz_lfo1hz)

---

## DSF Saw Roughness Sweep

### adv_dsf_saw_440hz_rough0_00
Roughness 0.00  
Expression: `constant(440.0f32) >> dsf_saw_r(0.00f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_saw_440hz_rough0_00.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_saw_440hz_rough0_00.snap.wav"></audio>

### adv_dsf_saw_440hz_rough0_50
Roughness 0.50  
Expression: `constant(440.0f32) >> dsf_saw_r(0.50f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_saw_440hz_rough0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_saw_440hz_rough0_50.snap.wav"></audio>

### adv_dsf_saw_440hz_rough1_00
Roughness 1.00  
Expression: `constant(440.0f32) >> dsf_saw_r(1.00f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_saw_440hz_rough1_00.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_saw_440hz_rough1_00.snap.wav"></audio>

### adv_dsf_saw_440hz_rough0_99
Roughness 0.99 (safe)  
Expression: `constant(440.0f32) >> dsf_saw_r(0.99f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_saw_440hz_rough0_99.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_saw_440hz_rough0_99.snap.wav"></audio>

---

## DSF Square Roughness Sweep

### adv_dsf_square_440hz_rough0_00
Roughness 0.00  
Expression: `constant(440.0f32) >> dsf_square_r(0.00f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_square_440hz_rough0_00.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_square_440hz_rough0_00.snap.wav"></audio>

### adv_dsf_square_440hz_rough0_50
Roughness 0.50  
Expression: `constant(440.0f32) >> dsf_square_r(0.50f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_square_440hz_rough0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_square_440hz_rough0_50.snap.wav"></audio>

### adv_dsf_square_440hz_rough1_00
Roughness 1.00  
Expression: `constant(440.0f32) >> dsf_square_r(1.00f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_square_440hz_rough1_00.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_square_440hz_rough1_00.snap.wav"></audio>

### adv_dsf_square_440hz_rough0_99
Roughness 0.99 (safe)  
Expression: `constant(440.0f32) >> dsf_square_r(0.99f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_square_440hz_rough0_99.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_dsf_square_440hz_rough0_99.snap.wav"></audio>

---

## Pulse Width Sweep

### adv_pulse_440hz_width0_10
Width 0.10  
Expression: `constant((440.0f32, 0.10f32)) >> pulse()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pulse_440hz_width0_10.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pulse_440hz_width0_10.snap.wav"></audio>

### adv_pulse_440hz_width0_25
Width 0.25  
Expression: `constant((440.0f32, 0.25f32)) >> pulse()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pulse_440hz_width0_25.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pulse_440hz_width0_25.snap.wav"></audio>

### adv_pulse_440hz_width0_50
Width 0.50  
Expression: `constant((440.0f32, 0.50f32)) >> pulse()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pulse_440hz_width0_50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pulse_440hz_width0_50.snap.wav"></audio>

### adv_pulse_440hz_width0_75
Width 0.75  
Expression: `constant((440.0f32, 0.75f32)) >> pulse()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pulse_440hz_width0_75.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pulse_440hz_width0_75.snap.wav"></audio>

---

## Synth Threshold Pulses

### adv_synth_pulse_440hz_duty0_10
Duty 0.10  
Expression: `synth_threshold_pulse(440.0f32, 0.10f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_synth_pulse_440hz_duty0_10.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_synth_pulse_440hz_duty0_10.snap.wav"></audio>

### adv_synth_pulse_440hz_duty0_25
Duty 0.25  
Expression: `synth_threshold_pulse(440.0f32, 0.25f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_synth_pulse_440hz_duty0_25.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_synth_pulse_440hz_duty0_25.snap.wav"></audio>

### adv_synth_pulse_440hz_duty0_75
Duty 0.75  
Expression: `synth_threshold_pulse(440.0f32, 0.75f32)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_synth_pulse_440hz_duty0_75.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_synth_pulse_440hz_duty0_75.snap.wav"></audio>

---

## Simple PWM

### adv_pwm_sine_440hz_lfo1hz
PWM (carrier 440 Hz, LFO 1 Hz)  
Expression: `(sine_hz(440.0f32) | sine_hz(1.0f32)) >> map(...)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pwm_sine_440hz_lfo1hz.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/oscillators_advanced__adv_pwm_sine_440hz_lfo1hz.snap.wav"></audio>

---