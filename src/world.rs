// originally, values were 40,40,10
// if we use values that can be divided by a power of two, its easier to store position as a single
// value.
pub const CHUNK_SIZE_X: u8 = 128;
pub const CHUNK_SIZE_Y: u8 = 128;
pub const CHUNK_SIZE_Z: u8 = 16;
// sqrt(18446744073709551615 / 128 / 128 / 16)
// or also, 2^23.
const CHUNK_COUNT_SQRT: u32 = 8388608;

