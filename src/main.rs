use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter};
use clap::Parser;
use inquire::{InquireError, MultiSelect};
use inquire::list_option::ListOption;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize};
#[derive(Debug, Clone)]
struct GoPkg{
    name: String,
    uri: String,
    description: Option<String>,
    version: Option<String>,
    imported: Option<i64>,
    published_on: Option<String>,
    is_installed: bool,
    installed_version: Option<String>,
}
#[derive(Debug,Clone,Deserialize)]
#[serde(rename_all = "PascalCase")]
struct OldPkg{
    path:String,
    #[serde(default)]
    version:Option<String>,

}
#[derive(Parser, Debug)]
#[command(name = "cmod", version = "1.0", about = "交互式 Go 包检索与安装工具")]
struct Cli {
    #[arg(required = true)]
    target: String,

    #[arg(short, long, default_value_t = 25)]
    limit: u64,
}
impl fmt::Display for GoPkg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, "  {}", self.uri)?;
        if let Some(ver) = &self.version {
            write!(f, " {}", ver)?;
        }
        if let Some(pub_date) = &self.published_on {
            write!(f, "  {}", pub_date)?;
        }
        if let Some(imp) = self.imported {
            write!(f, "  (+{})", imp)?;
        }
        if self.is_installed {
            if let Some(version) = &self.installed_version && !"".eq(version) {
                write!(f, "   (已安装  {}),", version)?;
            }else{
                write!(f, "   (已安装)")?;
            }
        }
        if let Some(desc) = &self.description {
            write!(f, "\n      {}", desc)?;
        }
        Ok(())
    }
}

 fn main(){
    let cli = Cli::parse();
    let target = cli.target;
    let limit = cli.limit;

    let output = std::process::Command::new("go")
        .arg("env")
        .arg("GOMOD")
        .output()
        .unwrap_or_else(|e|{
            eprintln!(" -> error: {}", e);
            std::process::exit(1);
        });
    let go_mod_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if go_mod_path == "/dev/null" || go_mod_path == "NUL" || go_mod_path.is_empty() {
        println!(" -> error: go.mod file not found");
        return;
    }
    let mut list=get_go_pkg_list(limit,&target).unwrap_or_else(|e| {
        eprintln!(" -> error: {}", e);
        std::process::exit(1);
    });
    if list.is_empty() {
        return;
    }
    let old_pkg=get_installed_pkg();
    let mut m:HashMap<String,Option<String>> = HashMap::new();
    for r in old_pkg{
        m.insert(r.path,r.version);
    }
    for i in list.iter_mut(){
        if let Some(res)=m.get(&i.uri) {
            i.is_installed = true;
            i.installed_version=res.clone();
        }
    }
    list.sort_by_key(|v|Reverse(v.imported));
    let selected_packages = MultiSelect::new(
        "==> 可安装的包 :\n",
        list,
    ).with_formatter(&|options: &[ListOption<&GoPkg>]| {
        let mut res:Vec<String> =Vec::new();
        for v in options.iter(){
            res.push(format!("  -> <{}> {} {}", v.index+1,v.value.name,v.value.uri));
        }
        res.join("\n")
    }).prompt();
    println!();
    let selected_packages = match selected_packages {
        Ok(choices) => {
            choices
        },
        Err(InquireError::OperationCanceled) => {
            println!(" -> 操作已取消.");
            std::process::exit(0);
        }
        Err(InquireError::OperationInterrupted) => {
            println!(" -> 操作被中断.");
            std::process::exit(130);
        }
        Err(err) => {
            eprintln!(" -> 交互界面发生错误: {}", err);
            std::process::exit(1);
        }
    };
    if selected_packages.is_empty() {
        println!(" -> 未选择任何包, 操作已取消.");
        return;
    }
    for pkg in selected_packages {
        println!("==> {} {} download...", pkg.name, pkg.uri);
        let status = std::process::Command::new("go")
            .arg("get")
            .arg(&pkg.uri)
            .status()
            .unwrap_or_else(|e|{
                eprintln!(" -> error: {}", e);
                std::process::exit(1);
            });
        if status.success() {
            println!(" -> {} install success...", pkg.name);
        }
        println!();
    }
}
fn get_installed_pkg() ->Vec<OldPkg> {
    let out = std::process::Command::new("go")
        .arg("list")
        .arg("-deps")
        .arg("-f")
        .arg(r#"{"Path":"{{.ImportPath}}","Version":"{{with .Module}}{{.Version}}{{end}}"}{{"\n"}}"#)
        .arg("./...")
        .output()
        .unwrap_or_else(|e|{
            eprintln!(" -> error: {}", e);
            std::process::exit(1);
        });
    let go_mod = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let old_pkg: Vec<OldPkg> = serde_json::Deserializer::from_str(&go_mod)
        .into_iter::<OldPkg>()
        .filter_map(|res| {
            match res {
                Ok(pkg) => Some(pkg),
                Err(e) => {
                    eprintln!(" -> warning: 解析包信息警告: {}", e);
                    None
                }
            }
        })
        .collect();

    old_pkg
}
 fn get_go_pkg_list(limit:u64,search:&str)->Result<Vec<GoPkg>, reqwest::Error>{
    let url = format!("https://pkg.go.dev/search?m=package&limit={}&q={}",limit,search);
     let client = reqwest::blocking::Client::builder()
         .build()?;

     let html_content = client.get(&url).send()?.text()?;
    let mut list:Vec<GoPkg>=Vec::new();
    let document = Html::parse_document(&html_content);
    let snippet_selector = Selector::parse(".SearchSnippet").unwrap();
    let name_selector = Selector::parse(r#"div.SearchSnippet-headerContainer > h2 > a[data-test-id="snippet-title"]"#).unwrap();
    let uri_selector = Selector::parse("span.SearchSnippet-header-path").unwrap();
    let description_selector = Selector::parse("p.SearchSnippet-synopsis").unwrap();
    let imported_selector = Selector::parse("div.SearchSnippet-infoLabel > a[aria-label=\"Go to Imported By\"] > strong").unwrap();
    let published_on_selector=Selector::parse(r#"div.SearchSnippet-infoLabel > .go-textSubtle > span[data-test-id="snippet-published"] > strong"#).unwrap();
    let version_selector=Selector::parse(r#"strong"#).unwrap();

    for snippet in document.select(&snippet_selector) {

        let Some(name_res) = snippet.select(&name_selector).next() else {
            continue;
        };
        let name = name_res.text().next()
            .unwrap_or("")
            .trim()
            .to_string();
        let Some(uri_res) = snippet.select(&uri_selector).next() else {
            continue;
        };
        let  uri = uri_res.text().collect::<String>().trim_start_matches('(').trim_end_matches(')').to_string();

        let description = snippet.select(&description_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let imported = snippet.select(&imported_selector)
            .next()
            .and_then(|el| el.text().collect::<String>().replace(",", "").trim().parse::<i64>().ok());
        let published_on = snippet.select(&published_on_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let version = snippet.select(&published_on_selector)
            .next()
            .and_then(|node| node.parent())
            .and_then(|node| node.parent())
            .and_then(ElementRef::wrap)
            .and_then(|parent_el| parent_el.select(&version_selector).next())
            .map(|el| el.text().collect::<String>().trim().to_string());


        list.push(GoPkg{name,uri,description,version,imported,published_on,is_installed:false,installed_version:None });
    }
    Ok(list)
}
