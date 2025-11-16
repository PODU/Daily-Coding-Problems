// Day 610: Integer division of positive ints without / , * , or %.
// Approach: repeated doubling subtraction (binary long division). Time O(log^2), Space O(1).
'use strict';

function divide(dividend, divisor) {
    let q = 0;
    while (dividend >= divisor) {
        let temp = divisor, mult = 1;
        while (dividend >= (temp << 1)) { temp <<= 1; mult <<= 1; }
        dividend -= temp;
        q += mult;
    }
    return q;
}

function main() {
    console.log(divide(10, 3)); // 3
    console.log(divide(43, 8)); // 5
}

main();
