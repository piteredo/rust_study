fn main() {
    let n1 = 3;
    let n2 = 5;

    // 評価式の（）は不要
    if n1 < n2 {
        println!("{} is smaller than {}", n1, n2);
    } else {
        println!("{} is not smaller than {}", n1, n2);
    }

    println!("{}", get_max(n1, n2));

    loop_ex();
}

fn get_max(n1: i32, n2: i32) -> i32 {
    // if 内で式を使い値を返せる。当然 let で変数にも入れられる
    let n3: i32 = if n1>=n2 {
        n1
    } else {
        n2

        // "string"
        // if と else で型が違うとコンパイルエラー
    };  //.trim().parse().expect("string NG!"); これ書いても駄目だった
    n3
}

fn loop_ex() {

    //条件なしループ => break で抜ける
    let mut i = 3;
    loop {
        if i<0 { break }; // {} 省略できない
        // i-- インクリメント・デクリメント使えない
        i = i-1;
    }

    //条件付きループ（ほぼ for 文つかってやる主流
    //while
    let mut j = 3;
    while j >= 0 { // != はあるけど、!< ない？？
        j = j-1;
    }

    //for in
    let arr = [11, 22, 33, 44, 55];
    for elem in arr.iter() {
        println!("{}", elem);
    }

    //for in (添え字あり / 従来の i=0; i<X; i++ は無さそう
    for num in 1..4 { // n1 以上 n2 未満
        println!("{}", num);
    }

    for num in (1..4).rev() { // rev() 逆順（つけるときは(1..4)かっこで
        println!("{}", num);
    }
}
