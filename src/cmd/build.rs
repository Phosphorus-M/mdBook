use super::command_prelude::*;
use crate::{get_book_dir, get_build_opts, open};
use mdbook::errors::Result;
use mdbook::MDBook;
use std::path::PathBuf;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("build")
        .about("Builds a book from its markdown files")
        .arg_dest_dir()
        .arg_root_dir()
        .arg_open()
        .arg(Arg::new("language").short('l').long("language").num_args(1).value_parser(clap::value_parser!(String)).help("Language to render the compiled book in.{n}\
        Only valid if the [language] table in the config is not empty.{n}\
        If omitted, builds all translations and provides a menu in the generated output for switching between them."))
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let opts = get_build_opts(args);
    println!("llegan {:?}", opts.language_ident);
    let mut book = MDBook::load_with_build_opts(&book_dir, opts)?;

    if let Some(dest_dir) = args.get_one::<PathBuf>("dest-dir") {
        book.config.build.build_dir = dest_dir.into();
    }

    book.build()?;

    if args.get_flag("open") {
        // FIXME: What's the right behaviour if we don't use the HTML renderer?
        let path = book.build_dir_for("html").join("index.html");
        if !path.exists() {
            error!("No chapter available to open");
            std::process::exit(1)
        }
        open(path);
    }

    Ok(())
}
