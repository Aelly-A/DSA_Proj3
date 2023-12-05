import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/shell";
import { FormControlLabel, FormLabel, Grid, IconButton, Paper, Radio, RadioGroup, Typography } from "@mui/material";
import { ArrowForward, ThumbDown, ThumbUp, PlayArrow, Settings } from "@mui/icons-material";

enum Selection {
    Like,
    Dislike,
    None
}

function CurrentSong() {
    const [currentState, setCurrentState] = useState({
        x: 0,
        y: 0,
        duration_ms: 0,
        explicit: false,
        id: "",
        name: "",
        artists: [""]
    });
    const [currentSong, setCurrentSong] = useState({
        x: 0,
        y: 0,
        duration_ms: 0,
        explicit: false,
        id: "",
        name: "",
        artists: [""]
    });
    const [selection, setSelection] = useState(Selection.None);
    const [implementation, setImplementation] = useState("STANDARD");
    const [timing, setTiming] = useState(0);
    const [thumbnail, setThumbnail] = useState("https://t4.ftcdn.net/jpg/02/55/17/43/360_F_255174366_ojDuATz84e5h7lIlxh2moUJa9Kpd5wKk.jpg")

    let generateUrl = (): string => {
        return "https://open.spotify.com/track/" + currentSong.id;
    }

    let next = async () => {
        if (selection == Selection.Like) {
            await invoke("modify_self", { xDiff: (currentSong.x - currentState.x)/2, yDiff: (currentSong.y - currentState.y)/2})
        } else if (selection == Selection.Dislike) {
            await invoke("modify_self", { xDiff: -(currentSong.x - currentState.x)/2, yDiff: -(currentSong.y - currentState.y)/2})
        }
        setSelection(Selection.None);
        setCurrentSong(await invoke("get_nearest_point"));
        setTiming(await invoke("get_timing"));
        setCurrentState(await invoke("current_state"));

        // Get url to image.
        let params = new URLSearchParams();
        params.set("url", generateUrl());
        let res = await fetch("https://embed.spotify.com/oembed?" + params);
        let body = await res.json()
        setThumbnail(body["thumbnail_url"])
    };

    let updateImplementation = async (newInput: string) => {
        setImplementation(newInput);
        console.log(await invoke("change_knn_type", { newType: newInput }))
    }

    useEffect(() => {
        next();
    }, [])

    return (
        <Paper elevation={2}>
            <img src={thumbnail} width="100%"></img>
            <Typography variant="h1" fontWeight="medium">{currentSong.name}</Typography>
            <Typography variant="h3">{currentSong.artists[0] == "" ? "" : JSON.parse(currentSong.artists[0].replace("['", "[\"").replace("']", "\"]")).join(", ")}</Typography>
            <Grid container spacing={4} sx={{ paddingLeft: "35px", paddingTop: "50px" }}>
                <Grid xs={3}>
                    <IconButton onClick={() => setSelection(Selection.Like)}><Settings style={styles.largeIcon} /></IconButton>
                </Grid>
                <Grid xs={2}>
                    <IconButton onClick={() => setSelection(Selection.Like)} color={selection == Selection.Like ? "info" : "default"}><ThumbUp style={styles.largeIcon} /></IconButton>
                </Grid>
                <Grid xs={2}>
                    <IconButton onClick={() => open(generateUrl())}><PlayArrow style={styles.largeIcon} /></IconButton>
                </Grid>
                <Grid xs={3}>
                    <IconButton onClick={() => setSelection(Selection.Dislike)} color={selection == Selection.Dislike ? "info" : "default"}><ThumbDown style={styles.largeIcon} /></IconButton>
                </Grid>
                <Grid xs={2}>
                    <IconButton onClick={() => next()}><ArrowForward style={styles.largeIcon} /></IconButton>
                </Grid>
            </Grid>
            <FormLabel id="radio-buttons-group-label">Implementation</FormLabel>
            <RadioGroup
                aria-labelledby="radio-buttons-group-label"
                value={implementation}
                name="radio-buttons-group"
                onChange={(event) => updateImplementation((event.target as HTMLInputElement).value)}
            >
                <FormControlLabel value="STANDARD" control={<Radio />} label="Vector (O(n))" />
                <FormControlLabel value="TREE" control={<Radio />} label="K-D Tree (O(log(n))" />
            </RadioGroup>
            <Typography>Time for Finding Neighbor: { timing} ns</Typography>
            <Typography>Current Coords: ({currentState.x}, {currentState.y}) </Typography>
        </Paper>
    );
}

const styles = {

    largeIcon: {
        width: 120,
        height: 120,
    },

};


export default CurrentSong;
