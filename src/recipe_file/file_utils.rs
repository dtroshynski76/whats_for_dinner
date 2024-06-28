const NAME: &str = "name";
const TAGS: &str = "tags";
const INGREDIENTS: &str = "ingredients";
const STEPS: &str = "steps";
const FILE: &str = "file";

#[derive(Debug)]
pub struct Recipe {
    pub name: String,
    pub tags: Vec<String>,
    pub ingredients: Vec<String>,
    pub steps: Vec<String>,
    pub file: Option<String>,
}

impl Default for Recipe {
    fn default() -> Self {
        Self::new()
    }
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe {
            name: String::new(),
            tags: Vec::new(),
            ingredients: Vec::new(),
            steps: Vec::new(),
            file: None,
        }
    }
}

pub fn parse_recipe_file(recipe_file_contents: String) -> Result<Vec<Recipe>, &'static str> {
    if recipe_file_contents.trim().is_empty() {
        return Err("Empty recipe file contents");
    }

    if !recipe_file_contents.contains("[recipe]") {
        return Err("Does not contain [recipe] header(s)");
    }

    let mut parsed: Vec<Recipe> = Vec::new();

    for chunk in recipe_file_contents.split("[recipe]\n") {
        let chunk = chunk.trim();

        if chunk.is_empty() {
            continue;
        }

        let recipe = parse_recipe_chunk(chunk)?;

        parsed.push(recipe);
    }

    Ok(parsed)
}

fn parse_recipe_chunk(chunk: &str) -> Result<Recipe, &'static str> {
    let chunk_lines = chunk.lines();
    let mut result = Recipe::new();

    let mut previous_title = "";
    for line in chunk_lines {
        let line_split: Vec<&str> = line.split('=').collect();

        if line_split.is_empty() {
            continue;
        }

        if line_split.len() == 1 {
            // must be a continuation of the previous title
            let intermediate_line = line_split[0];

            if intermediate_line.trim().is_empty() {
                continue;
            }

            update_result(&mut result, previous_title, line_split[0].trim())?;
            continue;
        }

        previous_title = line_split[0].trim();
        let value = line_split[1].trim();

        if value.trim().is_empty() {
            continue;
        }

        update_result(&mut result, previous_title, value)?;
    }

    Ok(result)
}

fn update_result(result: &mut Recipe, title: &str, value: &str) -> Result<(), &'static str> {
    let value = String::from(value);

    match title {
        NAME => result.name = value,
        TAGS => {
            result.tags = value.split(',').map(String::from).collect();
        }
        INGREDIENTS => {
            result.ingredients.push(value);
        }
        STEPS => {
            result.steps.push(value);
        }
        FILE => result.file = Some(value),
        _ => {
            return Err("Unknown title");
        }
    }

    Ok(())
}
