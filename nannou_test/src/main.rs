use nannou::prelude::*;
use rand::Rng; // gen_range() に必要

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Ball {
    x: f32,
    y: f32,
    _goal_x: f32,
    _goal_y: f32,
    radius: f32,
}

struct Model {
    balls: Vec<Ball>,
}

fn model(app: &App) -> Model {
    let window_w = 800;
    let window_h = 600;
    app.set_loop_mode(LoopMode::rate_fps(12.0)); // 効いてない？
    app.new_window()
        .size(window_w, window_h)
        .build()
        .unwrap();

    let mut balls: Vec<Ball> = Vec::new();
    let size = 30;
    for x in 0..size {
        for y in 0..size {
            let ball = Ball {
                x: rand::thread_rng().gen_range(-(window_w as f32)/2.0, window_w as f32/2.0),
                y: rand::thread_rng().gen_range(-(window_h as f32)/2.0, window_h as f32/2.0),
                radius: 10.0,
                _goal_x: (x * 20) as f32 - (window_w as f32)/2.0,
                _goal_y: (y * 20) as f32 - (window_h as f32)/2.0,
            };
            balls.push(ball);
        }
    }

    Model { balls: balls }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background()
        .color(SKYBLUE);

    for i in &model.balls {
        draw.rect()
            .x_y(i.x, i.y)
            .w_h(i.radius, i.radius)
            .color(BLUE);
    }

    draw.text(&app.fps().to_string())
        .font_size(30);

    draw.to_frame(app, &frame).unwrap();
}
