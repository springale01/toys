#[derive(Debug, Clone)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}

impl std::fmt::Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            self.blocks
                .iter()
                .map(|block| block.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        })
    }
}
impl Blocks {
    pub fn new() -> Self {
        Self {
            blocks: Vec::with_capacity(12),
        }
    }
    fn affect_other_blocks(&mut self) {
        let len = self.blocks.len();
        //stores idx of those whose values are less than 0
        let mut naughty_list: Vec<usize> = Vec::new();
        for idx in 0..len {
            let affect = self.blocks[idx].get_affect();

            if idx > 0 {
                if let Some(brock) = self.blocks.get_mut(idx - 1) {
                    brock.change_value(affect.expect("This shouldnt be here"));

                    if brock.get_value() < Some(0) {
                        naughty_list.push(idx - 1);
                    }
                }
            }

            if idx + 1 < len {
                if let Some(brock) = self.blocks.get_mut(idx + 1) {
                    brock.change_value(affect.expect("This REALLY shouldnt happen"));

                    if brock.get_value() < Some(0) {
                        naughty_list.push(idx + 1);
                    }
                }
            }
        }

        naughty_list
            .iter()
            .for_each(|idx| self.blocks[*idx] = Block::None);

        // CLEANSE THE NAUGHTY ONES
        self.blocks = self
            .blocks
            .clone()
            .into_iter()
            .filter(|brock| *brock != Block::None)
            .collect()
    }

    //USE THIS ONE, the other one screams a lot and I wonder why
    pub fn affect_other_blocks_v2(&mut self) {
        let len = self.blocks.len();
        let mut deltas = vec![0isize; len];

        for (idx, block) in self.blocks.iter().enumerate() {
            let affect = match block.get_affect() {
                Some(a) => a,
                None => continue,
            };

            if idx > 0 {
                deltas[idx - 1] += affect;
            }
            if idx + 1 < len {
                deltas[idx + 1] += affect;
            }
        }

        for (block, delta) in self.blocks.iter_mut().zip(deltas) {
            if block.change_value(delta).is_none() {
                *block = Block::None;
            }
        }

        self.blocks.retain(|broch| *broch != Block::None);
    }
}

#[derive(Debug, Clone)]
pub enum Block {
    Beneficial { value: usize, affect: isize },
    Harmful { value: usize, affect: isize },
    Normie { value: usize },
    None,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Block::Beneficial { value, affect } => {
                format!("BeneficialBlock: (value:{}, affect:{})", value, affect)
            }
            Block::Harmful { value, affect } => {
                format!("HarmfulBlock: (value:{}, affect:{})", value, affect)
            }
            Block::Normie { value } => format!("NormieDetected: (value:{}, effect:{})", value, 0),
            _ => format!("None"),
        };
        write!(f, "{}", content)
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Block {
    fn change_value(&mut self, change: isize) -> Option<()> {
        match self {
            Self::Beneficial { value, affect } => {
                // change can be negitive
                if *value as isize + change > 0 {
                    *value += change as usize
                } else {
                    return None;
                }
            }
            Self::Harmful { value, affect } => {
                // change can be negitive
                if *value as isize + change > 0 {
                    *value += change as usize
                } else {
                    return None;
                }
            }
            Self::Normie { value } => {
                // change can be negitive
                if *value as isize + change > 0 {
                    *value += change as usize
                } else {
                    return None;
                }
            }
            _ => {
                return None;
            }
        }

        Some(())
    }

    fn get_affect(&self) -> Option<isize> {
        Some(match self {
            Self::Beneficial { value, affect } => *affect,
            Self::Harmful { value, affect } => *affect,
            Self::Normie { value } => 0 as isize,
            _ => {
                return None;
            }
        })
    }
    fn get_value(&self) -> Option<usize> {
        Some(match self {
            Self::Beneficial { value, affect } => *value,
            Self::Harmful { value, affect } => *value,
            Self::Normie { value } => 0 as usize,
            _ => {
                return None;
            }
        })
    }
}
