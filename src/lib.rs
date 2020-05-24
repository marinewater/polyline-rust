//! # polyline_rust
//! Crate to encode/decode polylines in "Encoded Polyline Algorithm Format"
//!
//! Usage example:
//! ```
//! use polyline_rust::{Point, encode, decode};
//!
//! fn main() {
//!     let polyline = encode(vec![
//!         Point::new(12.34567, 89.01234),
//!         Point::new(12.34891, 89.01567),
//!         Point::new(12.35678, 89.01891),
//!     ], 5);
//!     println!("{}", polyline); // output: "mgjjAcfh~OgSySep@gS"
//!
//!     let coordinates = decode(&polyline, 5);
//!     for point in coordinates {
//!         println!("{}, {}", point.latitude, point.longitude);
//!     }
//!     /*
//!         output:
//!             12.34567, 89.01234
//!             12.34891, 89.01567
//!             12.35678, 89.01891
//!      */
//! }
//! ```

mod chunks;

/// Single Coordinate of a point on the polyline
#[derive(PartialEq, Debug)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64
}

impl Point {
    /// Creates a new `Point`.
    pub fn new(latitude: f64, longitude: f64) -> Point {
        return Point {
            latitude,
            longitude
        };
    }
}

/// Encodes coordinates to the "Encoded Polyline Algorithm Format".
///
/// More info: [https://developers.google.com/maps/documentation/utilities/polylinealgorithm](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
///
/// `points`: points of the polyline
///
/// `precision`: usually 5 or 6; Google's original algorithm uses 5 digits of decimal precision,
/// which is accurate to about a meter. A precision of 6 gives you an accuracy of about 10cm
///
/// More info: [https://mapzen.com/blog/polyline-precision/](https://mapzen.com/blog/polyline-precision/)
pub fn encode(points: Vec<Point>, precision: u32) -> String {
    let mut encoded = String::new();

    let mut latitude: f64 = 0.;
    let mut longitude: f64 = 0.;

    for point in points.iter() {
        let poly_latitude = encode_element(point.latitude-latitude, precision);
        encoded += poly_latitude.as_str();

        let poly_longitude = encode_element(point.longitude-longitude, precision);
        encoded += poly_longitude.as_str();

        latitude = point.latitude;
        longitude = point.longitude;
    }

    return encoded;
}

/// Shorthand call for encode with precision set to 5.
///
/// Accuracy is about one meter.
pub fn encode5(points: Vec<Point>) -> String {
    return encode(points, 5);
}

/// Shorthand call for encode with precision set to 6.
///
/// Accuracy is about ten centimeters.
pub fn encode6(points: Vec<Point>) -> String {
    return encode(points, 6);
}

/// Decodes coordinates from the "Encoded Polyline Algorithm Format".
///
/// More info: [https://developers.google.com/maps/documentation/utilities/polylinealgorithm](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
///
/// `polyline`: polyline string in "Encoded Polyline Algorithm Format"
///
/// `precision`: Usually 5 or 6; Google's original algorithm uses 5 digits of decimal precision,
/// which is accurate to about a meter. A precision of 6 gives you an accuracy of about 10cm.
///
/// More info: [https://mapzen.com/blog/polyline-precision/](https://mapzen.com/blog/polyline-precision/)
pub fn decode(polyline: &str, precision: u32) -> Vec<Point> {

    let mut group = String::new();
    let mut coordinates: Vec<f64> = Vec::new();

    for letter in polyline.chars() {
        group += letter.to_string().as_str();

        if (letter as i32 - 63) & 0x20 == 0 {
            coordinates.push(decode_element(group.as_str(), precision));
            group = String::new();
        }
    }

    let mut points: Vec<Point> = Vec::new();
    let mut i = 1;
    while i < coordinates.len() {
        points.push(Point{
            latitude: round(coordinates[i-1], precision),
            longitude: round(coordinates[i], precision)
        });
        i += 2;
    }

    let mut latitude: f64 = 0.0;
    let mut longitude: f64 = 0.0;
    for e in points.iter_mut() {
        e.latitude = round(latitude+e.latitude, precision);
        e.longitude = round(longitude+e.longitude, precision);
        latitude = e.latitude;
        longitude = e.longitude;
    }

    return points;
}

/// Shorthand call for Decode with precision set to 5.
///
/// Accuracy is about one meter.
pub fn decode5(polyline: &str) -> Vec<Point> {
    return decode(polyline, 5);
}

/// Shorthand call for Decode with precision set to 6.
///
/// Accuracy is about ten centimeters.
pub fn decode6(polyline: &str) -> Vec<Point> {
    return decode(polyline, 6);
}

fn encode_element(element: f64, precision: u32) -> String {
    let base10: u32 = 10;
    let mut element_int: i32 = (element * base10.pow(precision) as f64).round() as i32;
    element_int = element_int << 1;
    if element < 0 as f64 {
        element_int = !element_int;
    }
    
    let mut c = chunks::Chunks::new();
    c.parse(element_int as u32);

    return c.string();

}

fn decode_element(group: &str, precision: u32) -> f64 {

    let mut c = chunks::Chunks::new();
    c.parse_line(group);
    return c.coordinate(precision);

}

fn round(n: f64, precision: u32) -> f64 {
    let factor = 10_u32.pow(precision) as f64;

    return (n*factor).round() / factor;
}

#[cfg(test)]
mod tests {
    mod encode_tests {

        mod precision_5 {
            use crate::{Point, encode, encode5};

            #[test]
            fn empty_string() {
                assert_eq!(encode(vec![], 5), "");
            }

            #[test]
            fn single_point() {
                assert_eq!(encode(vec![Point::new(-79.448, -179.9832104)], 5), "~d|cN`~oia@");
            }

            #[test]
            fn multiple_points() {
                assert_eq!(encode(vec![
                    Point::new(38.5, -120.2),
                    Point::new(40.7, -120.95),
                    Point::new(43.252, -126.453)
                ], 5), "_p~iF~ps|U_ulLnnqC_mqNvxq`@");
            }

            #[test]
            fn same_point_twice() {
                assert_eq!(encode(vec![
                    Point::new(-37.472889, -72.353958),
                    Point::new(-37.472889, -72.353958)
                ], 5), "p|ucFfsrxL??");
            }

            #[test]
            fn test_encode5() {
                assert_eq!(encode5(vec![Point::new(-79.448, -179.9832104)]), "~d|cN`~oia@");
            }
        }

        mod precision_6 {
            use crate::{Point, encode, encode6};

            #[test]
            fn empty_string() {
                assert_eq!(encode(vec![], 6), "");
            }

            #[test]
            fn multiple_points() {
                assert_eq!(encode(vec![
                    Point::new(48.208771, 16.372572),
                    Point::new(48.210133, 16.374164),
                    Point::new(48.210495, 16.373436)
                ], 6), "ewl}zAwthf^ctAobBsUnl@");
            }

            #[test]
            fn multiple_points_with_minimal_distance() {
                assert_eq!(encode(vec![
                    Point::new(-37.472889, -72.353958),
                    Point::new(-37.472687, -72.357526),
                    Point::new(-37.472165, -72.357484),
                    Point::new(-37.472273, -72.355672),
                    Point::new(-37.472889, -72.353958),
                ], 6), "pfdnfAjic_iCsK~}Es_@sAvEgpBne@cjB");
            }

            #[test]
            fn same_point_twice() {
                assert_eq!(encode(vec![
                    Point::new(-37.472889, -72.353958),
                    Point::new(-37.472889, -72.353958)
                ], 6), "pfdnfAjic_iC??");
            }

            #[test]
            fn test_encode6() {
                assert_eq!(encode6(vec![Point::new(-79.4486385, -179.9832104)]), "|bdpvCruhhvI");
            }
        }
    }

    mod decode_tests {
        mod precision_5 {
            use crate::{Point, decode, decode5};
            #[test]
            fn empty_string() {
                assert_eq!(decode("", 5), vec![]);
            }

            #[test]
            fn not_a_polyline() {
                assert_eq!(decode("a", 5), vec![]);
            }

            #[test]
            fn single_point() {
                assert_eq!(decode("~d|cN`~oia@", 5), vec![
                    Point::new(-79.448, -179.98321)
                ]);
            }

            #[test]
            fn multiple_points() {
                assert_eq!(decode("_p~iF~ps|U_ulLnnqC_mqNvxq`@", 5), vec![
                    Point::new(38.5, -120.2),
                    Point::new(40.7, -120.95),
                    Point::new(43.252, -126.453)
                ]);
            }

            #[test]
            fn same_point_twice() {
                assert_eq!(decode("p|ucFfsrxL??", 5), vec![
                    Point::new(-37.47289, -72.35396),
                    Point::new(-37.47289, -72.35396)
                ]);
            }

            #[test]
            fn test_decode5() {
                assert_eq!(decode5("~d|cN`~oia@"), vec![
                    Point::new(-79.448, -179.98321)
                ]);
            }
        }

        mod precision_6 {
            use crate::{Point, decode, decode6};
            #[test]
            fn empty_string() {
                assert_eq!(decode("", 6), vec![]);
            }

            #[test]
            fn not_a_polyline() {
                assert_eq!(decode("a", 6), vec![]);
            }

            #[test]
            fn multiple_points() {
                assert_eq!(decode("ewl}zAwthf^ctAobBsUnl@", 6), vec![
                    Point::new(48.208771, 16.372572),
                    Point::new(48.210133, 16.374164),
                    Point::new(48.210495, 16.373436)
                ]);
            }

            #[test]
            fn multiple_points_with_minimal_distance() {
                assert_eq!(decode("pfdnfAjic_iCsK~}Es_@sAvEgpBne@cjB", 6), vec![
                    Point::new(-37.472889, -72.353958),
                    Point::new(-37.472687, -72.357526),
                    Point::new(-37.472165, -72.357484),
                    Point::new(-37.472273, -72.355672),
                    Point::new(-37.472889, -72.353958),
                ]);
            }

            #[test]
            fn same_point_twice() {
                assert_eq!(decode("pfdnfAjic_iC??", 6), vec![
                    Point::new(-37.472889, -72.353958),
                    Point::new(-37.472889, -72.353958)
                ]);
            }

            #[test]
            fn test_decode6() {
                assert_eq!(decode6("|bdpvCruhhvI"), vec![Point::new(-79.448639, -179.98321)]);
            }
        }
    }
}
