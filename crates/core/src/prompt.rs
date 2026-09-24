use crate::PeopleConfig;

pub fn portrait_prompt(config: &PeopleConfig) -> String {
    format!(
        r#"
  Generate a high-quality studio portrait of a {gender} model.

  Attributes:
  - Ethnicity: {ethnicity}
  - Skin Tone: {skin}
  - Hair Color: {hair}
  - Expression: {expression}

  Style:
  - Professional model audition photo or high-end fashion portrait.
  - Neutral background (grey or white).
  - Soft, professional studio lighting.
  - Focus on face and upper body.
  - Ultra-realistic skin texture, visible pores, natural features.
  - High resolution photography style (85mm lens).
  - NO text or artifacts.
"#,
        gender = config.gender,
        ethnicity = config.ethnicity,
        skin = config.skin_tone,
        hair = config.hair_color,
        expression = config.expression,
    )
}

pub fn remix_prompt(style_description: &str, custom_prompt: &str) -> String {
    let custom = custom_prompt.trim();
    let has_custom = !custom.is_empty();
    let user = if has_custom {
        custom
    } else {
        "Create a visually stunning variation maintaining the core message."
    };
    let typography = if has_custom {
        "- Professional typography if text is generated (though prioritize visual composition)."
    } else {
        "- DO NOT generate any text, letters, or typography in the image. The output must be purely visual without copy."
    };
    format!(
        r#"
    TASK: Remix the base image into a high-quality commercial poster or social media ad.

    STYLE GUIDELINES:
    {style_description}

    USER INSTRUCTIONS:
    {user}

    OUTPUT REQUIREMENTS:
    - High resolution commercial quality.
    - Seamless integration of replaced objects/people if specified.
    {typography}
"#
    )
}
