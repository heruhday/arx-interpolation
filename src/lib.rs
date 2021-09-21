#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[macro_use]
pub mod arx;
pub mod entry;
pub mod entry_functions;
pub mod commands;
pub mod functions;
pub mod transaction;
pub mod database;
pub mod error;
pub mod tables;


#[cfg(test)]
mod test {
    use crate::error;

    #[test]
    fn server_test() -> error::Result<()> {
        Ok(())
    }
}
