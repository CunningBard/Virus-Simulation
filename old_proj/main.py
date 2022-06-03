import sys
import matplotlib.pyplot as plt
import mymodule
import random

places = []
people = {}
peopleNum = 0
notInfectedNum = 0
infectedNum = 0
recoveredNum = 0
deathNum = 0
turn = 1

notInfected = []
infected = []
recovered = []
death = []
turns = []

la = False
ba = False
ra = True
za = 1000000000000

numOfPeople = 1000
environment = 20
envi = numOfPeople - 0

ed = random.randrange(1, numOfPeople)
edIsAlive = True
edMode = False

op = 0

lockdownLevel = 0
lastThing = 1
monthLast = 0

keep_plotting = True


class Virus:
    def __init__(self, daysTilRecovered, infectionRate, severity, Recovered=False):
        self.infectionProbability = 100 * (infectionRate / daysTilRecovered)
        self.recoveryDays = daysTilRecovered
        self.infectRecovered = Recovered
        self.severity = severity / daysTilRecovered


class Environment:
    def __init__(self, capacity: int = 5):
        self.capacity = capacity
        self.occupants = []
        self.all = []
        self.notInfected = []
        self.recovered = []
        self.infected = []
        self.infect = False
        self.infectedPlace = False
        self.infectCounter = 0
        self.lastVirus = None

    def Add(self, index):
        if len(self.occupants) < self.capacity:
            self.occupants.append(index)

    def Clear(self):
        self.occupants.clear()
        self.infected.clear()
        self.notInfected.clear()
        self.recovered.clear()
        self.all.clear()
        self.infect = False

    def CheckForInfected(self):
        if self.infectCounter > 0:
            self.infectCounter -= 1
        elif self.infectCounter <= 0:
            self.infectedPlace = False
        for occupant in self.occupants:
            if people[occupant].infected:
                self.infect = True
                self.infected.append(occupant)
                self.infectedPlace = True
                self.infectCounter = 6
                self.lastVirus = people[occupant].virus
            else:
                self.all.append(occupant)
                if people[occupant].recovered:
                    self.recovered.append(occupant)
                else:
                    self.notInfected.append(occupant)

    def Infect(self):
        for infect in self.infected:
            if people[infect].virus.infectionProbability > 100:
                prob = people[infect].virus.infectionProbability
                b = int(prob // 100)
                prob -= b * 100
                for _ in range(b):
                    self.InfectPeople(infect)
                if mymodule.randbool(prob):
                    self.InfectPeople(infect)
            else:
                if mymodule.randbool(people[infect].virus.infectionProbability):
                    self.InfectPeople(infect)

        if self.infectedPlace and mymodule.randbool(10) and len(self.all) > 0:
            toInfect = random.choice(self.all)
            if people[toInfect].recovered:
                self.all.remove(toInfect)
                self.recovered.remove(toInfect)
            else:
                self.all.remove(toInfect)
                self.notInfected.remove(toInfect)
            people[toInfect].GetInfected(self.lastVirus)

    def InfectPeople(self, infect):
        if people[infect].virus.infectRecovered:
            if len(self.all) > 0:
                toInfect = random.choice(self.all)
                if people[toInfect].recovered:
                    self.all.remove(toInfect)
                    self.recovered.remove(toInfect)
                else:
                    self.all.remove(toInfect)
                    self.notInfected.remove(toInfect)
                people[toInfect].GetInfected(people[infect].virus)
        else:
            if len(self.notInfected) > 0:
                toInfect = random.choice(self.notInfected)
                self.notInfected.remove(toInfect)
                people[toInfect].GetInfected(people[infect].virus)


class Person:
    def __init__(self):
        self.name = f"Person {peopleNum}"
        self.infected = False
        self.recovered = False
        self.alive = True
        self.virus = None
        self.virusCount = 0
        self.numRecovered = 0
        self.immunity = random.randrange(2, 4)
        self.listener = mymodule.randbool(50)
        self.foundInfected = False
        self.daySinceRecovery = 0

    def GoToPlace(self):
        a = 0
        if self.listener:
            a = 30
        if not mymodule.randbool(20 + a + (lockdownLevel * 10)) and not self.foundInfected:
            place = None
            random.shuffle(places)
            for place in places:
                if len(place.occupants) < place.capacity:
                    place = place
                    break
            if place:
                place.Add(self.name)

    def GetInfected(self, virus: Virus):
        global infectedNum
        global notInfectedNum
        global recoveredNum
        if self.numRecovered < self.immunity and self.alive:
            self.infected = True
            self.virus = virus
            infectedNum += 1
            if self.recovered:
                recoveredNum -= 1
                self.virusCount = 0
                self.recovered = False
            else:
                notInfectedNum -= 1
        #   print(self.name, "has been infected")

    def Recovered(self):
        global infectedNum
        global recoveredNum
        self.infected = False
        self.recovered = True
        self.numRecovered += 1
        recoveredNum += 1
        infectedNum -= 1
        self.foundInfected = False
        #   print(self.name, "has recovered")

    def Die(self):
        global recoveredNum
        global infectedNum
        global deathNum
        global edIsAlive
        if self.recovered:
            recoveredNum -= 1
        if self.infected:
            infectedNum -= 1
        deathNum += 1
        del people[self.name]
        self.alive = False

    def Main(self):
        global notInfectedNum
        global recoveredNum
        if self.infected and self.alive:
            if self.virus.recoveryDays <= self.virusCount:
                self.Recovered()
            else:
                self.virusCount += 1
            if (self.virus.recoveryDays // 2) // 2 < self.virusCount:
                if mymodule.randbool(self.virus.severity):
                    self.Die()
            if mymodule.randbool(infectedNum // 10):
                self.foundInfected = True

    def Info1(self):
        print(self.name)
        print(f"Alive: {self.alive}")
        print(f"Infected: {self.infected}")
        print(f"Recovered: {self.recovered}")
        print()


def Info(s=False):
    if s:
        pass
    else:
        print(f"People: {peopleNum}")
        print(f"Not Infected: {notInfectedNum}")
        print(f"Infected: {infectedNum}")
        print(f"Recovered: {recoveredNum}")
        print(f"Dead: {deathNum}\n")


def GetData():
    infected.append(infectedNum)
    notInfected.append(notInfectedNum)
    recovered.append(recoveredNum)
    death.append(deathNum)
    turns.append(turn)


def GoToPlace():
    peoples = list(people)
    random.shuffle(peoples)
    for person in peoples:
        people[person].GoToPlace()


def ClearPlaces():
    for place in places:
        place.Clear()


def CheckForInfected():
    for place in places:
        place.CheckForInfected()


def Infect():
    for place in places:
        place.Infect()


def MainThing():
    global lastThing
    global lockdownLevel
    global monthLast
    ClearPlaces()
    GoToPlace()
    CheckForInfected()
    Infect()
    for person in people.copy():
        people[person].Main()
    if infectedNum + recoveredNum > lastThing * int(peopleNum / 2):
        lockdownLevel += 1
        lastThing += 1
    if int(turn / 30) > monthLast:
        monthLast += 1
        for _ in range(int(numOfPeople / 1000)):
            new_person()
        if mymodule.divisible_by(monthLast, 2):
            a = random.choice(list(people))
            people[a].Die()


def AllFree():
    for person in people:
        if people[person].infected:
            return False
    return True


def new_person():
    global peopleNum
    global notInfectedNum
    notInfectedNum += 1
    peopleNum += 1
    people[f"Person {peopleNum}"] = Person()


def updating_graph(fig):
    fig.clear()
    ax = fig.add_subplot(1, 1, 1)
    ax.stackplot(turns, [infected,   recovered, death,   notInfected],
                 labels=[f"Infected({infectedNum})", f"Recovered({recoveredNum})", f"Dead({deathNum})",
                         f"Healthy({notInfectedNum})"])
    ax.set_xlabel(f'Day: {turns[-1]} ({lockdownLevel})[{notInfectedNum + recoveredNum + infectedNum} / '
                  f'{peopleNum}]', fontsize=14)
    fig.legend()
    fig.canvas.draw()
    fig.canvas.flush_events()


def show_graph():
    plt.stackplot(turns, [infected,   recovered, death,   notInfected],
                  labels=[f"Infected({infectedNum})", f"Recovered({recoveredNum})", f"Dead({deathNum})",
                          f"Healthy({notInfectedNum})"])
    plt.xlabel(f'Day: {turns[-1]} ({lockdownLevel})[{notInfectedNum + recoveredNum + infectedNum} / '
               f'{peopleNum}]', fontsize=14)
    plt.legend(loc="upper left")
    plt.show()


def create_environment():
    global envi
    for _ in range(environment):
        places.append(Environment(5))
        envi -= 5

    for _ in range(envi):
        random.choice(places).capacity += 1


def on_key(event):
    if event:
        pass
    global keep_plotting
    keep_plotting = False


def main():
    global turn
    global la
    global ba
    global op
    global za
    global ra
    run = True
    create_environment()
    for _ in range(numOfPeople):
        new_person()
    #   daysTilRecovery = input("Days Until Recovery: ")
    #   if not daysTilRecovery == "-":
    #       RNaught = input("Virus R-Naught: ")
    #       severity = input("Severity: ")
    #       virus = Virus(float(daysTilRecovery), float(RNaught), float(severity), True)
    #   else:
    #       virus = Virus(14, 2.5, 5, True)
    virus = Virus(14, 2.5, 5, True)
    a = random.choice(list(people))
    people[a].GetInfected(virus)
    target_turn = 0
    fig = None
    while run:
        if target_turn < turn:
            fig = None
            i = input(f": ")
            i = i.strip()
            print()
            if i == "-":
                target_turn = 1000000
                ba = True
            elif i.startswith("-"):
                target_turn = int(i.replace("-", ""))
                ba = True
            elif i.startswith("p"):
                targetPerson = i.replace("p", "")
                if f"Person {targetPerson}" in people:
                    print("Yes")
                    print(f"Person{targetPerson}")
                else:
                    print("No")
                    print(f"Person{targetPerson}")
            elif i == "":
                print(f"Days {turn}: ")
                print()
                turn += 1
                MainThing()
                Info()
                GetData()
                if AllFree() and turn > 20 and not la:
                    show_graph()
                    la = True
            elif i == "1":
                show_graph()
            elif i == "2":
                for place in places:
                    print(place.occupants)
        else:
            if ba:
                ba = False
                plt.ion()
                fig = plt.figure()
                plt.ioff()
                input(": ")
            print(f"Days {turn}: ")
            print()
            turn += 1
            MainThing()
            Info()
            GetData()
            updating_graph(fig)
            if infectedNum == 0 and ra:
                ra = False
                za = turn + 14
            if turn >= za:
                while keep_plotting:
                    updating_graph(fig)
                    fig.canvas.mpl_connect('close_event', on_key)
                target_turn = 0
            if not edIsAlive and edMode:
                while op < 100:
                    updating_graph(fig)
                    op += 1


if __name__ == '__main__':
    main()