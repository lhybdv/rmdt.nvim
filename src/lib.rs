mod fmt;
mod types;
mod swap;
mod utils;
mod opts;

use nvim_oxi::api::{self,  types::*};
use nvim_oxi::{self as oxi, Dictionary, Error, Result};
use oxi::Function;
use oxi::api::opts::CreateCommandOpts;

#[oxi::plugin]
fn rmdt() -> Result<Dictionary> {
    let setup = Function::from_fn::<_, Error>(move |()| {
        let opts = CreateCommandOpts::builder()
            .bang(true)
            .nargs(CommandNArgs::ZeroOrOne)
            .build();

        let r_format_table = |_args: CommandArgs| {
            if let Err(e) = fmt::format_table() {
                api::err_writeln(e.to_string().as_str())
            }
            Ok(())
        };

        let r_col_swap = |_args: CommandArgs| {
            if let Err(e) = swap::col_swap() {
                api::err_writeln(e.to_string().as_str())
            }
            Ok(())
        };

        api::create_user_command("RFormatTable", r_format_table, &opts)?;

        api::create_user_command("RColumnSwap", r_col_swap, &opts)?;

        Ok(())
    });

    Ok(Dictionary::from_iter([
        ("setup", setup),
    ]))
}
