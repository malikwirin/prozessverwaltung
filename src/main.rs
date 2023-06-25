use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

mod process;
mod scheduler;
mod output;

use scheduler::set_scheduling_algorithm;
use output::output;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Lesen der Kommandozeilenargumente
    let args: Vec<String> = env::args().collect();

    // Überprüfen, ob genügend Argumente übergeben wurden
    if args.len() < 3 {
        panic!("Bitte geben Sie den Pfad zur CSV-Datei, den Namen des Scheduling-Verfahrens an, und die Menge an Zeiteinheiten falls Round Robin.");
    }
    
    let mut switch_time: u32 = 0;
    let mut time_units: u32 = 3;

    // Extrahieren der Argumente
    let csv_path = &args[1];
    let scheduling_algorithm = &args[2];
    let scheduling_algorithm = set_scheduling_algorithm(&scheduling_algorithm);
    if scheduling_algorithm == "Round Robin (RR)" {
        if args.len() >= 4 { 
            time_units = args[3].parse::<u32>()?;
        }
        if args.len() >= 5 {
            switch_time = args[4].parse::<u32>()?;
        } 
    } else {
        if args.len() >= 4 {
            switch_time = args[3].parse::<u32>()?;
        } else {
            switch_time = 0;
        }
    }

    // Öffnen der CSV-Datei
    let file = File::open(Path::new(csv_path))?;

    // Lesen der Prozesse aus der CSV-Datei und Erstellen der Prozessobjekte

    let mut processes: Vec<process::Process> = Vec::new();
    let mut used_ids: HashSet<u32> = HashSet::new(); // HashSet für verwendete IDs

    let reader = io::BufReader::new(file);
    for line in reader.lines().skip(1) {
        if let Ok(line) = line {
            println!("CSV-Zeile: {}", line);
            let values: Vec<&str> = line.split(',').collect();
            println!("CSV-Werte: {:?}", values);
            if values.len() == 4 {
                let name = values[0].to_string();
                let id = values[1].parse::<u32>()?;
                // Überprüfen, ob die ID bereits verwendet wurde
                if used_ids.contains(&id) {
                    println!("Prozess mit ID {} existiert bereits. Überspringen.", id);
                    continue; // Zum nächsten Schleifendurchlauf springen
                }

                used_ids.insert(id); // ID als verwendet markieren
                let arrival_time = values[2].parse::<u32>()?;
                let execution_time = values[3].parse::<u32>()?;
                let start = 0;
                let end = 0;
                let waiting_time = 0;
                let process_time = 0;
                let reaction_time = 0;
                let execution_time_left = execution_time;
                let started = false;
                let completed = false;

                let process = process::Process {
                    name,
                    id,
                    arrival_time,
                    execution_time,
                    start,
                    end,
                    waiting_time,
                    process_time,
                    reaction_time,
                    execution_time_left,
                    started,
                    completed,
                };
                processes.push(process);
            } else {
                println!("Ungültige Anzahl von CSV-Werten: {}", values.len());
                println!("Zeile {} wird übersprungen.", line);
            }
        } 
    }

    // Debug-Ausgabe der Prozesse
    for process in &processes {
        println!(
            "Prozess: {} (ID: {}), Ankunftszeit: {}, Bearbeitungszeit: {}",
            process.name, process.id, process.arrival_time, process.execution_time
        );
    }

    //setze die Anzahl an Prozessen und ihr gesamter Zustand
    let amount: u64 = processes.len() as u64;
    let mut all_processes_completed: bool = processes.iter().all(|process| process.completed);


    match scheduling_algorithm.as_str() {
        "First-In, First-Out (FIFO)" => {
            sort_processes_by_arrival_time(&mut processes);
            let mut time: u32 = 0;
            let mut current: usize = 0;
            let mut last_process: usize = current;
            println!("FIFO-Scheduling");
            while !all_processes_completed {
                if current >= amount as usize {
                    current = 0;
                    //an dieser Stelle fehlt noch eine Logik um die switch_time zu beachten.
                }
                let current_process = &mut processes[current];

                if current_process.process(&time) {
                    time += 1;
                    last_process = current;

                    if current_process.completed {
                        time += switch_time;
                    }

                } else if processes_currently_available(&processes.clone(), &time) {
                    if last_process == current {
                        time += switch_time;
                    }
                    current += 1;
                } else {
                    time += 1;
                }

                //es wird geprüft ob alle Prozesse durch sind
                all_processes_completed = processes.iter().all(|process| process.completed);
            }
        }

        "Round Robin (RR)" => {
            sort_processes_by_arrival_time(&mut processes);
            let mut time: u32 = 0;
            let mut current: usize = 0;
            let mut last_process: usize = current;
            let mut process_time: u32 = 0;
            println!("Round-Robin-Scheduling");
            while !all_processes_completed {
                if process_time < time_units {
                    if current > amount as usize - 1 {
                        current = 0;
                    }
                    

                    if processes[current].arrival_time <= time && !processes[current].completed {
                        if !(last_process == current) {
                            time += switch_time;
                        }

                        if processes[current].process(&time) {
                            time += 1;
                            process_time += 1;
                            last_process = current;

                            if processes[current].completed {
                                current += 1;
                                process_time = 0;
                            }
                        } else {
                            current += 1;
                            process_time = 0;
                        }
                    } else {
                        if !processes_currently_available(&processes, &time) {
                            time += 1;
                        }
                        current += 1;
                        process_time = 0;
                    }  
                } else {
                    process_time = 0;
                    current += 1;
                }

                //es wird geprüft ob alle Prozesse durch sind
                all_processes_completed = processes.iter().all(|process| process.completed);
            }
        }

        "Non-preemptive Shortest Job First" => {
            processes = sort_processes_by_execution_time(processes);
            let mut time: u32 = 0;
            let mut current: usize = 0;
            println!("Non-preemptive Shortest Job First");
            while !all_processes_completed {
                if current >= amount as usize {
                    current = 0;
                }

                if processes[current].process(&time) {
                    time += 1;

                    if processes[current].completed {
                        time += switch_time;
                        current = 0;
                    }

                } else if processes_currently_available(&processes, &time) {
                    current += 1;
                } else {
                    time += 1;
                    current = 0;
                }

                //es wird geprüft ob alle Prozesse durch sind 
                all_processes_completed = processes.iter().all(|process| process.completed);
            }
        }
        
        "Preemptive Shortest Job First" => {
            processes = sort_processes_by_execution_time(processes);
            let mut time: u32 = 0;
            let mut current: usize = 0;
            let mut last_process: usize = amount as usize;
            println!("Preemptive Shortest Job First");
            while !all_processes_completed {
                processes = sort_processes_by_execution_time_left(processes);

                if current > amount as usize - 1 {
                    current = 0;
                }

                if processes[current].process(&time) {
                    time += 1;
                    last_process = current;
                    
                    current = 0;
                } else if processes_currently_available(&processes, &time) {
                    if last_process == current {
                        time += switch_time;
                    }
                    current += 1;
                } else if !processes_currently_available(&processes, &time) {
                    time += 1;
                }
                
                //es wird geprüft ob alle Prozesse durch sind
                all_processes_completed = processes.iter().all(|process| process.completed);
            }
        }

        _ => {
            println!("Unbekanntes Scheduling-Verfahren: {}", scheduling_algorithm);
            println!("Mögliche Argumente sind in der Dokumentation nachzulesen.");
            return Err("Unbekanntes Scheduling-Verfahren".into());
        }
    }
    
    output(&processes, &scheduling_algorithm, &switch_time, &time_units)?;

    Ok(())
}


fn processes_currently_available(processes: &Vec<process::Process>, time: &u32) -> bool {
    for process in processes {
        if !process.completed {
            if *time >= process.arrival_time {
                return true;
            }
        }
    }
    false
}

fn sort_processes_by_execution_time(mut processes: Vec<process::Process>) -> Vec<process::Process>{
    processes.sort_by(|a, b| a.execution_time.cmp(&b.execution_time));
    processes
}

fn sort_processes_by_execution_time_left(mut processes: Vec<process::Process>) -> Vec<process::Process>{
    processes.sort_by(|a, b| a.execution_time_left.cmp(&b.execution_time_left));
    processes
}

fn sort_processes_by_arrival_time(processes: &mut Vec<process::Process>) {
    processes.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
}
