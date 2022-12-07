use common::parse_day_num;
use proc_macro::TokenStream;
use quote::quote;
use std::{env, fs};
use syn::{Block, ExprMatch, Ident};

mod common;

const SOLUTION_FOLDER: &str = "solutions";
const INPUT_FOLDER: &str = "inputs";
const EXAMPLE_INPUT_FOLDER: &str = "example-inputs";

#[proc_macro]
pub fn import_solution_modules(_item: TokenStream) -> TokenStream {
    let file_stems = get_solution_file_stems();
    let module_decl_string = get_module_contents(&file_stems);
    let soln_folder_ident: Ident = syn::parse_str(SOLUTION_FOLDER).expect("Invalid ident");
    let module_decl_block: Block =
        syn::parse_str(&module_decl_string).expect("Invalid mod declaration block");
    let match_expr_string = get_match_expr_string(&file_stems);
    let match_expr: ExprMatch =
        syn::parse_str(&match_expr_string).expect("Invalid match expression");

    let token = quote!(
        mod #soln_folder_ident #module_decl_block

        fn solution_runner(day_num: u32, use_example_input: bool) -> (String, String) {
            let current_dir = env::current_dir().unwrap();
            let input_folder = if use_example_input {
                #EXAMPLE_INPUT_FOLDER
            } else {
                #INPUT_FOLDER
            };
            let input_file_name = if day_num < 10 {
                format!("day0{}.txt", day_num)
            } else {
                format!("day{}.txt", day_num)
            };
            let input_file_path = current_dir.join(input_folder).join(input_file_name);

            let input = fs::read_to_string(input_file_path).unwrap();

            #match_expr
        }
    );
    TokenStream::from(token)
}

fn get_solution_file_stems() -> Vec<String> {
    let cdir = env::current_dir()
        .unwrap()
        .join("src")
        .join(SOLUTION_FOLDER);

    let res = fs::read_dir(cdir).unwrap();
    res.filter_map(|x| {
        let entry = x.as_ref().unwrap();
        let path = entry.path();
        let fstem = path.file_stem().unwrap().to_str().unwrap();
        let ext = path.extension().unwrap();
        if ext == "rs" && fstem != "mod" {
            Some(String::from(fstem))
        } else {
            None
        }
    })
    .collect::<Vec<_>>()
}

fn get_module_contents(file_stems: &Vec<String>) -> String {
    let contents: String = file_stems
        .iter()
        .map(|x| format!("pub mod {x}; "))
        .collect();
    format! {"{{ {contents} }}"}
}

fn get_match_expr_string(file_stems: &Vec<String>) -> String {
    let contents: String = file_stems
        .iter()
        .map(|x| {
            let num = parse_day_num(x).expect("Invalid file name");
            format!(
                "{} => ({}::{}::part1(&input), {}::{}::part2(&input)), ",
                num, SOLUTION_FOLDER, x, SOLUTION_FOLDER, x
            )
        })
        .collect();
    format! {"match day_num {{ {contents} _ => (String::new(),String::new()),}}"}
}
