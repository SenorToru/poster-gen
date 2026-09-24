use poster_gen_core::ProviderId;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Zh,
    Ja,
    En,
}

impl Language {
    pub fn cycle(self) -> Self {
        match self {
            Self::Zh => Self::Ja,
            Self::Ja => Self::En,
            Self::En => Self::Zh,
        }
    }

    pub fn html_lang(self) -> &'static str {
        match self {
            Self::Zh => "zh",
            Self::Ja => "ja",
            Self::En => "en",
        }
    }

    /// Label of this language, shown on the button that switches *to* it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Zh => "中文",
            Self::Ja => "日本語",
            Self::En => "English",
        }
    }
}

pub struct Copy {
    pub subtitle: &'static str,
    pub sources_title: &'static str,
    pub sources_hint: &'static str,
    pub config_title: &'static str,
    pub config_hint: &'static str,
    pub result_title: &'static str,
    pub result_hint: &'static str,
    pub upload_base: &'static str,
    pub upload_product: &'static str,
    pub upload_people: &'static str,
    pub drag_drop: &'static str,
    pub replacement_hint: &'static str,
    pub provider: &'static str,
    pub api_key: &'static str,
    pub model_id: &'static str,
    pub show_key: &'static str,
    pub hide_key: &'static str,
    pub forget_key: &'static str,
    pub missing_key: &'static str,
    pub style: &'static str,
    pub aspect_ratio: &'static str,
    pub resolution: &'static str,
    pub custom_instruction: &'static str,
    pub custom_placeholder: &'static str,
    pub generate: &'static str,
    pub generating: &'static str,
    pub download: &'static str,
    pub error: &'static str,
    pub remove: &'static str,
    pub resolution_info: &'static str,
    pub mode_upload: &'static str,
    pub mode_generate: &'static str,
    pub hair_color: &'static str,
    pub skin_tone: &'static str,
    pub ethnicity: &'static str,
    pub expression: &'static str,
    pub gender: &'static str,
    pub generate_model: &'static str,
    pub generating_model: &'static str,
    pub save_model: &'static str,
    pub model_preview: &'static str,
    pub ready: &'static str,
    pub composing: &'static str,
    pub composing_hint: &'static str,
    pub missing_base: &'static str,
    pub missing_reference: &'static str,
}

pub fn copy(language: Language) -> &'static Copy {
    match language {
        Language::Zh => &ZH,
        Language::Ja => &JA,
        Language::En => &EN,
    }
}

const ZH: Copy = Copy {
    subtitle: "专业商业图像混合生成",
    sources_title: "来源",
    sources_hint: "组合你的输入",
    config_title: "配置",
    config_hint: "设定生成意图",
    result_title: "结果",
    result_hint: "确认你的构图",
    upload_base: "基础图片（必填）",
    upload_product: "产品替换（可选）",
    upload_people: "人物替换",
    drag_drop: "拖拽或点击上传",
    replacement_hint: "请至少提供一项替换——产品图、上传的人物，或生成的模特。",
    provider: "AI 模型",
    api_key: "密钥",
    model_id: "模型",
    show_key: "显示",
    hide_key: "隐藏",
    forget_key: "忘记密钥",
    missing_key: "请先填写当前模型的密钥",
    style: "风格",
    aspect_ratio: "宽高比",
    resolution: "分辨率",
    custom_instruction: "自定义指令",
    custom_placeholder: "例如：「把背景换成夕阳沙滩……」",
    generate: "混合生成",
    generating: "生成中…",
    download: "下载",
    error: "错误",
    remove: "移除",
    resolution_info: "更高的分辨率需要更长生成时间。",
    mode_upload: "上传照片",
    mode_generate: "生成模特",
    hair_color: "发色",
    skin_tone: "肤色",
    ethnicity: "人种",
    expression: "表情",
    gender: "性别",
    generate_model: "生成模特",
    generating_model: "模特生成中…",
    save_model: "保存模特",
    model_preview: "模特预览",
    ready: "准备就绪",
    composing: "正在合成你的图像…",
    composing_hint: "高分辨率渲染最多可能需要 30 秒。",
    missing_base: "请先上传基础图",
    missing_reference: "请至少提供一项替换——产品图、上传的人物，或生成的模特",
};

const EN: Copy = Copy {
    subtitle: "Professional Commercial Image Remixing",
    sources_title: "Sources",
    sources_hint: "Compose your inputs",
    config_title: "Configuration",
    config_hint: "Set the intent",
    result_title: "Result",
    result_hint: "Confirm your composition",
    upload_base: "Base Image (Required)",
    upload_product: "Product Replacement (Optional)",
    upload_people: "People Replacement",
    drag_drop: "Drag & drop or click to upload",
    replacement_hint: "Provide at least one replacement — a product photo, an uploaded person, or a generated model.",
    provider: "AI Model",
    api_key: "API key",
    model_id: "Model",
    show_key: "Show",
    hide_key: "Hide",
    forget_key: "Forget key",
    missing_key: "Enter an API key for the selected model first",
    style: "Style",
    aspect_ratio: "Aspect Ratio",
    resolution: "Resolution",
    custom_instruction: "Custom Instructions",
    custom_placeholder: "E.g., \"Make the background a sunset beach...\"",
    generate: "Remix Image",
    generating: "Generating...",
    download: "Download",
    error: "Error",
    remove: "Remove",
    resolution_info: "Higher resolutions take longer to generate.",
    mode_upload: "Upload Photo",
    mode_generate: "Generate Model",
    hair_color: "Hair Color",
    skin_tone: "Skin Tone",
    ethnicity: "Ethnicity",
    expression: "Expression",
    gender: "Gender",
    generate_model: "Generate Model",
    generating_model: "Creating Model...",
    save_model: "Save Model",
    model_preview: "Model Preview",
    ready: "Ready to create",
    composing: "Composing your image…",
    composing_hint: "High resolution rendering may take up to 30 seconds.",
    missing_base: "Upload a base image first",
    missing_reference: "Provide at least one replacement — a product, an uploaded person, or a generated model",
};

const JA: Copy = Copy {
    subtitle: "プロフェッショナル商業画像リミックス",
    sources_title: "ソース",
    sources_hint: "入力を組み合わせる",
    config_title: "設定",
    config_hint: "生成の意図を設定",
    result_title: "結果",
    result_hint: "構図を確認",
    upload_base: "ベース画像（必須）",
    upload_product: "製品置き換え（オプション）",
    upload_people: "人物置き換え",
    drag_drop: "ドラッグ＆ドロップまたはクリック",
    replacement_hint: "製品画像・アップロードした人物・生成したモデルのいずれかを少なくとも一つ指定してください。",
    provider: "AIモデル",
    api_key: "キー",
    model_id: "モデル",
    show_key: "表示",
    hide_key: "隠す",
    forget_key: "キーを忘れる",
    missing_key: "選択中のモデルのキーを入力してください",
    style: "スタイル",
    aspect_ratio: "アスペクト比",
    resolution: "解像度",
    custom_instruction: "カスタム指示",
    custom_placeholder: "例：「背景を夕日のビーチにして...」",
    generate: "画像をリミックス",
    generating: "生成中...",
    download: "ダウンロード",
    error: "エラー",
    remove: "削除",
    resolution_info: "高解像度は生成に時間がかかります。",
    mode_upload: "写真をアップロード",
    mode_generate: "モデルを生成",
    hair_color: "髪の色",
    skin_tone: "肌の色",
    ethnicity: "人種",
    expression: "表情",
    gender: "性別",
    generate_model: "モデルを生成",
    generating_model: "モデル生成中...",
    save_model: "モデルを保存",
    model_preview: "モデルプレビュー",
    ready: "準備完了",
    composing: "画像を合成中…",
    composing_hint: "高解像度レンダリングには最大30秒かかる場合があります。",
    missing_base: "ベース画像をアップロードしてください",
    missing_reference: "製品画像・アップロードした人物・生成したモデルのいずれかを指定してください",
};

#[derive(Clone, Copy)]
pub struct Style {
    pub id: &'static str,
    pub label_zh: &'static str,
    pub label_ja: &'static str,
    pub label_en: &'static str,
    pub prompt: &'static str,
}

impl Style {
    pub fn label(self, language: Language) -> &'static str {
        match language {
            Language::Zh => self.label_zh,
            Language::Ja => self.label_ja,
            Language::En => self.label_en,
        }
    }
}

pub const STYLES: &[Style] = &[
    Style {
        id: "minimalist_luxury",
        label_zh: "极简 & 奢华",
        label_ja: "ミニマリスト & ラグジュアリー",
        label_en: "Minimalist & Luxurious",
        prompt: "A high-end, minimalist aesthetic with plenty of negative space, elegant typography, and a premium feel. Use neutral tones, gold or silver accents, and soft lighting.",
    },
    Style {
        id: "natural_organic",
        label_zh: "自然 & 有机",
        label_ja: "ナチュラル & オーガニック",
        label_en: "Natural & Organic",
        prompt: "A fresh, earthy look featuring natural textures like wood, stone, or leaves. Soft daylight, green and brown tones, emphasizing purity and sustainability.",
    },
    Style {
        id: "cyberpunk_neon",
        label_zh: "赛博朋克 & 霓虹",
        label_ja: "サイバーパンク & ネオン",
        label_en: "Cyberpunk & Neon",
        prompt: "Futuristic, high-contrast style with vibrant neon lights, dark backgrounds, and a tech-heavy atmosphere. Glitch effects and bold, glowing colors.",
    },
    Style {
        id: "retro_vintage",
        label_zh: "复古 & 怀旧",
        label_ja: "レトロ & ヴィンテージ",
        label_en: "Retro & Vintage",
        prompt: "Nostalgic style reminiscent of the 70s, 80s, or 90s. Grainy textures, washed-out colors, bold serif fonts, and classic layout elements.",
    },
    Style {
        id: "bold_pop",
        label_zh: "大胆 & 波普艺术",
        label_ja: "ボールド & ポップアート",
        label_en: "Bold & Pop Art",
        prompt: "Eye-catching, vibrant colors with strong outlines and comic-book style elements. High energy, playful, and dynamic composition.",
    },
    Style {
        id: "corporate_clean",
        label_zh: "商务 & 简洁",
        label_ja: "コーポレート & クリーン",
        label_en: "Corporate & Clean",
        prompt: "Professional, trustworthy business style. Blue and white color palette, clean grid layout, sans-serif fonts, and a focus on clarity.",
    },
];

#[derive(Clone, Copy)]
pub struct Choice {
    pub value: &'static str,
    pub zh: &'static str,
    pub ja: &'static str,
    pub en: &'static str,
}

impl Choice {
    pub fn label(self, language: Language) -> &'static str {
        match language {
            Language::Zh => self.zh,
            Language::Ja => self.ja,
            Language::En => self.en,
        }
    }
}

pub const GENDERS: &[Choice] = &[
    Choice { value: "Female", zh: "女性", ja: "女性", en: "Female" },
    Choice { value: "Male", zh: "男性", ja: "男性", en: "Male" },
];

pub const HAIR: &[Choice] = &[
    Choice { value: "Black", zh: "黑色", ja: "黒", en: "Black" },
    Choice { value: "Brown", zh: "棕色", ja: "茶", en: "Brown" },
    Choice { value: "Blonde", zh: "金色", ja: "金", en: "Blonde" },
    Choice { value: "Red", zh: "红色", ja: "赤", en: "Red" },
    Choice { value: "Grey", zh: "灰色", ja: "グレー", en: "Grey" },
    Choice { value: "White", zh: "白色", ja: "白", en: "White" },
    Choice { value: "Dyed (Pink/Blue)", zh: "挑染（粉/蓝）", ja: "派手髪", en: "Dyed (Pink/Blue)" },
];

pub const SKIN: &[Choice] = &[
    Choice { value: "Fair", zh: "白皙", ja: "色白", en: "Fair" },
    Choice { value: "Light", zh: "偏白", ja: "明るめ", en: "Light" },
    Choice { value: "Medium", zh: "中等", ja: "中間", en: "Medium" },
    Choice { value: "Olive", zh: "橄榄色", ja: "オリーブ", en: "Olive" },
    Choice { value: "Tan", zh: "小麦色", ja: "小麦色", en: "Tan" },
    Choice { value: "Brown", zh: "褐色", ja: "褐色", en: "Brown" },
    Choice { value: "Dark", zh: "深色", ja: "暗め", en: "Dark" },
];

pub const ETHNICITIES: &[Choice] = &[
    Choice { value: "Asian", zh: "亚洲", ja: "アジア系", en: "Asian" },
    Choice { value: "Black", zh: "非裔", ja: "黒人系", en: "Black" },
    Choice { value: "Caucasian", zh: "欧裔", ja: "白人系", en: "Caucasian" },
    Choice { value: "Hispanic", zh: "拉丁裔", ja: "ヒスパニック", en: "Hispanic" },
    Choice { value: "Indian", zh: "印度", ja: "インド系", en: "Indian" },
    Choice { value: "Middle Eastern", zh: "中东", ja: "中東系", en: "Middle Eastern" },
    Choice { value: "Mixed", zh: "混血", ja: "ミックス", en: "Mixed" },
];

pub const EXPRESSIONS: &[Choice] = &[
    Choice { value: "Neutral", zh: "自然", ja: "自然・真顔", en: "Neutral" },
    Choice { value: "Smiling", zh: "微笑", ja: "笑顔", en: "Smiling" },
    Choice { value: "Laughing", zh: "大笑", ja: "笑い", en: "Laughing" },
    Choice { value: "Confident", zh: "自信", ja: "自信", en: "Confident" },
    Choice { value: "Serious", zh: "严肃", ja: "シリアス", en: "Serious" },
    Choice { value: "Surprised", zh: "惊讶", ja: "驚き", en: "Surprised" },
];

pub const ASPECTS: &[&str] = &["1:1", "9:16", "16:9", "3:4", "4:3"];
pub const RESOLUTIONS: &[&str] = &["1K", "2K"];

pub fn aspect_label(value: &str) -> &'static str {
    match value {
        "1:1" => "1:1 (Square)",
        "9:16" => "9:16 (Story/Reel)",
        "16:9" => "16:9 (Landscape)",
        "3:4" => "3:4 (Portrait)",
        "4:3" => "4:3 (Standard)",
        _ => "1:1 (Square)",
    }
}

pub fn provider_storage_key(provider: ProviderId) -> &'static str {
    match provider {
        ProviderId::Grok => "poster-gen.credential.grok",
        ProviderId::Gemini => "poster-gen.credential.gemini",
    }
}
