use eggfinder_core::Pkm;
use iced::widget::{
    Column, Space, button, checkbox, column, combo_box, row, table, text, text_input,
};
use iced::{Center, Left, Length, Size};

const COMPATIBILITY_OPTIONS: &[&str] = &[
    "The two don't seem to like each other",
    "The two seem to get along",
    "The two seem to get along very well",
];

const GENDER_RATIO_OPTIONS: &[&str] = &[
    "Genderless",
    "100% F",
    "25% M / 75% F",
    "50% M / 50% F",
    "75% M / 25% F",
    "88% M / 12% F",
    "Nidoran / VI",
    "100% M",
];

/// Keeps only the ASCII digits of `value`, capped at 5 digits, so the result always fits the 0-99999 range.
fn sanitize_id(value: String) -> String {
    let mut digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.truncate(5);
    digits
}

/// Keeps only the hex digits of `value`, capped at 16 digits, so the result always fits the 0-FFFFFFFFFFFFFFFF range.
fn sanitize_seed(value: String) -> String {
    let mut digits: String = value.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    digits.truncate(16);
    digits
}

/// Keeps only the ASCII digits of `value`, capped at 19 digits, so the result always fits the 0-9999999999999999999 range.
fn sanitize_advance(value: String) -> String {
    let mut digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.truncate(19);
    digits
}

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(iced::Theme::Nord)
        .window_size(Size::new(1280.0, 800.0))
        .run()
}

struct App {
    tid: String,
    sid: String,
    shiny_charm: bool,
    oval_charm: bool,
    compatibility: &'static str,
    gender_ratio: &'static str,
    masuda: bool,
    seed0: String,
    seed1: String,
    initial_advances: String,
    max_advances: String,
    compatibility_state: combo_box::State<&'static str>,
    gender_ratio_state: combo_box::State<&'static str>,
    results: Vec<Pkm>,
}

impl Default for App {
    fn default() -> Self {
        let compatibility = "The two seem to get along very well";
        let gender_ratio = "Genderless";

        Self {
            tid: "12345".to_string(),
            sid: "54321".to_string(),
            shiny_charm: false,
            oval_charm: true,
            compatibility,
            gender_ratio,
            masuda: true,
            seed0: "1234567887654321".to_string(),
            seed1: "8765432112345678".to_string(),
            initial_advances: "0".to_string(),
            max_advances: "10000".to_string(),
            compatibility_state: combo_box::State::new(COMPATIBILITY_OPTIONS.to_vec()),
            gender_ratio_state: combo_box::State::new(GENDER_RATIO_OPTIONS.to_vec()),
            results: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TidChanged(String),
    SidChanged(String),
    ShinyCharmToggled(bool),
    OvalCharmToggled(bool),
    CompatibilityChanged(&'static str),
    GenderRatioChanged(&'static str),
    MasudaToggled(bool),
    Seed0Changed(String),
    Seed1Changed(String),
    InitialAdvancesChanged(String),
    MaxAdvancesChanged(String),
    Search,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::TidChanged(value) => self.tid = sanitize_id(value),
            Message::SidChanged(value) => self.sid = sanitize_id(value),
            Message::ShinyCharmToggled(value) => self.shiny_charm = value,
            Message::OvalCharmToggled(value) => self.oval_charm = value,
            Message::CompatibilityChanged(value) => self.compatibility = value,
            Message::GenderRatioChanged(value) => self.gender_ratio = value,
            Message::MasudaToggled(value) => self.masuda = value,
            Message::Seed0Changed(value) => self.seed0 = sanitize_seed(value),
            Message::Seed1Changed(value) => self.seed1 = sanitize_seed(value),
            Message::InitialAdvancesChanged(value) => {
                self.initial_advances = sanitize_advance(value)
            }
            Message::MaxAdvancesChanged(value) => self.max_advances = sanitize_advance(value),
            Message::Search => {
                let tid = self.tid.parse::<u32>().unwrap_or(0);
                let sid = self.sid.parse::<u32>().unwrap_or(0);
                let seed0 = u64::from_str_radix(&self.seed0, 16).unwrap_or(0);
                let seed1 = u64::from_str_radix(&self.seed1, 16).unwrap_or(0);
                let initial_advances = self.initial_advances.parse::<u64>().unwrap_or(0);
                let max_advances = self.max_advances.parse::<u64>().unwrap_or(0);
                self.results = Pkm::search(
                    tid,
                    sid,
                    self.shiny_charm,
                    self.oval_charm,
                    self.compatibility,
                    self.gender_ratio,
                    self.masuda,
                    seed0,
                    seed1,
                    initial_advances,
                    max_advances,
                );
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let pkm_table = table(
            [
                table::column("Advances", |pkm: &Pkm| text(pkm.advances)),
                table::column("Egg Seed", |pkm: &Pkm| text(pkm.egg_seed)),
                table::column("PID", |pkm: &Pkm| text(pkm.pid)),
                table::column("Shiny", |pkm: &Pkm| text(pkm.shiny)),
                table::column("Nature", |pkm: &Pkm| text(pkm.nature)),
                table::column("Ability", |pkm: &Pkm| text(pkm.ability)),
                table::column("HP", |pkm: &Pkm| text(pkm.hp)),
                table::column("Atk", |pkm: &Pkm| text(pkm.atk)),
                table::column("Def", |pkm: &Pkm| text(pkm.def)),
                table::column("SpA", |pkm: &Pkm| text(pkm.spa)),
                table::column("SpD", |pkm: &Pkm| text(pkm.spd)),
                table::column("Spe", |pkm: &Pkm| text(pkm.spe)),
                table::column("Gender", |pkm: &Pkm| text(pkm.gender)),
                table::column("EC", |pkm: &Pkm| text(pkm.ec)),
            ],
            &self.results,
        )
        .width(Length::Fill);

        column![
            row![
                column![
                    text("Game Info").size(20),
                    text("TID"),
                    text_input("0", &self.tid)
                        .on_input(Message::TidChanged)
                        .width(Length::Fill),
                    text("SID"),
                    text_input("0", &self.sid)
                        .on_input(Message::SidChanged)
                        .width(Length::Fill),
                    checkbox(self.shiny_charm)
                        .label("Shiny Charm")
                        .on_toggle(Message::ShinyCharmToggled),
                    checkbox(self.oval_charm)
                        .label("Oval Charm")
                        .on_toggle(Message::OvalCharmToggled),
                ]
                .spacing(12)
                .width(Length::Fill)
                .align_x(Left),
                Space::new().width(Length::Fixed(12.0)),
                column![
                    text("Nursery").size(20),
                    text("Compatibility"),
                    combo_box(
                        &self.compatibility_state,
                        "Select compatibility",
                        Some(&self.compatibility),
                        Message::CompatibilityChanged,
                    )
                    .width(Length::Fill),
                    text("Gender Ratio"),
                    combo_box(
                        &self.gender_ratio_state,
                        "Select gender ratio",
                        Some(&self.gender_ratio),
                        Message::GenderRatioChanged,
                    )
                    .width(Length::Fill),
                    checkbox(self.masuda)
                        .label("Masuda")
                        .on_toggle(Message::MasudaToggled),
                ]
                .spacing(12)
                .width(Length::Fill)
                .align_x(Left),
                Space::new().width(Length::Fixed(12.0)),
                column![
                    text("RNG State").size(20),
                    text("Seeds"),
                    text_input("0", &self.seed0)
                        .on_input(Message::Seed0Changed)
                        .width(Length::Fill),
                    text_input("0", &self.seed1)
                        .on_input(Message::Seed1Changed)
                        .width(Length::Fill),
                    text("Advances"),
                    text_input("0", &self.initial_advances)
                        .on_input(Message::InitialAdvancesChanged)
                        .width(Length::Fill),
                    text_input("0", &self.max_advances)
                        .on_input(Message::MaxAdvancesChanged)
                        .width(Length::Fill),
                ]
                .spacing(12)
                .width(Length::Fill)
                .align_x(Left),
            ],
            Space::new().height(Length::Fixed(12.0)),
            button("Search").on_press(Message::Search).padding([10, 20]),
            Space::new().height(Length::Fill),
            pkm_table,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Center)
        .padding(12)
    }
}
