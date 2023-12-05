use std::collections::{HashMap, BTreeSet};

use crate::types::{TrackPoint, AbstractKNN, Neighbor};

pub struct PointMap {
    points: HashMap<String, TrackPoint>, // Track ID -> TrackPoint
    ignore: Vec<String>
}

impl PointMap {
    pub fn new() -> Self {
        return Self {
            points: HashMap::new(),
            ignore: Vec::new()
        }
    }
}

impl AbstractKNN for PointMap {
    fn insert(&mut self, point: TrackPoint) {
        self.points.insert(point.id.clone(), point);
    }

    fn size(&self) -> usize {
        return self.points.len();
    }

    // This function is O(nlog(n))
    fn nearest_neighbors(&self, point: &TrackPoint, k: usize) -> Vec<TrackPoint> {
        // First let's grab info about the starting point.
        let start = point;

        // Create a sorted set. (Inserts are O(log(n)))
        let mut set: BTreeSet<Neighbor> = BTreeSet::new();

        for pair in self.points.iter() {
            let (id, tp) = pair;
            let id = id.clone();
            if id == start.id || self.ignore.contains(&id) {
                continue;
            }

            set.insert(Neighbor { distance: distance(tp.x, start.x, tp.y, start.y), point: tp.clone() });
        }

        // Now grab first k members of sorted set.
        return set.iter().take(k).map(|x| x.point.clone()).collect()
    }

    fn add_ignore(&mut self, id: String) {
        self.ignore.push(id);
    }

    fn ignore_size(&self) -> usize {
        self.ignore.len()
    }

    fn pop_ignore(&mut self) {
        self.ignore.pop();
    }
}

fn distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::types::{TrackPoint, AbstractKNN};

    use super::PointMap;

    #[test]
    fn test_map_creation() {
        let mut map = PointMap::new();

        map.insert(TrackPoint{
            x: 12.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test1".into(),
            artists: Vec::new(),
        });

        map.insert(TrackPoint{
            x: 10.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test2".into(),
            artists: Vec::new(),
        });

        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_aknn() {
        let mut map = PointMap::new();

        map.insert(TrackPoint{
            x: 12.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test1".into(),
            artists: Vec::new(),
        });

        map.insert(TrackPoint{
            x: 10.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test2".into(),
            artists: Vec::new(),
        });

        map.insert(TrackPoint{
            x: 24.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test3".into(),
            artists: Vec::new(),
        });

        assert_eq!(map.size(), 3);

        let out = map.nearest_neighbors(&TrackPoint{
            x: 10.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test3".into(),
            artists: Vec::new(),
        }, 2);

        assert_eq!(out.len(), 2);
    }
}