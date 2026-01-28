use regex::Regex;
use reqwest::Client;
use std::fs::File;
use std::io::{self, Write};
use tokio;

/// 视频质量选项
#[derive(Debug, Clone)]
struct VideoQuality {
    name: String,
    url: String,
    resolution: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 抖音视频下载工具 (支持多清晰度) ===\n");

    // 获取用户下载目录
    let download_dir = dirs::download_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("./video"));

    let video_dir = download_dir.join("douyin_videos");
    std::fs::create_dir_all(&video_dir)?;

    println!("视频保存目录: {:?}\n", video_dir);

    loop {
        println!("请粘贴抖音分享的内容 (输入 'q' 退出):");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        if input.to_lowercase() == "q" {
            println!("再见!");
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match process_douyin_video(input).await {
            Ok(filename) => {
                println!("✓ 视频下载完成: {}\n", filename);
            }
            Err(e) => {
                eprintln!("✗ 错误: {}\n", e);
            }
        }
    }
    
    Ok(())
}

async fn process_douyin_video(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 提取短链接
    let url = extract_douyin_url(input)?;
    println!("✓ 提取到链接: {}", url);
    
    // 获取所有可用的视频清晰度
    println!("正在解析视频地址...");
    let qualities = get_video_qualities(&url).await?;
    
    // 让用户选择清晰度
    let selected_url = select_quality(&qualities)?;

    // 获取下载目录
    let download_dir = dirs::download_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("./video"));
    let video_dir = download_dir.join("douyin_videos");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let filename = video_dir.join(format!("douyin_{}.mp4", timestamp));
    let filename_str = filename.to_string_lossy().to_string();


    // 下载视频
    println!("\n开始下载视频...");
    download_video(&selected_url, &filename_str).await?;
    
    Ok(filename_str)
}

/// 让用户选择视频清晰度
fn select_quality(qualities: &[VideoQuality]) -> Result<String, Box<dyn std::error::Error>> {
    if qualities.is_empty() {
        return Err("没有找到可用的视频".into());
    }
    
    if qualities.len() == 1 {
        println!("✓ 自动选择: {}", qualities[0].name);
        return Ok(qualities[0].url.clone());
    }
    
    println!("\n可用的清晰度:");
    for (i, quality) in qualities.iter().enumerate() {
        println!("  {}. {}", i + 1, quality.name);
    }
    
    // 默认选择第一个（通常是最高清晰度）
    println!("\n输入数字选择清晰度 (直接回车选择最高清晰度):");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();
    
    let index = if choice.is_empty() {
        0
    } else {
        match choice.parse::<usize>() {
            Ok(n) if n > 0 && n <= qualities.len() => n - 1,
            _ => {
                println!("无效选择，使用默认: {}", qualities[0].name);
                0
            }
        }
    };
    
    println!("✓ 已选择: {}", qualities[index].name);
    Ok(qualities[index].url.clone())
}

/// 从分享文本中提取抖音链接
fn extract_douyin_url(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let patterns = vec![
        r"https://v\.douyin\.com/[A-Za-z0-9\-]+/?",
        r"https://www\.douyin\.com/video/\d+",
        r"https://www\.iesdouyin\.com/share/video/\d+",
        r"https://www\.douyin\.com/note/\d+",
    ];
    
    for pattern in patterns {
        let re = Regex::new(pattern).unwrap();
        if let Some(mat) = re.find(text) {
            return Ok(mat.as_str().to_string());
        }
    }
    
    Err("未找到有效的抖音链接".into())
}

/// 获取所有可用的视频清晰度
async fn get_video_qualities(short_url: &str) -> Result<Vec<VideoQuality>, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15")
        .build()?;
    
    let response = client.get(short_url).send().await?;
    let location = response
        .headers()
        .get("location")
        .ok_or("未找到重定向地址")?
        .to_str()?;
    
    println!("  → 重定向地址");
    
    if let Ok(video_id) = extract_video_id(location) {
        println!("  → 视频ID: {}", video_id);
    } else {
        println!("  → 未找到数字ID，直接解析网页");
    }
    
    println!("  → 正在访问网页...");
    let html = client.get(location).send().await?.text().await?;
    
    if std::env::var("DEBUG").is_ok() {
        if let Ok(mut file) = File::create("debug_page.html") {
            let _ = file.write_all(html.as_bytes());
            println!("  [调试] 已保存页面到 debug_page.html");
        }
    }
    
    let qualities = extract_all_qualities(&html)?;
    
    Ok(qualities)
}

/// 从 URL 中提取视频 ID
fn extract_video_id(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let patterns = vec![
        r"/video/(\d+)",
        r"/share/video/(\d+)",
        r"video_id=(\d+)",
        r"aweme_id=(\d+)",
        r"/note/(\d+)",
    ];
    
    for pattern in patterns {
        let re = Regex::new(pattern).unwrap();
        if let Some(caps) = re.captures(url) {
            return Ok(caps.get(1).unwrap().as_str().to_string());
        }
    }
    
    Err("URL中没有数字ID格式".into())
}

/// 从 HTML 中提取所有可用的清晰度
fn extract_all_qualities(html: &str) -> Result<Vec<VideoQuality>, Box<dyn std::error::Error>> {
    let mut qualities = Vec::new();
    
    println!("  → 正在查找所有清晰度...");
    
    // 从 bit_rate 数组中提取多个清晰度
    let bit_rate_re = Regex::new(r#""bit_rate":\[([^\]]+)\]"#).unwrap();
    
    if let Some(caps) = bit_rate_re.captures(html) {
        let bit_rates_section = caps.get(1).unwrap().as_str();
        
        // 查找每个包含 play_addr 的对象
        let object_re = Regex::new(r#"\{[^{}]*?"play_addr"[^{}]*?\}"#).unwrap();
        
        for obj_match in object_re.find_iter(bit_rates_section) {
            let obj = obj_match.as_str();
            
            let url_re = Regex::new(r#""url_list":\["([^"]+)""#).unwrap();
            let gear_re = Regex::new(r#""gear_name":"([^"]+)""#).unwrap();
            let quality_re = Regex::new(r#""quality_type":(\d+)"#).unwrap();
            
            if let Some(url_caps) = url_re.captures(obj) {
                let raw_url = url_caps.get(1).unwrap().as_str();
                let mut clean_url = decode_unicode_escapes(&raw_url.replace(r"\/", "/"));
                clean_url = remove_watermark_from_url(&clean_url);
                
                let gear_name = gear_re.captures(obj)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_else(|| "未知".to_string());
                
                let quality_type = quality_re.captures(obj)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().parse::<u32>().ok())
                    .unwrap_or(0);
                
                let (display_name, resolution) = match quality_type {
                    10 => ("流畅", "360P"),
                    20 => ("标清", "540P"),
                    30 => ("高清", "720P"),
                    40 => ("超清", "1080P"),
                    _ => (gear_name.as_str(), "未知"),
                };
                
                qualities.push(VideoQuality {
                    name: format!("{} {}", display_name, resolution),
                    url: clean_url,
                    resolution: resolution.to_string(),
                });
            }
        }
    }
    
    // 如果没有找到多个清晰度，使用标准方法
    if qualities.is_empty() {
        println!("  → 未找到多清晰度选项，使用标准方法");
        if let Ok(url) = extract_video_url_standard(html) {
            qualities.push(VideoQuality {
                name: "默认清晰度 (无水印)".to_string(),
                url,
                resolution: "自动".to_string(),
            });
        }
    }
    
    if qualities.is_empty() {
        return Err("无法提取任何视频地址".into());
    }
    
    // 按质量排序（从高到低）
    qualities.sort_by(|a, b| {
        let order = |res: &str| match res {
            "1080P" => 4,
            "720P" => 3,
            "540P" => 2,
            "360P" => 1,
            _ => 0,
        };
        order(&b.resolution).cmp(&order(&a.resolution))
    });
    
    // 去重
    qualities.dedup_by(|a, b| a.url == b.url);
    
    println!("  ✓ 找到 {} 个清晰度选项", qualities.len());
    
    Ok(qualities)
}

/// 标准方法提取视频地址
fn extract_video_url_standard(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let patterns = vec![
        (r#""download_addr":\{"url_list":\["([^"]+)""#, "download_addr"),
        (r#""play_addr":\{[^}]*"url_list":\["([^"]+)""#, "play_addr"),
        (r#""playAddr":\[?\{"src":"([^"]+)""#, "playAddr"),
        (r#""playApi":"([^"]+)""#, "playApi"),
        (r#""video_url":"([^"]+)""#, "video_url"),
    ];
    
    for (pattern, method_name) in patterns {
        let re = Regex::new(pattern).unwrap();
        if let Some(caps) = re.captures(html) {
            let url = caps.get(1).unwrap().as_str();
            let mut clean_url = decode_unicode_escapes(&url.replace(r"\/", "/"));
            clean_url = remove_watermark_from_url(&clean_url);
            
            println!("  [成功] 使用方法: {}", method_name);
            return Ok(clean_url);
        }
    }
    
    Err("无法提取视频地址".into())
}

/// 转换为无水印版本
fn remove_watermark_from_url(url: &str) -> String {
    let mut clean_url = url.to_string();
    
    if clean_url.contains("/playwm/") {
        clean_url = clean_url.replace("/playwm/", "/play/");
    }
    
    if clean_url.contains("watermark=1") {
        clean_url = clean_url.replace("watermark=1", "watermark=0");
    }
    
    clean_url
}

/// 解码 Unicode 转义序列
fn decode_unicode_escapes(s: &str) -> String {
    let re = Regex::new(r"\\u([0-9a-fA-F]{4})").unwrap();
    let mut result = s.to_string();
    
    loop {
        let mut changed = false;
        let temp = result.clone();
        
        for cap in re.captures_iter(&temp) {
            if let Ok(code) = u32::from_str_radix(&cap[1], 16) {
                if let Some(ch) = char::from_u32(code) {
                    result = result.replace(&cap[0], &ch.to_string());
                    changed = true;
                }
            }
        }
        
        if !changed {
            break;
        }
    }
    
    result
}

/// 下载视频
async fn download_video(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15")
        .build()?;
    
    let response = client.get(url).send().await?;
    let total_size = response.content_length().unwrap_or(0);
    
    let mut file = File::create(filename)?;
    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();
    
    use futures_util::StreamExt;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        
        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64) * 100.0;
            print!("\r  下载进度: {:.1}%", progress);
            io::stdout().flush()?;
        }
    }
    
    println!();
    Ok(())
}
