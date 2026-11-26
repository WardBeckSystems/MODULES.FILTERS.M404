// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
//! The M404A escutcheon as drawn on 404-04-010 (Feb 1970): 1.58 × 6.625 in, black anodised, photo-etched.
//! Upright as it sits beside a straight-line fader: WBS M404A at the top, HF CUT (kHz), the HI / HI & LO / LO
//! selector, LF CUT (Hz × 100), then EQUALIZER ON switch and lamp. Positions measured off the drawing in
//! panel units (159 across × 667 down).

use crate::plugin::Params404;
use nih_plug::prelude::*;
use std::sync::Arc;
use wbs_panel::egui::{self, Frame};
use wbs_panel::nih_plug_egui::{create_egui_editor, EguiState};
use wbs_panel::{ink, Button, Cap, Face, Led, Size};

const WIDTH: f32 = 159.0;
const LEN: f32 = 667.0;

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(214, 860)
}

pub fn create(params: Arc<Params404>) -> Option<Box<dyn Editor>> {
    create_egui_editor(params.editor_state.clone(), (), |_, _| {}, move |ctx, setter, _| {
        egui::CentralPanel::default().frame(Frame::NONE.fill(ink::RACK)).show(ctx, |ui| {
            let f = Face::fit_size(ui.max_rect().shrink(8.0), WIDTH, LEN);
            f.panel(ui.painter(), "M404A");
            draw(ui, &f, setter, &params);
        });
    })
}

fn legends(l: &'static [&'static str; 11]) -> [(f32, &'static str); 11] {
    std::array::from_fn(|i| (i as f32 / 10.0, l[i]))
}

const HF: [&str; 11] = [".5", "1", "2", "3", "4", "5", "6", "8", "10", "12", "OFF"];
const LF: [&str; 11] = ["OFF", ".7", "1", "1.5", "2.5", "5", "10", "20", "30", "40", "50"];
const SEL: [(f32, &str); 3] = [(0.0, "HI"), (0.5, "HI&LO"), (1.0, "LO")];

fn draw(ui: &mut egui::Ui, f: &Face, s: &ParamSetter, p: &Params404) {
    let at = |across: f32, down: f32| f.at(across, down);
    let label = |ui: &egui::Ui, across: f32, down: f32, t: &str, size: f32| {
        let (x, y) = at(across, down);
        f.text(ui.painter(), x, y, t, size, ink::SUB);
    };

    let (x, y) = at(84.0, 125.0);
    f.knob(ui, s, &p.hf, x, y, Size::Small, Cap::Black, &legends(&HF), 11);
    label(ui, 84.0, 178.0, "HF CUT kHz", 8.0);

    let (x, y) = at(84.0, 285.0);
    f.knob(ui, s, &p.mode, x, y, Size::Small, Cap::Black, &SEL, 3);

    let (x, y) = at(84.0, 448.0);
    f.knob(ui, s, &p.lf, x, y, Size::Small, Cap::Black, &legends(&LF), 11);
    label(ui, 84.0, 501.0, "LF CUT Hz×100", 8.0);

    let (x, y) = at(118.0, 575.0);
    f.push(ui, s, &p.eq_on, x, y, Size::Small, Button::White);
    let (x, y) = at(42.0, 575.0);
    f.led(ui.painter(), x, y, Led::Amber, p.eq_on.value());
    label(ui, 80.0, 612.0, "EQUALIZER ON", 7.5);
}
