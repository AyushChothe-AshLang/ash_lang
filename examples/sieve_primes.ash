fn range(s, e) {
    let i = s, res = [];

    while (i < e) {
        res += [i];

        i += 1;
    }

    return res;
}

fn sieve(n) {
    let nums = range(2, n + 1);

    let i = 2, iLoop = int(n ^ 0.5) + 1;

    while (i < iLoop) {
        if (get(nums, i - 2) != 0) {
            let j = i * i, jLoop = n + 1;

            while (j < jLoop) {
                nums = set(nums, j - 2, 0);

                j += i;
            }
        }

        i += 1;
    }

    let k = 0, res = [];

    while (k < len(nums)) {
        let p = get(nums, k);

        if (p != 0) {
            res += [p];
        }

        k += 1;
    }

    return res;
}

fn main() {
    println(sieve(1000));
}