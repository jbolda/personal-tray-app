import React, { createContext, useState, useEffect } from "react";
import "../view/App.css";
import Home from "./Home";
import { setupTray } from "../app-api/setupTray";
import { SystemTrayContext } from "../context";
import { setupGlobalShortcuts } from "../app-api/setupShortcuts";

function App() {
  const [tray, setTray] = useState(null);

  useEffect(async () => {
    const initialTray = await setupTray({ tooltip: "personal tray app" });
    setTray(initialTray);

    await setupGlobalShortcuts();
  }, []);

  return (
    <SystemTrayContext.Provider value={tray}>
      <Home />
    </SystemTrayContext.Provider>
  );
}

export default App;
