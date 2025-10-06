// Sliding window median via two heaps with lazy deletion.
// Time: O(n log k), Space: O(k).
import java.util.*;

public class Solution {
    static class DualHeap {
        PriorityQueue<Integer> small = new PriorityQueue<>(Collections.reverseOrder()); // max
        PriorityQueue<Integer> large = new PriorityQueue<>();                            // min
        Map<Integer,Integer> delayed = new HashMap<>();
        int ss = 0, ls = 0;

        void prune(PriorityQueue<Integer> heap){
            while(!heap.isEmpty()){
                int num = heap.peek();
                Integer cnt = delayed.get(num);
                if(cnt != null && cnt > 0){
                    if(cnt == 1) delayed.remove(num); else delayed.put(num, cnt-1);
                    heap.poll();
                } else break;
            }
        }
        void balance(){
            if(ss > ls + 1){ large.offer(small.poll()); ss--; ls++; prune(small); }
            else if(ss < ls){ small.offer(large.poll()); ls--; ss++; prune(large); }
        }
        void insert(int num){
            if(small.isEmpty() || num <= small.peek()){ small.offer(num); ss++; }
            else { large.offer(num); ls++; }
            balance();
        }
        void erase(int num){
            delayed.merge(num, 1, Integer::sum);
            if(num <= small.peek()){ ss--; if(num == small.peek()) prune(small); }
            else { ls--; if(num == large.peek()) prune(large); }
            balance();
        }
        double median(int k){
            if(k % 2 == 1) return small.peek();
            return (small.peek() + (long)large.peek()) / 2.0;
        }
    }

    static String fmt(double d){
        if(d == Math.floor(d)) return Long.toString((long)d);
        return Double.toString(d);
    }

    public static void main(String[] args){
        int[] arr = {-1,5,13,8,2,3,3,1};
        int k = 3;
        DualHeap dh = new DualHeap();
        for(int i=0;i<k;i++) dh.insert(arr[i]);
        int n = arr.length;
        for(int i=k;i<=n;i++){
            StringBuilder win = new StringBuilder("[");
            for(int j=i-k;j<i;j++){ if(j>i-k) win.append(", "); win.append(arr[j]); }
            win.append("]");
            System.out.println(fmt(dh.median(k)) + " <- median of " + win);
            if(i < n){ dh.insert(arr[i]); dh.erase(arr[i-k]); }
        }
    }
}
