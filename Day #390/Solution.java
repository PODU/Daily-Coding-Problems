// Presence bitset: mark each present value, then report unmarked ones.
// Time: O(N), Space: O(N) bits.  (N = 1,000,000)
import java.util.*;

public class Solution {
    static List<Integer> findMissing(int[] present, int N){
        boolean[] seen = new boolean[N+1];
        for(int x : present) seen[x] = true;
        List<Integer> missing = new ArrayList<>();
        for(int i=1;i<=N;i++) if(!seen[i]) missing.add(i);
        return missing;
    }

    public static void main(String[] args){
        final int N = 1000000;
        int[] present = new int[N - 1000];
        int idx = 0;
        for(int i=1;i<=N;i++) if(i % 1000 != 0) present[idx++] = i;

        List<Integer> missing = findMissing(present, N);
        System.out.println("Missing count: " + missing.size());
        StringBuilder sb = new StringBuilder("First 10 missing:");
        for(int i=0;i<10 && i<missing.size();i++) sb.append(" ").append(missing.get(i));
        System.out.println(sb);
        System.out.println("Time complexity: O(N), Space complexity: O(N) bits (N = 1,000,000)");
    }
}
