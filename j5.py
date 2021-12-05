from inputj5 import input
# from samplej5 import input
from j5_hydrothermal_vent import direction


diag = []
for row in input.split('\n'):
    # print(row)
    from_pos, to_pos = row.split(' -> ')
    fx, fy = map(int, from_pos.split(','))
    tx, ty = map(int, to_pos.split(','))

    x_dir = direction(fx, tx)
    y_dir = direction(fy, ty)
    if(x_dir > 0):
        x_range = range(fx, tx + 1, x_dir)
    else:
        x_range = range(fx, tx - 1, x_dir)

    if(y_dir > 0):
        y_range = range(fy, ty + 1, y_dir)
    else:
        y_range = range(fy, ty - 1, y_dir)

    if(fx == tx):
        for y in y_range:
            diag.append((fx, y))
    elif(fy == ty):
        for x in x_range:
            diag.append((x, fy))
    else:
        distance = len(x_range)
        for i in range(0, distance):
            diag.append((fx + (i * x_dir), fy + (i * y_dir)))

    # print(diag)

# duplicates = {i:diag.count(i) for i in diag}

unique_points = set(diag)
print("unique points")
for first in unique_points:
    diag.remove(first)
print("removed")

print(len(set(diag)))
