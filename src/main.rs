/*!
 * main.rs - ファイルハッシュ計算ツール メインモジュール
 * 
 * 作成者: cacao999
 * 作成日: 2024年
 * 概要: 指定されたファイルのハッシュ値を計算し、標準出力またはファイルに出力
 *       SHA256/SHA512アルゴリズムに対応し、コマンドライン引数で制御
 * 
 * 使用方法:
 *   hash -f target_file -H sha256 -o output.txt
 *   hash -f target_file -H sha512
 * 
 * サポートアルゴリズム:
 *   - SHA256 (デフォルト)
 *   - SHA512
 * 
 * 依存関係:
 *   - args: コマンドライン引数解析モジュール
 *   - my_lib: ハッシュ計算・ファイル出力モジュール
 * 
 * ライセンス: MIT
 * バージョン: 0.1.0
 */

use std::fs;
use std::process;

mod args;
mod my_lib;

fn main() {
    // コマンドライン引数の解析
    // args::parse_args()でCliArgs構造体を取得
    let args: args::CliArgs = args::parse_args();

    // 対象ファイルの読み込み
    // バイナリファイルにも対応するためVec<u8>で読み込み
    let data = match fs::read(&args.file) {
        Ok(data) => data,
        Err(err) => {
            // ファイル読み込みエラー時の処理
            // エラー内容とファイル名を表示してプログラム終了
            eprintln!("{} {}", err, &args.file);
            process::exit(1);
        }
    };

    // ハッシュアルゴリズムの選択と実行
    // 文字列スライスを使用してパターンマッチング
    let hash = &args.hash[..];
    let result = match hash {
        "sha512" => my_lib::hash512(&data), // SHA512でハッシュ計算
        _ => my_lib::hash256(&data),         // デフォルトはSHA256
    };

    // 出力処理
    // output引数が指定されている場合はファイル出力
    let out_file = args.output.unwrap_or_default();
    if out_file != "" {
        // ファイルへの出力処理
        // unwrap()でエラー処理を簡略化（TODO: より詳細なエラーハンドリング）
        my_lib::output_file(&out_file, &result).unwrap();
    }

    // ハッシュ値を標準出力に表示
    // ファイル出力の有無に関わらず常に表示
    println!("\nハッシュ値　 : {}\n", result);
}