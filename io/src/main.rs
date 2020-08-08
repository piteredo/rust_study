
extern crate io;

// コマンドラインの引数を読み取る
//
use std::env; // ::args まで読み込まずに、env::args() と env 依存を示すのが乙
fn main() {
    let iter = env::args(); // コマンドライン引数のイテレータを読み込む関数
    let args: Vec<String> = iter.collect(); // iter を vec に集約する関数（ 型注釈必要
    //println!("{:?}", args); // デバッグする
    /*
        cargo run seachstr ex-filename.txt
        で、
        ["target\\debug\\io.exe", "seachstr", "ex-filename.txt"]
        を返す。
        １つめに固定でバイナリファイルの名前が返る。２つめ以降が自分で送った引数(String型)
    */

    // 変数に保存
    let args: Vec<String> = env::args().collect(); // env => enviroment (環境)
    let query = &args[1]; // ファイル内を検索する文字列
    let filename = &args[2]; // 検索対象のファイル　というてい
    //println!("serching {} in FILE: {}", query, filename);



    // 実際にファイルを開く
    use std::io::prelude::*; // read_ で使う
    use std::fs::File;
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("somthing went wrong reading the file");

    //println!("<With text>\n{}", contents);





    // リファクタリング
    //
    // main 関数の役割
    // コマンドライン引数の解析ロジック呼び出し
    // 他の設定いろいろ
    // あとは lib.rs の run() を呼んで、全部やってもらう
    // lib.rs がエラーを返したときに処理する
    let args2: Vec<String> = env::args().collect();

    // 構造体のコンストラクタにパース処理をしてもらう
    // Result 型で返ってくるのでエラーを考慮する。
    // unwrap_or_else => Ok なら config に代入、Errならクロージャ実行(err引数はErr戻り値)
    let config = io::Config::new(&args2).unwrap_or_else(|err| {
        eprintln!("Problem: {}", err);
        std::process::exit(1); // 0 以外はエラー終了
    });

    // 残りの処理は全部 lib.rs の run 関数に任せてしまう
    // run がエラーを返した場合を想定する
    if let Err(e) = io::run(config) {
        eprintln!("app err: {}", e); // eprintln! 標準エラーに出力
        std::process::exit(1);
    }
    // lib.rs に続く...
}

/*
cargo run > output.txt で出力をファイルにできる
*/
