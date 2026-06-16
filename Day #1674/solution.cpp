// Day 1674: Low-bandwidth file sync (rsync algorithm).
// Receiver sends per-block weak(rolling)+strong checksums; sender emits COPY/LITERAL
// tokens using a rolling hash to find matches. Time O(n) amortized, transfers only diffs.
#include <bits/stdc++.h>
using namespace std;

const int M = 1 << 16;

pair<int,int> weakInit(const string& d, int i, int L) {
    long a = 0, b = 0;
    for (int k = i; k < i + L; ++k) {
        a = (a + (unsigned char)d[k]) % M;
        b = (b + (long)(L - (k - i)) * (unsigned char)d[k]) % M;
    }
    return {(int)a, (int)b};
}
pair<int,int> weakRoll(int a, int b, const string& d, int i, int L) {
    a = ((a - (unsigned char)d[i] + (unsigned char)d[i + L]) % M + M) % M;
    b = ((b - (long)L * (unsigned char)d[i] + a) % M + M) % M;
    return {a, b};
}
unsigned long long strongHash(const string& s, int i, int L) {
    unsigned long long h = 1469598103934665603ULL;
    for (int k = i; k < i + L; ++k) { h ^= (unsigned char)s[k]; h *= 1099511628211ULL; }
    return h;
}

struct Tok { bool copy; int idx; string lit; };

vector<Tok> diff(const string& old, const string& neu, int L) {
    unordered_map<long long, unordered_map<unsigned long long,int>> table;
    int nblocks = old.size() / L;
    for (int idx = 0; idx < nblocks; ++idx) {
        auto ab = weakInit(old, idx * L, L);
        int a = ab.first, b = ab.second;
        table[((long long)b << 16) | a][strongHash(old, idx * L, L)] = idx;
    }
    vector<Tok> tokens; string lit; int i = 0, n = neu.size(), a = 0, b = 0; bool have = false;
    while (i < n) {
        if (i + L <= n) {
            if (!have) { auto p = weakInit(neu, i, L); a = p.first; b = p.second; have = true; }
            long long w = ((long long)b << 16) | a;
            auto it = table.find(w);
            if (it != table.end()) {
                auto s = strongHash(neu, i, L);
                auto it2 = it->second.find(s);
                if (it2 != it->second.end()) {
                    if (!lit.empty()) { tokens.push_back({false, 0, lit}); lit.clear(); }
                    tokens.push_back({true, it2->second, ""});
                    i += L; have = false; continue;
                }
            }
            lit.push_back(neu[i]);
            if (i + L <= n - 1) { auto p = weakRoll(a, b, neu, i, L); a = p.first; b = p.second; }
            else have = false;
            ++i;
        } else { lit.push_back(neu[i]); ++i; }
    }
    if (!lit.empty()) tokens.push_back({false, 0, lit});
    return tokens;
}
string patch(const string& old, const vector<Tok>& tokens, int L) {
    string out;
    for (auto& t : tokens) {
        if (t.copy) out += old.substr(t.idx * L, L);
        else out += t.lit;
    }
    return out;
}

int main() {
    int L = 5;
    string old = "the quick brown fox jumps over";
    string neu = "the quick brown cat jumps over";
    auto tokens = diff(old, neu, L);
    string rebuilt = patch(old, tokens, L);
    int litBytes = 0;
    for (auto& t : tokens) if (!t.copy) litBytes += t.lit.size();
    cout << (rebuilt == neu ? "true" : "false") << "\n";
    cout << litBytes << " of " << neu.size() << "\n";
    return 0;
}
