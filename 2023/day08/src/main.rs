fn part_1(input: &str) -> usize {
    let mut current_node = "AAA";
    let mut steps = 0;

    loop {
        for instruction in input.chars() {
            // Update the current node based on the instruction
            current_node = match instruction {
                'R' => current_node.split(", ").nth(1).unwrap_or(current_node),
                'L' => current_node.split(", ").next().unwrap_or(current_node),
                _ => current_node, // Ignore other characters
            };

            // Increment the step count
            steps += 1;

            // If ZZZ is reached, return the step count
            if current_node == "ZZZ" {
                return steps;
            }
        }
    }
}