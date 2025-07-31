pub mod config;

pub type FieldID = u16;

#[derive(Clone, Default, Debug)]
pub struct Field {
    pub id : FieldID,

    pub taxi_conn : Vec<FieldID>,

    pub bus_conn : Option<Vec<FieldID>>,
    pub metro_conn : Option<Vec<FieldID>>,
    pub ferry_conn : Option<Vec<FieldID>>
}

impl Field {
    pub fn new(id : FieldID) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn has_bus(&self) -> bool {
        self.bus_conn.is_some()
    }

    pub fn has_metro(&self) -> bool {
        self.metro_conn.is_some()
    }
}

pub struct Map {
    pub field_list : Vec<Field>
}

impl Map {
    pub fn add_field(&mut self) -> &mut Field {
        self.field_list.push(Field::new(self.field_list.len() as FieldID)); 
        self.field_list.last_mut().unwrap()  
    }
}