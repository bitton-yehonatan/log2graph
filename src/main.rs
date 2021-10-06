use structopt::StructOpt;
use std::collections::HashMap;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
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
    /// The key for grouping logs
    #[structopt(short = "v", long = "keys_to_print")]
    keys_to_print: Vec<String>,
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
            None => println!("No key found")
        }
    }
    return res;
}

fn parse_log(content: String, pattern: String, key_del: String, data_del: String, group_by: &String, label_keys: &Vec<String>) -> HashMap<String, Graph::<String, i32>>{
    let mut log_map: HashMap<String, Graph::<String, i32>> = HashMap::new();
    let mut latest_nodes: HashMap<String, NodeIndex> = HashMap::new();
    for line in content.lines() {
        if line.contains(&pattern) {
            let splitted_line: Vec<&str> = line.split(&pattern).collect();
            let parsed_line = parse_line(splitted_line[1], &key_del, &data_del);
            let group_key_value = parsed_line.get(group_by).unwrap().to_string();
            let group_graph = log_map.get_mut(&group_key_value);
            let mut label = "".to_owned();
            for key in label_keys {
                label.push_str(format!("{}: {}", key, parsed_line.get(key).unwrap()).as_str());
                label.push_str("\n");
            }
            match group_graph {
                Some(events) => {
                    let node = events.add_node(label);
                    let last_node = latest_nodes.get(&group_key_value).unwrap();
                    events.add_edge(*last_node, node, 0);
                    latest_nodes.insert(group_key_value, node);
                },
                None => {
                    let mut events = Graph::<String, i32>::new();
                    let node = events.add_node(label);
                    log_map.insert(group_key_value.to_string(), events);
                    latest_nodes.insert(group_key_value, node);
                }
            }
        }
    }
    return log_map;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Cli::from_args();
    let content = std::fs::read_to_string(opts.path)?;
    let parsed_log = parse_log(content, opts.pattern, opts.keys_delimiter,  opts.params_delimiter, &opts.group_by, &opts.keys_to_print);
    for (group, group_graph) in parsed_log {
        println!("saving graph! {}", group);
        println!("{:?}", Dot::with_config(&group_graph, &[Config::EdgeNoLabel]));
        let mut graph_file = File::create(format!("{}.dot", group)).unwrap();
        let output = format!("{}", Dot::with_config(&group_graph, &[Config::EdgeNoLabel]));
        graph_file.write_all(&output.as_bytes()).expect("could not write file");
    }
    Ok(())
}