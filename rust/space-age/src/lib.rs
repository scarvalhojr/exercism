#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

pub trait Planet {
    const EARTH_ORBIT_SEC: f64 = 31_557_600.0;
    const ORBIT_EARTH_YEARS: f64;

    fn years_during(d: &Duration) -> f64 {
        (d.seconds / Self::EARTH_ORBIT_SEC) / Self::ORBIT_EARTH_YEARS
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBIT_EARTH_YEARS: f64 = 0.240_846_700;
}

impl Planet for Venus {
    const ORBIT_EARTH_YEARS: f64 = 0.615_197_260;
}

impl Planet for Earth {
    const ORBIT_EARTH_YEARS: f64 = 1.0;
}

impl Planet for Mars {
    const ORBIT_EARTH_YEARS: f64 = 1.880_815_800;
}

impl Planet for Jupiter {
    const ORBIT_EARTH_YEARS: f64 = 11.862_615;
}

impl Planet for Saturn {
    const ORBIT_EARTH_YEARS: f64 = 29.447_498;
}

impl Planet for Uranus {
    const ORBIT_EARTH_YEARS: f64 = 84.016_846;
}

impl Planet for Neptune {
    const ORBIT_EARTH_YEARS: f64 = 164.791_320;
}
