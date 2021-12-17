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
    while(binary[index] == '1'):
        index += 5
    index += 5
    return index


def read_operator(index, binary):
    i = binary[index: index + 1]
    index += 1
    versions = []
    if(i == '0'):
        bits = int(binary[index:index + 15], 2)
        index += 15
        final_index = index + bits
        while(index < final_index):
            packet_versions, ptype, index = read_packet(index, binary)
            versions += packet_versions
    else:
        number_of_p = int(binary[index:index + 11], 2)
        index += 11
        for i in range(0, number_of_p):
            packet_versions, ptype, index = read_packet(index, binary)
            versions += packet_versions

    return versions, index


def read_packet(index, binary):
    versions = []
    version = int(binary[index:index + 3], 2)
    versions.append(version)
    index += 3
    ptype = int(binary[index:index + 3], 2)
    index += 3
    if(ptype == 4):
        index = read_litteral(index, binary)
    else:
        packet_versions, index = read_operator(index, binary)
        versions += packet_versions
    return versions, ptype, index


def count_versions(binary):
    versions = []
    index = 0
    while(index < len(binary) - 7):
        packet_versions, ptype, index = read_packet(index, binary)
        versions += packet_versions

    return sum(versions)
