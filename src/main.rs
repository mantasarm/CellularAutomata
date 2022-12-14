#![windows_subsystem = "windows"]

mod engine;
mod grid;
mod heap_array;

use egui::{Pos2, RichText, Color32};
use engine::{camera::Camera, shape_renderer::ShapeBatch, engine::Info, engine::start_engine};
use grid::{CellGrid, Cell, ElementData, COLS};
use miniquad::Context;

fn main() {
    let game = Game::new();

    start_engine("sandbox", 750, 1000, false, game);
}

pub struct Game {
    camera: Camera,
    shape_renderer: Option<ShapeBatch>,
    heat_map_renderer: Option<ShapeBatch>,
    render_heat_map: bool,

    cell_grid: CellGrid,
    selected_cell: Cell,
    brush_size: i32,
    update_simulation: bool
}

impl  Game {
    fn new() -> Self {
        let camera = Camera::new(0f32, 0f32, 750f32, 1000f32);


        Self {
            camera,
            shape_renderer: None,
            heat_map_renderer: None,
            render_heat_map: false,

            cell_grid: CellGrid::new(750f32 / COLS as f32),
            selected_cell: Cell::new(ElementData::sand_element()),
            brush_size: 8,
            update_simulation: true
        }
    }

    fn init(&mut self, ctx: &mut Context) {
       self.shape_renderer = Some(ShapeBatch::new(ctx, 2048 * 40));
       self.heat_map_renderer = Some(ShapeBatch::new(ctx, 2048 * 40));
        
        self.cell_grid.set_borders();
    }

    fn update(&mut self, info: &Info) {
        if info.input.is_button_down(miniquad::MouseButton::Left) {
            self.cell_grid.modify_cell((info.input.mouse_x  / self.cell_grid.get_size()) as i32 - self.brush_size / 2, grid::ROWS - (info.input.mouse_y  / self.cell_grid.get_size()) as i32 - self.brush_size / 2, self.selected_cell, self.brush_size);
        }

        if self.update_simulation {
            self.cell_grid.update();
        }
    }
    
    fn render(&mut self, ctx: &mut Context, bg_color: &mut (f32, f32, f32, f32)) {
        *bg_color = (0.13, 0.1, 0.11, 1f32);


        let mut shape_renderer = self.shape_renderer.as_mut().unwrap();
        shape_renderer.begin();
        self.cell_grid.render(&mut shape_renderer);
        shape_renderer.end(ctx, &mut self.camera);

        if self.render_heat_map {
            let mut heat_map_renderer = self.heat_map_renderer.as_mut().unwrap();
            heat_map_renderer.begin();
            self.cell_grid.render_heatmap(&mut heat_map_renderer);
            heat_map_renderer.end(ctx, &mut self.camera);
        }
    }

    fn render_egui(&mut self, info: &Info, egui_ctx: &egui::Context) {
        egui::Window::new("window").title_bar(false).resizable(false).default_pos(Pos2::new(0.0, 750.0)).show(egui_ctx, |ui| {
            ui.label("performance");

            ui.add_space(10f32);

            ui.label(format!("Num of indices: {}", self.shape_renderer.as_ref().unwrap().get_num_indices()));
            ui.label(format!("Num of vertices: {}", self.shape_renderer.as_ref().unwrap().get_num_vertices()));
            ui.label(format!("Num of triangles: {}", self.shape_renderer.as_ref().unwrap().get_num_indices() / 3));

            ui.label(format!("Num of swaps: {}", self.cell_grid.num_of_swaps()));

            ui.add_space(10f32);
            ui.label(format!("fps: {}", info.perf.fps));
        });


        egui::Window::new("options").title_bar(false).resizable(false).default_pos(Pos2::new(0.0, 900.0)).show(egui_ctx, |ui| {
            ui.label("options");

            ui.checkbox(&mut self.render_heat_map, "render_heat_map");
            ui.checkbox(&mut self.update_simulation, "update_simulation");
        });

        egui::Window::new("elements").title_bar(false).resizable(false).default_pos(Pos2::new(145.0, 750.0)).show(egui_ctx, |ui| {
            
            ui.label(format!("elements ({:?})", self.cell_grid.get_element_on_mouse((info.input.mouse_x  / self.cell_grid.get_size()) as i32, grid::ROWS - (info.input.mouse_y  / self.cell_grid.get_size()) as i32).unwrap_or(grid::CellType::Air)));
            ui.separator();

            ui.horizontal_wrapped(|ui| {
                if ui.button(RichText::new("Air").color(Color32::WHITE)).clicked() {
                    self.selected_cell = Cell::new(ElementData::air_element())
                }
    
                if ui.button("Solid").clicked() {
                    self.selected_cell = Cell::new(ElementData::solid_element())
                }
    
                if ui.button(RichText::new("Sand").color(Color32::from_rgb(243, 239, 118))).clicked() {
                    self.selected_cell = Cell::new(ElementData::sand_element())
                }
    
                if ui.button(RichText::new("Water").color(Color32::from_rgb(18, 24, 204))).clicked() {
                    self.selected_cell = Cell::new(ElementData::water_element())
                }

                if ui.button(RichText::new("Steam").color(Color32::from_rgb(195, 225, 247))).clicked() {
                    self.selected_cell = Cell::new(ElementData::steam_element())
                }

                if ui.button(RichText::new("Fire").color(Color32::from_rgb(214, 32, 19))).clicked() {
                    self.selected_cell = Cell::new(ElementData::fire_element(80))
                }

                if ui.button(RichText::new("Coal").color(Color32::LIGHT_GRAY)).clicked() {
                    self.selected_cell = Cell::new(ElementData::coal_element())
                }
                
                if ui.button(RichText::new("Saw dust").color(Color32::from_rgb(219, 199, 120))).clicked() {
                    self.selected_cell = Cell::new(ElementData::sawdust_element())
                }
                
                if ui.button(RichText::new("Methane").color(Color32::from_rgb(133, 191, 47))).clicked() {
                    self.selected_cell = Cell::new(ElementData::methane_element())
                }
                
                if ui.button(RichText::new("Lava").color(Color32::from_rgb(150, 59, 28))).clicked() {
                    self.selected_cell = Cell::new(ElementData::lava_element());
                }
                
                if ui.button(RichText::new("Cold fire").color(Color32::from_rgb(59, 205, 219))).clicked() {
                    self.selected_cell = Cell::new(ElementData::coldfire_element(80));
                }
                
                if ui.button(RichText::new("Ice").color(Color32::from_rgb(112, 169, 229))).clicked() {
                    self.selected_cell = Cell::new(ElementData::ice_element());
                }
                
                if ui.button(RichText::new("Gasoline").color(Color32::from_rgb(220, 207, 61))).clicked() {
                    self.selected_cell = Cell::new(ElementData::gasoline_element());
                }
            });
        });
        
        egui::Window::new("brush size").title_bar(false).resizable(false).default_pos(Pos2::new(145.0, 860.0)).show(egui_ctx, |ui| {
            ui.label("brush_size");
            
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.brush_size, 1..=50).integer().prefix("size: "));
            });
        });
    }
}