import { useApp } from "../context/AppContext";
import { DEFAULT_PRESETS } from "../context/AppContext";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import { save } from "@tauri-apps/plugin-dialog";

interface Preset {
  name: string;
  width: number;
  height: number;
}

export function usePresetManagement() {
  const {
    store,
    presets,
    setPresets,
    newPresetName,
    setNewPresetName,
    setShowAddPreset,
    setMessage,
  } = useApp();

  async function loadPresets() {
    if (!store) return;

    try {
      const saved = await store.get<Preset[]>("customPresets");

      if (saved && Array.isArray(saved) && saved.length > 0) {
        setPresets([...DEFAULT_PRESETS, ...saved]);
      } else {
        setPresets(DEFAULT_PRESETS);

        if (saved === null || saved === undefined) {
          await store.set("customPresets", []);
          await store.save();
        }
      }
    } catch (error) {
      console.error("Failed to load presets:", error);
      setPresets(DEFAULT_PRESETS);

      try {
        await store.set("customPresets", []);
        await store.save();
      } catch (initError) {
        console.error("Failed to initialize store:", initError);
      }
    }
  }

  async function saveCustomPresets(customPresets: Preset[]) {
    if (!store) {
      console.error("Store not initialized");
      return;
    }
    try {
      await store.set("customPresets", customPresets);
      await store.save();
      setPresets([...DEFAULT_PRESETS, ...customPresets]);
    } catch (error) {
      console.error("Failed to save presets:", error);
      setMessage("❌ Failed to save presets");
    }
  }

  async function addPreset(width: number, height: number) {
    if (!newPresetName.trim()) {
      setMessage("Please enter a preset name");
      return;
    }

    const newPreset: Preset = {
      name: newPresetName.trim(),
      width,
      height,
    };

    const customPresets = presets.slice(DEFAULT_PRESETS.length);
    const updated = [...customPresets, newPreset];

    await saveCustomPresets(updated);
    setNewPresetName("");
    setShowAddPreset(false);
    setMessage("✅ Preset saved!");
    setTimeout(() => setMessage(""), 2000);
  }

  async function deletePreset(index: number) {
    if (index < DEFAULT_PRESETS.length) {
      return;
    }
    const customPresets = presets.slice(DEFAULT_PRESETS.length);
    const customIndex = index - DEFAULT_PRESETS.length;
    const updated = customPresets.filter(
      (_: Preset, i: number) => i !== customIndex
    );
    await saveCustomPresets(updated);
    setMessage("✅ Preset deleted!");
    setTimeout(() => setMessage(""), 2000);
  }

  async function exportPresets() {
    try {
      const customPresets = presets.slice(DEFAULT_PRESETS.length);
      const filePath = await save({
        defaultPath: "framefit-presets.json",
        filters: [
          {
            name: "JSON",
            extensions: ["json"],
          },
        ],
      });

      if (filePath) {
        await writeTextFile(filePath, JSON.stringify(customPresets, null, 2));
        setMessage("✅ Presets exported!");
        setTimeout(() => setMessage(""), 2000);
      }
    } catch (error) {
      console.error("Failed to export presets:", error);
      setMessage("❌ Failed to export presets");
    }
  }

  async function importPresets(file: File) {
    try {
      const text = await file.text();
      const imported = JSON.parse(text) as Preset[];

      if (
        Array.isArray(imported) &&
        imported.every((p) => p.name && p.width && p.height)
      ) {
        await saveCustomPresets(imported);
        setMessage("✅ Presets imported!");
        setTimeout(() => setMessage(""), 2000);
      } else {
        setMessage("❌ Invalid preset file");
      }
    } catch (error) {
      console.error("Failed to import presets:", error);
      setMessage("❌ Failed to import presets");
    }
  }

  async function resetPresets() {
    if (
      confirm("Reset to default presets? This will delete all custom presets.")
    ) {
      await saveCustomPresets([]);
      setMessage("✅ Presets reset!");
      setTimeout(() => setMessage(""), 2000);
    }
  }

  function importPresetsHandler() {
    try {
      const input = document.createElement("input");
      input.type = "file";
      input.accept = ".json";
      input.onchange = async (e) => {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (file) {
          await importPresets(file);
        }
      };
      input.click();
    } catch (error) {
      console.error("Failed to import presets:", error);
      setMessage("❌ Failed to import presets");
    }
  }

  return {
    loadPresets,
    saveCustomPresets,
    addPreset,
    deletePreset,
    exportPresets,
    importPresetsHandler,
    resetPresets,
  };
}
