use core_shared::Centimeters;
use crate::vitals::vital_types::Celsius;

#[derive(Debug, Clone)]
pub struct Environment {
    pub climate: Climate,
    pub terrain: Terrain,
    pub altitude: Centimeters,
    pub humidity: f32,
}

#[derive(Debug, Clone)]
pub enum Climate {
    Tropical,
    Temperate,
    Arctic,
    Desert,
    Custom {
        avg_temp: Celsius,
        rainfall_mm: u32
    },
}

#[derive(Debug, Clone)]
pub enum Terrain {
    Forest,
    Plains,
    Mountains,
    Ocean,
    Urban,
    Wetlands,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            climate: Climate::Temperate,
            terrain: Terrain::Plains,
            altitude: Centimeters(0),
            humidity: 0.5,
        }
    }
}