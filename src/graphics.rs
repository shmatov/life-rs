use ::core::CellType;


pub trait Graphics {
    fn visualize_state<S: ::core::State>(&self, &S);
}

pub struct CUI {
    min: (i32, i32),
    max: (i32, i32),
}

impl CUI {
    pub fn new(min: (i32, i32), max: (i32, i32)) -> Self {
        CUI { min: min, max: max }
    }
}

use ::std::io::{self, Write};

impl Graphics for CUI {
    fn visualize_state<S: ::core::State>(&self, state: &S) {
        let (min_row, min_col) = self.min;
        let (max_row, max_col) = self.max;

        let mut writer = io::BufWriter::new(io::stdout());
        write!(writer, "{}[2J", 27 as char).unwrap();

        for row in min_row...max_row {
            for col in min_col...max_col {
                let symbol = cell_type_to_symbol(&state.get_cell_type(&(row, col)));
                writer.write_all(&[symbol]).unwrap();
            }
            writer.write_all(b"\n").unwrap();
        }
        writer.flush().unwrap();
    }
}

fn cell_type_to_symbol(cell_type: &CellType) -> u8 {
    match cell_type {
        &CellType::Alive => b'+',
        &CellType::Dead => b' ',
    }
}
