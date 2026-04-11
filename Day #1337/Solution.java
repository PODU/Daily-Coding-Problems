// Sieve of Eratosthenes: mark multiples of each prime from p*p. O(N log log N) time, O(N) space.
// Bonus: an incremental prime generator (Iterator) yielding primes indefinitely.
import java.util.*;

public class Solution {
    static List<Integer> sieve(int n) {
        boolean[] composite = new boolean[Math.max(n, 2)];
        List<Integer> primes = new ArrayList<>();
        for (int p = 2; p < n; ++p) {
            if (!composite[p]) {
                primes.add(p);
                for (long m = (long) p * p; m < n; m += p) composite[(int) m] = true;
            }
        }
        return primes;
    }

    // Incremental sieve generator producing primes indefinitely.
    static class PrimeGenerator implements Iterator<Long> {
        Map<Long, List<Long>> composites = new HashMap<>();
        long candidate = 1;
        public boolean hasNext() { return true; }
        public Long next() {
            while (true) {
                candidate++;
                List<Long> factors = composites.remove(candidate);
                if (factors == null) {
                    composites.computeIfAbsent(candidate * candidate, k -> new ArrayList<>()).add(candidate);
                    return candidate;
                } else {
                    for (long p : factors)
                        composites.computeIfAbsent(candidate + p, k -> new ArrayList<>()).add(p);
                }
            }
        }
    }

    public static void main(String[] args) {
        List<Integer> primes = sieve(100);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < primes.size(); ++i) {
            if (i > 0) sb.append(' ');
            sb.append(primes.get(i));
        }
        System.out.println(sb);

        PrimeGenerator gen = new PrimeGenerator();
        StringBuilder sb2 = new StringBuilder();
        for (int i = 0; i < 10; ++i) {
            if (i > 0) sb2.append(' ');
            sb2.append(gen.next());
        }
        System.out.println(sb2);
    }
}
