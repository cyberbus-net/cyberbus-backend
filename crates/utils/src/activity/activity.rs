use crate::settings::structs::TrophyActivityConfig;

pub fn get_trophy_for_post(config: &TrophyActivityConfig, post_title: &str) -> Option<String> {
    // If feature is disabled, return None directly
    if !config.enabled {
        return None;
    }

    // Iterate through all configured activities
    for activity in &config.activities {
        // Check if post title contains the activity title pattern
        if post_title.contains(&activity.title_pattern) {
            return Some(activity.trophy_name.clone());
        }
    }

    // If no matching activity found, return None
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::structs::TrophyActivity;

    #[test]
    fn test_get_trophy_for_post() {
        let mut config = TrophyActivityConfig {
            enabled: true,
            activities: vec![
                TrophyActivity {
                    title_pattern: "[第一届晒机大赛]".to_string(),
                    trophy_name: "_1st_show_your_build_competition".to_string(),
                },
            ],
        };

        // Test matching case
        assert_eq!(
            get_trophy_for_post(&config, "[第一届晒机大赛] 我的配置"),
            Some("_1st_show_your_build_competition".to_string())
        );

        // Test non-matching case
        assert_eq!(
            get_trophy_for_post(&config, "普通帖子"),
            None
        );

        // Test disabled feature case
        config.enabled = false;
        assert_eq!(
            get_trophy_for_post(&config, "[第一届晒机大赛] 测试"),
            None
        );
    }
}
