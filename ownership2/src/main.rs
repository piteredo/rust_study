fn main() {

    // スタック領域
    // コンパイル時にすべてのサイズが判明していてスタックにバイナリとして並べられる
    // 使用終わると順に消される（これはメモリ解放とは呼ばない？
    let s1 = "hello";
    let s2 = s1; // 完全コピーされる（双方に所有権 - 所有権はヒープだけの話？

    // ヒープ領域
    // 実行時にメモリ確保され、１変数に必ず１所有権(分散しない)で、所有権つき変数のスコープが抜けると解放
    // 所有権を失った変数は使えない
    let s3 = String::from("hello");
    let s4 = s3; // 所有権が s3 に移る(ムーブ)。
    // println!("{}", s3); s3 使用はコンパイルエラー (value borrowed after move)

    // 引数として送ると所有権も移る。
    let mut s5 = hoge(s4);
    // println!("{}", s4); s4 は使用できなくなる (value borrowed after move)
    println!("{}", s5);

    let mut s6 = s5.clone(); // clone() すればディープコピー

    // 参照渡しすれば所有権は移らない
    hoge2(&s5); //　送り先では不変（ s5 の定義が mut であっても
    hoge3(&mut s6); // mut 付ければ送り先でも可変、ただし１スコープ１可変まで。また不変で使用中の場合は可変不可。s6 自体の定義も mut つけること
    println!("{}", s6); // hello world

    // スライス(説明下記)
    let s7 = ex_slice(&s6); // あくまでも String s6 を参照したものなので、所有権はない。s6が消えれば使用不可
    println!("{}", s7); // hello

    ex_slice2(&s6[..]); // &String を &str として渡す

    // 配列のスライス
    let arr = [1, 2, 3, 4, 5];
    let sl = &arr[1..3]; // 配列先頭のポインタ[n1..n2] => &[i32] 型 という
    for elm in sl {
        println!("{}", elm); // 2, 3
    }


    // 所有権保持のヒープ領域変数 s5, s6 のみ drop() 呼んで「変数のメモリを解放」する。他変数は何も起こらない。
}

fn hoge(s4: String) -> String {
    s4 // 返せば所有権も返る
}

fn hoge2(s5: &String) {
    // s5.push_str(" world"); defalut では不変なので、改変はコンパイルエラー
}

fn hoge3(s6: &mut String) {
    s6.push_str(" world");
}

fn ex_slice(s5: &String) -> &str {
    // &str は スライス型 である（所有権なし・String の参照である
    // String との違いは、領域 を確保しているかどうか
    // String 内のポインタ指定範囲で不変の文字列を切り抜く == 文字列リテラルと同じ
    let sliced_str = &s5[0..5]; // String たる s5 のポインタ [n1番目 から 配列先頭からn2個分]
    let sliced_str2 = &s5[..]; // 無記入 == 0 .. 無記入 == len と同義

    sliced_str
}

fn ex_slice2(s6: &str) {
    // &String[..] を &str として受けることが出来る。&String[..] と &str どちらにも対応できてベター
    // &String のまま渡されてきてもエラーはでない
    println!("{}", s6);
}
