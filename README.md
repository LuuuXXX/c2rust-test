# c2rust-test

c2rust 工作流的 C 项目测试执行工具。

## 概述

`c2rust-test` 是一个命令行工具，用于在指定目录中执行 C 构建项目的测试命令。该工具是 c2rust 工作流的一部分，用于管理 C 到 Rust 的转换。

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

## 使用方法

### 基本命令

```bash
c2rust-test test --dir <directory> -- <test-command> [args...]
```

`test` 子命令将：
1. 在指定目录中执行指定的测试命令，实时显示输出
2. 返回命令的退出状态（成功时退出码为 0，失败时返回底层命令的退出码）

### 命令行选项

- `--dir <directory>`：执行测试命令的目录（必需）
- `--`：c2rust-test 选项与测试命令之间的分隔符
- `<command> [args...]`：要执行的测试命令及其参数（必需）

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

#### 带环境变量的测试

```bash
c2rust-test test --dir /path/to/project -- env VERBOSE=1 make test
```

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

1. **验证参数**：检查必需的 `--dir` 和命令参数是否已提供
2. **执行**：在指定目录中运行指定的测试命令，实时显示输出
   - 显示执行的命令和目录
   - 实时流式传输 stdout 和 stderr
   - 显示退出代码
3. **返回状态**：如果测试命令失败，工具将以非零退出代码退出

## 错误处理

工具将在以下情况下退出并报错：
- 未提供目录参数（`--dir`）
- 未提供测试命令
- 测试命令执行失败

## 输出示例

执行测试命令时，工具会显示详细的实时输出：

```
Executing command: make test
In directory: /path/to/project

[测试输出实时显示在这里...]

Exit code: 0

Test command executed successfully.
```

## 开发

### 构建

```bash
cargo build
```

### 运行测试

```bash
cargo test
```

### 仅运行单元测试

```bash
cargo test --lib
```

## 许可证

该项目是 c2rust 生态系统的一部分。

## 相关项目

- [c2rust-clean](https://github.com/LuuuXXX/c2rust-clean) - 构建产物清理工具

## 贡献

欢迎贡献！请随时提交问题或拉取请求。