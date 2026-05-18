<script lang="ts">
  import { RefreshCw, Save, Plus, X, AlertCircle, Star, Trash2, TrendingUp } from "lucide-svelte";
  import { saveJsonFile } from "../lib/fileService";
  import { t, type Lang } from "../lib/i18n";

  interface Props { modPath: string; lang: Lang; }
  let { modPath, lang }: Props = $props();

  interface Rank { min: number; rankLangKey: string; rewardAction?: string; }
  interface NpcReputation { titleLangKey: string; ranks: Rank[]; }
  interface ReputationConfig { npcs: Record<string, NpcReputation>; }

  let config = $state<ReputationConfig | null>(null);
  let configPath = $state("");
  let loading = $state(false);
  let error = $state("");
  let selectedNpc = $state<string>("");
  let unsaved = $state(false);
  let saveStatus = $state("");

  let npcIds = $derived.by(() => config ? Object.keys(config.npcs) : []);
  let currentNpc = $derived.by(() => config && selectedNpc ? config.npcs[selectedNpc] : null);

  async function load() {
    if (!modPath) return;
    loading = true; error = "";
    try {
      const api = window.electronAPI;
      const assetsPath = await api.joinPath(modPath, "assets");
      const dirs = await api.readDir(assetsPath);
      for (const d of dirs.entries.filter((e: any) => e.isDirectory)) {
        const rPath = await api.joinPath(assetsPath, d.name, "config", "reputation.json");
        if (await api.exists(rPath)) {
          const content = await api.readTextFile(rPath);
          config = JSON.parse(content.replace(/^\uFEFF/, ""));
          configPath = rPath;
          const keys = Object.keys(config!.npcs);
          if (keys.length > 0) selectedNpc = keys[0];
          loading = false; return;
        }
      }
      error = lang === "ru" ? "reputation.json не найден" : "reputation.json not found";
    } catch (e: any) { error = e?.message || e; }
    loading = false;
  }

  function markChanged() { unsaved = true; config = { ...config! }; }

  function addRank() {
    if (!currentNpc) return;
    const lastMin = currentNpc.ranks.length > 0 ? currentNpc.ranks[currentNpc.ranks.length - 1].min : 0;
    currentNpc.ranks.push({ min: lastMin + 500, rankLangKey: "modid:rank-new" });
    markChanged();
  }

  function removeRank(idx: number) { if (!currentNpc) return; currentNpc.ranks.splice(idx, 1); markChanged(); }

  function updateRank(idx: number, field: string, value: string) {
    if (!currentNpc) return;
    if (field === "min") { const n = parseInt(value); if (!isNaN(n)) currentNpc.ranks[idx].min = n; }
    else if (field === "rewardAction") { if (value) currentNpc.ranks[idx].rewardAction = value; else delete currentNpc.ranks[idx].rewardAction; }
    else (currentNpc.ranks[idx] as any)[field] = value;
    markChanged();
  }

  function addNpc() {
    if (!config) return;
    const id = "modid:new-npc";
    config.npcs[id] = { titleLangKey: "modid:new-npc-title", ranks: [{ min: 0, rankLangKey: "modid:rank-0" }] };
    selectedNpc = id; markChanged();
  }

  async function save() {
    if (!config || !configPath) return; saveStatus = "...";
    try { await saveJsonFile(configPath, config); unsaved = false; saveStatus = t("bosses.saved", lang); setTimeout(() => saveStatus = "", 3000); }
    catch (e: any) { saveStatus = `Error: ${e?.message || e}`; }
  }

  $effect(() => { if (modPath) load(); });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">{t("nav.reputation", lang)}</h2>
    <span class="text-xs text-zinc-500">{npcIds.length} NPCs</span>
    <div class="ml-auto flex items-center gap-2">
      <button onclick={addNpc} class="flex items-center gap-1.5 bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Plus size={13} /></button>
      <button onclick={load} disabled={loading} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-50"><RefreshCw size={13} class={loading ? "animate-spin" : ""} /></button>
      {#if unsaved}<button onclick={save} class="flex items-center gap-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Save size={13} /></button>{/if}
    </div>
  </div>
  {#if saveStatus}<div class="px-6 py-1.5 text-xs text-emerald-400 bg-emerald-500/5 border-b border-emerald-500/20">{saveStatus}</div>{/if}
  {#if error}<div class="mx-6 mt-3 flex items-start gap-2 bg-red-500/10 border border-red-500/30 rounded-md p-3 text-sm text-red-400"><AlertCircle size={16} class="shrink-0 mt-0.5" /><span>{error}</span></div>{/if}

  {#if config}
    <div class="flex-1 flex overflow-hidden">
      <div class="w-52 border-r border-zinc-800 bg-zinc-950 flex flex-col overflow-y-auto p-2 space-y-0.5">
        {#each npcIds as npcId}
          <button onclick={() => selectedNpc = npcId} class="w-full flex items-center gap-2 px-3 py-2 rounded-md text-xs transition-colors {selectedNpc === npcId ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'}">
            <Star size={13} class="shrink-0 text-amber-400" />
            <span class="truncate font-mono text-[11px]">{npcId.split(":").pop()}</span>
          </button>
        {/each}
      </div>
      <div class="flex-1 overflow-y-auto p-6">
        {#if currentNpc}
          <div class="max-w-3xl space-y-5">
            <div class="flex items-center gap-3"><Star size={20} class="text-amber-400" /><h3 class="text-lg font-semibold text-zinc-100 font-mono">{selectedNpc}</h3></div>
            <div><label class="text-[10px] text-zinc-500 uppercase">Title Lang Key</label><input type="text" value={currentNpc.titleLangKey} onchange={(e) => { currentNpc.titleLangKey = (e.target as HTMLInputElement).value; markChanged(); }} class="w-full text-sm font-mono mt-0.5" /></div>

            <!-- Reputation progress visualization -->
            <div class="bg-zinc-900 rounded-lg p-4">
              <div class="text-[10px] text-zinc-500 uppercase mb-2">Progression</div>
              <div class="relative h-8 bg-zinc-800 rounded overflow-hidden">
                {#each currentNpc.ranks as rank, idx}
                  {@const maxRep = currentNpc.ranks[currentNpc.ranks.length - 1]?.min || 1}
                  <div class="absolute top-0 bottom-0 border-l border-amber-500/50" style="left: {(rank.min / maxRep) * 100}%" title="{rank.min}: {rank.rankLangKey}">
                    <div class="absolute -top-0.5 -left-1 w-2 h-2 bg-amber-400 rounded-full"></div>
                  </div>
                {/each}
                <div class="absolute inset-0 bg-gradient-to-r from-zinc-700/50 to-amber-500/20 rounded"></div>
              </div>
              <div class="flex justify-between text-[10px] text-zinc-600 mt-1">
                <span>0</span>
                <span>{currentNpc.ranks[currentNpc.ranks.length - 1]?.min || 0}</span>
              </div>
            </div>

            <!-- Ranks -->
            <div class="border border-zinc-800 rounded-lg p-4">
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-semibold text-zinc-100">Ranks ({currentNpc.ranks.length})</h4>
                <button onclick={addRank} class="flex items-center gap-1 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 px-2 py-1 rounded text-xs"><Plus size={11} /></button>
              </div>
              <div class="space-y-2">
                {#each currentNpc.ranks as rank, idx}
                  <div class="bg-zinc-900 border border-zinc-800 rounded-md p-3">
                    <div class="flex items-center gap-3">
                      <div class="flex items-center gap-2 shrink-0">
                        <TrendingUp size={12} class="text-amber-400" />
                        <input type="number" value={rank.min} onchange={(e) => updateRank(idx, "min", (e.target as HTMLInputElement).value)} class="w-16 text-xs text-center" />
                      </div>
                      <input type="text" value={rank.rankLangKey} onchange={(e) => updateRank(idx, "rankLangKey", (e.target as HTMLInputElement).value)} class="flex-1 text-xs font-mono" />
                      <button onclick={() => removeRank(idx)} class="p-1 text-zinc-500 hover:text-red-400 rounded"><X size={12} /></button>
                    </div>
                    {#if rank.rewardAction !== undefined || idx > 0}
                      <div class="mt-2">
                        <input type="text" value={rank.rewardAction || ""} onchange={(e) => updateRank(idx, "rewardAction", (e.target as HTMLInputElement).value)} class="w-full text-[11px] font-mono text-emerald-400/70" placeholder="reward action..." />
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
