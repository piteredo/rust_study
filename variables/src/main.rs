extern crate rand;

use rand::Rng;

fn main() {

    // 不変変数
    let n1 = 5;
    // n1 = 6;  immutable なので変更はエラー
    println!("{}", n1);


    // 可変変数
    let mut n2 = 3;
    println!("{}", n2); // 変更前に使用しないと警告でる！( n2=3 使われてないよと
    n2 = 4; // mut を付けると mutable になるので変更 OK
    println!("{}", n2);


    // 定数と不変変数は違う
    // 定数は広く使いまわす "定数" 。使い方違いは、
    const POINTS : u32 = 100_000; // 型名の注釈が必須
    // 実行時に評価される値は不可、定数値のみ。
    // const POINTS2 : u32 = rand::thread_rng().gen_range(0, 100); これはコンパイルエラー
    let points2 = rand::thread_rng().gen_range(0, 100); // これは OK そして不変
    println!("{} {}", POINTS, points2);


    // シャドーイング
    // 不変変数の上書きができる。
    // 1) しかしその後再び不変になるという点で可変変数とは違う
    // 2) シャドーイングは 型の変更 ができる。可変変数は型変えられない
    let spaces = "    ";
    println!("{}", spaces);
    let spaces = spaces.len(); // シャドーイング時に、string => i32 になってる
    println!("{}", spaces); // spaces_xxx の様に余計な temp 変数が増えることを避けれる
    //
    let mut spaces2 = "    ";
    println!("{}", spaces2);
    // spaces2 = spaces2.len(); コンパイルエラー => 可変変数の型変換は不可
}
