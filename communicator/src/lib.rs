pub mod client; // mod で宣言し、; で閉じた場合は同階層同名 rs ファイルを探す。
// pub で公開 default は非公開
pub mod network; // network::*

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}
        fn secret_function() {}
    }

    // inside mod , middle_secret_function はこの中からのみ呼べる
}

fn try_me() { // 現在モジュールからは outermost は呼べる
    outermost::middle_function(); // これのみ呼べる
    // このルートモジュールを読み込む main.rs からは outermost は読み込めない
}






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // client::connect(); 相対パスで tests から下らないとアクセスできないのでエラー
        // ::client::connect(); // このモジュールのトップまで行ってフルパスで記載 :: を先頭に　[動かないなぜ？]
        super::client::connect(); // super:: で１階層上に上がる
        /*
            use super::client で
            client::connect();
            にもできる
        */
    }
}
