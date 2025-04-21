use kd_tree::{KdPoint, KdTreeN};
use std::error::Error;
use geohash::{encode, Coord};
use csv::Reader;

#[derive(Debug, Clone)]
pub struct Location {
    pub sub_district_th: String,
    pub sub_district_en: String,
    pub district_th: String,
    pub district_en: String,
    pub latitude: f64,
    pub longitude: f64,
    pub province_th: String,
    pub province_en: String,
    pub address: String,
    pub address_en: String,
}

impl KdPoint for Location {
    type Scalar = f64;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f64 {
        match k {
            0 => self.latitude,
            1 => self.longitude,
            _ => panic!("Invalid dimension"),
        }
    }
}

pub fn load_geolocation() -> Result<Vec<Location>, Box<dyn Error>> {
    let csv_content = include_str!("../dataset.csv");
    let mut reader = Reader::from_reader(csv_content.as_bytes());

    let mut locations: Vec<Location> = vec![];
    for result in reader.records() {
        let record = result?;
        let location = Location {
            sub_district_th: record[8].to_string(),
            sub_district_en: record[9].to_string(),
            district_th: record[2].to_string(),
            district_en: record[3].to_string(),
            latitude: record[4].parse::<f64>()?,
            longitude: record[5].parse::<f64>()?,
            province_th: record[6].to_string(),
            province_en: record[7].to_string(),
            address: record[0].to_string(),
            address_en: record[1].to_string()
        };
        locations.push(location);
    }

    Ok(locations)
}

pub fn geohash(lat: f64, lng: f64) -> Result<String, Box<dyn Error>> {
    Ok(encode(Coord { x: lat, y: lng }, 9usize)?)
}

pub async fn find_geolocation(locations: Vec<Location>, lat: f64, lng: f64) -> Option<Location> {
    let tree = KdTreeN::build_by_ordered_float(locations);
    let nearest = tree.nearest(&[lat, lng]);
    nearest.map(|n| n.item.clone())
}
