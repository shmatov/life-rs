use std::collections::HashSet;
use std::iter::IntoIterator;
use ::iterator_ext::IteratorExt;


pub type Cell = (i32, i32);  // row, col


fn get_neighbours(cell: &Cell) -> impl Iterator<Item=Cell> {
    let &(x, y) = cell;
    (-1...1)
        .flat_map(move |dx| (-1...1).map(move |dy| (x + dx, y + dy)))
        .filter(move |neighbour| neighbour != &(x, y))
}

pub struct Life {
    alive_cells: HashSet<Cell>,
}

impl Life {
    pub fn new<I>(cells: I) -> Self where I: IntoIterator<Item=Cell> {
        Life {
            alive_cells: cells.into_iter().collect::<HashSet<Cell>>()
        }
    }
}

pub enum CellType {
    Alive,
    Dead
}

pub trait Engine {
    type State : State;

    fn make_step(&self) -> Self;
    fn get_state(&self) -> &Self::State;
}

pub trait State {
    fn get_cell_type(&self, &Cell) -> CellType;
}

fn will_alive(alive_cells: &HashSet<Cell>, cell: Cell) -> Option<Cell> {
    let alive_neighbours_count = get_neighbours(&cell).filter(|x| alive_cells.contains(x)).count();
    match (alive_cells.contains(&cell), alive_neighbours_count) {
        (true, 2) | (_, 3) => Some(cell),
        _ => None
    }
}


impl Engine for Life {
    type State = Life;

    fn make_step(&self) -> Self {
        let alive_cells = self.alive_cells.iter()
            .flat_map(get_neighbours)
            .unique()
            .filter_map(|x| will_alive(&self.alive_cells, x))
            .collect::<HashSet<Cell>>();

        Life { alive_cells: alive_cells }
    }

    fn get_state(&self) -> &Life { self }
}


impl State for Life {
    fn get_cell_type(&self, cell: &Cell) -> CellType {
        match self.alive_cells.contains(cell) {
            true => CellType::Alive,
            false => CellType::Dead,
        }
    }
}
