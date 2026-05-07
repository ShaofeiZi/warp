<a href="https://www.warp.dev">
    <img width="1024" alt="Warp 智能开发环境产品预览" src="https://github.com/user-attachments/assets/9976b2da-2edd-4604-a36c-8fd53719c6d4" />
</a>
<br />
<p align="center">
  <a href="https://www.warp.dev"><img height="20" alt="Built with Warp" src="./images/Built-With-Warp-Export@2x.png" /></a>
  <a href="https://oz.warp.dev"><img height="20" alt="Powered by Oz" src="./images/Powered-By-Oz-Export@2x.png" /></a>
</p>

<p align="center">
  <a href="https://www.warp.dev">官网</a>
  ·
  <a href="https://www.warp.dev/code">代码</a>
  ·
  <a href="https://www.warp.dev/agents">智能体</a>
  ·
  <a href="https://www.warp.dev/terminal">终端</a>
  ·
  <a href="https://www.warp.dev/drive">云盘</a>
  ·
  <a href="https://docs.warp.dev">文档</a>
  ·
  <a href="https://www.warp.dev/blog/how-warp-works">Warp 工作原理</a>
</p>

> [!NOTE]
> OpenAI 是全新开源 Warp 仓库的创始赞助商，全新的智能体管理工作流由 GPT 模型提供支持。

<h1></h1>

## 关于

[Warp](https://www.warp.dev) 是一款智能开发环境，诞生于终端。你可以使用 Warp 内置的编码智能体，也可以接入自己的 CLI 智能体（如 Claude Code、Codex、Gemini CLI 等）。

## 安装

你可以[下载 Warp](https://www.warp.dev/download) 并阅读我们的[文档](https://docs.warp.dev/)获取各平台的安装指南。

## Warp 贡献概览仪表板

访问 [build.warp.dev](https://build.warp.dev) 可以：
- 观看数千个 Oz 智能体进行 Issue 分流、编写规格说明、实施代码变更和审查 PR
- 查看顶级贡献者和进行中的功能开发
- 通过 GitHub 登录追踪你自己的 Issue
- 点击进入活跃的智能体会话，在网页版 Warp 终端中查看

## 许可证

Warp 的 UI 框架（`warpui_core` 和 `warpui` crate）采用 [MIT 许可证](LICENSE-MIT)。

本仓库中的其余代码采用 [AGPL v3](LICENSE-AGPL) 许可证。

## 开源与贡献

Warp 的客户端代码库是开源的，存放于本仓库。我们欢迎社区贡献，并设计了轻量级的工作流来帮助新贡献者快速上手。完整的贡献流程请阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 指南。

> [!TIP]
> **与贡献者和 Warp 团队交流** — 加入 Slack 频道 [`#oss-contributors`](https://warpcommunity.slack.com/archives/C0B0LM8N4DB)，这里适合进行非正式提问、设计讨论和与维护者结对协作。如果你是新成员，请先[加入 Warp Slack 社区](https://go.warp.dev/join-preview)，然后进入 `#oss-contributors` 频道。

如果你在维护一个热门的开源项目，可以[申请 Oz 额度](https://tally.so/r/LZWxqG)，为你的仓库引入[智能体工作流](https://github.com/warpdotdev/oz-for-oss)，如 Issue 分流、PR 审查和社区管理。

### 从 Issue 到 PR

在提交之前，请先[搜索现有 Issue](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+sort%3Areactions-%2B1-desc)，确认是否已有相同的 Bug 或功能请求。如果没有，请使用我们的模板[提交 Issue](https://github.com/warpdotdev/warp/issues/new/choose)。安全漏洞应按照 [CONTRIBUTING.md](CONTRIBUTING.md#reporting-security-issues) 中的说明私下报告。

提交后，Warp 维护者会审核 Issue，并可能添加就绪标签：[`ready-to-spec`](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-spec) 表示设计已开放，欢迎贡献者编写规格说明；[`ready-to-implement`](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-implement) 表示设计已确定，欢迎提交代码 PR。任何人都可以认领带标签的 Issue — 如果希望某个 Issue 被考虑添加就绪标签，请在 Issue 中提及 **@oss-maintainers**。

### 本地构建仓库

从源码构建并运行 Warp：

```bash
./script/bootstrap   # 平台特定的环境配置
./script/run         # 构建并运行 Warp
./script/presubmit   # 代码格式化、clippy 检查和测试
```

完整的工程指南（包括编码风格、测试和平台特定说明）请参见 [WARP.md](WARP.md)。

## 加入团队

有兴趣加入我们？请查看我们的[开放职位](https://www.warp.dev/careers)。

## 支持与问答

1. 请参阅我们的[文档](https://docs.warp.dev/)，获取 Warp 功能的全面指南。
2. 加入我们的 [Slack 社区](https://go.warp.dev/join-preview)，与其他用户交流并向 Warp 团队获取帮助 — 贡献者常驻 [`#oss-contributors`](https://warpcommunity.slack.com/archives/C0B0LM8N4DB) 频道。
3. 尝试我们的 [Preview 构建](https://www.warp.dev/download-preview)，测试最新的实验性功能。
4. 在任何 Issue 中提及 **@oss-maintainers** 可将问题升级至团队 — 例如，当你遇到自动化智能体相关问题时。

## 行为准则

我们要求所有人保持尊重和同理心。Warp 遵循[行为准则](CODE_OF_CONDUCT.md)。如需举报违规行为，请发送邮件至 warp-coc at warp.dev。

## 开源依赖

我们要感谢以下[开源依赖](https://docs.warp.dev/help/licenses)对 Warp 项目的帮助：

* [Tokio](https://github.com/tokio-rs/tokio) — 异步运行时
* [NuShell](https://github.com/nushell/nushell) — 现代化 Shell
* [Fig Completion Specs](https://github.com/withfig/autocomplete) — 自动补全规范
* [Warp Server Framework](https://github.com/seanmonstar/warp) — Web 服务框架
* [Alacritty](https://github.com/alacritty/alacritty) — GPU 加速终端
* [Hyper HTTP library](https://github.com/hyperium/hyper) — HTTP 库
* [FontKit](https://github.com/servo/font-kit) — 字体渲染
* [Core-foundation](https://github.com/servo/core-foundation-rs) — macOS 核心基础绑定
* [Smol](https://github.com/smol-rs/smol) — 轻量异步运行时
