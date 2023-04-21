mod fs;

use atprs_lex::lexicon::LexUserType;
use atprs_lex::LexiconDoc;
use heck::ToSnakeCase;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{create_dir_all, read_dir, read_to_string, File};
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

pub fn genapi(lexdir: impl AsRef<Path>, outdir: impl AsRef<Path>, prefix: &str) -> Result<()> {
    let lexdir = lexdir.as_ref().canonicalize()?;
    let outdir = outdir.as_ref().canonicalize()?;
    let paths = fs::find_schemas(&lexdir)?;
    let mut schemas = Vec::with_capacity(paths.len());
    for path in &paths {
        schemas.push((read_to_string(path)?).parse::<LexiconDoc>()?);
    }
    let defmap = build_defmap(&schemas);
    for schema in schemas
        .iter()
        .filter(|schema| schema.id.starts_with(prefix))
    {
        generate_code(schema, &outdir, &defmap)?;
    }
    generate_modules(&outdir)?;
    Ok(())
}

fn build_defmap(schemas: &[LexiconDoc]) -> HashMap<String, &LexUserType> {
    let mut result = HashMap::new();
    for schema in schemas {
        for (name, def) in &schema.defs {
            let key = if name == "main" {
                schema.id.clone()
            } else {
                format!("{}#{}", schema.id, name)
            };
            assert!(!result.contains_key(&key), "duplicate key: {key}");
            result.insert(key, def);
        }
    }
    result
}

fn generate_code(
    schema: &LexiconDoc,
    outdir: &Path,
    _defmap: &HashMap<String, &LexUserType>,
) -> Result<()> {
    let mut paths = schema.id.split('.').collect::<Vec<_>>();
    if let Some(name) = paths.pop() {
        create_dir_all(outdir.join(paths.join("/")))?;
        let mut filename = PathBuf::from(name.to_snake_case());
        filename.set_extension("rs");

        let mut buf = Vec::new();
        writeln!(
            &mut buf,
            "// This file is generated by atprs-codegen. Do not edit"
        )?;
        // TODO
        File::create(outdir.join(paths.join("/")).join(filename))?.write_all(&buf)?;
    }
    Ok(())
}

fn generate_modules(outdir: &Path) -> Result<()> {
    let paths = fs::find_dirs(outdir)?;
    let mut hm = HashMap::new();
    // create ".rs" files
    for path in &paths {
        let mut p = path.to_path_buf();
        if path == outdir {
            p = p.join("lib.rs");
        } else {
            p.set_extension("rs");
        }
        hm.insert(path, File::create(&p)?);
    }
    // write "mod" statements
    for path in &paths {
        let mut modules = read_dir(path)?
            .filter(|entry| entry.as_ref().map_or(false, |e| e.path().is_file()))
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| e.path().file_stem().map(OsStr::to_owned))
            })
            .collect::<Vec<_>>();
        modules.sort();

        let mut buf = Vec::new();
        writeln!(
            &mut buf,
            "// This file is generated by atprs-codegen. Do not edit"
        )?;
        for module in &modules {
            if let Some(name) = module.to_str() {
                if path == outdir && name == "lib" {
                    continue;
                }
                writeln!(&mut buf, "pub mod {name};")?;
            }
        }
        if let Some(mut file) = hm.get(path) {
            file.write_all(&buf)?;
        }
    }
    Ok(())
}