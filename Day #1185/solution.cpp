// Day 1185: Smallest distance (words in between) between two words in a text.
// Single pass tracking last index of each target word; min |i-j| - 1.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int shortestDistance(const string& text, const string& w1, const string& w2) {
    istringstream iss(text);
    string token;
    int idx = 0, p1 = -1, p2 = -1, best = INT_MAX;
    while (iss >> token) {
        if (token == w1) p1 = idx;
        if (token == w2) p2 = idx;
        if (p1 >= 0 && p2 >= 0) best = min(best, abs(p1 - p2));
        idx++;
    }
    return best == INT_MAX ? -1 : best - 1; // words separating the occurrences
}

int main() {
    string text = "dog cat hello cat dog dog hello cat world";
    cout << shortestDistance(text, "hello", "world") << "\n"; // 1
    return 0;
}
