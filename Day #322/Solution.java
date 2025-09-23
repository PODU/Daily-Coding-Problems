// Min jumps on number line: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even.
// Time: O(sqrt N), Space: O(1).
public class Solution {
    static int minJumps(long N) {
        long target = Math.abs(N);
        long k = 0, S = 0;
        while (S < target || (S - target) % 2 != 0) {
            k++;
            S += k;
        }
        return (int) k;
    }

    public static void main(String[] args) {
        long N = 10;
        System.out.println("Minimum jumps to reach " + N + ": " + minJumps(N));
    }
}
