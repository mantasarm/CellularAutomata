#![allow(dead_code)]

use std::{collections::HashSet, time::Instant};

use miniquad::*;

use crate::Game;

use super::triangle;

pub struct Engine {
    game_logic: Game,

    clear_color: (f32, f32, f32, f32),

    pipeline: Pipeline,
    bindings: Bindings,

    initialized: bool,

    egui_mq: egui_miniquad::EguiMq,

    info: Info
}

pub struct Perf {
    last_frame: Instant,
    pub dt: f32,
    pub fps: f32,
}

pub struct Input {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,

    pub last_char: char,
    keys_down: HashSet<KeyCode>,

    buttons_down: HashSet<MouseButton>
}

impl Input {
    pub fn is_button_down(&self, button: MouseButton) -> bool {
        self.buttons_down.contains(&button)
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }
}

pub struct Info {
    pub perf: Perf,
    pub input: Input
}


impl Engine {
    pub fn new(game_logic: Game, ctx: &mut miniquad::Context) -> Self {

        let triangle_info = triangle::create_triangle(ctx);

        Self {
            game_logic,
            
            clear_color: (0f32, 0f32, 0f32, 0f32),

            pipeline: triangle_info.0,
            bindings: triangle_info.1,

            initialized: false,

            egui_mq: egui_miniquad::EguiMq::new(ctx),

            info: Info {
                perf: Perf {
                    last_frame: std::time::Instant::now(),
                    dt: 1f32 / 60f32,
                    fps: 60f32,
                },

                input: Input {
                    mouse_x: 0f32,
                    mouse_y: 0f32,
                    mouse_dx: 0f32,
                    mouse_dy: 0f32,

                    last_char: ' ',
                    keys_down: HashSet::new(),

                    buttons_down: HashSet::new()
                }
            }
        }
    }
}

impl EventHandler for Engine {
    fn update(&mut self, ctx: &mut Context) {
        if !self.initialized {
            self.game_logic.init(ctx);
            self.initialized = true;
        }

        let now = std::time::Instant::now();

        self.info.perf.dt = (now - self.info.perf.last_frame).as_secs_f32();
        self.info.perf.fps = 1f32 / self.info.perf.dt;

        self.info.perf.last_frame = now;

        self.game_logic.update(&self.info);
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(PassAction::clear_color(self.clear_color.0, self.clear_color.1, self.clear_color.2, self.clear_color.3));

        // ctx.apply_pipeline(&self.pipeline);
        // ctx.apply_bindings(&self.bindings);
        // ctx.apply_uniforms(&shader::Uniforms { uProjection: self.game_logic.camera.get_proj_matrix(), uView: self.game_logic.camera.get_view_matrix()});

        // ctx.draw(0, 3, 1);

        self.game_logic.render(ctx, &mut self.clear_color);

        self.egui_mq.run(ctx, |egui_ctx| {
            self.game_logic.render_egui(&self.info, egui_ctx);
        });

        ctx.end_render_pass();

        ctx.commit_frame();
        self.egui_mq.draw(ctx);
    }
 
    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.info.input.mouse_x = x;
        self.info.input.mouse_y = y;

        self.egui_mq.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut miniquad::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(ctx, dx, dy);
    }

    fn char_event(&mut self, _: &mut Context, character: char, _: KeyMods, _: bool) {
        self.info.input.last_char = character;
        self.egui_mq.char_event(character);
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.info.input.buttons_down.insert(button);
        self.egui_mq.mouse_button_down_event(ctx, button, x, y);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.info.input.buttons_down.remove(&button);
        self.egui_mq.mouse_button_up_event(ctx, button, x, y);
    }

    fn raw_mouse_motion(&mut self, _ctx: &mut Context, dx: f32, dy: f32) {
        self.info.input.mouse_dx = dx;
        self.info.input.mouse_dy = dy;
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, _: bool) {
        self.info.input.keys_down.insert(keycode);

        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.info.input.keys_down.remove(&keycode);
        self.egui_mq.key_up_event(keycode, keymods);
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {

    }

    fn touch_event(&mut self, ctx: &mut Context, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(ctx, x, y);
        }
    }
}

pub fn start_engine(title: &str, width: i32, height: i32, fullscreen: bool, game_logic: Game) {
    miniquad::start(
        conf::Conf {
            window_title: title.to_string(),
            window_width: width,
            window_height: height,
            fullscreen,
            window_resizable: false,
            sample_count:0,
            high_dpi: false,
            
            ..Default::default()
        }, 
        move |mut ctx| UserData::owning(Engine::new(game_logic, &mut ctx), ctx),
    );
}

pub trait GameLogic {
    fn new() -> Self;

    fn init(&mut self, ctx: &mut Context);

    fn update(&mut self, input: &Input);
    
    fn render(&mut self, ctx: &mut Context, bg_color: &mut (f32, f32, f32, f32));

    fn render_egui(&mut self, egui_ctx: &egui::Context);
}

