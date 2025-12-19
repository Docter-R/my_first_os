PART2分支实现了批处理操作系统。

```bash
os
├── Cargo.lock           # Cargo 锁定文件，确保依赖库版本的固定
├── Cargo.toml           # Cargo 配置文件，定义项目及其依赖信息
├── Makefile             # GNU Make 的构建配置文件，用于构建项目
├── build.rs             # 一个用于构建脚本的 Rust 文件，可在编译时执行额外策略
└── src                  # 源代码目录
    ├── batch.rs         # 实现批处理逻辑的模块
    ├── boards           # 存放与板级支持相关的模块
    │   └── qemu.rs      # QEMU 相关的实现，可能包含 QEMU 模拟器的相关配置
    ├── console.rs       # 控制台输出及相关功能实现
    ├── entry.asm        # 汇编语言文件，定义程序的入口点
    ├── lang_items.rs     # 自定义语言项目所需的项，解决特定语言特性的支持
    ├── link_app.S       # 链接器的汇编文件，定义应用程序的链接细节
    ├── linker.ld        # 链接器配置文件，定义如何链接这个项目的二进制文件
    ├── logging.rs       # 日志记录功能的实现模块
    ├── main.rs          # 主程序入口文件，它是应用程序的主要逻辑
    ├── sbi.rs           # 硬件抽象层接口，实现与硬件的交互逻辑
    ├── sync              # 同步相关模块
    │   ├── mod.rs       # 同步模块的主文件，可能导出其他同步实现
    │   └── up.rs        # 实现上层同步功能的文件
    ├── syscall           # 系统调用模块
    │   ├── fs.rs        # 文件系统相关的系统调用实现
    │   ├── mod.rs       # 系统调用模块的主文件
    │   └── process.rs   # 进程管理相关的系统调用实现
    └── trap             # 中断和异常处理模块
        ├── context.rs    # 保存和恢复上下文的实现
        ├── mod.rs       # 中断模块的主文件
        └── trap.S       # 中断处理的汇编实现
```

```bash
user
├── Cargo.lock           # Cargo 锁定文件，确保依赖库版本的固定
├── Cargo.toml           # Cargo 配置文件，定义项目及其依赖信息
├── Makefile             # GNU Make 的构建配置文件，用于构建项目
├── build                # 存放构建产物的目录
│   ├── app              # 存放应用示例的目录
│   │   ├── ch2b_bad_address.rs  # 示例应用 - 处理坏地址的示例
│   │   ├── ch2b_bad_instructions.rs # 示例应用 - 处理坏指令的示例
│   │   ├── ch2b_bad_register.rs  # 示例应用 - 处理坏寄存器的示例
│   │   ├── ch2b_hello_world.rs    # 示例应用 - 简单的你好世界示例
│   │   ├── ch2b_power_3.rs        # 示例应用 - 计算 3 的幂的示例
│   │   ├── ch2b_power_5.rs        # 示例应用 - 计算 5 的幂的示例
│   │   └── ch2b_power_7.rs        # 示例应用 - 计算 7 的幂的示例
│   ├── asm                # 存放汇编文件的目录
│   ├── bin                # 存放二进制文件的目录
│   │   ├── ch2b_bad_address.bin  # 编译后生成的坏地址示例的二进制文件
│   │   ├── ch2b_bad_instructions.bin # 编译后生成的坏指令示例的二进制文件
│   │   ├── ch2b_bad_register.bin  # 编译后生成的坏寄存器示例的二进制文件
│   │   ├── ch2b_hello_world.bin    # 编译后生成的你好世界示例的二进制文件
│   │   ├── ch2b_power_3.bin        # 编译后生成的 3 的幂示例的二进制文件
│   │   ├── ch2b_power_5.bin        # 编译后生成的 5 的幂示例的二进制文件
│   │   └── ch2b_power_7.bin        # 编译后生成的 7 的幂示例的二进制文件
│   └── elf                # 存放 ELF 文件的目录
│       ├── ch2b_bad_address.elf  # 编译后生成的坏地址示例的 ELF 文件
│       ├── ch2b_bad_instructions.elf # 编译后生成的坏指令示例的 ELF 文件
│       ├── ch2b_bad_register.elf  # 编译后生成的坏寄存器示例的 ELF 文件
│       ├── ch2b_hello_world.elf    # 编译后生成的你好世界示例的 ELF 文件
│       ├── ch2b_power_3.elf        # 编译后生成的 3 的幂示例的 ELF 文件
│       ├── ch2b_power_5.elf        # 编译后生成的 5 的幂示例的 ELF 文件
│       └── ch2b_power_7.elf        # 编译后生成的 7 的幂示例的 ELF 文件
├── build.py              # 用于构建过程的 Python 脚本
├── rust-toolchain.toml   # Rust 工具链版本配置文件
└── src                   # 源代码目录
    ├── bin               # 主要的二进制文件源代码目录
    │   ├── ch2b_bad_address.rs  # 是示例应用的源代码文件
    │   ├── ch2b_bad_instructions.rs # 是示例应用的源代码文件
    │   ├── ch2b_bad_register.rs  # 是示例应用的源代码文件
    │   ├── ch2b_hello_world.rs    # 是示例应用的源代码文件
    │   ├── ch2b_power_3.rs        # 是示例应用的源代码文件
    │   ├── ch2b_power_5.rs        # 是示例应用的源代码文件
    │   └── ch2b_power_7.rs        # 是示例应用的源代码文件
    ├── console.rs       # 控制台输出功能的实现
    ├── lang_items.rs     # 自定义语言项目所需的项，解决特定语言特性的支持
    ├── lib.rs            # 项目的库 crate，定义模块和公共接口
    ├── linker.ld        # 链接器配置文件，定义如何链接这个项目的二进制文件
    └── syscall.rs       # 系统调用的实现模块
```