use structopt::StructOpt;
use std::collections::HashMap;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The delimiter for each key
    #[structopt(short = "k", long = "keys_delimiter", default_value=":")]
    keys_delimiter: String,
    /// The delimiter for each param
    #[structopt(short = "d", long = "params_delimiter", default_value=",")]
    params_delimiter: String,
    /// The pattern for parsed lines
    #[structopt(short = "p", long = "pattern")]
    pattern: String,
    /// The key for grouping logs
    #[structopt(short = "g", long = "group_by")]
    group_by: String,
    /// Keys for node labling
    #[structopt(short = "v", long = "keys_to_print")]
    keys_to_print: Vec<String>,
    /// split dot file by group
    #[structopt(short = "s", long = "split_files")]
    split_files: bool,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn parse_line(line: &str, key_del: &str, data_del: &str) -> HashMap<String, String>{
    let mut res: HashMap<String, String> = HashMap::new();
    let splitted_line = line.split(data_del);
    for data in splitted_line {
        let splitted_data = data.split_once(key_del);
        match splitted_data {
            Some((key, value)) => {
                res.insert(key.trim().to_string(), value.trim().to_string());
            },
            None => println!("No key found: {}", data)
        }
    }
    return res;
}

fn log_to_map(content: String, pattern: String, key_del: String, data_del: String, group_by: &String) -> HashMap<String, Vec<HashMap<String, String>>>{
    let mut log_map:HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for line in content.lines() {
        if line.contains(&pattern) {
            let splitted_line: Vec<&str> = line.split(&pattern).collect();
            let parsed_line = parse_line(splitted_line[1], &key_del, &data_del);
            match parsed_line.get(group_by) {
                Some(group_key_value) => {
                    match log_map.get_mut(group_key_value) {
                        Some(events) => {
                            events.push(parsed_line);
                        },
                        None => {
                            let mut events = Vec::new();
                            events.push(parsed_line.to_owned());
                            log_map.insert(group_key_value.to_string(), events);
                        }
                    }
                },
                None => println!("no group key {}", group_by)
            }
        }
    }
    return log_map;
}

fn map_to_graph(log_map: HashMap<String, Vec<HashMap<String, String>>>, keys_to_print: &Vec<String>, split_files: bool) -> HashMap<String, Graph<String, i32>> {
    let mut log_graphs = HashMap::new();
    let mut log_graph = Graph::new();
    for (group_key, group_events) in log_map {
        let mut previous_event = log_graph.add_node(format!("start, {}", group_key));
        for event in group_events {
            let mut label = "".to_owned();
            for key in keys_to_print {
                let parsed_key = event.get(key);
                match  parsed_key {
                    Some(value) => {
                        label.push_str(format!("{}: {}", key, value).as_str());
                        label.push_str("\n");
                    },
                    None => continue
                }
            }
            let current_event = log_graph.add_node(label);
            log_graph.add_edge(previous_event, current_event, 0);
            previous_event = current_event;
        }
        if split_files {
            log_graphs.insert(group_key, log_graph);
            log_graph = Graph::new();
        }
    }
    if !split_files {
        log_graphs.insert("graph_result".to_owned(), log_graph);
    }
    return log_graphs;
}

fn parse_log(content: String, pattern: String, key_del: String, data_del: String, group_by: &String, label_keys: &Vec<String>, split_files: bool) {
    let log_map = log_to_map(
        content,
        pattern,
        key_del,
        data_del,
        group_by,
    );

    let log_graphs = map_to_graph(log_map, label_keys, split_files);
    let output_dir = std::path::Path::new("./graph_results");
    std::fs::create_dir_all(output_dir);
    for (group_key, log_graph) in log_graphs {
        println!("saving graph!");
        println!("{:?}", Dot::with_config(&log_graph, &[Config::EdgeNoLabel]));
        let mut graph_file = File::create(
            output_dir.join(format!("{}.dot", group_key))
        ).unwrap();
        let output = format!("{}", Dot::with_config(&log_graph, &[Config::EdgeNoLabel]));
        graph_file.write_all(&output.as_bytes()).expect("could not write file");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Cli::from_args();
    let content = std::fs::read_to_string(opts.path)?;
    parse_log(content, opts.pattern, opts.keys_delimiter,  opts.params_delimiter, &opts.group_by, &opts.keys_to_print, opts.split_files);
    Ok(())
}