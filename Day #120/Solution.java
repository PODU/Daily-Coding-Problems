// Day 120: Dual singleton. Two fixed instances; odd call -> 2nd, even call -> 1st.
// getInstance() is O(1); instances created once (eager, thread-safe via static init).
public class Solution {
    private final String id;
    private Solution(String s){ id = s; }
    public String name(){ return id; }

    private static final Solution FIRST = new Solution("first");
    private static final Solution SECOND = new Solution("second");
    private static long count = 0;

    public static synchronized Solution getInstance(){
        count++; // 1-based call number
        return (count % 2 == 0) ? FIRST : SECOND; // even -> first, odd -> second
    }

    public static void main(String[] args){
        for (int i = 0; i < 4; i++)
            System.out.println(getInstance().name()); // second, first, second, first
    }
}
