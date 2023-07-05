use elasticlunr::{lang::common::RegexTrimmer, pipeline::FnWrapper, Language, Pipeline};

#[derive(Clone)]
pub struct Chinese {
    jieba: jieba_rs::Jieba,
}

impl Chinese {
    pub fn new() -> Self {
        Self {
            jieba: jieba_rs::Jieba::new(),
        }
    }
}

impl Language for Chinese {
    fn name(&self) -> String {
        "Chinese".into()
    }

    fn code(&self) -> String {
        "zh".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        self.jieba
            .cut_for_search(text, false)
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn make_pipeline(&self) -> elasticlunr::Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new(
                    "trimmer-zh",
                    r"\p{Unified_Ideograph}\p{Latin}",
                )),
                Box::new(FnWrapper("stopWordFilter-zh".into(), stop_word_filter)),
                Box::new(FnWrapper("stemmer-zh".into(), stemmer)),
            ],
        }
    }
}

fn stop_word_filter(token: String) -> Option<String> {
    match token.as_str() {
        "的" | "一" | "不" | "在" | "人" | "有" | "是" | "为" | "為" | "以" | "于" | "於"
        | "上" | "他" | "而" | "后" | "後" | "之" | "来" | "來" | "及" | "了" | "因" | "下"
        | "可" | "到" | "由" | "这" | "這" | "与" | "與" | "也" | "此" | "但" | "并" | "並"
        | "个" | "個" | "其" | "已" | "无" | "無" | "小" | "我" | "们" | "們" | "起" | "最"
        | "再" | "今" | "去" | "好" | "只" | "又" | "或" | "很" | "亦" | "某" | "把" | "那"
        | "你" | "乃" | "它" | "吧" | "被" | "比" | "别" | "趁" | "当" | "當" | "从" | "從"
        | "得" | "打" | "凡" | "儿" | "兒" | "尔" | "爾" | "该" | "該" | "各" | "给" | "給"
        | "跟" | "和" | "何" | "还" | "還" | "即" | "几" | "幾" | "既" | "看" | "据" | "據"
        | "距" | "靠" | "啦" | "另" | "么" | "麽" | "每" | "嘛" | "拿" | "哪" | "您" | "凭"
        | "憑" | "且" | "却" | "卻" | "让" | "讓" | "仍" | "啥" | "如" | "若" | "使" | "谁"
        | "誰" | "虽" | "雖" | "随" | "隨" | "同" | "所" | "她" | "哇" | "嗡" | "往" | "些"
        | "向" | "沿" | "哟" | "喲" | "用" | "咱" | "则" | "則" | "怎" | "曾" | "至" | "致"
        | "着" | "著" | "诸" | "諸" | "自" => None,
        _ => Some(token),
    }
}

fn stemmer(token: String) -> Option<String> {
    Some(token)
}
