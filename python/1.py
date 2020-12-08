nums = list(map(int, open('../inputs/1')))

def part1() -> int:
    for x in nums:
        for y in nums:
            if x + y == 2020:
                return x * y

print(part1())

def part2() -> int:
    for x in nums:
        for y in nums:
            for z in nums:
                if x + y + z == 2020:
                    return x * y * z

print(part2())
