import re
from collections import defaultdict
from typing import Iterable

def build_graph(lines: Iterable[str]) -> (dict, dict):
    # {color -> {child_color -> num_copies}}
    graph = defaultdict(dict)

    # {color -> {parent_color -> num_copies}}
    back_edges = defaultdict(dict)

    for line in lines:
        color, edges = re.match(r'^(.*) bags contain (.*).\n?$', line).groups()

        if edges == 'no other bags':
            continue

        for e in edges.split(', '):
            num_copies, child_color = re.match(r'^(\d+) (.*) bags?$', e).groups()
            num_copies = int(num_copies)
            assert num_copies > 0
            graph[color][child_color] = num_copies
            back_edges[child_color][color] = num_copies

    return graph, back_edges

def count_ancestors(back_edges: dict, start_color: str) -> int:
    seen = set()
    def dfs(color):
        seen.add(color)
        for nbr in back_edges[color]:
            if nbr not in seen:
                dfs(nbr)

    dfs(start_color)
    return len(seen) - 1  # exclude start_color

def weighted_child_count(graph: dict, start_color: str) -> int:
    seen = set()
    def dfs(color) -> int:
        seen.add(color)
        total = 1  # self
        for (nbr, weight) in graph[color].items():
            assert nbr not in seen
            total += weight * dfs(nbr)
        seen.remove(color)
        return total

    return dfs(start_color) - 1  # exclude start

def main():
    g, b = build_graph(open('input'))
    count = count_ancestors(b, 'shiny gold')
    print(count)
    count = weighted_child_count(g, 'shiny gold')
    print(count)

if __name__ == '__main__':
    main()
