use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

// TODO: abstract the indexing

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GolCell {
    Dead,
    Alive,
}

impl From<bool> for GolCell {
    #[inline]
    fn from(b: bool) -> Self {
        if b {
            GolCell::Alive
        } else {
            GolCell::Dead
        }
    }
}

macro_rules! impl_integer_to_golcell {
    ( $( $t:ty ),* ) => {
        $(
        impl From<$t> for GolCell {
            #[inline]
            fn from(x: $t) -> Self {
                if x == 0 {
                    GolCell::Dead
                } else {
                    GolCell::Alive
                }
            }
        }
        )*
    }
}

impl_integer_to_golcell! {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize}

impl GolCell {
    pub fn is_dead(&self) -> bool {
        match *self {
            GolCell::Dead => true,
            GolCell::Alive => false,
        }
    }

    pub fn is_alive(&self) -> bool {
        match *self {
            GolCell::Dead => false,
            GolCell::Alive => true,
        }
    }

    pub fn flip(&mut self) {
        *self = match *self {
            GolCell::Dead => GolCell::Alive,
            GolCell::Alive => GolCell::Dead,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GolBoard {
    height: usize,
    width: usize,
    cells: HashSet<(usize, usize)>,
}

impl GolBoard {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            cells: HashSet::new(),
        }
    }

    pub fn new_square(size: usize) -> Self {
        Self::new(size, size)
    }

    pub fn from_slice<T, U>(matrix: &[U]) -> Self
    where
        T: Clone + Into<GolCell>,
        U: AsRef<[T]>,
    {
        let mut result = Self {
            height: 0,
            width: 0,
            cells: HashSet::new(),
        };

        let height = matrix.len();

        if height > 0 {
            result.height = height;
            result.width = matrix[0].as_ref().len();
            for r in 0..matrix.len() {
                let row = matrix[r].as_ref();
                for c in 0..row.len() {
                    if row[c].clone().into().is_alive() {
                        result.cells.insert((r, c));
                    }
                }
            }
        }
        result
    }

    pub fn from_alive_list<T>(height: usize, width: usize, alive_locs: T) -> Self
    where
        T: IntoIterator<Item = (usize, usize)>,
    {
        Self {
            height,
            width,
            cells: alive_locs.into_iter().collect(),
        }
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn alive_cells(&self) -> impl Iterator<Item = (usize, usize)> {
        self.cells.iter().copied().collect::<Vec<_>>().into_iter()
    }

    fn neighbor_indices(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        const NEIGHBORS: &[(isize, isize)] = &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            //(0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        const MAX_NEIGHBORS: usize = NEIGHBORS.len();
        let mut result = Vec::with_capacity(MAX_NEIGHBORS);
        let height: isize = self.height.try_into().unwrap();
        let width: isize = self.width.try_into().unwrap();
        let row: isize = row.try_into().unwrap();
        let col: isize = col.try_into().unwrap();
        for (r, c) in NEIGHBORS {
            let (r, c) = (row + r, col + c);
            if r >= 0 && r < height && c >= 0 && c < width {
                let (r, c) = (r.try_into().unwrap(), c.try_into().unwrap());
                result.push((r, c));
            }
        }
        result.into_iter()
    }

    pub fn process_step(&mut self) {
        let mut live_neighbor_counts = HashMap::new();
        for (r, c) in self.cells.iter() {
            live_neighbor_counts.entry((*r, *c)).or_insert(0);
            for (r, c) in self.neighbor_indices(*r, *c) {
                let count = live_neighbor_counts.entry((r, c)).or_insert(0);
                *count += 1;
            }
        }

        for ((r, c), count) in live_neighbor_counts {
            if count < 2 || count > 3 {
                self.cells.remove(&(r, c));
            } else if count == 3 {
                self.cells.insert((r, c));
            }
        }
    }
}
