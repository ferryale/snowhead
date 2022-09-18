import subprocess
import os

def pgn_to_fen(pgnpath):
    cmd = ["pgn-extract", "-Wfen", "--quiet", pgnpath, f"-o{fenpath}"]
    subprocess.check_call(cmd)

def fen_to_epd(fenpath):
    with open(fenpath, "r") as f:
        lines = f.readlines()

    new_lines = []
    for line in lines:
        if "Result" in line:
            result = line.split("Result")[1].split("]")[0].strip()
        if line.startswith("[") or line == "\n": continue

        new_line = f"{line.strip()}; c0 {result}"
        new_lines.append(new_line)

    with open(epdpath, "w") as f:
        f.write("\n".join(new_lines))




tuner_dir = os.path.dirname(os.getcwd())
data_dir = os.path.join(tuner_dir, "data")

for pgnfile in os.listdir(data_dir):
    fenfile = pgnfile.replace(".pgn", ".fen")
    epdfile = pgnfile.replace(".pgn", ".epd")

    pgnpath = os.path.join(data_dir, pgnfile)
    fenpath = os.path.join(data_dir, fenfile)
    epdpath = os.path.join(data_dir, epdfile)

    pgn_to_fen(pgnpath)
    fen_to_epd(fenpath)

    
