use macroquad::prelude::*;
use std::thread;
use std::time::Duration;

const SQUARE_SIZE: f32 = 20.;

fn conf() -> Conf {
    Conf {
        window_title: "Conway's game of life".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

fn draw_grid() {
    let row_num = (screen_width() / SQUARE_SIZE) as usize + 1;
    let col_num = (screen_height() / SQUARE_SIZE) as usize + 1;

    let (mut x, mut y) = (0., 0.);
    for _ in 0..col_num {
        draw_line(0., y, screen_width(), y, 2., DARKGRAY);
        y += SQUARE_SIZE;
    }

    for _ in 0..row_num {
        draw_line(x, 0., x, screen_height(), 2., DARKGRAY);
        x += SQUARE_SIZE;
    }
}

enum GameState {
    Menu,
    Running,
    Building,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

struct Cell {
    x: f32,
    y: f32,
    index: usize,
}

impl Cell {
    fn new(x: f32, y: f32, index: usize) -> Cell {
        Cell { x, y, index }
    }

    fn toggle_state(&self, cell_states: &mut Vec<CellState>) {
        match cell_states[self.index] {
            CellState::Alive => {
                cell_states[self.index] = CellState::Dead;
            }
            CellState::Dead => {
                cell_states[self.index] = CellState::Alive;
            }
        }
    }

    fn draw(&self, cell_state: &Vec<CellState>) {
        match cell_state[self.index] {
            CellState::Alive => {
                draw_rectangle(self.x, self.y, SQUARE_SIZE, SQUARE_SIZE, WHITE);
            }
            _ => {}
        }
    }

    fn is_clicked(&self) -> bool {
        let (mx, my) = mouse_position();
        let sq = Rect::new(self.x, self.y, SQUARE_SIZE, SQUARE_SIZE);
        let m_rect = Rect::new(mx, my, 0.01, 0.01);
        return m_rect.intersect(sq).is_some();
    }

    fn handle_state(&mut self, states: &mut Vec<CellState>) {
        if self.is_clicked() {
            self.toggle_state(states);
        }
    }
}

fn switch_game_state(game_state: &mut GameState) {
    if is_key_pressed(KeyCode::Space) {
        match game_state {
            GameState::Menu => {
                *game_state = GameState::Building;
            }
            GameState::Building => {
                *game_state = GameState::Running;
            }
            GameState::Running => {
                *game_state = GameState::Building;
            }
        }
    }
}

fn init_cells() -> Vec<Cell> {
    let w = (screen_width() / SQUARE_SIZE) as usize + 1;
    let h = (screen_height() / SQUARE_SIZE) as usize + 1;
    let mut x = 0.;
    let mut y = 0.;
    let mut out: Vec<Cell> = Vec::new();
    let mut index = 0 as usize;
    for _ in 0..h {
        for _ in 0..w {
            out.push(Cell::new(x, y, index));
            x += SQUARE_SIZE;
            index += 1
        }
        x = 0.;
        y += SQUARE_SIZE;
    }
    out
}

fn game_logic(cells: &mut Vec<CellState>) {
    let w = (screen_width() / SQUARE_SIZE) as usize + 1;
    let h = (screen_height() / SQUARE_SIZE) as usize + 1;
    let mut buffer = cells.clone();
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            let mut neighbors_count = 0;

            for j in -1i32..=1 {
                for i in -1i32..=1 {
                    // out of bounds
                    if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
                        continue;
                    }
                    // cell itself
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let neighbor = cells[(y + j) as usize * w + (x + i) as usize];
                    if neighbor == CellState::Alive {
                        neighbors_count += 1;
                    }
                }
            }
            let current_cell = cells[y as usize * w + x as usize];
            buffer[y as usize * w + x as usize] = match (current_cell, neighbors_count) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (CellState::Alive, x) if x < 2 => CellState::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (CellState::Alive, x) if x > 3 => CellState::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (CellState::Dead, 3) => CellState::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };        
            }
    }
    
 *cells = buffer;
}

fn reset_board(cells:&mut Vec<CellState>) {
    let w = (screen_width() / SQUARE_SIZE) as usize + 1;
    let h = (screen_height() / SQUARE_SIZE) as usize + 1;
    if is_key_pressed(KeyCode::R) {
        *cells = vec![CellState::Dead;w*h];
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut game_state = GameState::Menu;
    let w = (screen_width() / SQUARE_SIZE) as usize + 1;
    let h = (screen_height() / SQUARE_SIZE) as usize + 1;
    let mut cell_states = vec![CellState::Dead; w * h];
    let mut cells = init_cells();
    let mut render_delay = Duration::from_millis(0);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        switch_game_state(&mut game_state);
        clear_background(BLACK);
        match game_state {
            GameState::Menu => {
                draw_text(
                    "Conway's Game of Life",
                    screen_width() / 2. - 250.,
                    screen_height() / 2.,
                    50.,
                    WHITE,
                );
                draw_text(
                    "Press Space to build/run/play",
                    screen_width() / 2. - 190.,
                    screen_height() / 2. + 100.,
                    30.,
                    WHITE,
                );
                
                draw_text(
                    "Escape to quit",
                    screen_width() / 2. - 150.,
                    screen_height() / 2. + 200.,
                    30.,
                    WHITE,
                );
                
                
                draw_text(
                    "R to reset",
                    screen_width() / 2. - 150.,
                    screen_height() / 2. + 300.,
                    30.,
                    WHITE,
                );
                
            }
            GameState::Running => {
                render_delay = Duration::from_millis(100);
                draw_grid();
                for cell in cells.iter() {
                    cell.draw(&cell_states);
                }
                reset_board(&mut cell_states);
                game_logic(&mut cell_states);
            }
            GameState::Building => {
                draw_grid();
                for cell in cells.iter_mut() {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        cell.handle_state(&mut cell_states);
                    }
                    cell.draw(&cell_states);
                }
                reset_board(&mut cell_states);
                render_delay = Duration::from_millis(50);
            }
        }
        //draw_grid();
        next_frame().await;
        thread::sleep(render_delay);
    }
}
