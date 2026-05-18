# Alegacy Quest Framework

**A large-scale quest and boss combat framework for Vintage Story**

[![License](https://img.shields.io/badge/License-Custom-red.svg)](LICENSE)
[![Game Version](https://img.shields.io/badge/Vintage%20Story-1.22.1+-green.svg)](https://www.vintagestory.at/)
[![Version](https://img.shields.io/badge/Version-3.0.0-orange.svg)](resources/modinfo.json)
[![C#](https://img.shields.io/badge/C%23-.NET%2010-purple.svg)]()
[![Editor](https://img.shields.io/badge/Download%20Editor-Windows%20x64-blue?style=for-the-badge&logo=windows)](https://github.com/user-attachments/files/27977382/Alegacy-QF-Editor-0.3.0-x64-setup.zip)

---

## 🔥 About

**Alegacy Quest Framework** is a full-featured quest engine built for [Vintage Story](https://www.vintagestory.at/). Originally started as a fork of [VSQuest](https://github.com/G3rste/vsquest), the project has since been rewritten and expanded into a standalone framework powering the [alegacy.online](https://alegacy.online) server.

The mod now includes systems far beyond basic questing — boss hunts with complex AI abilities, hollow trials (solo boss challenges with tiered progression), NPC dialogues, item quality systems, promo codes, quizzes, reputation trees, and more.

> **Note:** This project originally began as a fork/inspired rewrite of [VSQuest](https://github.com/G3rste/vsquest), which was licensed under MIT. The current project has since been fully rewritten and is distributed as an independent project under a [custom license](LICENSE). This repository does not contain code or assets from the original project.

---

## ✨ Key Features

| System | Description |
|--------|-------------|
| 🗡️ **Quest Engine** | Multi-stage quests with branching objectives, actions, and rewards |
| 🐉 **Boss Hunt** | Rotating world boss system with anchors, arenas, and combat state machines |
| 👹 **40+ Boss Abilities** | Dashes, grabs, teleports, clones, soul chains, void zones, and more |
| ⚔️ **Hollow Trials** | Solo boss challenge system with tiers, weekly modifiers, and a reward shop |
| 💬 **Dialogue System** | Branching NPC conversations with conditions and triggers |
| 📦 **Action Items** | Custom items with cast abilities, charge drain, and quality tiers |
| 🏆 **Reputation** | Faction reputation with unlock trees and tiered rewards |
| 🎰 **Reroll System** | Boss reward rerolling with animated UI |
| 📜 **Journal** | In-game quest journal with tabs, entries, and discovery |
| 🎫 **Promo Codes** | Redeemable codes with configurable rewards |
| ❓ **Quiz System** | Interactive quizzes tied to quest progression |
| 🌍 **QuestLand** | Land-claim based notifications and area triggers |

---

## 🏗️ Architecture

```
src/
├── BossHunt/          — Boss rotation, arenas, combat state machine
├── HollowTrials/      — Solo trial system, challenges, shop
├── Entity/            — Boss behaviors (40+ abilities), NPC behaviors
├── Quests/            — Core quest engine, objectives, actions
├── Systems/           — Items, reputation, database, performance
├── Commands/          — Admin and player chat commands
├── Gui/               — Journal, dialogs, reroll, notifications
├── Harmony/           — Patches for vanilla game integration
├── Network/           — Client-server packet handling
└── Utils/             — Shared utilities and helpers

quests/                — Content packs (quest definitions, entities, configs)
├── albase/            — Main server content
├── ALStory/           — Story questline
└── debugging/         — Development testing content
```

---

## 📚 Documentation

Full docs are in the [`docs/`](docs/) folder:

<details>
<summary><strong>Core Systems</strong></summary>

- [Getting Started](docs/start.md) — Mod structure overview
- [Architecture](docs/architecture.md) — Technical architecture
- [Example Guide](docs/example.md) — Step-by-step quest creation
- [Quest Stages](docs/queststages.md) — Multi-phase quests

</details>

<details>
<summary><strong>Quest Content</strong></summary>

- [Actions](docs/actions.md) — All quest actions
- [Objectives](docs/objectives.md) — All quest objectives
- [Action Items](docs/actionitems.md) — Custom item system
- [Item Quality](docs/itemquality.md) — Quality and attributes
- [Dialogues](docs/dialogue.md) — NPC dialogue system
- [Journal](docs/journal.md) — Quest journal UI

</details>

<details>
<summary><strong>Boss Systems</strong></summary>

- [Boss Hunt](docs/bosshunt.md) — World boss rotation
- [Boss Behaviors](docs/bossbehaviors.md) — 40+ boss abilities
- [Hollow Trials](docs/hollowtrials.md) — Solo challenge system

</details>

<details>
<summary><strong>Utility Systems</strong></summary>

- [Spawner](docs/spawner.md) — Quest mob spawner
- [Reroll](docs/reroll.md) — Reward rerolling
- [QuestLand](docs/questland.md) — Land-claim notifications
- [Promo Codes](docs/promocodes.md) — Code redemption
- [Quiz](docs/quiz.md) — Interactive quizzes
- [Commands](docs/commands.md) — Chat commands

</details>

---

## �️ Alegacy Quest Framework Editor

The repository includes a **visual desktop application** (`config-editor/`) for creating and editing all framework configurations without touching JSON manually.

**Built with:** Rust + Tauri + Svelte 5 + Tailwind CSS

### Features:
- 📊 **Analytics Dashboard** — Balance charts, attribute distribution, boss HP ranges, quest category breakdown
- ⚔️ **Item Editor** — Attributes from dropdown, color picker for names, table/card view, folder grouping
- 🐉 **Boss Editor** — Abilities, stages, HP phase visualization
- 📜 **Quest Editor** — All objectives/actions from dropdown, stages support, gather/kill/action objectives, random rewards
- 🎯 **Boss Hunt** — Relocate intervals, respawn, rotation, activation range
- 💎 **Quality Tiers** — Color picker, bonus %, applicable items
- 👥 **NPCs** — Quest giver config + dialogue editor in one view
- 🔥 **Hollow Trials** — Tiers, challenges, difficulty visualization
- ⭐ **Reputation** — Ranks, rewards, progression bar
- 🌐 **Localization** — All lang files, search, jump between locales, auto-sized fields for long texts
- 🌍 **Bilingual UI** — Russian / English

### Running the editor:
```bash
cd config-editor
npm install
# Use dev-tauri.bat on Windows (sets up MSVC environment)
npx tauri dev
```

---

## �🚀 Building

```bash
dotnet build
```

Requires .NET 10 SDK and Vintage Story 1.22.1+ references.

---

## 👥 Authors

- **[DreadMob](https://github.com/DreadMob)** — Lead developer
- **[Flajakay](https://github.com/swiftkoi)** — Developer

---

## 📄 License

This project uses a **custom license** — see [LICENSE](LICENSE).

**TL;DR:**
- ✅ You **can** create quest packs (content mods) on top of this framework
- ✅ You **can** view and learn from the source code
- ❌ You **cannot** fork or redistribute the framework code
- 📩 Want to modify the framework? Contact us on [Discord](https://discord.gg/aK4GtFpnt)

Originally began as a fork of [VSQuest](https://github.com/G3rste/vsquest) by G3rste (MIT License). Fully rewritten — no original code remains.

---

## 💬 Community

[![Discord](https://img.shields.io/badge/Discord-Join%20Us-5865F2?logo=discord&logoColor=white)](https://discord.gg/aK4GtFpnt)

Join our Discord for support, quest pack development help, and permission requests.
