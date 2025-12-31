// Day 826: rsync-style file sync over a low-bandwidth link.
// Receiver signs old file's fixed-size blocks (weak rolling Adler checksum +
// strong FNV-1a hash); sender rolls a window over the new file to find matching
// blocks, emitting COPY/LITERAL tokens; receiver rebuilds.
// Time O(n) average (rolling hash), Space O(n).
#include <bits/stdc++.h>
using namespace std;

static const long long MOD = 1 << 16;
static const int L = 4;

unsigned long long strongHash(const string &b) {
    unsigned long long h = 1469598103934665603ULL;
    for (unsigned char x : b) { h ^= x; h *= 1099511628211ULL; }
    return h;
}

long long weakBlock(const string &b, long long &a, long long &s) {
    a = 0; s = 0;
    int len = (int)b.size();
    for (int k = 0; k < len; k++) {
        a += (unsigned char)b[k];
        s += (long long)(len - k) * (unsigned char)b[k];
    }
    a %= MOD; s %= MOD;
    return a + MOD * s;
}

struct Token { bool copy; int idx; string lit; };

int main() {
    string oldF = "the quick brown fox jumps";
    string newF = "the quick red fox leaps over";

    // signature
    vector<string> blocks;
    for (size_t i = 0; i < oldF.size(); i += L) blocks.push_back(oldF.substr(i, L));
    unordered_map<long long, vector<pair<unsigned long long,int>>> table;
    for (int idx = 0; idx < (int)blocks.size(); idx++) {
        if ((int)blocks[idx].size() == L) {
            long long a, s; long long w = weakBlock(blocks[idx], a, s);
            table[w].push_back({strongHash(blocks[idx]), idx});
        }
    }

    // diff
    vector<Token> delta;
    string lit;
    int n = (int)newF.size(), i = 0;
    long long a = 0, s = 0, cs = 0; bool have = false;
    while (i < n) {
        if (i + L <= n) {
            if (!have) { cs = weakBlock(newF.substr(i, L), a, s); have = true; }
            bool matched = false;
            auto it = table.find(cs);
            if (it != table.end()) {
                string win = newF.substr(i, L);
                unsigned long long hh = strongHash(win);
                for (auto &pr : it->second)
                    if (pr.first == hh && blocks[pr.second] == win) {
                        if (!lit.empty()) { delta.push_back({false, -1, lit}); lit.clear(); }
                        delta.push_back({true, pr.second, ""});
                        i += L; have = false; matched = true; break;
                    }
            }
            if (matched) continue;
            lit.push_back(newF[i]);
            if (i + L < n) {
                a = ((a - (unsigned char)newF[i] + (unsigned char)newF[i + L]) % MOD + MOD) % MOD;
                s = ((s - (long long)L * (unsigned char)newF[i] + a) % MOD + MOD) % MOD;
                cs = a + MOD * s;
            }
            i++;
        } else {
            lit.push_back(newF[i]); i++;
        }
    }
    if (!lit.empty()) delta.push_back({false, -1, lit});

    // reconstruct
    string rebuilt;
    int copies = 0;
    for (auto &t : delta) {
        if (t.copy) { rebuilt += blocks[t.idx]; copies++; }
        else rebuilt += t.lit;
    }

    cout << rebuilt << "\n";
    cout << "reconstruction OK: " << (rebuilt == newF ? "True" : "False") << "\n";
    cout << "blocks reused (copy tokens): " << copies << "\n";
    return 0;
}
