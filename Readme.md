# RISC-V操作系统内核基础架构

## 一、各文件功能简要概述

| 文件            | 功能           | 作用                                          |
| --------------- | -------------- | --------------------------------------------- |
| `entry.asm`     | 汇编启动代码   | 设置栈指针，调用Rust入口函数                  |
| `logging.rs`    | 日志系统       | 提供Trace/Debug/Info/Warn/Error级别的日志记录 |
| `console.rs`    | 控制台输出     | 实现`Write` trait，支持`println!`宏           |
| `main.rs`       | 操作系统主入口 | 清除BSS段，初始化日志，打印启动信息           |
| `lang_items.rs` | panic处理      | 处理Rust代码中的panic错误                     |
| `sbi.rs`        | SBI接口封装    | 提供SBI调用的Rust接口                         |
| `linker.ld`     | 链接脚本       | 定义内存布局和各段的起始/结束地址             |

## 二、程序执行流程

1. **系统启动**：RISC-V处理器从`_start`开始执行
2. **汇编启动**（`entry.asm`）：
   - 设置栈指针(`sp`)指向`boot_stack_top`
   - 调用`rust_main`作为Rust代码的入口点
3. **Rust初始化**（`main.rs`）：
   - `clear_bss()`：清除BSS段（未初始化的全局变量）
   - `logging::init()`：初始化日志系统
   - `println!("[kernel] Hello, world!")`：打印启动信息
4. **输出流程**：
   - `println!`调用`console.rs`中的`print`
   - `console.rs`通过`Stdout`实现的`Write` trait处理字符串
   - 调用`sbi::console_putchar`将字符输出
   - `sbi::console_putchar`通过`sbi_rt`库调用SBI接口
   - SBI接口由QEMU加载的`rustsbi.bin`处理，输出到QEMU控制台
