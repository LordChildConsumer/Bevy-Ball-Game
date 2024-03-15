use bevy::{
    window::Window,
    math::Vec3,
};


/*
    ----------------------------------
    ---- Clamp Position To Window ----
    ----------------------------------
*/

pub fn clamp_to_window(x: f32, y: f32, radius: f32, window: &Window) -> Vec3 {
    let x_min = 0.0 + radius;
    let y_min = 0.0 + radius;
    let x_max = window.width() - radius;
    let y_max = window.height() - radius;

    return Vec3::new(
        x.clamp(x_min, x_max),
        y.clamp(y_min, y_max),
        0.0
    );
}


pub fn clamp_to_window_margin(x: f32, y: f32, radius: f32, margin: f32, window: &Window) -> Vec3 {
    return clamp_to_window(x, y, radius + margin, window);
}