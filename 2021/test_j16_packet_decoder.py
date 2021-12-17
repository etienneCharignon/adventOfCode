from inputj16 import input
from j16_packet_decoder import read_operator, binary_stream, read_litteral, calculate


def test_binary_stream():
    assert binary_stream('D2FE28') == '110100101111111000101000'
    assert binary_stream('38006F45291200') == '00111000000000000110111101000101001010010001001000000000'


def test_read_literal():
    assert read_litteral(0, '101111111000101000') == (2021, 15)


def test_read_operator():
    assert read_operator(6, 0, '0 000000000011011 11010001010 0101001000100100'.replace(' ', ''))[0] == 1
    assert read_operator(2, 0, '1 00000000011 01010000001 10010000010 00110000011'.replace(' ', ''))[0] == 1


def test_calculate():
    assert calculate(binary_stream('C200B40A82')) == 3
    assert calculate(binary_stream(input)) == 180616437720
