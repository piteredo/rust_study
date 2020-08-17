use nannou::prelude::*;
use nannou::text::*;
use rand::Rng; // gen_range() に必要

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model(u8, u8, u8);

fn model(app: &App) -> Model {
    let window_w = 800;
    let window_h = 600;
    app.new_window()
        .size(window_w, window_h)
        .build()
        .unwrap();

    Model (50, 50, 50)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    app.set_loop_mode(LoopMode::rate_fps(1.0));
    model.0 += 1;
    model.1 += 1;
    model.2 += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    draw.background()
        .color(Rgb::new(model.0, model.1, model.2));

    draw.to_frame(app, &frame).unwrap();
}
