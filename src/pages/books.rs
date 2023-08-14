use dioxus::prelude::*;
use crate::components::book::Book;

#[inline_props]
pub fn Books(cx: Scope) -> Element {
    render! {
        // Top paragraph
        p { class: "mb-4",
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
            title: String::from("The House of Morgan"),
            author: String::from("Ron Chernow"),
            special: false,
            description: String::from("This behemoth of a book is about the Morgan banking dynasty, from George Peabody and J.S. Morgan to now. Because the Morgan Dynasty has been around for 200 years, this book also is nearly a history of modern banking as a whole. What I thought was interesting was the evolution of the relation between bank and the state. From king makers to diplomats to tools have they fallen."),
        }

        Book {
            number: 129,
            title: String::from("The Odyssey"),
            author: String::from("Translated by Emily Wilson"),
            special: true,
            description: String::from("Surprisingly, I've never read many 'classics'. Emily Wilson did a fantastic translation. I did enjoy the story. These old archetypal stories always reflect aspects of the human condition that we can relate to and cause us to reflect deeper about our lives. We are all the hero in our own lives on our own journey, and we must make the best of it."),
        }

        Book {
            number: 128,
            title: String::from("Physics for Engineers and Scientists, Vol. 1"),
            author: String::from("Hans C. Ohanian & John T. Markert"),
            special: true,
            description: String::from("This is the first textbook I've actually nearly read to from cover to cover. Everything excepct for the sections on heat and thermodynamics. It was a very good textbook. I noticed that there's a really large gap in my understanding of the world. Math and computer science is solid knowledge, but in terms of how the world actually works, I've been fully ignorant until now."),
        }

        Book {
            number: 127,
            title: String::from("Six Easy Pieces"),
            author: String::from("Richard P. Feynman"),
            special: false,
            description: String::from("Feels great to actually understand physics (currently also taking a physics sequence at UT). After a couple of books about general relativity and quantum mechanics, I'll tap out of physics. I'll have a reasonable understanding. I can die satisfied."),
        }

        Book {
            number: 126,
            title: String::from("Rome: An Empire's Story"),
            author: String::from("Greg Woolf"),
            special: false,
            description: String::from("I loved the end, when it was talking about what causes a culture to create monuments. The late republic and early empire, tons of monuments were built. People that build monuments have enough trust that there will be someone there in the future to remember them. Very few monuments were built after the early life of the empire, which also lines up with the start of its decline. Of course, economic factors play in, but maybe people didn't believe that future generations would be there for them? Or at least future generations that they cared about."),
        }

        Book {
            number: 125,
            title: String::from("The Man From The Future"),
            author: String::from("Ananyo Bhattacharya"),
            special: false,
            description: String::from("This book talks about each field that John von Neumann was involved in: quantum mechanics, the manhatten project, computers, and cellular automaton. It's too late for me, but there seems to be a systematic way to raise kids with a touch of genius. Every time I think about it, focusing my entire life on gathering resources and then pivoting all my focus on my future kids seems like it should surely have a higher long term expected value than me trying to reach great heights, as my kids would surely be able to do more. Of course, the future is always full of uncertainties."),
        }

        Book {
            number: 124,
            title: String::from("American Prometheus"),
            author: String::from("Kai Bird & Martin J. Sherwin"),
            special: true,
            description: String::from("This is a great biography about J. Robert Oppenheimer. I've read so many books about such great figures. It's strange, going through someone's entire life in just a week. It always leaves an impression on me greatly, reading about their death, and Oppenheimer is of no exception. Wasted away from cancer. Hearing about how people change when faced with their own mortality, it always makes me stop and ponder what I am doing with my life."),
        }

        Book {
            number: 123,
            title: String::from("Einstein"),
            author: String::from("Walter Isaacson"),
            special: false,
            description: String::from("This book became so much better and more enriching after I learned about the basics of topology and the history of non-Euclidian geometry and the early Prussian university system with the characters and historic figues that came out of there. I love more about the same things through different lenses. Great biography in general."),
        }

        Book {
            number: 122,
            title: String::from("Algorithms to Live By"),
            author: String::from("Brian Christian & Tom Griffiths"),
            special: false,
            description: String::from("This book made me wonder how much knowledge I didn't catch throughout my life. I read a part of this book in the past before. Rereading it again, after taking computer architecture and OS, is a completely different experience. Things that I didn't understand or pick up on are now obvious to me and the amount I take away is greatly different."),
        }

        Book {
            number: 121,
            title: String::from("Gödel's Proof"),
            author: String::from("Ernest Nagel & James R. Newman"),
            special: true,
            description: String::from("For the first time in my life, I feel like I actually have a fairly intuitive understanding of Gödel's Incompleteness Theorem, to a point to where I could explain it to a child. I decided to read this because I started Gödel Escher Bach and wanted to truly understand the concept that sparked it all before reading such a Tour De Force."),
        }

        Book {
            number: 120,
            title: String::from("The Joy of X"),
            author: String::from("Steven Strogatz"),
            special: false,
            description: String::from("Just another book about math history and trivia. One thing that I thought was super interesting is that it mentioned how to optimal stopping point for any problem where you know the size, the solution is 1/e. In another book, 'Agorithms To Live By', it mentioned this, but it just said .37% (the rounded decimal version of 1/e). I'm just reminded of how amazing math is and how it is unexplanably beautiful and tied together."),
        }

        Book {
            number: 119,
            title: String::from("The Creature from Jekyll Island"),
            author: String::from("G. Edward Griffin"),
            special: true,
            description: String::from("This is mandatory reading. This book does what Howard Zinn's 'The People's History Of The United States' wanted to do, while funnily being on the exact opposite of the political spectrum. I'm always reminded how, in response to the criticism of conspiracy theories in general, people remind us that the founding of our country was a massive conspiracy theory. The more I exist, the more I realize that a lot of the largest socio, political, and economic events in human history have all been just strings of conspiracy theories, and we are almost certainly living amongst many at the current moment."),
        }

        Book {
            number: 118,
            title: String::from("How Google Works"),
            author: String::from("Eric Schmidt & Jonathan Rosenberg"),
            special: false,
            description: String::from("Bland. This was a mix between 'History of Google' and 'Google's management philosophy'. The history part is nice, the latter was just very generic stuff. Most likely my perspective. When this book came out these ideas could have been novel, but throughout the years, much of this advice propagated to most of the startups out there to the point where a lot of this is common knowledge"),
        }

        Book {
            number: 117,
            title: String::from("A Mind at Play"),
            author: String::from("Rob Goodman & Jimmy Soni"),
            special: true,
            description: String::from("I love Claude Shannon. Really a human to cause inspires in others. The archetypal tinkerer. Just a terrific man. While I might not ever be able to outshine his genius, I might one day be able to juggle better than him (5 balls instead of 4)!"),
        }

        Book {
            number: 116,
            title: String::from("The Poincaré Conjecture"),
            author: String::from("Donal O'Shea"),
            special: true,
            description: String::from("A truly fantastic book, one of the best I've ever read. Goes through the history of math, from Euclid to Gauss, Riemann, etc. and covers the origin of geometry, topology, calculus, and other foundations of math and how it all connects. I very much appreciated its intro to topology as I'm very new to a lot of higher math topics. Truly a wonderful book and has helped spark a deeper curiousity and love for math within me."),
        }

        Book {
            number: 115,
            title: String::from("The Everthing Store"),
            author: String::from("Brad Stone"),
            special: true,
            description: String::from("This book is about Jeff Bezos and the founding of Amazon. Really surprised me. Who knows how much the lens of this story warps the reality, but from this book, it seems like Jeff Bezos doing something great was inevitable. I recently read half of 'Shoe Dog', the founding story about nike, and it really seems like the guy just got insanely lucky. Jeff Bezos seems to be at the very opposite of the spectrum; it was inevitable he would do something great. Very much holds admirable traits to emulate."),
        }

        Book {
            number: 114,
            title: String::from("All the Math You Missed"),
            author: String::from("Thomas A. Garrity"),
            special: false,
            description: String::from("This covers everything from Linear Algebra to Topology, from Differential Equations to Non-Euclidian Geometry. It was really nice to just see everything that one could cover in an undergraduate math degree and introduced me to a bunch of new topics."),
        }

        Book {
            number: 113,
            title: String::from("Cracking The Coding Interview"),
            author: String::from("Gayle Laakmann Mcdowell"),
            special: false,
            description: String::from("Just had to go through this quickly to refresh my memory"),
        }

        Book {
            number: 112,
            title: String::from("Chip War"),
            author: String::from("Chris Miller"),
            special: false,
            description: String::from("This book is about the history of the microchip industry, mentioning the rise of globalism, of Japan, Tiwan, South Korea, the history of Silicon Valley and its relation with the US military, and the national security concerns China and the US has over the lack of control over technologies that make up the majority of their military might. Fantastic book, right down my alley."),
        }

        Book {
            number: 111,
            title: String::from("Empire of Pain"),
            author: String::from("Patrick Radden Keefe"),
            special: false,
            description: String::from("This book is about the Sackler family and the opioid pandemic. I realized that, like how a lot of women enjoy true crime stories, I am a sucker for the male equivilant: white-collar crime stories. This is like the 8th book I've read over some corporate story of fraud and greed."),
        }

        Book {
            number: 110,
            title: String::from("No Longer Human"),
            author: String::from("Osamu Dazai"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 109,
            title: String::from("The Roman Empire"),
            author: String::from("Gregory S. Aldrete"),
            special: false,
            description: String::from("Hail Caesar!"),
        }

        Book {
            number: 108,
            title: String::from("The Rise of Rome"),
            author: String::from("Gregory S. Aldrete"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 107,
            title: String::from("The Smartest Guys In The Room"),
            author: String::from("Bethany McLean"),
            special: true,
            description: String::from("It's always interesting to ready more about these interconnected web of major corporations that had a large impact on american society, mainly because we learn about these things as if they happened in a vacuum while in reality thousands of other things were going on and much of it interweaved. After reading books about Bechtel and McKinsey and seeing how those companies had a large impact on Enron and how it all feeds one another, one realizes what a complicated beast humanity is. I also just thought it's interesting seeing how historically, periods of exuberance come again and again and how our window of pattern recognition is far smaller than how often these periods occur. Problems are always ignored when the momentum of things are positive and it is only until the floor from under us begins to shake that we realize our folly. Innate suscpicion is the only cure to exaulted hubris, but it's a fine line between conservatism and letting opportunities pass."),
        }

        Book {
            number: 106,
            title: String::from("The Firm"),
            author: String::from("Duff McDonald"),
            special: true,
            description: String::from("This is a history of McKinsey, the famous consulting firm. It's crazy to see that these guys literally created consulting as an industry and how it shaped business across the world. Helped bring the managerial revolution to the world and now currently has the largest alumni network, with ex-McKinsey people running or holding high positions in the majority of the fortune 500 companies."),
        }

        Book {
            number: 105,
            title: String::from("The United States and the Middle East"),
            author: String::from("Salim Yaqub"),
            special: false,
            description: String::from("This is a Greats Course lecture series on Audible. I've always had a fairly deep amount of knowledge with western history but lacked historical knowledge of other cultures. The whole rise of pan-Arabism, the origin of anti-US hostility, anti-Zionism and the relation between Israel and the other states, the history of colonial european powers in the area, and the dynamic the cold war played are things I think are necessary to know to understand the last century of conflicts with this part of the world."),
        }

        Book {
            number: 104,
            title: String::from("Of Mice and Men"),
            author: String::from("John Steinbeck"),
            special: true,
            description: String::from("I actually read this book when I was in middle school. I forgot about it and, with my current setup of the website, if I wanted to add it in correct chronological order, I'd have to manually increment each book number by one (so I'd have to do this about a hundred times). I'm just gonna put it here. This spot was actually the giver, but when I moved from my old react website to here I actually put the giver at like #4 but forgot to remove this so everything above was incremented by one. I just placed the old Giver spot with this. I'm surprised I forgot about it, it was my favorite book for the longest time."),
        }

        h2 { class: "mt-16 mb-5", "[2022 | Age 19]" }

        Book {
            number: 103,
            title: String::from("A Beautiful Mind"),
            author: String::from("Sylvia Nasar"),
            special: false,
            description: String::from("This is a biography of the mathematician John Nash. He produced great work in multiple areas of mathematics but you probably have heard of him from Game Theory via the Nash Equilibrium. He sufferd from Schizophrenia and lost a large chunk of his productive life due to it, kind of in a reverse Flowers for Algernon way. Started off incredibly smart and promising, slowly lost the ability to reason, and then gained it back. Sad in its own way. By then much of his life had passed him by. He still produced work but his golden years had pass. Always reminds me to not take the time I have today for granted."),
        }

        Book {
            number: 102,
            title: String::from("How Markets Fail"),
            author: String::from("John Cassidy"),
            special: true,
            description: String::from("This book was excellent and very broad in scope. Talked about the economic theories of Keynes, Friedman, Hayek, Minsky, and others, John Von Nuemann and game theory, causes of market failures with natural occuring prisoner dillemas, market spillovers, information failure, and many others, and dove into the events that led to the 2008 financial crises."),
        }

        Book {
            number: 101,
            title: String::from("The Lost Bank"),
            author: String::from("Kristen Grind"),
            special: false,
            description: String::from("This was a great telling of the story and history of Washington Mutual, the largest bank failure in U.S. History. It goes through the 132 year history of the bank and goes into great detail about option adjustable-rate and sub-prime loans. I never knew before hand how insane lending had got. They were literally lending out money to dead and homeless people, and not suprisingly, ended up losing billions in bad loans. Reading all these stories about 2008 and market failures have made me less bullish on unregulated laissez-faire libertarian free-market capitalism. There are many cases where market participants enter a prisoner's delima (which seems to be the crux of 2008) and spill overs which creates consistent market failures that can only be handled by a counter balancing arbiter (the government)."),
        }

        Book {
            number: 100,
            title: String::from("Masters of DOOM"),
            author: String::from("David Kushner"),
            special: true,
            description: String::from("I've always been a massive fan of John Carmack. The paragon of the engineer, of competence itself. This story goes over the entire video game arc of him and John Romero. The ingenious of Carmack for every game engine he developed was other wordly. Once in a generation mind."),
        }

        Book {
            number: 99,
            title: String::from("Flash Boys"),
            author: String::from("Michael Lewis"),
            special: true,
            description: String::from("This book covers the rise of high frequency trading firms and the absurd length they went to gain a speed advantage over others and how they were making billions of dollars risk-free by front running everyone else's trades. Absolutely insane that nothing happened to these guys."),
        }

        Book {
            number: 98,
            title: String::from("The Machiavellians"),
            author: String::from("James Burnham"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 97,
            title: String::from("Human Nature"),
            author: String::from("Robert Greene"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 96,
            title: String::from("The Lessons Of History"),
            author: String::from("Will & Ariel Durant"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 95,
            title: String::from("Atlas Shrugged"),
            author: String::from("Ayn Rand"),
            special: true,
            description: String::from("I did love Ayn Rand's novels. I think I did prefer 'The Fountainhead' for the most part, but this story does a great job at appealing to the ideal side of humanism. Ayn Rand always gets a bad rap for essentially being humanist propaganda for having simplified dichotimistic world views, but I feel like that's a part of what makes her philosophy speak to many on a deep level. Yes, her characters are black and white, but it paints an archetypal ideal of the best and worst of man and paints an image of what we could inspire to be: individuals who make the world a better place through human ingenuity alone despite all else."),
        }

        Book {
            number: 94,
            title: String::from("The Road To Serfdom"),
            author: String::from("F. A. Hayek"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 93,
            title: String::from("Zero To One"),
            author: String::from("Peter Thiel"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 92,
            title: String::from("The Fountain Head"),
            author: String::from("Ayn Rand"),
            special: true,
            description: String::from("Already wrote a bit about Rand for Atlas Shrugged (banger title btw). I think that this is her best work as it just solely inspires the creative side of man through the archetypal story of a hero that will complete his quest no matter what and who stays true to himself and his vision despite the world telling him that his perception of the world is incorrect."),
        }

        Book {
            number: 91,
            title: String::from("The New Right"),
            author: String::from("Michael Malice"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 90,
            title: String::from("A People's History Of The United States"),
            author: String::from("Howard Zinn"),
            special: false,
            description: String::from("I think that this is an important book for everyone to read. I disagreed with a lot of things and the book is very heavily biased, but as Zinn points out himself, it is to give the counterview of the majority of mainstream history."),
        }

        h2 { class: "mt-16 mb-5", "[2021 | Age 18]" }

        Book {
            number: 89,
            title: String::from("The Soverign Individual"),
            author: String::from("James Dale Davidson & William Rees-Mogg"),
            special: true,
            description: String::from("There seems to be a trend in human history where new innovations start under a centralized institution. The internet and dozens of other innovations under the U.S. Military. Literateness used to be solely for the royal and religious sectors of society. Civilization itself, under the egyptians, babylonians, etc, started under a centralized power structure. As society progresses, such innovations trickle down from centralized institutions down to individuals. This book talks a lot about this past trend and tries to predict future trends from it, and most of the predictions have held out quite well."),
        }

        Book {
            number: 88,
            title: String::from("The Origin Of Virtue"),
            author: String::from("Matt Ridley"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 87,
            title: String::from("The Rational Male"),
            author: String::from("Rollo Tamassi"),
            special: false,
            description: String::from("The original redpill manefesto. I personally don't agree with this life philosophy, but I do think it has a lot of things any young male should be conscious of. One may disagree with the nihilistic hypergamistic view of cross-gender relations or disagree with the opinion that men's lives are harder, but at the base of the book is just vouching for good habits and looking inwards for validation instead of from the people around us."),
        }

        Book {
            number: 86,
            title: String::from("The Infinite Machine"),
            author: String::from("Camila Russo"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 85,
            title: String::from("The Bitcoin Standard"),
            author: String::from("Saifedean Ammous"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 84,
            title: String::from("Beyond Order"),
            author: String::from("Jordan B. Peterson"),
            special: false,
            description: String::from("I used to be a big Jordan B. Peterson fan when I was younger. I liked his earliest stuff with his Maps of Meaning and Biblical Series. I do enjoy the talks about morality and philosophy. I've listened to the majority of content he had produced up to 2020 and have found much of the new to be rehash of the old. I saw him give a lecture in Austin, his first public tour in a couple of years. It was all stuff I've already heard him say in the hundreds of hours I've listened to him. I feel like that's how this book felt to me."),
        }

        Book {
            number: 83,
            title: String::from("Colenel Roosevelt"),
            author: String::from("Edmumd Moris"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 82,
            title: String::from("21 Lessons For The 21st Century"),
            author: String::from("Yuval Noah Harari"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 81,
            title: String::from("Homo Dues"),
            author: String::from("Yuval Noah Harari"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 80,
            title: String::from("Life 3.0"),
            author: String::from("Max Tegmark"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 79,
            title: String::from("The Nuremberg Trials"),
            author: String::from("Paul Roland"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 78,
            title: String::from("Give Me Tomorrow"),
            author: String::from("Patrick K. O'Donnel"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 77,
            title: String::from("In The Shadows Of The American Century"),
            author: String::from("Alfred W. McCoy"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 76,
            title: String::from("How The Internet Happened"),
            author: String::from("Brian McCullough"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 75,
            title: String::from("Up From Slavery"),
            author: String::from("Booker T. Washington"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 74,
            title: String::from("Bury My Heart At Wounded Knee"),
            author: String::from("Dee Brown"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 73,
            title: String::from("Super Intelligence"),
            author: String::from("Nick Bostrom"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 72,
            title: String::from("About Face"),
            author: String::from("David H. Hackworth"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 71,
            title: String::from("All Quiet On The Western Front"),
            author: String::from("Erich Maria Remarque"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 70,
            title: String::from("Sapiens"),
            author: String::from("Yuval Noah Harari"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 69,
            title: String::from("American Ulysses"),
            author: String::from("Ronald C. White"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 68,
            title: String::from("Theodore Rex"),
            author: String::from("Edmund Moris"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 67,
            title: String::from("Haroun And The Sea Of Stories"),
            author: String::from("Salman Rushdie"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 66,
            title: String::from("The Power Broker"),
            author: String::from("Robert Caro"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 65,
            title: String::from("Endurance: Shackleton's Incredible Voyage"),
            author: String::from("Alfred Lansing"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 64,
            title: String::from("Surely You're Joking, Mr. Feynman"),
            author: String::from("Richard Feynman"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 63,
            title: String::from("The Great Gatsby"),
            author: String::from("F. Scott Fitzgerald"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 62,
            title: String::from("The Rise Of Theodore Roosevelt"),
            author: String::from("Edmund Moris"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 61,
            title: String::from("Atomic Habits"),
            author: String::from("James Clear"),
            special: true,
            description: String::from("The most important insight I got from this book was in the first 20 pages. It verbalized something that I've vaguely understood. It was the fact that your habits reinforce your identity and your identity reinforces your habits. If you wake up early, you probably identify as a morning person. Because you identify as a morning person, there's much less cognative friction in waking up early. It's just *who you are*. It's just *what you do*. Anytime I want to make a positive change in my life, I try to just become someone that does that. I want to get good at math? I'll just dive into it and *become a math guy*. Goes for anything."),
        }

        Book {
            number: 60,
            title: String::from("Brave New World"),
            author: String::from("Aldus Huxley"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 59,
            title: String::from("Alice In Wonderland"),
            author: String::from("Lewis Carrol"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 58,
            title: String::from("The Rise And Fall Of The Third Reich"),
            author: String::from("William L. Shirer"),
            special: true,
            description: String::from(""),
        }

        h2 { class: "mt-16 mb-5", "[2020 | Age 17]" }

        Book {
            number: 57,
            title: String::from("Memories, Dreams, Reflections"),
            author: String::from("Carl Jung"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 56,
            title: String::from("Ecce Homo"),
            author: String::from("Fridrich Nietsche"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 55,
            title: String::from("Benjamin Franklin"),
            author: String::from("Walter Isaacson"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 54,
            title: String::from("The Alchemist"),
            author: String::from("Paulo Coelho"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 53,
            title: String::from("Crime And Punishment"),
            author: String::from("Fyodor Dostoevsky"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 52,
            title: String::from("The Things They Carried"),
            author: String::from("Tim O'Brian"),
            special: false,
            description: String::from("I really liked this book. I'm always been a big fan of vietnam media like Full Metal Jacket and Apolalypse Now (which is just Conrad's The Heart Of Darkness told in a modern setting). It's fascinating because these stories always delve into the minds of men who were thrown into unthinkable circumstances but who are quite relatable. WW2 and every war before seems as if it is from a distant time, but Vietnam, it is actually comprehensible how we went from there to now."),
        }

        Book {
            number: 51,
            title: String::from("Notes From The Underground"),
            author: String::from("Fyodor Dostoevsky"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 50,
            title: String::from("A Short History Of Nearly Everything"),
            author: String::from("Bill Bryson"),
            special: true, // Assuming the value 50 should be interpreted as a boolean `true`
            description: String::from(""),
        }

        Book {
            number: 49,
            title: String::from("The Compound Effect"),
            author: String::from("Darren Hardy"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 48,
            title: String::from("Beowulf"),
            author: String::from("Ancient Tale"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 47,
            title: String::from("Animal Farm"),
            author: String::from("George Orwell"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 46,
            title: String::from("The Coddling Of The American Mind"),
            author: String::from("Greg Lukianoff & Jonathan Haidt"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 45,
            title: String::from("The Boys In The Boat"),
            author: String::from("Daniel James Brown"),
            special: true,
            description: String::from("My high school cross country coach, Bob Miller, brought up this boat. A beautiful story of a group of young boys sacrificing for something greater than themselves. Truly one of the most special moments a human can experience. One day I wish to do the same."),
        }

        Book {
            number: 44,
            title: String::from("Bad Blood"),
            author: String::from("John Carreyrou"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 43,
            title: String::from("The 50th Law"),
            author: String::from("Robert Greene & 50 Cent"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 42,
            title: String::from("AI Superpowers"),
            author: String::from("Kai-Fu Lee"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 41,
            title: String::from("The Autobiography Of Ben Franklin"),
            author: String::from("Ben Franklin"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 40,
            title: String::from("Can't Hurt Me"),
            author: String::from("David Goggins"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 39,
            title: String::from("The Denial Of Death"),
            author: String::from("Ernest Becker"),
            special: true,
            description: String::from(""),
        }

        h2 { class: "mt-16 mb-5", "[2019 | Age 16]" }

        Book {
            number: 38,
            title: String::from("The Tipping Point"),
            author: String::from("Malcom Gladwell"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 37,
            title: String::from("Talking To Strangers"),
            author: String::from("Malcom Gladwell"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 36,
            title: String::from("Elon Musk"),
            author: String::from("Ashlee Vance"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 35,
            title: String::from("Man's Search For Meaning"),
            author: String::from("Viktor E. Frankl"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 34,
            title: String::from("The River Of Doubt"),
            author: String::from("Candice Millard"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 33,
            title: String::from("Rich Dad Poor Dad"),
            author: String::from("Robert Kiyosaki"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 32,
            title: String::from("Be Obsessed Or Be Average"),
            author: String::from("Grant Cardone"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 31,
            title: String::from("The Art Of War"),
            author: String::from("Sun Tzu"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 30,
            title: String::from("Steve Jobs"),
            author: String::from("Walter Isaacson"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 29,
            title: String::from("The Snowball"),
            author: String::from("Alice Schroeder"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 28,
            title: String::from("The Art Of Seduction"),
            author: String::from("Robert Greene"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 27,
            title: String::from("E-Myth Revisited"),
            author: String::from("Michael Gerber"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 26,
            title: String::from("Andrew Carnegie"),
            author: String::from("David Nasaw"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 25,
            title: String::from("The Selfish Gene"),
            author: String::from("Richard Dawkins"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 24,
            title: String::from("Alexander Hamilton's Guide To Life"),
            author: String::from("Jeff Wilser"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 23,
            title: String::from("Managing One's Self"),
            author: String::from("Peter Drucker"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 22,
            title: String::from("The Curious Incident Of The Dog In The Night"),
            author: String::from("Mark Haddon"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 21,
            title: String::from("Mango Street"),
            author: String::from("Sandra Cisneros"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 20,
            title: String::from("Explosive Growth"),
            author: String::from("Cliff Lerner"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 19,
            title: String::from("Smarter, Faster, Better"),
            author: String::from("Charles Duhigg"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 18,
            title: String::from("12 Rules For Life"),
            author: String::from("Jordan B. Peterson"),
            special: true,
            description: String::from(""),
        }

        h2 { class: "mt-16 mb-5", "[2018 | Age 15]" }

        Book {
            number: 17,
            title: String::from("The Richest Man In Babylon"),
            author: String::from("George S. Clason"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 16,
            title: String::from("How To Win Friends And Influence People"),
            author: String::from("Dale Carnegie"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 15,
            title: String::from("Blink"),
            author: String::from("Malcom Gladwell"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 14,
            title: String::from("Drop Out And Get Schooled"),
            author: String::from("Patrick Bet-David"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 13,
            title: String::from("Exactly What To Say"),
            author: String::from("Phil M. Jones"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 12,
            title: String::from("Unlimited Memory"),
            author: String::from("Kevin Horsley"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 11,
            title: String::from("The 1 Page Marketing Plan"),
            author: String::from("Allan Dib"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 10,
            title: String::from("Flowers for Algernon"),
            author: String::from("Daniel Keyes"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 9,
            title: String::from("Sell Or Be Sold"),
            author: String::from("Grant Cardone"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 8,
            title: String::from("The Power Of Habit"),
            author: String::from("Charles Duhigg"),
            special: false,
            description: String::from(""),
        }

        h2 { class: "mt-16 mb-5", "[2017 | Age 14]" }
        // Book {
        //     number: ,
        //     title: String::from(""),
        //     author: String::from(""),
        //     special: false,
        //     description: String::from(""),
        // }

        Book {
            number: 7,
            title: String::from("Mastery"),
            author: String::from("Robert Greene"),
            special: true,
            description: String::from(""),
        }

        Book {
            number: 6,
            title: String::from("1984"),
            author: String::from("George Orwell"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 5,
            title: String::from("To Kill A Mockingbird"),
            author: String::from("Harper Lee"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 4,
            title: String::from("Unbroken"),
            author: String::from("Laura Hillenbrand"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 3,
            title: String::from("The Giver"),
            author: String::from("Lois Lowry"),
            special: false,
            description: String::from(""),
        }

        Book {
            number: 2,
            title: String::from("Excellent Sheep"),
            author: String::from("William Deresiewicz"),
            special: false,
            description: String::from(""),
        }

        h2 { class: "mt-16 mb-5", "[2016 | Age 13]" }

        Book {
            number: 1,
            title: String::from("The 48 Laws of Power"),
            author: String::from("Robert Green"),
            special: true,
            description: String::from(""),
        }

        p { class: "mt-16 italic",
            "\"The average person puts only 25% of his energy and ability into his
            work. The world takes off its hat to those who put in more than 50% of
            their capacity, and stands on its head for those few and far between
            souls who devote 100%\" - Andrew Carnegie"
        }

    }
}

