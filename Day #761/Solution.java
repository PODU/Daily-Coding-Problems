// Day 761: rand5() from rand7() via rejection sampling.
// Accept values 1..5, reject 6..7 and retry. Uniform; expected O(1) calls (7/5).
public class Solution {
    static long s = 1;
    static int rand7() {
        s = (s * 1103515245L + 12345L) & 0x7fffffffL;
        return (int) (s % 7) + 1;   // uniform 1..7
    }
    static int rand5() {
        int x;
        do { x = rand7(); } while (x > 5);
        return x;
    }

    public static void main(String[] args) {
        long N = 100000;
        long[] cnt = new long[6];
        for (long i = 0; i < N; i++) cnt[rand5()]++;
        System.out.println("counts over " + N + " samples:");
        for (int v = 1; v <= 5; v++) System.out.println(v + ": " + cnt[v]);
    }
}
