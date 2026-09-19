<div align="center">
  <h1> Wishes · 众愿 </h1>
  
  <img src="src/assets/logos/wishes.svg" alt="Wishes Logo" width="128">

  <p>
    <img src="https://img.shields.io/badge/framework-Tauri_2-blue?logo=tauri" />
    <img src="https://img.shields.io/badge/frontend-Vue_3-4FC08D?logo=vuedotjs" />
    <img src="https://img.shields.io/badge/NaiveUI-18A058?logo=naiveui&logoColor=white" />
    <img src="https://img.shields.io/badge/storage-SQLite-003B57?logo=sqlite" />
    <img src="https://img.shields.io/badge/LICENSE-GPL_v3.0_or_later-green" />
  </p>

  <p>
    <span>语言: 简体中文</span> |
    <a href="docs/README.en.md">English</a> |
    <a href="docs/README.ja.md">日本語</a>
  </p>
</div>

## 简介

> [!IMPORTANT]
>
> 当前的 `Wishes` 仍处于测试阶段，部分功能尚不完善。

`Wishes` 是一款通用的、可高度自定义的模拟抽卡工具。
通过 *标签系统* 统一管理所有卡片、卡组、卡池与抽卡逻辑，并利用 *组合式的逻辑引擎*，让你能够自由模拟 **几乎任何** 游戏的抽卡体验。

> 其实不止游戏：如果你愿意的话，`Wishes` 可以模拟 **几乎任何** 随机系统。

所有数据均存储于 **本地** ; 卡池的逻辑状态 (如保底计数) 会持久化至 `SQLite`, 每个卡池相互独立。

## 功能清单

- [x] 标签系统
- [x] 卡池加载
- [x] 完整的抽卡流程
- [ ] 数据管理
  - [x] 卡片管理
  - [x] 卡组管理
  - [ ] 抽卡逻辑管理
  - [ ] 卡池管理
- [ ] 卡组声明规则
  - [x] 成员规则
  - [ ] 活动组规则 (*不完善*)
- [ ] 内置抽卡逻辑 (*硬编码*)
  - [ ] *原神*
    - [x] *角色 · UP* (含 *捕获明光*)
    - [ ] *武器 · UP*
    - [ ] *常驻*
    - [ ] *新手*
  - [ ] *崩坏星穹铁道*
    - [x] *角色 · UP*
    - [ ] *光锥 · UP*
    - [ ] *常驻*
    - [ ] *新手*
  - [ ] *绝区零*
    - [ ] *角色 · UP*
    - [ ] *音擎 · UP*
    - [ ] *邦布*
    - [ ] *常驻*
    - [ ] *新手*
  - [ ] *明日方舟*
  - [ ] *鸣潮*
  - [ ] *蔚蓝档案*
    - [ ] *角色 · UP*
    - [ ] *角色 · FES*
- [ ] 自定义抽卡逻辑
  - [ ] 组合式抽卡规则
  - [ ] 自定义规则蓝图/脚本 (*预想*)
- [ ] 本地多用户隔离
- [ ] 配置全英文化
- [ ] 卡片立绘管理
- [ ] 更完善的抽卡动画
- [ ] 国际化
  - [x] 简体中文
  - [ ] 英语 (English)
  - [ ] 日语 (日本語)
- [ ] 初始化引导

### 预想

- [ ] 建立远程数据仓库，自动拉取更新数据，并通过设置 `追踪的游戏`，仅拉取需要的内容

## 快速开始

### 下载安装包

普通用户可直接通过预编译的安装包快速体验 `Wishes`。

前往 [**Releases**](https://github.com/G-GAZE/wishes/releases) 页面，下载对应平台的最新安装包。

下载后按常规方式安装并启动即可，首次启动时会把内置的默认卡片、卡组、逻辑与卡池数据复制到用户数据目录。

### 本地构建

面向希望自行编译、修改源码或参与开发的用户。

#### 环境要求

- [**Node.js**](https://nodejs.org/) ≥ 18
- [**Rust**](https://www.rust-lang.org/tools/install) (*stable*)
- 对应平台的 [**Tauri 2 系统依赖**](https://tauri.app/start/prerequisites/)

#### 开发模式

```bash
# 安装前端依赖
npm install

# 使用 npm 启动开发服务器
npm run tauri dev
# 或使用 cargo
# cargo run tauri
```

> [!NOTE]
>
> 开发模式下，应用会优先读取项目根目录中的 `data/` 与 `db/`，修改数据文件后重启即可生效。

### 构建发布版

```bash
# 使用 npm
npm run tauri build
# 或使用 cargo
# cargo tauri build
```

构建产物位于 `src-tauri/target/release/bundle`，按平台生成对应产物。

## 内置支持

`Wishes` 目前有以下受支持游戏清单:

- [x] 原神
- [x] 崩坏星穹铁道
- [ ] 绝区零
- [ ] 明日方舟
- [ ] 鸣潮
- [ ] 蔚蓝档案

> [!IMPORTANT]
>
> 游戏排名 **不分先后** !
>
> 此清单 **不固定**，未来可能有 **更多** 游戏加入到计划清单中。

`Wishes` 默认包含受支持游戏的 **所有卡片** 、提供受支持游戏的 **所有抽卡逻辑** 的实现。并随游戏更新而 **同步更新**。

## 更多文档

- [**`Wishes` 核心概念**](docs/concepts.md)：`Wishes` 的灵活性来自一些小巧而通用的抽象概念，理解这些概念后即可自定义 **任意** 卡池。
- [**`data/` 数据目录结构说明**]()
- [**English README** 说明文档](docs/README.en.md)
- [**日本語 README** 说明文档](docs/README.ja.md)

## 图片版权

### Wishes Logo

`Wishes` Logo 及其变体 (位于 `src/assets/logos/`) 的版权归 `Wishes` 项目及作者所有，采用与项目一致的 `GPL-3.0-or-later` 协议授权。在保留原始版权声明的前提下，可自由用于本项目的衍生项目。

### 卡片立绘与游戏素材

未来 `data/assets/` 目录中若包含来自各游戏的卡片立绘、图标或其他美术资源，其版权均归 **相应的游戏公司** 所有。这些素材仅用于 `Wishes` 的本地模拟展示，**未经版权方许可，不得用于任何其他用途**，包括但不限于商业用途、二次分发或再创作。

若版权方对相关素材的使用存在异议，请联系作者 (通过邮箱)，我们将在第一时间移除相关内容。

## 开源协议

`Wishes` 基于 **GNU General Public License v3.0 or later** (`GPL-3.0-or-later`) 协议开源。

> [!NOTE]
>
> `Wishes` 是自由软件: 你可以按照由自由软件基金会发布的 **GNU 通用公共许可证来** 再发布该软件或者修改该软件; 你可以使用该许可证的第 3 版, 或者 (作为可选项) 使用该许可证的任何更新版本。
>
> `Wishes` 的发布是希望它能发挥作用, 但是 **并无担保** ; 甚至也 **不担保** 其可销售性或适用于某个特殊的目的。请参看 GNU 通用公共许可证来了解详情。
>
> 完整的许可证文本可见仓库根目录的 [`LICENSE`](LICENSE) 文件。

任何基于 `Wishes` 的衍生项目 **必须** 开放其源代码，并保留原始的版权声明与许可证声明。

<div align="center">
  <sub>Copyright (C) 2026 G-GAZE and contributors · <a href="LICENSE">GPL-3.0-or-later</a></sub>
</div>
