use solar_rust::parser::parse_log;
use solar_rust::messages_motorcontroller;

#[test]
#[cfg(feature = "debug")]
fn golden_decode_motorcontroller() {
    let frames = parse_log("tests/golden/motorcontroller_sample.txt");
    assert!(!frames.is_empty());

    for parsed in frames {
        let decoded = messages_motorcontroller::message.from_can_message(frame.id(), frame.data());
        let result = decoded.unwrap();
        println!("{:?}", result);
    }
}
