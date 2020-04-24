use nannou::prelude::*;
//use rand::Rng;
use nannou::text;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .event(event)
        .view(view)
        .build()
        .expect("err");
    Model {
        x: 0.0,
        y: 0.0,
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    println!("{:?}", event);

    // キーイベントによって update されるものはここで update
    match event {
        KeyPressed(_key) => {
            model.x += 5.0;
            model.y += 5.0;
        }

        _other => {}
    }
}

// それ以外の条件によって update されるものはこっち
fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background()
        .color(DIMGRAY);

    draw.ellipse()
        .x_y(model.x, model.y)
        .radius(10.0)
        .color(RED);

    let font_data: &[u8] = include_bytes!("../arial.TTF");
    // let owned_font_data: Vec<u8> = font_data.to_vec();
    let font: nannou::text::Font = nannou::text::Font::from_bytes(font_data).unwrap();
    let text = "W";
    draw.text(text).font_size(60).font(font);

    draw.to_frame(app, &frame).expect("err");
}
