use fxhash::{FxHashMap, FxHashSet};

fn solution_to_piece_id_matrix(
    solution: &[Vec<usize>],
    grid_width: usize,
    grid_height: usize,
) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; grid_width]; grid_height];
    let mut piece_id = 1;
    for option in solution.iter() {
        if option.len() <= 1 {
            continue;
        }
        for &col in option {
            if col <= grid_height * grid_width {
                let row = (col - 1) / grid_width;
                let col_idx = (col - 1) % grid_width;
                matrix[row][col_idx] = piece_id;
            }
        }
        piece_id += 1;
    }
    matrix
}

fn color_matrix_ids(matrix: &[Vec<usize>]) -> FxHashMap<usize, usize> {
    if matrix.is_empty() {
        return FxHashMap::default();
    }
    let mut graph: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
    let n = matrix.len();
    let m = matrix[0].len();

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 0 {
                continue;
            }
            let id = matrix[i][j];
            graph.entry(id).or_default().insert(id);

            for (di, dj) in &dirs {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                    let neighbor_id = matrix[ni as usize][nj as usize];
                    if neighbor_id == 0 {
                        continue;
                    }
                    if neighbor_id != id {
                        graph.entry(id).or_default().insert(neighbor_id);
                        graph.entry(neighbor_id).or_default().insert(id);
                    }
                }
            }
        }
    }

    let mut colors: FxHashMap<usize, usize> = FxHashMap::default();
    let ids: Vec<usize> = graph.keys().copied().collect();

    for id in ids {
        let mut used_colors = FxHashSet::default();
        if let Some(neighbors) = graph.get(&id) {
            for neighbor in neighbors {
                if let Some(&color) = colors.get(neighbor) {
                    used_colors.insert(color);
                }
            }
        }

        let mut color = 0;
        while used_colors.contains(&color) {
            color += 1;
        }
        colors.insert(id, color);
    }

    colors
}

const ANSI_COLORS: [&str; 9] = [
    "\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[95m", "\x1b[96m", "\x1b[97m", "\x1b[31m",
    "\x1b[32m",
];
const RESET: &str = "\x1b[0m";

pub fn print_solution_colored(solution: &[Vec<usize>], grid_width: usize, grid_height: usize) {
    fn print_colored(color_id: usize, text: &str) {
        let ansi_color = ANSI_COLORS[color_id % ANSI_COLORS.len()];
        print!("{}{}{}", ansi_color, text, RESET);
    }

    let piece_id_matrix = solution_to_piece_id_matrix(solution, grid_width, grid_height);
    let color_map = color_matrix_ids(&piece_id_matrix);

    print!("╭");
    for _ in 0..grid_width * 2 {
        print!("─");
    }
    println!("╮");

    for row in piece_id_matrix.iter().take(grid_height) {
        print!("│");
        for piece_id in row.iter().take(grid_width) {
            if let Some(&color) = color_map.get(piece_id) {
                print_colored(color, "██");
            } else {
                print!("  ");
            }
        }
        println!("│");
    }

    print!("╰");
    for _ in 0..grid_width * 2 {
        print!("─");
    }
    println!("╯");
}
