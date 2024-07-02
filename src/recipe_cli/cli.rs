use tracing::{debug_span, event, instrument, trace_span, Level};

#[derive(Debug)]
enum TagOption {
    Include,
    Exclude,
}

impl TagOption {
    #[instrument(name = "TagOption::build")]
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
        let _trace_span = trace_span!("Arguments::build trace").entered();
        let _debug_span = debug_span!("Arguments::build debug").entered();

        args.next();

        let file = args
            .next()
            .ok_or("Must have at least one argument")?
            .trim()
            .to_string();
        event!(Level::DEBUG, "File name: {file}");
        let mut include_tags: Vec<String> = Vec::new();
        let mut exclude_tags: Vec<String> = Vec::new();

        let second = args.next().unwrap_or(String::from("")).trim().to_string();
        event!(Level::DEBUG, "Second arg: {second}");

        if !second.is_empty() {
            event!(Level::TRACE, "Second argument is not empty");
            let mut tag_option = TagOption::build(second.as_str())?;

            for value in args {
                let value = value.trim().to_string();
                event!(Level::TRACE, "arg value: '{value}'");

                if value == tag_option.other().as_str() {
                    event!(Level::TRACE, "Previous tag option: {tag_option:?}");
                    tag_option = tag_option.other();
                    event!(Level::TRACE, "New tag option: {tag_option:?}");
                    continue;
                }

                match tag_option {
                    TagOption::Include => include_tags.push(value),
                    TagOption::Exclude => exclude_tags.push(value),
                }
                event!(Level::DEBUG, include_tags = ?include_tags, exclude_tags = ?exclude_tags);
            }
        }

        Ok(Arguments {
            file,
            include_tags,
            exclude_tags,
        })
    }
}
