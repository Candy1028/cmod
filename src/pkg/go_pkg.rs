use std::cmp::Reverse;
use std::collections::HashMap;
use colored::Colorize;
use crate::error::error::Error::BizError;
use crate::error::error::Result;
use crate::pkg::loading::Loading;
use crate::pkg::tui::{new_multiple_choice};
use crate::types;

pub fn search_package(limit:u64, target:&str,pb :&mut Loading) ->Result<()>{
    let output = std::process::Command::new("go")
        .arg("env")
        .arg("GOMOD")
        .output()?;
    let go_mod_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if go_mod_path == "/dev/null" || go_mod_path == "NUL" || go_mod_path.is_empty() {
        return Err(BizError("go.mod file not found".to_string()));
    }
    let mut list=types::go_pkg::GoPkg::list(limit,target)?;
    if list.is_empty() {
        return Ok(());
    }
    let installed_pkg=types::installed_pkg::InstalledPkg::list()?;
    let mut m:HashMap<String,Option<String>> = HashMap::new();
    for r in installed_pkg{
        m.insert(r.path,r.version);
    }
    for i in list.iter_mut(){
        if let Some(res)=m.get(&i.uri) {
            i.is_installed = true;
            i.installed_version=res.clone();
        }
    }
    list.sort_by_key(|v|Reverse(v.imported));
    pb.final_loading();
    let selected_packages=new_multiple_choice(&list);

    println!();
    let selected_packages = match selected_packages {
        Ok(choices) => {
            choices
        },

        Err(err) => {
            eprintln!("{}",format!("==> 交互界面发生错误 : {}", err).green());
            std::process::exit(1);
        }
    };
    if selected_packages.is_empty() {
        println!("{}","==> 未选择任何包, 操作已取消.".green());
        return Ok(());
    }
    let mut  index=1;
    for v in selected_packages.iter(){
        println!("==> <{}> {} {}", index,v.name,v.uri);
        index+=1;
    }
    for pkg in selected_packages {
        println!("{}",format!("==> {} {} download...", pkg.name, pkg.uri).green());
        let status = std::process::Command::new("go")
            .arg("get")
            .arg(&pkg.uri)
            .status()?;
        if status.success() {
            println!("{}",format!(" -> {} install success!", pkg.name).green());
        }
        println!();
    }
    println!("{}","==> Done.".green());
    Ok(())
}
