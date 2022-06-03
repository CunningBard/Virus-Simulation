import subprocess
import matplotlib.pyplot as plt


subprocess.run(["./target/release/VirusThing", "10000", "3", "19"])


days = []
population = []
healthy = []
infected = []
recovered = []
with open("out/data.txt", "r") as f:
    data = f.readlines()

day = 1
for line in data:
    line.replace("\n", "")
    line = line.split(" ")
    day += 1
    days.append(day)
    population.append(int(line[0]))
    healthy.append(int(line[1]))
    infected.append(int(line[2]))
    recovered.append(int(line[3]))


plt.stackplot(days, infected, recovered, healthy, labels=['Infected', 'Recovered', 'Healthy'])
plt.legend(loc='upper left')
plt.show()
