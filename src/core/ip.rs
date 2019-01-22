mod ip {
    const DIGIT_AMOUNT: usize = 8;
    pub fn convert_to_octet(number: u8) -> [u8; 8] {
        let mut octet: [u8; 8] = [0; 8];
        for i in 0..DIGIT_AMOUNT {
            octet[i] = (number >> i) & 1;
        }
        octet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_convert_to_octet_1() {
        // Arrange
        let expected_number: u8 = 1;
        let expected_octet: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];
        // Act
        let actual_octet = ip::convert_to_octet(expected_number);
        // Assert
        assert_eq!(actual_octet, expected_octet);
    }
    
    #[test]
    fn test_convert_to_octet_193() {
        // Arrange
        let expected_number: u8 = 193;
        let expected_octet: [u8; 8] = [1, 0, 0, 0, 0, 0, 1, 1];
        // Act
        let actual_octet = ip::convert_to_octet(expected_number);
        // Assert
        assert_eq!(actual_octet, expected_octet);
    }
}
