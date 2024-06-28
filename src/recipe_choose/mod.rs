use crate::recipe_file::Recipe;

pub fn choose_random_recipe<'a>(
    recipes: &'a [Recipe],
    include_tags: &[String],
    exclude_tags: &[String],
) -> Result<&'a Recipe, &'static str> {
    if recipes.is_empty() {
        return Err("Empty recipe list");
    }

    // TODO: get random recipe, respecting the include and exclude tags

    let element = fastrand::usize(..recipes.len());
    Ok(&recipes[element])
}
