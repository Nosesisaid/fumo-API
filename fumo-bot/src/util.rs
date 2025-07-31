use std::fmt;
use strum::EnumIter;

//IMPORTANT to keep synced with fumo_db::INVOLVABLE. Haven't found a good way to automate this.
#[derive(Debug, poise::ChoiceParameter, EnumIter)]
pub enum InvolvableChoice {
    Cirno,
    Reimu,
    Remilia,
}
impl From<InvolvableChoice> for String {
    fn from(i: InvolvableChoice) -> Self {
        match i {
            InvolvableChoice::Cirno => "Cirno",
            InvolvableChoice::Reimu => "Reimu",
            InvolvableChoice::Remilia => "Remilia",
        }
        .to_string()
    }
}
impl fmt::Display for InvolvableChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
// Really, if anyone knows a better way to do this, please tell me. I feel very dumb doing this.
