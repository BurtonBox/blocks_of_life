#[derive(Debug, Default)]
pub enum LimbStatus {
    #[default]
    Intact,
    Injured(String),
    Amputated,
    Prosthetic,
}
