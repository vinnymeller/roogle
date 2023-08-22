use std::path::Path;

use crate::common::UserQuery;
use crate::parse::get_fuctions_from_paths;

use clap::Parser;
use distance::levenshtein;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file or directory to parse. If not provided, will attempt to search for Rust source files
    #[clap(short, long)]
    path: Vec<String>,

    // Arguments and their types to search for. Can be provided multiple times.
    #[clap(short, long)]
    arg_type: Option<Vec<String>>,

    // Return type to search for.
    #[clap(short, long)]
    return_type: Option<String>,

    // Number of results to display
    #[clap(short, long, default_value = "10")]
    n_results: usize,
}

pub fn parse() {
    let cli = Cli::parse();
    // from the cli input, parse this into something more standardized we can use later
    let user_fn = UserQuery::builder()
        .args(cli.arg_type)
        .return_type(cli.return_type)
        .build();

    // now that we have that, find all `.rs` files in the path(s) provided
    let functions = get_fuctions_from_paths(cli.path.iter().map(Path::new).collect::<Vec<_>>());

    // fuck around with the first function to figure out how to do some of the replacements i need
    let myfn = functions.first().unwrap();
    
    // how do i unify the output format of 

    // for each function, put it through `UserFunction.comparable_string(other: Function) -> &str` and pass that to levenshtein distance
    let user_fn_str = user_fn.to_string();
    let mut dists = functions
        .iter()
        .map(|f| {
            (
                levenshtein(&user_fn_str, &user_fn.comparable_func_str(f)),
                f,
            )
        })
        .collect::<Vec<_>>();

    // sort dists by distance
    dists.sort_by(|a, b| a.0.cmp(&b.0));
    // display the top n results to the user
    dists.iter().take(cli.n_results).for_each(|(dist, f)| {
        println!("{} {}: {}", f.file.to_string_lossy(), dist, f);
    });
}
