pub type Units = u32;
pub type Milliseconds = f64;

pub const UNIT_0_625_MS: Units = 625;
pub const UNIT_1_25_MS: Units = 1250;
pub const UNIT_10_MS: Units = 10000;


pub trait UnitsMethods {
    fn to_ms(&self, resolution: Units) -> Milliseconds;

    fn from_ms(value: Milliseconds, resolution: Units) -> Self;
}

pub trait MillisecondsMethods {
    fn to_units(&self, resolution: Units) -> Units;
    fn from_units(value: Units, resolution: Units) -> Self;
}

impl UnitsMethods for Units {
    fn to_ms(&self, resolution: Units) -> Milliseconds {
        let num = (*self as f64) * (resolution as f64);
        return num / 1000_f64;
    }

    fn from_ms(value: Milliseconds, resolution: Units) -> Self {
        value.to_units(resolution)
    }
}

impl MillisecondsMethods for Milliseconds {
    fn to_units(&self, resolution: Units) -> Units {
        let num = *self * 1000_f64;
        let value = (num as Units) / resolution;
        value
    }

    fn from_units(value: Units, resolution: Units) -> Self {
        value.to_ms(resolution)
    }
}
