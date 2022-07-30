use std::mem::ManuallyDrop;

use crate::grid::{Cell, ROWS, COLS};

pub fn create_cells_array() -> Box<[[Cell; ROWS as usize]; COLS as usize]> {
    let mut data = ManuallyDrop::new(vec![Cell::default(); ROWS as usize * COLS as usize]);
    
    unsafe {
        Box::from_raw(data.as_mut_ptr() as *mut [[Cell; ROWS as usize]; COLS as usize])
    }
}