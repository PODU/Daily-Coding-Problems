// Day 1064: Implement rand7() from rand5() via rejection sampling.
// (rand5()-1)*5 + rand5() -> uniform 1..25; reject >21; ((v-1)%7)+1. Expected O(1) calls, O(1) space.
import java.util.Random;

public class Solution {
    static Random rng = new Random(42);

    static int rand5() {
        return rng.nextInt(5) + 1;
    }

    static int rand7() {
        while (true) {
            int v = (rand5() - 1) * 5 + rand5(); // uniform 1..25
            if (v <= 21) return ((v - 1) % 7) + 1;
        }
    }

    public static void main(String[] args) {
        int[] counts = new int[8];
        for (int i = 0; i < 70000; i++) counts[rand7()]++;
        for (int i = 1; i <= 7; i++)
            System.out.println(i + ": " + counts[i]);
    }
}
