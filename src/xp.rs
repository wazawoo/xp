// TODO: define serde clearly (case, etc)
// TODO: unambiguouis but flexible date formats (year, yearmonth, yearmonthday)
// TODO: extra fields allowed (ignored unless supported by spec, but you can use for your own purposes)
// TODO: vscode extension to treat .xp as .json for syntax highlighting

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Xp {
    personal_info: PersonalInfo,
    skills: Vec<Skill>,
    links: Vec<Link>,
    education: Vec<Education>,
    experience: Vec<Experience>
}

#[derive(Serialize, Deserialize)]
pub struct PersonalInfo {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    name: String,
    category: Option<String>,
    years_of_experience: Option<u32>
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    // TODO: use poper path type?
    url: String,
    title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Experience {
    position: Option<String>,
    company_name: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    bullets: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub struct Education {
    degree: Option<String>,
    majors: Vec<Major>,
    minors: Vec<Minor>,
    institution_name: Option<String>,
    start_year: Option<String>,
    end_year: Option<String>,
    gpa: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Major {
    name: String,
    gpa: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Minor {
    name: String,
    gpa: Option<String>
}