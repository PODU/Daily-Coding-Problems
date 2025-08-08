// Day 82: readN(n) built on read7() by buffering leftover characters between calls.
// Time O(n) per call amortized, Space O(7) buffer.
public class Solution {
    static class Reader {
        String file;
        int pos = 0;
        StringBuilder buffer = new StringBuilder();

        Reader(String f) { file = f; }

        // Returns up to 7 characters from the file, advancing the cursor.
        String read7() {
            int end = Math.min(pos + 7, file.length());
            String chunk = file.substring(pos, end);
            pos = end;
            return chunk;
        }

        String readN(int n) {
            StringBuilder result = new StringBuilder();
            while (result.length() < n) {
                if (buffer.length() == 0) {
                    String chunk = read7();
                    if (chunk.isEmpty()) break; // EOF
                    buffer.append(chunk);
                }
                int take = Math.min(buffer.length(), n - result.length());
                result.append(buffer, 0, take);
                buffer.delete(0, take);
            }
            return result.toString();
        }
    }

    public static void main(String[] args) {
        Reader r = new Reader("Hello world");
        System.out.println("\"" + r.readN(7) + "\""); // "Hello w"
        System.out.println("\"" + r.readN(7) + "\""); // "orld"
        System.out.println("\"" + r.readN(7) + "\""); // ""
    }
}
