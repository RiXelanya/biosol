use crate::error::ProcessingError;
use crate::instruction::DNAOperation;
use crate::state::NucleotideState;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = DNAOperation::unpack(instruction_data)?;
    match instruction {
        DNAOperation::TranscribeDNA {
            dna,
        } => dtranscribe(dna),
        DNAOperation::ComplementDNA {
            dna,
        } => complement(dna),
        DNAOperation::TranscribeRNA {
            rna,
        } => rtranscribe(rna),
        DNAOperation::CreateNucleotideInfo {
            dna,
        } => create(program_id, accounts, dna),
        DNAOperation::StoreNucleotideInfo {
            dna,
        } => store(program_id, accounts, dna),
    }
}

pub fn try_from_slice_unchecked<T: borsh::BorshDeserialize>(data: &[u8]) -> Result<T, ProgramError> {
  let mut data_mut = data;
  match T::deserialize(&mut data_mut) {
    Ok(result) => Ok(result),
    Err(_) => Err(ProgramError::InvalidInstructionData)
  }
}

pub fn dtranscribe(dna: String) -> ProgramResult {
    msg!(dtranscribe1(&dna).as_str());
    Ok(())
}

pub fn dtranscribe1(dna: &String) -> String {
    dna.chars().map(|a| match a {
        'T' => 'U' ,
        _ => a ,
    }).collect::<String>()
}

pub fn rtranscribe(rna: String) -> ProgramResult {
    msg!(rna.chars().map(|a| match a {
        'U' => 'T' ,
        _ => a ,
    }).collect::<String>().as_str()
);
    Ok(())
}

pub fn complement(dna: String) -> ProgramResult {
    msg!(complement1(&dna).as_str());
    Ok(())
}

pub fn complement1(dna: &String) -> String {
    dna.chars().map(|a| match a {
    'A' => 'T' ,
    'T' => 'A' ,
    'C' => 'G' ,
    'G' => 'C' ,
    _ => a ,
    }).collect::<String>()

}

pub fn create(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    dna: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let seed = String::from("nucleotides") ;

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), seed.as_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let account_len: usize = 1000;

    let datasize: usize = NucleotideState::get_account_size(dna.clone()) ;
    if datasize > account_len {
        msg!("Data length is larger than 1000 bytes");
        return Err(ProcessingError::InvalidDataLength.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

     invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            seed.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    let mut counter_data =
        try_from_slice_unchecked::<NucleotideState>(&pda_account.data.borrow()).unwrap();
    counter_data.rna = dtranscribe1(&dna) ;
    counter_data.complement = complement1(&dna) ;
    counter_data.dna = dna ;

    counter_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn store(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    dna: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let seed = String::from("nucleotides");

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), seed.as_bytes().as_ref()],
        program_id,
    );

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let account_len: usize = 1000;

    let datasize: usize = NucleotideState::get_account_size(dna.clone()) ;
    if datasize > account_len {
        msg!("Data length is larger than 1000 bytes");
        return Err(ProcessingError::InvalidDataLength.into());
    }


    let mut counter_data =
        try_from_slice_unchecked::<NucleotideState>(&pda_account.data.borrow()).unwrap();
    
    counter_data.rna = dtranscribe1(&dna) ;
    counter_data.complement = complement1(&dna) ;
    counter_data.dna = dna ;

    counter_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}