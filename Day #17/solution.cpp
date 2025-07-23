// Approach: Single pass, track running path length per depth via a map/stack. O(n) time, O(d) space.
#include <bits/stdc++.h>
using namespace std;

int lengthLongestPath(const string& input) {
    unordered_map<int, int> lenAtDepth; // depth -> path length up to this dir (incl trailing '/')
    lenAtDepth[-1] = 0;
    int maxLen = 0;
    size_t i = 0, n = input.size();
    while (i < n) {
        size_t j = input.find('\n', i);
        if (j == string::npos) j = n;
        string line = input.substr(i, j - i);
        int depth = 0;
        while (depth < (int)line.size() && line[depth] == '\t') depth++;
        string name = line.substr(depth);
        bool isFile = name.find('.') != string::npos;
        int curLen = lenAtDepth[depth - 1] + (int)name.size();
        if (isFile) {
            maxLen = max(maxLen, curLen);
        } else {
            lenAtDepth[depth] = curLen + 1; // +1 for '/'
        }
        i = j + 1;
    }
    return maxLen;
}

int main() {
    string input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    cout << lengthLongestPath(input) << "\n";
    return 0;
}
