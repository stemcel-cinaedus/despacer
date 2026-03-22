import os
import tkinter
from tkinter import filedialog

path = filedialog.askdirectory()

if path is not None:
	for item in os.listdir(path):
		fullpath = os.path.join(path, item)
		spaceless = fullpath.replace(' ', '')
		os.rename(fullpath, spaceless)
	print("Finished")
else:
	print("Invalid directory")
