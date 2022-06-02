import subprocess

import matplotlib.pyplot as plt


subprocess.run("./target/release/VirusThing")

days = []
population = []
healthy = []
infected = []
recovered = []
with open("out/data.txt", "r") as f:
    a = f.readlines()

day = 1
for line in a:
    line.replace("\n", "")
    line = line.split(" ")
    day += 1
    days.append(day)
    population.append(int(line[0]))
    healthy.append(int(line[1]))
    infected.append(int(line[2]))
    recovered.append(int(line[3]))


# Create data
x=range(1,6)
y1=[1,4,6,8,9]
y2=[2,2,7,10,12]
y3=[2,8,5,10,6]

# Basic stacked area chart.
plt.stackplot(days, healthy, infected, recovered, labels=['Healthy', 'Infected', 'Recovered'])
plt.legend(loc='upper left')
plt.show()