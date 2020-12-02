numbers = []
with open("inputs/input-1", 'r') as f:
    r = f.readline()
    while r != "":
        numbers.append(int(r))
        r = f.readline()

for num1 in numbers:
    for num2 in numbers:
        for num3 in numbers:
            if num1 + num2 + num3 == 2020:
                print(num1)
                print(num2)
                print(num3)
                print(num1 * num2 * num3)
                exit(0)