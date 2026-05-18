<script lang="ts">
  import { FileCode2, Skull, ScrollText, Sword, Info, Target, BarChart3, TrendingUp } from "lucide-svelte";
  import { t, type Lang } from "../lib/i18n";
  import { loadItemConfigs, loadBossEntities, loadBossHuntConfigs, loadQuestConfigs } from "../lib/fileService";
  import { ATTRIBUTE_NAMES } from "../lib/types";

  interface Props { modPath: string; lang: Lang; onSelectFolder: () => void; }
  let { modPath, lang, onSelectFolder }: Props = $props();

  interface Stats {
    items: number; bosses: number; quests: number; bossHunts: number;
    attrDistribution: Record<string, { min: number; max: number; avg: number; count: number }>;
    bossHpRange: { min: number; max: number; avg: number; values: number[] };
    questsByCategory: Record<string, number>;
    itemsByFolder: Record<string, number>;
    topAttributes: [string, number][];
  }

  let stats = $state<Stats | null>(null);
  let loadingStats = $state(false);

  async function loadStats(path: string) {
    loadingStats = true;
    try {
      const [itemConfigs, bossEntities, bossHunts, questConfigs] = await Promise.all([
        loadItemConfigs(path).catch(() => []),
        loadBossEntities(path).catch(() => []),
        loadBossHuntConfigs(path).catch(() => []),
        loadQuestConfigs(path).catch(() => []),
      ]);

      // Items
      const allItems: any[] = [];
      for (const { data } of itemConfigs) {
        if (data.actionItems) allItems.push(...data.actionItems);
      }

      // Attribute distribution
      const attrDist: Record<string, number[]> = {};
      for (const item of allItems) {
        if (!item.attributes) continue;
        for (const [k, v] of Object.entries(item.attributes)) {
          if (!attrDist[k]) attrDist[k] = [];
          attrDist[k].push(v as number);
        }
      }
      const attrDistribution: Stats["attrDistribution"] = {};
      for (const [k, vals] of Object.entries(attrDist)) {
        attrDistribution[k] = {
          min: Math.min(...vals), max: Math.max(...vals),
          avg: vals.reduce((a, b) => a + b, 0) / vals.length, count: vals.length,
        };
      }

      // Boss HP
      const hpValues = bossEntities.map(b => b.data.server?.behaviors?.find((bh: any) => bh.code === "health")?.maxhealth ?? 0).filter(h => h > 0);
      const bossHpRange = hpValues.length > 0 ? {
        min: Math.min(...hpValues), max: Math.max(...hpValues),
        avg: hpValues.reduce((a, b) => a + b, 0) / hpValues.length, values: hpValues.sort((a, b) => a - b),
      } : { min: 0, max: 0, avg: 0, values: [] };

      // Quests by category
      const questsByCategory: Record<string, number> = {};
      for (const q of questConfigs) {
        questsByCategory[q.category] = (questsByCategory[q.category] || 0) + 1;
      }

      // Items by folder
      const itemsByFolder: Record<string, number> = {};
      for (const { path: p } of itemConfigs) {
        const parts = p.replace(/\\/g, "/").split("/");
        const idx = parts.indexOf("itemconfig");
        const folder = idx >= 0 && idx < parts.length - 1 ? parts[idx + 1] : "root";
        for (const item of (itemConfigs.find(c => c.path === p)?.data.actionItems || [])) {
          itemsByFolder[folder] = (itemsByFolder[folder] || 0) + 1;
        }
      }

      // Top attributes by usage
      const topAttributes = Object.entries(attrDist).sort((a, b) => b[1].length - a[1].length).slice(0, 10).map(([k, v]) => [k, v.length] as [string, number]);

      stats = {
        items: allItems.length, bosses: bossEntities.length,
        quests: questConfigs.length, bossHunts: bossHunts.length,
        attrDistribution, bossHpRange, questsByCategory, itemsByFolder, topAttributes,
      };
    } catch { stats = null; }
    loadingStats = false;
  }

  function getAttrName(attr: string): string {
    const entry = ATTRIBUTE_NAMES[attr];
    if (!entry) return attr;
    return entry[lang] || entry["en"] || attr;
  }

  function barWidth(value: number, max: number): number {
    return max > 0 ? Math.max(2, (value / max) * 100) : 0;
  }

  $effect(() => { if (modPath) loadStats(modPath); });
</script>

<div class="h-full overflow-y-auto p-8">
  <div class="max-w-6xl mx-auto space-y-8">
    <!-- Header -->
    <div class="flex items-center gap-4">
      <img src="/logo.png" alt="AQF" class="w-12 h-12 rounded-lg" />
      <div>
        <h1 class="text-2xl font-semibold text-zinc-100 tracking-tight">{t("welcome.title", lang)}</h1>
        <p class="text-sm text-zinc-400">{t("welcome.subtitle", lang)}</p>
      </div>
    </div>

    {#if !modPath}
      <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-6 text-center">
        <p class="text-zinc-400 text-sm mb-4">{t("welcome.selectPrompt", lang)}</p>
        <button onclick={onSelectFolder} class="bg-blue-500/10 hover:bg-blue-500/20 text-blue-400 px-4 py-2 rounded-md text-sm font-medium transition-colors border border-blue-500/30">{t("app.selectFolder", lang)}</button>
      </div>
    {:else if loadingStats}
      <div class="text-zinc-500 text-sm">Loading analytics...</div>
    {:else if stats}
      <!-- Stats Cards -->
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
          <div class="flex items-center gap-3"><Sword size={18} class="text-blue-400" /><div><div class="text-2xl font-bold text-zinc-100">{stats.items}</div><div class="text-[11px] text-zinc-500">{t("welcome.itemConfigs", lang)}</div></div></div>
        </div>
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
          <div class="flex items-center gap-3"><Skull size={18} class="text-red-400" /><div><div class="text-2xl font-bold text-zinc-100">{stats.bosses}</div><div class="text-[11px] text-zinc-500">{t("welcome.bossConfigs", lang)}</div></div></div>
        </div>
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
          <div class="flex items-center gap-3"><ScrollText size={18} class="text-emerald-400" /><div><div class="text-2xl font-bold text-zinc-100">{stats.quests}</div><div class="text-[11px] text-zinc-500">{t("welcome.activeQuests", lang)}</div></div></div>
        </div>
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
          <div class="flex items-center gap-3"><Target size={18} class="text-amber-400" /><div><div class="text-2xl font-bold text-zinc-100">{stats.bossHunts}</div><div class="text-[11px] text-zinc-500">Boss Hunts</div></div></div>
        </div>
      </div>

      <!-- Charts Row -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Boss HP Distribution -->
        {#if stats.bossHpRange.values.length > 0}
          <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-5">
            <div class="flex items-center gap-2 mb-4">
              <BarChart3 size={16} class="text-red-400" />
              <h3 class="text-sm font-semibold text-zinc-200">{lang === "ru" ? "Распределение HP боссов" : "Boss HP Distribution"}</h3>
            </div>
            <div class="flex items-end gap-1 h-32">
              {#each stats.bossHpRange.values as hp, i}
                <div class="flex-1 flex flex-col items-center justify-end h-full">
                  <div class="w-full bg-gradient-to-t from-red-500/60 to-red-400/30 rounded-t" style="height: {barWidth(hp, stats.bossHpRange.max)}%"></div>
                </div>
              {/each}
            </div>
            <div class="flex justify-between text-[10px] text-zinc-500 mt-2">
              <span>Min: {stats.bossHpRange.min}</span>
              <span>Avg: {Math.round(stats.bossHpRange.avg)}</span>
              <span>Max: {stats.bossHpRange.max}</span>
            </div>
          </div>
        {/if}

        <!-- Quests by Category -->
        {#if Object.keys(stats.questsByCategory).length > 0}
          <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-5">
            <div class="flex items-center gap-2 mb-4">
              <ScrollText size={16} class="text-emerald-400" />
              <h3 class="text-sm font-semibold text-zinc-200">{lang === "ru" ? "Квесты по категориям" : "Quests by Category"}</h3>
            </div>
            <div class="space-y-2">
              {#each Object.entries(stats.questsByCategory).sort((a, b) => b[1] - a[1]) as [cat, count]}
                {@const maxCount = Math.max(...Object.values(stats.questsByCategory))}
                <div class="flex items-center gap-3">
                  <span class="text-xs text-zinc-400 w-24 truncate font-mono">{cat}</span>
                  <div class="flex-1 h-5 bg-zinc-800 rounded overflow-hidden">
                    <div class="h-full bg-emerald-500/40 rounded" style="width: {barWidth(count, maxCount)}%"></div>
                  </div>
                  <span class="text-xs text-zinc-500 w-8 text-right">{count}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Attribute Balance Analysis -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Top Attributes -->
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-5">
          <div class="flex items-center gap-2 mb-4">
            <TrendingUp size={16} class="text-blue-400" />
            <h3 class="text-sm font-semibold text-zinc-200">{lang === "ru" ? "Популярность атрибутов" : "Attribute Usage"}</h3>
          </div>
          <div class="space-y-2">
            {#each stats.topAttributes as [attr, count]}
              {@const maxCount = stats.topAttributes[0]?.[1] || 1}
              <div class="flex items-center gap-3">
                <span class="text-xs text-zinc-400 w-32 truncate">{getAttrName(attr)}</span>
                <div class="flex-1 h-4 bg-zinc-800 rounded overflow-hidden">
                  <div class="h-full bg-blue-500/40 rounded" style="width: {barWidth(count, maxCount)}%"></div>
                </div>
                <span class="text-[10px] text-zinc-500 w-6 text-right">{count}</span>
              </div>
            {/each}
          </div>
        </div>

        <!-- Attribute Ranges (Balance View) -->
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-5">
          <div class="flex items-center gap-2 mb-4">
            <BarChart3 size={16} class="text-amber-400" />
            <h3 class="text-sm font-semibold text-zinc-200">{lang === "ru" ? "Диапазоны атрибутов (баланс)" : "Attribute Ranges (Balance)"}</h3>
          </div>
          <div class="space-y-2.5 max-h-64 overflow-y-auto">
            {#each Object.entries(stats.attrDistribution).sort((a, b) => b[1].count - a[1].count).slice(0, 12) as [attr, data]}
              <div class="flex items-center gap-2">
                <span class="text-[10px] text-zinc-400 w-28 truncate">{getAttrName(attr)}</span>
                <div class="flex-1 relative h-3 bg-zinc-800 rounded">
                  <div class="absolute top-0 h-full bg-amber-500/30 rounded" style="left: {Math.max(0, (data.min / Math.max(Math.abs(data.min), Math.abs(data.max), 1)) * 50 + 50)}%; width: {Math.min(100, ((data.max - data.min || 1) / Math.max(Math.abs(data.min), Math.abs(data.max), 1)) * 50)}%"></div>
                  <div class="absolute top-0 h-full w-0.5 bg-amber-400" style="left: {Math.max(0, Math.min(100, (data.avg / Math.max(Math.abs(data.min), Math.abs(data.max), 1)) * 50 + 50))}%"></div>
                </div>
                <span class="text-[9px] text-zinc-600 w-20 text-right font-mono">{data.min.toFixed(2)}..{data.max.toFixed(2)}</span>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Items by Folder -->
      {#if Object.keys(stats.itemsByFolder).length > 1}
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-5">
          <div class="flex items-center gap-2 mb-4">
            <Sword size={16} class="text-blue-400" />
            <h3 class="text-sm font-semibold text-zinc-200">{lang === "ru" ? "Предметы по группам" : "Items by Group"}</h3>
          </div>
          <div class="flex items-end gap-4 h-24">
            {#each Object.entries(stats.itemsByFolder) as [folder, count]}
              {@const maxCount = Math.max(...Object.values(stats.itemsByFolder))}
              <div class="flex-1 flex flex-col items-center">
                <div class="w-full bg-blue-500/40 rounded-t" style="height: {barWidth(count, maxCount)}%"></div>
                <span class="text-[10px] text-zinc-500 mt-1 truncate w-full text-center">{folder}</span>
                <span class="text-[10px] text-zinc-600">{count}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Tips -->
      <div class="border border-zinc-800 bg-zinc-900/30 rounded-lg p-5">
        <div class="flex items-center gap-2 mb-3"><Info size={16} class="text-zinc-400" /><h3 class="text-sm font-medium text-zinc-200">{t("welcome.tips", lang)}</h3></div>
        <ul class="space-y-1.5">
          <li class="flex items-start gap-2 text-sm text-zinc-400"><span class="text-zinc-600 mt-0.5">•</span><span>{t("welcome.tip1", lang)}</span></li>
          <li class="flex items-start gap-2 text-sm text-zinc-400"><span class="text-zinc-600 mt-0.5">•</span><span>{t("welcome.tip2", lang)}</span></li>
          <li class="flex items-start gap-2 text-sm text-zinc-400"><span class="text-zinc-600 mt-0.5">•</span><span>{t("welcome.tip3", lang)}</span></li>
        </ul>
      </div>
    {/if}
  </div>
</div>
