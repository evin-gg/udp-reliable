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

    with open("../log.txt", "r") as file:
        for line in file:
            line = line.strip()
            print("Line read:\'"+line+"\'")
            if line == "[ACK]":
                acks += 1

            if line == "[SENT]":
                sent += 1
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
            print("Sent: "+str(sent))
            print("Acks: "+str(acks))
            


        time.sleep(1)

    

if __name__ == "__main__":

    filechanges = threading.Thread(target=detect_file_changes, args=("../log.txt",))
    filechanges.start()
    
    plt.ion()
    fig, ax = plt.subplots()

    x = ["Sent", "Ack"]
    bars = ax.bar(x, [sent, acks])
    while True:
        bars[0].set_height(sent)
        bars[1].set_height(acks)

        ax.relim()
        ax.autoscale_view()

        plt.draw()
        plt.pause(0.2)
