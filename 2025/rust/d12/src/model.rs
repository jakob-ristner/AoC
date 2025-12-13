use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Shape {
    c1: bool,
    c2: bool,
    c3: bool,
    c4: bool,
    c5: bool,
    c6: bool,
    c7: bool,
    c8: bool,
    c9: bool,
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        let mut cells = s
            .chars()
            .filter(|&c| c == '#' || c == '.')
            .map(|c| c == '#');
        Shape {
            c1: cells.next().unwrap_or(false),
            c2: cells.next().unwrap_or(false),
            c3: cells.next().unwrap_or(false),
            c4: cells.next().unwrap_or(false),
            c5: cells.next().unwrap_or(false),
            c6: cells.next().unwrap_or(false),
            c7: cells.next().unwrap_or(false),
            c8: cells.next().unwrap_or(false),
            c9: cells.next().unwrap_or(false),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..3 {
            for col in 0..3 {
                let cell = match (row, col) {
                    (0, 0) => self.c1,
                    (0, 1) => self.c2,
                    (0, 2) => self.c3,
                    (1, 0) => self.c4,
                    (1, 1) => self.c5,
                    (1, 2) => self.c6,
                    (2, 0) => self.c7,
                    (2, 1) => self.c8,
                    (2, 2) => self.c9,
                    _ => unreachable!(),
                };
                let symbol = if cell { '#' } else { '.' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Shape {
    pub fn set_piece_constraint(
        options: Vec<Vec<usize>>,
        grid_width: usize,
        grid_height: usize,
        piece_index: usize,
    ) -> Vec<Vec<usize>> {
        options
            .into_iter()
            .map(|mut option| {
                option.push(grid_width * grid_height + piece_index + 1);
                option
            })
            .collect()
    }

    pub fn area(&self) -> usize {
        self.c1 as usize
            + self.c2 as usize
            + self.c3 as usize
            + self.c4 as usize
            + self.c5 as usize
            + self.c6 as usize
            + self.c7 as usize
            + self.c8 as usize
            + self.c9 as usize
    }

    pub fn all_variant_options(&self, grid_width: usize, grid_height: usize) -> Vec<Vec<usize>> {
        let mut all_options = Vec::new();
        for variant in self.all_variants() {
            let mut options = variant.all_options(grid_width, grid_height);
            all_options.append(&mut options);
        }
        all_options
    }

    fn all_options(&self, grid_width: usize, grid_height: usize) -> Vec<Vec<usize>> {
        let mut options = Vec::new();
        for row in 0..=grid_height.saturating_sub(3) {
            for col in 0..=grid_width.saturating_sub(3) {
                if let Some(option) = self.to_option(row, col, grid_width, grid_height) {
                    options.push(option);
                }
            }
        }
        options
    }

    fn to_option(
        &self,
        top_left_row: usize,
        top_left_col: usize,
        grid_width: usize,
        grid_height: usize,
    ) -> Option<Vec<usize>> {
        let mut option = Vec::new();
        let mut valid = true;
        for row in 0..3 {
            for col in 0..3 {
                let cell_filled = match (row, col) {
                    (0, 0) => self.c1,
                    (0, 1) => self.c2,
                    (0, 2) => self.c3,
                    (1, 0) => self.c4,
                    (1, 1) => self.c5,
                    (1, 2) => self.c6,
                    (2, 0) => self.c7,
                    (2, 1) => self.c8,
                    (2, 2) => self.c9,
                    _ => unreachable!(),
                };
                if cell_filled {
                    let grid_row = top_left_row + row;
                    let grid_col = top_left_col + col;
                    if grid_row >= grid_height || grid_col >= grid_width {
                        valid = false;
                        break;
                    }
                    let col_index = grid_row * grid_width + grid_col + 1;
                    option.push(col_index);
                }
            }
            if !valid {
                break;
            }
        }
        if valid { Some(option) } else { None }
    }

    fn all_variants(&self) -> Vec<Shape> {
        fn rotate_90(shape: Shape) -> Shape {
            Shape {
                c1: shape.c7,
                c2: shape.c4,
                c3: shape.c1,
                c4: shape.c8,
                c5: shape.c5,
                c6: shape.c2,
                c7: shape.c9,
                c8: shape.c6,
                c9: shape.c3,
            }
        }

        fn reflect_horizontally(shape: Shape) -> Shape {
            Shape {
                c1: shape.c3,
                c2: shape.c2,
                c3: shape.c1,
                c4: shape.c6,
                c5: shape.c5,
                c6: shape.c4,
                c7: shape.c9,
                c8: shape.c8,
                c9: shape.c7,
            }
        }

        let mut variants = Vec::new();
        let mut current = self.clone();
        for _ in 0..4 {
            variants.push(current);
            current = rotate_90(current);
        }
        current = reflect_horizontally(self.clone());
        for _ in 0..4 {
            variants.push(current);
            current = rotate_90(current);
        }
        variants.sort();
        variants.dedup();
        return variants;
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub shapes: Vec<(Shape, usize)>,
    pub grid_width: usize,
    pub grid_height: usize,
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid: {} x {}", self.grid_width, self.grid_height)?;
        for (shape, count) in &self.shapes {
            writeln!(f, "Count: {}", count)?;
            writeln!(f, "{}", shape)?;
        }
        Ok(())
    }
}
