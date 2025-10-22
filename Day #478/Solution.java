// rsync-style sync: split destination into fixed blocks indexed by a weak rolling Adler-32 checksum
// + strong FNV hash; slide a rolling window over the source to emit COPY/LITERAL tokens (only diffs
// sent), then reconstruct new = old blocks + literals. Time O(n) (constant block size).
import java.util.*;
import java.io.ByteArrayOutputStream;

public class Solution {
    static final int M = 65521, BLOCK = 4;

    static long fnv(byte[] d, int from, int len) {
        long h = 0xcbf29ce484222325L;
        for (int k = from; k < from + len; k++) { h ^= (d[k] & 0xff); h *= 0x100000001b3L; }
        return h;
    }

    static long weak(int a, int b) { return (a & 0xffffffffL) + ((long) b << 16); }

    public static void main(String[] args) {
        String oldS = "the quick brown fox jumps over the lazy dog";
        String newS = "the quick BROWN fox jumps over the lazy cat";
        byte[] oldB = oldS.getBytes(), newB = newS.getBytes();

        Map<Long, List<long[]>> idx = new HashMap<>(); // weak -> list of {blockIndex, strong}
        int nb = oldB.length / BLOCK;
        for (int bi = 0; bi < nb; bi++) {
            int s = bi * BLOCK, a = 0, b = 0;
            for (int k = s; k < s + BLOCK; k++) { a = (a + (oldB[k] & 0xff)) % M; b = (b + a) % M; }
            idx.computeIfAbsent(weak(a, b), x -> new ArrayList<>()).add(new long[]{bi, fnv(oldB, s, BLOCK)});
        }

        List<Object> tokens = new ArrayList<>(); // Integer = copy block index, byte[] = literal
        ByteArrayOutputStream lit = new ByteArrayOutputStream();
        int n = newB.length, pos = 0, a = 0, b = 0;
        boolean have = false;
        while (pos < n) {
            if (pos + BLOCK <= n) {
                if (!have) {
                    a = 0; b = 0;
                    for (int k = pos; k < pos + BLOCK; k++) { a = (a + (newB[k] & 0xff)) % M; b = (b + a) % M; }
                    have = true;
                }
                int matched = -1;
                List<long[]> cand = idx.get(weak(a, b));
                if (cand != null) {
                    long strong = fnv(newB, pos, BLOCK);
                    for (long[] c : cand) if (c[1] == strong) { matched = (int) c[0]; break; }
                }
                if (matched >= 0) {
                    if (lit.size() > 0) { tokens.add(lit.toByteArray()); lit.reset(); }
                    tokens.add(Integer.valueOf(matched));
                    pos += BLOCK; have = false; continue;
                }
                lit.write(newB[pos] & 0xff);
                if (pos + BLOCK < n) {                                  // roll forward one byte
                    int out = newB[pos] & 0xff, in = newB[pos + BLOCK] & 0xff;
                    a = ((a - out + in) % M + M) % M;
                    b = ((b - BLOCK * out + a) % M + M) % M;
                } else have = false;
                pos++;
            } else {
                lit.write(newB[pos] & 0xff); pos++;
            }
        }
        if (lit.size() > 0) tokens.add(lit.toByteArray());

        ByteArrayOutputStream out = new ByteArrayOutputStream();
        int mb = 0; long lb = 0;
        for (Object t : tokens) {
            if (t instanceof Integer) { int bi = (Integer) t; out.write(oldB, bi * BLOCK, BLOCK); mb++; }
            else { byte[] by = (byte[]) t; out.write(by, 0, by.length); lb += by.length; }
        }
        String recon = new String(out.toByteArray());
        System.out.println("Matched blocks: " + mb + " (" + (mb * BLOCK) + " bytes), Literal bytes: " + lb);
        System.out.println("Reconstructed: " + recon);
        System.out.println("Equals new file: " + recon.equals(newS));
    }
}
