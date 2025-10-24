// Day 483: Josephus problem.
// Iterative recurrence f(i)=(f(i-1)+k)%i in O(n) time, O(1) space.
// Special O(log n) closed form when k=2.
public class Solution {
    static int josephus(int n, int k) {
        int result = 0; // 0-indexed survivor among 1 person
        for (int i = 2; i <= n; i++)
            result = (result + k) % i;
        return result + 1; // 1-indexed
    }

    // O(log n) special case for k == 2.
    static int josephusK2(int n) {
        int p = 1;
        while (p * 2 <= n) p *= 2;
        return 2 * (n - p) + 1;
    }

    public static void main(String[] args) {
        int n = 5, k = 2;
        System.out.println(josephus(n, k)); // 3
    }
}
