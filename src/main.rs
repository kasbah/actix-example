use actix::{Actor, Addr, Handler, Message};

#[derive(Message)]
#[rtype(usize)]
struct MoveRequest;


// -------------------
struct HumanPlayer {}

impl Actor for HumanPlayer {
    type Context = actix::Context<Self>;
}

impl Handler<MoveRequest> for HumanPlayer {
    type Result = usize;

    fn handle(&mut self, _msg: MoveRequest, _: &mut Self::Context) -> Self::Result {
        let mut input = String::new();
        println!("Enter a number:");
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }
}

// -------------------
struct AiPlayer {}

impl Actor for AiPlayer {
    type Context = actix::Context<Self>;
}

impl Handler<MoveRequest> for AiPlayer {
    type Result = usize;

    fn handle(&mut self, _msg: MoveRequest, _: &mut Self::Context) -> Self::Result {
        // chosen by fair dice roll.
        4
    }
}

async fn run_game(player1: Addr<impl Handler<MoveRequest>>, player2: Addr<impl Handler<MoveRequest>>) -> bool {
    let res1 = player1.send(MoveRequest {}).await.unwrap();
    let res2 = player2.send(MoveRequest {}).await.unwrap();
    res1 > res2
}

#[actix::main]
async fn main() {
    let player1 = HumanPlayer {}.start();
    let player2 = AiPlayer {}.start();
    let game_res1 = run_game(player1, player2).await;
    println!("Player 1 wins: {}", game_res1);

    let player3 = AiPlayer {}.start();
    let player4 = AiPlayer {}.start();
    let game_res2 = run_game(player3, player4).await;
    println!("Player 3 wins: {}", game_res2);
}
