// Day 1102: Min total movement to seat people contiguously (order preserved).
// person i lands at start+i; minimize sum|pos[i]-(start+i)| => shift b[i]=pos[i]-i
// to its median. Time: O(N) avg (selection) / O(M log M). Space: O(M).
import java.util.*;

public class Solution {
    static long minCost(int[] seats){
        ArrayList<Long> b = new ArrayList<>();
        int p = 0;
        for (int i = 0; i < seats.length; i++)
            if (seats[i] == 1) b.add((long)(i - p++));
        if (b.isEmpty()) return 0;
        Collections.sort(b);
        long med = b.get(b.size()/2), cost = 0;
        for (long x : b) cost += Math.abs(x - med);
        return cost;
    }
    public static void main(String[] args){
        System.out.println(minCost(new int[]{0,1,1,0,1,0,0,0,1})); // 5
    }
}
