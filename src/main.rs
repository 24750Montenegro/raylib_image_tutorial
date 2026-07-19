mod framebuffer;
mod line;
mod draw_polygon;
mod fill;

use raylib::prelude::*;
use framebuffer::Framebuffer;

fn main() {
    let mut fb = Framebuffer::new(800, 600, Color::BLACK);

    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");
    println!("Imagenes generadas: out.png, out.bmp");
}
