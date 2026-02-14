# Yanify - 喫煙者構文変換

## Project Overview
入力テキストを喫煙者構文に変換するWebアプリ。Rust (Axum) バックエンド + React (Vite + Tailwind) フロントエンドのモノレポ構成。

## Tech Stack
- **Backend**: Rust + Axum + tokio + serde
- **Frontend**: React + TypeScript + Tailwind CSS (Vite)
- **Package Manager**: Nix (flake.nix) + npm (frontend)

## Project Structure
```
yanify/
├── flake.nix / .envrc       # Nix開発環境 (direnv連携)
├── Cargo.toml               # Rustプロジェクト設定
├── src/
│   ├── main.rs              # Axumサーバー (port 3000, API + 静的ファイル配信)
│   └── transform.rs         # 喫煙者構文変換ロジック + ユニットテスト
└── frontend/
    ├── package.json         # npm設定
    ├── vite.config.ts       # Vite設定 (dev時は/apiをlocalhost:6543にproxy)
    └── src/
        ├── App.tsx          # メインコンポーネント
        ├── main.tsx         # エントリポイント
        └── components/
            └── TransformCard.tsx  # 変換UI
```

## Commands
- `nix develop` or `direnv allow`: 開発環境のセットアップ
- `cargo test`: ユニットテスト実行 (18テスト)
- `cargo build`: バックエンドビルド
- `cargo run`: サーバー起動 (http://localhost:6543)
- `cd frontend && npm install`: フロントエンド依存インストール
- `cd frontend && npm run dev`: フロントエンド開発サーバー (Vite)
- `cd frontend && npm run build`: フロントエンドプロダクションビルド

## API
- `POST /api/transform` - `{ "text": "..." }` → `{ "original": "...", "transformed": "..." }`
- `GET /` - frontend/dist/ の静的ファイル配信

## Development Workflow
1. `cd frontend && npm run build` でフロントエンドをビルド
2. `cargo run` でサーバー起動
3. http://localhost:6543 でアクセス

## Transform Rules
1. 語彙置換: 美味しい→ニコチンが染みる, 最高→一服の至福, 休憩→一服タイム
2. 挨拶変換: おはよう→おはヤニ, おやすみ→おやすヤニ
3. 煙の表現: 〜→〜〜〜💨
4. 感嘆符: ！/!→🚬💨
5. 句読点: 。→🚬, 、→🚬
6. 語尾変換: 文末にヤニ/ヤニね/ヤニよをランダム追加
7. フィラー挿入: 文間にふぅ.../一服...をランダム挿入

## Agent Team Configuration
チーム開発時は以下の構成で作業する:
- **team-lead**: プロジェクト管理、統合テスト
- **backend-dev** (general-purpose): Rust バックエンド担当 (src/)
- **frontend-dev** (general-purpose): React フロントエンド担当 (frontend/)

チーム作成コマンド:
```
TeamCreate: team_name="smoker-app", description="喫煙者構文変換Webアプリ開発チーム"
```

## Notes
- macOS では Nix 環境内 (`nix develop`) でビルドすること (libiconv依存)
- Axum 0.8 では `nest_service("/", ...)` は非推奨、`fallback_service(...)` を使用
