/*!
 * my_lib.rs - ハッシュ計算・ファイル出力ライブラリ
 * 
 * 作成者: cacao999
 * 作成日: 2024年
 * 概要: SHA256/SHA512によるハッシュ値計算とファイル出力機能を提供
 *       バイナリデータに対応し、16進数文字列としてハッシュ値を返却
 * 
 * 提供機能:
 *   - SHA256ハッシュ値計算
 *   - SHA512ハッシュ値計算
 *   - ハッシュ値のファイル出力
 * 
 * 依存関係:
 *   - sha2: SHA-2ファミリーのハッシュアルゴリズム実装
 *   - std::fs, std::io: ファイル操作
 * 
 * セキュリティ用途:
 *   - ファイル整合性チェック
 *   - デジタル署名の前処理
 *   - データの同一性検証
 */

use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::Write;

/**
 * SHA256ハッシュ値を計算
 * 
 * @param data ハッシュ計算対象のバイナリデータ
 * @return 16進数文字列形式のSHA256ハッシュ値
 * 
 * 処理フロー:
 *   1. SHA256ハッシャーを初期化
 *   2. 入力データでハッシャーを更新
 *   3. ハッシュ値を確定し16進数文字列で返却
 * 
 * 注意: バイナリファイルにも対応するためVec<u8>を受け取る
 */
pub fn hash256(data: &Vec<u8>) -> String {
    // SHA256ハッシャーの初期化
    let mut hasher = Sha256::new();
    
    // 入力データでハッシャーを更新
    // 大容量ファイルでも効率的に処理
    hasher.update(&data);
    
    // ハッシュ値の確定と16進数文字列への変換
    let result = hasher.finalize();
    format!("{:x}", result)
}

/**
 * SHA512ハッシュ値を計算
 * 
 * @param data ハッシュ計算対象のバイナリデータ
 * @return 16進数文字列形式のSHA512ハッシュ値
 * 
 * 処理フロー:
 *   1. SHA512ハッシャーを初期化
 *   2. 入力データでハッシャーを更新
 *   3. ハッシュ値を確定し16進数文字列で返却
 * 
 * SHA512はSHA256より長いハッシュ値（512bit）を生成し、
 * より高いセキュリティレベルを提供
 */
pub fn hash512(data: &Vec<u8>) -> String {
    // SHA512ハッシャーの初期化
    let mut hasher = Sha512::new();
    
    // 入力データでハッシャーを更新
    hasher.update(&data);
    
    // ハッシュ値の確定と16進数文字列への変換
    let result = hasher.finalize();
    format!("{:x}", result)
}

/**
 * ハッシュ値をファイルに出力
 * 
 * @param file_path 出力先ファイルパス
 * @param data 出力するハッシュ値（文字列）
 * @return 処理結果（Ok(())または詳細エラーメッセージ）
 * 
 * エラーハンドリング:
 *   - ファイル作成失敗時: 日本語エラーメッセージ
 *   - 書き込み失敗時: 日本語エラーメッセージ
 * 
 * TODO: エラーメッセージの誤字修正が必要
 */
pub fn output_file(file_path: &str, data: &str) -> Result<(), String> {
    // 出力ファイルの作成
    // 既存ファイルは上書きされる
    let mut file = File::create(file_path)
        .map_err(|_e_| "ハッシュ値書き込みようファイルが開けません".to_string())?; // TODO: "用"に修正
    
    // ハッシュ値の書き込み
    // 文字列をバイト配列に変換してファイルに書き込み
    file.write(data.as_bytes())
        .map_err(|_e_| "ハッシュ値が書き込みめません".to_string())?; // TODO: "書き込めません"に修正
    
    Ok(())
}