use std::fmt::Display;

/// Enumeration of all states in the US.
///
/// ```
/// use us_state_info_rs::State;
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
    PuertoRico
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
        match s.as_str() {
            "AL" | "Alabama" => Ok(State::Alabama),
            "AK" | "Alaska" => Ok(State::Alaska),
            "AZ" | "Arizona" => Ok(State::Arizona),
            "AR" | "Arkansas" => Ok(State::Arkansas),
            "CA" | "California" => Ok(State::California),
            "CO" | "Colorado" => Ok(State::Colorado),
            "CT" | "Connecticut" => Ok(State::Connecticut),
            "DE" | "Delaware" => Ok(State::Delaware),
            "DC" | "District of Columbia" => Ok(State::DistrictOfColumbia),
            "FL" | "Florida" => Ok(State::Florida),
            "GA" | "Georgia" => Ok(State::Georgia),
            "HI" | "Hawaii" => Ok(State::Hawaii),
            "ID" | "Idaho" => Ok(State::Idaho),
            "IL" | "Illinois" => Ok(State::Illinois),
            "IN" | "Indiana" => Ok(State::Indiana),
            "IA" | "Iowa" => Ok(State::Iowa),
            "KS" | "Kansas" => Ok(State::Kansas),
            "KY" | "Kentucky" => Ok(State::Kentucky),
            "LA" | "Louisiana" => Ok(State::Louisiana),
            "ME" | "Maine" => Ok(State::Maine),
            "MD" | "Maryland" => Ok(State::Maryland),
            "MA" | "Massachusetts" => Ok(State::Massachusetts),
            "MI" | "Michigan" => Ok(State::Michigan),
            "MN" | "Minnesota" => Ok(State::Minnesota),
            "MS" | "Mississippi" => Ok(State::Mississippi),
            "MO" | "Missouri" => Ok(State::Missouri),
            "MT" | "Montana" => Ok(State::Montana),
            "NE" | "Nebraska" => Ok(State::Nebraska),
            "NV" | "Nevada" => Ok(State::Nevada),
            "NH" | "New Hampshire" => Ok(State::NewHampshire),
            "NJ" | "New Jersey" => Ok(State::NewJersey),
            "NM" | "New Mexico" => Ok(State::NewMexico),
            "NY" | "New York" => Ok(State::NewYork),
            "NC" | "North Carolina" => Ok(State::NorthCarolina),
            "ND" | "North Dakota" => Ok(State::NorthDakota),
            "OH" | "Ohio" => Ok(State::Ohio),
            "OK" | "Oklahoma" => Ok(State::Oklahoma),
            "OR" | "Oregon" => Ok(State::Oregon),
            "PA" | "Pennsylvania" => Ok(State::Pennsylvania),
            "RI" | "Rhode Island" => Ok(State::RhodeIsland),
            "SC" | "South Carolina" => Ok(State::SouthCarolina),
            "SD" | "South Dakota" => Ok(State::SouthDakota),
            "TN" | "Tennessee" => Ok(State::Tennessee),
            "TX" | "Texas" => Ok(State::Texas),
            "UT" | "Utah" => Ok(State::Utah),
            "VT" | "Vermont" => Ok(State::Vermont),
            "VA" | "Virginia" => Ok(State::Virginia),
            "WA" | "Washington" => Ok(State::Washington),
            "WV" | "West Virginia" => Ok(State::WestVirginia),
            "WI" | "Wisconsin" => Ok(State::Wisconsin),
            "WY" | "Wyoming" => Ok(State::Wyoming),
            "PR" | "Puerto Rico" => Ok(State::PuertoRico),
            _ => Err(serde::de::Error::unknown_variant(
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
