# insta-fun

`insta-fun` is a crate for snapshot testing `fundsp` `AudioUnit`s.

You can get it from [`crates.io`](https://crates.io/crates/insta-fun)

## Why this page?

I'm just learning audio programming and sound design. And for my own understanding of the `fundsp` material I provide this overview covering all of filters, oscillators and effects provided by `fundsp`. I'm happy if someone finds it useful too.

---

## Contents

### Categories
- Filters: [gallery](filters.md)
- Advanced Filters: [gallery](filters_advanced.md)
- Oscillators: [gallery](oscillators.md)
- Advanced Oscillators: [gallery](oscillators_advanced.md)
- Time FX & Noise: [gallery](time_fx_and_noise.md)

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
- Synth Threshold Pulses: [adv_synth_pulse_440hz_duty0_10](oscillators_advanced.md#adv_synth_pulse_440hz_duty0_10), [adv_synth_pulse_440hz_duty0_25](oscillators_advanced.md#adv_synth_pulse_440hz_duty0_25), [adv_synth_pulse_440hz_duty0_75](oscillators_advanced.md#adv_synth_pulse_440hz_duty0_75)  
- PWM: [adv_pwm_sine_440hz_lfo1hz](oscillators_advanced.md#adv_pwm_sine_440hz_lfo1hz)

### Time FX & Noise
- FX: [fx_chorus](time_fx_and_noise.md#fx_chorus), [fx_flanger](time_fx_and_noise.md#fx_flanger), [fx_phaser](time_fx_and_noise.md#fx_phaser), [dyn_limiter_mul2_in](time_fx_and_noise.md#dyn_limiter_mul2_in), [fx_resonator_440hz_bw50](time_fx_and_noise.md#fx_resonator_440hz_bw50), [fx_reverb_stereo_room10m_time2s_damp0_5](time_fx_and_noise.md#fx_reverb_stereo_room10m_time2s_damp0_5), [fx_delay_250ms_sine_440](time_fx_and_noise.md#fx_delay_250ms_sine_440)  
- Noise: [noise_white](time_fx_and_noise.md#noise_white), [noise_pink](time_fx_and_noise.md#noise_pink), [noise_brown](time_fx_and_noise.md#noise_brown), [noise_mls](time_fx_and_noise.md#noise_mls)

---

### Navigation Notes
Each detail page contains: SVG chart image, audio player, short expression, minimal description. Use links above to jump directly.

---

Enjoy exploring the sound palette!
