// Day 59: File sync over low-bandwidth link, rsync-style.
// Receiver sends per-block (weak rolling + strong) checksums of its OLD file;
// sender scans NEW file with a rolling checksum, emitting COPY(block) tokens for
// matches and LITERAL bytes otherwise -> only differing bytes cross the wire.
// Time: O(n) expected, bandwidth ~ size of the changes.
import java.util.*;

public class Solution {
    static final int M = 1 << 16;
    static final int B = 4;

    static long fnv1a(byte[] d, int s, int e) {
        long h = 1469598103934665603L;
        for (int i = s; i < e; i++) { h ^= (d[i] & 0xFF); h *= 1099511628211L; }
        return h;
    }

    static int[] weakFull(byte[] d, int s, int e) {
        int a = 0, b = 0;
        for (int i = s; i < e; i++) {
            a = (a + (d[i] & 0xFF)) % M;
            b = (b + (e - i) * (d[i] & 0xFF)) % M;
        }
        return new int[]{a, b};
    }

    static class Token { boolean copy; int idx; byte[] lit; }

    // Receiver side: weak -> (strong -> index).
    static Map<Integer, Map<Long, Integer>> signatures(byte[] old) {
        Map<Integer, Map<Long, Integer>> sigs = new HashMap<>();
        int n = old.length / B;
        for (int i = 0; i < n; i++) {
            int[] ab = weakFull(old, i * B, i * B + B);
            int weak = (ab[1] << 16) | ab[0];
            sigs.computeIfAbsent(weak, k -> new HashMap<>())
                .put(fnv1a(old, i * B, i * B + B), i);
        }
        return sigs;
    }

    static List<Token> diff(byte[] nw, Map<Integer, Map<Long, Integer>> sigs) {
        List<Token> tokens = new ArrayList<>();
        List<Byte> literal = new ArrayList<>();
        int pos = 0, n = nw.length, a = 0, b = 0;
        boolean haveWindow = false;
        while (pos + B <= n) {
            if (!haveWindow) { int[] ab = weakFull(nw, pos, pos + B); a = ab[0]; b = ab[1]; haveWindow = true; }
            int weak = (b << 16) | a;
            Integer idx = null;
            Map<Long, Integer> m = sigs.get(weak);
            if (m != null) idx = m.get(fnv1a(nw, pos, pos + B));
            if (idx != null) {
                if (!literal.isEmpty()) { tokens.add(litToken(literal)); literal = new ArrayList<>(); }
                Token t = new Token(); t.copy = true; t.idx = idx; tokens.add(t);
                pos += B; haveWindow = false;
            } else {
                int first = nw[pos] & 0xFF;
                literal.add(nw[pos]);
                a = ((a - first + (nw[pos + B] & 0xFF)) % M + M) % M;
                b = ((b - B * first + a) % M + M) % M;
                pos++;
            }
        }
        for (int i = pos; i < n; i++) literal.add(nw[i]);
        if (!literal.isEmpty()) tokens.add(litToken(literal));
        return tokens;
    }

    static Token litToken(List<Byte> literal) {
        Token t = new Token(); t.copy = false;
        t.lit = new byte[literal.size()];
        for (int i = 0; i < literal.size(); i++) t.lit[i] = literal.get(i);
        return t;
    }

    static byte[] applyDelta(byte[] old, List<Token> tokens) {
        java.io.ByteArrayOutputStream out = new java.io.ByteArrayOutputStream();
        for (Token t : tokens) {
            if (t.copy) out.write(old, t.idx * B, B);
            else out.write(t.lit, 0, t.lit.length);
        }
        return out.toByteArray();
    }

    public static void main(String[] args) {
        byte[] old = "the quick brown fox jumps over the lazy dog".getBytes();
        byte[] nw = "the quick brown cat jumps over the lazy dog".getBytes();
        Map<Integer, Map<Long, Integer>> sigs = signatures(old);
        List<Token> tokens = diff(nw, sigs);
        byte[] rebuilt = applyDelta(old, tokens);
        int literalBytes = 0, copied = 0;
        for (Token t : tokens) { if (t.copy) copied++; else literalBytes += t.lit.length; }
        System.out.println("synced: " + Arrays.equals(rebuilt, nw));
        System.out.println("literal bytes sent: " + literalBytes + " of " + nw.length);
        System.out.println("blocks reused: " + copied);
    }
}
