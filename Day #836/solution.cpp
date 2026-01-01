// Day 836: Fisher-Yates shuffle using only a rand(k) RNG (uniform 1..k) and swaps.
// For i=n-1..1: pick j uniform in 0..i via rand(i+1)-1, swap a[i],a[j]. O(N) time, O(1) extra.
// Unbiased: each step picks uniformly among i+1 positions, so all n! permutations are equally likely.
#include <bits/stdc++.h>
using namespace std;

struct RNG {
    uint64_t state;
    RNG(uint64_t seed) : state(seed) {}
    uint64_t next() {
        state = state * 6364136223846793005ULL + 1442695040888963407ULL;
        return state >> 16;
    }
    // Uniform in [1, k] with rejection to avoid modulo bias.
    uint64_t rand(uint64_t k) {
        uint64_t mask = (1ULL << 48) - 1;
        uint64_t limit = (1ULL << 48) - ((1ULL << 48) % k);
        while (true) {
            uint64_t r = next() & mask;
            if (r < limit) return r % k + 1;
        }
    }
};

void shuffleArr(vector<int>& a, RNG& rng) {
    for (int i = (int)a.size() - 1; i > 0; --i) {
        int j = (int)rng.rand(i + 1) - 1; // uniform 0..i
        swap(a[i], a[j]);
    }
}

int main() {
    vector<int> deck(52);
    iota(deck.begin(), deck.end(), 1);
    RNG rng(12345);
    shuffleArr(deck, rng);
    for (size_t i = 0; i < deck.size(); ++i)
        cout << deck[i] << (i + 1 < deck.size() ? " " : "\n");
    return 0;
}
