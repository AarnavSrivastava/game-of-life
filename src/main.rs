use nannou::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const CELL_SIZE: f32 = 10.0; 

struct Model {
    grid: Vec<Vec<bool>>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size((GRID_WIDTH as f32 * CELL_SIZE) as u32, (GRID_HEIGHT as f32 * CELL_SIZE) as u32)
        .view(view)
        .build()
        .unwrap();

    // Initialize the grid with random on/off cells
    // let grid = (0..GRID_WIDTH)
    //     .map(|_| (0..GRID_HEIGHT).map(|_| random_f32() < 0.1).collect())
    //     .collect();

    // Gosper gun setup
    let mut grid = vec![vec![false; GRID_HEIGHT]; GRID_WIDTH];

    let gosper_glider_gun = vec![
        (0, 5), (0, 6), (1, 5), (1, 6),
        (10, 5), (10, 6), (10, 7),
        (11, 4), (11, 8),
        (12, 3), (12, 9),
        (13, 3), (13, 9),
        (14, 6),
        (15, 4), (15, 8),
        (16, 5), (16, 6), (16, 7),
        (17, 6),
        (20, 3), (20, 4), (20, 5),
        (21, 3), (21, 4), (21, 5),
        (22, 2), (22, 6),
        (24, 1), (24, 2), (24, 6), (24, 7),
        (34, 3), (34, 4), (35, 3), (35, 4),
    ];



    // Set the cells in the grid to `true` (alive) according to the Gosper Glider Gun pattern
    for (x, y) in gosper_glider_gun.iter() {
        grid[*x + 10][GRID_HEIGHT - (*y + 10)] = true;
    }

    Model {
        grid
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut grid = vec![vec![false; GRID_HEIGHT]; GRID_WIDTH];

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let neighbor_count = count_neighbors(&model.grid, x as i32, y as i32);
            
            grid[x][y] = match model.grid[x][y] {
                true => neighbor_count == 2 || neighbor_count == 3,
                false => neighbor_count == 3,
            };
        }
    }

    model.grid = grid
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: i32, y: i32) -> usize {
    let mut count = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let pos_x = x + dx;
            let pos_y = y + dy;

            if pos_x >= 0 && pos_y >= 0 && pos_x < GRID_WIDTH as i32 && pos_y < GRID_HEIGHT as i32 {
                count += if grid[pos_x as usize][pos_y as usize] { 1 } else { 0 }
            }
        }
    }

    count
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let color = if model.grid[x][y] { BLACK } else { WHITE };

            let x_pos = x as f32 * CELL_SIZE - (GRID_WIDTH as f32 * CELL_SIZE / 2.0);
            let y_pos = y as f32 * CELL_SIZE - (GRID_HEIGHT as f32 * CELL_SIZE / 2.0);
            draw.rect()
                .x_y(x_pos, y_pos)
                .w_h(CELL_SIZE, CELL_SIZE)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
