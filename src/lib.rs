pub mod config;

pub type FieldID = u16;

pub enum ConnectionType {
    Taxi,
    Bus,
    Underground,
    Ferry,
}

pub struct Map {
    taxi_connections: Vec<(FieldID, FieldID)>,
    bus_connections: Vec<(FieldID, FieldID)>,
    underground_connections: Vec<(FieldID, FieldID)>,
    ferry_connections: Vec<(FieldID, FieldID)>,
}

impl Map {
    pub fn possible_targets(&self, field: FieldID, ty: ConnectionType) -> Vec<FieldID> {
        let to_search = match ty {
            ConnectionType::Taxi => &self.taxi_connections,
            ConnectionType::Bus => &self.bus_connections,
            ConnectionType::Underground => &self.underground_connections,
            ConnectionType::Ferry => &self.ferry_connections,
        };

        to_search
            .iter()
            .filter_map(|(a, b)| (*a == field).then(|| *b).or((*b == field).then(|| *a)))
            .collect()
    }
}
