---
layout: no_sidebar
---
# time_fx_and_noise

Snapshots of time-based effects, spatial processing, dynamics, resonators, delay, reverb, and noise generators. Each entry shows a terse description, expression, SVG chart, and WAV audio.

---

### fx_chorus
Mono chorus on 440 Hz sine.  
Expression: `sine_hz::<f32>(440.0) >> chorus(1, 0.015, 0.005, 0.2)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_chorus.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_chorus@audio.snap.wav"></audio>

### fx_flanger
Flanger 5–10 ms sweep, strong feedback, sine 440 Hz.  
Expression: `sine_hz::<f32>(440.0) >> flanger(0.9, 0.005, 0.010, |t| 0.0075 + 0.0025 * sin_hz(0.1, t))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_flanger.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_flanger@audio.snap.wav"></audio>

### fx_phaser
Phaser with feedback 0.5, sine 440 Hz; phase mod 0..1.  
Expression: `sine_hz::<f32>(440.0) >> phaser(0.5, |t| sin_hz(0.2, t) * 0.5 + 0.5)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_phaser.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_phaser@audio.snap.wav"></audio>

### spatial_pan_0_25_sine440
Equal‑power pan position 0.25 (toward left).  
Expression: `sine_hz::<f32>(440.0) >> pan(0.25)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__spatial_pan_0_25_sine440.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__spatial_pan_0_25_sine440@audio.snap.wav"></audio>

### spatial_rotate_pi4_gain0_8
Stereo rotate π/4 with gain 0.8 (220 & 330 Hz pair).  
Expression: `(sine_hz::<f32>(220.0) | sine_hz::<f32>(330.0)) >> rotate(std::f32::consts::FRAC_PI_4, 0.8)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__spatial_rotate_pi4_gain0_8.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__spatial_rotate_pi4_gain0_8@audio.snap.wav"></audio>

### meter_rms_sine440
RMS meter (100 ms smoothing) on 440 Hz sine (chart only).  
Expression: `sine_hz::<f32>(440.0) >> meter(Meter::Rms(0.1))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__meter_rms_sine440.snap.svg)  

### dyn_limiter_mul2_in
Limiter after 2× boosted sine input.  
Expression: `sine_hz::<f32>(440.0) * 2.0 >> limiter(0.01, 0.1)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__dyn_limiter_mul2_in.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__dyn_limiter_mul2_in@audio.snap.wav"></audio>

### fx_resonator_440hz_bw50
Bandpass resonator at 440 Hz (~50 Hz bandwidth) on 440 Hz sine.  
Expression: `sine_hz::<f32>(440.0) >> resonator_hz(440.0, 50.0)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_resonator_440hz_bw50.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_resonator_440hz_bw50@audio.snap.wav"></audio>

### fx_reverb_stereo_room10m_time2s_damp0_5
Stereo reverb (room 10 m, time 2 s, damping 0.5) on dual 220 Hz source.  
Expression: `(sine_hz::<f32>(220.0) | sine_hz::<f32>(220.0)) >> reverb_stereo(10.0, 2.0, 0.5)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_reverb_stereo_room10m_time2s_damp0_5.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_reverb_stereo_room10m_time2s_damp0_5@audio.snap.wav"></audio>

### reverb2_room15m_time2_5s_diff0_7_mod0_3_lp5k
Hybrid FDN reverb2 (room 15 m, 2.5 s, diff 0.7, mod 0.3, loop LP 5 kHz).  
Expression: `(sine_hz::<f32>(220.0) | sine_hz::<f32>(330.0)) >> reverb2_stereo(15.0, 2.5, 0.7, 0.3, lowpass_hz(5000.0, 0.7))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__reverb2_room15m_time2_5s_diff0_7_mod0_3_lp5k.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__reverb2_room15m_time2_5s_diff0_7_mod0_3_lp5k@audio.snap.wav"></audio>

### reverb3_time2_0s_diff0_7_lp6k
Allpass-loop reverb3 (time 2.0 s, diff 0.7, loop LP 6 kHz).  
Expression: `(sine_hz::<f32>(220.0) | sine_hz::<f32>(330.0)) >> reverb3_stereo(2.0, 0.7, lowpass_hz(6000.0, 0.7))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__reverb3_time2_0s_diff0_7_lp6k.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__reverb3_time2_0s_diff0_7_lp6k@audio.snap.wav"></audio>

### fx_delay_250ms_sine_440
Simple delay of 250 ms on sine 440 Hz.  
Expression: `sine_hz::<f32>(440.0) >> delay(0.25)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_delay_250ms_sine_440.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__fx_delay_250ms_sine_440@audio.snap.wav"></audio>

### delay_tap_linear_30ms_90ms_sine440
Single tapped linear delay (30–90 ms sweep range) driven by sine 440 Hz.  
Expression: `sine_hz::<f32>(440.0) >> tap_linear(0.03, 0.09)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__delay_tap_linear_30ms_90ms_sine440.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__delay_tap_linear_30ms_90ms_sine440@audio.snap.wav"></audio>

### delay_multitap_linear_3_10ms_50ms_sine330
Multitap linear delay (3 taps, 10–50 ms) on sine 330 Hz.  
Expression: `sine_hz::<f32>(330.0) >> multitap_linear::<U3>(0.01, 0.05)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__delay_multitap_linear_3_10ms_50ms_sine330.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__delay_multitap_linear_3_10ms_50ms_sine330@audio.snap.wav"></audio>

### feedback_lowpass_1k_q0_7_sine220
Feedback loop with lowpass (1 kHz, Q=0.7) on 220 Hz sine.  
Expression: `sine_hz::<f32>(220.0) >> feedback(lowpass_hz(1000.0, 0.7))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__feedback_lowpass_1k_q0_7_sine220.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__feedback_lowpass_1k_q0_7_sine220@audio.snap.wav"></audio>

### noise_white
White noise generator.  
Expression: `white()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_white.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_white@audio.snap.wav"></audio>

### noise_pink
Pink noise generator.  
Expression: `pink::<f32>()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_pink.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_pink@audio.snap.wav"></audio>

### noise_brown
Brown noise generator.  
Expression: `brown::<f32>()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_brown.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_brown@audio.snap.wav"></audio>

### noise_mls
MLS noise generator.  
Expression: `mls()`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_mls.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/time_fx_and_noise__noise_mls@audio.snap.wav"></audio>

---
