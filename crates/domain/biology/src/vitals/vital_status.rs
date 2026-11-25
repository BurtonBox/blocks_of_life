use crate::vitals::live_vitals::LiveVitals;
use crate::vitals::post_mortem_report::PostMortemReport;

#[derive(Debug)]
pub enum VitalStatus {
    Alive(LiveVitals),
    Deceased(PostMortemReport),
}
