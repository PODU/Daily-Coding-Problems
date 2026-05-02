// Egg drop: DP on trials. f(t,e)=f(t-1,e-1)+f(t-1,e)+1 = max floors with t trials, e eggs.
// Smallest t with f(t,N)>=k. Time O(N*answer), Space O(N).
public class Solution {
    // Minimum worst-case trials for N eggs and k floors.
    static int eggDrop(int N, int k) {
        if (k == 0) return 0;
        long[] f = new long[N + 1];
        int t = 0;
        while (f[N] < k) {
            t++;
            for (int e = N; e >= 1; e--) {
                f[e] = f[e] + f[e - 1] + 1;
            }
        }
        return t;
    }

    public static void main(String[] args) {
        System.out.println(eggDrop(1, 5));
    }
}
