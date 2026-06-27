// Sort a k-sorted array (each element <= k from its sorted position).
// Min-heap of size k+1: pop the min as the next sorted element. O(N log k) time, O(k) space.
import java.util.*;

public class Solution {
    static int[] sortKSorted(int[] arr, int k) {
        PriorityQueue<Integer> minHeap = new PriorityQueue<>();
        int n = arr.length;
        int[] result = new int[n];
        int idx = 0, i = 0;
        for (; i <= k && i < n; i++) minHeap.add(arr[i]);
        for (; i < n; i++) {
            result[idx++] = minHeap.poll();
            minHeap.add(arr[i]);
        }
        while (!minHeap.isEmpty()) result[idx++] = minHeap.poll();
        return result;
    }

    public static void main(String[] args) {
        int[] arr = {2, 1, 4, 3, 6, 5}; // k-sorted with k = 2
        int[] sorted = sortKSorted(arr, 2);
        StringBuilder sb = new StringBuilder();
        for (int j = 0; j < sorted.length; j++) {
            sb.append(sorted[j]);
            if (j + 1 < sorted.length) sb.append(" ");
        }
        System.out.println(sb.toString());
    }
}
