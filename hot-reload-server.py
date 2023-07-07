import contextlib
import os
import time
import subprocess
import signal

# Create a hot reload server
# This will watch for changes in the files and reload the server
# reload command is `make start`

files = {}
process = None


def start_server():
    global process
    try:
        if process and process.poll() is None:
            os.killpg(os.getpgid(process.pid), signal.SIGTERM)
        process = subprocess.Popen("make start", shell=True, preexec_fn=os.setsid)
    except Exception as e:
        print("Failed to (re)start the server: ", e)


# Start the server
start_server()

while True:
    try:
        changed = False
        for root, _, filenames in os.walk("."):
            for filename in filenames:
                if filename.endswith(".go"):  # change from ".py" to ".go"
                    full_path = os.path.join(root, filename)
                    mtime = os.stat(full_path).st_mtime
                    if full_path not in files:
                        files[full_path] = mtime
                    elif files[full_path] != mtime:
                        print(f"File {full_path} has been modified. Reloading...")
                        files[full_path] = mtime
                        changed = True

        if changed:
            # Kill the previous running process if exists and start a new one
            start_server()

        time.sleep(1)

    except KeyboardInterrupt:
        # Kill the process if exists
        if process:
            with contextlib.suppress(Exception):
                os.killpg(os.getpgid(process.pid), signal.SIGTERM)
        print("Received keyboard interrupt. Stopping server.")
        break

    except Exception as e:
        print("An error occurred: ", e)
        # Instead of stopping the server, just print the error and continue
        continue
