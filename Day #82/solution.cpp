// Day 82: readN(n) built on read7() by buffering leftover characters between calls.
// Time O(n) per call amortized, Space O(7) buffer.
#include <bits/stdc++.h>
using namespace std;

struct Reader {
    string file;
    size_t pos = 0;
    string buffer;            // leftover characters from previous read7 calls

    Reader(string f) : file(move(f)) {}

    // Returns up to 7 characters from the file, advancing the cursor.
    string read7() {
        string chunk = file.substr(pos, 7);
        pos += chunk.size();
        return chunk;
    }

    string readN(int n) {
        string result;
        while ((int)result.size() < n) {
            if (buffer.empty()) {
                string chunk = read7();
                if (chunk.empty()) break;   // EOF
                buffer += chunk;
            }
            int take = min((int)buffer.size(), n - (int)result.size());
            result += buffer.substr(0, take);
            buffer.erase(0, take);
        }
        return result;
    }
};

int main() {
    Reader r("Hello world");
    cout << "\"" << r.readN(7) << "\"\n"; // "Hello w"
    cout << "\"" << r.readN(7) << "\"\n"; // "orld"
    cout << "\"" << r.readN(7) << "\"\n"; // ""
    return 0;
}
