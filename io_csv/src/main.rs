use csv::{Writer,Reader};

#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: usize,
}

fn main() {
    // Mock data
    let dollar_films = vec![
        ["A Fistful of Dollars", "Rojo", "1964"],
        ["For a Few Dollars More", "El Indio", "1965"],
        ["The Good, the Bad and the Ugly", "Tuco", "1966"],
    ];

    // File path 
    let path = "movies.csv";

    println!("--- Create file");
    // Initialize writer, write records and flush
    let mut writer = Writer::from_path(path).unwrap();
    for row in dollar_films {
        writer.write_record(&row).expect("Writer error!");
    }

    writer.flush().expect("Flush error");

    println!("--- Read file");

    let mut reader = Reader::from_path(path).unwrap();
    for row in reader.records() {
        let record = row;
        println!("{:?}", record);
    }
}
