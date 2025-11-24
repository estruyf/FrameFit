import { useEffect } from "react";
import { Store } from "@tauri-apps/plugin-store";
import { AppProvider, useApp, DEFAULT_PRESETS } from "./context/AppContext";
import { usePresetManagement } from "./hooks/usePresetManagement";
import { useWindowOperations } from "./hooks/useWindowOperations";
import "./App.css";

function AppContent() {
  const {
    windows,
    setWindows,
    selectedWindow,
    setSelectedWindow,
    width,
    setWidth,
    height,
    setHeight,
    hasPermissions,
    setHasPermissions,
    loading,
    message,
    setMessage,
    showWindowList,
    setShowWindowList,
    presets,
    showAddPreset,
    setShowAddPreset,
    newPresetName,
    setNewPresetName,
    showMenu,
    setShowMenu,
    centerWindow,
    setCenterWindow,
    setStore: setContextStore,
  } = useApp();

  const { loadPresets, addPreset, deletePreset, exportPresets, importPresetsHandler, resetPresets } = usePresetManagement();
  const { checkPermissions, loadWindows, resizeFrontmost, resizeSelected } = useWindowOperations();

  useEffect(() => {
    async function initialize() {
      const s = await Store.load("presets.json");
      setContextStore(s);
      await loadPresets();

      const hasPerms = await checkPermissions();
      setHasPermissions(hasPerms);
      if (!hasPerms) {
        setMessage("⚠️ Accessibility permissions required");
      }

      const windowsList = await loadWindows();
      setWindows(windowsList);
    }

    initialize();
  }, []);

  return (
    <div className="app">
      {!hasPermissions && (
        <div className="permissions-modal-overlay">
          <div className="permissions-modal">
            <h2>🔒 Accessibility Permissions Required</h2>
            <p>FrameFit needs accessibility permissions to resize windows.</p>

            <div className="instructions">
              <h3>How to enable:</h3>
              <ol>
                <li>Open <strong>System Settings</strong></li>
                <li>Go to <strong>Privacy & Security</strong></li>
                <li>Select <strong>Accessibility</strong></li>
                <li>Click the <strong>+</strong> button</li>
                <li>Find and select <strong>FrameFit</strong></li>
              </ol>
            </div>

            <p className="note">After enabling, you may need to restart the app.</p>
          </div>
        </div>
      )}

      <div className="header title__bar">
        <h1 className="app__title">FrameFit</h1>
        <div className="tile__bar">
          <div className="tile">
            <span className="tile__label">Size</span>
            <span className="tile__value">{width} × {height}</span>
          </div>
          <div className="tile">
            <span className="tile__label">Windows</span>
            <span className="tile__value">{windows.length}</span>
          </div>
          <div className="tile">
            <span className="tile__label">Center</span>
            <span className="tile__value">{centerWindow ? '✓' : '✗'}</span>
          </div>
        </div>

        <div className="header-actions">
          {!hasPermissions && <div className="badge warning">No Permissions</div>}
          <div className="menu-wrapper">
            <button className="btn-icon" onClick={() => setShowMenu(!showMenu)}>
              ⋮
            </button>
            {showMenu && (
              <div className="menu">
                <button onClick={() => { exportPresets(); setShowMenu(false); }}>Export Presets</button>
                <button onClick={() => { importPresetsHandler(); setShowMenu(false); }}>Import Presets</button>
                <button onClick={() => { resetPresets(); setShowMenu(false); }}>Reset to Defaults</button>
              </div>
            )}
          </div>
        </div>
      </div>

      <div className="wrapper">
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
              onKeyDown={(e) => e.key === 'Enter' && addPreset(width, height)}
              autoFocus
            />
            <div className="form-actions">
              <button onClick={() => addPreset(width, height)} className="btn btn-primary btn-sm">
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
            onClick={() => resizeFrontmost()}
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
                  {window.title && <div className="window-title">{window.title}</div>}
                  <div className="window-dims">{window.width} × {window.height}</div>
                </div>
              ))}
            </div>
            <button
              onClick={() => resizeSelected(selectedWindow)}
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
    </div>
  );
}

export default function App() {
  return (
    <AppProvider>
      <AppContent />
    </AppProvider>
  );
}
