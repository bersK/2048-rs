
pub fn clamp(x: u32, mut min: u32, mut max: u32) -> u32 {
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}