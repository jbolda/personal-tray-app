import React, { useContext } from "react";
import "../view/App.css";
import { MenuItem } from "@tauri-apps/api/menu";
import { SystemTrayContext } from "../context";

export default function Home() {
  const menu = useContext(SystemTrayContext);

  async function handleAddMenuItem() {
    const now = Date.now();
    const text = `Menu Item at ${now.toPrecision(3)}`;
    const nextMenuItem = await MenuItem.new({
      text,
      id: `Boop${now.toPrecision(3)}`,
      action: async () => {
        console.log(`menu "${text}" clicked`);
      },
    });

    await menu.append(nextMenuItem);
  }

  return (
    <div className="App">
      <header className="App-header">
        <p>hi</p>
        <p>
          <button type="button" onClick={() => handleAddMenuItem()}>
            add tray menu item
          </button>
        </p>
      </header>
    </div>
  );
}
