// Reservoir sampling (size 1): i-th element (1-indexed) replaces pick with prob 1/i. O(1) space.
// Demo uses a portable 64-bit LCG seeded with 1 so output is deterministic across languages -> 7.
public class Solution {
    public static void main(String[] args) {
        int[] stream = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
        final long A = 6364136223846793005L, C = 1442695040888963407L;
        long state = 1L; // fixed seed

        int pick = 0;
        for (int i = 1; i <= stream.length; i++) {
            state = state * A + C;                       // advance LCG (mod 2^64 via wraparound)
            if (Long.remainderUnsigned(state, i) == 0)   // replace with probability 1/i
                pick = stream[i - 1];
        }
        System.out.println("Selected: " + pick);
    }
}
