# Format with:
# message - reason why the move is invalid
invalid_move_template = "Invalid move: {}"

# Message used when there is no piece to move
# Format with:
# piece - piece that  
# i - index of row
# j - index of column
invalid_move_no_piece_message = invalid_move_template.format("No piece to move on {}, {}")

# Message used when trying to move a piece of the wrong colour
# Format with:
# piece - piece
# i - index of row
# j - index of column
invalid_move_wrong_colour = invalid_move_template.format("Cannot move piece of other colour.")
