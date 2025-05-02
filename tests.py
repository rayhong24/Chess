import unittest

def main():
    runner = unittest.TextTestRunner()

    piece_dir = 'Tests/TestPieces'
    piece_loader = unittest.TestLoader()
    piece_suite = piece_loader.discover(piece_dir)

    print(f"\nTesting dir: {piece_dir}")
    runner.run(piece_suite)

    # game_dir = 'Tests/TestGame'
    # game_loader = unittest.TestLoader()
    # game_suite = game_loader.discover(game_dir)

    # print(f"\nTesting dir: {game_dir}")
    # runner.run(game_suite)
    return

if __name__ == "__main__":
    main()