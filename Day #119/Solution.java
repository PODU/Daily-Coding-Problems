// Day 119: Min points to stab all intervals. Greedy: sort by start desc, pick start
// of each not-yet-stabbed interval (mirror of the classic sort-by-end greedy). O(n log n).
import java.util.*;

public class Solution {
    static List<Integer> minCover(int[][] iv){
        Arrays.sort(iv, (a, b) -> Integer.compare(b[0], a[0]));
        List<Integer> pts = new ArrayList<>();
        boolean has = false; int last = 0;
        for (int[] in : iv){
            if (!has || last > in[1]){ last = in[0]; pts.add(in[0]); has = true; }
        }
        Collections.sort(pts);
        return pts;
    }
    public static void main(String[] args){
        List<Integer> r = minCover(new int[][]{{0,3},{2,6},{3,4},{6,9}});
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < r.size(); i++){
            sb.append(r.get(i));
            if (i + 1 < r.size()) sb.append(", ");
        }
        sb.append("}");
        System.out.println(sb); // {3, 6}
    }
}
