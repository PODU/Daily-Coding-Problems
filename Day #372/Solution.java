// Day 372: Count digits of a natural number without loops.
// Recursion: 1 digit for n<10, else 1 + digits(n/10). Time O(d), Space O(d).
public class Solution {
    static int numDigits(long n) {
        return n < 10 ? 1 : 1 + numDigits(n / 10);
    }

    public static void main(String[] args) {
        System.out.println(numDigits(0));      // 1
        System.out.println(numDigits(7));      // 1
        System.out.println(numDigits(42));     // 2
        System.out.println(numDigits(12345));  // 5
    }
}
