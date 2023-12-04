import re
from collections import namedtuple
from dataclasses import dataclass
from itertools import tee


Neighbour = namedtuple('Neighbour', ['pos', 'char'])


@dataclass
class Gear:
    r: int
    c: int
    parts: list[int]

    def ratio(self):
        return self.parts[0] * self.parts[1] if len(self.parts) == 2 else 0


# Puzzle input

schematic = [line.rstrip() for line in open("../input/day3.txt")]
padded = ['.' * (len(schematic[0]) + 2),
          *('.' + line + '.' for line in schematic),
          '.' * (len(schematic[0]) + 2)]

def neighbouring_positions(row, col_start, col_end):
    yield from ((row - 1, c) for c in range(col_start - 1, col_end + 1))
    yield (row, col_start - 1)
    yield (row, col_end)
    yield from ((row + 1, c) for c in range(col_start - 1, col_end + 1))
    

parts_sum = 0
gears = {}

for r, line in enumerate(schematic):
    for match in re.finditer(r'\d+', line):
        part_number = int(match.group())
        ns1, ns2 = tee(Neighbour(pos=(r + 1, c + 1), char=padded[r + 1][c + 1])
                       for r, c in neighbouring_positions(r, *match.span()))

        if any(n.char not in '0123456789.' for n in ns1):
            parts_sum += part_number

        for n in ns2:
            if n.char == '*':
                if n.pos not in gears:
                    gears[n.pos] = Gear(*n.pos, [part_number])
                else:
                    gears[n.pos].parts.append(part_number)


gear_ratios = sum(gear.ratio() for gear in gears.values())

print("Part 1:", parts_sum)
print("Part 2:", gear_ratios)

# Part 1 = 532428
# Part 2 = 84051670
