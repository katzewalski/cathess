# Cathess

A chess inspired game about pushing cats at eachother ( No cats were harmed in the process )

### Important note

I made this to learn rust with OpenGL, expect bugs!

## Game play

### To start
Starting the game, you will get 5 cats and 2 effects. Totaling 7 cards.

This game is played on a 5x5 board. The rows closest to the player is called the 'Home row'. This is where you can place cats at the start or any point in the game. If any of your enemy cats enters your home row, you lose and vice versa.

The board looks something like this, I don't have any pictures of it yet.

```
#####
.....
.....
.....
#####
```

#### Taking turns
This is a turn based game. In one turn you can choose to these three actions.

- Push your cat

- Use power

- Draw card

#### Cats
Cats are like chess peices, You can push them around in any vertical or horizontal direction.

Cats can be grouped together in stacks as if it is one cat, combinding their power. But since in one game many of the same cat can exist, to balance it the same cat cannot be grouped with eachother, keep that in mind!

Cats also have 'Sides'. Cats with the same side cannot be grouped with eachother.

Cats can 'push' to do this move your cat or your cat stack to occupy another cat or stack of cats tile that has less power that you. The cat that you pushed must now move to any legal tile. If it cannot move to any legal tile. That cat is discarded.

#### Power
Power is well, Special cards. There are four types.

Water, this turns a tile into water. You cannot occupy water tiles. Cannot be used on home row tiles.

Catnip, this makes a cat become a brick for 3 turns. No one can now move that cat nor can they push that cat.

Soil, this is used to turn a water tile into mud. Mud makes it so that after moving a cat, you have to wait to move it for one turn.

Map, this lets your cat move in a vertical direction. Can only be used once though, use it wisely.

#### Drawing a card
Click on the card pile. Congratulations you drew a card!
