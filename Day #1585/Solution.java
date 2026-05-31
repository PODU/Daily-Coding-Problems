// Day 1585: Sieve of Eratosthenes + incremental infinite prime generator.
// Sieve marks composites up to N; generator keeps a map of next composite per prime.
// Time: O(N log log N) sieve; Space: O(N).
import java.util.*;

public class Solution {
    static List<Integer> sieve(int n) {
        boolean[] comp = new boolean[n];
        List<Integer> primes = new ArrayList<>();
        for (int i = 2; i < n; i++) {
            if (!comp[i]) {
                primes.add(i);
                for (long j = (long) i * i; j < n; j += i) comp[(int) j] = true;
            }
        }
        return primes;
    }

    static List<Integer> firstPrimes(int count) {
        List<Integer> primes = new ArrayList<>();
        Map<Long, List<Long>> composites = new HashMap<>();
        for (long n = 2; primes.size() < count; n++) {
            List<Long> hit = composites.remove(n);
            if (hit == null) {
                primes.add((int) n);
                composites.computeIfAbsent(n * n, k -> new ArrayList<>()).add(n);
            } else {
                for (long p : hit) composites.computeIfAbsent(n + p, k -> new ArrayList<>()).add(p);
            }
        }
        return primes;
    }

    public static void main(String[] args) {
        System.out.println("Primes < 30: " + sieve(30));
        System.out.println("First 10 primes: " + firstPrimes(10));
    }
}
