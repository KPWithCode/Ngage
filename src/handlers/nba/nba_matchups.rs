use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Game {
    id: u32,
    date: String,
    home_team: Team,
    visitor_team: Team,
}

#[derive(Deserialize)]
struct Team {
    id: u32,
    abbreviation: String,
    full_name: String,
    name: String,
}

#[derive(Deserialize)]
struct PlayerStats {
    player_name: String,
    ppg: f64,
    apg: f64,
    rpg: f64,
    spg: f64,
    bpg: f64,
    topg: f64,
    mpg: f64,
    ft_percentage: f64,
    fg_percentage: f64,
    threept_percentage: f64,
}

async fn get_games(date: &str) -> Result<Vec<Game>, reqwest::Error> {
    let url = format!("https://www.balldontlie.io/api/v1/games?date={}", date);
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let games: Vec<Game> = response.json().await?;
    Ok(games)
}

async fn get_player_stats(player_id: u32) -> Result<Vec<PlayerStats>, reqwest::Error> {
    let url = format!(
        "https://www.balldontlie.io/api/v1/players/{}/stats",
        player_id
    );
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let stats: Vec<PlayerStats> = response.json().await?;
    Ok(stats)
}



fn filter_player_stats(
    player_stats: &[PlayerStats],
    opposing_team: &str,
) -> Vec<&PlayerStats> {
    player_stats
        .iter()
        .filter(|stats| stats.team_abbreviation != opposing_team)
        .collect()
}

async fn get_players_stats_for_games(
    games: &[Game],
) -> Result<Vec<Vec<&PlayerStats>>, reqwest::Error> {
    let mut players_stats = Vec::new();

    for game in games {
        let home_team_id = game.home_team.id;
        let visitor_team_id = game.visitor_team.id;

        let home_team_stats = get_player_stats(home_team_id).await?;
        let visitor_team_stats = get_player_stats(visitor_team_id).await?;

        let home_team_filtered_stats = filter_player_stats(&home_team_stats, &game.visitor_team.abbreviation);
        let visitor_team_filtered_stats = filter_player_stats(&visitor_team_stats, &game.home_team.abbreviation);

        players_stats.push(home_team_filtered_stats);
        players_stats.push(visitor_team_filtered_stats);
    }

    Ok(players_stats)
}