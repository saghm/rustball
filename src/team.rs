pub const ALL_TEAMS : [&'static str; 30] = [
    "ARI", "ATL", "BAL", "BOS", "CHC", "CWS", "CIN", "CLE", "COL", "DET",
    "HOU", "KC", "LAA", "LAD", "MIA", "MIL", "MIN", "NYM", "NYY", "OAK", "PHI",
    "PIT", "SD", "SF", "SEA", "STL", "TB", "TEX", "TOR", "WAS"
];

pub fn get_full_team_name(abbrev: &str) -> Option<&'static str> {
    let name = match abbrev {
        "ARI" => "Arizona Diamondbacks",
        "ATL" => "Atlanta Braves",
        "BAL" => "Baltimore Orioles",
        "BOS" => "Boston Red Sox", // Best team!
        "CHC" => "Chicago Cubs",
        "CIN" => "Cinncinati Reds",
        "CLE" => "Cleveland Indians",
        "COL" => "Colorado Rockies",
        "CWS" => "Chicago White Sox",
        "DET" => "Detroit Tigers",
        "HOU" => "Houston Astros",
        "KC" => "Kansas City Royals",
        "LAA" => "Los Angeles Angels of Anaheim",
        "LAD" => "Los Angeles Dodgers",
        "MIA" => "Miami Marlins",
        "MIL" => "Milwaukee Brewers",
        "MIN" => "Minnesota Twins",
        "NYM" => "New York Mets",
        "NYY" => "New York Yankees", // Worst team
        "OAK" => "Oakland Athletics",
        "PHI" => "Philadelphia Phillies",
        "PIT" => "Pittsburgh Pirates",
        "SD" => "San Diego Padres",
        "SEA" => "Seattle Mariners",
        "SF" => "San Franciso Giants",
        "STL" => "St. Louis Cardinals",
        "TEX" => "Texas",
        "TB" => "Tampa Bay Rays",
        "TOR" => "Toronto Blue Jays",
        "WAS" => "Washington Nationals",
        _ => return None
    };

    Some(name)
}
