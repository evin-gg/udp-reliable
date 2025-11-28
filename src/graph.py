import matplotlib.pyplot as plt
from matplotlib import colors as mcolors
import time
import os
import threading

# global variables
sent = 0
acks = 0

# updating global variables
def update_data():
    global sent, acks

    sent = 0
    acks = 0

    with open("log.txt", "r") as file:
        for line in file:
            line = line.strip()
            if line == "[ACK]":
                acks += 1

            if line == "[SENT]":
                sent += 1

        os.system('clear')
        print("[LOGGER] CURRENT STATISTICS")
        print("\nSENT: ", sent)
        print("ACKS: ", acks)
        file.close()

# polling thread
def detect_file_changes(file_path):
    last_modified = os.path.getmtime(file_path)
    while True:
        current_modified = os.path.getmtime(file_path)
        if current_modified != last_modified:
            print("File has changed!")
            last_modified = current_modified
            update_data()
        time.sleep(1)

    

if __name__ == "__main__":
    os.system('clear')

    filechanges = threading.Thread(target=detect_file_changes, args=("log.txt",))
    filechanges.start()
    
    plt.ion()
    fig, ax = plt.subplots()
    
    x = ["Sent", "Ack"]
    bars = ax.bar(x, [sent, acks], color=["skyblue", "mediumspringgreen"], width=0.4)

    plt.title("Packet Statistics")

    bars[0].set_label("Sent")
    bars[1].set_label("Ack")
    ax.legend()
    while True:
        bars[0].set_height(sent)
        bars[1].set_height(acks)

        ax.relim()
        ax.autoscale_view()

        plt.draw()
        plt.pause(0.2)
