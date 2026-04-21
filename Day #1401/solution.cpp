// rsync-style delta sync: index fixed blocks of the old file by a rolling
// (Adler-like) weak hash + exact-block strong check; slide a rolling window over
// the new file emitting COPY(block) or literal bytes. Time O(N) avg, Space O(old/B).
#include <bits/stdc++.h>
using namespace std;
static const int B = 4;

pair<int,int> weak(const string& s, int off, int len) {
    int a = 0, b = 0;
    for (int i = 0; i < len; i++) {
        unsigned char c = s[off + i];
        a = (a + c) & 0xFFFF;
        b = (b + (len - i) * c) & 0xFFFF;
    }
    return {a, b};
}

struct Tok { bool copy; int idx; unsigned char lit; };

vector<Tok> makeDelta(const string& old_, const string& nw) {
    unordered_map<int, vector<pair<int,string>>> table;
    for (int idx = 0; idx + B <= (int)old_.size(); idx += B) {
        string blk = old_.substr(idx, B);
        pair<int,int> ab = weak(old_, idx, B);
        int a = ab.first, b = ab.second;
        table[(b << 16) | a].push_back({idx, blk});
    }
    vector<Tok> delta;
    int n = nw.size(), i = 0, a = 0, b = 0;
    if (n >= B) { auto p = weak(nw, 0, B); a = p.first; b = p.second; }
    while (i < n) {
        if (i + B <= n) {
            int h = (b << 16) | a, match = -1;
            auto it = table.find(h);
            if (it != table.end())
                for (auto& pr : it->second)
                    if (pr.second == nw.substr(i, B)) { match = pr.first; break; }
            if (match >= 0) {
                delta.push_back({true, match, 0});
                i += B;
                if (i + B <= n) { auto p = weak(nw, i, B); a = p.first; b = p.second; }
                continue;
            }
        }
        delta.push_back({false, 0, (unsigned char)nw[i]});
        if (i + B < n) {
            unsigned char out = nw[i], in = nw[i + B];
            a = (a - out + in) & 0xFFFF;
            b = (b - B * out + a) & 0xFFFF;
        }
        i++;
    }
    return delta;
}

string rebuild(const string& old_, const vector<Tok>& delta) {
    string out;
    for (auto& t : delta) {
        if (t.copy) out += old_.substr(t.idx, B);
        else out += (char)t.lit;
    }
    return out;
}

int main() {
    string old_ = "the quick brown fox jumps over the lazy dog";
    string nw   = "the quick brown cat jumps over the lazy dog";
    auto delta = makeDelta(old_, nw);
    int copies = 0, lits = 0;
    for (auto& t : delta) (t.copy ? copies : lits)++;
    string rb = rebuild(old_, delta);
    cout << "Reconstructed: " << rb << "\n";
    cout << "Match: " << (rb == nw ? "true" : "false") << "\n";
    cout << "copied blocks: " << copies << " literal bytes: " << lits << "\n";
    return 0;
}
