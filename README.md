# ファイルハッシュ計算ツール（Rust製）

Rustで実装された、指定したファイルのハッシュ値を計算するCLIツールです。  
SHA-256 / SHA-512 / MD5 を選択可能で、標準出力またはファイルへの出力にも対応しています。

---

## 🔧 コンパイル手順

1. Rust がインストールされていることを確認します：
   ```bash
   rustc --version
   ```

2. プロジェクトディレクトリに移動して、ビルドします：
   ```bash
   cargo build --release
   ```

3. 実行ファイルは `target/release/hash.exe` に生成されます（Windows の場合）

---

## 🚀 利用方法

```bash
cargo run -- --file <ファイルパス> [--hash <アルゴリズム>] [--output <出力先ファイル>]
```

---

## ✅ 引数の説明

| オプション          | 説明                                      |
|---------------------|-------------------------------------------|
| `--file` または `-f` | 対象ファイルのパス（必須）                 |
| `--hash` または `-H` | ハッシュ関数の指定（省略時は `sha256`）     |
| `--output` または `-o`| 出力先ファイル（省略時は標準出力）         |

---

## 🧪 使用例

```bash
cargo run -- --file ./example.txt --hash sha512
cargo run -- --file ./example.txt --hash md5 --output ./result.txt
```

---

# File Hash Calculator (Rust)

A command-line tool written in Rust to compute the hash value of a given file.  
Supports SHA-256, SHA-512, and MD5. You can output the result to the console or to a file.

---

## 🔧 Build Instructions

1. Make sure Rust is installed:
   ```bash
   rustc --version
   ```

2. Navigate to the project directory and build:
   ```bash
   cargo build --release
   ```

3. The binary will be generated at `target/release/hash.exe` (on Windows)

---

## 🚀 Usage

```bash
cargo run -- --file <file_path> [--hash <algorithm>] [--output <output_file>]
```

---

## ✅ Arguments

| Option              | Description                               |
|---------------------|-------------------------------------------|
| `--file` or `-f`     | Path to the target file (required)        |
| `--hash` or `-H`     | Hash algorithm (default: `sha256`)        |
| `--output` or `-o`   | Output file path (default: stdout)        |

---

## 🧪 Examples

```bash
cargo run -- --file ./example.txt --hash sha512
cargo run -- --file ./example.txt --hash md5 --output ./result.txt
```
