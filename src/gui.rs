use crate::emulator::cartridge::Cartridge;
use crate::emulator::Emulator;
use eframe::epaint::textures::{TextureFilter, TextureWrapMode};
use eframe::Frame;
use egui::{Context, TextureOptions};
use rfd::FileDialog;

pub struct NestApp {
    // Emulator State
    emulator: Emulator,
    rom_path: Option<String>,
    frame_buffer: Vec<u8>,
    pattern_table_buffer: Vec<u8>,
    is_running: bool,

    // UI State
    show_pattern_table: bool,
    show_cpu_state: bool,
    raw_tick_amount: String,
    tick_amout: usize,
}

impl eframe::App for NestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // grab emulator state each frame for gui
        let emu_state = self.emulator.get_state();

        // tick emulator one frame and update buffers
        if self.is_running {
            self.emulator.step_frame(&mut |frame, pattern_table| {
                self.frame_buffer.clone_from(&frame.rgb());
                if self.show_pattern_table {
                    self.pattern_table_buffer.clone_from(&pattern_table.rgb());
                }
            });
        }

        // top menu bar, to open debugging windows
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Options", |ui| {
                    // open pattern table window
                    if ui.button("Pattern Table").clicked() {
                        self.show_pattern_table = true;
                        ui.close_menu();
                    }

                    // open cpu state and control window
                    if ui.button("Cpu").clicked() {
                        self.show_cpu_state = true;
                        ui.close_menu();
                    }
                });
            });
        });

        // main left panel
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Nes Emulator Controls");

            // opens file explorer to select rom
            if ui.button("Open File").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.rom_path = Some(path.display().to_string());
                    match Cartridge::new(path.display().to_string()) {
                        Ok(c) => {
                            self.emulator.load_cartridge(c);
                            self.emulator.reset();
                        }
                        Err(e) => println!("{e}"),
                    };
                };
            }

            // run/pause emulator running
            if ui
                .button(if self.is_running { "Pause" } else { "Run" })
                .clicked()
            {
                self.is_running = !self.is_running;
                // only log cpu instructions when not running at full speed
                // i.e. when ticking single instructions or frames
                self.emulator
                    .toggle_debug_flag(crate::emulator::DebugFlag::Cpu);
            }

            // reset emulator
            if ui.button("Reset").clicked() {
                self.emulator.reset();
            }
        });

        // main panel showing emulator
        egui::CentralPanel::default().show(ctx, |ui| {
            // only render if there is something to render
            if !self.frame_buffer.is_empty() {
                // setup texture using frame buffer we got earlier from emulator
                let size = [crate::emulator::NES_WIDTH, crate::emulator::NES_HEIGHT];
                let image = egui::ColorImage::from_rgb(size, &self.frame_buffer);
                let texture = ctx.load_texture(
                    "nes_display",
                    image,
                    TextureOptions {
                        magnification: TextureFilter::Nearest,
                        minification: TextureFilter::Nearest,
                        wrap_mode: TextureWrapMode::ClampToEdge,
                        mipmap_mode: None,
                    },
                );
                // render buffer as an egui image
                ui.image(&texture);
            }
        });

        // popup pattern table window
        // shows pattern table of current inserted rom
        egui::Window::new("Pattern Tables")
            .open(&mut self.show_pattern_table)
            .show(ctx, |ui| {
                // only render if there is something to render
                if !self.pattern_table_buffer.is_empty() {
                    // setup texture using frame buffer we got earlier from emulator
                    let size = [
                        crate::emulator::PATTERN_TABLE_WIDTH,
                        crate::emulator::PATTERN_TABLE_HEIGHT,
                    ];
                    let image = egui::ColorImage::from_rgb(size, &self.pattern_table_buffer);
                    let texture = ctx.load_texture(
                        "pattern_table",
                        image,
                        TextureOptions {
                            magnification: TextureFilter::Nearest,
                            minification: TextureFilter::Nearest,
                            wrap_mode: TextureWrapMode::ClampToEdge,
                            mipmap_mode: None,
                        },
                    );
                    // render buffer as an egui image
                    ui.image(&texture);
                }
            });

        // cpu state and control window
        egui::Window::new("Cpu")
            .open(&mut self.show_cpu_state)
            .show(ctx, |ui| {
                // only allow ticking single instuctions and frames when
                // emulator is paused
                if !self.is_running {
                    // box to enter amout of instuctions to run when
                    // tick button is pressed
                    if ui
                        .text_edit_singleline(&mut self.raw_tick_amount)
                        .lost_focus()
                    {
                        if let Ok(ticks) = self.raw_tick_amount.parse::<usize>() {
                            self.tick_amout = ticks;
                        }
                    }

                    // tick emulator for specified amount of ticks
                    if ui.button("Tick").clicked() {
                        for _ in 0..self.tick_amout {
                            self.emulator.tick(&mut |frame, pattern_table| {
                                self.frame_buffer.clone_from(&frame.rgb());
                                self.pattern_table_buffer.clone_from(&pattern_table.rgb());
                            });
                        }
                    }

                    // tick to the next frame
                    if ui.button("Next Frame").clicked() {
                        self.emulator.step_frame(&mut |frame, pattern_table| {
                            self.frame_buffer.clone_from(&frame.rgb());
                            self.pattern_table_buffer.clone_from(&pattern_table.rgb());
                        });
                    }
                }

                // show cpu state (registers, flags and last instruction)
                // view only at the moment but plans to change
                ui.label(format!("pc: {:#06x}", emu_state.r_pc));
                ui.label(format!("cycles: {}", emu_state.cycles));
                ui.label(format!("a: {:#04x}", emu_state.r_a));
                ui.label(format!("x: {:#04x}", emu_state.r_x));
                ui.label(format!("y: {:#04x}", emu_state.r_y));
                ui.label("Flags");
                ui.label(format!("z: {}", emu_state.f_z));
                ui.label(format!("c: {}", emu_state.f_c));
                ui.label(format!("d: {}", emu_state.f_d));
                ui.label(format!("i: {}", emu_state.f_i));
                ui.label(format!("n: {}", emu_state.f_n));
                ui.label(format!("v: {}", emu_state.f_v));
                ui.label(format!("sp: {:#06x}", emu_state.r_sp));
                ui.label(self.emulator.get_logged_instr());
            });

        ctx.request_repaint();
    }
}

// holds egui state
impl NestApp {
    pub fn new(emulator: Emulator) -> Self {
        Self {
            emulator,
            rom_path: None,
            frame_buffer: vec![],
            pattern_table_buffer: vec![],
            is_running: false,
            show_pattern_table: false,
            show_cpu_state: false,
            raw_tick_amount: String::new(),
            tick_amout: 1,
        }
    }
}
