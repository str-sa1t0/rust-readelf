# Rust で実装した簡易的なreadelf

Rust で実装した簡易的な `readelf` です。

ELF ファイルを読み込み、ファイルヘッダ、プログラムヘッダ、セクションヘッダ、シンボルテーブル、動的リンク情報、リロケーション情報、ノート情報などを表示します。

- ELF Header
- Program Header
- Section Header
- Symbol Table
- Dynamic Section
- Relocation
- Note Section

## ディレクトリ構成

```text
src/
├── rust-readelf.rs
├── cli.rs
├── types.rs
└── printers/
    ├── mod.rs
    ├── file_header.rs
    ├── program_headers.rs
    ├── section_headers.rs
    ├── symbols.rs
    ├── dynamic.rs
    ├── relocations.rs
    └── notes.rs
```

## ビルド方法

```bash
cargo build
```

コンパイル確認のみ行う場合は、以下を実行します。

```bash
cargo check
```

## 使い方


```bash
cargo run -- --file-name <ELFファイル> <オプション>
```

例：

```bash
cargo run -- --file-name /bin/ls --file-header
```

## 使用例

### ELF ヘッダを表示

```bash
cargo run -- --file-name /bin/ls --file-header
```

### プログラムヘッダを表示

```bash
cargo run -- --file-name /bin/ls --program-headers
```

### セクションヘッダを表示

```bash
cargo run -- --file-name /bin/ls --section-headers
```

### シンボルテーブルを表示

```bash
cargo run -- --file-name /bin/ls --symbols
```

### 動的シンボルを表示

```bash
cargo run -- --file-name /bin/ls --dynamic-symbols
```

### dynamic セクションを表示

```bash
cargo run -- --file-name /bin/ls --dynamic
```

### リロケーション情報を表示

```bash
cargo run -- --file-name /bin/ls --relocations
```

### note 情報を表示

```bash
cargo run -- --file-name /bin/ls --notes
```
