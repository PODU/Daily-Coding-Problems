// Day 779: Egg drop - minimum worst-case trials for N eggs, k floors.
// Find smallest m with sum_{i=1..N} C(m,i) >= k. O(N * answer) time, O(1) space.
public class Solution {
    static int eggDrop(int eggs, int floors) {
        if (floors == 0) return 0;
        int m = 0;
        while (true) {
            m++;
            long reach = 0, c = 1;
            for (int i = 1; i <= eggs; i++) {
                c = c * (m - i + 1) / i;
                reach += c;
                if (reach >= floors) break;
            }
            if (reach >= floors) return m;
        }
    }

    public static void main(String[] args) {
        System.out.println(eggDrop(1, 5));   // 5
        System.out.println(eggDrop(2, 100)); // 14
    }
}
