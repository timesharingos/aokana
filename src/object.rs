// basic object of LOR combat

use rand::Rng;
use std::ops::Sub;

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

//Page(key, combat)
//Key Page
//Combat Page(melee, ranged, mass, instant)
//Melee Page(normal melee, priority melee)
//Ranged Page
//Mass Page(summation, individual)
pub trait Page {
    fn ptype(&self) -> PageType;
}

#[derive(Clone)]
pub enum PageType {
    Key,
    Combat(CombatPageType),
}

#[derive(Clone)]
pub enum CombatPageType {
    Melee(MeleePageType),
    Ranged,
    Mass(MassPageType),
    Instant,
}

#[derive(Clone)]
pub enum MeleePageType {
    NormalMelee,
    PriorityMelee,
}

#[derive(Clone)]
pub enum MassPageType {
    Summation,
    Individual,
}

#[derive(Clone)]
pub enum PageRarity {
    Paperback,
    Hardcover,
    Limited,
    Art,
}

#[derive(Clone)]
pub enum Resistance {
    Fatal,
    Weak,
    Normal,
    Endured,
    Ineffective,
    Immune,
}

impl Resistance {
    pub fn number(&self) -> f32 {
        match self {
            Self::Fatal => 2.0,
            Self::Weak => 1.5,
            Self::Normal => 1.0,
            Self::Endured => 0.5,
            Self::Ineffective => 0.25,
            Self::Immune => 0.0,
        }
    }
}

#[derive(Clone)]
pub struct KeyPageResistances {
    pub hslash: Resistance,
    pub hpierce: Resistance,
    pub hblunt: Resistance,
    pub sslash: Resistance,
    pub spierce: Resistance,
    pub sblunt: Resistance,
}

impl KeyPageResistances {
    //health, stagger
    pub fn get(&self, dtype: &AttackDice) -> (&Resistance, &Resistance) {
        match dtype {
            AttackDice::Slash => (&self.hslash, &self.sslash),
            AttackDice::Pierce => (&self.hpierce, &self.spierce),
            AttackDice::Blunt => (&self.hblunt, &self.sblunt),
        }
    }
}

#[derive(Clone)]
pub struct KeyPage {
    pub name: String,
    pub rarity: PageRarity,
    pub speed: Vec<Dice>,
    pub maxhealth: i32,
    pub maxstagger: i32,
    pub maxlights: i32,
    pub resistances: KeyPageResistances,
    pub curlights: i32,
    pub curhealth: i32,
    pub curstagger: i32,
}

#[derive(Clone)]
pub struct KeyPageBuilder {
    name: String,
    rarity: PageRarity,
    speed: Vec<Dice>,
    maxhealth: i32,
    maxstagger: i32,
    lights: i32,
    resistances: KeyPageResistances,
}

impl KeyPageBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            rarity: PageRarity::Paperback,
            speed: Vec::new(),
            maxhealth: 1,
            maxstagger: 1,
            lights: 0,
            resistances: KeyPageResistances {
                hslash: Resistance::Normal,
                hpierce: Resistance::Normal,
                hblunt: Resistance::Normal,
                sslash: Resistance::Normal,
                spierce: Resistance::Normal,
                sblunt: Resistance::Normal,
            },
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn rarity(mut self, rarity: PageRarity) -> Self {
        self.rarity = rarity;
        self
    }

    pub fn speed(mut self, speed: Dice) -> Self {
        match speed.dtype() {
            DiceType::Speed => {}
            _ => unreachable!(),
        };
        self.speed.push(speed);
        self
    }

    pub fn health(mut self, health: i32) -> Self {
        self.maxhealth = health;
        self
    }

    pub fn stagger(mut self, stagger: i32) -> Self {
        self.maxstagger = stagger;
        self
    }

    pub fn lights(mut self, lights: i32) -> Self {
        self.lights = lights;
        self
    }

    pub fn resistances(mut self, resistances: KeyPageResistances) -> Self {
        self.resistances = resistances;
        self
    }

    pub fn hslash_resistance(mut self, resistance: Resistance) -> Self {
        self.resistances.hslash = resistance;
        self
    }

    pub fn hpierce_resistance(mut self, resistance: Resistance) -> Self {
        self.resistances.hpierce = resistance;
        self
    }

    pub fn hblunt_resistance(mut self, resistance: Resistance) -> Self {
        self.resistances.hblunt = resistance;
        self
    }

    pub fn sslash_resistance(mut self, resistance: Resistance) -> Self {
        self.resistances.sslash = resistance;
        self
    }

    pub fn spierce_resistance(mut self, resistance: Resistance) -> Self {
        self.resistances.spierce = resistance;
        self
    }

    pub fn sblunt_resistance(mut self, resistance: Resistance) -> Self {
        self.resistances.sblunt = resistance;
        self
    }

    pub fn build(mut self) -> KeyPage {
        //for preserved defense/counter dice
        self.speed.push(Dice::new(DiceType::Speed, 0, 0));
        KeyPage {
            name: self.name,
            rarity: self.rarity,
            speed: self.speed,
            maxhealth: self.maxhealth,
            maxstagger: self.maxstagger,
            maxlights: self.lights,
            resistances: self.resistances,
            curlights: self.lights,
            curhealth: self.maxhealth,
            curstagger: self.maxstagger,
        }
    }
}

impl Default for KeyPageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyPage {
    pub fn eval(&mut self, result: &Dice) {
        match result.dtype() {
            DiceType::Speed => unreachable!(),
            DiceType::Combat(CombatDice::Normal(dice))
            | DiceType::Combat(CombatDice::Counter(dice)) => {
                match dice {
                    NormalDice::Attack(atype) => {
                        if result.cur() < 0 {
                            let (hresist, sresist) = self.resistances.get(atype);
                            self.curhealth += (hresist.number() * result.cur() as f32) as i32;
                            self.curstagger += (sresist.number() * result.cur() as f32) as i32;
                        }
                    }
                    NormalDice::Defense(DefenseDice::Block) => {
                        if result.cur() < 0 {
                            self.curstagger += result.cur();
                            if self.curstagger > self.maxstagger {
                                self.curstagger = self.maxstagger;
                            }
                        }
                    }
                    NormalDice::Defense(DefenseDice::Evade) => {
                        if result.cur() > 0 {
                            self.curstagger += result.cur();
                            if self.curstagger > self.maxstagger {
                                self.curstagger = self.maxstagger;
                            }
                        }
                    }
                };
            }
        };
    }

    pub fn health(&self) -> i32 {
        self.curhealth
    }

    pub fn stagger(&self) -> i32 {
        self.curstagger
    }

    pub fn gets(&self) -> Option<&Vec<Dice>> {
        if self.curstagger <= 0 || self.curhealth <= 0 {
            return None;
        }
        Some(&self.speed)
    }

    pub fn get(&self, index: usize) -> Option<&Dice> {
        let speed = self.gets();
        match speed {
            None => None,
            Some(speed) => speed.get(index),
        }
    }
}

impl Page for KeyPage {
    fn ptype(&self) -> PageType {
        PageType::Key
    }
}

impl Default for KeyPage {
    fn default() -> Self {
        KeyPageBuilder::default().build()
    }
}

#[derive(Clone)]
pub struct CombatPage {
    pub name: String,
    pub rarity: PageRarity,
    pub dices: Vec<Dice>,
    pub ptype: PageType,
}

impl Default for CombatPage {
    fn default() -> Self {
        CombatPageBuilder::default().build()
    }
}

impl CombatPage {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rarity(&self) -> &PageRarity {
        &self.rarity
    }

    pub fn gets(&self) -> &Vec<Dice> {
        &self.dices
    }

    pub fn get(&self, index: usize) -> Option<&Dice> {
        self.dices.get(index)
    }

    //(result, self, other)
    pub fn eval(self, other: Self) -> (Vec<Dice>, Vec<Dice>, Vec<Dice>) {
        let self_dices = self.dices;
        let other_dices = other.dices;
        let mut results = Vec::new();
        let mut self_recycle = Vec::new();
        let mut other_recycle = Vec::new();

        let mut self_dices_iter = self_dices.into_iter();
        let mut other_dices_iter = other_dices.into_iter();
        loop {
            let self_next = self_dices_iter.next();
            let other_next = other_dices_iter.next();
            match (self_next, other_next) {
                (None, None) => break,
                (None, Some(mut other_next)) => {
                    other_next.roll();
                    other_recycle.push(other_next);
                    break;
                }
                (Some(mut self_next), None) => {
                    self_next.roll();
                    self_recycle.push(self_next);
                    break;
                }
                (Some(mut self_dice), Some(mut other_dice)) => {
                    self_dice.roll();
                    other_dice.roll();
                    results.push(self_dice - other_dice);
                }
            };
        }

        self_dices_iter.for_each(|mut x| {
            x.roll();
            self_recycle.push(x);
        });
        other_dices_iter.for_each(|mut x| {
            x.roll();
            other_recycle.push(x);
        });

        (results, self_recycle, other_recycle)
    }
}

#[derive(Clone)]
pub struct CombatPageBuilder {
    name: String,
    rarity: PageRarity,
    dices: Vec<Dice>,
    ptype: PageType,
}

impl CombatPageBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            rarity: PageRarity::Paperback,
            dices: Vec::new(),
            ptype: PageType::Combat(CombatPageType::Melee(MeleePageType::NormalMelee)),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn rarity(mut self, rarity: PageRarity) -> Self {
        self.rarity = rarity;
        self
    }

    pub fn dice(mut self, dice: Dice) -> Self {
        self.dices.push(dice);
        self
    }

    pub fn ptype(mut self, ptype: PageType) -> Self {
        match ptype {
            PageType::Key => unreachable!(),
            _ => self.ptype = ptype,
        };
        self
    }

    pub fn build(self) -> CombatPage {
        CombatPage {
            name: self.name,
            rarity: self.rarity,
            dices: self.dices,
            ptype: self.ptype,
        }
    }
}

impl Default for CombatPageBuilder {
    fn default() -> Self {
        Self::new()
    }
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
                Self::Attack(dice) => match other {
                    Self::Attack(dice2) => {
                        if cur > cur2 {
                            (Self::Attack(dice), cur)
                        } else if cur < cur2 {
                            (Self::Attack(dice2), (-1) * cur2)
                        } else {
                            (Self::Attack(dice), 0)
                        }
                    }
                    Self::Defense(DefenseDice::Block) => {
                        if cur > cur2 {
                            (Self::Attack(dice), cur - cur2)
                        } else {
                            (Self::Defense(DefenseDice::Block), cur - cur2)
                        }
                    }
                    Self::Defense(DefenseDice::Evade) => {
                        if cur > cur2 {
                            (Self::Attack(dice), cur)
                        } else if cur < cur2 {
                            (Self::Defense(DefenseDice::Evade), (-1) * cur2)
                        } else {
                            (Self::Defense(DefenseDice::Evade), 0)
                        }
                    }
                },
                Self::Defense(DefenseDice::Block) => match other {
                    Self::Attack(dice2) => {
                        if cur >= cur2 {
                            (Self::Defense(DefenseDice::Block), cur - cur2)
                        } else {
                            (Self::Attack(dice2), cur - cur2)
                        }
                    }
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
                    }
                    Self::Defense(DefenseDice::Evade) => {
                        if cur > cur2 {
                            (Self::Defense(DefenseDice::Block), cur)
                        } else if cur < cur2 {
                            (Self::Defense(DefenseDice::Evade), (-1) * cur2)
                        } else {
                            (Self::Defense(DefenseDice::Evade), 0)
                        }
                    }
                },
                Self::Defense(DefenseDice::Evade) => match other {
                    Self::Attack(dice2) => {
                        if cur > cur2 {
                            (Self::Defense(DefenseDice::Evade), cur)
                        } else if cur < cur2 {
                            (Self::Attack(dice2), (-1) * cur2)
                        } else {
                            (Self::Defense(DefenseDice::Evade), 0)
                        }
                    }
                    Self::Defense(DefenseDice::Block) => {
                        if cur > cur2 {
                            (Self::Defense(DefenseDice::Evade), cur)
                        } else if cur < cur2 {
                            (Self::Defense(DefenseDice::Block), (-1) * cur2)
                        } else {
                            (Self::Defense(DefenseDice::Block), 0)
                        }
                    }
                    Self::Defense(DefenseDice::Evade) => (Self::Defense(DefenseDice::Evade), 0),
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
            DiceType::Speed => match other.dtype {
                DiceType::Speed => Self {
                    min: 0,
                    max: 0,
                    cur: self.cur - other.cur,
                    dtype: DiceType::Speed,
                },
                _ => unreachable!(),
            },

            //combat
            DiceType::Combat(CombatDice::Normal(dice)) => match other.dtype {
                DiceType::Speed => unreachable!(),
                DiceType::Combat(CombatDice::Normal(dice2)) => {
                    let (side, val) = dice.combat(self.cur)(&dice2, other.cur);
                    Self {
                        min: 0,
                        max: 0,
                        cur: val,
                        dtype: DiceType::Combat(CombatDice::Normal(side)),
                    }
                }
                DiceType::Combat(CombatDice::Counter(dice2)) => {
                    let (side, val) = dice.combat(self.cur)(&dice2, other.cur);
                    match val {
                        val if val >= 0 => Self {
                            min: 0,
                            max: 0,
                            cur: val,
                            dtype: DiceType::Combat(CombatDice::Normal(side)),
                        },
                        val => Self {
                            min: 0,
                            max: 0,
                            cur: val,
                            dtype: DiceType::Combat(CombatDice::Counter(side)),
                        },
                    }
                }
            },

            DiceType::Combat(CombatDice::Counter(dice)) => match other.dtype {
                DiceType::Speed | DiceType::Combat(CombatDice::Counter(_)) => unreachable!(),
                DiceType::Combat(CombatDice::Normal(dice2)) => {
                    let (side, val) = dice.combat(self.cur)(&dice2, other.cur);
                    match val {
                        val if val >= 0 => Self {
                            min: 0,
                            max: 0,
                            cur: val,
                            dtype: DiceType::Combat(CombatDice::Counter(side)),
                        },
                        val => Self {
                            min: 0,
                            max: 0,
                            cur: val,
                            dtype: DiceType::Combat(CombatDice::Normal(side)),
                        },
                    }
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
            DiceType::Speed => {}
            _ => unreachable!(),
        };

        assert!(duel_result.cur() < 0);
    }

    #[test]
    #[should_panic]
    fn illegal_duel_speed() {
        let mut speed = Dice::new(DiceType::Speed, 1, 4);
        let mut attack = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))),
            2,
            3,
        );

        speed.roll();
        attack.roll();

        let _ = speed - attack;
    }

    #[test]
    #[should_panic]
    fn illegal_duel_counter() {
        let mut counter = Dice::new(
            DiceType::Combat(CombatDice::Counter(NormalDice::Attack(AttackDice::Slash))),
            2,
            3,
        );
        let mut attack = Dice::new(
            DiceType::Combat(CombatDice::Counter(NormalDice::Attack(AttackDice::Slash))),
            2,
            3,
        );

        counter.roll();
        attack.roll();

        let _ = counter - attack;
    }

    #[test]
    #[should_panic]
    fn illegal_duel_combat() {
        let mut combat = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))),
            2,
            3,
        );
        let mut speed = Dice::new(DiceType::Speed, 1, 4);

        combat.roll();
        speed.roll();

        let _ = combat - speed;
    }

    #[test]
    fn duel_combat_attack_attack() {
        let mut combat1 = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))),
            2,
            3,
        );
        let mut combat2 = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Pierce))),
            4,
            6,
        );

        combat1.roll();
        combat2.roll();

        let duel_result = combat1 - combat2;
        match duel_result.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Pierce))) => {}
            _ => unreachable!(),
        };
        assert!(duel_result.cur() < 0);
        assert!(duel_result.cur() >= -6);
        assert!(duel_result.cur() <= -4);
    }

    #[test]
    fn duel_combat_defense() {
        let mut attack = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))),
            2,
            3,
        );
        let mut block = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))),
            4,
            6,
        );
        let mut evade = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))),
            15,
            15,
        );

        attack.roll();
        block.roll();
        evade.roll();

        let attack_block = attack.clone() - block.clone();
        let attack_evade = attack.clone() - evade.clone();

        match attack_block.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))) => {}
            _ => unreachable!(),
        };
        assert!(attack_block.cur() < 0);
        assert!(attack_block.cur() >= -4);
        assert!(attack_block.cur() <= -1);

        match attack_evade.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))) => {}
            _ => unreachable!(),
        };
        assert!(attack_evade.cur() < 0);
        assert!(attack_evade.cur() == -15);
    }

    #[test]
    fn duel_combat_counter() {
        let mut attack = Dice::new(
            DiceType::Combat(CombatDice::Counter(NormalDice::Attack(AttackDice::Slash))),
            2,
            3,
        );
        let mut block = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))),
            4,
            6,
        );
        let mut evade = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))),
            15,
            15,
        );

        attack.roll();
        block.roll();
        evade.roll();

        let attack_block = attack.clone() - block.clone();
        let attack_evade = attack.clone() - evade.clone();

        match attack_block.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))) => {}
            _ => unreachable!(),
        };
        assert!(attack_block.cur() < 0);
        assert!(attack_block.cur() >= -4);
        assert!(attack_block.cur() <= -1);

        match attack_evade.dtype() {
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))) => {}
            _ => unreachable!(),
        };
        assert!(attack_evade.cur() < 0);
        assert!(attack_evade.cur() == -15);
    }

    #[test]
    fn combat_duel() {
        let block = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))),
            4,
            6,
        );
        let evade = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))),
            15,
            15,
        );
        let counter = Dice::new(
            DiceType::Combat(CombatDice::Counter(NormalDice::Attack(AttackDice::Slash))),
            2,
            3,
        );
        let block2 = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))),
            4,
            6,
        );
        let evade2 = Dice::new(
            DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Evade))),
            15,
            15,
        );
        let combat_self = CombatPageBuilder::new()
            .name("Test")
            .rarity(PageRarity::Paperback)
            .dice(block)
            .dice(evade)
            .dice(counter)
            .build();
        let combat_other = CombatPageBuilder::new()
            .name("Test2")
            .rarity(PageRarity::Paperback)
            .dice(block2)
            .dice(evade2)
            .build();
        let (result, selfre, otherre) = combat_self.eval(combat_other);
        assert_eq!(result.len(), 2);
        assert_eq!(selfre.len(), 1);
        assert_eq!(otherre.len(), 0);
    }

    #[test]
    fn key_eval() {
        let combat_page = CombatPageBuilder::new()
            .name("Test")
            .rarity(PageRarity::Paperback)
            .dice(Dice::new(
                DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))),
                2,
                3,
            ))
            .dice(Dice::new(
                DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))),
                4,
                6,
            ))
            .dice(Dice::new(
                DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))),
                15,
                15,
            ))
            .build();
        let mut key_page = KeyPageBuilder::new()
            .name("Key")
            .rarity(PageRarity::Paperback)
            .speed(Dice::new(DiceType::Speed, 1, 4))
            .speed(Dice::new(DiceType::Speed, 2, 3))
            .health(100)
            .stagger(40)
            .lights(4)
            .hslash_resistance(Resistance::Fatal)
            .sslash_resistance(Resistance::Fatal)
            .build();
        let empty_combat_page = CombatPageBuilder::new()
            .name("Empty")
            .rarity(PageRarity::Paperback)
            .dice(Dice::new(
                DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))),
                0,
                0,
            ))
            .dice(Dice::new(
                DiceType::Combat(CombatDice::Normal(NormalDice::Defense(DefenseDice::Block))),
                0,
                0,
            ))
            .dice(Dice::new(
                DiceType::Combat(CombatDice::Normal(NormalDice::Attack(AttackDice::Slash))),
                0,
                0,
            ))
            .build();
        let (results, _, _) = empty_combat_page.eval(combat_page);
        for result in results {
            key_page.eval(&result);
        }

        assert!(key_page.stagger() <= 2);
        assert!(key_page.stagger() >= -2);
        assert!(key_page.health() == 64 || key_page.health() == 66);
    }
}
