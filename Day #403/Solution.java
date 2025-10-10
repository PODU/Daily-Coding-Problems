// rand7 from rand5 via rejection sampling: idx=(rand5-1)*5+rand5 in 1..25, reject>21,
// return (idx-1)%7+1. O(1) expected calls. Space O(1).
import java.util.Random;

public class Solution {
    static Random gen = new Random(12345);

    static int rand5() { return gen.nextInt(5) + 1; }

    static int rand7() {
        while (true) {
            int idx = (rand5() - 1) * 5 + rand5(); // uniform 1..25
            if (idx <= 21) return (idx - 1) % 7 + 1;
        }
    }

    public static void main(String[] args) {
        final int N = 70000;
        int[] counts = new int[8];
        for (int i = 0; i < N; i++) counts[rand7()]++;
        for (int v = 1; v <= 7; v++) {
            StringBuilder sb = new StringBuilder();
            sb.append(v).append(": ").append(counts[v]).append(" ");
            for (int b = 0; b < counts[v] / 250; b++) sb.append("#");
            System.out.println(sb.toString());
        }
    }
}
