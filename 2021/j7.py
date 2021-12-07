# from samplej7 import position
from inputj7 import position


def calculate_median(positions):
    numberOfCrabs = len(positions)
    if numberOfCrabs < 1:
        return None
    if numberOfCrabs % 2 == 0:
        return int((positions[int((numberOfCrabs-1)/2)] + positions[int((numberOfCrabs+1)/2)]) / 2.0)
    else:
        return positions[int((numberOfCrabs-1)/2)]


def calcul_fuel(destination):
    def fuel(position):
        return sum(range(1, abs(position - destination) + 1))

    fuel = (sum(map(fuel, positions)))
    return fuel


positions = list(map(int, position.split(',')))
positions = sorted(positions)
median = calculate_median(positions)

min_fuel = calcul_fuel(0)
print(min_fuel)
for target in range(1, len(positions)):
    fuel = calcul_fuel(target)
    print(fuel)
    if(fuel < min_fuel):
        min_fuel = fuel
    else:
        print(min_fuel)
        break
