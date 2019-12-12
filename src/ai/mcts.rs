use std::time::SystemTime;

use crate::game::action::Action;
use crate::actor::Actor;
use crate::game::game_state::GameState;
use crate::game::player::{Player, GameResult};

use crate::util::non_nan::NonNan;

pub struct Node {
    pub visits: usize,
    value: f32,
    player: Player,
    children: Vec<(Action, Option<Node>)>,
    children_left: isize,
    state: GameState,
    result: Option<GameResult>,
}

impl Node {
    pub fn new(player: Player, state: GameState, result: Option<GameResult>) -> Node {
        Node {
            visits: 0,
            value: 0.,
            player,
            children: Vec::new(),
            children_left: -1,
            state,
            result,
        }
    }

    pub fn children(&self) -> &Vec<(Action, Option<Node>)> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<(Action, Option<Node>)> {
        &mut self.children
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }

    pub fn best_child(&self, visits: usize) -> usize {
        let visits_ln = (visits as f32).ln();

        let (index, _) = self.children().iter().enumerate().max_by_key(|(index, (action, node))| {
            let weight = match node {
                None => 0.,
                Some(node) => node.search_weight(visits_ln),
            };

            NonNan::new(weight).unwrap()
        }).unwrap();

        index
    }

    pub fn best_action(&self) -> Action {
        let (_, (action, _)) = self.children().iter().enumerate().max_by_key(|(index, (action, node))| {
            let weight = match node {
                None => 0.,
                Some(node) => node.weight(),
            };

            NonNan::new(weight).unwrap()
        }).unwrap();

        action.clone()
    }

    pub fn weight(&self) -> f32 {
        if self.visits == 0 { 0. } else { self.value / self.visits as f32 }
    }

    pub fn search_weight(&self, parent_visits_ln: f32) -> f32 {
        self.weight() + (2. * parent_visits_ln / self.visits as f32).sqrt()
    }

    pub fn fully_expanded(&self) -> bool {
        if self.children_left == 0 {
            true
        } else {
            false
        }
    }

    pub fn expand(&mut self) -> usize {
        if self.children_left == -1 {
            self.children = initial_vec(self.state_mut());
            self.children_left = self.children.len() as isize;
        }

        let index = self.children.len() - self.children_left as usize;
        let (action, node) = self.children.get(index).unwrap();
        assert!(node.is_none());

        let mut new_game_state = self.state().clone();
        let result = action.apply(&mut new_game_state);
        let new_node = Node::new(
            self.player.next(),
            new_game_state,
            result,
        );

        let action = action.clone();
        self.children_mut().insert(index, (action, Some(new_node)));
        self.children_left -= 1;

        index
    }

    pub fn update(&mut self, result: GameResult) {
        self.visits += 1;
        self.value += result.score(self.player);
    }
}

fn initial_vec(game_state: &GameState) -> Vec<(Action, Option<Node>)> {
    game_state.possible_actions()
        .iter()
        .map(|action| (action.clone(), None))
        .collect()
}

pub fn mcts_rec(root: &mut Node) -> GameResult {
    if root.fully_expanded() {
        let index = root.best_child(root.visits);
        let (_, best_child) = root.children_mut().get_mut(index).unwrap();

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
        let index = root.expand();
        let (_, new_child) = root.children_mut().get_mut(index).unwrap();

        let result = match new_child {
            None => panic!("Unexpanded child!"),
            Some(new_child) => {
                match new_child.result {
                    Some(game_result) => game_result,
                    None => root.state_mut().play_randomly(),
                }
            }
        };

        let (_, new_child) = root.children_mut().get_mut(index).unwrap();
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
    let best_action = root.best_action();
    for (action, child) in root.children().iter() {
        if action.clone() == best_action {
            if let Some(child) = child {
                println!("Expected result: {}", child.weight());
            }
        }
    }

    best_action
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
