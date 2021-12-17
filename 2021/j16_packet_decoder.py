import numpy

hex_map = {
    '0': '0000',
    '1': '0001',
    '2': '0010',
    '3': '0011',
    '4': '0100',
    '5': '0101',
    '6': '0110',
    '7': '0111',
    '8': '1000',
    '9': '1001',
    'A': '1010',
    'B': '1011',
    'C': '1100',
    'D': '1101',
    'E': '1110',
    'F': '1111'
}


def binary_stream(hex_string):
    def to_bin(h):
        return hex_map[h]

    return ''.join(map(to_bin, hex_string))


def read_litteral(index, binary):
    litteral = ''
    while(binary[index] == '1'):
        litteral += binary[index+1:index+5]
        index += 5
    litteral += binary[index+1:index+5]
    index += 5
    return int(litteral, 2), index


def compute_operator(operator, values):
    if(operator == 0):  # sum
        return sum(values)
    elif(operator == 1):  # product
        return numpy.prod(values)
    elif(operator == 2):  # min
        return min(values)
    elif(operator == 3):  # max
        return max(values)
    elif(operator == 5):  # greater than
        if(values[0] > values[1]):
            return 1
        else:
            return 0
    elif(operator == 6):  # less than
        if(values[0] < values[1]):
            return 1
        else:
            return 0
    elif(operator == 7):  # equal
        if(values[0] == values[1]):
            return 1
        else:
            return 0


def read_operator(operator, index, binary):
    i = binary[index: index + 1]
    index += 1
    if(i == '0'):
        bits = int(binary[index:index + 15], 2)
        index += 15
        final_index = index + bits
        values = []
        while(index < final_index):
            value, ptype, index = read_packet(index, binary)
            values.append(value)
    else:
        number_of_p = int(binary[index:index + 11], 2)
        index += 11
        values = []
        for i in range(0, number_of_p):
            value, ptype, index = read_packet(index, binary)
            values.append(value)

    value = compute_operator(operator, values)
    return value, index


def read_packet(index, binary):
    # version = int(binary[index:index + 3], 2)
    index += 3
    ptype = int(binary[index:index + 3], 2)
    index += 3
    if(ptype == 4):
        value, index = read_litteral(index, binary)
    else:
        value, index = read_operator(ptype, index, binary)
    return value, ptype, index


def calculate(binary):
    index = 0
    while(index < len(binary) - 7):
        value, ptype, index = read_packet(index, binary)
    return value
