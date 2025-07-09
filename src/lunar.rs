use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
use ciborium::{from_reader, into_writer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct InputDate {
    year: u16,
    month: u8,
    day: u8,
}

impl TryFrom<InputDate> for SolarDate {
    type Error = String;
    fn try_from(input: InputDate) -> Result<Self, Self::Error> {
        SolarDate::from_ymd(input.year, input.month, input.day).map_err(|e| e.to_string())
    }
}

#[derive(Serialize)]
struct OutputLunisolarDate {
    year: String,
    month: u8,
    day: u8,
}

impl From<LunisolarDate> for OutputLunisolarDate {
    fn from(date: LunisolarDate) -> Self {
        let year = date.to_lunar_year();
        let month = date.to_lunar_month();
        let day = date.to_lunar_day();
        OutputLunisolarDate {
            year: year.to_string(),
            month: month.to_u8(),
            day: day.to_u8(),
        }
    }
}

pub fn solar_to_lunisolar(input: &[u8]) -> Result<Vec<u8>, String> {
    let input: InputDate = from_reader(input).map_err(|e| e.to_string())?;
    let solar_date: SolarDate = input.try_into()?;
    let lunisolar_date = LunisolarDate::from_solar_date(solar_date).map_err(|e| e.to_string())?;
    let output: OutputLunisolarDate = lunisolar_date.into();
    let mut output_bytes = Vec::new();
    into_writer(&output, &mut output_bytes).map_err(|e| e.to_string())?;
    Ok(output_bytes)
}
