from collections import Counter

nums = list(map(int, open('../inputs/10')))
nums.append(0)  # Charging outlet.
nums.sort()
nums.append(nums[-1] + 3)  # Device.

def sub(pair) -> int:
    a, b = pair
    return a - b
diffs = map(sub, zip(nums[1:], nums[:-1]))

freqs = Counter(diffs)
assert len(freqs) <= 3
print(freqs[1] * freqs[3])
