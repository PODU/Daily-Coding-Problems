// Day 633: Sort a k-sorted (nearly sorted) array.
// Approach: min-heap of size k+1; pop smallest as we slide the window.
// Time: O(N log k), Space: O(k).
import java.util.*;

public class Solution {
    static int[] sortKSorted(int[] arr, int k) {
        PriorityQueue<Integer> pq = new PriorityQueue<>();
        int[] res = new int[arr.length];
        int idx = 0;
        for (int x : arr) {
            pq.add(x);
            if (pq.size() > k) res[idx++] = pq.poll();
        }
        while (!pq.isEmpty()) res[idx++] = pq.poll();
        return res;
    }

    public static void main(String[] args) {
        int[] arr = {2, 1, 4, 3, 5}; // k = 1
        System.out.println(Arrays.toString(sortKSorted(arr, 1)));
    }
}
