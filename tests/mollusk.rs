
#[cfg(test)]
mod test {

    use solana_account::Account;
    use solana_sdk::{message::{AccountMeta, Instruction}, msg, pubkey::Pubkey};
    use mollusk_svm::{Mollusk, program::keyed_account_for_system_program};

    const ID: Pubkey = solana_sdk::pubkey!("22222222222222222222222222222222222222222222");
    #[test]
    fn mollusk() {
        let user = Pubkey::new_unique();
        let user_account = Account::default();
        let (system_program, system_program_account) = keyed_account_for_system_program();

        let addresses = vec![
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program, false),
        ];

        let loader = vec![
            (user, user_account),
            (system_program, system_program_account),
        ];

        let mut instruction_data = [0u8; 289];

        let mut mollusk = Mollusk::new(&ID, "target/deploy/pinocchio_with_arkworks");

        
        let res = mollusk.process_instruction(
            &Instruction::new_with_bytes(
                ID, 
                &instruction_data, 
                addresses
            ), 
            &loader
        );
        msg!("{:?}", res.program_result);

    }
    
}