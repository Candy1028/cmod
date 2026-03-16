use std::cmp::Reverse;
use std::collections::HashMap;
use colored::Colorize;
use inquire::list_option::ListOption;
use inquire::{InquireError, MultiSelect};
use crate::error::error::Error::BizError;
use crate::error::error::Result;
use crate::pkg::loading::Loading;
use crate::types;
use crate::types::go_pkg::GoPkg;

pub fn search_package(limit:u64, target:&str,pb :&mut Loading) ->Result<()>{
    let output = std::process::Command::new("go")
        .arg("env")
        .arg("GOMOD")
        .output()?;
    let go_mod_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if go_mod_path == "/dev/null" || go_mod_path == "NUL" || go_mod_path.is_empty() {
        return Err(BizError("go.mod file not found"));
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
    let selected_packages = MultiSelect::new(
        "==> 可安装的包 :\n",
        list,
    ).with_formatter(&|options: &[ListOption<&GoPkg>]| {
        let mut res:Vec<String> =Vec::new();
        let mut index:usize = 1;
        for v in options.iter(){
            res.push(format!("  -> <{}> {} {}", index,v.value.name,v.value.uri));
            index+=1;
        }
        res.join("\n")
    }).prompt();
    println!();
    let selected_packages = match selected_packages {
        Ok(choices) => {
            choices
        },
        Err(InquireError::OperationCanceled) => {
            println!("{}","==> 操作已取消 .".yellow());
            std::process::exit(0);
        }
        Err(InquireError::OperationInterrupted) => {
            println!("{}","==> 操作被中断 .".yellow());
            std::process::exit(130);
        }
        Err(err) => {
            eprintln!("{}",format!("==> 交互界面发生错误 : {}", err).green());
            std::process::exit(1);
        }
    };
    if selected_packages.is_empty() {
        println!("{}","==> 未选择任何包, 操作已取消.".green());
        return Ok(());
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
