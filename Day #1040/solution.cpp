// KMP string matching: build LPS failure array O(k), scan text O(N). Time O(N+k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

int kmpSearch(const string& text, const string& pat) {
    int n = text.size(), k = pat.size();
    if (k == 0) return 0;
    vector<int> lps(k, 0);
    for (int i = 1, len = 0; i < k; ) {
        if (pat[i] == pat[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    for (int i = 0, j = 0; i < n; ) {
        if (text[i] == pat[j]) { i++; j++; if (j == k) return i - j; }
        else if (j) j = lps[j - 1];
        else i++;
    }
    return -1;
}

int main() {
    string text = "abxabcabcaby";
    int r1 = kmpSearch(text, "abcaby");
    cout << (r1 == -1 ? "Not found" : "Found at index " + to_string(r1)) << "\n";
    int r2 = kmpSearch(text, "xyz");
    cout << (r2 == -1 ? "Not found" : "Found at index " + to_string(r2)) << "\n";
    return 0;
}
