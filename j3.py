from inputj3 import input

diag = input.split('\n')

counters = []
for i in range(len(diag[0])):
    counters.append([0, 0])

for binary in diag:
    for j, bit in enumerate(binary):
        print(j)
        if(bit == "0"):
            counters[j][0] += 1
        else:
            counters[j][1] += 1


print(counters)


def mapGamma(counter):
    if counter[0] > counter[1]:
        return "0"
    else:
        return "1"


def mapEpsylon(counter):
    if counter[0] < counter[1]:
        return "0"
    else:
        return "1"


gamma = int("".join(map(mapGamma, counters)), 2)
epsylon = int("".join(map(mapEpsylon, counters)), 2)
print(gamma)
print(epsylon)
print(gamma*epsylon)
