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

## 三、控制台文件和SBI文件深度解读

### 1. 控制台文件（console.rs）

```rust
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}
```

- **核心功能**：实现了`Write` trait，使`println!`宏能够工作
- **工作原理**：
  - 将字符串转换为字符序列
  - 对每个字符调用`sbi::console_putchar`
  - 不需要直接与硬件交互，通过SBI接口实现

### 2. SBI文件（sbi.rs）

```rust
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}
```

- **核心功能**：提供SBI调用的Rust接口

- **依赖**：需要`Cargo.toml`中添加`sbi-rt`依赖

  ```toml
  sbi-rt = { version = "0.0.2", features = ["legacy"] }
  ```

- **关键点**：使用`legacy`特性是因为串口输出属于SBI的遗留接口

### 3. SBI接口与rustsbi.bin的关系

- **SBI (Supervisor Binary Interface)**：RISC-V架构定义的标准接口规范

- **rustsbi.bin**：RustSBI在QEMU平台上的编译输出，是SBI的Rust实现

- **QEMU启动时使用**：通过`-bios`参数指定

  ```
  qemu-system-riscv64 -bios ./rustsbi-qemu.bin -device loader,file=kernel.bin,addr=0x80200000
  ```

- **工作流程**：

  ```
  内核代码 → console.rs → sbi.rs → sbi_rt → SBI调用 → rustsbi.bin → QEMU控制台
  ```

## 四、sbi.rs文件和rustsbi-qemu.bin的调用关系

这段代码通过 **SBI (Supervisor Binary Interface)** 机制，利用 RISC-V 架构特有的 **`ecall` 指令**，请求底层的固件（Firmware，如 OpenSBI 或 RustSBI）代为执行输出操作。

代码本身并**不直接操作**串口（UART）硬件，而是像“发订单”一样向更高权限层级发送请求。

以下是详细的步骤解析：

### 1. 核心机制：`ecall` 指令

代码的核心在于 `sbi_call` 函数中的 `unsafe` 代码块：

```rust
asm!(
    "li x16, 0",          // 这是一个辅助指令（在某些旧标准中可能用到，这里主要是占位）
    "ecall",              // 关键指令！Environment Call
    inlateout("x10") arg0 => ret, // 传入参数0 (a0)，也是返回值
    in("x11") arg1,       // 传入参数1 (a1)
    in("x12") arg2,       // 传入参数2 (a2)
    in("x17") which,      // 传入功能编号 (a7)
);
```

- 寄存器准备：
  - **`x17` (即 `a7`)**：存放**功能编号**（Extension ID）。在这段代码中，`SBI_CONSOLE_PUTCHAR` 的值为 `1`，代表“Legacy SBI Console Putchar”功能。
  - **`x10` (即 `a0`)**：存放**参数**。在 `console_putchar` 函数中，这里存放的是要打印的那个字符 `c`。
- `ecall` (Environment Call)：
  - 当 CPU 执行到这条指令时，会触发一个**异常（Trap）**。
  - CPU 的特权级会从当前的 **S-Mode (Supervisor Mode，内核所在模式)** 切换到更高的 **M-Mode (Machine Mode，固件（即`rustsbi-qemu.bin`）所在模式)**。
  - 控制权移交给 M-Mode 下运行的软件（通常是 OpenSBI 或 RustSBI）。

### 2. 流程图解

当你在内核中调用 `console_putchar('A')` 时，发生了以下接力跑：

1. **准备阶段 (S-Mode)**:
   - `console_putchar('A')` 调用 `sbi_call(1, 'A', 0, 0)`。
   - Rust 将 `1` 放入寄存器 `a7`，将 `'A'` 放入寄存器 `a0`。
   - 执行 `ecall`。
2. **切换阶段 (硬件行为)**:
   - CPU 暂停执行内核代码。
   - CPU 跳转到预设的 M-Mode 异常处理入口地址（mtvec）。
3. **执行阶段 (M-Mode / 固件)**:
   - **OpenSBI/RustSBI 捕获到这个异常。**
   - 它检查 `a7` 寄存器，发现是 `1`，知道内核想要打印字符。
   - 它读取 `a0` 寄存器拿到字符 `'A'`。
   - **固件（即`rustsbi-qemu.bin`）直接操作硬件**：固件向实际的物理地址（UART 串口控制器的内存映射地址）写入数据。
   - 字符通过串口线发送出去，最终显示在 QEMU 的控制台窗口上。
4. **返回阶段**:
   - 固件执行 `mret` 指令。
   - CPU 切回 S-Mode，回到 `ecall` 的下一条指令继续执行内核代码。

### 3. 为什么要这样做？

这是一种**抽象和保护机制**：

- **硬件抽象**：操作系统内核不需要知道具体的硬件细节（比如串口寄存器的物理地址是多少）。不同的板子硬件地址不同，但 SBI 调用的接口（寄存器 `a7` 放功能号）是统一标准的。
- **权限隔离**：在虚拟化场景下，S-Mode 只是客户机（Guest），不能让它随意摸硬件。所有的硬件访问都必须经过 M-Mode（类似 Hypervisor）审核和代理。

### 总结

这段代码实现控制台输出的本质是：**配置好寄存器（a7=1, a0=字符） -> 执行 `ecall` 陷入 M-Mode -> 由底层的 SBI 固件完成真正的硬件写入操作**。

> **关键点**：控制台输出不是直接与硬件交互，而是通过SBI接口，由rustsbi.bin（SBI实现）处理，这使得内核代码可以保持简洁和可移植。