# mr geolocation thailand

This Rust library provides utilities for working with geolocation data in Thailand. It includes functionality for loading geolocation data from a CSV file, calculating geohashes, and finding the nearest location using a k-d tree.

## Features

- **Load Geolocation Data**: Parse a CSV file containing geolocation data into a vector of `Location` structs.
- **Geohash Calculation**: Generate geohashes for given latitude and longitude coordinates.
- **Find Nearest Location**: Use a k-d tree to find the nearest location to a given latitude and longitude.

## Data Structure

The library uses the following `Location` struct to represent geolocation data:

```rust
#[derive(Debug, Clone)]
pub struct Location {
    pub sub_district_th: String, // Sub-district name in Thai
    pub sub_district_en: String, // Sub-district name in English
    pub district_th: String,     // District name in Thai
    pub district_en: String,     // District name in English
    pub latitude: f64,           // Latitude
    pub longitude: f64,          // Longitude
    pub province_th: String,     // Province name in Thai
    pub province_en: String,     // Province name in English
    pub address: String,         // Full address in Thai
    pub address_en: String,      // Full address in English
}
```

### Functions
load_geolocation(file_path: &str) -> Result<Vec<Location>, Box<dyn Error>>
Loads geolocation data from a CSV file and returns a vector of Location structs.

Parameters:
file_path: Path to the CSV file.
Returns: A Result containing a vector of Location structs or an error.
geohash(lat: f64, lng: f64) -> Result<String, Box<dyn Error>>
Generates a geohash for the given latitude and longitude.

Parameters:
lat: Latitude.
lng: Longitude.
Returns: A Result containing the geohash string or an error.
find_geolocation(locations: Vec<Location>, lat: f64, lng: f64) -> Option<Location>
Finds the nearest location to the given latitude and longitude using a k-d tree.

Parameters:
- locations: A vector of Location structs.
- lat: Latitude of the target point.
- lng: Longitude of the target point.
- Returns: An Option containing the nearest Location or None if no locations are found.

#### Example Usage
- Loading Geolocation Data
- Calculating a Geohash
- Finding the Nearest Location


#### Example Test Output
Dependencies
This library uses the following crates:

csv: For reading CSV files.
kd_tree: For k-d tree implementation.
geohash: For geohash generation.
tokio: For asynchronous testing.
License
This project is licensed under the MIT License. See the LICENSE file for details. ```