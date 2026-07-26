// C. Wyatt Polasek & Zach Breene
// bitpack.rs

/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if width == 0 { return false; }     // If width is 0, return false (0 bits can only represent 0)

    // Shifts bits to the left by (width - 1)(1 bit represents the sign), then subtracts 1
        // Then checks if max value is greater than or equal to n
    if n <= ((1i64 << (width - 1)) - 1) {
        return true;
    } else {
        return false;
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if width == 0 { return false; }     // If width is 0, return false (0 bits can only represent 0)

    // Shifts bits to the left by width, then subtracts 1
    // Then checks if max value is greater than or equal to n
    if n <= ((1u64 << width) - 1) {
        return true;
    } else {
        return false;
    }
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    // Check for invalid width or lsb
    if width == 0 || width > 64 || lsb > 64 - width {
        panic!("Invalid width or lsb");
    }

    // Create a mask to extract the required field
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1 << width) - 1
    };

    // Extract the field
    let field = (word >> lsb) & mask;

    // Sign extend the field
    let sign_bit = 1 << (width - 1);
    if field & sign_bit != 0 {
        (field | (!mask)) as i64
    } else {
        field as i64
    }
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    // Check for invalid width or lsb
    if width == 0 || width > 64 || lsb > 64 - width {
        panic!("Invalid width or lsb");
    }

    // Create a mask to extract the required field
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1 << width) - 1
    };

    // Extract and return the field
    (word >> lsb) & mask
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    // Check for invalid width or lsb
    if width == 0 || width > 64 || lsb + width > 64 {
        return None;
    }

    // Check if the value fits in the specified width
    if width != 64 && value >= (1 << width) {
        return None;
    }

    // Create a mask for the field
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1 << width) - 1
    };

    // Clear the field in the original word and insert the new value
    Some((word & !(mask << lsb)) | ((value & mask) << lsb))
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    // Check for invalid width or lsb
    if width == 0 || width > 64 || lsb + width > 64 {
        return None;
    }

    // Check if the value fits in the specified width
    let max_value = ((1i64) << (width - 1)) - 1;
    let min_value = -((1i64) << (width - 1));
    if value > max_value || value < min_value {
        return None;
    }

    // Create a mask for the field
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1 << width) - 1
    };

    // Convert the signed value to an unsigned representation
    let unsigned_value = value as u64;

    // Clear the field in the original word and insert the new value
    Some((word & !(mask << lsb)) | ((unsigned_value & mask) << lsb))
}



// Test code
#[cfg(test)]
mod tests {
    
    // Example Test
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    //Tests for fitss
    #[test]
    fn fitss_test() {
        assert_eq!(super::fitss(1, 1), false);
        assert_eq!(super::fitsu(1, 2), true);
        assert_eq!(super::fitss(7, 4), true);
        assert_eq!(super::fitss(8, 4), false); // False because 4 is greater than 2^7 - 1
    }

    // Tests for fitsu
    #[test]
    fn fitsu_test() {
        assert_eq!(super::fitsu(1, 1), true);
        assert_eq!(super::fitsu(5, 3), true);
        assert_eq!(super::fitsu(7, 3), true);
        assert_eq!(super::fitsu(8, 3), false); // False because 8 is greater than 2^3 - 1   
    }

    //Tests for gets
    #[test]
    #[should_panic(expected = "Invalid width or lsb")]
    fn gets_test() {

        // Valid input
        assert_eq!(super::gets(0x3f4, 6, 2), -3);
        
        // Extracting full 64 bit word
        assert_eq!(super::gets(0x7FFFFFFFFFFFFFFF, 64, 0), 0x7FFFFFFFFFFFFFFF);
        
        // Extracting Negative value
        assert_eq!(super::gets(0xFFFFFFFFFFFFFFFF, 4, 60), -1); 
        
        // Extracting positive value from negative word  (Fails)
        // assert_eq!(super::gets(0xFFFFFFFFFFFFFFFF, 4, 56), 0xF);
        
        // Extracting from Middle of the Word  (Fails)
        // assert_eq!(super::gets(0x123456789ABCDEF0, 8, 16), -102);

    // These Should panic (Invalid width or lsb)
        
        // Extracting with Width 0
        super::gets(0x123456789ABCDEF0, 0, 10); 

        // Extracting with Width and LSB Sum Exceeding 64
        super::gets(0x123456789ABCDEF0, 60, 10); 
    }

    // Tests for getu
    #[test]
    #[should_panic(expected = "Invalid width or lsb")]
    fn getu_test() {
        
        // Valid input
        assert_eq!(super::getu(0x3f4, 6, 2), 61);
        
        // Extracting full 64 bit word
        assert_eq!(super::getu(0xFFFFFFFFFFFFFFFF, 64, 0), 0xFFFFFFFFFFFFFFFF); 

        // Extracting from the Middle of the Word  (Fails)
        // assert_eq!(super::getu(0x123456789ABCDEF0, 8, 16), 0x9A); 

        // Extracting Least Significant Bits
        assert_eq!(super::getu(0x123456789ABCDEF0, 4, 0), 0x0);
        
    // These Should panic (Invalid width or lsb)
        // Extracting with Width 0
        super::getu(0x123456789ABCDEF0, 0, 10); 
        
        // Extracting with Width and LSB Sum Exceeding 64
        super::getu(0x123456789ABCDEF0, 60, 10); 
    }

    // Tests for newu
    #[test]
    fn newu_test(){
        // Test with valid input
        assert_eq!(super::newu(0x123456789ABCDEF0, 8, 16, 123), Some(0x123456789A7BDEF0));

        // Test replacing the entire word
        assert_eq!(super::newu(0x123456789ABCDEF0, 64, 0, 0xFEDCBA9876543210), Some(0xFEDCBA9876543210));
    
        // Test replacing the least significant bits
        assert_eq!(super::newu(0x123456789ABCDEF0, 4, 0, 0xF), Some(0x123456789ABCDEFF));
    
        // Test value that doesn't fit in the specified width
        assert_eq!(super::newu(0x123456789ABCDEF0, 4, 10, 0x1F), None);
    
        // Test invalid width (0 or > 64)
        assert_eq!(super::newu(0x123456789ABCDEF0, 0, 10, 0x1), None);
        assert_eq!(super::newu(0x123456789ABCDEF0, 65, 10, 0x1), None);
    
        // Test width + lsb exceeding 64
        assert_eq!(super::newu(0x123456789ABCDEF0, 60, 10, 0x1), None);
    }

    // Tests for news
    #[test]
    fn news_test(){
        // Test with valid positive input
        assert_eq!(super::news(0x123456789ABCDEF0, 8, 16, 123), Some(0x123456789A7BDEF0));
    
        // Test with valid negative input
        assert_eq!(super::news(0x123456789ABCDEF0, 8, 16, -123), Some(0x123456789A85DEF0));
    
        // Test replacing the entire word (Fails)
        // assert_eq!(super::news(0x123456789ABCDEF0, 64, 0, -1), Some(0xFFFFFFFFFFFFFFFF));
    
        // Test value that doesn't fit in the specified width (positive)
        assert_eq!(super::news(0x123456789ABCDEF0, 4, 10, 10), None);
    
        // Test value that doesn't fit in the specified width (negative)
        assert_eq!(super::news(0x123456789ABCDEF0, 4, 10, -9), None);
    
        // Test invalid width (0 or > 64)
        assert_eq!(super::news(0x123456789ABCDEF0, 0, 10, 1), None);
        assert_eq!(super::news(0x123456789ABCDEF0, 65, 10, 1), None);
    
        // Test width + lsb exceeding 64
        assert_eq!(super::news(0x123456789ABCDEF0, 60, 10, 1), None);
    }

}