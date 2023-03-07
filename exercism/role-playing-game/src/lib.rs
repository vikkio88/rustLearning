pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health <= 0 {
            return Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            });
        }

        None
    }

    pub fn get_mana(&self) -> Option<u32> {
        if self.level >= 10 {
            return self.mana;
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mut damage: u32 = 0;
        match self.get_mana() {
            Some(v) => {
                if mana_cost < v {
                    self.mana = Some(v - mana_cost);
                    damage = mana_cost * 2;
                }
            }
            None => {
                self.health = if self.health < mana_cost {
                    0
                } else {
                    self.health - mana_cost
                };
            }
        };

        damage
    }
}
