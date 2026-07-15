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

    let poligono2 = [
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];

    fb.set_current_color(Color::BLUE);
    fill_polygon(&mut fb, &poligono2);
    fb.set_current_color(Color::WHITE);
    draw_polygon(&mut fb, &poligono2);

    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");
    println!("Imagenes generadas: out.png, out.bmp");
}
