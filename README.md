My very first programming project remade. From running python in the terminal to a browser UI, now with TDD and Rust.

## Description

As the title suggests, this is a dice roll probability simulator. **It runs on any computer with a web browser without any installation, like mobile devices!** This project uses a pseudo-random number generator available on the user browser to simulate a die being rolled and does it several times to estimate the probability of a specific roll.

## Brief Project History

Before going to university, with the end of high school, I decided to learn something new and chose programming. I found some guy's playlist teaching Python and followed it until I had some confidence. More or less like: "oh, now I know how to give commands to a computer to execute instead of only using it to play video games". Then the first thing I wanted to do was writing a program that would tell me the probability of dice roll. 

I thought something in the lines of: "Although some rolls can be easily calculated, not all of them are and you have to have pen and paper nearby, plus it takes some time and I, more often than I'd like, make mistakes calculating them. So, computers can do simple things fast, like rolling a dice (calling a rng function). If it manages to do it lots of time, I'd have the same probability without having to calculate." Academically you could call it Monte Carlo dice roll probability estimation, as I discovered.

Although it initially worked, it wasn't as fast as I wanted due to my Python implementation of it. Still, it's a project that always stood in my mind because I wanted to know why it didn't work very well and how I could fix it. So when learning Rust I decided to remake that project, to add something more usable than running it on the terminal and try TDD in the process.

### Footnote

I know this isn't the best way to estimate dice probabilities, but it's the method I came up with when I was 17 and had little to no idea how to calculate every possible combination efficiently. In the end, suffering through the wait times caused by the millions of iterations I ran in Python 2.0 led me on a journey to learn about performance, which is topic I'm particularly fond of, including memory, computer architecture, and code execution/runtime behavior.

## Usage

No installation, terms of use agreement, registration, or data of any kind is required (and never will be) to be an user of this project. The philosophy is literally, "Hey, I did something cool. Please use it if you find it useful."

TODO: Add usage instructions when hosted on GitHub Pages.

## License

Apache License 2.0

Check the LICENSE file for more details.
