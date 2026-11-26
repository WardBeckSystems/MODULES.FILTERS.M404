// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
//! Ward-Beck M404 Hi-Lo Pass Equalizer (1968).
//!
//! `dsp` is the signal path on its own (used by the Ward-Beck Audio Tap). With the default feature `vst`
//! this crate is also a full VST3 / CLAP plugin whose editor is the M404A escutcheon (drawing 404-04-010).

pub mod dsp;
pub mod filter;

#[cfg(feature = "vst")]
mod editor;
#[cfg(feature = "vst")]
pub use plugin::*;

#[cfg(feature = "vst")]
mod plugin {
    use crate::dsp::{Mode, Settings, HF_HZ, LF_HZ, M404};
    use nih_plug::prelude::*;
    use std::sync::Arc;

    fn hz(v: f32) -> String {
        if v == 0.0 {
            "OFF".into()
        } else if v >= 1000.0 {
            format!("{} kHz", v / 1000.0)
        } else {
            format!("{v} Hz")
        }
    }

    #[derive(Enum, Debug, PartialEq, Clone, Copy)]
    pub enum Selector {
        #[name = "HI"]
        Hi,
        #[name = "HI & LO"]
        Both,
        #[name = "LO"]
        Lo,
    }

    #[derive(Params)]
    pub struct Params404 {
        #[persist = "editor-state"]
        pub editor_state: Arc<wbs_panel::nih_plug_egui::EguiState>,
        #[id = "eq_on"]
        pub eq_on: BoolParam,
        #[id = "mode"]
        pub mode: EnumParam<Selector>,
        #[id = "hf"]
        pub hf: IntParam,
        #[id = "lf"]
        pub lf: IntParam,
    }

    impl Default for Params404 {
        fn default() -> Self {
            let d = Settings::default();
            Params404 {
                editor_state: crate::editor::default_state(),
                eq_on: BoolParam::new("EQUALIZER ON", d.eq_on),
                mode: EnumParam::new("HI / LO", Selector::Both),
                hf: IntParam::new("HF CUT", d.hf as i32, IntRange::Linear { min: 0, max: 10 })
                    .with_value_to_string(Arc::new(|i| hz(HF_HZ[i.clamp(0, 10) as usize]))),
                lf: IntParam::new("LF CUT", d.lf as i32, IntRange::Linear { min: 0, max: 10 })
                    .with_value_to_string(Arc::new(|i| hz(LF_HZ[i.clamp(0, 10) as usize]))),
            }
        }
    }

    impl Params404 {
        pub fn settings(&self) -> Settings {
            Settings {
                eq_on: self.eq_on.value(),
                mode: match self.mode.value() {
                    Selector::Hi => Mode::Hi,
                    Selector::Both => Mode::Both,
                    Selector::Lo => Mode::Lo,
                },
                hf: self.hf.value() as usize,
                lf: self.lf.value() as usize,
            }
        }
    }

    pub struct Plugin {
        params: Arc<Params404>,
        dsp: M404,
    }

    impl Default for Plugin {
        fn default() -> Self {
            Plugin { params: Arc::new(Params404::default()), dsp: M404::new(48000.0) }
        }
    }

    impl nih_plug::prelude::Plugin for Plugin {
        const NAME: &'static str = "WBS M404 Hi-Lo Pass Equalizer";
        const VENDOR: &'static str = "Ward-Beck Systems";
        const URL: &'static str = "https://ward-beck.ca";
        const EMAIL: &'static str = "info@ward-beck.ca";
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");

        const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
            AudioIOLayout { main_input_channels: NonZeroU32::new(2), main_output_channels: NonZeroU32::new(2), ..AudioIOLayout::const_default() },
            AudioIOLayout { main_input_channels: NonZeroU32::new(1), main_output_channels: NonZeroU32::new(1), ..AudioIOLayout::const_default() },
        ];
        const SAMPLE_ACCURATE_AUTOMATION: bool = false;
        type SysExMessage = ();
        type BackgroundTask = ();

        fn params(&self) -> Arc<dyn Params> {
            self.params.clone()
        }

        fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
            crate::editor::create(self.params.clone())
        }

        fn initialize(&mut self, _layout: &AudioIOLayout, config: &BufferConfig, _context: &mut impl InitContext<Self>) -> bool {
            self.dsp.set_sample_rate(config.sample_rate);
            true
        }

        fn reset(&mut self) {
            self.dsp.reset();
        }

        fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, _context: &mut impl ProcessContext<Self>) -> ProcessStatus {
            self.dsp.set(self.params.settings());
            self.dsp.process(buffer.as_slice());
            ProcessStatus::Normal
        }
    }

    impl ClapPlugin for Plugin {
        const CLAP_ID: &'static str = "ca.ward-beck.m404";
        const CLAP_DESCRIPTION: Option<&'static str> = Some("Ward-Beck M404 Hi-Lo Pass Equalizer (1968): 10-step LF and HF cuts, 18 dB/octave");
        const CLAP_MANUAL_URL: Option<&'static str> = Some("https://wardbecksystems.github.io/WBSps.ca/PAGES/M404.html");
        const CLAP_SUPPORT_URL: Option<&'static str> = None;
        const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo, ClapFeature::Mono, ClapFeature::Filter];
    }

    impl Vst3Plugin for Plugin {
        const VST3_CLASS_ID: [u8; 16] = *b"WBS-M404-HiLoEq1";
        const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::Filter, Vst3SubCategory::Eq];
    }

    nih_export_clap!(Plugin);
    nih_export_vst3!(Plugin);
}
