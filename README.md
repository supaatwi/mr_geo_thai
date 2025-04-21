# mr_geo_thai
A Rust library for geolocation lookups in Thailand.

## Overview
```mr_geo_thai``` is a library that provides efficient geolocation services for Thailand using latitude and longitude coordinates. It utilizes a KD-Tree data structure for fast nearest-neighbor searches.

## Features
- Fast lookup of Thailand locations based on latitude/longitude coordinates
- Asynchronous API support with Tokio
- Geohash generation functionality
- Simple and intuitive API

## Installation
- Add this to your ```Cargo.toml```:
```toml
[dependencies]
mr_geo_thai = "0.2.0"
```

### Usage
```rust
use mr_geo_thai::{find_geolocation, load_geolocation, geohash};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the geolocation database
    let locations = load_geolocation()?;
    
    // Define coordinates
    let lat = 14.2617633;
    let lng = 100.782535;
    
    // Generate geohash
    let hash = geohash(lat, lng)?;
    println!("Geohash: {}", hash);
    
    // Find the nearest location
    if let Some(location) = find_geolocation(locations, lat, lng).await {
        println!("Found location: {} ({}, {})", location.address_en, location.latitude, location.longitude);
        println!("Address: {} Sub-district, {} District, {} Province", 
            location.sub_district_en, 
            location.district_en, 
            location.province_en);
    } else {
        println!("No location found for these coordinates");
    }
    
    Ok(())
}
```

## API Reference
```rust 
load_geolocation() -> Result<Vec<Location>, Box<dyn Error>>
```
Loads the geolocation database containing Thailand locations. The data is embedded in |the library using include_str!, so no external file loading is required.

```rust
geohash(lat: f64, lng: f64) -> Result<String, Box<dyn Error>>
```
Generates a geohash string from the provided latitude and longitude coordinates.

```rust
find_geolocation(locations: Vec<Location>, lat: f64, lng: f64) -> Future<Option<Location>>
```
Asynchronously finds the closest location in the database to the provided latitude and longitude.

Returns ```Some(Location)``` if a match is found, or ```None``` if no matching location exists.


```Location``` Structure
```rust
pub struct Location {
    pub sub_district_th: String,  // Sub-district name in Thai
    pub sub_district_en: String,  // Sub-district name in English
    pub district_th: String,      // District name in Thai
    pub district_en: String,      // District name in English
    pub latitude: f64,            // Latitude coordinate
    pub longitude: f64,           // Longitude coordinate
    pub province_th: String,      // Province name in Thai
    pub province_en: String,      // Province name in English
    pub address: String,          // Full address in Thai
    pub address_en: String,       // Full address in English
}
```


## Dependencies
- kd_tree: For efficient nearest-neighbor searches
- geohash: For generating geohash strings from coordinates
- csv: For reading data from the embedded CSV file

### License
MIT
#### Contributing
Contributions are welcome! Please feel free to submit a Pull Request.