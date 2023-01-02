fn generate(numRows){
    let triangle = [[1], [1, 1]];

    if (numRows == 1) {
        return [[1]];
    } elif (numRows == 2) {
        return triangle;
    }

    let i = 2;

    while (i < numRows) {
        let temp = [1];

        let j = 1;

        while (j < len(get(triangle, i - 1))) {
            temp += [get(get(triangle, i - 1), j) + get(get(triangle, i - 1), j - 1)];

            j += 1;
        }

        temp += [1];

        triangle += [temp];

        i += 1;
    }

    return (triangle);
}

fn main() {
    print(generate(5));
}