import subprocess
import sys
import time
import os
import threading
from hft_bot import start_feed_loop
from strategy import on_quote

FEED_PROC = None

def stream_subprocess_output(proc):
    def stream(pipe):
        for line in iter(pipe.readline, b''):
            sys.stdout.buffer.write(line)
            sys.stdout.flush()
    threading.Thread(target=stream, args=(proc.stdout,), daemon=True).start()
    threading.Thread(target=stream, args=(proc.stderr,), daemon=True).start()

def start_rust_feed():
    global FEED_PROC
    # Always start the Rust feed as a subprocess
    FEED_PROC = subprocess.Popen([
        "cargo", "run", "--bin", "binance_feed"],
        cwd=os.path.join(os.path.dirname(__file__), "libs", "market_data"),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        preexec_fn=os.setsid
    )
    stream_subprocess_output(FEED_PROC)
    # Wait for the feed file to appear
    feed_path = "/dev/shm/feed"
    for _ in range(30):
        if os.path.exists(feed_path):
            return
        time.sleep(0.5)
    print("Warning: /dev/shm/feed not found after starting Rust feed.")

if __name__ == "__main__":
    start_rust_feed()
    try:
        start_feed_loop("/dev/shm/feed", 128, on_quote)
    finally:
        if FEED_PROC:
            FEED_PROC.terminate()
