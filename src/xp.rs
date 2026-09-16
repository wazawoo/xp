// TODO: define serde clearly (case, etc)
// TODO: unambiguouis but flexible date formats (year, yearmonth, yearmonthday)
// TODO: extra fields allowed (ignored unless supported by spec, but you can use for your own purposes)
// TODO: vscode extension to treat .xp as .json for syntax highlighting

use schemars::JsonSchema;
use serde::{Serialize};
use eserde::{Deserialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Xp {
    /// Identifiable information for the person
    personal_info: PersonalInfo,
    /// Links related to the person
    links: Vec<Link>,
    /// Work experience started/completed by the person
    work: Vec<Work>,
    /// Education started/completed by the person
    education: Vec<Education>,
    /// Categories of skills possessed by the person
    skill_categories: Vec<SkillCategory>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct PersonalInfo {
    /// Name (format unspecified)
    name: String,
    /// Phone number (format unspecified)
    phone_number: Option<String>,
    /// Email address (format unspecified)
    email: Option<String>,
    /// Personal website url (format unspecified)
    website: Option<String>,
    /// Physical location, if applicable
    physical_location: Option<String>,
    /// Work location preference, if applicable
    work_location_preference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Link {
    /// Identifier, potentially unique
    id: Option<String>,
    /// Web url (format unspecified)
    url: String,
    /// Title for display or alt-text
    title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Work {
    /// Identifier, potentially unique
    id: Option<String>,
    /// Title of started/completed work position
    position: Option<String>,
    /// Company name
    company_name: Option<String>,
    /// Start date (format unspecified to allow "2010", "May 1995", etc.)
    start_date: Option<String>,
    /// End date (format unspecified to allow "Present", for example)
    end_date: Option<String>,
    /// Individual items describing nature of started/completed work
    bullets: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SkillCategory {
    /// Title of skill category
    category: String,
    /// Whether to include skill category (useful for modular resumes)
    include: bool,
    /// List of skills in this category
    skills: Vec<String>
}
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Education {
    /// Name of started/completed degree
    degree: Option<String>,
    /// List of started/completed majors
    majors: Option<Vec<Major>>,
    /// List of started/completed minors
    minors: Option<Vec<Minor>>,
    /// Name of educational institution
    institution_name: Option<String>,
    /// Start date (format unspecified to allow "2010", "May 1995", etc.)
    start_date: Option<String>,
    /// End date (format unspecified to allow "Present", for example)
    end_date: Option<String>,
    /// Grade point average (format unspecified to allow for GPA variants, adjustable decimal precision, etc.)
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