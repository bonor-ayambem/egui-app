
use eframe::egui::{self, *, Rect};
use emath::RectTransform;

use std::fs::File;
use serde_json;
use std::io::{Write, Read};


use crate::object::Object;

// Constants
const SOLAR_MASS: f32 = 150.0;
const YEAR: f32 = 365.24;
const N_BODIES: usize = 5;

static PLANET_NAMES: [&str; N_BODIES] = ["Sun", "Jupiter", "Saturn", "Uranus", "Neptune",];
static PLANET_COLORS: [Color32; N_BODIES] = [Color32::YELLOW, Color32::BROWN, Color32::GREEN, Color32::GRAY, Color32::KHAKI,];


static SCALE_FACTOR: f32 = 10.0;
static OFFSET_X: f32 = 0.0;
static OFFSET_Y: f32 = 100.0;
static PLANETS: [Object;N_BODIES] = [
    // Sun
    Object {
        x: 55.0 * SCALE_FACTOR + OFFSET_X, 
        y: 15.0 * SCALE_FACTOR + OFFSET_Y, 
        z: 50.0 * SCALE_FACTOR,
        vx: 0.0, vy: 0.0, vz: 0.0,
        mass: SOLAR_MASS,
    },
    // Jupiter
    Object {
        x: 4.84143144246472090e+00 * SCALE_FACTOR + OFFSET_X,
        y: -1.16032004402742839e+00 * SCALE_FACTOR + OFFSET_Y,
        z: -1.03622044471123109e-01 * SCALE_FACTOR,
        vx: 1.66007664274403694e-03 * YEAR,
        vy: 7.69901118419740425e-03 * YEAR,
        vz: -6.90460016972063023e-05 * YEAR,
        mass: 15.41,
    },
    // Saturn
    Object {
        x: 8.34336671824457987e+00 * SCALE_FACTOR + OFFSET_X,
        y: 4.12479856412430479e+00 * SCALE_FACTOR + OFFSET_Y,
        z: -4.03523417114321381e-01 * SCALE_FACTOR,
        vx: -2.76742510726862411e-03 * YEAR,
        vy: 4.99852801234917238e-03 * YEAR,
        vz: 2.30417297573763929e-05 * YEAR,
        mass: 12.55,
    },
    // Uranus
    Object {
        x: 1.28943695621391310e+01 * SCALE_FACTOR + OFFSET_X,
        y: -1.51111514016986312e+01 * SCALE_FACTOR + OFFSET_Y,
        z: -2.23307578892655734e-01 * SCALE_FACTOR,
        vx: 2.96460137564761618e-03 * YEAR,
        vy: 2.37847173959480950e-03 * YEAR,
        vz: -2.96589568540237556e-05 * YEAR,
        mass: 5.059,
    },
    // Neptune
    Object {
        x: 1.53796971148509165e+01 * SCALE_FACTOR + OFFSET_X,
        y: -2.59193146099879641e+01 * SCALE_FACTOR + OFFSET_Y,
        z: 1.79258772950371181e-01 * SCALE_FACTOR,
        vx: 2.68067772490389322e-03 * YEAR,
        vy: 1.62824170038242295e-03 * YEAR,
        vz: -9.51592254519715870e-05 * YEAR,
        mass: 4.896,
    },
];

/// Application state struct
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)] // This how you opt-out of serialization of a field
    planets: Vec<Object>,
    input_files: Vec<String>,
    data_file: String,
    save_count: u32,
    paused: bool,
}

// Default implementation for the application state
impl Default for TemplateApp {
    fn default() -> Self {
        let mut sim = Self {
            planets: PLANETS.to_vec(),
            input_files: Vec::<String>::new(),
            data_file: "data/original_input.json".to_owned(),
            save_count: 1,
            paused: false,
        };
        sim.input_files.push(sim.data_file.clone());
        sim
    }
}

// Implementation of the application
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    ui.menu_button("Open", |ui|{
                        for file in self.input_files.clone() {
                            if ui.button(&file).clicked() {
                                let _ = read_data(&file, &mut self.planets);

                                for p in &mut self.planets {
                                    p.x = p.x * SCALE_FACTOR + OFFSET_X;
                                    p.y = p.y * SCALE_FACTOR + OFFSET_Y;
                                    p.z = p.z * SCALE_FACTOR;
                                    p.vx = p.vx * 365.24;
                                    p.vy = p.vy * 365.24;
                                    p.vz = p.vz * 365.24;
                                }
                            }
                        }
                    });
                    if ui.button("Save").clicked(){
                        // write to file and save
                        let filename = format!("data/saved_file{}.json", self.save_count);
                        let _ = write_data(&filename, self.planets.clone());
                        self.input_files.push(String::from(filename));
                        self.save_count = self.save_count + 1;
                        ui.close_menu();
                    }
                });
                if ui.button("About").clicked(){
                    egui::Window::new("About Window").open(&mut true).show(ctx, |ui|{
                        ui.label("Project: Data Simulation and Display");
                        ui.label("Author: Bonor Ayambem");
                        ui.label("Course: CSE-411 Advanced Programming Techniques");
                        ui.label("Date: November 27, 2023");
                    });
                }
                egui::menu::menu_button(ui, "Quit", |_| {
                    self.input_files.clear();
                    self.save_count = 1;
                    _frame.close();
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_buttons(ui);
            if ui.button("Pause").clicked(){
                self.paused = true;
            }
            if ui.button("Play").clicked(){
                self.paused = false;
            }
        });

        egui::CentralPanel::default().show(ctx, |_| {
            egui::SidePanel::left("data_panel").exact_width(200.0).show(ctx, |ui| {
                ui.label("Planets Data").highlight();

                for ix in 0..N_BODIES {
                    let name = PLANET_NAMES[ix];
                    ui.label(name);
                    
                    ui.label(format!("\tx = {}", self.planets[ix].x));
                    ui.label(format!("\ty = {}", self.planets[ix].y));
                    ui.label(format!("\tvx = {}", self.planets[ix].vx));
                    ui.label(format!("\tvy = {}", self.planets[ix].vy));
                    ui.label(format!("\tmass = {}", self.planets[ix].mass));
                }
            });

            egui::CentralPanel::default().show(ctx, |_| {
                egui::TopBottomPanel::bottom("contorls_panel").show(ctx, |ui| {
                    ui.label("Controls Panel").highlight();

                    egui::Grid::new("control_grid").show(ui, |ui|{
                        for ix in 0..N_BODIES {
                            ui.label(format!("{}", PLANET_NAMES[ix])).highlight();
                            ui.add(
                                Slider::new(&mut self.planets[ix].mass, 0.0..=500.0)
                                    .clamp_to_range(true)
                                    .text("mass"),
                            );
                            ui.add(
                                Slider::new(&mut self.planets[ix].x, 0.0..=500.0)
                                    .clamp_to_range(true)
                                    .text("x"),
                            );
                            ui.add(
                                Slider::new(&mut self.planets[ix].y, 0.0..=500.0)
                                    .clamp_to_range(true)
                                    .text("y"),
                            );
                            ui.end_row();
                        }
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Canvas Visualization");
                    let (response, painter) = ui.allocate_painter(
                        bevy_egui::egui::Vec2::new(ui.available_width(), ui.available_height()),
                        Sense::focusable_noninteractive(),
                    );

                    let to_screen = RectTransform::from_to(
                        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                        response.rect,
                    );
                    
                    let mut shapes: Vec<epaint::Shape> = Vec::with_capacity(N_BODIES);

                    for ix in 0..N_BODIES {
                        let x = self.planets[ix].x;
                        let y = self.planets[ix].y;
                        let circle = epaint::Shape::Circle(epaint::CircleShape{
                            center: to_screen.transform_pos(Pos2{x: x, y: y}),
                            radius: self.planets[ix].mass,
                            fill: PLANET_COLORS[ix],
                            stroke: epaint::Stroke::new(0.0, Color32::TRANSPARENT),
                        });
                        shapes.push(circle);
                    }
                    painter.extend(shapes.clone());

                    if self.paused == false {
                        offset_momentum(&mut self.planets);
                        advance(&mut self.planets, 0.001, 100);
                    }
                    
                });
            });
        });
    }
}

fn offset_momentum(bodies: &mut Vec<Object>) {
    let mut px = 0.0;
    let mut py = 0.0;
    let mut pz = 0.0;
    for bi in bodies.iter() {
        px += bi.vx * bi.mass;
        py += bi.vy * bi.mass;
        pz += bi.vz * bi.mass;
    }
    let sun = &mut bodies[0];
    sun.vx = -px / SOLAR_MASS;
    sun.vy = -py / SOLAR_MASS;
    sun.vz = -pz / SOLAR_MASS;
}

fn advance(bodies: &mut Vec<Object>, dt: f32, steps: i32) {
    for _ in 0..steps{
        let mut b_slice: &mut [_] = bodies;
        loop {
            let bi = match shift_mut_ref(&mut b_slice) {
                Some(bi) => bi,
                None => break
            };
            for bj in b_slice.iter_mut() {
                let dx = bi.x - bj.x;
                let dy = bi.y - bj.y;
                let dz = bi.z - bj.z;
    
                let d2 = dx * dx + dy * dy + dz * dz;
                let mag = dt / (d2 * d2.sqrt());
    
                let massj_mag = bj.mass * mag;
                bi.vx -= dx * massj_mag;
                bi.vy -= dy * massj_mag;
                bi.vz -= dz * massj_mag;
    
                let massi_mag = bi.mass * mag;
                bj.vx += dx * massi_mag;
                bj.vy += dy * massi_mag;
                bj.vz += dz * massi_mag;
            }
            bi.x += dt * bi.vx;
            bi.y += dt * bi.vy;
            bi.z += dt * bi.vz;
        }
    }
}

/// Pop a mutable reference off the head of a slice, mutating the slice to no
/// longer contain the mutable reference.
fn shift_mut_ref<'a, T>(r: &mut &'a mut [T]) -> Option<&'a mut T> {
    if r.len() == 0 { return None }
    let tmp = std::mem::replace(r, &mut []);
    let (h, t) = tmp.split_at_mut(1);
    *r = t;
    Some(&mut h[0])
}
   
#[warn(dead_code)]
fn energy(bodies: &mut Vec<Object>) -> f32 {
    let mut e = 0.0;
    let mut bodies = bodies.iter();
    loop {
        let bi = match bodies.next() {
            Some(bi) => bi,
            None => break
        };
        e += (bi.vx * bi.vx + bi.vy * bi.vy + bi.vz * bi.vz) * bi.mass / 2.0;
        for bj in bodies.clone() {
            let dx = bi.x - bj.x;
            let dy = bi.y - bj.y;
            let dz = bi.z - bj.z;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            e -= bi.mass * bj.mass / dist;
        }
    }
    e
}

// Read data from file
fn read_data(filename: &str, planets: &mut Vec<Object>) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file in read-only mode
    let file = File::open(filename)?;

    // Create a buffer to hold the file contents
    let mut buffer = String::new();

    // Read the file contents into the buffer
    file.take(u64::MAX as u64).read_to_string(&mut buffer)?;

    // Deserialize the JSON data into a vector of Objects
    let new_planets: Vec<Object> = serde_json::from_str(&buffer)?;

    // Clear the existing 'planets' vector and replace it with the new data
    planets.clear();
    planets.extend(new_planets);

    // The function executed successfully
    Ok(())
}

// Write data to file
fn write_data(filename: &str, mut planets: Vec<Object>) -> Result<(), Box<dyn std::error::Error>> {
    for p in &mut planets {
        p.x = (p.x - OFFSET_X) / SCALE_FACTOR;
        p.y = (p.y - OFFSET_Y) / SCALE_FACTOR;
        p.z = p.z / SCALE_FACTOR;
        p.vx = p.vx / 365.24;
        p.vy = p.vy / 365.24;
        p.vz = p.vz / 365.24;
    }

    // Serialize the 'planets' vector into a JSON string
    let planets_json = serde_json::to_string(&planets)?;

    // Open the file in write mode, creating it if it doesn't exist
    let mut file = File::create(filename)?;

    // Write the JSON string to the file
    file.write_all(planets_json.as_bytes())?;

    // The function executed successfully
    Ok(())
}