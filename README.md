# PosterGen

把商品图重混成商业海报。界面是 Leptos（编译成 WebAssembly），请求由 Cloudflare Worker（workers-rs）转给 xAI Grok Imagine 或 Google Gemini。密钥由使用者填在页面上，保存在自己的浏览器里，Worker 不保存。

## 准备

- Rust stable，以及 `wasm32-unknown-unknown`
- [pnpm](https://pnpm.io/) 10。本机若没有 `pnpm` 命令，用 Node 自带的 Corepack：`corepack pnpm`
- `trunk` 和 `worker-build`：

```bash
cargo install trunk worker-build
```

## 本地

```bash
corepack pnpm install
corepack pnpm run check
export PATH="$HOME/.cargo/bin:$PATH"
corepack pnpm exec wrangler dev
```

打开 Wrangler 给出的本地地址。在页面上选择 Grok 或 Gemini，填入自己的密钥和模型 id。没有密钥时不能生成，服务器上也没有默认密钥。

Remix 需要基础图，以及至少一张参照：产品图、上传的人物，或生成的模特。

## 部署

部署目标是已经存在的 Worker `poster-gen`。这会覆盖线上正在跑的版本。

```bash
corepack pnpm run deploy
```

`pnpm run deploy` 会构建 CSS、WASM 界面和 Worker，然后执行 `wrangler deploy`。不要用 npm 或 npx。

## 布局

- `crates/core`：Remix 门槛、提示词、两种上游请求。不接触密钥。
- `crates/app`：浏览器界面。
- `crates/worker`：只接受 `/api/generate-model` 和 `/api/remix`，按次转发。
- `style/input.css`：和纸主题。
