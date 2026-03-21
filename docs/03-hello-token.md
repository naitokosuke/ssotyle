# Phase 0: Hello, Token

最初の一歩として、JSON ファイルを読み込んで中身を表示するプログラムを作る。

## この章で学ぶこと

- `std::fs` によるファイル読み込み
- `Result` によるエラーハンドリング
- コマンドライン引数の取得 (`std::env::args`)
- 自作 JSON パーサーの実装 (文字列の走査、再帰下降パーサー)
- enum を使った JSON 値の表現
- トレイトによるパーサーの抽象化

## ゴール

```sh
cargo run -- tokens/colors.json
```

を実行すると、以下のようにトークンが一覧表示される。

```
  colors.black = "#000000"
  colors.white = "#ffffff"
  colors.brand = "{colors.orange.500}"
  colors.orange.500 = "#ed8936"
  colors.orange.700 = "#c05621"
```

## 準備

### テスト用トークンファイルの作成

プロジェクトルートに `tokens/colors.json` を作成する。
これは Style Dictionary の DTCG 形式に従ったデザイントークンである。

```json
{
  "colors": {
    "$type": "color",
    "black": {
      "$value": "#000000"
    },
    "white": {
      "$value": "#ffffff"
    },
    "brand": {
      "$value": "{colors.orange.500}"
    },
    "orange": {
      "500": {
        "$value": "#ed8936"
      },
      "700": {
        "$value": "#c05621"
      }
    }
  }
}
```

### 依存クレートについて

Phase 0 では外部クレートを使わず、標準ライブラリだけで JSON パーサーを自作する。
`Cargo.toml` の `[dependencies]` は空のままで良い。

後のフェーズで `serde_json` に差し替える。
トレイトで抽象化しておけば、パーサーの実装を自由に入れ替えられる。

## 知識ガイド

### JSON の構文

JSON パーサーを書くには、JSON の構文規則を知る必要がある。
デザイントークンで使う範囲に絞ると、以下の要素だけ対応すればよい。

| 要素 | 例 |
|------|------|
| 文字列 | `"hello"` |
| 数値 | `42`, `3.14` |
| 真偽値 | `true`, `false` |
| null | `null` |
| 配列 | `[1, 2, 3]` |
| オブジェクト | `{"key": "value"}` |

オブジェクトの構文:

```
{ "key1": value1, "key2": value2 }
```

- `{` で始まる
- `"key": value` のペアが `,` で区切られる
- `}` で終わる
- ホワイトスペース (空白、改行、タブ) は無視される

### enum で JSON 値を表現する

Rust の enum はデータを持てる。これを使って JSON の値を表現できる。

```rust
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),  // キーと値のペアのリスト
}
```

これが自作パーサーの出力となる内部表現である。
`Object` に `HashMap` ではなく `Vec<(String, JsonValue)>` を使うと、キーの順序が保持される。

### 再帰下降パーサーとは

JSON パーサーは「再帰下降パーサー」で実装するのが最も自然である。
各 JSON 要素に対応する関数を作り、互いに呼び合う構造になる。

```mermaid
graph TD
    A["parse_value"] --> B["parse_string"]
    A --> C["parse_number"]
    A --> D["parse_object"]
    A --> E["parse_array"]
    A --> F["parse_bool / parse_null"]
    D -->|各 value を| A
    E -->|各要素を| A
```

パーサーは文字列の「現在位置」を進めながら読み取る。

```rust
struct Parser {
    input: Vec<char>,  // 入力文字列
    pos: usize,        // 現在位置
}
```

基本的なパターン:

- 現在の文字を見て (`input[pos]`)、何をパースするか決める
- パースしたら `pos` を進める
- 予期しない文字に出会ったらエラーを返す

### std::fs::read_to_string

ファイルの中身を `String` として読み込む。戻り値は `Result<String, std::io::Error>` である。

### std::env::args

コマンドライン引数を取得する。`cargo run -- tokens/colors.json` と実行した場合:

- `args[0]` = 実行ファイルパス
- `args[1]` = `"tokens/colors.json"` (ユーザーが渡した引数)

### ? 演算子と main の戻り値

`main` の戻り値を `Result` にすると、`?` 演算子でエラーを簡潔に処理できる。

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("file.json")?;  // エラーなら即終了
    Ok(())
}
```

`Box<dyn std::error::Error>` は「あらゆるエラー型を受け取れる箱」である。

### 再帰と可変借用

`Vec` を `&mut` で渡すと、呼び出し先で `push` / `pop` して状態を管理できる。
再帰関数でパスを積み上げていくのに使う。

```rust
fn walk(path: &mut Vec<String>) {
    path.push("child".to_string());
    // ... 再帰呼び出し ...
    path.pop();  // 戻るときに元に戻す
}
```

## 課題

### 課題 0: コマンドライン引数でファイルパスを受け取る

`std::env::args` を使い、引数が足りないときはエラーメッセージを表示して終了するプログラムを書こう。

確認:

```sh
cargo run                        # → "Usage: ssotyle <file>" と表示されること
cargo run -- tokens/colors.json  # → ファイルパスが表示されること
```

ヒント: `eprintln!` は標準エラー出力に書き出す。`std::process::exit(1)` で異常終了できる。

<details>
<summary>課題 0 の解説</summary>

### 実装例

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, filepath] => {
            println!("{}", filepath);
        }
        _ => {
            eprintln!("Usage: ssotyle <file>");
            std::process::exit(1);
        }
    }
}
```

### `fn main()`

プログラムのエントリポイント (最初に実行される関数)。
`fn` は function (関数) の略。

### `let args: Vec<String> = ...`

`let` は変数宣言。`: Vec<String>` は型注釈で、「`String` の可変長配列」を意味する。
型注釈は省略可能で、コンパイラが推論してくれる場合が多い。

```rust
let args = std::env::args().collect::<Vec<String>>();  // 型注釈なし、turbofish で指定
let args: Vec<String> = std::env::args().collect();    // 型注釈あり (こちらが読みやすい)
```

### `std::env::args()`

標準ライブラリの `std` → `env` モジュール → `args` 関数。
コマンドライン引数のイテレータを返す。

`std::env::args()` と書く代わりに、ファイル冒頭で `use` して短く書くこともできる:

```rust
use std::env;
// ...
let args: Vec<String> = env::args().collect();
```

### `.collect()`

イテレータを具体的なコレクション型に変換するメソッド。
`args()` が返すのはイテレータ (遅延評価) なので、`.collect()` で `Vec` に確定させる。

### `match args.as_slice() { ... }`

`match` は Rust のパターンマッチ構文。値を複数のパターンと照合し、一致したアームを実行する。

`args.as_slice()` は `Vec<String>` を `&[String]` (スライス参照) に変換する。
スライスにすることで、要素数に基づくパターンマッチが使える。

```rust
match args.as_slice() {
    [_, filepath] => { ... }  // 要素がちょうど 2 つ
    _ => { ... }              // それ以外すべて
}
```

`match` は**すべてのパターンを網羅する**必要がある。
`_` (ワイルドカード) は「上のどのパターンにも一致しなかった場合」をキャッチする。

### `[_, filepath]`

スライスパターン。`[A, B]` は「要素がちょうど 2 つ」のスライスに一致する。

- `_` — 1 つ目の要素 (実行ファイルパス) を無視する
- `filepath` — 2 つ目の要素を `filepath` という名前に束縛 (バインド) する

束縛された `filepath` はそのアーム内で使える。型は `&String` (参照) になる。

スライスパターンの他の例:

```rust
[]           // 空のスライスに一致
[x]          // 要素が 1 つのスライスに一致、x に束縛
[_, _, z]    // 要素が 3 つ、3 つ目だけ使う
[first, ..]  // 1 つ以上、最初の要素だけ使う (.. は残りを無視)
```

### `=>` (ファットアロー)

`match` アームでパターンと実行するコードを繋ぐ記号。
「このパターンに一致したら、この処理を実行する」という意味。

### `println!` と `eprintln!`

どちらも文字列を出力するマクロ (`!` はマクロ呼び出しの印)。

- `println!` — 標準出力 (stdout) に書く。プログラムの正常な出力用
- `eprintln!` — 標準エラー出力 (stderr) に書く。エラーメッセージ用

`{}` はプレースホルダーで、引数の値が埋め込まれる:

```rust
println!("{}", filepath);  // filepath の中身が表示される
```

### `std::process::exit(1)`

プログラムを即座に終了する。引数は終了コード。

- `0` — 正常終了
- `1` 以上 — 異常終了

シェルが終了コードを見て成功/失敗を判断するため、エラー時は `1` を返す慣習がある。

</details>

### 課題 1: ファイルを読み込んで表示する

引数で受け取ったパスのファイルを読み込み、中身をそのまま表示しよう。

確認:

```sh
cargo run -- tokens/colors.json  # → JSON の中身がそのまま表示されること
```

ヒント: `std::fs::read_to_string` と `?` 演算子を使う。`main` の戻り値の型を変える必要がある。

<details>
<summary>課題 1 の解説</summary>

### 実装例

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, filepath] => {
            let content = std::fs::read_to_string(filepath)?;
            println!("{}", content);
        }
        _ => {
            eprintln!("Usage: ssotyle <file>");
            std::process::exit(1);
        }
    }
    Ok(())
}
```

### `-> Result<(), Box<dyn std::error::Error>>`

関数の戻り値の型。課題 0 の `main` は何も返さなかったが、`?` 演算子を使うために `Result` を返す必要がある。

- `Result<(), Box<dyn std::error::Error>>` — 成功時は `()` (何もない)、失敗時はエラー
- `Box<dyn std::error::Error>` — あらゆる種類のエラーを入れられる箱。`std::io::Error` (ファイル読み込み失敗) なども受け取れる

### `std::fs::read_to_string(filepath)`

ファイルの中身を `String` として読み込む。戻り値は `Result<String, std::io::Error>`。

- ファイルが存在すれば `Ok(String)` を返す
- ファイルが見つからない、読み取り権限がないなどの場合は `Err(std::io::Error)` を返す

### `?` 演算子

`Result` を返す式の末尾に `?` を付けると:

- `Ok(値)` なら → 値を取り出して処理を続ける
- `Err(エラー)` なら → 関数から即座に `Err(エラー)` を返す

```rust
let content = std::fs::read_to_string(filepath)?;
```

これは以下と同じ意味:

```rust
let content = match std::fs::read_to_string(filepath) {
    Ok(s) => s,
    Err(e) => return Err(e.into()),
};
```

`?` のおかげで 1 行で書ける。

### `Ok(())`

関数の最後に書く。「正常に終了した」ことを表す。
`Result` を返す関数なので、成功時も明示的に `Ok` で包む必要がある。

`()` は Rust の「何もない値」(ユニット型)。他の言語の `void` に近い。

</details>

### 課題 2: JsonValue enum を定義する

JSON の値を表す `JsonValue` enum を定義しよう。

対応すべき型:

- null
- 真偽値
- 数値 (f64)
- 文字列
- 配列
- オブジェクト

この時点ではまだ使わなくて良い。定義だけで OK。

### 課題 3: 簡易 JSON パーサーを作る

ここが本題。`Parser` 構造体を作り、JSON 文字列を `JsonValue` に変換する。

まず以下のヘルパーメソッドを実装する:

- `skip_whitespace` — 空白・改行・タブを読み飛ばす
- `peek` — 現在位置の文字を返す (位置は進めない)
- `advance` — 現在位置を 1 つ進める

次に、各 JSON 要素のパース関数を実装する。推奨する順序:

- `parse_string` — `"` で始まり `"` で終わる文字列を読む
- `parse_number` — 数字と `.` を読んで `f64` に変換する
- `parse_null` — `null` の 4 文字を読む
- `parse_bool` — `true` または `false` を読む
- `parse_array` — `[` で始まり、`,` 区切りで `parse_value` を呼び、`]` で終わる
- `parse_object` — `{` で始まり、`"key": value` のペアを読み、`}` で終わる
- `parse_value` — 最初の文字を見て上記のどれかを呼び分ける

確認:

```rust
let input = r#"{"name": "test", "value": 42}"#;
// パースして JsonValue::Object が得られること
```

ヒント:

- `parse_value` は `peek()` の結果で分岐する。`"` なら文字列、`{` ならオブジェクト、`[` なら配列、`t`/`f` なら真偽値、`n` なら null、数字なら数値
- エラーは `Result<JsonValue, String>` で返すのが最も簡単。エラーメッセージに現在位置を含めると、デバッグしやすい
- エスケープシーケンス (`\"`, `\\` 等) は最初は無視して良い。デザイントークンでは滅多に出てこない

### 課題 4: トークンを再帰的に探索して一覧表示する

パースした `JsonValue` を再帰的に走査し、`$value` を持つノードをトークンとして表示する関数を作ろう。

以下のシグネチャの関数を実装する:

```rust
fn visit_tokens(value: &JsonValue, path: &mut Vec<String>) {
    // ここを実装する
}
```

考えるポイント:

- `value` がオブジェクトでなければ何もしない
- オブジェクトに `$value` キーがあれば、それはトークン。`path` を `.` で結合して表示する
- `$value` がなければ、各キーについて再帰的に探索する
- `$` で始まるキー (`$type` 等) はメタデータなのでスキップする

確認:

```sh
cargo run -- tokens/colors.json
```

期待する出力:

```
  colors.black = "#000000"
  colors.white = "#ffffff"
  colors.brand = "{colors.orange.500}"
  colors.orange.500 = "#ed8936"
  colors.orange.700 = "#c05621"
```

## チャレンジ課題

Phase 0 の理解を深めるための追加課題。必須ではない。

- `$type` の情報も一緒に表示してみよう。`$type` はグループレベルで定義され、子トークンに継承される。`visit_tokens` に `current_type: Option<&str>` 引数を追加して、型の伝播を実装できるか試してみよう
- 存在しないファイルパスを渡したとき、どんなエラーメッセージが表示されるか確認しよう。JSON として不正な内容のファイルを渡した場合はどうなるか
- `tokens/dimensions.json` を新たに作成し、2 つのファイルを順番に読み込んで両方のトークンを表示するプログラムに改造してみよう (Phase 1 への布石)

## 次のステップへ

Phase 0 で自作した `JsonValue` と `Parser` は、Phase 1 で `Parser` トレイトとして抽象化する。

```rust
trait TokenParser {
    fn extensions(&self) -> &[&str];
    fn parse(&self, content: &[u8]) -> Result<JsonValue, SsotyleError>;
}
```

こうすることで、自作パーサーと serde ベースのパーサーを自由に差し替えられるようになる。

```mermaid
graph TD
    T["TokenParser trait"] --> H["HandWrittenParser\n(Phase 0 で作ったもの)"]
    T --> S["SerdeJsonParser\n(serde_json ベース)"]
    T --> Y["YamlParser\n(Phase 8 で追加)"]
    H --> V["JsonValue"]
    S --> V
    Y --> V
```
