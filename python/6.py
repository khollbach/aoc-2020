from typing import List, Set
from string import ascii_lowercase
from functools import reduce

def main():
    groups = []
    for paragraph in open('../inputs/6').read().split('\n\n'):
        group: List[Set[str]] = []
        for line in paragraph.rstrip('\n').split('\n'):
            assert line
            group.append(answers(line))
        assert group
        groups.append(group)

    print(sum(map(len, map(union, groups))))
    print(sum(map(len, map(intersection, groups))))

def answers(line: str) -> Set[str]:
    ans = set()
    for c in line:
        assert c in ascii_lowercase
        ans.add(c)
    return ans

def union(group: List[Set[str]]) -> Set[str]:
    return reduce(set.union, group)

def intersection(group: List[Set[str]]) -> Set[str]:
    return reduce(set.intersection, group)

main()
