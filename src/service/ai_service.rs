use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

pub struct AiContent {
    pub title: String,
    pub body: String,
    pub score: f64,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessageRequest>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct OpenAIMessageRequest {
    role: String,
    content: String,
}

/// AI 内容生成服务
/// 
/// 支持 OpenAI GPT 系列模型
pub async fn generate_content(prompt: &str, platform: &str) -> Result<AiContent> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "".to_string());
    
    // 如果没有 API key，使用模拟数据
    if api_key.is_empty() {
        return generate_mock_content(prompt, platform).await;
    }
    
    // 根据平台生成不同的 prompt
    let (system_prompt, user_prompt) = match platform {
        "xiaohongshu" => get_xiaohongshu_prompts(prompt),
        "wechat" => get_wechat_prompts(prompt),
        _ => get_default_prompts(prompt),
    };
    
    // 调用 OpenAI API
    let content = call_openai_api(&api_key, &system_prompt, &user_prompt).await?;
    
    // 解析返回的内容
    let (title, body, score) = parse_ai_content(&content, platform)?;
    
    Ok(AiContent {
        title,
        body,
        score,
    })
}

/// 生成模拟内容（当没有 API key 时）
async fn generate_mock_content(prompt: &str, platform: &str) -> Result<AiContent> {
    let (title, body) = match platform {
        "xiaohongshu" => generate_xiaohongshu_content(prompt),
        "wechat" => generate_wechat_content(prompt),
        _ => generate_default_content(prompt),
    };
    
    // 模拟 AI 打分
    let score = (rand::random::<f64>() * 3.0 + 7.0).round() / 10.0;
    
    Ok(AiContent {
        title,
        body,
        score,
    })
}

/// 调用 OpenAI API
async fn call_openai_api(api_key: &str, system_prompt: &str, user_prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();
    
    let request = OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            OpenAIMessageRequest {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            OpenAIMessageRequest {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ],
        temperature: 0.8,
    };
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to call OpenAI API: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(anyhow!("OpenAI API error: {} - {}", status, text));
    }
    
    let result: OpenAIResponse = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse OpenAI response: {}", e))?;
    
    if let Some(choice) = result.choices.into_iter().next() {
        Ok(choice.message.content)
    } else {
        Err(anyhow!("No content in OpenAI response"))
    }
}

/// 获取小红书 prompt 模板
fn get_xiaohongshu_prompts(topic: &str) -> (String, String) {
    let system_prompt = r#"你是一个专业的小红书文案写作专家。你需要根据用户提供的 topic 生成一篇吸引人的小红书笔记。

输出格式要求：
1. 第一行是标题（不加任何前缀，30字以内）
2. 空一行
3. 接下来的所有内容是正文
4. 正文要有 emoji，表情符号，小红书风格的标签（#开头）
5. 语气要亲切、自然，像和闺蜜聊天

请直接输出内容，不要有任何解释或前缀。"#.to_string();
    
    let user_prompt = format!("请为以下 topic 写一篇小红书笔记：{}", topic);
    
    (system_prompt, user_prompt)
}

/// 获取微信公众号 prompt 模板
fn get_wechat_prompts(topic: &str) -> (String, String) {
    let system_prompt = r#"你是一个专业的微信公众号内容创作者。你需要根据用户提供的 topic 撰写一篇高质量的公众号文章。

输出格式要求：
1. 第一行是标题（20字以内）
2. 空一行
3. 正文要有清晰的层次结构，使用分级标题
4. 语言正式但不失亲和力
5. 每段不宜过长，适合手机阅读
6. 文章要有深度，价值感强

请直接输出内容，不要有任何解释或前缀。"#.to_string();
    
    let user_prompt = format!("请撰写一篇关于 {} 的公众号文章", topic);
    
    (system_prompt, user_prompt)
}

/// 获取默认 prompt 模板
fn get_default_prompts(topic: &str) -> (String, String) {
    let system_prompt = "你是一个专业的内容创作者。请根据用户提供的 topic 生成内容。".to_string();
    let user_prompt = format!("请生成关于 {} 的内容", topic);
    (system_prompt, user_prompt)
}

/// 解析 AI 返回的内容
fn parse_ai_content(content: &str, platform: &str) -> Result<(String, String, f64)> {
    let lines: Vec<&str> = content.lines().collect();
    
    if lines.is_empty() {
        return Err(anyhow!("Empty AI response"));
    }
    
    // 第一行是标题
    let title = lines[0].trim().to_string();
    
    // 剩余部分是正文
    let body = if lines.len() > 1 {
        lines[1..].join("\n").trim().to_string()
    } else {
        String::new()
    };
    
    // 根据平台给一个模拟分数
    let score = match platform {
        "xiaohongshu" => 8.5,
        "wechat" => 9.0,
        _ => 8.0,
    };
    
    Ok((title, body, score))
}

fn generate_xiaohongshu_content(prompt: &str) -> (String, String) {
    let title = format!("✨ {} | 亲测有效！", prompt);
    let body = format!(
        r#"姐妹们，今天来聊聊{}~

😱 问题描述：
最近发现好多宝宝都在问这个，赶紧来分享一下我的经验！

💡 解决方案：
1️⃣ 第一步：准备好材料
2️⃣ 第二步：按步骤操作
3️⃣ 第三步：坚持就是胜利！

✨ 效果真的绝！用了之后明显感觉不一样了~

👉 关注我，获取更多干货！

#{} #分享"#,
        prompt, prompt
    );
    (title, body)
}

fn generate_wechat_content(prompt: &str) -> (String, String) {
    let title = format!("关于{}的深度解析", prompt);
    let body = format!(
        r#"一、背景介绍

随着时代的发展，{}成为了大家关注的焦点。本文将深入探讨这一话题。

二、核心观点

本文认为，要正确理解{}，需要从多个维度进行分析。

三、详细论述

（一）理论基础
任何实践都离不开理论的指导。我们需要先建立扎实的理论基础。

（二）实践方法
理论联系实际，才能真正发挥作用。以下是具体的实践方法：

1. 前期准备
2. 中期执行
3. 后期复盘

四、总结

希望通过本文的分析，能让大家对{}有更深入的了解。

---

如果觉得有帮助，欢迎转发分享！"#,
        prompt, prompt, prompt
    );
    (title, body)
}

fn generate_default_content(prompt: &str) -> (String, String) {
    let title = format!("关于{}", prompt);
    let body = format!("以下是关于{}的内容：\n\n{}", prompt, prompt);
    (title, body)
}
