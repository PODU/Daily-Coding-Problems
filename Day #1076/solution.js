// Curried function: valueOf/toString return current total; callable to chain.
// First arg seeds the total; later calls alternate +, -, +, ... O(1)/call.
function add_subtract(a) {
    function make(total, sign) {
        function chain(x) { return make(total + sign * x, -sign); }
        chain.valueOf  = () => total;
        chain.toString = () => String(total);
        return chain;
    }
    return make(a, 1);  // next op after first arg is add
}

console.log("add_subtract(7) = "            + add_subtract(7));
console.log("add_subtract(1)(2)(3) = "      + add_subtract(1)(2)(3));
console.log("add_subtract(-5)(10)(3)(9) = " + add_subtract(-5)(10)(3)(9));
