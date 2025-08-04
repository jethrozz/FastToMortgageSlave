use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChengjiaoData {
    pub title: String,
    pub deal_price: String,
    pub list_price: String,
    pub deal_date: String,
    pub is_renovated: String,
    pub deal_cycle: String,
}

pub struct ChengjiaoScraper {
    client: Client,
    cookies: String,
}

impl ChengjiaoScraper {
    pub fn new(cookies: String) -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .unwrap();
        
        Self { client, cookies }
    }

    pub async fn scrape_chengjiao_page(&self, url: &str) -> Result<Vec<ChengjiaoData>> {
        let response = self.client
            .get(url)
            .header("Cookie", &self.cookies)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Connection", "keep-alive")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .header("Host", "cq.ke.com")
            .header("Referer", "https://cq.ke.com/")
            .send()
            .await?;

        // 检查响应状态
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP请求失败: {}", response.status()));
        }

        // 检查内容类型
        let content_type = response.headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("unknown");
        
        println!("响应状态: {}", response.status());
        println!("内容类型: {}", content_type);

        // 获取原始字节数据
        let bytes = response.bytes().await?;
        
        // 检查是否被gzip压缩
        let html_content = if bytes.len() > 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
            // 这是gzip压缩的数据
            println!("检测到gzip压缩数据，正在解压...");
            use std::io::Read;
            use flate2::read::GzDecoder;
            
            let mut decoder = GzDecoder::new(&bytes[..]);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed)
                .map_err(|e| anyhow::anyhow!("解压失败: {}", e))?;
            decompressed
        } else {
            // 尝试直接解码为UTF-8
            String::from_utf8(bytes.to_vec())
                .map_err(|e| anyhow::anyhow!("UTF-8解码失败: {}", e))?
        };
        
        // 以UTF-8编码输出HTML内容
        println!("=== HTML内容 (UTF-8编码) ===");
        println!("HTML内容长度: {} 字符", html_content.len());
        
        // 安全地显示前100个字符
        let preview_start = html_content.chars().take(100).collect::<String>();
        println!("HTML内容前100个字符: {}", preview_start);
        
        // 安全地显示后100个字符
        let preview_end = html_content.chars().rev().take(100).collect::<Vec<char>>();
        let preview_end: String = preview_end.into_iter().rev().collect();
        println!("HTML内容后100个字符: {}", preview_end);
        
        // 检查是否包含HTML标签
        if html_content.contains("<html") || html_content.contains("<!DOCTYPE") {
            println!("检测到HTML内容");
        } else {
            println!("未检测到标准HTML内容，可能是压缩或编码问题");
        }
        
        println!("=== HTML内容结束 ===");
        
        let document = Html::parse_document(&html_content);

        // 检查是否遇到人机验证
        if html_content.contains("人机验证") || html_content.contains("CAPTCHA") {
            return Err(anyhow::anyhow!("遇到人机验证，请稍后重试"));
        }

        // 选择成交列表项 - 尝试多种可能的选择器
        let list_selectors = vec![
            ".listContent .listItem",
            ".listContent .VIEWDATA", 
            ".listContent li",
            ".listContent .item",
            ".listContent div[class*='item']",
            ".listContent div[class*='list']",
            "div[class*='list'] div[class*='item']",
            ".listContent",
            "div[class*='chengjiao']",
            "div[class*='deal']"
        ];
        
        let mut results: Vec<ChengjiaoData> = Vec::new();
        
        for selector_str in list_selectors {
            if let Ok(list_selector) = Selector::parse(selector_str) {
                let items = document.select(&list_selector);
                if items.count() > 0 {
                    println!("找到选择器: {}", selector_str);
                    // 重新选择，因为count()消耗了迭代器
                    let items = document.select(&list_selector);
                    for item in items {
                        if let Ok(chengjiao_data) = self.parse_list_item(&item) {
                            results.push(chengjiao_data);
                        }
                    }
                    break; // 找到有效选择器后退出
                }
            }
        }

        Ok(results)
    }

    fn parse_list_item(&self, item: &scraper::ElementRef) -> Result<ChengjiaoData> {
        // 尝试多种选择器来提取数据
        let title_selectors = vec![
            ".title a",
            "a[class*='title']",
            "h3 a",
            "h2 a", 
            "a[href*='chengjiao']",
            "a"
        ];
        
        let price_selectors = vec![
            ".totalPrice .number",
            ".totalPrice",
            ".price .number",
            ".price",
            "span[class*='price']",
            "span[class*='total']"
        ];
        
        let list_price_selectors = vec![
            ".unitPrice .number",
            ".unitPrice",
            ".listPrice",
            "span[class*='unit']",
            "span[class*='list']"
        ];
        
        let date_selectors = vec![
            ".dealDate",
            ".date",
            "span[class*='date']",
            "span[class*='deal']"
        ];
        
        let info_selectors = vec![
            ".houseInfo",
            ".info",
            "span[class*='info']",
            "span[class*='house']"
        ];
        
        let cycle_selectors = vec![
            ".dealCycleInfo",
            ".cycle",
            "span[class*='cycle']",
            "span[class*='deal']"
        ];

        // 提取标题
        let mut title = "未知".to_string();
        for selector_str in &title_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(el) = item.select(&selector).next() {
                    let text = el.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text != "未知" {
                        title = text;
                        break;
                    }
                }
            }
        }

        // 提取成交价
        let mut deal_price = "未知".to_string();
        for selector_str in &price_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(el) = item.select(&selector).next() {
                    let text = el.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text != "未知" {
                        deal_price = text;
                        break;
                    }
                }
            }
        }

        // 提取挂牌价
        let mut list_price = "未知".to_string();
        for selector_str in &list_price_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(el) = item.select(&selector).next() {
                    let text = el.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text != "未知" {
                        list_price = text;
                        break;
                    }
                }
            }
        }

        // 提取成交日期
        let mut deal_date = "未知".to_string();
        for selector_str in &date_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(el) = item.select(&selector).next() {
                    let text = el.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text != "未知" {
                        deal_date = text;
                        break;
                    }
                }
            }
        }

        // 提取装修信息
        let mut house_info = "未知".to_string();
        for selector_str in &info_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(el) = item.select(&selector).next() {
                    let text = el.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text != "未知" {
                        house_info = text;
                        break;
                    }
                }
            }
        }
        
        let is_renovated = if house_info.contains("精装") {
            "精装".to_string()
        } else if house_info.contains("简装") {
            "简装".to_string()
        } else if house_info.contains("毛坯") {
            "毛坯".to_string()
        } else {
            "未知".to_string()
        };

        // 提取成交周期
        let mut deal_cycle = "未知".to_string();
        for selector_str in &cycle_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(el) = item.select(&selector).next() {
                    let text = el.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text != "未知" {
                        deal_cycle = text;
                        break;
                    }
                }
            }
        }

        // 如果所有字段都是"未知"，尝试从整个元素的文本中提取信息
        if title == "未知" && deal_price == "未知" && list_price == "未知" {
            let full_text = item.text().collect::<String>();
            println!("调试 - 整个元素的文本: {}", full_text);
        }

        Ok(ChengjiaoData {
            title,
            deal_price,
            list_price,
            deal_date,
            is_renovated,
            deal_cycle,
        })
    }

    pub async fn scrape_multiple_pages(&self, base_url: &str, start_page: u32, end_page: u32) -> Result<Vec<ChengjiaoData>> {
        let mut all_results = Vec::new();
        
        for page in start_page..=end_page {
            let url = base_url.replace("pg1", &format!("pg{}", page));
            println!("正在爬取第 {} 页: {}", page, url);
            
            match self.scrape_chengjiao_page(&url).await {
                Ok(results) => {
                    println!("第 {} 页成功获取 {} 条数据", page, results.len());
                    all_results.extend(results);
                }
                Err(e) => {
                    println!("第 {} 页爬取失败: {}", page, e);
                    // 如果遇到人机验证，暂停一下再继续
                    if e.to_string().contains("人机验证") {
                        println!("遇到人机验证，等待5秒后继续...");
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
            
            // 添加延迟避免被封
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
        
        Ok(all_results)
    }
}
