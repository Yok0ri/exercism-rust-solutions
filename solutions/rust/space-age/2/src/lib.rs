/// Version with encapsulation
#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    pub fn seconds(&self) -> u64 {
        self.seconds
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds() as f64 / (31_557_600.0 * Self::orbit_period())
    }
    fn orbit_period() -> f64;
}

/// Version that allows defining multiple planets in a single macro
macro_rules! planets {
    ( $( $name:ident => $period:expr ),* ) => {
        $(
            impl Planet for $name {
                fn orbit_period() -> f64 {
                    $period
                }
            }
        )*
    };
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

planets! {
    Mercury => 0.2408467,
    Venus => 0.61519726,
    Earth => 1.0,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.79132
}
