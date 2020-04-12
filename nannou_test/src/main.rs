// nannou の最小例
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // キャンバス取得して draw に設定
    let draw = app.draw();

    // 背景色設定
    draw.background().color(BLUE);

    // 処理したキャンバスをフレームに送る
    draw.to_frame(app, &frame).unwrap();
}
