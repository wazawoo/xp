// TODO: define serde clearly (case, etc)
// TODO: unambiguouis but flexible date formats (year, yearmonth, yearmonthday)
// TODO: extra fields allowed (ignored unless supported by spec, but you can use for your own purposes)
// TODO: vscode extension to treat .xp as .json for syntax highlighting

use schemars::JsonSchema;
use serde::{Serialize};
use eserde::{Deserialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Xp {
    personal_info: PersonalInfo,
    links: Vec<Link>,
    work: Vec<Work>,
    education: Vec<Education>,
    skill_categories: Vec<SkillCategory>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct PersonalInfo {
    name: String,
    phone_number: Option<String>,
    email: Option<String>,
    website: Option<String>,
    physical_location: Option<String>,
    work_location_preference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Link {
    // TODO: use poper path type?
    id: Option<String>,
    url: String,
    title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Work {
    id: Option<String>,
    position: Option<String>,
    company_name: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    bullets: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SkillCategory {
    category: String,
    include: bool,
    skills: Vec<String>
}
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Education {
    degree: Option<String>,
    majors: Vec<Major>,
    minors: Vec<Minor>,
    institution_name: Option<String>,
    start_year: Option<isize>,
    end_year: Option<isize>,
    gpa: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Major {
    name: String,
    gpa: Option<String>
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Minor {
    name: String,
    gpa: Option<String>
}