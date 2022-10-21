# FizzBuzz Python
def main():
    i, n = 1, 1000
    while i <= n:
        print(("Fizz" * int((i % 3) == 0) + "Buzz" * int((i % 5) == 0)) or str(i))
        i += 1

if __name__ == "__main__":
    main()