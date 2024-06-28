enum TagOption {
    Include,
    Exclude,
}

impl TagOption {
    pub fn build(input: &str) -> Result<TagOption, &'static str> {
        match input {
            "--include" => Ok(TagOption::Include),
            "--exclude" => Ok(TagOption::Exclude),
            _ => Err("Unknown option"),
        }
    }

    pub fn other(&self) -> TagOption {
        match self {
            TagOption::Include => TagOption::Exclude,
            TagOption::Exclude => TagOption::Include,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TagOption::Include => "--include",
            TagOption::Exclude => "--exclude",
        }
    }
}

#[derive(Debug)]
pub struct Arguments {
    pub file: String,
    pub include_tags: Vec<String>,
    pub exclude_tags: Vec<String>,
}

impl Arguments {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Arguments, &'static str> {
        args.next();

        let file = args
            .next()
            .ok_or("Must have at least one argument")?
            .trim()
            .to_string();
        let mut include_tags: Vec<String> = Vec::new();
        let mut exclude_tags: Vec<String> = Vec::new();

        let second = args.next().unwrap_or(String::from("")).trim().to_string();

        if !second.is_empty() {
            let mut tag_option = TagOption::build(second.as_str())?;

            for value in args {
                let value = value.trim().to_string();

                if value == tag_option.other().as_str() {
                    tag_option = tag_option.other();
                    continue;
                }

                match tag_option {
                    TagOption::Include => include_tags.push(value),
                    TagOption::Exclude => exclude_tags.push(value),
                }
            }
        }

        Ok(Arguments {
            file,
            include_tags,
            exclude_tags,
        })
    }
}
