// main.rs

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

// Dimensiones de la cuadrícula para el Juego de la Vida
const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;
const SCALE_FACTOR: usize = 10; // Escala para hacer las células más grandes en la ventana

// Función para inicializar un "Glider"
fn glider(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    grid[start_y][start_x + 1] = true;
    grid[start_y + 1][start_x + 2] = true;
    grid[start_y + 2][start_x] = true;
    grid[start_y + 2][start_x + 1] = true;
    grid[start_y + 2][start_x + 2] = true;
}

// Función para inicializar un "Light-weight spaceship" (LWSS)
fn lwss(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    grid[start_y][start_x + 1] = true;
    grid[start_y][start_x + 2] = true;
    grid[start_y][start_x + 3] = true;
    grid[start_y + 1][start_x] = true;
    grid[start_y + 1][start_x + 3] = true;
    grid[start_y + 2][start_x + 3] = true;
    grid[start_y + 3][start_x] = true;
    grid[start_y + 3][start_x + 2] = true;
}

// Función para inicializar un "Block" (Still Life)
fn block(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    grid[start_y][start_x] = true;
    grid[start_y][start_x + 1] = true;
    grid[start_y + 1][start_x] = true;
    grid[start_y + 1][start_x + 1] = true;
}

// Función para inicializar un "Beacon" (Oscillator)
fn beacon(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    grid[start_y][start_x] = true;
    grid[start_y][start_x + 1] = true;
    grid[start_y + 1][start_x] = true;
    grid[start_y + 1][start_x + 1] = true;

    grid[start_y + 2][start_x + 2] = true;
    grid[start_y + 2][start_x + 3] = true;
    grid[start_y + 3][start_x + 2] = true;
    grid[start_y + 3][start_x + 3] = true;
}

// Función para inicializar un "Toad" (Oscillator)
fn toad(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    grid[start_y][start_x + 1] = true;
    grid[start_y][start_x + 2] = true;
    grid[start_y][start_x + 3] = true;

    grid[start_y + 1][start_x] = true;
    grid[start_y + 1][start_x + 1] = true;
    grid[start_y + 1][start_x + 2] = true;
}

// Función para inicializar un "Bee-hive" (Still Life)
fn bee_hive(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    grid[start_y][start_x + 1] = true;
    grid[start_y][start_x + 2] = true;
    grid[start_y + 1][start_x] = true;
    grid[start_y + 1][start_x + 3] = true;
    grid[start_y + 2][start_x + 1] = true;
    grid[start_y + 2][start_x + 2] = true;
}

// Función para inicializar un "Loaf" (Still Life)
fn loaf(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    grid[start_y][start_x + 1] = true;
    grid[start_y][start_x + 2] = true;
    grid[start_y + 1][start_x] = true;
    grid[start_y + 1][start_x + 3] = true;
    grid[start_y + 2][start_x + 1] = true;
    grid[start_y + 2][start_x + 3] = true;
    grid[start_y + 3][start_x + 2] = true;
}

// Función para inicializar un "Tub" (Still Life)
fn tub(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    grid[start_y][start_x + 1] = true;
    grid[start_y + 1][start_x] = true;
    grid[start_y + 1][start_x + 2] = true;
    grid[start_y + 2][start_x + 1] = true;
}

// Función para inicializar un "Pulsar" (Oscillator)
fn pulsar(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize) {
    for i in 0..3 {
        grid[start_y + i][start_x + 2] = true;
        grid[start_y + i][start_x + 6] = true;
        grid[start_y + 4 + i][start_x + 2] = true;
        grid[start_y + 4 + i][start_x + 6] = true;

        grid[start_y + 2][start_x + i] = true;
        grid[start_y + 6][start_x + i] = true;
        grid[start_y + 2][start_x + 4 + i] = true;
        grid[start_y + 6][start_x + 4 + i] = true;
    }
}

// Inicializa el patrón con múltiples organismos
fn initialize_pattern(grid: &mut Vec<Vec<bool>>) {
    glider(grid, 2, 2);
    glider(grid, 10, 10);
    lwss(grid, 15, 5);
    lwss(grid, 30, 20);
    block(grid, 5, 25);
    block(grid, 40, 40);
    bee_hive(grid, 25, 10);
    bee_hive(grid, 35, 15);
    loaf(grid, 20, 30);
    tub(grid, 15, 35);
    pulsar(grid, 10, 40);
    beacon(grid, 5, 5);
    toad(grid, 35, 25);
}

// Cuenta los vecinos vivos de una célula
fn count_live_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < GRID_WIDTH as isize && ny >= 0 && ny < GRID_HEIGHT as isize {
                if grid[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

// Realiza un turno del Juego de la Vida
fn game_of_life_step(current: &Vec<Vec<bool>>, next: &mut Vec<Vec<bool>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let live_neighbors = count_live_neighbors(current, x, y);
            next[y][x] = match (current[y][x], live_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
}

// Renderiza la cuadrícula en el framebuffer con escala
fn render_grid(framebuffer: &mut Framebuffer, grid: &Vec<Vec<bool>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let color = if grid[y][x] { 0xFFFFFF } else { 0x000000 };
            for sy in 0..SCALE_FACTOR {
                for sx in 0..SCALE_FACTOR {
                    framebuffer.point(
                        x * SCALE_FACTOR + sx,
                        y * SCALE_FACTOR + sy,
                        color,
                    );
                }
            }
        }
    }
}

fn main() {
    let framebuffer_width = GRID_WIDTH * SCALE_FACTOR;
    let framebuffer_height = GRID_HEIGHT * SCALE_FACTOR;
    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Juego de la Vida de Conway",
        framebuffer_width,
        framebuffer_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut current_grid = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];
    let mut next_grid = current_grid.clone();
    initialize_pattern(&mut current_grid);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render_grid(&mut framebuffer, &current_grid);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        game_of_life_step(&current_grid, &mut next_grid);
        std::mem::swap(&mut current_grid, &mut next_grid);

        std::thread::sleep(frame_delay);
    }
}
