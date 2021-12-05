from inputj3 import input

diag = input.split('\n')


def counters(diag):
    counters = []
    for i in range(len(diag[0])):
        counters.append([0, 0])

    for binary in diag:
        for j, bit in enumerate(binary):
            if(bit == "0"):
                counters[j][0] += 1
            else:
                counters[j][1] += 1
    return counters


print(counters(diag))


def mapGamma(counter):
    if counter[0] > counter[1]:
        return "0"
    else:
        return "1"


def mapEpsylon(counter):
    if counter[0] <= counter[1]:
        return "0"
    else:
        return "1"


gamma = int("".join(map(mapGamma, counters(diag))), 2)
epsylon = int("".join(map(mapEpsylon, counters(diag))), 2)
print(gamma)
print(epsylon)
print(gamma*epsylon)


print("Partie 2")


def search(binaries, i, criteria):
    if(len(binaries) == 1):
        return binaries[0]

    mostCommon = criteria(counters(binaries)[i])
    selectedBinaries = [b for b in binaries if b[i] == mostCommon]
    return search(selectedBinaries, i+1, criteria)


oxygen = int(search(diag, 0, mapGamma), 2)
co2 = int(search(diag, 0, mapEpsylon), 2)

print(oxygen * co2)
