use std::collections::HashMap;
use std::time::SystemTime;

use crate::game::action::Action;
use crate::actor::Actor;
use crate::game::game_state::GameState;
use crate::game::player::{Player, GameResult};

use crate::util::non_nan::NonNan;

pub struct Node {
    pub visits: usize,
    value: f64,
    player: Player,
    children: HashMap<Action, Option<Node>>,
    children_constructed: bool,
    state: GameState,
    result: Option<GameResult>,
}

impl Node {
    pub fn new(player: Player, state: GameState, result: Option<GameResult>) -> Node {
        Node {
            visits: 0,
            value: 0.,
            player,
            children: HashMap::new(),
            children_constructed: false,
            state,
            result,
        }
    }

    pub fn children(&self) -> &HashMap<Action, Option<Node>> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut HashMap<Action, Option<Node>> {
        &mut self.children
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }

    pub fn best_child(&self, visits: usize) -> Action {
        let (action, _) = self.children().iter().max_by_key(|(_, v)| {
            let weight = match v {
                None => 0.,
                Some(node) => node.search_weight(visits),
            };

            NonNan::new(weight).unwrap()
        }).unwrap();

        action.clone()
    }

    pub fn best_child_final(&self) -> Action {
        let (action, _) = self.children().iter().max_by_key(|(_, v)| {
            let weight = match v {
                None => 0.,
                Some(node) => node.weight(),
            };

            NonNan::new(weight).unwrap()
        }).unwrap();

        action.clone()
    }

    pub fn weight(&self) -> f64 {
        if self.visits == 0 { 0. } else { self.value / self.visits as f64 }
    }

    pub fn search_weight(&self, parent_visits: usize) -> f64 {
        self.weight() + (2. * (parent_visits as f64).ln() / self.visits as f64).sqrt()
    }

    pub fn fully_expanded(&self) -> bool {
        if self.children_constructed {
            for value in self.children().values() {
                if value.is_none() {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn expand(&mut self) -> Action {
        if !self.children_constructed {
            self.children = action_hash_map(self.state_mut());
            self.children_constructed = true;
        }

        let mut unexpanded_action: Option<Action> = None;

        for (key, value) in self.children().iter() {
            if value.is_none() {
                unexpanded_action = Some(key.clone());
                break;
            }
        }

        if let Some(action) = unexpanded_action {
            let mut new_game_state = self.state().clone();
            let result = action.apply(&mut new_game_state);
            let new_node = Node::new(
                self.player.next(),
                new_game_state,
                result,
            );

            self.children_mut().insert(action.clone(), Some(new_node));
            action
        } else {
            panic!("All children were already expanded!");
        }
    }

    pub fn update(&mut self, result: GameResult) {
        self.visits += 1;
        self.value += result.score(self.player);
    }
}

fn action_hash_map(game_state: &GameState) -> HashMap<Action, Option<Node>> {
    game_state.possible_actions()
        .iter()
        .map(|action| (action.clone(), None))
        .collect()
}

pub fn mcts_rec(root: &mut Node) -> GameResult {
    if root.fully_expanded() {
        let action = root.best_child(root.visits);
        let best_child = root.children_mut().get_mut(&action).unwrap();

        let result = match best_child {
            None => panic!("Unexpanded child!"),
            Some(best_child) => {
                match best_child.result {
                    Some(game_result) => game_result,
                    None => mcts_rec(best_child),
                }
            }
        };

        match best_child {
            Some(best_child) => best_child.update(result),
            None => panic!("Unexpanded child!"),
        };

        result
    } else {
        let action = root.expand();
        let new_child = root.children_mut().get_mut(&action).unwrap();

        let result = match new_child {
            None => panic!("Unexpanded child!"),
            Some(new_child) => {
                match new_child.result {
                    Some(game_result) => game_result,
                    None => root.state_mut().play_randomly(),
                }
            }
        };

        let new_child = root.children_mut().get_mut(&action).unwrap();
        match new_child {
            Some(new_child) => new_child.update(result),
            None => panic!("Unexpanded child!"),
        };

        result
    }
}

pub fn mcts(game_state: &mut GameState, time: u128) -> Action {
    let mut root = Node::new(
        game_state.current_player().next(),
        game_state.clone(),
        None,
    );

    let start_time = SystemTime::now();
    let mut count: usize = 0;
    while SystemTime::now().duration_since(start_time).unwrap().as_millis() < time {
        let result = mcts_rec(&mut root);
        root.update(result);
        count += 1;
    }

    println!("Number of simulations: {}", count);
    let action = root.best_child_final();
    if let Some(child) = root.children_mut().get_mut(&action).unwrap() {
        println!("Expected result: {}", child.weight());
    }

    action
}

pub struct MCTSActor {
    time_limit: u128,
}

impl MCTSActor {
    pub fn new(time_limit: u128) -> MCTSActor {
        MCTSActor { time_limit }
    }
}

impl Actor for MCTSActor {
    fn get_action(&self, game_state: &mut GameState) -> Action {
        mcts(game_state, self.time_limit)
    }
}
