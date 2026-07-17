// Word search: DFS backtracking from each cell. O(R*C*4^L) time, O(L) space.
public class Solution {
    static boolean dfs(char[][] b, String w, int i, int r, int c){
        if(i == w.length()) return true;
        if(r < 0 || r >= b.length || c < 0 || c >= b[0].length || b[r][c] != w.charAt(i))
            return false;
        char saved = b[r][c];
        b[r][c] = '#';
        boolean found = dfs(b,w,i+1,r+1,c) || dfs(b,w,i+1,r-1,c) ||
                        dfs(b,w,i+1,r,c+1) || dfs(b,w,i+1,r,c-1);
        b[r][c] = saved;
        return found;
    }
    static boolean exists(char[][] b, String w){
        for(int r = 0; r < b.length; r++)
            for(int c = 0; c < b[0].length; c++)
                if(dfs(b, w, 0, r, c)) return true;
        return false;
    }
    public static void main(String[] args){
        char[][] board = {
            {'A','B','C','E'},
            {'S','F','C','S'},
            {'A','D','E','E'}
        };
        for(String w : new String[]{"ABCCED","SEE","ABCB"})
            System.out.println("exists(board, \"" + w + "\") returns " +
                (exists(board, w) ? "true" : "false"));
    }
}
