use std::collections::HashMap;
use std::time::SystemTime;

use crate::game::action::Action;
use crate::game::game_state::GameState;
use crate::game::player::{Player, GameResult};

use crate::util::non_nan::NonNan;

#[derive(Clone, Debug)]
pub struct Node {
    visits: usize,
    value: f64,
    player: Player,
    children: HashMap<Action, Option<Node>>,
    children_constructed: bool,
    action: Option<Action>,
    parent_visits: usize,
}

impl Node {
    pub fn new(player: Player, action: Option<Action>, parent_visits: usize) -> Node {
        Node {
            visits: 0,
            value: 0.,
            player,
            children: HashMap::new(),
            children_constructed: false,
            action,
            parent_visits,
        }
    }

    pub fn children(&self) -> &HashMap<Action, Option<Node>> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut HashMap<Action, Option<Node>> {
        &mut self.children
    }

    pub fn best_child(&self) -> Action {
        let (action, _) = self.children().iter().max_by_key(|(_, v)| {
            let weight = match v {
                None => 0.,
                Some(node) => node.search_weight(),
            };

            NonNan::new(weight).unwrap()
        }).unwrap();

        action.clone()
    }

    pub fn weight(&self) -> f64 {
        if self.visits == 0 { 0. } else { self.value / self.visits as f64 }
    }

    pub fn search_weight(&self) -> f64 {
        self.weight() + (2. * (self.parent_visits as f64).ln() / self.visits as f64).sqrt()
    }

    pub fn fully_expanded(&self) -> bool {
        if self.children_constructed {
            for value in self.children().values() {
                if let None = value {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn expand(&mut self, game_state: &GameState) -> Action {
        if !self.children_constructed {
            self.children = action_hash_map(game_state);
            self.children_constructed = true;
        }

        let mut unexpanded_action: Option<Action> = None;

        for (key, value) in self.children().iter() {
            if let None = value {
                unexpanded_action = Some(key.clone());
                break;
            }
        }

        if let Some(action) = unexpanded_action {
            let new_node = Node::new(self.player.next(), Some(action.clone()), self.visits);

            self.children_mut().insert(action.clone(), Some(new_node));
            return action;
        } else {
            panic!("All children were already expanded!");
        }
    }

    pub fn update(&mut self, result: &GameResult) {
        self.visits += 1;
        for node in self.children_mut().values_mut() {
            if let Some(node) = node {
                node.parent_visits += 1;
            }
        }
        self.value += result.score(self.player);
    }
}

fn action_hash_map(game_state: &GameState) -> HashMap<Action, Option<Node>> {
    game_state.possible_actions()
        .iter()
        .map(|action| (action.clone(), None))
        .collect()
}

pub fn mcts_rec(root: &mut Node, game_state: &mut GameState) -> GameResult {
    let result;
    if root.fully_expanded() {
        let action = root.best_child();
        let best_child = root.children_mut().get_mut(&action).unwrap();

        action.apply(game_state);
        result = match best_child {
            Some(best_child) => mcts_rec(best_child, game_state),
            None => panic!("Unexpanded child!"),
        };
        action.unapply(game_state);

        match best_child {
            Some(best_child) => best_child.update(&result),
            None => panic!("Unexpanded child!"),
        };
    } else {
        let action = root.expand(game_state);
        let new_child = root.children_mut().get_mut(&action).unwrap();

        action.apply(game_state);
        result = game_state.play_randomly();
        action.unapply(game_state);

        match new_child {
            Some(new_child) => new_child.update(&result),
            None => panic!("Unexpanded child!"),
        }
    }

    result
}

pub fn mcts(game_state: &mut GameState, time: u128) -> Action {
    let mut root = Node::new(
        game_state.current_player().next(),
        None,
        0,
    );

    let start_time = SystemTime::now();
    let mut count: usize = 0;
    while SystemTime::now().duration_since(start_time).unwrap().as_millis() < time {
        let result = &mcts_rec(&mut root, game_state);
        root.update(&result);
        count += 1;
    }

    println!("Number of simulations: {}", count);
    root.best_child()
}
