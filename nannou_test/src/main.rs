use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {}

fn model(app: &App) -> Model {
    let _window = app
        .new_window() // newして
        .size(1920, 1080)
        .view(view)
        .build() // buildする
        .expect("err");
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background()
        .color(OLIVE);
    draw.to_frame(app, &frame).expect("err");
}
