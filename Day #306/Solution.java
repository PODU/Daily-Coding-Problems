// Day 306: Sort a k-sorted array with a min-heap of size k+1. O(N log k).
import java.util.*;
public class Solution {
    static int[] sortK(int[] arr, int k) {
        PriorityQueue<Integer> pq = new PriorityQueue<>();
        int[] res = new int[arr.length];
        int idx = 0, i = 0;
        for (; i < arr.length && i <= k; i++) pq.add(arr[i]);
        for (; i < arr.length; i++) { res[idx++] = pq.poll(); pq.add(arr[i]); }
        while (!pq.isEmpty()) res[idx++] = pq.poll();
        return res;
    }
    public static void main(String[] a) {
        int[] arr = {6, 5, 3, 2, 8, 10, 9};
        int[] r = sortK(arr, 3);
        StringBuilder sb = new StringBuilder();
        for (int x : r) { if (sb.length() > 0) sb.append(" "); sb.append(x); }
        System.out.println(sb.toString()); // 2 3 5 6 8 9 10
    }
}
