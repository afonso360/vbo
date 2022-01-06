use log::error;
use core::fmt;
use dms_coordinates::DMS;
use time::{format_description, Time};


#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ChannelName {
    Satellites,
    Time,
    Latitude,
    Longitude,
    Velocity,
    Heading,
    Height,
    LongAccel,
    LatAccel,
    Custom(String)
}

impl ChannelName {
    pub fn as_str(&self) -> &str {
        match self {
            ChannelName::Satellites => "satellites",
            ChannelName::Time => "time",
            ChannelName::Latitude => "latitude",
            ChannelName::Longitude => "longitude",
            ChannelName::Velocity => "velocity",
            ChannelName::Heading => "heading",
            ChannelName::Height => "height",
            ChannelName::LongAccel => "long accel",
            ChannelName::LatAccel => "lat accel",
            ChannelName::Custom(s) => s.as_str(),
        }
    }
}

impl fmt::Display for ChannelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for ChannelName {
    fn from(s: String) -> Self {
        ChannelName::from(s.as_str())
    }
}

impl<'a> From<&'a str> for ChannelName {
    fn from(s: &'a str) -> Self {
        match s {
            "satellites" => ChannelName::Satellites,
            "time" => ChannelName::Time,
            "latitude" => ChannelName::Latitude,
            "longitude" => ChannelName::Longitude,
            "velocity" => ChannelName::Velocity,
            "heading" => ChannelName::Heading,
            "height" => ChannelName::Height,
            "long accel" => ChannelName::LongAccel,
            "lat accel" => ChannelName::LatAccel,
            s => ChannelName::Custom(s.into()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ChannelUnit {
    Kmh,
    G,
    Custom(String)
}

impl ChannelUnit {
    pub fn as_str(&self) -> &str {
        match self {
            ChannelUnit::Kmh => "kmh",
            ChannelUnit::G => "g",
            ChannelUnit::Custom(s) => s.as_str(),
        }
    }
}

impl fmt::Display for ChannelUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for ChannelUnit {
    fn from(s: String) -> Self {
        ChannelUnit::from(s.as_str())
    }
}

impl<'a> From<&'a str> for ChannelUnit {
    fn from(s: &'a str) -> Self {
        match s {
            "kmh" => ChannelUnit::Kmh,
            "g" => ChannelUnit::G,
            s => ChannelUnit::Custom(s.into()),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Channel {
    pub name: ChannelName,
    pub unit: Option<ChannelUnit>,
}

impl Channel {
    pub fn new(name: ChannelName, unit: Option<ChannelUnit>) -> Self {
        Self {
            name,
            unit
        }
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(unit) = &self.unit {
            write!(f, " {}", unit)?;
        }
        Ok(())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum ChannelValue {
    ///This is the number of satellites in use in decimal format. 64 is added to this number if the brake trigger input is activated. 128 is added to this number if the VBOX is using a DGPS correction.
    /// e.g. in the file above the sats column shows 137 = 128(DGPS) + 9 sats.
    Satellites(u8),

    /// This is UTC time since midnight in the form HH:MM:SS.SS.
    Time(Time),

    /// Latitude in minutes MMMMM.MMMMM +ve = North e.g. 03119.09973M = 51D, 59M, 5.9838S
    /// Longitude in minutes MMMMM.MMM +ve = West e.g. 00058.49277M = 00D, 58M, 29.562S.
    Coordinates(DMS),

    /// Velocity in km/h. e.g.: `010.184`
    Velocity(f64),

    /// Heading in degrees with respect to North.. e.g.: `213.90`
    Heading(f64),

    ///  Height above sea level in meters based on the WGS84 model of the earth used by VBOX GPS engines. e.g.: `+00091.70`
    Height(f64),
}

/// Converts a DMS into a minutes only representation used by VBOX
fn dms_to_minutes(dms: &DMS) -> f64 {
    let deg = dms.get_degrees() as f64 * 60.0;
    let min = dms.get_minutes() as f64;
    let sec = dms.get_seconds() / 60.0;
    let bearing = dms.get_bearing();
    let nw_multiplier = if bearing == 'N' || bearing == 'W' {
        1.0
    } else {
        -1.0
    };

    (deg + min + sec) * nw_multiplier
}


impl fmt::Display for ChannelValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelValue::Satellites(n) => write!(f, "{:0>3}", n),
            ChannelValue::Time(t) => {
                let format = format_description::parse("[hour padding:zero][minute padding:zero][second padding:zero].[subsecond digits:2]").unwrap();
                let formatted = t.format(&format).map_err(|e| {
                    error!("Failed to format time: {} - error: {}", t, e);
                    fmt::Error
                })?;
                write!(f, "{}", formatted)
            },
            ChannelValue::Coordinates(c) => write!(f, "{:0>+013.6}", dms_to_minutes(c)),
            ChannelValue::Velocity(v) => write!(f, "{:0>7.3}", v),
            ChannelValue::Heading(v) => write!(f, "{:0>6.2}", v),
            ChannelValue::Height(v) => write!(f, "{:0>+08.2}", v),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_name_from() {
        let channels = [
            ("satellites", ChannelName::Satellites),
            ("time", ChannelName::Time),
            ("latitude", ChannelName::Latitude),
            ("longitude", ChannelName::Longitude),
            ("velocity", ChannelName::Velocity),
            ("heading", ChannelName::Heading),
            ("height", ChannelName::Height),
            ("long accel", ChannelName::LongAccel),
            ("lat accel", ChannelName::LatAccel),
            ("device_update_rate", ChannelName::Custom("device_update_rate".into())),
            ("lean_angle", ChannelName::Custom("lean_angle".into())),
            ("combined_acc", ChannelName::Custom("combined_acc".into())),
            ("fix_type", ChannelName::Custom("fix_type".into())),
            ("coordinate_precision", ChannelName::Custom("coordinate_precision".into())),
            ("altitude_precision", ChannelName::Custom("altitude_precision".into())),
        ];

        for (name, cn) in channels.into_iter() {
            let cn_from_name = ChannelName::from(name);
            assert_eq!(cn_from_name, cn);

            let name_from_cn = cn_from_name.as_str();
            assert_eq!(name_from_cn, name);
        }
    }


    #[test]
    fn channel_unit_from() {
        let units = [
            ("kmh", ChannelUnit::Kmh),
            ("g", ChannelUnit::G),
            ("ms2", ChannelUnit::Custom("ms2".into())),
        ];

        for (name, cu) in units.into_iter() {
            let cu_from_name = ChannelUnit::from(name);
            assert_eq!(cu_from_name, cu);

            let name_from_cu = cu_from_name.as_str();
            assert_eq!(name_from_cu, name);
        }
    }

    #[test]
    fn fmt_channel() {
        let channels = [
            ("longitude", Channel::new(ChannelName::Longitude, None)),
            ("velocity kmh", Channel::new(ChannelName::Velocity, Some(ChannelUnit::Kmh))),
            ("long accel g", Channel::new(ChannelName::LongAccel, Some(ChannelUnit::G))),
        ];

        for (formatted, channel) in channels.into_iter() {
            assert_eq!(formatted, format!("{}", channel));
        }
    }

    #[test]
    fn fmt_channel_value() {
        let values = [
            ("003", ChannelValue::Satellites(3)),
            ("031", ChannelValue::Satellites(31)),
            ("170538.19", ChannelValue::Time(Time::from_hms_milli(17, 05, 38, 190).unwrap())),
            ("172317.59", ChannelValue::Time(Time::from_hms_milli(17, 23, 17, 590).unwrap())),
            ("+03119.099730", ChannelValue::Coordinates(DMS::new(51, 59, 5.9838, 'N').unwrap())),
            ("-03119.099730", ChannelValue::Coordinates(DMS::new(51, 59, 5.9838, 'S').unwrap())),
            ("+00058.492700", ChannelValue::Coordinates(DMS::new(0, 58, 29.562, 'W').unwrap())),
            ("-00058.492700", ChannelValue::Coordinates(DMS::new(0, 58, 29.562, 'E').unwrap())),
            ("058.493", ChannelValue::Velocity(58.493)),
            ("000.001", ChannelValue::Velocity(0.001)),
            ("039.40", ChannelValue::Heading(39.40)),
            ("293.00", ChannelValue::Heading(293.00)),
            ("+0155.06", ChannelValue::Height(155.06)),
            ("-0293.00", ChannelValue::Height(-293.00)),
        ];

        for (formatted, value) in values.into_iter() {
            assert_eq!(formatted, format!("{}", value));
        }
    }
}
