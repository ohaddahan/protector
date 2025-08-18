use anchor_lang::prelude::*;

/// Errors
#[error_code]
pub enum Errors {
    #[msg("Account is not authorized to execute this instruction")]
    Unauthorized,
    #[msg("Inspector Error 1")]
    InspectorError1,
    #[msg("Inspector Error 2")]
    InspectorError2,
    #[msg("Inspector Error 3")]
    InspectorError3,
    #[msg("Inspector Error 4")]
    InspectorError4,
    #[msg("Deserialize filter failure")]
    DeserializeFilterFailure,
}
