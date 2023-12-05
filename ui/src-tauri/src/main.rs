use std::{sync::Mutex, time::SystemTime};

use aknn::KDTree;
use knn::PointMap;
use loader::{load_pointmap, load_kd_tree};
use types::{TrackPoint, AbstractKNN};

pub mod knn;
pub mod aknn;
pub mod loader;
pub mod types;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

enum KNNType {
    STANDARD(PointMap),
    TREE(KDTree)
}

struct GlobalState(Mutex<KNNType>, Mutex<TrackPoint>, Mutex<u128>);

#[tauri::command]
fn current_state(state: tauri::State<GlobalState>) -> TrackPoint {
    state.1.lock().unwrap().clone()
}

#[tauri::command]
fn change_knn_type(state: tauri::State<GlobalState>, new_type: &str) {
    if new_type == "STANDARD" {
        *state.0.lock().unwrap() = KNNType::STANDARD(load_pointmap().unwrap());
    } else if new_type == "TREE" {
        *state.0.lock().unwrap() = KNNType::TREE(load_kd_tree().unwrap());
    }
}

#[tauri::command]
fn modify_self(state: tauri::State<GlobalState>, x_diff: f64, y_diff: f64) {
    let mut current = state.1.lock().unwrap();
    current.x += x_diff;
    current.y += y_diff
}

#[tauri::command]
fn get_nearest_point(state: tauri::State<GlobalState>) -> Option<TrackPoint> {
    let mut current = state.0.lock().unwrap();
    let tp = state.1.lock().unwrap();
    let mut timing = state.2.lock().unwrap();
    let start = SystemTime::now();
    let out = match &mut *current {
        KNNType::STANDARD(standard) => standard.nearest_neighbors(&tp, 1),
        KNNType::TREE(tree) => tree.nearest_neighbors(&tp, 1),
    };
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    *timing = duration.as_nanos();

    if out.len() > 0 {
        let tp_out = out[0].clone();
        match &mut *current {
            KNNType::STANDARD(standard) => {
                if standard.ignore_size() > 100 {
                    standard.pop_ignore();
                }
                standard.add_ignore(tp_out.id.clone());
            },
            KNNType::TREE(tree) => {
                if tree.ignore_size() > 100 {
                    tree.pop_ignore();
                }
                tree.add_ignore(tp_out.id.clone());
            },
        };
        return Some(out[0].clone())
    };

    None
}

#[tauri::command]
fn get_timing(state: tauri::State<GlobalState>) -> u128 {
    let timing = state.2.lock().unwrap();
    return *timing
}



fn main() -> std::io::Result<()> {
    let state = GlobalState(Mutex::new(KNNType::STANDARD(load_pointmap()?)), Mutex::new(TrackPoint{
        x: 0.1,
        y: 0.1,
        duration_ms: 0,
        explicit: false,
        name: "".into(),
        id: "user".into(),
        artists: Vec::new(),
    }), Mutex::new(0));

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![current_state, change_knn_type, modify_self, get_nearest_point, get_timing])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
