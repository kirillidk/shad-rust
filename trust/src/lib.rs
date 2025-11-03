#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentMove {
    Cooperate,
    Cheat,
}

pub struct Game {
    left_score: i32,
    right_score: i32,
    left_agent: Box<dyn Agent>,
    right_agent: Box<dyn Agent>,
    left_move: Option<AgentMove>,
    right_move: Option<AgentMove>,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            left_score: 0,
            right_score: 0,
            left_agent: left,
            right_agent: right,
            left_move: None,
            right_move: None,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left_move = self.left_agent.make_move(self.right_move);
        let right_move = self.right_agent.make_move(self.left_move);

        self.left_move = Some(left_move);
        self.right_move = Some(right_move);

        if left_move == AgentMove::Cooperate && right_move == AgentMove::Cooperate {
            self.left_score += 2;
            self.right_score += 2;

            return RoundOutcome::BothCooperated;
        }

        if left_move == AgentMove::Cheat && right_move == AgentMove::Cheat {
            return RoundOutcome::BothCheated;
        }

        if left_move == AgentMove::Cheat {
            self.left_score += 3;
            self.right_score -= 1;

            return RoundOutcome::LeftCheated;
        }

        self.left_score -= 1;
        self.right_score += 3;

        RoundOutcome::RightCheated
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Agent {
    fn make_move(&mut self, opponents_move: Option<AgentMove>) -> AgentMove;
}

////////////////////////////////////////////////////////////////////////////////

pub struct CheatingAgent {}

impl Default for CheatingAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl CheatingAgent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent for CheatingAgent {
    fn make_move(&mut self, _opponents_move: Option<AgentMove>) -> AgentMove {
        AgentMove::Cheat
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CooperatingAgent {}

impl Default for CooperatingAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl CooperatingAgent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent for CooperatingAgent {
    fn make_move(&mut self, _opponents_move: Option<AgentMove>) -> AgentMove {
        AgentMove::Cooperate
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct GrudgerAgent {
    was_cheated_on: bool,
}

impl Default for GrudgerAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl GrudgerAgent {
    pub fn new() -> Self {
        Self {
            was_cheated_on: false,
        }
    }
}

impl Agent for GrudgerAgent {
    fn make_move(&mut self, opponents_move: Option<AgentMove>) -> AgentMove {
        if self.was_cheated_on {
            return AgentMove::Cheat;
        }

        if opponents_move == Some(AgentMove::Cheat) {
            self.was_cheated_on = true;
            return AgentMove::Cheat;
        }

        AgentMove::Cooperate
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {}

impl Default for CopycatAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl CopycatAgent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent for CopycatAgent {
    fn make_move(&mut self, opponents_move: Option<AgentMove>) -> AgentMove {
        if opponents_move.is_none() {
            return AgentMove::Cooperate;
        }
        opponents_move.unwrap()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    turn_number: usize,
    was_cheated_on: bool,
}

impl Default for DetectiveAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DetectiveAgent {
    pub fn new() -> Self {
        Self {
            turn_number: 0,
            was_cheated_on: false,
        }
    }
}

impl Agent for DetectiveAgent {
    fn make_move(&mut self, opponents_move: Option<AgentMove>) -> AgentMove {
        if opponents_move == Some(AgentMove::Cheat) {
            self.was_cheated_on = true;
        }

        let default_moves: [AgentMove; 4] = [
            AgentMove::Cooperate,
            AgentMove::Cheat,
            AgentMove::Cooperate,
            AgentMove::Cooperate,
        ];

        if self.turn_number < default_moves.len() {
            let agent_move = default_moves[self.turn_number];
            self.turn_number += 1;

            return agent_move;
        }

        if self.was_cheated_on {
            opponents_move.unwrap()
        } else {
            AgentMove::Cheat
        }
    }
}
