# webgine

A collection of components with the intent to eventually build out a functioning web browser.
Current focus is on the groundwork for the coral application and the display interface in sunbeam.

## Component Breakdown

- sunbeam_html : The HTML engine
- stiletto_script : The script engine
- coral_app : The main gui application
- krait_web : Network operations

## Why seperate the html from the gui?

Current architecture seperates the sunbeam html engine and coral gui system so
as to provide greater versitility within the project.
Sunbeam aims to serve as an abstraction, that any GUI system could choose to render.
Ideally a TUI will also be created, but that is a long term goal depending heavily
on the progress of the project.
