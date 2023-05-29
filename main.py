import game
from board import Board

#TODO: add verbose flag
def pawn_test():
    print("Testing Pawn implementation")
    forward_test_board = Board()
    forward_test_board.print_board()
    i, j = 1, 1
    while True:
        moves = forward_test_board.board[i][j].get_moves(forward_test_board.board)

        if not moves:
            print("No more valid moves")
            break

        ii, jj = moves[0]
        forward_test_board.move_piece(i, j, ii, jj)
        i, j = ii, jj
        forward_test_board.print_board()
    
def main():
    pawn_test()
    return
if __name__ == "__main__":
    main()