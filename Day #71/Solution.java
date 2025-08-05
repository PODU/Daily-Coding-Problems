// rand5 from rand7 via rejection sampling: call rand7() until <=5. Uniform 1..5. Time O(1) expected, Space O(1).
import java.util.Random;

public class Solution {
    static final Random RNG = new Random();

    static int rand7() {
        return RNG.nextInt(7) + 1;
    }

    static int rand5() {
        int x;
        do { x = rand7(); } while (x > 5);
        return x;
    }

    public static void main(String[] args) {
        int trials = 100000;
        long[] counts = new long[6];
        for (int i = 0; i < trials; i++) {
            int v = rand5();
            assert v >= 1 && v <= 5;
            if (v < 1 || v > 5) throw new RuntimeException("out of range");
            counts[v]++;
        }
        double expected = trials / 5.0;
        for (int v = 1; v <= 5; v++) {
            if (Math.abs(counts[v] - expected) >= expected * 0.1)
                throw new RuntimeException("not uniform");
        }
        System.out.println("rand5 OK");
    }
}
