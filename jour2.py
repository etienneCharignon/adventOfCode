from inputj2 import input

commands = input.split('\n')

x = 0
aim = 0
d = 0
for command in commands:
    command, value = command.split(' ')
    distance = int(value)
    if(command == 'forward'):
        x += distance
        d += distance * aim
    elif(command == 'up'):
        aim -= distance
    elif(command == 'down'):
        aim += distance

print(x)
print(d)
print(x * d)
