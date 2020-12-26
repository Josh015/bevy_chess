# Chess in Bevy Ex

This is just me tinkering with some modifications to [Guim Caballero's](https://github.com/guimcaballero) code from his [Rust Bevy Chess tutorial](https://caballerocoll.com/blog/bevy-chess-tutorial/).

- [x] Centralize PieceColor display code.
- [x] Generalize piece spawn code to make it data-driven.
- [x] Generalize code to allow setting up both sets with same function.
- [ ] Prevent picking while piece(s) are moving.
- [ ] Rotate black knights & bishops to face correct direction.
- [ ] Highlight valid destination squares for selected piece.
- [ ] Move taken pieces to their corresponding positions beside the board. May require pre-storing their final destination.
- [ ] Make knights move along right angle path rather than straight to destination.
- [ ] Turn movement logic into components rather than cases?
- [ ] Allow promotion of pawns.
- [ ] Implement castling.
- [ ] Centralize end-game logic and account for check-mate.
