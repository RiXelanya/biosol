use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum DNAOperation {
    TranscribeRNA { rna: String },
    ComplementDNA { dna: String },
    TranscribeDNA { dna: String },
    CreateNucleotideInfo { dna: String },
    StoreNucleotideInfo { dna: String },
}

#[derive(BorshDeserialize)]
struct Nucleotide {
    nucleotide: String,
}

impl DNAOperation {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => {
                let payload = Nucleotide::try_from_slice(rest).unwrap();
                Self::TranscribeDNA { dna: payload.nucleotide }
            }
            1 => {
                let payload = Nucleotide::try_from_slice(rest).unwrap();
                Self::ComplementDNA { dna: payload.nucleotide }
            }
            2 => {
                let payload = Nucleotide::try_from_slice(rest).unwrap();
                Self::TranscribeRNA { rna: payload.nucleotide }
            }
            3 => {
                let payload = Nucleotide::try_from_slice(rest).unwrap();
                Self::CreateNucleotideInfo { dna: payload.nucleotide }
            }
            4 => {
                let payload = Nucleotide::try_from_slice(rest).unwrap();
                Self::StoreNucleotideInfo { dna: payload.nucleotide }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
