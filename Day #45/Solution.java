// rand7 from rand5: rejection sampling over 5*(rand5-1)+rand5 in 1..25,
// reject >21, map ((v-1)%7)+1. Expected O(1) amortized. rand5 from a seeded LCG.
public class Solution {
    static long state = 1; // deterministic seed

    static int rand5() {
        state = (state * 75 + 74) % 65537;
        return (int) (state % 5) + 1; // uniform-ish 1..5 for the demo
    }

    static int rand7() {
        while (true) {
            int v = 5 * (rand5() - 1) + rand5(); // 1..25
            if (v <= 21) return (v - 1) % 7 + 1;
        }
    }

    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder("rand7 samples:");
        for (int i = 0; i < 20; i++) sb.append(" ").append(rand7());
        System.out.println(sb);

        int[] counts = new int[8];
        for (int i = 0; i < 7000; i++) counts[rand7()]++;
        StringBuilder c = new StringBuilder("counts over 7000 trials:");
        for (int v = 1; v <= 7; v++) c.append(" ").append(v).append(":").append(counts[v]);
        System.out.println(c);
    }
}
