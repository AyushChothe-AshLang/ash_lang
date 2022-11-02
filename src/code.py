import math


def sieve(n):
    nums = list(range(2, n + 1))
    for i in list(range(2, math.ceil(math.sqrt(n)))):
        if nums[i - 2] != 0:
            for j in list(range(i * i, n + 1, i)):
                nums[j - 2] = 0
    return [x for x in nums if x != 0]


if __name__ == "__main__":
    print(sieve(1000))
