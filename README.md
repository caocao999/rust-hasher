# ファイルハッシュ計算ツール（Rust製）

Rustで実装された、指定したファイルのハッシュ値を計算するCLIツールです。  
SHA-256 / SHA-512 / MD5 を選択可能で、標準出力またはファイルへの出力にも対応しています。

## 🔧 コンパイル手順

1. Rustがインストールされていることを確認してください：
   ```
   rustc --version
   ```
## プロジェクトディレクトリに移動し、ビルドします：
1.ビルドコマンド
   ```
   cargo build --release
   ```

## 実行ファイルは target/release/hash.exe に生成されます（Windowsの場合）

##　利用方法
cargo run -- --file <ファイルパス> [--hash <アルゴリズム>] [--output <出力先ファイル>]

## 使用例
cargo run -- --file ./example.txt --hash sha512
cargo run -- --file ./example.txt --hash md5 --output ./result.txt


## File Hash Calculator (Rust)

## Build Instructions
#Make sure Rust is installed:
rustc --version
#Navigate to the project directory and build:
cargo build --release
The binary will be generated at target/release/hash.exe (on Windows)


