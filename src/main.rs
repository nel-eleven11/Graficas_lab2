use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

// Dimensiones de la cuadrícula para el Juego de la Vida
const GRID_WIDTH: usize = 500;
const GRID_HEIGHT: usize = 500;

// Inicializa un patrón en la cuadrícula (ejemplo: "Glider")
fn initialize_pattern(grid: &mut Vec<Vec<bool>>) {
    // Limpia la cuadrícula
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = false;
        }
    }

    // Patrón "Glider"
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;
}

// Cuenta los vecinos vivos de una célula
fn count_live_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue; // Ignora la célula actual
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

            // Aplica las reglas del Juego de la Vida
            next[y][x] = match (current[y][x], live_neighbors) {
                (true, 2) | (true, 3) => true,  // Sobrevive
                (false, 3) => true,            // Reproducción
                _ => false,                    // Muere
            };
        }
    }
}

// Renderiza la cuadrícula en el framebuffer
fn render_grid(framebuffer: &mut Framebuffer, grid: &Vec<Vec<bool>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y][x] {
                framebuffer.point(x, y, 0xFFFFFF); // Vivo: blanco
            } else {
                framebuffer.point(x, y, 0x000000); // Muerto: negro
            }
        }
    }
}

fn main() {
    // Tamaño del framebuffer
    let framebuffer_width = GRID_WIDTH;
    let framebuffer_height = GRID_HEIGHT;

    let frame_delay = Duration::from_millis(100);

    // Crear framebuffer
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    // Crear ventana
    let mut window = Window::new(
        "Juego de la Vida de Conway",
        framebuffer_width,
        framebuffer_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Inicializar la cuadrícula
    let mut current_grid = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];
    let mut next_grid = current_grid.clone();
    initialize_pattern(&mut current_grid);

    // Bucle principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Renderizar la cuadrícula
        render_grid(&mut framebuffer, &current_grid);

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        // Avanzar al siguiente turno
        game_of_life_step(&current_grid, &mut next_grid);

        // Intercambiar cuadrículas
        std::mem::swap(&mut current_grid, &mut next_grid);

        // Esperar antes de renderizar el siguiente frame
        std::thread::sleep(frame_delay);
    }
}
