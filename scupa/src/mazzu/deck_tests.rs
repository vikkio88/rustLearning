use super::*;

#[test]
fn it_builds_with_40_cards_by_default() {
    let deck = Deck::new();
    assert_eq!(deck.len(), 40);
}

#[test]
fn it_comes_back_with_the_right_number_of_cards_when_drawing() {
    let mut deck = Deck::new();
    let res: Option<Card> = deck.draw();
    assert_eq!(res.is_some(), true);
    assert_eq!(deck.len(), 39);
}

#[test]
fn it_comes_back_with_none_if_drawing_after_40_times() {
    let mut deck = Deck::new();
    for _ in 0..40 {
        let res: Option<Card> = deck.draw();
        assert_eq!(res.is_some(), true);
    }
    
    let res: Option<Card> = deck.draw();
    assert_eq!(res.is_none(), true);
    assert_eq!(deck.len(), 0);
}
