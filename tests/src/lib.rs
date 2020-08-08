// テスト
// cargo test でコンパイルすると、#[test] の関数をテストする

/*
running 1 test
test tests::it_works ... ok　　← テスト対象の関数
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out　　← 各結果
　passed 問題なし
　failed 失敗
　ignoerd #[ignore] を書いて、対象から省いた関数
　measured ベンチマーク用（？）ナイトリ版のみ
　filterd out テスト実行コマンドで条件から除外された関数
*/


// テストモジュール
// 通常の cargo run ではコンパイルされず、cargo test でだけコンパイルされる（cfg = config
//
// 単体テストのテストモジュールは、テストする関数に該当するファイルに直接書くため、このモジュールで区別するとコスト削減できる
// 結合テストはテスト専用のフォルダに書くので、これを書かずともテストだということになる
//
// 単体テストは非公開関数もテストできる。結合テストは公開されてるもののみ
#[cfg(test)]
mod tests {

     #[test]
    fn it_works() {
        // assert => １つの引数の戻り値が true かどうかだけ確かめる。trueなら何もしない、falseならpanic（以下同
        assert!(1==1);

        // assert_eq => 引数１==引数２であることをチェック。エラー時に右辺左辺を書いてくれるのでこっちのがいい
        assert_eq!(2 + 2, 4);
        // asset_ne => 引数１ != 引数２のとき通って、trueだとエラー。not equal ?

        /*
        asset 類に送る引数は比較トレイト PartialOrd, デバッグトレイト Debug を実装してないとならない
        組込み型すべて・標準ライブラリほぼ全ては実装済みだけど、自作型は自分で継承しないとならない
        どちらも「導出可能トレイト」なので　構造体定義に #[derive(PartialOrd, Debug)] と書けばいい
        derive == 継承と訳されている
        */

        /*
        assert! の２つめ以降の引数、assert_eq!, assert_ne! の３つめ以降の引数は format! と同様になっていて
        エラーメッセージ表示として使える
        */
        let result = String::from("Carolaaaaa");
        assert!(
            result.contains("Carol"), // 引数１はbool検証
            "Err: {}", // 引数２はformat! の引数１
            result // 引数３以降は format! の引数２以降
        );
    }

    #[test]
    #[should_panic]
    fn panic_test() {
        // panic なので、passed する
        // ただこれだと期待したのと違う理由でのpanicかどうか判断できかねる
        panic!("eeeerror");
    }

    #[test]
    #[should_panic(expected = "eee")]
    fn panic_test_with_expected_arg() {
        // こちらは、expected="" で panic時のエラーテキストにそれが含まれているか確かめる
        panic!("eeerror");
        /*
        ---- tests::panic_test_with_expected_arg stdout ----
        thread 'tests::panic_test_with_expected_arg' panicked at 'eeerror', src\lib.rs:59:9
        note: panic did not contain expected string
            panic message: `"eeerror"`,
        expected substring: `"eede"`
        */
    }


    /*
    テスト関数の println はキャプチャされてしまい、エラーでない限り表示されない。Okでも表示したい場合は
        cargo test -- --nocapture
    と書く。


    複数のテストはマルチスレッドで行われる。内容によって結果順が入れ替わることもある。シングルスレッドにしたい場合は
        cargo test -- --test-threads=1
    と書く。

    cargo run func_namexxx
    で特定の関数のみテストできる。このテキストが含まれる全てをテストする。カンマで複数指定したりはできない。
    ここで除外されたものが fillterd out の関数になる

    #[test]
    #[ignore]
    fn ffff(){

    }
    で 特定の関数を予めテストから無視することができる。ignored の関数になる。
    逆に、ignore の関数だけテストしたい場合は
        cargo test -- --ignored
    と書く。
    */
}

pub fn add_three(i: u32) -> u32 {
    i + 3
}
