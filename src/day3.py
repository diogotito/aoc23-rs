import re
from collections import namedtuple, defaultdict
from itertools import tee

Neighbour = namedtuple('Neighbour', ['pos', 'char'])

def neighbouring_positions(row, col_start, col_end):
    yield from ((row - 1, c) for c in range(col_start - 1, col_end + 1))
    yield (row, col_start - 1)
    yield (row, col_end)
    yield from ((row + 1, c) for c in range(col_start - 1, col_end + 1))
    

schematic = [line.rstrip() for line in open("../input/day3.txt")]
padded = (
    ['.' * (len(schematic[0]) + 2)] +
    ['.' + line + '.' for line in schematic] + 
    ['.' * (len(schematic[0]) + 2)]
)

parts_sum = 0               # Part 1
gears = defaultdict(list)   # Part 2

for r, line in enumerate(schematic):
    for match in re.finditer(r'\d+', line):
        part_number = int(match.group())
        ns1, ns2 = tee(Neighbour(pos=(r + 1, c + 1), char=padded[r + 1][c + 1])
                       for r, c in neighbouring_positions(r, *match.span()))

        # For part 1
        if any(n.char not in '0123456789.' for n in ns1):
            parts_sum += part_number

        # For part 2
        for n in ns2:
            if n.char == '*':
                gears[n.pos].append(part_number)


gear_ratios = sum(parts[0] * parts[1]
                  for parts in gears.values() if len(parts) == 2)

print("Part 1:", parts_sum)
print("Part 2:", gear_ratios)

assert(parts_sum == 532428)
assert(gear_ratios == 84051670)
