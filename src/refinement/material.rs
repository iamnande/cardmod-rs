use crate::card::Card;
use crate::item::Item;
use crate::limitbreak::LimitBreak;
use crate::magic::Magic;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Material {
    Card(card::Card),
    Item(item::Item),
    LimitBreak(limitbreak::LimitBreak),
    Magic(magic::Magic),
}
