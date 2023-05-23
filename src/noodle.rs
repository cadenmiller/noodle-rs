use std::collections::HashMap;

pub enum NoodleKind {
    Group,
}

pub struct Noodle {
    kind:       NoodleKind, 
    name:       String,
    children:   HashMap<String, Noodle>,
}

impl Noodle {

    fn default() -> Noodle
    {
        Noodle{
            kind: NoodleKind::Group,
            name: String::new(),
            children: HashMap::new()
        }
    }

    fn make_group(name: String) -> Noodle {
        Noodle{
            kind: NoodleKind::Group, 
            name: name,
            children: HashMap::new()}
    }

    fn parse(content: &String) -> Noodle {
        // Create an empty group type
        let mut root = Self::make_group(String::new());
    }

    pub fn from_string(content: &String) -> Noodle {
        parse()

        let mut root: Noodle = Noodle{ name: "".to_string(), children: HashMap::new() };
        
        return root;
    }
}