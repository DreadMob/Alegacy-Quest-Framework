<script lang="ts">
  import { RefreshCw, Save, Plus, X, AlertCircle, Gem, Trash2 } from "lucide-svelte";
  import { saveJsonFile } from "../lib/fileService";
  import { t, type Lang } from "../lib/i18n";

  interface Props { modPath: string; lang: Lang; }
  let { modPath, lang }: Props = $props();

  interface QualityTier {
    id: string; name: string; color: string; chance: number;
    minBonusPercent: number; maxBonusPercent: number;
    bonusMode: string; perAttribute: boolean; exclusive: boolean;
    applicableItems: string[];
  }
  interface QualityConfig { qualities: QualityTier[]; }

  let config = $state<QualityConfig | null>(null);
  let configPath = $state("");
  let loading = $state(false);
  let error = $state("");
  let selected = $state<number>(-1);
  let unsaved = $state(false);
  let saveStatus = $state("");

  let current = $derived.by(() => config && selected >= 0 && selected < config.qualities.length ? config.qualities[selected] : null);

  async function load() {
    if (!modPath) return;
    loading = true; error = "";
    try {
      const api = window.electronAPI;
      const assetsPath = await api.joinPath(modPath, "assets");
      const dirs = await api.readDir(assetsPath);
      for (const d of dirs.entries.filter((e: any) => e.isDirectory)) {
        const qPath = await api.joinPath(assetsPath, d.name, "config", "qualityconfig.json");
        if (await api.exists(qPath)) {
          const content = await api.readTextFile(qPath);
          config = JSON.parse(content.replace(/^\uFEFF/, ""));
          configPath = qPath;
          if (config!.qualities.length > 0) selected = 0;
          loading = false; return;
        }
      }
      error = lang === "ru" ? "qualityconfig.json не найден" : "qualityconfig.json not found";
    } catch (e: any) { error = e?.message || e; }
    loading = false;
  }

  function markChanged() { unsaved = true; config = { ...config! }; }

  function updateField(field: string, value: string) {
    if (!current) return;
    if (["chance","minBonusPercent","maxBonusPercent"].includes(field)) {
      const n = parseFloat(value); if (!isNaN(n)) (current as any)[field] = n;
    } else if (["perAttribute","exclusive"].includes(field)) {
      (current as any)[field] = value === "true";
    } else { (current as any)[field] = value; }
    markChanged();
  }

  function addTier() {
    if (!config) return;
    config.qualities.push({ id: "new-tier", name: "NEW", color: "#FFFFFF", chance: 0.1, minBonusPercent: 0, maxBonusPercent: 10, bonusMode: "all", perAttribute: true, exclusive: false, applicableItems: [] });
    selected = config.qualities.length - 1; markChanged();
  }

  function removeTier(idx: number) {
    if (!config) return;
    config.qualities.splice(idx, 1);
    if (selected >= config.qualities.length) selected = config.qualities.length - 1;
    markChanged();
  }

  function addItem() {
    if (!current) return;
    current.applicableItems.push("modid:item-id");
    markChanged();
  }

  function removeItem(idx: number) {
    if (!current) return;
    current.applicableItems.splice(idx, 1);
    markChanged();
  }

  function updateItem(idx: number, value: string) {
    if (!current) return;
    current.applicableItems[idx] = value;
    markChanged();
  }

  async function save() {
    if (!config || !configPath) return;
    saveStatus = "...";
    try {
      await saveJsonFile(configPath, config);
      unsaved = false; saveStatus = t("bosses.saved", lang); setTimeout(() => saveStatus = "", 3000);
    } catch (e: any) { saveStatus = `Error: ${e?.message || e}`; }
  }

  $effect(() => { if (modPath) load(); });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">{t("nav.quality", lang)}</h2>
    <span class="text-xs text-zinc-500">{config?.qualities.length ?? 0} tiers</span>
    <div class="ml-auto flex items-center gap-2">
      <button onclick={addTier} class="flex items-center gap-1.5 bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Plus size={13} />{t("common.add", lang)}</button>
      <button onclick={load} disabled={loading} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-50"><RefreshCw size={13} class={loading ? "animate-spin" : ""} /></button>
      {#if unsaved}
        <button onclick={save} class="flex items-center gap-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Save size={13} />{t("quests.save", lang)}</button>
      {/if}
    </div>
  </div>
  {#if saveStatus}<div class="px-6 py-1.5 text-xs text-emerald-400 bg-emerald-500/5 border-b border-emerald-500/20">{saveStatus}</div>{/if}
  {#if error}<div class="mx-6 mt-3 flex items-start gap-2 bg-red-500/10 border border-red-500/30 rounded-md p-3 text-sm text-red-400"><AlertCircle size={16} class="shrink-0 mt-0.5" /><span>{error}</span></div>{/if}

  {#if loading}
    <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm"><RefreshCw size={16} class="animate-spin mr-2" /></div>
  {:else if config}
    <div class="flex-1 flex overflow-hidden">
      <div class="w-56 border-r border-zinc-800 bg-zinc-950 flex flex-col overflow-y-auto p-2 space-y-0.5">
        {#each config.qualities as tier, idx}
          <button onclick={() => selected = idx} class="w-full flex items-center gap-2 px-3 py-2 rounded-md text-xs transition-colors {selected === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'}">
            <div class="w-3 h-3 rounded-full shrink-0" style="background-color: {tier.color}"></div>
            <span class="truncate font-medium">{tier.name}</span>
            <span class="text-[10px] text-zinc-600 ml-auto">{(tier.chance * 100).toFixed(0)}%</span>
          </button>
        {/each}
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        {#if current}
          <div class="max-w-3xl space-y-5">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="w-6 h-6 rounded-md border border-zinc-700" style="background-color: {current.color}"></div>
                <h3 class="text-lg font-bold" style="color: {current.color}">{current.name}</h3>
              </div>
              <button onclick={() => removeTier(selected)} class="flex items-center gap-1.5 text-red-400 hover:bg-red-500/10 px-2 py-1 rounded text-xs"><Trash2 size={13} />{t("common.delete", lang)}</button>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div><label class="text-[10px] text-zinc-500 uppercase">ID</label><input type="text" value={current.id} onchange={(e) => updateField("id", (e.target as HTMLInputElement).value)} class="w-full text-sm font-mono mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Name</label><input type="text" value={current.name} onchange={(e) => updateField("name", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Color</label><div class="flex gap-2 mt-0.5"><input type="color" value={current.color} oninput={(e) => updateField("color", (e.target as HTMLInputElement).value)} class="w-10 h-8 rounded border border-zinc-700 cursor-pointer bg-transparent p-0.5" /><input type="text" value={current.color} onchange={(e) => updateField("color", (e.target as HTMLInputElement).value)} class="flex-1 text-sm font-mono" /></div></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Chance</label><input type="number" step="0.01" value={current.chance} onchange={(e) => updateField("chance", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Min Bonus %</label><input type="number" value={current.minBonusPercent} onchange={(e) => updateField("minBonusPercent", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Max Bonus %</label><input type="number" value={current.maxBonusPercent} onchange={(e) => updateField("maxBonusPercent", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Bonus Mode</label><select value={current.bonusMode} onchange={(e) => updateField("bonusMode", (e.target as HTMLSelectElement).value)} class="w-full text-sm mt-0.5 bg-zinc-900 border border-zinc-800 rounded px-2 py-1 text-zinc-300"><option value="all">all</option><option value="random">random</option><option value="single">single</option></select></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Per Attribute</label><select value={current.perAttribute ? "true" : "false"} onchange={(e) => updateField("perAttribute", (e.target as HTMLSelectElement).value)} class="w-full text-sm mt-0.5 bg-zinc-900 border border-zinc-800 rounded px-2 py-1 text-zinc-300"><option value="true">true</option><option value="false">false</option></select></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Exclusive</label><select value={current.exclusive ? "true" : "false"} onchange={(e) => updateField("exclusive", (e.target as HTMLSelectElement).value)} class="w-full text-sm mt-0.5 bg-zinc-900 border border-zinc-800 rounded px-2 py-1 text-zinc-300"><option value="true">true</option><option value="false">false</option></select></div>
            </div>

            <!-- Applicable Items -->
            <div class="border border-zinc-800 rounded-lg p-4">
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-semibold text-zinc-100">Applicable Items <span class="text-zinc-500 font-normal">({current.applicableItems.length})</span></h4>
                <button onclick={addItem} class="flex items-center gap-1 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 px-2 py-1 rounded text-xs"><Plus size={11} /></button>
              </div>
              <div class="space-y-1.5 max-h-60 overflow-y-auto">
                {#each current.applicableItems as item, idx}
                  <div class="flex items-center gap-2">
                    <input type="text" value={item} onchange={(e) => updateItem(idx, (e.target as HTMLInputElement).value)} class="flex-1 text-xs font-mono" />
                    <button onclick={() => removeItem(idx)} class="p-1 text-zinc-500 hover:text-red-400 rounded"><X size={12} /></button>
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
