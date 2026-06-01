// readN using read7: buffer leftover chars from read7 between calls; pull until n satisfied or EOF.
// Time O(n) per readN call.
public class Solution {
    static class Reader {
        private final String file;
        private int pos = 0;            // internal pointer for read7
        private StringBuilder buf = new StringBuilder(); // leftover between readN calls

        Reader(String f) { this.file = f; }

        // read7 primitive: returns up to 7 chars, advances pointer, "" at EOF
        String read7() {
            int end = Math.min(pos + 7, file.length());
            String res = file.substring(pos, end);
            pos = end;
            return res;
        }

        // readN: read exactly n chars (or fewer at EOF), buffering leftovers
        String readN(int n) {
            StringBuilder out = new StringBuilder();
            while (out.length() < n) {
                if (buf.length() == 0) {
                    String chunk = read7();
                    if (chunk.isEmpty()) break; // EOF
                    buf.append(chunk);
                }
                int take = Math.min(n - out.length(), buf.length());
                out.append(buf, 0, take);
                buf.delete(0, take);
            }
            return out.toString();
        }
    }

    public static void main(String[] args) {
        Reader r1 = new Reader("Hello world");
        System.out.println("read7: \"" + r1.read7() + "\"");
        System.out.println("read7: \"" + r1.read7() + "\"");
        System.out.println("read7: \"" + r1.read7() + "\"");

        Reader r2 = new Reader("Hello world");
        System.out.println("readN(5): \"" + r2.readN(5) + "\"");
        System.out.println("readN(100): \"" + r2.readN(100) + "\"");
        System.out.println("readN(3): \"" + r2.readN(3) + "\"");
    }
}
