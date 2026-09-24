# 覆盖现有的 poster-gen Worker

新版本部署到已经在跑的 Cloudflare Worker `poster-gen`，不另建一个叫 post-gen 的脚本。GitHub 仓库也叫 poster-gen。页面标题仍然是 PosterGen。

覆盖会换掉原来的 TypeScript 版本。Worker 上已有的 GEMINI_API_KEY 和 XAI_API_KEY 留着不删，这样如果把 Cloudflare 的版本回滚到旧脚本，旧脚本还读得到密钥。新代码不读取这两把密钥。
