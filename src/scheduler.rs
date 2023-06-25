pub fn set_scheduling_algorithm(scheduling_algorithm: &str) -> String {
    match scheduling_algorithm {
        "1" => "First-In, First-Out (FIFO)".to_string(),
        "2" => "Round Robin (RR)".to_string().to_string(),
        "3" => "Non-preemptive Shortest Job First".to_string(),
        "4" => "Preemptive Shortest Job First".to_string(),
        "fifo" => "First-In, First-Out (FIFO)".to_string(),
        "rr" => "Round Robin (RR)".to_string(),
        "FCFS" => "First-In, First-Out (FIFO)".to_string(),
        "NpSJF" => "Non-preemptive Shortest Job First".to_string(),
        "PSJF" => "Preemptive Shortest Job First".to_string(),

        "Non-preemptive Shortest Job First" => scheduling_algorithm.to_string(),
        "First-In, First-Out (FIFO)" => scheduling_algorithm.to_string(),
        "Round Robin (RR)" => scheduling_algorithm.to_string(),
        "Preemptive Shortest Job First" => scheduling_algorithm.to_string(),


        _ => "Unbekanntes Scheduling-Verfahren".to_string(),
    }
}
