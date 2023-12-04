import re
from collections import namedtuple
from dataclasses import dataclass
from itertools import chain, tee
from pprint import pprint


# Data structures

Neighbour = namedtuple('Neighbour', ['r', 'c', 'char'])

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


# Part 1 and 2

def neighbours(s, r, c):
    return (Neighbour(nr, nc, s[nr][nc]) for nr, nc in
            ((r - 1, c - 1),  (r - 1, c),  (r - 1, c + 1), 
             (r    , c - 1),               (r    , c + 1), 
             (r + 1, c - 1),  (r + 1, c),  (r + 1, c + 1)))

parts_sum = 0
gears = {}

for r, line in enumerate(schematic):
    for match in re.finditer(r'\d+', line):
        part_number = int(match.group())
        all_neighbours = chain.from_iterable(neighbours(padded, r + 1, col + 1)
                                             for col in range(*match.span()))
        ns1, ns2 = tee(all_neighbours)

        if any(n.char not in '0123456789.' for n in ns1):
            parts_sum += part_number

        for n in ns2:
            if n.char == '*':
                if (n.r, n.c) not in gears:
                    gears[n.r, n.c] = Gear(n.r, n.c, [part_number])
                else:
                    gears[n.r, n.c].parts.append(part_number)


gear_ratios = sum(gear.ratio() for gear in gears.values())

print("Part 1:", parts_sum)
print("Part 2:", gear_ratios)

# 39591044: too low
