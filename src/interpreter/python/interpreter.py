from pest import Parser, Rule

class VelvetParser(Parser):
    grammar_file = "src/parser/velvet.pest"

    def parse_program(self, input_str):
        pairs = self.parse(Rule.program, input_str)
        ast = []
        for pair in pairs:
            ast.append(self._process_pair(pair))
        return ast

    def _process_pair(self, pair):
        match pair.as_rule():
            case Rule.say:
                expr = pair.inner()[0]
                return ("say", self._process_pair(expr))
            case Rule.expr:
                return self._process_pair(pair.inner()[0])
            case Rule.primary:
                return pair.as_str()
            # Add other rules as needed
            case _:
                return None

    def interpret(self, ast):
        for node in ast:
            if node[0] == "say":
                print(f"Output: {node[1]}")

if __name__ == "__main__":
    parser = VelvetParser()
    program = 'say "Hello, Velvet!"'
    ast = parser.parse_program(program)
    parser.interpret(ast)
