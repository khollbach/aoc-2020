from typing import List, Optional

day1 = __import__('1')

def main():
    nums = list(map(int, open('../inputs/9')))

    invalid_num = None
    for i in range(25, len(nums)):
        if day1.two_sum(nums[i-25:i], nums[i]) is None:
            invalid_num = nums[i]
            break
    assert invalid_num is not None, 'No anomaly found'
    print(invalid_num)

    range_ = contiguous_sum(nums, invalid_num)
    print(min(range_) + max(range_))

def contiguous_sum(nums: List[int], target: int) -> Optional[List[int]]:
    assert target >= 0
    assert all(map(lambda n: n >= 0, nums))

    # The current "window" is nums[lo:hi].
    lo = 0
    hi = 0
    window_sum = 0
    while True:
        if window_sum == target:
            return nums[lo:hi]
        elif window_sum < target:
            if hi >= len(nums):
                return None
            window_sum += nums[hi]
            hi += 1
        else:
            assert window_sum > target
            assert lo < hi
            window_sum -= nums[lo]
            lo += 1

main()
