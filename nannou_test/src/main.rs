use nannou::{color::named, prelude::*};

fn main() {
    nannou::app(model) // model からアプリ作って
        .update(update) // update の中身で更新して
        .simple_window(view) // 画面に view 送って
        .run(); // 実行　な感じ？
}

struct Model {
    bg_color: String,
    x: f32,
    y: f32,
    radius: f32,
    change_mode: ChangeMode,
}

enum ChangeMode {
    BIGGER,
    SMALLER,
}

fn model(_app: &App) -> Model {
    Model {
        bg_color: "plum".to_string(),
        x: 0.0, // 数値設定は必ず f32
        y: 0.0,
        radius: 10.0,
        change_mode: ChangeMode::BIGGER,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    match model.change_mode {
        ChangeMode::BIGGER => {
            model.radius += 1.0;
            if model.radius > 500.0 {
                model.change_mode = ChangeMode::SMALLER;
            }
        },
        ChangeMode::SMALLER => {
            model.radius -= 1.0;
            if model.radius < 0.0 {
                model.change_mode = ChangeMode::BIGGER;
            }
        }
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background()
        .color(named::from_str(&model.bg_color).unwrap());
    draw.ellipse()
        .color(STEELBLUE)
        .w(model.radius)
        .h(model.radius)
        .x_y(model.x, model.y);
    draw.to_frame(app, &frame).unwrap();
}
