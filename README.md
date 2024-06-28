# Description

This project is intended to be used to decide what to make for dinner. In essence, to answer the question "What's for dinner?".

The goal is to achieve a program that can:

-   read dinner options, their recipes, and associated "tags" from a file
    -   "tags" can be anything, but the intent is for them to specify a food group, temperature, time to prepare, or "type" of food that recipe results in
        -   for example, spaghetti & meatballs could have the tags "carbs", "hot", "meat", "30m", etc.
        -   could also have a tag to denote if you have made that recipe before or not, e.g. "new" (haven't made before) or "tried" (have made before)
-   then, randomly select an option from that list
-   output that option to stdout with its name, tags, and recipe
-   support the CLI options:
    -   `--file`: path to the file containing dinner options to ingest; defaults to `./dinner_options.txt`
    -   `--include`: list of tags to include, excluding all others
    -   `--exclude`: list of tags to exclude, including all others
    -   if both `--include` and `--exclude` options are given, only those options with the tags that are the intersection of the two options will be returned
    -   if a given excluded or included tag denotes time (e.g. `30m`) and is prepended by a lesser or greater than operator - `<` or `>` - then the result will include or exclude those recipes whose times are correspondingly greater or lesser than the given tag
        -   additionally, the lesser than or equal to (`<=`) and greater than or equal to (`>=`) operators will also be supported

# Input File Format

```
[recipe]
name=Spaghetti & Meatballs
tags=hot,carbs,30m,dinner
ingredients=
1. spaghetti
2. meatballs
3. spaghetti sauce
steps=
1. boil water
2. put spaghetti in water
3. while cooking spaghettie, heat sauce with meatballs in it
4. drain water
5. put spaghetti on plate, top with sauce + meatballs

[recipe]
name=Baked
tags=1h30m,hot,carbs,meat,favorite
file=/path/to/recipe/baked.txt
```

# Example Usage

```bash
// randomly picks a recipe
whats_for_dinner recipes.txt

// randomly picks a recipe from the subset of recipes that have the "favorite" tag
whats_for_dinner recipes.txt --include favorite

// randomly picks a recipe from the subset of recipes that have the tags ("new" OR "30m" OR "hot")
whats_for_dinner recipes.txt --include new 30m hot

// randomly picks a recipe from the subset of recipes that have the tags ("new" OR "30m" OR "hot") AND NOT ("meat" OR "carbs")
whats_for_dinner recipes.txt --include new 30m hot --exclude meat carbs

// randomly picks a dinner whose tags include all those less than 30m but not equal to or greater than 30m
whats_for_dinner /path/to/recipes.txt --include <30m --exclude >=30m
```
