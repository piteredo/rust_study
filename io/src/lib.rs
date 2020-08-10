// query と filename のように２つが関連する要素を扱うときは、構造体。
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool, // 検索の大文字小文字どうするかの環境変数
}
impl Config {
    // イテレータをつかってコマンドライン引数を直接処理する。要 mut
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
            // 細かくエラーを書く。panic ではなくresult に包む
            // return 書かないとエラーになる。早期リターンだから？
        }

        args.next(); // １引数目は使わないのでまずnext

        // ２引数目をクエリに
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get"), // 引数なければエラーを早期リターン
        };

        // ３引数目をfilenameに
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get"),
        };

        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
        /*
        $env:CASE_INSENSITIVE=1
        cargo run to poem.txt
        で環境変数変えれる。ターミナル閉じるまで有効？
        */

        Ok( Config { query, filename, case_sensitive })
    }
}


// エラーがあった場合に備えてresult型で返す
use std::error::Error;
pub fn run(config: Config) -> Result<(), Box<Error>>{
    use std::io::prelude::*;
    use std::fs::File;
    let mut f = File::open(config.filename)?; // ? でエラーケースを親に投げる

    let mut contents = String::new();

    f.read_to_string(&mut contents)?; // ? でエラーケースを親に投げる
    //println!("<refactaring ver.>\n{}", contents);


    // 環境変数による対応変化を追加
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(()) // Result型返すので問題なければ最後ユニット型 () をOkに包んで返す
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    let mut result = Vec::new();
    for line in contents.lines() { // テキストの各行を検証
        if line.contains(query) { // 行のなかに検索したい文字列があるか
            result.push(line);
        }
    }
    result
    */

    // ↑ をイテレータをつかって書き換えられる。短い！
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // 大文字小文字気にしないので全部小文字にしちゃう
    let mut result = Vec::new();

    for line in contents.lines() { // テキストの各行を検証
        if line.to_lowercase().contains(&query) { // 行のなかに検索したい文字列があるか
            result.push(line);
        }
    }

    result
}


// テストを書く
// 成功したら、実装に組み込む。という流れ
#[cfg(test)]
mod test {
    use super::*; // 親階層ぜんぶつかう

    #[test]
    #[ignore]
    fn on_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    #[test]
    fn case_sensitive() { // 大文字小文字区別する
        let  query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() { // 大文字小文字区別しない
        let  query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
