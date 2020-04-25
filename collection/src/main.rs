enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    ex_vector();
    ex_string();
    ex_hashmap();
}

fn ex_vector() {
    // ベクタ型 Vec<T> なんの型でもOK、違う型詰め込みNG
    let mut v1: Vec<i32>  = Vec::new(); // new なら型注釈（ジェネリック
    let mut v2 = vec![1, 2, 3]; // 実用はこちら。定義兼初期化

    v1.push(0); // push で追加。mut で宣言すること
    let _i1 = &v2[0];
    // v2.push(4); 要素単位ではなく Vec 丸ごとで、どっか借用されてたら可変NG

    // 取得方法２つ（型注釈は分かりやすくするため。実際は不要
    let _third_1: &i32 = &v2[2]; // 添え字 => 範囲外等エラーで panic
    let _third_2: Option<&i32> = v2.get(2); // Option型 (Some or None) => エラーだと None 返す（エラーあり得る時の選択肢

    for i in &mut v2 { // 可変参照をとり、
        *i += 50; // 参照はずしで可変する
        println!("{}", i); // for ループ普通に
    }

    let _row = vec![
        SpreadsheetCell::Float(3.0),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("ssss")),
    ]; // enum 使えば違う型の要素保持もできる

    // Vec の drop と同時に、含むすべての要素も破棄される
}

fn ex_string() {
    // &str はスタックに保存された str の借用（スライス）である。不変
    // String はヒープに保存された可変の文字列コレクション。 Vec<T> のラッパである。
    let _s1 = String::new(); // 空文字列の生成
    let mut s2 = "hello world".to_string(); // String::from("hello world") と同義

    // 追加
    s2.push_str("bababa"); // 文字 "列" の追加
    s2.push('c'); // 文字 "単一" の追加（まさに Vec のそれと同じ

    // 結合１
    let s3 = String::from("hello");
    let s4 = String::from("world");
    let s5 = s3 + &s4; // s3 は s5 にムーブされ使用不可に。 &s4 は &str に強制変換され足される
    println!("{}", s4); // s4 はそのまま使える

    // 結合２
    // ↑ がややこしいので、こっちが推奨
    let _s6 = format!("{} {}", s4, s5); // println! と同じ感じで、String を返すやつ
    println!("{} {}", s4, s5); // しかも結合前のどちらの String もムーブしない

    // String には添え字でアクセスできない！
    // ２バイト文字の１バイト分にだけアクセスなど、変な崩壊を防ぐため
    let s7 = String::from("Здравствуйте"); // ２バイト文字のテキスト
    let _s8 = &s7[0..4]; // ４バイト分＝２文字スライス。ここで３文字とか取った場合にはpanic!()

    for c in s7.chars() {
        println!("{}", c); // これで２バイト文字崩壊せず char ごとに取れる => スカラー値
    }
    for b in s7.bytes() {
        println!("{}", b); // バイト単位でとるならこれ。バイト数値が出力
    }
}

fn ex_hashmap() {
    use std::collections::HashMap; // まず use が必要
    let mut scores = HashMap::new(); // ここで型注釈しなくていい（でも要素挿入がないと赤でる
    scores.insert(String::from("Blue"), 10); // 次いで、組み込み生成のマクロもないので new した上で挿入
    scores.insert(String::from("Yellow"), 50); // 値はコピーされるため、ムーブされ、String文字列は使用不可に

    // ２つの Vec をまとめる形でもつくれる
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // アクセス１
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name); // キーとなる要素の参照を渡す
    // Option型で返ってくる(Some(&V), None)・V は参照が返ってくる（ハッシュマップが所有権を保持し続ける

    // アクセス２
    for (key, value) in &scores { // ハッシュマップの参照を取って、タプル k, v に流し込む
        println!("{} {}", key, value);
    }

    // 値の更新１　上書き
    let mut scores_2 = HashMap::new();
    scores_2.insert(String::from("Blue"), 10);
    scores_2.insert(String::from("Blue"), 25); // insert は上書きされる => 25

    // 値の更新２　キーに値が無い時のみ挿入
    scores_2.entry(String::from("Yellow")).or_insert(22); // Yellow に値無ければ 22 入れてね（入る）
    scores_2.entry(String::from("Blue")).or_insert(32); // 既に値あるので 32 は入らない（25のまま
    for (k, v) in &scores_2 {
        println!("{} {}", k, v);
    }

    // 更新２ => 古い値ににもとづいて更新することができる
    // 例えば、単語の登場回数を数える
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert は値を返す
        *count += 1; // 新規の、または既に存在する Key の Value を受け取り１を足す
    }
    println!("{:?}", map); // {wonderful:1, hello:1, world:2}
}
