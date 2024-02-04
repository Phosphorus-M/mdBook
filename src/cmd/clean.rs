use super::command_prelude::*;
use crate::{get_book_dir, get_build_opts};
use anyhow::Context;
use mdbook::MDBook;
use std::fs;
use std::path::PathBuf;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("clean")
        .about("Deletes a built book")
        .arg_dest_dir()
        .arg_root_dir()
        .arg(Arg::new("language").short('l').long("language").num_args(1).value_parser(clap::value_parser!(String)).help("Language to render the compiled book in.{n}\
        Only valid if the [language] table in the config is not empty.{n}\
        If omitted, builds all translations and provides a menu in the generated output for switching between them."))
}

// Clean command implementation
pub fn execute(args: &ArgMatches) -> mdbook::errors::Result<()> {
    let book_dir = get_book_dir(args);
    let build_opts = get_build_opts(args);
    let book = MDBook::load_with_build_opts(book_dir, build_opts)?;

    let dir_to_remove = match args.get_one::<PathBuf>("dest-dir") {
        Some(dest_dir) => dest_dir.into(),
        None => book.root.join(&book.config.build.build_dir),
    };

    if dir_to_remove.exists() {
        fs::remove_dir_all(&dir_to_remove)
            .with_context(|| "Unable to remove the build directory")?;
    }

    Ok(())
}
