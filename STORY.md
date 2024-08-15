# Story and Walkthrough

I wanted to do embedded since I had begun to learn Rust. It seems that it was one of the multiple great applications it has.
It all started the week after holidays, where I had played a little with a Raspberry Pi embedded learning repo (link the repo). I shared my idea to a group of friends. The only idea I had at the moment that seemed realistic to me was a little car that I could control myself. I started to do some research about how to do that. In the evening, I watched a lot of content (like 5-6 videos) on electricity, DC motors, resistors, etc. That’s how I got hyped about a non-existing car in my own bedroom.

Next Friday, a friend and I went to an electronics store in Paris called LetMeKnow that I already knew about because of 3D stuff. A guy on the phone told me he could help me there with my project. I finally bought basic material to begin my embedded journey, jumper wires, a breadboard, a battery and his charger, a micro-servo, a motor driver and a base car kit which wasn’t in my plans originally because I planned to print the body of the car myself, but I thought would be a great idea for prototyping. The base car kit had two DC motors (with gears to reduce speed but increase couple?) and a caddy wheel. At that point, I got everything I needed, but the microcontroller, which was kind of the central point of this project. I found another shop on my way home that could provide me the Holy Blue Pill (STM32F103C8T6) and the associated ST-Link V2 Programmer to flash the controller. I was actually one euro short to pay because his card machine was broken, but the vendor was nice and let me go with the Holy Grail.
The same day, I set up the repository for the project and began my experimentation. I found the different data sheets about my components. I didn’t want to do all the embedded implementation details myself for now, so I used Embassy-rs, which seemed enough mature to be used. The project was pretty simple to begin. Btw, I would love to contribute to embassy’s documentation, which is quite empty.

In the following days, I did some soldering and added a l298n motor driver implementation.
I found Fritzing, open-source software to build circuits, it took me 1h30 to get the components to build the app, but that’s always cheaper than an $8 bundle download on their website.

Here, there is an exam revision pause for half a week.
The week-end I bought more things, I wanted an ultrasonic sensor for evaluating distances and eventually react to close walls. A target Bluetooth module, which can only be paired to. Along with 2 m of red and black wire, (1 m each), some 9V to power jack and power jack to cables.

I did some experiments with the ultrasonic sensor, and I did not have good results for now.
I later bought a pack of resistors to play with the Bluetooth sensor that needs a power divider system.

I tried to ping the module, but I didn’t get any satisfying response, only weird and correlated bytes.

I reworked the repository to welcome a new ‘car-controller’ crate. It will contain the software (GUI and CLI to interact with the car). A video found on Reddit by my friend show a car being controlled by an X-Box controller. I actually have one, great idea that is now mine. 😉 (GilRS library)

Found a great guy on the internet that has many of the modules I have, and A BLUE PILL (From now on I love this person). He did some troubleshoot videos about the Bluetooth module, wiring the TX pin to the Rx one. If everything you send to the module comes back, the modules work fine.

---

Oh man, it's been a year.
