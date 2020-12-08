import re
from collections import defaultdict
from typing import Iterable

def build_graph(lines: Iterable[str]) -> (dict, dict):
    # {color -> {child_color -> num_copies}}
    graph = defaultdict(dict)

    # {color -> {parent_color -> num_copies}}
    back_edges = defaultdict(dict)

    for line in lines:
        color, edges = re.match(r'^(.*) bags contain (.*)\.\n?$', line).groups()

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
        for neighbour in back_edges[color]:
            if neighbour not in seen:
                dfs(neighbour)

    dfs(start_color)
    return len(seen) - 1  # exclude start_color

def weighted_child_count(graph: dict, start_color: str) -> int:
    # This is the set of nodes on the path from the start node to the current
    # node. We use it to detect cycles.
    current_path = set()

    def dfs(color) -> int:
        current_path.add(color)

        total = 1  # self
        for (neighbour, weight) in graph[color].items():
            assert neighbour not in current_path
            total += weight * dfs(neighbour)

        current_path.remove(color)
        return total

    return dfs(start_color) - 1  # exclude start

def main():
    graph, back_edges = build_graph(open('../inputs/7'))
    count = count_ancestors(back_edges, 'shiny gold')
    print(count)
    count = weighted_child_count(graph, 'shiny gold')
    print(count)

if __name__ == '__main__':
    main()
