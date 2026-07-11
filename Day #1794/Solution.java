// Misere Nim: P-position (loss for mover) iff (all heaps==1 with even count) or (some heap>1 and xor==0). First wins otherwise.
// Time O(n), Space O(1).
public class Solution {
    static boolean firstPlayerWins(int[] heaps) {
        int x = 0, mx = 0;
        for (int h : heaps) { x ^= h; mx = Math.max(mx, h); }
        boolean pPosition;
        if (mx <= 1) pPosition = (x == 0);   // all heaps == 1: P iff even count
        else pPosition = (x == 0);            // some heap > 1: P iff xor == 0
        return !pPosition;
    }
    public static void main(String[] args) {
        int[] heaps = {3, 4, 5};
        System.out.println(firstPlayerWins(heaps)); // expected true
    }
}
