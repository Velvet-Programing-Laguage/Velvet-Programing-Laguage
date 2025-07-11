import unittest
from src.hooks.interpreter import execute_velvet

class TestVelvetInterpreter(unittest.TestCase):
    def test_execute(self):
        execute_velvet("tests/test_velvet.vel")
        # Add assertions based on output
