<script lang="ts">
  import { RefreshCw, Save, Plus, Search, X, AlertCircle, Languages, FileText, Trash2 } from "lucide-svelte";
  import { saveJsonFile } from "../lib/fileService";
  import { t, type Lang } from "../lib/i18n";

  interface Props { modPath: string; lang: Lang; }
  let { modPath, lang }: Props = $props();

  interface LangFile { path: string; relativePath: string; locale: string; data: Record<string, string>; }

  let files = $state<LangFile[]>([]);
  let loading = $state(false);
  let error = $state("");
  let selectedFile = $state<number>(-1);
  let filter = $state("");
  let unsavedChanges = $state<Set<string>>(new Set());
  let saveStatus = $state("");
  let newKey = $state("");
  let newValue = $state("");

  let currentFile = $derived.by(() => selectedFile >= 0 && selectedFile < files.length ? files[selectedFile] : null);

  let filteredKeys = $derived.by(() => {
    if (!currentFile) return [];
    const entries = Object.entries(currentFile.data);
    if (!filter) return entries;
    const f = filter.toLowerCase();
    return entries.filter(([k, v]) => k.toLowerCase().includes(f) || v.toLowerCase().includes(f));
  });

  let locales = $derived.by(() => [...new Set(files.map(f => f.locale))].sort());
  let filesByLocale = $derived.by(() => {
    const map = new Map<string, LangFile[]>();
    for (const f of files) {
      if (!map.has(f.locale)) map.set(f.locale, []);
      map.get(f.locale)!.push(f);
    }
    return map;
  });

  async function load() {
    if (!modPath) return;
    loading = true; error = "";
    try {
      const api = window.electronAPI;
      const assetsPath = await api.joinPath(modPath, "assets");
      const dirs = await api.readDir(assetsPath);
      const result: LangFile[] = [];
      for (const d of dirs.entries.filter((e: any) => e.isDirectory)) {
        const langPath = await api.joinPath(assetsPath, d.name, "lang");
        if (!(await api.exists(langPath))) continue;
        const langDirs = await api.readDir(langPath);
        for (const localeDir of langDirs.entries.filter((e: any) => e.isDirectory)) {
          const locale = localeDir.name;
          const localePath = await api.joinPath(langPath, locale);
          const jsonFiles = await api.findJsonFiles(localePath);
          const parsed = await api.readJsonFiles(jsonFiles);
          for (const { path, data, error: err } of parsed) {
            if (err || !data || typeof data !== "object") continue;
            const relativePath = path.replace(/\\/g, "/").split("/lang/")[1] || path.split("\\").pop() || "";
            result.push({ path, relativePath, locale, data: data as Record<string, string> });
          }
        }
      }
      files = result;
      if (files.length > 0 && selectedFile < 0) selectedFile = 0;
      if (files.length === 0) error = lang === "ru" ? "Файлы локализации не найдены" : "No localization files found";
    } catch (e: any) { error = e?.message || e; }
    loading = false;
  }

  function markChanged() {
    if (!currentFile) return;
    unsavedChanges.add(currentFile.path); unsavedChanges = new Set(unsavedChanges);
    files = [...files];
  }

  function updateValue(key: string, value: string) {
    if (!currentFile) return;
    currentFile.data[key] = value;
    markChanged();
  }

  function deleteKey(key: string) {
    if (!currentFile) return;
    delete currentFile.data[key];
    markChanged();
  }

  function addEntry() {
    if (!currentFile || !newKey.trim()) return;
    currentFile.data[newKey.trim()] = newValue;
    newKey = ""; newValue = "";
    markChanged();
  }

  function findKeyInLocale(key: string, targetLocale: string): number {
    return files.findIndex(f => f.locale === targetLocale && key in f.data);
  }

  function jumpToKey(key: string, targetLocale: string) {
    const idx = findKeyInLocale(key, targetLocale);
    if (idx >= 0) { selectedFile = idx; filter = key; }
  }

  async function save() {
    if (!currentFile) return; saveStatus = "...";
    try { await saveJsonFile(currentFile.path, currentFile.data, false); unsavedChanges.delete(currentFile.path); unsavedChanges = new Set(unsavedChanges); saveStatus = t("bosses.saved", lang); setTimeout(() => saveStatus = "", 3000); }
    catch (e: any) { saveStatus = `Error: ${e?.message || e}`; }
  }

  // Expose for other editors to jump here
  export function navigateToKey(key: string) { filter = key; }

  $effect(() => { if (modPath) load(); });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">{t("nav.localization", lang)}</h2>
    <span class="text-xs text-zinc-500">{files.length} files · {locales.join(", ")}</span>
    <div class="ml-auto flex items-center gap-2">
      <button onclick={load} disabled={loading} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-50"><RefreshCw size={13} class={loading ? "animate-spin" : ""} /></button>
      {#if unsavedChanges.size > 0}
        <button onclick={save} class="flex items-center gap-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Save size={13} />{t("quests.save", lang)} ({unsavedChanges.size})</button>
      {/if}
    </div>
  </div>
  {#if saveStatus}<div class="px-6 py-1.5 text-xs text-emerald-400 bg-emerald-500/5 border-b border-emerald-500/20">{saveStatus}</div>{/if}
  {#if error}<div class="mx-6 mt-3 flex items-start gap-2 bg-red-500/10 border border-red-500/30 rounded-md p-3 text-sm text-red-400"><AlertCircle size={16} class="shrink-0 mt-0.5" /><span>{error}</span></div>{/if}

  {#if loading}
    <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm"><RefreshCw size={16} class="animate-spin mr-2" /></div>
  {:else if files.length > 0}
    <div class="flex-1 flex overflow-hidden">
      <!-- File tree -->
      <div class="w-60 border-r border-zinc-800 bg-zinc-950 flex flex-col overflow-y-auto p-2">
        {#each [...filesByLocale.entries()] as [locale, localeFiles]}
          {@const folders = new Map()}
          {#each localeFiles as file}
            {@const parts = file.relativePath.replace(locale + "/", "").split("/")}
            {@const folder = parts.length > 1 ? parts.slice(0, -1).join("/") : ""}
            {@const fileName = parts[parts.length - 1]}
          {/each}
          <div class="mb-3">
            <div class="text-[10px] font-semibold text-zinc-500 uppercase tracking-wider px-2 mb-1 flex items-center gap-1.5">
              <Languages size={11} />{locale.toUpperCase()} <span class="text-zinc-600 font-normal">({localeFiles.length})</span>
            </div>
            {#each localeFiles.sort((a, b) => a.relativePath.localeCompare(b.relativePath)) as file}
              {@const idx = files.indexOf(file)}
              {@const shortName = file.relativePath.replace(locale + "/", "")}
              <button onclick={() => { selectedFile = idx; filter = ""; }} class="w-full flex items-center gap-1.5 px-2.5 py-1.5 rounded text-[11px] transition-colors {selectedFile === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'} {unsavedChanges.has(file.path) ? '!border-emerald-500/40' : ''}">
                <FileText size={11} class="shrink-0" />
                <span class="truncate">{shortName}</span>
                <span class="text-[9px] text-zinc-600 ml-auto shrink-0">{Object.keys(file.data).length}</span>
              </button>
            {/each}
          </div>
        {/each}
      </div>

      <!-- Editor -->
      <div class="flex-1 flex flex-col overflow-hidden">
        {#if currentFile}
          <!-- Search + Add -->
          <div class="p-3 border-b border-zinc-800 flex items-center gap-2">
            <div class="relative flex-1">
              <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-zinc-500" />
              <input type="text" placeholder={t("common.search", lang)} bind:value={filter} class="pl-8 pr-3 py-1.5 w-full text-xs" />
            </div>
            <span class="text-[10px] text-zinc-500 bg-zinc-800 px-2 py-1 rounded">{currentFile.locale.toUpperCase()}</span>
            <!-- Jump to other locale -->
            {#each locales.filter(l => l !== currentFile.locale) as otherLocale}
              <button onclick={() => { if (filter) jumpToKey(filter, otherLocale); else { const otherFiles = filesByLocale.get(otherLocale); if (otherFiles?.length) selectedFile = files.indexOf(otherFiles[0]); } }} class="text-[10px] text-zinc-500 hover:text-blue-400 bg-zinc-800 hover:bg-zinc-700 px-2 py-1 rounded transition-colors">
                → {otherLocale.toUpperCase()}
              </button>
            {/each}
          </div>

          <!-- Key-value list -->
          <div class="flex-1 overflow-y-auto">
            <div class="divide-y divide-zinc-800/50">
              {#each filteredKeys as [key, value]}
                <div class="px-4 py-2.5 hover:bg-zinc-900/30 group">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-[11px] font-mono text-blue-400/80 select-all">{key}</span>
                    <!-- Jump buttons to other locales -->
                    {#each locales.filter(l => l !== currentFile.locale) as otherLocale}
                      {@const otherIdx = findKeyInLocale(key, otherLocale)}
                      <button onclick={() => jumpToKey(key, otherLocale)} class="text-[9px] px-1.5 py-0.5 rounded transition-colors {otherIdx >= 0 ? 'text-emerald-400 bg-emerald-500/10 hover:bg-emerald-500/20' : 'text-red-400 bg-red-500/10 hover:bg-red-500/20'}" title={otherIdx >= 0 ? `Found in ${otherLocale}` : `Missing in ${otherLocale}`}>
                        {otherLocale}
                      </button>
                    {/each}
                    <button onclick={() => deleteKey(key)} class="ml-auto p-0.5 text-zinc-600 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-all rounded"><X size={11} /></button>
                  </div>
                  <textarea
                    value={value}
                    onchange={(e) => updateValue(key, (e.target as HTMLTextAreaElement).value)}
                    class="w-full text-xs text-zinc-300 bg-zinc-900 border border-zinc-800 rounded px-2 py-1.5 resize-y focus:border-blue-500/50 {value.length > 100 ? 'min-h-[80px]' : value.length > 40 || value.includes('\\n') ? 'min-h-[50px]' : 'min-h-[28px]'}"
                    rows={value.length > 200 ? 5 : value.length > 100 || value.includes("\\n") ? 3 : 1}
                  ></textarea>
                </div>
              {/each}
            </div>
          </div>

          <!-- Add new key -->
          <div class="p-3 border-t border-zinc-800 flex items-center gap-2">
            <input type="text" placeholder="key..." bind:value={newKey} onkeydown={(e) => { if (e.key === "Enter") addEntry(); }} class="w-48 text-xs font-mono" />
            <input type="text" placeholder="value..." bind:value={newValue} onkeydown={(e) => { if (e.key === "Enter") addEntry(); }} class="flex-1 text-xs" />
            <button onclick={addEntry} disabled={!newKey.trim()} class="flex items-center gap-1 bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 px-2.5 py-1.5 rounded text-xs font-medium transition-colors disabled:opacity-30"><Plus size={12} /></button>
          </div>

          <!-- Stats -->
          <div class="px-4 py-1.5 border-t border-zinc-800 text-[10px] text-zinc-600 flex items-center gap-4">
            <span>{Object.keys(currentFile.data).length} keys</span>
            <span>{filteredKeys.length} shown</span>
            <span class="font-mono">{currentFile.path.split("\\").slice(-3).join("/")}</span>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
