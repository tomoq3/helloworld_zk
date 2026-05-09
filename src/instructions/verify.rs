use pinocchio::{AccountView, Address, ProgramResult, address::address, entrypoint, error::ProgramError, nostd_panic_handler};
use pinocchio_groth16::groth16::Groth16Verifier;

pub struct VerifyAccounts<'a> {
    pub owner: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for VerifyAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [owner, _] = accounts else {
            return Err(ProgramError::InvalidAccountOwner);
        };
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Self { owner })
    }
}



// 証明に必要なもの
#[repr(C)]
pub struct VerifyInstructionData {
    pub proof: [u8; 256],
    pub public_inputs: [u8; 32],
}


impl<'a> TryFrom<&'a [u8]> for VerifyInstructionData {
    type Error = ProgramError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<VerifyInstructionData>(){
            return Err(ProgramError::InvalidInstructionData);
        }
        Ok(unsafe {
            (data.as_ptr() as *const Self).read_unaligned()       
        })

    }
}

pub struct Verify<'a> {
    pub accounts: VerifyAccounts<'a>,
    pub instruction_data: VerifyInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for Verify<'a> {
    type Error = ProgramError;
    fn try_from((data, accounts): (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
        let accounts = VerifyAccounts::try_from(accounts)?;
        let instruction_data = VerifyInstructionData::try_from(data)?;

        Ok(Self { accounts, instruction_data })
    }
}

impl<'a> Verify<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;
    pub fn process(&mut self) -> ProgramResult {


        
        // let proof_a = &self.instruction_data.proof[0..64].try_into().unwrap();
        // let proof_b = &self.instruction_data.proof[64..192].try_into().unwrap();
        // let proof_c = &self.instruction_data.proof[192..256].try_into().unwrap();

        // // 配列の一要素にする。
        // let input_data = self.instruction_data.public_inputs;
        // let formatted_inputs = &[input_data];

        // let mut verifier = Groth16Verifier::new(
        //     proof_a, 
        //     proof_b, 
        //     proof_c, 
        //     formatted_inputs, 
        //     verifyingkey
        // ).unwrap();

        Ok(())
    }
}