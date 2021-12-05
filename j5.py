from inputj5 import input
# from samplej5 import input

diag = []
for row in input.split('\n'):
    print(row)
    from_pos, to_pos = row.split(' -> ')
    fxstr, fystr = from_pos.split(',')
    txstr, tystr = to_pos.split(',')
    fx = int(fxstr)
    fy = int(fystr)
    tx = int(txstr)
    ty = int(tystr)
    if(fx == tx):
        if(ty > fy):
            y_range = range(fy, ty + 1)
        else:
            y_range = range(ty, fy + 1)
        for y in y_range:
            diag.append((fx, y))
    if(fy == ty):
        if(tx > fx):
            x_range = range(fx, tx + 1)
        else:
            x_range = range(tx, fx + 1)
        for x in x_range:
            diag.append((x, fy))

print(len(diag) - len(set(diag)))

unique_points = set(diag)
for first in unique_points:
    diag.remove(first)

print(len(set(diag)))
