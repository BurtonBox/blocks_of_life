#[derive(Debug, Default)]
pub struct PostMortemReport {
    pub cause_of_death: String,
    pub date_of_death: String,
    pub findings: Vec<String>,
}

impl PostMortemReport {
    // A constructor for this struct is also a good idea.
    pub fn new(cause_of_death: String, date_of_death: String, findings: Vec<String>) -> Self {
        Self {
            cause_of_death,
            date_of_death,
            findings,
        }
    }
}
