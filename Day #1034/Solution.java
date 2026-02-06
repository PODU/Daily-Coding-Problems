// Day 1034: Expected rounds until one of n fair coins remains.
// Model: expected max of n iid Geometric(1/2) lifetimes; E = sum_{m>=0}(1-(1-2^-m)^n). O(iter).
public class Solution {
    static double expectedRounds(int n) {
        double e = 0.0, p = 1.0; // p = 2^-m
        for (int m = 0; m < 100000; m++) {
            double term = 1.0 - Math.pow(1.0 - p, n);
            if (term < 1e-12 && m > 0) break;
            e += term;
            p *= 0.5;
        }
        return e;
    }

    public static void main(String[] args) {
        int n = 4;
        System.out.printf("n=%d -> expected rounds: %.4f%n", n, expectedRounds(n));
    }
}
