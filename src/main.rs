mod chengjiao;
mod ershoufang;

use chengjiao::{ChengjiaoScraper, ChengjiaoData};
use ershoufang::{ErshoufangScraper, ErshoufangData};
use std::fs;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("贝壳网成交数据爬虫启动...");
    
    // 你提供的cookie
    let cookies = "";
    
    let chengjiao_scraper = ChengjiaoScraper::new(cookies.to_string());
    let ershoufang_scraper = ErshoufangScraper::new(cookies.to_string());
    
    // 成交数据爬取配置
    //精装修 de1  https://cq.ke.com/chengjiao/dazhulin/pg1de1/
    //简装修 de2 https://cq.ke.com/chengjiao/dazhulin/pg1de2/
    //毛坯 de3 https://cq.ke.com/chengjiao/dazhulin/pg1de3/
    //
    let chengjiao_base_url = "https://cq.ke.com/chengjiao/dazhulin/pg1/";
    let chengjiao_start_page = 1;
    let chengjiao_end_page = 5; // 爬取前5页
    
    // 在售数据爬取配置
    let ershoufang_base_url = "https://cq.ke.com/ershoufang/dazhulin/pg1/";
    let ershoufang_start_page = 1;
    let ershoufang_end_page = 50; // 爬取前50页
    
    println!("=== 第一阶段：爬取成交数据 ===");
    println!("目标URL: {}", chengjiao_base_url);
    println!("开始爬取第 {} 页到第 {} 页...", chengjiao_start_page, chengjiao_end_page);
    
    let chengjiao_results = match chengjiao_scraper.scrape_multiple_pages(chengjiao_base_url, chengjiao_start_page, chengjiao_end_page).await {
        Ok(results) => {
            println!("\n成交数据爬取完成！总共获取 {} 条数据", results.len());
            
            if results.is_empty() {
                println!("未获取到任何成交数据，可能是页面结构发生变化或遇到反爬虫机制");
                Vec::new()
            } else {
                // 打印前几条数据作为预览
                println!("\n=== 成交数据预览 ===");
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
                
                // 保存成交数据到JSON文件
                let json_data = serde_json::to_string_pretty(&results)?;
                fs::write("chengjiao_data.json", json_data)?;
                println!("\n成交数据已保存到 chengjiao_data.json");
                
                // 保存成交数据到CSV文件
                save_chengjiao_to_csv(&results, "chengjiao_data.csv")?;
                println!("成交数据已保存到 chengjiao_data.csv");
                
                // 成交数据统计信息
                print_chengjiao_statistics(&results);
                
                results
            }
        }
        Err(e) => {
            println!("成交数据爬取失败: {}", e);
            Vec::new()
        }
    };
    
    println!("\n=== 第二阶段：爬取在售数据 ===");
    println!("目标URL: {}", ershoufang_base_url);
    println!("开始爬取第 {} 页到第 {} 页...", ershoufang_start_page, ershoufang_end_page);
    
    let ershoufang_results = match ershoufang_scraper.scrape_multiple_pages(ershoufang_base_url, ershoufang_start_page, ershoufang_end_page).await {
        Ok(results) => {
            println!("\n在售数据爬取完成！总共获取 {} 条数据", results.len());
            
            if results.is_empty() {
                println!("未获取到任何在售数据，可能是页面结构发生变化或遇到反爬虫机制");
                Vec::new()
            } else {
                // 打印前几条数据作为预览
                println!("\n=== 在售数据预览 ===");
                for (i, data) in results.iter().take(3).enumerate() {
                    println!("\n--- 第 {} 条数据 ---", i + 1);
                    println!("标题: {}", data.title);
                    println!("总价: {}", data.total_price);
                    println!("单价: {}", data.unit_price);
                    println!("面积: {}", data.area);
                    println!("户型: {}", data.layout);
                    println!("楼层: {}", data.floor);
                    println!("建成年份: {}", data.build_year);
                    println!("小区: {}", data.community);
                    println!("区域: {}", data.district);
                    println!("标签: {}", data.tags.join(", "));
                    println!("链接: {}", data.url);
                    println!("关注人数: {}", data.attention_count);
                    println!("发布时间: {}", data.publish_time);
                }
                
                if results.len() > 3 {
                    println!("\n... 还有 {} 条数据", results.len() - 3);
                }
                
                // 保存在售数据到JSON文件
                let json_data = serde_json::to_string_pretty(&results)?;
                fs::write("ershoufang_data.json", json_data)?;
                println!("\n在售数据已保存到 ershoufang_data.json");
                
                // 保存在售数据到CSV文件
                save_ershoufang_to_csv(&results, "ershoufang_data.csv")?;
                println!("在售数据已保存到 ershoufang_data.csv");
                
                // 在售数据统计信息
                print_ershoufang_statistics(&results);
                
                results
            }
        }
        Err(e) => {
            println!("在售数据爬取失败: {}", e);
            Vec::new()
        }
    };
    
    // 总结报告
    println!("\n=== 爬取总结报告 ===");
    println!("成交数据: {} 条", chengjiao_results.len());
    println!("在售数据: {} 条", ershoufang_results.len());
    println!("总数据量: {} 条", chengjiao_results.len() + ershoufang_results.len());
    
    if chengjiao_results.is_empty() && ershoufang_results.is_empty() {
        println!("\n警告：未获取到任何数据，请检查网络连接或网站结构是否发生变化");
    } else {
        println!("\n所有数据爬取完成！");
    }
    
    Ok(())
}

fn save_chengjiao_to_csv(data: &[ChengjiaoData], filename: &str) -> Result<()> {
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

fn save_ershoufang_to_csv(data: &[ErshoufangData], filename: &str) -> Result<()> {
    let mut csv_content = String::new();
    csv_content.push_str("标题,总价,单价,面积,户型,楼层,建成年份,小区,区域,标签,链接,关注人数,发布时间\n");
    
    for item in data {
        let tags = item.tags.join(";");
        csv_content.push_str(&format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
            item.title,
            item.total_price,
            item.unit_price,
            item.area,
            item.layout,
            item.floor,
            item.build_year,
            item.community,
            item.district,
            tags,
            item.url,
            item.attention_count,
            item.publish_time
        ));
    }
    
    fs::write(filename, csv_content)?;
    Ok(())
}

fn print_chengjiao_statistics(data: &[ChengjiaoData]) {
    println!("\n=== 成交数据统计 ===");
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

fn print_ershoufang_statistics(data: &[ErshoufangData]) {
    println!("\n=== 在售数据统计 ===");
    println!("总数据量: {} 条", data.len());
    
    // 价格区间统计
    let mut price_ranges = std::collections::HashMap::new();
    for item in data {
        if let Ok(price) = item.total_price.replace("万", "").parse::<f64>() {
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
    
    println!("\n在售价格分布:");
    for (range, count) in price_ranges {
        println!("  {}: {} 条", range, count);
    }
    
    // 区域统计
    let mut district_stats = std::collections::HashMap::new();
    for item in data {
        *district_stats.entry(&item.district).or_insert(0) += 1;
    }
    
    println!("\n区域分布:");
    for (district, count) in district_stats {
        println!("  {}: {} 条", district, count);
    }
}
