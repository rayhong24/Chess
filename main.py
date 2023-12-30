import unittest
from game import Game
from board import Board
from Pieces.piece import Piece

def keep_making_first_move(board: Board, piece: Piece, num_moves: int):
    for i in range(num_moves):
        moves = piece.get_moves(board.board)
        print(moves)

        if not moves or (piece not in board.white_pieces and piece not in board.black_pieces):
            print(f"No more valid moves. Stopped at move {i}.")
            break

        move = moves[0]
        board.handle_move(move)
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

def bishop_test():
    print("Testing Bishop implementation")
    bishop_test_board = Board()
    bishop_test_board.print_board()

    print("Printing moves in starting position (should be none)")
    bishop = bishop_test_board.board[0][2]
    keep_making_first_move(bishop_test_board, bishop, 10)
    pawn = bishop_test_board.board[1][1]
    print("Moving pawn blocking bishop")
    keep_making_first_move(bishop_test_board, pawn, 10)
    print("Moving bishop")
    keep_making_first_move(bishop_test_board, bishop, 3)

def queen_test():
    print("Testing Queen implementation")
    queen_test_board = Board()
    queen_test_board.print_board()

    print("Printing moves in starting position (should be none)")
    queen = queen_test_board.board[0][3]
    keep_making_first_move(queen_test_board, queen, 10)
    pawn = queen_test_board.board[1][2]
    print("Moving pawn blocking queen")
    keep_making_first_move(queen_test_board, pawn, 10)
    print("Moving queen")
    keep_making_first_move(queen_test_board, queen, 3)

def king_test():
    print("Testing King implementation")
    king_test_board = Board()
    king_test_board.print_board()

    print("Printing moves in starting position (should be none)")
    king = king_test_board.board[0][4]
    keep_making_first_move(king_test_board, king, 10)
    pawn = king_test_board.board[1][4]
    print("Moving pawn blocking king")
    keep_making_first_move(king_test_board, pawn, 10)
    print("Moving king")
    keep_making_first_move(king_test_board, king, 3)

def knight_test():
    print("Testing knight implementation")
    knight_test_board = Board()
    knight_test_board.print_board()

    knight = knight_test_board.board[0][1]
    keep_making_first_move(knight_test_board, knight, 10)

def main():
    game = Game()
    game.display_game()
    return

if __name__ == "__main__":
    main()