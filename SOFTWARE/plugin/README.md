# M404 Hi-Lo Pass Equalizer: plugin

The Ward-Beck M404 Hi-Lo Pass Equalizer (1968|1) as a **VST3 / CLAP plugin** with its M404A escutcheon, and as
the DSP the **Ward-Beck Audio Tap** can run as an insert.

**Download** the VST3 / CLAP for Windows, macOS and Linux from this repo's [releases](https://github.com/WardBeckSystems/MODULES.FILTERS.M404/releases/latest). All Ward-Beck plugins: [Ward-Beck Plugin Core](https://github.com/WardBeckSystems/Ward-Beck-Plugin-Core#plugins).

| Control | Positions | What it does |
|---|---|---|
| HF CUT | .5, 1, 2, 3, 4, 5, 6, 8, 10, 12 kHz, OFF | low pass, 18 dB/octave, 3 dB points |
| HI / HI&LO / LO | 3 | the HF cut alone, both, or the LF cut alone |
| LF CUT | OFF, 70, 100, 150, 250, 500 Hz, 1, 2, 3, 4, 5 kHz | high pass, 18 dB/octave, 3 dB points |
| EQUALIZER ON + lamp | in / out | the equalizer in or out (lamp lit when in) |

The hardware's 35 dB mic preamplifier is left out: the plugin runs at unity, as the equalizer section does.

```bash
cargo xtask bundle wbs-m404 --release        # target/bundled/wbs-m404.vst3 and wbs-m404.clap
cargo test --no-default-features
python3 tools/build_plugins.py M404          # (from WBSps.ca) build, deploy to ~/.vst3 ~/.clap, load in REAPER
```

**Sources.** The M404 manual (general description, technical data); outline drawing 404-04-010 (M404A, Feb 1970)
for the escutcheon.

Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
