#[derive(Debug)] // derive: 派生する
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッド記法（クラスメソッドぽい感じ
// impl == implementation => 実装
// 複数の impl ブロックに分ける事もできるけど現状メリット無し。ジェネリックとトレイトで使用？
impl Rectangle {
    fn area(&self) -> u32 { // 自身(Rectangle)の参照を渡す。型注釈は不要
        self.width * self.height
    }

    // 別の引数を取る。インスタンスから呼ばれた関数は自動で第１引数に &self が加わる
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // &self を引数にとらない関数（staticメソッド的な感じ => 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 40,
    };
    println!("area of rect is {}", area(&rect)); // 1200

    // println!("rect is {}", rect); これはエラー(フォーマットできません
    println!("rect is {:#?}", rect); // :#? , さらに冒頭で #[derive(Debug)] 加える事でデバッグ可能に



    // メソッド記法のほうの area()
    // Rectangle 型に関連したメソッドであることをより強く結びつける
    // C で言うところの rect->area() だけど自動参照・自動参照はずし があるのでこの記述はない
    println!("area of rect is {}", rect.area());

    let rect2 = Rectangle {
        width: 22,
        height: 34,
    };
    // rect の面積に rect2 は収まるか？（別の引数を渡す
    println!("can rect hold rect2? {}", rect.can_hold(&rect2));

    // 関連関数
    let rect3 = Rectangle::square(20);
    println!("rect3 area is {:?}", rect3); // # が無いと改行整形なし
}

fn area(rect: &Rectangle) -> u32 {
    // ここで、より意味づけした計算が可能になる（実用性
    // タプルでくくったら rect.0, rect.1 分かりづらい
    // width, height 別々で渡したら、２つの変数の関連性が強く伝わりづらいコードになる
    rect.width * rect.height
}
