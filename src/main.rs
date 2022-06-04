mod virus;
mod essential_functions;
mod person;
mod building;

use std::{fs, vec};
use std::time::SystemTime;
use std::env;
use crate::building::Building;
use crate::essential_functions::rand_range;


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

    let mut virus = virus::Virus{ infection_chance: 1, life_span: 40 };

    if has_args {
        remain = args[1].parse::<i32>().unwrap();
        virus = virus::Virus {
            infection_chance: args[2].parse::<i32>().unwrap(),
            life_span: args[3].parse::<i32>().unwrap()
        };
    }
    println!("Population: {}\nVirus: {:?}\n", remain, virus);

    for i in 1..101 {
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
            new_house.add(person::Person::person(ids, house_id));
        }
        houses.push(new_house);
    }

    for i in 0..10 {
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

        for house in &mut houses {
            house.infect();
        }

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
                            let mut b = rand_range(0, malls.len() as i32);
                            malls[b as usize].add(pers);
                            None
                        } else {
                            Some(pers)
                        }
                    })
                    .collect()
            }
        }).collect();

        for mut mall in &mut malls {
            mall.infect();
        }

        malls = malls.into_iter().map(|mall| {
            Building {
                id: mall.id,
                capacity: mall.capacity,
                people_inside: mall
                    .people_inside
                    .into_iter()
                    .filter_map(|pers| {
                        houses[(pers.house_id - 1) as usize].people_inside.push(pers);
                        None
                    })
                    .collect()
            }
        }).collect();
    }
    let mut were_infected = 0;
    let mut num_infected = 0;
    for house in houses.clone() {
        for person in &house.people_inside {
            if person.is_recovered && person.num_infected_people > 0 {
                were_infected += 1;
                num_infected += person.num_infected_people;
            }
        }
    }
    println!("{} {}", were_infected, num_infected);
    let mut s = "".to_string();
    for ve in data {
        for v in ve {
            s += &format!("{} ", v);
        }
        s += "\n";
    }
    fs::write("out/data.txt", s).expect("WARNING: FAILED TO WRITE");
}