use std::time::SystemTime;

use crate::game::action::Action;
use crate::actor::Actor;
use crate::game::game_state::GameState;
use crate::game::board::Owned;
use crate::game::player::{Player, GameResult};

use crate::util::non_nan::NonNan;

pub struct Node {
    pub visits: usize,
    value: f32,
    children: Vec<Node>,
    children_left: isize,
    state: GameState,
    result: Option<GameResult>,
}

impl Node {
    pub fn new(state: GameState, result: Option<GameResult>) -> Node {
        Node {
            visits: 0,
            value: 0.,
            children: Vec::new(),
            children_left: -1,
            state,
            result,
        }
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Node> {
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

        let (index, _) = self.children().iter().enumerate().max_by_key(|(_, node)| {
            NonNan::new(node.search_weight(visits_ln)).unwrap()
        }).unwrap();

        index
    }

    pub fn best_action(&self) -> (Action, f32) {
        let node = self.children().iter().max_by_key(|node| {
            NonNan::new(node.weight()).unwrap()
        }).unwrap();

        (
            action_between(self, node).clone(),
            node.weight(),
        )
    }

    pub fn weight(&self) -> f32 {
        if self.visits == 0 { 0. } else { self.value / self.visits as f32 }
    }

    pub fn search_weight(&self, parent_visits_ln: f32) -> f32 {
        if self.visits == 0 {
            0.
        } else {
            self.value / self.visits as f32 + (2. * parent_visits_ln / self.visits as f32).sqrt()
        }
    }

    #[inline]
    pub fn fully_expanded(&self) -> bool {
        self.children_left == 0
    }

    pub fn expand(&mut self) -> usize {
        if self.children_left == -1 {
            self.children = initial_vec(self.state_mut());
            self.children_left = self.children.len() as isize;
        }

        self.children_left -= 1;
        self.children.len() - self.children_left as usize - 1
    }

    pub fn update(&mut self, result: GameResult) {
        self.visits += 1;
        self.value += result.score(self.state.current_player.next());
    }
}

fn action_between(node1: &Node, node2: &Node) -> Action {
    for sub_x in 0..3 {
        for sub_y in 0..3 {
            for x in 0..3 {
                for y in 0..3 {
                    if node1.state().board().get(sub_x, sub_y).get(x, y).result() !=
                        node2.state().board().get(sub_x, sub_y).get(x, y).result() {

                        return Action::new(sub_x, sub_y, x, y, node1.state().current_sub_x.is_none());
                    }
                }
            }
        }
    }

    panic!("No differences");
}

fn initial_vec(game_state: &GameState) -> Vec<Node> {
    game_state.possible_actions()
        .iter()
        .map(|action| {
            let mut new_game_state = game_state.clone();
            let result = action.apply(&mut new_game_state);
            Node::new(
                new_game_state,
                result,
            )
        }).collect()
}

pub fn mcts_rec(root: &mut Node) -> GameResult {
    if root.fully_expanded() {
        let index = root.best_child(root.visits);
        let best_child = root.children_mut().get_mut(index).unwrap();

        let result = match best_child.result {
            Some(game_result) => game_result,
            None => mcts_rec(best_child),
        };

        best_child.update(result);

        result
    } else {
        let index = root.expand();
        let new_child = root.children_mut().get_mut(index).unwrap();

        let result = match new_child.result {
            Some(game_result) => game_result,
            None => root.state_mut().play_randomly2(),
        };

        let new_child = root.children_mut().get_mut(index).unwrap();
        new_child.update(result);

        result
    }
}

pub fn mcts(game_state: &mut GameState, time: u128) -> Action {
    let mut root = Node::new(
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
    let (best_action, weight) = root.best_action();
    println!("Expected result: {}", weight);

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
