use crate::FastConventionalConfig;
use miette::Result;

pub fn run() -> Result<()> {
    let config = FastConventionalConfig {
        use_angular: Some(true),
        types: Some(vec!["custom_type".to_string()]),
        scopes: Some(vec![
            "src".to_string(),
            "actions".to_string(),
            "manpages".to_string(),
            "readme".to_string(),
            "e2e".to_string(),
            "unit".to_string(),
        ]),
    };

    let example: String = config.try_into()?;

    println!("{}", example);

    Ok(())
}
