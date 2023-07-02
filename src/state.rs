use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NucleotideState {
    pub dna: String,
    pub rna: String,
    pub complement: String,
}

impl NucleotideState {
    pub fn get_account_size(nucleotide: String) -> usize {
        (4 + nucleotide.len()) * 3
    }
}
