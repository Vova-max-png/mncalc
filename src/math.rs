pub fn getCube(num: f64) -> f64 {
    num*num*num
}

pub fn getChunks(num: f64) -> f64 {
    num / 16.0
}

pub fn getChunksForOne(chunks: f64, count: f64) -> f64 {
    chunks / count
}

pub fn getSquare(width: f64, height: f64) -> f64 {
    width * height
} 