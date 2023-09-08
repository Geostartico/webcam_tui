# WEBCAM from Terminal

Compatible with most linux terminals (tested on xfce-terminal) and partially on
tty show the the webcam in 3 modes:

* Pixel: using braille Characters show the image in color
* Character: Show the image through gray-scale in ascii Characters
* Color-Characters: show webcam as colored ascii characters

Go through modes using j-k keys
Using [Ratatui](https://github.com/ratatui-org/ratatui) rust library to write onto
the terminal using the crossterm backend
