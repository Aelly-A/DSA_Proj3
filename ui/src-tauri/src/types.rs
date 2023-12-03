// A TrackPoint is a point on the 2D map, this reperesents a single "Track" within Spotify.
// Be sure to read the documentation for each element!
#[derive(Clone)]
pub struct TrackPoint {
    pub x: f64, // The X dimension represents "groove", check out the "determineGroove" function in loader.rs.
    pub y: f64, // The Y dimension represents "style", check out the "determineStyle" function in loader.rs.
    pub duration_ms: u32, // This is the duration of the track.
    pub explicit: bool, // Is the track explicit?
    pub id: String, // Spotify UUID for the track.
    pub artists: Vec<String> // List of artists.
}