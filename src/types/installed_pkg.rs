use std::fmt;
use std::fmt::Formatter;
use serde::Deserialize;
use crate::error::error::Result;

#[derive(Debug,Clone,Deserialize)]
#[serde(rename_all = "PascalCase")]
// 已经安装的Pkg
pub struct InstalledPkg{
    pub path:String, // 路径 及 git地址
    #[serde(default)]
    pub version:Option<String>, // 已安装版本

}
impl fmt::Display for InstalledPkg{
    fn fmt(&self, f: &mut Formatter<'_>) ->fmt::Result {
        write!(f,"{}   ",self.path)?;
        if let Some(v)=&self.version{
            if v!=""{
                write!(f,"version: {}",v)?;
            }
        }
        Ok(())
    }
}
impl InstalledPkg{
    // 获取已安装包列表
    pub fn list()->Result<Vec<Self>>{
        let out = std::process::Command::new("go")
            .arg("list")
            .arg("-deps")
            .arg("-f")
            .arg(r#"{"Path":"{{.ImportPath}}","Version":"{{with .Module}}{{.Version}}{{end}}"}{{"\n"}}"#)
            .arg("./...")
            .output()?;
        let go_mod = String::from_utf8_lossy(&out.stdout).trim().to_string();
        Ok(serde_json::Deserializer::from_str(&go_mod)
            .into_iter::<Self>()
            .filter_map(|res| {
                match res {
                    Ok(pkg) => Some(pkg),
                    Err(e) => {
                        eprintln!(" -> warning: 解析包信息警告: {}", e);
                        None
                    }
                }
            })
            .collect())
    }
}