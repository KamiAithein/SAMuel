use std::{
    fmt::Display,
    sync::{Arc, Mutex},
    time::Duration,
};

use steamworks::{Client, UserAchievementStored};
use tokio::{sync::Notify, time::timeout};

pub struct SteamAchievement {
    pub achievement_id: String,
    pub achievement_name: String,
    pub achievement_description: String,
    pub achievement_got: bool,
}

impl Display for SteamAchievement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<5}{:<20}{:<30}{:<45}",
            if self.achievement_got { "✅" } else { "❌" },
            self.achievement_id,
            self.achievement_name,
            (&self.achievement_description[..42.min(self.achievement_description.len())])
                .to_string()
                + if self.achievement_description.len() >= 42 {
                    "..."
                } else {
                    ""
                }
        )
    }
}

pub fn print_achievement_full(achievement: &SteamAchievement) {
    println!(
        "{:<5}{:<20}{:<30}{:<45}",
        if achievement.achievement_got {
            "✅"
        } else {
            "❌"
        },
        achievement.achievement_id,
        achievement.achievement_name,
        achievement.achievement_description
    )
}
pub fn print_get_achievements_header() {
    println!(
        "{:<5} {:<20}{:<30}{:<45}",
        "got", "id", "name", "description"
    )
}
pub fn trigger_achievement(appid: &u32, achievementid: String) {
    let (client, single) = Client::init_app(*appid).unwrap();

    let user_stats = client.user_stats();

    let achievement = user_stats.achievement(&achievementid);

    achievement.set().unwrap(); //TODO error
}

pub fn clear_achievement(appid: &u32, achievementid: String) {
    let (client, single) = Client::init_app(*appid).unwrap();

    let user_stats = client.user_stats();

    let achievement = user_stats.achievement(&achievementid);

    achievement.clear().unwrap() //TODO error
}
pub async fn get_achievements(appid: &u32) -> Vec<SteamAchievement> {
    let (client, single) = Client::init_app(*appid).unwrap();

    let user_stats = client.user_stats();

    let names = user_stats
        .get_achievement_names()
        .expect("failed to get achievement list"); // TODO error

    names
        .iter()
        .map(|name| {
            let achievement = user_stats.achievement(name);

            let local_name = achievement
                .get_achievement_display_attribute("name")
                .unwrap(); // TODO error
            let local_desc = achievement
                .get_achievement_display_attribute("desc")
                .unwrap(); // TODO error

            SteamAchievement {
                achievement_id: name.to_owned(),
                achievement_name: local_name.to_owned(),
                achievement_description: local_desc.to_owned(),
                achievement_got: achievement.get().unwrap(),
            }
        })
        .collect::<Vec<SteamAchievement>>()
}
