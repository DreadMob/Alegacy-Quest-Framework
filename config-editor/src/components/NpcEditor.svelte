<script lang="ts">
  import { RefreshCw, Save, Plus, X, AlertCircle, Users, ScrollText, MessageSquare, Bot, User, ArrowRight } from "lucide-svelte";
  import { saveJsonFile } from "../lib/fileService";
  import { t, type Lang } from "../lib/i18n";

  interface Props { modPath: string; lang: Lang; }
  let { modPath, lang }: Props = $props();

  interface QuestGiverBehavior { code: string; quests?: string[]; alwaysquests?: string[]; priorityquests?: string[]; rotationpool?: string[]; reputationnpc?: string; singlequestatatime?: boolean; chaincooldowndays?: number; maxavailablequests?: number; bosshuntactiveonly?: boolean; trialactiveonly?: boolean; [key: string]: any; }
  interface NpcEntity { code: string; class?: string; server?: { behaviors: any[] }; client?: { behaviors: any[] }; [key: string]: any; }
  interface TextEntry { value: string; jumpTo?: string; trigger?: string; }
  interface DialogueComponent { code: string; owner: string; type: string; text: TextEntry[]; jumpTo?: string; trigger?: string; }
  interface DialogueFile { components: DialogueComponent[]; }

  let npcs = $state<{ path: string; data: NpcEntity; questGiver: QuestGiverBehavior | null }[]>([]);
  let dialogues = $state<{ path: string; name: string; data: DialogueFile }[]>([]);
  let loading = $state(false);
  let error = $state("");
  let selected = $state<number>(-1);
  let tab = $state<"config" | "dialogue">("config");
  let selectedComp = $state<number>(0);
  let unsavedChanges = $state<Set<string>>(new Set());
  let saveStatus = $state("");

  let current = $derived.by(() => selected >= 0 && selected < npcs.length ? npcs[selected] : null);
  let currentDialogue = $derived.by(() => {
    if (!current) return null;
    return dialogues.find(d => d.name === current.data.code) || null;
  });
  let currentComp = $derived.by(() => currentDialogue && selectedComp >= 0 && selectedComp < currentDialogue.data.components.length ? currentDialogue.data.components[selectedComp] : null);

  async function load() {
    if (!modPath) return;
    loading = true; error = "";
    try {
      const api = window.electronAPI;
      const assetsPath = await api.joinPath(modPath, "assets");
      const dirs = await api.readDir(assetsPath);
      const npcResult: typeof npcs = [];
      const dlgResult: typeof dialogues = [];
      for (const d of dirs.entries.filter((e: any) => e.isDirectory)) {
        // Load NPC entities
        const npcPath = await api.joinPath(assetsPath, d.name, "entities", "npc");
        if (await api.exists(npcPath)) {
          const files = await api.findJsonFiles(npcPath);
          const parsed = await api.readJsonFiles(files);
          for (const { path, data, error: err } of parsed) {
            if (err || !data?.code) continue;
            const qg = data.server?.behaviors?.find((b: any) => b.code === "questgiver") || null;
            npcResult.push({ path, data: data as NpcEntity, questGiver: qg });
          }
        }
        // Load dialogues
        const dlgPath = await api.joinPath(assetsPath, d.name, "config", "dialogue");
        if (await api.exists(dlgPath)) {
          const files = await api.findJsonFiles(dlgPath);
          const parsed = await api.readJsonFiles(files);
          for (const { path, data, error: err } of parsed) {
            if (err || !data?.components) continue;
            const name = path.split("\\").pop()?.replace(".json", "") || "";
            dlgResult.push({ path, name, data: data as DialogueFile });
          }
        }
      }
      npcs = npcResult; dialogues = dlgResult;
      if (npcs.length > 0 && selected < 0) selected = 0;
      if (npcs.length === 0) error = lang === "ru" ? "NPC не найдены" : "No NPCs found";
    } catch (e: any) { error = e?.message || e; }
    loading = false;
  }

  function markChanged(path?: string) {
    const p = path || current?.path;
    if (p) { unsavedChanges.add(p); unsavedChanges = new Set(unsavedChanges); }
    npcs = [...npcs];
  }

  function updateQuestList(field: string, value: string) {
    if (!current?.questGiver) return;
    (current.questGiver as any)[field] = value.split(",").map(s => s.trim()).filter(s => s);
    markChanged();
  }

  function updateQgField(field: string, value: string) {
    if (!current?.questGiver) return;
    if (["singlequestatatime","bosshuntactiveonly","trialactiveonly"].includes(field)) (current.questGiver as any)[field] = value === "true";
    else if (["chaincooldowndays","maxavailablequests"].includes(field)) { const n = parseInt(value); if (!isNaN(n)) (current.questGiver as any)[field] = n; }
    else { if (value) (current.questGiver as any)[field] = value; else delete (current.questGiver as any)[field]; }
    markChanged();
  }

  function updateDialogueComp(field: string, value: string) {
    if (!currentComp || !currentDialogue) return;
    if (value === "") delete (currentComp as any)[field]; else (currentComp as any)[field] = value;
    markChanged(currentDialogue.path);
  }

  function addDialogueComp() {
    if (!currentDialogue) return;
    currentDialogue.data.components.push({ code: "new-node", owner: "npc", type: "talk", text: [{ value: "langkey:text" }] });
    selectedComp = currentDialogue.data.components.length - 1;
    markChanged(currentDialogue.path);
  }

  async function save() {
    saveStatus = "...";
    try {
      for (const path of unsavedChanges) {
        const npc = npcs.find(n => n.path === path);
        if (npc) { await saveJsonFile(npc.path, npc.data); continue; }
        const dlg = dialogues.find(d => d.path === path);
        if (dlg) { await saveJsonFile(dlg.path, dlg.data); }
      }
      unsavedChanges = new Set(); saveStatus = t("bosses.saved", lang); setTimeout(() => saveStatus = "", 3000);
    } catch (e: any) { saveStatus = `Error: ${e?.message || e}`; }
  }

  $effect(() => { if (modPath) load(); });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-3 px-6 py-3 border-b border-zinc-800 bg-zinc-950">
    <h2 class="text-sm font-semibold text-zinc-100">NPCs</h2>
    <span class="text-xs text-zinc-500">{npcs.length}</span>
    <div class="ml-auto flex items-center gap-2">
      <button onclick={load} disabled={loading} class="flex items-center gap-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 px-3 py-1.5 rounded-md text-xs font-medium transition-colors disabled:opacity-50"><RefreshCw size={13} class={loading ? "animate-spin" : ""} /></button>
      {#if unsavedChanges.size > 0}<button onclick={save} class="flex items-center gap-1.5 bg-blue-500/10 hover:bg-blue-500/20 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-md text-xs font-medium transition-colors"><Save size={13} />{t("quests.save", lang)}</button>{/if}
    </div>
  </div>
  {#if saveStatus}<div class="px-6 py-1.5 text-xs text-emerald-400 bg-emerald-500/5 border-b border-emerald-500/20">{saveStatus}</div>{/if}
  {#if error}<div class="mx-6 mt-3 flex items-start gap-2 bg-red-500/10 border border-red-500/30 rounded-md p-3 text-sm text-red-400"><AlertCircle size={16} class="shrink-0 mt-0.5" /><span>{error}</span></div>{/if}

  {#if loading}
    <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm"><RefreshCw size={16} class="animate-spin mr-2" /></div>
  {:else if npcs.length > 0}
    <div class="flex-1 flex overflow-hidden">
      <!-- NPC list -->
      <div class="w-48 border-r border-zinc-800 bg-zinc-950 flex flex-col overflow-y-auto p-2 space-y-0.5">
        {#each npcs as npc, idx}
          <button onclick={() => { selected = idx; selectedComp = 0; }} class="w-full flex items-center gap-2 px-3 py-2 rounded-md text-xs transition-colors {selected === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-transparent'}">
            <Users size={13} class="shrink-0 {npc.questGiver ? 'text-emerald-400' : 'text-zinc-600'}" />
            <span class="truncate font-mono text-[11px]">{npc.data.code}</span>
          </button>
        {/each}
      </div>

      <!-- NPC detail -->
      <div class="flex-1 overflow-y-auto p-6">
        {#if current}
          <div class="max-w-4xl space-y-4">
            <div class="flex items-center gap-3 pb-3 border-b border-zinc-800">
              <Users size={20} class="text-emerald-400" />
              <h3 class="text-lg font-semibold text-zinc-100 font-mono">{current.data.code}</h3>
              {#if current.questGiver?.bosshuntactiveonly}<span class="text-[10px] bg-amber-500/10 text-amber-400 px-2 py-0.5 rounded border border-amber-500/20">Boss Hunt</span>{/if}
              {#if current.questGiver?.trialactiveonly}<span class="text-[10px] bg-purple-500/10 text-purple-400 px-2 py-0.5 rounded border border-purple-500/20">Trial</span>{/if}
            </div>

            <!-- Tabs -->
            <div class="flex items-center gap-1 bg-zinc-900 border border-zinc-800 rounded-md p-0.5 w-fit">
              <button onclick={() => tab = "config"} class="px-3 py-1.5 rounded text-xs font-medium transition-colors {tab === 'config' ? 'bg-zinc-700 text-zinc-100' : 'text-zinc-500 hover:text-zinc-300'}"><ScrollText size={12} class="inline mr-1.5" />Quest Giver</button>
              <button onclick={() => tab = "dialogue"} class="px-3 py-1.5 rounded text-xs font-medium transition-colors {tab === 'dialogue' ? 'bg-zinc-700 text-zinc-100' : 'text-zinc-500 hover:text-zinc-300'}"><MessageSquare size={12} class="inline mr-1.5" />Dialogue {currentDialogue ? `(${currentDialogue.data.components.length})` : ""}</button>
            </div>

            {#if tab === "config" && current.questGiver}
              <!-- Quest Giver Config -->
              <div class="border border-emerald-500/20 bg-emerald-500/5 rounded-lg p-4 space-y-4">
                <div class="grid grid-cols-2 gap-3">
                  <div><label class="text-[10px] text-zinc-500 uppercase">Reputation NPC</label><input type="text" value={current.questGiver.reputationnpc || ""} onchange={(e) => updateQgField("reputationnpc", (e.target as HTMLInputElement).value)} class="w-full text-sm font-mono mt-0.5" placeholder="none" /></div>
                  <div><label class="text-[10px] text-zinc-500 uppercase">Max Available</label><input type="number" value={current.questGiver.maxavailablequests ?? 1} onchange={(e) => updateQgField("maxavailablequests", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
                  <div><label class="text-[10px] text-zinc-500 uppercase">Chain Cooldown (days)</label><input type="number" value={current.questGiver.chaincooldowndays ?? 0} onchange={(e) => updateQgField("chaincooldowndays", (e.target as HTMLInputElement).value)} class="w-full text-sm mt-0.5" /></div>
                  <div><label class="text-[10px] text-zinc-500 uppercase">Single at a Time</label><select value={current.questGiver.singlequestatatime ? "true" : "false"} onchange={(e) => updateQgField("singlequestatatime", (e.target as HTMLSelectElement).value)} class="w-full text-sm mt-0.5 bg-zinc-900 border border-zinc-800 rounded px-2 py-1 text-zinc-300"><option value="true">true</option><option value="false">false</option></select></div>
                </div>
                <div><label class="text-[10px] text-zinc-500 uppercase">Always Quests</label><input type="text" value={(current.questGiver.alwaysquests || []).join(", ")} onchange={(e) => updateQuestList("alwaysquests", (e.target as HTMLInputElement).value)} class="w-full text-xs font-mono mt-0.5" /></div>
                <div><label class="text-[10px] text-zinc-500 uppercase">Priority Quests</label><input type="text" value={(current.questGiver.priorityquests || []).join(", ")} onchange={(e) => updateQuestList("priorityquests", (e.target as HTMLInputElement).value)} class="w-full text-xs font-mono mt-0.5" /></div>
                <div><label class="text-[10px] text-zinc-500 uppercase">Rotation Pool</label><textarea value={(current.questGiver.rotationpool || []).join("\n")} onchange={(e) => { if (current?.questGiver) { current.questGiver.rotationpool = (e.target as HTMLTextAreaElement).value.split("\n").map(s => s.trim()).filter(s => s); markChanged(); } }} class="w-full text-xs font-mono mt-0.5 min-h-[60px] resize-y" placeholder="one quest per line"></textarea></div>
                <div><label class="text-[10px] text-zinc-500 uppercase">Fixed Quests</label><input type="text" value={(current.questGiver.quests || []).join(", ")} onchange={(e) => updateQuestList("quests", (e.target as HTMLInputElement).value)} class="w-full text-xs font-mono mt-0.5" /></div>
              </div>
            {:else if tab === "config" && !current.questGiver}
              <div class="text-sm text-zinc-500 italic">{lang === "ru" ? "Этот NPC не является квестгивером" : "Not a quest giver"}</div>
            {/if}

            {#if tab === "dialogue"}
              {#if currentDialogue}
                <div class="flex gap-4">
                  <!-- Dialogue nodes -->
                  <div class="w-48 space-y-0.5 shrink-0">
                    <div class="flex items-center justify-between mb-2"><span class="text-[10px] text-zinc-500 uppercase font-semibold">Nodes</span><button onclick={addDialogueComp} class="p-1 text-zinc-500 hover:text-emerald-400 rounded"><Plus size={12} /></button></div>
                    {#each currentDialogue.data.components as comp, idx}
                      <button onclick={() => selectedComp = idx} class="w-full flex items-center gap-1.5 px-2 py-1.5 rounded text-[11px] transition-colors {selectedComp === idx ? 'bg-blue-500/10 text-blue-400 border border-blue-500/30' : 'text-zinc-400 hover:bg-zinc-900 border border-transparent'}">
                        {#if comp.owner === "npc"}<Bot size={10} class="text-amber-400 shrink-0" />{:else}<User size={10} class="text-blue-400 shrink-0" />{/if}
                        <span class="truncate font-mono">{comp.code}</span>
                        {#if comp.jumpTo}<ArrowRight size={8} class="text-zinc-600 shrink-0" />{/if}
                      </button>
                    {/each}
                  </div>
                  <!-- Node editor -->
                  {#if currentComp}
                    <div class="flex-1 space-y-3">
                      <div class="grid grid-cols-3 gap-2">
                        <div><label class="text-[10px] text-zinc-500 uppercase">Code</label><input type="text" value={currentComp.code} onchange={(e) => updateDialogueComp("code", (e.target as HTMLInputElement).value)} class="w-full text-xs font-mono mt-0.5" /></div>
                        <div><label class="text-[10px] text-zinc-500 uppercase">Owner</label><select value={currentComp.owner} onchange={(e) => updateDialogueComp("owner", (e.target as HTMLSelectElement).value)} class="w-full text-xs mt-0.5 bg-zinc-900 border border-zinc-800 rounded px-2 py-1 text-zinc-300"><option value="npc">NPC</option><option value="player">Player</option></select></div>
                        <div><label class="text-[10px] text-zinc-500 uppercase">Jump To</label><input type="text" value={currentComp.jumpTo || ""} onchange={(e) => updateDialogueComp("jumpTo", (e.target as HTMLInputElement).value)} class="w-full text-xs font-mono mt-0.5" /></div>
                      </div>
                      <div><label class="text-[10px] text-zinc-500 uppercase">Trigger</label><input type="text" value={currentComp.trigger || ""} onchange={(e) => updateDialogueComp("trigger", (e.target as HTMLInputElement).value)} class="w-full text-xs font-mono mt-0.5" placeholder="openquests, closedialogue..." /></div>
                      <div class="space-y-1.5">
                        <div class="flex items-center justify-between"><span class="text-[10px] text-zinc-500 uppercase font-semibold">Text ({currentComp.text.length})</span><button onclick={() => { currentComp.text.push({ value: "langkey:new" }); markChanged(currentDialogue.path); }} class="text-[10px] text-zinc-600 hover:text-emerald-400">+add</button></div>
                        {#each currentComp.text as entry, idx}
                          <div class="bg-zinc-900 border border-zinc-800 rounded p-2 space-y-1">
                            <div class="flex gap-1.5"><input type="text" value={entry.value} onchange={(e) => { entry.value = (e.target as HTMLInputElement).value; markChanged(currentDialogue.path); }} class="flex-1 text-xs font-mono" /><button onclick={() => { currentComp.text.splice(idx, 1); markChanged(currentDialogue.path); }} class="text-zinc-600 hover:text-red-400"><X size={10} /></button></div>
                            <div class="flex gap-1.5"><input type="text" value={entry.jumpTo || ""} onchange={(e) => { if ((e.target as HTMLInputElement).value) entry.jumpTo = (e.target as HTMLInputElement).value; else delete entry.jumpTo; markChanged(currentDialogue.path); }} class="flex-1 text-[10px] font-mono text-zinc-500" placeholder="jumpTo" /><input type="text" value={entry.trigger || ""} onchange={(e) => { if ((e.target as HTMLInputElement).value) entry.trigger = (e.target as HTMLInputElement).value; else delete entry.trigger; markChanged(currentDialogue.path); }} class="flex-1 text-[10px] font-mono text-zinc-500" placeholder="trigger" /></div>
                          </div>
                        {/each}
                      </div>
                    </div>
                  {/if}
                </div>
              {:else}
                <div class="text-sm text-zinc-500 italic">{lang === "ru" ? "Диалог не найден для этого NPC" : "No dialogue found for this NPC"}</div>
              {/if}
            {/if}

            <div class="text-[10px] text-zinc-600 font-mono border-t border-zinc-800 pt-3">{current.path}</div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
