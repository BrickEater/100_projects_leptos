Jan 14, 2024:
I finally had some success with inputs and type checking. I added a match construct into the set_celsius function call. Next, I need to ensure I can repeatedly input values, it currently permits a single input and has an output but restricts the ability to change. I think adding a basic loop will be enough. Then, I can do the calculations on the correct input.
Jan 12, 2024:
I am getting closer, I think, but this is still pretty confusing. I need to continue reading the book, though, to be sure I'm not missing references that may help me.
The issue still lies with the syntax but I think I understand the basic structure or logic of it. Components are still just functions so I write them with a function signature taking in a parameter just like any other function. The parameter, in this case, is a generic variable which is mutated and returned. That is where I need to work on understanding a bit more, though. Are all setters mutated to then be transferred to the getters to display? I should have a conversation with ChatGPT about this.
I should refer back to these pages next time I look at this:
https://docs.rs/leptos/latest/leptos/fn.create_signal.html
https://docs.rs/leptos/latest/leptos/struct.ReadSignal.html
https://docs.rs/leptos/latest/leptos/struct.WriteSignal.html
They are about the create_signal() method and how it works. I think there is some good insight here that I'm not acknowledging. Specifically... how it works... lol
Jan 11, 2024:
This is driving me nuts but I think I got it. I was intimidated by the new syntax but this is just passing information between multiple functions while considering scope. That is basic shit that I know how to do. Next time I approach this I'm going to break this down into basic Rust and then add the Leptos syntax. It will be a global tuple of two functions, one a getter, the other a setter, a component that calls the setter, and a component that calls the getter.
Jan 8, 2024:
I'm struggling to have multiple components share a signal. The goal is to have the radio component to change a global signal so the converter component can read it an make the correct conversion calculation.

I added some code I found to the bottom and commented it out, I think I can use it as an example of what shared signals look like.
