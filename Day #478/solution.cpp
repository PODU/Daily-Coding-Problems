// rsync-style sync: split destination into fixed blocks indexed by a weak rolling Adler-32 checksum
// + strong FNV hash; slide a rolling window over the source to emit COPY/LITERAL tokens (only diffs
// sent), then reconstruct new = old blocks + literals. Time O(n) (constant block size).
#include <bits/stdc++.h>
using namespace std;

static const int M = 65521, BLOCK = 4;

unsigned long long fnv(const string& d, int from, int len) {
    unsigned long long h = 1469598103934665603ULL; // FNV offset basis
    for (int k = from; k < from + len; k++) {
        h ^= (unsigned char)d[k];
        h *= 1099511628211ULL;                      // FNV prime
    }
    return h;
}

struct Token { int copy; string lit; }; // copy >= 0: copy block; copy == -1: literal

int main() {
    string oldS = "the quick brown fox jumps over the lazy dog";
    string newS = "the quick BROWN fox jumps over the lazy cat";

    unordered_map<unsigned long long, vector<pair<int, unsigned long long>>> idx;
    int nb = oldS.size() / BLOCK;
    for (int bi = 0; bi < nb; bi++) {
        int s = bi * BLOCK, a = 0, b = 0;
        for (int k = s; k < s + BLOCK; k++) { a = (a + (unsigned char)oldS[k]) % M; b = (b + a) % M; }
        unsigned long long weak = (unsigned long long)a + ((unsigned long long)b << 16);
        idx[weak].push_back({bi, fnv(oldS, s, BLOCK)});
    }

    vector<Token> tokens;
    string lit;
    int n = newS.size(), pos = 0, a = 0, b = 0;
    bool have = false;
    while (pos < n) {
        if (pos + BLOCK <= n) {
            if (!have) {
                a = 0; b = 0;
                for (int k = pos; k < pos + BLOCK; k++) { a = (a + (unsigned char)newS[k]) % M; b = (b + a) % M; }
                have = true;
            }
            unsigned long long weak = (unsigned long long)a + ((unsigned long long)b << 16);
            int matched = -1;
            auto it = idx.find(weak);
            if (it != idx.end()) {
                unsigned long long strong = fnv(newS, pos, BLOCK);
                for (auto& c : it->second) if (c.second == strong) { matched = c.first; break; }
            }
            if (matched >= 0) {
                if (!lit.empty()) { tokens.push_back({-1, lit}); lit.clear(); }
                tokens.push_back({matched, ""});
                pos += BLOCK; have = false; continue;
            }
            lit.push_back(newS[pos]);
            if (pos + BLOCK < n) {                                  // roll forward one byte
                int out = (unsigned char)newS[pos], in = (unsigned char)newS[pos + BLOCK];
                a = ((a - out + in) % M + M) % M;
                b = ((b - BLOCK * out + a) % M + M) % M;
            } else have = false;
            pos++;
        } else {
            lit.push_back(newS[pos]); pos++;
        }
    }
    if (!lit.empty()) tokens.push_back({-1, lit});

    string out;
    int mb = 0; long lb = 0;
    for (auto& t : tokens) {
        if (t.copy >= 0) { out += oldS.substr(t.copy * BLOCK, BLOCK); mb++; }
        else { out += t.lit; lb += t.lit.size(); }
    }
    cout << "Matched blocks: " << mb << " (" << mb * BLOCK << " bytes), Literal bytes: " << lb << "\n";
    cout << "Reconstructed: " << out << "\n";
    cout << "Equals new file: " << (out == newS ? "true" : "false") << "\n";
}
