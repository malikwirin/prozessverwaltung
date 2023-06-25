use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::Local;

use crate::process::Process;


pub fn output(processes: &Vec<Process>, scheduling_algorithm: &str, switch_time: &u32, time_units: &u32) -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Erstellen des Dateinamens mit Datum und Uhrzeit
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("Output/output_{}.org", timestamp);

    // Erstellen des Verzeichnisses, falls es nicht existiert
    fs::create_dir_all("Output")?;

    // Öffnen der Datei im Schreibmodus
    let mut file = File::create(Path::new(&filename))?;

    // Schreiben von Zeilen in die Datei
    writeln!(file, "* This is the output of the scheduler simulation.")?;

    writeln!(file, "** About the scheduling processing method")?;
    writeln!(file, "The simulation used the {} scheduling algorithm.", scheduling_algorithm)?;
    writeln!(file, "Switching processes costs {} time units.", switch_time)?;
    if scheduling_algorithm == "Round Robin (RR)" {
        writeln!(file, "The Round Robin algorithm processes every process up to {} time units at a time.", time_units)?;
    }

    writeln!(file, "** Averages")?;
    writeln!(file, "The average waiting time for the processes is {}.", average_waiting_time(processes))?;
    writeln!(file, "The average turnaround time for the processes is {}.", average_process_time(processes))?;
    writeln!(file, "The average reaction time for the processes is {}.", average_reaction_time(processes))?;

    writeln!(file, "** Processes")?;
    // Schreiben der Prozesswerte in die Ausgabedatei
    for process in processes {
        let line = format!(
            "Process: {} (ID: {}), Arrival Time: {}, Execution Time: {}, Start: {}, End: {}, Waiting Time: {}, Turnaround Time: {}, Reaction Time {}\n",
            process.name, process.id, process.arrival_time, process.execution_time, process.start, process.end, process.waiting_time, process.process_time, process.reaction_time
        );
        file.write_all(line.as_bytes())?;
    }
    // Abschließen und Speichern der Datei
    file.flush()?;
    println!("File {} was created succesfully.", filename);

    Ok(())
}

fn average_waiting_time(processes: &Vec<Process>) -> f32 {
    let mut result: f32 = 0.0;
    for process in processes {
        result += process.waiting_time as f32;
    }
    result = result / processes.len() as f32;
    result
}

fn average_process_time(processes: &Vec<Process>) -> f32 {
    let mut result: f32 = 0.0;
    for process in processes {
        result += process.process_time as f32;
    }
    result = result / processes.len() as f32;
    result
}

fn average_reaction_time(processes: &Vec<Process>) -> f32 {
    let mut result: f32 = 0.0;
    for process in processes {
        result += process.reaction_time as f32;
    }
    result = result / processes.len() as f32;
    result
}
