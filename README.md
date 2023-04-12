## Stone Automata Maze Challenge

The challenge was to find a **path** for a particle in a [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton) from the starting position (top left corner) to the finishing position (bottom right).

The animation below refers to the optimal path for the 65x85 matrix proposed at the first stage of the challenge and can give you a feeling for it.

![output](https://user-images.githubusercontent.com/60227644/231528356-4862d3b0-8912-414d-993b-5fed7f8dba60.gif)

The matrices at the second and final stage of the challenge could be as big as 2500x2500. For those, a graphical visualisation is prohibitive because of the sheer size. They were troublesome even to load in memory, as several iterations were needed to investigate a path. Interesting techniques were necessary to deal with them.

At the time this challenge came out, I had about 2 weeks of experience with Rust, but decided to give it a try! It was also my first time in a programming challenge, so everything was new to me.

It turned out to be an amazing experience and I learned a lot from it.
