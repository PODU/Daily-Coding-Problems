// Day 826: rsync-style file sync over a low-bandwidth link.
// Receiver signs old file's fixed-size blocks (weak rolling Adler checksum +
// strong FNV-1a hash); sender rolls a window over the new file to find matching
// blocks, emitting COPY/LITERAL tokens; receiver rebuilds.
// Time O(n) average (rolling hash), Space O(n).
import java.util.*;

public class Solution {
    static final long MOD = 1 << 16;
    static final int L = 4;

    static long strongHash(byte[] b, int off, int len) {
        long h = 1469598103934665603L;
        for (int k = 0; k < len; k++) {
            h ^= (b[off + k] & 0xff);
            h *= 1099511628211L;
        }
        return h;
    }

    static long[] weakBlock(byte[] b, int off, int len) {
        long a = 0, s = 0;
        for (int k = 0; k < len; k++) {
            int v = b[off + k] & 0xff;
            a += v;
            s += (long) (len - k) * v;
        }
        a %= MOD; s %= MOD;
        return new long[]{a, s, a + MOD * s};
    }

    static class Token { boolean copy; int idx; byte[] lit; }

    public static void main(String[] args) {
        byte[] oldF = "the quick brown fox jumps".getBytes();
        byte[] newF = "the quick red fox leaps over".getBytes();

        // signature
        List<byte[]> blocks = new ArrayList<>();
        for (int i = 0; i < oldF.length; i += L)
            blocks.add(Arrays.copyOfRange(oldF, i, Math.min(i + L, oldF.length)));
        Map<Long, List<long[]>> table = new HashMap<>(); // weak -> [strong, idx]
        for (int idx = 0; idx < blocks.size(); idx++) {
            byte[] blk = blocks.get(idx);
            if (blk.length == L) {
                long w = weakBlock(blk, 0, L)[2];
                table.computeIfAbsent(w, x -> new ArrayList<>())
                     .add(new long[]{strongHash(blk, 0, L), idx});
            }
        }

        // diff
        List<Token> delta = new ArrayList<>();
        ArrayList<Byte> lit = new ArrayList<>();
        int n = newF.length, i = 0;
        long a = 0, s = 0, cs = 0; boolean have = false;
        while (i < n) {
            if (i + L <= n) {
                if (!have) { long[] w = weakBlock(newF, i, L); a = w[0]; s = w[1]; cs = w[2]; have = true; }
                boolean matched = false;
                List<long[]> bucket = table.get(cs);
                if (bucket != null) {
                    long hh = strongHash(newF, i, L);
                    for (long[] pr : bucket) {
                        if (pr[0] == hh && eq(newF, i, blocks.get((int) pr[1]))) {
                            if (!lit.isEmpty()) { delta.add(litTok(lit)); lit.clear(); }
                            Token t = new Token(); t.copy = true; t.idx = (int) pr[1];
                            delta.add(t);
                            i += L; have = false; matched = true; break;
                        }
                    }
                }
                if (matched) continue;
                lit.add(newF[i]);
                if (i + L < n) {
                    int out = newF[i] & 0xff, in = newF[i + L] & 0xff;
                    a = ((a - out + in) % MOD + MOD) % MOD;
                    s = ((s - (long) L * out + a) % MOD + MOD) % MOD;
                    cs = a + MOD * s;
                }
                i++;
            } else {
                lit.add(newF[i]); i++;
            }
        }
        if (!lit.isEmpty()) delta.add(litTok(lit));

        // reconstruct
        ByteList rebuilt = new ByteList();
        int copies = 0;
        for (Token t : delta) {
            if (t.copy) { rebuilt.add(blocks.get(t.idx)); copies++; }
            else rebuilt.add(t.lit);
        }
        byte[] result = rebuilt.toArray();
        System.out.println(new String(result));
        System.out.println("reconstruction OK: " + (Arrays.equals(result, newF) ? "True" : "False"));
        System.out.println("blocks reused (copy tokens): " + copies);
    }

    static boolean eq(byte[] arr, int off, byte[] blk) {
        if (off + blk.length > arr.length) return false;
        for (int k = 0; k < blk.length; k++) if (arr[off + k] != blk[k]) return false;
        return true;
    }

    static Token litTok(ArrayList<Byte> lit) {
        Token t = new Token(); t.copy = false;
        t.lit = new byte[lit.size()];
        for (int k = 0; k < lit.size(); k++) t.lit[k] = lit.get(k);
        return t;
    }

    static class ByteList {
        byte[] data = new byte[16]; int size = 0;
        void add(byte[] b) {
            while (size + b.length > data.length) data = Arrays.copyOf(data, data.length * 2);
            System.arraycopy(b, 0, data, size, b.length); size += b.length;
        }
        byte[] toArray() { return Arrays.copyOf(data, size); }
    }
}
