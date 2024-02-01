use crate::parser::tokens::{expression::literal::primitive::number, splay, splay_mods};

splay_mods! {
    markup: [paragraph, sentence, number, word]
    subs: [paragraph, sentence, word]
}
