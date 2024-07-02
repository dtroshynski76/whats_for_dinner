use tracing::instrument;

use crate::recipe_file::file_utils::Recipe;

#[instrument]
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

#[instrument]
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

#[instrument]
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

#[instrument]
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
