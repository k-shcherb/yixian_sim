import pygetwindow
import pyautogui
import win32gui, win32com.client
from PIL import ImageGrab
import cv2
import matplotlib.pyplot as plt
import numpy as np

toplist, winlist = [], []
def enum_cb(hwnd, results):
    winlist.append((hwnd, win32gui.GetWindowText(hwnd)))
win32gui.EnumWindows(enum_cb, toplist)
yixian = [(hwnd, title) for hwnd, title in winlist if '弈仙牌' in title]
# win32gui.SetForegroundWindow(yixian[0][0])
bbox = win32gui.GetWindowRect(yixian[0][0])
img = np.array(ImageGrab.grab(bbox).convert('RGB'))

q = ""
card_num = 0
while q != "quit":
    img = np.array(ImageGrab.grab(bbox).convert('RGB'))
    gray = cv2.cvtColor(img, cv2.COLOR_RGB2GRAY)
    _, thresh = cv2.threshold(gray, 90, 255, type=cv2.THRESH_BINARY_INV)
    contours = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_NONE)
    contours = contours[0] if len(contours) == 2 else contours[1]

    cont_cards = []
    for i in contours:
        x, y, w, h = cv2.boundingRect(i)
        if 180000 > w*h > 60000:
            cont_cards.append(i)
        
    for cnt, i in enumerate(cont_cards):
        x, y, w, h = cv2.boundingRect(i) 
        card1 = gray[y:y+h, x:x+w]
        cv2.imwrite(f"cards/card_{card_num}.png", cv2.cvtColor(card1, cv2.COLOR_GRAY2BGR))
        card_num += 1
    
    q = input("type quit to quit, otherwise hit enter to take image: ")

print("finished taking images")
    
    