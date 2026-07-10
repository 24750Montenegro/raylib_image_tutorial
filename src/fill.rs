use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

/// Rellena un poligono simple con scanline (regla par-impar).
pub fn fill_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    fill_contours(framebuffer, &[points]);
}

/// Rellena el poligono exterior dejando sin pintar el interior de los agujeros.
pub fn fill_polygon_with_holes(
    framebuffer: &mut Framebuffer,
    outer: &[Vector2],
    holes: &[&[Vector2]],
) {
    let mut contours: Vec<&[Vector2]> = vec![outer];
    contours.extend_from_slice(holes);
    fill_contours(framebuffer, &contours);
}

fn fill_contours(framebuffer: &mut Framebuffer, contours: &[&[Vector2]]) {
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for c in contours {
        for p in *c {
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }
    }
    if min_y > max_y {
        return;
    }
    let min_y = min_y.ceil() as i32;
    let max_y = max_y.floor() as i32;

    for y in min_y..=max_y {
        let yf = y as f32;
        let mut xs: Vec<f32> = Vec::new();

        for c in contours {
            let n = c.len();
            if n < 3 {
                continue;
            }
            for i in 0..n {
                let a = c[i];
                let b = c[(i + 1) % n];
                let (y0, y1) = (a.y, b.y);
                // Regla semiabierta [y0, y1): no cuenta dos veces un vertice.
                let crosses = (yf >= y0 && yf < y1) || (yf >= y1 && yf < y0);
                if crosses {
                    let t = (yf - y0) / (y1 - y0);
                    xs.push(a.x + t * (b.x - a.x));
                }
            }
        }

        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut i = 0;
        while i + 1 < xs.len() {
            let x_start = xs[i].ceil() as i32;
            let x_end = xs[i + 1].floor() as i32;
            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y);
            }
            i += 2;
        }
    }
}
