use std::collections::{HashMap, BTreeSet};
use rand::Rng;

use crate::types::TrackPoint;

pub struct KNNMap {
    points: HashMap<String, TrackPoint>, // Track ID -> TrackPoint
    start_point: Option<TrackPoint>,
    ignore: Vec<String>
}

struct TPDistanceWrapper {
    dist: f64,
    tp: TrackPoint
}

impl PartialOrd for TPDistanceWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TPDistanceWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for TPDistanceWrapper {}

impl Ord for TPDistanceWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.dist == other.dist {
            std::cmp::Ordering::Equal
        } else if self.dist > other.dist {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

fn distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

impl KNNMap {
    pub fn add_point(&self, track_point: TrackPoint) {
        if self.start_point.is_none() {
            self.start_point = Some(track_point.clone());
        } else if rand::thread_rng().gen_range(0..100) == 25 {
            self.start_point = Some(track_point.clone());
        };
        self.points.insert(track_point.id.clone(), track_point);
    }

    pub fn starting_point(&self) -> Option<TrackPoint> {
        self.start_point.clone()
    }

    pub fn add_ignore(&self, id: String) {
        self.ignore.push(id);
    }

    pub fn ignore_size(&self) -> usize {
        self.ignore.len()
    }

    pub fn pop_ignore(&self) {
        self.ignore.pop();
    }

    pub fn k_nearest_neighbors(&self, start: String, k: usize) -> Vec<TrackPoint> {
        // First let's grab info about the starting point.
        let start = self.points.get(&start);

        if start.is_some() {
            // Unwrap.
            let start = start.unwrap();

            // Create a sorted map. (Inserts are O(log(n)))
            let mut set: BTreeSet<TPDistanceWrapper> = BTreeSet::new();

            for pair in self.points.iter() {
                let (id, tp) = pair;
                let id = id.clone();
                if id == start.id || self.ignore.contains(&id) {
                    continue;
                }

                set.insert(TPDistanceWrapper { dist: distance(tp.x, start.x, tp.y, start.y), tp: tp.clone() });
            }

            // Now grab first k members of sorted set.
            return set.iter().take(k).map(|x| x.tp).collect()
        }

        // Return nothing.
        Vec::new()
    }
}