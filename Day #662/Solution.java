// Day 662: GCD of n numbers via repeated Euclidean algorithm.
// Time O(n * log(max)), Space O(1).
public class Solution {
    static int gcd2(int a, int b) { while (b != 0) { int t = a % b; a = b; b = t; } return a; }

    static int gcdAll(int[] v) {
        int g = 0;
        for (int x : v) g = gcd2(g, x);
        return g;
    }

    public static void main(String[] args) {
        int[] v = {42, 56, 14};
        System.out.println(gcdAll(v)); // 14
    }
}
