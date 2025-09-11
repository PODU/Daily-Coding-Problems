// External/bucket sort demo for sorting ~1M ints in [0,1e9] with bounded memory.
// Real answer for data exceeding RAM: external merge sort (split into chunks ->
// sort each chunk on disk -> k-way merge). Here we bucket by range, sort each
// bucket, and concatenate. Time: O(n log(n/k)); Space: O(n) bounded per bucket.
import java.util.*;

public class Solution {
    static long[] bucketSort(long[] data, long maxVal, int numBuckets) {
        List<List<Long>> buckets = new ArrayList<>();
        for (int i = 0; i < numBuckets; i++) buckets.add(new ArrayList<>());
        long width = maxVal / numBuckets + 1;
        for (long x : data) {
            int b = (int) (x / width);
            if (b >= numBuckets) b = numBuckets - 1;
            buckets.get(b).add(x);
        }
        long[] out = new long[data.length];
        int idx = 0;
        for (List<Long> bk : buckets) {
            Collections.sort(bk); // each bucket fits in memory
            for (long x : bk) out[idx++] = x;
        }
        return out;
    }

    public static void main(String[] args) {
        long[] input = {5, 1, 4, 2, 8, 1000000000L, 0};
        long[] sorted = bucketSort(input, 1000000000L, 16);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < sorted.length; i++) {
            sb.append(sorted[i]);
            if (i + 1 < sorted.length) sb.append(" ");
        }
        System.out.println(sb.toString());
    }
}
