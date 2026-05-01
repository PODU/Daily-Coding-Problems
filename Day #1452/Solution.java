// Day 1452: Sort a k-sorted array (each element <= k from its place) using a
// min-heap of size k+1. Time O(N log k), Space O(k).
import java.util.*;

public class Solution {
    static int[] sortKSorted(int[] a, int k) {
        PriorityQueue<Integer> heap = new PriorityQueue<>(); // min-heap
        int[] out = new int[a.length];
        int idx = 0;
        for (int x : a) {
            heap.add(x);
            if (heap.size() > k) out[idx++] = heap.poll();
        }
        while (!heap.isEmpty()) out[idx++] = heap.poll();
        return out;
    }

    public static void main(String[] args) {
        int[] a = {2, 6, 3, 12, 56, 8};
        int[] s = sortKSorted(a, 3);
        System.out.println(Arrays.toString(s)); // [2, 3, 6, 8, 12, 56]
    }
}
