use dioxus::prelude::*;
use crate::components::book::Book;

#[inline_props]
pub fn Books(cx: Scope) -> Element {
    render! {
        // Top paragraph
        p {
            "I've always enjoyed seeing what books other people have read. Below are
            all the books I've read since middle school, roughly in order." 
        }
        p { 
            "Those highlighted "
            span { class: "text-highlight", "orange"} " were those I particularly enjoyed or
            found impactful. An asterisk (*) indicates you can " span{ class: "italic", "click" }
            " to see some of my
            thoughts on the book :)"
        }

        h2 { class: "mt-16 mb-5", "[2023 | Age 20]" }

        Book {
            number: 130,
            title: "The House of Morgan".into(),
            author: "Ron Chernow".into(),
            special: false,
            description: "This behemoth of a book is about the Morgan banking dynasty, from George Peabody and J.S. Morgan to now. Because the Morgan Dynasty has been around for 200 years, this book also is nearly a history of modern banking as a whole. What I thought was interesting was the evolution of the relation between bank and the state. From king makers to diplomats to tools have they fallen.".into(),
        }

        Book {
            number: 129,
            title: "The Odyssey".into(),
            author: "Translated by Emily Wilson".into(),
            special: true,
            description: "Surprisingly, I've never read many 'classics'. Emily Wilson did a fantastic translation. I did enjoy the story. These old archetypal stories always reflect aspects of the human condition that we can relate to and cause us to reflect deeper about our lives. We are all the hero in our own lives on our own journey, and we must make the best of it.".into(),
        }

        Book {
            number: 128,
            title: "Physics for Engineers and Scientists, Vol. 1".into(),
            author: "Hans C. Ohanian & John T. Markert".into(),
            special: true,
            description: "This is the first textbook I've actually nearly read to from cover to cover. Everything excepct for the sections on heat and thermodynamics. It was a very good textbook. I noticed that there's a really large gap in my understanding of the world. Math and computer science is solid knowledge, but in terms of how the world actually works, I've been fully ignorant until now.".into(),
        }

        Book {
            number: 127,
            title: "Six Easy Pieces".into(),
            author: "Richard P. Feynman".into(),
            special: false,
            description: "Feels great to actually understand physics (currently also taking a physics sequence at UT). After a couple of books about general relativity and quantum mechanics, I'll tap out of physics. I'll have a reasonable understanding. I can die satisfied.".into(),
        }

        Book {
            number: 126,
            title: "Rome: An Empire's Story".into(),
            author: "Greg Woolf".into(),
            special: false,
            description: "I loved the end, when it was talking about what causes a culture to create monuments. The late republic and early empire, tons of monuments were built. People that build monuments have enough trust that there will be someone there in the future to remember them. Very few monuments were built after the early life of the empire, which also lines up with the start of its decline. Of course, economic factors play in, but maybe people didn't believe that future generations would be there for them? Or at least future generations that they cared about.".into(),
        }

        Book {
            number: 125,
            title: "The Man From The Future".into(),
            author: "Ananyo Bhattacharya".into(),
            special: false,
            description: "This book talks about each field that John von Neumann was involved in: quantum mechanics, the manhatten project, computers, and cellular automaton. It's too late for me, but there seems to be a systematic way to raise kids with a touch of genius. Every time I think about it, focusing my entire life on gathering resources and then pivoting all my focus on my future kids seems like it should surely have a higher long term expected value than me trying to reach great heights, as my kids would surely be able to do more. Of course, the future is always full of uncertainties.".into(),
        }

        Book {
            number: 124,
            title: "American Prometheus".into(),
            author: "Kai Bird & Martin J. Sherwin".into(),
            special: true,
            description: "This is a great biography about J. Robert Oppenheimer. I've read so many books about such great figures. It's strange, going through someone's entire life in just a week. It always leaves an impression on me greatly, reading about their death, and Oppenheimer is of no exception. Wasted away from cancer. Hearing about how people change when faced with their own mortality, it always makes me stop and ponder what I am doing with my life.".into(),
        }

        Book {
            number: 123,
            title: "Einstein".into(),
            author: "Walter Isaacson".into(),
            special: false,
            description: "This book became so much better and more enriching after I learned about the basics of topology and the history of non-Euclidian geometry and the early Prussian university system with the characters and historic figues that came out of there. I love more about the same things through different lenses. Great biography in general.".into(),
        }

        Book {
            number: 122,
            title: "Algorithms to Live By".into(),
            author: "Brian Christian & Tom Griffiths".into(),
            special: false,
            description: "This book made me wonder how much knowledge I didn't catch throughout my life. I read a part of this book in the past before. Rereading it again, after taking computer architecture and OS, is a completely different experience. Things that I didn't understand or pick up on are now obvious to me and the amount I take away is greatly different.".into(),
        }

        Book {
            number: 121,
            title: "Gödel's Proof".into(),
            author: "Ernest Nagel & James R. Newman".into(),
            special: true,
            description: "For the first time in my life, I feel like I actually have a fairly intuitive understanding of Gödel's Incompleteness Theorem, to a point to where I could explain it to a child. I decided to read this because I started Gödel Escher Bach and wanted to truly understand the concept that sparked it all before reading such a Tour De Force.".into(),
        }

        Book {
            number: 120,
            title: "The Joy of X".into(),
            author: "Steven Strogatz".into(),
            special: false,
            description: "Just another book about math history and trivia. One thing that I thought was super interesting is that it mentioned how to optimal stopping point for any problem where you know the size, the solution is 1/e. In another book, 'Agorithms To Live By', it mentioned this, but it just said .37% (the rounded decimal version of 1/e). I'm just reminded of how amazing math is and how it is unexplanably beautiful and tied together.".into(),
        }

        Book {
            number: 119,
            title: "The Creature from Jekyll Island".into(),
            author: "G. Edward Griffin".into(),
            special: true,
            description: "This is mandatory reading. This book does what Howard Zinn's 'The People's History Of The United States' wanted to do, while funnily being on the exact opposite of the political spectrum. I'm always reminded how, in response to the criticism of conspiracy theories in general, people remind us that the founding of our country was a massive conspiracy theory. The more I exist, the more I realize that a lot of the largest socio, political, and economic events in human history have all been just strings of conspiracy theories, and we are almost certainly living amongst many at the current moment.".into(),
        }

        Book {
            number: 118,
            title: "How Google Works".into(),
            author: "Eric Schmidt & Jonathan Rosenberg".into(),
            special: false,
            description: "Bland. This was a mix between 'History of Google' and 'Google's management philosophy'. The history part is nice, the latter was just very generic stuff. Most likely my perspective. When this book came out these ideas could have been novel, but throughout the years, much of this advice propagated to most of the startups out there to the point where a lot of this is common knowledge".into(),
        }

        Book {
            number: 117,
            title: "A Mind at Play".into(),
            author: "Rob Goodman & Jimmy Soni".into(),
            special: true,
            description: "I love Claude Shannon. Really a human to cause inspires in others. The archetypal tinkerer. Just a terrific man. While I might not ever be able to outshine his genius, I might one day be able to juggle better than him (5 balls instead of 4)!".into(),
        }

        Book {
            number: 116,
            title: "The Poincaré Conjecture".into(),
            author: "Donal O'Shea".into(),
            special: true,
            description: "A truly fantastic book, one of the best I've ever read. Goes through the history of math, from Euclid to Gauss, Riemann, etc. and covers the origin of geometry, topology, calculus, and other foundations of math and how it all connects. I very much appreciated its intro to topology as I'm very new to a lot of higher math topics. Truly a wonderful book and has helped spark a deeper curiousity and love for math within me.".into(),
        }

        Book {
            number: 115,
            title: "The Everthing Store".into(),
            author: "Brad Stone".into(),
            special: true,
            description: "This book is about Jeff Bezos and the founding of Amazon. Really surprised me. Who knows how much the lens of this story warps the reality, but from this book, it seems like Jeff Bezos doing something great was inevitable. I recently read half of 'Shoe Dog', the founding story about nike, and it really seems like the guy just got insanely lucky. Jeff Bezos seems to be at the very opposite of the spectrum; it was inevitable he would do something great. Very much holds admirable traits to emulate.".into(),
        }

        Book {
            number: 114,
            title: "All the Math You Missed".into(),
            author: "Thomas A. Garrity".into(),
            special: false,
            description: "This covers everything from Linear Algebra to Topology, from Differential Equations to Non-Euclidian Geometry. It was really nice to just see everything that one could cover in an undergraduate math degree and introduced me to a bunch of new topics.".into(),
        }

        Book {
            number: 113,
            title: "Cracking The Coding Interview".into(),
            author: "Gayle Laakmann Mcdowell".into(),
            special: false,
            description: "Just had to go through this quickly to refresh my memory".into(),
        }

        Book {
            number: 112,
            title: "Chip War".into(),
            author: "Chris Miller".into(),
            special: false,
            description: "This book is about the history of the microchip industry, mentioning the rise of globalism, of Japan, Tiwan, South Korea, the history of Silicon Valley and its relation with the US military, and the national security concerns China and the US has over the lack of control over technologies that make up the majority of their military might. Fantastic book, right down my alley.".into(),
        }

        Book {
            number: 111,
            title: "Empire of Pain".into(),
            author: "Patrick Radden Keefe".into(),
            special: false,
            description: "This book is about the Sackler family and the opioid pandemic. I realized that, like how a lot of women enjoy true crime stories, I am a sucker for the male equivilant: white-collar crime stories. This is like the 8th book I've read over some corporate story of fraud and greed.".into(),
        }

        Book {
            number: 110,
            title: "No Longer Human".into(),
            author: "Osamu Dazai".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 109,
            title: "The Roman Empire".into(),
            author: "Gregory S. Aldrete".into(),
            special: false,
            description: "Hail Caesar!".into(),
        }

        Book {
            number: 108,
            title: "The Rise of Rome".into(),
            author: "Gregory S. Aldrete".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 107,
            title: "The Smartest Guys In The Room".into(),
            author: "Bethany McLean".into(),
            special: true,
            description: "It's always interesting to ready more about these interconnected web of major corporations that had a large impact on american society, mainly because we learn about these things as if they happened in a vacuum while in reality thousands of other things were going on and much of it interweaved. After reading books about Bechtel and McKinsey and seeing how those companies had a large impact on Enron and how it all feeds one another, one realizes what a complicated beast humanity is. I also just thought it's interesting seeing how historically, periods of exuberance come again and again and how our window of pattern recognition is far smaller than how often these periods occur. Problems are always ignored when the momentum of things are positive and it is only until the floor from under us begins to shake that we realize our folly. Innate suscpicion is the only cure to exaulted hubris, but it's a fine line between conservatism and letting opportunities pass.".into(),
        }

        Book {
            number: 106,
            title: "The Firm".into(),
            author: "Duff McDonald".into(),
            special: true,
            description: "This is a history of McKinsey, the famous consulting firm. It's crazy to see that these guys literally created consulting as an industry and how it shaped business across the world. Helped bring the managerial revolution to the world and now currently has the largest alumni network, with ex-McKinsey people running or holding high positions in the majority of the fortune 500 companies.".into(),
        }

        Book {
            number: 105,
            title: "The United States and the Middle East".into(),
            author: "Salim Yaqub".into(),
            special: false,
            description: "This is a Greats Course lecture series on Audible. I've always had a fairly deep amount of knowledge with western history but lacked historical knowledge of other cultures. The whole rise of pan-Arabism, the origin of anti-US hostility, anti-Zionism and the relation between Israel and the other states, the history of colonial european powers in the area, and the dynamic the cold war played are things I think are necessary to know to understand the last century of conflicts with this part of the world.".into(),
        }

        Book {
            number: 104,
            title: "Of Mice and Men".into(),
            author: "John Steinbeck".into(),
            special: true,
            description: "I actually read this book when I was in middle school. I forgot about it and, with my current setup of the website, if I wanted to add it in correct chronological order, I'd have to manually increment each book number by one (so I'd have to do this about a hundred times). I'm just gonna put it here. This spot was actually the giver, but when I moved from my old react website to here I actually put the giver at like #4 but forgot to remove this so everything above was incremented by one. I just placed the old Giver spot with this. I'm surprised I forgot about it, it was my favorite book for the longest time.".into(),
        }

        h2 { class: "mt-16 mb-5", "[2022 | Age 19]" }

        Book {
            number: 103,
            title: "A Beautiful Mind".into(),
            author: "Sylvia Nasar".into(),
            special: false,
            description: "This is a biography of the mathematician John Nash. He produced great work in multiple areas of mathematics but you probably have heard of him from Game Theory via the Nash Equilibrium. He sufferd from Schizophrenia and lost a large chunk of his productive life due to it, kind of in a reverse Flowers for Algernon way. Started off incredibly smart and promising, slowly lost the ability to reason, and then gained it back. Sad in its own way. By then much of his life had passed him by. He still produced work but his golden years had pass. Always reminds me to not take the time I have today for granted.".into(),
        }

        Book {
            number: 102,
            title: "How Markets Fail".into(),
            author: "John Cassidy".into(),
            special: true,
            description: "This book was excellent and very broad in scope. Talked about the economic theories of Keynes, Friedman, Hayek, Minsky, and others, John Von Nuemann and game theory, causes of market failures with natural occuring prisoner dillemas, market spillovers, information failure, and many others, and dove into the events that led to the 2008 financial crises.".into(),
        }

        Book {
            number: 101,
            title: "The Lost Bank".into(),
            author: "Kristen Grind".into(),
            special: false,
            description: "This was a great telling of the story and history of Washington Mutual, the largest bank failure in U.S. History. It goes through the 132 year history of the bank and goes into great detail about option adjustable-rate and sub-prime loans. I never knew before hand how insane lending had got. They were literally lending out money to dead and homeless people, and not suprisingly, ended up losing billions in bad loans. Reading all these stories about 2008 and market failures have made me less bullish on unregulated laissez-faire libertarian free-market capitalism. There are many cases where market participants enter a prisoner's delima (which seems to be the crux of 2008) and spill overs which creates consistent market failures that can only be handled by a counter balancing arbiter (the government).".into(),
        }

        Book {
            number: 100,
            title: "Masters of DOOM".into(),
            author: "David Kushner".into(),
            special: true,
            description: "I've always been a massive fan of John Carmack. The paragon of the engineer, of competence itself. This story goes over the entire video game arc of him and John Romero. The ingenious of Carmack for every game engine he developed was other wordly. Once in a generation mind.".into(),
        }

        Book {
            number: 99,
            title: "Flash Boys".into(),
            author: "Michael Lewis".into(),
            special: true,
            description: "This book covers the rise of high frequency trading firms and the absurd length they went to gain a speed advantage over others and how they were making billions of dollars risk-free by front running everyone else's trades. Absolutely insane that nothing happened to these guys.".into(),
        }

        Book {
            number: 98,
            title: "The Machiavellians".into(),
            author: "James Burnham".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 97,
            title: "Human Nature".into(),
            author: "Robert Greene".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 96,
            title: "The Lessons Of History".into(),
            author: "Will & Ariel Durant".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 95,
            title: "Atlas Shrugged".into(),
            author: "Ayn Rand".into(),
            special: true,
            description: "I did love Ayn Rand's novels. I think I did prefer 'The Fountainhead' for the most part, but this story does a great job at appealing to the ideal side of humanism. Ayn Rand always gets a bad rap for essentially being humanist propaganda for having simplified dichotimistic world views, but I feel like that's a part of what makes her philosophy speak to many on a deep level. Yes, her characters are black and white, but it paints an archetypal ideal of the best and worst of man and paints an image of what we could inspire to be: individuals who make the world a better place through human ingenuity alone despite all else.".into(),
        }

        Book {
            number: 94,
            title: "The Road To Serfdom".into(),
            author: "F. A. Hayek".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 93,
            title: "Zero To One".into(),
            author: "Peter Thiel".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 92,
            title: "The Fountain Head".into(),
            author: "Ayn Rand".into(),
            special: true,
            description: "Already wrote a bit about Rand for Atlas Shrugged (banger title btw). I think that this is her best work as it just solely inspires the creative side of man through the archetypal story of a hero that will complete his quest no matter what and who stays true to himself and his vision despite the world telling him that his perception of the world is incorrect.".into(),
        }

        Book {
            number: 91,
            title: "The New Right".into(),
            author: "Michael Malice".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 90,
            title: "A People's History Of The United States".into(),
            author: "Howard Zinn".into(),
            special: false,
            description: "I think that this is an important book for everyone to read. I disagreed with a lot of things and the book is very heavily biased, but as Zinn points out himself, it is to give the counterview of the majority of mainstream history.".into(),
        }

        h2 { class: "mt-16 mb-5", "[2021 | Age 18]" }

        Book {
            number: 89,
            title: "The Soverign Individual".into(),
            author: "James Dale Davidson & William Rees-Mogg".into(),
            special: true,
            description: "There seems to be a trend in human history where new innovations start under a centralized institution. The internet and dozens of other innovations under the U.S. Military. Literateness used to be solely for the royal and religious sectors of society. Civilization itself, under the egyptians, babylonians, etc, started under a centralized power structure. As society progresses, such innovations trickle down from centralized institutions down to individuals. This book talks a lot about this past trend and tries to predict future trends from it, and most of the predictions have held out quite well.".into(),
        }

        Book {
            number: 88,
            title: "The Origin Of Virtue".into(),
            author: "Matt Ridley".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 87,
            title: "The Rational Male".into(),
            author: "Rollo Tamassi".into(),
            special: false,
            description: "The original redpill manefesto. I personally don't agree with this life philosophy, but I do think it has a lot of things any young male should be conscious of. One may disagree with the nihilistic hypergamistic view of cross-gender relations or disagree with the opinion that men's lives are harder, but at the base of the book is just vouching for good habits and looking inwards for validation instead of from the people around us.".into(),
        }

        Book {
            number: 86,
            title: "The Infinite Machine".into(),
            author: "Camila Russo".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 85,
            title: "The Bitcoin Standard".into(),
            author: "Saifedean Ammous".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 84,
            title: "Beyond Order".into(),
            author: "Jordan B. Peterson".into(),
            special: false,
            description: "I used to be a big Jordan B. Peterson fan when I was younger. I liked his earliest stuff with his Maps of Meaning and Biblical Series. I do enjoy the talks about morality and philosophy. I've listened to the majority of content he had produced up to 2020 and have found much of the new to be rehash of the old. I saw him give a lecture in Austin, his first public tour in a couple of years. It was all stuff I've already heard him say in the hundreds of hours I've listened to him. I feel like that's how this book felt to me.".into(),
        }

        Book {
            number: 83,
            title: "Colenel Roosevelt".into(),
            author: "Edmumd Moris".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 82,
            title: "21 Lessons For The 21st Century".into(),
            author: "Yuval Noah Harari".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 81,
            title: "Homo Dues".into(),
            author: "Yuval Noah Harari".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 80,
            title: "Life 3.0".into(),
            author: "Max Tegmark".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 79,
            title: "The Nuremberg Trials".into(),
            author: "Paul Roland".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 78,
            title: "Give Me Tomorrow".into(),
            author: "Patrick K. O'Donnel".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 77,
            title: "In The Shadows Of The American Century".into(),
            author: "Alfred W. McCoy".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 76,
            title: "How The Internet Happened".into(),
            author: "Brian McCullough".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 75,
            title: "Up From Slavery".into(),
            author: "Booker T. Washington".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 74,
            title: "Bury My Heart At Wounded Knee".into(),
            author: "Dee Brown".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 73,
            title: "Super Intelligence".into(),
            author: "Nick Bostrom".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 72,
            title: "About Face".into(),
            author: "David H. Hackworth".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 71,
            title: "All Quiet On The Western Front".into(),
            author: "Erich Maria Remarque".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 70,
            title: "Sapiens".into(),
            author: "Yuval Noah Harari".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 69,
            title: "American Ulysses".into(),
            author: "Ronald C. White".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 68,
            title: "Theodore Rex".into(),
            author: "Edmund Moris".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 67,
            title: "Haroun And The Sea Of Stories".into(),
            author: "Salman Rushdie".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 66,
            title: "The Power Broker".into(),
            author: "Robert Caro".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 65,
            title: "Endurance: Shackleton's Incredible Voyage".into(),
            author: "Alfred Lansing".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 64,
            title: "Surely You're Joking, Mr. Feynman".into(),
            author: "Richard Feynman".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 63,
            title: "The Great Gatsby".into(),
            author: "F. Scott Fitzgerald".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 62,
            title: "The Rise Of Theodore Roosevelt".into(),
            author: "Edmund Moris".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 61,
            title: "Atomic Habits".into(),
            author: "James Clear".into(),
            special: true,
            description: "The most important insight I got from this book was in the first 20 pages. It verbalized something that I've vaguely understood. It was the fact that your habits reinforce your identity and your identity reinforces your habits. If you wake up early, you probably identify as a morning person. Because you identify as a morning person, there's much less cognative friction in waking up early. It's just *who you are*. It's just *what you do*. Anytime I want to make a positive change in my life, I try to just become someone that does that. I want to get good at math? I'll just dive into it and *become a math guy*. Goes for anything.".into(),
        }

        Book {
            number: 60,
            title: "Brave New World".into(),
            author: "Aldus Huxley".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 59,
            title: "Alice In Wonderland".into(),
            author: "Lewis Carrol".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 58,
            title: "The Rise And Fall Of The Third Reich".into(),
            author: "William L. Shirer".into(),
            special: true,
            description: "".into(),
        }

        h2 { class: "mt-16 mb-5", "[2020 | Age 17]" }

        Book {
            number: 57,
            title: "Memories, Dreams, Reflections".into(),
            author: "Carl Jung".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 56,
            title: "Ecce Homo".into(),
            author: "Fridrich Nietsche".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 55,
            title: "Benjamin Franklin".into(),
            author: "Walter Isaacson".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 54,
            title: "The Alchemist".into(),
            author: "Paulo Coelho".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 53,
            title: "Crime And Punishment".into(),
            author: "Fyodor Dostoevsky".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 52,
            title: "The Things They Carried".into(),
            author: "Tim O'Brian".into(),
            special: false,
            description: "I really liked this book. I'm always been a big fan of vietnam media like Full Metal Jacket and Apolalypse Now (which is just Conrad's The Heart Of Darkness told in a modern setting). It's fascinating because these stories always delve into the minds of men who were thrown into unthinkable circumstances but who are quite relatable. WW2 and every war before seems as if it is from a distant time, but Vietnam, it is actually comprehensible how we went from there to now.".into(),
        }

        Book {
            number: 51,
            title: "Notes From The Underground".into(),
            author: "Fyodor Dostoevsky".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 50,
            title: "A Short History Of Nearly Everything".into(),
            author: "Bill Bryson".into(),
            special: true, // Assuming the value 50 should be interpreted as a boolean `true`
            description: "".into(),
        }

        Book {
            number: 49,
            title: "The Compound Effect".into(),
            author: "Darren Hardy".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 48,
            title: "Beowulf".into(),
            author: "Ancient Tale".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 47,
            title: "Animal Farm".into(),
            author: "George Orwell".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 46,
            title: "The Coddling Of The American Mind".into(),
            author: "Greg Lukianoff & Jonathan Haidt".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 45,
            title: "The Boys In The Boat".into(),
            author: "Daniel James Brown".into(),
            special: true,
            description: "My high school cross country coach, Bob Miller, brought up this boat. A beautiful story of a group of young boys sacrificing for something greater than themselves. Truly one of the most special moments a human can experience. One day I wish to do the same.".into(),
        }

        Book {
            number: 44,
            title: "Bad Blood".into(),
            author: "John Carreyrou".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 43,
            title: "The 50th Law".into(),
            author: "Robert Greene & 50 Cent".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 42,
            title: "AI Superpowers".into(),
            author: "Kai-Fu Lee".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 41,
            title: "The Autobiography Of Ben Franklin".into(),
            author: "Ben Franklin".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 40,
            title: "Can't Hurt Me".into(),
            author: "David Goggins".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 39,
            title: "The Denial Of Death".into(),
            author: "Ernest Becker".into(),
            special: true,
            description: "".into(),
        }

        h2 { class: "mt-16 mb-5", "[2019 | Age 16]" }

        Book {
            number: 38,
            title: "The Tipping Point".into(),
            author: "Malcom Gladwell".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 37,
            title: "Talking To Strangers".into(),
            author: "Malcom Gladwell".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 36,
            title: "Elon Musk".into(),
            author: "Ashlee Vance".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 35,
            title: "Man's Search For Meaning".into(),
            author: "Viktor E. Frankl".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 34,
            title: "The River Of Doubt".into(),
            author: "Candice Millard".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 33,
            title: "Rich Dad Poor Dad".into(),
            author: "Robert Kiyosaki".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 32,
            title: "Be Obsessed Or Be Average".into(),
            author: "Grant Cardone".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 31,
            title: "The Art Of War".into(),
            author: "Sun Tzu".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 30,
            title: "Steve Jobs".into(),
            author: "Walter Isaacson".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 29,
            title: "The Snowball".into(),
            author: "Alice Schroeder".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 28,
            title: "The Art Of Seduction".into(),
            author: "Robert Greene".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 27,
            title: "E-Myth Revisited".into(),
            author: "Michael Gerber".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 26,
            title: "Andrew Carnegie".into(),
            author: "David Nasaw".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 25,
            title: "The Selfish Gene".into(),
            author: "Richard Dawkins".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 24,
            title: "Alexander Hamilton's Guide To Life".into(),
            author: "Jeff Wilser".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 23,
            title: "Managing One's Self".into(),
            author: "Peter Drucker".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 22,
            title: "The Curious Incident Of The Dog In The Night".into(),
            author: "Mark Haddon".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 21,
            title: "Mango Street".into(),
            author: "Sandra Cisneros".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 20,
            title: "Explosive Growth".into(),
            author: "Cliff Lerner".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 19,
            title: "Smarter, Faster, Better".into(),
            author: "Charles Duhigg".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 18,
            title: "12 Rules For Life".into(),
            author: "Jordan B. Peterson".into(),
            special: true,
            description: "".into(),
        }

        h2 { class: "mt-16 mb-5", "[2018 | Age 15]" }

        Book {
            number: 17,
            title: "The Richest Man In Babylon".into(),
            author: "George S. Clason".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 16,
            title: "How To Win Friends And Influence People".into(),
            author: "Dale Carnegie".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 15,
            title: "Blink".into(),
            author: "Malcom Gladwell".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 14,
            title: "Drop Out And Get Schooled".into(),
            author: "Patrick Bet-David".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 13,
            title: "Exactly What To Say".into(),
            author: "Phil M. Jones".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 12,
            title: "Unlimited Memory".into(),
            author: "Kevin Horsley".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 11,
            title: "The 1 Page Marketing Plan".into(),
            author: "Allan Dib".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 10,
            title: "Flowers for Algernon".into(),
            author: "Daniel Keyes".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 9,
            title: "Sell Or Be Sold".into(),
            author: "Grant Cardone".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 8,
            title: "The Power Of Habit".into(),
            author: "Charles Duhigg".into(),
            special: false,
            description: "".into(),
        }

        h2 { class: "mt-16 mb-5", "[2017 | Age 14]" }
        // Book {
        //     number: ,
        //     title: "".into(),
        //     author: "".into(),
        //     special: false,
        //     description: "".into(),
        // }

        Book {
            number: 7,
            title: "Mastery".into(),
            author: "Robert Greene".into(),
            special: true,
            description: "".into(),
        }

        Book {
            number: 6,
            title: "1984".into(),
            author: "George Orwell".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 5,
            title: "To Kill A Mockingbird".into(),
            author: "Harper Lee".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 4,
            title: "Unbroken".into(),
            author: "Laura Hillenbrand".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 3,
            title: "The Giver".into(),
            author: "Lois Lowry".into(),
            special: false,
            description: "".into(),
        }

        Book {
            number: 2,
            title: "Excellent Sheep".into(),
            author: "William Deresiewicz".into(),
            special: false,
            description: "".into(),
        }

        h2 { class: "mt-16 mb-5", "[2016 | Age 13]" }

        Book {
            number: 1,
            title: "The 48 Laws of Power".into(),
            author: "Robert Green".into(),
            special: true,
            description: "".into(),
        }

        p { class: "mt-16 italic",
            "\"The average person puts only 25% of his energy and ability into his
            work. The world takes off its hat to those who put in more than 50% of
            their capacity, and stands on its head for those few and far between
            souls who devote 100%\" - Andrew Carnegie"
        }

    }
}

