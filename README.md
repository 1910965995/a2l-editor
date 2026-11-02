# A2L Editor

ASAM MCD-2 MC（A2L）文件编辑与生成工具的 Tauri 2 重写版，功能与原 WPF 版对齐。

## 技术栈

- 前端：React 19 + TypeScript + Vite 8 + Ant Design 6
- 后端：Rust + Tauri 2（ELF32 / DWARF 解析、A2L 生成、XML 持久化、校验）
- 打包：NSIS 用户级安装包（`installMode: currentUser`）

## 功能

- 加载 TI CCS `.out` / ELF32 + DWARF，生成可展开的符号树（数组、结构体、联合体、枚举、typedef）
- 从符号添加测量量 / 标定量，支持数据类型映射、地址模式（RAW / BYTE / WORD）换算
- 测量量、标定量、内存段、DAQ_LIST / ODT、EVENT、XCP、时间戳配置
- 工程校验、A2L 预览（ASAP2 1.71）与导出
- 重新加载 ELF 时自动检测并更新变量地址
- 工程文件沿用原工具的 XML 结构，新旧工具可互相打开

## 开发

```bash
npm install
npm run tauri dev
```

仅调试前端时可直接运行 `npm run dev`，浏览器中会使用内置 mock 数据（不涉及文件系统与 ELF）。

## 构建安装包

```bash
npm run tauri build -- --bundles nsis
```

产物：

```text
src-tauri/target/release/bundle/nsis/A2L Editor_1.0.0_x64-setup.exe
```

安装包按当前用户安装，无需管理员权限。

## 测试

```bash
cd src-tauri
cargo test
```

覆盖类型映射、地址换算、A2L 文本输出、校验、XML round-trip，以及合成 ELF/DWARF 的导入路径。
