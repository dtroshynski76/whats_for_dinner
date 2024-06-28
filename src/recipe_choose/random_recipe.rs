use crate::recipe_file::file_utils::Recipe;

use chrono::DateTime;
use regex::Regex;

pub fn choose_random_recipe<'a>(
    recipes: &'a [Recipe],
    include_tags: &[String],
    exclude_tags: &[String],
) -> Result<&'a Recipe, &'static str> {
    if recipes.is_empty() {
        return Err("Empty recipe list");
    }

    let mut filtered_recipes: Vec<&Recipe> = Vec::new();

    match (include_tags.is_empty(), exclude_tags.is_empty()) {
        (false, false) => filtered_recipes = both_tags_used(recipes, include_tags, exclude_tags),
        (false, true) => filtered_recipes = only_include_tags(recipes, include_tags),
        (true, false) => filtered_recipes = only_exclude_tags(recipes, include_tags),
        (true, true) => {}
    }

    println!("Filtered recipes: {filtered_recipes:?}");

    if filtered_recipes.is_empty() {
        return Err("Filtered out all recipes");
    }

    let element = fastrand::usize(..filtered_recipes.len());
    Ok(filtered_recipes[element])
}

fn both_tags_used<'a>(
    recipes: &'a [Recipe],
    include_tags: &[String],
    exclude_tags: &[String],
) -> Vec<&'a Recipe> {
    let mut difference: Vec<&String> = Vec::new();

    for x in include_tags {
        if !exclude_tags.contains(x) {
            difference.push(x);
        }
    }

    recipes
        .iter()
        .filter(|r| {
            for recipe_tag in &r.tags {
                if difference.contains(&recipe_tag) {
                    return true;
                }
            }

            false
        })
        .collect()
}

fn only_include_tags<'a>(recipes: &'a [Recipe], include_tags: &[String]) -> Vec<&'a Recipe> {
    recipes
        .iter()
        .filter(|r| {
            for recipe_tag in &r.tags {
                if include_tags.contains(recipe_tag) {
                    return true;
                }
            }

            false
        })
        .collect()
}

fn only_exclude_tags<'a>(recipes: &'a [Recipe], exclude_tags: &[String]) -> Vec<&'a Recipe> {
    recipes
        .iter()
        .filter(|r| {
            for recipe_tag in &r.tags {
                if exclude_tags.contains(recipe_tag) {
                    return false;
                }
            }

            true
        })
        .collect()
}

fn convert_time_tag_to_number(tag: &str) -> Result<i64, &'static str> {
    let regex = Regex::new(r"^(\d{1,2}h)?\d{2,3}m$");

    if let Ok(regex) = regex {
        let matched = regex.is_match(tag);

        match matched {
            true => {}
            false => return Err("Time tag does not conform to the pattern HH'h'MM'm'"),
        }

        let time = DateTime::parse_from_str(tag, "%Hh%Mm");

        return match time {
            Ok(t) => Ok(t.timestamp_millis()),
            Err(_) => Err("Failed to parse time tag"),
        };
    }

    Err("Regex pattern invalid")
}
