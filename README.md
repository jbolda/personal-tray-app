# Tauri System Tray App Starter Template

Looking to create a [Tauri app](https://beta.tauri.app/) that runs in the background and is accessible through the system tray or a keyboard? Use this template as a way to jump start creating your app.

It implements a few specific features that we might expect this use case to require:

- system tray integration with mini-pop-up window
- global shortcut to open the app
- notification integration ready to handle native toasts

## Getting Started

This is marked as a template in GitHub to allow one to more easily clone this into a new repo. Click `Use this template` to get started.

The repository has three main folders: `javascript`, `rust`, and `snippets`. With the new features in v2 of Tauri, these APIs are accessible through either JavaScript or Rust. We have implemented the template at these two extremes. Feel free to choose whichever fits your preference, and pull code from the other to create a mix as needed; mix-and-match between these two implementations to find the best fit for your project.

The implementations between JavaScript and Rust are similar. The Rust implementations tend to be set within the Tauri builder which leads to the implementations being more build-time / setup-time oriented. Conversely the JavaScript implementation tends towards runtime. Each of these have both preference and possible use case considerations, hence the value of mix-and-matching.

The snippets folder has example implementations for features that you may find useful, but were not expected to be widely applicable. As such, we include copy-and-paste code, but don't include it directly in the main templates.

## Background And Development

Most of this was original done on live over a handful of YouTube streams! [Check out the playlist where we built the initial template.](https://www.youtube.com/playlist?list=PL6K94_Yb98WwE46qewjbk2WgmsLTnHnmd)

Interested in helping out? Check out the issues for planned work.
