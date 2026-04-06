// rand7 from rand5 via rejection sampling: combine two rand5 into uniform
// 1..25, accept 1..21, map to 1..7. Expected O(1) calls, O(1) space.
import java.util.*;

public class Solution {
    static Random rng = new Random(42);
    static int rand5() { return rng.nextInt(5) + 1; } // uniform 1..5

    static int rand7() {
        while (true) {
            int v = (rand5() - 1) * 5 + (rand5() - 1); // uniform 0..24
            if (v < 21) return v % 7 + 1;              // accept 0..20 -> 1..7
        }
    }

    public static void main(String[] args) {
        int[] counts = new int[8];
        for (int i = 0; i < 70000; i++) counts[rand7()]++;
        for (int i = 1; i <= 7; i++) System.out.println(i + ": " + counts[i]);
    }
}
