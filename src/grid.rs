use crate::{engine::shape_renderer::ShapeBatch, heap_array};


pub const COLS: i32 = 210;
pub const ROWS: i32 = 210;

pub struct CellGrid {
    cols: u32,
    rows: u32,

    size: f32,

    cells: Box<[[Cell; ROWS as usize]; COLS as usize]>,

    swaps: Vec<Swap>,
    num_of_swaps: u32
}

impl CellGrid {
    pub fn new(size: f32) -> Self {
        Self {
            cols: COLS as u32,
            rows: ROWS as u32,

            size,

            cells: heap_array::create_cells_array(),
            swaps: Vec::new(),
            num_of_swaps: 0
        }
    }
    
    pub fn set_borders(&mut self) {
        self.cells[10][10] = Cell::new(ElementData::sand_element());

        for i in 0..COLS {
            self.cells[i as usize][0] = Cell::new(ElementData::solid_element());
            self.cells[i as usize][(ROWS - 1) as usize] = Cell::new(ElementData::solid_element());
        }

        for j in 0..ROWS {
            self.cells[0][j as usize] = Cell::new(ElementData::solid_element());
            self.cells[(COLS - 1) as usize][j as usize] = Cell::new(ElementData::solid_element());
        }
    }

    pub fn render(&self, shape_renderer: &mut ShapeBatch) {
        for i in 0..self.cols {
            for j in 0..self.rows {

                let color = rgb(self.cells[i as usize][j as usize].element_data.color.0, self.cells[i as usize][j as usize].element_data.color.1, self.cells[i as usize][j as usize].element_data.color.2, 
                    self.cells[i as usize][j as usize].element_data.color.3);

                if color.3 != 0.0 {
                    shape_renderer.set_color(color.0, color.1, color.2, color.3);
                    shape_renderer.draw_rect(i as f32 * self.size, j as f32 * self.size + 250f32, self.size, self.size);
                }
            }
        }
    }

    pub fn render_heatmap(&self, shape_renderer: &mut ShapeBatch) {
        for i in 0..self.cols {
            for j in 0..self.rows {
                
                if self.cells[i as usize][j as usize].heat_value >= 0f32 {
                    let r = map(&self.cells[i as usize][j as usize].heat_value, 0f32, 2000f32, 0f32, 255f32);
                    shape_renderer.set_color(r, 1f32 - r, 0.0, 1.0); 
                } else {
                    let r = map(&self.cells[i as usize][j as usize].heat_value, 0f32, -2000f32, 0f32, 255f32);
                    shape_renderer.set_color(0.0, 1.0 - r, r, 1.0);
                }
                
                
                shape_renderer.draw_rect(i as f32 * self.size, j as f32 * self.size + 250f32, self.size, self.size);
            }
        }
    }

    pub fn update(&mut self) {
        for i in 1..self.cols - 1 {
            for j in 1..self.rows - 1 {
                let i = i as usize;
                let j = j as usize;

                self.cells[i][j].heat_value = (self.cells[i - 1][j].heat_value + self.cells[i + 1][j].heat_value
                                                                + self.cells[i][j - 1].heat_value * 0.9 + self.cells[i][j + 1].heat_value) / 4f32;

                match self.cells[i][j].element_data.cell_type {
                    CellType::Sand => {
                        self.falling_sand(i, j);
                    }

                    CellType::Water => {
                        self.liquid_movement(i, j);
                        
                        if self.cells[i][j].heat_value < -100f32 {
                            self.cells[i][j].element_data = ElementData::ice_element();
                        }

                        if self.cells[i][j].heat_value > 100f32 {
                            self.cells[i][j].element_data = ElementData::steam_element();
                        }
                    }

                    CellType::Steam => {
                        self.gas_movement(i, j, 0.33);
                        
                        if self.cells[i][j].heat_value < 1f32 {
                            if 0.001 > fastrand::f32() {
                                self.cells[i][j].element_data = ElementData::water_element();
                            }
                        }
                        
                        if self.cells[i][j].heat_value < -50f32 {
                            self.cells[i][j].element_data = ElementData::water_element();
                        }
                    }

                    CellType::Fire => {
                      self.cells[i][j].element_data.lifetime -= fastrand::i16(1..6);

                      self.cells[i][j].heat_value = self.cells[i][j].element_data.emitting_heat;

                       if self.cells[i][j].element_data.lifetime < 0 {
                         self.cells[i][j].element_data = ElementData::air_element();
                       }

                       self.gas_movement(i, j, 0.25);
                    }

                    CellType::Coal => {
                        if self.cells[i][j].heat_value > 400f32 && !self.cells[i][j].active {
                            self.cells[i][j].active = true;
                            self.cells[i][j].element_data.lifetime = 500;
                            self.cells[i][j].element_data.color = (50, 30, 29, 255)
                        }
                        
                        if self.cells[i][j].active {
                            self.cells[i][j].element_data.lifetime -= fastrand::i16(1..=2);
                            self.burn(i, j, 0.04, 40);
                        }
                        
                        if self.cells[i][j].element_data.lifetime < 0 && self.cells[i][j].active {
                            self.cells[i][j].element_data = ElementData::air_element();
                            self.cells[i][j].active = false;
                        }
                    }
                    
                    CellType::SawDust => {
                        if self.cells[i][j].heat_value > 250f32 && !self.cells[i][j].active {
                            self.cells[i][j].active = true;
                            self.cells[i][j].element_data.lifetime = 300;
                            self.cells[i][j].element_data.color = (50, 30, 29, 255)
                        }
                        
                        if self.cells[i][j].active {
                            self.cells[i][j].element_data.lifetime -= fastrand::i16(1..=2);
                            self.burn(i, j, 0.10, 60);
                        }
                        
                        if self.cells[i][j].element_data.lifetime < 0 && self.cells[i][j].active {
                            self.cells[i][j].element_data = ElementData::air_element();
                            self.cells[i][j].active = false;
                        }
                        
                        self.falling_sand(i, j);
                    }
                    
                    CellType::Methane => {
                        if self.cells[i][j].heat_value > 100f32 {
                            self.cells[i][j] = Cell::new(ElementData::fire_element(80));
                        } else {
                            self.gas_movement(i, j, 0.33);
                        }
                    }
                    
                    CellType::Lava => {
                        self.cells[i][j].heat_value = self.cells[i][j].element_data.emitting_heat;
                        
                        self.liquid_movement(i, j);
                        
                        self.burn(i, j, 0.05, 20);
                    }
                    
                    CellType::ColdFire => {
                        self.cells[i][j].element_data.lifetime -= fastrand::i16(1..6);
                        
                        self.cells[i][j].heat_value = self.cells[i][j].element_data.emitting_heat;
                        
                        if self.cells[i][j].element_data.lifetime < 0 {
                            self.cells[i][j].element_data = ElementData::air_element();
                        }
                        
                        self.gas_movement(i, j, 0.25);
                    }
                    
                    CellType::Ice => {
                        self.cells[i][j].heat_value += self.cells[i][j].element_data.emitting_heat;
                        
                        if self.cells[i][j].heat_value > -100f32 && 0.1 > fastrand::f32() {
                            self.cells[i][j].element_data = ElementData::water_element();
                        }
                    }
                    
                    CellType::Gasoline => {
                        if self.cells[i][j].heat_value > 250f32 && !self.cells[i][j].active {
                            self.cells[i][j].active = true;
                            self.cells[i][j].element_data.lifetime = 120;
                        }
                        
                        if self.cells[i][j].active {
                            self.cells[i][j].element_data.lifetime -= fastrand::i16(1..=3);
                            self.burn(i, j, 0.20, 90);
                        }
                        
                        self.liquid_movement(i, j);
                                                
                        if self.cells[i][j].element_data.lifetime < 0 && self.cells[i][j].active {
                            self.cells[i][j].element_data = ElementData::air_element();
                            self.cells[i][j].active = false;
                        }
                        
                    }

                    _ => ()
                }
            }
        }

        self.num_of_swaps = self.swaps.len().try_into().unwrap();

        let swaps = self.swaps.clone();
        self.swaps.clear();
        for swap in swaps {
            self.swap(swap.i1, swap.j1, swap.i2, swap.j2);
        }


    }

    fn swap(&mut self, i1: u32, j1: u32, i2: u32, j2: u32) {
        if i1 < self.cols && i2 < self.cols && j1 < self.rows && j2 < self.rows {

            let temp = self.cells[i2 as usize][j2 as usize].clone();

            self.cells[i2 as usize][j2 as usize] = self.cells[i1 as usize][j1 as usize];
            self.cells[i1 as usize][j1 as usize] = temp;
        }
    }

    fn falling_sand(&mut self, i: usize, j: usize) -> bool {
        if self.cells[i][j - 1].element_data.state != State::Solid {
            self.swaps.push(Swap::new_usize(i, j, i, j - 1));
            return true;
        } else if self.cells[i + 1][j - 1].element_data.state != State::Solid  && self.cells[i + 1][j].element_data.state != State::Solid {
            self.swaps.push(Swap::new_usize(i, j, i + 1, j - 1));
            return true;
        } else if self.cells[i - 1][j - 1].element_data.state != State::Solid && self.cells[i - 1][j].element_data.state != State::Solid {
            self.swaps.push(Swap::new_usize(i, j, i - 1, j - 1));
            return true;
        }
        false
    }

    fn liquid_movement(&mut self, i: usize, j: usize) {
        if self.cells[i][j - 1].element_data.state == State::Gas || self.cells[i][j - 1].element_data.state == State::Plasma  {
            self.swaps.push(Swap::new_usize(i, j, i, j - 1));
            
        } else if self.cells[i + 1][j - 1].element_data.state == State::Gas  && self.cells[i + 1][j].element_data.state == State::Gas
            || self.cells[i - 1][j - 1].element_data.state == State::Gas && self.cells[i - 1][j].element_data.state == State::Gas {
            if fastrand::bool() {
                if self.cells[i + 1][j - 1].element_data.state == State::Gas  && self.cells[i + 1][j].element_data.state == State::Gas {
                    self.swaps.push(Swap::new_usize(i, j, i + 1, j - 1))
                } else {
                    self.swaps.push(Swap::new_usize(i, j, i - 1, j - 1));
                }
            } else {
                if self.cells[i - 1][j - 1].element_data.state == State::Gas  && self.cells[i - 1][j].element_data.state == State::Gas {
                    self.swaps.push(Swap::new_usize(i, j, i - 1, j - 1))
                } else {
                    self.swaps.push(Swap::new_usize(i, j, i + 1, j - 1));
                }
            }
        } else if fastrand::bool() {
            if self.cells[i + 1][j].element_data.state == State::Gas || self.cells[i + 1][j].element_data.state == State::Plasma {
                self.swaps.push(Swap::new_usize(i, j, i + 1, j));
            }
        } else {
            if self.cells[i - 1][j].element_data.state == State::Gas || self.cells[i - 1][j].element_data.state == State::Plasma {
                self.swaps.push(Swap::new_usize(i, j, i - 1, j));
            }
        }
    }

    fn gas_movement(&mut self, i: usize, j: usize, spread: f32) {
        let mut offset = 0;

        if self.cells[i][j + 1].element_data.state == State::Gas && self.cells[i][j + 1].element_data.cell_type != self.cells[i][j].element_data.cell_type {
            self.swaps.push(Swap::new_usize(i, j, i, j + 1));

            offset = 1;
        }

        let r = fastrand::f32();
        if r < spread {
            if self.cells[i - 1][j + offset].element_data.state == State::Gas && self.cells[i][j + offset].element_data.cell_type != self.cells[i - 1][j + offset].element_data.cell_type {
                self.swaps.push(Swap::new_usize(i, j + offset, i - 1, j + offset));
            }
        } else if r > 1f32 - spread {
            if self.cells[i + 1][j + offset].element_data.state == State::Gas && self.cells[i][j + offset].element_data.cell_type != self.cells[i + 1][j + offset].element_data.cell_type {
                self.swaps.push(Swap::new_usize(i, j + offset, i + 1, j + offset));
            }
        }
    }
    
    fn burn(&mut self, i: usize, j: usize, fire_probability: f32, fire_lifetime: i16) {
        if self.cells[i + 1][j].element_data.cell_type == CellType::Air && fire_probability > fastrand::f32() {
            self.cells[i + 1][j] = Cell::new(ElementData::fire_element(fire_lifetime));
        }
        if self.cells[i - 1][j].element_data.cell_type == CellType::Air && fire_probability > fastrand::f32() {
            self.cells[i - 1][j] = Cell::new(ElementData::fire_element(fire_lifetime));
        }
        if self.cells[i][j + 1].element_data.cell_type == CellType::Air && fire_probability > fastrand::f32() {
            self.cells[i][j + 1] = Cell::new(ElementData::fire_element(fire_lifetime));
        }
        if self.cells[i][j - 1].element_data.cell_type == CellType::Air && fire_probability > fastrand::f32() {
            self.cells[i][j - 1] = Cell::new(ElementData::fire_element(fire_lifetime));
        }
    }
        
    pub fn modify_cell(&mut self, x: i32, y: i32, cell: Cell, brush_size: i32) {
        for i in 0..brush_size {
            for j in 0..brush_size {
                if x + i > 0 && x + i < self.cols as i32 - 1 && y + j > 0 && y + j < self.rows as i32 - 1 {
                    self.cells[(x + i) as usize][(y + j) as usize] = cell.clone();
                }
            }
        }
    }

    pub fn num_of_swaps(&self) -> &u32 {
        &self.num_of_swaps
    }

    pub fn get_size(&self) -> &f32 {
        &self.size
    }
    
    pub fn get_element_on_mouse(&self, i: i32, j: i32) -> Option<CellType> {
        if i >= 0 && i < self.cols as i32 && j >= 0 && j < self.rows as i32 {
            return Some(self.cells[i as usize][j as usize].element_data.cell_type);
        }
        
        None
    }


}

fn map(value: &f32, begin: f32, end: f32, new_begin: f32, new_end: f32) -> f32 {
    new_begin + (new_end - new_begin) * ((value - begin) / (end - begin))
}

#[derive(Copy, Clone)]
struct Swap {
    i1: u32,
    j1: u32,
    i2: u32,
    j2: u32
}

impl Swap {
    pub fn new_u32(i1: u32, j1: u32, i2: u32, j2: u32) -> Self {
        Self { i1, j1, i2, j2 }
    }

    pub fn new_usize(i1: usize, j1: usize, i2: usize, j2: usize) -> Self {
        Self { i1: i1.try_into().unwrap(), j1: j1.try_into().unwrap(), i2: i2.try_into().unwrap(), j2: j2.try_into().unwrap() }
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub element_data: ElementData,
    pub heat_value: f32,
    pub active: bool
}

impl Cell {
    pub fn default() -> Self {
        Self { element_data: ElementData::air_element(), heat_value: 0f32, active: false }
    }

    pub fn new(element_data: ElementData) -> Self {
        Self { element_data, heat_value: 0f32, active: false }
    }
}


#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Eq)]
pub enum CellType {
        Air, Sand, Solid, Water, Steam, Fire, Coal, SawDust, Methane, Lava, ColdFire, Ice, Gasoline
}

#[derive(Copy, Clone)]
pub struct ElementData {
    pub cell_type: CellType,
    pub color: (u8, u8, u8, u8),
    pub state: State,
    pub lifetime: i16,
    pub emitting_heat: f32
}

impl ElementData {
    pub fn air_element() -> Self {
        Self { cell_type: CellType::Air, color: (0, 0, 0, 0), state: State::Gas, lifetime: -1, emitting_heat: 0f32 }
    }

    pub fn sand_element() -> Self {
        Self { cell_type: CellType::Sand, color: (243, 239, 118, 255), state: State::Solid, lifetime: -1, emitting_heat: 0f32 }
    }
    pub fn solid_element() -> Self {
        Self { cell_type: CellType::Solid, color: (69, 62, 66, 255), state: State::Solid, lifetime: -1, emitting_heat: 0f32 }
    }

    pub fn water_element() -> Self {
        Self { cell_type: CellType::Water, color: (18, 24, 204, 255), state: State::Liquid, lifetime: -1, emitting_heat: 0f32 }
    }

    pub fn steam_element() -> Self {
        Self { cell_type: CellType::Steam, color: (195, 225, 247, 255), state: State::Gas, lifetime: -1, emitting_heat: 0f32 }
    }
    pub fn fire_element(lifetime: i16) -> Self {
        Self { cell_type: CellType::Fire, color: (214, 32, 19, 255), state: State::Plasma, lifetime, emitting_heat: 1000f32 }
    }

    pub fn coal_element() -> Self {
        Self { cell_type: CellType::Coal, color: (30, 30, 29, 255), state: State::Solid, lifetime: -1, emitting_heat: 0f32 }
    }
    
    pub fn sawdust_element() -> Self {
        Self { cell_type: CellType::SawDust, color: (219, 199, 120, 255), state: State::Solid, lifetime: -1, emitting_heat: 0f32 }
    }
    
    pub fn methane_element() -> Self {
        Self { cell_type: CellType::Methane, color: (133, 191, 47, 255), state: State::Gas, lifetime: -1, emitting_heat: 0f32 }
    }
    
    pub fn lava_element() -> Self {
        Self { cell_type: CellType::Lava, color: (150, 59, 28, 255), state: State::Liquid, lifetime: -1, emitting_heat: 2000f32 }
    }
    
    pub fn coldfire_element(lifetime: i16) -> Self {
        Self { cell_type: CellType::ColdFire, color: (59, 205, 219, 255), state: State::Plasma, lifetime, emitting_heat: -1000f32 }
    }
    
    pub fn ice_element() -> Self {
        Self { cell_type: CellType::Ice, color: (112, 169, 229, 255), state: State::Solid, lifetime: -1, emitting_heat: -8f32 }
    }
    
    pub fn gasoline_element() -> Self {
        Self { cell_type: CellType::Gasoline, color: (220, 207, 61, 255), state: State::Solid, lifetime: -1, emitting_heat: 0f32 }
    }
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum State {
    Solid, Liquid, Gas, Plasma
}

fn rgb(r: u8, g: u8, b: u8, a: u8) -> (f32, f32, f32, f32) {
    (r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32, a as f32 / 255f32)
}
