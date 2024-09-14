# Application Tracker
This Rust program tracks the time you spend on different windows and applications. By polling the currently focused window every second, it logs the time spent on each window and application throughout the session. The program provides detailed logs, including the time spent on the current window during the session, as well as the total time spent on each window and application.

## Features
Tracks the time spent on individual windows and applications.
Logs the session and total times for each window and application.
Polls the currently focused window every second.
Displays session logs in a readable format using terminal formatting.
## Modules
`get_focus`
This module is responsible for identifying the currently focused window. It provides the get_focused_window() function that retrieves the name of the currently active window.

`detect_display_server`
This module checks the current display server (such as X11 or Wayland) to ensure compatibility with the window tracking.

`terminal_formatting`
This module contains utilities for formatting the terminal output to make the logs easier to read.

Code Structure
Main Loop
Initializes two HashMaps:

`app_times`: stores the time spent on each window.
`app_total_times`: stores the total time spent on each application.
Every second, it checks the currently focused window:

If the focused window changes, it calculates the time spent on the previous window.
Logs the session time for the previous window and adds it to the `app_times` and `app_total_times` maps.
Switches the focus to the new window and resets the timer.
Periodically prints the total time spent on each application.

Example Output
```
Starting application tracking...
2024-09-12 12:34:56 ->
Time spent on The Window : 'Terminal - user': on this session: 00:02:45
Total Time Spent on The Window 'Terminal - user': 00:05:30
Total Time Spent on The App 'Terminal': 00:05:30
```
## Prerequisites
Rust installed on your system.
Ensure the required display server (like X11 or Wayland) is running.
## How to Run
Clone the repository.

```
git clone <repo_url>
cd <repo_directory>
```
## Build and run the program:

`cargo run`
The application will start tracking the active windows and log the time spent on each window and application.
