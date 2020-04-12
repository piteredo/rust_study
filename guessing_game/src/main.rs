extern crate rand;
// 外部依存のクレート(枠)の読み込み。use rand が使用可能に
// toml ファイルの dependencies で読み込み必要
// extern C と同じ？ == 宣言のみでグローバルにアクセス

use std::io;
use std::cmp::Ordering; // 比較メソッド含む（cmp == compare？
use rand::Rng; // 乱数生成メソッド含む

fn main() {
    println!("Guess a number!"); // 最後に ! がつく

    let secret_number = rand::thread_rng().gen_range(1, 100+1);
    // 整数型(デフォで i32 型 = 整数32ビット) と推論される
    // thread_rng() 乱数生成器
    // gen_range() 指定範囲の乱数生成 (n1以上, n2未満)
    // 整数固定？？

    //println!("(secret number is {})", secret_number);


    loop { // これでループできる

        let mut guess = String::new();
        // String 型と推論される
        // let で、型名は書かない（推論
        // mut は mutable == 変更可能
        // デフォでは immutable == 不変（今後変更する変数であることを明示できる！

        println!("Input num between 1&100");

        io::stdin().read_line(&mut guess)
            .expect("Fail to read line.");
        // &mut ポインタ＋変更できる を示して送る（送り先で変更されることが明示できる！
        // io:: staticの呼び出し同じ（== std::io::
        // stdin() 入力受付ハンドラ
        // read_line() 入力文字列の格納
        // .expect() Result型(ok, errのenum)で、エラー可能性をチェック。無いと警告だけど一応通る（チェック例外的な？
        // 以上をメソッドチェーンで行える

        // ↑ だと guess は String 型と推論され、整数と比較できずコンパイルエラーでるため、
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Result が Ok なら num 返す
            Err(_) => continue, // _ == 包括値　continue == loop先頭に
        };
        // guess の上書きができる！（シャドーイング
        // trim() 両端の空白除去(\nの削除)   parse() String=>Number
        // u32 == unsigned32bit == 32ビット非負整数 に型指定
        // { } 内で戻り値 == Result型enum を検証(match)する

        println!("your guessed: {}", guess);
        // {} フォーマット。ここに guess を出力。複数なら ("{}{}", a, b)

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // loop 抜ける
            }
        }
        // A.cmp(&B) で比較し、match に結果を返し、{ } 内に合う結果があるか検証する流れ
        // B は 参照渡し
        // Ordering 型 = Less, Greater, Equal を持つ enum

    } // (loop end)

    // void? は return 不要
}
