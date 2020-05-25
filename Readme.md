# polyline-rust

Crate to encode/decode polylines in "Encoded Polyline Algorithm Format".

Usage example:
 ```
 use polyline_rust::{Point, encode, decode};

 fn main() {
     let polyline = encode(vec![
         Point::new(12.34567, 89.01234),
         Point::new(12.34891, 89.01567),
         Point::new(12.35678, 89.01891),
     ], 5);
     println!("{}", polyline); // output: "mgjjAcfh~OgSySep@gS"

     let coordinates = decode(&polyline, 5);
     for point in coordinates {
         println!("{}, {}", point.latitude, point.longitude);
     }
     /*
         output:
             12.34567, 89.01234
             12.34891, 89.01567
             12.35678, 89.01891
      */
 }
 ```