// Egg drop: find min trials t such that maxFloors(t, eggs) >= k, where
// f(t,e) = f(t-1,e-1) + f(t-1,e) + 1 (floors distinguishable). Time: O(eggs * answer), Space: O(eggs).
public class Solution {
    static int eggDrop(int eggs, int k) {
        long[] f = new long[eggs + 1];
        int t = 0;
        while (f[eggs] < k) {
            t++;
            for (int e = eggs; e >= 1; e--) f[e] = f[e] + f[e - 1] + 1;
        }
        return t;
    }

    public static void main(String[] args) {
        System.out.println(eggDrop(1, 5)); // 5
    }
}
