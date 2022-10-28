fn bubbleSort(arr) {
  let i = 0, j = 0;
  while (i < len(arr)) {
    j = 0;
    while (j < (len(arr) - 1)) {
      let x = get(arr, j), y = get(arr, j + 1);
      if (x > y) {
        arr = set(set(arr, j, y), j + 1, x);
      }
      j += 1;
    }
    i += 1;
  }
  return arr;
}

fn main() {
  let nums = [1, 3, 5, 7, 9, 2, 4 ,6, 8, 0];
  return (bubbleSort(nums));
}