#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TurnState {
    AwaitingInput,
    GameOver,
    MonsterTurn,
    NextLevel,
    PlayerTurn,
    Victory,
}
