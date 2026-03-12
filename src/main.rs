mod error;
mod types;
mod pkg;

use crate::pkg::go_pkg::search_package;
use crate::pkg::installed_pkg::installed_pkg;

fn main(){
     let cli=types::cli::Cli::new();
    let target = cli.target;
    let limit = cli.limit;
    let old = cli.old;
    if old{
       if let Err(e)= installed_pkg(){
           eprintln!("{}", e);
       }
    }else{
        if let Some(target) = target{
            if let Err(e) = search_package(limit,&target){
                eprintln!("{}", e);
            }
        }else{
            eprintln!("==> error: 无效的参数");
            return;
        }
    }
}



