use structs::EspnError;
use ureq;
mod structs;
pub use structs::{News, Teams};

const REQUEST_PATH: &str = "https://site.api.espn.com/apis/site/v2/sports";

pub enum Sport {
    MensCollegeBasketball,
    WomensCollegeBasketball,
    CollegeFootball,
}

impl Sport {
    fn get_path(&self) -> &str {
        match self {
            Sport::MensCollegeBasketball => "basketball/mens-college-basketball",
            Sport::WomensCollegeBasketball => "basketball/womens-college-basketball",
            Sport::CollegeFootball => "football/college-football",
        }
    }
}

pub fn get_news(sport: &Sport) -> Result<News, EspnError> {
    let req_path = format!("{}/{}/{}", REQUEST_PATH, sport.get_path(), "news");

    let request = ureq::get(&req_path).call()?;
    Ok(request.into_json()?)
}

pub fn get_teams(sport: &Sport) -> Result<Teams, EspnError> {
    let req_path = format!("{}/{}/{}", REQUEST_PATH, sport.get_path(), "teams");

    let request = ureq::get(&req_path).call()?;
    Ok(request.into_json()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_success() {
        assert!(get_news(&Sport::MensCollegeBasketball).is_ok());
        assert!(get_news(&Sport::WomensCollegeBasketball).is_ok());
        assert!(get_news(&Sport::CollegeFootball).is_ok());
    }

    #[test]
    fn test_teams() {
        dbg!(get_teams(&Sport::MensCollegeBasketball));
        assert!(false);
    }
}
