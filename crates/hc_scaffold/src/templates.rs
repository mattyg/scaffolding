use build_fs_tree::serde::Serialize;
use convert_case::{Case, Casing};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use handlebars::{handlebars_helper, Handlebars};
use regex::Regex;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::{dir_content, find_files, flatten_file_tree, unflatten_file_tree, FileTree};

pub mod get;

// pub mod dna;
pub mod entry_type;
pub mod index;
pub mod link_type;
pub mod web_app;
// pub mod zome;
// pub mod zome_function;

pub fn build_handlebars<'a>(templates_dir: &FileTree) -> ScaffoldResult<Handlebars<'a>> {
    let h = Handlebars::new();

    let mut h = register_helpers(h);

    let field_types_path = PathBuf::from("field-types");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(field_types_templates) = templates_dir.path(&mut v.iter()) {
        h = register_all_partials_in_dir(h, field_types_templates)?;
    }
    h.register_escape_fn(handlebars::no_escape);

    Ok(h)
}

pub fn register_helpers<'a>(h: Handlebars<'a>) -> Handlebars<'a> {
    let h = register_concat_helper(h);
    let h = register_contains_helper(h);
    let h = register_case_helpers(h);

    h
}

pub fn register_concat_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(concat: |s1: String, s2: String| format!("{}{}", s1, s2));
    h.register_helper("concat", Box::new(concat));

    h
}

pub fn register_contains_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(contains: |list: Option<Vec<Value>>, value: Value| list.is_some() && list.unwrap().contains(&value));
    h.register_helper("contains", Box::new(contains));

    h
}

pub fn register_case_helpers<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(title_case: |s: String| s.to_case(Case::Title));
    h.register_helper("title_case", Box::new(title_case));

    handlebars_helper!(snake_case: |s: String| s.to_case(Case::Snake));
    h.register_helper("snake_case", Box::new(snake_case));

    handlebars_helper!(kebab_case: |s: String| s.to_case(Case::Kebab));
    h.register_helper("kebab_case", Box::new(kebab_case));

    handlebars_helper!(camel_case: |s: String| s.to_case(Case::Camel));
    h.register_helper("camel_case", Box::new(camel_case));

    handlebars_helper!(pascal_case: |s: String| s.to_case(Case::Pascal));
    h.register_helper("pascal_case", Box::new(pascal_case));

    h
}

pub fn register_all_partials_in_dir<'a>(
    mut h: Handlebars<'a>,
    file_tree: &FileTree,
) -> ScaffoldResult<Handlebars<'a>> {
    let partials = find_files(file_tree, &|path, _contents| {
        if let Some(e) = PathBuf::from(path).extension() {
            if e == "hbs" {
                return true;
            }
        }
        return false;
    });

    for (path, content) in partials {
        h.register_partial(
            path.with_extension("").as_os_str().to_str().unwrap(),
            content,
        )?;
    }

    Ok(h)
}

pub fn render_template_file_tree<'a, T: Serialize>(
    h: &Handlebars<'a>,
    templates_file_tree: &FileTree,
    data: &T,
) -> ScaffoldResult<FileTree> {
    let flattened_templates = flatten_file_tree(templates_file_tree);

    let mut transformed_templates: BTreeMap<PathBuf, String> = BTreeMap::new();

    for (path, contents) in flattened_templates {
        let path = PathBuf::from(path.to_str().unwrap().replace('¡', "/"));

        let re = Regex::new(
            r"(?P<c>(.)*)/\{\{#each (?P<b>([^\{\}])*)\}\}(?P<a>(.)*).hbs\{\{/each\}\}\z",
        )
        .unwrap();

        if re.is_match(path.to_str().unwrap()) {
            let path_prefix = re.replace(path.to_str().unwrap(), "${c}");
            let path_prefix = h.render_template(path_prefix.to_string().as_str(), data)?;

            let new_path_suffix =
                re.replace(path.to_str().unwrap(), "{{#each ${b} }}${a}.hbs{{/each}}");

            let all_paths = h.render_template(new_path_suffix.to_string().as_str(), data)?;

            let files_to_create: Vec<String> = all_paths
                .split(".hbs")
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect();

            for (i, f) in files_to_create.into_iter().enumerate() {
                let new_data = serde_json::to_string(data)?;
                let mut value: serde_json::Value = serde_json::from_str(new_data.as_str())?;

                value
                    .as_object_mut()
                    .unwrap()
                    .insert(String::from("index"), json!(i));

                let new_contents = h.render_template(contents.as_str(), &value)?;
                transformed_templates
                    .insert(PathBuf::from(path_prefix.clone()).join(f), new_contents);
            }
        } else if let Some(e) = path.extension() {
            if e == "hbs" {
                let new_path = h.render_template(path.as_os_str().to_str().unwrap(), data)?;
                let new_contents = h.render_template(contents.as_str(), data)?;

                transformed_templates
                    .insert(PathBuf::from(new_path).with_extension(""), new_contents);
            }
        }
    }

    unflatten_file_tree(&transformed_templates)
}

pub fn render_template_file_tree_and_merge_with_existing<'a, T: Serialize>(
    app_file_tree: FileTree,
    h: &Handlebars<'a>,
    template_file_tree: &FileTree,
    data: &T,
) -> ScaffoldResult<FileTree> {
    let rendered_templates = render_template_file_tree(h, template_file_tree, data)?;

    let mut flattened_app_file_tree = flatten_file_tree(&app_file_tree);
    let flattened_templates = flatten_file_tree(&rendered_templates);

    flattened_app_file_tree.extend(flattened_templates);

    unflatten_file_tree(&flattened_app_file_tree)
}

pub fn templates_path() -> PathBuf {
    PathBuf::from(".templates")
}

pub fn choose_or_get_template_file_tree(
    file_tree: &FileTree,
    template: &Option<String>,
) -> ScaffoldResult<FileTree> {
    let template_name = choose_or_get_template(file_tree, template)?;

    Ok(FileTree::Directory(dir_content(
        &file_tree,
        &templates_path().join(template_name),
    )?))
}

pub fn choose_or_get_template(
    file_tree: &FileTree,
    template: &Option<String>,
) -> ScaffoldResult<String> {
    let templates_path = PathBuf::new().join(templates_path());

    let templates_dir_content =
        dir_content(file_tree, &templates_path).map_err(|_e| ScaffoldError::NoTemplatesFound)?;

    let templates: Vec<String> = templates_dir_content
        .into_keys()
        .map(|k| k.to_str().unwrap().to_string())
        .collect();

    let chosen_template_name = match (template, templates.len()) {
        (_, 0) => Err(ScaffoldError::NoTemplatesFound),
        (None, 1) => Ok(templates[0].clone()),
        (None, _) => {
            let option = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which template should we use?")
                .default(0)
                .items(&templates[..])
                .interact()?;

            Ok(templates[option].clone())
        }
        (Some(t), _) => match templates.contains(&t) {
            true => Ok(t.clone()),
            false => Err(ScaffoldError::TemplateNotFound(t.clone())),
        },
    }?;

    Ok(chosen_template_name)
}
