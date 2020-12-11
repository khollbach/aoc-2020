from collections import Counter
from typing import List

def main():
    nums = list(map(int, open('../inputs/10')))
    nums.append(0)  # Charging outlet.
    nums.sort()
    nums.append(nums[-1] + 3)  # Device.

    print(part1(nums))
    print(part2(nums))

def part1(nums: List[int]) -> int:
    def sub(pair) -> int:
        a, b = pair
        return a - b

    # (n1 - n0), (n2 - n1), (n3 - n2), etc...
    diffs = map(sub, zip(nums[1:], nums[:-1]))

    freqs = Counter(diffs)
    assert max(freqs) <= 3
    assert 0 not in freqs  # nums should be unique for part 2.
    return freqs[1] * freqs[3]

def part2(nums: List[int]) -> int:
    # `num_paths[i]` is the # of unique ways to descend
    # from nums[i] down to nums[0] according to the rules.
    num_paths = [None] * len(nums)

    num_paths[0] = 1

    for i in range(1, len(nums)):
        total = 0
        for di in (1, 2, 3):
            j = i - di
            if j >= 0 and nums[i] - nums[j] <= 3:
                total += num_paths[j]
        num_paths[i] = total

    return num_paths[-1]

main()
