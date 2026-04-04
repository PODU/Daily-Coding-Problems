// Day 1297: Implement readN(n) on top of a read7() primitive.
// Keep a leftover buffer of unused chars; refill via read7 until n chars (or EOF). O(n) amortized.
public class Solution {
    static class Reader {
        String file;
        int pos = 0;
        Reader(String content) { file = content; }
        String read7() { // up to 7 chars, "" at EOF
            int end = Math.min(pos + 7, file.length());
            String r = file.substring(pos, end);
            pos = end;
            return r;
        }
    }

    static class NReader {
        Reader r;
        StringBuilder buf = new StringBuilder();
        NReader(Reader reader) { r = reader; }
        String readN(int n) {
            while (buf.length() < n) {
                String chunk = r.read7();
                if (chunk.isEmpty()) break;
                buf.append(chunk);
            }
            int take = Math.min(buf.length(), n);
            String out = buf.substring(0, take);
            buf.delete(0, take);
            return out;
        }
    }

    public static void main(String[] args) {
        NReader nr = new NReader(new Reader("Hello world"));
        System.out.println("'" + nr.readN(5) + "'");  // 'Hello'
        System.out.println("'" + nr.readN(4) + "'");  // ' wor'
        System.out.println("'" + nr.readN(10) + "'"); // 'ld'
    }
}
