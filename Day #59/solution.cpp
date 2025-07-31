// Day 59: File sync over low-bandwidth link, rsync-style.
// Receiver sends per-block (weak rolling + strong) checksums of its OLD file;
// sender scans NEW file with a rolling checksum, emitting COPY(block) tokens for
// matches and LITERAL bytes otherwise -> only differing bytes cross the wire.
// Time: O(n) expected, bandwidth ~ size of the changes.
#include <bits/stdc++.h>
using namespace std;

static const uint32_t M = 1u << 16;
static const int B = 4;

uint64_t fnv1a(const string& d, int s, int e) {
    uint64_t h = 1469598103934665603ULL;
    for (int i = s; i < e; i++) { h ^= (unsigned char)d[i]; h *= 1099511628211ULL; }
    return h;
}

void weakFull(const string& d, int s, int e, uint32_t& a, uint32_t& b) {
    a = b = 0;
    for (int i = s; i < e; i++) {
        a = (a + (unsigned char)d[i]) % M;
        b = (b + (uint32_t)(e - i) * (unsigned char)d[i]) % M;
    }
}

// Receiver side: weak -> (strong -> index).
unordered_map<uint32_t, unordered_map<uint64_t,int>> signatures(const string& old) {
    unordered_map<uint32_t, unordered_map<uint64_t,int>> sigs;
    int n = old.size() / B;
    for (int i = 0; i < n; i++) {
        uint32_t a, b; weakFull(old, i*B, i*B+B, a, b);
        sigs[(b << 16) | a][fnv1a(old, i*B, i*B+B)] = i;
    }
    return sigs;
}

struct Token { bool copy; int idx; string lit; };

vector<Token> diff(const string& nw,
        unordered_map<uint32_t, unordered_map<uint64_t,int>>& sigs) {
    vector<Token> tokens;
    string literal;
    int pos = 0, n = nw.size();
    uint32_t a = 0, b = 0;
    bool haveWindow = false;
    while (pos + B <= n) {
        if (!haveWindow) { weakFull(nw, pos, pos + B, a, b); haveWindow = true; }
        uint32_t weak = (b << 16) | a;
        int idx = -1;
        auto it = sigs.find(weak);
        if (it != sigs.end()) {
            auto jt = it->second.find(fnv1a(nw, pos, pos + B));
            if (jt != it->second.end()) idx = jt->second;
        }
        if (idx >= 0) {
            if (!literal.empty()) { tokens.push_back({false, 0, literal}); literal.clear(); }
            tokens.push_back({true, idx, ""});
            pos += B; haveWindow = false;
        } else {
            unsigned char first = nw[pos];
            literal.push_back(nw[pos]);
            a = (a - first + (unsigned char)nw[pos + B]) % M;
            b = (b - (uint32_t)B * first + a) % M;
            pos++;
        }
    }
    literal += nw.substr(pos);
    if (!literal.empty()) tokens.push_back({false, 0, literal});
    return tokens;
}

string applyDelta(const string& old, const vector<Token>& tokens) {
    string out;
    for (auto& t : tokens)
        out += t.copy ? old.substr(t.idx * B, B) : t.lit;
    return out;
}

int main() {
    string old = "the quick brown fox jumps over the lazy dog";
    string nw  = "the quick brown cat jumps over the lazy dog";
    auto sigs = signatures(old);
    auto tokens = diff(nw, sigs);
    string rebuilt = applyDelta(old, tokens);
    int literalBytes = 0, copied = 0;
    for (auto& t : tokens) { if (t.copy) copied++; else literalBytes += t.lit.size(); }
    cout << "synced: " << (rebuilt == nw ? "true" : "false") << endl;
    cout << "literal bytes sent: " << literalBytes << " of " << nw.size() << endl;
    cout << "blocks reused: " << copied << endl;
    return 0;
}
