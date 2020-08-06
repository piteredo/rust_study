// イテレータ



// すべてのイテレータは、Itelator トレイトを実装する
// 実装時に求められる唯一のメソッド定義が、next() である
/*
    こんな感じのもの
    trait Itelator {
        type Item; // 任意型を指定して使う Item
        fn next(&mut self) -> Option<Self::Item>; // 各要素を Some に包んで、繰り返し終わったら None を返す
        //（以下デフォルト実装いろいろ）
    }
*/

fn main() {
    // 生成・使用（for loop
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // イテレータ生成。この時点で繰り返しは起きていない
    for val in v1_iter { // for loop に呼ばれたときに初めて繰り返し使用(next())され、意味をなす。
        println!("got: {}", val);
    }
    // for val in v1_iter {} エラー
    // 所有権を ↑ のfor loop に奪われてる


    // 生成・使用（自分で next する
    let mut v2_iter = v1.iter();
    // next() によりイテレータ内部の状態が変わるため mut
    // for loop に mut が必要なかったのは、for loop がイテレータの所有権を奪って裏で可変にしてるから
    assert_eq!(v2_iter.next(), Some(&1)); // next() して起動。起動前にインデックスは[0]より手前にある
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);


    // イテレータを <消費する> メソッド
    let v3_iter = v1.iter();
    let total: i32 = v3_iter.sum(); // 合計を返す。所有権奪うので mut 不要。戻り値の型注釈が要る
    println!("total {}", total);


    // <他のイテレータを生成する> メソッド
    let v4_iter = v1.iter();
    let v5_iter = v1.iter();
    v4_iter.map(|x| x + 1); // 引数のクロージャで濾過した新しいイテレータを返す。
                            // ただし、「消費しないとなんもしないよ」と警告でる。
    let _new_vec: Vec<_> = v5_iter.map(|x| x + 1).collect(); // collect => イテレータの各要素を vec に集約する
                            // これでやっと(ひとつの手として)役を成したことになる


    filters_by_size();
    original_itelator_ex();
}


// ------------------------------------------


#[derive(PartialEq, Debug)] // assert_eq 内で何か(vec?)使うためにいるぽい
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size) // 環境をキャプチャする(クロージャの外の変数をつかう)例
                                         // filter は、クロージャ戻り値のbool==trueの時の要素だけ集めて新しいiterを作る
        .collect() // 作ったiterをvecに集約してreturn
}

fn filters_by_size() {
    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker")},
        Shoe {size: 13, style: String::from("sandal")},
        Shoe {size: 10, style: String::from("boot")},
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {size: 10, style: String::from("sneaker")},
            Shoe {size: 10, style: String::from("boot")},
        ]
    );
}


// ------------------------------------------


struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}
impl Iterator for Counter {
    type Item = u32; // 要素の型を指定

    fn next(&mut self) -> Option<Self::Item> { // SelfインスタンスのItem として返す
        self.count += 1; // カウントする（0 からなら 最後に置いても

        if self.count < 6 {
            Some(self.count) // 消費中は カウントを返す
        } else {
            None // 消費条件を追えたら None を返しておしまい
        }
    }
}

fn original_itelator_ex() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // --------------------

    // 他にもデフォルトメソッド色々
    //
    // zip 新しいiterを作って、書くiterの要素をタプルでまとめて返す＋さらに一方は1個分スキップ
    // map ペアのタプルを掛けたあたらしいiter を返す
    // filter そのうち x%3==0 になるものだけを抽出した新しいiter
    // sum それらを全部足す
    //
    // 所有権奪ってやってるので、mut 不要
    let cnt2: u32 = Counter::new().zip(Counter::new().skip(1))
                                  .map(|(a, b)| a * b)
                                  .filter(|x| x % 3 == 0)
                                  .sum();
    assert_eq!(18, cnt2);
}
