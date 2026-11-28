#[derive(Debug, Clone)]
pub struct Pkm {
    pub advances: &'static str,
    pub egg_seed: &'static str,
    pub pid: &'static str,
    pub shiny: &'static str,
    pub nature: &'static str,
    pub ability: &'static str,
    pub hp: &'static str,
    pub atk: &'static str,
    pub def: &'static str,
    pub spa: &'static str,
    pub spd: &'static str,
    pub spe: &'static str,
    pub gender: &'static str,
    pub ec: &'static str,
}

impl Pkm {
    pub fn search(
        tid: u32,
        sid: u32,
        shiny_charm: bool,
        oval_charm: bool,
        compatibility_str: &str,
        gender_ratio_str: &str,
        masuda: bool,
        seed0: u64,
        seed1: u64,
        initial_advances: u64,
        max_advances: u64,
    ) -> Vec<Pkm> {
        println!("TID: {}", tid);
        println!("SID: {}", sid);
        println!("Shiny Charm: {}", shiny_charm);
        println!("Oval Charm: {}", oval_charm);
        println!("Compatibility: {}", compatibility_str);
        println!("Gender Ratio: {}", gender_ratio_str);
        println!("Masuda: {}", masuda);
        println!("Seed0: {}", seed0);
        println!("Seed1: {}", seed1);
        println!("Initial Advances: {}", initial_advances);
        println!("Max Advances: {}", max_advances);

        vec![
            Pkm {
                advances: "928",
                egg_seed: "1D537A02",
                pid: "2EDCCAD2",
                shiny: "Yes",
                nature: "Lonely",
                ability: "95",
                hp: "2",
                atk: "-2",
                def: "-1",
                spa: "11",
                spd: "10",
                spe: "-2",
                gender: "-",
                ec: "AEF10229",
            },
            Pkm {
                advances: "1039",
                egg_seed: "B0E70D41",
                pid: "72AC96A9",
                shiny: "Yes",
                nature: "Modest",
                ability: "68",
                hp: "29",
                atk: "-2",
                def: "9",
                spa: "22",
                spd: "-2",
                spe: "-1",
                gender: "-",
                ec: "CA975BB",
            },
        ]
    }
}
