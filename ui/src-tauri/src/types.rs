use std::{cmp::Ordering, sync::Arc};

/// The Abstract definition for all of the KNN data structures.
pub trait AbstractKNN {
    /// Inserts a `TrackPoint` into the underlying data structure. Currently there is
    /// no way to delete this so keep that in mind.
    fn insert(&mut self, point: TrackPoint);

    /// The number of `TrackPoint`s within the underlying data structure. (How many inserts?)
    fn size(&self) -> usize;

    /// Determine the K nearest `TrackPoint`'s to the given TrackPoint.
    fn nearest_neighbors(&self, point: &TrackPoint, k: usize) -> Vec<TrackPoint>;

    // Add a TrackPoint ID to ignore.
    fn add_ignore(&mut self, id: String);

    // The amount of TrackPoints being ignored.
    fn ignore_size(&self) -> usize;

    // Remove the oldest TrackPoint being ignored.
    fn pop_ignore(&mut self);
}

// A TrackPoint is a point on the 2D map, this reperesents a single "Track" within Spotify.
// Be sure to read the documentation for each element!
#[derive(Clone, serde::Serialize)]
pub struct TrackPoint {
    pub x: f64, // The X dimension represents "groove", check out the "determineGroove" function in loader.rs.
    pub y: f64, // The Y dimension represents "style", check out the "determineStyle" function in loader.rs.
    pub duration_ms: u64, // This is the duration of the track.
    pub explicit: bool, // Is the track explicit?
    pub id: String, // Spotify UUID for the track.
    pub name: String, // Name of the track
    pub artists: Vec<String> // List of artists.
}

impl TrackPoint {
    /// Converts the dimension coefficient (cd) into its corresponding dimension.
    /// `0` is x, `1` is y.
    pub fn cd_to_float(&self, cd: usize) -> f64 {
        if cd == 0 {
            return self.x
        } else {
            return self.y
        }
    }
}

/// Implements all of the information needed about a neighbor.
pub struct Neighbor {
    pub distance: f64, // The euclidean distance to the given TrackPoint.
    pub point: TrackPoint // The TrackPoint of this neighbor.
}

// Implement comparators for neighbor ---->

impl Eq for Neighbor {}

impl Ord for Neighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Neighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Neighbor {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}