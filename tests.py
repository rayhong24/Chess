import unittest

def main():
    loader = unittest.TestLoader()
    start_dir = 'Tests'
    suite = loader.discover(start_dir)

    runner = unittest.TextTestRunner()
    runner.run(suite)
    return

if __name__ == "__main__":
    main()