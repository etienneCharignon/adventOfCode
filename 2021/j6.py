from inputj6 import input
# from samplej6 import input
from j6_lanternfish import next_generation

input = input.split(',')
input = list(map(int, input))

for generation in range(0, 80):
    input = next_generation(input)

print(len(input))
