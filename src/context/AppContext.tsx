import { createContext, useContext, useState, type ReactNode } from "react";
import { Store } from "@tauri-apps/plugin-store";

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

export const DEFAULT_PRESETS: Preset[] = [
  { name: "iPhone SE", width: 375, height: 667 },
  { name: "iPhone 14", width: 390, height: 844 },
  { name: "iPad", width: 768, height: 1024 },
  { name: "HD", width: 1280, height: 720 },
  { name: "FHD", width: 1920, height: 1080 },
];

interface AppContextType {
  // Window management
  windows: WindowInfo[];
  setWindows: (windows: WindowInfo[]) => void;
  selectedWindow: number | null;
  setSelectedWindow: (id: number | null) => void;

  // Resize dimensions
  width: number;
  setWidth: (width: number) => void;
  height: number;
  setHeight: (height: number) => void;

  // UI state
  hasPermissions: boolean;
  setHasPermissions: (has: boolean) => void;
  loading: boolean;
  setLoading: (loading: boolean) => void;
  message: string;
  setMessage: (message: string) => void;
  showWindowList: boolean;
  setShowWindowList: (show: boolean) => void;
  showMenu: boolean;
  setShowMenu: (show: boolean) => void;

  // Presets
  presets: Preset[];
  setPresets: (presets: Preset[]) => void;
  showAddPreset: boolean;
  setShowAddPreset: (show: boolean) => void;
  newPresetName: string;
  setNewPresetName: (name: string) => void;

  // Options
  centerWindow: boolean;
  setCenterWindow: (center: boolean) => void;

  // Store
  store: Store | null;
  setStore: (store: Store) => void;
}

const AppContext = createContext<AppContextType | undefined>(undefined);

export function AppProvider({ children }: { children: ReactNode }) {
  const [windows, setWindows] = useState<WindowInfo[]>([]);
  const [selectedWindow, setSelectedWindow] = useState<number | null>(null);
  const [width, setWidth] = useState<number>(800);
  const [height, setHeight] = useState<number>(600);
  const [hasPermissions, setHasPermissions] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [message, setMessage] = useState<string>("");
  const [showWindowList, setShowWindowList] = useState<boolean>(false);
  const [showAddPreset, setShowAddPreset] = useState<boolean>(false);
  const [newPresetName, setNewPresetName] = useState<string>("");
  const [showMenu, setShowMenu] = useState<boolean>(false);
  const [centerWindow, setCenterWindow] = useState<boolean>(true);
  const [store, setStore] = useState<Store | null>(null);
  const [presets, setPresets] = useState<Preset[]>(DEFAULT_PRESETS);

  const value: AppContextType = {
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
    setLoading,
    message,
    setMessage,
    showWindowList,
    setShowWindowList,
    showMenu,
    setShowMenu,
    presets,
    setPresets,
    showAddPreset,
    setShowAddPreset,
    newPresetName,
    setNewPresetName,
    centerWindow,
    setCenterWindow,
    store,
    setStore,
  };

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}

export function useApp() {
  const context = useContext(AppContext);
  if (context === undefined) {
    throw new Error("useApp must be used within AppProvider");
  }
  return context;
}
