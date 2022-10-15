cfn fib(n) {
  if (n <= 1) {
    return n;
  }
  return fib(n - 1) + fib(n - 2);
}

fn main() {
  let x = 20;
  let i = 0, n = x;
  while (i < n) {
    println(fib(i));
    i = i + 1;
  }
  let i = x, n = 0;
  while (i >= n) {
    println(fib(i));
    i = i - 1;
  }
}