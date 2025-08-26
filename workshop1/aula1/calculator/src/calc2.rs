pub fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

pub fn rate(a: u32, b: u32) -> u32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}