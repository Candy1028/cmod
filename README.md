# cmod

`cmod` 是一个用 Rust 编写的交互式 Go 软件包检索与安装工具。它可以帮助开发者在终端中快速搜索 Go 包，并通过交互界面选择并安装

![](./image/gust.png)

## 特性

- **交互式搜索**：输入关键词即可在 pkg.go.dev 上搜索相关的 Go 软件包。
- **批量安装**：支持通过交互式多选列表一次性安装多个软件包。
- **状态感知**：自动识别当前项目中已安装的包及其版本。
- **便捷管理**：提供查看已安装软件包的功能。
- **纯净体验**：简洁的终端界面，无冗余输出。

## 安装

目前支持通过 `cargo` 进行本地编译安装：

```bash
git clone https://github.com/Candy1028/cmod.git
cd cmod
cargo install --path .
```

*确保你的系统中已安装 Rust 环境。*

## 使用方法

### 搜索并安装包

直接运行 `cmod` 并带上搜索关键词：

```bash
cmod <关键词>
```

例如搜索 `gin`：
```bash
cmod gin
```

### 限制搜索结果数量

使用 `-l` 或 `--limit` 参数限制显示的搜索结果数量（默认为 25）：

```bash
cmod gin --limit 10
```

### 查看已安装的包

使用 `-o` 或 `--old` 参数查看当前项目中通过 `go.mod` 管理的已安装软件包：

```bash
cmod --old
```

## 交互快捷键

在选择列表界面：
- `Space`: 选中/取消选中
- `Enter`: 确认并开始安装
- `Up/Down`: 移动光标
- `Ctrl+C`: 取消操作

## 依赖说明

- **Rust**: 系统需安装 `cargo` 命令行工具
- **网络**: 需要能够访问 `pkg.go.dev`

## 构建

如果你想自行构建发布版本：

```bash
cargo build --release
```

编译产物位于 `target/release/cmod`。

## 开源协议

本项目采用 [MIT](LICENSE) 协议。
