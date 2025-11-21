---
layout: no_sidebar
---
# time_fx_and_noise

Snapshots of time-based effects, dynamics, resonators, delay, reverb, and noise generators. Each entry shows a terse description, expression, SVG chart, and WAV audio.

---

### fx_chorus
Chorus sine 440 Hz.  
Expression: `chorus(1, 0.015, 0.005, 0.2)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_chorus.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_chorus.snap.wav"></audio>

### fx_flanger
Flanger sine 440 Hz.  
Expression: `flanger(0.9, 0.005, 0.010, |t| ...)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_flanger.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_flanger.snap.wav"></audio>

### fx_phaser
Phaser sine 440 Hz.  
Expression: `phaser(0.5, |t| ...)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_phaser.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_phaser.snap.wav"></audio>

### dyn_limiter_mul2_in
Limiter after 2x gain.  
Expression: `mul(2.0) >> limiter(0.01, 0.1)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__dyn_limiter_mul2_in.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__dyn_limiter_mul2_in.snap.wav"></audio>

### fx_resonator_440hz_bw50
Resonator 440 Hz BW≈50.  
Expression: `resonator_hz(440.0, 50.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_resonator_440hz_bw50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_resonator_440hz_bw50.snap.wav"></audio>

### fx_reverb_stereo_room10m_time2s_damp0_5
Stereo reverb 10m 2s damp0.5.  
Expression: `(sine_hz::<f32>(220.0) | sine_hz::<f32>(220.0)) >> reverb_stereo(10.0, 2.0, 0.5)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_reverb_stereo_room10m_time2s_damp0_5.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_reverb_stereo_room10m_time2s_damp0_5.snap.wav"></audio>

### fx_delay_250ms_sine_440
Delay 250ms sine 440.  
Expression: `sine_hz::<f32>(440.0) >> delay(0.25)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_delay_250ms_sine_440.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_delay_250ms_sine_440.snap.wav"></audio>

### noise_white
White noise.  
Expression: `white()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_white.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_white.snap.wav"></audio>

### noise_pink
Pink noise.  
Expression: `pink::<f32>()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_pink.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_pink.snap.wav"></audio>

### noise_brown
Brown noise.  
Expression: `brown::<f32>()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_brown.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_brown.snap.wav"></audio>

### noise_mls
MLS noise.  
Expression: `mls()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_mls.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_mls.snap.wav"></audio>

---