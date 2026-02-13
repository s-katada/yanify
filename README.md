# Yanify 🚬

入力テキストを**喫煙者構文**に変換するWebアプリ。

> おはよう！今日はいい天気ですね。
> → おはヤニヤニよ🚬💨今日はいい天気ですねヤニね🚬ふぅ...

## 変換ルール

| ルール | 変換前 | 変換後 |
|--------|--------|--------|
| 語彙置換 | 美味しい / 最高 / 休憩 | ニコチンが染みる / 一服の至福 / 一服タイム |
| 挨拶変換 | おはよう / おやすみ | おはヤニ / おやすヤニ |
| 煙の表現 | 〜 | 〜〜〜💨 |
| 感嘆符 | ！ / ! | 🚬💨 |
| 句読点 | 。 / 、 | 🚬 |
| 語尾変換 | (文末) | ヤニ / ヤニね / ヤニよ |
| フィラー | (文間) | ふぅ... / 一服... |

## 技術スタック

- **バックエンド**: Rust + [Axum](https://github.com/tokio-rs/axum)
- **フロントエンド**: React + TypeScript + [Tailwind CSS](https://tailwindcss.com/) + [Vite](https://vite.dev/)
- **開発環境**: [Nix Flakes](https://nixos.wiki/wiki/Flakes) + [direnv](https://direnv.net/)

## はじめかた

### 前提条件

- [Nix](https://nixos.org/download/) (flakes 有効化済み)
- [direnv](https://direnv.net/) (任意)

### セットアップ

```bash
# direnvを使う場合
direnv allow

# または直接nix developに入る
nix develop
```

### ビルド・起動

```bash
# フロントエンドビルド
cd frontend && npm install && npm run build && cd ..

# サーバー起動
cargo run
```

http://localhost:3000 にアクセス。

### 開発

```bash
# バックエンドテスト
cargo test

# フロントエンド開発サーバー (HMR)
cd frontend && npm run dev
```

## API

```
POST /api/transform
Content-Type: application/json

{ "text": "おはよう！今日はいい天気ですね。" }
```

```json
{
  "original": "おはよう！今日はいい天気ですね。",
  "transformed": "おはヤニヤニよ🚬💨今日はいい天気ですねヤニね🚬"
}
```

## ライセンス

MIT
