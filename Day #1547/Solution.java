// XOR linked list simulated with arrays indexed by integer addresses.
// add appends in O(1); get traverses with nextAddr = curBoth XOR prevAddr in O(index). Space O(n).
import java.util.ArrayList;

public class Solution {
    static class XorList {
        ArrayList<Integer> val = new ArrayList<>();  // value per address
        ArrayList<Integer> both = new ArrayList<>();  // prevAddr XOR nextAddr
        int head = 0, tail = 0;                       // address 0 is sentinel/null
        XorList() { val.add(0); both.add(0); }
        void add(int v) {
            int addr = val.size();
            val.add(v); both.add(0);
            if (head == 0) { head = tail = addr; }
            else {
                both.set(tail, both.get(tail) ^ addr);
                both.set(addr, both.get(addr) ^ tail);
                tail = addr;
            }
        }
        int get(int index) {
            int prev = 0, cur = head;
            for (int i = 0; i < index; i++) {
                int next = both.get(cur) ^ prev;
                prev = cur; cur = next;
            }
            return val.get(cur);
        }
    }

    public static void main(String[] args) {
        XorList list = new XorList();
        for (int v : new int[]{10, 20, 30, 40, 50}) list.add(v);
        System.out.println(list.get(0));
        System.out.println(list.get(2));
        System.out.println(list.get(4));
    }
}
