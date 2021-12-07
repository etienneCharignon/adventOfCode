# from samplej7 import position
from inputj7 import position


def calculate_median(positions):
    positions = sorted(positions)
    numberOfCrabs = len(positions)
    if numberOfCrabs < 1:
        return None
    if numberOfCrabs % 2 == 0:
        print("paire")
        return int((positions[int((numberOfCrabs-1)/2)] + positions[int((numberOfCrabs+1)/2)]) / 2.0)
    else:
        print("impaire")
        return positions[int((numberOfCrabs-1)/2)]


positions = list(map(int, position.split(',')))
median = calculate_median(positions)


def distance(position):
    return abs(position - median)


print(sum(map(distance, positions)))
