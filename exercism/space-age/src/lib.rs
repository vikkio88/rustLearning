const EARTH_YEARS_SECONDS: u64 = 31557600;

const MERCURY: f64 = 0.2408467;
const VENUS: f64 = 0.61519726;
const Earth: f64 = 1.0;
const Mars: f64 = 1.8808158;
const Jupiter: f64 = 11.862615;
const Saturn: f64 = 29.447498;
const Uranus: f64 = 84.016846;
const Neptune: f64 = 164.79132;

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

impl Planet for Mercury {}
impl Planet for Venus {}
impl Planet for Earth {}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
