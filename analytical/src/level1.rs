use crate::{Action, CardId, CardType, GameState, Hint, Poll, Thought};

pub struct PromptThought {
    prompted: Vec<CardId>,
    hinted: CardId,
}

impl Thought for PromptThought {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll {
        let finessed: Vec<_> = self
            .prompted
            .iter()
            .map(|c| c.resolve(game_state))
            .collect();
        let _hinted = self.hinted.resolve(game_state);

        println!("I think a prompt is happening...");
        print!("Have all the prompted cards been played? ");
        if finessed
            .iter()
            .all(|c| matches!(c.typ, CardType::Played(_, _)))
        {
            println!("Yes!");
            println!("Therefore I should complete the line!");
            return Poll::Finished(Some(Action::Play(self.hinted)));
        }
        println!("No!");
        println!("Were any of the prompted card dicarded? ");
        if finessed
            .iter()
            .any(|c| matches!(c.typ, CardType::Discarded(_, _)))
        {
            println!("Yes!");
            println!("Therefore the line is broken, and I should stop thinking about this...");
            return Poll::Finished(None);
        }

        println!("No!");
        println!("The line is still playing out, and I should wait for it");
        Poll::Pending(None)
    }
}

pub struct PromptedThought {
    prompted: CardId,
    hinted: CardId,
}

impl Thought for PromptedThought {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll {
        let card = self.prompted.resolve(game_state);
        println!("I think I am being prompted with a {}...", card);
        println!("I should play that card!");
        Poll::Finished(Some(Action::Play(self.prompted)))
    }
}

pub struct FinesseThought {
    finessed: Vec<CardId>,
    hinted: CardId,
}

impl Thought for FinesseThought {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll {
        let finessed: Vec<_> = self
            .finessed
            .iter()
            .map(|c| c.resolve(game_state))
            .collect();
        let _hinted = self.hinted.resolve(game_state);

        println!("I think a finesse is happening...");
        print!("Have all the finessed cards been played? ");
        if finessed
            .iter()
            .all(|c| matches!(c.typ, CardType::Played(_, _)))
        {
            println!("Yes!");
            println!("Therefore I should complete the line!");
            return Poll::Finished(Some(Action::Play(self.hinted)));
        }
        println!("No!");
        println!("Were any of the finessed card dicarded? ");
        if finessed
            .iter()
            .any(|c| matches!(c.typ, CardType::Discarded(_, _)))
        {
            println!("Yes!");
            println!("Therefore the line is broken, and I should stop thinking about this...");
            return Poll::Finished(None);
        }

        println!("No!");
        println!("The line is still playing out, and I should wait for it");
        Poll::Pending(None)
    }
}

pub struct FinessedThought {
    card: CardId,
}

impl Thought for FinessedThought {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll {
        let card = self.card.resolve(game_state);
        println!("I think I am being finessed with a {}...", card);
        println!("I should play that card!");
        Poll::Finished(Some(Action::Play(self.card)))
    }
}

pub struct PlayThought {
    card: CardId,
    turn: usize,
}

impl Thought for PlayThought {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll {
        let _card = self.card.resolve(game_state);
        println!(
            "I was clued that this card was playable on turn {} ({} turns ago)...",
            self.turn,
            game_state.turn_counter - self.turn
        );
        println!("I will trust that is is playable!");
        Poll::Finished(Some(Action::Play(self.card)))
    }
}

pub struct FiveSaveThought {
    card: CardId,
}

impl Thought for FiveSaveThought {
    fn poll<const P: usize, const H: usize>(
        &mut self,
        game_state: &GameState<P, H>,
    ) -> crate::Poll {
        let card = self.card.resolve(game_state);
        println!(
            "I'm wondering if my five card is playable? I think it's a: {}",
            card
        );
        if card.is_playable(game_state) {
            println!("It is! I should play that then");
            return Poll::Finished(Some(Action::Play(self.card)));
        }

        println!("It's not...I should wait then");
        Poll::Pending(None)
    }
}

pub struct TwoSaveThought {
    card: CardId,
}

impl Thought for TwoSaveThought {
    fn poll<const P: usize, const H: usize>(
        &mut self,
        game_state: &GameState<P, H>,
    ) -> crate::Poll {
        let card = self.card.resolve(game_state);
        println!(
            "I'm wondering if my two card is playable? I think it's a: {}",
            card
        );
        if card.is_playable(game_state) {
            println!("It is! I should play that then");
            return Poll::Finished(Some(Action::Play(self.card)));
        }

        println!("It's not...I should wait then");
        Poll::Pending(None)
    }
}

pub struct SaveThought {
    card: CardId,
}

impl Thought for SaveThought {
    fn poll<const P: usize, const H: usize>(
        &mut self,
        game_state: &GameState<P, H>,
    ) -> crate::Poll {
        let card = self.card.resolve(game_state);
        println!(
            "I'm wondering if my saved card is playable? I think it's a: {}",
            card
        );
        if card.is_playable(game_state) {
            println!("It is! I should play that then");
            return Poll::Finished(Some(Action::Play(self.card)));
        }

        println!("It's not...I should wait then");
        Poll::Pending(None)
    }
}

pub struct FiveStallThought {
    card: CardId,
}

impl Thought for FiveStallThought {
    fn poll<const P: usize, const H: usize>(
        &mut self,
        game_state: &GameState<P, H>,
    ) -> crate::Poll {
        let card = self.card.resolve(game_state);
        println!(
            "I'm wondering if my stalled 5 card is playable? I think it's a: {}",
            card
        );
        if card.is_playable(game_state) {
            println!("It is! I should play that then");
            return Poll::Finished(Some(Action::Play(self.card)));
        }

        println!("It's not...I should wait then");
        Poll::Pending(None)
    }
}

pub struct EarlyGameThought {}

impl Thought for EarlyGameThought {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll {
        if game_state.hint_count == 0 {
            return Poll::Pending(None);
        }

        let playable_cards = game_state.playable_cards_in_teammate_hands();
        for (id, card) in playable_cards {
            println!(
                "I've noticed that one of player {}'s carss is playable!",
                card.player
            );
            if card.touched {
                println!("However, it's already been touched, so they probably know about it...");
                continue;
            }

            // We'll hint that then, as long it would be the focus
            let mut hint = Hint::Number(card.number);
            let mut focus_id = match game_state.get_focus_for_hint(card.player, hint) {
                crate::Focus::Chop(id, _) => id,
                crate::Focus::NewCard(id, _) => id,
                crate::Focus::LeftMost(id, _) => id,
            };
            if focus_id != id {
                // What about with a color hint?
                hint = Hint::Color(card.color);
                focus_id = match game_state.get_focus_for_hint(card.player, hint) {
                    crate::Focus::Chop(id, _) => id,
                    crate::Focus::NewCard(id, _) => id,
                    crate::Focus::LeftMost(id, _) => id,
                };
                if focus_id != id {
                    println!("I can't hint while focusing that card, so I can't hint it...");
                    continue;
                }
            }

            println!("And it's not been touched, let's hint it!");
            return Poll::Pending(Some(Action::Hint(card.player, hint)));
        }

        // There's no hintable playable cards
        // Is there a critical card instead?
        let critical_cards = game_state.critical_cards_in_teammate_hands();
        for (id, card) in critical_cards {
            println!(
                "I've noticed that one of player {}'s cards is critical!",
                card.player
            );
            if card.touched {
                println!("However, it's already been touched, so they probably know about it...");
                continue;
            }

            // We'll hint that then, as long it would be the focus
            let mut hint = Hint::Number(card.number);
            let mut focus_id = match game_state.get_focus_for_hint(card.player, hint) {
                crate::Focus::Chop(id, _) => id,
                crate::Focus::NewCard(id, _) => id,
                crate::Focus::LeftMost(id, _) => id,
            };
            if focus_id != id {
                // What about with a color hint?
                hint = Hint::Color(card.color);
                focus_id = match game_state.get_focus_for_hint(card.player, hint) {
                    crate::Focus::Chop(id, _) => id,
                    crate::Focus::NewCard(id, _) => id,
                    crate::Focus::LeftMost(id, _) => id,
                };
                if focus_id != id {
                    println!("I can't hint while focusing that card, so I can't hint it...");
                    // TODO: With a better playing algo, we should check if this player has a playable card instead
                    // Because of the above check for playable cards, the only instance we'll miss a card is if it's both playable
                    // and critical and we can't properly hint it.
                    //
                    // Additionally, a check for potential prompts/finesses here would help since we can also distract
                    continue;
                }
            }

            println!("And it's not been touched, let's hint it!");
            return Poll::Pending(Some(Action::Hint(card.player, hint)));
        }

        // 2 saves
        let twos_on_chops = game_state
            .team_hands
            .iter()
            .enumerate()
            .map(|(player, h)| (player, h.get_chop()))
            .filter(|(_, c)| c.is_some())
            .filter_map(|(player, c)| {
                c.map(|(id, color, number, touched, index)| {
                    (player, id, color, number, touched, index)
                })
            })
            .filter(|(_, _, _, n, _, _)| *n == 2)
            .filter(|(_, _, color, _, _, _)| {
                game_state
                    .played
                    .get(color)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|(_, n)| *n == 2)
            })
            .filter(|(_, _, _, _, touched, _)| !touched);

        for card in twos_on_chops {
            println!(
                "Player {} has a 2 on the chop, I should probably try and save it...",
                card.0
            );

            let focus_id = match game_state.get_focus_for_hint(card.0, Hint::Number(2)) {
                crate::Focus::Chop(id, _) => id,
                crate::Focus::NewCard(id, _) => id,
                crate::Focus::LeftMost(id, _) => id,
            };
            if focus_id != card.1 {
                println!("I can't hint while focusing that card, so I can't hint it...");
                continue;
            }

            return Poll::Pending(Some(Action::Hint(card.0, Hint::Number(2))));
        }

        // Any 5s stalls?
        let mut fives = game_state
            .team_hands
            .iter()
            .enumerate()
            .flat_map(|(player, h)| {
                h.hand.iter().copied().enumerate().map(
                    move |(index, (id, color, number, touched))| {
                        (player, id, color, number, touched, index)
                    },
                )
            })
            .filter(|(_, _, _, n, _, _)| *n == 5)
            .filter(|(_, _, color, _, _, _)| {
                game_state
                    .played
                    .get(color)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|(_, n)| *n == 2)
            })
            .filter(|(_, _, _, _, touched, _)| !touched);

        if let Some(five_card) = fives.next() {
            return Poll::Pending(Some(Action::Hint(five_card.0, Hint::Number(5))));
        }

        Poll::Finished(None)
    }
}

pub struct DiscardThought {}
impl Thought for DiscardThought {
    fn poll<const P: usize, const H: usize>(&mut self, game_state: &GameState<P, H>) -> Poll {
        let chop = game_state.player_hand.get_chop();

        // TODO: TBH this should be a lot better, we're only handling it like this
        // becuase we're discarding as the *last possible* action that could happen.
        Poll::Pending(Some(Action::Discard(match chop {
            Some((id, _)) => id,
            None => game_state.player_hand[0].0,
        })))
    }
}
