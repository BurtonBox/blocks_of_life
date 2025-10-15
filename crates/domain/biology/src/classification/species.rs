#![allow(dead_code)]
pub enum Kingdom {
    Animalia,
    Plantae,
    Fungi,
    Protista,
    Archaea,
    Bacteria,
}

#[allow(dead_code)]
pub struct Species {
    pub kingdom: Kingdom,
    pub name: &'static str,
    pub scientific_name: &'static str,
}
