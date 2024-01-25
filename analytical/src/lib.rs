#![allow(dead_code)]
// Recursive approach to "thinking"
// Thought tree records what the bot has learnt about cards and when
// Thought gen 1: Determine clue type
// Thought gen 2: Determine why clue was played
// Thought gen 3: Determine responses

use std::{
    collections::HashMap,
    fmt::Write,
    ops::{Deref, Index},
};

use level1::{
    DiscardThought, EarlyGameThought, FinesseThought, FinessedThought, FiveSaveThought,
    FiveStallThought, PlayThought, PromptThought, PromptedThought, SaveThought, TwoSaveThought,
};
use priority_queue::PriorityQueue;

mod level1;
mod priority_queue;

pub struct Brain {
    thoughts: PriorityQueue<usize, ThoughtType>,
}

impl Brain {
    pub fn new() -> Self {
        let mut queue = PriorityQueue::new();

        queue.push(ThoughtType::EarlyGame(EarlyGameThought {}), 10);

        Self { thoughts: queue }
    }

    pub fn play<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Action {
        let mut thoughts_to_requeue = Vec::new();
        let action_to_return;
        loop {
            let mut thought = self.thoughts.pop().unwrap();
            match thought.poll(game_state) {
                Poll::Pending(action) => {
                    thoughts_to_requeue.push(thought);
                    if let Some(a) = action {
                        action_to_return = a;
                        break;
                    }
                }
                Poll::Finished(Some(action)) => {
                    action_to_return = action;
                    break;
                }
                Poll::Finished(None) => {}
            }
        }
        for thought in thoughts_to_requeue.into_iter().rev() {
            let priority = match thought {
                ThoughtType::Prompt(_) => 20,
                ThoughtType::Prompted(_) => 20,
                ThoughtType::Finesse(_) => 10,
                ThoughtType::Finessed(_) => 10,
                ThoughtType::Play(_) => 5,
                ThoughtType::FiveSave(_) => 6,
                ThoughtType::TwoSave(_) => 6,
                ThoughtType::Save(_) => 9,
                ThoughtType::FiveStall(_) => 1,
                ThoughtType::EarlyGame(_) => 5,
                ThoughtType::Discard(_) => 0,
            };
            self.thoughts.push_front(thought, priority);
        }

        action_to_return
    }
}

pub trait Thought {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll;
}

pub enum Poll {
    Pending(Option<Action>),
    Finished(Option<Action>),
}

pub enum ThoughtType {
    Prompt(PromptThought),
    Prompted(PromptedThought),
    Finesse(FinesseThought),
    Finessed(FinessedThought),
    Play(PlayThought),
    FiveSave(FiveSaveThought),
    TwoSave(TwoSaveThought),
    Save(SaveThought),
    FiveStall(FiveStallThought),
    EarlyGame(EarlyGameThought),
    Discard(DiscardThought),
}

impl Thought for ThoughtType {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll {
        match self {
            ThoughtType::Prompt(t) => t.poll(game_state),
            ThoughtType::Prompted(t) => t.poll(game_state),
            ThoughtType::Finesse(t) => t.poll(game_state),
            ThoughtType::Finessed(t) => t.poll(game_state),
            ThoughtType::Play(t) => t.poll(game_state),
            ThoughtType::FiveSave(t) => t.poll(game_state),
            ThoughtType::TwoSave(t) => t.poll(game_state),
            ThoughtType::Save(t) => t.poll(game_state),
            ThoughtType::FiveStall(t) => t.poll(game_state),
            ThoughtType::EarlyGame(t) => t.poll(game_state),
            ThoughtType::Discard(t) => t.poll(game_state),
        }
    }
}

pub struct GameState<const P: usize, const H: usize> {
    team_hands: [TeammateHand<H>; P],
    player_hand: PlayerHand<H>,
    discarded: Vec<(CardId, Color, usize)>,
    played: HashMap<Color, Vec<(CardId, usize)>>,
    hint_count: usize,
    turn_counter: usize,
}

impl<const P: usize, const H: usize> GameState<P, H> {
    pub fn get_card(&self, id: CardId) -> Card {
        if let Some(card) = self.discarded.iter().find(|(cid, _, _)| *cid == id) {
            return Card {
                id,
                typ: CardType::Discarded(card.1, card.2),
            };
        }
        if let Some(card) = self
            .played
            .iter()
            .flat_map(|(c, s)| s.iter().copied().map(|(id, n)| (id, *c, n)))
            .find(|(cid, _, _)| *cid == id)
        {
            return Card {
                id,
                typ: CardType::Played(card.1, card.2),
            };
        }
        if let Some(card) = self
            .player_hand
            .iter()
            .enumerate()
            .find(|(_, (cid, _, _, _))| *cid == id)
        {
            return Card {
                id,
                typ: CardType::PlayerHand(PlayerCard {
                    color: card.1 .1,
                    number: card.1 .2,
                    touched: card.1 .3,
                    index: card.0,
                }),
            };
        }
        if let Some(card) = self
            .team_hands
            .iter()
            .enumerate()
            .flat_map(|(tmid, tm)| {
                tm.hand
                    .iter()
                    .copied()
                    .enumerate()
                    .map(move |(i, c)| (tmid, c, i))
            })
            .find(|(_, (cid, _, _, _), _)| *cid == id)
        {
            return Card {
                id,
                typ: CardType::TeamHand(TeammateCard {
                    player: card.0,
                    color: card.1 .1,
                    number: card.1 .2,
                    touched: card.1 .3,
                    index: card.2,
                }),
            };
        }

        Card {
            id,
            typ: CardType::InDeck,
        }
    }

    pub fn is_playable(&self, color: ColorFlags, number: NumberFlags) -> bool {
        for color in color.iter() {
            let c = Color::try_from(color).unwrap();
            let stack = self
                .played
                .get(&c)
                .and_then(|s| s.iter().max_by_key(|(_, n)| n));
            for number in number.iter() {
                let n = usize::try_from(number).unwrap();
                let r = match stack {
                    Some((_, i)) => i + 1 == n,
                    None => n == 1,
                };
                if !r {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_critical(&self, color: ColorFlags, number: NumberFlags) -> bool {
        for color in color.iter() {
            let color = Color::try_from(color).unwrap();
            'number: for number in number.iter() {
                let number = usize::try_from(number).unwrap();
                let has_been_played = self
                    .played
                    .get(&color)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|(_, n)| *n == number);
                if has_been_played {
                    continue 'number;
                }
                let discarded_amount = self
                    .discarded
                    .iter()
                    .filter(|(_, c, n)| *c == color && *n == number)
                    .count();
                let critical = match number {
                    1 => discarded_amount == 2,
                    5 => discarded_amount == 0,
                    _ => discarded_amount == 1,
                };
                if critical {
                    return true;
                }
            }
        }

        false
    }

    pub fn playable_cards_in_teammate_hands(
        &self,
    ) -> impl Iterator<Item = (CardId, TeammateCard)> + '_ {
        self.team_hands
            .iter()
            .enumerate()
            .flat_map(|(player, hand)| {
                hand.hand
                    .iter()
                    .copied()
                    .enumerate()
                    .map(move |c| (player, c))
            })
            .filter(|(_, (_, (_, c, n, _)))| self.is_playable((*c).into(), (*n).into()))
            .map(|(player, (index, (id, color, number, touched)))| {
                (
                    id,
                    TeammateCard {
                        player,
                        color,
                        number,
                        touched,
                        index,
                    },
                )
            })
    }

    pub fn critical_cards_in_teammate_hands(
        &self,
    ) -> impl Iterator<Item = (CardId, TeammateCard)> + '_ {
        self.team_hands
            .iter()
            .enumerate()
            .flat_map(|(player, hand)| {
                hand.hand
                    .iter()
                    .copied()
                    .enumerate()
                    .map(move |c| (player, c))
            })
            .filter(|(_, (_, (_, c, n, _)))| self.is_critical((*c).into(), (*n).into()))
            .map(|(player, (index, (id, color, number, touched)))| {
                (
                    id,
                    TeammateCard {
                        player,
                        color,
                        number,
                        touched,
                        index,
                    },
                )
            })
    }

    pub fn get_focus_for_hint(&self, player: usize, hint: Hint) -> Focus {
        self.team_hands[player].determine_focus_for_hint(hint)
    }

    pub fn get_chop_for_teammate(&self, player: usize) -> Option<Card> {
        self.team_hands[player]
            .get_chop()
            .map(|(id, color, number, touched, index)| Card {
                id,
                typ: CardType::TeamHand(TeammateCard {
                    player,
                    color,
                    number,
                    touched,
                    index,
                }),
            })
    }
}

pub struct TeammateHand<const H: usize> {
    hand: [(CardId, Color, usize, bool); H],
    empathy: [(CardId, ColorFlags, NumberFlags, bool); H],
}

impl<const H: usize> TeammateHand<H> {
    fn get_chop(&self) -> Option<(CardId, Color, usize, bool, usize)> {
        self.hand
            .iter()
            .copied()
            .enumerate()
            .map(|(i, (id, c, n, t))| (id, c, n, t, i))
            .filter(|(_, _, _, t, _)| !t)
            .min_by_key(|(_, _, _, _, index)| *index)
    }

    fn determine_focus_for_hint(&self, hint: Hint) -> Focus {
        let hinted_cards: Vec<_> = self
            .hand
            .iter()
            .copied()
            .enumerate()
            .map(|(index, (id, c, number, touched))| (id, c, number, touched, index))
            .filter(|(_, c, n, _, _)| hint.applies_to_card((*c, *n)))
            .collect();
        let chop = self.get_chop();

        let newly_touched_cards: Vec<_> =
            hinted_cards.iter().filter(|(_, _, _, t, _)| !t).collect();
        // More than one card touched for the first time?
        if newly_touched_cards.len() >= 2 {
            if let Some(chop) = chop {
                // Was the chop touched and it wasn't touched before?
                if hinted_cards.iter().any(|c| c.0 == chop.0) && !chop.3 {
                    // Then the chop was focused
                    return Focus::Chop(chop.0, chop.4);
                }
            }
        }

        if let Some((id, _, _, _, index)) = newly_touched_cards.first() {
            // Only one card was newly touched, so that was the focus
            return Focus::NewCard(*id, *index);
        }

        let left_most_card = newly_touched_cards
            .into_iter()
            .copied()
            .min_by_key(|(_, _, _, _, index)| *index)
            .unwrap();
        Focus::LeftMost(left_most_card.0, left_most_card.4)
    }
}

pub struct Thoughts(HashMap<CardId, Vec<ThoughtType>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardId(usize);

impl CardId {
    pub fn resolve<const P: usize, const H: usize>(&self, game_state: &GameState<P, H>) -> Card {
        game_state.get_card(*self)
    }
}

pub struct Card {
    id: CardId,
    typ: CardType,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.typ)
    }
}

impl Card {
    pub fn is_playable<const P: usize, const H: usize>(
        &self,
        game_state: &GameState<P, H>,
    ) -> bool {
        let (cflags, nflags): (ColorFlags, NumberFlags) = match &self.typ {
            CardType::Played(c, n) => ((*c).into(), (*n).into()),
            CardType::Discarded(c, n) => ((*c).into(), (*n).into()),
            CardType::InDeck => return false,
            CardType::TeamHand(c) => (c.color.into(), c.number.into()),
            CardType::PlayerHand(c) => (c.color, c.number),
        };

        game_state.is_playable(cflags, nflags)
    }

    pub fn is_critical<const P: usize, const H: usize>(
        &self,
        game_state: &GameState<P, H>,
    ) -> bool {
        let (cflags, nflags): (ColorFlags, NumberFlags) = match &self.typ {
            CardType::Played(c, n) => ((*c).into(), (*n).into()),
            CardType::Discarded(c, n) => ((*c).into(), (*n).into()),
            CardType::InDeck => return false,
            CardType::TeamHand(c) => (c.color.into(), c.number.into()),
            CardType::PlayerHand(c) => (c.color, c.number),
        };

        game_state.is_critical(cflags, nflags)
    }
}

pub enum CardType {
    Played(Color, usize),
    Discarded(Color, usize),
    InDeck,
    TeamHand(TeammateCard),
    PlayerHand(PlayerCard),
}

pub struct TeammateCard {
    player: usize,
    color: Color,
    number: usize,
    touched: bool,
    index: usize,
}

pub struct PlayerCard {
    color: ColorFlags,
    number: NumberFlags,
    touched: bool,
    index: usize,
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardType::Played(c, n) => write!(f, "{c}{n}"),
            CardType::Discarded(c, n) => write!(f, "{c}{n}"),
            CardType::InDeck => write!(f, "*Not Drawn*"),
            CardType::TeamHand(c) => write!(f, "{}{}", c.color, c.number),
            CardType::PlayerHand(c) => write!(f, "{}{}", c.color, c.number),
        }
    }
}

pub enum Action {
    Play(CardId),
    Discard(CardId),
    Hint(usize, Hint),
}

#[derive(Debug, Clone, Copy)]
pub enum Hint {
    Color(Color),
    Number(usize),
}

impl Hint {
    fn applies_to_card(&self, card: (Color, usize)) -> bool {
        match self {
            Hint::Color(c) if *c == card.0 => true,
            Hint::Number(n) if *n == card.1 => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Blue,
    Yellow,
    Green,
    Red,
    Purple,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Color::Blue => 'b',
            Color::Yellow => 'y',
            Color::Green => 'g',
            Color::Red => 'r',
            Color::Purple => 'p',
        };

        f.write_char(c)
    }
}

impl TryFrom<ColorFlags> for Color {
    type Error = ();

    fn try_from(value: ColorFlags) -> Result<Self, Self::Error> {
        match value {
            ColorFlags::Blue => Ok(Color::Blue),
            ColorFlags::Green => Ok(Color::Green),
            ColorFlags::Purple => Ok(Color::Purple),
            ColorFlags::Red => Ok(Color::Red),
            ColorFlags::Yellow => Ok(Color::Yellow),
            _ => Err(()),
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ColorFlags: u8 {
        const Blue = 1 << 0;
        const Yellow = 1 << 1;
        const Green = 1 << 2;
        const Red = 1 << 3;
        const Purple = 1 << 4;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct NumberFlags: u8 {
        const One = 1 << 0;
        const Two = 1 << 1;
        const Three = 1 << 2;
        const Four = 1 << 3;
        const Five = 1 << 4;
    }
}

impl From<Color> for ColorFlags {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => ColorFlags::Blue,
            Color::Yellow => ColorFlags::Yellow,
            Color::Green => ColorFlags::Green,
            Color::Red => ColorFlags::Red,
            Color::Purple => ColorFlags::Purple,
        }
    }
}

impl std::fmt::Display for ColorFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .iter()
            .map(|b| match b {
                ColorFlags::Blue => 'b',
                ColorFlags::Yellow => 'y',
                ColorFlags::Green => 'g',
                ColorFlags::Red => 'r',
                ColorFlags::Purple => 'p',
                _ => unreachable!(),
            })
            .collect::<String>();

        f.write_str(&s)
    }
}

impl From<usize> for NumberFlags {
    fn from(value: usize) -> Self {
        match value {
            1 => NumberFlags::One,
            2 => NumberFlags::Two,
            3 => NumberFlags::Three,
            4 => NumberFlags::Four,
            5 => NumberFlags::Five,
            _ => unimplemented!("Numbers other than standard hanabi are not supported!"),
        }
    }
}

impl TryFrom<NumberFlags> for usize {
    type Error = ();

    fn try_from(value: NumberFlags) -> Result<Self, Self::Error> {
        match value {
            NumberFlags::One => Ok(1),
            NumberFlags::Two => Ok(2),
            NumberFlags::Three => Ok(3),
            NumberFlags::Four => Ok(4),
            NumberFlags::Five => Ok(5),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for NumberFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .iter()
            .map(|b| (usize::try_from(b)).unwrap().to_string())
            .collect::<String>();

        f.write_str(&s)
    }
}

pub enum Focus {
    Chop(CardId, usize),
    NewCard(CardId, usize),
    LeftMost(CardId, usize),
}

pub struct PlayerHand<const H: usize>([(CardId, ColorFlags, NumberFlags, bool); H]);
impl<const H: usize> Index<usize> for PlayerHand<H> {
    type Output = (CardId, ColorFlags, NumberFlags, bool);

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<const H: usize> Deref for PlayerHand<H> {
    type Target = [(CardId, ColorFlags, NumberFlags, bool); H];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const H: usize> PlayerHand<H> {
    pub fn get_chop(&self) -> Option<(CardId, PlayerCard)> {
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, (_, _, _, touched))| !touched)
            .min_by_key(|(index, _)| *index)
            .map(|(index, (id, color, number, touched))| {
                (
                    id,
                    PlayerCard {
                        color,
                        number,
                        touched,
                        index,
                    },
                )
            })
    }
}
