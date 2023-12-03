(A) Rules of Minesweeper
    (A) Cell number represents the number of mines around it {cm:2023-12-01}
    (B) If uncovered cell has no mines around it, it will uncover all adjacent cells
    (C) First cell is never a mine {cm:2023-12-02}
    (D) Can flag a mine {cm:2023-12-01}
    (E) Flagging incorrectly will not cause a loss
    (F) Win-con is uncovering all cells that are not mines
    (G) Lose-con is uncovering a mine
(B) Status Information
    (A) Number of mines left
    (B) Time elapsed or Number of moves
(C) Optional
    (A) First cell's number is always 0 {cm:2023-12-02}
    (B) Question Marks
    # (If flags around a cell equal the number of mines around it, all adjacent cells will be uncovered) can cause instant loss
    (C) Area Open 
    # (If flags == number of mines total in the game, all remaining cells will be uncovered) can cause instant loss
    (D) Open Remaining 