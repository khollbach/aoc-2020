# pip3 install recordclass
# It's basically collections.namedtuple but mutable.
from recordclass import recordclass

from typing import List

Instr = recordclass('Instr', 'op arg')

def main():
    code: List[Instr] = []
    for line in open('../inputs/8'):
        op, arg = line.strip().split()
        assert op in 'acc jmp nop'.split()
        code.append(Instr(op, int(arg)))

    # Part 1
    try:
        run(code)
        assert False, 'Code should spin'
    except InfiniteLoop as e:
        print(e.acc)

    # Part 2
    def flip(instr):
        if instr.op == 'jmp':
            instr.op = 'nop'
        elif instr.op == 'nop':
            instr.op = 'jmp'

    ans = None
    for i in range(len(code)):
        if code[i].op == 'acc':
            continue

        try:
            flip(code[i])
            ans = run(code)
            break
        except InfiniteLoop:
            pass
        finally:
            flip(code[i])

    assert ans is not None
    print(ans)

class InfiniteLoop(Exception):
    def __init__(self, i: int, acc: int):
        super().__init__('Infinite loop at instruction {}. Accumulator: {}'.format(i, acc))
        self.i = i
        self.acc = acc

def run(code: List[Instr]) -> int:
    acc = 0
    seen = set()

    i = 0
    while i < len(code):
        if i in seen:
            raise InfiniteLoop(i, acc)

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
            assert False, 'Not an operation {}'.format(op)

        i += 1

    return acc

main()
