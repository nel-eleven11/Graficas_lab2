use minifb::{Window, WindowOptions};
use std::time::Duration;
mod framebuffer;

fn main() {
  let window_width = 500;
  let window_height = 600;

  let framebuffer_width = 50;
  let framebuffer_height = 60;

  let close_delay = Duration::from_secs(10);

  let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

  let mut window = Window::new(
    "Rust Graphics - Framebuffer Example",
    window_width,
    window_height,
    WindowOptions::default(),
  ).unwrap();

  // Clear the framebuffer
  framebuffer.set_background_color(0x333355);
  framebuffer.clear();

  // Draw a point
  framebuffer.set_current_color(0xFFDDDD);
  framebuffer.point(1, 1);

  // Update the window with the framebuffer contents
  window
   .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
   .unwrap();

  std::thread::sleep(close_delay);
}