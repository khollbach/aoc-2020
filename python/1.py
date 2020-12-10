from typing import List, Optional, Tuple

def main():
    nums = list(map(int, open('../inputs/1')))

    x, y = two_sum(nums, 2020)
    print(x * y)
    x, y, z = _three_sum(nums, 2020)
    print(x * y * z)

def two_sum(nums: List[int], target: int) -> Optional[Tuple[int, int]]:
    for (i, x) in enumerate(nums):
        for y in nums[i+1:]:
            if x + y == target:
                return (x, y)
    return None

def _three_sum(nums: List[int], target: int) -> Optional[Tuple[int, int, int]]:
    for x in nums:
        for y in nums:
            for z in nums:
                if x + y + z == 2020:
                    return (x, y, z)
    return None

if __name__ == '__main__':
    main()
