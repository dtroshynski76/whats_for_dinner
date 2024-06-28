use std::{env, fs, process};

mod recipe_choose;
mod recipe_cli;
mod recipe_file;

use recipe_choose::random_recipe;
use recipe_cli::cli::Arguments;
use recipe_file::file_utils;

fn main() {
    let args = Arguments::build(env::args()).unwrap_or_else(|failure| {
        eprintln!("Problem building arguments struct: {failure}");
        process::exit(1)
    });

    let file_contents = fs::read_to_string(&args.file).unwrap_or_else(|err| {
        eprintln!("Problem opening recipes file '{}': {err}", &args.file);
        process::exit(1)
    });

    let recipes = file_utils::parse_recipe_file(file_contents).unwrap_or_else(|failure| {
        eprintln!("Problem parsing recipes: {failure}");
        process::exit(1)
    });

    let chosen_recipe =
        random_recipe::choose_random_recipe(&recipes, &args.include_tags, &args.exclude_tags)
            .unwrap_or_else(|failure| {
                eprintln!("Problem choosing random recipe: {failure}");
                process::exit(1)
            });
    println!("Chosen recipe: {chosen_recipe:?}");
}
