use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    const CAMERA: Camera = Camera {
        position: Vector3(0.0, 0.0, -10.0),
        fov: 90,
        resolution: [800, 600],
    };
    const SIZE: f32 = 100.0;

    // Generated vertices, edges and faces for a cube
    let cube_vertices: Vec<Vector3> = vec![
        Vector3(-0.5 * SIZE, -0.5 * SIZE, -0.5 * SIZE), // 0
        Vector3(0.5 * SIZE, -0.5 * SIZE, -0.5 * SIZE),  // 1
        Vector3(0.5 * SIZE, 0.5 * SIZE, -0.5 * SIZE),   // 2
        Vector3(-0.5 * SIZE, 0.5 * SIZE, -0.5 * SIZE),  // 3
        Vector3(-0.5 * SIZE, -0.5 * SIZE, 0.5 * SIZE),  // 4
        Vector3(0.5 * SIZE, -0.5 * SIZE, 0.5 * SIZE),   // 5
        Vector3(0.5 * SIZE, 0.5 * SIZE, 0.5 * SIZE),    // 6
        Vector3(-0.5 * SIZE, 0.5 * SIZE, 0.5 * SIZE),   // 7
    ];

    let cube_edges_storage: Vec<Edge> = vec![
        Edge { vertex_pair: [&cube_vertices[0], &cube_vertices[1]] },
        Edge { vertex_pair: [&cube_vertices[1], &cube_vertices[2]] },
        Edge { vertex_pair: [&cube_vertices[2], &cube_vertices[3]] },
        Edge { vertex_pair: [&cube_vertices[3], &cube_vertices[0]] },
        Edge { vertex_pair: [&cube_vertices[4], &cube_vertices[5]] },
        Edge { vertex_pair: [&cube_vertices[5], &cube_vertices[6]] },
        Edge { vertex_pair: [&cube_vertices[6], &cube_vertices[7]] },
        Edge { vertex_pair: [&cube_vertices[7], &cube_vertices[4]] },
        Edge { vertex_pair: [&cube_vertices[0], &cube_vertices[4]] },
        Edge { vertex_pair: [&cube_vertices[1], &cube_vertices[5]] },
        Edge { vertex_pair: [&cube_vertices[2], &cube_vertices[6]] },
        Edge { vertex_pair: [&cube_vertices[3], &cube_vertices[7]] },
    ];
    // Borrow the Edge instances from the Vec
    let cube_edges: Vec<&Edge> = cube_edges_storage.iter().collect();

    let cube_faces = vec![];

    // Create the Mesh
    let cube = Mesh {
        vertices: &cube_vertices,
        edges: cube_edges,
        faces: cube_faces,
    };


    // Main loop
    loop {
        for vertex in cube.vertices {
            let vertex_2d = vertex.get_2d_coords(&CAMERA);
            draw_circle(vertex_2d.0 + (CAMERA.resolution[0] as f32 / 2.0),
                        vertex_2d.1 + (CAMERA.resolution[1] as f32 / 2.0),
                        5.0, RED);
        }
        next_frame().await
    }
}


#[derive(Debug)]
struct Vector2(f32, f32);
struct Vector3(f32, f32, f32);
struct Edge<'a> {
    vertex_pair: [&'a Vector3; 2],
}
struct Face<'a> {
    vertices: [&'a Vector3; 3],
}
struct Mesh<'a> {
    vertices: &'a Vec<Vector3>,
    edges: Vec<&'a Edge<'a>>,
    faces: Vec<&'a Face<'a>>,
}
struct Camera {
    position: Vector3,
    fov: u32,
    resolution: [u32; 2],

}

impl Vector3 {
    fn get_distance_between(&self, other: &Vector3) -> Vector3 {
        let x = self.0 - other.0;
        let y = self.1 - other.1;
        let z = self.2 - other.2;
        Vector3(x, y, z)
    }
    fn get_2d_coords(&self, camera: &Camera) -> Vector2 {
        let distance = self.get_distance_between(&camera.position);
        let x = (distance.0 / distance.2) * camera.fov as f32;
        let y = (distance.1 / distance.2) * camera.fov as f32;
        Vector2(x, y)
    }
}