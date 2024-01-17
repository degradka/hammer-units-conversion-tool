#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

#[derive(Clone, Copy, Debug, PartialEq)]
enum RealMeasurement {
    Yards,
    Feet,
    Inches,
    Meters,
    Decimeters,
    Centimeters,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum HammerMeasurement {
    Units,
    Skybox,
}

struct App {
    real_measurement_left: RealMeasurement,
    hammer_measurement_right: HammerMeasurement,
    value_left: String,
    value_right: String,
    prev_real_measurement_left: RealMeasurement,
    prev_hammer_measurement_right: HammerMeasurement,
}

impl App {
    fn new() -> Self {
        App {
            real_measurement_left: RealMeasurement::Feet,
            hammer_measurement_right: HammerMeasurement::Units,
            value_left: "1".to_string(),
            value_right: "16".to_string(),
            prev_real_measurement_left: RealMeasurement::Feet,
            prev_hammer_measurement_right: HammerMeasurement::Units,
        }
    }

    fn convert_values(from: f32, to: f32, value: f32) -> f32 {
        value * (from / to)
    }

    fn combo_box_value_changed(&self) -> bool {
        self.real_measurement_left != self.prev_real_measurement_left
            || self.hammer_measurement_right != self.prev_hammer_measurement_right
    }

    fn update_right_value(&mut self, ctx: &egui::Context) {
        if let Ok(value) = self.value_left.parse::<f32>() {
            let from_value = match self.real_measurement_left {
                RealMeasurement::Yards => 0.9144,
                RealMeasurement::Feet => 0.3048,
                RealMeasurement::Inches => 0.0254,
                RealMeasurement::Meters => 1.0,
                RealMeasurement::Decimeters => 0.1,
                RealMeasurement::Centimeters => 0.01,
            };

            let to_value = match self.hammer_measurement_right {
                HammerMeasurement::Units => 0.01905,
                HammerMeasurement::Skybox => 0.3048
            };

            let converted_value = Self::convert_values(from_value, to_value, value);
            self.value_right = format!("{:.2}", converted_value);
            self.value_right = ((self.value_right.parse::<f32>().unwrap() * 20.0).round() / 20.0).to_string();
    
            ctx.request_repaint();
        }
    }

    fn update_left_value(&mut self, ctx: &egui::Context) {
        if let Ok(value) = self.value_right.parse::<f32>() {
            let from_value = match self.hammer_measurement_right {
                HammerMeasurement::Units => 0.01905,
                HammerMeasurement::Skybox => 0.3048
            };

            let to_value = match self.real_measurement_left {
                RealMeasurement::Yards => 0.9144,
                RealMeasurement::Feet => 0.3048,
                RealMeasurement::Inches => 0.0254,
                RealMeasurement::Meters => 1.0,
                RealMeasurement::Decimeters => 0.1,
                RealMeasurement::Centimeters => 0.01,
            };

            let converted_value = Self::convert_values(from_value, to_value, value);
            self.value_left = format!("{:.2}", converted_value);
            self.value_left = ((self.value_left.parse::<f32>().unwrap() * 20.0).round() / 20.0).to_string();

            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([270.0, 200.0])
            .with_inner_size([270.0, 200.0])
            .with_resizable(false)
            .with_icon(load_icon()),
        ..Default::default()
    };

    let mut app = App::new();
    eframe::run_simple_native("Hammer Units Conversion Tool", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hammer Units Conversion Tool");

            ui.horizontal(|ui| {
                // Left side
                ui.vertical(|ui| {
                    ui.push_id(ui.id().with("real_units_combo_left"), |ui| {
                        egui::ComboBox::from_id_source(ui.id())
                            .selected_text(format!("{:?}", app.real_measurement_left))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut app.real_measurement_left, RealMeasurement::Yards, "Yards");
                                ui.selectable_value(&mut app.real_measurement_left, RealMeasurement::Feet, "Feet");
                                ui.selectable_value(&mut app.real_measurement_left, RealMeasurement::Inches, "Inches");
                                ui.selectable_value(&mut app.real_measurement_left, RealMeasurement::Meters, "Meters");
                                ui.selectable_value(&mut app.real_measurement_left, RealMeasurement::Decimeters, "Decimeters");
                                ui.selectable_value(&mut app.real_measurement_left, RealMeasurement::Centimeters, "Centimeters");
                            });
                    });

                    // Text Edit for left value
                    let left_value_response = ui.add(
                        egui::TextEdit::singleline(&mut app.value_left)
                            .hint_text("Enter value")
                            .desired_width(100.0),
                    );
                    // Check if left value changed and update the right value
                    if left_value_response.changed() {
                        app.update_right_value(ctx);
                    }
                    
                    ui.label("by degradka");
                });

                // Center - Equal Sign
                ui.horizontal_centered(|ui| {
                    ui.label("=");
                });

                // Right side
                ui.vertical(|ui| {
                    // Hammer Units selection
                    ui.push_id(ui.id().with("hammer_units_combo_right"), |ui| {
                        egui::ComboBox::from_id_source(ui.id())
                            .selected_text(format!("{:?}", app.hammer_measurement_right))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut app.hammer_measurement_right, HammerMeasurement::Units, "Units");
                                ui.selectable_value(&mut app.hammer_measurement_right, HammerMeasurement::Skybox, "Skybox");
                            });
                    });

                    // Text Edit for right value
                    let right_value_response = ui.add(
                        egui::TextEdit::singleline(&mut app.value_right)
                            .hint_text("Converted value")
                            .desired_width(100.0),
                    );
                    // Check if right value changed and update the left value
                    if right_value_response.changed() {
                        app.update_left_value(ctx);
                    }
                });

            // Check if combo box values changed and update accordingly
            if app.combo_box_value_changed() {
                if app.real_measurement_left != app.prev_real_measurement_left {
                    app.update_right_value(ctx);
                }
                if app.hammer_measurement_right != app.prev_hammer_measurement_right {
                    app.update_left_value(ctx);
                }

                // Update previous values
                app.prev_real_measurement_left = app.real_measurement_left;
                app.prev_hammer_measurement_right = app.hammer_measurement_right;
            }
            });
        });
    })
}

fn load_icon() -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../favicon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open the icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
        
    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}