import game
from board import Board

def keep_making_first_move(board, piece):
    while True:
        old_i, old_j = piece.row, piece.column
        moves = piece.get_moves(board.board)

        if not moves:
            print("No more valid moves")
            break

        new_i, new_j = moves[0]
        board.move_piece(old_i, old_j, new_i, new_j)
        board.print_board()

    return


#TODO: add verbose flag
def pawn_test():
    print("Testing Pawn implementation")
    forward_test_board = Board()
    forward_test_board.print_board()

    print("Moving pawn 1")
    pawn = forward_test_board.board[1][1]
    keep_making_first_move(forward_test_board, pawn)
    
    print("Moving pawn 2")
    pawn2 = forward_test_board.board[6][4]
    keep_making_first_move(forward_test_board, pawn2)
    
def main():
    pawn_test()
    return
if __name__ == "__main__":
    main()