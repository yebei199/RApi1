use bevy_vector_shapes::prelude::*;

pub mod alpha_mode {
    use super::*;
    use bevy::color::Color;
    use bevy::math::Vec3;
    use bevy_vector_shapes::painter::ShapePainter;
    use bevy_vector_shapes::prelude::ShapeAlphaMode;
    use std::f32::consts::TAU;

    fn draw(mut painter: ShapePainter) {
        // Draw a circle
        painter.circle(100.9);
    }

    fn draw_circles(
        painter: &mut ShapePainter,
        radius: f32,
    ) {
        painter.translate(
            -(Vec3::X + Vec3::NEG_Y)
                * f32::sqrt(radius)
                * 0.5,
        );
        painter.color = Color::srgba(1.0, 0.0, 0.0, 0.5);
        painter.circle(radius);

        painter.rotate_z(-TAU / 3.0);
        painter.translate(
            Vec3::Y * radius * 1.2 + Vec3::Z * 0.0001,
        );
        painter.color = Color::srgba(0.0, 1.0, 0.0, 0.5);
        painter.circle(radius);

        painter.rotate_z(-TAU / 3.0);
        painter.translate(
            Vec3::Y * radius * 1.2 + Vec3::Z * 0.0001,
        );
        painter.color = Color::srgba(0.0, 0.0, 1.0, 0.5);
        painter.circle(radius);
    }
    pub fn draw_gallery(mut painter: ShapePainter) {
        let radius = 200.0;

        painter.reset();
        painter.translate(Vec3::X * radius * -4.0);
        painter.alpha_mode = ShapeAlphaMode::Add;
        draw_circles(&mut painter, radius);

        painter.reset();
        painter.alpha_mode = ShapeAlphaMode::Multiply;
        draw_circles(&mut painter, radius);

        painter.reset();
        painter.translate(Vec3::X * radius * 4.0);
        painter.alpha_mode = ShapeAlphaMode::Blend;
        draw_circles(&mut painter, radius);
    }
}
