# RM-referee-protocol

**_注意_**：本项目处于早期开发阶段，未经实机验证，**不建议**在生产环境中使用。

基于 [RoboMaster 裁判系统串口协议附录 V1.9.0（2025-07-03）](https://terra-1-g.djicdn.com/b2a076471c6c4b72b574a977334d3e05/RM2025/RoboMaster%20%E8%A3%81%E5%88%A4%E7%B3%BB%E7%BB%9F%E4%B8%B2%E5%8F%A3%E5%8D%8F%E8%AE%AE%E9%99%84%E5%BD%95%20V1.9.0%EF%BC%8820250703%EF%BC%89.pdf)

Rust 实现的裁判系统协议数据结构与帧封装库；支持 `#![no_std]` + `alloc` 的嵌入式环境。使用 `deku` 精确定义位域，内置 CRC8/CRC16 校验。

## 快速开始

- 环境：[Rust](https://rust-lang.org/tools/install/)
- 构建：`cargo build`
- 运行[示例](./src/main.rs)：`cargo run`
- 测试：`cargo test`
