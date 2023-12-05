use std::fs::File;

use crate::{knn::PointMap, types::{AbstractKNN, TrackPoint}, aknn::KDTree};

#[derive(Debug, serde::Deserialize)]
struct CSVRecord {
    valence: f64,
    year: u64,
    acousticness: f64,
    artists: String,
    danceability: f64,
    duration_ms: u64,
    energy: f64,
    explicit: u64,
    id: String,
    instrumentalness: f64,
    key: u64,
    liveness: f64,
    loudness: f64, // In DB
    mode: u64,
    name: String,
    popularity: u64,
    release_date: String,
    speechiness: f64,
    tempo: f64
}

pub fn load_pointmap() -> std::io::Result<PointMap> {
    let mut pm = PointMap::new();

    let file = File::open("data/data.csv")?;

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: CSVRecord = result?;
        pm.insert(TrackPoint{
            x: calculate_x(&record),
            y: calculate_y(&record),
            duration_ms: record.duration_ms,
            explicit: record.explicit == 1,
            id: record.id,
            name: record.name,
            artists: Vec::from([record.artists]),
        })
    };

    return Ok(pm);
}

pub fn load_kd_tree() -> std::io::Result<KDTree> {
    let mut m: KDTree = KDTree::new();
    let mut points: Vec<TrackPoint> = Vec::new();

    let file = File::open("data/data.csv")?;

    let mut rdr = csv::Reader::from_reader(file);
    let mut i = 0;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: CSVRecord = result?;
        points.push(TrackPoint{
            x: calculate_x(&record),
            y: calculate_y(&record),
            duration_ms: record.duration_ms,
            explicit: record.explicit == 1,
            id: record.id,
            name: record.name,
            artists: Vec::from([record.artists]),
        });
        if i % 1000 == 0 {
            println!("Done with: {}", i);
        }
        i += 1 
    };

    m.create_in_place(points, 0);

    return Ok(m);
}

fn calculate_x(record: &CSVRecord) -> f64 {
    return (
        record.valence +
        record.acousticness +
        record.danceability +
        record.energy
    ) / 4.0
}

fn calculate_y(record: &CSVRecord) -> f64 {
    return (
        record.instrumentalness +
        record.liveness +
        record.speechiness
    ) / 3.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_load_csv_into_pointmap() {
        let map = super::load_pointmap().unwrap();
    }

    #[test]
    fn test_load_csv_into_kdtree() {
        let map = super::load_kd_tree().unwrap();
    }
}