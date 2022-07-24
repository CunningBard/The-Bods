use std::collections::HashMap;


trait Income {
    fn income(&self) -> u32;
    fn job_hours(&self) -> u8;
}

struct Job {
    level: u8,
    job_hours: u8,
    multiplier: f32,
    base: u32
}

impl Income for Job {
    fn income(&self) -> u32 {
        (self.base as f32 * self.multiplier) as u32
    }

    fn job_hours(&self) -> u8 {
        self.job_hours
    }
}


enum Feeling {
    Happy,
    Sad
}

enum RelationshipType {
    Father,
    Mother,
    Brother,
    Sister,
    Son,
    Daughter

}

struct Relationship {
    id: u32,
    strength: u8,
    relationship_type: RelationshipType
}

struct Bod {
    id: u32,
    age: u8,
    health: u8,
    hunger: u8,
    happiness: u8,
    money: i32,
    income: Vec<Box<dyn Income>>,
    feeling: Feeling,
    relationships: HashMap<String, Vec<Relationship>>
}

impl Bod {
    fn new(id: u32) -> Self {
        Self {
            id,
            age: 0,
            health: 100,
            hunger: 100,
            happiness: 100,
            money: 0,
            income: vec![],
            feeling: Feeling::Happy,
            relationships: Default::default()
        }
    }
    fn step(&mut self){

    }
}

struct World {
    people: HashMap<usize, Bod>
}

impl World {
    fn new(people_start: u32, _max_people: u32) -> Self {
        Self {
            people: HashMap::new()
        }
    }
    fn step(&mut self){
        for (_bod_id, bod) in &mut self.people {
            bod.step();
        }
    }
    fn run(&mut self){
        loop {
            self.step()
        }
    }
}

fn main() {
    World::new(100, 100).run();
}
