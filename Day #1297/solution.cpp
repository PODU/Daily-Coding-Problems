// Day 1297: Implement readN(n) on top of a read7() primitive.
// Keep a leftover buffer of unused chars; refill via read7 until n chars (or EOF). O(n) amortized.
#include <bits/stdc++.h>
using namespace std;

class Reader {
    string file;
    size_t pos = 0;
public:
    Reader(const string& content) : file(content) {}
    // returns up to 7 characters, "" at EOF
    string read7() {
        string r = file.substr(pos, 7);
        pos += r.size();
        return r;
    }
};

class NReader {
    Reader& r;
    string buf; // leftover chars not yet returned
public:
    NReader(Reader& reader) : r(reader) {}
    string readN(int n) {
        while ((int)buf.size() < n) {
            string chunk = r.read7();
            if (chunk.empty()) break; // EOF
            buf += chunk;
        }
        int take = min((int)buf.size(), n);
        string out = buf.substr(0, take);
        buf.erase(0, take);
        return out;
    }
};

int main() {
    Reader reader("Hello world");
    NReader nr(reader);
    cout << "'" << nr.readN(5) << "'" << endl;  // 'Hello'
    cout << "'" << nr.readN(4) << "'" << endl;  // ' wor'
    cout << "'" << nr.readN(10) << "'" << endl; // 'ld'
    return 0;
}
