import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Button, Paper, Typography } from "@mui/material";


function CurrentSong() {
  const [currentSong, setCurrentSong] = useState({
    x: 0,
    y: 0,
    duration_ms: 0,
    explicit: false,
    id: "",
    name: "",
    artists: ""
  });

  let next = async () => {
    setCurrentSong(await invoke("get_nearest_point"))
  };
  
  useEffect(() => {
    next();
  }, [])

  return (
    <Paper elevation={2}>
        <img src="https://t4.ftcdn.net/jpg/02/55/17/43/360_F_255174366_ojDuATz84e5h7lIlxh2moUJa9Kpd5wKk.jpg" width="100%"></img>
        <Typography variant="h1">{currentSong.name}</Typography>
        <Typography variant="h3">{currentSong.artists}</Typography>
        <Button>Like</Button>
        <Button>Dislike</Button>
        <Button onClick={() => next()}>Next</Button>
    </Paper>
  );
}

export default CurrentSong;
