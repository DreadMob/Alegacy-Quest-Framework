<script lang="ts">
  import { RefreshCw, Save, Plus, Search, ChevronRight, ChevronDown, X, AlertCircle, Skull, Heart } from "lucide-svelte";
  import { loadBossEntities, saveJsonFile } from "../lib/fileService";
  import { BOSS_ABILITIES, BOSS_ABILITY_CODES, type BossEntity, type BossBehavior, type BossStage } from "../lib/types";
  import { t, type Lang } from "../lib/i18n";

  interface Props {
    modPath: string;
    lang: Lang;
  }

  let { modPath, lang }: Props = $props();

  let bosses = $state<{ path: string; data: BossEntity }[]>([]);
  let selectedBoss = $state<number>(-1);
  let loading = $state(false);
  let error = $state("");
  let unsavedChanges = $state<Set<string>>(new Set());
  let saveStatus = $state("");
  let expandedAbilities = $state<Set<string>>(new Set());
  let bossFilter = $state("");

  let currentBoss = $derived.by(() => {
    if (selectedBoss >= 0 && selectedBoss < bosses.length) return bosses[selectedBoss];
    return null;
  });

  let bossAbilities = $derived.by(() => {
    if (!currentBoss) return [];
    return (currentBoss.data.server?.behaviors ?? []).filter(
      (b) => b.code.startsWith("boss") || b.code === "emotionstates"
    );
  });

  let filteredBosses = $derived.by(() => {
    if (!bossFilter) return bosses;
    const f = bossFilter.toLowerCase();
    return bosses.filter((b) => b.data.code.toLowerCase().includes(f));
  });

  async function loadBosses() {
    if (!modPath) {
      error = t("bosses.selectFolder", lang);
      return;
    }
    loading = true;
    error = "";
    try {
      bosses = await loadBossEntities(modPath);
      if (bosses.length > 0) selectedBoss = 0;
      else error = t("bosses.notFound", lang);
    } catch (e: any) {
      error = `${e?.message || e}`;
    }
    loading = false;
  }

  function toggleAbility(id: string) {
    if (expandedAbilities.has(id)) expandedAbilities.delete(id);
    else expandedAbilities.add(id);
    expandedAbilities = new Set(expandedAbilities);
  }

  function updateStageValue(abilityIdx: number, stageIdx: number, key: string, value: string) {
    if (!currentBoss) return;
    const abilities = (currentBoss.data.server?.behaviors ?? []).filter(
      (b) => b.code.startsWith("boss") || b.code === "emotionstates"
    );
    const ability = abilities[abilityIdx];
    if (!ability?.stages?.[stageIdx]) return;
    const numVal = parseFloat(value);
    if (!isNaN(numVal) && value.trim() !== "") {
      ability.stages[stageIdx][key] = numVal;
    } else {
      ability.stages[stageIdx][key] = value;
    }
    unsavedChanges.add(currentBoss.path);
    unsavedChanges = new Set(unsavedChanges);
    bosses = [...bosses];
  }

  function addStage(abilityIdx: number) {
    if (!currentBoss) return;
    const abilities = (currentBoss.data.server?.behaviors ?? []).filter(
      (b) => b.code.startsWith("boss") || b.code === "emotionstates"
    );
    const ability = abilities[abilityIdx];
    if (!ability) return;
    if (!ability.stages) ability.stages = [];
    const lastStage = ability.stages[ability.stages.length - 1];
    const newStage: BossStage = lastStage
      ? { ...JSON.parse(JSON.stringify(lastStage)), whenHealthRelBelow: Math.max(0.1, (lastStage.whenHealthRelBelow ?? 1) - 0.2) }
      : { whenHealthRelBelow: 0.5, cooldownSeconds: 10 };
    ability.stages.push(newStage);
    unsavedChanges.add(currentBoss.path);
    unsavedChanges = new Set(unsavedChanges);
    bosses = [...bosses];
  }

  function removeStage(abilityIdx: number, stageIdx: number) {
    if (!currentBoss) return;
    const abilities = (currentBoss.data.server?.behaviors ?? []).filter(
      (b) => b.code.startsWith("boss") || b.code === "emotionstates"
    );
    const ability = abilities[abilityIdx];
    if (!ability?.stages) return;
    ability.stages.splice(stageIdx, 1);
    unsavedChanges.add(currentBoss.path);
    unsavedChanges = new Set(unsavedChanges);
    bosses = [...bosses];
  }

  function addAbility() {
    if (!currentBoss) return;
    const codeList = BOSS_ABILITY_CODES.map((c) => `${c} - ${BOSS_ABILITIES[c]?.[lang] || BOSS_ABILITIES[c]?.["en"] || c}`).join("\n");
    const code = prompt(`${t("bosses.abilityCode", lang)}\n${codeList}`);
    if (!code) return;
    const cleanCode = code.split(" ")[0].trim();
    if (!currentBoss.data.server) currentBoss.data.server = { behaviors: [] };
    currentBoss.data.server.behaviors.push({
      code: cleanCode,
      stages: [{ whenHealthRelBelow: 0.8, cooldownSeconds: 10 }],
    });
    unsavedChanges.add(currentBoss.path);
    unsavedChanges = new Set(unsavedChanges);
    bosses = [...bosses];
  }

  function removeAbility(abilityIdx: number) {
    if (!currentBoss) return;
    if (!confirm(t("bosses.deleteAbility", lang))) return;
    const abilities = (currentBoss.data.server?.behaviors ?? []).filter(
      (b) => b.code.startsWith("boss") || b.code === "emotionstates"
    );
    const target = abilities[abilityIdx];
    const all = currentBoss.data.server!.behaviors;
    const idx = all.indexOf(target);
    if (idx >= 0) {
      all.splice(idx, 1);
      unsavedChanges.add(currentBoss.path);
      unsavedChanges = new Set(unsavedChanges);
      bosses = [...bosses];
    }
  }

  function updateBossHealth(value: string) {
    if (!currentBoss) return;
    const h = currentBoss.data.server?.behaviors.find((b) => b.code === "health");
    if (h) {
      const num = parseFloat(value);
      if (!isNaN(num)) {
        h.currenthealth = num;
        h.maxhealth = num;
        unsavedChanges.add(currentBoss.path);
        unsavedChanges = new Set(unsavedChanges);
        bosses = [...bosses];
      }
    }
  }

  async function saveCurrentBoss() {
    if (!currentBoss) return;
    saveStatus = t("bosses.saving", lang);
    try {
      await saveJsonFile(currentBoss.path, currentBoss.data);
      unsavedChanges.delete(currentBoss.path);
      unsavedChanges = new Set(unsavedChanges);
      saveStatus = t("bosses.saved", lang);
      setTimeout(() => (saveStatus = ""), 3000);
    } catch (e: any) {
      saveStatus = `${t("common.error", lang)}: ${e?.message || e}`;
    }
  }

  function getBossHealth(): number {
    if (!currentBoss) return 0;
    return currentBoss.data.server?.behaviors.find((b) => b.code === "health")?.maxhealth ?? 0;
  }

  function getAbilityDescription(code: string): string {
    const entry = BOSS_ABILITIES[code];
    if (!entry) return code;
    return entry[lang] || entry["en"] || code;
  }

  $effect(() => {
    if (modPath) loadBosses();
  });
</script>

<div class="h-full flex flex-col">
  <!-- Toolbar -->
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">{t("bosses.title", lang)}</h2>
    <span class="text-xs text-zinc-500">{bosses.length} {t("bosses.total", lang)}</span>

    <div class="ml-auto flex items-center gap-2">
      <button
        onclick={loadBosses}
        disabled={loading}
        class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-50"
      >
        <RefreshCw size={13} class={loading ? "animate-spin" : ""} />
        {t("bosses.refresh", lang)}
      </button>

      {#if unsavedChanges.size > 0}
        <button
          onclick={saveCurrentBoss}
          class="flex items-center gap-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"
        >
          <Save size={13} />
          {t("bosses.save", lang)}
        </button>
      {/if}
    </div>
  </div>

  {#if saveStatus}
    <div class="px-6 py-1.5 text-xs text-emerald-400 bg-emerald-500/5 border-b border-emerald-500/20">{saveStatus}</div>
  {/if}

  {#if error}
    <div class="mx-6 mt-4 flex items-start gap-2 bg-red-500/10 border border-red-500/30 rounded-md p-3 text-sm text-red-400">
      <AlertCircle size={16} class="shrink-0 mt-0.5" />
      <span>{error}</span>
    </div>
  {/if}

  {#if loading}
    <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm">
      <RefreshCw size={16} class="animate-spin mr-2" /> {t("bosses.loading", lang)}
    </div>
  {:else if bosses.length > 0}
    <div class="flex-1 flex overflow-hidden">
      <!-- Boss list -->
      <div class="w-64 border-r border-zinc-800 bg-zinc-950 flex flex-col">
        <div class="p-3 border-b border-zinc-800">
          <div class="relative">
            <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-zinc-500" />
            <input
              type="text"
              placeholder={t("common.filter", lang)}
              bind:value={bossFilter}
              class="pl-8 pr-3 py-1.5 w-full text-xs"
            />
          </div>
        </div>
        <div class="flex-1 overflow-y-auto p-2 space-y-0.5">
          {#each filteredBosses as boss}
            {@const idx = bosses.indexOf(boss)}
            <button
              onclick={() => (selectedBoss = idx)}
              class="w-full flex items-center justify-between px-3 py-2 rounded-md text-xs transition-colors {selectedBoss === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'} {unsavedChanges.has(boss.path) ? '!border-emerald-500/40' : ''}"
            >
              <span class="font-mono truncate">{boss.data.code}</span>
              <span class="text-[10px] text-zinc-500 shrink-0 ml-2">
                {boss.data.server?.behaviors?.find((b) => b.code === "health")?.maxhealth ?? "?"}
              </span>
            </button>
          {/each}
        </div>
      </div>

      <!-- Boss details -->
      <div class="flex-1 overflow-y-auto">
        {#if currentBoss}
          <div class="p-6 max-w-4xl">
            <!-- Header -->
            <div class="mb-6 pb-4 border-b border-zinc-800">
              <div class="flex items-center gap-3 mb-3">
                <div class="bg-zinc-900 p-2 rounded-md border border-zinc-800">
                  <Skull size={18} class="text-zinc-300" />
                </div>
                <h3 class="text-lg font-semibold text-zinc-100 font-mono">{currentBoss.data.code}</h3>
                {#if currentBoss.data.class}
                  <span class="text-[10px] font-mono bg-zinc-800 text-zinc-400 px-2 py-0.5 rounded">{currentBoss.data.class}</span>
                {/if}
                {#if currentBoss.data.tags?.length}
                  {#each currentBoss.data.tags as tag}
                    <span class="text-[10px] bg-amber-500/10 text-amber-400 px-2 py-0.5 rounded border border-amber-500/20">{tag}</span>
                  {/each}
                {/if}
              </div>

              <div class="flex items-center gap-4 text-xs">
                <label class="flex items-center gap-2">
                  <Heart size={13} class="text-red-400" />
                  <span class="text-zinc-400">HP:</span>
                  <input
                    type="number"
                    value={getBossHealth()}
                    onchange={(e) => updateBossHealth((e.target as HTMLInputElement).value)}
                    class="w-24 text-xs"
                  />
                </label>
                {#if currentBoss.data.hitboxSize}
                  <span class="text-zinc-500">Hitbox: {currentBoss.data.hitboxSize.x}×{currentBoss.data.hitboxSize.y}</span>
                {/if}
                {#if currentBoss.data.client?.size}
                  <span class="text-zinc-500">Size: {currentBoss.data.client.size}</span>
                {/if}
              </div>

              <!-- HP bar with phase markers -->
              <div class="mt-4">
                <div class="text-[10px] text-zinc-500 uppercase tracking-wider mb-1.5">{t("bosses.phases", lang)}</div>
                <div class="relative h-6 bg-gradient-to-r from-red-500/40 via-amber-500/40 to-emerald-500/40 rounded">
                  {#each bossAbilities as ability}
                    {#if ability.stages}
                      {#each ability.stages as stage}
                        {#if stage.whenHealthRelBelow !== undefined && stage.whenHealthRelBelow < 1}
                          <div
                            class="absolute top-0 bottom-0 w-0.5 bg-zinc-100"
                            style="left: {stage.whenHealthRelBelow * 100}%"
                            title="{ability.code} @ {(stage.whenHealthRelBelow * 100).toFixed(0)}%"
                          ></div>
                        {/if}
                      {/each}
                    {/if}
                  {/each}
                </div>
                <div class="flex justify-between text-[10px] text-zinc-600 mt-0.5">
                  <span>0%</span>
                  <span>50%</span>
                  <span>100%</span>
                </div>
              </div>
            </div>

            <!-- Abilities -->
            <div class="space-y-2">
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-semibold text-zinc-100">{t("bosses.abilities", lang)} <span class="text-zinc-500 font-normal">({bossAbilities.length})</span></h4>
                <button
                  onclick={addAbility}
                  class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"
                >
                  <Plus size={13} />
                  {t("bosses.addAbility", lang)}
                </button>
              </div>

              {#each bossAbilities as ability, abilityIdx}
                <div class="border border-zinc-800 rounded-lg overflow-hidden {expandedAbilities.has(`${abilityIdx}`) ? 'border-zinc-700' : ''}">
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <!-- svelte-ignore a11y_click_events_have_key_events -->
                  <div
                    onclick={() => toggleAbility(`${abilityIdx}`)}
                    class="flex items-center gap-2 px-3 py-2.5 bg-zinc-900/50 hover:bg-zinc-900 cursor-pointer transition-colors"
                  >
                    {#if expandedAbilities.has(`${abilityIdx}`)}
                      <ChevronDown size={13} class="text-zinc-500" />
                    {:else}
                      <ChevronRight size={13} class="text-zinc-500" />
                    {/if}
                    <span class="font-mono text-xs text-blue-400 font-semibold">{ability.code}</span>
                    <span class="text-xs text-zinc-500 flex-1 truncate">{getAbilityDescription(ability.code)}</span>
                    <span class="text-[10px] text-zinc-600">{ability.stages ? `${ability.stages.length} ${t("bosses.stages", lang)}` : t("bosses.noStages", lang)}</span>
                    <button
                      onclick={(e) => { e.stopPropagation(); removeAbility(abilityIdx); }}
                      class="p-1 text-zinc-500 hover:text-red-400 hover:bg-red-500/10 rounded transition-colors"
                      title="Удалить"
                    >
                      <X size={13} />
                    </button>
                  </div>

                  {#if expandedAbilities.has(`${abilityIdx}`)}
                    <div class="p-3 space-y-2 bg-zinc-950/50">
                      {#if ability.stages}
                        {#each ability.stages as stage, stageIdx}
                          <div class="bg-zinc-900 border border-zinc-800 rounded-md p-3">
                            <div class="flex items-center gap-2 mb-2">
                      <span class="text-xs font-medium text-zinc-300">{t("bosses.stage", lang)} {stageIdx + 1}</span>
                              {#if stage.whenHealthRelBelow !== undefined}
                                <span class="text-[10px] bg-amber-500/10 text-amber-400 px-1.5 py-0.5 rounded border border-amber-500/20 font-mono">
                                  HP &lt; {(stage.whenHealthRelBelow * 100).toFixed(0)}%
                                </span>
                              {/if}
                              <button
                                onclick={() => removeStage(abilityIdx, stageIdx)}
                                class="ml-auto p-1 text-zinc-500 hover:text-red-400 hover:bg-red-500/10 rounded transition-colors"
                              >
                                <X size={12} />
                              </button>
                            </div>
                            <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
                              {#each Object.keys(stage) as key}
                                <div class="flex flex-col gap-0.5">
                                  <span class="text-[10px] text-zinc-500 font-mono">{key}</span>
                                  {#if typeof stage[key] === "object"}
                                    <textarea
                                      class="text-[10px] font-mono min-h-[40px] resize-y"
                                      value={JSON.stringify(stage[key], null, 1)}
                                      onchange={(e) => {
                                        try {
                                          const parsed = JSON.parse((e.target as HTMLTextAreaElement).value);
                                          stage[key] = parsed;
                                          if (currentBoss) {
                                            unsavedChanges.add(currentBoss.path);
                                            unsavedChanges = new Set(unsavedChanges);
                                            bosses = [...bosses];
                                          }
                                        } catch {}
                                      }}
                                    ></textarea>
                                  {:else}
                                    <input
                                      type="text"
                                      class="text-xs"
                                      value={stage[key]}
                                      onchange={(e) => updateStageValue(abilityIdx, stageIdx, key, (e.target as HTMLInputElement).value)}
                                    />
                                  {/if}
                                </div>
                              {/each}
                            </div>
                          </div>
                        {/each}
                        <button
                          onclick={() => addStage(abilityIdx)}
                          class="w-full flex items-center justify-center gap-1.5 bg-zinc-900/50 hover:bg-zinc-900 border border-dashed border-zinc-800 hover:border-zinc-700 text-zinc-400 px-3 py-2 rounded-md text-xs transition-colors"
                        >
                          <Plus size={12} />
                          {t("bosses.addStage", lang)}
                        </button>
                      {:else}
                        <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
                          {#each Object.entries(ability).filter(([k]) => k !== "code" && k !== "stages") as [key, value]}
                            <div class="flex flex-col gap-0.5">
                              <span class="text-[10px] text-zinc-500 font-mono">{key}</span>
                              <input
                                type="text"
                                class="text-xs"
                                value={typeof value === "object" ? JSON.stringify(value) : value}
                                disabled
                              />
                            </div>
                          {/each}
                        </div>
                      {/if}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
        {:else}
          <div class="h-full flex items-center justify-center text-zinc-500 text-sm">
            {t("bosses.select", lang)}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
