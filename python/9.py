day1 = __import__('1')

def main():
    nums = list(map(int, open('../inputs/9')))

    for i in range(25, len(nums)):
        if day1.two_sum(nums[i-25:i], nums[i]) is None:
            print(nums[i])
            break
    else:
        assert False, 'No anomaly found'

main()
