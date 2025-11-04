pub fn egg_count(display_value: u32) -> usize {
    // The display shows the decimal value of the bitmask where each bit
    // represents an egg (1) or empty spot (0). Count the number of set
    // bits (population count) and return as usize.
    display_value.count_ones() as usize
}
