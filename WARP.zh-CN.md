# WARP.md

本文件为在此仓库中工作时提供工程指导。

## 开发命令

### 构建与运行
- `cargo run` - 在本地构建并运行 Warp
- `cargo bundle --bin warp` - 打包主应用程序

### 连接本地 warp-server 运行
要将 Warp 客户端连接到本地 warp-server 实例：

```bash
# 连接到默认端口 8080 上的服务器
cargo run --features with_local_server

# 连接到自定义端口上的服务器（例如 8082）
SERVER_ROOT_URL=http://localhost:8082 WS_SERVER_URL=ws://localhost:8082/graphql/v2 cargo run --features with_local_server
```

环境变量：
- `SERVER_ROOT_URL` - HTTP 端点（默认值：`http://localhost:8080`）
- `WS_SERVER_URL` - WebSocket 端点（默认值：`ws://localhost:8080/graphql/v2`）

### 测试
- `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2` - 使用 nextest 运行测试（`--no-fail-fast` 确保即使部分测试失败也会继续运行剩余测试）
- `cargo nextest run -p warp_completer --features v2` - 运行启用了 v2 特性的补全器测试
- `cargo test --doc` - 运行文档测试（验证代码示例的正确性）
- `cargo test` - 运行单个包的标准测试

### 代码检查与格式化
- `./script/presubmit` - 运行所有提交前检查（包括 fmt、clippy 和测试）
- `cargo fmt` - 格式化 Rust 代码
- `cargo clippy --workspace --all-targets --all-features --tests -- -D warnings` - 运行 clippy 静态分析（将警告视为错误）
- `./script/run-clang-format.py -r --extensions 'c,h,cpp,m' ./crates/warpui/src/ ./app/src/` - 格式化 C/C++/Obj-C 代码
- `find . -name "*.wgsl" -exec wgslfmt --check {} +` - 检查 WGSL 着色器代码格式

### 平台设置
- `./script/bootstrap` - 执行平台特定的初始化设置（内部调用各平台对应的 bootstrap 脚本）
- `./script/install_cargo_build_deps` - 安装 Cargo 构建依赖
- `./script/install_cargo_test_deps` - 安装 Cargo 测试依赖

## 架构概览

本项目是一个基于 Rust 的终端模拟器，使用名为 **WarpUI** 的自定义 UI 框架构建。

### 核心组件

**WarpUI 框架** (`ui/`)：
- 自定义 UI 框架，采用 Entity-Component-Handle 模式（实体-组件-句柄模式，一种将数据与引用解耦的设计模式）
- 全局 `App` 对象拥有所有视图/模型（实体）的所有权
- 视图通过持有 `ViewHandle<T>` 引用其他视图（而非直接持有引用，从而避免所有权冲突）
- `AppContext` 在渲染/事件处理期间提供对句柄的临时访问
- Element 用于描述视觉布局（灵感来自 Flutter 的声明式 UI 模型）
- Actions 系统用于事件处理
- MouseStateHandle 必须在构造期间创建一次，然后在任何需要鼠标输入的地方引用/克隆它来跟踪鼠标状态变化。如果在渲染时内联使用 `MouseStateHandle::default()`，将导致所有鼠标交互失效。

**主应用** (`app/`)：
- 终端模拟和 Shell 管理 (`terminal/`)
- AI 集成，包括 Agent Mode（智能代理模式）(`ai/`)
- 云同步和 Drive 功能 (`drive/`)
- 认证和用户管理 (`auth/`)
- 设置和偏好配置 (`settings/`)
- 工作区和会话管理 (`workspace/`)

**核心库**：
- `crates/warp_core/` - 核心工具函数和平台抽象层
- `crates/editor/` - 文本编辑功能
- `crates/warpui/` 和 `crates/warpui_core/` - 自定义 UI 框架
- `crates/ipc/` - 进程间通信
- `crates/graphql/` - GraphQL 客户端和 Schema

### 关键架构模式

1. **Entity-Handle 系统**：视图通过 Handle 引用其他视图，而非直接拥有所有权（这避免了 Rust 所有权系统中的借用冲突，同时允许视图之间灵活地相互引用）
2. **模块化结构**：Workspace 包含多个工作区配置，每个配置拥有终端、笔记本等子组件
3. **跨平台**：为 macOS、Windows、Linux 提供原生实现，另有 WASM 目标平台
4. **AI 集成**：内置 AI 助手，具备上下文感知和代码库索引能力
5. **云同步**：对象可通过 Warp Drive 在多设备间同步

### 开发指南

**Workspace 结构**：
- 这是一个包含 60+ 成员 crate 的 Cargo workspace
- 主二进制文件位于 `app/`，UI 框架位于 `crates/warpui/`
- 平台特定代码通过条件编译（`cfg`）控制
- 集成测试位于 `crates/integration/`

**编码风格偏好**：
- 避免不必要的类型标注，尤其是闭包参数中的类型标注（Rust 的类型推断通常足以推断闭包参数类型）
- 避免使用过多的 Rust 路径限定符，优先使用 import 以保持简洁。按照惯例将 import 语句放在文件顶部。
  一个例外是在 cfg 守卫的代码分支中。在这种情况下，可以将 import 嵌入到相关作用域中，或者对于一次性使用直接使用绝对路径。
- 如果函数接受一个上下文参数（`AppContext`、`ViewContext` 或 `ModelContext`），应将其命名为 `ctx` 并放在参数列表最后。唯一的例外是当函数接受闭包参数时，闭包应放在最后。
- 始终完全删除未使用的参数，而不是给它们加上 `_` 前缀。同时更新函数签名和所有调用点。
- 在 `println!`、`eprintln!` 和 `format!` 等宏中优先使用内联格式参数（例如，使用 `eprintln!("{message}")` 而非 `eprintln!("{}", message)`），以满足 Clippy 的 `uninlined_format_args` lint 规则。
- 在进行无关更改时，不要删除已有注释。仅当注释所描述的逻辑发生变化时，才删除或修改该注释。

**Terminal Model 锁机制**：
- 在对终端模型（`TerminalModel`）调用 `model.lock()` 时必须格外小心。从不同调用点对同一模型获取多个锁可能导致死锁，进而造成 UI 冻结（在 macOS 上表现为旋转彩球）。
- 在添加新的 `model.lock()` 调用之前，必须验证当前调用栈中没有任何调用者已经持有该锁。
- 优先将已锁定的模型引用沿调用栈向下传递，而不是重新获取锁。
- 如果必须锁定模型，请将锁的作用范围保持尽可能短，并避免调用可能同样尝试获取锁的其他函数。

**测试**：
- 使用 `cargo nextest` 进行并行测试执行（比标准 `cargo test` 更快）
- 集成测试使用 `integration/` 中的自定义框架
- 提交前应通过 presubmit 脚本运行测试
- 单元测试应放在单独的文件中，使用命名约定 `${filename}_tests.rs` 或 `mod_test.rs`
- 测试文件应在其对应模块的末尾通过以下方式引入：
  ```rust
  #[cfg(test)]
  #[path = "filename_tests.rs"]  // 或 "mod_test.rs"
  mod tests;
  ```

**Pull Request 工作流**：
- **始终**在创建 PR 或向现有 PR 分支推送更新之前，运行 cargo fmt 和 cargo clippy（使用 ./script/presubmit 中指定的版本）
- 这些命令必须在创建或更新 Pull Request 之前完全通过
- 特别要确保 `cargo fmt` 和 `cargo clippy` 检查通过
- 如果检查失败，必须在继续 PR 流程之前修复所有问题
- 此规则适用于以下场景：
  - 创建新的 Pull Request
  - 向现有 PR 分支推送新提交
  - 任何将被审查的分支更新
- 创建 PR 时，请使用 `.github/pull_request_template.md` 中的 PR 模板
- 在适当时候添加 Changelog 条目，使用 PR 模板底部的格式。使用以下前缀（不包含 `{{}}` 括号）：
   - `CHANGELOG-NEW-FEATURE:` 用于新的、相对较大的功能（谨慎使用 - 这些可能会被市场/文档团队采用）
   - `CHANGELOG-IMPROVEMENT:` 用于现有功能的新增改进
   - `CHANGELOG-BUG-FIX:` 用于与已知 Bug 或回归问题相关的修复
   - `CHANGELOG-IMAGE:` 用于 GCP 托管的图片 URL
   - 如果不需要 Changelog 条目，请留空或删除相应行

**数据库**：
- 使用 Diesel ORM 配合 SQLite 数据库
- 数据库迁移文件位于 `crates/persistence/migrations/`
- Schema 定义在 `crates/persistence/src/schema.rs`

**GraphQL**：
- Schema 和客户端代码从 `crates/warp_graphql_schema/api/schema.graphql` 生成
- 为前端集成生成 TypeScript 类型

### Feature Flags（功能标志）

Warp 使用编译时 Feature Flags，并配有一个轻量的运行时桥接层。

如何添加 Feature Flag：
- 在 `warp_core/src/features.rs` 的 `FeatureFlag` 枚举中添加新变体
- （可选）通过将其列入 `DOGFOOD_FLAGS` 来为 dogfood 构建默认启用该标志
- 使用 `FeatureFlag::YourFlag.is_enabled()` 来门控代码路径
- 如需面向 preview 或 release 推出，分别添加到 `PREVIEW_FLAGS` 或 `RELEASE_FLAGS`（视情况而定）

最佳实践：
- **优先使用运行时检查而非 cfg 指令**：优先使用 `FeatureFlag::YourFlag.is_enabled()` 而非 `#[cfg(...)]` 编译时指令，这样标志无需重新编译即可切换，且后续清理更加容易。仅当代码在禁用该功能时无法编译（例如平台特定代码或禁用功能时不存在的依赖）时才使用 `#[cfg(...)]`。
- 保持标志的高层次和产品导向，而非针对每个调用点单独设置
- 在功能上线稳定后，移除标志和已失效的代码分支
- 对于暴露新功能的 UI 部分，在相同的标志后面隐藏 UI

示例：
```rust
#[derive(Sequence)]
pub enum FeatureFlag {
    YourNewFeature,
}

// Dogfood 构建中默认启用
pub const DOGFOOD_FLAGS: &[FeatureFlag] = &[
    FeatureFlag::YourNewFeature,
];

// 在代码中使用
if FeatureFlag::YourNewFeature.is_enabled() {
    // 受标志门控的行为
}
```

### 穷举匹配（Exhaustive Matching）

在添加或编辑 match 语句时，尽可能避免使用通配符 `_`。穷举匹配有助于确保所有变体都被处理，尤其是在未来向枚举添加新变体时，编译器会提醒你处理新增的变体，而不是被通配符静默忽略。
