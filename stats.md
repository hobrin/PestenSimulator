Definitions:
Random -> looks at what cards he can play and picks a random one
bully -> throws the 2 and the joker as fast as possible
peacekeeper -> retains the bully cards for as long as possible
carefulbully -> becomes bully when opponent has < 3 cards or is bullying him, otherwise peacekeeper

No starting bias unless otherwise mentioned.

All games are played 1,000,000 times and the first number corrosponds to how many games p1 won.
The increased winrate can be calculated by doing that gamesWon/1,000,000*2-1


Random vs Random with starting bias
[515885, 484115]
Conclusion: starting increases your win rate by 3%

Random vs Random
[500066, 499934]
Conclusion: program does not seem to be flawed.

Bully vs Random
[530467, 469536]

Peacekeeper vs Random
[493553, 506450]

Peacekeeper vs Bully
[465908, 534095]

Carefulbully(returntosender) vs Random
[559695, 440308]
Conclusion: Return to sender performs better.

Carefulbully(returntosender + < 3 cards) vs Random
[561997, 438006]

Carefulbully(returntosender + < 3 cards) vs Peacekeeper
[554907, 445096]

Carefulbully(returntosender + < 3 cards) vs Carefulbully(returntosender + < 3 cards) with starting bias
[517475, 482528]
[517735, 482268]
Conclusion: 3% higher winrate if you start.

Carefulbully(returntosender + < 3 cards) vs Carefulbully(returntosender + < 4 cards)
[504877, 495126]
Conclusion: it is slightly better to throw at less than 2 cards than less than 4 cards.

Carefulbully(returntosender + < 2 cards) vs Carefulbully(returntosender + < 3 cards)
[488622, 511381]
[489361, 510639]

Keep the jack for as long as possible.
players.push(Box::new(CarefulBullyPlayer::new(3, false)));
players.push(Box::new(CarefulBullyPlayer::new(3, true)));
[471151, 528849]
winrate p1: -0.05769801
Conclusion: 5.7% better to keep the jack for as long as possible.

players.push(Box::new(CarefulBullyPlayer::new(3, false, false)));
players.push(Box::new(CarefulBullyPlayer::new(3, true, true)));
[476490, 523510]
winrate p1: -0.047020018
Conclusion: it is apparantly better to avoid desperado's in these low-iq environments.


players.push(Box::new(RandomPlayer::new()));
players.push(Box::new(CarefulBullyPlayer::new(3, true, false)));
[409164, 590836]
winrate p1: -0.18167198
Conclusion: strategy really does matter.

Doesn't check ahead yet.
players.push(Box::new(CarefulBullyPlayer::new(3, true, false, true)));
players.push(Box::new(CarefulBullyPlayer::new(3, true, false, false)));
[507791, 492209]
winrate p1: 0.015581965

Also checks if follow up is present
players.push(Box::new(CarefulBullyPlayer::new(3, true, false, true)));
players.push(Box::new(CarefulBullyPlayer::new(3, true, false, false)));
[508911, 491089]
winrate p1: 0.017822027
Conclusion: performs _slightly_ better.

Retains 7 for as long as possible.
players.push(Box::new(CarefulBullyPlayer::new(3, true, false, false, true)));
players.push(Box::new(CarefulBullyPlayer::new(3, true, false, true, false)));
[476906, 523094]
winrate p1: -0.046187997
Conclusion: terrible strategy.

players.push(Box::new(RandomPlayer::new()));
players.push(Box::new(CarefulBullyPlayer::new(3, true, false, true, false)));
[401934, 598066]
winrate p1: -0.196132

