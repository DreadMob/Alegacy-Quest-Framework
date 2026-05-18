<script lang="ts">
  import "./app.css";
  import { HardDrive, FolderOpen, Globe } from "lucide-svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import ItemEditor from "./components/ItemEditor.svelte";
  import BossEditor from "./components/BossEditor.svelte";
  import QuestEditor from "./components/QuestEditor.svelte";
  import BossHuntEditor from "./components/BossHuntEditor.svelte";
  import QualityEditor from "./components/QualityEditor.svelte";
  import NpcEditor from "./components/NpcEditor.svelte";
  import HollowTrialsEditor from "./components/HollowTrialsEditor.svelte";
  import ReputationEditor from "./components/ReputationEditor.svelte";
  import LocalizationEditor from "./components/LocalizationEditor.svelte";
  import Welcome from "./components/Welcome.svelte";
  import { openFolderDialog } from "./lib/fileService";
  import { getLang, setLang, t, type Lang } from "./lib/i18n";

  type View = "welcome" | "items" | "bosses" | "quests" | "bosshunt" | "quality" | "npcs" | "trials" | "reputation" | "localization";

  let currentView = $state<View>("welcome");
  let questsRoot = $state<string>(localStorage.getItem("alegacy-quests-root") || "");
  let questPacks = $state<string[]>([]);
  let activePack = $state<string>(localStorage.getItem("alegacy-active-pack") || "");
  let modPath = $derived(questsRoot && activePack ? `${questsRoot}\\${activePack}` : "");
  let folderError = $state("");
  let lang = $state<Lang>(getLang());

  function toggleLang() {
    lang = lang === "ru" ? "en" : "ru";
    setLang(lang);
  }

  function onNavigate(view: View) {
    currentView = view;
  }

  async function selectFolder() {
    try {
      const selected = await openFolderDialog();
      if (!selected) return;
      const result = await window.electronAPI.readDir(selected);
      const packs = result.entries.filter((e) => e.isDirectory).map((e) => e.name);
      if (packs.length === 0) {
        folderError = lang === "ru" ? "Папка пуста." : "Folder is empty.";
        return;
      }
      let validPacks: string[] = [];
      for (const pack of packs) {
        const packPath = await window.electronAPI.joinPath(selected, pack);
        const packEntries = await window.electronAPI.readDir(packPath);
        if (packEntries.entries.some((e) => e.name === "assets")) {
          validPacks.push(pack);
        }
      }
      if (validPacks.length === 0) {
        folderError = lang === "ru" ? "Нет подпапок с assets/." : "No subfolders with assets/.";
        return;
      }
      folderError = "";
      questsRoot = selected;
      questPacks = validPacks;
      localStorage.setItem("alegacy-quests-root", selected);
      if (!activePack || !validPacks.includes(activePack)) {
        activePack = validPacks[0];
        localStorage.setItem("alegacy-active-pack", activePack);
      }
    } catch (e: any) {
      folderError = `Error: ${e?.message || e}`;
    }
  }

  function selectPack(pack: string) {
    activePack = pack;
    localStorage.setItem("alegacy-active-pack", pack);
  }

  async function initPacks() {
    if (!questsRoot) {
      // Auto-detect quests folder from app location
      try {
        const defaultRoot = await window.electronAPI.getDefaultQuestsRoot();
        if (defaultRoot) {
          questsRoot = defaultRoot;
          localStorage.setItem("alegacy-quests-root", defaultRoot);
        }
      } catch {}
    }
    if (!questsRoot) return;
    try {
      const result = await window.electronAPI.readDir(questsRoot);
      const packs: string[] = [];
      for (const e of result.entries) {
        if (!e.isDirectory) continue;
        const packPath = await window.electronAPI.joinPath(questsRoot, e.name);
        const packEntries = await window.electronAPI.readDir(packPath);
        if (packEntries.entries.some((pe) => pe.name === "assets")) {
          packs.push(e.name);
        }
      }
      questPacks = packs;
      if (!activePack || !packs.includes(activePack)) {
        activePack = packs[0] || "";
      }
    } catch {
      questPacks = [];
    }
  }

  $effect(() => { initPacks(); });
</script>

<Sidebar {currentView} {modPath} {questPacks} {activePack} {lang} {onNavigate} onSelectFolder={selectFolder} onSelectPack={selectPack} />

<main class="flex-1 flex flex-col min-w-0 overflow-hidden bg-[#0a0a0b]">
  <header class="h-12 flex items-center justify-between px-6 border-b border-zinc-800 bg-zinc-950 shrink-0">
    <div class="flex items-center gap-2 text-sm min-w-0">
      <HardDrive size={14} class="text-zinc-500 shrink-0" />
      {#if modPath}
        <span class="text-zinc-500 font-mono text-xs truncate">{modPath}</span>
      {:else}
        <span class="text-zinc-600 text-xs italic">{t("app.noFolder", lang)}</span>
      {/if}
    </div>
    <div class="flex items-center gap-3 shrink-0">
      <button onclick={toggleLang} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-400 hover:text-zinc-200 px-2.5 py-1.5 rounded-md text-xs font-medium transition-colors" title="Switch language">
        <Globe size={13} />
        {lang.toUpperCase()}
      </button>
      <button onclick={selectFolder} class="flex items-center gap-2 bg-zinc-100 hover:bg-white text-zinc-900 px-3 py-1.5 rounded-md text-xs font-medium transition-colors">
        <FolderOpen size={14} />
        {modPath ? t("app.changeFolder", lang) : t("app.selectFolder", lang)}
      </button>
    </div>
  </header>

  {#if folderError}
    <div class="px-6 py-2 bg-red-500/10 border-b border-red-500/30 text-red-400 text-sm">{folderError}</div>
  {/if}

  <div class="flex-1 overflow-hidden">
    {#if currentView === "welcome"}
      <Welcome {modPath} {lang} onSelectFolder={selectFolder} />
    {:else if currentView === "items"}
      <ItemEditor {modPath} {lang} />
    {:else if currentView === "bosses"}
      <BossEditor {modPath} {lang} />
    {:else if currentView === "quests"}
      <QuestEditor {modPath} {lang} />
    {:else if currentView === "bosshunt"}
      <BossHuntEditor {modPath} {lang} />
    {:else if currentView === "quality"}
      <QualityEditor {modPath} {lang} />
    {:else if currentView === "npcs"}
      <NpcEditor {modPath} {lang} />
    {:else if currentView === "trials"}
      <HollowTrialsEditor {modPath} {lang} />
    {:else if currentView === "reputation"}
      <ReputationEditor {modPath} {lang} />
    {:else if currentView === "localization"}
      <LocalizationEditor {modPath} {lang} />
    {/if}
  </div>
</main>
