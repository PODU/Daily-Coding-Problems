// Day 762: Smallest distance (words in between) between two target words.
// Single pass tracking last seen index of each word. Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int smallestDistance(const vector<string>& words, const string& a, const string& b) {
    int lastA = -1, lastB = -1, bestGap = INT_MAX;
    for (int i = 0; i < (int)words.size(); ++i) {
        if (words[i] == a) {
            lastA = i;
            if (lastB != -1) bestGap = min(bestGap, lastA - lastB);
        }
        if (words[i] == b) {
            lastB = i;
            if (lastA != -1) bestGap = min(bestGap, lastB - lastA);
        }
    }
    if (bestGap == INT_MAX) return -1;       // a word not found
    return bestGap - 1;                      // number of words strictly between
}

int main() {
    vector<string> words = {"dog","cat","hello","cat","dog","dog","hello","cat","world"};
    cout << smallestDistance(words, "hello", "world") << "\n";  // 1
    return 0;
}
