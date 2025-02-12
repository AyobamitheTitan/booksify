#  Booksify

A simple Rust backend API I built to show that I can build backend apps with Rust. Some of the functionalities it provides are:

- Registration and Login (Jwt is returned on login)
- Proper error display and formatting
- Uniform response display
- Add and retrieving books

I wanted to get this up on time so I could put this up on github for potential viewers to see what I am about, but here are some features I would love to add:


- Swagger Documentation
- Deleting and updating books
- Leaving a review on a book
- Viewing reviews on a book

## Stack
__Framework:__ Actix-web \
__Database:__ PostgreSQL\
__ORM:__ SeaORM


## How to run the app

Run the `cargo run` run command on your terminal to activate the app. For this to run you must first install rust. More information on that [here](https://www.rust-lang.org/tools/install).

In development, I made a lot of changes and it was necessary for me to see the changes I was making real time, so the command I used was `cargo watch -x run`. To run this command, you need to have __cargo-watch__ [installed](https://crates.io/crates/cargo-watch).

Create a ``.env`` file. The expected contents of the file are contained in the ``.env.example`` file. The program will run into problems if the ``.env`` file is not found

## Design choices

I got some inspiration for the file structure on the Actix-web example apps on their github [repository](https://github.com/actix/examples#readme). Other than that, my main inspiration for file arrangement was from my work with FastAPI, which has been my tool of choice for building backend APIs over the pas couple months.

__Below are the directories and what they contain:__

api - contains the routes and their handlers.

database - contains the database object for making mutations and queries.

entity - contains the database models.

migrations - contains the migrations that define the models.

schema - contains the schemas that define how data is presented, formatted and received over the API. Heavily inspired by Pydantic models.

service - contains the interactinos with the database, whether queries or mutations.