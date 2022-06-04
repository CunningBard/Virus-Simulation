use crate::virus;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Person
{
    pub(crate) id: i32,
    pub(crate) house_id: i32,
    pub(crate) is_infected: bool,
    pub(crate) is_recovered: bool,
    pub(crate) is_quarantined: bool,
    pub(crate) days_since_infected: i32,
    pub(crate) num_infected_people: i32,
    pub(crate) virus: virus::Virus
}

impl Person
{
    pub(crate) fn person(id: i32, house_id: i32) -> Person {
        Person
        {
            id,
            house_id,
            is_infected: false,
            is_recovered: false,
            is_quarantined: false,
            days_since_infected: 0,
            num_infected_people: 0,
            virus: virus::Virus {infection_chance: 0, life_span: 0}
        }
    }
    pub(crate) fn infect(&mut self, virus: virus::Virus) {
        self.is_infected = true;
        self.virus = virus;
    }
    fn recover(&mut self) {
        assert!(self.is_infected, "how the fuck can an non infected recover");

        self.is_infected = false;
        self.is_recovered = true;
    }

    pub(crate) fn handle(&mut self)
    {
        if self.is_infected {
            self.days_since_infected += 1;
            if self.days_since_infected > self.virus.life_span {
                self.recover()
            }
        }
    }

    pub(crate) fn go_to_mall(&self) -> bool
    {
        if self.is_quarantined {
            false
        } else {
            true
        }
    }

}