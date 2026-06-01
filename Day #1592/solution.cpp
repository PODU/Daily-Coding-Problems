// readN using read7: buffer leftover chars from read7 between calls; pull until n satisfied or EOF.
// Time O(n) per readN call (amortized over chars read).
#include <bits/stdc++.h>
using namespace std;

struct Reader {
    string file;
    size_t pos = 0;        // internal pointer for read7
    string buf;            // leftover chars buffered between readN calls

    Reader(string f) : file(move(f)) {}

    // read7 primitive: returns up to 7 chars, advances pointer, "" at EOF
    string read7() {
        string res = file.substr(pos, 7);
        pos += res.size();
        return res;
    }

    // readN: read exactly n chars (or fewer at EOF), buffering leftovers
    string readN(int n) {
        string out;
        while ((int)out.size() < n) {
            if (buf.empty()) {
                buf = read7();
                if (buf.empty()) break; // EOF
            }
            int need = n - (int)out.size();
            int take = min(need, (int)buf.size());
            out += buf.substr(0, take);
            buf.erase(0, take);
        }
        return out;
    }
};

int main() {
    // Demonstrate read7 directly on "Hello world"
    Reader r1("Hello world");
    cout << "read7: \"" << r1.read7() << "\"\n"; // "Hello w"
    cout << "read7: \"" << r1.read7() << "\"\n"; // "orld"
    cout << "read7: \"" << r1.read7() << "\"\n"; // ""

    // Demonstrate readN on a fresh reader
    Reader r2("Hello world");
    cout << "readN(5): \"" << r2.readN(5) << "\"\n";     // "Hello"
    cout << "readN(100): \"" << r2.readN(100) << "\"\n"; // "world"
    cout << "readN(3): \"" << r2.readN(3) << "\"\n";     // ""
    return 0;
}
