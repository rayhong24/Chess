# Fairy Penguin - Custom Chess Engine
One random evening many years ago, I was playing chess in the living room with my family. None of us were very good, and I remember throwing away a completely winning position. What followed was a slight obsession with the game where I proceeded to spend most of my free time playing online. Eventually, this led to me starting the high school chess club, and on occasion, having me participate in local tournaments.

After entering university to study computer science, I thought creating my own custom chess engine would be a great project to combine my interests. Not only would I be able to apply what I learned during my studies and deepen my understanding about them, but also learn new concepts in computer science.

This is what inspired me to start the development of Fairy Penguin. I started this at some point during university and have been gradually iterating on it between semesters or during my free time. Although I will be graduating from university soon, I do not plan on stopping development of Fairy Penguin. Like I said earlier, this is a passion project that I am using to apply my knowledge and to learn. There is a near endless list of features and improvements I would like to implement and plan on doing so, however gradually that is. 

I have created a bot account on Lichess that I generally use to test the engine. You can follow Fairy Penguin [here](https://lichess.org/@/FairyPenguin). Feel free to check out its game history or challenge it to a game if I have it online and connected.

## Features
- Implements the [Universal Chess Interface (UCI)](https://www.shredderchess.com/chess-features/uci-universal-chess-interface.html) so it can communicate with external GUI applications.
- Engine "thinking" is done with a minimax algorithm (with alpha-beta pruning) along with a custom board-evaluation heuristic.

## Lessons Learned
### 1. Object-Oriented Design
When I started this project, I had limited experience creating my own large and complex projects. In addition, I had mostly only seen basic examples of OOP principles and never had to design and implement my own classes. This led to many mistakes in my implementation. 

The first one that comes to mind is that my classes were too tightly coupled. Whenever I made changes to one class, I would inevitably need to change multiple other classes to avoid runtime errors. 
The second big mistake I made is that I would give my classes too much responsibility. Each class would start off with a specific purpose in mind, but rather than create a new class for new features, I would add to the old ones, making them bloated and unmanageable. 

To solve these issues, I decided to do 2 major refactors. I rewrote the majority of my code, taking into account the shortcomings of my previous implementations. I was more thoughtful in designing my class methods to reduce coupling and I broke down my larger classes into multiple smaller classes, each with their own specified purpose. These changes has made many aspects of developing the engine easier. Adding features is a less complicated process due to the reduced coupling and isolating bugs is much simpler because of the single-responsibility of each class.
