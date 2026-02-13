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

## Tech Stack

- **Backend**: Rust + [Axum](https://github.com/tokio-rs/axum)
- **Frontend**: React + TypeScript + [Tailwind CSS](https://tailwindcss.com/) + [Vite](https://vite.dev/)
- **Dev Environment**: [Nix Flakes](https://nixos.wiki/wiki/Flakes) + [direnv](https://direnv.net/)

## Getting Started

### Prerequisites

- [Nix](https://nixos.org/download/) (with flakes enabled)
- [direnv](https://direnv.net/) (optional)

### Setup

```bash
# direnvを使う場合
direnv allow

# または直接nix developに入る
nix develop
```

### Build & Run

```bash
# フロントエンドビルド
cd frontend && npm install && npm run build && cd ..

# サーバー起動
cargo run
```

http://localhost:3000 にアクセス。

### Development

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

## License

MIT
