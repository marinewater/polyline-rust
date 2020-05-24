use std::convert::TryFrom;

pub struct Chunks {
    chunks: Vec<u32>
}

impl Chunks {
    pub fn new() -> Chunks {
        return Chunks {
            chunks: vec![]
        };
    }

    /// splices an integer into chunks
    pub fn parse(&mut self, element: u32) {
        self.slice(element);
    }

    /// converts and splices string into integer chunks
    pub fn parse_line(&mut self, line: &str) {
        let mut chunk_slice: Vec<u32> = Vec::new();

        let line_length = line.len();
        for (i, letter) in line.chars().enumerate() {
            let mut element_int: u32 = letter as u32 - 63;
            if i != line_length - 1 {
                element_int = element_int & 0b11111;
            }

            chunk_slice.push(element_int);
        }
        self.chunks = chunk_slice;
    }

    /// returns the chunks as polyline in base64
    pub fn string(&mut self) -> String {
        self.or();

        let mut s = String::new();
        for e in self.chunks.iter() {
            s += char::try_from(*e).unwrap().to_string().as_str();
        }

        return s;
    }

    /// converts integer chunks into a single coordinate
    pub fn coordinate(&self, precision: u32) -> f64 {
        let mut result_int: i32 = 0;

        for (i, element) in self.chunks.iter().enumerate() {
            result_int += (element << i*5) as i32;
        }

        if result_int & 1 == 1 {
            result_int = !result_int;
        }

        result_int = result_int >> 1;

        return result_int as f64 / 10_u32.pow(precision) as f64;
    }

    /// splits elements into group of 5 bits
    fn slice(&mut self, element: u32) {
        if element == 0 {
            self.chunks = vec![0];
            return;
        }


        let mut chunk_slice: Vec<u32> = Vec::new();
        let bit_mask = 0b11111;
        let base: u32 = 2;

        let mut i: u32 = 0;
        while base.pow(i) <= element {
            let group = (element >> i) & bit_mask;
            chunk_slice.push(group);
            i += 5;
        }

        self.chunks = chunk_slice;
    }

    /// sets the 6th bit to 1 for every chunk except the last one
    /// as indicator bit for coordinate boundaries.
    /// It also adds 63 (decimal) to every group to ensure it is in
    /// ASCII range
    fn or(&mut self) {
        let chunks_length = self.chunks.len();

        for (i, e) in self.chunks.iter_mut().enumerate() {
            if i < chunks_length - 1 {
                *e = *e | 0x20;
            }
            *e += 63;
        }
    }
}