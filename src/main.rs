mod chengjiao;

use chengjiao::{ChengjiaoScraper, ChengjiaoData};
use std::fs;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("贝壳网成交数据爬虫启动...");
    
    // 你提供的cookie
    let cookies = "";
    
    let scraper = ChengjiaoScraper::new(cookies.to_string());
    
    let base_url = "https://cq.ke.com/chengjiao/dazhulin/pg1/";
    
    println!("开始爬取贝壳网成交数据...");
    println!("目标URL: {}", base_url);
    
    // 选择爬取模式
    println!("\n请选择爬取模式:");
    println!("1. 爬取单页数据");
    println!("2. 爬取多页数据");
    
    // 这里简化处理，直接爬取多页数据
    let start_page = 1;
    let end_page = 3; // 爬取前3页
    
    println!("\n开始爬取第 {} 页到第 {} 页...", start_page, end_page);
    
    match scraper.scrape_multiple_pages(base_url, start_page, end_page).await {
        Ok(results) => {
            println!("\n爬取完成！总共获取 {} 条成交数据", results.len());
            
            if results.is_empty() {
                println!("未获取到任何数据，可能是页面结构发生变化或遇到反爬虫机制");
                return Ok(());
            }
            
            // 打印前几条数据作为预览
            println!("\n=== 数据预览 ===");
            for (i, data) in results.iter().take(3).enumerate() {
                println!("\n--- 第 {} 条数据 ---", i + 1);
                println!("标题: {}", data.title);
                println!("成交价: {}", data.deal_price);
                println!("挂牌价: {}", data.list_price);
                println!("成交日期: {}", data.deal_date);
                println!("装修情况: {}", data.is_renovated);
                println!("成交周期: {}", data.deal_cycle);
            }
            
            if results.len() > 3 {
                println!("\n... 还有 {} 条数据", results.len() - 3);
            }
            
            // 保存到JSON文件
            let json_data = serde_json::to_string_pretty(&results)?;
            fs::write("chengjiao_data.json", json_data)?;
            println!("\n数据已保存到 chengjiao_data.json");
            
            // 保存到CSV文件
            save_to_csv(&results, "chengjiao_data.csv")?;
            println!("数据已保存到 chengjiao_data.csv");
            
            // 统计信息
            print_statistics(&results);
        }
        Err(e) => {
            println!("爬取失败: {}", e);
        }
    }
    
    Ok(())
}

fn save_to_csv(data: &[ChengjiaoData], filename: &str) -> Result<()> {
    let mut csv_content = String::new();
    csv_content.push_str("标题,成交价,挂牌价,成交日期,装修情况,成交周期\n");
    
    for item in data {
        csv_content.push_str(&format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
            item.title,
            item.deal_price,
            item.list_price,
            item.deal_date,
            item.is_renovated,
            item.deal_cycle
        ));
    }
    
    fs::write(filename, csv_content)?;
    Ok(())
}

fn print_statistics(data: &[ChengjiaoData]) {
    println!("\n=== 数据统计 ===");
    println!("总数据量: {} 条", data.len());
    
    // 装修情况统计
    let mut renovation_stats = std::collections::HashMap::new();
    for item in data {
        *renovation_stats.entry(&item.is_renovated).or_insert(0) += 1;
    }
    
    println!("\n装修情况分布:");
    for (renovation, count) in renovation_stats {
        println!("  {}: {} 条", renovation, count);
    }
    
    // 价格区间统计
    let mut price_ranges = std::collections::HashMap::new();
    for item in data {
        if let Ok(price) = item.deal_price.replace("万", "").parse::<f64>() {
            let range = match price {
                p if p < 100.0 => "100万以下",
                p if p < 200.0 => "100-200万",
                p if p < 300.0 => "200-300万",
                p if p < 500.0 => "300-500万",
                _ => "500万以上",
            };
            *price_ranges.entry(range).or_insert(0) += 1;
        }
    }
    
    println!("\n成交价格分布:");
    for (range, count) in price_ranges {
        println!("  {}: {} 条", range, count);
    }
}
