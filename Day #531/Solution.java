// readN(n) on top of read7(): buffer leftover chars between calls.
// Time O(n) per readN call, Space O(1) extra (small buffer).
public class Solution {
    static class Reader {
        private final String content;
        private int pos = 0;            // read7 pointer
        private final StringBuilder buf = new StringBuilder();

        Reader(String content) { this.content = content; }

        String read7() {
            int end = Math.min(pos + 7, content.length());
            String r = content.substring(pos, end);
            pos = end;
            return r;
        }

        String readN(int n) {
            while (buf.length() < n) {
                String chunk = read7();
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
        Reader r = new Reader("Hello world");
        System.out.println("\"" + r.readN(7) + "\"");
        System.out.println("\"" + r.readN(7) + "\"");
        System.out.println("\"" + r.readN(7) + "\"");
    }
}
