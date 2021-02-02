//! Data structure to represent a single combat calculation.

use parthia::simple_calc::{CombatStats, SpeedDiff, Outcome,
                           possible_outcomes};
use parthia::fegame::FEGame;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub(crate) struct Battle {
    pub(crate) atk_hp: u32,
    pub(crate) atk: CombatStats,
    pub(crate) def_hp: u32,
    pub(crate) def: CombatStats,
    pub(crate) speed: SpeedDiff,
    pub(crate) game: FEGame
}

impl Battle {
    pub(crate) fn outcomes(&self) -> Vec<Outcome> {
        possible_outcomes(self.game, self.atk, self.atk_hp,
                          self.def, self.def_hp, self.speed)
    }
}
