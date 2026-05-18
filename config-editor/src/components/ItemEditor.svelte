<script lang="ts">
  import { RefreshCw, Save, Search, Copy, Trash2, Plus, AlertCircle, X, Table, LayoutGrid, ChevronRight, ChevronDown, FolderOpen } from "lucide-svelte";
  import { loadItemConfigs, saveJsonFile } from "../lib/fileService";
  import { ATTRIBUTE_NAMES, type ActionItem, type ItemConfigFile } from "../lib/types";
  import { t, type Lang } from "../lib/i18n";

  interface Props {
    modPath: string;
    lang: Lang;
  }

  let { modPath, lang }: Props = $props();

  let items = $state<{ path: string; item: ActionItem }[]>([]);
  let loading = $state(false);
  let error = $state("");
  let filter = $state("");
  let selectedItem = $state<number>(-1);
  let unsavedChanges = $state<Set<string>>(new Set());
  let saveStatus = $state("");
  let newAttrName = $state("");
  let viewMode = $state<"cards" | "table">(localStorage.getItem("alegacy-item-view") as any || "cards");
  let editingCell = $state<{ row: number; col: string } | null>(null);
  let editingValue = $state("");
  let collapsedFolders = $state<Set<string>>(new Set());

  // All known attribute keys for dropdown
  let allKnownAttrs = $derived.by(() => Object.keys(ATTRIBUTE_NAMES));
  let availableAttrs = $derived.by(() => {
    if (!currentItem) return allKnownAttrs;
    const existing = new Set(Object.keys(currentItem.item.attributes));
    return allKnownAttrs.filter((a) => !existing.has(a));
  });

  // Group items by folder
  function getFolderName(path: string): string {
    const parts = path.replace(/\\/g, "/").split("/");
    const configIdx = parts.indexOf("itemconfig");
    if (configIdx >= 0 && configIdx < parts.length - 1) {
      return parts.slice(configIdx + 1, -1).join("/") || "root";
    }
    return "root";
  }

  let folders = $derived.by(() => {
    const map = new Map<string, number[]>();
    for (let i = 0; i < items.length; i++) {
      const folder = getFolderName(items[i].path);
      if (!map.has(folder)) map.set(folder, []);
      map.get(folder)!.push(i);
    }
    return map;
  });

  function toggleFolder(folder: string) {
    if (collapsedFolders.has(folder)) collapsedFolders.delete(folder);
    else collapsedFolders.add(folder);
    collapsedFolders = new Set(collapsedFolders);
  }

  let filteredItems = $derived.by(() => {
    if (!filter) return items;
    const f = filter.toLowerCase();
    return items.filter(({ item }) =>
      item.id.toLowerCase().includes(f) ||
      (item.name && item.name.toLowerCase().includes(f)) ||
      item.itemCode.toLowerCase().includes(f)
    );
  });

  let currentItem = $derived.by(() => {
    if (selectedItem >= 0 && selectedItem < items.length) return items[selectedItem];
    return null;
  });

  async function loadItems() {
    if (!modPath) { error = t("items.notFound", lang); return; }
    loading = true;
    error = "";
    try {
      const configs = await loadItemConfigs(modPath);
      const newItems: { path: string; item: ActionItem }[] = [];
      for (const { path, data } of configs) {
        if (!data.actionItems) continue;
        for (const item of data.actionItems) {
          if (!item.attributes) item.attributes = {};
          if (!item.showAttributes) item.showAttributes = [];
          newItems.push({ path, item });
        }
      }
      items = newItems;
      if (newItems.length > 0 && selectedItem < 0) selectedItem = 0;
      if (newItems.length === 0) error = t("items.notFound", lang);
    } catch (e: any) {
      error = `${e?.message || e}`;
    }
    loading = false;
  }

  function markChanged() {
    if (!currentItem) return;
    unsavedChanges.add(currentItem.path);
    unsavedChanges = new Set(unsavedChanges);
    items = [...items];
  }

  function updateAttr(attr: string, value: string) {
    if (!currentItem) return;
    const num = parseFloat(value);
    if (!isNaN(num)) {
      currentItem.item.attributes[attr] = num;
      markChanged();
    }
  }

  function removeAttr(attr: string) {
    if (!currentItem) return;
    delete currentItem.item.attributes[attr];
    currentItem.item.showAttributes = currentItem.item.showAttributes.filter((a) => a !== attr);
    markChanged();
  }

  function addAttr() {
    if (!currentItem || !newAttrName.trim()) return;
    const name = newAttrName.trim();
    currentItem.item.attributes[name] = 0;
    if (!currentItem.item.showAttributes.includes(name)) {
      currentItem.item.showAttributes.push(name);
    }
    newAttrName = "";
    markChanged();
  }

  function addAttrFromDropdown(attrKey: string) {
    if (!currentItem || !attrKey) return;
    currentItem.item.attributes[attrKey] = 0;
    if (!currentItem.item.showAttributes.includes(attrKey)) {
      currentItem.item.showAttributes.push(attrKey);
    }
    markChanged();
  }

  function createNewItem() {
    if (items.length === 0) return;
    // Use the first item's path as default file
    const targetPath = currentItem?.path || items[0].path;
    const newItem: ActionItem = {
      id: "new-item-" + Date.now().toString(36),
      itemCode: "new-item",
      name: "",
      description: "",
      actions: [],
      attributes: {},
      showAttributes: [],
    };
    items = [...items, { path: targetPath, item: newItem }];
    unsavedChanges.add(targetPath);
    unsavedChanges = new Set(unsavedChanges);
    selectedItem = items.length - 1;
  }

  function updateField(field: "id" | "itemCode" | "name" | "description", value: string) {
    if (!currentItem) return;
    (currentItem.item as any)[field] = value;
    markChanged();
  }

  function duplicateItem() {
    if (!currentItem) return;
    const newId = prompt("ID нового предмета:", currentItem.item.id + "-copy");
    if (!newId) return;
    const newItem: ActionItem = JSON.parse(JSON.stringify(currentItem.item));
    newItem.id = newId;
    newItem.itemCode = newId;
    items = [...items, { path: currentItem.path, item: newItem }];
    unsavedChanges.add(currentItem.path);
    unsavedChanges = new Set(unsavedChanges);
    selectedItem = items.length - 1;
  }

  function deleteItem() {
    if (!currentItem) return;
    if (!confirm(`${t("items.delete", lang)} "${currentItem.item.id}"?`)) return;
    unsavedChanges.add(currentItem.path);
    items = items.filter((_, i) => i !== selectedItem);
    unsavedChanges = new Set(unsavedChanges);
    if (selectedItem >= items.length) selectedItem = items.length - 1;
  }

  async function saveAll() {
    saveStatus = "...";
    try {
      const byPath = new Map<string, ActionItem[]>();
      for (const { path, item } of items) {
        if (!byPath.has(path)) byPath.set(path, []);
        byPath.get(path)!.push(item);
      }
      for (const path of unsavedChanges) {
        const fileItems = byPath.get(path);
        if (fileItems) await saveJsonFile(path, { actionItems: fileItems } as ItemConfigFile);
      }
      unsavedChanges = new Set();
      saveStatus = t("items.saved", lang);
      setTimeout(() => (saveStatus = ""), 3000);
    } catch (e: any) {
      saveStatus = `Error: ${e?.message || e}`;
    }
  }

  function stripHtml(s: string): string {
    return s.replace(/<[^>]*>/g, "");
  }

  function extractColor(s: string): string | null {
    const match = s.match(/<font\s+color="([^"]+)">/i);
    return match ? match[1] : null;
  }

  function wrapWithColor(text: string, color: string): string {
    // Strip existing font tags first
    const clean = text.replace(/<\/?font[^>]*>/gi, "");
    return `<font color="${color}">${clean}</font>`;
  }

  function setFieldColor(field: "name" | "description", color: string) {
    if (!currentItem) return;
    const current = currentItem.item[field] || "";
    (currentItem.item as any)[field] = wrapWithColor(current, color);
    markChanged();
  }

  function getAttrName(attr: string): string {
    const entry = ATTRIBUTE_NAMES[attr];
    if (!entry) return attr;
    return entry[lang] || entry["en"] || attr;
  }

  // Table mode helpers
  let allAttrKeys = $derived.by(() => {
    const keys = new Set<string>();
    for (const { item } of items) {
      for (const k of Object.keys(item.attributes)) keys.add(k);
    }
    return Array.from(keys).sort();
  });

  function setViewMode(mode: "cards" | "table") {
    viewMode = mode;
    localStorage.setItem("alegacy-item-view", mode);
  }

  function startEdit(row: number, col: string, value: string) {
    editingCell = { row, col };
    editingValue = value;
  }

  function commitEdit() {
    if (!editingCell) return;
    const entry = filteredItems[editingCell.row];
    if (!entry) { editingCell = null; return; }
    const col = editingCell.col;
    if (col === "id" || col === "itemCode" || col === "name" || col === "description") {
      (entry.item as any)[col] = editingValue;
    } else {
      // attribute
      const num = parseFloat(editingValue);
      if (!isNaN(num)) {
        entry.item.attributes[col] = num;
      }
    }
    unsavedChanges.add(entry.path);
    unsavedChanges = new Set(unsavedChanges);
    items = [...items];
    editingCell = null;
  }

  function cancelEdit() {
    editingCell = null;
  }

  $effect(() => { if (modPath) loadItems(); });
</script>

<div class="h-full flex flex-col">
  <!-- Toolbar -->
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">{t("items.title", lang)}</h2>
    <span class="text-xs text-zinc-500">{items.length} {t("items.total", lang)}</span>

    <!-- View mode toggle -->
    <div class="flex items-center gap-0.5 bg-zinc-900 border border-zinc-800 rounded-md p-0.5 ml-3">
      <button
        onclick={() => setViewMode("cards")}
        class="flex items-center gap-1.5 px-2.5 py-1 rounded text-xs font-medium transition-colors {viewMode === 'cards' ? 'bg-zinc-700 text-zinc-100' : 'text-zinc-500 hover:text-zinc-300'}"
      >
        <LayoutGrid size={12} />
        {t("items.editMode", lang)}
      </button>
      <button
        onclick={() => setViewMode("table")}
        class="flex items-center gap-1.5 px-2.5 py-1 rounded text-xs font-medium transition-colors {viewMode === 'table' ? 'bg-zinc-700 text-zinc-100' : 'text-zinc-500 hover:text-zinc-300'}"
      >
        <Table size={12} />
        {t("items.tableMode", lang)}
      </button>
    </div>

    <div class="ml-auto flex items-center gap-2">
      <button onclick={createNewItem} class="flex items-center gap-1.5 bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors">
        <Plus size={13} />
        {t("items.newItem", lang)}
      </button>
      <button onclick={loadItems} disabled={loading} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-50">
        <RefreshCw size={13} class={loading ? "animate-spin" : ""} />
        {t("items.refresh", lang)}
      </button>
      {#if unsavedChanges.size > 0}
        <button onclick={saveAll} class="flex items-center gap-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors">
          <Save size={13} />
          {t("items.save", lang)} ({unsavedChanges.size})
        </button>
      {/if}
    </div>
  </div>

  {#if saveStatus}
    <div class="px-6 py-1.5 text-xs text-emerald-400 bg-emerald-500/5 border-b border-emerald-500/20">{saveStatus}</div>
  {/if}
  {#if error}
    <div class="mx-6 mt-3 flex items-start gap-2 bg-red-500/10 border border-red-500/30 rounded-md p-3 text-sm text-red-400">
      <AlertCircle size={16} class="shrink-0 mt-0.5" /><span>{error}</span>
    </div>
  {/if}

  {#if loading}
    <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm">
      <RefreshCw size={16} class="animate-spin mr-2" /> {t("items.loading", lang)}
    </div>
  {:else if items.length > 0}
    {#if viewMode === "table"}
      <!-- TABLE MODE -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="p-3 border-b border-zinc-800">
          <div class="relative w-72">
            <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-zinc-500" />
            <input type="text" placeholder={t("items.search", lang)} bind:value={filter} class="pl-8 pr-3 py-1.5 w-full text-xs" />
          </div>
        </div>
        <div class="flex-1 overflow-auto">
          <table class="w-full text-xs border-collapse">
            <thead class="sticky top-0 z-10 bg-zinc-950">
              <tr class="border-b border-zinc-800">
                <th class="text-left px-3 py-2 text-zinc-500 font-medium whitespace-nowrap">ID</th>
                <th class="text-left px-3 py-2 text-zinc-500 font-medium whitespace-nowrap">Item Code</th>
                <th class="text-left px-3 py-2 text-zinc-500 font-medium whitespace-nowrap">{t("items.name", lang)}</th>
                {#each allAttrKeys as attr}
                  <th class="text-right px-3 py-2 text-zinc-500 font-medium whitespace-nowrap" title={attr}>{getAttrName(attr)}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each filteredItems as entry, rowIdx}
                <tr class="border-b border-zinc-800/50 hover:bg-zinc-900/50 transition-colors {unsavedChanges.has(entry.path) ? 'bg-emerald-500/5' : ''}">
                  <!-- ID -->
                  <td class="px-3 py-1.5 font-mono text-zinc-300 whitespace-nowrap"
                    ondblclick={() => startEdit(rowIdx, "id", entry.item.id)}
                  >
                    {#if editingCell?.row === rowIdx && editingCell?.col === "id"}
                      <input type="text" bind:value={editingValue} onblur={commitEdit} onkeydown={(e) => { if (e.key === "Enter") commitEdit(); if (e.key === "Escape") cancelEdit(); }} class="w-full text-xs font-mono" autofocus />
                    {:else}
                      {entry.item.id}
                    {/if}
                  </td>
                  <!-- Item Code -->
                  <td class="px-3 py-1.5 font-mono text-zinc-400 whitespace-nowrap"
                    ondblclick={() => startEdit(rowIdx, "itemCode", entry.item.itemCode)}
                  >
                    {#if editingCell?.row === rowIdx && editingCell?.col === "itemCode"}
                      <input type="text" bind:value={editingValue} onblur={commitEdit} onkeydown={(e) => { if (e.key === "Enter") commitEdit(); if (e.key === "Escape") cancelEdit(); }} class="w-full text-xs font-mono" autofocus />
                    {:else}
                      {entry.item.itemCode}
                    {/if}
                  </td>
                  <!-- Name -->
                  <td class="px-3 py-1.5 text-zinc-300 max-w-[200px] truncate"
                    ondblclick={() => startEdit(rowIdx, "name", entry.item.name || "")}
                  >
                    {#if editingCell?.row === rowIdx && editingCell?.col === "name"}
                      <input type="text" bind:value={editingValue} onblur={commitEdit} onkeydown={(e) => { if (e.key === "Enter") commitEdit(); if (e.key === "Escape") cancelEdit(); }} class="w-full text-xs" autofocus />
                    {:else}
                      {stripHtml(entry.item.name || "")}
                    {/if}
                  </td>
                  <!-- Attributes -->
                  {#each allAttrKeys as attr}
                    {@const val = entry.item.attributes[attr]}
                    <td
                      class="px-3 py-1.5 text-right font-mono whitespace-nowrap cursor-pointer {val > 0 ? 'text-emerald-400' : val < 0 ? 'text-red-400' : 'text-zinc-600'}"
                      ondblclick={() => startEdit(rowIdx, attr, val !== undefined ? String(val) : "0")}
                    >
                      {#if editingCell?.row === rowIdx && editingCell?.col === attr}
                        <input type="number" step="0.01" bind:value={editingValue} onblur={commitEdit} onkeydown={(e) => { if (e.key === "Enter") commitEdit(); if (e.key === "Escape") cancelEdit(); }} class="w-20 text-xs text-right font-mono" autofocus />
                      {:else}
                        {val !== undefined ? val : "—"}
                      {/if}
                    </td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
        <div class="px-6 py-2 border-t border-zinc-800 text-[10px] text-zinc-500">
          {filteredItems.length} / {items.length} · {t("items.dblclick", lang)}
        </div>
      </div>
    {:else}
      <!-- CARDS MODE -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Item list (left panel) -->
      <div class="w-72 border-r border-zinc-800 bg-zinc-950 flex flex-col">
        <div class="p-3 border-b border-zinc-800">
          <div class="relative">
            <Search size={13} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-zinc-500" />
            <input type="text" placeholder={t("items.search", lang)} bind:value={filter} class="pl-8 pr-3 py-1.5 w-full text-xs" />
          </div>
        </div>
        <div class="flex-1 overflow-y-auto p-2 space-y-0.5">
          {#if filter}
            {#each filteredItems as entry, i}
              {@const idx = items.indexOf(entry)}
              <button
                onclick={() => (selectedItem = idx)}
                class="w-full text-left px-3 py-2 rounded-md text-xs transition-colors {selectedItem === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'} {unsavedChanges.has(entry.path) ? '!border-emerald-500/40' : ''}"
              >
                <div class="font-mono text-[11px] truncate">{entry.item.id}</div>
                <div class="text-[10px] truncate mt-0.5" style="color: {extractColor(entry.item.name || '') || '#52525b'}">{stripHtml(entry.item.name || "")}</div>
              </button>
            {/each}
          {:else}
            {#each [...folders.entries()] as [folder, indices]}
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <div onclick={() => toggleFolder(folder)} class="flex items-center gap-1.5 px-2 py-1.5 cursor-pointer text-zinc-500 hover:text-zinc-300 select-none">
                {#if collapsedFolders.has(folder)}<ChevronRight size={12} />{:else}<ChevronDown size={12} />{/if}
                <FolderOpen size={12} />
                <span class="text-[10px] font-semibold uppercase tracking-wider">{folder}</span>
                <span class="text-[10px] text-zinc-600 ml-auto">{indices.length}</span>
              </div>
              {#if !collapsedFolders.has(folder)}
                {#each indices as idx}
                  {@const entry = items[idx]}
                  <button
                    onclick={() => (selectedItem = idx)}
                    class="w-full text-left px-3 py-2 rounded-md text-xs transition-colors ml-2 {selectedItem === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'} {unsavedChanges.has(entry.path) ? '!border-emerald-500/40' : ''}"
                  >
                    <div class="font-mono text-[11px] truncate">{entry.item.id}</div>
                    <div class="text-[10px] truncate mt-0.5" style="color: {extractColor(entry.item.name || '') || '#52525b'}">{stripHtml(entry.item.name || "")}</div>
                  </button>
                {/each}
              {/if}
            {/each}
          {/if}
        </div>
        <div class="p-2 border-t border-zinc-800 text-[10px] text-zinc-600 text-center">
          {filteredItems.length} / {items.length}
        </div>
      </div>

      <!-- Item detail (right panel) -->
      <div class="flex-1 overflow-y-auto">
        {#if currentItem}
          <div class="p-6 max-w-3xl space-y-6">
            <!-- Header -->
            <div class="flex items-start justify-between">
              <div class="space-y-2 flex-1 min-w-0">
                <div>
                  <label class="text-[10px] text-zinc-500 uppercase tracking-wider">ID</label>
                  <input type="text" value={currentItem.item.id} onchange={(e) => updateField("id", (e.target as HTMLInputElement).value)} class="w-full text-sm font-mono mt-0.5" />
                </div>
                <div>
                  <label class="text-[10px] text-zinc-500 uppercase tracking-wider">Item Code</label>
                  <input type="text" value={currentItem.item.itemCode} onchange={(e) => updateField("itemCode", (e.target as HTMLInputElement).value)} class="w-full text-sm font-mono mt-0.5" />
                </div>
                <div>
                  <label class="text-[10px] text-zinc-500 uppercase tracking-wider">{t("items.name", lang)}</label>
                  <div class="flex items-center gap-2 mt-0.5">
                    <input type="text" value={currentItem.item.name || ""} onchange={(e) => updateField("name", (e.target as HTMLInputElement).value)} class="flex-1 text-sm" />
                    <input
                      type="color"
                      value={extractColor(currentItem.item.name || "") || "#ffffff"}
                      oninput={(e) => setFieldColor("name", (e.target as HTMLInputElement).value)}
                      class="w-8 h-8 rounded border border-zinc-700 cursor-pointer bg-transparent p-0.5"
                      title="Name color"
                    />
                  </div>
                  {#if currentItem.item.name}
                    <div class="mt-1.5 text-sm font-medium px-2 py-1 bg-zinc-900 rounded border border-zinc-800" style="color: {extractColor(currentItem.item.name) || '#e4e4e7'}">
                      {stripHtml(currentItem.item.name)}
                    </div>
                  {/if}
                </div>
                <div>
                  <label class="text-[10px] text-zinc-500 uppercase tracking-wider">Description</label>
                  <div class="flex items-center gap-2 mt-0.5">
                    <input type="text" value={currentItem.item.description || ""} onchange={(e) => updateField("description", (e.target as HTMLInputElement).value)} class="flex-1 text-sm" />
                    <input
                      type="color"
                      value={extractColor(currentItem.item.description || "") || "#9ca3af"}
                      oninput={(e) => setFieldColor("description", (e.target as HTMLInputElement).value)}
                      class="w-8 h-8 rounded border border-zinc-700 cursor-pointer bg-transparent p-0.5"
                      title="Description color"
                    />
                  </div>
                  {#if currentItem.item.description}
                    <div class="mt-1.5 text-xs px-2 py-1 bg-zinc-900 rounded border border-zinc-800 whitespace-pre-wrap" style="color: {extractColor(currentItem.item.description) || '#9ca3af'}">
                      {stripHtml(currentItem.item.description)}
                    </div>
                  {/if}
                </div>
              </div>
              <div class="flex items-center gap-1 ml-4 shrink-0">
                <button onclick={duplicateItem} class="p-2 text-zinc-500 hover:text-zinc-200 hover:bg-zinc-800 rounded-md transition-colors" title={t("items.duplicate", lang)}><Copy size={14} /></button>
                <button onclick={deleteItem} class="p-2 text-zinc-500 hover:text-red-400 hover:bg-red-500/10 rounded-md transition-colors" title={t("items.delete", lang)}><Trash2 size={14} /></button>
              </div>
            </div>

            <!-- Attributes -->
            <div>
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-semibold text-zinc-100">Attributes <span class="text-zinc-500 font-normal">({Object.keys(currentItem.item.attributes).length})</span></h4>
              </div>

              <div class="space-y-2">
                {#each Object.entries(currentItem.item.attributes) as [attr, value]}
                  <div class="flex items-center gap-3 bg-zinc-900 border border-zinc-800 rounded-md px-3 py-2 group">
                    <span class="text-xs text-zinc-400 font-mono w-44 truncate shrink-0" title={attr}>{getAttrName(attr)}</span>
                    <input
                      type="number"
                      step="0.01"
                      {value}
                      onchange={(e) => updateAttr(attr, (e.target as HTMLInputElement).value)}
                      class="flex-1 text-sm text-right {value > 0 ? 'text-emerald-400' : value < 0 ? 'text-red-400' : 'text-zinc-400'}"
                    />
                    <!-- Visual bar -->
                    <div class="w-24 h-1.5 bg-zinc-800 rounded-full overflow-hidden shrink-0">
                      {#if value > 0}
                        <div class="h-full bg-emerald-500/60 rounded-full" style="width: {Math.min(100, Math.abs(value) * 50)}%"></div>
                      {:else if value < 0}
                        <div class="h-full bg-red-500/60 rounded-full" style="width: {Math.min(100, Math.abs(value) * 50)}%"></div>
                      {/if}
                    </div>
                    <button
                      onclick={() => removeAttr(attr)}
                      class="p-1 text-zinc-600 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-all"
                    >
                      <X size={12} />
                    </button>
                  </div>
                {/each}
              </div>

              <!-- Add attribute -->
              <div class="flex items-center gap-2 mt-3">
                <select
                  onchange={(e) => { const v = (e.target as HTMLSelectElement).value; if (v) { addAttrFromDropdown(v); (e.target as HTMLSelectElement).value = ""; } }}
                  class="flex-1 text-xs bg-zinc-900 border border-zinc-800 rounded-md px-2 py-1.5 text-zinc-300"
                >
                  <option value="">+ {t("items.addAttr", lang)}...</option>
                  {#each availableAttrs as attr}
                    <option value={attr}>{getAttrName(attr)} ({attr})</option>
                  {/each}
                </select>
                <input
                  type="text"
                  placeholder="custom..."
                  bind:value={newAttrName}
                  onkeydown={(e) => { if (e.key === "Enter") addAttr(); }}
                  class="w-32 text-xs"
                />
                <button
                  onclick={addAttr}
                  disabled={!newAttrName.trim()}
                  class="flex items-center gap-1 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-2.5 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-30"
                >
                  <Plus size={12} />
                </button>
              </div>
            </div>

            <!-- Show Attributes -->
            <div>
              <h4 class="text-xs font-semibold text-zinc-400 mb-2">showAttributes</h4>
              <div class="flex flex-wrap gap-1.5">
                {#each currentItem.item.showAttributes as attr}
                  <span class="text-[10px] bg-zinc-800 text-zinc-400 px-2 py-0.5 rounded font-mono">{attr}</span>
                {/each}
              </div>
            </div>

            <!-- File path -->
            <div class="text-[10px] text-zinc-600 font-mono border-t border-zinc-800 pt-3">
              {currentItem.path}
            </div>
          </div>
        {:else}
          <div class="h-full flex items-center justify-center text-zinc-500 text-sm">
            {t("items.notFound", lang)}
          </div>
        {/if}
      </div>
    </div>
    {/if}
  {/if}
</div>
