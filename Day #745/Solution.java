// Bloom filter: fixed bit array, k hash functions via double hashing.
// check() may report false positives but never false negatives.
// Time: O(k) per add/check, Space: O(m bits).
import java.util.*;

public class Solution {
    static class BloomFilter {
        boolean[] bits;
        int m, k;
        BloomFilter(int size, int numHashes){ bits = new boolean[size]; m = size; k = numHashes; }

        long[] hashes(String s){
            long h1 = 1469598103934665603L; // FNV-1a
            for(int i=0;i<s.length();i++){ h1 ^= (s.charAt(i) & 0xff); h1 *= 1099511628211L; }
            long h2 = 5381;
            for(int i=0;i<s.length();i++){ h2 = ((h2 << 5) + h2) + (s.charAt(i) & 0xff); } // djb2
            return new long[]{h1, h2};
        }
        int idx(long h1, long h2, int i){
            long v = (h1 + (long)i * h2) % m;
            if(v < 0) v += m;
            return (int)v;
        }
        void add(String s){
            long[] h = hashes(s);
            for(int i=0;i<k;i++) bits[idx(h[0], h[1], i)] = true;
        }
        boolean check(String s){
            long[] h = hashes(s);
            for(int i=0;i<k;i++) if(!bits[idx(h[0], h[1], i)]) return false;
            return true;
        }
    }

    public static void main(String[] args){
        BloomFilter bf = new BloomFilter(1000, 4);
        bf.add("apple"); bf.add("banana");
        System.out.println("apple: "  + bf.check("apple"));  // true
        System.out.println("banana: " + bf.check("banana")); // true
        System.out.println("cherry: " + bf.check("cherry")); // false
    }
}
