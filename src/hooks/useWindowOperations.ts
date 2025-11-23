import { useApp } from "../context/AppContext";
import { invoke } from "@tauri-apps/api/core";

interface WindowInfo {
  id: number;
  title: string;
  app_name: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export function useWindowOperations() {
  const { width, height, centerWindow, setLoading, setMessage } = useApp();

  async function checkPermissions(): Promise<boolean> {
    try {
      const result = await invoke<boolean>("check_permissions");
      return result;
    } catch (error) {
      console.error("Failed to check permissions:", error);
      return false;
    }
  }

  async function loadWindows(): Promise<WindowInfo[]> {
    setLoading(true);
    try {
      const result = await invoke<WindowInfo[]>("get_windows");
      setMessage("");
      return result;
    } catch (error) {
      setMessage(`Error: ${error}`);
      return [];
    } finally {
      setLoading(false);
    }
  }

  async function resizeFrontmost(): Promise<void> {
    setLoading(true);
    try {
      await invoke("resize_frontmost_window", {
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

  async function resizeSelected(selectedWindow: number | null): Promise<void> {
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

  return {
    checkPermissions,
    loadWindows,
    resizeFrontmost,
    resizeSelected,
  };
}
