// Day 369: Stock price tracker.
// ts2price maps timestamp->price; a TreeMap<price,count> multiset gives min/max in
// O(log n), running sum+count give average in O(1). add/update/remove O(log n).
import java.util.*;

public class Solution {
    static class StockTracker {
        Map<Long, Integer> ts2price = new HashMap<>();
        TreeMap<Integer, Integer> prices = new TreeMap<>();
        long sum = 0;
        int count = 0;

        void add(long ts, int price) {
            ts2price.put(ts, price);
            prices.merge(price, 1, Integer::sum);
            sum += price; count++;
        }
        void update(long ts, int price) { remove(ts); add(ts, price); }
        void remove(long ts) {
            Integer price = ts2price.remove(ts);
            if (price == null) return;
            int c = prices.get(price);
            if (c == 1) prices.remove(price); else prices.put(price, c - 1);
            sum -= price; count--;
        }
        int maxPrice() { return prices.lastKey(); }
        int minPrice() { return prices.firstKey(); }
        double average() { return (double) sum / count; }
    }

    public static void main(String[] args) {
        StockTracker s = new StockTracker();
        s.add(1, 100); s.add(2, 200); s.add(3, 150);
        System.out.printf("max=%d min=%d avg=%.1f%n", s.maxPrice(), s.minPrice(), s.average());
        s.update(2, 50);
        System.out.printf("max=%d min=%d avg=%.1f%n", s.maxPrice(), s.minPrice(), s.average());
        s.remove(3);
        System.out.printf("max=%d min=%d avg=%.1f%n", s.maxPrice(), s.minPrice(), s.average());
    }
}
