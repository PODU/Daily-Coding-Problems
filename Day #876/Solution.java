// Sort a k-sorted array with a min-heap of size k+1. Time O(N log k), Space O(k).
import java.util.*;

public class Solution {
    static int[] sortKSorted(int[] a, int k){
        PriorityQueue<Integer> heap = new PriorityQueue<>();
        int[] res = new int[a.length];
        int idx = 0;
        for(int x : a){
            heap.add(x);
            if(heap.size() > k) res[idx++] = heap.poll();
        }
        while(!heap.isEmpty()) res[idx++] = heap.poll();
        return res;
    }

    public static void main(String[] args){
        int[] a = {6, 5, 3, 2, 8, 10, 9};
        int k = 3;
        System.out.println(Arrays.toString(sortKSorted(a, k)));
    }
}
