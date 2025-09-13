use crate::vector_math::vector::Float3;

pub fn load_obj_file(obj_str: String) -> Vec<Float3> {

    let mut vertices: Vec<Float3> = Vec::new();
    let mut triangle_vertices: Vec<Float3> = Vec::new();

    // Loop over lines in the file
    for line in obj_str.lines() {
        // Handle a line for a vertex
        if let Some(trimmed) = line.strip_prefix("v ") {
            let v: Vec<f64> = trimmed.split(' ')
                                     .map(|s| s.parse::<f64>()
                                                     .expect("Failed parsing a vertex line!"))
                                     .collect();
            // Add the vertex as a Float3 - fails if the line is missing data!
            vertices.push(Float3 { x: v[0], y: v[1], z: v[2]});
        // Handle the triplets for faces. Each triplet is face_idx/texture_idx/normal_idx
        } else if let Some(trimmed) = line.strip_prefix("f ") {
            let triplets: Vec<Vec<usize>> = trimmed.split(' ')
                                                    .map(|s| s.split('/')
                                                                    .map(|t| t.parse::<usize>()
                                                                    .expect("Failed parsing a triplet line!"))
                                                    .collect::<Vec<usize>>()).collect();

            for idx in 0..triplets.len() {
                // Extract the face index
                let point_idx = triplets[idx][0] - 1;
                // If the face has more than 3 vertices, we build n-gon triangle fan
                if idx >= 3 {
                    triangle_vertices.push(triangle_vertices[triangle_vertices.len() - (3*idx - 6)]);
                    triangle_vertices.push(triangle_vertices[triangle_vertices.len() - 2]);
                }
                triangle_vertices.push(vertices[point_idx]);
            }
        }
    }
    triangle_vertices
}