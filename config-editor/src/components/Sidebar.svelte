<script lang="ts">
  import { Home, Sword, Skull, ScrollText, Swords, Package, FolderOpen, Target, Gem, Users, Flame, Star, Languages } from "lucide-svelte";
  import { t, type Lang } from "../lib/i18n";

  type View = "welcome" | "items" | "bosses" | "quests" | "bosshunt" | "quality" | "npcs" | "trials" | "reputation" | "localization";

  interface Props {
    currentView: View;
    modPath: string;
    questPacks: string[];
    activePack: string;
    lang: Lang;
    onNavigate: (view: View) => void;
    onSelectFolder: () => void;
    onSelectPack: (pack: string) => void;
  }

  let { currentView, modPath, questPacks, activePack, lang, onNavigate, onSelectFolder, onSelectPack }: Props = $props();
</script>

<aside class="w-60 bg-zinc-950 border-r border-zinc-800 flex flex-col z-10">
  <!-- Logo -->
  <div class="h-12 flex items-center px-4 border-b border-zinc-800">
    <div class="flex items-center gap-2.5">
      <img src="/logo.png" alt="AQF" class="w-7 h-7 rounded-md" />
      <span class="font-semibold text-sm text-zinc-100 tracking-tight">Alegacy Quest Framework</span>
      <span class="text-[10px] font-mono bg-zinc-800 text-zinc-500 px-1.5 py-0.5 rounded ml-1">0.3</span>
    </div>
  </div>

  <!-- Quest Packs -->
  <div class="p-3 border-b border-zinc-800">
    <div class="text-[10px] font-semibold text-zinc-500 uppercase tracking-wider mb-2 px-1">{t("packs.label", lang)}</div>
    {#if questPacks.length > 0}
      <div class="space-y-0.5 max-h-40 overflow-y-auto">
        {#each questPacks as pack}
          <button
            onclick={() => onSelectPack(pack)}
            class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded text-xs transition-colors {activePack === pack ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'}"
          >
            <Package size={13} class={activePack === pack ? "text-blue-400" : "text-zinc-600"} />
            <span class="truncate font-medium">{pack}</span>
          </button>
        {/each}
      </div>
    {:else}
      <button
        onclick={onSelectFolder}
        class="w-full flex items-center gap-2 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 rounded-md px-3 py-2 text-xs text-zinc-400 transition-colors"
      >
        <FolderOpen size={13} class="text-zinc-500" />
        <span>{t("packs.selectMod", lang)}</span>
      </button>
    {/if}
  </div>

  <!-- Navigation -->
  <nav class="flex-1 px-3 py-3 space-y-0.5">
    <div class="text-[10px] font-semibold text-zinc-500 uppercase tracking-wider mb-2 px-2">{t("nav.menu", lang)}</div>

    <button
      onclick={() => onNavigate("welcome")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'welcome' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Home size={15} class={currentView === "welcome" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.home", lang)}
    </button>

    <button
      onclick={() => onNavigate("items")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'items' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Sword size={15} class={currentView === "items" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.items", lang)}
    </button>

    <button
      onclick={() => onNavigate("bosses")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'bosses' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Skull size={15} class={currentView === "bosses" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.bosses", lang)}
    </button>

    <button
      onclick={() => onNavigate("quests")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'quests' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <ScrollText size={15} class={currentView === "quests" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.quests", lang)}
    </button>

    <button
      onclick={() => onNavigate("bosshunt")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'bosshunt' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Target size={15} class={currentView === "bosshunt" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.bosshunt", lang)}
    </button>

    <button
      onclick={() => onNavigate("quality")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'quality' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Gem size={15} class={currentView === "quality" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.quality", lang)}
    </button>

    <button
      onclick={() => onNavigate("npcs")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'npcs' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Users size={15} class={currentView === "npcs" ? "text-blue-400" : "text-zinc-500"} />
      NPCs
    </button>

    <button
      onclick={() => onNavigate("trials")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'trials' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Flame size={15} class={currentView === "trials" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.trials", lang)}
    </button>

    <button
      onclick={() => onNavigate("reputation")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'reputation' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Star size={15} class={currentView === "reputation" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.reputation", lang)}
    </button>

    <button
      onclick={() => onNavigate("localization")}
      class="w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors {currentView === 'localization' ? 'bg-blue-500/10 text-blue-400' : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-zinc-200'}"
    >
      <Languages size={15} class={currentView === "localization" ? "text-blue-400" : "text-zinc-500"} />
      {t("nav.localization", lang)}
    </button>
  </nav>
</aside>
