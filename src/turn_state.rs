#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    GameOver,
    MonsterTurn,
    NextLevel,
    PlayerTurn,
    Victory,
}
