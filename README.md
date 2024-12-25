Chess piece assets:

https://wildlifestudios.itch.io/chess-set-pixel-art

https://peterellisjones.com/posts/generating-legal-chess-moves-efficiently/

# Major TODOs 

## King related

* Find a way to get valid moves that don't leave the king in check
* Implement check & checkmate
* Implement castle

To implement more complex king mvoes it is required to generate a threat map to show which pieces are attacked by the opposing colour. 

## Pawn related

* ~~Implement pawn promotion to queen~~
* Implement en passant
* ~~Implement the correct attack pattern for pawns~~


# The game will not be finished since the complexity increases greatly ...

Calculating the legal king moves in the late game is incredibly difficult. The game will be handled like a board game and the player simply has to know when they won. 