// rsync-style delta sync: index fixed blocks of the old file by a rolling
// (Adler-like) weak hash + exact-block strong check; slide a rolling window over
// the new file emitting COPY(block) or literal bytes. Time O(N) avg, Space O(old/B).
import java.util.*;

public class Solution {
    static final int B = 4;

    static int[] weak(byte[] s, int off, int len) {
        int a = 0, b = 0;
        for (int i = 0; i < len; i++) {
            int c = s[off + i] & 0xFF;
            a = (a + c) & 0xFFFF;
            b = (b + (len - i) * c) & 0xFFFF;
        }
        return new int[]{a, b};
    }

    static class Tok { boolean copy; int idx; int lit; }

    static List<Tok> makeDelta(byte[] old, byte[] nw) {
        Map<Integer, List<int[]>> table = new HashMap<>(); // h -> list of {idx}
        Map<Integer, List<String>> blocks = new HashMap<>();
        for (int idx = 0; idx + B <= old.length; idx += B) {
            int[] ab = weak(old, idx, B);
            int h = (ab[1] << 16) | ab[0];
            table.computeIfAbsent(h, k -> new ArrayList<>()).add(new int[]{idx});
            blocks.computeIfAbsent(h, k -> new ArrayList<>())
                  .add(new String(old, idx, B));
        }
        List<Tok> delta = new ArrayList<>();
        int n = nw.length, i = 0, a = 0, b = 0;
        if (n >= B) { int[] ab = weak(nw, 0, B); a = ab[0]; b = ab[1]; }
        while (i < n) {
            if (i + B <= n) {
                int h = (b << 16) | a, match = -1;
                List<int[]> cand = table.get(h);
                if (cand != null) {
                    String win = new String(nw, i, B);
                    List<String> bl = blocks.get(h);
                    for (int j = 0; j < cand.size(); j++)
                        if (bl.get(j).equals(win)) { match = cand.get(j)[0]; break; }
                }
                if (match >= 0) {
                    Tok t = new Tok(); t.copy = true; t.idx = match; delta.add(t);
                    i += B;
                    if (i + B <= n) { int[] ab = weak(nw, i, B); a = ab[0]; b = ab[1]; }
                    continue;
                }
            }
            Tok t = new Tok(); t.copy = false; t.lit = nw[i] & 0xFF; delta.add(t);
            if (i + B < n) {
                int out = nw[i] & 0xFF, in = nw[i + B] & 0xFF;
                a = (a - out + in) & 0xFFFF;
                b = (b - B * out + a) & 0xFFFF;
            }
            i++;
        }
        return delta;
    }

    static byte[] rebuild(byte[] old, List<Tok> delta) {
        java.io.ByteArrayOutputStream out = new java.io.ByteArrayOutputStream();
        for (Tok t : delta) {
            if (t.copy) out.write(old, t.idx, B);
            else out.write(t.lit);
        }
        return out.toByteArray();
    }

    public static void main(String[] args) {
        byte[] old = "the quick brown fox jumps over the lazy dog".getBytes();
        byte[] nw  = "the quick brown cat jumps over the lazy dog".getBytes();
        List<Tok> delta = makeDelta(old, nw);
        int copies = 0, lits = 0;
        for (Tok t : delta) { if (t.copy) copies++; else lits++; }
        String rb = new String(rebuild(old, delta));
        System.out.println("Reconstructed: " + rb);
        System.out.println("Match: " + rb.equals(new String(nw)));
        System.out.println("copied blocks: " + copies + " literal bytes: " + lits);
    }
}
