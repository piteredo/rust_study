// 呼ばれたモジュール２
// この中にさらに子 mod がある場合は、親モジュール含め、mod名フォルダを呼び出し元階層に置き、
// mod.rs にmod名モジュールを、子を並べて子のmod名ファイルで置く

pub fn connect() { // network::connect

}
pub mod server; // network::server
