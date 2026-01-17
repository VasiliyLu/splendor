use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Card<Gem> {
    id: u32,
    level: u8,
    points: u8,
    costs: HashMap<Gem, u8>,
    bonus: Gem,
}
