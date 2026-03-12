use inquire::Select;
use crate::types;
use crate::error::*;
pub fn installed_pkg()->error::Result<()>{
    let res=types::installed_pkg::InstalledPkg::list()?;
    if res.is_empty(){
        println!("==> 未安装任何包");
        return Ok(());
    }
    Select::new("installed: \n", res)
        .prompt()?;
    Ok(())
}