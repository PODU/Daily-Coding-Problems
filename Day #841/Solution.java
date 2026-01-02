// Day 841: count positive integer pairs (a,b) with a+b=M and a^b=N.
// Math: a+b = (a^b) + 2*(a&b) => a&b=(M-N)/2; answer = 2^popcount(N) minus zero-cases. O(1).
public class Solution {
    static long countPairs(long M, long N){
        long d = M - N;
        if(d < 0 || (d & 1) != 0) return 0;
        long c = d / 2;                         // c = a & b
        if((c & N) != 0) return 0;              // AND and XOR bits cannot overlap
        long res = 1L << Long.bitCount(N);
        if(c == 0) res -= (N != 0) ? 2 : 1;
        return res < 0 ? 0 : res;
    }
    public static void main(String[] args){
        System.out.println(countPairs(4, 2));   // 2
    }
}
