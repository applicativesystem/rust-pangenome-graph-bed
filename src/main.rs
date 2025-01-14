mod args;
mod graph;
use crate::args::GraphArgs;
use crate::graph::Analyze;
use crate::graph::Segment;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam and SLB Potsdam
*Date 2025-1-14

 from pangenome graph to bed analyze for analysis into the panache. This also
 outputs a bed file for the analysis of the pangenomes and then you
 can use that also a import file into other programs.

*/

fn main() {
    let graphargs = GraphArgs::parse();
    let output = graph_bed_segment(&graphargs.graph).unwrap();
    for i in output.iter() {
        println!(
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            i.tag, i.start, i.end, i.oritag, i.rank
        );
    }
}
fn graph_bed_segment(path: &str) -> Result<Vec<Analyze>, Box<dyn Error>> {
    let graphopen = File::open(path).expect("file not found");
    let graphread = BufReader::new(graphopen);
    let mut segmentopen: Vec<Segment> = Vec::new();

    for i in graphread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("S"){
        let linevec = line
            .split("\t")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        segmentopen.push(Segment {
            segment: linevec[0].to_string(),
            id: linevec[1].to_string(),
            seq: linevec[2].to_string(),
            connection: linevec[3].to_string(),
            tag: linevec[5].to_string(),
        });
      }
    }

    let mut segmentanalyzer: Vec<Analyze> = Vec::new();
    for i in segmentopen.iter() {
        let _idtag: String =
            i.id.split(|c: char| c.is_numeric())
                .collect::<Vec<_>>()
                .join("");
        let idnumber: usize =
            i.id.split(|c: char| !c.is_numeric())
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
                .join("")
                .parse::<usize>()
                .unwrap();
        let idalignmenttag: String = i.connection.split(":").collect::<Vec<_>>()[2].to_string();
        let rankdefine: usize = i.tag.split(":").collect::<Vec<_>>()[2]
            .parse::<usize>()
            .unwrap();
        segmentanalyzer.push(Analyze {
            tag: idalignmenttag,
            start: idnumber,
            end: idnumber+i.seq.len(),
            oritag: i.id.clone(),
            rank: rankdefine,
        });
    }

    let mut samfile = File::create("gfa-bed.txt").expect("file not found");

    for i in segmentanalyzer.iter() {
        writeln!(
            samfile,
            "{}\t{}\t{}\t{}\t{}",
            i.tag, i.start, i.end, i.oritag, i.rank
        )
        .expect("line not present");
    }

    Ok(segmentanalyzer)
}
