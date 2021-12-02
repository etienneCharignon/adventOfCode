from inputj2 import input

commands = input.split('\n')

x = 0
d = 0
for command in commands:
    command, value = command.split(' ')
    distance = int(value)
    if(command == 'forward'):
        x += distance
    elif(command == 'up'):
        d -= distance
    elif(command == 'down'):
        d += distance

print(x)
print(d)
print(x * d)
