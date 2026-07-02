use rustyline::DefaultEditor;

fn  process_line(line: String) {
    println!("{line}");
}

fn  main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let line = rl.readline(" > ").unwrap();
        if line == "exit" {
            break
        }
        process_line(line);
    }
}
