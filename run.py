import subprocess
import matplotlib.pyplot as plt

subprocess.run(["cargo", "run", "--release", "10000", "40", "14"])
# population, infection(30 is 3%), life span

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

# numbers = []
# nums = []
# times = 0
# for infectedd in infected:
#     times += 1
#     nums.append(infectedd)
#     if times == 3:
#         times = 0
#         numbers.append(nums)
#         nums = []
#
# print(numbers)


plt.stackplot(days, infected, recovered, healthy, labels=['Infected', 'Recovered', 'Healthy'])
plt.legend(loc='upper left')
plt.show()
