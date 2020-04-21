// main.rs はバイナリクレート

// ほとんどの機能はライブラリクレートにあり、バイナリクレートをそれを使用するという考え方

// communicator ライブラリクレートをスコープに導入
// communicator => 今回作ったモジュールたちの「ルートモジュール」と言う
extern crate communicator;

pub mod a { // 同ファイル内なら pub なくても呼べる(けど)
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// Enum も名前空間の一種なので、use でスコープに読み込める。
// Enum に限らず use 文は最後に{A, B, ..} とすることで１つの名前空間から複数の要素を導入できる
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
use TrafficLight::{Red, Yellow};
// use TrafficLight::* とすれば全列挙子呼べる( glob==塊 と呼ぶ。過剰読み込みは非推奨

use a::series::of;

fn main() {
    communicator::client::connect();

    // a::series::of::nested_modules(); フルパスで書くとこうだけど、上の use を使うと
    of::nested_modules(); // でアクセスできる
    // use a::series::of::nested_module; まで呼べば 関数直呼びできる。use に最後の ()　は付けない

    let red = Red; // まんま使える
}
