use solana_program::pubkey::Pubkey;
use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey_1: Pubkey,
    pub initializer_pubkey_2: Pubkey,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Escrow {
    const LEN: usize = 65;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_pubkey_1,
            initializer_pubkey_2,
        ) = array_refs![src, 1, 32, 32];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Escrow {
            is_initialized,
            initializer_pubkey_1: Pubkey::new_from_array(*initializer_pubkey_1),
            initializer_pubkey_2: Pubkey::new_from_array(*initializer_pubkey_2),
            
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];
        let (
            is_initialized_dst,
            initializer_pubkey_1_dst,
            initializer_pubkey_2_dst,
        ) = mut_array_refs![dst, 1, 32, 32];

        let Escrow {
            is_initialized,
            initializer_pubkey_1,
            initializer_pubkey_2,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_1_dst.copy_from_slice(initializer_pubkey_1.as_ref());
        initializer_pubkey_2_dst.copy_from_slice(initializer_pubkey_2.as_ref());
    }
}