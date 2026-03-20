# Rust 基礎知識

ssotyle の開発を通じて Rust を学ぶための基礎ガイドである。
全体を網羅するのではなく、このプロジェクトで実際に使う概念に絞って説明する。

## プロジェクト構成

`cargo init` で生成されたファイル:

```
ssotyle/
├── Cargo.toml    # パッケージ設定と依存クレートの管理
└── src/
    └── main.rs   # エントリポイント
```

### Cargo.toml

Rust の依存管理ファイル。JavaScript でいう `package.json` に相当する。

```toml
[package]
name = "ssotyle"       # パッケージ名
version = "0.1.0"      # バージョン
edition = "2024"       # Rust エディション (言語仕様のバージョン)

[dependencies]
# ここに外部クレート (ライブラリ) を追加する
```

## 基本の型

### スカラー型

```rust
let x: i32 = 42;          // 符号付き 32 ビット整数
let y: f64 = 3.14;        // 64 ビット浮動小数点
let flag: bool = true;     // 真偽値
let c: char = 'あ';        // Unicode 文字
```

### 文字列

Rust には 2 種類の文字列がある。

```rust
// &str: 文字列スライス (参照)。文字列リテラルはこの型になる
let s1: &str = "hello";

// String: ヒープに確保される可変長文字列
let s2: String = String::from("hello");
let s3: String = "hello".to_string();  // 同じ意味
```

ssotyle ではトークン名やパスの操作で `String` を多用する。

### コレクション

```rust
// Vec: 可変長配列。JavaScript の Array に近い
let mut tokens: Vec<String> = Vec::new();
tokens.push("color".to_string());

// HashMap: キーと値のペア。JavaScript の Object / Map に近い
use std::collections::HashMap;
let mut map: HashMap<String, String> = HashMap::new();
map.insert("brand".to_string(), "#ff0000".to_string());
```

## 所有権 (Ownership)

Rust 最大の特徴。メモリ安全性をコンパイル時に保証する仕組みである。

### ルール

- 値には「所有者」が 1 つだけ存在する
- 所有者がスコープを抜けると、値は自動的に解放される
- 所有権は「移動 (move)」するか「借用 (borrow)」する

```rust
let s1 = String::from("hello");
let s2 = s1;  // 所有権が s1 → s2 に移動 (move)
// println!("{}", s1);  // コンパイルエラー！s1 はもう使えない
println!("{}", s2);     // OK
```

### 借用 (Borrowing)

所有権を移動せず、参照を渡すことを「借用」という。

```rust
fn print_token(name: &str) {    // &str は借用 (読み取り専用)
    println!("{}", name);
}

let token_name = String::from("colors-brand");
print_token(&token_name);       // & で借用を渡す
println!("{}", token_name);     // まだ使える
```

### 可変借用

```rust
fn add_prefix(name: &mut String) {  // &mut は可変借用
    name.insert_str(0, "--");
}

let mut token_name = String::from("colors-brand");
add_prefix(&mut token_name);
// token_name は "--colors-brand" になっている
```

制約: ある値に対して、不変借用は同時に複数可能だが、可変借用は同時に 1 つだけ。

## 構造体 (Struct)

データをまとめる。ssotyle のトークンやコンフィグを表現するのに使う。

```rust
struct DesignToken {
    value: String,
    token_type: Option<String>,  // Option は値がないかもしれないことを表す
    name: String,
    path: Vec<String>,
}

// インスタンス生成
let token = DesignToken {
    value: "#ff0000".to_string(),
    token_type: Some("color".to_string()),
    name: "colors-brand".to_string(),
    path: vec!["colors".to_string(), "brand".to_string()],
};
```

### impl ブロック

構造体にメソッドを定義する。

```rust
impl DesignToken {
    // 関連関数 (コンストラクタとしてよく使う)
    fn new(value: String, path: Vec<String>) -> Self {
        let name = path.join("-");
        DesignToken {
            value,
            token_type: None,
            name,
            path,
        }
    }

    // メソッド (&self は自身への不変参照)
    fn css_variable_name(&self) -> String {
        format!("--{}", self.path.join("-"))
    }
}

let token = DesignToken::new("#ff0000".to_string(), vec!["colors".to_string(), "brand".to_string()]);
println!("{}", token.css_variable_name()); // "--colors-brand"
```

## 列挙型 (Enum)

取り得る値を列挙する。Rust の enum はデータを持てるのが特徴。

```rust
// トークンの値は文字列か、他のトークンへの参照か
enum TokenValue {
    Literal(String),              // 直接の値
    Reference(String),            // "{colors.brand}" のような参照
}

// パターンマッチで分岐する
fn resolve(val: &TokenValue) -> String {
    match val {
        TokenValue::Literal(s) => s.clone(),
        TokenValue::Reference(path) => {
            // 参照を解決する処理
            format!("(resolved: {})", path)
        }
    }
}
```

### Option と Result

Rust には null がない。代わりに `Option` と `Result` を使う。

```rust
// Option<T>: 値があるかないか
let found: Option<&str> = Some("hello");
let not_found: Option<&str> = None;

// match で安全に取り出す
match found {
    Some(value) => println!("見つかった: {}", value),
    None => println!("見つからない"),
}

// if let で簡潔に書くこともできる
if let Some(value) = found {
    println!("見つかった: {}", value);
}
```

```rust
// Result<T, E>: 成功か失敗か
use std::fs;

fn read_config(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

match read_config("config.json") {
    Ok(content) => println!("読み込み成功"),
    Err(e) => println!("エラー: {}", e),
}
```

### ? 演算子

`Result` を返す関数の中で `?` を使うと、エラー時に即座に返す。

```rust
fn read_config(path: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path)?;  // エラーなら即 return Err(e)
    Ok(content)
}
```

## トレイト (Trait)

共通のインターフェースを定義する。JavaScript の interface に近い。

```rust
// トランスフォームの共通インターフェース
trait Transform {
    fn name(&self) -> &str;
    fn transform_value(&self, value: &str) -> String;
}

// 具体的な実装
struct KebabCase;

impl Transform for KebabCase {
    fn name(&self) -> &str {
        "name/kebab"
    }

    fn transform_value(&self, value: &str) -> String {
        value.to_lowercase().replace(' ', "-")
    }
}
```

## モジュールシステム

Rust はファイルとディレクトリでモジュールを構成する。

```
src/
├── main.rs          # use ssotyle::config; で使う
├── lib.rs           # ライブラリルート。pub mod で公開する
├── config.rs        # config モジュール
├── token.rs         # token モジュール
└── transform/
    ├── mod.rs       # transform モジュールのルート
    └── builtins.rs  # transform::builtins サブモジュール
```

```rust
// lib.rs
pub mod config;
pub mod token;
pub mod transform;
```

```rust
// main.rs
use ssotyle::config::Config;

fn main() {
    let config = Config::load("config.json").unwrap();
    println!("{:?}", config);
}
```

## serde による JSON パース

ssotyle の核心機能の一つ。`serde` と `serde_json` クレートを使う。

`Cargo.toml` に追加:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// derive マクロで JSON ⇔ Rust 構造体の変換を自動生成
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    source: Vec<String>,
    platforms: HashMap<String, PlatformConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]  // JSON の camelCase を Rust の snake_case に対応
struct PlatformConfig {
    transform_group: Option<String>,
    build_path: Option<String>,
    files: Option<Vec<FileConfig>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FileConfig {
    destination: String,
    format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "source": ["tokens/**/*.json"],
        "platforms": {
            "css": {
                "transformGroup": "css",
                "buildPath": "build/css/",
                "files": [{
                    "destination": "_variables.css",
                    "format": "css/variables"
                }]
            }
        }
    }"#;

    let config: Config = serde_json::from_str(json)?;
    println!("{:#?}", config);
    Ok(())
}
```

## エラーハンドリング

ssotyle では以下のようなエラーが想定される:

- ファイルが見つからない
- JSON パースエラー
- 循環参照
- 未知の transform / format 名

`thiserror` クレートを使うと、独自のエラー型を簡潔に定義できる。

```toml
[dependencies]
thiserror = "2"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum SsotyleError {
    #[error("ファイルの読み込みに失敗: {path}")]
    FileRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("JSON パースエラー: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("循環参照を検出: {0}")]
    CircularReference(String),

    #[error("不明な transform: {0}")]
    UnknownTransform(String),
}
```

## よく使うコマンド

```sh
cargo build          # コンパイル
cargo run            # コンパイルして実行
cargo test           # テスト実行
cargo check          # コンパイルチェック (ビルドより速い)
cargo clippy         # lint (より良い書き方の提案)
cargo fmt            # コードフォーマット
cargo add serde      # 依存クレートの追加
```
