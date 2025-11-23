---
layout: no_sidebar
---
# utility_shaping_rate

Snapshot gallery for utility, shaping & rate-related processing.
Each entry:
- Brief WHY (representative intent)
- Core expression (concise form)
- SVG chart (2000 samples)
- 1 s WAV audio

[Back to main index](./index.md)

---

## Contents
- [util_clip_to_m0_5_p0_5_mul2_sine440](#util_clip_to_m0_5_p0_5_mul2_sine440)
- [util_shape_softcrush0_5_sine220](#util_shape_softcrush0_5_sine220)
- [util_map_abs_sine110](#util_map_abs_sine110)
- [util_oversample2x_sine440](#util_oversample2x_sine440)
- [util_resample_speed_ramp_0_5_to_1_5_sine440](#util_resample_speed_ramp_0_5_to_1_5_sine440)

---

### util_clip_to_m0_5_p0_5_mul2_sine440
Hard clipping after gain boost (illustrates limiting with visible flat tops).  
Expression: `(sine_hz::<f32>(440.0) * 2.0) >> clip_to(-0.5, 0.5)`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_clip_to_m0_5_p0_5_mul2_sine440.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_clip_to_m0_5_p0_5_mul2_sine440@audio.snap.wav"></audio>

### util_shape_softcrush0_5_sine220
Gentle nonlinear waveshaping (harmonic enrichment via SoftCrush).  
Expression: `sine_hz::<f32>(220.0) >> shape(SoftCrush(0.5))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_shape_softcrush0_5_sine220.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_shape_softcrush0_5_sine220@audio.snap.wav"></audio>

### util_map_abs_sine110
Absolute value mapping (rectification → folds negative half-cycle).  
Expression: `sine_hz::<f32>(110.0) >> map(|f: &Frame<f32, U1>| f[0].abs())`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_map_abs_sine110.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_map_abs_sine110@audio.snap.wav"></audio>

### util_oversample2x_sine440
Structural oversampling (2×) of a simple sine; demonstrates usage (alias-reduction context).  
Expression: `oversample(sine_hz::<f32>(440.0))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_oversample2x_sine440.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_oversample2x_sine440@audio.snap.wav"></audio>

### util_resample_speed_ramp_0_5_to_1_5_sine440
Dynamic resampling speed (time warp 0.5×→1.5× via ramp + lerp) producing pitch glide.  
Expression (speed input): `speed = ramp_hz::<f32>(0.25) >> map(|f| lerp(0.5f32, 1.5f32, f[0])); resample(sine_hz::<f32>(440.0))`  
![](https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_resample_speed_ramp_0_5_to_1_5_sine440.snap.svg)  
<audio controls src="https://github.com/anvlkv/insta-fun/raw/refs/heads/main/examples/snapshots/utility_shaping_rate__util_resample_speed_ramp_0_5_to_1_5_sine440@audio.snap.wav"></audio>

---
