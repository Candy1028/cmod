use colored::Colorize;
use inquire::Select;
use crate::types;
use crate::error::*;
use crate::pkg::loading::Loading;

pub fn installed_pkg(pb :&mut Loading) ->error::Result<()>{
    let res=types::installed_pkg::InstalledPkg::list()?;

    if res.is_empty(){
        println!("{}","==> 未安装任何包".green());
        return Ok(());
    }
    pb.final_loading();
    Select::new("installed: \n", res)
        .prompt()?;
    Ok(())
}