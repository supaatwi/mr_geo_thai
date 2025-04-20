use csv::ReaderBuilder;
use kd_tree::{KdPoint, KdTreeN};
use std::error::Error;
use std::fs::File;
use geohash::{encode, Coord};

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

pub fn load_geolocation(file_path: &str) -> Result<Vec<Location>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut locations: Vec<Location> = vec![];
    for result in rdr.records() {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_geolocation() -> Result<(), Box<dyn Error>> {
        let file_path = "dataset.csv";
        let locations = load_geolocation(file_path)?;
        let latlng = (14.2617633,100.782535);
        

        let result = find_geolocation(locations, latlng.0, latlng.1).await;

        print!("{:?}", result);
        Ok(())
    }
}