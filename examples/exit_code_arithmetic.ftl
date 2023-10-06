# Exits with a 'special', calculated exit code
#
# Run this file using: cargo run -- run examples/exit_code_arithmetic.ftl


# 'Import' the exit function from the C standard library
extern exit(status: int)

def main() {
	var code: int = (20 * 2 + 2)
	exit(code)
}
