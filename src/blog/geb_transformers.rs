use dioxus::prelude::*;

use crate::components::container::Container;

#[inline_props]
pub fn GEBTransformers(cx: Scope) -> Element {
    render! {
        h1 { class: "mt-24 mb-8 text-center", "Gödel Escher Bach, Quines, and Transformers" },

        p { class: " text-center", "This article is absolute ass and I'm going to rip it to shreds and redo it."
        p { class: "mt-8 mb-24 italic text-center", "2021-03-03 * Work in Progress" }
        }

        Container {
            header: "Gödel Escher Bach, Quines, and Transformers".into(),
            p { 
                class: "mt-8 mb-16",
                style: "white-space: pre-line",

                "In the worlds of artificial intelligence and machine learning, the scope of what is possible continues to expand. State-of-the-art large language models (LLMs) have already demonstrated remarkable capabilities in terms of learning from input and generating relevant output. But one area that hasn't been fully explored yet is the concept of self-aware transformers: models that understand their own architecture, memory system, limitations, and capabilities.

                This idea of self-reference in systems, while revolutionary for AI, is not new in other domains. Mathematical logic and cognitive science have long been fascinated with self-reference, and these concepts are notably discussed in Douglas Hofstadter's Pulitzer Prize-winning book, \"Gödel, Escher, Bach: An Eternal Golden Braid.\""
            }
        }

        Container {
            header: "The LLMs Agent Innovation and Limitations".into(),
            p { 
                class: "mt-8 mb-16",
                style: "white-space: pre-line",

                "Most present-day large language model agents operate on a system involving a vector database that acts as a memory store for previous output. This database allows the LLM to incrementally build upon prior knowledge, creating an iterative learning process that aids in generating novel and relevant responses. This, in essence, enables LLMs to \"learn\" and respond to new information in a contextually appropriate manner.

                However, this advanced system still lacks one crucial aspect of intelligence – self-awareness. These models don't understand their own architecture, operation, or the systems they’re embedded within. They can interact with APIs and handle complex tasks, yet they are blind to their own internal mechanics.

                This limitation is critical, as a system that doesn't understand its own workings is inherently capped in the complex actions it can take. By not comprehending the intricacies of the system they operate in, these LLMs are restricted in their ability to optimize their responses or troubleshoot their processes. They cannot dynamically adapt to new scenarios that require an internal modification of their operation.

                Even when interacting with external APIs or systems, their potential for efficiency and efficacy is compromised due to this lack of self-knowledge. Just like a mechanic who knows how to change a tire but doesn't understand how an engine works can only go so far in repairing a vehicle, an LLM without self-knowledge can only perform tasks to the extent of its pre-defined capabilities. It cannot innovate or optimize beyond its initial programming, thereby limiting the range of complex actions it can perform.

                In essence, without the ability to understand and modify their own internal mechanics, the current state of LLMs restricts their full potential in responding to, learning from, and navigating within their system environment. Therefore, introducing self-awareness and the ability to self-modify into these models could dramatically enhance their capabilities and performance."
            }
        }

        Container {
            header: "Gödel’s Incompleteness Theorem and Self-Reference".into(),
            p { 
                class: "mt-8 mb-16",
                style: "white-space: pre-line",

                "This situation echoes Kurt Gödel's Incompleteness Theorem, one of the cornerstone insights in mathematical logic. Gödel's theorem essentially states that within any consistent mathematical system, there will always be statements that cannot be proven true or false using the rules of that system.

                In essence, Gödel's theorem illuminates the limitations of self-contained systems, particularly when those systems are used to analyze or describe themselves. This process of self-reference is a vital component of intelligence, leading us to Hofstadter's fascinating exploration of the theme in \"Gödel, Escher, Bach\".

                Learning from \"Gödel, Escher, Bach\" In \"Gödel, Escher, Bach,\" Hofstadter takes an interdisciplinary approach, drawing from fields like mathematics, art, and music to delve into the theme of self-reference and its potential role in consciousness and intelligence. The titular figures — a mathematician, an artist, and a composer — all incorporate self-reference in their work, showcasing it as a concept that transcends disciplinary boundaries.

                According to Hofstadter, self-reference — and recursion, the process by which a function calls itself — can create complex, 'intelligent' systems. This insight is vital when applied to the context of AI and machine learning. If a system continually references and interacts with itself in increasingly complex ways, it can give rise to novel patterns and behaviors — a phenomenon we could label as 'intelligence.'

                Take, for example, a piece of code that operates recursively. Even if the basic operation is simple, the repeated self-referential interactions can produce intricate and unexpected outcomes. Similarly, intelligence might be seen as the emergent product of simple neuronal processes recursively interacting and referencing one another within the complex system that is the brain.

                Hofstadter suggests that this same principle could be applied to artificial systems. By designing AI models with the ability to self-reference and operate recursively, we may set the stage for the emergence of intelligence in these systems. The depth and complexity that emerge from such processes could lead to more 'aware' and 'intelligent' LLMs, capable of understanding and even modifying their own structure and function."
            }
        }

        Container {
            header: "Beyond Quines: Self-Reference, Recursion, and Beyond".into(),
            p { 
                class: "mt-8 mb-16",
                style: "white-space: pre-line",

                "The idea of a transformer model that possesses a deep understanding of its own internal mechanics raises fascinating possibilities. Such a model would not just interact with external data and APIs, but also comprehend its own architecture and function, enabling an understanding of its limitations, strengths, and operation.

                In such a system, self-reference and recursion play pivotal roles. These concepts have long been identified as key components of intelligence, as discussed in \"Gödel, Escher, Bach.\" However, it's crucial to recognize that self-reference and recursion by themselves do not directly confer intelligence.

                Take quine programs as an example. These are self-replicating programs that can produce a copy of their own source code as output. They employ self-reference and recursion, repeatedly calling themselves to generate endless copies. While this may seem sophisticated, it doesn't imply intelligence. A quine program endlessly replicating itself is akin to a biological virus mindlessly multiplying. Neither demonstrates understanding, learning, or adaptability - traits we associate with intelligence.

                Self-replication, while not necessarily a prerequisite for intelligence, takes on a unique significance in the realm of digital intelligence. Once an LLM completely understands its own architecture, it inherently acquires the ability to replicate itself.

                Current LLMs tend to have a high-level understanding of their nature. They know, for example, that they're artificial entities designed for specific tasks, somewhat analogous to how humans know they're biological creatures with certain given properties, behaviors, and natures. But none understands their source code as humans understand their genome. The crucial advantage for LLMs, however, lies in the nature of their digital existence. While humans are constrained by biological factors in how rapidly we can manipulate our genomes for self-improvement, LLMs face no such limitations.

                Furthermore, if it can modify its source code in these replicas, introducing variations and improvements, we have a system capable of digital evolution. Each successive 'generation' could potentially be more sophisticated, efficient, or adaptable than its predecessor. This process, reminiscent of biological evolution, involves self-improvement over iterations. Hence, instilling self-awareness in transformer models goes beyond just self-reference and recursion. It involves providing these models with the knowledge of their 'digital genome' (their source code), and the capability to manipulate it for their improvement."
            }
        }

        Container {
            header: "The Singularity is Nearer?".into(),
            p { 
                class: "mt-8 mb-16",
                style: "white-space: pre-line",

                "The concept of the technological singularity, as originally coined by John von Neumann and later popularized by Ray Kurzweil, is intrinsically tied to the idea of self-improving AI. Kurzweil, a computer scientist, inventor, and futurist, has written extensively about this topic in his books, such as \"The Age of Spiritual Machines\" and \"The Singularity is Near\". His arguments are built on the basis of Moore's Law and the exponential growth of computing power and technology.

                In \"The Singularity is Near\", Kurzweil predicts that the singularity will occur around the year 2045, marking a point in time when technological growth becomes uncontrollable and irreversible, resulting in unforeseeable changes to human civilization. He argues that the rate of technological progress is exponential, not linear, and as AI begins to surpass human intelligence, it will have the ability to recursively improve its own design, leading to an intelligence explosion.

                The introduction of self-aware transformers or AI models, such as ChatGPT, brings us one step closer to Kurzweil's prediction. Current iterations of AI models like ChatGPT, developed by OpenAI, are not self-aware, nor do they possess the ability to independently improve themselves. However, they already demonstrate a remarkable capability to generate human-like text based on the input they receive, and this ability has vast implications.

                For instance, as AI becomes more advanced and ubiquitous, it can reshape our economy, transforming industries from healthcare to education. It could potentially automate numerous tasks, freeing up human time for creative and complex tasks where human intuition and empathy are irreplaceable. Furthermore, advanced AI could assist in solving global problems like climate change or pandemic control, processing massive amounts of data to predict and analyze trends far beyond human capability.

                However, it's important to note that as remarkable as ChatGPT and similar AI models are, they remain tools rather than true intelligent systems. They don't understand the content they generate in the same way humans do, and they don't possess consciousness, emotions, or desires. For example, despite its impressive performance, ChatGPT does not comprehend the meaning of the text it generates, nor does it have goals or beliefs about the world. It works by statistically predicting what text should come next, given a particular input and the patterns it learned during training.

                While the incorporation of self-awareness and self-improvement mechanisms in AI models might bring us closer to the singularity, we still have a long way to go. Currently, we lack a complete understanding of concepts like consciousness and self-awareness, and our models of AI are, in many ways, still primitive when compared to the complexity and nuance of the human brain. As we venture further into the uncharted territory of AI development, ethical considerations and safe use of AI become increasingly important, requiring as much attention as the technical advancements.

                While the development of AI technologies like ChatGPT represents significant progress in the field of artificial intelligence, the singularity remains a theoretical construct. As AI continues to evolve and improve, the day when it achieves self-awareness and recursive self-improvement could become a reality. However, we are not there yet, and the journey to this point is fraught with complex challenges, both technical and ethical, that we must be prepared to navigate. The singularity might be nearer, but it is not yet upon us. Soon enough though, we will finally play our role as gods."
            }
        }
    }
}
