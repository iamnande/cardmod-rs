use crate::refinement::Material;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: Material,
    pub target: Material,
    pub numerator: u8,
    pub denominator: u8,
}

pub static REFINEMENTS: &[Edge] = &[
    Edge { source: Material::Card(card::Card::Gesper), target: Material::Item(item::Item::BlackHole), numerator: 1, denominator: 1 },
    Edge { source: Material::Item(item::Item::BlackHole), target: Material::Magic(magic::Magic::Degenerator), numerator: 1, denominator: 1 },
];
