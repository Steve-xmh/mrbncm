> **由于官方已经更新到了 3.0.0，且原生支持目前 MRBNCM 给 AMLL 提供的音频可视化功能，故本项目不再更新，请大家去用本家吧～**

# My Rust Better Neko Cat Music (MRBNCM)

使用纯 Rust 编写的更好的猫猫音乐扩展喵（雾）

## 编译

准备好 NodeJS 和 Rust 工具套件。

构建 JS 框架~~构建猫粮~~：
```bash
yarn
yarn build:dev
yarn build
```

构建本体~~构建猫猫~~：

```bash
cargo build -p betterncm-loader
cargo run -p betterncm-loader --example debug # 直接把猫猫放在大房子里跑哦
```

发行构建~~让猫猫变得更小~~：

```bash
cargo +nightly build --release -Z build-std=core,alloc,std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc -p betterncm-loader
cargo +nightly run --release -Z build-std=core,alloc,std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc -p betterncm-loader --example debug # 直接把猫猫放在大房子里跑哦
```
