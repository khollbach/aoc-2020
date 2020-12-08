from collections import namedtuple
from typing import List

Instr = namedtuple('Instr', 'op arg')

code: List[Instr] = []
for line in open('../inputs/8'):
    op, arg = line.strip().split()
    assert op in 'acc jmp nop'.split()
    code.append(Instr(op, int(arg)))

acc = 0
seen = set()

i = 0
while i not in seen:
    seen.add(i)
    op, arg = code[i]

    if op == 'jmp':
        i += arg
        continue

    if op == 'acc':
        acc += arg
    elif op == 'nop':
        pass
    else:
        assert False, 'Not an operation {}' % op

    i += 1

print(acc)
