#[allow(dead_code)]
/*
The goal of this app is to create a function
that will list all of the factor pairs of
any given number.
*/
use factors::numput;
use factors::numthry;

fn main() {
    println!("\nPlease input an integer:  ");
    let trunk = numput::inpt_u32();
    println!("\nYou entered {}.", trunk);

    let leaves = findfctrs_list(trunk);
    println!("\nThe factors of {} are:  \n{:?}", trunk, leaves);
}

fn findfctrs_list(spur: u32) -> Vec<u32> {
    let mut branches = vec![1, spur];

    let mut i = 2;
    while i < spur {
        if spur % i == 0 {
            branches.push(i);
            if i != (spur / i) {
                branches.push(spur / i);
            }
        }
        i += 1;

        if i >= branches[branches.len() - 1] {
            return branches;
        }
    }
    return branches;
}
