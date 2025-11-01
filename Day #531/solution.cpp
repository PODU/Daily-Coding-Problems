// readN(n) on top of read7(): buffer leftover chars between calls.
// Time O(n) per readN call, Space O(1) extra (small buffer).
#include <bits/stdc++.h>
using namespace std;

struct Reader {
    string content;
    size_t pos = 0;     // read7 pointer
    string buf;         // leftover unconsumed chars

    string read7() {
        string r = content.substr(pos, 7);
        pos += r.size();
        return r;
    }

    string readN(int n) {
        while ((int)buf.size() < n) {
            string chunk = read7();
            if (chunk.empty()) break;
            buf += chunk;
        }
        int take = min((int)buf.size(), n);
        string out = buf.substr(0, take);
        buf.erase(0, take);
        return out;
    }
};

int main() {
    Reader r{"Hello world"};
    cout << "\"" << r.readN(7) << "\"\n";
    cout << "\"" << r.readN(7) << "\"\n";
    cout << "\"" << r.readN(7) << "\"\n";
    return 0;
}
