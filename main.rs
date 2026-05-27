use std::collections::HashMap;

enum PlayerColor {
    Red,
    Blue,
    Green,
    Orange
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
struct Game {
    board: std::collections::HashMap<(i32, i32), Hex>
}
impl Game {
    fn new() -> Self {
        Self {
            board: std::collections::HashMap::new()
        }
    }
}
fn main() {
    let mut g = Game::new();
    g.board.insert((0,0), Hex::new(HexType::Mountains(10)));
    g.board.insert((1,0), Hex::new(HexType::Pasture(2)));
    g.board.insert((2,0), Hex::new(HexType::Forest(9)));

    g.board.insert((-1,1), Hex::new(HexType::Fields(12)));
    g.board.insert((0,1), Hex::new(HexType::Hills(6)));
    g.board.insert((1,1), Hex::new(HexType::Pasture(4)));
    g.board.insert((2,1), Hex::new(HexType::Hills(10)));

    g.board.insert((-2,2), Hex::new(HexType::Fields(9)));
    g.board.insert((-1,2), Hex::new(HexType::Forest(11)));
    g.board.insert((0,2), Hex::new(HexType::Desert));
    g.board.insert((1,2), Hex::new(HexType::Forest(3)));
    g.board.insert((2,2), Hex::new(HexType::Mountains(8)));

    g.board.insert((-2,3), Hex::new(HexType::Forest(8)));
    g.board.insert((-1,3), Hex::new(HexType::Mountains(3)));
    g.board.insert((0,3), Hex::new(HexType::Fields(4)));
    g.board.insert((1,3), Hex::new(HexType::Pasture(5)));

    g.board.insert((-2,3), Hex::new(HexType::Hills(5)));
    g.board.insert((-1,3), Hex::new(HexType::Fields(6)));
    g.board.insert((0,3), Hex::new(HexType::Pasture(11)));
}
