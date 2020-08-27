use std::convert::TryInto;

/// Returns a 16-bit unsigned integer converted from two bytes at a specified
/// position in a byte array.
///
/// The `to_uint16` function converts the bytes from index start_index to
/// start_index + 1 to a `u16` value. The order of bytes in the array must
/// reflect the endianness of the  computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bitconverter::to_uint16;
/// let buffer =  [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
/// assert_eq!(65280, to_uint16(&buffer, 2));
/// ```
///

pub fn to_uint16(data: &[u8], start_index: usize) -> u16 {
    u16::from_le_bytes(
        data[start_index..start_index + 2]
            .try_into()
            .expect("Failed to read u16. Invalid buffer provided."),
    )
}

/// Returns a 32-bit unsigned integer converted from four bytes at a specified
/// position in a byte array.
///
/// The `to_uint32` function converts the bytes from index start_index to
/// start_index + 3 to a `u32` value. The order of bytes in the array must
/// reflect the endianness of the computer system's architecture.
/// # Example
///
/// ```
/// use pkhexcore::util::bitconverter::to_uint32;
/// let buffer =  [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 19];
/// assert_eq!(261888, to_uint32(&buffer, 6));
/// ```
///

pub fn to_uint32(data: &[u8], start_index: usize) -> u32 {
    u32::from_le_bytes(
        data[start_index..start_index + 4]
            .try_into()
            .expect("Failed to read u32. Invalid buffer provided."),
    )
}
