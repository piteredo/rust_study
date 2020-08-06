
// ジェネリック
// 慣例で他言語同様 T (=type) を使うけど、何の文字でも動く

// ＜関数＞
//
// T で任意の型で実行できる
// T: ~ トレイト（インターフェイス的な）ものの指定で、T の対象を限定できる
// PartialOrd => 比較が必要な関数なので、比較できる型であるよう限定
// Copy => listがコピーできるものとは限らないので、コピーを実装してる（スタックに格納できる）型に限定
// 限定するトレイトを指し、トレイト境界　という（トレイトは別ファイルで）
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ＜構造体＞
// enum でも同じように動く（Option、Result がまさにそう
//
// T で任意の型の（ただし x, y は同じ型であること）構造体を定義できる
struct Point1<T> {
    x: T,
    y: T,
}
// こうすることで、x, y, に違う型を充てれる (同じ型でもOK、意味はないけど
// ジェネリックの型引数の個数が違っても ↑ と同じ構造体名にはできない
struct Point2<T, U> {
    x: T,
    y: U,
}
// メソッド定義は、impl 後と、構造体名 後 両方に<T>が必要
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // メソッドでのみ使う新しいジェネリックがあっても OK
    fn mixup<U>(self, other: Point1<U>) -> Point2<T, U> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
// 逆にジェネリック構造体のメソッド定義として型を指定することもできる
// T が f32 のときだけ呼び出すことができる
impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    let num_list = vec![34, 2, 43, 133, 35];
    let result = largest(&num_list);
    println!("largest-num: {}", result);

    let char_list = vec!['a', 'f', 'b', 'z', 'd'];
    let result = largest(&char_list);
    println!("largest-char: {}", result);

    let integer = Point1 {x: 5, y: 4};
    let float = Point1 {x: 5.3, y: 4.4};
    let number = Point2 {x:2.2, y: 1};
    println!("x: {}", integer.x());
    println!("dist_from_origin: {}", float.distance_from_origin());
    // println!("dist_from_origin: {}", integer.distance_from_origin()); 構文エラー(method not found)

    let p3 = integer.mixup(float);
    println!("p3 x:{} y:{}", p3.x, p3.y); // x:5 y:4.4
}
