use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErshoufangData {
    pub title: String,
    pub total_price: String,
    pub unit_price: String,
    pub area: String,
    pub layout: String,
    pub floor: String,
    pub build_year: String,
    pub community: String,
    pub district: String,
    pub tags: Vec<String>,
    pub url: String,
    pub attention_count: String,
    pub publish_time: String,
}

pub struct ErshoufangScraper {
    client: Client,
    cookies: String,
}

impl ErshoufangScraper {
    pub fn new(cookies: String) -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .unwrap();

        Self { client, cookies }
    }

    pub async fn scrape_ershoufang_page(&self, url: &str) -> Result<Vec<ErshoufangData>> {
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

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP请求失败: {}", response.status()));
        }

        let bytes = response.bytes().await?;

        let html_content = if bytes.len() > 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
            println!("检测到gzip压缩数据，正在解压...");
            use std::io::Read;
            use flate2::read::GzDecoder;

            let mut decoder = GzDecoder::new(&bytes[..]);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed)
                .map_err(|e| anyhow::anyhow!("解压失败: {}", e))?;
            decompressed
        } else {
            String::from_utf8(bytes.to_vec())
                .map_err(|e| anyhow::anyhow!("UTF-8解码失败: {}", e))?
        };

        let document = Html::parse_document(&html_content);

        if html_content.contains("人机验证") || html_content.contains("CAPTCHA") {
            return Err(anyhow::anyhow!("遇到人机验证，请稍后重试"));
        }

        // 使用正确的选择器：sellListContent下的li元素
        let list_selector = Selector::parse(".sellListContent li.clear").unwrap();
        let items = document.select(&list_selector);
        
        let mut results: Vec<ErshoufangData> = Vec::new();
        
        for item in items {
            if let Ok(ershoufang_data) = self.parse_list_item(&item) {
                results.push(ershoufang_data);
            }
        }

        println!("成功解析 {} 条二手房数据", results.len());
        Ok(results)
    }

    fn parse_list_item(&self, item: &scraper::ElementRef) -> Result<ErshoufangData> {
        // 解析标题和链接
        let title_selector = Selector::parse(".title a").unwrap();
        let title = item.select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "未知".to_string());
        
        let url = item.select(&title_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .unwrap_or("")
            .to_string();

        // 解析总价
        let total_price_selector = Selector::parse(".totalPrice").unwrap();
        let total_price = item.select(&total_price_selector)
            .next()
            .map(|el| {
                let text = el.text().collect::<String>();
                // 提取数字部分，去掉"万"等文字
                text.chars()
                    .filter(|c| c.is_digit(10) || *c == '.')
                    .collect::<String>()
            })
            .unwrap_or_else(|| "未知".to_string());

        // 解析单价
        let unit_price_selector = Selector::parse(".unitPrice span").unwrap();
        let unit_price = item.select(&unit_price_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "未知".to_string());

        // 解析房屋信息（户型、面积、朝向、楼层、建筑年份）
        let house_info_selector = Selector::parse(".houseInfo").unwrap();
        let house_info = item.select(&house_info_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_else(|| "".to_string());
        
        let (layout, area, floor, build_year) = self.parse_house_info(&house_info);

        // 解析小区信息
        let community_selector = Selector::parse(".positionInfo a").unwrap();
        let community = item.select(&community_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "未知".to_string());

        // 解析关注信息
        let follow_info_selector = Selector::parse(".followInfo").unwrap();
        let follow_info = item.select(&follow_info_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_else(|| "".to_string());
        
        let (attention_count, publish_time) = self.parse_follow_info(&follow_info);

        // 解析标签
        let tag_selector = Selector::parse(".tag span").unwrap();
        let tags: Vec<String> = item.select(&tag_selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect();

        Ok(ErshoufangData {
            title,
            total_price: if total_price.is_empty() { "未知".to_string() } else { format!("{}万", total_price) },
            unit_price,
            area,
            layout,
            floor,
            build_year,
            community,
            district: "重庆渝北".to_string(), // 可以根据实际情况动态获取
            tags,
            url,
            attention_count,
            publish_time,
        })
    }

    fn parse_house_info(&self, house_info: &str) -> (String, String, String, String) {
        let info = house_info.replace('\n', " ").replace('\r', " ");
        let parts: Vec<&str> = info.split('|').collect();
        
        let mut layout = "未知".to_string();
        let mut area = "未知".to_string();
        let mut floor = "未知".to_string();
        let mut build_year = "未知".to_string();

        for part in parts {
            let part = part.trim();
            if part.contains("室") && part.contains("厅") {
                layout = part.to_string();
            } else if part.contains("平米") {
                area = part.to_string();
            } else if part.contains("层") {
                floor = part.to_string();
            } else if part.contains("年") && part.chars().any(|c| c.is_digit(10)) {
                build_year = part.to_string();
            }
        }

        (layout, area, floor, build_year)
    }

    fn parse_follow_info(&self, follow_info: &str) -> (String, String) {
        let info = follow_info.replace('\n', " ").replace('\r', " ");
        let parts: Vec<&str> = info.split('/').collect();
        
        let attention_count = if parts.len() > 0 {
            parts[0].trim().to_string()
        } else {
            "未知".to_string()
        };
        
        let publish_time = if parts.len() > 1 {
            parts[1].trim().to_string()
        } else {
            "未知".to_string()
        };

        (attention_count, publish_time)
    }

    pub async fn scrape_multiple_pages(&self, base_url: &str, start_page: u32, end_page: u32) -> Result<Vec<ErshoufangData>> {
        let mut all_results: Vec<ErshoufangData> = Vec::new();

        for page in start_page..=end_page {
            let url = base_url.replace("pg1", &format!("pg{}", page));

            println!("正在爬取第 {} 页: {}", page, url);

            match self.scrape_ershoufang_page(&url).await {
                Ok(page_results) => {
                    println!("第 {} 页成功获取 {} 条数据", page, page_results.len());
                    all_results.extend(page_results);
                },
                Err(e) => {
                    println!("第 {} 页爬取失败: {}", page, e);
                    if e.to_string().contains("人机验证") {
                        println!("遇到人机验证，等待5秒后重试...");
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        continue;
                    }
                }
            }

            if page < end_page {
                println!("等待1秒后继续...");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }

        Ok(all_results)
    }
}
