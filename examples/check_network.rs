use quork::network::get_connection;

fn main() {
    let con = get_connection();

    for c in con {
        println!("{:?}", c);
    }
}
