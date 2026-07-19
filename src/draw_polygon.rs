use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

/// Dibuja el contorno de un poligono conectando cada punto con el siguiente
/// mediante `line`, y cierra la figura (ultimo punto -> primer punto).
/// Usa el color actual del framebuffer.
pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 2 {
        return;
    }

    let n = points.len();
    for i in 0..n {
        let start = points[i];
        let end = points[(i + 1) % n]; // el % cierra el poligono
        line(framebuffer, start, end);
    }
}
