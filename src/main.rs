use std::ffi::OsString;
use std::fmt;
use std::fmt::{Formatter};
use inquire::{InquireError, MultiSelect};
use inquire::list_option::ListOption;
use pico_args::Arguments;
use scraper::{ElementRef, Html, Selector};
#[derive(Debug, Clone)]
struct GoPkg{
    name: String,
    uri: String,
    description: Option<String>,
    version: Option<String>,
    imported: Option<i64>,
    published_on: Option<String>,
}
impl fmt::Display for GoPkg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, "  {}", self.uri)?;
        if let Some(ver) = &self.version {
            write!(f, " {}", ver)?;
        }

        if let Some(imp) = self.imported {
            write!(f, " ({})", imp)?;
        }

        if let Some(pub_date) = &self.published_on {
            write!(f, " {}", pub_date)?;
        }

        if let Some(desc) = &self.description {
            write!(f, "\n    {}", desc)?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main(){
    let mut args = Arguments::from_env();

    let option_limit: Option<u64> = args.opt_value_from_str(["-l", "--limit"]).unwrap_or(Some(25));

    let target: String = args.free_from_str().expect("无效参数");
    let remaining: Vec<OsString> = args.finish();
    if !remaining.is_empty() {
        eprintln!("warning: 存在未识别的额外参数: {:?}", remaining);
    }
    let mut limit =25;

    if let Some(res) = option_limit {
         limit=res;
    }

    let list=get_go_pkg_list(limit,&target).await.expect("获取错误");
    if list.is_empty() {
        println!("没有找到相关的包.");
        return;
    }
    let selected_packages = MultiSelect::new(
        "请选择你需要安装的 (按空格键勾选, 按回车键确认):",
        list,
    ).with_formatter(&|_: &[ListOption<&GoPkg>]| {
        String::new()
    }).prompt();
    println!("\n");
    let selected_packages = match selected_packages {
        Ok(choices) => {
            choices
        },

        Err(InquireError::OperationCanceled) => {
            println!(" 操作已取消.");
            std::process::exit(0); // 正常退出
        }
        Err(InquireError::OperationInterrupted) => {
            println!("\n 操作被中断.");
            std::process::exit(130);
        }

        Err(err) => {
            eprintln!("交互界面发生错误: {}", err);
            std::process::exit(1);
        }
    };
    if selected_packages.is_empty() {
        println!("未选择任何包, 操作已取消.");
        return;
    }
    for pkg in selected_packages {
        println!("==>> {} {}\n", pkg.name, pkg.uri);
        let status = std::process::Command::new("go")
            .arg("get")
            .arg(&pkg.uri)
            .status()
            .expect("Error");

        if status.success() {
            println!("   {} Success...", pkg.name);
        }

    }
}
async fn get_go_pkg_list(limit:u64,search:&str)->Result<Vec<GoPkg>, reqwest::Error>{
    let url = format!("https://pkg.go.dev/search?m=package&limit={}&q={}",limit,search);

    let html_content = reqwest::get(url).await?.text().await?;
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


        list.push(GoPkg{name,uri,description,version,imported,published_on});
    }
    Ok(list)
}
