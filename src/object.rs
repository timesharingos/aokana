// basic object of LOR combat

use std::ops::Sub;
use rand::Rng;

//Basic Dice(speed, combat)
//Speed Dice
//Combat Dice(normal, counter)
//Normal Dice(attack, defense)
//Attack Dice(slash, pierce, blunt)
//Defense Dice(block, evade)
//Counter Dice

#[derive(Clone)]
pub enum DiceType {
    Speed,
    Combat(CombatDice),
}

#[derive(Clone)]
pub enum CombatDice {
    Normal(NormalDice),
    Counter(NormalDice),
}

#[derive(Clone)]
pub enum NormalDice {
    Attack(AttackDice),
    Defense(DefenseDice),
}

#[derive(Clone)]
pub enum AttackDice {
    Slash,
    Pierce,
    Blunt,
}

#[derive(Clone)]
pub enum DefenseDice {
    Block,
    Evade,
}

#[derive(Clone)]
pub struct Dice {
    min: i32,
    max: i32,
    cur: i32,
    dtype: DiceType,
}

impl Dice {
    pub fn new(dtype: DiceType, min: i32, max: i32) -> Self {
        Self {
            min,
            max,
            cur: -1,
            dtype,
        }
    }

    pub fn roll(&mut self) -> i32 {
        if self.min > self.max {
            self.cur = self.max;
            return self.cur;
        }
        self.cur = rand::thread_rng().gen_range(self.min..=self.max);
        self.cur
    }

    pub fn cur(&self) -> i32 {
        self.cur
    }

    pub fn dtype(&self) -> &DiceType {
        &self.dtype
    }
}

impl NormalDice {
    pub fn combat(&self, cur: i32) -> impl Fn(&Self, i32) -> (Self, i32) + '_ {
	assert!(cur != -1);
        return move |d2: &Self, cur2: i32| -> (Self, i32) {
            assert!(cur2 != -1);
            let other = d2.to_owned();
            let our = self.to_owned();
            match our {
                Self::Attack(dice) => {
		    match other {
		    	Self::Attack(dice2) => {
                            if cur > cur2 {
                            	(Self::Attack(dice), cur)
			    }
			    else if cur < cur2 {
                                (Self::Attack(dice2), (-1)*cur2)
			    }
			    else {
                                (Self::Attack(dice), 0)
			    }
                        },
                        Self::Defense(DefenseDice::Block) => {
			    if cur > cur2 {
                                (Self::Attack(dice), cur - cur2)
                            } else {
                                (Self::Defense(DefenseDice::Block), cur - cur2)
                            }
                        },
                        Self::Defense(DefenseDice::Evade) => {
			    if cur > cur2 {
                                (Self::Attack(dice), cur)
                            } else if cur < cur2 {
                                (Self::Defense(DefenseDice::Evade), (-1)*cur2)
                            } else {
                                (Self::Defense(DefenseDice::Evade), 0)
                            }
                        }
                    }
                },
                Self::Defense(DefenseDice::Block) => {
		    match other {
                        Self::Attack(dice2) => {
                            if cur >= cur2 {
                                (Self::Defense(DefenseDice::Block), cur - cur2)
                            } else {
                                (Self::Attack(dice2), cur - cur2)
                            }
                        },
                        Self::Defense(DefenseDice::Block) => {
                            let val;
                            if cur > cur2 {
                                val = cur;
                            } else if cur < cur2 {
                                val = (-1) * cur2;
                            } else {
                                val = 0;
                            }
                            (Self::Defense(DefenseDice::Block), val)
                        },
                        Self::Defense(DefenseDice::Evade) => {
                            if cur > cur2 {
                                (Self::Defense(DefenseDice::Block), cur)
                            } else if cur < cur2 {
                                (Self::Defense(DefenseDice::Evade), (-1) * cur2)
                            } else {
                                (Self::Defense(DefenseDice::Evade), 0)
                            }
                        },
                    }
                },
                Self::Defense(DefenseDice::Evade) => {
		    match other {
                        Self::Attack(dice2) => {
			    if cur > cur2 {
                                (Self::Defense(DefenseDice::Evade), cur)
                            } else if cur < cur2 {
                                (Self::Attack(dice2), (-1) * cur2)
                            } else {
                                (Self::Defense(DefenseDice::Evade), 0)
                            }
                        },
                        Self::Defense(DefenseDice::Block) => {
			    if cur > cur2 {
                                (Self::Defense(DefenseDice::Evade), cur)
                            } else if cur < cur2 {
                                (Self::Defense(DefenseDice::Block), (-1) * cur2)
                            } else {
                                (Self::Defense(DefenseDice::Block), 0)
                            }
                        },
                        Self::Defense(DefenseDice::Evade) => {
                            (Self::Defense(DefenseDice::Evade), 0)
                        },
                    }
                },
            }
        };
    }
}

//dice combat
impl Sub for Dice {
    type Output = Dice;
    
    fn sub(self, other: Self) -> Self::Output {
        assert!(self.cur != -1);
        assert!(other.cur != -1);
        
        match self.dtype {
            //speed comparasion
            DiceType::Speed => {
                match other.dtype {
                    DiceType::Speed => Self {min: 0, max: 0, cur: self.cur - other.cur, dtype: DiceType::Speed},
                    _ => unreachable!(),
                }
            },

            //combat
            DiceType::Combat(CombatDice::Normal(dice)) => {
                match other.dtype {
                    DiceType::Speed => unreachable!(),
                    DiceType::Combat(CombatDice::Normal(dice2)) => {
			let (side, val) = dice.combat(self.cur)(&dice2, other.cur);
                        Self {min: 0, max: 0, cur: val, dtype: DiceType::Combat(CombatDice::Normal(side))}
                    },
                    DiceType::Combat(CombatDice::Counter(dice2)) => {
			let (side, val) = dice.combat(self.cur)(&dice2, other.cur);
                        match val {
                            val if val >= 0 => Self {min: 0, max: 0, cur: val, dtype: DiceType::Combat(CombatDice::Normal(side))},
                            val => Self {min: 0, max: 0, cur: val, dtype: DiceType::Combat(CombatDice::Counter(side))},
                        }
                    },
                }
            },

            DiceType::Combat(CombatDice::Counter(dice)) => {
                match other.dtype {
                    DiceType::Speed | DiceType::Combat(CombatDice::Counter(_)) => unreachable!(),
                    DiceType::Combat(CombatDice::Normal(dice2)) => {
			let (side, val) = dice.combat(self.cur)(&dice2, other.cur);
                        match val {
                            val if val >= 0 => Self {min: 0, max: 0, cur: val, dtype: DiceType::Combat(CombatDice::Counter(side))},
                            val => Self {min: 0, max: 0, cur: val, dtype: DiceType::Combat(CombatDice::Normal(side))},
                        }
                    },
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_roll() {
        let mut dice = Dice::new(DiceType::Speed, 1, 4);

        dice.roll();
        assert!(dice.cur() >= 1);
        assert!(dice.cur() <= 4);

        dice.roll();
        assert!(dice.cur() >= 1);
        assert!(dice.cur() <= 4);

        let mut dice = Dice::new(DiceType::Speed, 3, 2);
        dice.roll();
        assert!(dice.cur() == 2);

        let mut dice = Dice::new(DiceType::Speed, 3, 3);
        dice.roll();
        assert!(dice.cur() == 3);
    }

    #[test]
    fn speed_duel() {
        let mut dice1 = Dice::new(DiceType::Speed, 2, 2);
        let mut dice2 = Dice::new(DiceType::Speed, 3, 4);

        dice1.roll();
        dice2.roll();
        let duel_result = dice1 - dice2;
        
        match duel_result.dtype() {
            DiceType::Speed => {},
            _ => unreachable!(),
        };

        assert!(duel_result.cur() < 0);
    }

    #[test]
    #[should_panic]
    fn illegal_duel_speed() {
        let mut speed = Dice::new(DiceType::Speed, 1, 4);
        let mut attack = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))), 2, 3);
        
        speed.roll();
        attack.roll();

        let _ = speed - attack;
    }

    #[test]
    #[should_panic]
    fn illegal_duel_counter() {
        let mut counter = Dice::new(DiceType::Combat(CombatDice::Counter(NormalDice::Attack(AttackDice::Slash))), 2, 3);
        let mut attack = Dice::new(DiceType::Combat(CombatDice::Counter(NormalDice::Attack(AttackDice::Slash))), 2, 3);
        
        counter.roll();
        attack.roll();

        let _ = counter - attack;
    }

    #[test]
    #[should_panic]
    fn illegal_duel_combat() {
        let mut combat = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))), 2, 3);
        let mut speed = Dice::new(DiceType::Speed, 1, 4);
        
        combat.roll();
        speed.roll();

        let _ = combat - speed;
    }

    #[test]
    fn duel_combat_attack_attack() {
        let mut combat1 = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))), 2, 3);
        let mut combat2 = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Pierce))), 4, 6);
        
        combat1.roll();
        combat2.roll();

        let duel_result = combat1 - combat2;
        match duel_result.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Pierce))) => {},
            _ => unreachable!(),
        };
        assert!(duel_result.cur() < 0);
        assert!(duel_result.cur() >= -6);
	assert!(duel_result.cur() <= -4);
    }

    #[test]
    fn duel_combat_defense() {
        
        let mut attack = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))), 2, 3);
        let mut block = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))), 4, 6);
    	let mut evade = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))), 15, 15);

        attack.roll();
        block.roll();
        evade.roll();

        let attack_block = attack.clone() - block.clone();
        let attack_evade = attack.clone() - evade.clone();

        match attack_block.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))) => {},
            _ => unreachable!(),
        };
        assert!(attack_block.cur() < 0);
        assert!(attack_block.cur() >= -4);
        assert!(attack_block.cur() <= -1);
        
        match attack_evade.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))) => {},
            _ => unreachable!(),
        };
        assert!(attack_evade.cur() < 0);
        assert!(attack_evade.cur() == -15);
    }

    #[test]
    fn duel_combat_counter() {
        
        let mut attack = Dice::new(DiceType::Combat(CombatDice::Counter(NormalDice::Attack(AttackDice::Slash))), 2, 3);
        let mut block = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))), 4, 6);
    	let mut evade = Dice::new(DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))), 15, 15);

        attack.roll();
        block.roll();
        evade.roll();

        let attack_block = attack.clone() - block.clone();
        let attack_evade = attack.clone() - evade.clone();

        match attack_block.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))) => {},
            _ => unreachable!(),
        };
        assert!(attack_block.cur() < 0);
        assert!(attack_block.cur() >= -4);
        assert!(attack_block.cur() <= -1);
        
        match attack_evade.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))) => {},
            _ => unreachable!(),
        };
        assert!(attack_evade.cur() < 0);
        assert!(attack_evade.cur() == -15);
    }
}
