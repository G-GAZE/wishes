<div align="center">
  <h1> Wishes · 众愿 </h1>
  
  <img src="../src/assets/logos/wishes.svg" alt="Wishes Logo" width="128">

  <p>
    <img src="https://img.shields.io/badge/framework-Tauri_2-blue?logo=tauri" />
    <img src="https://img.shields.io/badge/frontend-Vue_3-4FC08D?logo=vuedotjs" />
    <img src="https://img.shields.io/badge/NaiveUI-18A058?logo=naiveui&logoColor=white" />
    <img src="https://img.shields.io/badge/storage-SQLite-003B57?logo=sqlite" />
    <img src="https://img.shields.io/badge/LICENSE-GPL_v3.0_or_later-green" />
  </p>

  <p>
    <a href="../README.md">简体中文</a> |
    <span>English</span> |
    <a href="README.ja.md">日本語</a>
  </p>
</div>

> [!WARNING]
>
> This document was translated using AI tools and may be inaccurate.
>
> For the most accurate information, please refer to the **main** README (*Simplified Chinese* version).

## Introduction

> [!IMPORTANT]
>
> The current `Wishes` is still in testing phase, and some features are incomplete.

`Wishes` is a universal, highly customizable gacha simulator.
It manages all cards, decks, banners, and wish logic through a unified *tag system*, and leverages a *combinatorial logic engine* to let you freely simulate the gacha experience of **almost any** game.

> In fact, it goes beyond games: if you wish, `Wishes` can simulate **almost any** random system.

All data is stored **locally**; the logic state of each banner (such as pity counters) is persisted to `SQLite`, and each banner is independent.

## Feature List

- [x] Tag system
- [x] Banner loading
- [x] Complete wish flow
- [ ] Data management
  - [x] Card management
  - [x] Deck management
  - [ ] Wish logic management
  - [ ] Banner management
- [ ] Deck declaration rules
  - [x] Membership rules
  - [ ] Event group rules (*incomplete*)
- [ ] Built-in wish logic (*hardcoded*)
  - [ ] *Genshin Impact*
    - [x] *Character · UP* (with *Capturing Radiance*)
    - [ ] *Weapon · UP*
    - [ ] *Standard*
    - [ ] *Beginner*
  - [ ] *Honkai: Star Rail*
    - [x] *Character · UP*
    - [ ] *Light Cone · UP*
    - [ ] *Standard*
    - [ ] *Beginner*
  - [ ] *Zenless Zone Zero*
    - [ ] *Character · UP*
    - [ ] *W-Engine · UP*
    - [ ] *Bangboo*
    - [ ] *Standard*
    - [ ] *Beginner*
  - [ ] *Arknights*
  - [ ] *Wuthering Waves*
  - [ ] *Blue Archive*
    - [ ] *Character · UP*
    - [ ] *Character · FES*
- [ ] Custom wish logic
  - [ ] Combinatorial wish rules
  - [ ] Custom rule blueprint/script (*planned*)
- [ ] Local multi-user isolation
- [ ] Full English configuration
- [ ] Card artwork management
- [ ] Improved wish animation
- [ ] Internationalization
  - [x] Simplified Chinese
  - [ ] English
  - [ ] Japanese
- [ ] Initialization guide

### Planned

- [ ] Establish a remote data repository, automatically pull updates, and via setting `Tracked Games` only fetch required content

## Quick Start

### Download Installer

Regular users can quickly experience `Wishes` via pre-compiled installers.

Go to the [**Releases**](https://github.com/G-GAZE/wishes/releases) page and download the latest installer for your platform.

After downloading, install and launch it as usual. On first launch, the built-in default cards, decks, logics, and banner data will be copied to the user data directory.

### Build from Source

For users who wish to compile, modify the source code, or contribute to development.

#### Prerequisites

- [**Node.js**](https://nodejs.org/) ≥ 18
- [**Rust**](https://www.rust-lang.org/tools/install) (*stable*)
- [**Tauri 2 system dependencies**](https://tauri.app/start/prerequisites/) for your platform

#### Development Mode

```bash
# Install frontend dependencies
npm install

# Start the development server using npm
npm run tauri dev
# Or using cargo
# cargo run tauri
```

> [!NOTE]
>
> In development mode, the app prioritizes reading `data/` and `db/` in the project root. Restart to apply changes after modifying data files.

#### Build Release

```bash
# Using npm
npm run tauri build
# Or using cargo
# cargo tauri build
```

Build artifacts are located in src-tauri/target/release/bundle, generating platform-specific outputs.

## Built-in Support

`Wishes` currently supports the following games:

- [x] Genshin Impact
- [x] Honkai: Star Rail
- [ ] Zenless Zone Zero
- [ ] Arknights
- [ ] Wuthering Waves
- [ ] Blue Archive

> [!IMPORTANT]
>
> Game order is **not ranked**!
>
> This list is **not fixed**; more games **may be** added to the planned list in the future.

`Wishes` includes all cards for supported games by default and provides implementations of **all wish logic** for supported games. It is **synchronized** with game updates.

## More Documentation

- [**Wishes Core Concepts**](concepts.en.md): `Wishes`' flexibility comes from a set of small and universal abstractions. Understanding these concepts allows you to customize **any** banner.
- **`data/` Directory Structure** (TBD)
- [**简体中文 README**](../README.md)
- [**日本語 README**](README.ja.md)

## Image Copyright

### Wishes Logo

The copyright of the Wishes Logo and its variants (located in `src/assets/logos/`) belongs to the Wishes project and its author, licensed under the same `GPL-3.0-or-later` as the project. It may be freely used for this project and derivative projects, provided the original copyright notice is retained.

### Card Artwork and Game Assets

If the future `data/assets/` directory contains card artwork, icons, or other art assets from various games, their copyright belongs to **the respective game companies**. These assets are used solely for local simulation display in Wishes. **Without permission from the copyright holder, they may not be used for any other purpose**, including but not limited to commercial use, redistribution, or derivative works.

If the copyright holder has objections to the use of such assets, please contact the author (via email), and we will remove the content as soon as possible.

## License

`Wishes` is open-sourced under the **GNU General Public License v3.0 or later** (`GPL-3.0-or-later`).

> [!NOTE]
>
> `Wishes` is free software: you can redistribute it and/or modify it under the terms of the **GNU General Public License** as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
>
> `Wishes` is distributed in the hope that it will be useful, but **WITHOUT ANY WARRANTY**; without even the implied warranty of **MERCHANTABILITY** or **FITNESS FOR A PARTICULAR PURPOSE**. See the GNU General Public License for more details.
>
> The full license text is available in the [`LICENSE`](../LICENSE) file at the repository root.

Any derivative project based on `Wishes` **must** open-source its code and retain the original copyright and license notices.

<div align="center">
  <sub>Copyright (C) 2026 G-GAZE and contributors · <a href="../LICENSE">GPL-3.0-or-later</a></sub>
</div>
