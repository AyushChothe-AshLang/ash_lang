fn main(){
  let i = 0, nums = [1, 2, 2, 3, 3, 3], counter = {};
  while (i < len(nums)){
    let num = get(nums, i);
    if (!has(counter, num)){
      counter = set(counter, num, 1);
    } else {
      let val = get(counter, num);
      counter = set(counter, num, (val + 1));
    }
    i += 1;
  }
  println(counter);
}