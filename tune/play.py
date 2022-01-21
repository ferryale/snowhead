import os
import subprocess

c_chess_cli = "/mnt/c/Users/Alex/Desktop/engines/c-chess-cli/c-chess-cli"
snowhead = "/mnt/c/Users/Alex/Desktop/rust/snowhead/target/release/snowhead"

cmd = [c_chess_cli, "-each", "-engine" , f"cmd={snowhead}", "-engine" , f"cmd={snowhead}", "tc=1+0.1", "-pgn", "test.pgn"]

subprocess.check_call(cmd)

print(" ".join(cmd))