import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import CurrentSong from "./screens/CurrentSong";

enum Screen {
  CurrentSong,
  Settings,
  SongList
}

function App() {
  const [screen, setScreen] = useState(Screen.CurrentSong);

  if (screen == Screen.CurrentSong) {
    return <CurrentSong />
  }

  return <div></div>
}

export default App;
