import sys

map = []
x = 0
y = 0

for line in sys.stdin:
        splitlist = list(line)
        print(splitlist[0])
        if "\n" not in splitlist[0]:
            y += 1
            for space in line:
                if "\n" not in space:
                    map.append(space)
                elif x == 0:
                    x = len(map)
                #print(space)
#index = ((map_check.y*32)+map_check.x)
print(map)
print(x,"x",y)


