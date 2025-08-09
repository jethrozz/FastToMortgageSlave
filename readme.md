# Fast-track your way to becoming a 'mortgage slave'

这是一个由AI完成的用于帮助你快速成为房奴的贝壳网数据爬虫，可以爬取贝壳网的成交数据和在售数据，包括标题、价格、日期、装修情况、户型、面积等详细信息。
## 项目总结

### 🏗️ 技术架构
- **语言**: Rust (高性能、内存安全)
- **异步处理**: Tokio 运行时，支持并发爬取
- **数据解析**: HTML 解析器，支持多种选择器策略
- **数据存储**: JSON 和 CSV 双重格式，便于数据分析

### 📊 数据覆盖
- **成交数据**: 历史成交记录，包含成交价、挂牌价、成交周期等
- **在售数据**: 当前在售房源，包含总价、单价、户型、面积等
- **统计分析**: 价格分布、区域分布、装修情况等统计信息

### 🔄 爬取策略
- **两阶段爬取**: 先爬取成交数据，再爬取在售数据
- **多页支持**: 支持批量爬取多页数据
- **智能重试**: 遇到人机验证时自动等待并重试
- **延迟控制**: 内置请求延迟，避免触发反爬虫机制

## 功能特点

- 🔍 **双数据源爬取**: 支持爬取贝壳网成交数据和在售数据
- 📊 **智能数据提取**: 自动提取标题、价格、日期、装修情况、户型、面积等关键信息
- 📄 **多页批量爬取**: 支持批量爬取多页数据，提高数据收集效率
- 💾 **双重格式保存**: 数据同时保存为JSON和CSV格式，便于不同场景使用
- 📈 **丰富统计分析**: 提供价格分布、区域分布、装修情况等多维度统计
- 🛡️ **反爬虫防护**: 内置反爬虫检测、延迟机制和人机验证处理
- 🔄 **智能重试机制**: 遇到网络问题或验证码时自动重试
- ⚡ **异步高性能**: 基于Tokio的异步处理，提升爬取效率

## 安装和运行

### 环境要求

- **Rust**: 1.70+ (推荐使用最新稳定版本)
- **网络连接**: 稳定的网络连接，能够访问贝壳网
- **操作系统**: 支持 Rust 的任意操作系统 (Windows, macOS, Linux)

### 快速开始

1. **克隆项目**
```bash
git clone <your-repo-url>
cd fastToMortgageSlave
```

2. **安装依赖**
```bash
cargo build
```

3. **运行爬虫**
```bash
cargo run
```

### 详细使用步骤

#### 第一步：配置Cookie
在 `src/main.rs` 中配置你的贝壳网Cookie：
```rust
let cookies = "your_cookie_string_here";
```

#### 第二步：调整爬取参数
在 `src/main.rs` 中修改爬取页数：
```rust
let chengjiao_end_page = 3;    // 成交数据爬取页数
let ershoufang_end_page = 3;   // 在售数据爬取页数
```

#### 第三步：运行程序
```bash
cargo run
```

#### 第四步：查看结果
程序运行完成后，会在项目根目录生成以下文件：
- `chengjiao_data.json` - 成交数据JSON格式
- `chengjiao_data.csv` - 成交数据CSV格式
- `ershoufang_data.json` - 在售数据JSON格式
- `ershoufang_data.csv` - 在售数据CSV格式

## 数据字段说明

### 成交数据字段

| 字段名 | 说明 | 示例 |
|--------|------|------|
| title | 房源标题 | "大竹林 2室1厅 精装修" |
| deal_price | 成交价 | "180万" |
| list_price | 挂牌价 | "185万" |
| deal_date | 成交日期 | "2024-01-15" |
| is_renovated | 装修情况 | "精装"/"简装"/"毛坯" |
| deal_cycle | 成交周期 | "30天" |

### 在售数据字段

| 字段名 | 说明 | 示例 |
|--------|------|------|
| title | 房源标题 | "大竹林 2室1厅 精装修" |
| total_price | 总价 | "180万" |
| unit_price | 单价 | "15000元/㎡" |
| area | 面积 | "120㎡" |
| layout | 户型 | "2室1厅" |
| floor | 楼层 | "中层/6层" |
| build_year | 建成年份 | "2010年" |
| community | 小区名称 | "大竹林小区" |
| district | 所在区域 | "渝北区" |
| tags | 房源标签 | ["精装", "地铁房", "学区房"] |

## 输出文件

程序运行后会生成以下文件：

### 成交数据文件
- `chengjiao_data.json` - JSON格式的完整成交数据
- `chengjiao_data.csv` - CSV格式的成交数据，方便在Excel中查看和分析

### 在售数据文件
- `ershoufang_data.json` - JSON格式的完整在售数据
- `ershoufang_data.csv` - CSV格式的在售数据，方便在Excel中查看和分析

### 文件格式说明
- **JSON格式**: 保留完整的数据结构，适合程序处理
- **CSV格式**: 表格形式，适合在Excel、Google Sheets等工具中分析

## 配置说明

### Cookie配置

程序中使用的是你提供的cookie，包含登录状态和认证信息。如果cookie过期，需要更新main.rs中的cookie字符串。

**获取Cookie的方法**:
1. 登录贝壳网
2. 按F12打开开发者工具
3. 在Network标签页中找到任意请求
4. 复制请求头中的Cookie值

### 爬取范围配置

默认爬取前3页数据，可以在main.rs中修改：

```rust
// 成交数据爬取配置
let chengjiao_start_page = 1;
let chengjiao_end_page = 3;    // 修改这个数字来爬取更多页

// 在售数据爬取配置
let ershoufang_start_page = 1;
let ershoufang_end_page = 3;   // 修改这个数字来爬取更多页
```

### 延迟设置

为了避免被反爬虫机制检测，程序在每次请求之间会等待1秒。可以在相关文件中调整：

```rust
// 在 chengjiao.rs 和 ershoufang.rs 中
tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
```

### 目标URL配置

可以在main.rs中修改爬取的目标区域：

```rust
// 成交数据URL (大竹林区域)
let chengjiao_base_url = "https://cq.ke.com/chengjiao/dazhulin/pg1/";

// 在售数据URL (大竹林区域)
let ershoufang_base_url = "https://cq.ke.com/ershoufang/dazhulin/pg1/";
```

## 使用示例

### 基本使用流程

```bash
# 1. 克隆项目
git clone <your-repo-url>
cd fastToMortgageSlave

# 2. 配置Cookie (在src/main.rs中)
# 3. 调整爬取参数 (页数、区域等)
# 4. 运行程序
cargo run

# 5. 查看结果文件
ls -la *.json *.csv
```

### 输出示例

程序运行时会显示详细的爬取进度：

```
贝壳网成交数据爬虫启动...
=== 第一阶段：爬取成交数据 ===
目标URL: https://cq.ke.com/chengjiao/dazhulin/pg1/
开始爬取第 1 页到第 3 页...
正在爬取第 1 页: https://cq.ke.com/chengjiao/dazhulin/pg1/
第 1 页成功获取 20 条数据
...

=== 第二阶段：爬取在售数据 ===
目标URL: https://cq.ke.com/ershoufang/dazhulin/pg1/
开始爬取第 1 页到第 3 页...
...

=== 爬取总结报告 ===
成交数据: 60 条
在售数据: 60 条
总数据量: 120 条
所有数据爬取完成！
```

## 注意事项

1. **Cookie有效期**：提供的cookie可能会过期，需要定期更新
2. **反爬虫机制**：贝壳网网站有反爬虫机制，如果遇到人机验证，程序会自动等待并重试
3. **页面结构变化**：如果贝壳网页面结构发生变化，可能需要更新CSS选择器
4. **使用频率**：建议不要过于频繁地运行爬虫，避免IP被封
5. **数据准确性**：爬取的数据仅供参考，实际购房时请以官方数据为准

## 错误处理

程序包含以下错误处理机制：

- 网络连接错误
- 页面解析错误
- 人机验证检测
- 数据格式错误

## 依赖包

- `reqwest` - HTTP客户端
- `tokio` - 异步运行时
- `serde` - 序列化/反序列化
- `scraper` - HTML解析
- `anyhow` - 错误处理

## 项目结构

```
fastToMortgageSlave/
├── src/
│   ├── main.rs              # 主程序入口，协调两个爬虫
│   ├── chengjiao.rs         # 成交数据爬虫实现
│   └── ershoufang.rs        # 在售数据爬虫实现
├── Cargo.toml               # Rust项目依赖配置
├── Cargo.lock               # 依赖版本锁定文件
├── README.md                # 项目说明文档
└── 输出文件/
    ├── chengjiao_data.json  # 成交数据JSON格式
    ├── chengjiao_data.csv   # 成交数据CSV格式
    ├── ershoufang_data.json # 在售数据JSON格式
    └── ershoufang_data.csv  # 在售数据CSV格式
```

## 技术特性

- **模块化设计**: 成交数据和在售数据爬虫分离，便于维护和扩展
- **异步处理**: 基于Tokio的异步运行时，提升性能
- **智能解析**: 多种CSS选择器策略，提高数据提取成功率
- **错误恢复**: 完善的错误处理机制，包含重试和降级策略
- **数据验证**: 内置数据完整性检查，确保输出质量

## 许可证

MIT License

## 免责声明

本工具仅供学习和研究使用，请遵守相关网站的使用条款和robots.txt规定。使用者需要自行承担使用风险，开发者不承担任何法律责任。

## 贡献指南

欢迎提交Issue和Pull Request来改进这个项目！

## 更新日志

### v0.1.0
- 实现成交数据爬虫
- 实现在售数据爬虫
- 支持JSON和CSV双重格式输出
- 内置统计分析功能
- 完善的反爬虫防护机制
## 数据字段说明

### 成交数据字段

| 字段名 | 说明 | 示例 |
|--------|------|------|
| title | 房源标题 | "大竹林 2室1厅 精装修" |
| deal_price | 成交价 | "180万" |
| list_price | 挂牌价 | "185万" |
| deal_date | 成交日期 | "2024-01-15" |
| is_renovated | 装修情况 | "精装"/"简装"/"毛坯" |
| deal_cycle | 成交周期 | "30天" |

### 在售数据字段

| 字段名 | 说明 | 示例 |
|--------|------|------|
| title | 房源标题 | "大竹林 2室1厅 精装修" |
| total_price | 总价 | "180万" |
| unit_price | 单价 | "15000元/㎡" |
| area | 面积 | "120㎡" |
| layout | 户型 | "2室1厅" |
| floor | 楼层 | "中层/6层" |
| build_year | 建成年份 | "2010年" |
| community | 小区名称 | "大竹林小区" |
| district | 所在区域 | "渝北区" |
| tags | 房源标签 | ["精装", "地铁房", "学区房"] |

## 输出文件

程序运行后会生成以下文件：

### 成交数据文件
- `chengjiao_data.json` - JSON格式的完整成交数据
- `chengjiao_data.csv` - CSV格式的成交数据，方便在Excel中查看和分析

### 在售数据文件
- `ershoufang_data.json` - JSON格式的完整在售数据
- `ershoufang_data.csv` - CSV格式的在售数据，方便在Excel中查看和分析

### 文件格式说明
- **JSON格式**: 保留完整的数据结构，适合程序处理
- **CSV格式**: 表格形式，适合在Excel、Google Sheets等工具中分析

## 配置说明

### Cookie配置

程序中使用的是你提供的cookie，包含登录状态和认证信息。如果cookie过期，需要更新main.rs中的cookie字符串。

**获取Cookie的方法**:
1. 登录贝壳网
2. 按F12打开开发者工具
3. 在Network标签页中找到任意请求
4. 复制请求头中的Cookie值

### 爬取范围配置

默认爬取前3页数据，可以在main.rs中修改：

```rust
// 成交数据爬取配置
let chengjiao_start_page = 1;
let chengjiao_end_page = 3;    // 修改这个数字来爬取更多页

// 在售数据爬取配置
let ershoufang_start_page = 1;
let ershoufang_end_page = 3;   // 修改这个数字来爬取更多页
```

### 延迟设置

为了避免被反爬虫机制检测，程序在每次请求之间会等待1秒。可以在相关文件中调整：

```rust
// 在 chengjiao.rs 和 ershoufang.rs 中
tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
```

### 目标URL配置

可以在main.rs中修改爬取的目标区域：

```rust
// 成交数据URL (大竹林区域)
let chengjiao_base_url = "https://cq.ke.com/chengjiao/dazhulin/pg1/";

// 在售数据URL (大竹林区域)
let ershoufang_base_url = "https://cq.ke.com/ershoufang/dazhulin/pg1/";
```

## 使用示例

### 基本使用流程

```bash
# 1. 克隆项目
git clone <your-repo-url>
cd fastToMortgageSlave

# 2. 配置Cookie (在src/main.rs中)
# 3. 调整爬取参数 (页数、区域等)
# 4. 运行程序
cargo run

# 5. 查看结果文件
ls -la *.json *.csv
```

### 输出示例

程序运行时会显示详细的爬取进度：

```
贝壳网成交数据爬虫启动...
=== 第一阶段：爬取成交数据 ===
目标URL: https://cq.ke.com/chengjiao/dazhulin/pg1/
开始爬取第 1 页到第 3 页...
正在爬取第 1 页: https://cq.ke.com/chengjiao/dazhulin/pg1/
第 1 页成功获取 20 条数据
...

=== 第二阶段：爬取在售数据 ===
目标URL: https://cq.ke.com/ershoufang/dazhulin/pg1/
开始爬取第 1 页到第 3 页...
...

=== 爬取总结报告 ===
成交数据: 60 条
在售数据: 60 条
总数据量: 120 条
所有数据爬取完成！
```

## 注意事项

1. **Cookie有效期**：提供的cookie可能会过期，需要定期更新
2. **反爬虫机制**：贝壳网网站有反爬虫机制，如果遇到人机验证，程序会自动等待并重试
3. **页面结构变化**：如果贝壳网页面结构发生变化，可能需要更新CSS选择器
4. **使用频率**：建议不要过于频繁地运行爬虫，避免IP被封
5. **数据准确性**：爬取的数据仅供参考，实际购房时请以官方数据为准

## 错误处理

程序包含以下错误处理机制：

- 网络连接错误
- 页面解析错误
- 人机验证检测
- 数据格式错误

## 依赖包

- `reqwest` - HTTP客户端
- `tokio` - 异步运行时
- `serde` - 序列化/反序列化
- `scraper` - HTML解析
- `anyhow` - 错误处理

## 项目结构

```
fastToMortgageSlave/
├── src/
│   ├── main.rs              # 主程序入口，协调两个爬虫
│   ├── chengjiao.rs         # 成交数据爬虫实现
│   └── ershoufang.rs        # 在售数据爬虫实现
├── Cargo.toml               # Rust项目依赖配置
├── Cargo.lock               # 依赖版本锁定文件
├── README.md                # 项目说明文档
└── 输出文件/
    ├── chengjiao_data.json  # 成交数据JSON格式
    ├── chengjiao_data.csv   # 成交数据CSV格式
    ├── ershoufang_data.json # 在售数据JSON格式
    └── ershoufang_data.csv  # 在售数据CSV格式
```

## 技术特性

- **模块化设计**: 成交数据和在售数据爬虫分离，便于维护和扩展
- **异步处理**: 基于Tokio的异步运行时，提升性能
- **智能解析**: 多种CSS选择器策略，提高数据提取成功率
- **错误恢复**: 完善的错误处理机制，包含重试和降级策略
- **数据验证**: 内置数据完整性检查，确保输出质量

## 许可证

MIT License
## 免责声明

本工具仅供学习和研究使用，请遵守相关网站的使用条款和robots.txt规定。使用者需要自行承担使用风险，开发者不承担任何法律责任。

## 贡献指南

欢迎提交Issue和Pull Request来改进这个项目！

## 更新日志

### v0.1.0
- 实现成交数据爬虫
- 实现在售数据爬虫
- 支持JSON和CSV双重格式输出
- 内置统计分析功能
- 完善的反爬虫防护机制
