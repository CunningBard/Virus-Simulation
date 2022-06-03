use std::fs;
use std::time::SystemTime;
use rand::thread_rng;
use rand::Rng;
use std::env;
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

#[derive(Debug, Clone, Copy)]
struct Virus
{
    r_naught: i32,
    life_span: i32,
}

#[derive(Debug, Clone, Copy)]
struct Person
{
    id: i32,
    house_id: i32,
    is_infected: bool,
    is_recovered: bool,
    is_quarantined: bool,
    days_since_infected: i32,
    virus: Virus
}

impl Person
{
    fn person(id: i32, house_id: i32) -> Person {
        Person
        {
            id,
            house_id,
            is_infected: false,
            is_recovered: false,
            is_quarantined: false,
            days_since_infected: 0,
            virus: Virus {r_naught: 0, life_span: 0}
        }
    }
    fn infect(&mut self, virus: Virus) {
        self.is_infected = true;
        self.virus = virus;
    }
    fn recover(&mut self) {
        assert!(self.is_infected, "how the fuck can an non infected recover");

        self.is_infected = false;
        self.is_recovered = true;
    }

    fn get_infection_chance(&self) -> i32 {
        if self.is_infected {
            let infection_chance = (((self.virus.r_naught as f32 /(self.virus.life_span as f32)) / 2 as f32) * 100 as f32) as i32;
            return infection_chance
        }
        0
    }

    fn handle(&mut self)
    {
        self.days_since_infected += 1;
        if self.days_since_infected > self.virus.life_span {
            self.recover()
        }
    }

    fn go_to_mall(&self) -> bool
    {
        true
    }

}

#[derive(Debug, Clone)]
struct Building
{
    id: i32,
    capacity: i32,           
    people_inside: Vec<Person>,
}

impl Building
{
    fn add(&mut self, person: Person)
    {
        self.people_inside.push(person);
    }

    fn infect_random(&mut self, virus: Virus) {
        let mut non_infected = vec![];
        let mut ind = -1;
        for person in &self.people_inside {
            ind += 1;
            if !person.is_infected && !person.is_recovered {
                non_infected.push(ind)
            }
        }

        if !non_infected.is_empty(){
            &self.people_inside[non_infected[rand_range(0, non_infected.len() as i32) as usize] as usize].infect(virus);
        }
    }

    fn infect(&mut self)
    {
        let mut times_to_infect = 0;
        let mut current_virus = Virus{ r_naught: 0, life_span: 0 };
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

            if person.is_infected {
                current_virus = person.virus.clone();
                let mut chance = person.get_infection_chance();
                while chance > 100 {
                    chance -= 100;
                    times_to_infect += 1;
                }
                if rand_prob(chance){
                    times_to_infect += 1;
                }
            }
        }
        for _ in 0..times_to_infect
        {
            self.infect_random(current_virus);
        }
    }
}


fn main()  {
    // init
    let mut has_args = false;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        assert_eq!(args.len(), 4, "args is not of length 3");
        has_args = true;
    }


    let mut houses: Vec<Building> = vec![];
    let mut malls: Vec<Building> = vec![];
    let mut remain = 10000;
    let mut days = 0;

    let mut house_id = 0;
    let mut ids = 0;

    let mut population = 0;
    let mut pop_infected = 0;
    let mut pop_healthy = 0;
    let mut pop_recovered = 0;
    let mut threshold = 5;

    let mut data: Vec<Vec<i32>> = vec![];

    let mut virus = Virus{ r_naught: 2, life_span: 40 };

    if has_args {
        remain = args[1].parse::<i32>().unwrap();
        virus = Virus {
            r_naught: args[2].parse::<i32>().unwrap(),
            life_span: args[3].parse::<i32>().unwrap()
        };
    }
    println!("Population: {}\nVirus: {:?}\n", remain, virus);

    for i in 1..11 {
        malls.push(Building{id: i, capacity: 100, people_inside: vec![]})
    }

    while remain > 0
    {
        house_id += 1;
        // let mut res_prob = rand_number_increase_prob(100, 10);
        let mut res_prob = 5;
        if res_prob > remain { res_prob = remain }

        let mut new_house = Building {id: house_id, people_inside: vec![], capacity: res_prob };
        remain -= res_prob;

        for _i in 1..(res_prob + 1) {
            ids += 1;
            new_house.add(Person::person(ids, house_id));
        }
        houses.push(new_house);
    }
    vec_shuffle(&mut houses);

    for i in 0..5 {
        houses[i].people_inside[0].infect(virus.clone());
    }
    // end init

    while threshold > 0 {
        days += 1;
        threshold -= 1;
        population = 0;
        pop_infected = 0;
        pop_healthy = 0;
        pop_recovered = 0;
        for house in houses.clone() {
            for person in &house.people_inside {
                population += 1;
                if person.is_infected {
                    pop_infected += 1;
                } else if person.is_recovered {
                    pop_recovered += 1;
                } else {
                    pop_healthy += 1;
                }
            }
        }
        if pop_infected > 0 {
            threshold = 5;
        }
        data.push(vec![population, pop_healthy, pop_infected, pop_recovered]);
        println!("Day: {}\nPopulation: {}\nHealthy: {}\nInfected: {} \nRecovered: {}\n", days, population, pop_healthy, pop_infected, pop_recovered);
        // ready

        let mut new_houses = vec![];
        for mut house in houses {
            house.infect();
            new_houses.push(house);
        }
        houses = new_houses;

        houses = houses.into_iter().map(|house| {
            Building {
                id: house.id,
                capacity: house.capacity,
                people_inside: house
                    .people_inside
                    .into_iter()
                    .filter_map(|mut pers| {
                        if pers.is_infected {
                            pers.handle();
                        }
                        if pers.go_to_mall() {
                            let mut temp = rand_number_increase_prob(100,20) as usize;
                            if temp > malls.len(){
                                temp = malls.len() - 1;
                            }
                            malls[temp].people_inside.push(pers);
                            None
                        } else {
                            Some(pers)
                        }
                    })
                    .collect()
            }
        }).collect();


        let mut new_malls = vec![];
        for mut mall in malls {
            mall.infect();
            new_malls.push(mall);
        }
        malls = new_malls;

        malls = malls.into_iter().map(|mall| {
            Building {
                id: mall.id,
                capacity: mall.capacity,
                people_inside: mall
                    .people_inside
                    .into_iter()
                    .filter_map(|pers| {
                        let mut ind: i32 = -1;
                        for house in &houses {
                            ind += 1;
                            if pers.house_id == house.id
                            {
                                break;
                            }
                        }
                        houses[ind as usize].people_inside.push(pers);
                        None
                    })
                    .collect()
            }
        }).collect();
    }
    let mut s = "".to_string();
    for ve in data {
        for v in ve {
            s += &format!("{} ", v);
        }
        s += "\n";
    }
    fs::write("out/data.txt", s);
}