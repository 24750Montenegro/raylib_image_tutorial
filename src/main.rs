mod framebuffer;
mod line;
mod draw_polygon;
mod fill;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use draw_polygon::draw_polygon;
use fill::fill_polygon;

fn main() {
    let mut fb = Framebuffer::new(800, 600, Color::BLACK);

    let poligono3 = [
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];

    fb.set_current_color(Color::RED);
    fill_polygon(&mut fb, &poligono3);
    fb.set_current_color(Color::WHITE);
    draw_polygon(&mut fb, &poligono3);

    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");
    println!("Imagenes generadas: out.png, out.bmp");
}
