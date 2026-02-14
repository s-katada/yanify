# Yanify - 喫煙者構文変換

## Project Overview
入力テキストを喫煙者構文に変換するWebアプリ。Cargo Workspace構成で、変換ロジックをローカルサーバーとCloudflare Workerで共有する。

## Tech Stack
- **Transform**: Rust (共有ライブラリ `yanify-transform`)
- **Server**: Rust + Axum (ローカル開発用 `yanify-server`)
- **Worker**: Rust → WASM + Cloudflare Workers (`yanify-worker`)
- **Frontend**: React + TypeScript + Tailwind CSS (Vite)
- **Infra**: Cloudflare Pages (フロントエンド) + Cloudflare Workers (API)
- **Dev Environment**: Nix (flake.nix) + npm (frontend)

## Project Structure
```
yanify/
├── Cargo.toml                    # Workspace ルート
├── wrangler.toml                 # Cloudflare Worker 設定
├── flake.nix / .envrc            # Nix開発環境 (direnv連携)
├── crates/
│   ├── yanify-transform/         # 共有変換ロジック (lib crate)
│   │   └── src/lib.rs            # 変換ルール + 18ユニットテスト
│   ├── yanify-server/            # ローカル開発用 Axum サーバー
│   │   └── src/main.rs           # port 6543, API + 静的ファイル配信
│   └── yanify-worker/            # Cloudflare Worker (WASM)
│       └── src/lib.rs            # POST /api/transform + CORS
└── frontend/
    ├── package.json
    ├── vite.config.ts             # dev時は /api を localhost:6543 にプロキシ
    ├── .env.production            # 本番 Worker URL (VITE_API_URL)
    └── src/
        ├── App.tsx
        ├── main.tsx
        ├── index.css
        └── components/
            └── TransformCard.tsx
```

## Commands

### 開発
- `nix develop` or `direnv allow`: 開発環境セットアップ
- `cargo test`: ユニットテスト実行 (18テスト)
- `cargo run -p yanify-server`: ローカルサーバー起動 (http://localhost:6543)
- `cd frontend && npm run dev`: フロントエンド開発サーバー (Vite)
- `cd frontend && npm run build`: フロントエンドビルド

### デプロイ
- `wrangler deploy`: Worker デプロイ (Rust → WASM ビルド含む)
- `cd frontend && npm run build && wrangler pages deploy dist --project-name yanify`: Pages デプロイ

## API
- `POST /api/transform` - `{ "text": "..." }` → `{ "original": "...", "transformed": "..." }`
- ローカル: http://localhost:6543/api/transform
- 本番: https://yanify-api.shunya-saitama.workers.dev/api/transform

## Development Workflow
1. `cd frontend && npm run build` でフロントエンドをビルド
2. `cargo run -p yanify-server` でローカルサーバー起動
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
- **backend-dev** (general-purpose, bypassPermissions): Rust バックエンド担当 (crates/)
- **frontend-dev** (general-purpose, bypassPermissions): React フロントエンド担当 (frontend/)

チーム作成コマンド:
```
TeamCreate: team_name="smoker-app", description="喫煙者構文変換Webアプリ開発チーム"
```

## Notes
- macOS では Nix 環境内 (`nix develop`) でビルドすること (libiconv依存)
- Axum 0.8 では `nest_service("/", ...)` は非推奨、`fallback_service(...)` を使用
- worker crate は v0.7 以上が必要 (worker-build の要件)
- フロントエンドの API URL は `VITE_API_URL` 環境変数で切り替え (空文字でローカル)
