use std::{convert::{TryFrom, TryInto}, fmt::Display};

use crate::ConversionError;

/// Enumeration of all states in the US.
///
/// ```
/// use us_state_info::State;
/// let iowa_state = State::Iowa;
/// format!("{}-{}", iowa_state, iowa_state.abbreviation());
///
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]

pub enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    DistrictOfColumbia,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
    PuertoRico,
}

impl State {
    /// Returns the two letter abbreviation for the state.
    pub fn abbreviation(&self) -> &'static str {
        match self {
            State::Alabama => "AL",
            State::Alaska => "AK",
            State::Arizona => "AZ",
            State::Arkansas => "AR",
            State::California => "CA",
            State::Colorado => "CO",
            State::Connecticut => "CT",
            State::Delaware => "DE",
            State::DistrictOfColumbia => "DC",
            State::Florida => "FL",
            State::Georgia => "GA",
            State::Hawaii => "HI",
            State::Idaho => "ID",
            State::Illinois => "IL",
            State::Indiana => "IN",
            State::Iowa => "IA",
            State::Kansas => "KS",
            State::Kentucky => "KY",
            State::Louisiana => "LA",
            State::Maine => "ME",
            State::Maryland => "MD",
            State::Massachusetts => "MA",
            State::Michigan => "MI",
            State::Minnesota => "MN",
            State::Mississippi => "MS",
            State::Missouri => "MO",
            State::Montana => "MT",
            State::Nebraska => "NE",
            State::Nevada => "NV",
            State::NewHampshire => "NH",
            State::NewJersey => "NJ",
            State::NewMexico => "NM",
            State::NewYork => "NY",
            State::NorthCarolina => "NC",
            State::NorthDakota => "ND",
            State::Ohio => "OH",
            State::Oklahoma => "OK",
            State::Oregon => "OR",
            State::Pennsylvania => "PA",
            State::RhodeIsland => "RI",
            State::SouthCarolina => "SC",
            State::SouthDakota => "SD",
            State::Tennessee => "TN",
            State::Texas => "TX",
            State::Utah => "UT",
            State::Vermont => "VT",
            State::Virginia => "VA",
            State::Washington => "WA",
            State::WestVirginia => "WV",
            State::Wisconsin => "WI",
            State::Wyoming => "WY",
            State::PuertoRico => "PR",
        }
    }
}

impl TryFrom<String> for State {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().replace(" ", "").as_str() {
            "al" | "alabama" => Ok(State::Alabama),
            "ak" | "alaska" => Ok(State::Alaska),
            "az" | "arizona" => Ok(State::Arizona),
            "ar" | "arkansas" => Ok(State::Arkansas),
            "ca" | "california" => Ok(State::California),
            "co" | "colorado" => Ok(State::Colorado),
            "ct" | "connecticut" => Ok(State::Connecticut),
            "de" | "delaware" => Ok(State::Delaware),
            "dc" | "districtofcolumbia" => Ok(State::DistrictOfColumbia),
            "fl" | "florida" => Ok(State::Florida),
            "ga" | "georgia" => Ok(State::Georgia),
            "hi" | "hawaii" => Ok(State::Hawaii),
            "id" | "idaho" => Ok(State::Idaho),
            "il" | "illinois" => Ok(State::Illinois),
            "in" | "indiana" => Ok(State::Indiana),
            "ia" | "iowa" => Ok(State::Iowa),
            "ks" | "kansas" => Ok(State::Kansas),
            "ky" | "kentucky" => Ok(State::Kentucky),
            "la" | "louisiana" => Ok(State::Louisiana),
            "me" | "maine" => Ok(State::Maine),
            "md" | "maryland" => Ok(State::Maryland),
            "ma" | "massachusetts" => Ok(State::Massachusetts),
            "mi" | "michigan" => Ok(State::Michigan),
            "mn" | "minnesota" => Ok(State::Minnesota),
            "ms" | "mississippi" => Ok(State::Mississippi),
            "mo" | "missouri" => Ok(State::Missouri),
            "mt" | "montana" => Ok(State::Montana),
            "ne" | "nebraska" => Ok(State::Nebraska),
            "nv" | "nevada" => Ok(State::Nevada),
            "nh" | "newhampshire" => Ok(State::NewHampshire),
            "nj" | "newjersey" => Ok(State::NewJersey),
            "nm" | "newmexico" => Ok(State::NewMexico),
            "ny" | "newyork" => Ok(State::NewYork),
            "nc" | "northcarolina" => Ok(State::NorthCarolina),
            "nd" | "northdakota" => Ok(State::NorthDakota),
            "oh" | "ohio" => Ok(State::Ohio),
            "ok" | "oklahoma" => Ok(State::Oklahoma),
            "or" | "oregon" => Ok(State::Oregon),
            "pa" | "pennsylvania" => Ok(State::Pennsylvania),
            "ri" | "rhodeisland" => Ok(State::RhodeIsland),
            "sc" | "southcarolina" => Ok(State::SouthCarolina),
            "sd" | "southdakota" => Ok(State::SouthDakota),
            "tn" | "tennessee" => Ok(State::Tennessee),
            "tx" | "texas" => Ok(State::Texas),
            "ut" | "utah" => Ok(State::Utah),
            "vt" | "vermont" => Ok(State::Vermont),
            "va" | "virginia" => Ok(State::Virginia),
            "wa" | "washington" => Ok(State::Washington),
            "wv" | "westvirginia" => Ok(State::WestVirginia),
            "wi" | "wisconsin" => Ok(State::Wisconsin),
            "wy" | "wyoming" => Ok(State::Wyoming),
            "pr" | "puertorico" => Ok(State::PuertoRico),
            _ => Err(ConversionError),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Alabama => "Alabama",
                State::Alaska => "Alaska",
                State::Arizona => "Arizona",
                State::Arkansas => "Arkansas",
                State::California => "California",
                State::Colorado => "Colorado",
                State::Connecticut => "Connecticut",
                State::Delaware => "Delaware",
                State::DistrictOfColumbia => "District of Columbia",
                State::Florida => "Florida",
                State::Georgia => "Georgia",
                State::Hawaii => "Hawaii",
                State::Idaho => "Idaho",
                State::Illinois => "Illinois",
                State::Indiana => "Indiana",
                State::Iowa => "Iowa",
                State::Kansas => "Kansas",
                State::Kentucky => "Kentucky",
                State::Louisiana => "Louisiana",
                State::Maine => "Maine",
                State::Maryland => "Maryland",
                State::Massachusetts => "Massachusetts",
                State::Michigan => "Michigan",
                State::Minnesota => "Minnesota",
                State::Mississippi => "Mississippi",
                State::Missouri => "Missouri",
                State::Montana => "Montana",
                State::Nebraska => "Nebraska",
                State::Nevada => "Nevada",
                State::NewHampshire => "New Hampshire",
                State::NewJersey => "New Jersey",
                State::NewMexico => "New Mexico",
                State::NewYork => "New York",
                State::NorthCarolina => "North Carolina",
                State::NorthDakota => "North Dakota",
                State::Ohio => "Ohio",
                State::Oklahoma => "Oklahoma",
                State::Oregon => "Oregon",
                State::Pennsylvania => "Pennsylvania",
                State::RhodeIsland => "Rhode Island",
                State::SouthCarolina => "South Carolina",
                State::SouthDakota => "South Dakota",
                State::Tennessee => "Tennessee",
                State::Texas => "Texas",
                State::Utah => "Utah",
                State::Vermont => "Vermont",
                State::Virginia => "Virginia",
                State::Washington => "Washington",
                State::WestVirginia => "West Virginia",
                State::Wisconsin => "Wisconsin",
                State::Wyoming => "Wyoming",
                State::PuertoRico => "Puerto Rico",
            }
        )
    }
}

#[cfg(all(feature = "serde", not(feature = "serde_abbreviation")))]
impl serde::Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde_abbreviation")]
impl serde::Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.abbreviation())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.clone().try_into() {
            Ok(state) => Ok(state),
            Err(_) => Err(serde::de::Error::unknown_variant(
                &s,
                &[
                    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "DC", "FL", "GA", "HI", "ID",
                    "IL", "IN", "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO",
                    "MT", "NE", "NV", "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA",
                    "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY",
                ],
            )),
        }
    }
}
