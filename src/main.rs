mod error;
mod types;
mod pkg;

use colored::Colorize;
use crate::error::error::Error::BizError;
use crate::pkg::go_pkg::search_package;
use crate::pkg::installed_pkg::installed_pkg;
use crate::types::cli;
use crate::error::error::Result;
use crate::pkg::loading::Loading;
fn main(){
     let cli=cli::Cli::new();
    let ref mut pb=Loading::new();
    if let Err(e)=choose(&cli,pb){
        pb.final_loading();
        println!("{}",format!("{}",e).red());
    }else{
        pb.final_loading();
    }
}
fn choose(cli:&cli::Cli,pb :&mut Loading)->Result<()>{
    let target = &cli.target;
    let limit = cli.limit;
    let old = cli.old;
    if old{
         installed_pkg(pb)
    }else{
        if let Some(target) = target{
            search_package(limit,&target,pb)
        }else{
            Err(BizError("==> error: 无效的参数"))
        }
    }
}



