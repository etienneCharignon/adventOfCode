import time
from samplej11 import rinput
from test_j11_dumbo_octopuses import flash, parse, increase, print_zeros

print("starting")
field = parse(rinput)
count = 0
for step in range(0, 346):
    print(step)
    field = increase(field)
    field, flashes = flash(field)
    print(print_zeros(field))
    time.sleep(0.1)
