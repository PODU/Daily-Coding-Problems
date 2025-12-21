// Day 774: Implement readN(n) on top of a read7() primitive.
// Buffer leftover chars from read7 between calls. O(n) per readN call.
public class Solution {
    static class Reader {
        String file; int pos = 0;
        StringBuilder buf = new StringBuilder();
        Reader(String f) { file = f; }

        String read7() {
            int end = Math.min(pos + 7, file.length());
            String r = file.substring(pos, end);
            pos = end;
            return r;
        }

        String readN(int n) {
            StringBuilder out = new StringBuilder();
            while (out.length() < n) {
                if (buf.length() == 0) {
                    String r = read7();
                    if (r.isEmpty()) break;
                    buf.append(r);
                }
                int take = Math.min(buf.length(), n - out.length());
                out.append(buf, 0, take);
                buf.delete(0, take);
            }
            return out.toString();
        }
    }

    public static void main(String[] args) {
        Reader r = new Reader("Hello world");
        System.out.print("\"" + r.readN(7) + "\", ");  // "Hello w"
        System.out.print("\"" + r.readN(7) + "\", ");  // "orld"
        System.out.println("\"" + r.readN(7) + "\"");   // ""
    }
}
