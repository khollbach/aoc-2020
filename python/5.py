from typing import List, Optional

def main():
    nums = []
    for line in open('../inputs/5'):
        bits = map(to_bit, line.strip())
        bit_string = ''.join(map(str, bits))
        n = int(bit_string, 2)
        nums.append(n)

    print(max(nums))
    print(missing(nums))

def to_bit(c: str) -> int:
    assert c in 'FBLR'
    return int(c in 'BR')

def missing(nums: List[int]) -> Optional[int]:
    nums = sorted(nums)
    for i in range(1, len(nums)):
        if nums[i] > nums[i-1] + 1:
            return nums[i-1] + 1
    return None

main()
