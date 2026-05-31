// Smallest window containing every distinct char: sliding window with counts.
// Expand right, shrink left while all distinct chars covered. Time O(n), Space O(k).
#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
using namespace std;

int smallestWindow(const string& s) {
    unordered_set<char> distinct(s.begin(), s.end());
    int need = (int)distinct.size();
    unordered_map<char, int> cnt;
    int formed = 0, left = 0, best = (int)s.size();
    for (int right = 0; right < (int)s.size(); right++) {
        if (++cnt[s[right]] == 1) formed++;
        while (formed == need) {
            best = min(best, right - left + 1);
            if (--cnt[s[left]] == 0) formed--;
            left++;
        }
    }
    return best;
}

int main() {
    cout << smallestWindow("jiujitsu") << endl;
    return 0;
}
