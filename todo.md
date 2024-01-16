# TODO
- Add tests for Promotion
- Make test errors clearer
- Implement en Passant
- Make sure valid moves returned by a piece does not leave the king in check
- No castling if through a check.
- Implement the interface (following UCI)
- Make sure move is valid in handle move
- 50 move rule and turn counter
    - Like from fenstr
- Refactor castling
- Change board check in testGameClass 
    - either change formatting or implementation
- Change stored coords of a piece to use file and row
    - i.e. row=6, col=4 becomes file=e, rank=2

- Fix fenstring castling rights check for tests
- Create a Piece factory so logic is not part of the Board class.