use crate::i18n as t;
use ::i18n::t as tr;
use egui_sc::egui_charts::{
    self, Axis, ChartTheme, ChartView, ChartWidget, Distribution, Harmony,
    Series, ThemeMode, XyData,
};
use egui_sc::egui_components::spacing::Spacing;
use egui_sc::egui_components::{
    ShadcnTheme,
    button::{Button, ButtonVariant},
    card::{Card, card_header},
    carousel::Carousel,
    command::{Command, CommandGroup, CommandItem},
    resizable::{Resizable, ResizeDir},
    switch::Switch,
};
use std::sync::OnceLock;

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_carousel(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(12);
        let subtitle = tr!(t::CarouselSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::CarouselSec::HItems).as_ref(), None);
            Carousel::new("demo_carousel", 5).height(180.0).show(
                ui,
                |idx, ui| {
                    let theme = ShadcnTheme::get(ui.ctx());
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(ui.available_width(), 160.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(
                        rect,
                        egui::CornerRadius::same(8),
                        theme.muted,
                    );
                    let n = idx + 1;
                    let slide_label = tr!(t::CarouselSec::Slide, n = n);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        slide_label.as_ref(),
                        egui::FontId::new(24.0, egui::FontFamily::Proportional),
                        theme.muted_foreground,
                    );
                },
            );
        });
    }

    /// Build an `egui_charts` theme that tracks the active Shadcn theme: the
    /// palette is derived from the Shadcn `primary` color (so picking a new
    /// primary recolors the charts automatically) and the canvas background is
    /// forced transparent so charts blend into their host `Card`.
    fn chart_theme(ui: &egui::Ui, series: usize) -> ChartTheme {
        let theme = ShadcnTheme::get(ui.ctx());
        let mode = if theme.dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        let mut ct = ChartTheme::from_primary(
            theme.primary,
            mode,
            Harmony::Analogous,
            series.max(1),
            Distribution::Even,
        );
        ct.background = egui::Color32::TRANSPARENT;
        // Drop the alternating split-area bands — keep only the grid lines.
        ct.split_area = [egui::Color32::TRANSPARENT; 2];
        ct.text = theme.foreground;
        ct.text_dim = theme.muted_foreground;
        ct.axis_line = theme.border;
        ct.grid_line = ShadcnTheme::with_alpha(theme.muted_foreground, 40);
        ct
    }

    pub(in crate::app) fn section_chart(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(13);
        let subtitle = tr!(t::ChartSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let m1 = egui_sc::egui_components::i18n::month_short(1);
        let m2 = egui_sc::egui_components::i18n::month_short(2);
        let m3 = egui_sc::egui_components::i18n::month_short(3);
        let m4 = egui_sc::egui_components::i18n::month_short(4);
        let m5 = egui_sc::egui_components::i18n::month_short(5);
        let m6 = egui_sc::egui_components::i18n::month_short(6);
        let months = [
            m1.as_ref(),
            m2.as_ref(),
            m3.as_ref(),
            m4.as_ref(),
            m5.as_ref(),
            m6.as_ref(),
        ];
        let desktop = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
        let mobile = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];

        // ── Bar chart (Desktop + Mobile) ──────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ChartSec::HBar).as_ref(), None);
            let desktop_lbl = tr!(t::ChartSec::Desktop);
            let mobile_lbl = tr!(t::ChartSec::Mobile);
            let chart = egui_charts::Chart::new()
                .x_axis(Axis::category(months))
                .y_axis(Axis::value())
                .series(Series::bar(desktop_lbl.as_ref()).data(desktop))
                .series(Series::bar(mobile_lbl.as_ref()).data(mobile));
            let w = ui.available_width();
            ChartWidget::new(&chart)
                .id("demo_bar_chart")
                .theme(Self::chart_theme(ui, 2))
                .min_size(egui::vec2(w, 240.0))
                .show(ui);
        });

        Spacing::Lg.show(ui);

        // ── Line chart (Desktop) ──────────────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ChartSec::HLine).as_ref(), None);
            let desktop_lbl = tr!(t::ChartSec::Desktop);
            let chart = egui_charts::Chart::new()
                .x_axis(Axis::category(months))
                .y_axis(Axis::value())
                .series(
                    Series::line(desktop_lbl.as_ref())
                        .data(desktop)
                        .smooth(true),
                );
            let w = ui.available_width();
            ChartWidget::new(&chart)
                .id("demo_line_chart")
                .theme(Self::chart_theme(ui, 1))
                .min_size(egui::vec2(w, 240.0))
                .show(ui);
        });

        Spacing::Lg.show(ui);
        self.chart_live_card(ui);
        Spacing::Lg.show(ui);
        Self::chart_bode_card(ui);
    }

    /// Streaming-style time series: three channels with fixed, token-derived
    /// colours, an opt-in interactive view, a "follow latest 10 s" mode that
    /// drives the x range from outside, and a reset button.
    fn chart_live_card(&mut self, ui: &mut egui::Ui) {
        const ID: &str = "demo_live_chart";
        const WINDOW_S: f64 = 10.0;
        Card::new().show(ui, |ui| {
            let hint = tr!(t::ChartSec::LiveHint);
            card_header(
                ui,
                tr!(t::ChartSec::HLive).as_ref(),
                Some(hint.as_ref()),
            );

            let follow_lbl = tr!(t::ChartSec::Follow);
            let reset_lbl = tr!(t::ChartSec::ResetView);
            ui.horizontal(|ui| {
                Switch::new(&mut self.chart_follow)
                    .label(follow_lbl.as_ref())
                    .show(ui);
                if Button::new(reset_lbl.as_ref())
                    .variant(ButtonVariant::Outline)
                    .show(ui)
                    .clicked()
                {
                    self.chart_follow = false;
                    ChartView::reset(ui.ctx(), ID);
                }
            });
            Spacing::Sm.show(ui);

            // Replay the recording: the playhead walks through it in real
            // time, and "follow" pins the x range to the latest window. The
            // y range stays whatever the user zoomed to (or auto-fits to the
            // visible window).
            if self.chart_follow {
                let now = ui.input(|i| i.time);
                let head =
                    WINDOW_S + (now % (LIVE_SECONDS - WINDOW_S)).max(0.0);
                ChartView::update(ui.ctx(), ID, |v| {
                    v.x = Some((head - WINDOW_S, head));
                });
                ui.ctx().request_repaint();
            }

            let theme = ShadcnTheme::get(ui.ctx());
            let [p, temp, valve] = live_channels().clone();
            let chart = egui_charts::Chart::new()
                .x_axis(Axis::value().name(tr!(t::ChartSec::Time).as_ref()))
                .y_axis(Axis::value().name(tr!(t::ChartSec::Value).as_ref()))
                .series(
                    Series::xy_line(tr!(t::ChartSec::Pressure).as_ref())
                        .data(p)
                        .color(theme.primary),
                )
                .series(
                    Series::xy_line(tr!(t::ChartSec::Temperature).as_ref())
                        .data(temp)
                        .color(theme.destructive),
                )
                .series(
                    Series::xy_line(tr!(t::ChartSec::Valve).as_ref())
                        .data(valve)
                        .color(theme.success)
                        .dashed(),
                );
            let w = ui.available_width();
            ChartWidget::new(&chart)
                .id(ID)
                .interactive(true)
                .theme(Self::chart_theme(ui, 3))
                .min_size(egui::vec2(w, 300.0))
                .show(ui);
        });
    }

    /// Bode plot: magnitude over phase on a log frequency axis. The two
    /// charts share one x view — zooming either one zooms both.
    fn chart_bode_card(ui: &mut egui::Ui) {
        const MAG: &str = "demo_bode_mag";
        const PHASE: &str = "demo_bode_phase";
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ChartSec::HBode).as_ref(), None);
            let (plant, model) = bode_points();
            let plant_lbl = tr!(t::ChartSec::Plant);
            let model_lbl = tr!(t::ChartSec::Model);
            let freq = tr!(t::ChartSec::Frequency);
            let theme = ShadcnTheme::get(ui.ctx());

            let chart = |y_name: &str, pick: fn(&(f64, f64, f64)) -> f64| {
                egui_charts::Chart::new()
                    .x_axis(Axis::log().name(freq.as_ref()))
                    .y_axis(Axis::value().name(y_name))
                    .series(
                        Series::xy_line(plant_lbl.as_ref())
                            .points(plant.iter().map(|p| (p.0, pick(p))))
                            .color(theme.primary),
                    )
                    .series(
                        Series::xy_line(model_lbl.as_ref())
                            .points(model.iter().map(|p| (p.0, pick(p))))
                            .color(theme.destructive)
                            .dashed(),
                    )
            };
            let mag = chart(tr!(t::ChartSec::Magnitude).as_ref(), |p| p.1);
            let phase = chart(tr!(t::ChartSec::Phase).as_ref(), |p| p.2);

            let w = ui.available_width();
            let show = |ui: &mut egui::Ui, c: &egui_charts::Chart, id: &str| {
                ChartWidget::new(c)
                    .id(egui::Id::new(id))
                    .interactive(true)
                    .theme(Self::chart_theme(ui, 2))
                    .min_size(egui::vec2(w, 220.0))
                    .show(ui);
            };

            // Link the x views: whichever chart the user zoomed last wins.
            let ctx = ui.ctx().clone();
            let before = ChartView::load(&ctx, MAG).x;
            show(ui, &mag, MAG);
            let after_mag = ChartView::load(&ctx, MAG).x;
            if after_mag != before {
                ChartView::update(&ctx, PHASE, |v| v.x = after_mag);
            }
            let phase_before = ChartView::load(&ctx, PHASE).x;
            show(ui, &phase, PHASE);
            let after_phase = ChartView::load(&ctx, PHASE).x;
            if after_phase != phase_before {
                ChartView::update(&ctx, MAG, |v| v.x = after_phase);
                ctx.request_repaint();
            }
        });
    }

    pub(in crate::app) fn section_command(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(17);
        let subtitle = tr!(t::CommandSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let desc = tr!(t::CommandSec::HPaletteDesc);
            card_header(
                ui,
                tr!(t::CommandSec::HPalette).as_ref(),
                Some(desc.as_ref()),
            );
            if Button::new(tr!(t::CommandSec::Open).as_ref())
                .variant(ButtonVariant::Outline)
                .show(ui)
                .clicked()
            {
                self.command_open = true;
            }
        });

        let grp_sugg = tr!(t::CommandSec::GrpSuggestions);
        let grp_set = tr!(t::CommandSec::GrpSettings);
        let i_cal = tr!(t::CommandSec::ItemCalendar);
        let i_emoji = tr!(t::CommandSec::ItemEmoji);
        let i_calc = tr!(t::CommandSec::ItemCalculator);
        let i_prof = tr!(t::CommandSec::ItemProfile);
        let i_bill = tr!(t::CommandSec::ItemBilling);
        let i_set = tr!(t::CommandSec::ItemSettings);
        let placeholder = tr!(t::CommandSec::Placeholder);

        let suggestion_items = [
            CommandItem {
                label: i_cal.as_ref(),
                description: None,
                shortcut: None,
                icon: None,
            },
            CommandItem {
                label: i_emoji.as_ref(),
                description: None,
                shortcut: None,
                icon: None,
            },
            CommandItem {
                label: i_calc.as_ref(),
                description: None,
                shortcut: None,
                icon: None,
            },
        ];
        let settings_items = [
            CommandItem {
                label: i_prof.as_ref(),
                description: Some("⌘P"),
                shortcut: None,
                icon: None,
            },
            CommandItem {
                label: i_bill.as_ref(),
                description: None,
                shortcut: None,
                icon: None,
            },
            CommandItem {
                label: i_set.as_ref(),
                description: Some("⌘S"),
                shortcut: None,
                icon: None,
            },
        ];
        let groups = [
            CommandGroup {
                heading: Some(grp_sugg.as_ref()),
                items: &suggestion_items,
            },
            CommandGroup {
                heading: Some(grp_set.as_ref()),
                items: &settings_items,
            },
        ];

        let ctx = ui.ctx().clone();
        Command::new("demo_command", &groups, &mut self.command_open)
            .placeholder(placeholder.as_ref())
            .show(&ctx);
    }

    pub(in crate::app) fn section_resizable(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(34);
        let subtitle = tr!(t::ResizableSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().padding(0.0).show(ui, |ui| {
            Resizable::new("demo_resizable_h")
                .dir(ResizeDir::Horizontal)
                .initial_split(0.33)
                .height(220.0)
                .show(
                    ui,
                    |ui| {
                        let theme = ShadcnTheme::get(ui.ctx());
                        let rect = ui.available_rect_before_wrap();
                        let one = tr!(t::ResizableSec::One);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            one.as_ref(),
                            egui::FontId::new(
                                13.0,
                                egui::FontFamily::Proportional,
                            ),
                            theme.muted_foreground,
                        );
                        let _ = ui.allocate_exact_size(
                            rect.size(),
                            egui::Sense::hover(),
                        );
                    },
                    |ui| {
                        Resizable::new("demo_resizable_v")
                            .dir(ResizeDir::Vertical)
                            .initial_split(0.5)
                            .show(
                                ui,
                                |ui| {
                                    let theme = ShadcnTheme::get(ui.ctx());
                                    let rect = ui.available_rect_before_wrap();
                                    let two = tr!(t::ResizableSec::Two);
                                    ui.painter().text(
                                        rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        two.as_ref(),
                                        egui::FontId::new(
                                            13.0,
                                            egui::FontFamily::Proportional,
                                        ),
                                        theme.muted_foreground,
                                    );
                                    let _ = ui.allocate_exact_size(
                                        rect.size(),
                                        egui::Sense::hover(),
                                    );
                                },
                                |ui| {
                                    let theme = ShadcnTheme::get(ui.ctx());
                                    let rect = ui.available_rect_before_wrap();
                                    let three = tr!(t::ResizableSec::Three);
                                    ui.painter().text(
                                        rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        three.as_ref(),
                                        egui::FontId::new(
                                            13.0,
                                            egui::FontFamily::Proportional,
                                        ),
                                        theme.muted_foreground,
                                    );
                                    let _ = ui.allocate_exact_size(
                                        rect.size(),
                                        egui::Sense::hover(),
                                    );
                                },
                            );
                    },
                );
        });
    }
}

/// Length of the demo recording, in seconds (1 kHz → 100k samples/channel).
const LIVE_SECONDS: f64 = 100.0;

/// Deterministic pseudo-noise in [-0.5, 0.5) (xorshift).
fn noise(state: &mut u64) -> f64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
}

/// Three simulated plant channels, generated once. Every frame's `Chart`
/// clones the `XyData` — an `Arc` bump, the samples are never copied.
fn live_channels() -> &'static [XyData; 3] {
    static CHANNELS: OnceLock<[XyData; 3]> = OnceLock::new();
    CHANNELS.get_or_init(|| {
        let n = (LIVE_SECONDS * 1_000.0) as usize;
        let mut seed = 0x2545_f491_4f6c_dd1d_u64;
        let (mut p, mut temp) = (4.0_f64, 150.0_f64);
        let mut ch: [Vec<[f64; 2]>; 3] =
            std::array::from_fn(|_| Vec::with_capacity(n));
        for i in 0..n {
            let t = i as f64 / 1_000.0;
            let v = if ((t / 20.0) as usize).is_multiple_of(2) {
                20.0
            } else {
                80.0
            };
            p += (4.0 + v * 0.05 - p) / 3_000.0;
            temp += (140.0 + v * 0.6 - temp) / 8_000.0;
            ch[0].push([t, p + 0.03 * noise(&mut seed)]);
            ch[1].push([t, temp / 30.0 + 0.01 * noise(&mut seed)]);
            ch[2].push([t, v / 10.0]);
        }
        // A short sensor dropout: NaN leaves a gap in the line.
        for pt in &mut ch[1][55_000..55_800] {
            pt[1] = f64::NAN;
        }
        ch.map(|v| XyData::from(v).assume_sorted())
    })
}

/// `(f Hz, |G| dB, phase deg)` of a 2nd-order plant with dead time and a
/// 1st-order model of it, over 0.01 … 100 Hz.
/// One Bode sample: `(frequency Hz, magnitude dB, phase deg)`.
type BodePoint = (f64, f64, f64);

fn bode_points() -> (Vec<BodePoint>, Vec<BodePoint>) {
    use std::f64::consts::PI;
    let n = 300;
    let (k, fn_hz, zeta, delay) = (2.0_f64, 1.5_f64, 0.25_f64, 0.05_f64);
    let tau = 1.0 / (2.0 * PI * 0.8);
    (0..n)
        .map(|i| {
            let f = 10f64.powf(-2.0 + 4.0 * i as f64 / (n - 1) as f64);
            let r = f / fn_hz;
            let (re, im) = (1.0 - r * r, 2.0 * zeta * r);
            let mag = k / (re * re + im * im).sqrt();
            let lag = 360.0 * f * delay;
            let w = 2.0 * PI * f * tau;
            (
                (f, 20.0 * mag.log10(), -im.atan2(re).to_degrees() - lag),
                (
                    f,
                    20.0 * (k / (1.0 + w * w).sqrt()).log10(),
                    -w.atan().to_degrees() - lag,
                ),
            )
        })
        .unzip()
}
