import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import "./App.css";

interface WindowInfo {
  id: number;
  title: string;
  app_name: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

interface Preset {
  name: string;
  width: number;
  height: number;
}

const DEFAULT_PRESETS: Preset[] = [
  { name: "iPhone SE", width: 375, height: 667 },
  { name: "iPhone 14", width: 390, height: 844 },
  { name: "iPad", width: 768, height: 1024 },
  { name: "HD", width: 1280, height: 720 },
  { name: "FHD", width: 1920, height: 1080 },
];

function App() {
  const [windows, setWindows] = useState<WindowInfo[]>([]);
  const [selectedWindow, setSelectedWindow] = useState<number | null>(null);
  const [width, setWidth] = useState<number>(800);
  const [height, setHeight] = useState<number>(600);
  const [hasPermissions, setHasPermissions] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [message, setMessage] = useState<string>("");
  const [showWindowList, setShowWindowList] = useState<boolean>(false);
  const [presets, setPresets] = useState<Preset[]>(DEFAULT_PRESETS);
  const [showAddPreset, setShowAddPreset] = useState<boolean>(false);
  const [newPresetName, setNewPresetName] = useState<string>("");
  const [showMenu, setShowMenu] = useState<boolean>(false);
  const [centerWindow, setCenterWindow] = useState<boolean>(true);
  const [store, setStore] = useState<Store | null>(null);

  async function loadPresets(store: Store) {
    try {
      const saved = await store.get<Preset[]>("customPresets");

      if (saved && Array.isArray(saved) && saved.length > 0) {
        setPresets([...DEFAULT_PRESETS, ...saved]);
      } else {
        setPresets(DEFAULT_PRESETS);

        // Initialize the store with empty array if it doesn't exist
        if (saved === null || saved === undefined) {
          await store.set("customPresets", []);
          await store.save();
        }
      }
    } catch (error) {
      console.error("Failed to load presets:", error);
      setPresets(DEFAULT_PRESETS);

      // Try to initialize the store
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

  async function addPreset() {
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
    const updated = customPresets.filter((_, i) => i !== customIndex);
    await saveCustomPresets(updated);
    setMessage("✅ Preset deleted!");
    setTimeout(() => setMessage(""), 2000);
  }

  async function exportPresets() {
    try {
      const customPresets = presets.slice(DEFAULT_PRESETS.length);
      const filePath = await save({
        defaultPath: "framefit-presets.json",
        filters: [{
          name: "JSON",
          extensions: ["json"]
        }]
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
    setShowMenu(false);
  }

  async function importPresets() {
    try {
      const input = document.createElement("input");
      input.type = "file";
      input.accept = ".json";
      input.onchange = async (e) => {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (file) {
          const text = await file.text();
          const imported = JSON.parse(text) as Preset[];

          if (Array.isArray(imported) && imported.every(p => p.name && p.width && p.height)) {
            await saveCustomPresets(imported);
            setMessage("✅ Presets imported!");
            setTimeout(() => setMessage(""), 2000);
          } else {
            setMessage("❌ Invalid preset file");
          }
        }
      };
      input.click();
    } catch (error) {
      console.error("Failed to import presets:", error);
      setMessage("❌ Failed to import presets");
    }
    setShowMenu(false);
  }

  async function resetPresets() {
    if (confirm("Reset to default presets? This will delete all custom presets.")) {
      await saveCustomPresets([]);
      setMessage("✅ Presets reset!");
      setTimeout(() => setMessage(""), 2000);
    }
    setShowMenu(false);
  }

  async function checkPermissions() {
    try {
      const result = await invoke<boolean>("check_permissions");
      setHasPermissions(result);
      if (!result) {
        setMessage("⚠️ Accessibility permissions required");
      }
    } catch (error) {
      console.error("Failed to check permissions:", error);
    }
  }

  async function loadWindows() {
    setLoading(true);
    try {
      const result = await invoke<WindowInfo[]>("get_windows");
      setWindows(result);
      setMessage("");
    } catch (error) {
      setMessage(`Error: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  async function resizeFrontmost() {
    setLoading(true);
    try {
      await invoke("resize_frontmost_window", { width, height, center: centerWindow });
      setMessage("✅ Resized!");
      setTimeout(() => setMessage(""), 2000);
    } catch (error) {
      setMessage(`❌ ${error}`);
      setTimeout(() => setMessage(""), 10000);
    } finally {
      setLoading(false);
    }
  }

  async function resizeSelected() {
    if (selectedWindow === null) {
      setMessage("Please select a window first");
      return;
    }

    setLoading(true);
    try {
      await invoke("resize_specific_window", {
        windowId: selectedWindow,
        width,
        height,
        center: centerWindow,
      });
      setMessage("✅ Resized!");
      setTimeout(() => setMessage(""), 2000);
    } catch (error) {
      setMessage(`❌ ${error}`);
      setTimeout(() => setMessage(""), 10000);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    const initialize = async () => {
      const s = await Store.load("presets.json");
      setStore(s);
      await loadPresets(s);
    };
    checkPermissions();
    loadWindows();
    initialize();
  }, []);

  return (
    <div className="app">
      <div className="header">
        <h1>FrameFit</h1>
        <div className="header-actions">
          {!hasPermissions && <div className="badge warning">No Permissions</div>}
          <div className="menu-wrapper">
            <button className="btn-icon" onClick={() => setShowMenu(!showMenu)}>
              ⋮
            </button>
            {showMenu && (
              <div className="menu">
                <button onClick={exportPresets}>Export Presets</button>
                <button onClick={importPresets}>Import Presets</button>
                <button onClick={resetPresets}>Reset to Defaults</button>
              </div>
            )}
          </div>
        </div>
      </div>

      <div className="presets">
        {presets.map((size, index) => (
          <div key={index} className="preset-wrapper">
            <button
              onClick={() => {
                setWidth(size.width);
                setHeight(size.height);
              }}
              className={`preset-btn ${width === size.width && height === size.height ? 'active' : ''}`}
            >
              <div className="preset-name">{size.name}</div>
              <div className="preset-size">{size.width} × {size.height}</div>
            </button>
            {index >= DEFAULT_PRESETS.length && (
              <button
                className="delete-btn"
                onClick={(e) => {
                  e.stopPropagation();
                  deletePreset(index);
                }}
                title="Delete preset"
              >
                ×
              </button>
            )}
          </div>
        ))}
        <button
          onClick={() => setShowAddPreset(!showAddPreset)}
          className="preset-btn add-btn"
        >
          <div className="preset-name">+</div>
          <div className="preset-size">Add</div>
        </button>
      </div>

      {showAddPreset && (
        <div className="add-preset-form">
          <input
            type="text"
            placeholder="Preset name"
            value={newPresetName}
            onChange={(e) => setNewPresetName(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && addPreset()}
            autoFocus
          />
          <div className="form-actions">
            <button onClick={addPreset} className="btn btn-primary btn-sm">
              Save
            </button>
            <button onClick={() => {
              setShowAddPreset(false);
              setNewPresetName("");
            }} className="btn btn-secondary btn-sm">
              Cancel
            </button>
          </div>
        </div>
      )}

      <div className="custom-size">
        <div className="input-wrapper">
          <label>Width</label>
          <input
            type="number"
            value={width}
            onChange={(e) => setWidth(Number(e.target.value))}
            min="100"
            max="5000"
          />
        </div>
        <div className="input-wrapper">
          <label>Height</label>
          <input
            type="number"
            value={height}
            onChange={(e) => setHeight(Number(e.target.value))}
            min="100"
            max="5000"
          />
        </div>
      </div>

      <div className="center-option">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={centerWindow}
            onChange={(e) => setCenterWindow(e.target.checked)}
          />
          <span>Center window after resize</span>
        </label>
      </div>

      <div className="actions">
        <button
          onClick={resizeFrontmost}
          disabled={loading || !hasPermissions}
          className="btn btn-primary"
        >
          Resize Top Window
        </button>
        <button
          onClick={() => {
            setShowWindowList(!showWindowList);
            if (!showWindowList) loadWindows();
          }}
          className="btn btn-secondary"
        >
          {showWindowList ? 'Hide' : 'Select'} Window
        </button>
      </div>

      {showWindowList && (
        <div className="window-list">
          <div className="window-list-header">
            <span>Select a window to resize</span>
            <button onClick={loadWindows} className="btn-icon" disabled={loading}>
              ↻
            </button>
          </div>
          <div className="window-items">
            {windows.map((window) => (
              <div
                key={window.id}
                className={`window-item ${selectedWindow === window.id ? "selected" : ""}`}
                onClick={() => setSelectedWindow(window.id)}
              >
                <div className="window-app">{window.app_name}</div>
                <div className="window-title">{window.title || "(No title)"}</div>
                <div className="window-dims">{window.width} × {window.height}</div>
              </div>
            ))}
          </div>
          <button
            onClick={resizeSelected}
            disabled={loading || !hasPermissions || selectedWindow === null}
            className="btn btn-primary"
          >
            Resize Selected
          </button>
        </div>
      )}

      {message && (
        <div
          className={`toast ${message.includes("❌") ? "error" : message.includes("⚠️") ? "warning" : "success"}`}
          onClick={() => setMessage("")}
        >
          {message}
          <span className="toast-dismiss">×</span>
        </div>
      )}
    </div>
  );
}

export default App;
