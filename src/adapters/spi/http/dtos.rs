use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HttpBinRequestDto {
    pub value: u8,
}

#[derive(Serialize, Deserialize)]
pub struct HttpBinResponseDto {
    pub json: HttpBinRequestDto,
}
