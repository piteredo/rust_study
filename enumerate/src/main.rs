
#[derive(Debug)]
enum Coin { // 列挙型
    Penny, // 列挙子
    Nickel,
    Dime {}, // 列挙子が直接データを保持できる(構造体型・タプル型)
    Quarter(String), // 保持するデータの型はばらばらでもOK
}

impl Coin { // 構造体と同様、メソッドも定義できる
    fn call(&self) {
        println!("{:?}", self); // Quarter("hello") => そのままデバッグ（データのみ取得は？
        if let Coin::Quarter(s) = self { println!("{}", s) }; // データのみ取得これ？？
    }
}

fn main() {
    // ！ 一見似たこれらが struct と使い勝手を分けるところは、全て Coin 型として使える、ことである
    // 同型で使えるということは同じ処理にまわせる
    // また並列事項として、それらを比較することができる match 構文（switch ぽいやつ

    let coin_1 = Coin::Quarter(String::from("hello")); // Coin 列挙型の列挙子(名前空間)を Penny にする
    coin_1.call(); // メソッド呼べる


    // NULL を避ける Option 型
    // enum Option<T> {
    //    Some(T),
    //    None,
    // }
    let some_num = Option::Some(3); // 型推論で列挙子をOption型に包む
    let absent_num: Option<i32> = None; // None の場合は型指定必須、またOption型は汎用過ぎて型(Option::)省略可
    // Option<i32> + i32 は別型になるので演算できない
    plus_one(some_num);
    plus_one(absent_num);


    println!("{}", value_in_cents(coin_1));
}

fn value_in_cents(coin: Coin) -> u32 {
    // match 制御フロー演算子
    // Enum の要素を判定してそれぞれの反応を設定する
    // 全列挙子に対する判断を書かないとエラー。または _ => ()   _ はother, () は do nothing を添える
    match coin {
        Coin::Penny => 1, // Coin:: 型名も書き忘れないこと
        Coin::Nickel => 5,
        Coin::Dime{} => 10, // データを所持してれば｛｝() 仮引数書くこと
        Coin::Quarter(s) => {
            println!("{}", s); // 複数処理は {} 式にする
            25
        }
        // _ => () その他があればこれ
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // if let で簡潔に match を現わすことができる。特定条件OK or その他で区別
    // しかし match の包括性(全列挙子忘れてないか)を失うので、簡潔性をとるかケースバイケース
    if let Some(n) = x {
        Some(n + 1)
    } else {
        None
    }
    /*
    match x {
        Some(n) => Some(n + 1),
        _ => None,
    } に同義
    */
}
