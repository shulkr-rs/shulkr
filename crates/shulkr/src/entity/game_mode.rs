use shulkr_macros::Enumeration;

#[derive(Enumeration)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}
