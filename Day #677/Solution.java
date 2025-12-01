// Sieve of Eratosthenes for primes < N (O(N log log N)); plus an incremental
// generator using a map of next composite multiples. Space: O(N) sieve.
import java.util.*;

public class Solution {
    static List<Integer> sieve(int n) {
        boolean[] isComp = new boolean[Math.max(0, n)];
        List<Integer> primes = new ArrayList<>();
        for (int i = 2; i < n; i++) {
            if (!isComp[i]) {
                primes.add(i);
                for (long j = (long) i * i; j < n; j += i) isComp[(int) j] = true;
            }
        }
        return primes;
    }

    // Incremental generator: yields primes one-by-one via a map of next composites.
    static class PrimeGen {
        Map<Long, Long> composites = new HashMap<>();
        long current = 1;
        long next() {
            while (true) {
                current++;
                if (!composites.containsKey(current)) {
                    composites.put(current * current, current); // prime found
                    return current;
                } else {
                    long prime = composites.remove(current);
                    long x = current + prime;
                    while (composites.containsKey(x)) x += prime;
                    composites.put(x, prime);
                }
            }
        }
    }

    public static void main(String[] args) {
        System.out.println(sieve(100));
        PrimeGen gen = new PrimeGen();
        List<Long> first10 = new ArrayList<>();
        for (int i = 0; i < 10; i++) first10.add(gen.next());
        System.out.println(first10);
    }
}
