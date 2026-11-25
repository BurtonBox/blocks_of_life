/// The top-level taxonomic rank used by this project.
///
/// Each kingdom has its own domain crate (`biology_animalia`, `biology_plantae`, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kingdom {
    Animalia,
    Plantae,
    Fungi,
    Protista,
    Archaea,
    Bacteria,
}
