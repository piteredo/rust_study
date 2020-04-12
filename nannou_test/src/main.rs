// nannou の最小例
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // キャンバス取得して draw に設定
    let draw = app.draw();

    // 背景色設定
    draw.background().color(PLUM);

    // 円描画
    draw.ellipse()
        .color(STEELBLUE)
        .w(300.0) // 数値必ず f32 で記述
        .h(200.0)
        .x_y(100.0, 300.0); // origin は中央にあるよう。Y 軸は下０～上height？

    // 処理したキャンバスをフレームに送る
    draw.to_frame(app, &frame).unwrap();
}
