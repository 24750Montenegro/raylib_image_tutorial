mod framebuffer;
mod line;
mod draw_polygon;
mod fill;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use draw_polygon::draw_polygon;
use fill::fill_polygon_with_holes;

fn main() {
    let mut fb = Framebuffer::new(800, 600, Color::BLACK);

    let poligono4 = [
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0),
        Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),
        Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0),
        Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0),
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0),
        Vector2::new(466.0, 180.0),
    ];

    let poligono5 = [
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];

    fb.set_current_color(Color::GREEN);
    fill_polygon_with_holes(&mut fb, &poligono4, &[&poligono5]);
    fb.set_current_color(Color::WHITE);
    draw_polygon(&mut fb, &poligono4);
    draw_polygon(&mut fb, &poligono5);

    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");
    println!("Imagenes generadas: out.png, out.bmp");
}
