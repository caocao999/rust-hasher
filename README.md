# ファイルハッシュ計算ツール（Rust製） / File Hash Calculator (Rust)

Rustで実装された、指定したファイルのハッシュ値を計算するCLIツールです。  
SHA-256 / SHA-512 / MD5 を選択可能で、標準出力またはファイルへの出力にも対応しています。  
A CLI tool written in Rust to compute hash values of a given file.  
Supports SHA-256, SHA-512, and MD5. Output can be printed to console or written to a file.

---

## 🔧 コンパイル手順 / Build Instructions

1. Rust がインストされていることを確認します：  
   Make sure Rust is installed:
   ```bash
   rustc --version
   ```

2. プロジェクトディレクトリに移動して、ビルドします：  
   Navigate to the project directory and build:
   ```bash
   cargo build --release
   ```

3. 実行ファイルは `target/release/hash.exe` に生成されます（Windows の場合）  
   The binary will be generated at `target/release/hash.exe` (on Windows)

---

## 🚀 利用方法 / Usage

```bash
cargo run -- --file <ファイルパス / file_path> [--hash <アルゴリズム / algorithm>] [--output <出力先ファイル / output_file>]
```

---

## ✅ 引数の説明 / Argument Descriptions

| オプション / Option    | 説明 / Description                                |
|------------------------|---------------------------------------------------|
| `--file` または `-f`   | 対象ファイルのパス（必須）<br>Path to the target file (required) |
| `--hash` または `-H`   | ハッシュ関数の指定（省略時は `sha256`）<br>Hash algorithm (default: `sha256`) |
| `--output` または `-o` | 出力ファイルパス（省略時は標準出力）<br>File to save output (default: stdout) |

---

## 🧪 使用例 / Examples

```bash
cargo run -- --file ./example.txt --hash sha512
cargo run -- --file ./example.txt --hash md5 --output ./result.txt
```
