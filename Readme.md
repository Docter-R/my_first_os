此文件是操作系统的初始化配置文件，其中包含了去除标准库、增加#[panic_handler]功能、定义入口地址 _start，以及linker.ld链接文件。

```cmd
os
├── Cargo.lock
├── Cargo.toml  
├── record
│   └── 项目反汇编结果.md
├── src
    ├── linker.ld
    └── main.rs
```

