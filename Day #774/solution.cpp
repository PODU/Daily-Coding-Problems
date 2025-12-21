// Day 774: Implement readN(n) on top of a read7() primitive.
// Buffer leftover chars from read7 between calls. O(n) per readN call.
#include <bits/stdc++.h>
using namespace std;

struct Reader {
    string file; size_t pos = 0;            // simulated file + cursor
    string buf;                              // leftover from read7
    Reader(string f) : file(move(f)) {}

    string read7() {                          // returns up to 7 chars
        string r = file.substr(pos, 7);
        pos += r.size();
        return r;
    }

    string readN(int n) {
        string out;
        while ((int)out.size() < n) {
            if (buf.empty()) {
                buf = read7();
                if (buf.empty()) break;       // EOF
            }
            int take = min((int)buf.size(), n - (int)out.size());
            out += buf.substr(0, take);
            buf.erase(0, take);
        }
        return out;
    }
};

int main() {
    Reader r("Hello world");
    cout << "\"" << r.readN(7) << "\", ";  // "Hello w"
    cout << "\"" << r.readN(7) << "\", ";  // "orld"
    cout << "\"" << r.readN(7) << "\"\n";  // ""
    return 0;
}
