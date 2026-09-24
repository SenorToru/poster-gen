# 密钥留在浏览器，Worker 只按次转发

Gemini 和 xAI 的密钥不再放进 Worker 的环境变量。每个人在页面上为自己要使用的 Provider 填写 Credential，浏览器把它记在 localStorage 里。每次出图，浏览器用 Authorization 头把这一次的密钥交给同源 Worker，Worker 转给固定的上游主机后就丢掉，不写入日志，也不存下来。

直接从浏览器调用 xAI 没有可用的跨域路径，所以保留 Worker 做同源代理。上游主机只有 api.x.ai 和 generativelanguage.googleapis.com，模型 id 可以改，但不能把 Worker 变成任意网址的转发器。
