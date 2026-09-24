# Leptos CSR 加 workers-rs，样式仍用 Tailwind

PosterGen 的界面和 Worker 都改成 Rust，编译到 WebAssembly。界面用 Leptos 的客户端渲染，Worker 用 workers-rs。Tailwind v4 负责和纸外观，Wrangler 负责部署；这两样是 Node 工具，只用 pnpm 安装和运行，不用 npm 或 npx。

Cloudflare 仍然会为 Rust Worker 生成一层 JavaScript 入口。这一点换不掉，所以仓库里留着 Wrangler，而不是再写一套 TypeScript 应用。
