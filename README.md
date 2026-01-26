# c2rust-test

c2rust 工作流的 C 项目测试执行工具。

## 概述

`c2rust-test` 是一个命令行工具，用于执行 C 构建项目的测试命令，并使用 `c2rust-config` 自动保存配置。该工具是 c2rust 工作流的一部分，用于管理 C 到 Rust 的转换。

## 安装

### 从源代码安装

```bash
cargo install --path .
```

或本地构建：

```bash
cargo build --release
# 二进制文件将位于 target/release/c2rust-test
```

## 前置条件

该工具需要安装 `c2rust-config`。请从以下地址安装：
https://github.com/LuuuXXX/c2rust-config

### 环境变量

- `C2RUST_CONFIG`：可选。c2rust-config 二进制文件的路径。如果未设置，工具将在您的 PATH 中查找 `c2rust-config`。

## 使用方法

### 基本命令

```bash
c2rust-test test --dir <directory> -- <test-command> [args...]
```

`test` 子命令将：
1. 在指定目录中执行指定的测试命令
2. 将测试配置保存到 c2rust-config 以供后续使用

### 示例

#### 运行 Make 测试

```bash
c2rust-test test --dir /path/to/project -- make test
```

#### 运行自定义测试脚本

```bash
c2rust-test test --dir . -- ./run_tests.sh
```

#### 使用 CMake 运行测试

```bash
c2rust-test test --dir build -- ctest --output-on-failure
```

#### 使用功能标志运行测试

您可以指定一个功能名称来组织不同的测试配置：

```bash
c2rust-test test --feature debug --dir /path/to/project -- make test
```

#### 使用自定义 c2rust-config 路径

如果 `c2rust-config` 不在您的 PATH 中，或者您想使用特定版本：

```bash
export C2RUST_CONFIG=/path/to/custom/c2rust-config
c2rust-test test --dir /path/to/project -- make test
```

### 命令行选项

- `--dir <directory>`：执行测试命令的目录（必需）
- `--feature <name>`：配置的可选功能名称（默认："default"）
- `--`：c2rust-test 选项与测试命令之间的分隔符
- `<command> [args...]`：要执行的测试命令及其参数

### 帮助

获取一般帮助：

```bash
c2rust-test --help
```

获取 test 子命令的帮助：

```bash
c2rust-test test --help
```

## 工作原理

1. **验证**：检查 `c2rust-config` 是否已安装
2. **执行**：在指定目录中运行指定的测试命令
3. **配置**：保存两个配置值：
   - `test.dir`：执行测试的目录
   - `test`：完整的测试命令字符串

## 配置存储

该工具使用 `c2rust-config` 来存储测试配置。这些配置可以稍后由其他 c2rust 工具检索。

存储的配置示例：
```
test.dir = "/path/to/project"
test = "make test"
```

使用功能：
```
test.dir = "/path/to/project" （用于功能 "debug"）
test = "make test" （用于功能 "debug"）
```

## 错误处理

工具将在以下情况下退出并报错：
- 在 PATH 中找不到 `c2rust-config`
- 测试命令执行失败
- 无法保存配置

## 开发

### 构建

```bash
cargo build
```

### 运行测试

```bash
cargo test
```

注意：如果未安装 `c2rust-config`，某些集成测试可能会失败。

### 仅运行单元测试

```bash
cargo test --lib
```

## 许可证

该项目是 c2rust 生态系统的一部分。

## 相关项目

- [c2rust-config](https://github.com/LuuuXXX/c2rust-config) - 配置管理工具
- [c2rust-clean](https://github.com/LuuuXXX/c2rust-clean) - 构建产物清理工具

## 贡献

欢迎贡献！请随时提交问题或拉取请求。