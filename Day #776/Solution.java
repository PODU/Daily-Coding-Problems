// Day 776: Josephus problem. General O(N) recurrence j=(j+k)%i.
// For k=2 an O(log N) closed form is also given. Returns 1-indexed survivor.
public class Solution {
    static int josephus(int n, int k) {
        int r = 0;
        for (int i = 2; i <= n; i++) r = (r + k) % i;
        return r + 1;
    }

    static int josephusK2(int n) {
        int p = 1;
        while (p * 2 <= n) p *= 2;
        return 2 * (n - p) + 1;
    }

    public static void main(String[] args) {
        System.out.println(josephus(5, 2));   // 3
        System.out.println(josephusK2(5));    // 3
    }
}
