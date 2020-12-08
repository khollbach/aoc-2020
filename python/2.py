import re
from typing import Callable

Policy = Callable[[int, int, str, str], bool]

def count_valid(policy: Policy) -> int:
    num_valid = 0
    for line in open('../inputs/2'):
        lo, hi, c, p = re.match(r'^(\d+)-(\d+) (.): (.*)$', line).groups()
        num_valid += policy(int(lo), int(hi), c, p)
    return num_valid

def policy_1(lo, hi, c, p):
    count = len(list(filter(lambda a: a == c, p)))
    return lo <= count <= hi

def policy_2(lo, hi, c, p):
    return (p[lo-1] == c) ^ (p[hi-1] == c)

print(count_valid(policy_1))
print(count_valid(policy_2))
