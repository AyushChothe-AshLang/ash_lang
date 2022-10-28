cfn fact(n) {
    if(n < 2){
        return n;
    }
    return fact(n-1) + fact(n-2);
}

fn main() {
    return fact(80);
}