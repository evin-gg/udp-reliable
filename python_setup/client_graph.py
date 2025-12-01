import matplotlib.pyplot as plt
from matplotlib import colors as mcolors
import time
import os
import threading

# global variables
sent = 0
acks = 0
resend = 0

# updating global variables
def update_data():
    global sent, acks, resend

    sent = 0
    acks = 0
    resend = 0

    with open("./logs/client.log", "r") as file:
        for line in file:
            line = line.strip()
            if line == "[ACK]":
                acks += 1

            if line == "[SENT]":
                sent += 1
            
            if line == "[RETRANSMISSION]":
                resend += 1

        os.system('clear')
        print("[CLIENT LOGGER] CURRENT STATISTICS")
        print("\nSENT: ", sent)
        print("ACKS: ", acks)
        print("RETRANSMISSION: ", resend)
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

    filechanges = threading.Thread(target=detect_file_changes, args=("./logs/client.log",))
    filechanges.start()
    
    plt.ion()
    fig, ax = plt.subplots()
    
    x = ["Sent", "Ack", "Retransmission"]
    bars = ax.bar(x, [sent, acks, resend], color=["skyblue", "mediumspringgreen", "slateblue"], width=0.4)

    plt.title("Client Packet Statistics")

    bars[0].set_label("Sent")
    bars[1].set_label("Ack")
    bars[2].set_label("Retransmission")
    ax.legend()
    while True:
        bars[0].set_height(sent)
        bars[1].set_height(acks)
        bars[2].set_height(resend)

        ax.relim()
        ax.autoscale_view()

        plt.draw()
        plt.pause(0.2)
