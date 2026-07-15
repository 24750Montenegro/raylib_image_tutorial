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

    let poligono1 = [
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];

    fb.set_current_color(Color::YELLOW);
    fill_polygon(&mut fb, &poligono1);
    fb.set_current_color(Color::WHITE);
    draw_polygon(&mut fb, &poligono1);

    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");
    println!("Imagenes generadas: out.png, out.bmp");
}
