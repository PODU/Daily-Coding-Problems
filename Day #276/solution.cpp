// Day 276: KMP pattern search. Time O(N + k), Space O(k) -- beats O(N*k).
// Returns start index of first match, or -1 (False).
#include <bits/stdc++.h>
using namespace std;

int kmp(const string& text, const string& pat) {
    int n = (int)text.size(), k = (int)pat.size();
    if (k == 0) return 0;
    vector<int> lps(k, 0);
    for (int i = 1, len = 0; i < k;) {
        if (pat[i] == pat[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    for (int i = 0, j = 0; i < n;) {
        if (text[i] == pat[j]) {
            i++; j++;
            if (j == k) return i - j;
        } else if (j) j = lps[j - 1];
        else i++;
    }
    return -1; // False
}

int main() {
    cout << kmp("abxabcabcaby", "abcaby") << "\n"; // 6
    cout << kmp("abxabcabcaby", "zzz") << "\n";    // -1
    return 0;
}
