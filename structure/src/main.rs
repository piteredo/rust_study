struct User {
    username: String, // 要素 => フィールド
    email: String,
    sign_in_count: u64,
    active: bool, // 最後もカンマ打ったほうがいい(追加のとき忘れないよう
}

fn main() {
     // 定義順不同でOK。 参照は特殊事例なので取り敢えず &str は無しで
     // 原則、構造体のインスタンスには全データを所有させる必要がある（回避は保留
     // 全部定義しないとコンパイルエラー
    let mut user1 = User {
        email: String::from("ppp@ss.bb"),
        username: String::from("Taro"),
        sign_in_count: 1,
        active: true,
    };

    // mut で定義すれば変更も可。ただし、全フィールド可変 or 全フィールド不変 しかできない
    user1.email = String::from("fef@gg.hh");
    println!("{}", user1.email); // 使い方同じ

    // 初期化省略記法
    let email = String::from("erf@gg@ff");
    let username = String::from("HANAKO");
    let user2 = build_user(email, username);

    // 構造体更新記法 (構造体コピーするときに、変更ない箇所は引き継げる
    let user3 = User {
        email: String::from("rty@fd.sd"),
        username: String::from("john"),
        ..user1 // これで、上で設定した項目以外のフィールド情報は user1 を継ぐ。最後にコンマ打つとコンパイルエラー
    };


    //--------------------

    // タプル構造体
    // 単純な内容のためフィールド名は要らない、でも型の定義が必要なときに使う
    // 普通のタプルひとくくりの型はない
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0); // フィールドの型構成が同じだろうが、決して Point と混同して使えない
    let origin = (0, 0, 0);


    //--------------------

    // ユニット様(よう)構造体
    // フィールドの無い型だけの構造体。使い方保留 (トレイト実装に使うと...
    struct Unit;
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // 仮引数と同じ要素名に入れる場合は email: email を省略できる！(初期化省略記法
        username,
        active: true,
        sign_in_count: 1,
    }
}
