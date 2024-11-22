use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    pub sports: Vec<Sport>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub leagues: Vec<League>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
    pub short_name: String,
    pub slug: String,
    pub teams: Vec<TeamObj>,
    pub year: u64,
    pub season: Season,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub year: u64,
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamObj {
    pub team: Team,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub abbreviation: String,
    pub display_name: String,
    pub short_display_name: String,
    pub name: String,
    pub nickname: String,
    pub location: String,
    pub color: String,
    pub color_alt: Option<String>,
    pub is_active: bool,
    pub is_all_star: bool,
    pub logos: Vec<TeamLogo>,
    pub links: Vec<TeamLink>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamLogo {
    pub href: String,
    pub alt: String,
    pub width: u64,
    pub height: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamLink {
    pub href: String,
    pub text: String,
    pub short_text: String,
}
