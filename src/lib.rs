mod fmt;
mod types;
mod swap;
mod utils;

use nvim_oxi::api::{self,  types::*};
use nvim_oxi::{self as oxi, Dictionary};
use oxi::Function;
use oxi::api::opts::CreateCommandOpts;

#[oxi::module]
fn rmdt() -> oxi::Result<Dictionary> {
    let setup = Function::from_fn::<_, oxi::Error>(move |()| {
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

        if let Err(e) = api::create_user_command("RFormatTable", r_format_table, &opts) {
            api::err_writeln(e.to_string().as_str());
        }
        Ok(())
    });

    Ok(Dictionary::from_iter([
        ("setup", setup),
    ]))
}