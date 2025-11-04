// Von Neumann fair-coin from biased coin: toss pairs, (0,1)->0, (1,0)->1, else retry.
// Expected O(1) tosses per fair toss; O(1) space.
public class Solution {
    static long rngState = 88172645463325252L;
    static double nextUniform() { // xorshift64 -> [0,1)
        rngState ^= rngState << 13;
        rngState ^= rngState >>> 7;
        rngState ^= rngState << 17;
        return (rngState >>> 11) * (1.0 / 9007199254740992.0);
    }

    static int tossBiased() { return nextUniform() < 0.3 ? 1 : 0; } // p(1)=0.3

    static int fairToss() {
        while (true) {
            int a = tossBiased();
            int b = tossBiased();
            if (a == 0 && b == 1) return 0;
            if (a == 1 && b == 0) return 1;
        }
    }

    public static void main(String[] args) {
        long heads = 0, tails = 0;
        for (int i = 0; i < 100000; i++) {
            if (fairToss() == 1) heads++; else tails++;
        }
        System.out.println("heads: " + heads + ", tails: " + tails);
    }
}
