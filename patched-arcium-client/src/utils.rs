use solana_alt_bn128_bls::{BLSError, PrivKey as BLSPrivKey};
use solana_program::hash::hashv;

#[cfg(feature = "transactions")]
pub type X25519PubKey = x25519_dalek::PublicKey;
#[cfg(feature = "transactions")]
pub type X25519PrivKey = x25519_dalek::StaticSecret;

// Maximum realloc size per ix
pub const MAX_REALLOC_SIZE_PER_IX: usize = 10240; // 10KB = 10 * 1024

// Maximum size of a Solana account in bytes.
pub const MAX_SOL_ACCOUNT_SIZE_BYTES: usize = 10485760; // 10MB = 10 * 1024 * 1024

// Size of the metadata in the raw circuit account. 1 byte for the bump, 8 for the discriminator.
pub const METADATA_SIZE_RAW_CIRCUIT_ACC: usize = 9;

// Maximum number of recovery peers per MXE.
pub const MAX_RECOVERY_PEERS: usize = 100;

// Maximum number of circuit bytes per raw circuit account.
pub const MAX_RAW_CIRCUIT_BYTES_PER_ACC: usize =
    MAX_SOL_ACCOUNT_SIZE_BYTES - METADATA_SIZE_RAW_CIRCUIT_ACC;

pub(crate) fn sha256(vals: &[&[u8]]) -> [u8; 32] {
    hashv(vals).to_bytes()
}

pub fn parse_bls_keypair(keypair_str: &str) -> Result<BLSPrivKey, BLSError> {
    let key_array =
        parse_byte_array::<32>(keypair_str).map_err(|_| BLSError::SerializationError)?;
    Ok(BLSPrivKey(key_array))
}

/// Parse a byte array from a string.
/// The string should be in the format
/// `"[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32]"`.
/// Returns an error if the string is not in the correct format.
pub fn parse_byte_array<const N: usize>(bytes_str: &str) -> Result<[u8; N], String> {
    let bytes_str = bytes_str.trim();
    if !bytes_str.starts_with('[') || !bytes_str.ends_with(']') {
        return Err("Invalid byte array format".to_string());
    }
    let inner = &bytes_str[1..bytes_str.len() - 1];
    let bytes: Vec<u8> = inner
        .split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| "Failed to parse byte array".to_string())?;
    let byte_array: [u8; N] = bytes
        .try_into()
        .map_err(|_| "Failed to parse byte array".to_string())?;
    Ok(byte_array)
}

pub fn serialize_bls_keypair(keypair: &BLSPrivKey) -> String {
    format!("{:?}", keypair.0)
}

#[cfg(feature = "transactions")]
pub fn serialize_x25519_keypair(keypair: &X25519PrivKey) -> Result<String, serde_json::Error> {
    serde_json::to_string(&keypair.as_bytes())
}

#[cfg(feature = "transactions")]
pub fn parse_x25519_keypair(keypair_str: &str) -> Result<X25519PrivKey, serde_json::Error> {
    serde_json::from_str(keypair_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bls_keypair_round_trip() {
        // Create a keypair with known bytes
        let original = BLSPrivKey([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ]);

        // Serialize it
        let serialized = serialize_bls_keypair(&original);

        // Parse it back
        let parsed = parse_bls_keypair(&serialized).expect("Failed to parse serialized keypair");

        // Should match original
        assert_eq!(original.0, parsed.0);
    }

    #[test]
    fn test_bls_keypair_serialize_format() {
        let keypair = BLSPrivKey([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ]);
        let serialized = serialize_bls_keypair(&keypair);

        // Should be in array format
        assert!(serialized.starts_with('['));
        assert!(serialized.ends_with(']'));
        assert!(serialized.contains(", "));
    }

    #[test]
    fn test_bls_keypair_parse_valid() {
        let input = "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]";
        let parsed = parse_bls_keypair(input).expect("Failed to parse valid keypair");

        assert_eq!(parsed.0[0], 1);
        assert_eq!(parsed.0[31], 32);
    }

    #[test]
    fn test_bls_keypair_parse_with_whitespace() {
        let input = "  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32]  ";
        let parsed = parse_bls_keypair(input).expect("Failed to parse keypair with whitespace");

        assert_eq!(parsed.0[0], 1);
        assert_eq!(parsed.0[31], 32);
    }

    #[test]
    fn test_bls_keypair_parse_invalid_format() {
        let inputs = vec!["not an array", "[1, 2, 3", "1, 2, 3]", "{1, 2, 3}", ""];

        for input in inputs {
            assert!(
                parse_bls_keypair(input).is_err(),
                "Should fail for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_bls_keypair_parse_wrong_length() {
        // Too few bytes
        let input = "[1, 2, 3]";
        assert!(parse_bls_keypair(input).is_err());

        // Too many bytes
        let input = "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33]";
        assert!(parse_bls_keypair(input).is_err());
    }

    #[test]
    fn test_bls_keypair_parse_invalid_numbers() {
        // Not a number
        let input = "[1, 2, abc, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]";
        assert!(parse_bls_keypair(input).is_err());

        // Out of u8 range
        let input = "[256, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]";
        assert!(parse_bls_keypair(input).is_err());
    }
}
