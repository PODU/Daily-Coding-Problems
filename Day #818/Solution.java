// Sieve of Eratosthenes for primes below N; incremental generator via trial division by sqrt. O(N log log N) sieve.
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

    // Incremental generator: trial division of candidates against found primes up to sqrt.
    static List<Integer> firstNPrimes(int count) {
        List<Integer> primes = new ArrayList<>();
        int cand = 2;
        while (primes.size() < count) {
            boolean isPrime = true;
            for (int p : primes) {
                if ((long) p * p > cand) break;
                if (cand % p == 0) { isPrime = false; break; }
            }
            if (isPrime) primes.add(cand);
            cand++;
        }
        return primes;
    }

    public static void main(String[] args) {
        System.out.println("Primes below 30: " + sieve(30));
        System.out.println("First 10 primes: " + firstNPrimes(10));
    }
}
