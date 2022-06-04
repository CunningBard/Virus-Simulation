use crate::{person, virus};
use crate::essential_functions::{rand_prob_, rand_range};
use crate::virus::Virus;

#[derive(Debug, Clone)]
pub(crate) struct Building
{
    pub(crate) id: i32,
    pub(crate) capacity: i32,
    pub(crate) people_inside: Vec<person::Person>,
}

impl Building
{
    pub(crate) fn add(&mut self, person: person::Person)
    {
        self.people_inside.push(person);
    }

    fn infect_random(&mut self, virus: virus::Virus) {
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

    pub(crate) fn infect(&mut self)
    {
        let mut times_to_infect = 0;
        let mut current_virus = Virus{ infection_chance: 0, life_span: 0 };
        let mut can_be_infected = 0;
        for person_ in &self.people_inside {
            if !person_.is_infected && !person_.is_recovered {
                can_be_infected += 1;
            }
        }
        if can_be_infected < 1 {
                return;
        }

        for person in &mut self.people_inside {
            if person.is_infected && !person.is_quarantined {
                if person.is_recovered {
                    assert!(false);
                }
                current_virus = person.virus.clone();
                let mut chance = current_virus.infection_chance;
                while chance > 100 {
                    chance -= 100;
                    times_to_infect += 1;
                    person.num_infected_people += 1;
                }
                if rand_prob_(chance, 1000){
                    times_to_infect += 1;
                    person.num_infected_people += 1;
                }
            }
        }

        for _ in 0..times_to_infect
        {
            self.infect_random(current_virus);
        }
    }
}