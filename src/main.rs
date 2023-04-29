use crossterm::{cursor, terminal, ExecutableCommand};
use dialoguer::Input;
use std::io::{stdout, Write};

fn main() {
    let mut term = stdout();

    term.execute(cursor::Hide).unwrap();
    term.execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    term.write_fmt(format_args!("Initiative Tracker\n"))
        .unwrap();
    term.write_fmt(format_args!("Enter the name of the actor and their initiative, separated by a space. Leave blank to finish.\n")).unwrap();

    let mut actors: Vec<(String, i32)> = Vec::new();
    let mut finished: bool = false;

    while !finished {
        let input: String = Input::new()
            .with_prompt("<name> <initiative>")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if input == "" {
            finished = true;
        } else {
            let split: Vec<&str> = input.split(" ").collect();

            if split.len() != 2 {
                term.write_fmt(format_args!("Invalid number of arguments.\n"))
                    .unwrap();
                continue;
            }
            if split[0] == "" {
                term.write_fmt(format_args!("Name cannot be empty.\n"))
                    .unwrap();
                continue;
            }
            if split[1].parse::<i32>().is_err() {
                term.write_fmt(format_args!("Initiative must be a number.\n"))
                    .unwrap();
                continue;
            }

            let name: String = split[0].to_string();
            let initiative: i32 = split[1].parse().unwrap();

            actors.push((name, initiative));
        }
    }
    term.write(b"\n").unwrap();

    actors.sort_by(|a, b| b.1.cmp(&a.1));
    let mut index: usize = 0;

    term.write_fmt(format_args!(
        "| {:<20} | {:<10} |\n",
        "--------------------", "----------"
    ))
    .unwrap();
    term.write_fmt(format_args!("| {:<20} | {:<10} |\n", "Name", "Initiative"))
        .unwrap();
    term.write_fmt(format_args!(
        "| {:<20} | {:<10} |\n",
        "--------------------", "----------"
    ))
    .unwrap();
    for (i, actor) in actors.iter().enumerate() {
        if i == 0 {
            term.write_fmt(format_args!("| {:<20} | {:<10} | <---\n", actor.0, actor.1))
                .unwrap();
        } else {
            term.write_fmt(format_args!("| {:<20} | {:<10} |\n", actor.0, actor.1))
                .unwrap();
        }
    }
    term.write_fmt(format_args!(
        "| {:<20} | {:<10} |\n",
        "--------------------", "----------"
    ))
    .unwrap();

    loop {
        let input: String = Input::new()
            .with_prompt("Press enter to continue or enter 'quit' to exit.")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if input == "quit" {
            break;
        }

        if input == "" {
            index += 1;
            if index >= actors.len() {
                index = 0;
            }
            term.execute(cursor::MoveUp(actors.len() as u16 + 4))
                .unwrap();
            term.execute(cursor::MoveToColumn(0)).unwrap();
            term.execute(terminal::Clear(terminal::ClearType::FromCursorDown))
                .unwrap();

            term.write_fmt(format_args!("| {:<20} | {:<10} |\n", "Name", "Initiative"))
                .unwrap();
            term.write_fmt(format_args!(
                "| {:<20} | {:<10} |\n",
                "--------------------", "----------"
            ))
            .unwrap();
            for (i, actor) in actors.iter().enumerate() {
                if i == index {
                    term.write_fmt(format_args!("| {:<20} | {:<10} | <---\n", actor.0, actor.1))
                        .unwrap();
                } else {
                    term.write_fmt(format_args!("| {:<20} | {:<10} |\n", actor.0, actor.1))
                        .unwrap();
                }
            }
            term.write_fmt(format_args!(
                "| {:<20} | {:<10} |\n",
                "--------------------", "----------"
            ))
            .unwrap();
        }
    }
}
