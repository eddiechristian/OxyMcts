use core::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, Div};
use std::thread::{self, JoinHandle};

use ascii_tree::Tree::{Leaf, Node};
use ascii_tree::{write_tree, Tree};
use ego_tree::NodeId;
use num_traits::{ToPrimitive, Zero};

use crate::aliases::{LazyMctsNode, LazyMctsTree};
use crate::traits::{BackPropPolicy, GameTrait, LazyTreePolicy, Playout, self};
use crate::Evaluator;


lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
     };
    // static ref COUNT: usize = HASHMAP.len();
    // static ref NUMBER: u32 = times_two(21);
}

/// This is a special MCTS because it doesn't store the state in the node but instead stores the
/// historic to the node.
#[derive(Clone)]
pub struct LazyMcts<'a, State, TP, PP, BP, EV, AddInfo, Reward>
    where
        State: GameTrait,
        TP: LazyTreePolicy<State, EV, AddInfo, Reward>,
        PP: Playout<State>,
        BP: BackPropPolicy<Vec<State::Move>, State::Move, Reward, AddInfo, EV::EvalResult>,
        EV: Evaluator<State, Reward, AddInfo>,
        AddInfo: Clone + Default,
        Reward: Clone,
{
    root_state: &'a State,
    tree_policy: PhantomData<TP>,
    playout_policy: PhantomData<PP>,
    backprop_policy: PhantomData<BP>,
    evaluator: PhantomData<EV>,
    tree: LazyMctsTree<State, Reward, AddInfo>,
}

impl<'a, State, TP, PP, BP, EV, A, R> LazyMcts<'a, State, TP, PP, BP, EV, A, R>
    where
        State: GameTrait + std::fmt::Debug + std::marker::Send,
        TP: LazyTreePolicy<State, EV, A, R>,
        PP: Playout<State>,
        BP: BackPropPolicy<Vec<State::Move>, State::Move, R, A, EV::EvalResult>,
        EV: Evaluator<State, R, A>,
        A: Clone + Default,
        R: Clone + Div + ToPrimitive + Zero + Add + Display,
{
    pub fn new(root_state: &'a State) -> Self {
        Self::with_capacity(root_state, 0)
    }

    pub fn with_capacity(root_state: &'a State, capacity: usize) -> Self {
        let tree = LazyMctsTree::<State, R, A>::with_capacity(
            LazyMctsNode::<State, R, A> {
                sum_rewards: Zero::zero(),
                n_visits: 0,
                unvisited_moves: root_state.legals_moves(),
                hash: root_state.hash(),
                state: vec![],
                additional_info: Default::default(),
            },
            capacity,
        );
        Self {
            root_state,
            tree_policy: PhantomData,
            playout_policy: PhantomData,
            backprop_policy: PhantomData,
            evaluator: PhantomData,
            tree,
        }
    }

    /// Executes one selection, expansion?, simulation, backpropagation.
    pub fn execute(&mut self, evaluation_args: &EV::Args, playout_args: PP::Args) where <EV as traits::Evaluator<State, R, A>>::EvalResult: std::fmt::Debug, <PP as Playout<State>>::Args: Send {
        
        let mut tree_policy_states = Vec::new();
        for i in 0..30 {
            let (node_id, state) =
            TP::tree_policy(&mut self.tree, self.root_state.clone(), evaluation_args);
            println!("node_id: {:?} state: {:?}",node_id, state );
            tree_policy_states.push((node_id, state));
        } 
        let mut playout_states = Vec::<JoinHandle<(State, NodeId)>>::new();
        let mut join_handles = vec![];
        for state_tup in  tree_policy_states {
            // join_handles.push(thread::spawn(move || {
            //     return (PP::playout(state_tup.1.clone(), playout_args),state_tup.0);
            // }));
            join_handles.push(thread::spawn({
                let ss = state_tup.1.clone();
                move || return (PP::playout(ss, playout_args).clone(),state_tup.0)
            }));
        }
        for child in join_handles {
            // Wait for the thread to finish. Returns a result.
            let (final_state, node_id ) = child.join().unwrap();
            println!("final_state: {:?} ", final_state);
            let eval = EV::evaluate_leaf(final_state, &self.root_state.player_turn());
            println!(" eval: {:?} ", eval);
            BP::backprop(&mut self.tree, node_id, eval);
        }
        
       
    }

    /// Returns the best move from the root.
    pub fn best_move(&self, evaluator_args: &EV::Args) -> State::Move {
        let best_child = TP::best_child(
            &self.tree,
            &self.root_state.player_turn(),
            self.tree.root().id(),
            evaluator_args,
        );
        self.tree
            .get(best_child)
            .unwrap()
            .value()
            .state
            .last()
            .expect("The historic of the children of the root is empty, cannot happen")
            .clone()
    }

    pub fn write_tree(&self) -> String {
        let tree = self.dfs(self.tree.root().id());
        let mut output = String::new();
        write_tree(&mut output, &tree).unwrap();
        return output;
    }

    fn dfs(&self, node_id: NodeId) -> Tree {
        let node = self.tree.get(node_id).unwrap();
        if node.has_children() {
            let mut nodes = vec![];
            for c in node.children() {
                nodes.push(self.dfs(c.id()))
            }
            Node(
                format!("{};{}", node.value().n_visits, node.value().sum_rewards),
                nodes,
            )
        } else {
            Leaf(vec![format!(
                "{};{}",
                node.value().n_visits,
                node.value().sum_rewards
            )])
        }
    }

    pub fn tree(&self) -> &LazyMctsTree<State, R, A> {
        &self.tree
    }
}

impl<State, TP, PP, BP, EV, A, R> Debug for LazyMcts<'_, State, TP, PP, BP, EV, A, R>
    where
        State: GameTrait,
        TP: LazyTreePolicy<State, EV, A, R>,
        PP: Playout<State>,
        BP: BackPropPolicy<Vec<State::Move>, State::Move, R, A, EV::EvalResult>,
        EV: Evaluator<State, R, A>,
        EV::EvalResult: Debug,
        A: Clone + Default + Debug,
        R: Clone + Debug + Div + Add + Zero + ToPrimitive,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self.tree))
    }
}
