// Day 1674: Low-bandwidth file sync (rsync algorithm).
// Receiver sends per-block weak(rolling)+strong checksums; sender emits COPY/LITERAL
// tokens using a rolling hash to find matches. Time O(n) amortized, transfers only diffs.
import java.util.*;

public class Solution {
    static final int M = 1 << 16;

    static int[] weakInit(byte[] d, int i, int L) {
        long a = 0, b = 0;
        for (int k = i; k < i + L; k++) {
            int v = d[k] & 0xFF;
            a = (a + v) % M;
            b = (b + (long) (L - (k - i)) * v) % M;
        }
        return new int[]{(int) a, (int) b};
    }
    static int[] weakRoll(int a, int b, byte[] d, int i, int L) {
        int out = d[i] & 0xFF, in = d[i + L] & 0xFF;
        a = ((a - out + in) % M + M) % M;
        b = (int) (((b - (long) L * out + a) % M + M) % M);
        return new int[]{a, b};
    }
    static long strong(byte[] s, int i, int L) {
        long h = 1469598103934665603L;
        for (int k = i; k < i + L; k++) { h ^= (s[k] & 0xFF); h *= 1099511628211L; }
        return h;
    }

    static class Tok { boolean copy; int idx; byte[] lit; }

    static List<Tok> diff(byte[] old, byte[] neu, int L) {
        Map<Long, Map<Long, Integer>> table = new HashMap<>();
        int nblocks = old.length / L;
        for (int idx = 0; idx < nblocks; idx++) {
            int[] ab = weakInit(old, idx * L, L);
            long w = ((long) ab[1] << 16) | ab[0];
            table.computeIfAbsent(w, x -> new HashMap<>()).put(strong(old, idx * L, L), idx);
        }
        List<Tok> tokens = new ArrayList<>();
        List<Byte> lit = new ArrayList<>();
        int i = 0, n = neu.length, a = 0, b = 0; boolean have = false;
        while (i < n) {
            if (i + L <= n) {
                if (!have) { int[] ab = weakInit(neu, i, L); a = ab[0]; b = ab[1]; have = true; }
                long w = ((long) b << 16) | a;
                Map<Long, Integer> bucket = table.get(w);
                if (bucket != null) {
                    Integer idx = bucket.get(strong(neu, i, L));
                    if (idx != null) {
                        if (!lit.isEmpty()) { tokens.add(litTok(lit)); lit.clear(); }
                        Tok t = new Tok(); t.copy = true; t.idx = idx; tokens.add(t);
                        i += L; have = false; continue;
                    }
                }
                lit.add(neu[i]);
                if (i + L <= n - 1) { int[] ab = weakRoll(a, b, neu, i, L); a = ab[0]; b = ab[1]; }
                else have = false;
                i++;
            } else { lit.add(neu[i]); i++; }
        }
        if (!lit.isEmpty()) tokens.add(litTok(lit));
        return tokens;
    }
    static Tok litTok(List<Byte> lit) {
        Tok t = new Tok(); t.copy = false; t.lit = new byte[lit.size()];
        for (int j = 0; j < lit.size(); j++) t.lit[j] = lit.get(j);
        return t;
    }
    static byte[] patch(byte[] old, List<Tok> tokens, int L) {
        java.io.ByteArrayOutputStream out = new java.io.ByteArrayOutputStream();
        for (Tok t : tokens) {
            if (t.copy) out.write(old, t.idx * L, L);
            else out.write(t.lit, 0, t.lit.length);
        }
        return out.toByteArray();
    }

    public static void main(String[] args) {
        int L = 5;
        byte[] old = "the quick brown fox jumps over".getBytes();
        byte[] neu = "the quick brown cat jumps over".getBytes();
        List<Tok> tokens = diff(old, neu, L);
        byte[] rebuilt = patch(old, tokens, L);
        int litBytes = 0;
        for (Tok t : tokens) if (!t.copy) litBytes += t.lit.length;
        System.out.println(Arrays.equals(rebuilt, neu)); // true
        System.out.println(litBytes + " of " + neu.length);
    }
}
