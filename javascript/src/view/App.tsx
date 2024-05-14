import React, { useState, useEffect } from "react";
import "../view/App.css";
import Home from "./Home";
import { setupTray } from "../app-api/setupTray";
import { SystemTrayContext } from "../context";
import { setupGlobalShortcuts } from "../app-api/setupShortcuts";
import {
  NotificationAPI,
  setupNotifications,
} from "../app-api/setupNotifications";
import { Menu } from "@tauri-apps/api/menu";

function App() {
  const [tray, setTray] = useState<null | Menu>(null);
  const [notifications, setNotifications] = useState<Awaited<NotificationAPI>>({
    send: async () => console.log("notifications not initialized"),
    permissionGranted: false,
  });

  useEffect(() => {
    async function effectUsed() {
      const initialTray = await setupTray({ tooltip: "personal tray app" });
      setTray(initialTray);
      await setupGlobalShortcuts();
    }
    effectUsed();
  }, []);

  useEffect(() => {
    async function effectUsed() {
      if (!notifications.permissionGranted) {
        const notify = await setupNotifications();
        setNotifications(notify);
      }
    }
    effectUsed();
  }, [notifications.permissionGranted]);

  return (
    <SystemTrayContext.Provider value={{ tray, notifications }}>
      <Home />
    </SystemTrayContext.Provider>
  );
}

export default App;
