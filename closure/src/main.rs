// クロージャ
// 匿名関数



fn foo(v1: u32, v2: u32) {
    /*
    何か重い処理を含む可能性のある関数内として、
    毎回該当する重い処理を呼び出してたんじゃ時間かかって仕方がない

    じゃあ冒頭で１回実行して変数に保存しておこうか？
    でもそうすると重い処理が必要ない分岐のケースでも重い処理をしなきゃならず非効率

    そんなときクロージャ使う。匿名関数。無名関数？ラムダっぽい短い処理の塊
    処理結果ではなく、関数そのものを変数に収める
    変数に収めたときは実行されておらず、使わない場合に無駄に実行されなくていい

    |x| ← 縦棒で囲った中が変数。小範囲スコープでの使用なので、引数の型注釈は不要。戻り値の型注釈も不要。
    |x|{~~~} ← 引数の直後に括弧で処理を書く。でも処理が１行の場合は｛｝省略できる
    |x| x+1　← 最短ならこの記述でクロージャに成り得る。
    |x, y|　← 複数引数はカンマで
    |x: i32| -> i32 {}　← 敢えてどうしても注釈するならこういうことになっている
    */
    let heavy_calculation_closure_ex = |num| {
        println!("FOO calc. slowly....");
        std::thread::sleep(std::time::Duration::from_secs(2)); // ２秒停止するスレッドスリープ(javaにもあったやつ)
        num
    }; // セミコロンで閉じる(文)

    if v1 < 25 {
        println!("result_1_a: {}", heavy_calculation_closure_ex(v1));
        println!("result_1_b: {}", heavy_calculation_closure_ex(v1));
        // クロージャの型推論は最初に実行されたときの引数で行う。
        // 続けて違う型の引数で実行しようとすると（下記）これはエラーをくらう(mismatch type)
        // println!("result_1_b: {}", heavy_calculation_closure_ex(String::from("ccc")));
    } else {
        if v2 == 7 {
            println!("result_2_a: (no_heavy_calculation)");
        } else {
            println!("result_2_b: {}", heavy_calculation_closure_ex(v1));
        }
    }
}



/*
でも、↑ の書き方だと 外側ifブロック内の処理で、該当の重い処理が２回呼ばれて非効率
そこで、クロージャと、クロージャ呼び出し結果を保存する構造体をつくる
インスタンスから結果を求めたときに None ならクロージャ実行、既にある = Some(v) なら実行せず値を返す。やつ

構造体にクロージャを保存するためには、クロージャの型　が要る。
ジェネリクスとトレイト境界を使って実現する。
すべてのクロージャは Fn, FnMut, FnOnce どれかのトレイトを実装している

Fn トレイト境界は(他の２つも)実行するクロージャの引数と戻り値の型を加えて使う
Fn(u32) -> u32　というトレイト境界になる
*/
struct Cacher<T>
    where T: Fn(u32) -> u32 // 何でもいいけどこれにハマるやつでね、というジェネリクス。関数も Fn, FnMut, FnOnce 実装してる。
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    // インスタンス生成。value の初期値は None
    fn new(calculation: T) -> Cacher<T> { // 戻り値の Cacher にも <T> 書かないと引数過不足エラーになる
        Cacher {
            calculation,
            value: None,
        }
    }

    // value の値を変えるため、インスタンスの可変参照をもらう
    // すでに Some(v) なら v 返送。なければこの場でクロージャ実行して v 作る
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg); // (self.calculation) => calucrationクロージャ(arg=引数)
                self.value = Some(v);
                v
            },
        }
    }
}
fn hoge(v1: u32, v2: u32) {
    // クロージャを保存した構造体インスタンスを作る。
    let mut heavy_calculation_closure_ex = Cacher::new(|num| { // value 関数で value値いじるので mut
        println!("HOGE calc. slowly....");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    });

    if v1 < 25 {
        // value あるならくれよ、という形で実行することになる
        println!("result_1_a: {}", heavy_calculation_closure_ex.value(v1));
        println!("result_1_b: {}", heavy_calculation_closure_ex.value(v1)); // この時、値はあるので重処理は実行されずに済む
    } else {
        if v2 == 7 {
            println!("result_2_a: (no_heavy_calculation)");
        } else {
            println!("result_2_b: {}", heavy_calculation_closure_ex.value(v1));
        }
    }
}



/*
↑ でもまだ問題がある。
重処理を続けて実行するその２回目の引数が違ったとしても、
既に値あるから～と１回目処理時の引数で処理されたものが返ってきてしまう。大問題。

そこで value の部分を　HashMap にして、送られてきた引数をキーとして、その値があるか調べて、あればクロージャ使わず返す。
なければ引数を新しいキーとして、その実行結果を値にして登録。値を返す。
と、未登録の引数に対してだけクロージャ実行、というCacherができあがる
*/
struct Cacher2<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: std::collections::HashMap<u32, u32>,
}
impl<T> Cacher2<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher2<T> {
        Cacher2 {
            calculation,
            value: std::collections::HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {

        match self.value.get(&arg) {
            // .get は Option型　Some(&v), None で返ってくる
            Some(v) => *v, // すでにキー(引数)＋値セットが存在したらその値を返す（参照はずす
            None => {
                // なければ、生成・登録してから値を返す
                let k = arg;
                let v = (self.calculation)(arg);
                self.value.insert(k, v);
                v
            }
        }
    }
}
fn boo(v1: u32, v2: u32) {

    let mut heavy_calculation_closure_ex = Cacher2::new(|num| {
        println!("BOO calc. slowly....");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    });

    if v1 < 25 {
        println!("result_1_a: {}", heavy_calculation_closure_ex.value(v1));
        println!("result_1_b: {}", heavy_calculation_closure_ex.value(v2)); // ← v2
    } else {
        if v2 == 7 {
            println!("result_2_a: (no_heavy_calculation)");
        } else {
            println!("result_2_b: {}", heavy_calculation_closure_ex.value(v1));
        }
    }
}



/*
さらに、すべてをジェネリクスにしたもの。
ただし動作を保証するのに必要なトレイト境界が多くてあまり便利な感じがしない…使い方が悪いのか。こういうもんか
*/
struct Cacher3<T, K, V>
    where T: Fn(K) -> V,
    K: std::fmt::Debug + Eq + std::hash::Hash + Copy, // HashMap K に保証が必要なトレイト境界（コピーはクロージャ引数に使うため
    V: std::fmt::Debug + Copy  // HashMap V に保証が必要なトレイト境界（コピーは参照外して返すため
{
    calculation: T,
    value: std::collections::HashMap<K, V>,
}
impl<T, K, V> Cacher3<T, K, V>
    where T: Fn(K) -> V,
    K: std::fmt::Debug + Eq + std::hash::Hash + Copy,
    V: std::fmt::Debug + Copy
{
    fn new(calculation: T) -> Cacher3<T, K, V> {
        Cacher3 {
            calculation,
            value: std::collections::HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let k = arg;
                let v = (self.calculation)(arg);
                self.value.insert(k, v);
                v
            }
        }
    }
}
fn bobobo<T>(v1: T, v2: T)
    // Cacher の K になるための全てのトレイト境界 + 結果表示するための Display
    where T: std::fmt::Debug + Eq + std::hash::Hash + Copy + std::fmt::Display
{

    let mut heavy_calculation_closure_ex = Cacher3::new(|arg| {
        println!("BOBOBO calc. slowly....");
        std::thread::sleep(std::time::Duration::from_secs(2));
        'c'
    });

    println!("result_1_a: {}", heavy_calculation_closure_ex.value(v1));
    println!("result_1_b: {}", heavy_calculation_closure_ex.value(v2));
}



fn main() {
    let value1 = 10;
    let value2 = 7;
    foo(value1, value2);
    hoge(value1, value2);
    boo(value1, value2); // ２つめの値が違うので、両方実行
    boo(value1, value1); // ２つめの値が同じなので、２つめはキー登録済。実行しない

    bobobo('s', 'd'); // ジェネリクスだといろいろ送れる（実際トレイト境界のせいでそんなに送れない？
    bobobo(2, 3);
}
