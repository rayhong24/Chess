import game
from board import Board

def keep_making_first_move(board, piece, num_moves):
    for i in range(num_moves):
        old_i, old_j = piece.row, piece.column
        moves = piece.get_moves(board.board)
        print(moves)

        if not moves:
            print(f"No more valid moves. Stopped at move {i}.")
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
    keep_making_first_move(forward_test_board, pawn, 10)
    
    print("Moving pawn 2")
    pawn2 = forward_test_board.board[6][4]
    keep_making_first_move(forward_test_board, pawn2, 10)


def rook_test():
    print("Testing Rook implementation")
    rook_test_board = Board()
    rook_test_board.print_board()

    print("Printing moves in starting position (should be none)")
    rook = rook_test_board.board[0][0]
    keep_making_first_move(rook_test_board, rook, 10)

    pawn = rook_test_board.board[1][0]
    print("Moving pawn blocking rook")
    keep_making_first_move(rook_test_board, pawn, 10)
    print("Moving rook")
    keep_making_first_move(rook_test_board, rook, 3)
    
def main():
    # pawn_test()
    rook_test()
    return
if __name__ == "__main__":
    main()