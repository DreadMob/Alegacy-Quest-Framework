<script lang="ts">
  import { RefreshCw, Save, Plus, Search, X, AlertCircle, ScrollText, Target, Gift, Play, Trash2, ChevronDown, ChevronRight, Layers, Clock, MapPin, Sword, Eye, Zap, Copy } from "lucide-svelte";
  import { loadQuestConfigs, saveJsonFile } from "../lib/fileService";
  import { t, type Lang } from "../lib/i18n";
  import { QUEST_OBJECTIVES, QUEST_OBJECTIVE_CODES, QUEST_ACTIONS, QUEST_ACTION_CODES } from "../lib/types";

  interface Props { modPath: string; lang: Lang; }
  let { modPath, lang }: Props = $props();

  interface QuestConfig {
    id: string; cooldown?: number; perPlayer?: boolean; predecessor?: string;
    killObjectives?: { validCodes: string[]; demand: number }[];
    actionObjectives?: { id: string; objectiveId?: string; args: string[]; onCompleteActions?: string }[];
    gatherObjectives?: { validCodes: string[]; demand: number }[];
    itemRewards?: { itemCode: string; amount: number }[];
    randomItemRewards?: { selectAmount: number; items: { itemCode: string; minAmount: number; maxAmount: number }[] };
    actionRewards?: { id: string; args: string[] }[];
    onAcceptedActions?: { id: string; args: string[] }[];
    onFailedActions?: { id: string; args: string[] }[];
    stages?: Stage[];
    [key: string]: any;
  }

  interface Stage {
    stageTitleLangKey: string;
    gatherObjectives?: { validCodes: string[]; demand: number }[];
    killObjectives?: { validCodes: string[]; demand: number }[];
    actionObjectives?: { id: string; objectiveId?: string; args: string[]; onCompleteActions?: string }[];
    onStageCompleteActions?: { id: string; args: string[] }[];
  }

  let quests = $state<{ path: string; data: QuestConfig; category: string }[]>([]);
  let loading = $state(false);
  let error = $state("");
  let selectedQuest = $state<number>(-1);
  let filter = $state("");
  let unsavedChanges = $state<Set<string>>(new Set());
  let saveStatus = $state("");
  let expandedSections = $state<Set<string>>(new Set(["meta","killObj","actionObj","itemRewards","actionRewards","onAccepted","stages","gatherObj","randomRewards"]));
  let selectedStage = $state<number>(0);

  let categories = $derived.by(() => { const cats = new Set<string>(); for (const q of quests) cats.add(q.category); return Array.from(cats).sort(); });
  let filteredQuests = $derived.by(() => { if (!filter) return quests; const f = filter.toLowerCase(); return quests.filter(q => q.data.id.toLowerCase().includes(f) || q.category.toLowerCase().includes(f)); });
  let currentQuest = $derived.by(() => selectedQuest >= 0 && selectedQuest < quests.length ? quests[selectedQuest] : null);
  let hasStages = $derived.by(() => currentQuest?.data.stages && currentQuest.data.stages.length > 0);
  let currentStage = $derived.by(() => hasStages && currentQuest?.data.stages?.[selectedStage] ? currentQuest.data.stages[selectedStage] : null);

  function toggleSection(id: string) { if (expandedSections.has(id)) expandedSections.delete(id); else expandedSections.add(id); expandedSections = new Set(expandedSections); }
  function markChanged() { if (!currentQuest) return; unsavedChanges.add(currentQuest.path); unsavedChanges = new Set(unsavedChanges); quests = [...quests]; }

  async function loadQuests() {
    if (!modPath) { error = t("quests.selectFolder", lang); return; }
    loading = true; error = "";
    try {
      const results = await loadQuestConfigs(modPath);
      quests = results.map(r => ({ path: r.path, data: r.data as QuestConfig, category: r.category }));
      if (quests.length > 0 && selectedQuest < 0) selectedQuest = 0;
      if (quests.length === 0) error = t("quests.notFound", lang);
    } catch (e: any) { error = `${e?.message || e}`; }
    loading = false;
  }

  function updateMeta(field: string, value: string) {
    if (!currentQuest) return;
    if (field === "cooldown") { const n = parseInt(value); currentQuest.data.cooldown = isNaN(n) ? -1 : n; }
    else if (field === "perPlayer") currentQuest.data.perPlayer = value === "true";
    else if (field === "predecessor") currentQuest.data.predecessor = value || undefined;
    else if (field === "id") currentQuest.data.id = value;
    markChanged();
  }

  // Generic action/objective helpers
  function addToArray(arr: any[], item: any) { arr.push(item); markChanged(); }
  function removeFromArray(arr: any[], idx: number) { arr.splice(idx, 1); markChanged(); }
  function updateArg(obj: { args: string[] }, idx: number, value: string) { obj.args[idx] = value; markChanged(); }
  function addArg(obj: { args: string[] }) { obj.args.push(""); markChanged(); }
  function removeArg(obj: { args: string[] }, idx: number) { obj.args.splice(idx, 1); markChanged(); }

  // Kill/Gather objectives
  function addKillObj(target?: Stage) {
    const arr = target ? (target.killObjectives ??= []) : (currentQuest!.data.killObjectives ??= []);
    arr.push({ validCodes: ["entity-code"], demand: 1 }); markChanged();
  }
  function addGatherObj(target?: Stage) {
    const arr = target ? (target.gatherObjectives ??= []) : (currentQuest!.data.gatherObjectives ??= []);
    arr.push({ validCodes: ["game:item-code"], demand: 1 }); markChanged();
  }
  function addActionObj(type: string, target?: Stage) {
    const arr = target ? (target.actionObjectives ??= []) : (currentQuest!.data.actionObjectives ??= []);
    arr.push({ id: type, args: [currentQuest!.data.id] }); markChanged();
  }
  function addAction(type: string, section: "actionRewards" | "onAcceptedActions" | "onFailedActions") {
    if (!currentQuest) return;
    const arr = (currentQuest.data[section] ??= []) as any[];
    arr.push({ id: type, args: [] }); markChanged();
  }
  function addStageAction(type: string) {
    if (!currentStage) return;
    (currentStage.onStageCompleteActions ??= []).push({ id: type, args: [] }); markChanged();
  }

  // Item rewards
  function addItemReward() { if (!currentQuest) return; (currentQuest.data.itemRewards ??= []).push({ itemCode: "game:item", amount: 1 }); markChanged(); }
  function addRandomRewardItem() {
    if (!currentQuest) return;
    if (!currentQuest.data.randomItemRewards) currentQuest.data.randomItemRewards = { selectAmount: 2, items: [] };
    currentQuest.data.randomItemRewards.items.push({ itemCode: "game:item", minAmount: 1, maxAmount: 3 }); markChanged();
  }

  // Stages
  function addStage() {
    if (!currentQuest) return;
    (currentQuest.data.stages ??= []).push({ stageTitleLangKey: `${currentQuest.data.id}-stage${(currentQuest.data.stages?.length || 0) + 1}-title`, actionObjectives: [] });
    selectedStage = (currentQuest.data.stages?.length || 1) - 1; markChanged();
  }
  function removeStage(idx: number) {
    if (!currentQuest?.data.stages) return;
    currentQuest.data.stages.splice(idx, 1);
    if (selectedStage >= (currentQuest.data.stages.length || 0)) selectedStage = Math.max(0, (currentQuest.data.stages.length || 0) - 1);
    markChanged();
  }
  function convertToStaged() {
    if (!currentQuest || hasStages) return;
    const stage: Stage = { stageTitleLangKey: `${currentQuest.data.id}-stage1-title`, killObjectives: currentQuest.data.killObjectives, gatherObjectives: currentQuest.data.gatherObjectives, actionObjectives: currentQuest.data.actionObjectives };
    currentQuest.data.stages = [stage];
    delete currentQuest.data.killObjectives; delete currentQuest.data.gatherObjectives; delete currentQuest.data.actionObjectives;
    selectedStage = 0; markChanged();
  }

  // Duplicate quest
  function duplicateQuest() {
    if (!currentQuest) return;
    const newData = JSON.parse(JSON.stringify(currentQuest.data));
    newData.id = newData.id + "-copy";
    quests = [...quests, { path: currentQuest.path, data: newData, category: currentQuest.category }];
    unsavedChanges.add(currentQuest.path); unsavedChanges = new Set(unsavedChanges);
    selectedQuest = quests.length - 1;
  }

  async function saveCurrentQuest() {
    if (!currentQuest) return; saveStatus = t("quests.saving", lang);
    try { await saveJsonFile(currentQuest.path, currentQuest.data); unsavedChanges.delete(currentQuest.path); unsavedChanges = new Set(unsavedChanges); saveStatus = t("quests.saved", lang); setTimeout(() => saveStatus = "", 3000); }
    catch (e: any) { saveStatus = `Error: ${e?.message || e}`; }
  }

  // Objective icon helper
  function getObjIcon(id: string): string {
    if (["walkdistance","reachwaypoint","inland","landgate"].includes(id)) return "🗺️";
    if (["randomkill","killnear","killactiontarget","killwithweapon"].includes(id)) return "⚔️";
    if (["hasitem","wearing"].includes(id)) return "🎒";
    if (["interactat","interactcount","interactwithentity"].includes(id)) return "👆";
    if (["timeofday","timer"].includes(id)) return "⏰";
    if (["checkvariable","hasattribute"].includes(id)) return "🔍";
    if (["harvestcrop","mineblock","placeblock","fishcatch","craftitem","smeltitem"].includes(id)) return "⛏️";
    if (["temporalstorm"].includes(id)) return "🌩️";
    if (["sequence"].includes(id)) return "📋";
    return "📌";
  }

  $effect(() => { if (modPath) loadQuests(); });
</script>

{#snippet actionNode(action: {id: string; args: string[]}, onRemove: () => void, color: string)}
  <div class="bg-zinc-950 border border-zinc-800 rounded-md p-2.5">
    <div class="flex items-center gap-2 mb-1.5">
      <span class="font-mono text-xs font-semibold" style="color: {color}">{action.id}</span>
      <span class="text-[10px] text-zinc-500 truncate flex-1">{QUEST_ACTIONS[action.id] || ""}</span>
      <button onclick={onRemove} class="p-0.5 text-zinc-600 hover:text-red-400 rounded"><X size={11} /></button>
    </div>
    <div class="flex flex-wrap gap-1 items-center">
      {#each action.args as arg, argIdx}
        <input type="text" value={arg} onchange={(e) => { action.args[argIdx] = (e.target as HTMLInputElement).value; markChanged(); }} class="text-[11px] font-mono w-28 px-1.5 py-0.5 bg-zinc-900 border border-zinc-800 rounded" />
      {/each}
      <button onclick={() => { action.args.push(""); markChanged(); }} class="text-[10px] text-zinc-600 hover:text-zinc-300 border border-dashed border-zinc-700 rounded px-1.5 py-0.5">+</button>
    </div>
  </div>
{/snippet}

{#snippet objectiveNode(obj: {id: string; objectiveId?: string; args: string[]; onCompleteActions?: string}, onRemove: () => void)}
  <div class="bg-zinc-950 border border-zinc-800 rounded-md p-2.5">
    <div class="flex items-center gap-2 mb-1.5">
      <span class="text-sm">{getObjIcon(obj.id)}</span>
      <span class="font-mono text-xs text-blue-400 font-semibold">{obj.id}</span>
      <span class="text-[10px] text-zinc-500 truncate flex-1">{QUEST_OBJECTIVES[obj.id] || ""}</span>
      <button onclick={onRemove} class="p-0.5 text-zinc-600 hover:text-red-400 rounded"><X size={11} /></button>
    </div>
    {#if obj.objectiveId !== undefined}
      <div class="mb-1.5"><input type="text" value={obj.objectiveId || ""} onchange={(e) => { obj.objectiveId = (e.target as HTMLInputElement).value; markChanged(); }} class="text-[11px] font-mono w-full px-1.5 py-0.5 bg-zinc-900 border border-zinc-800 rounded" placeholder="objectiveId" /></div>
    {/if}
    <div class="flex flex-wrap gap-1 items-center">
      {#each obj.args as arg, argIdx}
        <input type="text" value={arg} onchange={(e) => { obj.args[argIdx] = (e.target as HTMLInputElement).value; markChanged(); }} class="text-[11px] font-mono w-28 px-1.5 py-0.5 bg-zinc-900 border border-zinc-800 rounded" />
      {/each}
      <button onclick={() => { obj.args.push(""); markChanged(); }} class="text-[10px] text-zinc-600 hover:text-zinc-300 border border-dashed border-zinc-700 rounded px-1.5 py-0.5">+</button>
    </div>
    {#if obj.onCompleteActions !== undefined}
      <div class="mt-1.5"><input type="text" value={obj.onCompleteActions || ""} onchange={(e) => { obj.onCompleteActions = (e.target as HTMLInputElement).value; markChanged(); }} class="text-[10px] font-mono w-full px-1.5 py-0.5 bg-zinc-900 border border-emerald-800/30 rounded text-emerald-400/70" placeholder="onCompleteActions..." /></div>
    {/if}
  </div>
{/snippet}

{#snippet killGatherNode(obj: {validCodes: string[]; demand: number}, type: string, onRemove: () => void)}
  <div class="flex items-center gap-2 bg-zinc-950 border border-zinc-800 rounded-md p-2">
    <span class="text-sm">{type === "kill" ? "⚔️" : "📦"}</span>
    <input type="text" value={obj.validCodes.join(", ")} onchange={(e) => { obj.validCodes = (e.target as HTMLInputElement).value.split(",").map(s => s.trim()); markChanged(); }} class="flex-1 text-xs font-mono" placeholder="code1, code2..." />
    <span class="text-[10px] text-zinc-500">×</span>
    <input type="number" value={obj.demand} onchange={(e) => { const n = parseInt((e.target as HTMLInputElement).value); if (!isNaN(n)) obj.demand = n; markChanged(); }} class="w-12 text-xs text-center" />
    <button onclick={onRemove} class="p-1 text-zinc-500 hover:text-red-400 rounded"><X size={11} /></button>
  </div>
{/snippet}

{#snippet sectionHeader(id: string, icon: any, title: string, count: number, addFn?: () => void, selectFn?: (type: string) => void)}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div onclick={() => toggleSection(id)} class="flex items-center gap-2 px-4 py-2.5 bg-zinc-900/50 hover:bg-zinc-900 cursor-pointer rounded-t-lg border border-zinc-800">
    {#if expandedSections.has(id)}<ChevronDown size={13} class="text-zinc-500" />{:else}<ChevronRight size={13} class="text-zinc-500" />{/if}
    <svelte:component this={icon} size={14} class="text-zinc-400" />
    <span class="text-sm font-semibold text-zinc-100">{title}</span>
    <span class="text-zinc-500 text-xs">({count})</span>
    {#if addFn}
      <button onclick={(e) => { e.stopPropagation(); addFn(); }} class="ml-auto flex items-center gap-1 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 px-2 py-1 rounded text-xs"><Plus size={11} /></button>
    {/if}
    {#if selectFn}
      <select onclick={(e) => e.stopPropagation()} onchange={(e) => { const v = (e.target as HTMLSelectElement).value; if (v) { selectFn(v); (e.target as HTMLSelectElement).value = ""; } }} class="ml-auto bg-zinc-800 border border-zinc-700 text-zinc-300 text-[11px] rounded px-2 py-1">
        <option value="">+ Add...</option>
        {#each QUEST_OBJECTIVE_CODES as code}<option value={code}>{getObjIcon(code)} {code}</option>{/each}
      </select>
    {/if}
  </div>
{/snippet}

{#snippet actionSectionHeader(id: string, icon: any, title: string, count: number, addFn: (type: string) => void, color: string)}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div onclick={() => toggleSection(id)} class="flex items-center gap-2 px-4 py-2.5 bg-zinc-900/50 hover:bg-zinc-900 cursor-pointer rounded-t-lg border border-zinc-800">
    {#if expandedSections.has(id)}<ChevronDown size={13} class="text-zinc-500" />{:else}<ChevronRight size={13} class="text-zinc-500" />{/if}
    <svelte:component this={icon} size={14} style="color: {color}" />
    <span class="text-sm font-semibold text-zinc-100">{title}</span>
    <span class="text-zinc-500 text-xs">({count})</span>
    <select onclick={(e) => e.stopPropagation()} onchange={(e) => { const v = (e.target as HTMLSelectElement).value; if (v) { addFn(v); (e.target as HTMLSelectElement).value = ""; } }} class="ml-auto bg-zinc-800 border border-zinc-700 text-zinc-300 text-[11px] rounded px-2 py-1">
      <option value="">+ Add...</option>
      {#each QUEST_ACTION_CODES as code}<option value={code}>{code}</option>{/each}
    </select>
  </div>
{/snippet}

<div class="h-full flex flex-col">
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">{t("quests.title", lang)}</h2>
    <span class="text-xs text-zinc-500">{quests.length} {t("quests.total", lang)}</span>
    <div class="ml-auto flex items-center gap-2">
      <button onclick={loadQuests} disabled={loading} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-50"><RefreshCw size={13} class={loading ? "animate-spin" : ""} /></button>
      {#if currentQuest}<button onclick={duplicateQuest} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Copy size={13} /></button>{/if}
      {#if unsavedChanges.size > 0}<button onclick={saveCurrentQuest} class="flex items-center gap-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Save size={13} />{t("quests.save", lang)}</button>{/if}
    </div>
  </div>
  {#if saveStatus}<div class="px-6 py-1.5 text-xs text-emerald-400 bg-emerald-500/5 border-b border-emerald-500/20">{saveStatus}</div>{/if}
  {#if error}<div class="mx-6 mt-4 flex items-start gap-2 bg-red-500/10 border border-red-500/30 rounded-md p-3 text-sm text-red-400"><AlertCircle size={16} class="shrink-0 mt-0.5" /><span>{error}</span></div>{/if}

  {#if loading}
    <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm"><RefreshCw size={16} class="animate-spin mr-2" /></div>
  {:else if quests.length > 0}
    <div class="flex-1 flex overflow-hidden">
      <!-- Quest list -->
      <div class="w-64 border-r border-zinc-800 bg-zinc-950 flex flex-col">
        <div class="p-3 border-b border-zinc-800"><div class="relative"><Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-zinc-500" /><input type="text" placeholder={t("common.filter", lang)} bind:value={filter} class="pl-8 pr-3 py-1.5 w-full text-xs" /></div></div>
        <div class="flex-1 overflow-y-auto p-2">
          {#each categories as cat}
            <div class="mb-3">
              <div class="text-[10px] font-semibold text-zinc-500 uppercase tracking-wider px-2 mb-1">{cat}</div>
              {#each filteredQuests.filter(q => q.category === cat) as quest}
                {@const idx = quests.indexOf(quest)}
                <button onclick={() => { selectedQuest = idx; selectedStage = 0; }} class="w-full text-left px-3 py-1.5 rounded text-[11px] font-mono transition-colors {selectedQuest === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'} {unsavedChanges.has(quest.path) ? '!border-emerald-500/40' : ''}">
                  <div class="flex items-center gap-1.5">
                    {#if quest.data.stages?.length}<Layers size={10} class="text-purple-400 shrink-0" />{/if}
                    <span class="truncate">{quest.data.id.split(":").pop()}</span>
                  </div>
                </button>
              {/each}
            </div>
          {/each}
        </div>
      </div>

      <!-- Quest details -->
      <div class="flex-1 overflow-y-auto p-6">
        {#if currentQuest}
          <div class="max-w-4xl space-y-4">
            <!-- Header -->
            <div class="flex items-center gap-3 pb-4 border-b border-zinc-800">
              <ScrollText size={20} class="text-zinc-400" />
              <input type="text" value={currentQuest.data.id} onchange={(e) => updateMeta("id", (e.target as HTMLInputElement).value)} class="text-lg font-semibold text-zinc-100 font-mono bg-transparent border-b border-transparent hover:border-zinc-700 focus:border-blue-500 outline-none flex-1" />
              {#if hasStages}<span class="text-[10px] bg-purple-500/10 text-purple-400 px-2 py-0.5 rounded border border-purple-500/20">{currentQuest.data.stages?.length} stages</span>{/if}
              {#if currentQuest.data.perPlayer}<span class="text-[10px] bg-blue-500/10 text-blue-400 px-2 py-0.5 rounded border border-blue-500/20">per-player</span>{/if}
            </div>

            <!-- Meta -->
            <div class="grid grid-cols-4 gap-3">
              <div><label class="text-[10px] text-zinc-500 uppercase">Cooldown</label><input type="number" value={currentQuest.data.cooldown ?? -1} onchange={(e) => updateMeta("cooldown", (e.target as HTMLInputElement).value)} class="w-full text-xs mt-0.5" /></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Per Player</label><select value={currentQuest.data.perPlayer ? "true" : "false"} onchange={(e) => updateMeta("perPlayer", (e.target as HTMLSelectElement).value)} class="w-full text-xs mt-0.5 bg-zinc-900 border border-zinc-800 rounded px-2 py-1 text-zinc-300"><option value="true">true</option><option value="false">false</option></select></div>
              <div><label class="text-[10px] text-zinc-500 uppercase">Predecessor</label><input type="text" value={currentQuest.data.predecessor || ""} onchange={(e) => updateMeta("predecessor", (e.target as HTMLInputElement).value)} class="w-full text-xs mt-0.5 font-mono" placeholder="none" /></div>
              <div class="flex items-end">
                {#if !hasStages}<button onclick={convertToStaged} class="w-full flex items-center justify-center gap-1.5 bg-purple-500/10 hover:bg-purple-500/20 border border-purple-500/30 text-purple-400 px-2 py-1.5 rounded text-xs transition-colors"><Layers size={12} />{lang === "ru" ? "→ Стадии" : "→ Stages"}</button>{/if}
              </div>
            </div>

            {#if hasStages}
              <!-- STAGED QUEST -->
              <div class="border border-purple-500/20 bg-purple-500/5 rounded-lg p-4">
                <div class="flex items-center gap-2 mb-3">
                  <Layers size={14} class="text-purple-400" />
                  <h4 class="text-sm font-semibold text-purple-300">Stages ({currentQuest.data.stages?.length})</h4>
                  <button onclick={addStage} class="ml-auto flex items-center gap-1 bg-purple-500/10 hover:bg-purple-500/20 border border-purple-500/30 text-purple-400 px-2 py-1 rounded text-xs"><Plus size={11} /></button>
                </div>
                <!-- Stage tabs -->
                <div class="flex items-center gap-1 mb-4 flex-wrap">
                  {#each currentQuest.data.stages || [] as stage, idx}
                    <button onclick={() => selectedStage = idx} class="flex items-center gap-1.5 px-3 py-1.5 rounded text-xs font-medium transition-colors {selectedStage === idx ? 'bg-purple-500/20 text-purple-300 border border-purple-500/30' : 'bg-zinc-900 text-zinc-400 border border-zinc-800 hover:text-zinc-200'}">
                      Stage {idx + 1}
                    </button>
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <span onclick={(e) => { e.stopPropagation(); removeStage(idx); }} class="text-zinc-600 hover:text-red-400 cursor-pointer -ml-1"><X size={10} /></span>
                  {/each}
                </div>

                {#if currentStage}
                  <div class="space-y-3">
                    <div><label class="text-[10px] text-zinc-500 uppercase">Stage Title Lang Key</label><input type="text" value={currentStage.stageTitleLangKey} onchange={(e) => { currentStage.stageTitleLangKey = (e.target as HTMLInputElement).value; markChanged(); }} class="w-full text-xs font-mono mt-0.5" /></div>

                    <!-- Stage Kill Objectives -->
                    {#if currentStage.killObjectives?.length || true}
                      <div class="space-y-1.5">
                        <div class="flex items-center gap-2"><span class="text-[10px] text-zinc-500 uppercase font-semibold">Kill Objectives</span><button onclick={() => addKillObj(currentStage)} class="text-[10px] text-zinc-600 hover:text-emerald-400">+add</button></div>
                        {#each currentStage.killObjectives || [] as obj, idx}
                          {@render killGatherNode(obj, "kill", () => { currentStage.killObjectives?.splice(idx, 1); markChanged(); })}
                        {/each}
                      </div>
                    {/if}

                    <!-- Stage Gather Objectives -->
                    <div class="space-y-1.5">
                      <div class="flex items-center gap-2"><span class="text-[10px] text-zinc-500 uppercase font-semibold">Gather Objectives</span><button onclick={() => addGatherObj(currentStage)} class="text-[10px] text-zinc-600 hover:text-emerald-400">+add</button></div>
                      {#each currentStage.gatherObjectives || [] as obj, idx}
                        {@render killGatherNode(obj, "gather", () => { currentStage.gatherObjectives?.splice(idx, 1); markChanged(); })}
                      {/each}
                    </div>

                    <!-- Stage Action Objectives -->
                    <div class="space-y-1.5">
                      <div class="flex items-center gap-2">
                        <span class="text-[10px] text-zinc-500 uppercase font-semibold">Action Objectives</span>
                        <select onchange={(e) => { const v = (e.target as HTMLSelectElement).value; if (v) { addActionObj(v, currentStage); (e.target as HTMLSelectElement).value = ""; } }} class="text-[10px] bg-zinc-800 border border-zinc-700 text-zinc-400 rounded px-1.5 py-0.5"><option value="">+add</option>{#each QUEST_OBJECTIVE_CODES as c}<option value={c}>{c}</option>{/each}</select>
                      </div>
                      {#each currentStage.actionObjectives || [] as obj, idx}
                        {@render objectiveNode(obj, () => { currentStage.actionObjectives?.splice(idx, 1); markChanged(); })}
                      {/each}
                    </div>

                    <!-- Stage Complete Actions -->
                    <div class="space-y-1.5">
                      <div class="flex items-center gap-2">
                        <span class="text-[10px] text-zinc-500 uppercase font-semibold">On Stage Complete</span>
                        <select onchange={(e) => { const v = (e.target as HTMLSelectElement).value; if (v) { addStageAction(v); (e.target as HTMLSelectElement).value = ""; } }} class="text-[10px] bg-zinc-800 border border-zinc-700 text-zinc-400 rounded px-1.5 py-0.5"><option value="">+add</option>{#each QUEST_ACTION_CODES as c}<option value={c}>{c}</option>{/each}</select>
                      </div>
                      {#each currentStage.onStageCompleteActions || [] as action, idx}
                        {@render actionNode(action, () => { currentStage.onStageCompleteActions?.splice(idx, 1); markChanged(); }, "#a78bfa")}
                      {/each}
                    </div>
                  </div>
                {/if}
              </div>
            {:else}
              <!-- NON-STAGED: Kill/Gather/Action Objectives -->
              <div class="rounded-lg overflow-hidden border border-zinc-800">
                {@render sectionHeader("killObj", Target, `Kill Objectives`, currentQuest.data.killObjectives?.length ?? 0, () => addKillObj())}
                {#if expandedSections.has("killObj") && currentQuest.data.killObjectives?.length}
                  <div class="p-3 space-y-1.5 border-x border-b border-zinc-800 rounded-b-lg">{#each currentQuest.data.killObjectives as obj, idx}{@render killGatherNode(obj, "kill", () => { currentQuest.data.killObjectives?.splice(idx, 1); markChanged(); })}{/each}</div>
                {/if}
              </div>

              <div class="rounded-lg overflow-hidden border border-zinc-800">
                {@render sectionHeader("gatherObj", Gift, `Gather Objectives`, currentQuest.data.gatherObjectives?.length ?? 0, () => addGatherObj())}
                {#if expandedSections.has("gatherObj") && currentQuest.data.gatherObjectives?.length}
                  <div class="p-3 space-y-1.5 border-x border-b border-zinc-800 rounded-b-lg">{#each currentQuest.data.gatherObjectives as obj, idx}{@render killGatherNode(obj, "gather", () => { currentQuest.data.gatherObjectives?.splice(idx, 1); markChanged(); })}{/each}</div>
                {/if}
              </div>

              <div class="rounded-lg overflow-hidden border border-zinc-800">
                {@render sectionHeader("actionObj", Target, `Action Objectives`, currentQuest.data.actionObjectives?.length ?? 0, undefined, (type) => addActionObj(type))}
                {#if expandedSections.has("actionObj") && currentQuest.data.actionObjectives?.length}
                  <div class="p-3 space-y-2 border-x border-b border-zinc-800 rounded-b-lg">{#each currentQuest.data.actionObjectives as obj, idx}{@render objectiveNode(obj, () => { currentQuest.data.actionObjectives?.splice(idx, 1); markChanged(); })}{/each}</div>
                {/if}
              </div>
            {/if}

            <!-- Rewards (always shown) -->
            <div class="rounded-lg overflow-hidden border border-zinc-800">
              {@render sectionHeader("itemRewards", Gift, `Item Rewards`, currentQuest.data.itemRewards?.length ?? 0, addItemReward)}
              {#if expandedSections.has("itemRewards") && currentQuest.data.itemRewards?.length}
                <div class="p-3 space-y-1.5 border-x border-b border-zinc-800 rounded-b-lg">
                  {#each currentQuest.data.itemRewards as reward, idx}
                    <div class="flex items-center gap-2 bg-zinc-950 border border-zinc-800 rounded-md p-2">
                      <input type="text" value={reward.itemCode} onchange={(e) => { reward.itemCode = (e.target as HTMLInputElement).value; markChanged(); }} class="flex-1 text-xs font-mono" />
                      <span class="text-[10px] text-zinc-500">×</span>
                      <input type="number" value={reward.amount} onchange={(e) => { const n = parseInt((e.target as HTMLInputElement).value); if (!isNaN(n)) reward.amount = n; markChanged(); }} class="w-12 text-xs text-center" />
                      <button onclick={() => { currentQuest.data.itemRewards?.splice(idx, 1); markChanged(); }} class="p-1 text-zinc-500 hover:text-red-400 rounded"><X size={11} /></button>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>

            <!-- Random Item Rewards -->
            <div class="rounded-lg overflow-hidden border border-zinc-800">
              {@render sectionHeader("randomRewards", Zap, `Random Rewards`, currentQuest.data.randomItemRewards?.items?.length ?? 0, addRandomRewardItem)}
              {#if expandedSections.has("randomRewards") && currentQuest.data.randomItemRewards}
                <div class="p-3 space-y-2 border-x border-b border-zinc-800 rounded-b-lg">
                  <div class="flex items-center gap-2 mb-2"><span class="text-[10px] text-zinc-500">Select:</span><input type="number" value={currentQuest.data.randomItemRewards.selectAmount} onchange={(e) => { const n = parseInt((e.target as HTMLInputElement).value); if (!isNaN(n) && currentQuest.data.randomItemRewards) currentQuest.data.randomItemRewards.selectAmount = n; markChanged(); }} class="w-12 text-xs text-center" /></div>
                  {#each currentQuest.data.randomItemRewards.items as item, idx}
                    <div class="flex items-center gap-2 bg-zinc-950 border border-zinc-800 rounded-md p-2">
                      <input type="text" value={item.itemCode} onchange={(e) => { item.itemCode = (e.target as HTMLInputElement).value; markChanged(); }} class="flex-1 text-xs font-mono" />
                      <input type="number" value={item.minAmount} onchange={(e) => { const n = parseInt((e.target as HTMLInputElement).value); if (!isNaN(n)) item.minAmount = n; markChanged(); }} class="w-10 text-xs text-center" title="min" />
                      <span class="text-[10px] text-zinc-600">-</span>
                      <input type="number" value={item.maxAmount} onchange={(e) => { const n = parseInt((e.target as HTMLInputElement).value); if (!isNaN(n)) item.maxAmount = n; markChanged(); }} class="w-10 text-xs text-center" title="max" />
                      <button onclick={() => { currentQuest.data.randomItemRewards?.items.splice(idx, 1); markChanged(); }} class="p-1 text-zinc-500 hover:text-red-400 rounded"><X size={11} /></button>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>

            <!-- Action Rewards -->
            <div class="rounded-lg overflow-hidden border border-zinc-800">
              {@render actionSectionHeader("actionRewards", Gift, `Action Rewards`, currentQuest.data.actionRewards?.length ?? 0, (type) => addAction(type, "actionRewards"), "#34d399")}
              {#if expandedSections.has("actionRewards") && currentQuest.data.actionRewards?.length}
                <div class="p-3 space-y-2 border-x border-b border-zinc-800 rounded-b-lg">{#each currentQuest.data.actionRewards as action, idx}{@render actionNode(action, () => { currentQuest.data.actionRewards?.splice(idx, 1); markChanged(); }, "#34d399")}{/each}</div>
              {/if}
            </div>

            <!-- On Accepted -->
            <div class="rounded-lg overflow-hidden border border-zinc-800">
              {@render actionSectionHeader("onAccepted", Play, `On Accepted`, currentQuest.data.onAcceptedActions?.length ?? 0, (type) => addAction(type, "onAcceptedActions"), "#f59e0b")}
              {#if expandedSections.has("onAccepted") && currentQuest.data.onAcceptedActions?.length}
                <div class="p-3 space-y-2 border-x border-b border-zinc-800 rounded-b-lg">{#each currentQuest.data.onAcceptedActions as action, idx}{@render actionNode(action, () => { currentQuest.data.onAcceptedActions?.splice(idx, 1); markChanged(); }, "#f59e0b")}{/each}</div>
              {/if}
            </div>

            <!-- Raw JSON -->
            <details class="border border-zinc-800 rounded-lg"><summary class="px-4 py-2 text-xs text-zinc-400 cursor-pointer hover:text-zinc-200 select-none">Raw JSON</summary><pre class="px-4 pb-4 text-[10px] font-mono text-zinc-500 overflow-auto max-h-96">{JSON.stringify(currentQuest.data, null, 2)}</pre></details>
          </div>
        {:else}
          <div class="h-full flex items-center justify-center text-zinc-500 text-sm">{t("quests.select", lang)}</div>
        {/if}
      </div>
    </div>
  {/if}
</div>
