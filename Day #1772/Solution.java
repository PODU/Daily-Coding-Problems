// Day 1772: Min jumps to reach N. Grow k until triangular sum >= |N| and (sum-|N|) even.
// Flipping any jump j changes parity of (sum-N) by 2j, so even surplus is reachable. O(sqrt(N)).
public class Solution {
    static int minJumps(long n) {
        long s = Math.abs(n);
        long sum = 0;
        int k = 0;
        while (sum < s || (sum - s) % 2 != 0) {
            k++;
            sum += k;
        }
        return k;
    }

    public static void main(String[] args) {
        System.out.println(minJumps(10));
    }
}
