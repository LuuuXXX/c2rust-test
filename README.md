# c2rust-test

c2rust 工作流的 C 项目测试执行工具。

## 概述

`c2rust-test` 是一个命令行工具，用于在当前目录中执行 C 构建项目的测试命令。该工具是 c2rust 工作流的一部分，用于管理 C 到 Rust 的转换。

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
c2rust-test test -- <test-command> [args...]
```

**注意**：使用 `--` 分隔符来区分 c2rust-test 的参数和测试命令的参数。

`test` 子命令将：
1. 在**当前目录**中执行指定的测试命令，实时显示输出
2. 返回命令的退出状态（成功时退出码为 0，失败时返回底层命令的退出码）

### 命令行参数

- `--`：参数分隔符，之后的所有参数都是测试命令及其参数；**当测试命令或其参数以 `-` 开头时，必须使用该分隔符**，其他情况下也推荐始终使用

### 示例

#### 运行 Make 测试

```bash
cd /path/to/project
c2rust-test test -- make test
```

#### 运行自定义测试脚本

```bash
c2rust-test test -- ./run_tests.sh
```

#### 使用 CMake 运行测试

```bash
cd build
c2rust-test test -- ctest --output-on-failure
```

#### 带环境变量的测试

```bash
cd /path/to/project
c2rust-test test -- env VERBOSE=1 make test
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

## Git 自动提交

工具会在执行测试命令并保存配置后，自动检查 `.c2rust` 目录下是否有任何修改。如果存在修改，会自动执行 git commit 来保存这些修改。

**工作方式**：
- Git 仓库位置：`<项目根目录>/.c2rust/.git`
- 该 git 仓库由前置工具初始化，工具只负责检测和提交修改
- 只在有实际修改时才执行 commit
- Commit 消息：`Auto-commit: c2rust-test changes`

**注意**：
- 此功能无需配置，会自动运行
- 如果 `.c2rust/.git` 不存在，此功能会静默跳过
- 提交操作在程序执行的最后阶段进行

## 工作原理

1. **获取当前目录**：自动使用命令执行时的当前工作目录
2. **项目根目录查找**: 
   - 从当前目录向上查找包含以下标识之一的目录：
     - `Cargo.toml` 文件
     - `.git` 目录
     - `.c2rust` 目录
   - 如果未找到任何标识，使用当前目录作为项目根目录
3. **执行**：在当前目录中运行指定的测试命令，实时显示输出
   - 显示执行的命令和目录
   - 实时流式传输 stdout 和 stderr
   - 显示退出代码
4. **返回状态**：如果测试命令失败，工具将以非零退出代码退出
5. **自动提交**: 如果 `.c2rust` 目录下有任何修改，自动执行 git commit 保存修改信息

## 错误处理

工具将在以下情况下退出并报错：
- 无法获取当前工作目录
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

## 项目结构

```
src/
├── main.rs           # CLI 入口点和参数解析
├── error.rs          # 错误类型定义
├── executor.rs       # 命令执行逻辑
├── config_helper.rs  # 配置管理
└── git_helper.rs     # Git 自动提交
```

## 许可证

该项目是 c2rust 生态系统的一部分。

## 相关项目

- [c2rust-clean](https://github.com/LuuuXXX/c2rust-clean) - 构建产物清理工具

## 贡献

欢迎贡献！请随时提交问题或拉取请求。