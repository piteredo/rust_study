// トレイト
// 他で言うインターフェイス的なもの


//定義する
trait Summary {
    fn summarize(&self) -> String; // ここで実装しない場合はセミコロンで閉じる(文)

    // デフォルトを実装できる。メソッド定義のときに実装しなくてもいいし、オーバーライドもできる
    // メソッド定義で実装がないときは、 impl Summary for Tweet {}　と空ブロックだけ書く
    fn summarize_temp(&self) -> String {
        String::from("(read more..)")
    }
}


//トレイトを型に実装する
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle { // トレイト名 for 構造体名 でメソッド定義する
    fn summarize(&self) -> String { // トレイト定義で実装してないものはここで実装
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // 続きに関係ないメソッドは書けない、impl を別｛｝で定義すること
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


//　トレイト境界
//　ジェネリックな型引数の制限として使う。制限するトレイトを指して「トレイト境界」
//　＋　で複数のトレイト境界を使える　<T: Summary + Display>
//　下例は、T を Summary を実装してる型に限定している
fn notify<T: Summary>(item: &T) {
    println!("news_notify: {}", item.summarize());
}
// トレイト境界を where を使って別記できる（多いときとか便利）
fn notify2<T, U>(_t: T, _u: U) -> i32
    where T: Summary, U: Summary
    {
        32
    }


//　ジェネリックな型引数をもつ impl にトレイト境界を使って、部分的に型を限定させられる
struct Pair<T> {
    x: T,
    y: T
}
impl<T> Pair<T> {
    //DO SOMETHING
}
// impl後の<> にトレイト境界書くこと
// こうすることで、cmp は比較可能なジェネリック型にのみ呼び出されると限定させられる
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp(&self) {
        if self.x > self.y {
            println!("as{}", self.x);
        } else {
            println!("as{}", self.y);
        }
    }
}
// トレイト境界を充てた impl、に、トレイトを実装することもできる
// T は Summary を実装した型であること、また、impl は Summary を実装する。となる
// ブランケット実装　という
impl<T: Summary> Summary for Pair<T> { // 本当は別のトレイトだったりする
    fn summarize(&self) -> String { //本当は T を使う
        String::from("sdddd")
    }
}


fn main() {
    let news = NewsArticle {
        headline: String::from("ssssss"),
        location: String::from("tokyo"),
        author: String::from("pi"),
        content: String::from("snsnsn"),
    };
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // どちらの構造体でも必要なメソッドだけど実装内容は異なるときに使う（いつもの感じ
    println!("news: {}", news.summarize());
    println!("tweet: {}", tweet.summarize());

    println!("tweet: {}", tweet.summarize_temp());

    // notify(333); Summary を実装してないのでエラー
    notify(&news); // 呼べる

    println!("dedede {}", notify2(news, tweet));


    let pair = Pair {
        x: 22,
        y: 23,
    };
    pair.cmp();
}
