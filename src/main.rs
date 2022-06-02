use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;


fn rand_prob(number: i32) -> bool
{
    let num = thread_rng().gen_range(0..100);
    if number > num {
        true
    }
    else {
        false
    }
}
fn rand_range(start: i32, stop: i32) -> i32
{
    thread_rng().gen_range(start..stop)
}

fn vec_shuffle<T>(vec: &mut Vec<T>)
{
    vec.shuffle(&mut thread_rng());
}

fn rand_number_increase_prob(mut start_prob: i32, minus_per_iteration: i32) -> i32
{
    let mut res = rand_prob(start_prob);
    let mut num = 0;
    if res {
        num += 1;
    }
    while res {
        start_prob -= minus_per_iteration;
        res = rand_prob(start_prob);
        if res {
            num += 1;
        }
    }
    num
}

#[derive(Debug, Clone)]
struct Virus
{
    r_naught: i32,
    life_span: i32,
}

#[derive(Debug)]
struct Person
{
    id: i32,
    is_infected: bool,
    is_recovered: bool,
    is_quarantined: bool,
    virus: Virus
}

impl Person
{
    fn person(id: i32) -> Person {
        Person
        {
            id,
            is_infected: false,
            is_recovered: false,
            is_quarantined: false,
            virus: Virus {r_naught: 0, life_span: 0}
        }
    }
    fn infect(&mut self, virus: Virus) {
        self.is_infected = true;
        self.virus = virus;
    }
    fn recover(&mut self) {
        assert!(!self.is_infected, "how the fuck can an non infected recover");

        self.is_infected = false;
        self.is_recovered = true;
    }

    fn get_infection_chance(&self) -> i32 {
        if self.is_infected {
            let infection_chance = (self.virus.r_naught * 100 / self.virus.life_span * 100) / 2;
            return infection_chance
        }
        0
    }
}

#[derive(Debug)]
struct Building
{
    capacity: i32,           
    people_inside: Vec<Person>,
}

impl Building
{
    fn add(&mut self, person: Person)
    {
        self.people_inside.push(person);
    }

    fn infect_random(&mut self) {

    }

    fn infect(&self)
    {
        let mut can_be_infected = 0;
        for person_ in &self.people_inside {
            if !person_.is_infected && !person_.is_recovered {
                can_be_infected += 1;
            }
        }
        for person in &self.people_inside {
            if can_be_infected < 1 {
                break
            }

            if person.is_infected{
                let mut chance = person.get_infection_chance();
                while chance > 100 {
                    chance -= 100;
                }
            }
        }
    }
}

fn main()  {
    let mut houses: Vec<Building> = vec![];
    let mut remain = 100;
    let mut ids = 0;
    while remain > 0
    {
        let mut res_prob = rand_number_increase_prob(100, 10);
        if res_prob > remain { res_prob = remain }

        let mut new_house = Building { people_inside: vec![], capacity: res_prob };
        remain -= res_prob;

        for _i in 1..(res_prob + 1) {
            ids += 1;
            new_house.add(Person::person(ids));
        }
        houses.push(new_house);
    }
    vec_shuffle(&mut houses);
    let virus = Virus{ r_naught: 5, life_span: 16 };
    houses[0].people_inside[0].infect(virus.clone());
    for house in &houses {
        println!("{:?}", house);
    }
}