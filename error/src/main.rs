fn main() {
    //　自分でエラー作る場合（そりゃないよってケースに予め仕掛けておく
    // panic() はクライアント向けというより開発者向け。余計なテキストがいっぱいでる。

    /*
    thread 'main' panicked at 'explicit panic', src\main.rs:2:5
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\error.exe` (exit code: 101)
    */
    // panic!();

    /*
    thread 'main' panicked at 'error msg', src\main.rs:9:5    　←メッセージでる
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\error.exe` (exit code: 101)
    */
    //  panic!("error msg");

    //----------------------

    // 実行時エラーを呼んでみる（out of bounds
    /*
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99',/rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447\src\libcore\slice\mod.rs:2791:10
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\error.exe` (exit code: 101)
    */
    //let v = vec![1, 2, 3];
    //v[99];



    //----------------------
    // 失敗刷る可能性のある関数を Result に包んで呼ぶ
    // そしてチェックしてから使う
    let result = with_result();
    match result {
        Ok(r) => println!("Ok: {}", r),
        Err(error) => println!("Err: {}" , error),
    }

    // でも match だと長くなる。ので、.unwrap(), .expect() を使う
    // どちらも開封がエラーだったときにパニくってくれる
    // unwrap() は引数なし。panic のみ
    // expect("---") は引数必須。panic 時にそのテキスト書いてくれる
    let _re2 = with_result().unwrap();
    let _re3 = with_result_b().expect("err msg");
}

fn with_result() -> Result<u32, std::io::Error> {
    // 開封結果を親に投げることができる throw
    match with_result2() {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}

fn with_result_b() -> Result<u32, std::io::Error> {
    // エラー委譲を演算子で書ける（親に投げる
    let r = with_result2()?; // Errならこの場で早期リターン。OKなら代入
    Ok(r) //ただ Ok も返さないとならない？
}

fn with_result2() -> Result<u32, std::io::Error> {
    Ok(32)
}


/*
    致命的エラー、開発者むけは panic
    クライアントの選択肢でおきるかもしれない範囲のエラーは result で。なるべくこっち
*/
