use std::f64::consts::TAU;

use eframe::{
    egui::{self, CentralPanel, Grid, Layout, SidePanel, Slider, Visuals},
    emath::Align,
    epaint::Color32,
    CreationContext,
};
use egui_plot::{Legend, Line, Plot, PlotPoints};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Spiro {
    a: u32,
    b: u32,
    c: u32,
    color: Color32,
    width: f32,
    param_max: u32,
}

impl Default for Spiro {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            color: Color32::LIGHT_RED,
            width: 1.0,
            param_max: 20,
        }
    }
}

impl Spiro {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());

        if let Some(storage) = cc.storage {
            if let Some(storage) = eframe::get_value(storage, eframe::APP_KEY) {
                return storage;
            }
        }
                
        let mut f = Self::default();
        f.shuffle();
        f
    }

    fn shuffle(&mut self) {
        (self.a, self.b, self.c) = (
            fastrand::u32(1..self.param_max),
            fastrand::u32(1..self.param_max),
            fastrand::u32(1..self.param_max),
        );
    }
}

impl eframe::App for Spiro {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::right("right_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(3.0);
                ui.vertical_centered_justified(|ui| ui.heading("Spiro"));
                ui.add_space(3.0);

                Grid::new("params").show(ui, |ui| {
                    ui.add(Slider::new(&mut self.a, 1..=self.param_max));
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| ui.label("A"));
                    ui.end_row();

                    ui.add(Slider::new(&mut self.b, 1..=self.param_max));
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| ui.label("B"));
                    ui.end_row();

                    ui.add(Slider::new(&mut self.c, 1..=self.param_max));
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| ui.label("C"));
                    ui.end_row();
                });

                ui.separator();

                Grid::new("style").show(ui, |ui| {
                    ui.color_edit_button_srgba(&mut self.color);
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| ui.label("color"));
                    ui.end_row();
                    ui.add(Slider::new(&mut self.width, 0.5..=5.0));
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.label("thickness")
                    });
                });

                ui.separator();

                Grid::new("settings").show(ui, |ui| {
                    ui.add(Slider::new(&mut self.param_max, 1..=50));
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.label("max")
                    });

                    ui.end_row();
                    if ui.button("Shuffle").clicked() {
                        self.shuffle();
                    }
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            let a = self.a as f64;
            let b = self.b as f64;
            let c = self.c as f64;

            let range_max: f64 = TAU * a;
            let dt: f64 = 0.001;

            let points = PlotPoints::from_parametric_callback(
                |t| {
                    let k = (b * t) / a;

                    let t_k = t + k;
                    let a_b = a + b;

                    (
                        ((a_b * k.cos()) - (c * t_k.cos())),
                        ((a_b * k.sin()) - (c * t_k.sin())),
                    )
                },
                0.0..range_max,
                (range_max / dt) as usize,
            );

            Plot::new("graph")
                .legend(Legend::default())
                .center_x_axis(true)
                .center_y_axis(true)
                .allow_boxed_zoom(false)
                .allow_zoom(false)
                .allow_drag(false)
                .data_aspect(1.0)
                .show_grid(false)
                .show_axes(false)
                .show_x(false)
                .show_y(false)
                .show_axes(false)
                .show_background(false)
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(points).color(self.color).width(self.width));
                });
        });
    }
}
