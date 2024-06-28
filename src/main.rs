use std::{env, fs, process};

use whats_for_dinner::{recipe_choose, recipe_cli::Arguments, recipe_file};

fn main() {
    let args = Arguments::build(env::args()).unwrap_or_else(|failure| {
        eprintln!("Problem building arguments struct: {failure}");
        process::exit(1)
    });

    let file_contents = fs::read_to_string(&args.file).unwrap_or_else(|err| {
        eprintln!("Problem opening recipes file '{}': {err}", &args.file);
        process::exit(1)
    });

    let recipes = recipe_file::parse_recipe_file(file_contents).unwrap_or_else(|failure| {
        eprintln!("Problem parsing recipes: {failure}");
        process::exit(1)
    });

    let chosen_recipe =
        recipe_choose::choose_random_recipe(&recipes, &args.include_tags, &args.exclude_tags)
            .unwrap_or_else(|failure| {
                eprintln!("Problem choosing random recipe: {failure}");
                process::exit(1)
            });
    println!("Chosen recipe: {chosen_recipe:?}");
}
