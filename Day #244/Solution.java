// Sieve of Eratosthenes: primes < N in O(N log log N) time, O(N) space.
// Plus an indefinite prime generator implemented as an Iterator using trial division by known primes.
import java.util.*;

public class Solution {
    static List<Integer> sieve(int n) {
        boolean[] comp = new boolean[Math.max(n, 0)];
        List<Integer> primes = new ArrayList<>();
        for (int i = 2; i < n; i++) {
            if (!comp[i]) {
                primes.add(i);
                for (long j = (long) i * i; j < n; j += i) comp[(int) j] = true;
            }
        }
        return primes;
    }

    // Indefinite generator as an Iterator<Long> via trial division.
    static class PrimeGen implements Iterator<Long> {
        List<Long> primes = new ArrayList<>();
        long cand = 1;
        public boolean hasNext() { return true; }
        public Long next() {
            while (true) {
                cand++;
                boolean prime = true;
                for (long p : primes) {
                    if (p * p > cand) break;
                    if (cand % p == 0) { prime = false; break; }
                }
                if (prime) { primes.add(cand); return cand; }
            }
        }
    }

    public static void main(String[] args) {
        List<Integer> p = sieve(100);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < p.size(); i++) { if (i > 0) sb.append(' '); sb.append(p.get(i)); }
        System.out.println(sb);

        PrimeGen g = new PrimeGen();
        sb = new StringBuilder();
        for (int i = 0; i < 10; i++) { if (i > 0) sb.append(' '); sb.append(g.next()); }
        System.out.println(sb);
    }
}
