import pyautogui
import keyboard
import time
import random

positions = []

with open("positions.txt") as f:
    for line in f:
        line = line.strip()
        split = line.split(' ')
        positions.append([(split[i], split[i+1])
                         for i in range(0, len(split), 2)])

random.shuffle(positions)

keyboard.wait('insert')
left = pyautogui.position()[0]
top = pyautogui.position()[1]
time.sleep(.5)
keyboard.wait('insert')
right = pyautogui.position()[0]
bottom = pyautogui.position()[1]

xstep = (right - left) / 3
ystep = (bottom - top) / 3

for trip in positions:
    pyautogui.moveTo(int(left + int(trip[0][0]) * xstep),
                     int(top + int(trip[0][1]) * ystep))
    pyautogui.mouseDown()
    for pos in trip:
        if keyboard.is_pressed('esc'):
            exit()
        pyautogui.moveTo(
            int(left + int(pos[0]) * xstep), int(top + int(pos[1]) * ystep))
    pyautogui.mouseUp()
