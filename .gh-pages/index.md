# insta-fun

`insta-fun` provides snapshot tests (SVG waveform + 1 s WAV audio) for representative `fundsp` `AudioUnit`s.

Get the crate on [`crates.io`](https://crates.io/crates/insta-fun)

## Why this page?

This is a curated learning gallery – not exhaustive API coverage. Each category shows one clear example per family (filters, oscillators, modulation/control, time & spatial FX, noise, utility/shaping/rate). The aim is quick visual/audible inspection while exploring audio programming and sound design. If it helps you too, great.

---

## Contents

### Categories
- Filters: [gallery](filters.md)
- Advanced Filters: [gallery](filters_advanced.md)
- Oscillators: [gallery](oscillators.md)
- Advanced Oscillators: [gallery](oscillators_advanced.md)
- Time FX & Noise: [gallery](time_fx_and_noise.md)
- Modulation & Control: [gallery](modulation_and_control.md)
- Utility, Shaping & Rate: [gallery](utility_shaping_rate.md)

---

### Filters
- [filter_svf_lowpass_1k_q0_707](filters.md#filter_svf_lowpass_1k_q0_707)
- [filter_svf_highpass_1k_q0_707](filters.md#filter_svf_highpass_1k_q0_707)
- [filter_svf_bandpass_1k_q1_0](filters.md#filter_svf_bandpass_1k_q1_0)
- [filter_svf_notch_1k_q1_0](filters.md#filter_svf_notch_1k_q1_0)
- [filter_peak_1k_q1_0](filters.md#filter_peak_1k_q1_0)
- [filter_bell_1k_q0_707_gain1_5](filters.md#filter_bell_1k_q0_707_gain1_5)
- [filter_lowshelf_500hz_q0_707_gain1_5](filters.md#filter_lowshelf_500hz_q0_707_gain1_5)
- [filter_highshelf_2k_q0_707_gain1_5](filters.md#filter_highshelf_2k_q0_707_gain1_5)
- [filter_moog_lowpass_1k_q0_5](filters.md#filter_moog_lowpass_1k_q0_5)
- [filter_lowrez_lowpass_1k_q1_0](filters.md#filter_lowrez_lowpass_1k_q1_0)
- [filter_bandrez_bandpass_1k_q3_0](filters.md#filter_bandrez_bandpass_1k_q3_0)
- [filter_lowpole_800hz](filters.md#filter_lowpole_800hz)
- [filter_highpole_300hz](filters.md#filter_highpole_300hz)
- [filter_morph_lowpass_1k_q0_8](filters.md#filter_morph_lowpass_1k_q0_8)
- [filter_dcblock_10hz](filters.md#filter_dcblock_10hz)
- [filter_allpass_1k_q0_707](filters.md#filter_allpass_1k_q0_707)
- [filter_butter_lowpass_1k](filters.md#filter_butter_lowpass_1k)
- [filter_fir3_gain0_50_white](filters.md#filter_fir3_gain0_50_white)
- [filter_pinkpass_white](filters.md#filter_pinkpass_white)
- [filter_follow_attack100ms](filters.md#filter_follow_attack100ms)

### Advanced Filters
- Dirty: [adv_dirty_lowpass_1000hz_q0_70_shape0_50](filters_advanced.md#adv_dirty_lowpass_1000hz_q0_70_shape0_50), [adv_dirty_highpass_1000hz_q0_70_shape0_50](filters_advanced.md#adv_dirty_highpass_1000hz_q0_70_shape0_50), [adv_dirty_resonator_1000hz_q5_00_shape0_80](filters_advanced.md#adv_dirty_resonator_1000hz_q5_00_shape0_80), [adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50](filters_advanced.md#adv_dirty_bell_1000hz_q0_70_gain1_50_shape0_50)  
- Feedback: [adv_fb_lowpass_1000hz_q0_70_shape0_50](filters_advanced.md#adv_fb_lowpass_1000hz_q0_70_shape0_50), [adv_fb_highpass_1000hz_q0_70_shape0_50](filters_advanced.md#adv_fb_highpass_1000hz_q0_70_shape0_50), [adv_fb_resonator_1000hz_q6_00_shape0_80](filters_advanced.md#adv_fb_resonator_1000hz_q6_00_shape0_80), [adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50](filters_advanced.md#adv_fb_bell_1000hz_q0_70_gain1_50_shape0_50)  
- Morph: [adv_morph_1000hz_q0_80_m_lp](filters_advanced.md#adv_morph_1000hz_q0_80_m_lp), [adv_morph_1000hz_q0_80_m_lp_mid](filters_advanced.md#adv_morph_1000hz_q0_80_m_lp_mid), [adv_morph_1000hz_q0_80_m_peak](filters_advanced.md#adv_morph_1000hz_q0_80_m_peak), [adv_morph_1000hz_q0_80_m_hp_mid](filters_advanced.md#adv_morph_1000hz_q0_80_m_hp_mid), [adv_morph_1000hz_q0_80_m_hp](filters_advanced.md#adv_morph_1000hz_q0_80_m_hp)

### Oscillators
- [osc_sine_440hz](oscillators.md#osc_sine_440hz)
- [osc_saw_440hz](oscillators.md#osc_saw_440hz)
- [osc_square_440hz](oscillators.md#osc_square_440hz)
- [osc_triangle_440hz](oscillators.md#osc_triangle_440hz)
- [osc_soft_saw_440hz](oscillators.md#osc_soft_saw_440hz)
- [osc_organ_440hz](oscillators.md#osc_organ_440hz)
- [osc_hammond_440hz](oscillators.md#osc_hammond_440hz)
- [osc_ramp_1hz](oscillators.md#osc_ramp_1hz)

### Advanced Oscillators
- DSF Saw: [adv_dsf_saw_440hz_rough0_00](oscillators_advanced.md#adv_dsf_saw_440hz_rough0_00), [adv_dsf_saw_440hz_rough0_50](oscillators_advanced.md#adv_dsf_saw_440hz_rough0_50), [adv_dsf_saw_440hz_rough1_00](oscillators_advanced.md#adv_dsf_saw_440hz_rough1_00), [adv_dsf_saw_440hz_rough0_99](oscillators_advanced.md#adv_dsf_saw_440hz_rough0_99)  
- DSF Square: [adv_dsf_square_440hz_rough0_00](oscillators_advanced.md#adv_dsf_square_440hz_rough0_00), [adv_dsf_square_440hz_rough0_50](oscillators_advanced.md#adv_dsf_square_440hz_rough0_50), [adv_dsf_square_440hz_rough1_00](oscillators_advanced.md#adv_dsf_square_440hz_rough1_00), [adv_dsf_square_440hz_rough0_99](oscillators_advanced.md#adv_dsf_square_440hz_rough0_99)  
- Pulse Widths: [adv_pulse_440hz_width0_10](oscillators_advanced.md#adv_pulse_440hz_width0_10), [adv_pulse_440hz_width0_25](oscillators_advanced.md#adv_pulse_440hz_width0_25), [adv_pulse_440hz_width0_50](oscillators_advanced.md#adv_pulse_440hz_width0_50), [adv_pulse_440hz_width0_75](oscillators_advanced.md#adv_pulse_440hz_width0_75)  
- Physical & Chaotic: [osc_pluck_110hz_damp0_50](oscillators_advanced.md#osc_pluck_110hz_damp0_50), [osc_lorenz](oscillators_advanced.md#osc_lorenz), [osc_rossler](oscillators_advanced.md#osc_rossler)
 
### Time FX & Noise
- FX & Spatial/Dynamics/Delay: [fx_chorus](time_fx_and_noise.md#fx_chorus), [fx_flanger](time_fx_and_noise.md#fx_flanger), [fx_phaser](time_fx_and_noise.md#fx_phaser), [dyn_limiter_mul2_in](time_fx_and_noise.md#dyn_limiter_mul2_in), [fx_resonator_440hz_bw50](time_fx_and_noise.md#fx_resonator_440hz_bw50), [fx_reverb_stereo_room10m_time2s_damp0_5](time_fx_and_noise.md#fx_reverb_stereo_room10m_time2s_damp0_5), [fx_delay_250ms_sine_440](time_fx_and_noise.md#fx_delay_250ms_sine_440), [spatial_pan_0_25_sine440](time_fx_and_noise.md#spatial_pan_0_25_sine440), [spatial_rotate_pi4_gain0_8](time_fx_and_noise.md#spatial_rotate_pi4_gain0_8), [meter_rms_sine440](time_fx_and_noise.md#meter_rms_sine440), [delay_tap_linear_30ms_90ms_sine440](time_fx_and_noise.md#delay_tap_linear_30ms_90ms_sine440), [delay_multitap_linear_3_10ms_50ms_sine330](time_fx_and_noise.md#delay_multitap_linear_3_10ms_50ms_sine330), [feedback_lowpass_1k_q0_7_sine220](time_fx_and_noise.md#feedback_lowpass_1k_q0_7_sine220), [reverb2_room15m_time2_5s_diff0_7_mod0_3_lp5k](time_fx_and_noise.md#reverb2_room15m_time2_5s_diff0_7_mod0_3_lp5k), [reverb3_time2_0s_diff0_7_lp6k](time_fx_and_noise.md#reverb3_time2_0s_diff0_7_lp6k)  
- Noise: [noise_white](time_fx_and_noise.md#noise_white), [noise_pink](time_fx_and_noise.md#noise_pink), [noise_brown](time_fx_and_noise.md#noise_brown), [noise_mls](time_fx_and_noise.md#noise_mls)
 
### Modulation & Control
- [mod_adsr_a10ms_d100ms_s0_70_r200ms](modulation_and_control.md#mod_adsr_a10ms_d100ms_s0_70_r200ms)
- [mod_lfo](modulation_and_control.md#mod_lfo)
- [mod_sample_hold_5hz_var0_2](modulation_and_control.md#mod_sample_hold_5hz_var0_2)
- [mod_follow_attack100ms](modulation_and_control.md#mod_follow_attack100ms)

### Utility, Shaping & Rate
- [util_clip_to_m0_5_p0_5_mul2_sine440](utility_shaping_rate.md#util_clip_to_m0_5_p0_5_mul2_sine440)
- [util_shape_softcrush0_5_sine220](utility_shaping_rate.md#util_shape_softcrush0_5_sine220)
- [util_map_abs_sine110](utility_shaping_rate.md#util_map_abs_sine110)
- [util_oversample2x_sine440](utility_shaping_rate.md#util_oversample2x_sine440)
- [util_resample_speed_ramp_0_5_to_1_5_sine440](utility_shaping_rate.md#util_resample_speed_ramp_0_5_to_1_5_sine440)
 
---
 
### Navigation Notes
Each detail page contains: SVG chart image, audio player, short expression, minimal description. Use links above to jump directly.
 
---
 
Enjoy exploring the sound palette!
