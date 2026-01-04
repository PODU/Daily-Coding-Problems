// Day 849: Collatz - verify each n reaches 1; bonus: longest chain for n <= 1,000,000.
// Memoized chain lengths in an array. ~O(limit) amortized.
public class Solution {
    static int steps(long n){
        int c = 0;
        while(n != 1){ n = (n & 1) == 1 ? 3*n + 1 : n/2; c++; }
        return c;
    }
    public static void main(String[] args){
        System.out.println("27 reaches 1 in " + steps(27) + " steps"); // 111

        final int LIMIT = 1000000;
        int[] cache = new int[LIMIT + 1];
        cache[1] = 1;
        int bestN = 1, bestLen = 1;
        for(int i = 2; i <= LIMIT; i++){
            long n = i; int len = 0;
            while(n >= i || cache[(int)n] == 0){
                n = (n & 1) == 1 ? 3*n + 1 : n/2;
                len++;
                if(n == 1) break;
            }
            int total = len + (n <= LIMIT ? cache[(int)n] : 1);
            cache[i] = total;
            if(total > bestLen){ bestLen = total; bestN = i; }
        }
        System.out.println("Longest chain for n <= 1000000: n = " + bestN
            + " (length " + bestLen + ")"); // n = 837799
    }
}
