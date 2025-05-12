mod emulator;

use eframe::epaint::textures::{TextureFilter, TextureWrapMode};
use eframe::Frame;
use egui::{Context, TextureOptions};
use emulator::cartridge::Cartridge;
use emulator::{DebugMode, Emulator};
use simple_graphics::display::Display;

const WINDOW_TITLE: &str = "Nest";

struct NestApp {
    emulator: Emulator,
    frame_buffer: Vec<u8>,
    pattern_table_buffer: Vec<u8>,
    is_running: bool,
}

impl eframe::App for NestApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if self.is_running {
            self.emulator.step_frame(&mut |frame, pattern_table| {
                self.frame_buffer = frame.rgb().to_owned();
                self.pattern_table_buffer = pattern_table.rgb().to_owned();
            });
        }

        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Nes Emulator Controls");

            if ui.button(if self.is_running { "Pause" } else { "Run" }).clicked() {
                self.is_running = !self.is_running;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.frame_buffer.is_empty() {
                let size = [emulator::NES_WIDTH, emulator::NES_HEIGHT];
                let image = egui::ColorImage::from_rgb(size, &self.frame_buffer);
                let texture = ctx.load_texture(
                    "nes_display",
                    image,
                    TextureOptions {
                        magnification: TextureFilter::Nearest,
                        minification: TextureFilter::Nearest,
                        wrap_mode: TextureWrapMode::ClampToEdge
                    }
                );
                ui.image(&texture);
            }
        });

        egui::Window::new("Pattern Tables").show(ctx, |ui| {
            if !self.pattern_table_buffer.is_empty() {
                let size = [emulator::PATTERN_TABLE_WIDTH, emulator::PATTERN_TABLE_HEIGHT];
                let image = egui::ColorImage::from_rgb(size, &self.pattern_table_buffer);
                let texture = ctx.load_texture(
                    "pattern_table",
                    image,
                    TextureOptions {
                        magnification: TextureFilter::Nearest,
                        minification: TextureFilter::Nearest,
                        wrap_mode: TextureWrapMode::ClampToEdge
                    }
                );
                ui.image(&texture);
            }
        });

        ctx.request_repaint();
    }
}

impl NestApp {
    fn new() -> Self {
        let mut emulator = Emulator::new();
        emulator.set_debug_mode(vec![DebugMode::Ppu]);

        let cartridge = Cartridge::new("./roms/DonkeyKong.nes".to_string());
        emulator.load_cartridge(cartridge);
        emulator.reset();
        Self {
            emulator,
            frame_buffer: vec![],
            pattern_table_buffer: vec![],
            is_running: false,
        }
    }
}

fn main() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(WINDOW_TITLE, options, Box::new(|_cc| Box::new(NestApp::new()))).unwrap()
}
