from inputj6 import input
# from samplej6 import input
from j6_lanternfish import next_generation, init_world

world = init_world(input)

for generation in range(0, 256):
    world = next_generation(world)
    print(world)

print(sum(world.values()))
