use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use glob::glob;
use regex::Regex;
use syn::parse_quote;

fn main() -> eyre::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/solutions");

    let attr_re = Regex::new(r#"aoc\((.+)\)\]"#).unwrap();
    let kv_re = Regex::new(r#"(?P<key>(year|day|part|version|separator))\s*=\s*\"?(?P<value>[a-zA-Z0-9_\s\\]+)\"?"#).unwrap();
    let mut refs = Vec::new();
    let mut modules = HashMap::new();

    for entry in glob("src/solutions/**/*.rs").expect("failed to read glob pattern") {
        if let Ok(file) = entry {
            for l in BufReader::new(File::open(file.clone())?).lines() {
                let l = l?;
                if let Some(m) = attr_re.captures(&l) {
                    let attr = m.get(1).unwrap().as_str();
                    let mut kv: HashMap<&str, &str> = HashMap::new();

                    for m in kv_re.captures_iter(attr) {
                        let key = m.name("key").unwrap().as_str();
                        let value = m.name("value").unwrap().as_str();
                        let entry = kv.entry(key).or_insert(value);
                        *entry = value;
                    }

                    if !kv.contains_key("year") || !kv.contains_key("day") || !kv.contains_key("part") {
                        eprintln!("cannot register runner without setting year, day, or part");
                        continue;
                    }

                    let year: usize = kv["year"].parse()?;
                    let day: usize = kv["day"].parse()?;
                    let part = kv["part"];
                    let version = kv
                        .get("version")
                        .map(|v| *v);
                    let (version, extension): (syn::Expr, &str) = match version {
                        Some(v) => (parse_quote!(Some(String::from(#v))), v),
                        None => (parse_quote!(None), "none"),
                    };
                    let year_mod_name: syn::Ident = syn::parse_str(&file
                        .parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_owned()
                        .into_string()
                        .unwrap()
                    )?;
                    let mod_name: syn::Ident = syn::parse_str(file
                        .file_name()
                        .unwrap()
                        .to_owned()
                        .into_string()
                        .unwrap()
                        .strip_suffix(".rs")
                        .unwrap()
                    )?;
                    let runner_name: syn::Ident =
                        syn::parse_str(&format!("runner_y{year}_day{day:02}_part{part}_{extension}"))?;
                    let stmt: syn::Stmt = parse_quote! {
                        crate::register_runner(#year, #day, #part, #version, crate::solutions::#year_mod_name::#mod_name::#runner_name);
                    };
                    refs.push(stmt);

                    let entry = modules
                        .entry(year_mod_name)
                        .or_insert_with(|| HashSet::new());
                    entry.insert(mod_name);
                }
            }
        }
    }

    let current_dir = std::env::current_dir().unwrap();
    // TODO: Rewrite the way that years is written to file, so it can be sorted.
    // Use common function to write collection of syn::Stmt to file?
    let mut years = Vec::new();
    for (year, days) in modules.into_iter() {
        let year_dir_name = format!(
            "{}/src/solutions/{}",
            current_dir.display(),
            year.to_string(),
        );
        let year_dir = Path::new(&year_dir_name);
        if !year_dir.exists() {
            fs::create_dir_all(year_dir)?;
        }

        let mut days = days
            .into_iter()
            .collect::<Vec<_>>();
        days.sort_by_cached_key(|i| i.to_string());
        let days: Vec<syn::Stmt> = days
            .into_iter()
            .map(|day| parse_quote! {
                pub(crate) mod #day;
            })
            .collect();

        let year_mod = year_dir.join("mod.rs");
        let days: syn::File = parse_quote! {
            #(#days)*
        };
        fs::write(year_mod, &prettyplease::unparse(&days))?;

        let year: syn::Stmt = parse_quote! {
            pub(crate) mod #year;
        };
        years.push(year);
    }

    let solution_mod_name = format!(
        "{}/src/solutions/mod.rs",
        current_dir.display(),
    );
    let solution_mod = Path::new(&solution_mod_name);
    let years: syn::File = parse_quote! {
        #(#years)*
    };
    fs::write(solution_mod, &prettyplease::unparse(&years))?;

    let register: syn::File = parse_quote! {
        #[allow(unused_must_use)]
        pub fn register_runners() {
            #(#refs)*
        }
    };
    fs::write("src/register.rs", &prettyplease::unparse(&register))?;

    Ok(())
}
