/*!
 * args.rs - コマンドライン引数解析モジュール
 * 
 * 作成者: cacao999
 * 作成日: 2024年
 * 概要: ハッシュ計算ツールのコマンドライン引数を解析し、
 *       構造体として整理して返す機能を提供
 * 
 * 機能:
 *   - ファイルパスの指定
 *   - ハッシュアルゴリズムの選択（SHA256, SHA512, MD5等）
 *   - 出力先の指定（ファイルまたは標準出力）
 * 
 * 依存関係:
 *   - clap: コマンドライン引数解析ライブラリ
 * 
 * 使用例:
 *   hash -f sample.txt -H sha256 -o result.txt
 */

use clap::{Arg, Command};

/**
 * コマンドライン引数を格納する構造体
 * 
 * 各フィールドは解析されたコマンドライン引数の値を保持
 */
pub struct CliArgs {
    /// ハッシュ計算対象のファイルパス
    pub file: String,
    /// 使用するハッシュアルゴリズム（sha256, sha512, md5等）
    pub hash: String,
    /// 出力先ファイルパス（Noneの場合は標準出力）
    pub output: Option<String>,
}

/**
 * コマンドライン引数を解析してCliArgs構造体を返す
 * 
 * @return CliArgs 解析されたコマンドライン引数
 * 
 * 解析する引数:
 *   -f, --file: 対象ファイルパス（必須）
 *   -H, --hash: ハッシュアルゴリズム（デフォルト: sha256）
 *   -o, --output: 出力ファイルパス（オプション）
 */
pub fn parse_args() -> CliArgs {
    // コマンドライン引数の定義
    // clapクレートを使用してCLIインターフェースを構築
    let matches = Command::new("hash")
        .version("0.1.0")
        .author("Your Name") // TODO: 実際の作成者名に変更
        .about("Calculate hash of a file")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .required(true) // 必須パラメータ
                .value_name("FILE")
                .help("Path to the file"),
        )
        .arg(
            Arg::new("hash")
                .short('H') // 大文字Hを使用（-hはhelpと競合するため）
                .long("hash")
                .default_value("sha256") // デフォルトはSHA256
                .help("Hash algorithm (e.g., sha256, sha512, md5)")
                .value_name("HASH"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path (default: stdout)")
                .value_name("OUTPUT"),
        )
        .get_matches();

    // 解析結果をCliArgs構造体に格納して返却
    // unwrap()は必須パラメータなので安全
    CliArgs {
        file: matches.get_one::<String>("file").unwrap().to_string(),
        hash: matches.get_one::<String>("hash").unwrap().to_string(),
        output: matches.get_one::<String>("output").map(|s| s.to_string()),
    }
}