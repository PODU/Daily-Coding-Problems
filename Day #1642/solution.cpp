// Min word distance: single pass tracking last-seen index of each word; on each
// hit, update min with |i-j|-1 (words strictly between). Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minWordDistance(const vector<string>& text, const string& w1, const string& w2) {
    int last1 = -1, last2 = -1, best = INT_MAX;
    for (int i = 0; i < (int)text.size(); ++i) {
        if (text[i] == w1) {
            last1 = i;
            if (last2 != -1) best = min(best, abs(last1 - last2) - 1);
        }
        if (text[i] == w2) {
            last2 = i;
            if (last1 != -1) best = min(best, abs(last1 - last2) - 1);
        }
    }
    return best;
}

int main() {
    vector<string> text = {"dog","cat","hello","cat","dog","dog","hello","cat","world"};
    cout << minWordDistance(text, "hello", "world") << endl;
    return 0;
}
