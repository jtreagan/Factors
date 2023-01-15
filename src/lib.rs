// Input, error check, and return numbers
// from standard input.

pub mod numput {

    // Return an integer from standard input.
    // Probably shouldn't use this.  Instead create
    // functions specific to type.

    pub fn inpt_int() -> isize {
        loop {
            let mut input = String::new();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            let input: isize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a numeric value.  Thanks!");
                    continue;
                }
            };
            break input;
        }
    }

    // Return a u32 from standard input.
    // Largest value possible with U32 is   2^32 - 1 =
    //                          4294967295
    // Need to check to see how this function handles negative input.

    pub fn inpt_u32() -> u32 {
        loop {
            let mut input = String::new();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a numeric value.  Thanks!");
                    continue;
                }
            };
            break input;
        }
    }

    // Return an f64 from standard input.

    pub fn inpt_f64() -> f64 {
        loop {
            let mut input = String::new();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            let input: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a numeric value.  Thanks!");
                    continue;
                }
            };
            break input;
        }
    }

    // Return a u64 from standard input.
    // Largest value possible with U64 is   2^64 - 1 =
    //           18,446,744,073,709,551,615
    // Need to check to see how this function handles negative input.

    pub fn inpt_u64() -> u64 {
        loop {
            let mut input = String::new();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            let input: u64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a numeric value.  Thanks!");
                    continue;
                }
            };
            break input;
        }
    }

    // Check a passed string to see if it is an integer.

    pub fn chk4int(george: &str) -> bool {
        match george.trim().parse::<isize>() {
            Ok(_) => true,
            Err(_) => {
                println!("/n{} is not a numerical value.", george);
                return false;
            }
        }
    }
} // End of numput module.

// Wild function!

// Return a string from standard input.

pub fn inpt_strng() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub mod numthry {

    /*
    Prime or Composite (porc())
    Determine if a number is prime returning a flag of true
           for primeness along with a divisor of 1.
    If not, then return a flag of false along with the first divisor encountered.
    This uses a brute force tactic.  May be able to reduce the number of
    factors that have to be checked by applying some number theory.

    First:  Check that the number is not == 2, the only even prime.
    Second:  If the number is even, it can't be prime (except for 2).
                    Checking this here lets me increment i by 2 rather than 1,
                    thus cutting in half the number of necessary iterations.
    Third:  Use a loop to test for divisibility by all integers less than the
                number being passed to the function.  Increment by two to test
                only odd numbers.
    */

    pub fn porc_u64(egg: u64) -> (bool, u64) {
        if egg == 2 {
            return (true, 1);
        }

        if egg % 2 == 0 {
            return (false, 2);
        }

        let mut i = 3;
        while i < egg {
            if egg % i == 0 {
                return (false, i);
            }
            i += 2;
        }
        return (true, 1);
    }

    pub fn porc_u32(egg: u32) -> (bool, u32) {
        if egg == 2 {
            return (true, 1);
        }

        if egg % 2 == 0 {
            return (false, 2);
        }

        let mut i = 3;
        while i < egg {
            if egg % i == 0 {
                return (false, i);
            }
            i += 2;
        }
        return (true, 1);
    }

    /*
        Lists all the factors of a given integer.  The returned
        vector is unsorted with the factor pairs listed next to
        each other.  If the number is a perfect square, then the
        list will include two entries of the squared factor.
    */

    pub fn findfctrs_pairs(spur: u32) -> Vec<u32> {
        let mut branches = vec![1, spur];

        let mut i = 2;
        while i < spur {
            if spur % i == 0 {
                branches.push(i);
                branches.push(spur / i);
            }
            i += 1;

            if i >= branches[branches.len() - 1] {
                return branches;
            }
        }
        return branches;
    }
} // End of numthry module.

pub mod geometry {

    /*
        Pass two points as cartesian coords to this function and it
        will return the distance between those two points.
    */

    pub struct Point {
        x: f64,
        y: f64,
    }

    pub fn dist_2pts(p1: &Point, p2: &Point) -> f64 {
        ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt()
    }
} // End of geometry module.
