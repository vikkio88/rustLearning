const EARTH_YEARS_SECONDS: u64 = 31557600;

const MERCURY: f64 = 0.2408467;
const VENUS: f64 = 0.61519726;
const EARTH: f64 = 1.0;
const MARS: f64 = 1.8808158;
const JUPITER: f64 = 11.862615;
const SATURN: f64 = 29.447498;
const URANUS: f64 = 84.016846;
const NEPTUNE: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64
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
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64 / EARTH_YEARS_SECONDS as f64) / MERCURY as f64
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64 / EARTH_YEARS_SECONDS as f64) / VENUS as f64
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 * EARTH / EARTH_YEARS_SECONDS as f64
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64 / EARTH_YEARS_SECONDS as f64) / MARS as f64
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64 / EARTH_YEARS_SECONDS as f64) / JUPITER as f64
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64 / EARTH_YEARS_SECONDS as f64) / SATURN as f64
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64 / EARTH_YEARS_SECONDS as f64) / URANUS as f64
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64 / EARTH_YEARS_SECONDS as f64) / NEPTUNE as f64
    }
}
