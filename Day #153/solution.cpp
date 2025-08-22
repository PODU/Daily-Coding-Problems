// Day 153: Min words separating two words. Single pass tracking last seen index
// of each word; answer is min(|i-j|-1). Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minDistance(const vector<string>& words, const string& a, const string& b) {
    int lastA = -1, lastB = -1, best = INT_MAX;
    for (int i = 0; i < (int)words.size(); i++) {
        if (words[i] == a) {
            lastA = i;
            if (lastB != -1) best = min(best, abs(lastA - lastB) - 1);
        }
        if (words[i] == b) {
            lastB = i;
            if (lastA != -1) best = min(best, abs(lastA - lastB) - 1);
        }
    }
    return best;
}

int main() {
    string text = "dog cat hello cat dog dog hello cat world";
    vector<string> words;
    stringstream ss(text);
    string w;
    while (ss >> w) words.push_back(w);
    cout << minDistance(words, "hello", "world") << "\n";
    return 0;
}
