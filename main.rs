#[derive(PartialEq)]
#[derive(Debug)]
enum PlayerColor {
    Red,
    Blue,
    White,
    Orange
}
impl PlayerColor {
    fn next(&self) -> Self {
        match self {
            PlayerColor::Red => PlayerColor::Blue,
            PlayerColor::Blue => PlayerColor::White,
            PlayerColor::White => PlayerColor::Orange,
            PlayerColor::Orange => PlayerColor::Red
        }
    }
    fn prev(&self) -> Self {
        return self.next().next().next();
    }
    fn first() -> Self {
        PlayerColor::Red
    }
    fn last() -> Self {
        PlayerColor::Orange
    }
}
enum Building {
    Settlement,
    City
}
enum HexType {
    Fields(i32),
    Mountains(i32),
    Pasture(i32),
    Forest(i32),
    Hills(i32),
    Sea(i32),
    Desert
}
struct Edge {
    color: Option<PlayerColor>,
}
struct Vertex {
    color: Option<PlayerColor>,
    building: Option<Building>
}
struct Hex {
    north_vertex: Vertex,
    north_east_vertex: Vertex,
    north_east_edge: Edge,
    east_edge: Edge,
    hex_type: HexType,
}
impl Hex {
    fn new(hex_type: HexType) -> Self {
        Self {
            north_vertex: Vertex{color: None, building: None},
            north_east_vertex: Vertex{color: None, building: None},
            north_east_edge: Edge{color: None},
            east_edge: Edge{color: None},
            hex_type: hex_type,
        }
    }
}
#[derive(Debug)]
enum GamePhase {
    ForwardSetup,
    ReverseSetup,
    NormalPlay,
}
struct Game {
    board: std::collections::HashMap<(i32, i32), Hex>,
    current_player: PlayerColor,
    phase: GamePhase
}
impl Game {
    fn new() -> Self {
        let mut b = std::collections::HashMap::new();
        b.insert((0,0), Hex::new(HexType::Mountains(10)));
        b.insert((1,0), Hex::new(HexType::Pasture(2)));
        b.insert((2,0), Hex::new(HexType::Forest(9)));

        b.insert((-1,1), Hex::new(HexType::Fields(12)));
        b.insert((0,1), Hex::new(HexType::Hills(6)));
        b.insert((1,1), Hex::new(HexType::Pasture(4)));
        b.insert((2,1), Hex::new(HexType::Hills(10)));

        b.insert((-2,2), Hex::new(HexType::Fields(9)));
        b.insert((-1,2), Hex::new(HexType::Forest(11)));
        b.insert((0,2), Hex::new(HexType::Desert));
        b.insert((1,2), Hex::new(HexType::Forest(3)));
        b.insert((2,2), Hex::new(HexType::Mountains(8)));

        b.insert((-2,3), Hex::new(HexType::Forest(8)));
        b.insert((-1,3), Hex::new(HexType::Mountains(3)));
        b.insert((0,3), Hex::new(HexType::Fields(4)));
        b.insert((1,3), Hex::new(HexType::Pasture(5)));

        b.insert((-2,3), Hex::new(HexType::Hills(5)));
        b.insert((-1,3), Hex::new(HexType::Fields(6)));
        b.insert((0,3), Hex::new(HexType::Pasture(11)));

        Self {
            board: b,
            current_player: PlayerColor::Red,
            phase: GamePhase::ForwardSetup
        }
    }
    fn end_turn(&mut self) {
        match self.phase {
            ::GamePhase::ForwardSetup => if self.current_player == PlayerColor::last() {
                self.phase = ::GamePhase::ReverseSetup;
            } else {
                self.current_player = self.current_player.next();
            }
            ::GamePhase::ReverseSetup => if self.current_player == PlayerColor::first() {
                self.phase = ::GamePhase::NormalPlay;
            } else {
                self.current_player = self.current_player.prev();
            }
            ::GamePhase::NormalPlay => {self.current_player = self.current_player.next();}  
        }
    }
}
fn main() {
    let mut g = Game::new();
    for n in 0..16 {
        println!("{:?} {:?}", g.current_player, g.phase);
        g.end_turn();
    }
}
