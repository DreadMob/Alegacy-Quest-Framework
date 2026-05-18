<script lang="ts">
  import { RefreshCw, Save, Plus, Search, X, AlertCircle, Target, Trash2 } from "lucide-svelte";
  import { loadBossHuntConfigs, saveJsonFile } from "../lib/fileService";
  import { t, type Lang } from "../lib/i18n";

  interface Props { modPath: string; lang: Lang; }
  let { modPath, lang }: Props = $props();

  interface BossHuntConfig {
    bossKey: string; questId: string; relocateIntervalHours: number;
    respawnInGameHours: number; noRelocateAfterDamageMinutes: number;
    rotationDays: number; activationRange: number; playerLockRange: number;
  }

  let configs = $state<{ path: string; data: BossHuntConfig }[]>([]);
  let loading = $state(false);
  let error = $state("");
  let selected = $state<number>(-1);
  let unsavedChanges = $state<Set<string>>(new Set());
  let saveStatus = $state("");

  let current = $derived.by(() => selected >= 0 && selected < configs.length ? configs[selected] : null);

  async function load() {
    if (!modPath) return;
    loading = true; error = "";
    try {
      configs = await loadBossHuntConfigs(modPath);
      if (configs.length > 0 && selected < 0) selected = 0;
      if (configs.length === 0) error = lang === "ru" ? "Конфиги босс-ханта не найдены" : "No boss hunt configs found";
    } catch (e: any) { error = e?.message || e; }
    loading = false;
  }

  function update(field: string, value: string) {
    if (!current) return;
    const num = parseFloat(value);
    if (!isNaN(num)) (current.data as any)[field] = num;
    else (current.data as any)[field] = value;
    unsavedChanges.add(current.path); unsavedChanges = new Set(unsavedChanges);
    configs = [...configs];
  }

  async function save() {
    if (!current) return;
    saveStatus = "...";
    try {
      await saveJsonFile(current.path, current.data);
      unsavedChanges.delete(current.path); unsavedChanges = new Set(unsavedChanges);
      saveStatus = t("bosses.saved", lang); setTimeout(() => saveStatus = "", 3000);
    } catch (e: any) { saveStatus = `Error: ${e?.message || e}`; }
  }

  function createNew() {
    if (configs.length === 0) return;
    const path = current?.path || configs[0].path;
    const dir = path.substring(0, path.lastIndexOf("\\"));
    const newConfig: BossHuntConfig = {
      bossKey: "modid:bosshunt:new-boss", questId: "modid:bosshunt-new-boss",
      relocateIntervalHours: 168, respawnInGameHours: 24,
      noRelocateAfterDamageMinutes: 10, rotationDays: 365,
      activationRange: 160, playerLockRange: 0,
    };
    const newPath = dir + "\\new-boss.json";
    configs = [...configs, { path: newPath, data: newConfig }];
    unsavedChanges.add(newPath); unsavedChanges = new Set(unsavedChanges);
    selected = configs.length - 1;
  }

  $effect(() => { if (modPath) load(); });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">{t("nav.bosshunt", lang)}</h2>
    <span class="text-xs text-zinc-500">{configs.length}</span>
    <div class="ml-auto flex items-center gap-2">
      <button onclick={createNew} class="flex items-center gap-1.5 bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Plus size={13} />{t("common.add", lang)}</button>
      <button onclick={load} disabled={loading} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-50"><RefreshCw size={13} class={loading ? "animate-spin" : ""} /></button>
      {#if unsavedChanges.size > 0}
        <button onclick={save} class="flex items-center gap-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Save size={13} />{t("quests.save", lang)}</button>
      {/if}
    </div>
  </div>
  {#if saveStatus}<div class="px-6 py-1.5 text-xs text-emerald-400 bg-emerald-500/5 border-b border-emerald-500/20">{saveStatus}</div>{/if}
  {#if error}<div class="mx-6 mt-3 flex items-start gap-2 bg-red-500/10 border border-red-500/30 rounded-md p-3 text-sm text-red-400"><AlertCircle size={16} class="shrink-0 mt-0.5" /><span>{error}</span></div>{/if}

  {#if loading}
    <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm"><RefreshCw size={16} class="animate-spin mr-2" /></div>
  {:else if configs.length > 0}
    <div class="flex-1 flex overflow-hidden">
      <div class="w-64 border-r border-zinc-800 bg-zinc-950 flex flex-col overflow-y-auto p-2 space-y-0.5">
        {#each configs as cfg, idx}
          <button onclick={() => selected = idx} class="w-full text-left px-3 py-2 rounded-md text-xs transition-colors {selected === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'} {unsavedChanges.has(cfg.path) ? '!border-emerald-500/40' : ''}">
            <div class="font-mono text-[11px] truncate">{cfg.data.bossKey.split(":").pop()}</div>
          </button>
        {/each}
      </div>
      <div class="flex-1 overflow-y-auto p-6">
        {#if current}
          <div class="max-w-2xl space-y-4">
            <div class="flex items-center gap-3 mb-4"><Target size={20} class="text-zinc-400" /><h3 class="text-lg font-semibold text-zinc-100 font-mono">{current.data.bossKey}</h3></div>
            <div class="grid grid-cols-2 gap-4">
              {#each [["bossKey","Boss Key"],["questId","Quest ID"]] as [field, label]}
                <div><label class="text-[10px] text-zinc-500 uppercase">{label}</label><input type="text" value={current.data[field]} onchange={(e) => update(field, (e.target as HTMLInputElement).value)} class="w-full text-sm font-mono mt-0.5" /></div>
              {/each}
              {#each [["relocateIntervalHours","Relocate Interval (h)"],["respawnInGameHours","Respawn (game h)"],["noRelocateAfterDamageMinutes","No Relocate After Dmg (min)"],["rotationDays","Rotation Days"],["activationRange","Activation Range"],["playerLockRange","Player Lock Range"]] as [field, label]}
                <div><label class="text-[10px] text-zinc-500 uppercase">{label}</label><input type="number" value={current.data[field]} onchange={(e) => update(field, (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
              {/each}
            </div>
            <div class="text-[10px] text-zinc-600 font-mono border-t border-zinc-800 pt-3 mt-6">{current.path}</div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
