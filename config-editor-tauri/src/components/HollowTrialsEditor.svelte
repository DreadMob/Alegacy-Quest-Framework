<script lang="ts">
  import { RefreshCw, Save, Plus, X, AlertCircle, Flame, Trash2, Shield } from "lucide-svelte";
  import { saveJsonFile } from "../lib/fileService";
  import { t, type Lang } from "../lib/i18n";

  interface Props { modPath: string; lang: Lang; }
  let { modPath, lang }: Props = $props();

  interface Challenge { type: string; thresholdMinutes?: number; abilityCode?: string; maxArmorTier?: number; }
  interface Tier { questId: string; maxHealth: number; damageMult: number; speedMult: number; enrageTimerSeconds: number; challenges: Challenge[]; }
  interface TrialConfig { trialKey: string; entityCode: string; respawnInGameHours: number; activationRange: number; softResetIdleHours: number; tiers: Record<string, Tier>; }

  let trials = $state<{ path: string; data: TrialConfig }[]>([]);
  let loading = $state(false);
  let error = $state("");
  let selected = $state<number>(-1);
  let selectedTier = $state<string>("1");
  let unsavedChanges = $state<Set<string>>(new Set());
  let saveStatus = $state("");

  let current = $derived.by(() => selected >= 0 && selected < trials.length ? trials[selected] : null);
  let currentTier = $derived.by(() => current?.data.tiers[selectedTier] || null);

  const CHALLENGE_TYPES = ["speedkill", "deathless", "nofood", "lowgear", "perfectdodge", "soloonly", "nohealing"];

  async function load() {
    if (!modPath) return;
    loading = true; error = "";
    try {
      const api = window.electronAPI;
      const assetsPath = await api.joinPath(modPath, "assets");
      const dirs = await api.readDir(assetsPath);
      const result: typeof trials = [];
      for (const d of dirs.entries.filter((e: any) => e.isDirectory)) {
        const htPath = await api.joinPath(assetsPath, d.name, "config", "hollowtrials");
        if (!(await api.exists(htPath))) continue;
        const files = await api.findJsonFiles(htPath);
        const parsed = await api.readJsonFiles(files);
        for (const { path, data, error: err } of parsed) {
          if (err || !data?.trialKey) continue;
          result.push({ path, data: data as TrialConfig });
        }
      }
      trials = result;
      if (trials.length > 0 && selected < 0) selected = 0;
      if (trials.length === 0) error = lang === "ru" ? "Конфиги испытаний не найдены" : "No trial configs found";
    } catch (e: any) { error = e?.message || e; }
    loading = false;
  }

  function markChanged() { if (!current) return; unsavedChanges.add(current.path); unsavedChanges = new Set(unsavedChanges); trials = [...trials]; }

  function updateMeta(field: string, value: string) {
    if (!current) return;
    const num = parseFloat(value);
    if (!isNaN(num) && ["respawnInGameHours","activationRange","softResetIdleHours"].includes(field)) (current.data as any)[field] = num;
    else (current.data as any)[field] = value;
    markChanged();
  }

  function updateTier(field: string, value: string) {
    if (!currentTier) return;
    const num = parseFloat(value);
    if (!isNaN(num)) (currentTier as any)[field] = num;
    else (currentTier as any)[field] = value;
    markChanged();
  }

  function addChallenge(type: string) {
    if (!currentTier || !type) return;
    const ch: Challenge = { type };
    if (type === "speedkill") ch.thresholdMinutes = 3;
    if (type === "lowgear") ch.maxArmorTier = 3;
    if (type === "perfectdodge") ch.abilityCode = "bossability";
    currentTier.challenges.push(ch);
    markChanged();
  }

  function removeChallenge(idx: number) { if (!currentTier) return; currentTier.challenges.splice(idx, 1); markChanged(); }

  async function save() {
    if (!current) return; saveStatus = "...";
    try { await saveJsonFile(current.path, current.data); unsavedChanges.delete(current.path); unsavedChanges = new Set(unsavedChanges); saveStatus = t("bosses.saved", lang); setTimeout(() => saveStatus = "", 3000); }
    catch (e: any) { saveStatus = `Error: ${e?.message || e}`; }
  }

  $effect(() => { if (modPath) load(); });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">{t("nav.trials", lang)}</h2>
    <span class="text-xs text-zinc-500">{trials.length}</span>
    <div class="ml-auto flex items-center gap-2">
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
  {:else if trials.length > 0}
    <div class="flex-1 flex overflow-hidden">
      <div class="w-56 border-r border-zinc-800 bg-zinc-950 flex flex-col overflow-y-auto p-2 space-y-0.5">
        {#each trials as trial, idx}
          <button onclick={() => { selected = idx; selectedTier = "1"; }} class="w-full flex items-center gap-2 px-3 py-2 rounded-md text-xs transition-colors {selected === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'} {unsavedChanges.has(trial.path) ? '!border-emerald-500/40' : ''}">
            <Flame size={13} class="shrink-0 text-orange-400" />
            <span class="truncate font-mono text-[11px]">{trial.data.entityCode.split(":").pop()}</span>
          </button>
        {/each}
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        {#if current}
          <div class="max-w-3xl space-y-5">
            <div class="flex items-center gap-3"><Flame size={20} class="text-orange-400" /><h3 class="text-lg font-semibold text-zinc-100 font-mono">{current.data.trialKey}</h3></div>

            <!-- Meta -->
            <div class="grid grid-cols-2 gap-4">
              <div><label class="text-[10px] text-zinc-500 uppercase">Trial Key</label><input type="text" value={current.data.trialKey} onchange={(e) => updateMeta("trialKey", (e.target as HTMLInputElement).value)} class="w-full text-sm font-mono mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Entity Code</label><input type="text" value={current.data.entityCode} onchange={(e) => updateMeta("entityCode", (e.target as HTMLInputElement).value)} class="w-full text-sm font-mono mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Respawn (game h)</label><input type="number" value={current.data.respawnInGameHours} onchange={(e) => updateMeta("respawnInGameHours", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Activation Range</label><input type="number" value={current.data.activationRange} onchange={(e) => updateMeta("activationRange", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
            </div>

            <!-- Tier selector -->
            <div class="flex items-center gap-2">
              {#each Object.keys(current.data.tiers).sort() as tier}
                <button onclick={() => selectedTier = tier} class="px-4 py-2 rounded-md text-xs font-semibold transition-colors {selectedTier === tier ? 'bg-orange-500/20 text-orange-400 border border-orange-500/30' : 'bg-zinc-900 text-zinc-400 border border-zinc-800 hover:text-zinc-200'}">
                  Tier {tier}
                </button>
              {/each}
            </div>

            <!-- Tier details -->
            {#if currentTier}
              <div class="border border-zinc-800 rounded-lg p-4 space-y-4">
                <div class="grid grid-cols-2 lg:grid-cols-3 gap-3">
                  <div><label class="text-[10px] text-zinc-500 uppercase">Max Health</label><input type="number" value={currentTier.maxHealth} onchange={(e) => updateTier("maxHealth", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
                  <div><label class="text-[10px] text-zinc-500 uppercase">Damage Mult</label><input type="number" step="0.1" value={currentTier.damageMult} onchange={(e) => updateTier("damageMult", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
                  <div><label class="text-[10px] text-zinc-500 uppercase">Speed Mult</label><input type="number" step="0.05" value={currentTier.speedMult} onchange={(e) => updateTier("speedMult", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
                  <div><label class="text-[10px] text-zinc-500 uppercase">Enrage Timer (s)</label><input type="number" value={currentTier.enrageTimerSeconds} onchange={(e) => updateTier("enrageTimerSeconds", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
                  <div><label class="text-[10px] text-zinc-500 uppercase">Quest ID</label><input type="text" value={currentTier.questId} onchange={(e) => updateTier("questId", (e.target as HTMLInputElement).value)} class="w-full text-sm font-mono mt-0.5" /></div>
                </div>

                <!-- Difficulty visualization -->
                <div class="bg-zinc-900 rounded-md p-3">
                  <div class="text-[10px] text-zinc-500 uppercase mb-2">Difficulty</div>
                  <div class="space-y-1.5">
                    <div class="flex items-center gap-2"><span class="text-[10px] text-zinc-400 w-16">HP</span><div class="flex-1 h-2 bg-zinc-800 rounded"><div class="h-full bg-red-500/60 rounded" style="width: {Math.min(100, currentTier.maxHealth / 20)}%"></div></div><span class="text-[10px] text-zinc-500 w-12 text-right">{currentTier.maxHealth}</span></div>
                    <div class="flex items-center gap-2"><span class="text-[10px] text-zinc-400 w-16">Damage</span><div class="flex-1 h-2 bg-zinc-800 rounded"><div class="h-full bg-orange-500/60 rounded" style="width: {Math.min(100, currentTier.damageMult * 50)}%"></div></div><span class="text-[10px] text-zinc-500 w-12 text-right">×{currentTier.damageMult}</span></div>
                    <div class="flex items-center gap-2"><span class="text-[10px] text-zinc-400 w-16">Speed</span><div class="flex-1 h-2 bg-zinc-800 rounded"><div class="h-full bg-blue-500/60 rounded" style="width: {Math.min(100, currentTier.speedMult * 50)}%"></div></div><span class="text-[10px] text-zinc-500 w-12 text-right">×{currentTier.speedMult}</span></div>
                  </div>
                </div>

                <!-- Challenges -->
                <div>
                  <div class="flex items-center justify-between mb-2">
                    <h4 class="text-sm font-semibold text-zinc-100 flex items-center gap-2"><Shield size={14} class="text-amber-400" />Challenges ({currentTier.challenges.length})</h4>
                    <select onchange={(e) => { const v = (e.target as HTMLSelectElement).value; if (v) { addChallenge(v); (e.target as HTMLSelectElement).value = ""; } }} class="bg-zinc-800 border border-zinc-700 text-zinc-300 text-xs rounded px-2 py-1">
                      <option value="">+ Add...</option>
                      {#each CHALLENGE_TYPES as ct}<option value={ct}>{ct}</option>{/each}
                    </select>
                  </div>
                  <div class="space-y-1.5">
                    {#each currentTier.challenges as ch, idx}
                      <div class="flex items-center gap-2 bg-zinc-900 border border-zinc-800 rounded px-3 py-2">
                        <span class="text-xs font-mono text-amber-400 font-semibold">{ch.type}</span>
                        {#if ch.thresholdMinutes !== undefined}<span class="text-[10px] text-zinc-500">{ch.thresholdMinutes}min</span>{/if}
                        {#if ch.abilityCode}<span class="text-[10px] text-zinc-500 font-mono">{ch.abilityCode}</span>{/if}
                        {#if ch.maxArmorTier !== undefined}<span class="text-[10px] text-zinc-500">tier≤{ch.maxArmorTier}</span>{/if}
                        <button onclick={() => removeChallenge(idx)} class="ml-auto p-1 text-zinc-500 hover:text-red-400 rounded"><X size={12} /></button>
                      </div>
                    {/each}
                  </div>
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
