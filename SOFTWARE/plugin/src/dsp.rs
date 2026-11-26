// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
//! M404 Hi-Lo Pass Equalizer (1968|1) signal path, free of any plugin framework (the Audio Tap uses this alone).
//!
//! From the M404 manual: ten cut-off frequencies (and OFF) for each end of the spectrum, 3 dB points,
//! 18 dB/octave; a third control picks the HI (high frequency cut) or LO (low frequency cut) section alone, or
//! both in its centre position; EQUALIZER ON switches the equalizer section in (lamp lit) or out.
//! The hardware's 35 dB preamplifier is left out: the plugin runs at unity, like the equalizer section itself.

use crate::filter::{Cut, Kind, MAX_CH};

/// LOW FREQUENCY CUT (high pass) positions; position 0 is OFF.
pub const LF_HZ: [f32; 11] = [0.0, 70.0, 100.0, 150.0, 250.0, 500.0, 1000.0, 2000.0, 3000.0, 4000.0, 5000.0];
/// HIGH FREQUENCY CUT (low pass) positions; position 10 is OFF.
pub const HF_HZ: [f32; 11] = [500.0, 1000.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 8000.0, 10000.0, 12000.0, 0.0];
pub const LF_OFF: usize = 0;
pub const HF_OFF: usize = 10;

/// The selector: which sections are in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Hi,
    Both,
    Lo,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub eq_on: bool,
    pub mode: Mode,
    /// HF CUT position 0 … 10 (10 = OFF)
    pub hf: usize,
    /// LF CUT position 0 … 10 (0 = OFF)
    pub lf: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { eq_on: true, mode: Mode::Both, hf: HF_OFF, lf: LF_OFF }
    }
}

pub struct M404 {
    rate: f32,
    s: Settings,
    lf: Cut,
    hf: Cut,
}

impl M404 {
    pub fn new(sample_rate: f32) -> Self {
        let mut m = M404 { rate: sample_rate, s: Settings::default(), lf: Cut::default(), hf: Cut::default() };
        m.update();
        m
    }
    pub fn set_sample_rate(&mut self, r: f32) {
        self.rate = r;
        self.reset();
        self.update();
    }
    pub fn reset(&mut self) {
        self.lf.reset();
        self.hf.reset();
    }
    pub fn settings(&self) -> Settings {
        self.s
    }
    pub fn set(&mut self, s: Settings) {
        let s = Settings { lf: s.lf.min(10), hf: s.hf.min(10), ..s };
        if s != self.s {
            let (lf_was, hf_was) = self.active();
            self.s = s;
            let (lf_now, hf_now) = self.active();
            if lf_now && !lf_was {
                self.lf.reset();
            }
            if hf_now && !hf_was {
                self.hf.reset();
            }
            self.update();
        }
    }
    /// (LF cut in, HF cut in) with the current switches.
    pub fn active(&self) -> (bool, bool) {
        let s = self.s;
        (
            s.eq_on && s.mode != Mode::Hi && s.lf != LF_OFF,
            s.eq_on && s.mode != Mode::Lo && s.hf != HF_OFF,
        )
    }
    fn update(&mut self) {
        if self.s.lf != LF_OFF {
            self.lf.set(Kind::HighPass, LF_HZ[self.s.lf], self.rate);
        }
        if self.s.hf != HF_OFF {
            self.hf.set(Kind::LowPass, HF_HZ[self.s.hf], self.rate);
        }
    }
    pub fn process(&mut self, chans: &mut [&mut [f32]]) {
        let (lf_in, hf_in) = self.active();
        if !lf_in && !hf_in {
            return;
        }
        for (ch, data) in chans.iter_mut().enumerate().take(MAX_CH) {
            for v in data.iter_mut() {
                let mut x = *v;
                if lf_in {
                    x = self.lf.run(ch, x);
                }
                if hf_in {
                    x = self.hf.run(ch, x);
                }
                *v = x;
            }
        }
        self.lf.ok();
        self.hf.ok();
    }
    pub fn response_db(&self, hz: f32) -> f32 {
        let (lf_in, hf_in) = self.active();
        (if lf_in { self.lf.response_db(hz, self.rate) } else { 0.0 }) + (if hf_in { self.hf.response_db(hz, self.rate) } else { 0.0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_positions_and_eq_off_are_flat() {
        let mut m = M404::new(48000.0);
        assert_eq!(m.active(), (false, false));
        m.set(Settings { eq_on: false, mode: Mode::Both, hf: 3, lf: 5 });
        assert_eq!(m.active(), (false, false));
        let mut a = [0.5f32, -0.25];
        m.process(&mut [&mut a[..]]);
        assert_eq!(a, [0.5, -0.25]);
    }

    #[test]
    fn the_selector_picks_the_sections() {
        let mut m = M404::new(48000.0);
        m.set(Settings { eq_on: true, mode: Mode::Hi, hf: 1, lf: 3 });
        assert_eq!(m.active(), (false, true));
        assert!((m.response_db(1000.0) + 3.0).abs() < 0.1);
        m.set(Settings { mode: Mode::Lo, ..m.settings() });
        assert_eq!(m.active(), (true, false));
        assert!((m.response_db(150.0) + 3.0).abs() < 0.1);
        m.set(Settings { mode: Mode::Both, ..m.settings() });
        assert_eq!(m.active(), (true, true));
    }

    #[test]
    fn every_position_is_its_3_db_point() {
        let mut m = M404::new(44100.0);
        for i in 1..10 {
            m.set(Settings { eq_on: true, mode: Mode::Lo, hf: HF_OFF, lf: i });
            assert!((m.response_db(LF_HZ[i]) + 3.0).abs() < 0.1);
            m.set(Settings { eq_on: true, mode: Mode::Hi, hf: i - 1, lf: LF_OFF });
            assert!((m.response_db(HF_HZ[i - 1]) + 3.0).abs() < 0.1);
        }
    }
}
