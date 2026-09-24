# PosterGen

一个在浏览器里把商品图重混成商业海报的工具。调用图像服务的密钥由使用它的人自己保管。

## Language

**Poster**:
一次 Remix 产出的商业图。

**Remix**:
从必填的基础图，加上至少一张参照，做出的一次出图。
_Avoid_: 生成、渲染

**参照**:
产品图、上传的人物，或一张模特图。Remix 至少要有其中一张。

**Provider**:
Grok Imagine 或 Gemini。只有这两种协议。
_Avoid_: 模型厂商、API

**Credential**:
某人在自己的浏览器里为一个 Provider 保存的密钥和模型 id。
_Avoid_: API key、令牌、环境变量

**模特图**:
按人物属性生成、用作人物参照的图。
_Avoid_: 头像、人像模板

**Style**:
六种固定画面描述之一。选中的 Style 会写进 Remix 的提示，不改变参照规则。
_Avoid_: 模板、滤镜
