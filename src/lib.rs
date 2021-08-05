use std::convert::TryInto;

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
    pub fn is_dead(self) -> bool {
        match self {
            GolCell::Dead => true,
            GolCell::Alive => false,
        }
    }

    pub fn is_alive(self) -> bool {
        match self {
            GolCell::Dead => false,
            GolCell::Alive => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GolBoard {
    height: usize,
    width: usize,
    cells: Vec<GolCell>,
}

impl GolBoard {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            cells: vec![GolCell::Dead; height * width],
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
            cells: Vec::new(),
        };

        let height = matrix.len();

        if height > 0 {
            result.height = height;
            result.width = matrix[0].as_ref().len();
            result.cells.reserve_exact(result.height * result.width);
            for row in matrix.iter() {
                result
                    .cells
                    .extend(row.as_ref().iter().map(|x| x.clone().into()));
            }
        }
        result
    }

    fn neighbor_indices(&self, i: usize) -> impl Iterator<Item = usize> {
        let mut result = Vec::new();
        let (row, col) = (i / self.width, i % self.width);
        let row: isize = row.try_into().unwrap();
        let col: isize = col.try_into().unwrap();
        let height: isize = self.height.try_into().unwrap();
        let width: isize = self.width.try_into().unwrap();
        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }
                let (r, c) = (row + r, col + c);
                if r >= 0 && r < height && c >= 0 && c < width {
                    result.push((r * width + c).try_into().unwrap());
                }
            }
        }
        result.into_iter()
    }

    pub fn process_step(&mut self) {
        let mut fliplist = Vec::new();
        for i in 0..self.cells.len() {
            let living_neighbor_count = self
                .neighbor_indices(i)
                .filter(|&i| self.cells[i].is_alive())
                .count();

            match self.cells[i] {
                GolCell::Dead => {
                    if living_neighbor_count == 3 {
                        fliplist.push(i);
                    }
                }
                GolCell::Alive => {
                    if living_neighbor_count != 2 && living_neighbor_count != 3 {
                        fliplist.push(i);
                    }
                }
            }
        }

        for i in fliplist {
            self.cells[i] = match self.cells[i] {
                GolCell::Dead => GolCell::Alive,
                GolCell::Alive => GolCell::Dead,
            }
        }
    }
}
