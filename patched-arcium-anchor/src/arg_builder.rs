pub struct ArgBuilder {
    args: ArgumentList,
}

macro_rules! impl_plaintext_direct {
    ($($fn_name:ident => $variant:ident: $ty:ty),* $(,)?) => {
        $(
            pub fn $fn_name(mut self, value: $ty) -> Self {
                self.args.args.push(ArgumentRef::$variant(value));
                self
            }
        )*
    };
}

macro_rules! impl_plaintext_number {
    ($($fn_name:ident => $variant:ident: $ty:ty),* $(,)?) => {
        $(
            pub fn $fn_name(mut self, value: $ty) -> Self {
                self.args.args.push(ArgumentRef::$variant(self.args.plaintext_numbers.len() as u8));
                self.args.plaintext_numbers.push(value as u64);
                self
            }
        )*
    };
}

macro_rules! impl_plaintext_128 {
    ($($fn_name:ident => $variant:ident: $ty:ty),* $(,)?) => {
        $(
            pub fn $fn_name(mut self, value: $ty) -> Self {
                self.args.args.push(ArgumentRef::$variant(self.args.values_128_bit.len() as u8));
                self.args.values_128_bit.push(value as u128);
                self
            }
        )*
    };
}

macro_rules! impl_byte_array {
    ($($fn_name:ident => $variant:ident),* $(,)?) => {
        $(
            pub fn $fn_name(mut self, value: [u8; 32]) -> Self {
                self.args.args.push(ArgumentRef::$variant(self.args.byte_arrays.len() as u8));
                self.args.byte_arrays.push(value);
                self
            }
        )*
    };
}

impl ArgBuilder {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            args: ArgumentList {
                args: Vec::new(),
                byte_arrays: Vec::new(),
                plaintext_numbers: Vec::new(),
                values_128_bit: Vec::new(),
                accounts: Vec::new(),
            },
        }
    }

    impl_plaintext_direct! {
        plaintext_bool => PlaintextBool: bool,
        plaintext_u8 => PlaintextU8: u8,
        plaintext_i8 => PlaintextI8: i8,
    }

    impl_plaintext_number! {
        plaintext_u16 => PlaintextU16: u16,
        plaintext_u32 => PlaintextU32: u32,
        plaintext_u64 => PlaintextU64: u64,
        plaintext_i16 => PlaintextI16: i16,
        plaintext_i32 => PlaintextI32: i32,
        plaintext_i64 => PlaintextI64: i64,
        plaintext_float => PlaintextFloat: f64,
    }

    impl_plaintext_128! {
        plaintext_i128 => PlaintextI128: i128,
        plaintext_u128 => PlaintextU128: u128,
    }

    impl_byte_array! {
        encrypted_bool => EncryptedBool,
        encrypted_u8 => EncryptedU8,
        encrypted_u16 => EncryptedU16,
        encrypted_u32 => EncryptedU32,
        encrypted_u64 => EncryptedU64,
        encrypted_u128 => EncryptedU128,
        encrypted_float => EncryptedFloat,
        encrypted_i8 => EncryptedI8,
        encrypted_i16 => EncryptedI16,
        encrypted_i32 => EncryptedI32,
        encrypted_i64 => EncryptedI64,
        encrypted_i128 => EncryptedI128,
        plaintext_point => PlaintextPoint,
        x25519_pubkey => X25519Pubkey,
    }

    pub fn arcis_ed25519_signature(mut self, value: [u8; 64]) -> Self {
        self.args.args.push(ArgumentRef::ArcisEd25519Signature(
            self.args.byte_arrays.len() as u8,
        ));
        let mut lower_bytes = [0u8; 32];
        let mut upper_bytes = [0u8; 32];
        lower_bytes.copy_from_slice(&value[0..32]);
        upper_bytes.copy_from_slice(&value[32..64]);

        self.args.byte_arrays.push(lower_bytes);
        self.args.byte_arrays.push(upper_bytes);
        self
    }

    pub fn account(mut self, pubkey: Pubkey, offset: u32, length: u32) -> Self {
        self.args
            .args
            .push(ArgumentRef::Account(self.args.accounts.len() as u8));
        self.args.accounts.push(AccountArgument {
            pubkey,
            offset,
            length,
        });
        self
    }

    pub fn add_shared_encrypted_struct<const LEN: usize>(mut self, value: crate::SharedEncryptedStruct<LEN>) -> Self {
        // Shared encrypted structs are typically passed as an account or a sequence of parameters
        // In this SDK version, we'll treat them as a series of byte arrays for the ciphertexts,
        // and plaintext for the nonce and key.
        // NOTE: This is a simplified implementation for the SBF environment.
        self = self.x25519_pubkey(value.encryption_key);
        self = self.plaintext_u128(value.nonce);
        for ct in value.ciphertexts {
            self = self.plaintext_point(ct);
        }
        self
    }

    pub fn build(self) -> ArgumentList {
        self.args
    }
}
