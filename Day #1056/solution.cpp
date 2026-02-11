// Look-and-say sequence: start "1"; each term describes digit runs of previous.
// Iteratively build N terms by run-length encoding. Time O(N * L), Space O(L).
#include <iostream>
#include <string>

std::string lookAndSay(int n) {
    std::string cur = "1";
    for (int i = 1; i < n; ++i) {
        std::string next;
        int j = 0;
        while (j < (int)cur.size()) {
            int count = 1;
            while (j + count < (int)cur.size() && cur[j + count] == cur[j]) ++count;
            next += std::to_string(count);
            next += cur[j];
            j += count;
        }
        cur = next;
    }
    return cur;
}

int main() {
    int N = 5; // terms: 1, 11, 21, 1211, 111221
    std::cout << lookAndSay(N) << std::endl;
    return 0;
}
