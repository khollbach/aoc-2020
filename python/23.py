import sys

def read_input(lines=sys.stdin) -> [int]:
    lines = list(lines)
    assert len(lines) == 1

    nums = []
    for c in lines[0].strip():
        assert '0' <= c <= '9'
        digit = ord(c) - ord('0')
        nums.append(digit)

    return nums

def extend_nums(nums):
    '''Modifies nums to include all the way up to LARGEST_CUP.'''
    LARGEST_CUP = 1_000_000
    for x in range(max(nums) + 1, LARGEST_CUP + 1):
        nums.append(x)
    assert len(nums) == LARGEST_CUP

def next_map(nums) -> {int: int}:
    n = len(nums)

    next = {}
    for i in range(n):
        next[nums[i]] = nums[(i + 1) % n]
    return next

def remove_after(next, curr: int) -> int:
    ret = next[curr]
    next[curr] = next[ret]
    next.pop(ret)
    return ret

def get_dest(next, curr, largest_cup) -> int:
    curr -= 1
    while curr not in next:
        if curr <= 0:
            curr = largest_cup
        else:
            curr -= 1
    return curr

def insert_after(next, dest, num):
    rest = next[dest]
    next[dest] = num
    next[num] = rest

def print_ring(next):
    '''Start at 1, but don't print 1'''
    curr = 1

    while True:
        curr = next[curr]
        if curr == 1:
            break

        print(curr, end='')

    print()

def part_1(nums, num_iters=100):
    largest_cup = max(nums)
    next = next_map(nums)

    curr = nums[0]
    for _ in range(num_iters):
        removed = []
        for _ in range(3):
            num = remove_after(next, curr)
            removed.append(num)

        dest = get_dest(next, curr, largest_cup)

        for x in reversed(removed):
            insert_after(next, dest, x)

        curr = next[curr]

    return next

def part_2(nums):
    extend_nums(nums)
    return part_1(nums, 10_000_000)

def main():
    nums = read_input()
    #nums = read_input(["389125467"])

    print_ring(part_1(nums))

    next = part_2(nums)
    a = next[1]
    b = next[a]
    print(a, b, a * b)

if __name__ == "__main__":
    main()