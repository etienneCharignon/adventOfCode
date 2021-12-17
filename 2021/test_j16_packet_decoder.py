from inputj16 import input
from j16_packet_decoder import read_operator, binary_stream, count_versions, read_litteral


def test_binary_stream():
    assert binary_stream('D2FE28') == '110100101111111000101000'
    assert binary_stream('38006F45291200') == '00111000000000000110111101000101001010010001001000000000'


def test_count_versions():
    assert count_versions(binary_stream('D2FE28')) == 6
    assert count_versions(binary_stream('38006F45291200')) == \
        1 + int('110', 2) + int('010', 2)
    assert count_versions(binary_stream('8A004A801A8002F478')) == 16
    assert count_versions(binary_stream('A0016C880162017C3686B18A3D4780')) == 31
    assert count_versions(binary_stream(input)) == 974


def test_read_literal():
    assert read_litteral(0, '101111111000101000') == 15


def test_read_operator():
    assert read_operator(0, '0 000000000011011 11010001010 0101001000100100'.replace(' ', ''))[0] == [
        int('110', 2), int('010', 2)
    ]

    assert read_operator(0, '1 00000000011 01010000001 10010000010 00110000011'.replace(' ', ''))[0] == [
        int('010', 2), int('100', 2), int('001', 2)
    ]
