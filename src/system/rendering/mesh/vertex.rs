pub struct Vertex{
    pub position: [f64; 3] ,
    pub uv: [f32; 2],
    // color: [u8]
}

impl Default for Vertex{
    fn default() -> Self {
        Vertex{
            position: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0],
        }
    }
}