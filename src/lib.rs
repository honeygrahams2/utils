use solana_program::{
    entrypoint::ProgramResult, 
    msg,
    program_error::ProgramError,    
};

pub fn assert_true(cond: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !cond {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}
