// Josephus problem: iterative recurrence J(n)=(J(n-1)+k)%n, plus k=2 closed form.
// Time O(N) (O(log N) for k=2 closed form), Space O(1).
public class Solution {
    static int josephus(int n, int k) {
        int r = 0;
        for (int i = 2; i <= n; i++) r = (r + k) % i;
        return r + 1;
    }
    static int josephus2(int n) {
        int p = 1;
        while (p * 2 <= n) p *= 2;
        return 2 * (n - p) + 1;
    }
    public static void main(String[] args) {
        int n = 5, k = 2;
        int ans = josephus(n, k);
        if (k == 2) ans = josephus2(n);
        System.out.println(ans);
    }
}
