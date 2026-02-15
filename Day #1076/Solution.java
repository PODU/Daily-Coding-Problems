// Variadic alternating add/subtract: first arg seeds the total, then the
// rest alternate +, -, +, ... O(n) time, O(1) space.
public class Solution {
    static int addSubtract(int... args) {
        if (args.length == 0) return 0;
        int result = args[0], sign = 1;
        for (int i = 1; i < args.length; i++) { result += sign * args[i]; sign = -sign; }
        return result;
    }
    public static void main(String[] args) {
        System.out.println("add_subtract(7) = "            + addSubtract(7));
        System.out.println("add_subtract(1)(2)(3) = "      + addSubtract(1, 2, 3));
        System.out.println("add_subtract(-5)(10)(3)(9) = " + addSubtract(-5, 10, 3, 9));
    }
}
