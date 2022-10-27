def bubbleSort(arr):
    i = 0
    j = 0
    while i < len(arr):
        j = 0
        while j < (len(arr) - 1):
            x = arr[j]
            y = arr[j + 1]
            if x > y:
                arr[j] = y
                arr[j + 1] = x
            j += 1
        i += 1
    return arr


def main():
    nums = [1, 3, 5, 7, 9, 2, 4, 6, 8, 0]
    print(bubbleSort(nums))


if __name__ == "__main__":
    main()
