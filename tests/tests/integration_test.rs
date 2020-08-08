// 結合テスト
// ROOT/tests で専用フォルダをつくっていれる
/*
Running target\debug\deps\integration_test-65e49fed50c4a114.exe
running 1 test
test testes ... ok　←クレート単位でチェック？
*/

extern crate tests;
// tests/lib.rs の読み込み（lib は親フォルダ名がクレート名となる

#[test]
fn testes() {
    let n = 2;
    let result = tests::add_three(n);
    assert_eq!(2 + 3, result);
}
