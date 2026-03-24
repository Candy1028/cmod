use std::fmt;
use std::fmt::Formatter;
use scraper::{ElementRef, Html, Selector};
use crate::error::error::Error::BizError;
use crate::error::error::Result;
use crate::types::tui::CheckedInfo;

#[derive(Debug, Clone)]
// 从远程获取go pkg
pub struct GoPkg{
    pub name: String,
    pub uri: String, //路径
    pub description: Option<String>, //简介
    pub version: Option<String>, //版本
    pub imported: Option<i64>, //下载量
    pub published_on: Option<String>, //公开时间
    pub is_installed: bool, //是否安装
    pub installed_version: Option<String>, // 已安装版本
}
impl CheckedInfo for GoPkg{
    fn info(&self) -> &str{
        self.uri.as_str()
    }
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
impl GoPkg {
    pub fn list(limit:u64,search:&str)->Result<Vec<GoPkg>>{
        let url = format!("https://pkg.go.dev/search?m=package&limit={}&q={}",limit,search);
        let client = reqwest::blocking::Client::builder()
            .build()?;

        let html_content = client.get(&url).send()?.text()?;
        let mut list:Vec<GoPkg>=Vec::new();
        let document = Html::parse_document(&html_content);
        let snippet_selector = Selector::parse(".SearchSnippet")
            .map_err(|_| BizError("Selector 发生错误".to_string()))?;
        let name_selector = Selector::parse(r#"div.SearchSnippet-headerContainer > h2 > a[data-test-id="snippet-title"]"#)
            .map_err(|_| BizError("Selector 发生错误".to_string()))?;
        let uri_selector = Selector::parse("span.SearchSnippet-header-path")
            .map_err(|_| BizError("Selector 发生错误".to_string()))?;
        let description_selector = Selector::parse("p.SearchSnippet-synopsis")
            .map_err(|_| BizError("Selector 发生错误".to_string()))?;
        let imported_selector = Selector::parse("div.SearchSnippet-infoLabel > a[aria-label=\"Go to Imported By\"] > strong")
            .map_err(|_| BizError("Selector 发生错误".to_string()))?;
        let published_on_selector=Selector::parse(r#"div.SearchSnippet-infoLabel > .go-textSubtle > span[data-test-id="snippet-published"] > strong"#)
            .map_err(|_| BizError("Selector 发生错误".to_string()))?;
        let version_selector=Selector::parse(r#"strong"#)
            .map_err(|_| BizError("Selector 发生错误".to_string()))?;

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
}